use stwo::prover::backend::simd::conversion::{Pack, Unpack};
use stwo::prover::backend::simd::m31::PackedM31;
use stwo_cairo_common::preprocessed_columns::program::get_program_limbs;
use stwo_cairo_common::prover_types::cpu::Felt252;
use stwo_cairo_common::prover_types::simd::PackedFelt252;

pub struct PackedProgramComponent;
impl PackedProgramComponent {
    pub fn deduce_output(input: [PackedM31; 1]) -> PackedFelt252 {
        let unpacked_inputs = input.unpack();
        <_ as Pack>::pack(unpacked_inputs.map(|seq| {
            let limbs = get_program_limbs(seq[0].0 as usize);
            Felt252::from_limbs(&limbs)
        }))
    }
}
