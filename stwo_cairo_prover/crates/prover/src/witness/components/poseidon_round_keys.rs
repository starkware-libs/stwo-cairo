#![allow(unused_parens)]
use cairo_air::components::poseidon_round_keys::{Claim, InteractionClaim, N_TRACE_COLUMNS};
use cairo_air::poseidon::const_columns::PoseidonRoundKeys;
use cairo_air::preprocessed::Seq;
use stwo_cairo_common::preprocessed_consts::poseidon::{N_ROUNDS, N_WORDS};

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
        let log_size = N_ROUNDS.next_power_of_two().ilog2();

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

    pub fn add_packed_inputs(&self, inputs: &[PackedInputType]) {
        inputs.into_par_iter().for_each(|input| {
            self.add_inputs(&input.unpack());
        });
    }
}

fn write_trace_simd(mults: Vec<PackedM31>) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData) {
    let log_size = N_ROUNDS.next_power_of_two().ilog2();
    let log_n_packed_rows = log_size - LOG_N_LANES;
    let (mut trace, mut lookup_data) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
        )
    };
    let poseidon_round_keys: [PoseidonRoundKeys; N_WORDS] =
        std::array::from_fn(PoseidonRoundKeys::new);
    let seq = Seq::new(log_size);

    trace
        .par_iter_mut()
        .enumerate()
        .zip(lookup_data.par_iter_mut())
        .for_each(|((row_index, mut row), lookup_data)| {
            let poseidon_round_keys: [PackedM31; N_WORDS] =
                std::array::from_fn(|i| poseidon_round_keys[i].packed_at(row_index));
            let seq = seq.packed_at(row_index);

            *lookup_data.poseidon_round_keys_0 = [
                seq,
                poseidon_round_keys[0],
                poseidon_round_keys[1],
                poseidon_round_keys[2],
                poseidon_round_keys[3],
                poseidon_round_keys[4],
                poseidon_round_keys[5],
                poseidon_round_keys[6],
                poseidon_round_keys[7],
                poseidon_round_keys[8],
                poseidon_round_keys[9],
                poseidon_round_keys[10],
                poseidon_round_keys[11],
                poseidon_round_keys[12],
                poseidon_round_keys[13],
                poseidon_round_keys[14],
                poseidon_round_keys[15],
                poseidon_round_keys[16],
                poseidon_round_keys[17],
                poseidon_round_keys[18],
                poseidon_round_keys[19],
                poseidon_round_keys[20],
                poseidon_round_keys[21],
                poseidon_round_keys[22],
                poseidon_round_keys[23],
                poseidon_round_keys[24],
                poseidon_round_keys[25],
                poseidon_round_keys[26],
                poseidon_round_keys[27],
                poseidon_round_keys[28],
                poseidon_round_keys[29],
            ];

            let mult_at_row = *mults.get(row_index).unwrap_or(&PackedM31::zero());
            *row[0] = mult_at_row;
            *lookup_data.mults = mult_at_row;
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
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        poseidon_round_keys: &relations::PoseidonRoundKeys,
    ) -> InteractionClaim {
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
