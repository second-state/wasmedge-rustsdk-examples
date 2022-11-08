
# Run wasm app from host

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
