use stwo::core::air::Component;
use stwo_constraint_framework::TraceLocationAllocator;

use crate::claims::{CairoClaim, CairoInteractionClaim};
use crate::components::{
    add_mod_builtin, bitwise_builtin, indented_component_display, mul_mod_builtin,
    pedersen_builtin, poseidon_builtin, range_check96_builtin, range_check_builtin,
};
use crate::relations::CommonLookupElements;

pub struct BuiltinComponents {
    pub add_mod_builtin: Option<add_mod_builtin::Component>,
    pub bitwise_builtin: Option<bitwise_builtin::Component>,
    pub mul_mod_builtin: Option<mul_mod_builtin::Component>,
    pub pedersen_builtin: Option<pedersen_builtin::Component>,
    pub poseidon_builtin: Option<poseidon_builtin::Component>,
    pub range_check_96_builtin: Option<range_check96_builtin::Component>,
    pub range_check_128_builtin: Option<range_check_builtin::Component>,
}
impl BuiltinComponents {
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        cairo_claim: &CairoClaim,
        common_lookup_elements: &CommonLookupElements,
        interaction_claim: &CairoInteractionClaim,
    ) -> Self {
        let add_mod_builtin_component = cairo_claim.add_mod_builtin.map(|add_mod_builtin| {
            add_mod_builtin::Component::new(
                tree_span_provider,
                add_mod_builtin::Eval {
                    claim: add_mod_builtin,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.add_mod_builtin.unwrap().claimed_sum,
            )
        });
        let bitwise_builtin_component = cairo_claim.bitwise_builtin.map(|bitwise_builtin| {
            bitwise_builtin::Component::new(
                tree_span_provider,
                bitwise_builtin::Eval {
                    claim: bitwise_builtin,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.bitwise_builtin.unwrap().claimed_sum,
            )
        });
        let mul_mod_builtin_component = cairo_claim.mul_mod_builtin.map(|mul_mod_builtin| {
            mul_mod_builtin::Component::new(
                tree_span_provider,
                mul_mod_builtin::Eval {
                    claim: mul_mod_builtin,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.mul_mod_builtin.unwrap().claimed_sum,
            )
        });
        let pedersen_builtin_component = cairo_claim.pedersen_builtin.map(|pedersen_builtin| {
            pedersen_builtin::Component::new(
                tree_span_provider,
                pedersen_builtin::Eval {
                    claim: pedersen_builtin,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.pedersen_builtin.unwrap().claimed_sum,
            )
        });
        let poseidon_builtin_component = cairo_claim.poseidon_builtin.map(|poseidon_builtin| {
            poseidon_builtin::Component::new(
                tree_span_provider,
                poseidon_builtin::Eval {
                    claim: poseidon_builtin,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.poseidon_builtin.unwrap().claimed_sum,
            )
        });
        let range_check_96_builtin_component =
            cairo_claim
                .range_check96_builtin
                .map(|range_check_96_builtin| {
                    range_check96_builtin::Component::new(
                        tree_span_provider,
                        range_check96_builtin::Eval {
                            claim: range_check_96_builtin,
                            common_lookup_elements: common_lookup_elements.clone(),
                        },
                        interaction_claim.range_check96_builtin.unwrap().claimed_sum,
                    )
                });
        let range_check_128_builtin_component =
            cairo_claim
                .range_check_builtin
                .map(|range_check_128_builtin| {
                    range_check_builtin::Component::new(
                        tree_span_provider,
                        range_check_builtin::Eval {
                            claim: range_check_128_builtin,
                            common_lookup_elements: common_lookup_elements.clone(),
                        },
                        interaction_claim.range_check_builtin.unwrap().claimed_sum,
                    )
                });
        Self {
            add_mod_builtin: add_mod_builtin_component,
            bitwise_builtin: bitwise_builtin_component,
            mul_mod_builtin: mul_mod_builtin_component,
            pedersen_builtin: pedersen_builtin_component,
            poseidon_builtin: poseidon_builtin_component,
            range_check_96_builtin: range_check_96_builtin_component,
            range_check_128_builtin: range_check_128_builtin_component,
        }
    }

    pub fn components(&self) -> Vec<&dyn Component> {
        let mut vec: Vec<&dyn Component> = vec![];
        if let Some(add_mod_builtin) = &self.add_mod_builtin {
            vec.push(add_mod_builtin as &dyn Component);
        }
        if let Some(bitwise_builtin) = &self.bitwise_builtin {
            vec.push(bitwise_builtin as &dyn Component);
        }
        if let Some(mul_mod_builtin) = &self.mul_mod_builtin {
            vec.push(mul_mod_builtin as &dyn Component);
        }
        if let Some(pedersen_builtin) = &self.pedersen_builtin {
            vec.push(pedersen_builtin as &dyn Component);
        }
        if let Some(poseidon_builtin) = &self.poseidon_builtin {
            vec.push(poseidon_builtin as &dyn Component);
        }
        if let Some(range_check_96_builtin) = &self.range_check_96_builtin {
            vec.push(range_check_96_builtin as &dyn Component);
        }
        if let Some(range_check_128_builtin) = &self.range_check_128_builtin {
            vec.push(range_check_128_builtin as &dyn Component);
        }
        vec
    }
}

impl std::fmt::Display for BuiltinComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(add_mod_builtin) = &self.add_mod_builtin {
            writeln!(
                f,
                "AddModBuiltin: {}",
                indented_component_display(add_mod_builtin)
            )?;
        }
        if let Some(bitwise_builtin) = &self.bitwise_builtin {
            writeln!(
                f,
                "BitwiseBuiltin: {}",
                indented_component_display(bitwise_builtin)
            )?;
        }
        if let Some(mul_mod_builtin) = &self.mul_mod_builtin {
            writeln!(
                f,
                "MulModBuiltin: {}",
                indented_component_display(mul_mod_builtin)
            )?;
        }
        if let Some(pedersen_builtin) = &self.pedersen_builtin {
            writeln!(
                f,
                "PedersenBuiltin: {}",
                indented_component_display(pedersen_builtin)
            )?;
        }
        if let Some(poseidon_builtin) = &self.poseidon_builtin {
            writeln!(
                f,
                "PoseidonBuiltin: {}",
                indented_component_display(poseidon_builtin)
            )?;
        }
        if let Some(range_check_96_builtin) = &self.range_check_96_builtin {
            writeln!(
                f,
                "RangeCheck96Builtin: {}",
                indented_component_display(range_check_96_builtin)
            )?;
        }
        if let Some(range_check_128_builtin) = &self.range_check_128_builtin {
            writeln!(
                f,
                "RangeCheck128Builtin: {}",
                indented_component_display(range_check_128_builtin)
            )?;
        }
        Ok(())
    }
}
