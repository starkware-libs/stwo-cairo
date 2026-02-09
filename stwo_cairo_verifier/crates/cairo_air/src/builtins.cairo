use components::add_mod_builtin::InteractionClaimImpl as AddModBuiltinInteractionClaimImpl;
use components::bitwise_builtin::InteractionClaimImpl as BitwiseBuiltinInteractionClaimImpl;
use components::mul_mod_builtin::InteractionClaimImpl as MulModBuiltinInteractionClaimImpl;
use components::pedersen_builtin::InteractionClaimImpl as PedersenBuiltinInteractionClaimImpl;
use components::poseidon_builtin::InteractionClaimImpl as PoseidonBuiltinInteractionClaimImpl;
use components::range_check96_builtin::InteractionClaimImpl as RangeCheckBuiltinBits96InteractionClaimImpl;
use components::range_check_builtin::InteractionClaimImpl as RangeCheckBuiltinBits128InteractionClaimImpl;
use core::array::Span;
use core::box::BoxImpl;
use stwo_cairo_air::cairo_component::CairoComponent;
use stwo_cairo_air::claims::{CairoClaim, CairoInteractionClaim};
use stwo_cairo_air::components;
use stwo_constraint_framework::{
    CommonLookupElements, LookupElementsImpl, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::ColumnSpan;
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::utils::{ArrayImpl, OptionImpl};
#[derive(Drop)]
#[cfg(not(feature: "poseidon252_verifier"))]
pub struct BuiltinComponents {
    pub add_mod_builtin: Option<components::add_mod_builtin::Component>,
    pub bitwise_builtin: Option<components::bitwise_builtin::Component>,
    pub mul_mod_builtin: Option<components::mul_mod_builtin::Component>,
    pub pedersen_builtin: Option<components::pedersen_builtin::Component>,
    pub poseidon_builtin: Option<components::poseidon_builtin::Component>,
    pub range_check_96_builtin: Option<components::range_check96_builtin::Component>,
    pub range_check_128_builtin: Option<components::range_check_builtin::Component>,
}

#[derive(Drop)]
#[cfg(and(feature: "poseidon252_verifier", not(feature: "poseidon_outputs_packing")))]
pub struct BuiltinComponents {
    pub bitwise_builtin: Option<components::bitwise_builtin::Component>,
    pub range_check_128_builtin: Option<components::range_check_builtin::Component>,
}

#[derive(Drop)]
#[cfg(and(feature: "poseidon252_verifier", feature: "poseidon_outputs_packing"))]
pub struct BuiltinComponents {
    pub bitwise_builtin: Option<components::bitwise_builtin::Component>,
    pub poseidon_builtin: Option<components::poseidon_builtin::Component>,
    pub range_check_128_builtin: Option<components::range_check_builtin::Component>,
}


#[generate_trait]
#[cfg(not(feature: "poseidon252_verifier"))]
pub impl BuiltinComponentsImpl of BuiltinComponentsTrait {
    fn new(
        cairo_claim: @CairoClaim,
        common_lookup_elements: @CommonLookupElements,
        interaction_claim: @CairoInteractionClaim,
    ) -> BuiltinComponents {
        assert!(
            cairo_claim.pedersen_builtin_narrow_windows.is_none()
                && interaction_claim.pedersen_builtin_narrow_windows.is_none(),
        );

        let add_mod_builtin_component = components::add_mod_builtin::NewComponentImpl::try_new(
            cairo_claim.add_mod_builtin, interaction_claim.add_mod_builtin, common_lookup_elements,
        );

        let bitwise_builtin_component = components::bitwise_builtin::NewComponentImpl::try_new(
            cairo_claim.bitwise_builtin, interaction_claim.bitwise_builtin, common_lookup_elements,
        );

        let mul_mod_builtin_component = components::mul_mod_builtin::NewComponentImpl::try_new(
            cairo_claim.mul_mod_builtin, interaction_claim.mul_mod_builtin, common_lookup_elements,
        );

        let pedersen_builtin_component = components::pedersen_builtin::NewComponentImpl::try_new(
            cairo_claim.pedersen_builtin,
            interaction_claim.pedersen_builtin,
            common_lookup_elements,
        );

        let poseidon_builtin_component = components::poseidon_builtin::NewComponentImpl::try_new(
            cairo_claim.poseidon_builtin,
            interaction_claim.poseidon_builtin,
            common_lookup_elements,
        );

        let range_check_96_builtin_component =
            components::range_check96_builtin::NewComponentImpl::try_new(
            cairo_claim.range_check96_builtin,
            interaction_claim.range_check96_builtin,
            common_lookup_elements,
        );

        let range_check_128_builtin_component =
            components::range_check_builtin::NewComponentImpl::try_new(
            cairo_claim.range_check_builtin,
            interaction_claim.range_check_builtin,
            common_lookup_elements,
        );

        BuiltinComponents {
            add_mod_builtin: add_mod_builtin_component,
            bitwise_builtin: bitwise_builtin_component,
            mul_mod_builtin: mul_mod_builtin_component,
            pedersen_builtin: pedersen_builtin_component,
            poseidon_builtin: poseidon_builtin_component,
            range_check_96_builtin: range_check_96_builtin_component,
            range_check_128_builtin: range_check_128_builtin_component,
        }
    }

    fn evaluate_constraints_at_point(
        self: @BuiltinComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
    ) {
        let BuiltinComponents {
            add_mod_builtin,
            bitwise_builtin,
            mul_mod_builtin,
            pedersen_builtin,
            poseidon_builtin,
            range_check_96_builtin,
            range_check_128_builtin,
        } = self;

        if let Option::Some(component) = add_mod_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        if let Option::Some(component) = bitwise_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        if let Option::Some(component) = mul_mod_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        if let Option::Some(component) = pedersen_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        if let Option::Some(component) = poseidon_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        if let Option::Some(component) = range_check_96_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        if let Option::Some(component) = range_check_128_builtin.as_snap() {
            component
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

#[generate_trait]
#[cfg(and(feature: "poseidon252_verifier", not(feature: "poseidon_outputs_packing")))]
pub impl BuiltinComponentsImpl of BuiltinComponentsTrait {
    fn new(
        cairo_claim: @CairoClaim,
        common_lookup_elements: @CommonLookupElements,
        interaction_claim: @CairoInteractionClaim,
    ) -> BuiltinComponents {
        assert!(
            cairo_claim.range_check96_builtin.is_none()
                && interaction_claim.range_check96_builtin.is_none(),
        );
        assert!(
            cairo_claim.add_mod_builtin.is_none() && interaction_claim.add_mod_builtin.is_none(),
        );
        assert!(
            cairo_claim.mul_mod_builtin.is_none() && interaction_claim.mul_mod_builtin.is_none(),
        );
        assert!(
            cairo_claim.pedersen_builtin.is_none() && interaction_claim.pedersen_builtin.is_none(),
        );
        assert!(
            cairo_claim.pedersen_builtin_narrow_windows.is_none()
                && interaction_claim.pedersen_builtin_narrow_windows.is_none(),
        );
        assert!(
            cairo_claim.poseidon_builtin.is_none() && interaction_claim.poseidon_builtin.is_none(),
        );

        let bitwise_builtin_component = components::bitwise_builtin::NewComponentImpl::try_new(
            cairo_claim.bitwise_builtin, interaction_claim.bitwise_builtin, common_lookup_elements,
        );

        let range_check_128_builtin_component =
            components::range_check_builtin::NewComponentImpl::try_new(
            cairo_claim.range_check_builtin,
            interaction_claim.range_check_builtin,
            common_lookup_elements,
        );

        BuiltinComponents {
            bitwise_builtin: bitwise_builtin_component,
            range_check_128_builtin: range_check_128_builtin_component,
        }
    }

    fn evaluate_constraints_at_point(
        self: @BuiltinComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
    ) {
        let BuiltinComponents { bitwise_builtin, range_check_128_builtin } = self;

        if let Option::Some(component) = bitwise_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        if let Option::Some(component) = range_check_128_builtin.as_snap() {
            component
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

#[generate_trait]
#[cfg(and(feature: "poseidon252_verifier", feature: "poseidon_outputs_packing"))]
pub impl BuiltinComponentsImpl of BuiltinComponentsTrait {
    fn new(
        cairo_claim: @CairoClaim,
        common_lookup_elements: @CommonLookupElements,
        interaction_claim: @CairoInteractionClaim,
    ) -> BuiltinComponents {
        assert!(
            cairo_claim.range_check96_builtin.is_none()
                && interaction_claim.range_check96_builtin.is_none(),
        );
        assert!(
            cairo_claim.add_mod_builtin.is_none() && interaction_claim.add_mod_builtin.is_none(),
        );
        assert!(
            cairo_claim.mul_mod_builtin.is_none() && interaction_claim.mul_mod_builtin.is_none(),
        );
        assert!(
            cairo_claim.pedersen_builtin.is_none() && interaction_claim.pedersen_builtin.is_none(),
        );
        assert!(
            cairo_claim.pedersen_builtin_narrow_windows.is_none()
                && interaction_claim.pedersen_builtin_narrow_windows.is_none(),
        );

        let bitwise_builtin_component = components::bitwise_builtin::NewComponentImpl::try_new(
            cairo_claim.bitwise_builtin, interaction_claim.bitwise_builtin, common_lookup_elements,
        );

        let poseidon_builtin_component = components::poseidon_builtin::NewComponentImpl::try_new(
            cairo_claim.poseidon_builtin,
            interaction_claim.poseidon_builtin,
            common_lookup_elements,
        );

        let range_check_128_builtin_component =
            components::range_check_builtin::NewComponentImpl::try_new(
            cairo_claim.range_check_builtin,
            interaction_claim.range_check_builtin,
            common_lookup_elements,
        );

        BuiltinComponents {
            bitwise_builtin: bitwise_builtin_component,
            poseidon_builtin: poseidon_builtin_component,
            range_check_128_builtin: range_check_128_builtin_component,
        }
    }

    fn evaluate_constraints_at_point(
        self: @BuiltinComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
    ) {
        let BuiltinComponents { bitwise_builtin, poseidon_builtin, range_check_128_builtin } = self;

        if let Option::Some(component) = bitwise_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        if let Option::Some(component) = poseidon_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        if let Option::Some(component) = range_check_128_builtin.as_snap() {
            component
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
