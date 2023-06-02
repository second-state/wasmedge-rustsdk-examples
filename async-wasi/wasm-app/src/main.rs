fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[wasm-app] enter >>>>>");

    let val = std::env::var("ENV").expect("failed to get environment variable: ENV");
    println!("[wasm-app] ENV={val}");

    println!("[wasm-app] entry in the current guest preopened dir:");
    let path = std::env::current_dir().expect("failed to get current directory");
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            println!("  {:?}", entry.path());
        }
    }

    for i in 0..10 {
        println!("[wasm-app] say hello: {i}");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println!("[wasm-app] exit <<<<<");

    Ok(())
}
