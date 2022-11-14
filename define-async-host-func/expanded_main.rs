#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use wasmedge_sdk::{
    async_host_function, error::HostFuncError, params, Caller, ImportObjectBuilder, Vm, WasmValue,
};
fn say_hello(
    frame: wasmedge_sdk::CallingFrame,
    _args: Vec<WasmValue>,
    _data: *mut std::os::raw::c_void,
) -> Box<(dyn std::future::Future<Output = Result<Vec<WasmValue>, HostFuncError>> + Send + 'static)>
{
    Box::new(async move {
        let caller = Caller::new(frame);
        {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(&["Hello, world!\n"], &[]));
            };
            Ok(::alloc::vec::Vec::new())
        }
    })
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = async {
        let import = ImportObjectBuilder::new()
            .with_func_async::<(), ()>("say_hello", say_hello)?
            .build("extern")?;
        let vm = Vm::new(None)?.register_import_module(import)?;
        tokio::spawn(async move {
            let _ = vm
                .run_func_async(Some("extern"), "say_hello", {
                    let mut temp_vec = ::alloc::vec::Vec::new();
                    temp_vec
                })
                .await
                .unwrap();
        })
        .await?;
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(&["main thread\n"], &[]));
        };
        Ok(())
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
