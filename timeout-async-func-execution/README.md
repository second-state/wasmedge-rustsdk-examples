# Cancel Async Host Functions in Exeuction

This example demonstrates how to cancel an async host function when the specified time is out.

Now let's build and run this example.

- Install `rustup` and `Rust`

  Go to the [official Rust webpage](https://www.rust-lang.org/tools/install) and follow the instructions to install `rustup` and `Rust`.

  > It is recommended to use Rust 1.66 or above in the stable channel.

  Then, add `wasm32-wasi` target to the Rustup toolchain:

  ```bash
  rustup target add wasm32-wasi
  ```

- Install WasmEdge Runtime

  Refer to the [Quick Install](https://wasmedge.org/book/en/quick_start/install.html#quick-install) section of WasmEdge Runtime Book to install WasmEdge Runtime. Or, use the following command directly

  ```bash
  # NOTICE that the installation script needs `sudo` access

  # install wasmedge to the directory /usr/local/
  curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash -s -- -v 0.12.0 -p /usr/local
  ```

  > For users in China mainland (中国大陆地区), try the following command to install WasmEdge Runtime if failed to run the command above
  >
  > ```bash
  > # NOTICE that the installation script needs `sudo` access
  >
  > bash install_zh.sh -v 0.12.0 -p /usr/local
  > ```

- Download example

  ```bash
  git clone git@github.com:second-state/wasmedge-rustsdk-examples.git
  cd wasmedge-rust-examples/timeout-async-func-execution
  ```

- Build & run

  ```bash
  cargo run
  ```

  If the command runs successfully, then the following result is printed out on the screen:

  ```bash
  0: Hello, world!
  1: Hello, world!
  2: Hello, world!
  3: Hello, world!
  thread 'tokio-runtime-worker' panicked at 'fiber dropped without finishing', /root/.cargo/registry/src/mirrors.tuna.tsinghua.edu.  cn-df7c3c540f42cdbd/wasmtime-fiber-6.0.1/src/lib.rs:164:9
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
  err
  sleeping 5 secs in main thread
  done
  ```

  Note that the panic message above is from `wasmetime-fiber` (see [wasmtime issue 6212](https://github.com/bytecodealliance/wasmtime/issues/6212)).
