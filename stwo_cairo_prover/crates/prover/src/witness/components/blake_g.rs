// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::blake_g::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{
    verify_bitwise_xor_12, verify_bitwise_xor_4, verify_bitwise_xor_7, verify_bitwise_xor_8,
    verify_bitwise_xor_8_b, verify_bitwise_xor_9,
};
use crate::witness::prelude::*;

pub type PackedInputType = [PackedUInt32; 6];

#[derive(Default)]
pub struct ClaimGenerator {
    pub packed_inputs: Vec<PackedInputType>,
}

impl ClaimGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.packed_inputs.is_empty()
    }

    pub fn write_trace(
        mut self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        verify_bitwise_xor_8_state: &verify_bitwise_xor_8::ClaimGenerator,
        verify_bitwise_xor_8_b_state: &verify_bitwise_xor_8_b::ClaimGenerator,
        verify_bitwise_xor_12_state: &verify_bitwise_xor_12::ClaimGenerator,
        verify_bitwise_xor_4_state: &verify_bitwise_xor_4::ClaimGenerator,
        verify_bitwise_xor_7_state: &verify_bitwise_xor_7::ClaimGenerator,
        verify_bitwise_xor_9_state: &verify_bitwise_xor_9::ClaimGenerator,
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
            verify_bitwise_xor_8_state,
            verify_bitwise_xor_8_b_state,
            verify_bitwise_xor_12_state,
            verify_bitwise_xor_4_state,
            verify_bitwise_xor_7_state,
            verify_bitwise_xor_9_state,
        );
        sub_component_inputs
            .verify_bitwise_xor_8
            .iter()
            .for_each(|inputs| {
                verify_bitwise_xor_8_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .verify_bitwise_xor_8_b
            .iter()
            .for_each(|inputs| {
                verify_bitwise_xor_8_b_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .verify_bitwise_xor_12
            .iter()
            .for_each(|inputs| {
                verify_bitwise_xor_12_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .verify_bitwise_xor_4
            .iter()
            .for_each(|inputs| {
                verify_bitwise_xor_4_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .verify_bitwise_xor_7
            .iter()
            .for_each(|inputs| {
                verify_bitwise_xor_7_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .verify_bitwise_xor_9
            .iter()
            .for_each(|inputs| {
                verify_bitwise_xor_9_state.add_packed_inputs(inputs);
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
    verify_bitwise_xor_8: [Vec<verify_bitwise_xor_8::PackedInputType>; 4],
    verify_bitwise_xor_8_b: [Vec<verify_bitwise_xor_8_b::PackedInputType>; 4],
    verify_bitwise_xor_12: [Vec<verify_bitwise_xor_12::PackedInputType>; 2],
    verify_bitwise_xor_4: [Vec<verify_bitwise_xor_4::PackedInputType>; 2],
    verify_bitwise_xor_7: [Vec<verify_bitwise_xor_7::PackedInputType>; 2],
    verify_bitwise_xor_9: [Vec<verify_bitwise_xor_9::PackedInputType>; 2],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    n_rows: usize,
    verify_bitwise_xor_8_state: &verify_bitwise_xor_8::ClaimGenerator,
    verify_bitwise_xor_8_b_state: &verify_bitwise_xor_8_b::ClaimGenerator,
    verify_bitwise_xor_12_state: &verify_bitwise_xor_12::ClaimGenerator,
    verify_bitwise_xor_4_state: &verify_bitwise_xor_4::ClaimGenerator,
    verify_bitwise_xor_7_state: &verify_bitwise_xor_7::ClaimGenerator,
    verify_bitwise_xor_9_state: &verify_bitwise_xor_9::ClaimGenerator,
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

    let M31_128 = PackedM31::broadcast(M31::from(128));
    let M31_16 = PackedM31::broadcast(M31::from(16));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_4096 = PackedM31::broadcast(M31::from(4096));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let UInt16_12 = PackedUInt16::broadcast(UInt16::from(12));
    let UInt16_7 = PackedUInt16::broadcast(UInt16::from(7));
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
            |(row_index, (row, lookup_data, sub_component_inputs, blake_g_input))| {
                let input_limb_0_col0 = blake_g_input[0].low().as_m31();
                *row[0] = input_limb_0_col0;
                let input_limb_1_col1 = blake_g_input[0].high().as_m31();
                *row[1] = input_limb_1_col1;
                let input_limb_2_col2 = blake_g_input[1].low().as_m31();
                *row[2] = input_limb_2_col2;
                let input_limb_3_col3 = blake_g_input[1].high().as_m31();
                *row[3] = input_limb_3_col3;
                let input_limb_4_col4 = blake_g_input[2].low().as_m31();
                *row[4] = input_limb_4_col4;
                let input_limb_5_col5 = blake_g_input[2].high().as_m31();
                *row[5] = input_limb_5_col5;
                let input_limb_6_col6 = blake_g_input[3].low().as_m31();
                *row[6] = input_limb_6_col6;
                let input_limb_7_col7 = blake_g_input[3].high().as_m31();
                *row[7] = input_limb_7_col7;
                let input_limb_8_col8 = blake_g_input[4].low().as_m31();
                *row[8] = input_limb_8_col8;
                let input_limb_9_col9 = blake_g_input[4].high().as_m31();
                *row[9] = input_limb_9_col9;
                let input_limb_10_col10 = blake_g_input[5].low().as_m31();
                *row[10] = input_limb_10_col10;
                let input_limb_11_col11 = blake_g_input[5].high().as_m31();
                *row[11] = input_limb_11_col11;

                // Triple Sum 32.

                let triple_sum32_res_tmp_f72c8_0 =
                    (((blake_g_input[0]) + (blake_g_input[1])) + (blake_g_input[4]));
                let triple_sum32_res_limb_0_col12 = triple_sum32_res_tmp_f72c8_0.low().as_m31();
                *row[12] = triple_sum32_res_limb_0_col12;
                let triple_sum32_res_limb_1_col13 = triple_sum32_res_tmp_f72c8_0.high().as_m31();
                *row[13] = triple_sum32_res_limb_1_col13;
                let triple_sum_32_output_tmp_f72c8_3 = triple_sum32_res_tmp_f72c8_0;

                // Xor Rot 32 R 16.

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_f72c8_4 =
                    ((triple_sum_32_output_tmp_f72c8_3.low()) >> (UInt16_8));
                let ms_8_bits_col14 = ms_8_bits_tmp_f72c8_4.as_m31();
                *row[14] = ms_8_bits_col14;
                let split_16_low_part_size_8_output_tmp_f72c8_5 = [
                    ((triple_sum32_res_limb_0_col12) - ((ms_8_bits_col14) * (M31_256))),
                    ms_8_bits_col14,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_f72c8_6 =
                    ((triple_sum_32_output_tmp_f72c8_3.high()) >> (UInt16_8));
                let ms_8_bits_col15 = ms_8_bits_tmp_f72c8_6.as_m31();
                *row[15] = ms_8_bits_col15;
                let split_16_low_part_size_8_output_tmp_f72c8_7 = [
                    ((triple_sum32_res_limb_1_col13) - ((ms_8_bits_col15) * (M31_256))),
                    ms_8_bits_col15,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_f72c8_8 = ((blake_g_input[3].low()) >> (UInt16_8));
                let ms_8_bits_col16 = ms_8_bits_tmp_f72c8_8.as_m31();
                *row[16] = ms_8_bits_col16;
                let split_16_low_part_size_8_output_tmp_f72c8_9 = [
                    ((input_limb_6_col6) - ((ms_8_bits_col16) * (M31_256))),
                    ms_8_bits_col16,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_f72c8_10 = ((blake_g_input[3].high()) >> (UInt16_8));
                let ms_8_bits_col17 = ms_8_bits_tmp_f72c8_10.as_m31();
                *row[17] = ms_8_bits_col17;
                let split_16_low_part_size_8_output_tmp_f72c8_11 = [
                    ((input_limb_7_col7) - ((ms_8_bits_col17) * (M31_256))),
                    ms_8_bits_col17,
                ];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_f72c8_12 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_f72c8_5[0]))
                        ^ (PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_f72c8_9[0])));
                let xor_col18 = xor_tmp_f72c8_12.as_m31();
                *row[18] = xor_col18;
                *sub_component_inputs.verify_bitwise_xor_8[0] = [
                    split_16_low_part_size_8_output_tmp_f72c8_5[0],
                    split_16_low_part_size_8_output_tmp_f72c8_9[0],
                    xor_col18,
                ];
                *lookup_data.verify_bitwise_xor_8_0 = [
                    split_16_low_part_size_8_output_tmp_f72c8_5[0],
                    split_16_low_part_size_8_output_tmp_f72c8_9[0],
                    xor_col18,
                ];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_f72c8_14 = ((PackedUInt16::from_m31(ms_8_bits_col14))
                    ^ (PackedUInt16::from_m31(ms_8_bits_col16)));
                let xor_col19 = xor_tmp_f72c8_14.as_m31();
                *row[19] = xor_col19;
                *sub_component_inputs.verify_bitwise_xor_8[1] =
                    [ms_8_bits_col14, ms_8_bits_col16, xor_col19];
                *lookup_data.verify_bitwise_xor_8_1 = [ms_8_bits_col14, ms_8_bits_col16, xor_col19];

                // Bitwise Xor Num Bits 8 B.

                let xor_tmp_f72c8_16 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_f72c8_7[0]))
                        ^ (PackedUInt16::from_m31(
                            split_16_low_part_size_8_output_tmp_f72c8_11[0],
                        )));
                let xor_col20 = xor_tmp_f72c8_16.as_m31();
                *row[20] = xor_col20;
                *sub_component_inputs.verify_bitwise_xor_8_b[0] = [
                    split_16_low_part_size_8_output_tmp_f72c8_7[0],
                    split_16_low_part_size_8_output_tmp_f72c8_11[0],
                    xor_col20,
                ];
                *lookup_data.verify_bitwise_xor_8_b_0 = [
                    split_16_low_part_size_8_output_tmp_f72c8_7[0],
                    split_16_low_part_size_8_output_tmp_f72c8_11[0],
                    xor_col20,
                ];

                // Bitwise Xor Num Bits 8 B.

                let xor_tmp_f72c8_18 = ((PackedUInt16::from_m31(ms_8_bits_col15))
                    ^ (PackedUInt16::from_m31(ms_8_bits_col17)));
                let xor_col21 = xor_tmp_f72c8_18.as_m31();
                *row[21] = xor_col21;
                *sub_component_inputs.verify_bitwise_xor_8_b[1] =
                    [ms_8_bits_col15, ms_8_bits_col17, xor_col21];
                *lookup_data.verify_bitwise_xor_8_b_1 =
                    [ms_8_bits_col15, ms_8_bits_col17, xor_col21];

                let xor_rot_16_output_tmp_f72c8_20 = PackedUInt32::from_limbs([
                    ((xor_col20) + ((xor_col21) * (M31_256))),
                    ((xor_col18) + ((xor_col19) * (M31_256))),
                ]);
                let xor_rot_32_r_16_output_tmp_f72c8_21 = xor_rot_16_output_tmp_f72c8_20;

                // Triple Sum 32.

                let triple_sum32_res_tmp_f72c8_22 =
                    (((blake_g_input[2]) + (xor_rot_32_r_16_output_tmp_f72c8_21)) + (UInt32_0));
                let triple_sum32_res_limb_0_col22 = triple_sum32_res_tmp_f72c8_22.low().as_m31();
                *row[22] = triple_sum32_res_limb_0_col22;
                let triple_sum32_res_limb_1_col23 = triple_sum32_res_tmp_f72c8_22.high().as_m31();
                *row[23] = triple_sum32_res_limb_1_col23;
                let triple_sum_32_output_tmp_f72c8_25 = triple_sum32_res_tmp_f72c8_22;

                // Xor Rot 32 R 12.

                // Split 16 Low Part Size 12.

                let ms_4_bits_tmp_f72c8_26 = ((blake_g_input[1].low()) >> (UInt16_12));
                let ms_4_bits_col24 = ms_4_bits_tmp_f72c8_26.as_m31();
                *row[24] = ms_4_bits_col24;
                let split_16_low_part_size_12_output_tmp_f72c8_27 = [
                    ((input_limb_2_col2) - ((ms_4_bits_col24) * (M31_4096))),
                    ms_4_bits_col24,
                ];

                // Split 16 Low Part Size 12.

                let ms_4_bits_tmp_f72c8_28 = ((blake_g_input[1].high()) >> (UInt16_12));
                let ms_4_bits_col25 = ms_4_bits_tmp_f72c8_28.as_m31();
                *row[25] = ms_4_bits_col25;
                let split_16_low_part_size_12_output_tmp_f72c8_29 = [
                    ((input_limb_3_col3) - ((ms_4_bits_col25) * (M31_4096))),
                    ms_4_bits_col25,
                ];

                // Split 16 Low Part Size 12.

                let ms_4_bits_tmp_f72c8_30 =
                    ((triple_sum_32_output_tmp_f72c8_25.low()) >> (UInt16_12));
                let ms_4_bits_col26 = ms_4_bits_tmp_f72c8_30.as_m31();
                *row[26] = ms_4_bits_col26;
                let split_16_low_part_size_12_output_tmp_f72c8_31 = [
                    ((triple_sum32_res_limb_0_col22) - ((ms_4_bits_col26) * (M31_4096))),
                    ms_4_bits_col26,
                ];

                // Split 16 Low Part Size 12.

                let ms_4_bits_tmp_f72c8_32 =
                    ((triple_sum_32_output_tmp_f72c8_25.high()) >> (UInt16_12));
                let ms_4_bits_col27 = ms_4_bits_tmp_f72c8_32.as_m31();
                *row[27] = ms_4_bits_col27;
                let split_16_low_part_size_12_output_tmp_f72c8_33 = [
                    ((triple_sum32_res_limb_1_col23) - ((ms_4_bits_col27) * (M31_4096))),
                    ms_4_bits_col27,
                ];

                // Bitwise Xor Num Bits 12.

                let xor_tmp_f72c8_34 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_12_output_tmp_f72c8_27[0]))
                        ^ (PackedUInt16::from_m31(
                            split_16_low_part_size_12_output_tmp_f72c8_31[0],
                        )));
                let xor_col28 = xor_tmp_f72c8_34.as_m31();
                *row[28] = xor_col28;
                *sub_component_inputs.verify_bitwise_xor_12[0] = [
                    split_16_low_part_size_12_output_tmp_f72c8_27[0],
                    split_16_low_part_size_12_output_tmp_f72c8_31[0],
                    xor_col28,
                ];
                *lookup_data.verify_bitwise_xor_12_0 = [
                    split_16_low_part_size_12_output_tmp_f72c8_27[0],
                    split_16_low_part_size_12_output_tmp_f72c8_31[0],
                    xor_col28,
                ];

                // Bitwise Xor Num Bits 4.

                let xor_tmp_f72c8_36 = ((PackedUInt16::from_m31(ms_4_bits_col24))
                    ^ (PackedUInt16::from_m31(ms_4_bits_col26)));
                let xor_col29 = xor_tmp_f72c8_36.as_m31();
                *row[29] = xor_col29;
                *sub_component_inputs.verify_bitwise_xor_4[0] =
                    [ms_4_bits_col24, ms_4_bits_col26, xor_col29];
                *lookup_data.verify_bitwise_xor_4_0 = [ms_4_bits_col24, ms_4_bits_col26, xor_col29];

                // Bitwise Xor Num Bits 12.

                let xor_tmp_f72c8_38 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_12_output_tmp_f72c8_29[0]))
                        ^ (PackedUInt16::from_m31(
                            split_16_low_part_size_12_output_tmp_f72c8_33[0],
                        )));
                let xor_col30 = xor_tmp_f72c8_38.as_m31();
                *row[30] = xor_col30;
                *sub_component_inputs.verify_bitwise_xor_12[1] = [
                    split_16_low_part_size_12_output_tmp_f72c8_29[0],
                    split_16_low_part_size_12_output_tmp_f72c8_33[0],
                    xor_col30,
                ];
                *lookup_data.verify_bitwise_xor_12_1 = [
                    split_16_low_part_size_12_output_tmp_f72c8_29[0],
                    split_16_low_part_size_12_output_tmp_f72c8_33[0],
                    xor_col30,
                ];

                // Bitwise Xor Num Bits 4.

                let xor_tmp_f72c8_40 = ((PackedUInt16::from_m31(ms_4_bits_col25))
                    ^ (PackedUInt16::from_m31(ms_4_bits_col27)));
                let xor_col31 = xor_tmp_f72c8_40.as_m31();
                *row[31] = xor_col31;
                *sub_component_inputs.verify_bitwise_xor_4[1] =
                    [ms_4_bits_col25, ms_4_bits_col27, xor_col31];
                *lookup_data.verify_bitwise_xor_4_1 = [ms_4_bits_col25, ms_4_bits_col27, xor_col31];

                let xor_rot_12_output_tmp_f72c8_42 = PackedUInt32::from_limbs([
                    ((xor_col29) + ((xor_col30) * (M31_16))),
                    ((xor_col31) + ((xor_col28) * (M31_16))),
                ]);
                let xor_rot_32_r_12_output_tmp_f72c8_43 = xor_rot_12_output_tmp_f72c8_42;

                // Triple Sum 32.

                let triple_sum32_res_tmp_f72c8_44 = (((triple_sum_32_output_tmp_f72c8_3)
                    + (xor_rot_32_r_12_output_tmp_f72c8_43))
                    + (blake_g_input[5]));
                let triple_sum32_res_limb_0_col32 = triple_sum32_res_tmp_f72c8_44.low().as_m31();
                *row[32] = triple_sum32_res_limb_0_col32;
                let triple_sum32_res_limb_1_col33 = triple_sum32_res_tmp_f72c8_44.high().as_m31();
                *row[33] = triple_sum32_res_limb_1_col33;
                let triple_sum_32_output_tmp_f72c8_47 = triple_sum32_res_tmp_f72c8_44;

                // Xor Rot 32 R 8.

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_f72c8_48 =
                    ((triple_sum_32_output_tmp_f72c8_47.low()) >> (UInt16_8));
                let ms_8_bits_col34 = ms_8_bits_tmp_f72c8_48.as_m31();
                *row[34] = ms_8_bits_col34;
                let split_16_low_part_size_8_output_tmp_f72c8_49 = [
                    ((triple_sum32_res_limb_0_col32) - ((ms_8_bits_col34) * (M31_256))),
                    ms_8_bits_col34,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_f72c8_50 =
                    ((triple_sum_32_output_tmp_f72c8_47.high()) >> (UInt16_8));
                let ms_8_bits_col35 = ms_8_bits_tmp_f72c8_50.as_m31();
                *row[35] = ms_8_bits_col35;
                let split_16_low_part_size_8_output_tmp_f72c8_51 = [
                    ((triple_sum32_res_limb_1_col33) - ((ms_8_bits_col35) * (M31_256))),
                    ms_8_bits_col35,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_f72c8_52 =
                    ((xor_rot_32_r_16_output_tmp_f72c8_21.low()) >> (UInt16_8));
                let ms_8_bits_col36 = ms_8_bits_tmp_f72c8_52.as_m31();
                *row[36] = ms_8_bits_col36;
                let split_16_low_part_size_8_output_tmp_f72c8_53 = [
                    ((xor_rot_16_output_tmp_f72c8_20.low().as_m31())
                        - ((ms_8_bits_col36) * (M31_256))),
                    ms_8_bits_col36,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_f72c8_54 =
                    ((xor_rot_32_r_16_output_tmp_f72c8_21.high()) >> (UInt16_8));
                let ms_8_bits_col37 = ms_8_bits_tmp_f72c8_54.as_m31();
                *row[37] = ms_8_bits_col37;
                let split_16_low_part_size_8_output_tmp_f72c8_55 = [
                    ((xor_rot_16_output_tmp_f72c8_20.high().as_m31())
                        - ((ms_8_bits_col37) * (M31_256))),
                    ms_8_bits_col37,
                ];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_f72c8_56 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_f72c8_49[0]))
                        ^ (PackedUInt16::from_m31(
                            split_16_low_part_size_8_output_tmp_f72c8_53[0],
                        )));
                let xor_col38 = xor_tmp_f72c8_56.as_m31();
                *row[38] = xor_col38;
                *sub_component_inputs.verify_bitwise_xor_8[2] = [
                    split_16_low_part_size_8_output_tmp_f72c8_49[0],
                    split_16_low_part_size_8_output_tmp_f72c8_53[0],
                    xor_col38,
                ];
                *lookup_data.verify_bitwise_xor_8_2 = [
                    split_16_low_part_size_8_output_tmp_f72c8_49[0],
                    split_16_low_part_size_8_output_tmp_f72c8_53[0],
                    xor_col38,
                ];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_f72c8_58 = ((PackedUInt16::from_m31(ms_8_bits_col34))
                    ^ (PackedUInt16::from_m31(ms_8_bits_col36)));
                let xor_col39 = xor_tmp_f72c8_58.as_m31();
                *row[39] = xor_col39;
                *sub_component_inputs.verify_bitwise_xor_8[3] =
                    [ms_8_bits_col34, ms_8_bits_col36, xor_col39];
                *lookup_data.verify_bitwise_xor_8_3 = [ms_8_bits_col34, ms_8_bits_col36, xor_col39];

                // Bitwise Xor Num Bits 8 B.

                let xor_tmp_f72c8_60 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_f72c8_51[0]))
                        ^ (PackedUInt16::from_m31(
                            split_16_low_part_size_8_output_tmp_f72c8_55[0],
                        )));
                let xor_col40 = xor_tmp_f72c8_60.as_m31();
                *row[40] = xor_col40;
                *sub_component_inputs.verify_bitwise_xor_8_b[2] = [
                    split_16_low_part_size_8_output_tmp_f72c8_51[0],
                    split_16_low_part_size_8_output_tmp_f72c8_55[0],
                    xor_col40,
                ];
                *lookup_data.verify_bitwise_xor_8_b_2 = [
                    split_16_low_part_size_8_output_tmp_f72c8_51[0],
                    split_16_low_part_size_8_output_tmp_f72c8_55[0],
                    xor_col40,
                ];

                // Bitwise Xor Num Bits 8 B.

                let xor_tmp_f72c8_62 = ((PackedUInt16::from_m31(ms_8_bits_col35))
                    ^ (PackedUInt16::from_m31(ms_8_bits_col37)));
                let xor_col41 = xor_tmp_f72c8_62.as_m31();
                *row[41] = xor_col41;
                *sub_component_inputs.verify_bitwise_xor_8_b[3] =
                    [ms_8_bits_col35, ms_8_bits_col37, xor_col41];
                *lookup_data.verify_bitwise_xor_8_b_3 =
                    [ms_8_bits_col35, ms_8_bits_col37, xor_col41];

                let xor_rot_8_output_tmp_f72c8_64 = PackedUInt32::from_limbs([
                    ((xor_col39) + ((xor_col40) * (M31_256))),
                    ((xor_col41) + ((xor_col38) * (M31_256))),
                ]);
                let xor_rot_32_r_8_output_tmp_f72c8_65 = xor_rot_8_output_tmp_f72c8_64;

                // Triple Sum 32.

                let triple_sum32_res_tmp_f72c8_66 = (((triple_sum_32_output_tmp_f72c8_25)
                    + (xor_rot_32_r_8_output_tmp_f72c8_65))
                    + (UInt32_0));
                let triple_sum32_res_limb_0_col42 = triple_sum32_res_tmp_f72c8_66.low().as_m31();
                *row[42] = triple_sum32_res_limb_0_col42;
                let triple_sum32_res_limb_1_col43 = triple_sum32_res_tmp_f72c8_66.high().as_m31();
                *row[43] = triple_sum32_res_limb_1_col43;
                let triple_sum_32_output_tmp_f72c8_69 = triple_sum32_res_tmp_f72c8_66;

                // Xor Rot 32 R 7.

                // Split 16 Low Part Size 7.

                let ms_9_bits_tmp_f72c8_70 =
                    ((xor_rot_32_r_12_output_tmp_f72c8_43.low()) >> (UInt16_7));
                let ms_9_bits_col44 = ms_9_bits_tmp_f72c8_70.as_m31();
                *row[44] = ms_9_bits_col44;
                let split_16_low_part_size_7_output_tmp_f72c8_71 = [
                    ((xor_rot_12_output_tmp_f72c8_42.low().as_m31())
                        - ((ms_9_bits_col44) * (M31_128))),
                    ms_9_bits_col44,
                ];

                // Split 16 Low Part Size 7.

                let ms_9_bits_tmp_f72c8_72 =
                    ((xor_rot_32_r_12_output_tmp_f72c8_43.high()) >> (UInt16_7));
                let ms_9_bits_col45 = ms_9_bits_tmp_f72c8_72.as_m31();
                *row[45] = ms_9_bits_col45;
                let split_16_low_part_size_7_output_tmp_f72c8_73 = [
                    ((xor_rot_12_output_tmp_f72c8_42.high().as_m31())
                        - ((ms_9_bits_col45) * (M31_128))),
                    ms_9_bits_col45,
                ];

                // Split 16 Low Part Size 7.

                let ms_9_bits_tmp_f72c8_74 =
                    ((triple_sum_32_output_tmp_f72c8_69.low()) >> (UInt16_7));
                let ms_9_bits_col46 = ms_9_bits_tmp_f72c8_74.as_m31();
                *row[46] = ms_9_bits_col46;
                let split_16_low_part_size_7_output_tmp_f72c8_75 = [
                    ((triple_sum32_res_limb_0_col42) - ((ms_9_bits_col46) * (M31_128))),
                    ms_9_bits_col46,
                ];

                // Split 16 Low Part Size 7.

                let ms_9_bits_tmp_f72c8_76 =
                    ((triple_sum_32_output_tmp_f72c8_69.high()) >> (UInt16_7));
                let ms_9_bits_col47 = ms_9_bits_tmp_f72c8_76.as_m31();
                *row[47] = ms_9_bits_col47;
                let split_16_low_part_size_7_output_tmp_f72c8_77 = [
                    ((triple_sum32_res_limb_1_col43) - ((ms_9_bits_col47) * (M31_128))),
                    ms_9_bits_col47,
                ];

                // Bitwise Xor Num Bits 7.

                let xor_tmp_f72c8_78 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_7_output_tmp_f72c8_71[0]))
                        ^ (PackedUInt16::from_m31(
                            split_16_low_part_size_7_output_tmp_f72c8_75[0],
                        )));
                let xor_col48 = xor_tmp_f72c8_78.as_m31();
                *row[48] = xor_col48;
                *sub_component_inputs.verify_bitwise_xor_7[0] = [
                    split_16_low_part_size_7_output_tmp_f72c8_71[0],
                    split_16_low_part_size_7_output_tmp_f72c8_75[0],
                    xor_col48,
                ];
                *lookup_data.verify_bitwise_xor_7_0 = [
                    split_16_low_part_size_7_output_tmp_f72c8_71[0],
                    split_16_low_part_size_7_output_tmp_f72c8_75[0],
                    xor_col48,
                ];

                // Bitwise Xor Num Bits 9.

                let xor_tmp_f72c8_80 = ((PackedUInt16::from_m31(ms_9_bits_col44))
                    ^ (PackedUInt16::from_m31(ms_9_bits_col46)));
                let xor_col49 = xor_tmp_f72c8_80.as_m31();
                *row[49] = xor_col49;
                *sub_component_inputs.verify_bitwise_xor_9[0] =
                    [ms_9_bits_col44, ms_9_bits_col46, xor_col49];
                *lookup_data.verify_bitwise_xor_9_0 = [ms_9_bits_col44, ms_9_bits_col46, xor_col49];

                // Bitwise Xor Num Bits 7.

                let xor_tmp_f72c8_82 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_7_output_tmp_f72c8_73[0]))
                        ^ (PackedUInt16::from_m31(
                            split_16_low_part_size_7_output_tmp_f72c8_77[0],
                        )));
                let xor_col50 = xor_tmp_f72c8_82.as_m31();
                *row[50] = xor_col50;
                *sub_component_inputs.verify_bitwise_xor_7[1] = [
                    split_16_low_part_size_7_output_tmp_f72c8_73[0],
                    split_16_low_part_size_7_output_tmp_f72c8_77[0],
                    xor_col50,
                ];
                *lookup_data.verify_bitwise_xor_7_1 = [
                    split_16_low_part_size_7_output_tmp_f72c8_73[0],
                    split_16_low_part_size_7_output_tmp_f72c8_77[0],
                    xor_col50,
                ];

                // Bitwise Xor Num Bits 9.

                let xor_tmp_f72c8_84 = ((PackedUInt16::from_m31(ms_9_bits_col45))
                    ^ (PackedUInt16::from_m31(ms_9_bits_col47)));
                let xor_col51 = xor_tmp_f72c8_84.as_m31();
                *row[51] = xor_col51;
                *sub_component_inputs.verify_bitwise_xor_9[1] =
                    [ms_9_bits_col45, ms_9_bits_col47, xor_col51];
                *lookup_data.verify_bitwise_xor_9_1 = [ms_9_bits_col45, ms_9_bits_col47, xor_col51];

                let xor_rot_7_output_tmp_f72c8_86 = PackedUInt32::from_limbs([
                    ((xor_col49) + ((xor_col50) * (M31_512))),
                    ((xor_col51) + ((xor_col48) * (M31_512))),
                ]);
                let xor_rot_32_r_7_output_tmp_f72c8_87 = xor_rot_7_output_tmp_f72c8_86;

                *lookup_data.blake_g_0 = [
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
                    triple_sum32_res_limb_0_col32,
                    triple_sum32_res_limb_1_col33,
                    xor_rot_7_output_tmp_f72c8_86.low().as_m31(),
                    xor_rot_7_output_tmp_f72c8_86.high().as_m31(),
                    triple_sum32_res_limb_0_col42,
                    triple_sum32_res_limb_1_col43,
                    xor_rot_8_output_tmp_f72c8_64.low().as_m31(),
                    xor_rot_8_output_tmp_f72c8_64.high().as_m31(),
                ];
                *row[52] = enabler_col.packed_at(row_index);
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    blake_g_0: Vec<[PackedM31; 20]>,
    verify_bitwise_xor_12_0: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_12_1: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_4_0: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_4_1: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_7_0: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_7_1: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_0: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_1: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_2: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_3: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_b_0: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_b_1: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_b_2: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_b_3: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_0: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_1: Vec<[PackedM31; 3]>,
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
        verify_bitwise_xor_8: &relations::VerifyBitwiseXor_8,
        verify_bitwise_xor_8_b: &relations::VerifyBitwiseXor_8_B,
        verify_bitwise_xor_12: &relations::VerifyBitwiseXor_12,
        verify_bitwise_xor_4: &relations::VerifyBitwiseXor_4,
        verify_bitwise_xor_7: &relations::VerifyBitwiseXor_7,
        verify_bitwise_xor_9: &relations::VerifyBitwiseXor_9,
        blake_g: &relations::BlakeG,
    ) -> InteractionClaim {
        let enabler_col = Enabler::new(self.n_rows);
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
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
            &self.lookup_data.verify_bitwise_xor_8_b_0,
            &self.lookup_data.verify_bitwise_xor_8_b_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_xor_8_b.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_xor_8_b.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_xor_12_0,
            &self.lookup_data.verify_bitwise_xor_4_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_xor_12.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_xor_4.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_xor_12_1,
            &self.lookup_data.verify_bitwise_xor_4_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_xor_12.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_xor_4.combine(values1);
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
            &self.lookup_data.verify_bitwise_xor_8_b_2,
            &self.lookup_data.verify_bitwise_xor_8_b_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_xor_8_b.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_xor_8_b.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_xor_7_0,
            &self.lookup_data.verify_bitwise_xor_9_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_xor_7.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_xor_9.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_xor_7_1,
            &self.lookup_data.verify_bitwise_xor_9_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_xor_7.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_xor_9.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(), &self.lookup_data.blake_g_0)
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (writer, values))| {
                let denom = blake_g.combine(values);
                writer.write_frac(-PackedQM31::one() * enabler_col.packed_at(i), denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
