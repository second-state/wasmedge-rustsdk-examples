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

- Install `libwasmedge` with WASI-NN PyTorch Backend

  ```bash
  ./install_libwasmedge.sh -p /usr/local -e all
  ```

  > For users in China mainland, try the following command to install `libwasmedge` if failed to run the command above
  >
  > ```bash
  > ./install_libwasmedge_zh.sh -p /usr/local -e all
  > ```

  > To install a specific version of `libwasmedge`, use `-v` option. For example, the following command installs `libwasmedge 0.11.2` to `/usr/local/`
  >
  > ```bash
  > ./install_libwasmedge.sh -p /usr/local -e all -v 0.11.2
  > ```

- Build & run

  ```bash
  cd object-detection-via-wasinn
  cargo run
  ```

  If the command runs successfully, then the following result is printed out on the screen:

  ```bash
  
  ```
