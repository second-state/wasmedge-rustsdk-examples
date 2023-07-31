use wasmedge_sdk::{
    async_host_function, error::HostFuncError, params, r#async::AsyncState, Caller,
    ImportObjectBuilder, NeverType, VmBuilder, WasmValue,
};

#[async_host_function]
async fn async_hello(
    _frame: CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, HostFuncError> {
    for _ in 0..10 {
        println!("[async hello] say hello");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    println!("[async hello] Done!");

    Ok(vec![])
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create an import module
    let import = ImportObjectBuilder::new()
        .with_async_func::<(), (), NeverType>("hello", async_hello, None)?
        .build::<NeverType>("extern", None)?;

    // create a Vm
    let mut vm = VmBuilder::new().build()?;

    // register the import module
    vm.register_import_module(&import)?;

    // create an async state
    let async_state = AsyncState::new();

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
    let _ = vm
        .run_func_async(&async_state, Some("extern"), "hello", params!())
        .await?;

    Ok(())
}
