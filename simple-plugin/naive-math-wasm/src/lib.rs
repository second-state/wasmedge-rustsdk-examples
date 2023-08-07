mod plugin {
    #[link(wasm_import_module = "naive-math")]
    extern "C" {
        pub fn add(x: i32, y: i32) -> i32;
    }
}

#[no_mangle]
pub fn add(x: i32, y: i32) -> i32 {
    unsafe { plugin::add(x, y) }
}
