# tenrose

A plugin to create [Penrose](https://penrose.cs.cmu.edu/) diagrams directly inside a [Typst](https://typst.app) document.

## How to use

TODO once I can get it to work.

## How to contribute

Install the dependencies listed in `shell.nix`, then run `just` to build and test.

### How it works

1. Penrose is a JS library.
1. QuickJS is a lightweight JS runtime; [rquickjs](https://github.com/DelSkayn/rquickjs) is a Rust library to call JS from Rust (among other things).
1. [wasm-minimal-protocol](https://github.com/astrale-sharp/wasm-minimal-protocol) is a Rust library to compile Rust programs to a Typst-compliant WASM binary.

Typst reads from a WASM binary. This binary is compiled from a Rust program, which embeds some Penrose functions through rquickjs. wasm-minimal-protocol is there to make sure that the binary fits the Typst WASM plugin protocol.

### In detail

Here are various notes about how to project is structured. This is my current understanding, please feel free to let me know if it's unclear/incorrect/badly worded.
Also look at the `Justfile` to understand the dependency structure.

The Penrose source contains a *bundle* file, which is a minified `index.js` file that contains the whole penrose code. I don't know yet if this file is self-contained (i.e. if it also contains all of penrose's dependencies). If not, I'll have to find a way to make a self-contained JS file.
This file can be found by `npm install`ing penrose.

Once the file is on disk, rquickjs can read and evaluate it, and expose functions from that file to Rust.

The WASM binary architecture ("target"?) is `wasm32-wasip1`. Typst will also allow binaries compiled for `wasm32-unknown-unknown`, however, [rquickjs can only be used with `wasm32-wasi`](https://github.com/DelSkayn/rquickjs/issues/93#issuecomment-1935955666).
That means that we need to stub some functions of our binary to make Typst happy. This is the job of wasi-stub, which comes from [wasm-minimal-protocol](https://github.com/astrale-sharp/wasm-minimal-protocol). I haven't found a way to `cargo install` it from GitHub/crates.io yet so it's vendored.

## Acknowledgements

Thanks to all creators and maintainers of the above-mentioned libraries and tools.

Thanks to [svgo](https://github.com/svg/svgo) for an example of using rquickjs to call JS functions.

Thanks to the authors of [diagraph](https://github.com/Robotechnic/diagraph) for the inspiration for this package.
