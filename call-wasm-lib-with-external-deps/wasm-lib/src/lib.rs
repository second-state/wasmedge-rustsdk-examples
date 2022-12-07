
extern "C" { fn real_add(x: i32, y: i32) -> i32;}

#[no_mangle]
pub fn add(left: i32, right: i32) -> i32 {
    unsafe { real_add(left, right) } 
}
