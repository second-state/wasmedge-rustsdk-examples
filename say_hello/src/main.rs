use wasmedge_sdk::{
    error::HostFuncError, host_function, params, Caller, ImportObjectBuilder, NeverType, VmBuilder,
    WasmValue,
};

#[host_function]
fn hello<T>(
    _caller: Caller,
    _args: Vec<WasmValue>,
    _ctx: Option<&mut T>,
) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("Hello, world!");

    Ok(vec![])
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create a new WasmEdge Vm instance
    let vm = VmBuilder::new().build::<NeverType>()?;

    // create an import_object module with the host function
    let import = ImportObjectBuilder::<NeverType>::new()
        .with_func::<(), ()>("say_hello", hello)?
        .build("extern")?;

    // register the import module into vm
    let vm = vm.register_import_module(import)?;

    let _ = vm.run_func(Some("extern"), "say_hello", params!())?;

    Ok(())
}
