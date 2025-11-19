use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo::core::channel::Channel;
use stwo::core::fields::qm31::{SecureField, QM31};
use stwo::core::pcs::TreeVec;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::ComponentProver;
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};
use stwo_constraint_framework::TraceLocationAllocator;

use crate::components::{
    indented_component_display, range_check_11, range_check_12, range_check_18, range_check_18_b,
    range_check_20, range_check_20_b, range_check_20_c, range_check_20_d, range_check_20_e,
    range_check_20_f, range_check_20_g, range_check_20_h, range_check_3_3_3_3_3,
    range_check_3_6_6_3, range_check_4_3, range_check_4_4, range_check_4_4_4_4, range_check_5_4,
    range_check_6, range_check_7_2_5, range_check_8, range_check_9_9, range_check_9_9_b,
    range_check_9_9_c, range_check_9_9_d, range_check_9_9_e, range_check_9_9_f, range_check_9_9_g,
    range_check_9_9_h,
};
use crate::relations;

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct RangeChecksClaim {
    pub rc_6: range_check_6::Claim,
    pub rc_8: range_check_8::Claim,
    pub rc_11: range_check_11::Claim,
    pub rc_12: range_check_12::Claim,
    pub rc_18: range_check_18::Claim,
    pub rc_18_b: range_check_18_b::Claim,
    pub rc_20: range_check_20::Claim,
    pub rc_20_b: range_check_20_b::Claim,
    pub rc_20_c: range_check_20_c::Claim,
    pub rc_20_d: range_check_20_d::Claim,
    pub rc_20_e: range_check_20_e::Claim,
    pub rc_20_f: range_check_20_f::Claim,
    pub rc_20_g: range_check_20_g::Claim,
    pub rc_20_h: range_check_20_h::Claim,
    pub rc_4_3: range_check_4_3::Claim,
    pub rc_4_4: range_check_4_4::Claim,
    pub rc_5_4: range_check_5_4::Claim,
    pub rc_9_9: range_check_9_9::Claim,
    pub rc_9_9_b: range_check_9_9_b::Claim,
    pub rc_9_9_c: range_check_9_9_c::Claim,
    pub rc_9_9_d: range_check_9_9_d::Claim,
    pub rc_9_9_e: range_check_9_9_e::Claim,
    pub rc_9_9_f: range_check_9_9_f::Claim,
    pub rc_9_9_g: range_check_9_9_g::Claim,
    pub rc_9_9_h: range_check_9_9_h::Claim,
    pub rc_7_2_5: range_check_7_2_5::Claim,
    pub rc_3_6_6_3: range_check_3_6_6_3::Claim,
    pub rc_4_4_4_4: range_check_4_4_4_4::Claim,
    pub rc_3_3_3_3_3: range_check_3_3_3_3_3::Claim,
}
impl RangeChecksClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.rc_6.mix_into(channel);
        self.rc_8.mix_into(channel);
        self.rc_11.mix_into(channel);
        self.rc_12.mix_into(channel);
        self.rc_18.mix_into(channel);
        self.rc_18_b.mix_into(channel);
        self.rc_20.mix_into(channel);
        self.rc_20_b.mix_into(channel);
        self.rc_20_c.mix_into(channel);
        self.rc_20_d.mix_into(channel);
        self.rc_20_e.mix_into(channel);
        self.rc_20_f.mix_into(channel);
        self.rc_20_g.mix_into(channel);
        self.rc_20_h.mix_into(channel);
        self.rc_4_3.mix_into(channel);
        self.rc_4_4.mix_into(channel);
        self.rc_5_4.mix_into(channel);
        self.rc_9_9.mix_into(channel);
        self.rc_9_9_b.mix_into(channel);
        self.rc_9_9_c.mix_into(channel);
        self.rc_9_9_d.mix_into(channel);
        self.rc_9_9_e.mix_into(channel);
        self.rc_9_9_f.mix_into(channel);
        self.rc_9_9_g.mix_into(channel);
        self.rc_9_9_h.mix_into(channel);
        self.rc_7_2_5.mix_into(channel);
        self.rc_3_6_6_3.mix_into(channel);
        self.rc_4_4_4_4.mix_into(channel);
        self.rc_3_3_3_3_3.mix_into(channel);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        TreeVec::concat_cols(
            vec![
                self.rc_6.log_sizes(),
                self.rc_8.log_sizes(),
                self.rc_11.log_sizes(),
                self.rc_12.log_sizes(),
                self.rc_18.log_sizes(),
                self.rc_18_b.log_sizes(),
                self.rc_20.log_sizes(),
                self.rc_20_b.log_sizes(),
                self.rc_20_c.log_sizes(),
                self.rc_20_d.log_sizes(),
                self.rc_20_e.log_sizes(),
                self.rc_20_f.log_sizes(),
                self.rc_20_g.log_sizes(),
                self.rc_20_h.log_sizes(),
                self.rc_4_3.log_sizes(),
                self.rc_4_4.log_sizes(),
                self.rc_5_4.log_sizes(),
                self.rc_9_9.log_sizes(),
                self.rc_9_9_b.log_sizes(),
                self.rc_9_9_c.log_sizes(),
                self.rc_9_9_d.log_sizes(),
                self.rc_9_9_e.log_sizes(),
                self.rc_9_9_f.log_sizes(),
                self.rc_9_9_g.log_sizes(),
                self.rc_9_9_h.log_sizes(),
                self.rc_7_2_5.log_sizes(),
                self.rc_3_6_6_3.log_sizes(),
                self.rc_4_4_4_4.log_sizes(),
                self.rc_3_3_3_3_3.log_sizes(),
            ]
            .into_iter(),
        )
    }
}

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct RangeChecksInteractionClaim {
    pub rc_6: range_check_6::InteractionClaim,
    pub rc_8: range_check_8::InteractionClaim,
    pub rc_11: range_check_11::InteractionClaim,
    pub rc_12: range_check_12::InteractionClaim,
    pub rc_18: range_check_18::InteractionClaim,
    pub rc_18_b: range_check_18_b::InteractionClaim,
    pub rc_20: range_check_20::InteractionClaim,
    pub rc_20_b: range_check_20_b::InteractionClaim,
    pub rc_20_c: range_check_20_c::InteractionClaim,
    pub rc_20_d: range_check_20_d::InteractionClaim,
    pub rc_20_e: range_check_20_e::InteractionClaim,
    pub rc_20_f: range_check_20_f::InteractionClaim,
    pub rc_20_g: range_check_20_g::InteractionClaim,
    pub rc_20_h: range_check_20_h::InteractionClaim,
    pub rc_4_3: range_check_4_3::InteractionClaim,
    pub rc_4_4: range_check_4_4::InteractionClaim,
    pub rc_5_4: range_check_5_4::InteractionClaim,
    pub rc_9_9: range_check_9_9::InteractionClaim,
    pub rc_9_9_b: range_check_9_9_b::InteractionClaim,
    pub rc_9_9_c: range_check_9_9_c::InteractionClaim,
    pub rc_9_9_d: range_check_9_9_d::InteractionClaim,
    pub rc_9_9_e: range_check_9_9_e::InteractionClaim,
    pub rc_9_9_f: range_check_9_9_f::InteractionClaim,
    pub rc_9_9_g: range_check_9_9_g::InteractionClaim,
    pub rc_9_9_h: range_check_9_9_h::InteractionClaim,
    pub rc_7_2_5: range_check_7_2_5::InteractionClaim,
    pub rc_3_6_6_3: range_check_3_6_6_3::InteractionClaim,
    pub rc_4_4_4_4: range_check_4_4_4_4::InteractionClaim,
    pub rc_3_3_3_3_3: range_check_3_3_3_3_3::InteractionClaim,
}
impl RangeChecksInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.rc_6.mix_into(channel);
        self.rc_8.mix_into(channel);
        self.rc_11.mix_into(channel);
        self.rc_12.mix_into(channel);
        self.rc_18.mix_into(channel);
        self.rc_18_b.mix_into(channel);
        self.rc_20.mix_into(channel);
        self.rc_20_b.mix_into(channel);
        self.rc_20_c.mix_into(channel);
        self.rc_20_d.mix_into(channel);
        self.rc_20_e.mix_into(channel);
        self.rc_20_f.mix_into(channel);
        self.rc_20_g.mix_into(channel);
        self.rc_20_h.mix_into(channel);
        self.rc_4_3.mix_into(channel);
        self.rc_4_4.mix_into(channel);
        self.rc_5_4.mix_into(channel);
        self.rc_9_9.mix_into(channel);
        self.rc_9_9_b.mix_into(channel);
        self.rc_9_9_c.mix_into(channel);
        self.rc_9_9_d.mix_into(channel);
        self.rc_9_9_e.mix_into(channel);
        self.rc_9_9_f.mix_into(channel);
        self.rc_9_9_g.mix_into(channel);
        self.rc_9_9_h.mix_into(channel);
        self.rc_7_2_5.mix_into(channel);
        self.rc_3_6_6_3.mix_into(channel);
        self.rc_4_4_4_4.mix_into(channel);
        self.rc_3_3_3_3_3.mix_into(channel);
    }

    pub fn sum(&self) -> SecureField {
        let mut sum = QM31::zero();
        sum += self.rc_6.claimed_sum;
        sum += self.rc_8.claimed_sum;
        sum += self.rc_11.claimed_sum;
        sum += self.rc_12.claimed_sum;
        sum += self.rc_18.claimed_sum;
        sum += self.rc_18_b.claimed_sum;
        sum += self.rc_20.claimed_sum;
        sum += self.rc_20_b.claimed_sum;
        sum += self.rc_20_c.claimed_sum;
        sum += self.rc_20_d.claimed_sum;
        sum += self.rc_20_e.claimed_sum;
        sum += self.rc_20_f.claimed_sum;
        sum += self.rc_20_g.claimed_sum;
        sum += self.rc_20_h.claimed_sum;
        sum += self.rc_4_3.claimed_sum;
        sum += self.rc_4_4.claimed_sum;
        sum += self.rc_5_4.claimed_sum;
        sum += self.rc_9_9.claimed_sum;
        sum += self.rc_9_9_b.claimed_sum;
        sum += self.rc_9_9_c.claimed_sum;
        sum += self.rc_9_9_d.claimed_sum;
        sum += self.rc_9_9_e.claimed_sum;
        sum += self.rc_9_9_f.claimed_sum;
        sum += self.rc_9_9_g.claimed_sum;
        sum += self.rc_9_9_h.claimed_sum;
        sum += self.rc_7_2_5.claimed_sum;
        sum += self.rc_3_6_6_3.claimed_sum;
        sum += self.rc_4_4_4_4.claimed_sum;
        sum += self.rc_3_3_3_3_3.claimed_sum;
        sum
    }
}

pub struct RangeChecksComponents {
    pub rc_6: range_check_6::Component,
    pub rc_8: range_check_8::Component,
    pub rc_11: range_check_11::Component,
    pub rc_12: range_check_12::Component,
    pub rc_18: range_check_18::Component,
    pub rc_18_b: range_check_18_b::Component,
    pub rc_20: range_check_20::Component,
    pub rc_20_b: range_check_20_b::Component,
    pub rc_20_c: range_check_20_c::Component,
    pub rc_20_d: range_check_20_d::Component,
    pub rc_20_e: range_check_20_e::Component,
    pub rc_20_f: range_check_20_f::Component,
    pub rc_20_g: range_check_20_g::Component,
    pub rc_20_h: range_check_20_h::Component,
    pub rc_4_3: range_check_4_3::Component,
    pub rc_4_4: range_check_4_4::Component,
    pub rc_5_4: range_check_5_4::Component,
    pub rc_9_9: range_check_9_9::Component,
    pub rc_9_9_b: range_check_9_9_b::Component,
    pub rc_9_9_c: range_check_9_9_c::Component,
    pub rc_9_9_d: range_check_9_9_d::Component,
    pub rc_9_9_e: range_check_9_9_e::Component,
    pub rc_9_9_f: range_check_9_9_f::Component,
    pub rc_9_9_g: range_check_9_9_g::Component,
    pub rc_9_9_h: range_check_9_9_h::Component,
    pub rc_7_2_5: range_check_7_2_5::Component,
    pub rc_3_6_6_3: range_check_3_6_6_3::Component,
    pub rc_4_4_4_4: range_check_4_4_4_4::Component,
    pub rc_3_3_3_3_3: range_check_3_3_3_3_3::Component,
}
impl RangeChecksComponents {
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        common_lookup_elements: &relations::CommonLookupElements,
        interaction_claim: &RangeChecksInteractionClaim,
    ) -> Self {
        let rc_6_component = range_check_6::Component::new(
            tree_span_provider,
            range_check_6::Eval {
                claim: range_check_6::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_6.claimed_sum,
        );
        let rc_8_component = range_check_8::Component::new(
            tree_span_provider,
            range_check_8::Eval {
                claim: range_check_8::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_8.claimed_sum,
        );
        let rc_11_component = range_check_11::Component::new(
            tree_span_provider,
            range_check_11::Eval {
                claim: range_check_11::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_11.claimed_sum,
        );
        let rc_12_component = range_check_12::Component::new(
            tree_span_provider,
            range_check_12::Eval {
                claim: range_check_12::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_12.claimed_sum,
        );
        let rc_18_component = range_check_18::Component::new(
            tree_span_provider,
            range_check_18::Eval {
                claim: range_check_18::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_18.claimed_sum,
        );
        let rc_18_b_component = range_check_18_b::Component::new(
            tree_span_provider,
            range_check_18_b::Eval {
                claim: range_check_18_b::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_18_b.claimed_sum,
        );
        let rc_20_component = range_check_20::Component::new(
            tree_span_provider,
            range_check_20::Eval {
                claim: range_check_20::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_20.claimed_sum,
        );
        let rc_20_b_component = range_check_20_b::Component::new(
            tree_span_provider,
            range_check_20_b::Eval {
                claim: range_check_20_b::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_20_b.claimed_sum,
        );
        let rc_20_c_component = range_check_20_c::Component::new(
            tree_span_provider,
            range_check_20_c::Eval {
                claim: range_check_20_c::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_20_c.claimed_sum,
        );
        let rc_20_d_component = range_check_20_d::Component::new(
            tree_span_provider,
            range_check_20_d::Eval {
                claim: range_check_20_d::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_20_d.claimed_sum,
        );
        let rc_20_e_component = range_check_20_e::Component::new(
            tree_span_provider,
            range_check_20_e::Eval {
                claim: range_check_20_e::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_20_e.claimed_sum,
        );
        let rc_20_f_component = range_check_20_f::Component::new(
            tree_span_provider,
            range_check_20_f::Eval {
                claim: range_check_20_f::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_20_f.claimed_sum,
        );
        let rc_20_g_component = range_check_20_g::Component::new(
            tree_span_provider,
            range_check_20_g::Eval {
                claim: range_check_20_g::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_20_g.claimed_sum,
        );
        let rc_20_h_component = range_check_20_h::Component::new(
            tree_span_provider,
            range_check_20_h::Eval {
                claim: range_check_20_h::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_20_h.claimed_sum,
        );
        let rc_4_3_component = range_check_4_3::Component::new(
            tree_span_provider,
            range_check_4_3::Eval {
                claim: range_check_4_3::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_4_3.claimed_sum,
        );
        let rc_4_4_component = range_check_4_4::Component::new(
            tree_span_provider,
            range_check_4_4::Eval {
                claim: range_check_4_4::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_4_4.claimed_sum,
        );
        let rc_5_4_component = range_check_5_4::Component::new(
            tree_span_provider,
            range_check_5_4::Eval {
                claim: range_check_5_4::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_5_4.claimed_sum,
        );
        let rc_9_9_component = range_check_9_9::Component::new(
            tree_span_provider,
            range_check_9_9::Eval {
                claim: range_check_9_9::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_9_9.claimed_sum,
        );
        let rc_9_9_b_component = range_check_9_9_b::Component::new(
            tree_span_provider,
            range_check_9_9_b::Eval {
                claim: range_check_9_9_b::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_9_9_b.claimed_sum,
        );
        let rc_9_9_c_component = range_check_9_9_c::Component::new(
            tree_span_provider,
            range_check_9_9_c::Eval {
                claim: range_check_9_9_c::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_9_9_c.claimed_sum,
        );
        let rc_9_9_d_component = range_check_9_9_d::Component::new(
            tree_span_provider,
            range_check_9_9_d::Eval {
                claim: range_check_9_9_d::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_9_9_d.claimed_sum,
        );
        let rc_9_9_e_component = range_check_9_9_e::Component::new(
            tree_span_provider,
            range_check_9_9_e::Eval {
                claim: range_check_9_9_e::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_9_9_e.claimed_sum,
        );
        let rc_9_9_f_component = range_check_9_9_f::Component::new(
            tree_span_provider,
            range_check_9_9_f::Eval {
                claim: range_check_9_9_f::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_9_9_f.claimed_sum,
        );
        let rc_9_9_g_component = range_check_9_9_g::Component::new(
            tree_span_provider,
            range_check_9_9_g::Eval {
                claim: range_check_9_9_g::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_9_9_g.claimed_sum,
        );
        let rc_9_9_h_component = range_check_9_9_h::Component::new(
            tree_span_provider,
            range_check_9_9_h::Eval {
                claim: range_check_9_9_h::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_9_9_h.claimed_sum,
        );
        let rc_7_2_5_component = range_check_7_2_5::Component::new(
            tree_span_provider,
            range_check_7_2_5::Eval {
                claim: range_check_7_2_5::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_7_2_5.claimed_sum,
        );
        let rc_3_6_6_3_component = range_check_3_6_6_3::Component::new(
            tree_span_provider,
            range_check_3_6_6_3::Eval {
                claim: range_check_3_6_6_3::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_3_6_6_3.claimed_sum,
        );
        let rc_4_4_4_4_component = range_check_4_4_4_4::Component::new(
            tree_span_provider,
            range_check_4_4_4_4::Eval {
                claim: range_check_4_4_4_4::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_4_4_4_4.claimed_sum,
        );
        let rc_3_3_3_3_3_component = range_check_3_3_3_3_3::Component::new(
            tree_span_provider,
            range_check_3_3_3_3_3::Eval {
                claim: range_check_3_3_3_3_3::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_3_3_3_3_3.claimed_sum,
        );
        Self {
            rc_6: rc_6_component,
            rc_8: rc_8_component,
            rc_11: rc_11_component,
            rc_12: rc_12_component,
            rc_18: rc_18_component,
            rc_18_b: rc_18_b_component,
            rc_20: rc_20_component,
            rc_20_b: rc_20_b_component,
            rc_20_c: rc_20_c_component,
            rc_20_d: rc_20_d_component,
            rc_20_e: rc_20_e_component,
            rc_20_f: rc_20_f_component,
            rc_20_g: rc_20_g_component,
            rc_20_h: rc_20_h_component,
            rc_4_3: rc_4_3_component,
            rc_4_4: rc_4_4_component,
            rc_5_4: rc_5_4_component,
            rc_9_9: rc_9_9_component,
            rc_9_9_b: rc_9_9_b_component,
            rc_9_9_c: rc_9_9_c_component,
            rc_9_9_d: rc_9_9_d_component,
            rc_9_9_e: rc_9_9_e_component,
            rc_9_9_f: rc_9_9_f_component,
            rc_9_9_g: rc_9_9_g_component,
            rc_9_9_h: rc_9_9_h_component,
            rc_7_2_5: rc_7_2_5_component,
            rc_3_6_6_3: rc_3_6_6_3_component,
            rc_4_4_4_4: rc_4_4_4_4_component,
            rc_3_3_3_3_3: rc_3_3_3_3_3_component,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        vec![
            &self.rc_6 as &dyn ComponentProver<SimdBackend>,
            &self.rc_8 as &dyn ComponentProver<SimdBackend>,
            &self.rc_11 as &dyn ComponentProver<SimdBackend>,
            &self.rc_12 as &dyn ComponentProver<SimdBackend>,
            &self.rc_18 as &dyn ComponentProver<SimdBackend>,
            &self.rc_18_b as &dyn ComponentProver<SimdBackend>,
            &self.rc_20 as &dyn ComponentProver<SimdBackend>,
            &self.rc_20_b as &dyn ComponentProver<SimdBackend>,
            &self.rc_20_c as &dyn ComponentProver<SimdBackend>,
            &self.rc_20_d as &dyn ComponentProver<SimdBackend>,
            &self.rc_20_e as &dyn ComponentProver<SimdBackend>,
            &self.rc_20_f as &dyn ComponentProver<SimdBackend>,
            &self.rc_20_g as &dyn ComponentProver<SimdBackend>,
            &self.rc_20_h as &dyn ComponentProver<SimdBackend>,
            &self.rc_4_3 as &dyn ComponentProver<SimdBackend>,
            &self.rc_4_4 as &dyn ComponentProver<SimdBackend>,
            &self.rc_5_4 as &dyn ComponentProver<SimdBackend>,
            &self.rc_9_9 as &dyn ComponentProver<SimdBackend>,
            &self.rc_9_9_b as &dyn ComponentProver<SimdBackend>,
            &self.rc_9_9_c as &dyn ComponentProver<SimdBackend>,
            &self.rc_9_9_d as &dyn ComponentProver<SimdBackend>,
            &self.rc_9_9_e as &dyn ComponentProver<SimdBackend>,
            &self.rc_9_9_f as &dyn ComponentProver<SimdBackend>,
            &self.rc_9_9_g as &dyn ComponentProver<SimdBackend>,
            &self.rc_9_9_h as &dyn ComponentProver<SimdBackend>,
            &self.rc_7_2_5 as &dyn ComponentProver<SimdBackend>,
            &self.rc_3_6_6_3 as &dyn ComponentProver<SimdBackend>,
            &self.rc_4_4_4_4 as &dyn ComponentProver<SimdBackend>,
            &self.rc_3_3_3_3_3 as &dyn ComponentProver<SimdBackend>,
        ]
    }
}

impl std::fmt::Display for RangeChecksComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "RangeCheck6: {}", indented_component_display(&self.rc_6))?;
        writeln!(f, "RangeCheck8: {}", indented_component_display(&self.rc_8))?;
        writeln!(
            f,
            "RangeCheck11: {}",
            indented_component_display(&self.rc_11)
        )?;
        writeln!(
            f,
            "RangeCheck12: {}",
            indented_component_display(&self.rc_12)
        )?;
        writeln!(
            f,
            "RangeCheck18: {}",
            indented_component_display(&self.rc_18)
        )?;
        writeln!(
            f,
            "RangeCheck18_B: {}",
            indented_component_display(&self.rc_18_b)
        )?;
        writeln!(
            f,
            "RangeCheck20: {}",
            indented_component_display(&self.rc_20)
        )?;
        writeln!(
            f,
            "RangeCheck20_B: {}",
            indented_component_display(&self.rc_20_b)
        )?;
        writeln!(
            f,
            "RangeCheck20_C: {}",
            indented_component_display(&self.rc_20_c)
        )?;
        writeln!(
            f,
            "RangeCheck20_D: {}",
            indented_component_display(&self.rc_20_d)
        )?;
        writeln!(
            f,
            "RangeCheck20_E: {}",
            indented_component_display(&self.rc_20_e)
        )?;
        writeln!(
            f,
            "RangeCheck20_F: {}",
            indented_component_display(&self.rc_20_f)
        )?;
        writeln!(
            f,
            "RangeCheck20_G: {}",
            indented_component_display(&self.rc_20_g)
        )?;
        writeln!(
            f,
            "RangeCheck20_H: {}",
            indented_component_display(&self.rc_20_h)
        )?;
        writeln!(
            f,
            "RangeCheck4_3: {}",
            indented_component_display(&self.rc_4_3)
        )?;
        writeln!(
            f,
            "RangeCheck4_4: {}",
            indented_component_display(&self.rc_4_4)
        )?;
        writeln!(
            f,
            "RangeCheck5_4: {}",
            indented_component_display(&self.rc_5_4)
        )?;
        writeln!(
            f,
            "RangeCheck9_9: {}",
            indented_component_display(&self.rc_9_9)
        )?;
        writeln!(
            f,
            "RangeCheck9_9_B: {}",
            indented_component_display(&self.rc_9_9_b)
        )?;
        writeln!(
            f,
            "RangeCheck9_9_C: {}",
            indented_component_display(&self.rc_9_9_c)
        )?;
        writeln!(
            f,
            "RangeCheck9_9_D: {}",
            indented_component_display(&self.rc_9_9_d)
        )?;
        writeln!(
            f,
            "RangeCheck9_9_E: {}",
            indented_component_display(&self.rc_9_9_e)
        )?;
        writeln!(
            f,
            "RangeCheck9_9_F: {}",
            indented_component_display(&self.rc_9_9_f)
        )?;
        writeln!(
            f,
            "RangeCheck9_9_G: {}",
            indented_component_display(&self.rc_9_9_g)
        )?;
        writeln!(
            f,
            "RangeCheck9_9_H: {}",
            indented_component_display(&self.rc_9_9_h)
        )?;
        writeln!(
            f,
            "RangeCheck7_2_5: {}",
            indented_component_display(&self.rc_7_2_5)
        )?;
        writeln!(
            f,
            "RangeCheck3_6_6_3: {}",
            indented_component_display(&self.rc_3_6_6_3)
        )?;
        writeln!(
            f,
            "RangeCheck4_4_4_4: {}",
            indented_component_display(&self.rc_4_4_4_4)
        )?;
        writeln!(
            f,
            "RangeCheck3_3_3_3_3: {}",
            indented_component_display(&self.rc_3_3_3_3_3)
        )?;
        Ok(())
    }
}
