#![allow(unused_parens)]
#![allow(dead_code)]
use cairo_air::components::poseidon_round_keys::{
    Claim, InteractionClaim, LOG_SIZE, N_TRACE_COLUMNS,
};
use stwo_cairo_common::preprocessed_consts::poseidon::N_ROUNDS;

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
            mults: AtomicMultiplicityColumn::new(N_ROUNDS),
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

    pub fn add_input(&self, input: &InputType) {
        self.mults.increase_at(input[0].0);
    }

    pub fn add_packed_inputs(&self, packed_inputs: &[PackedInputType]) {
        packed_inputs.into_par_iter().for_each(|packed_input| {
            packed_input.unpack().into_iter().for_each(|input| {
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

    let poseidonroundkeys_0 = PoseidonRoundKeys::new(0);
    let poseidonroundkeys_1 = PoseidonRoundKeys::new(1);
    let poseidonroundkeys_2 = PoseidonRoundKeys::new(2);
    let poseidonroundkeys_3 = PoseidonRoundKeys::new(3);
    let poseidonroundkeys_4 = PoseidonRoundKeys::new(4);
    let poseidonroundkeys_5 = PoseidonRoundKeys::new(5);
    let poseidonroundkeys_6 = PoseidonRoundKeys::new(6);
    let poseidonroundkeys_7 = PoseidonRoundKeys::new(7);
    let poseidonroundkeys_8 = PoseidonRoundKeys::new(8);
    let poseidonroundkeys_9 = PoseidonRoundKeys::new(9);
    let poseidonroundkeys_10 = PoseidonRoundKeys::new(10);
    let poseidonroundkeys_11 = PoseidonRoundKeys::new(11);
    let poseidonroundkeys_12 = PoseidonRoundKeys::new(12);
    let poseidonroundkeys_13 = PoseidonRoundKeys::new(13);
    let poseidonroundkeys_14 = PoseidonRoundKeys::new(14);
    let poseidonroundkeys_15 = PoseidonRoundKeys::new(15);
    let poseidonroundkeys_16 = PoseidonRoundKeys::new(16);
    let poseidonroundkeys_17 = PoseidonRoundKeys::new(17);
    let poseidonroundkeys_18 = PoseidonRoundKeys::new(18);
    let poseidonroundkeys_19 = PoseidonRoundKeys::new(19);
    let poseidonroundkeys_20 = PoseidonRoundKeys::new(20);
    let poseidonroundkeys_21 = PoseidonRoundKeys::new(21);
    let poseidonroundkeys_22 = PoseidonRoundKeys::new(22);
    let poseidonroundkeys_23 = PoseidonRoundKeys::new(23);
    let poseidonroundkeys_24 = PoseidonRoundKeys::new(24);
    let poseidonroundkeys_25 = PoseidonRoundKeys::new(25);
    let poseidonroundkeys_26 = PoseidonRoundKeys::new(26);
    let poseidonroundkeys_27 = PoseidonRoundKeys::new(27);
    let poseidonroundkeys_28 = PoseidonRoundKeys::new(28);
    let poseidonroundkeys_29 = PoseidonRoundKeys::new(29);
    let seq = Seq::new(LOG_SIZE);

    (trace.par_iter_mut(), lookup_data.par_iter_mut())
        .into_par_iter()
        .enumerate()
        .for_each(|(row_index, (mut row, lookup_data))| {
            let poseidonroundkeys_0 = poseidonroundkeys_0.packed_at(row_index);
            let poseidonroundkeys_1 = poseidonroundkeys_1.packed_at(row_index);
            let poseidonroundkeys_2 = poseidonroundkeys_2.packed_at(row_index);
            let poseidonroundkeys_3 = poseidonroundkeys_3.packed_at(row_index);
            let poseidonroundkeys_4 = poseidonroundkeys_4.packed_at(row_index);
            let poseidonroundkeys_5 = poseidonroundkeys_5.packed_at(row_index);
            let poseidonroundkeys_6 = poseidonroundkeys_6.packed_at(row_index);
            let poseidonroundkeys_7 = poseidonroundkeys_7.packed_at(row_index);
            let poseidonroundkeys_8 = poseidonroundkeys_8.packed_at(row_index);
            let poseidonroundkeys_9 = poseidonroundkeys_9.packed_at(row_index);
            let poseidonroundkeys_10 = poseidonroundkeys_10.packed_at(row_index);
            let poseidonroundkeys_11 = poseidonroundkeys_11.packed_at(row_index);
            let poseidonroundkeys_12 = poseidonroundkeys_12.packed_at(row_index);
            let poseidonroundkeys_13 = poseidonroundkeys_13.packed_at(row_index);
            let poseidonroundkeys_14 = poseidonroundkeys_14.packed_at(row_index);
            let poseidonroundkeys_15 = poseidonroundkeys_15.packed_at(row_index);
            let poseidonroundkeys_16 = poseidonroundkeys_16.packed_at(row_index);
            let poseidonroundkeys_17 = poseidonroundkeys_17.packed_at(row_index);
            let poseidonroundkeys_18 = poseidonroundkeys_18.packed_at(row_index);
            let poseidonroundkeys_19 = poseidonroundkeys_19.packed_at(row_index);
            let poseidonroundkeys_20 = poseidonroundkeys_20.packed_at(row_index);
            let poseidonroundkeys_21 = poseidonroundkeys_21.packed_at(row_index);
            let poseidonroundkeys_22 = poseidonroundkeys_22.packed_at(row_index);
            let poseidonroundkeys_23 = poseidonroundkeys_23.packed_at(row_index);
            let poseidonroundkeys_24 = poseidonroundkeys_24.packed_at(row_index);
            let poseidonroundkeys_25 = poseidonroundkeys_25.packed_at(row_index);
            let poseidonroundkeys_26 = poseidonroundkeys_26.packed_at(row_index);
            let poseidonroundkeys_27 = poseidonroundkeys_27.packed_at(row_index);
            let poseidonroundkeys_28 = poseidonroundkeys_28.packed_at(row_index);
            let poseidonroundkeys_29 = poseidonroundkeys_29.packed_at(row_index);
            let seq = seq.packed_at(row_index);
            *lookup_data.poseidon_round_keys_0 = [
                seq,
                poseidonroundkeys_0,
                poseidonroundkeys_1,
                poseidonroundkeys_2,
                poseidonroundkeys_3,
                poseidonroundkeys_4,
                poseidonroundkeys_5,
                poseidonroundkeys_6,
                poseidonroundkeys_7,
                poseidonroundkeys_8,
                poseidonroundkeys_9,
                poseidonroundkeys_10,
                poseidonroundkeys_11,
                poseidonroundkeys_12,
                poseidonroundkeys_13,
                poseidonroundkeys_14,
                poseidonroundkeys_15,
                poseidonroundkeys_16,
                poseidonroundkeys_17,
                poseidonroundkeys_18,
                poseidonroundkeys_19,
                poseidonroundkeys_20,
                poseidonroundkeys_21,
                poseidonroundkeys_22,
                poseidonroundkeys_23,
                poseidonroundkeys_24,
                poseidonroundkeys_25,
                poseidonroundkeys_26,
                poseidonroundkeys_27,
                poseidonroundkeys_28,
                poseidonroundkeys_29,
            ];
            let mult_at_row = *mults.get(row_index).unwrap_or(&PackedM31::zero());
            *row[0] = mult_at_row;
            *lookup_data.mults = mult_at_row;
        });

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    poseidon_round_keys_0: Vec<[PackedM31; 31]>,
    mults: Vec<PackedM31>,
}

pub struct InteractionClaimGenerator {
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        poseidon_round_keys: &relations::PoseidonRoundKeys,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(LOG_SIZE);

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.poseidon_round_keys_0,
            self.lookup_data.mults,
        )
            .into_par_iter()
            .for_each(|(writer, values, mults)| {
                let denom = poseidon_round_keys.combine(values);
                writer.write_frac(-PackedQM31::one() * mults, denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
