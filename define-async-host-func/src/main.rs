use wasmedge_sdk::{
    async_host_function, error::HostFuncError, params, Caller, ImportObjectBuilder, VmBuilder,
    WasmValue, NeverType, r#async::AsyncState
};

#[async_host_function]
async fn read_book<T>(
    _caller: Caller,
    _args: Vec<WasmValue>,
    _ctx: Option<&mut T>,
) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("[read_book] sleep 2 second");
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    println!("[read_book] reading ...");

    Ok(vec![])
}

#[async_host_function]
async fn enjoy_music<T>(
    _caller: Caller,
    _args: Vec<WasmValue>,
    _ctx: Option<&mut T>,
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
        .with_func_async::<(), (), NeverType>("read_book", read_book, None)?
        .with_func_async::<(), (), NeverType>("enjoy_music", enjoy_music, None)?
        .build("extern")?;

    let vm = VmBuilder::new().build()?.register_import_module(import)?;

    let async_state1 = AsyncState::new();
    let async_state2 = AsyncState::new();
    let (_, _) = tokio::join!(
        vm.run_func_async(&async_state1, Some("extern"), "read_book", params!()),
        vm.run_func_async(&async_state2, Some("extern"), "enjoy_music", params!()),
    );

    Ok(())
}
