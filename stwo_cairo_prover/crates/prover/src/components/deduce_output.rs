use stwo_prover::core::backend::simd::conversion::Pack;

use crate::cairo_air::poseidon::deduce_output;
use crate::components::prelude::proving::*;
use crate::components::{
    cube_252, poseidon_3_partial_rounds_chain, poseidon_full_round_chain, poseidon_round_keys,
};

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

// TODO(Gali): Remove.
#[allow(dead_code)]
impl poseidon_3_partial_rounds_chain::ClaimGenerator {
    pub fn deduce_output(
        &self,
        input: (PackedM31, PackedM31, [PackedFelt252Width27; 4]),
    ) -> (PackedM31, PackedM31, [PackedFelt252Width27; 4]) {
        let unpacked_inputs = input.unpack();
        <_ as Pack>::pack(unpacked_inputs.map(|(chain, round, state)| {
            deduce_output::Poseidon3PartialRoundsChain::deduce_output(chain, round, state)
        }))
    }
}

// TODO(Gali): Remove.
#[allow(dead_code)]
impl poseidon_full_round_chain::ClaimGenerator {
    pub fn deduce_output(
        &self,
        input: (PackedM31, PackedM31, [PackedFelt252Width27; 3]),
    ) -> (PackedM31, PackedM31, [PackedFelt252Width27; 3]) {
        let unpacked_inputs = input.unpack();
        <_ as Pack>::pack(unpacked_inputs.map(|(chain, round, state)| {
            deduce_output::PoseidonFullRoundChain::deduce_output(chain, round, state)
        }))
    }
}
