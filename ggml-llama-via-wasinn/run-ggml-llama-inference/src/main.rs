#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    params,
    plugin::{ExecutionTarget, GraphEncoding, NNPreload, PluginManager},
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

    // load wasinn-pytorch-plugin from the default plugin directory: /usr/local/lib/wasmedge
    PluginManager::load(None)?;
    // preload named model
    PluginManager::nn_preload(vec![NNPreload::new(
        "default",
        GraphEncoding::GGML,
        ExecutionTarget::CPU,
        "llama-2-7b-chat.Q5_K_M.gguf",
    )]);

    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()?;
    assert!(config.wasi_enabled());

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
            Some(vec![wasm_file, model_name]),
            Some(vec!["ENCODING=GGML", "TARGET=CPU"]),
            Some(vec![dir_mapping]),
        );

    vm.run_func(Some("extern"), "_start", params!())?;

    Ok(())
}
