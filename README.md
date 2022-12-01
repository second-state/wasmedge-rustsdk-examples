# Examples of WasmEdge Rust SDK

## Example 1: Run a wasm app from host

This example demonstrates how to create a WebAssembly application and run it as a wasm module over WasmEdge Runtime.

- [Run wasm app from host](run-wasm-app-from-host/README.md)
  - [x] Source Code
  - [x] README
  - [x] Tutorial

## Example 2: Call a wasm library from host

This example demonstrates how to define a wasm library and call the APIs exported from the wasm library in WasmEdge Runtime.

- [Call wasm library from host](call-wasm-lib-from-host/README.md)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

## Example 3: Arguments of non-standard wasm types in wasm function

This example demonstrates how to define wasm functions, of which the arguments are of Rust built-in types instead of WebAssembly types.

- [WasmEdge-Bindgen](wasmedge-bindgen/README.md)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

## Example 4: Define and register a host function

This example demonstrates how to define a native function as a host function with `#[host_function]` macro, and register it into a WasmEdge `Vm` and run it over WasmEdge Runtime.

- [Define host function](define-host-func/README.md)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

## Example 5: Define and register async host functions

- [Define async host functions](define-async-host-func/README.md)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

## Example 6: AI inference over WasmEdge WASI-NN Plugin (PyTroch)

- [Object detection via WASI-NN Plugin PyTorch Backend](object-detection-via-wasinn/README.md)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial
