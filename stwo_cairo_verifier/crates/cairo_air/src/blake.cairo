use components::blake_g::InteractionClaimImpl as BlakeGInteractionClaimImpl;
use components::blake_round::InteractionClaimImpl as BlakeRoundInteractionClaimImpl;
use components::blake_round_sigma::InteractionClaimImpl as BlakeRoundSigmaInteractionClaimImpl;
use components::triple_xor_32::InteractionClaimImpl as TripleXor32InteractionClaimImpl;
use components::verify_bitwise_xor_12::InteractionClaimImpl as VerifyBitwiseXor12InteractionClaimImpl;
use core::array::Span;
use stwo_cairo_air::cairo_component::CairoComponent;
use stwo_cairo_air::claim::ClaimTrait;
use stwo_cairo_air::claims::CairoClaim;
use stwo_cairo_air::components;
use stwo_constraint_framework::{
    CommonLookupElements, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::utils::OptionImpl;
use stwo_verifier_core::{ColumnSpan, TreeArray};
use crate::utils;

pub fn blake_context_log_sizes(claim: @CairoClaim) -> TreeArray<Span<u32>> {
    if let Some(_) = claim.blake_round {
        return utils::tree_array_concat_cols(
            array![
                claim.blake_round.unwrap().log_sizes(), claim.blake_g.unwrap().log_sizes(),
                claim.blake_round_sigma.unwrap().log_sizes(),
                claim.triple_xor_32.unwrap().log_sizes(),
                claim.verify_bitwise_xor_12.unwrap().log_sizes(),
            ],
        );
    } else {
        return utils::tree_array_concat_cols(array![]);
    };
}

#[derive(Drop)]
pub struct BlakeContextComponents {
    components: Option<BlakeComponents>,
}

#[generate_trait]
pub impl BlakeContextComponentsImpl of BlakeContextComponentsTrait {
    fn new(
        blake_round_claim: @Option<components::blake_round::Claim>,
        blake_g_claim: @Option<components::blake_g::Claim>,
        blake_round_sigma_claim: @Option<components::blake_round_sigma::Claim>,
        triple_xor_32_claim: @Option<components::triple_xor_32::Claim>,
        verify_bitwise_xor_12_claim: @Option<components::verify_bitwise_xor_12::Claim>,
        common_lookup_elements: @CommonLookupElements,
        blake_round_interaction_claim: @Option<components::blake_round::InteractionClaim>,
        blake_g_interaction_claim: @Option<components::blake_g::InteractionClaim>,
        blake_round_sigma_interaction_claim: @Option<
            components::blake_round_sigma::InteractionClaim,
        >,
        triple_xor_32_interaction_claim: @Option<components::triple_xor_32::InteractionClaim>,
        verify_bitwise_xor_12_interaction_claim: @Option<
            components::verify_bitwise_xor_12::InteractionClaim,
        >,
    ) -> BlakeContextComponents {
        if let Some(_) = blake_round_claim {
            let components = BlakeComponentsImpl::new(
                blake_round_claim,
                blake_g_claim,
                blake_round_sigma_claim,
                triple_xor_32_claim,
                verify_bitwise_xor_12_claim,
                common_lookup_elements,
                blake_round_interaction_claim,
                blake_g_interaction_claim,
                blake_round_sigma_interaction_claim,
                triple_xor_32_interaction_claim,
                verify_bitwise_xor_12_interaction_claim,
            );

            BlakeContextComponents { components: Some(components) }
        } else {
            assert!(blake_g_claim.is_none());
            assert!(blake_round_sigma_claim.is_none());
            assert!(triple_xor_32_claim.is_none());
            assert!(verify_bitwise_xor_12_claim.is_none());
            assert!(blake_round_interaction_claim.is_none());
            assert!(blake_g_interaction_claim.is_none());
            assert!(blake_round_sigma_interaction_claim.is_none());
            assert!(triple_xor_32_interaction_claim.is_none());
            assert!(verify_bitwise_xor_12_interaction_claim.is_none());
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
        blake_round_claim: @Option<components::blake_round::Claim>,
        blake_g_claim: @Option<components::blake_g::Claim>,
        blake_round_sigma_claim: @Option<components::blake_round_sigma::Claim>,
        triple_xor_32_claim: @Option<components::triple_xor_32::Claim>,
        verify_bitwise_xor_12_claim: @Option<components::verify_bitwise_xor_12::Claim>,
        common_lookup_elements: @CommonLookupElements,
        blake_round_interaction_claim: @Option<components::blake_round::InteractionClaim>,
        blake_g_interaction_claim: @Option<components::blake_g::InteractionClaim>,
        blake_round_sigma_interaction_claim: @Option<
            components::blake_round_sigma::InteractionClaim,
        >,
        triple_xor_32_interaction_claim: @Option<components::triple_xor_32::InteractionClaim>,
        verify_bitwise_xor_12_interaction_claim: @Option<
            components::verify_bitwise_xor_12::InteractionClaim,
        >,
    ) -> BlakeComponents {
        let blake_round = components::blake_round::NewComponentImpl::try_new(
            blake_round_claim, blake_round_interaction_claim, common_lookup_elements,
        )
            .unwrap();

        let blake_g = components::blake_g::NewComponentImpl::try_new(
            blake_g_claim, blake_g_interaction_claim, common_lookup_elements,
        )
            .unwrap();

        let blake_round_sigma = components::blake_round_sigma::NewComponentImpl::try_new(
            blake_round_sigma_claim, blake_round_sigma_interaction_claim, common_lookup_elements,
        )
            .unwrap();

        let triple_xor_32 = components::triple_xor_32::NewComponentImpl::try_new(
            triple_xor_32_claim, triple_xor_32_interaction_claim, common_lookup_elements,
        )
            .unwrap();

        let verify_bitwise_xor_12 = components::verify_bitwise_xor_12::NewComponentImpl::try_new(
            verify_bitwise_xor_12_claim,
            verify_bitwise_xor_12_interaction_claim,
            common_lookup_elements,
        )
            .unwrap();

        BlakeComponents {
            blake_round, blake_g, blake_round_sigma, triple_xor_32, verify_bitwise_xor_12,
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

