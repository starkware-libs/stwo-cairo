use super::component::{Claim, InteractionClaim, BITWISE_XOR_8_LOG_SIZE, BITWISE_XOR_8_N_BITS};
use crate::cairo_air::preprocessed::BitwiseXor;
use crate::components::prelude::proving::*;

pub type InputType = [M31; 3];
pub type PackedInputType = [PackedM31; 3];

const N_TRACE_COLUMNS: usize = 1;
const PACKED_LOG_SIZE: u32 = BITWISE_XOR_8_LOG_SIZE - LOG_N_LANES;

pub struct ClaimGenerator {
    pub mults: AtomicMultiplicityColumn,
}
impl ClaimGenerator {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            mults: AtomicMultiplicityColumn::new(1 << BITWISE_XOR_8_LOG_SIZE),
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
        self.mults
            .increase_at((input[0].0 << BITWISE_XOR_8_N_BITS) + input[1].0);
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

fn write_trace_simd(mults: Vec<PackedM31>) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData) {
    let xor_a_column = BitwiseXor::new(BITWISE_XOR_8_N_BITS, 0);
    let xor_b_column = BitwiseXor::new(BITWISE_XOR_8_N_BITS, 1);
    let xor_c_column = BitwiseXor::new(BITWISE_XOR_8_N_BITS, 2);
    let (mut trace, mut lookup_data) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(BITWISE_XOR_8_LOG_SIZE),
            LookupData::uninitialized(PACKED_LOG_SIZE),
        )
    };

    trace
        .par_iter_mut()
        .enumerate()
        .zip(lookup_data.par_iter_mut())
        .for_each(|((row_index, mut row), lookup_data)| {
            *row[0] = mults[row_index];

            *lookup_data.bitwise_xor_trios = [
                xor_a_column.packed_at(row_index),
                xor_b_column.packed_at(row_index),
                xor_c_column.packed_at(row_index),
            ];
            *lookup_data.mults = mults[row_index];
        });

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    bitwise_xor_trios: Vec<[PackedM31; 3]>,
    mults: Vec<PackedM31>,
}

pub struct InteractionClaimGenerator {
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        verify_bitwise_xor_8: &relations::VerifyBitwiseXor_8,
    ) -> InteractionClaim {
        assert!(self.lookup_data.bitwise_xor_trios.len() == 1 << PACKED_LOG_SIZE);
        let mut logup_gen = LogupTraceGenerator::new(BITWISE_XOR_8_LOG_SIZE);

        let mut col_gen = logup_gen.new_col();
        for (i, (values, mults)) in self
            .lookup_data
            .bitwise_xor_trios
            .iter()
            .zip(self.lookup_data.mults)
            .enumerate()
        {
            let denom = verify_bitwise_xor_8.combine(values);
            col_gen.write_frac(i, -PackedQM31::one() * mults, denom);
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
