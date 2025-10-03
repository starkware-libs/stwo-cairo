use components::sha_256_big_sigma_0::InteractionClaimImpl as Sha256BigSigma0InteractionClaimImpl;
use components::sha_256_big_sigma_0_o_0::InteractionClaimImpl as Sha256BigSigma0O0InteractionClaimImpl;
use components::sha_256_big_sigma_0_o_1::InteractionClaimImpl as Sha256BigSigma0O1InteractionClaimImpl;
use components::sha_256_big_sigma_1::InteractionClaimImpl as Sha256BigSigma1InteractionClaimImpl;
use components::sha_256_big_sigma_1_o_0::InteractionClaimImpl as Sha256BigSigma1O0InteractionClaimImpl;
use components::sha_256_big_sigma_1_o_1::InteractionClaimImpl as Sha256BigSigma1O1InteractionClaimImpl;
use components::sha_256_k_table::InteractionClaimImpl as Sha256KTableInteractionClaimImpl;
use components::sha_256_round::InteractionClaimImpl as Sha256RoundInteractionClaimImpl;
use components::sha_256_schedule::InteractionClaimImpl as Sha256ScheduleInteractionClaimImpl;
use components::sha_256_small_sigma_0::InteractionClaimImpl as Sha256SmallSigma0InteractionClaimImpl;
use components::sha_256_small_sigma_0_o_0::InteractionClaimImpl as Sha256SmallSigma0O0InteractionClaimImpl;
use components::sha_256_small_sigma_0_o_1::InteractionClaimImpl as Sha256SmallSigma0O1InteractionClaimImpl;
use components::sha_256_small_sigma_1::InteractionClaimImpl as Sha256SmallSigma1InteractionClaimImpl;
use components::sha_256_small_sigma_1_o_0::InteractionClaimImpl as Sha256SmallSigma1O0InteractionClaimImpl;
use components::sha_256_small_sigma_1_o_1::InteractionClaimImpl as Sha256SmallSigma1O1InteractionClaimImpl;
use core::box::BoxImpl;
use core::num::traits::Zero;
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_cairo_air::CairoInteractionElements;
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_cairo_air::cairo_component::CairoComponent;
use stwo_cairo_air::claim::ClaimTrait;
use stwo_cairo_air::{RelationUsesDict, components, utils};
use stwo_constraint_framework::{
    LookupElementsImpl, PreprocessedColumnImpl, PreprocessedColumnKey, PreprocessedMaskValuesImpl,
};
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_constraint_framework::{PreprocessedColumnSet, PreprocessedMaskValues};
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
pub struct Sha256Claim {
    pub sha_256_round: components::sha_256_round::Claim,
    pub sha_256_big_sigma_0: components::sha_256_big_sigma_0::Claim,
    pub sha_256_big_sigma_1: components::sha_256_big_sigma_1::Claim,
    pub sha_256_schedule: components::sha_256_schedule::Claim,
    pub sha_256_small_sigma_0: components::sha_256_small_sigma_0::Claim,
    pub sha_256_small_sigma_1: components::sha_256_small_sigma_1::Claim,
    pub sha_256_big_sigma_0_o_0: components::sha_256_big_sigma_0_o_0::Claim,
    pub sha_256_big_sigma_0_o_1: components::sha_256_big_sigma_0_o_1::Claim,
    pub sha_256_big_sigma_1_o_0: components::sha_256_big_sigma_1_o_0::Claim,
    pub sha_256_big_sigma_1_o_1: components::sha_256_big_sigma_1_o_1::Claim,
    pub sha_256_small_sigma_0_o_0: components::sha_256_small_sigma_0_o_0::Claim,
    pub sha_256_small_sigma_0_o_1: components::sha_256_small_sigma_0_o_1::Claim,
    pub sha_256_small_sigma_1_o_0: components::sha_256_small_sigma_1_o_0::Claim,
    pub sha_256_small_sigma_1_o_1: components::sha_256_small_sigma_1_o_1::Claim,
    pub sha_256_k_table: components::sha_256_k_table::Claim,
}

pub impl Sha256ClaimImpl of ClaimTrait<Sha256Claim> {
    fn mix_into(self: @Sha256Claim, ref channel: Channel) {
        self.sha_256_round.mix_into(ref channel);
        self.sha_256_big_sigma_0.mix_into(ref channel);
        self.sha_256_big_sigma_1.mix_into(ref channel);
        self.sha_256_schedule.mix_into(ref channel);
        self.sha_256_small_sigma_0.mix_into(ref channel);
        self.sha_256_small_sigma_1.mix_into(ref channel);
        self.sha_256_big_sigma_0_o_0.mix_into(ref channel);
        self.sha_256_big_sigma_0_o_1.mix_into(ref channel);
        self.sha_256_big_sigma_1_o_0.mix_into(ref channel);
        self.sha_256_big_sigma_1_o_1.mix_into(ref channel);
        self.sha_256_small_sigma_0_o_0.mix_into(ref channel);
        self.sha_256_small_sigma_0_o_1.mix_into(ref channel);
        self.sha_256_small_sigma_1_o_0.mix_into(ref channel);
        self.sha_256_small_sigma_1_o_1.mix_into(ref channel);
        self.sha_256_k_table.mix_into(ref channel);
    }

    fn log_sizes(self: @Sha256Claim) -> TreeArray<Span<u32>> {
        utils::tree_array_concat_cols(
            array![
                self.sha_256_round.log_sizes(), self.sha_256_big_sigma_0.log_sizes(),
                self.sha_256_big_sigma_1.log_sizes(), self.sha_256_schedule.log_sizes(),
                self.sha_256_small_sigma_0.log_sizes(), self.sha_256_small_sigma_1.log_sizes(),
                self.sha_256_big_sigma_0_o_0.log_sizes(), self.sha_256_big_sigma_0_o_1.log_sizes(),
                self.sha_256_big_sigma_1_o_0.log_sizes(), self.sha_256_big_sigma_1_o_1.log_sizes(),
                self.sha_256_small_sigma_0_o_0.log_sizes(),
                self.sha_256_small_sigma_0_o_1.log_sizes(),
                self.sha_256_small_sigma_1_o_0.log_sizes(),
                self.sha_256_small_sigma_1_o_1.log_sizes(), self.sha_256_k_table.log_sizes(),
            ],
        )
    }

    fn accumulate_relation_uses(self: @Sha256Claim, ref relation_uses: RelationUsesDict) {
        let Sha256Claim {
            sha_256_round,
            sha_256_big_sigma_0,
            sha_256_big_sigma_1,
            sha_256_schedule,
            sha_256_small_sigma_0,
            sha_256_small_sigma_1,
            ..,
        } = self;

        sha_256_round.accumulate_relation_uses(ref relation_uses);
        sha_256_big_sigma_0.accumulate_relation_uses(ref relation_uses);
        sha_256_big_sigma_1.accumulate_relation_uses(ref relation_uses);
        sha_256_schedule.accumulate_relation_uses(ref relation_uses);
        sha_256_small_sigma_0.accumulate_relation_uses(ref relation_uses);
        sha_256_small_sigma_1.accumulate_relation_uses(ref relation_uses);
    }
}

#[derive(Drop, Serde)]
pub struct Sha256InteractionClaim {
    pub sha_256_round: components::sha_256_round::InteractionClaim,
    pub sha_256_big_sigma_0: components::sha_256_big_sigma_0::InteractionClaim,
    pub sha_256_big_sigma_1: components::sha_256_big_sigma_1::InteractionClaim,
    pub sha_256_schedule: components::sha_256_schedule::InteractionClaim,
    pub sha_256_small_sigma_0: components::sha_256_small_sigma_0::InteractionClaim,
    pub sha_256_small_sigma_1: components::sha_256_small_sigma_1::InteractionClaim,
    pub sha_256_big_sigma_0_o_0: components::sha_256_big_sigma_0_o_0::InteractionClaim,
    pub sha_256_big_sigma_0_o_1: components::sha_256_big_sigma_0_o_1::InteractionClaim,
    pub sha_256_big_sigma_1_o_0: components::sha_256_big_sigma_1_o_0::InteractionClaim,
    pub sha_256_big_sigma_1_o_1: components::sha_256_big_sigma_1_o_1::InteractionClaim,
    pub sha_256_small_sigma_0_o_0: components::sha_256_small_sigma_0_o_0::InteractionClaim,
    pub sha_256_small_sigma_0_o_1: components::sha_256_small_sigma_0_o_1::InteractionClaim,
    pub sha_256_small_sigma_1_o_0: components::sha_256_small_sigma_1_o_0::InteractionClaim,
    pub sha_256_small_sigma_1_o_1: components::sha_256_small_sigma_1_o_1::InteractionClaim,
    pub sha_256_k_table: components::sha_256_k_table::InteractionClaim,
}

#[generate_trait]
pub impl Sha256InteractionClaimImpl of Sha256InteractionClaimTrait {
    fn mix_into(self: @Sha256InteractionClaim, ref channel: Channel) {
        self.sha_256_round.mix_into(ref channel);
        self.sha_256_big_sigma_0.mix_into(ref channel);
        self.sha_256_big_sigma_1.mix_into(ref channel);
        self.sha_256_schedule.mix_into(ref channel);
        self.sha_256_small_sigma_0.mix_into(ref channel);
        self.sha_256_small_sigma_1.mix_into(ref channel);
        self.sha_256_big_sigma_0_o_0.mix_into(ref channel);
        self.sha_256_big_sigma_0_o_1.mix_into(ref channel);
        self.sha_256_big_sigma_1_o_0.mix_into(ref channel);
        self.sha_256_big_sigma_1_o_1.mix_into(ref channel);
        self.sha_256_small_sigma_0_o_0.mix_into(ref channel);
        self.sha_256_small_sigma_0_o_1.mix_into(ref channel);
        self.sha_256_small_sigma_1_o_0.mix_into(ref channel);
        self.sha_256_small_sigma_1_o_1.mix_into(ref channel);
        self.sha_256_k_table.mix_into(ref channel);
    }

    fn sum(self: @Sha256InteractionClaim) -> QM31 {
        let mut sum = Zero::zero();
        sum += *self.sha_256_round.claimed_sum;
        sum += *self.sha_256_big_sigma_0.claimed_sum;
        sum += *self.sha_256_big_sigma_1.claimed_sum;
        sum += *self.sha_256_schedule.claimed_sum;
        sum += *self.sha_256_small_sigma_0.claimed_sum;
        sum += *self.sha_256_small_sigma_1.claimed_sum;
        sum += *self.sha_256_big_sigma_0_o_0.claimed_sum;
        sum += *self.sha_256_big_sigma_0_o_1.claimed_sum;
        sum += *self.sha_256_big_sigma_1_o_0.claimed_sum;
        sum += *self.sha_256_big_sigma_1_o_1.claimed_sum;
        sum += *self.sha_256_small_sigma_0_o_0.claimed_sum;
        sum += *self.sha_256_small_sigma_0_o_1.claimed_sum;
        sum += *self.sha_256_small_sigma_1_o_0.claimed_sum;
        sum += *self.sha_256_small_sigma_1_o_1.claimed_sum;
        sum += *self.sha_256_k_table.claimed_sum;
        sum
    }
}

#[derive(Drop, Serde)]
pub struct Sha256ContextClaim {
    pub claim: Option<Sha256Claim>,
}

pub impl Sha256ContextClaimImpl of ClaimTrait<Sha256ContextClaim> {
    fn mix_into(self: @Sha256ContextClaim, ref channel: Channel) {
        if let Option::Some(claim) = self.claim {
            claim.mix_into(ref channel);
        }
    }

    fn log_sizes(self: @Sha256ContextClaim) -> TreeArray<Span<u32>> {
        if let Option::Some(claim) = self.claim {
            claim.log_sizes()
        } else {
            array![]
        }
    }

    fn accumulate_relation_uses(self: @Sha256ContextClaim, ref relation_uses: RelationUsesDict) {
        if let Some(claim) = self.claim {
            claim.accumulate_relation_uses(ref relation_uses);
        }
    }
}

#[derive(Drop, Serde)]
pub struct Sha256ContextInteractionClaim {
    pub interaction_claim: Option<Sha256InteractionClaim>,
}

#[generate_trait]
pub impl Sha256ContextInteractionClaimImpl of Sha256ContextInteractionClaimTrait {
    fn mix_into(self: @Sha256ContextInteractionClaim, ref channel: Channel) {
        if let Some(interaction_claim) = self.interaction_claim {
            interaction_claim.mix_into(ref channel);
        }
    }

    fn sum(self: @Sha256ContextInteractionClaim) -> QM31 {
        if let Some(interaction_claim) = self.interaction_claim {
            interaction_claim.sum()
        } else {
            Zero::zero()
        }
    }
}


#[cfg(not(feature: "poseidon252_verifier"))]
#[derive(Drop)]
pub struct Sha256ContextComponents {
    components: Option<Sha256Components>,
}

#[generate_trait]
#[cfg(not(feature: "poseidon252_verifier"))]
pub impl Sha256ContextComponentsImpl of Sha256ContextComponentsTrait {
    fn new(
        claim: @Sha256ContextClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @Sha256ContextInteractionClaim,
    ) -> Sha256ContextComponents {
        if let Some(claim) = claim.claim {
            Sha256ContextComponents {
                components: Some(
                    Sha256ComponentsImpl::new(
                        claim,
                        interaction_elements,
                        interaction_claim.interaction_claim.as_snap().unwrap(),
                    ),
                ),
            }
        } else {
            Sha256ContextComponents { components: None }
        }
    }

    fn max_constraint_log_degree_bound(self: @Sha256ContextComponents) -> u32 {
        if let Option::Some(components) = self.components {
            components.max_constraint_log_degree_bound()
        } else {
            0
        }
    }

    fn mask_points(
        self: @Sha256ContextComponents,
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
        self: @Sha256ContextComponents,
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
pub struct Sha256Components {
    pub sha_256_round: components::sha_256_round::Component,
    pub sha_256_big_sigma_0: components::sha_256_big_sigma_0::Component,
    pub sha_256_big_sigma_1: components::sha_256_big_sigma_1::Component,
    pub sha_256_schedule: components::sha_256_schedule::Component,
    pub sha_256_small_sigma_0: components::sha_256_small_sigma_0::Component,
    pub sha_256_small_sigma_1: components::sha_256_small_sigma_1::Component,
    pub sha_256_big_sigma_0_o_0: components::sha_256_big_sigma_0_o_0::Component,
    pub sha_256_big_sigma_0_o_1: components::sha_256_big_sigma_0_o_1::Component,
    pub sha_256_big_sigma_1_o_0: components::sha_256_big_sigma_1_o_0::Component,
    pub sha_256_big_sigma_1_o_1: components::sha_256_big_sigma_1_o_1::Component,
    pub sha_256_small_sigma_0_o_0: components::sha_256_small_sigma_0_o_0::Component,
    pub sha_256_small_sigma_0_o_1: components::sha_256_small_sigma_0_o_1::Component,
    pub sha_256_small_sigma_1_o_0: components::sha_256_small_sigma_1_o_0::Component,
    pub sha_256_small_sigma_1_o_1: components::sha_256_small_sigma_1_o_1::Component,
    pub sha_256_k_table: components::sha_256_k_table::Component,
}

#[cfg(not(feature: "poseidon252_verifier"))]
#[generate_trait]
pub impl Sha256ComponentsImpl of Sha256ComponentsTrait {
    fn new(
        claim: @Sha256Claim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @Sha256InteractionClaim,
    ) -> Sha256Components {
        let sha_256_round_component = components::sha_256_round::NewComponentImpl::new(
            claim.sha_256_round, interaction_claim.sha_256_round, interaction_elements,
        );

        let sha_256_big_sigma_0_component = components::sha_256_big_sigma_0::NewComponentImpl::new(
            claim.sha_256_big_sigma_0, interaction_claim.sha_256_big_sigma_0, interaction_elements,
        );

        let sha_256_big_sigma_1_component = components::sha_256_big_sigma_1::NewComponentImpl::new(
            claim.sha_256_big_sigma_1, interaction_claim.sha_256_big_sigma_1, interaction_elements,
        );

        let sha_256_schedule_component = components::sha_256_schedule::NewComponentImpl::new(
            claim.sha_256_schedule, interaction_claim.sha_256_schedule, interaction_elements,
        );
        let sha_256_small_sigma_0_component =
            components::sha_256_small_sigma_0::NewComponentImpl::new(
            claim.sha_256_small_sigma_0,
            interaction_claim.sha_256_small_sigma_0,
            interaction_elements,
        );
        let sha_256_small_sigma_1_component =
            components::sha_256_small_sigma_1::NewComponentImpl::new(
            claim.sha_256_small_sigma_1,
            interaction_claim.sha_256_small_sigma_1,
            interaction_elements,
        );
        let sha_256_big_sigma_0_o_0_component =
            components::sha_256_big_sigma_0_o_0::NewComponentImpl::new(
            claim.sha_256_big_sigma_0_o_0,
            interaction_claim.sha_256_big_sigma_0_o_0,
            interaction_elements,
        );
        let sha_256_big_sigma_0_o_1_component =
            components::sha_256_big_sigma_0_o_1::NewComponentImpl::new(
            claim.sha_256_big_sigma_0_o_1,
            interaction_claim.sha_256_big_sigma_0_o_1,
            interaction_elements,
        );
        let sha_256_big_sigma_1_o_0_component =
            components::sha_256_big_sigma_1_o_0::NewComponentImpl::new(
            claim.sha_256_big_sigma_1_o_0,
            interaction_claim.sha_256_big_sigma_1_o_0,
            interaction_elements,
        );
        let sha_256_big_sigma_1_o_1_component =
            components::sha_256_big_sigma_1_o_1::NewComponentImpl::new(
            claim.sha_256_big_sigma_1_o_1,
            interaction_claim.sha_256_big_sigma_1_o_1,
            interaction_elements,
        );
        let sha_256_small_sigma_0_o_0_component =
            components::sha_256_small_sigma_0_o_0::NewComponentImpl::new(
            claim.sha_256_small_sigma_0_o_0,
            interaction_claim.sha_256_small_sigma_0_o_0,
            interaction_elements,
        );
        let sha_256_small_sigma_0_o_1_component =
            components::sha_256_small_sigma_0_o_1::NewComponentImpl::new(
            claim.sha_256_small_sigma_0_o_1,
            interaction_claim.sha_256_small_sigma_0_o_1,
            interaction_elements,
        );
        let sha_256_small_sigma_1_o_0_component =
            components::sha_256_small_sigma_1_o_0::NewComponentImpl::new(
            claim.sha_256_small_sigma_1_o_0,
            interaction_claim.sha_256_small_sigma_1_o_0,
            interaction_elements,
        );
        let sha_256_small_sigma_1_o_1_component =
            components::sha_256_small_sigma_1_o_1::NewComponentImpl::new(
            claim.sha_256_small_sigma_1_o_1,
            interaction_claim.sha_256_small_sigma_1_o_1,
            interaction_elements,
        );
        let sha_256_k_table_component = components::sha_256_k_table::NewComponentImpl::new(
            claim.sha_256_k_table, interaction_claim.sha_256_k_table, interaction_elements,
        );

        Sha256Components {
            sha_256_round: sha_256_round_component,
            sha_256_big_sigma_0: sha_256_big_sigma_0_component,
            sha_256_big_sigma_1: sha_256_big_sigma_1_component,
            sha_256_schedule: sha_256_schedule_component,
            sha_256_small_sigma_0: sha_256_small_sigma_0_component,
            sha_256_small_sigma_1: sha_256_small_sigma_1_component,
            sha_256_big_sigma_0_o_0: sha_256_big_sigma_0_o_0_component,
            sha_256_big_sigma_0_o_1: sha_256_big_sigma_0_o_1_component,
            sha_256_big_sigma_1_o_0: sha_256_big_sigma_1_o_0_component,
            sha_256_big_sigma_1_o_1: sha_256_big_sigma_1_o_1_component,
            sha_256_small_sigma_0_o_0: sha_256_small_sigma_0_o_0_component,
            sha_256_small_sigma_0_o_1: sha_256_small_sigma_0_o_1_component,
            sha_256_small_sigma_1_o_0: sha_256_small_sigma_1_o_0_component,
            sha_256_small_sigma_1_o_1: sha_256_small_sigma_1_o_1_component,
            sha_256_k_table: sha_256_k_table_component,
        }
    }

    fn mask_points(
        self: @Sha256Components,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        self
            .sha_256_round
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .sha_256_big_sigma_0
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .sha_256_big_sigma_1
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .sha_256_schedule
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .sha_256_small_sigma_0
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .sha_256_small_sigma_1
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .sha_256_big_sigma_0_o_0
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .sha_256_big_sigma_0_o_1
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .sha_256_big_sigma_1_o_0
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .sha_256_big_sigma_1_o_1
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .sha_256_small_sigma_0_o_0
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .sha_256_small_sigma_0_o_1
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .sha_256_small_sigma_1_o_0
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .sha_256_small_sigma_1_o_1
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .sha_256_k_table
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
    }

    fn evaluate_constraints_at_point(
        self: @Sha256Components,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        self
            .sha_256_round
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .sha_256_big_sigma_0
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .sha_256_big_sigma_1
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .sha_256_schedule
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .sha_256_small_sigma_0
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .sha_256_small_sigma_1
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .sha_256_big_sigma_0_o_0
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .sha_256_big_sigma_0_o_1
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .sha_256_big_sigma_1_o_0
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .sha_256_big_sigma_1_o_1
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .sha_256_small_sigma_0_o_0
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .sha_256_small_sigma_0_o_1
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .sha_256_small_sigma_1_o_0
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .sha_256_small_sigma_1_o_1
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .sha_256_k_table
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
    }

    fn max_constraint_log_degree_bound(self: @Sha256Components) -> u32 {
        let mut max_degree = 0;
        max_degree =
            core::cmp::max(max_degree, self.sha_256_round.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.sha_256_big_sigma_0.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.sha_256_big_sigma_1.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.sha_256_schedule.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(
                max_degree, self.sha_256_small_sigma_0.max_constraint_log_degree_bound(),
            );
        max_degree =
            core::cmp::max(
                max_degree, self.sha_256_small_sigma_1.max_constraint_log_degree_bound(),
            );
        max_degree =
            core::cmp::max(
                max_degree, self.sha_256_big_sigma_0_o_0.max_constraint_log_degree_bound(),
            );
        max_degree =
            core::cmp::max(
                max_degree, self.sha_256_big_sigma_0_o_1.max_constraint_log_degree_bound(),
            );
        max_degree =
            core::cmp::max(
                max_degree, self.sha_256_big_sigma_1_o_0.max_constraint_log_degree_bound(),
            );
        max_degree =
            core::cmp::max(
                max_degree, self.sha_256_big_sigma_1_o_1.max_constraint_log_degree_bound(),
            );
        max_degree =
            core::cmp::max(
                max_degree, self.sha_256_small_sigma_0_o_0.max_constraint_log_degree_bound(),
            );
        max_degree =
            core::cmp::max(
                max_degree, self.sha_256_small_sigma_0_o_1.max_constraint_log_degree_bound(),
            );
        max_degree =
            core::cmp::max(
                max_degree, self.sha_256_small_sigma_1_o_0.max_constraint_log_degree_bound(),
            );
        max_degree =
            core::cmp::max(
                max_degree, self.sha_256_small_sigma_1_o_1.max_constraint_log_degree_bound(),
            );
        max_degree =
            core::cmp::max(max_degree, self.sha_256_k_table.max_constraint_log_degree_bound());
        max_degree
    }
}
