use crate::{
    complement::Complement, difference::Difference, intersection::Intersection, ops::*,
    union::Union,
};

#[derive(Clone, Copy, Debug)]
pub struct Layered<T, B, const N: usize> {
    top: T,
    bottom: [B; N],
}

impl<T, B, const N: usize> BitEmpty for Layered<T, B, N>
where
    T: BitEmpty,
    B: BitEmpty,
{
    fn empty() -> Self {
        Layered {
            top: T::empty(),
            bottom: crate::make_array(B::empty),
        }
    }
}

impl<T, B, const N: usize> BitTest for Layered<T, B, N>
where
    T: BitTest + BitSetLimit,
    B: BitTest + BitSetLimit,
{
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

impl<T, B, const N: usize> BitSearch for Layered<T, B, N>
where
    T: BitSearch + BitSetLimit,
    B: BitSearch + BitSetLimit,
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

impl<T, B, const N: usize> BitSetLimit for Layered<T, B, N>
where
    T: BitSetLimit,
    B: BitSetLimit,
{
    const MAX_SET_INDEX: usize =
        crate::max(T::MAX_SET_INDEX, N - 1) * (B::MAX_SET_INDEX + 1) + B::MAX_SET_INDEX;
}

impl<T, B, const N: usize> BitSet for Layered<T, B, N>
where
    T: BitSet,
    B: BitSet,
{
    unsafe fn set_unchecked(&mut self, idx: usize) {
        let t = idx / (B::MAX_SET_INDEX + 1);
        let u = idx % (B::MAX_SET_INDEX + 1);

        self.top.set_unchecked(t);
        self.bottom[t as usize].set_unchecked(u)
    }
}

impl<T, B, const N: usize> BitUnsetLimit for Layered<T, B, N> {
    const MAX_UNSET_INDEX: usize = usize::MAX;
}

impl<T, B, const N: usize> BitUnset for Layered<T, B, N>
where
    T: BitUnset,
    B: BitUnset + BitSetLimit + BitTestNone,
{
    unsafe fn unset_unchecked(&mut self, idx: usize) {
        if T::MAX_UNSET_INDEX < N || B::MAX_UNSET_INDEX < B::MAX_SET_INDEX {
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

impl<T, B, const N: usize> BitComplement for Layered<T, B, N> {
    type Output = Complement<Self>;

    fn complement(self) -> Complement<Self> {
        Complement(self)
    }
}

impl<T, B, U, const N: usize> BitUnion<U> for Layered<T, B, N> {
    type Output = Union<Self, U>;

    fn union(self, rhs: U) -> Union<Self, U> {
        Union(self, rhs)
    }
}

impl<T, B, U, const N: usize> BitIntersection<U> for Layered<T, B, N> {
    type Output = Intersection<Self, U>;

    fn intersection(self, rhs: U) -> Intersection<Self, U> {
        Intersection(self, rhs)
    }
}

impl<T, B, U, const N: usize> BitDifference<U> for Layered<T, B, N> {
    type Output = Difference<Self, U>;

    fn difference(self, rhs: U) -> Difference<Self, U> {
        Difference(self, rhs)
    }
}
