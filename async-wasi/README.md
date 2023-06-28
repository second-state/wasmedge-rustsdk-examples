# Run a Wasm App from Host

As a typical application development process, we put some code in the `main` function, and then compile it to an execution binary and run it to get the result. In this example, we will do something different. We will compile application code to a wasm binary, and then run it as a wasm module over WasmEdge Runtime.

This example consists of two projects:

- [`wasm-app`](wasm-app) is a *wasm application*, which is a common application but compiled into a wasm binary instead of a general binary execution.

- [`run-wasm-app`](run-wasm-app) is a host application that is responsible for creating a WasmEdge [Vm](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Vm.html) instance, loading the wasm binary generated from `wasm-app` and finally running it over WasmEdge Runtime.

Now let's build and run this example.

- Install `rustup` and `Rust`

  Go to the [official Rust webpage](https://www.rust-lang.org/tools/install) and follow the instructions to install `rustup` and `Rust`.

  > It is recommended to use Rust 1.68 or above in the stable channel.

  Then, add `wasm32-wasi` target to the Rustup toolchain:

  ```bash
  rustup target add wasm32-wasi
  ```

- Install WasmEdge Runtime

  Refer to the [Quick Install](https://wasmedge.org/book/en/quick_start/install.html#quick-install) section of WasmEdge Runtime Book to install WasmEdge Runtime. Or, use the following command directly

  ```bash
  # NOTICE that the installation script needs `sudo` access

  # install wasmedge to the directory /usr/local/
  curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash -s -- -v 0.13.0 -p /usr/local
  ```

  > For users in China mainland (中国大陆地区), try the following command to install WasmEdge Runtime if failed to run the command above
  >
  > ```bash
  > # NOTICE that the installation script needs `sudo` access
  >
  > bash install_zh.sh -v 0.13.0 -p /usr/local
  > ```

- Download example

  ```bash
  git clone git@github.com:second-state/wasmedge-rustsdk-examples.git
  cd wasmedge-rustsdk-examples/async-wasi
  ```

- Build `wasm-app`

  ```bash
  cargo build -p wasm-app --target wasm32-wasi --release
  ```

  If the command runs successfully, `wasm-app.wasm` can be found in the directory of `./target/wasm32-wasi/release/`.

- Build and run `run-wasm-app`

  ```bash
  cargo run -p run-wasm-app -- ./target/wasm32-wasi/release/wasm-app.wasm
  ```

  If the command runs successfully, then the following message is printed out on the screen:

  ```bash
  [tick] i=0
  [wasm-app] enter >>>>>
  [wasm-app] ENV=VAL
  [wasm-app] entry in the current guest preopened dir:
    "/README.md"
    "/run-wasm-app"
    "/Cargo.lock"
    "/Cargo.toml"
    "/target"
    "/wasm-app"
  [wasm-app] say hello: 0
  [tick] i=1
  [tick] i=2
  [wasm-app] say hello: 1
  [tick] i=3
  [tick] i=4
  [wasm-app] say hello: 2
  [tick] i=5
  [tick] i=6
  [wasm-app] say hello: 3
  [tick] i=7
  [tick] i=8
  [wasm-app] say hello: 4
  [tick] i=9
  [tick] i=10
  [wasm-app] say hello: 5
  [tick] i=11
  [tick] i=12
  [wasm-app] say hello: 6
  [tick] i=13
  [tick] i=14
  [wasm-app] say hello: 7
  [tick] i=15
  [wasm-app] say hello: 8
  [tick] i=16
  [tick] i=17
  [wasm-app] say hello: 9
  [tick] i=18
  [tick] i=19
  [wasm-app] exit <<<<<
  exit_code: 0
  ```
