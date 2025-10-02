// AIR version 98896da1
#![allow(unused_parens)]
use cairo_air::components::sha_256_round::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{
    sha_256_big_sigma_0, sha_256_big_sigma_1, sha_256_k_table, sha_256_schedule,
    verify_bitwise_and_16, verify_bitwise_not_16, verify_bitwise_xor_16,
};
use crate::witness::prelude::*;

pub type PackedInputType = (
    PackedM31,
    PackedM31,
    ([PackedUInt32; 8], [PackedUInt32; 16]),
);

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
        sha_256_big_sigma_0_state: &mut sha_256_big_sigma_0::ClaimGenerator,
        sha_256_big_sigma_1_state: &mut sha_256_big_sigma_1::ClaimGenerator,
        sha_256_k_table_state: &sha_256_k_table::ClaimGenerator,
        sha_256_schedule_state: &mut sha_256_schedule::ClaimGenerator,
        verify_bitwise_and_16_state: &verify_bitwise_and_16::ClaimGenerator,
        verify_bitwise_not_16_state: &verify_bitwise_not_16::ClaimGenerator,
        verify_bitwise_xor_16_state: &verify_bitwise_xor_16::ClaimGenerator,
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
            sha_256_big_sigma_0_state,
            sha_256_big_sigma_1_state,
            sha_256_k_table_state,
            sha_256_schedule_state,
            verify_bitwise_and_16_state,
            verify_bitwise_not_16_state,
            verify_bitwise_xor_16_state,
        );
        sub_component_inputs
            .sha_256_big_sigma_1
            .iter()
            .for_each(|inputs| {
                sha_256_big_sigma_1_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .sha_256_big_sigma_0
            .iter()
            .for_each(|inputs| {
                sha_256_big_sigma_0_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .verify_bitwise_and_16
            .iter()
            .for_each(|inputs| {
                verify_bitwise_and_16_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .verify_bitwise_not_16
            .iter()
            .for_each(|inputs| {
                verify_bitwise_not_16_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .verify_bitwise_xor_16
            .iter()
            .for_each(|inputs| {
                verify_bitwise_xor_16_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .sha_256_k_table
            .iter()
            .for_each(|inputs| {
                sha_256_k_table_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .sha_256_schedule
            .iter()
            .for_each(|inputs| {
                sha_256_schedule_state.add_packed_inputs(inputs);
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
    sha_256_big_sigma_1: [Vec<sha_256_big_sigma_1::PackedInputType>; 1],
    sha_256_big_sigma_0: [Vec<sha_256_big_sigma_0::PackedInputType>; 1],
    verify_bitwise_and_16: [Vec<verify_bitwise_and_16::PackedInputType>; 10],
    verify_bitwise_not_16: [Vec<verify_bitwise_not_16::PackedInputType>; 2],
    verify_bitwise_xor_16: [Vec<verify_bitwise_xor_16::PackedInputType>; 6],
    sha_256_k_table: [Vec<sha_256_k_table::PackedInputType>; 1],
    sha_256_schedule: [Vec<sha_256_schedule::PackedInputType>; 1],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    n_rows: usize,
    sha_256_big_sigma_0_state: &mut sha_256_big_sigma_0::ClaimGenerator,
    sha_256_big_sigma_1_state: &mut sha_256_big_sigma_1::ClaimGenerator,
    sha_256_k_table_state: &sha_256_k_table::ClaimGenerator,
    sha_256_schedule_state: &mut sha_256_schedule::ClaimGenerator,
    verify_bitwise_and_16_state: &verify_bitwise_and_16::ClaimGenerator,
    verify_bitwise_not_16_state: &verify_bitwise_not_16::ClaimGenerator,
    verify_bitwise_xor_16_state: &verify_bitwise_xor_16::ClaimGenerator,
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

    let M31_1 = PackedM31::broadcast(M31::from(1));
    let UInt32_0 = PackedUInt32::broadcast(UInt32::from(0));
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
            |(row_index, (mut row, lookup_data, sub_component_inputs, sha_256_round_input))| {
                let input_limb_0_col0 = sha_256_round_input.0;
                *row[0] = input_limb_0_col0;
                let input_limb_1_col1 = sha_256_round_input.1;
                *row[1] = input_limb_1_col1;
                let input_limb_2_col2 = sha_256_round_input.2 .0[0].low().as_m31();
                *row[2] = input_limb_2_col2;
                let input_limb_3_col3 = sha_256_round_input.2 .0[0].high().as_m31();
                *row[3] = input_limb_3_col3;
                let input_limb_4_col4 = sha_256_round_input.2 .0[1].low().as_m31();
                *row[4] = input_limb_4_col4;
                let input_limb_5_col5 = sha_256_round_input.2 .0[1].high().as_m31();
                *row[5] = input_limb_5_col5;
                let input_limb_6_col6 = sha_256_round_input.2 .0[2].low().as_m31();
                *row[6] = input_limb_6_col6;
                let input_limb_7_col7 = sha_256_round_input.2 .0[2].high().as_m31();
                *row[7] = input_limb_7_col7;
                let input_limb_8_col8 = sha_256_round_input.2 .0[3].low().as_m31();
                *row[8] = input_limb_8_col8;
                let input_limb_9_col9 = sha_256_round_input.2 .0[3].high().as_m31();
                *row[9] = input_limb_9_col9;
                let input_limb_10_col10 = sha_256_round_input.2 .0[4].low().as_m31();
                *row[10] = input_limb_10_col10;
                let input_limb_11_col11 = sha_256_round_input.2 .0[4].high().as_m31();
                *row[11] = input_limb_11_col11;
                let input_limb_12_col12 = sha_256_round_input.2 .0[5].low().as_m31();
                *row[12] = input_limb_12_col12;
                let input_limb_13_col13 = sha_256_round_input.2 .0[5].high().as_m31();
                *row[13] = input_limb_13_col13;
                let input_limb_14_col14 = sha_256_round_input.2 .0[6].low().as_m31();
                *row[14] = input_limb_14_col14;
                let input_limb_15_col15 = sha_256_round_input.2 .0[6].high().as_m31();
                *row[15] = input_limb_15_col15;
                let input_limb_16_col16 = sha_256_round_input.2 .0[7].low().as_m31();
                *row[16] = input_limb_16_col16;
                let input_limb_17_col17 = sha_256_round_input.2 .0[7].high().as_m31();
                *row[17] = input_limb_17_col17;
                let input_limb_18_col18 = sha_256_round_input.2 .1[0].low().as_m31();
                *row[18] = input_limb_18_col18;
                let input_limb_19_col19 = sha_256_round_input.2 .1[0].high().as_m31();
                *row[19] = input_limb_19_col19;
                let input_limb_20_col20 = sha_256_round_input.2 .1[1].low().as_m31();
                *row[20] = input_limb_20_col20;
                let input_limb_21_col21 = sha_256_round_input.2 .1[1].high().as_m31();
                *row[21] = input_limb_21_col21;
                let input_limb_22_col22 = sha_256_round_input.2 .1[2].low().as_m31();
                *row[22] = input_limb_22_col22;
                let input_limb_23_col23 = sha_256_round_input.2 .1[2].high().as_m31();
                *row[23] = input_limb_23_col23;
                let input_limb_24_col24 = sha_256_round_input.2 .1[3].low().as_m31();
                *row[24] = input_limb_24_col24;
                let input_limb_25_col25 = sha_256_round_input.2 .1[3].high().as_m31();
                *row[25] = input_limb_25_col25;
                let input_limb_26_col26 = sha_256_round_input.2 .1[4].low().as_m31();
                *row[26] = input_limb_26_col26;
                let input_limb_27_col27 = sha_256_round_input.2 .1[4].high().as_m31();
                *row[27] = input_limb_27_col27;
                let input_limb_28_col28 = sha_256_round_input.2 .1[5].low().as_m31();
                *row[28] = input_limb_28_col28;
                let input_limb_29_col29 = sha_256_round_input.2 .1[5].high().as_m31();
                *row[29] = input_limb_29_col29;
                let input_limb_30_col30 = sha_256_round_input.2 .1[6].low().as_m31();
                *row[30] = input_limb_30_col30;
                let input_limb_31_col31 = sha_256_round_input.2 .1[6].high().as_m31();
                *row[31] = input_limb_31_col31;
                let input_limb_32_col32 = sha_256_round_input.2 .1[7].low().as_m31();
                *row[32] = input_limb_32_col32;
                let input_limb_33_col33 = sha_256_round_input.2 .1[7].high().as_m31();
                *row[33] = input_limb_33_col33;
                let input_limb_34_col34 = sha_256_round_input.2 .1[8].low().as_m31();
                *row[34] = input_limb_34_col34;
                let input_limb_35_col35 = sha_256_round_input.2 .1[8].high().as_m31();
                *row[35] = input_limb_35_col35;
                let input_limb_36_col36 = sha_256_round_input.2 .1[9].low().as_m31();
                *row[36] = input_limb_36_col36;
                let input_limb_37_col37 = sha_256_round_input.2 .1[9].high().as_m31();
                *row[37] = input_limb_37_col37;
                let input_limb_38_col38 = sha_256_round_input.2 .1[10].low().as_m31();
                *row[38] = input_limb_38_col38;
                let input_limb_39_col39 = sha_256_round_input.2 .1[10].high().as_m31();
                *row[39] = input_limb_39_col39;
                let input_limb_40_col40 = sha_256_round_input.2 .1[11].low().as_m31();
                *row[40] = input_limb_40_col40;
                let input_limb_41_col41 = sha_256_round_input.2 .1[11].high().as_m31();
                *row[41] = input_limb_41_col41;
                let input_limb_42_col42 = sha_256_round_input.2 .1[12].low().as_m31();
                *row[42] = input_limb_42_col42;
                let input_limb_43_col43 = sha_256_round_input.2 .1[12].high().as_m31();
                *row[43] = input_limb_43_col43;
                let input_limb_44_col44 = sha_256_round_input.2 .1[13].low().as_m31();
                *row[44] = input_limb_44_col44;
                let input_limb_45_col45 = sha_256_round_input.2 .1[13].high().as_m31();
                *row[45] = input_limb_45_col45;
                let input_limb_46_col46 = sha_256_round_input.2 .1[14].low().as_m31();
                *row[46] = input_limb_46_col46;
                let input_limb_47_col47 = sha_256_round_input.2 .1[14].high().as_m31();
                *row[47] = input_limb_47_col47;
                let input_limb_48_col48 = sha_256_round_input.2 .1[15].low().as_m31();
                *row[48] = input_limb_48_col48;
                let input_limb_49_col49 = sha_256_round_input.2 .1[15].high().as_m31();
                *row[49] = input_limb_49_col49;
                *sub_component_inputs.sha_256_big_sigma_1[0] = sha_256_round_input.2 .0[4];
                let sha_256_big_sigma_1_output_tmp_ce7d8_0 =
                    PackedSha256BigSigma1::deduce_output(sha_256_round_input.2 .0[4]);
                let sha_256_big_sigma_1_output_limb_0_col50 =
                    sha_256_big_sigma_1_output_tmp_ce7d8_0.low().as_m31();
                *row[50] = sha_256_big_sigma_1_output_limb_0_col50;
                let sha_256_big_sigma_1_output_limb_1_col51 =
                    sha_256_big_sigma_1_output_tmp_ce7d8_0.high().as_m31();
                *row[51] = sha_256_big_sigma_1_output_limb_1_col51;
                *lookup_data.sha_256_big_sigma_1_0 = [
                    input_limb_10_col10,
                    input_limb_11_col11,
                    sha_256_big_sigma_1_output_limb_0_col50,
                    sha_256_big_sigma_1_output_limb_1_col51,
                ];
                *sub_component_inputs.sha_256_big_sigma_0[0] = sha_256_round_input.2 .0[0];
                let sha_256_big_sigma_0_output_tmp_ce7d8_1 =
                    PackedSha256BigSigma0::deduce_output(sha_256_round_input.2 .0[0]);
                let sha_256_big_sigma_0_output_limb_0_col52 =
                    sha_256_big_sigma_0_output_tmp_ce7d8_1.low().as_m31();
                *row[52] = sha_256_big_sigma_0_output_limb_0_col52;
                let sha_256_big_sigma_0_output_limb_1_col53 =
                    sha_256_big_sigma_0_output_tmp_ce7d8_1.high().as_m31();
                *row[53] = sha_256_big_sigma_0_output_limb_1_col53;
                *lookup_data.sha_256_big_sigma_0_0 = [
                    input_limb_2_col2,
                    input_limb_3_col3,
                    sha_256_big_sigma_0_output_limb_0_col52,
                    sha_256_big_sigma_0_output_limb_1_col53,
                ];

                // Bitwise And Num Bits 16.

                let and_tmp_ce7d8_2 = ((PackedUInt16::from_m31(input_limb_10_col10))
                    & (PackedUInt16::from_m31(input_limb_12_col12)));
                let and_col54 = and_tmp_ce7d8_2.as_m31();
                *row[54] = and_col54;
                *sub_component_inputs.verify_bitwise_and_16[0] =
                    [input_limb_10_col10, input_limb_12_col12, and_col54];
                *lookup_data.verify_bitwise_and_16_0 =
                    [input_limb_10_col10, input_limb_12_col12, and_col54];

                // Bitwise And Num Bits 16.

                let and_tmp_ce7d8_4 = ((PackedUInt16::from_m31(input_limb_11_col11))
                    & (PackedUInt16::from_m31(input_limb_13_col13)));
                let and_col55 = and_tmp_ce7d8_4.as_m31();
                *row[55] = and_col55;
                *sub_component_inputs.verify_bitwise_and_16[1] =
                    [input_limb_11_col11, input_limb_13_col13, and_col55];
                *lookup_data.verify_bitwise_and_16_1 =
                    [input_limb_11_col11, input_limb_13_col13, and_col55];

                // Bitwise Not Num Bits 16.

                let not_a_tmp_ce7d8_6 = !(PackedUInt16::from_m31(input_limb_10_col10));
                let not_a_col56 = not_a_tmp_ce7d8_6.as_m31();
                *row[56] = not_a_col56;
                *sub_component_inputs.verify_bitwise_not_16[0] = [input_limb_10_col10, not_a_col56];
                *lookup_data.verify_bitwise_not_16_0 = [input_limb_10_col10, not_a_col56];

                // Bitwise Not Num Bits 16.

                let not_a_tmp_ce7d8_8 = !(PackedUInt16::from_m31(input_limb_11_col11));
                let not_a_col57 = not_a_tmp_ce7d8_8.as_m31();
                *row[57] = not_a_col57;
                *sub_component_inputs.verify_bitwise_not_16[1] = [input_limb_11_col11, not_a_col57];
                *lookup_data.verify_bitwise_not_16_1 = [input_limb_11_col11, not_a_col57];

                // Bitwise And Num Bits 16.

                let and_tmp_ce7d8_10 = ((PackedUInt16::from_m31(not_a_col56))
                    & (PackedUInt16::from_m31(input_limb_14_col14)));
                let and_col58 = and_tmp_ce7d8_10.as_m31();
                *row[58] = and_col58;
                *sub_component_inputs.verify_bitwise_and_16[2] =
                    [not_a_col56, input_limb_14_col14, and_col58];
                *lookup_data.verify_bitwise_and_16_2 =
                    [not_a_col56, input_limb_14_col14, and_col58];

                // Bitwise And Num Bits 16.

                let and_tmp_ce7d8_12 = ((PackedUInt16::from_m31(not_a_col57))
                    & (PackedUInt16::from_m31(input_limb_15_col15)));
                let and_col59 = and_tmp_ce7d8_12.as_m31();
                *row[59] = and_col59;
                *sub_component_inputs.verify_bitwise_and_16[3] =
                    [not_a_col57, input_limb_15_col15, and_col59];
                *lookup_data.verify_bitwise_and_16_3 =
                    [not_a_col57, input_limb_15_col15, and_col59];

                // Bitwise Xor Num Bits 16.

                let xor_tmp_ce7d8_14 =
                    ((PackedUInt16::from_m31(and_col54)) ^ (PackedUInt16::from_m31(and_col58)));
                let xor_col60 = xor_tmp_ce7d8_14.as_m31();
                *row[60] = xor_col60;
                *sub_component_inputs.verify_bitwise_xor_16[0] = [and_col54, and_col58, xor_col60];
                *lookup_data.verify_bitwise_xor_16_0 = [and_col54, and_col58, xor_col60];

                // Bitwise Xor Num Bits 16.

                let xor_tmp_ce7d8_16 =
                    ((PackedUInt16::from_m31(and_col55)) ^ (PackedUInt16::from_m31(and_col59)));
                let xor_col61 = xor_tmp_ce7d8_16.as_m31();
                *row[61] = xor_col61;
                *sub_component_inputs.verify_bitwise_xor_16[1] = [and_col55, and_col59, xor_col61];
                *lookup_data.verify_bitwise_xor_16_1 = [and_col55, and_col59, xor_col61];

                let ch_tmp_ce7d8_18 = PackedUInt32::from_limbs([xor_col60, xor_col61]);
                let ch_limb_0_col62 = xor_col60;
                *row[62] = ch_limb_0_col62;
                let ch_limb_1_col63 = xor_col61;
                *row[63] = ch_limb_1_col63;
                *sub_component_inputs.sha_256_k_table[0] = [input_limb_1_col1];
                let sha_256_k_table_output_tmp_ce7d8_19 =
                    PackedSha256KTable::deduce_output([input_limb_1_col1]);
                let sha_256_k_table_output_limb_0_col64 =
                    sha_256_k_table_output_tmp_ce7d8_19.low().as_m31();
                *row[64] = sha_256_k_table_output_limb_0_col64;
                let sha_256_k_table_output_limb_1_col65 =
                    sha_256_k_table_output_tmp_ce7d8_19.high().as_m31();
                *row[65] = sha_256_k_table_output_limb_1_col65;
                *lookup_data.sha_256_k_table_0 = [
                    input_limb_1_col1,
                    sha_256_k_table_output_limb_0_col64,
                    sha_256_k_table_output_limb_1_col65,
                ];

                // Triple Sum 32.

                let triple_sum32_res_tmp_ce7d8_20 = (((sha_256_round_input.2 .0[7])
                    + (sha_256_big_sigma_1_output_tmp_ce7d8_0))
                    + (ch_tmp_ce7d8_18));
                let triple_sum32_res_limb_0_col66 = triple_sum32_res_tmp_ce7d8_20.low().as_m31();
                *row[66] = triple_sum32_res_limb_0_col66;
                let triple_sum32_res_limb_1_col67 = triple_sum32_res_tmp_ce7d8_20.high().as_m31();
                *row[67] = triple_sum32_res_limb_1_col67;
                let triple_sum_32_output_tmp_ce7d8_23 = triple_sum32_res_tmp_ce7d8_20;

                // Triple Sum 32.

                let triple_sum32_res_tmp_ce7d8_24 = (((triple_sum_32_output_tmp_ce7d8_23)
                    + (sha_256_k_table_output_tmp_ce7d8_19))
                    + (sha_256_round_input.2 .1[0]));
                let triple_sum32_res_limb_0_col68 = triple_sum32_res_tmp_ce7d8_24.low().as_m31();
                *row[68] = triple_sum32_res_limb_0_col68;
                let triple_sum32_res_limb_1_col69 = triple_sum32_res_tmp_ce7d8_24.high().as_m31();
                *row[69] = triple_sum32_res_limb_1_col69;
                let triple_sum_32_output_tmp_ce7d8_27 = triple_sum32_res_tmp_ce7d8_24;

                // Bitwise And Num Bits 16.

                let and_tmp_ce7d8_28 = ((PackedUInt16::from_m31(input_limb_2_col2))
                    & (PackedUInt16::from_m31(input_limb_4_col4)));
                let and_col70 = and_tmp_ce7d8_28.as_m31();
                *row[70] = and_col70;
                *sub_component_inputs.verify_bitwise_and_16[4] =
                    [input_limb_2_col2, input_limb_4_col4, and_col70];
                *lookup_data.verify_bitwise_and_16_4 =
                    [input_limb_2_col2, input_limb_4_col4, and_col70];

                // Bitwise And Num Bits 16.

                let and_tmp_ce7d8_30 = ((PackedUInt16::from_m31(input_limb_3_col3))
                    & (PackedUInt16::from_m31(input_limb_5_col5)));
                let and_col71 = and_tmp_ce7d8_30.as_m31();
                *row[71] = and_col71;
                *sub_component_inputs.verify_bitwise_and_16[5] =
                    [input_limb_3_col3, input_limb_5_col5, and_col71];
                *lookup_data.verify_bitwise_and_16_5 =
                    [input_limb_3_col3, input_limb_5_col5, and_col71];

                // Bitwise And Num Bits 16.

                let and_tmp_ce7d8_32 = ((PackedUInt16::from_m31(input_limb_2_col2))
                    & (PackedUInt16::from_m31(input_limb_6_col6)));
                let and_col72 = and_tmp_ce7d8_32.as_m31();
                *row[72] = and_col72;
                *sub_component_inputs.verify_bitwise_and_16[6] =
                    [input_limb_2_col2, input_limb_6_col6, and_col72];
                *lookup_data.verify_bitwise_and_16_6 =
                    [input_limb_2_col2, input_limb_6_col6, and_col72];

                // Bitwise And Num Bits 16.

                let and_tmp_ce7d8_34 = ((PackedUInt16::from_m31(input_limb_3_col3))
                    & (PackedUInt16::from_m31(input_limb_7_col7)));
                let and_col73 = and_tmp_ce7d8_34.as_m31();
                *row[73] = and_col73;
                *sub_component_inputs.verify_bitwise_and_16[7] =
                    [input_limb_3_col3, input_limb_7_col7, and_col73];
                *lookup_data.verify_bitwise_and_16_7 =
                    [input_limb_3_col3, input_limb_7_col7, and_col73];

                // Bitwise And Num Bits 16.

                let and_tmp_ce7d8_36 = ((PackedUInt16::from_m31(input_limb_4_col4))
                    & (PackedUInt16::from_m31(input_limb_6_col6)));
                let and_col74 = and_tmp_ce7d8_36.as_m31();
                *row[74] = and_col74;
                *sub_component_inputs.verify_bitwise_and_16[8] =
                    [input_limb_4_col4, input_limb_6_col6, and_col74];
                *lookup_data.verify_bitwise_and_16_8 =
                    [input_limb_4_col4, input_limb_6_col6, and_col74];

                // Bitwise And Num Bits 16.

                let and_tmp_ce7d8_38 = ((PackedUInt16::from_m31(input_limb_5_col5))
                    & (PackedUInt16::from_m31(input_limb_7_col7)));
                let and_col75 = and_tmp_ce7d8_38.as_m31();
                *row[75] = and_col75;
                *sub_component_inputs.verify_bitwise_and_16[9] =
                    [input_limb_5_col5, input_limb_7_col7, and_col75];
                *lookup_data.verify_bitwise_and_16_9 =
                    [input_limb_5_col5, input_limb_7_col7, and_col75];

                // Bitwise Xor Num Bits 16.

                let xor_tmp_ce7d8_40 =
                    ((PackedUInt16::from_m31(and_col70)) ^ (PackedUInt16::from_m31(and_col72)));
                let xor_col76 = xor_tmp_ce7d8_40.as_m31();
                *row[76] = xor_col76;
                *sub_component_inputs.verify_bitwise_xor_16[2] = [and_col70, and_col72, xor_col76];
                *lookup_data.verify_bitwise_xor_16_2 = [and_col70, and_col72, xor_col76];

                // Bitwise Xor Num Bits 16.

                let xor_tmp_ce7d8_42 =
                    ((PackedUInt16::from_m31(and_col71)) ^ (PackedUInt16::from_m31(and_col73)));
                let xor_col77 = xor_tmp_ce7d8_42.as_m31();
                *row[77] = xor_col77;
                *sub_component_inputs.verify_bitwise_xor_16[3] = [and_col71, and_col73, xor_col77];
                *lookup_data.verify_bitwise_xor_16_3 = [and_col71, and_col73, xor_col77];

                // Bitwise Xor Num Bits 16.

                let xor_tmp_ce7d8_44 =
                    ((PackedUInt16::from_m31(xor_col76)) ^ (PackedUInt16::from_m31(and_col74)));
                let xor_col78 = xor_tmp_ce7d8_44.as_m31();
                *row[78] = xor_col78;
                *sub_component_inputs.verify_bitwise_xor_16[4] = [xor_col76, and_col74, xor_col78];
                *lookup_data.verify_bitwise_xor_16_4 = [xor_col76, and_col74, xor_col78];

                // Bitwise Xor Num Bits 16.

                let xor_tmp_ce7d8_46 =
                    ((PackedUInt16::from_m31(xor_col77)) ^ (PackedUInt16::from_m31(and_col75)));
                let xor_col79 = xor_tmp_ce7d8_46.as_m31();
                *row[79] = xor_col79;
                *sub_component_inputs.verify_bitwise_xor_16[5] = [xor_col77, and_col75, xor_col79];
                *lookup_data.verify_bitwise_xor_16_5 = [xor_col77, and_col75, xor_col79];

                let maj_tmp_ce7d8_48 = PackedUInt32::from_limbs([xor_col78, xor_col79]);
                let maj_limb_0_col80 = xor_col78;
                *row[80] = maj_limb_0_col80;
                let maj_limb_1_col81 = xor_col79;
                *row[81] = maj_limb_1_col81;

                // Triple Sum 32.

                let triple_sum32_res_tmp_ce7d8_49 =
                    (((sha_256_big_sigma_0_output_tmp_ce7d8_1) + (maj_tmp_ce7d8_48)) + (UInt32_0));
                let triple_sum32_res_limb_0_col82 = triple_sum32_res_tmp_ce7d8_49.low().as_m31();
                *row[82] = triple_sum32_res_limb_0_col82;
                let triple_sum32_res_limb_1_col83 = triple_sum32_res_tmp_ce7d8_49.high().as_m31();
                *row[83] = triple_sum32_res_limb_1_col83;
                let triple_sum_32_output_tmp_ce7d8_52 = triple_sum32_res_tmp_ce7d8_49;

                *sub_component_inputs.sha_256_schedule[0] = [
                    sha_256_round_input.2 .1[0],
                    sha_256_round_input.2 .1[1],
                    sha_256_round_input.2 .1[2],
                    sha_256_round_input.2 .1[3],
                    sha_256_round_input.2 .1[4],
                    sha_256_round_input.2 .1[5],
                    sha_256_round_input.2 .1[6],
                    sha_256_round_input.2 .1[7],
                    sha_256_round_input.2 .1[8],
                    sha_256_round_input.2 .1[9],
                    sha_256_round_input.2 .1[10],
                    sha_256_round_input.2 .1[11],
                    sha_256_round_input.2 .1[12],
                    sha_256_round_input.2 .1[13],
                    sha_256_round_input.2 .1[14],
                    sha_256_round_input.2 .1[15],
                ];
                let sha_256_schedule_output_tmp_ce7d8_53 = PackedSha256Schedule::deduce_output([
                    sha_256_round_input.2 .1[0],
                    sha_256_round_input.2 .1[1],
                    sha_256_round_input.2 .1[2],
                    sha_256_round_input.2 .1[3],
                    sha_256_round_input.2 .1[4],
                    sha_256_round_input.2 .1[5],
                    sha_256_round_input.2 .1[6],
                    sha_256_round_input.2 .1[7],
                    sha_256_round_input.2 .1[8],
                    sha_256_round_input.2 .1[9],
                    sha_256_round_input.2 .1[10],
                    sha_256_round_input.2 .1[11],
                    sha_256_round_input.2 .1[12],
                    sha_256_round_input.2 .1[13],
                    sha_256_round_input.2 .1[14],
                    sha_256_round_input.2 .1[15],
                ]);
                let sha_256_schedule_output_limb_0_col84 =
                    sha_256_schedule_output_tmp_ce7d8_53.low().as_m31();
                *row[84] = sha_256_schedule_output_limb_0_col84;
                let sha_256_schedule_output_limb_1_col85 =
                    sha_256_schedule_output_tmp_ce7d8_53.high().as_m31();
                *row[85] = sha_256_schedule_output_limb_1_col85;
                *lookup_data.sha_256_schedule_0 = [
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
                    input_limb_32_col32,
                    input_limb_33_col33,
                    input_limb_34_col34,
                    input_limb_35_col35,
                    input_limb_36_col36,
                    input_limb_37_col37,
                    input_limb_38_col38,
                    input_limb_39_col39,
                    input_limb_40_col40,
                    input_limb_41_col41,
                    input_limb_42_col42,
                    input_limb_43_col43,
                    input_limb_44_col44,
                    input_limb_45_col45,
                    input_limb_46_col46,
                    input_limb_47_col47,
                    input_limb_48_col48,
                    input_limb_49_col49,
                    sha_256_schedule_output_limb_0_col84,
                    sha_256_schedule_output_limb_1_col85,
                ];

                // Triple Sum 32.

                let triple_sum32_res_tmp_ce7d8_54 = (((triple_sum_32_output_tmp_ce7d8_27)
                    + (sha_256_round_input.2 .0[3]))
                    + (UInt32_0));
                let triple_sum32_res_limb_0_col86 = triple_sum32_res_tmp_ce7d8_54.low().as_m31();
                *row[86] = triple_sum32_res_limb_0_col86;
                let triple_sum32_res_limb_1_col87 = triple_sum32_res_tmp_ce7d8_54.high().as_m31();
                *row[87] = triple_sum32_res_limb_1_col87;
                let triple_sum_32_output_tmp_ce7d8_57 = triple_sum32_res_tmp_ce7d8_54;

                // Triple Sum 32.

                let triple_sum32_res_tmp_ce7d8_58 = (((triple_sum_32_output_tmp_ce7d8_27)
                    + (triple_sum_32_output_tmp_ce7d8_52))
                    + (UInt32_0));
                let triple_sum32_res_limb_0_col88 = triple_sum32_res_tmp_ce7d8_58.low().as_m31();
                *row[88] = triple_sum32_res_limb_0_col88;
                let triple_sum32_res_limb_1_col89 = triple_sum32_res_tmp_ce7d8_58.high().as_m31();
                *row[89] = triple_sum32_res_limb_1_col89;
                let triple_sum_32_output_tmp_ce7d8_61 = triple_sum32_res_tmp_ce7d8_58;

                *lookup_data.sha_256_round_0 = [
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
                    input_limb_32_col32,
                    input_limb_33_col33,
                    input_limb_34_col34,
                    input_limb_35_col35,
                    input_limb_36_col36,
                    input_limb_37_col37,
                    input_limb_38_col38,
                    input_limb_39_col39,
                    input_limb_40_col40,
                    input_limb_41_col41,
                    input_limb_42_col42,
                    input_limb_43_col43,
                    input_limb_44_col44,
                    input_limb_45_col45,
                    input_limb_46_col46,
                    input_limb_47_col47,
                    input_limb_48_col48,
                    input_limb_49_col49,
                ];
                *lookup_data.sha_256_round_1 = [
                    input_limb_0_col0,
                    ((input_limb_1_col1) + (M31_1)),
                    triple_sum32_res_limb_0_col88,
                    triple_sum32_res_limb_1_col89,
                    input_limb_2_col2,
                    input_limb_3_col3,
                    input_limb_4_col4,
                    input_limb_5_col5,
                    input_limb_6_col6,
                    input_limb_7_col7,
                    triple_sum32_res_limb_0_col86,
                    triple_sum32_res_limb_1_col87,
                    input_limb_10_col10,
                    input_limb_11_col11,
                    input_limb_12_col12,
                    input_limb_13_col13,
                    input_limb_14_col14,
                    input_limb_15_col15,
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
                    input_limb_32_col32,
                    input_limb_33_col33,
                    input_limb_34_col34,
                    input_limb_35_col35,
                    input_limb_36_col36,
                    input_limb_37_col37,
                    input_limb_38_col38,
                    input_limb_39_col39,
                    input_limb_40_col40,
                    input_limb_41_col41,
                    input_limb_42_col42,
                    input_limb_43_col43,
                    input_limb_44_col44,
                    input_limb_45_col45,
                    input_limb_46_col46,
                    input_limb_47_col47,
                    input_limb_48_col48,
                    input_limb_49_col49,
                    sha_256_schedule_output_limb_0_col84,
                    sha_256_schedule_output_limb_1_col85,
                ];
                *row[90] = enabler_col.packed_at(row_index);
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    sha_256_big_sigma_0_0: Vec<[PackedM31; 4]>,
    sha_256_big_sigma_1_0: Vec<[PackedM31; 4]>,
    sha_256_k_table_0: Vec<[PackedM31; 3]>,
    sha_256_round_0: Vec<[PackedM31; 50]>,
    sha_256_round_1: Vec<[PackedM31; 50]>,
    sha_256_schedule_0: Vec<[PackedM31; 34]>,
    verify_bitwise_and_16_0: Vec<[PackedM31; 3]>,
    verify_bitwise_and_16_1: Vec<[PackedM31; 3]>,
    verify_bitwise_and_16_2: Vec<[PackedM31; 3]>,
    verify_bitwise_and_16_3: Vec<[PackedM31; 3]>,
    verify_bitwise_and_16_4: Vec<[PackedM31; 3]>,
    verify_bitwise_and_16_5: Vec<[PackedM31; 3]>,
    verify_bitwise_and_16_6: Vec<[PackedM31; 3]>,
    verify_bitwise_and_16_7: Vec<[PackedM31; 3]>,
    verify_bitwise_and_16_8: Vec<[PackedM31; 3]>,
    verify_bitwise_and_16_9: Vec<[PackedM31; 3]>,
    verify_bitwise_not_16_0: Vec<[PackedM31; 2]>,
    verify_bitwise_not_16_1: Vec<[PackedM31; 2]>,
    verify_bitwise_xor_16_0: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_16_1: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_16_2: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_16_3: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_16_4: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_16_5: Vec<[PackedM31; 3]>,
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
        sha_256_big_sigma_1: &relations::Sha256BigSigma1,
        sha_256_big_sigma_0: &relations::Sha256BigSigma0,
        verify_bitwise_and_16: &relations::VerifyBitwiseAnd_16,
        verify_bitwise_not_16: &relations::VerifyBitwiseNot_16,
        verify_bitwise_xor_16: &relations::VerifyBitwiseXor_16,
        sha_256_k_table: &relations::Sha256KTable,
        sha_256_schedule: &relations::Sha256Schedule,
        sha_256_round: &relations::Sha256Round,
    ) -> InteractionClaim {
        let enabler_col = Enabler::new(self.n_rows);
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.sha_256_big_sigma_1_0,
            &self.lookup_data.sha_256_big_sigma_0_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = sha_256_big_sigma_1.combine(values0);
                let denom1: PackedQM31 = sha_256_big_sigma_0.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_and_16_0,
            &self.lookup_data.verify_bitwise_and_16_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_and_16.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_and_16.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_not_16_0,
            &self.lookup_data.verify_bitwise_not_16_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_not_16.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_not_16.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_and_16_2,
            &self.lookup_data.verify_bitwise_and_16_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_and_16.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_and_16.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_xor_16_0,
            &self.lookup_data.verify_bitwise_xor_16_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_xor_16.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_xor_16.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.sha_256_k_table_0,
            &self.lookup_data.verify_bitwise_and_16_4,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = sha_256_k_table.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_and_16.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_and_16_5,
            &self.lookup_data.verify_bitwise_and_16_6,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_and_16.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_and_16.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_and_16_7,
            &self.lookup_data.verify_bitwise_and_16_8,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_and_16.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_and_16.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_and_16_9,
            &self.lookup_data.verify_bitwise_xor_16_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_and_16.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_xor_16.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_xor_16_3,
            &self.lookup_data.verify_bitwise_xor_16_4,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_xor_16.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_xor_16.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_xor_16_5,
            &self.lookup_data.sha_256_schedule_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_xor_16.combine(values0);
                let denom1: PackedQM31 = sha_256_schedule.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.sha_256_round_0,
            &self.lookup_data.sha_256_round_1,
        )
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (writer, values0, values1))| {
                let denom0: PackedQM31 = sha_256_round.combine(values0);
                let denom1: PackedQM31 = sha_256_round.combine(values1);
                writer.write_frac(
                    denom1 * enabler_col.packed_at(i) - denom0 * enabler_col.packed_at(i),
                    denom0 * denom1,
                );
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
