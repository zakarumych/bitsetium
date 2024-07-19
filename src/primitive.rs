use crate::{comp::Comp, ops::*};

macro_rules! impl_for_primitive {
    ($ty:ty : $size:literal) => {

        impl BitEmpty for $ty {
            fn empty() -> $ty {
                <$ty>::MIN
            }
        }

        impl Bits for $ty {
            const MAX_SET_INDEX: usize = $size - 1;
            const MAX_UNSET_INDEX: usize = usize::MAX;

            #[inline]
            fn test(&self, idx: usize) -> bool {
                if idx < $size {
                    0 != *self & ((1 as $ty) << idx)
                } else {
                    false
                }
            }
        }

        impl BitTestAll for $ty {
            #[inline]
            fn test_all(&self) -> bool {
                $size == usize::MAX && *self == <$ty>::MAX
            }
        }

        impl BitTestNone for $ty {
            #[inline]
            fn test_none(&self) -> bool {
                *self == 0
            }
        }

        impl BitSet for $ty {
            #[inline]
            unsafe fn set_unchecked(&mut self, idx: usize) {
                debug_assert!(idx < $size);
                *self |= (1 as $ty << idx);
            }
        }

        impl BitUnset for $ty {
            #[inline]
            unsafe fn unset_unchecked(&mut self, idx: usize) {
                if idx < $size {
                    *self &= !(1 as $ty << idx);
                }
            }
        }

        impl BitFind for $ty {
            fn find_first_set(&self, lower_bound: usize) -> Option<usize> {
                if lower_bound > Self::MAX_SET_INDEX {
                    return None;
                }

                let masked = *self & (<$ty>::MAX).wrapping_shl((lower_bound as u32));
                match masked.trailing_zeros() {
                    $size => None,
                    idx => Some(idx as usize),
                }
            }
        }

        impl BitComp for $ty {
            type Output = $ty;

            fn comp(self) -> Self {
                !self
            }
        }

        impl BitUnion for $ty {
            type Output = Self;

            fn union(self, rhs: Self) -> Self {
                self | rhs
            }
        }

        impl BitUnion<Comp<$ty>> for $ty {
            type Output = Comp<Self>;

            fn union(self, rhs: Comp<Self>) -> Comp<Self> {
                Comp(rhs.0 & (!self))
            }
        }

        impl BitIntersect for $ty {
            type Output = Self;

            fn intersect(self, rhs: Self) -> Self {
                self & rhs
            }
        }

        impl BitIntersect<Comp<$ty>> for $ty {
            type Output = Self;

            fn intersect(self, rhs: Comp<Self>) -> Self {
                self & (!rhs.0)
            }
        }

        impl BitDiff for $ty {
            type Output = Self;

            fn diff(self, rhs: Self) -> Self {
                self & (!rhs)
            }
        }

        impl BitSymDiff for $ty {
            type Output = Self;

            fn sym_diff(self, rhs: Self) -> Self {
                self ^ rhs
            }
        }

        impl BitSubset for $ty {
            fn is_subset_of(&self, rhs: &Self) -> bool {
                *self & !*rhs == 0
            }
        }

        impl BitSubset<Comp<$ty>> for $ty {
            fn is_subset_of(&self, rhs: &Comp<Self>) -> bool {
                *self & rhs.0 == 0
            }
        }

        impl BitDisjoint for $ty {
            fn is_disjoint(&self, rhs: &Self) -> bool {
                *self & *rhs == 0
            }
        }

        impl BitDisjoint<Comp<$ty>> for $ty {
            fn is_disjoint(&self, rhs: &Comp<Self>) -> bool {
                *self & !rhs.0 == 0
            }
        }


        impl<const N: usize> BitEmpty for [$ty; N] {
            fn empty() -> [$ty; N] {
                [<$ty>::MIN; N]
            }
        }

        impl<const N: usize> Bits for [$ty; N] {
            const MAX_SET_INDEX: usize = ($size * N) - 1;
            const MAX_UNSET_INDEX: usize = usize::MAX;

            #[inline]
            fn test(&self, idx: usize) -> bool {
                if idx < $size * N {
                    let i = idx / $size;
                    let j = idx % $size;
                    0 != self[i] & ((1 as $ty) << j)
                } else {
                    false
                }
            }
        }

        impl<const N: usize> BitTestAll for [$ty; N] {
            #[inline]
            fn test_all(&self) -> bool {
                false
            }
        }

        impl<const N: usize> BitTestNone for [$ty; N] {
            #[inline]
            fn test_none(&self) -> bool {
                self.iter().all(|e| *e == 0)
            }
        }

        impl<const N: usize> BitSet for [$ty; N] {
            #[inline]
            unsafe fn set_unchecked(&mut self, idx: usize) {
                debug_assert!(idx < $size * N);
                let i = idx / $size;
                let j = idx % $size;
                self[i] |= (1 as $ty << j);
            }
        }

        impl<const N: usize> BitUnset for [$ty; N] {
            #[inline]
            unsafe fn unset_unchecked(&mut self, idx: usize) {
                if idx < $size * N {
                    let i = idx / $size;
                    let j = idx % $size;

                    self[i] &= !(1 as $ty << j);
                }
            }
        }

        impl<const N: usize> BitFind for [$ty; N] {
            fn find_first_set(&self, lower_bound: usize) -> Option<usize> {
                if lower_bound > Self::MAX_SET_INDEX {
                    return None;
                }

                let mut i = lower_bound / $size;
                let j = lower_bound % $size;

                let mut masked = self[i] & (<$ty>::MAX).wrapping_shl((j as u32));

                loop {
                    match masked.trailing_zeros() {
                        $size => {
                            i += 1;
                            if i >= N {
                                return None;
                            }
                            masked = self[i];
                        },
                        idx => return Some(i * $size + idx as usize)
                    }
                }
            }
        }

        impl<const N: usize> BitComp for [$ty; N] {
            type Output = Self;

            fn comp(mut self) -> Self {
                for e in &mut self {
                    *e = !*e;
                }
                self
            }
        }

        impl<const N: usize> BitUnion for [$ty; N] {
            type Output = Self;

            fn union(self, rhs: Self) -> Self {
                crate::map2_arrays(self, rhs, |l, r| l | r)
            }
        }

        impl<const N: usize> BitUnion<Comp<[$ty; N]>> for [$ty; N] {
            type Output = Comp<Self>;

            fn union(self, rhs: Comp<Self>) -> Comp<Self> {
                Comp(crate::map2_arrays(self, rhs.0, |l, r| r & !l))
            }
        }

        impl<const N: usize> BitIntersect for [$ty; N] {
            type Output = Self;

            fn intersect(self, rhs: Self) -> Self {
                crate::map2_arrays(self, rhs, |l, r| l & r)
            }
        }

        impl<const N: usize> BitIntersect<Comp<[$ty; N]>> for [$ty; N] {
            type Output = Self;

            fn intersect(self, rhs: Comp<Self>) -> Self {
                crate::map2_arrays(self, rhs.0, |l, r| l & !r)
            }
        }

        impl<const N: usize> BitDiff for [$ty; N] {
            type Output = Self;

            fn diff(self, rhs: Self) -> Self {
                crate::map2_arrays(self, rhs, |l, r| l & !r)
            }
        }

        impl<const N: usize> BitSubset for [$ty; N] {
            fn is_subset_of(&self, rhs: &Self) -> bool {
                self.iter().zip(rhs).all(|(lhs, rhs)| *lhs & !*rhs == 0)
            }
        }

        impl<const N: usize> BitSubset<Comp<[$ty; N]>> for [$ty; N] {
            fn is_subset_of(&self, rhs: &Comp<Self>) -> bool {
                self.iter().zip(&rhs.0).all(|(lhs, rhs)| *lhs & *rhs == 0)
            }
        }

        impl<const N: usize> BitDisjoint for [$ty; N] {
            fn is_disjoint(&self, rhs: &Self) -> bool {
                self.iter().zip(rhs).all(|(lhs, rhs)| *lhs & *rhs == 0)
            }
        }

        impl<const N: usize> BitDisjoint<Comp<[$ty; N]>> for [$ty; N] {
            fn is_disjoint(&self, rhs: &Comp<Self>) -> bool {
                self.iter().zip(&rhs.0).all(|(lhs, rhs)| *lhs & !*rhs == 0)
            }
        }
    };

    ($ty:ty : $size:literal, $($tail:tt)+) => {
        impl_for_primitive!($ty : $size);
        impl_for_primitive!($($tail)+);
    }
}

impl_for_primitive!(u8 : 8, u16 : 16, u32 : 32, u64 : 64, u128 : 128);

impl BitEmpty for bool {
    fn empty() -> bool {
        false
    }
}

impl Bits for bool {
    const MAX_SET_INDEX: usize = 0;
    const MAX_UNSET_INDEX: usize = usize::MAX;

    #[inline]
    fn test(&self, idx: usize) -> bool {
        if idx == 0 {
            *self
        } else {
            false
        }
    }
}

impl BitTestAll for bool {
    #[inline]
    fn test_all(&self) -> bool {
        usize::MAX == 0 && *self
    }
}

impl BitTestNone for bool {
    #[inline]
    fn test_none(&self) -> bool {
        !*self
    }
}

impl BitSet for bool {
    #[inline]
    unsafe fn set_unchecked(&mut self, idx: usize) {
        debug_assert_eq!(idx, 0);
        *self = true;
    }
}

impl BitUnset for bool {
    #[inline]
    unsafe fn unset_unchecked(&mut self, idx: usize) {
        if idx == 0 {
            *self = false;
        }
    }
}

impl BitFind for bool {
    fn find_first_set(&self, lower_bound: usize) -> Option<usize> {
        if lower_bound > 0 {
            return None;
        }

        if *self {
            Some(0)
        } else {
            None
        }
    }
}

impl BitComp for bool {
    type Output = Comp<bool>;

    fn comp(self) -> Comp<Self> {
        Comp(self)
    }
}

impl BitUnion for bool {
    type Output = Self;

    fn union(self, rhs: Self) -> Self {
        self || rhs
    }
}

impl BitUnion<Comp<bool>> for bool {
    type Output = Comp<Self>;

    fn union(self, rhs: Comp<Self>) -> Comp<Self> {
        Comp(rhs.0 && (!self))
    }
}

impl BitIntersect for bool {
    type Output = Self;

    fn intersect(self, rhs: Self) -> Self {
        self && rhs
    }
}

impl BitIntersect<Comp<bool>> for bool {
    type Output = Self;

    fn intersect(self, rhs: Comp<Self>) -> Self {
        self && (!rhs.0)
    }
}

impl BitDiff for bool {
    type Output = Self;

    fn diff(self, rhs: Self) -> Self {
        self && (!rhs)
    }
}

impl BitDiff<Comp<bool>> for bool {
    type Output = Self;

    fn diff(self, rhs: Comp<Self>) -> Self {
        self && rhs.0
    }
}

impl BitSubset for bool {
    fn is_subset_of(&self, rhs: &Self) -> bool {
        !*self || *rhs
    }
}

impl BitSubset<Comp<bool>> for bool {
    fn is_subset_of(&self, rhs: &Comp<Self>) -> bool {
        !*self || !rhs.0
    }
}

impl BitDisjoint for bool {
    fn is_disjoint(&self, rhs: &Self) -> bool {
        !*self || !*rhs
    }
}

impl BitDisjoint<Comp<bool>> for bool {
    fn is_disjoint(&self, rhs: &Comp<Self>) -> bool {
        !*self || rhs.0
    }
}
