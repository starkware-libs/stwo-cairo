use itertools::{chain, Itertools};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo_cairo_serialize::CairoSerialize;
use stwo_prover::constraint_framework::TraceLocationAllocator;
use stwo_prover::core::air::ComponentProver;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::BackendForChannel;
use stwo_prover::core::channel::{Channel, MerkleChannel};
use stwo_prover::core::fields::qm31::{SecureField, QM31};
use stwo_prover::core::pcs::{TreeBuilder, TreeVec};

use super::air::CairoInteractionElements;
use super::debug_tools::display_components;
use crate::components::{
    add_ap_opcode, add_ap_opcode_imm, add_ap_opcode_op_1_base_fp, add_opcode, add_opcode_imm,
    add_opcode_small, add_opcode_small_imm, assert_eq_opcode, assert_eq_opcode_double_deref,
    assert_eq_opcode_imm, call_opcode, call_opcode_op_1_base_fp, call_opcode_rel, generic_opcode,
    jnz_opcode, jnz_opcode_dst_base_fp, jnz_opcode_taken, jnz_opcode_taken_dst_base_fp,
    jump_opcode, jump_opcode_double_deref, jump_opcode_rel, jump_opcode_rel_imm,
    memory_address_to_id, memory_id_to_big, mul_opcode, mul_opcode_imm, range_check_19,
    range_check_9_9, ret_opcode, verify_instruction,
};
use crate::input::state_transitions::StateTransitions;

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct OpcodeClaim {
    pub add_f_f: Vec<add_opcode::Claim>,
    pub add_f_t: Vec<add_opcode_imm::Claim>,
    pub add_t_f: Vec<add_opcode_small::Claim>,
    pub add_t_t: Vec<add_opcode_small_imm::Claim>,
    pub add_ap_f_f: Vec<add_ap_opcode::Claim>,
    pub add_ap_f_t: Vec<add_ap_opcode_op_1_base_fp::Claim>,
    pub add_ap_t_f: Vec<add_ap_opcode_imm::Claim>,
    pub assert_eq_f_f: Vec<assert_eq_opcode::Claim>,
    pub assert_eq_f_t: Vec<assert_eq_opcode_imm::Claim>,
    pub assert_eq_t_f: Vec<assert_eq_opcode_double_deref::Claim>,
    pub call_f_f: Vec<call_opcode::Claim>,
    pub call_f_t: Vec<call_opcode_op_1_base_fp::Claim>,
    pub call_t_f: Vec<call_opcode_rel::Claim>,
    pub generic: Vec<generic_opcode::Claim>,
    pub jnz_f_f: Vec<jnz_opcode::Claim>,
    pub jnz_f_t: Vec<jnz_opcode_dst_base_fp::Claim>,
    pub jnz_t_f: Vec<jnz_opcode_taken::Claim>,
    pub jnz_t_t: Vec<jnz_opcode_taken_dst_base_fp::Claim>,
    pub jump_f_f_f: Vec<jump_opcode::Claim>,
    pub jump_f_f_t: Vec<jump_opcode_double_deref::Claim>,
    pub jump_t_f_f: Vec<jump_opcode_rel::Claim>,
    pub jump_t_t_f: Vec<jump_opcode_rel_imm::Claim>,
    pub mul_f_f: Vec<mul_opcode::Claim>,
    pub mul_f_t: Vec<mul_opcode_imm::Claim>,
    pub ret: Vec<ret_opcode::Claim>,
}
impl OpcodeClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.add_f_f.iter().for_each(|c| c.mix_into(channel));
        self.add_f_t.iter().for_each(|c| c.mix_into(channel));
        self.add_t_f.iter().for_each(|c| c.mix_into(channel));
        self.add_t_t.iter().for_each(|c| c.mix_into(channel));
        self.add_ap_f_f.iter().for_each(|c| c.mix_into(channel));
        self.add_ap_f_t.iter().for_each(|c| c.mix_into(channel));
        self.add_ap_t_f.iter().for_each(|c| c.mix_into(channel));
        self.assert_eq_f_f.iter().for_each(|c| c.mix_into(channel));
        self.assert_eq_f_t.iter().for_each(|c| c.mix_into(channel));
        self.assert_eq_t_f.iter().for_each(|c| c.mix_into(channel));
        self.call_f_f.iter().for_each(|c| c.mix_into(channel));
        self.call_f_t.iter().for_each(|c| c.mix_into(channel));
        self.call_t_f.iter().for_each(|c| c.mix_into(channel));
        self.generic.iter().for_each(|c| c.mix_into(channel));
        self.jnz_f_f.iter().for_each(|c| c.mix_into(channel));
        self.jnz_f_t.iter().for_each(|c| c.mix_into(channel));
        self.jnz_t_f.iter().for_each(|c| c.mix_into(channel));
        self.jnz_t_t.iter().for_each(|c| c.mix_into(channel));
        self.jump_f_f_f.iter().for_each(|c| c.mix_into(channel));
        self.jump_f_f_t.iter().for_each(|c| c.mix_into(channel));
        self.jump_t_f_f.iter().for_each(|c| c.mix_into(channel));
        self.jump_t_t_f.iter().for_each(|c| c.mix_into(channel));
        self.mul_f_f.iter().for_each(|c| c.mix_into(channel));
        self.mul_f_t.iter().for_each(|c| c.mix_into(channel));
        self.ret.iter().for_each(|c| c.mix_into(channel));
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        TreeVec::concat_cols(chain!(
            self.add_f_f.iter().map(|c| c.log_sizes()),
            self.add_f_t.iter().map(|c| c.log_sizes()),
            self.add_t_f.iter().map(|c| c.log_sizes()),
            self.add_t_t.iter().map(|c| c.log_sizes()),
            self.add_ap_f_f.iter().map(|c| c.log_sizes()),
            self.add_ap_f_t.iter().map(|c| c.log_sizes()),
            self.add_ap_t_f.iter().map(|c| c.log_sizes()),
            self.assert_eq_f_f.iter().map(|c| c.log_sizes()),
            self.assert_eq_f_t.iter().map(|c| c.log_sizes()),
            self.assert_eq_t_f.iter().map(|c| c.log_sizes()),
            self.call_f_f.iter().map(|c| c.log_sizes()),
            self.call_f_t.iter().map(|c| c.log_sizes()),
            self.call_t_f.iter().map(|c| c.log_sizes()),
            self.generic.iter().map(|c| c.log_sizes()),
            self.jnz_f_f.iter().map(|c| c.log_sizes()),
            self.jnz_f_t.iter().map(|c| c.log_sizes()),
            self.jnz_t_f.iter().map(|c| c.log_sizes()),
            self.jnz_t_t.iter().map(|c| c.log_sizes()),
            self.jump_f_f_f.iter().map(|c| c.log_sizes()),
            self.jump_f_f_t.iter().map(|c| c.log_sizes()),
            self.jump_t_f_f.iter().map(|c| c.log_sizes()),
            self.jump_t_t_f.iter().map(|c| c.log_sizes()),
            self.mul_f_f.iter().map(|c| c.log_sizes()),
            self.mul_f_t.iter().map(|c| c.log_sizes()),
            self.ret.iter().map(|c| c.log_sizes()),
        ))
    }
}

pub struct OpcodesClaimGenerator {
    add_f_f: Vec<add_opcode::ClaimGenerator>,
    add_f_t: Vec<add_opcode_imm::ClaimGenerator>,
    add_t_f: Vec<add_opcode_small::ClaimGenerator>,
    add_t_t: Vec<add_opcode_small_imm::ClaimGenerator>,
    add_ap_f_f: Vec<add_ap_opcode::ClaimGenerator>,
    add_ap_f_t: Vec<add_ap_opcode_op_1_base_fp::ClaimGenerator>,
    add_ap_t_f: Vec<add_ap_opcode_imm::ClaimGenerator>,
    assert_eq_f_f: Vec<assert_eq_opcode::ClaimGenerator>,
    assert_eq_f_t: Vec<assert_eq_opcode_imm::ClaimGenerator>,
    assert_eq_t_f: Vec<assert_eq_opcode_double_deref::ClaimGenerator>,
    call_f_f: Vec<call_opcode::ClaimGenerator>,
    call_f_t: Vec<call_opcode_op_1_base_fp::ClaimGenerator>,
    call_t_f: Vec<call_opcode_rel::ClaimGenerator>,
    generic: Vec<generic_opcode::ClaimGenerator>,
    jnz_f_f: Vec<jnz_opcode::ClaimGenerator>,
    jnz_f_t: Vec<jnz_opcode_dst_base_fp::ClaimGenerator>,
    jnz_t_f: Vec<jnz_opcode_taken::ClaimGenerator>,
    jnz_t_t: Vec<jnz_opcode_taken_dst_base_fp::ClaimGenerator>,
    jump_f_f_f: Vec<jump_opcode::ClaimGenerator>,
    jump_f_f_t: Vec<jump_opcode_double_deref::ClaimGenerator>,
    jump_t_f_f: Vec<jump_opcode_rel::ClaimGenerator>,
    jump_t_t_f: Vec<jump_opcode_rel_imm::ClaimGenerator>,
    mul_f_f: Vec<mul_opcode::ClaimGenerator>,
    mul_f_t: Vec<mul_opcode_imm::ClaimGenerator>,
    ret: Vec<ret_opcode::ClaimGenerator>,
}
impl OpcodesClaimGenerator {
    pub fn new(input: StateTransitions) -> Self {
        // TODO(Ohad): decide split sizes for opcode traces.
        let mut add_f_f = vec![];
        let mut add_f_t = vec![];
        let mut add_t_f = vec![];
        let mut add_t_t = vec![];
        let mut add_ap_f_f = vec![];
        let mut add_ap_f_t = vec![];
        let mut add_ap_t_f = vec![];
        let mut assert_eq_f_f = vec![];
        let mut assert_eq_f_t = vec![];
        let mut assert_eq_t_f = vec![];
        let mut call_f_f = vec![];
        let mut call_f_t = vec![];
        let mut call_t_f = vec![];
        let mut generic = vec![];
        let mut jnz_f_f = vec![];
        let mut jnz_f_t = vec![];
        let mut jnz_t_f = vec![];
        let mut jnz_t_t = vec![];
        let mut jump_f_f_f = vec![];
        let mut jump_f_f_t = vec![];
        let mut jump_t_f_f = vec![];
        let mut jump_t_t_f = vec![];
        let mut mul_f_f = vec![];
        let mut mul_f_t = vec![];
        let mut ret = vec![];
        if !input.casm_states_by_opcode.add_opcode.is_empty() {
            add_f_f.push(add_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.add_opcode,
            ));
        }
        if !input.casm_states_by_opcode.add_opcode_imm.is_empty() {
            add_f_t.push(add_opcode_imm::ClaimGenerator::new(
                input.casm_states_by_opcode.add_opcode_imm,
            ));
        }
        if !input.casm_states_by_opcode.add_opcode_small.is_empty() {
            add_t_f.push(add_opcode_small::ClaimGenerator::new(
                input.casm_states_by_opcode.add_opcode_small,
            ));
        }
        if !input.casm_states_by_opcode.add_opcode_small_imm.is_empty() {
            add_t_t.push(add_opcode_small_imm::ClaimGenerator::new(
                input.casm_states_by_opcode.add_opcode_small_imm,
            ));
        }
        if !input.casm_states_by_opcode.add_ap_opcode.is_empty() {
            add_ap_f_f.push(add_ap_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.add_ap_opcode,
            ));
        }
        if !input
            .casm_states_by_opcode
            .add_ap_opcode_op_1_base_fp
            .is_empty()
        {
            add_ap_f_t.push(add_ap_opcode_op_1_base_fp::ClaimGenerator::new(
                input.casm_states_by_opcode.add_ap_opcode_op_1_base_fp,
            ));
        }
        if !input.casm_states_by_opcode.add_ap_opcode_imm.is_empty() {
            add_ap_t_f.push(add_ap_opcode_imm::ClaimGenerator::new(
                input.casm_states_by_opcode.add_ap_opcode_imm,
            ));
        }
        if !input.casm_states_by_opcode.assert_eq_opcode.is_empty() {
            assert_eq_f_f.push(assert_eq_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.assert_eq_opcode,
            ));
        }
        if !input.casm_states_by_opcode.assert_eq_opcode_imm.is_empty() {
            assert_eq_f_t.push(assert_eq_opcode_imm::ClaimGenerator::new(
                input.casm_states_by_opcode.assert_eq_opcode_imm,
            ));
        }
        if !input
            .casm_states_by_opcode
            .assert_eq_opcode_double_deref
            .is_empty()
        {
            assert_eq_t_f.push(assert_eq_opcode_double_deref::ClaimGenerator::new(
                input.casm_states_by_opcode.assert_eq_opcode_double_deref,
            ));
        }
        if !input.casm_states_by_opcode.call_opcode.is_empty() {
            call_f_f.push(call_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.call_opcode,
            ));
        }
        if !input
            .casm_states_by_opcode
            .call_opcode_op_1_base_fp
            .is_empty()
        {
            call_f_t.push(call_opcode_op_1_base_fp::ClaimGenerator::new(
                input.casm_states_by_opcode.call_opcode_op_1_base_fp,
            ));
        }
        if !input.casm_states_by_opcode.call_opcode_rel.is_empty() {
            call_t_f.push(call_opcode_rel::ClaimGenerator::new(
                input.casm_states_by_opcode.call_opcode_rel,
            ));
        }
        if !input.casm_states_by_opcode.generic_opcode.is_empty() {
            generic.push(generic_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.generic_opcode,
            ));
        }
        if !input.casm_states_by_opcode.jnz_opcode.is_empty() {
            jnz_f_f.push(jnz_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.jnz_opcode,
            ));
        }
        if !input
            .casm_states_by_opcode
            .jnz_opcode_dst_base_fp
            .is_empty()
        {
            jnz_f_t.push(jnz_opcode_dst_base_fp::ClaimGenerator::new(
                input.casm_states_by_opcode.jnz_opcode_dst_base_fp,
            ));
        }
        if !input.casm_states_by_opcode.jnz_opcode_taken.is_empty() {
            jnz_t_f.push(jnz_opcode_taken::ClaimGenerator::new(
                input.casm_states_by_opcode.jnz_opcode_taken,
            ));
        }
        if !input
            .casm_states_by_opcode
            .jnz_opcode_taken_dst_base_fp
            .is_empty()
        {
            jnz_t_t.push(jnz_opcode_taken_dst_base_fp::ClaimGenerator::new(
                input.casm_states_by_opcode.jnz_opcode_taken_dst_base_fp,
            ));
        }
        if !input.casm_states_by_opcode.jump_opcode.is_empty() {
            jump_f_f_f.push(jump_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.jump_opcode,
            ));
        }
        if !input
            .casm_states_by_opcode
            .jump_opcode_double_deref
            .is_empty()
        {
            jump_f_f_t.push(jump_opcode_double_deref::ClaimGenerator::new(
                input.casm_states_by_opcode.jump_opcode_double_deref,
            ));
        }
        if !input.casm_states_by_opcode.jump_opcode_rel.is_empty() {
            jump_t_f_f.push(jump_opcode_rel::ClaimGenerator::new(
                input.casm_states_by_opcode.jump_opcode_rel,
            ));
        }
        if !input.casm_states_by_opcode.jump_opcode_rel_imm.is_empty() {
            jump_t_t_f.push(jump_opcode_rel_imm::ClaimGenerator::new(
                input.casm_states_by_opcode.jump_opcode_rel_imm,
            ));
        }
        // Handle small mul in big mul component. Temporary until airs are written with Rc_3_6_6.
        // TODO(Ohad): mul small.
        if !input.casm_states_by_opcode.mul_opcode.is_empty() {
            mul_f_f.push(mul_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.mul_opcode,
            ));
        }
        if !input.casm_states_by_opcode.mul_opcode_imm.is_empty() {
            mul_f_t.push(mul_opcode_imm::ClaimGenerator::new(
                input.casm_states_by_opcode.mul_opcode_imm,
            ));
        }
        if !input.casm_states_by_opcode.ret_opcode.is_empty() {
            ret.push(ret_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.ret_opcode,
            ));
        }
        Self {
            add_f_f,
            add_f_t,
            add_t_f,
            add_t_t,
            add_ap_f_f,
            add_ap_f_t,
            add_ap_t_f,
            assert_eq_f_f,
            assert_eq_f_t,
            assert_eq_t_f,
            call_f_f,
            call_f_t,
            call_t_f,
            generic,
            jnz_f_f,
            jnz_f_t,
            jnz_t_f,
            jnz_t_t,
            jump_f_f_f,
            jump_f_f_t,
            jump_t_f_f,
            jump_t_t_f,
            mul_f_f,
            mul_f_t,
            ret,
        }
    }

    pub fn write_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        memory_address_to_id_trace_generator: &memory_address_to_id::ClaimGenerator,
        memory_id_to_value_trace_generator: &memory_id_to_big::ClaimGenerator,
        range_check_19_trace_generator: &range_check_19::ClaimGenerator,
        range_check_9_9_trace_generator: &range_check_9_9::ClaimGenerator,
        verify_instruction_trace_generator: &verify_instruction::ClaimGenerator,
    ) -> (OpcodeClaim, OpcodesInteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let (add_f_f_claims, add_f_f_interaction_gens) = self
            .add_f_f
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (add_f_t_claims, add_f_t_interaction_gens) = self
            .add_f_t
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (add_t_f_claims, add_t_f_interaction_gens) = self
            .add_t_f
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (add_t_t_claims, add_t_t_interaction_gens) = self
            .add_t_t
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (add_ap_f_f_claims, add_ap_f_f_interaction_gens) = self
            .add_ap_f_f
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (add_ap_f_t_claims, add_ap_f_t_interaction_gens) = self
            .add_ap_f_t
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (add_ap_t_f_claims, add_ap_t_f_interaction_gens) = self
            .add_ap_t_f
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (assert_eq_f_f_claims, assert_eq_f_f_interaction_gens) = self
            .assert_eq_f_f
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (assert_eq_f_t_claims, assert_eq_f_t_interaction_gens) = self
            .assert_eq_f_t
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (assert_eq_t_f_claims, assert_eq_t_f_interaction_gens) = self
            .assert_eq_t_f
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (call_f_f_claims, call_f_f_interaction_gens) = self
            .call_f_f
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (call_f_t_claims, call_f_t_interaction_gens) = self
            .call_f_t
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (call_t_f_claims, call_t_f_interaction_gens) = self
            .call_t_f
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (generic_opcode_claims, generic_opcode_interaction_gens) = self
            .generic
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    range_check_19_trace_generator,
                    range_check_9_9_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (jnz_f_f_claims, jnz_f_f_interaction_gens) = self
            .jnz_f_f
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (jnz_f_t_claims, jnz_f_t_interaction_gens) = self
            .jnz_f_t
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (jnz_t_f_claims, jnz_t_f_interaction_gens) = self
            .jnz_t_f
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (jnz_t_t_claims, jnz_t_t_interaction_gens) = self
            .jnz_t_t
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (jump_f_f_f_claims, jump_f_f_f_interaction_gens) = self
            .jump_f_f_f
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (jump_f_f_t_claims, jump_f_f_t_interaction_gens) = self
            .jump_f_f_t
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (jump_t_f_f_claims, jump_t_f_f_interaction_gens) = self
            .jump_t_f_f
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (jump_t_t_f_claims, jump_t_t_f_interaction_gens) = self
            .jump_t_t_f
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (mul_f_f_claims, mul_f_f_interaction_gens) = self
            .mul_f_f
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    range_check_19_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (mul_f_t_claims, mul_f_t_interaction_gens) = self
            .mul_f_t
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    range_check_19_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (ret_claims, ret_interaction_gens) = self
            .ret
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        (
            OpcodeClaim {
                add_f_f: add_f_f_claims,
                add_f_t: add_f_t_claims,
                add_t_f: add_t_f_claims,
                add_t_t: add_t_t_claims,
                add_ap_f_f: add_ap_f_f_claims,
                add_ap_f_t: add_ap_f_t_claims,
                add_ap_t_f: add_ap_t_f_claims,
                assert_eq_f_f: assert_eq_f_f_claims,
                assert_eq_f_t: assert_eq_f_t_claims,
                assert_eq_t_f: assert_eq_t_f_claims,
                call_f_f: call_f_f_claims,
                call_f_t: call_f_t_claims,
                call_t_f: call_t_f_claims,
                generic: generic_opcode_claims,
                jnz_f_f: jnz_f_f_claims,
                jnz_f_t: jnz_f_t_claims,
                jnz_t_f: jnz_t_f_claims,
                jnz_t_t: jnz_t_t_claims,
                jump_f_f_f: jump_f_f_f_claims,
                jump_f_f_t: jump_f_f_t_claims,
                jump_t_f_f: jump_t_f_f_claims,
                jump_t_t_f: jump_t_t_f_claims,
                mul_f_f: mul_f_f_claims,
                mul_f_t: mul_f_t_claims,
                ret: ret_claims,
            },
            OpcodesInteractionClaimGenerator {
                add_f_f: add_f_f_interaction_gens,
                add_f_t: add_f_t_interaction_gens,
                add_t_f: add_t_f_interaction_gens,
                add_t_t: add_t_t_interaction_gens,
                add_ap_f_f: add_ap_f_f_interaction_gens,
                add_ap_f_t: add_ap_f_t_interaction_gens,
                add_ap_t_f: add_ap_t_f_interaction_gens,
                assert_eq_f_f: assert_eq_f_f_interaction_gens,
                assert_eq_f_t: assert_eq_f_t_interaction_gens,
                assert_eq_t_f: assert_eq_t_f_interaction_gens,
                call_f_f: call_f_f_interaction_gens,
                call_f_t: call_f_t_interaction_gens,
                call_t_f: call_t_f_interaction_gens,
                generic_opcode_interaction_gens,
                jnz_f_f: jnz_f_f_interaction_gens,
                jnz_f_t: jnz_f_t_interaction_gens,
                jnz_t_f: jnz_t_f_interaction_gens,
                jnz_t_t: jnz_t_t_interaction_gens,
                jump_f_f_f: jump_f_f_f_interaction_gens,
                jump_f_f_t: jump_f_f_t_interaction_gens,
                jump_t_f_f: jump_t_f_f_interaction_gens,
                jump_t_t_f: jump_t_t_f_interaction_gens,
                mul_f_f: mul_f_f_interaction_gens,
                mul_f_t: mul_f_t_interaction_gens,
                ret_interaction_gens,
            },
        )
    }
}

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct OpcodeInteractionClaim {
    add_f_f: Vec<add_opcode::InteractionClaim>,
    add_f_t: Vec<add_opcode_imm::InteractionClaim>,
    add_t_f: Vec<add_opcode_small::InteractionClaim>,
    add_t_t: Vec<add_opcode_small_imm::InteractionClaim>,
    add_ap_f_f: Vec<add_ap_opcode::InteractionClaim>,
    add_ap_f_t: Vec<add_ap_opcode_op_1_base_fp::InteractionClaim>,
    add_ap_t_f: Vec<add_ap_opcode_imm::InteractionClaim>,
    assert_eq_f_f: Vec<assert_eq_opcode::InteractionClaim>,
    assert_eq_f_t: Vec<assert_eq_opcode_imm::InteractionClaim>,
    assert_eq_t_f: Vec<assert_eq_opcode_double_deref::InteractionClaim>,
    call_f_f: Vec<call_opcode::InteractionClaim>,
    call_f_t: Vec<call_opcode_op_1_base_fp::InteractionClaim>,
    call_t_f: Vec<call_opcode_rel::InteractionClaim>,
    generic: Vec<generic_opcode::InteractionClaim>,
    jnz_f_f: Vec<jnz_opcode::InteractionClaim>,
    jnz_f_t: Vec<jnz_opcode_dst_base_fp::InteractionClaim>,
    jnz_t_f: Vec<jnz_opcode_taken::InteractionClaim>,
    jnz_t_t: Vec<jnz_opcode_taken_dst_base_fp::InteractionClaim>,
    jump_f_f_f: Vec<jump_opcode::InteractionClaim>,
    jump_f_f_t: Vec<jump_opcode_double_deref::InteractionClaim>,
    jump_t_f_f: Vec<jump_opcode_rel::InteractionClaim>,
    jump_t_t_f: Vec<jump_opcode_rel_imm::InteractionClaim>,
    mul_f_f: Vec<mul_opcode::InteractionClaim>,
    mul_f_t: Vec<mul_opcode_imm::InteractionClaim>,
    ret: Vec<ret_opcode::InteractionClaim>,
}
impl OpcodeInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.add_f_f.iter().for_each(|c| c.mix_into(channel));
        self.add_f_t.iter().for_each(|c| c.mix_into(channel));
        self.add_t_f.iter().for_each(|c| c.mix_into(channel));
        self.add_t_t.iter().for_each(|c| c.mix_into(channel));
        self.add_ap_f_f.iter().for_each(|c| c.mix_into(channel));
        self.add_ap_f_t.iter().for_each(|c| c.mix_into(channel));
        self.add_ap_t_f.iter().for_each(|c| c.mix_into(channel));
        self.assert_eq_f_f.iter().for_each(|c| c.mix_into(channel));
        self.assert_eq_f_t.iter().for_each(|c| c.mix_into(channel));
        self.assert_eq_t_f.iter().for_each(|c| c.mix_into(channel));
        self.call_f_f.iter().for_each(|c| c.mix_into(channel));
        self.call_f_t.iter().for_each(|c| c.mix_into(channel));
        self.call_t_f.iter().for_each(|c| c.mix_into(channel));
        self.generic.iter().for_each(|c| c.mix_into(channel));
        self.jnz_f_f.iter().for_each(|c| c.mix_into(channel));
        self.jnz_f_t.iter().for_each(|c| c.mix_into(channel));
        self.jnz_t_f.iter().for_each(|c| c.mix_into(channel));
        self.jnz_t_t.iter().for_each(|c| c.mix_into(channel));
        self.jump_f_f_f.iter().for_each(|c| c.mix_into(channel));
        self.jump_f_f_t.iter().for_each(|c| c.mix_into(channel));
        self.jump_t_f_f.iter().for_each(|c| c.mix_into(channel));
        self.jump_t_t_f.iter().for_each(|c| c.mix_into(channel));
        self.mul_f_f.iter().for_each(|c| c.mix_into(channel));
        self.mul_f_t.iter().for_each(|c| c.mix_into(channel));
        self.ret.iter().for_each(|c| c.mix_into(channel));
    }

    pub fn sum(&self) -> SecureField {
        let mut sum = QM31::zero();
        for interaction_claim in &self.add_f_f {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.add_f_t {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.add_t_f {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.add_t_t {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.add_ap_f_f {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.add_ap_f_t {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.add_ap_t_f {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.assert_eq_f_f {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.assert_eq_f_t {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.assert_eq_t_f {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.call_f_f {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.call_f_t {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.call_t_f {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.generic {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.jnz_f_f {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.jnz_f_t {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.jnz_t_f {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.jnz_t_t {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.jump_f_f_f {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.jump_f_f_t {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.jump_t_f_f {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.jump_t_t_f {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.mul_f_f {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.mul_f_t {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.ret {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        sum
    }
}

pub struct OpcodesInteractionClaimGenerator {
    add_f_f: Vec<add_opcode::InteractionClaimGenerator>,
    add_f_t: Vec<add_opcode_imm::InteractionClaimGenerator>,
    add_t_f: Vec<add_opcode_small::InteractionClaimGenerator>,
    add_t_t: Vec<add_opcode_small_imm::InteractionClaimGenerator>,
    add_ap_f_f: Vec<add_ap_opcode::InteractionClaimGenerator>,
    add_ap_f_t: Vec<add_ap_opcode_op_1_base_fp::InteractionClaimGenerator>,
    add_ap_t_f: Vec<add_ap_opcode_imm::InteractionClaimGenerator>,
    assert_eq_f_f: Vec<assert_eq_opcode::InteractionClaimGenerator>,
    assert_eq_f_t: Vec<assert_eq_opcode_imm::InteractionClaimGenerator>,
    assert_eq_t_f: Vec<assert_eq_opcode_double_deref::InteractionClaimGenerator>,
    call_f_f: Vec<call_opcode::InteractionClaimGenerator>,
    call_f_t: Vec<call_opcode_op_1_base_fp::InteractionClaimGenerator>,
    call_t_f: Vec<call_opcode_rel::InteractionClaimGenerator>,
    generic_opcode_interaction_gens: Vec<generic_opcode::InteractionClaimGenerator>,
    jnz_f_f: Vec<jnz_opcode::InteractionClaimGenerator>,
    jnz_f_t: Vec<jnz_opcode_dst_base_fp::InteractionClaimGenerator>,
    jnz_t_f: Vec<jnz_opcode_taken::InteractionClaimGenerator>,
    jnz_t_t: Vec<jnz_opcode_taken_dst_base_fp::InteractionClaimGenerator>,
    jump_f_f_f: Vec<jump_opcode::InteractionClaimGenerator>,
    jump_f_f_t: Vec<jump_opcode_double_deref::InteractionClaimGenerator>,
    jump_t_f_f: Vec<jump_opcode_rel::InteractionClaimGenerator>,
    jump_t_t_f: Vec<jump_opcode_rel_imm::InteractionClaimGenerator>,
    mul_f_f: Vec<mul_opcode::InteractionClaimGenerator>,
    mul_f_t: Vec<mul_opcode_imm::InteractionClaimGenerator>,
    ret_interaction_gens: Vec<ret_opcode::InteractionClaimGenerator>,
}
impl OpcodesInteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        interaction_elements: &CairoInteractionElements,
    ) -> OpcodeInteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let add_f_f_interaction_claims = self
            .add_f_f
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let add_f_t_interaction_claims = self
            .add_f_t
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let add_t_f_interaction_claims = self
            .add_t_f
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let add_t_t_interaction_claims = self
            .add_t_t
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let add_ap_f_f_interaction_claims = self
            .add_ap_f_f
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let add_ap_f_t_interaction_claims = self
            .add_ap_f_t
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let add_ap_t_f_interaction_claims = self
            .add_ap_t_f
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let assert_eq_f_f_interaction_claims = self
            .assert_eq_f_f
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let assert_eq_f_t_interaction_claims = self
            .assert_eq_f_t
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let assert_eq_t_f_interaction_claims = self
            .assert_eq_t_f
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let call_f_f_interaction_claims = self
            .call_f_f
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let call_f_t_interaction_claims = self
            .call_f_t
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let call_t_f_interaction_claims = self
            .call_t_f
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let generic_opcode_interaction_claims = self
            .generic_opcode_interaction_gens
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.range_check_19,
                    &interaction_elements.range_check_9_9,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let jnz_f_f_interaction_claims = self
            .jnz_f_f
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let jnz_f_t_interaction_claims = self
            .jnz_f_t
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let jnz_t_f_interaction_claims = self
            .jnz_t_f
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let jnz_t_t_interaction_claims = self
            .jnz_t_t
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let jump_f_f_f_interaction_claims = self
            .jump_f_f_f
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let jump_f_f_t_interaction_claims = self
            .jump_f_f_t
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let jump_t_f_f_interaction_claims = self
            .jump_t_f_f
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let jump_t_t_f_interaction_claims = self
            .jump_t_t_f
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let mul_f_f_interaction_claims = self
            .mul_f_f
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.range_check_19,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let mul_f_t_interaction_claims = self
            .mul_f_t
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.range_check_19,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let ret_interaction_claims = self
            .ret_interaction_gens
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        OpcodeInteractionClaim {
            add_f_f: add_f_f_interaction_claims,
            add_f_t: add_f_t_interaction_claims,
            add_t_f: add_t_f_interaction_claims,
            add_t_t: add_t_t_interaction_claims,
            add_ap_f_f: add_ap_f_f_interaction_claims,
            add_ap_f_t: add_ap_f_t_interaction_claims,
            add_ap_t_f: add_ap_t_f_interaction_claims,
            assert_eq_f_f: assert_eq_f_f_interaction_claims,
            assert_eq_f_t: assert_eq_f_t_interaction_claims,
            assert_eq_t_f: assert_eq_t_f_interaction_claims,
            call_f_f: call_f_f_interaction_claims,
            call_f_t: call_f_t_interaction_claims,
            call_t_f: call_t_f_interaction_claims,
            generic: generic_opcode_interaction_claims,
            jnz_f_f: jnz_f_f_interaction_claims,
            jnz_f_t: jnz_f_t_interaction_claims,
            jnz_t_f: jnz_t_f_interaction_claims,
            jnz_t_t: jnz_t_t_interaction_claims,
            jump_f_f_f: jump_f_f_f_interaction_claims,
            jump_f_f_t: jump_f_f_t_interaction_claims,
            jump_t_f_f: jump_t_f_f_interaction_claims,
            jump_t_t_f: jump_t_t_f_interaction_claims,
            mul_f_f: mul_f_f_interaction_claims,
            mul_f_t: mul_f_t_interaction_claims,
            ret: ret_interaction_claims,
        }
    }
}

pub struct OpcodeComponents {
    add_f_f: Vec<add_opcode::Component>,
    add_f_t: Vec<add_opcode_imm::Component>,
    add_t_f: Vec<add_opcode_small::Component>,
    add_t_t: Vec<add_opcode_small_imm::Component>,
    add_ap_f_f: Vec<add_ap_opcode::Component>,
    add_ap_f_t: Vec<add_ap_opcode_op_1_base_fp::Component>,
    add_ap_t_f: Vec<add_ap_opcode_imm::Component>,
    assert_eq_f_f: Vec<assert_eq_opcode::Component>,
    assert_eq_f_t: Vec<assert_eq_opcode_imm::Component>,
    assert_eq_t_f: Vec<assert_eq_opcode_double_deref::Component>,
    call_f_f: Vec<call_opcode::Component>,
    call_f_t: Vec<call_opcode_op_1_base_fp::Component>,
    call_t_f: Vec<call_opcode_rel::Component>,
    generic: Vec<generic_opcode::Component>,
    jnz_f_f: Vec<jnz_opcode::Component>,
    jnz_f_t: Vec<jnz_opcode_dst_base_fp::Component>,
    jnz_t_f: Vec<jnz_opcode_taken::Component>,
    jnz_t_t: Vec<jnz_opcode_taken_dst_base_fp::Component>,
    jump_f_f_f: Vec<jump_opcode::Component>,
    jump_f_f_t: Vec<jump_opcode_double_deref::Component>,
    jump_t_f_f: Vec<jump_opcode_rel::Component>,
    jump_t_t_f: Vec<jump_opcode_rel_imm::Component>,
    mul_f_f: Vec<mul_opcode::Component>,
    mul_f_t: Vec<mul_opcode_imm::Component>,
    ret: Vec<ret_opcode::Component>,
}
impl OpcodeComponents {
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        claim: &OpcodeClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &OpcodeInteractionClaim,
    ) -> Self {
        let add_f_f_components = claim
            .add_f_f
            .iter()
            .zip(interaction_claim.add_f_f.iter())
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
                    interaction_claim.logup_sums,
                )
            })
            .collect_vec();
        let add_f_t_components = claim
            .add_f_t
            .iter()
            .zip(interaction_claim.add_f_t.iter())
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
                    interaction_claim.logup_sums,
                )
            })
            .collect_vec();
        let add_t_f_components = claim
            .add_t_f
            .iter()
            .zip(interaction_claim.add_t_f.iter())
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
                    interaction_claim.logup_sums,
                )
            })
            .collect_vec();
        let add_t_t_components = claim
            .add_t_t
            .iter()
            .zip(interaction_claim.add_t_t.iter())
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
                    interaction_claim.logup_sums,
                )
            })
            .collect_vec();
        let add_ap_f_f_components = claim
            .add_ap_f_f
            .iter()
            .zip(interaction_claim.add_ap_f_f.iter())
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
                    interaction_claim.logup_sums,
                )
            })
            .collect_vec();
        let add_ap_f_t_components = claim
            .add_ap_f_t
            .iter()
            .zip(interaction_claim.add_ap_f_t.iter())
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
                    interaction_claim.logup_sums,
                )
            })
            .collect_vec();
        let add_ap_t_f_components = claim
            .add_ap_t_f
            .iter()
            .zip(interaction_claim.add_ap_t_f.iter())
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
                    interaction_claim.logup_sums,
                )
            })
            .collect_vec();
        let assert_eq_f_f_components = claim
            .assert_eq_f_f
            .iter()
            .zip(interaction_claim.assert_eq_f_f.iter())
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
                    interaction_claim.logup_sums,
                )
            })
            .collect_vec();
        let assert_eq_f_t_components = claim
            .assert_eq_f_t
            .iter()
            .zip(interaction_claim.assert_eq_f_t.iter())
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
                    interaction_claim.logup_sums,
                )
            })
            .collect_vec();
        let assert_eq_t_f_components = claim
            .assert_eq_t_f
            .iter()
            .zip(interaction_claim.assert_eq_t_f.iter())
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
                    interaction_claim.logup_sums,
                )
            })
            .collect_vec();
        let call_f_f_components = claim
            .call_f_f
            .iter()
            .zip(interaction_claim.call_f_f.iter())
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
                    interaction_claim.logup_sums,
                )
            })
            .collect();
        let call_f_t_components = claim
            .call_f_t
            .iter()
            .zip(interaction_claim.call_f_t.iter())
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
                    interaction_claim.logup_sums,
                )
            })
            .collect();
        let call_t_f_components = claim
            .call_t_f
            .iter()
            .zip(interaction_claim.call_t_f.iter())
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
                    interaction_claim.logup_sums,
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
                        range_check_19_lookup_elements: interaction_elements.range_check_19.clone(),
                        range_check_9_9_lookup_elements: interaction_elements
                            .range_check_9_9
                            .clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.logup_sums,
                )
            })
            .collect_vec();
        let jnz_f_f_components = claim
            .jnz_f_f
            .iter()
            .zip(interaction_claim.jnz_f_f.iter())
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
                    interaction_claim.logup_sums,
                )
            })
            .collect_vec();
        let jnz_f_t_components = claim
            .jnz_f_t
            .iter()
            .zip(interaction_claim.jnz_f_t.iter())
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
                    interaction_claim.logup_sums,
                )
            })
            .collect_vec();
        let jnz_t_f_components = claim
            .jnz_t_f
            .iter()
            .zip(interaction_claim.jnz_t_f.iter())
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
                    interaction_claim.logup_sums,
                )
            })
            .collect_vec();
        let jnz_t_t_components = claim
            .jnz_t_t
            .iter()
            .zip(interaction_claim.jnz_t_t.iter())
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
                    interaction_claim.logup_sums,
                )
            })
            .collect_vec();
        let jump_f_f_f_components = claim
            .jump_f_f_f
            .iter()
            .zip(interaction_claim.jump_f_f_f.iter())
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
                    interaction_claim.logup_sums,
                )
            })
            .collect();
        let jump_f_f_t_components = claim
            .jump_f_f_t
            .iter()
            .zip(interaction_claim.jump_f_f_t.iter())
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
                    interaction_claim.logup_sums,
                )
            })
            .collect();
        let jump_t_f_f_components = claim
            .jump_t_f_f
            .iter()
            .zip(interaction_claim.jump_t_f_f.iter())
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
                    interaction_claim.logup_sums,
                )
            })
            .collect();
        let jump_t_t_f_components = claim
            .jump_t_t_f
            .iter()
            .zip(interaction_claim.jump_t_t_f.iter())
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
                    interaction_claim.logup_sums,
                )
            })
            .collect();
        let mul_f_f_components = claim
            .mul_f_f
            .iter()
            .zip(interaction_claim.mul_f_f.iter())
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
                        range_check_19_lookup_elements: interaction_elements.range_check_19.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.logup_sums,
                )
            })
            .collect_vec();
        let mul_f_t_components = claim
            .mul_f_t
            .iter()
            .zip(interaction_claim.mul_f_t.iter())
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
                        range_check_19_lookup_elements: interaction_elements.range_check_19.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.logup_sums,
                )
            })
            .collect_vec();
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
                    interaction_claim.logup_sums,
                )
            })
            .collect_vec();
        Self {
            add_f_f: add_f_f_components,
            add_f_t: add_f_t_components,
            add_t_f: add_t_f_components,
            add_t_t: add_t_t_components,
            add_ap_f_f: add_ap_f_f_components,
            add_ap_f_t: add_ap_f_t_components,
            add_ap_t_f: add_ap_t_f_components,
            assert_eq_f_f: assert_eq_f_f_components,
            assert_eq_f_t: assert_eq_f_t_components,
            assert_eq_t_f: assert_eq_t_f_components,
            call_f_f: call_f_f_components,
            call_f_t: call_f_t_components,
            call_t_f: call_t_f_components,
            generic: generic_components,
            jnz_f_f: jnz_f_f_components,
            jnz_f_t: jnz_f_t_components,
            jnz_t_f: jnz_t_f_components,
            jnz_t_t: jnz_t_t_components,
            jump_f_f_f: jump_f_f_f_components,
            jump_f_f_t: jump_f_f_t_components,
            jump_t_f_f: jump_t_f_f_components,
            jump_t_t_f: jump_t_t_f_components,
            mul_f_f: mul_f_f_components,
            mul_f_t: mul_f_t_components,
            ret: ret_components,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        let mut vec: Vec<&dyn ComponentProver<SimdBackend>> = vec![];
        vec.extend(
            self.add_f_f
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.add_f_t
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.add_t_f
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.add_t_t
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.add_ap_f_f
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.add_ap_f_t
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.add_ap_t_f
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.assert_eq_f_f
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.assert_eq_f_t
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.assert_eq_t_f
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.call_f_f
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.call_f_t
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.call_t_f
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.generic
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.jnz_f_f
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.jnz_f_t
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.jnz_t_f
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.jnz_t_t
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.jump_f_f_f
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.jump_f_f_t
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.jump_t_f_f
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.jump_t_t_f
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.mul_f_f
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.mul_f_t
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
        writeln!(f, "add_f_f:")?;
        writeln!(f, "{}", display_components(&self.add_f_f))?;
        writeln!(f, "add_f_t:")?;
        writeln!(f, "{}", display_components(&self.add_f_t))?;
        writeln!(f, "add_t_f:")?;
        writeln!(f, "{}", display_components(&self.add_t_f))?;
        writeln!(f, "add_t_t:")?;
        writeln!(f, "{}", display_components(&self.add_t_t))?;
        writeln!(f, "add_ap_f_f:")?;
        writeln!(f, "{}", display_components(&self.add_ap_f_f))?;
        writeln!(f, "add_ap_f_t:")?;
        writeln!(f, "{}", display_components(&self.add_ap_f_t))?;
        writeln!(f, "add_ap_t_f:")?;
        writeln!(f, "{}", display_components(&self.add_ap_t_f))?;
        writeln!(f, "assert_eq_f_f:")?;
        writeln!(f, "{}", display_components(&self.assert_eq_f_f))?;
        writeln!(f, "assert_eq_f_t:")?;
        writeln!(f, "{}", display_components(&self.assert_eq_f_t))?;
        writeln!(f, "assert_eq_t_f:")?;
        writeln!(f, "{}", display_components(&self.assert_eq_t_f))?;
        writeln!(f, "call_f_f:")?;
        writeln!(f, "{}", display_components(&self.call_f_f))?;
        writeln!(f, "call_f_t:")?;
        writeln!(f, "{}", display_components(&self.call_f_t))?;
        writeln!(f, "call_t_f:")?;
        writeln!(f, "{}", display_components(&self.call_t_f))?;
        writeln!(f, "generic:")?;
        writeln!(f, "{}", display_components(&self.generic))?;
        writeln!(f, "jnz_f_f:")?;
        writeln!(f, "{}", display_components(&self.jnz_f_f))?;
        writeln!(f, "jnz_f_t:")?;
        writeln!(f, "{}", display_components(&self.jnz_f_t))?;
        writeln!(f, "jnz_t_f:")?;
        writeln!(f, "{}", display_components(&self.jnz_t_f))?;
        writeln!(f, "jnz_t_t:")?;
        writeln!(f, "{}", display_components(&self.jnz_t_t))?;
        writeln!(f, "jump_f_f_f:")?;
        writeln!(f, "{}", display_components(&self.jump_f_f_f))?;
        writeln!(f, "jump_f_f_t:")?;
        writeln!(f, "{}", display_components(&self.jump_f_f_t))?;
        writeln!(f, "jump_t_f_f:")?;
        writeln!(f, "{}", display_components(&self.jump_t_f_f))?;
        writeln!(f, "jump_t_t_f:")?;
        writeln!(f, "{}", display_components(&self.jump_t_t_f))?;
        writeln!(f, "mul_f_f:")?;
        writeln!(f, "{}", display_components(&self.mul_f_f))?;
        writeln!(f, "mul_f_t:")?;
        writeln!(f, "{}", display_components(&self.mul_f_t))?;
        writeln!(f, "ret:")?;
        writeln!(f, "{}", display_components(&self.ret))?;
        Ok(())
    }
}
