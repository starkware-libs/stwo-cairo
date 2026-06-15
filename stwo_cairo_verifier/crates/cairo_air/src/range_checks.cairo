use core::array::Span;
use stwo_cairo_air::claims::CairoClaim;
use stwo_cairo_air::components;
use stwo_constraint_framework::{
    AirComponent, CommonLookupElements, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
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
        ref claimed_sums: Span<QM31>,
    ) -> RangeChecksComponents {
        RangeChecksComponents {
            rc_6: components::range_check_6::NewComponentImpl::try_new(
                cairo_claim.range_check_6, ref claimed_sums, common_lookup_elements,
            )
                .unwrap(),
            rc_8: components::range_check_8::NewComponentImpl::try_new(
                cairo_claim.range_check_8, ref claimed_sums, common_lookup_elements,
            )
                .unwrap(),
            rc_11: components::range_check_11::NewComponentImpl::try_new(
                cairo_claim.range_check_11, ref claimed_sums, common_lookup_elements,
            )
                .unwrap(),
            rc_12: components::range_check_12::NewComponentImpl::try_new(
                cairo_claim.range_check_12, ref claimed_sums, common_lookup_elements,
            )
                .unwrap(),
            rc_18: components::range_check_18::NewComponentImpl::try_new(
                cairo_claim.range_check_18, ref claimed_sums, common_lookup_elements,
            )
                .unwrap(),
            rc_20: components::range_check_20::NewComponentImpl::try_new(
                cairo_claim.range_check_20, ref claimed_sums, common_lookup_elements,
            )
                .unwrap(),
            rc_4_3: components::range_check_4_3::NewComponentImpl::try_new(
                cairo_claim.range_check_4_3, ref claimed_sums, common_lookup_elements,
            )
                .unwrap(),
            rc_4_4: components::range_check_4_4::NewComponentImpl::try_new(
                cairo_claim.range_check_4_4, ref claimed_sums, common_lookup_elements,
            )
                .unwrap(),
            rc_9_9: components::range_check_9_9::NewComponentImpl::try_new(
                cairo_claim.range_check_9_9, ref claimed_sums, common_lookup_elements,
            )
                .unwrap(),
            rc_7_2_5: components::range_check_7_2_5::NewComponentImpl::try_new(
                cairo_claim.range_check_7_2_5, ref claimed_sums, common_lookup_elements,
            )
                .unwrap(),
            rc_3_6_6_3: components::range_check_3_6_6_3::NewComponentImpl::try_new(
                cairo_claim.range_check_3_6_6_3, ref claimed_sums, common_lookup_elements,
            )
                .unwrap(),
            rc_4_4_4_4: components::range_check_4_4_4_4::NewComponentImpl::try_new(
                cairo_claim.range_check_4_4_4_4, ref claimed_sums, common_lookup_elements,
            )
                .unwrap(),
            rc_3_3_3_3_3: components::range_check_3_3_3_3_3::NewComponentImpl::try_new(
                cairo_claim.range_check_3_3_3_3_3, ref claimed_sums, common_lookup_elements,
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
                [].span(),
            );
        self
            .rc_8
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        self
            .rc_11
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        self
            .rc_12
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        self
            .rc_18
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        self
            .rc_20
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        self
            .rc_4_3
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        self
            .rc_4_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        self
            .rc_9_9
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        self
            .rc_7_2_5
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        self
            .rc_3_6_6_3
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        self
            .rc_4_4_4_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                [].span(),
            );
        self
            .rc_3_3_3_3_3
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
