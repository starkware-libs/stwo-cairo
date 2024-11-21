#![feature(array_methods, portable_simd, iter_array_chunks, array_chunks)]
pub mod cairo_air;
pub mod components;
pub mod felt;
pub mod input;

#[cfg(test)]
mod tests {
    use cairo_lang_casm::casm;

    use crate::input::plain::input_from_plain_casm;

    // TODO: Move next to the opcode.
    #[test]
    fn test_jmp_abs() {
        let instructions = casm! {
            call rel 2;
            [ap] = [ap-1] + 3;
            jmp abs [ap];
        }
        .instructions;

        let inp = input_from_plain_casm(instructions);
        let state_transition_counts = inp.state_transition.counts();
        assert_eq!(
            state_transition_counts["jump_opcode_is_rel_f_is_imm_f_is_double_deref_f"],
            1
        );
        assert_eq!(
            state_transition_counts["call_opcode_is_rel_t_op1_base_fp_f"],
            1
        );
        assert_eq!(state_transition_counts["add_opcode_is_small_t_is_imm_t"], 1);
    }

    #[test]
    fn test_add_ap() {
        let instructions = casm! {
            [ap] = 38, ap++;
            [ap] = 12, ap++;
            ap += [ap -2];
            ap += [fp + 1];
        }
        .instructions;

        let inp = input_from_plain_casm(instructions);
        let state_transition_counts = inp.state_transition.counts();
        assert_eq!(
            state_transition_counts["add_ap_opcode_is_imm_f_op1_base_fp_f"],
            1
        );
        assert_eq!(
            state_transition_counts["add_ap_opcode_is_imm_f_op1_base_fp_t"],
            1
        );
    }

    #[test]
    fn test_call() {
        let instructions = casm! {
            call rel 2;
            call abs [fp - 1];
        }
        .instructions;

        let inp = input_from_plain_casm(instructions);
        let state_transition_counts = inp.state_transition.counts();
        assert_eq!(
            state_transition_counts["call_opcode_is_rel_f_op1_base_fp_t"],
            2
        );
        assert_eq!(
            state_transition_counts["call_opcode_is_rel_t_op1_base_fp_f"],
            1
        );
    }

    #[test]
    fn test_jnz_taken() {
        let instructions = casm! {
            [ap] = 0, ap++;
            jmp rel 2 if [ap-1] != 0;
        }
        .instructions;

        let inp = input_from_plain_casm(instructions);
        let state_transition_counts = inp.state_transition.counts();
        assert_eq!(
            state_transition_counts["jnz_opcode_is_taken_f_dst_base_fp_f"],
            1
        );
    }

    #[test]
    fn test_jnz_not_taken() {
        let instructions = casm! {
            call rel 2;
            jmp rel 2 if [fp-1] != 0;
        }
        .instructions;

        let inp = input_from_plain_casm(instructions);
        let state_transition_counts = inp.state_transition.counts();
        assert_eq!(
            state_transition_counts["jnz_opcode_is_taken_t_dst_base_fp_t"],
            1
        );
    }

    #[test]
    fn test_assert_equal() {
        let instructions = casm! {
            [ap] =  8, ap++;
            [ap] =  8, ap++;
            [ap+2] = [fp + 1];
        }
        .instructions;

        let inp = input_from_plain_casm(instructions);
        let state_transition_counts = inp.state_transition.counts();
        assert_eq!(
            state_transition_counts["assert_eq_opcode_is_double_deref_f_is_imm_f"],
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
        }
        .instructions;

        let inp = input_from_plain_casm(instructions);
        let state_transition_counts = inp.state_transition.counts();
        assert_eq!(state_transition_counts["add_opcode_is_small_t_is_imm_f"], 1);
    }

    #[test]
    fn test_mul() {
        let instructions = casm! {
            [ap] =  8, ap++;
            [ap] = 12, ap++;
            [ap] = [ap-1] * [ap-2], ap++;
            [ap] = [ap-1]*7, ap++;
        }
        .instructions;

        let inp = input_from_plain_casm(instructions);
        let state_transition_counts = inp.state_transition.counts();
        assert_eq!(state_transition_counts["mul_opcode_is_small_t_is_imm_f"], 1);
        assert_eq!(state_transition_counts["mul_opcode_is_small_t_is_imm_t"], 1);
    }
}
