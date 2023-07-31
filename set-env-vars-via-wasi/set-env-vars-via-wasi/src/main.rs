use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    params, VmBuilder,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let wasm_file = std::path::PathBuf::from(&args[1]);

    // enable the `wasi` option
    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()?;

    // create a vm
    let mut vm = VmBuilder::new().with_config(config).build()?;

    // set the envs and args for the wasi module
    let args = vec!["arg1", "arg2"];
    let envs = vec!["ENV1=VAL1", "ENV2=VAL2", "ENV3=VAL3"];
    // the preopened directory is the current directory. You have to guarantee
    // the write permission if you wanna write something in this directory.
    let preopens = vec![(".:./target")];
    let wasi_module = vm.wasi_module_mut().expect("Not found wasi module");
    wasi_module.initialize(Some(args), Some(envs), Some(preopens));
    assert_eq!(wasi_module.exit_code(), 0);

    // load wasm module and run the wasm function named `print_env`
    vm.run_func_from_file(wasm_file, "print_env", params!())?;

    Ok(())
}
