use core::array::SpanTrait;
use core::box::BoxTrait;
use core::dict::{Felt252Dict, Felt252DictEntryTrait, Felt252DictTrait};
use core::traits::PanicDestruct;
use stwo_cairo_verifier::BaseField;

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
pub impl ArrayImpl<T, +Copy<T>, +Drop<T>> of ArrayExTrait<T> {
    fn pop_n(ref self: Array<T>, mut n: usize) -> Array<T> {
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
    fn max<+PartialOrd<T>>(mut self: @Array<T>) -> Option<@T> {
        self.span().max()
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

pub fn pow(base: u32, mut exponent: u32) -> u32 {
    let mut result = 1;
    let mut base_power = base;
    loop {
        if exponent & 1 == 1 {
            result *= base_power;
        }
        exponent = exponent / 2;
        if exponent == 0 {
            break;
        }
        base_power = base_power * base_power;
    };
    result
}

/// Reverses the first `n_bits` many bits of `index`.
///
/// `index` should be in the range `[0..2^n_bits)`.
// TODO(andrew): Can consider byte lookup table if this is expensive.
pub fn bit_reverse_index(mut index: u32, mut n_bits: u32) -> usize {
    assert!(n_bits < 32);
    let mut res = 0;
    while n_bits != 0 {
        res = res * 2 + (index & 1);
        index /= 2;
        n_bits -= 1;
    };
    res
}

#[cfg(test)]
mod tests {
    use super::{pow, bit_reverse_index};

    #[test]
    fn test_pow() {
        assert_eq!(25, pow(5, 2));
        assert_eq!(16, pow(2, 4));
        assert_eq!(1024, pow(2, 10));
        assert_eq!(4096, pow(2, 12));
        assert_eq!(1048576, pow(2, 20));
    }

    #[test]
    fn test_bit_rev() {
        assert_eq!(0b0001, bit_reverse_index(0b1000, 4));
        assert_eq!(0b1011_0000, bit_reverse_index(0b0000_1101, 8));
    }
}

