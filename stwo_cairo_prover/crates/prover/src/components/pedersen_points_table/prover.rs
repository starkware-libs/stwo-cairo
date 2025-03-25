#![allow(unused_parens)]
#![allow(dead_code)]
use super::component::{Claim, InteractionClaim, N_TRACE_COLUMNS};
use crate::components::prelude::proving::*;

pub type InputType = [M31; 1];
pub type PackedInputType = [PackedM31; 1];

#[derive(Default)]
pub struct ClaimGenerator {
    pub inputs: Vec<InputType>,
}
impl ClaimGenerator {
    pub fn new(inputs: Vec<InputType>) -> Self {
        Self { inputs }
    }

    pub fn write_trace(
        mut self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
    ) -> (Claim, InteractionClaimGenerator) {
        let n_rows = self.inputs.len();
        assert_ne!(n_rows, 0);
        let size = std::cmp::max(n_rows.next_power_of_two(), N_LANES);
        let log_size = size.ilog2();
        self.inputs.resize(size, *self.inputs.first().unwrap());
        let packed_inputs = pack_values(&self.inputs);

        let (trace, lookup_data, sub_component_inputs) = write_trace_simd(n_rows, packed_inputs);
        tree_builder.extend_evals(trace.to_evals());

        (
            Claim { log_size },
            InteractionClaimGenerator {
                log_size,
                lookup_data,
            },
        )
    }

    pub fn add_packed_input(&self, input: &PackedInputType) {
        unimplemented!("Implement manually");
    }

    pub fn add_packed_inputs(&self, inputs: &[PackedInputType]) {
        unimplemented!("Implement manually");
    }
}

struct SubComponentInputs {}

fn write_trace_simd(
    n_rows: usize,
    inputs: Vec<PackedInputType>,
) -> (
    ComponentTrace<N_TRACE_COLUMNS>,
    LookupData,
    SubComponentInputs,
) {
    unimplemented!()
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    pedersen_points_table_0: Vec<[PackedM31; 57]>,
}

pub struct InteractionClaimGenerator {
    log_size: u32,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        pedersen_points_table: &relations::PedersenPointsTable,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

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
