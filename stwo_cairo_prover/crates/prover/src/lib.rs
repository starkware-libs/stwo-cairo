#![feature(array_methods, portable_simd, iter_array_chunks, array_chunks)]
// TODO(Ohad): remove.
#![allow(clippy::too_many_arguments)]

pub mod cairo_air;
pub mod components;
pub mod felt;
pub mod input;
pub mod relations;

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
        let casm_states_by_opcode = inp.state_transitions.casm_states_by_opcode;
        assert_eq!(
            casm_states_by_opcode
                .jump_opcode_is_rel_f_is_imm_f_is_double_deref_f
                .len(),
            1
        );
        assert_eq!(
            casm_states_by_opcode
                .call_opcode_is_rel_t_op1_base_fp_f
                .len(),
            1
        );
        assert_eq!(
            casm_states_by_opcode.add_opcode_is_small_t_is_imm_t.len(),
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
        let casm_states_by_opcode = inp.state_transitions.casm_states_by_opcode;
        assert_eq!(
            casm_states_by_opcode
                .add_ap_opcode_is_imm_f_op1_base_fp_f
                .len(),
            1
        );
        assert_eq!(
            casm_states_by_opcode
                .add_ap_opcode_is_imm_f_op1_base_fp_f
                .len(),
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
        let casm_states_by_opcode = inp.state_transitions.casm_states_by_opcode;
        assert_eq!(
            casm_states_by_opcode
                .call_opcode_is_rel_f_op1_base_fp_t
                .len(),
            2
        );
        assert_eq!(
            casm_states_by_opcode
                .call_opcode_is_rel_t_op1_base_fp_f
                .len(),
            1
        );
    }

    #[test]
    fn test_call2() {
        let instructions = casm! {
            call rel 2;
            call abs [ap - 1];
            [ap] = 1, ap++;
        }
        .instructions;

        let inp = input_from_plain_casm(instructions, false);
        let casm_states_by_opcode = inp.state_transitions.casm_states_by_opcode;
        assert_eq!(
            casm_states_by_opcode
                .call_opcode_is_rel_f_op1_base_fp_f
                .len(),
            2
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
        let casm_states_by_opcode = inp.state_transitions.casm_states_by_opcode;
        assert_eq!(
            casm_states_by_opcode
                .jnz_opcode_is_taken_f_dst_base_fp_f
                .len(),
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
        let casm_states_by_opcode = inp.state_transitions.casm_states_by_opcode;
        assert_eq!(
            casm_states_by_opcode
                .jnz_opcode_is_taken_t_dst_base_fp_t
                .len(),
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
        let casm_states_by_opcode = inp.state_transitions.casm_states_by_opcode;
        assert_eq!(
            casm_states_by_opcode
                .assert_eq_opcode_is_double_deref_f_is_imm_f
                .len(),
            1
        );
    }

    // TODO(Ohad/Stav): un-ignore.
    #[ignore = "disabled until adapter is fixed"]
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
        let casm_states_by_opcode = inp.state_transitions.casm_states_by_opcode;
        assert_eq!(
            casm_states_by_opcode.add_opcode_is_small_t_is_imm_f.len(),
            1
        );
    }

    // TODO(Ohad): un-ignore.
    #[ignore = "mul small opcode is not implemented yet"]
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
        let casm_states_by_opcode = inp.state_transitions.casm_states_by_opcode;
        assert_eq!(
            casm_states_by_opcode.mul_opcode_is_small_t_is_imm_f.len(),
            1
        );
        assert_eq!(
            casm_states_by_opcode.mul_opcode_is_small_t_is_imm_t.len(),
            1
        );
    }
}
