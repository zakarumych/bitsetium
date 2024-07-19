use {
    crate::ops::*,
    core::fmt::{self, Display},
};

/// Bit-set wrapper that acts like set complement.
///
/// Effectively inverses all bits in the underlying bitset.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Diff<A, B>(pub A, pub B);

impl<A, B> Display for Diff<A, B>
where
    A: Display,
    B: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Diff({}, {})", self.0, self.1)
    }
}

impl<A, B> Bits for Diff<A, B>
where
    A: Bits,
    B: Bits,
{
    const MAX_SET_INDEX: usize = crate::min(A::MAX_SET_INDEX, B::MAX_UNSET_INDEX);
    const MAX_UNSET_INDEX: usize = crate::max(A::MAX_UNSET_INDEX, B::MAX_SET_INDEX);

    fn test(&self, idx: usize) -> bool {
        self.0.test(idx) && !self.1.test(idx)
    }
}

impl<A, B> BitEmpty for Diff<A, B>
where
    A: BitEmpty,
    B: BitFull,
{
    #[inline]
    fn empty() -> Self {
        Diff(A::empty(), B::full())
    }
}

impl<A, B> BitFull for Diff<A, B>
where
    A: BitFull,
    B: BitEmpty,
{
    #[inline]
    fn full() -> Self {
        Diff(A::full(), B::empty())
    }
}

impl<A, B> BitTestNone for Diff<A, B>
where
    A: BitSubset<B>,
    B: Bits,
{
    #[inline]
    fn test_none(&self) -> bool {
        self.0.is_subset_of(&self.1)
    }
}

impl<A, B> BitTestAll for Diff<A, B>
where
    A: BitTestAll,
    B: BitTestNone,
{
    #[inline]
    fn test_all(&self) -> bool {
        self.0.test_all() && self.1.test_none()
    }
}

impl<A, B> BitSet for Diff<A, B>
where
    A: BitSet,
    B: BitUnset,
{
    #[inline]
    unsafe fn set_unchecked(&mut self, idx: usize) {
        self.0.set_unchecked(idx);
        self.1.unset_unchecked(idx);
    }
}

impl<A, B> BitUnset for Diff<A, B>
where
    A: BitUnset,
    B: BitSet,
{
    #[inline]
    unsafe fn unset_unchecked(&mut self, idx: usize) {
        if idx <= A::MAX_UNSET_INDEX {
            self.0.unset_unchecked(idx);
        } else {
            self.1.set_unchecked(idx);
        }
    }
}

impl<A, B> BitFind for Diff<A, B>
where
    A: BitFind,
    B: Bits,
{
    fn find_first_set(&self, lower_bound: usize) -> Option<usize> {
        let mut idx = self.0.find_first_set(lower_bound)?;
        loop {
            if !self.1.test(idx) {
                return Some(idx);
            } else if idx < usize::MAX {
                idx = self.0.find_first_set(idx + 1)?;
            } else {
                return None;
            }
        }
    }
}

impl<A, B, C, U> BitComp for Diff<A, B>
where
    A: BitComp<Output = C>,
    B: Bits,
    C: BitUnion<B, Output = U>,
    U: Bits,
{
    type Output = U;

    #[inline]
    fn comp(self) -> U {
        self.0.comp().union(self.1)
    }
}
