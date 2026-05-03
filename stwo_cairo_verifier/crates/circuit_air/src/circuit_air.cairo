use core::box::BoxImpl;
use core::num::traits::Zero;
use stwo_constraint_framework::{
    AirComponent, CommonLookupElements, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::{QM31, QM31_EXTENSION_DEGREE};
use stwo_verifier_core::verifier::Air;
use stwo_verifier_core::{ColumnSpan, TreeSpan};
use crate::claims::{CircuitClaim, CircuitInteractionClaim};
use crate::component_indices::*;
use crate::components;

/// Validates that every `mask_value` provided in the proof (in `sampled_values`) is used by at
/// least one component. Mirrors the logic in `stwo_cairo_air::cairo_air::validate_mask_usage`.
fn validate_mask_usage(
    preprocessed_mask_values: PreprocessedMaskValues,
    trace_mask_values: ColumnSpan<Span<QM31>>,
    interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
) {
    preprocessed_mask_values.validate_usage();
    assert!(trace_mask_values.is_empty());
    assert!(interaction_trace_mask_values.is_empty());
}

/// Override the preprocessed trace log sizes in an aggregate `TreeArray<Span<u32>>`.
/// The concatenation of per-component preprocessed columns is not the actual preprocessed
/// trace layout — there is one flat preprocessed trace for the whole circuit. Callers
/// supply the real per-column preprocessed log sizes (`preprocessed_column_log_sizes`).
pub fn override_preprocessed_trace_log_sizes(
    aggregated_log_sizes: stwo_verifier_core::TreeArray<Span<u32>>,
    preprocessed_column_log_sizes: Span<u32>,
) -> stwo_verifier_core::TreeArray<Span<u32>> {
    let boxed_triplet: Box<[Span<u32>; 3]> = *aggregated_log_sizes.span().try_into().unwrap();
    let [_invalid_preprocessed_trace_log_sizes, trace_log_sizes, interaction_log_sizes] =
        boxed_triplet
        .unbox();

    array![preprocessed_column_log_sizes, trace_log_sizes, interaction_log_sizes]
}

#[derive(Drop)]
pub struct CircuitAir {
    pub qm31_ops: components::qm31_ops::Component,
    pub verify_bitwise_xor_8: components::verify_bitwise_xor_8::Component,
    pub verify_bitwise_xor_12: components::verify_bitwise_xor_12::Component,
    pub verify_bitwise_xor_4: components::verify_bitwise_xor_4::Component,
    pub verify_bitwise_xor_9: components::verify_bitwise_xor_9::Component,
    pub verify_bitwise_xor_7: components::verify_bitwise_xor_7::Component,
    pub blake_g_gate: components::blake_g_gate::Component,
    pub range_check_16: components::range_check_16::Component,
    pub m_31_to_u_32: components::m_31_to_u_32::Component,
    pub triple_xor: components::triple_xor::Component,
    pub eq: components::eq::Component,
}

#[generate_trait]
pub impl CircuitAirNewImpl of CircuitAirNewTrait {
    fn new(
        circuit_claim: @CircuitClaim,
        common_lookup_elements: @CommonLookupElements,
        interaction_claim: @CircuitInteractionClaim,
    ) -> CircuitAir {
        let log_size_per_component = circuit_claim.component_log_sizes.span();
        let claimed_sum_per_component = interaction_claim.component_claimed_sums.span();

        CircuitAir {
            qm31_ops: components::qm31_ops::NewComponentImpl::try_new(
                log_size_per_component.at(QM31_OPS_COMPONENT_IDX),
                claimed_sum_per_component.at(QM31_OPS_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
            verify_bitwise_xor_8: components::verify_bitwise_xor_8::NewComponentImpl::try_new(
                log_size_per_component.at(VERIFY_BITWISE_XOR_8_COMPONENT_IDX),
                claimed_sum_per_component.at(VERIFY_BITWISE_XOR_8_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
            verify_bitwise_xor_12: components::verify_bitwise_xor_12::NewComponentImpl::try_new(
                log_size_per_component.at(VERIFY_BITWISE_XOR_12_COMPONENT_IDX),
                claimed_sum_per_component.at(VERIFY_BITWISE_XOR_12_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
            verify_bitwise_xor_4: components::verify_bitwise_xor_4::NewComponentImpl::try_new(
                log_size_per_component.at(VERIFY_BITWISE_XOR_4_COMPONENT_IDX),
                claimed_sum_per_component.at(VERIFY_BITWISE_XOR_4_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
            verify_bitwise_xor_9: components::verify_bitwise_xor_9::NewComponentImpl::try_new(
                log_size_per_component.at(VERIFY_BITWISE_XOR_9_COMPONENT_IDX),
                claimed_sum_per_component.at(VERIFY_BITWISE_XOR_9_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
            verify_bitwise_xor_7: components::verify_bitwise_xor_7::NewComponentImpl::try_new(
                log_size_per_component.at(VERIFY_BITWISE_XOR_7_COMPONENT_IDX),
                claimed_sum_per_component.at(VERIFY_BITWISE_XOR_7_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
            blake_g_gate: components::blake_g_gate::NewComponentImpl::try_new(
                log_size_per_component.at(BLAKE_G_GATE_COMPONENT_IDX),
                claimed_sum_per_component.at(BLAKE_G_GATE_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
            range_check_16: components::range_check_16::NewComponentImpl::try_new(
                log_size_per_component.at(RANGE_CHECK_16_COMPONENT_IDX),
                claimed_sum_per_component.at(RANGE_CHECK_16_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
            m_31_to_u_32: components::m_31_to_u_32::NewComponentImpl::try_new(
                log_size_per_component.at(M_31_TO_U_32_COMPONENT_IDX),
                claimed_sum_per_component.at(M_31_TO_U_32_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
            triple_xor: components::triple_xor::NewComponentImpl::try_new(
                log_size_per_component.at(TRIPLE_XOR_COMPONENT_IDX),
                claimed_sum_per_component.at(TRIPLE_XOR_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
            eq: components::eq::NewComponentImpl::try_new(
                log_size_per_component.at(EQ_COMPONENT_IDX),
                claimed_sum_per_component.at(EQ_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
        }
    }
}

pub impl CircuitAirImpl of Air<CircuitAir> {
    fn eval_composition_polynomial_at_point(
        self: @CircuitAir,
        point: CirclePoint<QM31>,
        mask_values: TreeSpan<ColumnSpan<Span<QM31>>>,
        random_coeff: QM31,
    ) -> QM31 {
        let _ = point;
        let mut sum = Zero::zero();

        let [
            preprocessed_mask_values,
            mut trace_mask_values,
            mut interaction_trace_mask_values,
            _composition_trace_mask_values,
        ]: [ColumnSpan<Span<QM31>>; QM31_EXTENSION_DEGREE] =
            (*mask_values
            .try_into()
            .unwrap())
            .unbox();

        let mut preprocessed_mask_values = PreprocessedMaskValuesImpl::new(
            preprocessed_mask_values,
        );

        let CircuitAir {
            qm31_ops,
            verify_bitwise_xor_8,
            verify_bitwise_xor_12,
            verify_bitwise_xor_4,
            verify_bitwise_xor_9,
            verify_bitwise_xor_7,
            blake_g_gate,
            range_check_16,
            m_31_to_u_32,
            triple_xor,
            eq,
        } = self;

        qm31_ops
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        verify_bitwise_xor_8
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        verify_bitwise_xor_12
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        verify_bitwise_xor_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        verify_bitwise_xor_9
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        verify_bitwise_xor_7
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        blake_g_gate
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        range_check_16
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        m_31_to_u_32
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        triple_xor
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        eq
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );

        validate_mask_usage(
            preprocessed_mask_values, trace_mask_values, interaction_trace_mask_values,
        );
        sum
    }
}
