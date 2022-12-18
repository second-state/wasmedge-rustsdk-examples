# Run wasm functions in AOT mode

This example demonstrates how to generate a AOT file with WasmEdge AOT compiler, and then load the file and run the wasm function in AOT mode.

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
  git clone git@github.com:apepkuss/wasmedge-rust-examples.git
  cd wasmedge-rust-examples/run-wasm-func-in-aot-mode
  ```

- Build & run

  ```bash
  cargo run
  ```

  If the command runs successfully, then the following result is printed out on the screen:

  ```bash
  [2022-12-18 21:19:37.370] [info] compile start
  [2022-12-18 21:19:37.372] [info] verify start
  [2022-12-18 21:19:37.373] [info] optimize start
  [2022-12-18 21:19:37.384] [info] codegen start
  [2022-12-18 21:19:37.391] [info] output start
  [2022-12-18 21:19:37.402] [info] compile done
  fib(5): 8
  ```
