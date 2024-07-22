use std::collections::HashMap;

use wasmedge_sdk::{
    error::CoreError, params, CallingFrame, ImportObjectBuilder, Instance, Store, Vm, WasmValue,
};

fn hello(
    _data: &mut (),
    _inst: &mut Instance,
    _frame: &mut CallingFrame,
    _args: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    println!("Hello, world!");

    Ok(vec![])
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create a new WasmEdge Vm instance

    // create an import_object module with the host function
    let mut builder = ImportObjectBuilder::new("extern", ())?;
    builder.with_func::<(), ()>("say_hello", hello)?;
    let mut import = builder.build();

    let mut instances = HashMap::new();
    instances.insert("extern".into(), &mut import);
    let store = Store::new(None, instances)?;
    let mut vm = Vm::new(store);

    let _ = vm.run_func(Some("extern"), "say_hello", params!())?;

    Ok(())
}
