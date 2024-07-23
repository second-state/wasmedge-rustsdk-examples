use std::collections::HashMap;

use wasmedge_sdk::{
    error::CoreError, params, vm::SyncInst, wasi::WasiModule, AsInstance, CallingFrame,
    ImportObjectBuilder, Instance, Store, ValType, Vm, WasmVal, WasmValue,
};

// define host data
#[derive(Clone, Debug)]
struct Circle {
    radius: i32,
}

fn get_radius(
    data: &mut Circle,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _input: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    Ok(vec![WasmValue::from_i32(data.radius)])
}

fn inc_radius(
    data: &mut Circle,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    input: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    // check the number of inputs
    if input.len() != 1 {
        return Err(CoreError::Execution(
            wasmedge_sdk::error::CoreExecutionError::FuncSigMismatch,
        ));
    }

    // parse the first input of WebAssembly value type into Rust built-in value type
    let value = if input[0].ty() == ValType::I32 {
        input[0].to_i32()
    } else {
        return Err(CoreError::Execution(
            wasmedge_sdk::error::CoreExecutionError::FuncSigMismatch,
        ));
    };

    data.radius += value;

    Ok(vec![])
}

fn main() {
    let circle = Circle { radius: 10 };

    let mut wasi_module = WasiModule::create(None, None, None).unwrap();

    // create an import module
    let mut import_builder = ImportObjectBuilder::new("extern", circle).unwrap();
    import_builder
        .with_func::<i32, ()>("inc_radius", inc_radius)
        .unwrap();
    import_builder
        .with_func::<(), i32>("get_radius", get_radius)
        .unwrap();
    let mut import_object = import_builder.build();

    let mut instances: HashMap<String, &mut dyn SyncInst> = HashMap::new();
    instances.insert(wasi_module.name().to_string(), wasi_module.as_mut());
    instances.insert(import_object.name().unwrap(), &mut import_object);

    // create a new Vm with default config
    let mut vm = Vm::new(Store::new(None, instances).unwrap());

    let res = vm.run_func(Some("extern"), "get_radius", vec![]).unwrap();
    println!("get_radius() = {}", res[0].to_i32());

    let res = vm.run_func(Some("extern"), "inc_radius", params!(5));
    println!("inc_radius(5) = {:?}", res);

    let res = vm.run_func(Some("extern"), "get_radius", vec![]).unwrap();
    println!("get_radius() = {}", res[0].to_i32());
}
