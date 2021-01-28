![Crates.io](https://img.shields.io/crates/v/variter)

# VarIter

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
    for greeting in Greeting::ALL_VARIANTS.iter() {
        println!("{:?}", greeting);
    }
}
```

If you're familiar with the Swift language, it's similar to using the
compiler-synthesized `CaseIterable` protocol conformance.

See the [module documentation](src/lib.rs) for more information!

### `no_std` Support

Given the simplicity of the crate, `no_std` support can be found through the 
`default-features = false` flag in your `Cargo.toml` dependency table.

```toml
[dependencies]
# ...
variter = { version = "0.1", default-features = false }
```

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
