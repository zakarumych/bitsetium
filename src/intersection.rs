use {
    crate::{ops::*, union::Union},
    core::fmt::{self, Display},
};

/// Bit-set wrapper that acts like set complement.
///
/// Effectively inverses all bits in the underlying bit-set.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Intersection<T, U>(pub T, pub U);

impl<T, U> Intersection<T, U> {
    /// Swap sets of the intersection.
    pub fn swap_sets(self) -> Intersection<U, T> {
        Intersection(self.1, self.0)
    }
}

impl<T, U> Display for Intersection<T, U>
where
    T: Display,
    U: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Intersection({}, {})", self.0, self.1)
    }
}

impl<T, U> BitEmpty for Intersection<T, U>
where
    T: BitEmpty,
    U: Default,
{
    fn empty() -> Self {
        Intersection(T::empty(), U::default())
    }
}

impl<T, U> BitFull for Intersection<T, U>
where
    T: BitFull,
    U: BitFull,
{
    fn full() -> Self {
        Intersection(T::full(), U::full())
    }
}

impl<T, U> BitTest for Intersection<T, U>
where
    T: BitTest,
    U: BitTest,
{
    fn test(&self, idx: usize) -> bool {
        self.0.test(idx) && self.1.test(idx)
    }
}

impl<T, U> BitTestNone for Intersection<T, U>
where
    T: BitDisjoint<U>,
{
    fn test_none(&self) -> bool {
        self.0.is_disjoint(&self.1)
    }
}

impl<T, U> BitTestAll for Intersection<T, U>
where
    T: BitTestAll,
    U: BitTestAll,
{
    fn test_all(&self) -> bool {
        self.0.test_all() && self.1.test_all()
    }
}

impl<T, U> BitSetLimit for Intersection<T, U>
where
    T: BitSetLimit,
    U: BitSetLimit,
{
    const MAX_SET_INDEX: usize = crate::min(T::MAX_SET_INDEX, U::MAX_SET_INDEX);
}

impl<T, U> BitSet for Intersection<T, U>
where
    T: BitSet,
    U: BitSet,
{
    unsafe fn set_unchecked(&mut self, idx: usize) {
        self.0.set_unchecked(idx);
        self.1.set_unchecked(idx);
    }
}

impl<T, U> BitUnsetLimit for Intersection<T, U>
where
    T: BitUnsetLimit,
    U: BitUnsetLimit,
{
    const MAX_UNSET_INDEX: usize = crate::max(T::MAX_UNSET_INDEX, U::MAX_UNSET_INDEX);
}

impl<T, U> BitUnset for Intersection<T, U>
where
    T: BitUnset,
    U: BitUnset,
{
    unsafe fn unset_unchecked(&mut self, idx: usize) {
        if idx <= T::MAX_UNSET_INDEX {
            self.0.unset_unchecked(idx);
        } else {
            self.1.unset_unchecked(idx);
        }
    }
}

impl<T, U> BitSearch for Intersection<T, U>
where
    T: BitSearch,
    U: BitSearch,
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

impl<T, U> BitComplement for Intersection<T, U>
where
    T: BitComplement,
    U: BitComplement,
{
    type Output = Union<T::Output, U::Output>;

    fn complement(self) -> Self::Output {
        Union(self.0.complement(), self.1.complement())
    }
}

impl<T, U, Y> BitUnion<Y> for Intersection<T, U>
where
    T: BitUnion<Y>,
    U: BitUnion<Y>,
    Y: Copy,
{
    type Output = Intersection<T::Output, U::Output>;

    fn union(self, rhs: Y) -> Self::Output {
        Intersection(self.0.union(rhs), self.1.union(rhs))
    }
}

impl<T, U, Y> BitIntersection<Y> for Intersection<T, U>
where
    T: BitIntersection<Y>,
{
    type Output = Intersection<T::Output, U>;

    fn intersection(self, rhs: Y) -> Self::Output {
        Intersection(self.0.intersection(rhs), self.1)
    }
}

impl<T, U, Y> BitDifference<Y> for Intersection<T, U>
where
    T: BitDifference<Y>,
{
    type Output = Intersection<T::Output, U>;

    fn difference(self, rhs: Y) -> Self::Output {
        Intersection(self.0.difference(rhs), self.1)
    }
}
