#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
use wasmedge_sdk::{params, plugin::PluginManager, Module};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    infer()?;

    Ok(())
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn infer() -> Result<(), Box<dyn std::error::Error>> {
    // parse arguments

    use std::collections::HashMap;

    use wasmedge_sdk::{wasi::WasiModule, AsInstance, Store, Vm};
    let args: Vec<String> = std::env::args().collect();
    let dir_mapping = &args[1];
    let wasm_file = &args[2];
    let model_bin = &args[3];
    let image_file = &args[4];

    println!("load plugin");

    // load wasinn-pytorch-plugin from the default plugin directory: /usr/local/lib/wasmedge
    PluginManager::load(None)?;

    let mut instances = HashMap::new();

    let mut wasi = WasiModule::create(
        Some(vec![wasm_file, model_bin, image_file]),
        None,
        Some(vec![dir_mapping]),
    )
    .unwrap();

    instances.insert(wasi.name().to_string(), wasi.as_mut());

    let mut wasi_nn = PluginManager::load_plugin_wasi_nn().unwrap();
    instances.insert(wasi_nn.name().unwrap().to_string(), &mut wasi_nn);

    let store = Store::new(None, instances).unwrap();

    // load wasm module from file
    let module = Module::from_file(None, wasm_file)?;

    // create a Vm
    let mut vm = Vm::new(store);
    vm.register_module(Some("extern"), module).unwrap();

    vm.run_func(Some("extern"), "_start", params!())?;

    Ok(())
}
