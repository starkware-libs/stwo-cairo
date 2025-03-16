#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::array::from_fn;

use stwo_cairo_common::preprocessed_consts::pedersen::PEDERSEN_TABLE_N_ROWS;

use super::component::{Claim, InteractionClaim, PEDERSEN_POINTS_TABLE_LOG_SIZE};
use crate::cairo_air::pedersen::const_columns::PEDERSEN_TABLE_N_COLUMNS;
use crate::cairo_air::preprocessed::PedersenPoints;
use crate::components::prelude::proving::*;

pub type InputType = [M31; 1];
pub type PackedInputType = [PackedM31; 1];
const N_TRACE_COLUMNS: usize = 1; //?
const PACKED_LOG_SIZE: u32 = PEDERSEN_POINTS_TABLE_LOG_SIZE - LOG_N_LANES;

pub struct ClaimGenerator {
    // pub inputs: Vec<InputType>, // pub mults: AtomicMultiplicityColumn,
    pub mults: AtomicMultiplicityColumn,
}
impl ClaimGenerator {
    // pub fn new(inputs: Vec<InputType>) -> Self {
    //     Self { inputs }
    // }
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            mults: AtomicMultiplicityColumn::new(1 << PEDERSEN_POINTS_TABLE_LOG_SIZE), // 2**23
        }
    }

    // pub fn write_trace<MC: MerkleChannel>(
    //     mut self,
    //     tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
    // ) -> (Claim, InteractionClaimGenerator)
    // where
    //     SimdBackend: BackendForChannel<MC>,
    // {
    //     let n_rows = self.inputs.len();
    //     assert_ne!(n_rows, 0);
    //     let size = std::cmp::max(n_rows.next_power_of_two(), N_LANES);
    //     let log_size = size.ilog2();
    //     self.inputs.resize(size, *self.inputs.first().unwrap());
    //     let packed_inputs = pack_values(&self.inputs);

    //     let (trace, lookup_data) = write_trace_simd(n_rows, packed_inputs);

    //     tree_builder.extend_evals(trace.to_evals());

    //     (
    //         Claim { log_size },
    //         InteractionClaimGenerator {
    //             log_size,
    //             lookup_data,
    //         },
    //     )
    // }

    pub fn write_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
    ) -> (Claim, InteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let mults = self.mults.into_simd_vec();
        let (trace, lookup_data) = write_trace_simd(mults);
        tree_builder.extend_evals(trace.to_evals());

        (Claim {}, InteractionClaimGenerator { lookup_data })
    }

    pub fn add_input(&self, input: &InputType) {
        self.mults.increase_at(input[0].0); // correct?
    }

    pub fn add_inputs(&self, inputs: &[InputType]) {
        for input in inputs {
            self.add_input(input);
        }
    }
}

fn write_trace_simd(mults: Vec<PackedM31>) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData) {
    // let pedersen_points_table_0: [PedersenPoints; PEDERSEN_TABLE_N_COLUMNS + 1] = from_fn(|i| {
    //     if i == 0 {
    //         Seq::new(PEDERSEN_POINTS_TABLE_LOG_SIZE) // Seq::new(self.log_size())
    //     } else {
    //         PedersenPoints::new(i - 1)
    //     }
    // });
    let pedersen_points_table: [PedersenPoints; PEDERSEN_TABLE_N_COLUMNS] =
        from_fn(PedersenPoints::new); // const outside function??

    let (mut trace, mut lookup_data) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(PEDERSEN_POINTS_TABLE_LOG_SIZE),
            LookupData::uninitialized(PACKED_LOG_SIZE),
        )
    };

    let seq = Seq::new(PEDERSEN_POINTS_TABLE_LOG_SIZE);

    trace
        .par_iter_mut()
        .enumerate()
        .zip(lookup_data.par_iter_mut())
        .for_each(|((row_index, mut row), lookup_data)| {
            *row[0] = mults[row_index];

            // let pedersen_points = pedersen_points_table[row_index];

            *lookup_data.pedersen_points_table_0 = from_fn(|i| {
                if i == 0 {
                    seq.packed_at(row_index) // Seq::new(self.log_size())
                } else {
                    pedersen_points_table[i - 1].packed_at(row_index)
                }
            });
            // [
            //     xor_a_column.packed_at(row_index),
            //     xor_b_column.packed_at(row_index),
            //     xor_c_column.packed_at(row_index),
            // ];
            *lookup_data.mults = mults[row_index];
        });

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    pedersen_points_table_0: Vec<[PackedM31; 57]>,
    mults: Vec<PackedM31>,
}

pub struct InteractionClaimGenerator {
    // log_size: u32,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        pedersen_points_table: &relations::PedersenPointsTable,
    ) -> InteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let mut logup_gen = LogupTraceGenerator::new(PEDERSEN_POINTS_TABLE_LOG_SIZE);

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        for (i, values) in self.lookup_data.pedersen_points_table_0.iter().enumerate() {
            let denom = pedersen_points_table.combine(values);
            col_gen.write_frac(i, -PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
