#![allow(unused_parens)]
use cairo_air::components::cube_252::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{range_check_9_9, range_check_20};
use crate::witness::prelude::*;

pub type InputType = Felt252Width27;
pub type PackedInputType = PackedFelt252Width27;

#[derive(Default)]
pub struct ClaimGenerator {
    pub packed_inputs: Mutex<Vec<PackedInputType>>,
    pub remainder_inputs: Mutex<Vec<InputType>>,
}

impl ClaimGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write_trace(
        self,
        range_check_9_9_state: &range_check_9_9::ClaimGenerator,
        range_check_20_state: &range_check_20::ClaimGenerator,
    ) -> (ComponentTrace<N_TRACE_COLUMNS>, Claim, InteractionClaimGenerator) {
        let mut packed_inputs = self.packed_inputs.into_inner().unwrap();
        let remainder_inputs = self.remainder_inputs.into_inner().unwrap();
        let n_active_rows = packed_inputs.len() * N_LANES + remainder_inputs.len();
        add_remainder(&mut packed_inputs, &remainder_inputs);
        assert!(!packed_inputs.is_empty());
        let n_vec_rows = packed_inputs.len();
        let packed_size = n_vec_rows.next_power_of_two();
        let log_size = packed_size.ilog2() + LOG_N_LANES;
        packed_inputs.resize(packed_size, *packed_inputs.first().unwrap());

        // Decreasing this value may cause a stack-overflow during witness generation.
        // NOTE: This is not autogened, when updating the code, re-add this.
        // TODO(Ohad): remove.
        const RAYON_THREAD_STACK_SIZE: usize = 1024 * 1024 * 8;
        let pool =
            rayon::ThreadPoolBuilder::new().stack_size(RAYON_THREAD_STACK_SIZE).build().unwrap();
        let (trace, lookup_data, sub_component_inputs) = pool.install(|| {
            write_trace_simd(
                packed_inputs,
                n_active_rows,
                range_check_9_9_state,
                range_check_20_state,
            )
        });
        for inputs in sub_component_inputs.range_check_9_9 {
            add_inputs(range_check_9_9_state, &inputs, n_active_rows, 0);
        }
        for inputs in sub_component_inputs.range_check_9_9_b {
            add_inputs(range_check_9_9_state, &inputs, n_active_rows, 1);
        }
        for inputs in sub_component_inputs.range_check_9_9_c {
            add_inputs(range_check_9_9_state, &inputs, n_active_rows, 2);
        }
        for inputs in sub_component_inputs.range_check_9_9_d {
            add_inputs(range_check_9_9_state, &inputs, n_active_rows, 3);
        }
        for inputs in sub_component_inputs.range_check_9_9_e {
            add_inputs(range_check_9_9_state, &inputs, n_active_rows, 4);
        }
        for inputs in sub_component_inputs.range_check_9_9_f {
            add_inputs(range_check_9_9_state, &inputs, n_active_rows, 5);
        }
        for inputs in sub_component_inputs.range_check_9_9_g {
            add_inputs(range_check_9_9_state, &inputs, n_active_rows, 6);
        }
        for inputs in sub_component_inputs.range_check_9_9_h {
            add_inputs(range_check_9_9_state, &inputs, n_active_rows, 7);
        }
        for inputs in sub_component_inputs.range_check_20 {
            add_inputs(range_check_20_state, &inputs, n_active_rows, 0);
        }
        for inputs in sub_component_inputs.range_check_20_b {
            add_inputs(range_check_20_state, &inputs, n_active_rows, 1);
        }
        for inputs in sub_component_inputs.range_check_20_c {
            add_inputs(range_check_20_state, &inputs, n_active_rows, 2);
        }
        for inputs in sub_component_inputs.range_check_20_d {
            add_inputs(range_check_20_state, &inputs, n_active_rows, 3);
        }
        for inputs in sub_component_inputs.range_check_20_e {
            add_inputs(range_check_20_state, &inputs, n_active_rows, 4);
        }
        for inputs in sub_component_inputs.range_check_20_f {
            add_inputs(range_check_20_state, &inputs, n_active_rows, 5);
        }
        for inputs in sub_component_inputs.range_check_20_g {
            add_inputs(range_check_20_state, &inputs, n_active_rows, 6);
        }
        for inputs in sub_component_inputs.range_check_20_h {
            add_inputs(range_check_20_state, &inputs, n_active_rows, 7);
        }

        (trace, Claim { log_size }, InteractionClaimGenerator { log_size, lookup_data })
    }
}

impl AddInputs for ClaimGenerator {
    type PackedInputType = PackedInputType;
    type InputType = InputType;

    fn add_packed_inputs(&self, inputs: &[PackedInputType], _relation_index: usize) {
        self.packed_inputs.lock().unwrap().extend(inputs);
    }
    fn add_input(&self, input: &InputType, _relation_index: usize) {
        self.remainder_inputs.lock().unwrap().push(*input);
    }
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct SubComponentInputs {
    range_check_9_9: [Vec<range_check_9_9::PackedInputType>; 6],
    range_check_9_9_b: [Vec<range_check_9_9::PackedInputType>; 6],
    range_check_9_9_c: [Vec<range_check_9_9::PackedInputType>; 6],
    range_check_9_9_d: [Vec<range_check_9_9::PackedInputType>; 6],
    range_check_9_9_e: [Vec<range_check_9_9::PackedInputType>; 6],
    range_check_9_9_f: [Vec<range_check_9_9::PackedInputType>; 6],
    range_check_9_9_g: [Vec<range_check_9_9::PackedInputType>; 3],
    range_check_9_9_h: [Vec<range_check_9_9::PackedInputType>; 3],
    range_check_20: [Vec<range_check_20::PackedInputType>; 8],
    range_check_20_b: [Vec<range_check_20::PackedInputType>; 8],
    range_check_20_c: [Vec<range_check_20::PackedInputType>; 8],
    range_check_20_d: [Vec<range_check_20::PackedInputType>; 8],
    range_check_20_e: [Vec<range_check_20::PackedInputType>; 6],
    range_check_20_f: [Vec<range_check_20::PackedInputType>; 6],
    range_check_20_g: [Vec<range_check_20::PackedInputType>; 6],
    range_check_20_h: [Vec<range_check_20::PackedInputType>; 6],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    n_rows: usize,
    range_check_9_9_state: &range_check_9_9::ClaimGenerator,
    range_check_20_state: &range_check_20::ClaimGenerator,
) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData, SubComponentInputs) {
    let log_n_packed_rows = inputs.len().ilog2();
    let log_size = log_n_packed_rows + LOG_N_LANES;
    let (mut trace, mut lookup_data, mut sub_component_inputs) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
            SubComponentInputs::uninitialized(log_n_packed_rows),
        )
    };

    let M31_134217728 = PackedM31::broadcast(M31::from(134217728));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_1410849886 = PackedM31::broadcast(M31::from(1410849886));
    let M31_1813904000 = PackedM31::broadcast(M31::from(1813904000));
    let M31_1830681619 = PackedM31::broadcast(M31::from(1830681619));
    let M31_1847459238 = PackedM31::broadcast(M31::from(1847459238));
    let M31_1864236857 = PackedM31::broadcast(M31::from(1864236857));
    let M31_1881014476 = PackedM31::broadcast(M31::from(1881014476));
    let M31_1897792095 = PackedM31::broadcast(M31::from(1897792095));
    let M31_1987997202 = PackedM31::broadcast(M31::from(1987997202));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_2065568285 = PackedM31::broadcast(M31::from(2065568285));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_4 = PackedM31::broadcast(M31::from(4));
    let M31_4194304 = PackedM31::broadcast(M31::from(4194304));
    let M31_447122465 = PackedM31::broadcast(M31::from(447122465));
    let M31_463900084 = PackedM31::broadcast(M31::from(463900084));
    let M31_480677703 = PackedM31::broadcast(M31::from(480677703));
    let M31_497455322 = PackedM31::broadcast(M31::from(497455322));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let M31_514232941 = PackedM31::broadcast(M31::from(514232941));
    let M31_517791011 = PackedM31::broadcast(M31::from(517791011));
    let M31_524288 = PackedM31::broadcast(M31::from(524288));
    let M31_531010560 = PackedM31::broadcast(M31::from(531010560));
    let M31_64 = PackedM31::broadcast(M31::from(64));
    let M31_65536 = PackedM31::broadcast(M31::from(65536));
    let M31_682009131 = PackedM31::broadcast(M31::from(682009131));
    let M31_8 = PackedM31::broadcast(M31::from(8));
    let M31_8192 = PackedM31::broadcast(M31::from(8192));
    let UInt32_131072 = PackedUInt32::broadcast(UInt32::from(131072));
    let UInt32_262143 = PackedUInt32::broadcast(UInt32::from(262143));
    let UInt32_511 = PackedUInt32::broadcast(UInt32::from(511));
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
        .for_each(|(row_index, (row, lookup_data, sub_component_inputs, cube_252_input))| {
            let enabler_col0 = enabler_col.packed_at(row_index);
            *row[0] = enabler_col0;
            let input_limb_0_col1 = cube_252_input.get_m31(0);
            *row[1] = input_limb_0_col1;
            let input_limb_1_col2 = cube_252_input.get_m31(1);
            *row[2] = input_limb_1_col2;
            let input_limb_2_col3 = cube_252_input.get_m31(2);
            *row[3] = input_limb_2_col3;
            let input_limb_3_col4 = cube_252_input.get_m31(3);
            *row[4] = input_limb_3_col4;
            let input_limb_4_col5 = cube_252_input.get_m31(4);
            *row[5] = input_limb_4_col5;
            let input_limb_5_col6 = cube_252_input.get_m31(5);
            *row[6] = input_limb_5_col6;
            let input_limb_6_col7 = cube_252_input.get_m31(6);
            *row[7] = input_limb_6_col7;
            let input_limb_7_col8 = cube_252_input.get_m31(7);
            *row[8] = input_limb_7_col8;
            let input_limb_8_col9 = cube_252_input.get_m31(8);
            *row[9] = input_limb_8_col9;
            let input_limb_9_col10 = cube_252_input.get_m31(9);
            *row[10] = input_limb_9_col10;

            // Felt 252 Unpack From 27 Range Check Output.

            let input_as_felt252_tmp_715f4_0 =
                PackedFelt252::from_packed_felt252width27(cube_252_input);
            let unpacked_limb_0_col11 = input_as_felt252_tmp_715f4_0.get_m31(0);
            *row[11] = unpacked_limb_0_col11;
            let unpacked_limb_1_col12 = input_as_felt252_tmp_715f4_0.get_m31(1);
            *row[12] = unpacked_limb_1_col12;
            let unpacked_limb_3_col13 = input_as_felt252_tmp_715f4_0.get_m31(3);
            *row[13] = unpacked_limb_3_col13;
            let unpacked_limb_4_col14 = input_as_felt252_tmp_715f4_0.get_m31(4);
            *row[14] = unpacked_limb_4_col14;
            let unpacked_limb_6_col15 = input_as_felt252_tmp_715f4_0.get_m31(6);
            *row[15] = unpacked_limb_6_col15;
            let unpacked_limb_7_col16 = input_as_felt252_tmp_715f4_0.get_m31(7);
            *row[16] = unpacked_limb_7_col16;
            let unpacked_limb_9_col17 = input_as_felt252_tmp_715f4_0.get_m31(9);
            *row[17] = unpacked_limb_9_col17;
            let unpacked_limb_10_col18 = input_as_felt252_tmp_715f4_0.get_m31(10);
            *row[18] = unpacked_limb_10_col18;
            let unpacked_limb_12_col19 = input_as_felt252_tmp_715f4_0.get_m31(12);
            *row[19] = unpacked_limb_12_col19;
            let unpacked_limb_13_col20 = input_as_felt252_tmp_715f4_0.get_m31(13);
            *row[20] = unpacked_limb_13_col20;
            let unpacked_limb_15_col21 = input_as_felt252_tmp_715f4_0.get_m31(15);
            *row[21] = unpacked_limb_15_col21;
            let unpacked_limb_16_col22 = input_as_felt252_tmp_715f4_0.get_m31(16);
            *row[22] = unpacked_limb_16_col22;
            let unpacked_limb_18_col23 = input_as_felt252_tmp_715f4_0.get_m31(18);
            *row[23] = unpacked_limb_18_col23;
            let unpacked_limb_19_col24 = input_as_felt252_tmp_715f4_0.get_m31(19);
            *row[24] = unpacked_limb_19_col24;
            let unpacked_limb_21_col25 = input_as_felt252_tmp_715f4_0.get_m31(21);
            *row[25] = unpacked_limb_21_col25;
            let unpacked_limb_22_col26 = input_as_felt252_tmp_715f4_0.get_m31(22);
            *row[26] = unpacked_limb_22_col26;
            let unpacked_limb_24_col27 = input_as_felt252_tmp_715f4_0.get_m31(24);
            *row[27] = unpacked_limb_24_col27;
            let unpacked_limb_25_col28 = input_as_felt252_tmp_715f4_0.get_m31(25);
            *row[28] = unpacked_limb_25_col28;
            let unpacked_tmp_715f4_1 = PackedFelt252::from_limbs([
                unpacked_limb_0_col11,
                unpacked_limb_1_col12,
                ((((input_limb_0_col1) - (unpacked_limb_0_col11))
                    - ((unpacked_limb_1_col12) * (M31_512)))
                    * (M31_8192)),
                unpacked_limb_3_col13,
                unpacked_limb_4_col14,
                ((((input_limb_1_col2) - (unpacked_limb_3_col13))
                    - ((unpacked_limb_4_col14) * (M31_512)))
                    * (M31_8192)),
                unpacked_limb_6_col15,
                unpacked_limb_7_col16,
                ((((input_limb_2_col3) - (unpacked_limb_6_col15))
                    - ((unpacked_limb_7_col16) * (M31_512)))
                    * (M31_8192)),
                unpacked_limb_9_col17,
                unpacked_limb_10_col18,
                ((((input_limb_3_col4) - (unpacked_limb_9_col17))
                    - ((unpacked_limb_10_col18) * (M31_512)))
                    * (M31_8192)),
                unpacked_limb_12_col19,
                unpacked_limb_13_col20,
                ((((input_limb_4_col5) - (unpacked_limb_12_col19))
                    - ((unpacked_limb_13_col20) * (M31_512)))
                    * (M31_8192)),
                unpacked_limb_15_col21,
                unpacked_limb_16_col22,
                ((((input_limb_5_col6) - (unpacked_limb_15_col21))
                    - ((unpacked_limb_16_col22) * (M31_512)))
                    * (M31_8192)),
                unpacked_limb_18_col23,
                unpacked_limb_19_col24,
                ((((input_limb_6_col7) - (unpacked_limb_18_col23))
                    - ((unpacked_limb_19_col24) * (M31_512)))
                    * (M31_8192)),
                unpacked_limb_21_col25,
                unpacked_limb_22_col26,
                ((((input_limb_7_col8) - (unpacked_limb_21_col25))
                    - ((unpacked_limb_22_col26) * (M31_512)))
                    * (M31_8192)),
                unpacked_limb_24_col27,
                unpacked_limb_25_col28,
                ((((input_limb_8_col9) - (unpacked_limb_24_col27))
                    - ((unpacked_limb_25_col28) * (M31_512)))
                    * (M31_8192)),
                input_limb_9_col10,
            ]);

            // Range Check Mem Value N 28.

            *sub_component_inputs.range_check_9_9[0] =
                [unpacked_limb_0_col11, unpacked_limb_1_col12];
            *lookup_data.range_check_9_9_0 =
                [M31_517791011, unpacked_limb_0_col11, unpacked_limb_1_col12];
            *sub_component_inputs.range_check_9_9_b[0] =
                [unpacked_tmp_715f4_1.get_m31(2), unpacked_limb_3_col13];
            *lookup_data.range_check_9_9_b_1 =
                [M31_1897792095, unpacked_tmp_715f4_1.get_m31(2), unpacked_limb_3_col13];
            *sub_component_inputs.range_check_9_9_c[0] =
                [unpacked_limb_4_col14, unpacked_tmp_715f4_1.get_m31(5)];
            *lookup_data.range_check_9_9_c_2 =
                [M31_1881014476, unpacked_limb_4_col14, unpacked_tmp_715f4_1.get_m31(5)];
            *sub_component_inputs.range_check_9_9_d[0] =
                [unpacked_limb_6_col15, unpacked_limb_7_col16];
            *lookup_data.range_check_9_9_d_3 =
                [M31_1864236857, unpacked_limb_6_col15, unpacked_limb_7_col16];
            *sub_component_inputs.range_check_9_9_e[0] =
                [unpacked_tmp_715f4_1.get_m31(8), unpacked_limb_9_col17];
            *lookup_data.range_check_9_9_e_4 =
                [M31_1847459238, unpacked_tmp_715f4_1.get_m31(8), unpacked_limb_9_col17];
            *sub_component_inputs.range_check_9_9_f[0] =
                [unpacked_limb_10_col18, unpacked_tmp_715f4_1.get_m31(11)];
            *lookup_data.range_check_9_9_f_5 =
                [M31_1830681619, unpacked_limb_10_col18, unpacked_tmp_715f4_1.get_m31(11)];
            *sub_component_inputs.range_check_9_9_g[0] =
                [unpacked_limb_12_col19, unpacked_limb_13_col20];
            *lookup_data.range_check_9_9_g_6 =
                [M31_1813904000, unpacked_limb_12_col19, unpacked_limb_13_col20];
            *sub_component_inputs.range_check_9_9_h[0] =
                [unpacked_tmp_715f4_1.get_m31(14), unpacked_limb_15_col21];
            *lookup_data.range_check_9_9_h_7 =
                [M31_2065568285, unpacked_tmp_715f4_1.get_m31(14), unpacked_limb_15_col21];
            *sub_component_inputs.range_check_9_9[1] =
                [unpacked_limb_16_col22, unpacked_tmp_715f4_1.get_m31(17)];
            *lookup_data.range_check_9_9_8 =
                [M31_517791011, unpacked_limb_16_col22, unpacked_tmp_715f4_1.get_m31(17)];
            *sub_component_inputs.range_check_9_9_b[1] =
                [unpacked_limb_18_col23, unpacked_limb_19_col24];
            *lookup_data.range_check_9_9_b_9 =
                [M31_1897792095, unpacked_limb_18_col23, unpacked_limb_19_col24];
            *sub_component_inputs.range_check_9_9_c[1] =
                [unpacked_tmp_715f4_1.get_m31(20), unpacked_limb_21_col25];
            *lookup_data.range_check_9_9_c_10 =
                [M31_1881014476, unpacked_tmp_715f4_1.get_m31(20), unpacked_limb_21_col25];
            *sub_component_inputs.range_check_9_9_d[1] =
                [unpacked_limb_22_col26, unpacked_tmp_715f4_1.get_m31(23)];
            *lookup_data.range_check_9_9_d_11 =
                [M31_1864236857, unpacked_limb_22_col26, unpacked_tmp_715f4_1.get_m31(23)];
            *sub_component_inputs.range_check_9_9_e[1] =
                [unpacked_limb_24_col27, unpacked_limb_25_col28];
            *lookup_data.range_check_9_9_e_12 =
                [M31_1847459238, unpacked_limb_24_col27, unpacked_limb_25_col28];
            *sub_component_inputs.range_check_9_9_f[1] =
                [unpacked_tmp_715f4_1.get_m31(26), input_limb_9_col10];
            *lookup_data.range_check_9_9_f_13 =
                [M31_1830681619, unpacked_tmp_715f4_1.get_m31(26), input_limb_9_col10];

            let felt_252_unpack_from_27_range_check_output_output_tmp_715f4_2 =
                unpacked_tmp_715f4_1;

            // Mul 252.

            let mul_res_tmp_715f4_3 =
                ((felt_252_unpack_from_27_range_check_output_output_tmp_715f4_2)
                    * (felt_252_unpack_from_27_range_check_output_output_tmp_715f4_2));
            let mul_res_limb_0_col29 = mul_res_tmp_715f4_3.get_m31(0);
            *row[29] = mul_res_limb_0_col29;
            let mul_res_limb_1_col30 = mul_res_tmp_715f4_3.get_m31(1);
            *row[30] = mul_res_limb_1_col30;
            let mul_res_limb_2_col31 = mul_res_tmp_715f4_3.get_m31(2);
            *row[31] = mul_res_limb_2_col31;
            let mul_res_limb_3_col32 = mul_res_tmp_715f4_3.get_m31(3);
            *row[32] = mul_res_limb_3_col32;
            let mul_res_limb_4_col33 = mul_res_tmp_715f4_3.get_m31(4);
            *row[33] = mul_res_limb_4_col33;
            let mul_res_limb_5_col34 = mul_res_tmp_715f4_3.get_m31(5);
            *row[34] = mul_res_limb_5_col34;
            let mul_res_limb_6_col35 = mul_res_tmp_715f4_3.get_m31(6);
            *row[35] = mul_res_limb_6_col35;
            let mul_res_limb_7_col36 = mul_res_tmp_715f4_3.get_m31(7);
            *row[36] = mul_res_limb_7_col36;
            let mul_res_limb_8_col37 = mul_res_tmp_715f4_3.get_m31(8);
            *row[37] = mul_res_limb_8_col37;
            let mul_res_limb_9_col38 = mul_res_tmp_715f4_3.get_m31(9);
            *row[38] = mul_res_limb_9_col38;
            let mul_res_limb_10_col39 = mul_res_tmp_715f4_3.get_m31(10);
            *row[39] = mul_res_limb_10_col39;
            let mul_res_limb_11_col40 = mul_res_tmp_715f4_3.get_m31(11);
            *row[40] = mul_res_limb_11_col40;
            let mul_res_limb_12_col41 = mul_res_tmp_715f4_3.get_m31(12);
            *row[41] = mul_res_limb_12_col41;
            let mul_res_limb_13_col42 = mul_res_tmp_715f4_3.get_m31(13);
            *row[42] = mul_res_limb_13_col42;
            let mul_res_limb_14_col43 = mul_res_tmp_715f4_3.get_m31(14);
            *row[43] = mul_res_limb_14_col43;
            let mul_res_limb_15_col44 = mul_res_tmp_715f4_3.get_m31(15);
            *row[44] = mul_res_limb_15_col44;
            let mul_res_limb_16_col45 = mul_res_tmp_715f4_3.get_m31(16);
            *row[45] = mul_res_limb_16_col45;
            let mul_res_limb_17_col46 = mul_res_tmp_715f4_3.get_m31(17);
            *row[46] = mul_res_limb_17_col46;
            let mul_res_limb_18_col47 = mul_res_tmp_715f4_3.get_m31(18);
            *row[47] = mul_res_limb_18_col47;
            let mul_res_limb_19_col48 = mul_res_tmp_715f4_3.get_m31(19);
            *row[48] = mul_res_limb_19_col48;
            let mul_res_limb_20_col49 = mul_res_tmp_715f4_3.get_m31(20);
            *row[49] = mul_res_limb_20_col49;
            let mul_res_limb_21_col50 = mul_res_tmp_715f4_3.get_m31(21);
            *row[50] = mul_res_limb_21_col50;
            let mul_res_limb_22_col51 = mul_res_tmp_715f4_3.get_m31(22);
            *row[51] = mul_res_limb_22_col51;
            let mul_res_limb_23_col52 = mul_res_tmp_715f4_3.get_m31(23);
            *row[52] = mul_res_limb_23_col52;
            let mul_res_limb_24_col53 = mul_res_tmp_715f4_3.get_m31(24);
            *row[53] = mul_res_limb_24_col53;
            let mul_res_limb_25_col54 = mul_res_tmp_715f4_3.get_m31(25);
            *row[54] = mul_res_limb_25_col54;
            let mul_res_limb_26_col55 = mul_res_tmp_715f4_3.get_m31(26);
            *row[55] = mul_res_limb_26_col55;
            let mul_res_limb_27_col56 = mul_res_tmp_715f4_3.get_m31(27);
            *row[56] = mul_res_limb_27_col56;

            // Range Check Mem Value N 28.

            *sub_component_inputs.range_check_9_9[2] = [mul_res_limb_0_col29, mul_res_limb_1_col30];
            *lookup_data.range_check_9_9_14 =
                [M31_517791011, mul_res_limb_0_col29, mul_res_limb_1_col30];
            *sub_component_inputs.range_check_9_9_b[2] =
                [mul_res_limb_2_col31, mul_res_limb_3_col32];
            *lookup_data.range_check_9_9_b_15 =
                [M31_1897792095, mul_res_limb_2_col31, mul_res_limb_3_col32];
            *sub_component_inputs.range_check_9_9_c[2] =
                [mul_res_limb_4_col33, mul_res_limb_5_col34];
            *lookup_data.range_check_9_9_c_16 =
                [M31_1881014476, mul_res_limb_4_col33, mul_res_limb_5_col34];
            *sub_component_inputs.range_check_9_9_d[2] =
                [mul_res_limb_6_col35, mul_res_limb_7_col36];
            *lookup_data.range_check_9_9_d_17 =
                [M31_1864236857, mul_res_limb_6_col35, mul_res_limb_7_col36];
            *sub_component_inputs.range_check_9_9_e[2] =
                [mul_res_limb_8_col37, mul_res_limb_9_col38];
            *lookup_data.range_check_9_9_e_18 =
                [M31_1847459238, mul_res_limb_8_col37, mul_res_limb_9_col38];
            *sub_component_inputs.range_check_9_9_f[2] =
                [mul_res_limb_10_col39, mul_res_limb_11_col40];
            *lookup_data.range_check_9_9_f_19 =
                [M31_1830681619, mul_res_limb_10_col39, mul_res_limb_11_col40];
            *sub_component_inputs.range_check_9_9_g[1] =
                [mul_res_limb_12_col41, mul_res_limb_13_col42];
            *lookup_data.range_check_9_9_g_20 =
                [M31_1813904000, mul_res_limb_12_col41, mul_res_limb_13_col42];
            *sub_component_inputs.range_check_9_9_h[1] =
                [mul_res_limb_14_col43, mul_res_limb_15_col44];
            *lookup_data.range_check_9_9_h_21 =
                [M31_2065568285, mul_res_limb_14_col43, mul_res_limb_15_col44];
            *sub_component_inputs.range_check_9_9[3] =
                [mul_res_limb_16_col45, mul_res_limb_17_col46];
            *lookup_data.range_check_9_9_22 =
                [M31_517791011, mul_res_limb_16_col45, mul_res_limb_17_col46];
            *sub_component_inputs.range_check_9_9_b[3] =
                [mul_res_limb_18_col47, mul_res_limb_19_col48];
            *lookup_data.range_check_9_9_b_23 =
                [M31_1897792095, mul_res_limb_18_col47, mul_res_limb_19_col48];
            *sub_component_inputs.range_check_9_9_c[3] =
                [mul_res_limb_20_col49, mul_res_limb_21_col50];
            *lookup_data.range_check_9_9_c_24 =
                [M31_1881014476, mul_res_limb_20_col49, mul_res_limb_21_col50];
            *sub_component_inputs.range_check_9_9_d[3] =
                [mul_res_limb_22_col51, mul_res_limb_23_col52];
            *lookup_data.range_check_9_9_d_25 =
                [M31_1864236857, mul_res_limb_22_col51, mul_res_limb_23_col52];
            *sub_component_inputs.range_check_9_9_e[3] =
                [mul_res_limb_24_col53, mul_res_limb_25_col54];
            *lookup_data.range_check_9_9_e_26 =
                [M31_1847459238, mul_res_limb_24_col53, mul_res_limb_25_col54];
            *sub_component_inputs.range_check_9_9_f[3] =
                [mul_res_limb_26_col55, mul_res_limb_27_col56];
            *lookup_data.range_check_9_9_f_27 =
                [M31_1830681619, mul_res_limb_26_col55, mul_res_limb_27_col56];

            // Verify Mul 252.

            // Double Karatsuba F 0 Fc 6.

            // Single Karatsuba N 7.

            let z0_tmp_715f4_4 = [
                ((unpacked_limb_0_col11) * (unpacked_limb_0_col11)),
                (((unpacked_limb_0_col11) * (unpacked_limb_1_col12))
                    + ((unpacked_limb_1_col12) * (unpacked_limb_0_col11))),
                ((((unpacked_limb_0_col11) * (unpacked_tmp_715f4_1.get_m31(2)))
                    + ((unpacked_limb_1_col12) * (unpacked_limb_1_col12)))
                    + ((unpacked_tmp_715f4_1.get_m31(2)) * (unpacked_limb_0_col11))),
                (((((unpacked_limb_0_col11) * (unpacked_limb_3_col13))
                    + ((unpacked_limb_1_col12) * (unpacked_tmp_715f4_1.get_m31(2))))
                    + ((unpacked_tmp_715f4_1.get_m31(2)) * (unpacked_limb_1_col12)))
                    + ((unpacked_limb_3_col13) * (unpacked_limb_0_col11))),
                ((((((unpacked_limb_0_col11) * (unpacked_limb_4_col14))
                    + ((unpacked_limb_1_col12) * (unpacked_limb_3_col13)))
                    + ((unpacked_tmp_715f4_1.get_m31(2)) * (unpacked_tmp_715f4_1.get_m31(2))))
                    + ((unpacked_limb_3_col13) * (unpacked_limb_1_col12)))
                    + ((unpacked_limb_4_col14) * (unpacked_limb_0_col11))),
                (((((((unpacked_limb_0_col11) * (unpacked_tmp_715f4_1.get_m31(5)))
                    + ((unpacked_limb_1_col12) * (unpacked_limb_4_col14)))
                    + ((unpacked_tmp_715f4_1.get_m31(2)) * (unpacked_limb_3_col13)))
                    + ((unpacked_limb_3_col13) * (unpacked_tmp_715f4_1.get_m31(2))))
                    + ((unpacked_limb_4_col14) * (unpacked_limb_1_col12)))
                    + ((unpacked_tmp_715f4_1.get_m31(5)) * (unpacked_limb_0_col11))),
                ((((((((unpacked_limb_0_col11) * (unpacked_limb_6_col15))
                    + ((unpacked_limb_1_col12) * (unpacked_tmp_715f4_1.get_m31(5))))
                    + ((unpacked_tmp_715f4_1.get_m31(2)) * (unpacked_limb_4_col14)))
                    + ((unpacked_limb_3_col13) * (unpacked_limb_3_col13)))
                    + ((unpacked_limb_4_col14) * (unpacked_tmp_715f4_1.get_m31(2))))
                    + ((unpacked_tmp_715f4_1.get_m31(5)) * (unpacked_limb_1_col12)))
                    + ((unpacked_limb_6_col15) * (unpacked_limb_0_col11))),
                (((((((unpacked_limb_1_col12) * (unpacked_limb_6_col15))
                    + ((unpacked_tmp_715f4_1.get_m31(2)) * (unpacked_tmp_715f4_1.get_m31(5))))
                    + ((unpacked_limb_3_col13) * (unpacked_limb_4_col14)))
                    + ((unpacked_limb_4_col14) * (unpacked_limb_3_col13)))
                    + ((unpacked_tmp_715f4_1.get_m31(5)) * (unpacked_tmp_715f4_1.get_m31(2))))
                    + ((unpacked_limb_6_col15) * (unpacked_limb_1_col12))),
                ((((((unpacked_tmp_715f4_1.get_m31(2)) * (unpacked_limb_6_col15))
                    + ((unpacked_limb_3_col13) * (unpacked_tmp_715f4_1.get_m31(5))))
                    + ((unpacked_limb_4_col14) * (unpacked_limb_4_col14)))
                    + ((unpacked_tmp_715f4_1.get_m31(5)) * (unpacked_limb_3_col13)))
                    + ((unpacked_limb_6_col15) * (unpacked_tmp_715f4_1.get_m31(2)))),
                (((((unpacked_limb_3_col13) * (unpacked_limb_6_col15))
                    + ((unpacked_limb_4_col14) * (unpacked_tmp_715f4_1.get_m31(5))))
                    + ((unpacked_tmp_715f4_1.get_m31(5)) * (unpacked_limb_4_col14)))
                    + ((unpacked_limb_6_col15) * (unpacked_limb_3_col13))),
                ((((unpacked_limb_4_col14) * (unpacked_limb_6_col15))
                    + ((unpacked_tmp_715f4_1.get_m31(5)) * (unpacked_tmp_715f4_1.get_m31(5))))
                    + ((unpacked_limb_6_col15) * (unpacked_limb_4_col14))),
                (((unpacked_tmp_715f4_1.get_m31(5)) * (unpacked_limb_6_col15))
                    + ((unpacked_limb_6_col15) * (unpacked_tmp_715f4_1.get_m31(5)))),
                ((unpacked_limb_6_col15) * (unpacked_limb_6_col15)),
            ];
            let z2_tmp_715f4_5 = [
                ((unpacked_limb_7_col16) * (unpacked_limb_7_col16)),
                (((unpacked_limb_7_col16) * (unpacked_tmp_715f4_1.get_m31(8)))
                    + ((unpacked_tmp_715f4_1.get_m31(8)) * (unpacked_limb_7_col16))),
                ((((unpacked_limb_7_col16) * (unpacked_limb_9_col17))
                    + ((unpacked_tmp_715f4_1.get_m31(8)) * (unpacked_tmp_715f4_1.get_m31(8))))
                    + ((unpacked_limb_9_col17) * (unpacked_limb_7_col16))),
                (((((unpacked_limb_7_col16) * (unpacked_limb_10_col18))
                    + ((unpacked_tmp_715f4_1.get_m31(8)) * (unpacked_limb_9_col17)))
                    + ((unpacked_limb_9_col17) * (unpacked_tmp_715f4_1.get_m31(8))))
                    + ((unpacked_limb_10_col18) * (unpacked_limb_7_col16))),
                ((((((unpacked_limb_7_col16) * (unpacked_tmp_715f4_1.get_m31(11)))
                    + ((unpacked_tmp_715f4_1.get_m31(8)) * (unpacked_limb_10_col18)))
                    + ((unpacked_limb_9_col17) * (unpacked_limb_9_col17)))
                    + ((unpacked_limb_10_col18) * (unpacked_tmp_715f4_1.get_m31(8))))
                    + ((unpacked_tmp_715f4_1.get_m31(11)) * (unpacked_limb_7_col16))),
                (((((((unpacked_limb_7_col16) * (unpacked_limb_12_col19))
                    + ((unpacked_tmp_715f4_1.get_m31(8)) * (unpacked_tmp_715f4_1.get_m31(11))))
                    + ((unpacked_limb_9_col17) * (unpacked_limb_10_col18)))
                    + ((unpacked_limb_10_col18) * (unpacked_limb_9_col17)))
                    + ((unpacked_tmp_715f4_1.get_m31(11)) * (unpacked_tmp_715f4_1.get_m31(8))))
                    + ((unpacked_limb_12_col19) * (unpacked_limb_7_col16))),
                ((((((((unpacked_limb_7_col16) * (unpacked_limb_13_col20))
                    + ((unpacked_tmp_715f4_1.get_m31(8)) * (unpacked_limb_12_col19)))
                    + ((unpacked_limb_9_col17) * (unpacked_tmp_715f4_1.get_m31(11))))
                    + ((unpacked_limb_10_col18) * (unpacked_limb_10_col18)))
                    + ((unpacked_tmp_715f4_1.get_m31(11)) * (unpacked_limb_9_col17)))
                    + ((unpacked_limb_12_col19) * (unpacked_tmp_715f4_1.get_m31(8))))
                    + ((unpacked_limb_13_col20) * (unpacked_limb_7_col16))),
                (((((((unpacked_tmp_715f4_1.get_m31(8)) * (unpacked_limb_13_col20))
                    + ((unpacked_limb_9_col17) * (unpacked_limb_12_col19)))
                    + ((unpacked_limb_10_col18) * (unpacked_tmp_715f4_1.get_m31(11))))
                    + ((unpacked_tmp_715f4_1.get_m31(11)) * (unpacked_limb_10_col18)))
                    + ((unpacked_limb_12_col19) * (unpacked_limb_9_col17)))
                    + ((unpacked_limb_13_col20) * (unpacked_tmp_715f4_1.get_m31(8)))),
                ((((((unpacked_limb_9_col17) * (unpacked_limb_13_col20))
                    + ((unpacked_limb_10_col18) * (unpacked_limb_12_col19)))
                    + ((unpacked_tmp_715f4_1.get_m31(11)) * (unpacked_tmp_715f4_1.get_m31(11))))
                    + ((unpacked_limb_12_col19) * (unpacked_limb_10_col18)))
                    + ((unpacked_limb_13_col20) * (unpacked_limb_9_col17))),
                (((((unpacked_limb_10_col18) * (unpacked_limb_13_col20))
                    + ((unpacked_tmp_715f4_1.get_m31(11)) * (unpacked_limb_12_col19)))
                    + ((unpacked_limb_12_col19) * (unpacked_tmp_715f4_1.get_m31(11))))
                    + ((unpacked_limb_13_col20) * (unpacked_limb_10_col18))),
                ((((unpacked_tmp_715f4_1.get_m31(11)) * (unpacked_limb_13_col20))
                    + ((unpacked_limb_12_col19) * (unpacked_limb_12_col19)))
                    + ((unpacked_limb_13_col20) * (unpacked_tmp_715f4_1.get_m31(11)))),
                (((unpacked_limb_12_col19) * (unpacked_limb_13_col20))
                    + ((unpacked_limb_13_col20) * (unpacked_limb_12_col19))),
                ((unpacked_limb_13_col20) * (unpacked_limb_13_col20)),
            ];
            let x_sum_tmp_715f4_6 = [
                ((unpacked_limb_0_col11) + (unpacked_limb_7_col16)),
                ((unpacked_limb_1_col12) + (unpacked_tmp_715f4_1.get_m31(8))),
                ((unpacked_tmp_715f4_1.get_m31(2)) + (unpacked_limb_9_col17)),
                ((unpacked_limb_3_col13) + (unpacked_limb_10_col18)),
                ((unpacked_limb_4_col14) + (unpacked_tmp_715f4_1.get_m31(11))),
                ((unpacked_tmp_715f4_1.get_m31(5)) + (unpacked_limb_12_col19)),
                ((unpacked_limb_6_col15) + (unpacked_limb_13_col20)),
            ];
            let y_sum_tmp_715f4_7 = [
                ((unpacked_limb_0_col11) + (unpacked_limb_7_col16)),
                ((unpacked_limb_1_col12) + (unpacked_tmp_715f4_1.get_m31(8))),
                ((unpacked_tmp_715f4_1.get_m31(2)) + (unpacked_limb_9_col17)),
                ((unpacked_limb_3_col13) + (unpacked_limb_10_col18)),
                ((unpacked_limb_4_col14) + (unpacked_tmp_715f4_1.get_m31(11))),
                ((unpacked_tmp_715f4_1.get_m31(5)) + (unpacked_limb_12_col19)),
                ((unpacked_limb_6_col15) + (unpacked_limb_13_col20)),
            ];
            let single_karatsuba_n_7_output_tmp_715f4_8 = [
                z0_tmp_715f4_4[0],
                z0_tmp_715f4_4[1],
                z0_tmp_715f4_4[2],
                z0_tmp_715f4_4[3],
                z0_tmp_715f4_4[4],
                z0_tmp_715f4_4[5],
                z0_tmp_715f4_4[6],
                ((z0_tmp_715f4_4[7])
                    + ((((x_sum_tmp_715f4_6[0]) * (y_sum_tmp_715f4_7[0])) - (z0_tmp_715f4_4[0]))
                        - (z2_tmp_715f4_5[0]))),
                ((z0_tmp_715f4_4[8])
                    + (((((x_sum_tmp_715f4_6[0]) * (y_sum_tmp_715f4_7[1]))
                        + ((x_sum_tmp_715f4_6[1]) * (y_sum_tmp_715f4_7[0])))
                        - (z0_tmp_715f4_4[1]))
                        - (z2_tmp_715f4_5[1]))),
                ((z0_tmp_715f4_4[9])
                    + ((((((x_sum_tmp_715f4_6[0]) * (y_sum_tmp_715f4_7[2]))
                        + ((x_sum_tmp_715f4_6[1]) * (y_sum_tmp_715f4_7[1])))
                        + ((x_sum_tmp_715f4_6[2]) * (y_sum_tmp_715f4_7[0])))
                        - (z0_tmp_715f4_4[2]))
                        - (z2_tmp_715f4_5[2]))),
                ((z0_tmp_715f4_4[10])
                    + (((((((x_sum_tmp_715f4_6[0]) * (y_sum_tmp_715f4_7[3]))
                        + ((x_sum_tmp_715f4_6[1]) * (y_sum_tmp_715f4_7[2])))
                        + ((x_sum_tmp_715f4_6[2]) * (y_sum_tmp_715f4_7[1])))
                        + ((x_sum_tmp_715f4_6[3]) * (y_sum_tmp_715f4_7[0])))
                        - (z0_tmp_715f4_4[3]))
                        - (z2_tmp_715f4_5[3]))),
                ((z0_tmp_715f4_4[11])
                    + ((((((((x_sum_tmp_715f4_6[0]) * (y_sum_tmp_715f4_7[4]))
                        + ((x_sum_tmp_715f4_6[1]) * (y_sum_tmp_715f4_7[3])))
                        + ((x_sum_tmp_715f4_6[2]) * (y_sum_tmp_715f4_7[2])))
                        + ((x_sum_tmp_715f4_6[3]) * (y_sum_tmp_715f4_7[1])))
                        + ((x_sum_tmp_715f4_6[4]) * (y_sum_tmp_715f4_7[0])))
                        - (z0_tmp_715f4_4[4]))
                        - (z2_tmp_715f4_5[4]))),
                ((z0_tmp_715f4_4[12])
                    + (((((((((x_sum_tmp_715f4_6[0]) * (y_sum_tmp_715f4_7[5]))
                        + ((x_sum_tmp_715f4_6[1]) * (y_sum_tmp_715f4_7[4])))
                        + ((x_sum_tmp_715f4_6[2]) * (y_sum_tmp_715f4_7[3])))
                        + ((x_sum_tmp_715f4_6[3]) * (y_sum_tmp_715f4_7[2])))
                        + ((x_sum_tmp_715f4_6[4]) * (y_sum_tmp_715f4_7[1])))
                        + ((x_sum_tmp_715f4_6[5]) * (y_sum_tmp_715f4_7[0])))
                        - (z0_tmp_715f4_4[5]))
                        - (z2_tmp_715f4_5[5]))),
                ((((((((((x_sum_tmp_715f4_6[0]) * (y_sum_tmp_715f4_7[6]))
                    + ((x_sum_tmp_715f4_6[1]) * (y_sum_tmp_715f4_7[5])))
                    + ((x_sum_tmp_715f4_6[2]) * (y_sum_tmp_715f4_7[4])))
                    + ((x_sum_tmp_715f4_6[3]) * (y_sum_tmp_715f4_7[3])))
                    + ((x_sum_tmp_715f4_6[4]) * (y_sum_tmp_715f4_7[2])))
                    + ((x_sum_tmp_715f4_6[5]) * (y_sum_tmp_715f4_7[1])))
                    + ((x_sum_tmp_715f4_6[6]) * (y_sum_tmp_715f4_7[0])))
                    - (z0_tmp_715f4_4[6]))
                    - (z2_tmp_715f4_5[6])),
                ((z2_tmp_715f4_5[0])
                    + (((((((((x_sum_tmp_715f4_6[1]) * (y_sum_tmp_715f4_7[6]))
                        + ((x_sum_tmp_715f4_6[2]) * (y_sum_tmp_715f4_7[5])))
                        + ((x_sum_tmp_715f4_6[3]) * (y_sum_tmp_715f4_7[4])))
                        + ((x_sum_tmp_715f4_6[4]) * (y_sum_tmp_715f4_7[3])))
                        + ((x_sum_tmp_715f4_6[5]) * (y_sum_tmp_715f4_7[2])))
                        + ((x_sum_tmp_715f4_6[6]) * (y_sum_tmp_715f4_7[1])))
                        - (z0_tmp_715f4_4[7]))
                        - (z2_tmp_715f4_5[7]))),
                ((z2_tmp_715f4_5[1])
                    + ((((((((x_sum_tmp_715f4_6[2]) * (y_sum_tmp_715f4_7[6]))
                        + ((x_sum_tmp_715f4_6[3]) * (y_sum_tmp_715f4_7[5])))
                        + ((x_sum_tmp_715f4_6[4]) * (y_sum_tmp_715f4_7[4])))
                        + ((x_sum_tmp_715f4_6[5]) * (y_sum_tmp_715f4_7[3])))
                        + ((x_sum_tmp_715f4_6[6]) * (y_sum_tmp_715f4_7[2])))
                        - (z0_tmp_715f4_4[8]))
                        - (z2_tmp_715f4_5[8]))),
                ((z2_tmp_715f4_5[2])
                    + (((((((x_sum_tmp_715f4_6[3]) * (y_sum_tmp_715f4_7[6]))
                        + ((x_sum_tmp_715f4_6[4]) * (y_sum_tmp_715f4_7[5])))
                        + ((x_sum_tmp_715f4_6[5]) * (y_sum_tmp_715f4_7[4])))
                        + ((x_sum_tmp_715f4_6[6]) * (y_sum_tmp_715f4_7[3])))
                        - (z0_tmp_715f4_4[9]))
                        - (z2_tmp_715f4_5[9]))),
                ((z2_tmp_715f4_5[3])
                    + ((((((x_sum_tmp_715f4_6[4]) * (y_sum_tmp_715f4_7[6]))
                        + ((x_sum_tmp_715f4_6[5]) * (y_sum_tmp_715f4_7[5])))
                        + ((x_sum_tmp_715f4_6[6]) * (y_sum_tmp_715f4_7[4])))
                        - (z0_tmp_715f4_4[10]))
                        - (z2_tmp_715f4_5[10]))),
                ((z2_tmp_715f4_5[4])
                    + (((((x_sum_tmp_715f4_6[5]) * (y_sum_tmp_715f4_7[6]))
                        + ((x_sum_tmp_715f4_6[6]) * (y_sum_tmp_715f4_7[5])))
                        - (z0_tmp_715f4_4[11]))
                        - (z2_tmp_715f4_5[11]))),
                ((z2_tmp_715f4_5[5])
                    + ((((x_sum_tmp_715f4_6[6]) * (y_sum_tmp_715f4_7[6])) - (z0_tmp_715f4_4[12]))
                        - (z2_tmp_715f4_5[12]))),
                z2_tmp_715f4_5[6],
                z2_tmp_715f4_5[7],
                z2_tmp_715f4_5[8],
                z2_tmp_715f4_5[9],
                z2_tmp_715f4_5[10],
                z2_tmp_715f4_5[11],
                z2_tmp_715f4_5[12],
            ];

            // Single Karatsuba N 7.

            let z0_tmp_715f4_9 = [
                ((unpacked_tmp_715f4_1.get_m31(14)) * (unpacked_tmp_715f4_1.get_m31(14))),
                (((unpacked_tmp_715f4_1.get_m31(14)) * (unpacked_limb_15_col21))
                    + ((unpacked_limb_15_col21) * (unpacked_tmp_715f4_1.get_m31(14)))),
                ((((unpacked_tmp_715f4_1.get_m31(14)) * (unpacked_limb_16_col22))
                    + ((unpacked_limb_15_col21) * (unpacked_limb_15_col21)))
                    + ((unpacked_limb_16_col22) * (unpacked_tmp_715f4_1.get_m31(14)))),
                (((((unpacked_tmp_715f4_1.get_m31(14)) * (unpacked_tmp_715f4_1.get_m31(17)))
                    + ((unpacked_limb_15_col21) * (unpacked_limb_16_col22)))
                    + ((unpacked_limb_16_col22) * (unpacked_limb_15_col21)))
                    + ((unpacked_tmp_715f4_1.get_m31(17)) * (unpacked_tmp_715f4_1.get_m31(14)))),
                ((((((unpacked_tmp_715f4_1.get_m31(14)) * (unpacked_limb_18_col23))
                    + ((unpacked_limb_15_col21) * (unpacked_tmp_715f4_1.get_m31(17))))
                    + ((unpacked_limb_16_col22) * (unpacked_limb_16_col22)))
                    + ((unpacked_tmp_715f4_1.get_m31(17)) * (unpacked_limb_15_col21)))
                    + ((unpacked_limb_18_col23) * (unpacked_tmp_715f4_1.get_m31(14)))),
                (((((((unpacked_tmp_715f4_1.get_m31(14)) * (unpacked_limb_19_col24))
                    + ((unpacked_limb_15_col21) * (unpacked_limb_18_col23)))
                    + ((unpacked_limb_16_col22) * (unpacked_tmp_715f4_1.get_m31(17))))
                    + ((unpacked_tmp_715f4_1.get_m31(17)) * (unpacked_limb_16_col22)))
                    + ((unpacked_limb_18_col23) * (unpacked_limb_15_col21)))
                    + ((unpacked_limb_19_col24) * (unpacked_tmp_715f4_1.get_m31(14)))),
                ((((((((unpacked_tmp_715f4_1.get_m31(14))
                    * (unpacked_tmp_715f4_1.get_m31(20)))
                    + ((unpacked_limb_15_col21) * (unpacked_limb_19_col24)))
                    + ((unpacked_limb_16_col22) * (unpacked_limb_18_col23)))
                    + ((unpacked_tmp_715f4_1.get_m31(17)) * (unpacked_tmp_715f4_1.get_m31(17))))
                    + ((unpacked_limb_18_col23) * (unpacked_limb_16_col22)))
                    + ((unpacked_limb_19_col24) * (unpacked_limb_15_col21)))
                    + ((unpacked_tmp_715f4_1.get_m31(20)) * (unpacked_tmp_715f4_1.get_m31(14)))),
                (((((((unpacked_limb_15_col21) * (unpacked_tmp_715f4_1.get_m31(20)))
                    + ((unpacked_limb_16_col22) * (unpacked_limb_19_col24)))
                    + ((unpacked_tmp_715f4_1.get_m31(17)) * (unpacked_limb_18_col23)))
                    + ((unpacked_limb_18_col23) * (unpacked_tmp_715f4_1.get_m31(17))))
                    + ((unpacked_limb_19_col24) * (unpacked_limb_16_col22)))
                    + ((unpacked_tmp_715f4_1.get_m31(20)) * (unpacked_limb_15_col21))),
                ((((((unpacked_limb_16_col22) * (unpacked_tmp_715f4_1.get_m31(20)))
                    + ((unpacked_tmp_715f4_1.get_m31(17)) * (unpacked_limb_19_col24)))
                    + ((unpacked_limb_18_col23) * (unpacked_limb_18_col23)))
                    + ((unpacked_limb_19_col24) * (unpacked_tmp_715f4_1.get_m31(17))))
                    + ((unpacked_tmp_715f4_1.get_m31(20)) * (unpacked_limb_16_col22))),
                (((((unpacked_tmp_715f4_1.get_m31(17)) * (unpacked_tmp_715f4_1.get_m31(20)))
                    + ((unpacked_limb_18_col23) * (unpacked_limb_19_col24)))
                    + ((unpacked_limb_19_col24) * (unpacked_limb_18_col23)))
                    + ((unpacked_tmp_715f4_1.get_m31(20)) * (unpacked_tmp_715f4_1.get_m31(17)))),
                ((((unpacked_limb_18_col23) * (unpacked_tmp_715f4_1.get_m31(20)))
                    + ((unpacked_limb_19_col24) * (unpacked_limb_19_col24)))
                    + ((unpacked_tmp_715f4_1.get_m31(20)) * (unpacked_limb_18_col23))),
                (((unpacked_limb_19_col24) * (unpacked_tmp_715f4_1.get_m31(20)))
                    + ((unpacked_tmp_715f4_1.get_m31(20)) * (unpacked_limb_19_col24))),
                ((unpacked_tmp_715f4_1.get_m31(20)) * (unpacked_tmp_715f4_1.get_m31(20))),
            ];
            let z2_tmp_715f4_10 = [
                ((unpacked_limb_21_col25) * (unpacked_limb_21_col25)),
                (((unpacked_limb_21_col25) * (unpacked_limb_22_col26))
                    + ((unpacked_limb_22_col26) * (unpacked_limb_21_col25))),
                ((((unpacked_limb_21_col25) * (unpacked_tmp_715f4_1.get_m31(23)))
                    + ((unpacked_limb_22_col26) * (unpacked_limb_22_col26)))
                    + ((unpacked_tmp_715f4_1.get_m31(23)) * (unpacked_limb_21_col25))),
                (((((unpacked_limb_21_col25) * (unpacked_limb_24_col27))
                    + ((unpacked_limb_22_col26) * (unpacked_tmp_715f4_1.get_m31(23))))
                    + ((unpacked_tmp_715f4_1.get_m31(23)) * (unpacked_limb_22_col26)))
                    + ((unpacked_limb_24_col27) * (unpacked_limb_21_col25))),
                ((((((unpacked_limb_21_col25) * (unpacked_limb_25_col28))
                    + ((unpacked_limb_22_col26) * (unpacked_limb_24_col27)))
                    + ((unpacked_tmp_715f4_1.get_m31(23)) * (unpacked_tmp_715f4_1.get_m31(23))))
                    + ((unpacked_limb_24_col27) * (unpacked_limb_22_col26)))
                    + ((unpacked_limb_25_col28) * (unpacked_limb_21_col25))),
                (((((((unpacked_limb_21_col25) * (unpacked_tmp_715f4_1.get_m31(26)))
                    + ((unpacked_limb_22_col26) * (unpacked_limb_25_col28)))
                    + ((unpacked_tmp_715f4_1.get_m31(23)) * (unpacked_limb_24_col27)))
                    + ((unpacked_limb_24_col27) * (unpacked_tmp_715f4_1.get_m31(23))))
                    + ((unpacked_limb_25_col28) * (unpacked_limb_22_col26)))
                    + ((unpacked_tmp_715f4_1.get_m31(26)) * (unpacked_limb_21_col25))),
                ((((((((unpacked_limb_21_col25) * (input_limb_9_col10))
                    + ((unpacked_limb_22_col26) * (unpacked_tmp_715f4_1.get_m31(26))))
                    + ((unpacked_tmp_715f4_1.get_m31(23)) * (unpacked_limb_25_col28)))
                    + ((unpacked_limb_24_col27) * (unpacked_limb_24_col27)))
                    + ((unpacked_limb_25_col28) * (unpacked_tmp_715f4_1.get_m31(23))))
                    + ((unpacked_tmp_715f4_1.get_m31(26)) * (unpacked_limb_22_col26)))
                    + ((input_limb_9_col10) * (unpacked_limb_21_col25))),
                (((((((unpacked_limb_22_col26) * (input_limb_9_col10))
                    + ((unpacked_tmp_715f4_1.get_m31(23))
                        * (unpacked_tmp_715f4_1.get_m31(26))))
                    + ((unpacked_limb_24_col27) * (unpacked_limb_25_col28)))
                    + ((unpacked_limb_25_col28) * (unpacked_limb_24_col27)))
                    + ((unpacked_tmp_715f4_1.get_m31(26)) * (unpacked_tmp_715f4_1.get_m31(23))))
                    + ((input_limb_9_col10) * (unpacked_limb_22_col26))),
                ((((((unpacked_tmp_715f4_1.get_m31(23)) * (input_limb_9_col10))
                    + ((unpacked_limb_24_col27) * (unpacked_tmp_715f4_1.get_m31(26))))
                    + ((unpacked_limb_25_col28) * (unpacked_limb_25_col28)))
                    + ((unpacked_tmp_715f4_1.get_m31(26)) * (unpacked_limb_24_col27)))
                    + ((input_limb_9_col10) * (unpacked_tmp_715f4_1.get_m31(23)))),
                (((((unpacked_limb_24_col27) * (input_limb_9_col10))
                    + ((unpacked_limb_25_col28) * (unpacked_tmp_715f4_1.get_m31(26))))
                    + ((unpacked_tmp_715f4_1.get_m31(26)) * (unpacked_limb_25_col28)))
                    + ((input_limb_9_col10) * (unpacked_limb_24_col27))),
                ((((unpacked_limb_25_col28) * (input_limb_9_col10))
                    + ((unpacked_tmp_715f4_1.get_m31(26)) * (unpacked_tmp_715f4_1.get_m31(26))))
                    + ((input_limb_9_col10) * (unpacked_limb_25_col28))),
                (((unpacked_tmp_715f4_1.get_m31(26)) * (input_limb_9_col10))
                    + ((input_limb_9_col10) * (unpacked_tmp_715f4_1.get_m31(26)))),
                ((input_limb_9_col10) * (input_limb_9_col10)),
            ];
            let x_sum_tmp_715f4_11 = [
                ((unpacked_tmp_715f4_1.get_m31(14)) + (unpacked_limb_21_col25)),
                ((unpacked_limb_15_col21) + (unpacked_limb_22_col26)),
                ((unpacked_limb_16_col22) + (unpacked_tmp_715f4_1.get_m31(23))),
                ((unpacked_tmp_715f4_1.get_m31(17)) + (unpacked_limb_24_col27)),
                ((unpacked_limb_18_col23) + (unpacked_limb_25_col28)),
                ((unpacked_limb_19_col24) + (unpacked_tmp_715f4_1.get_m31(26))),
                ((unpacked_tmp_715f4_1.get_m31(20)) + (input_limb_9_col10)),
            ];
            let y_sum_tmp_715f4_12 = [
                ((unpacked_tmp_715f4_1.get_m31(14)) + (unpacked_limb_21_col25)),
                ((unpacked_limb_15_col21) + (unpacked_limb_22_col26)),
                ((unpacked_limb_16_col22) + (unpacked_tmp_715f4_1.get_m31(23))),
                ((unpacked_tmp_715f4_1.get_m31(17)) + (unpacked_limb_24_col27)),
                ((unpacked_limb_18_col23) + (unpacked_limb_25_col28)),
                ((unpacked_limb_19_col24) + (unpacked_tmp_715f4_1.get_m31(26))),
                ((unpacked_tmp_715f4_1.get_m31(20)) + (input_limb_9_col10)),
            ];
            let single_karatsuba_n_7_output_tmp_715f4_13 = [
                z0_tmp_715f4_9[0],
                z0_tmp_715f4_9[1],
                z0_tmp_715f4_9[2],
                z0_tmp_715f4_9[3],
                z0_tmp_715f4_9[4],
                z0_tmp_715f4_9[5],
                z0_tmp_715f4_9[6],
                ((z0_tmp_715f4_9[7])
                    + ((((x_sum_tmp_715f4_11[0]) * (y_sum_tmp_715f4_12[0]))
                        - (z0_tmp_715f4_9[0]))
                        - (z2_tmp_715f4_10[0]))),
                ((z0_tmp_715f4_9[8])
                    + (((((x_sum_tmp_715f4_11[0]) * (y_sum_tmp_715f4_12[1]))
                        + ((x_sum_tmp_715f4_11[1]) * (y_sum_tmp_715f4_12[0])))
                        - (z0_tmp_715f4_9[1]))
                        - (z2_tmp_715f4_10[1]))),
                ((z0_tmp_715f4_9[9])
                    + ((((((x_sum_tmp_715f4_11[0]) * (y_sum_tmp_715f4_12[2]))
                        + ((x_sum_tmp_715f4_11[1]) * (y_sum_tmp_715f4_12[1])))
                        + ((x_sum_tmp_715f4_11[2]) * (y_sum_tmp_715f4_12[0])))
                        - (z0_tmp_715f4_9[2]))
                        - (z2_tmp_715f4_10[2]))),
                ((z0_tmp_715f4_9[10])
                    + (((((((x_sum_tmp_715f4_11[0]) * (y_sum_tmp_715f4_12[3]))
                        + ((x_sum_tmp_715f4_11[1]) * (y_sum_tmp_715f4_12[2])))
                        + ((x_sum_tmp_715f4_11[2]) * (y_sum_tmp_715f4_12[1])))
                        + ((x_sum_tmp_715f4_11[3]) * (y_sum_tmp_715f4_12[0])))
                        - (z0_tmp_715f4_9[3]))
                        - (z2_tmp_715f4_10[3]))),
                ((z0_tmp_715f4_9[11])
                    + ((((((((x_sum_tmp_715f4_11[0]) * (y_sum_tmp_715f4_12[4]))
                        + ((x_sum_tmp_715f4_11[1]) * (y_sum_tmp_715f4_12[3])))
                        + ((x_sum_tmp_715f4_11[2]) * (y_sum_tmp_715f4_12[2])))
                        + ((x_sum_tmp_715f4_11[3]) * (y_sum_tmp_715f4_12[1])))
                        + ((x_sum_tmp_715f4_11[4]) * (y_sum_tmp_715f4_12[0])))
                        - (z0_tmp_715f4_9[4]))
                        - (z2_tmp_715f4_10[4]))),
                ((z0_tmp_715f4_9[12])
                    + (((((((((x_sum_tmp_715f4_11[0]) * (y_sum_tmp_715f4_12[5]))
                        + ((x_sum_tmp_715f4_11[1]) * (y_sum_tmp_715f4_12[4])))
                        + ((x_sum_tmp_715f4_11[2]) * (y_sum_tmp_715f4_12[3])))
                        + ((x_sum_tmp_715f4_11[3]) * (y_sum_tmp_715f4_12[2])))
                        + ((x_sum_tmp_715f4_11[4]) * (y_sum_tmp_715f4_12[1])))
                        + ((x_sum_tmp_715f4_11[5]) * (y_sum_tmp_715f4_12[0])))
                        - (z0_tmp_715f4_9[5]))
                        - (z2_tmp_715f4_10[5]))),
                ((((((((((x_sum_tmp_715f4_11[0]) * (y_sum_tmp_715f4_12[6]))
                    + ((x_sum_tmp_715f4_11[1]) * (y_sum_tmp_715f4_12[5])))
                    + ((x_sum_tmp_715f4_11[2]) * (y_sum_tmp_715f4_12[4])))
                    + ((x_sum_tmp_715f4_11[3]) * (y_sum_tmp_715f4_12[3])))
                    + ((x_sum_tmp_715f4_11[4]) * (y_sum_tmp_715f4_12[2])))
                    + ((x_sum_tmp_715f4_11[5]) * (y_sum_tmp_715f4_12[1])))
                    + ((x_sum_tmp_715f4_11[6]) * (y_sum_tmp_715f4_12[0])))
                    - (z0_tmp_715f4_9[6]))
                    - (z2_tmp_715f4_10[6])),
                ((z2_tmp_715f4_10[0])
                    + (((((((((x_sum_tmp_715f4_11[1]) * (y_sum_tmp_715f4_12[6]))
                        + ((x_sum_tmp_715f4_11[2]) * (y_sum_tmp_715f4_12[5])))
                        + ((x_sum_tmp_715f4_11[3]) * (y_sum_tmp_715f4_12[4])))
                        + ((x_sum_tmp_715f4_11[4]) * (y_sum_tmp_715f4_12[3])))
                        + ((x_sum_tmp_715f4_11[5]) * (y_sum_tmp_715f4_12[2])))
                        + ((x_sum_tmp_715f4_11[6]) * (y_sum_tmp_715f4_12[1])))
                        - (z0_tmp_715f4_9[7]))
                        - (z2_tmp_715f4_10[7]))),
                ((z2_tmp_715f4_10[1])
                    + ((((((((x_sum_tmp_715f4_11[2]) * (y_sum_tmp_715f4_12[6]))
                        + ((x_sum_tmp_715f4_11[3]) * (y_sum_tmp_715f4_12[5])))
                        + ((x_sum_tmp_715f4_11[4]) * (y_sum_tmp_715f4_12[4])))
                        + ((x_sum_tmp_715f4_11[5]) * (y_sum_tmp_715f4_12[3])))
                        + ((x_sum_tmp_715f4_11[6]) * (y_sum_tmp_715f4_12[2])))
                        - (z0_tmp_715f4_9[8]))
                        - (z2_tmp_715f4_10[8]))),
                ((z2_tmp_715f4_10[2])
                    + (((((((x_sum_tmp_715f4_11[3]) * (y_sum_tmp_715f4_12[6]))
                        + ((x_sum_tmp_715f4_11[4]) * (y_sum_tmp_715f4_12[5])))
                        + ((x_sum_tmp_715f4_11[5]) * (y_sum_tmp_715f4_12[4])))
                        + ((x_sum_tmp_715f4_11[6]) * (y_sum_tmp_715f4_12[3])))
                        - (z0_tmp_715f4_9[9]))
                        - (z2_tmp_715f4_10[9]))),
                ((z2_tmp_715f4_10[3])
                    + ((((((x_sum_tmp_715f4_11[4]) * (y_sum_tmp_715f4_12[6]))
                        + ((x_sum_tmp_715f4_11[5]) * (y_sum_tmp_715f4_12[5])))
                        + ((x_sum_tmp_715f4_11[6]) * (y_sum_tmp_715f4_12[4])))
                        - (z0_tmp_715f4_9[10]))
                        - (z2_tmp_715f4_10[10]))),
                ((z2_tmp_715f4_10[4])
                    + (((((x_sum_tmp_715f4_11[5]) * (y_sum_tmp_715f4_12[6]))
                        + ((x_sum_tmp_715f4_11[6]) * (y_sum_tmp_715f4_12[5])))
                        - (z0_tmp_715f4_9[11]))
                        - (z2_tmp_715f4_10[11]))),
                ((z2_tmp_715f4_10[5])
                    + ((((x_sum_tmp_715f4_11[6]) * (y_sum_tmp_715f4_12[6]))
                        - (z0_tmp_715f4_9[12]))
                        - (z2_tmp_715f4_10[12]))),
                z2_tmp_715f4_10[6],
                z2_tmp_715f4_10[7],
                z2_tmp_715f4_10[8],
                z2_tmp_715f4_10[9],
                z2_tmp_715f4_10[10],
                z2_tmp_715f4_10[11],
                z2_tmp_715f4_10[12],
            ];

            let x_sum_tmp_715f4_14 = [
                ((unpacked_limb_0_col11) + (unpacked_tmp_715f4_1.get_m31(14))),
                ((unpacked_limb_1_col12) + (unpacked_limb_15_col21)),
                ((unpacked_tmp_715f4_1.get_m31(2)) + (unpacked_limb_16_col22)),
                ((unpacked_limb_3_col13) + (unpacked_tmp_715f4_1.get_m31(17))),
                ((unpacked_limb_4_col14) + (unpacked_limb_18_col23)),
                ((unpacked_tmp_715f4_1.get_m31(5)) + (unpacked_limb_19_col24)),
                ((unpacked_limb_6_col15) + (unpacked_tmp_715f4_1.get_m31(20))),
                ((unpacked_limb_7_col16) + (unpacked_limb_21_col25)),
                ((unpacked_tmp_715f4_1.get_m31(8)) + (unpacked_limb_22_col26)),
                ((unpacked_limb_9_col17) + (unpacked_tmp_715f4_1.get_m31(23))),
                ((unpacked_limb_10_col18) + (unpacked_limb_24_col27)),
                ((unpacked_tmp_715f4_1.get_m31(11)) + (unpacked_limb_25_col28)),
                ((unpacked_limb_12_col19) + (unpacked_tmp_715f4_1.get_m31(26))),
                ((unpacked_limb_13_col20) + (input_limb_9_col10)),
            ];
            let y_sum_tmp_715f4_15 = [
                ((unpacked_limb_0_col11) + (unpacked_tmp_715f4_1.get_m31(14))),
                ((unpacked_limb_1_col12) + (unpacked_limb_15_col21)),
                ((unpacked_tmp_715f4_1.get_m31(2)) + (unpacked_limb_16_col22)),
                ((unpacked_limb_3_col13) + (unpacked_tmp_715f4_1.get_m31(17))),
                ((unpacked_limb_4_col14) + (unpacked_limb_18_col23)),
                ((unpacked_tmp_715f4_1.get_m31(5)) + (unpacked_limb_19_col24)),
                ((unpacked_limb_6_col15) + (unpacked_tmp_715f4_1.get_m31(20))),
                ((unpacked_limb_7_col16) + (unpacked_limb_21_col25)),
                ((unpacked_tmp_715f4_1.get_m31(8)) + (unpacked_limb_22_col26)),
                ((unpacked_limb_9_col17) + (unpacked_tmp_715f4_1.get_m31(23))),
                ((unpacked_limb_10_col18) + (unpacked_limb_24_col27)),
                ((unpacked_tmp_715f4_1.get_m31(11)) + (unpacked_limb_25_col28)),
                ((unpacked_limb_12_col19) + (unpacked_tmp_715f4_1.get_m31(26))),
                ((unpacked_limb_13_col20) + (input_limb_9_col10)),
            ];

            // Single Karatsuba N 7.

            let z0_tmp_715f4_16 = [
                ((x_sum_tmp_715f4_14[0]) * (y_sum_tmp_715f4_15[0])),
                (((x_sum_tmp_715f4_14[0]) * (y_sum_tmp_715f4_15[1]))
                    + ((x_sum_tmp_715f4_14[1]) * (y_sum_tmp_715f4_15[0]))),
                ((((x_sum_tmp_715f4_14[0]) * (y_sum_tmp_715f4_15[2]))
                    + ((x_sum_tmp_715f4_14[1]) * (y_sum_tmp_715f4_15[1])))
                    + ((x_sum_tmp_715f4_14[2]) * (y_sum_tmp_715f4_15[0]))),
                (((((x_sum_tmp_715f4_14[0]) * (y_sum_tmp_715f4_15[3]))
                    + ((x_sum_tmp_715f4_14[1]) * (y_sum_tmp_715f4_15[2])))
                    + ((x_sum_tmp_715f4_14[2]) * (y_sum_tmp_715f4_15[1])))
                    + ((x_sum_tmp_715f4_14[3]) * (y_sum_tmp_715f4_15[0]))),
                ((((((x_sum_tmp_715f4_14[0]) * (y_sum_tmp_715f4_15[4]))
                    + ((x_sum_tmp_715f4_14[1]) * (y_sum_tmp_715f4_15[3])))
                    + ((x_sum_tmp_715f4_14[2]) * (y_sum_tmp_715f4_15[2])))
                    + ((x_sum_tmp_715f4_14[3]) * (y_sum_tmp_715f4_15[1])))
                    + ((x_sum_tmp_715f4_14[4]) * (y_sum_tmp_715f4_15[0]))),
                (((((((x_sum_tmp_715f4_14[0]) * (y_sum_tmp_715f4_15[5]))
                    + ((x_sum_tmp_715f4_14[1]) * (y_sum_tmp_715f4_15[4])))
                    + ((x_sum_tmp_715f4_14[2]) * (y_sum_tmp_715f4_15[3])))
                    + ((x_sum_tmp_715f4_14[3]) * (y_sum_tmp_715f4_15[2])))
                    + ((x_sum_tmp_715f4_14[4]) * (y_sum_tmp_715f4_15[1])))
                    + ((x_sum_tmp_715f4_14[5]) * (y_sum_tmp_715f4_15[0]))),
                ((((((((x_sum_tmp_715f4_14[0]) * (y_sum_tmp_715f4_15[6]))
                    + ((x_sum_tmp_715f4_14[1]) * (y_sum_tmp_715f4_15[5])))
                    + ((x_sum_tmp_715f4_14[2]) * (y_sum_tmp_715f4_15[4])))
                    + ((x_sum_tmp_715f4_14[3]) * (y_sum_tmp_715f4_15[3])))
                    + ((x_sum_tmp_715f4_14[4]) * (y_sum_tmp_715f4_15[2])))
                    + ((x_sum_tmp_715f4_14[5]) * (y_sum_tmp_715f4_15[1])))
                    + ((x_sum_tmp_715f4_14[6]) * (y_sum_tmp_715f4_15[0]))),
                (((((((x_sum_tmp_715f4_14[1]) * (y_sum_tmp_715f4_15[6]))
                    + ((x_sum_tmp_715f4_14[2]) * (y_sum_tmp_715f4_15[5])))
                    + ((x_sum_tmp_715f4_14[3]) * (y_sum_tmp_715f4_15[4])))
                    + ((x_sum_tmp_715f4_14[4]) * (y_sum_tmp_715f4_15[3])))
                    + ((x_sum_tmp_715f4_14[5]) * (y_sum_tmp_715f4_15[2])))
                    + ((x_sum_tmp_715f4_14[6]) * (y_sum_tmp_715f4_15[1]))),
                ((((((x_sum_tmp_715f4_14[2]) * (y_sum_tmp_715f4_15[6]))
                    + ((x_sum_tmp_715f4_14[3]) * (y_sum_tmp_715f4_15[5])))
                    + ((x_sum_tmp_715f4_14[4]) * (y_sum_tmp_715f4_15[4])))
                    + ((x_sum_tmp_715f4_14[5]) * (y_sum_tmp_715f4_15[3])))
                    + ((x_sum_tmp_715f4_14[6]) * (y_sum_tmp_715f4_15[2]))),
                (((((x_sum_tmp_715f4_14[3]) * (y_sum_tmp_715f4_15[6]))
                    + ((x_sum_tmp_715f4_14[4]) * (y_sum_tmp_715f4_15[5])))
                    + ((x_sum_tmp_715f4_14[5]) * (y_sum_tmp_715f4_15[4])))
                    + ((x_sum_tmp_715f4_14[6]) * (y_sum_tmp_715f4_15[3]))),
                ((((x_sum_tmp_715f4_14[4]) * (y_sum_tmp_715f4_15[6]))
                    + ((x_sum_tmp_715f4_14[5]) * (y_sum_tmp_715f4_15[5])))
                    + ((x_sum_tmp_715f4_14[6]) * (y_sum_tmp_715f4_15[4]))),
                (((x_sum_tmp_715f4_14[5]) * (y_sum_tmp_715f4_15[6]))
                    + ((x_sum_tmp_715f4_14[6]) * (y_sum_tmp_715f4_15[5]))),
                ((x_sum_tmp_715f4_14[6]) * (y_sum_tmp_715f4_15[6])),
            ];
            let z2_tmp_715f4_17 = [
                ((x_sum_tmp_715f4_14[7]) * (y_sum_tmp_715f4_15[7])),
                (((x_sum_tmp_715f4_14[7]) * (y_sum_tmp_715f4_15[8]))
                    + ((x_sum_tmp_715f4_14[8]) * (y_sum_tmp_715f4_15[7]))),
                ((((x_sum_tmp_715f4_14[7]) * (y_sum_tmp_715f4_15[9]))
                    + ((x_sum_tmp_715f4_14[8]) * (y_sum_tmp_715f4_15[8])))
                    + ((x_sum_tmp_715f4_14[9]) * (y_sum_tmp_715f4_15[7]))),
                (((((x_sum_tmp_715f4_14[7]) * (y_sum_tmp_715f4_15[10]))
                    + ((x_sum_tmp_715f4_14[8]) * (y_sum_tmp_715f4_15[9])))
                    + ((x_sum_tmp_715f4_14[9]) * (y_sum_tmp_715f4_15[8])))
                    + ((x_sum_tmp_715f4_14[10]) * (y_sum_tmp_715f4_15[7]))),
                ((((((x_sum_tmp_715f4_14[7]) * (y_sum_tmp_715f4_15[11]))
                    + ((x_sum_tmp_715f4_14[8]) * (y_sum_tmp_715f4_15[10])))
                    + ((x_sum_tmp_715f4_14[9]) * (y_sum_tmp_715f4_15[9])))
                    + ((x_sum_tmp_715f4_14[10]) * (y_sum_tmp_715f4_15[8])))
                    + ((x_sum_tmp_715f4_14[11]) * (y_sum_tmp_715f4_15[7]))),
                (((((((x_sum_tmp_715f4_14[7]) * (y_sum_tmp_715f4_15[12]))
                    + ((x_sum_tmp_715f4_14[8]) * (y_sum_tmp_715f4_15[11])))
                    + ((x_sum_tmp_715f4_14[9]) * (y_sum_tmp_715f4_15[10])))
                    + ((x_sum_tmp_715f4_14[10]) * (y_sum_tmp_715f4_15[9])))
                    + ((x_sum_tmp_715f4_14[11]) * (y_sum_tmp_715f4_15[8])))
                    + ((x_sum_tmp_715f4_14[12]) * (y_sum_tmp_715f4_15[7]))),
                ((((((((x_sum_tmp_715f4_14[7]) * (y_sum_tmp_715f4_15[13]))
                    + ((x_sum_tmp_715f4_14[8]) * (y_sum_tmp_715f4_15[12])))
                    + ((x_sum_tmp_715f4_14[9]) * (y_sum_tmp_715f4_15[11])))
                    + ((x_sum_tmp_715f4_14[10]) * (y_sum_tmp_715f4_15[10])))
                    + ((x_sum_tmp_715f4_14[11]) * (y_sum_tmp_715f4_15[9])))
                    + ((x_sum_tmp_715f4_14[12]) * (y_sum_tmp_715f4_15[8])))
                    + ((x_sum_tmp_715f4_14[13]) * (y_sum_tmp_715f4_15[7]))),
                (((((((x_sum_tmp_715f4_14[8]) * (y_sum_tmp_715f4_15[13]))
                    + ((x_sum_tmp_715f4_14[9]) * (y_sum_tmp_715f4_15[12])))
                    + ((x_sum_tmp_715f4_14[10]) * (y_sum_tmp_715f4_15[11])))
                    + ((x_sum_tmp_715f4_14[11]) * (y_sum_tmp_715f4_15[10])))
                    + ((x_sum_tmp_715f4_14[12]) * (y_sum_tmp_715f4_15[9])))
                    + ((x_sum_tmp_715f4_14[13]) * (y_sum_tmp_715f4_15[8]))),
                ((((((x_sum_tmp_715f4_14[9]) * (y_sum_tmp_715f4_15[13]))
                    + ((x_sum_tmp_715f4_14[10]) * (y_sum_tmp_715f4_15[12])))
                    + ((x_sum_tmp_715f4_14[11]) * (y_sum_tmp_715f4_15[11])))
                    + ((x_sum_tmp_715f4_14[12]) * (y_sum_tmp_715f4_15[10])))
                    + ((x_sum_tmp_715f4_14[13]) * (y_sum_tmp_715f4_15[9]))),
                (((((x_sum_tmp_715f4_14[10]) * (y_sum_tmp_715f4_15[13]))
                    + ((x_sum_tmp_715f4_14[11]) * (y_sum_tmp_715f4_15[12])))
                    + ((x_sum_tmp_715f4_14[12]) * (y_sum_tmp_715f4_15[11])))
                    + ((x_sum_tmp_715f4_14[13]) * (y_sum_tmp_715f4_15[10]))),
                ((((x_sum_tmp_715f4_14[11]) * (y_sum_tmp_715f4_15[13]))
                    + ((x_sum_tmp_715f4_14[12]) * (y_sum_tmp_715f4_15[12])))
                    + ((x_sum_tmp_715f4_14[13]) * (y_sum_tmp_715f4_15[11]))),
                (((x_sum_tmp_715f4_14[12]) * (y_sum_tmp_715f4_15[13]))
                    + ((x_sum_tmp_715f4_14[13]) * (y_sum_tmp_715f4_15[12]))),
                ((x_sum_tmp_715f4_14[13]) * (y_sum_tmp_715f4_15[13])),
            ];
            let x_sum_tmp_715f4_18 = [
                ((x_sum_tmp_715f4_14[0]) + (x_sum_tmp_715f4_14[7])),
                ((x_sum_tmp_715f4_14[1]) + (x_sum_tmp_715f4_14[8])),
                ((x_sum_tmp_715f4_14[2]) + (x_sum_tmp_715f4_14[9])),
                ((x_sum_tmp_715f4_14[3]) + (x_sum_tmp_715f4_14[10])),
                ((x_sum_tmp_715f4_14[4]) + (x_sum_tmp_715f4_14[11])),
                ((x_sum_tmp_715f4_14[5]) + (x_sum_tmp_715f4_14[12])),
                ((x_sum_tmp_715f4_14[6]) + (x_sum_tmp_715f4_14[13])),
            ];
            let y_sum_tmp_715f4_19 = [
                ((y_sum_tmp_715f4_15[0]) + (y_sum_tmp_715f4_15[7])),
                ((y_sum_tmp_715f4_15[1]) + (y_sum_tmp_715f4_15[8])),
                ((y_sum_tmp_715f4_15[2]) + (y_sum_tmp_715f4_15[9])),
                ((y_sum_tmp_715f4_15[3]) + (y_sum_tmp_715f4_15[10])),
                ((y_sum_tmp_715f4_15[4]) + (y_sum_tmp_715f4_15[11])),
                ((y_sum_tmp_715f4_15[5]) + (y_sum_tmp_715f4_15[12])),
                ((y_sum_tmp_715f4_15[6]) + (y_sum_tmp_715f4_15[13])),
            ];
            let single_karatsuba_n_7_output_tmp_715f4_20 = [
                z0_tmp_715f4_16[0],
                z0_tmp_715f4_16[1],
                z0_tmp_715f4_16[2],
                z0_tmp_715f4_16[3],
                z0_tmp_715f4_16[4],
                z0_tmp_715f4_16[5],
                z0_tmp_715f4_16[6],
                ((z0_tmp_715f4_16[7])
                    + ((((x_sum_tmp_715f4_18[0]) * (y_sum_tmp_715f4_19[0]))
                        - (z0_tmp_715f4_16[0]))
                        - (z2_tmp_715f4_17[0]))),
                ((z0_tmp_715f4_16[8])
                    + (((((x_sum_tmp_715f4_18[0]) * (y_sum_tmp_715f4_19[1]))
                        + ((x_sum_tmp_715f4_18[1]) * (y_sum_tmp_715f4_19[0])))
                        - (z0_tmp_715f4_16[1]))
                        - (z2_tmp_715f4_17[1]))),
                ((z0_tmp_715f4_16[9])
                    + ((((((x_sum_tmp_715f4_18[0]) * (y_sum_tmp_715f4_19[2]))
                        + ((x_sum_tmp_715f4_18[1]) * (y_sum_tmp_715f4_19[1])))
                        + ((x_sum_tmp_715f4_18[2]) * (y_sum_tmp_715f4_19[0])))
                        - (z0_tmp_715f4_16[2]))
                        - (z2_tmp_715f4_17[2]))),
                ((z0_tmp_715f4_16[10])
                    + (((((((x_sum_tmp_715f4_18[0]) * (y_sum_tmp_715f4_19[3]))
                        + ((x_sum_tmp_715f4_18[1]) * (y_sum_tmp_715f4_19[2])))
                        + ((x_sum_tmp_715f4_18[2]) * (y_sum_tmp_715f4_19[1])))
                        + ((x_sum_tmp_715f4_18[3]) * (y_sum_tmp_715f4_19[0])))
                        - (z0_tmp_715f4_16[3]))
                        - (z2_tmp_715f4_17[3]))),
                ((z0_tmp_715f4_16[11])
                    + ((((((((x_sum_tmp_715f4_18[0]) * (y_sum_tmp_715f4_19[4]))
                        + ((x_sum_tmp_715f4_18[1]) * (y_sum_tmp_715f4_19[3])))
                        + ((x_sum_tmp_715f4_18[2]) * (y_sum_tmp_715f4_19[2])))
                        + ((x_sum_tmp_715f4_18[3]) * (y_sum_tmp_715f4_19[1])))
                        + ((x_sum_tmp_715f4_18[4]) * (y_sum_tmp_715f4_19[0])))
                        - (z0_tmp_715f4_16[4]))
                        - (z2_tmp_715f4_17[4]))),
                ((z0_tmp_715f4_16[12])
                    + (((((((((x_sum_tmp_715f4_18[0]) * (y_sum_tmp_715f4_19[5]))
                        + ((x_sum_tmp_715f4_18[1]) * (y_sum_tmp_715f4_19[4])))
                        + ((x_sum_tmp_715f4_18[2]) * (y_sum_tmp_715f4_19[3])))
                        + ((x_sum_tmp_715f4_18[3]) * (y_sum_tmp_715f4_19[2])))
                        + ((x_sum_tmp_715f4_18[4]) * (y_sum_tmp_715f4_19[1])))
                        + ((x_sum_tmp_715f4_18[5]) * (y_sum_tmp_715f4_19[0])))
                        - (z0_tmp_715f4_16[5]))
                        - (z2_tmp_715f4_17[5]))),
                ((((((((((x_sum_tmp_715f4_18[0]) * (y_sum_tmp_715f4_19[6]))
                    + ((x_sum_tmp_715f4_18[1]) * (y_sum_tmp_715f4_19[5])))
                    + ((x_sum_tmp_715f4_18[2]) * (y_sum_tmp_715f4_19[4])))
                    + ((x_sum_tmp_715f4_18[3]) * (y_sum_tmp_715f4_19[3])))
                    + ((x_sum_tmp_715f4_18[4]) * (y_sum_tmp_715f4_19[2])))
                    + ((x_sum_tmp_715f4_18[5]) * (y_sum_tmp_715f4_19[1])))
                    + ((x_sum_tmp_715f4_18[6]) * (y_sum_tmp_715f4_19[0])))
                    - (z0_tmp_715f4_16[6]))
                    - (z2_tmp_715f4_17[6])),
                ((z2_tmp_715f4_17[0])
                    + (((((((((x_sum_tmp_715f4_18[1]) * (y_sum_tmp_715f4_19[6]))
                        + ((x_sum_tmp_715f4_18[2]) * (y_sum_tmp_715f4_19[5])))
                        + ((x_sum_tmp_715f4_18[3]) * (y_sum_tmp_715f4_19[4])))
                        + ((x_sum_tmp_715f4_18[4]) * (y_sum_tmp_715f4_19[3])))
                        + ((x_sum_tmp_715f4_18[5]) * (y_sum_tmp_715f4_19[2])))
                        + ((x_sum_tmp_715f4_18[6]) * (y_sum_tmp_715f4_19[1])))
                        - (z0_tmp_715f4_16[7]))
                        - (z2_tmp_715f4_17[7]))),
                ((z2_tmp_715f4_17[1])
                    + ((((((((x_sum_tmp_715f4_18[2]) * (y_sum_tmp_715f4_19[6]))
                        + ((x_sum_tmp_715f4_18[3]) * (y_sum_tmp_715f4_19[5])))
                        + ((x_sum_tmp_715f4_18[4]) * (y_sum_tmp_715f4_19[4])))
                        + ((x_sum_tmp_715f4_18[5]) * (y_sum_tmp_715f4_19[3])))
                        + ((x_sum_tmp_715f4_18[6]) * (y_sum_tmp_715f4_19[2])))
                        - (z0_tmp_715f4_16[8]))
                        - (z2_tmp_715f4_17[8]))),
                ((z2_tmp_715f4_17[2])
                    + (((((((x_sum_tmp_715f4_18[3]) * (y_sum_tmp_715f4_19[6]))
                        + ((x_sum_tmp_715f4_18[4]) * (y_sum_tmp_715f4_19[5])))
                        + ((x_sum_tmp_715f4_18[5]) * (y_sum_tmp_715f4_19[4])))
                        + ((x_sum_tmp_715f4_18[6]) * (y_sum_tmp_715f4_19[3])))
                        - (z0_tmp_715f4_16[9]))
                        - (z2_tmp_715f4_17[9]))),
                ((z2_tmp_715f4_17[3])
                    + ((((((x_sum_tmp_715f4_18[4]) * (y_sum_tmp_715f4_19[6]))
                        + ((x_sum_tmp_715f4_18[5]) * (y_sum_tmp_715f4_19[5])))
                        + ((x_sum_tmp_715f4_18[6]) * (y_sum_tmp_715f4_19[4])))
                        - (z0_tmp_715f4_16[10]))
                        - (z2_tmp_715f4_17[10]))),
                ((z2_tmp_715f4_17[4])
                    + (((((x_sum_tmp_715f4_18[5]) * (y_sum_tmp_715f4_19[6]))
                        + ((x_sum_tmp_715f4_18[6]) * (y_sum_tmp_715f4_19[5])))
                        - (z0_tmp_715f4_16[11]))
                        - (z2_tmp_715f4_17[11]))),
                ((z2_tmp_715f4_17[5])
                    + ((((x_sum_tmp_715f4_18[6]) * (y_sum_tmp_715f4_19[6]))
                        - (z0_tmp_715f4_16[12]))
                        - (z2_tmp_715f4_17[12]))),
                z2_tmp_715f4_17[6],
                z2_tmp_715f4_17[7],
                z2_tmp_715f4_17[8],
                z2_tmp_715f4_17[9],
                z2_tmp_715f4_17[10],
                z2_tmp_715f4_17[11],
                z2_tmp_715f4_17[12],
            ];

            let double_karatsuba_f0fc6_output_tmp_715f4_21 = [
                single_karatsuba_n_7_output_tmp_715f4_8[0],
                single_karatsuba_n_7_output_tmp_715f4_8[1],
                single_karatsuba_n_7_output_tmp_715f4_8[2],
                single_karatsuba_n_7_output_tmp_715f4_8[3],
                single_karatsuba_n_7_output_tmp_715f4_8[4],
                single_karatsuba_n_7_output_tmp_715f4_8[5],
                single_karatsuba_n_7_output_tmp_715f4_8[6],
                single_karatsuba_n_7_output_tmp_715f4_8[7],
                single_karatsuba_n_7_output_tmp_715f4_8[8],
                single_karatsuba_n_7_output_tmp_715f4_8[9],
                single_karatsuba_n_7_output_tmp_715f4_8[10],
                single_karatsuba_n_7_output_tmp_715f4_8[11],
                single_karatsuba_n_7_output_tmp_715f4_8[12],
                single_karatsuba_n_7_output_tmp_715f4_8[13],
                ((single_karatsuba_n_7_output_tmp_715f4_8[14])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[0])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[0]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[0]))),
                ((single_karatsuba_n_7_output_tmp_715f4_8[15])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[1])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[1]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[1]))),
                ((single_karatsuba_n_7_output_tmp_715f4_8[16])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[2])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[2]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[2]))),
                ((single_karatsuba_n_7_output_tmp_715f4_8[17])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[3])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[3]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[3]))),
                ((single_karatsuba_n_7_output_tmp_715f4_8[18])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[4])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[4]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[4]))),
                ((single_karatsuba_n_7_output_tmp_715f4_8[19])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[5])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[5]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[5]))),
                ((single_karatsuba_n_7_output_tmp_715f4_8[20])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[6])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[6]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[6]))),
                ((single_karatsuba_n_7_output_tmp_715f4_8[21])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[7])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[7]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[7]))),
                ((single_karatsuba_n_7_output_tmp_715f4_8[22])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[8])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[8]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[8]))),
                ((single_karatsuba_n_7_output_tmp_715f4_8[23])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[9])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[9]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[9]))),
                ((single_karatsuba_n_7_output_tmp_715f4_8[24])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[10])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[10]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[10]))),
                ((single_karatsuba_n_7_output_tmp_715f4_8[25])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[11])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[11]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[11]))),
                ((single_karatsuba_n_7_output_tmp_715f4_8[26])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[12])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[12]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[12]))),
                (((single_karatsuba_n_7_output_tmp_715f4_20[13])
                    - (single_karatsuba_n_7_output_tmp_715f4_8[13]))
                    - (single_karatsuba_n_7_output_tmp_715f4_13[13])),
                ((single_karatsuba_n_7_output_tmp_715f4_13[0])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[14])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[14]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[14]))),
                ((single_karatsuba_n_7_output_tmp_715f4_13[1])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[15])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[15]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[15]))),
                ((single_karatsuba_n_7_output_tmp_715f4_13[2])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[16])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[16]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[16]))),
                ((single_karatsuba_n_7_output_tmp_715f4_13[3])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[17])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[17]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[17]))),
                ((single_karatsuba_n_7_output_tmp_715f4_13[4])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[18])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[18]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[18]))),
                ((single_karatsuba_n_7_output_tmp_715f4_13[5])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[19])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[19]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[19]))),
                ((single_karatsuba_n_7_output_tmp_715f4_13[6])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[20])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[20]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[20]))),
                ((single_karatsuba_n_7_output_tmp_715f4_13[7])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[21])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[21]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[21]))),
                ((single_karatsuba_n_7_output_tmp_715f4_13[8])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[22])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[22]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[22]))),
                ((single_karatsuba_n_7_output_tmp_715f4_13[9])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[23])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[23]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[23]))),
                ((single_karatsuba_n_7_output_tmp_715f4_13[10])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[24])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[24]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[24]))),
                ((single_karatsuba_n_7_output_tmp_715f4_13[11])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[25])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[25]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[25]))),
                ((single_karatsuba_n_7_output_tmp_715f4_13[12])
                    + (((single_karatsuba_n_7_output_tmp_715f4_20[26])
                        - (single_karatsuba_n_7_output_tmp_715f4_8[26]))
                        - (single_karatsuba_n_7_output_tmp_715f4_13[26]))),
                single_karatsuba_n_7_output_tmp_715f4_13[13],
                single_karatsuba_n_7_output_tmp_715f4_13[14],
                single_karatsuba_n_7_output_tmp_715f4_13[15],
                single_karatsuba_n_7_output_tmp_715f4_13[16],
                single_karatsuba_n_7_output_tmp_715f4_13[17],
                single_karatsuba_n_7_output_tmp_715f4_13[18],
                single_karatsuba_n_7_output_tmp_715f4_13[19],
                single_karatsuba_n_7_output_tmp_715f4_13[20],
                single_karatsuba_n_7_output_tmp_715f4_13[21],
                single_karatsuba_n_7_output_tmp_715f4_13[22],
                single_karatsuba_n_7_output_tmp_715f4_13[23],
                single_karatsuba_n_7_output_tmp_715f4_13[24],
                single_karatsuba_n_7_output_tmp_715f4_13[25],
                single_karatsuba_n_7_output_tmp_715f4_13[26],
            ];

            let conv_tmp_715f4_22 = [
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[0]) - (mul_res_limb_0_col29)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[1]) - (mul_res_limb_1_col30)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[2]) - (mul_res_limb_2_col31)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[3]) - (mul_res_limb_3_col32)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[4]) - (mul_res_limb_4_col33)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[5]) - (mul_res_limb_5_col34)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[6]) - (mul_res_limb_6_col35)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[7]) - (mul_res_limb_7_col36)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[8]) - (mul_res_limb_8_col37)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[9]) - (mul_res_limb_9_col38)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[10]) - (mul_res_limb_10_col39)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[11]) - (mul_res_limb_11_col40)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[12]) - (mul_res_limb_12_col41)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[13]) - (mul_res_limb_13_col42)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[14]) - (mul_res_limb_14_col43)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[15]) - (mul_res_limb_15_col44)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[16]) - (mul_res_limb_16_col45)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[17]) - (mul_res_limb_17_col46)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[18]) - (mul_res_limb_18_col47)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[19]) - (mul_res_limb_19_col48)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[20]) - (mul_res_limb_20_col49)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[21]) - (mul_res_limb_21_col50)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[22]) - (mul_res_limb_22_col51)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[23]) - (mul_res_limb_23_col52)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[24]) - (mul_res_limb_24_col53)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[25]) - (mul_res_limb_25_col54)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[26]) - (mul_res_limb_26_col55)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_21[27]) - (mul_res_limb_27_col56)),
                double_karatsuba_f0fc6_output_tmp_715f4_21[28],
                double_karatsuba_f0fc6_output_tmp_715f4_21[29],
                double_karatsuba_f0fc6_output_tmp_715f4_21[30],
                double_karatsuba_f0fc6_output_tmp_715f4_21[31],
                double_karatsuba_f0fc6_output_tmp_715f4_21[32],
                double_karatsuba_f0fc6_output_tmp_715f4_21[33],
                double_karatsuba_f0fc6_output_tmp_715f4_21[34],
                double_karatsuba_f0fc6_output_tmp_715f4_21[35],
                double_karatsuba_f0fc6_output_tmp_715f4_21[36],
                double_karatsuba_f0fc6_output_tmp_715f4_21[37],
                double_karatsuba_f0fc6_output_tmp_715f4_21[38],
                double_karatsuba_f0fc6_output_tmp_715f4_21[39],
                double_karatsuba_f0fc6_output_tmp_715f4_21[40],
                double_karatsuba_f0fc6_output_tmp_715f4_21[41],
                double_karatsuba_f0fc6_output_tmp_715f4_21[42],
                double_karatsuba_f0fc6_output_tmp_715f4_21[43],
                double_karatsuba_f0fc6_output_tmp_715f4_21[44],
                double_karatsuba_f0fc6_output_tmp_715f4_21[45],
                double_karatsuba_f0fc6_output_tmp_715f4_21[46],
                double_karatsuba_f0fc6_output_tmp_715f4_21[47],
                double_karatsuba_f0fc6_output_tmp_715f4_21[48],
                double_karatsuba_f0fc6_output_tmp_715f4_21[49],
                double_karatsuba_f0fc6_output_tmp_715f4_21[50],
                double_karatsuba_f0fc6_output_tmp_715f4_21[51],
                double_karatsuba_f0fc6_output_tmp_715f4_21[52],
                double_karatsuba_f0fc6_output_tmp_715f4_21[53],
                double_karatsuba_f0fc6_output_tmp_715f4_21[54],
            ];
            let conv_mod_tmp_715f4_23 = [
                ((((M31_32) * (conv_tmp_715f4_22[0])) - ((M31_4) * (conv_tmp_715f4_22[21])))
                    + ((M31_8) * (conv_tmp_715f4_22[49]))),
                ((((conv_tmp_715f4_22[0]) + ((M31_32) * (conv_tmp_715f4_22[1])))
                    - ((M31_4) * (conv_tmp_715f4_22[22])))
                    + ((M31_8) * (conv_tmp_715f4_22[50]))),
                ((((conv_tmp_715f4_22[1]) + ((M31_32) * (conv_tmp_715f4_22[2])))
                    - ((M31_4) * (conv_tmp_715f4_22[23])))
                    + ((M31_8) * (conv_tmp_715f4_22[51]))),
                ((((conv_tmp_715f4_22[2]) + ((M31_32) * (conv_tmp_715f4_22[3])))
                    - ((M31_4) * (conv_tmp_715f4_22[24])))
                    + ((M31_8) * (conv_tmp_715f4_22[52]))),
                ((((conv_tmp_715f4_22[3]) + ((M31_32) * (conv_tmp_715f4_22[4])))
                    - ((M31_4) * (conv_tmp_715f4_22[25])))
                    + ((M31_8) * (conv_tmp_715f4_22[53]))),
                ((((conv_tmp_715f4_22[4]) + ((M31_32) * (conv_tmp_715f4_22[5])))
                    - ((M31_4) * (conv_tmp_715f4_22[26])))
                    + ((M31_8) * (conv_tmp_715f4_22[54]))),
                (((conv_tmp_715f4_22[5]) + ((M31_32) * (conv_tmp_715f4_22[6])))
                    - ((M31_4) * (conv_tmp_715f4_22[27]))),
                (((((M31_2) * (conv_tmp_715f4_22[0])) + (conv_tmp_715f4_22[6]))
                    + ((M31_32) * (conv_tmp_715f4_22[7])))
                    - ((M31_4) * (conv_tmp_715f4_22[28]))),
                (((((M31_2) * (conv_tmp_715f4_22[1])) + (conv_tmp_715f4_22[7]))
                    + ((M31_32) * (conv_tmp_715f4_22[8])))
                    - ((M31_4) * (conv_tmp_715f4_22[29]))),
                (((((M31_2) * (conv_tmp_715f4_22[2])) + (conv_tmp_715f4_22[8]))
                    + ((M31_32) * (conv_tmp_715f4_22[9])))
                    - ((M31_4) * (conv_tmp_715f4_22[30]))),
                (((((M31_2) * (conv_tmp_715f4_22[3])) + (conv_tmp_715f4_22[9]))
                    + ((M31_32) * (conv_tmp_715f4_22[10])))
                    - ((M31_4) * (conv_tmp_715f4_22[31]))),
                (((((M31_2) * (conv_tmp_715f4_22[4])) + (conv_tmp_715f4_22[10]))
                    + ((M31_32) * (conv_tmp_715f4_22[11])))
                    - ((M31_4) * (conv_tmp_715f4_22[32]))),
                (((((M31_2) * (conv_tmp_715f4_22[5])) + (conv_tmp_715f4_22[11]))
                    + ((M31_32) * (conv_tmp_715f4_22[12])))
                    - ((M31_4) * (conv_tmp_715f4_22[33]))),
                (((((M31_2) * (conv_tmp_715f4_22[6])) + (conv_tmp_715f4_22[12]))
                    + ((M31_32) * (conv_tmp_715f4_22[13])))
                    - ((M31_4) * (conv_tmp_715f4_22[34]))),
                (((((M31_2) * (conv_tmp_715f4_22[7])) + (conv_tmp_715f4_22[13]))
                    + ((M31_32) * (conv_tmp_715f4_22[14])))
                    - ((M31_4) * (conv_tmp_715f4_22[35]))),
                (((((M31_2) * (conv_tmp_715f4_22[8])) + (conv_tmp_715f4_22[14]))
                    + ((M31_32) * (conv_tmp_715f4_22[15])))
                    - ((M31_4) * (conv_tmp_715f4_22[36]))),
                (((((M31_2) * (conv_tmp_715f4_22[9])) + (conv_tmp_715f4_22[15]))
                    + ((M31_32) * (conv_tmp_715f4_22[16])))
                    - ((M31_4) * (conv_tmp_715f4_22[37]))),
                (((((M31_2) * (conv_tmp_715f4_22[10])) + (conv_tmp_715f4_22[16]))
                    + ((M31_32) * (conv_tmp_715f4_22[17])))
                    - ((M31_4) * (conv_tmp_715f4_22[38]))),
                (((((M31_2) * (conv_tmp_715f4_22[11])) + (conv_tmp_715f4_22[17]))
                    + ((M31_32) * (conv_tmp_715f4_22[18])))
                    - ((M31_4) * (conv_tmp_715f4_22[39]))),
                (((((M31_2) * (conv_tmp_715f4_22[12])) + (conv_tmp_715f4_22[18]))
                    + ((M31_32) * (conv_tmp_715f4_22[19])))
                    - ((M31_4) * (conv_tmp_715f4_22[40]))),
                (((((M31_2) * (conv_tmp_715f4_22[13])) + (conv_tmp_715f4_22[19]))
                    + ((M31_32) * (conv_tmp_715f4_22[20])))
                    - ((M31_4) * (conv_tmp_715f4_22[41]))),
                (((((M31_2) * (conv_tmp_715f4_22[14])) + (conv_tmp_715f4_22[20]))
                    - ((M31_4) * (conv_tmp_715f4_22[42])))
                    + ((M31_64) * (conv_tmp_715f4_22[49]))),
                (((((M31_2) * (conv_tmp_715f4_22[15])) - ((M31_4) * (conv_tmp_715f4_22[43])))
                    + ((M31_2) * (conv_tmp_715f4_22[49])))
                    + ((M31_64) * (conv_tmp_715f4_22[50]))),
                (((((M31_2) * (conv_tmp_715f4_22[16])) - ((M31_4) * (conv_tmp_715f4_22[44])))
                    + ((M31_2) * (conv_tmp_715f4_22[50])))
                    + ((M31_64) * (conv_tmp_715f4_22[51]))),
                (((((M31_2) * (conv_tmp_715f4_22[17])) - ((M31_4) * (conv_tmp_715f4_22[45])))
                    + ((M31_2) * (conv_tmp_715f4_22[51])))
                    + ((M31_64) * (conv_tmp_715f4_22[52]))),
                (((((M31_2) * (conv_tmp_715f4_22[18])) - ((M31_4) * (conv_tmp_715f4_22[46])))
                    + ((M31_2) * (conv_tmp_715f4_22[52])))
                    + ((M31_64) * (conv_tmp_715f4_22[53]))),
                (((((M31_2) * (conv_tmp_715f4_22[19])) - ((M31_4) * (conv_tmp_715f4_22[47])))
                    + ((M31_2) * (conv_tmp_715f4_22[53])))
                    + ((M31_64) * (conv_tmp_715f4_22[54]))),
                ((((M31_2) * (conv_tmp_715f4_22[20])) - ((M31_4) * (conv_tmp_715f4_22[48])))
                    + ((M31_2) * (conv_tmp_715f4_22[54]))),
            ];
            let k_mod_2_18_biased_tmp_715f4_24 =
                ((((PackedUInt32::from_m31(((conv_mod_tmp_715f4_23[0]) + (M31_134217728))))
                    + (((PackedUInt32::from_m31(
                        ((conv_mod_tmp_715f4_23[1]) + (M31_134217728)),
                    )) & (UInt32_511))
                        << (UInt32_9)))
                    + (UInt32_131072))
                    & (UInt32_262143));
            let k_col57 = ((k_mod_2_18_biased_tmp_715f4_24.low().as_m31())
                + (((k_mod_2_18_biased_tmp_715f4_24.high().as_m31()) - (M31_2)) * (M31_65536)));
            *row[57] = k_col57;
            *sub_component_inputs.range_check_20[0] = [((k_col57) + (M31_524288))];
            *lookup_data.range_check_20_28 = [M31_1410849886, ((k_col57) + (M31_524288))];
            let carry_0_col58 = (((conv_mod_tmp_715f4_23[0]) - (k_col57)) * (M31_4194304));
            *row[58] = carry_0_col58;
            *sub_component_inputs.range_check_20_b[0] = [((carry_0_col58) + (M31_524288))];
            *lookup_data.range_check_20_b_29 = [M31_514232941, ((carry_0_col58) + (M31_524288))];
            let carry_1_col59 = (((conv_mod_tmp_715f4_23[1]) + (carry_0_col58)) * (M31_4194304));
            *row[59] = carry_1_col59;
            *sub_component_inputs.range_check_20_c[0] = [((carry_1_col59) + (M31_524288))];
            *lookup_data.range_check_20_c_30 = [M31_531010560, ((carry_1_col59) + (M31_524288))];
            let carry_2_col60 = (((conv_mod_tmp_715f4_23[2]) + (carry_1_col59)) * (M31_4194304));
            *row[60] = carry_2_col60;
            *sub_component_inputs.range_check_20_d[0] = [((carry_2_col60) + (M31_524288))];
            *lookup_data.range_check_20_d_31 = [M31_480677703, ((carry_2_col60) + (M31_524288))];
            let carry_3_col61 = (((conv_mod_tmp_715f4_23[3]) + (carry_2_col60)) * (M31_4194304));
            *row[61] = carry_3_col61;
            *sub_component_inputs.range_check_20_e[0] = [((carry_3_col61) + (M31_524288))];
            *lookup_data.range_check_20_e_32 = [M31_497455322, ((carry_3_col61) + (M31_524288))];
            let carry_4_col62 = (((conv_mod_tmp_715f4_23[4]) + (carry_3_col61)) * (M31_4194304));
            *row[62] = carry_4_col62;
            *sub_component_inputs.range_check_20_f[0] = [((carry_4_col62) + (M31_524288))];
            *lookup_data.range_check_20_f_33 = [M31_447122465, ((carry_4_col62) + (M31_524288))];
            let carry_5_col63 = (((conv_mod_tmp_715f4_23[5]) + (carry_4_col62)) * (M31_4194304));
            *row[63] = carry_5_col63;
            *sub_component_inputs.range_check_20_g[0] = [((carry_5_col63) + (M31_524288))];
            *lookup_data.range_check_20_g_34 = [M31_463900084, ((carry_5_col63) + (M31_524288))];
            let carry_6_col64 = (((conv_mod_tmp_715f4_23[6]) + (carry_5_col63)) * (M31_4194304));
            *row[64] = carry_6_col64;
            *sub_component_inputs.range_check_20_h[0] = [((carry_6_col64) + (M31_524288))];
            *lookup_data.range_check_20_h_35 = [M31_682009131, ((carry_6_col64) + (M31_524288))];
            let carry_7_col65 = (((conv_mod_tmp_715f4_23[7]) + (carry_6_col64)) * (M31_4194304));
            *row[65] = carry_7_col65;
            *sub_component_inputs.range_check_20[1] = [((carry_7_col65) + (M31_524288))];
            *lookup_data.range_check_20_36 = [M31_1410849886, ((carry_7_col65) + (M31_524288))];
            let carry_8_col66 = (((conv_mod_tmp_715f4_23[8]) + (carry_7_col65)) * (M31_4194304));
            *row[66] = carry_8_col66;
            *sub_component_inputs.range_check_20_b[1] = [((carry_8_col66) + (M31_524288))];
            *lookup_data.range_check_20_b_37 = [M31_514232941, ((carry_8_col66) + (M31_524288))];
            let carry_9_col67 = (((conv_mod_tmp_715f4_23[9]) + (carry_8_col66)) * (M31_4194304));
            *row[67] = carry_9_col67;
            *sub_component_inputs.range_check_20_c[1] = [((carry_9_col67) + (M31_524288))];
            *lookup_data.range_check_20_c_38 = [M31_531010560, ((carry_9_col67) + (M31_524288))];
            let carry_10_col68 = (((conv_mod_tmp_715f4_23[10]) + (carry_9_col67)) * (M31_4194304));
            *row[68] = carry_10_col68;
            *sub_component_inputs.range_check_20_d[1] = [((carry_10_col68) + (M31_524288))];
            *lookup_data.range_check_20_d_39 = [M31_480677703, ((carry_10_col68) + (M31_524288))];
            let carry_11_col69 = (((conv_mod_tmp_715f4_23[11]) + (carry_10_col68)) * (M31_4194304));
            *row[69] = carry_11_col69;
            *sub_component_inputs.range_check_20_e[1] = [((carry_11_col69) + (M31_524288))];
            *lookup_data.range_check_20_e_40 = [M31_497455322, ((carry_11_col69) + (M31_524288))];
            let carry_12_col70 = (((conv_mod_tmp_715f4_23[12]) + (carry_11_col69)) * (M31_4194304));
            *row[70] = carry_12_col70;
            *sub_component_inputs.range_check_20_f[1] = [((carry_12_col70) + (M31_524288))];
            *lookup_data.range_check_20_f_41 = [M31_447122465, ((carry_12_col70) + (M31_524288))];
            let carry_13_col71 = (((conv_mod_tmp_715f4_23[13]) + (carry_12_col70)) * (M31_4194304));
            *row[71] = carry_13_col71;
            *sub_component_inputs.range_check_20_g[1] = [((carry_13_col71) + (M31_524288))];
            *lookup_data.range_check_20_g_42 = [M31_463900084, ((carry_13_col71) + (M31_524288))];
            let carry_14_col72 = (((conv_mod_tmp_715f4_23[14]) + (carry_13_col71)) * (M31_4194304));
            *row[72] = carry_14_col72;
            *sub_component_inputs.range_check_20_h[1] = [((carry_14_col72) + (M31_524288))];
            *lookup_data.range_check_20_h_43 = [M31_682009131, ((carry_14_col72) + (M31_524288))];
            let carry_15_col73 = (((conv_mod_tmp_715f4_23[15]) + (carry_14_col72)) * (M31_4194304));
            *row[73] = carry_15_col73;
            *sub_component_inputs.range_check_20[2] = [((carry_15_col73) + (M31_524288))];
            *lookup_data.range_check_20_44 = [M31_1410849886, ((carry_15_col73) + (M31_524288))];
            let carry_16_col74 = (((conv_mod_tmp_715f4_23[16]) + (carry_15_col73)) * (M31_4194304));
            *row[74] = carry_16_col74;
            *sub_component_inputs.range_check_20_b[2] = [((carry_16_col74) + (M31_524288))];
            *lookup_data.range_check_20_b_45 = [M31_514232941, ((carry_16_col74) + (M31_524288))];
            let carry_17_col75 = (((conv_mod_tmp_715f4_23[17]) + (carry_16_col74)) * (M31_4194304));
            *row[75] = carry_17_col75;
            *sub_component_inputs.range_check_20_c[2] = [((carry_17_col75) + (M31_524288))];
            *lookup_data.range_check_20_c_46 = [M31_531010560, ((carry_17_col75) + (M31_524288))];
            let carry_18_col76 = (((conv_mod_tmp_715f4_23[18]) + (carry_17_col75)) * (M31_4194304));
            *row[76] = carry_18_col76;
            *sub_component_inputs.range_check_20_d[2] = [((carry_18_col76) + (M31_524288))];
            *lookup_data.range_check_20_d_47 = [M31_480677703, ((carry_18_col76) + (M31_524288))];
            let carry_19_col77 = (((conv_mod_tmp_715f4_23[19]) + (carry_18_col76)) * (M31_4194304));
            *row[77] = carry_19_col77;
            *sub_component_inputs.range_check_20_e[2] = [((carry_19_col77) + (M31_524288))];
            *lookup_data.range_check_20_e_48 = [M31_497455322, ((carry_19_col77) + (M31_524288))];
            let carry_20_col78 = (((conv_mod_tmp_715f4_23[20]) + (carry_19_col77)) * (M31_4194304));
            *row[78] = carry_20_col78;
            *sub_component_inputs.range_check_20_f[2] = [((carry_20_col78) + (M31_524288))];
            *lookup_data.range_check_20_f_49 = [M31_447122465, ((carry_20_col78) + (M31_524288))];
            let carry_21_col79 = ((((conv_mod_tmp_715f4_23[21]) - ((M31_136) * (k_col57)))
                + (carry_20_col78))
                * (M31_4194304));
            *row[79] = carry_21_col79;
            *sub_component_inputs.range_check_20_g[2] = [((carry_21_col79) + (M31_524288))];
            *lookup_data.range_check_20_g_50 = [M31_463900084, ((carry_21_col79) + (M31_524288))];
            let carry_22_col80 = (((conv_mod_tmp_715f4_23[22]) + (carry_21_col79)) * (M31_4194304));
            *row[80] = carry_22_col80;
            *sub_component_inputs.range_check_20_h[2] = [((carry_22_col80) + (M31_524288))];
            *lookup_data.range_check_20_h_51 = [M31_682009131, ((carry_22_col80) + (M31_524288))];
            let carry_23_col81 = (((conv_mod_tmp_715f4_23[23]) + (carry_22_col80)) * (M31_4194304));
            *row[81] = carry_23_col81;
            *sub_component_inputs.range_check_20[3] = [((carry_23_col81) + (M31_524288))];
            *lookup_data.range_check_20_52 = [M31_1410849886, ((carry_23_col81) + (M31_524288))];
            let carry_24_col82 = (((conv_mod_tmp_715f4_23[24]) + (carry_23_col81)) * (M31_4194304));
            *row[82] = carry_24_col82;
            *sub_component_inputs.range_check_20_b[3] = [((carry_24_col82) + (M31_524288))];
            *lookup_data.range_check_20_b_53 = [M31_514232941, ((carry_24_col82) + (M31_524288))];
            let carry_25_col83 = (((conv_mod_tmp_715f4_23[25]) + (carry_24_col82)) * (M31_4194304));
            *row[83] = carry_25_col83;
            *sub_component_inputs.range_check_20_c[3] = [((carry_25_col83) + (M31_524288))];
            *lookup_data.range_check_20_c_54 = [M31_531010560, ((carry_25_col83) + (M31_524288))];
            let carry_26_col84 = (((conv_mod_tmp_715f4_23[26]) + (carry_25_col83)) * (M31_4194304));
            *row[84] = carry_26_col84;
            *sub_component_inputs.range_check_20_d[3] = [((carry_26_col84) + (M31_524288))];
            *lookup_data.range_check_20_d_55 = [M31_480677703, ((carry_26_col84) + (M31_524288))];

            let mul_252_output_tmp_715f4_25 = mul_res_tmp_715f4_3;

            // Mul 252.

            let mul_res_tmp_715f4_26 =
                ((felt_252_unpack_from_27_range_check_output_output_tmp_715f4_2)
                    * (mul_252_output_tmp_715f4_25));
            let mul_res_limb_0_col85 = mul_res_tmp_715f4_26.get_m31(0);
            *row[85] = mul_res_limb_0_col85;
            let mul_res_limb_1_col86 = mul_res_tmp_715f4_26.get_m31(1);
            *row[86] = mul_res_limb_1_col86;
            let mul_res_limb_2_col87 = mul_res_tmp_715f4_26.get_m31(2);
            *row[87] = mul_res_limb_2_col87;
            let mul_res_limb_3_col88 = mul_res_tmp_715f4_26.get_m31(3);
            *row[88] = mul_res_limb_3_col88;
            let mul_res_limb_4_col89 = mul_res_tmp_715f4_26.get_m31(4);
            *row[89] = mul_res_limb_4_col89;
            let mul_res_limb_5_col90 = mul_res_tmp_715f4_26.get_m31(5);
            *row[90] = mul_res_limb_5_col90;
            let mul_res_limb_6_col91 = mul_res_tmp_715f4_26.get_m31(6);
            *row[91] = mul_res_limb_6_col91;
            let mul_res_limb_7_col92 = mul_res_tmp_715f4_26.get_m31(7);
            *row[92] = mul_res_limb_7_col92;
            let mul_res_limb_8_col93 = mul_res_tmp_715f4_26.get_m31(8);
            *row[93] = mul_res_limb_8_col93;
            let mul_res_limb_9_col94 = mul_res_tmp_715f4_26.get_m31(9);
            *row[94] = mul_res_limb_9_col94;
            let mul_res_limb_10_col95 = mul_res_tmp_715f4_26.get_m31(10);
            *row[95] = mul_res_limb_10_col95;
            let mul_res_limb_11_col96 = mul_res_tmp_715f4_26.get_m31(11);
            *row[96] = mul_res_limb_11_col96;
            let mul_res_limb_12_col97 = mul_res_tmp_715f4_26.get_m31(12);
            *row[97] = mul_res_limb_12_col97;
            let mul_res_limb_13_col98 = mul_res_tmp_715f4_26.get_m31(13);
            *row[98] = mul_res_limb_13_col98;
            let mul_res_limb_14_col99 = mul_res_tmp_715f4_26.get_m31(14);
            *row[99] = mul_res_limb_14_col99;
            let mul_res_limb_15_col100 = mul_res_tmp_715f4_26.get_m31(15);
            *row[100] = mul_res_limb_15_col100;
            let mul_res_limb_16_col101 = mul_res_tmp_715f4_26.get_m31(16);
            *row[101] = mul_res_limb_16_col101;
            let mul_res_limb_17_col102 = mul_res_tmp_715f4_26.get_m31(17);
            *row[102] = mul_res_limb_17_col102;
            let mul_res_limb_18_col103 = mul_res_tmp_715f4_26.get_m31(18);
            *row[103] = mul_res_limb_18_col103;
            let mul_res_limb_19_col104 = mul_res_tmp_715f4_26.get_m31(19);
            *row[104] = mul_res_limb_19_col104;
            let mul_res_limb_20_col105 = mul_res_tmp_715f4_26.get_m31(20);
            *row[105] = mul_res_limb_20_col105;
            let mul_res_limb_21_col106 = mul_res_tmp_715f4_26.get_m31(21);
            *row[106] = mul_res_limb_21_col106;
            let mul_res_limb_22_col107 = mul_res_tmp_715f4_26.get_m31(22);
            *row[107] = mul_res_limb_22_col107;
            let mul_res_limb_23_col108 = mul_res_tmp_715f4_26.get_m31(23);
            *row[108] = mul_res_limb_23_col108;
            let mul_res_limb_24_col109 = mul_res_tmp_715f4_26.get_m31(24);
            *row[109] = mul_res_limb_24_col109;
            let mul_res_limb_25_col110 = mul_res_tmp_715f4_26.get_m31(25);
            *row[110] = mul_res_limb_25_col110;
            let mul_res_limb_26_col111 = mul_res_tmp_715f4_26.get_m31(26);
            *row[111] = mul_res_limb_26_col111;
            let mul_res_limb_27_col112 = mul_res_tmp_715f4_26.get_m31(27);
            *row[112] = mul_res_limb_27_col112;

            // Range Check Mem Value N 28.

            *sub_component_inputs.range_check_9_9[4] = [mul_res_limb_0_col85, mul_res_limb_1_col86];
            *lookup_data.range_check_9_9_56 =
                [M31_517791011, mul_res_limb_0_col85, mul_res_limb_1_col86];
            *sub_component_inputs.range_check_9_9_b[4] =
                [mul_res_limb_2_col87, mul_res_limb_3_col88];
            *lookup_data.range_check_9_9_b_57 =
                [M31_1897792095, mul_res_limb_2_col87, mul_res_limb_3_col88];
            *sub_component_inputs.range_check_9_9_c[4] =
                [mul_res_limb_4_col89, mul_res_limb_5_col90];
            *lookup_data.range_check_9_9_c_58 =
                [M31_1881014476, mul_res_limb_4_col89, mul_res_limb_5_col90];
            *sub_component_inputs.range_check_9_9_d[4] =
                [mul_res_limb_6_col91, mul_res_limb_7_col92];
            *lookup_data.range_check_9_9_d_59 =
                [M31_1864236857, mul_res_limb_6_col91, mul_res_limb_7_col92];
            *sub_component_inputs.range_check_9_9_e[4] =
                [mul_res_limb_8_col93, mul_res_limb_9_col94];
            *lookup_data.range_check_9_9_e_60 =
                [M31_1847459238, mul_res_limb_8_col93, mul_res_limb_9_col94];
            *sub_component_inputs.range_check_9_9_f[4] =
                [mul_res_limb_10_col95, mul_res_limb_11_col96];
            *lookup_data.range_check_9_9_f_61 =
                [M31_1830681619, mul_res_limb_10_col95, mul_res_limb_11_col96];
            *sub_component_inputs.range_check_9_9_g[2] =
                [mul_res_limb_12_col97, mul_res_limb_13_col98];
            *lookup_data.range_check_9_9_g_62 =
                [M31_1813904000, mul_res_limb_12_col97, mul_res_limb_13_col98];
            *sub_component_inputs.range_check_9_9_h[2] =
                [mul_res_limb_14_col99, mul_res_limb_15_col100];
            *lookup_data.range_check_9_9_h_63 =
                [M31_2065568285, mul_res_limb_14_col99, mul_res_limb_15_col100];
            *sub_component_inputs.range_check_9_9[5] =
                [mul_res_limb_16_col101, mul_res_limb_17_col102];
            *lookup_data.range_check_9_9_64 =
                [M31_517791011, mul_res_limb_16_col101, mul_res_limb_17_col102];
            *sub_component_inputs.range_check_9_9_b[5] =
                [mul_res_limb_18_col103, mul_res_limb_19_col104];
            *lookup_data.range_check_9_9_b_65 =
                [M31_1897792095, mul_res_limb_18_col103, mul_res_limb_19_col104];
            *sub_component_inputs.range_check_9_9_c[5] =
                [mul_res_limb_20_col105, mul_res_limb_21_col106];
            *lookup_data.range_check_9_9_c_66 =
                [M31_1881014476, mul_res_limb_20_col105, mul_res_limb_21_col106];
            *sub_component_inputs.range_check_9_9_d[5] =
                [mul_res_limb_22_col107, mul_res_limb_23_col108];
            *lookup_data.range_check_9_9_d_67 =
                [M31_1864236857, mul_res_limb_22_col107, mul_res_limb_23_col108];
            *sub_component_inputs.range_check_9_9_e[5] =
                [mul_res_limb_24_col109, mul_res_limb_25_col110];
            *lookup_data.range_check_9_9_e_68 =
                [M31_1847459238, mul_res_limb_24_col109, mul_res_limb_25_col110];
            *sub_component_inputs.range_check_9_9_f[5] =
                [mul_res_limb_26_col111, mul_res_limb_27_col112];
            *lookup_data.range_check_9_9_f_69 =
                [M31_1830681619, mul_res_limb_26_col111, mul_res_limb_27_col112];

            // Verify Mul 252.

            // Double Karatsuba F 0 Fc 6.

            // Single Karatsuba N 7.

            let z0_tmp_715f4_27 = [
                ((unpacked_limb_0_col11) * (mul_res_limb_0_col29)),
                (((unpacked_limb_0_col11) * (mul_res_limb_1_col30))
                    + ((unpacked_limb_1_col12) * (mul_res_limb_0_col29))),
                ((((unpacked_limb_0_col11) * (mul_res_limb_2_col31))
                    + ((unpacked_limb_1_col12) * (mul_res_limb_1_col30)))
                    + ((unpacked_tmp_715f4_1.get_m31(2)) * (mul_res_limb_0_col29))),
                (((((unpacked_limb_0_col11) * (mul_res_limb_3_col32))
                    + ((unpacked_limb_1_col12) * (mul_res_limb_2_col31)))
                    + ((unpacked_tmp_715f4_1.get_m31(2)) * (mul_res_limb_1_col30)))
                    + ((unpacked_limb_3_col13) * (mul_res_limb_0_col29))),
                ((((((unpacked_limb_0_col11) * (mul_res_limb_4_col33))
                    + ((unpacked_limb_1_col12) * (mul_res_limb_3_col32)))
                    + ((unpacked_tmp_715f4_1.get_m31(2)) * (mul_res_limb_2_col31)))
                    + ((unpacked_limb_3_col13) * (mul_res_limb_1_col30)))
                    + ((unpacked_limb_4_col14) * (mul_res_limb_0_col29))),
                (((((((unpacked_limb_0_col11) * (mul_res_limb_5_col34))
                    + ((unpacked_limb_1_col12) * (mul_res_limb_4_col33)))
                    + ((unpacked_tmp_715f4_1.get_m31(2)) * (mul_res_limb_3_col32)))
                    + ((unpacked_limb_3_col13) * (mul_res_limb_2_col31)))
                    + ((unpacked_limb_4_col14) * (mul_res_limb_1_col30)))
                    + ((unpacked_tmp_715f4_1.get_m31(5)) * (mul_res_limb_0_col29))),
                ((((((((unpacked_limb_0_col11) * (mul_res_limb_6_col35))
                    + ((unpacked_limb_1_col12) * (mul_res_limb_5_col34)))
                    + ((unpacked_tmp_715f4_1.get_m31(2)) * (mul_res_limb_4_col33)))
                    + ((unpacked_limb_3_col13) * (mul_res_limb_3_col32)))
                    + ((unpacked_limb_4_col14) * (mul_res_limb_2_col31)))
                    + ((unpacked_tmp_715f4_1.get_m31(5)) * (mul_res_limb_1_col30)))
                    + ((unpacked_limb_6_col15) * (mul_res_limb_0_col29))),
                (((((((unpacked_limb_1_col12) * (mul_res_limb_6_col35))
                    + ((unpacked_tmp_715f4_1.get_m31(2)) * (mul_res_limb_5_col34)))
                    + ((unpacked_limb_3_col13) * (mul_res_limb_4_col33)))
                    + ((unpacked_limb_4_col14) * (mul_res_limb_3_col32)))
                    + ((unpacked_tmp_715f4_1.get_m31(5)) * (mul_res_limb_2_col31)))
                    + ((unpacked_limb_6_col15) * (mul_res_limb_1_col30))),
                ((((((unpacked_tmp_715f4_1.get_m31(2)) * (mul_res_limb_6_col35))
                    + ((unpacked_limb_3_col13) * (mul_res_limb_5_col34)))
                    + ((unpacked_limb_4_col14) * (mul_res_limb_4_col33)))
                    + ((unpacked_tmp_715f4_1.get_m31(5)) * (mul_res_limb_3_col32)))
                    + ((unpacked_limb_6_col15) * (mul_res_limb_2_col31))),
                (((((unpacked_limb_3_col13) * (mul_res_limb_6_col35))
                    + ((unpacked_limb_4_col14) * (mul_res_limb_5_col34)))
                    + ((unpacked_tmp_715f4_1.get_m31(5)) * (mul_res_limb_4_col33)))
                    + ((unpacked_limb_6_col15) * (mul_res_limb_3_col32))),
                ((((unpacked_limb_4_col14) * (mul_res_limb_6_col35))
                    + ((unpacked_tmp_715f4_1.get_m31(5)) * (mul_res_limb_5_col34)))
                    + ((unpacked_limb_6_col15) * (mul_res_limb_4_col33))),
                (((unpacked_tmp_715f4_1.get_m31(5)) * (mul_res_limb_6_col35))
                    + ((unpacked_limb_6_col15) * (mul_res_limb_5_col34))),
                ((unpacked_limb_6_col15) * (mul_res_limb_6_col35)),
            ];
            let z2_tmp_715f4_28 = [
                ((unpacked_limb_7_col16) * (mul_res_limb_7_col36)),
                (((unpacked_limb_7_col16) * (mul_res_limb_8_col37))
                    + ((unpacked_tmp_715f4_1.get_m31(8)) * (mul_res_limb_7_col36))),
                ((((unpacked_limb_7_col16) * (mul_res_limb_9_col38))
                    + ((unpacked_tmp_715f4_1.get_m31(8)) * (mul_res_limb_8_col37)))
                    + ((unpacked_limb_9_col17) * (mul_res_limb_7_col36))),
                (((((unpacked_limb_7_col16) * (mul_res_limb_10_col39))
                    + ((unpacked_tmp_715f4_1.get_m31(8)) * (mul_res_limb_9_col38)))
                    + ((unpacked_limb_9_col17) * (mul_res_limb_8_col37)))
                    + ((unpacked_limb_10_col18) * (mul_res_limb_7_col36))),
                ((((((unpacked_limb_7_col16) * (mul_res_limb_11_col40))
                    + ((unpacked_tmp_715f4_1.get_m31(8)) * (mul_res_limb_10_col39)))
                    + ((unpacked_limb_9_col17) * (mul_res_limb_9_col38)))
                    + ((unpacked_limb_10_col18) * (mul_res_limb_8_col37)))
                    + ((unpacked_tmp_715f4_1.get_m31(11)) * (mul_res_limb_7_col36))),
                (((((((unpacked_limb_7_col16) * (mul_res_limb_12_col41))
                    + ((unpacked_tmp_715f4_1.get_m31(8)) * (mul_res_limb_11_col40)))
                    + ((unpacked_limb_9_col17) * (mul_res_limb_10_col39)))
                    + ((unpacked_limb_10_col18) * (mul_res_limb_9_col38)))
                    + ((unpacked_tmp_715f4_1.get_m31(11)) * (mul_res_limb_8_col37)))
                    + ((unpacked_limb_12_col19) * (mul_res_limb_7_col36))),
                ((((((((unpacked_limb_7_col16) * (mul_res_limb_13_col42))
                    + ((unpacked_tmp_715f4_1.get_m31(8)) * (mul_res_limb_12_col41)))
                    + ((unpacked_limb_9_col17) * (mul_res_limb_11_col40)))
                    + ((unpacked_limb_10_col18) * (mul_res_limb_10_col39)))
                    + ((unpacked_tmp_715f4_1.get_m31(11)) * (mul_res_limb_9_col38)))
                    + ((unpacked_limb_12_col19) * (mul_res_limb_8_col37)))
                    + ((unpacked_limb_13_col20) * (mul_res_limb_7_col36))),
                (((((((unpacked_tmp_715f4_1.get_m31(8)) * (mul_res_limb_13_col42))
                    + ((unpacked_limb_9_col17) * (mul_res_limb_12_col41)))
                    + ((unpacked_limb_10_col18) * (mul_res_limb_11_col40)))
                    + ((unpacked_tmp_715f4_1.get_m31(11)) * (mul_res_limb_10_col39)))
                    + ((unpacked_limb_12_col19) * (mul_res_limb_9_col38)))
                    + ((unpacked_limb_13_col20) * (mul_res_limb_8_col37))),
                ((((((unpacked_limb_9_col17) * (mul_res_limb_13_col42))
                    + ((unpacked_limb_10_col18) * (mul_res_limb_12_col41)))
                    + ((unpacked_tmp_715f4_1.get_m31(11)) * (mul_res_limb_11_col40)))
                    + ((unpacked_limb_12_col19) * (mul_res_limb_10_col39)))
                    + ((unpacked_limb_13_col20) * (mul_res_limb_9_col38))),
                (((((unpacked_limb_10_col18) * (mul_res_limb_13_col42))
                    + ((unpacked_tmp_715f4_1.get_m31(11)) * (mul_res_limb_12_col41)))
                    + ((unpacked_limb_12_col19) * (mul_res_limb_11_col40)))
                    + ((unpacked_limb_13_col20) * (mul_res_limb_10_col39))),
                ((((unpacked_tmp_715f4_1.get_m31(11)) * (mul_res_limb_13_col42))
                    + ((unpacked_limb_12_col19) * (mul_res_limb_12_col41)))
                    + ((unpacked_limb_13_col20) * (mul_res_limb_11_col40))),
                (((unpacked_limb_12_col19) * (mul_res_limb_13_col42))
                    + ((unpacked_limb_13_col20) * (mul_res_limb_12_col41))),
                ((unpacked_limb_13_col20) * (mul_res_limb_13_col42)),
            ];
            let x_sum_tmp_715f4_29 = [
                ((unpacked_limb_0_col11) + (unpacked_limb_7_col16)),
                ((unpacked_limb_1_col12) + (unpacked_tmp_715f4_1.get_m31(8))),
                ((unpacked_tmp_715f4_1.get_m31(2)) + (unpacked_limb_9_col17)),
                ((unpacked_limb_3_col13) + (unpacked_limb_10_col18)),
                ((unpacked_limb_4_col14) + (unpacked_tmp_715f4_1.get_m31(11))),
                ((unpacked_tmp_715f4_1.get_m31(5)) + (unpacked_limb_12_col19)),
                ((unpacked_limb_6_col15) + (unpacked_limb_13_col20)),
            ];
            let y_sum_tmp_715f4_30 = [
                ((mul_res_limb_0_col29) + (mul_res_limb_7_col36)),
                ((mul_res_limb_1_col30) + (mul_res_limb_8_col37)),
                ((mul_res_limb_2_col31) + (mul_res_limb_9_col38)),
                ((mul_res_limb_3_col32) + (mul_res_limb_10_col39)),
                ((mul_res_limb_4_col33) + (mul_res_limb_11_col40)),
                ((mul_res_limb_5_col34) + (mul_res_limb_12_col41)),
                ((mul_res_limb_6_col35) + (mul_res_limb_13_col42)),
            ];
            let single_karatsuba_n_7_output_tmp_715f4_31 = [
                z0_tmp_715f4_27[0],
                z0_tmp_715f4_27[1],
                z0_tmp_715f4_27[2],
                z0_tmp_715f4_27[3],
                z0_tmp_715f4_27[4],
                z0_tmp_715f4_27[5],
                z0_tmp_715f4_27[6],
                ((z0_tmp_715f4_27[7])
                    + ((((x_sum_tmp_715f4_29[0]) * (y_sum_tmp_715f4_30[0]))
                        - (z0_tmp_715f4_27[0]))
                        - (z2_tmp_715f4_28[0]))),
                ((z0_tmp_715f4_27[8])
                    + (((((x_sum_tmp_715f4_29[0]) * (y_sum_tmp_715f4_30[1]))
                        + ((x_sum_tmp_715f4_29[1]) * (y_sum_tmp_715f4_30[0])))
                        - (z0_tmp_715f4_27[1]))
                        - (z2_tmp_715f4_28[1]))),
                ((z0_tmp_715f4_27[9])
                    + ((((((x_sum_tmp_715f4_29[0]) * (y_sum_tmp_715f4_30[2]))
                        + ((x_sum_tmp_715f4_29[1]) * (y_sum_tmp_715f4_30[1])))
                        + ((x_sum_tmp_715f4_29[2]) * (y_sum_tmp_715f4_30[0])))
                        - (z0_tmp_715f4_27[2]))
                        - (z2_tmp_715f4_28[2]))),
                ((z0_tmp_715f4_27[10])
                    + (((((((x_sum_tmp_715f4_29[0]) * (y_sum_tmp_715f4_30[3]))
                        + ((x_sum_tmp_715f4_29[1]) * (y_sum_tmp_715f4_30[2])))
                        + ((x_sum_tmp_715f4_29[2]) * (y_sum_tmp_715f4_30[1])))
                        + ((x_sum_tmp_715f4_29[3]) * (y_sum_tmp_715f4_30[0])))
                        - (z0_tmp_715f4_27[3]))
                        - (z2_tmp_715f4_28[3]))),
                ((z0_tmp_715f4_27[11])
                    + ((((((((x_sum_tmp_715f4_29[0]) * (y_sum_tmp_715f4_30[4]))
                        + ((x_sum_tmp_715f4_29[1]) * (y_sum_tmp_715f4_30[3])))
                        + ((x_sum_tmp_715f4_29[2]) * (y_sum_tmp_715f4_30[2])))
                        + ((x_sum_tmp_715f4_29[3]) * (y_sum_tmp_715f4_30[1])))
                        + ((x_sum_tmp_715f4_29[4]) * (y_sum_tmp_715f4_30[0])))
                        - (z0_tmp_715f4_27[4]))
                        - (z2_tmp_715f4_28[4]))),
                ((z0_tmp_715f4_27[12])
                    + (((((((((x_sum_tmp_715f4_29[0]) * (y_sum_tmp_715f4_30[5]))
                        + ((x_sum_tmp_715f4_29[1]) * (y_sum_tmp_715f4_30[4])))
                        + ((x_sum_tmp_715f4_29[2]) * (y_sum_tmp_715f4_30[3])))
                        + ((x_sum_tmp_715f4_29[3]) * (y_sum_tmp_715f4_30[2])))
                        + ((x_sum_tmp_715f4_29[4]) * (y_sum_tmp_715f4_30[1])))
                        + ((x_sum_tmp_715f4_29[5]) * (y_sum_tmp_715f4_30[0])))
                        - (z0_tmp_715f4_27[5]))
                        - (z2_tmp_715f4_28[5]))),
                ((((((((((x_sum_tmp_715f4_29[0]) * (y_sum_tmp_715f4_30[6]))
                    + ((x_sum_tmp_715f4_29[1]) * (y_sum_tmp_715f4_30[5])))
                    + ((x_sum_tmp_715f4_29[2]) * (y_sum_tmp_715f4_30[4])))
                    + ((x_sum_tmp_715f4_29[3]) * (y_sum_tmp_715f4_30[3])))
                    + ((x_sum_tmp_715f4_29[4]) * (y_sum_tmp_715f4_30[2])))
                    + ((x_sum_tmp_715f4_29[5]) * (y_sum_tmp_715f4_30[1])))
                    + ((x_sum_tmp_715f4_29[6]) * (y_sum_tmp_715f4_30[0])))
                    - (z0_tmp_715f4_27[6]))
                    - (z2_tmp_715f4_28[6])),
                ((z2_tmp_715f4_28[0])
                    + (((((((((x_sum_tmp_715f4_29[1]) * (y_sum_tmp_715f4_30[6]))
                        + ((x_sum_tmp_715f4_29[2]) * (y_sum_tmp_715f4_30[5])))
                        + ((x_sum_tmp_715f4_29[3]) * (y_sum_tmp_715f4_30[4])))
                        + ((x_sum_tmp_715f4_29[4]) * (y_sum_tmp_715f4_30[3])))
                        + ((x_sum_tmp_715f4_29[5]) * (y_sum_tmp_715f4_30[2])))
                        + ((x_sum_tmp_715f4_29[6]) * (y_sum_tmp_715f4_30[1])))
                        - (z0_tmp_715f4_27[7]))
                        - (z2_tmp_715f4_28[7]))),
                ((z2_tmp_715f4_28[1])
                    + ((((((((x_sum_tmp_715f4_29[2]) * (y_sum_tmp_715f4_30[6]))
                        + ((x_sum_tmp_715f4_29[3]) * (y_sum_tmp_715f4_30[5])))
                        + ((x_sum_tmp_715f4_29[4]) * (y_sum_tmp_715f4_30[4])))
                        + ((x_sum_tmp_715f4_29[5]) * (y_sum_tmp_715f4_30[3])))
                        + ((x_sum_tmp_715f4_29[6]) * (y_sum_tmp_715f4_30[2])))
                        - (z0_tmp_715f4_27[8]))
                        - (z2_tmp_715f4_28[8]))),
                ((z2_tmp_715f4_28[2])
                    + (((((((x_sum_tmp_715f4_29[3]) * (y_sum_tmp_715f4_30[6]))
                        + ((x_sum_tmp_715f4_29[4]) * (y_sum_tmp_715f4_30[5])))
                        + ((x_sum_tmp_715f4_29[5]) * (y_sum_tmp_715f4_30[4])))
                        + ((x_sum_tmp_715f4_29[6]) * (y_sum_tmp_715f4_30[3])))
                        - (z0_tmp_715f4_27[9]))
                        - (z2_tmp_715f4_28[9]))),
                ((z2_tmp_715f4_28[3])
                    + ((((((x_sum_tmp_715f4_29[4]) * (y_sum_tmp_715f4_30[6]))
                        + ((x_sum_tmp_715f4_29[5]) * (y_sum_tmp_715f4_30[5])))
                        + ((x_sum_tmp_715f4_29[6]) * (y_sum_tmp_715f4_30[4])))
                        - (z0_tmp_715f4_27[10]))
                        - (z2_tmp_715f4_28[10]))),
                ((z2_tmp_715f4_28[4])
                    + (((((x_sum_tmp_715f4_29[5]) * (y_sum_tmp_715f4_30[6]))
                        + ((x_sum_tmp_715f4_29[6]) * (y_sum_tmp_715f4_30[5])))
                        - (z0_tmp_715f4_27[11]))
                        - (z2_tmp_715f4_28[11]))),
                ((z2_tmp_715f4_28[5])
                    + ((((x_sum_tmp_715f4_29[6]) * (y_sum_tmp_715f4_30[6]))
                        - (z0_tmp_715f4_27[12]))
                        - (z2_tmp_715f4_28[12]))),
                z2_tmp_715f4_28[6],
                z2_tmp_715f4_28[7],
                z2_tmp_715f4_28[8],
                z2_tmp_715f4_28[9],
                z2_tmp_715f4_28[10],
                z2_tmp_715f4_28[11],
                z2_tmp_715f4_28[12],
            ];

            // Single Karatsuba N 7.

            let z0_tmp_715f4_32 = [
                ((unpacked_tmp_715f4_1.get_m31(14)) * (mul_res_limb_14_col43)),
                (((unpacked_tmp_715f4_1.get_m31(14)) * (mul_res_limb_15_col44))
                    + ((unpacked_limb_15_col21) * (mul_res_limb_14_col43))),
                ((((unpacked_tmp_715f4_1.get_m31(14)) * (mul_res_limb_16_col45))
                    + ((unpacked_limb_15_col21) * (mul_res_limb_15_col44)))
                    + ((unpacked_limb_16_col22) * (mul_res_limb_14_col43))),
                (((((unpacked_tmp_715f4_1.get_m31(14)) * (mul_res_limb_17_col46))
                    + ((unpacked_limb_15_col21) * (mul_res_limb_16_col45)))
                    + ((unpacked_limb_16_col22) * (mul_res_limb_15_col44)))
                    + ((unpacked_tmp_715f4_1.get_m31(17)) * (mul_res_limb_14_col43))),
                ((((((unpacked_tmp_715f4_1.get_m31(14)) * (mul_res_limb_18_col47))
                    + ((unpacked_limb_15_col21) * (mul_res_limb_17_col46)))
                    + ((unpacked_limb_16_col22) * (mul_res_limb_16_col45)))
                    + ((unpacked_tmp_715f4_1.get_m31(17)) * (mul_res_limb_15_col44)))
                    + ((unpacked_limb_18_col23) * (mul_res_limb_14_col43))),
                (((((((unpacked_tmp_715f4_1.get_m31(14)) * (mul_res_limb_19_col48))
                    + ((unpacked_limb_15_col21) * (mul_res_limb_18_col47)))
                    + ((unpacked_limb_16_col22) * (mul_res_limb_17_col46)))
                    + ((unpacked_tmp_715f4_1.get_m31(17)) * (mul_res_limb_16_col45)))
                    + ((unpacked_limb_18_col23) * (mul_res_limb_15_col44)))
                    + ((unpacked_limb_19_col24) * (mul_res_limb_14_col43))),
                ((((((((unpacked_tmp_715f4_1.get_m31(14)) * (mul_res_limb_20_col49))
                    + ((unpacked_limb_15_col21) * (mul_res_limb_19_col48)))
                    + ((unpacked_limb_16_col22) * (mul_res_limb_18_col47)))
                    + ((unpacked_tmp_715f4_1.get_m31(17)) * (mul_res_limb_17_col46)))
                    + ((unpacked_limb_18_col23) * (mul_res_limb_16_col45)))
                    + ((unpacked_limb_19_col24) * (mul_res_limb_15_col44)))
                    + ((unpacked_tmp_715f4_1.get_m31(20)) * (mul_res_limb_14_col43))),
                (((((((unpacked_limb_15_col21) * (mul_res_limb_20_col49))
                    + ((unpacked_limb_16_col22) * (mul_res_limb_19_col48)))
                    + ((unpacked_tmp_715f4_1.get_m31(17)) * (mul_res_limb_18_col47)))
                    + ((unpacked_limb_18_col23) * (mul_res_limb_17_col46)))
                    + ((unpacked_limb_19_col24) * (mul_res_limb_16_col45)))
                    + ((unpacked_tmp_715f4_1.get_m31(20)) * (mul_res_limb_15_col44))),
                ((((((unpacked_limb_16_col22) * (mul_res_limb_20_col49))
                    + ((unpacked_tmp_715f4_1.get_m31(17)) * (mul_res_limb_19_col48)))
                    + ((unpacked_limb_18_col23) * (mul_res_limb_18_col47)))
                    + ((unpacked_limb_19_col24) * (mul_res_limb_17_col46)))
                    + ((unpacked_tmp_715f4_1.get_m31(20)) * (mul_res_limb_16_col45))),
                (((((unpacked_tmp_715f4_1.get_m31(17)) * (mul_res_limb_20_col49))
                    + ((unpacked_limb_18_col23) * (mul_res_limb_19_col48)))
                    + ((unpacked_limb_19_col24) * (mul_res_limb_18_col47)))
                    + ((unpacked_tmp_715f4_1.get_m31(20)) * (mul_res_limb_17_col46))),
                ((((unpacked_limb_18_col23) * (mul_res_limb_20_col49))
                    + ((unpacked_limb_19_col24) * (mul_res_limb_19_col48)))
                    + ((unpacked_tmp_715f4_1.get_m31(20)) * (mul_res_limb_18_col47))),
                (((unpacked_limb_19_col24) * (mul_res_limb_20_col49))
                    + ((unpacked_tmp_715f4_1.get_m31(20)) * (mul_res_limb_19_col48))),
                ((unpacked_tmp_715f4_1.get_m31(20)) * (mul_res_limb_20_col49)),
            ];
            let z2_tmp_715f4_33 = [
                ((unpacked_limb_21_col25) * (mul_res_limb_21_col50)),
                (((unpacked_limb_21_col25) * (mul_res_limb_22_col51))
                    + ((unpacked_limb_22_col26) * (mul_res_limb_21_col50))),
                ((((unpacked_limb_21_col25) * (mul_res_limb_23_col52))
                    + ((unpacked_limb_22_col26) * (mul_res_limb_22_col51)))
                    + ((unpacked_tmp_715f4_1.get_m31(23)) * (mul_res_limb_21_col50))),
                (((((unpacked_limb_21_col25) * (mul_res_limb_24_col53))
                    + ((unpacked_limb_22_col26) * (mul_res_limb_23_col52)))
                    + ((unpacked_tmp_715f4_1.get_m31(23)) * (mul_res_limb_22_col51)))
                    + ((unpacked_limb_24_col27) * (mul_res_limb_21_col50))),
                ((((((unpacked_limb_21_col25) * (mul_res_limb_25_col54))
                    + ((unpacked_limb_22_col26) * (mul_res_limb_24_col53)))
                    + ((unpacked_tmp_715f4_1.get_m31(23)) * (mul_res_limb_23_col52)))
                    + ((unpacked_limb_24_col27) * (mul_res_limb_22_col51)))
                    + ((unpacked_limb_25_col28) * (mul_res_limb_21_col50))),
                (((((((unpacked_limb_21_col25) * (mul_res_limb_26_col55))
                    + ((unpacked_limb_22_col26) * (mul_res_limb_25_col54)))
                    + ((unpacked_tmp_715f4_1.get_m31(23)) * (mul_res_limb_24_col53)))
                    + ((unpacked_limb_24_col27) * (mul_res_limb_23_col52)))
                    + ((unpacked_limb_25_col28) * (mul_res_limb_22_col51)))
                    + ((unpacked_tmp_715f4_1.get_m31(26)) * (mul_res_limb_21_col50))),
                ((((((((unpacked_limb_21_col25) * (mul_res_limb_27_col56))
                    + ((unpacked_limb_22_col26) * (mul_res_limb_26_col55)))
                    + ((unpacked_tmp_715f4_1.get_m31(23)) * (mul_res_limb_25_col54)))
                    + ((unpacked_limb_24_col27) * (mul_res_limb_24_col53)))
                    + ((unpacked_limb_25_col28) * (mul_res_limb_23_col52)))
                    + ((unpacked_tmp_715f4_1.get_m31(26)) * (mul_res_limb_22_col51)))
                    + ((input_limb_9_col10) * (mul_res_limb_21_col50))),
                (((((((unpacked_limb_22_col26) * (mul_res_limb_27_col56))
                    + ((unpacked_tmp_715f4_1.get_m31(23)) * (mul_res_limb_26_col55)))
                    + ((unpacked_limb_24_col27) * (mul_res_limb_25_col54)))
                    + ((unpacked_limb_25_col28) * (mul_res_limb_24_col53)))
                    + ((unpacked_tmp_715f4_1.get_m31(26)) * (mul_res_limb_23_col52)))
                    + ((input_limb_9_col10) * (mul_res_limb_22_col51))),
                ((((((unpacked_tmp_715f4_1.get_m31(23)) * (mul_res_limb_27_col56))
                    + ((unpacked_limb_24_col27) * (mul_res_limb_26_col55)))
                    + ((unpacked_limb_25_col28) * (mul_res_limb_25_col54)))
                    + ((unpacked_tmp_715f4_1.get_m31(26)) * (mul_res_limb_24_col53)))
                    + ((input_limb_9_col10) * (mul_res_limb_23_col52))),
                (((((unpacked_limb_24_col27) * (mul_res_limb_27_col56))
                    + ((unpacked_limb_25_col28) * (mul_res_limb_26_col55)))
                    + ((unpacked_tmp_715f4_1.get_m31(26)) * (mul_res_limb_25_col54)))
                    + ((input_limb_9_col10) * (mul_res_limb_24_col53))),
                ((((unpacked_limb_25_col28) * (mul_res_limb_27_col56))
                    + ((unpacked_tmp_715f4_1.get_m31(26)) * (mul_res_limb_26_col55)))
                    + ((input_limb_9_col10) * (mul_res_limb_25_col54))),
                (((unpacked_tmp_715f4_1.get_m31(26)) * (mul_res_limb_27_col56))
                    + ((input_limb_9_col10) * (mul_res_limb_26_col55))),
                ((input_limb_9_col10) * (mul_res_limb_27_col56)),
            ];
            let x_sum_tmp_715f4_34 = [
                ((unpacked_tmp_715f4_1.get_m31(14)) + (unpacked_limb_21_col25)),
                ((unpacked_limb_15_col21) + (unpacked_limb_22_col26)),
                ((unpacked_limb_16_col22) + (unpacked_tmp_715f4_1.get_m31(23))),
                ((unpacked_tmp_715f4_1.get_m31(17)) + (unpacked_limb_24_col27)),
                ((unpacked_limb_18_col23) + (unpacked_limb_25_col28)),
                ((unpacked_limb_19_col24) + (unpacked_tmp_715f4_1.get_m31(26))),
                ((unpacked_tmp_715f4_1.get_m31(20)) + (input_limb_9_col10)),
            ];
            let y_sum_tmp_715f4_35 = [
                ((mul_res_limb_14_col43) + (mul_res_limb_21_col50)),
                ((mul_res_limb_15_col44) + (mul_res_limb_22_col51)),
                ((mul_res_limb_16_col45) + (mul_res_limb_23_col52)),
                ((mul_res_limb_17_col46) + (mul_res_limb_24_col53)),
                ((mul_res_limb_18_col47) + (mul_res_limb_25_col54)),
                ((mul_res_limb_19_col48) + (mul_res_limb_26_col55)),
                ((mul_res_limb_20_col49) + (mul_res_limb_27_col56)),
            ];
            let single_karatsuba_n_7_output_tmp_715f4_36 = [
                z0_tmp_715f4_32[0],
                z0_tmp_715f4_32[1],
                z0_tmp_715f4_32[2],
                z0_tmp_715f4_32[3],
                z0_tmp_715f4_32[4],
                z0_tmp_715f4_32[5],
                z0_tmp_715f4_32[6],
                ((z0_tmp_715f4_32[7])
                    + ((((x_sum_tmp_715f4_34[0]) * (y_sum_tmp_715f4_35[0]))
                        - (z0_tmp_715f4_32[0]))
                        - (z2_tmp_715f4_33[0]))),
                ((z0_tmp_715f4_32[8])
                    + (((((x_sum_tmp_715f4_34[0]) * (y_sum_tmp_715f4_35[1]))
                        + ((x_sum_tmp_715f4_34[1]) * (y_sum_tmp_715f4_35[0])))
                        - (z0_tmp_715f4_32[1]))
                        - (z2_tmp_715f4_33[1]))),
                ((z0_tmp_715f4_32[9])
                    + ((((((x_sum_tmp_715f4_34[0]) * (y_sum_tmp_715f4_35[2]))
                        + ((x_sum_tmp_715f4_34[1]) * (y_sum_tmp_715f4_35[1])))
                        + ((x_sum_tmp_715f4_34[2]) * (y_sum_tmp_715f4_35[0])))
                        - (z0_tmp_715f4_32[2]))
                        - (z2_tmp_715f4_33[2]))),
                ((z0_tmp_715f4_32[10])
                    + (((((((x_sum_tmp_715f4_34[0]) * (y_sum_tmp_715f4_35[3]))
                        + ((x_sum_tmp_715f4_34[1]) * (y_sum_tmp_715f4_35[2])))
                        + ((x_sum_tmp_715f4_34[2]) * (y_sum_tmp_715f4_35[1])))
                        + ((x_sum_tmp_715f4_34[3]) * (y_sum_tmp_715f4_35[0])))
                        - (z0_tmp_715f4_32[3]))
                        - (z2_tmp_715f4_33[3]))),
                ((z0_tmp_715f4_32[11])
                    + ((((((((x_sum_tmp_715f4_34[0]) * (y_sum_tmp_715f4_35[4]))
                        + ((x_sum_tmp_715f4_34[1]) * (y_sum_tmp_715f4_35[3])))
                        + ((x_sum_tmp_715f4_34[2]) * (y_sum_tmp_715f4_35[2])))
                        + ((x_sum_tmp_715f4_34[3]) * (y_sum_tmp_715f4_35[1])))
                        + ((x_sum_tmp_715f4_34[4]) * (y_sum_tmp_715f4_35[0])))
                        - (z0_tmp_715f4_32[4]))
                        - (z2_tmp_715f4_33[4]))),
                ((z0_tmp_715f4_32[12])
                    + (((((((((x_sum_tmp_715f4_34[0]) * (y_sum_tmp_715f4_35[5]))
                        + ((x_sum_tmp_715f4_34[1]) * (y_sum_tmp_715f4_35[4])))
                        + ((x_sum_tmp_715f4_34[2]) * (y_sum_tmp_715f4_35[3])))
                        + ((x_sum_tmp_715f4_34[3]) * (y_sum_tmp_715f4_35[2])))
                        + ((x_sum_tmp_715f4_34[4]) * (y_sum_tmp_715f4_35[1])))
                        + ((x_sum_tmp_715f4_34[5]) * (y_sum_tmp_715f4_35[0])))
                        - (z0_tmp_715f4_32[5]))
                        - (z2_tmp_715f4_33[5]))),
                ((((((((((x_sum_tmp_715f4_34[0]) * (y_sum_tmp_715f4_35[6]))
                    + ((x_sum_tmp_715f4_34[1]) * (y_sum_tmp_715f4_35[5])))
                    + ((x_sum_tmp_715f4_34[2]) * (y_sum_tmp_715f4_35[4])))
                    + ((x_sum_tmp_715f4_34[3]) * (y_sum_tmp_715f4_35[3])))
                    + ((x_sum_tmp_715f4_34[4]) * (y_sum_tmp_715f4_35[2])))
                    + ((x_sum_tmp_715f4_34[5]) * (y_sum_tmp_715f4_35[1])))
                    + ((x_sum_tmp_715f4_34[6]) * (y_sum_tmp_715f4_35[0])))
                    - (z0_tmp_715f4_32[6]))
                    - (z2_tmp_715f4_33[6])),
                ((z2_tmp_715f4_33[0])
                    + (((((((((x_sum_tmp_715f4_34[1]) * (y_sum_tmp_715f4_35[6]))
                        + ((x_sum_tmp_715f4_34[2]) * (y_sum_tmp_715f4_35[5])))
                        + ((x_sum_tmp_715f4_34[3]) * (y_sum_tmp_715f4_35[4])))
                        + ((x_sum_tmp_715f4_34[4]) * (y_sum_tmp_715f4_35[3])))
                        + ((x_sum_tmp_715f4_34[5]) * (y_sum_tmp_715f4_35[2])))
                        + ((x_sum_tmp_715f4_34[6]) * (y_sum_tmp_715f4_35[1])))
                        - (z0_tmp_715f4_32[7]))
                        - (z2_tmp_715f4_33[7]))),
                ((z2_tmp_715f4_33[1])
                    + ((((((((x_sum_tmp_715f4_34[2]) * (y_sum_tmp_715f4_35[6]))
                        + ((x_sum_tmp_715f4_34[3]) * (y_sum_tmp_715f4_35[5])))
                        + ((x_sum_tmp_715f4_34[4]) * (y_sum_tmp_715f4_35[4])))
                        + ((x_sum_tmp_715f4_34[5]) * (y_sum_tmp_715f4_35[3])))
                        + ((x_sum_tmp_715f4_34[6]) * (y_sum_tmp_715f4_35[2])))
                        - (z0_tmp_715f4_32[8]))
                        - (z2_tmp_715f4_33[8]))),
                ((z2_tmp_715f4_33[2])
                    + (((((((x_sum_tmp_715f4_34[3]) * (y_sum_tmp_715f4_35[6]))
                        + ((x_sum_tmp_715f4_34[4]) * (y_sum_tmp_715f4_35[5])))
                        + ((x_sum_tmp_715f4_34[5]) * (y_sum_tmp_715f4_35[4])))
                        + ((x_sum_tmp_715f4_34[6]) * (y_sum_tmp_715f4_35[3])))
                        - (z0_tmp_715f4_32[9]))
                        - (z2_tmp_715f4_33[9]))),
                ((z2_tmp_715f4_33[3])
                    + ((((((x_sum_tmp_715f4_34[4]) * (y_sum_tmp_715f4_35[6]))
                        + ((x_sum_tmp_715f4_34[5]) * (y_sum_tmp_715f4_35[5])))
                        + ((x_sum_tmp_715f4_34[6]) * (y_sum_tmp_715f4_35[4])))
                        - (z0_tmp_715f4_32[10]))
                        - (z2_tmp_715f4_33[10]))),
                ((z2_tmp_715f4_33[4])
                    + (((((x_sum_tmp_715f4_34[5]) * (y_sum_tmp_715f4_35[6]))
                        + ((x_sum_tmp_715f4_34[6]) * (y_sum_tmp_715f4_35[5])))
                        - (z0_tmp_715f4_32[11]))
                        - (z2_tmp_715f4_33[11]))),
                ((z2_tmp_715f4_33[5])
                    + ((((x_sum_tmp_715f4_34[6]) * (y_sum_tmp_715f4_35[6]))
                        - (z0_tmp_715f4_32[12]))
                        - (z2_tmp_715f4_33[12]))),
                z2_tmp_715f4_33[6],
                z2_tmp_715f4_33[7],
                z2_tmp_715f4_33[8],
                z2_tmp_715f4_33[9],
                z2_tmp_715f4_33[10],
                z2_tmp_715f4_33[11],
                z2_tmp_715f4_33[12],
            ];

            let x_sum_tmp_715f4_37 = [
                ((unpacked_limb_0_col11) + (unpacked_tmp_715f4_1.get_m31(14))),
                ((unpacked_limb_1_col12) + (unpacked_limb_15_col21)),
                ((unpacked_tmp_715f4_1.get_m31(2)) + (unpacked_limb_16_col22)),
                ((unpacked_limb_3_col13) + (unpacked_tmp_715f4_1.get_m31(17))),
                ((unpacked_limb_4_col14) + (unpacked_limb_18_col23)),
                ((unpacked_tmp_715f4_1.get_m31(5)) + (unpacked_limb_19_col24)),
                ((unpacked_limb_6_col15) + (unpacked_tmp_715f4_1.get_m31(20))),
                ((unpacked_limb_7_col16) + (unpacked_limb_21_col25)),
                ((unpacked_tmp_715f4_1.get_m31(8)) + (unpacked_limb_22_col26)),
                ((unpacked_limb_9_col17) + (unpacked_tmp_715f4_1.get_m31(23))),
                ((unpacked_limb_10_col18) + (unpacked_limb_24_col27)),
                ((unpacked_tmp_715f4_1.get_m31(11)) + (unpacked_limb_25_col28)),
                ((unpacked_limb_12_col19) + (unpacked_tmp_715f4_1.get_m31(26))),
                ((unpacked_limb_13_col20) + (input_limb_9_col10)),
            ];
            let y_sum_tmp_715f4_38 = [
                ((mul_res_limb_0_col29) + (mul_res_limb_14_col43)),
                ((mul_res_limb_1_col30) + (mul_res_limb_15_col44)),
                ((mul_res_limb_2_col31) + (mul_res_limb_16_col45)),
                ((mul_res_limb_3_col32) + (mul_res_limb_17_col46)),
                ((mul_res_limb_4_col33) + (mul_res_limb_18_col47)),
                ((mul_res_limb_5_col34) + (mul_res_limb_19_col48)),
                ((mul_res_limb_6_col35) + (mul_res_limb_20_col49)),
                ((mul_res_limb_7_col36) + (mul_res_limb_21_col50)),
                ((mul_res_limb_8_col37) + (mul_res_limb_22_col51)),
                ((mul_res_limb_9_col38) + (mul_res_limb_23_col52)),
                ((mul_res_limb_10_col39) + (mul_res_limb_24_col53)),
                ((mul_res_limb_11_col40) + (mul_res_limb_25_col54)),
                ((mul_res_limb_12_col41) + (mul_res_limb_26_col55)),
                ((mul_res_limb_13_col42) + (mul_res_limb_27_col56)),
            ];

            // Single Karatsuba N 7.

            let z0_tmp_715f4_39 = [
                ((x_sum_tmp_715f4_37[0]) * (y_sum_tmp_715f4_38[0])),
                (((x_sum_tmp_715f4_37[0]) * (y_sum_tmp_715f4_38[1]))
                    + ((x_sum_tmp_715f4_37[1]) * (y_sum_tmp_715f4_38[0]))),
                ((((x_sum_tmp_715f4_37[0]) * (y_sum_tmp_715f4_38[2]))
                    + ((x_sum_tmp_715f4_37[1]) * (y_sum_tmp_715f4_38[1])))
                    + ((x_sum_tmp_715f4_37[2]) * (y_sum_tmp_715f4_38[0]))),
                (((((x_sum_tmp_715f4_37[0]) * (y_sum_tmp_715f4_38[3]))
                    + ((x_sum_tmp_715f4_37[1]) * (y_sum_tmp_715f4_38[2])))
                    + ((x_sum_tmp_715f4_37[2]) * (y_sum_tmp_715f4_38[1])))
                    + ((x_sum_tmp_715f4_37[3]) * (y_sum_tmp_715f4_38[0]))),
                ((((((x_sum_tmp_715f4_37[0]) * (y_sum_tmp_715f4_38[4]))
                    + ((x_sum_tmp_715f4_37[1]) * (y_sum_tmp_715f4_38[3])))
                    + ((x_sum_tmp_715f4_37[2]) * (y_sum_tmp_715f4_38[2])))
                    + ((x_sum_tmp_715f4_37[3]) * (y_sum_tmp_715f4_38[1])))
                    + ((x_sum_tmp_715f4_37[4]) * (y_sum_tmp_715f4_38[0]))),
                (((((((x_sum_tmp_715f4_37[0]) * (y_sum_tmp_715f4_38[5]))
                    + ((x_sum_tmp_715f4_37[1]) * (y_sum_tmp_715f4_38[4])))
                    + ((x_sum_tmp_715f4_37[2]) * (y_sum_tmp_715f4_38[3])))
                    + ((x_sum_tmp_715f4_37[3]) * (y_sum_tmp_715f4_38[2])))
                    + ((x_sum_tmp_715f4_37[4]) * (y_sum_tmp_715f4_38[1])))
                    + ((x_sum_tmp_715f4_37[5]) * (y_sum_tmp_715f4_38[0]))),
                ((((((((x_sum_tmp_715f4_37[0]) * (y_sum_tmp_715f4_38[6]))
                    + ((x_sum_tmp_715f4_37[1]) * (y_sum_tmp_715f4_38[5])))
                    + ((x_sum_tmp_715f4_37[2]) * (y_sum_tmp_715f4_38[4])))
                    + ((x_sum_tmp_715f4_37[3]) * (y_sum_tmp_715f4_38[3])))
                    + ((x_sum_tmp_715f4_37[4]) * (y_sum_tmp_715f4_38[2])))
                    + ((x_sum_tmp_715f4_37[5]) * (y_sum_tmp_715f4_38[1])))
                    + ((x_sum_tmp_715f4_37[6]) * (y_sum_tmp_715f4_38[0]))),
                (((((((x_sum_tmp_715f4_37[1]) * (y_sum_tmp_715f4_38[6]))
                    + ((x_sum_tmp_715f4_37[2]) * (y_sum_tmp_715f4_38[5])))
                    + ((x_sum_tmp_715f4_37[3]) * (y_sum_tmp_715f4_38[4])))
                    + ((x_sum_tmp_715f4_37[4]) * (y_sum_tmp_715f4_38[3])))
                    + ((x_sum_tmp_715f4_37[5]) * (y_sum_tmp_715f4_38[2])))
                    + ((x_sum_tmp_715f4_37[6]) * (y_sum_tmp_715f4_38[1]))),
                ((((((x_sum_tmp_715f4_37[2]) * (y_sum_tmp_715f4_38[6]))
                    + ((x_sum_tmp_715f4_37[3]) * (y_sum_tmp_715f4_38[5])))
                    + ((x_sum_tmp_715f4_37[4]) * (y_sum_tmp_715f4_38[4])))
                    + ((x_sum_tmp_715f4_37[5]) * (y_sum_tmp_715f4_38[3])))
                    + ((x_sum_tmp_715f4_37[6]) * (y_sum_tmp_715f4_38[2]))),
                (((((x_sum_tmp_715f4_37[3]) * (y_sum_tmp_715f4_38[6]))
                    + ((x_sum_tmp_715f4_37[4]) * (y_sum_tmp_715f4_38[5])))
                    + ((x_sum_tmp_715f4_37[5]) * (y_sum_tmp_715f4_38[4])))
                    + ((x_sum_tmp_715f4_37[6]) * (y_sum_tmp_715f4_38[3]))),
                ((((x_sum_tmp_715f4_37[4]) * (y_sum_tmp_715f4_38[6]))
                    + ((x_sum_tmp_715f4_37[5]) * (y_sum_tmp_715f4_38[5])))
                    + ((x_sum_tmp_715f4_37[6]) * (y_sum_tmp_715f4_38[4]))),
                (((x_sum_tmp_715f4_37[5]) * (y_sum_tmp_715f4_38[6]))
                    + ((x_sum_tmp_715f4_37[6]) * (y_sum_tmp_715f4_38[5]))),
                ((x_sum_tmp_715f4_37[6]) * (y_sum_tmp_715f4_38[6])),
            ];
            let z2_tmp_715f4_40 = [
                ((x_sum_tmp_715f4_37[7]) * (y_sum_tmp_715f4_38[7])),
                (((x_sum_tmp_715f4_37[7]) * (y_sum_tmp_715f4_38[8]))
                    + ((x_sum_tmp_715f4_37[8]) * (y_sum_tmp_715f4_38[7]))),
                ((((x_sum_tmp_715f4_37[7]) * (y_sum_tmp_715f4_38[9]))
                    + ((x_sum_tmp_715f4_37[8]) * (y_sum_tmp_715f4_38[8])))
                    + ((x_sum_tmp_715f4_37[9]) * (y_sum_tmp_715f4_38[7]))),
                (((((x_sum_tmp_715f4_37[7]) * (y_sum_tmp_715f4_38[10]))
                    + ((x_sum_tmp_715f4_37[8]) * (y_sum_tmp_715f4_38[9])))
                    + ((x_sum_tmp_715f4_37[9]) * (y_sum_tmp_715f4_38[8])))
                    + ((x_sum_tmp_715f4_37[10]) * (y_sum_tmp_715f4_38[7]))),
                ((((((x_sum_tmp_715f4_37[7]) * (y_sum_tmp_715f4_38[11]))
                    + ((x_sum_tmp_715f4_37[8]) * (y_sum_tmp_715f4_38[10])))
                    + ((x_sum_tmp_715f4_37[9]) * (y_sum_tmp_715f4_38[9])))
                    + ((x_sum_tmp_715f4_37[10]) * (y_sum_tmp_715f4_38[8])))
                    + ((x_sum_tmp_715f4_37[11]) * (y_sum_tmp_715f4_38[7]))),
                (((((((x_sum_tmp_715f4_37[7]) * (y_sum_tmp_715f4_38[12]))
                    + ((x_sum_tmp_715f4_37[8]) * (y_sum_tmp_715f4_38[11])))
                    + ((x_sum_tmp_715f4_37[9]) * (y_sum_tmp_715f4_38[10])))
                    + ((x_sum_tmp_715f4_37[10]) * (y_sum_tmp_715f4_38[9])))
                    + ((x_sum_tmp_715f4_37[11]) * (y_sum_tmp_715f4_38[8])))
                    + ((x_sum_tmp_715f4_37[12]) * (y_sum_tmp_715f4_38[7]))),
                ((((((((x_sum_tmp_715f4_37[7]) * (y_sum_tmp_715f4_38[13]))
                    + ((x_sum_tmp_715f4_37[8]) * (y_sum_tmp_715f4_38[12])))
                    + ((x_sum_tmp_715f4_37[9]) * (y_sum_tmp_715f4_38[11])))
                    + ((x_sum_tmp_715f4_37[10]) * (y_sum_tmp_715f4_38[10])))
                    + ((x_sum_tmp_715f4_37[11]) * (y_sum_tmp_715f4_38[9])))
                    + ((x_sum_tmp_715f4_37[12]) * (y_sum_tmp_715f4_38[8])))
                    + ((x_sum_tmp_715f4_37[13]) * (y_sum_tmp_715f4_38[7]))),
                (((((((x_sum_tmp_715f4_37[8]) * (y_sum_tmp_715f4_38[13]))
                    + ((x_sum_tmp_715f4_37[9]) * (y_sum_tmp_715f4_38[12])))
                    + ((x_sum_tmp_715f4_37[10]) * (y_sum_tmp_715f4_38[11])))
                    + ((x_sum_tmp_715f4_37[11]) * (y_sum_tmp_715f4_38[10])))
                    + ((x_sum_tmp_715f4_37[12]) * (y_sum_tmp_715f4_38[9])))
                    + ((x_sum_tmp_715f4_37[13]) * (y_sum_tmp_715f4_38[8]))),
                ((((((x_sum_tmp_715f4_37[9]) * (y_sum_tmp_715f4_38[13]))
                    + ((x_sum_tmp_715f4_37[10]) * (y_sum_tmp_715f4_38[12])))
                    + ((x_sum_tmp_715f4_37[11]) * (y_sum_tmp_715f4_38[11])))
                    + ((x_sum_tmp_715f4_37[12]) * (y_sum_tmp_715f4_38[10])))
                    + ((x_sum_tmp_715f4_37[13]) * (y_sum_tmp_715f4_38[9]))),
                (((((x_sum_tmp_715f4_37[10]) * (y_sum_tmp_715f4_38[13]))
                    + ((x_sum_tmp_715f4_37[11]) * (y_sum_tmp_715f4_38[12])))
                    + ((x_sum_tmp_715f4_37[12]) * (y_sum_tmp_715f4_38[11])))
                    + ((x_sum_tmp_715f4_37[13]) * (y_sum_tmp_715f4_38[10]))),
                ((((x_sum_tmp_715f4_37[11]) * (y_sum_tmp_715f4_38[13]))
                    + ((x_sum_tmp_715f4_37[12]) * (y_sum_tmp_715f4_38[12])))
                    + ((x_sum_tmp_715f4_37[13]) * (y_sum_tmp_715f4_38[11]))),
                (((x_sum_tmp_715f4_37[12]) * (y_sum_tmp_715f4_38[13]))
                    + ((x_sum_tmp_715f4_37[13]) * (y_sum_tmp_715f4_38[12]))),
                ((x_sum_tmp_715f4_37[13]) * (y_sum_tmp_715f4_38[13])),
            ];
            let x_sum_tmp_715f4_41 = [
                ((x_sum_tmp_715f4_37[0]) + (x_sum_tmp_715f4_37[7])),
                ((x_sum_tmp_715f4_37[1]) + (x_sum_tmp_715f4_37[8])),
                ((x_sum_tmp_715f4_37[2]) + (x_sum_tmp_715f4_37[9])),
                ((x_sum_tmp_715f4_37[3]) + (x_sum_tmp_715f4_37[10])),
                ((x_sum_tmp_715f4_37[4]) + (x_sum_tmp_715f4_37[11])),
                ((x_sum_tmp_715f4_37[5]) + (x_sum_tmp_715f4_37[12])),
                ((x_sum_tmp_715f4_37[6]) + (x_sum_tmp_715f4_37[13])),
            ];
            let y_sum_tmp_715f4_42 = [
                ((y_sum_tmp_715f4_38[0]) + (y_sum_tmp_715f4_38[7])),
                ((y_sum_tmp_715f4_38[1]) + (y_sum_tmp_715f4_38[8])),
                ((y_sum_tmp_715f4_38[2]) + (y_sum_tmp_715f4_38[9])),
                ((y_sum_tmp_715f4_38[3]) + (y_sum_tmp_715f4_38[10])),
                ((y_sum_tmp_715f4_38[4]) + (y_sum_tmp_715f4_38[11])),
                ((y_sum_tmp_715f4_38[5]) + (y_sum_tmp_715f4_38[12])),
                ((y_sum_tmp_715f4_38[6]) + (y_sum_tmp_715f4_38[13])),
            ];
            let single_karatsuba_n_7_output_tmp_715f4_43 = [
                z0_tmp_715f4_39[0],
                z0_tmp_715f4_39[1],
                z0_tmp_715f4_39[2],
                z0_tmp_715f4_39[3],
                z0_tmp_715f4_39[4],
                z0_tmp_715f4_39[5],
                z0_tmp_715f4_39[6],
                ((z0_tmp_715f4_39[7])
                    + ((((x_sum_tmp_715f4_41[0]) * (y_sum_tmp_715f4_42[0]))
                        - (z0_tmp_715f4_39[0]))
                        - (z2_tmp_715f4_40[0]))),
                ((z0_tmp_715f4_39[8])
                    + (((((x_sum_tmp_715f4_41[0]) * (y_sum_tmp_715f4_42[1]))
                        + ((x_sum_tmp_715f4_41[1]) * (y_sum_tmp_715f4_42[0])))
                        - (z0_tmp_715f4_39[1]))
                        - (z2_tmp_715f4_40[1]))),
                ((z0_tmp_715f4_39[9])
                    + ((((((x_sum_tmp_715f4_41[0]) * (y_sum_tmp_715f4_42[2]))
                        + ((x_sum_tmp_715f4_41[1]) * (y_sum_tmp_715f4_42[1])))
                        + ((x_sum_tmp_715f4_41[2]) * (y_sum_tmp_715f4_42[0])))
                        - (z0_tmp_715f4_39[2]))
                        - (z2_tmp_715f4_40[2]))),
                ((z0_tmp_715f4_39[10])
                    + (((((((x_sum_tmp_715f4_41[0]) * (y_sum_tmp_715f4_42[3]))
                        + ((x_sum_tmp_715f4_41[1]) * (y_sum_tmp_715f4_42[2])))
                        + ((x_sum_tmp_715f4_41[2]) * (y_sum_tmp_715f4_42[1])))
                        + ((x_sum_tmp_715f4_41[3]) * (y_sum_tmp_715f4_42[0])))
                        - (z0_tmp_715f4_39[3]))
                        - (z2_tmp_715f4_40[3]))),
                ((z0_tmp_715f4_39[11])
                    + ((((((((x_sum_tmp_715f4_41[0]) * (y_sum_tmp_715f4_42[4]))
                        + ((x_sum_tmp_715f4_41[1]) * (y_sum_tmp_715f4_42[3])))
                        + ((x_sum_tmp_715f4_41[2]) * (y_sum_tmp_715f4_42[2])))
                        + ((x_sum_tmp_715f4_41[3]) * (y_sum_tmp_715f4_42[1])))
                        + ((x_sum_tmp_715f4_41[4]) * (y_sum_tmp_715f4_42[0])))
                        - (z0_tmp_715f4_39[4]))
                        - (z2_tmp_715f4_40[4]))),
                ((z0_tmp_715f4_39[12])
                    + (((((((((x_sum_tmp_715f4_41[0]) * (y_sum_tmp_715f4_42[5]))
                        + ((x_sum_tmp_715f4_41[1]) * (y_sum_tmp_715f4_42[4])))
                        + ((x_sum_tmp_715f4_41[2]) * (y_sum_tmp_715f4_42[3])))
                        + ((x_sum_tmp_715f4_41[3]) * (y_sum_tmp_715f4_42[2])))
                        + ((x_sum_tmp_715f4_41[4]) * (y_sum_tmp_715f4_42[1])))
                        + ((x_sum_tmp_715f4_41[5]) * (y_sum_tmp_715f4_42[0])))
                        - (z0_tmp_715f4_39[5]))
                        - (z2_tmp_715f4_40[5]))),
                ((((((((((x_sum_tmp_715f4_41[0]) * (y_sum_tmp_715f4_42[6]))
                    + ((x_sum_tmp_715f4_41[1]) * (y_sum_tmp_715f4_42[5])))
                    + ((x_sum_tmp_715f4_41[2]) * (y_sum_tmp_715f4_42[4])))
                    + ((x_sum_tmp_715f4_41[3]) * (y_sum_tmp_715f4_42[3])))
                    + ((x_sum_tmp_715f4_41[4]) * (y_sum_tmp_715f4_42[2])))
                    + ((x_sum_tmp_715f4_41[5]) * (y_sum_tmp_715f4_42[1])))
                    + ((x_sum_tmp_715f4_41[6]) * (y_sum_tmp_715f4_42[0])))
                    - (z0_tmp_715f4_39[6]))
                    - (z2_tmp_715f4_40[6])),
                ((z2_tmp_715f4_40[0])
                    + (((((((((x_sum_tmp_715f4_41[1]) * (y_sum_tmp_715f4_42[6]))
                        + ((x_sum_tmp_715f4_41[2]) * (y_sum_tmp_715f4_42[5])))
                        + ((x_sum_tmp_715f4_41[3]) * (y_sum_tmp_715f4_42[4])))
                        + ((x_sum_tmp_715f4_41[4]) * (y_sum_tmp_715f4_42[3])))
                        + ((x_sum_tmp_715f4_41[5]) * (y_sum_tmp_715f4_42[2])))
                        + ((x_sum_tmp_715f4_41[6]) * (y_sum_tmp_715f4_42[1])))
                        - (z0_tmp_715f4_39[7]))
                        - (z2_tmp_715f4_40[7]))),
                ((z2_tmp_715f4_40[1])
                    + ((((((((x_sum_tmp_715f4_41[2]) * (y_sum_tmp_715f4_42[6]))
                        + ((x_sum_tmp_715f4_41[3]) * (y_sum_tmp_715f4_42[5])))
                        + ((x_sum_tmp_715f4_41[4]) * (y_sum_tmp_715f4_42[4])))
                        + ((x_sum_tmp_715f4_41[5]) * (y_sum_tmp_715f4_42[3])))
                        + ((x_sum_tmp_715f4_41[6]) * (y_sum_tmp_715f4_42[2])))
                        - (z0_tmp_715f4_39[8]))
                        - (z2_tmp_715f4_40[8]))),
                ((z2_tmp_715f4_40[2])
                    + (((((((x_sum_tmp_715f4_41[3]) * (y_sum_tmp_715f4_42[6]))
                        + ((x_sum_tmp_715f4_41[4]) * (y_sum_tmp_715f4_42[5])))
                        + ((x_sum_tmp_715f4_41[5]) * (y_sum_tmp_715f4_42[4])))
                        + ((x_sum_tmp_715f4_41[6]) * (y_sum_tmp_715f4_42[3])))
                        - (z0_tmp_715f4_39[9]))
                        - (z2_tmp_715f4_40[9]))),
                ((z2_tmp_715f4_40[3])
                    + ((((((x_sum_tmp_715f4_41[4]) * (y_sum_tmp_715f4_42[6]))
                        + ((x_sum_tmp_715f4_41[5]) * (y_sum_tmp_715f4_42[5])))
                        + ((x_sum_tmp_715f4_41[6]) * (y_sum_tmp_715f4_42[4])))
                        - (z0_tmp_715f4_39[10]))
                        - (z2_tmp_715f4_40[10]))),
                ((z2_tmp_715f4_40[4])
                    + (((((x_sum_tmp_715f4_41[5]) * (y_sum_tmp_715f4_42[6]))
                        + ((x_sum_tmp_715f4_41[6]) * (y_sum_tmp_715f4_42[5])))
                        - (z0_tmp_715f4_39[11]))
                        - (z2_tmp_715f4_40[11]))),
                ((z2_tmp_715f4_40[5])
                    + ((((x_sum_tmp_715f4_41[6]) * (y_sum_tmp_715f4_42[6]))
                        - (z0_tmp_715f4_39[12]))
                        - (z2_tmp_715f4_40[12]))),
                z2_tmp_715f4_40[6],
                z2_tmp_715f4_40[7],
                z2_tmp_715f4_40[8],
                z2_tmp_715f4_40[9],
                z2_tmp_715f4_40[10],
                z2_tmp_715f4_40[11],
                z2_tmp_715f4_40[12],
            ];

            let double_karatsuba_f0fc6_output_tmp_715f4_44 = [
                single_karatsuba_n_7_output_tmp_715f4_31[0],
                single_karatsuba_n_7_output_tmp_715f4_31[1],
                single_karatsuba_n_7_output_tmp_715f4_31[2],
                single_karatsuba_n_7_output_tmp_715f4_31[3],
                single_karatsuba_n_7_output_tmp_715f4_31[4],
                single_karatsuba_n_7_output_tmp_715f4_31[5],
                single_karatsuba_n_7_output_tmp_715f4_31[6],
                single_karatsuba_n_7_output_tmp_715f4_31[7],
                single_karatsuba_n_7_output_tmp_715f4_31[8],
                single_karatsuba_n_7_output_tmp_715f4_31[9],
                single_karatsuba_n_7_output_tmp_715f4_31[10],
                single_karatsuba_n_7_output_tmp_715f4_31[11],
                single_karatsuba_n_7_output_tmp_715f4_31[12],
                single_karatsuba_n_7_output_tmp_715f4_31[13],
                ((single_karatsuba_n_7_output_tmp_715f4_31[14])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[0])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[0]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[0]))),
                ((single_karatsuba_n_7_output_tmp_715f4_31[15])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[1])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[1]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[1]))),
                ((single_karatsuba_n_7_output_tmp_715f4_31[16])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[2])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[2]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[2]))),
                ((single_karatsuba_n_7_output_tmp_715f4_31[17])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[3])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[3]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[3]))),
                ((single_karatsuba_n_7_output_tmp_715f4_31[18])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[4])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[4]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[4]))),
                ((single_karatsuba_n_7_output_tmp_715f4_31[19])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[5])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[5]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[5]))),
                ((single_karatsuba_n_7_output_tmp_715f4_31[20])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[6])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[6]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[6]))),
                ((single_karatsuba_n_7_output_tmp_715f4_31[21])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[7])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[7]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[7]))),
                ((single_karatsuba_n_7_output_tmp_715f4_31[22])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[8])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[8]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[8]))),
                ((single_karatsuba_n_7_output_tmp_715f4_31[23])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[9])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[9]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[9]))),
                ((single_karatsuba_n_7_output_tmp_715f4_31[24])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[10])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[10]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[10]))),
                ((single_karatsuba_n_7_output_tmp_715f4_31[25])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[11])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[11]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[11]))),
                ((single_karatsuba_n_7_output_tmp_715f4_31[26])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[12])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[12]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[12]))),
                (((single_karatsuba_n_7_output_tmp_715f4_43[13])
                    - (single_karatsuba_n_7_output_tmp_715f4_31[13]))
                    - (single_karatsuba_n_7_output_tmp_715f4_36[13])),
                ((single_karatsuba_n_7_output_tmp_715f4_36[0])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[14])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[14]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[14]))),
                ((single_karatsuba_n_7_output_tmp_715f4_36[1])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[15])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[15]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[15]))),
                ((single_karatsuba_n_7_output_tmp_715f4_36[2])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[16])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[16]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[16]))),
                ((single_karatsuba_n_7_output_tmp_715f4_36[3])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[17])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[17]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[17]))),
                ((single_karatsuba_n_7_output_tmp_715f4_36[4])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[18])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[18]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[18]))),
                ((single_karatsuba_n_7_output_tmp_715f4_36[5])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[19])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[19]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[19]))),
                ((single_karatsuba_n_7_output_tmp_715f4_36[6])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[20])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[20]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[20]))),
                ((single_karatsuba_n_7_output_tmp_715f4_36[7])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[21])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[21]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[21]))),
                ((single_karatsuba_n_7_output_tmp_715f4_36[8])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[22])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[22]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[22]))),
                ((single_karatsuba_n_7_output_tmp_715f4_36[9])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[23])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[23]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[23]))),
                ((single_karatsuba_n_7_output_tmp_715f4_36[10])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[24])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[24]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[24]))),
                ((single_karatsuba_n_7_output_tmp_715f4_36[11])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[25])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[25]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[25]))),
                ((single_karatsuba_n_7_output_tmp_715f4_36[12])
                    + (((single_karatsuba_n_7_output_tmp_715f4_43[26])
                        - (single_karatsuba_n_7_output_tmp_715f4_31[26]))
                        - (single_karatsuba_n_7_output_tmp_715f4_36[26]))),
                single_karatsuba_n_7_output_tmp_715f4_36[13],
                single_karatsuba_n_7_output_tmp_715f4_36[14],
                single_karatsuba_n_7_output_tmp_715f4_36[15],
                single_karatsuba_n_7_output_tmp_715f4_36[16],
                single_karatsuba_n_7_output_tmp_715f4_36[17],
                single_karatsuba_n_7_output_tmp_715f4_36[18],
                single_karatsuba_n_7_output_tmp_715f4_36[19],
                single_karatsuba_n_7_output_tmp_715f4_36[20],
                single_karatsuba_n_7_output_tmp_715f4_36[21],
                single_karatsuba_n_7_output_tmp_715f4_36[22],
                single_karatsuba_n_7_output_tmp_715f4_36[23],
                single_karatsuba_n_7_output_tmp_715f4_36[24],
                single_karatsuba_n_7_output_tmp_715f4_36[25],
                single_karatsuba_n_7_output_tmp_715f4_36[26],
            ];

            let conv_tmp_715f4_45 = [
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[0]) - (mul_res_limb_0_col85)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[1]) - (mul_res_limb_1_col86)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[2]) - (mul_res_limb_2_col87)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[3]) - (mul_res_limb_3_col88)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[4]) - (mul_res_limb_4_col89)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[5]) - (mul_res_limb_5_col90)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[6]) - (mul_res_limb_6_col91)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[7]) - (mul_res_limb_7_col92)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[8]) - (mul_res_limb_8_col93)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[9]) - (mul_res_limb_9_col94)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[10]) - (mul_res_limb_10_col95)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[11]) - (mul_res_limb_11_col96)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[12]) - (mul_res_limb_12_col97)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[13]) - (mul_res_limb_13_col98)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[14]) - (mul_res_limb_14_col99)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[15]) - (mul_res_limb_15_col100)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[16]) - (mul_res_limb_16_col101)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[17]) - (mul_res_limb_17_col102)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[18]) - (mul_res_limb_18_col103)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[19]) - (mul_res_limb_19_col104)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[20]) - (mul_res_limb_20_col105)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[21]) - (mul_res_limb_21_col106)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[22]) - (mul_res_limb_22_col107)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[23]) - (mul_res_limb_23_col108)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[24]) - (mul_res_limb_24_col109)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[25]) - (mul_res_limb_25_col110)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[26]) - (mul_res_limb_26_col111)),
                ((double_karatsuba_f0fc6_output_tmp_715f4_44[27]) - (mul_res_limb_27_col112)),
                double_karatsuba_f0fc6_output_tmp_715f4_44[28],
                double_karatsuba_f0fc6_output_tmp_715f4_44[29],
                double_karatsuba_f0fc6_output_tmp_715f4_44[30],
                double_karatsuba_f0fc6_output_tmp_715f4_44[31],
                double_karatsuba_f0fc6_output_tmp_715f4_44[32],
                double_karatsuba_f0fc6_output_tmp_715f4_44[33],
                double_karatsuba_f0fc6_output_tmp_715f4_44[34],
                double_karatsuba_f0fc6_output_tmp_715f4_44[35],
                double_karatsuba_f0fc6_output_tmp_715f4_44[36],
                double_karatsuba_f0fc6_output_tmp_715f4_44[37],
                double_karatsuba_f0fc6_output_tmp_715f4_44[38],
                double_karatsuba_f0fc6_output_tmp_715f4_44[39],
                double_karatsuba_f0fc6_output_tmp_715f4_44[40],
                double_karatsuba_f0fc6_output_tmp_715f4_44[41],
                double_karatsuba_f0fc6_output_tmp_715f4_44[42],
                double_karatsuba_f0fc6_output_tmp_715f4_44[43],
                double_karatsuba_f0fc6_output_tmp_715f4_44[44],
                double_karatsuba_f0fc6_output_tmp_715f4_44[45],
                double_karatsuba_f0fc6_output_tmp_715f4_44[46],
                double_karatsuba_f0fc6_output_tmp_715f4_44[47],
                double_karatsuba_f0fc6_output_tmp_715f4_44[48],
                double_karatsuba_f0fc6_output_tmp_715f4_44[49],
                double_karatsuba_f0fc6_output_tmp_715f4_44[50],
                double_karatsuba_f0fc6_output_tmp_715f4_44[51],
                double_karatsuba_f0fc6_output_tmp_715f4_44[52],
                double_karatsuba_f0fc6_output_tmp_715f4_44[53],
                double_karatsuba_f0fc6_output_tmp_715f4_44[54],
            ];
            let conv_mod_tmp_715f4_46 = [
                ((((M31_32) * (conv_tmp_715f4_45[0])) - ((M31_4) * (conv_tmp_715f4_45[21])))
                    + ((M31_8) * (conv_tmp_715f4_45[49]))),
                ((((conv_tmp_715f4_45[0]) + ((M31_32) * (conv_tmp_715f4_45[1])))
                    - ((M31_4) * (conv_tmp_715f4_45[22])))
                    + ((M31_8) * (conv_tmp_715f4_45[50]))),
                ((((conv_tmp_715f4_45[1]) + ((M31_32) * (conv_tmp_715f4_45[2])))
                    - ((M31_4) * (conv_tmp_715f4_45[23])))
                    + ((M31_8) * (conv_tmp_715f4_45[51]))),
                ((((conv_tmp_715f4_45[2]) + ((M31_32) * (conv_tmp_715f4_45[3])))
                    - ((M31_4) * (conv_tmp_715f4_45[24])))
                    + ((M31_8) * (conv_tmp_715f4_45[52]))),
                ((((conv_tmp_715f4_45[3]) + ((M31_32) * (conv_tmp_715f4_45[4])))
                    - ((M31_4) * (conv_tmp_715f4_45[25])))
                    + ((M31_8) * (conv_tmp_715f4_45[53]))),
                ((((conv_tmp_715f4_45[4]) + ((M31_32) * (conv_tmp_715f4_45[5])))
                    - ((M31_4) * (conv_tmp_715f4_45[26])))
                    + ((M31_8) * (conv_tmp_715f4_45[54]))),
                (((conv_tmp_715f4_45[5]) + ((M31_32) * (conv_tmp_715f4_45[6])))
                    - ((M31_4) * (conv_tmp_715f4_45[27]))),
                (((((M31_2) * (conv_tmp_715f4_45[0])) + (conv_tmp_715f4_45[6]))
                    + ((M31_32) * (conv_tmp_715f4_45[7])))
                    - ((M31_4) * (conv_tmp_715f4_45[28]))),
                (((((M31_2) * (conv_tmp_715f4_45[1])) + (conv_tmp_715f4_45[7]))
                    + ((M31_32) * (conv_tmp_715f4_45[8])))
                    - ((M31_4) * (conv_tmp_715f4_45[29]))),
                (((((M31_2) * (conv_tmp_715f4_45[2])) + (conv_tmp_715f4_45[8]))
                    + ((M31_32) * (conv_tmp_715f4_45[9])))
                    - ((M31_4) * (conv_tmp_715f4_45[30]))),
                (((((M31_2) * (conv_tmp_715f4_45[3])) + (conv_tmp_715f4_45[9]))
                    + ((M31_32) * (conv_tmp_715f4_45[10])))
                    - ((M31_4) * (conv_tmp_715f4_45[31]))),
                (((((M31_2) * (conv_tmp_715f4_45[4])) + (conv_tmp_715f4_45[10]))
                    + ((M31_32) * (conv_tmp_715f4_45[11])))
                    - ((M31_4) * (conv_tmp_715f4_45[32]))),
                (((((M31_2) * (conv_tmp_715f4_45[5])) + (conv_tmp_715f4_45[11]))
                    + ((M31_32) * (conv_tmp_715f4_45[12])))
                    - ((M31_4) * (conv_tmp_715f4_45[33]))),
                (((((M31_2) * (conv_tmp_715f4_45[6])) + (conv_tmp_715f4_45[12]))
                    + ((M31_32) * (conv_tmp_715f4_45[13])))
                    - ((M31_4) * (conv_tmp_715f4_45[34]))),
                (((((M31_2) * (conv_tmp_715f4_45[7])) + (conv_tmp_715f4_45[13]))
                    + ((M31_32) * (conv_tmp_715f4_45[14])))
                    - ((M31_4) * (conv_tmp_715f4_45[35]))),
                (((((M31_2) * (conv_tmp_715f4_45[8])) + (conv_tmp_715f4_45[14]))
                    + ((M31_32) * (conv_tmp_715f4_45[15])))
                    - ((M31_4) * (conv_tmp_715f4_45[36]))),
                (((((M31_2) * (conv_tmp_715f4_45[9])) + (conv_tmp_715f4_45[15]))
                    + ((M31_32) * (conv_tmp_715f4_45[16])))
                    - ((M31_4) * (conv_tmp_715f4_45[37]))),
                (((((M31_2) * (conv_tmp_715f4_45[10])) + (conv_tmp_715f4_45[16]))
                    + ((M31_32) * (conv_tmp_715f4_45[17])))
                    - ((M31_4) * (conv_tmp_715f4_45[38]))),
                (((((M31_2) * (conv_tmp_715f4_45[11])) + (conv_tmp_715f4_45[17]))
                    + ((M31_32) * (conv_tmp_715f4_45[18])))
                    - ((M31_4) * (conv_tmp_715f4_45[39]))),
                (((((M31_2) * (conv_tmp_715f4_45[12])) + (conv_tmp_715f4_45[18]))
                    + ((M31_32) * (conv_tmp_715f4_45[19])))
                    - ((M31_4) * (conv_tmp_715f4_45[40]))),
                (((((M31_2) * (conv_tmp_715f4_45[13])) + (conv_tmp_715f4_45[19]))
                    + ((M31_32) * (conv_tmp_715f4_45[20])))
                    - ((M31_4) * (conv_tmp_715f4_45[41]))),
                (((((M31_2) * (conv_tmp_715f4_45[14])) + (conv_tmp_715f4_45[20]))
                    - ((M31_4) * (conv_tmp_715f4_45[42])))
                    + ((M31_64) * (conv_tmp_715f4_45[49]))),
                (((((M31_2) * (conv_tmp_715f4_45[15])) - ((M31_4) * (conv_tmp_715f4_45[43])))
                    + ((M31_2) * (conv_tmp_715f4_45[49])))
                    + ((M31_64) * (conv_tmp_715f4_45[50]))),
                (((((M31_2) * (conv_tmp_715f4_45[16])) - ((M31_4) * (conv_tmp_715f4_45[44])))
                    + ((M31_2) * (conv_tmp_715f4_45[50])))
                    + ((M31_64) * (conv_tmp_715f4_45[51]))),
                (((((M31_2) * (conv_tmp_715f4_45[17])) - ((M31_4) * (conv_tmp_715f4_45[45])))
                    + ((M31_2) * (conv_tmp_715f4_45[51])))
                    + ((M31_64) * (conv_tmp_715f4_45[52]))),
                (((((M31_2) * (conv_tmp_715f4_45[18])) - ((M31_4) * (conv_tmp_715f4_45[46])))
                    + ((M31_2) * (conv_tmp_715f4_45[52])))
                    + ((M31_64) * (conv_tmp_715f4_45[53]))),
                (((((M31_2) * (conv_tmp_715f4_45[19])) - ((M31_4) * (conv_tmp_715f4_45[47])))
                    + ((M31_2) * (conv_tmp_715f4_45[53])))
                    + ((M31_64) * (conv_tmp_715f4_45[54]))),
                ((((M31_2) * (conv_tmp_715f4_45[20])) - ((M31_4) * (conv_tmp_715f4_45[48])))
                    + ((M31_2) * (conv_tmp_715f4_45[54]))),
            ];
            let k_mod_2_18_biased_tmp_715f4_47 =
                ((((PackedUInt32::from_m31(((conv_mod_tmp_715f4_46[0]) + (M31_134217728))))
                    + (((PackedUInt32::from_m31(
                        ((conv_mod_tmp_715f4_46[1]) + (M31_134217728)),
                    )) & (UInt32_511))
                        << (UInt32_9)))
                    + (UInt32_131072))
                    & (UInt32_262143));
            let k_col113 = ((k_mod_2_18_biased_tmp_715f4_47.low().as_m31())
                + (((k_mod_2_18_biased_tmp_715f4_47.high().as_m31()) - (M31_2)) * (M31_65536)));
            *row[113] = k_col113;
            *sub_component_inputs.range_check_20[4] = [((k_col113) + (M31_524288))];
            *lookup_data.range_check_20_70 = [M31_1410849886, ((k_col113) + (M31_524288))];
            let carry_0_col114 = (((conv_mod_tmp_715f4_46[0]) - (k_col113)) * (M31_4194304));
            *row[114] = carry_0_col114;
            *sub_component_inputs.range_check_20_b[4] = [((carry_0_col114) + (M31_524288))];
            *lookup_data.range_check_20_b_71 = [M31_514232941, ((carry_0_col114) + (M31_524288))];
            let carry_1_col115 = (((conv_mod_tmp_715f4_46[1]) + (carry_0_col114)) * (M31_4194304));
            *row[115] = carry_1_col115;
            *sub_component_inputs.range_check_20_c[4] = [((carry_1_col115) + (M31_524288))];
            *lookup_data.range_check_20_c_72 = [M31_531010560, ((carry_1_col115) + (M31_524288))];
            let carry_2_col116 = (((conv_mod_tmp_715f4_46[2]) + (carry_1_col115)) * (M31_4194304));
            *row[116] = carry_2_col116;
            *sub_component_inputs.range_check_20_d[4] = [((carry_2_col116) + (M31_524288))];
            *lookup_data.range_check_20_d_73 = [M31_480677703, ((carry_2_col116) + (M31_524288))];
            let carry_3_col117 = (((conv_mod_tmp_715f4_46[3]) + (carry_2_col116)) * (M31_4194304));
            *row[117] = carry_3_col117;
            *sub_component_inputs.range_check_20_e[3] = [((carry_3_col117) + (M31_524288))];
            *lookup_data.range_check_20_e_74 = [M31_497455322, ((carry_3_col117) + (M31_524288))];
            let carry_4_col118 = (((conv_mod_tmp_715f4_46[4]) + (carry_3_col117)) * (M31_4194304));
            *row[118] = carry_4_col118;
            *sub_component_inputs.range_check_20_f[3] = [((carry_4_col118) + (M31_524288))];
            *lookup_data.range_check_20_f_75 = [M31_447122465, ((carry_4_col118) + (M31_524288))];
            let carry_5_col119 = (((conv_mod_tmp_715f4_46[5]) + (carry_4_col118)) * (M31_4194304));
            *row[119] = carry_5_col119;
            *sub_component_inputs.range_check_20_g[3] = [((carry_5_col119) + (M31_524288))];
            *lookup_data.range_check_20_g_76 = [M31_463900084, ((carry_5_col119) + (M31_524288))];
            let carry_6_col120 = (((conv_mod_tmp_715f4_46[6]) + (carry_5_col119)) * (M31_4194304));
            *row[120] = carry_6_col120;
            *sub_component_inputs.range_check_20_h[3] = [((carry_6_col120) + (M31_524288))];
            *lookup_data.range_check_20_h_77 = [M31_682009131, ((carry_6_col120) + (M31_524288))];
            let carry_7_col121 = (((conv_mod_tmp_715f4_46[7]) + (carry_6_col120)) * (M31_4194304));
            *row[121] = carry_7_col121;
            *sub_component_inputs.range_check_20[5] = [((carry_7_col121) + (M31_524288))];
            *lookup_data.range_check_20_78 = [M31_1410849886, ((carry_7_col121) + (M31_524288))];
            let carry_8_col122 = (((conv_mod_tmp_715f4_46[8]) + (carry_7_col121)) * (M31_4194304));
            *row[122] = carry_8_col122;
            *sub_component_inputs.range_check_20_b[5] = [((carry_8_col122) + (M31_524288))];
            *lookup_data.range_check_20_b_79 = [M31_514232941, ((carry_8_col122) + (M31_524288))];
            let carry_9_col123 = (((conv_mod_tmp_715f4_46[9]) + (carry_8_col122)) * (M31_4194304));
            *row[123] = carry_9_col123;
            *sub_component_inputs.range_check_20_c[5] = [((carry_9_col123) + (M31_524288))];
            *lookup_data.range_check_20_c_80 = [M31_531010560, ((carry_9_col123) + (M31_524288))];
            let carry_10_col124 =
                (((conv_mod_tmp_715f4_46[10]) + (carry_9_col123)) * (M31_4194304));
            *row[124] = carry_10_col124;
            *sub_component_inputs.range_check_20_d[5] = [((carry_10_col124) + (M31_524288))];
            *lookup_data.range_check_20_d_81 = [M31_480677703, ((carry_10_col124) + (M31_524288))];
            let carry_11_col125 =
                (((conv_mod_tmp_715f4_46[11]) + (carry_10_col124)) * (M31_4194304));
            *row[125] = carry_11_col125;
            *sub_component_inputs.range_check_20_e[4] = [((carry_11_col125) + (M31_524288))];
            *lookup_data.range_check_20_e_82 = [M31_497455322, ((carry_11_col125) + (M31_524288))];
            let carry_12_col126 =
                (((conv_mod_tmp_715f4_46[12]) + (carry_11_col125)) * (M31_4194304));
            *row[126] = carry_12_col126;
            *sub_component_inputs.range_check_20_f[4] = [((carry_12_col126) + (M31_524288))];
            *lookup_data.range_check_20_f_83 = [M31_447122465, ((carry_12_col126) + (M31_524288))];
            let carry_13_col127 =
                (((conv_mod_tmp_715f4_46[13]) + (carry_12_col126)) * (M31_4194304));
            *row[127] = carry_13_col127;
            *sub_component_inputs.range_check_20_g[4] = [((carry_13_col127) + (M31_524288))];
            *lookup_data.range_check_20_g_84 = [M31_463900084, ((carry_13_col127) + (M31_524288))];
            let carry_14_col128 =
                (((conv_mod_tmp_715f4_46[14]) + (carry_13_col127)) * (M31_4194304));
            *row[128] = carry_14_col128;
            *sub_component_inputs.range_check_20_h[4] = [((carry_14_col128) + (M31_524288))];
            *lookup_data.range_check_20_h_85 = [M31_682009131, ((carry_14_col128) + (M31_524288))];
            let carry_15_col129 =
                (((conv_mod_tmp_715f4_46[15]) + (carry_14_col128)) * (M31_4194304));
            *row[129] = carry_15_col129;
            *sub_component_inputs.range_check_20[6] = [((carry_15_col129) + (M31_524288))];
            *lookup_data.range_check_20_86 = [M31_1410849886, ((carry_15_col129) + (M31_524288))];
            let carry_16_col130 =
                (((conv_mod_tmp_715f4_46[16]) + (carry_15_col129)) * (M31_4194304));
            *row[130] = carry_16_col130;
            *sub_component_inputs.range_check_20_b[6] = [((carry_16_col130) + (M31_524288))];
            *lookup_data.range_check_20_b_87 = [M31_514232941, ((carry_16_col130) + (M31_524288))];
            let carry_17_col131 =
                (((conv_mod_tmp_715f4_46[17]) + (carry_16_col130)) * (M31_4194304));
            *row[131] = carry_17_col131;
            *sub_component_inputs.range_check_20_c[6] = [((carry_17_col131) + (M31_524288))];
            *lookup_data.range_check_20_c_88 = [M31_531010560, ((carry_17_col131) + (M31_524288))];
            let carry_18_col132 =
                (((conv_mod_tmp_715f4_46[18]) + (carry_17_col131)) * (M31_4194304));
            *row[132] = carry_18_col132;
            *sub_component_inputs.range_check_20_d[6] = [((carry_18_col132) + (M31_524288))];
            *lookup_data.range_check_20_d_89 = [M31_480677703, ((carry_18_col132) + (M31_524288))];
            let carry_19_col133 =
                (((conv_mod_tmp_715f4_46[19]) + (carry_18_col132)) * (M31_4194304));
            *row[133] = carry_19_col133;
            *sub_component_inputs.range_check_20_e[5] = [((carry_19_col133) + (M31_524288))];
            *lookup_data.range_check_20_e_90 = [M31_497455322, ((carry_19_col133) + (M31_524288))];
            let carry_20_col134 =
                (((conv_mod_tmp_715f4_46[20]) + (carry_19_col133)) * (M31_4194304));
            *row[134] = carry_20_col134;
            *sub_component_inputs.range_check_20_f[5] = [((carry_20_col134) + (M31_524288))];
            *lookup_data.range_check_20_f_91 = [M31_447122465, ((carry_20_col134) + (M31_524288))];
            let carry_21_col135 = ((((conv_mod_tmp_715f4_46[21]) - ((M31_136) * (k_col113)))
                + (carry_20_col134))
                * (M31_4194304));
            *row[135] = carry_21_col135;
            *sub_component_inputs.range_check_20_g[5] = [((carry_21_col135) + (M31_524288))];
            *lookup_data.range_check_20_g_92 = [M31_463900084, ((carry_21_col135) + (M31_524288))];
            let carry_22_col136 =
                (((conv_mod_tmp_715f4_46[22]) + (carry_21_col135)) * (M31_4194304));
            *row[136] = carry_22_col136;
            *sub_component_inputs.range_check_20_h[5] = [((carry_22_col136) + (M31_524288))];
            *lookup_data.range_check_20_h_93 = [M31_682009131, ((carry_22_col136) + (M31_524288))];
            let carry_23_col137 =
                (((conv_mod_tmp_715f4_46[23]) + (carry_22_col136)) * (M31_4194304));
            *row[137] = carry_23_col137;
            *sub_component_inputs.range_check_20[7] = [((carry_23_col137) + (M31_524288))];
            *lookup_data.range_check_20_94 = [M31_1410849886, ((carry_23_col137) + (M31_524288))];
            let carry_24_col138 =
                (((conv_mod_tmp_715f4_46[24]) + (carry_23_col137)) * (M31_4194304));
            *row[138] = carry_24_col138;
            *sub_component_inputs.range_check_20_b[7] = [((carry_24_col138) + (M31_524288))];
            *lookup_data.range_check_20_b_95 = [M31_514232941, ((carry_24_col138) + (M31_524288))];
            let carry_25_col139 =
                (((conv_mod_tmp_715f4_46[25]) + (carry_24_col138)) * (M31_4194304));
            *row[139] = carry_25_col139;
            *sub_component_inputs.range_check_20_c[7] = [((carry_25_col139) + (M31_524288))];
            *lookup_data.range_check_20_c_96 = [M31_531010560, ((carry_25_col139) + (M31_524288))];
            let carry_26_col140 =
                (((conv_mod_tmp_715f4_46[26]) + (carry_25_col139)) * (M31_4194304));
            *row[140] = carry_26_col140;
            *sub_component_inputs.range_check_20_d[7] = [((carry_26_col140) + (M31_524288))];
            *lookup_data.range_check_20_d_97 = [M31_480677703, ((carry_26_col140) + (M31_524288))];

            let mul_252_output_tmp_715f4_48 = mul_res_tmp_715f4_26;

            *lookup_data.cube_252_98 = [
                M31_1987997202,
                input_limb_0_col1,
                input_limb_1_col2,
                input_limb_2_col3,
                input_limb_3_col4,
                input_limb_4_col5,
                input_limb_5_col6,
                input_limb_6_col7,
                input_limb_7_col8,
                input_limb_8_col9,
                input_limb_9_col10,
                (((mul_res_limb_0_col85) + ((mul_res_limb_1_col86) * (M31_512)))
                    + ((mul_res_limb_2_col87) * (M31_262144))),
                (((mul_res_limb_3_col88) + ((mul_res_limb_4_col89) * (M31_512)))
                    + ((mul_res_limb_5_col90) * (M31_262144))),
                (((mul_res_limb_6_col91) + ((mul_res_limb_7_col92) * (M31_512)))
                    + ((mul_res_limb_8_col93) * (M31_262144))),
                (((mul_res_limb_9_col94) + ((mul_res_limb_10_col95) * (M31_512)))
                    + ((mul_res_limb_11_col96) * (M31_262144))),
                (((mul_res_limb_12_col97) + ((mul_res_limb_13_col98) * (M31_512)))
                    + ((mul_res_limb_14_col99) * (M31_262144))),
                (((mul_res_limb_15_col100) + ((mul_res_limb_16_col101) * (M31_512)))
                    + ((mul_res_limb_17_col102) * (M31_262144))),
                (((mul_res_limb_18_col103) + ((mul_res_limb_19_col104) * (M31_512)))
                    + ((mul_res_limb_20_col105) * (M31_262144))),
                (((mul_res_limb_21_col106) + ((mul_res_limb_22_col107) * (M31_512)))
                    + ((mul_res_limb_23_col108) * (M31_262144))),
                (((mul_res_limb_24_col109) + ((mul_res_limb_25_col110) * (M31_512)))
                    + ((mul_res_limb_26_col111) * (M31_262144))),
                mul_res_limb_27_col112,
            ];
            *lookup_data.mults_0 = enabler_col0;
        });

    (trace, lookup_data, sub_component_inputs)
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
    range_check_9_9_8: Vec<[PackedM31; 3]>,
    range_check_9_9_b_9: Vec<[PackedM31; 3]>,
    range_check_9_9_c_10: Vec<[PackedM31; 3]>,
    range_check_9_9_d_11: Vec<[PackedM31; 3]>,
    range_check_9_9_e_12: Vec<[PackedM31; 3]>,
    range_check_9_9_f_13: Vec<[PackedM31; 3]>,
    range_check_9_9_14: Vec<[PackedM31; 3]>,
    range_check_9_9_b_15: Vec<[PackedM31; 3]>,
    range_check_9_9_c_16: Vec<[PackedM31; 3]>,
    range_check_9_9_d_17: Vec<[PackedM31; 3]>,
    range_check_9_9_e_18: Vec<[PackedM31; 3]>,
    range_check_9_9_f_19: Vec<[PackedM31; 3]>,
    range_check_9_9_g_20: Vec<[PackedM31; 3]>,
    range_check_9_9_h_21: Vec<[PackedM31; 3]>,
    range_check_9_9_22: Vec<[PackedM31; 3]>,
    range_check_9_9_b_23: Vec<[PackedM31; 3]>,
    range_check_9_9_c_24: Vec<[PackedM31; 3]>,
    range_check_9_9_d_25: Vec<[PackedM31; 3]>,
    range_check_9_9_e_26: Vec<[PackedM31; 3]>,
    range_check_9_9_f_27: Vec<[PackedM31; 3]>,
    range_check_20_28: Vec<[PackedM31; 2]>,
    range_check_20_b_29: Vec<[PackedM31; 2]>,
    range_check_20_c_30: Vec<[PackedM31; 2]>,
    range_check_20_d_31: Vec<[PackedM31; 2]>,
    range_check_20_e_32: Vec<[PackedM31; 2]>,
    range_check_20_f_33: Vec<[PackedM31; 2]>,
    range_check_20_g_34: Vec<[PackedM31; 2]>,
    range_check_20_h_35: Vec<[PackedM31; 2]>,
    range_check_20_36: Vec<[PackedM31; 2]>,
    range_check_20_b_37: Vec<[PackedM31; 2]>,
    range_check_20_c_38: Vec<[PackedM31; 2]>,
    range_check_20_d_39: Vec<[PackedM31; 2]>,
    range_check_20_e_40: Vec<[PackedM31; 2]>,
    range_check_20_f_41: Vec<[PackedM31; 2]>,
    range_check_20_g_42: Vec<[PackedM31; 2]>,
    range_check_20_h_43: Vec<[PackedM31; 2]>,
    range_check_20_44: Vec<[PackedM31; 2]>,
    range_check_20_b_45: Vec<[PackedM31; 2]>,
    range_check_20_c_46: Vec<[PackedM31; 2]>,
    range_check_20_d_47: Vec<[PackedM31; 2]>,
    range_check_20_e_48: Vec<[PackedM31; 2]>,
    range_check_20_f_49: Vec<[PackedM31; 2]>,
    range_check_20_g_50: Vec<[PackedM31; 2]>,
    range_check_20_h_51: Vec<[PackedM31; 2]>,
    range_check_20_52: Vec<[PackedM31; 2]>,
    range_check_20_b_53: Vec<[PackedM31; 2]>,
    range_check_20_c_54: Vec<[PackedM31; 2]>,
    range_check_20_d_55: Vec<[PackedM31; 2]>,
    range_check_9_9_56: Vec<[PackedM31; 3]>,
    range_check_9_9_b_57: Vec<[PackedM31; 3]>,
    range_check_9_9_c_58: Vec<[PackedM31; 3]>,
    range_check_9_9_d_59: Vec<[PackedM31; 3]>,
    range_check_9_9_e_60: Vec<[PackedM31; 3]>,
    range_check_9_9_f_61: Vec<[PackedM31; 3]>,
    range_check_9_9_g_62: Vec<[PackedM31; 3]>,
    range_check_9_9_h_63: Vec<[PackedM31; 3]>,
    range_check_9_9_64: Vec<[PackedM31; 3]>,
    range_check_9_9_b_65: Vec<[PackedM31; 3]>,
    range_check_9_9_c_66: Vec<[PackedM31; 3]>,
    range_check_9_9_d_67: Vec<[PackedM31; 3]>,
    range_check_9_9_e_68: Vec<[PackedM31; 3]>,
    range_check_9_9_f_69: Vec<[PackedM31; 3]>,
    range_check_20_70: Vec<[PackedM31; 2]>,
    range_check_20_b_71: Vec<[PackedM31; 2]>,
    range_check_20_c_72: Vec<[PackedM31; 2]>,
    range_check_20_d_73: Vec<[PackedM31; 2]>,
    range_check_20_e_74: Vec<[PackedM31; 2]>,
    range_check_20_f_75: Vec<[PackedM31; 2]>,
    range_check_20_g_76: Vec<[PackedM31; 2]>,
    range_check_20_h_77: Vec<[PackedM31; 2]>,
    range_check_20_78: Vec<[PackedM31; 2]>,
    range_check_20_b_79: Vec<[PackedM31; 2]>,
    range_check_20_c_80: Vec<[PackedM31; 2]>,
    range_check_20_d_81: Vec<[PackedM31; 2]>,
    range_check_20_e_82: Vec<[PackedM31; 2]>,
    range_check_20_f_83: Vec<[PackedM31; 2]>,
    range_check_20_g_84: Vec<[PackedM31; 2]>,
    range_check_20_h_85: Vec<[PackedM31; 2]>,
    range_check_20_86: Vec<[PackedM31; 2]>,
    range_check_20_b_87: Vec<[PackedM31; 2]>,
    range_check_20_c_88: Vec<[PackedM31; 2]>,
    range_check_20_d_89: Vec<[PackedM31; 2]>,
    range_check_20_e_90: Vec<[PackedM31; 2]>,
    range_check_20_f_91: Vec<[PackedM31; 2]>,
    range_check_20_g_92: Vec<[PackedM31; 2]>,
    range_check_20_h_93: Vec<[PackedM31; 2]>,
    range_check_20_94: Vec<[PackedM31; 2]>,
    range_check_20_b_95: Vec<[PackedM31; 2]>,
    range_check_20_c_96: Vec<[PackedM31; 2]>,
    range_check_20_d_97: Vec<[PackedM31; 2]>,
    cube_252_98: Vec<[PackedM31; 21]>,
    mults_0: Vec<PackedM31>,
}

pub struct InteractionClaimGenerator {
    log_size: u32,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        common_lookup_elements: &relations::CommonLookupElements,
    ) -> (Vec<CircleEvaluation<SimdBackend, M31, BitReversedOrder>>, InteractionClaim) {
        let mut logup_gen = unsafe { LogupTraceGenerator::uninitialized(self.log_size) };

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_0,
            &self.lookup_data.range_check_9_9_b_1,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_c_2,
            &self.lookup_data.range_check_9_9_d_3,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_e_4,
            &self.lookup_data.range_check_9_9_f_5,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_g_6,
            &self.lookup_data.range_check_9_9_h_7,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_8,
            &self.lookup_data.range_check_9_9_b_9,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_c_10,
            &self.lookup_data.range_check_9_9_d_11,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_e_12,
            &self.lookup_data.range_check_9_9_f_13,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_14,
            &self.lookup_data.range_check_9_9_b_15,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_c_16,
            &self.lookup_data.range_check_9_9_d_17,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_e_18,
            &self.lookup_data.range_check_9_9_f_19,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_g_20,
            &self.lookup_data.range_check_9_9_h_21,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_22,
            &self.lookup_data.range_check_9_9_b_23,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_c_24,
            &self.lookup_data.range_check_9_9_d_25,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_e_26,
            &self.lookup_data.range_check_9_9_f_27,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_28,
            &self.lookup_data.range_check_20_b_29,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_c_30,
            &self.lookup_data.range_check_20_d_31,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_e_32,
            &self.lookup_data.range_check_20_f_33,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_g_34,
            &self.lookup_data.range_check_20_h_35,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_36,
            &self.lookup_data.range_check_20_b_37,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_c_38,
            &self.lookup_data.range_check_20_d_39,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_e_40,
            &self.lookup_data.range_check_20_f_41,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_g_42,
            &self.lookup_data.range_check_20_h_43,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_44,
            &self.lookup_data.range_check_20_b_45,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_c_46,
            &self.lookup_data.range_check_20_d_47,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_e_48,
            &self.lookup_data.range_check_20_f_49,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_g_50,
            &self.lookup_data.range_check_20_h_51,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_52,
            &self.lookup_data.range_check_20_b_53,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_c_54,
            &self.lookup_data.range_check_20_d_55,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_56,
            &self.lookup_data.range_check_9_9_b_57,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_c_58,
            &self.lookup_data.range_check_9_9_d_59,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_e_60,
            &self.lookup_data.range_check_9_9_f_61,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_g_62,
            &self.lookup_data.range_check_9_9_h_63,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_64,
            &self.lookup_data.range_check_9_9_b_65,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_c_66,
            &self.lookup_data.range_check_9_9_d_67,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_e_68,
            &self.lookup_data.range_check_9_9_f_69,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_70,
            &self.lookup_data.range_check_20_b_71,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_c_72,
            &self.lookup_data.range_check_20_d_73,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_e_74,
            &self.lookup_data.range_check_20_f_75,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_g_76,
            &self.lookup_data.range_check_20_h_77,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_78,
            &self.lookup_data.range_check_20_b_79,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_c_80,
            &self.lookup_data.range_check_20_d_81,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_e_82,
            &self.lookup_data.range_check_20_f_83,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_g_84,
            &self.lookup_data.range_check_20_h_85,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_86,
            &self.lookup_data.range_check_20_b_87,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_c_88,
            &self.lookup_data.range_check_20_d_89,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_e_90,
            &self.lookup_data.range_check_20_f_91,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_g_92,
            &self.lookup_data.range_check_20_h_93,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_94,
            &self.lookup_data.range_check_20_b_95,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_c_96,
            &self.lookup_data.range_check_20_d_97,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 + denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(), &self.lookup_data.cube_252_98, self.lookup_data.mults_0)
            .into_par_iter()
            .for_each(|(writer, values, mult)| {
                let denom = common_lookup_elements.combine(values);
                writer.write_frac((-mult).into(), denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();

        (trace, InteractionClaim { claimed_sum })
    }
}
