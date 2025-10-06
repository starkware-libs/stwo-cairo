// AIR version 98896da1-dirty
#![allow(unused_parens)]
use cairo_air::components::sha_256_small_sigma_0_o_0::{
    Claim, InteractionClaim, LOG_SIZE, N_TRACE_COLUMNS,
};

use crate::witness::prelude::*;

pub type InputType = [M31; 6];
pub type PackedInputType = [PackedM31; 6];

pub struct ClaimGenerator {
    pub mults: AtomicMultiplicityColumn,
}
impl Default for ClaimGenerator {
    fn default() -> Self {
        Self {
            mults: AtomicMultiplicityColumn::new(1 << LOG_SIZE),
        }
    }
}
impl ClaimGenerator {
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

    let sha256sigmatable_Sha256SigmaType_SmallSigma0O0_0 =
        Sha256SigmaTable::new(Sha256SigmaType::SmallSigma0O0, 0);
    let sha256sigmatable_Sha256SigmaType_SmallSigma0O0_1 =
        Sha256SigmaTable::new(Sha256SigmaType::SmallSigma0O0, 1);
    let sha256sigmatable_Sha256SigmaType_SmallSigma0O0_2 =
        Sha256SigmaTable::new(Sha256SigmaType::SmallSigma0O0, 2);
    let sha256sigmatable_Sha256SigmaType_SmallSigma0O0_3 =
        Sha256SigmaTable::new(Sha256SigmaType::SmallSigma0O0, 3);
    let sha256sigmatable_Sha256SigmaType_SmallSigma0O0_4 =
        Sha256SigmaTable::new(Sha256SigmaType::SmallSigma0O0, 4);
    let sha256sigmatable_Sha256SigmaType_SmallSigma0O0_5 =
        Sha256SigmaTable::new(Sha256SigmaType::SmallSigma0O0, 5);

    (trace.par_iter_mut(), lookup_data.par_iter_mut())
        .into_par_iter()
        .enumerate()
        .for_each(|(row_index, (mut row, lookup_data))| {
            let sha256sigmatable_Sha256SigmaType_SmallSigma0O0_0 =
                sha256sigmatable_Sha256SigmaType_SmallSigma0O0_0.packed_at(row_index);
            let sha256sigmatable_Sha256SigmaType_SmallSigma0O0_1 =
                sha256sigmatable_Sha256SigmaType_SmallSigma0O0_1.packed_at(row_index);
            let sha256sigmatable_Sha256SigmaType_SmallSigma0O0_2 =
                sha256sigmatable_Sha256SigmaType_SmallSigma0O0_2.packed_at(row_index);
            let sha256sigmatable_Sha256SigmaType_SmallSigma0O0_3 =
                sha256sigmatable_Sha256SigmaType_SmallSigma0O0_3.packed_at(row_index);
            let sha256sigmatable_Sha256SigmaType_SmallSigma0O0_4 =
                sha256sigmatable_Sha256SigmaType_SmallSigma0O0_4.packed_at(row_index);
            let sha256sigmatable_Sha256SigmaType_SmallSigma0O0_5 =
                sha256sigmatable_Sha256SigmaType_SmallSigma0O0_5.packed_at(row_index);
            *lookup_data.sha_256_small_sigma_0_o_0_0 = [
                sha256sigmatable_Sha256SigmaType_SmallSigma0O0_0,
                sha256sigmatable_Sha256SigmaType_SmallSigma0O0_1,
                sha256sigmatable_Sha256SigmaType_SmallSigma0O0_2,
                sha256sigmatable_Sha256SigmaType_SmallSigma0O0_3,
                sha256sigmatable_Sha256SigmaType_SmallSigma0O0_4,
                sha256sigmatable_Sha256SigmaType_SmallSigma0O0_5,
            ];
            let mult_at_row = *mults.get(row_index).unwrap_or(&PackedM31::zero());
            *row[0] = mult_at_row;
            *lookup_data.mults = mult_at_row;
        });

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    sha_256_small_sigma_0_o_0_0: Vec<[PackedM31; 6]>,
    mults: Vec<PackedM31>,
}

pub struct InteractionClaimGenerator {
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        sha_256_small_sigma_0_o_0: &relations::Sha256SmallSigma0O0,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(LOG_SIZE);

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.sha_256_small_sigma_0_o_0_0,
            self.lookup_data.mults,
        )
            .into_par_iter()
            .for_each(|(writer, values, mults)| {
                let denom = sha_256_small_sigma_0_o_0.combine(values);
                writer.write_frac(-PackedQM31::one() * mults, denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
