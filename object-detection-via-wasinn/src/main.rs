use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    params, Module, Vm,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse arguments
    let args: Vec<String> = std::env::args().collect();
    let dir_mapping = &args[1];
    let wasm_file = &args[2];
    let model_bin = &args[3];
    let image_file = &args[4];

    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(
            HostRegistrationConfigOptions::default()
                .wasi(true)
                .wasi_nn(true),
        )
        .build()?;
    assert!(config.wasi_enabled());
    assert!(config.wasi_nn_enabled());

    // load wasm module from file
    let module = Module::from_file(Some(&config), wasm_file)?;

    // create a Vm
    let mut vm = Vm::new(Some(config))?;

    // init wasi module
    let mut wasi_module = vm.wasi_module()?.initialize(
        Some(vec!["placeholder", model_bin, image_file]),
        None,
        Some(vec![dir_mapping]),
    );

    // register the wasm module hosting the wasi-nn-pytorch app into vm
    let _ =
        vm.register_module(Some("extern"), module)?
            .run_func(Some("extern"), "_start", params!());

    Ok(())
}
