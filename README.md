# VarIter

[![Crates.io](https://img.shields.io/crates/v/variter)](https://crates.io/crates/variter)
[![Docs.rs](https://docs.rs/variter/badge.svg)](https://docs.rs/variter)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

`variter` is a Rust crate that exports some simple tools for iterating
over `enum` types with zero or more variants that have no fields.

```rust
use variter::{VarIter, derive_var_iter};
derive_var_iter! {
    #[derive(Debug)]
    enum Greeting {
        Hello,
        Hola,
        Bonjour,
    }
}

fn main() {
    for greeting in Greeting::ALL_VARIANTS {
        println!("{:?}", greeting);
    }
}
```

If you're familiar with the Swift language, it's similar to using the
compiler-synthesized `CaseIterable` protocol conformance.

See the [module documentation](src/lib.rs) for more information!

## Usage

Add `variter` as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
# ...
variter = "0.2"
```

## Features

- `foreign_impls` (default): include implementions of `VarIter` on stable `core` types.
- `std` (default): if `foreign_impls` is turned on, `VarIter` implementations are included for stable `std` types.

## `no_std` Support

Given the simplicity of the crate, `no_std` support can be toggled through the
`default-features = false` flag in your `Cargo.toml` dependency table.

```toml
[dependencies]
# ...
variter = { version = "0.2", default-features = false }
```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
