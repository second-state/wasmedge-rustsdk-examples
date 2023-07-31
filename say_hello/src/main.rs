use wasmedge_sdk::{
    error::HostFuncError, host_function, params, Caller, ImportObjectBuilder, NeverType, VmBuilder,
    WasmValue,
};

#[host_function]
fn hello(_caller: Caller, _args: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("Hello, world!");

    Ok(vec![])
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create a new WasmEdge Vm instance
    let mut vm = VmBuilder::new().build()?;

    // create an import_object module with the host function
    let import = ImportObjectBuilder::new()
        .with_func::<(), (), NeverType>("say_hello", hello, None)?
        .build::<NeverType>("extern", None)?;

    // register the import module into vm
    vm.register_import_module(&import)?;

    let _ = vm.run_func(Some("extern"), "say_hello", params!())?;

    Ok(())
}
