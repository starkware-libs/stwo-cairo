// AIR version 98896da1-dirty
#![allow(unused_parens)]
use cairo_air::components::sha_256_round::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{
    sha_256_big_sigma_0, sha_256_big_sigma_1, sha_256_k_table, sha_256_schedule,
    verify_bitwise_and_8, verify_bitwise_not_16, verify_bitwise_xor_8,
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
        verify_bitwise_and_8_state: &verify_bitwise_and_8::ClaimGenerator,
        verify_bitwise_not_16_state: &verify_bitwise_not_16::ClaimGenerator,
        verify_bitwise_xor_8_state: &verify_bitwise_xor_8::ClaimGenerator,
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
            verify_bitwise_and_8_state,
            verify_bitwise_not_16_state,
            verify_bitwise_xor_8_state,
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
            .verify_bitwise_and_8
            .iter()
            .for_each(|inputs| {
                verify_bitwise_and_8_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .verify_bitwise_not_16
            .iter()
            .for_each(|inputs| {
                verify_bitwise_not_16_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .verify_bitwise_xor_8
            .iter()
            .for_each(|inputs| {
                verify_bitwise_xor_8_state.add_packed_inputs(inputs);
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
    verify_bitwise_and_8: [Vec<verify_bitwise_and_8::PackedInputType>; 20],
    verify_bitwise_not_16: [Vec<verify_bitwise_not_16::PackedInputType>; 2],
    verify_bitwise_xor_8: [Vec<verify_bitwise_xor_8::PackedInputType>; 12],
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
    verify_bitwise_and_8_state: &verify_bitwise_and_8::ClaimGenerator,
    verify_bitwise_not_16_state: &verify_bitwise_not_16::ClaimGenerator,
    verify_bitwise_xor_8_state: &verify_bitwise_xor_8::ClaimGenerator,
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
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let UInt16_8 = PackedUInt16::broadcast(UInt16::from(8));
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

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_ce7d8_2 = ((sha_256_round_input.2 .0[4].low()) >> (UInt16_8));
                let ms_8_bits_col54 = ms_8_bits_tmp_ce7d8_2.as_m31();
                *row[54] = ms_8_bits_col54;
                let split_16_low_part_size_8_output_tmp_ce7d8_3 = [
                    ((input_limb_10_col10) - ((ms_8_bits_col54) * (M31_256))),
                    ms_8_bits_col54,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_ce7d8_4 = ((sha_256_round_input.2 .0[4].high()) >> (UInt16_8));
                let ms_8_bits_col55 = ms_8_bits_tmp_ce7d8_4.as_m31();
                *row[55] = ms_8_bits_col55;
                let split_16_low_part_size_8_output_tmp_ce7d8_5 = [
                    ((input_limb_11_col11) - ((ms_8_bits_col55) * (M31_256))),
                    ms_8_bits_col55,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_ce7d8_6 = ((sha_256_round_input.2 .0[5].low()) >> (UInt16_8));
                let ms_8_bits_col56 = ms_8_bits_tmp_ce7d8_6.as_m31();
                *row[56] = ms_8_bits_col56;
                let split_16_low_part_size_8_output_tmp_ce7d8_7 = [
                    ((input_limb_12_col12) - ((ms_8_bits_col56) * (M31_256))),
                    ms_8_bits_col56,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_ce7d8_8 = ((sha_256_round_input.2 .0[5].high()) >> (UInt16_8));
                let ms_8_bits_col57 = ms_8_bits_tmp_ce7d8_8.as_m31();
                *row[57] = ms_8_bits_col57;
                let split_16_low_part_size_8_output_tmp_ce7d8_9 = [
                    ((input_limb_13_col13) - ((ms_8_bits_col57) * (M31_256))),
                    ms_8_bits_col57,
                ];

                // Bitwise And Num Bits 8.

                let and_tmp_ce7d8_10 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_ce7d8_3[0]))
                        & (PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_ce7d8_7[0])));
                let and_col58 = and_tmp_ce7d8_10.as_m31();
                *row[58] = and_col58;
                *sub_component_inputs.verify_bitwise_and_8[0] = [
                    split_16_low_part_size_8_output_tmp_ce7d8_3[0],
                    split_16_low_part_size_8_output_tmp_ce7d8_7[0],
                    and_col58,
                ];
                *lookup_data.verify_bitwise_and_8_0 = [
                    split_16_low_part_size_8_output_tmp_ce7d8_3[0],
                    split_16_low_part_size_8_output_tmp_ce7d8_7[0],
                    and_col58,
                ];

                // Bitwise And Num Bits 8.

                let and_tmp_ce7d8_12 = ((PackedUInt16::from_m31(ms_8_bits_col54))
                    & (PackedUInt16::from_m31(ms_8_bits_col56)));
                let and_col59 = and_tmp_ce7d8_12.as_m31();
                *row[59] = and_col59;
                *sub_component_inputs.verify_bitwise_and_8[1] =
                    [ms_8_bits_col54, ms_8_bits_col56, and_col59];
                *lookup_data.verify_bitwise_and_8_1 = [ms_8_bits_col54, ms_8_bits_col56, and_col59];

                // Bitwise And Num Bits 8.

                let and_tmp_ce7d8_14 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_ce7d8_5[0]))
                        & (PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_ce7d8_9[0])));
                let and_col60 = and_tmp_ce7d8_14.as_m31();
                *row[60] = and_col60;
                *sub_component_inputs.verify_bitwise_and_8[2] = [
                    split_16_low_part_size_8_output_tmp_ce7d8_5[0],
                    split_16_low_part_size_8_output_tmp_ce7d8_9[0],
                    and_col60,
                ];
                *lookup_data.verify_bitwise_and_8_2 = [
                    split_16_low_part_size_8_output_tmp_ce7d8_5[0],
                    split_16_low_part_size_8_output_tmp_ce7d8_9[0],
                    and_col60,
                ];

                // Bitwise And Num Bits 8.

                let and_tmp_ce7d8_16 = ((PackedUInt16::from_m31(ms_8_bits_col55))
                    & (PackedUInt16::from_m31(ms_8_bits_col57)));
                let and_col61 = and_tmp_ce7d8_16.as_m31();
                *row[61] = and_col61;
                *sub_component_inputs.verify_bitwise_and_8[3] =
                    [ms_8_bits_col55, ms_8_bits_col57, and_col61];
                *lookup_data.verify_bitwise_and_8_3 = [ms_8_bits_col55, ms_8_bits_col57, and_col61];

                // Bitwise Not Num Bits 16.

                let not_a_tmp_ce7d8_18 = !(PackedUInt16::from_m31(input_limb_10_col10));
                let not_a_col62 = not_a_tmp_ce7d8_18.as_m31();
                *row[62] = not_a_col62;
                *sub_component_inputs.verify_bitwise_not_16[0] = [input_limb_10_col10, not_a_col62];
                *lookup_data.verify_bitwise_not_16_0 = [input_limb_10_col10, not_a_col62];

                // Bitwise Not Num Bits 16.

                let not_a_tmp_ce7d8_20 = !(PackedUInt16::from_m31(input_limb_11_col11));
                let not_a_col63 = not_a_tmp_ce7d8_20.as_m31();
                *row[63] = not_a_col63;
                *sub_component_inputs.verify_bitwise_not_16[1] = [input_limb_11_col11, not_a_col63];
                *lookup_data.verify_bitwise_not_16_1 = [input_limb_11_col11, not_a_col63];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_ce7d8_22 = ((PackedUInt16::from_m31(not_a_col62)) >> (UInt16_8));
                let ms_8_bits_col64 = ms_8_bits_tmp_ce7d8_22.as_m31();
                *row[64] = ms_8_bits_col64;
                let split_16_low_part_size_8_output_tmp_ce7d8_23 = [
                    ((not_a_col62) - ((ms_8_bits_col64) * (M31_256))),
                    ms_8_bits_col64,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_ce7d8_24 = ((PackedUInt16::from_m31(not_a_col63)) >> (UInt16_8));
                let ms_8_bits_col65 = ms_8_bits_tmp_ce7d8_24.as_m31();
                *row[65] = ms_8_bits_col65;
                let split_16_low_part_size_8_output_tmp_ce7d8_25 = [
                    ((not_a_col63) - ((ms_8_bits_col65) * (M31_256))),
                    ms_8_bits_col65,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_ce7d8_26 = ((sha_256_round_input.2 .0[6].low()) >> (UInt16_8));
                let ms_8_bits_col66 = ms_8_bits_tmp_ce7d8_26.as_m31();
                *row[66] = ms_8_bits_col66;
                let split_16_low_part_size_8_output_tmp_ce7d8_27 = [
                    ((input_limb_14_col14) - ((ms_8_bits_col66) * (M31_256))),
                    ms_8_bits_col66,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_ce7d8_28 = ((sha_256_round_input.2 .0[6].high()) >> (UInt16_8));
                let ms_8_bits_col67 = ms_8_bits_tmp_ce7d8_28.as_m31();
                *row[67] = ms_8_bits_col67;
                let split_16_low_part_size_8_output_tmp_ce7d8_29 = [
                    ((input_limb_15_col15) - ((ms_8_bits_col67) * (M31_256))),
                    ms_8_bits_col67,
                ];

                // Bitwise And Num Bits 8.

                let and_tmp_ce7d8_30 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_ce7d8_23[0]))
                        & (PackedUInt16::from_m31(
                            split_16_low_part_size_8_output_tmp_ce7d8_27[0],
                        )));
                let and_col68 = and_tmp_ce7d8_30.as_m31();
                *row[68] = and_col68;
                *sub_component_inputs.verify_bitwise_and_8[4] = [
                    split_16_low_part_size_8_output_tmp_ce7d8_23[0],
                    split_16_low_part_size_8_output_tmp_ce7d8_27[0],
                    and_col68,
                ];
                *lookup_data.verify_bitwise_and_8_4 = [
                    split_16_low_part_size_8_output_tmp_ce7d8_23[0],
                    split_16_low_part_size_8_output_tmp_ce7d8_27[0],
                    and_col68,
                ];

                // Bitwise And Num Bits 8.

                let and_tmp_ce7d8_32 = ((PackedUInt16::from_m31(ms_8_bits_col64))
                    & (PackedUInt16::from_m31(ms_8_bits_col66)));
                let and_col69 = and_tmp_ce7d8_32.as_m31();
                *row[69] = and_col69;
                *sub_component_inputs.verify_bitwise_and_8[5] =
                    [ms_8_bits_col64, ms_8_bits_col66, and_col69];
                *lookup_data.verify_bitwise_and_8_5 = [ms_8_bits_col64, ms_8_bits_col66, and_col69];

                // Bitwise And Num Bits 8.

                let and_tmp_ce7d8_34 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_ce7d8_25[0]))
                        & (PackedUInt16::from_m31(
                            split_16_low_part_size_8_output_tmp_ce7d8_29[0],
                        )));
                let and_col70 = and_tmp_ce7d8_34.as_m31();
                *row[70] = and_col70;
                *sub_component_inputs.verify_bitwise_and_8[6] = [
                    split_16_low_part_size_8_output_tmp_ce7d8_25[0],
                    split_16_low_part_size_8_output_tmp_ce7d8_29[0],
                    and_col70,
                ];
                *lookup_data.verify_bitwise_and_8_6 = [
                    split_16_low_part_size_8_output_tmp_ce7d8_25[0],
                    split_16_low_part_size_8_output_tmp_ce7d8_29[0],
                    and_col70,
                ];

                // Bitwise And Num Bits 8.

                let and_tmp_ce7d8_36 = ((PackedUInt16::from_m31(ms_8_bits_col65))
                    & (PackedUInt16::from_m31(ms_8_bits_col67)));
                let and_col71 = and_tmp_ce7d8_36.as_m31();
                *row[71] = and_col71;
                *sub_component_inputs.verify_bitwise_and_8[7] =
                    [ms_8_bits_col65, ms_8_bits_col67, and_col71];
                *lookup_data.verify_bitwise_and_8_7 = [ms_8_bits_col65, ms_8_bits_col67, and_col71];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_ce7d8_38 =
                    ((PackedUInt16::from_m31(and_col58)) ^ (PackedUInt16::from_m31(and_col68)));
                let xor_col72 = xor_tmp_ce7d8_38.as_m31();
                *row[72] = xor_col72;
                *sub_component_inputs.verify_bitwise_xor_8[0] = [and_col58, and_col68, xor_col72];
                *lookup_data.verify_bitwise_xor_8_0 = [and_col58, and_col68, xor_col72];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_ce7d8_40 =
                    ((PackedUInt16::from_m31(and_col59)) ^ (PackedUInt16::from_m31(and_col69)));
                let xor_col73 = xor_tmp_ce7d8_40.as_m31();
                *row[73] = xor_col73;
                *sub_component_inputs.verify_bitwise_xor_8[1] = [and_col59, and_col69, xor_col73];
                *lookup_data.verify_bitwise_xor_8_1 = [and_col59, and_col69, xor_col73];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_ce7d8_42 =
                    ((PackedUInt16::from_m31(and_col60)) ^ (PackedUInt16::from_m31(and_col70)));
                let xor_col74 = xor_tmp_ce7d8_42.as_m31();
                *row[74] = xor_col74;
                *sub_component_inputs.verify_bitwise_xor_8[2] = [and_col60, and_col70, xor_col74];
                *lookup_data.verify_bitwise_xor_8_2 = [and_col60, and_col70, xor_col74];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_ce7d8_44 =
                    ((PackedUInt16::from_m31(and_col61)) ^ (PackedUInt16::from_m31(and_col71)));
                let xor_col75 = xor_tmp_ce7d8_44.as_m31();
                *row[75] = xor_col75;
                *sub_component_inputs.verify_bitwise_xor_8[3] = [and_col61, and_col71, xor_col75];
                *lookup_data.verify_bitwise_xor_8_3 = [and_col61, and_col71, xor_col75];

                let chl_col76 = ((xor_col72) + ((xor_col73) * (M31_256)));
                *row[76] = chl_col76;
                let chh_col77 = ((xor_col74) + ((xor_col75) * (M31_256)));
                *row[77] = chh_col77;
                let ch_tmp_ce7d8_46 = PackedUInt32::from_limbs([chl_col76, chh_col77]);
                let ch_limb_0_col78 = chl_col76;
                *row[78] = ch_limb_0_col78;
                let ch_limb_1_col79 = chh_col77;
                *row[79] = ch_limb_1_col79;
                *sub_component_inputs.sha_256_k_table[0] = [input_limb_1_col1];
                let sha_256_k_table_output_tmp_ce7d8_47 =
                    PackedSha256KTable::deduce_output([input_limb_1_col1]);
                let sha_256_k_table_output_limb_0_col80 =
                    sha_256_k_table_output_tmp_ce7d8_47.low().as_m31();
                *row[80] = sha_256_k_table_output_limb_0_col80;
                let sha_256_k_table_output_limb_1_col81 =
                    sha_256_k_table_output_tmp_ce7d8_47.high().as_m31();
                *row[81] = sha_256_k_table_output_limb_1_col81;
                *lookup_data.sha_256_k_table_0 = [
                    input_limb_1_col1,
                    sha_256_k_table_output_limb_0_col80,
                    sha_256_k_table_output_limb_1_col81,
                ];

                // Triple Sum 32.

                let triple_sum32_res_tmp_ce7d8_48 = (((sha_256_round_input.2 .0[7])
                    + (sha_256_big_sigma_1_output_tmp_ce7d8_0))
                    + (ch_tmp_ce7d8_46));
                let triple_sum32_res_limb_0_col82 = triple_sum32_res_tmp_ce7d8_48.low().as_m31();
                *row[82] = triple_sum32_res_limb_0_col82;
                let triple_sum32_res_limb_1_col83 = triple_sum32_res_tmp_ce7d8_48.high().as_m31();
                *row[83] = triple_sum32_res_limb_1_col83;
                let triple_sum_32_output_tmp_ce7d8_51 = triple_sum32_res_tmp_ce7d8_48;

                // Triple Sum 32.

                let triple_sum32_res_tmp_ce7d8_52 = (((triple_sum_32_output_tmp_ce7d8_51)
                    + (sha_256_k_table_output_tmp_ce7d8_47))
                    + (sha_256_round_input.2 .1[0]));
                let triple_sum32_res_limb_0_col84 = triple_sum32_res_tmp_ce7d8_52.low().as_m31();
                *row[84] = triple_sum32_res_limb_0_col84;
                let triple_sum32_res_limb_1_col85 = triple_sum32_res_tmp_ce7d8_52.high().as_m31();
                *row[85] = triple_sum32_res_limb_1_col85;
                let triple_sum_32_output_tmp_ce7d8_55 = triple_sum32_res_tmp_ce7d8_52;

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_ce7d8_56 = ((sha_256_round_input.2 .0[0].low()) >> (UInt16_8));
                let ms_8_bits_col86 = ms_8_bits_tmp_ce7d8_56.as_m31();
                *row[86] = ms_8_bits_col86;
                let split_16_low_part_size_8_output_tmp_ce7d8_57 = [
                    ((input_limb_2_col2) - ((ms_8_bits_col86) * (M31_256))),
                    ms_8_bits_col86,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_ce7d8_58 = ((sha_256_round_input.2 .0[0].high()) >> (UInt16_8));
                let ms_8_bits_col87 = ms_8_bits_tmp_ce7d8_58.as_m31();
                *row[87] = ms_8_bits_col87;
                let split_16_low_part_size_8_output_tmp_ce7d8_59 = [
                    ((input_limb_3_col3) - ((ms_8_bits_col87) * (M31_256))),
                    ms_8_bits_col87,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_ce7d8_60 = ((sha_256_round_input.2 .0[1].low()) >> (UInt16_8));
                let ms_8_bits_col88 = ms_8_bits_tmp_ce7d8_60.as_m31();
                *row[88] = ms_8_bits_col88;
                let split_16_low_part_size_8_output_tmp_ce7d8_61 = [
                    ((input_limb_4_col4) - ((ms_8_bits_col88) * (M31_256))),
                    ms_8_bits_col88,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_ce7d8_62 = ((sha_256_round_input.2 .0[1].high()) >> (UInt16_8));
                let ms_8_bits_col89 = ms_8_bits_tmp_ce7d8_62.as_m31();
                *row[89] = ms_8_bits_col89;
                let split_16_low_part_size_8_output_tmp_ce7d8_63 = [
                    ((input_limb_5_col5) - ((ms_8_bits_col89) * (M31_256))),
                    ms_8_bits_col89,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_ce7d8_64 = ((sha_256_round_input.2 .0[2].low()) >> (UInt16_8));
                let ms_8_bits_col90 = ms_8_bits_tmp_ce7d8_64.as_m31();
                *row[90] = ms_8_bits_col90;
                let split_16_low_part_size_8_output_tmp_ce7d8_65 = [
                    ((input_limb_6_col6) - ((ms_8_bits_col90) * (M31_256))),
                    ms_8_bits_col90,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_ce7d8_66 = ((sha_256_round_input.2 .0[2].high()) >> (UInt16_8));
                let ms_8_bits_col91 = ms_8_bits_tmp_ce7d8_66.as_m31();
                *row[91] = ms_8_bits_col91;
                let split_16_low_part_size_8_output_tmp_ce7d8_67 = [
                    ((input_limb_7_col7) - ((ms_8_bits_col91) * (M31_256))),
                    ms_8_bits_col91,
                ];

                // Bitwise And Num Bits 8.

                let and_tmp_ce7d8_68 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_ce7d8_57[0]))
                        & (PackedUInt16::from_m31(
                            split_16_low_part_size_8_output_tmp_ce7d8_61[0],
                        )));
                let and_col92 = and_tmp_ce7d8_68.as_m31();
                *row[92] = and_col92;
                *sub_component_inputs.verify_bitwise_and_8[8] = [
                    split_16_low_part_size_8_output_tmp_ce7d8_57[0],
                    split_16_low_part_size_8_output_tmp_ce7d8_61[0],
                    and_col92,
                ];
                *lookup_data.verify_bitwise_and_8_8 = [
                    split_16_low_part_size_8_output_tmp_ce7d8_57[0],
                    split_16_low_part_size_8_output_tmp_ce7d8_61[0],
                    and_col92,
                ];

                // Bitwise And Num Bits 8.

                let and_tmp_ce7d8_70 = ((PackedUInt16::from_m31(ms_8_bits_col86))
                    & (PackedUInt16::from_m31(ms_8_bits_col88)));
                let and_col93 = and_tmp_ce7d8_70.as_m31();
                *row[93] = and_col93;
                *sub_component_inputs.verify_bitwise_and_8[9] =
                    [ms_8_bits_col86, ms_8_bits_col88, and_col93];
                *lookup_data.verify_bitwise_and_8_9 = [ms_8_bits_col86, ms_8_bits_col88, and_col93];

                // Bitwise And Num Bits 8.

                let and_tmp_ce7d8_72 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_ce7d8_59[0]))
                        & (PackedUInt16::from_m31(
                            split_16_low_part_size_8_output_tmp_ce7d8_63[0],
                        )));
                let and_col94 = and_tmp_ce7d8_72.as_m31();
                *row[94] = and_col94;
                *sub_component_inputs.verify_bitwise_and_8[10] = [
                    split_16_low_part_size_8_output_tmp_ce7d8_59[0],
                    split_16_low_part_size_8_output_tmp_ce7d8_63[0],
                    and_col94,
                ];
                *lookup_data.verify_bitwise_and_8_10 = [
                    split_16_low_part_size_8_output_tmp_ce7d8_59[0],
                    split_16_low_part_size_8_output_tmp_ce7d8_63[0],
                    and_col94,
                ];

                // Bitwise And Num Bits 8.

                let and_tmp_ce7d8_74 = ((PackedUInt16::from_m31(ms_8_bits_col87))
                    & (PackedUInt16::from_m31(ms_8_bits_col89)));
                let and_col95 = and_tmp_ce7d8_74.as_m31();
                *row[95] = and_col95;
                *sub_component_inputs.verify_bitwise_and_8[11] =
                    [ms_8_bits_col87, ms_8_bits_col89, and_col95];
                *lookup_data.verify_bitwise_and_8_11 =
                    [ms_8_bits_col87, ms_8_bits_col89, and_col95];

                // Bitwise And Num Bits 8.

                let and_tmp_ce7d8_76 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_ce7d8_57[0]))
                        & (PackedUInt16::from_m31(
                            split_16_low_part_size_8_output_tmp_ce7d8_65[0],
                        )));
                let and_col96 = and_tmp_ce7d8_76.as_m31();
                *row[96] = and_col96;
                *sub_component_inputs.verify_bitwise_and_8[12] = [
                    split_16_low_part_size_8_output_tmp_ce7d8_57[0],
                    split_16_low_part_size_8_output_tmp_ce7d8_65[0],
                    and_col96,
                ];
                *lookup_data.verify_bitwise_and_8_12 = [
                    split_16_low_part_size_8_output_tmp_ce7d8_57[0],
                    split_16_low_part_size_8_output_tmp_ce7d8_65[0],
                    and_col96,
                ];

                // Bitwise And Num Bits 8.

                let and_tmp_ce7d8_78 = ((PackedUInt16::from_m31(ms_8_bits_col86))
                    & (PackedUInt16::from_m31(ms_8_bits_col90)));
                let and_col97 = and_tmp_ce7d8_78.as_m31();
                *row[97] = and_col97;
                *sub_component_inputs.verify_bitwise_and_8[13] =
                    [ms_8_bits_col86, ms_8_bits_col90, and_col97];
                *lookup_data.verify_bitwise_and_8_13 =
                    [ms_8_bits_col86, ms_8_bits_col90, and_col97];

                // Bitwise And Num Bits 8.

                let and_tmp_ce7d8_80 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_ce7d8_59[0]))
                        & (PackedUInt16::from_m31(
                            split_16_low_part_size_8_output_tmp_ce7d8_67[0],
                        )));
                let and_col98 = and_tmp_ce7d8_80.as_m31();
                *row[98] = and_col98;
                *sub_component_inputs.verify_bitwise_and_8[14] = [
                    split_16_low_part_size_8_output_tmp_ce7d8_59[0],
                    split_16_low_part_size_8_output_tmp_ce7d8_67[0],
                    and_col98,
                ];
                *lookup_data.verify_bitwise_and_8_14 = [
                    split_16_low_part_size_8_output_tmp_ce7d8_59[0],
                    split_16_low_part_size_8_output_tmp_ce7d8_67[0],
                    and_col98,
                ];

                // Bitwise And Num Bits 8.

                let and_tmp_ce7d8_82 = ((PackedUInt16::from_m31(ms_8_bits_col87))
                    & (PackedUInt16::from_m31(ms_8_bits_col91)));
                let and_col99 = and_tmp_ce7d8_82.as_m31();
                *row[99] = and_col99;
                *sub_component_inputs.verify_bitwise_and_8[15] =
                    [ms_8_bits_col87, ms_8_bits_col91, and_col99];
                *lookup_data.verify_bitwise_and_8_15 =
                    [ms_8_bits_col87, ms_8_bits_col91, and_col99];

                // Bitwise And Num Bits 8.

                let and_tmp_ce7d8_84 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_ce7d8_61[0]))
                        & (PackedUInt16::from_m31(
                            split_16_low_part_size_8_output_tmp_ce7d8_65[0],
                        )));
                let and_col100 = and_tmp_ce7d8_84.as_m31();
                *row[100] = and_col100;
                *sub_component_inputs.verify_bitwise_and_8[16] = [
                    split_16_low_part_size_8_output_tmp_ce7d8_61[0],
                    split_16_low_part_size_8_output_tmp_ce7d8_65[0],
                    and_col100,
                ];
                *lookup_data.verify_bitwise_and_8_16 = [
                    split_16_low_part_size_8_output_tmp_ce7d8_61[0],
                    split_16_low_part_size_8_output_tmp_ce7d8_65[0],
                    and_col100,
                ];

                // Bitwise And Num Bits 8.

                let and_tmp_ce7d8_86 = ((PackedUInt16::from_m31(ms_8_bits_col88))
                    & (PackedUInt16::from_m31(ms_8_bits_col90)));
                let and_col101 = and_tmp_ce7d8_86.as_m31();
                *row[101] = and_col101;
                *sub_component_inputs.verify_bitwise_and_8[17] =
                    [ms_8_bits_col88, ms_8_bits_col90, and_col101];
                *lookup_data.verify_bitwise_and_8_17 =
                    [ms_8_bits_col88, ms_8_bits_col90, and_col101];

                // Bitwise And Num Bits 8.

                let and_tmp_ce7d8_88 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_ce7d8_63[0]))
                        & (PackedUInt16::from_m31(
                            split_16_low_part_size_8_output_tmp_ce7d8_67[0],
                        )));
                let and_col102 = and_tmp_ce7d8_88.as_m31();
                *row[102] = and_col102;
                *sub_component_inputs.verify_bitwise_and_8[18] = [
                    split_16_low_part_size_8_output_tmp_ce7d8_63[0],
                    split_16_low_part_size_8_output_tmp_ce7d8_67[0],
                    and_col102,
                ];
                *lookup_data.verify_bitwise_and_8_18 = [
                    split_16_low_part_size_8_output_tmp_ce7d8_63[0],
                    split_16_low_part_size_8_output_tmp_ce7d8_67[0],
                    and_col102,
                ];

                // Bitwise And Num Bits 8.

                let and_tmp_ce7d8_90 = ((PackedUInt16::from_m31(ms_8_bits_col89))
                    & (PackedUInt16::from_m31(ms_8_bits_col91)));
                let and_col103 = and_tmp_ce7d8_90.as_m31();
                *row[103] = and_col103;
                *sub_component_inputs.verify_bitwise_and_8[19] =
                    [ms_8_bits_col89, ms_8_bits_col91, and_col103];
                *lookup_data.verify_bitwise_and_8_19 =
                    [ms_8_bits_col89, ms_8_bits_col91, and_col103];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_ce7d8_92 =
                    ((PackedUInt16::from_m31(and_col92)) ^ (PackedUInt16::from_m31(and_col96)));
                let xor_col104 = xor_tmp_ce7d8_92.as_m31();
                *row[104] = xor_col104;
                *sub_component_inputs.verify_bitwise_xor_8[4] = [and_col92, and_col96, xor_col104];
                *lookup_data.verify_bitwise_xor_8_4 = [and_col92, and_col96, xor_col104];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_ce7d8_94 =
                    ((PackedUInt16::from_m31(and_col93)) ^ (PackedUInt16::from_m31(and_col97)));
                let xor_col105 = xor_tmp_ce7d8_94.as_m31();
                *row[105] = xor_col105;
                *sub_component_inputs.verify_bitwise_xor_8[5] = [and_col93, and_col97, xor_col105];
                *lookup_data.verify_bitwise_xor_8_5 = [and_col93, and_col97, xor_col105];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_ce7d8_96 =
                    ((PackedUInt16::from_m31(and_col94)) ^ (PackedUInt16::from_m31(and_col98)));
                let xor_col106 = xor_tmp_ce7d8_96.as_m31();
                *row[106] = xor_col106;
                *sub_component_inputs.verify_bitwise_xor_8[6] = [and_col94, and_col98, xor_col106];
                *lookup_data.verify_bitwise_xor_8_6 = [and_col94, and_col98, xor_col106];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_ce7d8_98 =
                    ((PackedUInt16::from_m31(and_col95)) ^ (PackedUInt16::from_m31(and_col99)));
                let xor_col107 = xor_tmp_ce7d8_98.as_m31();
                *row[107] = xor_col107;
                *sub_component_inputs.verify_bitwise_xor_8[7] = [and_col95, and_col99, xor_col107];
                *lookup_data.verify_bitwise_xor_8_7 = [and_col95, and_col99, xor_col107];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_ce7d8_100 =
                    ((PackedUInt16::from_m31(xor_col104)) ^ (PackedUInt16::from_m31(and_col100)));
                let xor_col108 = xor_tmp_ce7d8_100.as_m31();
                *row[108] = xor_col108;
                *sub_component_inputs.verify_bitwise_xor_8[8] =
                    [xor_col104, and_col100, xor_col108];
                *lookup_data.verify_bitwise_xor_8_8 = [xor_col104, and_col100, xor_col108];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_ce7d8_102 =
                    ((PackedUInt16::from_m31(xor_col105)) ^ (PackedUInt16::from_m31(and_col101)));
                let xor_col109 = xor_tmp_ce7d8_102.as_m31();
                *row[109] = xor_col109;
                *sub_component_inputs.verify_bitwise_xor_8[9] =
                    [xor_col105, and_col101, xor_col109];
                *lookup_data.verify_bitwise_xor_8_9 = [xor_col105, and_col101, xor_col109];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_ce7d8_104 =
                    ((PackedUInt16::from_m31(xor_col106)) ^ (PackedUInt16::from_m31(and_col102)));
                let xor_col110 = xor_tmp_ce7d8_104.as_m31();
                *row[110] = xor_col110;
                *sub_component_inputs.verify_bitwise_xor_8[10] =
                    [xor_col106, and_col102, xor_col110];
                *lookup_data.verify_bitwise_xor_8_10 = [xor_col106, and_col102, xor_col110];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_ce7d8_106 =
                    ((PackedUInt16::from_m31(xor_col107)) ^ (PackedUInt16::from_m31(and_col103)));
                let xor_col111 = xor_tmp_ce7d8_106.as_m31();
                *row[111] = xor_col111;
                *sub_component_inputs.verify_bitwise_xor_8[11] =
                    [xor_col107, and_col103, xor_col111];
                *lookup_data.verify_bitwise_xor_8_11 = [xor_col107, and_col103, xor_col111];

                let majl_col112 = ((xor_col108) + ((xor_col109) * (M31_256)));
                *row[112] = majl_col112;
                let majh_col113 = ((xor_col110) + ((xor_col111) * (M31_256)));
                *row[113] = majh_col113;
                let maj_tmp_ce7d8_108 = PackedUInt32::from_limbs([majl_col112, majh_col113]);
                let maj_limb_0_col114 = majl_col112;
                *row[114] = maj_limb_0_col114;
                let maj_limb_1_col115 = majh_col113;
                *row[115] = maj_limb_1_col115;

                // Triple Sum 32.

                let triple_sum32_res_tmp_ce7d8_109 =
                    (((sha_256_big_sigma_0_output_tmp_ce7d8_1) + (maj_tmp_ce7d8_108)) + (UInt32_0));
                let triple_sum32_res_limb_0_col116 = triple_sum32_res_tmp_ce7d8_109.low().as_m31();
                *row[116] = triple_sum32_res_limb_0_col116;
                let triple_sum32_res_limb_1_col117 = triple_sum32_res_tmp_ce7d8_109.high().as_m31();
                *row[117] = triple_sum32_res_limb_1_col117;
                let triple_sum_32_output_tmp_ce7d8_112 = triple_sum32_res_tmp_ce7d8_109;

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
                let sha_256_schedule_output_tmp_ce7d8_113 = PackedSha256Schedule::deduce_output([
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
                let sha_256_schedule_output_limb_0_col118 =
                    sha_256_schedule_output_tmp_ce7d8_113.low().as_m31();
                *row[118] = sha_256_schedule_output_limb_0_col118;
                let sha_256_schedule_output_limb_1_col119 =
                    sha_256_schedule_output_tmp_ce7d8_113.high().as_m31();
                *row[119] = sha_256_schedule_output_limb_1_col119;
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
                    sha_256_schedule_output_limb_0_col118,
                    sha_256_schedule_output_limb_1_col119,
                ];

                // Triple Sum 32.

                let triple_sum32_res_tmp_ce7d8_114 = (((triple_sum_32_output_tmp_ce7d8_55)
                    + (sha_256_round_input.2 .0[3]))
                    + (UInt32_0));
                let triple_sum32_res_limb_0_col120 = triple_sum32_res_tmp_ce7d8_114.low().as_m31();
                *row[120] = triple_sum32_res_limb_0_col120;
                let triple_sum32_res_limb_1_col121 = triple_sum32_res_tmp_ce7d8_114.high().as_m31();
                *row[121] = triple_sum32_res_limb_1_col121;
                let triple_sum_32_output_tmp_ce7d8_117 = triple_sum32_res_tmp_ce7d8_114;

                // Triple Sum 32.

                let triple_sum32_res_tmp_ce7d8_118 = (((triple_sum_32_output_tmp_ce7d8_55)
                    + (triple_sum_32_output_tmp_ce7d8_112))
                    + (UInt32_0));
                let triple_sum32_res_limb_0_col122 = triple_sum32_res_tmp_ce7d8_118.low().as_m31();
                *row[122] = triple_sum32_res_limb_0_col122;
                let triple_sum32_res_limb_1_col123 = triple_sum32_res_tmp_ce7d8_118.high().as_m31();
                *row[123] = triple_sum32_res_limb_1_col123;
                let triple_sum_32_output_tmp_ce7d8_121 = triple_sum32_res_tmp_ce7d8_118;

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
                    triple_sum32_res_limb_0_col122,
                    triple_sum32_res_limb_1_col123,
                    input_limb_2_col2,
                    input_limb_3_col3,
                    input_limb_4_col4,
                    input_limb_5_col5,
                    input_limb_6_col6,
                    input_limb_7_col7,
                    triple_sum32_res_limb_0_col120,
                    triple_sum32_res_limb_1_col121,
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
                    sha_256_schedule_output_limb_0_col118,
                    sha_256_schedule_output_limb_1_col119,
                ];
                *row[124] = enabler_col.packed_at(row_index);
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
    verify_bitwise_and_8_0: Vec<[PackedM31; 3]>,
    verify_bitwise_and_8_1: Vec<[PackedM31; 3]>,
    verify_bitwise_and_8_2: Vec<[PackedM31; 3]>,
    verify_bitwise_and_8_3: Vec<[PackedM31; 3]>,
    verify_bitwise_and_8_4: Vec<[PackedM31; 3]>,
    verify_bitwise_and_8_5: Vec<[PackedM31; 3]>,
    verify_bitwise_and_8_6: Vec<[PackedM31; 3]>,
    verify_bitwise_and_8_7: Vec<[PackedM31; 3]>,
    verify_bitwise_and_8_8: Vec<[PackedM31; 3]>,
    verify_bitwise_and_8_9: Vec<[PackedM31; 3]>,
    verify_bitwise_and_8_10: Vec<[PackedM31; 3]>,
    verify_bitwise_and_8_11: Vec<[PackedM31; 3]>,
    verify_bitwise_and_8_12: Vec<[PackedM31; 3]>,
    verify_bitwise_and_8_13: Vec<[PackedM31; 3]>,
    verify_bitwise_and_8_14: Vec<[PackedM31; 3]>,
    verify_bitwise_and_8_15: Vec<[PackedM31; 3]>,
    verify_bitwise_and_8_16: Vec<[PackedM31; 3]>,
    verify_bitwise_and_8_17: Vec<[PackedM31; 3]>,
    verify_bitwise_and_8_18: Vec<[PackedM31; 3]>,
    verify_bitwise_and_8_19: Vec<[PackedM31; 3]>,
    verify_bitwise_not_16_0: Vec<[PackedM31; 2]>,
    verify_bitwise_not_16_1: Vec<[PackedM31; 2]>,
    verify_bitwise_xor_8_0: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_1: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_2: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_3: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_4: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_5: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_6: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_7: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_8: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_9: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_10: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_11: Vec<[PackedM31; 3]>,
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
        verify_bitwise_and_8: &relations::VerifyBitwiseAnd_8,
        verify_bitwise_not_16: &relations::VerifyBitwiseNot_16,
        verify_bitwise_xor_8: &relations::VerifyBitwiseXor_8,
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
            &self.lookup_data.verify_bitwise_and_8_0,
            &self.lookup_data.verify_bitwise_and_8_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_and_8.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_and_8.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_and_8_2,
            &self.lookup_data.verify_bitwise_and_8_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_and_8.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_and_8.combine(values1);
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
            &self.lookup_data.verify_bitwise_and_8_4,
            &self.lookup_data.verify_bitwise_and_8_5,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_and_8.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_and_8.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_and_8_6,
            &self.lookup_data.verify_bitwise_and_8_7,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_and_8.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_and_8.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_xor_8_0,
            &self.lookup_data.verify_bitwise_xor_8_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_xor_8.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_xor_8.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_xor_8_2,
            &self.lookup_data.verify_bitwise_xor_8_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_xor_8.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_xor_8.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.sha_256_k_table_0,
            &self.lookup_data.verify_bitwise_and_8_8,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = sha_256_k_table.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_and_8.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_and_8_9,
            &self.lookup_data.verify_bitwise_and_8_10,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_and_8.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_and_8.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_and_8_11,
            &self.lookup_data.verify_bitwise_and_8_12,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_and_8.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_and_8.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_and_8_13,
            &self.lookup_data.verify_bitwise_and_8_14,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_and_8.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_and_8.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_and_8_15,
            &self.lookup_data.verify_bitwise_and_8_16,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_and_8.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_and_8.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_and_8_17,
            &self.lookup_data.verify_bitwise_and_8_18,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_and_8.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_and_8.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_and_8_19,
            &self.lookup_data.verify_bitwise_xor_8_4,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_and_8.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_xor_8.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_xor_8_5,
            &self.lookup_data.verify_bitwise_xor_8_6,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_xor_8.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_xor_8.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_xor_8_7,
            &self.lookup_data.verify_bitwise_xor_8_8,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_xor_8.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_xor_8.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_xor_8_9,
            &self.lookup_data.verify_bitwise_xor_8_10,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_xor_8.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_xor_8.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_xor_8_11,
            &self.lookup_data.sha_256_schedule_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_xor_8.combine(values0);
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
