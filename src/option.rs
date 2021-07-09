use crate::ops::*;

impl<T> BitEmpty for Option<T> {
    fn empty() -> Self {
        None
    }
}

impl<T> BitFull for Option<T>
where
    T: BitFull,
{
    fn full() -> Self {
        Some(T::full())
    }
}

impl<T> BitTest for Option<T>
where
    T: BitTest,
{
    fn test(&self, idx: usize) -> bool {
        match self {
            None => false,
            Some(bits) => bits.test(idx),
        }
    }
}

impl<T> BitTestNone for Option<T>
where
    T: BitTestNone,
{
    fn test_none(&self) -> bool {
        match self {
            None => true,
            Some(bits) => bits.test_none(),
        }
    }
}

impl<T> BitTestAll for Option<T>
where
    T: BitTestAll,
{
    fn test_all(&self) -> bool {
        match self {
            None => false,
            Some(bits) => bits.test_all(),
        }
    }
}

impl<T> BitSetLimit for Option<T>
where
    T: BitSetLimit,
{
    const MAX_SET_INDEX: usize = T::MAX_SET_INDEX;
}

impl<T> BitSet for Option<T>
where
    T: BitSet + BitEmpty,
{
    unsafe fn set_unchecked(&mut self, idx: usize) {
        self.get_or_insert_with(T::empty).set_unchecked(idx)
    }
}

impl<T> BitUnsetLimit for Option<T>
where
    T: BitUnsetLimit,
{
    const MAX_UNSET_INDEX: usize = T::MAX_UNSET_INDEX;
}

impl<T> BitUnset for Option<T>
where
    T: BitUnset + BitTestNone,
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

impl<T> BitSearch for Option<T>
where
    T: BitSearch,
{
    fn find_first_set(&self, lower_bound: usize) -> Option<usize> {
        match self {
            None => None,
            Some(bits) => bits.find_first_set(lower_bound),
        }
    }
}

impl<T> BitComplement for Option<T>
where
    T: BitComplement,
    T::Output: BitFull,
{
    type Output = Option<T::Output>;

    fn complement(self) -> Self::Output {
        match self {
            None => Some(BitFull::full()),
            Some(bits) => Some(bits.complement()),
        }
    }
}

impl<T, U> BitUnion<U> for Option<T>
where
    T: BitUnion<U>,
    U: Into<T::Output>,
{
    type Output = T::Output;

    fn union(self, rhs: U) -> T::Output {
        match self {
            None => rhs.into(),
            Some(lhs) => lhs.union(rhs),
        }
    }
}

impl<T, U> BitIntersection<U> for Option<T>
where
    T: BitIntersection<U>,
{
    type Output = Option<T::Output>;

    fn intersection(self, rhs: U) -> Option<T::Output> {
        match self {
            None => None,
            Some(lhs) => Some(lhs.intersection(rhs)),
        }
    }
}

impl<T, U> BitDifference<U> for Option<T>
where
    T: BitDifference<U>,
{
    type Output = Option<T::Output>;

    fn difference(self, rhs: U) -> Option<T::Output> {
        match self {
            None => None,
            Some(lhs) => Some(lhs.difference(rhs)),
        }
    }
}

impl<T, U> BitSubset<U> for Option<T>
where
    T: BitSubset<U>,
{
    fn is_subset_of(&self, rhs: &U) -> bool {
        match self {
            None => true,
            Some(lhs) => lhs.is_subset_of(rhs),
        }
    }
}

impl<T, U> BitDisjoint<U> for Option<T>
where
    T: BitDisjoint<U>,
{
    fn is_disjoint(&self, rhs: &U) -> bool {
        match self {
            None => true,
            Some(lhs) => lhs.is_disjoint(rhs),
        }
    }
}
