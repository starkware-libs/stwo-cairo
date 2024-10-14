use core::array::SpanTrait;
use core::traits::PanicDestruct;
use core::option::OptionTrait;
use core::box::BoxTrait;
use core::dict::Felt252DictEntryTrait;
use core::dict::Felt252DictTrait;
use core::iter::Iterator;
use core::num::traits::BitSize;
use stwo_cairo_verifier::fields::qm31::{QM31, qm31};
use stwo_cairo_verifier::BaseField;
use core::traits::DivRem;
use stwo_cairo_verifier::fields::cm31::CM31;

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

pub fn bit_reverse(v: Span<CM31>) -> Array<CM31> {
    let n = v.len();
    let log_n = ilog2(n);

    let mut result = array![];
    let mut i = 0;
    while i < n {
        let j = bit_reverse_index(i, log_n);
        result.append(*v[j]);
        i = i + 1;
    };
    result
}

fn ilog2(n: u32) -> u32 {
    let mut log = 0;
    let mut current = n;
    while current > 1 {
        current = current / 2;
        log = log + 1;
    };
    log
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

pub fn flat_3d_array<T, +Copy<T>, +Drop<T>>(array: Span<Array<Array<T>>>) -> Array<T> {
    let mut flattened = array![];

    let mut i = 0;
    while i < array.len() {
        let middle_array = array.at(i).span();

        let mut j = 0;
        while j < middle_array.len() {
            let inner_array = middle_array.at(j).span();

            let mut k = 0;
            while k < inner_array.len() {
                let element = inner_array.at(k).clone();
                flattened.append(element);

                k = k + 1;
            };

            j = j + 1;
        };

        i = i + 1;
    };

    flattened
}

pub fn substract_map_2d_array<T, +Sub<T>, +Copy<T>, +Drop<T>>(
    array: Span<Array<T>>, substracting: T
) -> Array<T> {
    let mut mapped_array = array![];

    let mut i = 0;
    while i < array.len() {
        let inner_array = array.at(i).span();

        let mut j = 0;
        while j < inner_array.len() {
            let minuend = inner_array.at(j);
            mapped_array.append(*minuend - substracting);
            j = j + 1;
        };
        i = i + 1;
    };

    mapped_array
}

#[cfg(test)]
mod tests {
    use super::{
        pow, pow_qm31, qm31, bit_reverse_index, bit_reverse, get_unique_elements, flat_3d_array,
        substract_map_2d_array
    };
    use stwo_cairo_verifier::fields::cm31::cm31;

    #[test]
    fn test_pow() {
        assert_eq!(25, pow(5, 2));
        assert_eq!(16, pow(2, 4));
        assert_eq!(1024, pow(2, 10));
        assert_eq!(4096, pow(2, 12));
        assert_eq!(1048576, pow(2, 20));
    }

    #[test]
    fn test_bit_reverse_index() {
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
    fn test_bit_reverse() {
        // 1 bit
        let array_to_bit_reverse = array![cm31(0, 0), cm31(1, 0)];
        assert_eq!(array![cm31(0, 0), cm31(1, 0)], bit_reverse(array_to_bit_reverse.span()));

        // 2 bits
        let array_to_bit_reverse = array![cm31(0, 0), cm31(1, 0), cm31(2, 0), cm31(3, 0)];
        assert_eq!(
            array![cm31(0, 0), cm31(2, 0), cm31(1, 0), cm31(3, 0)],
            bit_reverse(array_to_bit_reverse.span())
        );

        // 3 bits
        let array_to_bit_reverse = array![
            cm31(0, 0),
            cm31(1, 0),
            cm31(2, 0),
            cm31(3, 0),
            cm31(4, 0),
            cm31(5, 0),
            cm31(6, 0),
            cm31(7, 0)
        ];
        assert_eq!(
            bit_reverse(array_to_bit_reverse.span()),
            array![
                cm31(0, 0),
                cm31(4, 0),
                cm31(2, 0),
                cm31(6, 0),
                cm31(1, 0),
                cm31(5, 0),
                cm31(3, 0),
                cm31(7, 0)
            ]
        );
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

    #[test]
    fn test_flat_3d_array() {
        let array_3d = array![
            array![array![1, 2], array![3, 4], array![5, 6]],
            array![array![7, 8], array![9, 10], array![11, 12]]
        ];

        assert_eq!(flat_3d_array(array_3d.span()), array![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }

    #[test]
    fn test_substract_a_value_from_2d_array() {
        let array_2d = array![
            array![1, 2], array![3, 4], array![5, 6], array![7, 8], array![9, 10], array![11, 12]
        ];

        assert_eq!(
            substract_map_2d_array(array_2d.span(), 1), array![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
        );
    }
}

