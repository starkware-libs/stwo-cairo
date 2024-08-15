#![feature(array_methods, portable_simd, iter_array_chunks)]
pub mod cairo_air;
pub mod components;
pub mod input;
pub mod prover_types;

#[cfg(test)]
mod tests {
    use cairo_lang_casm::casm;

    // TODO: Move next to the opcode.
    #[test]
    fn test_jmp_abs() {
        let instructions = casm! {
            call rel 2;
            [ap] = [ap-1] + 3;
            jmp abs [ap];
        }
        .instructions;

        let inp = stwo_cairo_runner::input_from_plain_casm(instructions);
        let usage = inp.instructions.usage();
        assert_eq!(usage.jmp_abs[0], 1);
        println!("Usage: {:#?}", usage);
    }
}
