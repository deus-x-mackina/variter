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
