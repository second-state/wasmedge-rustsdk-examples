use wasmedge_sdk::{params, plugin::PluginManager, VmBuilder, WasmVal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    PluginManager::load(None)?;

    // create a Vm
    let vm = VmBuilder::new()
        .with_plugin("naive_math_plugin", Some(vec!["naive_math_module"]))
        .build()?;

    let wasm_file = "target/wasm32-wasi/release/naive_math_wasm.wasm";
    let returns = vm.register_module_from_file("math", &wasm_file)?.run_func(
        Some("math"),
        "add",
        params!(1, 2),
    )?;

    println!("1 + 2 = {}", returns[0].to_i32());

    Ok(())
}
