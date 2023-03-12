# Arguments of non-standard wasm types in wasm function

This example demonstrates how to make use of `wasmedge-bindgen` and `wasmedge-bindgen-macro` crates to define wasm functions that have arguments of Rust built-in types, including numeric types (such as u8, i32, f32), `bool`,  `String`, `Vec<T>`( `T` is one of `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64` and `u64`).

This example consists of two projects:

- [`wasm-lib`](wasm-lib) is a wasm library, in which wasm functions are defined.

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
  cd wasmedge-rustsdk-examples/wasmedge-bindgen
  ```

- Build `wasm-lib`

  ```bash
  cargo build -p wasm-lib --target wasm32-wasi --release
  ```

  If the command runs successfully, `wasm-lib.wasm` can be found in the directory of `./target/wasm32-wasi/release/`.

- Build and run `call-wasm-lib`

  ```bash
  cargo run -p call-wasm-lib -- ./target/wasm32-wasi/release/wasm_lib.wasm 5
  ```

  If the command runs successfully, then the following message is printed out on the screen:

  ```bash
    Run bindgen -- say: hello bindgen funcs test 5
    Run bindgen -- say FAILED abc
    Run bindgen -- create_line: {"points":[{"x":2.5,"y":7.8},{"x":2.5,"y":5.8}],"valid":true,"length":2.0,"desc":"A thin red line"}
    Run bindgen -- obfusticate: N dhvpx oebja sbk whzcf bire gur ynml qbt
    Run bindgen -- sha3_digest: [87, 27, 231, 209, 189, 105, 251, 49, 159, 10, 211, 250, 15, 159, 154, 181, 43, 218, 26, 141, 56, 199, 25, 45, 60, 10, 20, 163, 54, 211, 195, 203]
    Run bindgen -- keccak_digest: [126, 194, 241, 200, 151, 116, 227, 33, 216, 99, 159, 22, 107, 3, 177, 169, 216, 191, 114, 156, 174, 193, 32, 159, 246, 228, 245, 133, 52, 75, 55, 27]
  ```
