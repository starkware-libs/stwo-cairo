use components::partial_ec_mul::{
    ClaimImpl as PartialEcMulClaimImpl, InteractionClaimImpl as PartialEcMulInteractionClaimImpl,
};
use components::pedersen_builtin::{
    ClaimImpl as PedersenBuiltinClaimImpl,
    InteractionClaimImpl as PedersenBuiltinInteractionClaimImpl,
};
use components::pedersen_points_table::{
    ClaimImpl as PedersenPointsTableClaimImpl,
    InteractionClaimImpl as PedersenPointsTableInteractionClaimImpl,
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
pub struct PedersenClaim {
    pub partial_ec_mul: components::partial_ec_mul::Claim,
    pub pedersen_points_table: components::pedersen_points_table::Claim,
}

#[generate_trait]
pub impl PedersenClaimImpl of PedersenClaimTrait {
    fn mix_into(self: @PedersenClaim, ref channel: Channel) {
        self.partial_ec_mul.mix_into(ref channel);
        self.pedersen_points_table.mix_into(ref channel);
    }

    fn log_sizes(self: @PedersenClaim) -> TreeArray<Span<u32>> {
        utils::tree_array_concat_cols(
            array![self.partial_ec_mul.log_sizes(), self.pedersen_points_table.log_sizes()],
        )
    }

    fn accumulate_relation_uses(self: @PedersenClaim, ref relation_uses: RelationUsesDict) {
        let PedersenClaim { partial_ec_mul, pedersen_points_table: _ } = self;

        // NOTE: The following components do not USE relations:
        // - pedersen_points_table

        accumulate_relation_uses(
            ref relation_uses,
            components::partial_ec_mul::RELATION_USES_PER_ROW.span(),
            *partial_ec_mul.log_size,
        );
    }
}

#[derive(Drop, Serde)]
pub struct PedersenInteractionClaim {
    pub partial_ec_mul: components::partial_ec_mul::InteractionClaim,
    pub pedersen_points_table: components::pedersen_points_table::InteractionClaim,
}

#[generate_trait]
pub impl PedersenInteractionClaimImpl of PedersenInteractionClaimTrait {
    fn mix_into(self: @PedersenInteractionClaim, ref channel: Channel) {
        self.partial_ec_mul.mix_into(ref channel);
        self.pedersen_points_table.mix_into(ref channel);
    }

    fn sum(self: @PedersenInteractionClaim) -> QM31 {
        let mut sum = Zero::zero();
        sum += *self.partial_ec_mul.claimed_sum;
        sum += *self.pedersen_points_table.claimed_sum;
        sum
    }
}

#[derive(Drop, Serde)]
pub struct PedersenContextClaim {
    pub claim: Option<PedersenClaim>,
}

#[generate_trait]
pub impl PedersenContextClaimImpl of PedersenContextClaimTrait {
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

    fn max_constraint_log_degree_bound(self: @PedersenContextComponents) -> u32 {
        if let Option::Some(components) = self.components {
            components.max_constraint_log_degree_bound()
        } else {
            0
        }
    }

    fn mask_points(
        self: @PedersenContextComponents,
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
        let partial_ec_mul_component = components::partial_ec_mul::Component {
            claim: *claim.partial_ec_mul,
            interaction_claim: *interaction_claim.partial_ec_mul,
            partial_ec_mul_lookup_elements: interaction_elements.partial_ec_mul.clone(),
            pedersen_points_table_lookup_elements: interaction_elements
                .pedersen_points_table
                .clone(),
            range_check_19_lookup_elements: interaction_elements.range_checks.rc_19.clone(),
            range_check_19_b_lookup_elements: interaction_elements.range_checks.rc_19_b.clone(),
            range_check_19_c_lookup_elements: interaction_elements.range_checks.rc_19_c.clone(),
            range_check_19_d_lookup_elements: interaction_elements.range_checks.rc_19_d.clone(),
            range_check_19_e_lookup_elements: interaction_elements.range_checks.rc_19_e.clone(),
            range_check_19_f_lookup_elements: interaction_elements.range_checks.rc_19_f.clone(),
            range_check_19_g_lookup_elements: interaction_elements.range_checks.rc_19_g.clone(),
            range_check_19_h_lookup_elements: interaction_elements.range_checks.rc_19_h.clone(),
            range_check_9_9_lookup_elements: interaction_elements.range_checks.rc_9_9.clone(),
            range_check_9_9_b_lookup_elements: interaction_elements.range_checks.rc_9_9_b.clone(),
            range_check_9_9_c_lookup_elements: interaction_elements.range_checks.rc_9_9_c.clone(),
            range_check_9_9_d_lookup_elements: interaction_elements.range_checks.rc_9_9_d.clone(),
            range_check_9_9_e_lookup_elements: interaction_elements.range_checks.rc_9_9_e.clone(),
            range_check_9_9_f_lookup_elements: interaction_elements.range_checks.rc_9_9_f.clone(),
            range_check_9_9_g_lookup_elements: interaction_elements.range_checks.rc_9_9_g.clone(),
            range_check_9_9_h_lookup_elements: interaction_elements.range_checks.rc_9_9_h.clone(),
        };

        let pedersen_points_table_component = components::pedersen_points_table::Component {
            claim: *claim.pedersen_points_table,
            interaction_claim: *interaction_claim.pedersen_points_table,
            pedersen_points_table_lookup_elements: interaction_elements
                .pedersen_points_table
                .clone(),
        };

        PedersenComponents {
            partial_ec_mul: partial_ec_mul_component,
            pedersen_points_table: pedersen_points_table_component,
        }
    }

    fn mask_points(
        self: @PedersenComponents,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        self
            .partial_ec_mul
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .pedersen_points_table
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
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

    fn max_constraint_log_degree_bound(self: @PedersenComponents) -> u32 {
        let mut max_degree = 0;
        max_degree =
            core::cmp::max(max_degree, self.partial_ec_mul.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(
                max_degree, self.pedersen_points_table.max_constraint_log_degree_bound(),
            );
        max_degree
    }
}
