use crate::{
    complement::Complement, difference::Difference, intersection::Intersection, ops::*,
    union::Union,
};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

impl<T> BitTest for &'_ T
where
    T: BitTest,
{
    fn test(&self, idx: usize) -> bool {
        T::test(*self, idx)
    }
}

impl<T> BitTestNone for &'_ T
where
    T: BitTestNone,
{
    fn test_none(&self) -> bool {
        T::test_none(*self)
    }
}

impl<T> BitTestAll for &'_ T
where
    T: BitTestAll,
{
    fn test_all(&self) -> bool {
        T::test_all(*self)
    }
}

impl<T> BitSetLimit for &'_ T
where
    T: BitSetLimit,
{
    const MAX_SET_INDEX: usize = T::MAX_SET_INDEX;
}

impl<T> BitUnsetLimit for &'_ T
where
    T: BitUnsetLimit,
{
    const MAX_UNSET_INDEX: usize = T::MAX_UNSET_INDEX;
}

impl<T> BitSearch for &'_ T
where
    T: BitSearch,
{
    fn find_first_set(&self, lower_bound: usize) -> Option<usize> {
        T::find_first_set(*self, lower_bound)
    }
}

impl<T> BitComplement for &'_ T {
    type Output = Complement<Self>;

    fn complement(self) -> Complement<Self> {
        Complement(self)
    }
}

impl<T, U> BitUnion<U> for &'_ T {
    type Output = Union<Self, U>;

    fn union(self, rhs: U) -> Union<Self, U> {
        Union(self, rhs)
    }
}

impl<T, U> BitIntersection<U> for &'_ T {
    type Output = Intersection<Self, U>;

    fn intersection(self, rhs: U) -> Intersection<Self, U> {
        Intersection(self, rhs)
    }
}

impl<T, U> BitDifference<U> for &'_ T {
    type Output = Difference<Self, U>;

    fn difference(self, rhs: U) -> Difference<Self, U> {
        Difference(self, rhs)
    }
}

impl<T, U> BitSubset<U> for &'_ T
where
    T: BitSubset<U>,
{
    fn is_subset_of(&self, rhs: &U) -> bool {
        T::is_subset_of(*self, rhs)
    }
}

impl<T, U> BitDisjoint<U> for &'_ T
where
    T: BitDisjoint<U>,
{
    fn is_disjoint(&self, rhs: &U) -> bool {
        T::is_disjoint(*self, rhs)
    }
}

impl<T> BitTest for &'_ mut T
where
    T: BitTest,
{
    fn test(&self, idx: usize) -> bool {
        T::test(*self, idx)
    }
}

impl<T> BitTestNone for &'_ mut T
where
    T: BitTestNone,
{
    fn test_none(&self) -> bool {
        T::test_none(*self)
    }
}

impl<T> BitTestAll for &'_ mut T
where
    T: BitTestAll,
{
    fn test_all(&self) -> bool {
        T::test_all(*self)
    }
}

impl<T> BitSetLimit for &'_ mut T
where
    T: BitSetLimit,
{
    const MAX_SET_INDEX: usize = T::MAX_SET_INDEX;
}

impl<T> BitSet for &'_ mut T
where
    T: BitSet,
{
    unsafe fn set_unchecked(&mut self, idx: usize) {
        T::set_unchecked(*self, idx)
    }
}

impl<T> BitUnsetLimit for &'_ mut T
where
    T: BitUnsetLimit,
{
    const MAX_UNSET_INDEX: usize = T::MAX_UNSET_INDEX;
}

impl<T> BitUnset for &'_ mut T
where
    T: BitUnset,
{
    unsafe fn unset_unchecked(&mut self, idx: usize) {
        T::unset_unchecked(*self, idx)
    }
}

impl<T> BitSearch for &'_ mut T
where
    T: BitSearch,
{
    fn find_first_set(&self, lower_bound: usize) -> Option<usize> {
        T::find_first_set(*self, lower_bound)
    }
}

impl<T> BitComplement for &'_ mut T {
    type Output = Complement<Self>;

    fn complement(self) -> Complement<Self> {
        Complement(self)
    }
}

impl<T, U> BitUnion<U> for &'_ mut T {
    type Output = Union<Self, U>;

    fn union(self, rhs: U) -> Union<Self, U> {
        Union(self, rhs)
    }
}

impl<T, U> BitIntersection<U> for &'_ mut T {
    type Output = Intersection<Self, U>;

    fn intersection(self, rhs: U) -> Intersection<Self, U> {
        Intersection(self, rhs)
    }
}

impl<T, U> BitDifference<U> for &'_ mut T {
    type Output = Difference<Self, U>;

    fn difference(self, rhs: U) -> Difference<Self, U> {
        Difference(self, rhs)
    }
}

impl<T, U> BitSubset<U> for &'_ mut T
where
    T: BitSubset<U>,
{
    fn is_subset_of(&self, rhs: &U) -> bool {
        T::is_subset_of(*self, rhs)
    }
}

impl<T, U> BitDisjoint<U> for &'_ mut T
where
    T: BitDisjoint<U>,
{
    fn is_disjoint(&self, rhs: &U) -> bool {
        T::is_disjoint(*self, rhs)
    }
}

#[cfg(feature = "alloc")]
impl<T> BitEmpty for Box<T>
where
    T: BitEmpty,
{
    fn empty() -> Self {
        Box::new(T::empty())
    }
}

#[cfg(feature = "alloc")]
impl<T> BitFull for Box<T>
where
    T: BitFull,
{
    fn full() -> Self {
        Box::new(T::full())
    }
}

#[cfg(feature = "alloc")]
impl<T> BitTest for Box<T>
where
    T: BitTest,
{
    fn test(&self, idx: usize) -> bool {
        T::test(&**self, idx)
    }
}

#[cfg(feature = "alloc")]
impl<T> BitTestNone for Box<T>
where
    T: BitTestNone,
{
    fn test_none(&self) -> bool {
        T::test_none(&**self)
    }
}

#[cfg(feature = "alloc")]
impl<T> BitTestAll for Box<T>
where
    T: BitTestAll,
{
    fn test_all(&self) -> bool {
        T::test_all(&**self)
    }
}

#[cfg(feature = "alloc")]
impl<T> BitSetLimit for Box<T>
where
    T: BitSetLimit,
{
    const MAX_SET_INDEX: usize = T::MAX_SET_INDEX;
}

#[cfg(feature = "alloc")]
impl<T> BitSet for Box<T>
where
    T: BitSet,
{
    unsafe fn set_unchecked(&mut self, idx: usize) {
        T::set_unchecked(&mut **self, idx)
    }
}

#[cfg(feature = "alloc")]
impl<T> BitUnsetLimit for Box<T>
where
    T: BitUnsetLimit,
{
    const MAX_UNSET_INDEX: usize = T::MAX_UNSET_INDEX;
}

#[cfg(feature = "alloc")]
impl<T> BitUnset for Box<T>
where
    T: BitUnset,
{
    unsafe fn unset_unchecked(&mut self, idx: usize) {
        T::unset_unchecked(&mut **self, idx)
    }
}

#[cfg(feature = "alloc")]
impl<T> BitSearch for Box<T>
where
    T: BitSearch,
{
    fn find_first_set(&self, lower_bound: usize) -> Option<usize> {
        T::find_first_set(&**self, lower_bound)
    }
}

#[cfg(feature = "alloc")]
impl<T> BitComplement for Box<T> {
    type Output = Complement<Self>;

    fn complement(self) -> Complement<Self> {
        Complement(self)
    }
}

#[cfg(feature = "alloc")]
impl<T, U> BitUnion<U> for Box<T> {
    type Output = Union<Self, U>;

    fn union(self, rhs: U) -> Union<Self, U> {
        Union(self, rhs)
    }
}

#[cfg(feature = "alloc")]
impl<T, U> BitIntersection<U> for Box<T> {
    type Output = Intersection<Self, U>;

    fn intersection(self, rhs: U) -> Intersection<Self, U> {
        Intersection(self, rhs)
    }
}

#[cfg(feature = "alloc")]
impl<T, U> BitDifference<U> for Box<T> {
    type Output = Difference<Self, U>;

    fn difference(self, rhs: U) -> Difference<Self, U> {
        Difference(self, rhs)
    }
}

#[cfg(feature = "alloc")]
impl<T, U> BitSubset<U> for Box<T>
where
    T: BitSubset<U>,
{
    fn is_subset_of(&self, rhs: &U) -> bool {
        T::is_subset_of(&**self, rhs)
    }
}

#[cfg(feature = "alloc")]
impl<T, U> BitDisjoint<U> for Box<T>
where
    T: BitDisjoint<U>,
{
    fn is_disjoint(&self, rhs: &U) -> bool {
        T::is_disjoint(&**self, rhs)
    }
}
