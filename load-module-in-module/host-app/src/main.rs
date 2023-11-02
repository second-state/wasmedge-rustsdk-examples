use std::collections::HashMap;

use wasmedge_sdk::{params, wasi::WasiModule, Module, Store, Vm, WasmVal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    println!("args: {:?}", args);

    let num1: i32 = args[1].parse().unwrap();
    let num2: i32 = args[2].parse().unwrap();

    // create a new Vm with default config

    let mut wasi_module = WasiModule::create(None, None, None).unwrap();
    let mut instances = HashMap::new();
    instances.insert(wasi_module.name().to_string(), wasi_module.as_mut());

    // create a vm and register bob and alice wasm modules into the vm
    let store = Store::new(None, instances).unwrap();
    let mut vm = Vm::new(store);

    let bob_wasm = Module::from_file(None, "target/wasm32-wasi/release/bob_wasm_lib.wasm").unwrap();
    let alice_wasm =
        Module::from_file(None, "target/wasm32-wasi/release/alice_wasm_lib.wasm").unwrap();

    vm.register_module(Some("my_math_lib"), bob_wasm).unwrap();
    vm.register_module(Some("alice"), alice_wasm).unwrap();

    // call the `add` wasm function defined in the `alice` module instance.
    // `alice::add` will call `bob::read_add` internally.
    let res = vm.run_func(Some("alice"), "add", params!(num1, num2))?;

    println!("add({}, {}) = {}", num1, num2, res[0].to_i32());

    Ok(())
}
