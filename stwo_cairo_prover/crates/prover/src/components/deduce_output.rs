use stwo_prover::core::backend::simd::conversion::Pack;

use crate::cairo_air::poseidon::deduce_output;
use crate::components::prelude::proving::*;
use crate::components::{cube_252, poseidon_round_keys};

// TODO(Gali): Remove.
#[allow(dead_code)]
impl cube_252::ClaimGenerator {
    pub fn deduce_output(&self, input: PackedFelt252Width27) -> PackedFelt252Width27 {
        let unpacked_inputs = input.unpack();
        <_ as Pack>::pack(unpacked_inputs.map(deduce_output::Cube252::deduce_output))
    }
}

// TODO(Gali): Remove.
#[allow(dead_code)]
impl poseidon_round_keys::ClaimGenerator {
    pub fn deduce_output(&self, input: [PackedM31; 1]) -> [PackedFelt252Width27; 3] {
        let unpacked_inputs = input.unpack();
        <_ as Pack>::pack(
            unpacked_inputs.map(|round| deduce_output::PoseidonRoundKeys::deduce_output(round[0])),
        )
    }
}
