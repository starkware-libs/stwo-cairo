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

    pub fn write_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
    ) -> (Claim, InteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let mults = self.mults.into_simd_vec();
        assert!(mults.len().is_power_of_two());
        let (trace, lookup_data) = write_trace_simd(mults);
        tree_builder.extend_evals(trace.to_evals());

        (Claim {}, InteractionClaimGenerator { lookup_data }) //
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
            let pedersen_points_table: [PackedM31; PEDERSEN_TABLE_N_COLUMNS] =
                std::array::from_fn(|i| pedersen_points_table[i].packed_at(row_index));
            let seq = seq.packed_at(row_index);

            *lookup_data.pedersen_points_table_0 = [
                seq,
                pedersen_points_table[0],
                pedersen_points_table[1],
                pedersen_points_table[2],
                pedersen_points_table[3],
                pedersen_points_table[4],
                pedersen_points_table[5],
                pedersen_points_table[6],
                pedersen_points_table[7],
                pedersen_points_table[8],
                pedersen_points_table[9],
                pedersen_points_table[10],
                pedersen_points_table[11],
                pedersen_points_table[12],
                pedersen_points_table[13],
                pedersen_points_table[14],
                pedersen_points_table[15],
                pedersen_points_table[16],
                pedersen_points_table[17],
                pedersen_points_table[18],
                pedersen_points_table[19],
                pedersen_points_table[20],
                pedersen_points_table[21],
                pedersen_points_table[22],
                pedersen_points_table[23],
                pedersen_points_table[24],
                pedersen_points_table[25],
                pedersen_points_table[26],
                pedersen_points_table[27],
                pedersen_points_table[28],
                pedersen_points_table[29],
                pedersen_points_table[30],
                pedersen_points_table[31],
                pedersen_points_table[32],
                pedersen_points_table[33],
                pedersen_points_table[34],
                pedersen_points_table[35],
                pedersen_points_table[36],
                pedersen_points_table[37],
                pedersen_points_table[38],
                pedersen_points_table[39],
                pedersen_points_table[40],
                pedersen_points_table[41],
                pedersen_points_table[42],
                pedersen_points_table[43],
                pedersen_points_table[44],
                pedersen_points_table[45],
                pedersen_points_table[46],
                pedersen_points_table[47],
                pedersen_points_table[48],
                pedersen_points_table[49],
                pedersen_points_table[50],
                pedersen_points_table[51],
                pedersen_points_table[54],
                pedersen_points_table[53],
                pedersen_points_table[54],
                pedersen_points_table[55],
            ];
            *row[0] = mults[row_index];
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
        for (i, (values, mults)) in self
            .lookup_data
            .pedersen_points_table_0
            .iter()
            .zip(self.lookup_data.mults)
            .enumerate()
        {
            let denom = pedersen_points_table.combine(values);
            col_gen.write_frac(i, -PackedQM31::one() * mults, denom);
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
