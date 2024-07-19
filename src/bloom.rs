use core::hash::{BuildHasher, BuildHasherDefault, Hash, Hasher};

use crate::{ops::BitSet, BitEmpty, Bits};

/// Bloom is a probabilistic set data structure
/// that uses hashing and bitset to store element existence.
///
/// It can only tell if an element is definitely not in the set,
/// or might be in the set.
///
/// Configured with hash state and number of cycles.
///
/// When cycles number is greater than 1, then hash value is hashed again that much time,
/// and bits for each hash value are set.
/// On test all bits are checked, and if any of them is not set, then element is not in the set.
///
/// This trades speed with which bitset if filled, increasing probability of false positives,
/// with lower probability of false positives while bitset is mostly empty.
///
/// For example with bitset capacity = 100, cycles = 0 and 1 element added, probability of false positive ~= 0.01,
/// as only 1 bit is set, and 99 bits are not set and another element has to hash to the same bit.
///
/// With cycles = 1, probability of false positive ~= 0.02 * 0.02 = 0.0004, as 2 bits are set, but another element has hit both bits.
pub struct Bloom<B, S> {
    bitset: B,
    state: S,
    cycles: usize,
}

impl<B, S> Bloom<B, S>
where
    B: BitEmpty,
{
    /// Creates a new Bloom filter with given hash state.
    pub fn with_hasher(state: S) -> Self {
        Bloom::with_hasher_and_cycles(state, 0)
    }

    /// Creates a new Bloom filter with given hash state and number of cycles.
    pub fn with_hasher_and_cycles(state: S, cycles: usize) -> Self {
        Bloom {
            bitset: B::empty(),
            state,
            cycles,
        }
    }
}

impl<B, H> Bloom<B, BuildHasherDefault<H>>
where
    B: BitEmpty,
    H: Default,
{
    /// Creates a new Bloom filter with default hash state.
    pub fn new() -> Self {
        Self::with_hasher(BuildHasherDefault::default())
    }

    /// Creates a new Bloom filter with default hash state and number of cycles.
    pub fn with_cycles(cycles: usize) -> Self {
        Bloom::with_hasher_and_cycles(BuildHasherDefault::default(), cycles)
    }
}

impl<B, S> Bloom<B, S>
where
    B: BitSet,
    S: BuildHasher,
{
    /// Inserts an element into the Bloom filter.
    pub fn insert(&mut self, elem: impl Hash) {
        let mut hasher = self.state.build_hasher();
        elem.hash(&mut hasher);
        let mut hash = hasher.finish();

        self.bitset.set(into_idx::<B>(hash));

        for _ in 0..self.cycles {
            let mut hasher = self.state.build_hasher();
            hash.hash(&mut hasher);
            hash = hasher.finish();

            self.bitset.set(into_idx::<B>(hash));
        }
    }
}

impl<B, S> Bloom<B, S>
where
    B: Bits,
    S: BuildHasher,
{
    /// Checks if an element is in the Bloom filter.
    ///
    /// This can return false positives, but never false negatives.
    pub fn contains(&self, elem: impl Hash) -> bool {
        let mut hasher = self.state.build_hasher();
        elem.hash(&mut hasher);
        let mut hash = hasher.finish();

        if !self.bitset.test(into_idx::<B>(hash)) {
            return false;
        }

        for _ in 0..self.cycles {
            let mut hasher = self.state.build_hasher();
            hash.hash(&mut hasher);
            hash = hasher.finish();

            if !self.bitset.test(into_idx::<B>(hash)) {
                return false;
            }
        }

        true
    }
}

fn into_idx<B: Bits>(hash: u64) -> usize {
    if B::MAX_SET_INDEX == usize::MAX {
        hash as usize
    } else {
        (hash as usize) % (1 + B::MAX_SET_INDEX)
    }
}
