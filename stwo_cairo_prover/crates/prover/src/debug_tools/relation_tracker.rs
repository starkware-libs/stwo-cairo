use cairo_air::air::{CairoComponents, PublicData};
use cairo_air::builtins_air::BuiltinComponents;
use cairo_air::opcodes_air::OpcodeComponents;
use cairo_air::range_checks_air::RangeChecksComponents;
use itertools::{chain, Itertools};
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
use stwo_constraint_framework::{FrameworkComponent, FrameworkEval};

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
        verify_bitwise_xor_8_b,
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

    let mut entries = chain!(
        add_to_relation_entries_many(add, trace),
        add_to_relation_entries_many(add_small, trace),
        add_to_relation_entries_many(add_ap, trace),
        add_to_relation_entries_many(assert_eq, trace),
        add_to_relation_entries_many(assert_eq_imm, trace),
        add_to_relation_entries_many(assert_eq_double_deref, trace),
        add_to_relation_entries_many(blake, trace),
        add_to_relation_entries_many(call, trace),
        add_to_relation_entries_many(call_rel_imm, trace),
        add_to_relation_entries_many(generic, trace),
        add_to_relation_entries_many(jnz, trace),
        add_to_relation_entries_many(jnz_taken, trace),
        add_to_relation_entries_many(jump, trace),
        add_to_relation_entries_many(jump_double_deref, trace),
        add_to_relation_entries_many(jump_rel, trace),
        add_to_relation_entries_many(jump_rel_imm, trace),
        add_to_relation_entries_many(mul, trace),
        add_to_relation_entries_many(mul_small, trace),
        add_to_relation_entries_many(qm31, trace),
        add_to_relation_entries_many(ret, trace),
        add_to_relation_entries(verify_instruction, trace),
        add_to_relation_entries(rc_6, trace),
        add_to_relation_entries(rc_8, trace),
        add_to_relation_entries(rc_11, trace),
        add_to_relation_entries(rc_12, trace),
        add_to_relation_entries(rc_18, trace),
        add_to_relation_entries(rc_20, trace),
        add_to_relation_entries(rc_4_3, trace),
        add_to_relation_entries(rc_4_4, trace),
        add_to_relation_entries(rc_5_4, trace),
        add_to_relation_entries(rc_9_9, trace),
        add_to_relation_entries(rc_9_9_b, trace),
        add_to_relation_entries(rc_9_9_c, trace),
        add_to_relation_entries(rc_9_9_d, trace),
        add_to_relation_entries(rc_9_9_e, trace),
        add_to_relation_entries(rc_9_9_f, trace),
        add_to_relation_entries(rc_9_9_g, trace),
        add_to_relation_entries(rc_9_9_h, trace),
        add_to_relation_entries(rc_18_b, trace),
        add_to_relation_entries(rc_20_b, trace),
        add_to_relation_entries(rc_20_c, trace),
        add_to_relation_entries(rc_20_d, trace),
        add_to_relation_entries(rc_20_e, trace),
        add_to_relation_entries(rc_20_f, trace),
        add_to_relation_entries(rc_20_g, trace),
        add_to_relation_entries(rc_20_h, trace),
        add_to_relation_entries(rc_7_2_5, trace),
        add_to_relation_entries(rc_3_6_6_3, trace),
        add_to_relation_entries(rc_4_4_4_4, trace),
        add_to_relation_entries(rc_3_3_3_3_3, trace),
        add_to_relation_entries(verify_bitwise_xor_4, trace),
        add_to_relation_entries(verify_bitwise_xor_7, trace),
        add_to_relation_entries(verify_bitwise_xor_8, trace),
        add_to_relation_entries(verify_bitwise_xor_8_b, trace),
        add_to_relation_entries(verify_bitwise_xor_9, trace),
        add_to_relation_entries(memory_address_to_id, trace),
        add_to_relation_entries_many(&memory_id_to_value.0, trace),
        add_to_relation_entries(&memory_id_to_value.1, trace),
    )
    .collect_vec();

    if let Some(cairo_air::blake::air::Components {
        blake_round,
        blake_g,
        blake_sigma,
        triple_xor_32,
        verify_bitwise_xor_12,
    }) = &blake_context.components
    {
        entries.extend(chain!(
            add_to_relation_entries(blake_round, trace),
            add_to_relation_entries(blake_g, trace),
            add_to_relation_entries(blake_sigma, trace),
            add_to_relation_entries(triple_xor_32, trace),
            add_to_relation_entries(verify_bitwise_xor_12, trace),
        ));
    }

    // Builtins
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
        entries.extend(add_to_relation_entries(add_mod, trace));
    }
    if let Some(bitwise) = bitwise_builtin {
        entries.extend(add_to_relation_entries(bitwise, trace));
    }
    if let Some(pederson) = pedersen_builtin {
        entries.extend(add_to_relation_entries(pederson, trace));
    }
    if let Some(poseidon) = poseidon_builtin {
        entries.extend(add_to_relation_entries(poseidon, trace));
    }
    if let Some(mul_mod) = mul_mod_builtin {
        entries.extend(add_to_relation_entries(mul_mod, trace));
    }
    if let Some(rc_96) = range_check_96_builtin {
        entries.extend(add_to_relation_entries(rc_96, trace));
    }
    if let Some(rc_128) = range_check_128_builtin {
        entries.extend(add_to_relation_entries(rc_128, trace));
    }

    if let Some(cairo_air::poseidon::air::Components {
        poseidon_aggregator,
        poseidon_3_partial_rounds_chain,
        poseidon_full_round_chain,
        cube_252,
        poseidon_round_keys,
        range_check_252_width_27,
    }) = &poseidon_context.components
    {
        entries.extend(chain!(
            add_to_relation_entries(poseidon_aggregator, trace),
            add_to_relation_entries(poseidon_3_partial_rounds_chain, trace),
            add_to_relation_entries(poseidon_full_round_chain, trace),
            add_to_relation_entries(cube_252, trace),
            add_to_relation_entries(poseidon_round_keys, trace),
            add_to_relation_entries(range_check_252_width_27, trace),
        ));
    }

    if let Some(cairo_air::pedersen::air::Components {
        pedersen_aggregator,
        pedersen_points_table,
        partial_ec_mul,
    }) = &pedersen_context.components
    {
        entries.extend(chain!(
            add_to_relation_entries(pedersen_aggregator, trace),
            add_to_relation_entries(pedersen_points_table, trace),
            add_to_relation_entries(partial_ec_mul, trace),
        ));
    }

    entries
}

fn add_to_relation_entries_many<E: FrameworkEval>(
    components: &[FrameworkComponent<E>],
    trace: &TreeVec<Vec<&Vec<M31>>>,
) -> Vec<RelationTrackerEntry> {
    components
        .iter()
        .flat_map(|x| add_to_relation_entries(x, trace))
        .collect()
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
