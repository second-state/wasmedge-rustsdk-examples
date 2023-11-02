use std::{collections::HashMap, future::Future};

use wasmedge_sdk::{
    error::CoreError,
    r#async::{import::ImportObjectBuilder, vm::Vm, AsyncInstance},
    CallingFrame, Store, WasmValue,
};

fn async_hello_with_data<'fut>(
    circle: &'fut mut Circle,
    _inst: &mut AsyncInstance,
    _frame: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Box<dyn Future<Output = Result<Vec<WasmValue>, CoreError>> + Send + 'fut>
where
{
    Box::new(async {
        for _ in 0..10 {
            println!("[async hello] say hello");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            println!("[async hello] radius of circle: {}", circle.radius);
            circle.radius += 1;
        }

        println!("[async hello] Done!");

        Ok(vec![])
    })
}

// define host data
#[derive(Clone, Debug)]
struct Circle {
    radius: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let circle = Circle { radius: 10 };

    // create an import module
    // let import = ImportObjectBuilder::new()
    //     .with_async_func::<(), (), Circle>(
    //         "hello_with_data",
    //         async_hello_with_data,
    //         Some(Box::new(circle)),
    //     )?
    //     .build::<NeverType>("extern", None)?;

    let mut builder = ImportObjectBuilder::new("extern", circle).unwrap();
    builder
        .with_func::<(), ()>("hello_with_data", async_hello_with_data)
        .unwrap();
    let mut import = builder.build();
    // create a Vm
    let mut instances = HashMap::new();
    instances.insert("extern".into(), &mut import);
    let store = Store::new(None, instances).unwrap();
    let mut vm = Vm::new(store);

    async fn tick() {
        let mut i = 0;
        loop {
            println!("[tick] i={i}");
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            i += 1;
        }
    }
    tokio::spawn(tick());

    // run the async host function
    let _ = vm.run_func(Some("extern"), "hello_with_data", []).await?;

    Ok(())
}
