use {
    crate::ops::*,
    core::fmt::{self, Display},
};

/// Bit-set wrapper that acts like set comp.
///
/// Effectively inverses all bits in the underlying bitset.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Union<A, B>(pub A, pub B);

impl<A, B> Union<A, B> {
    /// Swap sets of the union.
    pub fn swap_sets(self) -> Union<B, A> {
        Union(self.1, self.0)
    }
}

impl<A, B> Display for Union<A, B>
where
    A: Display,
    B: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Union({}, {})", self.0, self.1)
    }
}

impl<A, B> Bits for Union<A, B>
where
    A: Bits,
    B: Bits,
{
    const MAX_SET_INDEX: usize = crate::max(A::MAX_SET_INDEX, B::MAX_SET_INDEX);
    const MAX_UNSET_INDEX: usize = crate::min(A::MAX_UNSET_INDEX, B::MAX_UNSET_INDEX);

    fn test(&self, idx: usize) -> bool {
        self.0.test(idx) || self.1.test(idx)
    }
}

impl<A, B> BitEmpty for Union<A, B>
where
    A: BitEmpty,
    B: BitEmpty,
{
    fn empty() -> Self {
        Union(A::empty(), B::empty())
    }
}

impl<A, B> BitFull for Union<A, B>
where
    A: BitFull,
    B: BitEmpty,
{
    fn full() -> Self {
        Union(A::full(), B::empty())
    }
}

impl<A, B> BitTestNone for Union<A, B>
where
    A: BitTestNone,
    B: BitTestNone,
{
    fn test_none(&self) -> bool {
        self.0.test_none() && self.1.test_none()
    }
}

impl<A, B> BitTestAll for Union<A, B>
where
    A: BitTestAll,
    B: BitTestAll,
{
    fn test_all(&self) -> bool {
        self.0.test_all() || self.1.test_all()
    }
}

impl<A, B> BitSet for Union<A, B>
where
    A: BitSet,
    B: BitSet,
{
    unsafe fn set_unchecked(&mut self, idx: usize) {
        if idx <= A::MAX_SET_INDEX {
            self.0.set_unchecked(idx);
        } else {
            self.1.set_unchecked(idx);
        }
    }
}

impl<A, B> BitUnset for Union<A, B>
where
    A: BitUnset,
    B: BitUnset,
{
    unsafe fn unset_unchecked(&mut self, idx: usize) {
        self.0.unset_unchecked(idx);
        self.1.unset_unchecked(idx);
    }
}

impl<A, B> BitFind for Union<A, B>
where
    A: BitFind,
    B: BitFind,
{
    fn find_first_set(&self, lower_bound: usize) -> Option<usize> {
        let t = self.0.find_first_set(lower_bound);
        let u = self.1.find_first_set(lower_bound);

        match (t, u) {
            (None, None) => None,
            (Some(t), None) => Some(t),
            (None, Some(u)) => Some(u),
            (Some(t), Some(u)) => Some(t.min(u)),
        }
    }
}

impl<A, B, C, P, I> BitComp for Union<A, B>
where
    A: BitComp<Output = C>,
    B: BitComp<Output = P>,
    C: BitIntersect<P, Output = I>,
    P: Bits,
    I: Bits,
{
    type Output = I;

    fn comp(self) -> I {
        self.0.comp().intersect(self.1.comp())
    }
}

impl<A, B, C, U, N> BitUnion<C> for Union<A, B>
where
    A: BitUnion<C, Output = U>,
    B: Bits,
    C: Bits,
    U: BitUnion<B, Output = N>,
    N: Bits,
{
    type Output = N;

    fn union(self, rhs: C) -> N {
        self.0.union(rhs).union(self.1)
    }
}

impl<A, B, C, I, X, U> BitIntersect<C> for Union<A, B>
where
    A: BitIntersect<C, Output = I>,
    B: BitIntersect<C, Output = X>,
    C: Bits + Copy,
    I: BitUnion<X, Output = U>,
    X: Bits,
    U: Bits,
{
    type Output = U;

    fn intersect(self, rhs: C) -> U {
        self.0.intersect(rhs).union(self.1.intersect(rhs))
    }
}

impl<A, B, C, D, F, U> BitDiff<C> for Union<A, B>
where
    A: BitDiff<C, Output = D>,
    B: BitDiff<C, Output = F>,
    C: Bits + Copy,
    D: BitUnion<F, Output = U>,
    F: Bits,
    U: Bits,
{
    type Output = U;

    fn diff(self, rhs: C) -> U {
        self.0.diff(rhs).union(self.1.diff(rhs))
    }
}

impl<A, B, C, D, F, U> BitSymDiff<C> for Union<A, B>
where
    A: BitSymDiff<C, Output = D>,
    B: BitSymDiff<C, Output = F>,
    C: Bits + Copy,
    D: BitUnion<F, Output = U>,
    F: Bits,
    U: Bits,
{
    type Output = U;

    fn sym_diff(self, rhs: C) -> U {
        self.0.sym_diff(rhs).union(self.1.sym_diff(rhs))
    }
}
