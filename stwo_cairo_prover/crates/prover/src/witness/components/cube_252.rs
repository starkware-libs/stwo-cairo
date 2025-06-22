#![allow(unused_parens)]
use cairo_air::components::cube_252::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{
    range_check_19, range_check_19_b, range_check_19_c, range_check_19_d, range_check_9_9,
    range_check_9_9_b, range_check_9_9_c, range_check_9_9_d,
};
use crate::witness::prelude::*;

pub type PackedInputType = PackedFelt252Width27;

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

    pub fn write_trace(
        mut self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        range_check_19_state: &range_check_19::ClaimGenerator,
        range_check_19_b_state: &range_check_19_b::ClaimGenerator,
        range_check_19_c_state: &range_check_19_c::ClaimGenerator,
        range_check_19_d_state: &range_check_19_d::ClaimGenerator,
        range_check_9_9_state: &range_check_9_9::ClaimGenerator,
        range_check_9_9_b_state: &range_check_9_9_b::ClaimGenerator,
        range_check_9_9_c_state: &range_check_9_9_c::ClaimGenerator,
        range_check_9_9_d_state: &range_check_9_9_d::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator) {
        assert!(!self.packed_inputs.is_empty());
        let n_vec_rows = self.packed_inputs.len();
        let n_rows = n_vec_rows * N_LANES;
        let packed_size = n_vec_rows.next_power_of_two();
        let log_size = packed_size.ilog2() + LOG_N_LANES;
        self.packed_inputs
            .resize(packed_size, *self.packed_inputs.first().unwrap());

        // Decreasing this value may cause a stack-overflow during witness generation.
        // NOTE: This is not autogened, when updating the code, re-add this.
        // TODO(Ohad): remove.
        const RAYON_THREAD_STACK_SIZE: usize = 1024 * 1024 * 8;
        let pool = rayon::ThreadPoolBuilder::new()
            .stack_size(RAYON_THREAD_STACK_SIZE)
            .build()
            .unwrap();
        let (trace, lookup_data, sub_component_inputs) = pool.install(|| {
            write_trace_simd(
                self.packed_inputs,
                n_rows,
                range_check_19_state,
                range_check_19_b_state,
                range_check_19_c_state,
                range_check_19_d_state,
                range_check_9_9_state,
                range_check_9_9_b_state,
                range_check_9_9_c_state,
                range_check_9_9_d_state,
            )
        });
        sub_component_inputs
            .range_check_9_9
            .iter()
            .for_each(|inputs| {
                range_check_9_9_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_9_9_b
            .iter()
            .for_each(|inputs| {
                range_check_9_9_b_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_9_9_c
            .iter()
            .for_each(|inputs| {
                range_check_9_9_c_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_9_9_d
            .iter()
            .for_each(|inputs| {
                range_check_9_9_d_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_19
            .iter()
            .for_each(|inputs| {
                range_check_19_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_19_b
            .iter()
            .for_each(|inputs| {
                range_check_19_b_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_19_c
            .iter()
            .for_each(|inputs| {
                range_check_19_c_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_19_d
            .iter()
            .for_each(|inputs| {
                range_check_19_d_state.add_packed_inputs(inputs);
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
    range_check_9_9: [Vec<range_check_9_9::PackedInputType>; 12],
    range_check_9_9_b: [Vec<range_check_9_9_b::PackedInputType>; 12],
    range_check_9_9_c: [Vec<range_check_9_9_c::PackedInputType>; 9],
    range_check_9_9_d: [Vec<range_check_9_9_d::PackedInputType>; 9],
    range_check_19: [Vec<range_check_19::PackedInputType>; 16],
    range_check_19_b: [Vec<range_check_19_b::PackedInputType>; 14],
    range_check_19_c: [Vec<range_check_19_c::PackedInputType>; 14],
    range_check_19_d: [Vec<range_check_19_d::PackedInputType>; 12],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    n_rows: usize,
    range_check_19_state: &range_check_19::ClaimGenerator,
    range_check_19_b_state: &range_check_19_b::ClaimGenerator,
    range_check_19_c_state: &range_check_19_c::ClaimGenerator,
    range_check_19_d_state: &range_check_19_d::ClaimGenerator,
    range_check_9_9_state: &range_check_9_9::ClaimGenerator,
    range_check_9_9_b_state: &range_check_9_9_b::ClaimGenerator,
    range_check_9_9_c_state: &range_check_9_9_c::ClaimGenerator,
    range_check_9_9_d_state: &range_check_9_9_d::ClaimGenerator,
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
    let M31_131072 = PackedM31::broadcast(M31::from(131072));
    let M31_134217728 = PackedM31::broadcast(M31::from(134217728));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_4 = PackedM31::broadcast(M31::from(4));
    let M31_4194304 = PackedM31::broadcast(M31::from(4194304));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let M31_64 = PackedM31::broadcast(M31::from(64));
    let M31_65536 = PackedM31::broadcast(M31::from(65536));
    let M31_8 = PackedM31::broadcast(M31::from(8));
    let M31_8192 = PackedM31::broadcast(M31::from(8192));
    let UInt32_262143 = PackedUInt32::broadcast(UInt32::from(262143));
    let UInt32_511 = PackedUInt32::broadcast(UInt32::from(511));
    let UInt32_65536 = PackedUInt32::broadcast(UInt32::from(65536));
    let UInt32_9 = PackedUInt32::broadcast(UInt32::from(9));
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
            |(row_index, (mut row, lookup_data, sub_component_inputs, cube_252_input))| {
                let input_limb_0_col0 = cube_252_input.get_m31(0);
                *row[0] = input_limb_0_col0;
                let input_limb_1_col1 = cube_252_input.get_m31(1);
                *row[1] = input_limb_1_col1;
                let input_limb_2_col2 = cube_252_input.get_m31(2);
                *row[2] = input_limb_2_col2;
                let input_limb_3_col3 = cube_252_input.get_m31(3);
                *row[3] = input_limb_3_col3;
                let input_limb_4_col4 = cube_252_input.get_m31(4);
                *row[4] = input_limb_4_col4;
                let input_limb_5_col5 = cube_252_input.get_m31(5);
                *row[5] = input_limb_5_col5;
                let input_limb_6_col6 = cube_252_input.get_m31(6);
                *row[6] = input_limb_6_col6;
                let input_limb_7_col7 = cube_252_input.get_m31(7);
                *row[7] = input_limb_7_col7;
                let input_limb_8_col8 = cube_252_input.get_m31(8);
                *row[8] = input_limb_8_col8;
                let input_limb_9_col9 = cube_252_input.get_m31(9);
                *row[9] = input_limb_9_col9;

                // Felt 252 Unpack From 27 Range Check Output.

                let input_as_felt252_tmp_fec87_0 =
                    PackedFelt252::from_packed_felt252width27(cube_252_input);
                let unpacked_limb_0_col10 = input_as_felt252_tmp_fec87_0.get_m31(0);
                *row[10] = unpacked_limb_0_col10;
                let unpacked_limb_1_col11 = input_as_felt252_tmp_fec87_0.get_m31(1);
                *row[11] = unpacked_limb_1_col11;
                let unpacked_limb_3_col12 = input_as_felt252_tmp_fec87_0.get_m31(3);
                *row[12] = unpacked_limb_3_col12;
                let unpacked_limb_4_col13 = input_as_felt252_tmp_fec87_0.get_m31(4);
                *row[13] = unpacked_limb_4_col13;
                let unpacked_limb_6_col14 = input_as_felt252_tmp_fec87_0.get_m31(6);
                *row[14] = unpacked_limb_6_col14;
                let unpacked_limb_7_col15 = input_as_felt252_tmp_fec87_0.get_m31(7);
                *row[15] = unpacked_limb_7_col15;
                let unpacked_limb_9_col16 = input_as_felt252_tmp_fec87_0.get_m31(9);
                *row[16] = unpacked_limb_9_col16;
                let unpacked_limb_10_col17 = input_as_felt252_tmp_fec87_0.get_m31(10);
                *row[17] = unpacked_limb_10_col17;
                let unpacked_limb_12_col18 = input_as_felt252_tmp_fec87_0.get_m31(12);
                *row[18] = unpacked_limb_12_col18;
                let unpacked_limb_13_col19 = input_as_felt252_tmp_fec87_0.get_m31(13);
                *row[19] = unpacked_limb_13_col19;
                let unpacked_limb_15_col20 = input_as_felt252_tmp_fec87_0.get_m31(15);
                *row[20] = unpacked_limb_15_col20;
                let unpacked_limb_16_col21 = input_as_felt252_tmp_fec87_0.get_m31(16);
                *row[21] = unpacked_limb_16_col21;
                let unpacked_limb_18_col22 = input_as_felt252_tmp_fec87_0.get_m31(18);
                *row[22] = unpacked_limb_18_col22;
                let unpacked_limb_19_col23 = input_as_felt252_tmp_fec87_0.get_m31(19);
                *row[23] = unpacked_limb_19_col23;
                let unpacked_limb_21_col24 = input_as_felt252_tmp_fec87_0.get_m31(21);
                *row[24] = unpacked_limb_21_col24;
                let unpacked_limb_22_col25 = input_as_felt252_tmp_fec87_0.get_m31(22);
                *row[25] = unpacked_limb_22_col25;
                let unpacked_limb_24_col26 = input_as_felt252_tmp_fec87_0.get_m31(24);
                *row[26] = unpacked_limb_24_col26;
                let unpacked_limb_25_col27 = input_as_felt252_tmp_fec87_0.get_m31(25);
                *row[27] = unpacked_limb_25_col27;
                let unpacked_tmp_fec87_1 = PackedFelt252::from_limbs([
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

                // Range Check Mem Value N 28.

                *sub_component_inputs.range_check_9_9[0] =
                    [unpacked_limb_0_col10, unpacked_limb_1_col11];
                *lookup_data.range_check_9_9_0 = [unpacked_limb_0_col10, unpacked_limb_1_col11];
                *sub_component_inputs.range_check_9_9_b[0] =
                    [unpacked_tmp_fec87_1.get_m31(2), unpacked_limb_3_col12];
                *lookup_data.range_check_9_9_b_0 =
                    [unpacked_tmp_fec87_1.get_m31(2), unpacked_limb_3_col12];
                *sub_component_inputs.range_check_9_9_c[0] =
                    [unpacked_limb_4_col13, unpacked_tmp_fec87_1.get_m31(5)];
                *lookup_data.range_check_9_9_c_0 =
                    [unpacked_limb_4_col13, unpacked_tmp_fec87_1.get_m31(5)];
                *sub_component_inputs.range_check_9_9_d[0] =
                    [unpacked_limb_6_col14, unpacked_limb_7_col15];
                *lookup_data.range_check_9_9_d_0 = [unpacked_limb_6_col14, unpacked_limb_7_col15];
                *sub_component_inputs.range_check_9_9[1] =
                    [unpacked_tmp_fec87_1.get_m31(8), unpacked_limb_9_col16];
                *lookup_data.range_check_9_9_1 =
                    [unpacked_tmp_fec87_1.get_m31(8), unpacked_limb_9_col16];
                *sub_component_inputs.range_check_9_9_b[1] =
                    [unpacked_limb_10_col17, unpacked_tmp_fec87_1.get_m31(11)];
                *lookup_data.range_check_9_9_b_1 =
                    [unpacked_limb_10_col17, unpacked_tmp_fec87_1.get_m31(11)];
                *sub_component_inputs.range_check_9_9_c[1] =
                    [unpacked_limb_12_col18, unpacked_limb_13_col19];
                *lookup_data.range_check_9_9_c_1 = [unpacked_limb_12_col18, unpacked_limb_13_col19];
                *sub_component_inputs.range_check_9_9_d[1] =
                    [unpacked_tmp_fec87_1.get_m31(14), unpacked_limb_15_col20];
                *lookup_data.range_check_9_9_d_1 =
                    [unpacked_tmp_fec87_1.get_m31(14), unpacked_limb_15_col20];
                *sub_component_inputs.range_check_9_9[2] =
                    [unpacked_limb_16_col21, unpacked_tmp_fec87_1.get_m31(17)];
                *lookup_data.range_check_9_9_2 =
                    [unpacked_limb_16_col21, unpacked_tmp_fec87_1.get_m31(17)];
                *sub_component_inputs.range_check_9_9_b[2] =
                    [unpacked_limb_18_col22, unpacked_limb_19_col23];
                *lookup_data.range_check_9_9_b_2 = [unpacked_limb_18_col22, unpacked_limb_19_col23];
                *sub_component_inputs.range_check_9_9_c[2] =
                    [unpacked_tmp_fec87_1.get_m31(20), unpacked_limb_21_col24];
                *lookup_data.range_check_9_9_c_2 =
                    [unpacked_tmp_fec87_1.get_m31(20), unpacked_limb_21_col24];
                *sub_component_inputs.range_check_9_9_d[2] =
                    [unpacked_limb_22_col25, unpacked_tmp_fec87_1.get_m31(23)];
                *lookup_data.range_check_9_9_d_2 =
                    [unpacked_limb_22_col25, unpacked_tmp_fec87_1.get_m31(23)];
                *sub_component_inputs.range_check_9_9[3] =
                    [unpacked_limb_24_col26, unpacked_limb_25_col27];
                *lookup_data.range_check_9_9_3 = [unpacked_limb_24_col26, unpacked_limb_25_col27];
                *sub_component_inputs.range_check_9_9_b[3] =
                    [unpacked_tmp_fec87_1.get_m31(26), input_limb_9_col9];
                *lookup_data.range_check_9_9_b_3 =
                    [unpacked_tmp_fec87_1.get_m31(26), input_limb_9_col9];

                let felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2 =
                    unpacked_tmp_fec87_1;

                // Mul 252.

                let mul_res_tmp_fec87_3 =
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2)
                        * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2));
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

                *sub_component_inputs.range_check_9_9[4] =
                    [mul_res_limb_0_col28, mul_res_limb_1_col29];
                *lookup_data.range_check_9_9_4 = [mul_res_limb_0_col28, mul_res_limb_1_col29];
                *sub_component_inputs.range_check_9_9_b[4] =
                    [mul_res_limb_2_col30, mul_res_limb_3_col31];
                *lookup_data.range_check_9_9_b_4 = [mul_res_limb_2_col30, mul_res_limb_3_col31];
                *sub_component_inputs.range_check_9_9_c[3] =
                    [mul_res_limb_4_col32, mul_res_limb_5_col33];
                *lookup_data.range_check_9_9_c_3 = [mul_res_limb_4_col32, mul_res_limb_5_col33];
                *sub_component_inputs.range_check_9_9_d[3] =
                    [mul_res_limb_6_col34, mul_res_limb_7_col35];
                *lookup_data.range_check_9_9_d_3 = [mul_res_limb_6_col34, mul_res_limb_7_col35];
                *sub_component_inputs.range_check_9_9[5] =
                    [mul_res_limb_8_col36, mul_res_limb_9_col37];
                *lookup_data.range_check_9_9_5 = [mul_res_limb_8_col36, mul_res_limb_9_col37];
                *sub_component_inputs.range_check_9_9_b[5] =
                    [mul_res_limb_10_col38, mul_res_limb_11_col39];
                *lookup_data.range_check_9_9_b_5 = [mul_res_limb_10_col38, mul_res_limb_11_col39];
                *sub_component_inputs.range_check_9_9_c[4] =
                    [mul_res_limb_12_col40, mul_res_limb_13_col41];
                *lookup_data.range_check_9_9_c_4 = [mul_res_limb_12_col40, mul_res_limb_13_col41];
                *sub_component_inputs.range_check_9_9_d[4] =
                    [mul_res_limb_14_col42, mul_res_limb_15_col43];
                *lookup_data.range_check_9_9_d_4 = [mul_res_limb_14_col42, mul_res_limb_15_col43];
                *sub_component_inputs.range_check_9_9[6] =
                    [mul_res_limb_16_col44, mul_res_limb_17_col45];
                *lookup_data.range_check_9_9_6 = [mul_res_limb_16_col44, mul_res_limb_17_col45];
                *sub_component_inputs.range_check_9_9_b[6] =
                    [mul_res_limb_18_col46, mul_res_limb_19_col47];
                *lookup_data.range_check_9_9_b_6 = [mul_res_limb_18_col46, mul_res_limb_19_col47];
                *sub_component_inputs.range_check_9_9_c[5] =
                    [mul_res_limb_20_col48, mul_res_limb_21_col49];
                *lookup_data.range_check_9_9_c_5 = [mul_res_limb_20_col48, mul_res_limb_21_col49];
                *sub_component_inputs.range_check_9_9_d[5] =
                    [mul_res_limb_22_col50, mul_res_limb_23_col51];
                *lookup_data.range_check_9_9_d_5 = [mul_res_limb_22_col50, mul_res_limb_23_col51];
                *sub_component_inputs.range_check_9_9[7] =
                    [mul_res_limb_24_col52, mul_res_limb_25_col53];
                *lookup_data.range_check_9_9_7 = [mul_res_limb_24_col52, mul_res_limb_25_col53];
                *sub_component_inputs.range_check_9_9_b[7] =
                    [mul_res_limb_26_col54, mul_res_limb_27_col55];
                *lookup_data.range_check_9_9_b_7 = [mul_res_limb_26_col54, mul_res_limb_27_col55];

                // Verify Mul 252.

                // Double Karatsuba N 7 Limb Max Bound 511.

                // Single Karatsuba N 7.

                let z0_tmp_fec87_4 = [
                    ((unpacked_limb_0_col10) * (unpacked_limb_0_col10)),
                    (((unpacked_limb_0_col10) * (unpacked_limb_1_col11))
                        + ((unpacked_limb_1_col11) * (unpacked_limb_0_col10))),
                    ((((unpacked_limb_0_col10)
                        * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(2)))
                        + ((unpacked_limb_1_col11) * (unpacked_limb_1_col11)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(2))
                            * (unpacked_limb_0_col10))),
                    (((((unpacked_limb_0_col10) * (unpacked_limb_3_col12))
                        + ((unpacked_limb_1_col11)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(2))))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(2))
                            * (unpacked_limb_1_col11)))
                        + ((unpacked_limb_3_col12) * (unpacked_limb_0_col10))),
                    ((((((unpacked_limb_0_col10) * (unpacked_limb_4_col13))
                        + ((unpacked_limb_1_col11) * (unpacked_limb_3_col12)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(2))
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(2))))
                        + ((unpacked_limb_3_col12) * (unpacked_limb_1_col11)))
                        + ((unpacked_limb_4_col13) * (unpacked_limb_0_col10))),
                    (((((((unpacked_limb_0_col10)
                        * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(5)))
                        + ((unpacked_limb_1_col11) * (unpacked_limb_4_col13)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(2))
                            * (unpacked_limb_3_col12)))
                        + ((unpacked_limb_3_col12)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(2))))
                        + ((unpacked_limb_4_col13) * (unpacked_limb_1_col11)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(5))
                            * (unpacked_limb_0_col10))),
                    ((((((((unpacked_limb_0_col10) * (unpacked_limb_6_col14))
                        + ((unpacked_limb_1_col11)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(5))))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(2))
                            * (unpacked_limb_4_col13)))
                        + ((unpacked_limb_3_col12) * (unpacked_limb_3_col12)))
                        + ((unpacked_limb_4_col13)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(2))))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(5))
                            * (unpacked_limb_1_col11)))
                        + ((unpacked_limb_6_col14) * (unpacked_limb_0_col10))),
                    (((((((unpacked_limb_1_col11) * (unpacked_limb_6_col14))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(2))
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(5))))
                        + ((unpacked_limb_3_col12) * (unpacked_limb_4_col13)))
                        + ((unpacked_limb_4_col13) * (unpacked_limb_3_col12)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(5))
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(2))))
                        + ((unpacked_limb_6_col14) * (unpacked_limb_1_col11))),
                    ((((((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                        .get_m31(2))
                        * (unpacked_limb_6_col14))
                        + ((unpacked_limb_3_col12)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(5))))
                        + ((unpacked_limb_4_col13) * (unpacked_limb_4_col13)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(5))
                            * (unpacked_limb_3_col12)))
                        + ((unpacked_limb_6_col14)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(2)))),
                    (((((unpacked_limb_3_col12) * (unpacked_limb_6_col14))
                        + ((unpacked_limb_4_col13)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(5))))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(5))
                            * (unpacked_limb_4_col13)))
                        + ((unpacked_limb_6_col14) * (unpacked_limb_3_col12))),
                    ((((unpacked_limb_4_col13) * (unpacked_limb_6_col14))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(5))
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(5))))
                        + ((unpacked_limb_6_col14) * (unpacked_limb_4_col13))),
                    (((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(5))
                        * (unpacked_limb_6_col14))
                        + ((unpacked_limb_6_col14)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(5)))),
                    ((unpacked_limb_6_col14) * (unpacked_limb_6_col14)),
                ];
                let z2_tmp_fec87_5 = [
                    ((unpacked_limb_7_col15) * (unpacked_limb_7_col15)),
                    (((unpacked_limb_7_col15)
                        * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(8)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(8))
                            * (unpacked_limb_7_col15))),
                    ((((unpacked_limb_7_col15) * (unpacked_limb_9_col16))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(8))
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(8))))
                        + ((unpacked_limb_9_col16) * (unpacked_limb_7_col15))),
                    (((((unpacked_limb_7_col15) * (unpacked_limb_10_col17))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(8))
                            * (unpacked_limb_9_col16)))
                        + ((unpacked_limb_9_col16)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(8))))
                        + ((unpacked_limb_10_col17) * (unpacked_limb_7_col15))),
                    ((((((unpacked_limb_7_col15)
                        * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(11)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(8))
                            * (unpacked_limb_10_col17)))
                        + ((unpacked_limb_9_col16) * (unpacked_limb_9_col16)))
                        + ((unpacked_limb_10_col17)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(8))))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(11))
                            * (unpacked_limb_7_col15))),
                    (((((((unpacked_limb_7_col15) * (unpacked_limb_12_col18))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(8))
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(11))))
                        + ((unpacked_limb_9_col16) * (unpacked_limb_10_col17)))
                        + ((unpacked_limb_10_col17) * (unpacked_limb_9_col16)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(11))
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(8))))
                        + ((unpacked_limb_12_col18) * (unpacked_limb_7_col15))),
                    ((((((((unpacked_limb_7_col15) * (unpacked_limb_13_col19))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(8))
                            * (unpacked_limb_12_col18)))
                        + ((unpacked_limb_9_col16)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(11))))
                        + ((unpacked_limb_10_col17) * (unpacked_limb_10_col17)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(11))
                            * (unpacked_limb_9_col16)))
                        + ((unpacked_limb_12_col18)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(8))))
                        + ((unpacked_limb_13_col19) * (unpacked_limb_7_col15))),
                    (((((((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                        .get_m31(8))
                        * (unpacked_limb_13_col19))
                        + ((unpacked_limb_9_col16) * (unpacked_limb_12_col18)))
                        + ((unpacked_limb_10_col17)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(11))))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(11))
                            * (unpacked_limb_10_col17)))
                        + ((unpacked_limb_12_col18) * (unpacked_limb_9_col16)))
                        + ((unpacked_limb_13_col19)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(8)))),
                    ((((((unpacked_limb_9_col16) * (unpacked_limb_13_col19))
                        + ((unpacked_limb_10_col17) * (unpacked_limb_12_col18)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(11))
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(11))))
                        + ((unpacked_limb_12_col18) * (unpacked_limb_10_col17)))
                        + ((unpacked_limb_13_col19) * (unpacked_limb_9_col16))),
                    (((((unpacked_limb_10_col17) * (unpacked_limb_13_col19))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(11))
                            * (unpacked_limb_12_col18)))
                        + ((unpacked_limb_12_col18)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(11))))
                        + ((unpacked_limb_13_col19) * (unpacked_limb_10_col17))),
                    ((((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                        .get_m31(11))
                        * (unpacked_limb_13_col19))
                        + ((unpacked_limb_12_col18) * (unpacked_limb_12_col18)))
                        + ((unpacked_limb_13_col19)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(11)))),
                    (((unpacked_limb_12_col18) * (unpacked_limb_13_col19))
                        + ((unpacked_limb_13_col19) * (unpacked_limb_12_col18))),
                    ((unpacked_limb_13_col19) * (unpacked_limb_13_col19)),
                ];
                let x_sum_tmp_fec87_6 = [
                    ((unpacked_limb_0_col10) + (unpacked_limb_7_col15)),
                    ((unpacked_limb_1_col11)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(8))),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(2))
                        + (unpacked_limb_9_col16)),
                    ((unpacked_limb_3_col12) + (unpacked_limb_10_col17)),
                    ((unpacked_limb_4_col13)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(11))),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(5))
                        + (unpacked_limb_12_col18)),
                    ((unpacked_limb_6_col14) + (unpacked_limb_13_col19)),
                ];
                let y_sum_tmp_fec87_7 = [
                    ((unpacked_limb_0_col10) + (unpacked_limb_7_col15)),
                    ((unpacked_limb_1_col11)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(8))),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(2))
                        + (unpacked_limb_9_col16)),
                    ((unpacked_limb_3_col12) + (unpacked_limb_10_col17)),
                    ((unpacked_limb_4_col13)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(11))),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(5))
                        + (unpacked_limb_12_col18)),
                    ((unpacked_limb_6_col14) + (unpacked_limb_13_col19)),
                ];
                let single_karatsuba_n_7_output_tmp_fec87_8 = [
                    z0_tmp_fec87_4[0],
                    z0_tmp_fec87_4[1],
                    z0_tmp_fec87_4[2],
                    z0_tmp_fec87_4[3],
                    z0_tmp_fec87_4[4],
                    z0_tmp_fec87_4[5],
                    z0_tmp_fec87_4[6],
                    ((z0_tmp_fec87_4[7])
                        + ((((x_sum_tmp_fec87_6[0]) * (y_sum_tmp_fec87_7[0]))
                            - (z0_tmp_fec87_4[0]))
                            - (z2_tmp_fec87_5[0]))),
                    ((z0_tmp_fec87_4[8])
                        + (((((x_sum_tmp_fec87_6[0]) * (y_sum_tmp_fec87_7[1]))
                            + ((x_sum_tmp_fec87_6[1]) * (y_sum_tmp_fec87_7[0])))
                            - (z0_tmp_fec87_4[1]))
                            - (z2_tmp_fec87_5[1]))),
                    ((z0_tmp_fec87_4[9])
                        + ((((((x_sum_tmp_fec87_6[0]) * (y_sum_tmp_fec87_7[2]))
                            + ((x_sum_tmp_fec87_6[1]) * (y_sum_tmp_fec87_7[1])))
                            + ((x_sum_tmp_fec87_6[2]) * (y_sum_tmp_fec87_7[0])))
                            - (z0_tmp_fec87_4[2]))
                            - (z2_tmp_fec87_5[2]))),
                    ((z0_tmp_fec87_4[10])
                        + (((((((x_sum_tmp_fec87_6[0]) * (y_sum_tmp_fec87_7[3]))
                            + ((x_sum_tmp_fec87_6[1]) * (y_sum_tmp_fec87_7[2])))
                            + ((x_sum_tmp_fec87_6[2]) * (y_sum_tmp_fec87_7[1])))
                            + ((x_sum_tmp_fec87_6[3]) * (y_sum_tmp_fec87_7[0])))
                            - (z0_tmp_fec87_4[3]))
                            - (z2_tmp_fec87_5[3]))),
                    ((z0_tmp_fec87_4[11])
                        + ((((((((x_sum_tmp_fec87_6[0]) * (y_sum_tmp_fec87_7[4]))
                            + ((x_sum_tmp_fec87_6[1]) * (y_sum_tmp_fec87_7[3])))
                            + ((x_sum_tmp_fec87_6[2]) * (y_sum_tmp_fec87_7[2])))
                            + ((x_sum_tmp_fec87_6[3]) * (y_sum_tmp_fec87_7[1])))
                            + ((x_sum_tmp_fec87_6[4]) * (y_sum_tmp_fec87_7[0])))
                            - (z0_tmp_fec87_4[4]))
                            - (z2_tmp_fec87_5[4]))),
                    ((z0_tmp_fec87_4[12])
                        + (((((((((x_sum_tmp_fec87_6[0]) * (y_sum_tmp_fec87_7[5]))
                            + ((x_sum_tmp_fec87_6[1]) * (y_sum_tmp_fec87_7[4])))
                            + ((x_sum_tmp_fec87_6[2]) * (y_sum_tmp_fec87_7[3])))
                            + ((x_sum_tmp_fec87_6[3]) * (y_sum_tmp_fec87_7[2])))
                            + ((x_sum_tmp_fec87_6[4]) * (y_sum_tmp_fec87_7[1])))
                            + ((x_sum_tmp_fec87_6[5]) * (y_sum_tmp_fec87_7[0])))
                            - (z0_tmp_fec87_4[5]))
                            - (z2_tmp_fec87_5[5]))),
                    ((((((((((x_sum_tmp_fec87_6[0]) * (y_sum_tmp_fec87_7[6]))
                        + ((x_sum_tmp_fec87_6[1]) * (y_sum_tmp_fec87_7[5])))
                        + ((x_sum_tmp_fec87_6[2]) * (y_sum_tmp_fec87_7[4])))
                        + ((x_sum_tmp_fec87_6[3]) * (y_sum_tmp_fec87_7[3])))
                        + ((x_sum_tmp_fec87_6[4]) * (y_sum_tmp_fec87_7[2])))
                        + ((x_sum_tmp_fec87_6[5]) * (y_sum_tmp_fec87_7[1])))
                        + ((x_sum_tmp_fec87_6[6]) * (y_sum_tmp_fec87_7[0])))
                        - (z0_tmp_fec87_4[6]))
                        - (z2_tmp_fec87_5[6])),
                    ((z2_tmp_fec87_5[0])
                        + (((((((((x_sum_tmp_fec87_6[1]) * (y_sum_tmp_fec87_7[6]))
                            + ((x_sum_tmp_fec87_6[2]) * (y_sum_tmp_fec87_7[5])))
                            + ((x_sum_tmp_fec87_6[3]) * (y_sum_tmp_fec87_7[4])))
                            + ((x_sum_tmp_fec87_6[4]) * (y_sum_tmp_fec87_7[3])))
                            + ((x_sum_tmp_fec87_6[5]) * (y_sum_tmp_fec87_7[2])))
                            + ((x_sum_tmp_fec87_6[6]) * (y_sum_tmp_fec87_7[1])))
                            - (z0_tmp_fec87_4[7]))
                            - (z2_tmp_fec87_5[7]))),
                    ((z2_tmp_fec87_5[1])
                        + ((((((((x_sum_tmp_fec87_6[2]) * (y_sum_tmp_fec87_7[6]))
                            + ((x_sum_tmp_fec87_6[3]) * (y_sum_tmp_fec87_7[5])))
                            + ((x_sum_tmp_fec87_6[4]) * (y_sum_tmp_fec87_7[4])))
                            + ((x_sum_tmp_fec87_6[5]) * (y_sum_tmp_fec87_7[3])))
                            + ((x_sum_tmp_fec87_6[6]) * (y_sum_tmp_fec87_7[2])))
                            - (z0_tmp_fec87_4[8]))
                            - (z2_tmp_fec87_5[8]))),
                    ((z2_tmp_fec87_5[2])
                        + (((((((x_sum_tmp_fec87_6[3]) * (y_sum_tmp_fec87_7[6]))
                            + ((x_sum_tmp_fec87_6[4]) * (y_sum_tmp_fec87_7[5])))
                            + ((x_sum_tmp_fec87_6[5]) * (y_sum_tmp_fec87_7[4])))
                            + ((x_sum_tmp_fec87_6[6]) * (y_sum_tmp_fec87_7[3])))
                            - (z0_tmp_fec87_4[9]))
                            - (z2_tmp_fec87_5[9]))),
                    ((z2_tmp_fec87_5[3])
                        + ((((((x_sum_tmp_fec87_6[4]) * (y_sum_tmp_fec87_7[6]))
                            + ((x_sum_tmp_fec87_6[5]) * (y_sum_tmp_fec87_7[5])))
                            + ((x_sum_tmp_fec87_6[6]) * (y_sum_tmp_fec87_7[4])))
                            - (z0_tmp_fec87_4[10]))
                            - (z2_tmp_fec87_5[10]))),
                    ((z2_tmp_fec87_5[4])
                        + (((((x_sum_tmp_fec87_6[5]) * (y_sum_tmp_fec87_7[6]))
                            + ((x_sum_tmp_fec87_6[6]) * (y_sum_tmp_fec87_7[5])))
                            - (z0_tmp_fec87_4[11]))
                            - (z2_tmp_fec87_5[11]))),
                    ((z2_tmp_fec87_5[5])
                        + ((((x_sum_tmp_fec87_6[6]) * (y_sum_tmp_fec87_7[6]))
                            - (z0_tmp_fec87_4[12]))
                            - (z2_tmp_fec87_5[12]))),
                    z2_tmp_fec87_5[6],
                    z2_tmp_fec87_5[7],
                    z2_tmp_fec87_5[8],
                    z2_tmp_fec87_5[9],
                    z2_tmp_fec87_5[10],
                    z2_tmp_fec87_5[11],
                    z2_tmp_fec87_5[12],
                ];

                // Single Karatsuba N 7.

                let z0_tmp_fec87_9 = [
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(14))
                        * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(14))),
                    (((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(14))
                        * (unpacked_limb_15_col20))
                        + ((unpacked_limb_15_col20)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(14)))),
                    ((((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                        .get_m31(14))
                        * (unpacked_limb_16_col21))
                        + ((unpacked_limb_15_col20) * (unpacked_limb_15_col20)))
                        + ((unpacked_limb_16_col21)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(14)))),
                    (((((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                        .get_m31(14))
                        * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(17)))
                        + ((unpacked_limb_15_col20) * (unpacked_limb_16_col21)))
                        + ((unpacked_limb_16_col21) * (unpacked_limb_15_col20)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(17))
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(14)))),
                    ((((((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                        .get_m31(14))
                        * (unpacked_limb_18_col22))
                        + ((unpacked_limb_15_col20)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(17))))
                        + ((unpacked_limb_16_col21) * (unpacked_limb_16_col21)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(17))
                            * (unpacked_limb_15_col20)))
                        + ((unpacked_limb_18_col22)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(14)))),
                    (((((((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                        .get_m31(14))
                        * (unpacked_limb_19_col23))
                        + ((unpacked_limb_15_col20) * (unpacked_limb_18_col22)))
                        + ((unpacked_limb_16_col21)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(17))))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(17))
                            * (unpacked_limb_16_col21)))
                        + ((unpacked_limb_18_col22) * (unpacked_limb_15_col20)))
                        + ((unpacked_limb_19_col23)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(14)))),
                    ((((((((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                        .get_m31(14))
                        * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(20)))
                        + ((unpacked_limb_15_col20) * (unpacked_limb_19_col23)))
                        + ((unpacked_limb_16_col21) * (unpacked_limb_18_col22)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(17))
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(17))))
                        + ((unpacked_limb_18_col22) * (unpacked_limb_16_col21)))
                        + ((unpacked_limb_19_col23) * (unpacked_limb_15_col20)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(20))
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(14)))),
                    (((((((unpacked_limb_15_col20)
                        * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(20)))
                        + ((unpacked_limb_16_col21) * (unpacked_limb_19_col23)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(17))
                            * (unpacked_limb_18_col22)))
                        + ((unpacked_limb_18_col22)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(17))))
                        + ((unpacked_limb_19_col23) * (unpacked_limb_16_col21)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(20))
                            * (unpacked_limb_15_col20))),
                    ((((((unpacked_limb_16_col21)
                        * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(20)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(17))
                            * (unpacked_limb_19_col23)))
                        + ((unpacked_limb_18_col22) * (unpacked_limb_18_col22)))
                        + ((unpacked_limb_19_col23)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(17))))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(20))
                            * (unpacked_limb_16_col21))),
                    (((((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                        .get_m31(17))
                        * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(20)))
                        + ((unpacked_limb_18_col22) * (unpacked_limb_19_col23)))
                        + ((unpacked_limb_19_col23) * (unpacked_limb_18_col22)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(20))
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(17)))),
                    ((((unpacked_limb_18_col22)
                        * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(20)))
                        + ((unpacked_limb_19_col23) * (unpacked_limb_19_col23)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(20))
                            * (unpacked_limb_18_col22))),
                    (((unpacked_limb_19_col23)
                        * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(20)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(20))
                            * (unpacked_limb_19_col23))),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(20))
                        * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(20))),
                ];
                let z2_tmp_fec87_10 = [
                    ((unpacked_limb_21_col24) * (unpacked_limb_21_col24)),
                    (((unpacked_limb_21_col24) * (unpacked_limb_22_col25))
                        + ((unpacked_limb_22_col25) * (unpacked_limb_21_col24))),
                    ((((unpacked_limb_21_col24)
                        * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(23)))
                        + ((unpacked_limb_22_col25) * (unpacked_limb_22_col25)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(23))
                            * (unpacked_limb_21_col24))),
                    (((((unpacked_limb_21_col24) * (unpacked_limb_24_col26))
                        + ((unpacked_limb_22_col25)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(23))))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(23))
                            * (unpacked_limb_22_col25)))
                        + ((unpacked_limb_24_col26) * (unpacked_limb_21_col24))),
                    ((((((unpacked_limb_21_col24) * (unpacked_limb_25_col27))
                        + ((unpacked_limb_22_col25) * (unpacked_limb_24_col26)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(23))
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(23))))
                        + ((unpacked_limb_24_col26) * (unpacked_limb_22_col25)))
                        + ((unpacked_limb_25_col27) * (unpacked_limb_21_col24))),
                    (((((((unpacked_limb_21_col24)
                        * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(26)))
                        + ((unpacked_limb_22_col25) * (unpacked_limb_25_col27)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(23))
                            * (unpacked_limb_24_col26)))
                        + ((unpacked_limb_24_col26)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(23))))
                        + ((unpacked_limb_25_col27) * (unpacked_limb_22_col25)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(26))
                            * (unpacked_limb_21_col24))),
                    ((((((((unpacked_limb_21_col24) * (input_limb_9_col9))
                        + ((unpacked_limb_22_col25)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(26))))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(23))
                            * (unpacked_limb_25_col27)))
                        + ((unpacked_limb_24_col26) * (unpacked_limb_24_col26)))
                        + ((unpacked_limb_25_col27)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(23))))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(26))
                            * (unpacked_limb_22_col25)))
                        + ((input_limb_9_col9) * (unpacked_limb_21_col24))),
                    (((((((unpacked_limb_22_col25) * (input_limb_9_col9))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(23))
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(26))))
                        + ((unpacked_limb_24_col26) * (unpacked_limb_25_col27)))
                        + ((unpacked_limb_25_col27) * (unpacked_limb_24_col26)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(26))
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(23))))
                        + ((input_limb_9_col9) * (unpacked_limb_22_col25))),
                    ((((((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                        .get_m31(23))
                        * (input_limb_9_col9))
                        + ((unpacked_limb_24_col26)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(26))))
                        + ((unpacked_limb_25_col27) * (unpacked_limb_25_col27)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(26))
                            * (unpacked_limb_24_col26)))
                        + ((input_limb_9_col9)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(23)))),
                    (((((unpacked_limb_24_col26) * (input_limb_9_col9))
                        + ((unpacked_limb_25_col27)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(26))))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(26))
                            * (unpacked_limb_25_col27)))
                        + ((input_limb_9_col9) * (unpacked_limb_24_col26))),
                    ((((unpacked_limb_25_col27) * (input_limb_9_col9))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(26))
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(26))))
                        + ((input_limb_9_col9) * (unpacked_limb_25_col27))),
                    (((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(26))
                        * (input_limb_9_col9))
                        + ((input_limb_9_col9)
                            * (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                                .get_m31(26)))),
                    ((input_limb_9_col9) * (input_limb_9_col9)),
                ];
                let x_sum_tmp_fec87_11 = [
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(14))
                        + (unpacked_limb_21_col24)),
                    ((unpacked_limb_15_col20) + (unpacked_limb_22_col25)),
                    ((unpacked_limb_16_col21)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(23))),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(17))
                        + (unpacked_limb_24_col26)),
                    ((unpacked_limb_18_col22) + (unpacked_limb_25_col27)),
                    ((unpacked_limb_19_col23)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(26))),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(20))
                        + (input_limb_9_col9)),
                ];
                let y_sum_tmp_fec87_12 = [
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(14))
                        + (unpacked_limb_21_col24)),
                    ((unpacked_limb_15_col20) + (unpacked_limb_22_col25)),
                    ((unpacked_limb_16_col21)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(23))),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(17))
                        + (unpacked_limb_24_col26)),
                    ((unpacked_limb_18_col22) + (unpacked_limb_25_col27)),
                    ((unpacked_limb_19_col23)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(26))),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(20))
                        + (input_limb_9_col9)),
                ];
                let single_karatsuba_n_7_output_tmp_fec87_13 = [
                    z0_tmp_fec87_9[0],
                    z0_tmp_fec87_9[1],
                    z0_tmp_fec87_9[2],
                    z0_tmp_fec87_9[3],
                    z0_tmp_fec87_9[4],
                    z0_tmp_fec87_9[5],
                    z0_tmp_fec87_9[6],
                    ((z0_tmp_fec87_9[7])
                        + ((((x_sum_tmp_fec87_11[0]) * (y_sum_tmp_fec87_12[0]))
                            - (z0_tmp_fec87_9[0]))
                            - (z2_tmp_fec87_10[0]))),
                    ((z0_tmp_fec87_9[8])
                        + (((((x_sum_tmp_fec87_11[0]) * (y_sum_tmp_fec87_12[1]))
                            + ((x_sum_tmp_fec87_11[1]) * (y_sum_tmp_fec87_12[0])))
                            - (z0_tmp_fec87_9[1]))
                            - (z2_tmp_fec87_10[1]))),
                    ((z0_tmp_fec87_9[9])
                        + ((((((x_sum_tmp_fec87_11[0]) * (y_sum_tmp_fec87_12[2]))
                            + ((x_sum_tmp_fec87_11[1]) * (y_sum_tmp_fec87_12[1])))
                            + ((x_sum_tmp_fec87_11[2]) * (y_sum_tmp_fec87_12[0])))
                            - (z0_tmp_fec87_9[2]))
                            - (z2_tmp_fec87_10[2]))),
                    ((z0_tmp_fec87_9[10])
                        + (((((((x_sum_tmp_fec87_11[0]) * (y_sum_tmp_fec87_12[3]))
                            + ((x_sum_tmp_fec87_11[1]) * (y_sum_tmp_fec87_12[2])))
                            + ((x_sum_tmp_fec87_11[2]) * (y_sum_tmp_fec87_12[1])))
                            + ((x_sum_tmp_fec87_11[3]) * (y_sum_tmp_fec87_12[0])))
                            - (z0_tmp_fec87_9[3]))
                            - (z2_tmp_fec87_10[3]))),
                    ((z0_tmp_fec87_9[11])
                        + ((((((((x_sum_tmp_fec87_11[0]) * (y_sum_tmp_fec87_12[4]))
                            + ((x_sum_tmp_fec87_11[1]) * (y_sum_tmp_fec87_12[3])))
                            + ((x_sum_tmp_fec87_11[2]) * (y_sum_tmp_fec87_12[2])))
                            + ((x_sum_tmp_fec87_11[3]) * (y_sum_tmp_fec87_12[1])))
                            + ((x_sum_tmp_fec87_11[4]) * (y_sum_tmp_fec87_12[0])))
                            - (z0_tmp_fec87_9[4]))
                            - (z2_tmp_fec87_10[4]))),
                    ((z0_tmp_fec87_9[12])
                        + (((((((((x_sum_tmp_fec87_11[0]) * (y_sum_tmp_fec87_12[5]))
                            + ((x_sum_tmp_fec87_11[1]) * (y_sum_tmp_fec87_12[4])))
                            + ((x_sum_tmp_fec87_11[2]) * (y_sum_tmp_fec87_12[3])))
                            + ((x_sum_tmp_fec87_11[3]) * (y_sum_tmp_fec87_12[2])))
                            + ((x_sum_tmp_fec87_11[4]) * (y_sum_tmp_fec87_12[1])))
                            + ((x_sum_tmp_fec87_11[5]) * (y_sum_tmp_fec87_12[0])))
                            - (z0_tmp_fec87_9[5]))
                            - (z2_tmp_fec87_10[5]))),
                    ((((((((((x_sum_tmp_fec87_11[0]) * (y_sum_tmp_fec87_12[6]))
                        + ((x_sum_tmp_fec87_11[1]) * (y_sum_tmp_fec87_12[5])))
                        + ((x_sum_tmp_fec87_11[2]) * (y_sum_tmp_fec87_12[4])))
                        + ((x_sum_tmp_fec87_11[3]) * (y_sum_tmp_fec87_12[3])))
                        + ((x_sum_tmp_fec87_11[4]) * (y_sum_tmp_fec87_12[2])))
                        + ((x_sum_tmp_fec87_11[5]) * (y_sum_tmp_fec87_12[1])))
                        + ((x_sum_tmp_fec87_11[6]) * (y_sum_tmp_fec87_12[0])))
                        - (z0_tmp_fec87_9[6]))
                        - (z2_tmp_fec87_10[6])),
                    ((z2_tmp_fec87_10[0])
                        + (((((((((x_sum_tmp_fec87_11[1]) * (y_sum_tmp_fec87_12[6]))
                            + ((x_sum_tmp_fec87_11[2]) * (y_sum_tmp_fec87_12[5])))
                            + ((x_sum_tmp_fec87_11[3]) * (y_sum_tmp_fec87_12[4])))
                            + ((x_sum_tmp_fec87_11[4]) * (y_sum_tmp_fec87_12[3])))
                            + ((x_sum_tmp_fec87_11[5]) * (y_sum_tmp_fec87_12[2])))
                            + ((x_sum_tmp_fec87_11[6]) * (y_sum_tmp_fec87_12[1])))
                            - (z0_tmp_fec87_9[7]))
                            - (z2_tmp_fec87_10[7]))),
                    ((z2_tmp_fec87_10[1])
                        + ((((((((x_sum_tmp_fec87_11[2]) * (y_sum_tmp_fec87_12[6]))
                            + ((x_sum_tmp_fec87_11[3]) * (y_sum_tmp_fec87_12[5])))
                            + ((x_sum_tmp_fec87_11[4]) * (y_sum_tmp_fec87_12[4])))
                            + ((x_sum_tmp_fec87_11[5]) * (y_sum_tmp_fec87_12[3])))
                            + ((x_sum_tmp_fec87_11[6]) * (y_sum_tmp_fec87_12[2])))
                            - (z0_tmp_fec87_9[8]))
                            - (z2_tmp_fec87_10[8]))),
                    ((z2_tmp_fec87_10[2])
                        + (((((((x_sum_tmp_fec87_11[3]) * (y_sum_tmp_fec87_12[6]))
                            + ((x_sum_tmp_fec87_11[4]) * (y_sum_tmp_fec87_12[5])))
                            + ((x_sum_tmp_fec87_11[5]) * (y_sum_tmp_fec87_12[4])))
                            + ((x_sum_tmp_fec87_11[6]) * (y_sum_tmp_fec87_12[3])))
                            - (z0_tmp_fec87_9[9]))
                            - (z2_tmp_fec87_10[9]))),
                    ((z2_tmp_fec87_10[3])
                        + ((((((x_sum_tmp_fec87_11[4]) * (y_sum_tmp_fec87_12[6]))
                            + ((x_sum_tmp_fec87_11[5]) * (y_sum_tmp_fec87_12[5])))
                            + ((x_sum_tmp_fec87_11[6]) * (y_sum_tmp_fec87_12[4])))
                            - (z0_tmp_fec87_9[10]))
                            - (z2_tmp_fec87_10[10]))),
                    ((z2_tmp_fec87_10[4])
                        + (((((x_sum_tmp_fec87_11[5]) * (y_sum_tmp_fec87_12[6]))
                            + ((x_sum_tmp_fec87_11[6]) * (y_sum_tmp_fec87_12[5])))
                            - (z0_tmp_fec87_9[11]))
                            - (z2_tmp_fec87_10[11]))),
                    ((z2_tmp_fec87_10[5])
                        + ((((x_sum_tmp_fec87_11[6]) * (y_sum_tmp_fec87_12[6]))
                            - (z0_tmp_fec87_9[12]))
                            - (z2_tmp_fec87_10[12]))),
                    z2_tmp_fec87_10[6],
                    z2_tmp_fec87_10[7],
                    z2_tmp_fec87_10[8],
                    z2_tmp_fec87_10[9],
                    z2_tmp_fec87_10[10],
                    z2_tmp_fec87_10[11],
                    z2_tmp_fec87_10[12],
                ];

                let x_sum_tmp_fec87_14 = [
                    ((unpacked_limb_0_col10)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(14))),
                    ((unpacked_limb_1_col11) + (unpacked_limb_15_col20)),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(2))
                        + (unpacked_limb_16_col21)),
                    ((unpacked_limb_3_col12)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(17))),
                    ((unpacked_limb_4_col13) + (unpacked_limb_18_col22)),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(5))
                        + (unpacked_limb_19_col23)),
                    ((unpacked_limb_6_col14)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(20))),
                    ((unpacked_limb_7_col15) + (unpacked_limb_21_col24)),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(8))
                        + (unpacked_limb_22_col25)),
                    ((unpacked_limb_9_col16)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(23))),
                    ((unpacked_limb_10_col17) + (unpacked_limb_24_col26)),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(11))
                        + (unpacked_limb_25_col27)),
                    ((unpacked_limb_12_col18)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(26))),
                    ((unpacked_limb_13_col19) + (input_limb_9_col9)),
                ];
                let y_sum_tmp_fec87_15 = [
                    ((unpacked_limb_0_col10)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(14))),
                    ((unpacked_limb_1_col11) + (unpacked_limb_15_col20)),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(2))
                        + (unpacked_limb_16_col21)),
                    ((unpacked_limb_3_col12)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(17))),
                    ((unpacked_limb_4_col13) + (unpacked_limb_18_col22)),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(5))
                        + (unpacked_limb_19_col23)),
                    ((unpacked_limb_6_col14)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(20))),
                    ((unpacked_limb_7_col15) + (unpacked_limb_21_col24)),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(8))
                        + (unpacked_limb_22_col25)),
                    ((unpacked_limb_9_col16)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(23))),
                    ((unpacked_limb_10_col17) + (unpacked_limb_24_col26)),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(11))
                        + (unpacked_limb_25_col27)),
                    ((unpacked_limb_12_col18)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(26))),
                    ((unpacked_limb_13_col19) + (input_limb_9_col9)),
                ];

                // Single Karatsuba N 7.

                let z0_tmp_fec87_16 = [
                    ((x_sum_tmp_fec87_14[0]) * (y_sum_tmp_fec87_15[0])),
                    (((x_sum_tmp_fec87_14[0]) * (y_sum_tmp_fec87_15[1]))
                        + ((x_sum_tmp_fec87_14[1]) * (y_sum_tmp_fec87_15[0]))),
                    ((((x_sum_tmp_fec87_14[0]) * (y_sum_tmp_fec87_15[2]))
                        + ((x_sum_tmp_fec87_14[1]) * (y_sum_tmp_fec87_15[1])))
                        + ((x_sum_tmp_fec87_14[2]) * (y_sum_tmp_fec87_15[0]))),
                    (((((x_sum_tmp_fec87_14[0]) * (y_sum_tmp_fec87_15[3]))
                        + ((x_sum_tmp_fec87_14[1]) * (y_sum_tmp_fec87_15[2])))
                        + ((x_sum_tmp_fec87_14[2]) * (y_sum_tmp_fec87_15[1])))
                        + ((x_sum_tmp_fec87_14[3]) * (y_sum_tmp_fec87_15[0]))),
                    ((((((x_sum_tmp_fec87_14[0]) * (y_sum_tmp_fec87_15[4]))
                        + ((x_sum_tmp_fec87_14[1]) * (y_sum_tmp_fec87_15[3])))
                        + ((x_sum_tmp_fec87_14[2]) * (y_sum_tmp_fec87_15[2])))
                        + ((x_sum_tmp_fec87_14[3]) * (y_sum_tmp_fec87_15[1])))
                        + ((x_sum_tmp_fec87_14[4]) * (y_sum_tmp_fec87_15[0]))),
                    (((((((x_sum_tmp_fec87_14[0]) * (y_sum_tmp_fec87_15[5]))
                        + ((x_sum_tmp_fec87_14[1]) * (y_sum_tmp_fec87_15[4])))
                        + ((x_sum_tmp_fec87_14[2]) * (y_sum_tmp_fec87_15[3])))
                        + ((x_sum_tmp_fec87_14[3]) * (y_sum_tmp_fec87_15[2])))
                        + ((x_sum_tmp_fec87_14[4]) * (y_sum_tmp_fec87_15[1])))
                        + ((x_sum_tmp_fec87_14[5]) * (y_sum_tmp_fec87_15[0]))),
                    ((((((((x_sum_tmp_fec87_14[0]) * (y_sum_tmp_fec87_15[6]))
                        + ((x_sum_tmp_fec87_14[1]) * (y_sum_tmp_fec87_15[5])))
                        + ((x_sum_tmp_fec87_14[2]) * (y_sum_tmp_fec87_15[4])))
                        + ((x_sum_tmp_fec87_14[3]) * (y_sum_tmp_fec87_15[3])))
                        + ((x_sum_tmp_fec87_14[4]) * (y_sum_tmp_fec87_15[2])))
                        + ((x_sum_tmp_fec87_14[5]) * (y_sum_tmp_fec87_15[1])))
                        + ((x_sum_tmp_fec87_14[6]) * (y_sum_tmp_fec87_15[0]))),
                    (((((((x_sum_tmp_fec87_14[1]) * (y_sum_tmp_fec87_15[6]))
                        + ((x_sum_tmp_fec87_14[2]) * (y_sum_tmp_fec87_15[5])))
                        + ((x_sum_tmp_fec87_14[3]) * (y_sum_tmp_fec87_15[4])))
                        + ((x_sum_tmp_fec87_14[4]) * (y_sum_tmp_fec87_15[3])))
                        + ((x_sum_tmp_fec87_14[5]) * (y_sum_tmp_fec87_15[2])))
                        + ((x_sum_tmp_fec87_14[6]) * (y_sum_tmp_fec87_15[1]))),
                    ((((((x_sum_tmp_fec87_14[2]) * (y_sum_tmp_fec87_15[6]))
                        + ((x_sum_tmp_fec87_14[3]) * (y_sum_tmp_fec87_15[5])))
                        + ((x_sum_tmp_fec87_14[4]) * (y_sum_tmp_fec87_15[4])))
                        + ((x_sum_tmp_fec87_14[5]) * (y_sum_tmp_fec87_15[3])))
                        + ((x_sum_tmp_fec87_14[6]) * (y_sum_tmp_fec87_15[2]))),
                    (((((x_sum_tmp_fec87_14[3]) * (y_sum_tmp_fec87_15[6]))
                        + ((x_sum_tmp_fec87_14[4]) * (y_sum_tmp_fec87_15[5])))
                        + ((x_sum_tmp_fec87_14[5]) * (y_sum_tmp_fec87_15[4])))
                        + ((x_sum_tmp_fec87_14[6]) * (y_sum_tmp_fec87_15[3]))),
                    ((((x_sum_tmp_fec87_14[4]) * (y_sum_tmp_fec87_15[6]))
                        + ((x_sum_tmp_fec87_14[5]) * (y_sum_tmp_fec87_15[5])))
                        + ((x_sum_tmp_fec87_14[6]) * (y_sum_tmp_fec87_15[4]))),
                    (((x_sum_tmp_fec87_14[5]) * (y_sum_tmp_fec87_15[6]))
                        + ((x_sum_tmp_fec87_14[6]) * (y_sum_tmp_fec87_15[5]))),
                    ((x_sum_tmp_fec87_14[6]) * (y_sum_tmp_fec87_15[6])),
                ];
                let z2_tmp_fec87_17 = [
                    ((x_sum_tmp_fec87_14[7]) * (y_sum_tmp_fec87_15[7])),
                    (((x_sum_tmp_fec87_14[7]) * (y_sum_tmp_fec87_15[8]))
                        + ((x_sum_tmp_fec87_14[8]) * (y_sum_tmp_fec87_15[7]))),
                    ((((x_sum_tmp_fec87_14[7]) * (y_sum_tmp_fec87_15[9]))
                        + ((x_sum_tmp_fec87_14[8]) * (y_sum_tmp_fec87_15[8])))
                        + ((x_sum_tmp_fec87_14[9]) * (y_sum_tmp_fec87_15[7]))),
                    (((((x_sum_tmp_fec87_14[7]) * (y_sum_tmp_fec87_15[10]))
                        + ((x_sum_tmp_fec87_14[8]) * (y_sum_tmp_fec87_15[9])))
                        + ((x_sum_tmp_fec87_14[9]) * (y_sum_tmp_fec87_15[8])))
                        + ((x_sum_tmp_fec87_14[10]) * (y_sum_tmp_fec87_15[7]))),
                    ((((((x_sum_tmp_fec87_14[7]) * (y_sum_tmp_fec87_15[11]))
                        + ((x_sum_tmp_fec87_14[8]) * (y_sum_tmp_fec87_15[10])))
                        + ((x_sum_tmp_fec87_14[9]) * (y_sum_tmp_fec87_15[9])))
                        + ((x_sum_tmp_fec87_14[10]) * (y_sum_tmp_fec87_15[8])))
                        + ((x_sum_tmp_fec87_14[11]) * (y_sum_tmp_fec87_15[7]))),
                    (((((((x_sum_tmp_fec87_14[7]) * (y_sum_tmp_fec87_15[12]))
                        + ((x_sum_tmp_fec87_14[8]) * (y_sum_tmp_fec87_15[11])))
                        + ((x_sum_tmp_fec87_14[9]) * (y_sum_tmp_fec87_15[10])))
                        + ((x_sum_tmp_fec87_14[10]) * (y_sum_tmp_fec87_15[9])))
                        + ((x_sum_tmp_fec87_14[11]) * (y_sum_tmp_fec87_15[8])))
                        + ((x_sum_tmp_fec87_14[12]) * (y_sum_tmp_fec87_15[7]))),
                    ((((((((x_sum_tmp_fec87_14[7]) * (y_sum_tmp_fec87_15[13]))
                        + ((x_sum_tmp_fec87_14[8]) * (y_sum_tmp_fec87_15[12])))
                        + ((x_sum_tmp_fec87_14[9]) * (y_sum_tmp_fec87_15[11])))
                        + ((x_sum_tmp_fec87_14[10]) * (y_sum_tmp_fec87_15[10])))
                        + ((x_sum_tmp_fec87_14[11]) * (y_sum_tmp_fec87_15[9])))
                        + ((x_sum_tmp_fec87_14[12]) * (y_sum_tmp_fec87_15[8])))
                        + ((x_sum_tmp_fec87_14[13]) * (y_sum_tmp_fec87_15[7]))),
                    (((((((x_sum_tmp_fec87_14[8]) * (y_sum_tmp_fec87_15[13]))
                        + ((x_sum_tmp_fec87_14[9]) * (y_sum_tmp_fec87_15[12])))
                        + ((x_sum_tmp_fec87_14[10]) * (y_sum_tmp_fec87_15[11])))
                        + ((x_sum_tmp_fec87_14[11]) * (y_sum_tmp_fec87_15[10])))
                        + ((x_sum_tmp_fec87_14[12]) * (y_sum_tmp_fec87_15[9])))
                        + ((x_sum_tmp_fec87_14[13]) * (y_sum_tmp_fec87_15[8]))),
                    ((((((x_sum_tmp_fec87_14[9]) * (y_sum_tmp_fec87_15[13]))
                        + ((x_sum_tmp_fec87_14[10]) * (y_sum_tmp_fec87_15[12])))
                        + ((x_sum_tmp_fec87_14[11]) * (y_sum_tmp_fec87_15[11])))
                        + ((x_sum_tmp_fec87_14[12]) * (y_sum_tmp_fec87_15[10])))
                        + ((x_sum_tmp_fec87_14[13]) * (y_sum_tmp_fec87_15[9]))),
                    (((((x_sum_tmp_fec87_14[10]) * (y_sum_tmp_fec87_15[13]))
                        + ((x_sum_tmp_fec87_14[11]) * (y_sum_tmp_fec87_15[12])))
                        + ((x_sum_tmp_fec87_14[12]) * (y_sum_tmp_fec87_15[11])))
                        + ((x_sum_tmp_fec87_14[13]) * (y_sum_tmp_fec87_15[10]))),
                    ((((x_sum_tmp_fec87_14[11]) * (y_sum_tmp_fec87_15[13]))
                        + ((x_sum_tmp_fec87_14[12]) * (y_sum_tmp_fec87_15[12])))
                        + ((x_sum_tmp_fec87_14[13]) * (y_sum_tmp_fec87_15[11]))),
                    (((x_sum_tmp_fec87_14[12]) * (y_sum_tmp_fec87_15[13]))
                        + ((x_sum_tmp_fec87_14[13]) * (y_sum_tmp_fec87_15[12]))),
                    ((x_sum_tmp_fec87_14[13]) * (y_sum_tmp_fec87_15[13])),
                ];
                let x_sum_tmp_fec87_18 = [
                    ((x_sum_tmp_fec87_14[0]) + (x_sum_tmp_fec87_14[7])),
                    ((x_sum_tmp_fec87_14[1]) + (x_sum_tmp_fec87_14[8])),
                    ((x_sum_tmp_fec87_14[2]) + (x_sum_tmp_fec87_14[9])),
                    ((x_sum_tmp_fec87_14[3]) + (x_sum_tmp_fec87_14[10])),
                    ((x_sum_tmp_fec87_14[4]) + (x_sum_tmp_fec87_14[11])),
                    ((x_sum_tmp_fec87_14[5]) + (x_sum_tmp_fec87_14[12])),
                    ((x_sum_tmp_fec87_14[6]) + (x_sum_tmp_fec87_14[13])),
                ];
                let y_sum_tmp_fec87_19 = [
                    ((y_sum_tmp_fec87_15[0]) + (y_sum_tmp_fec87_15[7])),
                    ((y_sum_tmp_fec87_15[1]) + (y_sum_tmp_fec87_15[8])),
                    ((y_sum_tmp_fec87_15[2]) + (y_sum_tmp_fec87_15[9])),
                    ((y_sum_tmp_fec87_15[3]) + (y_sum_tmp_fec87_15[10])),
                    ((y_sum_tmp_fec87_15[4]) + (y_sum_tmp_fec87_15[11])),
                    ((y_sum_tmp_fec87_15[5]) + (y_sum_tmp_fec87_15[12])),
                    ((y_sum_tmp_fec87_15[6]) + (y_sum_tmp_fec87_15[13])),
                ];
                let single_karatsuba_n_7_output_tmp_fec87_20 = [
                    z0_tmp_fec87_16[0],
                    z0_tmp_fec87_16[1],
                    z0_tmp_fec87_16[2],
                    z0_tmp_fec87_16[3],
                    z0_tmp_fec87_16[4],
                    z0_tmp_fec87_16[5],
                    z0_tmp_fec87_16[6],
                    ((z0_tmp_fec87_16[7])
                        + ((((x_sum_tmp_fec87_18[0]) * (y_sum_tmp_fec87_19[0]))
                            - (z0_tmp_fec87_16[0]))
                            - (z2_tmp_fec87_17[0]))),
                    ((z0_tmp_fec87_16[8])
                        + (((((x_sum_tmp_fec87_18[0]) * (y_sum_tmp_fec87_19[1]))
                            + ((x_sum_tmp_fec87_18[1]) * (y_sum_tmp_fec87_19[0])))
                            - (z0_tmp_fec87_16[1]))
                            - (z2_tmp_fec87_17[1]))),
                    ((z0_tmp_fec87_16[9])
                        + ((((((x_sum_tmp_fec87_18[0]) * (y_sum_tmp_fec87_19[2]))
                            + ((x_sum_tmp_fec87_18[1]) * (y_sum_tmp_fec87_19[1])))
                            + ((x_sum_tmp_fec87_18[2]) * (y_sum_tmp_fec87_19[0])))
                            - (z0_tmp_fec87_16[2]))
                            - (z2_tmp_fec87_17[2]))),
                    ((z0_tmp_fec87_16[10])
                        + (((((((x_sum_tmp_fec87_18[0]) * (y_sum_tmp_fec87_19[3]))
                            + ((x_sum_tmp_fec87_18[1]) * (y_sum_tmp_fec87_19[2])))
                            + ((x_sum_tmp_fec87_18[2]) * (y_sum_tmp_fec87_19[1])))
                            + ((x_sum_tmp_fec87_18[3]) * (y_sum_tmp_fec87_19[0])))
                            - (z0_tmp_fec87_16[3]))
                            - (z2_tmp_fec87_17[3]))),
                    ((z0_tmp_fec87_16[11])
                        + ((((((((x_sum_tmp_fec87_18[0]) * (y_sum_tmp_fec87_19[4]))
                            + ((x_sum_tmp_fec87_18[1]) * (y_sum_tmp_fec87_19[3])))
                            + ((x_sum_tmp_fec87_18[2]) * (y_sum_tmp_fec87_19[2])))
                            + ((x_sum_tmp_fec87_18[3]) * (y_sum_tmp_fec87_19[1])))
                            + ((x_sum_tmp_fec87_18[4]) * (y_sum_tmp_fec87_19[0])))
                            - (z0_tmp_fec87_16[4]))
                            - (z2_tmp_fec87_17[4]))),
                    ((z0_tmp_fec87_16[12])
                        + (((((((((x_sum_tmp_fec87_18[0]) * (y_sum_tmp_fec87_19[5]))
                            + ((x_sum_tmp_fec87_18[1]) * (y_sum_tmp_fec87_19[4])))
                            + ((x_sum_tmp_fec87_18[2]) * (y_sum_tmp_fec87_19[3])))
                            + ((x_sum_tmp_fec87_18[3]) * (y_sum_tmp_fec87_19[2])))
                            + ((x_sum_tmp_fec87_18[4]) * (y_sum_tmp_fec87_19[1])))
                            + ((x_sum_tmp_fec87_18[5]) * (y_sum_tmp_fec87_19[0])))
                            - (z0_tmp_fec87_16[5]))
                            - (z2_tmp_fec87_17[5]))),
                    ((((((((((x_sum_tmp_fec87_18[0]) * (y_sum_tmp_fec87_19[6]))
                        + ((x_sum_tmp_fec87_18[1]) * (y_sum_tmp_fec87_19[5])))
                        + ((x_sum_tmp_fec87_18[2]) * (y_sum_tmp_fec87_19[4])))
                        + ((x_sum_tmp_fec87_18[3]) * (y_sum_tmp_fec87_19[3])))
                        + ((x_sum_tmp_fec87_18[4]) * (y_sum_tmp_fec87_19[2])))
                        + ((x_sum_tmp_fec87_18[5]) * (y_sum_tmp_fec87_19[1])))
                        + ((x_sum_tmp_fec87_18[6]) * (y_sum_tmp_fec87_19[0])))
                        - (z0_tmp_fec87_16[6]))
                        - (z2_tmp_fec87_17[6])),
                    ((z2_tmp_fec87_17[0])
                        + (((((((((x_sum_tmp_fec87_18[1]) * (y_sum_tmp_fec87_19[6]))
                            + ((x_sum_tmp_fec87_18[2]) * (y_sum_tmp_fec87_19[5])))
                            + ((x_sum_tmp_fec87_18[3]) * (y_sum_tmp_fec87_19[4])))
                            + ((x_sum_tmp_fec87_18[4]) * (y_sum_tmp_fec87_19[3])))
                            + ((x_sum_tmp_fec87_18[5]) * (y_sum_tmp_fec87_19[2])))
                            + ((x_sum_tmp_fec87_18[6]) * (y_sum_tmp_fec87_19[1])))
                            - (z0_tmp_fec87_16[7]))
                            - (z2_tmp_fec87_17[7]))),
                    ((z2_tmp_fec87_17[1])
                        + ((((((((x_sum_tmp_fec87_18[2]) * (y_sum_tmp_fec87_19[6]))
                            + ((x_sum_tmp_fec87_18[3]) * (y_sum_tmp_fec87_19[5])))
                            + ((x_sum_tmp_fec87_18[4]) * (y_sum_tmp_fec87_19[4])))
                            + ((x_sum_tmp_fec87_18[5]) * (y_sum_tmp_fec87_19[3])))
                            + ((x_sum_tmp_fec87_18[6]) * (y_sum_tmp_fec87_19[2])))
                            - (z0_tmp_fec87_16[8]))
                            - (z2_tmp_fec87_17[8]))),
                    ((z2_tmp_fec87_17[2])
                        + (((((((x_sum_tmp_fec87_18[3]) * (y_sum_tmp_fec87_19[6]))
                            + ((x_sum_tmp_fec87_18[4]) * (y_sum_tmp_fec87_19[5])))
                            + ((x_sum_tmp_fec87_18[5]) * (y_sum_tmp_fec87_19[4])))
                            + ((x_sum_tmp_fec87_18[6]) * (y_sum_tmp_fec87_19[3])))
                            - (z0_tmp_fec87_16[9]))
                            - (z2_tmp_fec87_17[9]))),
                    ((z2_tmp_fec87_17[3])
                        + ((((((x_sum_tmp_fec87_18[4]) * (y_sum_tmp_fec87_19[6]))
                            + ((x_sum_tmp_fec87_18[5]) * (y_sum_tmp_fec87_19[5])))
                            + ((x_sum_tmp_fec87_18[6]) * (y_sum_tmp_fec87_19[4])))
                            - (z0_tmp_fec87_16[10]))
                            - (z2_tmp_fec87_17[10]))),
                    ((z2_tmp_fec87_17[4])
                        + (((((x_sum_tmp_fec87_18[5]) * (y_sum_tmp_fec87_19[6]))
                            + ((x_sum_tmp_fec87_18[6]) * (y_sum_tmp_fec87_19[5])))
                            - (z0_tmp_fec87_16[11]))
                            - (z2_tmp_fec87_17[11]))),
                    ((z2_tmp_fec87_17[5])
                        + ((((x_sum_tmp_fec87_18[6]) * (y_sum_tmp_fec87_19[6]))
                            - (z0_tmp_fec87_16[12]))
                            - (z2_tmp_fec87_17[12]))),
                    z2_tmp_fec87_17[6],
                    z2_tmp_fec87_17[7],
                    z2_tmp_fec87_17[8],
                    z2_tmp_fec87_17[9],
                    z2_tmp_fec87_17[10],
                    z2_tmp_fec87_17[11],
                    z2_tmp_fec87_17[12],
                ];

                let double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21 = [
                    single_karatsuba_n_7_output_tmp_fec87_8[0],
                    single_karatsuba_n_7_output_tmp_fec87_8[1],
                    single_karatsuba_n_7_output_tmp_fec87_8[2],
                    single_karatsuba_n_7_output_tmp_fec87_8[3],
                    single_karatsuba_n_7_output_tmp_fec87_8[4],
                    single_karatsuba_n_7_output_tmp_fec87_8[5],
                    single_karatsuba_n_7_output_tmp_fec87_8[6],
                    single_karatsuba_n_7_output_tmp_fec87_8[7],
                    single_karatsuba_n_7_output_tmp_fec87_8[8],
                    single_karatsuba_n_7_output_tmp_fec87_8[9],
                    single_karatsuba_n_7_output_tmp_fec87_8[10],
                    single_karatsuba_n_7_output_tmp_fec87_8[11],
                    single_karatsuba_n_7_output_tmp_fec87_8[12],
                    single_karatsuba_n_7_output_tmp_fec87_8[13],
                    ((single_karatsuba_n_7_output_tmp_fec87_8[14])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[0])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[0]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[0]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_8[15])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[1])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[1]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[1]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_8[16])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[2])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[2]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[2]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_8[17])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[3])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[3]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[3]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_8[18])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[4])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[4]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[4]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_8[19])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[5])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[5]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[5]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_8[20])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[6])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[6]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[6]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_8[21])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[7])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[7]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[7]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_8[22])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[8])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[8]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[8]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_8[23])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[9])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[9]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[9]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_8[24])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[10])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[10]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[10]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_8[25])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[11])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[11]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[11]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_8[26])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[12])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[12]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[12]))),
                    (((single_karatsuba_n_7_output_tmp_fec87_20[13])
                        - (single_karatsuba_n_7_output_tmp_fec87_8[13]))
                        - (single_karatsuba_n_7_output_tmp_fec87_13[13])),
                    ((single_karatsuba_n_7_output_tmp_fec87_13[0])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[14])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[14]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[14]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_13[1])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[15])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[15]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[15]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_13[2])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[16])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[16]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[16]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_13[3])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[17])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[17]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[17]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_13[4])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[18])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[18]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[18]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_13[5])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[19])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[19]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[19]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_13[6])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[20])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[20]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[20]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_13[7])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[21])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[21]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[21]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_13[8])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[22])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[22]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[22]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_13[9])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[23])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[23]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[23]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_13[10])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[24])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[24]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[24]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_13[11])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[25])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[25]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[25]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_13[12])
                        + (((single_karatsuba_n_7_output_tmp_fec87_20[26])
                            - (single_karatsuba_n_7_output_tmp_fec87_8[26]))
                            - (single_karatsuba_n_7_output_tmp_fec87_13[26]))),
                    single_karatsuba_n_7_output_tmp_fec87_13[13],
                    single_karatsuba_n_7_output_tmp_fec87_13[14],
                    single_karatsuba_n_7_output_tmp_fec87_13[15],
                    single_karatsuba_n_7_output_tmp_fec87_13[16],
                    single_karatsuba_n_7_output_tmp_fec87_13[17],
                    single_karatsuba_n_7_output_tmp_fec87_13[18],
                    single_karatsuba_n_7_output_tmp_fec87_13[19],
                    single_karatsuba_n_7_output_tmp_fec87_13[20],
                    single_karatsuba_n_7_output_tmp_fec87_13[21],
                    single_karatsuba_n_7_output_tmp_fec87_13[22],
                    single_karatsuba_n_7_output_tmp_fec87_13[23],
                    single_karatsuba_n_7_output_tmp_fec87_13[24],
                    single_karatsuba_n_7_output_tmp_fec87_13[25],
                    single_karatsuba_n_7_output_tmp_fec87_13[26],
                ];

                let conv_tmp_fec87_22 = [
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[0])
                        - (mul_res_limb_0_col28)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[1])
                        - (mul_res_limb_1_col29)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[2])
                        - (mul_res_limb_2_col30)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[3])
                        - (mul_res_limb_3_col31)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[4])
                        - (mul_res_limb_4_col32)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[5])
                        - (mul_res_limb_5_col33)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[6])
                        - (mul_res_limb_6_col34)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[7])
                        - (mul_res_limb_7_col35)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[8])
                        - (mul_res_limb_8_col36)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[9])
                        - (mul_res_limb_9_col37)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[10])
                        - (mul_res_limb_10_col38)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[11])
                        - (mul_res_limb_11_col39)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[12])
                        - (mul_res_limb_12_col40)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[13])
                        - (mul_res_limb_13_col41)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[14])
                        - (mul_res_limb_14_col42)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[15])
                        - (mul_res_limb_15_col43)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[16])
                        - (mul_res_limb_16_col44)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[17])
                        - (mul_res_limb_17_col45)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[18])
                        - (mul_res_limb_18_col46)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[19])
                        - (mul_res_limb_19_col47)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[20])
                        - (mul_res_limb_20_col48)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[21])
                        - (mul_res_limb_21_col49)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[22])
                        - (mul_res_limb_22_col50)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[23])
                        - (mul_res_limb_23_col51)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[24])
                        - (mul_res_limb_24_col52)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[25])
                        - (mul_res_limb_25_col53)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[26])
                        - (mul_res_limb_26_col54)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[27])
                        - (mul_res_limb_27_col55)),
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[28],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[29],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[30],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[31],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[32],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[33],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[34],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[35],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[36],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[37],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[38],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[39],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[40],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[41],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[42],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[43],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[44],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[45],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[46],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[47],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[48],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[49],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[50],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[51],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[52],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[53],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_21[54],
                ];
                let conv_mod_tmp_fec87_23 = [
                    ((((M31_32) * (conv_tmp_fec87_22[0])) - ((M31_4) * (conv_tmp_fec87_22[21])))
                        + ((M31_8) * (conv_tmp_fec87_22[49]))),
                    ((((conv_tmp_fec87_22[0]) + ((M31_32) * (conv_tmp_fec87_22[1])))
                        - ((M31_4) * (conv_tmp_fec87_22[22])))
                        + ((M31_8) * (conv_tmp_fec87_22[50]))),
                    ((((conv_tmp_fec87_22[1]) + ((M31_32) * (conv_tmp_fec87_22[2])))
                        - ((M31_4) * (conv_tmp_fec87_22[23])))
                        + ((M31_8) * (conv_tmp_fec87_22[51]))),
                    ((((conv_tmp_fec87_22[2]) + ((M31_32) * (conv_tmp_fec87_22[3])))
                        - ((M31_4) * (conv_tmp_fec87_22[24])))
                        + ((M31_8) * (conv_tmp_fec87_22[52]))),
                    ((((conv_tmp_fec87_22[3]) + ((M31_32) * (conv_tmp_fec87_22[4])))
                        - ((M31_4) * (conv_tmp_fec87_22[25])))
                        + ((M31_8) * (conv_tmp_fec87_22[53]))),
                    ((((conv_tmp_fec87_22[4]) + ((M31_32) * (conv_tmp_fec87_22[5])))
                        - ((M31_4) * (conv_tmp_fec87_22[26])))
                        + ((M31_8) * (conv_tmp_fec87_22[54]))),
                    (((conv_tmp_fec87_22[5]) + ((M31_32) * (conv_tmp_fec87_22[6])))
                        - ((M31_4) * (conv_tmp_fec87_22[27]))),
                    (((((M31_2) * (conv_tmp_fec87_22[0])) + (conv_tmp_fec87_22[6]))
                        + ((M31_32) * (conv_tmp_fec87_22[7])))
                        - ((M31_4) * (conv_tmp_fec87_22[28]))),
                    (((((M31_2) * (conv_tmp_fec87_22[1])) + (conv_tmp_fec87_22[7]))
                        + ((M31_32) * (conv_tmp_fec87_22[8])))
                        - ((M31_4) * (conv_tmp_fec87_22[29]))),
                    (((((M31_2) * (conv_tmp_fec87_22[2])) + (conv_tmp_fec87_22[8]))
                        + ((M31_32) * (conv_tmp_fec87_22[9])))
                        - ((M31_4) * (conv_tmp_fec87_22[30]))),
                    (((((M31_2) * (conv_tmp_fec87_22[3])) + (conv_tmp_fec87_22[9]))
                        + ((M31_32) * (conv_tmp_fec87_22[10])))
                        - ((M31_4) * (conv_tmp_fec87_22[31]))),
                    (((((M31_2) * (conv_tmp_fec87_22[4])) + (conv_tmp_fec87_22[10]))
                        + ((M31_32) * (conv_tmp_fec87_22[11])))
                        - ((M31_4) * (conv_tmp_fec87_22[32]))),
                    (((((M31_2) * (conv_tmp_fec87_22[5])) + (conv_tmp_fec87_22[11]))
                        + ((M31_32) * (conv_tmp_fec87_22[12])))
                        - ((M31_4) * (conv_tmp_fec87_22[33]))),
                    (((((M31_2) * (conv_tmp_fec87_22[6])) + (conv_tmp_fec87_22[12]))
                        + ((M31_32) * (conv_tmp_fec87_22[13])))
                        - ((M31_4) * (conv_tmp_fec87_22[34]))),
                    (((((M31_2) * (conv_tmp_fec87_22[7])) + (conv_tmp_fec87_22[13]))
                        + ((M31_32) * (conv_tmp_fec87_22[14])))
                        - ((M31_4) * (conv_tmp_fec87_22[35]))),
                    (((((M31_2) * (conv_tmp_fec87_22[8])) + (conv_tmp_fec87_22[14]))
                        + ((M31_32) * (conv_tmp_fec87_22[15])))
                        - ((M31_4) * (conv_tmp_fec87_22[36]))),
                    (((((M31_2) * (conv_tmp_fec87_22[9])) + (conv_tmp_fec87_22[15]))
                        + ((M31_32) * (conv_tmp_fec87_22[16])))
                        - ((M31_4) * (conv_tmp_fec87_22[37]))),
                    (((((M31_2) * (conv_tmp_fec87_22[10])) + (conv_tmp_fec87_22[16]))
                        + ((M31_32) * (conv_tmp_fec87_22[17])))
                        - ((M31_4) * (conv_tmp_fec87_22[38]))),
                    (((((M31_2) * (conv_tmp_fec87_22[11])) + (conv_tmp_fec87_22[17]))
                        + ((M31_32) * (conv_tmp_fec87_22[18])))
                        - ((M31_4) * (conv_tmp_fec87_22[39]))),
                    (((((M31_2) * (conv_tmp_fec87_22[12])) + (conv_tmp_fec87_22[18]))
                        + ((M31_32) * (conv_tmp_fec87_22[19])))
                        - ((M31_4) * (conv_tmp_fec87_22[40]))),
                    (((((M31_2) * (conv_tmp_fec87_22[13])) + (conv_tmp_fec87_22[19]))
                        + ((M31_32) * (conv_tmp_fec87_22[20])))
                        - ((M31_4) * (conv_tmp_fec87_22[41]))),
                    (((((M31_2) * (conv_tmp_fec87_22[14])) + (conv_tmp_fec87_22[20]))
                        - ((M31_4) * (conv_tmp_fec87_22[42])))
                        + ((M31_64) * (conv_tmp_fec87_22[49]))),
                    (((((M31_2) * (conv_tmp_fec87_22[15])) - ((M31_4) * (conv_tmp_fec87_22[43])))
                        + ((M31_2) * (conv_tmp_fec87_22[49])))
                        + ((M31_64) * (conv_tmp_fec87_22[50]))),
                    (((((M31_2) * (conv_tmp_fec87_22[16])) - ((M31_4) * (conv_tmp_fec87_22[44])))
                        + ((M31_2) * (conv_tmp_fec87_22[50])))
                        + ((M31_64) * (conv_tmp_fec87_22[51]))),
                    (((((M31_2) * (conv_tmp_fec87_22[17])) - ((M31_4) * (conv_tmp_fec87_22[45])))
                        + ((M31_2) * (conv_tmp_fec87_22[51])))
                        + ((M31_64) * (conv_tmp_fec87_22[52]))),
                    (((((M31_2) * (conv_tmp_fec87_22[18])) - ((M31_4) * (conv_tmp_fec87_22[46])))
                        + ((M31_2) * (conv_tmp_fec87_22[52])))
                        + ((M31_64) * (conv_tmp_fec87_22[53]))),
                    (((((M31_2) * (conv_tmp_fec87_22[19])) - ((M31_4) * (conv_tmp_fec87_22[47])))
                        + ((M31_2) * (conv_tmp_fec87_22[53])))
                        + ((M31_64) * (conv_tmp_fec87_22[54]))),
                    ((((M31_2) * (conv_tmp_fec87_22[20])) - ((M31_4) * (conv_tmp_fec87_22[48])))
                        + ((M31_2) * (conv_tmp_fec87_22[54]))),
                ];
                let k_mod_2_18_biased_tmp_fec87_24 =
                    ((((PackedUInt32::from_m31(((conv_mod_tmp_fec87_23[0]) + (M31_134217728))))
                        + (((PackedUInt32::from_m31(
                            ((conv_mod_tmp_fec87_23[1]) + (M31_134217728)),
                        )) & (UInt32_511))
                            << (UInt32_9)))
                        + (UInt32_65536))
                        & (UInt32_262143));
                let k_col56 = ((k_mod_2_18_biased_tmp_fec87_24.low().as_m31())
                    + (((k_mod_2_18_biased_tmp_fec87_24.high().as_m31()) - (M31_1)) * (M31_65536)));
                *row[56] = k_col56;
                *sub_component_inputs.range_check_19[0] = [((k_col56) + (M31_262144))];
                *lookup_data.range_check_19_0 = [((k_col56) + (M31_262144))];
                let carry_0_col57 = (((conv_mod_tmp_fec87_23[0]) - (k_col56)) * (M31_4194304));
                *row[57] = carry_0_col57;
                *sub_component_inputs.range_check_19[1] = [((carry_0_col57) + (M31_131072))];
                *lookup_data.range_check_19_1 = [((carry_0_col57) + (M31_131072))];
                let carry_1_col58 =
                    (((conv_mod_tmp_fec87_23[1]) + (carry_0_col57)) * (M31_4194304));
                *row[58] = carry_1_col58;
                *sub_component_inputs.range_check_19_b[0] = [((carry_1_col58) + (M31_131072))];
                *lookup_data.range_check_19_b_0 = [((carry_1_col58) + (M31_131072))];
                let carry_2_col59 =
                    (((conv_mod_tmp_fec87_23[2]) + (carry_1_col58)) * (M31_4194304));
                *row[59] = carry_2_col59;
                *sub_component_inputs.range_check_19_c[0] = [((carry_2_col59) + (M31_131072))];
                *lookup_data.range_check_19_c_0 = [((carry_2_col59) + (M31_131072))];
                let carry_3_col60 =
                    (((conv_mod_tmp_fec87_23[3]) + (carry_2_col59)) * (M31_4194304));
                *row[60] = carry_3_col60;
                *sub_component_inputs.range_check_19_d[0] = [((carry_3_col60) + (M31_131072))];
                *lookup_data.range_check_19_d_0 = [((carry_3_col60) + (M31_131072))];
                let carry_4_col61 =
                    (((conv_mod_tmp_fec87_23[4]) + (carry_3_col60)) * (M31_4194304));
                *row[61] = carry_4_col61;
                *sub_component_inputs.range_check_19[2] = [((carry_4_col61) + (M31_131072))];
                *lookup_data.range_check_19_2 = [((carry_4_col61) + (M31_131072))];
                let carry_5_col62 =
                    (((conv_mod_tmp_fec87_23[5]) + (carry_4_col61)) * (M31_4194304));
                *row[62] = carry_5_col62;
                *sub_component_inputs.range_check_19_b[1] = [((carry_5_col62) + (M31_131072))];
                *lookup_data.range_check_19_b_1 = [((carry_5_col62) + (M31_131072))];
                let carry_6_col63 =
                    (((conv_mod_tmp_fec87_23[6]) + (carry_5_col62)) * (M31_4194304));
                *row[63] = carry_6_col63;
                *sub_component_inputs.range_check_19_c[1] = [((carry_6_col63) + (M31_131072))];
                *lookup_data.range_check_19_c_1 = [((carry_6_col63) + (M31_131072))];
                let carry_7_col64 =
                    (((conv_mod_tmp_fec87_23[7]) + (carry_6_col63)) * (M31_4194304));
                *row[64] = carry_7_col64;
                *sub_component_inputs.range_check_19_d[1] = [((carry_7_col64) + (M31_131072))];
                *lookup_data.range_check_19_d_1 = [((carry_7_col64) + (M31_131072))];
                let carry_8_col65 =
                    (((conv_mod_tmp_fec87_23[8]) + (carry_7_col64)) * (M31_4194304));
                *row[65] = carry_8_col65;
                *sub_component_inputs.range_check_19[3] = [((carry_8_col65) + (M31_131072))];
                *lookup_data.range_check_19_3 = [((carry_8_col65) + (M31_131072))];
                let carry_9_col66 =
                    (((conv_mod_tmp_fec87_23[9]) + (carry_8_col65)) * (M31_4194304));
                *row[66] = carry_9_col66;
                *sub_component_inputs.range_check_19_b[2] = [((carry_9_col66) + (M31_131072))];
                *lookup_data.range_check_19_b_2 = [((carry_9_col66) + (M31_131072))];
                let carry_10_col67 =
                    (((conv_mod_tmp_fec87_23[10]) + (carry_9_col66)) * (M31_4194304));
                *row[67] = carry_10_col67;
                *sub_component_inputs.range_check_19_c[2] = [((carry_10_col67) + (M31_131072))];
                *lookup_data.range_check_19_c_2 = [((carry_10_col67) + (M31_131072))];
                let carry_11_col68 =
                    (((conv_mod_tmp_fec87_23[11]) + (carry_10_col67)) * (M31_4194304));
                *row[68] = carry_11_col68;
                *sub_component_inputs.range_check_19_d[2] = [((carry_11_col68) + (M31_131072))];
                *lookup_data.range_check_19_d_2 = [((carry_11_col68) + (M31_131072))];
                let carry_12_col69 =
                    (((conv_mod_tmp_fec87_23[12]) + (carry_11_col68)) * (M31_4194304));
                *row[69] = carry_12_col69;
                *sub_component_inputs.range_check_19[4] = [((carry_12_col69) + (M31_131072))];
                *lookup_data.range_check_19_4 = [((carry_12_col69) + (M31_131072))];
                let carry_13_col70 =
                    (((conv_mod_tmp_fec87_23[13]) + (carry_12_col69)) * (M31_4194304));
                *row[70] = carry_13_col70;
                *sub_component_inputs.range_check_19_b[3] = [((carry_13_col70) + (M31_131072))];
                *lookup_data.range_check_19_b_3 = [((carry_13_col70) + (M31_131072))];
                let carry_14_col71 =
                    (((conv_mod_tmp_fec87_23[14]) + (carry_13_col70)) * (M31_4194304));
                *row[71] = carry_14_col71;
                *sub_component_inputs.range_check_19_c[3] = [((carry_14_col71) + (M31_131072))];
                *lookup_data.range_check_19_c_3 = [((carry_14_col71) + (M31_131072))];
                let carry_15_col72 =
                    (((conv_mod_tmp_fec87_23[15]) + (carry_14_col71)) * (M31_4194304));
                *row[72] = carry_15_col72;
                *sub_component_inputs.range_check_19_d[3] = [((carry_15_col72) + (M31_131072))];
                *lookup_data.range_check_19_d_3 = [((carry_15_col72) + (M31_131072))];
                let carry_16_col73 =
                    (((conv_mod_tmp_fec87_23[16]) + (carry_15_col72)) * (M31_4194304));
                *row[73] = carry_16_col73;
                *sub_component_inputs.range_check_19[5] = [((carry_16_col73) + (M31_131072))];
                *lookup_data.range_check_19_5 = [((carry_16_col73) + (M31_131072))];
                let carry_17_col74 =
                    (((conv_mod_tmp_fec87_23[17]) + (carry_16_col73)) * (M31_4194304));
                *row[74] = carry_17_col74;
                *sub_component_inputs.range_check_19_b[4] = [((carry_17_col74) + (M31_131072))];
                *lookup_data.range_check_19_b_4 = [((carry_17_col74) + (M31_131072))];
                let carry_18_col75 =
                    (((conv_mod_tmp_fec87_23[18]) + (carry_17_col74)) * (M31_4194304));
                *row[75] = carry_18_col75;
                *sub_component_inputs.range_check_19_c[4] = [((carry_18_col75) + (M31_131072))];
                *lookup_data.range_check_19_c_4 = [((carry_18_col75) + (M31_131072))];
                let carry_19_col76 =
                    (((conv_mod_tmp_fec87_23[19]) + (carry_18_col75)) * (M31_4194304));
                *row[76] = carry_19_col76;
                *sub_component_inputs.range_check_19_d[4] = [((carry_19_col76) + (M31_131072))];
                *lookup_data.range_check_19_d_4 = [((carry_19_col76) + (M31_131072))];
                let carry_20_col77 =
                    (((conv_mod_tmp_fec87_23[20]) + (carry_19_col76)) * (M31_4194304));
                *row[77] = carry_20_col77;
                *sub_component_inputs.range_check_19[6] = [((carry_20_col77) + (M31_131072))];
                *lookup_data.range_check_19_6 = [((carry_20_col77) + (M31_131072))];
                let carry_21_col78 = ((((conv_mod_tmp_fec87_23[21]) - ((M31_136) * (k_col56)))
                    + (carry_20_col77))
                    * (M31_4194304));
                *row[78] = carry_21_col78;
                *sub_component_inputs.range_check_19_b[5] = [((carry_21_col78) + (M31_131072))];
                *lookup_data.range_check_19_b_5 = [((carry_21_col78) + (M31_131072))];
                let carry_22_col79 =
                    (((conv_mod_tmp_fec87_23[22]) + (carry_21_col78)) * (M31_4194304));
                *row[79] = carry_22_col79;
                *sub_component_inputs.range_check_19_c[5] = [((carry_22_col79) + (M31_131072))];
                *lookup_data.range_check_19_c_5 = [((carry_22_col79) + (M31_131072))];
                let carry_23_col80 =
                    (((conv_mod_tmp_fec87_23[23]) + (carry_22_col79)) * (M31_4194304));
                *row[80] = carry_23_col80;
                *sub_component_inputs.range_check_19_d[5] = [((carry_23_col80) + (M31_131072))];
                *lookup_data.range_check_19_d_5 = [((carry_23_col80) + (M31_131072))];
                let carry_24_col81 =
                    (((conv_mod_tmp_fec87_23[24]) + (carry_23_col80)) * (M31_4194304));
                *row[81] = carry_24_col81;
                *sub_component_inputs.range_check_19[7] = [((carry_24_col81) + (M31_131072))];
                *lookup_data.range_check_19_7 = [((carry_24_col81) + (M31_131072))];
                let carry_25_col82 =
                    (((conv_mod_tmp_fec87_23[25]) + (carry_24_col81)) * (M31_4194304));
                *row[82] = carry_25_col82;
                *sub_component_inputs.range_check_19_b[6] = [((carry_25_col82) + (M31_131072))];
                *lookup_data.range_check_19_b_6 = [((carry_25_col82) + (M31_131072))];
                let carry_26_col83 =
                    (((conv_mod_tmp_fec87_23[26]) + (carry_25_col82)) * (M31_4194304));
                *row[83] = carry_26_col83;
                *sub_component_inputs.range_check_19_c[6] = [((carry_26_col83) + (M31_131072))];
                *lookup_data.range_check_19_c_6 = [((carry_26_col83) + (M31_131072))];

                let mul_252_output_tmp_fec87_25 = mul_res_tmp_fec87_3;

                // Mul 252.

                let mul_res_tmp_fec87_26 =
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2)
                        * (mul_252_output_tmp_fec87_25));
                let mul_res_limb_0_col84 = mul_res_tmp_fec87_26.get_m31(0);
                *row[84] = mul_res_limb_0_col84;
                let mul_res_limb_1_col85 = mul_res_tmp_fec87_26.get_m31(1);
                *row[85] = mul_res_limb_1_col85;
                let mul_res_limb_2_col86 = mul_res_tmp_fec87_26.get_m31(2);
                *row[86] = mul_res_limb_2_col86;
                let mul_res_limb_3_col87 = mul_res_tmp_fec87_26.get_m31(3);
                *row[87] = mul_res_limb_3_col87;
                let mul_res_limb_4_col88 = mul_res_tmp_fec87_26.get_m31(4);
                *row[88] = mul_res_limb_4_col88;
                let mul_res_limb_5_col89 = mul_res_tmp_fec87_26.get_m31(5);
                *row[89] = mul_res_limb_5_col89;
                let mul_res_limb_6_col90 = mul_res_tmp_fec87_26.get_m31(6);
                *row[90] = mul_res_limb_6_col90;
                let mul_res_limb_7_col91 = mul_res_tmp_fec87_26.get_m31(7);
                *row[91] = mul_res_limb_7_col91;
                let mul_res_limb_8_col92 = mul_res_tmp_fec87_26.get_m31(8);
                *row[92] = mul_res_limb_8_col92;
                let mul_res_limb_9_col93 = mul_res_tmp_fec87_26.get_m31(9);
                *row[93] = mul_res_limb_9_col93;
                let mul_res_limb_10_col94 = mul_res_tmp_fec87_26.get_m31(10);
                *row[94] = mul_res_limb_10_col94;
                let mul_res_limb_11_col95 = mul_res_tmp_fec87_26.get_m31(11);
                *row[95] = mul_res_limb_11_col95;
                let mul_res_limb_12_col96 = mul_res_tmp_fec87_26.get_m31(12);
                *row[96] = mul_res_limb_12_col96;
                let mul_res_limb_13_col97 = mul_res_tmp_fec87_26.get_m31(13);
                *row[97] = mul_res_limb_13_col97;
                let mul_res_limb_14_col98 = mul_res_tmp_fec87_26.get_m31(14);
                *row[98] = mul_res_limb_14_col98;
                let mul_res_limb_15_col99 = mul_res_tmp_fec87_26.get_m31(15);
                *row[99] = mul_res_limb_15_col99;
                let mul_res_limb_16_col100 = mul_res_tmp_fec87_26.get_m31(16);
                *row[100] = mul_res_limb_16_col100;
                let mul_res_limb_17_col101 = mul_res_tmp_fec87_26.get_m31(17);
                *row[101] = mul_res_limb_17_col101;
                let mul_res_limb_18_col102 = mul_res_tmp_fec87_26.get_m31(18);
                *row[102] = mul_res_limb_18_col102;
                let mul_res_limb_19_col103 = mul_res_tmp_fec87_26.get_m31(19);
                *row[103] = mul_res_limb_19_col103;
                let mul_res_limb_20_col104 = mul_res_tmp_fec87_26.get_m31(20);
                *row[104] = mul_res_limb_20_col104;
                let mul_res_limb_21_col105 = mul_res_tmp_fec87_26.get_m31(21);
                *row[105] = mul_res_limb_21_col105;
                let mul_res_limb_22_col106 = mul_res_tmp_fec87_26.get_m31(22);
                *row[106] = mul_res_limb_22_col106;
                let mul_res_limb_23_col107 = mul_res_tmp_fec87_26.get_m31(23);
                *row[107] = mul_res_limb_23_col107;
                let mul_res_limb_24_col108 = mul_res_tmp_fec87_26.get_m31(24);
                *row[108] = mul_res_limb_24_col108;
                let mul_res_limb_25_col109 = mul_res_tmp_fec87_26.get_m31(25);
                *row[109] = mul_res_limb_25_col109;
                let mul_res_limb_26_col110 = mul_res_tmp_fec87_26.get_m31(26);
                *row[110] = mul_res_limb_26_col110;
                let mul_res_limb_27_col111 = mul_res_tmp_fec87_26.get_m31(27);
                *row[111] = mul_res_limb_27_col111;

                // Range Check Mem Value N 28.

                *sub_component_inputs.range_check_9_9[8] =
                    [mul_res_limb_0_col84, mul_res_limb_1_col85];
                *lookup_data.range_check_9_9_8 = [mul_res_limb_0_col84, mul_res_limb_1_col85];
                *sub_component_inputs.range_check_9_9_b[8] =
                    [mul_res_limb_2_col86, mul_res_limb_3_col87];
                *lookup_data.range_check_9_9_b_8 = [mul_res_limb_2_col86, mul_res_limb_3_col87];
                *sub_component_inputs.range_check_9_9_c[6] =
                    [mul_res_limb_4_col88, mul_res_limb_5_col89];
                *lookup_data.range_check_9_9_c_6 = [mul_res_limb_4_col88, mul_res_limb_5_col89];
                *sub_component_inputs.range_check_9_9_d[6] =
                    [mul_res_limb_6_col90, mul_res_limb_7_col91];
                *lookup_data.range_check_9_9_d_6 = [mul_res_limb_6_col90, mul_res_limb_7_col91];
                *sub_component_inputs.range_check_9_9[9] =
                    [mul_res_limb_8_col92, mul_res_limb_9_col93];
                *lookup_data.range_check_9_9_9 = [mul_res_limb_8_col92, mul_res_limb_9_col93];
                *sub_component_inputs.range_check_9_9_b[9] =
                    [mul_res_limb_10_col94, mul_res_limb_11_col95];
                *lookup_data.range_check_9_9_b_9 = [mul_res_limb_10_col94, mul_res_limb_11_col95];
                *sub_component_inputs.range_check_9_9_c[7] =
                    [mul_res_limb_12_col96, mul_res_limb_13_col97];
                *lookup_data.range_check_9_9_c_7 = [mul_res_limb_12_col96, mul_res_limb_13_col97];
                *sub_component_inputs.range_check_9_9_d[7] =
                    [mul_res_limb_14_col98, mul_res_limb_15_col99];
                *lookup_data.range_check_9_9_d_7 = [mul_res_limb_14_col98, mul_res_limb_15_col99];
                *sub_component_inputs.range_check_9_9[10] =
                    [mul_res_limb_16_col100, mul_res_limb_17_col101];
                *lookup_data.range_check_9_9_10 = [mul_res_limb_16_col100, mul_res_limb_17_col101];
                *sub_component_inputs.range_check_9_9_b[10] =
                    [mul_res_limb_18_col102, mul_res_limb_19_col103];
                *lookup_data.range_check_9_9_b_10 =
                    [mul_res_limb_18_col102, mul_res_limb_19_col103];
                *sub_component_inputs.range_check_9_9_c[8] =
                    [mul_res_limb_20_col104, mul_res_limb_21_col105];
                *lookup_data.range_check_9_9_c_8 = [mul_res_limb_20_col104, mul_res_limb_21_col105];
                *sub_component_inputs.range_check_9_9_d[8] =
                    [mul_res_limb_22_col106, mul_res_limb_23_col107];
                *lookup_data.range_check_9_9_d_8 = [mul_res_limb_22_col106, mul_res_limb_23_col107];
                *sub_component_inputs.range_check_9_9[11] =
                    [mul_res_limb_24_col108, mul_res_limb_25_col109];
                *lookup_data.range_check_9_9_11 = [mul_res_limb_24_col108, mul_res_limb_25_col109];
                *sub_component_inputs.range_check_9_9_b[11] =
                    [mul_res_limb_26_col110, mul_res_limb_27_col111];
                *lookup_data.range_check_9_9_b_11 =
                    [mul_res_limb_26_col110, mul_res_limb_27_col111];

                // Verify Mul 252.

                // Double Karatsuba N 7 Limb Max Bound 511.

                // Single Karatsuba N 7.

                let z0_tmp_fec87_27 = [
                    ((unpacked_limb_0_col10) * (mul_res_limb_0_col28)),
                    (((unpacked_limb_0_col10) * (mul_res_limb_1_col29))
                        + ((unpacked_limb_1_col11) * (mul_res_limb_0_col28))),
                    ((((unpacked_limb_0_col10) * (mul_res_limb_2_col30))
                        + ((unpacked_limb_1_col11) * (mul_res_limb_1_col29)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(2))
                            * (mul_res_limb_0_col28))),
                    (((((unpacked_limb_0_col10) * (mul_res_limb_3_col31))
                        + ((unpacked_limb_1_col11) * (mul_res_limb_2_col30)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(2))
                            * (mul_res_limb_1_col29)))
                        + ((unpacked_limb_3_col12) * (mul_res_limb_0_col28))),
                    ((((((unpacked_limb_0_col10) * (mul_res_limb_4_col32))
                        + ((unpacked_limb_1_col11) * (mul_res_limb_3_col31)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(2))
                            * (mul_res_limb_2_col30)))
                        + ((unpacked_limb_3_col12) * (mul_res_limb_1_col29)))
                        + ((unpacked_limb_4_col13) * (mul_res_limb_0_col28))),
                    (((((((unpacked_limb_0_col10) * (mul_res_limb_5_col33))
                        + ((unpacked_limb_1_col11) * (mul_res_limb_4_col32)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(2))
                            * (mul_res_limb_3_col31)))
                        + ((unpacked_limb_3_col12) * (mul_res_limb_2_col30)))
                        + ((unpacked_limb_4_col13) * (mul_res_limb_1_col29)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(5))
                            * (mul_res_limb_0_col28))),
                    ((((((((unpacked_limb_0_col10) * (mul_res_limb_6_col34))
                        + ((unpacked_limb_1_col11) * (mul_res_limb_5_col33)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(2))
                            * (mul_res_limb_4_col32)))
                        + ((unpacked_limb_3_col12) * (mul_res_limb_3_col31)))
                        + ((unpacked_limb_4_col13) * (mul_res_limb_2_col30)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(5))
                            * (mul_res_limb_1_col29)))
                        + ((unpacked_limb_6_col14) * (mul_res_limb_0_col28))),
                    (((((((unpacked_limb_1_col11) * (mul_res_limb_6_col34))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(2))
                            * (mul_res_limb_5_col33)))
                        + ((unpacked_limb_3_col12) * (mul_res_limb_4_col32)))
                        + ((unpacked_limb_4_col13) * (mul_res_limb_3_col31)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(5))
                            * (mul_res_limb_2_col30)))
                        + ((unpacked_limb_6_col14) * (mul_res_limb_1_col29))),
                    ((((((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                        .get_m31(2))
                        * (mul_res_limb_6_col34))
                        + ((unpacked_limb_3_col12) * (mul_res_limb_5_col33)))
                        + ((unpacked_limb_4_col13) * (mul_res_limb_4_col32)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(5))
                            * (mul_res_limb_3_col31)))
                        + ((unpacked_limb_6_col14) * (mul_res_limb_2_col30))),
                    (((((unpacked_limb_3_col12) * (mul_res_limb_6_col34))
                        + ((unpacked_limb_4_col13) * (mul_res_limb_5_col33)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(5))
                            * (mul_res_limb_4_col32)))
                        + ((unpacked_limb_6_col14) * (mul_res_limb_3_col31))),
                    ((((unpacked_limb_4_col13) * (mul_res_limb_6_col34))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(5))
                            * (mul_res_limb_5_col33)))
                        + ((unpacked_limb_6_col14) * (mul_res_limb_4_col32))),
                    (((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(5))
                        * (mul_res_limb_6_col34))
                        + ((unpacked_limb_6_col14) * (mul_res_limb_5_col33))),
                    ((unpacked_limb_6_col14) * (mul_res_limb_6_col34)),
                ];
                let z2_tmp_fec87_28 = [
                    ((unpacked_limb_7_col15) * (mul_res_limb_7_col35)),
                    (((unpacked_limb_7_col15) * (mul_res_limb_8_col36))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(8))
                            * (mul_res_limb_7_col35))),
                    ((((unpacked_limb_7_col15) * (mul_res_limb_9_col37))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(8))
                            * (mul_res_limb_8_col36)))
                        + ((unpacked_limb_9_col16) * (mul_res_limb_7_col35))),
                    (((((unpacked_limb_7_col15) * (mul_res_limb_10_col38))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(8))
                            * (mul_res_limb_9_col37)))
                        + ((unpacked_limb_9_col16) * (mul_res_limb_8_col36)))
                        + ((unpacked_limb_10_col17) * (mul_res_limb_7_col35))),
                    ((((((unpacked_limb_7_col15) * (mul_res_limb_11_col39))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(8))
                            * (mul_res_limb_10_col38)))
                        + ((unpacked_limb_9_col16) * (mul_res_limb_9_col37)))
                        + ((unpacked_limb_10_col17) * (mul_res_limb_8_col36)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(11))
                            * (mul_res_limb_7_col35))),
                    (((((((unpacked_limb_7_col15) * (mul_res_limb_12_col40))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(8))
                            * (mul_res_limb_11_col39)))
                        + ((unpacked_limb_9_col16) * (mul_res_limb_10_col38)))
                        + ((unpacked_limb_10_col17) * (mul_res_limb_9_col37)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(11))
                            * (mul_res_limb_8_col36)))
                        + ((unpacked_limb_12_col18) * (mul_res_limb_7_col35))),
                    ((((((((unpacked_limb_7_col15) * (mul_res_limb_13_col41))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(8))
                            * (mul_res_limb_12_col40)))
                        + ((unpacked_limb_9_col16) * (mul_res_limb_11_col39)))
                        + ((unpacked_limb_10_col17) * (mul_res_limb_10_col38)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(11))
                            * (mul_res_limb_9_col37)))
                        + ((unpacked_limb_12_col18) * (mul_res_limb_8_col36)))
                        + ((unpacked_limb_13_col19) * (mul_res_limb_7_col35))),
                    (((((((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                        .get_m31(8))
                        * (mul_res_limb_13_col41))
                        + ((unpacked_limb_9_col16) * (mul_res_limb_12_col40)))
                        + ((unpacked_limb_10_col17) * (mul_res_limb_11_col39)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(11))
                            * (mul_res_limb_10_col38)))
                        + ((unpacked_limb_12_col18) * (mul_res_limb_9_col37)))
                        + ((unpacked_limb_13_col19) * (mul_res_limb_8_col36))),
                    ((((((unpacked_limb_9_col16) * (mul_res_limb_13_col41))
                        + ((unpacked_limb_10_col17) * (mul_res_limb_12_col40)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(11))
                            * (mul_res_limb_11_col39)))
                        + ((unpacked_limb_12_col18) * (mul_res_limb_10_col38)))
                        + ((unpacked_limb_13_col19) * (mul_res_limb_9_col37))),
                    (((((unpacked_limb_10_col17) * (mul_res_limb_13_col41))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(11))
                            * (mul_res_limb_12_col40)))
                        + ((unpacked_limb_12_col18) * (mul_res_limb_11_col39)))
                        + ((unpacked_limb_13_col19) * (mul_res_limb_10_col38))),
                    ((((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                        .get_m31(11))
                        * (mul_res_limb_13_col41))
                        + ((unpacked_limb_12_col18) * (mul_res_limb_12_col40)))
                        + ((unpacked_limb_13_col19) * (mul_res_limb_11_col39))),
                    (((unpacked_limb_12_col18) * (mul_res_limb_13_col41))
                        + ((unpacked_limb_13_col19) * (mul_res_limb_12_col40))),
                    ((unpacked_limb_13_col19) * (mul_res_limb_13_col41)),
                ];
                let x_sum_tmp_fec87_29 = [
                    ((unpacked_limb_0_col10) + (unpacked_limb_7_col15)),
                    ((unpacked_limb_1_col11)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(8))),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(2))
                        + (unpacked_limb_9_col16)),
                    ((unpacked_limb_3_col12) + (unpacked_limb_10_col17)),
                    ((unpacked_limb_4_col13)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(11))),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(5))
                        + (unpacked_limb_12_col18)),
                    ((unpacked_limb_6_col14) + (unpacked_limb_13_col19)),
                ];
                let y_sum_tmp_fec87_30 = [
                    ((mul_res_limb_0_col28) + (mul_res_limb_7_col35)),
                    ((mul_res_limb_1_col29) + (mul_res_limb_8_col36)),
                    ((mul_res_limb_2_col30) + (mul_res_limb_9_col37)),
                    ((mul_res_limb_3_col31) + (mul_res_limb_10_col38)),
                    ((mul_res_limb_4_col32) + (mul_res_limb_11_col39)),
                    ((mul_res_limb_5_col33) + (mul_res_limb_12_col40)),
                    ((mul_res_limb_6_col34) + (mul_res_limb_13_col41)),
                ];
                let single_karatsuba_n_7_output_tmp_fec87_31 = [
                    z0_tmp_fec87_27[0],
                    z0_tmp_fec87_27[1],
                    z0_tmp_fec87_27[2],
                    z0_tmp_fec87_27[3],
                    z0_tmp_fec87_27[4],
                    z0_tmp_fec87_27[5],
                    z0_tmp_fec87_27[6],
                    ((z0_tmp_fec87_27[7])
                        + ((((x_sum_tmp_fec87_29[0]) * (y_sum_tmp_fec87_30[0]))
                            - (z0_tmp_fec87_27[0]))
                            - (z2_tmp_fec87_28[0]))),
                    ((z0_tmp_fec87_27[8])
                        + (((((x_sum_tmp_fec87_29[0]) * (y_sum_tmp_fec87_30[1]))
                            + ((x_sum_tmp_fec87_29[1]) * (y_sum_tmp_fec87_30[0])))
                            - (z0_tmp_fec87_27[1]))
                            - (z2_tmp_fec87_28[1]))),
                    ((z0_tmp_fec87_27[9])
                        + ((((((x_sum_tmp_fec87_29[0]) * (y_sum_tmp_fec87_30[2]))
                            + ((x_sum_tmp_fec87_29[1]) * (y_sum_tmp_fec87_30[1])))
                            + ((x_sum_tmp_fec87_29[2]) * (y_sum_tmp_fec87_30[0])))
                            - (z0_tmp_fec87_27[2]))
                            - (z2_tmp_fec87_28[2]))),
                    ((z0_tmp_fec87_27[10])
                        + (((((((x_sum_tmp_fec87_29[0]) * (y_sum_tmp_fec87_30[3]))
                            + ((x_sum_tmp_fec87_29[1]) * (y_sum_tmp_fec87_30[2])))
                            + ((x_sum_tmp_fec87_29[2]) * (y_sum_tmp_fec87_30[1])))
                            + ((x_sum_tmp_fec87_29[3]) * (y_sum_tmp_fec87_30[0])))
                            - (z0_tmp_fec87_27[3]))
                            - (z2_tmp_fec87_28[3]))),
                    ((z0_tmp_fec87_27[11])
                        + ((((((((x_sum_tmp_fec87_29[0]) * (y_sum_tmp_fec87_30[4]))
                            + ((x_sum_tmp_fec87_29[1]) * (y_sum_tmp_fec87_30[3])))
                            + ((x_sum_tmp_fec87_29[2]) * (y_sum_tmp_fec87_30[2])))
                            + ((x_sum_tmp_fec87_29[3]) * (y_sum_tmp_fec87_30[1])))
                            + ((x_sum_tmp_fec87_29[4]) * (y_sum_tmp_fec87_30[0])))
                            - (z0_tmp_fec87_27[4]))
                            - (z2_tmp_fec87_28[4]))),
                    ((z0_tmp_fec87_27[12])
                        + (((((((((x_sum_tmp_fec87_29[0]) * (y_sum_tmp_fec87_30[5]))
                            + ((x_sum_tmp_fec87_29[1]) * (y_sum_tmp_fec87_30[4])))
                            + ((x_sum_tmp_fec87_29[2]) * (y_sum_tmp_fec87_30[3])))
                            + ((x_sum_tmp_fec87_29[3]) * (y_sum_tmp_fec87_30[2])))
                            + ((x_sum_tmp_fec87_29[4]) * (y_sum_tmp_fec87_30[1])))
                            + ((x_sum_tmp_fec87_29[5]) * (y_sum_tmp_fec87_30[0])))
                            - (z0_tmp_fec87_27[5]))
                            - (z2_tmp_fec87_28[5]))),
                    ((((((((((x_sum_tmp_fec87_29[0]) * (y_sum_tmp_fec87_30[6]))
                        + ((x_sum_tmp_fec87_29[1]) * (y_sum_tmp_fec87_30[5])))
                        + ((x_sum_tmp_fec87_29[2]) * (y_sum_tmp_fec87_30[4])))
                        + ((x_sum_tmp_fec87_29[3]) * (y_sum_tmp_fec87_30[3])))
                        + ((x_sum_tmp_fec87_29[4]) * (y_sum_tmp_fec87_30[2])))
                        + ((x_sum_tmp_fec87_29[5]) * (y_sum_tmp_fec87_30[1])))
                        + ((x_sum_tmp_fec87_29[6]) * (y_sum_tmp_fec87_30[0])))
                        - (z0_tmp_fec87_27[6]))
                        - (z2_tmp_fec87_28[6])),
                    ((z2_tmp_fec87_28[0])
                        + (((((((((x_sum_tmp_fec87_29[1]) * (y_sum_tmp_fec87_30[6]))
                            + ((x_sum_tmp_fec87_29[2]) * (y_sum_tmp_fec87_30[5])))
                            + ((x_sum_tmp_fec87_29[3]) * (y_sum_tmp_fec87_30[4])))
                            + ((x_sum_tmp_fec87_29[4]) * (y_sum_tmp_fec87_30[3])))
                            + ((x_sum_tmp_fec87_29[5]) * (y_sum_tmp_fec87_30[2])))
                            + ((x_sum_tmp_fec87_29[6]) * (y_sum_tmp_fec87_30[1])))
                            - (z0_tmp_fec87_27[7]))
                            - (z2_tmp_fec87_28[7]))),
                    ((z2_tmp_fec87_28[1])
                        + ((((((((x_sum_tmp_fec87_29[2]) * (y_sum_tmp_fec87_30[6]))
                            + ((x_sum_tmp_fec87_29[3]) * (y_sum_tmp_fec87_30[5])))
                            + ((x_sum_tmp_fec87_29[4]) * (y_sum_tmp_fec87_30[4])))
                            + ((x_sum_tmp_fec87_29[5]) * (y_sum_tmp_fec87_30[3])))
                            + ((x_sum_tmp_fec87_29[6]) * (y_sum_tmp_fec87_30[2])))
                            - (z0_tmp_fec87_27[8]))
                            - (z2_tmp_fec87_28[8]))),
                    ((z2_tmp_fec87_28[2])
                        + (((((((x_sum_tmp_fec87_29[3]) * (y_sum_tmp_fec87_30[6]))
                            + ((x_sum_tmp_fec87_29[4]) * (y_sum_tmp_fec87_30[5])))
                            + ((x_sum_tmp_fec87_29[5]) * (y_sum_tmp_fec87_30[4])))
                            + ((x_sum_tmp_fec87_29[6]) * (y_sum_tmp_fec87_30[3])))
                            - (z0_tmp_fec87_27[9]))
                            - (z2_tmp_fec87_28[9]))),
                    ((z2_tmp_fec87_28[3])
                        + ((((((x_sum_tmp_fec87_29[4]) * (y_sum_tmp_fec87_30[6]))
                            + ((x_sum_tmp_fec87_29[5]) * (y_sum_tmp_fec87_30[5])))
                            + ((x_sum_tmp_fec87_29[6]) * (y_sum_tmp_fec87_30[4])))
                            - (z0_tmp_fec87_27[10]))
                            - (z2_tmp_fec87_28[10]))),
                    ((z2_tmp_fec87_28[4])
                        + (((((x_sum_tmp_fec87_29[5]) * (y_sum_tmp_fec87_30[6]))
                            + ((x_sum_tmp_fec87_29[6]) * (y_sum_tmp_fec87_30[5])))
                            - (z0_tmp_fec87_27[11]))
                            - (z2_tmp_fec87_28[11]))),
                    ((z2_tmp_fec87_28[5])
                        + ((((x_sum_tmp_fec87_29[6]) * (y_sum_tmp_fec87_30[6]))
                            - (z0_tmp_fec87_27[12]))
                            - (z2_tmp_fec87_28[12]))),
                    z2_tmp_fec87_28[6],
                    z2_tmp_fec87_28[7],
                    z2_tmp_fec87_28[8],
                    z2_tmp_fec87_28[9],
                    z2_tmp_fec87_28[10],
                    z2_tmp_fec87_28[11],
                    z2_tmp_fec87_28[12],
                ];

                // Single Karatsuba N 7.

                let z0_tmp_fec87_32 = [
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(14))
                        * (mul_res_limb_14_col42)),
                    (((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(14))
                        * (mul_res_limb_15_col43))
                        + ((unpacked_limb_15_col20) * (mul_res_limb_14_col42))),
                    ((((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                        .get_m31(14))
                        * (mul_res_limb_16_col44))
                        + ((unpacked_limb_15_col20) * (mul_res_limb_15_col43)))
                        + ((unpacked_limb_16_col21) * (mul_res_limb_14_col42))),
                    (((((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                        .get_m31(14))
                        * (mul_res_limb_17_col45))
                        + ((unpacked_limb_15_col20) * (mul_res_limb_16_col44)))
                        + ((unpacked_limb_16_col21) * (mul_res_limb_15_col43)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(17))
                            * (mul_res_limb_14_col42))),
                    ((((((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                        .get_m31(14))
                        * (mul_res_limb_18_col46))
                        + ((unpacked_limb_15_col20) * (mul_res_limb_17_col45)))
                        + ((unpacked_limb_16_col21) * (mul_res_limb_16_col44)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(17))
                            * (mul_res_limb_15_col43)))
                        + ((unpacked_limb_18_col22) * (mul_res_limb_14_col42))),
                    (((((((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                        .get_m31(14))
                        * (mul_res_limb_19_col47))
                        + ((unpacked_limb_15_col20) * (mul_res_limb_18_col46)))
                        + ((unpacked_limb_16_col21) * (mul_res_limb_17_col45)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(17))
                            * (mul_res_limb_16_col44)))
                        + ((unpacked_limb_18_col22) * (mul_res_limb_15_col43)))
                        + ((unpacked_limb_19_col23) * (mul_res_limb_14_col42))),
                    ((((((((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                        .get_m31(14))
                        * (mul_res_limb_20_col48))
                        + ((unpacked_limb_15_col20) * (mul_res_limb_19_col47)))
                        + ((unpacked_limb_16_col21) * (mul_res_limb_18_col46)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(17))
                            * (mul_res_limb_17_col45)))
                        + ((unpacked_limb_18_col22) * (mul_res_limb_16_col44)))
                        + ((unpacked_limb_19_col23) * (mul_res_limb_15_col43)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(20))
                            * (mul_res_limb_14_col42))),
                    (((((((unpacked_limb_15_col20) * (mul_res_limb_20_col48))
                        + ((unpacked_limb_16_col21) * (mul_res_limb_19_col47)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(17))
                            * (mul_res_limb_18_col46)))
                        + ((unpacked_limb_18_col22) * (mul_res_limb_17_col45)))
                        + ((unpacked_limb_19_col23) * (mul_res_limb_16_col44)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(20))
                            * (mul_res_limb_15_col43))),
                    ((((((unpacked_limb_16_col21) * (mul_res_limb_20_col48))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(17))
                            * (mul_res_limb_19_col47)))
                        + ((unpacked_limb_18_col22) * (mul_res_limb_18_col46)))
                        + ((unpacked_limb_19_col23) * (mul_res_limb_17_col45)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(20))
                            * (mul_res_limb_16_col44))),
                    (((((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                        .get_m31(17))
                        * (mul_res_limb_20_col48))
                        + ((unpacked_limb_18_col22) * (mul_res_limb_19_col47)))
                        + ((unpacked_limb_19_col23) * (mul_res_limb_18_col46)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(20))
                            * (mul_res_limb_17_col45))),
                    ((((unpacked_limb_18_col22) * (mul_res_limb_20_col48))
                        + ((unpacked_limb_19_col23) * (mul_res_limb_19_col47)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(20))
                            * (mul_res_limb_18_col46))),
                    (((unpacked_limb_19_col23) * (mul_res_limb_20_col48))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(20))
                            * (mul_res_limb_19_col47))),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(20))
                        * (mul_res_limb_20_col48)),
                ];
                let z2_tmp_fec87_33 = [
                    ((unpacked_limb_21_col24) * (mul_res_limb_21_col49)),
                    (((unpacked_limb_21_col24) * (mul_res_limb_22_col50))
                        + ((unpacked_limb_22_col25) * (mul_res_limb_21_col49))),
                    ((((unpacked_limb_21_col24) * (mul_res_limb_23_col51))
                        + ((unpacked_limb_22_col25) * (mul_res_limb_22_col50)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(23))
                            * (mul_res_limb_21_col49))),
                    (((((unpacked_limb_21_col24) * (mul_res_limb_24_col52))
                        + ((unpacked_limb_22_col25) * (mul_res_limb_23_col51)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(23))
                            * (mul_res_limb_22_col50)))
                        + ((unpacked_limb_24_col26) * (mul_res_limb_21_col49))),
                    ((((((unpacked_limb_21_col24) * (mul_res_limb_25_col53))
                        + ((unpacked_limb_22_col25) * (mul_res_limb_24_col52)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(23))
                            * (mul_res_limb_23_col51)))
                        + ((unpacked_limb_24_col26) * (mul_res_limb_22_col50)))
                        + ((unpacked_limb_25_col27) * (mul_res_limb_21_col49))),
                    (((((((unpacked_limb_21_col24) * (mul_res_limb_26_col54))
                        + ((unpacked_limb_22_col25) * (mul_res_limb_25_col53)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(23))
                            * (mul_res_limb_24_col52)))
                        + ((unpacked_limb_24_col26) * (mul_res_limb_23_col51)))
                        + ((unpacked_limb_25_col27) * (mul_res_limb_22_col50)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(26))
                            * (mul_res_limb_21_col49))),
                    ((((((((unpacked_limb_21_col24) * (mul_res_limb_27_col55))
                        + ((unpacked_limb_22_col25) * (mul_res_limb_26_col54)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(23))
                            * (mul_res_limb_25_col53)))
                        + ((unpacked_limb_24_col26) * (mul_res_limb_24_col52)))
                        + ((unpacked_limb_25_col27) * (mul_res_limb_23_col51)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(26))
                            * (mul_res_limb_22_col50)))
                        + ((input_limb_9_col9) * (mul_res_limb_21_col49))),
                    (((((((unpacked_limb_22_col25) * (mul_res_limb_27_col55))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(23))
                            * (mul_res_limb_26_col54)))
                        + ((unpacked_limb_24_col26) * (mul_res_limb_25_col53)))
                        + ((unpacked_limb_25_col27) * (mul_res_limb_24_col52)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(26))
                            * (mul_res_limb_23_col51)))
                        + ((input_limb_9_col9) * (mul_res_limb_22_col50))),
                    ((((((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                        .get_m31(23))
                        * (mul_res_limb_27_col55))
                        + ((unpacked_limb_24_col26) * (mul_res_limb_26_col54)))
                        + ((unpacked_limb_25_col27) * (mul_res_limb_25_col53)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(26))
                            * (mul_res_limb_24_col52)))
                        + ((input_limb_9_col9) * (mul_res_limb_23_col51))),
                    (((((unpacked_limb_24_col26) * (mul_res_limb_27_col55))
                        + ((unpacked_limb_25_col27) * (mul_res_limb_26_col54)))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(26))
                            * (mul_res_limb_25_col53)))
                        + ((input_limb_9_col9) * (mul_res_limb_24_col52))),
                    ((((unpacked_limb_25_col27) * (mul_res_limb_27_col55))
                        + ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(26))
                            * (mul_res_limb_26_col54)))
                        + ((input_limb_9_col9) * (mul_res_limb_25_col53))),
                    (((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(26))
                        * (mul_res_limb_27_col55))
                        + ((input_limb_9_col9) * (mul_res_limb_26_col54))),
                    ((input_limb_9_col9) * (mul_res_limb_27_col55)),
                ];
                let x_sum_tmp_fec87_34 = [
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(14))
                        + (unpacked_limb_21_col24)),
                    ((unpacked_limb_15_col20) + (unpacked_limb_22_col25)),
                    ((unpacked_limb_16_col21)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(23))),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(17))
                        + (unpacked_limb_24_col26)),
                    ((unpacked_limb_18_col22) + (unpacked_limb_25_col27)),
                    ((unpacked_limb_19_col23)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(26))),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(20))
                        + (input_limb_9_col9)),
                ];
                let y_sum_tmp_fec87_35 = [
                    ((mul_res_limb_14_col42) + (mul_res_limb_21_col49)),
                    ((mul_res_limb_15_col43) + (mul_res_limb_22_col50)),
                    ((mul_res_limb_16_col44) + (mul_res_limb_23_col51)),
                    ((mul_res_limb_17_col45) + (mul_res_limb_24_col52)),
                    ((mul_res_limb_18_col46) + (mul_res_limb_25_col53)),
                    ((mul_res_limb_19_col47) + (mul_res_limb_26_col54)),
                    ((mul_res_limb_20_col48) + (mul_res_limb_27_col55)),
                ];
                let single_karatsuba_n_7_output_tmp_fec87_36 = [
                    z0_tmp_fec87_32[0],
                    z0_tmp_fec87_32[1],
                    z0_tmp_fec87_32[2],
                    z0_tmp_fec87_32[3],
                    z0_tmp_fec87_32[4],
                    z0_tmp_fec87_32[5],
                    z0_tmp_fec87_32[6],
                    ((z0_tmp_fec87_32[7])
                        + ((((x_sum_tmp_fec87_34[0]) * (y_sum_tmp_fec87_35[0]))
                            - (z0_tmp_fec87_32[0]))
                            - (z2_tmp_fec87_33[0]))),
                    ((z0_tmp_fec87_32[8])
                        + (((((x_sum_tmp_fec87_34[0]) * (y_sum_tmp_fec87_35[1]))
                            + ((x_sum_tmp_fec87_34[1]) * (y_sum_tmp_fec87_35[0])))
                            - (z0_tmp_fec87_32[1]))
                            - (z2_tmp_fec87_33[1]))),
                    ((z0_tmp_fec87_32[9])
                        + ((((((x_sum_tmp_fec87_34[0]) * (y_sum_tmp_fec87_35[2]))
                            + ((x_sum_tmp_fec87_34[1]) * (y_sum_tmp_fec87_35[1])))
                            + ((x_sum_tmp_fec87_34[2]) * (y_sum_tmp_fec87_35[0])))
                            - (z0_tmp_fec87_32[2]))
                            - (z2_tmp_fec87_33[2]))),
                    ((z0_tmp_fec87_32[10])
                        + (((((((x_sum_tmp_fec87_34[0]) * (y_sum_tmp_fec87_35[3]))
                            + ((x_sum_tmp_fec87_34[1]) * (y_sum_tmp_fec87_35[2])))
                            + ((x_sum_tmp_fec87_34[2]) * (y_sum_tmp_fec87_35[1])))
                            + ((x_sum_tmp_fec87_34[3]) * (y_sum_tmp_fec87_35[0])))
                            - (z0_tmp_fec87_32[3]))
                            - (z2_tmp_fec87_33[3]))),
                    ((z0_tmp_fec87_32[11])
                        + ((((((((x_sum_tmp_fec87_34[0]) * (y_sum_tmp_fec87_35[4]))
                            + ((x_sum_tmp_fec87_34[1]) * (y_sum_tmp_fec87_35[3])))
                            + ((x_sum_tmp_fec87_34[2]) * (y_sum_tmp_fec87_35[2])))
                            + ((x_sum_tmp_fec87_34[3]) * (y_sum_tmp_fec87_35[1])))
                            + ((x_sum_tmp_fec87_34[4]) * (y_sum_tmp_fec87_35[0])))
                            - (z0_tmp_fec87_32[4]))
                            - (z2_tmp_fec87_33[4]))),
                    ((z0_tmp_fec87_32[12])
                        + (((((((((x_sum_tmp_fec87_34[0]) * (y_sum_tmp_fec87_35[5]))
                            + ((x_sum_tmp_fec87_34[1]) * (y_sum_tmp_fec87_35[4])))
                            + ((x_sum_tmp_fec87_34[2]) * (y_sum_tmp_fec87_35[3])))
                            + ((x_sum_tmp_fec87_34[3]) * (y_sum_tmp_fec87_35[2])))
                            + ((x_sum_tmp_fec87_34[4]) * (y_sum_tmp_fec87_35[1])))
                            + ((x_sum_tmp_fec87_34[5]) * (y_sum_tmp_fec87_35[0])))
                            - (z0_tmp_fec87_32[5]))
                            - (z2_tmp_fec87_33[5]))),
                    ((((((((((x_sum_tmp_fec87_34[0]) * (y_sum_tmp_fec87_35[6]))
                        + ((x_sum_tmp_fec87_34[1]) * (y_sum_tmp_fec87_35[5])))
                        + ((x_sum_tmp_fec87_34[2]) * (y_sum_tmp_fec87_35[4])))
                        + ((x_sum_tmp_fec87_34[3]) * (y_sum_tmp_fec87_35[3])))
                        + ((x_sum_tmp_fec87_34[4]) * (y_sum_tmp_fec87_35[2])))
                        + ((x_sum_tmp_fec87_34[5]) * (y_sum_tmp_fec87_35[1])))
                        + ((x_sum_tmp_fec87_34[6]) * (y_sum_tmp_fec87_35[0])))
                        - (z0_tmp_fec87_32[6]))
                        - (z2_tmp_fec87_33[6])),
                    ((z2_tmp_fec87_33[0])
                        + (((((((((x_sum_tmp_fec87_34[1]) * (y_sum_tmp_fec87_35[6]))
                            + ((x_sum_tmp_fec87_34[2]) * (y_sum_tmp_fec87_35[5])))
                            + ((x_sum_tmp_fec87_34[3]) * (y_sum_tmp_fec87_35[4])))
                            + ((x_sum_tmp_fec87_34[4]) * (y_sum_tmp_fec87_35[3])))
                            + ((x_sum_tmp_fec87_34[5]) * (y_sum_tmp_fec87_35[2])))
                            + ((x_sum_tmp_fec87_34[6]) * (y_sum_tmp_fec87_35[1])))
                            - (z0_tmp_fec87_32[7]))
                            - (z2_tmp_fec87_33[7]))),
                    ((z2_tmp_fec87_33[1])
                        + ((((((((x_sum_tmp_fec87_34[2]) * (y_sum_tmp_fec87_35[6]))
                            + ((x_sum_tmp_fec87_34[3]) * (y_sum_tmp_fec87_35[5])))
                            + ((x_sum_tmp_fec87_34[4]) * (y_sum_tmp_fec87_35[4])))
                            + ((x_sum_tmp_fec87_34[5]) * (y_sum_tmp_fec87_35[3])))
                            + ((x_sum_tmp_fec87_34[6]) * (y_sum_tmp_fec87_35[2])))
                            - (z0_tmp_fec87_32[8]))
                            - (z2_tmp_fec87_33[8]))),
                    ((z2_tmp_fec87_33[2])
                        + (((((((x_sum_tmp_fec87_34[3]) * (y_sum_tmp_fec87_35[6]))
                            + ((x_sum_tmp_fec87_34[4]) * (y_sum_tmp_fec87_35[5])))
                            + ((x_sum_tmp_fec87_34[5]) * (y_sum_tmp_fec87_35[4])))
                            + ((x_sum_tmp_fec87_34[6]) * (y_sum_tmp_fec87_35[3])))
                            - (z0_tmp_fec87_32[9]))
                            - (z2_tmp_fec87_33[9]))),
                    ((z2_tmp_fec87_33[3])
                        + ((((((x_sum_tmp_fec87_34[4]) * (y_sum_tmp_fec87_35[6]))
                            + ((x_sum_tmp_fec87_34[5]) * (y_sum_tmp_fec87_35[5])))
                            + ((x_sum_tmp_fec87_34[6]) * (y_sum_tmp_fec87_35[4])))
                            - (z0_tmp_fec87_32[10]))
                            - (z2_tmp_fec87_33[10]))),
                    ((z2_tmp_fec87_33[4])
                        + (((((x_sum_tmp_fec87_34[5]) * (y_sum_tmp_fec87_35[6]))
                            + ((x_sum_tmp_fec87_34[6]) * (y_sum_tmp_fec87_35[5])))
                            - (z0_tmp_fec87_32[11]))
                            - (z2_tmp_fec87_33[11]))),
                    ((z2_tmp_fec87_33[5])
                        + ((((x_sum_tmp_fec87_34[6]) * (y_sum_tmp_fec87_35[6]))
                            - (z0_tmp_fec87_32[12]))
                            - (z2_tmp_fec87_33[12]))),
                    z2_tmp_fec87_33[6],
                    z2_tmp_fec87_33[7],
                    z2_tmp_fec87_33[8],
                    z2_tmp_fec87_33[9],
                    z2_tmp_fec87_33[10],
                    z2_tmp_fec87_33[11],
                    z2_tmp_fec87_33[12],
                ];

                let x_sum_tmp_fec87_37 = [
                    ((unpacked_limb_0_col10)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(14))),
                    ((unpacked_limb_1_col11) + (unpacked_limb_15_col20)),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(2))
                        + (unpacked_limb_16_col21)),
                    ((unpacked_limb_3_col12)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(17))),
                    ((unpacked_limb_4_col13) + (unpacked_limb_18_col22)),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(5))
                        + (unpacked_limb_19_col23)),
                    ((unpacked_limb_6_col14)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(20))),
                    ((unpacked_limb_7_col15) + (unpacked_limb_21_col24)),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(8))
                        + (unpacked_limb_22_col25)),
                    ((unpacked_limb_9_col16)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(23))),
                    ((unpacked_limb_10_col17) + (unpacked_limb_24_col26)),
                    ((felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2.get_m31(11))
                        + (unpacked_limb_25_col27)),
                    ((unpacked_limb_12_col18)
                        + (felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2
                            .get_m31(26))),
                    ((unpacked_limb_13_col19) + (input_limb_9_col9)),
                ];
                let y_sum_tmp_fec87_38 = [
                    ((mul_res_limb_0_col28) + (mul_res_limb_14_col42)),
                    ((mul_res_limb_1_col29) + (mul_res_limb_15_col43)),
                    ((mul_res_limb_2_col30) + (mul_res_limb_16_col44)),
                    ((mul_res_limb_3_col31) + (mul_res_limb_17_col45)),
                    ((mul_res_limb_4_col32) + (mul_res_limb_18_col46)),
                    ((mul_res_limb_5_col33) + (mul_res_limb_19_col47)),
                    ((mul_res_limb_6_col34) + (mul_res_limb_20_col48)),
                    ((mul_res_limb_7_col35) + (mul_res_limb_21_col49)),
                    ((mul_res_limb_8_col36) + (mul_res_limb_22_col50)),
                    ((mul_res_limb_9_col37) + (mul_res_limb_23_col51)),
                    ((mul_res_limb_10_col38) + (mul_res_limb_24_col52)),
                    ((mul_res_limb_11_col39) + (mul_res_limb_25_col53)),
                    ((mul_res_limb_12_col40) + (mul_res_limb_26_col54)),
                    ((mul_res_limb_13_col41) + (mul_res_limb_27_col55)),
                ];

                // Single Karatsuba N 7.

                let z0_tmp_fec87_39 = [
                    ((x_sum_tmp_fec87_37[0]) * (y_sum_tmp_fec87_38[0])),
                    (((x_sum_tmp_fec87_37[0]) * (y_sum_tmp_fec87_38[1]))
                        + ((x_sum_tmp_fec87_37[1]) * (y_sum_tmp_fec87_38[0]))),
                    ((((x_sum_tmp_fec87_37[0]) * (y_sum_tmp_fec87_38[2]))
                        + ((x_sum_tmp_fec87_37[1]) * (y_sum_tmp_fec87_38[1])))
                        + ((x_sum_tmp_fec87_37[2]) * (y_sum_tmp_fec87_38[0]))),
                    (((((x_sum_tmp_fec87_37[0]) * (y_sum_tmp_fec87_38[3]))
                        + ((x_sum_tmp_fec87_37[1]) * (y_sum_tmp_fec87_38[2])))
                        + ((x_sum_tmp_fec87_37[2]) * (y_sum_tmp_fec87_38[1])))
                        + ((x_sum_tmp_fec87_37[3]) * (y_sum_tmp_fec87_38[0]))),
                    ((((((x_sum_tmp_fec87_37[0]) * (y_sum_tmp_fec87_38[4]))
                        + ((x_sum_tmp_fec87_37[1]) * (y_sum_tmp_fec87_38[3])))
                        + ((x_sum_tmp_fec87_37[2]) * (y_sum_tmp_fec87_38[2])))
                        + ((x_sum_tmp_fec87_37[3]) * (y_sum_tmp_fec87_38[1])))
                        + ((x_sum_tmp_fec87_37[4]) * (y_sum_tmp_fec87_38[0]))),
                    (((((((x_sum_tmp_fec87_37[0]) * (y_sum_tmp_fec87_38[5]))
                        + ((x_sum_tmp_fec87_37[1]) * (y_sum_tmp_fec87_38[4])))
                        + ((x_sum_tmp_fec87_37[2]) * (y_sum_tmp_fec87_38[3])))
                        + ((x_sum_tmp_fec87_37[3]) * (y_sum_tmp_fec87_38[2])))
                        + ((x_sum_tmp_fec87_37[4]) * (y_sum_tmp_fec87_38[1])))
                        + ((x_sum_tmp_fec87_37[5]) * (y_sum_tmp_fec87_38[0]))),
                    ((((((((x_sum_tmp_fec87_37[0]) * (y_sum_tmp_fec87_38[6]))
                        + ((x_sum_tmp_fec87_37[1]) * (y_sum_tmp_fec87_38[5])))
                        + ((x_sum_tmp_fec87_37[2]) * (y_sum_tmp_fec87_38[4])))
                        + ((x_sum_tmp_fec87_37[3]) * (y_sum_tmp_fec87_38[3])))
                        + ((x_sum_tmp_fec87_37[4]) * (y_sum_tmp_fec87_38[2])))
                        + ((x_sum_tmp_fec87_37[5]) * (y_sum_tmp_fec87_38[1])))
                        + ((x_sum_tmp_fec87_37[6]) * (y_sum_tmp_fec87_38[0]))),
                    (((((((x_sum_tmp_fec87_37[1]) * (y_sum_tmp_fec87_38[6]))
                        + ((x_sum_tmp_fec87_37[2]) * (y_sum_tmp_fec87_38[5])))
                        + ((x_sum_tmp_fec87_37[3]) * (y_sum_tmp_fec87_38[4])))
                        + ((x_sum_tmp_fec87_37[4]) * (y_sum_tmp_fec87_38[3])))
                        + ((x_sum_tmp_fec87_37[5]) * (y_sum_tmp_fec87_38[2])))
                        + ((x_sum_tmp_fec87_37[6]) * (y_sum_tmp_fec87_38[1]))),
                    ((((((x_sum_tmp_fec87_37[2]) * (y_sum_tmp_fec87_38[6]))
                        + ((x_sum_tmp_fec87_37[3]) * (y_sum_tmp_fec87_38[5])))
                        + ((x_sum_tmp_fec87_37[4]) * (y_sum_tmp_fec87_38[4])))
                        + ((x_sum_tmp_fec87_37[5]) * (y_sum_tmp_fec87_38[3])))
                        + ((x_sum_tmp_fec87_37[6]) * (y_sum_tmp_fec87_38[2]))),
                    (((((x_sum_tmp_fec87_37[3]) * (y_sum_tmp_fec87_38[6]))
                        + ((x_sum_tmp_fec87_37[4]) * (y_sum_tmp_fec87_38[5])))
                        + ((x_sum_tmp_fec87_37[5]) * (y_sum_tmp_fec87_38[4])))
                        + ((x_sum_tmp_fec87_37[6]) * (y_sum_tmp_fec87_38[3]))),
                    ((((x_sum_tmp_fec87_37[4]) * (y_sum_tmp_fec87_38[6]))
                        + ((x_sum_tmp_fec87_37[5]) * (y_sum_tmp_fec87_38[5])))
                        + ((x_sum_tmp_fec87_37[6]) * (y_sum_tmp_fec87_38[4]))),
                    (((x_sum_tmp_fec87_37[5]) * (y_sum_tmp_fec87_38[6]))
                        + ((x_sum_tmp_fec87_37[6]) * (y_sum_tmp_fec87_38[5]))),
                    ((x_sum_tmp_fec87_37[6]) * (y_sum_tmp_fec87_38[6])),
                ];
                let z2_tmp_fec87_40 = [
                    ((x_sum_tmp_fec87_37[7]) * (y_sum_tmp_fec87_38[7])),
                    (((x_sum_tmp_fec87_37[7]) * (y_sum_tmp_fec87_38[8]))
                        + ((x_sum_tmp_fec87_37[8]) * (y_sum_tmp_fec87_38[7]))),
                    ((((x_sum_tmp_fec87_37[7]) * (y_sum_tmp_fec87_38[9]))
                        + ((x_sum_tmp_fec87_37[8]) * (y_sum_tmp_fec87_38[8])))
                        + ((x_sum_tmp_fec87_37[9]) * (y_sum_tmp_fec87_38[7]))),
                    (((((x_sum_tmp_fec87_37[7]) * (y_sum_tmp_fec87_38[10]))
                        + ((x_sum_tmp_fec87_37[8]) * (y_sum_tmp_fec87_38[9])))
                        + ((x_sum_tmp_fec87_37[9]) * (y_sum_tmp_fec87_38[8])))
                        + ((x_sum_tmp_fec87_37[10]) * (y_sum_tmp_fec87_38[7]))),
                    ((((((x_sum_tmp_fec87_37[7]) * (y_sum_tmp_fec87_38[11]))
                        + ((x_sum_tmp_fec87_37[8]) * (y_sum_tmp_fec87_38[10])))
                        + ((x_sum_tmp_fec87_37[9]) * (y_sum_tmp_fec87_38[9])))
                        + ((x_sum_tmp_fec87_37[10]) * (y_sum_tmp_fec87_38[8])))
                        + ((x_sum_tmp_fec87_37[11]) * (y_sum_tmp_fec87_38[7]))),
                    (((((((x_sum_tmp_fec87_37[7]) * (y_sum_tmp_fec87_38[12]))
                        + ((x_sum_tmp_fec87_37[8]) * (y_sum_tmp_fec87_38[11])))
                        + ((x_sum_tmp_fec87_37[9]) * (y_sum_tmp_fec87_38[10])))
                        + ((x_sum_tmp_fec87_37[10]) * (y_sum_tmp_fec87_38[9])))
                        + ((x_sum_tmp_fec87_37[11]) * (y_sum_tmp_fec87_38[8])))
                        + ((x_sum_tmp_fec87_37[12]) * (y_sum_tmp_fec87_38[7]))),
                    ((((((((x_sum_tmp_fec87_37[7]) * (y_sum_tmp_fec87_38[13]))
                        + ((x_sum_tmp_fec87_37[8]) * (y_sum_tmp_fec87_38[12])))
                        + ((x_sum_tmp_fec87_37[9]) * (y_sum_tmp_fec87_38[11])))
                        + ((x_sum_tmp_fec87_37[10]) * (y_sum_tmp_fec87_38[10])))
                        + ((x_sum_tmp_fec87_37[11]) * (y_sum_tmp_fec87_38[9])))
                        + ((x_sum_tmp_fec87_37[12]) * (y_sum_tmp_fec87_38[8])))
                        + ((x_sum_tmp_fec87_37[13]) * (y_sum_tmp_fec87_38[7]))),
                    (((((((x_sum_tmp_fec87_37[8]) * (y_sum_tmp_fec87_38[13]))
                        + ((x_sum_tmp_fec87_37[9]) * (y_sum_tmp_fec87_38[12])))
                        + ((x_sum_tmp_fec87_37[10]) * (y_sum_tmp_fec87_38[11])))
                        + ((x_sum_tmp_fec87_37[11]) * (y_sum_tmp_fec87_38[10])))
                        + ((x_sum_tmp_fec87_37[12]) * (y_sum_tmp_fec87_38[9])))
                        + ((x_sum_tmp_fec87_37[13]) * (y_sum_tmp_fec87_38[8]))),
                    ((((((x_sum_tmp_fec87_37[9]) * (y_sum_tmp_fec87_38[13]))
                        + ((x_sum_tmp_fec87_37[10]) * (y_sum_tmp_fec87_38[12])))
                        + ((x_sum_tmp_fec87_37[11]) * (y_sum_tmp_fec87_38[11])))
                        + ((x_sum_tmp_fec87_37[12]) * (y_sum_tmp_fec87_38[10])))
                        + ((x_sum_tmp_fec87_37[13]) * (y_sum_tmp_fec87_38[9]))),
                    (((((x_sum_tmp_fec87_37[10]) * (y_sum_tmp_fec87_38[13]))
                        + ((x_sum_tmp_fec87_37[11]) * (y_sum_tmp_fec87_38[12])))
                        + ((x_sum_tmp_fec87_37[12]) * (y_sum_tmp_fec87_38[11])))
                        + ((x_sum_tmp_fec87_37[13]) * (y_sum_tmp_fec87_38[10]))),
                    ((((x_sum_tmp_fec87_37[11]) * (y_sum_tmp_fec87_38[13]))
                        + ((x_sum_tmp_fec87_37[12]) * (y_sum_tmp_fec87_38[12])))
                        + ((x_sum_tmp_fec87_37[13]) * (y_sum_tmp_fec87_38[11]))),
                    (((x_sum_tmp_fec87_37[12]) * (y_sum_tmp_fec87_38[13]))
                        + ((x_sum_tmp_fec87_37[13]) * (y_sum_tmp_fec87_38[12]))),
                    ((x_sum_tmp_fec87_37[13]) * (y_sum_tmp_fec87_38[13])),
                ];
                let x_sum_tmp_fec87_41 = [
                    ((x_sum_tmp_fec87_37[0]) + (x_sum_tmp_fec87_37[7])),
                    ((x_sum_tmp_fec87_37[1]) + (x_sum_tmp_fec87_37[8])),
                    ((x_sum_tmp_fec87_37[2]) + (x_sum_tmp_fec87_37[9])),
                    ((x_sum_tmp_fec87_37[3]) + (x_sum_tmp_fec87_37[10])),
                    ((x_sum_tmp_fec87_37[4]) + (x_sum_tmp_fec87_37[11])),
                    ((x_sum_tmp_fec87_37[5]) + (x_sum_tmp_fec87_37[12])),
                    ((x_sum_tmp_fec87_37[6]) + (x_sum_tmp_fec87_37[13])),
                ];
                let y_sum_tmp_fec87_42 = [
                    ((y_sum_tmp_fec87_38[0]) + (y_sum_tmp_fec87_38[7])),
                    ((y_sum_tmp_fec87_38[1]) + (y_sum_tmp_fec87_38[8])),
                    ((y_sum_tmp_fec87_38[2]) + (y_sum_tmp_fec87_38[9])),
                    ((y_sum_tmp_fec87_38[3]) + (y_sum_tmp_fec87_38[10])),
                    ((y_sum_tmp_fec87_38[4]) + (y_sum_tmp_fec87_38[11])),
                    ((y_sum_tmp_fec87_38[5]) + (y_sum_tmp_fec87_38[12])),
                    ((y_sum_tmp_fec87_38[6]) + (y_sum_tmp_fec87_38[13])),
                ];
                let single_karatsuba_n_7_output_tmp_fec87_43 = [
                    z0_tmp_fec87_39[0],
                    z0_tmp_fec87_39[1],
                    z0_tmp_fec87_39[2],
                    z0_tmp_fec87_39[3],
                    z0_tmp_fec87_39[4],
                    z0_tmp_fec87_39[5],
                    z0_tmp_fec87_39[6],
                    ((z0_tmp_fec87_39[7])
                        + ((((x_sum_tmp_fec87_41[0]) * (y_sum_tmp_fec87_42[0]))
                            - (z0_tmp_fec87_39[0]))
                            - (z2_tmp_fec87_40[0]))),
                    ((z0_tmp_fec87_39[8])
                        + (((((x_sum_tmp_fec87_41[0]) * (y_sum_tmp_fec87_42[1]))
                            + ((x_sum_tmp_fec87_41[1]) * (y_sum_tmp_fec87_42[0])))
                            - (z0_tmp_fec87_39[1]))
                            - (z2_tmp_fec87_40[1]))),
                    ((z0_tmp_fec87_39[9])
                        + ((((((x_sum_tmp_fec87_41[0]) * (y_sum_tmp_fec87_42[2]))
                            + ((x_sum_tmp_fec87_41[1]) * (y_sum_tmp_fec87_42[1])))
                            + ((x_sum_tmp_fec87_41[2]) * (y_sum_tmp_fec87_42[0])))
                            - (z0_tmp_fec87_39[2]))
                            - (z2_tmp_fec87_40[2]))),
                    ((z0_tmp_fec87_39[10])
                        + (((((((x_sum_tmp_fec87_41[0]) * (y_sum_tmp_fec87_42[3]))
                            + ((x_sum_tmp_fec87_41[1]) * (y_sum_tmp_fec87_42[2])))
                            + ((x_sum_tmp_fec87_41[2]) * (y_sum_tmp_fec87_42[1])))
                            + ((x_sum_tmp_fec87_41[3]) * (y_sum_tmp_fec87_42[0])))
                            - (z0_tmp_fec87_39[3]))
                            - (z2_tmp_fec87_40[3]))),
                    ((z0_tmp_fec87_39[11])
                        + ((((((((x_sum_tmp_fec87_41[0]) * (y_sum_tmp_fec87_42[4]))
                            + ((x_sum_tmp_fec87_41[1]) * (y_sum_tmp_fec87_42[3])))
                            + ((x_sum_tmp_fec87_41[2]) * (y_sum_tmp_fec87_42[2])))
                            + ((x_sum_tmp_fec87_41[3]) * (y_sum_tmp_fec87_42[1])))
                            + ((x_sum_tmp_fec87_41[4]) * (y_sum_tmp_fec87_42[0])))
                            - (z0_tmp_fec87_39[4]))
                            - (z2_tmp_fec87_40[4]))),
                    ((z0_tmp_fec87_39[12])
                        + (((((((((x_sum_tmp_fec87_41[0]) * (y_sum_tmp_fec87_42[5]))
                            + ((x_sum_tmp_fec87_41[1]) * (y_sum_tmp_fec87_42[4])))
                            + ((x_sum_tmp_fec87_41[2]) * (y_sum_tmp_fec87_42[3])))
                            + ((x_sum_tmp_fec87_41[3]) * (y_sum_tmp_fec87_42[2])))
                            + ((x_sum_tmp_fec87_41[4]) * (y_sum_tmp_fec87_42[1])))
                            + ((x_sum_tmp_fec87_41[5]) * (y_sum_tmp_fec87_42[0])))
                            - (z0_tmp_fec87_39[5]))
                            - (z2_tmp_fec87_40[5]))),
                    ((((((((((x_sum_tmp_fec87_41[0]) * (y_sum_tmp_fec87_42[6]))
                        + ((x_sum_tmp_fec87_41[1]) * (y_sum_tmp_fec87_42[5])))
                        + ((x_sum_tmp_fec87_41[2]) * (y_sum_tmp_fec87_42[4])))
                        + ((x_sum_tmp_fec87_41[3]) * (y_sum_tmp_fec87_42[3])))
                        + ((x_sum_tmp_fec87_41[4]) * (y_sum_tmp_fec87_42[2])))
                        + ((x_sum_tmp_fec87_41[5]) * (y_sum_tmp_fec87_42[1])))
                        + ((x_sum_tmp_fec87_41[6]) * (y_sum_tmp_fec87_42[0])))
                        - (z0_tmp_fec87_39[6]))
                        - (z2_tmp_fec87_40[6])),
                    ((z2_tmp_fec87_40[0])
                        + (((((((((x_sum_tmp_fec87_41[1]) * (y_sum_tmp_fec87_42[6]))
                            + ((x_sum_tmp_fec87_41[2]) * (y_sum_tmp_fec87_42[5])))
                            + ((x_sum_tmp_fec87_41[3]) * (y_sum_tmp_fec87_42[4])))
                            + ((x_sum_tmp_fec87_41[4]) * (y_sum_tmp_fec87_42[3])))
                            + ((x_sum_tmp_fec87_41[5]) * (y_sum_tmp_fec87_42[2])))
                            + ((x_sum_tmp_fec87_41[6]) * (y_sum_tmp_fec87_42[1])))
                            - (z0_tmp_fec87_39[7]))
                            - (z2_tmp_fec87_40[7]))),
                    ((z2_tmp_fec87_40[1])
                        + ((((((((x_sum_tmp_fec87_41[2]) * (y_sum_tmp_fec87_42[6]))
                            + ((x_sum_tmp_fec87_41[3]) * (y_sum_tmp_fec87_42[5])))
                            + ((x_sum_tmp_fec87_41[4]) * (y_sum_tmp_fec87_42[4])))
                            + ((x_sum_tmp_fec87_41[5]) * (y_sum_tmp_fec87_42[3])))
                            + ((x_sum_tmp_fec87_41[6]) * (y_sum_tmp_fec87_42[2])))
                            - (z0_tmp_fec87_39[8]))
                            - (z2_tmp_fec87_40[8]))),
                    ((z2_tmp_fec87_40[2])
                        + (((((((x_sum_tmp_fec87_41[3]) * (y_sum_tmp_fec87_42[6]))
                            + ((x_sum_tmp_fec87_41[4]) * (y_sum_tmp_fec87_42[5])))
                            + ((x_sum_tmp_fec87_41[5]) * (y_sum_tmp_fec87_42[4])))
                            + ((x_sum_tmp_fec87_41[6]) * (y_sum_tmp_fec87_42[3])))
                            - (z0_tmp_fec87_39[9]))
                            - (z2_tmp_fec87_40[9]))),
                    ((z2_tmp_fec87_40[3])
                        + ((((((x_sum_tmp_fec87_41[4]) * (y_sum_tmp_fec87_42[6]))
                            + ((x_sum_tmp_fec87_41[5]) * (y_sum_tmp_fec87_42[5])))
                            + ((x_sum_tmp_fec87_41[6]) * (y_sum_tmp_fec87_42[4])))
                            - (z0_tmp_fec87_39[10]))
                            - (z2_tmp_fec87_40[10]))),
                    ((z2_tmp_fec87_40[4])
                        + (((((x_sum_tmp_fec87_41[5]) * (y_sum_tmp_fec87_42[6]))
                            + ((x_sum_tmp_fec87_41[6]) * (y_sum_tmp_fec87_42[5])))
                            - (z0_tmp_fec87_39[11]))
                            - (z2_tmp_fec87_40[11]))),
                    ((z2_tmp_fec87_40[5])
                        + ((((x_sum_tmp_fec87_41[6]) * (y_sum_tmp_fec87_42[6]))
                            - (z0_tmp_fec87_39[12]))
                            - (z2_tmp_fec87_40[12]))),
                    z2_tmp_fec87_40[6],
                    z2_tmp_fec87_40[7],
                    z2_tmp_fec87_40[8],
                    z2_tmp_fec87_40[9],
                    z2_tmp_fec87_40[10],
                    z2_tmp_fec87_40[11],
                    z2_tmp_fec87_40[12],
                ];

                let double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44 = [
                    single_karatsuba_n_7_output_tmp_fec87_31[0],
                    single_karatsuba_n_7_output_tmp_fec87_31[1],
                    single_karatsuba_n_7_output_tmp_fec87_31[2],
                    single_karatsuba_n_7_output_tmp_fec87_31[3],
                    single_karatsuba_n_7_output_tmp_fec87_31[4],
                    single_karatsuba_n_7_output_tmp_fec87_31[5],
                    single_karatsuba_n_7_output_tmp_fec87_31[6],
                    single_karatsuba_n_7_output_tmp_fec87_31[7],
                    single_karatsuba_n_7_output_tmp_fec87_31[8],
                    single_karatsuba_n_7_output_tmp_fec87_31[9],
                    single_karatsuba_n_7_output_tmp_fec87_31[10],
                    single_karatsuba_n_7_output_tmp_fec87_31[11],
                    single_karatsuba_n_7_output_tmp_fec87_31[12],
                    single_karatsuba_n_7_output_tmp_fec87_31[13],
                    ((single_karatsuba_n_7_output_tmp_fec87_31[14])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[0])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[0]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[0]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_31[15])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[1])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[1]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[1]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_31[16])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[2])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[2]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[2]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_31[17])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[3])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[3]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[3]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_31[18])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[4])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[4]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[4]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_31[19])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[5])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[5]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[5]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_31[20])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[6])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[6]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[6]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_31[21])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[7])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[7]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[7]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_31[22])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[8])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[8]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[8]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_31[23])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[9])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[9]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[9]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_31[24])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[10])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[10]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[10]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_31[25])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[11])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[11]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[11]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_31[26])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[12])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[12]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[12]))),
                    (((single_karatsuba_n_7_output_tmp_fec87_43[13])
                        - (single_karatsuba_n_7_output_tmp_fec87_31[13]))
                        - (single_karatsuba_n_7_output_tmp_fec87_36[13])),
                    ((single_karatsuba_n_7_output_tmp_fec87_36[0])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[14])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[14]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[14]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_36[1])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[15])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[15]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[15]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_36[2])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[16])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[16]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[16]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_36[3])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[17])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[17]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[17]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_36[4])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[18])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[18]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[18]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_36[5])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[19])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[19]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[19]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_36[6])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[20])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[20]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[20]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_36[7])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[21])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[21]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[21]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_36[8])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[22])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[22]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[22]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_36[9])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[23])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[23]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[23]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_36[10])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[24])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[24]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[24]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_36[11])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[25])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[25]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[25]))),
                    ((single_karatsuba_n_7_output_tmp_fec87_36[12])
                        + (((single_karatsuba_n_7_output_tmp_fec87_43[26])
                            - (single_karatsuba_n_7_output_tmp_fec87_31[26]))
                            - (single_karatsuba_n_7_output_tmp_fec87_36[26]))),
                    single_karatsuba_n_7_output_tmp_fec87_36[13],
                    single_karatsuba_n_7_output_tmp_fec87_36[14],
                    single_karatsuba_n_7_output_tmp_fec87_36[15],
                    single_karatsuba_n_7_output_tmp_fec87_36[16],
                    single_karatsuba_n_7_output_tmp_fec87_36[17],
                    single_karatsuba_n_7_output_tmp_fec87_36[18],
                    single_karatsuba_n_7_output_tmp_fec87_36[19],
                    single_karatsuba_n_7_output_tmp_fec87_36[20],
                    single_karatsuba_n_7_output_tmp_fec87_36[21],
                    single_karatsuba_n_7_output_tmp_fec87_36[22],
                    single_karatsuba_n_7_output_tmp_fec87_36[23],
                    single_karatsuba_n_7_output_tmp_fec87_36[24],
                    single_karatsuba_n_7_output_tmp_fec87_36[25],
                    single_karatsuba_n_7_output_tmp_fec87_36[26],
                ];

                let conv_tmp_fec87_45 = [
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[0])
                        - (mul_res_limb_0_col84)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[1])
                        - (mul_res_limb_1_col85)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[2])
                        - (mul_res_limb_2_col86)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[3])
                        - (mul_res_limb_3_col87)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[4])
                        - (mul_res_limb_4_col88)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[5])
                        - (mul_res_limb_5_col89)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[6])
                        - (mul_res_limb_6_col90)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[7])
                        - (mul_res_limb_7_col91)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[8])
                        - (mul_res_limb_8_col92)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[9])
                        - (mul_res_limb_9_col93)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[10])
                        - (mul_res_limb_10_col94)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[11])
                        - (mul_res_limb_11_col95)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[12])
                        - (mul_res_limb_12_col96)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[13])
                        - (mul_res_limb_13_col97)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[14])
                        - (mul_res_limb_14_col98)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[15])
                        - (mul_res_limb_15_col99)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[16])
                        - (mul_res_limb_16_col100)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[17])
                        - (mul_res_limb_17_col101)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[18])
                        - (mul_res_limb_18_col102)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[19])
                        - (mul_res_limb_19_col103)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[20])
                        - (mul_res_limb_20_col104)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[21])
                        - (mul_res_limb_21_col105)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[22])
                        - (mul_res_limb_22_col106)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[23])
                        - (mul_res_limb_23_col107)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[24])
                        - (mul_res_limb_24_col108)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[25])
                        - (mul_res_limb_25_col109)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[26])
                        - (mul_res_limb_26_col110)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[27])
                        - (mul_res_limb_27_col111)),
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[28],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[29],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[30],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[31],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[32],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[33],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[34],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[35],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[36],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[37],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[38],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[39],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[40],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[41],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[42],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[43],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[44],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[45],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[46],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[47],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[48],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[49],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[50],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[51],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[52],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[53],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_fec87_44[54],
                ];
                let conv_mod_tmp_fec87_46 = [
                    ((((M31_32) * (conv_tmp_fec87_45[0])) - ((M31_4) * (conv_tmp_fec87_45[21])))
                        + ((M31_8) * (conv_tmp_fec87_45[49]))),
                    ((((conv_tmp_fec87_45[0]) + ((M31_32) * (conv_tmp_fec87_45[1])))
                        - ((M31_4) * (conv_tmp_fec87_45[22])))
                        + ((M31_8) * (conv_tmp_fec87_45[50]))),
                    ((((conv_tmp_fec87_45[1]) + ((M31_32) * (conv_tmp_fec87_45[2])))
                        - ((M31_4) * (conv_tmp_fec87_45[23])))
                        + ((M31_8) * (conv_tmp_fec87_45[51]))),
                    ((((conv_tmp_fec87_45[2]) + ((M31_32) * (conv_tmp_fec87_45[3])))
                        - ((M31_4) * (conv_tmp_fec87_45[24])))
                        + ((M31_8) * (conv_tmp_fec87_45[52]))),
                    ((((conv_tmp_fec87_45[3]) + ((M31_32) * (conv_tmp_fec87_45[4])))
                        - ((M31_4) * (conv_tmp_fec87_45[25])))
                        + ((M31_8) * (conv_tmp_fec87_45[53]))),
                    ((((conv_tmp_fec87_45[4]) + ((M31_32) * (conv_tmp_fec87_45[5])))
                        - ((M31_4) * (conv_tmp_fec87_45[26])))
                        + ((M31_8) * (conv_tmp_fec87_45[54]))),
                    (((conv_tmp_fec87_45[5]) + ((M31_32) * (conv_tmp_fec87_45[6])))
                        - ((M31_4) * (conv_tmp_fec87_45[27]))),
                    (((((M31_2) * (conv_tmp_fec87_45[0])) + (conv_tmp_fec87_45[6]))
                        + ((M31_32) * (conv_tmp_fec87_45[7])))
                        - ((M31_4) * (conv_tmp_fec87_45[28]))),
                    (((((M31_2) * (conv_tmp_fec87_45[1])) + (conv_tmp_fec87_45[7]))
                        + ((M31_32) * (conv_tmp_fec87_45[8])))
                        - ((M31_4) * (conv_tmp_fec87_45[29]))),
                    (((((M31_2) * (conv_tmp_fec87_45[2])) + (conv_tmp_fec87_45[8]))
                        + ((M31_32) * (conv_tmp_fec87_45[9])))
                        - ((M31_4) * (conv_tmp_fec87_45[30]))),
                    (((((M31_2) * (conv_tmp_fec87_45[3])) + (conv_tmp_fec87_45[9]))
                        + ((M31_32) * (conv_tmp_fec87_45[10])))
                        - ((M31_4) * (conv_tmp_fec87_45[31]))),
                    (((((M31_2) * (conv_tmp_fec87_45[4])) + (conv_tmp_fec87_45[10]))
                        + ((M31_32) * (conv_tmp_fec87_45[11])))
                        - ((M31_4) * (conv_tmp_fec87_45[32]))),
                    (((((M31_2) * (conv_tmp_fec87_45[5])) + (conv_tmp_fec87_45[11]))
                        + ((M31_32) * (conv_tmp_fec87_45[12])))
                        - ((M31_4) * (conv_tmp_fec87_45[33]))),
                    (((((M31_2) * (conv_tmp_fec87_45[6])) + (conv_tmp_fec87_45[12]))
                        + ((M31_32) * (conv_tmp_fec87_45[13])))
                        - ((M31_4) * (conv_tmp_fec87_45[34]))),
                    (((((M31_2) * (conv_tmp_fec87_45[7])) + (conv_tmp_fec87_45[13]))
                        + ((M31_32) * (conv_tmp_fec87_45[14])))
                        - ((M31_4) * (conv_tmp_fec87_45[35]))),
                    (((((M31_2) * (conv_tmp_fec87_45[8])) + (conv_tmp_fec87_45[14]))
                        + ((M31_32) * (conv_tmp_fec87_45[15])))
                        - ((M31_4) * (conv_tmp_fec87_45[36]))),
                    (((((M31_2) * (conv_tmp_fec87_45[9])) + (conv_tmp_fec87_45[15]))
                        + ((M31_32) * (conv_tmp_fec87_45[16])))
                        - ((M31_4) * (conv_tmp_fec87_45[37]))),
                    (((((M31_2) * (conv_tmp_fec87_45[10])) + (conv_tmp_fec87_45[16]))
                        + ((M31_32) * (conv_tmp_fec87_45[17])))
                        - ((M31_4) * (conv_tmp_fec87_45[38]))),
                    (((((M31_2) * (conv_tmp_fec87_45[11])) + (conv_tmp_fec87_45[17]))
                        + ((M31_32) * (conv_tmp_fec87_45[18])))
                        - ((M31_4) * (conv_tmp_fec87_45[39]))),
                    (((((M31_2) * (conv_tmp_fec87_45[12])) + (conv_tmp_fec87_45[18]))
                        + ((M31_32) * (conv_tmp_fec87_45[19])))
                        - ((M31_4) * (conv_tmp_fec87_45[40]))),
                    (((((M31_2) * (conv_tmp_fec87_45[13])) + (conv_tmp_fec87_45[19]))
                        + ((M31_32) * (conv_tmp_fec87_45[20])))
                        - ((M31_4) * (conv_tmp_fec87_45[41]))),
                    (((((M31_2) * (conv_tmp_fec87_45[14])) + (conv_tmp_fec87_45[20]))
                        - ((M31_4) * (conv_tmp_fec87_45[42])))
                        + ((M31_64) * (conv_tmp_fec87_45[49]))),
                    (((((M31_2) * (conv_tmp_fec87_45[15])) - ((M31_4) * (conv_tmp_fec87_45[43])))
                        + ((M31_2) * (conv_tmp_fec87_45[49])))
                        + ((M31_64) * (conv_tmp_fec87_45[50]))),
                    (((((M31_2) * (conv_tmp_fec87_45[16])) - ((M31_4) * (conv_tmp_fec87_45[44])))
                        + ((M31_2) * (conv_tmp_fec87_45[50])))
                        + ((M31_64) * (conv_tmp_fec87_45[51]))),
                    (((((M31_2) * (conv_tmp_fec87_45[17])) - ((M31_4) * (conv_tmp_fec87_45[45])))
                        + ((M31_2) * (conv_tmp_fec87_45[51])))
                        + ((M31_64) * (conv_tmp_fec87_45[52]))),
                    (((((M31_2) * (conv_tmp_fec87_45[18])) - ((M31_4) * (conv_tmp_fec87_45[46])))
                        + ((M31_2) * (conv_tmp_fec87_45[52])))
                        + ((M31_64) * (conv_tmp_fec87_45[53]))),
                    (((((M31_2) * (conv_tmp_fec87_45[19])) - ((M31_4) * (conv_tmp_fec87_45[47])))
                        + ((M31_2) * (conv_tmp_fec87_45[53])))
                        + ((M31_64) * (conv_tmp_fec87_45[54]))),
                    ((((M31_2) * (conv_tmp_fec87_45[20])) - ((M31_4) * (conv_tmp_fec87_45[48])))
                        + ((M31_2) * (conv_tmp_fec87_45[54]))),
                ];
                let k_mod_2_18_biased_tmp_fec87_47 =
                    ((((PackedUInt32::from_m31(((conv_mod_tmp_fec87_46[0]) + (M31_134217728))))
                        + (((PackedUInt32::from_m31(
                            ((conv_mod_tmp_fec87_46[1]) + (M31_134217728)),
                        )) & (UInt32_511))
                            << (UInt32_9)))
                        + (UInt32_65536))
                        & (UInt32_262143));
                let k_col112 = ((k_mod_2_18_biased_tmp_fec87_47.low().as_m31())
                    + (((k_mod_2_18_biased_tmp_fec87_47.high().as_m31()) - (M31_1)) * (M31_65536)));
                *row[112] = k_col112;
                *sub_component_inputs.range_check_19[8] = [((k_col112) + (M31_262144))];
                *lookup_data.range_check_19_8 = [((k_col112) + (M31_262144))];
                let carry_0_col113 = (((conv_mod_tmp_fec87_46[0]) - (k_col112)) * (M31_4194304));
                *row[113] = carry_0_col113;
                *sub_component_inputs.range_check_19[9] = [((carry_0_col113) + (M31_131072))];
                *lookup_data.range_check_19_9 = [((carry_0_col113) + (M31_131072))];
                let carry_1_col114 =
                    (((conv_mod_tmp_fec87_46[1]) + (carry_0_col113)) * (M31_4194304));
                *row[114] = carry_1_col114;
                *sub_component_inputs.range_check_19_b[7] = [((carry_1_col114) + (M31_131072))];
                *lookup_data.range_check_19_b_7 = [((carry_1_col114) + (M31_131072))];
                let carry_2_col115 =
                    (((conv_mod_tmp_fec87_46[2]) + (carry_1_col114)) * (M31_4194304));
                *row[115] = carry_2_col115;
                *sub_component_inputs.range_check_19_c[7] = [((carry_2_col115) + (M31_131072))];
                *lookup_data.range_check_19_c_7 = [((carry_2_col115) + (M31_131072))];
                let carry_3_col116 =
                    (((conv_mod_tmp_fec87_46[3]) + (carry_2_col115)) * (M31_4194304));
                *row[116] = carry_3_col116;
                *sub_component_inputs.range_check_19_d[6] = [((carry_3_col116) + (M31_131072))];
                *lookup_data.range_check_19_d_6 = [((carry_3_col116) + (M31_131072))];
                let carry_4_col117 =
                    (((conv_mod_tmp_fec87_46[4]) + (carry_3_col116)) * (M31_4194304));
                *row[117] = carry_4_col117;
                *sub_component_inputs.range_check_19[10] = [((carry_4_col117) + (M31_131072))];
                *lookup_data.range_check_19_10 = [((carry_4_col117) + (M31_131072))];
                let carry_5_col118 =
                    (((conv_mod_tmp_fec87_46[5]) + (carry_4_col117)) * (M31_4194304));
                *row[118] = carry_5_col118;
                *sub_component_inputs.range_check_19_b[8] = [((carry_5_col118) + (M31_131072))];
                *lookup_data.range_check_19_b_8 = [((carry_5_col118) + (M31_131072))];
                let carry_6_col119 =
                    (((conv_mod_tmp_fec87_46[6]) + (carry_5_col118)) * (M31_4194304));
                *row[119] = carry_6_col119;
                *sub_component_inputs.range_check_19_c[8] = [((carry_6_col119) + (M31_131072))];
                *lookup_data.range_check_19_c_8 = [((carry_6_col119) + (M31_131072))];
                let carry_7_col120 =
                    (((conv_mod_tmp_fec87_46[7]) + (carry_6_col119)) * (M31_4194304));
                *row[120] = carry_7_col120;
                *sub_component_inputs.range_check_19_d[7] = [((carry_7_col120) + (M31_131072))];
                *lookup_data.range_check_19_d_7 = [((carry_7_col120) + (M31_131072))];
                let carry_8_col121 =
                    (((conv_mod_tmp_fec87_46[8]) + (carry_7_col120)) * (M31_4194304));
                *row[121] = carry_8_col121;
                *sub_component_inputs.range_check_19[11] = [((carry_8_col121) + (M31_131072))];
                *lookup_data.range_check_19_11 = [((carry_8_col121) + (M31_131072))];
                let carry_9_col122 =
                    (((conv_mod_tmp_fec87_46[9]) + (carry_8_col121)) * (M31_4194304));
                *row[122] = carry_9_col122;
                *sub_component_inputs.range_check_19_b[9] = [((carry_9_col122) + (M31_131072))];
                *lookup_data.range_check_19_b_9 = [((carry_9_col122) + (M31_131072))];
                let carry_10_col123 =
                    (((conv_mod_tmp_fec87_46[10]) + (carry_9_col122)) * (M31_4194304));
                *row[123] = carry_10_col123;
                *sub_component_inputs.range_check_19_c[9] = [((carry_10_col123) + (M31_131072))];
                *lookup_data.range_check_19_c_9 = [((carry_10_col123) + (M31_131072))];
                let carry_11_col124 =
                    (((conv_mod_tmp_fec87_46[11]) + (carry_10_col123)) * (M31_4194304));
                *row[124] = carry_11_col124;
                *sub_component_inputs.range_check_19_d[8] = [((carry_11_col124) + (M31_131072))];
                *lookup_data.range_check_19_d_8 = [((carry_11_col124) + (M31_131072))];
                let carry_12_col125 =
                    (((conv_mod_tmp_fec87_46[12]) + (carry_11_col124)) * (M31_4194304));
                *row[125] = carry_12_col125;
                *sub_component_inputs.range_check_19[12] = [((carry_12_col125) + (M31_131072))];
                *lookup_data.range_check_19_12 = [((carry_12_col125) + (M31_131072))];
                let carry_13_col126 =
                    (((conv_mod_tmp_fec87_46[13]) + (carry_12_col125)) * (M31_4194304));
                *row[126] = carry_13_col126;
                *sub_component_inputs.range_check_19_b[10] = [((carry_13_col126) + (M31_131072))];
                *lookup_data.range_check_19_b_10 = [((carry_13_col126) + (M31_131072))];
                let carry_14_col127 =
                    (((conv_mod_tmp_fec87_46[14]) + (carry_13_col126)) * (M31_4194304));
                *row[127] = carry_14_col127;
                *sub_component_inputs.range_check_19_c[10] = [((carry_14_col127) + (M31_131072))];
                *lookup_data.range_check_19_c_10 = [((carry_14_col127) + (M31_131072))];
                let carry_15_col128 =
                    (((conv_mod_tmp_fec87_46[15]) + (carry_14_col127)) * (M31_4194304));
                *row[128] = carry_15_col128;
                *sub_component_inputs.range_check_19_d[9] = [((carry_15_col128) + (M31_131072))];
                *lookup_data.range_check_19_d_9 = [((carry_15_col128) + (M31_131072))];
                let carry_16_col129 =
                    (((conv_mod_tmp_fec87_46[16]) + (carry_15_col128)) * (M31_4194304));
                *row[129] = carry_16_col129;
                *sub_component_inputs.range_check_19[13] = [((carry_16_col129) + (M31_131072))];
                *lookup_data.range_check_19_13 = [((carry_16_col129) + (M31_131072))];
                let carry_17_col130 =
                    (((conv_mod_tmp_fec87_46[17]) + (carry_16_col129)) * (M31_4194304));
                *row[130] = carry_17_col130;
                *sub_component_inputs.range_check_19_b[11] = [((carry_17_col130) + (M31_131072))];
                *lookup_data.range_check_19_b_11 = [((carry_17_col130) + (M31_131072))];
                let carry_18_col131 =
                    (((conv_mod_tmp_fec87_46[18]) + (carry_17_col130)) * (M31_4194304));
                *row[131] = carry_18_col131;
                *sub_component_inputs.range_check_19_c[11] = [((carry_18_col131) + (M31_131072))];
                *lookup_data.range_check_19_c_11 = [((carry_18_col131) + (M31_131072))];
                let carry_19_col132 =
                    (((conv_mod_tmp_fec87_46[19]) + (carry_18_col131)) * (M31_4194304));
                *row[132] = carry_19_col132;
                *sub_component_inputs.range_check_19_d[10] = [((carry_19_col132) + (M31_131072))];
                *lookup_data.range_check_19_d_10 = [((carry_19_col132) + (M31_131072))];
                let carry_20_col133 =
                    (((conv_mod_tmp_fec87_46[20]) + (carry_19_col132)) * (M31_4194304));
                *row[133] = carry_20_col133;
                *sub_component_inputs.range_check_19[14] = [((carry_20_col133) + (M31_131072))];
                *lookup_data.range_check_19_14 = [((carry_20_col133) + (M31_131072))];
                let carry_21_col134 = ((((conv_mod_tmp_fec87_46[21]) - ((M31_136) * (k_col112)))
                    + (carry_20_col133))
                    * (M31_4194304));
                *row[134] = carry_21_col134;
                *sub_component_inputs.range_check_19_b[12] = [((carry_21_col134) + (M31_131072))];
                *lookup_data.range_check_19_b_12 = [((carry_21_col134) + (M31_131072))];
                let carry_22_col135 =
                    (((conv_mod_tmp_fec87_46[22]) + (carry_21_col134)) * (M31_4194304));
                *row[135] = carry_22_col135;
                *sub_component_inputs.range_check_19_c[12] = [((carry_22_col135) + (M31_131072))];
                *lookup_data.range_check_19_c_12 = [((carry_22_col135) + (M31_131072))];
                let carry_23_col136 =
                    (((conv_mod_tmp_fec87_46[23]) + (carry_22_col135)) * (M31_4194304));
                *row[136] = carry_23_col136;
                *sub_component_inputs.range_check_19_d[11] = [((carry_23_col136) + (M31_131072))];
                *lookup_data.range_check_19_d_11 = [((carry_23_col136) + (M31_131072))];
                let carry_24_col137 =
                    (((conv_mod_tmp_fec87_46[24]) + (carry_23_col136)) * (M31_4194304));
                *row[137] = carry_24_col137;
                *sub_component_inputs.range_check_19[15] = [((carry_24_col137) + (M31_131072))];
                *lookup_data.range_check_19_15 = [((carry_24_col137) + (M31_131072))];
                let carry_25_col138 =
                    (((conv_mod_tmp_fec87_46[25]) + (carry_24_col137)) * (M31_4194304));
                *row[138] = carry_25_col138;
                *sub_component_inputs.range_check_19_b[13] = [((carry_25_col138) + (M31_131072))];
                *lookup_data.range_check_19_b_13 = [((carry_25_col138) + (M31_131072))];
                let carry_26_col139 =
                    (((conv_mod_tmp_fec87_46[26]) + (carry_25_col138)) * (M31_4194304));
                *row[139] = carry_26_col139;
                *sub_component_inputs.range_check_19_c[13] = [((carry_26_col139) + (M31_131072))];
                *lookup_data.range_check_19_c_13 = [((carry_26_col139) + (M31_131072))];

                let mul_252_output_tmp_fec87_48 = mul_res_tmp_fec87_26;

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
                    (((mul_res_limb_0_col84) + ((mul_res_limb_1_col85) * (M31_512)))
                        + ((mul_res_limb_2_col86) * (M31_262144))),
                    (((mul_res_limb_3_col87) + ((mul_res_limb_4_col88) * (M31_512)))
                        + ((mul_res_limb_5_col89) * (M31_262144))),
                    (((mul_res_limb_6_col90) + ((mul_res_limb_7_col91) * (M31_512)))
                        + ((mul_res_limb_8_col92) * (M31_262144))),
                    (((mul_res_limb_9_col93) + ((mul_res_limb_10_col94) * (M31_512)))
                        + ((mul_res_limb_11_col95) * (M31_262144))),
                    (((mul_res_limb_12_col96) + ((mul_res_limb_13_col97) * (M31_512)))
                        + ((mul_res_limb_14_col98) * (M31_262144))),
                    (((mul_res_limb_15_col99) + ((mul_res_limb_16_col100) * (M31_512)))
                        + ((mul_res_limb_17_col101) * (M31_262144))),
                    (((mul_res_limb_18_col102) + ((mul_res_limb_19_col103) * (M31_512)))
                        + ((mul_res_limb_20_col104) * (M31_262144))),
                    (((mul_res_limb_21_col105) + ((mul_res_limb_22_col106) * (M31_512)))
                        + ((mul_res_limb_23_col107) * (M31_262144))),
                    (((mul_res_limb_24_col108) + ((mul_res_limb_25_col109) * (M31_512)))
                        + ((mul_res_limb_26_col110) * (M31_262144))),
                    mul_res_limb_27_col111,
                ];
                *row[140] = enabler_col.packed_at(row_index);
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    cube_252_0: Vec<[PackedM31; 20]>,
    range_check_19_0: Vec<[PackedM31; 1]>,
    range_check_19_1: Vec<[PackedM31; 1]>,
    range_check_19_2: Vec<[PackedM31; 1]>,
    range_check_19_3: Vec<[PackedM31; 1]>,
    range_check_19_4: Vec<[PackedM31; 1]>,
    range_check_19_5: Vec<[PackedM31; 1]>,
    range_check_19_6: Vec<[PackedM31; 1]>,
    range_check_19_7: Vec<[PackedM31; 1]>,
    range_check_19_8: Vec<[PackedM31; 1]>,
    range_check_19_9: Vec<[PackedM31; 1]>,
    range_check_19_10: Vec<[PackedM31; 1]>,
    range_check_19_11: Vec<[PackedM31; 1]>,
    range_check_19_12: Vec<[PackedM31; 1]>,
    range_check_19_13: Vec<[PackedM31; 1]>,
    range_check_19_14: Vec<[PackedM31; 1]>,
    range_check_19_15: Vec<[PackedM31; 1]>,
    range_check_19_b_0: Vec<[PackedM31; 1]>,
    range_check_19_b_1: Vec<[PackedM31; 1]>,
    range_check_19_b_2: Vec<[PackedM31; 1]>,
    range_check_19_b_3: Vec<[PackedM31; 1]>,
    range_check_19_b_4: Vec<[PackedM31; 1]>,
    range_check_19_b_5: Vec<[PackedM31; 1]>,
    range_check_19_b_6: Vec<[PackedM31; 1]>,
    range_check_19_b_7: Vec<[PackedM31; 1]>,
    range_check_19_b_8: Vec<[PackedM31; 1]>,
    range_check_19_b_9: Vec<[PackedM31; 1]>,
    range_check_19_b_10: Vec<[PackedM31; 1]>,
    range_check_19_b_11: Vec<[PackedM31; 1]>,
    range_check_19_b_12: Vec<[PackedM31; 1]>,
    range_check_19_b_13: Vec<[PackedM31; 1]>,
    range_check_19_c_0: Vec<[PackedM31; 1]>,
    range_check_19_c_1: Vec<[PackedM31; 1]>,
    range_check_19_c_2: Vec<[PackedM31; 1]>,
    range_check_19_c_3: Vec<[PackedM31; 1]>,
    range_check_19_c_4: Vec<[PackedM31; 1]>,
    range_check_19_c_5: Vec<[PackedM31; 1]>,
    range_check_19_c_6: Vec<[PackedM31; 1]>,
    range_check_19_c_7: Vec<[PackedM31; 1]>,
    range_check_19_c_8: Vec<[PackedM31; 1]>,
    range_check_19_c_9: Vec<[PackedM31; 1]>,
    range_check_19_c_10: Vec<[PackedM31; 1]>,
    range_check_19_c_11: Vec<[PackedM31; 1]>,
    range_check_19_c_12: Vec<[PackedM31; 1]>,
    range_check_19_c_13: Vec<[PackedM31; 1]>,
    range_check_19_d_0: Vec<[PackedM31; 1]>,
    range_check_19_d_1: Vec<[PackedM31; 1]>,
    range_check_19_d_2: Vec<[PackedM31; 1]>,
    range_check_19_d_3: Vec<[PackedM31; 1]>,
    range_check_19_d_4: Vec<[PackedM31; 1]>,
    range_check_19_d_5: Vec<[PackedM31; 1]>,
    range_check_19_d_6: Vec<[PackedM31; 1]>,
    range_check_19_d_7: Vec<[PackedM31; 1]>,
    range_check_19_d_8: Vec<[PackedM31; 1]>,
    range_check_19_d_9: Vec<[PackedM31; 1]>,
    range_check_19_d_10: Vec<[PackedM31; 1]>,
    range_check_19_d_11: Vec<[PackedM31; 1]>,
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
    range_check_9_9_b_0: Vec<[PackedM31; 2]>,
    range_check_9_9_b_1: Vec<[PackedM31; 2]>,
    range_check_9_9_b_2: Vec<[PackedM31; 2]>,
    range_check_9_9_b_3: Vec<[PackedM31; 2]>,
    range_check_9_9_b_4: Vec<[PackedM31; 2]>,
    range_check_9_9_b_5: Vec<[PackedM31; 2]>,
    range_check_9_9_b_6: Vec<[PackedM31; 2]>,
    range_check_9_9_b_7: Vec<[PackedM31; 2]>,
    range_check_9_9_b_8: Vec<[PackedM31; 2]>,
    range_check_9_9_b_9: Vec<[PackedM31; 2]>,
    range_check_9_9_b_10: Vec<[PackedM31; 2]>,
    range_check_9_9_b_11: Vec<[PackedM31; 2]>,
    range_check_9_9_c_0: Vec<[PackedM31; 2]>,
    range_check_9_9_c_1: Vec<[PackedM31; 2]>,
    range_check_9_9_c_2: Vec<[PackedM31; 2]>,
    range_check_9_9_c_3: Vec<[PackedM31; 2]>,
    range_check_9_9_c_4: Vec<[PackedM31; 2]>,
    range_check_9_9_c_5: Vec<[PackedM31; 2]>,
    range_check_9_9_c_6: Vec<[PackedM31; 2]>,
    range_check_9_9_c_7: Vec<[PackedM31; 2]>,
    range_check_9_9_c_8: Vec<[PackedM31; 2]>,
    range_check_9_9_d_0: Vec<[PackedM31; 2]>,
    range_check_9_9_d_1: Vec<[PackedM31; 2]>,
    range_check_9_9_d_2: Vec<[PackedM31; 2]>,
    range_check_9_9_d_3: Vec<[PackedM31; 2]>,
    range_check_9_9_d_4: Vec<[PackedM31; 2]>,
    range_check_9_9_d_5: Vec<[PackedM31; 2]>,
    range_check_9_9_d_6: Vec<[PackedM31; 2]>,
    range_check_9_9_d_7: Vec<[PackedM31; 2]>,
    range_check_9_9_d_8: Vec<[PackedM31; 2]>,
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
        cube_252: &relations::Cube252,
        range_check_19: &relations::RangeCheck_19,
        range_check_19_b: &relations::RangeCheck_19_B,
        range_check_19_c: &relations::RangeCheck_19_C,
        range_check_19_d: &relations::RangeCheck_19_D,
        range_check_9_9: &relations::RangeCheck_9_9,
        range_check_9_9_b: &relations::RangeCheck_9_9_B,
        range_check_9_9_c: &relations::RangeCheck_9_9_C,
        range_check_9_9_d: &relations::RangeCheck_9_9_D,
    ) -> InteractionClaim {
        let enabler_col = Enabler::new(self.n_rows);
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_0,
            &self.lookup_data.range_check_9_9_b_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_b.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_c_0,
            &self.lookup_data.range_check_9_9_d_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_c.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_d.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_9_9_1,
                &self.lookup_data.range_check_9_9_b_1,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_9_9.combine(values0);
                    let denom1: PackedQM31 = range_check_9_9_b.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_c_1,
            &self.lookup_data.range_check_9_9_d_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_c.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_d.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_9_9_2,
                &self.lookup_data.range_check_9_9_b_2,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_9_9.combine(values0);
                    let denom1: PackedQM31 = range_check_9_9_b.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_c_2,
            &self.lookup_data.range_check_9_9_d_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_c.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_d.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_9_9_3,
                &self.lookup_data.range_check_9_9_b_3,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_9_9.combine(values0);
                    let denom1: PackedQM31 = range_check_9_9_b.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_9_9_4,
                &self.lookup_data.range_check_9_9_b_4,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_9_9.combine(values0);
                    let denom1: PackedQM31 = range_check_9_9_b.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_9_9_c_3,
                &self.lookup_data.range_check_9_9_d_3,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_9_9_c.combine(values0);
                    let denom1: PackedQM31 = range_check_9_9_d.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_9_9_5,
                &self.lookup_data.range_check_9_9_b_5,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_9_9.combine(values0);
                    let denom1: PackedQM31 = range_check_9_9_b.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_c_4,
            &self.lookup_data.range_check_9_9_d_4,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_c.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_d.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_9_9_6,
                &self.lookup_data.range_check_9_9_b_6,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_9_9.combine(values0);
                    let denom1: PackedQM31 = range_check_9_9_b.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_c_5,
            &self.lookup_data.range_check_9_9_d_5,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_c.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_d.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_9_9_7,
                &self.lookup_data.range_check_9_9_b_7,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_9_9.combine(values0);
                    let denom1: PackedQM31 = range_check_9_9_b.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_19_0,
                &self.lookup_data.range_check_19_1,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_19.combine(values0);
                    let denom1: PackedQM31 = range_check_19.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_19_b_0,
                &self.lookup_data.range_check_19_c_0,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_19_b.combine(values0);
                    let denom1: PackedQM31 = range_check_19_c.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_19_d_0,
                &self.lookup_data.range_check_19_2,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_19_d.combine(values0);
                    let denom1: PackedQM31 = range_check_19.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_19_b_1,
                &self.lookup_data.range_check_19_c_1,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_19_b.combine(values0);
                    let denom1: PackedQM31 = range_check_19_c.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_19_d_1,
                &self.lookup_data.range_check_19_3,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_19_d.combine(values0);
                    let denom1: PackedQM31 = range_check_19.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_19_b_2,
                &self.lookup_data.range_check_19_c_2,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_19_b.combine(values0);
                    let denom1: PackedQM31 = range_check_19_c.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_19_d_2,
                &self.lookup_data.range_check_19_4,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_19_d.combine(values0);
                    let denom1: PackedQM31 = range_check_19.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_19_b_3,
                &self.lookup_data.range_check_19_c_3,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_19_b.combine(values0);
                    let denom1: PackedQM31 = range_check_19_c.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_19_d_3,
                &self.lookup_data.range_check_19_5,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_19_d.combine(values0);
                    let denom1: PackedQM31 = range_check_19.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_b_4,
            &self.lookup_data.range_check_19_c_4,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19_b.combine(values0);
                let denom1: PackedQM31 = range_check_19_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_19_d_4,
                &self.lookup_data.range_check_19_6,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_19_d.combine(values0);
                    let denom1: PackedQM31 = range_check_19.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_b_5,
            &self.lookup_data.range_check_19_c_5,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19_b.combine(values0);
                let denom1: PackedQM31 = range_check_19_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_19_d_5,
                &self.lookup_data.range_check_19_7,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_19_d.combine(values0);
                    let denom1: PackedQM31 = range_check_19.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_b_6,
            &self.lookup_data.range_check_19_c_6,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19_b.combine(values0);
                let denom1: PackedQM31 = range_check_19_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_9_9_8,
                &self.lookup_data.range_check_9_9_b_8,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_9_9.combine(values0);
                    let denom1: PackedQM31 = range_check_9_9_b.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_9_9_c_6,
                &self.lookup_data.range_check_9_9_d_6,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_9_9_c.combine(values0);
                    let denom1: PackedQM31 = range_check_9_9_d.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_9_9_9,
                &self.lookup_data.range_check_9_9_b_9,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_9_9.combine(values0);
                    let denom1: PackedQM31 = range_check_9_9_b.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_9_9_c_7,
                &self.lookup_data.range_check_9_9_d_7,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_9_9_c.combine(values0);
                    let denom1: PackedQM31 = range_check_9_9_d.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_9_9_10,
                &self.lookup_data.range_check_9_9_b_10,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_9_9.combine(values0);
                    let denom1: PackedQM31 = range_check_9_9_b.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_9_9_c_8,
                &self.lookup_data.range_check_9_9_d_8,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_9_9_c.combine(values0);
                    let denom1: PackedQM31 = range_check_9_9_d.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_9_9_11,
                &self.lookup_data.range_check_9_9_b_11,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_9_9.combine(values0);
                    let denom1: PackedQM31 = range_check_9_9_b.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_19_8,
                &self.lookup_data.range_check_19_9,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_19.combine(values0);
                    let denom1: PackedQM31 = range_check_19.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_b_7,
            &self.lookup_data.range_check_19_c_7,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19_b.combine(values0);
                let denom1: PackedQM31 = range_check_19_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_19_d_6,
                &self.lookup_data.range_check_19_10,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_19_d.combine(values0);
                    let denom1: PackedQM31 = range_check_19.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_19_b_8,
                &self.lookup_data.range_check_19_c_8,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_19_b.combine(values0);
                    let denom1: PackedQM31 = range_check_19_c.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_19_d_7,
                &self.lookup_data.range_check_19_11,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_19_d.combine(values0);
                    let denom1: PackedQM31 = range_check_19.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_19_b_9,
                &self.lookup_data.range_check_19_c_9,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_19_b.combine(values0);
                    let denom1: PackedQM31 = range_check_19_c.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_19_d_8,
                &self.lookup_data.range_check_19_12,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_19_d.combine(values0);
                    let denom1: PackedQM31 = range_check_19.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_19_b_10,
                &self.lookup_data.range_check_19_c_10,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_19_b.combine(values0);
                    let denom1: PackedQM31 = range_check_19_c.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_19_d_9,
                &self.lookup_data.range_check_19_13,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_19_d.combine(values0);
                    let denom1: PackedQM31 = range_check_19.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_19_b_11,
                &self.lookup_data.range_check_19_c_11,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_19_b.combine(values0);
                    let denom1: PackedQM31 = range_check_19_c.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_19_d_10,
                &self.lookup_data.range_check_19_14,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_19_d.combine(values0);
                    let denom1: PackedQM31 = range_check_19.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_19_b_12,
                &self.lookup_data.range_check_19_c_12,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_19_b.combine(values0);
                    let denom1: PackedQM31 = range_check_19_c.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_19_d_11,
                &self.lookup_data.range_check_19_15,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_19_d.combine(values0);
                    let denom1: PackedQM31 = range_check_19.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        logup_gen.col_from_par_iter(
            (
                &self.lookup_data.range_check_19_b_13,
                &self.lookup_data.range_check_19_c_13,
            )
                .into_par_iter()
                .map(|(values0, values1)| {
                    let denom0: PackedQM31 = range_check_19_b.combine(values0);
                    let denom1: PackedQM31 = range_check_19_c.combine(values1);
                    (denom0 + denom1, denom0 * denom1)
                }),
        );

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(), &self.lookup_data.cube_252_0)
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (writer, values))| {
                let denom = cube_252.combine(values);
                writer.write_frac(-PackedQM31::one() * enabler_col.packed_at(i), denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
