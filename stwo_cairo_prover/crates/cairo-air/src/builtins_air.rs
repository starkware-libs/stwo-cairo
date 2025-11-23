use itertools::chain;
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo::core::channel::Channel;
use stwo::core::fields::qm31::{SecureField, QM31};
use stwo::core::pcs::TreeVec;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::ComponentProver;
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};
use stwo_constraint_framework::TraceLocationAllocator;

use super::air::CairoInteractionElements;
use crate::air::{accumulate_relation_uses, RelationUsesDict};
use crate::components::{
    add_mod_builtin, bitwise_builtin, indented_component_display, mul_mod_builtin,
    pedersen_builtin, poseidon_builtin, range_check_builtin_bits_128, range_check_builtin_bits_96,
};

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct BuiltinsClaim {
    pub add_mod_builtin: Option<add_mod_builtin::Claim>,
    pub bitwise_builtin: Option<bitwise_builtin::Claim>,
    pub mul_mod_builtin: Option<mul_mod_builtin::Claim>,
    pub pedersen_builtin: Option<pedersen_builtin::Claim>,
    pub poseidon_builtin: Option<poseidon_builtin::Claim>,
    pub range_check_96_builtin: Option<range_check_builtin_bits_96::Claim>,
    pub range_check_128_builtin: Option<range_check_builtin_bits_128::Claim>,
}
impl BuiltinsClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        if let Some(add_mod_builtin) = &self.add_mod_builtin {
            add_mod_builtin.mix_into(channel);
        }
        if let Some(bitwise_builtin) = &self.bitwise_builtin {
            bitwise_builtin.mix_into(channel);
        }
        if let Some(mul_mod_builtin) = &self.mul_mod_builtin {
            mul_mod_builtin.mix_into(channel);
        }
        if let Some(pedersen_builtin) = &self.pedersen_builtin {
            pedersen_builtin.mix_into(channel);
        }
        if let Some(poseidon_builtin) = &self.poseidon_builtin {
            poseidon_builtin.mix_into(channel);
        }
        if let Some(range_check_96_builtin) = &self.range_check_96_builtin {
            range_check_96_builtin.mix_into(channel);
        }
        if let Some(range_check_128_builtin) = &self.range_check_128_builtin {
            range_check_128_builtin.mix_into(channel);
        }
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        TreeVec::concat_cols(chain!(
            self.add_mod_builtin
                .map(|add_mod_builtin| add_mod_builtin.log_sizes())
                .into_iter(),
            self.bitwise_builtin
                .map(|bitwise_builtin| bitwise_builtin.log_sizes())
                .into_iter(),
            self.mul_mod_builtin
                .map(|mul_mod_builtin| mul_mod_builtin.log_sizes())
                .into_iter(),
            self.pedersen_builtin
                .map(|pedersen_builtin| pedersen_builtin.log_sizes())
                .into_iter(),
            self.poseidon_builtin
                .map(|poseidon_builtin| poseidon_builtin.log_sizes())
                .into_iter(),
            self.range_check_96_builtin
                .map(|range_check_96_builtin| range_check_96_builtin.log_sizes())
                .into_iter(),
            self.range_check_128_builtin
                .map(|range_check_128_builtin| range_check_128_builtin.log_sizes())
                .into_iter(),
        ))
    }
    pub fn accumulate_relation_uses(&self, relation_uses: &mut RelationUsesDict) {
        let Self {
            add_mod_builtin,
            bitwise_builtin,
            mul_mod_builtin,
            pedersen_builtin,
            poseidon_builtin,
            range_check_96_builtin,
            range_check_128_builtin,
        } = self;

        // TODO(alonf): canonicalize the name of field and module.
        macro_rules! relation_uses {
            ($field:ident, $module:ident) => {
                if let Some(field) = $field {
                    accumulate_relation_uses(
                        relation_uses,
                        $module::RELATION_USES_PER_ROW,
                        field.log_size,
                    );
                };
            };
        }
        relation_uses!(add_mod_builtin, add_mod_builtin);
        relation_uses!(bitwise_builtin, bitwise_builtin);
        relation_uses!(mul_mod_builtin, mul_mod_builtin);
        relation_uses!(pedersen_builtin, pedersen_builtin);
        relation_uses!(poseidon_builtin, poseidon_builtin);
        relation_uses!(range_check_96_builtin, range_check_builtin_bits_96);
        relation_uses!(range_check_128_builtin, range_check_builtin_bits_128);
    }
}

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct BuiltinsInteractionClaim {
    pub add_mod_builtin: Option<add_mod_builtin::InteractionClaim>,
    pub bitwise_builtin: Option<bitwise_builtin::InteractionClaim>,
    pub mul_mod_builtin: Option<mul_mod_builtin::InteractionClaim>,
    pub pedersen_builtin: Option<pedersen_builtin::InteractionClaim>,
    pub poseidon_builtin: Option<poseidon_builtin::InteractionClaim>,
    pub range_check_96_builtin: Option<range_check_builtin_bits_96::InteractionClaim>,
    pub range_check_128_builtin: Option<range_check_builtin_bits_128::InteractionClaim>,
}
impl BuiltinsInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        if let Some(add_mod_builtin) = &self.add_mod_builtin {
            add_mod_builtin.mix_into(channel);
        }
        if let Some(bitwise_builtin) = self.bitwise_builtin {
            bitwise_builtin.mix_into(channel)
        }
        if let Some(mul_mod_builtin) = &self.mul_mod_builtin {
            mul_mod_builtin.mix_into(channel);
        }
        if let Some(pedersen_builtin) = &self.pedersen_builtin {
            pedersen_builtin.mix_into(channel);
        }
        if let Some(poseidon_builtin) = self.poseidon_builtin {
            poseidon_builtin.mix_into(channel)
        }
        if let Some(range_check_96_builtin) = &self.range_check_96_builtin {
            range_check_96_builtin.mix_into(channel);
        }
        if let Some(range_check_128_builtin) = self.range_check_128_builtin {
            range_check_128_builtin.mix_into(channel)
        }
    }

    pub fn sum(&self) -> SecureField {
        let mut sum = QM31::zero();
        if let Some(add_mod_builtin) = &self.add_mod_builtin {
            sum += add_mod_builtin.claimed_sum;
        }
        if let Some(bitwise_builtin) = &self.bitwise_builtin {
            sum += bitwise_builtin.claimed_sum;
        }
        if let Some(mul_mod_builtin) = &self.mul_mod_builtin {
            sum += mul_mod_builtin.claimed_sum;
        }
        if let Some(pedersen_builtin) = &self.pedersen_builtin {
            sum += pedersen_builtin.claimed_sum;
        }
        if let Some(poseidon_builtin) = &self.poseidon_builtin {
            sum += poseidon_builtin.claimed_sum;
        }
        if let Some(range_check_96_builtin) = &self.range_check_96_builtin {
            sum += range_check_96_builtin.claimed_sum;
        }
        if let Some(range_check_128_builtin) = &self.range_check_128_builtin {
            sum += range_check_128_builtin.claimed_sum;
        }
        sum
    }
}
pub struct BuiltinComponents {
    pub add_mod_builtin: Option<add_mod_builtin::Component>,
    pub bitwise_builtin: Option<bitwise_builtin::Component>,
    pub mul_mod_builtin: Option<mul_mod_builtin::Component>,
    pub pedersen_builtin: Option<pedersen_builtin::Component>,
    pub poseidon_builtin: Option<poseidon_builtin::Component>,
    pub range_check_96_builtin: Option<range_check_builtin_bits_96::Component>,
    pub range_check_128_builtin: Option<range_check_builtin_bits_128::Component>,
}
impl BuiltinComponents {
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        claim: &BuiltinsClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &BuiltinsInteractionClaim,
    ) -> Self {
        let add_mod_builtin_component = claim.add_mod_builtin.map(|add_mod_builtin| {
            add_mod_builtin::Component::new(
                tree_span_provider,
                add_mod_builtin::Eval {
                    claim: add_mod_builtin,
                    memory_address_to_id_lookup_elements: interaction_elements
                        .memory_address_to_id
                        .clone(),
                    memory_id_to_big_lookup_elements: interaction_elements
                        .memory_id_to_value
                        .clone(),
                },
                interaction_claim.add_mod_builtin.unwrap().claimed_sum,
            )
        });
        let bitwise_builtin_component = claim.bitwise_builtin.map(|bitwise_builtin| {
            bitwise_builtin::Component::new(
                tree_span_provider,
                bitwise_builtin::Eval {
                    claim: bitwise_builtin,
                    memory_address_to_id_lookup_elements: interaction_elements
                        .memory_address_to_id
                        .clone(),
                    memory_id_to_big_lookup_elements: interaction_elements
                        .memory_id_to_value
                        .clone(),
                    verify_bitwise_xor_9_lookup_elements: interaction_elements
                        .verify_bitwise_xor_9
                        .clone(),
                    verify_bitwise_xor_8_lookup_elements: interaction_elements
                        .verify_bitwise_xor_8
                        .clone(),
                },
                interaction_claim.bitwise_builtin.unwrap().claimed_sum,
            )
        });
        let mul_mod_builtin_component = claim.mul_mod_builtin.map(|mul_mod_builtin| {
            mul_mod_builtin::Component::new(
                tree_span_provider,
                mul_mod_builtin::Eval {
                    claim: mul_mod_builtin,
                    memory_address_to_id_lookup_elements: interaction_elements
                        .memory_address_to_id
                        .clone(),
                    memory_id_to_big_lookup_elements: interaction_elements
                        .memory_id_to_value
                        .clone(),
                    range_check_12_lookup_elements: interaction_elements.range_checks.rc_12.clone(),
                    range_check_18_lookup_elements: interaction_elements.range_checks.rc_18.clone(),
                    range_check_3_6_6_3_lookup_elements: interaction_elements
                        .range_checks
                        .rc_3_6_6_3
                        .clone(),
                },
                interaction_claim.mul_mod_builtin.unwrap().claimed_sum,
            )
        });
        let pedersen_builtin_component = claim.pedersen_builtin.map(|pedersen_builtin| {
            pedersen_builtin::Component::new(
                tree_span_provider,
                pedersen_builtin::Eval {
                    claim: pedersen_builtin,
                    memory_address_to_id_lookup_elements: interaction_elements
                        .memory_address_to_id
                        .clone(),
                    pedersen_aggregator_lookup_elements: interaction_elements
                        .pedersen_aggregator
                        .clone(),
                },
                interaction_claim.pedersen_builtin.unwrap().claimed_sum,
            )
        });
        let poseidon_builtin_component = claim.poseidon_builtin.map(|poseidon_builtin| {
            poseidon_builtin::Component::new(
                tree_span_provider,
                poseidon_builtin::Eval {
                    claim: poseidon_builtin,
                    memory_address_to_id_lookup_elements: interaction_elements
                        .memory_address_to_id
                        .clone(),
                    poseidon_aggregator_lookup_elements: interaction_elements
                        .poseidon_aggregator
                        .clone(),
                },
                interaction_claim.poseidon_builtin.unwrap().claimed_sum,
            )
        });
        let range_check_96_builtin_component =
            claim.range_check_96_builtin.map(|range_check_96_builtin| {
                range_check_builtin_bits_96::Component::new(
                    tree_span_provider,
                    range_check_builtin_bits_96::Eval {
                        claim: range_check_96_builtin,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        range_check_6_lookup_elements: interaction_elements
                            .range_checks
                            .rc_6
                            .clone(),
                    },
                    interaction_claim
                        .range_check_96_builtin
                        .unwrap()
                        .claimed_sum,
                )
            });
        let range_check_128_builtin_component =
            claim
                .range_check_128_builtin
                .map(|range_check_128_builtin| {
                    range_check_builtin_bits_128::Component::new(
                        tree_span_provider,
                        range_check_builtin_bits_128::Eval {
                            claim: range_check_128_builtin,
                            memory_address_to_id_lookup_elements: interaction_elements
                                .memory_address_to_id
                                .clone(),
                            memory_id_to_big_lookup_elements: interaction_elements
                                .memory_id_to_value
                                .clone(),
                        },
                        interaction_claim
                            .range_check_128_builtin
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
