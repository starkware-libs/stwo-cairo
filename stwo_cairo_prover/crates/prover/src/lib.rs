#![feature(array_methods, portable_simd, iter_array_chunks, array_chunks)]
pub mod cairo_air;
pub mod components;
pub mod felt;
pub mod input;

#[cfg(test)]
mod tests {
    use cairo_lang_casm::casm;

    use crate::input::plain::input_from_plain_casm;

    // TODO(Ohad): un-ignore when the opcode is in.
    #[ignore]
    #[test]
    fn test_jmp_abs() {
        let instructions = casm! {
            call rel 2;
            [ap] = [ap-1] + 3;
            jmp abs [ap];
            [ap] = 1, ap++;
        }
        .instructions;

        let inp = input_from_plain_casm(instructions, false);
        let state_transitions_counts = inp.state_transitions.opcode_components_count();
        assert_eq!(
            state_transitions_counts["jump_opcode_is_rel_f_is_imm_f_is_double_deref_f"],
            1
        );
        assert_eq!(
            state_transitions_counts["call_opcode_is_rel_t_op1_base_fp_f"],
            1
        );
        assert_eq!(
            state_transitions_counts["add_opcode_is_small_t_is_imm_t"],
            1
        );
    }

    #[test]
    fn test_add_ap() {
        let instructions = casm! {
            [ap] = 38, ap++;
            [ap] = 12, ap++;
            ap += [ap -2];
            ap += [fp + 1];
            [ap] = 1, ap++;
        }
        .instructions;

        let inp = input_from_plain_casm(instructions, false);
        let state_transitions_counts = inp.state_transitions.opcode_components_count();
        println!("instruction count: {:?}", state_transitions_counts);
        assert_eq!(
            state_transitions_counts["add_ap_opcode_is_imm_f_op1_base_fp_f"],
            1
        );
        assert_eq!(
            state_transitions_counts["add_ap_opcode_is_imm_f_op1_base_fp_t"],
            1
        );
    }

    #[test]
    fn test_call() {
        let instructions = casm! {
            call rel 2;
            call abs [fp - 1];
            [ap] = 1, ap++;
        }
        .instructions;

        let inp = input_from_plain_casm(instructions, false);
        let state_transitions_counts = inp.state_transitions.opcode_components_count();
        assert_eq!(
            state_transitions_counts["call_opcode_is_rel_f_op1_base_fp_t"],
            2
        );
        assert_eq!(
            state_transitions_counts["call_opcode_is_rel_t_op1_base_fp_f"],
            1
        );
    }

    #[test]
    fn test_jnz_taken() {
        let instructions = casm! {
            [ap] = 0, ap++;
            jmp rel 2 if [ap-1] != 0;
            [ap] = 1, ap++;
        }
        .instructions;

        let inp = input_from_plain_casm(instructions, false);
        let state_transitions_counts = inp.state_transitions.opcode_components_count();
        assert_eq!(
            state_transitions_counts["jnz_opcode_is_taken_f_dst_base_fp_f"],
            1
        );
    }

    #[test]
    fn test_jnz_not_taken() {
        let instructions = casm! {
            call rel 2;
            jmp rel 2 if [fp-1] != 0;
            [ap] = 1, ap++;
        }
        .instructions;

        let inp = input_from_plain_casm(instructions, false);
        let state_transitions_counts = inp.state_transitions.opcode_components_count();
        assert_eq!(
            state_transitions_counts["jnz_opcode_is_taken_t_dst_base_fp_t"],
            1
        );
    }

    #[test]
    fn test_assert_equal() {
        let instructions = casm! {
            [ap] =  8, ap++;
            [ap] =  8, ap++;
            [ap+2] = [fp + 1];
            [ap] = 1, ap++;
        }
        .instructions;

        let inp = input_from_plain_casm(instructions, false);
        let state_transitions_counts = inp.state_transitions.opcode_components_count();
        assert_eq!(
            state_transitions_counts["assert_eq_opcode_is_double_deref_f_is_imm_f"],
            1
        );
    }

    #[test]
    fn test_add() {
        let instructions = casm! {
            call rel 2;
            [ap] =  8, ap++;
            [ap] = 12, ap++;
            [ap+2] = [fp-1] + [ap-2];
            [ap] = 1, ap++;
        }
        .instructions;

        let inp = input_from_plain_casm(instructions, false);
        let state_transitions_counts = inp.state_transitions.opcode_components_count();
        assert_eq!(
            state_transitions_counts["add_opcode_is_small_t_is_imm_f"],
            1
        );
    }

    #[test]
    fn test_mul() {
        let instructions = casm! {
            [ap] =  8, ap++;
            [ap] = 12, ap++;
            [ap] = [ap-1] * [ap-2], ap++;
            [ap] = [ap-1]*7, ap++;
            [ap] = 1, ap++;
        }
        .instructions;

        let inp = input_from_plain_casm(instructions, false);
        let state_transitions_counts = inp.state_transitions.opcode_components_count();
        assert_eq!(
            state_transitions_counts["mul_opcode_is_small_t_is_imm_f"],
            1
        );
        assert_eq!(
            state_transitions_counts["mul_opcode_is_small_t_is_imm_t"],
            1
        );
    }
}
