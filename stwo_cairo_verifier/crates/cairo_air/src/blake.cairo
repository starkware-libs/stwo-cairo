use core::array::Span;
use stwo_cairo_air::component_indices::*;
use stwo_cairo_air::components;
use stwo_constraint_framework::{
    AirComponent, CommonLookupElements, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::ColumnSpan;
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::utils::OptionImpl;
#[derive(Drop)]
pub struct BlakeContextComponents {
    components: Option<BlakeComponents>,
}

#[generate_trait]
pub impl BlakeContextComponentsImpl of BlakeContextComponentsTrait {
    fn new(
        log_size_per_component: Span<Option<u32>>,
        claimed_sum_per_component: Span<Option<QM31>>,
        common_lookup_elements: @CommonLookupElements,
    ) -> BlakeContextComponents {
        if (*log_size_per_component.at(BLAKE_ROUND_COMPONENT_IDX)).is_some() {
            BlakeContextComponents {
                components: Some(
                    BlakeComponentsImpl::new(
                        log_size_per_component, claimed_sum_per_component, common_lookup_elements,
                    ),
                ),
            }
        } else {
            assert!((*log_size_per_component.at(BLAKE_G_COMPONENT_IDX)).is_none());
            assert!((*log_size_per_component.at(BLAKE_ROUND_SIGMA_COMPONENT_IDX)).is_none());
            assert!((*log_size_per_component.at(TRIPLE_XOR_32_COMPONENT_IDX)).is_none());
            assert!((*log_size_per_component.at(VERIFY_BITWISE_XOR_12_COMPONENT_IDX)).is_none());
            assert!((*claimed_sum_per_component.at(BLAKE_ROUND_COMPONENT_IDX)).is_none());
            assert!((*claimed_sum_per_component.at(BLAKE_G_COMPONENT_IDX)).is_none());
            assert!((*claimed_sum_per_component.at(BLAKE_ROUND_SIGMA_COMPONENT_IDX)).is_none());
            assert!((*claimed_sum_per_component.at(TRIPLE_XOR_32_COMPONENT_IDX)).is_none());
            assert!((*claimed_sum_per_component.at(VERIFY_BITWISE_XOR_12_COMPONENT_IDX)).is_none());
            BlakeContextComponents { components: None }
        }
    }

    fn evaluate_constraints_at_point(
        self: @BlakeContextComponents,
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
        log_size_per_component: Span<Option<u32>>,
        claimed_sum_per_component: Span<Option<QM31>>,
        common_lookup_elements: @CommonLookupElements,
    ) -> BlakeComponents {
        BlakeComponents {
            blake_round: components::blake_round::NewComponentImpl::try_new(
                log_size_per_component.at(BLAKE_ROUND_COMPONENT_IDX),
                claimed_sum_per_component.at(BLAKE_ROUND_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
            blake_g: components::blake_g::NewComponentImpl::try_new(
                log_size_per_component.at(BLAKE_G_COMPONENT_IDX),
                claimed_sum_per_component.at(BLAKE_G_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
            blake_round_sigma: components::blake_round_sigma::NewComponentImpl::try_new(
                log_size_per_component.at(BLAKE_ROUND_SIGMA_COMPONENT_IDX),
                claimed_sum_per_component.at(BLAKE_ROUND_SIGMA_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
            triple_xor_32: components::triple_xor_32::NewComponentImpl::try_new(
                log_size_per_component.at(TRIPLE_XOR_32_COMPONENT_IDX),
                claimed_sum_per_component.at(TRIPLE_XOR_32_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
            verify_bitwise_xor_12: components::verify_bitwise_xor_12::NewComponentImpl::try_new(
                log_size_per_component.at(VERIFY_BITWISE_XOR_12_COMPONENT_IDX),
                claimed_sum_per_component.at(VERIFY_BITWISE_XOR_12_COMPONENT_IDX),
                common_lookup_elements,
            )
                .unwrap(),
        }
    }

    fn evaluate_constraints_at_point(
        self: @BlakeComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
    ) {
        self
            .blake_round
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        self
            .blake_g
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        self
            .blake_round_sigma
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        self
            .triple_xor_32
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        self
            .verify_bitwise_xor_12
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
