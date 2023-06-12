use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    r#async::AsyncState,
    VmBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()
        .expect("failed to create config");
    assert!(config.wasi_enabled());

    // create a Vm
    let mut vm = VmBuilder::default()
        .with_config(config)
        .build()
        .expect("failed to create vm");

    let wasi_module = vm.wasi_module_mut().ok_or("failed to get wasi module")?;
    wasi_module.initialize(
        None,
        Some(vec![("ENV", "VAL")]),
        Some(vec![(
            std::path::PathBuf::from("."),
            std::path::PathBuf::from("."),
        )]),
    )?;

    // run the wasm function from a specified wasm file
    let async_state = AsyncState::new();
    let wasm_file = "target/wasm32-wasi/release/wasm-app.wasm";
    let _ = vm
        .run_func_from_file_async(&async_state, wasm_file, "_start", [])
        .await
        .expect("failed to run func from file");

    let wasi_module = vm.wasi_module_mut().ok_or("failed to get wasi module")?;
    println!("exit_code: {}", wasi_module.exit_code());

    Ok(())
}
