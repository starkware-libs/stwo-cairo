use stwo_prover::core::backend::simd::conversion::Pack;

use crate::cairo_air::pedersen::deduce_output;
use crate::components::pedersen_points_table;
use crate::components::prelude::proving::*;

// // TODO(Gali): Remove.
// #[allow(dead_code)]
// impl cube_252::ClaimGenerator {
//     pub fn deduce_output(&self, input: PackedFelt252Width27) -> PackedFelt252Width27 {
//         let unpacked_inputs = input.unpack();
//         <_ as Pack>::pack(unpacked_inputs.map(deduce_output::Cube252::deduce_output))
//     }
// }

// TODO(ohadn): Remove.
#[allow(dead_code)]
impl pedersen_points_table::ClaimGenerator {
    pub fn deduce_output(&self, input: [PackedM31; 1]) -> [PackedFelt252; 2] {
        let unpacked_inputs = input.unpack();
        <_ as Pack>::pack(
            unpacked_inputs.map(|row| deduce_output::PedersenPoints2::deduce_output(row[0])),
        )
    }
}
