use std::collections::HashMap;

use wasmedge_sdk::{
    error::CoreError, params, vm::SyncInst, wasi::WasiModule, AsInstance, CallingFrame,
    ImportObjectBuilder, Instance, Module, Store, ValType, Vm, WasmVal, WasmValue,
};

fn my_add(
    _: &mut (),
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    input: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    // check the number of inputs
    if input.len() != 2 {
        return Err(CoreError::Execution(
            wasmedge_sdk::error::CoreExecutionError::FuncTypeMismatch,
        ));
    }

    // parse the first input of WebAssembly value type into Rust built-in value type
    let a = if input[0].ty() == ValType::I32 {
        input[0].to_i32()
    } else {
        return Err(CoreError::Execution(
            wasmedge_sdk::error::CoreExecutionError::FuncTypeMismatch,
        ));
    };

    // parse the second input of WebAssembly value type into Rust built-in value type
    let b = if input[1].ty() == ValType::I32 {
        input[1].to_i32()
    } else {
        return Err(CoreError::Execution(
            wasmedge_sdk::error::CoreExecutionError::FuncTypeMismatch,
        ));
    };

    let c = a + b;

    Ok(vec![WasmValue::from_i32(c)])
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("args: {:?}", args);

    let wasm_lib_file = &args[1];
    let num1: i32 = args[2].parse().unwrap();
    let num2: i32 = args[3].parse().unwrap();

    let mut wasi_module = WasiModule::create(None, None, None).unwrap();

    // create an import module
    let mut import_builder = ImportObjectBuilder::new("my_math_lib", ()).unwrap();
    import_builder
        .with_func::<(i32, i32), i32>("real_add", my_add)
        .unwrap();
    let mut import_object = import_builder.build();

    let mut instances: HashMap<String, &mut dyn SyncInst> = HashMap::new();
    instances.insert(wasi_module.name().to_string(), wasi_module.as_mut());
    instances.insert(import_object.name().unwrap(), &mut import_object);

    let mut vm = Vm::new(Store::new(None, instances).unwrap());

    let module = Module::from_file(None, wasm_lib_file).unwrap();

    vm.register_module(Some("extern"), module).unwrap();

    let res = vm
        .run_func(Some("extern"), "add", params!(num1, num2))
        .unwrap();

    println!("add({}, {}) = {}", num1, num2, res[0].to_i32());
}
