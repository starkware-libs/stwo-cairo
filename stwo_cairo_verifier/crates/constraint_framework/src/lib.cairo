use core::dict::{Felt252Dict, Felt252DictTrait};
use core::nullable::{Nullable, NullableTrait};
use core::num::traits::One;
use stwo_verifier_core::ColumnSpan;
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::fields::m31::{M31, MulByM31Trait};
use stwo_verifier_core::fields::qm31::QM31;

#[cfg(test)]
mod test;

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


pub trait LookupElementsTrait<const N: usize> {
    fn draw(
        ref channel: Channel,
    ) -> LookupElements<
        N,
    > {
        assert!(N != 0);
        let [z, alpha]: [QM31; 2] = (*channel.draw_secure_felts(2).span().try_into().unwrap())
            .unbox();

        let mut acc = One::one();
        let mut alpha_powers = array![acc];

        for _ in 1..N {
            acc *= alpha;
            alpha_powers.append(acc);
        }

        LookupElements { z, alpha, alpha_powers }
    }


    /// Computes \sigma_i = -z + values[i] * self.alpha^i where values[i] is in qm31.
    ///
    /// We use horner evaluation here regardless of the qm31_opcode feature flag as it is faster in
    /// both cases.
    fn combine_qm31<impl IntoSpan: ToSpanTrait<[QM31; N], QM31>>(
        self: @LookupElements<N>, values: [QM31; N],
    ) -> QM31 {
        let alpha = *self.alpha;
        let mut values_span = IntoSpan::span(@values);
        let mut sum = *values_span.pop_back().unwrap();

        while let Some(value) = values_span.pop_back() {
            sum = sum * alpha + *value;
        }

        sum - *self.z
    }

    /// Computes \sigma_i = -z + values[i] * self.alpha^i where values[i] is in m31.
    ///
    /// The implementation varies based on the qm31_opcode feature flag.
    fn combine<impl IntoSpan: ToSpanTrait<[M31; N], M31>>(
        self: @LookupElements<N>, values: [M31; N],
    ) -> QM31;
}

#[cfg(feature: "qm31_opcode")]
pub impl LookupElementsImpl<const N: usize> of LookupElementsTrait<N> {
    /// With qm31_opcode enabled, qm31 by qm31 multiplication becomes a single opcode, making
    /// Horner's method the more efficient choice.
    fn combine<impl IntoSpan: ToSpanTrait<[M31; N], M31>>(
        self: @LookupElements<N>, values: [M31; N],
    ) -> QM31 {
        let alpha = *self.alpha;
        let mut values_span = IntoSpan::span(@values);
        let mut sum = (*values_span.pop_back().unwrap()).into();

        while let Some(value) = values_span.pop_back() {
            sum = sum * alpha + (*value).into();
        }

        sum - *self.z
    }
}

#[cfg(not(feature: "qm31_opcode"))]
pub impl LookupElementsImpl<const N: usize> of LookupElementsTrait<N> {
    /// Without qm31_opcode, the naive approach using precomputed alpha powers is faster than
    /// Horner's method because it uses qm31 by m31 multiplication instead of qm31 by qm31.
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
}


#[derive(Destruct, Default)]
pub struct PreprocessedColumnSet {
    /// Map indicating if a preprocessed column is already in the set.
    pub contains: Felt252Dict<bool>,
}

#[generate_trait]
pub impl PreprocessedColumnSetImpl of PreprocessedColumnSetTrait {
    fn insert(ref self: PreprocessedColumnSet, idx: PreprocessedColumnIdx) {
        self.contains.insert(idx.into(), true);
    }

    fn contains(ref self: PreprocessedColumnSet, idx: PreprocessedColumnIdx) -> bool {
        self.contains.get(idx.into())
    }
}

#[derive(Destruct)]
pub struct PreprocessedMaskValues {
    pub values: Array<Nullable<QM31>>,
}

#[generate_trait]
pub impl PreprocessedMaskValuesImpl of PreprocessedMaskValuesTrait {
    fn new(mut preprocessed_mask_values: ColumnSpan<Span<QM31>>) -> PreprocessedMaskValues {
        let mut values: Array<Nullable<QM31>> = array![];

        for _i in 0..(preprocessed_mask_values.len()) {
            let mut column_mask_values = *preprocessed_mask_values.pop_front().unwrap();

            if let Some(mask_value) = column_mask_values.pop_front() {
                values.append(NullableTrait::new(*mask_value));

                // Preprocessed columns should have at most one mask item.
                assert!(column_mask_values.is_empty());
            } else {
                values.append(Default::default());
            }
        }

        assert!(preprocessed_mask_values.is_empty());

        PreprocessedMaskValues { values }
    }

    fn get(ref self: PreprocessedMaskValues, idx: PreprocessedColumnIdx) -> QM31 {
        Deref::deref(*self.values.at(idx))
    }
}

#[derive(Debug, Default, Drop)]
enum PreprocessedColumnsAllocationMode {
    #[default]
    Dynamic,
    Static,
}

#[derive(Drop)]
pub struct PreprocessedColumn {
    pub log_size: u32,
}

pub type PreprocessedColumnIdx = u32;

