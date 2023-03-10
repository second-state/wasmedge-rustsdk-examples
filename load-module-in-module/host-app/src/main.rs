use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    params, VmBuilder, WasmVal,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    println!("args: {:?}", args);

    let num1: i32 = args[1].parse().unwrap();
    let num2: i32 = args[2].parse().unwrap();

    // create a new Vm with default config
    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()?;

    // create a vm and register bob and alice wasm modules into the vm
    let vm = VmBuilder::new()
        .with_config(config)
        .build()?
        .register_module_from_file(
            "my_math_lib",
            "target/wasm32-wasi/release/bob_wasm_lib.wasm",
        )?
        .register_module_from_file("alice", "target/wasm32-wasi/release/alice_wasm_lib.wasm")?;

    // call the `add` wasm function defined in the `alice` module instance.
    // `alice::add` will call `bob::read_add` internally.
    let res = vm.run_func(Some("alice"), "add", params!(num1, num2))?;

    println!("add({}, {}) = {}", num1, num2, res[0].to_i32());

    Ok(())
}
