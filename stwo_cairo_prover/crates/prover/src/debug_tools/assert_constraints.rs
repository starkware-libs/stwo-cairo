use std::ops::Deref;

use cairo_air::air::CairoComponents;
use cairo_air::builtins_air::BuiltinComponents;
use cairo_air::cairo_interaction_elements::CairoInteractionElements;
use cairo_air::range_checks_air::RangeChecksComponents;
use itertools::Itertools;
use stwo::core::channel::Blake2sChannel;
use stwo::core::fields::m31::M31;
use stwo::core::pcs::TreeVec;
use stwo_cairo_adapter::ProverInput;
use stwo_cairo_common::preprocessed_columns::preprocessed_trace::PreProcessedTrace;
use stwo_constraint_framework::{
    assert_constraints_on_trace, FrameworkComponent, FrameworkEval, PREPROCESSED_TRACE_IDX,
};

use crate::debug_tools::mock_tree_builder::MockCommitmentScheme;
use crate::witness::cairo::CairoClaimGenerator;

pub fn assert_component<E: FrameworkEval + Sync>(
    component: &FrameworkComponent<E>,
    trace: &TreeVec<Vec<&Vec<M31>>>,
) {
    let mut component_trace = trace
        .sub_tree(component.trace_locations())
        .map(|tree| tree.into_iter().cloned().collect_vec());
    component_trace[PREPROCESSED_TRACE_IDX] = component
        .preprocessed_column_indices()
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
        verify_instruction,
        blake_round,
        blake_g,
        blake_sigma,
        triple_xor_32,
        verify_bitwise_xor_12,
        builtins,
        pedersen_context,
        poseidon_context,
        memory_address_to_id,
        memory_id_to_value,
        range_checks,
        verify_bitwise_xor_4,
        verify_bitwise_xor_7,
        verify_bitwise_xor_8,
        verify_bitwise_xor_8_b,
        verify_bitwise_xor_9,
    } = cairo_components;
    let RangeChecksComponents {
        rc_6,
        rc_8,
        rc_11,
        rc_12,
        rc_18,
        rc_18_b,
        rc_20,
        rc_20_b,
        rc_20_c,
        rc_20_d,
        rc_20_e,
        rc_20_f,
        rc_20_g,
        rc_20_h,
        rc_4_3,
        rc_4_4,
        rc_5_4,
        rc_9_9,
        rc_9_9_b,
        rc_9_9_c,
        rc_9_9_d,
        rc_9_9_e,
        rc_9_9_f,
        rc_9_9_g,
        rc_9_9_h,
        rc_7_2_5,
        rc_3_6_6_3,
        rc_4_4_4_4,
        rc_3_3_3_3_3,
    } = range_checks;
    if let Some(add) = add.as_ref() {
        assert_component(add, &trace);
    }
    if let Some(add_small) = add_small.as_ref() {
        assert_component(add_small, &trace);
    }
    if let Some(add_ap) = add_ap.as_ref() {
        assert_component(add_ap, &trace);
    }
    if let Some(assert_eq) = assert_eq.as_ref() {
        assert_component(assert_eq, &trace);
    }
    if let Some(assert_eq_imm) = assert_eq_imm.as_ref() {
        assert_component(assert_eq_imm, &trace);
    }
    if let Some(assert_eq_double_deref) = assert_eq_double_deref.as_ref() {
        assert_component(assert_eq_double_deref, &trace);
    }
    if let Some(blake) = blake.as_ref() {
        assert_component(blake, &trace);
    }
    if let Some(call) = call.as_ref() {
        assert_component(call, &trace);
    }
    if let Some(call_rel_imm) = call_rel_imm.as_ref() {
        assert_component(call_rel_imm, &trace);
    }
    if let Some(generic) = generic.as_ref() {
        assert_component(generic, &trace);
    }
    if let Some(jnz) = jnz.as_ref() {
        assert_component(jnz, &trace);
    }
    if let Some(jnz_taken) = jnz_taken.as_ref() {
        assert_component(jnz_taken, &trace);
    }
    if let Some(jump) = jump.as_ref() {
        assert_component(jump, &trace);
    }
    if let Some(jump_double_deref) = jump_double_deref.as_ref() {
        assert_component(jump_double_deref, &trace);
    }
    if let Some(jump_rel) = jump_rel.as_ref() {
        assert_component(jump_rel, &trace);
    }
    if let Some(jump_rel_imm) = jump_rel_imm.as_ref() {
        assert_component(jump_rel_imm, &trace);
    }
    if let Some(mul) = mul.as_ref() {
        assert_component(mul, &trace);
    }
    if let Some(mul_small) = mul_small.as_ref() {
        assert_component(mul_small, &trace);
    }
    if let Some(qm31) = qm31.as_ref() {
        assert_component(qm31, &trace);
    }
    if let Some(ret) = ret.as_ref() {
        assert_component(ret, &trace);
    }
    assert_component(verify_instruction, &trace);
    assert_component(rc_6, &trace);
    assert_component(rc_8, &trace);
    assert_component(rc_11, &trace);
    assert_component(rc_12, &trace);
    assert_component(rc_18, &trace);
    assert_component(rc_18_b, &trace);
    assert_component(rc_20, &trace);
    assert_component(rc_20_b, &trace);
    assert_component(rc_20_c, &trace);
    assert_component(rc_20_d, &trace);
    assert_component(rc_20_e, &trace);
    assert_component(rc_20_f, &trace);
    assert_component(rc_20_g, &trace);
    assert_component(rc_20_h, &trace);
    assert_component(rc_4_3, &trace);
    assert_component(rc_4_4, &trace);
    assert_component(rc_5_4, &trace);
    assert_component(rc_9_9, &trace);
    assert_component(rc_9_9_b, &trace);
    assert_component(rc_9_9_c, &trace);
    assert_component(rc_9_9_d, &trace);
    assert_component(rc_9_9_e, &trace);
    assert_component(rc_9_9_f, &trace);
    assert_component(rc_9_9_g, &trace);
    assert_component(rc_9_9_h, &trace);
    assert_component(rc_7_2_5, &trace);
    assert_component(rc_3_6_6_3, &trace);
    assert_component(rc_4_4_4_4, &trace);
    assert_component(rc_3_3_3_3_3, &trace);
    assert_component(verify_bitwise_xor_4, &trace);
    assert_component(verify_bitwise_xor_7, &trace);
    assert_component(verify_bitwise_xor_8, &trace);
    assert_component(verify_bitwise_xor_8_b, &trace);
    assert_component(verify_bitwise_xor_9, &trace);
    assert_component(memory_address_to_id, &trace);
    for component in &memory_id_to_value.0 {
        assert_component(component, &trace);
    }
    assert_component(&memory_id_to_value.1, &trace);

    if let Some(blake_round) = blake_round.as_ref() {
        assert_component(blake_round, &trace);
    }
    if let Some(blake_g) = blake_g.as_ref() {
        assert_component(blake_g, &trace);
    }
    if let Some(blake_sigma) = blake_sigma.as_ref() {
        assert_component(blake_sigma, &trace);
    }
    if let Some(triple_xor_32) = triple_xor_32.as_ref() {
        assert_component(triple_xor_32, &trace);
    }
    if let Some(verify_bitwise_xor_12) = verify_bitwise_xor_12.as_ref() {
        assert_component(verify_bitwise_xor_12, &trace);
    }

    let BuiltinComponents {
        add_mod_builtin,
        bitwise_builtin,
        pedersen_builtin,
        poseidon_builtin,
        mul_mod_builtin,
        range_check_96_builtin,
        range_check_128_builtin,
    } = builtins;
    if let Some(add_mod) = add_mod_builtin {
        assert_component(add_mod, &trace);
    }
    if let Some(mul_mod) = mul_mod_builtin {
        assert_component(mul_mod, &trace);
    }
    if let Some(bitwise) = bitwise_builtin {
        assert_component(bitwise, &trace);
    }
    if let Some(pedersen) = pedersen_builtin {
        assert_component(pedersen, &trace);
    }
    if let Some(poseidon) = poseidon_builtin {
        assert_component(poseidon, &trace);
    }
    if let Some(rc_96) = range_check_96_builtin {
        assert_component(rc_96, &trace);
    }
    if let Some(rc_128) = range_check_128_builtin {
        assert_component(rc_128, &trace);
    }
    if let Some(cairo_air::pedersen::air::Components {
        partial_ec_mul,
        pedersen_points_table,
    }) = &pedersen_context.components
    {
        assert_component(partial_ec_mul, &trace);
        assert_component(pedersen_points_table, &trace);
    }
    if let Some(cairo_air::poseidon::air::Components {
        poseidon_3_partial_rounds_chain,
        poseidon_full_round_chain,
        cube_252,
        poseidon_round_keys,
        range_check_252_width_27,
        poseidon_aggregator,
    }) = &poseidon_context.components
    {
        assert_component(poseidon_3_partial_rounds_chain, &trace);
        assert_component(poseidon_full_round_chain, &trace);
        assert_component(cube_252, &trace);
        assert_component(poseidon_round_keys, &trace);
        assert_component(range_check_252_width_27, &trace);
        assert_component(poseidon_aggregator, &trace);
    }
}

pub fn assert_cairo_constraints(input: ProverInput, preprocessed_trace: PreProcessedTrace) {
    let mut commitment_scheme = MockCommitmentScheme::default();

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
