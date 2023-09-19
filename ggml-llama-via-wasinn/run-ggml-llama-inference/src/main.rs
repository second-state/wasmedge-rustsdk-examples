#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    params,
    plugin::PluginManager,
    Module, VmBuilder,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    infer()?;

    Ok(())
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn infer() -> Result<(), Box<dyn std::error::Error>> {
    // parse arguments
    let args: Vec<String> = std::env::args().collect();
    let dir_mapping = &args[1];
    let wasm_file = &args[2];
    let model_name = &args[3];
    let prompt = &args[4];

    println!("load plugin");

    // load wasinn-pytorch-plugin from the default plugin directory: /usr/local/lib/wasmedge
    PluginManager::load(None)?;
    // preload named model
    let preloads = vec!["default:GGML:CPU:orca-mini-3b.ggmlv3.q4_0.bin"];
    PluginManager::nn_preload(preloads);

    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()?;
    assert!(config.wasi_enabled());
    // assert!(config.wasi_nn_enabled());

    // load wasm module from file
    let module = Module::from_file(Some(&config), wasm_file)?;

    // create a Vm
    let mut vm = VmBuilder::new()
        .with_config(config)
        .with_plugin_wasi_nn()
        .build()?
        .register_module(Some("extern"), module)?;

    // init wasi module
    vm.wasi_module_mut()
        .expect("Not found wasi module")
        .initialize(
            Some(vec![wasm_file, model_name, prompt]),
            None,
            Some(vec![dir_mapping]),
        );

    vm.run_func(Some("extern"), "_start", params!())?;

    Ok(())
}
