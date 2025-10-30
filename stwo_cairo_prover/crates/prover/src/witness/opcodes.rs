use cairo_air::air::CairoInteractionElements;
use cairo_air::opcodes_air::{OpcodeClaim, OpcodeInteractionClaim};
use stwo::prover::backend::simd::SimdBackend;
use stwo_cairo_adapter::opcodes::StateTransitions;

use super::blake_context::BlakeContextClaimGenerator;
use super::range_checks::RangeChecksClaimGenerator;
use crate::witness::components::{
    add_ap_opcode, add_opcode, add_opcode_small, assert_eq_opcode, assert_eq_opcode_double_deref,
    assert_eq_opcode_imm, blake_compress_opcode, call_opcode_abs, call_opcode_rel_imm,
    generic_opcode, jnz_opcode_non_taken, jnz_opcode_taken, jump_opcode_abs,
    jump_opcode_double_deref, jump_opcode_rel, jump_opcode_rel_imm, memory_address_to_id,
    memory_id_to_big, mul_opcode, mul_opcode_small, qm_31_add_mul_opcode, ret_opcode,
    verify_bitwise_xor_8, verify_instruction,
};
use crate::witness::utils::TreeBuilder;

pub struct OpcodesClaimGenerator {
    add: Vec<add_opcode::ClaimGenerator>,
    add_small: Vec<add_opcode_small::ClaimGenerator>,
    add_ap: Vec<add_ap_opcode::ClaimGenerator>,
    assert_eq: Vec<assert_eq_opcode::ClaimGenerator>,
    assert_eq_imm: Vec<assert_eq_opcode_imm::ClaimGenerator>,
    assert_eq_double_deref: Vec<assert_eq_opcode_double_deref::ClaimGenerator>,
    blake: Vec<blake_compress_opcode::ClaimGenerator>,
    call: Vec<call_opcode_abs::ClaimGenerator>,
    call_rel_imm: Vec<call_opcode_rel_imm::ClaimGenerator>,
    generic: Vec<generic_opcode::ClaimGenerator>,
    jnz: Vec<jnz_opcode_non_taken::ClaimGenerator>,
    jnz_taken: Vec<jnz_opcode_taken::ClaimGenerator>,
    jump: Vec<jump_opcode_abs::ClaimGenerator>,
    jump_double_deref: Vec<jump_opcode_double_deref::ClaimGenerator>,
    jump_rel: Vec<jump_opcode_rel::ClaimGenerator>,
    jump_rel_imm: Vec<jump_opcode_rel_imm::ClaimGenerator>,
    mul: Vec<mul_opcode::ClaimGenerator>,
    mul_small: Vec<mul_opcode_small::ClaimGenerator>,
    qm31: Vec<qm_31_add_mul_opcode::ClaimGenerator>,
    ret: Vec<ret_opcode::ClaimGenerator>,
}
impl OpcodesClaimGenerator {
    pub fn new(input: StateTransitions) -> Self {
        // TODO(Ohad): decide split sizes for opcode traces.
        let mut add = vec![];
        let mut add_small = vec![];
        let mut add_ap = vec![];
        let mut assert_eq = vec![];
        let mut assert_eq_imm = vec![];
        let mut assert_eq_double_deref = vec![];
        let mut blake = vec![];
        let mut call = vec![];
        let mut call_rel_imm = vec![];
        let mut generic = vec![];
        let mut jnz = vec![];
        let mut jnz_taken = vec![];
        let mut jump = vec![];
        let mut jump_double_deref = vec![];
        let mut jump_rel = vec![];
        let mut jump_rel_imm = vec![];
        let mut mul = vec![];
        let mut mul_small = vec![];
        let mut qm31 = vec![];
        let mut ret = vec![];
        if !input.casm_states_by_opcode.add_opcode.is_empty() {
            add.push(add_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.add_opcode,
            ));
        }
        if !input.casm_states_by_opcode.add_opcode_small.is_empty() {
            add_small.push(add_opcode_small::ClaimGenerator::new(
                input.casm_states_by_opcode.add_opcode_small,
            ));
        }
        if !input.casm_states_by_opcode.add_ap_opcode.is_empty() {
            add_ap.push(add_ap_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.add_ap_opcode,
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
        if !input.casm_states_by_opcode.blake_compress_opcode.is_empty() {
            blake.push(blake_compress_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.blake_compress_opcode,
            ));
        }
        if !input.casm_states_by_opcode.call_opcode_abs.is_empty() {
            call.push(call_opcode_abs::ClaimGenerator::new(
                input.casm_states_by_opcode.call_opcode_abs,
            ));
        }
        if !input.casm_states_by_opcode.call_opcode_rel_imm.is_empty() {
            call_rel_imm.push(call_opcode_rel_imm::ClaimGenerator::new(
                input.casm_states_by_opcode.call_opcode_rel_imm,
            ));
        }
        if !input.casm_states_by_opcode.generic_opcode.is_empty() {
            generic.push(generic_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.generic_opcode,
            ));
        }
        if !input.casm_states_by_opcode.jnz_opcode_non_taken.is_empty() {
            jnz.push(jnz_opcode_non_taken::ClaimGenerator::new(
                input.casm_states_by_opcode.jnz_opcode_non_taken,
            ));
        }
        if !input.casm_states_by_opcode.jnz_opcode_taken.is_empty() {
            jnz_taken.push(jnz_opcode_taken::ClaimGenerator::new(
                input.casm_states_by_opcode.jnz_opcode_taken,
            ));
        }
        if !input.casm_states_by_opcode.jump_opcode_abs.is_empty() {
            jump.push(jump_opcode_abs::ClaimGenerator::new(
                input.casm_states_by_opcode.jump_opcode_abs,
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
        if !input.casm_states_by_opcode.mul_opcode_small.is_empty() {
            mul_small.push(mul_opcode_small::ClaimGenerator::new(
                input.casm_states_by_opcode.mul_opcode_small,
            ));
        }
        if !input.casm_states_by_opcode.qm_31_add_mul_opcode.is_empty() {
            qm31.push(qm_31_add_mul_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.qm_31_add_mul_opcode,
            ));
        }
        if !input.casm_states_by_opcode.ret_opcode.is_empty() {
            ret.push(ret_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.ret_opcode,
            ));
        }
        Self {
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
        }
    }

    pub fn write_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        blake_context_trace_generator: &mut BlakeContextClaimGenerator,
        memory_address_to_id_trace_generator: &memory_address_to_id::ClaimGenerator,
        memory_id_to_value_trace_generator: &memory_id_to_big::ClaimGenerator,
        range_checks_trace_generator: &RangeChecksClaimGenerator,
        verify_instruction_trace_generator: &verify_instruction::ClaimGenerator,
        verify_bitwise_xor_8_trace_generator: &mut verify_bitwise_xor_8::ClaimGenerator,
    ) -> (OpcodeClaim, OpcodesInteractionClaimGenerator) {
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
        let (add_ap_claims, add_ap_interaction_gens) = self
            .add_ap
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                    &range_checks_trace_generator.rc_18_trace_generator,
                    &range_checks_trace_generator.rc_11_trace_generator,
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
        let (blake_claims, blake_interaction_gens) = self
            .blake
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                    &range_checks_trace_generator.rc_7_2_5_trace_generator,
                    verify_bitwise_xor_8_trace_generator,
                    &mut blake_context_trace_generator.blake_round,
                    &mut blake_context_trace_generator.triple_xor_32,
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
        let (call_rel_imm_claims, call_rel_imm_interaction_gens) = self
            .call_rel_imm
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
                    verify_instruction_trace_generator,
                    &range_checks_trace_generator.rc_9_9_trace_generator,
                    &range_checks_trace_generator.rc_9_9_b_trace_generator,
                    &range_checks_trace_generator.rc_9_9_c_trace_generator,
                    &range_checks_trace_generator.rc_9_9_d_trace_generator,
                    &range_checks_trace_generator.rc_9_9_e_trace_generator,
                    &range_checks_trace_generator.rc_9_9_f_trace_generator,
                    &range_checks_trace_generator.rc_9_9_g_trace_generator,
                    &range_checks_trace_generator.rc_9_9_h_trace_generator,
                    &range_checks_trace_generator.rc_20_trace_generator,
                    &range_checks_trace_generator.rc_20_b_trace_generator,
                    &range_checks_trace_generator.rc_20_c_trace_generator,
                    &range_checks_trace_generator.rc_20_d_trace_generator,
                    &range_checks_trace_generator.rc_20_e_trace_generator,
                    &range_checks_trace_generator.rc_20_f_trace_generator,
                    &range_checks_trace_generator.rc_20_g_trace_generator,
                    &range_checks_trace_generator.rc_20_h_trace_generator,
                    &range_checks_trace_generator.rc_18_trace_generator,
                    &range_checks_trace_generator.rc_11_trace_generator,
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
                    verify_instruction_trace_generator,
                    &range_checks_trace_generator.rc_20_trace_generator,
                    &range_checks_trace_generator.rc_20_b_trace_generator,
                    &range_checks_trace_generator.rc_20_c_trace_generator,
                    &range_checks_trace_generator.rc_20_d_trace_generator,
                    &range_checks_trace_generator.rc_20_e_trace_generator,
                    &range_checks_trace_generator.rc_20_f_trace_generator,
                    &range_checks_trace_generator.rc_20_g_trace_generator,
                    &range_checks_trace_generator.rc_20_h_trace_generator,
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
                    verify_instruction_trace_generator,
                    &range_checks_trace_generator.rc_11_trace_generator,
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
                    verify_instruction_trace_generator,
                    &range_checks_trace_generator.rc_4_4_4_4_trace_generator,
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
                add_small: add_small_claims,
                add_ap: add_ap_claims,
                assert_eq: assert_eq_claims,
                assert_eq_imm: assert_eq_imm_claims,
                assert_eq_double_deref: assert_eq_double_deref_claims,
                blake: blake_claims,
                call: call_claims,
                call_rel_imm: call_rel_imm_claims,
                generic: generic_opcode_claims,
                jnz: jnz_claims,
                jnz_taken: jnz_taken_claims,
                jump: jump_claims,
                jump_double_deref: jump_double_deref_claims,
                jump_rel: jump_rel_claims,
                jump_rel_imm: jump_rel_imm_claims,
                mul: mul_claims,
                mul_small: mul_small_claims,
                qm31: qm31_claims,
                ret: ret_claims,
            },
            OpcodesInteractionClaimGenerator {
                add: add_interaction_gens,
                add_small: add_small_interaction_gens,
                add_ap: add_ap_interaction_gens,
                assert_eq: assert_eq_interaction_gens,
                assert_eq_imm: assert_eq_imm_interaction_gens,
                assert_eq_double_deref: assert_eq_double_deref_interaction_gens,
                blake: blake_interaction_gens,
                call: call_interaction_gens,
                call_rel_imm: call_rel_imm_interaction_gens,
                generic_opcode_interaction_gens,
                jnz: jnz_interaction_gens,
                jnz_taken: jnz_taken_interaction_gens,
                jump: jump_interaction_gens,
                jump_double_deref: jump_double_deref_interaction_gens,
                jump_rel: jump_rel_interaction_gens,
                jump_rel_imm: jump_rel_imm_interaction_gens,
                mul: mul_interaction_gens,
                mul_small: mul_small_interaction_gens,
                qm31: qm31_interaction_gens,
                ret_interaction_gens,
            },
        )
    }
}

pub struct OpcodesInteractionClaimGenerator {
    add: Vec<add_opcode::InteractionClaimGenerator>,
    add_small: Vec<add_opcode_small::InteractionClaimGenerator>,
    add_ap: Vec<add_ap_opcode::InteractionClaimGenerator>,
    assert_eq: Vec<assert_eq_opcode::InteractionClaimGenerator>,
    assert_eq_imm: Vec<assert_eq_opcode_imm::InteractionClaimGenerator>,
    assert_eq_double_deref: Vec<assert_eq_opcode_double_deref::InteractionClaimGenerator>,
    blake: Vec<blake_compress_opcode::InteractionClaimGenerator>,
    call: Vec<call_opcode_abs::InteractionClaimGenerator>,
    call_rel_imm: Vec<call_opcode_rel_imm::InteractionClaimGenerator>,
    generic_opcode_interaction_gens: Vec<generic_opcode::InteractionClaimGenerator>,
    jnz: Vec<jnz_opcode_non_taken::InteractionClaimGenerator>,
    jnz_taken: Vec<jnz_opcode_taken::InteractionClaimGenerator>,
    jump: Vec<jump_opcode_abs::InteractionClaimGenerator>,
    jump_double_deref: Vec<jump_opcode_double_deref::InteractionClaimGenerator>,
    jump_rel: Vec<jump_opcode_rel::InteractionClaimGenerator>,
    jump_rel_imm: Vec<jump_opcode_rel_imm::InteractionClaimGenerator>,
    mul: Vec<mul_opcode::InteractionClaimGenerator>,
    mul_small: Vec<mul_opcode_small::InteractionClaimGenerator>,
    qm31: Vec<qm_31_add_mul_opcode::InteractionClaimGenerator>,
    ret_interaction_gens: Vec<ret_opcode::InteractionClaimGenerator>,
}
impl OpcodesInteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        interaction_elements: &CairoInteractionElements,
    ) -> OpcodeInteractionClaim {
        let add_interaction_claims = self
            .add
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.verify_instruction,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                )
            })
            .collect();
        let add_small_interaction_claims = self
            .add_small
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.verify_instruction,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                )
            })
            .collect();
        let add_ap_interaction_claims = self
            .add_ap
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.verify_instruction,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.range_checks.rc_18,
                    &interaction_elements.range_checks.rc_11,
                    &interaction_elements.opcodes,
                )
            })
            .collect();
        let assert_eq_interaction_claims = self
            .assert_eq
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.verify_instruction,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.opcodes,
                )
            })
            .collect();
        let assert_eq_imm_interaction_claims = self
            .assert_eq_imm
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.verify_instruction,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.opcodes,
                )
            })
            .collect();
        let assert_eq_double_deref_interaction_claims = self
            .assert_eq_double_deref
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.verify_instruction,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                )
            })
            .collect();
        let blake_interaction_claims = self
            .blake
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.verify_instruction,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.range_checks.rc_7_2_5,
                    &interaction_elements.verify_bitwise_xor_8,
                    &interaction_elements.blake_round,
                    &interaction_elements.triple_xor_32,
                    &interaction_elements.opcodes,
                )
            })
            .collect();
        let call_interaction_claims = self
            .call
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.verify_instruction,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                )
            })
            .collect();
        let call_rel_imm_interaction_claims = self
            .call_rel_imm
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.verify_instruction,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                )
            })
            .collect();
        let generic_opcode_interaction_claims = self
            .generic_opcode_interaction_gens
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.verify_instruction,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.range_checks.rc_9_9,
                    &interaction_elements.range_checks.rc_9_9_b,
                    &interaction_elements.range_checks.rc_9_9_c,
                    &interaction_elements.range_checks.rc_9_9_d,
                    &interaction_elements.range_checks.rc_9_9_e,
                    &interaction_elements.range_checks.rc_9_9_f,
                    &interaction_elements.range_checks.rc_9_9_g,
                    &interaction_elements.range_checks.rc_9_9_h,
                    &interaction_elements.range_checks.rc_20,
                    &interaction_elements.range_checks.rc_20_b,
                    &interaction_elements.range_checks.rc_20_c,
                    &interaction_elements.range_checks.rc_20_d,
                    &interaction_elements.range_checks.rc_20_e,
                    &interaction_elements.range_checks.rc_20_f,
                    &interaction_elements.range_checks.rc_20_g,
                    &interaction_elements.range_checks.rc_20_h,
                    &interaction_elements.range_checks.rc_18,
                    &interaction_elements.range_checks.rc_11,
                    &interaction_elements.opcodes,
                )
            })
            .collect();
        let jnz_interaction_claims = self
            .jnz
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.verify_instruction,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                )
            })
            .collect();
        let jnz_taken_interaction_claims = self
            .jnz_taken
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.verify_instruction,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                )
            })
            .collect();
        let jump_interaction_claims = self
            .jump
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.verify_instruction,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                )
            })
            .collect();
        let jump_double_deref_interaction_claims = self
            .jump_double_deref
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.verify_instruction,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                )
            })
            .collect();
        let jump_rel_interaction_claims = self
            .jump_rel
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.verify_instruction,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                )
            })
            .collect();
        let jump_rel_imm_interaction_claims = self
            .jump_rel_imm
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.verify_instruction,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                )
            })
            .collect();
        let mul_interaction_claims = self
            .mul
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.verify_instruction,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.range_checks.rc_20,
                    &interaction_elements.range_checks.rc_20_b,
                    &interaction_elements.range_checks.rc_20_c,
                    &interaction_elements.range_checks.rc_20_d,
                    &interaction_elements.range_checks.rc_20_e,
                    &interaction_elements.range_checks.rc_20_f,
                    &interaction_elements.range_checks.rc_20_g,
                    &interaction_elements.range_checks.rc_20_h,
                    &interaction_elements.opcodes,
                )
            })
            .collect();
        let mul_small_interaction_claims = self
            .mul_small
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.verify_instruction,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.range_checks.rc_11,
                    &interaction_elements.opcodes,
                )
            })
            .collect();
        let qm31_interaction_claims = self
            .qm31
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.verify_instruction,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.range_checks.rc_4_4_4_4,
                    &interaction_elements.opcodes,
                )
            })
            .collect();
        let ret_interaction_claims = self
            .ret_interaction_gens
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.verify_instruction,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                )
            })
            .collect();
        OpcodeInteractionClaim {
            add: add_interaction_claims,
            add_small: add_small_interaction_claims,
            add_ap: add_ap_interaction_claims,
            assert_eq: assert_eq_interaction_claims,
            assert_eq_imm: assert_eq_imm_interaction_claims,
            assert_eq_double_deref: assert_eq_double_deref_interaction_claims,
            blake: blake_interaction_claims,
            call: call_interaction_claims,
            call_rel_imm: call_rel_imm_interaction_claims,
            generic: generic_opcode_interaction_claims,
            jnz: jnz_interaction_claims,
            jnz_taken: jnz_taken_interaction_claims,
            jump: jump_interaction_claims,
            jump_double_deref: jump_double_deref_interaction_claims,
            jump_rel: jump_rel_interaction_claims,
            jump_rel_imm: jump_rel_imm_interaction_claims,
            mul: mul_interaction_claims,
            mul_small: mul_small_interaction_claims,
            qm31: qm31_interaction_claims,
            ret: ret_interaction_claims,
        }
    }
}
