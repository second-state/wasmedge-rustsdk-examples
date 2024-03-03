# GGML Llama via WASI-NN

WasmEdge Runtime implements the WASI-NN proposal. This example demonstrates how to chat with Llama2 model driven by WasmEdge wasi-nn-ggml backend.

> To run this example, the operating system should be **Ubuntu-20.04 or above** on **x86_64 target**.

Now let's build and run this example.

- Install `rustup` and `Rust`

  Go to the [official Rust webpage](https://www.rust-lang.org/tools/install) and follow the instructions to install `rustup` and `Rust`.

  > It is recommended to use Rust 1.71 or above in the stable channel.

  Then, add `wasm32-wasi` target to the Rustup toolchain:

  ```bash
  rustup target add wasm32-wasi
  ```

- Install `openblas`

  ```bash
  apt install -y libopenblas-dev
  ```

- Install WasmEdge Runtime

  Use the following command to install WasmEdge Runtime and the `wasi_nn-ggml` plugin:

  ```bash
  # NOTICE that the installation script needs `sudo` access

  # install wasmedge to the directory /usr/local/
  curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash -s -- -v 0.13.5 --plugins wasi_nn-ggml -p /usr/local
  ```

  > For users in China mainland (中国大陆地区), try the following command to install WasmEdge Runtime if failed to run the command above
  >
  > ```bash
  > # NOTICE that the installation script needs `sudo` access
  >
  > bash install_zh.sh -v 0.13.5 --plugins wasi_nn-ggml -p /usr/local
  > ```

- Download example

  ```console
  git clone git@github.com:second-state/wasmedge-rustsdk-examples.git
  cd wasmedge-rustsdk-examples/ggml-llama-via-wasinn
  ```

- Build the `ggml-llama-wasm` wasm app

  ```console
  cargo build -p ggml-llama-wasm --target wasm32-wasi --release
  ```

  If the command runs successfully, you can find the `ggml-llama-wasm.wasm` file in the `target/wasm32-wasi/release` directory.

- Download the Llama2 model of GGUF format

  ```console
  curl -LO https://huggingface.co/second-state/Llama-2-7B-Chat-GGUF/resolve/main/llama-2-7b-chat.Q5_K_M.gguf
  ```

- Build & run the `run-ggml-llama-wasm` app

  ```console
  cargo run -p run-ggml-llama-inference -- .:. target/wasm32-wasi/release/ggml-llama-wasm.wasm default
  ```

  If the command runs successfully, you can try the multi-turn conversations like below:

  ```console
  [Question]:
  What's the capital of France?
  [Answer]:
  The capital of France is Paris.
  [Question]:
  What about Norway?
  [Answer]:
  The capital of Norway is Oslo.
  [Question]:
  How many planets are in the solar system?
  [Answer]:
  There are 8 planets in the solar system: Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, and Neptune.
  ```
