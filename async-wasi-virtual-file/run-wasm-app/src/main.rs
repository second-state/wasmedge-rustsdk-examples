use std::{collections::HashMap, io::Write};

use wasmedge_sdk::{
    r#async::{
        vm::Vm,
        wasi::{
            async_wasi::{
                snapshots::env::{vfs::WasiStdout, VFD},
                WasiCtx,
            },
            AsyncWasiModule,
        },
    },
    Module, Store,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: main-app <wasm_file>");
        return Ok(());
    }

    let wasm_stdout_file = std::fs::File::create("./wasm_stdout.txt").unwrap();

    let wasm_file = &args[1];

    let mut wasi_ctx = WasiCtx::new();

    if let Ok(wasi_stdout) = wasi_ctx.get_mut_vfd(1) {
        let fake_stdout: Box<dyn Write + Send> = Box::new(wasm_stdout_file);
        *wasi_stdout = VFD::Inode(WasiStdout::from(fake_stdout).into());
    }

    let mut wasi_module = AsyncWasiModule::create_from_wasi_context(wasi_ctx).unwrap();

    let module = Module::from_file(None, wasm_file)?;

    let mut instance_map = HashMap::new();
    instance_map.insert(wasi_module.name().to_string(), wasi_module.as_mut());

    let store = Store::new(None, instance_map)?;

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
