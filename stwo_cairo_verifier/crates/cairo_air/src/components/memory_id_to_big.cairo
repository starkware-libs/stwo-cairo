use stwo_constraint_framework::{AirComponent, PreprocessedMaskValues, PreprocessedMaskValuesImpl};
use stwo_verifier_core::ColumnSpan;
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use crate::prelude::*;
use super::super::utils::UsizeImpl;

mod constraints_big;

/// The smallest ID yielded by the id_to_big component.
pub const LARGE_MEMORY_VALUE_ID_BASE: u32 = 0x40000000; // 2^30.

pub const N_BITS_PER_FELT: usize = 9;

pub const MEMORY_ID_SIZE: usize = 1;

pub const N_M31_IN_FELT252: usize = 28;

pub const N_MULTIPLICITY_COLUMNS: usize = 1;

pub const BIG_MULTIPLICITY_COLUMN_OFFSET: usize = N_M31_IN_FELT252;

pub const BIG_N_COLUMNS: usize = N_M31_IN_FELT252 + N_MULTIPLICITY_COLUMNS;

// QM31_EXTENSION_DEGREE(4) * ((N_M31_IN_FELT252(28).div_ceil(2)(=14) + 1)(=15).div_ceil(2)(=8))
pub const BIG_N_INTERACTION_COLUMNS: usize = 32;

pub const RELATION_USES_PER_ROW_BIG: [(felt252, u32); 8] = [
    ('RangeCheck_9_9', 2), ('RangeCheck_9_9_B', 2), ('RangeCheck_9_9_C', 2),
    ('RangeCheck_9_9_D', 2), ('RangeCheck_9_9_E', 2), ('RangeCheck_9_9_F', 2),
    ('RangeCheck_9_9_G', 1), ('RangeCheck_9_9_H', 1),
];

#[derive(Drop)]
pub struct BigComponent {
    pub log_n_rows: u32,
    pub offset: u32,
    pub claimed_sum: QM31,
    pub common_lookup_elements: CommonLookupElements,
}

#[generate_trait]
pub impl NewBigComponentImpl of NewBigComponent {
    fn new(
        log_n_rows: u32,
        offset: u32,
        claimed_sum: QM31,
        common_lookup_elements: @CommonLookupElements,
    ) -> BigComponent {
        BigComponent {
            log_n_rows: log_n_rows,
            offset: offset,
            claimed_sum: claimed_sum,
            common_lookup_elements: common_lookup_elements.clone(),
        }
    }
}

pub impl CairoBigComponentImpl of AirComponent<BigComponent> {
    fn evaluate_constraints_at_point(
        self: @BigComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        public_params: Span<u32>,
    ) {
        let params = constraints_big::ConstraintParams {
            column_size: pow2(*self.log_n_rows).try_into().unwrap(),
            offset: (*self.offset).try_into().unwrap(),
            common_lookup_elements: self.common_lookup_elements.clone(),
            claimed_sum: *self.claimed_sum,
            seq: preprocessed_mask_values
                .get_and_mark_used(preprocessed_columns::seq_column_idx(*self.log_n_rows)),
        };

        constraints_big::evaluate_constraints_at_point(
            ref sum, ref trace_mask_values, ref interaction_trace_mask_values, params, random_coeff,
        );
    }
}

