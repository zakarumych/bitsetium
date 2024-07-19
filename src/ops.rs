//! Traits to define operations on bitset.

/// Basic trait for bit-sets.
pub trait Bits {
    /// Largest possible bit index that can be set.
    /// Any larger index will always be unset.
    /// Setting larger index is now allowed.
    ///
    /// Unbound bit-sets should specify `MAX_SET_INDEX = usize::MAX;
    const MAX_SET_INDEX: usize;

    /// Largest possible bit index that can be unset.
    /// Any larger index will always be set.
    /// Un-setting larger index is now allowed.
    ///
    /// Unbound bit-sets should specify `MAX_UNSET_INDEX = usize::MAX;
    ///
    /// Most types are unbound, except [`Complement`] wrapper
    /// that flips this constraint with `MAX_SET_INDEX`.
    const MAX_UNSET_INDEX: usize;

    /// Tests if bit at specified index is set.
    /// This operation is supported on all bitset types.
    fn test(&self, idx: usize) -> bool;
}

/// Trait for bitset types that can be created empty.
pub trait BitEmpty: Bits {
    /// Returns empty bitset for which all bits are unset.
    fn empty() -> Self;
}

/// Trait for bitset types that can be created full.
pub trait BitFull: Bits {
    /// Returns full bitset for which all bits are set.
    fn full() -> Self;
}

/// Trait to test if no bit is set.
pub trait BitTestNone: Bits {
    /// Tests if none bits is set.
    fn test_none(&self) -> bool;
}

/// Trait to test if all bits are set.
pub trait BitTestAll: Bits {
    /// Tests if all bits are set.
    fn test_all(&self) -> bool;
}

/// Trait to set single bit in the bitset.
pub trait BitSet: Bits {
    /// Sets bit at specified index.
    ///
    /// # Panics
    ///
    /// Calling with `idx` greater than [`MAX_SET_INDEX`](Bits::MAX_SET_INDEX) may panic.
    #[inline]
    #[track_caller]
    fn set(&mut self, idx: usize) {
        assert!(
            idx <= Self::MAX_SET_INDEX,
            "index {idx} is out of bounds. Valid range is `0..{}`.",
            Self::MAX_SET_INDEX
        );
        unsafe {
            // # Safe
            // Condition is checked above.
            self.set_unchecked(idx)
        }
    }

    /// Sets bit at specified index.
    ///
    /// # Safety
    ///
    /// Calling with `idx` greater than [`MAX_SET_INDEX`](Bits::MAX_SET_INDEX) may trigger UB.
    /// For any valid `idx` behavior is identical to [`set`](BitSet::set), but may produce better optimized code.
    ///
    /// Implementations are encouraged to use debug assertions.
    unsafe fn set_unchecked(&mut self, idx: usize);
}

/// Unset single bit.
/// This trait should be implemented for most mutable bitset that support resetting bits.
pub trait BitUnset: Bits {
    /// Un-sets bit at specified index.
    ///
    /// # Panics
    ///
    /// Calling with `idx` greater than [`MAX_UNSET_INDEX`](Bits::MAX_UNSET_INDEX) may panic.
    #[inline]
    #[track_caller]
    fn unset(&mut self, idx: usize) {
        assert!(
            idx <= Self::MAX_UNSET_INDEX,
            "index {idx} is out of bounds. Valid range is `0..{}`.",
            Self::MAX_SET_INDEX
        );
        unsafe {
            // # Safe
            // Condition is checked above.
            self.unset_unchecked(idx)
        }
    }

    /// Un-sets bit at specified index.
    ///
    /// # Safety
    ///
    /// Calling with `idx` greater than [`MAX_UNSET_INDEX`](Bits::MAX_UNSET_INDEX) may trigger UB.
    /// For any valid `idx` behavior is identical to [`unset`](BitUnset::unset), but may produce better optimized code.
    ///
    /// Implementations are encouraged to use debug assertions.
    unsafe fn unset_unchecked(&mut self, idx: usize);
}

/// Trait to find first bit set in the bitset.
pub trait BitFind: Bits {
    /// Searches for first bit set starting at `lower_bound`.
    /// Returns found bit index.
    /// If no bit is set starting at `lower_bound`, returns `None`.
    ///
    /// # Example
    ///
    /// ```
    /// # use {core::ops::Range, bitsetium::*};
    /// fn validate<S: BitSearch + BitSetLimit + BitTest>(set: &S, lower_bound: usize) {
    ///   match set.find_first_set(lower_bound) {
    ///     None => assert!((lower_bound..=S::MAX_SET_INDEX).all(|idx| !set.test(idx))),
    ///     Some(idx) => {
    ///       assert!(idx <= S::MAX_SET_INDEX);
    ///       assert!((lower_bound..idx).all(|idx| !set.test(idx)));
    ///       assert!(set.test(idx));
    ///     }
    ///   }
    /// }
    /// ```
    fn find_first_set(&self, lower_bound: usize) -> Option<usize>;

    /// Searches for first bit set in the specified range.
    /// Returns found bit index.
    /// If no bit is set in the specified range, returns `None`.
    ///
    /// # Example
    ///
    /// ```
    /// # use {core::ops::Range, bitsetium::*};
    /// fn validate<S: BitSearch + BitTest>(set: &S, mut range: Range<usize>) {
    ///   match set.find_set_in_range(range.clone()) {
    ///     None => assert!(range.all(|idx| !set.test(idx))),
    ///     Some(idx) => {
    ///       assert!(idx < range.end);
    ///       assert!((range.start..idx).all(|idx| !set.test(idx)));
    ///       assert!(set.test(idx));
    ///     }
    ///   }
    /// }
    /// ```
    #[inline]
    fn find_set_in_range<R>(&self, range: R) -> Option<usize>
    where
        R: core::ops::RangeBounds<usize>,
    {
        use core::ops::Bound;

        let lower_bound = match range.start_bound() {
            Bound::Included(bound) => *bound,
            Bound::Excluded(bound) => *bound + 1,
            Bound::Unbounded => 0,
        };

        self.find_first_set(lower_bound)
            .and_then(|idx| match range.end_bound() {
                Bound::Included(bound) => {
                    if *bound >= idx {
                        Some(idx)
                    } else {
                        None
                    }
                }
                Bound::Excluded(bound) => {
                    if *bound > idx {
                        Some(idx)
                    } else {
                        None
                    }
                }
                Bound::Unbounded => Some(idx),
            })
    }
}

/// Trait to get complement of the bitset.
pub trait BitComp: Bits {
    /// Type of bitset complement.
    type Output: Bits;

    /// Returns complement of bitset.
    fn comp(self) -> Self::Output;
}

/// Trait to get union of two bitset.
pub trait BitUnion<Rhs: Bits = Self>: Bits {
    /// Type of bitset union.
    type Output: Bits;

    /// Returns bitset with bits set for each index that has bit set in both of two arguments.
    fn union(self, rhs: Rhs) -> Self::Output;
}

/// Trait to get intersection of two bitset.
pub trait BitIntersect<Rhs: Bits = Self>: Bits {
    /// Type of bitset intersection.
    type Output: Bits;

    /// Returns bitset with bits set for each index that has bit set in either of two arguments.
    fn intersect(self, rhs: Rhs) -> Self::Output;
}

/// Difference between two subsets.
pub trait BitDiff<Rhs: Bits = Self>: Bits {
    /// Type of bitset difference.
    type Output: Bits;

    /// Returns bitset has bits set for each index that has bit set in first but not in second argument.
    fn diff(self, rhs: Rhs) -> Self::Output;
}

/// Symmetric difference between two subsets.
pub trait BitSymDiff<Rhs: Bits = Self>: Bits {
    /// Type of bitset symmetric difference.
    type Output: Bits;

    /// Returns bitset has bits set for each index that has bit set in exactly one of two arguments.
    fn sym_diff(self, rhs: Rhs) -> Self::Output;
}

/// Tests if one bitset is subset of another.
/// That is, all bits set in `self` are also set in `rhs`.
pub trait BitSubset<Rhs: Bits = Self>: Bits {
    /// Returns true if `self` is subset of `rhs`.
    fn is_subset_of(&self, rhs: &Rhs) -> bool;
}

/// Tests if two bit-sets are disjoint.
/// That is, no bit is set in both `self` and `rhs`.
pub trait BitDisjoint<Rhs: Bits = Self>: Bits {
    /// Returns true if `self` is disjoint with `rhs`.
    fn is_disjoint(&self, rhs: &Rhs) -> bool;
}

/// Trait for bitset types that support all operations.
pub trait UltimateBitSet:
    Sized
    + BitEmpty
    + BitFull
    + BitTestNone
    + BitTestAll
    + BitSet
    + BitUnset
    + BitFind
    + BitComp
    + BitUnion
    + BitIntersect
    + BitDiff
    + BitSymDiff
    + BitSubset
    + BitDisjoint
{
}

impl<T> UltimateBitSet for T where
    T: BitEmpty
        + BitFull
        + BitTestNone
        + BitTestAll
        + BitSet
        + BitUnset
        + BitFind
        + BitComp
        + BitUnion
        + BitIntersect
        + BitDiff
        + BitSymDiff
        + BitSubset
        + BitDisjoint
{
}
