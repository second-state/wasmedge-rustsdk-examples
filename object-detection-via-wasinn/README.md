# Object Detection via WASI-NN

WasmEdge Runtime implements the WASI-NN proposal. This example demonstrates how to perform an AI inference task, object detection, over WasmEdge Runtime.

> To run this example, the operating system should be **Ubuntu-20.04 or above** on **x86_64 target**.

> The AI model, the wasm file and input image used in this example are from [pytorch-mobilenet-image](https://github.com/second-state/WasmEdge-WASINN-examples/blob/master/pytorch-mobilenet-image/README.md) project.

Now let's build and run this example.

- Install `rustup` and `Rust`

  Go to the [official Rust webpage](https://www.rust-lang.org/tools/install) and follow the instructions to install `rustup` and `Rust`.

  > It is recommended to use Rust 1.66 or above in the stable channel.

  Then, add `wasm32-wasi` target to the Rustup toolchain:

  ```bash
  rustup target add wasm32-wasi
  ```

- Install WasmEdge Runtime

  Refer to the [Quick Install](https://wasmedge.org/book/en/quick_start/install.html#quick-install) section of WasmEdge Runtime Book to install WasmEdge Runtime. Or, use the following command directly

  ```bash
  # NOTICE that the installation script needs `sudo` access

  # install wasmedge to the directory /usr/local/
  curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash -s -- -v 0.12.0 -p /usr/local
  ```

  > For users in China mainland (中国大陆地区), try the following command to install WasmEdge Runtime if failed to run the command above
  >
  > ```bash
  > # NOTICE that the installation script needs `sudo` access
  >
  > bash install_zh.sh -v 0.12.0 -p /usr/local
  > ```

- Install `libtorch` and `WasmEdge WASI-NN PyTorch Backend`
  
  [Get WasmEdge with WASI-NN Plug-in PyTorch Backend](https://wasmedge.org/book/en/write_wasm/rust/wasinn.html#get-wasmedge-with-wasi-nn-plug-in-pytorch-backend) in WasmEdge Book describes the steps of deploying the `libtorch` library and `WASI-NN PyTorch Backend`. For convenience, the commands in the book are listed below:

  - Install `libtorch`

    ```bash
    export PYTORCH_VERSION="1.8.2"
    
    # download and unzip libtorch-1.8.2
    curl -s -L -O --remote-name-all https://download.pytorch.org/libtorch/lts/1.8/cpu/libtorch-cxx11-abi-shared-with-deps-${PYTORCH_VERSION}%2Bcpu.zip

    # unzip
    unzip -q "libtorch-cxx11-abi-shared-with-deps-${PYTORCH_VERSION}%2Bcpu.zip"

    # set LD_LIBRARY_PATH
    export LD_LIBRARY_PATH=${LD_LIBRARY_PATH}:$(pwd)/libtorch/lib

    # set Torch_DIR
    export Torch_DIR=$(pwd)/libtorch
    ```

  - Download `WasmEdge WASI-NN Plugin PyTorch Backend`

    ```bash
    # Assume that the installation path of WasmEdge is `/usr/local/`
    # go to the plugin directory.
    cd /usr/local/lib/wasmedge

    # download and unzip WASI-NN plugin PyTorch Backend
    curl -sLO https://github.com/WasmEdge/WasmEdge/releases/download/0.12.0/WasmEdge-plugin-wasi_nn-pytorch-0.12.0-ubuntu20.04_x86_64.tar.gz

    # unzip plugin
    tar -zxf WasmEdge-plugin-wasi_nn-pytorch-0.12.0-ubuntu20.04_x86_64.tar.gz

    # set WASMEDGE_PLUGIN_PATH environment variable
    export WASMEDGE_PLUGIN_PATH=/usr/local/lib/wasmedge
    ```

- Download example

  ```bash
  git clone git@github.com:second-state/wasmedge-rustsdk-examples.git
  cd wasmedge-rustsdk-examples/object-detection-via-wasinn
  ```

- Build & run

  ```bash
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
