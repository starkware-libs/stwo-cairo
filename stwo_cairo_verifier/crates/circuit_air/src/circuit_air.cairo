use core::box::BoxImpl;
use core::num::traits::Zero;
use stwo_constraint_framework::{
    AirComponent, CommonLookupElements, PreprocessedMaskValuesImpl, validate_mask_usage,
};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::{QM31, QM31_EXTENSION_DEGREE};
use stwo_verifier_core::verifier::Air;
use stwo_verifier_core::{ColumnSpan, TreeSpan};
use crate::claims::CircuitInteractionClaim;
use crate::component_indices::*;
use crate::components;

/// Circuit components, in `crate::component_indices` order.
#[derive(Drop)]
pub struct CircuitAir {
    pub eq: components::eq::Component,
    pub qm31_ops: components::qm31_ops::Component,
    pub triple_xor: components::triple_xor::Component,
    pub m_31_to_u_32: components::m_31_to_u_32::Component,
    pub blake_g_gate: components::blake_g_gate::Component,
    pub verify_bitwise_xor_8: components::verify_bitwise_xor_8::Component,
    pub verify_bitwise_xor_12: components::verify_bitwise_xor_12::Component,
    pub verify_bitwise_xor_4: components::verify_bitwise_xor_4::Component,
    pub verify_bitwise_xor_7: components::verify_bitwise_xor_7::Component,
    pub verify_bitwise_xor_9: components::verify_bitwise_xor_9::Component,
    pub range_check_16: components::range_check_16::Component,
}

#[generate_trait]
pub impl CircuitAirNewImpl of CircuitAirNewTrait {
    /// Builds the circuit components. Component log sizes are derived verifier-side (they are
    /// not part of the claim); `component_log_sizes` is indexed by COMPONENT_IDX. The circuit
    /// is fixed-size, so every component is present.
    fn new(
        component_log_sizes: [u32; N_COMPONENTS],
        common_lookup_elements: @CommonLookupElements,
        interaction_claim: @CircuitInteractionClaim,
    ) -> CircuitAir {
        let log_sizes = component_log_sizes.span();

        CircuitAir {
            eq: components::eq::NewComponentImpl::new(
                @components::eq::Claim { log_size: *log_sizes.at(EQ_COMPONENT_IDX) },
                interaction_claim.eq,
                common_lookup_elements,
            ),
            qm31_ops: components::qm31_ops::NewComponentImpl::new(
                @components::qm31_ops::Claim { log_size: *log_sizes.at(QM31_OPS_COMPONENT_IDX) },
                interaction_claim.qm31_ops,
                common_lookup_elements,
            ),
            triple_xor: components::triple_xor::NewComponentImpl::new(
                @components::triple_xor::Claim {
                    log_size: *log_sizes.at(TRIPLE_XOR_COMPONENT_IDX),
                },
                interaction_claim.triple_xor,
                common_lookup_elements,
            ),
            m_31_to_u_32: components::m_31_to_u_32::NewComponentImpl::new(
                @components::m_31_to_u_32::Claim {
                    log_size: *log_sizes.at(M_31_TO_U_32_COMPONENT_IDX),
                },
                interaction_claim.m_31_to_u_32,
                common_lookup_elements,
            ),
            blake_g_gate: components::blake_g_gate::NewComponentImpl::new(
                @components::blake_g_gate::Claim {
                    log_size: *log_sizes.at(BLAKE_G_GATE_COMPONENT_IDX),
                },
                interaction_claim.blake_g_gate,
                common_lookup_elements,
            ),
            verify_bitwise_xor_8: components::verify_bitwise_xor_8::NewComponentImpl::new(
                @components::verify_bitwise_xor_8::Claim {},
                interaction_claim.verify_bitwise_xor_8,
                common_lookup_elements,
            ),
            verify_bitwise_xor_12: components::verify_bitwise_xor_12::NewComponentImpl::new(
                @components::verify_bitwise_xor_12::Claim {},
                interaction_claim.verify_bitwise_xor_12,
                common_lookup_elements,
            ),
            verify_bitwise_xor_4: components::verify_bitwise_xor_4::NewComponentImpl::new(
                @components::verify_bitwise_xor_4::Claim {},
                interaction_claim.verify_bitwise_xor_4,
                common_lookup_elements,
            ),
            verify_bitwise_xor_7: components::verify_bitwise_xor_7::NewComponentImpl::new(
                @components::verify_bitwise_xor_7::Claim {},
                interaction_claim.verify_bitwise_xor_7,
                common_lookup_elements,
            ),
            verify_bitwise_xor_9: components::verify_bitwise_xor_9::NewComponentImpl::new(
                @components::verify_bitwise_xor_9::Claim {},
                interaction_claim.verify_bitwise_xor_9,
                common_lookup_elements,
            ),
            range_check_16: components::range_check_16::NewComponentImpl::new(
                @components::range_check_16::Claim {},
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
        ]: [ColumnSpan<Span<QM31>>; QM31_EXTENSION_DEGREE] =
            (*mask_values
            .try_into()
            .unwrap())
            .unbox();

        let mut preprocessed_mask_values = PreprocessedMaskValuesImpl::new(
            preprocessed_mask_values,
        );

        // Evaluate components in `ComponentList` order — this must match the order in which the
        // prover commits trace/interaction columns, since each component consumes its columns
        // from the front of the mask spans.
        let CircuitAir {
            eq,
            qm31_ops,
            triple_xor,
            m_31_to_u_32,
            blake_g_gate,
            verify_bitwise_xor_8,
            verify_bitwise_xor_12,
            verify_bitwise_xor_4,
            verify_bitwise_xor_7,
            verify_bitwise_xor_9,
            range_check_16,
        } = self;

        eq
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        qm31_ops
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
        m_31_to_u_32
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
        verify_bitwise_xor_7
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
        range_check_16
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
