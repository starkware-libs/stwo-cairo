use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo::core::air::Component;
use stwo::core::channel::Channel;
use stwo::core::fields::qm31::{SecureField, QM31};
use stwo::core::pcs::TreeVec;
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};
use stwo_constraint_framework::TraceLocationAllocator;

use crate::components::{
    indented_component_display, range_check_11, range_check_12, range_check_18, range_check_20,
    range_check_3_3_3_3_3, range_check_3_6_6_3, range_check_4_3, range_check_4_4,
    range_check_4_4_4_4, range_check_6, range_check_7_2_5, range_check_8, range_check_9_9,
};
use crate::relations;

#[derive(Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct RangeChecksClaim {
    pub rc_6: range_check_6::Claim,
    pub rc_8: range_check_8::Claim,
    pub rc_11: range_check_11::Claim,
    pub rc_12: range_check_12::Claim,
    pub rc_18: range_check_18::Claim,
    pub rc_20: range_check_20::Claim,
    pub rc_4_3: range_check_4_3::Claim,
    pub rc_4_4: range_check_4_4::Claim,
    pub rc_9_9: range_check_9_9::Claim,
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
        self.rc_20.mix_into(channel);
        self.rc_4_3.mix_into(channel);
        self.rc_4_4.mix_into(channel);
        self.rc_9_9.mix_into(channel);
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
                self.rc_20.log_sizes(),
                self.rc_4_3.log_sizes(),
                self.rc_4_4.log_sizes(),
                self.rc_9_9.log_sizes(),
                self.rc_7_2_5.log_sizes(),
                self.rc_3_6_6_3.log_sizes(),
                self.rc_4_4_4_4.log_sizes(),
                self.rc_3_3_3_3_3.log_sizes(),
            ]
            .into_iter(),
        )
    }
}

#[derive(Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct RangeChecksInteractionClaim {
    pub rc_6: range_check_6::InteractionClaim,
    pub rc_8: range_check_8::InteractionClaim,
    pub rc_11: range_check_11::InteractionClaim,
    pub rc_12: range_check_12::InteractionClaim,
    pub rc_18: range_check_18::InteractionClaim,
    pub rc_20: range_check_20::InteractionClaim,
    pub rc_4_3: range_check_4_3::InteractionClaim,
    pub rc_4_4: range_check_4_4::InteractionClaim,
    pub rc_9_9: range_check_9_9::InteractionClaim,
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
        self.rc_20.mix_into(channel);
        self.rc_4_3.mix_into(channel);
        self.rc_4_4.mix_into(channel);
        self.rc_9_9.mix_into(channel);
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
        sum += self.rc_20.claimed_sum;
        sum += self.rc_4_3.claimed_sum;
        sum += self.rc_4_4.claimed_sum;
        sum += self.rc_9_9.claimed_sum;
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
    pub rc_20: range_check_20::Component,
    pub rc_4_3: range_check_4_3::Component,
    pub rc_4_4: range_check_4_4::Component,
    pub rc_9_9: range_check_9_9::Component,
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
        let rc_20_component = range_check_20::Component::new(
            tree_span_provider,
            range_check_20::Eval {
                claim: range_check_20::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_20.claimed_sum,
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
        let rc_9_9_component = range_check_9_9::Component::new(
            tree_span_provider,
            range_check_9_9::Eval {
                claim: range_check_9_9::Claim {},
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.rc_9_9.claimed_sum,
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
            rc_20: rc_20_component,
            rc_4_3: rc_4_3_component,
            rc_4_4: rc_4_4_component,
            rc_9_9: rc_9_9_component,
            rc_7_2_5: rc_7_2_5_component,
            rc_3_6_6_3: rc_3_6_6_3_component,
            rc_4_4_4_4: rc_4_4_4_4_component,
            rc_3_3_3_3_3: rc_3_3_3_3_3_component,
        }
    }

    pub fn components(&self) -> Vec<&dyn Component> {
        vec![
            &self.rc_6 as &dyn Component,
            &self.rc_8 as &dyn Component,
            &self.rc_11 as &dyn Component,
            &self.rc_12 as &dyn Component,
            &self.rc_18 as &dyn Component,
            &self.rc_20 as &dyn Component,
            &self.rc_4_3 as &dyn Component,
            &self.rc_4_4 as &dyn Component,
            &self.rc_9_9 as &dyn Component,
            &self.rc_7_2_5 as &dyn Component,
            &self.rc_3_6_6_3 as &dyn Component,
            &self.rc_4_4_4_4 as &dyn Component,
            &self.rc_3_3_3_3_3 as &dyn Component,
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
            "RangeCheck20: {}",
            indented_component_display(&self.rc_20)
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
            "RangeCheck9_9: {}",
            indented_component_display(&self.rc_9_9)
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
