use wasmedge_sdk::{
    error::HostFuncError, host_function, params, Caller, ImportObject, ImportObjectBuilder, Vm,
    WasmValue,
};

#[host_function]
fn hello(_caller: Caller, _args: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("Hello, world!");

    Ok(vec![])
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create a new WasmEdge Vm instance
    let vm = Vm::new(None, None)?;

    // create an import_object module with the host function
    let import: ImportObject = ImportObjectBuilder::new()
        .with_func::<(), ()>("say_hello", hello)?
        .build("extern")?;

    // register the import module into vm
    let vm = vm.register_import_module(import)?;

    let _ = vm.run_func(Some("extern"), "say_hello", params!())?;

    Ok(())
}
