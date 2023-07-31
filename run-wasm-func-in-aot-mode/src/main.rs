use wasmedge_sdk::{
    config::{
        CommonConfigOptions, CompilerConfigOptions, ConfigBuilder, HostRegistrationConfigOptions,
    },
    params, wat2wasm, Compiler, CompilerOutputFormat, VmBuilder, WasmVal,
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
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
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

    let mut vm = VmBuilder::new().with_config(config).build()?;

    // call the wasm function `fib` with the parameter 5
    let res = vm.run_func_from_file(&aot_file_path, "fib", params!(5))?;
    println!("fib(5): {}", res[0].to_i32());

    // remove the generated aot file
    assert!(std::fs::remove_file(&aot_file_path).is_ok());

    Ok(())
}
