use std::collections::HashMap;

use cairo_air::air::{CairoComponents, CairoInteractionElements, PublicData};
use cairo_air::opcodes_air::OpcodeComponents;
use cairo_air::preprocessed::PreProcessedTrace;
use num_traits::One;
use stwo::core::channel::Blake2sChannel;
use stwo::core::fields::m31::M31;
use stwo::core::pcs::TreeVec;
use stwo_cairo_adapter::ProverInput;
use stwo_cairo_common::prover_types::felt::split_f252;
use stwo_constraint_framework::relation_tracker::{add_to_relation_entries, RelationTrackerEntry};
use stwo_constraint_framework::{FrameworkComponent, FrameworkEval};

use crate::debug_tools::mock_tree_builder::MockCommitmentScheme;
use crate::witness::cairo::CairoClaimGenerator;

type CairoRelationEntries = HashMap<String, Vec<RelationTrackerEntry>>;

pub fn relation_tracker(
    input: ProverInput,
    preprocessed_trace: PreProcessedTrace,
) -> CairoRelationEntries {
    // Proof preliminaries.
    let mut dummy_channel = Blake2sChannel::default();
    let mut commitment_scheme = MockCommitmentScheme::default();
    let mut tree_builder = commitment_scheme.tree_builder();
    tracing::info!("Generating preprocessed trace");
    tree_builder.extend_evals(preprocessed_trace.gen_trace());
    tree_builder.finalize_interaction();

    // Base trace.
    tracing::info!("Generating base trace");
    let cairo_claim_generator = CairoClaimGenerator::new(input);
    let mut tree_builder = commitment_scheme.tree_builder();
    let (claim, interaction_generator) = cairo_claim_generator.write_trace(&mut tree_builder);
    tree_builder.finalize_interaction();

    // Interaction trace. NOTE: we only need the interaction claim, not the interaction trace.
    tracing::info!("Generating interaction trace");
    let dummy_interaction_elements = CairoInteractionElements::draw(&mut dummy_channel);
    let mut tree_builder = commitment_scheme.tree_builder();
    let interaction_claim = interaction_generator
        .write_interaction_trace(&mut tree_builder, &dummy_interaction_elements);

    let trace = commitment_scheme.trace_domain_evaluations();
    let components = CairoComponents::new(
        &claim,
        &dummy_interaction_elements,
        &interaction_claim,
        &preprocessed_trace.ids(),
    );

    tracing::info!("Tracking Cairo relations");
    track_cairo_relations(&trace, &components, &claim.public_data)
}

pub fn track_cairo_relations(
    trace: &TreeVec<Vec<&Vec<M31>>>,
    components: &CairoComponents,
    public_data: &PublicData,
) -> CairoRelationEntries {
    let mut entries = cairo_relation_entries(components, trace);

    // Public data.
    let get_location = || -> (String, u32, u32) {
        let caller = std::panic::Location::caller();
        (caller.file().to_string(), caller.line(), caller.column())
    };
    let mut memory_address_to_id_relation = entries.remove("MemoryAddressToId").unwrap();
    let mut memory_id_to_big_relation = entries.remove("MemoryIdToBig").unwrap();
    let initial_pc = public_data.initial_state.pc.0;
    let initial_ap = public_data.initial_state.ap.0;
    let final_ap = public_data.final_state.ap.0;
    public_data
        .public_memory
        .get_entries(initial_pc, initial_ap, final_ap)
        .for_each(|(addr, id, val)| {
            memory_address_to_id_relation.push(RelationTrackerEntry {
                mult: M31::one(),
                values: vec![M31::from_u32_unchecked(addr), M31::from_u32_unchecked(id)],
                location: get_location(),
            });
            memory_id_to_big_relation.push(RelationTrackerEntry {
                mult: M31::one(),
                values: [
                    [M31::from_u32_unchecked(id)].as_slice(),
                    split_f252(val).as_slice(),
                ]
                .concat(),
                location: get_location(),
            });
        });
    let mut opcodes_relation = entries.remove("Opcodes").unwrap();
    opcodes_relation.push(RelationTrackerEntry {
        mult: M31::one(),
        values: public_data.final_state.values().to_vec(),
        location: get_location(),
    });
    opcodes_relation.push(RelationTrackerEntry {
        mult: -M31::one(),
        values: public_data.initial_state.values().to_vec(),
        location: get_location(),
    });

    entries.insert(
        "MemoryAddressToId".to_string(),
        memory_address_to_id_relation,
    );
    entries.insert("MemoryIdToBig".to_string(), memory_id_to_big_relation);
    entries.insert("Opcodes".to_string(), opcodes_relation);

    entries
}

fn cairo_relation_entries(
    cairo_components: &CairoComponents,
    trace: &TreeVec<Vec<&Vec<M31>>>,
) -> CairoRelationEntries {
    let CairoComponents {
        opcodes,
        verify_instruction,
        blake_context,
        builtins,
        memory_address_to_id,
        memory_id_to_value,
        range_checks,
        verify_bitwise_xor_4,
        verify_bitwise_xor_7,
        verify_bitwise_xor_8,
        verify_bitwise_xor_9,
        pedersen_context,
        poseidon_context,
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

    let mut entries = HashMap::new();
    tracing::info!("Tracking opcodes");
    entries = add_to_relation_entries_many(add, trace, entries);
    entries = add_to_relation_entries_many(add_small, trace, entries);
    entries = add_to_relation_entries_many(add_ap, trace, entries);
    entries = add_to_relation_entries_many(assert_eq, trace, entries);
    entries = add_to_relation_entries_many(assert_eq_imm, trace, entries);
    entries = add_to_relation_entries_many(assert_eq_double_deref, trace, entries);
    entries = add_to_relation_entries_many(blake, trace, entries);
    entries = add_to_relation_entries_many(call, trace, entries);
    entries = add_to_relation_entries_many(call_rel_imm, trace, entries);
    entries = add_to_relation_entries_many(generic, trace, entries);
    entries = add_to_relation_entries_many(jnz, trace, entries);
    entries = add_to_relation_entries_many(jnz_taken, trace, entries);
    entries = add_to_relation_entries_many(jump, trace, entries);
    entries = add_to_relation_entries_many(jump_double_deref, trace, entries);
    entries = add_to_relation_entries_many(jump_rel, trace, entries);
    entries = add_to_relation_entries_many(jump_rel_imm, trace, entries);
    entries = add_to_relation_entries_many(mul, trace, entries);
    entries = add_to_relation_entries_many(mul_small, trace, entries);
    entries = add_to_relation_entries_many(qm31, trace, entries);
    entries = add_to_relation_entries_many(ret, trace, entries);
    entries = add_to_relation_entries(verify_instruction, trace, entries);
    entries = add_to_relation_entries(&range_checks.rc_6, trace, entries);
    entries = add_to_relation_entries(&range_checks.rc_8, trace, entries);
    entries = add_to_relation_entries(&range_checks.rc_11, trace, entries);
    entries = add_to_relation_entries(&range_checks.rc_12, trace, entries);
    entries = add_to_relation_entries(&range_checks.rc_18, trace, entries);
    entries = add_to_relation_entries(&range_checks.rc_19, trace, entries);
    entries = add_to_relation_entries(&range_checks.rc_4_3, trace, entries);
    entries = add_to_relation_entries(&range_checks.rc_4_4, trace, entries);
    entries = add_to_relation_entries(&range_checks.rc_5_4, trace, entries);
    entries = add_to_relation_entries(&range_checks.rc_9_9, trace, entries);
    entries = add_to_relation_entries(&range_checks.rc_7_2_5, trace, entries);
    entries = add_to_relation_entries(&range_checks.rc_3_6_6_3, trace, entries);
    entries = add_to_relation_entries(&range_checks.rc_4_4_4_4, trace, entries);
    entries = add_to_relation_entries(&range_checks.rc_3_3_3_3_3, trace, entries);
    entries = add_to_relation_entries(verify_bitwise_xor_4, trace, entries);
    entries = add_to_relation_entries(verify_bitwise_xor_7, trace, entries);
    entries = add_to_relation_entries(verify_bitwise_xor_8, trace, entries);
    entries = add_to_relation_entries(verify_bitwise_xor_9, trace, entries);
    entries = add_to_relation_entries(memory_address_to_id, trace, entries);
    entries = add_to_relation_entries_many(&memory_id_to_value.0, trace, entries);
    entries = add_to_relation_entries(&memory_id_to_value.1, trace, entries);

    if let Some(components) = &blake_context.components {
        tracing::info!("Tracking Blake components");
        entries = add_to_relation_entries(&components.blake_round, trace, entries);
        entries = add_to_relation_entries(&components.blake_g, trace, entries);
        entries = add_to_relation_entries(&components.blake_sigma, trace, entries);
        entries = add_to_relation_entries(&components.triple_xor_32, trace, entries);
        entries = add_to_relation_entries(&components.verify_bitwise_xor_12, trace, entries);
    }

    // Builtins
    if let Some(add_mod) = &builtins.add_mod_builtin {
        tracing::info!("Tracking add_mod");
        entries = add_to_relation_entries(add_mod, trace, entries);
    }
    if let Some(bitwise) = &builtins.bitwise_builtin {
        tracing::info!("Tracking bitwise");
        entries = add_to_relation_entries(bitwise, trace, entries);
    }
    if let Some(pederson) = &builtins.pedersen_builtin {
        tracing::info!("Tracking pedersen");
        entries = add_to_relation_entries(pederson, trace, entries);
    }
    if let Some(poseidon) = &builtins.poseidon_builtin {
        tracing::info!("Tracking poseidon");
        entries = add_to_relation_entries(poseidon, trace, entries);
    }
    if let Some(mul_mod) = &builtins.mul_mod_builtin {
        tracing::info!("Tracking mul_mod");
        entries = add_to_relation_entries(mul_mod, trace, entries);
    }
    if let Some(rc_96) = &builtins.range_check_96_builtin {
        tracing::info!("Tracking rc_96");
        entries = add_to_relation_entries(rc_96, trace, entries);
    }
    if let Some(rc_128) = &builtins.range_check_128_builtin {
        tracing::info!("Tracking rc_128");
        entries = add_to_relation_entries(rc_128, trace, entries);
    }

    if let Some(components) = &poseidon_context.components {
        tracing::info!("Tracking poseidon components");
        entries =
            add_to_relation_entries(&components.poseidon_3_partial_rounds_chain, trace, entries);
        entries = add_to_relation_entries(&components.poseidon_full_round_chain, trace, entries);
        entries = add_to_relation_entries(&components.cube_252, trace, entries);
        entries = add_to_relation_entries(&components.poseidon_round_keys, trace, entries);
        entries =
            add_to_relation_entries(&components.range_check_felt_252_width_27, trace, entries);
    }

    if let Some(components) = &pedersen_context.components {
        tracing::info!("Tracking pedersen components");
        entries = add_to_relation_entries(&components.pedersen_points_table, trace, entries);
        entries = add_to_relation_entries(&components.partial_ec_mul, trace, entries);
    }

    entries
}

fn add_to_relation_entries_many<E: FrameworkEval + Sync>(
    components: &[FrameworkComponent<E>],
    trace: &TreeVec<Vec<&Vec<M31>>>,
    mut entries: CairoRelationEntries,
) -> CairoRelationEntries {
    for component in components {
        entries = add_to_relation_entries(component, trace, entries);
    }
    entries
}

#[cfg(test)]
mod tests {
    use stwo_cairo_adapter::test_utils::{get_test_program, run_program_and_adapter};
    use stwo_constraint_framework::relation_tracker::RelationSummary;

    use super::*;

    #[test]
    fn test_relation_tracker() {
        let compiled_program = get_test_program("test_prove_verify_all_opcode_components");
        let input = run_program_and_adapter(&compiled_program);
        let preprocessed_trace = PreProcessedTrace::canonical_without_pedersen();

        let entries = relation_tracker(input, preprocessed_trace);

        let summary = RelationSummary::summarize_relations(&entries);
        let addr_to_id_relation = summary
            .get_relation_info("MemoryAddressToId")
            .map(|r| r.to_vec());
        let cleaned_summary = summary.cleaned();
        let addr_to_id_relation_cleaned = cleaned_summary.get_relation_info("MemoryAddressToId");

        assert!(!addr_to_id_relation.unwrap().is_empty());
        assert!(addr_to_id_relation_cleaned.is_none());
    }
}
