# Retrieve the Socket Address

This example presents how to retrieve the socket address information asynchronously.

This example consists of two projects:

- [`socket-addr`](socket-addr) is a *wasm application*, in which a group of socket addresses are set.

- [`main-app`](main-app) is a host application that is responsible for creating a WasmEdge [Vm](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Vm.html) instance, loading the wasm binary generated from `socket-addr` and finally running it over WasmEdge Runtime.

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
  curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash -s -- -v 0.13.5 -p /usr/local
  ```

  > For users in China mainland (中国大陆地区), try the following command to install WasmEdge Runtime if failed to run the command above
  >
  > ```bash
  > # NOTICE that the installation script needs `sudo` access
  >
  > bash install_zh.sh -v 0.13.5 -p /usr/local
  > ```

- Download example

  ```bash
  git clone git@github.com:second-state/wasmedge-rustsdk-examples.git
  cd wasmedge-rustsdk-examples/async-wasi-socket-addr
  ```

- Build `socket-addr`

  ```bash
  cargo build -p socket-addr --target wasm32-wasi --release
  ```

  If the command runs successfully, `socket-addr.wasm` can be found in the directory of `./target/wasm32-wasi/release/`.

- Build and run `main-app`

  ```bash
  cargo run -p main-app -- ./target/wasm32-wasi/release/socket-addr.wasm
  ```

  If the command runs successfully, then the following message is printed out on the screen:

  ```bash
  [socket-addr] addr: 127.0.0.1:3000
  [socket-addr] addr: 127.0.0.1:3000
  [socket-addr] addr1: 0.0.0.0:80
  [socket-addr] addr2: 127.0.0.1:443
  [socket-addr] err: InvalidInput
  [socket-addr] err: Uncategorized
  [socket-addr] addr: None
  exit_code: 0
  ```
