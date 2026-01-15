use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::ComponentProver;
use stwo_constraint_framework::TraceLocationAllocator;

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
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        add_mod_builtin_claim: &Option<add_mod_builtin::Claim>,
        bitwise_builtin_claim: &Option<bitwise_builtin::Claim>,
        mul_mod_builtin_claim: &Option<mul_mod_builtin::Claim>,
        pedersen_builtin_claim: &Option<pedersen_builtin::Claim>,
        poseidon_builtin_claim: &Option<poseidon_builtin::Claim>,
        range_check_96_builtin_claim: &Option<range_check96_builtin::Claim>,
        range_check_128_builtin_claim: &Option<range_check_builtin::Claim>,
        common_lookup_elements: &CommonLookupElements,
        add_mod_builtin_interaction_claim: &Option<add_mod_builtin::InteractionClaim>,
        bitwise_builtin_interaction_claim: &Option<bitwise_builtin::InteractionClaim>,
        mul_mod_builtin_interaction_claim: &Option<mul_mod_builtin::InteractionClaim>,
        pedersen_builtin_interaction_claim: &Option<pedersen_builtin::InteractionClaim>,
        poseidon_builtin_interaction_claim: &Option<poseidon_builtin::InteractionClaim>,
        range_check_96_builtin_interaction_claim: &Option<range_check96_builtin::InteractionClaim>,
        range_check_128_builtin_interaction_claim: &Option<range_check_builtin::InteractionClaim>,
    ) -> Self {
        let add_mod_builtin_component = add_mod_builtin_claim.map(|add_mod_builtin| {
            add_mod_builtin::Component::new(
                tree_span_provider,
                add_mod_builtin::Eval {
                    claim: add_mod_builtin,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                add_mod_builtin_interaction_claim.unwrap().claimed_sum,
            )
        });
        let bitwise_builtin_component = bitwise_builtin_claim.map(|bitwise_builtin| {
            bitwise_builtin::Component::new(
                tree_span_provider,
                bitwise_builtin::Eval {
                    claim: bitwise_builtin,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                bitwise_builtin_interaction_claim.unwrap().claimed_sum,
            )
        });
        let mul_mod_builtin_component = mul_mod_builtin_claim.map(|mul_mod_builtin| {
            mul_mod_builtin::Component::new(
                tree_span_provider,
                mul_mod_builtin::Eval {
                    claim: mul_mod_builtin,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                mul_mod_builtin_interaction_claim.unwrap().claimed_sum,
            )
        });
        let pedersen_builtin_component = pedersen_builtin_claim.map(|pedersen_builtin| {
            pedersen_builtin::Component::new(
                tree_span_provider,
                pedersen_builtin::Eval {
                    claim: pedersen_builtin,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                pedersen_builtin_interaction_claim.unwrap().claimed_sum,
            )
        });
        let poseidon_builtin_component = poseidon_builtin_claim.map(|poseidon_builtin| {
            poseidon_builtin::Component::new(
                tree_span_provider,
                poseidon_builtin::Eval {
                    claim: poseidon_builtin,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                poseidon_builtin_interaction_claim.unwrap().claimed_sum,
            )
        });
        let range_check_96_builtin_component =
            range_check_96_builtin_claim.map(|range_check_96_builtin| {
                range_check96_builtin::Component::new(
                    tree_span_provider,
                    range_check96_builtin::Eval {
                        claim: range_check_96_builtin,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    range_check_96_builtin_interaction_claim
                        .unwrap()
                        .claimed_sum,
                )
            });
        let range_check_128_builtin_component =
            range_check_128_builtin_claim.map(|range_check_128_builtin| {
                range_check_builtin::Component::new(
                    tree_span_provider,
                    range_check_builtin::Eval {
                        claim: range_check_128_builtin,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    range_check_128_builtin_interaction_claim
                        .unwrap()
                        .claimed_sum,
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

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        let mut vec: Vec<&dyn ComponentProver<SimdBackend>> = vec![];
        if let Some(add_mod_builtin) = &self.add_mod_builtin {
            vec.push(add_mod_builtin as &dyn ComponentProver<SimdBackend>);
        }
        if let Some(bitwise_builtin) = &self.bitwise_builtin {
            vec.push(bitwise_builtin as &dyn ComponentProver<SimdBackend>);
        }
        if let Some(mul_mod_builtin) = &self.mul_mod_builtin {
            vec.push(mul_mod_builtin as &dyn ComponentProver<SimdBackend>);
        }
        if let Some(pedersen_builtin) = &self.pedersen_builtin {
            vec.push(pedersen_builtin as &dyn ComponentProver<SimdBackend>);
        }
        if let Some(poseidon_builtin) = &self.poseidon_builtin {
            vec.push(poseidon_builtin as &dyn ComponentProver<SimdBackend>);
        }
        if let Some(range_check_96_builtin) = &self.range_check_96_builtin {
            vec.push(range_check_96_builtin as &dyn ComponentProver<SimdBackend>);
        }
        if let Some(range_check_128_builtin) = &self.range_check_128_builtin {
            vec.push(range_check_128_builtin as &dyn ComponentProver<SimdBackend>);
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
