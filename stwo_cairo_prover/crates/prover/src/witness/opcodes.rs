use stwo_cairo_adapter::opcodes::CasmStatesByOpcode;

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
<<<<<<< HEAD

#[allow(clippy::type_complexity)]
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
) -> (
    Option<add_opcode_claim::Claim>,
    Option<add_opcode_small_claim::Claim>,
    Option<add_ap_opcode_claim::Claim>,
    Option<assert_eq_opcode_claim::Claim>,
    Option<assert_eq_opcode_imm_claim::Claim>,
    Option<assert_eq_opcode_double_deref_claim::Claim>,
    Option<blake_compress_opcode_claim::Claim>,
    Option<call_opcode_abs_claim::Claim>,
    Option<call_opcode_rel_imm_claim::Claim>,
    Option<generic_opcode_claim::Claim>,
    Option<jnz_opcode_non_taken_claim::Claim>,
    Option<jnz_opcode_taken_claim::Claim>,
    Option<jump_opcode_abs_claim::Claim>,
    Option<jump_opcode_double_deref_claim::Claim>,
    Option<jump_opcode_rel_claim::Claim>,
    Option<jump_opcode_rel_imm_claim::Claim>,
    Option<mul_opcode_claim::Claim>,
    Option<mul_opcode_small_claim::Claim>,
    Option<qm_31_add_mul_opcode_claim::Claim>,
    Option<ret_opcode_claim::Claim>,
    OpcodesInteractionClaimGenerator,
) {
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
    scope(|s| {
        s.spawn(|_| {
            add_result = add.map(|gen| {
                gen.write_trace(
                    memory_address_to_id_trace_generator.unwrap(),
                    memory_id_to_value_trace_generator.unwrap(),
                    verify_instruction_trace_generator.unwrap(),
                )
            });
        });

        s.spawn(|_| {
            add_small_result = add_small.map(|gen| {
                gen.write_trace(
                    memory_address_to_id_trace_generator.unwrap(),
                    memory_id_to_value_trace_generator.unwrap(),
                    verify_instruction_trace_generator.unwrap(),
                )
            });
        });

        s.spawn(|_| {
            add_ap_result = add_ap.map(|gen| {
                gen.write_trace(
                    memory_address_to_id_trace_generator.unwrap(),
                    memory_id_to_value_trace_generator.unwrap(),
                    verify_instruction_trace_generator.unwrap(),
                    rc_18_trace_generator.unwrap(),
                    rc_11_trace_generator.unwrap(),
                )
            });
        });

        s.spawn(|_| {
            assert_eq_result = assert_eq.map(|gen| {
                gen.write_trace(
                    memory_address_to_id_trace_generator.unwrap(),
                    memory_id_to_value_trace_generator.unwrap(),
                    verify_instruction_trace_generator.unwrap(),
                )
            });
        });

        s.spawn(|_| {
            assert_eq_imm_result = assert_eq_imm.map(|gen| {
                gen.write_trace(
                    memory_address_to_id_trace_generator.unwrap(),
                    memory_id_to_value_trace_generator.unwrap(),
                    verify_instruction_trace_generator.unwrap(),
                )
            });
        });

        s.spawn(|_| {
            assert_eq_double_deref_result = assert_eq_double_deref.map(|gen| {
                gen.write_trace(
                    memory_address_to_id_trace_generator.unwrap(),
                    memory_id_to_value_trace_generator.unwrap(),
                    verify_instruction_trace_generator.unwrap(),
                )
            });
        });

        s.spawn(|_| {
            call_result = call.map(|gen| {
                gen.write_trace(
                    memory_address_to_id_trace_generator.unwrap(),
                    memory_id_to_value_trace_generator.unwrap(),
                    verify_instruction_trace_generator.unwrap(),
                )
            });
        });

        s.spawn(|_| {
            call_rel_imm_result = call_rel_imm.map(|gen| {
                gen.write_trace(
                    memory_address_to_id_trace_generator.unwrap(),
                    memory_id_to_value_trace_generator.unwrap(),
                    verify_instruction_trace_generator.unwrap(),
                )
            });
        });

        s.spawn(|_| {
            generic_result = generic.map(|gen| {
                gen.write_trace(
                    memory_address_to_id_trace_generator.unwrap(),
                    memory_id_to_value_trace_generator.unwrap(),
                    verify_instruction_trace_generator.unwrap(),
                    rc_9_9_trace_generator.unwrap(),
                    rc_20_trace_generator.unwrap(),
                    rc_18_trace_generator.unwrap(),
                    rc_11_trace_generator.unwrap(),
                )
            });
        });

        s.spawn(|_| {
            jnz_result = jnz.map(|gen| {
                gen.write_trace(
                    memory_address_to_id_trace_generator.unwrap(),
                    memory_id_to_value_trace_generator.unwrap(),
                    verify_instruction_trace_generator.unwrap(),
                )
            });
        });

        s.spawn(|_| {
            jnz_taken_result = jnz_taken.map(|gen| {
                gen.write_trace(
                    memory_address_to_id_trace_generator.unwrap(),
                    memory_id_to_value_trace_generator.unwrap(),
                    verify_instruction_trace_generator.unwrap(),
                )
            });
        });

        s.spawn(|_| {
            jump_result = jump.map(|gen| {
                gen.write_trace(
                    memory_address_to_id_trace_generator.unwrap(),
                    memory_id_to_value_trace_generator.unwrap(),
                    verify_instruction_trace_generator.unwrap(),
                )
            });
        });

        s.spawn(|_| {
            jump_double_deref_result = jump_double_deref.map(|gen| {
                gen.write_trace(
                    memory_address_to_id_trace_generator.unwrap(),
                    memory_id_to_value_trace_generator.unwrap(),
                    verify_instruction_trace_generator.unwrap(),
                )
            });
        });

        s.spawn(|_| {
            jump_rel_result = jump_rel.map(|gen| {
                gen.write_trace(
                    memory_address_to_id_trace_generator.unwrap(),
                    memory_id_to_value_trace_generator.unwrap(),
                    verify_instruction_trace_generator.unwrap(),
                )
            });
        });

        s.spawn(|_| {
            jump_rel_imm_result = jump_rel_imm.map(|gen| {
                gen.write_trace(
                    memory_address_to_id_trace_generator.unwrap(),
                    memory_id_to_value_trace_generator.unwrap(),
                    verify_instruction_trace_generator.unwrap(),
                )
            });
        });

        s.spawn(|_| {
            mul_result = mul.map(|gen| {
                gen.write_trace(
                    memory_address_to_id_trace_generator.unwrap(),
                    memory_id_to_value_trace_generator.unwrap(),
                    verify_instruction_trace_generator.unwrap(),
                    rc_20_trace_generator.unwrap(),
                )
            });
        });

        s.spawn(|_| {
            mul_small_result = mul_small.map(|gen| {
                gen.write_trace(
                    memory_address_to_id_trace_generator.unwrap(),
                    memory_id_to_value_trace_generator.unwrap(),
                    verify_instruction_trace_generator.unwrap(),
                    rc_11_trace_generator.unwrap(),
                )
            });
        });

        s.spawn(|_| {
            qm31_result = qm31.map(|gen| {
                gen.write_trace(
                    memory_address_to_id_trace_generator.unwrap(),
                    memory_id_to_value_trace_generator.unwrap(),
                    verify_instruction_trace_generator.unwrap(),
                    rc_4_4_4_4_trace_generator.unwrap(),
                )
            });
        });

        s.spawn(|_| {
            ret_result = ret.map(|gen| {
                gen.write_trace(
                    memory_address_to_id_trace_generator.unwrap(),
                    memory_id_to_value_trace_generator.unwrap(),
                    verify_instruction_trace_generator.unwrap(),
                )
            });
        });
    });

    let blake_result = blake.map(|gen| {
        gen.write_trace(
            memory_address_to_id_trace_generator.unwrap(),
            memory_id_to_value_trace_generator.unwrap(),
            verify_instruction_trace_generator.unwrap(),
            rc_7_2_5_trace_generator.unwrap(),
            verify_bitwise_xor_8_trace_generator.unwrap(),
            blake_round.as_mut().unwrap(),
            triple_xor_32.as_mut().unwrap(),
        )
    });

    let (add_claims, add_interaction_gens) = add_result
        .map(|(trace, claim, interaction_gen)| {
            tree_builder.extend_evals(trace.to_evals());
            (claim, interaction_gen)
        })
        .unzip();
    let (add_small_claims, add_small_interaction_gens) = add_small_result
        .map(|(trace, claim, interaction_gen)| {
            tree_builder.extend_evals(trace.to_evals());
            (claim, interaction_gen)
        })
        .unzip();
    let (add_ap_claims, add_ap_interaction_gens) = add_ap_result
        .map(|(trace, claim, interaction_gen)| {
            tree_builder.extend_evals(trace.to_evals());
            (claim, interaction_gen)
        })
        .unzip();
    let (assert_eq_claims, assert_eq_interaction_gens) = assert_eq_result
        .map(|(trace, claim, interaction_gen)| {
            tree_builder.extend_evals(trace.to_evals());
            (claim, interaction_gen)
        })
        .unzip();
    let (assert_eq_imm_claims, assert_eq_imm_interaction_gens) = assert_eq_imm_result
        .map(|(trace, claim, interaction_gen)| {
            tree_builder.extend_evals(trace.to_evals());
            (claim, interaction_gen)
        })
        .unzip();
    let (assert_eq_double_deref_claims, assert_eq_double_deref_interaction_gens) =
        assert_eq_double_deref_result
            .map(|(trace, claim, interaction_gen)| {
                tree_builder.extend_evals(trace.to_evals());
                (claim, interaction_gen)
            })
            .unzip();
    let (blake_claims, blake_interaction_gens) = blake_result
        .map(|(trace, claim, interaction_gen)| {
            tree_builder.extend_evals(trace.to_evals());
            (claim, interaction_gen)
        })
        .unzip();
    let (call_claims, call_interaction_gens) = call_result
        .map(|(trace, claim, interaction_gen)| {
            tree_builder.extend_evals(trace.to_evals());
            (claim, interaction_gen)
        })
        .unzip();
    let (call_rel_imm_claims, call_rel_imm_interaction_gens) = call_rel_imm_result
        .map(|(trace, claim, interaction_gen)| {
            tree_builder.extend_evals(trace.to_evals());
            (claim, interaction_gen)
        })
        .unzip();
    let (generic_opcode_claims, generic_opcode_interaction_gens) = generic_result
        .map(|(trace, claim, interaction_gen)| {
            tree_builder.extend_evals(trace.to_evals());
            (claim, interaction_gen)
        })
        .unzip();
    let (jnz_claims, jnz_interaction_gens) = jnz_result
        .map(|(trace, claim, interaction_gen)| {
            tree_builder.extend_evals(trace.to_evals());
            (claim, interaction_gen)
        })
        .unzip();
    let (jnz_taken_claims, jnz_taken_interaction_gens) = jnz_taken_result
        .map(|(trace, claim, interaction_gen)| {
            tree_builder.extend_evals(trace.to_evals());
            (claim, interaction_gen)
        })
        .unzip();
    let (jump_claims, jump_interaction_gens) = jump_result
        .map(|(trace, claim, interaction_gen)| {
            tree_builder.extend_evals(trace.to_evals());
            (claim, interaction_gen)
        })
        .unzip();
    let (jump_double_deref_claims, jump_double_deref_interaction_gens) = jump_double_deref_result
        .map(|(trace, claim, interaction_gen)| {
            tree_builder.extend_evals(trace.to_evals());
            (claim, interaction_gen)
        })
        .unzip();
    let (jump_rel_claims, jump_rel_interaction_gens) = jump_rel_result
        .map(|(trace, claim, interaction_gen)| {
            tree_builder.extend_evals(trace.to_evals());
            (claim, interaction_gen)
        })
        .unzip();
    let (jump_rel_imm_claims, jump_rel_imm_interaction_gens) = jump_rel_imm_result
        .map(|(trace, claim, interaction_gen)| {
            tree_builder.extend_evals(trace.to_evals());
            (claim, interaction_gen)
        })
        .unzip();
    let (mul_claims, mul_interaction_gens) = mul_result
        .map(|(trace, claim, interaction_gen)| {
            tree_builder.extend_evals(trace.to_evals());
            (claim, interaction_gen)
        })
        .unzip();
    let (mul_small_claims, mul_small_interaction_gens) = mul_small_result
        .map(|(trace, claim, interaction_gen)| {
            tree_builder.extend_evals(trace.to_evals());
            (claim, interaction_gen)
        })
        .unzip();
    let (qm31_claims, qm31_interaction_gens) = qm31_result
        .map(|(trace, claim, interaction_gen)| {
            tree_builder.extend_evals(trace.to_evals());
            (claim, interaction_gen)
        })
        .unzip();
    let (ret_claims, ret_interaction_gens) = ret_result
        .map(|(trace, claim, interaction_gen)| {
            tree_builder.extend_evals(trace.to_evals());
            (claim, interaction_gen)
        })
        .unzip();

    (
        add_claims,
        add_small_claims,
        add_ap_claims,
        assert_eq_claims,
        assert_eq_imm_claims,
        assert_eq_double_deref_claims,
        blake_claims,
        call_claims,
        call_rel_imm_claims,
        generic_opcode_claims,
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
        OpcodesInteractionClaimGenerator {
            add: add_interaction_gens.into_iter().collect(),
            add_small: add_small_interaction_gens.into_iter().collect(),
            add_ap: add_ap_interaction_gens.into_iter().collect(),
            assert_eq: assert_eq_interaction_gens.into_iter().collect(),
            assert_eq_imm: assert_eq_imm_interaction_gens.into_iter().collect(),
            assert_eq_double_deref: assert_eq_double_deref_interaction_gens
                .into_iter()
                .collect(),
            blake: blake_interaction_gens.into_iter().collect(),
            call: call_interaction_gens.into_iter().collect(),
            call_rel_imm: call_rel_imm_interaction_gens.into_iter().collect(),
            generic_opcode_interaction_gens: generic_opcode_interaction_gens.into_iter().collect(),
            jnz: jnz_interaction_gens.into_iter().collect(),
            jnz_taken: jnz_taken_interaction_gens.into_iter().collect(),
            jump: jump_interaction_gens.into_iter().collect(),
            jump_double_deref: jump_double_deref_interaction_gens.into_iter().collect(),
            jump_rel: jump_rel_interaction_gens.into_iter().collect(),
            jump_rel_imm: jump_rel_imm_interaction_gens.into_iter().collect(),
            mul: mul_interaction_gens.into_iter().collect(),
            mul_small: mul_small_interaction_gens.into_iter().collect(),
            qm31: qm31_interaction_gens.into_iter().collect(),
            ret_interaction_gens: ret_interaction_gens.into_iter().collect(),
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
=======
>>>>>>> 6367d241 (tmp)
