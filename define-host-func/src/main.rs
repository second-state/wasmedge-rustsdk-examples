use wasmedge_sdk::{
    error::HostFuncError, host_function, params, Caller, ImportObjectBuilder, NeverType, ValType,
    VmBuilder, WasmVal, WasmValue,
};

#[host_function]
fn my_add(_caller: Caller, input: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    // check the number of inputs
    if input.len() != 2 {
        return Err(HostFuncError::User(1));
    }

    // parse the first input of WebAssembly value type into Rust built-in value type
    let a = if input[0].ty() == ValType::I32 {
        input[0].to_i32()
    } else {
        return Err(HostFuncError::User(2));
    };

    // parse the second input of WebAssembly value type into Rust built-in value type
    let b = if input[1].ty() == ValType::I32 {
        input[1].to_i32()
    } else {
        return Err(HostFuncError::User(3));
    };

    let c = a + b;

    Ok(vec![WasmValue::from_i32(c)])
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create an import module
    let import = ImportObjectBuilder::new()
        .with_func::<(i32, i32), i32, NeverType>("add", my_add, None)?
        .build("extern")?;

    // create a new Vm with default config
    let res = VmBuilder::new()
        .build()?
        .register_import_module(import)?
        .run_func(Some("extern"), "add", params!(15, 51))?;

    println!("add({}, {}) = {}", 15, 51, res[0].to_i32());

    Ok(())
}
