//! Traits to define operations on bit-set.

pub trait BitEmpty {
    /// Returns empty bitset for which all bits are unset.
    fn empty() -> Self;
}

pub trait BitFull {
    /// Returns full bitset for which all bits are set.
    fn full() -> Self;
}

/// Test single bit.
/// This trait should be implemented for all bit-set.
pub trait BitTest {
    /// Tests if bit at specified index is set.
    fn test(&self, idx: usize) -> bool;
}

/// Test any bit.
/// This trait should be implemented for most bit-set.
pub trait BitTestNone {
    /// Tests if none bits is set.
    fn test_none(&self) -> bool;
}

/// Test all bit.
/// This trait should be implemented for most bit-set.
pub trait BitTestAll {
    /// Tests if all bits are set.
    fn test_all(&self) -> bool;
}

/// Trait to define static limit on set bits.
pub trait BitSetLimit {
    /// Largest possible bit index that can be set.
    /// Any larger index will always be unset.
    /// Setting larger index is now allowed.
    ///
    /// Unbound bit-sets should specify `MAX_SET_INDEX = usize::MAX;
    const MAX_SET_INDEX: usize;
}

/// Sets single bit.
/// This trait should be implemented for all mutable bit-set.
///
/// Note that not all kind of bit-sets may support resetting bits.
pub trait BitSet: BitSetLimit {
    /// Sets bit at specified index.
    ///
    /// # Panics
    ///
    /// Calling with `idx > MAX_SET_INDEX` should panic.
    ///
    /// Implementations are encouraged to use assertions.
    #[inline]
    fn set(&mut self, idx: usize) {
        assert!(idx <= Self::MAX_SET_INDEX, "Idx out of bounds");
        unsafe {
            // # Safe
            // Condition is checked above.
            self.set_unchecked(idx)
        }
    }

    /// Set bit at specified index.
    ///
    /// # Safety
    ///
    /// Calling with `idx > MAX_SET_INDEX` may trigger UB.
    /// For any `idx <= MAX_SET_INDEX` behavior is identical to `set`, but may produce better optimized code.
    ///
    /// Implementations are encouraged to use debug assertions.
    unsafe fn set_unchecked(&mut self, idx: usize);
}

/// Trait to define static limit on set bits.
pub trait BitUnsetLimit {
    /// Largest possible bit index that can be unset.
    /// Any larger index will always be set.
    /// Unsetting larger index is now allowed.
    ///
    /// Unbound bit-sets should specify `MAX_UNSET_INDEX = usize::MAX;
    const MAX_UNSET_INDEX: usize;
}

/// Unset single bit.
/// This trait should be implemented for most mutable bit-set that support resetting bits.
pub trait BitUnset: BitUnsetLimit {
    /// Unsets bit at specified index.
    ///
    /// # Panics
    ///
    /// Calling with `idx > MAX_UNSET_INDEX` should panic.
    ///
    /// Implementations are encouraged to use assertions.
    #[inline]
    fn unset(&mut self, idx: usize) {
        assert!(idx <= Self::MAX_UNSET_INDEX, "Idx out of bounds");
        unsafe {
            // # Safe
            // Condition is checked above.
            self.unset_unchecked(idx)
        }
    }

    /// Unsets bit at specified index.
    ///
    /// # Safety
    ///
    /// Calling with `idx > MAX_UNSET_INDEX` may trigger UB.
    /// For any `idx <= MAX_UNSET_INDEX` behavior is identical to `unset`, but may produce better optimized code.
    ///
    /// Implementations are encouraged to use debug assertions.
    unsafe fn unset_unchecked(&mut self, idx: usize);
}

/// Search for set its.
pub trait BitSearch {
    /// Searches for first bit set starting with `lower_bound`.
    /// Returns index of first bit set.
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

    /// Searches for bit set specified range.
    /// Returns index of bit set.
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

/// Trait to get dual set to the given.
pub trait BitComplement {
    type Output;

    fn complement(self) -> Self::Output;
}

/// Union of bit-sets.
pub trait BitUnion<Rhs = Self> {
    type Output;

    /// Returns bit-set with bits set for each index that has bit set in both of two arguments.
    fn union(self, rhs: Rhs) -> Self::Output;
}

/// Intersection of bit-sets.
pub trait BitIntersection<Rhs = Self> {
    type Output;

    /// Returns bit-set with bits set for each index that has bit set in either of two arguments.
    fn intersection(self, rhs: Rhs) -> Self::Output;
}

/// Difference between two subsets.
pub trait BitDifference<Rhs = Self> {
    type Output;

    /// Returns bit-set has bits set for each index that has bit set in exactly one of two arguments.
    fn difference(self, rhs: Rhs) -> Self::Output;
}

/// Tests any of the following equivalent properties:
/// - one bit-set is subset of another.
/// - all bits that are set in one bit-set are set in another.
/// - `self - rhs` is empty.
pub trait BitSubset<Rhs = Self> {
    /// Returns true if `self` is subset of `rhs`.
    fn is_subset_of(&self, rhs: &Rhs) -> bool;
}

/// Tests any of the following equivalent properties:
/// - two bit-sets are disjoint.
/// - all bits that are set in one bit-set are unset in another and vice versa.
/// - their intersection is empty.
pub trait BitDisjoint<Rhs = Self> {
    /// Returns true if `self` is disjoint with `rhs`.
    fn is_disjoint(&self, rhs: &Rhs) -> bool;
}

/// BitSet that supports all operations.
pub trait UltimateBitSet:
    Sized
    + BitEmpty
    + BitFull
    + BitTest
    + BitTestNone
    + BitTestAll
    + BitSet
    + BitUnset
    + BitSearch
    + BitComplement
    + BitUnion
    + BitIntersection
    + BitDifference
    + BitSubset
    + BitDisjoint
{
}

impl<T> UltimateBitSet for T where
    T: BitEmpty
        + BitFull
        + BitTest
        + BitTestNone
        + BitTestAll
        + BitSet
        + BitUnset
        + BitSearch
        + BitComplement
        + BitUnion
        + BitIntersection
        + BitDifference
        + BitSubset
        + BitDisjoint
{
}
