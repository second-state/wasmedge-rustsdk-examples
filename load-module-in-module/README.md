# Function Dependecies between Wasm Modules

This example demonstrates the function dependencies between different wasm modules. That is, a wasm function defined in one wasm module can be called by another wasm module.

This example consists of three projects:

- [`alice-wasm-lib`](wasm-lib) is a wasm library, in which a function named `add` is defined.

- [`bob-wasm-lib`](wasm-lib) is a wasm library, in which a function named `real_add` is defined. This function is called by the function `add` defined in `alice-wasm-lib`.

- [`host-app`](host-app) is a host application that loads the wasm binaries generated from `alice-wasm-lib` and `bob-wasm-lib`, and runs the wasm function `add` exported by `alice` over WasmEdge Runtime.

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
  cd wasmedge-rustsdk-examples/load-module-in-module
  ```

- Build `alice-wasm-lib`

  ```bash
  cargo build -p alice-wasm-lib --target wasm32-wasi --release
  ```

  If the command runs successfully, `alice-wasm-lib.wasm` can be found in the directory of `./target/wasm32-wasi/release/`.

- Build `bob-wasm-lib`

  ```bash
  cargo build -p bob-wasm-lib --target wasm32-wasi --release
  ```

  If the command runs successfully, `bob-wasm-lib.wasm` can be found in the directory of `./target/wasm32-wasi/release/`.

- Build and run `host-app`

  ```bash
  cargo run -p host-app -- 2 3
  ```

  If the command runs successfully, then the following message is printed out on the screen:

  ```bash
  args: ["target/debug/host-app", "2", "3"]
  add(2, 3) = 5
  ```
