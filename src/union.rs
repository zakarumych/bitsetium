use {
    crate::{intersection::Intersection, ops::*},
    core::fmt::{self, Display},
};

/// Bit-set wrapper that acts like set complement.
///
/// Effectively inverses all bits in the underlying bit-set.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Union<T, U>(pub T, pub U);

impl<T, U> Union<T, U> {
    /// Swap sets of the union.
    pub fn swap_sets(self) -> Union<U, T> {
        Union(self.1, self.0)
    }
}

impl<T, U> Display for Union<T, U>
where
    T: Display,
    U: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Union({}, {})", self.0, self.1)
    }
}

impl<T, U> BitEmpty for Union<T, U>
where
    T: BitEmpty,
    U: BitEmpty,
{
    fn empty() -> Self {
        Union(T::empty(), U::empty())
    }
}

impl<T, U> BitFull for Union<T, U>
where
    T: BitFull,
    U: Default,
{
    fn full() -> Self {
        Union(T::full(), U::default())
    }
}

impl<T, U> BitTest for Union<T, U>
where
    T: BitTest,
    U: BitTest,
{
    fn test(&self, idx: usize) -> bool {
        self.0.test(idx) || self.1.test(idx)
    }
}

impl<T, U> BitTestNone for Union<T, U>
where
    T: BitTestNone,
    U: BitTestNone,
{
    fn test_none(&self) -> bool {
        self.0.test_none() && self.1.test_none()
    }
}

impl<T, U> BitTestAll for Union<T, U>
where
    T: BitTestAll,
    U: BitTestAll,
{
    fn test_all(&self) -> bool {
        self.0.test_all() || self.1.test_all()
    }
}

impl<T, U> BitSetLimit for Union<T, U>
where
    T: BitSetLimit,
    U: BitSetLimit,
{
    const MAX_SET_INDEX: usize = crate::max(T::MAX_SET_INDEX, U::MAX_SET_INDEX);
}

impl<T, U> BitSet for Union<T, U>
where
    T: BitSet,
    U: BitSet,
{
    unsafe fn set_unchecked(&mut self, idx: usize) {
        if idx <= T::MAX_SET_INDEX {
            self.0.set_unchecked(idx);
        } else {
            self.1.set_unchecked(idx);
        }
    }
}

impl<T, U> BitUnsetLimit for Union<T, U>
where
    T: BitUnsetLimit,
    U: BitUnsetLimit,
{
    const MAX_UNSET_INDEX: usize = crate::min(T::MAX_UNSET_INDEX, U::MAX_UNSET_INDEX);
}

impl<T, U> BitUnset for Union<T, U>
where
    T: BitUnset,
    U: BitUnset,
{
    unsafe fn unset_unchecked(&mut self, idx: usize) {
        self.0.unset_unchecked(idx);
        self.1.unset_unchecked(idx);
    }
}

impl<T, U> BitSearch for Union<T, U>
where
    T: BitSearch,
    U: BitSearch,
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

impl<T, U> BitComplement for Union<T, U>
where
    T: BitComplement,
    U: BitComplement,
{
    type Output = Intersection<T::Output, U::Output>;

    fn complement(self) -> Self::Output {
        Intersection(self.0.complement(), self.1.complement())
    }
}

impl<T, U, Y> BitUnion<Y> for Union<T, U>
where
    T: BitUnion<Y>,
{
    type Output = Union<T::Output, U>;

    fn union(self, rhs: Y) -> Self::Output {
        Union(self.0.union(rhs), self.1)
    }
}

impl<T, U, Y> BitIntersection<Y> for Union<T, U>
where
    T: BitIntersection<Y>,
    U: BitIntersection<Y>,
    Y: Copy,
{
    type Output = Union<T::Output, U::Output>;

    fn intersection(self, rhs: Y) -> Self::Output {
        Union(self.0.intersection(rhs), self.1.intersection(rhs))
    }
}

impl<T, U, Y> BitDifference<Y> for Union<T, U>
where
    T: BitDifference<Y>,
    U: BitDifference<Y>,
    Y: Copy,
{
    type Output = Union<T::Output, U::Output>;

    fn difference(self, rhs: Y) -> Self::Output {
        Union(self.0.difference(rhs), self.1.difference(rhs))
    }
}
