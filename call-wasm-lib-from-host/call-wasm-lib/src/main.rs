use wasmedge_sdk::{config::ConfigBuilder, params, VmBuilder, WasmVal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    println!("args: {:?}", args);

    let wasm_lib_file = &args[1];
    let num: i32 = args[2].parse().unwrap();

    // create a config with the `wasi` option enabled
    let config = ConfigBuilder::default().build()?;

    // create a VM with the config
    let vm = VmBuilder::new().with_config(config).build()?;

    let res = vm
        .register_module_from_file("wasm-lib", &wasm_lib_file)?
        .run_func(Some("wasm-lib"), "fib", params!(num))?;
    println!("fib({}) = {}", num, res[0].to_i32());

    Ok(())
}
