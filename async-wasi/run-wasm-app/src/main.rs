use std::collections::HashMap;

use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder},
    r#async::{vm::Vm, wasi::AsyncWasiModule},
    Module, Store,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: main-app <wasm_file>");
        return Ok(());
    }
    let wasm_file = &args[1];

    async fn tick() {
        let mut i = 0;
        loop {
            println!("[tick] i={i}");
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            i += 1;
        }
    }

    // run `tick` function
    tokio::spawn(tick());

    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .build()
        .expect("failed to create config");

    let mut wasi_module = AsyncWasiModule::create(
        None::<Vec<&str>>,
        Some(vec![("ENV", "VAL")]),
        Some(vec![(".".into(), ".".into())]),
    )?;
    let module = Module::from_file(Some(&config), wasm_file)?;

    let mut instance_map = HashMap::new();
    instance_map.insert(wasi_module.name().to_string(), wasi_module.as_mut());

    let store = Store::new(Some(&config), instance_map)?;

    // create a Vm
    let mut vm = Vm::new(store);

    vm.register_module(None, module)?;
    let _ = vm
        .run_func(None, "_start", [])
        .await
        .expect("failed to run func from file");
    // run the wasm function from a specified wasm file

    println!("exit_code: {}", wasi_module.exit_code());

    Ok(())
}
