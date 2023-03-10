use std::path::Path;
use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    dock::{Param, VmDock},
    Module, VmBuilder,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let wasm_file = Path::new(&args[1]);
    let module = Module::from_file(None, wasm_file)?;

    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()?;
    let vm = VmBuilder::new()
        .with_config(config)
        .build()?
        .register_module(None, module)?;

    let vm = VmDock::new(vm);

    // call func "say_ok" in wasm-lib.wasm: String -> Result<(u16, String), String>
    let params = vec![Param::String("bindgen funcs test")];
    match vm.run_func("say_ok", params)? {
        Ok(mut res) => println!(
            "Run bindgen -- say: {} {}",
            res.pop().unwrap().downcast::<String>().unwrap(),
            res.pop().unwrap().downcast::<u16>().unwrap()
        ),

        Err(err) => {
            println!("Run bindgen -- say FAILED {}", err);
        }
    }

    // call func "say_err" in wasm-lib.wasm: String -> Result<(u16, String), String>
    let params = vec![Param::String("bindgen funcs test")];
    match vm.run_func("say_err", params)? {
        Ok(mut res) => println!(
            "Run bindgen -- say: {} {}",
            res.pop().unwrap().downcast::<String>().unwrap(),
            res.pop().unwrap().downcast::<u16>().unwrap()
        ),

        Err(err) => {
            println!("Run bindgen -- say FAILED {}", err);
        }
    }

    // call func "create_line" in wasm-lib.wasm: string, string, string -> string (inputs are JSON stringified)
    let params = vec![
        Param::String("{\"x\":2.5,\"y\":7.8}"),
        Param::String("{\"x\":2.5,\"y\":5.8}"),
        Param::String("A thin red line"),
    ];
    match vm.run_func("create_line", params)? {
        Ok(mut res) => {
            println!(
                "Run bindgen -- create_line: {}",
                res.pop().unwrap().downcast::<String>().unwrap()
            );
        }
        Err(err) => {
            println!("Run bindgen -- create_line FAILED {}", err);
        }
    }

    // call func "obfusticate" in wasm-lib.wasm: String -> String
    let params = vec![Param::String("A quick brown fox jumps over the lazy dog")];
    match vm.run_func("obfusticate", params)? {
        Ok(mut res) => {
            println!(
                "Run bindgen -- obfusticate: {}",
                res.pop().unwrap().downcast::<String>().unwrap()
            );
        }
        Err(err) => {
            println!("Run bindgen -- obfusticate FAILED {}", err);
        }
    }

    // call func "lowest_common_multiple" in wasm-lib.wasm: (i32, i32) -> i32
    let params = vec![Param::I32(123), Param::I32(2)];
    match vm.run_func("lowest_common_multiple", params)? {
        Ok(mut res) => {
            println!(
                "Run bindgen -- lowest_common_multiple: {:?}",
                res.pop().unwrap().downcast::<i32>().unwrap()
            );
        }
        Err(err) => {
            println!("Run bindgen -- lowest_common_multiple FAILED {:?}", err);
        }
    }

    // call func "sha3_digest" in wasm-lib.wasm: Vec<u8> -> Vec<u8>
    let params = "This is an important message".as_bytes().to_vec();
    let params = vec![Param::VecU8(&params)];
    match vm.run_func("sha3_digest", params)? {
        Ok(mut res) => {
            println!(
                "Run bindgen -- sha3_digest: {:?}",
                res.pop().unwrap().downcast::<Vec<u8>>().unwrap()
            );
        }
        Err(err) => {
            println!("Run bindgen -- sha3_digest FAILED {}", err);
        }
    }

    // call func "keccak_digest" in wasm-lib.wasm: Vec<u8> -> Vec<u8>
    let params = "This is an important message".as_bytes().to_vec();
    let params = vec![Param::VecU8(&params)];
    match vm.run_func("keccak_digest", params)? {
        Ok(mut res) => {
            println!(
                "Run bindgen -- keccak_digest: {:?}",
                res.pop().unwrap().downcast::<Vec<u8>>().unwrap()
            );
        }
        Err(e) => {
            println!("Run bindgen -- keccak_digest FAILED {:?}", e);
        }
    }

    Ok(())
}
