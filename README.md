# dApp-IPFS

Simple dApp written in Rust, compiled to Wasm and stored on IPFS.

To run make sure you have installed `Rust` and `Yew`. Run to check installed:
```=bash
cargo --version

rustup target list | grep 'wasm32-unknown-unknwon'

trunk --version
```

To run server (on port 8080)
```=bash
trunk serve
```