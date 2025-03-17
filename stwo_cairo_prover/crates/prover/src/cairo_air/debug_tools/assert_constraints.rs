// TODO(Ohad):remove allow.
#![allow(unused)]
use std::ops::Deref;

use itertools::Itertools;
use num_traits::Zero;
use stwo_cairo_adapter::ProverInput;
use stwo_prover::constraint_framework::{
    assert_constraints_on_polys, FrameworkComponent, FrameworkEval, PREPROCESSED_TRACE_IDX,
};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::Blake2sChannel;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::pcs::{CommitmentSchemeProver, PcsConfig, TreeVec};
use stwo_prover::core::poly::circle::{CanonicCoset, CirclePoly, PolyOps};
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

use crate::cairo_air::air::{
    lookup_sum, CairoClaimGenerator, CairoComponents, CairoInteractionElements,
};
use crate::cairo_air::opcodes_air::OpcodeComponents;
use crate::cairo_air::preprocessed::{PreProcessedColumn, PreProcessedTrace};
use crate::cairo_air::prover::LOG_MAX_ROWS;

pub fn assert_component<E: FrameworkEval + Sync>(
    component: &FrameworkComponent<E>,
    trace_polys: &TreeVec<Vec<&CirclePoly<SimdBackend>>>,
) {
    let mut component_polys = trace_polys
        .sub_tree(component.trace_locations())
        .map(|tree| tree.into_iter().cloned().collect_vec());
    component_polys[PREPROCESSED_TRACE_IDX] = component
        .preproccessed_column_indices()
        .iter()
        .map(|idx| trace_polys[PREPROCESSED_TRACE_IDX][*idx])
        .collect();
    let polys = component_polys.map(|tree| tree.into_iter().cloned().collect_vec());

    let log_size = component.log_size();
    let trace_domain = CanonicCoset::new(log_size);

    let component_eval = component.deref();
    assert_constraints_on_polys(
        &polys,
        trace_domain,
        |eval| {
            component_eval.evaluate(eval);
        },
        component.claimed_sum(),
    );
}

fn assert_cairo_components(
    trace_polys: TreeVec<Vec<&CirclePoly<SimdBackend>>>,
    cairo_components: &CairoComponents,
) {
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
    assert_many(add, &trace_polys);
    assert_many(add_imm, &trace_polys);
    assert_many(add_small, &trace_polys);
    assert_many(add_small_imm, &trace_polys);
    assert_many(add_ap, &trace_polys);
    assert_many(add_ap_op_1_base_fp, &trace_polys);
    assert_many(add_ap_imm, &trace_polys);
    assert_many(assert_eq, &trace_polys);
    assert_many(assert_eq_imm, &trace_polys);
    assert_many(assert_eq_double_deref, &trace_polys);
    assert_many(blake, &trace_polys);
    assert_many(call, &trace_polys);
    assert_many(call_op_1_base_fp, &trace_polys);
    assert_many(call_rel, &trace_polys);
    assert_many(generic, &trace_polys);
    assert_many(jnz, &trace_polys);
    assert_many(jnz_dst_base_fp, &trace_polys);
    assert_many(jnz_taken, &trace_polys);
    assert_many(jnz_taken_dst_base_fp, &trace_polys);
    assert_many(jump, &trace_polys);
    assert_many(jump_double_deref, &trace_polys);
    assert_many(jump_rel, &trace_polys);
    assert_many(jump_rel_imm, &trace_polys);
    assert_many(mul, &trace_polys);
    assert_many(mul_imm, &trace_polys);
    assert_many(mul_small, &trace_polys);
    assert_many(mul_small_imm, &trace_polys);
    assert_many(qm31, &trace_polys);
    assert_many(ret, &trace_polys);

    assert_component(verify_instruction, &trace_polys);
    assert_component(verify_instruction, &trace_polys);
    assert_component(&range_checks.rc_6, &trace_polys);
    assert_component(&range_checks.rc_11, &trace_polys);
    assert_component(&range_checks.rc_12, &trace_polys);
    assert_component(&range_checks.rc_18, &trace_polys);
    assert_component(&range_checks.rc_19, &trace_polys);
    assert_component(&range_checks.rc_3_6, &trace_polys);
    assert_component(&range_checks.rc_4_3, &trace_polys);
    assert_component(&range_checks.rc_4_4, &trace_polys);
    assert_component(&range_checks.rc_9_9, &trace_polys);
    assert_component(&range_checks.rc_7_2_5, &trace_polys);
    assert_component(&range_checks.rc_3_6_6_3, &trace_polys);
    assert_component(&range_checks.rc_4_4_4_4, &trace_polys);
    assert_component(&range_checks.rc_3_3_3_3_3, &trace_polys);
    assert_component(verify_bitwise_xor_4, &trace_polys);
    assert_component(verify_bitwise_xor_7, &trace_polys);
    assert_component(verify_bitwise_xor_8, &trace_polys);
    assert_component(verify_bitwise_xor_9, &trace_polys);
    assert_component(memory_address_to_id, &trace_polys);
    assert_component(&memory_id_to_value.0, &trace_polys);
    assert_component(&memory_id_to_value.1, &trace_polys);

    if let Some(components) = &blake_context.components {
        assert_component(&components.blake_round, &trace_polys);
        assert_component(&components.blake_g, &trace_polys);
        assert_component(&components.blake_sigma, &trace_polys);
        assert_component(&components.triple_xor_32, &trace_polys);
        assert_component(&components.verify_bitwise_xor_12, &trace_polys);
    }

    // Builtins
    if let Some(add_mod) = &builtins.add_mod_builtin {
        assert_component(add_mod, &trace_polys);
    }
    if let Some(bitwise) = &builtins.bitwise_builtin {
        assert_component(bitwise, &trace_polys);
    }
    if let Some(rc_96) = &builtins.range_check_96_builtin {
        assert_component(rc_96, &trace_polys);
    }
    if let Some(rc_128) = &builtins.range_check_128_builtin {
        assert_component(rc_128, &trace_polys);
    }

    if let Some(components) = &poseidon_context.components {
        assert_component(&components.poseidon_3_partial_rounds_chain, &trace_polys);
        assert_component(&components.poseidon_full_round_chain, &trace_polys);
        assert_component(&components.cube_252, &trace_polys);
        assert_component(&components.poseidon_round_keys, &trace_polys);
        assert_component(&components.range_check_felt_252_width_27, &trace_polys);
    }
}

pub fn assert_cairo_constraints(input: ProverInput, preprocessed_trace: PreProcessedTrace) {
    let pcs_config = PcsConfig::default();
    let twiddles = SimdBackend::precompute_twiddles(
        CanonicCoset::new(LOG_MAX_ROWS + pcs_config.fri_config.log_blowup_factor + 2)
            .circle_domain()
            .half_coset,
    );
    let channel = &mut Blake2sChannel::default();
    let mut commitment_scheme =
        CommitmentSchemeProver::<SimdBackend, Blake2sMerkleChannel>::new(pcs_config, &twiddles);

    // Preprocessed trace.
    let mut tree_builder = commitment_scheme.tree_builder();
    tree_builder.extend_evals(preprocessed_trace.gen_trace());
    tree_builder.commit(channel);

    // Base trace.
    let cairo_claim_generator = CairoClaimGenerator::new(input);
    let mut tree_builder = commitment_scheme.tree_builder();
    let (claim, interaction_generator) = cairo_claim_generator.write_trace(&mut tree_builder);
    claim.mix_into(channel);
    tree_builder.commit(channel);

    // Interaction trace.
    let interaction_elements = CairoInteractionElements::draw(channel);
    let mut tree_builder = commitment_scheme.tree_builder();
    let interaction_claim =
        interaction_generator.write_interaction_trace(&mut tree_builder, &interaction_elements);
    tree_builder.commit(channel);

    let components = CairoComponents::new(
        &claim,
        &interaction_elements,
        &interaction_claim,
        &preprocessed_trace.ids(),
    );

    assert_eq!(
        lookup_sum(&claim, &interaction_elements, &interaction_claim),
        SecureField::zero()
    );
    assert_cairo_components(commitment_scheme.polynomials(), &components);
}

fn assert_many<E: FrameworkEval + Sync>(
    components: &[FrameworkComponent<E>],
    trace_polys: &TreeVec<Vec<&CirclePoly<SimdBackend>>>,
) {
    components
        .iter()
        .for_each(|x| assert_component(x, trace_polys));
}
