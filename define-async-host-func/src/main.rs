use std::{collections::HashMap, future::Future};

use wasmedge_sdk::{
    error::CoreError,
    r#async::{import::ImportObjectBuilder, vm::Vm, AsyncInstance},
    CallingFrame, Store, WasmValue,
};

fn async_hello(
    _data: &mut (),
    _inst: &mut AsyncInstance,
    _frame: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Box<dyn Future<Output = Result<Vec<WasmValue>, CoreError>> + Send> {
    Box::new(async {
        for _ in 0..10 {
            println!("[async hello] say hello");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }

        println!("[async hello] Done!");

        Ok(vec![])
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create an import module
    let mut builder = ImportObjectBuilder::new("extern", ()).unwrap();
    builder.with_func::<(), ()>("hello", async_hello).unwrap();
    let mut import = builder.build();

    let mut instances = HashMap::new();
    instances.insert("extern".into(), &mut import);
    let store = Store::new(None, instances).unwrap();
    // create a Vm
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
    let _ = vm.run_func(Some("extern"), "hello", []).await?;

    Ok(())
}
