use crate::prelude::*;

mod constraints;

pub const ELEM_BITS: u32 = 12;

pub const EXPAND_BITS: u32 = 2;

pub const LOG_SIZE: u32 = (ELEM_BITS - EXPAND_BITS) * 2;

pub const N_TRACE_COLUMNS: usize = 16;

pub const N_INTERACTION_COLUMNS: usize = QM31_EXTENSION_DEGREE * 8;


#[derive(Drop)]
pub struct Component {
    pub claimed_sum: QM31,
    pub common_lookup_elements: CommonLookupElements,
}

pub impl NewComponentImpl of NewComponent<Component> {
    fn new(
        log_size: @u32, claimed_sum: @QM31, common_lookup_elements: @CommonLookupElements,
    ) -> Component {
        Component {
            claimed_sum: *claimed_sum, common_lookup_elements: common_lookup_elements.clone(),
        }
    }
}

pub impl AirComponentImpl of AirComponent<Component> {
    fn evaluate_constraints_at_point(
        self: @Component,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        public_params: Span<u32>,
    ) {
        let claimed_sum = *self.claimed_sum;

        let params = constraints::ConstraintParams {
            lookup_elements: self.common_lookup_elements,
            claimed_sum,
            bitwise_xor_10_0: preprocessed_mask_values
                .get_and_mark_used(preprocessed_columns::BITWISE_XOR_10_0_IDX),
            bitwise_xor_10_1: preprocessed_mask_values
                .get_and_mark_used(preprocessed_columns::BITWISE_XOR_10_1_IDX),
            bitwise_xor_10_2: preprocessed_mask_values
                .get_and_mark_used(preprocessed_columns::BITWISE_XOR_10_2_IDX),
            column_size: pow2(LOG_SIZE).try_into().unwrap(),
        };

        constraints::evaluate_constraints_at_point(
            ref sum, ref trace_mask_values, ref interaction_trace_mask_values, params, random_coeff,
        );
    }
}
