fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    infer()?;

    Ok(())
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn infer() -> Result<(), Box<dyn std::error::Error>> {
    // parse arguments

    use std::collections::HashMap;

    use wasmedge_sdk::{
        params,
        plugin::{ExecutionTarget, GraphEncoding, NNPreload, PluginManager},
        wasi::WasiModule,
        AsInstance, Module, Store, Vm,
    };
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

    // load wasm module from file
    let module = Module::from_file(None, wasm_file).unwrap();

    let mut wasi = WasiModule::create(
        Some(vec![wasm_file, model_name]),
        Some(vec!["ENCODING=GGML", "TARGET=CPU"]),
        Some(vec![dir_mapping]),
    )
    .unwrap();

    let mut wasi_nn = PluginManager::load_plugin_wasi_nn().unwrap();

    let mut instances: HashMap<String, _> = HashMap::new();

    instances.insert(wasi.name().to_string(), wasi.as_mut());
    instances.insert(wasi_nn.name().unwrap(), &mut wasi_nn);
    let store = Store::new(None, instances).unwrap();

    let mut vm = Vm::new(store);

    vm.register_module(Some("extern"), module).unwrap();

    // create a Vm

    vm.run_func(Some("extern"), "_start", params!())?;

    Ok(())
}
