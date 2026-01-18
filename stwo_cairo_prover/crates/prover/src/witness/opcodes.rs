use std::sync::Mutex;

use cairo_air::opcodes_air::{OpcodeClaim, OpcodeInteractionClaim};
use cairo_air::relations::CommonLookupElements;
use rayon::prelude::*;
use stwo::prover::backend::simd::SimdBackend;
use stwo_cairo_adapter::opcodes::CasmStatesByOpcode;

use crate::witness::components::{
    add_ap_opcode, add_opcode, add_opcode_small, assert_eq_opcode, assert_eq_opcode_double_deref,
    assert_eq_opcode_imm, blake_compress_opcode, blake_round, call_opcode_abs, call_opcode_rel_imm,
    generic_opcode, jnz_opcode_non_taken, jnz_opcode_taken, jump_opcode_abs,
    jump_opcode_double_deref, jump_opcode_rel, jump_opcode_rel_imm, memory_address_to_id,
    memory_id_to_big, mul_opcode, mul_opcode_small, qm_31_add_mul_opcode, range_check_11,
    range_check_18, range_check_20, range_check_4_4_4_4, range_check_7_2_5, range_check_9_9,
    ret_opcode, triple_xor_32, verify_bitwise_xor_8, verify_instruction,
};
use crate::witness::utils::{CollectingTreeBuilder, TreeBuilder};

/// Enum to hold opcode write_trace results for parallel processing.
enum OpcodeWriteResult {
    Add(Vec<cairo_air::components::add_opcode::Claim>, Vec<add_opcode::InteractionClaimGenerator>, CollectingTreeBuilder),
    AddSmall(Vec<cairo_air::components::add_opcode_small::Claim>, Vec<add_opcode_small::InteractionClaimGenerator>, CollectingTreeBuilder),
    AddAp(Vec<cairo_air::components::add_ap_opcode::Claim>, Vec<add_ap_opcode::InteractionClaimGenerator>, CollectingTreeBuilder),
    AssertEq(Vec<cairo_air::components::assert_eq_opcode::Claim>, Vec<assert_eq_opcode::InteractionClaimGenerator>, CollectingTreeBuilder),
    AssertEqImm(Vec<cairo_air::components::assert_eq_opcode_imm::Claim>, Vec<assert_eq_opcode_imm::InteractionClaimGenerator>, CollectingTreeBuilder),
    AssertEqDoubleDeref(Vec<cairo_air::components::assert_eq_opcode_double_deref::Claim>, Vec<assert_eq_opcode_double_deref::InteractionClaimGenerator>, CollectingTreeBuilder),
    Call(Vec<cairo_air::components::call_opcode_abs::Claim>, Vec<call_opcode_abs::InteractionClaimGenerator>, CollectingTreeBuilder),
    CallRelImm(Vec<cairo_air::components::call_opcode_rel_imm::Claim>, Vec<call_opcode_rel_imm::InteractionClaimGenerator>, CollectingTreeBuilder),
    Generic(Vec<cairo_air::components::generic_opcode::Claim>, Vec<generic_opcode::InteractionClaimGenerator>, CollectingTreeBuilder),
    Jnz(Vec<cairo_air::components::jnz_opcode_non_taken::Claim>, Vec<jnz_opcode_non_taken::InteractionClaimGenerator>, CollectingTreeBuilder),
    JnzTaken(Vec<cairo_air::components::jnz_opcode_taken::Claim>, Vec<jnz_opcode_taken::InteractionClaimGenerator>, CollectingTreeBuilder),
    Jump(Vec<cairo_air::components::jump_opcode_abs::Claim>, Vec<jump_opcode_abs::InteractionClaimGenerator>, CollectingTreeBuilder),
    JumpDoubleDeref(Vec<cairo_air::components::jump_opcode_double_deref::Claim>, Vec<jump_opcode_double_deref::InteractionClaimGenerator>, CollectingTreeBuilder),
    JumpRel(Vec<cairo_air::components::jump_opcode_rel::Claim>, Vec<jump_opcode_rel::InteractionClaimGenerator>, CollectingTreeBuilder),
    JumpRelImm(Vec<cairo_air::components::jump_opcode_rel_imm::Claim>, Vec<jump_opcode_rel_imm::InteractionClaimGenerator>, CollectingTreeBuilder),
    Mul(Vec<cairo_air::components::mul_opcode::Claim>, Vec<mul_opcode::InteractionClaimGenerator>, CollectingTreeBuilder),
    MulSmall(Vec<cairo_air::components::mul_opcode_small::Claim>, Vec<mul_opcode_small::InteractionClaimGenerator>, CollectingTreeBuilder),
    Qm31(Vec<cairo_air::components::qm_31_add_mul_opcode::Claim>, Vec<qm_31_add_mul_opcode::InteractionClaimGenerator>, CollectingTreeBuilder),
    Ret(Vec<cairo_air::components::ret_opcode::Claim>, Vec<ret_opcode::InteractionClaimGenerator>, CollectingTreeBuilder),
}

/// Enum to hold opcode task inputs for parallel processing.
enum OpcodeTask<'a> {
    Add(add_opcode::ClaimGenerator, &'a memory_address_to_id::ClaimGenerator, &'a memory_id_to_big::ClaimGenerator, &'a verify_instruction::ClaimGenerator),
    AddSmall(add_opcode_small::ClaimGenerator, &'a memory_address_to_id::ClaimGenerator, &'a memory_id_to_big::ClaimGenerator, &'a verify_instruction::ClaimGenerator),
    AddAp(add_ap_opcode::ClaimGenerator, &'a memory_address_to_id::ClaimGenerator, &'a memory_id_to_big::ClaimGenerator, &'a verify_instruction::ClaimGenerator, &'a range_check_18::ClaimGenerator, &'a range_check_11::ClaimGenerator),
    AssertEq(assert_eq_opcode::ClaimGenerator, &'a memory_address_to_id::ClaimGenerator, &'a memory_id_to_big::ClaimGenerator, &'a verify_instruction::ClaimGenerator),
    AssertEqImm(assert_eq_opcode_imm::ClaimGenerator, &'a memory_address_to_id::ClaimGenerator, &'a memory_id_to_big::ClaimGenerator, &'a verify_instruction::ClaimGenerator),
    AssertEqDoubleDeref(assert_eq_opcode_double_deref::ClaimGenerator, &'a memory_address_to_id::ClaimGenerator, &'a memory_id_to_big::ClaimGenerator, &'a verify_instruction::ClaimGenerator),
    Call(call_opcode_abs::ClaimGenerator, &'a memory_address_to_id::ClaimGenerator, &'a memory_id_to_big::ClaimGenerator, &'a verify_instruction::ClaimGenerator),
    CallRelImm(call_opcode_rel_imm::ClaimGenerator, &'a memory_address_to_id::ClaimGenerator, &'a memory_id_to_big::ClaimGenerator, &'a verify_instruction::ClaimGenerator),
    Generic(generic_opcode::ClaimGenerator, &'a memory_address_to_id::ClaimGenerator, &'a memory_id_to_big::ClaimGenerator, &'a verify_instruction::ClaimGenerator, &'a range_check_9_9::ClaimGenerator, &'a range_check_20::ClaimGenerator, &'a range_check_18::ClaimGenerator, &'a range_check_11::ClaimGenerator),
    Jnz(jnz_opcode_non_taken::ClaimGenerator, &'a memory_address_to_id::ClaimGenerator, &'a memory_id_to_big::ClaimGenerator, &'a verify_instruction::ClaimGenerator),
    JnzTaken(jnz_opcode_taken::ClaimGenerator, &'a memory_address_to_id::ClaimGenerator, &'a memory_id_to_big::ClaimGenerator, &'a verify_instruction::ClaimGenerator),
    Jump(jump_opcode_abs::ClaimGenerator, &'a memory_address_to_id::ClaimGenerator, &'a memory_id_to_big::ClaimGenerator, &'a verify_instruction::ClaimGenerator),
    JumpDoubleDeref(jump_opcode_double_deref::ClaimGenerator, &'a memory_address_to_id::ClaimGenerator, &'a memory_id_to_big::ClaimGenerator, &'a verify_instruction::ClaimGenerator),
    JumpRel(jump_opcode_rel::ClaimGenerator, &'a memory_address_to_id::ClaimGenerator, &'a memory_id_to_big::ClaimGenerator, &'a verify_instruction::ClaimGenerator),
    JumpRelImm(jump_opcode_rel_imm::ClaimGenerator, &'a memory_address_to_id::ClaimGenerator, &'a memory_id_to_big::ClaimGenerator, &'a verify_instruction::ClaimGenerator),
    Mul(mul_opcode::ClaimGenerator, &'a memory_address_to_id::ClaimGenerator, &'a memory_id_to_big::ClaimGenerator, &'a verify_instruction::ClaimGenerator, &'a range_check_20::ClaimGenerator),
    MulSmall(mul_opcode_small::ClaimGenerator, &'a memory_address_to_id::ClaimGenerator, &'a memory_id_to_big::ClaimGenerator, &'a verify_instruction::ClaimGenerator, &'a range_check_11::ClaimGenerator),
    Qm31(qm_31_add_mul_opcode::ClaimGenerator, &'a memory_address_to_id::ClaimGenerator, &'a memory_id_to_big::ClaimGenerator, &'a verify_instruction::ClaimGenerator, &'a range_check_4_4_4_4::ClaimGenerator),
    Ret(ret_opcode::ClaimGenerator, &'a memory_address_to_id::ClaimGenerator, &'a memory_id_to_big::ClaimGenerator, &'a verify_instruction::ClaimGenerator),
}

impl OpcodeTask<'_> {
    fn process(self) -> OpcodeWriteResult {
        match self {
            OpcodeTask::Add(gen, mem_addr, mem_id, verify_inst) => {
                let mut collector = CollectingTreeBuilder::new();
                let (claim, ig) = gen.write_trace(&mut collector, mem_addr, mem_id, verify_inst);
                OpcodeWriteResult::Add(vec![claim], vec![ig], collector)
            }
            OpcodeTask::AddSmall(gen, mem_addr, mem_id, verify_inst) => {
                let mut collector = CollectingTreeBuilder::new();
                let (claim, ig) = gen.write_trace(&mut collector, mem_addr, mem_id, verify_inst);
                OpcodeWriteResult::AddSmall(vec![claim], vec![ig], collector)
            }
            OpcodeTask::AddAp(gen, mem_addr, mem_id, verify_inst, rc_18, rc_11) => {
                let mut collector = CollectingTreeBuilder::new();
                let (claim, ig) = gen.write_trace(&mut collector, mem_addr, mem_id, verify_inst, rc_18, rc_11);
                OpcodeWriteResult::AddAp(vec![claim], vec![ig], collector)
            }
            OpcodeTask::AssertEq(gen, mem_addr, mem_id, verify_inst) => {
                let mut collector = CollectingTreeBuilder::new();
                let (claim, ig) = gen.write_trace(&mut collector, mem_addr, mem_id, verify_inst);
                OpcodeWriteResult::AssertEq(vec![claim], vec![ig], collector)
            }
            OpcodeTask::AssertEqImm(gen, mem_addr, mem_id, verify_inst) => {
                let mut collector = CollectingTreeBuilder::new();
                let (claim, ig) = gen.write_trace(&mut collector, mem_addr, mem_id, verify_inst);
                OpcodeWriteResult::AssertEqImm(vec![claim], vec![ig], collector)
            }
            OpcodeTask::AssertEqDoubleDeref(gen, mem_addr, mem_id, verify_inst) => {
                let mut collector = CollectingTreeBuilder::new();
                let (claim, ig) = gen.write_trace(&mut collector, mem_addr, mem_id, verify_inst);
                OpcodeWriteResult::AssertEqDoubleDeref(vec![claim], vec![ig], collector)
            }
            OpcodeTask::Call(gen, mem_addr, mem_id, verify_inst) => {
                let mut collector = CollectingTreeBuilder::new();
                let (claim, ig) = gen.write_trace(&mut collector, mem_addr, mem_id, verify_inst);
                OpcodeWriteResult::Call(vec![claim], vec![ig], collector)
            }
            OpcodeTask::CallRelImm(gen, mem_addr, mem_id, verify_inst) => {
                let mut collector = CollectingTreeBuilder::new();
                let (claim, ig) = gen.write_trace(&mut collector, mem_addr, mem_id, verify_inst);
                OpcodeWriteResult::CallRelImm(vec![claim], vec![ig], collector)
            }
            OpcodeTask::Generic(gen, mem_addr, mem_id, verify_inst, rc_9_9, rc_20, rc_18, rc_11) => {
                let mut collector = CollectingTreeBuilder::new();
                let (claim, ig) = gen.write_trace(&mut collector, mem_addr, mem_id, verify_inst, rc_9_9, rc_20, rc_18, rc_11);
                OpcodeWriteResult::Generic(vec![claim], vec![ig], collector)
            }
            OpcodeTask::Jnz(gen, mem_addr, mem_id, verify_inst) => {
                let mut collector = CollectingTreeBuilder::new();
                let (claim, ig) = gen.write_trace(&mut collector, mem_addr, mem_id, verify_inst);
                OpcodeWriteResult::Jnz(vec![claim], vec![ig], collector)
            }
            OpcodeTask::JnzTaken(gen, mem_addr, mem_id, verify_inst) => {
                let mut collector = CollectingTreeBuilder::new();
                let (claim, ig) = gen.write_trace(&mut collector, mem_addr, mem_id, verify_inst);
                OpcodeWriteResult::JnzTaken(vec![claim], vec![ig], collector)
            }
            OpcodeTask::Jump(gen, mem_addr, mem_id, verify_inst) => {
                let mut collector = CollectingTreeBuilder::new();
                let (claim, ig) = gen.write_trace(&mut collector, mem_addr, mem_id, verify_inst);
                OpcodeWriteResult::Jump(vec![claim], vec![ig], collector)
            }
            OpcodeTask::JumpDoubleDeref(gen, mem_addr, mem_id, verify_inst) => {
                let mut collector = CollectingTreeBuilder::new();
                let (claim, ig) = gen.write_trace(&mut collector, mem_addr, mem_id, verify_inst);
                OpcodeWriteResult::JumpDoubleDeref(vec![claim], vec![ig], collector)
            }
            OpcodeTask::JumpRel(gen, mem_addr, mem_id, verify_inst) => {
                let mut collector = CollectingTreeBuilder::new();
                let (claim, ig) = gen.write_trace(&mut collector, mem_addr, mem_id, verify_inst);
                OpcodeWriteResult::JumpRel(vec![claim], vec![ig], collector)
            }
            OpcodeTask::JumpRelImm(gen, mem_addr, mem_id, verify_inst) => {
                let mut collector = CollectingTreeBuilder::new();
                let (claim, ig) = gen.write_trace(&mut collector, mem_addr, mem_id, verify_inst);
                OpcodeWriteResult::JumpRelImm(vec![claim], vec![ig], collector)
            }
            OpcodeTask::Mul(gen, mem_addr, mem_id, verify_inst, rc_20) => {
                let mut collector = CollectingTreeBuilder::new();
                let (claim, ig) = gen.write_trace(&mut collector, mem_addr, mem_id, verify_inst, rc_20);
                OpcodeWriteResult::Mul(vec![claim], vec![ig], collector)
            }
            OpcodeTask::MulSmall(gen, mem_addr, mem_id, verify_inst, rc_11) => {
                let mut collector = CollectingTreeBuilder::new();
                let (claim, ig) = gen.write_trace(&mut collector, mem_addr, mem_id, verify_inst, rc_11);
                OpcodeWriteResult::MulSmall(vec![claim], vec![ig], collector)
            }
            OpcodeTask::Qm31(gen, mem_addr, mem_id, verify_inst, rc_4_4_4_4) => {
                let mut collector = CollectingTreeBuilder::new();
                let (claim, ig) = gen.write_trace(&mut collector, mem_addr, mem_id, verify_inst, rc_4_4_4_4);
                OpcodeWriteResult::Qm31(vec![claim], vec![ig], collector)
            }
            OpcodeTask::Ret(gen, mem_addr, mem_id, verify_inst) => {
                let mut collector = CollectingTreeBuilder::new();
                let (claim, ig) = gen.write_trace(&mut collector, mem_addr, mem_id, verify_inst);
                OpcodeWriteResult::Ret(vec![claim], vec![ig], collector)
            }
        }
    }
}

pub fn get_opcodes(casm_states_by_opcode: &CasmStatesByOpcode) -> Vec<&'static str> {
    let mut opcodes = vec![];
    if !casm_states_by_opcode.add_opcode.is_empty() {
        opcodes.push("add_opcode");
    }
    if !casm_states_by_opcode.add_opcode_small.is_empty() {
        opcodes.push("add_opcode_small");
    }
    if !casm_states_by_opcode.add_ap_opcode.is_empty() {
        opcodes.push("add_ap_opcode");
    }
    if !casm_states_by_opcode.assert_eq_opcode.is_empty() {
        opcodes.push("assert_eq_opcode");
    }
    if !casm_states_by_opcode.assert_eq_opcode_imm.is_empty() {
        opcodes.push("assert_eq_opcode_imm");
    }
    if !casm_states_by_opcode
        .assert_eq_opcode_double_deref
        .is_empty()
    {
        opcodes.push("assert_eq_opcode_double_deref");
    }
    if !casm_states_by_opcode.blake_compress_opcode.is_empty() {
        opcodes.push("blake_compress_opcode");
    }
    if !casm_states_by_opcode.call_opcode_abs.is_empty() {
        opcodes.push("call_opcode_abs");
    }
    if !casm_states_by_opcode.call_opcode_rel_imm.is_empty() {
        opcodes.push("call_opcode_rel_imm");
    }
    if !casm_states_by_opcode.generic_opcode.is_empty() {
        opcodes.push("generic_opcode");
    }
    if !casm_states_by_opcode.jnz_opcode_non_taken.is_empty() {
        opcodes.push("jnz_opcode_non_taken");
    }
    if !casm_states_by_opcode.jnz_opcode_taken.is_empty() {
        opcodes.push("jnz_opcode_taken");
    }
    if !casm_states_by_opcode.jump_opcode_abs.is_empty() {
        opcodes.push("jump_opcode_abs");
    }
    if !casm_states_by_opcode.jump_opcode_double_deref.is_empty() {
        opcodes.push("jump_opcode_double_deref");
    }
    if !casm_states_by_opcode.jump_opcode_rel.is_empty() {
        opcodes.push("jump_opcode_rel");
    }
    if !casm_states_by_opcode.jump_opcode_rel_imm.is_empty() {
        opcodes.push("jump_opcode_rel_imm");
    }
    if !casm_states_by_opcode.mul_opcode.is_empty() {
        opcodes.push("mul_opcode");
    }
    if !casm_states_by_opcode.mul_opcode_small.is_empty() {
        opcodes.push("mul_opcode_small");
    }
    if !casm_states_by_opcode.qm_31_add_mul_opcode.is_empty() {
        opcodes.push("qm_31_add_mul_opcode");
    }
    if !casm_states_by_opcode.ret_opcode.is_empty() {
        opcodes.push("ret_opcode");
    }

    opcodes
}

pub fn opcodes_write_trace(
    add: Option<add_opcode::ClaimGenerator>,
    add_small: Option<add_opcode_small::ClaimGenerator>,
    add_ap: Option<add_ap_opcode::ClaimGenerator>,
    assert_eq: Option<assert_eq_opcode::ClaimGenerator>,
    assert_eq_imm: Option<assert_eq_opcode_imm::ClaimGenerator>,
    assert_eq_double_deref: Option<assert_eq_opcode_double_deref::ClaimGenerator>,
    blake: Option<blake_compress_opcode::ClaimGenerator>,
    call: Option<call_opcode_abs::ClaimGenerator>,
    call_rel_imm: Option<call_opcode_rel_imm::ClaimGenerator>,
    generic: Option<generic_opcode::ClaimGenerator>,
    jnz: Option<jnz_opcode_non_taken::ClaimGenerator>,
    jnz_taken: Option<jnz_opcode_taken::ClaimGenerator>,
    jump: Option<jump_opcode_abs::ClaimGenerator>,
    jump_double_deref: Option<jump_opcode_double_deref::ClaimGenerator>,
    jump_rel: Option<jump_opcode_rel::ClaimGenerator>,
    jump_rel_imm: Option<jump_opcode_rel_imm::ClaimGenerator>,
    mul: Option<mul_opcode::ClaimGenerator>,
    mul_small: Option<mul_opcode_small::ClaimGenerator>,
    qm31: Option<qm_31_add_mul_opcode::ClaimGenerator>,
    ret: Option<ret_opcode::ClaimGenerator>,
    tree_builder: &mut impl TreeBuilder<SimdBackend>,
    blake_round: &mut Option<blake_round::ClaimGenerator>,
    triple_xor_32: &mut Option<triple_xor_32::ClaimGenerator>,
    memory_address_to_id_trace_generator: Option<&memory_address_to_id::ClaimGenerator>,
    memory_id_to_value_trace_generator: Option<&memory_id_to_big::ClaimGenerator>,
    rc_7_2_5_trace_generator: Option<&range_check_7_2_5::ClaimGenerator>,
    rc_11_trace_generator: Option<&range_check_11::ClaimGenerator>,
    rc_18_trace_generator: Option<&range_check_18::ClaimGenerator>,
    rc_20_trace_generator: Option<&range_check_20::ClaimGenerator>,
    rc_4_4_4_4_trace_generator: Option<&range_check_4_4_4_4::ClaimGenerator>,
    rc_9_9_trace_generator: Option<&range_check_9_9::ClaimGenerator>,
    verify_instruction_trace_generator: Option<&verify_instruction::ClaimGenerator>,
    verify_bitwise_xor_8_trace_generator: Option<&mut verify_bitwise_xor_8::ClaimGenerator>,
) -> (OpcodeClaim, OpcodesInteractionClaimGenerator) {
    // Build tasks for all opcodes (except blake which needs &mut references).
    let mem_addr = memory_address_to_id_trace_generator.unwrap();
    let mem_id = memory_id_to_value_trace_generator.unwrap();
    let verify_inst = verify_instruction_trace_generator.unwrap();
    let rc_11 = rc_11_trace_generator.unwrap();
    let rc_18 = rc_18_trace_generator.unwrap();
    let rc_20 = rc_20_trace_generator.unwrap();
    let rc_9_9 = rc_9_9_trace_generator.unwrap();
    let rc_4_4_4_4 = rc_4_4_4_4_trace_generator.unwrap();

    let mut tasks: Vec<OpcodeTask> = Vec::new();

    if let Some(gen) = add {
        tasks.push(OpcodeTask::Add(gen, mem_addr, mem_id, verify_inst));
    }
    if let Some(gen) = add_small {
        tasks.push(OpcodeTask::AddSmall(gen, mem_addr, mem_id, verify_inst));
    }
    if let Some(gen) = add_ap {
        tasks.push(OpcodeTask::AddAp(gen, mem_addr, mem_id, verify_inst, rc_18, rc_11));
    }
    if let Some(gen) = assert_eq {
        tasks.push(OpcodeTask::AssertEq(gen, mem_addr, mem_id, verify_inst));
    }
    if let Some(gen) = assert_eq_imm {
        tasks.push(OpcodeTask::AssertEqImm(gen, mem_addr, mem_id, verify_inst));
    }
    if let Some(gen) = assert_eq_double_deref {
        tasks.push(OpcodeTask::AssertEqDoubleDeref(gen, mem_addr, mem_id, verify_inst));
    }
    if let Some(gen) = call {
        tasks.push(OpcodeTask::Call(gen, mem_addr, mem_id, verify_inst));
    }
    if let Some(gen) = call_rel_imm {
        tasks.push(OpcodeTask::CallRelImm(gen, mem_addr, mem_id, verify_inst));
    }
    if let Some(gen) = generic {
        tasks.push(OpcodeTask::Generic(gen, mem_addr, mem_id, verify_inst, rc_9_9, rc_20, rc_18, rc_11));
    }
    if let Some(gen) = jnz {
        tasks.push(OpcodeTask::Jnz(gen, mem_addr, mem_id, verify_inst));
    }
    if let Some(gen) = jnz_taken {
        tasks.push(OpcodeTask::JnzTaken(gen, mem_addr, mem_id, verify_inst));
    }
    if let Some(gen) = jump {
        tasks.push(OpcodeTask::Jump(gen, mem_addr, mem_id, verify_inst));
    }
    if let Some(gen) = jump_double_deref {
        tasks.push(OpcodeTask::JumpDoubleDeref(gen, mem_addr, mem_id, verify_inst));
    }
    if let Some(gen) = jump_rel {
        tasks.push(OpcodeTask::JumpRel(gen, mem_addr, mem_id, verify_inst));
    }
    if let Some(gen) = jump_rel_imm {
        tasks.push(OpcodeTask::JumpRelImm(gen, mem_addr, mem_id, verify_inst));
    }
    if let Some(gen) = mul {
        tasks.push(OpcodeTask::Mul(gen, mem_addr, mem_id, verify_inst, rc_20));
    }
    if let Some(gen) = mul_small {
        tasks.push(OpcodeTask::MulSmall(gen, mem_addr, mem_id, verify_inst, rc_11));
    }
    if let Some(gen) = qm31 {
        tasks.push(OpcodeTask::Qm31(gen, mem_addr, mem_id, verify_inst, rc_4_4_4_4));
    }
    if let Some(gen) = ret {
        tasks.push(OpcodeTask::Ret(gen, mem_addr, mem_id, verify_inst));
    }

    // Process all tasks in parallel.
    let results: Vec<OpcodeWriteResult> = tasks.into_par_iter().map(|task| task.process()).collect();

    // Extract results into typed containers.
    let mut add_claims = vec![];
    let mut add_interaction_gens = vec![];
    let mut add_trace = CollectingTreeBuilder::new();
    let mut add_small_claims = vec![];
    let mut add_small_interaction_gens = vec![];
    let mut add_small_trace = CollectingTreeBuilder::new();
    let mut add_ap_claims = vec![];
    let mut add_ap_interaction_gens = vec![];
    let mut add_ap_trace = CollectingTreeBuilder::new();
    let mut assert_eq_claims = vec![];
    let mut assert_eq_interaction_gens = vec![];
    let mut assert_eq_trace = CollectingTreeBuilder::new();
    let mut assert_eq_imm_claims = vec![];
    let mut assert_eq_imm_interaction_gens = vec![];
    let mut assert_eq_imm_trace = CollectingTreeBuilder::new();
    let mut assert_eq_double_deref_claims = vec![];
    let mut assert_eq_double_deref_interaction_gens = vec![];
    let mut assert_eq_double_deref_trace = CollectingTreeBuilder::new();
    let mut call_claims = vec![];
    let mut call_interaction_gens = vec![];
    let mut call_trace = CollectingTreeBuilder::new();
    let mut call_rel_imm_claims = vec![];
    let mut call_rel_imm_interaction_gens = vec![];
    let mut call_rel_imm_trace = CollectingTreeBuilder::new();
    let mut generic_opcode_claims = vec![];
    let mut generic_opcode_interaction_gens = vec![];
    let mut generic_trace = CollectingTreeBuilder::new();
    let mut jnz_claims = vec![];
    let mut jnz_interaction_gens = vec![];
    let mut jnz_trace = CollectingTreeBuilder::new();
    let mut jnz_taken_claims = vec![];
    let mut jnz_taken_interaction_gens = vec![];
    let mut jnz_taken_trace = CollectingTreeBuilder::new();
    let mut jump_claims = vec![];
    let mut jump_interaction_gens = vec![];
    let mut jump_trace = CollectingTreeBuilder::new();
    let mut jump_double_deref_claims = vec![];
    let mut jump_double_deref_interaction_gens = vec![];
    let mut jump_double_deref_trace = CollectingTreeBuilder::new();
    let mut jump_rel_claims = vec![];
    let mut jump_rel_interaction_gens = vec![];
    let mut jump_rel_trace = CollectingTreeBuilder::new();
    let mut jump_rel_imm_claims = vec![];
    let mut jump_rel_imm_interaction_gens = vec![];
    let mut jump_rel_imm_trace = CollectingTreeBuilder::new();
    let mut mul_claims = vec![];
    let mut mul_interaction_gens = vec![];
    let mut mul_trace = CollectingTreeBuilder::new();
    let mut mul_small_claims = vec![];
    let mut mul_small_interaction_gens = vec![];
    let mut mul_small_trace = CollectingTreeBuilder::new();
    let mut qm31_claims = vec![];
    let mut qm31_interaction_gens = vec![];
    let mut qm31_trace = CollectingTreeBuilder::new();
    let mut ret_claims = vec![];
    let mut ret_interaction_gens = vec![];
    let mut ret_trace = CollectingTreeBuilder::new();

    for result in results {
        match result {
            OpcodeWriteResult::Add(c, i, t) => { add_claims = c; add_interaction_gens = i; add_trace = t; }
            OpcodeWriteResult::AddSmall(c, i, t) => { add_small_claims = c; add_small_interaction_gens = i; add_small_trace = t; }
            OpcodeWriteResult::AddAp(c, i, t) => { add_ap_claims = c; add_ap_interaction_gens = i; add_ap_trace = t; }
            OpcodeWriteResult::AssertEq(c, i, t) => { assert_eq_claims = c; assert_eq_interaction_gens = i; assert_eq_trace = t; }
            OpcodeWriteResult::AssertEqImm(c, i, t) => { assert_eq_imm_claims = c; assert_eq_imm_interaction_gens = i; assert_eq_imm_trace = t; }
            OpcodeWriteResult::AssertEqDoubleDeref(c, i, t) => { assert_eq_double_deref_claims = c; assert_eq_double_deref_interaction_gens = i; assert_eq_double_deref_trace = t; }
            OpcodeWriteResult::Call(c, i, t) => { call_claims = c; call_interaction_gens = i; call_trace = t; }
            OpcodeWriteResult::CallRelImm(c, i, t) => { call_rel_imm_claims = c; call_rel_imm_interaction_gens = i; call_rel_imm_trace = t; }
            OpcodeWriteResult::Generic(c, i, t) => { generic_opcode_claims = c; generic_opcode_interaction_gens = i; generic_trace = t; }
            OpcodeWriteResult::Jnz(c, i, t) => { jnz_claims = c; jnz_interaction_gens = i; jnz_trace = t; }
            OpcodeWriteResult::JnzTaken(c, i, t) => { jnz_taken_claims = c; jnz_taken_interaction_gens = i; jnz_taken_trace = t; }
            OpcodeWriteResult::Jump(c, i, t) => { jump_claims = c; jump_interaction_gens = i; jump_trace = t; }
            OpcodeWriteResult::JumpDoubleDeref(c, i, t) => { jump_double_deref_claims = c; jump_double_deref_interaction_gens = i; jump_double_deref_trace = t; }
            OpcodeWriteResult::JumpRel(c, i, t) => { jump_rel_claims = c; jump_rel_interaction_gens = i; jump_rel_trace = t; }
            OpcodeWriteResult::JumpRelImm(c, i, t) => { jump_rel_imm_claims = c; jump_rel_imm_interaction_gens = i; jump_rel_imm_trace = t; }
            OpcodeWriteResult::Mul(c, i, t) => { mul_claims = c; mul_interaction_gens = i; mul_trace = t; }
            OpcodeWriteResult::MulSmall(c, i, t) => { mul_small_claims = c; mul_small_interaction_gens = i; mul_small_trace = t; }
            OpcodeWriteResult::Qm31(c, i, t) => { qm31_claims = c; qm31_interaction_gens = i; qm31_trace = t; }
            OpcodeWriteResult::Ret(c, i, t) => { ret_claims = c; ret_interaction_gens = i; ret_trace = t; }
        }
    }

    // Write traces in deterministic order.
    add_trace.write_to(tree_builder);
    add_small_trace.write_to(tree_builder);
    add_ap_trace.write_to(tree_builder);
    assert_eq_trace.write_to(tree_builder);
    assert_eq_imm_trace.write_to(tree_builder);
    assert_eq_double_deref_trace.write_to(tree_builder);

    // Process blake separately (needs &mut references).
    let (blake_claims, blake_interaction_gens) = blake
        .map(|gen| {
            gen.write_trace(
                tree_builder,
                memory_address_to_id_trace_generator.unwrap(),
                memory_id_to_value_trace_generator.unwrap(),
                verify_instruction_trace_generator.unwrap(),
                rc_7_2_5_trace_generator.unwrap(),
                verify_bitwise_xor_8_trace_generator.unwrap(),
                blake_round.as_mut().unwrap(),
                triple_xor_32.as_mut().unwrap(),
            )
        })
        .map(|(claim, interaction_gen)| (vec![claim], vec![interaction_gen]))
        .unwrap_or_default();

    // Continue writing remaining traces.
    call_trace.write_to(tree_builder);
    call_rel_imm_trace.write_to(tree_builder);
    generic_trace.write_to(tree_builder);
    jnz_trace.write_to(tree_builder);
    jnz_taken_trace.write_to(tree_builder);
    jump_trace.write_to(tree_builder);
    jump_double_deref_trace.write_to(tree_builder);
    jump_rel_trace.write_to(tree_builder);
    jump_rel_imm_trace.write_to(tree_builder);
    mul_trace.write_to(tree_builder);
    mul_small_trace.write_to(tree_builder);
    qm31_trace.write_to(tree_builder);
    ret_trace.write_to(tree_builder);
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

/// Parts of the opcodes interaction claim generator for parallel processing.
pub struct OpcodesInteractionParts {
    pub add: Vec<add_opcode::InteractionClaimGenerator>,
    pub add_small: Vec<add_opcode_small::InteractionClaimGenerator>,
    pub add_ap: Vec<add_ap_opcode::InteractionClaimGenerator>,
    pub assert_eq: Vec<assert_eq_opcode::InteractionClaimGenerator>,
    pub assert_eq_imm: Vec<assert_eq_opcode_imm::InteractionClaimGenerator>,
    pub assert_eq_double_deref: Vec<assert_eq_opcode_double_deref::InteractionClaimGenerator>,
    pub blake: Vec<blake_compress_opcode::InteractionClaimGenerator>,
    pub call: Vec<call_opcode_abs::InteractionClaimGenerator>,
    pub call_rel_imm: Vec<call_opcode_rel_imm::InteractionClaimGenerator>,
    pub generic_opcode: Vec<generic_opcode::InteractionClaimGenerator>,
    pub jnz: Vec<jnz_opcode_non_taken::InteractionClaimGenerator>,
    pub jnz_taken: Vec<jnz_opcode_taken::InteractionClaimGenerator>,
    pub jump: Vec<jump_opcode_abs::InteractionClaimGenerator>,
    pub jump_double_deref: Vec<jump_opcode_double_deref::InteractionClaimGenerator>,
    pub jump_rel: Vec<jump_opcode_rel::InteractionClaimGenerator>,
    pub jump_rel_imm: Vec<jump_opcode_rel_imm::InteractionClaimGenerator>,
    pub mul: Vec<mul_opcode::InteractionClaimGenerator>,
    pub mul_small: Vec<mul_opcode_small::InteractionClaimGenerator>,
    pub qm31: Vec<qm_31_add_mul_opcode::InteractionClaimGenerator>,
    pub ret: Vec<ret_opcode::InteractionClaimGenerator>,
}

impl OpcodesInteractionClaimGenerator {
    /// Decompose into individual parts for parallel processing.
    pub fn into_parts(self) -> OpcodesInteractionParts {
        OpcodesInteractionParts {
            add: self.add,
            add_small: self.add_small,
            add_ap: self.add_ap,
            assert_eq: self.assert_eq,
            assert_eq_imm: self.assert_eq_imm,
            assert_eq_double_deref: self.assert_eq_double_deref,
            blake: self.blake,
            call: self.call,
            call_rel_imm: self.call_rel_imm,
            generic_opcode: self.generic_opcode_interaction_gens,
            jnz: self.jnz,
            jnz_taken: self.jnz_taken,
            jump: self.jump,
            jump_double_deref: self.jump_double_deref,
            jump_rel: self.jump_rel,
            jump_rel_imm: self.jump_rel_imm,
            mul: self.mul,
            mul_small: self.mul_small,
            qm31: self.qm31,
            ret: self.ret_interaction_gens,
        }
    }
}

/// Helper struct to hold the result of parallel interaction trace computation.
pub struct InteractionTraceResult<T> {
    pub claims: Vec<T>,
    pub evals: Vec<CollectingTreeBuilder>,
}

impl OpcodesInteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        common_lookup_elements: &CommonLookupElements,
    ) -> OpcodeInteractionClaim {
        // Use Mutex to store results from parallel tasks
        let add_result: Mutex<Option<InteractionTraceResult<_>>> = Mutex::new(None);
        let add_small_result: Mutex<Option<InteractionTraceResult<_>>> = Mutex::new(None);
        let add_ap_result: Mutex<Option<InteractionTraceResult<_>>> = Mutex::new(None);
        let assert_eq_result: Mutex<Option<InteractionTraceResult<_>>> = Mutex::new(None);
        let assert_eq_imm_result: Mutex<Option<InteractionTraceResult<_>>> = Mutex::new(None);
        let assert_eq_double_deref_result: Mutex<Option<InteractionTraceResult<_>>> =
            Mutex::new(None);
        let blake_result: Mutex<Option<InteractionTraceResult<_>>> = Mutex::new(None);
        let call_result: Mutex<Option<InteractionTraceResult<_>>> = Mutex::new(None);
        let call_rel_imm_result: Mutex<Option<InteractionTraceResult<_>>> = Mutex::new(None);
        let generic_opcode_result: Mutex<Option<InteractionTraceResult<_>>> = Mutex::new(None);
        let jnz_result: Mutex<Option<InteractionTraceResult<_>>> = Mutex::new(None);
        let jnz_taken_result: Mutex<Option<InteractionTraceResult<_>>> = Mutex::new(None);
        let jump_result: Mutex<Option<InteractionTraceResult<_>>> = Mutex::new(None);
        let jump_double_deref_result: Mutex<Option<InteractionTraceResult<_>>> = Mutex::new(None);
        let jump_rel_result: Mutex<Option<InteractionTraceResult<_>>> = Mutex::new(None);
        let jump_rel_imm_result: Mutex<Option<InteractionTraceResult<_>>> = Mutex::new(None);
        let mul_result: Mutex<Option<InteractionTraceResult<_>>> = Mutex::new(None);
        let mul_small_result: Mutex<Option<InteractionTraceResult<_>>> = Mutex::new(None);
        let qm31_result: Mutex<Option<InteractionTraceResult<_>>> = Mutex::new(None);
        let ret_result: Mutex<Option<InteractionTraceResult<_>>> = Mutex::new(None);

        // Process all opcode types in parallel using rayon::scope
        rayon::scope(|s| {
            s.spawn(|_| {
                *add_result.lock().unwrap() =
                    Some(process_interaction_gens(self.add, common_lookup_elements));
            });
            s.spawn(|_| {
                *add_small_result.lock().unwrap() = Some(process_interaction_gens(
                    self.add_small,
                    common_lookup_elements,
                ));
            });
            s.spawn(|_| {
                *add_ap_result.lock().unwrap() = Some(process_interaction_gens(
                    self.add_ap,
                    common_lookup_elements,
                ));
            });
            s.spawn(|_| {
                *assert_eq_result.lock().unwrap() = Some(process_interaction_gens(
                    self.assert_eq,
                    common_lookup_elements,
                ));
            });
            s.spawn(|_| {
                *assert_eq_imm_result.lock().unwrap() = Some(process_interaction_gens(
                    self.assert_eq_imm,
                    common_lookup_elements,
                ));
            });
            s.spawn(|_| {
                *assert_eq_double_deref_result.lock().unwrap() = Some(process_interaction_gens(
                    self.assert_eq_double_deref,
                    common_lookup_elements,
                ));
            });
            s.spawn(|_| {
                *blake_result.lock().unwrap() =
                    Some(process_interaction_gens(self.blake, common_lookup_elements));
            });
            s.spawn(|_| {
                *call_result.lock().unwrap() =
                    Some(process_interaction_gens(self.call, common_lookup_elements));
            });
            s.spawn(|_| {
                *call_rel_imm_result.lock().unwrap() = Some(process_interaction_gens(
                    self.call_rel_imm,
                    common_lookup_elements,
                ));
            });
            s.spawn(|_| {
                *generic_opcode_result.lock().unwrap() = Some(process_interaction_gens(
                    self.generic_opcode_interaction_gens,
                    common_lookup_elements,
                ));
            });
            s.spawn(|_| {
                *jnz_result.lock().unwrap() =
                    Some(process_interaction_gens(self.jnz, common_lookup_elements));
            });
            s.spawn(|_| {
                *jnz_taken_result.lock().unwrap() = Some(process_interaction_gens(
                    self.jnz_taken,
                    common_lookup_elements,
                ));
            });
            s.spawn(|_| {
                *jump_result.lock().unwrap() =
                    Some(process_interaction_gens(self.jump, common_lookup_elements));
            });
            s.spawn(|_| {
                *jump_double_deref_result.lock().unwrap() = Some(process_interaction_gens(
                    self.jump_double_deref,
                    common_lookup_elements,
                ));
            });
            s.spawn(|_| {
                *jump_rel_result.lock().unwrap() = Some(process_interaction_gens(
                    self.jump_rel,
                    common_lookup_elements,
                ));
            });
            s.spawn(|_| {
                *jump_rel_imm_result.lock().unwrap() = Some(process_interaction_gens(
                    self.jump_rel_imm,
                    common_lookup_elements,
                ));
            });
            s.spawn(|_| {
                *mul_result.lock().unwrap() =
                    Some(process_interaction_gens(self.mul, common_lookup_elements));
            });
            s.spawn(|_| {
                *mul_small_result.lock().unwrap() = Some(process_interaction_gens(
                    self.mul_small,
                    common_lookup_elements,
                ));
            });
            s.spawn(|_| {
                *qm31_result.lock().unwrap() =
                    Some(process_interaction_gens(self.qm31, common_lookup_elements));
            });
            s.spawn(|_| {
                *ret_result.lock().unwrap() = Some(process_interaction_gens(
                    self.ret_interaction_gens,
                    common_lookup_elements,
                ));
            });
        });

        // Extract results from mutexes
        let add_result = add_result.into_inner().unwrap().unwrap();
        let add_small_result = add_small_result.into_inner().unwrap().unwrap();
        let add_ap_result = add_ap_result.into_inner().unwrap().unwrap();
        let assert_eq_result = assert_eq_result.into_inner().unwrap().unwrap();
        let assert_eq_imm_result = assert_eq_imm_result.into_inner().unwrap().unwrap();
        let assert_eq_double_deref_result =
            assert_eq_double_deref_result.into_inner().unwrap().unwrap();
        let blake_result = blake_result.into_inner().unwrap().unwrap();
        let call_result = call_result.into_inner().unwrap().unwrap();
        let call_rel_imm_result = call_rel_imm_result.into_inner().unwrap().unwrap();
        let generic_opcode_result = generic_opcode_result.into_inner().unwrap().unwrap();
        let jnz_result = jnz_result.into_inner().unwrap().unwrap();
        let jnz_taken_result = jnz_taken_result.into_inner().unwrap().unwrap();
        let jump_result = jump_result.into_inner().unwrap().unwrap();
        let jump_double_deref_result = jump_double_deref_result.into_inner().unwrap().unwrap();
        let jump_rel_result = jump_rel_result.into_inner().unwrap().unwrap();
        let jump_rel_imm_result = jump_rel_imm_result.into_inner().unwrap().unwrap();
        let mul_result = mul_result.into_inner().unwrap().unwrap();
        let mul_small_result = mul_small_result.into_inner().unwrap().unwrap();
        let qm31_result = qm31_result.into_inner().unwrap().unwrap();
        let ret_result = ret_result.into_inner().unwrap().unwrap();

        // Sequentially extend the tree builder with all collected evaluations in deterministic
        // order
        for builder in add_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in add_small_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in add_ap_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in assert_eq_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in assert_eq_imm_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in assert_eq_double_deref_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in blake_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in call_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in call_rel_imm_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in generic_opcode_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in jnz_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in jnz_taken_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in jump_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in jump_double_deref_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in jump_rel_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in jump_rel_imm_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in mul_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in mul_small_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in qm31_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in ret_result.evals {
            builder.write_to(tree_builder);
        }

        OpcodeInteractionClaim {
            add: add_result.claims,
            add_small: add_small_result.claims,
            add_ap: add_ap_result.claims,
            assert_eq: assert_eq_result.claims,
            assert_eq_imm: assert_eq_imm_result.claims,
            assert_eq_double_deref: assert_eq_double_deref_result.claims,
            blake: blake_result.claims,
            call: call_result.claims,
            call_rel_imm: call_rel_imm_result.claims,
            generic: generic_opcode_result.claims,
            jnz: jnz_result.claims,
            jnz_taken: jnz_taken_result.claims,
            jump: jump_result.claims,
            jump_double_deref: jump_double_deref_result.claims,
            jump_rel: jump_rel_result.claims,
            jump_rel_imm: jump_rel_imm_result.claims,
            mul: mul_result.claims,
            mul_small: mul_small_result.claims,
            qm31: qm31_result.claims,
            ret: ret_result.claims,
        }
    }
}

/// Helper trait for interaction generators that can write interaction traces.
pub trait InteractionGen: Send {
    type Claim;
    fn write_interaction_trace(
        self,
        tree_builder: &mut CollectingTreeBuilder,
        common_lookup_elements: &CommonLookupElements,
    ) -> Self::Claim;
}

/// Process a vector of interaction generators in parallel, collecting their results.
pub fn process_interaction_gens<G>(
    gens: Vec<G>,
    common_lookup_elements: &CommonLookupElements,
) -> InteractionTraceResult<G::Claim>
where
    G: InteractionGen,
    G::Claim: Send,
{
    let results: Vec<_> = gens
        .into_par_iter()
        .map(|gen| {
            let mut builder = CollectingTreeBuilder::new();
            let claim = gen.write_interaction_trace(&mut builder, common_lookup_elements);
            (claim, builder)
        })
        .collect();

    let (claims, evals): (Vec<_>, Vec<_>) = results.into_iter().unzip();
    InteractionTraceResult { claims, evals }
}

// Implement InteractionGen for each opcode's InteractionClaimGenerator
macro_rules! impl_interaction_gen {
    ($module:ident) => {
        impl InteractionGen for $module::InteractionClaimGenerator {
            type Claim = cairo_air::components::$module::InteractionClaim;
            fn write_interaction_trace(
                self,
                tree_builder: &mut CollectingTreeBuilder,
                common_lookup_elements: &CommonLookupElements,
            ) -> Self::Claim {
                self.write_interaction_trace(tree_builder, common_lookup_elements)
            }
        }
    };
}

impl_interaction_gen!(add_opcode);
impl_interaction_gen!(add_opcode_small);
impl_interaction_gen!(add_ap_opcode);
impl_interaction_gen!(assert_eq_opcode);
impl_interaction_gen!(assert_eq_opcode_imm);
impl_interaction_gen!(assert_eq_opcode_double_deref);
impl_interaction_gen!(blake_compress_opcode);
impl_interaction_gen!(call_opcode_abs);
impl_interaction_gen!(call_opcode_rel_imm);
impl_interaction_gen!(generic_opcode);
impl_interaction_gen!(jnz_opcode_non_taken);
impl_interaction_gen!(jnz_opcode_taken);
impl_interaction_gen!(jump_opcode_abs);
impl_interaction_gen!(jump_opcode_double_deref);
impl_interaction_gen!(jump_opcode_rel);
impl_interaction_gen!(jump_opcode_rel_imm);
impl_interaction_gen!(mul_opcode);
impl_interaction_gen!(mul_opcode_small);
impl_interaction_gen!(qm_31_add_mul_opcode);
impl_interaction_gen!(ret_opcode);
