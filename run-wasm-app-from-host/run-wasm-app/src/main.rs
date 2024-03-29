use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    params, VmBuilder,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_app_file = std::env::args().nth(1).expect("Please specify a wasm file");

    // create a config with the `wasi` option enabled
    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()?;
    assert!(config.wasi_enabled());

    // create a VM with the config
    let mut vm = VmBuilder::new().with_config(config).build()?;

    vm.wasi_module_mut()
        .expect("Not found wasi module")
        .initialize(None, None, None);

    vm.register_module_from_file("wasm-app", &wasm_app_file)?
        .run_func(Some("wasm-app"), "_start", params!())?;

    Ok(())
}
