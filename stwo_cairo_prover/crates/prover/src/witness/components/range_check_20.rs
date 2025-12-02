// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::range_check_20::{Claim, InteractionClaim, LOG_SIZE, N_TRACE_COLUMNS};

use crate::witness::prelude::*;

pub type InputType = [M31; 1];
pub type PackedInputType = [PackedM31; 1];

pub struct ClaimGenerator {
    pub mults: [AtomicMultiplicityColumn; 8],
    input_to_row: HashMap<[M31; 1], usize>,
    preprocessed_trace: Arc<PreProcessedTrace>,
}

impl ClaimGenerator {
    pub fn new(preprocessed_trace: Arc<PreProcessedTrace>) -> Self {
        let mults = from_fn(|_| AtomicMultiplicityColumn::new(1 << LOG_SIZE));
        let column_ids = [PreProcessedColumnId {
            id: "seq_20".to_owned(),
        }];

        Self {
            mults,
            input_to_row: make_input_to_row(&preprocessed_trace, column_ids),
            preprocessed_trace,
        }
    }

    pub fn write_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
    ) -> (Claim, InteractionClaimGenerator) {
        let mults = self
            .mults
            .into_iter()
            .map(|v| v.into_simd_vec())
            .collect::<Vec<_>>();

        let (trace, lookup_data) = write_trace_simd(&self.preprocessed_trace, mults);
        tree_builder.extend_evals(trace.to_evals());

        (Claim {}, InteractionClaimGenerator { lookup_data })
    }

    pub fn add_input(&self, input: &InputType, relation_name: &str) {
        let rel_ind = [
            "RangeCheck_20",
            "RangeCheck_20_B",
            "RangeCheck_20_C",
            "RangeCheck_20_D",
            "RangeCheck_20_E",
            "RangeCheck_20_F",
            "RangeCheck_20_G",
            "RangeCheck_20_H",
        ]
        .iter()
        .position(|r| *r == relation_name)
        .unwrap();
        self.mults[rel_ind]
            .increase_at((*self.input_to_row.get(input).unwrap()).try_into().unwrap());
    }

    pub fn add_packed_inputs(&self, packed_inputs: &[PackedInputType], relation_name: &str) {
        packed_inputs.into_par_iter().for_each(|packed_input| {
            packed_input.unpack().into_par_iter().for_each(|input| {
                self.add_input(&input, relation_name);
            });
        });
    }
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    preprocessed_trace: &PreProcessedTrace,
    mults: Vec<Vec<PackedM31>>,
) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData) {
    let log_n_packed_rows = LOG_SIZE - LOG_N_LANES;
    let (mut trace, mut lookup_data) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(LOG_SIZE),
            LookupData::uninitialized(log_n_packed_rows),
        )
    };

    let seq_20 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "seq_20".to_owned(),
    });

    (trace.par_iter_mut(), lookup_data.par_iter_mut())
        .into_par_iter()
        .enumerate()
        .for_each(|(row_index, (row, lookup_data))| {
            let seq_20 = seq_20.packed_at(row_index);
            *lookup_data.range_check_20_0 = [seq_20];
            *lookup_data.range_check_20_b_0 = [seq_20];
            *lookup_data.range_check_20_c_0 = [seq_20];
            *lookup_data.range_check_20_d_0 = [seq_20];
            *lookup_data.range_check_20_e_0 = [seq_20];
            *lookup_data.range_check_20_f_0 = [seq_20];
            *lookup_data.range_check_20_g_0 = [seq_20];
            *lookup_data.range_check_20_h_0 = [seq_20];
            let mult = &mults[0];
            let mult_at_row = *mult.get(row_index).unwrap_or(&PackedM31::zero());
            *row[0] = mult_at_row;
            *lookup_data.mults_0 = mult_at_row;
            let mult = &mults[1];
            let mult_at_row = *mult.get(row_index).unwrap_or(&PackedM31::zero());
            *row[1] = mult_at_row;
            *lookup_data.mults_1 = mult_at_row;
            let mult = &mults[2];
            let mult_at_row = *mult.get(row_index).unwrap_or(&PackedM31::zero());
            *row[2] = mult_at_row;
            *lookup_data.mults_2 = mult_at_row;
            let mult = &mults[3];
            let mult_at_row = *mult.get(row_index).unwrap_or(&PackedM31::zero());
            *row[3] = mult_at_row;
            *lookup_data.mults_3 = mult_at_row;
            let mult = &mults[4];
            let mult_at_row = *mult.get(row_index).unwrap_or(&PackedM31::zero());
            *row[4] = mult_at_row;
            *lookup_data.mults_4 = mult_at_row;
            let mult = &mults[5];
            let mult_at_row = *mult.get(row_index).unwrap_or(&PackedM31::zero());
            *row[5] = mult_at_row;
            *lookup_data.mults_5 = mult_at_row;
            let mult = &mults[6];
            let mult_at_row = *mult.get(row_index).unwrap_or(&PackedM31::zero());
            *row[6] = mult_at_row;
            *lookup_data.mults_6 = mult_at_row;
            let mult = &mults[7];
            let mult_at_row = *mult.get(row_index).unwrap_or(&PackedM31::zero());
            *row[7] = mult_at_row;
            *lookup_data.mults_7 = mult_at_row;
        });

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    range_check_20_0: Vec<[PackedM31; 1]>,
    range_check_20_b_0: Vec<[PackedM31; 1]>,
    range_check_20_c_0: Vec<[PackedM31; 1]>,
    range_check_20_d_0: Vec<[PackedM31; 1]>,
    range_check_20_e_0: Vec<[PackedM31; 1]>,
    range_check_20_f_0: Vec<[PackedM31; 1]>,
    range_check_20_g_0: Vec<[PackedM31; 1]>,
    range_check_20_h_0: Vec<[PackedM31; 1]>,
    mults_0: Vec<PackedM31>,
    mults_1: Vec<PackedM31>,
    mults_2: Vec<PackedM31>,
    mults_3: Vec<PackedM31>,
    mults_4: Vec<PackedM31>,
    mults_5: Vec<PackedM31>,
    mults_6: Vec<PackedM31>,
    mults_7: Vec<PackedM31>,
}

pub struct InteractionClaimGenerator {
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        range_check_20: &relations::RangeCheck_20,
        range_check_20_b: &relations::RangeCheck_20_B,
        range_check_20_c: &relations::RangeCheck_20_C,
        range_check_20_d: &relations::RangeCheck_20_D,
        range_check_20_e: &relations::RangeCheck_20_E,
        range_check_20_f: &relations::RangeCheck_20_F,
        range_check_20_g: &relations::RangeCheck_20_G,
        range_check_20_h: &relations::RangeCheck_20_H,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(LOG_SIZE);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_0,
            &self.lookup_data.range_check_20_b_0,
            self.lookup_data.mults_0,
            self.lookup_data.mults_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mults_0, mults_1)| {
                let denom0: PackedQM31 = range_check_20.combine(values0);
                let denom1: PackedQM31 = range_check_20_b.combine(values1);
                writer.write_frac(-(denom0 * mults_1 + denom1 * mults_0), denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_c_0,
            &self.lookup_data.range_check_20_d_0,
            self.lookup_data.mults_2,
            self.lookup_data.mults_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mults_2, mults_3)| {
                let denom0: PackedQM31 = range_check_20_c.combine(values0);
                let denom1: PackedQM31 = range_check_20_d.combine(values1);
                writer.write_frac(-(denom0 * mults_3 + denom1 * mults_2), denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_e_0,
            &self.lookup_data.range_check_20_f_0,
            self.lookup_data.mults_4,
            self.lookup_data.mults_5,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mults_4, mults_5)| {
                let denom0: PackedQM31 = range_check_20_e.combine(values0);
                let denom1: PackedQM31 = range_check_20_f.combine(values1);
                writer.write_frac(-(denom0 * mults_5 + denom1 * mults_4), denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_g_0,
            &self.lookup_data.range_check_20_h_0,
            self.lookup_data.mults_6,
            self.lookup_data.mults_7,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mults_6, mults_7)| {
                let denom0: PackedQM31 = range_check_20_g.combine(values0);
                let denom1: PackedQM31 = range_check_20_h.combine(values1);
                writer.write_frac(-(denom0 * mults_7 + denom1 * mults_6), denom0 * denom1);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
