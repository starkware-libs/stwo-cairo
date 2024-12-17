use core::array::SpanTrait;
use core::box::BoxTrait;
use core::dict::Felt252Dict;
use core::dict::Felt252DictEntryTrait;
use core::dict::Felt252DictTrait;
use core::hash::HashStateTrait;
use core::iter::{IntoIterator, Iterator};
use core::num::traits::BitSize;
use core::poseidon::{HashState, hades_permutation};
use core::traits::DivRem;
use core::traits::PanicDestruct;
use crate::BaseField;

/// Look up table where index `i` stores value `2^i`.
const POW_2: [u32; 32] = [
    0b1, // 
    0b10, //
    0b100, //
    0b1000, //
    0b10000, //
    0b100000, //
    0b1000000, //
    0b10000000, //
    0b100000000, //
    0b1000000000, //
    0b10000000000, //
    0b100000000000, //
    0b1000000000000, //
    0b10000000000000, //
    0b100000000000000, //
    0b1000000000000000, //
    0b10000000000000000, //
    0b100000000000000000, //
    0b1000000000000000000, //
    0b10000000000000000000, //
    0b100000000000000000000, //
    0b1000000000000000000000, //
    0b10000000000000000000000, //
    0b100000000000000000000000, //
    0b1000000000000000000000000, //
    0b10000000000000000000000000, //
    0b100000000000000000000000000, //
    0b1000000000000000000000000000, //
    0b10000000000000000000000000000, //
    0b100000000000000000000000000000, //
    0b1000000000000000000000000000000, //
    0b10000000000000000000000000000000,
];

/// Returns `2^n`.
#[inline(always)]
pub fn pow2(n: u32) -> u32 {
    *POW_2.span()[n]
}

#[generate_trait]
pub impl DictImpl<T, +Felt252DictValue<T>> of DictTrait<T> {
    fn replace<+PanicDestruct<T>>(ref self: Felt252Dict<T>, key: felt252, new_value: T) -> T {
        let (entry, value) = self.entry(key);
        self = entry.finalize(new_value);
        value
    }

    // TODO(andrew): Is there a better way to handle this?
    fn clone_subset<+Copy<T>, +Drop<T>>(
        ref self: Felt252Dict<T>, subset_keys: Span<u32>,
    ) -> Felt252Dict<T> {
        let mut res: Felt252Dict<T> = Default::default();
        for key in subset_keys {
            let key = (*key).into();
            res.insert(key, self.get(key));
        };
        res
    }
}

#[generate_trait]
pub impl OptBoxImpl<T> of OptBoxTrait<T> {
    fn as_unboxed(self: Option<Box<T>>) -> Option<T> {
        match self {
            Option::Some(value) => Option::Some(value.unbox()),
            Option::None => Option::None,
        }
    }
}

#[generate_trait]
pub impl OptionImpl<T> of OptionExTrait<T> {
    /// Converts from `@Option<T>` to `Option<@T>`.
    fn as_snap(self: @Option<T>) -> Option<@T> {
        match self {
            Option::Some(x) => Option::Some(x),
            Option::None => Option::None,
        }
    }
}

#[generate_trait]
pub impl ArrayImpl<T, +Drop<T>> of ArrayExTrait<T> {
    fn max<+Copy<T>, +PartialOrd<T>>(mut self: @Array<T>) -> Option<@T> {
        self.span().max()
    }

    /// Sorts an array in ascending order. Uses quicksort algorithm.
    fn sort_ascending<+Clone<T>, +PartialOrd<T>>(self: Array<T>) -> Array<T> {
        if self.len() <= 1 {
            return self;
        }

        let mut lhs = array![];
        let mut rhs = array![];
        let mut iter = self.into_iter();
        let pivot = iter.next().unwrap();

        for v in iter {
            if v.clone() > pivot.clone() {
                rhs.append(v);
            } else {
                lhs.append(v);
            }
        };

        let mut res = lhs.sort_ascending();
        res.append(pivot);

        for v in rhs.sort_ascending() {
            res.append(v);
        };

        res
    }

    /// Removes consecutive repeated elements.
    ///
    /// If the vector is sorted, this removes all duplicates.
    fn dedup<+PartialEq<T>>(self: Array<T>) -> Array<T> {
        if self.len() == 0 {
            return array![];
        }

        let mut iter = self.into_iter();
        let mut res = array![iter.next().unwrap()];
        let mut last_value = res[0];
        for value in iter {
            if @value != last_value {
                last_value = @value;
                res.append(value);
            }
        };

        res
    }

    fn new_repeated<+Clone<T>>(n: usize, v: T) -> Array<T> {
        let mut res = array![];
        for _ in 0..n {
            res.append(v.clone());
        };
        res
    }
}

#[generate_trait]
pub impl SpanImpl<T> of SpanExTrait<T> {
    #[inline]
    fn first(mut self: Span<T>) -> Option<@T> {
        self.pop_front()
    }

    fn pop_front_n(ref self: Span<T>, n: usize) -> Span<T> {
        let (res, remainder) = self.split_at(n);
        self = remainder;
        res
    }

    #[inline]
    fn split_at(self: Span<T>, mid: usize) -> (Span<T>, Span<T>) {
        (self.slice(0, mid), self.slice(mid, self.len() - mid))
    }

    fn next_if_eq<+PartialEq<T>>(ref self: Span<T>, other: @T) -> Option<@T> {
        let mut self_copy = self;
        if let Option::Some(value) = self_copy.pop_front() {
            if value == other {
                self = self_copy;
                return Option::Some(other);
            }
        }
        Option::None
    }

    fn max<+PartialOrd<T>, +Copy<T>>(mut self: Span<T>) -> Option<@T> {
        let mut max = self.pop_front()?;
        loop {
            if let Option::Some(next) = self.pop_front() {
                if *next > *max {
                    max = next;
                }
            } else {
                break;
            }
        };
        Option::Some(max)
    }
}

const M31_SHIFT: felt252 = 0x80000000; // 2**31.
// Packs 4 BaseField values and "append" to a felt252.
// The resulting felt252 is: cur || x0 || x1 || x2 || x3.
pub fn pack4(cur: felt252, values: [BaseField; 4]) -> felt252 {
    let [x0, x1, x2, x3] = values;
    (((cur * M31_SHIFT + x0.into()) * M31_SHIFT + x1.into()) * M31_SHIFT + x2.into()) * M31_SHIFT
        + x3.into()
}

pub fn bit_reverse_index(mut index: usize, mut bits: u32) -> usize {
    assert!(bits <= BitSize::<usize>::bits());

    let NZ2: NonZero<usize> = 2;

    let mut result = 0;
    while bits > 0 {
        let (next_index, bit) = DivRem::div_rem(index, NZ2);
        result = (result * 2) | bit;
        index = next_index;
        bits -= 1;
    };
    result
}

/// Computes the Poseidon hash on the given input.
///
/// Applies the sponge construction to digest many elements.
/// To distinguish between use cases, the capacity element is initialized to 0.
/// To distinguish between different input sizes always pads with 1, and possibly with another 0 to
/// complete to an even-sized input.
// Copied from standard library. Removed gas counting.
pub fn poseidon_hash_span(mut span: Span<felt252>) -> felt252 {
    _poseidon_hash_span_inner((0, 0, 0), ref span)
}

/// Helper function for poseidon_hash_span.
// Copied from standard library. Removed gas counting.
fn _poseidon_hash_span_inner(
    state: (felt252, felt252, felt252), ref span: Span<felt252>,
) -> felt252 {
    let (s0, s1, s2) = state;
    let x = *match span.pop_front() {
        Option::Some(x) => x,
        Option::None => { return HashState { s0, s1, s2, odd: false }.finalize(); },
    };
    let y = *match span.pop_front() {
        Option::Some(y) => y,
        Option::None => { return HashState { s0: s0 + x, s1, s2, odd: true }.finalize(); },
    };
    let next_state = hades_permutation(s0 + x, s1 + y, s2);
    _poseidon_hash_span_inner(next_state, ref span)
}


#[cfg(test)]
mod tests {
    use super::{ArrayImpl, bit_reverse_index};

    #[test]
    fn test_bit_reverse() {
        // 1 bit
        assert_eq!(0, bit_reverse_index(0, 1));
        assert_eq!(1, bit_reverse_index(1, 1));

        // 2 bits
        assert_eq!(0, bit_reverse_index(0, 2));
        assert_eq!(2, bit_reverse_index(1, 2));
        assert_eq!(1, bit_reverse_index(2, 2));
        assert_eq!(3, bit_reverse_index(3, 2));

        // 3 bits
        assert_eq!(0, bit_reverse_index(0, 3));
        assert_eq!(4, bit_reverse_index(1, 3));
        assert_eq!(2, bit_reverse_index(2, 3));
        assert_eq!(6, bit_reverse_index(3, 3));

        // 16 bits
        assert_eq!(24415, bit_reverse_index(64250, 16));

        // 31 bits
        assert_eq!(16448250, bit_reverse_index(800042880, 31));
    }

    #[test]
    fn test_sort_ascending() {
        assert_eq!(array![6_usize, 5, 1, 4, 2, 3].sort_ascending(), array![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_dedup() {
        assert_eq!(array![1_usize, 1, 1, 2, 2, 3, 4, 5, 5, 5].dedup(), array![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_array_new_repeated() {
        assert_eq!(ArrayImpl::new_repeated(5, 3_usize), array![3, 3, 3, 3, 3]);
    }
}

