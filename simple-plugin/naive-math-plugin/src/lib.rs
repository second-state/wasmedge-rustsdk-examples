use wasmedge_sdk::{
    error::HostFuncError,
    host_function,
    plugin::{ffi, PluginDescriptor, PluginVersion},
    Caller, ImportObjectBuilder, NeverType, ValType, WasmValue,
};

// A native function to be wrapped as a host function
#[host_function]
fn real_add<T>(
    _: Caller,
    input: Vec<WasmValue>,
    _ctx: Option<&mut T>,
) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("Welcome! This is NaiveMath plugin.");

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

/// Defines Plugin module instance
unsafe extern "C" fn create_test_module(
    _arg1: *const ffi::WasmEdge_ModuleDescriptor,
) -> *mut ffi::WasmEdge_ModuleInstanceContext {
    let module_name = "naive-math";
    let import = ImportObjectBuilder::new()
        // add a function
        .with_func::<(i32, i32), i32, NeverType>("add", real_add, None)
        .expect("failed to create host function")
        .build(module_name)
        .expect("failed to create import object");

    import.as_raw_ptr() as *mut _
}

/// Defines PluginDescriptor
#[export_name = "WasmEdge_Plugin_GetDescriptor"]
pub extern "C" fn plugin_hook() -> *const ffi::WasmEdge_PluginDescriptor {
    let name = "naive_math_plugin";
    let desc = "this is naive math plugin";
    let version = PluginVersion::new(0, 0, 0, 0);
    let plugin_descriptor = PluginDescriptor::new(name, desc, version)
        .expect("Failed to create plugin descriptor")
        .add_module_descriptor(
            "naive_math_module",
            "this is naive math module",
            Some(create_test_module),
        )
        .expect("Failed to add module descriptor");

    plugin_descriptor.as_raw_ptr()
}
