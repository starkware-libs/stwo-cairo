use components::partial_ec_mul_window_bits_18::InteractionClaimImpl as PartialEcMulInteractionClaimImpl;
use components::pedersen_aggregator_window_bits_18::InteractionClaimImpl as PedersenAggregatorInteractionClaimImpl;
use components::pedersen_builtin::InteractionClaimImpl as PedersenBuiltinInteractionClaimImpl;
use components::pedersen_points_table_window_bits_18::InteractionClaimImpl as PedersenPointsTableInteractionClaimImpl;
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_cairo_air::cairo_component::CairoComponent;
use stwo_cairo_air::components;
use stwo_constraint_framework::PreprocessedMaskValuesImpl;
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_constraint_framework::{CommonLookupElements, PreprocessedMaskValues};
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_verifier_core::ColumnSpan;
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_verifier_core::circle::CirclePoint;
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::utils::OptionImpl;


#[cfg(not(feature: "poseidon252_verifier"))]
#[derive(Drop)]
pub struct PedersenContextComponents {
    components: Option<PedersenComponents>,
}

#[generate_trait]
#[cfg(not(feature: "poseidon252_verifier"))]
pub impl PedersenContextComponentsImpl of PedersenContextComponentsTrait {
    fn new(
        pedersen_aggregator_claim: @Option<components::pedersen_aggregator_window_bits_18::Claim>,
        partial_ec_mul_claim: @Option<components::partial_ec_mul_window_bits_18::Claim>,
        pedersen_points_table_claim: @Option<
            components::pedersen_points_table_window_bits_18::Claim,
        >,
        common_lookup_elements: @CommonLookupElements,
        pedersen_aggregator_interaction_claim: @Option<
            components::pedersen_aggregator_window_bits_18::InteractionClaim,
        >,
        partial_ec_mul_interaction_claim: @Option<
            components::partial_ec_mul_window_bits_18::InteractionClaim,
        >,
        pedersen_points_table_interaction_claim: @Option<
            components::pedersen_points_table_window_bits_18::InteractionClaim,
        >,
    ) -> PedersenContextComponents {
        if let Some(_) = pedersen_aggregator_claim {
            let components = PedersenComponentsImpl::new(
                pedersen_aggregator_claim,
                partial_ec_mul_claim,
                pedersen_points_table_claim,
                common_lookup_elements,
                pedersen_aggregator_interaction_claim,
                partial_ec_mul_interaction_claim,
                pedersen_points_table_interaction_claim,
            );

            PedersenContextComponents { components: Some(components) }
        } else {
            assert!(partial_ec_mul_claim.is_none());
            assert!(pedersen_points_table_claim.is_none());
            assert!(pedersen_aggregator_interaction_claim.is_none());
            assert!(partial_ec_mul_interaction_claim.is_none());
            assert!(pedersen_points_table_interaction_claim.is_none());
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
    pub pedersen_aggregator: components::pedersen_aggregator_window_bits_18::Component,
    pub partial_ec_mul: components::partial_ec_mul_window_bits_18::Component,
    pub pedersen_points_table: components::pedersen_points_table_window_bits_18::Component,
}

#[cfg(not(feature: "poseidon252_verifier"))]
#[generate_trait]
pub impl PedersenComponentsImpl of PedersenComponentsTrait {
    fn new(
        pedersen_aggregator_claim: @Option<components::pedersen_aggregator_window_bits_18::Claim>,
        partial_ec_mul_claim: @Option<components::partial_ec_mul_window_bits_18::Claim>,
        pedersen_points_table_claim: @Option<
            components::pedersen_points_table_window_bits_18::Claim,
        >,
        common_lookup_elements: @CommonLookupElements,
        pedersen_aggregator_interaction_claim: @Option<
            components::pedersen_aggregator_window_bits_18::InteractionClaim,
        >,
        partial_ec_mul_interaction_claim: @Option<
            components::partial_ec_mul_window_bits_18::InteractionClaim,
        >,
        pedersen_points_table_interaction_claim: @Option<
            components::pedersen_points_table_window_bits_18::InteractionClaim,
        >,
    ) -> PedersenComponents {
        let pedersen_aggregator =
            components::pedersen_aggregator_window_bits_18::NewComponentImpl::try_new(
            pedersen_aggregator_claim,
            pedersen_aggregator_interaction_claim,
            common_lookup_elements,
        )
            .unwrap();

        let partial_ec_mul = components::partial_ec_mul_window_bits_18::NewComponentImpl::try_new(
            partial_ec_mul_claim, partial_ec_mul_interaction_claim, common_lookup_elements,
        )
            .unwrap();

        let pedersen_points_table =
            components::pedersen_points_table_window_bits_18::NewComponentImpl::try_new(
            pedersen_points_table_claim,
            pedersen_points_table_interaction_claim,
            common_lookup_elements,
        )
            .unwrap();

        PedersenComponents { pedersen_aggregator, partial_ec_mul, pedersen_points_table }
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
