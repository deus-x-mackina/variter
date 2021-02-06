use std::collections::HashSet;

use variter::{derive_var_iter, VarIter};

derive_var_iter! {
    #[derive(Debug, PartialEq)]
    enum Foo {}

    #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
    enum Bar {
        X,
        Y,
        Z,
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    enum Baz {
        A = 1,
        B = 10,
        C,
    }
}

#[test]
fn foo_has_no_cases() {
    assert_eq!(Foo::ALL_VARIANTS, &[]);
}

#[test]
fn bar_has_three_cases() {
    assert_eq!(Bar::ALL_VARIANTS.len(), 3);
}

#[test]
fn bar_cases_are_unique() {
    let mut set = HashSet::with_capacity(Bar::ALL_VARIANTS.len());
    assert!(Bar::ALL_VARIANTS.iter().all(|&case| set.insert(case)));
}

#[test]
fn baz_has_three_cases() {
    assert_eq!(Baz::ALL_VARIANTS.len(), 3);
}

#[test]
fn baz_discriminants_work() {
    for &var in Baz::ALL_VARIANTS.iter() {
        match var {
            Baz::A => assert_eq!(var as isize, 1),
            Baz::B => assert_eq!(var as isize, 10),
            Baz::C => assert_eq!(var as isize, 11),
        }
    }
}

#[test]
fn macro_works_in_fn() {
    derive_var_iter! {
        enum Foo {}
        enum Bar { A, B, C }
    }
}

#[test]
fn crate_impls_work() { for _ in core::sync::atomic::Ordering::ALL_VARIANTS {} }
