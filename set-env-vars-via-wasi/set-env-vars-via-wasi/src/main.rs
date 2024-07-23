use std::collections::HashMap;

use wasmedge_sdk::{params, wasi::WasiModule, Module, Store, Vm};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let wasm_file = std::path::PathBuf::from(&args[1]);

    // set the envs and args for the wasi module
    let args = vec!["arg1", "arg2"];
    let envs = vec!["ENV1=VAL1", "ENV2=VAL2", "ENV3=VAL3"];
    // the preopened directory is the current directory. You have to guarantee
    // the write permission if you wanna write something in this directory.
    let preopens = vec![(".:.")];
    let mut wasi_module = WasiModule::create(Some(args), Some(envs), Some(preopens)).unwrap();
    let mut instances = HashMap::new();
    instances.insert(wasi_module.name().to_string(), wasi_module.as_mut());
    // create a vm
    let mut vm = Vm::new(Store::new(None, instances).unwrap());

    let module = Module::from_file(None, wasm_file).unwrap();
    vm.register_module(None, module).unwrap();
    // load wasm module and run the wasm function named `print_env`
    vm.run_func(None, "print_env", params!()).unwrap();

    Ok(())
}
