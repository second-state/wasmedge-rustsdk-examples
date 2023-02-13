use wasmedge_sdk::{
    async_host_function, error::HostFuncError, params, Caller, ImportObjectBuilder, Vm, WasmValue,
};

#[async_host_function]
async fn read_book(
    _caller: Caller,
    _args: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("[read_book] sleep 2 second");
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    println!("[read_book] reading ...");

    Ok(vec![])
}

#[async_host_function]
async fn enjoy_music(
    _caller: Caller,
    _args: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("[enjoy_music] sleep 1 second");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    println!("[enjoy_music] enjoying music ...");

    Ok(vec![])
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create an import module
    let import = ImportObjectBuilder::new()
        .with_func_async::<(), ()>("read_book", read_book)?
        .with_func_async::<(), ()>("enjoy_music", enjoy_music)?
        .build("extern")?;

    let vm = Vm::new(None, None)?.register_import_module(import)?;

    let (_, _) = tokio::join!(
        vm.run_func_async(Some("extern"), "read_book", params!()),
        vm.run_func_async(Some("extern"), "enjoy_music", params!()),
    );

    Ok(())
}
