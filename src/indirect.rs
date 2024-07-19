use crate::{
    comp::Comp, diff::Diff, intersect::Intersect, ops::*, sym_diff::SymDiff, union::Union,
};

impl<A> Bits for &'_ A
where
    A: Bits,
{
    const MAX_SET_INDEX: usize = A::MAX_SET_INDEX;
    const MAX_UNSET_INDEX: usize = A::MAX_UNSET_INDEX;

    fn test(&self, idx: usize) -> bool {
        A::test(*self, idx)
    }
}

impl<A> BitTestNone for &'_ A
where
    A: BitTestNone,
{
    fn test_none(&self) -> bool {
        A::test_none(*self)
    }
}

impl<A> BitTestAll for &'_ A
where
    A: BitTestAll,
{
    fn test_all(&self) -> bool {
        A::test_all(*self)
    }
}

impl<A> BitFind for &'_ A
where
    A: BitFind,
{
    fn find_first_set(&self, lower_bound: usize) -> Option<usize> {
        A::find_first_set(*self, lower_bound)
    }
}

impl<A> BitComp for &'_ A
where
    A: Bits,
{
    type Output = Comp<Self>;

    fn comp(self) -> Comp<Self> {
        Comp(self)
    }
}

impl<A, B> BitUnion<B> for &'_ A
where
    A: Bits,
    B: Bits,
{
    type Output = Union<Self, B>;

    fn union(self, rhs: B) -> Union<Self, B> {
        Union(self, rhs)
    }
}

impl<A, B> BitIntersect<B> for &'_ A
where
    A: Bits,
    B: Bits,
{
    type Output = Intersect<Self, B>;

    fn intersect(self, rhs: B) -> Intersect<Self, B> {
        Intersect(self, rhs)
    }
}

impl<A, B> BitDiff<B> for &'_ A
where
    A: Bits,
    B: Bits,
{
    type Output = Diff<Self, B>;

    fn diff(self, rhs: B) -> Diff<Self, B> {
        Diff(self, rhs)
    }
}

impl<A, B> BitSymDiff<B> for &'_ A
where
    A: Bits,
    B: Bits,
{
    type Output = SymDiff<Self, B>;

    fn sym_diff(self, rhs: B) -> SymDiff<Self, B> {
        SymDiff(self, rhs)
    }
}

impl<A, B> BitSubset<B> for &'_ A
where
    A: BitSubset<B>,
    B: Bits,
{
    fn is_subset_of(&self, rhs: &B) -> bool {
        A::is_subset_of(*self, rhs)
    }
}

impl<A, B> BitDisjoint<B> for &'_ A
where
    A: BitDisjoint<B>,
    B: Bits,
{
    fn is_disjoint(&self, rhs: &B) -> bool {
        A::is_disjoint(*self, rhs)
    }
}

impl<A> Bits for &'_ mut A
where
    A: Bits,
{
    const MAX_SET_INDEX: usize = A::MAX_SET_INDEX;
    const MAX_UNSET_INDEX: usize = A::MAX_UNSET_INDEX;

    fn test(&self, idx: usize) -> bool {
        A::test(*self, idx)
    }
}

impl<A> BitTestNone for &'_ mut A
where
    A: BitTestNone,
{
    fn test_none(&self) -> bool {
        A::test_none(*self)
    }
}

impl<A> BitTestAll for &'_ mut A
where
    A: BitTestAll,
{
    fn test_all(&self) -> bool {
        A::test_all(*self)
    }
}

impl<A> BitSet for &'_ mut A
where
    A: BitSet,
{
    unsafe fn set_unchecked(&mut self, idx: usize) {
        A::set_unchecked(*self, idx)
    }
}

impl<A> BitUnset for &'_ mut A
where
    A: BitUnset,
{
    unsafe fn unset_unchecked(&mut self, idx: usize) {
        A::unset_unchecked(*self, idx)
    }
}

impl<A> BitFind for &'_ mut A
where
    A: BitFind,
{
    fn find_first_set(&self, lower_bound: usize) -> Option<usize> {
        A::find_first_set(*self, lower_bound)
    }
}

impl<A> BitComp for &'_ mut A
where
    A: Bits,
{
    type Output = Comp<Self>;

    fn comp(self) -> Comp<Self> {
        Comp(self)
    }
}

impl<A, B> BitUnion<B> for &'_ mut A
where
    A: Bits,
    B: Bits,
{
    type Output = Union<Self, B>;

    fn union(self, rhs: B) -> Union<Self, B> {
        Union(self, rhs)
    }
}

impl<A, B> BitIntersect<B> for &'_ mut A
where
    A: Bits,
    B: Bits,
{
    type Output = Intersect<Self, B>;

    fn intersect(self, rhs: B) -> Intersect<Self, B> {
        Intersect(self, rhs)
    }
}

impl<A, B> BitDiff<B> for &'_ mut A
where
    A: Bits,
    B: Bits,
{
    type Output = Diff<Self, B>;

    fn diff(self, rhs: B) -> Diff<Self, B> {
        Diff(self, rhs)
    }
}

impl<A, B> BitSymDiff<B> for &'_ mut A
where
    A: Bits,
    B: Bits,
{
    type Output = SymDiff<Self, B>;

    fn sym_diff(self, rhs: B) -> SymDiff<Self, B> {
        SymDiff(self, rhs)
    }
}

impl<A, B> BitSubset<B> for &'_ mut A
where
    A: BitSubset<B>,
    B: Bits,
{
    fn is_subset_of(&self, rhs: &B) -> bool {
        A::is_subset_of(*self, rhs)
    }
}

impl<A, B> BitDisjoint<B> for &'_ mut A
where
    A: BitDisjoint<B>,
    B: Bits,
{
    fn is_disjoint(&self, rhs: &B) -> bool {
        A::is_disjoint(*self, rhs)
    }
}

#[cfg(feature = "alloc")]
mod boxed {

    use alloc::boxed::Box;

    use super::*;

    impl<A> Bits for Box<A>
    where
        A: Bits,
    {
        const MAX_SET_INDEX: usize = A::MAX_SET_INDEX;
        const MAX_UNSET_INDEX: usize = A::MAX_UNSET_INDEX;

        fn test(&self, idx: usize) -> bool {
            A::test(self, idx)
        }
    }

    impl<A> BitEmpty for Box<A>
    where
        A: BitEmpty,
    {
        fn empty() -> Self {
            Box::new(A::empty())
        }
    }

    impl<A> BitFull for Box<A>
    where
        A: BitFull,
    {
        fn full() -> Self {
            Box::new(A::full())
        }
    }

    impl<A> BitTestNone for Box<A>
    where
        A: BitTestNone,
    {
        fn test_none(&self) -> bool {
            A::test_none(&**self)
        }
    }

    impl<A> BitTestAll for Box<A>
    where
        A: BitTestAll,
    {
        fn test_all(&self) -> bool {
            A::test_all(&**self)
        }
    }

    impl<A> BitSet for Box<A>
    where
        A: BitSet,
    {
        unsafe fn set_unchecked(&mut self, idx: usize) {
            A::set_unchecked(&mut **self, idx)
        }
    }

    impl<A> BitUnset for Box<A>
    where
        A: BitUnset,
    {
        unsafe fn unset_unchecked(&mut self, idx: usize) {
            A::unset_unchecked(&mut **self, idx)
        }
    }

    impl<A> BitFind for Box<A>
    where
        A: BitFind,
    {
        fn find_first_set(&self, lower_bound: usize) -> Option<usize> {
            A::find_first_set(&**self, lower_bound)
        }
    }

    impl<A> BitComp for Box<A>
    where
        A: Bits,
    {
        type Output = Comp<Self>;

        fn comp(self) -> Comp<Self> {
            Comp(self)
        }
    }

    impl<A, B> BitUnion<B> for Box<A>
    where
        A: Bits,
        B: Bits,
    {
        type Output = Union<Self, B>;

        fn union(self, rhs: B) -> Union<Self, B> {
            Union(self, rhs)
        }
    }

    impl<A, B> BitIntersect<B> for Box<A>
    where
        A: Bits,
        B: Bits,
    {
        type Output = Intersect<Self, B>;

        fn intersect(self, rhs: B) -> Intersect<Self, B> {
            Intersect(self, rhs)
        }
    }

    impl<A, B> BitDiff<B> for Box<A>
    where
        A: Bits,
        B: Bits,
    {
        type Output = Diff<Self, B>;

        fn diff(self, rhs: B) -> Diff<Self, B> {
            Diff(self, rhs)
        }
    }

    impl<A, B> BitSymDiff<B> for Box<A>
    where
        A: Bits,
        B: Bits,
    {
        type Output = SymDiff<Self, B>;

        fn sym_diff(self, rhs: B) -> SymDiff<Self, B> {
            SymDiff(self, rhs)
        }
    }

    impl<A, B> BitSubset<B> for Box<A>
    where
        A: BitSubset<B>,
        B: Bits,
    {
        fn is_subset_of(&self, rhs: &B) -> bool {
            A::is_subset_of(&**self, rhs)
        }
    }

    impl<A, B> BitDisjoint<B> for Box<A>
    where
        A: BitDisjoint<B>,
        B: Bits,
    {
        fn is_disjoint(&self, rhs: &B) -> bool {
            A::is_disjoint(&**self, rhs)
        }
    }
}
