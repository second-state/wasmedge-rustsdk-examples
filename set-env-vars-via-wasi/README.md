# Call a Wasm Library from Host

This example demonstrates how to set environment variables and arguments via WasmEdge `wasi` interface.

This example consists of two projects:

- [`print-env-vars`](print-env-vars) is a wasm library, in which a function accessing the environment variables is defined.

- [`set-env-vars-via-wasi`](set-env-vars-via-wasi) is a host application that defines a group of arguments and environment variables and sets them via WasmEdge `wasi` interface; then,loads the wasm binary generated from `print-env-vars`, and runs the wasm function exported by the wasm library to print the pre-defined arguments and environment variables.

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
  git clone git@github.com:apepkuss/wasmedge-rust-examples.git
  cd wasmedge-rust-examples/set-env-vars-via-wasi
  ```

- Build `print-env-vars`

  ```bash
  cargo build -p print-env-vars --target wasm32-wasi --release
  ```

  If the command runs successfully, `print_env_vars.wasm` can be found in the directory of `./target/wasm32-wasi/release/`.

- Build and run `set-env-vars-via-wasi`

  ```bash
  // in wasmedge-rust-examples/set-env-vars-via-wasi
  cargo run -p set-env-vars-via-wasi -- ./target/wasm32-wasi/release/print_env_vars.wasm
  ```

  If the command runs successfully, then the following message is printed out on the screen:

  ```bash
  The env vars are as follows.
  ENV1: VAL1
  ENV2: VAL2
  ENV3: VAL3
  The args are as follows.
  arg1
  arg2
  ```
