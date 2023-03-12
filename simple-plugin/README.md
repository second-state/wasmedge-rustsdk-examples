
# Create a math plugin

This example demonstrates how to create a simple plugin with WasmEdge Rust SDK. This example consists of two projects:

- `naive-math-plugin`: a simple plugin that exports a function `add` to add two integers.

- `naive-math-wasm`: a wasm module that imports the function `add` from the plugin and calls it.

> Notice that WasmEdge Plugin is only available on Linux OS. In this example, all steps are validated on **Ubuntu-20.04**

Now let's build and run this example.

- Install `rustup` and `Rust`

  Go to the [official Rust webpage](https://www.rust-lang.org/tools/install) and follow the instructions to install `rustup` and `Rust`.

  > It is recommended to use Rust 1.63 or above in the stable channel.

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
  cd wasmedge-rustsdk-examples/simple-plugin
  ```

- Build `naive-math-plugin`

  ```bash
  cargo build -p naive-math-plugin --release
  ```

  If the command runs successfully, `libnaive_math_plugin.so` can be found in the directory of `./target/release/`. Then, copy the generated `libnaive_math_plugin.so` to the default plugin sub-directory in the WasmEdge installation path. In this example, we assume that the WasmEdge installation path is `~/.wasmedge`:

  ```bash
  cp target/release/libnaive_math_plugin.so ~/.wasmedge/lib/wasmedge
  ```

- Build `naive-math-wasm`

  ```bash
  cargo build -p naive-math-wasm --target wasm32-wasi --release
  ```

  If the command runs successfully, `naive-math-wasm.wasm` can be found in the directory of `./target/wasm32-wasi/release/`.

- Run `naive-math-wasm` with WasmEdge CLI

  ```bash
  wasmedge target/wasm32-wasi/release/naive-math-wasm.wasm 1 2
  ```
  
  If the command runs successfully, then the following message is printed out on the screen:

  ```bash
  Welcome! This is NaiveMath plugin.
  1 + 2 = 3
  ```
