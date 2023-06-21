# Examples of WasmEdge Rust SDK

> All examples have been verified on Ubuntu-20.04/22.04.

- [Examples of WasmEdge Rust SDK](#examples-of-wasmedge-rust-sdk)
  - [Wasm App](#wasm-app)
    - [Example: Run a wasm app from host](#example-run-a-wasm-app-from-host)
  - [Wasm Library](#wasm-library)
    - [Example: Define a wasm library](#example-define-a-wasm-library)
    - [Example: Use extern functions in a wasm library](#example-use-extern-functions-in-a-wasm-library)
    - [Example: Dependencies between wasm libraries](#example-dependencies-between-wasm-libraries)
  - [Host Functions](#host-functions)
    - [Example: Define and register a host function](#example-define-and-register-a-host-function)
    - [Example: Define a host function with string arguments](#example-define-a-host-function-with-string-arguments)
    - [Example: Arguments of non-standard wasm types in wasm function (wasmedge-bindgen)](#example-arguments-of-non-standard-wasm-types-in-wasm-function-wasmedge-bindgen)
  - [Memory and Table](#memory-and-table)
    - [Example: Manipulate WebAssembly linear memory](#example-manipulate-webassembly-linear-memory)
    - [Example: Use WebAssembly `Table` and `FuncRef` to invoke host functions](#example-use-webassembly-table-and-funcref-to-invoke-host-functions)
  - [AOT](#aot)
    - [Example: Call wasm library in AOT mode](#example-call-wasm-library-in-aot-mode)
  - [WASI](#wasi)
    - [Example: Set environment variables and arguments via WasmEdge `wasi` interface](#example-set-environment-variables-and-arguments-via-wasmedge-wasi-interface)
    - [Example: Set environment variables, arguments and preopen-dirs via `async-wasi` interface](#example-set-environment-variables-arguments-and-preopen-dirs-via-async-wasi-interface)
    - [Example: Retrieve the socket address via `async-wasi` interface](#example-retrieve-the-socket-address-via-async-wasi-interface)
  - [WASI-NN](#wasi-nn)
    - [Example: AI inference over WasmEdge PyTorch Plugin](#example-ai-inference-over-wasmedge-pytorch-plugin)
  - [Plugin Development](#plugin-development)
    - [Example: Create a simple plugin](#example-create-a-simple-plugin)
  - [Multi-threaded Parallelism](#multi-threaded-parallelism)
    - [Example: Multi-threaded parallel execution with memory sharing](#example-multi-threaded-parallel-execution-with-memory-sharing)
  - [Async\\await](#asyncawait)
    - [Example: Define and register async host functions](#example-define-and-register-async-host-functions)
    - [Example: Call a wasm function in asynchromous way](#example-call-a-wasm-function-in-asynchromous-way)

## Wasm App

### Example: Run a wasm app from host

This example demonstrates how to create a WebAssembly application and run it as a wasm module over WasmEdge Runtime.

- [Run wasm app from host](run-wasm-app-from-host)
  - [x] Source Code
  - [x] README
  - [x] Tutorial

## Wasm Library

### Example: Define a wasm library

This example demonstrates how to define a wasm library and call the APIs exported from the wasm library in WasmEdge Runtime.

- [Call wasm library from host](call-wasm-lib-from-host)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

### Example: Use extern functions in a wasm library

This example demonstrates how to use external functions in a wasm library, and how to call such a wasm library with WasmEdge Rust SDK.

- [Call wasm library with external dependency](call-wasm-lib-with-external-deps)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

### Example: Dependencies between wasm libraries

This example demonstrates how a wasm function defined in one wasm library can be imported and called by another wasm library.

- [Call wasm functions between wasm modules](load-module-in-module)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

## Host Functions

### Example: Define and register a host function

This example demonstrates how to define a native function as a host function with `#[host_function]` macro, and register it into a WasmEdge `Vm` and run it over WasmEdge Runtime.

- [Define host function](define-host-func)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

### Example: Define a host function with string arguments

This example demonstrates how to define a host function that takes two arguments of Rust built-in `String` or string slice type.

- [Define host function with string arguments](define-host-func-with-string-args)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

### Example: Arguments of non-standard wasm types in wasm function (wasmedge-bindgen)

This example demonstrates how to define wasm functions, of which the arguments are of Rust built-in types instead of WebAssembly types.

- [WasmEdge-Bindgen](wasmedge-bindgen)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

## Memory and Table

### Example: Manipulate WebAssembly linear memory

This example demonstrates how to manipulate linear memory.

- [Manipulate linear memory](manipulate-linear-memory)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

### Example: Use WebAssembly `Table` and `FuncRef` to invoke host functions

This example demonstrates how to use WebAssembly `Table` and `FuncRef` together to invoke host functions.

- [Table and FuncRef](table-and-funcref)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

## AOT

### Example: Call wasm library in AOT mode

This example demonstrates how to generate a AOT file with WasmEdge AOT compiler, and then load the file and run the wasm function in AOT mode.

- [Use WasmEdge AOT Compiler](run-wasm-func-in-aot-mode)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

## WASI

### Example: Set environment variables and arguments via WasmEdge `wasi` interface

This example demonstrates how to set environment variables and arguments via WasmEdge `wasi` interface.

- [Set env vars via wasi](set-env-vars-via-wasi)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

### Example: Set environment variables, arguments and preopen-dirs via `async-wasi` interface

This example demonstrates how to call a wasm function in asynchronous way. Also, it presents the usage of `async-wasi` interface.

- [Async wasi](async-wasi)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

### Example: Retrieve the socket address via `async-wasi` interface

This example presents how to retrieve the socket address information asynchronously.

- [Socket addr](async-wasi-socket-addr)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

## WASI-NN

### Example: AI inference over WasmEdge PyTorch Plugin

This example demonstrates how to perform an object detection task over WasmEdge Runtime.

- [Object detection via WASI-NN Plugin PyTorch Backend](object-detection-via-wasinn)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

## Plugin Development

### Example: Create a simple plugin

This example demonstrates how to create a simple plugin with WasmEdge Rust SDK.

- [Create a math plugin](simple-plugin)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

## Multi-threaded Parallelism

### Example: Multi-threaded parallel execution with memory sharing

This example demonstrates how to use WasmEdge Rust SDK to create multiple threads to help us render [Mandelbrot set](https://en.wikipedia.org/wiki/Mandelbrot_set) in parallel, which is a compute-intensive workload. This example also present sharing the image memory between threads while rendering the image parallelly.

- [Run in multi-threaded parallel](multi-threaded-parallel)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

## Async\await

### Example: Define and register async host functions

This example demonstrates how to define a native async function as an async host function with `#[async_host_function]` macro, and register it into a WasmEdge `Vm` and run it over WasmEdge Runtime.

- [Define async host functions](define-async-host-func)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial

### Example: Call a wasm function in asynchromous way

This example demonstrates how to call a wasm function in asynchronous way.

- [Async wasi](async-wasi)
  - [x] Source Code
  - [x] README
  - [ ] Tutorial
