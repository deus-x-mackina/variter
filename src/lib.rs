//! A trait and declarative macro for iterating through `enum` variants.
//!
//! ## The Problem
//!
//! If an `enum` type in Rust contains zero or more variants that have no
//! fields, it would be valid to iterate through each of these variants.
//! Unfortunately, there is not an easy way to achieve this.
//!
//! ```
//! // A suboptimal solution...
//! enum Status {
//!     Complete,
//!     Error,
//! }
//!
//! impl Status {
//!     // not a robust solution, requires tweaking whenever a variant is added or removed
//!     // this computation is also invalidated if any of the variants receive fields
//!     fn get_variants() -> &'static [Self] { &[Status::Complete, Status::Error] }
//! }
//! ```
//!
//! ## The Solution
//!
//! `variter` provides a trait [`VarIter`] that gives you the opportunity to
//! define an associated constant that returns `&'static [Self]`. This trait
//! alone won't help much on its own, but the macro [`derive_var_iter!`] can
//! wrap your `enum` declarations and implement [`VarIter`] for you. As cases
//! are added or removed, the implementation is automatically updated.
//!
//! ```
//! use variter::{derive_var_iter, VarIter};
//! derive_var_iter! {
//!     #[derive(Debug, PartialEq, Eq)]
//!     enum Status {
//!         Complete,
//!         Error,
//!     }
//! }
//! assert_eq!(Status::ALL_VARIANTS.len(), 2);
//! assert!(Status::ALL_VARIANTS.contains(&Status::Error));
//! assert!(Status::ALL_VARIANTS.contains(&Status::Complete));
//! ```
//!
//! ## What about a `#[derive]` macro?
//!
//! `#[derive]` macros are great, but many IDEs and editors don't support
//! autocompletion capabilities for traits derived in this way, except for the
//! builtin traits, and possibly [`serde`] traits. On the other hand, many IDEs
//! and editors can expand declarative macros in place and offer autocompletion
//! and other hints.
//!
//! ## Manual Implementations
//!
//! It is not recommended to implement [`VarIter`] manually for the following
//! reasons:
//!   - Manual implementations need to be updated as variants are added or
//!     removed.
//!   - Implementing [`VarIter`] on incorrect types is a logical error. Invalid
//!     types include:
//!     - `structs`
//!     - `enum`s where any variant contains a field, such as `Case1(i32)` or
//!       `Case2 { x: usize }`
//!   - Using [`derive_var_iter!`] results in a compile-time error if appiled to
//!     the above incorrect types (albeit with an unhelpful error message!)
//!
//! ## Additionaly...
//!
//! This crate exports an additional macro [`foreign_derive_var_iter!`] for
//! implementing [`VarIter`] on foreign types. The crate also implements
//! [`VarIter`] on stable, field-less enums from [`core`] and [`std`].
//!
//! [`serde`]: https://serde.rs/
//! [`core]: https://doc.rust-lang.org/stable/core/index.html
//! [`std`]: https://doc.rust-lang.org/stable/std/index.html

#![doc(html_root_url = "https://docs.rs/variter/0.1")]
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code, missing_docs)]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

#[cfg(feature = "foreign_impls")]
mod core_impls;

#[cfg(all(feature = "std", feature = "foreign_impls"))]
mod std_impls;

/// A trait for field-less `enum`s that gives access to each of its variants.
///
/// ```
/// use variter::{derive_var_iter, VarIter};
/// derive_var_iter! {
///     enum CardSuit {
///         Clubs,
///         Spades,
///         Hearts,
///         Diamonds,
///     }
/// }
/// use CardSuit::*;
/// print!("Card suits: ");
/// for suit in CardSuit::ALL_VARIANTS {
///     let symbol = match suit {
///         Clubs => '\u{2664}',
///         Hearts => '\u{2661}',
///         Spades => '\u{2667}',
///         Diamonds => '\u{2662}',
///     };
///     print!("{} ", symbol);
/// }
/// println!();
/// ```
pub trait VarIter: Sized + 'static {
    /// A static reference to a slice containing one of each of this `enum`'s
    /// variants.
    const ALL_VARIANTS: &'static [Self];
}

/// Automatically derive [`VarIter`] for your field-less `enum`s. Multiple
/// `enum` declarations can be included inside the macro.
///
/// ```
/// use variter::{derive_var_iter, VarIter};
/// derive_var_iter! {
///     enum Empty {}
///     pub enum TwoChoice {
///         First,
///         Second
///     }
///     #[derive(Debug)]
///     pub(crate) enum CoinFlip {
///         Heads,
///         Tails
///     }
/// }
/// for var in CoinFlip::ALL_VARIANTS {
///     println!("{:?}", var);
/// }
/// ```
#[macro_export]
macro_rules! derive_var_iter {
    // No variants at all
    (
        $(#[$attr:meta])*
        $vis:vis enum $name:ident {}
        $($rest:tt)*
    ) => {
        $(#[$attr])*
        $vis enum $name {}

        #[automatically_derived]
        impl $crate::VarIter for $name {
            const ALL_VARIANTS: &'static [Self] = &[];
        }

        derive_var_iter! { $($rest)* }
    };

    // One or more variants
    (
        $(#[$attr:meta])*
        $vis:vis enum $name:ident { $($cases:ident $(= $disc:expr)?),+ $(,)? }
        $($rest:tt)*
    ) => {
        $(#[$attr])*
        $vis enum $name {
            $($cases $(= $disc)?),+
        }

        #[automatically_derived]
        impl $crate::VarIter for $name {
            const ALL_VARIANTS: &'static [Self] = &[
                $(Self::$cases),+
            ];
        }

        derive_var_iter! { $($rest)* }
    };
    () => {};
}

/// Automatically derive [`VarIter`] for foreign field-less `enums`. Be sure to
/// include all variants. ## Syntax
/// ```text
/// foreign_derive_var_iter! {
///     Typename [] // enum has no variants
///     Typename [Typename::Variant1, Typename::variant2, ... ] // trailing comma supported
///     ...
/// }
/// ```
#[macro_export]
macro_rules! foreign_derive_var_iter {
    // No variants at all
    ($type:ty [] $($rest:tt)*) => {
        impl $crate::VarIter for $type {
            const ALL_VARIANTS: &'static [Self] = &[];
        }
        foreign_derive_var_iter!($($rest)*);
    };
    // One or more variants
    ($type:ty [$($cases:expr),+ $(,)?] $($rest:tt)*) => {
        impl $crate::VarIter for $type {
            const ALL_VARIANTS: &'static [Self] = &[
                $($cases),+
            ];
        }
        foreign_derive_var_iter!($($rest)*);
    };
    () => {};
}
