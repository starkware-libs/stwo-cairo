// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::range_check_9_9::{Claim, InteractionClaim, LOG_SIZE, N_TRACE_COLUMNS};

use crate::witness::prelude::*;

pub type InputType = [M31; 2];
pub type PackedInputType = [PackedM31; 2];

pub struct ClaimGenerator {
    pub mults: [AtomicMultiplicityColumn; 8],
    input_to_row: HashMap<[M31; 2], usize>,
    preprocessed_trace: Arc<PreProcessedTrace>,
}

impl ClaimGenerator {
    pub fn new(preprocessed_trace: Arc<PreProcessedTrace>) -> Self {
        let mults = from_fn(|_| AtomicMultiplicityColumn::new(1 << LOG_SIZE));
        let column_ids = [
            PreProcessedColumnId {
                id: "range_check_9_9_column_0".to_owned(),
            },
            PreProcessedColumnId {
                id: "range_check_9_9_column_1".to_owned(),
            },
        ];

        Self {
            mults,
            input_to_row: make_input_to_row(&preprocessed_trace, column_ids),
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
            for input in packed_input.unpack() {
                self.add_input(&input, relation_index);
            }
        });
    }
    fn add_input(&self, input: &InputType, relation_index: usize) {
        self.mults[relation_index]
            .increase_at((*self.input_to_row.get(input).unwrap()).try_into().unwrap());
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

    let M31_1813904000 = PackedM31::broadcast(M31::from(1813904000));
    let M31_1830681619 = PackedM31::broadcast(M31::from(1830681619));
    let M31_1847459238 = PackedM31::broadcast(M31::from(1847459238));
    let M31_1864236857 = PackedM31::broadcast(M31::from(1864236857));
    let M31_1881014476 = PackedM31::broadcast(M31::from(1881014476));
    let M31_1897792095 = PackedM31::broadcast(M31::from(1897792095));
    let M31_2065568285 = PackedM31::broadcast(M31::from(2065568285));
    let M31_517791011 = PackedM31::broadcast(M31::from(517791011));
    let range_check_9_9_column_0 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "range_check_9_9_column_0".to_owned(),
    });
    let range_check_9_9_column_1 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "range_check_9_9_column_1".to_owned(),
    });

    (trace.par_iter_mut(), lookup_data.par_iter_mut())
        .into_par_iter()
        .enumerate()
        .for_each(|(row_index, (row, lookup_data))| {
            let range_check_9_9_column_0 = range_check_9_9_column_0.packed_at(row_index);
            let range_check_9_9_column_1 = range_check_9_9_column_1.packed_at(row_index);
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
            *lookup_data.range_check_9_9_0 = [
                M31_517791011,
                range_check_9_9_column_0,
                range_check_9_9_column_1,
            ];
            *lookup_data.range_check_9_9_b_1 = [
                M31_1897792095,
                range_check_9_9_column_0,
                range_check_9_9_column_1,
            ];
            *lookup_data.range_check_9_9_c_2 = [
                M31_1881014476,
                range_check_9_9_column_0,
                range_check_9_9_column_1,
            ];
            *lookup_data.range_check_9_9_d_3 = [
                M31_1864236857,
                range_check_9_9_column_0,
                range_check_9_9_column_1,
            ];
            *lookup_data.range_check_9_9_e_4 = [
                M31_1847459238,
                range_check_9_9_column_0,
                range_check_9_9_column_1,
            ];
            *lookup_data.range_check_9_9_f_5 = [
                M31_1830681619,
                range_check_9_9_column_0,
                range_check_9_9_column_1,
            ];
            *lookup_data.range_check_9_9_g_6 = [
                M31_1813904000,
                range_check_9_9_column_0,
                range_check_9_9_column_1,
            ];
            *lookup_data.range_check_9_9_h_7 = [
                M31_2065568285,
                range_check_9_9_column_0,
                range_check_9_9_column_1,
            ];
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
    range_check_9_9_0: Vec<[PackedM31; 3]>,
    range_check_9_9_b_1: Vec<[PackedM31; 3]>,
    range_check_9_9_c_2: Vec<[PackedM31; 3]>,
    range_check_9_9_d_3: Vec<[PackedM31; 3]>,
    range_check_9_9_e_4: Vec<[PackedM31; 3]>,
    range_check_9_9_f_5: Vec<[PackedM31; 3]>,
    range_check_9_9_g_6: Vec<[PackedM31; 3]>,
    range_check_9_9_h_7: Vec<[PackedM31; 3]>,
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
            &self.lookup_data.range_check_9_9_0,
            &self.lookup_data.range_check_9_9_b_1,
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
            &self.lookup_data.range_check_9_9_c_2,
            &self.lookup_data.range_check_9_9_d_3,
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
            &self.lookup_data.range_check_9_9_e_4,
            &self.lookup_data.range_check_9_9_f_5,
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
            &self.lookup_data.range_check_9_9_g_6,
            &self.lookup_data.range_check_9_9_h_7,
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
