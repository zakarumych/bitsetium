use {
    crate::ops::*,
    core::fmt::{self, Display},
};

/// Bit-set wrapper that acts like set complement.
///
/// Effectively inverses all bits in the underlying bitset.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Intersect<A, B>(pub A, pub B);

impl<A, B> Intersect<A, B> {
    /// Swap sets of the intersection.
    pub fn swap_sets(self) -> Intersect<B, A> {
        Intersect(self.1, self.0)
    }
}

impl<A, B> Display for Intersect<A, B>
where
    A: Display,
    B: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Intersect({}, {})", self.0, self.1)
    }
}

impl<A, B> Bits for Intersect<A, B>
where
    A: Bits,
    B: Bits,
{
    const MAX_SET_INDEX: usize = crate::min(A::MAX_SET_INDEX, B::MAX_SET_INDEX);
    const MAX_UNSET_INDEX: usize = crate::max(A::MAX_UNSET_INDEX, B::MAX_UNSET_INDEX);

    fn test(&self, idx: usize) -> bool {
        self.0.test(idx) && self.1.test(idx)
    }
}

impl<A, B> BitEmpty for Intersect<A, B>
where
    A: BitEmpty,
    B: BitEmpty,
{
    fn empty() -> Self {
        Intersect(A::empty(), B::empty())
    }
}

impl<A, B> BitFull for Intersect<A, B>
where
    A: BitFull,
    B: BitFull,
{
    fn full() -> Self {
        Intersect(A::full(), B::full())
    }
}

impl<A, B> BitTestNone for Intersect<A, B>
where
    A: BitDisjoint<B>,
    B: Bits,
{
    fn test_none(&self) -> bool {
        self.0.is_disjoint(&self.1)
    }
}

impl<A, B> BitTestAll for Intersect<A, B>
where
    A: BitTestAll,
    B: BitTestAll,
{
    fn test_all(&self) -> bool {
        self.0.test_all() && self.1.test_all()
    }
}

impl<A, B> BitSet for Intersect<A, B>
where
    A: BitSet,
    B: BitSet,
{
    unsafe fn set_unchecked(&mut self, idx: usize) {
        self.0.set_unchecked(idx);
        self.1.set_unchecked(idx);
    }
}

impl<A, B> BitUnset for Intersect<A, B>
where
    A: BitUnset,
    B: BitUnset,
{
    unsafe fn unset_unchecked(&mut self, idx: usize) {
        if idx <= A::MAX_UNSET_INDEX {
            self.0.unset_unchecked(idx);
        } else {
            self.1.unset_unchecked(idx);
        }
    }
}

impl<A, B> BitFind for Intersect<A, B>
where
    A: BitFind,
    B: BitFind,
{
    fn find_first_set(&self, lower_bound: usize) -> Option<usize> {
        let mut t = self.0.find_first_set(lower_bound)?;
        let mut u = self.1.find_first_set(lower_bound)?;

        loop {
            if t == u {
                return Some(t);
            } else if t < u {
                t = self.0.find_first_set(t + 1)?;
            } else {
                u = self.1.find_first_set(u + 1)?;
            }
        }
    }
}

impl<A, B, C, P, U> BitComp for Intersect<A, B>
where
    A: BitComp<Output = C>,
    B: BitComp<Output = P>,
    C: BitUnion<P, Output = U>,
    P: Bits,
    U: Bits,
{
    type Output = U;

    fn comp(self) -> U {
        self.0.comp().union(self.1.comp())
    }
}

impl<A, B, C, U, X, I> BitUnion<C> for Intersect<A, B>
where
    A: BitUnion<C, Output = U>,
    B: BitUnion<C, Output = X>,
    C: Copy + Bits,
    U: BitIntersect<X, Output = I>,
    X: Bits,
    I: Bits,
{
    type Output = I;

    fn union(self, rhs: C) -> I {
        self.0.union(rhs).intersect(self.1.union(rhs))
    }
}

impl<A, B, C, I, X> BitIntersect<C> for Intersect<A, B>
where
    A: BitIntersect<C, Output = I>,
    B: Bits,
    C: Bits,
    I: BitIntersect<B, Output = X>,
    X: Bits,
{
    type Output = X;

    fn intersect(self, rhs: C) -> Self::Output {
        self.0.intersect(rhs).intersect(self.1)
    }
}

impl<A, B, C, I, X> BitDiff<C> for Intersect<A, B>
where
    A: BitDiff<C, Output = I>,
    I: BitIntersect<B, Output = X>,
    B: Bits,
    C: Bits,
    X: Bits,
{
    type Output = X;

    fn diff(self, rhs: C) -> X {
        self.0.diff(rhs).intersect(self.1)
    }
}
