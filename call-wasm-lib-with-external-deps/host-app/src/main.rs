use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    error::HostFuncError,
    host_function, params, Caller, ImportObjectBuilder, ValType, Vm, WasmVal, WasmValue,
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
    let args: Vec<String> = std::env::args().collect();
    println!("args: {:?}", args);

    let wasm_lib_file = &args[1];
    let num1: i32 = args[2].parse().unwrap();
    let num2: i32 = args[3].parse().unwrap();

    // create an import module
    let import = ImportObjectBuilder::new()
        .with_func::<(i32, i32), i32>("real_add", my_add)?
        .build("my_math_lib")?;

    // create a new Vm with default config
    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()?;

    let res = Vm::new(Some(config))?
        .register_import_module(import)?
        .register_module_from_file("extern", wasm_lib_file)?
        .run_func(Some("extern"), "add", params!(num1, num2))?;

    println!("add({}, {}) = {}", num1, num2, res[0].to_i32());

    Ok(())
}
