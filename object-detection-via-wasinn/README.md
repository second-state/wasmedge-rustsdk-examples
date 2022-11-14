# Object Detection via WASI-NN

WasmEdge Runtime implements the WASI-NN proposal. This example demonstrates how to perform an AI inference task, object detection, over WasmEdge Runtime.

> To run this example, the operating system should be **Ubuntu-20.04 or above** on **x86_64 target**

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

- Build & run

  ```bash
  cd object-detection-via-wasinn
  cargo run -- .:. wasmedge-wasinn-example-mobilenet-image.wasm mobilenet.pt input.jpg
  ```

  If the command runs successfully, then the following result is printed out on the screen:

  ```bash
  Read torchscript binaries, size in bytes: 14376860
  Loaded graph into wasi-nn with ID: 0
  Created wasi-nn execution context with ID: 0
  Read input tensor, size in bytes: 602112
  Executed graph inference
     1.) [954](20.6681)banana
     2.) [940](12.1483)spaghetti squash
     3.) [951](11.5748)lemon
     4.) [950](10.4899)orange
     5.) [953](9.4834)pineapple, ananas
  ```
