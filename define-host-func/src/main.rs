use std::collections::HashMap;

use wasmedge_sdk::{
    error::CoreError, params, AsInstance, CallingFrame, ImportObjectBuilder, Instance, Store,
    ValType, Vm, WasmVal, WasmValue,
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
    // create an import module

    let mut import_builder = ImportObjectBuilder::new("extern", ()).unwrap();
    import_builder
        .with_func::<(i32, i32), i32>("add", my_add)
        .unwrap();
    let mut import_object = import_builder.build();

    let mut instances = HashMap::new();
    instances.insert(import_object.name().unwrap(), &mut import_object);

    // create a new Vm with default config
    let mut vm = Vm::new(Store::new(None, instances).unwrap());

    let res = vm.run_func(Some("extern"), "add", params!(15, 51)).unwrap();

    println!("add({}, {}) = {}", 15, 51, res[0].to_i32());
}
