#![allow(unused_parens)]
#![allow(unused_imports)]
use itertools::Itertools;

use super::component::{Claim, InteractionClaim};
use crate::components::prelude::proving::*;
use crate::components::{
    verify_bitwise_xor_12, verify_bitwise_xor_4, verify_bitwise_xor_7, verify_bitwise_xor_8,
    verify_bitwise_xor_9,
};

pub type PackedInputType = [PackedUInt32; 6];
const N_TRACE_COLUMNS: usize = 52;

#[derive(Default)]
pub struct ClaimGenerator {
    pub inputs: Vec<PackedInputType>,
}
impl ClaimGenerator {
    pub fn new() -> Self {
        Self { inputs: vec![] }
    }

    pub fn write_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        verify_bitwise_xor_12_state: &verify_bitwise_xor_12::ClaimGenerator,
        verify_bitwise_xor_4_state: &verify_bitwise_xor_4::ClaimGenerator,
        verify_bitwise_xor_7_state: &verify_bitwise_xor_7::ClaimGenerator,
        verify_bitwise_xor_8_state: &verify_bitwise_xor_8::ClaimGenerator,
        verify_bitwise_xor_9_state: &verify_bitwise_xor_9::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let n_vec_rows = self.inputs.len();
        assert!(n_vec_rows.is_power_of_two());
        let log_size = n_vec_rows.ilog2() + LOG_N_LANES;

        let (trace, lookup_data) = write_trace_simd(
            n_vec_rows,
            self.inputs,
            verify_bitwise_xor_12_state,
            verify_bitwise_xor_4_state,
            verify_bitwise_xor_7_state,
            verify_bitwise_xor_8_state,
            verify_bitwise_xor_9_state,
        );
        tree_builder.extend_evals(trace.to_evals());

        (
            Claim { log_size },
            InteractionClaimGenerator {
                log_size,
                lookup_data,
            },
        )
    }

    pub fn add_packed_inputs(&mut self, inputs: &[PackedInputType]) {
        self.inputs.extend(inputs);
    }
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    n_rows: usize,
    inputs: Vec<PackedInputType>,
    verify_bitwise_xor_12_state: &verify_bitwise_xor_12::ClaimGenerator,
    verify_bitwise_xor_4_state: &verify_bitwise_xor_4::ClaimGenerator,
    verify_bitwise_xor_7_state: &verify_bitwise_xor_7::ClaimGenerator,
    verify_bitwise_xor_8_state: &verify_bitwise_xor_8::ClaimGenerator,
    verify_bitwise_xor_9_state: &verify_bitwise_xor_9::ClaimGenerator,
) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData) {
    let log_n_packed_rows = inputs.len().ilog2();
    let log_size = log_n_packed_rows + LOG_N_LANES;
    let (mut trace, mut lookup_data) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
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

    trace
        .par_iter_mut()
        .enumerate()
        .zip(inputs.into_par_iter())
        .zip(lookup_data.par_iter_mut())
        .for_each(|(((row_index, mut row), blake_g_input), lookup_data)| {
            let input_tmp_f72c8_0 = [
                blake_g_input[0],
                blake_g_input[1],
                blake_g_input[2],
                blake_g_input[3],
                blake_g_input[4],
                blake_g_input[5],
            ];
            let input_limb_0_col0 = input_tmp_f72c8_0[0].low().as_m31();
            *row[0] = input_limb_0_col0;
            let input_limb_1_col1 = input_tmp_f72c8_0[0].high().as_m31();
            *row[1] = input_limb_1_col1;
            let input_limb_2_col2 = input_tmp_f72c8_0[1].low().as_m31();
            *row[2] = input_limb_2_col2;
            let input_limb_3_col3 = input_tmp_f72c8_0[1].high().as_m31();
            *row[3] = input_limb_3_col3;
            let input_limb_4_col4 = input_tmp_f72c8_0[2].low().as_m31();
            *row[4] = input_limb_4_col4;
            let input_limb_5_col5 = input_tmp_f72c8_0[2].high().as_m31();
            *row[5] = input_limb_5_col5;
            let input_limb_6_col6 = input_tmp_f72c8_0[3].low().as_m31();
            *row[6] = input_limb_6_col6;
            let input_limb_7_col7 = input_tmp_f72c8_0[3].high().as_m31();
            *row[7] = input_limb_7_col7;
            let input_limb_8_col8 = input_tmp_f72c8_0[4].low().as_m31();
            *row[8] = input_limb_8_col8;
            let input_limb_9_col9 = input_tmp_f72c8_0[4].high().as_m31();
            *row[9] = input_limb_9_col9;
            let input_limb_10_col10 = input_tmp_f72c8_0[5].low().as_m31();
            *row[10] = input_limb_10_col10;
            let input_limb_11_col11 = input_tmp_f72c8_0[5].high().as_m31();
            *row[11] = input_limb_11_col11;

            // Triple Sum 32.

            let triple_sum32_res_tmp_f72c8_1 =
                (((input_tmp_f72c8_0[0]) + (input_tmp_f72c8_0[1])) + (input_tmp_f72c8_0[4]));
            let triple_sum32_res_low_col12 = triple_sum32_res_tmp_f72c8_1.low().as_m31();
            *row[12] = triple_sum32_res_low_col12;
            let triple_sum32_res_high_col13 = triple_sum32_res_tmp_f72c8_1.high().as_m31();
            *row[13] = triple_sum32_res_high_col13;

            // Xor Rot 32 R 16.

            // Split 16 Low Part Size 8.

            let ms_8_bits_tmp_f72c8_4 = ((triple_sum32_res_tmp_f72c8_1.low()) >> (UInt16_8));
            let ms_8_bits_col14 = ms_8_bits_tmp_f72c8_4.as_m31();
            *row[14] = ms_8_bits_col14;

            // Split 16 Low Part Size 8.

            let ms_8_bits_tmp_f72c8_5 = ((triple_sum32_res_tmp_f72c8_1.high()) >> (UInt16_8));
            let ms_8_bits_col15 = ms_8_bits_tmp_f72c8_5.as_m31();
            *row[15] = ms_8_bits_col15;

            // Split 16 Low Part Size 8.

            let ms_8_bits_tmp_f72c8_6 = ((input_tmp_f72c8_0[3].low()) >> (UInt16_8));
            let ms_8_bits_col16 = ms_8_bits_tmp_f72c8_6.as_m31();
            *row[16] = ms_8_bits_col16;

            // Split 16 Low Part Size 8.

            let ms_8_bits_tmp_f72c8_7 = ((input_tmp_f72c8_0[3].high()) >> (UInt16_8));
            let ms_8_bits_col17 = ms_8_bits_tmp_f72c8_7.as_m31();
            *row[17] = ms_8_bits_col17;

            // Bitwise Xor Num Bits 8.

            let xor_tmp_f72c8_8 = ((PackedUInt16::from_m31(
                ((triple_sum32_res_low_col12) - ((ms_8_bits_col14) * (M31_256))),
            )) ^ (PackedUInt16::from_m31(
                ((input_limb_6_col6) - ((ms_8_bits_col16) * (M31_256))),
            )));
            let xor_col18 = xor_tmp_f72c8_8.as_m31();
            *row[18] = xor_col18;
            let verify_bitwise_xor_8_inputs_0 = [
                ((triple_sum32_res_low_col12) - ((ms_8_bits_col14) * (M31_256))),
                ((input_limb_6_col6) - ((ms_8_bits_col16) * (M31_256))),
                xor_col18,
            ]
            .unpack();
            *lookup_data.verify_bitwise_xor_8_0 = [
                ((triple_sum32_res_low_col12) - ((ms_8_bits_col14) * (M31_256))),
                ((input_limb_6_col6) - ((ms_8_bits_col16) * (M31_256))),
                xor_col18,
            ];

            // Bitwise Xor Num Bits 8.

            let xor_tmp_f72c8_9 = ((PackedUInt16::from_m31(ms_8_bits_col14))
                ^ (PackedUInt16::from_m31(ms_8_bits_col16)));
            let xor_col19 = xor_tmp_f72c8_9.as_m31();
            *row[19] = xor_col19;
            let verify_bitwise_xor_8_inputs_1 =
                [ms_8_bits_col14, ms_8_bits_col16, xor_col19].unpack();
            *lookup_data.verify_bitwise_xor_8_1 = [ms_8_bits_col14, ms_8_bits_col16, xor_col19];

            // Bitwise Xor Num Bits 8.

            let xor_tmp_f72c8_10 = ((PackedUInt16::from_m31(
                ((triple_sum32_res_high_col13) - ((ms_8_bits_col15) * (M31_256))),
            )) ^ (PackedUInt16::from_m31(
                ((input_limb_7_col7) - ((ms_8_bits_col17) * (M31_256))),
            )));
            let xor_col20 = xor_tmp_f72c8_10.as_m31();
            *row[20] = xor_col20;
            let verify_bitwise_xor_8_inputs_2 = [
                ((triple_sum32_res_high_col13) - ((ms_8_bits_col15) * (M31_256))),
                ((input_limb_7_col7) - ((ms_8_bits_col17) * (M31_256))),
                xor_col20,
            ]
            .unpack();
            *lookup_data.verify_bitwise_xor_8_2 = [
                ((triple_sum32_res_high_col13) - ((ms_8_bits_col15) * (M31_256))),
                ((input_limb_7_col7) - ((ms_8_bits_col17) * (M31_256))),
                xor_col20,
            ];

            // Bitwise Xor Num Bits 8.

            let xor_tmp_f72c8_11 = ((PackedUInt16::from_m31(ms_8_bits_col15))
                ^ (PackedUInt16::from_m31(ms_8_bits_col17)));
            let xor_col21 = xor_tmp_f72c8_11.as_m31();
            *row[21] = xor_col21;
            let verify_bitwise_xor_8_inputs_3 =
                [ms_8_bits_col15, ms_8_bits_col17, xor_col21].unpack();
            *lookup_data.verify_bitwise_xor_8_3 = [ms_8_bits_col15, ms_8_bits_col17, xor_col21];

            let xor_rot_16_output_tmp_f72c8_12 = PackedUInt32::from_limbs(
                ((xor_col20) + ((xor_col21) * (M31_256))),
                ((xor_col18) + ((xor_col19) * (M31_256))),
            );
            let xor_rot_16_output_tmp_f72c8_12_limb_0 = ((xor_col20) + ((xor_col21) * (M31_256)));
            let xor_rot_16_output_tmp_f72c8_12_limb_1 = ((xor_col18) + ((xor_col19) * (M31_256)));

            // Triple Sum 32.

            let triple_sum32_res_tmp_f72c8_13 =
                (((input_tmp_f72c8_0[2]) + (xor_rot_16_output_tmp_f72c8_12)) + (UInt32_0));
            let triple_sum32_res_low_col22 = triple_sum32_res_tmp_f72c8_13.low().as_m31();
            *row[22] = triple_sum32_res_low_col22;
            let triple_sum32_res_high_col23 = triple_sum32_res_tmp_f72c8_13.high().as_m31();
            *row[23] = triple_sum32_res_high_col23;

            // Xor Rot 32 R 12.

            // Split 16 Low Part Size 12.

            let ms_4_bits_tmp_f72c8_16 = ((input_tmp_f72c8_0[1].low()) >> (UInt16_12));
            let ms_4_bits_col24 = ms_4_bits_tmp_f72c8_16.as_m31();
            *row[24] = ms_4_bits_col24;

            // Split 16 Low Part Size 12.

            let ms_4_bits_tmp_f72c8_17 = ((input_tmp_f72c8_0[1].high()) >> (UInt16_12));
            let ms_4_bits_col25 = ms_4_bits_tmp_f72c8_17.as_m31();
            *row[25] = ms_4_bits_col25;

            // Split 16 Low Part Size 12.

            let ms_4_bits_tmp_f72c8_18 = ((triple_sum32_res_tmp_f72c8_13.low()) >> (UInt16_12));
            let ms_4_bits_col26 = ms_4_bits_tmp_f72c8_18.as_m31();
            *row[26] = ms_4_bits_col26;

            // Split 16 Low Part Size 12.

            let ms_4_bits_tmp_f72c8_19 = ((triple_sum32_res_tmp_f72c8_13.high()) >> (UInt16_12));
            let ms_4_bits_col27 = ms_4_bits_tmp_f72c8_19.as_m31();
            *row[27] = ms_4_bits_col27;

            // Bitwise Xor Num Bits 12.

            let xor_tmp_f72c8_20 = ((PackedUInt16::from_m31(
                ((input_limb_2_col2) - ((ms_4_bits_col24) * (M31_4096))),
            )) ^ (PackedUInt16::from_m31(
                ((triple_sum32_res_low_col22) - ((ms_4_bits_col26) * (M31_4096))),
            )));
            let xor_col28 = xor_tmp_f72c8_20.as_m31();
            *row[28] = xor_col28;
            let verify_bitwise_xor_12_inputs_0 = [
                ((input_limb_2_col2) - ((ms_4_bits_col24) * (M31_4096))),
                ((triple_sum32_res_low_col22) - ((ms_4_bits_col26) * (M31_4096))),
                xor_col28,
            ]
            .unpack();
            *lookup_data.verify_bitwise_xor_12_0 = [
                ((input_limb_2_col2) - ((ms_4_bits_col24) * (M31_4096))),
                ((triple_sum32_res_low_col22) - ((ms_4_bits_col26) * (M31_4096))),
                xor_col28,
            ];

            // Bitwise Xor Num Bits 4.

            let xor_tmp_f72c8_21 = ((PackedUInt16::from_m31(ms_4_bits_col24))
                ^ (PackedUInt16::from_m31(ms_4_bits_col26)));
            let xor_col29 = xor_tmp_f72c8_21.as_m31();
            *row[29] = xor_col29;
            let verify_bitwise_xor_4_inputs_0 =
                [ms_4_bits_col24, ms_4_bits_col26, xor_col29].unpack();
            *lookup_data.verify_bitwise_xor_4_0 = [ms_4_bits_col24, ms_4_bits_col26, xor_col29];

            // Bitwise Xor Num Bits 12.

            let xor_tmp_f72c8_22 = ((PackedUInt16::from_m31(
                ((input_limb_3_col3) - ((ms_4_bits_col25) * (M31_4096))),
            )) ^ (PackedUInt16::from_m31(
                ((triple_sum32_res_high_col23) - ((ms_4_bits_col27) * (M31_4096))),
            )));
            let xor_col30 = xor_tmp_f72c8_22.as_m31();
            *row[30] = xor_col30;
            let verify_bitwise_xor_12_inputs_1 = [
                ((input_limb_3_col3) - ((ms_4_bits_col25) * (M31_4096))),
                ((triple_sum32_res_high_col23) - ((ms_4_bits_col27) * (M31_4096))),
                xor_col30,
            ]
            .unpack();
            *lookup_data.verify_bitwise_xor_12_1 = [
                ((input_limb_3_col3) - ((ms_4_bits_col25) * (M31_4096))),
                ((triple_sum32_res_high_col23) - ((ms_4_bits_col27) * (M31_4096))),
                xor_col30,
            ];

            // Bitwise Xor Num Bits 4.

            let xor_tmp_f72c8_23 = ((PackedUInt16::from_m31(ms_4_bits_col25))
                ^ (PackedUInt16::from_m31(ms_4_bits_col27)));
            let xor_col31 = xor_tmp_f72c8_23.as_m31();
            *row[31] = xor_col31;
            let verify_bitwise_xor_4_inputs_1 =
                [ms_4_bits_col25, ms_4_bits_col27, xor_col31].unpack();
            *lookup_data.verify_bitwise_xor_4_1 = [ms_4_bits_col25, ms_4_bits_col27, xor_col31];

            let xor_rot_12_output_tmp_f72c8_24 = PackedUInt32::from_limbs(
                ((xor_col29) + ((xor_col30) * (M31_16))),
                ((xor_col31) + ((xor_col28) * (M31_16))),
            );
            let xor_rot_12_output_tmp_f72c8_24_limb_0 = ((xor_col29) + ((xor_col30) * (M31_16)));
            let xor_rot_12_output_tmp_f72c8_24_limb_1 = ((xor_col31) + ((xor_col28) * (M31_16)));

            // Triple Sum 32.

            let triple_sum32_res_tmp_f72c8_25 = (((triple_sum32_res_tmp_f72c8_1)
                + (xor_rot_12_output_tmp_f72c8_24))
                + (input_tmp_f72c8_0[5]));
            let triple_sum32_res_low_col32 = triple_sum32_res_tmp_f72c8_25.low().as_m31();
            *row[32] = triple_sum32_res_low_col32;
            let triple_sum32_res_high_col33 = triple_sum32_res_tmp_f72c8_25.high().as_m31();
            *row[33] = triple_sum32_res_high_col33;

            // Xor Rot 32 R 8.

            // Split 16 Low Part Size 8.

            let ms_8_bits_tmp_f72c8_28 = ((triple_sum32_res_tmp_f72c8_25.low()) >> (UInt16_8));
            let ms_8_bits_col34 = ms_8_bits_tmp_f72c8_28.as_m31();
            *row[34] = ms_8_bits_col34;

            // Split 16 Low Part Size 8.

            let ms_8_bits_tmp_f72c8_29 = ((triple_sum32_res_tmp_f72c8_25.high()) >> (UInt16_8));
            let ms_8_bits_col35 = ms_8_bits_tmp_f72c8_29.as_m31();
            *row[35] = ms_8_bits_col35;

            // Split 16 Low Part Size 8.

            let ms_8_bits_tmp_f72c8_30 = ((xor_rot_16_output_tmp_f72c8_12.low()) >> (UInt16_8));
            let ms_8_bits_col36 = ms_8_bits_tmp_f72c8_30.as_m31();
            *row[36] = ms_8_bits_col36;

            // Split 16 Low Part Size 8.

            let ms_8_bits_tmp_f72c8_31 = ((xor_rot_16_output_tmp_f72c8_12.high()) >> (UInt16_8));
            let ms_8_bits_col37 = ms_8_bits_tmp_f72c8_31.as_m31();
            *row[37] = ms_8_bits_col37;

            // Bitwise Xor Num Bits 8.

            let xor_tmp_f72c8_32 = ((PackedUInt16::from_m31(
                ((triple_sum32_res_low_col32) - ((ms_8_bits_col34) * (M31_256))),
            )) ^ (PackedUInt16::from_m31(
                ((xor_rot_16_output_tmp_f72c8_12_limb_0) - ((ms_8_bits_col36) * (M31_256))),
            )));
            let xor_col38 = xor_tmp_f72c8_32.as_m31();
            *row[38] = xor_col38;
            let verify_bitwise_xor_8_inputs_4 = [
                ((triple_sum32_res_low_col32) - ((ms_8_bits_col34) * (M31_256))),
                ((xor_rot_16_output_tmp_f72c8_12_limb_0) - ((ms_8_bits_col36) * (M31_256))),
                xor_col38,
            ]
            .unpack();
            *lookup_data.verify_bitwise_xor_8_4 = [
                ((triple_sum32_res_low_col32) - ((ms_8_bits_col34) * (M31_256))),
                ((xor_rot_16_output_tmp_f72c8_12_limb_0) - ((ms_8_bits_col36) * (M31_256))),
                xor_col38,
            ];

            // Bitwise Xor Num Bits 8.

            let xor_tmp_f72c8_33 = ((PackedUInt16::from_m31(ms_8_bits_col34))
                ^ (PackedUInt16::from_m31(ms_8_bits_col36)));
            let xor_col39 = xor_tmp_f72c8_33.as_m31();
            *row[39] = xor_col39;
            let verify_bitwise_xor_8_inputs_5 =
                [ms_8_bits_col34, ms_8_bits_col36, xor_col39].unpack();
            *lookup_data.verify_bitwise_xor_8_5 = [ms_8_bits_col34, ms_8_bits_col36, xor_col39];

            // Bitwise Xor Num Bits 8.

            let xor_tmp_f72c8_34 = ((PackedUInt16::from_m31(
                ((triple_sum32_res_high_col33) - ((ms_8_bits_col35) * (M31_256))),
            )) ^ (PackedUInt16::from_m31(
                ((xor_rot_16_output_tmp_f72c8_12_limb_1) - ((ms_8_bits_col37) * (M31_256))),
            )));
            let xor_col40 = xor_tmp_f72c8_34.as_m31();
            *row[40] = xor_col40;
            let verify_bitwise_xor_8_inputs_6 = [
                ((triple_sum32_res_high_col33) - ((ms_8_bits_col35) * (M31_256))),
                ((xor_rot_16_output_tmp_f72c8_12_limb_1) - ((ms_8_bits_col37) * (M31_256))),
                xor_col40,
            ]
            .unpack();
            *lookup_data.verify_bitwise_xor_8_6 = [
                ((triple_sum32_res_high_col33) - ((ms_8_bits_col35) * (M31_256))),
                ((xor_rot_16_output_tmp_f72c8_12_limb_1) - ((ms_8_bits_col37) * (M31_256))),
                xor_col40,
            ];

            // Bitwise Xor Num Bits 8.

            let xor_tmp_f72c8_35 = ((PackedUInt16::from_m31(ms_8_bits_col35))
                ^ (PackedUInt16::from_m31(ms_8_bits_col37)));
            let xor_col41 = xor_tmp_f72c8_35.as_m31();
            *row[41] = xor_col41;
            let verify_bitwise_xor_8_inputs_7 =
                [ms_8_bits_col35, ms_8_bits_col37, xor_col41].unpack();
            *lookup_data.verify_bitwise_xor_8_7 = [ms_8_bits_col35, ms_8_bits_col37, xor_col41];

            let xor_rot_8_output_tmp_f72c8_36 = PackedUInt32::from_limbs(
                ((xor_col39) + ((xor_col40) * (M31_256))),
                ((xor_col41) + ((xor_col38) * (M31_256))),
            );
            let xor_rot_8_output_tmp_f72c8_36_limb_0 = ((xor_col39) + ((xor_col40) * (M31_256)));
            let xor_rot_8_output_tmp_f72c8_36_limb_1 = ((xor_col41) + ((xor_col38) * (M31_256)));

            // Triple Sum 32.

            let triple_sum32_res_tmp_f72c8_37 =
                (((triple_sum32_res_tmp_f72c8_13) + (xor_rot_8_output_tmp_f72c8_36)) + (UInt32_0));
            let triple_sum32_res_low_col42 = triple_sum32_res_tmp_f72c8_37.low().as_m31();
            *row[42] = triple_sum32_res_low_col42;
            let triple_sum32_res_high_col43 = triple_sum32_res_tmp_f72c8_37.high().as_m31();
            *row[43] = triple_sum32_res_high_col43;

            // Xor Rot 32 R 7.

            // Split 16 Low Part Size 7.

            let ms_9_bits_tmp_f72c8_40 = ((xor_rot_12_output_tmp_f72c8_24.low()) >> (UInt16_7));
            let ms_9_bits_col44 = ms_9_bits_tmp_f72c8_40.as_m31();
            *row[44] = ms_9_bits_col44;

            // Split 16 Low Part Size 7.

            let ms_9_bits_tmp_f72c8_41 = ((xor_rot_12_output_tmp_f72c8_24.high()) >> (UInt16_7));
            let ms_9_bits_col45 = ms_9_bits_tmp_f72c8_41.as_m31();
            *row[45] = ms_9_bits_col45;

            // Split 16 Low Part Size 7.

            let ms_9_bits_tmp_f72c8_42 = ((triple_sum32_res_tmp_f72c8_37.low()) >> (UInt16_7));
            let ms_9_bits_col46 = ms_9_bits_tmp_f72c8_42.as_m31();
            *row[46] = ms_9_bits_col46;

            // Split 16 Low Part Size 7.

            let ms_9_bits_tmp_f72c8_43 = ((triple_sum32_res_tmp_f72c8_37.high()) >> (UInt16_7));
            let ms_9_bits_col47 = ms_9_bits_tmp_f72c8_43.as_m31();
            *row[47] = ms_9_bits_col47;

            // Bitwise Xor Num Bits 7.

            let xor_tmp_f72c8_44 = ((PackedUInt16::from_m31(
                ((xor_rot_12_output_tmp_f72c8_24_limb_0) - ((ms_9_bits_col44) * (M31_128))),
            )) ^ (PackedUInt16::from_m31(
                ((triple_sum32_res_low_col42) - ((ms_9_bits_col46) * (M31_128))),
            )));
            let xor_col48 = xor_tmp_f72c8_44.as_m31();
            *row[48] = xor_col48;
            let verify_bitwise_xor_7_inputs_0 = [
                ((xor_rot_12_output_tmp_f72c8_24_limb_0) - ((ms_9_bits_col44) * (M31_128))),
                ((triple_sum32_res_low_col42) - ((ms_9_bits_col46) * (M31_128))),
                xor_col48,
            ]
            .unpack();
            *lookup_data.verify_bitwise_xor_7_0 = [
                ((xor_rot_12_output_tmp_f72c8_24_limb_0) - ((ms_9_bits_col44) * (M31_128))),
                ((triple_sum32_res_low_col42) - ((ms_9_bits_col46) * (M31_128))),
                xor_col48,
            ];

            // Bitwise Xor Num Bits 9.

            let xor_tmp_f72c8_45 = ((PackedUInt16::from_m31(ms_9_bits_col44))
                ^ (PackedUInt16::from_m31(ms_9_bits_col46)));
            let xor_col49 = xor_tmp_f72c8_45.as_m31();
            *row[49] = xor_col49;
            let verify_bitwise_xor_9_inputs_0 =
                [ms_9_bits_col44, ms_9_bits_col46, xor_col49].unpack();
            *lookup_data.verify_bitwise_xor_9_0 = [ms_9_bits_col44, ms_9_bits_col46, xor_col49];

            // Bitwise Xor Num Bits 7.

            let xor_tmp_f72c8_46 = ((PackedUInt16::from_m31(
                ((xor_rot_12_output_tmp_f72c8_24_limb_1) - ((ms_9_bits_col45) * (M31_128))),
            )) ^ (PackedUInt16::from_m31(
                ((triple_sum32_res_high_col43) - ((ms_9_bits_col47) * (M31_128))),
            )));
            let xor_col50 = xor_tmp_f72c8_46.as_m31();
            *row[50] = xor_col50;
            let verify_bitwise_xor_7_inputs_1 = [
                ((xor_rot_12_output_tmp_f72c8_24_limb_1) - ((ms_9_bits_col45) * (M31_128))),
                ((triple_sum32_res_high_col43) - ((ms_9_bits_col47) * (M31_128))),
                xor_col50,
            ]
            .unpack();
            *lookup_data.verify_bitwise_xor_7_1 = [
                ((xor_rot_12_output_tmp_f72c8_24_limb_1) - ((ms_9_bits_col45) * (M31_128))),
                ((triple_sum32_res_high_col43) - ((ms_9_bits_col47) * (M31_128))),
                xor_col50,
            ];

            // Bitwise Xor Num Bits 9.

            let xor_tmp_f72c8_47 = ((PackedUInt16::from_m31(ms_9_bits_col45))
                ^ (PackedUInt16::from_m31(ms_9_bits_col47)));
            let xor_col51 = xor_tmp_f72c8_47.as_m31();
            *row[51] = xor_col51;
            let verify_bitwise_xor_9_inputs_1 =
                [ms_9_bits_col45, ms_9_bits_col47, xor_col51].unpack();
            *lookup_data.verify_bitwise_xor_9_1 = [ms_9_bits_col45, ms_9_bits_col47, xor_col51];

            let xor_rot_7_output_tmp_f72c8_48 = PackedUInt32::from_limbs(
                ((xor_col49) + ((xor_col50) * (M31_512))),
                ((xor_col51) + ((xor_col48) * (M31_512))),
            );
            let xor_rot_7_output_tmp_f72c8_48_limb_0 = ((xor_col49) + ((xor_col50) * (M31_512)));
            let xor_rot_7_output_tmp_f72c8_48_limb_1 = ((xor_col51) + ((xor_col48) * (M31_512)));

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
                triple_sum32_res_low_col32,
                triple_sum32_res_high_col33,
                xor_rot_7_output_tmp_f72c8_48_limb_0,
                xor_rot_7_output_tmp_f72c8_48_limb_1,
                triple_sum32_res_low_col42,
                triple_sum32_res_high_col43,
                xor_rot_8_output_tmp_f72c8_36_limb_0,
                xor_rot_8_output_tmp_f72c8_36_limb_1,
            ];

            // Add sub-components inputs.
            verify_bitwise_xor_8_state.add_inputs(&verify_bitwise_xor_8_inputs_0);
            verify_bitwise_xor_8_state.add_inputs(&verify_bitwise_xor_8_inputs_1);
            verify_bitwise_xor_8_state.add_inputs(&verify_bitwise_xor_8_inputs_2);
            verify_bitwise_xor_8_state.add_inputs(&verify_bitwise_xor_8_inputs_3);
            verify_bitwise_xor_12_state.add_inputs(&verify_bitwise_xor_12_inputs_0);
            verify_bitwise_xor_4_state.add_inputs(&verify_bitwise_xor_4_inputs_0);
            verify_bitwise_xor_12_state.add_inputs(&verify_bitwise_xor_12_inputs_1);
            verify_bitwise_xor_4_state.add_inputs(&verify_bitwise_xor_4_inputs_1);
            verify_bitwise_xor_8_state.add_inputs(&verify_bitwise_xor_8_inputs_4);
            verify_bitwise_xor_8_state.add_inputs(&verify_bitwise_xor_8_inputs_5);
            verify_bitwise_xor_8_state.add_inputs(&verify_bitwise_xor_8_inputs_6);
            verify_bitwise_xor_8_state.add_inputs(&verify_bitwise_xor_8_inputs_7);
            verify_bitwise_xor_7_state.add_inputs(&verify_bitwise_xor_7_inputs_0);
            verify_bitwise_xor_9_state.add_inputs(&verify_bitwise_xor_9_inputs_0);
            verify_bitwise_xor_7_state.add_inputs(&verify_bitwise_xor_7_inputs_1);
            verify_bitwise_xor_9_state.add_inputs(&verify_bitwise_xor_9_inputs_1);
        });

    (trace, lookup_data)
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
    verify_bitwise_xor_8_4: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_5: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_6: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_7: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_0: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_1: Vec<[PackedM31; 3]>,
}

pub struct InteractionClaimGenerator {
    log_size: u32,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        blake_g: &relations::BlakeG,
        verify_bitwise_xor_12: &relations::VerifyBitwiseXor_12,
        verify_bitwise_xor_4: &relations::VerifyBitwiseXor_4,
        verify_bitwise_xor_7: &relations::VerifyBitwiseXor_7,
        verify_bitwise_xor_8: &relations::VerifyBitwiseXor_8,
        verify_bitwise_xor_9: &relations::VerifyBitwiseXor_9,
    ) -> InteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_8_0,
            &self.lookup_data.verify_bitwise_xor_8_1,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_8.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_8.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_8_2,
            &self.lookup_data.verify_bitwise_xor_8_3,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_8.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_8.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_12_0,
            &self.lookup_data.verify_bitwise_xor_4_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_12.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_4.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_12_1,
            &self.lookup_data.verify_bitwise_xor_4_1,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_12.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_4.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_8_4,
            &self.lookup_data.verify_bitwise_xor_8_5,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_8.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_8.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_8_6,
            &self.lookup_data.verify_bitwise_xor_8_7,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_8.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_8.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_7_0,
            &self.lookup_data.verify_bitwise_xor_9_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_7.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_7_1,
            &self.lookup_data.verify_bitwise_xor_9_1,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_7.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        for (i, values) in self.lookup_data.blake_g_0.iter().enumerate() {
            let denom = blake_g.combine(values);
            col_gen.write_frac(i, -PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
