//!
//! `bitsetium` crate contains traits for common bitset operations, and number of implementations.
//!
//!

#[cfg(feature = "alloc")]
extern crate alloc;

mod complement;
mod difference;
mod indirect;
mod intersection;
mod layered;
mod ops;
mod option;
mod primitive;
mod union;

pub use self::{
    complement::Complement, difference::Difference, intersection::Intersection, layered::Layered,
    ops::*, union::Union,
};

pub type Bits1 = bool;
pub type Bits8 = u8;
pub type Bits16 = u16;
pub type Bits32 = u32;
pub type Bits64 = u64;
pub type Bits128 = u128;

pub type Bits256 = Layered<u32, u8, 32>;
pub type Bits512 = Layered<u64, u8, 64>;
pub type Bits1024 = Layered<u64, u16, 64>;
pub type Bits2048 = Layered<u64, u32, 64>;
pub type Bits4096 = Layered<u64, u64, 64>;
pub type Bits8192 = Layered<u64, u128, 64>;
pub type Bits16384 = Layered<u128, u128, 128>;

#[cfg(feature = "alloc")]
pub type Bits32768 = Layered<u64, Option<alloc::boxed::Box<Bits512>>, 64>;

#[cfg(feature = "alloc")]
pub type Bits65536 = Layered<u64, Option<alloc::boxed::Box<Bits1024>>, 64>;

#[cfg(feature = "alloc")]
pub type Bits131072 = Layered<u64, Option<alloc::boxed::Box<Bits2048>>, 64>;

#[cfg(feature = "alloc")]
pub type Bits262144 = Layered<u64, Option<alloc::boxed::Box<Bits4096>>, 64>;

#[cfg(feature = "alloc")]
pub type Bits524288 = Layered<u64, Option<alloc::boxed::Box<Bits8192>>, 64>;

#[cfg(feature = "alloc")]
pub type Bits1048576 = Layered<u64, Option<alloc::boxed::Box<Bits16384>>, 64>;

#[cfg(feature = "alloc")]
pub type Bits2097152 = Layered<u64, Option<alloc::boxed::Box<Bits32768>>, 64>;

#[cfg(feature = "alloc")]
pub type Bits4194304 = Layered<u64, Option<alloc::boxed::Box<Bits65536>>, 64>;

#[cfg(feature = "alloc")]
pub type Bits8388608 = Layered<u64, Option<alloc::boxed::Box<Bits131072>>, 64>;

#[cfg(feature = "alloc")]
pub type Bits16777216 = Layered<u64, Option<alloc::boxed::Box<Bits262144>>, 64>;

#[cfg(feature = "alloc")]
pub type Bits33554432 = Layered<u64, Option<alloc::boxed::Box<Bits524288>>, 64>;

#[cfg(feature = "alloc")]
pub type Bits67108864 = Layered<u64, Option<alloc::boxed::Box<Bits1048576>>, 64>;

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
    use core::{array::IntoIter, mem::MaybeUninit};

    let mut result = unsafe {
        // # Safe
        // All elements are `MaybeUninit` and can be uninit.
        MaybeUninit::<[MaybeUninit<O>; N]>::uninit().assume_init()
    };

    for ((slot, left_elem), right_elem) in result
        .iter_mut()
        .zip(IntoIter::new(left_array))
        .zip(IntoIter::new(right_array))
    {
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
