//!
//! Bit sets manipulations emporium.
//!
//!
//!
//!
#![no_std]
#![deny(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod bloom;
mod comp;
mod diff;
mod indirect;
mod intersect;
mod layered;
mod ops;
mod option;
mod primitive;
mod sym_diff;
mod union;

pub use self::{
    bloom::Bloom, comp::Comp, diff::Diff, intersect::Intersect, layered::Layered, ops::*,
    sym_diff::SymDiff, union::Union,
};

/// BitSet type with capacity of 1.
pub type BitSet1 = bool;

/// BitSet type with capacity of 8.
pub type BitSet8 = u8;

/// BitSet type with capacity of 16.
pub type BitSet16 = u16;

/// BitSet type with capacity of 32.
pub type BitSet32 = u32;

/// BitSet type with capacity of 64.
pub type BitSet64 = u64;

/// BitSet type with capacity of 128.
pub type BitSet128 = u128;

/// BitSet type with capacity of 256.
pub type BitSet256 = Layered<u32, u8, 32>;

/// BitSet type with capacity of 512.
pub type BitSet512 = Layered<u64, u8, 64>;

/// BitSet type with capacity of 1024.
pub type BitSet1024 = Layered<u64, u16, 64>;

/// BitSet type with capacity of 2048.
pub type BitSet2048 = Layered<u64, u32, 64>;

/// BitSet type with capacity of 4096.
pub type BitSet4096 = Layered<u64, u64, 64>;

/// BitSet type with capacity of 8192.
pub type BitSet8192 = Layered<u64, u128, 64>;

/// BitSet type with capacity of 16384.
pub type BitSet16384 = Layered<u128, u128, 128>;

/// BitSet type with capacity of 32768.
#[cfg(feature = "alloc")]
pub type BitSet32768 = Layered<u64, Option<alloc::boxed::Box<BitSet512>>, 64>;

/// BitSet type with capacity of 65536.
#[cfg(feature = "alloc")]
pub type BitSet65536 = Layered<u64, Option<alloc::boxed::Box<BitSet1024>>, 64>;

/// BitSet type with capacity of 131072.
#[cfg(feature = "alloc")]
pub type BitSet131072 = Layered<u64, Option<alloc::boxed::Box<BitSet2048>>, 64>;

/// BitSet type with capacity of 262144.
#[cfg(feature = "alloc")]
pub type BitSet262144 = Layered<u64, Option<alloc::boxed::Box<BitSet4096>>, 64>;

/// BitSet type with capacity of 524288.
#[cfg(feature = "alloc")]
pub type BitSet524288 = Layered<u64, Option<alloc::boxed::Box<BitSet8192>>, 64>;

/// BitSet type with capacity of 1048576.
#[cfg(feature = "alloc")]
pub type BitSet1048576 = Layered<u64, Option<alloc::boxed::Box<BitSet16384>>, 64>;

/// BitSet type with capacity of 2097152.
#[cfg(feature = "alloc")]
pub type BitSet2097152 = Layered<u64, Option<alloc::boxed::Box<BitSet32768>>, 64>;

/// BitSet type with capacity of 4194304.
#[cfg(feature = "alloc")]
pub type BitSet4194304 = Layered<u64, Option<alloc::boxed::Box<BitSet65536>>, 64>;

/// BitSet type with capacity of 8388608.
#[cfg(feature = "alloc")]
pub type BitSet8388608 = Layered<u64, Option<alloc::boxed::Box<BitSet131072>>, 64>;

/// BitSet type with capacity of 16777216.
#[cfg(feature = "alloc")]
pub type BitSet16777216 = Layered<u64, Option<alloc::boxed::Box<BitSet262144>>, 64>;

/// BitSet type with capacity of 33554432.
#[cfg(feature = "alloc")]
pub type BitSet33554432 = Layered<u64, Option<alloc::boxed::Box<BitSet524288>>, 64>;

/// BitSet type with capacity of 67108864.
#[cfg(feature = "alloc")]
pub type BitSet67108864 = Layered<u64, Option<alloc::boxed::Box<BitSet1048576>>, 64>;

const fn max(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}

const fn min(a: usize, b: usize) -> usize {
    if a > b {
        b
    } else {
        a
    }
}

fn make_array<F, O, const N: usize>(mut f: F) -> [O; N]
where
    F: FnMut() -> O,
{
    use core::mem::MaybeUninit;

    let mut result = unsafe {
        // # Safe
        // All elements are `MaybeUninit` and can be uninit.
        MaybeUninit::<[MaybeUninit<O>; N]>::uninit().assume_init()
    };

    for slot in result.iter_mut() {
        unsafe {
            // # Safe. Writing to unit but valid and properly aligned memory.
            // Leaks all previously written elements on panic. Still safe.
            core::ptr::write(slot, MaybeUninit::new(f()));
        }
    }

    unsafe {
        // # Safe
        // All elements of the array were initialized.
        (&result as *const [MaybeUninit<O>; N] as *const [O; N]).read()
    }
}

fn map2_arrays<T, U, F, O, const N: usize>(
    left_array: [T; N],
    right_array: [U; N],
    mut f: F,
) -> [O; N]
where
    F: FnMut(T, U) -> O,
{
    use core::mem::MaybeUninit;

    let mut result = unsafe {
        // # Safe
        // All elements are `MaybeUninit` and can be uninit.
        MaybeUninit::<[MaybeUninit<O>; N]>::uninit().assume_init()
    };

    for ((slot, left_elem), right_elem) in result.iter_mut().zip(left_array).zip(right_array) {
        unsafe {
            // # Safe. Writing to unit but valid and properly aligned memory.
            // Leaks all previously written elements on panic. Still safe.
            core::ptr::write(slot, MaybeUninit::new(f(left_elem, right_elem)));
        }
    }

    unsafe {
        // # Safe
        // All elements of the array were initialized.
        (&result as *const [MaybeUninit<O>; N] as *const [O; N]).read()
    }
}
