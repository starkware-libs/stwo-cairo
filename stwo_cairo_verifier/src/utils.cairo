use core::array::SpanTrait;
use core::box::BoxTrait;
use core::dict::Felt252Dict;
use core::dict::Felt252DictEntryTrait;
use core::dict::Felt252DictTrait;
use core::iter::{IntoIterator, Iterator};
use core::num::traits::BitSize;
use core::traits::DivRem;
use core::traits::PanicDestruct;
use crate::BaseField;

/// Look up table where index `i` stores value `2^i`.
pub const POW_2: [u32; 32] = [
    0b1,
    0b10,
    0b100,
    0b1000,
    0b10000,
    0b100000,
    0b1000000,
    0b10000000,
    0b100000000,
    0b1000000000,
    0b10000000000,
    0b100000000000,
    0b1000000000000,
    0b10000000000000,
    0b100000000000000,
    0b1000000000000000,
    0b10000000000000000,
    0b100000000000000000,
    0b1000000000000000000,
    0b10000000000000000000,
    0b100000000000000000000,
    0b1000000000000000000000,
    0b10000000000000000000000,
    0b100000000000000000000000,
    0b1000000000000000000000000,
    0b10000000000000000000000000,
    0b100000000000000000000000000,
    0b1000000000000000000000000000,
    0b10000000000000000000000000000,
    0b100000000000000000000000000000,
    0b1000000000000000000000000000000,
    0b10000000000000000000000000000000,
];

#[generate_trait]
pub impl DictImpl<T, +Felt252DictValue<T>, +PanicDestruct<T>> of DictTrait<T> {
    fn replace(ref self: Felt252Dict<T>, key: felt252, new_value: T) -> T {
        let (entry, value) = self.entry(key);
        self = entry.finalize(new_value);
        value
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
    fn pop_front_n(ref self: Array<T>, mut n: usize) -> Array<T> {
        let mut res = array![];
        while n != 0 {
            if let Option::Some(value) = self.pop_front() {
                res.append(value);
            } else {
                break;
            }
            n -= 1;
        };
        res
    }

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
    fn next_if_eq<+PartialEq<T>>(ref self: Span<T>, other: @T) -> Option<@T> {
        if let Option::Some(value) = self.get(0) {
            if value.unbox() == other {
                return self.pop_front();
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

pub fn find(n: u32, a: Span<u32>) -> bool {
    let mut i = 0;
    let mut res = false;
    while i < a.len() {
        if (*a[i] == n) {
            res = true;
            break;
        }
        i = i + 1;
    };
    res
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

