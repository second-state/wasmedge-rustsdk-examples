# Define & Run Host Function

This example demonstrates how to manipulate linear memory.

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
  cd wasmedge-rust-examples/manipulate-linear-memory
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
