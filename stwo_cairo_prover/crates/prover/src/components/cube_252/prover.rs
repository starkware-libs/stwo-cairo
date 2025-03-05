#![allow(unused_parens)]
#![allow(unused_imports)]
use itertools::Itertools;

use super::component::{Claim, InteractionClaim};
use crate::components::prelude::proving::*;
use crate::components::{range_check_9_9, verify_mul_252};

pub type InputType = Felt252Width27;
pub type PackedInputType = PackedFelt252Width27;
const N_TRACE_COLUMNS: usize = 85;

#[derive(Default)]
pub struct ClaimGenerator {
    pub inputs: Vec<InputType>,
}
impl ClaimGenerator {
    pub fn new() -> Self {
        Self { inputs: Vec::new() }
    }

    pub fn write_trace<MC: MerkleChannel>(
        mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        range_check_9_9_state: &range_check_9_9::ClaimGenerator,
        verify_mul_252_state: &mut verify_mul_252::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let n_rows = self.inputs.len();
        assert_ne!(n_rows, 0);
        let size = std::cmp::max(n_rows.next_power_of_two(), N_LANES);
        let log_size = size.ilog2();
        self.inputs.resize(size, *self.inputs.first().unwrap());
        let packed_inputs = pack_values(&self.inputs);

        let (trace, lookup_data) = write_trace_simd(
            n_rows,
            packed_inputs,
            range_check_9_9_state,
            verify_mul_252_state,
        );

        tree_builder.extend_evals(trace.to_evals());

        (
            Claim { log_size },
            InteractionClaimGenerator {
                n_rows,
                lookup_data,
            },
        )
    }

    pub fn add_input(&mut self, input: &InputType) {
        self.inputs.push(*input);
    }

    pub fn add_inputs(&mut self, inputs: &[InputType]) {
        self.inputs.extend_from_slice(inputs);
    }
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    n_rows: usize,
    inputs: Vec<PackedInputType>,
    range_check_9_9_state: &range_check_9_9::ClaimGenerator,
    verify_mul_252_state: &mut verify_mul_252::ClaimGenerator,
) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData) {
    let log_n_packed_rows = inputs.len().ilog2();
    let log_size = log_n_packed_rows + LOG_N_LANES;
    let (mut trace, mut lookup_data) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
        )
    };

    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let M31_8192 = PackedM31::broadcast(M31::from(8192));
    let padding_col = Enabler::new(n_rows);

    let mut verify_mul_252_inputs_vec = vec![[[[Felt252::default(); 3]; 16]; 2]; 1 << log_size];

    trace
        .par_iter_mut()
        .enumerate()
        .zip(inputs.into_par_iter())
        .zip(lookup_data.par_iter_mut())
        .zip(verify_mul_252_inputs_vec.par_iter_mut())
        .for_each(
            |((((row_index, mut row), cube_252_input), lookup_data), verify_mul_252_input)| {
                let input_tmp_fec87_0 = cube_252_input;
                let input_limb_0_col0 = input_tmp_fec87_0.get_m31(0);
                *row[0] = input_limb_0_col0;
                let input_limb_1_col1 = input_tmp_fec87_0.get_m31(1);
                *row[1] = input_limb_1_col1;
                let input_limb_2_col2 = input_tmp_fec87_0.get_m31(2);
                *row[2] = input_limb_2_col2;
                let input_limb_3_col3 = input_tmp_fec87_0.get_m31(3);
                *row[3] = input_limb_3_col3;
                let input_limb_4_col4 = input_tmp_fec87_0.get_m31(4);
                *row[4] = input_limb_4_col4;
                let input_limb_5_col5 = input_tmp_fec87_0.get_m31(5);
                *row[5] = input_limb_5_col5;
                let input_limb_6_col6 = input_tmp_fec87_0.get_m31(6);
                *row[6] = input_limb_6_col6;
                let input_limb_7_col7 = input_tmp_fec87_0.get_m31(7);
                *row[7] = input_limb_7_col7;
                let input_limb_8_col8 = input_tmp_fec87_0.get_m31(8);
                *row[8] = input_limb_8_col8;
                let input_limb_9_col9 = input_tmp_fec87_0.get_m31(9);
                *row[9] = input_limb_9_col9;

                // Felt 252 Unpack From 27 Range Check Output.

                let input_as_felt252_tmp_fec87_1 =
                    PackedFelt252::from_packed_felt252width27(input_tmp_fec87_0);
                let unpacked_limb_0_col10 = input_as_felt252_tmp_fec87_1.get_m31(0);
                *row[10] = unpacked_limb_0_col10;
                let unpacked_limb_1_col11 = input_as_felt252_tmp_fec87_1.get_m31(1);
                *row[11] = unpacked_limb_1_col11;
                let unpacked_limb_3_col12 = input_as_felt252_tmp_fec87_1.get_m31(3);
                *row[12] = unpacked_limb_3_col12;
                let unpacked_limb_4_col13 = input_as_felt252_tmp_fec87_1.get_m31(4);
                *row[13] = unpacked_limb_4_col13;
                let unpacked_limb_6_col14 = input_as_felt252_tmp_fec87_1.get_m31(6);
                *row[14] = unpacked_limb_6_col14;
                let unpacked_limb_7_col15 = input_as_felt252_tmp_fec87_1.get_m31(7);
                *row[15] = unpacked_limb_7_col15;
                let unpacked_limb_9_col16 = input_as_felt252_tmp_fec87_1.get_m31(9);
                *row[16] = unpacked_limb_9_col16;
                let unpacked_limb_10_col17 = input_as_felt252_tmp_fec87_1.get_m31(10);
                *row[17] = unpacked_limb_10_col17;
                let unpacked_limb_12_col18 = input_as_felt252_tmp_fec87_1.get_m31(12);
                *row[18] = unpacked_limb_12_col18;
                let unpacked_limb_13_col19 = input_as_felt252_tmp_fec87_1.get_m31(13);
                *row[19] = unpacked_limb_13_col19;
                let unpacked_limb_15_col20 = input_as_felt252_tmp_fec87_1.get_m31(15);
                *row[20] = unpacked_limb_15_col20;
                let unpacked_limb_16_col21 = input_as_felt252_tmp_fec87_1.get_m31(16);
                *row[21] = unpacked_limb_16_col21;
                let unpacked_limb_18_col22 = input_as_felt252_tmp_fec87_1.get_m31(18);
                *row[22] = unpacked_limb_18_col22;
                let unpacked_limb_19_col23 = input_as_felt252_tmp_fec87_1.get_m31(19);
                *row[23] = unpacked_limb_19_col23;
                let unpacked_limb_21_col24 = input_as_felt252_tmp_fec87_1.get_m31(21);
                *row[24] = unpacked_limb_21_col24;
                let unpacked_limb_22_col25 = input_as_felt252_tmp_fec87_1.get_m31(22);
                *row[25] = unpacked_limb_22_col25;
                let unpacked_limb_24_col26 = input_as_felt252_tmp_fec87_1.get_m31(24);
                *row[26] = unpacked_limb_24_col26;
                let unpacked_limb_25_col27 = input_as_felt252_tmp_fec87_1.get_m31(25);
                *row[27] = unpacked_limb_25_col27;

                // Range Check Mem Value N 28.

                let range_check_9_9_inputs_0 =
                    [unpacked_limb_0_col10, unpacked_limb_1_col11].unpack();
                *lookup_data.range_check_9_9_0 = [unpacked_limb_0_col10, unpacked_limb_1_col11];
                let range_check_9_9_inputs_1 = [
                    ((((input_limb_0_col0) - (unpacked_limb_0_col10))
                        - ((unpacked_limb_1_col11) * (M31_512)))
                        * (M31_8192)),
                    unpacked_limb_3_col12,
                ]
                .unpack();
                *lookup_data.range_check_9_9_1 = [
                    ((((input_limb_0_col0) - (unpacked_limb_0_col10))
                        - ((unpacked_limb_1_col11) * (M31_512)))
                        * (M31_8192)),
                    unpacked_limb_3_col12,
                ];
                let range_check_9_9_inputs_2 = [
                    unpacked_limb_4_col13,
                    ((((input_limb_1_col1) - (unpacked_limb_3_col12))
                        - ((unpacked_limb_4_col13) * (M31_512)))
                        * (M31_8192)),
                ]
                .unpack();
                *lookup_data.range_check_9_9_2 = [
                    unpacked_limb_4_col13,
                    ((((input_limb_1_col1) - (unpacked_limb_3_col12))
                        - ((unpacked_limb_4_col13) * (M31_512)))
                        * (M31_8192)),
                ];
                let range_check_9_9_inputs_3 =
                    [unpacked_limb_6_col14, unpacked_limb_7_col15].unpack();
                *lookup_data.range_check_9_9_3 = [unpacked_limb_6_col14, unpacked_limb_7_col15];
                let range_check_9_9_inputs_4 = [
                    ((((input_limb_2_col2) - (unpacked_limb_6_col14))
                        - ((unpacked_limb_7_col15) * (M31_512)))
                        * (M31_8192)),
                    unpacked_limb_9_col16,
                ]
                .unpack();
                *lookup_data.range_check_9_9_4 = [
                    ((((input_limb_2_col2) - (unpacked_limb_6_col14))
                        - ((unpacked_limb_7_col15) * (M31_512)))
                        * (M31_8192)),
                    unpacked_limb_9_col16,
                ];
                let range_check_9_9_inputs_5 = [
                    unpacked_limb_10_col17,
                    ((((input_limb_3_col3) - (unpacked_limb_9_col16))
                        - ((unpacked_limb_10_col17) * (M31_512)))
                        * (M31_8192)),
                ]
                .unpack();
                *lookup_data.range_check_9_9_5 = [
                    unpacked_limb_10_col17,
                    ((((input_limb_3_col3) - (unpacked_limb_9_col16))
                        - ((unpacked_limb_10_col17) * (M31_512)))
                        * (M31_8192)),
                ];
                let range_check_9_9_inputs_6 =
                    [unpacked_limb_12_col18, unpacked_limb_13_col19].unpack();
                *lookup_data.range_check_9_9_6 = [unpacked_limb_12_col18, unpacked_limb_13_col19];
                let range_check_9_9_inputs_7 = [
                    ((((input_limb_4_col4) - (unpacked_limb_12_col18))
                        - ((unpacked_limb_13_col19) * (M31_512)))
                        * (M31_8192)),
                    unpacked_limb_15_col20,
                ]
                .unpack();
                *lookup_data.range_check_9_9_7 = [
                    ((((input_limb_4_col4) - (unpacked_limb_12_col18))
                        - ((unpacked_limb_13_col19) * (M31_512)))
                        * (M31_8192)),
                    unpacked_limb_15_col20,
                ];
                let range_check_9_9_inputs_8 = [
                    unpacked_limb_16_col21,
                    ((((input_limb_5_col5) - (unpacked_limb_15_col20))
                        - ((unpacked_limb_16_col21) * (M31_512)))
                        * (M31_8192)),
                ]
                .unpack();
                *lookup_data.range_check_9_9_8 = [
                    unpacked_limb_16_col21,
                    ((((input_limb_5_col5) - (unpacked_limb_15_col20))
                        - ((unpacked_limb_16_col21) * (M31_512)))
                        * (M31_8192)),
                ];
                let range_check_9_9_inputs_9 =
                    [unpacked_limb_18_col22, unpacked_limb_19_col23].unpack();
                *lookup_data.range_check_9_9_9 = [unpacked_limb_18_col22, unpacked_limb_19_col23];
                let range_check_9_9_inputs_10 = [
                    ((((input_limb_6_col6) - (unpacked_limb_18_col22))
                        - ((unpacked_limb_19_col23) * (M31_512)))
                        * (M31_8192)),
                    unpacked_limb_21_col24,
                ]
                .unpack();
                *lookup_data.range_check_9_9_10 = [
                    ((((input_limb_6_col6) - (unpacked_limb_18_col22))
                        - ((unpacked_limb_19_col23) * (M31_512)))
                        * (M31_8192)),
                    unpacked_limb_21_col24,
                ];
                let range_check_9_9_inputs_11 = [
                    unpacked_limb_22_col25,
                    ((((input_limb_7_col7) - (unpacked_limb_21_col24))
                        - ((unpacked_limb_22_col25) * (M31_512)))
                        * (M31_8192)),
                ]
                .unpack();
                *lookup_data.range_check_9_9_11 = [
                    unpacked_limb_22_col25,
                    ((((input_limb_7_col7) - (unpacked_limb_21_col24))
                        - ((unpacked_limb_22_col25) * (M31_512)))
                        * (M31_8192)),
                ];
                let range_check_9_9_inputs_12 =
                    [unpacked_limb_24_col26, unpacked_limb_25_col27].unpack();
                *lookup_data.range_check_9_9_12 = [unpacked_limb_24_col26, unpacked_limb_25_col27];
                let range_check_9_9_inputs_13 = [
                    ((((input_limb_8_col8) - (unpacked_limb_24_col26))
                        - ((unpacked_limb_25_col27) * (M31_512)))
                        * (M31_8192)),
                    input_limb_9_col9,
                ]
                .unpack();
                *lookup_data.range_check_9_9_13 = [
                    ((((input_limb_8_col8) - (unpacked_limb_24_col26))
                        - ((unpacked_limb_25_col27) * (M31_512)))
                        * (M31_8192)),
                    input_limb_9_col9,
                ];

                let a_tmp_fec87_2 = PackedFelt252::from_limbs([
                    unpacked_limb_0_col10,
                    unpacked_limb_1_col11,
                    ((((input_limb_0_col0) - (unpacked_limb_0_col10))
                        - ((unpacked_limb_1_col11) * (M31_512)))
                        * (M31_8192)),
                    unpacked_limb_3_col12,
                    unpacked_limb_4_col13,
                    ((((input_limb_1_col1) - (unpacked_limb_3_col12))
                        - ((unpacked_limb_4_col13) * (M31_512)))
                        * (M31_8192)),
                    unpacked_limb_6_col14,
                    unpacked_limb_7_col15,
                    ((((input_limb_2_col2) - (unpacked_limb_6_col14))
                        - ((unpacked_limb_7_col15) * (M31_512)))
                        * (M31_8192)),
                    unpacked_limb_9_col16,
                    unpacked_limb_10_col17,
                    ((((input_limb_3_col3) - (unpacked_limb_9_col16))
                        - ((unpacked_limb_10_col17) * (M31_512)))
                        * (M31_8192)),
                    unpacked_limb_12_col18,
                    unpacked_limb_13_col19,
                    ((((input_limb_4_col4) - (unpacked_limb_12_col18))
                        - ((unpacked_limb_13_col19) * (M31_512)))
                        * (M31_8192)),
                    unpacked_limb_15_col20,
                    unpacked_limb_16_col21,
                    ((((input_limb_5_col5) - (unpacked_limb_15_col20))
                        - ((unpacked_limb_16_col21) * (M31_512)))
                        * (M31_8192)),
                    unpacked_limb_18_col22,
                    unpacked_limb_19_col23,
                    ((((input_limb_6_col6) - (unpacked_limb_18_col22))
                        - ((unpacked_limb_19_col23) * (M31_512)))
                        * (M31_8192)),
                    unpacked_limb_21_col24,
                    unpacked_limb_22_col25,
                    ((((input_limb_7_col7) - (unpacked_limb_21_col24))
                        - ((unpacked_limb_22_col25) * (M31_512)))
                        * (M31_8192)),
                    unpacked_limb_24_col26,
                    unpacked_limb_25_col27,
                    ((((input_limb_8_col8) - (unpacked_limb_24_col26))
                        - ((unpacked_limb_25_col27) * (M31_512)))
                        * (M31_8192)),
                    input_limb_9_col9,
                ]);
                let a_tmp_fec87_2_limb_2 = ((((input_limb_0_col0) - (unpacked_limb_0_col10))
                    - ((unpacked_limb_1_col11) * (M31_512)))
                    * (M31_8192));
                let a_tmp_fec87_2_limb_5 = ((((input_limb_1_col1) - (unpacked_limb_3_col12))
                    - ((unpacked_limb_4_col13) * (M31_512)))
                    * (M31_8192));
                let a_tmp_fec87_2_limb_8 = ((((input_limb_2_col2) - (unpacked_limb_6_col14))
                    - ((unpacked_limb_7_col15) * (M31_512)))
                    * (M31_8192));
                let a_tmp_fec87_2_limb_11 = ((((input_limb_3_col3) - (unpacked_limb_9_col16))
                    - ((unpacked_limb_10_col17) * (M31_512)))
                    * (M31_8192));
                let a_tmp_fec87_2_limb_14 = ((((input_limb_4_col4) - (unpacked_limb_12_col18))
                    - ((unpacked_limb_13_col19) * (M31_512)))
                    * (M31_8192));
                let a_tmp_fec87_2_limb_17 = ((((input_limb_5_col5) - (unpacked_limb_15_col20))
                    - ((unpacked_limb_16_col21) * (M31_512)))
                    * (M31_8192));
                let a_tmp_fec87_2_limb_20 = ((((input_limb_6_col6) - (unpacked_limb_18_col22))
                    - ((unpacked_limb_19_col23) * (M31_512)))
                    * (M31_8192));
                let a_tmp_fec87_2_limb_23 = ((((input_limb_7_col7) - (unpacked_limb_21_col24))
                    - ((unpacked_limb_22_col25) * (M31_512)))
                    * (M31_8192));
                let a_tmp_fec87_2_limb_26 = ((((input_limb_8_col8) - (unpacked_limb_24_col26))
                    - ((unpacked_limb_25_col27) * (M31_512)))
                    * (M31_8192));

                // Mul 252.

                let mul_res_tmp_fec87_3 = ((a_tmp_fec87_2) * (a_tmp_fec87_2));
                let mul_res_limb_0_col28 = mul_res_tmp_fec87_3.get_m31(0);
                *row[28] = mul_res_limb_0_col28;
                let mul_res_limb_1_col29 = mul_res_tmp_fec87_3.get_m31(1);
                *row[29] = mul_res_limb_1_col29;
                let mul_res_limb_2_col30 = mul_res_tmp_fec87_3.get_m31(2);
                *row[30] = mul_res_limb_2_col30;
                let mul_res_limb_3_col31 = mul_res_tmp_fec87_3.get_m31(3);
                *row[31] = mul_res_limb_3_col31;
                let mul_res_limb_4_col32 = mul_res_tmp_fec87_3.get_m31(4);
                *row[32] = mul_res_limb_4_col32;
                let mul_res_limb_5_col33 = mul_res_tmp_fec87_3.get_m31(5);
                *row[33] = mul_res_limb_5_col33;
                let mul_res_limb_6_col34 = mul_res_tmp_fec87_3.get_m31(6);
                *row[34] = mul_res_limb_6_col34;
                let mul_res_limb_7_col35 = mul_res_tmp_fec87_3.get_m31(7);
                *row[35] = mul_res_limb_7_col35;
                let mul_res_limb_8_col36 = mul_res_tmp_fec87_3.get_m31(8);
                *row[36] = mul_res_limb_8_col36;
                let mul_res_limb_9_col37 = mul_res_tmp_fec87_3.get_m31(9);
                *row[37] = mul_res_limb_9_col37;
                let mul_res_limb_10_col38 = mul_res_tmp_fec87_3.get_m31(10);
                *row[38] = mul_res_limb_10_col38;
                let mul_res_limb_11_col39 = mul_res_tmp_fec87_3.get_m31(11);
                *row[39] = mul_res_limb_11_col39;
                let mul_res_limb_12_col40 = mul_res_tmp_fec87_3.get_m31(12);
                *row[40] = mul_res_limb_12_col40;
                let mul_res_limb_13_col41 = mul_res_tmp_fec87_3.get_m31(13);
                *row[41] = mul_res_limb_13_col41;
                let mul_res_limb_14_col42 = mul_res_tmp_fec87_3.get_m31(14);
                *row[42] = mul_res_limb_14_col42;
                let mul_res_limb_15_col43 = mul_res_tmp_fec87_3.get_m31(15);
                *row[43] = mul_res_limb_15_col43;
                let mul_res_limb_16_col44 = mul_res_tmp_fec87_3.get_m31(16);
                *row[44] = mul_res_limb_16_col44;
                let mul_res_limb_17_col45 = mul_res_tmp_fec87_3.get_m31(17);
                *row[45] = mul_res_limb_17_col45;
                let mul_res_limb_18_col46 = mul_res_tmp_fec87_3.get_m31(18);
                *row[46] = mul_res_limb_18_col46;
                let mul_res_limb_19_col47 = mul_res_tmp_fec87_3.get_m31(19);
                *row[47] = mul_res_limb_19_col47;
                let mul_res_limb_20_col48 = mul_res_tmp_fec87_3.get_m31(20);
                *row[48] = mul_res_limb_20_col48;
                let mul_res_limb_21_col49 = mul_res_tmp_fec87_3.get_m31(21);
                *row[49] = mul_res_limb_21_col49;
                let mul_res_limb_22_col50 = mul_res_tmp_fec87_3.get_m31(22);
                *row[50] = mul_res_limb_22_col50;
                let mul_res_limb_23_col51 = mul_res_tmp_fec87_3.get_m31(23);
                *row[51] = mul_res_limb_23_col51;
                let mul_res_limb_24_col52 = mul_res_tmp_fec87_3.get_m31(24);
                *row[52] = mul_res_limb_24_col52;
                let mul_res_limb_25_col53 = mul_res_tmp_fec87_3.get_m31(25);
                *row[53] = mul_res_limb_25_col53;
                let mul_res_limb_26_col54 = mul_res_tmp_fec87_3.get_m31(26);
                *row[54] = mul_res_limb_26_col54;
                let mul_res_limb_27_col55 = mul_res_tmp_fec87_3.get_m31(27);
                *row[55] = mul_res_limb_27_col55;

                // Range Check Mem Value N 28.

                let range_check_9_9_inputs_14 =
                    [mul_res_limb_0_col28, mul_res_limb_1_col29].unpack();
                *lookup_data.range_check_9_9_14 = [mul_res_limb_0_col28, mul_res_limb_1_col29];
                let range_check_9_9_inputs_15 =
                    [mul_res_limb_2_col30, mul_res_limb_3_col31].unpack();
                *lookup_data.range_check_9_9_15 = [mul_res_limb_2_col30, mul_res_limb_3_col31];
                let range_check_9_9_inputs_16 =
                    [mul_res_limb_4_col32, mul_res_limb_5_col33].unpack();
                *lookup_data.range_check_9_9_16 = [mul_res_limb_4_col32, mul_res_limb_5_col33];
                let range_check_9_9_inputs_17 =
                    [mul_res_limb_6_col34, mul_res_limb_7_col35].unpack();
                *lookup_data.range_check_9_9_17 = [mul_res_limb_6_col34, mul_res_limb_7_col35];
                let range_check_9_9_inputs_18 =
                    [mul_res_limb_8_col36, mul_res_limb_9_col37].unpack();
                *lookup_data.range_check_9_9_18 = [mul_res_limb_8_col36, mul_res_limb_9_col37];
                let range_check_9_9_inputs_19 =
                    [mul_res_limb_10_col38, mul_res_limb_11_col39].unpack();
                *lookup_data.range_check_9_9_19 = [mul_res_limb_10_col38, mul_res_limb_11_col39];
                let range_check_9_9_inputs_20 =
                    [mul_res_limb_12_col40, mul_res_limb_13_col41].unpack();
                *lookup_data.range_check_9_9_20 = [mul_res_limb_12_col40, mul_res_limb_13_col41];
                let range_check_9_9_inputs_21 =
                    [mul_res_limb_14_col42, mul_res_limb_15_col43].unpack();
                *lookup_data.range_check_9_9_21 = [mul_res_limb_14_col42, mul_res_limb_15_col43];
                let range_check_9_9_inputs_22 =
                    [mul_res_limb_16_col44, mul_res_limb_17_col45].unpack();
                *lookup_data.range_check_9_9_22 = [mul_res_limb_16_col44, mul_res_limb_17_col45];
                let range_check_9_9_inputs_23 =
                    [mul_res_limb_18_col46, mul_res_limb_19_col47].unpack();
                *lookup_data.range_check_9_9_23 = [mul_res_limb_18_col46, mul_res_limb_19_col47];
                let range_check_9_9_inputs_24 =
                    [mul_res_limb_20_col48, mul_res_limb_21_col49].unpack();
                *lookup_data.range_check_9_9_24 = [mul_res_limb_20_col48, mul_res_limb_21_col49];
                let range_check_9_9_inputs_25 =
                    [mul_res_limb_22_col50, mul_res_limb_23_col51].unpack();
                *lookup_data.range_check_9_9_25 = [mul_res_limb_22_col50, mul_res_limb_23_col51];
                let range_check_9_9_inputs_26 =
                    [mul_res_limb_24_col52, mul_res_limb_25_col53].unpack();
                *lookup_data.range_check_9_9_26 = [mul_res_limb_24_col52, mul_res_limb_25_col53];
                let range_check_9_9_inputs_27 =
                    [mul_res_limb_26_col54, mul_res_limb_27_col55].unpack();
                *lookup_data.range_check_9_9_27 = [mul_res_limb_26_col54, mul_res_limb_27_col55];

                let verify_mul_252_inputs_0 =
                    [a_tmp_fec87_2, a_tmp_fec87_2, mul_res_tmp_fec87_3].unpack();
                *lookup_data.verify_mul_252_0 = [
                    unpacked_limb_0_col10,
                    unpacked_limb_1_col11,
                    a_tmp_fec87_2_limb_2,
                    unpacked_limb_3_col12,
                    unpacked_limb_4_col13,
                    a_tmp_fec87_2_limb_5,
                    unpacked_limb_6_col14,
                    unpacked_limb_7_col15,
                    a_tmp_fec87_2_limb_8,
                    unpacked_limb_9_col16,
                    unpacked_limb_10_col17,
                    a_tmp_fec87_2_limb_11,
                    unpacked_limb_12_col18,
                    unpacked_limb_13_col19,
                    a_tmp_fec87_2_limb_14,
                    unpacked_limb_15_col20,
                    unpacked_limb_16_col21,
                    a_tmp_fec87_2_limb_17,
                    unpacked_limb_18_col22,
                    unpacked_limb_19_col23,
                    a_tmp_fec87_2_limb_20,
                    unpacked_limb_21_col24,
                    unpacked_limb_22_col25,
                    a_tmp_fec87_2_limb_23,
                    unpacked_limb_24_col26,
                    unpacked_limb_25_col27,
                    a_tmp_fec87_2_limb_26,
                    input_limb_9_col9,
                    unpacked_limb_0_col10,
                    unpacked_limb_1_col11,
                    a_tmp_fec87_2_limb_2,
                    unpacked_limb_3_col12,
                    unpacked_limb_4_col13,
                    a_tmp_fec87_2_limb_5,
                    unpacked_limb_6_col14,
                    unpacked_limb_7_col15,
                    a_tmp_fec87_2_limb_8,
                    unpacked_limb_9_col16,
                    unpacked_limb_10_col17,
                    a_tmp_fec87_2_limb_11,
                    unpacked_limb_12_col18,
                    unpacked_limb_13_col19,
                    a_tmp_fec87_2_limb_14,
                    unpacked_limb_15_col20,
                    unpacked_limb_16_col21,
                    a_tmp_fec87_2_limb_17,
                    unpacked_limb_18_col22,
                    unpacked_limb_19_col23,
                    a_tmp_fec87_2_limb_20,
                    unpacked_limb_21_col24,
                    unpacked_limb_22_col25,
                    a_tmp_fec87_2_limb_23,
                    unpacked_limb_24_col26,
                    unpacked_limb_25_col27,
                    a_tmp_fec87_2_limb_26,
                    input_limb_9_col9,
                    mul_res_limb_0_col28,
                    mul_res_limb_1_col29,
                    mul_res_limb_2_col30,
                    mul_res_limb_3_col31,
                    mul_res_limb_4_col32,
                    mul_res_limb_5_col33,
                    mul_res_limb_6_col34,
                    mul_res_limb_7_col35,
                    mul_res_limb_8_col36,
                    mul_res_limb_9_col37,
                    mul_res_limb_10_col38,
                    mul_res_limb_11_col39,
                    mul_res_limb_12_col40,
                    mul_res_limb_13_col41,
                    mul_res_limb_14_col42,
                    mul_res_limb_15_col43,
                    mul_res_limb_16_col44,
                    mul_res_limb_17_col45,
                    mul_res_limb_18_col46,
                    mul_res_limb_19_col47,
                    mul_res_limb_20_col48,
                    mul_res_limb_21_col49,
                    mul_res_limb_22_col50,
                    mul_res_limb_23_col51,
                    mul_res_limb_24_col52,
                    mul_res_limb_25_col53,
                    mul_res_limb_26_col54,
                    mul_res_limb_27_col55,
                ];

                // Mul 252.

                let mul_res_tmp_fec87_4 = ((a_tmp_fec87_2) * (mul_res_tmp_fec87_3));
                let mul_res_limb_0_col56 = mul_res_tmp_fec87_4.get_m31(0);
                *row[56] = mul_res_limb_0_col56;
                let mul_res_limb_1_col57 = mul_res_tmp_fec87_4.get_m31(1);
                *row[57] = mul_res_limb_1_col57;
                let mul_res_limb_2_col58 = mul_res_tmp_fec87_4.get_m31(2);
                *row[58] = mul_res_limb_2_col58;
                let mul_res_limb_3_col59 = mul_res_tmp_fec87_4.get_m31(3);
                *row[59] = mul_res_limb_3_col59;
                let mul_res_limb_4_col60 = mul_res_tmp_fec87_4.get_m31(4);
                *row[60] = mul_res_limb_4_col60;
                let mul_res_limb_5_col61 = mul_res_tmp_fec87_4.get_m31(5);
                *row[61] = mul_res_limb_5_col61;
                let mul_res_limb_6_col62 = mul_res_tmp_fec87_4.get_m31(6);
                *row[62] = mul_res_limb_6_col62;
                let mul_res_limb_7_col63 = mul_res_tmp_fec87_4.get_m31(7);
                *row[63] = mul_res_limb_7_col63;
                let mul_res_limb_8_col64 = mul_res_tmp_fec87_4.get_m31(8);
                *row[64] = mul_res_limb_8_col64;
                let mul_res_limb_9_col65 = mul_res_tmp_fec87_4.get_m31(9);
                *row[65] = mul_res_limb_9_col65;
                let mul_res_limb_10_col66 = mul_res_tmp_fec87_4.get_m31(10);
                *row[66] = mul_res_limb_10_col66;
                let mul_res_limb_11_col67 = mul_res_tmp_fec87_4.get_m31(11);
                *row[67] = mul_res_limb_11_col67;
                let mul_res_limb_12_col68 = mul_res_tmp_fec87_4.get_m31(12);
                *row[68] = mul_res_limb_12_col68;
                let mul_res_limb_13_col69 = mul_res_tmp_fec87_4.get_m31(13);
                *row[69] = mul_res_limb_13_col69;
                let mul_res_limb_14_col70 = mul_res_tmp_fec87_4.get_m31(14);
                *row[70] = mul_res_limb_14_col70;
                let mul_res_limb_15_col71 = mul_res_tmp_fec87_4.get_m31(15);
                *row[71] = mul_res_limb_15_col71;
                let mul_res_limb_16_col72 = mul_res_tmp_fec87_4.get_m31(16);
                *row[72] = mul_res_limb_16_col72;
                let mul_res_limb_17_col73 = mul_res_tmp_fec87_4.get_m31(17);
                *row[73] = mul_res_limb_17_col73;
                let mul_res_limb_18_col74 = mul_res_tmp_fec87_4.get_m31(18);
                *row[74] = mul_res_limb_18_col74;
                let mul_res_limb_19_col75 = mul_res_tmp_fec87_4.get_m31(19);
                *row[75] = mul_res_limb_19_col75;
                let mul_res_limb_20_col76 = mul_res_tmp_fec87_4.get_m31(20);
                *row[76] = mul_res_limb_20_col76;
                let mul_res_limb_21_col77 = mul_res_tmp_fec87_4.get_m31(21);
                *row[77] = mul_res_limb_21_col77;
                let mul_res_limb_22_col78 = mul_res_tmp_fec87_4.get_m31(22);
                *row[78] = mul_res_limb_22_col78;
                let mul_res_limb_23_col79 = mul_res_tmp_fec87_4.get_m31(23);
                *row[79] = mul_res_limb_23_col79;
                let mul_res_limb_24_col80 = mul_res_tmp_fec87_4.get_m31(24);
                *row[80] = mul_res_limb_24_col80;
                let mul_res_limb_25_col81 = mul_res_tmp_fec87_4.get_m31(25);
                *row[81] = mul_res_limb_25_col81;
                let mul_res_limb_26_col82 = mul_res_tmp_fec87_4.get_m31(26);
                *row[82] = mul_res_limb_26_col82;
                let mul_res_limb_27_col83 = mul_res_tmp_fec87_4.get_m31(27);
                *row[83] = mul_res_limb_27_col83;

                // Range Check Mem Value N 28.

                let range_check_9_9_inputs_28 =
                    [mul_res_limb_0_col56, mul_res_limb_1_col57].unpack();
                *lookup_data.range_check_9_9_28 = [mul_res_limb_0_col56, mul_res_limb_1_col57];
                let range_check_9_9_inputs_29 =
                    [mul_res_limb_2_col58, mul_res_limb_3_col59].unpack();
                *lookup_data.range_check_9_9_29 = [mul_res_limb_2_col58, mul_res_limb_3_col59];
                let range_check_9_9_inputs_30 =
                    [mul_res_limb_4_col60, mul_res_limb_5_col61].unpack();
                *lookup_data.range_check_9_9_30 = [mul_res_limb_4_col60, mul_res_limb_5_col61];
                let range_check_9_9_inputs_31 =
                    [mul_res_limb_6_col62, mul_res_limb_7_col63].unpack();
                *lookup_data.range_check_9_9_31 = [mul_res_limb_6_col62, mul_res_limb_7_col63];
                let range_check_9_9_inputs_32 =
                    [mul_res_limb_8_col64, mul_res_limb_9_col65].unpack();
                *lookup_data.range_check_9_9_32 = [mul_res_limb_8_col64, mul_res_limb_9_col65];
                let range_check_9_9_inputs_33 =
                    [mul_res_limb_10_col66, mul_res_limb_11_col67].unpack();
                *lookup_data.range_check_9_9_33 = [mul_res_limb_10_col66, mul_res_limb_11_col67];
                let range_check_9_9_inputs_34 =
                    [mul_res_limb_12_col68, mul_res_limb_13_col69].unpack();
                *lookup_data.range_check_9_9_34 = [mul_res_limb_12_col68, mul_res_limb_13_col69];
                let range_check_9_9_inputs_35 =
                    [mul_res_limb_14_col70, mul_res_limb_15_col71].unpack();
                *lookup_data.range_check_9_9_35 = [mul_res_limb_14_col70, mul_res_limb_15_col71];
                let range_check_9_9_inputs_36 =
                    [mul_res_limb_16_col72, mul_res_limb_17_col73].unpack();
                *lookup_data.range_check_9_9_36 = [mul_res_limb_16_col72, mul_res_limb_17_col73];
                let range_check_9_9_inputs_37 =
                    [mul_res_limb_18_col74, mul_res_limb_19_col75].unpack();
                *lookup_data.range_check_9_9_37 = [mul_res_limb_18_col74, mul_res_limb_19_col75];
                let range_check_9_9_inputs_38 =
                    [mul_res_limb_20_col76, mul_res_limb_21_col77].unpack();
                *lookup_data.range_check_9_9_38 = [mul_res_limb_20_col76, mul_res_limb_21_col77];
                let range_check_9_9_inputs_39 =
                    [mul_res_limb_22_col78, mul_res_limb_23_col79].unpack();
                *lookup_data.range_check_9_9_39 = [mul_res_limb_22_col78, mul_res_limb_23_col79];
                let range_check_9_9_inputs_40 =
                    [mul_res_limb_24_col80, mul_res_limb_25_col81].unpack();
                *lookup_data.range_check_9_9_40 = [mul_res_limb_24_col80, mul_res_limb_25_col81];
                let range_check_9_9_inputs_41 =
                    [mul_res_limb_26_col82, mul_res_limb_27_col83].unpack();
                *lookup_data.range_check_9_9_41 = [mul_res_limb_26_col82, mul_res_limb_27_col83];

                let verify_mul_252_inputs_1 =
                    [a_tmp_fec87_2, mul_res_tmp_fec87_3, mul_res_tmp_fec87_4].unpack();
                *lookup_data.verify_mul_252_1 = [
                    unpacked_limb_0_col10,
                    unpacked_limb_1_col11,
                    a_tmp_fec87_2_limb_2,
                    unpacked_limb_3_col12,
                    unpacked_limb_4_col13,
                    a_tmp_fec87_2_limb_5,
                    unpacked_limb_6_col14,
                    unpacked_limb_7_col15,
                    a_tmp_fec87_2_limb_8,
                    unpacked_limb_9_col16,
                    unpacked_limb_10_col17,
                    a_tmp_fec87_2_limb_11,
                    unpacked_limb_12_col18,
                    unpacked_limb_13_col19,
                    a_tmp_fec87_2_limb_14,
                    unpacked_limb_15_col20,
                    unpacked_limb_16_col21,
                    a_tmp_fec87_2_limb_17,
                    unpacked_limb_18_col22,
                    unpacked_limb_19_col23,
                    a_tmp_fec87_2_limb_20,
                    unpacked_limb_21_col24,
                    unpacked_limb_22_col25,
                    a_tmp_fec87_2_limb_23,
                    unpacked_limb_24_col26,
                    unpacked_limb_25_col27,
                    a_tmp_fec87_2_limb_26,
                    input_limb_9_col9,
                    mul_res_limb_0_col28,
                    mul_res_limb_1_col29,
                    mul_res_limb_2_col30,
                    mul_res_limb_3_col31,
                    mul_res_limb_4_col32,
                    mul_res_limb_5_col33,
                    mul_res_limb_6_col34,
                    mul_res_limb_7_col35,
                    mul_res_limb_8_col36,
                    mul_res_limb_9_col37,
                    mul_res_limb_10_col38,
                    mul_res_limb_11_col39,
                    mul_res_limb_12_col40,
                    mul_res_limb_13_col41,
                    mul_res_limb_14_col42,
                    mul_res_limb_15_col43,
                    mul_res_limb_16_col44,
                    mul_res_limb_17_col45,
                    mul_res_limb_18_col46,
                    mul_res_limb_19_col47,
                    mul_res_limb_20_col48,
                    mul_res_limb_21_col49,
                    mul_res_limb_22_col50,
                    mul_res_limb_23_col51,
                    mul_res_limb_24_col52,
                    mul_res_limb_25_col53,
                    mul_res_limb_26_col54,
                    mul_res_limb_27_col55,
                    mul_res_limb_0_col56,
                    mul_res_limb_1_col57,
                    mul_res_limb_2_col58,
                    mul_res_limb_3_col59,
                    mul_res_limb_4_col60,
                    mul_res_limb_5_col61,
                    mul_res_limb_6_col62,
                    mul_res_limb_7_col63,
                    mul_res_limb_8_col64,
                    mul_res_limb_9_col65,
                    mul_res_limb_10_col66,
                    mul_res_limb_11_col67,
                    mul_res_limb_12_col68,
                    mul_res_limb_13_col69,
                    mul_res_limb_14_col70,
                    mul_res_limb_15_col71,
                    mul_res_limb_16_col72,
                    mul_res_limb_17_col73,
                    mul_res_limb_18_col74,
                    mul_res_limb_19_col75,
                    mul_res_limb_20_col76,
                    mul_res_limb_21_col77,
                    mul_res_limb_22_col78,
                    mul_res_limb_23_col79,
                    mul_res_limb_24_col80,
                    mul_res_limb_25_col81,
                    mul_res_limb_26_col82,
                    mul_res_limb_27_col83,
                ];

                *lookup_data.cube_252_0 = [
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
                    (((mul_res_limb_0_col56) + ((mul_res_limb_1_col57) * (M31_512)))
                        + ((mul_res_limb_2_col58) * (M31_262144))),
                    (((mul_res_limb_3_col59) + ((mul_res_limb_4_col60) * (M31_512)))
                        + ((mul_res_limb_5_col61) * (M31_262144))),
                    (((mul_res_limb_6_col62) + ((mul_res_limb_7_col63) * (M31_512)))
                        + ((mul_res_limb_8_col64) * (M31_262144))),
                    (((mul_res_limb_9_col65) + ((mul_res_limb_10_col66) * (M31_512)))
                        + ((mul_res_limb_11_col67) * (M31_262144))),
                    (((mul_res_limb_12_col68) + ((mul_res_limb_13_col69) * (M31_512)))
                        + ((mul_res_limb_14_col70) * (M31_262144))),
                    (((mul_res_limb_15_col71) + ((mul_res_limb_16_col72) * (M31_512)))
                        + ((mul_res_limb_17_col73) * (M31_262144))),
                    (((mul_res_limb_18_col74) + ((mul_res_limb_19_col75) * (M31_512)))
                        + ((mul_res_limb_20_col76) * (M31_262144))),
                    (((mul_res_limb_21_col77) + ((mul_res_limb_22_col78) * (M31_512)))
                        + ((mul_res_limb_23_col79) * (M31_262144))),
                    (((mul_res_limb_24_col80) + ((mul_res_limb_25_col81) * (M31_512)))
                        + ((mul_res_limb_26_col82) * (M31_262144))),
                    mul_res_limb_27_col83,
                ];

                *row[84] = padding_col.packed_at(row_index);

                // Add sub-components inputs.
                *verify_mul_252_input = [verify_mul_252_inputs_0, verify_mul_252_inputs_1];
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_0);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_1);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_2);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_3);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_4);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_5);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_6);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_7);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_8);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_9);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_10);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_11);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_12);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_13);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_14);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_15);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_16);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_17);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_18);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_19);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_20);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_21);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_22);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_23);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_24);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_25);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_26);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_27);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_28);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_29);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_30);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_31);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_32);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_33);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_34);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_35);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_36);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_37);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_38);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_39);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_40);
                range_check_9_9_state.add_inputs(&range_check_9_9_inputs_41);
            },
        );

    verify_mul_252_state.add_inputs(
        &verify_mul_252_inputs_vec
            .into_iter()
            .flatten()
            .into_iter()
            .flatten()
            .collect_vec(),
    );

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    cube_252_0: Vec<[PackedM31; 20]>,
    range_check_9_9_0: Vec<[PackedM31; 2]>,
    range_check_9_9_1: Vec<[PackedM31; 2]>,
    range_check_9_9_2: Vec<[PackedM31; 2]>,
    range_check_9_9_3: Vec<[PackedM31; 2]>,
    range_check_9_9_4: Vec<[PackedM31; 2]>,
    range_check_9_9_5: Vec<[PackedM31; 2]>,
    range_check_9_9_6: Vec<[PackedM31; 2]>,
    range_check_9_9_7: Vec<[PackedM31; 2]>,
    range_check_9_9_8: Vec<[PackedM31; 2]>,
    range_check_9_9_9: Vec<[PackedM31; 2]>,
    range_check_9_9_10: Vec<[PackedM31; 2]>,
    range_check_9_9_11: Vec<[PackedM31; 2]>,
    range_check_9_9_12: Vec<[PackedM31; 2]>,
    range_check_9_9_13: Vec<[PackedM31; 2]>,
    range_check_9_9_14: Vec<[PackedM31; 2]>,
    range_check_9_9_15: Vec<[PackedM31; 2]>,
    range_check_9_9_16: Vec<[PackedM31; 2]>,
    range_check_9_9_17: Vec<[PackedM31; 2]>,
    range_check_9_9_18: Vec<[PackedM31; 2]>,
    range_check_9_9_19: Vec<[PackedM31; 2]>,
    range_check_9_9_20: Vec<[PackedM31; 2]>,
    range_check_9_9_21: Vec<[PackedM31; 2]>,
    range_check_9_9_22: Vec<[PackedM31; 2]>,
    range_check_9_9_23: Vec<[PackedM31; 2]>,
    range_check_9_9_24: Vec<[PackedM31; 2]>,
    range_check_9_9_25: Vec<[PackedM31; 2]>,
    range_check_9_9_26: Vec<[PackedM31; 2]>,
    range_check_9_9_27: Vec<[PackedM31; 2]>,
    range_check_9_9_28: Vec<[PackedM31; 2]>,
    range_check_9_9_29: Vec<[PackedM31; 2]>,
    range_check_9_9_30: Vec<[PackedM31; 2]>,
    range_check_9_9_31: Vec<[PackedM31; 2]>,
    range_check_9_9_32: Vec<[PackedM31; 2]>,
    range_check_9_9_33: Vec<[PackedM31; 2]>,
    range_check_9_9_34: Vec<[PackedM31; 2]>,
    range_check_9_9_35: Vec<[PackedM31; 2]>,
    range_check_9_9_36: Vec<[PackedM31; 2]>,
    range_check_9_9_37: Vec<[PackedM31; 2]>,
    range_check_9_9_38: Vec<[PackedM31; 2]>,
    range_check_9_9_39: Vec<[PackedM31; 2]>,
    range_check_9_9_40: Vec<[PackedM31; 2]>,
    range_check_9_9_41: Vec<[PackedM31; 2]>,
    verify_mul_252_0: Vec<[PackedM31; 84]>,
    verify_mul_252_1: Vec<[PackedM31; 84]>,
}

pub struct InteractionClaimGenerator {
    n_rows: usize,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        cube_252: &relations::Cube252,
        range_check_9_9: &relations::RangeCheck_9_9,
        verify_mul_252: &relations::VerifyMul252,
    ) -> InteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let log_size = std::cmp::max(self.n_rows.next_power_of_two().ilog2(), LOG_N_LANES);
        let padding_col = Enabler::new(self.n_rows);
        let mut logup_gen = LogupTraceGenerator::new(log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_9_9_0,
            &self.lookup_data.range_check_9_9_1,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_9_9_2,
            &self.lookup_data.range_check_9_9_3,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_9_9_4,
            &self.lookup_data.range_check_9_9_5,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_9_9_6,
            &self.lookup_data.range_check_9_9_7,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_9_9_8,
            &self.lookup_data.range_check_9_9_9,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_9_9_10,
            &self.lookup_data.range_check_9_9_11,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_9_9_12,
            &self.lookup_data.range_check_9_9_13,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_9_9_14,
            &self.lookup_data.range_check_9_9_15,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_9_9_16,
            &self.lookup_data.range_check_9_9_17,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_9_9_18,
            &self.lookup_data.range_check_9_9_19,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_9_9_20,
            &self.lookup_data.range_check_9_9_21,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_9_9_22,
            &self.lookup_data.range_check_9_9_23,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_9_9_24,
            &self.lookup_data.range_check_9_9_25,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_9_9_26,
            &self.lookup_data.range_check_9_9_27,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_mul_252_0,
            &self.lookup_data.range_check_9_9_28,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_mul_252.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_9_9_29,
            &self.lookup_data.range_check_9_9_30,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_9_9_31,
            &self.lookup_data.range_check_9_9_32,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_9_9_33,
            &self.lookup_data.range_check_9_9_34,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_9_9_35,
            &self.lookup_data.range_check_9_9_36,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_9_9_37,
            &self.lookup_data.range_check_9_9_38,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_9_9_39,
            &self.lookup_data.range_check_9_9_40,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_9_9_41,
            &self.lookup_data.verify_mul_252_1,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = verify_mul_252.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        for (i, values) in self.lookup_data.cube_252_0.iter().enumerate() {
            let denom = cube_252.combine(values);
            col_gen.write_frac(i, -PackedQM31::one() * padding_col.packed_at(i), denom);
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
