#[cfg(not(feature: "poseidon252_verifier"))]
use core::array::Span;
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
use stwo_cairo_air::claims::CairoClaim;
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
        cairo_claim: @CairoClaim,
        common_lookup_elements: @CommonLookupElements,
        ref claimed_sums: Span<QM31>,
    ) -> PoseidonContextComponents {
        if let Some(_) = cairo_claim.poseidon_aggregator {
            PoseidonContextComponents {
                components: Some(
                    PoseidonComponentsImpl::new(
                        cairo_claim, common_lookup_elements, ref claimed_sums,
                    ),
                ),
            }
        } else {
            assert!(cairo_claim.poseidon_3_partial_rounds_chain.is_none());
            assert!(cairo_claim.poseidon_full_round_chain.is_none());
            assert!(cairo_claim.cube_252.is_none());
            assert!(cairo_claim.poseidon_round_keys.is_none());
            assert!(cairo_claim.range_check_252_width_27.is_none());
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
        cairo_claim: @CairoClaim,
        common_lookup_elements: @CommonLookupElements,
        ref claimed_sums: Span<QM31>,
    ) -> PoseidonComponents {
        let poseidon_aggregator_component =
            components::poseidon_aggregator::NewComponentImpl::try_new(
            cairo_claim.poseidon_aggregator, ref claimed_sums, common_lookup_elements,
        )
            .unwrap();

        let poseidon_3_partial_rounds_chain_component =
            components::poseidon_3_partial_rounds_chain::NewComponentImpl::try_new(
            cairo_claim.poseidon_3_partial_rounds_chain, ref claimed_sums, common_lookup_elements,
        )
            .unwrap();

        let poseidon_full_round_chain_component =
            components::poseidon_full_round_chain::NewComponentImpl::try_new(
            cairo_claim.poseidon_full_round_chain, ref claimed_sums, common_lookup_elements,
        )
            .unwrap();

        let cube_252_component = components::cube_252::NewComponentImpl::try_new(
            cairo_claim.cube_252, ref claimed_sums, common_lookup_elements,
        )
            .unwrap();

        let poseidon_round_keys_component =
            components::poseidon_round_keys::NewComponentImpl::try_new(
            cairo_claim.poseidon_round_keys, ref claimed_sums, common_lookup_elements,
        )
            .unwrap();

        let range_check_felt_252_width_27_component =
            components::range_check_252_width_27::NewComponentImpl::try_new(
            cairo_claim.range_check_252_width_27, ref claimed_sums, common_lookup_elements,
        )
            .unwrap();

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
