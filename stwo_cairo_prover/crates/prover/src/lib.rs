#![feature(array_methods, portable_simd, iter_array_chunks, array_chunks)]
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
        }
        .instructions;

        let inp = input_from_plain_casm(instructions);
        let instruction_counts = inp.instructions.counts();
        assert_eq!(instruction_counts.jmp_abs[0], 1);
        println!("Instruction counts: {instruction_counts:#?}");
    }
}
