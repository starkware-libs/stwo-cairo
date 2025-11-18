use components::cube_252::InteractionClaimImpl as Cube252InteractionClaimImpl;
use components::partial_ec_mul::InteractionClaimImpl as PartialEcMulInteractionClaimImpl;
use components::poseidon_3_partial_rounds_chain::InteractionClaimImpl as Poseidon3PartialRoundsChainInteractionClaimImpl;
use components::poseidon_aggregator::InteractionClaimImpl as PoseidonAggregatorInteractionClaimImpl;
use components::poseidon_builtin::InteractionClaimImpl as PoseidonBuiltinInteractionClaimImpl;
use components::poseidon_full_round_chain::InteractionClaimImpl as PoseidonFullRoundChainInteractionClaimImpl;
use components::poseidon_round_keys::InteractionClaimImpl as PoseidonRoundKeysInteractionClaimImpl;
use components::range_check_252_width_27::InteractionClaimImpl as RangeCheckFelt252Width27InteractionClaimImpl;
use core::box::BoxImpl;
use core::num::traits::Zero;
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
use stwo_cairo_air::CairoInteractionElements;
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
use stwo_cairo_air::cairo_component::CairoComponent;
use stwo_cairo_air::claim::ClaimTrait;
use stwo_cairo_air::{RelationUsesDict, components, utils};
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
use stwo_constraint_framework::PreprocessedMaskValues;
use stwo_constraint_framework::{LookupElementsImpl, PreprocessedMaskValuesImpl};
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
use stwo_verifier_core::ColumnSpan;
use stwo_verifier_core::TreeArray;
use stwo_verifier_core::channel::Channel;
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::utils::{ArrayImpl, OptionImpl};

#[derive(Drop, Serde)]
pub struct PoseidonClaim {
    pub poseidon_aggregator: components::poseidon_aggregator::Claim,
    pub poseidon_3_partial_rounds_chain: components::poseidon_3_partial_rounds_chain::Claim,
    pub poseidon_full_round_chain: components::poseidon_full_round_chain::Claim,
    pub cube_252: components::cube_252::Claim,
    pub poseidon_round_keys: components::poseidon_round_keys::Claim,
    pub range_check_252_width_27: components::range_check_252_width_27::Claim,
}

pub impl PoseidonClaimImpl of ClaimTrait<PoseidonClaim> {
    fn mix_into(self: @PoseidonClaim, ref channel: Channel) {
        self.poseidon_aggregator.mix_into(ref channel);
        self.poseidon_3_partial_rounds_chain.mix_into(ref channel);
        self.poseidon_full_round_chain.mix_into(ref channel);
        self.cube_252.mix_into(ref channel);
        self.poseidon_round_keys.mix_into(ref channel);
        self.range_check_252_width_27.mix_into(ref channel);
    }

    fn log_sizes(self: @PoseidonClaim) -> TreeArray<Span<u32>> {
        utils::tree_array_concat_cols(
            array![
                self.poseidon_aggregator.log_sizes(),
                self.poseidon_3_partial_rounds_chain.log_sizes(),
                self.poseidon_full_round_chain.log_sizes(), self.cube_252.log_sizes(),
                self.poseidon_round_keys.log_sizes(), self.range_check_252_width_27.log_sizes(),
            ],
        )
    }

    fn accumulate_relation_uses(self: @PoseidonClaim, ref relation_uses: RelationUsesDict) {
        let PoseidonClaim {
            poseidon_aggregator,
            poseidon_3_partial_rounds_chain,
            poseidon_full_round_chain,
            cube_252,
            poseidon_round_keys: _,
            range_check_252_width_27,
        } = self;

        // NOTE: The following components do not USE relations:
        // - poseidon_round_keys
        poseidon_aggregator.accumulate_relation_uses(ref relation_uses);
        poseidon_3_partial_rounds_chain.accumulate_relation_uses(ref relation_uses);
        poseidon_full_round_chain.accumulate_relation_uses(ref relation_uses);
        cube_252.accumulate_relation_uses(ref relation_uses);
        range_check_252_width_27.accumulate_relation_uses(ref relation_uses);
    }
}

#[derive(Drop, Serde)]
pub struct PoseidonInteractionClaim {
    pub poseidon_aggregator: components::poseidon_aggregator::InteractionClaim,
    pub poseidon_3_partial_rounds_chain: components::poseidon_3_partial_rounds_chain::InteractionClaim,
    pub poseidon_full_round_chain: components::poseidon_full_round_chain::InteractionClaim,
    pub cube_252: components::cube_252::InteractionClaim,
    pub poseidon_round_keys: components::poseidon_round_keys::InteractionClaim,
    pub range_check_252_width_27: components::range_check_252_width_27::InteractionClaim,
}

#[generate_trait]
pub impl PoseidonInteractionClaimImpl of PoseidonInteractionClaimTrait {
    fn mix_into(self: @PoseidonInteractionClaim, ref channel: Channel) {
        self.poseidon_aggregator.mix_into(ref channel);
        self.poseidon_3_partial_rounds_chain.mix_into(ref channel);
        self.poseidon_full_round_chain.mix_into(ref channel);
        self.cube_252.mix_into(ref channel);
        self.poseidon_round_keys.mix_into(ref channel);
        self.range_check_252_width_27.mix_into(ref channel);
    }

    fn sum(self: @PoseidonInteractionClaim) -> QM31 {
        let mut sum = Zero::zero();
        sum += *self.poseidon_aggregator.claimed_sum;
        sum += *self.poseidon_3_partial_rounds_chain.claimed_sum;
        sum += *self.poseidon_full_round_chain.claimed_sum;
        sum += *self.cube_252.claimed_sum;
        sum += *self.poseidon_round_keys.claimed_sum;
        sum += *self.range_check_252_width_27.claimed_sum;
        sum
    }
}

#[derive(Drop, Serde)]
pub struct PoseidonContextClaim {
    pub claim: Option<PoseidonClaim>,
}

pub impl PoseidonContextClaimImpl of ClaimTrait<PoseidonContextClaim> {
    fn mix_into(self: @PoseidonContextClaim, ref channel: Channel) {
        if let Option::Some(claim) = self.claim {
            claim.mix_into(ref channel);
        }
    }

    fn log_sizes(self: @PoseidonContextClaim) -> TreeArray<Span<u32>> {
        if let Option::Some(claim) = self.claim {
            claim.log_sizes()
        } else {
            array![]
        }
    }

    fn accumulate_relation_uses(self: @PoseidonContextClaim, ref relation_uses: RelationUsesDict) {
        if let Some(claim) = self.claim {
            claim.accumulate_relation_uses(ref relation_uses);
        }
    }
}

#[derive(Drop, Serde)]
pub struct PoseidonContextInteractionClaim {
    pub interaction_claim: Option<PoseidonInteractionClaim>,
}

#[generate_trait]
pub impl PoseidonContextInteractionClaimImpl of PoseidonContextInteractionClaimTrait {
    fn mix_into(self: @PoseidonContextInteractionClaim, ref channel: Channel) {
        if let Some(interaction_claim) = self.interaction_claim {
            interaction_claim.mix_into(ref channel);
        }
    }

    fn sum(self: @PoseidonContextInteractionClaim) -> QM31 {
        if let Some(interaction_claim) = self.interaction_claim {
            interaction_claim.sum()
        } else {
            Zero::zero()
        }
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
        claim: @PoseidonContextClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @PoseidonContextInteractionClaim,
    ) -> PoseidonContextComponents {
        if let Some(claim) = claim.claim {
            PoseidonContextComponents {
                components: Some(
                    PoseidonComponentsImpl::new(
                        claim,
                        interaction_elements,
                        interaction_claim.interaction_claim.as_snap().unwrap(),
                    ),
                ),
            }
        } else {
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
        point: CirclePoint<QM31>,
    ) {
        if let Option::Some(components) = self.components {
            components
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
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
        claim: @PoseidonClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @PoseidonInteractionClaim,
    ) -> PoseidonComponents {
        let poseidon_aggregator_component = components::poseidon_aggregator::NewComponentImpl::new(
            claim.poseidon_aggregator, interaction_claim.poseidon_aggregator, interaction_elements,
        );

        let poseidon_3_partial_rounds_chain_component =
            components::poseidon_3_partial_rounds_chain::NewComponentImpl::new(
            claim.poseidon_3_partial_rounds_chain,
            interaction_claim.poseidon_3_partial_rounds_chain,
            interaction_elements,
        );

        let poseidon_full_round_chain_component =
            components::poseidon_full_round_chain::NewComponentImpl::new(
            claim.poseidon_full_round_chain,
            interaction_claim.poseidon_full_round_chain,
            interaction_elements,
        );

        let cube_252_component = components::cube_252::NewComponentImpl::new(
            claim.cube_252, interaction_claim.cube_252, interaction_elements,
        );

        let poseidon_round_keys_component = components::poseidon_round_keys::NewComponentImpl::new(
            claim.poseidon_round_keys, interaction_claim.poseidon_round_keys, interaction_elements,
        );

        let range_check_felt_252_width_27_component =
            components::range_check_252_width_27::NewComponentImpl::new(
            claim.range_check_252_width_27,
            interaction_claim.range_check_252_width_27,
            interaction_elements,
        );

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
        point: CirclePoint<QM31>,
    ) {
        self
            .poseidon_aggregator
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .poseidon_3_partial_rounds_chain
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .poseidon_full_round_chain
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .cube_252
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .poseidon_round_keys
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .range_check_252_width_27
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
    }
}
