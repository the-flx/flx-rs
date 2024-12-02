[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Release](https://img.shields.io/github/tag/the-flx/flx-rs.svg?label=release&logo=github)](https://github.com/the-flx/flx-rs/releases/latest)
[![crates.io](https://img.shields.io/crates/v/flx-rs.svg)](https://crates.io/crates/flx-rs)
[![crates.io.d](https://img.shields.io/crates/d/flx-rs)](https://crates.io/crates/flx-rs)

# flx-rs
> Rewrite emacs-flx in Rust for dynamic modules

[![CI](https://github.com/the-flx/flx-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/the-flx/flx-rs/actions/workflows/ci.yml)

## 🔨 Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
flx-rs = "0.2.1"
```

This package only exposed one function `flx_rs::score` that takes only two arguments
`(str: &str, query: &str)`.

```rust
use flx_rs;

fn main() {
    let result : Option<flx_rs::Result> = flx_rs::score("switch-to-buffer", "stb");

    println!("Score: {}", result.unwrap().score);   // Score: 237
}
```

See the official documentation at https://docs.rs/flx-rs/latest/flx_rs/.

## 📂 Example

- [flx-rs](https://github.com/jcs-elpa/flx-rs) - Emacs package uses this as a dynamic module

## ⚜️ License

`flx-rs` is distributed under the terms of the MIT license.

See [`LICENSE`](./LICENSE) for details.


<!-- Links -->

[flx]: https://github.com/lewang/flx
