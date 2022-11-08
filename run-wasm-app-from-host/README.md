
# Run wasm app from host

As a typical application development process, we put some code in the `main` function, and then compile it to an execution binary and run it to get the result. In this example, we will something differnt. We will compile the application code to a wasm binary, and then run it as a wasm module over WasmEdge runtime.

For the purpose of demonstration, this example consists of two projects:

- [`wasm-app`](wasm-app) is a common application. Instead of compiled to an execution binary, it is compiled to a wasm binary.
- [`run-wasm-app`](run-wasm-app) is a host application. With WasmEdge Rust SDK, it loads the wasm binary from `wasm-app` and executes it as a wasm module.

## Build

### Build wasm app

```bash
cargo build -p wasm-app --target wasm32-wasi --release
```

### Build & run host app

```bash
cargo build -p host-app --release
```

Run the host app

```bash
./target/release/run-wasm-app ./target/wasm32-wasi/release/wasm-app.wasm
```
