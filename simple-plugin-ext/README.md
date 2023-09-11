
# Create a math plugin

This example demonstrates how to create a simple plugin with WasmEdge Rust SDK. Different from [naive-math-plugin](../simple-plugin/), this example uses `Executor` and `Store` to load, register, and run a plugin.

This example consists of three projects:

- `naive-math-plugin`: a simple plugin that exports a function `add` to add two integers.

- `naive-math-wasm`: a wasm module that imports the function `add` from the plugin and calls it.

- `run-with-plugin` is a host application that loads the plugin from `naive-math-plugin` and the wasm file from `naive-math-wasm`, and runs the wasm function `add` exported by the wasm binary over WasmEdge Runtime.

> Notice that WasmEdge Plugin is only available on Linux OS. In this example, all steps are validated on **Ubuntu-20.04**

Now let's build and run this example.

- Install `rustup` and `Rust`

  Go to the [official Rust webpage](https://www.rust-lang.org/tools/install) and follow the instructions to install `rustup` and `Rust`.

  > It is recommended to use Rust 1.69 or above in the stable channel.

  Then, add `wasm32-wasi` target to the Rustup toolchain:

  ```bash
  rustup target add wasm32-wasi
  ```

- Install WasmEdge Runtime

  Refer to the [Quick Install](https://wasmedge.org/book/en/quick_start/install.html#quick-install) section of WasmEdge Runtime Book to install WasmEdge Runtime. Or, use the following command directly

  ```bash
  # NOTICE that the installation script needs `sudo` access

  # install wasmedge to the directory /usr/local/
  curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash -s -- -v 0.13.4 -p /usr/local
  ```

  > For users in China mainland (中国大陆地区), try the following command to install WasmEdge Runtime if failed to run the command above
  >
  > ```bash
  > # NOTICE that the installation script needs `sudo` access
  >
  > bash install_zh.sh -v 0.13.4 -p /usr/local
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

  If the command runs successfully, `libnaive_math_plugin.so` can be found in the directory of `./target/release/`. Then, copy the generated `libnaive_math_plugin.so` to the plugin sub-directory (named `wasmedge`) in the WasmEdge installation path. If there is no such a sub-directory, you can create one with the name `wasmedge`. In this example, we assume that the WasmEdge installation path is `/usr/local/`:

  ```bash
  cp target/release/libnaive_math_plugin.so /usr/local/lib/wasmedge
  ```

- Build `naive-math-wasm`

  ```bash
  cargo build -p naive-math-wasm --target wasm32-wasi --release
  ```

  If the command runs successfully, `naive-math-wasm.wasm` can be found in the directory of `./target/wasm32-wasi/release/`.

- Run `run-with-plugin`

  ```bash
  cargo run -p run-with-plugin
  ```
  
  If the command runs successfully, then the following message is printed out on the screen:

  ```bash
  Welcome! This is NaiveMath plugin.
  1 + 2 = 3
  ```
