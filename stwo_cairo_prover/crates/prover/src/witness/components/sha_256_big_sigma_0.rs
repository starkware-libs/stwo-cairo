// AIR version 52ac7695-dirty
#![allow(unused_parens)]
use cairo_air::components::sha_256_big_sigma_0::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{
    sha_256_big_sigma_0_o_0, sha_256_big_sigma_0_o_1, verify_bitwise_and_8, verify_bitwise_xor_8,
};
use crate::witness::prelude::*;

pub type PackedInputType = PackedUInt32;

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
        sha_256_big_sigma_0_o_0_state: &sha_256_big_sigma_0_o_0::ClaimGenerator,
        sha_256_big_sigma_0_o_1_state: &sha_256_big_sigma_0_o_1::ClaimGenerator,
        verify_bitwise_and_8_state: &verify_bitwise_and_8::ClaimGenerator,
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
            sha_256_big_sigma_0_o_0_state,
            sha_256_big_sigma_0_o_1_state,
            verify_bitwise_and_8_state,
            verify_bitwise_xor_8_state,
        );
        sub_component_inputs
            .verify_bitwise_and_8
            .iter()
            .for_each(|inputs| {
                verify_bitwise_and_8_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .sha_256_big_sigma_0_o_0
            .iter()
            .for_each(|inputs| {
                sha_256_big_sigma_0_o_0_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .sha_256_big_sigma_0_o_1
            .iter()
            .for_each(|inputs| {
                sha_256_big_sigma_0_o_1_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .verify_bitwise_xor_8
            .iter()
            .for_each(|inputs| {
                verify_bitwise_xor_8_state.add_packed_inputs(inputs);
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
    verify_bitwise_and_8: [Vec<verify_bitwise_and_8::PackedInputType>; 12],
    sha_256_big_sigma_0_o_0: [Vec<sha_256_big_sigma_0_o_0::PackedInputType>; 1],
    sha_256_big_sigma_0_o_1: [Vec<sha_256_big_sigma_0_o_1::PackedInputType>; 1],
    verify_bitwise_xor_8: [Vec<verify_bitwise_xor_8::PackedInputType>; 4],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    n_rows: usize,
    sha_256_big_sigma_0_o_0_state: &sha_256_big_sigma_0_o_0::ClaimGenerator,
    sha_256_big_sigma_0_o_1_state: &sha_256_big_sigma_0_o_1::ClaimGenerator,
    verify_bitwise_and_8_state: &verify_bitwise_and_8::ClaimGenerator,
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

    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_124 = PackedM31::broadcast(M31::from(124));
    let M31_131 = PackedM31::broadcast(M31::from(131));
    let M31_15 = PackedM31::broadcast(M31::from(15));
    let M31_240 = PackedM31::broadcast(M31::from(240));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let UInt16_0 = PackedUInt16::broadcast(UInt16::from(0));
    let UInt16_124 = PackedUInt16::broadcast(UInt16::from(124));
    let UInt16_131 = PackedUInt16::broadcast(UInt16::from(131));
    let UInt16_15 = PackedUInt16::broadcast(UInt16::from(15));
    let UInt16_240 = PackedUInt16::broadcast(UInt16::from(240));
    let UInt16_28702 = PackedUInt16::broadcast(UInt16::from(28702));
    let UInt16_35873 = PackedUInt16::broadcast(UInt16::from(35873));
    let UInt16_8 = PackedUInt16::broadcast(UInt16::from(8));
    let UInt16_960 = PackedUInt16::broadcast(UInt16::from(960));
    let UInt32_10 = PackedUInt32::broadcast(UInt32::from(10));
    let UInt32_13 = PackedUInt32::broadcast(UInt32::from(13));
    let UInt32_19 = PackedUInt32::broadcast(UInt32::from(19));
    let UInt32_2 = PackedUInt32::broadcast(UInt32::from(2));
    let UInt32_22 = PackedUInt32::broadcast(UInt32::from(22));
    let UInt32_30 = PackedUInt32::broadcast(UInt32::from(30));
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
            |(
                row_index,
                (mut row, lookup_data, sub_component_inputs, sha_256_big_sigma_0_input),
            )| {
                let input_limb_0_col0 = sha_256_big_sigma_0_input.low().as_m31();
                *row[0] = input_limb_0_col0;
                let input_limb_1_col1 = sha_256_big_sigma_0_input.high().as_m31();
                *row[1] = input_limb_1_col1;

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_e3d2a_0 = ((sha_256_big_sigma_0_input.low()) >> (UInt16_8));
                let ms_8_bits_col2 = ms_8_bits_tmp_e3d2a_0.as_m31();
                *row[2] = ms_8_bits_col2;
                let split_16_low_part_size_8_output_tmp_e3d2a_1 = [
                    ((input_limb_0_col0) - ((ms_8_bits_col2) * (M31_256))),
                    ms_8_bits_col2,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_e3d2a_2 = ((sha_256_big_sigma_0_input.high()) >> (UInt16_8));
                let ms_8_bits_col3 = ms_8_bits_tmp_e3d2a_2.as_m31();
                *row[3] = ms_8_bits_col3;
                let split_16_low_part_size_8_output_tmp_e3d2a_3 = [
                    ((input_limb_1_col1) - ((ms_8_bits_col3) * (M31_256))),
                    ms_8_bits_col3,
                ];

                // Bitwise And Num Bits 8.

                let and_tmp_e3d2a_4 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_e3d2a_1[0]))
                        & (UInt16_131));
                let and_col4 = and_tmp_e3d2a_4.as_m31();
                *row[4] = and_col4;
                *sub_component_inputs.verify_bitwise_and_8[0] = [
                    split_16_low_part_size_8_output_tmp_e3d2a_1[0],
                    M31_131,
                    and_col4,
                ];
                *lookup_data.verify_bitwise_and_8_0 = [
                    split_16_low_part_size_8_output_tmp_e3d2a_1[0],
                    M31_131,
                    and_col4,
                ];

                // Bitwise And Num Bits 8.

                let and_tmp_e3d2a_6 = ((PackedUInt16::from_m31(ms_8_bits_col2)) & (UInt16_15));
                let and_col5 = and_tmp_e3d2a_6.as_m31();
                *row[5] = and_col5;
                *sub_component_inputs.verify_bitwise_and_8[1] = [ms_8_bits_col2, M31_15, and_col5];
                *lookup_data.verify_bitwise_and_8_1 = [ms_8_bits_col2, M31_15, and_col5];

                let l0_col6 = ((and_col4) + ((and_col5) * (M31_256)));
                *row[6] = l0_col6;

                // Bitwise And Num Bits 8.

                let and_tmp_e3d2a_8 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_e3d2a_1[0]))
                        & (UInt16_124));
                let and_col7 = and_tmp_e3d2a_8.as_m31();
                *row[7] = and_col7;
                *sub_component_inputs.verify_bitwise_and_8[2] = [
                    split_16_low_part_size_8_output_tmp_e3d2a_1[0],
                    M31_124,
                    and_col7,
                ];
                *lookup_data.verify_bitwise_and_8_2 = [
                    split_16_low_part_size_8_output_tmp_e3d2a_1[0],
                    M31_124,
                    and_col7,
                ];

                // Bitwise And Num Bits 8.

                let and_tmp_e3d2a_10 = ((PackedUInt16::from_m31(ms_8_bits_col2)) & (UInt16_0));
                let and_col8 = and_tmp_e3d2a_10.as_m31();
                *row[8] = and_col8;
                *sub_component_inputs.verify_bitwise_and_8[3] = [ms_8_bits_col2, M31_0, and_col8];
                *lookup_data.verify_bitwise_and_8_3 = [ms_8_bits_col2, M31_0, and_col8];

                let l1_col9 = ((and_col7) + ((and_col8) * (M31_256)));
                *row[9] = l1_col9;

                // Bitwise And Num Bits 8.

                let and_tmp_e3d2a_12 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_e3d2a_1[0]))
                        & (UInt16_0));
                let and_col10 = and_tmp_e3d2a_12.as_m31();
                *row[10] = and_col10;
                *sub_component_inputs.verify_bitwise_and_8[4] = [
                    split_16_low_part_size_8_output_tmp_e3d2a_1[0],
                    M31_0,
                    and_col10,
                ];
                *lookup_data.verify_bitwise_and_8_4 = [
                    split_16_low_part_size_8_output_tmp_e3d2a_1[0],
                    M31_0,
                    and_col10,
                ];

                // Bitwise And Num Bits 8.

                let and_tmp_e3d2a_14 = ((PackedUInt16::from_m31(ms_8_bits_col2)) & (UInt16_240));
                let and_col11 = and_tmp_e3d2a_14.as_m31();
                *row[11] = and_col11;
                *sub_component_inputs.verify_bitwise_and_8[5] =
                    [ms_8_bits_col2, M31_240, and_col11];
                *lookup_data.verify_bitwise_and_8_5 = [ms_8_bits_col2, M31_240, and_col11];

                let l2_col12 = ((and_col10) + ((and_col11) * (M31_256)));
                *row[12] = l2_col12;

                // Bitwise And Num Bits 8.

                let and_tmp_e3d2a_16 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_e3d2a_3[0]))
                        & (UInt16_124));
                let and_col13 = and_tmp_e3d2a_16.as_m31();
                *row[13] = and_col13;
                *sub_component_inputs.verify_bitwise_and_8[6] = [
                    split_16_low_part_size_8_output_tmp_e3d2a_3[0],
                    M31_124,
                    and_col13,
                ];
                *lookup_data.verify_bitwise_and_8_6 = [
                    split_16_low_part_size_8_output_tmp_e3d2a_3[0],
                    M31_124,
                    and_col13,
                ];

                // Bitwise And Num Bits 8.

                let and_tmp_e3d2a_18 = ((PackedUInt16::from_m31(ms_8_bits_col3)) & (UInt16_0));
                let and_col14 = and_tmp_e3d2a_18.as_m31();
                *row[14] = and_col14;
                *sub_component_inputs.verify_bitwise_and_8[7] = [ms_8_bits_col3, M31_0, and_col14];
                *lookup_data.verify_bitwise_and_8_7 = [ms_8_bits_col3, M31_0, and_col14];

                let h0_col15 = ((and_col13) + ((and_col14) * (M31_256)));
                *row[15] = h0_col15;

                // Bitwise And Num Bits 8.

                let and_tmp_e3d2a_20 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_e3d2a_3[0]))
                        & (UInt16_0));
                let and_col16 = and_tmp_e3d2a_20.as_m31();
                *row[16] = and_col16;
                *sub_component_inputs.verify_bitwise_and_8[8] = [
                    split_16_low_part_size_8_output_tmp_e3d2a_3[0],
                    M31_0,
                    and_col16,
                ];
                *lookup_data.verify_bitwise_and_8_8 = [
                    split_16_low_part_size_8_output_tmp_e3d2a_3[0],
                    M31_0,
                    and_col16,
                ];

                // Bitwise And Num Bits 8.

                let and_tmp_e3d2a_22 = ((PackedUInt16::from_m31(ms_8_bits_col3)) & (UInt16_240));
                let and_col17 = and_tmp_e3d2a_22.as_m31();
                *row[17] = and_col17;
                *sub_component_inputs.verify_bitwise_and_8[9] =
                    [ms_8_bits_col3, M31_240, and_col17];
                *lookup_data.verify_bitwise_and_8_9 = [ms_8_bits_col3, M31_240, and_col17];

                let h1_col18 = ((and_col16) + ((and_col17) * (M31_256)));
                *row[18] = h1_col18;

                // Bitwise And Num Bits 8.

                let and_tmp_e3d2a_24 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_e3d2a_3[0]))
                        & (UInt16_131));
                let and_col19 = and_tmp_e3d2a_24.as_m31();
                *row[19] = and_col19;
                *sub_component_inputs.verify_bitwise_and_8[10] = [
                    split_16_low_part_size_8_output_tmp_e3d2a_3[0],
                    M31_131,
                    and_col19,
                ];
                *lookup_data.verify_bitwise_and_8_10 = [
                    split_16_low_part_size_8_output_tmp_e3d2a_3[0],
                    M31_131,
                    and_col19,
                ];

                // Bitwise And Num Bits 8.

                let and_tmp_e3d2a_26 = ((PackedUInt16::from_m31(ms_8_bits_col3)) & (UInt16_15));
                let and_col20 = and_tmp_e3d2a_26.as_m31();
                *row[20] = and_col20;
                *sub_component_inputs.verify_bitwise_and_8[11] =
                    [ms_8_bits_col3, M31_15, and_col20];
                *lookup_data.verify_bitwise_and_8_11 = [ms_8_bits_col3, M31_15, and_col20];

                let h2_col21 = ((and_col19) + ((and_col20) * (M31_256)));
                *row[21] = h2_col21;
                let sigma_rotated0_tmp_e3d2a_28 =
                    (((((PackedUInt32::from_limbs([l0_col6, ((h0_col15) + (h1_col18))]))
                        >> (UInt32_2))
                        | ((PackedUInt32::from_limbs([l0_col6, ((h0_col15) + (h1_col18))]))
                            << (UInt32_30)))
                        ^ (((PackedUInt32::from_limbs([l0_col6, ((h0_col15) + (h1_col18))]))
                            >> (UInt32_13))
                            | ((PackedUInt32::from_limbs([l0_col6, ((h0_col15) + (h1_col18))]))
                                << (UInt32_19))))
                        ^ (((PackedUInt32::from_limbs([l0_col6, ((h0_col15) + (h1_col18))]))
                            >> (UInt32_22))
                            | ((PackedUInt32::from_limbs([l0_col6, ((h0_col15) + (h1_col18))]))
                                << (UInt32_10))));
                let sigma_rotated1_tmp_e3d2a_29 =
                    (((((PackedUInt32::from_limbs([((l1_col9) + (l2_col12)), h2_col21]))
                        >> (UInt32_2))
                        | ((PackedUInt32::from_limbs([((l1_col9) + (l2_col12)), h2_col21]))
                            << (UInt32_30)))
                        ^ (((PackedUInt32::from_limbs([((l1_col9) + (l2_col12)), h2_col21]))
                            >> (UInt32_13))
                            | ((PackedUInt32::from_limbs([((l1_col9) + (l2_col12)), h2_col21]))
                                << (UInt32_19))))
                        ^ (((PackedUInt32::from_limbs([((l1_col9) + (l2_col12)), h2_col21]))
                            >> (UInt32_22))
                            | ((PackedUInt32::from_limbs([((l1_col9) + (l2_col12)), h2_col21]))
                                << (UInt32_10))));
                let sigma_O0_L_tmp_e3d2a_30 = ((sigma_rotated0_tmp_e3d2a_28.low()) & (UInt16_960));
                let sigma_O0_L_col22 = sigma_O0_L_tmp_e3d2a_30.as_m31();
                *row[22] = sigma_O0_L_col22;
                let sigma_O0_H_tmp_e3d2a_31 =
                    ((sigma_rotated0_tmp_e3d2a_28.high()) & (UInt16_28702));
                let sigma_O0_H_col23 = sigma_O0_H_tmp_e3d2a_31.as_m31();
                *row[23] = sigma_O0_H_col23;
                let sigma_O1_L_tmp_e3d2a_32 =
                    ((sigma_rotated1_tmp_e3d2a_29.low()) & (UInt16_28702));
                let sigma_O1_L_col24 = sigma_O1_L_tmp_e3d2a_32.as_m31();
                *row[24] = sigma_O1_L_col24;
                let sigma_O1_H_tmp_e3d2a_33 = ((sigma_rotated1_tmp_e3d2a_29.high()) & (UInt16_960));
                let sigma_O1_H_col25 = sigma_O1_H_tmp_e3d2a_33.as_m31();
                *row[25] = sigma_O1_H_col25;
                let sigma_O2_L_tmp_e3d2a_34 =
                    ((sigma_rotated0_tmp_e3d2a_28.low()) & (UInt16_35873));
                let sigma_O2_L_col26 = sigma_O2_L_tmp_e3d2a_34.as_m31();
                *row[26] = sigma_O2_L_col26;
                let sigma_O2_H_tmp_e3d2a_35 =
                    ((sigma_rotated0_tmp_e3d2a_28.high()) & (UInt16_35873));
                let sigma_O2_H_col27 = sigma_O2_H_tmp_e3d2a_35.as_m31();
                *row[27] = sigma_O2_H_col27;
                let sigma_O2_prime_L_tmp_e3d2a_36 =
                    ((sigma_rotated1_tmp_e3d2a_29.low()) & (UInt16_35873));
                let sigma_O2_prime_L_col28 = sigma_O2_prime_L_tmp_e3d2a_36.as_m31();
                *row[28] = sigma_O2_prime_L_col28;
                let sigma_O2_prime_H_tmp_e3d2a_37 =
                    ((sigma_rotated1_tmp_e3d2a_29.high()) & (UInt16_35873));
                let sigma_O2_prime_H_col29 = sigma_O2_prime_H_tmp_e3d2a_37.as_m31();
                *row[29] = sigma_O2_prime_H_col29;
                *sub_component_inputs.sha_256_big_sigma_0_o_0[0] = [
                    l0_col6,
                    ((h0_col15) + (h1_col18)),
                    sigma_O0_L_col22,
                    sigma_O0_H_col23,
                    sigma_O2_L_col26,
                    sigma_O2_H_col27,
                ];
                *lookup_data.sha_256_big_sigma_0_o_0_0 = [
                    l0_col6,
                    ((h0_col15) + (h1_col18)),
                    sigma_O0_L_col22,
                    sigma_O0_H_col23,
                    sigma_O2_L_col26,
                    sigma_O2_H_col27,
                ];
                *sub_component_inputs.sha_256_big_sigma_0_o_1[0] = [
                    ((l1_col9) + (l2_col12)),
                    h2_col21,
                    sigma_O1_L_col24,
                    sigma_O1_H_col25,
                    sigma_O2_prime_L_col28,
                    sigma_O2_prime_H_col29,
                ];
                *lookup_data.sha_256_big_sigma_0_o_1_0 = [
                    ((l1_col9) + (l2_col12)),
                    h2_col21,
                    sigma_O1_L_col24,
                    sigma_O1_H_col25,
                    sigma_O2_prime_L_col28,
                    sigma_O2_prime_H_col29,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_e3d2a_38 = ((sigma_O2_prime_L_tmp_e3d2a_36) >> (UInt16_8));
                let ms_8_bits_col30 = ms_8_bits_tmp_e3d2a_38.as_m31();
                *row[30] = ms_8_bits_col30;
                let split_16_low_part_size_8_output_tmp_e3d2a_39 = [
                    ((sigma_O2_prime_L_col28) - ((ms_8_bits_col30) * (M31_256))),
                    ms_8_bits_col30,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_e3d2a_40 = ((sigma_O2_L_tmp_e3d2a_34) >> (UInt16_8));
                let ms_8_bits_col31 = ms_8_bits_tmp_e3d2a_40.as_m31();
                *row[31] = ms_8_bits_col31;
                let split_16_low_part_size_8_output_tmp_e3d2a_41 = [
                    ((sigma_O2_L_col26) - ((ms_8_bits_col31) * (M31_256))),
                    ms_8_bits_col31,
                ];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_e3d2a_42 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_e3d2a_39[0]))
                        ^ (PackedUInt16::from_m31(
                            split_16_low_part_size_8_output_tmp_e3d2a_41[0],
                        )));
                let xor_col32 = xor_tmp_e3d2a_42.as_m31();
                *row[32] = xor_col32;
                *sub_component_inputs.verify_bitwise_xor_8[0] = [
                    split_16_low_part_size_8_output_tmp_e3d2a_39[0],
                    split_16_low_part_size_8_output_tmp_e3d2a_41[0],
                    xor_col32,
                ];
                *lookup_data.verify_bitwise_xor_8_0 = [
                    split_16_low_part_size_8_output_tmp_e3d2a_39[0],
                    split_16_low_part_size_8_output_tmp_e3d2a_41[0],
                    xor_col32,
                ];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_e3d2a_44 = ((PackedUInt16::from_m31(ms_8_bits_col30))
                    ^ (PackedUInt16::from_m31(ms_8_bits_col31)));
                let xor_col33 = xor_tmp_e3d2a_44.as_m31();
                *row[33] = xor_col33;
                *sub_component_inputs.verify_bitwise_xor_8[1] =
                    [ms_8_bits_col30, ms_8_bits_col31, xor_col33];
                *lookup_data.verify_bitwise_xor_8_1 = [ms_8_bits_col30, ms_8_bits_col31, xor_col33];

                let output2l_col34 = ((xor_col32) + ((xor_col33) * (M31_256)));
                *row[34] = output2l_col34;

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_e3d2a_46 = ((sigma_O2_prime_H_tmp_e3d2a_37) >> (UInt16_8));
                let ms_8_bits_col35 = ms_8_bits_tmp_e3d2a_46.as_m31();
                *row[35] = ms_8_bits_col35;
                let split_16_low_part_size_8_output_tmp_e3d2a_47 = [
                    ((sigma_O2_prime_H_col29) - ((ms_8_bits_col35) * (M31_256))),
                    ms_8_bits_col35,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_e3d2a_48 = ((sigma_O2_H_tmp_e3d2a_35) >> (UInt16_8));
                let ms_8_bits_col36 = ms_8_bits_tmp_e3d2a_48.as_m31();
                *row[36] = ms_8_bits_col36;
                let split_16_low_part_size_8_output_tmp_e3d2a_49 = [
                    ((sigma_O2_H_col27) - ((ms_8_bits_col36) * (M31_256))),
                    ms_8_bits_col36,
                ];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_e3d2a_50 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_e3d2a_47[0]))
                        ^ (PackedUInt16::from_m31(
                            split_16_low_part_size_8_output_tmp_e3d2a_49[0],
                        )));
                let xor_col37 = xor_tmp_e3d2a_50.as_m31();
                *row[37] = xor_col37;
                *sub_component_inputs.verify_bitwise_xor_8[2] = [
                    split_16_low_part_size_8_output_tmp_e3d2a_47[0],
                    split_16_low_part_size_8_output_tmp_e3d2a_49[0],
                    xor_col37,
                ];
                *lookup_data.verify_bitwise_xor_8_2 = [
                    split_16_low_part_size_8_output_tmp_e3d2a_47[0],
                    split_16_low_part_size_8_output_tmp_e3d2a_49[0],
                    xor_col37,
                ];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_e3d2a_52 = ((PackedUInt16::from_m31(ms_8_bits_col35))
                    ^ (PackedUInt16::from_m31(ms_8_bits_col36)));
                let xor_col38 = xor_tmp_e3d2a_52.as_m31();
                *row[38] = xor_col38;
                *sub_component_inputs.verify_bitwise_xor_8[3] =
                    [ms_8_bits_col35, ms_8_bits_col36, xor_col38];
                *lookup_data.verify_bitwise_xor_8_3 = [ms_8_bits_col35, ms_8_bits_col36, xor_col38];

                let output2h_col39 = ((xor_col37) + ((xor_col38) * (M31_256)));
                *row[39] = output2h_col39;
                let output_low_tmp_e3d2a_54 = (((sigma_O0_L_tmp_e3d2a_30)
                    + (sigma_O1_L_tmp_e3d2a_32))
                    + (PackedUInt16::from_m31(output2l_col34)));
                let output_low_col40 = output_low_tmp_e3d2a_54.as_m31();
                *row[40] = output_low_col40;
                let output_high_tmp_e3d2a_55 = (((sigma_O0_H_tmp_e3d2a_31)
                    + (sigma_O1_H_tmp_e3d2a_33))
                    + (PackedUInt16::from_m31(output2h_col39)));
                let output_high_col41 = output_high_tmp_e3d2a_55.as_m31();
                *row[41] = output_high_col41;
                *lookup_data.sha_256_big_sigma_0_0 = [
                    input_limb_0_col0,
                    input_limb_1_col1,
                    output_low_col40,
                    output_high_col41,
                ];
                *row[42] = enabler_col.packed_at(row_index);
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    sha_256_big_sigma_0_0: Vec<[PackedM31; 4]>,
    sha_256_big_sigma_0_o_0_0: Vec<[PackedM31; 6]>,
    sha_256_big_sigma_0_o_1_0: Vec<[PackedM31; 6]>,
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
    verify_bitwise_xor_8_0: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_1: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_2: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_3: Vec<[PackedM31; 3]>,
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
        verify_bitwise_and_8: &relations::VerifyBitwiseAnd_8,
        sha_256_big_sigma_0_o_0: &relations::Sha256BigSigma0O0,
        sha_256_big_sigma_0_o_1: &relations::Sha256BigSigma0O1,
        verify_bitwise_xor_8: &relations::VerifyBitwiseXor_8,
        sha_256_big_sigma_0: &relations::Sha256BigSigma0,
    ) -> InteractionClaim {
        let enabler_col = Enabler::new(self.n_rows);
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
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
            &self.lookup_data.verify_bitwise_and_8_8,
            &self.lookup_data.verify_bitwise_and_8_9,
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
            &self.lookup_data.verify_bitwise_and_8_10,
            &self.lookup_data.verify_bitwise_and_8_11,
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
            &self.lookup_data.sha_256_big_sigma_0_o_0_0,
            &self.lookup_data.sha_256_big_sigma_0_o_1_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = sha_256_big_sigma_0_o_0.combine(values0);
                let denom1: PackedQM31 = sha_256_big_sigma_0_o_1.combine(values1);
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

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.sha_256_big_sigma_0_0,
        )
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (writer, values))| {
                let denom = sha_256_big_sigma_0.combine(values);
                writer.write_frac(-PackedQM31::one() * enabler_col.packed_at(i), denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
