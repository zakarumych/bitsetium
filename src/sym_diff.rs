use {
    crate::ops::*,
    core::fmt::{self, Display},
};

/// Bit-set wrapper that acts like set complement.
///
/// Effectively inverses all bits in the underlying bitset.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SymDiff<A, B>(pub A, pub B);

impl<A, B> SymDiff<A, B> {
    /// Swap sets of the union.
    pub fn swap_sets(self) -> SymDiff<B, A> {
        SymDiff(self.1, self.0)
    }
}

impl<A, B> Display for SymDiff<A, B>
where
    A: Display,
    B: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SymDiff({}, {})", self.0, self.1)
    }
}

impl<A, B> Bits for SymDiff<A, B>
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

impl<A, B> BitEmpty for SymDiff<A, B>
where
    A: BitEmpty,
    B: BitEmpty,
{
    fn empty() -> Self {
        SymDiff(A::empty(), B::empty())
    }
}

impl<A, B> BitFull for SymDiff<A, B>
where
    A: BitFull,
    B: BitEmpty,
{
    fn full() -> Self {
        SymDiff(A::full(), B::empty())
    }
}

impl<A, B> BitTestNone for SymDiff<A, B>
where
    A: BitSubset<B>,
    B: Bits,
{
    fn test_none(&self) -> bool {
        self.0.is_subset_of(&self.1)
    }
}

impl<A, B> BitTestAll for SymDiff<A, B>
where
    A: BitTestAll,
    B: BitTestNone,
{
    fn test_all(&self) -> bool {
        self.0.test_all() && self.1.test_none()
    }
}

impl<A, B> BitSet for SymDiff<A, B>
where
    A: BitSet,
    B: BitUnset,
{
    unsafe fn set_unchecked(&mut self, idx: usize) {
        self.0.set_unchecked(idx);
        self.1.unset_unchecked(idx);
    }
}

impl<A, B> BitUnset for SymDiff<A, B>
where
    A: BitUnset,
    B: BitSet,
{
    unsafe fn unset_unchecked(&mut self, idx: usize) {
        if idx <= A::MAX_UNSET_INDEX {
            self.0.unset_unchecked(idx);
        } else {
            self.1.set_unchecked(idx);
        }
    }
}

impl<A, B> BitFind for SymDiff<A, B>
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

impl<A, B, C, S> BitComp for SymDiff<A, B>
where
    A: BitComp<Output = C>,
    B: Bits,
    C: BitSymDiff<B, Output = S>,
    S: Bits,
{
    type Output = S;

    fn comp(self) -> S {
        self.0.comp().sym_diff(self.1)
    }
}

impl<A, B, C, U, D, S> BitUnion<C> for SymDiff<A, B>
where
    A: BitUnion<C, Output = U>,
    B: BitDiff<C, Output = D>,
    C: Bits + Copy,
    U: BitSymDiff<D, Output = S>,
    D: Bits,
    S: Bits,
{
    type Output = S;

    fn union(self, rhs: C) -> S {
        self.0.union(rhs).sym_diff(self.1.diff(rhs))
    }
}

impl<A, B, C, I, X, S> BitIntersect<C> for SymDiff<A, B>
where
    A: BitIntersect<C, Output = I>,
    B: BitIntersect<C, Output = X>,
    C: Bits + Copy,
    I: BitSymDiff<X, Output = S>,
    X: Bits,
    S: Bits,
{
    type Output = S;

    fn intersect(self, rhs: C) -> S {
        self.0.intersect(rhs).sym_diff(self.1.intersect(rhs))
    }
}

impl<A, B, C, S, D> BitSymDiff<C> for SymDiff<A, B>
where
    A: BitSymDiff<C, Output = S>,
    B: Bits,
    C: Bits,
    S: BitSymDiff<B, Output = D>,
    D: Bits,
{
    type Output = D;

    fn sym_diff(self, rhs: C) -> D {
        self.0.sym_diff(rhs).sym_diff(self.1)
    }
}
