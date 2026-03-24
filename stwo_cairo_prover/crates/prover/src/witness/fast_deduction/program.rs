use stwo::core::fields::m31::M31;
use stwo::prover::backend::simd::conversion::{Pack, Unpack};
use stwo::prover::backend::simd::m31::PackedM31;
use stwo_cairo_common::preprocessed_columns::program::{get_program_len, get_program_limbs};
use stwo_cairo_common::prover_types::cpu::Felt252;
use stwo_cairo_common::prover_types::simd::PackedFelt252;

pub struct PackedProgramComponent;
impl PackedProgramComponent {
    pub fn deduce_output(input: [PackedM31; 1]) -> (PackedFelt252, PackedM31) {
        let unpacked_inputs = input.unpack();
        let program_len = get_program_len();
        let (felt252s, conds): (Vec<_>, Vec<_>) = unpacked_inputs
            .into_iter()
            .map(|seq| {
                let index = seq[0].0 as usize;
                let limbs = get_program_limbs(index);
                let cond = if index < program_len { M31(1) } else { M31(0) };
                (Felt252::from_limbs(&limbs), cond)
            })
            .unzip();
        let packed_felt252: PackedFelt252 = <_ as Pack>::pack(felt252s.try_into().unwrap());
        let packed_cond: PackedM31 = PackedM31::from_array(conds.try_into().unwrap());
        (packed_felt252, packed_cond)
    }
}
