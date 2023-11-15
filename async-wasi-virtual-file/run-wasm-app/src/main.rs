use std::{
    collections::HashMap,
    io::{Read, Write},
};

use bytes::Buf;
use wasmedge_sdk::{
    r#async::{
        vm::Vm,
        wasi::{
            async_wasi::{
                snapshots::env::{
                    vfs::{WasiStdin, WasiStdout},
                    VFD,
                },
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

    let wasm_file = &args[1];

    let mut wasi_ctx = WasiCtx::new();

    if let Ok(wasi_stdin) = wasi_ctx.get_mut_vfd(0) {
        let data = bytes::Bytes::from(Vec::from("Hello Wasm!".to_string())).reader();
        let virtual_stdin: Box<dyn Read + Send> = Box::new(data);
        *wasi_stdin = VFD::Inode(WasiStdin::from(virtual_stdin).into());
    }

    if let Ok(wasi_stdout) = wasi_ctx.get_mut_vfd(1) {
        let wasm_stdout_file = std::fs::File::create("./wasm_stdout.txt").unwrap();
        let virtual_stdout: Box<dyn Write + Send> = Box::new(wasm_stdout_file);
        *wasi_stdout = VFD::Inode(WasiStdout::from(virtual_stdout).into());
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
