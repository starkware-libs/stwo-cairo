use components::blake_g::{
    ClaimImpl as BlakeGClaimImpl, InteractionClaimImpl as BlakeGInteractionClaimImpl,
};
use components::blake_round::{
    ClaimImpl as BlakeRoundClaimImpl, InteractionClaimImpl as BlakeRoundInteractionClaimImpl,
};
use components::blake_round_sigma::{
    ClaimImpl as BlakeRoundSigmaClaimImpl,
    InteractionClaimImpl as BlakeRoundSigmaInteractionClaimImpl,
};
use components::triple_xor_32::{
    ClaimImpl as TripleXor32ClaimImpl, InteractionClaimImpl as TripleXor32InteractionClaimImpl,
};
use components::verify_bitwise_xor_12::{
    ClaimImpl as VerifyBitwiseXor12ClaimImpl,
    InteractionClaimImpl as VerifyBitwiseXor12InteractionClaimImpl,
};
use core::box::BoxImpl;
use core::num::traits::Zero;
use stwo_cairo_air::cairo_component::CairoComponent;
use stwo_cairo_air::{
    CairoInteractionElements, RelationUsesDict, accumulate_relation_uses, components, utils,
};
use stwo_constraint_framework::{
    LookupElementsImpl, PreprocessedColumnImpl, PreprocessedColumnKey, PreprocessedColumnSet,
    PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelImpl};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::utils::{ArrayImpl, OptionImpl};
use stwo_verifier_core::{ColumnSpan, TreeArray};


#[derive(Drop, Serde)]
pub struct BlakeClaim {
    pub blake_round: components::blake_round::Claim,
    pub blake_g: components::blake_g::Claim,
    pub blake_round_sigma: components::blake_round_sigma::Claim,
    pub triple_xor_32: components::triple_xor_32::Claim,
    pub verify_bitwise_xor_12: components::verify_bitwise_xor_12::Claim,
}

#[generate_trait]
pub impl BlakeClaimImpl of BlakeClaimTrait {
    fn mix_into(self: @BlakeClaim, ref channel: Channel) {
        self.blake_round.mix_into(ref channel);
        self.blake_g.mix_into(ref channel);
        self.blake_round_sigma.mix_into(ref channel);
        self.triple_xor_32.mix_into(ref channel);
        self.verify_bitwise_xor_12.mix_into(ref channel);
    }

    fn log_sizes(self: @BlakeClaim) -> TreeArray<Span<u32>> {
        utils::tree_array_concat_cols(
            array![
                self.blake_round.log_sizes(), self.blake_g.log_sizes(),
                self.blake_round_sigma.log_sizes(), self.triple_xor_32.log_sizes(),
                self.verify_bitwise_xor_12.log_sizes(),
            ],
        )
    }

    fn accumulate_relation_uses(self: @BlakeClaim, ref relation_uses: RelationUsesDict) {
        let BlakeClaim {
            blake_round, blake_g, blake_round_sigma: _, triple_xor_32, verify_bitwise_xor_12: _,
        } = self;
        // NOTE: The following components do not USE relations:
        // - blake_sigma
        // - verify_bitwise_xor_12

        accumulate_relation_uses(
            ref relation_uses,
            components::blake_round::RELATION_USES_PER_ROW.span(),
            *blake_round.log_size,
        );
        accumulate_relation_uses(
            ref relation_uses, components::blake_g::RELATION_USES_PER_ROW.span(), *blake_g.log_size,
        );
        accumulate_relation_uses(
            ref relation_uses,
            components::triple_xor_32::RELATION_USES_PER_ROW.span(),
            *triple_xor_32.log_size,
        );
    }
}

#[derive(Drop, Serde)]
pub struct BlakeInteractionClaim {
    pub blake_round: components::blake_round::InteractionClaim,
    pub blake_g: components::blake_g::InteractionClaim,
    pub blake_round_sigma: components::blake_round_sigma::InteractionClaim,
    pub triple_xor_32: components::triple_xor_32::InteractionClaim,
    pub verify_bitwise_xor_12: components::verify_bitwise_xor_12::InteractionClaim,
}

#[generate_trait]
pub impl BlakeInteractionClaimImpl of BlakeInteractionClaimTrait {
    fn mix_into(self: @BlakeInteractionClaim, ref channel: Channel) {
        self.blake_round.mix_into(ref channel);
        self.blake_g.mix_into(ref channel);
        self.blake_round_sigma.mix_into(ref channel);
        self.triple_xor_32.mix_into(ref channel);
        self.verify_bitwise_xor_12.mix_into(ref channel);
    }

    fn sum(self: @BlakeInteractionClaim) -> QM31 {
        let mut sum = Zero::zero();
        sum += *self.blake_round.claimed_sum;
        sum += *self.blake_g.claimed_sum;
        sum += *self.blake_round_sigma.claimed_sum;
        sum += *self.triple_xor_32.claimed_sum;
        sum += *self.verify_bitwise_xor_12.claimed_sum;
        sum
    }
}

#[derive(Drop, Serde)]
pub struct BlakeContextClaim {
    pub claim: Option<BlakeClaim>,
}

#[generate_trait]
pub impl BlakeContextClaimImpl of BlakeContextClaimTrait {
    fn mix_into(self: @BlakeContextClaim, ref channel: Channel) {
        if let Some(claim) = self.claim {
            claim.mix_into(ref channel);
        }
    }

    fn log_sizes(self: @BlakeContextClaim) -> TreeArray<Span<u32>> {
        if let Some(claim) = self.claim {
            claim.log_sizes()
        } else {
            array![]
        }
    }

    fn accumulate_relation_uses(self: @BlakeContextClaim, ref relation_uses: RelationUsesDict) {
        if let Some(claim) = self.claim {
            claim.accumulate_relation_uses(ref relation_uses);
        }
    }
}


#[derive(Drop, Serde)]
pub struct BlakeContextInteractionClaim {
    pub interaction_claim: Option<BlakeInteractionClaim>,
}

#[generate_trait]
pub impl BlakeContextInteractionClaimImpl of BlakeContextInteractionClaimTrait {
    fn mix_into(self: @BlakeContextInteractionClaim, ref channel: Channel) {
        if let Some(interaction_claim) = self.interaction_claim {
            interaction_claim.mix_into(ref channel);
        }
    }

    fn sum(self: @BlakeContextInteractionClaim) -> QM31 {
        if let Some(interaction_claim) = self.interaction_claim {
            interaction_claim.sum()
        } else {
            Zero::zero()
        }
    }
}


#[derive(Drop)]
pub struct BlakeContextComponents {
    components: Option<BlakeComponents>,
}

#[generate_trait]
pub impl BlakeContextComponentsImpl of BlakeContextComponentsTrait {
    fn new(
        claim: @BlakeContextClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @BlakeContextInteractionClaim,
    ) -> BlakeContextComponents {
        if let Some(claim) = claim.claim {
            BlakeContextComponents {
                components: Some(
                    BlakeComponentsImpl::new(
                        claim,
                        interaction_elements,
                        interaction_claim.interaction_claim.as_snap().unwrap(),
                    ),
                ),
            }
        } else {
            BlakeContextComponents { components: None }
        }
    }

    fn max_constraint_log_degree_bound(self: @BlakeContextComponents) -> u32 {
        if let Option::Some(components) = self.components {
            components.max_constraint_log_degree_bound()
        } else {
            0
        }
    }

    fn mask_points(
        self: @BlakeContextComponents,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        if let Option::Some(components) = self.components {
            components
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }
    }

    fn evaluate_constraints_at_point(
        self: @BlakeContextComponents,
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
        claim: @BlakeClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @BlakeInteractionClaim,
    ) -> BlakeComponents {
        let blake_round_component = components::blake_round::Component {
            claim: *claim.blake_round,
            interaction_claim: *interaction_claim.blake_round,
            blake_round_lookup_elements: interaction_elements.blake_round.clone(),
            blake_g_lookup_elements: interaction_elements.blake_g.clone(),
            blake_round_sigma_lookup_elements: interaction_elements.blake_round_sigma.clone(),
            memory_address_to_id_lookup_elements: interaction_elements.memory_address_to_id.clone(),
            memory_id_to_big_lookup_elements: interaction_elements.memory_id_to_value.clone(),
            range_check_7_2_5_lookup_elements: interaction_elements.range_checks.rc_7_2_5.clone(),
        };

        let blake_g_component = components::blake_g::Component {
            claim: *claim.blake_g,
            interaction_claim: *interaction_claim.blake_g,
            blake_g_lookup_elements: interaction_elements.blake_g.clone(),
            verify_bitwise_xor_12_lookup_elements: interaction_elements
                .verify_bitwise_xor_12
                .clone(),
            verify_bitwise_xor_4_lookup_elements: interaction_elements.verify_bitwise_xor_4.clone(),
            verify_bitwise_xor_7_lookup_elements: interaction_elements.verify_bitwise_xor_7.clone(),
            verify_bitwise_xor_8_lookup_elements: interaction_elements.verify_bitwise_xor_8.clone(),
            verify_bitwise_xor_9_lookup_elements: interaction_elements.verify_bitwise_xor_9.clone(),
        };

        let blake_round_sigma_component = components::blake_round_sigma::Component {
            claim: *claim.blake_round_sigma,
            interaction_claim: *interaction_claim.blake_round_sigma,
            blake_round_sigma_lookup_elements: interaction_elements.blake_round_sigma.clone(),
        };

        let triple_xor_32_component = components::triple_xor_32::Component {
            claim: *claim.triple_xor_32,
            interaction_claim: *interaction_claim.triple_xor_32,
            verify_bitwise_xor_8_lookup_elements: interaction_elements.verify_bitwise_xor_8.clone(),
            triple_xor_32_lookup_elements: interaction_elements.triple_xor_32.clone(),
        };

        let verify_bitwise_xor_12_component = components::verify_bitwise_xor_12::Component {
            claim: *claim.verify_bitwise_xor_12,
            interaction_claim: *interaction_claim.verify_bitwise_xor_12,
            verify_bitwise_xor_12_lookup_elements: interaction_elements
                .verify_bitwise_xor_12
                .clone(),
        };

        BlakeComponents {
            blake_round: blake_round_component,
            blake_g: blake_g_component,
            blake_round_sigma: blake_round_sigma_component,
            triple_xor_32: triple_xor_32_component,
            verify_bitwise_xor_12: verify_bitwise_xor_12_component,
        }
    }

    fn mask_points(
        self: @BlakeComponents,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        self
            .blake_round
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .blake_g
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .blake_round_sigma
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .triple_xor_32
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .verify_bitwise_xor_12
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
    }

    fn evaluate_constraints_at_point(
        self: @BlakeComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        self
            .blake_round
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .blake_g
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .blake_round_sigma
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .triple_xor_32
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .verify_bitwise_xor_12
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
    }

    fn max_constraint_log_degree_bound(self: @BlakeComponents) -> u32 {
        let mut max_degree = 0;
        max_degree = core::cmp::max(max_degree, self.blake_round.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.blake_g.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.blake_round_sigma.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.triple_xor_32.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(
                max_degree, self.verify_bitwise_xor_12.max_constraint_log_degree_bound(),
            );
        max_degree
    }
}

