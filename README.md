# cargo-刀

1. `if [] == ![] { console.log("javascript sucks") }`
2. The only replacement for JavaScript is WASM
3. Rust compiles to WASM
4. Rust is great
5. Replace Javascript with rust

## What is this?
Currently, the only way to have multiple library outputs from Rust is to use multiple crates. This has a few downsides, especially when you want multiple WASM files.

1. You need multiple `Cargo.toml` files, and need to specify dependencies for each one
2. If you have shared dependencies, you need to specify a separate crate for that as well

`cargo-dao` is a wrapper for [`wasm-pack`](https://github.com/rustwasm/wasm-pack) (a very great tool) that allows you to compile multiple wasm binaries with only one crate. All you need is to add `#[cfg(dao = "output_name")]` to each part of the code you need for each output binary (quite like features). `cargo-dao` will automagically detect all instences of `#[cfg]`, and compile a separate binary for each one.

## Example
In the `wasm-pack-example` directory, run `cargo dao --target web --out-dir web/pkg`, and start a web server in `web/`. Check the console output on your browser!
