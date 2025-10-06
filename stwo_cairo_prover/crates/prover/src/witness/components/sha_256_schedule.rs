// AIR version 98896da1-dirty
#![allow(unused_parens)]
use cairo_air::components::sha_256_schedule::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{range_check_2, sha_256_small_sigma_0, sha_256_small_sigma_1};
use crate::witness::prelude::*;

pub type PackedInputType = [PackedUInt32; 16];

#[derive(Default)]
pub struct ClaimGenerator {
    pub packed_inputs: Vec<PackedInputType>,
}
impl ClaimGenerator {
    pub fn new() -> Self {
        Self {
            packed_inputs: vec![],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.packed_inputs.is_empty()
    }

    pub fn write_trace(
        mut self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        range_check_2_state: &range_check_2::ClaimGenerator,
        sha_256_small_sigma_0_state: &mut sha_256_small_sigma_0::ClaimGenerator,
        sha_256_small_sigma_1_state: &mut sha_256_small_sigma_1::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator) {
        assert!(!self.packed_inputs.is_empty());
        let n_vec_rows = self.packed_inputs.len();
        let n_rows = n_vec_rows * N_LANES;
        let packed_size = n_vec_rows.next_power_of_two();
        let log_size = packed_size.ilog2() + LOG_N_LANES;
        self.packed_inputs
            .resize(packed_size, *self.packed_inputs.first().unwrap());

        let (trace, lookup_data, sub_component_inputs) = write_trace_simd(
            self.packed_inputs,
            n_rows,
            range_check_2_state,
            sha_256_small_sigma_0_state,
            sha_256_small_sigma_1_state,
        );
        sub_component_inputs
            .sha_256_small_sigma_0
            .iter()
            .for_each(|inputs| {
                sha_256_small_sigma_0_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .sha_256_small_sigma_1
            .iter()
            .for_each(|inputs| {
                sha_256_small_sigma_1_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_2
            .iter()
            .for_each(|inputs| {
                range_check_2_state.add_packed_inputs(inputs);
            });
        tree_builder.extend_evals(trace.to_evals());

        (
            Claim { log_size },
            InteractionClaimGenerator {
                n_rows,
                log_size,
                lookup_data,
            },
        )
    }

    pub fn add_packed_inputs(&mut self, inputs: &[PackedInputType]) {
        self.packed_inputs.extend(inputs);
    }
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct SubComponentInputs {
    sha_256_small_sigma_0: [Vec<sha_256_small_sigma_0::PackedInputType>; 1],
    sha_256_small_sigma_1: [Vec<sha_256_small_sigma_1::PackedInputType>; 1],
    range_check_2: [Vec<range_check_2::PackedInputType>; 2],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    n_rows: usize,
    range_check_2_state: &range_check_2::ClaimGenerator,
    sha_256_small_sigma_0_state: &mut sha_256_small_sigma_0::ClaimGenerator,
    sha_256_small_sigma_1_state: &mut sha_256_small_sigma_1::ClaimGenerator,
) -> (
    ComponentTrace<N_TRACE_COLUMNS>,
    LookupData,
    SubComponentInputs,
) {
    let log_n_packed_rows = inputs.len().ilog2();
    let log_size = log_n_packed_rows + LOG_N_LANES;
    let (mut trace, mut lookup_data, mut sub_component_inputs) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
            SubComponentInputs::uninitialized(log_n_packed_rows),
        )
    };

    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let enabler_col = Enabler::new(n_rows);

    (
        trace.par_iter_mut(),
        lookup_data.par_iter_mut(),
        sub_component_inputs.par_iter_mut(),
        inputs.into_par_iter(),
    )
        .into_par_iter()
        .enumerate()
        .for_each(
            |(row_index, (mut row, lookup_data, sub_component_inputs, sha_256_schedule_input))| {
                let input_limb_0_col0 = sha_256_schedule_input[0].low().as_m31();
                *row[0] = input_limb_0_col0;
                let input_limb_1_col1 = sha_256_schedule_input[0].high().as_m31();
                *row[1] = input_limb_1_col1;
                let input_limb_2_col2 = sha_256_schedule_input[1].low().as_m31();
                *row[2] = input_limb_2_col2;
                let input_limb_3_col3 = sha_256_schedule_input[1].high().as_m31();
                *row[3] = input_limb_3_col3;
                let input_limb_4_col4 = sha_256_schedule_input[2].low().as_m31();
                *row[4] = input_limb_4_col4;
                let input_limb_5_col5 = sha_256_schedule_input[2].high().as_m31();
                *row[5] = input_limb_5_col5;
                let input_limb_6_col6 = sha_256_schedule_input[3].low().as_m31();
                *row[6] = input_limb_6_col6;
                let input_limb_7_col7 = sha_256_schedule_input[3].high().as_m31();
                *row[7] = input_limb_7_col7;
                let input_limb_8_col8 = sha_256_schedule_input[4].low().as_m31();
                *row[8] = input_limb_8_col8;
                let input_limb_9_col9 = sha_256_schedule_input[4].high().as_m31();
                *row[9] = input_limb_9_col9;
                let input_limb_10_col10 = sha_256_schedule_input[5].low().as_m31();
                *row[10] = input_limb_10_col10;
                let input_limb_11_col11 = sha_256_schedule_input[5].high().as_m31();
                *row[11] = input_limb_11_col11;
                let input_limb_12_col12 = sha_256_schedule_input[6].low().as_m31();
                *row[12] = input_limb_12_col12;
                let input_limb_13_col13 = sha_256_schedule_input[6].high().as_m31();
                *row[13] = input_limb_13_col13;
                let input_limb_14_col14 = sha_256_schedule_input[7].low().as_m31();
                *row[14] = input_limb_14_col14;
                let input_limb_15_col15 = sha_256_schedule_input[7].high().as_m31();
                *row[15] = input_limb_15_col15;
                let input_limb_16_col16 = sha_256_schedule_input[8].low().as_m31();
                *row[16] = input_limb_16_col16;
                let input_limb_17_col17 = sha_256_schedule_input[8].high().as_m31();
                *row[17] = input_limb_17_col17;
                let input_limb_18_col18 = sha_256_schedule_input[9].low().as_m31();
                *row[18] = input_limb_18_col18;
                let input_limb_19_col19 = sha_256_schedule_input[9].high().as_m31();
                *row[19] = input_limb_19_col19;
                let input_limb_20_col20 = sha_256_schedule_input[10].low().as_m31();
                *row[20] = input_limb_20_col20;
                let input_limb_21_col21 = sha_256_schedule_input[10].high().as_m31();
                *row[21] = input_limb_21_col21;
                let input_limb_22_col22 = sha_256_schedule_input[11].low().as_m31();
                *row[22] = input_limb_22_col22;
                let input_limb_23_col23 = sha_256_schedule_input[11].high().as_m31();
                *row[23] = input_limb_23_col23;
                let input_limb_24_col24 = sha_256_schedule_input[12].low().as_m31();
                *row[24] = input_limb_24_col24;
                let input_limb_25_col25 = sha_256_schedule_input[12].high().as_m31();
                *row[25] = input_limb_25_col25;
                let input_limb_26_col26 = sha_256_schedule_input[13].low().as_m31();
                *row[26] = input_limb_26_col26;
                let input_limb_27_col27 = sha_256_schedule_input[13].high().as_m31();
                *row[27] = input_limb_27_col27;
                let input_limb_28_col28 = sha_256_schedule_input[14].low().as_m31();
                *row[28] = input_limb_28_col28;
                let input_limb_29_col29 = sha_256_schedule_input[14].high().as_m31();
                *row[29] = input_limb_29_col29;
                let input_limb_30_col30 = sha_256_schedule_input[15].low().as_m31();
                *row[30] = input_limb_30_col30;
                let input_limb_31_col31 = sha_256_schedule_input[15].high().as_m31();
                *row[31] = input_limb_31_col31;
                *sub_component_inputs.sha_256_small_sigma_0[0] = sha_256_schedule_input[1];
                let sha_256_small_sigma_0_output_tmp_3b5f1_0 =
                    PackedSha256SmallSigma0::deduce_output(sha_256_schedule_input[1]);
                let sha_256_small_sigma_0_output_limb_0_col32 =
                    sha_256_small_sigma_0_output_tmp_3b5f1_0.low().as_m31();
                *row[32] = sha_256_small_sigma_0_output_limb_0_col32;
                let sha_256_small_sigma_0_output_limb_1_col33 =
                    sha_256_small_sigma_0_output_tmp_3b5f1_0.high().as_m31();
                *row[33] = sha_256_small_sigma_0_output_limb_1_col33;
                *lookup_data.sha_256_small_sigma_0_0 = [
                    input_limb_2_col2,
                    input_limb_3_col3,
                    sha_256_small_sigma_0_output_limb_0_col32,
                    sha_256_small_sigma_0_output_limb_1_col33,
                ];
                *sub_component_inputs.sha_256_small_sigma_1[0] = sha_256_schedule_input[14];
                let sha_256_small_sigma_1_output_tmp_3b5f1_1 =
                    PackedSha256SmallSigma1::deduce_output(sha_256_schedule_input[14]);
                let sha_256_small_sigma_1_output_limb_0_col34 =
                    sha_256_small_sigma_1_output_tmp_3b5f1_1.low().as_m31();
                *row[34] = sha_256_small_sigma_1_output_limb_0_col34;
                let sha_256_small_sigma_1_output_limb_1_col35 =
                    sha_256_small_sigma_1_output_tmp_3b5f1_1.high().as_m31();
                *row[35] = sha_256_small_sigma_1_output_limb_1_col35;
                *lookup_data.sha_256_small_sigma_1_0 = [
                    input_limb_28_col28,
                    input_limb_29_col29,
                    sha_256_small_sigma_1_output_limb_0_col34,
                    sha_256_small_sigma_1_output_limb_1_col35,
                ];
                let w_i_tmp_3b5f1_2 = ((((sha_256_schedule_input[0])
                    + (sha_256_small_sigma_0_output_tmp_3b5f1_0))
                    + (sha_256_schedule_input[9]))
                    + (sha_256_small_sigma_1_output_tmp_3b5f1_1));
                let w_i_limb_0_col36 = w_i_tmp_3b5f1_2.low().as_m31();
                *row[36] = w_i_limb_0_col36;
                let w_i_limb_1_col37 = w_i_tmp_3b5f1_2.high().as_m31();
                *row[37] = w_i_limb_1_col37;
                let carry_low_tmp_3b5f1_3 = ((((((input_limb_0_col0)
                    + (sha_256_small_sigma_0_output_limb_0_col32))
                    + (input_limb_18_col18))
                    + (sha_256_small_sigma_1_output_limb_0_col34))
                    - (w_i_limb_0_col36))
                    * (M31_32768));
                *sub_component_inputs.range_check_2[0] = [carry_low_tmp_3b5f1_3];
                *lookup_data.range_check_2_0 = [carry_low_tmp_3b5f1_3];
                let carry_high_tmp_3b5f1_4 = (((((((input_limb_1_col1)
                    + (sha_256_small_sigma_0_output_limb_1_col33))
                    + (input_limb_19_col19))
                    + (sha_256_small_sigma_1_output_limb_1_col35))
                    + (carry_low_tmp_3b5f1_3))
                    - (w_i_limb_1_col37))
                    * (M31_32768));
                *sub_component_inputs.range_check_2[1] = [carry_high_tmp_3b5f1_4];
                *lookup_data.range_check_2_1 = [carry_high_tmp_3b5f1_4];
                *lookup_data.sha_256_schedule_0 = [
                    input_limb_0_col0,
                    input_limb_1_col1,
                    input_limb_2_col2,
                    input_limb_3_col3,
                    input_limb_4_col4,
                    input_limb_5_col5,
                    input_limb_6_col6,
                    input_limb_7_col7,
                    input_limb_8_col8,
                    input_limb_9_col9,
                    input_limb_10_col10,
                    input_limb_11_col11,
                    input_limb_12_col12,
                    input_limb_13_col13,
                    input_limb_14_col14,
                    input_limb_15_col15,
                    input_limb_16_col16,
                    input_limb_17_col17,
                    input_limb_18_col18,
                    input_limb_19_col19,
                    input_limb_20_col20,
                    input_limb_21_col21,
                    input_limb_22_col22,
                    input_limb_23_col23,
                    input_limb_24_col24,
                    input_limb_25_col25,
                    input_limb_26_col26,
                    input_limb_27_col27,
                    input_limb_28_col28,
                    input_limb_29_col29,
                    input_limb_30_col30,
                    input_limb_31_col31,
                    w_i_limb_0_col36,
                    w_i_limb_1_col37,
                ];
                *row[38] = enabler_col.packed_at(row_index);
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    range_check_2_0: Vec<[PackedM31; 1]>,
    range_check_2_1: Vec<[PackedM31; 1]>,
    sha_256_schedule_0: Vec<[PackedM31; 34]>,
    sha_256_small_sigma_0_0: Vec<[PackedM31; 4]>,
    sha_256_small_sigma_1_0: Vec<[PackedM31; 4]>,
}

pub struct InteractionClaimGenerator {
    n_rows: usize,
    log_size: u32,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        sha_256_small_sigma_0: &relations::Sha256SmallSigma0,
        sha_256_small_sigma_1: &relations::Sha256SmallSigma1,
        range_check_2: &relations::RangeCheck_2,
        sha_256_schedule: &relations::Sha256Schedule,
    ) -> InteractionClaim {
        let enabler_col = Enabler::new(self.n_rows);
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.sha_256_small_sigma_0_0,
            &self.lookup_data.sha_256_small_sigma_1_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = sha_256_small_sigma_0.combine(values0);
                let denom1: PackedQM31 = sha_256_small_sigma_1.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_2_0,
            &self.lookup_data.range_check_2_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_2.combine(values0);
                let denom1: PackedQM31 = range_check_2.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(), &self.lookup_data.sha_256_schedule_0)
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (writer, values))| {
                let denom = sha_256_schedule.combine(values);
                writer.write_frac(-PackedQM31::one() * enabler_col.packed_at(i), denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
