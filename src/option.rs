use crate::ops::*;

impl<A> Bits for Option<A>
where
    A: Bits,
{
    const MAX_SET_INDEX: usize = A::MAX_SET_INDEX;
    const MAX_UNSET_INDEX: usize = A::MAX_UNSET_INDEX;

    fn test(&self, idx: usize) -> bool {
        match self {
            None => false,
            Some(bits) => bits.test(idx),
        }
    }
}

impl<A> BitEmpty for Option<A>
where
    A: Bits,
{
    fn empty() -> Self {
        None
    }
}

impl<A> BitFull for Option<A>
where
    A: BitFull,
{
    fn full() -> Self {
        Some(A::full())
    }
}

impl<A> BitTestNone for Option<A>
where
    A: BitTestNone,
{
    fn test_none(&self) -> bool {
        match self {
            None => true,
            Some(bits) => bits.test_none(),
        }
    }
}

impl<A> BitTestAll for Option<A>
where
    A: BitTestAll,
{
    fn test_all(&self) -> bool {
        match self {
            None => false,
            Some(bits) => bits.test_all(),
        }
    }
}

impl<A> BitSet for Option<A>
where
    A: BitSet + BitEmpty,
{
    unsafe fn set_unchecked(&mut self, idx: usize) {
        self.get_or_insert_with(A::empty).set_unchecked(idx)
    }
}

impl<A> BitUnset for Option<A>
where
    A: BitUnset + BitTestNone,
{
    unsafe fn unset_unchecked(&mut self, idx: usize) {
        if let Some(bits) = self {
            bits.unset_unchecked(idx);
            if bits.test_none() {
                *self = None;
            }
        }
    }
}

impl<A> BitFind for Option<A>
where
    A: BitFind,
{
    fn find_first_set(&self, lower_bound: usize) -> Option<usize> {
        match self {
            None => None,
            Some(bits) => bits.find_first_set(lower_bound),
        }
    }
}

impl<A, C> BitComp for Option<A>
where
    A: BitComp<Output = C>,
    C: BitFull,
{
    type Output = Option<C>;

    fn comp(self) -> Option<C> {
        match self {
            None => Some(C::full()),
            Some(bits) => Some(bits.comp()),
        }
    }
}

impl<A, B> BitUnion<B> for Option<A>
where
    A: BitUnion<B>,
    B: Bits + Into<A::Output>,
{
    type Output = A::Output;

    fn union(self, rhs: B) -> A::Output {
        match self {
            None => rhs.into(),
            Some(lhs) => lhs.union(rhs),
        }
    }
}

impl<A, B, I> BitIntersect<B> for Option<A>
where
    A: BitIntersect<B, Output = I>,
    B: Bits,
    I: Bits,
{
    type Output = Option<I>;

    fn intersect(self, rhs: B) -> Option<I> {
        match self {
            None => None,
            Some(lhs) => Some(lhs.intersect(rhs)),
        }
    }
}

impl<A, B, D> BitDiff<B> for Option<A>
where
    A: BitDiff<B, Output = D>,
    B: Bits,
    D: Bits,
{
    type Output = Option<D>;

    fn diff(self, rhs: B) -> Option<D> {
        match self {
            None => None,
            Some(lhs) => Some(lhs.diff(rhs)),
        }
    }
}

impl<A, B, D> BitSymDiff<B> for Option<A>
where
    A: BitSymDiff<B, Output = D>,
    B: Bits,
    D: Bits + From<B>,
{
    type Output = D;

    fn sym_diff(self, rhs: B) -> D {
        match self {
            None => D::from(rhs),
            Some(lhs) => lhs.sym_diff(rhs),
        }
    }
}

impl<A, B> BitSubset<B> for Option<A>
where
    A: BitSubset<B>,
    B: Bits,
{
    fn is_subset_of(&self, rhs: &B) -> bool {
        match self {
            None => true,
            Some(lhs) => lhs.is_subset_of(rhs),
        }
    }
}

impl<A, B> BitDisjoint<B> for Option<A>
where
    A: BitDisjoint<B>,
    B: Bits,
{
    fn is_disjoint(&self, rhs: &B) -> bool {
        match self {
            None => true,
            Some(lhs) => lhs.is_disjoint(rhs),
        }
    }
}
