#![allow(unused_parens)]
use stwo_cairo_common::preprocessed_consts::poseidon::N_ROUNDS;

use super::component::{Claim, InteractionClaim};
use crate::cairo_air::poseidon::const_columns::PoseidonRoundKeys;
use crate::cairo_air::preprocessed::Seq;
use crate::components::prelude::proving::*;
pub type InputType = [M31; 1];
pub type PackedInputType = [PackedM31; 1];
const N_TRACE_COLUMNS: usize = 1;

pub struct ClaimGenerator {
    pub mults: AtomicMultiplicityColumn,
}
impl ClaimGenerator {
    pub fn new() -> Self {
        Self {
            mults: AtomicMultiplicityColumn::new(N_ROUNDS),
        }
    }

    pub fn write_trace<MC: MerkleChannel>(
        mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
    ) -> (Claim, InteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let mults = self.mults.into_simd_vec();
        let log_size = mults.len().ilog2() + LOG_N_LANES;

        let (trace, lookup_data) = write_trace_simd(mults);

        tree_builder.extend_evals(trace.to_evals());

        (
            Claim { log_size },
            InteractionClaimGenerator {
                log_size,
                lookup_data,
            },
        )
    }

    pub fn add_input(&self, input: &InputType) {
        self.mults.increase_at(input[0].0);
    }

    pub fn add_inputs(&self, inputs: &[InputType]) {
        for input in inputs {
            self.add_input(input);
        }
    }
}

fn write_trace_simd(mults: Vec<PackedM31>) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData) {
    let log_n_packed_rows = mults.len().ilog2();
    let log_size = log_n_packed_rows + LOG_N_LANES;
    let (mut trace, mut lookup_data) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
        )
    };

    let poseidon_round_keys_0 = PoseidonRoundKeys::new(0);
    let poseidon_round_keys_1 = PoseidonRoundKeys::new(1);
    let poseidon_round_keys_2 = PoseidonRoundKeys::new(2);
    let poseidon_round_keys_3 = PoseidonRoundKeys::new(3);
    let poseidon_round_keys_4 = PoseidonRoundKeys::new(4);
    let poseidon_round_keys_5 = PoseidonRoundKeys::new(5);
    let poseidon_round_keys_6 = PoseidonRoundKeys::new(6);
    let poseidon_round_keys_7 = PoseidonRoundKeys::new(7);
    let poseidon_round_keys_8 = PoseidonRoundKeys::new(8);
    let poseidon_round_keys_9 = PoseidonRoundKeys::new(9);
    let poseidon_round_keys_10 = PoseidonRoundKeys::new(10);
    let poseidon_round_keys_11 = PoseidonRoundKeys::new(11);
    let poseidon_round_keys_12 = PoseidonRoundKeys::new(12);
    let poseidon_round_keys_13 = PoseidonRoundKeys::new(13);
    let poseidon_round_keys_14 = PoseidonRoundKeys::new(14);
    let poseidon_round_keys_15 = PoseidonRoundKeys::new(15);
    let poseidon_round_keys_16 = PoseidonRoundKeys::new(16);
    let poseidon_round_keys_17 = PoseidonRoundKeys::new(17);
    let poseidon_round_keys_18 = PoseidonRoundKeys::new(18);
    let poseidon_round_keys_19 = PoseidonRoundKeys::new(19);
    let poseidon_round_keys_20 = PoseidonRoundKeys::new(20);
    let poseidon_round_keys_21 = PoseidonRoundKeys::new(21);
    let poseidon_round_keys_22 = PoseidonRoundKeys::new(22);
    let poseidon_round_keys_23 = PoseidonRoundKeys::new(23);
    let poseidon_round_keys_24 = PoseidonRoundKeys::new(24);
    let poseidon_round_keys_25 = PoseidonRoundKeys::new(25);
    let poseidon_round_keys_26 = PoseidonRoundKeys::new(26);
    let poseidon_round_keys_27 = PoseidonRoundKeys::new(27);
    let poseidon_round_keys_28 = PoseidonRoundKeys::new(28);
    let poseidon_round_keys_29 = PoseidonRoundKeys::new(29);
    let seq = Seq::new(log_size);

    trace
        .par_iter_mut()
        .enumerate()
        .zip(lookup_data.par_iter_mut())
        .for_each(|((row_index, row), lookup_data)| {
            let poseidon_round_keys_0 = poseidon_round_keys_0.packed_at(row_index);
            let poseidon_round_keys_1 = poseidon_round_keys_1.packed_at(row_index);
            let poseidon_round_keys_2 = poseidon_round_keys_2.packed_at(row_index);
            let poseidon_round_keys_3 = poseidon_round_keys_3.packed_at(row_index);
            let poseidon_round_keys_4 = poseidon_round_keys_4.packed_at(row_index);
            let poseidon_round_keys_5 = poseidon_round_keys_5.packed_at(row_index);
            let poseidon_round_keys_6 = poseidon_round_keys_6.packed_at(row_index);
            let poseidon_round_keys_7 = poseidon_round_keys_7.packed_at(row_index);
            let poseidon_round_keys_8 = poseidon_round_keys_8.packed_at(row_index);
            let poseidon_round_keys_9 = poseidon_round_keys_9.packed_at(row_index);
            let poseidon_round_keys_10 = poseidon_round_keys_10.packed_at(row_index);
            let poseidon_round_keys_11 = poseidon_round_keys_11.packed_at(row_index);
            let poseidon_round_keys_12 = poseidon_round_keys_12.packed_at(row_index);
            let poseidon_round_keys_13 = poseidon_round_keys_13.packed_at(row_index);
            let poseidon_round_keys_14 = poseidon_round_keys_14.packed_at(row_index);
            let poseidon_round_keys_15 = poseidon_round_keys_15.packed_at(row_index);
            let poseidon_round_keys_16 = poseidon_round_keys_16.packed_at(row_index);
            let poseidon_round_keys_17 = poseidon_round_keys_17.packed_at(row_index);
            let poseidon_round_keys_18 = poseidon_round_keys_18.packed_at(row_index);
            let poseidon_round_keys_19 = poseidon_round_keys_19.packed_at(row_index);
            let poseidon_round_keys_20 = poseidon_round_keys_20.packed_at(row_index);
            let poseidon_round_keys_21 = poseidon_round_keys_21.packed_at(row_index);
            let poseidon_round_keys_22 = poseidon_round_keys_22.packed_at(row_index);
            let poseidon_round_keys_23 = poseidon_round_keys_23.packed_at(row_index);
            let poseidon_round_keys_24 = poseidon_round_keys_24.packed_at(row_index);
            let poseidon_round_keys_25 = poseidon_round_keys_25.packed_at(row_index);
            let poseidon_round_keys_26 = poseidon_round_keys_26.packed_at(row_index);
            let poseidon_round_keys_27 = poseidon_round_keys_27.packed_at(row_index);
            let poseidon_round_keys_28 = poseidon_round_keys_28.packed_at(row_index);
            let poseidon_round_keys_29 = poseidon_round_keys_29.packed_at(row_index);
            let seq = seq.packed_at(row_index);

            *lookup_data.poseidon_round_keys_0 = [
                seq,
                poseidon_round_keys_0,
                poseidon_round_keys_1,
                poseidon_round_keys_2,
                poseidon_round_keys_3,
                poseidon_round_keys_4,
                poseidon_round_keys_5,
                poseidon_round_keys_6,
                poseidon_round_keys_7,
                poseidon_round_keys_8,
                poseidon_round_keys_9,
                poseidon_round_keys_10,
                poseidon_round_keys_11,
                poseidon_round_keys_12,
                poseidon_round_keys_13,
                poseidon_round_keys_14,
                poseidon_round_keys_15,
                poseidon_round_keys_16,
                poseidon_round_keys_17,
                poseidon_round_keys_18,
                poseidon_round_keys_19,
                poseidon_round_keys_20,
                poseidon_round_keys_21,
                poseidon_round_keys_22,
                poseidon_round_keys_23,
                poseidon_round_keys_24,
                poseidon_round_keys_25,
                poseidon_round_keys_26,
                poseidon_round_keys_27,
                poseidon_round_keys_28,
                poseidon_round_keys_29,
            ];
            *row[0] = mults[row_index];
            *lookup_data.mults = mults[row_index];
        });

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    mults: Vec<PackedM31>,
    poseidon_round_keys_0: Vec<[PackedM31; 31]>,
}

pub struct InteractionClaimGenerator {
    log_size: u32,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        poseidon_round_keys: &relations::PoseidonRoundKeys,
    ) -> InteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        for (i, (values, mults)) in self
            .lookup_data
            .poseidon_round_keys_0
            .iter()
            .zip(self.lookup_data.mults)
            .enumerate()
        {
            let denom = poseidon_round_keys.combine(values);
            col_gen.write_frac(i, -PackedQM31::one() * mults, denom);
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
