use wasmedge_sdk::{params, wat2wasm, VmBuilder, WasmVal, NeverType};

#[cfg_attr(test, test)]
fn main() -> anyhow::Result<()> {
    let wasm_bytes = wat2wasm(
        r#"
        (module
        (type $mem_size_t (func (result i32)))
        (type $get_at_t (func (param i32) (result i32)))
        (type $set_at_t (func (param i32) (param i32)))

        (memory $mem 1)

        (func $get_at (type $get_at_t) (param $idx i32) (result i32)
            (i32.load (local.get $idx)))

        (func $set_at (type $set_at_t) (param $idx i32) (param $val i32)
            (i32.store (local.get $idx) (local.get $val)))

        (func $mem_size (type $mem_size_t) (result i32)
            (memory.size))

        (export "get_at" (func $get_at))
        (export "set_at" (func $set_at))
        (export "mem_size" (func $mem_size))
        (export "memory" (memory $mem)))
        "#
        .as_bytes(),
    )?;

    // create a Vm instance and register the module into it
    let mut vm = VmBuilder::new()
        .build::<NeverType>()?
        .register_module_from_bytes("extern", &wasm_bytes)?;

    // get the module instance named "extern"
    let extern_instance = vm.named_module("extern")?;

    // get the exported memory instance named "memory"
    let mut memory = extern_instance.memory("memory")?;

    // check memory size
    println!("The memory size (in pages): {}", memory.page());
    println!("The data size (in bytes): {}", memory.size());

    // grow memory size
    memory.grow(2)?;
    println!(
        "The memory size (in pages) after growing memory additional 2 pages: {}",
        memory.page()
    );
    println!(
        "The data size (in bytes) after growing memory additional 2 pages: {}",
        memory.size()
    );

    // get the exported functions: "set_at" and "get_at"
    let set_at = extern_instance.func("set_at")?;
    let get_at = extern_instance.func("get_at")?;

    // call the exported function named "set_at"
    let mem_addr = 0x2220;
    let val = 0xFEFEFFE;
    println!("Set {:#X} at the memory address {:#X}", val, mem_addr);
    set_at.run(vm.executor_mut(), params!(mem_addr, val))?;

    // call the exported function named "get_at"
    let returns = get_at.run(vm.executor_mut(), params!(mem_addr))?;
    println!(
        "Retrieve the value at the memory address {:#X}: {:#X}",
        mem_addr,
        returns[0].to_i32()
    );

    // call the exported function named "set_at"
    let page_size = 0x1_0000;
    let mem_addr = (page_size * 2) - std::mem::size_of_val(&val) as i32;
    let val = 0xFEA09;
    println!("Set {:#X} at the memory address {:#X}", val, mem_addr);
    set_at.run(vm.executor_mut(), params!(mem_addr, val))?;

    // call the exported function named "get_at"
    let returns = get_at.run(vm.executor_mut(), params!(mem_addr))?;
    println!(
        "Retrieve the value at the memory address {:#X}: {:#X}",
        mem_addr,
        returns[0].to_i32()
    );

    Ok(())
}
