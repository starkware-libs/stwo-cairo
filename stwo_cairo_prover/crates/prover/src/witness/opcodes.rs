use cairo_air::components::{
    add_ap_opcode as add_ap_opcode_air, add_opcode as add_opcode_air,
    add_opcode_small as add_opcode_small_air, assert_eq_opcode as assert_eq_opcode_air,
    assert_eq_opcode_double_deref as assert_eq_opcode_double_deref_air,
    assert_eq_opcode_imm as assert_eq_opcode_imm_air,
    blake_compress_opcode as blake_compress_opcode_air, call_opcode_abs as call_opcode_abs_air,
    call_opcode_rel_imm as call_opcode_rel_imm_air, generic_opcode as generic_opcode_air,
    jnz_opcode_non_taken as jnz_opcode_non_taken_air, jnz_opcode_taken as jnz_opcode_taken_air,
    jump_opcode_abs as jump_opcode_abs_air,
    jump_opcode_double_deref as jump_opcode_double_deref_air,
    jump_opcode_rel as jump_opcode_rel_air, jump_opcode_rel_imm as jump_opcode_rel_imm_air,
    mul_opcode as mul_opcode_air, mul_opcode_small as mul_opcode_small_air,
    qm_31_add_mul_opcode as qm_31_add_mul_opcode_air, ret_opcode as ret_opcode_air,
};
use cairo_air::opcodes_air::OpcodeClaim;
use stwo::core::fields::m31::BaseField;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::poly::circle::CircleEvaluation;
use stwo::prover::poly::BitReversedOrder;
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
use crate::witness::utils::TreeBuilder;

/// Type alias for trace evaluations.
pub type TraceEval = Vec<CircleEvaluation<SimdBackend, BaseField, BitReversedOrder>>;

/// Holds the generated traces for all opcodes.
/// Traces are stored separately to allow parallel generation before sequential extension
/// to the tree_builder.
#[derive(Default)]
pub struct OpcodeTraces {
    pub add: Vec<TraceEval>,
    pub add_small: Vec<TraceEval>,
    pub add_ap: Vec<TraceEval>,
    pub assert_eq: Vec<TraceEval>,
    pub assert_eq_imm: Vec<TraceEval>,
    pub assert_eq_double_deref: Vec<TraceEval>,
    pub blake: Vec<TraceEval>,
    pub call: Vec<TraceEval>,
    pub call_rel_imm: Vec<TraceEval>,
    pub generic: Vec<TraceEval>,
    pub jnz: Vec<TraceEval>,
    pub jnz_taken: Vec<TraceEval>,
    pub jump: Vec<TraceEval>,
    pub jump_double_deref: Vec<TraceEval>,
    pub jump_rel: Vec<TraceEval>,
    pub jump_rel_imm: Vec<TraceEval>,
    pub mul: Vec<TraceEval>,
    pub mul_small: Vec<TraceEval>,
    pub qm31: Vec<TraceEval>,
    pub ret: Vec<TraceEval>,
}

/// Result of generating pre-blake opcode traces.
/// Contains the traces, claims, and interaction generators for opcodes before blake.
pub struct PreBlakeOpcodeResult {
    pub traces: PreBlakeOpcodeTraces,
    pub claims: PreBlakeOpcodeClaims,
    pub interaction_gens: PreBlakeOpcodeInteractionGens,
}

/// Traces for opcodes that come before blake.
#[derive(Default)]
pub struct PreBlakeOpcodeTraces {
    pub add: Vec<TraceEval>,
    pub add_small: Vec<TraceEval>,
    pub add_ap: Vec<TraceEval>,
    pub assert_eq: Vec<TraceEval>,
    pub assert_eq_imm: Vec<TraceEval>,
    pub assert_eq_double_deref: Vec<TraceEval>,
}

/// Claims for opcodes that come before blake.
#[derive(Default)]
pub struct PreBlakeOpcodeClaims {
    pub add: Vec<add_opcode_air::Claim>,
    pub add_small: Vec<add_opcode_small_air::Claim>,
    pub add_ap: Vec<add_ap_opcode_air::Claim>,
    pub assert_eq: Vec<assert_eq_opcode_air::Claim>,
    pub assert_eq_imm: Vec<assert_eq_opcode_imm_air::Claim>,
    pub assert_eq_double_deref: Vec<assert_eq_opcode_double_deref_air::Claim>,
}

/// Interaction generators for opcodes that come before blake.
#[derive(Default)]
pub struct PreBlakeOpcodeInteractionGens {
    pub add: Vec<add_opcode::InteractionClaimGenerator>,
    pub add_small: Vec<add_opcode_small::InteractionClaimGenerator>,
    pub add_ap: Vec<add_ap_opcode::InteractionClaimGenerator>,
    pub assert_eq: Vec<assert_eq_opcode::InteractionClaimGenerator>,
    pub assert_eq_imm: Vec<assert_eq_opcode_imm::InteractionClaimGenerator>,
    pub assert_eq_double_deref: Vec<assert_eq_opcode_double_deref::InteractionClaimGenerator>,
}

/// Result of generating blake and post-blake opcode traces.
pub struct BlakeAndPostOpcodeResult {
    pub traces: BlakeAndPostOpcodeTraces,
    pub claims: BlakeAndPostOpcodeClaims,
    pub interaction_gens: BlakeAndPostOpcodeInteractionGens,
}

/// Traces for blake and opcodes that come after blake.
#[derive(Default)]
pub struct BlakeAndPostOpcodeTraces {
    pub blake: Vec<TraceEval>,
    pub call: Vec<TraceEval>,
    pub call_rel_imm: Vec<TraceEval>,
    pub generic: Vec<TraceEval>,
    pub jnz: Vec<TraceEval>,
    pub jnz_taken: Vec<TraceEval>,
    pub jump: Vec<TraceEval>,
    pub jump_double_deref: Vec<TraceEval>,
    pub jump_rel: Vec<TraceEval>,
    pub jump_rel_imm: Vec<TraceEval>,
    pub mul: Vec<TraceEval>,
    pub mul_small: Vec<TraceEval>,
    pub qm31: Vec<TraceEval>,
    pub ret: Vec<TraceEval>,
}

/// Claims for blake and opcodes that come after blake.
#[derive(Default)]
pub struct BlakeAndPostOpcodeClaims {
    pub blake: Vec<blake_compress_opcode_air::Claim>,
    pub call: Vec<call_opcode_abs_air::Claim>,
    pub call_rel_imm: Vec<call_opcode_rel_imm_air::Claim>,
    pub generic: Vec<generic_opcode_air::Claim>,
    pub jnz: Vec<jnz_opcode_non_taken_air::Claim>,
    pub jnz_taken: Vec<jnz_opcode_taken_air::Claim>,
    pub jump: Vec<jump_opcode_abs_air::Claim>,
    pub jump_double_deref: Vec<jump_opcode_double_deref_air::Claim>,
    pub jump_rel: Vec<jump_opcode_rel_air::Claim>,
    pub jump_rel_imm: Vec<jump_opcode_rel_imm_air::Claim>,
    pub mul: Vec<mul_opcode_air::Claim>,
    pub mul_small: Vec<mul_opcode_small_air::Claim>,
    pub qm31: Vec<qm_31_add_mul_opcode_air::Claim>,
    pub ret: Vec<ret_opcode_air::Claim>,
}

/// Interaction generators for blake and opcodes that come after blake.
#[derive(Default)]
pub struct BlakeAndPostOpcodeInteractionGens {
    pub blake: Vec<blake_compress_opcode::InteractionClaimGenerator>,
    pub call: Vec<call_opcode_abs::InteractionClaimGenerator>,
    pub call_rel_imm: Vec<call_opcode_rel_imm::InteractionClaimGenerator>,
    pub generic: Vec<generic_opcode::InteractionClaimGenerator>,
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

/// Generates traces for opcodes that come BEFORE blake.
/// These can be run in parallel with gen_trace(preprocessed_trace) since they have no
/// dependency on blake components which require &mut.
pub fn pre_blake_opcodes_generate_traces(
    add: Option<add_opcode::ClaimGenerator>,
    add_small: Option<add_opcode_small::ClaimGenerator>,
    add_ap: Option<add_ap_opcode::ClaimGenerator>,
    assert_eq: Option<assert_eq_opcode::ClaimGenerator>,
    assert_eq_imm: Option<assert_eq_opcode_imm::ClaimGenerator>,
    assert_eq_double_deref: Option<assert_eq_opcode_double_deref::ClaimGenerator>,
    memory_address_to_id_trace_generator: &memory_address_to_id::ClaimGenerator,
    memory_id_to_value_trace_generator: &memory_id_to_big::ClaimGenerator,
    rc_11_trace_generator: &range_check_11::ClaimGenerator,
    rc_18_trace_generator: &range_check_18::ClaimGenerator,
    verify_instruction_trace_generator: &verify_instruction::ClaimGenerator,
) -> PreBlakeOpcodeResult {
    let mut traces = PreBlakeOpcodeTraces::default();
    let mut claims = PreBlakeOpcodeClaims::default();
    let mut interaction_gens = PreBlakeOpcodeInteractionGens::default();

    if let Some(gen) = add {
        let (trace, claim, interaction_gen) = gen.write_trace(
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            verify_instruction_trace_generator,
        );
        traces.add = vec![trace.to_evals()];
        claims.add = vec![claim];
        interaction_gens.add = vec![interaction_gen];
    }

    if let Some(gen) = add_small {
        let (trace, claim, interaction_gen) = gen.write_trace(
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            verify_instruction_trace_generator,
        );
        traces.add_small = vec![trace.to_evals()];
        claims.add_small = vec![claim];
        interaction_gens.add_small = vec![interaction_gen];
    }

    if let Some(gen) = add_ap {
        let (trace, claim, interaction_gen) = gen.write_trace(
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            verify_instruction_trace_generator,
            rc_18_trace_generator,
            rc_11_trace_generator,
        );
        traces.add_ap = vec![trace.to_evals()];
        claims.add_ap = vec![claim];
        interaction_gens.add_ap = vec![interaction_gen];
    }

    if let Some(gen) = assert_eq {
        let (trace, claim, interaction_gen) = gen.write_trace(
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            verify_instruction_trace_generator,
        );
        traces.assert_eq = vec![trace.to_evals()];
        claims.assert_eq = vec![claim];
        interaction_gens.assert_eq = vec![interaction_gen];
    }

    if let Some(gen) = assert_eq_imm {
        let (trace, claim, interaction_gen) = gen.write_trace(
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            verify_instruction_trace_generator,
        );
        traces.assert_eq_imm = vec![trace.to_evals()];
        claims.assert_eq_imm = vec![claim];
        interaction_gens.assert_eq_imm = vec![interaction_gen];
    }

    if let Some(gen) = assert_eq_double_deref {
        let (trace, claim, interaction_gen) = gen.write_trace(
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            verify_instruction_trace_generator,
        );
        traces.assert_eq_double_deref = vec![trace.to_evals()];
        claims.assert_eq_double_deref = vec![claim];
        interaction_gens.assert_eq_double_deref = vec![interaction_gen];
    }

    PreBlakeOpcodeResult {
        traces,
        claims,
        interaction_gens,
    }
}

/// Generates traces for blake and opcodes that come AFTER blake.
/// These must be run sequentially because blake requires &mut access to blake_round and
/// triple_xor_32.
#[allow(clippy::too_many_arguments)]
pub fn blake_and_post_opcodes_generate_traces(
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
    blake_round: &mut Option<blake_round::ClaimGenerator>,
    triple_xor_32: &mut Option<triple_xor_32::ClaimGenerator>,
    memory_address_to_id_trace_generator: &memory_address_to_id::ClaimGenerator,
    memory_id_to_value_trace_generator: &memory_id_to_big::ClaimGenerator,
    rc_7_2_5_trace_generator: &range_check_7_2_5::ClaimGenerator,
    rc_11_trace_generator: &range_check_11::ClaimGenerator,
    rc_18_trace_generator: &range_check_18::ClaimGenerator,
    rc_20_trace_generator: &range_check_20::ClaimGenerator,
    rc_4_4_4_4_trace_generator: &range_check_4_4_4_4::ClaimGenerator,
    rc_9_9_trace_generator: &range_check_9_9::ClaimGenerator,
    verify_instruction_trace_generator: &verify_instruction::ClaimGenerator,
    verify_bitwise_xor_8_trace_generator: &mut verify_bitwise_xor_8::ClaimGenerator,
) -> BlakeAndPostOpcodeResult {
    let mut traces = BlakeAndPostOpcodeTraces::default();
    let mut claims = BlakeAndPostOpcodeClaims::default();
    let mut interaction_gens = BlakeAndPostOpcodeInteractionGens::default();

    if let Some(gen) = blake {
        let (trace, claim, interaction_gen) = gen.write_trace(
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            verify_instruction_trace_generator,
            rc_7_2_5_trace_generator,
            verify_bitwise_xor_8_trace_generator,
            blake_round.as_mut().unwrap(),
            triple_xor_32.as_mut().unwrap(),
        );
        traces.blake = vec![trace.to_evals()];
        claims.blake = vec![claim];
        interaction_gens.blake = vec![interaction_gen];
    }

    if let Some(gen) = call {
        let (trace, claim, interaction_gen) = gen.write_trace(
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            verify_instruction_trace_generator,
        );
        traces.call = vec![trace.to_evals()];
        claims.call = vec![claim];
        interaction_gens.call = vec![interaction_gen];
    }

    if let Some(gen) = call_rel_imm {
        let (trace, claim, interaction_gen) = gen.write_trace(
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            verify_instruction_trace_generator,
        );
        traces.call_rel_imm = vec![trace.to_evals()];
        claims.call_rel_imm = vec![claim];
        interaction_gens.call_rel_imm = vec![interaction_gen];
    }

    if let Some(gen) = generic {
        let (trace, claim, interaction_gen) = gen.write_trace(
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            verify_instruction_trace_generator,
            rc_9_9_trace_generator,
            rc_20_trace_generator,
            rc_18_trace_generator,
            rc_11_trace_generator,
        );
        traces.generic = vec![trace.to_evals()];
        claims.generic = vec![claim];
        interaction_gens.generic = vec![interaction_gen];
    }

    if let Some(gen) = jnz {
        let (trace, claim, interaction_gen) = gen.write_trace(
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            verify_instruction_trace_generator,
        );
        traces.jnz = vec![trace.to_evals()];
        claims.jnz = vec![claim];
        interaction_gens.jnz = vec![interaction_gen];
    }

    if let Some(gen) = jnz_taken {
        let (trace, claim, interaction_gen) = gen.write_trace(
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            verify_instruction_trace_generator,
        );
        traces.jnz_taken = vec![trace.to_evals()];
        claims.jnz_taken = vec![claim];
        interaction_gens.jnz_taken = vec![interaction_gen];
    }

    if let Some(gen) = jump {
        let (trace, claim, interaction_gen) = gen.write_trace(
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            verify_instruction_trace_generator,
        );
        traces.jump = vec![trace.to_evals()];
        claims.jump = vec![claim];
        interaction_gens.jump = vec![interaction_gen];
    }

    if let Some(gen) = jump_double_deref {
        let (trace, claim, interaction_gen) = gen.write_trace(
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            verify_instruction_trace_generator,
        );
        traces.jump_double_deref = vec![trace.to_evals()];
        claims.jump_double_deref = vec![claim];
        interaction_gens.jump_double_deref = vec![interaction_gen];
    }

    if let Some(gen) = jump_rel {
        let (trace, claim, interaction_gen) = gen.write_trace(
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            verify_instruction_trace_generator,
        );
        traces.jump_rel = vec![trace.to_evals()];
        claims.jump_rel = vec![claim];
        interaction_gens.jump_rel = vec![interaction_gen];
    }

    if let Some(gen) = jump_rel_imm {
        let (trace, claim, interaction_gen) = gen.write_trace(
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            verify_instruction_trace_generator,
        );
        traces.jump_rel_imm = vec![trace.to_evals()];
        claims.jump_rel_imm = vec![claim];
        interaction_gens.jump_rel_imm = vec![interaction_gen];
    }

    if let Some(gen) = mul {
        let (trace, claim, interaction_gen) = gen.write_trace(
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            verify_instruction_trace_generator,
            rc_20_trace_generator,
        );
        traces.mul = vec![trace.to_evals()];
        claims.mul = vec![claim];
        interaction_gens.mul = vec![interaction_gen];
    }

    if let Some(gen) = mul_small {
        let (trace, claim, interaction_gen) = gen.write_trace(
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            verify_instruction_trace_generator,
            rc_11_trace_generator,
        );
        traces.mul_small = vec![trace.to_evals()];
        claims.mul_small = vec![claim];
        interaction_gens.mul_small = vec![interaction_gen];
    }

    if let Some(gen) = qm31 {
        let (trace, claim, interaction_gen) = gen.write_trace(
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            verify_instruction_trace_generator,
            rc_4_4_4_4_trace_generator,
        );
        traces.qm31 = vec![trace.to_evals()];
        claims.qm31 = vec![claim];
        interaction_gens.qm31 = vec![interaction_gen];
    }

    if let Some(gen) = ret {
        let (trace, claim, interaction_gen) = gen.write_trace(
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            verify_instruction_trace_generator,
        );
        traces.ret = vec![trace.to_evals()];
        claims.ret = vec![claim];
        interaction_gens.ret = vec![interaction_gen];
    }

    BlakeAndPostOpcodeResult {
        traces,
        claims,
        interaction_gens,
    }
}

/// Extends the tree builder with opcode traces in the correct order.
pub fn extend_opcode_traces(
    tree_builder: &mut impl TreeBuilder<SimdBackend>,
    pre_blake: PreBlakeOpcodeTraces,
    blake_and_post: BlakeAndPostOpcodeTraces,
) {
    // Pre-blake opcodes (in order)
    for trace in pre_blake.add {
        tree_builder.extend_evals(trace);
    }
    for trace in pre_blake.add_small {
        tree_builder.extend_evals(trace);
    }
    for trace in pre_blake.add_ap {
        tree_builder.extend_evals(trace);
    }
    for trace in pre_blake.assert_eq {
        tree_builder.extend_evals(trace);
    }
    for trace in pre_blake.assert_eq_imm {
        tree_builder.extend_evals(trace);
    }
    for trace in pre_blake.assert_eq_double_deref {
        tree_builder.extend_evals(trace);
    }

    // Blake and post-blake opcodes (in order)
    for trace in blake_and_post.blake {
        tree_builder.extend_evals(trace);
    }
    for trace in blake_and_post.call {
        tree_builder.extend_evals(trace);
    }
    for trace in blake_and_post.call_rel_imm {
        tree_builder.extend_evals(trace);
    }
    for trace in blake_and_post.generic {
        tree_builder.extend_evals(trace);
    }
    for trace in blake_and_post.jnz {
        tree_builder.extend_evals(trace);
    }
    for trace in blake_and_post.jnz_taken {
        tree_builder.extend_evals(trace);
    }
    for trace in blake_and_post.jump {
        tree_builder.extend_evals(trace);
    }
    for trace in blake_and_post.jump_double_deref {
        tree_builder.extend_evals(trace);
    }
    for trace in blake_and_post.jump_rel {
        tree_builder.extend_evals(trace);
    }
    for trace in blake_and_post.jump_rel_imm {
        tree_builder.extend_evals(trace);
    }
    for trace in blake_and_post.mul {
        tree_builder.extend_evals(trace);
    }
    for trace in blake_and_post.mul_small {
        tree_builder.extend_evals(trace);
    }
    for trace in blake_and_post.qm31 {
        tree_builder.extend_evals(trace);
    }
    for trace in blake_and_post.ret {
        tree_builder.extend_evals(trace);
    }
}

/// Combines pre-blake and blake/post results into OpcodeClaim and OpcodesInteractionClaimGenerator.
pub fn combine_opcode_results(
    pre_blake_claims: PreBlakeOpcodeClaims,
    pre_blake_interaction_gens: PreBlakeOpcodeInteractionGens,
    blake_and_post_claims: BlakeAndPostOpcodeClaims,
    blake_and_post_interaction_gens: BlakeAndPostOpcodeInteractionGens,
) -> (OpcodeClaim, OpcodesInteractionClaimGenerator) {
    (
        OpcodeClaim {
            add: pre_blake_claims.add,
            add_small: pre_blake_claims.add_small,
            add_ap: pre_blake_claims.add_ap,
            assert_eq: pre_blake_claims.assert_eq,
            assert_eq_imm: pre_blake_claims.assert_eq_imm,
            assert_eq_double_deref: pre_blake_claims.assert_eq_double_deref,
            blake: blake_and_post_claims.blake,
            call: blake_and_post_claims.call,
            call_rel_imm: blake_and_post_claims.call_rel_imm,
            generic: blake_and_post_claims.generic,
            jnz: blake_and_post_claims.jnz,
            jnz_taken: blake_and_post_claims.jnz_taken,
            jump: blake_and_post_claims.jump,
            jump_double_deref: blake_and_post_claims.jump_double_deref,
            jump_rel: blake_and_post_claims.jump_rel,
            jump_rel_imm: blake_and_post_claims.jump_rel_imm,
            mul: blake_and_post_claims.mul,
            mul_small: blake_and_post_claims.mul_small,
            qm31: blake_and_post_claims.qm31,
            ret: blake_and_post_claims.ret,
        },
        OpcodesInteractionClaimGenerator {
            add: pre_blake_interaction_gens.add,
            add_small: pre_blake_interaction_gens.add_small,
            add_ap: pre_blake_interaction_gens.add_ap,
            assert_eq: pre_blake_interaction_gens.assert_eq,
            assert_eq_imm: pre_blake_interaction_gens.assert_eq_imm,
            assert_eq_double_deref: pre_blake_interaction_gens.assert_eq_double_deref,
            blake: blake_and_post_interaction_gens.blake,
            call: blake_and_post_interaction_gens.call,
            call_rel_imm: blake_and_post_interaction_gens.call_rel_imm,
            generic_opcode_interaction_gens: blake_and_post_interaction_gens.generic,
            jnz: blake_and_post_interaction_gens.jnz,
            jnz_taken: blake_and_post_interaction_gens.jnz_taken,
            jump: blake_and_post_interaction_gens.jump,
            jump_double_deref: blake_and_post_interaction_gens.jump_double_deref,
            jump_rel: blake_and_post_interaction_gens.jump_rel,
            jump_rel_imm: blake_and_post_interaction_gens.jump_rel_imm,
            mul: blake_and_post_interaction_gens.mul,
            mul_small: blake_and_post_interaction_gens.mul_small,
            qm31: blake_and_post_interaction_gens.qm31,
            ret_interaction_gens: blake_and_post_interaction_gens.ret,
        },
    )
}

/// Legacy function that generates all opcode traces and extends them to the tree_builder.
/// Maintained for backward compatibility. Uses the new split functions internally.
#[allow(clippy::too_many_arguments)]
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
    let memory_address_to_id = memory_address_to_id_trace_generator.unwrap();
    let memory_id_to_value = memory_id_to_value_trace_generator.unwrap();
    let rc_11 = rc_11_trace_generator.unwrap();
    let rc_18 = rc_18_trace_generator.unwrap();
    let verify_instruction = verify_instruction_trace_generator.unwrap();

    // Generate pre-blake opcode traces
    let pre_blake_result = pre_blake_opcodes_generate_traces(
        add,
        add_small,
        add_ap,
        assert_eq,
        assert_eq_imm,
        assert_eq_double_deref,
        memory_address_to_id,
        memory_id_to_value,
        rc_11,
        rc_18,
        verify_instruction,
    );

    // Generate blake and post-blake opcode traces
    let blake_and_post_result = blake_and_post_opcodes_generate_traces(
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
        blake_round,
        triple_xor_32,
        memory_address_to_id,
        memory_id_to_value,
        rc_7_2_5_trace_generator.unwrap(),
        rc_11,
        rc_18,
        rc_20_trace_generator.unwrap(),
        rc_4_4_4_4_trace_generator.unwrap(),
        rc_9_9_trace_generator.unwrap(),
        verify_instruction,
        verify_bitwise_xor_8_trace_generator.unwrap(),
    );

    // Extend tree_builder with all traces in order
    extend_opcode_traces(
        tree_builder,
        pre_blake_result.traces,
        blake_and_post_result.traces,
    );

    // Combine results
    combine_opcode_results(
        pre_blake_result.claims,
        pre_blake_result.interaction_gens,
        blake_and_post_result.claims,
        blake_and_post_result.interaction_gens,
    )
}

pub struct OpcodesInteractionClaimGenerator {
    pub add: Vec<add_opcode::InteractionClaimGenerator>,
    pub add_small: Vec<add_opcode_small::InteractionClaimGenerator>,
    pub add_ap: Vec<add_ap_opcode::InteractionClaimGenerator>,
    pub assert_eq: Vec<assert_eq_opcode::InteractionClaimGenerator>,
    pub assert_eq_imm: Vec<assert_eq_opcode_imm::InteractionClaimGenerator>,
    pub assert_eq_double_deref: Vec<assert_eq_opcode_double_deref::InteractionClaimGenerator>,
    pub blake: Vec<blake_compress_opcode::InteractionClaimGenerator>,
    pub call: Vec<call_opcode_abs::InteractionClaimGenerator>,
    pub call_rel_imm: Vec<call_opcode_rel_imm::InteractionClaimGenerator>,
    pub generic_opcode_interaction_gens: Vec<generic_opcode::InteractionClaimGenerator>,
    pub jnz: Vec<jnz_opcode_non_taken::InteractionClaimGenerator>,
    pub jnz_taken: Vec<jnz_opcode_taken::InteractionClaimGenerator>,
    pub jump: Vec<jump_opcode_abs::InteractionClaimGenerator>,
    pub jump_double_deref: Vec<jump_opcode_double_deref::InteractionClaimGenerator>,
    pub jump_rel: Vec<jump_opcode_rel::InteractionClaimGenerator>,
    pub jump_rel_imm: Vec<jump_opcode_rel_imm::InteractionClaimGenerator>,
    pub mul: Vec<mul_opcode::InteractionClaimGenerator>,
    pub mul_small: Vec<mul_opcode_small::InteractionClaimGenerator>,
    pub qm31: Vec<qm_31_add_mul_opcode::InteractionClaimGenerator>,
    pub ret_interaction_gens: Vec<ret_opcode::InteractionClaimGenerator>,
}
