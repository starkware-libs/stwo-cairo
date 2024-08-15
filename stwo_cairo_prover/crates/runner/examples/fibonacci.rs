use cairo_lang_casm::builder::{CasmBuildResult, CasmBuilder};
use cairo_lang_casm::casm_build_extend;
use stwo_cairo_runner::input_from_plain_casm;

fn main() {
    let mut builder = CasmBuilder::default();
    casm_build_extend! {builder,
        const one = 1;
        const ten = 10;

        tempvar a = one;
        tempvar n = ten;
        tempvar b = one;
        rescope{a = a, b = b, n = n, one = one};

        FIB:
        #{ steps = 0; }
        tempvar new_n = n - one;
        tempvar new_b = a + b;
        rescope{a = b, b = new_b, n = new_n, one = one};
        jump FIB if n != 0;
    };
    let CasmBuildResult { instructions, .. } = builder.build(["Fallthrough"]);

    let inp = input_from_plain_casm(instructions);
    println!("Usage: {:#?}", inp.instructions.usage());
}
