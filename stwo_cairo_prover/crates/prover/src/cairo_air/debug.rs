use itertools::Itertools;
use num_traits::One;
use stwo_prover::constraint_framework::relation_tracker::{
    RelationTrackerComponent, RelationTrackerEntry,
};
use stwo_prover::constraint_framework::TraceLocationAllocator;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::pcs::CommitmentSchemeProver;
use stwo_prover::core::poly::circle::CanonicCoset;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

use super::air::CairoClaim;
use crate::components::{
    add_ap_opcode_is_imm_f_op1_base_fp_f, add_ap_opcode_is_imm_f_op1_base_fp_t,
    add_ap_opcode_is_imm_t_op1_base_fp_f, add_opcode_is_small_f_is_imm_f,
    add_opcode_is_small_f_is_imm_t, add_opcode_is_small_t_is_imm_f, add_opcode_is_small_t_is_imm_t,
    assert_eq_opcode_is_double_deref_f_is_imm_f, assert_eq_opcode_is_double_deref_f_is_imm_t,
    assert_eq_opcode_is_double_deref_t_is_imm_f, call_opcode_is_rel_f_op1_base_fp_f,
    call_opcode_is_rel_f_op1_base_fp_t, call_opcode_is_rel_t_op1_base_fp_f, generic_opcode,
    jnz_opcode_is_taken_f_dst_base_fp_f, jnz_opcode_is_taken_f_dst_base_fp_t,
    jnz_opcode_is_taken_t_dst_base_fp_f, jnz_opcode_is_taken_t_dst_base_fp_t,
    jump_opcode_is_rel_f_is_imm_f_is_double_deref_f,
    jump_opcode_is_rel_f_is_imm_f_is_double_deref_t,
    jump_opcode_is_rel_t_is_imm_f_is_double_deref_f,
    jump_opcode_is_rel_t_is_imm_t_is_double_deref_f, memory_address_to_id, memory_id_to_big,
    mul_opcode_is_small_f_is_imm_f, mul_opcode_is_small_f_is_imm_t, range_check_19,
    range_check_4_3, range_check_7_2_5, range_check_9_9, ret_opcode, verify_instruction,
};
use crate::felt::split_f252;
use crate::relations;

pub fn track_cairo_relations(
    commitment_scheme: &CommitmentSchemeProver<'_, SimdBackend, Blake2sMerkleChannel>,
    claim: &CairoClaim,
) -> Vec<RelationTrackerEntry> {
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
    for claim in claim.opcodes.add_f_f.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                add_opcode_is_small_f_is_imm_f::Eval {
                    claim,
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_calls,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.add_f_t.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                add_opcode_is_small_f_is_imm_t::Eval {
                    claim,
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_calls,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.add_t_f.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                add_opcode_is_small_t_is_imm_f::Eval {
                    claim,
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_calls,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.add_t_t.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                add_opcode_is_small_t_is_imm_t::Eval {
                    claim,
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_calls,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.add_ap_f_f.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                add_ap_opcode_is_imm_f_op1_base_fp_f::Eval {
                    claim,
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_calls,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.add_ap_f_t.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                add_ap_opcode_is_imm_f_op1_base_fp_t::Eval {
                    claim,
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_calls,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.add_ap_t_f.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                add_ap_opcode_is_imm_t_op1_base_fp_f::Eval {
                    claim,
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_calls,
            )
            .entries(trace),
        );
    }

    for claim in claim.opcodes.assert_eq_f_f.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                assert_eq_opcode_is_double_deref_f_is_imm_f::Eval {
                    claim,
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_calls,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.assert_eq_f_t.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                assert_eq_opcode_is_double_deref_f_is_imm_t::Eval {
                    claim,
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_calls,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.assert_eq_t_f.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                assert_eq_opcode_is_double_deref_t_is_imm_f::Eval {
                    claim,
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_calls,
            )
            .entries(trace),
        )
    }
    for claim in claim.opcodes.call_f_f.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                call_opcode_is_rel_f_op1_base_fp_f::Eval {
                    claim,
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_calls,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.call_f_t.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                call_opcode_is_rel_f_op1_base_fp_t::Eval {
                    claim,
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_calls,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.call_t_f.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                call_opcode_is_rel_t_op1_base_fp_f::Eval {
                    claim,
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_calls,
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
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                    rangecheck_19_lookup_elements: relations::RangeCheck_19::dummy(),
                    rangecheck_9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
                },
                claim.n_calls,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.jnz_f_f.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                jnz_opcode_is_taken_f_dst_base_fp_f::Eval {
                    claim,
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_calls,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.jnz_f_t.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                jnz_opcode_is_taken_f_dst_base_fp_t::Eval {
                    claim,
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_calls,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.jnz_t_f.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                jnz_opcode_is_taken_t_dst_base_fp_f::Eval {
                    claim,
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_calls,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.jnz_t_t.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                jnz_opcode_is_taken_t_dst_base_fp_t::Eval {
                    claim,
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_calls,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.jump_f_f.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                jump_opcode_is_rel_f_is_imm_f_is_double_deref_f::Eval {
                    claim,
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_calls,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.jump_f_t.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                jump_opcode_is_rel_f_is_imm_f_is_double_deref_t::Eval {
                    claim,
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_calls,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.jump_t_f.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                jump_opcode_is_rel_t_is_imm_f_is_double_deref_f::Eval {
                    claim,
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_calls,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.jump_t_t.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                jump_opcode_is_rel_t_is_imm_t_is_double_deref_f::Eval {
                    claim,
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_calls,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.mul_f_f.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                mul_opcode_is_small_f_is_imm_f::Eval {
                    claim,
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                    rangecheck_19_lookup_elements: relations::RangeCheck_19::dummy(),
                },
                claim.n_calls,
            )
            .entries(trace),
        );
    }
    for claim in claim.opcodes.mul_f_t.clone() {
        entries.extend(
            RelationTrackerComponent::new(
                tree_span_provider,
                mul_opcode_is_small_f_is_imm_t::Eval {
                    claim,
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                    rangecheck_19_lookup_elements: relations::RangeCheck_19::dummy(),
                },
                claim.n_calls,
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
                    memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                    memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
                    opcodes_lookup_elements: relations::Opcodes::dummy(),
                    verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
                },
                claim.n_calls,
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
                memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
                memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
                rangecheck_4_3_lookup_elements: relations::RangeCheck_4_3::dummy(),
                rangecheck_7_2_5_lookup_elements: relations::RangeCheck_7_2_5::dummy(),
                verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
            },
            claim.verify_instruction.n_calls,
        )
        .entries(trace),
    );

    // Memory.
    entries.extend(
        RelationTrackerComponent::new(
            tree_span_provider,
            memory_address_to_id::Eval {
                log_n_rows: claim.memory_address_to_id.log_size,
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
