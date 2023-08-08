use wasmedge_sdk::{params, plugin::PluginManager, Executor, Module, Store, WasmVal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load plugins from the default plugin path
    PluginManager::load(None)?;

    let wasm_file = "target/wasm32-wasi/release/naive_math_wasm.wasm";

    // create a Module from a wasm file
    let module = Module::from_file(None, &wasm_file)?;

    // load plugin
    let plugin_name = "naive_math_plugin";
    let plugin_module_name = "naive_math_module";
    let plugin_instance = PluginManager::find(&plugin_name)?.mod_instance(&plugin_module_name)?;

    // create an Executor
    let mut executor = Executor::new(None, None)?;

    // create a Store
    let mut store = Store::new()?;

    // register the plugin module into the store
    store.register_plugin_module(&mut executor, &plugin_instance)?;

    // register the wasm module
    let instance = store.register_named_module(&mut executor, "math", &module)?;

    // get the wasm function named `add`
    let add = instance.func("add")?;

    // run the wasm function
    let returns = executor.run_func(&add, params!(1, 2))?;

    println!("1 + 2 = {}", returns[0].to_i32());

    Ok(())
}
