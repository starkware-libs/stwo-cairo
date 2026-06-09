use components::partial_ec_mul_window_bits_18::InteractionClaimImpl as PartialEcMulInteractionClaimImpl;
use components::pedersen_aggregator_window_bits_18::InteractionClaimImpl as PedersenAggregatorInteractionClaimImpl;
use components::pedersen_builtin::InteractionClaimImpl as PedersenBuiltinInteractionClaimImpl;
use components::pedersen_points_table_window_bits_18::InteractionClaimImpl as PedersenPointsTableInteractionClaimImpl;
#[cfg(not(feature: "poseidon252_verifier"))]
use core::array::Span;
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_cairo_air::claims::CairoClaim;
use stwo_cairo_air::components;
use stwo_constraint_framework::PreprocessedMaskValuesImpl;
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_constraint_framework::{AirComponent, CommonLookupElements, PreprocessedMaskValues, pop_claimed_sum};
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_verifier_core::ColumnSpan;
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
        cairo_claim: @CairoClaim,
        common_lookup_elements: @CommonLookupElements,
        ref claimed_sums: Span<QM31>,
    ) -> PedersenContextComponents {
        if let Some(_) = cairo_claim.pedersen_aggregator_window_bits_18 {
            PedersenContextComponents {
                components: Some(
                    PedersenComponentsImpl::new(
                        cairo_claim, common_lookup_elements, ref claimed_sums,
                    ),
                ),
            }
        } else {
            assert!(cairo_claim.partial_ec_mul_window_bits_18.is_none());
            assert!(cairo_claim.pedersen_points_table_window_bits_18.is_none());
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
        cairo_claim: @CairoClaim,
        common_lookup_elements: @CommonLookupElements,
        ref claimed_sums: Span<QM31>,
    ) -> PedersenComponents {
        let interaction_claim_pedersen_aggregator = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(
                    components::pedersen_aggregator_window_bits_18::InteractionClaim {
                        claimed_sum,
                    },
                )
            },
            None => None,
        };
        let interaction_claim_partial_ec_mul = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::partial_ec_mul_window_bits_18::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_pedersen_points_table = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(
                    components::pedersen_points_table_window_bits_18::InteractionClaim {
                        claimed_sum,
                    },
                )
            },
            None => None,
        };

        let pedersen_aggregator_component =
            components::pedersen_aggregator_window_bits_18::NewComponentImpl::try_new(
            cairo_claim.pedersen_aggregator_window_bits_18,
            @interaction_claim_pedersen_aggregator,
            common_lookup_elements,
        )
            .unwrap();

        let partial_ec_mul_component =
            components::partial_ec_mul_window_bits_18::NewComponentImpl::try_new(
            cairo_claim.partial_ec_mul_window_bits_18,
            @interaction_claim_partial_ec_mul,
            common_lookup_elements,
        )
            .unwrap();

        let pedersen_points_table_component =
            components::pedersen_points_table_window_bits_18::NewComponentImpl::try_new(
            cairo_claim.pedersen_points_table_window_bits_18,
            @interaction_claim_pedersen_points_table,
            common_lookup_elements,
        )
            .unwrap();

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
    ) {
        self
            .pedersen_aggregator
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        self
            .partial_ec_mul
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        self
            .pedersen_points_table
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
    }
}
