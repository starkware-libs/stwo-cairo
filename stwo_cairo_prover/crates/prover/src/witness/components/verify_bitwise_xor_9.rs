#![allow(unused_parens)]
#![allow(dead_code)]
use cairo_air::components::verify_bitwise_xor_9::{
    Claim, InteractionClaim, LOG_SIZE, N_BITS, N_TRACE_COLUMNS,
};

use crate::witness::prelude::*;

pub type InputType = [M31; 3];
pub type PackedInputType = [PackedM31; 3];

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

    pub fn add_input(&self, input: &InputType) {
        self.mults.increase_at((input[0].0 << N_BITS) + input[1].0);
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

    let bitwisexor_9_0 = BitwiseXor::new(9, 0);
    let bitwisexor_9_1 = BitwiseXor::new(9, 1);
    let bitwisexor_9_2 = BitwiseXor::new(9, 2);

    (trace.par_iter_mut(), lookup_data.par_iter_mut())
        .into_par_iter()
        .enumerate()
        .for_each(|(row_index, (mut row, lookup_data))| {
            let bitwisexor_9_0 = bitwisexor_9_0.packed_at(row_index);
            let bitwisexor_9_1 = bitwisexor_9_1.packed_at(row_index);
            let bitwisexor_9_2 = bitwisexor_9_2.packed_at(row_index);
            *lookup_data.verify_bitwise_xor_9_0 = [bitwisexor_9_0, bitwisexor_9_1, bitwisexor_9_2];
            let mult_at_row = *mults.get(row_index).unwrap_or(&PackedM31::zero());
            *row[0] = mult_at_row;
            *lookup_data.mults = mult_at_row;
        });

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    verify_bitwise_xor_9_0: Vec<[PackedM31; 3]>,
    mults: Vec<PackedM31>,
}

pub struct InteractionClaimGenerator {
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        verify_bitwise_xor_9: &relations::VerifyBitwiseXor_9,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(LOG_SIZE);

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_xor_9_0,
            self.lookup_data.mults,
        )
            .into_par_iter()
            .for_each(|(writer, values, mults)| {
                let denom = verify_bitwise_xor_9.combine(values);
                writer.write_frac(-PackedQM31::one() * mults, denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
