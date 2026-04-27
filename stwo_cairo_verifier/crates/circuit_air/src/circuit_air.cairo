use core::box::BoxImpl;
use core::num::traits::Zero;
use stwo_constraint_framework::{
    CommonLookupElements, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::verifier::Air;
use stwo_verifier_core::{ColumnSpan, TreeSpan};
use crate::circuit_component::CircuitComponent;
use crate::claims::{CircuitClaim, CircuitInteractionClaim};
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
    pub eq: components::eq::Component,
    pub qm31_ops: components::qm31_ops::Component,
    pub blake_gate: components::blake_gate::Component,
    pub blake_round: components::blake_round::Component,
    pub blake_round_sigma: components::blake_round_sigma::Component,
    pub blake_g: components::blake_g::Component,
    pub blake_output: components::blake_output::Component,
    pub triple_xor_32: components::triple_xor_32::Component,
    pub m_31_to_u_32: components::m_31_to_u_32::Component,
    pub verify_bitwise_xor_8: components::verify_bitwise_xor_8::Component,
    pub verify_bitwise_xor_12: components::verify_bitwise_xor_12::Component,
    pub verify_bitwise_xor_4: components::verify_bitwise_xor_4::Component,
    pub verify_bitwise_xor_7: components::verify_bitwise_xor_7::Component,
    pub verify_bitwise_xor_9: components::verify_bitwise_xor_9::Component,
    pub range_check_15: components::range_check_15::Component,
    pub range_check_16: components::range_check_16::Component,
}

#[generate_trait]
pub impl CircuitAirNewImpl of CircuitAirNewTrait {
    fn new(
        circuit_claim: @CircuitClaim,
        common_lookup_elements: @CommonLookupElements,
        interaction_claim: @CircuitInteractionClaim,
    ) -> CircuitAir {
        CircuitAir {
            eq: components::eq::NewComponentImpl::new(
                circuit_claim.eq, interaction_claim.eq, common_lookup_elements,
            ),
            qm31_ops: components::qm31_ops::NewComponentImpl::new(
                circuit_claim.qm31_ops, interaction_claim.qm31_ops, common_lookup_elements,
            ),
            blake_gate: components::blake_gate::NewComponentImpl::new(
                circuit_claim.blake_gate, interaction_claim.blake_gate, common_lookup_elements,
            ),
            blake_round: components::blake_round::NewComponentImpl::new(
                circuit_claim.blake_round, interaction_claim.blake_round, common_lookup_elements,
            ),
            blake_round_sigma: components::blake_round_sigma::NewComponentImpl::new(
                circuit_claim.blake_round_sigma,
                interaction_claim.blake_round_sigma,
                common_lookup_elements,
            ),
            blake_g: components::blake_g::NewComponentImpl::new(
                circuit_claim.blake_g, interaction_claim.blake_g, common_lookup_elements,
            ),
            blake_output: components::blake_output::NewComponentImpl::new(
                circuit_claim.blake_output, interaction_claim.blake_output, common_lookup_elements,
            ),
            triple_xor_32: components::triple_xor_32::NewComponentImpl::new(
                circuit_claim.triple_xor_32,
                interaction_claim.triple_xor_32,
                common_lookup_elements,
            ),
            m_31_to_u_32: components::m_31_to_u_32::NewComponentImpl::new(
                circuit_claim.m_31_to_u_32, interaction_claim.m_31_to_u_32, common_lookup_elements,
            ),
            verify_bitwise_xor_8: components::verify_bitwise_xor_8::NewComponentImpl::new(
                circuit_claim.verify_bitwise_xor_8,
                interaction_claim.verify_bitwise_xor_8,
                common_lookup_elements,
            ),
            verify_bitwise_xor_12: components::verify_bitwise_xor_12::NewComponentImpl::new(
                circuit_claim.verify_bitwise_xor_12,
                interaction_claim.verify_bitwise_xor_12,
                common_lookup_elements,
            ),
            verify_bitwise_xor_4: components::verify_bitwise_xor_4::NewComponentImpl::new(
                circuit_claim.verify_bitwise_xor_4,
                interaction_claim.verify_bitwise_xor_4,
                common_lookup_elements,
            ),
            verify_bitwise_xor_7: components::verify_bitwise_xor_7::NewComponentImpl::new(
                circuit_claim.verify_bitwise_xor_7,
                interaction_claim.verify_bitwise_xor_7,
                common_lookup_elements,
            ),
            verify_bitwise_xor_9: components::verify_bitwise_xor_9::NewComponentImpl::new(
                circuit_claim.verify_bitwise_xor_9,
                interaction_claim.verify_bitwise_xor_9,
                common_lookup_elements,
            ),
            range_check_15: components::range_check_15::NewComponentImpl::new(
                circuit_claim.range_check_15,
                interaction_claim.range_check_15,
                common_lookup_elements,
            ),
            range_check_16: components::range_check_16::NewComponentImpl::new(
                circuit_claim.range_check_16,
                interaction_claim.range_check_16,
                common_lookup_elements,
            ),
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
        ]: [ColumnSpan<Span<QM31>>; 4] =
            (*mask_values
            .try_into()
            .unwrap())
            .unbox();

        let mut preprocessed_mask_values = PreprocessedMaskValuesImpl::new(
            preprocessed_mask_values,
        );

        let CircuitAir {
            eq,
            qm31_ops,
            blake_gate,
            blake_round,
            blake_round_sigma,
            blake_g,
            blake_output,
            triple_xor_32,
            m_31_to_u_32,
            verify_bitwise_xor_8,
            verify_bitwise_xor_12,
            verify_bitwise_xor_4,
            verify_bitwise_xor_7,
            verify_bitwise_xor_9,
            range_check_15,
            range_check_16,
        } = self;

        eq
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        qm31_ops
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        blake_gate
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        blake_round
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        blake_round_sigma
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        blake_g
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        blake_output
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        triple_xor_32
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        m_31_to_u_32
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        verify_bitwise_xor_8
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        verify_bitwise_xor_12
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        verify_bitwise_xor_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        verify_bitwise_xor_7
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        verify_bitwise_xor_9
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        range_check_15
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        range_check_16
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );

        validate_mask_usage(
            preprocessed_mask_values, trace_mask_values, interaction_trace_mask_values,
        );
        sum
    }
}
