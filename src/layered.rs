use crate::{
    comp::Comp, diff::Diff, intersect::Intersect, ops::*, sym_diff::SymDiff, union::Union,
};

/// Layered bitset.
#[derive(Clone, Copy, Debug)]
pub struct Layered<A, B, const N: usize> {
    top: A,
    bottom: [B; N],
}

impl<A, B, const N: usize> Bits for Layered<A, B, N>
where
    A: Bits,
    B: Bits,
{
    const MAX_SET_INDEX: usize =
        crate::max(A::MAX_SET_INDEX, N - 1) * (B::MAX_SET_INDEX + 1) + B::MAX_SET_INDEX;

    const MAX_UNSET_INDEX: usize = usize::MAX;

    fn test(&self, idx: usize) -> bool {
        if idx >= Self::MAX_SET_INDEX {
            false
        } else {
            let t = idx / (B::MAX_SET_INDEX + 1);
            let b = idx % (B::MAX_SET_INDEX + 1);

            self.bottom[t as usize].test(b)
        }
    }
}

impl<A, B, const N: usize> BitEmpty for Layered<A, B, N>
where
    A: BitEmpty,
    B: BitEmpty,
{
    fn empty() -> Self {
        Layered {
            top: A::empty(),
            bottom: crate::make_array(B::empty),
        }
    }
}

impl<A, B, const N: usize> BitFind for Layered<A, B, N>
where
    A: BitFind,
    B: BitFind,
{
    fn find_first_set(&self, lower_bound: usize) -> Option<usize> {
        if lower_bound >= Self::MAX_SET_INDEX {
            None
        } else {
            let t = lower_bound / (B::MAX_SET_INDEX + 1);
            let b = lower_bound % (B::MAX_SET_INDEX + 1);

            if b == 0 {
                let t = self.top.find_first_set(t)?;
                let b = self.bottom[t as usize].find_first_set(0)?;
                Some(t * (B::MAX_SET_INDEX + 1) + b)
            } else {
                let mut t_set = self.top.find_first_set(t)?;
                if t == t_set {
                    if let Some(b) = self.bottom[t as usize].find_first_set(b) {
                        return Some(t * (B::MAX_SET_INDEX + 1) + b);
                    }
                    t_set = self.top.find_first_set(t + 1)?;
                }

                let b = self.bottom[t_set as usize].find_first_set(0)?;
                Some(t_set * (B::MAX_SET_INDEX + 1) + b)
            }
        }
    }
}

impl<A, B, const N: usize> BitSet for Layered<A, B, N>
where
    A: BitSet,
    B: BitSet,
{
    unsafe fn set_unchecked(&mut self, idx: usize) {
        let t = idx / (B::MAX_SET_INDEX + 1);
        let u = idx % (B::MAX_SET_INDEX + 1);

        self.top.set_unchecked(t);
        self.bottom[t as usize].set_unchecked(u)
    }
}

impl<A, B, const N: usize> BitUnset for Layered<A, B, N>
where
    A: BitUnset,
    B: BitUnset + BitTestNone,
{
    unsafe fn unset_unchecked(&mut self, idx: usize) {
        if A::MAX_UNSET_INDEX < N || B::MAX_UNSET_INDEX < B::MAX_SET_INDEX {
            panic!("This kind of layered bitset cannot support bit unsetting");
        }

        let t = idx / (B::MAX_SET_INDEX + 1);
        let u = idx % (B::MAX_SET_INDEX + 1);

        self.bottom[t as usize].unset_unchecked(u);
        if !self.bottom[t as usize].test_none() {
            self.top.unset_unchecked(t);
        }
    }
}

impl<A, B, const N: usize> BitComp for Layered<A, B, N>
where
    A: Bits,
    B: Bits,
{
    type Output = Comp<Self>;

    fn comp(self) -> Comp<Self> {
        Comp(self)
    }
}

impl<A, B, C, const N: usize> BitUnion<C> for Layered<A, B, N>
where
    A: Bits,
    B: Bits,
    C: Bits,
{
    type Output = Union<Self, C>;

    fn union(self, rhs: C) -> Union<Self, C> {
        Union(self, rhs)
    }
}

impl<A, B, C, const N: usize> BitIntersect<C> for Layered<A, B, N>
where
    A: Bits,
    B: Bits,
    C: Bits,
{
    type Output = Intersect<Self, C>;

    fn intersect(self, rhs: C) -> Intersect<Self, C> {
        Intersect(self, rhs)
    }
}

impl<A, B, C, const N: usize> BitDiff<C> for Layered<A, B, N>
where
    A: Bits,
    B: Bits,
    C: Bits,
{
    type Output = Diff<Self, C>;

    fn diff(self, rhs: C) -> Diff<Self, C> {
        Diff(self, rhs)
    }
}

impl<A, B, C, const N: usize> BitSymDiff<C> for Layered<A, B, N>
where
    A: Bits,
    B: Bits,
    C: Bits,
{
    type Output = SymDiff<Self, C>;

    fn sym_diff(self, rhs: C) -> SymDiff<Self, C> {
        SymDiff(self, rhs)
    }
}
