# Call a Wasm Library from Host

This example demonstrates how to use WasmEdge Rust SDK to create multiple threads to help us render [Mandelbrot set](https://en.wikipedia.org/wiki/Mandelbrot_set) in parallel, which is a compute-intensive workload. This example also present sharing the image memory between threads while rendering the image parallelly.

This example consists of three parts:

- [`mandelbrot-c`](mandelbrot-c) contains `mandelbrot.c` that defines a C library of `mandelbrot`.

- [`run-mandelbrot-in-parallel`](run-mandelbrot-in-parallel) is a host application that loads the wasm binary generated from `wasm-lib`, and runs the wasm function exported by the wasm library over WasmEdge Runtime.

- [`visualize_result.ipynb`](visualize_result.ipynb) is a Python notebook file that visualizes the generated image file.

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

- Install `zig`

  Go to the [official Zig webpage](https://ziglang.org/download/) and follow the instructions to install `zig`. With Zig, you will not need to separately install the WASI SDK, as it is included with the toolchain.

- Download example

  ```bash
  git clone git@github.com:second-state/wasmedge-rustsdk-examples.git
  cd wasmedge-rustsdk-examples/multi-threaded-parallel
  ```

- Compile `mandelbrot-c` into `mandelbrot.wasm`

  ```bash
  cd mandelbrot-c

  # compile mandelbrot.c to mandelbrot.o
  zig build-obj -O ReleaseSmall -target wasm32-wasi mandelbrot.c
  
  # link mandelbrot.o to mandelbrot.wasm
  wasm-ld --no-entry mandelbrot.o -o mandelbrot.wasm --import-memory --export-all --shared-memory --features=mutable-globals,atomics,bulk-memory
  
  # go back to the root directory of `multi-threaded-parallel`
  cd ..
  ```

  If the commands run successfully, `mandelbrot.wasm` can be found in the directory of `mandelbrot-c`.

- Build and run `run-mandelbrot-in-parallel`

  ```bash
  # in wasmedge-rust-examples/multi-threaded-parallel
  cargo run -p run-mandelbrot-in-parallel -- ./mandelbrot-c/mandelbrot.wasm
  ```

  If the command runs successfully, then the following message is printed out on the screen:

  ```bash
  [2022-12-19 00:04:33.249] [info] compile start
  [2022-12-19 00:04:33.257] [info] verify start
  [2022-12-19 00:04:33.257] [info] optimize start
  [2022-12-19 00:04:33.280] [info] codegen start
  [2022-12-19 00:04:33.296] [info] output start
  [2022-12-19 00:04:33.311] [info] compile done
  Time elapsed: 486.07ms
  ```

  In addition, the command above generates two files: one is the image file `mandelbrot-image.bin`, one is a AOT file.

- Visualize the generated image file
  
  Open the notebook file `visualize_result.ipynb` and follow the instructions to visualize the generated image file.
