use cairo_air::components::{
    add_ap_opcode as add_ap_opcode_air, add_opcode as add_opcode_air,
    add_opcode_small as add_opcode_small_air, assert_eq_opcode as assert_eq_opcode_air,
    assert_eq_opcode_double_deref as assert_eq_double_deref_air,
    assert_eq_opcode_imm as assert_eq_imm_air, call_opcode_abs as call_opcode_abs_air,
    call_opcode_rel_imm as call_opcode_rel_imm_air, generic_opcode as generic_opcode_air,
    jnz_opcode_non_taken as jnz_opcode_non_taken_air, jnz_opcode_taken as jnz_opcode_taken_air,
    jump_opcode_abs as jump_opcode_abs_air,
    jump_opcode_double_deref as jump_opcode_double_deref_air,
    jump_opcode_rel as jump_opcode_rel_air, jump_opcode_rel_imm as jump_opcode_rel_imm_air,
    mul_opcode as mul_opcode_air, mul_opcode_small as mul_opcode_small_air,
    qm_31_add_mul_opcode as qm_31_add_mul_opcode_air, ret_opcode as ret_opcode_air,
};
use cairo_air::opcodes_air::OpcodeClaim;
use rayon::scope;
use stwo::prover::backend::simd::SimdBackend;
use stwo_cairo_adapter::opcodes::CasmStatesByOpcode;

use crate::witness::components::{
    add_ap_opcode, add_opcode, add_opcode_small, assert_eq_opcode, assert_eq_opcode_double_deref,
    assert_eq_opcode_imm, blake_compress_opcode, blake_round, call_opcode_abs, call_opcode_rel_imm,
    generic_opcode, jnz_opcode_non_taken, jnz_opcode_taken, jump_opcode_abs,
    jump_opcode_double_deref, jump_opcode_rel, jump_opcode_rel_imm, memory_address_to_id,
    memory_id_to_big, mul_opcode, mul_opcode_small, qm_31_add_mul_opcode, range_check_11,
    range_check_18, range_check_20, range_check_4_4_4_4, range_check_7_2_5, range_check_9_9,
    ret_opcode, triple_xor_32, verify_bitwise_xor_8, verify_instruction,
};
use crate::witness::prelude::{CircleEvaluation, M31};
use crate::witness::utils::TreeBuilder;

/// Type alias for trace evaluations
pub type TraceEval = Vec<CircleEvaluation<SimdBackend, M31, stwo::prover::poly::BitReversedOrder>>;

pub fn get_opcodes(casm_states_by_opcode: &CasmStatesByOpcode) -> Vec<&'static str> {
    let mut opcodes = vec![];
    if !casm_states_by_opcode.add_opcode.is_empty() {
        opcodes.push("add_opcode");
    }
    if !casm_states_by_opcode.add_opcode_small.is_empty() {
        opcodes.push("add_opcode_small");
    }
    if !casm_states_by_opcode.add_ap_opcode.is_empty() {
        opcodes.push("add_ap_opcode");
    }
    if !casm_states_by_opcode.assert_eq_opcode.is_empty() {
        opcodes.push("assert_eq_opcode");
    }
    if !casm_states_by_opcode.assert_eq_opcode_imm.is_empty() {
        opcodes.push("assert_eq_opcode_imm");
    }
    if !casm_states_by_opcode
        .assert_eq_opcode_double_deref
        .is_empty()
    {
        opcodes.push("assert_eq_opcode_double_deref");
    }
    if !casm_states_by_opcode.blake_compress_opcode.is_empty() {
        opcodes.push("blake_compress_opcode");
    }
    if !casm_states_by_opcode.call_opcode_abs.is_empty() {
        opcodes.push("call_opcode_abs");
    }
    if !casm_states_by_opcode.call_opcode_rel_imm.is_empty() {
        opcodes.push("call_opcode_rel_imm");
    }
    if !casm_states_by_opcode.generic_opcode.is_empty() {
        opcodes.push("generic_opcode");
    }
    if !casm_states_by_opcode.jnz_opcode_non_taken.is_empty() {
        opcodes.push("jnz_opcode_non_taken");
    }
    if !casm_states_by_opcode.jnz_opcode_taken.is_empty() {
        opcodes.push("jnz_opcode_taken");
    }
    if !casm_states_by_opcode.jump_opcode_abs.is_empty() {
        opcodes.push("jump_opcode_abs");
    }
    if !casm_states_by_opcode.jump_opcode_double_deref.is_empty() {
        opcodes.push("jump_opcode_double_deref");
    }
    if !casm_states_by_opcode.jump_opcode_rel.is_empty() {
        opcodes.push("jump_opcode_rel");
    }
    if !casm_states_by_opcode.jump_opcode_rel_imm.is_empty() {
        opcodes.push("jump_opcode_rel_imm");
    }
    if !casm_states_by_opcode.mul_opcode.is_empty() {
        opcodes.push("mul_opcode");
    }
    if !casm_states_by_opcode.mul_opcode_small.is_empty() {
        opcodes.push("mul_opcode_small");
    }
    if !casm_states_by_opcode.qm_31_add_mul_opcode.is_empty() {
        opcodes.push("qm_31_add_mul_opcode");
    }
    if !casm_states_by_opcode.ret_opcode.is_empty() {
        opcodes.push("ret_opcode");
    }

    opcodes
}

/// Result of pre-blake opcode trace generation.
/// Contains traces and intermediate data needed to complete with blake.
pub struct PreBlakeOpcodeResult {
    // Traces already converted to evals, along with claims and interaction generators
    pub add_result: Option<(TraceEval, add_opcode::InteractionClaimGenerator)>,
    pub add_small_result: Option<(TraceEval, add_opcode_small::InteractionClaimGenerator)>,
    pub add_ap_result: Option<(TraceEval, add_ap_opcode::InteractionClaimGenerator)>,
    pub assert_eq_result: Option<(TraceEval, assert_eq_opcode::InteractionClaimGenerator)>,
    pub assert_eq_imm_result: Option<(TraceEval, assert_eq_opcode_imm::InteractionClaimGenerator)>,
    pub assert_eq_double_deref_result: Option<(
        TraceEval,
        assert_eq_opcode_double_deref::InteractionClaimGenerator,
    )>,
    pub call_result: Option<(TraceEval, call_opcode_abs::InteractionClaimGenerator)>,
    pub call_rel_imm_result: Option<(TraceEval, call_opcode_rel_imm::InteractionClaimGenerator)>,
    pub generic_result: Option<(TraceEval, generic_opcode::InteractionClaimGenerator)>,
    pub jnz_result: Option<(TraceEval, jnz_opcode_non_taken::InteractionClaimGenerator)>,
    pub jnz_taken_result: Option<(TraceEval, jnz_opcode_taken::InteractionClaimGenerator)>,
    pub jump_result: Option<(TraceEval, jump_opcode_abs::InteractionClaimGenerator)>,
    pub jump_double_deref_result: Option<(
        TraceEval,
        jump_opcode_double_deref::InteractionClaimGenerator,
    )>,
    pub jump_rel_result: Option<(TraceEval, jump_opcode_rel::InteractionClaimGenerator)>,
    pub jump_rel_imm_result: Option<(TraceEval, jump_opcode_rel_imm::InteractionClaimGenerator)>,
    pub mul_result: Option<(TraceEval, mul_opcode::InteractionClaimGenerator)>,
    pub mul_small_result: Option<(TraceEval, mul_opcode_small::InteractionClaimGenerator)>,
    pub qm31_result: Option<(TraceEval, qm_31_add_mul_opcode::InteractionClaimGenerator)>,
    pub ret_result: Option<(TraceEval, ret_opcode::InteractionClaimGenerator)>,
    // Also store the claims separately (using cairo_air types directly)
    pub add_claims: Vec<add_opcode_air::Claim>,
    pub add_small_claims: Vec<add_opcode_small_air::Claim>,
    pub add_ap_claims: Vec<add_ap_opcode_air::Claim>,
    pub assert_eq_claims: Vec<assert_eq_opcode_air::Claim>,
    pub assert_eq_imm_claims: Vec<assert_eq_imm_air::Claim>,
    pub assert_eq_double_deref_claims: Vec<assert_eq_double_deref_air::Claim>,
    pub call_claims: Vec<call_opcode_abs_air::Claim>,
    pub call_rel_imm_claims: Vec<call_opcode_rel_imm_air::Claim>,
    pub generic_claims: Vec<generic_opcode_air::Claim>,
    pub jnz_claims: Vec<jnz_opcode_non_taken_air::Claim>,
    pub jnz_taken_claims: Vec<jnz_opcode_taken_air::Claim>,
    pub jump_claims: Vec<jump_opcode_abs_air::Claim>,
    pub jump_double_deref_claims: Vec<jump_opcode_double_deref_air::Claim>,
    pub jump_rel_claims: Vec<jump_opcode_rel_air::Claim>,
    pub jump_rel_imm_claims: Vec<jump_opcode_rel_imm_air::Claim>,
    pub mul_claims: Vec<mul_opcode_air::Claim>,
    pub mul_small_claims: Vec<mul_opcode_small_air::Claim>,
    pub qm31_claims: Vec<qm_31_add_mul_opcode_air::Claim>,
    pub ret_claims: Vec<ret_opcode_air::Claim>,
}

/// Generate pre-blake opcode traces in parallel.
/// This can run concurrently with preprocessed trace generation.
#[allow(clippy::too_many_arguments)]
pub fn pre_blake_opcodes_generate(
    add: Option<add_opcode::ClaimGenerator>,
    add_small: Option<add_opcode_small::ClaimGenerator>,
    add_ap: Option<add_ap_opcode::ClaimGenerator>,
    assert_eq: Option<assert_eq_opcode::ClaimGenerator>,
    assert_eq_imm: Option<assert_eq_opcode_imm::ClaimGenerator>,
    assert_eq_double_deref: Option<assert_eq_opcode_double_deref::ClaimGenerator>,
    call: Option<call_opcode_abs::ClaimGenerator>,
    call_rel_imm: Option<call_opcode_rel_imm::ClaimGenerator>,
    generic: Option<generic_opcode::ClaimGenerator>,
    jnz: Option<jnz_opcode_non_taken::ClaimGenerator>,
    jnz_taken: Option<jnz_opcode_taken::ClaimGenerator>,
    jump: Option<jump_opcode_abs::ClaimGenerator>,
    jump_double_deref: Option<jump_opcode_double_deref::ClaimGenerator>,
    jump_rel: Option<jump_opcode_rel::ClaimGenerator>,
    jump_rel_imm: Option<jump_opcode_rel_imm::ClaimGenerator>,
    mul: Option<mul_opcode::ClaimGenerator>,
    mul_small: Option<mul_opcode_small::ClaimGenerator>,
    qm31: Option<qm_31_add_mul_opcode::ClaimGenerator>,
    ret: Option<ret_opcode::ClaimGenerator>,
    memory_address_to_id_trace_generator: Option<&memory_address_to_id::ClaimGenerator>,
    memory_id_to_value_trace_generator: Option<&memory_id_to_big::ClaimGenerator>,
    rc_11_trace_generator: Option<&range_check_11::ClaimGenerator>,
    rc_18_trace_generator: Option<&range_check_18::ClaimGenerator>,
    rc_20_trace_generator: Option<&range_check_20::ClaimGenerator>,
    rc_4_4_4_4_trace_generator: Option<&range_check_4_4_4_4::ClaimGenerator>,
    rc_9_9_trace_generator: Option<&range_check_9_9::ClaimGenerator>,
    verify_instruction_trace_generator: Option<&verify_instruction::ClaimGenerator>,
) -> PreBlakeOpcodeResult {
    let mut add_result = None;
    let mut add_small_result = None;
    let mut add_ap_result = None;
    let mut assert_eq_result = None;
    let mut assert_eq_imm_result = None;
    let mut assert_eq_double_deref_result = None;
    let mut call_result = None;
    let mut call_rel_imm_result = None;
    let mut generic_result = None;
    let mut jnz_result = None;
    let mut jnz_taken_result = None;
    let mut jump_result = None;
    let mut jump_double_deref_result = None;
    let mut jump_rel_result = None;
    let mut jump_rel_imm_result = None;
    let mut mul_result = None;
    let mut mul_small_result = None;
    let mut qm31_result = None;
    let mut ret_result = None;

    scope(|s| {
        s.spawn(|_| {
            add_result = add.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            add_small_result = add_small.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            add_ap_result = add_ap.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                        rc_18_trace_generator.unwrap(),
                        rc_11_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            assert_eq_result = assert_eq.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            assert_eq_imm_result = assert_eq_imm.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            assert_eq_double_deref_result = assert_eq_double_deref.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            call_result = call.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            call_rel_imm_result = call_rel_imm.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            generic_result = generic.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                        rc_9_9_trace_generator.unwrap(),
                        rc_20_trace_generator.unwrap(),
                        rc_18_trace_generator.unwrap(),
                        rc_11_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            jnz_result = jnz.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            jnz_taken_result = jnz_taken.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            jump_result = jump.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            jump_double_deref_result = jump_double_deref.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            jump_rel_result = jump_rel.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            jump_rel_imm_result = jump_rel_imm.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            mul_result = mul.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                        rc_20_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            mul_small_result = mul_small.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                        rc_11_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            qm31_result = qm31.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                        rc_4_4_4_4_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            ret_result = ret.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });
    });

    // Convert results: extract traces as TraceEval and separate out claims
    let (add_converted, add_claims) = add_result
        .map(|(trace, claim, interaction_gen)| {
            (Some((trace.to_evals(), interaction_gen)), vec![claim])
        })
        .unwrap_or((None, vec![]));
    let (add_small_converted, add_small_claims) = add_small_result
        .map(|(trace, claim, interaction_gen)| {
            (Some((trace.to_evals(), interaction_gen)), vec![claim])
        })
        .unwrap_or((None, vec![]));
    let (add_ap_converted, add_ap_claims) = add_ap_result
        .map(|(trace, claim, interaction_gen)| {
            (Some((trace.to_evals(), interaction_gen)), vec![claim])
        })
        .unwrap_or((None, vec![]));
    let (assert_eq_converted, assert_eq_claims) = assert_eq_result
        .map(|(trace, claim, interaction_gen)| {
            (Some((trace.to_evals(), interaction_gen)), vec![claim])
        })
        .unwrap_or((None, vec![]));
    let (assert_eq_imm_converted, assert_eq_imm_claims) = assert_eq_imm_result
        .map(|(trace, claim, interaction_gen)| {
            (Some((trace.to_evals(), interaction_gen)), vec![claim])
        })
        .unwrap_or((None, vec![]));
    let (assert_eq_double_deref_converted, assert_eq_double_deref_claims) =
        assert_eq_double_deref_result
            .map(|(trace, claim, interaction_gen)| {
                (Some((trace.to_evals(), interaction_gen)), vec![claim])
            })
            .unwrap_or((None, vec![]));
    let (call_converted, call_claims) = call_result
        .map(|(trace, claim, interaction_gen)| {
            (Some((trace.to_evals(), interaction_gen)), vec![claim])
        })
        .unwrap_or((None, vec![]));
    let (call_rel_imm_converted, call_rel_imm_claims) = call_rel_imm_result
        .map(|(trace, claim, interaction_gen)| {
            (Some((trace.to_evals(), interaction_gen)), vec![claim])
        })
        .unwrap_or((None, vec![]));
    let (generic_converted, generic_claims) = generic_result
        .map(|(trace, claim, interaction_gen)| {
            (Some((trace.to_evals(), interaction_gen)), vec![claim])
        })
        .unwrap_or((None, vec![]));
    let (jnz_converted, jnz_claims) = jnz_result
        .map(|(trace, claim, interaction_gen)| {
            (Some((trace.to_evals(), interaction_gen)), vec![claim])
        })
        .unwrap_or((None, vec![]));
    let (jnz_taken_converted, jnz_taken_claims) = jnz_taken_result
        .map(|(trace, claim, interaction_gen)| {
            (Some((trace.to_evals(), interaction_gen)), vec![claim])
        })
        .unwrap_or((None, vec![]));
    let (jump_converted, jump_claims) = jump_result
        .map(|(trace, claim, interaction_gen)| {
            (Some((trace.to_evals(), interaction_gen)), vec![claim])
        })
        .unwrap_or((None, vec![]));
    let (jump_double_deref_converted, jump_double_deref_claims) = jump_double_deref_result
        .map(|(trace, claim, interaction_gen)| {
            (Some((trace.to_evals(), interaction_gen)), vec![claim])
        })
        .unwrap_or((None, vec![]));
    let (jump_rel_converted, jump_rel_claims) = jump_rel_result
        .map(|(trace, claim, interaction_gen)| {
            (Some((trace.to_evals(), interaction_gen)), vec![claim])
        })
        .unwrap_or((None, vec![]));
    let (jump_rel_imm_converted, jump_rel_imm_claims) = jump_rel_imm_result
        .map(|(trace, claim, interaction_gen)| {
            (Some((trace.to_evals(), interaction_gen)), vec![claim])
        })
        .unwrap_or((None, vec![]));
    let (mul_converted, mul_claims) = mul_result
        .map(|(trace, claim, interaction_gen)| {
            (Some((trace.to_evals(), interaction_gen)), vec![claim])
        })
        .unwrap_or((None, vec![]));
    let (mul_small_converted, mul_small_claims) = mul_small_result
        .map(|(trace, claim, interaction_gen)| {
            (Some((trace.to_evals(), interaction_gen)), vec![claim])
        })
        .unwrap_or((None, vec![]));
    let (qm31_converted, qm31_claims) = qm31_result
        .map(|(trace, claim, interaction_gen)| {
            (Some((trace.to_evals(), interaction_gen)), vec![claim])
        })
        .unwrap_or((None, vec![]));
    let (ret_converted, ret_claims) = ret_result
        .map(|(trace, claim, interaction_gen)| {
            (Some((trace.to_evals(), interaction_gen)), vec![claim])
        })
        .unwrap_or((None, vec![]));

    PreBlakeOpcodeResult {
        add_result: add_converted,
        add_small_result: add_small_converted,
        add_ap_result: add_ap_converted,
        assert_eq_result: assert_eq_converted,
        assert_eq_imm_result: assert_eq_imm_converted,
        assert_eq_double_deref_result: assert_eq_double_deref_converted,
        call_result: call_converted,
        call_rel_imm_result: call_rel_imm_converted,
        generic_result: generic_converted,
        jnz_result: jnz_converted,
        jnz_taken_result: jnz_taken_converted,
        jump_result: jump_converted,
        jump_double_deref_result: jump_double_deref_converted,
        jump_rel_result: jump_rel_converted,
        jump_rel_imm_result: jump_rel_imm_converted,
        mul_result: mul_converted,
        mul_small_result: mul_small_converted,
        qm31_result: qm31_converted,
        ret_result: ret_converted,
        add_claims,
        add_small_claims,
        add_ap_claims,
        assert_eq_claims,
        assert_eq_imm_claims,
        assert_eq_double_deref_claims,
        call_claims,
        call_rel_imm_claims,
        generic_claims,
        jnz_claims,
        jnz_taken_claims,
        jump_claims,
        jump_double_deref_claims,
        jump_rel_claims,
        jump_rel_imm_claims,
        mul_claims,
        mul_small_claims,
        qm31_claims,
        ret_claims,
    }
}

/// Complete opcode write_trace by processing blake and extending all traces to tree_builder.
#[allow(clippy::too_many_arguments)]
pub fn complete_opcodes_with_blake(
    pre_blake: PreBlakeOpcodeResult,
    blake: Option<blake_compress_opcode::ClaimGenerator>,
    tree_builder: &mut impl TreeBuilder<SimdBackend>,
    blake_round: &mut Option<blake_round::ClaimGenerator>,
    triple_xor_32: &mut Option<triple_xor_32::ClaimGenerator>,
    memory_address_to_id_trace_generator: Option<&memory_address_to_id::ClaimGenerator>,
    memory_id_to_value_trace_generator: Option<&memory_id_to_big::ClaimGenerator>,
    rc_7_2_5_trace_generator: Option<&range_check_7_2_5::ClaimGenerator>,
    verify_instruction_trace_generator: Option<&verify_instruction::ClaimGenerator>,
    verify_bitwise_xor_8_trace_generator: Option<&mut verify_bitwise_xor_8::ClaimGenerator>,
) -> (OpcodeClaim, OpcodesInteractionClaimGenerator) {
    // Extract pre-blake results (traces and interaction generators)
    let (add_trace, add_interaction_gens) = pre_blake
        .add_result
        .map(|(trace, interaction_gen)| (Some(trace), vec![interaction_gen]))
        .unwrap_or_default();
    let (add_small_trace, add_small_interaction_gens) = pre_blake
        .add_small_result
        .map(|(trace, interaction_gen)| (Some(trace), vec![interaction_gen]))
        .unwrap_or_default();
    let (add_ap_trace, add_ap_interaction_gens) = pre_blake
        .add_ap_result
        .map(|(trace, interaction_gen)| (Some(trace), vec![interaction_gen]))
        .unwrap_or_default();
    let (assert_eq_trace, assert_eq_interaction_gens) = pre_blake
        .assert_eq_result
        .map(|(trace, interaction_gen)| (Some(trace), vec![interaction_gen]))
        .unwrap_or_default();
    let (assert_eq_imm_trace, assert_eq_imm_interaction_gens) = pre_blake
        .assert_eq_imm_result
        .map(|(trace, interaction_gen)| (Some(trace), vec![interaction_gen]))
        .unwrap_or_default();
    let (assert_eq_double_deref_trace, assert_eq_double_deref_interaction_gens) = pre_blake
        .assert_eq_double_deref_result
        .map(|(trace, interaction_gen)| (Some(trace), vec![interaction_gen]))
        .unwrap_or_default();

    // Extract claims from separate vectors
    let add_claims = pre_blake.add_claims;
    let add_small_claims = pre_blake.add_small_claims;
    let add_ap_claims = pre_blake.add_ap_claims;
    let assert_eq_claims = pre_blake.assert_eq_claims;
    let assert_eq_imm_claims = pre_blake.assert_eq_imm_claims;
    let assert_eq_double_deref_claims = pre_blake.assert_eq_double_deref_claims;

    // Process blake (needs mutable references)
    let (blake_trace, blake_claims, blake_interaction_gens) = blake
        .map(|gen| {
            let (trace, claim, interaction_gen) = gen.write_trace(
                memory_address_to_id_trace_generator.unwrap(),
                memory_id_to_value_trace_generator.unwrap(),
                verify_instruction_trace_generator.unwrap(),
                rc_7_2_5_trace_generator.unwrap(),
                verify_bitwise_xor_8_trace_generator.unwrap(),
                blake_round.as_mut().unwrap(),
                triple_xor_32.as_mut().unwrap(),
            );
            (Some(trace), vec![claim], vec![interaction_gen])
        })
        .unwrap_or_default();

    let (call_trace, call_interaction_gens) = pre_blake
        .call_result
        .map(|(trace, interaction_gen)| (Some(trace), vec![interaction_gen]))
        .unwrap_or_default();
    let (call_rel_imm_trace, call_rel_imm_interaction_gens) = pre_blake
        .call_rel_imm_result
        .map(|(trace, interaction_gen)| (Some(trace), vec![interaction_gen]))
        .unwrap_or_default();
    let (generic_opcode_trace, generic_opcode_interaction_gens) = pre_blake
        .generic_result
        .map(|(trace, interaction_gen)| (Some(trace), vec![interaction_gen]))
        .unwrap_or_default();
    let (jnz_trace, jnz_interaction_gens) = pre_blake
        .jnz_result
        .map(|(trace, interaction_gen)| (Some(trace), vec![interaction_gen]))
        .unwrap_or_default();
    let (jnz_taken_trace, jnz_taken_interaction_gens) = pre_blake
        .jnz_taken_result
        .map(|(trace, interaction_gen)| (Some(trace), vec![interaction_gen]))
        .unwrap_or_default();
    let (jump_trace, jump_interaction_gens) = pre_blake
        .jump_result
        .map(|(trace, interaction_gen)| (Some(trace), vec![interaction_gen]))
        .unwrap_or_default();
    let (jump_double_deref_trace, jump_double_deref_interaction_gens) = pre_blake
        .jump_double_deref_result
        .map(|(trace, interaction_gen)| (Some(trace), vec![interaction_gen]))
        .unwrap_or_default();
    let (jump_rel_trace, jump_rel_interaction_gens) = pre_blake
        .jump_rel_result
        .map(|(trace, interaction_gen)| (Some(trace), vec![interaction_gen]))
        .unwrap_or_default();
    let (jump_rel_imm_trace, jump_rel_imm_interaction_gens) = pre_blake
        .jump_rel_imm_result
        .map(|(trace, interaction_gen)| (Some(trace), vec![interaction_gen]))
        .unwrap_or_default();
    let (mul_trace, mul_interaction_gens) = pre_blake
        .mul_result
        .map(|(trace, interaction_gen)| (Some(trace), vec![interaction_gen]))
        .unwrap_or_default();
    let (mul_small_trace, mul_small_interaction_gens) = pre_blake
        .mul_small_result
        .map(|(trace, interaction_gen)| (Some(trace), vec![interaction_gen]))
        .unwrap_or_default();
    let (qm31_trace, qm31_interaction_gens) = pre_blake
        .qm31_result
        .map(|(trace, interaction_gen)| (Some(trace), vec![interaction_gen]))
        .unwrap_or_default();
    let (ret_trace, ret_interaction_gens) = pre_blake
        .ret_result
        .map(|(trace, interaction_gen)| (Some(trace), vec![interaction_gen]))
        .unwrap_or_default();

    // Extract remaining claims from separate vectors
    let call_claims = pre_blake.call_claims;
    let call_rel_imm_claims = pre_blake.call_rel_imm_claims;
    let generic_opcode_claims = pre_blake.generic_claims;
    let jnz_claims = pre_blake.jnz_claims;
    let jnz_taken_claims = pre_blake.jnz_taken_claims;
    let jump_claims = pre_blake.jump_claims;
    let jump_double_deref_claims = pre_blake.jump_double_deref_claims;
    let jump_rel_claims = pre_blake.jump_rel_claims;
    let jump_rel_imm_claims = pre_blake.jump_rel_imm_claims;
    let mul_claims = pre_blake.mul_claims;
    let mul_small_claims = pre_blake.mul_small_claims;
    let qm31_claims = pre_blake.qm31_claims;
    let ret_claims = pre_blake.ret_claims;

    // Extend tree_builder with all traces in the original order
    // (pre-blake traces are already TraceEval, no need for .to_evals())
    if let Some(trace) = add_trace {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = add_small_trace {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = add_ap_trace {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = assert_eq_trace {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = assert_eq_imm_trace {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = assert_eq_double_deref_trace {
        tree_builder.extend_evals(trace);
    }
    // blake_trace is still ComponentTrace, needs .to_evals()
    if let Some(trace) = blake_trace {
        tree_builder.extend_evals(trace.to_evals());
    }
    if let Some(trace) = call_trace {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = call_rel_imm_trace {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = generic_opcode_trace {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = jnz_trace {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = jnz_taken_trace {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = jump_trace {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = jump_double_deref_trace {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = jump_rel_trace {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = jump_rel_imm_trace {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = mul_trace {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = mul_small_trace {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = qm31_trace {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = ret_trace {
        tree_builder.extend_evals(trace);
    }

    (
        OpcodeClaim {
            add: add_claims,
            add_small: add_small_claims,
            add_ap: add_ap_claims,
            assert_eq: assert_eq_claims,
            assert_eq_imm: assert_eq_imm_claims,
            assert_eq_double_deref: assert_eq_double_deref_claims,
            blake: blake_claims,
            call: call_claims,
            call_rel_imm: call_rel_imm_claims,
            generic: generic_opcode_claims,
            jnz: jnz_claims,
            jnz_taken: jnz_taken_claims,
            jump: jump_claims,
            jump_double_deref: jump_double_deref_claims,
            jump_rel: jump_rel_claims,
            jump_rel_imm: jump_rel_imm_claims,
            mul: mul_claims,
            mul_small: mul_small_claims,
            qm31: qm31_claims,
            ret: ret_claims,
        },
        OpcodesInteractionClaimGenerator {
            add: add_interaction_gens,
            add_small: add_small_interaction_gens,
            add_ap: add_ap_interaction_gens,
            assert_eq: assert_eq_interaction_gens,
            assert_eq_imm: assert_eq_imm_interaction_gens,
            assert_eq_double_deref: assert_eq_double_deref_interaction_gens,
            blake: blake_interaction_gens,
            call: call_interaction_gens,
            call_rel_imm: call_rel_imm_interaction_gens,
            generic_opcode_interaction_gens,
            jnz: jnz_interaction_gens,
            jnz_taken: jnz_taken_interaction_gens,
            jump: jump_interaction_gens,
            jump_double_deref: jump_double_deref_interaction_gens,
            jump_rel: jump_rel_interaction_gens,
            jump_rel_imm: jump_rel_imm_interaction_gens,
            mul: mul_interaction_gens,
            mul_small: mul_small_interaction_gens,
            qm31: qm31_interaction_gens,
            ret_interaction_gens,
        },
    )
}

pub fn opcodes_write_trace(
    add: Option<add_opcode::ClaimGenerator>,
    add_small: Option<add_opcode_small::ClaimGenerator>,
    add_ap: Option<add_ap_opcode::ClaimGenerator>,
    assert_eq: Option<assert_eq_opcode::ClaimGenerator>,
    assert_eq_imm: Option<assert_eq_opcode_imm::ClaimGenerator>,
    assert_eq_double_deref: Option<assert_eq_opcode_double_deref::ClaimGenerator>,
    blake: Option<blake_compress_opcode::ClaimGenerator>,
    call: Option<call_opcode_abs::ClaimGenerator>,
    call_rel_imm: Option<call_opcode_rel_imm::ClaimGenerator>,
    generic: Option<generic_opcode::ClaimGenerator>,
    jnz: Option<jnz_opcode_non_taken::ClaimGenerator>,
    jnz_taken: Option<jnz_opcode_taken::ClaimGenerator>,
    jump: Option<jump_opcode_abs::ClaimGenerator>,
    jump_double_deref: Option<jump_opcode_double_deref::ClaimGenerator>,
    jump_rel: Option<jump_opcode_rel::ClaimGenerator>,
    jump_rel_imm: Option<jump_opcode_rel_imm::ClaimGenerator>,
    mul: Option<mul_opcode::ClaimGenerator>,
    mul_small: Option<mul_opcode_small::ClaimGenerator>,
    qm31: Option<qm_31_add_mul_opcode::ClaimGenerator>,
    ret: Option<ret_opcode::ClaimGenerator>,
    tree_builder: &mut impl TreeBuilder<SimdBackend>,
    blake_round: &mut Option<blake_round::ClaimGenerator>,
    triple_xor_32: &mut Option<triple_xor_32::ClaimGenerator>,
    memory_address_to_id_trace_generator: Option<&memory_address_to_id::ClaimGenerator>,
    memory_id_to_value_trace_generator: Option<&memory_id_to_big::ClaimGenerator>,
    rc_7_2_5_trace_generator: Option<&range_check_7_2_5::ClaimGenerator>,
    rc_11_trace_generator: Option<&range_check_11::ClaimGenerator>,
    rc_18_trace_generator: Option<&range_check_18::ClaimGenerator>,
    rc_20_trace_generator: Option<&range_check_20::ClaimGenerator>,
    rc_4_4_4_4_trace_generator: Option<&range_check_4_4_4_4::ClaimGenerator>,
    rc_9_9_trace_generator: Option<&range_check_9_9::ClaimGenerator>,
    verify_instruction_trace_generator: Option<&verify_instruction::ClaimGenerator>,
    verify_bitwise_xor_8_trace_generator: Option<&mut verify_bitwise_xor_8::ClaimGenerator>,
) -> (OpcodeClaim, OpcodesInteractionClaimGenerator) {
    // Result holders for parallel execution
    let mut add_result = None;
    let mut add_small_result = None;
    let mut add_ap_result = None;
    let mut assert_eq_result = None;
    let mut assert_eq_imm_result = None;
    let mut assert_eq_double_deref_result = None;
    let mut call_result = None;
    let mut call_rel_imm_result = None;
    let mut generic_result = None;
    let mut jnz_result = None;
    let mut jnz_taken_result = None;
    let mut jump_result = None;
    let mut jump_double_deref_result = None;
    let mut jump_rel_result = None;
    let mut jump_rel_imm_result = None;
    let mut mul_result = None;
    let mut mul_small_result = None;
    let mut qm31_result = None;
    let mut ret_result = None;

    // Run all non-blake opcodes in parallel.
    // Blake is processed separately because it needs mutable access to blake_round,
    // triple_xor_32, and verify_bitwise_xor_8_trace_generator.
    scope(|s| {
        s.spawn(|_| {
            add_result = add.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            add_small_result = add_small.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            add_ap_result = add_ap.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                        rc_18_trace_generator.unwrap(),
                        rc_11_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            assert_eq_result = assert_eq.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            assert_eq_imm_result = assert_eq_imm.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            assert_eq_double_deref_result = assert_eq_double_deref.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            call_result = call.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            call_rel_imm_result = call_rel_imm.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            generic_result = generic.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                        rc_9_9_trace_generator.unwrap(),
                        rc_20_trace_generator.unwrap(),
                        rc_18_trace_generator.unwrap(),
                        rc_11_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            jnz_result = jnz.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            jnz_taken_result = jnz_taken.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            jump_result = jump.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            jump_double_deref_result = jump_double_deref.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            jump_rel_result = jump_rel.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            jump_rel_imm_result = jump_rel_imm.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            mul_result = mul.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                        rc_20_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            mul_small_result = mul_small.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                        rc_11_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            qm31_result = qm31.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                        rc_4_4_4_4_trace_generator.unwrap(),
                    )
                })
            });
        });

        s.spawn(|_| {
            ret_result = ret.map(|gen| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                pool.install(|| {
                    gen.write_trace(
                        memory_address_to_id_trace_generator.unwrap(),
                        memory_id_to_value_trace_generator.unwrap(),
                        verify_instruction_trace_generator.unwrap(),
                    )
                })
            });
        });
    });

    // Extract results and build claims/interaction_gens
    let (add_trace, add_claims, add_interaction_gens) = add_result
        .map(|(trace, claim, interaction_gen)| (Some(trace), vec![claim], vec![interaction_gen]))
        .unwrap_or_default();
    let (add_small_trace, add_small_claims, add_small_interaction_gens) = add_small_result
        .map(|(trace, claim, interaction_gen)| (Some(trace), vec![claim], vec![interaction_gen]))
        .unwrap_or_default();
    let (add_ap_trace, add_ap_claims, add_ap_interaction_gens) = add_ap_result
        .map(|(trace, claim, interaction_gen)| (Some(trace), vec![claim], vec![interaction_gen]))
        .unwrap_or_default();
    let (assert_eq_trace, assert_eq_claims, assert_eq_interaction_gens) = assert_eq_result
        .map(|(trace, claim, interaction_gen)| (Some(trace), vec![claim], vec![interaction_gen]))
        .unwrap_or_default();
    let (assert_eq_imm_trace, assert_eq_imm_claims, assert_eq_imm_interaction_gens) =
        assert_eq_imm_result
            .map(|(trace, claim, interaction_gen)| {
                (Some(trace), vec![claim], vec![interaction_gen])
            })
            .unwrap_or_default();
    let (
        assert_eq_double_deref_trace,
        assert_eq_double_deref_claims,
        assert_eq_double_deref_interaction_gens,
    ) = assert_eq_double_deref_result
        .map(|(trace, claim, interaction_gen)| (Some(trace), vec![claim], vec![interaction_gen]))
        .unwrap_or_default();

    // Blake processed separately (needs mutable references)
    let (blake_trace, blake_claims, blake_interaction_gens) = blake
        .map(|gen| {
            let (trace, claim, interaction_gen) = gen.write_trace(
                memory_address_to_id_trace_generator.unwrap(),
                memory_id_to_value_trace_generator.unwrap(),
                verify_instruction_trace_generator.unwrap(),
                rc_7_2_5_trace_generator.unwrap(),
                verify_bitwise_xor_8_trace_generator.unwrap(),
                blake_round.as_mut().unwrap(),
                triple_xor_32.as_mut().unwrap(),
            );
            (Some(trace), vec![claim], vec![interaction_gen])
        })
        .unwrap_or_default();

    let (call_trace, call_claims, call_interaction_gens) = call_result
        .map(|(trace, claim, interaction_gen)| (Some(trace), vec![claim], vec![interaction_gen]))
        .unwrap_or_default();
    let (call_rel_imm_trace, call_rel_imm_claims, call_rel_imm_interaction_gens) =
        call_rel_imm_result
            .map(|(trace, claim, interaction_gen)| {
                (Some(trace), vec![claim], vec![interaction_gen])
            })
            .unwrap_or_default();
    let (generic_opcode_trace, generic_opcode_claims, generic_opcode_interaction_gens) =
        generic_result
            .map(|(trace, claim, interaction_gen)| {
                (Some(trace), vec![claim], vec![interaction_gen])
            })
            .unwrap_or_default();
    let (jnz_trace, jnz_claims, jnz_interaction_gens) = jnz_result
        .map(|(trace, claim, interaction_gen)| (Some(trace), vec![claim], vec![interaction_gen]))
        .unwrap_or_default();
    let (jnz_taken_trace, jnz_taken_claims, jnz_taken_interaction_gens) = jnz_taken_result
        .map(|(trace, claim, interaction_gen)| (Some(trace), vec![claim], vec![interaction_gen]))
        .unwrap_or_default();
    let (jump_trace, jump_claims, jump_interaction_gens) = jump_result
        .map(|(trace, claim, interaction_gen)| (Some(trace), vec![claim], vec![interaction_gen]))
        .unwrap_or_default();
    let (jump_double_deref_trace, jump_double_deref_claims, jump_double_deref_interaction_gens) =
        jump_double_deref_result
            .map(|(trace, claim, interaction_gen)| {
                (Some(trace), vec![claim], vec![interaction_gen])
            })
            .unwrap_or_default();
    let (jump_rel_trace, jump_rel_claims, jump_rel_interaction_gens) = jump_rel_result
        .map(|(trace, claim, interaction_gen)| (Some(trace), vec![claim], vec![interaction_gen]))
        .unwrap_or_default();
    let (jump_rel_imm_trace, jump_rel_imm_claims, jump_rel_imm_interaction_gens) =
        jump_rel_imm_result
            .map(|(trace, claim, interaction_gen)| {
                (Some(trace), vec![claim], vec![interaction_gen])
            })
            .unwrap_or_default();
    let (mul_trace, mul_claims, mul_interaction_gens) = mul_result
        .map(|(trace, claim, interaction_gen)| (Some(trace), vec![claim], vec![interaction_gen]))
        .unwrap_or_default();
    let (mul_small_trace, mul_small_claims, mul_small_interaction_gens) = mul_small_result
        .map(|(trace, claim, interaction_gen)| (Some(trace), vec![claim], vec![interaction_gen]))
        .unwrap_or_default();
    let (qm31_trace, qm31_claims, qm31_interaction_gens) = qm31_result
        .map(|(trace, claim, interaction_gen)| (Some(trace), vec![claim], vec![interaction_gen]))
        .unwrap_or_default();
    let (ret_trace, ret_claims, ret_interaction_gens) = ret_result
        .map(|(trace, claim, interaction_gen)| (Some(trace), vec![claim], vec![interaction_gen]))
        .unwrap_or_default();

    // Extend tree_builder with all traces in the original order
    if let Some(trace) = add_trace {
        tree_builder.extend_evals(trace.to_evals());
    }
    if let Some(trace) = add_small_trace {
        tree_builder.extend_evals(trace.to_evals());
    }
    if let Some(trace) = add_ap_trace {
        tree_builder.extend_evals(trace.to_evals());
    }
    if let Some(trace) = assert_eq_trace {
        tree_builder.extend_evals(trace.to_evals());
    }
    if let Some(trace) = assert_eq_imm_trace {
        tree_builder.extend_evals(trace.to_evals());
    }
    if let Some(trace) = assert_eq_double_deref_trace {
        tree_builder.extend_evals(trace.to_evals());
    }
    if let Some(trace) = blake_trace {
        tree_builder.extend_evals(trace.to_evals());
    }
    if let Some(trace) = call_trace {
        tree_builder.extend_evals(trace.to_evals());
    }
    if let Some(trace) = call_rel_imm_trace {
        tree_builder.extend_evals(trace.to_evals());
    }
    if let Some(trace) = generic_opcode_trace {
        tree_builder.extend_evals(trace.to_evals());
    }
    if let Some(trace) = jnz_trace {
        tree_builder.extend_evals(trace.to_evals());
    }
    if let Some(trace) = jnz_taken_trace {
        tree_builder.extend_evals(trace.to_evals());
    }
    if let Some(trace) = jump_trace {
        tree_builder.extend_evals(trace.to_evals());
    }
    if let Some(trace) = jump_double_deref_trace {
        tree_builder.extend_evals(trace.to_evals());
    }
    if let Some(trace) = jump_rel_trace {
        tree_builder.extend_evals(trace.to_evals());
    }
    if let Some(trace) = jump_rel_imm_trace {
        tree_builder.extend_evals(trace.to_evals());
    }
    if let Some(trace) = mul_trace {
        tree_builder.extend_evals(trace.to_evals());
    }
    if let Some(trace) = mul_small_trace {
        tree_builder.extend_evals(trace.to_evals());
    }
    if let Some(trace) = qm31_trace {
        tree_builder.extend_evals(trace.to_evals());
    }
    if let Some(trace) = ret_trace {
        tree_builder.extend_evals(trace.to_evals());
    }
    (
        OpcodeClaim {
            add: add_claims,
            add_small: add_small_claims,
            add_ap: add_ap_claims,
            assert_eq: assert_eq_claims,
            assert_eq_imm: assert_eq_imm_claims,
            assert_eq_double_deref: assert_eq_double_deref_claims,
            blake: blake_claims,
            call: call_claims,
            call_rel_imm: call_rel_imm_claims,
            generic: generic_opcode_claims,
            jnz: jnz_claims,
            jnz_taken: jnz_taken_claims,
            jump: jump_claims,
            jump_double_deref: jump_double_deref_claims,
            jump_rel: jump_rel_claims,
            jump_rel_imm: jump_rel_imm_claims,
            mul: mul_claims,
            mul_small: mul_small_claims,
            qm31: qm31_claims,
            ret: ret_claims,
        },
        OpcodesInteractionClaimGenerator {
            add: add_interaction_gens,
            add_small: add_small_interaction_gens,
            add_ap: add_ap_interaction_gens,
            assert_eq: assert_eq_interaction_gens,
            assert_eq_imm: assert_eq_imm_interaction_gens,
            assert_eq_double_deref: assert_eq_double_deref_interaction_gens,
            blake: blake_interaction_gens,
            call: call_interaction_gens,
            call_rel_imm: call_rel_imm_interaction_gens,
            generic_opcode_interaction_gens,
            jnz: jnz_interaction_gens,
            jnz_taken: jnz_taken_interaction_gens,
            jump: jump_interaction_gens,
            jump_double_deref: jump_double_deref_interaction_gens,
            jump_rel: jump_rel_interaction_gens,
            jump_rel_imm: jump_rel_imm_interaction_gens,
            mul: mul_interaction_gens,
            mul_small: mul_small_interaction_gens,
            qm31: qm31_interaction_gens,
            ret_interaction_gens,
        },
    )
}

pub struct OpcodesInteractionClaimGenerator {
    pub add: Vec<add_opcode::InteractionClaimGenerator>,
    pub add_small: Vec<add_opcode_small::InteractionClaimGenerator>,
    pub add_ap: Vec<add_ap_opcode::InteractionClaimGenerator>,
    pub assert_eq: Vec<assert_eq_opcode::InteractionClaimGenerator>,
    pub assert_eq_imm: Vec<assert_eq_opcode_imm::InteractionClaimGenerator>,
    pub assert_eq_double_deref: Vec<assert_eq_opcode_double_deref::InteractionClaimGenerator>,
    pub blake: Vec<blake_compress_opcode::InteractionClaimGenerator>,
    pub call: Vec<call_opcode_abs::InteractionClaimGenerator>,
    pub call_rel_imm: Vec<call_opcode_rel_imm::InteractionClaimGenerator>,
    pub generic_opcode_interaction_gens: Vec<generic_opcode::InteractionClaimGenerator>,
    pub jnz: Vec<jnz_opcode_non_taken::InteractionClaimGenerator>,
    pub jnz_taken: Vec<jnz_opcode_taken::InteractionClaimGenerator>,
    pub jump: Vec<jump_opcode_abs::InteractionClaimGenerator>,
    pub jump_double_deref: Vec<jump_opcode_double_deref::InteractionClaimGenerator>,
    pub jump_rel: Vec<jump_opcode_rel::InteractionClaimGenerator>,
    pub jump_rel_imm: Vec<jump_opcode_rel_imm::InteractionClaimGenerator>,
    pub mul: Vec<mul_opcode::InteractionClaimGenerator>,
    pub mul_small: Vec<mul_opcode_small::InteractionClaimGenerator>,
    pub qm31: Vec<qm_31_add_mul_opcode::InteractionClaimGenerator>,
    pub ret_interaction_gens: Vec<ret_opcode::InteractionClaimGenerator>,
}
