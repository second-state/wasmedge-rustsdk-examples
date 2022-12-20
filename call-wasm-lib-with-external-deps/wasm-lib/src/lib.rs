#[no_mangle]
pub fn add(left: i32, right: i32) -> i32 {
    unsafe { my_math_lib::real_add(left, right) }
}

pub mod my_math_lib {
    #[link(wasm_import_module = "my_math_lib")]
    extern "C" {
        pub fn real_add(x: i32, y: i32) -> i32;
    }
}
