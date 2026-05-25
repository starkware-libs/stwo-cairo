#[cfg(not(feature: "poseidon252_verifier"))]
use core::array::Span;
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_cairo_air::component_indices::*;
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_cairo_air::components;
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_constraint_framework::{
    AirComponent, CommonLookupElements, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_verifier_core::ColumnSpan;
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_verifier_core::fields::qm31::QM31;
#[cfg(not(feature: "poseidon252_verifier"))]
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
        log_size_per_component: Span<Option<u32>>,
        claimed_sum_per_component: Span<Option<QM31>>,
        common_lookup_elements: @CommonLookupElements,
    ) -> PedersenContextComponents {
        if (*log_size_per_component.at(PEDERSEN_AGGREGATOR_WINDOW_BITS_18_COMPONENT_IDX))
            .is_some() {
            PedersenContextComponents {
                components: Some(
                    PedersenComponentsImpl::new(
                        log_size_per_component, claimed_sum_per_component, common_lookup_elements,
                    ),
                ),
            }
        } else {
            assert!(
                (*log_size_per_component.at(PARTIAL_EC_MUL_WINDOW_BITS_18_COMPONENT_IDX)).is_none(),
            );
            assert!(
                (*log_size_per_component.at(PEDERSEN_POINTS_TABLE_WINDOW_BITS_18_COMPONENT_IDX))
                    .is_none(),
            );
            assert!(
                (*claimed_sum_per_component.at(PEDERSEN_AGGREGATOR_WINDOW_BITS_18_COMPONENT_IDX))
                    .is_none(),
            );
            assert!(
                (*claimed_sum_per_component.at(PARTIAL_EC_MUL_WINDOW_BITS_18_COMPONENT_IDX))
                    .is_none(),
            );
            assert!(
                (*claimed_sum_per_component.at(PEDERSEN_POINTS_TABLE_WINDOW_BITS_18_COMPONENT_IDX))
                    .is_none(),
            );
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
        log_size_per_component: Span<Option<u32>>,
        claimed_sum_per_component: Span<Option<QM31>>,
        common_lookup_elements: @CommonLookupElements,
    ) -> PedersenComponents {
        PedersenComponents {
            pedersen_aggregator: components::pedersen_aggregator_window_bits_18::NewComponentImpl::try_new(
                log_size_per_component.at(PEDERSEN_AGGREGATOR_WINDOW_BITS_18_COMPONENT_IDX),
                claimed_sum_per_component.at(PEDERSEN_AGGREGATOR_WINDOW_BITS_18_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
            partial_ec_mul: components::partial_ec_mul_window_bits_18::NewComponentImpl::try_new(
                log_size_per_component.at(PARTIAL_EC_MUL_WINDOW_BITS_18_COMPONENT_IDX),
                claimed_sum_per_component.at(PARTIAL_EC_MUL_WINDOW_BITS_18_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
            pedersen_points_table: components::pedersen_points_table_window_bits_18::NewComponentImpl::try_new(
                log_size_per_component.at(PEDERSEN_POINTS_TABLE_WINDOW_BITS_18_COMPONENT_IDX),
                claimed_sum_per_component.at(PEDERSEN_POINTS_TABLE_WINDOW_BITS_18_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
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
