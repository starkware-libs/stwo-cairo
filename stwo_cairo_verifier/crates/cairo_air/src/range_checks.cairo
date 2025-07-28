use components::range_check_11::{
    ClaimImpl as RangeCheck_11ClaimImpl, InteractionClaimImpl as RangeCheck_11InteractionClaimImpl,
};
use components::range_check_12::{
    ClaimImpl as RangeCheck_12ClaimImpl, InteractionClaimImpl as RangeCheck_12InteractionClaimImpl,
};
use components::range_check_18::{
    ClaimImpl as RangeCheck_18ClaimImpl, InteractionClaimImpl as RangeCheck_18InteractionClaimImpl,
};
use components::range_check_18_b::{
    ClaimImpl as RangeCheck_18BClaimImpl,
    InteractionClaimImpl as RangeCheck_18BInteractionClaimImpl,
};
use components::range_check_19::{
    ClaimImpl as RangeCheck_19ClaimImpl, InteractionClaimImpl as RangeCheck_19InteractionClaimImpl,
};
use components::range_check_19_b::{
    ClaimImpl as RangeCheck_19BClaimImpl,
    InteractionClaimImpl as RangeCheck_19BInteractionClaimImpl,
};
use components::range_check_19_c::{
    ClaimImpl as RangeCheck_19CClaimImpl,
    InteractionClaimImpl as RangeCheck_19CInteractionClaimImpl,
};
use components::range_check_19_d::{
    ClaimImpl as RangeCheck_19DClaimImpl,
    InteractionClaimImpl as RangeCheck_19DInteractionClaimImpl,
};
use components::range_check_19_e::{
    ClaimImpl as RangeCheck_19EClaimImpl,
    InteractionClaimImpl as RangeCheck_19EInteractionClaimImpl,
};
use components::range_check_19_f::{
    ClaimImpl as RangeCheck_19FClaimImpl,
    InteractionClaimImpl as RangeCheck_19FInteractionClaimImpl,
};
use components::range_check_19_g::{
    ClaimImpl as RangeCheck_19GClaimImpl,
    InteractionClaimImpl as RangeCheck_19GInteractionClaimImpl,
};
use components::range_check_19_h::{
    ClaimImpl as RangeCheck_19HClaimImpl,
    InteractionClaimImpl as RangeCheck_19HInteractionClaimImpl,
};
use components::range_check_3_3_3_3_3::{
    ClaimImpl as RangeCheck_3_3_3_3_3ClaimImpl,
    InteractionClaimImpl as RangeCheck_3_3_3_3_3InteractionClaimImpl,
};
use components::range_check_3_6_6_3::{
    ClaimImpl as RangeCheck_3_6_6_3ClaimImpl,
    InteractionClaimImpl as RangeCheck_3_6_6_3InteractionClaimImpl,
};
use components::range_check_4_3::{
    ClaimImpl as RangeCheck_4_3ClaimImpl,
    InteractionClaimImpl as RangeCheck_4_3InteractionClaimImpl,
};
use components::range_check_4_4::{
    ClaimImpl as RangeCheck_4_4ClaimImpl,
    InteractionClaimImpl as RangeCheck_4_4InteractionClaimImpl,
};
use components::range_check_4_4_4_4::{
    ClaimImpl as RangeCheck_4_4_4_4ClaimImpl,
    InteractionClaimImpl as RangeCheck_4_4_4_4InteractionClaimImpl,
};
use components::range_check_5_4::{
    ClaimImpl as RangeCheck_5_4ClaimImpl,
    InteractionClaimImpl as RangeCheck_5_4InteractionClaimImpl,
};
use components::range_check_6::{
    ClaimImpl as RangeCheck_6ClaimImpl, InteractionClaimImpl as RangeCheck_6InteractionClaimImpl,
};
use components::range_check_7_2_5::{
    ClaimImpl as RangeCheck_7_2_5ClaimImpl,
    InteractionClaimImpl as RangeCheck_7_2_5InteractionClaimImpl,
};
use components::range_check_8::{
    ClaimImpl as RangeCheck_8ClaimImpl, InteractionClaimImpl as RangeCheck_8InteractionClaimImpl,
};
use components::range_check_9_9::{
    ClaimImpl as RangeCheck_9_9ClaimImpl,
    InteractionClaimImpl as RangeCheck_9_9InteractionClaimImpl,
};
use components::range_check_9_9_b::{
    ClaimImpl as RangeCheck_9_9BClaimImpl,
    InteractionClaimImpl as RangeCheck_9_9BInteractionClaimImpl,
};
use components::range_check_9_9_c::{
    ClaimImpl as RangeCheck_9_9CClaimImpl,
    InteractionClaimImpl as RangeCheck_9_9CInteractionClaimImpl,
};
use components::range_check_9_9_d::{
    ClaimImpl as RangeCheck_9_9DClaimImpl,
    InteractionClaimImpl as RangeCheck_9_9DInteractionClaimImpl,
};
use components::range_check_9_9_e::{
    ClaimImpl as RangeCheck_9_9EClaimImpl,
    InteractionClaimImpl as RangeCheck_9_9EInteractionClaimImpl,
};
use components::range_check_9_9_f::{
    ClaimImpl as RangeCheck_9_9FClaimImpl,
    InteractionClaimImpl as RangeCheck_9_9FInteractionClaimImpl,
};
use components::range_check_9_9_g::{
    ClaimImpl as RangeCheck_9_9GClaimImpl,
    InteractionClaimImpl as RangeCheck_9_9GInteractionClaimImpl,
};
use components::range_check_9_9_h::{
    ClaimImpl as RangeCheck_9_9HClaimImpl,
    InteractionClaimImpl as RangeCheck_9_9HInteractionClaimImpl,
};
use components::range_check_builtin_bits_128::{
    ClaimImpl as RangeCheckBuiltinBits128ClaimImpl,
    InteractionClaimImpl as RangeCheckBuiltinBits128InteractionClaimImpl,
};
use components::range_check_builtin_bits_96::{
    ClaimImpl as RangeCheckBuiltinBits96ClaimImpl,
    InteractionClaimImpl as RangeCheckBuiltinBits96InteractionClaimImpl,
};
use components::range_check_felt_252_width_27::{
    ClaimImpl as RangeCheckFelt252Width27ClaimImpl,
    InteractionClaimImpl as RangeCheckFelt252Width27InteractionClaimImpl,
};
use core::box::BoxImpl;
use core::num::traits::Zero;
use stwo_cairo_air::cairo_component::CairoComponent;
use stwo_cairo_air::{
    CairoInteractionElements, RelationUsesDict, accumulate_relation_uses, components, utils,
};
use stwo_constraint_framework::{
    LookupElementsImpl, PreprocessedColumnImpl, PreprocessedColumnKey, PreprocessedColumnSet,
    PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelImpl};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::utils::{ArrayImpl, OptionImpl};
use stwo_verifier_core::{ColumnSpan, TreeArray};

pub mod range_check_elements {
    use stwo_constraint_framework::LookupElements;
    pub type RangeCheck_6Elements = LookupElements<1>;

    pub type RangeCheck_8Elements = LookupElements<1>;

    pub type RangeCheck_11Elements = LookupElements<1>;

    pub type RangeCheck_12Elements = LookupElements<1>;

    pub type RangeCheck_18Elements = LookupElements<1>;
    pub type RangeCheck_18_BElements = LookupElements<1>;

    pub type RangeCheck_19Elements = LookupElements<1>;
    pub type RangeCheck_19_BElements = LookupElements<1>;
    pub type RangeCheck_19_CElements = LookupElements<1>;
    pub type RangeCheck_19_DElements = LookupElements<1>;
    pub type RangeCheck_19_EElements = LookupElements<1>;
    pub type RangeCheck_19_FElements = LookupElements<1>;
    pub type RangeCheck_19_GElements = LookupElements<1>;
    pub type RangeCheck_19_HElements = LookupElements<1>;

    pub type RangeCheck_9_9Elements = LookupElements<2>;
    pub type RangeCheck_9_9_BElements = LookupElements<2>;
    pub type RangeCheck_9_9_CElements = LookupElements<2>;
    pub type RangeCheck_9_9_DElements = LookupElements<2>;
    pub type RangeCheck_9_9_EElements = LookupElements<2>;
    pub type RangeCheck_9_9_FElements = LookupElements<2>;
    pub type RangeCheck_9_9_GElements = LookupElements<2>;
    pub type RangeCheck_9_9_HElements = LookupElements<2>;

    pub type RangeCheck_4_3Elements = LookupElements<2>;

    pub type RangeCheck_4_4Elements = LookupElements<2>;

    pub type RangeCheck_5_4Elements = LookupElements<2>;

    pub type RangeCheck_7_2_5Elements = LookupElements<3>;

    pub type RangeCheck_3_6_6_3Elements = LookupElements<4>;

    pub type RangeCheck_4_4_4_4Elements = LookupElements<4>;

    pub type RangeCheck_3_3_3_3_3Elements = LookupElements<5>;
}
use range_check_elements::*;


#[derive(Drop, Serde, Clone)]
pub struct RangeChecksClaim {
    pub rc_6: components::range_check_6::Claim,
    pub rc_8: components::range_check_8::Claim,
    pub rc_11: components::range_check_11::Claim,
    pub rc_12: components::range_check_12::Claim,
    pub rc_18: components::range_check_18::Claim,
    pub rc_18_b: components::range_check_18_b::Claim,
    pub rc_19: components::range_check_19::Claim,
    pub rc_19_b: components::range_check_19_b::Claim,
    pub rc_19_c: components::range_check_19_c::Claim,
    pub rc_19_d: components::range_check_19_d::Claim,
    pub rc_19_e: components::range_check_19_e::Claim,
    pub rc_19_f: components::range_check_19_f::Claim,
    pub rc_19_g: components::range_check_19_g::Claim,
    pub rc_19_h: components::range_check_19_h::Claim,
    pub rc_4_3: components::range_check_4_3::Claim,
    pub rc_4_4: components::range_check_4_4::Claim,
    pub rc_5_4: components::range_check_5_4::Claim,
    pub rc_9_9: components::range_check_9_9::Claim,
    pub rc_9_9_b: components::range_check_9_9_b::Claim,
    pub rc_9_9_c: components::range_check_9_9_c::Claim,
    pub rc_9_9_d: components::range_check_9_9_d::Claim,
    pub rc_9_9_e: components::range_check_9_9_e::Claim,
    pub rc_9_9_f: components::range_check_9_9_f::Claim,
    pub rc_9_9_g: components::range_check_9_9_g::Claim,
    pub rc_9_9_h: components::range_check_9_9_h::Claim,
    pub rc_7_2_5: components::range_check_7_2_5::Claim,
    pub rc_3_6_6_3: components::range_check_3_6_6_3::Claim,
    pub rc_4_4_4_4: components::range_check_4_4_4_4::Claim,
    pub rc_3_3_3_3_3: components::range_check_3_3_3_3_3::Claim,
}

#[generate_trait]
pub impl RangeChecksClaimImpl of RangeChecksClaimTrait {
    fn mix_into(self: @RangeChecksClaim, ref channel: Channel) {
        self.rc_6.mix_into(ref channel);
        self.rc_8.mix_into(ref channel);
        self.rc_11.mix_into(ref channel);
        self.rc_12.mix_into(ref channel);
        self.rc_18.mix_into(ref channel);
        self.rc_18_b.mix_into(ref channel);
        self.rc_19.mix_into(ref channel);
        self.rc_19_b.mix_into(ref channel);
        self.rc_19_c.mix_into(ref channel);
        self.rc_19_d.mix_into(ref channel);
        self.rc_19_e.mix_into(ref channel);
        self.rc_19_f.mix_into(ref channel);
        self.rc_19_g.mix_into(ref channel);
        self.rc_19_h.mix_into(ref channel);
        self.rc_4_3.mix_into(ref channel);
        self.rc_4_4.mix_into(ref channel);
        self.rc_5_4.mix_into(ref channel);
        self.rc_9_9.mix_into(ref channel);
        self.rc_9_9_b.mix_into(ref channel);
        self.rc_9_9_c.mix_into(ref channel);
        self.rc_9_9_d.mix_into(ref channel);
        self.rc_9_9_e.mix_into(ref channel);
        self.rc_9_9_f.mix_into(ref channel);
        self.rc_9_9_g.mix_into(ref channel);
        self.rc_9_9_h.mix_into(ref channel);
        self.rc_7_2_5.mix_into(ref channel);
        self.rc_3_6_6_3.mix_into(ref channel);
        self.rc_4_4_4_4.mix_into(ref channel);
        self.rc_3_3_3_3_3.mix_into(ref channel);
    }

    fn log_sizes(self: @RangeChecksClaim) -> TreeArray<Span<u32>> {
        utils::tree_array_concat_cols(
            array![
                self.rc_6.log_sizes(), self.rc_8.log_sizes(), self.rc_11.log_sizes(),
                self.rc_12.log_sizes(), self.rc_18.log_sizes(), self.rc_18_b.log_sizes(),
                self.rc_19.log_sizes(), self.rc_19_b.log_sizes(), self.rc_19_c.log_sizes(),
                self.rc_19_d.log_sizes(), self.rc_19_e.log_sizes(), self.rc_19_f.log_sizes(),
                self.rc_19_g.log_sizes(), self.rc_19_h.log_sizes(), self.rc_4_3.log_sizes(),
                self.rc_4_4.log_sizes(), self.rc_5_4.log_sizes(), self.rc_9_9.log_sizes(),
                self.rc_9_9_b.log_sizes(), self.rc_9_9_c.log_sizes(), self.rc_9_9_d.log_sizes(),
                self.rc_9_9_e.log_sizes(), self.rc_9_9_f.log_sizes(), self.rc_9_9_g.log_sizes(),
                self.rc_9_9_h.log_sizes(), self.rc_7_2_5.log_sizes(), self.rc_3_6_6_3.log_sizes(),
                self.rc_4_4_4_4.log_sizes(), self.rc_3_3_3_3_3.log_sizes(),
            ],
        )
    }
}


#[derive(Drop, Serde, Clone)]
pub struct RangeChecksInteractionClaim {
    pub rc_6: components::range_check_6::InteractionClaim,
    pub rc_8: components::range_check_8::InteractionClaim,
    pub rc_11: components::range_check_11::InteractionClaim,
    pub rc_12: components::range_check_12::InteractionClaim,
    pub rc_18: components::range_check_18::InteractionClaim,
    pub rc_18_b: components::range_check_18_b::InteractionClaim,
    pub rc_19: components::range_check_19::InteractionClaim,
    pub rc_19_b: components::range_check_19_b::InteractionClaim,
    pub rc_19_c: components::range_check_19_c::InteractionClaim,
    pub rc_19_d: components::range_check_19_d::InteractionClaim,
    pub rc_19_e: components::range_check_19_e::InteractionClaim,
    pub rc_19_f: components::range_check_19_f::InteractionClaim,
    pub rc_19_g: components::range_check_19_g::InteractionClaim,
    pub rc_19_h: components::range_check_19_h::InteractionClaim,
    pub rc_4_3: components::range_check_4_3::InteractionClaim,
    pub rc_4_4: components::range_check_4_4::InteractionClaim,
    pub rc_5_4: components::range_check_5_4::InteractionClaim,
    pub rc_9_9: components::range_check_9_9::InteractionClaim,
    pub rc_9_9_b: components::range_check_9_9_b::InteractionClaim,
    pub rc_9_9_c: components::range_check_9_9_c::InteractionClaim,
    pub rc_9_9_d: components::range_check_9_9_d::InteractionClaim,
    pub rc_9_9_e: components::range_check_9_9_e::InteractionClaim,
    pub rc_9_9_f: components::range_check_9_9_f::InteractionClaim,
    pub rc_9_9_g: components::range_check_9_9_g::InteractionClaim,
    pub rc_9_9_h: components::range_check_9_9_h::InteractionClaim,
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
        self.rc_18_b.mix_into(ref channel);
        self.rc_19.mix_into(ref channel);
        self.rc_19_b.mix_into(ref channel);
        self.rc_19_c.mix_into(ref channel);
        self.rc_19_d.mix_into(ref channel);
        self.rc_19_e.mix_into(ref channel);
        self.rc_19_f.mix_into(ref channel);
        self.rc_19_g.mix_into(ref channel);
        self.rc_19_h.mix_into(ref channel);
        self.rc_4_3.mix_into(ref channel);
        self.rc_4_4.mix_into(ref channel);
        self.rc_5_4.mix_into(ref channel);
        self.rc_9_9.mix_into(ref channel);
        self.rc_9_9_b.mix_into(ref channel);
        self.rc_9_9_c.mix_into(ref channel);
        self.rc_9_9_d.mix_into(ref channel);
        self.rc_9_9_e.mix_into(ref channel);
        self.rc_9_9_f.mix_into(ref channel);
        self.rc_9_9_g.mix_into(ref channel);
        self.rc_9_9_h.mix_into(ref channel);
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
        sum += *self.rc_18_b.claimed_sum;
        sum += *self.rc_19.claimed_sum;
        sum += *self.rc_19_b.claimed_sum;
        sum += *self.rc_19_c.claimed_sum;
        sum += *self.rc_19_d.claimed_sum;
        sum += *self.rc_19_e.claimed_sum;
        sum += *self.rc_19_f.claimed_sum;
        sum += *self.rc_19_g.claimed_sum;
        sum += *self.rc_19_h.claimed_sum;
        sum += *self.rc_4_3.claimed_sum;
        sum += *self.rc_4_4.claimed_sum;
        sum += *self.rc_5_4.claimed_sum;
        sum += *self.rc_9_9.claimed_sum;
        sum += *self.rc_9_9_b.claimed_sum;
        sum += *self.rc_9_9_c.claimed_sum;
        sum += *self.rc_9_9_d.claimed_sum;
        sum += *self.rc_9_9_e.claimed_sum;
        sum += *self.rc_9_9_f.claimed_sum;
        sum += *self.rc_9_9_g.claimed_sum;
        sum += *self.rc_9_9_h.claimed_sum;
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
    rc_18_b: components::range_check_18_b::Component,
    rc_19: components::range_check_19::Component,
    rc_19_b: components::range_check_19_b::Component,
    rc_19_c: components::range_check_19_c::Component,
    rc_19_d: components::range_check_19_d::Component,
    rc_19_e: components::range_check_19_e::Component,
    rc_19_f: components::range_check_19_f::Component,
    rc_19_g: components::range_check_19_g::Component,
    rc_19_h: components::range_check_19_h::Component,
    rc_4_3: components::range_check_4_3::Component,
    rc_4_4: components::range_check_4_4::Component,
    rc_5_4: components::range_check_5_4::Component,
    rc_9_9: components::range_check_9_9::Component,
    rc_9_9_b: components::range_check_9_9_b::Component,
    rc_9_9_c: components::range_check_9_9_c::Component,
    rc_9_9_d: components::range_check_9_9_d::Component,
    rc_9_9_e: components::range_check_9_9_e::Component,
    rc_9_9_f: components::range_check_9_9_f::Component,
    rc_9_9_g: components::range_check_9_9_g::Component,
    rc_9_9_h: components::range_check_9_9_h::Component,
    rc_7_2_5: components::range_check_7_2_5::Component,
    rc_3_6_6_3: components::range_check_3_6_6_3::Component,
    rc_4_4_4_4: components::range_check_4_4_4_4::Component,
    rc_3_3_3_3_3: components::range_check_3_3_3_3_3::Component,
}

#[generate_trait]
pub impl RangeChecksComponentsImpl of RangeChecksComponentsTrait {
    fn new(
        claim: @RangeChecksClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @RangeChecksInteractionClaim,
    ) -> RangeChecksComponents {
        RangeChecksComponents {
            rc_6: components::range_check_6::Component {
                claim: *claim.rc_6,
                interaction_claim: *interaction_claim.rc_6,
                range_check_6_lookup_elements: interaction_elements.range_checks.rc_6.clone(),
            },
            rc_8: components::range_check_8::Component {
                claim: *claim.rc_8,
                interaction_claim: *interaction_claim.rc_8,
                range_check_8_lookup_elements: interaction_elements.range_checks.rc_8.clone(),
            },
            rc_11: components::range_check_11::Component {
                claim: *claim.rc_11,
                interaction_claim: *interaction_claim.rc_11,
                range_check_11_lookup_elements: interaction_elements.range_checks.rc_11.clone(),
            },
            rc_12: components::range_check_12::Component {
                claim: *claim.rc_12,
                interaction_claim: *interaction_claim.rc_12,
                range_check_12_lookup_elements: interaction_elements.range_checks.rc_12.clone(),
            },
            rc_18: components::range_check_18::Component {
                claim: *claim.rc_18,
                interaction_claim: *interaction_claim.rc_18,
                range_check_18_lookup_elements: interaction_elements.range_checks.rc_18.clone(),
            },
            rc_18_b: components::range_check_18_b::Component {
                claim: *claim.rc_18_b,
                interaction_claim: *interaction_claim.rc_18_b,
                range_check_18_b_lookup_elements: interaction_elements.range_checks.rc_18_b.clone(),
            },
            rc_19: components::range_check_19::Component {
                claim: *claim.rc_19,
                interaction_claim: *interaction_claim.rc_19,
                range_check_19_lookup_elements: interaction_elements.range_checks.rc_19.clone(),
            },
            rc_19_b: components::range_check_19_b::Component {
                claim: *claim.rc_19_b,
                interaction_claim: *interaction_claim.rc_19_b,
                range_check_19_b_lookup_elements: interaction_elements.range_checks.rc_19_b.clone(),
            },
            rc_19_c: components::range_check_19_c::Component {
                claim: *claim.rc_19_c,
                interaction_claim: *interaction_claim.rc_19_c,
                range_check_19_c_lookup_elements: interaction_elements.range_checks.rc_19_c.clone(),
            },
            rc_19_d: components::range_check_19_d::Component {
                claim: *claim.rc_19_d,
                interaction_claim: *interaction_claim.rc_19_d,
                range_check_19_d_lookup_elements: interaction_elements.range_checks.rc_19_d.clone(),
            },
            rc_19_e: components::range_check_19_e::Component {
                claim: *claim.rc_19_e,
                interaction_claim: *interaction_claim.rc_19_e,
                range_check_19_e_lookup_elements: interaction_elements.range_checks.rc_19_e.clone(),
            },
            rc_19_f: components::range_check_19_f::Component {
                claim: *claim.rc_19_f,
                interaction_claim: *interaction_claim.rc_19_f,
                range_check_19_f_lookup_elements: interaction_elements.range_checks.rc_19_f.clone(),
            },
            rc_19_g: components::range_check_19_g::Component {
                claim: *claim.rc_19_g,
                interaction_claim: *interaction_claim.rc_19_g,
                range_check_19_g_lookup_elements: interaction_elements.range_checks.rc_19_g.clone(),
            },
            rc_19_h: components::range_check_19_h::Component {
                claim: *claim.rc_19_h,
                interaction_claim: *interaction_claim.rc_19_h,
                range_check_19_h_lookup_elements: interaction_elements.range_checks.rc_19_h.clone(),
            },
            rc_4_3: components::range_check_4_3::Component {
                claim: *claim.rc_4_3,
                interaction_claim: *interaction_claim.rc_4_3,
                range_check_4_3_lookup_elements: interaction_elements.range_checks.rc_4_3.clone(),
            },
            rc_4_4: components::range_check_4_4::Component {
                claim: *claim.rc_4_4,
                interaction_claim: *interaction_claim.rc_4_4,
                range_check_4_4_lookup_elements: interaction_elements.range_checks.rc_4_4.clone(),
            },
            rc_5_4: components::range_check_5_4::Component {
                claim: *claim.rc_5_4,
                interaction_claim: *interaction_claim.rc_5_4,
                range_check_5_4_lookup_elements: interaction_elements.range_checks.rc_5_4.clone(),
            },
            rc_9_9: components::range_check_9_9::Component {
                claim: *claim.rc_9_9,
                interaction_claim: *interaction_claim.rc_9_9,
                range_check_9_9_lookup_elements: interaction_elements.range_checks.rc_9_9.clone(),
            },
            rc_9_9_b: components::range_check_9_9_b::Component {
                claim: *claim.rc_9_9_b,
                interaction_claim: *interaction_claim.rc_9_9_b,
                range_check_9_9_b_lookup_elements: interaction_elements
                    .range_checks
                    .rc_9_9_b
                    .clone(),
            },
            rc_9_9_c: components::range_check_9_9_c::Component {
                claim: *claim.rc_9_9_c,
                interaction_claim: *interaction_claim.rc_9_9_c,
                range_check_9_9_c_lookup_elements: interaction_elements
                    .range_checks
                    .rc_9_9_c
                    .clone(),
            },
            rc_9_9_d: components::range_check_9_9_d::Component {
                claim: *claim.rc_9_9_d,
                interaction_claim: *interaction_claim.rc_9_9_d,
                range_check_9_9_d_lookup_elements: interaction_elements
                    .range_checks
                    .rc_9_9_d
                    .clone(),
            },
            rc_9_9_e: components::range_check_9_9_e::Component {
                claim: *claim.rc_9_9_e,
                interaction_claim: *interaction_claim.rc_9_9_e,
                range_check_9_9_e_lookup_elements: interaction_elements
                    .range_checks
                    .rc_9_9_e
                    .clone(),
            },
            rc_9_9_f: components::range_check_9_9_f::Component {
                claim: *claim.rc_9_9_f,
                interaction_claim: *interaction_claim.rc_9_9_f,
                range_check_9_9_f_lookup_elements: interaction_elements
                    .range_checks
                    .rc_9_9_f
                    .clone(),
            },
            rc_9_9_g: components::range_check_9_9_g::Component {
                claim: *claim.rc_9_9_g,
                interaction_claim: *interaction_claim.rc_9_9_g,
                range_check_9_9_g_lookup_elements: interaction_elements
                    .range_checks
                    .rc_9_9_g
                    .clone(),
            },
            rc_9_9_h: components::range_check_9_9_h::Component {
                claim: *claim.rc_9_9_h,
                interaction_claim: *interaction_claim.rc_9_9_h,
                range_check_9_9_h_lookup_elements: interaction_elements
                    .range_checks
                    .rc_9_9_h
                    .clone(),
            },
            rc_7_2_5: components::range_check_7_2_5::Component {
                claim: *claim.rc_7_2_5,
                interaction_claim: *interaction_claim.rc_7_2_5,
                range_check_7_2_5_lookup_elements: interaction_elements
                    .range_checks
                    .rc_7_2_5
                    .clone(),
            },
            rc_3_6_6_3: components::range_check_3_6_6_3::Component {
                claim: *claim.rc_3_6_6_3,
                interaction_claim: *interaction_claim.rc_3_6_6_3,
                range_check_3_6_6_3_lookup_elements: interaction_elements
                    .range_checks
                    .rc_3_6_6_3
                    .clone(),
            },
            rc_4_4_4_4: components::range_check_4_4_4_4::Component {
                claim: *claim.rc_4_4_4_4,
                interaction_claim: *interaction_claim.rc_4_4_4_4,
                range_check_4_4_4_4_lookup_elements: interaction_elements
                    .range_checks
                    .rc_4_4_4_4
                    .clone(),
            },
            rc_3_3_3_3_3: components::range_check_3_3_3_3_3::Component {
                claim: *claim.rc_3_3_3_3_3,
                interaction_claim: *interaction_claim.rc_3_3_3_3_3,
                range_check_3_3_3_3_3_lookup_elements: interaction_elements
                    .range_checks
                    .rc_3_3_3_3_3
                    .clone(),
            },
        }
    }

    fn mask_points(
        self: @RangeChecksComponents,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        self
            .rc_6
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_8
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_11
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_12
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_18
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_18_b
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_19
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_19_b
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_19_c
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_19_d
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_19_e
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_19_f
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_19_g
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_19_h
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_4_3
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_4_4
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_5_4
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_9_9
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_9_9_b
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_9_9_c
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_9_9_d
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_9_9_e
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_9_9_f
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_9_9_g
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_9_9_h
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_7_2_5
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_3_6_6_3
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_4_4_4_4
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_3_3_3_3_3
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
    }

    fn max_constraint_log_degree_bound(self: @RangeChecksComponents) -> u32 {
        let mut max_degree = 0;
        max_degree = core::cmp::max(max_degree, self.rc_6.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_8.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_11.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_12.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_18.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_18_b.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_19.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_19_b.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_19_c.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_19_d.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_19_e.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_19_f.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_19_g.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_19_h.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_4_3.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_4_4.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_5_4.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_9_9.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_9_9_b.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_9_9_c.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_9_9_d.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_9_9_e.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_9_9_f.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_9_9_g.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_9_9_h.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_7_2_5.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_3_6_6_3.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_4_4_4_4.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.rc_3_3_3_3_3.max_constraint_log_degree_bound());
        max_degree
    }

    fn evaluate_constraints_at_point(
        self: @RangeChecksComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        self
            .rc_6
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_8
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_11
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_12
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_18
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_18_b
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_19
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_19_b
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_19_c
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_19_d
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_19_e
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_19_f
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_19_g
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_19_h
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_4_3
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_4_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_5_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_9_9
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_9_9_b
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_9_9_c
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_9_9_d
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_9_9_e
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_9_9_f
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_9_9_g
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_9_9_h
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_7_2_5
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_3_6_6_3
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_4_4_4_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_3_3_3_3_3
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


#[derive(Drop)]
pub struct RangeChecksInteractionElements {
    pub rc_6: RangeCheck_6Elements,
    pub rc_8: RangeCheck_8Elements,
    pub rc_11: RangeCheck_11Elements,
    pub rc_12: RangeCheck_12Elements,
    pub rc_18: RangeCheck_18Elements,
    pub rc_18_b: RangeCheck_18Elements,
    pub rc_19: RangeCheck_19Elements,
    pub rc_19_b: RangeCheck_19Elements,
    pub rc_19_c: RangeCheck_19Elements,
    pub rc_19_d: RangeCheck_19Elements,
    pub rc_19_e: RangeCheck_19Elements,
    pub rc_19_f: RangeCheck_19Elements,
    pub rc_19_g: RangeCheck_19Elements,
    pub rc_19_h: RangeCheck_19Elements,
    pub rc_4_3: RangeCheck_4_3Elements,
    pub rc_4_4: RangeCheck_4_4Elements,
    pub rc_5_4: RangeCheck_5_4Elements,
    pub rc_9_9: RangeCheck_9_9Elements,
    pub rc_9_9_b: RangeCheck_9_9Elements,
    pub rc_9_9_c: RangeCheck_9_9Elements,
    pub rc_9_9_d: RangeCheck_9_9Elements,
    pub rc_9_9_e: RangeCheck_9_9Elements,
    pub rc_9_9_f: RangeCheck_9_9Elements,
    pub rc_9_9_g: RangeCheck_9_9Elements,
    pub rc_9_9_h: RangeCheck_9_9Elements,
    pub rc_7_2_5: RangeCheck_7_2_5Elements,
    pub rc_3_6_6_3: RangeCheck_3_6_6_3Elements,
    pub rc_4_4_4_4: RangeCheck_4_4_4_4Elements,
    pub rc_3_3_3_3_3: RangeCheck_3_3_3_3_3Elements,
}

#[generate_trait]
pub impl RangeChecksInteractionElementsImpl of RangeChecksInteractionElementsTrait {
    fn draw(ref channel: Channel) -> RangeChecksInteractionElements {
        RangeChecksInteractionElements {
            rc_6: LookupElementsImpl::draw(ref channel),
            rc_8: LookupElementsImpl::draw(ref channel),
            rc_11: LookupElementsImpl::draw(ref channel),
            rc_12: LookupElementsImpl::draw(ref channel),
            rc_18: LookupElementsImpl::draw(ref channel),
            rc_18_b: LookupElementsImpl::draw(ref channel),
            rc_19: LookupElementsImpl::draw(ref channel),
            rc_19_b: LookupElementsImpl::draw(ref channel),
            rc_19_c: LookupElementsImpl::draw(ref channel),
            rc_19_d: LookupElementsImpl::draw(ref channel),
            rc_19_e: LookupElementsImpl::draw(ref channel),
            rc_19_f: LookupElementsImpl::draw(ref channel),
            rc_19_g: LookupElementsImpl::draw(ref channel),
            rc_19_h: LookupElementsImpl::draw(ref channel),
            rc_4_3: LookupElementsImpl::draw(ref channel),
            rc_4_4: LookupElementsImpl::draw(ref channel),
            rc_5_4: LookupElementsImpl::draw(ref channel),
            rc_9_9: LookupElementsImpl::draw(ref channel),
            rc_9_9_b: LookupElementsImpl::draw(ref channel),
            rc_9_9_c: LookupElementsImpl::draw(ref channel),
            rc_9_9_d: LookupElementsImpl::draw(ref channel),
            rc_9_9_e: LookupElementsImpl::draw(ref channel),
            rc_9_9_f: LookupElementsImpl::draw(ref channel),
            rc_9_9_g: LookupElementsImpl::draw(ref channel),
            rc_9_9_h: LookupElementsImpl::draw(ref channel),
            rc_7_2_5: LookupElementsImpl::draw(ref channel),
            rc_3_6_6_3: LookupElementsImpl::draw(ref channel),
            rc_4_4_4_4: LookupElementsImpl::draw(ref channel),
            rc_3_3_3_3_3: LookupElementsImpl::draw(ref channel),
        }
    }
}
