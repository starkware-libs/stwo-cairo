use core::array::SpanTrait;
use core::box::BoxTrait;
use core::dict::{Felt252Dict, Felt252DictEntryTrait, Felt252DictTrait, SquashedFelt252DictTrait};
use core::nullable::{FromNullableResult, NullableTrait, match_nullable};
use core::num::traits::BitSize;
use core::traits::{DivRem, PanicDestruct};
use crate::fields::SecureField;
use crate::fields::m31::M31_SHIFT;
use crate::fields::qm31::QM31Trait;
use crate::{ColumnSpan, TreeSpan};

/// Returns `2^n`, n in range [0, 32).
/// Will panic (with index out of bounds) if n >= 32.
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

/// Returns `2^n` as a u64, n in range [0, 64).
/// Will panic (with index out of bounds) if n >= 64.
pub fn pow2_u64(n: u32) -> u64 {
    if n < 32 {
        pow2(n).into()
    } else {
        pow2(n - 32).into() * 0x100000000
    }
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

    /// Panics if self.len() < n.
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
        if let Some(value) = self_copy.pop_front() && value == other {
            self = self_copy;
            return Some(other);
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

// Packs a SecureField value into a felt252, injecting `cur` into
// the most significant bits.
// The resulting felt252 is: cur || x0 || x1 || x2 || x3.
pub fn pack_qm31(cur: felt252, secure_felt: SecureField) -> felt252 {
    let [x0, x1, x2, x3] = secure_felt.to_fixed_array();
    (((cur * M31_SHIFT + x0.into()) * M31_SHIFT + x1.into()) * M31_SHIFT + x2.into()) * M31_SHIFT
        + x3.into()
}

/// Takes the first `n_bits` bits of the given index, reverses them, and returns the result.
pub fn bit_reverse_index(mut index: usize, mut n_bits: u32) -> usize {
    assert!(n_bits <= BitSize::<usize>::bits());

    let mut n_bits: felt252 = n_bits.into();
    let mut result = 0;
    while n_bits != 0 {
        let (next_index, bit) = DivRem::div_rem(index, 2);
        result = result * 2 + bit;
        index = next_index;
        n_bits -= 1;
    }
    result
}

/// A span in which each element relates (by index) to the log 2 of a degree bound.
pub type LogDegreeBoundSpan<T> = Span<T>;

/// Holds the columns indices by log degree bound.
///
/// column_indices_by_degree_bound[log_degree_bound] is a span of the columns indices with degree
/// bound `degree_bound`.
/// The indices in each tree are 0-based.
///
pub type ColumnsIndicesByLogDegreeBound = LogDegreeBoundSpan<Span<usize>>;

/// Given a span of column log degree bounds, Return a span of the column indices grouped by their
/// log degree bound.
///
/// # Arguments
///
/// * `log_degree_bound_by_column`: The degree bounds of the columns.
///
/// # Returns
///
/// * `columns_by_log_degree_bound`: A span where the i'th element is a span of the column indices
/// of size 2**i.
pub fn group_columns_by_degree_bound(
    log_degree_bound_by_column: ColumnSpan<u32>,
) -> ColumnsIndicesByLogDegreeBound {
    let mut column_by_degree_bound: Felt252Dict<Nullable<Array<u32>>> = Default::default();
    let mut col_index = 0_usize;
    for column_log_degree_bound in log_degree_bound_by_column {
        let (column_by_degree_bound_entry, value) = column_by_degree_bound
            .entry((*column_log_degree_bound).into());
        let mut column_indices = match match_nullable(value) {
            FromNullableResult::Null => array![],
            FromNullableResult::NotNull(value) => value.unbox(),
        };
        column_indices.append(col_index);
        column_by_degree_bound = column_by_degree_bound_entry
            .finalize(NullableTrait::new(column_indices));
        col_index += 1;
    }

    let mut res = array![];
    for (column_degree_bound, _, column_indices) in column_by_degree_bound.squash().into_entries() {
        /// Add empty spans for missing degree bounds.
        while res.len().into() != column_degree_bound {
            res.append(array![].span());
        }
        res.append(column_indices.deref().span());
    }
    res.span()
}

/// Holds the columns indices per tree by degree bound.
///
/// columns_indices_per_tree_by_log_degree_bound[log_degree_bound][tree] is a span of the columns
/// indices with degree bound `log_degree_bound` in the tree `tree`.
/// The indices in each tree are 0-based.
///
pub type ColumnsIndicesPerTreeByLogDegreeBound = LogDegreeBoundSpan<TreeSpan<Span<usize>>>;

/// Pads all the trees in `columns_by_log_degree_bound_per_tree` to the length of the longest tree
/// and transposes the arrays from [tree][log_degree_bound][column] to
/// [log_degree_bound][tree][column].
///
/// # Arguments
///
/// * `columns_by_log_degree_bound_per_tree`: The columns by log size per tree.
///
/// # Returns
///
/// * `columns_per_tree_by_log_degree_bound`: The columns per tree by log degree bound.
pub fn pad_and_transpose_columns_by_log_deg_bound_per_tree(
    mut columns_by_log_deg_bound_per_tree: TreeSpan<ColumnsIndicesByLogDegreeBound>,
) -> ColumnsIndicesPerTreeByLogDegreeBound {
    let mut columns_per_tree_by_log_deg_bound = array![];

    loop {
        // In each iteration we pop the the columns corresponding to `log_degree_bound` from each
        // tree, so we need to prepare `next_columns_by_log_deg_bound_per_tree` for the next
        // iteration.
        let mut next_columns_by_log_deg_bound_per_tree = array![];

        let mut done = true;
        let mut columns_per_tree = array![];
        for columns_by_log_deg_bound in columns_by_log_deg_bound_per_tree {
            let mut columns_by_log_deg_bound = *columns_by_log_deg_bound;
            let column_indices = match columns_by_log_deg_bound.pop_front() {
                Some(column_indices) => {
                    done = false;
                    *column_indices
                },
                None => array![].span(),
            };
            columns_per_tree.append(column_indices);

            next_columns_by_log_deg_bound_per_tree.append(columns_by_log_deg_bound);
        }

        if done {
            break;
        }

        columns_by_log_deg_bound_per_tree = next_columns_by_log_deg_bound_per_tree.span();
        columns_per_tree_by_log_deg_bound.append(columns_per_tree.span());
    }

    columns_per_tree_by_log_deg_bound.span()
}
