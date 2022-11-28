# Run a Wasm App from Host

As a typical application development process, we put some code in the `main` function, and then compile it to an execution binary and run it to get the result. In this example, we will do something different. We will compile application code to a wasm binary, and then run it as a wasm module over WasmEdge Runtime.

This example consists of two projects:

- [`wasm-app`](wasm-app) is a *wasm application*, which is a common application but compiled into a wasm binary instead of a general binary execution.

- [`run-wasm-app`](run-wasm-app) is a host application that is responsible for creating a WasmEdge [Vm](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Vm.html) instance, loading the wasm binary generated from `wasm-app` and finally running it over WasmEdge Runtime.

Now let's build and run this example.

- Install `rustup` and `Rust nightly`

  Go to the [official Rust webpage](https://www.rust-lang.org/tools/install) and follow the instructions to install `rustup`. Then, run the following commands in your terminal:

  ```bash
  rustup default nightly
  rustup target add wasm32-wasi
  ```

- Download example

  ```bash
  git clone git@github.com:apepkuss/wasmedge-rust-examples.git
  cd wasmedge-rust-examples
  ```

- Install `libwasmedge`

  Refer to the [Quick Install](https://wasmedge.org/book/en/quick_start/install.html#quick-install) section of WasmEdge Runtime Book to install `libwasmedge`. Or, use the following command directly

  ```bash
  ./install_libwasmedge.sh -p /usr/local
  ```

  > For users in China mainland (中国大陆地区), try the following command to install `libwasmedge` if failed to run the command above
  >
  > ```bash
  > ./install_libwasmedge_zh.sh -p /usr/local
  > ```

  > To install a specific version of `libwasmedge`, use `-v` option. For example, the following command installs `libwasmedge 0.11.2` to `/usr/local/`
  >
  > ```bash
  > ./install_libwasmedge.sh -p /usr/local -v 0.11.2
  > ```

- Build `wasm-app`

  ```bash
  cd run-wasm-app-from-host
  cargo build -p wasm-app --target wasm32-wasi --release
  ```

  If the command runs successfully, `wasm-app.wasm` can be found in the directory of `./target/wasm32-wasi/release/`.

- Build and run `run-wasm-app`

  ```bash
  // in the directory of run-wasm-app-from-host
  cargo run -p run-wasm-app -- ./target/wasm32-wasi/release/wasm-app.wasm
  ```

  If the command runs successfully, then the following message is printed out on the screen:

  ```bash
  Greetings from wasm-app!
  ```
