# Define & Run Async Host Functions

This example demonstrates how to define async host functions, and register and run it over WasmEdge Runtime.

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

  ```bash
  ./install_libwasmedge.sh -p /usr/local
  ```

  > For users in China mainland, try the following command to install `libwasmedge` if failed to run the command above
  >
  > ```bash
  > ./install_libwasmedge_zh.sh -p /usr/local
  > ```

  > To install a specific version of `libwasmedge`, use `-v` option. For example, the following command installs `libwasmedge 0.11.2` to `/usr/local/`
  >
  > ```bash
  > ./install_libwasmedge.sh -p /usr/local -v 0.11.2
  > ```

- Build & run

  ```bash
  cd define-async-host-func
  cargo run
  ```

  If the command runs successfully, then the following result is printed out on the screen:

  ```bash
  [read_book] sleep 2 second
  [enjoy_music] sleep 1 second
  [enjoy_music] enjoying music ...
  [read_book] reading ...
  ```
