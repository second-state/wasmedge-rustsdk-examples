#[no_mangle]
pub fn fib(n: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;

    for _ in 1..n {
        let old = a;
        a = b;
        b += old;
    }
    b
}
