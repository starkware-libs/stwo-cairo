use itertools::{chain, Itertools};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo_cairo_adapter::opcodes::StateTransitions;
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
    memory_address_to_id, memory_id_to_big, mul_opcode, mul_opcode_imm, mul_opcode_small,
    mul_opcode_small_imm, qm_31_add_mul_opcode, range_check_11, range_check_19,
    range_check_4_4_4_4, range_check_9_9, ret_opcode, verify_instruction,
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

pub struct OpcodesClaimGenerator {
    add: Vec<add_opcode::ClaimGenerator>,
    add_imm: Vec<add_opcode_imm::ClaimGenerator>,
    add_small: Vec<add_opcode_small::ClaimGenerator>,
    add_small_imm: Vec<add_opcode_small_imm::ClaimGenerator>,
    add_ap: Vec<add_ap_opcode::ClaimGenerator>,
    add_ap_op_1_base_fp: Vec<add_ap_opcode_op_1_base_fp::ClaimGenerator>,
    add_ap_imm: Vec<add_ap_opcode_imm::ClaimGenerator>,
    assert_eq: Vec<assert_eq_opcode::ClaimGenerator>,
    assert_eq_imm: Vec<assert_eq_opcode_imm::ClaimGenerator>,
    assert_eq_double_deref: Vec<assert_eq_opcode_double_deref::ClaimGenerator>,
    call: Vec<call_opcode::ClaimGenerator>,
    call_op_1_base_fp: Vec<call_opcode_op_1_base_fp::ClaimGenerator>,
    call_rel: Vec<call_opcode_rel::ClaimGenerator>,
    generic: Vec<generic_opcode::ClaimGenerator>,
    jnz: Vec<jnz_opcode::ClaimGenerator>,
    jnz_dst_base_fp: Vec<jnz_opcode_dst_base_fp::ClaimGenerator>,
    jnz_taken: Vec<jnz_opcode_taken::ClaimGenerator>,
    jnz_taken_dst_base_fp: Vec<jnz_opcode_taken_dst_base_fp::ClaimGenerator>,
    jump: Vec<jump_opcode::ClaimGenerator>,
    jump_double_deref: Vec<jump_opcode_double_deref::ClaimGenerator>,
    jump_rel: Vec<jump_opcode_rel::ClaimGenerator>,
    jump_rel_imm: Vec<jump_opcode_rel_imm::ClaimGenerator>,
    mul: Vec<mul_opcode::ClaimGenerator>,
    mul_imm: Vec<mul_opcode_imm::ClaimGenerator>,
    mul_small: Vec<mul_opcode_small::ClaimGenerator>,
    mul_small_imm: Vec<mul_opcode_small_imm::ClaimGenerator>,
    qm31: Vec<qm_31_add_mul_opcode::ClaimGenerator>,
    ret: Vec<ret_opcode::ClaimGenerator>,
}
impl OpcodesClaimGenerator {
    pub fn new(input: StateTransitions) -> Self {
        // TODO(Ohad): decide split sizes for opcode traces.
        let mut add = vec![];
        let mut add_imm = vec![];
        let mut add_small = vec![];
        let mut add_small_imm = vec![];
        let mut add_ap = vec![];
        let mut add_ap_op_1_base_fp = vec![];
        let mut add_ap_imm = vec![];
        let mut assert_eq = vec![];
        let mut assert_eq_imm = vec![];
        let mut assert_eq_double_deref = vec![];
        let mut call = vec![];
        let mut call_op_1_base_fp = vec![];
        let mut call_rel = vec![];
        let mut generic = vec![];
        let mut jnz = vec![];
        let mut jnz_dst_base_fp = vec![];
        let mut jnz_taken = vec![];
        let mut jnz_taken_dst_base_fp = vec![];
        let mut jump = vec![];
        let mut jump_double_deref = vec![];
        let mut jump_rel = vec![];
        let mut jump_rel_imm = vec![];
        let mut mul = vec![];
        let mut mul_imm = vec![];
        let mut mul_small = vec![];
        let mut mul_small_imm = vec![];
        let mut qm31 = vec![];
        let mut ret = vec![];
        if !input.casm_states_by_opcode.add_opcode.is_empty() {
            add.push(add_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.add_opcode,
            ));
        }
        if !input.casm_states_by_opcode.add_opcode_imm.is_empty() {
            add_imm.push(add_opcode_imm::ClaimGenerator::new(
                input.casm_states_by_opcode.add_opcode_imm,
            ));
        }
        if !input.casm_states_by_opcode.add_opcode_small.is_empty() {
            add_small.push(add_opcode_small::ClaimGenerator::new(
                input.casm_states_by_opcode.add_opcode_small,
            ));
        }
        if !input.casm_states_by_opcode.add_opcode_small_imm.is_empty() {
            add_small_imm.push(add_opcode_small_imm::ClaimGenerator::new(
                input.casm_states_by_opcode.add_opcode_small_imm,
            ));
        }
        if !input.casm_states_by_opcode.add_ap_opcode.is_empty() {
            add_ap.push(add_ap_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.add_ap_opcode,
            ));
        }
        if !input
            .casm_states_by_opcode
            .add_ap_opcode_op_1_base_fp
            .is_empty()
        {
            add_ap_op_1_base_fp.push(add_ap_opcode_op_1_base_fp::ClaimGenerator::new(
                input.casm_states_by_opcode.add_ap_opcode_op_1_base_fp,
            ));
        }
        if !input.casm_states_by_opcode.add_ap_opcode_imm.is_empty() {
            add_ap_imm.push(add_ap_opcode_imm::ClaimGenerator::new(
                input.casm_states_by_opcode.add_ap_opcode_imm,
            ));
        }
        if !input.casm_states_by_opcode.assert_eq_opcode.is_empty() {
            assert_eq.push(assert_eq_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.assert_eq_opcode,
            ));
        }
        if !input.casm_states_by_opcode.assert_eq_opcode_imm.is_empty() {
            assert_eq_imm.push(assert_eq_opcode_imm::ClaimGenerator::new(
                input.casm_states_by_opcode.assert_eq_opcode_imm,
            ));
        }
        if !input
            .casm_states_by_opcode
            .assert_eq_opcode_double_deref
            .is_empty()
        {
            assert_eq_double_deref.push(assert_eq_opcode_double_deref::ClaimGenerator::new(
                input.casm_states_by_opcode.assert_eq_opcode_double_deref,
            ));
        }
        if !input.casm_states_by_opcode.call_opcode.is_empty() {
            call.push(call_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.call_opcode,
            ));
        }
        if !input
            .casm_states_by_opcode
            .call_opcode_op_1_base_fp
            .is_empty()
        {
            call_op_1_base_fp.push(call_opcode_op_1_base_fp::ClaimGenerator::new(
                input.casm_states_by_opcode.call_opcode_op_1_base_fp,
            ));
        }
        if !input.casm_states_by_opcode.call_opcode_rel.is_empty() {
            call_rel.push(call_opcode_rel::ClaimGenerator::new(
                input.casm_states_by_opcode.call_opcode_rel,
            ));
        }
        if !input.casm_states_by_opcode.generic_opcode.is_empty() {
            generic.push(generic_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.generic_opcode,
            ));
        }
        if !input.casm_states_by_opcode.jnz_opcode.is_empty() {
            jnz.push(jnz_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.jnz_opcode,
            ));
        }
        if !input
            .casm_states_by_opcode
            .jnz_opcode_dst_base_fp
            .is_empty()
        {
            jnz_dst_base_fp.push(jnz_opcode_dst_base_fp::ClaimGenerator::new(
                input.casm_states_by_opcode.jnz_opcode_dst_base_fp,
            ));
        }
        if !input.casm_states_by_opcode.jnz_opcode_taken.is_empty() {
            jnz_taken.push(jnz_opcode_taken::ClaimGenerator::new(
                input.casm_states_by_opcode.jnz_opcode_taken,
            ));
        }
        if !input
            .casm_states_by_opcode
            .jnz_opcode_taken_dst_base_fp
            .is_empty()
        {
            jnz_taken_dst_base_fp.push(jnz_opcode_taken_dst_base_fp::ClaimGenerator::new(
                input.casm_states_by_opcode.jnz_opcode_taken_dst_base_fp,
            ));
        }
        if !input.casm_states_by_opcode.jump_opcode.is_empty() {
            jump.push(jump_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.jump_opcode,
            ));
        }
        if !input
            .casm_states_by_opcode
            .jump_opcode_double_deref
            .is_empty()
        {
            jump_double_deref.push(jump_opcode_double_deref::ClaimGenerator::new(
                input.casm_states_by_opcode.jump_opcode_double_deref,
            ));
        }
        if !input.casm_states_by_opcode.jump_opcode_rel.is_empty() {
            jump_rel.push(jump_opcode_rel::ClaimGenerator::new(
                input.casm_states_by_opcode.jump_opcode_rel,
            ));
        }
        if !input.casm_states_by_opcode.jump_opcode_rel_imm.is_empty() {
            jump_rel_imm.push(jump_opcode_rel_imm::ClaimGenerator::new(
                input.casm_states_by_opcode.jump_opcode_rel_imm,
            ));
        }
        // Handle small mul in big mul component. Temporary until airs are written with Rc_3_6_6.
        if !input.casm_states_by_opcode.mul_opcode.is_empty() {
            mul.push(mul_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.mul_opcode,
            ));
        }
        if !input.casm_states_by_opcode.mul_opcode_imm.is_empty() {
            mul_imm.push(mul_opcode_imm::ClaimGenerator::new(
                input.casm_states_by_opcode.mul_opcode_imm,
            ));
        }
        if !input.casm_states_by_opcode.mul_opcode_small.is_empty() {
            mul_small.push(mul_opcode_small::ClaimGenerator::new(
                input.casm_states_by_opcode.mul_opcode_small,
            ));
        }
        if !input.casm_states_by_opcode.mul_opcode_small_imm.is_empty() {
            mul_small_imm.push(mul_opcode_small_imm::ClaimGenerator::new(
                input.casm_states_by_opcode.mul_opcode_small_imm,
            ));
        }
        if !input.casm_states_by_opcode.qm31_add_mul_opcode.is_empty() {
            qm31.push(qm_31_add_mul_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.qm31_add_mul_opcode,
            ));
        }
        if !input.casm_states_by_opcode.ret_opcode.is_empty() {
            ret.push(ret_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.ret_opcode,
            ));
        }
        Self {
            add,
            add_imm,
            add_small,
            add_small_imm,
            add_ap,
            add_ap_op_1_base_fp,
            add_ap_imm,
            assert_eq,
            assert_eq_imm,
            assert_eq_double_deref,
            call,
            call_op_1_base_fp,
            call_rel,
            generic,
            jnz,
            jnz_dst_base_fp,
            jnz_taken,
            jnz_taken_dst_base_fp,
            jump,
            jump_double_deref,
            jump_rel,
            jump_rel_imm,
            mul,
            mul_imm,
            mul_small,
            mul_small_imm,
            qm31,
            ret,
        }
    }

    pub fn write_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        memory_address_to_id_trace_generator: &memory_address_to_id::ClaimGenerator,
        memory_id_to_value_trace_generator: &memory_id_to_big::ClaimGenerator,
        range_check_11_trace_generator: &range_check_11::ClaimGenerator,
        range_check_19_trace_generator: &range_check_19::ClaimGenerator,
        range_check_9_9_trace_generator: &range_check_9_9::ClaimGenerator,
        range_check_4_4_4_4_trace_generator: &range_check_4_4_4_4::ClaimGenerator,
        verify_instruction_trace_generator: &verify_instruction::ClaimGenerator,
    ) -> (OpcodeClaim, OpcodesInteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let (add_claims, add_interaction_gens) = self
            .add
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
        let (add_imm_claims, add_imm_interaction_gens) = self
            .add_imm
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
        let (add_small_claims, add_small_interaction_gens) = self
            .add_small
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
        let (add_small_imm_claims, add_small_imm_interaction_gens) = self
            .add_small_imm
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
        let (add_ap_claims, add_ap_interaction_gens) = self
            .add_ap
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
        let (add_ap_op_1_base_fp_claims, add_ap_op_1_base_fp_interaction_gens) = self
            .add_ap_op_1_base_fp
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
        let (add_ap_imm_claims, add_ap_imm_interaction_gens) = self
            .add_ap_imm
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
        let (assert_eq_claims, assert_eq_interaction_gens) = self
            .assert_eq
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
        let (assert_eq_imm_claims, assert_eq_imm_interaction_gens) = self
            .assert_eq_imm
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
        let (assert_eq_double_deref_claims, assert_eq_double_deref_interaction_gens) = self
            .assert_eq_double_deref
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
        let (call_claims, call_interaction_gens) = self
            .call
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
        let (call_op_1_base_fp_claims, call_op_1_base_fp_interaction_gens) = self
            .call_op_1_base_fp
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
        let (call_rel_claims, call_rel_interaction_gens) = self
            .call_rel
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
        let (jnz_claims, jnz_interaction_gens) = self
            .jnz
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
        let (jnz_dst_base_fp_claims, jnz_dst_base_fp_interaction_gens) = self
            .jnz_dst_base_fp
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
        let (jnz_taken_claims, jnz_taken_interaction_gens) = self
            .jnz_taken
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
        let (jnz_taken_dst_base_fp_claims, jnz_taken_dst_base_fp_interaction_gens) = self
            .jnz_taken_dst_base_fp
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
        let (jump_claims, jump_interaction_gens) = self
            .jump
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
        let (jump_double_deref_claims, jump_double_deref_interaction_gens) = self
            .jump_double_deref
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
        let (jump_rel_claims, jump_rel_interaction_gens) = self
            .jump_rel
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
        let (jump_rel_imm_claims, jump_rel_imm_interaction_gens) = self
            .jump_rel_imm
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
        let (mul_claims, mul_interaction_gens) = self
            .mul
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
        let (mul_imm_claims, mul_imm_interaction_gens) = self
            .mul_imm
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
        let (mul_small_claims, mul_small_interaction_gens) = self
            .mul_small
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    range_check_11_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (mul_small_imm_claims, mul_small_imm_interaction_gens) = self
            .mul_small_imm
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    range_check_11_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (qm31_claims, qm31_interaction_gens) = self
            .qm31
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    range_check_4_4_4_4_trace_generator,
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
                add: add_claims,
                add_imm: add_imm_claims,
                add_small: add_small_claims,
                add_small_imm: add_small_imm_claims,
                add_ap: add_ap_claims,
                add_ap_op_1_base_fp: add_ap_op_1_base_fp_claims,
                add_ap_imm: add_ap_imm_claims,
                assert_eq: assert_eq_claims,
                assert_eq_imm: assert_eq_imm_claims,
                assert_eq_double_deref: assert_eq_double_deref_claims,
                call: call_claims,
                call_op_1_base_fp: call_op_1_base_fp_claims,
                call_rel: call_rel_claims,
                generic: generic_opcode_claims,
                jnz: jnz_claims,
                jnz_dst_base_fp: jnz_dst_base_fp_claims,
                jnz_taken: jnz_taken_claims,
                jnz_taken_dst_base_fp: jnz_taken_dst_base_fp_claims,
                jump: jump_claims,
                jump_double_deref: jump_double_deref_claims,
                jump_rel: jump_rel_claims,
                jump_rel_imm: jump_rel_imm_claims,
                mul: mul_claims,
                mul_imm: mul_imm_claims,
                mul_small: mul_small_claims,
                mul_small_imm: mul_small_imm_claims,
                qm31: qm31_claims,
                ret: ret_claims,
            },
            OpcodesInteractionClaimGenerator {
                add: add_interaction_gens,
                add_imm: add_imm_interaction_gens,
                add_small: add_small_interaction_gens,
                add_small_imm: add_small_imm_interaction_gens,
                add_ap: add_ap_interaction_gens,
                add_ap_op_1_base_fp: add_ap_op_1_base_fp_interaction_gens,
                add_ap_imm: add_ap_imm_interaction_gens,
                assert_eq: assert_eq_interaction_gens,
                assert_eq_imm: assert_eq_imm_interaction_gens,
                assert_eq_double_deref: assert_eq_double_deref_interaction_gens,
                call: call_interaction_gens,
                call_op_1_base_fp: call_op_1_base_fp_interaction_gens,
                call_rel: call_rel_interaction_gens,
                generic_opcode_interaction_gens,
                jnz: jnz_interaction_gens,
                jnz_dst_base_fp: jnz_dst_base_fp_interaction_gens,
                jnz_taken: jnz_taken_interaction_gens,
                jnz_taken_dst_base_fp: jnz_taken_dst_base_fp_interaction_gens,
                jump: jump_interaction_gens,
                jump_double_deref: jump_double_deref_interaction_gens,
                jump_rel: jump_rel_interaction_gens,
                jump_rel_imm: jump_rel_imm_interaction_gens,
                mul: mul_interaction_gens,
                mul_imm: mul_imm_interaction_gens,
                mul_small: mul_small_interaction_gens,
                mul_small_imm: mul_small_imm_interaction_gens,
                qm31: qm31_interaction_gens,
                ret_interaction_gens,
            },
        )
    }
}

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct OpcodeInteractionClaim {
    add: Vec<add_opcode::InteractionClaim>,
    add_imm: Vec<add_opcode_imm::InteractionClaim>,
    add_small: Vec<add_opcode_small::InteractionClaim>,
    add_small_imm: Vec<add_opcode_small_imm::InteractionClaim>,
    add_ap: Vec<add_ap_opcode::InteractionClaim>,
    add_ap_op_1_base_fp: Vec<add_ap_opcode_op_1_base_fp::InteractionClaim>,
    add_ap_imm: Vec<add_ap_opcode_imm::InteractionClaim>,
    assert_eq: Vec<assert_eq_opcode::InteractionClaim>,
    assert_eq_imm: Vec<assert_eq_opcode_imm::InteractionClaim>,
    assert_eq_double_deref: Vec<assert_eq_opcode_double_deref::InteractionClaim>,
    call: Vec<call_opcode::InteractionClaim>,
    call_op_1_base_fp: Vec<call_opcode_op_1_base_fp::InteractionClaim>,
    call_rel: Vec<call_opcode_rel::InteractionClaim>,
    generic: Vec<generic_opcode::InteractionClaim>,
    jnz: Vec<jnz_opcode::InteractionClaim>,
    jnz_dst_base_fp: Vec<jnz_opcode_dst_base_fp::InteractionClaim>,
    jnz_taken: Vec<jnz_opcode_taken::InteractionClaim>,
    jnz_taken_dst_base_fp: Vec<jnz_opcode_taken_dst_base_fp::InteractionClaim>,
    jump: Vec<jump_opcode::InteractionClaim>,
    jump_double_deref: Vec<jump_opcode_double_deref::InteractionClaim>,
    jump_rel: Vec<jump_opcode_rel::InteractionClaim>,
    jump_rel_imm: Vec<jump_opcode_rel_imm::InteractionClaim>,
    mul: Vec<mul_opcode::InteractionClaim>,
    mul_imm: Vec<mul_opcode_imm::InteractionClaim>,
    mul_small: Vec<mul_opcode_small::InteractionClaim>,
    mul_small_imm: Vec<mul_opcode_small_imm::InteractionClaim>,
    qm31: Vec<qm_31_add_mul_opcode::InteractionClaim>,
    ret: Vec<ret_opcode::InteractionClaim>,
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

pub struct OpcodesInteractionClaimGenerator {
    add: Vec<add_opcode::InteractionClaimGenerator>,
    add_imm: Vec<add_opcode_imm::InteractionClaimGenerator>,
    add_small: Vec<add_opcode_small::InteractionClaimGenerator>,
    add_small_imm: Vec<add_opcode_small_imm::InteractionClaimGenerator>,
    add_ap: Vec<add_ap_opcode::InteractionClaimGenerator>,
    add_ap_op_1_base_fp: Vec<add_ap_opcode_op_1_base_fp::InteractionClaimGenerator>,
    add_ap_imm: Vec<add_ap_opcode_imm::InteractionClaimGenerator>,
    assert_eq: Vec<assert_eq_opcode::InteractionClaimGenerator>,
    assert_eq_imm: Vec<assert_eq_opcode_imm::InteractionClaimGenerator>,
    assert_eq_double_deref: Vec<assert_eq_opcode_double_deref::InteractionClaimGenerator>,
    call: Vec<call_opcode::InteractionClaimGenerator>,
    call_op_1_base_fp: Vec<call_opcode_op_1_base_fp::InteractionClaimGenerator>,
    call_rel: Vec<call_opcode_rel::InteractionClaimGenerator>,
    generic_opcode_interaction_gens: Vec<generic_opcode::InteractionClaimGenerator>,
    jnz: Vec<jnz_opcode::InteractionClaimGenerator>,
    jnz_dst_base_fp: Vec<jnz_opcode_dst_base_fp::InteractionClaimGenerator>,
    jnz_taken: Vec<jnz_opcode_taken::InteractionClaimGenerator>,
    jnz_taken_dst_base_fp: Vec<jnz_opcode_taken_dst_base_fp::InteractionClaimGenerator>,
    jump: Vec<jump_opcode::InteractionClaimGenerator>,
    jump_double_deref: Vec<jump_opcode_double_deref::InteractionClaimGenerator>,
    jump_rel: Vec<jump_opcode_rel::InteractionClaimGenerator>,
    jump_rel_imm: Vec<jump_opcode_rel_imm::InteractionClaimGenerator>,
    mul: Vec<mul_opcode::InteractionClaimGenerator>,
    mul_imm: Vec<mul_opcode_imm::InteractionClaimGenerator>,
    mul_small: Vec<mul_opcode_small::InteractionClaimGenerator>,
    mul_small_imm: Vec<mul_opcode_small_imm::InteractionClaimGenerator>,
    qm31: Vec<qm_31_add_mul_opcode::InteractionClaimGenerator>,
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
        let add_interaction_claims = self
            .add
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
        let add_imm_interaction_claims = self
            .add_imm
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
        let add_small_interaction_claims = self
            .add_small
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
        let add_small_imm_interaction_claims = self
            .add_small_imm
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
        let add_ap_interaction_claims = self
            .add_ap
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
        let add_ap_op_1_base_fp_interaction_claims = self
            .add_ap_op_1_base_fp
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
        let add_ap_imm_interaction_claims = self
            .add_ap_imm
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
        let assert_eq_interaction_claims = self
            .assert_eq
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
        let assert_eq_imm_interaction_claims = self
            .assert_eq_imm
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
        let assert_eq_double_deref_interaction_claims = self
            .assert_eq_double_deref
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
        let call_interaction_claims = self
            .call
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
        let call_op_1_base_fp_interaction_claims = self
            .call_op_1_base_fp
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
        let call_rel_interaction_claims = self
            .call_rel
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
                    &interaction_elements.range_checks.rc_19,
                    &interaction_elements.range_checks.rc_9_9,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let jnz_interaction_claims = self
            .jnz
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
        let jnz_dst_base_fp_interaction_claims = self
            .jnz_dst_base_fp
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
        let jnz_taken_interaction_claims = self
            .jnz_taken
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
        let jnz_taken_dst_base_fp_interaction_claims = self
            .jnz_taken_dst_base_fp
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
        let jump_interaction_claims = self
            .jump
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
        let jump_double_deref_interaction_claims = self
            .jump_double_deref
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
        let jump_rel_interaction_claims = self
            .jump_rel
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
        let jump_rel_imm_interaction_claims = self
            .jump_rel_imm
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
        let mul_interaction_claims = self
            .mul
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.range_checks.rc_19,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let mul_imm_interaction_claims = self
            .mul_imm
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.range_checks.rc_19,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let mul_small_interaction_claims = self
            .mul_small
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.range_checks.rc_11,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let mul_small_imm_interaction_claims = self
            .mul_small_imm
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.range_checks.rc_11,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let qm31_interaction_claims = self
            .qm31
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.range_checks.rc_4_4_4_4,
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
            add: add_interaction_claims,
            add_imm: add_imm_interaction_claims,
            add_small: add_small_interaction_claims,
            add_small_imm: add_small_imm_interaction_claims,
            add_ap: add_ap_interaction_claims,
            add_ap_op_1_base_fp: add_ap_op_1_base_fp_interaction_claims,
            add_ap_imm: add_ap_imm_interaction_claims,
            assert_eq: assert_eq_interaction_claims,
            assert_eq_imm: assert_eq_imm_interaction_claims,
            assert_eq_double_deref: assert_eq_double_deref_interaction_claims,
            call: call_interaction_claims,
            call_op_1_base_fp: call_op_1_base_fp_interaction_claims,
            call_rel: call_rel_interaction_claims,
            generic: generic_opcode_interaction_claims,
            jnz: jnz_interaction_claims,
            jnz_dst_base_fp: jnz_dst_base_fp_interaction_claims,
            jnz_taken: jnz_taken_interaction_claims,
            jnz_taken_dst_base_fp: jnz_taken_dst_base_fp_interaction_claims,
            jump: jump_interaction_claims,
            jump_double_deref: jump_double_deref_interaction_claims,
            jump_rel: jump_rel_interaction_claims,
            jump_rel_imm: jump_rel_imm_interaction_claims,
            mul: mul_interaction_claims,
            mul_imm: mul_imm_interaction_claims,
            mul_small: mul_small_interaction_claims,
            mul_small_imm: mul_small_imm_interaction_claims,
            qm31: qm31_interaction_claims,
            ret: ret_interaction_claims,
        }
    }
}

pub struct OpcodeComponents {
    add: Vec<add_opcode::Component>,
    add_imm: Vec<add_opcode_imm::Component>,
    add_small: Vec<add_opcode_small::Component>,
    add_small_imm: Vec<add_opcode_small_imm::Component>,
    add_ap: Vec<add_ap_opcode::Component>,
    add_ap_op_1_base_fp: Vec<add_ap_opcode_op_1_base_fp::Component>,
    add_ap_imm: Vec<add_ap_opcode_imm::Component>,
    assert_eq: Vec<assert_eq_opcode::Component>,
    assert_eq_imm: Vec<assert_eq_opcode_imm::Component>,
    assert_eq_double_deref: Vec<assert_eq_opcode_double_deref::Component>,
    call: Vec<call_opcode::Component>,
    call_op_1_base_fp: Vec<call_opcode_op_1_base_fp::Component>,
    call_rel: Vec<call_opcode_rel::Component>,
    generic: Vec<generic_opcode::Component>,
    jnz: Vec<jnz_opcode::Component>,
    jnz_dst_base_fp: Vec<jnz_opcode_dst_base_fp::Component>,
    jnz_taken: Vec<jnz_opcode_taken::Component>,
    jnz_taken_dst_base_fp: Vec<jnz_opcode_taken_dst_base_fp::Component>,
    jump: Vec<jump_opcode::Component>,
    jump_double_deref: Vec<jump_opcode_double_deref::Component>,
    jump_rel: Vec<jump_opcode_rel::Component>,
    jump_rel_imm: Vec<jump_opcode_rel_imm::Component>,
    mul: Vec<mul_opcode::Component>,
    mul_imm: Vec<mul_opcode_imm::Component>,
    mul_small: Vec<mul_opcode_small::Component>,
    mul_small_imm: Vec<mul_opcode_small_imm::Component>,
    qm31: Vec<qm_31_add_mul_opcode::Component>,
    ret: Vec<ret_opcode::Component>,
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
