#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(dead_code)]
use super::component::{Claim, InteractionClaim};
use crate::components::prelude::proving::*;

pub type InputType = [M31; 2];
pub type PackedInputType = [PackedM31; 2];
const N_TRACE_COLUMNS: usize = 0;

#[derive(Default)]
pub struct ClaimGenerator {
    pub inputs: Vec<InputType>,
}
impl ClaimGenerator {
    pub fn new(inputs: Vec<InputType>) -> Self {
        Self { inputs }
    }

    pub fn write_trace<MC: MerkleChannel>(
        mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
    ) -> (Claim, InteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let n_rows = self.inputs.len();
        assert_ne!(n_rows, 0);
        let size = std::cmp::max(n_rows.next_power_of_two(), N_LANES);
        let log_size = size.ilog2();
        self.inputs.resize(size, *self.inputs.first().unwrap());
        let packed_inputs = pack_values(&self.inputs);

        let (trace, lookup_data) = write_trace_simd(n_rows, packed_inputs);

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
        unimplemented!("Implement manually");
    }

    pub fn add_inputs(&self, inputs: &[InputType]) {
        for input in inputs {
            self.add_input(input);
        }
    }
}

fn write_trace_simd(
    n_rows: usize,
    inputs: Vec<PackedInputType>,
) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData) {
    unimplemented!()
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    range_check_5_4_0: Vec<[PackedM31; 2]>,
}

pub struct InteractionClaimGenerator {
    log_size: u32,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        range_check_5_4: &relations::RangeCheck_5_4,
    ) -> InteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        for (i, values) in self.lookup_data.range_check_5_4_0.iter().enumerate() {
            let denom = range_check_5_4.combine(values);
            col_gen.write_frac(i, -PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
