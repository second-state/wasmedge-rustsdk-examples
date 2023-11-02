use std::collections::HashMap;

use wasmedge_sdk::{
    config::{CommonConfigOptions, CompilerConfigOptions, ConfigBuilder},
    params,
    wasi::WasiModule,
    wat2wasm, Compiler, CompilerOutputFormat, Module, Store, Vm, WasmVal,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read the wasm bytes
    let wasm_bytes = wat2wasm(
        br#"
        (module
            (export "fib" (func $fib))
            (func $fib (param $n i32) (result i32)
             (if
              (i32.lt_s
               (get_local $n)
               (i32.const 2)
              )
              (return
               (i32.const 1)
              )
             )
             (return
              (i32.add
               (call $fib
                (i32.sub
                 (get_local $n)
                 (i32.const 2)
                )
               )
               (call $fib
                (i32.sub
                 (get_local $n)
                 (i32.const 1)
                )
               )
              )
             )
            )
           )
"#,
    )?;
    let out_dir = std::env::current_dir()?;
    let aot_filename = "example_aot_fibonacci";

    // create a Config context
    let config = ConfigBuilder::new(CommonConfigOptions::new().bulk_memory_operations(true))
        .with_compiler_config(
            CompilerConfigOptions::new()
                .interruptible(true)
                .out_format(CompilerOutputFormat::Native),
        )
        .build()?;

    // compile wasm to so for runing in the `aot` mode
    let compiler = Compiler::new(Some(&config))?;
    let aot_file_path = compiler.compile_from_bytes(wasm_bytes, aot_filename, out_dir)?;
    assert!(&aot_file_path.exists());
    #[cfg(target_os = "macos")]
    assert!(aot_file_path.ends_with("example_aot_fibonacci.dylib"));
    #[cfg(target_os = "linux")]
    assert!(aot_file_path.ends_with("example_aot_fibonacci.so"));
    #[cfg(target_os = "windows")]
    assert!(aot_file_path.ends_with("example_aot_fibonacci.dll"));

    let mut wasi_module = WasiModule::create(None, None, None).unwrap();
    let mut instances = HashMap::new();
    instances.insert(wasi_module.name().to_string(), wasi_module.as_mut());

    let mut vm = Vm::new(Store::new(Some(&config), instances).unwrap());
    let module = Module::from_file(Some(&config), &aot_file_path).unwrap();

    // call the wasm function `fib` with the parameter 5
    vm.register_module(Some("fib"), module).unwrap();
    let res = vm.run_func(Some("fib"), "fib", params!(5)).unwrap();
    println!("fib(5): {}", res[0].to_i32());

    // remove the generated aot file
    assert!(std::fs::remove_file(&aot_file_path).is_ok());

    Ok(())
}
