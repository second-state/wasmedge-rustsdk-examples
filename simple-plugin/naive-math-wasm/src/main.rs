mod plugin {
    #[link(wasm_import_module = "naive-math")]
    extern "C" {
        pub fn add(x: i32, y: i32) -> i32;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let x: i32 = args[1].parse().unwrap();
    let y: i32 = args[2].parse().unwrap();

    let res = unsafe { plugin::add(x, y) };
    println!("{x} + {y} = {res}");
}
