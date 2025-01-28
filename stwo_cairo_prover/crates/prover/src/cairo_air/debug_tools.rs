use itertools::Itertools;
use num_traits::One;
use stwo_prover::constraint_framework::relation_tracker::{
    RelationTrackerComponent, RelationTrackerEntry,
};
use stwo_prover::constraint_framework::{
    FrameworkComponent, FrameworkEval, TraceLocationAllocator,
};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::BackendForChannel;
use stwo_prover::core::channel::MerkleChannel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::pcs::CommitmentSchemeProver;
use stwo_prover::core::poly::circle::CanonicCoset;

use super::air::CairoClaim;
use crate::components::{
    add_ap_opcode, add_ap_opcode_imm, add_ap_opcode_op_1_base_fp, add_opcode, add_opcode_imm,
    add_opcode_small, add_opcode_small_imm, assert_eq_opcode, assert_eq_opcode_double_deref,
    assert_eq_opcode_imm, call_opcode, call_opcode_op_1_base_fp, call_opcode_rel, generic_opcode,
    jnz_opcode, jnz_opcode_dst_base_fp, jnz_opcode_taken, jnz_opcode_taken_dst_base_fp,
    jump_opcode, jump_opcode_double_deref, jump_opcode_rel, jump_opcode_rel_imm,
    memory_address_to_id, memory_id_to_big, mul_opcode, mul_opcode_imm, range_check_11,
    range_check_19, range_check_4_3, range_check_6, range_check_7_2_5, range_check_9_9,
    range_check_builtin_bits_128, range_check_builtin_bits_96, ret_opcode, verify_instruction,
};
use crate::felt::split_f252;
use crate::relations;

pub fn track_cairo_relations<MC: MerkleChannel>(
    commitment_scheme: &CommitmentSchemeProver<'_, SimdBackend, MC>,
    claim: &CairoClaim,
) -> Vec<RelationTrackerEntry>
where
    SimdBackend: BackendForChannel<MC>,
{
    // Cairo air aggregates interpolated polynomials. Evaluate to get the original trace.
    // NOTE: this process is slow, and should be only used for debugging.
    let evals = commitment_scheme.trace().polys.map(|interaction_tree| {
        interaction_tree
            .iter()
            .map(|poly| poly.evaluate(CanonicCoset::new(poly.log_size()).circle_domain()))
            .collect_vec()
    });
    let evals = &evals.as_ref();
    let trace = &evals.into();

    let tree_span_provider = &mut TraceLocationAllocator::default();
    let mut entries = vec![];

    // TODO(Ohad): reduce boilerplate.
    for claim in claim.opcodes.add.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                add_opcode::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.add_imm.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                add_opcode_imm::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.add_small.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                add_opcode_small::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.add_small_imm.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                add_opcode_small_imm::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.add_ap.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                add_ap_opcode::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.add_ap_op_1_base_fp.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                add_ap_opcode_op_1_base_fp::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.add_ap_imm.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                add_ap_opcode_imm::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        );
    }

    for claim in claim.opcodes.assert_eq.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                assert_eq_opcode::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.assert_eq_imm.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                assert_eq_opcode_imm::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.assert_eq_double_deref.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                assert_eq_opcode_double_deref::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        )
    }
    for claim in claim.opcodes.call.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                call_opcode::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.call_op_1_base_fp.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                call_opcode_op_1_base_fp::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.call_rel.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                call_opcode_rel::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.generic.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                generic_opcode::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                    range_check_19_lookup_elements: relations::RangeCheck_19::dummy(),
                    range_check_9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.jnz.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                jnz_opcode::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.jnz_dst_base_fp.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                jnz_opcode_dst_base_fp::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.jnz_taken.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                jnz_opcode_taken::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.jnz_taken_dst_base_fp.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                jnz_opcode_taken_dst_base_fp::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.jump.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                jump_opcode::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.jump_double_deref.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                jump_opcode_double_deref::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.jump_rel.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                jump_opcode_rel::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.jump_rel_imm.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                jump_opcode_rel_imm::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.mul.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                mul_opcode::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                    range_check_19_lookup_elements: relations::RangeCheck_19::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.mul_imm.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                mul_opcode_imm::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                    range_check_19_lookup_elements: relations::RangeCheck_19::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.ret.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                ret_opcode::Eval {
                    claim,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_rows,
            )
            .entries(trace),
        );
    }

    // Verify instruction.
    entries.extend(
        RelationTrackerComponent::new(
            tree_span_provider,
            verify_instruction::Eval {
                claim: claim.verify_instruction,
                memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                range_check_4_3_lookup_elements: relations::RangeCheck_4_3::dummy(),
                range_check_7_2_5_lookup_elements: relations::RangeCheck_7_2_5::dummy(),
                verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            },
            1 << claim.verify_instruction.log_size,
        )
        .entries(trace),
    );

    if let Some(range_check_128_builtin) = claim.builtins.range_check_128_builtin {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                range_check_builtin_bits_128::Eval {
                    claim: range_check_128_builtin,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                },
                1 << range_check_128_builtin.log_size,
            )
            .entries(trace),
        );
    }

    if let Some(range_check_96_builtin) = claim.builtins.range_check_96_builtin {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                range_check_builtin_bits_96::Eval {
                    claim: range_check_96_builtin,
                    memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
                    range_check_6_lookup_elements: relations::RangeCheck_6::dummy(),
                },
                1 << range_check_96_builtin.log_size,
            )
            .entries(trace),
        );
    }

    // Memory.
    entries.extend(
        RelationTrackerComponent::new(
            tree_span_provider,
            memory_address_to_id::Eval {
                log_size: claim.memory_address_to_id.log_size,
                lookup_elements: relations::MemoryAddressToId::dummy(),
            },
            1 << claim.memory_address_to_id.log_size,
        )
        .entries(trace),
    );
    entries.extend(
        RelationTrackerComponent::new(
            tree_span_provider,
            memory_id_to_big::BigEval {
                log_n_rows: claim.memory_id_to_value.big_log_size,
                lookup_elements: relations::MemoryIdToBig::dummy(),
                range9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
            },
            1 << claim.memory_id_to_value.big_log_size,
        )
        .entries(trace),
    );
    entries.extend(
        RelationTrackerComponent::new(
            tree_span_provider,
            memory_id_to_big::SmallEval {
                log_n_rows: claim.memory_id_to_value.small_log_size,
                lookup_elements: relations::MemoryIdToBig::dummy(),
                range_check_9_9_relation: relations::RangeCheck_9_9::dummy(),
            },
            1 << claim.memory_id_to_value.small_log_size,
        )
        .entries(trace),
    );

    // Range Checks.
    entries.extend(
        RelationTrackerComponent::new(
            tree_span_provider,
            range_check_11::Eval {
                lookup_elements: relations::RangeCheck_11::dummy(),
            },
            1 << 11,
        )
        .entries(trace),
    );
    entries.extend(
        RelationTrackerComponent::new(
            tree_span_provider,
            range_check_19::Eval {
                lookup_elements: relations::RangeCheck_19::dummy(),
            },
            1 << 19,
        )
        .entries(trace),
    );
    entries.extend(
        RelationTrackerComponent::new(
            tree_span_provider,
            range_check_9_9::Eval {
                lookup_elements: relations::RangeCheck_9_9::dummy(),
            },
            1 << 18,
        )
        .entries(trace),
    );
    entries.extend(
        RelationTrackerComponent::new(
            tree_span_provider,
            range_check_7_2_5::Eval {
                lookup_elements: relations::RangeCheck_7_2_5::dummy(),
            },
            1 << 14,
        )
        .entries(trace),
    );
    entries.extend(
        RelationTrackerComponent::new(
            tree_span_provider,
            range_check_4_3::Eval {
                lookup_elements: relations::RangeCheck_4_3::dummy(),
            },
            1 << 7,
        )
        .entries(trace),
    );
    entries.extend(
        RelationTrackerComponent::new(
            tree_span_provider,
            range_check_6::Eval {
                lookup_elements: relations::RangeCheck_6::dummy(),
            },
            1 << 6,
        )
        .entries(trace),
    );

    // Public data.
    claim
        .public_data
        .public_memory
        .iter()
        .for_each(|(addr, id, val)| {
            entries.push(RelationTrackerEntry {
                relation: "MemoryAddressToId".to_string(),
                mult: M31::one(),
                values: vec![M31::from_u32_unchecked(*addr), M31::from_u32_unchecked(*id)],
            });
            entries.push(RelationTrackerEntry {
                relation: "MemoryIdToBig".to_string(),
                mult: M31::one(),
                values: [
                    [M31::from_u32_unchecked(*id)].as_slice(),
                    split_f252(*val).as_slice(),
                ]
                .concat(),
            });
        });
    entries.push(RelationTrackerEntry {
        relation: "Opcodes".to_string(),
        mult: M31::one(),
        values: claim.public_data.final_state.values().to_vec(),
    });
    entries.push(RelationTrackerEntry {
        relation: "Opcodes".to_string(),
        mult: -M31::one(),
        values: claim.public_data.initial_state.values().to_vec(),
    });

    entries
}

pub(super) fn indented_component_display<E: FrameworkEval>(
    component: &FrameworkComponent<E>,
) -> String {
    let component_display = &format!("\n{}", component);
    component_display
        .lines()
        .map(|line| format!("\t{}", line))
        .join("\n")
}

pub(super) fn display_components<E: FrameworkEval>(components: &[FrameworkComponent<E>]) -> String {
    components
        .iter()
        .map(|component| indented_component_display(component))
        .join("\n")
}
