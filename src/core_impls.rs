use core::{cmp::Ordering, fmt::Alignment, num::FpCategory, sync::atomic};

use super::*;

foreign_derive_var_iter! {
    Ordering [Ordering::Greater, Ordering::Less, Ordering::Equal]
    Alignment [Alignment::Center, Alignment::Right, Alignment::Left]
    FpCategory [FpCategory::Nan, FpCategory::Infinite, FpCategory::Zero, FpCategory::Subnormal, FpCategory::Normal]
    atomic::Ordering [
        atomic::Ordering::Relaxed,
        atomic::Ordering::Release,
        atomic::Ordering::Acquire,
        atomic::Ordering::AcqRel,
        atomic::Ordering::SeqCst,
    ]
}
