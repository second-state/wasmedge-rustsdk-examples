# Call a Wasm Library from Host

This example demonstrates how to define a wasm library that holds a function to be exported, and then run the wasm function over WasmEdge Runtime.

This example consists of two projects:

- [`wasm-lib`](wasm-lib) is a wasm library, in which a function is defined.

- [`call-wasm-lib`](call-wasm-lib) is a host application that loads the wasm binary generated from `wasm-lib`, and runs the wasm function exported by the wasm library over WasmEdge Runtime.

Now let's build and run this example.

- Install `rustup` and `Rust`

  Go to the [official Rust webpage](https://www.rust-lang.org/tools/install) and follow the instructions to install `rustup` and `Rust`.

  > It is recommended to use Rust 1.63 or above in the stable channel.

  Then, add `wasm32-wasi` target to the Rustup toolchain:

  ```bash
  rustup target add wasm32-wasi
  ```

- Install `libwasmedge`

  Refer to the [Quick Install](https://wasmedge.org/book/en/quick_start/install.html#quick-install) section of WasmEdge Runtime Book to install `libwasmedge`. Or, use the following command directly

  ```bash
  // The command will create a directory `$HOME/.wasmedge`
  ./install_libwasmedge.sh

  source $HOME/.wasmedge/env
  ```

  > For users in China mainland (中国大陆地区), try the following command to install `libwasmedge` if failed to run the command above
  >
  > ```bash
  > ./install_libwasmedge_zh.sh
  > source $HOME/.wasmedge/env
  > ```

  > To install a specific version of `libwasmedge`, use `-v` option. For example, the following command installs `libwasmedge 0.11.2` to `$HOME/.wasmedge`
  >
  > ```bash
  > ./install_libwasmedge.sh -v 0.11.2
  > source $HOME/.wasmedge/env
  > ```

- Download example

  ```bash
  git clone git@github.com:second-state/wasmedge-rustsdk-examples.git
  cd wasmedge-rust-examples/call-wasm-lib-from-host
  ```

- Build `wasm-lib`

  ```bash
  cargo build -p wasm-lib --target wasm32-wasi --release
  ```

  If the command runs successfully, `wasm-lib.wasm` can be found in the directory of `./target/wasm32-wasi/release/`.

- Build and run `call-wasm-lib`

  ```bash
  // in the directory of call-wasm-lib-from-host
  cargo run -p call-wasm-lib -- ./target/wasm32-wasi/release/wasm_lib.wasm 5
  ```

  If the command runs successfully, then the following message is printed out on the screen:

  ```bash
  fib(5) = 8
  ```
