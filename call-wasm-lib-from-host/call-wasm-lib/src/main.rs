use std::collections::HashMap;

use wasmedge_sdk::{params, wasi::WasiModule, Module, Store, Vm, WasmVal};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("args: {:?}", args);

    let wasm_lib_file = &args[1];
    let num: i32 = args[2].parse().unwrap();

    let mut wasi_module = WasiModule::create(None, None, Some(vec![".:."])).unwrap();

    let mut instances = HashMap::new();
    instances.insert(wasi_module.name().to_string(), wasi_module.as_mut());

    let store = Store::new(None, instances).unwrap();

    let mut vm = Vm::new(store);

    let module = Module::from_file(None, wasm_lib_file).unwrap();

    vm.register_module(Some("wasm-lib"), module).unwrap();

    let res = vm.run_func(Some("wasm-lib"), "fib", params!(num)).unwrap();
    println!("fib({}) = {}", num, res[0].to_i32());
}
