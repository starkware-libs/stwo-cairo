use core::array::SpanTrait;
use core::traits::PanicDestruct;
use core::option::OptionTrait;
use core::box::BoxTrait;
use core::dict::Felt252DictEntryTrait;
use core::dict::Felt252DictTrait;
use core::iter::Iterator;
use stwo_cairo_verifier::fields::qm31::{QM31, qm31};
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


pub fn bit_reverse_index(mut index: usize, mut bits: u32) -> usize {
    assert!(bits < 32);
    let mut result = 0;
    let mut pow_of_two = 1;
    while bits > 0 {
        result *= 2;
        result = result | ((index / pow_of_two) & 1);
        pow_of_two *= 2;
        bits -= 1;
    };
    result
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

pub fn pow_qm31(base: QM31, mut exponent: u32) -> QM31 {
    let mut result = qm31(1, 0, 0, 0);
    let mut base_power = base;
    loop {
        if exponent & 1 == 1 {
            result = result * base_power;
        }
        exponent = exponent / 2;
        if exponent == 0 {
            break;
        }
        base_power = base_power * base_power;
    };
    result
}

pub fn qm31_zero_array(n: u32) -> Array<QM31> {
    let mut result = array![];
    let mut i = 0;
    while i < n {
        result.append(qm31(0, 0, 0, 0));
        i += 1;
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

pub fn get_unique_elements<T, +PartialEq<T>, +Drop<T>, +Copy<T>>(vector: @Array<T>) -> Array<T> {
    let mut uniques: Array<T> = array![];

    let mut i = 0;
    while i < vector.len() {
        if !contains_element(@uniques, vector[i]) {
            uniques.append(*vector[i]);
        }
        i = i + 1
    };

    uniques
}


pub fn contains_element<T, +PartialEq<T>>(vector: @Array<T>, element: @T) -> bool {
    let mut contains = false;

    let mut i = 0;
    while i < vector.len() {
        if vector[i] == element {
            contains = true;
            break;
        }
        i = i + 1;
    };
    contains
}


#[cfg(test)]
mod tests {
    use super::{pow, bit_reverse_index, pow_qm31, qm31, get_unique_elements};

    #[test]
    fn test_pow() {
        assert_eq!(25, pow(5, 2));
        assert_eq!(16, pow(2, 4));
        assert_eq!(1024, pow(2, 10));
        assert_eq!(4096, pow(2, 12));
        assert_eq!(1048576, pow(2, 20));
    }

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
    fn test_pow_qm31_1() {
        let result = pow_qm31(qm31(1, 2, 3, 4), 0);
        let expected_result = qm31(1, 0, 0, 0);
        assert_eq!(expected_result, result)
    }

    #[test]
    fn test_pow_qm31_2() {
        let result = pow_qm31(qm31(1, 2, 3, 4), 1);
        let expected_result = qm31(1, 2, 3, 4);
        assert_eq!(expected_result, result)
    }

    #[test]
    fn test_pow_qm31_3() {
        let result = pow_qm31(qm31(1, 2, 3, 4), 37);
        let expected_result = qm31(1394542587, 260510989, 997191897, 2127074080);
        assert_eq!(expected_result, result)
    }

    #[test]
    fn test_can_get_unique_elements_of_array() {
        let vector_1 = array![1, 1, 2, 2, 3, 3, 4, 4, 5, 5];
        let expected_vector_1 = array![1, 2, 3, 4, 5];
        let vector_2 = array![1, 1, 1, 1, 1, 1, 1, 1];
        let expected_vector_2 = array![1];
        let vector_3 = array![1, 2, 3, 4, 5];
        let expected_vector_3 = array![1, 2, 3, 4, 5];

        let return_vector_1 = get_unique_elements(@vector_1);
        let return_vector_2 = get_unique_elements(@vector_2);
        let return_vector_3 = get_unique_elements(@vector_3);

        assert_eq!(expected_vector_1, return_vector_1);
        assert_eq!(expected_vector_2, return_vector_2);
        assert_eq!(expected_vector_3, return_vector_3);
    }
}

