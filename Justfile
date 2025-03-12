[working-directory: "penrose-wasm"]
test: stub-wasm
    typst compile hello.typ

[working-directory: "penrose-wasm/wasi-stub"]
stub-wasm: build-wasm
    cargo run -- ../target/wasm32-wasip1/debug/penrose_wasm.wasm -o ../penrose_wasm.wasm

[working-directory: "penrose-wasm"]
build-wasm:
    cargo build --target wasm32-wasip1
