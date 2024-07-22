use std::collections::HashMap;

use wasmedge_sdk::{
    r#async::{
        vm::Vm,
        wasi::{async_wasi::WasiCtx, AsyncWasiModule},
    },
    Module, Store,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: main-app <wasm_file>");
        return Ok(());
    }
    let wasm_file = &args[1];

    let wasi_ctx = WasiCtx::new();
    let mut wasi = AsyncWasiModule::create_from_wasi_context(wasi_ctx).unwrap();

    // let mut instance: HashMap<String, &mut (dyn AsInstance + Send)> = HashMap::new();
    let mut instances = HashMap::new();
    instances.insert(wasi.name().to_string(), wasi.as_mut().as_mut());

    let store = Store::new(None, instances)?;

    let module = Module::from_file(None, wasm_file)?;

    // create a Vm
    let mut vm = Vm::new(store);
    vm.register_module(None, module)?;
    vm.run_func(None, "_start", [])
        .await
        .expect("failed to run func from file");

    println!("exit_code: {}", wasi.exit_code());

    Ok(())
}
