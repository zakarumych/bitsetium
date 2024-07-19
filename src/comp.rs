use {
    crate::ops::*,
    core::fmt::{self, Display},
};

/// Bit-set wrapper that acts like set comp.
///
/// Effectively inverses all bits in the underlying bitset.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Comp<A>(pub A);

impl<A> Comp<A> {
    /// Returns reference to the inner bitset.
    pub fn inner(&self) -> &A {
        &self.0
    }

    /// Returns mutable reference to the inner bitset.
    pub fn inner_mut(&mut self) -> &mut A {
        &mut self.0
    }

    /// Unwraps the `Comp` wrapper and returns the inner bitset.
    pub fn into_inner(self) -> A {
        self.0
    }
}

impl<A> Comp<Comp<A>> {
    /// Convenience function to unwrap double `Comp` wrapper which must yield bitset with same bits set.
    pub fn double_complement_unwrap(self) -> A {
        self.0 .0
    }
}

impl<A> Display for Comp<A>
where
    A: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Inverse({})", self.0)
    }
}

impl<A> Bits for Comp<A>
where
    A: Bits,
{
    const MAX_SET_INDEX: usize = A::MAX_UNSET_INDEX;
    const MAX_UNSET_INDEX: usize = A::MAX_SET_INDEX;

    #[inline]
    fn test(&self, idx: usize) -> bool {
        !self.0.test(idx)
    }
}

impl<A> BitEmpty for Comp<A>
where
    A: BitFull,
{
    #[inline]
    fn empty() -> Self {
        Comp(A::full())
    }
}

impl<A> BitFull for Comp<A>
where
    A: BitEmpty,
{
    #[inline]
    fn full() -> Self {
        Comp(A::empty())
    }
}

impl<A> BitTestNone for Comp<A>
where
    A: BitTestAll,
{
    #[inline]
    fn test_none(&self) -> bool {
        self.0.test_all()
    }
}

impl<A> BitTestAll for Comp<A>
where
    A: BitTestNone,
{
    #[inline]
    fn test_all(&self) -> bool {
        self.0.test_none()
    }
}

impl<A> BitSet for Comp<A>
where
    A: BitUnset,
{
    #[inline]
    unsafe fn set_unchecked(&mut self, idx: usize) {
        self.0.unset_unchecked(idx)
    }
}

impl<A> BitUnset for Comp<A>
where
    A: BitSet,
{
    #[inline]
    unsafe fn unset_unchecked(&mut self, idx: usize) {
        self.0.set_unchecked(idx)
    }
}

impl<A, B> BitDisjoint<B> for Comp<A>
where
    A: Bits,
    B: BitSubset<A>,
{
    #[inline]
    fn is_disjoint(&self, rhs: &B) -> bool {
        rhs.is_subset_of(&self.0)
    }
}

impl<A> BitFind for Comp<Comp<A>>
where
    A: BitFind,
{
    #[inline]
    fn find_first_set(&self, lower_bound: usize) -> Option<usize> {
        self.inner().inner().find_first_set(lower_bound)
    }
}

impl<A> BitComp for Comp<A>
where
    A: Bits,
{
    type Output = A;

    #[inline]
    fn comp(self) -> A {
        self.0
    }
}

impl<A, B, D, C> BitUnion<B> for Comp<A>
where
    A: BitDiff<B, Output = D>,
    B: Bits,
    D: BitComp<Output = C>,
    C: Bits,
{
    type Output = C;

    #[inline]
    fn union(self, rhs: B) -> C {
        self.0.diff(rhs).comp()
    }
}

impl<A, B, D> BitIntersect<B> for Comp<A>
where
    A: Bits,
    B: BitDiff<A, Output = D>,
    D: Bits,
{
    type Output = D;

    #[inline]
    fn intersect(self, rhs: B) -> D {
        rhs.diff(self.0)
    }
}

impl<A, B, U, C> BitDiff<B> for Comp<A>
where
    A: BitUnion<B, Output = U>,
    B: Bits,
    U: BitComp<Output = C>,
    C: Bits,
{
    type Output = C;

    #[inline]
    fn diff(self, rhs: B) -> C {
        self.0.union(rhs).comp()
    }
}

impl<A, B, S, C> BitSymDiff<B> for Comp<A>
where
    A: BitSymDiff<B, Output = S>,
    B: Bits,
    S: BitComp<Output = C>,
    C: Bits,
{
    type Output = C;

    #[inline]
    fn sym_diff(self, rhs: B) -> C {
        self.0.sym_diff(rhs).comp()
    }
}
