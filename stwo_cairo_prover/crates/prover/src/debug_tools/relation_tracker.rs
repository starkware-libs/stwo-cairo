use cairo_air::air::PublicData;
use cairo_air::cairo_components::CairoComponents;
use itertools::Itertools;
use num_traits::{One, Zero};
use stwo::core::channel::MerkleChannel;
use stwo::core::fields::m31::{BaseField, M31};
use stwo::core::pcs::TreeVec;
use stwo::core::poly::circle::CanonicCoset;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::backend::{BackendForChannel, Col, Column};
use stwo::prover::poly::circle::CircleCoefficients;
use stwo::prover::CommitmentSchemeProver;
use stwo_cairo_common::prover_types::felt::split_f252;
use stwo_constraint_framework::relation_tracker::{
    add_to_relation_entries, RelationSummary, RelationTrackerEntry,
};

pub fn track_and_summarize_cairo_relations<MC: MerkleChannel>(
    commitment_scheme: &CommitmentSchemeProver<'_, SimdBackend, MC>,
    components: &CairoComponents,
    public_data: &PublicData,
) -> RelationSummary
where
    SimdBackend: BackendForChannel<MC>,
{
    let entries = track_cairo_relations(commitment_scheme, components, public_data);
    RelationSummary::summarize_relations(&entries).cleaned()
}

pub fn track_cairo_relations<MC: MerkleChannel>(
    commitment_scheme: &CommitmentSchemeProver<'_, SimdBackend, MC>,
    components: &CairoComponents,
    public_data: &PublicData,
) -> Vec<RelationTrackerEntry>
where
    SimdBackend: BackendForChannel<MC>,
{
    // Cairo air aggregates interpolated polynomials. Evaluate to get the original trace.
    // NOTE: this process is slow, and should be only used for debugging.
    // TODO(Ohad): skip lde and merkle.
    let evals = commitment_scheme.trace().polys.map(|interaction_tree| {
        interaction_tree
            .iter()
            .map(|poly| {
                let coeffs = reduce_degree(poly.evals.clone().interpolate());
                coeffs
                    .evaluate(CanonicCoset::new(coeffs.log_size()).circle_domain())
                    .values
                    .to_cpu()
            })
            .collect_vec()
    });
    let evals = &evals.as_ref();
    let trace = &evals.into();

    let mut entries = cairo_relation_entries(components, trace);

    // Public data.
    let initial_pc = public_data.initial_state.pc.0;
    let initial_ap = public_data.initial_state.ap.0;
    let final_ap = public_data.final_state.ap.0;
    public_data
        .public_memory
        .get_entries(initial_pc, initial_ap, final_ap)
        .for_each(|(addr, id, val)| {
            entries.push(RelationTrackerEntry {
                relation: "MemoryAddressToId".to_string(),
                mult: M31::one(),
                values: vec![M31::from_u32_unchecked(addr), M31::from_u32_unchecked(id)],
            });
            entries.push(RelationTrackerEntry {
                relation: "MemoryIdToBig".to_string(),
                mult: M31::one(),
                values: [
                    [M31::from_u32_unchecked(id)].as_slice(),
                    split_f252(val).as_slice(),
                ]
                .concat(),
            });
        });
    entries.push(RelationTrackerEntry {
        relation: "Opcodes".to_string(),
        mult: M31::one(),
        values: public_data.final_state.values().to_vec(),
    });
    entries.push(RelationTrackerEntry {
        relation: "Opcodes".to_string(),
        mult: -M31::one(),
        values: public_data.initial_state.values().to_vec(),
    });

    entries
}

fn cairo_relation_entries(
    cairo_components: &CairoComponents,
    trace: &TreeVec<Vec<&Vec<M31>>>,
) -> Vec<RelationTrackerEntry> {
    let mut entries: Vec<RelationTrackerEntry> = vec![];
    if let Some(component) = &cairo_components.add_opcode {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.add_opcode_small {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.add_ap_opcode {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.assert_eq_opcode {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.assert_eq_opcode_imm {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.assert_eq_opcode_double_deref {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.blake_compress_opcode {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.call_opcode_abs {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.call_opcode_rel_imm {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.generic_opcode {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.jnz_opcode_non_taken {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.jnz_opcode_taken {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.jump_opcode_abs {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.jump_opcode_double_deref {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.jump_opcode_rel {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.jump_opcode_rel_imm {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.mul_opcode {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.mul_opcode_small {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.qm_31_add_mul_opcode {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.ret_opcode {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.verify_instruction {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.blake_round {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.blake_g {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.blake_round_sigma {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.triple_xor_32 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.verify_bitwise_xor_12 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.add_mod_builtin {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.bitwise_builtin {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.mul_mod_builtin {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.pedersen_builtin {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.pedersen_builtin_narrow_windows {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.poseidon_builtin {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.range_check96_builtin {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.range_check_builtin {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.pedersen_aggregator_window_bits_18 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.partial_ec_mul_window_bits_18 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.pedersen_points_table_window_bits_18 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.pedersen_aggregator_window_bits_9 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.partial_ec_mul_window_bits_9 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.pedersen_points_table_window_bits_9 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.poseidon_aggregator {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.poseidon_3_partial_rounds_chain {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.poseidon_full_round_chain {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.cube_252 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.poseidon_round_keys {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.range_check_252_width_27 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.memory_address_to_id {
        entries.extend(add_to_relation_entries(component, trace));
    }
    for component in &cairo_components.memory_id_to_big {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.memory_id_to_small {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.range_check_6 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.range_check_8 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.range_check_11 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.range_check_12 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.range_check_18 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.range_check_20 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.range_check_4_3 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.range_check_4_4 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.range_check_9_9 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.range_check_7_2_5 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.range_check_3_6_6_3 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.range_check_4_4_4_4 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.range_check_3_3_3_3_3 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.verify_bitwise_xor_4 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.verify_bitwise_xor_7 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.verify_bitwise_xor_8 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    if let Some(component) = &cairo_components.verify_bitwise_xor_9 {
        entries.extend(add_to_relation_entries(component, trace));
    }
    entries
}

/// Reduces the polynomial to a minimal degree polynomial that evaluates to the same values.
pub fn reduce_degree(coeffs: CircleCoefficients<SimdBackend>) -> CircleCoefficients<SimdBackend> {
    let mut new_log_size = coeffs.log_size();
    while new_log_size > 1 {
        if ((1 << (new_log_size - 1))..(1 << new_log_size))
            .any(|i| coeffs.coeffs.at(i) != BaseField::zero())
        {
            break;
        }
        new_log_size -= 1;
    }
    CircleCoefficients::new(Col::<SimdBackend, BaseField>::from_iter(
        coeffs.coeffs.to_cpu()[..1 << new_log_size].iter().copied(),
    ))
}
