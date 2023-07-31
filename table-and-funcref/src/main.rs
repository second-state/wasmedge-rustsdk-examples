use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder},
    error::HostFuncError,
    host_function, params,
    types::Val,
    Caller, Func, ImportObjectBuilder, NeverType, RefType, Table, TableType, ValType, VmBuilder,
    WasmVal, WasmValue,
};

#[host_function]
fn real_add<T>(_: Caller, input: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    if input.len() != 2 {
        return Err(HostFuncError::User(1));
    }

    let a = if input[0].ty() == ValType::I32 {
        input[0].to_i32()
    } else {
        return Err(HostFuncError::User(2));
    };

    let b = if input[1].ty() == ValType::I32 {
        input[1].to_i32()
    } else {
        return Err(HostFuncError::User(3));
    };

    let c = a + b;

    Ok(vec![WasmValue::from_i32(c)])
}

#[cfg_attr(test, test)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create a table instance
    let result = Table::new(TableType::new(RefType::FuncRef, 10, Some(20)));
    assert!(result.is_ok());
    let table = result.unwrap();

    // create an import object
    let import = ImportObjectBuilder::new()
        .with_table("my-table", table)
        .build::<NeverType>("extern", None)?;

    // create a Vm and register the import object into it
    let config = ConfigBuilder::new(CommonConfigOptions::default()).build()?;
    let mut vm = VmBuilder::new().with_config(config).build()?;
    vm.register_import_module(&import)?;

    // get the module instance named "extern"
    let extern_instance = vm.named_module("extern")?;

    // get the exported table instance from the "extern" module instance
    let mut table = extern_instance
        .table("my-table")
        .expect("Not found table instance named 'my-table'");

    // create a host function
    let host_func = Func::wrap::<(i32, i32), i32, NeverType>(real_add, None)?;

    // store the reference to host_func at the given index of the table instance
    table.set(3, Val::FuncRef(Some(host_func.as_ref())))?;

    // retrieve the function reference at the given index of the table instance
    let value = table.get(3)?;
    if let Val::FuncRef(Some(func_ref)) = value {
        // get the function type by func_ref
        let func_ty = func_ref.ty();

        // arguments
        println!("num of argument types: {}", func_ty.args_len());
        let param_tys = func_ty
            .args()
            .expect("Failed to get argument types from func type");
        println!("types of arguments: {:?}", param_tys);

        // returns
        println!("num of return types: {}", func_ty.args_len());
        let return_tys = func_ty
            .returns()
            .expect("Failed to get return types from func type");
        println!("types of return: {:?}", return_tys);

        // call the function by func_ref
        let returns = func_ref.run(vm.executor_mut(), params!(1, 2))?;
        println!("real_add(1, 2) = {}", returns[0].to_i32());
    }

    Ok(())
}
