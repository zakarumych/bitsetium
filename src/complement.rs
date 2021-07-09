use {
    crate::ops::*,
    core::fmt::{self, Display},
};

/// Bit-set wrapper that acts like set complement.
///
/// Effectively inverses all bits in the underlying bit-set.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Complement<T>(pub T);

impl<T> Complement<T> {
    pub fn inner(&self) -> &T {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.0
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Complement<Complement<T>> {
    /// Convenience function to unwrap double `Complement` wrapper which must yield bitset with same bits set.
    pub fn double_complement_unwrap(self) -> T {
        self.0 .0
    }
}

impl<T> Display for Complement<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Inverse({})", self.0)
    }
}

impl<T> BitEmpty for Complement<T>
where
    T: BitFull,
{
    fn empty() -> Self {
        Complement(T::full())
    }
}

impl<T> BitFull for Complement<T>
where
    T: BitEmpty,
{
    fn full() -> Self {
        Complement(T::empty())
    }
}

impl<T> BitTest for Complement<T>
where
    T: BitTest,
{
    fn test(&self, idx: usize) -> bool {
        !self.0.test(idx)
    }
}

impl<T> BitTestNone for Complement<T>
where
    T: BitTestAll,
{
    fn test_none(&self) -> bool {
        self.0.test_all()
    }
}

impl<T> BitTestAll for Complement<T>
where
    T: BitTestNone,
{
    fn test_all(&self) -> bool {
        self.0.test_none()
    }
}

impl<T> BitSetLimit for Complement<T>
where
    T: BitUnsetLimit,
{
    const MAX_SET_INDEX: usize = T::MAX_UNSET_INDEX;
}

impl<T> BitSet for Complement<T>
where
    T: BitUnset,
{
    unsafe fn set_unchecked(&mut self, idx: usize) {
        self.0.unset_unchecked(idx)
    }
}

impl<T> BitUnsetLimit for Complement<T>
where
    T: BitSetLimit,
{
    const MAX_UNSET_INDEX: usize = T::MAX_SET_INDEX;
}

impl<T> BitUnset for Complement<T>
where
    T: BitSet,
{
    unsafe fn unset_unchecked(&mut self, idx: usize) {
        self.0.set_unchecked(idx)
    }
}

impl<T> BitSearch for Complement<Complement<T>>
where
    T: BitSearch,
{
    fn find_first_set(&self, lower_bound: usize) -> Option<usize> {
        self.inner().inner().find_first_set(lower_bound)
    }
}

impl<T> BitComplement for Complement<T> {
    type Output = T;

    fn complement(self) -> T {
        self.0
    }
}

impl<T, U> BitUnion<U> for Complement<T>
where
    T: BitDifference<U>,
{
    type Output = Complement<<T as BitDifference<U>>::Output>;

    fn union(self, rhs: U) -> Self::Output {
        Complement(self.0.difference(rhs))
    }
}

impl<T, U> BitIntersection<U> for Complement<T>
where
    U: BitDifference<T>,
{
    type Output = <U as BitDifference<T>>::Output;

    fn intersection(self, rhs: U) -> Self::Output {
        rhs.difference(self.0)
    }
}

impl<T, U> BitDifference<U> for Complement<T>
where
    T: BitUnion<U>,
{
    type Output = Complement<<T as BitUnion<U>>::Output>;

    fn difference(self, rhs: U) -> Self::Output {
        Complement(self.0.union(rhs))
    }
}

impl<T, U> BitDisjoint<U> for Complement<T>
where
    U: BitSubset<T>,
{
    fn is_disjoint(&self, rhs: &U) -> bool {
        rhs.is_subset_of(&self.0)
    }
}
