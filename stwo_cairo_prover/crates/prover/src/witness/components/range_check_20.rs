// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::range_check_20::{Claim, InteractionClaim, LOG_SIZE, N_TRACE_COLUMNS};

use crate::witness::prelude::*;

pub type InputType = [M31; 1];
pub type PackedInputType = [PackedM31; 1];

pub struct ClaimGenerator {
    pub mults: [AtomicMultiplicityColumn; 8],
    preprocessed_trace: Arc<PreProcessedTrace>,
}

impl ClaimGenerator {
    pub fn new(preprocessed_trace: Arc<PreProcessedTrace>) -> Self {
        let mults = from_fn(|_| AtomicMultiplicityColumn::new(1 << LOG_SIZE));
        Self {
            mults,
            preprocessed_trace,
        }
    }

    pub fn write_trace(
        self,
    ) -> (
        ComponentTrace<N_TRACE_COLUMNS>,
        Claim,
        InteractionClaimGenerator,
    ) {
        let mults = self
            .mults
            .into_iter()
            .map(|v| v.into_simd_vec())
            .collect::<Vec<_>>();

        let (trace, lookup_data) = write_trace_simd(&self.preprocessed_trace, mults);

        (trace, Claim {}, InteractionClaimGenerator { lookup_data })
    }
}

impl AddInputs for ClaimGenerator {
    type PackedInputType = PackedInputType;
    type InputType = InputType;

    fn add_packed_inputs(&self, packed_inputs: &[PackedInputType], relation_index: usize) {
        packed_inputs.into_par_iter().for_each(|packed_input| {
            for [idx] in packed_input.unpack() {
                self.mults[relation_index].increase_at(idx.0);
            }
        });
    }
    fn add_input(&self, input: &InputType, relation_index: usize) {
        self.mults[relation_index].increase_at(input[0].0);
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

    let M31_1410849886 = PackedM31::broadcast(M31::from(1410849886));
    let M31_447122465 = PackedM31::broadcast(M31::from(447122465));
    let M31_463900084 = PackedM31::broadcast(M31::from(463900084));
    let M31_480677703 = PackedM31::broadcast(M31::from(480677703));
    let M31_497455322 = PackedM31::broadcast(M31::from(497455322));
    let M31_514232941 = PackedM31::broadcast(M31::from(514232941));
    let M31_531010560 = PackedM31::broadcast(M31::from(531010560));
    let M31_682009131 = PackedM31::broadcast(M31::from(682009131));
    let seq_20 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "seq_20".to_owned(),
    });

    (trace.par_iter_mut(), lookup_data.par_iter_mut())
        .into_par_iter()
        .enumerate()
        .for_each(|(row_index, (row, lookup_data))| {
            let seq_20 = seq_20.packed_at(row_index);
            let multiplicity_0_col0 = *mults[0].get(row_index).unwrap_or(&PackedM31::zero());
            *row[0] = multiplicity_0_col0;
            let multiplicity_1_col1 = *mults[1].get(row_index).unwrap_or(&PackedM31::zero());
            *row[1] = multiplicity_1_col1;
            let multiplicity_2_col2 = *mults[2].get(row_index).unwrap_or(&PackedM31::zero());
            *row[2] = multiplicity_2_col2;
            let multiplicity_3_col3 = *mults[3].get(row_index).unwrap_or(&PackedM31::zero());
            *row[3] = multiplicity_3_col3;
            let multiplicity_4_col4 = *mults[4].get(row_index).unwrap_or(&PackedM31::zero());
            *row[4] = multiplicity_4_col4;
            let multiplicity_5_col5 = *mults[5].get(row_index).unwrap_or(&PackedM31::zero());
            *row[5] = multiplicity_5_col5;
            let multiplicity_6_col6 = *mults[6].get(row_index).unwrap_or(&PackedM31::zero());
            *row[6] = multiplicity_6_col6;
            let multiplicity_7_col7 = *mults[7].get(row_index).unwrap_or(&PackedM31::zero());
            *row[7] = multiplicity_7_col7;
            *lookup_data.range_check_20_0 = [M31_1410849886, seq_20];
            *lookup_data.range_check_20_b_1 = [M31_514232941, seq_20];
            *lookup_data.range_check_20_c_2 = [M31_531010560, seq_20];
            *lookup_data.range_check_20_d_3 = [M31_480677703, seq_20];
            *lookup_data.range_check_20_e_4 = [M31_497455322, seq_20];
            *lookup_data.range_check_20_f_5 = [M31_447122465, seq_20];
            *lookup_data.range_check_20_g_6 = [M31_463900084, seq_20];
            *lookup_data.range_check_20_h_7 = [M31_682009131, seq_20];
            *lookup_data.mults_0 = multiplicity_0_col0;
            *lookup_data.mults_1 = multiplicity_1_col1;
            *lookup_data.mults_2 = multiplicity_2_col2;
            *lookup_data.mults_3 = multiplicity_3_col3;
            *lookup_data.mults_4 = multiplicity_4_col4;
            *lookup_data.mults_5 = multiplicity_5_col5;
            *lookup_data.mults_6 = multiplicity_6_col6;
            *lookup_data.mults_7 = multiplicity_7_col7;
        });

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    range_check_20_0: Vec<[PackedM31; 2]>,
    range_check_20_b_1: Vec<[PackedM31; 2]>,
    range_check_20_c_2: Vec<[PackedM31; 2]>,
    range_check_20_d_3: Vec<[PackedM31; 2]>,
    range_check_20_e_4: Vec<[PackedM31; 2]>,
    range_check_20_f_5: Vec<[PackedM31; 2]>,
    range_check_20_g_6: Vec<[PackedM31; 2]>,
    range_check_20_h_7: Vec<[PackedM31; 2]>,
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
        common_lookup_elements: &relations::CommonLookupElements,
    ) -> (
        Vec<CircleEvaluation<SimdBackend, M31, BitReversedOrder>>,
        InteractionClaim,
    ) {
        let mut logup_gen = unsafe { LogupTraceGenerator::uninitialized(LOG_SIZE) };

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_0,
            &self.lookup_data.range_check_20_b_1,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(-(denom0 * *mult1 + denom1 * *mult0), denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_c_2,
            &self.lookup_data.range_check_20_d_3,
            &self.lookup_data.mults_2,
            &self.lookup_data.mults_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(-(denom0 * *mult1 + denom1 * *mult0), denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_e_4,
            &self.lookup_data.range_check_20_f_5,
            &self.lookup_data.mults_4,
            &self.lookup_data.mults_5,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(-(denom0 * *mult1 + denom1 * *mult0), denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_g_6,
            &self.lookup_data.range_check_20_h_7,
            &self.lookup_data.mults_6,
            &self.lookup_data.mults_7,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(-(denom0 * *mult1 + denom1 * *mult0), denom0 * denom1);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();

        (trace, InteractionClaim { claimed_sum })
    }
}
