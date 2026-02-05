use components::blake_g::InteractionClaimImpl as BlakeGInteractionClaimImpl;
use components::blake_round::InteractionClaimImpl as BlakeRoundInteractionClaimImpl;
use components::blake_round_sigma::InteractionClaimImpl as BlakeRoundSigmaInteractionClaimImpl;
use components::triple_xor_32::InteractionClaimImpl as TripleXor32InteractionClaimImpl;
use components::verify_bitwise_xor_12::InteractionClaimImpl as VerifyBitwiseXor12InteractionClaimImpl;
use core::array::Span;
use stwo_cairo_air::cairo_component::CairoComponent;
use stwo_cairo_air::claims::{CairoClaim, CairoInteractionClaim};
use stwo_cairo_air::components;
use stwo_constraint_framework::{
    CommonLookupElements, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::ColumnSpan;
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::utils::OptionImpl;
#[derive(Drop)]
pub struct BlakeContextComponents {
    components: Option<BlakeComponents>,
}

#[generate_trait]
pub impl BlakeContextComponentsImpl of BlakeContextComponentsTrait {
    fn new(
        cairo_claim: @CairoClaim,
        common_lookup_elements: @CommonLookupElements,
        interaction_claim: @CairoInteractionClaim,
    ) -> BlakeContextComponents {
        if let Some(_) = cairo_claim.blake_round {
            BlakeContextComponents {
                components: Some(
                    BlakeComponentsImpl::new(
                        cairo_claim, common_lookup_elements, interaction_claim,
                    ),
                ),
            }
        } else {
            assert!(cairo_claim.blake_g.is_none());
            assert!(cairo_claim.blake_round_sigma.is_none());
            assert!(cairo_claim.triple_xor_32.is_none());
            assert!(cairo_claim.verify_bitwise_xor_12.is_none());
            assert!(interaction_claim.blake_round.is_none());
            assert!(interaction_claim.blake_g.is_none());
            assert!(interaction_claim.blake_round_sigma.is_none());
            assert!(interaction_claim.triple_xor_32.is_none());
            assert!(interaction_claim.verify_bitwise_xor_12.is_none());
            BlakeContextComponents { components: None }
        }
    }

    fn evaluate_constraints_at_point(
        self: @BlakeContextComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
    ) {
        if let Option::Some(components) = self.components {
            components
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }
    }
}

#[derive(Drop)]
pub struct BlakeComponents {
    pub blake_round: components::blake_round::Component,
    pub blake_g: components::blake_g::Component,
    pub blake_round_sigma: components::blake_round_sigma::Component,
    pub triple_xor_32: components::triple_xor_32::Component,
    pub verify_bitwise_xor_12: components::verify_bitwise_xor_12::Component,
}

#[generate_trait]
pub impl BlakeComponentsImpl of BlakeComponentsTrait {
    fn new(
        cairo_claim: @CairoClaim,
        common_lookup_elements: @CommonLookupElements,
        interaction_claim: @CairoInteractionClaim,
    ) -> BlakeComponents {
        let blake_round_component = components::blake_round::NewComponentImpl::try_new(
            cairo_claim.blake_round, interaction_claim.blake_round, common_lookup_elements,
        )
            .unwrap();

        let blake_g_component = components::blake_g::NewComponentImpl::try_new(
            cairo_claim.blake_g, interaction_claim.blake_g, common_lookup_elements,
        )
            .unwrap();

        let blake_round_sigma_component = components::blake_round_sigma::NewComponentImpl::try_new(
            cairo_claim.blake_round_sigma,
            interaction_claim.blake_round_sigma,
            common_lookup_elements,
        )
            .unwrap();

        let triple_xor_32_component = components::triple_xor_32::NewComponentImpl::try_new(
            cairo_claim.triple_xor_32, interaction_claim.triple_xor_32, common_lookup_elements,
        )
            .unwrap();

        let verify_bitwise_xor_12_component =
            components::verify_bitwise_xor_12::NewComponentImpl::try_new(
            cairo_claim.verify_bitwise_xor_12,
            interaction_claim.verify_bitwise_xor_12,
            common_lookup_elements,
        )
            .unwrap();

        BlakeComponents {
            blake_round: blake_round_component,
            blake_g: blake_g_component,
            blake_round_sigma: blake_round_sigma_component,
            triple_xor_32: triple_xor_32_component,
            verify_bitwise_xor_12: verify_bitwise_xor_12_component,
        }
    }

    fn evaluate_constraints_at_point(
        self: @BlakeComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
    ) {
        self
            .blake_round
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        self
            .blake_g
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        self
            .blake_round_sigma
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        self
            .triple_xor_32
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        self
            .verify_bitwise_xor_12
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
    }
}
