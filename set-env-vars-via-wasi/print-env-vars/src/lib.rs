use std::env;

#[no_mangle]
pub fn print_env() {
    println!("The env vars are as follows.");
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }

    println!("The args are as follows.");
    for argument in env::args() {
        println!("{}", argument);
    }

    // write to the pre-opened directory
    std::fs::write("hello.txt", "Hi").expect("Unable to write file");
}
