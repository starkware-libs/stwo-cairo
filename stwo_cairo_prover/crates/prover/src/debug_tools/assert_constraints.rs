use std::ops::Deref;
use std::sync::Arc;

use cairo_air::air::{CairoComponents, CairoInteractionElements};
use cairo_air::builtins_air::BuiltinComponents;
use cairo_air::opcodes_air::OpcodeComponents;
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
        opcodes,
        verify_instruction,
        blake_context,
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
    let OpcodeComponents {
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
    } = opcodes;
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
    assert_many(add, &trace);
    assert_many(add_small, &trace);
    assert_many(add_ap, &trace);
    assert_many(assert_eq, &trace);
    assert_many(assert_eq_imm, &trace);
    assert_many(assert_eq_double_deref, &trace);
    assert_many(blake, &trace);
    assert_many(call, &trace);
    assert_many(call_rel_imm, &trace);
    assert_many(generic, &trace);
    assert_many(jnz, &trace);
    assert_many(jnz_taken, &trace);
    assert_many(jump, &trace);
    assert_many(jump_double_deref, &trace);
    assert_many(jump_rel, &trace);
    assert_many(jump_rel_imm, &trace);
    assert_many(mul, &trace);
    assert_many(mul_small, &trace);
    assert_many(qm31, &trace);
    assert_many(ret, &trace);

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

    if let Some(cairo_air::blake::air::Components {
        blake_round,
        blake_g,
        blake_sigma,
        triple_xor_32,
        verify_bitwise_xor_12,
    }) = &blake_context.components
    {
        assert_component(blake_round, &trace);
        assert_component(blake_g, &trace);
        assert_component(blake_sigma, &trace);
        assert_component(triple_xor_32, &trace);
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
        pedersen_aggregator,
        partial_ec_mul,
        pedersen_points_table,
    }) = &pedersen_context.components
    {
        assert_component(pedersen_aggregator, &trace);
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

pub fn assert_cairo_constraints(input: ProverInput, preprocessed_trace: Arc<PreProcessedTrace>) {
    let mut commitment_scheme = MockCommitmentScheme::default();

    // Preprocessed trace.
    let mut tree_builder = commitment_scheme.tree_builder();
    tree_builder.extend_evals(preprocessed_trace.gen_trace());
    tree_builder.finalize_interaction();

    // Base trace.
    let cairo_claim_generator = CairoClaimGenerator::new(input, Arc::clone(&preprocessed_trace));
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
