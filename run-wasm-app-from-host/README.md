# Run a Wasm App from Host

As a typical application development process, we put some code in the `main` function, and then compile it to an execution binary and run it to get the result. In this example, we will do something different. We will compile application code to a wasm binary, and then run it as a wasm module over WasmEdge Runtime.

This example consists of two projects:

- [`wasm-app`](wasm-app) is a *wasm application*, which is a common application but compiled into a wasm binary instead of a general binary execution.

- [`run-wasm-app`](run-wasm-app) is a host application that is responsible for creating a WasmEdge [Vm](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Vm.html) instance, loading the wasm binary generated from `wasm-app` and finally running it over WasmEdge Runtime.

Now let's build and run this example.

- Install `rustup` and `Rust`

  Go to the [official Rust webpage](https://www.rust-lang.org/tools/install) and follow the instructions to install `rustup` and `Rust`.

  > It is recommended to use Rust 1.71 or above in the stable channel.

  Then, add `wasm32-wasi` target to the Rustup toolchain:

  ```bash
  rustup target add wasm32-wasi
  ```

- Install WasmEdge Runtime

  Refer to the [Quick Install](https://wasmedge.org/book/en/quick_start/install.html#quick-install) section of WasmEdge Runtime Book to install WasmEdge Runtime. Or, use the following command directly

  ```bash
  # NOTICE that the installation script needs `sudo` access

  # install wasmedge to the directory /usr/local/
  curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash -s -- -v 0.14.0
  source $HOME/.wasmedge/env
  ```

  > For users in China mainland (中国大陆地区), try the following command to install WasmEdge Runtime if failed to run the command above
  >
  > ```bash
  > # NOTICE that the installation script needs `sudo` access
  >
  > bash install_zh.sh -v 0.14.0
  > source $HOME/.wasmedge/env
  > ```

- Download example

  ```bash
  git clone git@github.com:second-state/wasmedge-rustsdk-examples.git
  cd wasmedge-rustsdk-examples/run-wasm-app-from-host
  ```

- Build `wasm-app`

  ```bash
  cargo build -p wasm-app --target wasm32-wasi --release
  ```

  If the command runs successfully, `wasm-app.wasm` can be found in the directory of `./target/wasm32-wasi/release/`.

- Build and run `run-wasm-app`

  ```bash
  cargo run -p sync-run-wasm-app -- ../target/wasm32-wasi/release/sync-wasm-app.wasm
  ```

  If the command runs successfully, then the following message is printed out on the screen:

  ```bash
  Greetings from wasm-app!
  ```
