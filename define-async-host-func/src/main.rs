use wasmedge_sdk::{
    async_host_function, error::HostFuncError, params, r#async::AsyncState, Caller,
    ImportObjectBuilder, NeverType, VmBuilder, WasmValue,
};

#[async_host_function]
async fn read_book(
    _caller: Caller,
    _args: Vec<WasmValue>,
    _ctx: *mut std::ffi::c_void,
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
    _ctx: *mut std::ffi::c_void,
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
        .with_async_func::<(), (), NeverType>("read_book", read_book, None)?
        .with_async_func::<(), (), NeverType>("enjoy_music", enjoy_music, None)?
        .build::<NeverType>("extern", None)?;

    let mut vm = VmBuilder::new().build()?;

    vm.register_import_module(&import)?;

    let async_state1 = AsyncState::new();
    let async_state2 = AsyncState::new();
    let (_, _) = tokio::join!(
        vm.run_func_async(&async_state1, Some("extern"), "read_book", params!()),
        vm.run_func_async(&async_state2, Some("extern"), "enjoy_music", params!()),
    );

    Ok(())
}
