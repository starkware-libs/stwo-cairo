#![allow(unused_parens)]
#![allow(dead_code)]
use cairo_air::components::blake_round_sigma::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use super::component::LOG_SIZE;
use crate::witness::prelude::*;

pub type InputType = [M31; 1];
pub type PackedInputType = [PackedM31; 1];

pub struct ClaimGenerator {
    pub mults: AtomicMultiplicityColumn,
}
impl ClaimGenerator {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            mults: AtomicMultiplicityColumn::new(1 << LOG_SIZE),
        }
    }

    pub fn write_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
    ) -> (Claim, InteractionClaimGenerator) {
        let mults = self.mults.into_simd_vec();

        let (trace, lookup_data) = write_trace_simd(mults);
        tree_builder.extend_evals(trace.to_evals());

        (Claim {}, InteractionClaimGenerator { lookup_data })
    }

    pub fn add_input(&self, _input: &InputType) {
        todo!()
    }

    pub fn add_inputs(&self, inputs: &[InputType]) {
        for input in inputs {
            self.add_input(input);
        }
    }

    pub fn add_packed_inputs(&self, packed_inputs: &[PackedInputType]) {
        packed_inputs.into_par_iter().for_each(|packed_input| {
            packed_input.unpack().into_par_iter().for_each(|input| {
                self.add_input(&input);
            });
        });
    }
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(mults: Vec<PackedM31>) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData) {
    let log_n_packed_rows = LOG_SIZE - LOG_N_LANES;
    let (mut trace, mut lookup_data) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(LOG_SIZE),
            LookupData::uninitialized(log_n_packed_rows),
        )
    };

    let blakesigma_0 = BlakeSigma::new(0);
    let blakesigma_1 = BlakeSigma::new(1);
    let blakesigma_10 = BlakeSigma::new(10);
    let blakesigma_11 = BlakeSigma::new(11);
    let blakesigma_12 = BlakeSigma::new(12);
    let blakesigma_13 = BlakeSigma::new(13);
    let blakesigma_14 = BlakeSigma::new(14);
    let blakesigma_15 = BlakeSigma::new(15);
    let blakesigma_2 = BlakeSigma::new(2);
    let blakesigma_3 = BlakeSigma::new(3);
    let blakesigma_4 = BlakeSigma::new(4);
    let blakesigma_5 = BlakeSigma::new(5);
    let blakesigma_6 = BlakeSigma::new(6);
    let blakesigma_7 = BlakeSigma::new(7);
    let blakesigma_8 = BlakeSigma::new(8);
    let blakesigma_9 = BlakeSigma::new(9);
    let seq = Seq::new(LOG_SIZE);

    (trace.par_iter_mut(), lookup_data.par_iter_mut())
        .into_par_iter()
        .enumerate()
        .for_each(|(row_index, (mut row, lookup_data))| {
            let blakesigma_0 = blakesigma_0.packed_at(row_index);
            let blakesigma_1 = blakesigma_1.packed_at(row_index);
            let blakesigma_10 = blakesigma_10.packed_at(row_index);
            let blakesigma_11 = blakesigma_11.packed_at(row_index);
            let blakesigma_12 = blakesigma_12.packed_at(row_index);
            let blakesigma_13 = blakesigma_13.packed_at(row_index);
            let blakesigma_14 = blakesigma_14.packed_at(row_index);
            let blakesigma_15 = blakesigma_15.packed_at(row_index);
            let blakesigma_2 = blakesigma_2.packed_at(row_index);
            let blakesigma_3 = blakesigma_3.packed_at(row_index);
            let blakesigma_4 = blakesigma_4.packed_at(row_index);
            let blakesigma_5 = blakesigma_5.packed_at(row_index);
            let blakesigma_6 = blakesigma_6.packed_at(row_index);
            let blakesigma_7 = blakesigma_7.packed_at(row_index);
            let blakesigma_8 = blakesigma_8.packed_at(row_index);
            let blakesigma_9 = blakesigma_9.packed_at(row_index);
            let seq = seq.packed_at(row_index);
            *lookup_data.blake_round_sigma_0 = [
                seq,
                blakesigma_0,
                blakesigma_1,
                blakesigma_2,
                blakesigma_3,
                blakesigma_4,
                blakesigma_5,
                blakesigma_6,
                blakesigma_7,
                blakesigma_8,
                blakesigma_9,
                blakesigma_10,
                blakesigma_11,
                blakesigma_12,
                blakesigma_13,
                blakesigma_14,
                blakesigma_15,
            ];
            let mult_at_row = *mults.get(row_index).unwrap_or(&PackedM31::zero());
            *row[0] = mult_at_row;
            *lookup_data.mults = mult_at_row;
        });

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    blake_round_sigma_0: Vec<[PackedM31; 17]>,
    mults: Vec<PackedM31>,
}

pub struct InteractionClaimGenerator {
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        blake_round_sigma: &relations::BlakeRoundSigma,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(LOG_SIZE);

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.blake_round_sigma_0,
            self.lookup_data.mults,
        )
            .into_par_iter()
            .for_each(|(writer, values, mults)| {
                let denom = blake_round_sigma.combine(values);
                writer.write_frac(-PackedQM31::one() * mults, denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
