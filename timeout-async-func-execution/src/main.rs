use wasmedge_sdk::{
    async_host_function, error::HostFuncError, params, Caller, ImportObjectBuilder, VmBuilder,
    WasmValue,
};

#[async_host_function]
async fn say_hello(
    _caller: Caller,
    _args: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, HostFuncError> {
    for i in 0..20 {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        println!("{i}: Hello, world!");
    }

    Ok(vec![])
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create an import module
    let import = ImportObjectBuilder::new()
        .with_func_async::<(), ()>("say_hello", say_hello)?
        .build("extern")?;

    let vm = VmBuilder::new().build()?.register_import_module(import)?;

    let handle = tokio::spawn(async move {
        let fut = vm.run_func_async(Some("extern"), "say_hello", params!());
        tokio::time::timeout(std::time::Duration::from_secs(5), fut).await
    })
    .await;

    match handle {
        Ok(_) => println!("ok"),
        Err(_) => println!("err"),
    }

    println!("sleeping 5 secs in main thread");
    std::thread::sleep(std::time::Duration::from_secs(5));

    println!("done");

    Ok(())
}
