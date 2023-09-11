use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    wasi::r#async::{AsyncState, WasiContext},
    VmBuilder,
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
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()
        .expect("failed to create config");
    assert!(config.wasi_enabled());

    // create WasiContext
    let wasi_ctx = WasiContext::new(None, Some(vec![("ENV", "VAL")]), Some(vec![(".", ".")]));

    // create a Vm
    let mut vm = VmBuilder::new()
        .with_config(config)
        .with_wasi_context(wasi_ctx)
        .build()
        .expect("failed to create vm");

    // run the wasm function from a specified wasm file
    let async_state = AsyncState::new();
    let _ = vm
        .run_func_from_file_async(&async_state, wasm_file, "_start", [])
        .await
        .expect("failed to run func from file");

    let wasi_module = vm.wasi_module().ok_or("failed to get wasi module")?;
    println!("exit_code: {}", wasi_module.exit_code());

    Ok(())
}
