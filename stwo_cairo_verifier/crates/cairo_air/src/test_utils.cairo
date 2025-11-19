use core::num::traits::Zero;
use stwo_constraint_framework::{LookupElementsTrait, PreprocessedColumnIdx, PreprocessedMaskValues};
use stwo_verifier_core::ColumnSpan;
use stwo_verifier_core::fields::m31::M31;
use stwo_verifier_core::fields::qm31::{QM31, QM31Impl};
use stwo_verifier_core::utils::ArrayImpl;
use super::{
    LookupElements, MemorySmallValue, PublicDataImpl, PublicMemory, PublicSegmentRanges,
    SegmentRange,
};

/// Generates a public memory where only the output segment is populated with
/// `output_len` elements, each being a tuple of (id, value).
pub fn mock_public_memory_with_outputs(output_len: u32) -> PublicMemory {
    let mut output = array![];
    for i in 0..output_len {
        output.append((i, [i; 8]));
    }

    let empty_segment = SegmentRange {
        start_ptr: MemorySmallValue { id: 0, value: 0 },
        stop_ptr: MemorySmallValue { id: 0, value: 0 },
    };

    PublicMemory {
        program: [].span(),
        public_segments: PublicSegmentRanges {
            output: empty_segment,
            pedersen: empty_segment,
            range_check_128: empty_segment,
            ecdsa: empty_segment,
            bitwise: empty_segment,
            ec_op: empty_segment,
            keccak: empty_segment,
            poseidon: empty_segment,
            range_check_96: empty_segment,
            add_mod: empty_segment,
            mul_mod: empty_segment,
        },
        output: output.span(),
        safe_call_ids: [1, 2],
    }
}

pub fn make_lookup_elements(z: QM31, alpha: QM31) -> LookupElements {
    LookupElementsTrait::from_z_alpha(z, alpha)
}

fn qm31_from_m31(m31: M31) -> QM31 {
    QM31Impl::from_fixed_array([m31, Zero::zero(), Zero::zero(), Zero::zero()])
}

pub fn make_interaction_trace(values: Array<QM31>, last_row_sum: QM31) -> ColumnSpan<Span<QM31>> {
    let last_row_sum_parts = last_row_sum.to_fixed_array().span();
    let mut result = array![];

    for i in 0..values.len() {
        let value_parts = values[i].to_fixed_array().span();
        let columns: Array<_> = if i < values.len() - 1 {
            (0..4_u32).into_iter().map(|j| [qm31_from_m31(*value_parts[j])].span()).collect()
        } else {
            (0..4_u32)
                .into_iter()
                .map(
                    |j| [qm31_from_m31(*last_row_sum_parts[j]), qm31_from_m31(*value_parts[j])]
                        .span(),
                )
                .collect()
        };
        result.append_span(columns.span());
    }
    result.span()
}

pub fn preprocessed_mask_add(
    mut mask_values: PreprocessedMaskValues, column_idx: PreprocessedColumnIdx, value: QM31,
) -> PreprocessedMaskValues {
    mask_values.values.insert(column_idx.into(), NullableTrait::new((value, false)));
    mask_values
}
