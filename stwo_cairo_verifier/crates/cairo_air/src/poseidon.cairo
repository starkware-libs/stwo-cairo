use components::cube_252::InteractionClaimImpl as Cube252InteractionClaimImpl;
use components::poseidon_3_partial_rounds_chain::InteractionClaimImpl as Poseidon3PartialRoundsChainInteractionClaimImpl;
use components::poseidon_aggregator::InteractionClaimImpl as PoseidonAggregatorInteractionClaimImpl;
use components::poseidon_builtin::InteractionClaimImpl as PoseidonBuiltinInteractionClaimImpl;
use components::poseidon_full_round_chain::InteractionClaimImpl as PoseidonFullRoundChainInteractionClaimImpl;
use components::poseidon_round_keys::InteractionClaimImpl as PoseidonRoundKeysInteractionClaimImpl;
use components::range_check_252_width_27::InteractionClaimImpl as RangeCheckFelt252Width27InteractionClaimImpl;
use core::array::Span;
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
use stwo_cairo_air::cairo_component::CairoComponent;
use stwo_cairo_air::claim::ClaimTrait;
use stwo_cairo_air::claims::CairoClaim;
use stwo_cairo_air::components;
use stwo_constraint_framework::PreprocessedMaskValuesImpl;
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
use stwo_constraint_framework::{CommonLookupElements, PreprocessedMaskValues};
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
use stwo_verifier_core::ColumnSpan;
use stwo_verifier_core::TreeArray;
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::utils::OptionImpl;
use crate::utils;

pub fn poseidon_context_log_sizes(claim: @CairoClaim) -> TreeArray<Span<u32>> {
    if let Some(_) = claim.poseidon_aggregator {
        utils::tree_array_concat_cols(
            array![
                claim.poseidon_aggregator.unwrap().log_sizes(),
                claim.poseidon_3_partial_rounds_chain.unwrap().log_sizes(),
                claim.poseidon_full_round_chain.unwrap().log_sizes(),
                claim.cube_252.unwrap().log_sizes(), claim.poseidon_round_keys.unwrap().log_sizes(),
                claim.range_check_252_width_27.unwrap().log_sizes(),
            ],
        )
    } else {
        array![]
    }
}

#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
#[derive(Drop)]
pub struct PoseidonContextComponents {
    components: Option<PoseidonComponents>,
}

#[generate_trait]
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
pub impl PoseidonContextComponentsImpl of PoseidonContextComponentsTrait {
    fn new(
        poseidon_aggregator_claim: @Option<components::poseidon_aggregator::Claim>,
        poseidon_3_partial_rounds_chain_claim: @Option<
            components::poseidon_3_partial_rounds_chain::Claim,
        >,
        poseidon_full_round_chain_claim: @Option<components::poseidon_full_round_chain::Claim>,
        cube_252_claim: @Option<components::cube_252::Claim>,
        poseidon_round_keys_claim: @Option<components::poseidon_round_keys::Claim>,
        range_check_252_width_27_claim: @Option<components::range_check_252_width_27::Claim>,
        common_lookup_elements: @CommonLookupElements,
        poseidon_aggregator_interaction_claim: @Option<
            components::poseidon_aggregator::InteractionClaim,
        >,
        poseidon_3_partial_rounds_chain_interaction_claim: @Option<
            components::poseidon_3_partial_rounds_chain::InteractionClaim,
        >,
        poseidon_full_round_chain_interaction_claim: @Option<
            components::poseidon_full_round_chain::InteractionClaim,
        >,
        cube_252_interaction_claim: @Option<components::cube_252::InteractionClaim>,
        poseidon_round_keys_interaction_claim: @Option<
            components::poseidon_round_keys::InteractionClaim,
        >,
        range_check_252_width_27_interaction_claim: @Option<
            components::range_check_252_width_27::InteractionClaim,
        >,
    ) -> PoseidonContextComponents {
        if let Some(_) = poseidon_aggregator_claim {
            PoseidonContextComponents {
                components: Some(
                    PoseidonComponentsImpl::new(
                        poseidon_aggregator_claim,
                        poseidon_3_partial_rounds_chain_claim,
                        poseidon_full_round_chain_claim,
                        cube_252_claim,
                        poseidon_round_keys_claim,
                        range_check_252_width_27_claim,
                        common_lookup_elements,
                        poseidon_aggregator_interaction_claim,
                        poseidon_3_partial_rounds_chain_interaction_claim,
                        poseidon_full_round_chain_interaction_claim,
                        cube_252_interaction_claim,
                        poseidon_round_keys_interaction_claim,
                        range_check_252_width_27_interaction_claim,
                    ),
                ),
            }
        } else {
            assert!(poseidon_3_partial_rounds_chain_claim.is_none());
            assert!(poseidon_full_round_chain_claim.is_none());
            assert!(cube_252_claim.is_none());
            assert!(poseidon_round_keys_claim.is_none());
            assert!(range_check_252_width_27_claim.is_none());
            assert!(poseidon_aggregator_interaction_claim.is_none());
            assert!(poseidon_3_partial_rounds_chain_interaction_claim.is_none());
            assert!(poseidon_full_round_chain_interaction_claim.is_none());
            assert!(cube_252_interaction_claim.is_none());
            assert!(poseidon_round_keys_interaction_claim.is_none());
            assert!(range_check_252_width_27_interaction_claim.is_none());
            PoseidonContextComponents { components: None }
        }
    }

    fn evaluate_constraints_at_point(
        self: @PoseidonContextComponents,
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

#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
#[derive(Drop)]
pub struct PoseidonComponents {
    pub poseidon_aggregator: components::poseidon_aggregator::Component,
    pub poseidon_3_partial_rounds_chain: components::poseidon_3_partial_rounds_chain::Component,
    pub poseidon_full_round_chain: components::poseidon_full_round_chain::Component,
    pub cube_252: components::cube_252::Component,
    pub poseidon_round_keys: components::poseidon_round_keys::Component,
    pub range_check_252_width_27: components::range_check_252_width_27::Component,
}

#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
#[generate_trait]
pub impl PoseidonComponentsImpl of PoseidonComponentsTrait {
    fn new(
        poseidon_aggregator_claim: @Option<components::poseidon_aggregator::Claim>,
        poseidon_3_partial_rounds_chain_claim: @Option<
            components::poseidon_3_partial_rounds_chain::Claim,
        >,
        poseidon_full_round_chain_claim: @Option<components::poseidon_full_round_chain::Claim>,
        cube_252_claim: @Option<components::cube_252::Claim>,
        poseidon_round_keys_claim: @Option<components::poseidon_round_keys::Claim>,
        range_check_252_width_27_claim: @Option<components::range_check_252_width_27::Claim>,
        common_lookup_elements: @CommonLookupElements,
        poseidon_aggregator_interaction_claim: @Option<
            components::poseidon_aggregator::InteractionClaim,
        >,
        poseidon_3_partial_rounds_chain_interaction_claim: @Option<
            components::poseidon_3_partial_rounds_chain::InteractionClaim,
        >,
        poseidon_full_round_chain_interaction_claim: @Option<
            components::poseidon_full_round_chain::InteractionClaim,
        >,
        cube_252_interaction_claim: @Option<components::cube_252::InteractionClaim>,
        poseidon_round_keys_interaction_claim: @Option<
            components::poseidon_round_keys::InteractionClaim,
        >,
        range_check_252_width_27_interaction_claim: @Option<
            components::range_check_252_width_27::InteractionClaim,
        >,
    ) -> PoseidonComponents {
        let poseidon_aggregator_component =
            components::poseidon_aggregator::NewComponentImpl::try_new(
            poseidon_aggregator_claim,
            poseidon_aggregator_interaction_claim,
            common_lookup_elements,
        )
            .unwrap();

        let poseidon_3_partial_rounds_chain_component =
            components::poseidon_3_partial_rounds_chain::NewComponentImpl::try_new(
            poseidon_3_partial_rounds_chain_claim,
            poseidon_3_partial_rounds_chain_interaction_claim,
            common_lookup_elements,
        )
            .unwrap();

        let poseidon_full_round_chain_component =
            components::poseidon_full_round_chain::NewComponentImpl::try_new(
            poseidon_full_round_chain_claim,
            poseidon_full_round_chain_interaction_claim,
            common_lookup_elements,
        )
            .unwrap();

        let cube_252_component = components::cube_252::NewComponentImpl::try_new(
            cube_252_claim, cube_252_interaction_claim, common_lookup_elements,
        )
            .unwrap();

        let poseidon_round_keys_component =
            components::poseidon_round_keys::NewComponentImpl::try_new(
            poseidon_round_keys_claim,
            poseidon_round_keys_interaction_claim,
            common_lookup_elements,
        )
            .unwrap();

        let range_check_felt_252_width_27_component =
            components::range_check_252_width_27::NewComponentImpl::try_new(
            range_check_252_width_27_claim,
            range_check_252_width_27_interaction_claim,
            common_lookup_elements,
        )
            .unwrap();

        PoseidonComponents {
            poseidon_aggregator: poseidon_aggregator_component,
            poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain_component,
            poseidon_full_round_chain: poseidon_full_round_chain_component,
            cube_252: cube_252_component,
            poseidon_round_keys: poseidon_round_keys_component,
            range_check_252_width_27: range_check_felt_252_width_27_component,
        }
    }

    fn evaluate_constraints_at_point(
        self: @PoseidonComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
    ) {
        self
            .poseidon_aggregator
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        self
            .poseidon_3_partial_rounds_chain
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        self
            .poseidon_full_round_chain
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        self
            .cube_252
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        self
            .poseidon_round_keys
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        self
            .range_check_252_width_27
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
    }
}
