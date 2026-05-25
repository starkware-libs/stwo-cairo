#[cfg(not(feature: "poseidon252_verifier"))]
use core::array::Span;
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
use stwo_cairo_air::component_indices::*;
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
use stwo_cairo_air::components;
use stwo_constraint_framework::PreprocessedMaskValuesImpl;
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
use stwo_constraint_framework::{AirComponent, CommonLookupElements, PreprocessedMaskValues};
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
use stwo_verifier_core::ColumnSpan;
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::utils::OptionImpl;
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
#[derive(Drop)]
pub struct PoseidonContextComponents {
    components: Option<PoseidonComponents>,
}

#[generate_trait]
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
pub impl PoseidonContextComponentsImpl of PoseidonContextComponentsTrait {
    fn new(
        log_size_per_component: Span<Option<u32>>,
        claimed_sum_per_component: Span<Option<QM31>>,
        common_lookup_elements: @CommonLookupElements,
    ) -> PoseidonContextComponents {
        if (*log_size_per_component.at(POSEIDON_AGGREGATOR_COMPONENT_IDX)).is_some() {
            PoseidonContextComponents {
                components: Some(
                    PoseidonComponentsImpl::new(
                        log_size_per_component, claimed_sum_per_component, common_lookup_elements,
                    ),
                ),
            }
        } else {
            assert!(
                (*log_size_per_component.at(POSEIDON_3_PARTIAL_ROUNDS_CHAIN_COMPONENT_IDX))
                    .is_none(),
            );
            assert!(
                (*log_size_per_component.at(POSEIDON_FULL_ROUND_CHAIN_COMPONENT_IDX)).is_none(),
            );
            assert!((*log_size_per_component.at(CUBE_252_COMPONENT_IDX)).is_none());
            assert!((*log_size_per_component.at(POSEIDON_ROUND_KEYS_COMPONENT_IDX)).is_none());
            assert!((*log_size_per_component.at(RANGE_CHECK_252_WIDTH_27_COMPONENT_IDX)).is_none());
            assert!((*claimed_sum_per_component.at(POSEIDON_AGGREGATOR_COMPONENT_IDX)).is_none());
            assert!(
                (*claimed_sum_per_component.at(POSEIDON_3_PARTIAL_ROUNDS_CHAIN_COMPONENT_IDX))
                    .is_none(),
            );
            assert!(
                (*claimed_sum_per_component.at(POSEIDON_FULL_ROUND_CHAIN_COMPONENT_IDX)).is_none(),
            );
            assert!((*claimed_sum_per_component.at(CUBE_252_COMPONENT_IDX)).is_none());
            assert!((*claimed_sum_per_component.at(POSEIDON_ROUND_KEYS_COMPONENT_IDX)).is_none());
            assert!(
                (*claimed_sum_per_component.at(RANGE_CHECK_252_WIDTH_27_COMPONENT_IDX)).is_none(),
            );
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
        log_size_per_component: Span<Option<u32>>,
        claimed_sum_per_component: Span<Option<QM31>>,
        common_lookup_elements: @CommonLookupElements,
    ) -> PoseidonComponents {
        PoseidonComponents {
            poseidon_aggregator: components::poseidon_aggregator::NewComponentImpl::try_new(
                log_size_per_component.at(POSEIDON_AGGREGATOR_COMPONENT_IDX),
                claimed_sum_per_component.at(POSEIDON_AGGREGATOR_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
            poseidon_3_partial_rounds_chain: components::poseidon_3_partial_rounds_chain::NewComponentImpl::try_new(
                log_size_per_component.at(POSEIDON_3_PARTIAL_ROUNDS_CHAIN_COMPONENT_IDX),
                claimed_sum_per_component.at(POSEIDON_3_PARTIAL_ROUNDS_CHAIN_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
            poseidon_full_round_chain: components::poseidon_full_round_chain::NewComponentImpl::try_new(
                log_size_per_component.at(POSEIDON_FULL_ROUND_CHAIN_COMPONENT_IDX),
                claimed_sum_per_component.at(POSEIDON_FULL_ROUND_CHAIN_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
            cube_252: components::cube_252::NewComponentImpl::try_new(
                log_size_per_component.at(CUBE_252_COMPONENT_IDX),
                claimed_sum_per_component.at(CUBE_252_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
            poseidon_round_keys: components::poseidon_round_keys::NewComponentImpl::try_new(
                log_size_per_component.at(POSEIDON_ROUND_KEYS_COMPONENT_IDX),
                claimed_sum_per_component.at(POSEIDON_ROUND_KEYS_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
            range_check_252_width_27: components::range_check_252_width_27::NewComponentImpl::try_new(
                log_size_per_component.at(RANGE_CHECK_252_WIDTH_27_COMPONENT_IDX),
                claimed_sum_per_component.at(RANGE_CHECK_252_WIDTH_27_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
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
                [].span(),
            );
        self
            .poseidon_3_partial_rounds_chain
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        self
            .poseidon_full_round_chain
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        self
            .cube_252
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        self
            .poseidon_round_keys
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        self
            .range_check_252_width_27
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
