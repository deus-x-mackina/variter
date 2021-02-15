use core::{cmp::Ordering, fmt::Alignment, num::FpCategory};

use super::*;

foreign_derive_var_iter! {
    Ordering [Ordering::Greater, Ordering::Less, Ordering::Equal]
    Alignment [Alignment::Center, Alignment::Right, Alignment::Left]
    FpCategory [FpCategory::Nan, FpCategory::Infinite, FpCategory::Zero, FpCategory::Subnormal, FpCategory::Normal]
}
