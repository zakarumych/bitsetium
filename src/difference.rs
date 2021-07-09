use {
    crate::{ops::*, union::Union},
    core::fmt::{self, Display},
};

/// Bit-set wrapper that acts like set complement.
///
/// Effectively inverses all bits in the underlying bit-set.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Difference<T, U>(pub T, pub U);

impl<T, U> Display for Difference<T, U>
where
    T: Display,
    U: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Difference({}, {})", self.0, self.1)
    }
}

impl<T, U> BitEmpty for Difference<T, U>
where
    T: BitEmpty,
    U: Default,
{
    fn empty() -> Self {
        Difference(T::empty(), U::default())
    }
}

impl<T, U> BitFull for Difference<T, U>
where
    T: BitFull,
    U: BitEmpty,
{
    fn full() -> Self {
        Difference(T::full(), U::empty())
    }
}

impl<T, U> BitTest for Difference<T, U>
where
    T: BitTest,
    U: BitTest,
{
    fn test(&self, idx: usize) -> bool {
        self.0.test(idx) && !self.1.test(idx)
    }
}

impl<T, U> BitTestNone for Difference<T, U>
where
    T: BitSubset<U>,
{
    fn test_none(&self) -> bool {
        self.0.is_subset_of(&self.1)
    }
}

impl<T, U> BitTestAll for Difference<T, U>
where
    T: BitTestAll,
    U: BitTestNone,
{
    fn test_all(&self) -> bool {
        self.0.test_all() && self.1.test_none()
    }
}

impl<T, U> BitSetLimit for Difference<T, U>
where
    T: BitSetLimit,
    U: BitUnsetLimit,
{
    const MAX_SET_INDEX: usize = crate::min(T::MAX_SET_INDEX, U::MAX_UNSET_INDEX);
}

impl<T, U> BitSet for Difference<T, U>
where
    T: BitSet,
    U: BitUnset,
{
    unsafe fn set_unchecked(&mut self, idx: usize) {
        self.0.set_unchecked(idx);
        self.1.unset_unchecked(idx);
    }
}

impl<T, U> BitUnsetLimit for Difference<T, U>
where
    T: BitUnsetLimit,
    U: BitSetLimit,
{
    const MAX_UNSET_INDEX: usize = crate::max(T::MAX_UNSET_INDEX, U::MAX_SET_INDEX);
}

impl<T, U> BitUnset for Difference<T, U>
where
    T: BitUnset,
    U: BitSet,
{
    unsafe fn unset_unchecked(&mut self, idx: usize) {
        if idx <= T::MAX_UNSET_INDEX {
            self.0.unset_unchecked(idx);
        } else {
            self.1.set_unchecked(idx);
        }
    }
}

impl<T, U> BitSearch for Difference<T, U>
where
    T: BitSearch,
    U: BitTest,
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

impl<T, U> BitComplement for Difference<T, U>
where
    T: BitComplement,
{
    type Output = Union<T::Output, U>;

    fn complement(self) -> Self::Output {
        Union(self.0.complement(), self.1)
    }
}

impl<T, U, Y> BitUnion<Y> for Difference<T, U> {
    type Output = Union<Difference<T, U>, Y>;

    fn union(self, rhs: Y) -> Self::Output {
        Union(self, rhs)
    }
}

impl<T, U, Y> BitIntersection<Y> for Difference<T, U>
where
    T: BitIntersection<Y>,
{
    type Output = Difference<T::Output, U>;

    fn intersection(self, rhs: Y) -> Self::Output {
        Difference(self.0.intersection(rhs), self.1)
    }
}

impl<T, U, Y> BitDifference<Y> for Difference<T, U>
where
    T: BitDifference<Y>,
{
    type Output = Difference<T::Output, U>;

    fn difference(self, rhs: Y) -> Self::Output {
        Difference(self.0.difference(rhs), self.1)
    }
}
