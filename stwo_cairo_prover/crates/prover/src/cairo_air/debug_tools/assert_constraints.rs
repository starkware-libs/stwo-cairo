// TODO(Ohad):remove allow.
#![allow(unused)]
use std::ops::Deref;

use itertools::Itertools;
use num_traits::Zero;
use stwo_cairo_adapter::ProverInput;
use stwo_prover::constraint_framework::{
    assert_constraints_on_trace, FrameworkComponent, FrameworkEval, PREPROCESSED_TRACE_IDX,
};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::Blake2sChannel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::pcs::{CommitmentSchemeProver, PcsConfig, TreeVec};
use stwo_prover::core::poly::circle::{CanonicCoset, CirclePoly, PolyOps};
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

use crate::cairo_air::air::{
    lookup_sum, CairoClaimGenerator, CairoComponents, CairoInteractionElements,
};
use crate::cairo_air::debug_tools::mock_tree_builder::MockCommitmentScheme;
use crate::cairo_air::opcodes_air::OpcodeComponents;
use crate::cairo_air::preprocessed::{PreProcessedColumn, PreProcessedTrace};
use crate::cairo_air::prover::LOG_MAX_ROWS;

pub fn assert_component<E: FrameworkEval + Sync>(
    component: &FrameworkComponent<E>,
    trace: &TreeVec<Vec<&Vec<M31>>>,
) {
    let mut component_trace = trace
        .sub_tree(component.trace_locations())
        .map(|tree| tree.into_iter().cloned().collect_vec());
    component_trace[PREPROCESSED_TRACE_IDX] = component
        .preproccessed_column_indices()
        .iter()
        .map(|idx| trace[PREPROCESSED_TRACE_IDX][*idx])
        .collect();

    let log_size = component.log_size();

    let component_eval = component.deref();
    assert_constraints_on_trace(
        &component_trace,
        log_size,
        |eval| {
            component_eval.evaluate(eval);
        },
        component.claimed_sum(),
    );
}

// Asserts that all Cairo AIR constraints are satisfied for the given trace and components.
//
// # Parameters
//
// * `trace` - The evaluated polynomials on the trace domain.
// * `cairo_components` - The components constraints to check.
fn assert_cairo_components(trace: TreeVec<Vec<&Vec<M31>>>, cairo_components: &CairoComponents) {
    let CairoComponents {
        opcodes,
        verify_instruction,
        blake_context,
        builtins,
        poseidon_context,
        memory_address_to_id,
        memory_id_to_value,
        range_checks,
        verify_bitwise_xor_4,
        verify_bitwise_xor_7,
        verify_bitwise_xor_8,
        verify_bitwise_xor_9,
    } = cairo_components;
    let OpcodeComponents {
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
        blake,
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
    } = opcodes;
    assert_many(add, &trace);
    assert_many(add_imm, &trace);
    assert_many(add_small, &trace);
    assert_many(add_small_imm, &trace);
    assert_many(add_ap, &trace);
    assert_many(add_ap_op_1_base_fp, &trace);
    assert_many(add_ap_imm, &trace);
    assert_many(assert_eq, &trace);
    assert_many(assert_eq_imm, &trace);
    assert_many(assert_eq_double_deref, &trace);
    assert_many(blake, &trace);
    assert_many(call, &trace);
    assert_many(call_op_1_base_fp, &trace);
    assert_many(call_rel, &trace);
    assert_many(generic, &trace);
    assert_many(jnz, &trace);
    assert_many(jnz_dst_base_fp, &trace);
    assert_many(jnz_taken, &trace);
    assert_many(jnz_taken_dst_base_fp, &trace);
    assert_many(jump, &trace);
    assert_many(jump_double_deref, &trace);
    assert_many(jump_rel, &trace);
    assert_many(jump_rel_imm, &trace);
    assert_many(mul, &trace);
    assert_many(mul_imm, &trace);
    assert_many(mul_small, &trace);
    assert_many(mul_small_imm, &trace);
    assert_many(qm31, &trace);
    assert_many(ret, &trace);

    assert_component(verify_instruction, &trace);
    assert_component(&range_checks.rc_6, &trace);
    assert_component(&range_checks.rc_8, &trace);
    assert_component(&range_checks.rc_11, &trace);
    assert_component(&range_checks.rc_12, &trace);
    assert_component(&range_checks.rc_18, &trace);
    assert_component(&range_checks.rc_19, &trace);
    assert_component(&range_checks.rc_3_6, &trace);
    assert_component(&range_checks.rc_4_3, &trace);
    assert_component(&range_checks.rc_4_4, &trace);
    assert_component(&range_checks.rc_5_4, &trace);
    assert_component(&range_checks.rc_9_9, &trace);
    assert_component(&range_checks.rc_7_2_5, &trace);
    assert_component(&range_checks.rc_3_6_6_3, &trace);
    assert_component(&range_checks.rc_4_4_4_4, &trace);
    assert_component(&range_checks.rc_3_3_3_3_3, &trace);
    assert_component(verify_bitwise_xor_4, &trace);
    assert_component(verify_bitwise_xor_7, &trace);
    assert_component(verify_bitwise_xor_8, &trace);
    assert_component(verify_bitwise_xor_9, &trace);
    assert_component(memory_address_to_id, &trace);
    assert_component(&memory_id_to_value.0, &trace);
    assert_component(&memory_id_to_value.1, &trace);

    if let Some(components) = &blake_context.components {
        assert_component(&components.blake_round, &trace);
        assert_component(&components.blake_g, &trace);
        assert_component(&components.blake_sigma, &trace);
        assert_component(&components.triple_xor_32, &trace);
        assert_component(&components.verify_bitwise_xor_12, &trace);
    }

    // Builtins
    if let Some(add_mod) = &builtins.add_mod_builtin {
        assert_component(add_mod, &trace);
    }
    if let Some(bitwise) = &builtins.bitwise_builtin {
        assert_component(bitwise, &trace);
    }
    if let Some(poseidon) = &builtins.poseidon_builtin {
        assert_component(poseidon, &trace);
    }
    if let Some(rc_96) = &builtins.range_check_96_builtin {
        assert_component(rc_96, &trace);
    }
    if let Some(rc_128) = &builtins.range_check_128_builtin {
        assert_component(rc_128, &trace);
    }

    if let Some(components) = &poseidon_context.components {
        assert_component(&components.poseidon_3_partial_rounds_chain, &trace);
        assert_component(&components.poseidon_full_round_chain, &trace);
        assert_component(&components.cube_252, &trace);
        assert_component(&components.poseidon_round_keys, &trace);
        assert_component(&components.range_check_felt_252_width_27, &trace);
    }
}

pub fn assert_cairo_constraints(input: ProverInput, preprocessed_trace: PreProcessedTrace) {
    let mut commitment_scheme = MockCommitmentScheme::new();

    // Preprocessed trace.
    let mut tree_builder = commitment_scheme.tree_builder();
    tree_builder.extend_evals(preprocessed_trace.gen_trace());
    tree_builder.finalize_interaction();

    // Base trace.
    let cairo_claim_generator = CairoClaimGenerator::new(input);
    let mut tree_builder = commitment_scheme.tree_builder();
    let (claim, interaction_generator) = cairo_claim_generator.write_trace(&mut tree_builder);
    tree_builder.finalize_interaction();

    // Interaction trace.
    let mut dummy_channel = Blake2sChannel::default();
    let interaction_elements = CairoInteractionElements::draw(&mut dummy_channel);
    let mut tree_builder = commitment_scheme.tree_builder();
    let interaction_claim =
        interaction_generator.write_interaction_trace(&mut tree_builder, &interaction_elements);
    tree_builder.finalize_interaction();

    let components = CairoComponents::new(
        &claim,
        &interaction_elements,
        &interaction_claim,
        &preprocessed_trace.ids(),
    );

    assert_cairo_components(commitment_scheme.trace_domain_evaluations(), &components);
}

fn assert_many<E: FrameworkEval + Sync>(
    components: &[FrameworkComponent<E>],
    trace: &TreeVec<Vec<&Vec<M31>>>,
) {
    components.iter().for_each(|x| assert_component(x, trace));
}
