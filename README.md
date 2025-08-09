# mavlink2rest-wasm

This is a minimal Rust WebAssembly project that exposes a `ParserEmitter` object to JavaScript. You can send a string from JS to Rust and have Rust call back into JS with the string plus an exclamation mark.

## Test it
- https://patrickelectric.work/mavlink2rest-wasm/

## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [basic-http-server](https://crates.io/crates/basic-http-server) (for serving the files)

## Build Instructions

1. **Clone the repository** (if you haven't already):
   ```sh
   git clone <your-repo-url>
   cd mavlink2rest-wasm
   ```

2. **Build the WebAssembly package:**
   ```sh
   wasm-pack build --target web
   ```

3. **Serve the project locally:**
   ```sh
   basic-http-server .
   ```
   This will start a server at [http://localhost:4000](http://localhost:4000).

4. **Open the demo:**
   Open your browser and go to [http://localhost:4000/index.html](http://localhost:4000/index.html)

## How it works
- The `ParserEmitter` object in Rust has two methods:
  - `parser(input: &str)`: stores a string.
  - `emit(callback: Function)`: calls the JS callback with the stored string plus an exclamation mark.
- See `index.html` for a usage example.
