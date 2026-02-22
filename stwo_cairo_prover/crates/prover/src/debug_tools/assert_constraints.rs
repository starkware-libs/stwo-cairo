use std::ops::Deref;
use std::sync::Arc;

use cairo_air::cairo_components::CairoComponents;
use cairo_air::relations::CommonLookupElements;
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
use crate::witness::cairo::create_cairo_claim_generator;
use crate::witness::preprocessed_trace::gen_trace;

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

/// Asserts that all Cairo AIR constraints are satisfied for the given trace and components.
///
/// # Parameters
///
/// * `trace` - The evaluated polynomials on the trace domain.
/// * `cairo_components` - The components constraints to check.
/// * `cairo_components` - The components constraints to check.
pub fn assert_cairo_components(trace: TreeVec<Vec<&Vec<M31>>>, cairo_components: &CairoComponents) {
    fn assert_component<E: FrameworkEval + Sync>(
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
        let component_eval = component.deref();
        assert_constraints_on_trace(
            &component_trace,
            component.log_size(),
            |eval| {
                component_eval.evaluate(eval);
            },
            component.claimed_sum(),
        );
    }

    if let Some(component) = &cairo_components.add_opcode {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.add_opcode_small {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.add_ap_opcode {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.assert_eq_opcode {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.assert_eq_opcode_imm {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.assert_eq_opcode_double_deref {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.blake_compress_opcode {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.call_opcode_abs {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.call_opcode_rel_imm {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.generic_opcode {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.jnz_opcode_non_taken {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.jnz_opcode_taken {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.jump_opcode_abs {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.jump_opcode_double_deref {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.jump_opcode_rel {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.jump_opcode_rel_imm {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.mul_opcode {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.mul_opcode_small {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.qm_31_add_mul_opcode {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.ret_opcode {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.verify_instruction {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.blake_round {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.blake_g {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.blake_round_sigma {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.triple_xor_32 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.verify_bitwise_xor_12 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.add_mod_builtin {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.bitwise_builtin {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.mul_mod_builtin {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.pedersen_builtin {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.pedersen_builtin_narrow_windows {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.poseidon_builtin {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.range_check96_builtin {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.range_check_builtin {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.pedersen_aggregator_window_bits_18 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.partial_ec_mul_window_bits_18 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.pedersen_points_table_window_bits_18 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.pedersen_aggregator_window_bits_9 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.partial_ec_mul_window_bits_9 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.pedersen_points_table_window_bits_9 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.poseidon_aggregator {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.poseidon_3_partial_rounds_chain {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.poseidon_full_round_chain {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.cube_252 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.poseidon_round_keys {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.range_check_252_width_27 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.memory_address_to_id {
        assert_component(component, &trace);
    }
    for component in &cairo_components.memory_id_to_big {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.memory_id_to_small {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.range_check_6 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.range_check_8 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.range_check_11 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.range_check_12 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.range_check_18 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.range_check_20 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.range_check_4_3 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.range_check_4_4 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.range_check_9_9 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.range_check_7_2_5 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.range_check_3_6_6_3 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.range_check_4_4_4_4 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.range_check_3_3_3_3_3 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.verify_bitwise_xor_4 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.verify_bitwise_xor_7 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.verify_bitwise_xor_8 {
        assert_component(component, &trace);
    }
    if let Some(component) = &cairo_components.verify_bitwise_xor_9 {
        assert_component(component, &trace);
    }
}

pub fn assert_cairo_constraints(input: ProverInput, preprocessed_trace: Arc<PreProcessedTrace>) {
    let mut commitment_scheme = MockCommitmentScheme::default();

    // Preprocessed trace.
    let mut tree_builder = commitment_scheme.tree_builder();
    tree_builder.extend_evals(gen_trace(preprocessed_trace.clone()));
    tree_builder.finalize_interaction();

    // Base trace.
    let cairo_claim_generator = create_cairo_claim_generator(input, preprocessed_trace.clone());
    let mut tree_builder = commitment_scheme.tree_builder();
    let (claim, interaction_generator) = cairo_claim_generator.write_trace(&mut tree_builder);
    tree_builder.finalize_interaction();

    // Interaction trace.
    let mut dummy_channel = Blake2sChannel::default();
    let interaction_elements = CommonLookupElements::draw(&mut dummy_channel);
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
