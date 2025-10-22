use itertools::{chain, Itertools};
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
use super::components::display_components;
use crate::air::{accumulate_relation_uses, RelationUsesDict};
use crate::components::{
    add_ap_opcode, add_opcode, add_opcode_small, assert_eq_opcode, assert_eq_opcode_double_deref,
    assert_eq_opcode_imm, blake_compress_opcode, call_opcode_abs, call_opcode_rel_imm,
    generic_opcode, jnz_opcode_non_taken, jnz_opcode_taken, jump_opcode_abs,
    jump_opcode_double_deref, jump_opcode_rel, jump_opcode_rel_imm, mul_opcode, mul_opcode_small,
    qm_31_add_mul_opcode, ret_opcode,
};

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct OpcodeClaim {
    pub add: Vec<add_opcode::Claim>,
    pub add_small: Vec<add_opcode_small::Claim>,
    pub add_ap: Vec<add_ap_opcode::Claim>,
    pub assert_eq: Vec<assert_eq_opcode::Claim>,
    pub assert_eq_imm: Vec<assert_eq_opcode_imm::Claim>,
    pub assert_eq_double_deref: Vec<assert_eq_opcode_double_deref::Claim>,
    pub blake: Vec<blake_compress_opcode::Claim>,
    pub call: Vec<call_opcode_abs::Claim>,
    pub call_rel_imm: Vec<call_opcode_rel_imm::Claim>,
    pub generic: Vec<generic_opcode::Claim>,
    pub jnz: Vec<jnz_opcode_non_taken::Claim>,
    pub jnz_taken: Vec<jnz_opcode_taken::Claim>,
    pub jump: Vec<jump_opcode_abs::Claim>,
    pub jump_double_deref: Vec<jump_opcode_double_deref::Claim>,
    pub jump_rel: Vec<jump_opcode_rel::Claim>,
    pub jump_rel_imm: Vec<jump_opcode_rel_imm::Claim>,
    pub mul: Vec<mul_opcode::Claim>,
    pub mul_small: Vec<mul_opcode_small::Claim>,
    pub qm31: Vec<qm_31_add_mul_opcode::Claim>,
    pub ret: Vec<ret_opcode::Claim>,
}
impl OpcodeClaim {
    /// For each opcode component vector, mixes the length and then the claims.
    pub fn mix_into(&self, channel: &mut impl Channel) {
        macro_rules! mix_component_vector {
            ($field:ident) => {
                channel.mix_u64(self.$field.len() as u64);
                self.$field.iter().for_each(|c| c.mix_into(channel));
            };
        }

        mix_component_vector!(add);
        mix_component_vector!(add_small);
        mix_component_vector!(add_ap);
        mix_component_vector!(assert_eq);
        mix_component_vector!(assert_eq_imm);
        mix_component_vector!(assert_eq_double_deref);
        mix_component_vector!(blake);
        mix_component_vector!(call);
        mix_component_vector!(call_rel_imm);
        mix_component_vector!(generic);
        mix_component_vector!(jnz);
        mix_component_vector!(jnz_taken);
        mix_component_vector!(jump);
        mix_component_vector!(jump_double_deref);
        mix_component_vector!(jump_rel);
        mix_component_vector!(jump_rel_imm);
        mix_component_vector!(mul);
        mix_component_vector!(mul_small);
        mix_component_vector!(qm31);
        mix_component_vector!(ret);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        TreeVec::concat_cols(chain!(
            self.add.iter().map(|c| c.log_sizes()),
            self.add_small.iter().map(|c| c.log_sizes()),
            self.add_ap.iter().map(|c| c.log_sizes()),
            self.assert_eq.iter().map(|c| c.log_sizes()),
            self.assert_eq_imm.iter().map(|c| c.log_sizes()),
            self.assert_eq_double_deref.iter().map(|c| c.log_sizes()),
            self.blake.iter().map(|c| c.log_sizes()),
            self.call.iter().map(|c| c.log_sizes()),
            self.call_rel_imm.iter().map(|c| c.log_sizes()),
            self.generic.iter().map(|c| c.log_sizes()),
            self.jnz.iter().map(|c| c.log_sizes()),
            self.jnz_taken.iter().map(|c| c.log_sizes()),
            self.jump.iter().map(|c| c.log_sizes()),
            self.jump_double_deref.iter().map(|c| c.log_sizes()),
            self.jump_rel.iter().map(|c| c.log_sizes()),
            self.jump_rel_imm.iter().map(|c| c.log_sizes()),
            self.mul.iter().map(|c| c.log_sizes()),
            self.mul_small.iter().map(|c| c.log_sizes()),
            self.qm31.iter().map(|c| c.log_sizes()),
            self.ret.iter().map(|c| c.log_sizes()),
        ))
    }

    pub fn accumulate_relation_uses(&self, relation_uses: &mut RelationUsesDict) {
        let Self {
            add,
            add_small,
            add_ap,
            assert_eq,
            assert_eq_imm,
            assert_eq_double_deref,
            blake,
            call,
            call_rel_imm,
            generic,
            jnz,
            jnz_taken,
            jump,
            jump_double_deref,
            jump_rel,
            jump_rel_imm,
            mul,
            mul_small,
            qm31,
            ret,
        } = self;

        // TODO(alonf): canonicalize the name of field and module.
        macro_rules! relation_uses {
            ($field:ident, $module:ident) => {
                $field.iter().for_each(|c| {
                    accumulate_relation_uses(
                        relation_uses,
                        $module::RELATION_USES_PER_ROW,
                        c.log_size,
                    )
                });
            };
        }
        relation_uses!(add, add_opcode);
        relation_uses!(add_small, add_opcode_small);
        relation_uses!(add_ap, add_ap_opcode);
        relation_uses!(assert_eq, assert_eq_opcode);
        relation_uses!(assert_eq_imm, assert_eq_opcode_imm);
        relation_uses!(assert_eq_double_deref, assert_eq_opcode_double_deref);
        relation_uses!(blake, blake_compress_opcode);
        relation_uses!(call, call_opcode_abs);
        relation_uses!(call_rel_imm, call_opcode_rel_imm);
        relation_uses!(generic, generic_opcode);
        relation_uses!(jnz, jnz_opcode_non_taken);
        relation_uses!(jnz_taken, jnz_opcode_taken);
        relation_uses!(jump, jump_opcode_abs);
        relation_uses!(jump_double_deref, jump_opcode_double_deref);
        relation_uses!(jump_rel, jump_opcode_rel);
        relation_uses!(jump_rel_imm, jump_opcode_rel_imm);
        relation_uses!(mul, mul_opcode);
        relation_uses!(mul_small, mul_opcode_small);
        relation_uses!(qm31, qm_31_add_mul_opcode);
        relation_uses!(ret, ret_opcode);
    }
}

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct OpcodeInteractionClaim {
    pub add: Vec<add_opcode::InteractionClaim>,
    pub add_small: Vec<add_opcode_small::InteractionClaim>,
    pub add_ap: Vec<add_ap_opcode::InteractionClaim>,
    pub assert_eq: Vec<assert_eq_opcode::InteractionClaim>,
    pub assert_eq_imm: Vec<assert_eq_opcode_imm::InteractionClaim>,
    pub assert_eq_double_deref: Vec<assert_eq_opcode_double_deref::InteractionClaim>,
    pub blake: Vec<blake_compress_opcode::InteractionClaim>,
    pub call: Vec<call_opcode_abs::InteractionClaim>,
    pub call_rel_imm: Vec<call_opcode_rel_imm::InteractionClaim>,
    pub generic: Vec<generic_opcode::InteractionClaim>,
    pub jnz: Vec<jnz_opcode_non_taken::InteractionClaim>,
    pub jnz_taken: Vec<jnz_opcode_taken::InteractionClaim>,
    pub jump: Vec<jump_opcode_abs::InteractionClaim>,
    pub jump_double_deref: Vec<jump_opcode_double_deref::InteractionClaim>,
    pub jump_rel: Vec<jump_opcode_rel::InteractionClaim>,
    pub jump_rel_imm: Vec<jump_opcode_rel_imm::InteractionClaim>,
    pub mul: Vec<mul_opcode::InteractionClaim>,
    pub mul_small: Vec<mul_opcode_small::InteractionClaim>,
    pub qm31: Vec<qm_31_add_mul_opcode::InteractionClaim>,
    pub ret: Vec<ret_opcode::InteractionClaim>,
}
impl OpcodeInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.add.iter().for_each(|c| c.mix_into(channel));
        self.add_small.iter().for_each(|c| c.mix_into(channel));
        self.add_ap.iter().for_each(|c| c.mix_into(channel));
        self.assert_eq.iter().for_each(|c| c.mix_into(channel));
        self.assert_eq_imm.iter().for_each(|c| c.mix_into(channel));
        self.assert_eq_double_deref
            .iter()
            .for_each(|c| c.mix_into(channel));
        self.blake.iter().for_each(|c| c.mix_into(channel));
        self.call.iter().for_each(|c| c.mix_into(channel));
        self.call_rel_imm.iter().for_each(|c| c.mix_into(channel));
        self.generic.iter().for_each(|c| c.mix_into(channel));
        self.jnz.iter().for_each(|c| c.mix_into(channel));
        self.jnz_taken.iter().for_each(|c| c.mix_into(channel));
        self.jump.iter().for_each(|c| c.mix_into(channel));
        self.jump_double_deref
            .iter()
            .for_each(|c| c.mix_into(channel));
        self.jump_rel.iter().for_each(|c| c.mix_into(channel));
        self.jump_rel_imm.iter().for_each(|c| c.mix_into(channel));
        self.mul.iter().for_each(|c| c.mix_into(channel));
        self.mul_small.iter().for_each(|c| c.mix_into(channel));
        self.qm31.iter().for_each(|c| c.mix_into(channel));
        self.ret.iter().for_each(|c| c.mix_into(channel));
    }

    pub fn sum(&self) -> SecureField {
        let mut sum = QM31::zero();
        for interaction_claim in &self.add {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.add_small {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.add_ap {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.assert_eq {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.assert_eq_imm {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.assert_eq_double_deref {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.blake {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.call {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.call_rel_imm {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.generic {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.jnz {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.jnz_taken {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.jump {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.jump_double_deref {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.jump_rel {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.jump_rel_imm {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.mul {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.mul_small {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.qm31 {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.ret {
            sum += interaction_claim.claimed_sum;
        }
        sum
    }
}

pub struct OpcodeComponents {
    pub add: Vec<add_opcode::Component>,
    pub add_small: Vec<add_opcode_small::Component>,
    pub add_ap: Vec<add_ap_opcode::Component>,
    pub assert_eq: Vec<assert_eq_opcode::Component>,
    pub assert_eq_imm: Vec<assert_eq_opcode_imm::Component>,
    pub assert_eq_double_deref: Vec<assert_eq_opcode_double_deref::Component>,
    pub blake: Vec<blake_compress_opcode::Component>,
    pub call: Vec<call_opcode_abs::Component>,
    pub call_rel_imm: Vec<call_opcode_rel_imm::Component>,
    pub generic: Vec<generic_opcode::Component>,
    pub jnz: Vec<jnz_opcode_non_taken::Component>,
    pub jnz_taken: Vec<jnz_opcode_taken::Component>,
    pub jump: Vec<jump_opcode_abs::Component>,
    pub jump_double_deref: Vec<jump_opcode_double_deref::Component>,
    pub jump_rel: Vec<jump_opcode_rel::Component>,
    pub jump_rel_imm: Vec<jump_opcode_rel_imm::Component>,
    pub mul: Vec<mul_opcode::Component>,
    pub mul_small: Vec<mul_opcode_small::Component>,
    pub qm31: Vec<qm_31_add_mul_opcode::Component>,
    pub ret: Vec<ret_opcode::Component>,
}
impl OpcodeComponents {
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        claim: &OpcodeClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &OpcodeInteractionClaim,
    ) -> Self {
        let add_components = claim
            .add
            .iter()
            .zip(interaction_claim.add.iter())
            .map(|(&claim, &interaction_claim)| {
                add_opcode::Component::new(
                    tree_span_provider,
                    add_opcode::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let add_small_components = claim
            .add_small
            .iter()
            .zip(interaction_claim.add_small.iter())
            .map(|(&claim, &interaction_claim)| {
                add_opcode_small::Component::new(
                    tree_span_provider,
                    add_opcode_small::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let add_ap_components = claim
            .add_ap
            .iter()
            .zip(interaction_claim.add_ap.iter())
            .map(|(&claim, &interaction_claim)| {
                add_ap_opcode::Component::new(
                    tree_span_provider,
                    add_ap_opcode::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        range_check_18_lookup_elements: interaction_elements
                            .range_checks
                            .rc_18
                            .clone(),
                        range_check_11_lookup_elements: interaction_elements
                            .range_checks
                            .rc_11
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let assert_eq_components = claim
            .assert_eq
            .iter()
            .zip(interaction_claim.assert_eq.iter())
            .map(|(&claim, &interaction_claim)| {
                assert_eq_opcode::Component::new(
                    tree_span_provider,
                    assert_eq_opcode::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let assert_eq_imm_components = claim
            .assert_eq_imm
            .iter()
            .zip(interaction_claim.assert_eq_imm.iter())
            .map(|(&claim, &interaction_claim)| {
                assert_eq_opcode_imm::Component::new(
                    tree_span_provider,
                    assert_eq_opcode_imm::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let assert_eq_double_deref_components = claim
            .assert_eq_double_deref
            .iter()
            .zip(interaction_claim.assert_eq_double_deref.iter())
            .map(|(&claim, &interaction_claim)| {
                assert_eq_opcode_double_deref::Component::new(
                    tree_span_provider,
                    assert_eq_opcode_double_deref::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let blake_components = claim
            .blake
            .iter()
            .zip(interaction_claim.blake.iter())
            .map(|(&claim, &interaction_claim)| {
                blake_compress_opcode::Component::new(
                    tree_span_provider,
                    blake_compress_opcode::Eval {
                        claim,
                        blake_round_lookup_elements: interaction_elements.blake_round.clone(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        range_check_7_2_5_lookup_elements: interaction_elements
                            .range_checks
                            .rc_7_2_5
                            .clone(),
                        triple_xor_32_lookup_elements: interaction_elements.triple_xor_32.clone(),
                        verify_bitwise_xor_8_lookup_elements: interaction_elements
                            .verify_bitwise_xor_8
                            .clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let call_components = claim
            .call
            .iter()
            .zip(interaction_claim.call.iter())
            .map(|(&claim, &interaction_claim)| {
                call_opcode_abs::Component::new(
                    tree_span_provider,
                    call_opcode_abs::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let call_rel_imm_components = claim
            .call_rel_imm
            .iter()
            .zip(interaction_claim.call_rel_imm.iter())
            .map(|(&claim, &interaction_claim)| {
                call_opcode_rel_imm::Component::new(
                    tree_span_provider,
                    call_opcode_rel_imm::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let generic_components = claim
            .generic
            .iter()
            .zip(interaction_claim.generic.iter())
            .map(|(&claim, &interaction_claim)| {
                generic_opcode::Component::new(
                    tree_span_provider,
                    generic_opcode::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        range_check_20_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20
                            .clone(),
                        range_check_20_b_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_b
                            .clone(),
                        range_check_20_c_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_c
                            .clone(),
                        range_check_20_d_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_d
                            .clone(),
                        range_check_20_e_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_e
                            .clone(),
                        range_check_20_f_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_f
                            .clone(),
                        range_check_20_g_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_g
                            .clone(),
                        range_check_20_h_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_h
                            .clone(),
                        range_check_9_9_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9
                            .clone(),
                        range_check_9_9_b_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_b
                            .clone(),
                        range_check_9_9_c_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_c
                            .clone(),
                        range_check_9_9_d_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_d
                            .clone(),
                        range_check_9_9_e_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_e
                            .clone(),
                        range_check_9_9_f_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_f
                            .clone(),
                        range_check_9_9_g_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_g
                            .clone(),
                        range_check_9_9_h_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_h
                            .clone(),
                        range_check_18_lookup_elements: interaction_elements
                            .range_checks
                            .rc_18
                            .clone(),
                        range_check_11_lookup_elements: interaction_elements
                            .range_checks
                            .rc_11
                            .clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let jnz_components = claim
            .jnz
            .iter()
            .zip(interaction_claim.jnz.iter())
            .map(|(&claim, &interaction_claim)| {
                jnz_opcode_non_taken::Component::new(
                    tree_span_provider,
                    jnz_opcode_non_taken::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let jnz_taken_components = claim
            .jnz_taken
            .iter()
            .zip(interaction_claim.jnz_taken.iter())
            .map(|(&claim, &interaction_claim)| {
                jnz_opcode_taken::Component::new(
                    tree_span_provider,
                    jnz_opcode_taken::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let jump_components = claim
            .jump
            .iter()
            .zip(interaction_claim.jump.iter())
            .map(|(&claim, &interaction_claim)| {
                jump_opcode_abs::Component::new(
                    tree_span_provider,
                    jump_opcode_abs::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let jump_double_deref_components = claim
            .jump_double_deref
            .iter()
            .zip(interaction_claim.jump_double_deref.iter())
            .map(|(&claim, &interaction_claim)| {
                jump_opcode_double_deref::Component::new(
                    tree_span_provider,
                    jump_opcode_double_deref::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let jump_rel_components = claim
            .jump_rel
            .iter()
            .zip(interaction_claim.jump_rel.iter())
            .map(|(&claim, &interaction_claim)| {
                jump_opcode_rel::Component::new(
                    tree_span_provider,
                    jump_opcode_rel::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let jump_rel_imm_components = claim
            .jump_rel_imm
            .iter()
            .zip(interaction_claim.jump_rel_imm.iter())
            .map(|(&claim, &interaction_claim)| {
                jump_opcode_rel_imm::Component::new(
                    tree_span_provider,
                    jump_opcode_rel_imm::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let mul_components = claim
            .mul
            .iter()
            .zip(interaction_claim.mul.iter())
            .map(|(&claim, &interaction_claim)| {
                mul_opcode::Component::new(
                    tree_span_provider,
                    mul_opcode::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        range_check_20_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20
                            .clone(),
                        range_check_20_b_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_b
                            .clone(),
                        range_check_20_c_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_c
                            .clone(),
                        range_check_20_d_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_d
                            .clone(),
                        range_check_20_e_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_e
                            .clone(),
                        range_check_20_f_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_f
                            .clone(),
                        range_check_20_g_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_g
                            .clone(),
                        range_check_20_h_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_h
                            .clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let mul_small_components = claim
            .mul_small
            .iter()
            .zip(interaction_claim.mul_small.iter())
            .map(|(&claim, &interaction_claim)| {
                mul_opcode_small::Component::new(
                    tree_span_provider,
                    mul_opcode_small::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        range_check_11_lookup_elements: interaction_elements
                            .range_checks
                            .rc_11
                            .clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let qm31_components = claim
            .qm31
            .iter()
            .zip(interaction_claim.qm31.iter())
            .map(|(&claim, &interaction_claim)| {
                qm_31_add_mul_opcode::Component::new(
                    tree_span_provider,
                    qm_31_add_mul_opcode::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        range_check_4_4_4_4_lookup_elements: interaction_elements
                            .range_checks
                            .rc_4_4_4_4
                            .clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let ret_components = claim
            .ret
            .iter()
            .zip(interaction_claim.ret.iter())
            .map(|(&claim, &interaction_claim)| {
                ret_opcode::Component::new(
                    tree_span_provider,
                    ret_opcode::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        Self {
            add: add_components,
            add_small: add_small_components,
            add_ap: add_ap_components,
            assert_eq: assert_eq_components,
            assert_eq_imm: assert_eq_imm_components,
            assert_eq_double_deref: assert_eq_double_deref_components,
            blake: blake_components,
            call: call_components,
            call_rel_imm: call_rel_imm_components,
            generic: generic_components,
            jnz: jnz_components,
            jnz_taken: jnz_taken_components,
            jump: jump_components,
            jump_double_deref: jump_double_deref_components,
            jump_rel: jump_rel_components,
            jump_rel_imm: jump_rel_imm_components,
            mul: mul_components,
            mul_small: mul_small_components,
            qm31: qm31_components,
            ret: ret_components,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        let mut vec: Vec<&dyn ComponentProver<SimdBackend>> = vec![];
        vec.extend(
            self.add
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.add_small
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.add_ap
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.assert_eq
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.assert_eq_imm
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.assert_eq_double_deref
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.blake
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.call
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.call_rel_imm
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.generic
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.jnz
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.jnz_taken
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.jump
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.jump_double_deref
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.jump_rel
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.jump_rel_imm
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.mul
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.mul_small
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.qm31
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.ret
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec
    }
}

impl std::fmt::Display for OpcodeComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "add:")?;
        writeln!(f, "{}", display_components(&self.add))?;
        writeln!(f, "add_small:")?;
        writeln!(f, "{}", display_components(&self.add_small))?;
        writeln!(f, "add_ap:")?;
        writeln!(f, "{}", display_components(&self.add_ap))?;
        writeln!(f, "assert_eq:")?;
        writeln!(f, "{}", display_components(&self.assert_eq))?;
        writeln!(f, "assert_eq_imm:")?;
        writeln!(f, "{}", display_components(&self.assert_eq_imm))?;
        writeln!(f, "assert_eq_double_deref:")?;
        writeln!(f, "{}", display_components(&self.assert_eq_double_deref))?;
        writeln!(f, "blake:")?;
        writeln!(f, "{}", display_components(&self.blake))?;
        writeln!(f, "call:")?;
        writeln!(f, "{}", display_components(&self.call))?;
        writeln!(f, "call_rel_imm:")?;
        writeln!(f, "{}", display_components(&self.call_rel_imm))?;
        writeln!(f, "generic:")?;
        writeln!(f, "{}", display_components(&self.generic))?;
        writeln!(f, "jnz:")?;
        writeln!(f, "{}", display_components(&self.jnz))?;
        writeln!(f, "jnz_taken:")?;
        writeln!(f, "{}", display_components(&self.jnz_taken))?;
        writeln!(f, "jump:")?;
        writeln!(f, "{}", display_components(&self.jump))?;
        writeln!(f, "jump_double_deref:")?;
        writeln!(f, "{}", display_components(&self.jump_double_deref))?;
        writeln!(f, "jump_rel:")?;
        writeln!(f, "{}", display_components(&self.jump_rel))?;
        writeln!(f, "jump_rel_imm:")?;
        writeln!(f, "{}", display_components(&self.jump_rel_imm))?;
        writeln!(f, "mul:")?;
        writeln!(f, "{}", display_components(&self.mul))?;
        writeln!(f, "mul_small:")?;
        writeln!(f, "{}", display_components(&self.mul_small))?;
        writeln!(f, "qm31:")?;
        writeln!(f, "{}", display_components(&self.qm31))?;
        writeln!(f, "ret:")?;
        writeln!(f, "{}", display_components(&self.ret))?;
        Ok(())
    }
}
