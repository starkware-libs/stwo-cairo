use core::dict::{Felt252Dict, Felt252DictTrait};
use core::nullable::{Nullable, NullableTrait};
use core::num::traits::One;
use stwo_verifier_core::ColumnSpan;
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::fields::m31::M31;
use stwo_verifier_core::fields::qm31::{QM31, QM31Trait};

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
        let [z, alpha]: [QM31; 2] = (*channel.draw_secure_felts(2).span().try_into().unwrap()).unbox();

        let mut acc = One::one();
        let mut alpha_powers = array![acc];

        for _ in 1..N {
            acc *= alpha;
            alpha_powers.append(acc);
        }

        LookupElements { z, alpha, alpha_powers }
    }

    fn combine<impl IntoSpan: ToSpanTrait<[M31; N], M31>>(
        self: @LookupElements<N>, values: [M31; N],
    ) -> QM31 {
        let mut alpha_powers = self.alpha_powers.span();
        let mut values_span = IntoSpan::span(@values);
        let mut sum = -*self.z;

        while let (Some(alpha), Some(value)) = (alpha_powers.pop_front(), values_span.pop_front()) {
            sum += (*alpha).mul_m31(*value);
        }

        sum
    }

    fn combine_qm31<impl IntoSpan: ToSpanTrait<[QM31; N], QM31>>(
        self: @LookupElements<N>, values: [QM31; N],
    ) -> QM31 {
        let mut alpha_powers = self.alpha_powers.span();
        let mut values_span = IntoSpan::span(@values);
        let mut sum = -*self.z;

        while let (Some(alpha), Some(value)) = (alpha_powers.pop_front(), values_span.pop_front()) {
            sum += (*alpha) * (*value);
        }

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
        mut preprocessed_mask_values: ColumnSpan<Span<QM31>>,
        preprocessed_columns: Span<PreprocessedColumn>,
    ) -> PreprocessedMaskValues {
        let mut values: Felt252Dict<Nullable<QM31>> = Default::default();

        for preprocessed_column in preprocessed_columns {
            let mut column_mask_values = *preprocessed_mask_values.pop_front().unwrap();

            if let Some(mask_value) = column_mask_values.pop_front() {
                values
                    .insert(
                        PreprocessedColumnKey::encode(preprocessed_column),
                        NullableTrait::new(*mask_value),
                    );

                // Preprocessed columns should have at most one mask item.
                assert!(column_mask_values.is_empty());
            }
        }

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
    /// Symbolic representation of xor lookup table column of the form: `(n_term_bits, term)`.
    /// Where term is `{ 0 = left operand, 1 = right operand, 2 = xor result }`.
    BitwiseXor: (u32, usize),
    /// A column with the numbers [0..2^log_size-1].
    Seq: u32,
    /// Symbolic representation of range check column.
    /// The column is of the form `(log_ranges, column_index)`.
    RangeCheck5: ([u32; 5], usize),
    RangeCheck4: ([u32; 4], usize),
    RangeCheck3: ([u32; 3], usize),
    RangeCheck2: ([u32; 2], usize),
    /// Poseidon round keys of the form `(column_index)`.
    PoseidonRoundKeys: usize,
    /// Blake2s sigma column.
    BlakeSigma: usize,
    /// Pedersen points of the form `(column_index)`.
    PedersenPoints: usize,
}

#[generate_trait]
pub impl PreprocessedColumnImpl of PreprocessedColumnTrait {
    fn log_size(self: @PreprocessedColumn) -> u32 {
        match self {
            PreprocessedColumn::BitwiseXor((n_term_bits, _)) => *n_term_bits * 2,
            PreprocessedColumn::Seq(log_size) => *log_size,
            PreprocessedColumn::RangeCheck5((values, _)) => range_check_size(values),
            PreprocessedColumn::RangeCheck4((values, _)) => range_check_size(values),
            PreprocessedColumn::RangeCheck3((values, _)) => range_check_size(values),
            PreprocessedColumn::RangeCheck2((values, _)) => range_check_size(values),
            PreprocessedColumn::PoseidonRoundKeys(_) => 6,
            PreprocessedColumn::BlakeSigma(_) => 4,
            PreprocessedColumn::PedersenPoints(_) => 23,
        }
    }
}

pub fn range_check_size<const N: usize, impl IntoSpan: ToSpanTrait<[u32; N], u32>>(
    n_bits_vec: @[u32; N],
) -> u32 {
    let mut total: u32 = 0;
    for n_bits in IntoSpan::span(n_bits_vec) {
        total = total + *n_bits;
    }
    total
}

/// An encoding of a [`PreprocessedColumn`] to index into [`Felt252Dict`].
#[generate_trait]
pub impl PreprocessedColumnKey of PreprocessedColumnKeyTrait {
    fn encode(key: @PreprocessedColumn) -> felt252 {
        const FELT252_2_POW_32: felt252 = 0x100000000;

        const XOR_DISCRIMINANT: felt252 = 0;
        const SEQ_TABLE_DISCRIMINANT: felt252 = 1;
        const RANGE_CHECK_2_DISCRIMINANT: felt252 = 2;
        const RANGE_CHECK_3_DISCRIMINANT: felt252 = 3;
        const RANGE_CHECK_4_DISCRIMINANT: felt252 = 4;
        const RANGE_CHECK_5_DISCRIMINANT: felt252 = 5;
        const POSEIDON_ROUND_KEYS_DISCRIMINANT: felt252 = 6;
        const BLAKE_SIGMA_DISCRIMINANT: felt252 = 7;
        const PEDERSEN_POINTS_DISCRIMINANT: felt252 = 8;

        match key {
            PreprocessedColumn::BitwiseXor((
                n_term_bits, term,
            )) => {
                let mut res = (*term).into();
                res = res * FELT252_2_POW_32 + (*n_term_bits).into();
                res = res * FELT252_2_POW_32 + XOR_DISCRIMINANT;
                res
            },
            PreprocessedColumn::Seq(log_size) => {
                let mut res = (*log_size).into();
                res = res * FELT252_2_POW_32 + SEQ_TABLE_DISCRIMINANT;
                res
            },
            PreprocessedColumn::RangeCheck5((
                values, column_index,
            )) => range_check_encode(values, *column_index, RANGE_CHECK_5_DISCRIMINANT),
            PreprocessedColumn::RangeCheck4((
                values, column_index,
            )) => range_check_encode(values, *column_index, RANGE_CHECK_4_DISCRIMINANT),
            PreprocessedColumn::RangeCheck3((
                values, column_index,
            )) => range_check_encode(values, *column_index, RANGE_CHECK_3_DISCRIMINANT),
            PreprocessedColumn::RangeCheck2((
                values, column_index,
            )) => range_check_encode(values, *column_index, RANGE_CHECK_2_DISCRIMINANT),
            PreprocessedColumn::PoseidonRoundKeys(column_index) => {
                let mut res = (*column_index).into();
                res = res * FELT252_2_POW_32 + POSEIDON_ROUND_KEYS_DISCRIMINANT;
                res
            },
            PreprocessedColumn::BlakeSigma(column_index) => {
                let mut res = (*column_index).into();
                res = res * FELT252_2_POW_32 + BLAKE_SIGMA_DISCRIMINANT;
                res
            },
            PreprocessedColumn::PedersenPoints(column_index) => {
                let mut res = (*column_index).into();
                res = res * FELT252_2_POW_32 + PEDERSEN_POINTS_DISCRIMINANT;
                res
            },
        }
    }
}

pub fn range_check_encode<const N: usize, impl IntoSpan: ToSpanTrait<[u32; N], u32>>(
    n_bits_vec: @[u32; N], column_index: usize, discriminant: felt252,
) -> felt252 {
    const FELT252_2_POW_32: felt252 = 0x100000000;

    let mut total: felt252 = column_index.into();
    for n_bits in IntoSpan::span(n_bits_vec) {
        total = total * FELT252_2_POW_32 + (*n_bits).into();
    }
    total = total * FELT252_2_POW_32 + discriminant;
    total
}

#[cfg(test)]
mod tests {
    use super::{PreprocessedColumn, PreprocessedColumnSet, PreprocessedColumnSetImpl};

    #[test]
    fn test_preprocessed_column_set() {
        let mut set: PreprocessedColumnSet = Default::default();
        let seq_16_column = PreprocessedColumn::Seq(16);
        let seq_10_column = PreprocessedColumn::Seq(10);

        set.insert(seq_16_column);
        set.insert(seq_16_column);

        assert!(set.contains(seq_16_column));
        assert!(!set.contains(seq_10_column));
        assert_eq!(set.values, array![seq_16_column]);
    }
}
