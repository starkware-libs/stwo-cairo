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
use core::box::BoxImpl;
use core::num::traits::Zero;
use stwo_cairo_air::cairo_component::CairoComponent;
use stwo_cairo_air::claim::ClaimTrait;
use stwo_cairo_air::{RelationUsesDict, components, utils};
use stwo_constraint_framework::{
    CommonLookupElements, LookupElementsImpl, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::Channel;
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::utils::{ArrayImpl, OptionImpl};
use stwo_verifier_core::{ColumnSpan, TreeArray};

#[derive(Drop, Serde, Clone)]
pub struct RangeChecksClaim {
    pub rc_6: components::range_check_6::Claim,
    pub rc_8: components::range_check_8::Claim,
    pub rc_11: components::range_check_11::Claim,
    pub rc_12: components::range_check_12::Claim,
    pub rc_18: components::range_check_18::Claim,
    pub rc_20: components::range_check_20::Claim,
    pub rc_4_3: components::range_check_4_3::Claim,
    pub rc_4_4: components::range_check_4_4::Claim,
    pub rc_9_9: components::range_check_9_9::Claim,
    pub rc_7_2_5: components::range_check_7_2_5::Claim,
    pub rc_3_6_6_3: components::range_check_3_6_6_3::Claim,
    pub rc_4_4_4_4: components::range_check_4_4_4_4::Claim,
    pub rc_3_3_3_3_3: components::range_check_3_3_3_3_3::Claim,
}

pub impl RangeChecksClaimImpl of ClaimTrait<RangeChecksClaim> {
    fn mix_into(self: @RangeChecksClaim, ref channel: Channel) {
        self.rc_6.mix_into(ref channel);
        self.rc_8.mix_into(ref channel);
        self.rc_11.mix_into(ref channel);
        self.rc_12.mix_into(ref channel);
        self.rc_18.mix_into(ref channel);
        self.rc_20.mix_into(ref channel);
        self.rc_4_3.mix_into(ref channel);
        self.rc_4_4.mix_into(ref channel);
        self.rc_9_9.mix_into(ref channel);
        self.rc_7_2_5.mix_into(ref channel);
        self.rc_3_6_6_3.mix_into(ref channel);
        self.rc_4_4_4_4.mix_into(ref channel);
        self.rc_3_3_3_3_3.mix_into(ref channel);
    }

    fn log_sizes(self: @RangeChecksClaim) -> TreeArray<Span<u32>> {
        utils::tree_array_concat_cols(
            array![
                self.rc_6.log_sizes(), self.rc_8.log_sizes(), self.rc_11.log_sizes(),
                self.rc_12.log_sizes(), self.rc_18.log_sizes(), self.rc_20.log_sizes(),
                self.rc_4_3.log_sizes(), self.rc_4_4.log_sizes(), self.rc_9_9.log_sizes(),
                self.rc_7_2_5.log_sizes(), self.rc_3_6_6_3.log_sizes(), self.rc_4_4_4_4.log_sizes(),
                self.rc_3_3_3_3_3.log_sizes(),
            ],
        )
    }
    // Range checks components do not contribute to relations "uses".
    fn accumulate_relation_uses(self: @RangeChecksClaim, ref relation_uses: RelationUsesDict) {}
}


#[derive(Drop, Serde, Clone)]
pub struct RangeChecksInteractionClaim {
    pub rc_6: components::range_check_6::InteractionClaim,
    pub rc_8: components::range_check_8::InteractionClaim,
    pub rc_11: components::range_check_11::InteractionClaim,
    pub rc_12: components::range_check_12::InteractionClaim,
    pub rc_18: components::range_check_18::InteractionClaim,
    pub rc_20: components::range_check_20::InteractionClaim,
    pub rc_4_3: components::range_check_4_3::InteractionClaim,
    pub rc_4_4: components::range_check_4_4::InteractionClaim,
    pub rc_9_9: components::range_check_9_9::InteractionClaim,
    pub rc_7_2_5: components::range_check_7_2_5::InteractionClaim,
    pub rc_3_6_6_3: components::range_check_3_6_6_3::InteractionClaim,
    pub rc_4_4_4_4: components::range_check_4_4_4_4::InteractionClaim,
    pub rc_3_3_3_3_3: components::range_check_3_3_3_3_3::InteractionClaim,
}

#[generate_trait]
pub impl RangeChecksInteractionClaimImpl of RangeChecksInteractionClaimTrait {
    fn mix_into(self: @RangeChecksInteractionClaim, ref channel: Channel) {
        self.rc_6.mix_into(ref channel);
        self.rc_8.mix_into(ref channel);
        self.rc_11.mix_into(ref channel);
        self.rc_12.mix_into(ref channel);
        self.rc_18.mix_into(ref channel);
        self.rc_20.mix_into(ref channel);
        self.rc_4_3.mix_into(ref channel);
        self.rc_4_4.mix_into(ref channel);
        self.rc_9_9.mix_into(ref channel);
        self.rc_7_2_5.mix_into(ref channel);
        self.rc_3_6_6_3.mix_into(ref channel);
        self.rc_4_4_4_4.mix_into(ref channel);
        self.rc_3_3_3_3_3.mix_into(ref channel);
    }

    fn sum(self: @RangeChecksInteractionClaim) -> QM31 {
        let mut sum = Zero::zero();
        sum += *self.rc_6.claimed_sum;
        sum += *self.rc_8.claimed_sum;
        sum += *self.rc_11.claimed_sum;
        sum += *self.rc_12.claimed_sum;
        sum += *self.rc_18.claimed_sum;
        sum += *self.rc_20.claimed_sum;
        sum += *self.rc_4_3.claimed_sum;
        sum += *self.rc_4_4.claimed_sum;
        sum += *self.rc_9_9.claimed_sum;
        sum += *self.rc_7_2_5.claimed_sum;
        sum += *self.rc_3_6_6_3.claimed_sum;
        sum += *self.rc_4_4_4_4.claimed_sum;
        sum += *self.rc_3_3_3_3_3.claimed_sum;
        sum
    }
}


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
        claim: @RangeChecksClaim,
        common_lookup_elements: @CommonLookupElements,
        interaction_claim: @RangeChecksInteractionClaim,
    ) -> RangeChecksComponents {
        RangeChecksComponents {
            rc_6: components::range_check_6::NewComponentImpl::new(
                claim.rc_6, interaction_claim.rc_6, common_lookup_elements,
            ),
            rc_8: components::range_check_8::NewComponentImpl::new(
                claim.rc_8, interaction_claim.rc_8, common_lookup_elements,
            ),
            rc_11: components::range_check_11::NewComponentImpl::new(
                claim.rc_11, interaction_claim.rc_11, common_lookup_elements,
            ),
            rc_12: components::range_check_12::NewComponentImpl::new(
                claim.rc_12, interaction_claim.rc_12, common_lookup_elements,
            ),
            rc_18: components::range_check_18::NewComponentImpl::new(
                claim.rc_18, interaction_claim.rc_18, common_lookup_elements,
            ),
            rc_20: components::range_check_20::NewComponentImpl::new(
                claim.rc_20, interaction_claim.rc_20, common_lookup_elements,
            ),
            rc_4_3: components::range_check_4_3::NewComponentImpl::new(
                claim.rc_4_3, interaction_claim.rc_4_3, common_lookup_elements,
            ),
            rc_4_4: components::range_check_4_4::NewComponentImpl::new(
                claim.rc_4_4, interaction_claim.rc_4_4, common_lookup_elements,
            ),
            rc_9_9: components::range_check_9_9::NewComponentImpl::new(
                claim.rc_9_9, interaction_claim.rc_9_9, common_lookup_elements,
            ),
            rc_7_2_5: components::range_check_7_2_5::NewComponentImpl::new(
                claim.rc_7_2_5, interaction_claim.rc_7_2_5, common_lookup_elements,
            ),
            rc_3_6_6_3: components::range_check_3_6_6_3::NewComponentImpl::new(
                claim.rc_3_6_6_3, interaction_claim.rc_3_6_6_3, common_lookup_elements,
            ),
            rc_4_4_4_4: components::range_check_4_4_4_4::NewComponentImpl::new(
                claim.rc_4_4_4_4, interaction_claim.rc_4_4_4_4, common_lookup_elements,
            ),
            rc_3_3_3_3_3: components::range_check_3_3_3_3_3::NewComponentImpl::new(
                claim.rc_3_3_3_3_3, interaction_claim.rc_3_3_3_3_3, common_lookup_elements,
            ),
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

