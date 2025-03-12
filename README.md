# tenrose

A simple Penrose binding for Typst using the WebAssembly plugin system.

## The Plan

1. Penrose is a JS library.
You can download it by running `npm install @penrose/core` in a directory containing a `node_modules` directory.
2. [wasm-minimal-protocol](https://github.com/astrale-sharp/wasm-minimal-protocol) is a Rust library to compile Rust programs to a Typst-compliant WASM binary.
See this command to build the binary defined in `penrose-wasm`:
```sh
cargo build --release --target wasm32-unknown-unknown
cp ./target/wasm32-unknown-unknown/release/hello.wasm ./
```

The idea is to run Penrose from Rust, and expose WASM functions from a Rust program this way.

## Todo

- [ ] Make a diagram from the above information.

## Acknowledgements

Thanks to all creators and maintainers of the above-mentioned libraries and tools.

The inspiration for this package comes from the equivalent for graphviz, `diagraph`.
