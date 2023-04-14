# Cancel Async Host Functions in Exeuction

This example demonstrates how to cancel an async host function when the specified time is out.

Now let's build and run this example.

- Install `rustup` and `Rust`

  Go to the [official Rust webpage](https://www.rust-lang.org/tools/install) and follow the instructions to install `rustup` and `Rust`.

  > It is recommended to use Rust 1.65 or above in the stable channel.

- Install `libwasmedge`

  Refer to the [Quick Install](https://wasmedge.org/book/en/quick_start/install.html#quick-install) section of WasmEdge Runtime Book to install `libwasmedge`. Or, use the following command directly

  ```bash
  // The command will create a directory `$HOME/.wasmedge`
  ./install_libwasmedge.sh -v 0.12.0-alpha.2

  source $HOME/.wasmedge/env
  ```

  > For users in China mainland (中国大陆地区), try the following command to install `libwasmedge` if failed to run the command above
  >
  > ```bash
  > ./install_libwasmedge_zh.sh -v 0.12.0-alpha.2
  > source $HOME/.wasmedge/env
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
