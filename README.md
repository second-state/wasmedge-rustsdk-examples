# Examples of WasmEdge Rust SDK

> All examples have been verified on Ubuntu-20.04/22.04.

- [Examples of WasmEdge Rust SDK](#examples-of-wasmedge-rust-sdk)
  - [Wasm App](#wasm-app)
    - [[Example: Run a wasm app from host](run-wasm-app-from-host)](#example-run-a-wasm-app-from-host)
  - [Wasm Library](#wasm-library)
    - [[Example: Define a wasm library](call-wasm-lib-from-host)](#example-define-a-wasm-library)
    - [[Example: Use extern functions in a wasm library](call-wasm-lib-with-external-deps)](#example-use-extern-functions-in-a-wasm-library)
    - [[Example: Dependencies between wasm libraries](load-module-in-module)](#example-dependencies-between-wasm-libraries)
  - [Host Functions](#host-functions)
    - [[Example: Define a host function](define-host-func)](#example-define-a-host-function)
    - [[Example: Define a host function with host context data](define-host-func-with-host-data)](#example-define-a-host-function-with-host-context-data)
    - [[Example: Define a host function with string arguments](define-host-func-with-string-args)](#example-define-a-host-function-with-string-arguments)
    - [[Example: Arguments of non-standard wasm types in wasm function (wasmedge-bindgen)](wasmedge-bindgen)](#example-arguments-of-non-standard-wasm-types-in-wasm-function-wasmedge-bindgen)
  - [Memory and Table](#memory-and-table)
    - [[Example: Manipulate WebAssembly linear memory](manipulate-linear-memory)](#example-manipulate-webassembly-linear-memory)
    - [[Example: Use WebAssembly `Table` and `FuncRef` to invoke host functions](table-and-funcref)](#example-use-webassembly-table-and-funcref-to-invoke-host-functions)
  - [AOT](#aot)
    - [[Example: Call wasm library in AOT mode](run-wasm-func-in-aot-mode)](#example-call-wasm-library-in-aot-mode)
  - [WASI](#wasi)
    - [[Example: Set environment variables and arguments via WasmEdge `wasi` interface](set-env-vars-via-wasi)](#example-set-environment-variables-and-arguments-via-wasmedge-wasi-interface)
    - [[Example: Set environment variables, arguments and preopen-dirs via `async-wasi` interface](async-wasi)](#example-set-environment-variables-arguments-and-preopen-dirs-via-async-wasi-interface)
    - [[Example: Retrieve the socket address via `async-wasi` interface](async-wasi-socket-addr)](#example-retrieve-the-socket-address-via-async-wasi-interface)
  - [WASI-NN](#wasi-nn)
    - [[Example: AI inference over WasmEdge PyTorch Plugin](object-detection-via-wasinn)](#example-ai-inference-over-wasmedge-pytorch-plugin)
  - [Plugin Development](#plugin-development)
    - [[Example: Create a simple plugin](simple-plugin)](#example-create-a-simple-plugin)
  - [Multi-threading](#multi-threading)
    - [[Example: Multi-threaded parallel execution with memory sharing](multi-threaded-parallel)](#example-multi-threaded-parallel-execution-with-memory-sharing)
  - [Async\await](#asyncawait)
    - [[Example: Define async host functions](define-async-host-func)](#example-define-async-host-functions)
    - [[Example: Define async host functions with host context data](define-async-host-func-with-host-data)](#example-define-async-host-functions-with-host-context-data)
    - [[Example: Call a wasm function in asynchromous way](async-wasi)](#example-call-a-wasm-function-in-asynchromous-way)

## Wasm App

### [Example: Run a wasm app from host](run-wasm-app-from-host)

This example demonstrates how to create a WebAssembly application and run it as a wasm module over WasmEdge Runtime.

## Wasm Library

### [Example: Define a wasm library](call-wasm-lib-from-host)

This example demonstrates how to define a wasm library and call the APIs exported from the wasm library in WasmEdge Runtime.

### [Example: Use extern functions in a wasm library](call-wasm-lib-with-external-deps)

This example demonstrates how to use external functions in a wasm library, and how to call such a wasm library with WasmEdge Rust SDK.

### [Example: Dependencies between wasm libraries](load-module-in-module)

This example demonstrates how a wasm function defined in one wasm library can be imported and called by another wasm library.

## Host Functions

### [Example: Define a host function](define-host-func)

This example demonstrates how to define a native function as a host function with `#[host_function]` macro, and register it into a WasmEdge `Vm` and run it over WasmEdge Runtime.

### [Example: Define a host function with host context data](define-host-func-with-host-data)

This example demonstrates how to define a native function as a host function with `#[host_function]` macro. A host context data is used in the host function.

### [Example: Define a host function with string arguments](define-host-func-with-string-args)

This example demonstrates how to define a host function that takes two arguments of Rust built-in `String` or string slice type.

### [Example: Arguments of non-standard wasm types in wasm function (wasmedge-bindgen)](wasmedge-bindgen)

This example demonstrates how to define wasm functions, of which the arguments are of Rust built-in types instead of WebAssembly types.

## Memory and Table

### [Example: Manipulate WebAssembly linear memory](manipulate-linear-memory)

This example demonstrates how to manipulate linear memory.

### [Example: Use WebAssembly `Table` and `FuncRef` to invoke host functions](table-and-funcref)

This example demonstrates how to use WebAssembly `Table` and `FuncRef` together to invoke host functions.

## AOT

### [Example: Call wasm library in AOT mode](run-wasm-func-in-aot-mode)

This example demonstrates how to generate a AOT file with WasmEdge AOT compiler, and then load the file and run the wasm function in AOT mode.

## WASI

### [Example: Set environment variables and arguments via WasmEdge `wasi` interface](set-env-vars-via-wasi)

This example demonstrates how to set environment variables and arguments via WasmEdge `wasi` interface.

### [Example: Set environment variables, arguments and preopen-dirs via `async-wasi` interface](async-wasi)

This example demonstrates how to call a wasm function in asynchronous way. Also, it presents the usage of `async-wasi` interface.

### [Example: Retrieve the socket address via `async-wasi` interface](async-wasi-socket-addr)

This example presents how to retrieve the socket address information asynchronously.

## WASI-NN

### [Example: AI inference over WasmEdge PyTorch Plugin](object-detection-via-wasinn)

This example demonstrates how to perform an object detection task over WasmEdge Runtime.

## Plugin Development

### [Example: Create a simple plugin](simple-plugin)

This example demonstrates how to create a simple plugin with WasmEdge Rust SDK.

### [Example: Run a plugin with `Executor`](simple-plugin-ext)

This example demonstrates how to create a simple plugin with WasmEdge Rust SDK, and run it with WasmEdge Executor.

## Multi-threading

### [Example: Multi-threaded parallel execution with memory sharing](multi-threaded-parallel)

This example demonstrates how to use WasmEdge Rust SDK to create multiple threads to help us render [Mandelbrot set](https://en.wikipedia.org/wiki/Mandelbrot_set) in parallel, which is a compute-intensive workload. This example also present sharing the image memory between threads while rendering the image parallelly.

## Async\await

### [Example: Define async host functions](define-async-host-func)

This example demonstrates how to define a native async function as an async host function with `#[async_host_function]` macro, and register it into a WasmEdge `Vm` and run it over WasmEdge Runtime.

### [Example: Define async host functions with host context data](define-async-host-func-with-host-data)

This example demonstrates how to define a native async function as an async host function with `#[async_host_function]` macro. A host context data is used in the host function.

### [Example: Call a wasm function in asynchromous way](async-wasi)

This example demonstrates how to call a wasm function in asynchronous way.
