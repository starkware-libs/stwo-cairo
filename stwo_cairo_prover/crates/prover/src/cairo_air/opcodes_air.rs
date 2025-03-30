use itertools::{chain, Itertools};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo_cairo_serialize::CairoSerialize;
use stwo_prover::constraint_framework::TraceLocationAllocator;
use stwo_prover::core::air::ComponentProver;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::qm31::{SecureField, QM31};
use stwo_prover::core::pcs::TreeVec;

use super::air::CairoInteractionElements;
use super::debug_tools::display_components;
use crate::cairo_air::components::{
    add_ap_opcode, add_ap_opcode_imm, add_ap_opcode_op_1_base_fp, add_opcode, add_opcode_imm,
    add_opcode_small, add_opcode_small_imm, assert_eq_opcode, assert_eq_opcode_double_deref,
    assert_eq_opcode_imm, blake_compress_opcode, call_opcode, call_opcode_op_1_base_fp,
    call_opcode_rel, generic_opcode, jnz_opcode, jnz_opcode_dst_base_fp, jnz_opcode_taken,
    jnz_opcode_taken_dst_base_fp, jump_opcode, jump_opcode_double_deref, jump_opcode_rel,
    jump_opcode_rel_imm, mul_opcode, mul_opcode_imm, mul_opcode_small, mul_opcode_small_imm,
    qm_31_add_mul_opcode, ret_opcode,
};

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct OpcodeClaim {
    pub add: Vec<add_opcode::Claim>,
    pub add_imm: Vec<add_opcode_imm::Claim>,
    pub add_small: Vec<add_opcode_small::Claim>,
    pub add_small_imm: Vec<add_opcode_small_imm::Claim>,
    pub add_ap: Vec<add_ap_opcode::Claim>,
    pub add_ap_op_1_base_fp: Vec<add_ap_opcode_op_1_base_fp::Claim>,
    pub add_ap_imm: Vec<add_ap_opcode_imm::Claim>,
    pub assert_eq: Vec<assert_eq_opcode::Claim>,
    pub assert_eq_imm: Vec<assert_eq_opcode_imm::Claim>,
    pub assert_eq_double_deref: Vec<assert_eq_opcode_double_deref::Claim>,
    pub blake: Vec<blake_compress_opcode::Claim>,
    pub call: Vec<call_opcode::Claim>,
    pub call_op_1_base_fp: Vec<call_opcode_op_1_base_fp::Claim>,
    pub call_rel: Vec<call_opcode_rel::Claim>,
    pub generic: Vec<generic_opcode::Claim>,
    pub jnz: Vec<jnz_opcode::Claim>,
    pub jnz_dst_base_fp: Vec<jnz_opcode_dst_base_fp::Claim>,
    pub jnz_taken: Vec<jnz_opcode_taken::Claim>,
    pub jnz_taken_dst_base_fp: Vec<jnz_opcode_taken_dst_base_fp::Claim>,
    pub jump: Vec<jump_opcode::Claim>,
    pub jump_double_deref: Vec<jump_opcode_double_deref::Claim>,
    pub jump_rel: Vec<jump_opcode_rel::Claim>,
    pub jump_rel_imm: Vec<jump_opcode_rel_imm::Claim>,
    pub mul: Vec<mul_opcode::Claim>,
    pub mul_imm: Vec<mul_opcode_imm::Claim>,
    pub mul_small: Vec<mul_opcode_small::Claim>,
    pub mul_small_imm: Vec<mul_opcode_small_imm::Claim>,
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
        mix_component_vector!(add_imm);
        mix_component_vector!(add_small);
        mix_component_vector!(add_small_imm);
        mix_component_vector!(add_ap);
        mix_component_vector!(add_ap_op_1_base_fp);
        mix_component_vector!(add_ap_imm);
        mix_component_vector!(assert_eq);
        mix_component_vector!(assert_eq_imm);
        mix_component_vector!(assert_eq_double_deref);
        mix_component_vector!(blake);
        mix_component_vector!(call);
        mix_component_vector!(call_op_1_base_fp);
        mix_component_vector!(call_rel);
        mix_component_vector!(generic);
        mix_component_vector!(jnz);
        mix_component_vector!(jnz_dst_base_fp);
        mix_component_vector!(jnz_taken);
        mix_component_vector!(jnz_taken_dst_base_fp);
        mix_component_vector!(jump);
        mix_component_vector!(jump_double_deref);
        mix_component_vector!(jump_rel);
        mix_component_vector!(jump_rel_imm);
        mix_component_vector!(mul);
        mix_component_vector!(mul_imm);
        mix_component_vector!(mul_small);
        mix_component_vector!(mul_small_imm);
        mix_component_vector!(qm31);
        mix_component_vector!(ret);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        TreeVec::concat_cols(chain!(
            self.add.iter().map(|c| c.log_sizes()),
            self.add_imm.iter().map(|c| c.log_sizes()),
            self.add_small.iter().map(|c| c.log_sizes()),
            self.add_small_imm.iter().map(|c| c.log_sizes()),
            self.add_ap.iter().map(|c| c.log_sizes()),
            self.add_ap_op_1_base_fp.iter().map(|c| c.log_sizes()),
            self.add_ap_imm.iter().map(|c| c.log_sizes()),
            self.assert_eq.iter().map(|c| c.log_sizes()),
            self.assert_eq_imm.iter().map(|c| c.log_sizes()),
            self.assert_eq_double_deref.iter().map(|c| c.log_sizes()),
            self.blake.iter().map(|c| c.log_sizes()),
            self.call.iter().map(|c| c.log_sizes()),
            self.call_op_1_base_fp.iter().map(|c| c.log_sizes()),
            self.call_rel.iter().map(|c| c.log_sizes()),
            self.generic.iter().map(|c| c.log_sizes()),
            self.jnz.iter().map(|c| c.log_sizes()),
            self.jnz_dst_base_fp.iter().map(|c| c.log_sizes()),
            self.jnz_taken.iter().map(|c| c.log_sizes()),
            self.jnz_taken_dst_base_fp.iter().map(|c| c.log_sizes()),
            self.jump.iter().map(|c| c.log_sizes()),
            self.jump_double_deref.iter().map(|c| c.log_sizes()),
            self.jump_rel.iter().map(|c| c.log_sizes()),
            self.jump_rel_imm.iter().map(|c| c.log_sizes()),
            self.mul.iter().map(|c| c.log_sizes()),
            self.mul_imm.iter().map(|c| c.log_sizes()),
            self.mul_small.iter().map(|c| c.log_sizes()),
            self.mul_small_imm.iter().map(|c| c.log_sizes()),
            self.qm31.iter().map(|c| c.log_sizes()),
            self.ret.iter().map(|c| c.log_sizes()),
        ))
    }
}

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct OpcodeInteractionClaim {
    pub add: Vec<add_opcode::InteractionClaim>,
    pub add_imm: Vec<add_opcode_imm::InteractionClaim>,
    pub add_small: Vec<add_opcode_small::InteractionClaim>,
    pub add_small_imm: Vec<add_opcode_small_imm::InteractionClaim>,
    pub add_ap: Vec<add_ap_opcode::InteractionClaim>,
    pub add_ap_op_1_base_fp: Vec<add_ap_opcode_op_1_base_fp::InteractionClaim>,
    pub add_ap_imm: Vec<add_ap_opcode_imm::InteractionClaim>,
    pub assert_eq: Vec<assert_eq_opcode::InteractionClaim>,
    pub assert_eq_imm: Vec<assert_eq_opcode_imm::InteractionClaim>,
    pub assert_eq_double_deref: Vec<assert_eq_opcode_double_deref::InteractionClaim>,
    pub blake: Vec<blake_compress_opcode::InteractionClaim>,
    pub call: Vec<call_opcode::InteractionClaim>,
    pub call_op_1_base_fp: Vec<call_opcode_op_1_base_fp::InteractionClaim>,
    pub call_rel: Vec<call_opcode_rel::InteractionClaim>,
    pub generic: Vec<generic_opcode::InteractionClaim>,
    pub jnz: Vec<jnz_opcode::InteractionClaim>,
    pub jnz_dst_base_fp: Vec<jnz_opcode_dst_base_fp::InteractionClaim>,
    pub jnz_taken: Vec<jnz_opcode_taken::InteractionClaim>,
    pub jnz_taken_dst_base_fp: Vec<jnz_opcode_taken_dst_base_fp::InteractionClaim>,
    pub jump: Vec<jump_opcode::InteractionClaim>,
    pub jump_double_deref: Vec<jump_opcode_double_deref::InteractionClaim>,
    pub jump_rel: Vec<jump_opcode_rel::InteractionClaim>,
    pub jump_rel_imm: Vec<jump_opcode_rel_imm::InteractionClaim>,
    pub mul: Vec<mul_opcode::InteractionClaim>,
    pub mul_imm: Vec<mul_opcode_imm::InteractionClaim>,
    pub mul_small: Vec<mul_opcode_small::InteractionClaim>,
    pub mul_small_imm: Vec<mul_opcode_small_imm::InteractionClaim>,
    pub qm31: Vec<qm_31_add_mul_opcode::InteractionClaim>,
    pub ret: Vec<ret_opcode::InteractionClaim>,
}
impl OpcodeInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.add.iter().for_each(|c| c.mix_into(channel));
        self.add_imm.iter().for_each(|c| c.mix_into(channel));
        self.add_small.iter().for_each(|c| c.mix_into(channel));
        self.add_small_imm.iter().for_each(|c| c.mix_into(channel));
        self.add_ap.iter().for_each(|c| c.mix_into(channel));
        self.add_ap_op_1_base_fp
            .iter()
            .for_each(|c| c.mix_into(channel));
        self.add_ap_imm.iter().for_each(|c| c.mix_into(channel));
        self.assert_eq.iter().for_each(|c| c.mix_into(channel));
        self.assert_eq_imm.iter().for_each(|c| c.mix_into(channel));
        self.assert_eq_double_deref
            .iter()
            .for_each(|c| c.mix_into(channel));
        self.blake.iter().for_each(|c| c.mix_into(channel));
        self.call.iter().for_each(|c| c.mix_into(channel));
        self.call_op_1_base_fp
            .iter()
            .for_each(|c| c.mix_into(channel));
        self.call_rel.iter().for_each(|c| c.mix_into(channel));
        self.generic.iter().for_each(|c| c.mix_into(channel));
        self.jnz.iter().for_each(|c| c.mix_into(channel));
        self.jnz_dst_base_fp
            .iter()
            .for_each(|c| c.mix_into(channel));
        self.jnz_taken.iter().for_each(|c| c.mix_into(channel));
        self.jnz_taken_dst_base_fp
            .iter()
            .for_each(|c| c.mix_into(channel));
        self.jump.iter().for_each(|c| c.mix_into(channel));
        self.jump_double_deref
            .iter()
            .for_each(|c| c.mix_into(channel));
        self.jump_rel.iter().for_each(|c| c.mix_into(channel));
        self.jump_rel_imm.iter().for_each(|c| c.mix_into(channel));
        self.mul.iter().for_each(|c| c.mix_into(channel));
        self.mul_imm.iter().for_each(|c| c.mix_into(channel));
        self.mul_small.iter().for_each(|c| c.mix_into(channel));
        self.mul_small_imm.iter().for_each(|c| c.mix_into(channel));
        self.qm31.iter().for_each(|c| c.mix_into(channel));
        self.ret.iter().for_each(|c| c.mix_into(channel));
    }

    pub fn sum(&self) -> SecureField {
        let mut sum = QM31::zero();
        for interaction_claim in &self.add {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.add_imm {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.add_small {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.add_small_imm {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.add_ap {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.add_ap_op_1_base_fp {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.add_ap_imm {
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
        for interaction_claim in &self.call_op_1_base_fp {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.call_rel {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.generic {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.jnz {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.jnz_dst_base_fp {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.jnz_taken {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.jnz_taken_dst_base_fp {
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
        for interaction_claim in &self.mul_imm {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.mul_small {
            sum += interaction_claim.claimed_sum;
        }
        for interaction_claim in &self.mul_small_imm {
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
    pub add_imm: Vec<add_opcode_imm::Component>,
    pub add_small: Vec<add_opcode_small::Component>,
    pub add_small_imm: Vec<add_opcode_small_imm::Component>,
    pub add_ap: Vec<add_ap_opcode::Component>,
    pub add_ap_op_1_base_fp: Vec<add_ap_opcode_op_1_base_fp::Component>,
    pub add_ap_imm: Vec<add_ap_opcode_imm::Component>,
    pub assert_eq: Vec<assert_eq_opcode::Component>,
    pub assert_eq_imm: Vec<assert_eq_opcode_imm::Component>,
    pub assert_eq_double_deref: Vec<assert_eq_opcode_double_deref::Component>,
    pub blake: Vec<blake_compress_opcode::Component>,
    pub call: Vec<call_opcode::Component>,
    pub call_op_1_base_fp: Vec<call_opcode_op_1_base_fp::Component>,
    pub call_rel: Vec<call_opcode_rel::Component>,
    pub generic: Vec<generic_opcode::Component>,
    pub jnz: Vec<jnz_opcode::Component>,
    pub jnz_dst_base_fp: Vec<jnz_opcode_dst_base_fp::Component>,
    pub jnz_taken: Vec<jnz_opcode_taken::Component>,
    pub jnz_taken_dst_base_fp: Vec<jnz_opcode_taken_dst_base_fp::Component>,
    pub jump: Vec<jump_opcode::Component>,
    pub jump_double_deref: Vec<jump_opcode_double_deref::Component>,
    pub jump_rel: Vec<jump_opcode_rel::Component>,
    pub jump_rel_imm: Vec<jump_opcode_rel_imm::Component>,
    pub mul: Vec<mul_opcode::Component>,
    pub mul_imm: Vec<mul_opcode_imm::Component>,
    pub mul_small: Vec<mul_opcode_small::Component>,
    pub mul_small_imm: Vec<mul_opcode_small_imm::Component>,
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
        let add_imm_components = claim
            .add_imm
            .iter()
            .zip(interaction_claim.add_imm.iter())
            .map(|(&claim, &interaction_claim)| {
                add_opcode_imm::Component::new(
                    tree_span_provider,
                    add_opcode_imm::Eval {
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
        let add_small_imm_components = claim
            .add_small_imm
            .iter()
            .zip(interaction_claim.add_small_imm.iter())
            .map(|(&claim, &interaction_claim)| {
                add_opcode_small_imm::Component::new(
                    tree_span_provider,
                    add_opcode_small_imm::Eval {
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
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let add_ap_op_1_base_fp_components = claim
            .add_ap_op_1_base_fp
            .iter()
            .zip(interaction_claim.add_ap_op_1_base_fp.iter())
            .map(|(&claim, &interaction_claim)| {
                add_ap_opcode_op_1_base_fp::Component::new(
                    tree_span_provider,
                    add_ap_opcode_op_1_base_fp::Eval {
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
        let add_ap_imm_components = claim
            .add_ap_imm
            .iter()
            .zip(interaction_claim.add_ap_imm.iter())
            .map(|(&claim, &interaction_claim)| {
                add_ap_opcode_imm::Component::new(
                    tree_span_provider,
                    add_ap_opcode_imm::Eval {
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
                call_opcode::Component::new(
                    tree_span_provider,
                    call_opcode::Eval {
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
        let call_op_1_base_fp_components = claim
            .call_op_1_base_fp
            .iter()
            .zip(interaction_claim.call_op_1_base_fp.iter())
            .map(|(&claim, &interaction_claim)| {
                call_opcode_op_1_base_fp::Component::new(
                    tree_span_provider,
                    call_opcode_op_1_base_fp::Eval {
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
        let call_rel_components = claim
            .call_rel
            .iter()
            .zip(interaction_claim.call_rel.iter())
            .map(|(&claim, &interaction_claim)| {
                call_opcode_rel::Component::new(
                    tree_span_provider,
                    call_opcode_rel::Eval {
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
                        range_check_19_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19
                            .clone(),
                        range_check_9_9_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9
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
                jnz_opcode::Component::new(
                    tree_span_provider,
                    jnz_opcode::Eval {
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
        let jnz_dst_base_fp_components = claim
            .jnz_dst_base_fp
            .iter()
            .zip(interaction_claim.jnz_dst_base_fp.iter())
            .map(|(&claim, &interaction_claim)| {
                jnz_opcode_dst_base_fp::Component::new(
                    tree_span_provider,
                    jnz_opcode_dst_base_fp::Eval {
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
        let jnz_taken_dst_base_fp_components = claim
            .jnz_taken_dst_base_fp
            .iter()
            .zip(interaction_claim.jnz_taken_dst_base_fp.iter())
            .map(|(&claim, &interaction_claim)| {
                jnz_opcode_taken_dst_base_fp::Component::new(
                    tree_span_provider,
                    jnz_opcode_taken_dst_base_fp::Eval {
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
                jump_opcode::Component::new(
                    tree_span_provider,
                    jump_opcode::Eval {
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
                        range_check_19_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19
                            .clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let mul_imm_components = claim
            .mul_imm
            .iter()
            .zip(interaction_claim.mul_imm.iter())
            .map(|(&claim, &interaction_claim)| {
                mul_opcode_imm::Component::new(
                    tree_span_provider,
                    mul_opcode_imm::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        range_check_19_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19
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
        let mul_small_imm_components = claim
            .mul_small_imm
            .iter()
            .zip(interaction_claim.mul_small_imm.iter())
            .map(|(&claim, &interaction_claim)| {
                mul_opcode_small_imm::Component::new(
                    tree_span_provider,
                    mul_opcode_small_imm::Eval {
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
            add_imm: add_imm_components,
            add_small: add_small_components,
            add_small_imm: add_small_imm_components,
            add_ap: add_ap_components,
            add_ap_op_1_base_fp: add_ap_op_1_base_fp_components,
            add_ap_imm: add_ap_imm_components,
            assert_eq: assert_eq_components,
            assert_eq_imm: assert_eq_imm_components,
            assert_eq_double_deref: assert_eq_double_deref_components,
            blake: blake_components,
            call: call_components,
            call_op_1_base_fp: call_op_1_base_fp_components,
            call_rel: call_rel_components,
            generic: generic_components,
            jnz: jnz_components,
            jnz_dst_base_fp: jnz_dst_base_fp_components,
            jnz_taken: jnz_taken_components,
            jnz_taken_dst_base_fp: jnz_taken_dst_base_fp_components,
            jump: jump_components,
            jump_double_deref: jump_double_deref_components,
            jump_rel: jump_rel_components,
            jump_rel_imm: jump_rel_imm_components,
            mul: mul_components,
            mul_imm: mul_imm_components,
            mul_small: mul_small_components,
            mul_small_imm: mul_small_imm_components,
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
            self.add_imm
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.add_small
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.add_small_imm
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.add_ap
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.add_ap_op_1_base_fp
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.add_ap_imm
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
            self.call_op_1_base_fp
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.call_rel
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
            self.jnz_dst_base_fp
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.jnz_taken
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.jnz_taken_dst_base_fp
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
            self.mul_imm
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.mul_small
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.mul_small_imm
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
        writeln!(f, "add_imm:")?;
        writeln!(f, "{}", display_components(&self.add_imm))?;
        writeln!(f, "add_small:")?;
        writeln!(f, "{}", display_components(&self.add_small))?;
        writeln!(f, "add_small_imm:")?;
        writeln!(f, "{}", display_components(&self.add_small_imm))?;
        writeln!(f, "add_ap:")?;
        writeln!(f, "{}", display_components(&self.add_ap))?;
        writeln!(f, "add_ap_op_1_base_fp:")?;
        writeln!(f, "{}", display_components(&self.add_ap_op_1_base_fp))?;
        writeln!(f, "add_ap_imm:")?;
        writeln!(f, "{}", display_components(&self.add_ap_imm))?;
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
        writeln!(f, "call_op_1_base_fp:")?;
        writeln!(f, "{}", display_components(&self.call_op_1_base_fp))?;
        writeln!(f, "call_rel:")?;
        writeln!(f, "{}", display_components(&self.call_rel))?;
        writeln!(f, "generic:")?;
        writeln!(f, "{}", display_components(&self.generic))?;
        writeln!(f, "jnz:")?;
        writeln!(f, "{}", display_components(&self.jnz))?;
        writeln!(f, "jnz_dst_base_fp:")?;
        writeln!(f, "{}", display_components(&self.jnz_dst_base_fp))?;
        writeln!(f, "jnz_taken:")?;
        writeln!(f, "{}", display_components(&self.jnz_taken))?;
        writeln!(f, "jnz_taken_dst_base_fp:")?;
        writeln!(f, "{}", display_components(&self.jnz_taken_dst_base_fp))?;
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
        writeln!(f, "mul_imm:")?;
        writeln!(f, "{}", display_components(&self.mul_imm))?;
        writeln!(f, "mul_small:")?;
        writeln!(f, "{}", display_components(&self.mul_small))?;
        writeln!(f, "mul_small_imm:")?;
        writeln!(f, "{}", display_components(&self.mul_small_imm))?;
        writeln!(f, "qm31:")?;
        writeln!(f, "{}", display_components(&self.qm31))?;
        writeln!(f, "ret:")?;
        writeln!(f, "{}", display_components(&self.ret))?;
        Ok(())
    }
}
