# Define & Run Host Function

This example demonstrates how to manipulate linear memory.

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
  cd wasmedge-rustsdk-examples/manipulate-linear-memory
  ```

- Build & run

  ```bash
  cargo run
  ```

  If the command runs successfully, then the following result is printed out on the screen:

  ```bash
  The memory size (in pages): 1
  The data size (in bytes): 65536
  The memory size (in pages) after growing memory additional 2 pages: 3
  The data size (in bytes) after growing memory additional 2 pages: 196608
  Set 0xFEFEFFE at the memory address 0x2220
  Retrieve the value at the memory address 0x2220: 0xFEFEFFE
  Set 0xFEA09 at the memory address 0x1FFFC
  Retrieve the value at the memory address 0x1FFFC: 0xFEA09
  ```
