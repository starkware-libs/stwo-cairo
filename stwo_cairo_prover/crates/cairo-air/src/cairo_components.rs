use itertools::chain;
use stwo::core::air::Component;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::ComponentProver;
use stwo_constraint_framework::preprocessed_columns::PreProcessedColumnId;
use stwo_constraint_framework::TraceLocationAllocator;

use crate::blake::air::BlakeContextComponents;
use crate::builtins_air::BuiltinComponents;
use crate::cairo_claim::CairoClaim;
use crate::cairo_interaction_claim::CairoInteractionClaim;
use crate::cairo_interaction_elements::CairoInteractionElements;
use crate::components::{
    add_ap_opcode, add_opcode, add_opcode_small, assert_eq_opcode, assert_eq_opcode_double_deref,
    assert_eq_opcode_imm, blake_compress_opcode, call_opcode_abs, call_opcode_rel_imm,
    display_components, generic_opcode, indented_component_display, jnz_opcode_non_taken,
    jnz_opcode_taken, jump_opcode_abs, jump_opcode_double_deref, jump_opcode_rel,
    jump_opcode_rel_imm, memory_address_to_id, memory_id_to_big, mul_opcode, mul_opcode_small,
    qm_31_add_mul_opcode, ret_opcode, verify_bitwise_xor_4, verify_bitwise_xor_7,
    verify_bitwise_xor_8, verify_bitwise_xor_8_b, verify_bitwise_xor_9, verify_instruction,
};
use crate::pedersen::air::PedersenContextComponents;
use crate::poseidon::air::PoseidonContextComponents;
use crate::range_checks_air::RangeChecksComponents;
pub struct CairoComponents {
    // Opcode components (inlined).
    pub add: Vec<add_opcode::Component>,
    pub add_small: Vec<add_opcode_small::Component>,
    pub add_ap: Vec<add_ap_opcode::Component>,
    pub assert_eq: Vec<assert_eq_opcode::Component>,
    pub assert_eq_imm: Vec<assert_eq_opcode_imm::Component>,
    pub assert_eq_double_deref: Vec<assert_eq_opcode_double_deref::Component>,
    pub blake: Vec<blake_compress_opcode::Component>,
    pub call: Vec<call_opcode_abs::Component>,
    pub call_rel_imm: Vec<call_opcode_rel_imm::Component>,
    pub generic: Vec<generic_opcode::Component>,
    pub jnz: Vec<jnz_opcode_non_taken::Component>,
    pub jnz_taken: Vec<jnz_opcode_taken::Component>,
    pub jump: Vec<jump_opcode_abs::Component>,
    pub jump_double_deref: Vec<jump_opcode_double_deref::Component>,
    pub jump_rel: Vec<jump_opcode_rel::Component>,
    pub jump_rel_imm: Vec<jump_opcode_rel_imm::Component>,
    pub mul: Vec<mul_opcode::Component>,
    pub mul_small: Vec<mul_opcode_small::Component>,
    pub qm31: Vec<qm_31_add_mul_opcode::Component>,
    pub ret: Vec<ret_opcode::Component>,
    // Internal components.
    pub verify_instruction: verify_instruction::Component,
    pub blake_context: BlakeContextComponents,
    pub builtins: BuiltinComponents,
    pub pedersen_context: PedersenContextComponents,
    pub poseidon_context: PoseidonContextComponents,
    pub memory_address_to_id: memory_address_to_id::Component,
    pub memory_id_to_value: (
        Vec<memory_id_to_big::BigComponent>,
        memory_id_to_big::SmallComponent,
    ),
    pub range_checks: RangeChecksComponents,
    pub verify_bitwise_xor_4: verify_bitwise_xor_4::Component,
    pub verify_bitwise_xor_7: verify_bitwise_xor_7::Component,
    pub verify_bitwise_xor_8: verify_bitwise_xor_8::Component,
    pub verify_bitwise_xor_8_b: verify_bitwise_xor_8_b::Component,
    pub verify_bitwise_xor_9: verify_bitwise_xor_9::Component,
    // ...
}
impl CairoComponents {
    pub fn new(
        cairo_claim: &CairoClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &CairoInteractionClaim,
        // Describes the structure of the preprocessed trace. Sensitive to order.
        preprocessed_column_ids: &[PreProcessedColumnId],
    ) -> Self {
        let tree_span_provider =
            &mut TraceLocationAllocator::new_with_preprocessed_columns(preprocessed_column_ids);

        // Build opcode components (inlined former OpcodeComponents::new)
        let add = cairo_claim
            .add
            .iter()
            .zip(interaction_claim.add.iter())
            .map(|(&claim, &interaction_claim)| {
                add_opcode::Component::new(
                    tree_span_provider,
                    add_opcode::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let add_small = cairo_claim
            .add_small
            .iter()
            .zip(interaction_claim.add_small.iter())
            .map(|(&claim, &interaction_claim)| {
                add_opcode_small::Component::new(
                    tree_span_provider,
                    add_opcode_small::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let add_ap = cairo_claim
            .add_ap
            .iter()
            .zip(interaction_claim.add_ap.iter())
            .map(|(&claim, &interaction_claim)| {
                add_ap_opcode::Component::new(
                    tree_span_provider,
                    add_ap_opcode::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        range_check_18_lookup_elements: interaction_elements
                            .range_checks
                            .rc_18
                            .clone(),
                        range_check_11_lookup_elements: interaction_elements
                            .range_checks
                            .rc_11
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let assert_eq = cairo_claim
            .assert_eq
            .iter()
            .zip(interaction_claim.assert_eq.iter())
            .map(|(&claim, &interaction_claim)| {
                assert_eq_opcode::Component::new(
                    tree_span_provider,
                    assert_eq_opcode::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let assert_eq_imm = cairo_claim
            .assert_eq_imm
            .iter()
            .zip(interaction_claim.assert_eq_imm.iter())
            .map(|(&claim, &interaction_claim)| {
                assert_eq_opcode_imm::Component::new(
                    tree_span_provider,
                    assert_eq_opcode_imm::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let assert_eq_double_deref = cairo_claim
            .assert_eq_double_deref
            .iter()
            .zip(interaction_claim.assert_eq_double_deref.iter())
            .map(|(&claim, &interaction_claim)| {
                assert_eq_opcode_double_deref::Component::new(
                    tree_span_provider,
                    assert_eq_opcode_double_deref::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let blake = cairo_claim
            .blake
            .iter()
            .zip(interaction_claim.blake.iter())
            .map(|(&claim, &interaction_claim)| {
                blake_compress_opcode::Component::new(
                    tree_span_provider,
                    blake_compress_opcode::Eval {
                        claim,
                        blake_round_lookup_elements: interaction_elements.blake_round.clone(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        range_check_7_2_5_lookup_elements: interaction_elements
                            .range_checks
                            .rc_7_2_5
                            .clone(),
                        triple_xor_32_lookup_elements: interaction_elements.triple_xor_32.clone(),
                        verify_bitwise_xor_8_lookup_elements: interaction_elements
                            .verify_bitwise_xor_8
                            .clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let call = cairo_claim
            .call
            .iter()
            .zip(interaction_claim.call.iter())
            .map(|(&claim, &interaction_claim)| {
                call_opcode_abs::Component::new(
                    tree_span_provider,
                    call_opcode_abs::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let call_rel_imm = cairo_claim
            .call_rel_imm
            .iter()
            .zip(interaction_claim.call_rel_imm.iter())
            .map(|(&claim, &interaction_claim)| {
                call_opcode_rel_imm::Component::new(
                    tree_span_provider,
                    call_opcode_rel_imm::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let generic = cairo_claim
            .generic
            .iter()
            .zip(interaction_claim.generic.iter())
            .map(|(&claim, &interaction_claim)| {
                generic_opcode::Component::new(
                    tree_span_provider,
                    generic_opcode::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        range_check_20_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20
                            .clone(),
                        range_check_20_b_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_b
                            .clone(),
                        range_check_20_c_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_c
                            .clone(),
                        range_check_20_d_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_d
                            .clone(),
                        range_check_20_e_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_e
                            .clone(),
                        range_check_20_f_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_f
                            .clone(),
                        range_check_20_g_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_g
                            .clone(),
                        range_check_20_h_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_h
                            .clone(),
                        range_check_9_9_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9
                            .clone(),
                        range_check_9_9_b_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_b
                            .clone(),
                        range_check_9_9_c_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_c
                            .clone(),
                        range_check_9_9_d_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_d
                            .clone(),
                        range_check_9_9_e_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_e
                            .clone(),
                        range_check_9_9_f_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_f
                            .clone(),
                        range_check_9_9_g_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_g
                            .clone(),
                        range_check_9_9_h_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_h
                            .clone(),
                        range_check_18_lookup_elements: interaction_elements
                            .range_checks
                            .rc_18
                            .clone(),
                        range_check_11_lookup_elements: interaction_elements
                            .range_checks
                            .rc_11
                            .clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let jnz = cairo_claim
            .jnz
            .iter()
            .zip(interaction_claim.jnz.iter())
            .map(|(&claim, &interaction_claim)| {
                jnz_opcode_non_taken::Component::new(
                    tree_span_provider,
                    jnz_opcode_non_taken::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let jnz_taken = cairo_claim
            .jnz_taken
            .iter()
            .zip(interaction_claim.jnz_taken.iter())
            .map(|(&claim, &interaction_claim)| {
                jnz_opcode_taken::Component::new(
                    tree_span_provider,
                    jnz_opcode_taken::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let jump = cairo_claim
            .jump
            .iter()
            .zip(interaction_claim.jump.iter())
            .map(|(&claim, &interaction_claim)| {
                jump_opcode_abs::Component::new(
                    tree_span_provider,
                    jump_opcode_abs::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let jump_double_deref = cairo_claim
            .jump_double_deref
            .iter()
            .zip(interaction_claim.jump_double_deref.iter())
            .map(|(&claim, &interaction_claim)| {
                jump_opcode_double_deref::Component::new(
                    tree_span_provider,
                    jump_opcode_double_deref::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let jump_rel = cairo_claim
            .jump_rel
            .iter()
            .zip(interaction_claim.jump_rel.iter())
            .map(|(&claim, &interaction_claim)| {
                jump_opcode_rel::Component::new(
                    tree_span_provider,
                    jump_opcode_rel::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let jump_rel_imm = cairo_claim
            .jump_rel_imm
            .iter()
            .zip(interaction_claim.jump_rel_imm.iter())
            .map(|(&claim, &interaction_claim)| {
                jump_opcode_rel_imm::Component::new(
                    tree_span_provider,
                    jump_opcode_rel_imm::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let mul = cairo_claim
            .mul
            .iter()
            .zip(interaction_claim.mul.iter())
            .map(|(&claim, &interaction_claim)| {
                mul_opcode::Component::new(
                    tree_span_provider,
                    mul_opcode::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        range_check_20_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20
                            .clone(),
                        range_check_20_b_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_b
                            .clone(),
                        range_check_20_c_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_c
                            .clone(),
                        range_check_20_d_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_d
                            .clone(),
                        range_check_20_e_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_e
                            .clone(),
                        range_check_20_f_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_f
                            .clone(),
                        range_check_20_g_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_g
                            .clone(),
                        range_check_20_h_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_h
                            .clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let mul_small = cairo_claim
            .mul_small
            .iter()
            .zip(interaction_claim.mul_small.iter())
            .map(|(&claim, &interaction_claim)| {
                mul_opcode_small::Component::new(
                    tree_span_provider,
                    mul_opcode_small::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        range_check_11_lookup_elements: interaction_elements
                            .range_checks
                            .rc_11
                            .clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let qm31 = cairo_claim
            .qm31
            .iter()
            .zip(interaction_claim.qm31.iter())
            .map(|(&claim, &interaction_claim)| {
                qm_31_add_mul_opcode::Component::new(
                    tree_span_provider,
                    qm_31_add_mul_opcode::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        range_check_4_4_4_4_lookup_elements: interaction_elements
                            .range_checks
                            .rc_4_4_4_4
                            .clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let ret = cairo_claim
            .ret
            .iter()
            .zip(interaction_claim.ret.iter())
            .map(|(&claim, &interaction_claim)| {
                ret_opcode::Component::new(
                    tree_span_provider,
                    ret_opcode::Eval {
                        claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();

        let verify_instruction_component = verify_instruction::Component::new(
            tree_span_provider,
            verify_instruction::Eval {
                claim: cairo_claim.verify_instruction,
                memory_address_to_id_lookup_elements: interaction_elements
                    .memory_address_to_id
                    .clone(),
                verify_instruction_lookup_elements: interaction_elements.verify_instruction.clone(),
                memory_id_to_big_lookup_elements: interaction_elements.memory_id_to_value.clone(),
                range_check_4_3_lookup_elements: interaction_elements.range_checks.rc_4_3.clone(),
                range_check_7_2_5_lookup_elements: interaction_elements
                    .range_checks
                    .rc_7_2_5
                    .clone(),
            },
            interaction_claim.verify_instruction.claimed_sum,
        );

        let blake_context = BlakeContextComponents::new(
            tree_span_provider,
            &cairo_claim.blake_context,
            interaction_elements,
            &interaction_claim.blake_context,
        );
        let builtin_components = BuiltinComponents::new(
            tree_span_provider,
            &cairo_claim.builtins,
            interaction_elements,
            &interaction_claim.builtins,
        );
        let pedersen_context = PedersenContextComponents::new(
            tree_span_provider,
            &cairo_claim.pedersen_context,
            interaction_elements,
            &interaction_claim.pedersen_context,
        );
        let poseidon_context = PoseidonContextComponents::new(
            tree_span_provider,
            &cairo_claim.poseidon_context,
            interaction_elements,
            &interaction_claim.poseidon_context,
        );
        let memory_address_to_id_component = memory_address_to_id::Component::new(
            tree_span_provider,
            memory_address_to_id::Eval::new(
                cairo_claim.memory_address_to_id.clone(),
                interaction_elements.memory_address_to_id.clone(),
            ),
            interaction_claim.memory_address_to_id.clone().claimed_sum,
        );

        let memory_id_to_value_components = memory_id_to_big::big_components_from_claim(
            &cairo_claim.memory_id_to_value.big_log_sizes,
            &interaction_claim.memory_id_to_value.big_claimed_sums,
            &interaction_elements.memory_id_to_value,
            &interaction_elements.range_checks.rc_9_9,
            &interaction_elements.range_checks.rc_9_9_b,
            &interaction_elements.range_checks.rc_9_9_c,
            &interaction_elements.range_checks.rc_9_9_d,
            &interaction_elements.range_checks.rc_9_9_e,
            &interaction_elements.range_checks.rc_9_9_f,
            &interaction_elements.range_checks.rc_9_9_g,
            &interaction_elements.range_checks.rc_9_9_h,
            tree_span_provider,
        );
        let small_memory_id_to_value_component = memory_id_to_big::SmallComponent::new(
            tree_span_provider,
            memory_id_to_big::SmallEval::new(
                cairo_claim.memory_id_to_value.clone(),
                interaction_elements.memory_id_to_value.clone(),
                interaction_elements.range_checks.rc_9_9.clone(),
                interaction_elements.range_checks.rc_9_9_b.clone(),
                interaction_elements.range_checks.rc_9_9_c.clone(),
                interaction_elements.range_checks.rc_9_9_d.clone(),
            ),
            interaction_claim
                .memory_id_to_value
                .clone()
                .small_claimed_sum,
        );
        let range_checks_component = RangeChecksComponents::new(
            tree_span_provider,
            &interaction_elements.range_checks,
            &interaction_claim.range_checks,
        );
        let verify_bitwise_xor_4_component = verify_bitwise_xor_4::Component::new(
            tree_span_provider,
            verify_bitwise_xor_4::Eval {
                claim: cairo_claim.verify_bitwise_xor_4,
                verify_bitwise_xor_4_lookup_elements: interaction_elements
                    .verify_bitwise_xor_4
                    .clone(),
            },
            interaction_claim.verify_bitwise_xor_4.claimed_sum,
        );
        let verify_bitwise_xor_7_component = verify_bitwise_xor_7::Component::new(
            tree_span_provider,
            verify_bitwise_xor_7::Eval {
                claim: cairo_claim.verify_bitwise_xor_7,
                verify_bitwise_xor_7_lookup_elements: interaction_elements
                    .verify_bitwise_xor_7
                    .clone(),
            },
            interaction_claim.verify_bitwise_xor_7.claimed_sum,
        );
        let verify_bitwise_xor_8_component = verify_bitwise_xor_8::Component::new(
            tree_span_provider,
            verify_bitwise_xor_8::Eval {
                claim: cairo_claim.verify_bitwise_xor_8,
                verify_bitwise_xor_8_lookup_elements: interaction_elements
                    .verify_bitwise_xor_8
                    .clone(),
            },
            interaction_claim.verify_bitwise_xor_8.claimed_sum,
        );
        let verify_bitwise_xor_8_b_component = verify_bitwise_xor_8_b::Component::new(
            tree_span_provider,
            verify_bitwise_xor_8_b::Eval {
                claim: cairo_claim.verify_bitwise_xor_8_b,
                verify_bitwise_xor_8_b_lookup_elements: interaction_elements
                    .verify_bitwise_xor_8_b
                    .clone(),
            },
            interaction_claim.verify_bitwise_xor_8_b.claimed_sum,
        );
        let verify_bitwise_xor_9_component = verify_bitwise_xor_9::Component::new(
            tree_span_provider,
            verify_bitwise_xor_9::Eval {
                claim: cairo_claim.verify_bitwise_xor_9,
                verify_bitwise_xor_9_lookup_elements: interaction_elements
                    .verify_bitwise_xor_9
                    .clone(),
            },
            interaction_claim.verify_bitwise_xor_9.claimed_sum,
        );
        Self {
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
            verify_instruction: verify_instruction_component,
            blake_context,
            builtins: builtin_components,
            pedersen_context,
            poseidon_context,
            memory_address_to_id: memory_address_to_id_component,
            memory_id_to_value: (
                memory_id_to_value_components,
                small_memory_id_to_value_component,
            ),
            range_checks: range_checks_component,
            verify_bitwise_xor_4: verify_bitwise_xor_4_component,
            verify_bitwise_xor_7: verify_bitwise_xor_7_component,
            verify_bitwise_xor_8: verify_bitwise_xor_8_component,
            verify_bitwise_xor_8_b: verify_bitwise_xor_8_b_component,
            verify_bitwise_xor_9: verify_bitwise_xor_9_component,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        chain!(
            self.add
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.add_small
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.add_ap
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.assert_eq
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.assert_eq_imm
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.assert_eq_double_deref
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.blake
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.call
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.call_rel_imm
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.generic
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.jnz
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.jnz_taken
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.jump
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.jump_double_deref
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.jump_rel
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.jump_rel_imm
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.mul
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.mul_small
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.qm31
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.ret
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            [&self.verify_instruction as &dyn ComponentProver<SimdBackend>,],
            self.blake_context.provers(),
            self.builtins.provers(),
            self.pedersen_context.provers(),
            self.poseidon_context.provers(),
            [&self.memory_address_to_id as &dyn ComponentProver<SimdBackend>,],
            self.memory_id_to_value
                .0
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
            [&self.memory_id_to_value.1 as &dyn ComponentProver<SimdBackend>,],
            self.range_checks.provers(),
            [
                &self.verify_bitwise_xor_4 as &dyn ComponentProver<SimdBackend>,
                &self.verify_bitwise_xor_7 as &dyn ComponentProver<SimdBackend>,
                &self.verify_bitwise_xor_8 as &dyn ComponentProver<SimdBackend>,
                &self.verify_bitwise_xor_8_b as &dyn ComponentProver<SimdBackend>,
                &self.verify_bitwise_xor_9 as &dyn ComponentProver<SimdBackend>,
            ]
        )
        .collect()
    }

    pub fn components(&self) -> Vec<&dyn Component> {
        self.provers()
            .into_iter()
            .map(|component| component as &dyn Component)
            .collect()
    }
}

impl std::fmt::Display for CairoComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CairoComponents")?;
        writeln!(f, "add:\n{}", display_components(&self.add))?;
        writeln!(f, "add_small:\n{}", display_components(&self.add_small))?;
        writeln!(f, "add_ap:\n{}", display_components(&self.add_ap))?;
        writeln!(f, "assert_eq:\n{}", display_components(&self.assert_eq))?;
        writeln!(
            f,
            "assert_eq_imm:\n{}",
            display_components(&self.assert_eq_imm)
        )?;
        writeln!(
            f,
            "assert_eq_double_deref:\n{}",
            display_components(&self.assert_eq_double_deref)
        )?;
        writeln!(f, "blake:\n{}", display_components(&self.blake))?;
        writeln!(f, "call:\n{}", display_components(&self.call))?;
        writeln!(
            f,
            "call_rel_imm:\n{}",
            display_components(&self.call_rel_imm)
        )?;
        writeln!(f, "generic:\n{}", display_components(&self.generic))?;
        writeln!(f, "jnz:\n{}", display_components(&self.jnz))?;
        writeln!(f, "jnz_taken:\n{}", display_components(&self.jnz_taken))?;
        writeln!(f, "jump:\n{}", display_components(&self.jump))?;
        writeln!(
            f,
            "jump_double_deref:\n{}",
            display_components(&self.jump_double_deref)
        )?;
        writeln!(f, "jump_rel:\n{}", display_components(&self.jump_rel))?;
        writeln!(
            f,
            "jump_rel_imm:\n{}",
            display_components(&self.jump_rel_imm)
        )?;
        writeln!(f, "mul:\n{}", display_components(&self.mul))?;
        writeln!(f, "mul_small:\n{}", display_components(&self.mul_small))?;
        writeln!(f, "qm31:\n{}", display_components(&self.qm31))?;
        writeln!(f, "ret:\n{}", display_components(&self.ret))?;
        writeln!(
            f,
            "VerifyInstruction: {}",
            indented_component_display(&self.verify_instruction)
        )?;
        writeln!(f, "BlakeContext: {}", self.blake_context)?;
        writeln!(f, "Builtins: {}", self.builtins)?;
        writeln!(f, "PedersenContext: {}", self.pedersen_context)?;
        writeln!(f, "PoseidonContext: {}", self.poseidon_context)?;
        writeln!(
            f,
            "MemoryAddressToId: {}",
            indented_component_display(&self.memory_address_to_id)
        )?;
        for component in &self.memory_id_to_value.0 {
            writeln!(
                f,
                "MemoryIdToValue: {}",
                indented_component_display(component)
            )?;
        }
        writeln!(
            f,
            "SmallMemoryIdToValue: {}",
            indented_component_display(&self.memory_id_to_value.1)
        )?;
        writeln!(f, "RangeChecks: {}", self.range_checks)?;
        writeln!(
            f,
            "VerifyBitwiseXor4: {}",
            indented_component_display(&self.verify_bitwise_xor_4)
        )?;
        writeln!(
            f,
            "VerifyBitwiseXor7: {}",
            indented_component_display(&self.verify_bitwise_xor_7)
        )?;
        writeln!(
            f,
            "VerifyBitwiseXor8: {}",
            indented_component_display(&self.verify_bitwise_xor_8)
        )?;
        writeln!(
            f,
            "VerifyBitwiseXor8B: {}",
            indented_component_display(&self.verify_bitwise_xor_8_b)
        )?;
        writeln!(
            f,
            "VerifyBitwiseXor9: {}",
            indented_component_display(&self.verify_bitwise_xor_9)
        )?;
        Ok(())
    }
}
