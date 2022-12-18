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

### Example 2-1: Use extern functions in a wasm library

This example demonstrates how to use external functions in a wasm library, and how to call such a wasm library with WasmEdge Rust SDK.

- [Call wasm library with external dependency](call-wasm-lib-with-external-deps/README.md)
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

### Example 4-1: Define a host function with string arguments

This example demonstrates how to define a host function that takes two arguments of Rust built-in `String` or string slice type.

- [Define host function with string arguments](define-host-func-with-string-args/README.md)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

## Example 5: Define and register async host functions

This example demonstrates how to define a native async function as an async host function with `#[async_host_function]` macro, and register it into a WasmEdge `Vm` and run it over WasmEdge Runtime.

- [Define async host functions](define-async-host-func/README.md)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

## Example 6: AI inference over WasmEdge WASI-NN Plugin (PyTroch)

This example demonstrates how to perform an object detection task over WasmEdge Runtime.

- [Object detection via WASI-NN Plugin PyTorch Backend](object-detection-via-wasinn/README.md)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

## Example 7: Manipulate WebAssembly linear memory

This example demonstrates how to manipulate linear memory.

- [Manipulate linear memory](manipulate-linear-memory/README.md)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

## Example 8: Use WebAssembly `Table` and `FuncRef` to invoke host functions

This example demonstrates how to use WebAssembly `Table` and `FuncRef` together to invoke host functions.

- [Table and FuncRef](table-and-funcref/README.md)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

## Example 9: Run wasm functions in AOT mode

This example demonstrates how to generate a AOT file with WasmEdge AOT compiler, and then load the file and run the wasm function in AOT mode.

- [Use WasmEdge AOT Compiler](run-wasm-func-in-aot-mode/README.md)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial
