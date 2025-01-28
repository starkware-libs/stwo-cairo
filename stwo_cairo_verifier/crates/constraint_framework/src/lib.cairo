use core::dict::{Felt252Dict, Felt252DictTrait};
use core::nullable::{Nullable, NullableTrait};
use stwo_verifier_core::channel::{Channel, ChannelImpl};
use stwo_verifier_core::fields::m31::M31;
use stwo_verifier_core::fields::qm31::{QM31, QM31Impl, QM31One, QM31Zero};

/// Represents the value of the prefix sum column at some index.
/// Should be used to eliminate padded rows for the logup sum.
// Copied from:
pub type ClaimedPrefixSum = (QM31, usize);

#[derive(Drop, Clone)]
pub struct LookupElements<const N: usize> {
    pub z: QM31,
    pub alpha: QM31,
    pub alpha_powers: Array<QM31>,
}

#[generate_trait]
pub impl LookupElementsImpl<const N: usize> of LookupElementsTrait<N> {
    fn draw(ref channel: Channel) -> LookupElements<N> {
        assert!(N != 0);
        let [z, alpha]: [QM31; 2] = (*channel.draw_felts(2).span().try_into().unwrap()).unbox();

        let mut acc = QM31One::one();
        let mut alpha_powers = array![acc];

        for _ in 1..N {
            acc *= alpha;
            alpha_powers.append(acc);
        };

        LookupElements { z, alpha, alpha_powers }
    }

    fn combine<impl IntoSpan: ToSpanTrait<[M31; N], M31>>(
        self: @LookupElements<N>, values: [M31; N],
    ) -> QM31 {
        let mut alpha_powers = self.alpha_powers.span();
        let mut values_span = IntoSpan::span(@values);
        let mut sum = -*self.z;

        while let (Option::Some(alpha), Option::Some(value)) =
            (alpha_powers.pop_front(), values_span.pop_front()) {
            sum += (*alpha).mul_m31(*value);
        };

        sum
    }
}


#[derive(Destruct, Default)]
pub struct PreprocessedColumnSet {
    /// Unique list of preprocessed columns in the set.
    pub values: Array<PreprocessedColumn>,
    /// Map indicating if a preprocessed column is already in the set.
    pub contains: Felt252Dict<bool>,
}

#[generate_trait]
pub impl PreprocessedColumnSetImpl of PreprocessedColumnSetTrait {
    fn insert(ref self: PreprocessedColumnSet, preprocessed_column: PreprocessedColumn) {
        let column_key = PreprocessedColumnKey::encode(@preprocessed_column);

        if self.contains.get(column_key) {
            return;
        }

        self.values.append(preprocessed_column);
        self.contains.insert(column_key, true);
    }

    fn contains(ref self: PreprocessedColumnSet, preprocessed_column: PreprocessedColumn) -> bool {
        let column_key = PreprocessedColumnKey::encode(@preprocessed_column);
        self.contains.get(column_key)
    }
}

#[derive(Destruct)]
pub struct PreprocessedMaskValues {
    pub values: Felt252Dict<Nullable<QM31>>,
}

#[generate_trait]
pub impl PreprocessedMaskValuesImpl of PreprocessedMaskValuesTrait {
    fn new(
        mut preprocessed_mask_values: Span<Array<QM31>>,
        preprocessed_columns: Span<PreprocessedColumn>,
    ) -> PreprocessedMaskValues {
        let mut values: Felt252Dict<Nullable<QM31>> = Default::default();

        for preprocessed_column in preprocessed_columns {
            let mut column_mask_values = preprocessed_mask_values.pop_front().unwrap().span();
            if let Option::Some(mask_value) = column_mask_values.pop_front() {
                values
                    .insert(
                        PreprocessedColumnKey::encode(preprocessed_column),
                        NullableTrait::new(*mask_value),
                    );
            }
        };

        assert!(preprocessed_mask_values.is_empty());

        PreprocessedMaskValues { values }
    }

    fn get(ref self: PreprocessedMaskValues, preprocessed_column: PreprocessedColumn) -> QM31 {
        self.values.get(PreprocessedColumnKey::encode(@preprocessed_column)).deref()
    }
}

#[derive(Debug, Default, Drop)]
enum PreprocessedColumnsAllocationMode {
    #[default]
    Dynamic,
    Static,
}

#[derive(Drop, Debug, Copy, PartialEq)]
pub enum PreprocessedColumn {
    /// Symbolic representation of xor lookup table of the form: `(elem_bits, expand_bits, term)`.
    /// Where term is `{ 0 = left operand, 1 = right operand, 2 = xor result }`.
    XorTable: (u32, u32, usize),
    /// Stores the log size of the column.
    IsFirst: u32,
    Plonk: usize,
    /// A column with the numbers [0..2^log_size-1].
    Seq: u32,
}

/// An encoding of a [`PreprocessedColumn`] to index into [`Felt252Dict`].
#[generate_trait]
pub impl PreprocessedColumnKey of PreprocessedColumnKeyTrait {
    fn encode(key: @PreprocessedColumn) -> felt252 {
        const FELT252_2_POW_32: felt252 = 0x100000000;
        // TODO: Is there something like Rust's `core::mem::discriminant` in Cairo?
        const XOR_TABLE_DISCRIMINANT: felt252 = 0;
        const IS_FIRST_TABLE_DISCRIMINANT: felt252 = 1;
        const PLONK_TABLE_DISCRIMINANT: felt252 = 2;
        const SEQ_TABLE_DISCRIMINANT: felt252 = 3;

        match key {
            PreprocessedColumn::XorTable((
                elem_bits, expand_bits, term,
            )) => {
                let mut res = (*term).into();
                res = res * FELT252_2_POW_32 + (*expand_bits).into();
                res = res * FELT252_2_POW_32 + (*elem_bits).into();
                res = res * FELT252_2_POW_32 + XOR_TABLE_DISCRIMINANT;
                res
            },
            PreprocessedColumn::IsFirst(log_size) => {
                let mut res = (*log_size).into();
                res = res * FELT252_2_POW_32 + IS_FIRST_TABLE_DISCRIMINANT;
                res
            },
            PreprocessedColumn::Plonk(v) => {
                let mut res = (*v).into();
                res = res * FELT252_2_POW_32 + PLONK_TABLE_DISCRIMINANT;
                res
            },
            PreprocessedColumn::Seq(log_size) => {
                let mut res = (*log_size).into();
                res = res * FELT252_2_POW_32 + SEQ_TABLE_DISCRIMINANT;
                res
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PreprocessedColumn, PreprocessedColumnSetImpl, PreprocessedColumnSet};

    #[test]
    fn test_preprocessed_column_set() {
        let mut set: PreprocessedColumnSet = Default::default();
        let is_first_16_column = PreprocessedColumn::IsFirst(16);
        let is_first_10_column = PreprocessedColumn::IsFirst(10);

        set.insert(is_first_16_column);
        set.insert(is_first_16_column);

        assert!(set.contains(is_first_16_column));
        assert!(!set.contains(is_first_10_column));
        assert_eq!(set.values, array![is_first_16_column]);
    }
}
