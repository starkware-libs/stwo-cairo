// This file was created by the AIR team.

use std::fmt::Display;

use stwo::core::air::Component;
use stwo_constraint_framework::preprocessed_columns::PreProcessedColumnId;
use stwo_constraint_framework::TraceLocationAllocator;

use crate::claims::{CairoClaim, CairoInteractionClaim};
use crate::components::{
    add_ap_opcode, add_mod_builtin, add_opcode, add_opcode_small, assert_eq_opcode,
    assert_eq_opcode_double_deref, assert_eq_opcode_imm, bitwise_builtin, blake_compress_opcode,
    blake_g, blake_round, blake_round_sigma, call_opcode_abs, call_opcode_rel_imm, cube_252,
    generic_opcode, indented_component_display, jnz_opcode_non_taken, jnz_opcode_taken,
    jump_opcode_abs, jump_opcode_double_deref, jump_opcode_rel, jump_opcode_rel_imm,
    memory_address_to_id, memory_id_to_big, memory_id_to_small, mul_mod_builtin, mul_opcode,
    mul_opcode_small, partial_ec_mul_window_bits_18, partial_ec_mul_window_bits_9,
    pedersen_aggregator_window_bits_18, pedersen_aggregator_window_bits_9, pedersen_builtin,
    pedersen_builtin_narrow_windows, pedersen_points_table_window_bits_18,
    pedersen_points_table_window_bits_9, poseidon_3_partial_rounds_chain, poseidon_aggregator,
    poseidon_builtin, poseidon_full_round_chain, poseidon_round_keys, qm_31_add_mul_opcode,
    range_check96_builtin, range_check_11, range_check_12, range_check_18, range_check_20,
    range_check_252_width_27, range_check_3_3_3_3_3, range_check_3_6_6_3, range_check_4_3,
    range_check_4_4, range_check_4_4_4_4, range_check_6, range_check_7_2_5, range_check_8,
    range_check_9_9, range_check_builtin, ret_opcode, triple_xor_32, verify_bitwise_xor_12,
    verify_bitwise_xor_4, verify_bitwise_xor_7, verify_bitwise_xor_8, verify_bitwise_xor_9,
    verify_instruction,
};
use crate::relations::CommonLookupElements;

pub struct CairoComponents {
    pub add_opcode: Option<add_opcode::Component>,
    pub add_opcode_small: Option<add_opcode_small::Component>,
    pub add_ap_opcode: Option<add_ap_opcode::Component>,
    pub assert_eq_opcode: Option<assert_eq_opcode::Component>,
    pub assert_eq_opcode_imm: Option<assert_eq_opcode_imm::Component>,
    pub assert_eq_opcode_double_deref: Option<assert_eq_opcode_double_deref::Component>,
    pub blake_compress_opcode: Option<blake_compress_opcode::Component>,
    pub call_opcode_abs: Option<call_opcode_abs::Component>,
    pub call_opcode_rel_imm: Option<call_opcode_rel_imm::Component>,
    pub generic_opcode: Option<generic_opcode::Component>,
    pub jnz_opcode_non_taken: Option<jnz_opcode_non_taken::Component>,
    pub jnz_opcode_taken: Option<jnz_opcode_taken::Component>,
    pub jump_opcode_abs: Option<jump_opcode_abs::Component>,
    pub jump_opcode_double_deref: Option<jump_opcode_double_deref::Component>,
    pub jump_opcode_rel: Option<jump_opcode_rel::Component>,
    pub jump_opcode_rel_imm: Option<jump_opcode_rel_imm::Component>,
    pub mul_opcode: Option<mul_opcode::Component>,
    pub mul_opcode_small: Option<mul_opcode_small::Component>,
    pub qm_31_add_mul_opcode: Option<qm_31_add_mul_opcode::Component>,
    pub ret_opcode: Option<ret_opcode::Component>,
    pub verify_instruction: Option<verify_instruction::Component>,
    pub blake_round: Option<blake_round::Component>,
    pub blake_g: Option<blake_g::Component>,
    pub blake_round_sigma: Option<blake_round_sigma::Component>,
    pub triple_xor_32: Option<triple_xor_32::Component>,
    pub verify_bitwise_xor_12: Option<verify_bitwise_xor_12::Component>,
    pub add_mod_builtin: Option<add_mod_builtin::Component>,
    pub bitwise_builtin: Option<bitwise_builtin::Component>,
    pub mul_mod_builtin: Option<mul_mod_builtin::Component>,
    pub pedersen_builtin: Option<pedersen_builtin::Component>,
    pub pedersen_builtin_narrow_windows: Option<pedersen_builtin_narrow_windows::Component>,
    pub poseidon_builtin: Option<poseidon_builtin::Component>,
    pub range_check96_builtin: Option<range_check96_builtin::Component>,
    pub range_check_builtin: Option<range_check_builtin::Component>,
    pub pedersen_aggregator_window_bits_18: Option<pedersen_aggregator_window_bits_18::Component>,
    pub partial_ec_mul_window_bits_18: Option<partial_ec_mul_window_bits_18::Component>,
    pub pedersen_points_table_window_bits_18:
        Option<pedersen_points_table_window_bits_18::Component>,
    pub pedersen_aggregator_window_bits_9: Option<pedersen_aggregator_window_bits_9::Component>,
    pub partial_ec_mul_window_bits_9: Option<partial_ec_mul_window_bits_9::Component>,
    pub pedersen_points_table_window_bits_9: Option<pedersen_points_table_window_bits_9::Component>,
    pub poseidon_aggregator: Option<poseidon_aggregator::Component>,
    pub poseidon_3_partial_rounds_chain: Option<poseidon_3_partial_rounds_chain::Component>,
    pub poseidon_full_round_chain: Option<poseidon_full_round_chain::Component>,
    pub cube_252: Option<cube_252::Component>,
    pub poseidon_round_keys: Option<poseidon_round_keys::Component>,
    pub range_check_252_width_27: Option<range_check_252_width_27::Component>,
    pub memory_address_to_id: Option<memory_address_to_id::Component>,
    pub memory_id_to_big: Vec<memory_id_to_big::BigComponent>,
    pub memory_id_to_small: Option<memory_id_to_small::Component>,
    pub range_check_6: Option<range_check_6::Component>,
    pub range_check_8: Option<range_check_8::Component>,
    pub range_check_11: Option<range_check_11::Component>,
    pub range_check_12: Option<range_check_12::Component>,
    pub range_check_18: Option<range_check_18::Component>,
    pub range_check_20: Option<range_check_20::Component>,
    pub range_check_4_3: Option<range_check_4_3::Component>,
    pub range_check_4_4: Option<range_check_4_4::Component>,
    pub range_check_9_9: Option<range_check_9_9::Component>,
    pub range_check_7_2_5: Option<range_check_7_2_5::Component>,
    pub range_check_3_6_6_3: Option<range_check_3_6_6_3::Component>,
    pub range_check_4_4_4_4: Option<range_check_4_4_4_4::Component>,
    pub range_check_3_3_3_3_3: Option<range_check_3_3_3_3_3::Component>,
    pub verify_bitwise_xor_4: Option<verify_bitwise_xor_4::Component>,
    pub verify_bitwise_xor_7: Option<verify_bitwise_xor_7::Component>,
    pub verify_bitwise_xor_8: Option<verify_bitwise_xor_8::Component>,
    pub verify_bitwise_xor_9: Option<verify_bitwise_xor_9::Component>,
}

impl CairoComponents {
    pub fn new(
        cairo_claim: &CairoClaim,
        common_lookup_elements: &CommonLookupElements,
        interaction_claim: &CairoInteractionClaim,
        // Describes the structure of the preprocessed trace. Sensitive to order.
        preprocessed_column_ids: &[PreProcessedColumnId],
    ) -> Self {
        let tree_span_provider =
            &mut TraceLocationAllocator::new_with_preprocessed_columns(preprocessed_column_ids);

        let add_opcode_component = cairo_claim.add_opcode.map(|add_opcode| {
            add_opcode::Component::new(
                tree_span_provider,
                add_opcode::Eval {
                    claim: add_opcode,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.add_opcode.unwrap().claimed_sum,
            )
        });
        let add_opcode_small_component = cairo_claim.add_opcode_small.map(|add_opcode_small| {
            add_opcode_small::Component::new(
                tree_span_provider,
                add_opcode_small::Eval {
                    claim: add_opcode_small,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.add_opcode_small.unwrap().claimed_sum,
            )
        });
        let add_ap_opcode_component = cairo_claim.add_ap_opcode.map(|add_ap_opcode| {
            add_ap_opcode::Component::new(
                tree_span_provider,
                add_ap_opcode::Eval {
                    claim: add_ap_opcode,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.add_ap_opcode.unwrap().claimed_sum,
            )
        });
        let assert_eq_opcode_component = cairo_claim.assert_eq_opcode.map(|assert_eq_opcode| {
            assert_eq_opcode::Component::new(
                tree_span_provider,
                assert_eq_opcode::Eval {
                    claim: assert_eq_opcode,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.assert_eq_opcode.unwrap().claimed_sum,
            )
        });
        let assert_eq_opcode_imm_component =
            cairo_claim
                .assert_eq_opcode_imm
                .map(|assert_eq_opcode_imm| {
                    assert_eq_opcode_imm::Component::new(
                        tree_span_provider,
                        assert_eq_opcode_imm::Eval {
                            claim: assert_eq_opcode_imm,
                            common_lookup_elements: common_lookup_elements.clone(),
                        },
                        interaction_claim.assert_eq_opcode_imm.unwrap().claimed_sum,
                    )
                });
        let assert_eq_opcode_double_deref_component = cairo_claim
            .assert_eq_opcode_double_deref
            .map(|assert_eq_opcode_double_deref| {
                assert_eq_opcode_double_deref::Component::new(
                    tree_span_provider,
                    assert_eq_opcode_double_deref::Eval {
                        claim: assert_eq_opcode_double_deref,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim
                        .assert_eq_opcode_double_deref
                        .unwrap()
                        .claimed_sum,
                )
            });
        let blake_compress_opcode_component =
            cairo_claim
                .blake_compress_opcode
                .map(|blake_compress_opcode| {
                    blake_compress_opcode::Component::new(
                        tree_span_provider,
                        blake_compress_opcode::Eval {
                            claim: blake_compress_opcode,
                            common_lookup_elements: common_lookup_elements.clone(),
                        },
                        interaction_claim.blake_compress_opcode.unwrap().claimed_sum,
                    )
                });
        let call_opcode_abs_component = cairo_claim.call_opcode_abs.map(|call_opcode_abs| {
            call_opcode_abs::Component::new(
                tree_span_provider,
                call_opcode_abs::Eval {
                    claim: call_opcode_abs,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.call_opcode_abs.unwrap().claimed_sum,
            )
        });
        let call_opcode_rel_imm_component =
            cairo_claim.call_opcode_rel_imm.map(|call_opcode_rel_imm| {
                call_opcode_rel_imm::Component::new(
                    tree_span_provider,
                    call_opcode_rel_imm::Eval {
                        claim: call_opcode_rel_imm,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.call_opcode_rel_imm.unwrap().claimed_sum,
                )
            });
        let generic_opcode_component = cairo_claim.generic_opcode.map(|generic_opcode| {
            generic_opcode::Component::new(
                tree_span_provider,
                generic_opcode::Eval {
                    claim: generic_opcode,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.generic_opcode.unwrap().claimed_sum,
            )
        });
        let jnz_opcode_non_taken_component =
            cairo_claim
                .jnz_opcode_non_taken
                .map(|jnz_opcode_non_taken| {
                    jnz_opcode_non_taken::Component::new(
                        tree_span_provider,
                        jnz_opcode_non_taken::Eval {
                            claim: jnz_opcode_non_taken,
                            common_lookup_elements: common_lookup_elements.clone(),
                        },
                        interaction_claim.jnz_opcode_non_taken.unwrap().claimed_sum,
                    )
                });
        let jnz_opcode_taken_component = cairo_claim.jnz_opcode_taken.map(|jnz_opcode_taken| {
            jnz_opcode_taken::Component::new(
                tree_span_provider,
                jnz_opcode_taken::Eval {
                    claim: jnz_opcode_taken,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.jnz_opcode_taken.unwrap().claimed_sum,
            )
        });
        let jump_opcode_abs_component = cairo_claim.jump_opcode_abs.map(|jump_opcode_abs| {
            jump_opcode_abs::Component::new(
                tree_span_provider,
                jump_opcode_abs::Eval {
                    claim: jump_opcode_abs,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.jump_opcode_abs.unwrap().claimed_sum,
            )
        });
        let jump_opcode_double_deref_component =
            cairo_claim
                .jump_opcode_double_deref
                .map(|jump_opcode_double_deref| {
                    jump_opcode_double_deref::Component::new(
                        tree_span_provider,
                        jump_opcode_double_deref::Eval {
                            claim: jump_opcode_double_deref,
                            common_lookup_elements: common_lookup_elements.clone(),
                        },
                        interaction_claim
                            .jump_opcode_double_deref
                            .unwrap()
                            .claimed_sum,
                    )
                });
        let jump_opcode_rel_component = cairo_claim.jump_opcode_rel.map(|jump_opcode_rel| {
            jump_opcode_rel::Component::new(
                tree_span_provider,
                jump_opcode_rel::Eval {
                    claim: jump_opcode_rel,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.jump_opcode_rel.unwrap().claimed_sum,
            )
        });
        let jump_opcode_rel_imm_component =
            cairo_claim.jump_opcode_rel_imm.map(|jump_opcode_rel_imm| {
                jump_opcode_rel_imm::Component::new(
                    tree_span_provider,
                    jump_opcode_rel_imm::Eval {
                        claim: jump_opcode_rel_imm,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.jump_opcode_rel_imm.unwrap().claimed_sum,
                )
            });
        let mul_opcode_component = cairo_claim.mul_opcode.map(|mul_opcode| {
            mul_opcode::Component::new(
                tree_span_provider,
                mul_opcode::Eval {
                    claim: mul_opcode,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.mul_opcode.unwrap().claimed_sum,
            )
        });
        let mul_opcode_small_component = cairo_claim.mul_opcode_small.map(|mul_opcode_small| {
            mul_opcode_small::Component::new(
                tree_span_provider,
                mul_opcode_small::Eval {
                    claim: mul_opcode_small,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.mul_opcode_small.unwrap().claimed_sum,
            )
        });
        let qm_31_add_mul_opcode_component =
            cairo_claim
                .qm_31_add_mul_opcode
                .map(|qm_31_add_mul_opcode| {
                    qm_31_add_mul_opcode::Component::new(
                        tree_span_provider,
                        qm_31_add_mul_opcode::Eval {
                            claim: qm_31_add_mul_opcode,
                            common_lookup_elements: common_lookup_elements.clone(),
                        },
                        interaction_claim.qm_31_add_mul_opcode.unwrap().claimed_sum,
                    )
                });
        let ret_opcode_component = cairo_claim.ret_opcode.map(|ret_opcode| {
            ret_opcode::Component::new(
                tree_span_provider,
                ret_opcode::Eval {
                    claim: ret_opcode,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.ret_opcode.unwrap().claimed_sum,
            )
        });
        let verify_instruction_component =
            cairo_claim.verify_instruction.map(|verify_instruction| {
                verify_instruction::Component::new(
                    tree_span_provider,
                    verify_instruction::Eval {
                        claim: verify_instruction,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.verify_instruction.unwrap().claimed_sum,
                )
            });
        let blake_round_component = cairo_claim.blake_round.map(|blake_round| {
            blake_round::Component::new(
                tree_span_provider,
                blake_round::Eval {
                    claim: blake_round,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.blake_round.unwrap().claimed_sum,
            )
        });
        let blake_g_component = cairo_claim.blake_g.map(|blake_g| {
            blake_g::Component::new(
                tree_span_provider,
                blake_g::Eval {
                    claim: blake_g,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.blake_g.unwrap().claimed_sum,
            )
        });
        let blake_round_sigma_component = cairo_claim.blake_round_sigma.map(|blake_round_sigma| {
            blake_round_sigma::Component::new(
                tree_span_provider,
                blake_round_sigma::Eval {
                    claim: blake_round_sigma,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.blake_round_sigma.unwrap().claimed_sum,
            )
        });
        let triple_xor_32_component = cairo_claim.triple_xor_32.map(|triple_xor_32| {
            triple_xor_32::Component::new(
                tree_span_provider,
                triple_xor_32::Eval {
                    claim: triple_xor_32,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.triple_xor_32.unwrap().claimed_sum,
            )
        });
        let verify_bitwise_xor_12_component =
            cairo_claim
                .verify_bitwise_xor_12
                .map(|verify_bitwise_xor_12| {
                    verify_bitwise_xor_12::Component::new(
                        tree_span_provider,
                        verify_bitwise_xor_12::Eval {
                            claim: verify_bitwise_xor_12,
                            common_lookup_elements: common_lookup_elements.clone(),
                        },
                        interaction_claim.verify_bitwise_xor_12.unwrap().claimed_sum,
                    )
                });
        let add_mod_builtin_component = cairo_claim.add_mod_builtin.map(|add_mod_builtin| {
            add_mod_builtin::Component::new(
                tree_span_provider,
                add_mod_builtin::Eval {
                    claim: add_mod_builtin,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.add_mod_builtin.unwrap().claimed_sum,
            )
        });
        let bitwise_builtin_component = cairo_claim.bitwise_builtin.map(|bitwise_builtin| {
            bitwise_builtin::Component::new(
                tree_span_provider,
                bitwise_builtin::Eval {
                    claim: bitwise_builtin,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.bitwise_builtin.unwrap().claimed_sum,
            )
        });
        let mul_mod_builtin_component = cairo_claim.mul_mod_builtin.map(|mul_mod_builtin| {
            mul_mod_builtin::Component::new(
                tree_span_provider,
                mul_mod_builtin::Eval {
                    claim: mul_mod_builtin,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.mul_mod_builtin.unwrap().claimed_sum,
            )
        });
        let pedersen_builtin_component = cairo_claim.pedersen_builtin.map(|pedersen_builtin| {
            pedersen_builtin::Component::new(
                tree_span_provider,
                pedersen_builtin::Eval {
                    claim: pedersen_builtin,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.pedersen_builtin.unwrap().claimed_sum,
            )
        });
        let pedersen_builtin_narrow_windows_component = cairo_claim
            .pedersen_builtin_narrow_windows
            .map(|pedersen_builtin_narrow_windows| {
                pedersen_builtin_narrow_windows::Component::new(
                    tree_span_provider,
                    pedersen_builtin_narrow_windows::Eval {
                        claim: pedersen_builtin_narrow_windows,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim
                        .pedersen_builtin_narrow_windows
                        .unwrap()
                        .claimed_sum,
                )
            });
        let poseidon_builtin_component = cairo_claim.poseidon_builtin.map(|poseidon_builtin| {
            poseidon_builtin::Component::new(
                tree_span_provider,
                poseidon_builtin::Eval {
                    claim: poseidon_builtin,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.poseidon_builtin.unwrap().claimed_sum,
            )
        });
        let range_check96_builtin_component =
            cairo_claim
                .range_check96_builtin
                .map(|range_check96_builtin| {
                    range_check96_builtin::Component::new(
                        tree_span_provider,
                        range_check96_builtin::Eval {
                            claim: range_check96_builtin,
                            common_lookup_elements: common_lookup_elements.clone(),
                        },
                        interaction_claim.range_check96_builtin.unwrap().claimed_sum,
                    )
                });
        let range_check_builtin_component =
            cairo_claim.range_check_builtin.map(|range_check_builtin| {
                range_check_builtin::Component::new(
                    tree_span_provider,
                    range_check_builtin::Eval {
                        claim: range_check_builtin,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.range_check_builtin.unwrap().claimed_sum,
                )
            });
        let pedersen_aggregator_window_bits_18_component = cairo_claim
            .pedersen_aggregator_window_bits_18
            .map(|pedersen_aggregator_window_bits_18| {
                pedersen_aggregator_window_bits_18::Component::new(
                    tree_span_provider,
                    pedersen_aggregator_window_bits_18::Eval {
                        claim: pedersen_aggregator_window_bits_18,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim
                        .pedersen_aggregator_window_bits_18
                        .unwrap()
                        .claimed_sum,
                )
            });
        let partial_ec_mul_window_bits_18_component = cairo_claim
            .partial_ec_mul_window_bits_18
            .map(|partial_ec_mul_window_bits_18| {
                partial_ec_mul_window_bits_18::Component::new(
                    tree_span_provider,
                    partial_ec_mul_window_bits_18::Eval {
                        claim: partial_ec_mul_window_bits_18,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim
                        .partial_ec_mul_window_bits_18
                        .unwrap()
                        .claimed_sum,
                )
            });
        let pedersen_points_table_window_bits_18_component = cairo_claim
            .pedersen_points_table_window_bits_18
            .map(|pedersen_points_table_window_bits_18| {
                pedersen_points_table_window_bits_18::Component::new(
                    tree_span_provider,
                    pedersen_points_table_window_bits_18::Eval {
                        claim: pedersen_points_table_window_bits_18,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim
                        .pedersen_points_table_window_bits_18
                        .unwrap()
                        .claimed_sum,
                )
            });
        let pedersen_aggregator_window_bits_9_component = cairo_claim
            .pedersen_aggregator_window_bits_9
            .map(|pedersen_aggregator_window_bits_9| {
                pedersen_aggregator_window_bits_9::Component::new(
                    tree_span_provider,
                    pedersen_aggregator_window_bits_9::Eval {
                        claim: pedersen_aggregator_window_bits_9,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim
                        .pedersen_aggregator_window_bits_9
                        .unwrap()
                        .claimed_sum,
                )
            });
        let partial_ec_mul_window_bits_9_component =
            cairo_claim
                .partial_ec_mul_window_bits_9
                .map(|partial_ec_mul_window_bits_9| {
                    partial_ec_mul_window_bits_9::Component::new(
                        tree_span_provider,
                        partial_ec_mul_window_bits_9::Eval {
                            claim: partial_ec_mul_window_bits_9,
                            common_lookup_elements: common_lookup_elements.clone(),
                        },
                        interaction_claim
                            .partial_ec_mul_window_bits_9
                            .unwrap()
                            .claimed_sum,
                    )
                });
        let pedersen_points_table_window_bits_9_component = cairo_claim
            .pedersen_points_table_window_bits_9
            .map(|pedersen_points_table_window_bits_9| {
                pedersen_points_table_window_bits_9::Component::new(
                    tree_span_provider,
                    pedersen_points_table_window_bits_9::Eval {
                        claim: pedersen_points_table_window_bits_9,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim
                        .pedersen_points_table_window_bits_9
                        .unwrap()
                        .claimed_sum,
                )
            });
        let poseidon_aggregator_component =
            cairo_claim.poseidon_aggregator.map(|poseidon_aggregator| {
                poseidon_aggregator::Component::new(
                    tree_span_provider,
                    poseidon_aggregator::Eval {
                        claim: poseidon_aggregator,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.poseidon_aggregator.unwrap().claimed_sum,
                )
            });
        let poseidon_3_partial_rounds_chain_component = cairo_claim
            .poseidon_3_partial_rounds_chain
            .map(|poseidon_3_partial_rounds_chain| {
                poseidon_3_partial_rounds_chain::Component::new(
                    tree_span_provider,
                    poseidon_3_partial_rounds_chain::Eval {
                        claim: poseidon_3_partial_rounds_chain,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim
                        .poseidon_3_partial_rounds_chain
                        .unwrap()
                        .claimed_sum,
                )
            });
        let poseidon_full_round_chain_component =
            cairo_claim
                .poseidon_full_round_chain
                .map(|poseidon_full_round_chain| {
                    poseidon_full_round_chain::Component::new(
                        tree_span_provider,
                        poseidon_full_round_chain::Eval {
                            claim: poseidon_full_round_chain,
                            common_lookup_elements: common_lookup_elements.clone(),
                        },
                        interaction_claim
                            .poseidon_full_round_chain
                            .unwrap()
                            .claimed_sum,
                    )
                });
        let cube_252_component = cairo_claim.cube_252.map(|cube_252| {
            cube_252::Component::new(
                tree_span_provider,
                cube_252::Eval {
                    claim: cube_252,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.cube_252.unwrap().claimed_sum,
            )
        });
        let poseidon_round_keys_component =
            cairo_claim.poseidon_round_keys.map(|poseidon_round_keys| {
                poseidon_round_keys::Component::new(
                    tree_span_provider,
                    poseidon_round_keys::Eval {
                        claim: poseidon_round_keys,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.poseidon_round_keys.unwrap().claimed_sum,
                )
            });
        let range_check_252_width_27_component =
            cairo_claim
                .range_check_252_width_27
                .map(|range_check_252_width_27| {
                    range_check_252_width_27::Component::new(
                        tree_span_provider,
                        range_check_252_width_27::Eval {
                            claim: range_check_252_width_27,
                            common_lookup_elements: common_lookup_elements.clone(),
                        },
                        interaction_claim
                            .range_check_252_width_27
                            .unwrap()
                            .claimed_sum,
                    )
                });
        let memory_address_to_id_component =
            cairo_claim
                .memory_address_to_id
                .map(|memory_address_to_id| {
                    memory_address_to_id::Component::new(
                        tree_span_provider,
                        memory_address_to_id::Eval {
                            claim: memory_address_to_id,
                            common_lookup_elements: common_lookup_elements.clone(),
                        },
                        interaction_claim.memory_address_to_id.unwrap().claimed_sum,
                    )
                });
        let memory_id_to_big_component = memory_id_to_big::big_components_from_claim(
            &cairo_claim.memory_id_to_big.as_ref().unwrap().big_log_sizes,
            &interaction_claim
                .memory_id_to_big
                .as_ref()
                .unwrap()
                .big_claimed_sums,
            &common_lookup_elements.clone(),
            tree_span_provider,
        );
        let memory_id_to_small_component =
            cairo_claim.memory_id_to_small.map(|memory_id_to_small| {
                memory_id_to_small::Component::new(
                    tree_span_provider,
                    memory_id_to_small::Eval {
                        claim: memory_id_to_small,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.memory_id_to_small.unwrap().claimed_sum,
                )
            });
        let range_check_6_component = cairo_claim.range_check_6.map(|range_check_6| {
            range_check_6::Component::new(
                tree_span_provider,
                range_check_6::Eval {
                    claim: range_check_6,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.range_check_6.unwrap().claimed_sum,
            )
        });
        let range_check_8_component = cairo_claim.range_check_8.map(|range_check_8| {
            range_check_8::Component::new(
                tree_span_provider,
                range_check_8::Eval {
                    claim: range_check_8,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.range_check_8.unwrap().claimed_sum,
            )
        });
        let range_check_11_component = cairo_claim.range_check_11.map(|range_check_11| {
            range_check_11::Component::new(
                tree_span_provider,
                range_check_11::Eval {
                    claim: range_check_11,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.range_check_11.unwrap().claimed_sum,
            )
        });
        let range_check_12_component = cairo_claim.range_check_12.map(|range_check_12| {
            range_check_12::Component::new(
                tree_span_provider,
                range_check_12::Eval {
                    claim: range_check_12,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.range_check_12.unwrap().claimed_sum,
            )
        });
        let range_check_18_component = cairo_claim.range_check_18.map(|range_check_18| {
            range_check_18::Component::new(
                tree_span_provider,
                range_check_18::Eval {
                    claim: range_check_18,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.range_check_18.unwrap().claimed_sum,
            )
        });
        let range_check_20_component = cairo_claim.range_check_20.map(|range_check_20| {
            range_check_20::Component::new(
                tree_span_provider,
                range_check_20::Eval {
                    claim: range_check_20,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.range_check_20.unwrap().claimed_sum,
            )
        });
        let range_check_4_3_component = cairo_claim.range_check_4_3.map(|range_check_4_3| {
            range_check_4_3::Component::new(
                tree_span_provider,
                range_check_4_3::Eval {
                    claim: range_check_4_3,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.range_check_4_3.unwrap().claimed_sum,
            )
        });
        let range_check_4_4_component = cairo_claim.range_check_4_4.map(|range_check_4_4| {
            range_check_4_4::Component::new(
                tree_span_provider,
                range_check_4_4::Eval {
                    claim: range_check_4_4,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.range_check_4_4.unwrap().claimed_sum,
            )
        });
        let range_check_9_9_component = cairo_claim.range_check_9_9.map(|range_check_9_9| {
            range_check_9_9::Component::new(
                tree_span_provider,
                range_check_9_9::Eval {
                    claim: range_check_9_9,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.range_check_9_9.unwrap().claimed_sum,
            )
        });
        let range_check_7_2_5_component = cairo_claim.range_check_7_2_5.map(|range_check_7_2_5| {
            range_check_7_2_5::Component::new(
                tree_span_provider,
                range_check_7_2_5::Eval {
                    claim: range_check_7_2_5,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim.range_check_7_2_5.unwrap().claimed_sum,
            )
        });
        let range_check_3_6_6_3_component =
            cairo_claim.range_check_3_6_6_3.map(|range_check_3_6_6_3| {
                range_check_3_6_6_3::Component::new(
                    tree_span_provider,
                    range_check_3_6_6_3::Eval {
                        claim: range_check_3_6_6_3,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.range_check_3_6_6_3.unwrap().claimed_sum,
                )
            });
        let range_check_4_4_4_4_component =
            cairo_claim.range_check_4_4_4_4.map(|range_check_4_4_4_4| {
                range_check_4_4_4_4::Component::new(
                    tree_span_provider,
                    range_check_4_4_4_4::Eval {
                        claim: range_check_4_4_4_4,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.range_check_4_4_4_4.unwrap().claimed_sum,
                )
            });
        let range_check_3_3_3_3_3_component =
            cairo_claim
                .range_check_3_3_3_3_3
                .map(|range_check_3_3_3_3_3| {
                    range_check_3_3_3_3_3::Component::new(
                        tree_span_provider,
                        range_check_3_3_3_3_3::Eval {
                            claim: range_check_3_3_3_3_3,
                            common_lookup_elements: common_lookup_elements.clone(),
                        },
                        interaction_claim.range_check_3_3_3_3_3.unwrap().claimed_sum,
                    )
                });
        let verify_bitwise_xor_4_component =
            cairo_claim
                .verify_bitwise_xor_4
                .map(|verify_bitwise_xor_4| {
                    verify_bitwise_xor_4::Component::new(
                        tree_span_provider,
                        verify_bitwise_xor_4::Eval {
                            claim: verify_bitwise_xor_4,
                            common_lookup_elements: common_lookup_elements.clone(),
                        },
                        interaction_claim.verify_bitwise_xor_4.unwrap().claimed_sum,
                    )
                });
        let verify_bitwise_xor_7_component =
            cairo_claim
                .verify_bitwise_xor_7
                .map(|verify_bitwise_xor_7| {
                    verify_bitwise_xor_7::Component::new(
                        tree_span_provider,
                        verify_bitwise_xor_7::Eval {
                            claim: verify_bitwise_xor_7,
                            common_lookup_elements: common_lookup_elements.clone(),
                        },
                        interaction_claim.verify_bitwise_xor_7.unwrap().claimed_sum,
                    )
                });
        let verify_bitwise_xor_8_component =
            cairo_claim
                .verify_bitwise_xor_8
                .map(|verify_bitwise_xor_8| {
                    verify_bitwise_xor_8::Component::new(
                        tree_span_provider,
                        verify_bitwise_xor_8::Eval {
                            claim: verify_bitwise_xor_8,
                            common_lookup_elements: common_lookup_elements.clone(),
                        },
                        interaction_claim.verify_bitwise_xor_8.unwrap().claimed_sum,
                    )
                });
        let verify_bitwise_xor_9_component =
            cairo_claim
                .verify_bitwise_xor_9
                .map(|verify_bitwise_xor_9| {
                    verify_bitwise_xor_9::Component::new(
                        tree_span_provider,
                        verify_bitwise_xor_9::Eval {
                            claim: verify_bitwise_xor_9,
                            common_lookup_elements: common_lookup_elements.clone(),
                        },
                        interaction_claim.verify_bitwise_xor_9.unwrap().claimed_sum,
                    )
                });

        Self {
            add_opcode: add_opcode_component,
            add_opcode_small: add_opcode_small_component,
            add_ap_opcode: add_ap_opcode_component,
            assert_eq_opcode: assert_eq_opcode_component,
            assert_eq_opcode_imm: assert_eq_opcode_imm_component,
            assert_eq_opcode_double_deref: assert_eq_opcode_double_deref_component,
            blake_compress_opcode: blake_compress_opcode_component,
            call_opcode_abs: call_opcode_abs_component,
            call_opcode_rel_imm: call_opcode_rel_imm_component,
            generic_opcode: generic_opcode_component,
            jnz_opcode_non_taken: jnz_opcode_non_taken_component,
            jnz_opcode_taken: jnz_opcode_taken_component,
            jump_opcode_abs: jump_opcode_abs_component,
            jump_opcode_double_deref: jump_opcode_double_deref_component,
            jump_opcode_rel: jump_opcode_rel_component,
            jump_opcode_rel_imm: jump_opcode_rel_imm_component,
            mul_opcode: mul_opcode_component,
            mul_opcode_small: mul_opcode_small_component,
            qm_31_add_mul_opcode: qm_31_add_mul_opcode_component,
            ret_opcode: ret_opcode_component,
            verify_instruction: verify_instruction_component,
            blake_round: blake_round_component,
            blake_g: blake_g_component,
            blake_round_sigma: blake_round_sigma_component,
            triple_xor_32: triple_xor_32_component,
            verify_bitwise_xor_12: verify_bitwise_xor_12_component,
            add_mod_builtin: add_mod_builtin_component,
            bitwise_builtin: bitwise_builtin_component,
            mul_mod_builtin: mul_mod_builtin_component,
            pedersen_builtin: pedersen_builtin_component,
            pedersen_builtin_narrow_windows: pedersen_builtin_narrow_windows_component,
            poseidon_builtin: poseidon_builtin_component,
            range_check96_builtin: range_check96_builtin_component,
            range_check_builtin: range_check_builtin_component,
            pedersen_aggregator_window_bits_18: pedersen_aggregator_window_bits_18_component,
            partial_ec_mul_window_bits_18: partial_ec_mul_window_bits_18_component,
            pedersen_points_table_window_bits_18: pedersen_points_table_window_bits_18_component,
            pedersen_aggregator_window_bits_9: pedersen_aggregator_window_bits_9_component,
            partial_ec_mul_window_bits_9: partial_ec_mul_window_bits_9_component,
            pedersen_points_table_window_bits_9: pedersen_points_table_window_bits_9_component,
            poseidon_aggregator: poseidon_aggregator_component,
            poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain_component,
            poseidon_full_round_chain: poseidon_full_round_chain_component,
            cube_252: cube_252_component,
            poseidon_round_keys: poseidon_round_keys_component,
            range_check_252_width_27: range_check_252_width_27_component,
            memory_address_to_id: memory_address_to_id_component,
            memory_id_to_big: memory_id_to_big_component,
            memory_id_to_small: memory_id_to_small_component,
            range_check_6: range_check_6_component,
            range_check_8: range_check_8_component,
            range_check_11: range_check_11_component,
            range_check_12: range_check_12_component,
            range_check_18: range_check_18_component,
            range_check_20: range_check_20_component,
            range_check_4_3: range_check_4_3_component,
            range_check_4_4: range_check_4_4_component,
            range_check_9_9: range_check_9_9_component,
            range_check_7_2_5: range_check_7_2_5_component,
            range_check_3_6_6_3: range_check_3_6_6_3_component,
            range_check_4_4_4_4: range_check_4_4_4_4_component,
            range_check_3_3_3_3_3: range_check_3_3_3_3_3_component,
            verify_bitwise_xor_4: verify_bitwise_xor_4_component,
            verify_bitwise_xor_7: verify_bitwise_xor_7_component,
            verify_bitwise_xor_8: verify_bitwise_xor_8_component,
            verify_bitwise_xor_9: verify_bitwise_xor_9_component,
        }
    }

    pub fn components(&self) -> Vec<&dyn Component> {
        let mut vec: Vec<&dyn Component> = vec![];
        if let Some(component) = &self.add_opcode {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.add_opcode_small {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.add_ap_opcode {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.assert_eq_opcode {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.assert_eq_opcode_imm {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.assert_eq_opcode_double_deref {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.blake_compress_opcode {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.call_opcode_abs {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.call_opcode_rel_imm {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.generic_opcode {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.jnz_opcode_non_taken {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.jnz_opcode_taken {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.jump_opcode_abs {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.jump_opcode_double_deref {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.jump_opcode_rel {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.jump_opcode_rel_imm {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.mul_opcode {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.mul_opcode_small {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.qm_31_add_mul_opcode {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.ret_opcode {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.verify_instruction {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.blake_round {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.blake_g {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.blake_round_sigma {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.triple_xor_32 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.verify_bitwise_xor_12 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.add_mod_builtin {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.bitwise_builtin {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.mul_mod_builtin {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.pedersen_builtin {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.pedersen_builtin_narrow_windows {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.poseidon_builtin {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check96_builtin {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_builtin {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.pedersen_aggregator_window_bits_18 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.partial_ec_mul_window_bits_18 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.pedersen_points_table_window_bits_18 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.pedersen_aggregator_window_bits_9 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.partial_ec_mul_window_bits_9 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.pedersen_points_table_window_bits_9 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.poseidon_aggregator {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.poseidon_3_partial_rounds_chain {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.poseidon_full_round_chain {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.cube_252 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.poseidon_round_keys {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_252_width_27 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.memory_address_to_id {
            vec.push(component as &dyn Component);
        }
        for component in &self.memory_id_to_big {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.memory_id_to_small {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_6 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_8 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_11 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_12 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_18 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_20 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_4_3 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_4_4 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_9_9 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_7_2_5 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_3_6_6_3 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_4_4_4_4 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_3_3_3_3_3 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.verify_bitwise_xor_4 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.verify_bitwise_xor_7 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.verify_bitwise_xor_8 {
            vec.push(component as &dyn Component);
        }
        if let Some(component) = &self.verify_bitwise_xor_9 {
            vec.push(component as &dyn Component);
        }
        vec
    }
}

impl Display for CairoComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CairoComponents")?;
        if let Some(ref component) = self.add_opcode {
            writeln!(f, "AddOpcode: {}", indented_component_display(component))?;
        }
        if let Some(ref component) = self.add_opcode_small {
            writeln!(
                f,
                "AddOpcodeSmall: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.add_ap_opcode {
            writeln!(f, "AddApOpcode: {}", indented_component_display(component))?;
        }
        if let Some(ref component) = self.assert_eq_opcode {
            writeln!(
                f,
                "AssertEqOpcode: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.assert_eq_opcode_imm {
            writeln!(
                f,
                "AssertEqOpcodeImm: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.assert_eq_opcode_double_deref {
            writeln!(
                f,
                "AssertEqOpcodeDoubleDeref: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.blake_compress_opcode {
            writeln!(
                f,
                "BlakeCompressOpcode: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.call_opcode_abs {
            writeln!(
                f,
                "CallOpcodeAbs: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.call_opcode_rel_imm {
            writeln!(
                f,
                "CallOpcodeRelImm: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.generic_opcode {
            writeln!(
                f,
                "GenericOpcode: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.jnz_opcode_non_taken {
            writeln!(
                f,
                "JnzOpcodeNonTaken: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.jnz_opcode_taken {
            writeln!(
                f,
                "JnzOpcodeTaken: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.jump_opcode_abs {
            writeln!(
                f,
                "JumpOpcodeAbs: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.jump_opcode_double_deref {
            writeln!(
                f,
                "JumpOpcodeDoubleDeref: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.jump_opcode_rel {
            writeln!(
                f,
                "JumpOpcodeRel: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.jump_opcode_rel_imm {
            writeln!(
                f,
                "JumpOpcodeRelImm: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.mul_opcode {
            writeln!(f, "MulOpcode: {}", indented_component_display(component))?;
        }
        if let Some(ref component) = self.mul_opcode_small {
            writeln!(
                f,
                "MulOpcodeSmall: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.qm_31_add_mul_opcode {
            writeln!(
                f,
                "Qm31AddMulOpcode: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.ret_opcode {
            writeln!(f, "RetOpcode: {}", indented_component_display(component))?;
        }
        if let Some(ref component) = self.verify_instruction {
            writeln!(
                f,
                "VerifyInstruction: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.blake_round {
            writeln!(f, "BlakeRound: {}", indented_component_display(component))?;
        }
        if let Some(ref component) = self.blake_g {
            writeln!(f, "BlakeG: {}", indented_component_display(component))?;
        }
        if let Some(ref component) = self.blake_round_sigma {
            writeln!(
                f,
                "BlakeRoundSigma: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.triple_xor_32 {
            writeln!(f, "TripleXor32: {}", indented_component_display(component))?;
        }
        if let Some(ref component) = self.verify_bitwise_xor_12 {
            writeln!(
                f,
                "VerifyBitwiseXor12: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.add_mod_builtin {
            writeln!(
                f,
                "AddModBuiltin: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.bitwise_builtin {
            writeln!(
                f,
                "BitwiseBuiltin: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.mul_mod_builtin {
            writeln!(
                f,
                "MulModBuiltin: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.pedersen_builtin {
            writeln!(
                f,
                "PedersenBuiltin: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.pedersen_builtin_narrow_windows {
            writeln!(
                f,
                "PedersenBuiltinNarrowWindows: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.poseidon_builtin {
            writeln!(
                f,
                "PoseidonBuiltin: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.range_check96_builtin {
            writeln!(
                f,
                "RangeCheck96Builtin: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.range_check_builtin {
            writeln!(
                f,
                "RangeCheckBuiltin: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.pedersen_aggregator_window_bits_18 {
            writeln!(
                f,
                "PedersenAggregatorWindowBits18: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.partial_ec_mul_window_bits_18 {
            writeln!(
                f,
                "PartialEcMulWindowBits18: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.pedersen_points_table_window_bits_18 {
            writeln!(
                f,
                "PedersenPointsTableWindowBits18: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.pedersen_aggregator_window_bits_9 {
            writeln!(
                f,
                "PedersenAggregatorWindowBits9: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.partial_ec_mul_window_bits_9 {
            writeln!(
                f,
                "PartialEcMulWindowBits9: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.pedersen_points_table_window_bits_9 {
            writeln!(
                f,
                "PedersenPointsTableWindowBits9: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.poseidon_aggregator {
            writeln!(
                f,
                "PoseidonAggregator: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.poseidon_3_partial_rounds_chain {
            writeln!(
                f,
                "Poseidon3PartialRoundsChain: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.poseidon_full_round_chain {
            writeln!(
                f,
                "PoseidonFullRoundChain: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.cube_252 {
            writeln!(f, "Cube252: {}", indented_component_display(component))?;
        }
        if let Some(ref component) = self.poseidon_round_keys {
            writeln!(
                f,
                "PoseidonRoundKeys: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.range_check_252_width_27 {
            writeln!(
                f,
                "RangeCheck252Width27: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.memory_address_to_id {
            writeln!(
                f,
                "MemoryAddressToId: {}",
                indented_component_display(component)
            )?;
        }
        for component in &self.memory_id_to_big {
            writeln!(
                f,
                "MemoryIdToValue: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.memory_id_to_small {
            writeln!(
                f,
                "MemoryIdToSmall: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.range_check_6 {
            writeln!(f, "RangeCheck6: {}", indented_component_display(component))?;
        }
        if let Some(ref component) = self.range_check_8 {
            writeln!(f, "RangeCheck8: {}", indented_component_display(component))?;
        }
        if let Some(ref component) = self.range_check_11 {
            writeln!(f, "RangeCheck11: {}", indented_component_display(component))?;
        }
        if let Some(ref component) = self.range_check_12 {
            writeln!(f, "RangeCheck12: {}", indented_component_display(component))?;
        }
        if let Some(ref component) = self.range_check_18 {
            writeln!(f, "RangeCheck18: {}", indented_component_display(component))?;
        }
        if let Some(ref component) = self.range_check_20 {
            writeln!(f, "RangeCheck20: {}", indented_component_display(component))?;
        }
        if let Some(ref component) = self.range_check_4_3 {
            writeln!(f, "RangeCheck43: {}", indented_component_display(component))?;
        }
        if let Some(ref component) = self.range_check_4_4 {
            writeln!(f, "RangeCheck44: {}", indented_component_display(component))?;
        }
        if let Some(ref component) = self.range_check_9_9 {
            writeln!(f, "RangeCheck99: {}", indented_component_display(component))?;
        }
        if let Some(ref component) = self.range_check_7_2_5 {
            writeln!(
                f,
                "RangeCheck725: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.range_check_3_6_6_3 {
            writeln!(
                f,
                "RangeCheck3663: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.range_check_4_4_4_4 {
            writeln!(
                f,
                "RangeCheck4444: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.range_check_3_3_3_3_3 {
            writeln!(
                f,
                "RangeCheck33333: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.verify_bitwise_xor_4 {
            writeln!(
                f,
                "VerifyBitwiseXor4: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.verify_bitwise_xor_7 {
            writeln!(
                f,
                "VerifyBitwiseXor7: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.verify_bitwise_xor_8 {
            writeln!(
                f,
                "VerifyBitwiseXor8: {}",
                indented_component_display(component)
            )?;
        }
        if let Some(ref component) = self.verify_bitwise_xor_9 {
            writeln!(
                f,
                "VerifyBitwiseXor9: {}",
                indented_component_display(component)
            )?;
        }
        Ok(())
    }
}
