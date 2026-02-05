use components::range_check96_builtin::InteractionClaimImpl as RangeCheck96BuiltinInteractionClaimImpl;
use components::range_check_11::InteractionClaimImpl as RangeCheck_11InteractionClaimImpl;
use components::range_check_12::InteractionClaimImpl as RangeCheck_12InteractionClaimImpl;
use components::range_check_18::InteractionClaimImpl as RangeCheck_18InteractionClaimImpl;
use components::range_check_20::InteractionClaimImpl as RangeCheck_20InteractionClaimImpl;
use components::range_check_252_width_27::InteractionClaimImpl as RangeCheckFelt252Width27InteractionClaimImpl;
use components::range_check_3_3_3_3_3::InteractionClaimImpl as RangeCheck_3_3_3_3_3InteractionClaimImpl;
use components::range_check_3_6_6_3::InteractionClaimImpl as RangeCheck_3_6_6_3InteractionClaimImpl;
use components::range_check_4_3::InteractionClaimImpl as RangeCheck_4_3InteractionClaimImpl;
use components::range_check_4_4::InteractionClaimImpl as RangeCheck_4_4InteractionClaimImpl;
use components::range_check_4_4_4_4::InteractionClaimImpl as RangeCheck_4_4_4_4InteractionClaimImpl;
use components::range_check_6::InteractionClaimImpl as RangeCheck_6InteractionClaimImpl;
use components::range_check_7_2_5::InteractionClaimImpl as RangeCheck_7_2_5InteractionClaimImpl;
use components::range_check_8::InteractionClaimImpl as RangeCheck_8InteractionClaimImpl;
use components::range_check_9_9::InteractionClaimImpl as RangeCheck_9_9InteractionClaimImpl;
use components::range_check_builtin::InteractionClaimImpl as RangeCheckBuiltinInteractionClaimImpl;
use core::array::Span;
use stwo_cairo_air::cairo_component::CairoComponent;
use stwo_cairo_air::claims::{CairoClaim, CairoInteractionClaim};
use stwo_cairo_air::components;
use stwo_constraint_framework::{
    CommonLookupElements, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::ColumnSpan;
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::utils::OptionImpl;
#[derive(Drop)]
pub struct RangeChecksComponents {
    rc_6: components::range_check_6::Component,
    rc_8: components::range_check_8::Component,
    rc_11: components::range_check_11::Component,
    rc_12: components::range_check_12::Component,
    rc_18: components::range_check_18::Component,
    rc_20: components::range_check_20::Component,
    rc_4_3: components::range_check_4_3::Component,
    rc_4_4: components::range_check_4_4::Component,
    rc_9_9: components::range_check_9_9::Component,
    rc_7_2_5: components::range_check_7_2_5::Component,
    rc_3_6_6_3: components::range_check_3_6_6_3::Component,
    rc_4_4_4_4: components::range_check_4_4_4_4::Component,
    rc_3_3_3_3_3: components::range_check_3_3_3_3_3::Component,
}

#[generate_trait]
pub impl RangeChecksComponentsImpl of RangeChecksComponentsTrait {
    fn new(
        cairo_claim: @CairoClaim,
        common_lookup_elements: @CommonLookupElements,
        interaction_claim: @CairoInteractionClaim,
    ) -> RangeChecksComponents {
        RangeChecksComponents {
            rc_6: components::range_check_6::NewComponentImpl::try_new(
                cairo_claim.range_check_6, interaction_claim.range_check_6, common_lookup_elements,
            )
                .unwrap(),
            rc_8: components::range_check_8::NewComponentImpl::try_new(
                cairo_claim.range_check_8, interaction_claim.range_check_8, common_lookup_elements,
            )
                .unwrap(),
            rc_11: components::range_check_11::NewComponentImpl::try_new(
                cairo_claim.range_check_11,
                interaction_claim.range_check_11,
                common_lookup_elements,
            )
                .unwrap(),
            rc_12: components::range_check_12::NewComponentImpl::try_new(
                cairo_claim.range_check_12,
                interaction_claim.range_check_12,
                common_lookup_elements,
            )
                .unwrap(),
            rc_18: components::range_check_18::NewComponentImpl::try_new(
                cairo_claim.range_check_18,
                interaction_claim.range_check_18,
                common_lookup_elements,
            )
                .unwrap(),
            rc_20: components::range_check_20::NewComponentImpl::try_new(
                cairo_claim.range_check_20,
                interaction_claim.range_check_20,
                common_lookup_elements,
            )
                .unwrap(),
            rc_4_3: components::range_check_4_3::NewComponentImpl::try_new(
                cairo_claim.range_check_4_3,
                interaction_claim.range_check_4_3,
                common_lookup_elements,
            )
                .unwrap(),
            rc_4_4: components::range_check_4_4::NewComponentImpl::try_new(
                cairo_claim.range_check_4_4,
                interaction_claim.range_check_4_4,
                common_lookup_elements,
            )
                .unwrap(),
            rc_9_9: components::range_check_9_9::NewComponentImpl::try_new(
                cairo_claim.range_check_9_9,
                interaction_claim.range_check_9_9,
                common_lookup_elements,
            )
                .unwrap(),
            rc_7_2_5: components::range_check_7_2_5::NewComponentImpl::try_new(
                cairo_claim.range_check_7_2_5,
                interaction_claim.range_check_7_2_5,
                common_lookup_elements,
            )
                .unwrap(),
            rc_3_6_6_3: components::range_check_3_6_6_3::NewComponentImpl::try_new(
                cairo_claim.range_check_3_6_6_3,
                interaction_claim.range_check_3_6_6_3,
                common_lookup_elements,
            )
                .unwrap(),
            rc_4_4_4_4: components::range_check_4_4_4_4::NewComponentImpl::try_new(
                cairo_claim.range_check_4_4_4_4,
                interaction_claim.range_check_4_4_4_4,
                common_lookup_elements,
            )
                .unwrap(),
            rc_3_3_3_3_3: components::range_check_3_3_3_3_3::NewComponentImpl::try_new(
                cairo_claim.range_check_3_3_3_3_3,
                interaction_claim.range_check_3_3_3_3_3,
                common_lookup_elements,
            )
                .unwrap(),
        }
    }

    fn evaluate_constraints_at_point(
        self: @RangeChecksComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
    ) {
        self
            .rc_6
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        self
            .rc_8
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        self
            .rc_11
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        self
            .rc_12
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        self
            .rc_18
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        self
            .rc_20
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        self
            .rc_4_3
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        self
            .rc_4_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        self
            .rc_9_9
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        self
            .rc_7_2_5
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        self
            .rc_3_6_6_3
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        self
            .rc_4_4_4_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        self
            .rc_3_3_3_3_3
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
    }
}

