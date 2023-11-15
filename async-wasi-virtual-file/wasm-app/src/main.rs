fn main() {
    let mut stdin_str = String::new();
    let _ = std::io::stdin().read_line(&mut stdin_str).unwrap();
    println!("read '{}' from stdin", stdin_str);
}
