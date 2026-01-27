use cairo_air::opcodes_air::OpcodeClaim;
use stwo::prover::backend::simd::SimdBackend;
use stwo_cairo_adapter::opcodes::CasmStatesByOpcode;

use crate::witness::components::{
    add_ap_opcode,
    // add_opcode, add_opcode_small,
    assert_eq_opcode,
    // assert_eq_opcode_double_deref, assert_eq_opcode_imm, blake_compress_opcode, blake_round,
    // call_opcode_abs,
    call_opcode_rel_imm,
    // generic_opcode,
    // jnz_opcode_non_taken,
    // jnz_opcode_taken,
    // jump_opcode_abs, jump_opcode_double_deref, jump_opcode_rel,
    // jump_opcode_rel_imm,
    memory_address_to_id,
    memory_id_to_big,
    // mul_opcode, mul_opcode_small, qm_31_add_mul_opcode,
    range_check_11,
    range_check_18,
    // range_check_20, range_check_4_4_4_4, range_check_9_9,
    range_check_7_2_5,
    ret_opcode,
    // triple_xor_32, verify_bitwise_xor_8,
    verify_instruction,
};
use crate::witness::utils::TreeBuilder;

pub fn get_opcodes(casm_states_by_opcode: &CasmStatesByOpcode) -> Vec<&'static str> {
    let mut opcodes = vec![];
    // if !casm_states_by_opcode.add_opcode.is_empty() {
    //     opcodes.push("add_opcode");
    // }
    // if !casm_states_by_opcode.add_opcode_small.is_empty() {
    //     opcodes.push("add_opcode_small");
    // }
    if !casm_states_by_opcode.add_ap_opcode.is_empty() {
        opcodes.push("add_ap_opcode");
    }
    if !casm_states_by_opcode.assert_eq_opcode.is_empty() {
        opcodes.push("assert_eq_opcode");
    }
    // if !casm_states_by_opcode.assert_eq_opcode_imm.is_empty() {
    //     opcodes.push("assert_eq_opcode_imm");
    // }
    // if !casm_states_by_opcode
    //     .assert_eq_opcode_double_deref
    //     .is_empty()
    // {
    //     opcodes.push("assert_eq_opcode_double_deref");
    // }
    // if !casm_states_by_opcode.blake_compress_opcode.is_empty() {
    //     opcodes.push("blake_compress_opcode");
    // }
    // if !casm_states_by_opcode.call_opcode_abs.is_empty() {
    //     opcodes.push("call_opcode_abs");
    // }
    if !casm_states_by_opcode.call_opcode_rel_imm.is_empty() {
        opcodes.push("call_opcode_rel_imm");
    }
    // if !casm_states_by_opcode.generic_opcode.is_empty() {
    //     opcodes.push("generic_opcode");
    // }
    // if !casm_states_by_opcode.jnz_opcode_non_taken.is_empty() {
    //     opcodes.push("jnz_opcode_non_taken");
    // }
    // if !casm_states_by_opcode.jnz_opcode_taken.is_empty() {
    //     opcodes.push("jnz_opcode_taken");
    // }
    // if !casm_states_by_opcode.jump_opcode_abs.is_empty() {
    //     opcodes.push("jump_opcode_abs");
    // }
    // if !casm_states_by_opcode.jump_opcode_double_deref.is_empty() {
    //     opcodes.push("jump_opcode_double_deref");
    // }
    // if !casm_states_by_opcode.jump_opcode_rel.is_empty() {
    //     opcodes.push("jump_opcode_rel");
    // }
    // if !casm_states_by_opcode.jump_opcode_rel_imm.is_empty() {
    //     opcodes.push("jump_opcode_rel_imm");
    // }
    // if !casm_states_by_opcode.mul_opcode.is_empty() {
    //     opcodes.push("mul_opcode");
    // }
    // if !casm_states_by_opcode.mul_opcode_small.is_empty() {
    //     opcodes.push("mul_opcode_small");
    // }
    // if !casm_states_by_opcode.qm_31_add_mul_opcode.is_empty() {
    //     opcodes.push("qm_31_add_mul_opcode");
    // }
    if !casm_states_by_opcode.ret_opcode.is_empty() {
        opcodes.push("ret_opcode");
    }

    opcodes
}

#[allow(clippy::too_many_arguments)]
pub fn opcodes_write_trace(
    // add: Option<add_opcode::ClaimGenerator>,
    // add_small: Option<add_opcode_small::ClaimGenerator>,
    add_ap: Option<add_ap_opcode::ClaimGenerator>,
    assert_eq: Option<assert_eq_opcode::ClaimGenerator>,
    // assert_eq_imm: Option<assert_eq_opcode_imm::ClaimGenerator>,
    // assert_eq_double_deref: Option<assert_eq_opcode_double_deref::ClaimGenerator>,
    // blake: Option<blake_compress_opcode::ClaimGenerator>,
    // call: Option<call_opcode_abs::ClaimGenerator>,
    call_rel_imm: Option<call_opcode_rel_imm::ClaimGenerator>,
    // generic: Option<generic_opcode::ClaimGenerator>,
    // jnz: Option<jnz_opcode_non_taken::ClaimGenerator>,
    // jnz_taken: Option<jnz_opcode_taken::ClaimGenerator>,
    // jump: Option<jump_opcode_abs::ClaimGenerator>,
    // jump_double_deref: Option<jump_opcode_double_deref::ClaimGenerator>,
    // jump_rel: Option<jump_opcode_rel::ClaimGenerator>,
    // jump_rel_imm: Option<jump_opcode_rel_imm::ClaimGenerator>,
    // mul: Option<mul_opcode::ClaimGenerator>,
    // mul_small: Option<mul_opcode_small::ClaimGenerator>,
    // qm31: Option<qm_31_add_mul_opcode::ClaimGenerator>,
    ret: Option<ret_opcode::ClaimGenerator>,
    tree_builder: &mut impl TreeBuilder<SimdBackend>,
    // blake_round: &mut Option<blake_round::ClaimGenerator>,
    // triple_xor_32: &mut Option<triple_xor_32::ClaimGenerator>,
    memory_address_to_id_trace_generator: Option<&memory_address_to_id::ClaimGenerator>,
    memory_id_to_value_trace_generator: Option<&memory_id_to_big::ClaimGenerator>,
    _rc_7_2_5_trace_generator: Option<&range_check_7_2_5::ClaimGenerator>,
    rc_11_trace_generator: Option<&range_check_11::ClaimGenerator>,
    rc_18_trace_generator: Option<&range_check_18::ClaimGenerator>,
    // rc_20_trace_generator: Option<&range_check_20::ClaimGenerator>,
    // rc_4_4_4_4_trace_generator: Option<&range_check_4_4_4_4::ClaimGenerator>,
    // rc_9_9_trace_generator: Option<&range_check_9_9::ClaimGenerator>,
    verify_instruction_trace_generator: Option<&verify_instruction::ClaimGenerator>,
    // verify_bitwise_xor_8_trace_generator: Option<&mut verify_bitwise_xor_8::ClaimGenerator>,
) -> (OpcodeClaim, OpcodesInteractionClaimGenerator) {
    // let (add_claims, add_interaction_gens) = add
    //     .map(|gen| {
    //         let (trace, claim, interaction_gen) = gen.write_trace(
    //             memory_address_to_id_trace_generator.unwrap(),
    //             memory_id_to_value_trace_generator.unwrap(),
    //             verify_instruction_trace_generator.unwrap(),
    //         );
    //         tree_builder.extend_evals(trace.to_evals());
    //         (claim, interaction_gen)
    //     })
    //     .map(|(claim, interaction_gen)| (vec![claim], vec![interaction_gen]))
    //     .unwrap_or_default();
    // let (add_small_claims, add_small_interaction_gens) = add_small
    //     .map(|gen| {
    //         let (trace, claim, interaction_gen) = gen.write_trace(
    //             memory_address_to_id_trace_generator.unwrap(),
    //             memory_id_to_value_trace_generator.unwrap(),
    //             verify_instruction_trace_generator.unwrap(),
    //         );
    //         tree_builder.extend_evals(trace.to_evals());
    //         (claim, interaction_gen)
    //     })
    //     .map(|(claim, interaction_gen)| (vec![claim], vec![interaction_gen]))
    //     .unwrap_or_default();
    let (add_ap_claims, add_ap_interaction_gens) = add_ap
        .map(|gen| {
            let (trace, claim, interaction_gen) = gen.write_trace(
                memory_address_to_id_trace_generator.unwrap(),
                memory_id_to_value_trace_generator.unwrap(),
                verify_instruction_trace_generator.unwrap(),
                rc_18_trace_generator.unwrap(),
                rc_11_trace_generator.unwrap(),
            );
            tree_builder.extend_evals(trace.to_evals());
            (claim, interaction_gen)
        })
        .map(|(claim, interaction_gen)| (vec![claim], vec![interaction_gen]))
        .unwrap_or_default();
    let (assert_eq_claims, assert_eq_interaction_gens) = assert_eq
        .map(|gen| {
            let (trace, claim, interaction_gen) = gen.write_trace(
                memory_address_to_id_trace_generator.unwrap(),
                memory_id_to_value_trace_generator.unwrap(),
                verify_instruction_trace_generator.unwrap(),
            );
            tree_builder.extend_evals(trace.to_evals());
            (claim, interaction_gen)
        })
        .map(|(claim, interaction_gen)| (vec![claim], vec![interaction_gen]))
        .unwrap_or_default();
    // let (assert_eq_imm_claims, assert_eq_imm_interaction_gens) = assert_eq_imm
    //     .map(|gen| {
    //         let (trace, claim, interaction_gen) = gen.write_trace(
    //             memory_address_to_id_trace_generator.unwrap(),
    //             memory_id_to_value_trace_generator.unwrap(),
    //             verify_instruction_trace_generator.unwrap(),
    //         );
    //         tree_builder.extend_evals(trace.to_evals());
    //         (claim, interaction_gen)
    //     })
    //     .map(|(claim, interaction_gen)| (vec![claim], vec![interaction_gen]))
    //     .unwrap_or_default();
    // let (assert_eq_double_deref_claims, assert_eq_double_deref_interaction_gens) =
    //     assert_eq_double_deref
    //         .map(|gen| {
    //             let (trace, claim, interaction_gen) = gen.write_trace(
    //                 memory_address_to_id_trace_generator.unwrap(),
    //                 memory_id_to_value_trace_generator.unwrap(),
    //                 verify_instruction_trace_generator.unwrap(),
    //             );
    //             tree_builder.extend_evals(trace.to_evals());
    //             (claim, interaction_gen)
    //         })
    //         .map(|(claim, interaction_gen)| (vec![claim], vec![interaction_gen]))
    //         .unwrap_or_default();
    // let (blake_claims, blake_interaction_gens) = blake
    //     .map(|gen| {
    //         let (trace, claim, interaction_gen) = gen.write_trace(
    //             memory_address_to_id_trace_generator.unwrap(),
    //             memory_id_to_value_trace_generator.unwrap(),
    //             verify_instruction_trace_generator.unwrap(),
    //             rc_7_2_5_trace_generator.unwrap(),
    //             verify_bitwise_xor_8_trace_generator.unwrap(),
    //             blake_round.as_mut().unwrap(),
    //             triple_xor_32.as_mut().unwrap(),
    //         );
    //         tree_builder.extend_evals(trace.to_evals());
    //         (claim, interaction_gen)
    //     })
    //     .map(|(claim, interaction_gen)| (vec![claim], vec![interaction_gen]))
    //     .unwrap_or_default();
    // let (call_claims, call_interaction_gens) = call
    //     .map(|gen| {
    //         let (trace, claim, interaction_gen) = gen.write_trace(
    //             memory_address_to_id_trace_generator.unwrap(),
    //             memory_id_to_value_trace_generator.unwrap(),
    //             verify_instruction_trace_generator.unwrap(),
    //         );
    //         tree_builder.extend_evals(trace.to_evals());
    //         (claim, interaction_gen)
    //     })
    //     .map(|(claim, interaction_gen)| (vec![claim], vec![interaction_gen]))
    //     .unwrap_or_default();
    let (call_rel_imm_claims, call_rel_imm_interaction_gens) = call_rel_imm
        .map(|gen| {
            let (trace, claim, interaction_gen) = gen.write_trace(
                memory_address_to_id_trace_generator.unwrap(),
                memory_id_to_value_trace_generator.unwrap(),
                verify_instruction_trace_generator.unwrap(),
            );
            tree_builder.extend_evals(trace.to_evals());
            (claim, interaction_gen)
        })
        .map(|(claim, interaction_gen)| (vec![claim], vec![interaction_gen]))
        .unwrap_or_default();
    // let (generic_opcode_claims, generic_opcode_interaction_gens) = generic
    //     .map(|gen| {
    //         let (trace, claim, interaction_gen) = gen.write_trace(
    //             memory_address_to_id_trace_generator.unwrap(),
    //             memory_id_to_value_trace_generator.unwrap(),
    //             verify_instruction_trace_generator.unwrap(),
    //             rc_9_9_trace_generator.unwrap(),
    //             rc_20_trace_generator.unwrap(),
    //             rc_18_trace_generator.unwrap(),
    //             rc_11_trace_generator.unwrap(),
    //         );
    //         tree_builder.extend_evals(trace.to_evals());
    //         (claim, interaction_gen)
    //     })
    //     .map(|(claim, interaction_gen)| (vec![claim], vec![interaction_gen]))
    //     .unwrap_or_default();
    // let (jnz_claims, jnz_interaction_gens) = jnz
    //     .map(|gen| {
    //         let (trace, claim, interaction_gen) = gen.write_trace(
    //             memory_address_to_id_trace_generator.unwrap(),
    //             memory_id_to_value_trace_generator.unwrap(),
    //             verify_instruction_trace_generator.unwrap(),
    //         );
    //         tree_builder.extend_evals(trace.to_evals());
    //         (claim, interaction_gen)
    //     })
    //     .map(|(claim, interaction_gen)| (vec![claim], vec![interaction_gen]))
    //     .unwrap_or_default();
    // let (jnz_taken_claims, jnz_taken_interaction_gens) = jnz_taken
    //     .map(|gen| {
    //         let (trace, claim, interaction_gen) = gen.write_trace(
    //             memory_address_to_id_trace_generator.unwrap(),
    //             memory_id_to_value_trace_generator.unwrap(),
    //             verify_instruction_trace_generator.unwrap(),
    //         );
    //         tree_builder.extend_evals(trace.to_evals());
    //         (claim, interaction_gen)
    //     })
    //     .map(|(claim, interaction_gen)| (vec![claim], vec![interaction_gen]))
    //     .unwrap_or_default();
    // let (jump_claims, jump_interaction_gens) = jump
    //     .map(|gen| {
    //         let (trace, claim, interaction_gen) = gen.write_trace(
    //             memory_address_to_id_trace_generator.unwrap(),
    //             memory_id_to_value_trace_generator.unwrap(),
    //             verify_instruction_trace_generator.unwrap(),
    //         );
    //         tree_builder.extend_evals(trace.to_evals());
    //         (claim, interaction_gen)
    //     })
    //     .map(|(claim, interaction_gen)| (vec![claim], vec![interaction_gen]))
    //     .unwrap_or_default();
    // let (jump_double_deref_claims, jump_double_deref_interaction_gens) = jump_double_deref
    //     .map(|gen| {
    //         let (trace, claim, interaction_gen) = gen.write_trace(
    //             memory_address_to_id_trace_generator.unwrap(),
    //             memory_id_to_value_trace_generator.unwrap(),
    //             verify_instruction_trace_generator.unwrap(),
    //         );
    //         tree_builder.extend_evals(trace.to_evals());
    //         (claim, interaction_gen)
    //     })
    //     .map(|(claim, interaction_gen)| (vec![claim], vec![interaction_gen]))
    //     .unwrap_or_default();
    // let (jump_rel_claims, jump_rel_interaction_gens) = jump_rel
    //     .map(|gen| {
    //         let (trace, claim, interaction_gen) = gen.write_trace(
    //             memory_address_to_id_trace_generator.unwrap(),
    //             memory_id_to_value_trace_generator.unwrap(),
    //             verify_instruction_trace_generator.unwrap(),
    //         );
    //         tree_builder.extend_evals(trace.to_evals());
    //         (claim, interaction_gen)
    //     })
    //     .map(|(claim, interaction_gen)| (vec![claim], vec![interaction_gen]))
    //     .unwrap_or_default();
    // let (jump_rel_imm_claims, jump_rel_imm_interaction_gens) = jump_rel_imm
    //     .map(|gen| {
    //         let (trace, claim, interaction_gen) = gen.write_trace(
    //             memory_address_to_id_trace_generator.unwrap(),
    //             memory_id_to_value_trace_generator.unwrap(),
    //             verify_instruction_trace_generator.unwrap(),
    //         );
    //         tree_builder.extend_evals(trace.to_evals());
    //         (claim, interaction_gen)
    //     })
    //     .map(|(claim, interaction_gen)| (vec![claim], vec![interaction_gen]))
    //     .unwrap_or_default();
    // let (mul_claims, mul_interaction_gens) = mul
    //     .map(|gen| {
    //         let (trace, claim, interaction_gen) = gen.write_trace(
    //             memory_address_to_id_trace_generator.unwrap(),
    //             memory_id_to_value_trace_generator.unwrap(),
    //             verify_instruction_trace_generator.unwrap(),
    //             rc_20_trace_generator.unwrap(),
    //         );
    //         tree_builder.extend_evals(trace.to_evals());
    //         (claim, interaction_gen)
    //     })
    //     .map(|(claim, interaction_gen)| (vec![claim], vec![interaction_gen]))
    //     .unwrap_or_default();
    // let (mul_small_claims, mul_small_interaction_gens) = mul_small
    //     .map(|gen| {
    //         let (trace, claim, interaction_gen) = gen.write_trace(
    //             memory_address_to_id_trace_generator.unwrap(),
    //             memory_id_to_value_trace_generator.unwrap(),
    //             verify_instruction_trace_generator.unwrap(),
    //             rc_11_trace_generator.unwrap(),
    //         );
    //         tree_builder.extend_evals(trace.to_evals());
    //         (claim, interaction_gen)
    //     })
    //     .map(|(claim, interaction_gen)| (vec![claim], vec![interaction_gen]))
    //     .unwrap_or_default();
    // let (qm31_claims, qm31_interaction_gens) = qm31
    //     .map(|gen| {
    //         let (trace, claim, interaction_gen) = gen.write_trace(
    //             memory_address_to_id_trace_generator.unwrap(),
    //             memory_id_to_value_trace_generator.unwrap(),
    //             verify_instruction_trace_generator.unwrap(),
    //             rc_4_4_4_4_trace_generator.unwrap(),
    //         );
    //         tree_builder.extend_evals(trace.to_evals());
    //         (claim, interaction_gen)
    //     })
    //     .map(|(claim, interaction_gen)| (vec![claim], vec![interaction_gen]))
    //     .unwrap_or_default();
    let (ret_claims, ret_interaction_gens) = ret
        .map(|gen| {
            let (trace, claim, interaction_gen) = gen.write_trace(
                memory_address_to_id_trace_generator.unwrap(),
                memory_id_to_value_trace_generator.unwrap(),
                verify_instruction_trace_generator.unwrap(),
            );
            tree_builder.extend_evals(trace.to_evals());
            (claim, interaction_gen)
        })
        .map(|(claim, interaction_gen)| (vec![claim], vec![interaction_gen]))
        .unwrap_or_default();
    (
        OpcodeClaim {
            add_ap: add_ap_claims,
            assert_eq: assert_eq_claims,
            call_rel_imm: call_rel_imm_claims,
            // jnz: jnz_claims,
            // jnz_taken: jnz_taken_claims,
            // jump_rel_imm: jump_rel_imm_claims,
            ret: ret_claims,
        },
        OpcodesInteractionClaimGenerator {
            add_ap: add_ap_interaction_gens,
            assert_eq: assert_eq_interaction_gens,
            call_rel_imm: call_rel_imm_interaction_gens,
            // jnz: jnz_interaction_gens,
            // jnz_taken: jnz_taken_interaction_gens,
            // jump_rel_imm: jump_rel_imm_interaction_gens,
            ret_interaction_gens,
        },
    )
}

pub struct OpcodesInteractionClaimGenerator {
    // pub add: Vec<add_opcode::InteractionClaimGenerator>,
    // pub add_small: Vec<add_opcode_small::InteractionClaimGenerator>,
    pub add_ap: Vec<add_ap_opcode::InteractionClaimGenerator>,
    pub assert_eq: Vec<assert_eq_opcode::InteractionClaimGenerator>,
    // pub assert_eq_imm: Vec<assert_eq_opcode_imm::InteractionClaimGenerator>,
    // pub assert_eq_double_deref: Vec<assert_eq_opcode_double_deref::InteractionClaimGenerator>,
    // pub blake: Vec<blake_compress_opcode::InteractionClaimGenerator>,
    // pub call: Vec<call_opcode_abs::InteractionClaimGenerator>,
    pub call_rel_imm: Vec<call_opcode_rel_imm::InteractionClaimGenerator>,
    // pub generic_opcode_interaction_gens: Vec<generic_opcode::InteractionClaimGenerator>,
    // pub jnz: Vec<jnz_opcode_non_taken::InteractionClaimGenerator>,
    // pub jnz_taken: Vec<jnz_opcode_taken::InteractionClaimGenerator>,
    // pub jump: Vec<jump_opcode_abs::InteractionClaimGenerator>,
    // pub jump_double_deref: Vec<jump_opcode_double_deref::InteractionClaimGenerator>,
    // pub jump_rel: Vec<jump_opcode_rel::InteractionClaimGenerator>,
    // pub jump_rel_imm: Vec<jump_opcode_rel_imm::InteractionClaimGenerator>,
    // pub mul: Vec<mul_opcode::InteractionClaimGenerator>,
    // pub mul_small: Vec<mul_opcode_small::InteractionClaimGenerator>,
    // pub qm31: Vec<qm_31_add_mul_opcode::InteractionClaimGenerator>,
    pub ret_interaction_gens: Vec<ret_opcode::InteractionClaimGenerator>,
}
