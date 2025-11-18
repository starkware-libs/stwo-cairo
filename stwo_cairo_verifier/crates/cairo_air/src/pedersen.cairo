use components::partial_ec_mul::InteractionClaimImpl as PartialEcMulInteractionClaimImpl;
use components::pedersen_aggregator::InteractionClaimImpl as PedersenAggregatorInteractionClaimImpl;
use components::pedersen_builtin::InteractionClaimImpl as PedersenBuiltinInteractionClaimImpl;
use components::pedersen_points_table::InteractionClaimImpl as PedersenPointsTableInteractionClaimImpl;
use core::box::BoxImpl;
use core::num::traits::Zero;
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_cairo_air::CairoInteractionElements;
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_cairo_air::cairo_component::CairoComponent;
use stwo_cairo_air::claim::ClaimTrait;
use stwo_cairo_air::{RelationUsesDict, components, utils};
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_constraint_framework::PreprocessedMaskValues;
use stwo_constraint_framework::{LookupElementsImpl, PreprocessedMaskValuesImpl};
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_verifier_core::ColumnSpan;
use stwo_verifier_core::TreeArray;
use stwo_verifier_core::channel::Channel;
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::utils::{ArrayImpl, OptionImpl};


#[derive(Drop, Serde)]
pub struct PedersenClaim {
    pub pedersen_aggregator: components::pedersen_aggregator::Claim,
    pub partial_ec_mul: components::partial_ec_mul::Claim,
    pub pedersen_points_table: components::pedersen_points_table::Claim,
}

pub impl PedersenClaimImpl of ClaimTrait<PedersenClaim> {
    fn mix_into(self: @PedersenClaim, ref channel: Channel) {
        self.pedersen_aggregator.mix_into(ref channel);
        self.partial_ec_mul.mix_into(ref channel);
        self.pedersen_points_table.mix_into(ref channel);
    }

    fn log_sizes(self: @PedersenClaim) -> TreeArray<Span<u32>> {
        utils::tree_array_concat_cols(
            array![
                self.pedersen_aggregator.log_sizes(), self.partial_ec_mul.log_sizes(),
                self.pedersen_points_table.log_sizes(),
            ],
        )
    }

    fn accumulate_relation_uses(self: @PedersenClaim, ref relation_uses: RelationUsesDict) {
        let PedersenClaim { pedersen_aggregator, partial_ec_mul, pedersen_points_table: _ } = self;

        // NOTE: The following components do not USE relations:
        // - pedersen_points_table
        pedersen_aggregator.accumulate_relation_uses(ref relation_uses);
        partial_ec_mul.accumulate_relation_uses(ref relation_uses);
    }
}

#[derive(Drop, Serde)]
pub struct PedersenInteractionClaim {
    pub pedersen_aggregator: components::pedersen_aggregator::InteractionClaim,
    pub partial_ec_mul: components::partial_ec_mul::InteractionClaim,
    pub pedersen_points_table: components::pedersen_points_table::InteractionClaim,
}

#[generate_trait]
pub impl PedersenInteractionClaimImpl of PedersenInteractionClaimTrait {
    fn mix_into(self: @PedersenInteractionClaim, ref channel: Channel) {
        self.pedersen_aggregator.mix_into(ref channel);
        self.partial_ec_mul.mix_into(ref channel);
        self.pedersen_points_table.mix_into(ref channel);
    }

    fn sum(self: @PedersenInteractionClaim) -> QM31 {
        let mut sum = Zero::zero();
        sum += *self.pedersen_aggregator.claimed_sum;
        sum += *self.partial_ec_mul.claimed_sum;
        sum += *self.pedersen_points_table.claimed_sum;
        sum
    }
}

#[derive(Drop, Serde)]
pub struct PedersenContextClaim {
    pub claim: Option<PedersenClaim>,
}

pub impl PedersenContextClaimImpl of ClaimTrait<PedersenContextClaim> {
    fn mix_into(self: @PedersenContextClaim, ref channel: Channel) {
        if let Option::Some(claim) = self.claim {
            claim.mix_into(ref channel);
        }
    }

    fn log_sizes(self: @PedersenContextClaim) -> TreeArray<Span<u32>> {
        if let Option::Some(claim) = self.claim {
            claim.log_sizes()
        } else {
            array![]
        }
    }

    fn accumulate_relation_uses(self: @PedersenContextClaim, ref relation_uses: RelationUsesDict) {
        if let Some(claim) = self.claim {
            claim.accumulate_relation_uses(ref relation_uses);
        }
    }
}

#[derive(Drop, Serde)]
pub struct PedersenContextInteractionClaim {
    pub interaction_claim: Option<PedersenInteractionClaim>,
}

#[generate_trait]
pub impl PedersenContextInteractionClaimImpl of PedersenContextInteractionClaimTrait {
    fn mix_into(self: @PedersenContextInteractionClaim, ref channel: Channel) {
        if let Some(interaction_claim) = self.interaction_claim {
            interaction_claim.mix_into(ref channel);
        }
    }

    fn sum(self: @PedersenContextInteractionClaim) -> QM31 {
        if let Some(interaction_claim) = self.interaction_claim {
            interaction_claim.sum()
        } else {
            Zero::zero()
        }
    }
}


#[cfg(not(feature: "poseidon252_verifier"))]
#[derive(Drop)]
pub struct PedersenContextComponents {
    components: Option<PedersenComponents>,
}

#[generate_trait]
#[cfg(not(feature: "poseidon252_verifier"))]
pub impl PedersenContextComponentsImpl of PedersenContextComponentsTrait {
    fn new(
        claim: @PedersenContextClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @PedersenContextInteractionClaim,
    ) -> PedersenContextComponents {
        if let Some(claim) = claim.claim {
            PedersenContextComponents {
                components: Some(
                    PedersenComponentsImpl::new(
                        claim,
                        interaction_elements,
                        interaction_claim.interaction_claim.as_snap().unwrap(),
                    ),
                ),
            }
        } else {
            PedersenContextComponents { components: None }
        }
    }

    fn evaluate_constraints_at_point(
        self: @PedersenContextComponents,
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

#[cfg(not(feature: "poseidon252_verifier"))]
#[derive(Drop)]
pub struct PedersenComponents {
    pub pedersen_aggregator: components::pedersen_aggregator::Component,
    pub partial_ec_mul: components::partial_ec_mul::Component,
    pub pedersen_points_table: components::pedersen_points_table::Component,
}

#[cfg(not(feature: "poseidon252_verifier"))]
#[generate_trait]
pub impl PedersenComponentsImpl of PedersenComponentsTrait {
    fn new(
        claim: @PedersenClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @PedersenInteractionClaim,
    ) -> PedersenComponents {
        let pedersen_aggregator_component = components::pedersen_aggregator::NewComponentImpl::new(
            claim.pedersen_aggregator, interaction_claim.pedersen_aggregator, interaction_elements,
        );

        let partial_ec_mul_component = components::partial_ec_mul::NewComponentImpl::new(
            claim.partial_ec_mul, interaction_claim.partial_ec_mul, interaction_elements,
        );

        let pedersen_points_table_component =
            components::pedersen_points_table::NewComponentImpl::new(
            claim.pedersen_points_table,
            interaction_claim.pedersen_points_table,
            interaction_elements,
        );

        PedersenComponents {
            pedersen_aggregator: pedersen_aggregator_component,
            partial_ec_mul: partial_ec_mul_component,
            pedersen_points_table: pedersen_points_table_component,
        }
    }

    fn evaluate_constraints_at_point(
        self: @PedersenComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        self
            .pedersen_aggregator
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .partial_ec_mul
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .pedersen_points_table
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
