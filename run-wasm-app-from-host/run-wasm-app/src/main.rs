use std::collections::HashMap;

use wasmedge_sdk::{params, vm::SyncInst, wasi::WasiModule, Module, Store, Vm};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_app_file = std::env::args().nth(1).expect("Please specify a wasm file");

    let mut wasi_module = WasiModule::create(None, None, None).unwrap();

    let mut instances: HashMap<String, &mut dyn SyncInst> = HashMap::new();
    instances.insert(wasi_module.name().to_string(), wasi_module.as_mut());
    let mut vm = Vm::new(Store::new(None, instances).unwrap());

    let module = Module::from_file(None, &wasm_app_file).unwrap();
    vm.register_module(Some("wasm-app"), module).unwrap();
    vm.run_func(Some("wasm-app"), "_start", params!())?;
    Ok(())
}
