use core::array::SpanTrait;
use core::box::BoxTrait;
use core::dict::{Felt252Dict, Felt252DictEntryTrait, Felt252DictTrait, SquashedFelt252DictTrait};
use core::iter::{IntoIterator, Iterator};
use core::nullable::{FromNullableResult, NullableTrait, match_nullable, null};
use core::num::traits::{BitSize, WrappingMul, WrappingSub};
use core::traits::{DivRem, PanicDestruct};
use crate::circle::M31_CIRCLE_LOG_ORDER;
use crate::fields::m31::{M31, M31_SHIFT};
use crate::{TreeArray, TreeSpan};

/// Returns `2^n`.
#[inline(always)]
pub fn pow2(n: u32) -> u32 {
    /// Look up table where index `i` stores value `2^i`.
    #[cairofmt::skip]
    const POW_2: [u32; 32] = [
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
        }
        res
    }
}

#[generate_trait]
pub impl OptBoxImpl<T> of OptBoxTrait<T> {
    fn as_unboxed(self: Option<Box<T>>) -> Option<T> {
        match self {
            Some(value) => Some(value.unbox()),
            None => None,
        }
    }
}

#[generate_trait]
pub impl OptionImpl<T> of OptionExTrait<T> {
    /// Converts from `@Option<T>` to `Option<@T>`.
    fn as_snap(self: @Option<T>) -> Option<@T> {
        match self {
            Some(x) => Some(x),
            None => None,
        }
    }
}

#[generate_trait]
pub impl ArrayImpl<T, +Drop<T>> of ArrayExTrait<T> {
    fn max<+Copy<T>, +PartialOrd<T>>(mut self: @Array<T>) -> Option<@T> {
        self.span().max()
    }

    fn new_repeated<+Clone<T>>(n: usize, v: T) -> Array<T> {
        let mut res = array![];
        for _ in 0..n {
            res.append(v.clone());
        }
        res
    }
}

#[generate_trait]
pub impl SpanImpl<T> of SpanExTrait<T> {
    #[inline]
    fn first(mut self: Span<T>) -> Option<@T> {
        self.pop_front()
    }

    #[inline]
    fn last(mut self: Span<T>) -> Option<@T> {
        self.pop_back()
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
        if let Some(value) = self_copy.pop_front() {
            if value == other {
                self = self_copy;
                return Some(other);
            }
        }
        None
    }

    fn max<+PartialOrd<T>, +Copy<T>>(mut self: Span<T>) -> Option<@T> {
        let mut max = self.pop_front()?;
        while let Some(next) = self.pop_front() {
            if *next > *max {
                max = next;
            }
        }
        Some(max)
    }
}

// Packs 4 BaseField values and "append" to a felt252.
// The resulting felt252 is: cur || x0 || x1 || x2 || x3.
pub fn pack4(cur: felt252, values: [M31; 4]) -> felt252 {
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
    }
    result
}

/// Generates a bit mask with the least significant `n_bits` set to 1.
pub fn gen_bit_mask(n_bits: u32) -> u64 {
    /// Look up table where index `i` stores a bit mask with the least significant `i` bits set to
    /// 1.
    #[cairofmt::skip]
    const BIT_MASKS: [u64; 65] = [
        0b0,
        0b1,
        0b11,
        0b111,
        0b1111,
        0b11111,
        0b111111,
        0b1111111,
        0b11111111,
        0b111111111,
        0b1111111111,
        0b11111111111,
        0b111111111111,
        0b1111111111111,
        0b11111111111111,
        0b111111111111111,
        0b1111111111111111,
        0b11111111111111111,
        0b111111111111111111,
        0b1111111111111111111,
        0b11111111111111111111,
        0b111111111111111111111,
        0b1111111111111111111111,
        0b11111111111111111111111,
        0b111111111111111111111111,
        0b1111111111111111111111111,
        0b11111111111111111111111111,
        0b111111111111111111111111111,
        0b1111111111111111111111111111,
        0b11111111111111111111111111111,
        0b111111111111111111111111111111,
        0b1111111111111111111111111111111,
        0b11111111111111111111111111111111,
        0b111111111111111111111111111111111,
        0b1111111111111111111111111111111111,
        0b11111111111111111111111111111111111,
        0b111111111111111111111111111111111111,
        0b1111111111111111111111111111111111111,
        0b11111111111111111111111111111111111111,
        0b111111111111111111111111111111111111111,
        0b1111111111111111111111111111111111111111,
        0b11111111111111111111111111111111111111111,
        0b111111111111111111111111111111111111111111,
        0b1111111111111111111111111111111111111111111,
        0b11111111111111111111111111111111111111111111,
        0b111111111111111111111111111111111111111111111,
        0b1111111111111111111111111111111111111111111111,
        0b11111111111111111111111111111111111111111111111,
        0b111111111111111111111111111111111111111111111111,
        0b1111111111111111111111111111111111111111111111111,
        0b11111111111111111111111111111111111111111111111111,
        0b111111111111111111111111111111111111111111111111111,
        0b1111111111111111111111111111111111111111111111111111,
        0b11111111111111111111111111111111111111111111111111111,
        0b111111111111111111111111111111111111111111111111111111,
        0b1111111111111111111111111111111111111111111111111111111,
        0b11111111111111111111111111111111111111111111111111111111,
        0b111111111111111111111111111111111111111111111111111111111,
        0b1111111111111111111111111111111111111111111111111111111111,
        0b11111111111111111111111111111111111111111111111111111111111,
        0b111111111111111111111111111111111111111111111111111111111111,
        0b1111111111111111111111111111111111111111111111111111111111111,
        0b11111111111111111111111111111111111111111111111111111111111111,
        0b111111111111111111111111111111111111111111111111111111111111111,
        0b1111111111111111111111111111111111111111111111111111111111111111
        ];

    *BIT_MASKS.span()[n_bits]
}

/// Given a span of column log sizes, Return a span of the column indices grouped by their log
/// size.
///
/// # Arguments
///
/// * `column_log_sizes`: The log sizes of the columns.
///
/// # Returns
///
/// * `columns_by_log_size`: A span where the i'th element is a span of the column indices of size
/// 2**i.
pub fn group_columns_by_log_size(column_log_sizes: Span<u32>) -> Span<Span<usize>> {
    let mut columns_by_log_size = Default::default();
    let mut col_index = 0_usize;
    for col_size in column_log_sizes {
        let (columns_by_log_size_entry, value) = columns_by_log_size.entry((*col_size).into());
        let mut columns_of_size = match match_nullable(value) {
            FromNullableResult::Null => array![],
            FromNullableResult::NotNull(value) => value.unbox(),
        };
        columns_of_size.append(col_index);
        columns_by_log_size = columns_by_log_size_entry
            .finalize(NullableTrait::new(columns_of_size));
        col_index += 1;
    }

    let mut res = array![];
    let empty_span = array![].span();
    for (columns_log_size, _, columns) in columns_by_log_size.squash().into_entries() {
        /// Add empty spans for missing log sizes.
        while res.len().into() != columns_log_size {
            res.append(empty_span);
        }
        res.append(columns.deref().span());
    }
    res.span()
}
