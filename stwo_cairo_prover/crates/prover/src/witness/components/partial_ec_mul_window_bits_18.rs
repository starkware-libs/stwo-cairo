#![allow(unused_parens)]
use cairo_air::components::partial_ec_mul_window_bits_18::{
    Claim, InteractionClaim, N_TRACE_COLUMNS,
};

use crate::witness::components::{
    pedersen_points_table_window_bits_18, range_check_20, range_check_9_9,
};
use crate::witness::prelude::*;

pub type InputType = (M31, M31, ([M31; 14], [Felt252; 2]));
pub type PackedInputType = (PackedM31, PackedM31, ([PackedM31; 14], [PackedFelt252; 2]));

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
        pedersen_points_table_window_bits_18_state: &pedersen_points_table_window_bits_18::ClaimGenerator,
        range_check_9_9_state: &range_check_9_9::ClaimGenerator,
        range_check_20_state: &range_check_20::ClaimGenerator,
    ) -> (
        ComponentTrace<N_TRACE_COLUMNS>,
        Claim,
        InteractionClaimGenerator,
    ) {
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
        let pool = rayon::ThreadPoolBuilder::new()
            .stack_size(RAYON_THREAD_STACK_SIZE)
            .build()
            .unwrap();
        let (trace, lookup_data, sub_component_inputs) = pool.install(|| {
            write_trace_simd(
                packed_inputs,
                n_active_rows,
                pedersen_points_table_window_bits_18_state,
                range_check_9_9_state,
                range_check_20_state,
            )
        });
        for inputs in sub_component_inputs.pedersen_points_table_window_bits_18 {
            add_inputs(
                pedersen_points_table_window_bits_18_state,
                &inputs,
                n_active_rows,
                0,
            );
        }
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

        (
            trace,
            Claim { log_size },
            InteractionClaimGenerator {
                log_size,
                lookup_data,
            },
        )
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
    pedersen_points_table_window_bits_18:
        [Vec<pedersen_points_table_window_bits_18::PackedInputType>; 1],
    range_check_9_9: [Vec<range_check_9_9::PackedInputType>; 6],
    range_check_9_9_b: [Vec<range_check_9_9::PackedInputType>; 6],
    range_check_9_9_c: [Vec<range_check_9_9::PackedInputType>; 6],
    range_check_9_9_d: [Vec<range_check_9_9::PackedInputType>; 6],
    range_check_9_9_e: [Vec<range_check_9_9::PackedInputType>; 6],
    range_check_9_9_f: [Vec<range_check_9_9::PackedInputType>; 6],
    range_check_9_9_g: [Vec<range_check_9_9::PackedInputType>; 3],
    range_check_9_9_h: [Vec<range_check_9_9::PackedInputType>; 3],
    range_check_20: [Vec<range_check_20::PackedInputType>; 12],
    range_check_20_b: [Vec<range_check_20::PackedInputType>; 12],
    range_check_20_c: [Vec<range_check_20::PackedInputType>; 12],
    range_check_20_d: [Vec<range_check_20::PackedInputType>; 12],
    range_check_20_e: [Vec<range_check_20::PackedInputType>; 9],
    range_check_20_f: [Vec<range_check_20::PackedInputType>; 9],
    range_check_20_g: [Vec<range_check_20::PackedInputType>; 9],
    range_check_20_h: [Vec<range_check_20::PackedInputType>; 9],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    n_rows: usize,
    pedersen_points_table_window_bits_18_state: &pedersen_points_table_window_bits_18::ClaimGenerator,
    range_check_9_9_state: &range_check_9_9::ClaimGenerator,
    range_check_20_state: &range_check_20::ClaimGenerator,
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
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_134217728 = PackedM31::broadcast(M31::from(134217728));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_1410849886 = PackedM31::broadcast(M31::from(1410849886));
    let M31_1444721856 = PackedM31::broadcast(M31::from(1444721856));
    let M31_1621226978 = PackedM31::broadcast(M31::from(1621226978));
    let M31_1813904000 = PackedM31::broadcast(M31::from(1813904000));
    let M31_1830681619 = PackedM31::broadcast(M31::from(1830681619));
    let M31_1847459238 = PackedM31::broadcast(M31::from(1847459238));
    let M31_1864236857 = PackedM31::broadcast(M31::from(1864236857));
    let M31_1881014476 = PackedM31::broadcast(M31::from(1881014476));
    let M31_1897792095 = PackedM31::broadcast(M31::from(1897792095));
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
    let M31_514232941 = PackedM31::broadcast(M31::from(514232941));
    let M31_517791011 = PackedM31::broadcast(M31::from(517791011));
    let M31_524288 = PackedM31::broadcast(M31::from(524288));
    let M31_531010560 = PackedM31::broadcast(M31::from(531010560));
    let M31_64 = PackedM31::broadcast(M31::from(64));
    let M31_65536 = PackedM31::broadcast(M31::from(65536));
    let M31_682009131 = PackedM31::broadcast(M31::from(682009131));
    let M31_8 = PackedM31::broadcast(M31::from(8));
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
        .for_each(
            |(
                row_index,
                (row, lookup_data, sub_component_inputs, partial_ec_mul_window_bits_18_input),
            )| {
                let enabler_col0 = enabler_col.packed_at(row_index);
                *row[0] = enabler_col0;
                let input_limb_0_col1 = partial_ec_mul_window_bits_18_input.0;
                *row[1] = input_limb_0_col1;
                let input_limb_1_col2 = partial_ec_mul_window_bits_18_input.1;
                *row[2] = input_limb_1_col2;
                let input_limb_2_col3 = partial_ec_mul_window_bits_18_input.2 .0[0];
                *row[3] = input_limb_2_col3;
                let input_limb_3_col4 = partial_ec_mul_window_bits_18_input.2 .0[1];
                *row[4] = input_limb_3_col4;
                let input_limb_4_col5 = partial_ec_mul_window_bits_18_input.2 .0[2];
                *row[5] = input_limb_4_col5;
                let input_limb_5_col6 = partial_ec_mul_window_bits_18_input.2 .0[3];
                *row[6] = input_limb_5_col6;
                let input_limb_6_col7 = partial_ec_mul_window_bits_18_input.2 .0[4];
                *row[7] = input_limb_6_col7;
                let input_limb_7_col8 = partial_ec_mul_window_bits_18_input.2 .0[5];
                *row[8] = input_limb_7_col8;
                let input_limb_8_col9 = partial_ec_mul_window_bits_18_input.2 .0[6];
                *row[9] = input_limb_8_col9;
                let input_limb_9_col10 = partial_ec_mul_window_bits_18_input.2 .0[7];
                *row[10] = input_limb_9_col10;
                let input_limb_10_col11 = partial_ec_mul_window_bits_18_input.2 .0[8];
                *row[11] = input_limb_10_col11;
                let input_limb_11_col12 = partial_ec_mul_window_bits_18_input.2 .0[9];
                *row[12] = input_limb_11_col12;
                let input_limb_12_col13 = partial_ec_mul_window_bits_18_input.2 .0[10];
                *row[13] = input_limb_12_col13;
                let input_limb_13_col14 = partial_ec_mul_window_bits_18_input.2 .0[11];
                *row[14] = input_limb_13_col14;
                let input_limb_14_col15 = partial_ec_mul_window_bits_18_input.2 .0[12];
                *row[15] = input_limb_14_col15;
                let input_limb_15_col16 = partial_ec_mul_window_bits_18_input.2 .0[13];
                *row[16] = input_limb_15_col16;
                let input_limb_16_col17 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(0);
                *row[17] = input_limb_16_col17;
                let input_limb_17_col18 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(1);
                *row[18] = input_limb_17_col18;
                let input_limb_18_col19 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(2);
                *row[19] = input_limb_18_col19;
                let input_limb_19_col20 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(3);
                *row[20] = input_limb_19_col20;
                let input_limb_20_col21 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(4);
                *row[21] = input_limb_20_col21;
                let input_limb_21_col22 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(5);
                *row[22] = input_limb_21_col22;
                let input_limb_22_col23 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(6);
                *row[23] = input_limb_22_col23;
                let input_limb_23_col24 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(7);
                *row[24] = input_limb_23_col24;
                let input_limb_24_col25 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(8);
                *row[25] = input_limb_24_col25;
                let input_limb_25_col26 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(9);
                *row[26] = input_limb_25_col26;
                let input_limb_26_col27 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(10);
                *row[27] = input_limb_26_col27;
                let input_limb_27_col28 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(11);
                *row[28] = input_limb_27_col28;
                let input_limb_28_col29 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(12);
                *row[29] = input_limb_28_col29;
                let input_limb_29_col30 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(13);
                *row[30] = input_limb_29_col30;
                let input_limb_30_col31 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(14);
                *row[31] = input_limb_30_col31;
                let input_limb_31_col32 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(15);
                *row[32] = input_limb_31_col32;
                let input_limb_32_col33 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(16);
                *row[33] = input_limb_32_col33;
                let input_limb_33_col34 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(17);
                *row[34] = input_limb_33_col34;
                let input_limb_34_col35 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(18);
                *row[35] = input_limb_34_col35;
                let input_limb_35_col36 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(19);
                *row[36] = input_limb_35_col36;
                let input_limb_36_col37 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(20);
                *row[37] = input_limb_36_col37;
                let input_limb_37_col38 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(21);
                *row[38] = input_limb_37_col38;
                let input_limb_38_col39 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(22);
                *row[39] = input_limb_38_col39;
                let input_limb_39_col40 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(23);
                *row[40] = input_limb_39_col40;
                let input_limb_40_col41 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(24);
                *row[41] = input_limb_40_col41;
                let input_limb_41_col42 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(25);
                *row[42] = input_limb_41_col42;
                let input_limb_42_col43 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(26);
                *row[43] = input_limb_42_col43;
                let input_limb_43_col44 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(27);
                *row[44] = input_limb_43_col44;
                let input_limb_44_col45 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(0);
                *row[45] = input_limb_44_col45;
                let input_limb_45_col46 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(1);
                *row[46] = input_limb_45_col46;
                let input_limb_46_col47 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(2);
                *row[47] = input_limb_46_col47;
                let input_limb_47_col48 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(3);
                *row[48] = input_limb_47_col48;
                let input_limb_48_col49 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(4);
                *row[49] = input_limb_48_col49;
                let input_limb_49_col50 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(5);
                *row[50] = input_limb_49_col50;
                let input_limb_50_col51 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(6);
                *row[51] = input_limb_50_col51;
                let input_limb_51_col52 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(7);
                *row[52] = input_limb_51_col52;
                let input_limb_52_col53 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(8);
                *row[53] = input_limb_52_col53;
                let input_limb_53_col54 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(9);
                *row[54] = input_limb_53_col54;
                let input_limb_54_col55 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(10);
                *row[55] = input_limb_54_col55;
                let input_limb_55_col56 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(11);
                *row[56] = input_limb_55_col56;
                let input_limb_56_col57 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(12);
                *row[57] = input_limb_56_col57;
                let input_limb_57_col58 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(13);
                *row[58] = input_limb_57_col58;
                let input_limb_58_col59 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(14);
                *row[59] = input_limb_58_col59;
                let input_limb_59_col60 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(15);
                *row[60] = input_limb_59_col60;
                let input_limb_60_col61 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(16);
                *row[61] = input_limb_60_col61;
                let input_limb_61_col62 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(17);
                *row[62] = input_limb_61_col62;
                let input_limb_62_col63 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(18);
                *row[63] = input_limb_62_col63;
                let input_limb_63_col64 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(19);
                *row[64] = input_limb_63_col64;
                let input_limb_64_col65 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(20);
                *row[65] = input_limb_64_col65;
                let input_limb_65_col66 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(21);
                *row[66] = input_limb_65_col66;
                let input_limb_66_col67 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(22);
                *row[67] = input_limb_66_col67;
                let input_limb_67_col68 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(23);
                *row[68] = input_limb_67_col68;
                let input_limb_68_col69 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(24);
                *row[69] = input_limb_68_col69;
                let input_limb_69_col70 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(25);
                *row[70] = input_limb_69_col70;
                let input_limb_70_col71 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(26);
                *row[71] = input_limb_70_col71;
                let input_limb_71_col72 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(27);
                *row[72] = input_limb_71_col72;
                *sub_component_inputs.pedersen_points_table_window_bits_18[0] =
                    [(((M31_262144) * (input_limb_1_col2)) + (input_limb_2_col3))];
                let pedersen_points_table_window_bits_18_output_tmp_d9509_0 =
                    PackedPedersenPointsTableWindowBits18::deduce_output([(((M31_262144)
                        * (input_limb_1_col2))
                        + (input_limb_2_col3))]);
                let pedersen_points_table_window_bits_18_output_limb_0_col73 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(0);
                *row[73] = pedersen_points_table_window_bits_18_output_limb_0_col73;
                let pedersen_points_table_window_bits_18_output_limb_1_col74 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(1);
                *row[74] = pedersen_points_table_window_bits_18_output_limb_1_col74;
                let pedersen_points_table_window_bits_18_output_limb_2_col75 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(2);
                *row[75] = pedersen_points_table_window_bits_18_output_limb_2_col75;
                let pedersen_points_table_window_bits_18_output_limb_3_col76 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(3);
                *row[76] = pedersen_points_table_window_bits_18_output_limb_3_col76;
                let pedersen_points_table_window_bits_18_output_limb_4_col77 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(4);
                *row[77] = pedersen_points_table_window_bits_18_output_limb_4_col77;
                let pedersen_points_table_window_bits_18_output_limb_5_col78 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(5);
                *row[78] = pedersen_points_table_window_bits_18_output_limb_5_col78;
                let pedersen_points_table_window_bits_18_output_limb_6_col79 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(6);
                *row[79] = pedersen_points_table_window_bits_18_output_limb_6_col79;
                let pedersen_points_table_window_bits_18_output_limb_7_col80 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(7);
                *row[80] = pedersen_points_table_window_bits_18_output_limb_7_col80;
                let pedersen_points_table_window_bits_18_output_limb_8_col81 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(8);
                *row[81] = pedersen_points_table_window_bits_18_output_limb_8_col81;
                let pedersen_points_table_window_bits_18_output_limb_9_col82 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(9);
                *row[82] = pedersen_points_table_window_bits_18_output_limb_9_col82;
                let pedersen_points_table_window_bits_18_output_limb_10_col83 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(10);
                *row[83] = pedersen_points_table_window_bits_18_output_limb_10_col83;
                let pedersen_points_table_window_bits_18_output_limb_11_col84 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(11);
                *row[84] = pedersen_points_table_window_bits_18_output_limb_11_col84;
                let pedersen_points_table_window_bits_18_output_limb_12_col85 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(12);
                *row[85] = pedersen_points_table_window_bits_18_output_limb_12_col85;
                let pedersen_points_table_window_bits_18_output_limb_13_col86 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(13);
                *row[86] = pedersen_points_table_window_bits_18_output_limb_13_col86;
                let pedersen_points_table_window_bits_18_output_limb_14_col87 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(14);
                *row[87] = pedersen_points_table_window_bits_18_output_limb_14_col87;
                let pedersen_points_table_window_bits_18_output_limb_15_col88 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(15);
                *row[88] = pedersen_points_table_window_bits_18_output_limb_15_col88;
                let pedersen_points_table_window_bits_18_output_limb_16_col89 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(16);
                *row[89] = pedersen_points_table_window_bits_18_output_limb_16_col89;
                let pedersen_points_table_window_bits_18_output_limb_17_col90 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(17);
                *row[90] = pedersen_points_table_window_bits_18_output_limb_17_col90;
                let pedersen_points_table_window_bits_18_output_limb_18_col91 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(18);
                *row[91] = pedersen_points_table_window_bits_18_output_limb_18_col91;
                let pedersen_points_table_window_bits_18_output_limb_19_col92 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(19);
                *row[92] = pedersen_points_table_window_bits_18_output_limb_19_col92;
                let pedersen_points_table_window_bits_18_output_limb_20_col93 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(20);
                *row[93] = pedersen_points_table_window_bits_18_output_limb_20_col93;
                let pedersen_points_table_window_bits_18_output_limb_21_col94 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(21);
                *row[94] = pedersen_points_table_window_bits_18_output_limb_21_col94;
                let pedersen_points_table_window_bits_18_output_limb_22_col95 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(22);
                *row[95] = pedersen_points_table_window_bits_18_output_limb_22_col95;
                let pedersen_points_table_window_bits_18_output_limb_23_col96 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(23);
                *row[96] = pedersen_points_table_window_bits_18_output_limb_23_col96;
                let pedersen_points_table_window_bits_18_output_limb_24_col97 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(24);
                *row[97] = pedersen_points_table_window_bits_18_output_limb_24_col97;
                let pedersen_points_table_window_bits_18_output_limb_25_col98 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(25);
                *row[98] = pedersen_points_table_window_bits_18_output_limb_25_col98;
                let pedersen_points_table_window_bits_18_output_limb_26_col99 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(26);
                *row[99] = pedersen_points_table_window_bits_18_output_limb_26_col99;
                let pedersen_points_table_window_bits_18_output_limb_27_col100 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[0].get_m31(27);
                *row[100] = pedersen_points_table_window_bits_18_output_limb_27_col100;
                let pedersen_points_table_window_bits_18_output_limb_28_col101 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(0);
                *row[101] = pedersen_points_table_window_bits_18_output_limb_28_col101;
                let pedersen_points_table_window_bits_18_output_limb_29_col102 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(1);
                *row[102] = pedersen_points_table_window_bits_18_output_limb_29_col102;
                let pedersen_points_table_window_bits_18_output_limb_30_col103 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(2);
                *row[103] = pedersen_points_table_window_bits_18_output_limb_30_col103;
                let pedersen_points_table_window_bits_18_output_limb_31_col104 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(3);
                *row[104] = pedersen_points_table_window_bits_18_output_limb_31_col104;
                let pedersen_points_table_window_bits_18_output_limb_32_col105 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(4);
                *row[105] = pedersen_points_table_window_bits_18_output_limb_32_col105;
                let pedersen_points_table_window_bits_18_output_limb_33_col106 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(5);
                *row[106] = pedersen_points_table_window_bits_18_output_limb_33_col106;
                let pedersen_points_table_window_bits_18_output_limb_34_col107 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(6);
                *row[107] = pedersen_points_table_window_bits_18_output_limb_34_col107;
                let pedersen_points_table_window_bits_18_output_limb_35_col108 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(7);
                *row[108] = pedersen_points_table_window_bits_18_output_limb_35_col108;
                let pedersen_points_table_window_bits_18_output_limb_36_col109 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(8);
                *row[109] = pedersen_points_table_window_bits_18_output_limb_36_col109;
                let pedersen_points_table_window_bits_18_output_limb_37_col110 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(9);
                *row[110] = pedersen_points_table_window_bits_18_output_limb_37_col110;
                let pedersen_points_table_window_bits_18_output_limb_38_col111 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(10);
                *row[111] = pedersen_points_table_window_bits_18_output_limb_38_col111;
                let pedersen_points_table_window_bits_18_output_limb_39_col112 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(11);
                *row[112] = pedersen_points_table_window_bits_18_output_limb_39_col112;
                let pedersen_points_table_window_bits_18_output_limb_40_col113 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(12);
                *row[113] = pedersen_points_table_window_bits_18_output_limb_40_col113;
                let pedersen_points_table_window_bits_18_output_limb_41_col114 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(13);
                *row[114] = pedersen_points_table_window_bits_18_output_limb_41_col114;
                let pedersen_points_table_window_bits_18_output_limb_42_col115 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(14);
                *row[115] = pedersen_points_table_window_bits_18_output_limb_42_col115;
                let pedersen_points_table_window_bits_18_output_limb_43_col116 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(15);
                *row[116] = pedersen_points_table_window_bits_18_output_limb_43_col116;
                let pedersen_points_table_window_bits_18_output_limb_44_col117 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(16);
                *row[117] = pedersen_points_table_window_bits_18_output_limb_44_col117;
                let pedersen_points_table_window_bits_18_output_limb_45_col118 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(17);
                *row[118] = pedersen_points_table_window_bits_18_output_limb_45_col118;
                let pedersen_points_table_window_bits_18_output_limb_46_col119 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(18);
                *row[119] = pedersen_points_table_window_bits_18_output_limb_46_col119;
                let pedersen_points_table_window_bits_18_output_limb_47_col120 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(19);
                *row[120] = pedersen_points_table_window_bits_18_output_limb_47_col120;
                let pedersen_points_table_window_bits_18_output_limb_48_col121 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(20);
                *row[121] = pedersen_points_table_window_bits_18_output_limb_48_col121;
                let pedersen_points_table_window_bits_18_output_limb_49_col122 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(21);
                *row[122] = pedersen_points_table_window_bits_18_output_limb_49_col122;
                let pedersen_points_table_window_bits_18_output_limb_50_col123 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(22);
                *row[123] = pedersen_points_table_window_bits_18_output_limb_50_col123;
                let pedersen_points_table_window_bits_18_output_limb_51_col124 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(23);
                *row[124] = pedersen_points_table_window_bits_18_output_limb_51_col124;
                let pedersen_points_table_window_bits_18_output_limb_52_col125 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(24);
                *row[125] = pedersen_points_table_window_bits_18_output_limb_52_col125;
                let pedersen_points_table_window_bits_18_output_limb_53_col126 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(25);
                *row[126] = pedersen_points_table_window_bits_18_output_limb_53_col126;
                let pedersen_points_table_window_bits_18_output_limb_54_col127 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(26);
                *row[127] = pedersen_points_table_window_bits_18_output_limb_54_col127;
                let pedersen_points_table_window_bits_18_output_limb_55_col128 =
                    pedersen_points_table_window_bits_18_output_tmp_d9509_0[1].get_m31(27);
                *row[128] = pedersen_points_table_window_bits_18_output_limb_55_col128;
                *lookup_data.pedersen_points_table_window_bits_18_0 = [
                    M31_1444721856,
                    (((M31_262144) * (input_limb_1_col2)) + (input_limb_2_col3)),
                    pedersen_points_table_window_bits_18_output_limb_0_col73,
                    pedersen_points_table_window_bits_18_output_limb_1_col74,
                    pedersen_points_table_window_bits_18_output_limb_2_col75,
                    pedersen_points_table_window_bits_18_output_limb_3_col76,
                    pedersen_points_table_window_bits_18_output_limb_4_col77,
                    pedersen_points_table_window_bits_18_output_limb_5_col78,
                    pedersen_points_table_window_bits_18_output_limb_6_col79,
                    pedersen_points_table_window_bits_18_output_limb_7_col80,
                    pedersen_points_table_window_bits_18_output_limb_8_col81,
                    pedersen_points_table_window_bits_18_output_limb_9_col82,
                    pedersen_points_table_window_bits_18_output_limb_10_col83,
                    pedersen_points_table_window_bits_18_output_limb_11_col84,
                    pedersen_points_table_window_bits_18_output_limb_12_col85,
                    pedersen_points_table_window_bits_18_output_limb_13_col86,
                    pedersen_points_table_window_bits_18_output_limb_14_col87,
                    pedersen_points_table_window_bits_18_output_limb_15_col88,
                    pedersen_points_table_window_bits_18_output_limb_16_col89,
                    pedersen_points_table_window_bits_18_output_limb_17_col90,
                    pedersen_points_table_window_bits_18_output_limb_18_col91,
                    pedersen_points_table_window_bits_18_output_limb_19_col92,
                    pedersen_points_table_window_bits_18_output_limb_20_col93,
                    pedersen_points_table_window_bits_18_output_limb_21_col94,
                    pedersen_points_table_window_bits_18_output_limb_22_col95,
                    pedersen_points_table_window_bits_18_output_limb_23_col96,
                    pedersen_points_table_window_bits_18_output_limb_24_col97,
                    pedersen_points_table_window_bits_18_output_limb_25_col98,
                    pedersen_points_table_window_bits_18_output_limb_26_col99,
                    pedersen_points_table_window_bits_18_output_limb_27_col100,
                    pedersen_points_table_window_bits_18_output_limb_28_col101,
                    pedersen_points_table_window_bits_18_output_limb_29_col102,
                    pedersen_points_table_window_bits_18_output_limb_30_col103,
                    pedersen_points_table_window_bits_18_output_limb_31_col104,
                    pedersen_points_table_window_bits_18_output_limb_32_col105,
                    pedersen_points_table_window_bits_18_output_limb_33_col106,
                    pedersen_points_table_window_bits_18_output_limb_34_col107,
                    pedersen_points_table_window_bits_18_output_limb_35_col108,
                    pedersen_points_table_window_bits_18_output_limb_36_col109,
                    pedersen_points_table_window_bits_18_output_limb_37_col110,
                    pedersen_points_table_window_bits_18_output_limb_38_col111,
                    pedersen_points_table_window_bits_18_output_limb_39_col112,
                    pedersen_points_table_window_bits_18_output_limb_40_col113,
                    pedersen_points_table_window_bits_18_output_limb_41_col114,
                    pedersen_points_table_window_bits_18_output_limb_42_col115,
                    pedersen_points_table_window_bits_18_output_limb_43_col116,
                    pedersen_points_table_window_bits_18_output_limb_44_col117,
                    pedersen_points_table_window_bits_18_output_limb_45_col118,
                    pedersen_points_table_window_bits_18_output_limb_46_col119,
                    pedersen_points_table_window_bits_18_output_limb_47_col120,
                    pedersen_points_table_window_bits_18_output_limb_48_col121,
                    pedersen_points_table_window_bits_18_output_limb_49_col122,
                    pedersen_points_table_window_bits_18_output_limb_50_col123,
                    pedersen_points_table_window_bits_18_output_limb_51_col124,
                    pedersen_points_table_window_bits_18_output_limb_52_col125,
                    pedersen_points_table_window_bits_18_output_limb_53_col126,
                    pedersen_points_table_window_bits_18_output_limb_54_col127,
                    pedersen_points_table_window_bits_18_output_limb_55_col128,
                ];

                // Ec Add.

                let slope_tmp_d9509_1 = (((pedersen_points_table_window_bits_18_output_tmp_d9509_0
                    [1])
                    - (partial_ec_mul_window_bits_18_input.2 .1[1]))
                    / ((pedersen_points_table_window_bits_18_output_tmp_d9509_0[0])
                        - (partial_ec_mul_window_bits_18_input.2 .1[0])));
                let slope_limb_0_col129 = slope_tmp_d9509_1.get_m31(0);
                *row[129] = slope_limb_0_col129;
                let slope_limb_1_col130 = slope_tmp_d9509_1.get_m31(1);
                *row[130] = slope_limb_1_col130;
                let slope_limb_2_col131 = slope_tmp_d9509_1.get_m31(2);
                *row[131] = slope_limb_2_col131;
                let slope_limb_3_col132 = slope_tmp_d9509_1.get_m31(3);
                *row[132] = slope_limb_3_col132;
                let slope_limb_4_col133 = slope_tmp_d9509_1.get_m31(4);
                *row[133] = slope_limb_4_col133;
                let slope_limb_5_col134 = slope_tmp_d9509_1.get_m31(5);
                *row[134] = slope_limb_5_col134;
                let slope_limb_6_col135 = slope_tmp_d9509_1.get_m31(6);
                *row[135] = slope_limb_6_col135;
                let slope_limb_7_col136 = slope_tmp_d9509_1.get_m31(7);
                *row[136] = slope_limb_7_col136;
                let slope_limb_8_col137 = slope_tmp_d9509_1.get_m31(8);
                *row[137] = slope_limb_8_col137;
                let slope_limb_9_col138 = slope_tmp_d9509_1.get_m31(9);
                *row[138] = slope_limb_9_col138;
                let slope_limb_10_col139 = slope_tmp_d9509_1.get_m31(10);
                *row[139] = slope_limb_10_col139;
                let slope_limb_11_col140 = slope_tmp_d9509_1.get_m31(11);
                *row[140] = slope_limb_11_col140;
                let slope_limb_12_col141 = slope_tmp_d9509_1.get_m31(12);
                *row[141] = slope_limb_12_col141;
                let slope_limb_13_col142 = slope_tmp_d9509_1.get_m31(13);
                *row[142] = slope_limb_13_col142;
                let slope_limb_14_col143 = slope_tmp_d9509_1.get_m31(14);
                *row[143] = slope_limb_14_col143;
                let slope_limb_15_col144 = slope_tmp_d9509_1.get_m31(15);
                *row[144] = slope_limb_15_col144;
                let slope_limb_16_col145 = slope_tmp_d9509_1.get_m31(16);
                *row[145] = slope_limb_16_col145;
                let slope_limb_17_col146 = slope_tmp_d9509_1.get_m31(17);
                *row[146] = slope_limb_17_col146;
                let slope_limb_18_col147 = slope_tmp_d9509_1.get_m31(18);
                *row[147] = slope_limb_18_col147;
                let slope_limb_19_col148 = slope_tmp_d9509_1.get_m31(19);
                *row[148] = slope_limb_19_col148;
                let slope_limb_20_col149 = slope_tmp_d9509_1.get_m31(20);
                *row[149] = slope_limb_20_col149;
                let slope_limb_21_col150 = slope_tmp_d9509_1.get_m31(21);
                *row[150] = slope_limb_21_col150;
                let slope_limb_22_col151 = slope_tmp_d9509_1.get_m31(22);
                *row[151] = slope_limb_22_col151;
                let slope_limb_23_col152 = slope_tmp_d9509_1.get_m31(23);
                *row[152] = slope_limb_23_col152;
                let slope_limb_24_col153 = slope_tmp_d9509_1.get_m31(24);
                *row[153] = slope_limb_24_col153;
                let slope_limb_25_col154 = slope_tmp_d9509_1.get_m31(25);
                *row[154] = slope_limb_25_col154;
                let slope_limb_26_col155 = slope_tmp_d9509_1.get_m31(26);
                *row[155] = slope_limb_26_col155;
                let slope_limb_27_col156 = slope_tmp_d9509_1.get_m31(27);
                *row[156] = slope_limb_27_col156;

                // Range Check Mem Value N 28.

                *sub_component_inputs.range_check_9_9[0] =
                    [slope_limb_0_col129, slope_limb_1_col130];
                *lookup_data.range_check_9_9_1 =
                    [M31_517791011, slope_limb_0_col129, slope_limb_1_col130];
                *sub_component_inputs.range_check_9_9_b[0] =
                    [slope_limb_2_col131, slope_limb_3_col132];
                *lookup_data.range_check_9_9_b_2 =
                    [M31_1897792095, slope_limb_2_col131, slope_limb_3_col132];
                *sub_component_inputs.range_check_9_9_c[0] =
                    [slope_limb_4_col133, slope_limb_5_col134];
                *lookup_data.range_check_9_9_c_3 =
                    [M31_1881014476, slope_limb_4_col133, slope_limb_5_col134];
                *sub_component_inputs.range_check_9_9_d[0] =
                    [slope_limb_6_col135, slope_limb_7_col136];
                *lookup_data.range_check_9_9_d_4 =
                    [M31_1864236857, slope_limb_6_col135, slope_limb_7_col136];
                *sub_component_inputs.range_check_9_9_e[0] =
                    [slope_limb_8_col137, slope_limb_9_col138];
                *lookup_data.range_check_9_9_e_5 =
                    [M31_1847459238, slope_limb_8_col137, slope_limb_9_col138];
                *sub_component_inputs.range_check_9_9_f[0] =
                    [slope_limb_10_col139, slope_limb_11_col140];
                *lookup_data.range_check_9_9_f_6 =
                    [M31_1830681619, slope_limb_10_col139, slope_limb_11_col140];
                *sub_component_inputs.range_check_9_9_g[0] =
                    [slope_limb_12_col141, slope_limb_13_col142];
                *lookup_data.range_check_9_9_g_7 =
                    [M31_1813904000, slope_limb_12_col141, slope_limb_13_col142];
                *sub_component_inputs.range_check_9_9_h[0] =
                    [slope_limb_14_col143, slope_limb_15_col144];
                *lookup_data.range_check_9_9_h_8 =
                    [M31_2065568285, slope_limb_14_col143, slope_limb_15_col144];
                *sub_component_inputs.range_check_9_9[1] =
                    [slope_limb_16_col145, slope_limb_17_col146];
                *lookup_data.range_check_9_9_9 =
                    [M31_517791011, slope_limb_16_col145, slope_limb_17_col146];
                *sub_component_inputs.range_check_9_9_b[1] =
                    [slope_limb_18_col147, slope_limb_19_col148];
                *lookup_data.range_check_9_9_b_10 =
                    [M31_1897792095, slope_limb_18_col147, slope_limb_19_col148];
                *sub_component_inputs.range_check_9_9_c[1] =
                    [slope_limb_20_col149, slope_limb_21_col150];
                *lookup_data.range_check_9_9_c_11 =
                    [M31_1881014476, slope_limb_20_col149, slope_limb_21_col150];
                *sub_component_inputs.range_check_9_9_d[1] =
                    [slope_limb_22_col151, slope_limb_23_col152];
                *lookup_data.range_check_9_9_d_12 =
                    [M31_1864236857, slope_limb_22_col151, slope_limb_23_col152];
                *sub_component_inputs.range_check_9_9_e[1] =
                    [slope_limb_24_col153, slope_limb_25_col154];
                *lookup_data.range_check_9_9_e_13 =
                    [M31_1847459238, slope_limb_24_col153, slope_limb_25_col154];
                *sub_component_inputs.range_check_9_9_f[1] =
                    [slope_limb_26_col155, slope_limb_27_col156];
                *lookup_data.range_check_9_9_f_14 =
                    [M31_1830681619, slope_limb_26_col155, slope_limb_27_col156];

                let x_diff_0_tmp_d9509_2 =
                    ((pedersen_points_table_window_bits_18_output_limb_0_col73)
                        - (input_limb_16_col17));
                let x_diff_1_tmp_d9509_3 =
                    ((pedersen_points_table_window_bits_18_output_limb_1_col74)
                        - (input_limb_17_col18));
                let x_diff_2_tmp_d9509_4 =
                    ((pedersen_points_table_window_bits_18_output_limb_2_col75)
                        - (input_limb_18_col19));
                let x_diff_3_tmp_d9509_5 =
                    ((pedersen_points_table_window_bits_18_output_limb_3_col76)
                        - (input_limb_19_col20));
                let x_diff_4_tmp_d9509_6 =
                    ((pedersen_points_table_window_bits_18_output_limb_4_col77)
                        - (input_limb_20_col21));
                let x_diff_5_tmp_d9509_7 =
                    ((pedersen_points_table_window_bits_18_output_limb_5_col78)
                        - (input_limb_21_col22));
                let x_diff_6_tmp_d9509_8 =
                    ((pedersen_points_table_window_bits_18_output_limb_6_col79)
                        - (input_limb_22_col23));
                let x_diff_7_tmp_d9509_9 =
                    ((pedersen_points_table_window_bits_18_output_limb_7_col80)
                        - (input_limb_23_col24));
                let x_diff_8_tmp_d9509_10 =
                    ((pedersen_points_table_window_bits_18_output_limb_8_col81)
                        - (input_limb_24_col25));
                let x_diff_9_tmp_d9509_11 =
                    ((pedersen_points_table_window_bits_18_output_limb_9_col82)
                        - (input_limb_25_col26));
                let x_diff_10_tmp_d9509_12 =
                    ((pedersen_points_table_window_bits_18_output_limb_10_col83)
                        - (input_limb_26_col27));
                let x_diff_11_tmp_d9509_13 =
                    ((pedersen_points_table_window_bits_18_output_limb_11_col84)
                        - (input_limb_27_col28));
                let x_diff_12_tmp_d9509_14 =
                    ((pedersen_points_table_window_bits_18_output_limb_12_col85)
                        - (input_limb_28_col29));
                let x_diff_13_tmp_d9509_15 =
                    ((pedersen_points_table_window_bits_18_output_limb_13_col86)
                        - (input_limb_29_col30));
                let x_diff_14_tmp_d9509_16 =
                    ((pedersen_points_table_window_bits_18_output_limb_14_col87)
                        - (input_limb_30_col31));
                let x_diff_15_tmp_d9509_17 =
                    ((pedersen_points_table_window_bits_18_output_limb_15_col88)
                        - (input_limb_31_col32));
                let x_diff_16_tmp_d9509_18 =
                    ((pedersen_points_table_window_bits_18_output_limb_16_col89)
                        - (input_limb_32_col33));
                let x_diff_17_tmp_d9509_19 =
                    ((pedersen_points_table_window_bits_18_output_limb_17_col90)
                        - (input_limb_33_col34));
                let x_diff_18_tmp_d9509_20 =
                    ((pedersen_points_table_window_bits_18_output_limb_18_col91)
                        - (input_limb_34_col35));
                let x_diff_19_tmp_d9509_21 =
                    ((pedersen_points_table_window_bits_18_output_limb_19_col92)
                        - (input_limb_35_col36));
                let x_diff_20_tmp_d9509_22 =
                    ((pedersen_points_table_window_bits_18_output_limb_20_col93)
                        - (input_limb_36_col37));
                let x_diff_21_tmp_d9509_23 =
                    ((pedersen_points_table_window_bits_18_output_limb_21_col94)
                        - (input_limb_37_col38));
                let x_diff_22_tmp_d9509_24 =
                    ((pedersen_points_table_window_bits_18_output_limb_22_col95)
                        - (input_limb_38_col39));
                let x_diff_23_tmp_d9509_25 =
                    ((pedersen_points_table_window_bits_18_output_limb_23_col96)
                        - (input_limb_39_col40));
                let x_diff_24_tmp_d9509_26 =
                    ((pedersen_points_table_window_bits_18_output_limb_24_col97)
                        - (input_limb_40_col41));
                let x_diff_25_tmp_d9509_27 =
                    ((pedersen_points_table_window_bits_18_output_limb_25_col98)
                        - (input_limb_41_col42));
                let x_diff_26_tmp_d9509_28 =
                    ((pedersen_points_table_window_bits_18_output_limb_26_col99)
                        - (input_limb_42_col43));
                let x_diff_27_tmp_d9509_29 =
                    ((pedersen_points_table_window_bits_18_output_limb_27_col100)
                        - (input_limb_43_col44));
                let y_diff_0_tmp_d9509_30 =
                    ((pedersen_points_table_window_bits_18_output_limb_28_col101)
                        - (input_limb_44_col45));
                let y_diff_1_tmp_d9509_31 =
                    ((pedersen_points_table_window_bits_18_output_limb_29_col102)
                        - (input_limb_45_col46));
                let y_diff_2_tmp_d9509_32 =
                    ((pedersen_points_table_window_bits_18_output_limb_30_col103)
                        - (input_limb_46_col47));
                let y_diff_3_tmp_d9509_33 =
                    ((pedersen_points_table_window_bits_18_output_limb_31_col104)
                        - (input_limb_47_col48));
                let y_diff_4_tmp_d9509_34 =
                    ((pedersen_points_table_window_bits_18_output_limb_32_col105)
                        - (input_limb_48_col49));
                let y_diff_5_tmp_d9509_35 =
                    ((pedersen_points_table_window_bits_18_output_limb_33_col106)
                        - (input_limb_49_col50));
                let y_diff_6_tmp_d9509_36 =
                    ((pedersen_points_table_window_bits_18_output_limb_34_col107)
                        - (input_limb_50_col51));
                let y_diff_7_tmp_d9509_37 =
                    ((pedersen_points_table_window_bits_18_output_limb_35_col108)
                        - (input_limb_51_col52));
                let y_diff_8_tmp_d9509_38 =
                    ((pedersen_points_table_window_bits_18_output_limb_36_col109)
                        - (input_limb_52_col53));
                let y_diff_9_tmp_d9509_39 =
                    ((pedersen_points_table_window_bits_18_output_limb_37_col110)
                        - (input_limb_53_col54));
                let y_diff_10_tmp_d9509_40 =
                    ((pedersen_points_table_window_bits_18_output_limb_38_col111)
                        - (input_limb_54_col55));
                let y_diff_11_tmp_d9509_41 =
                    ((pedersen_points_table_window_bits_18_output_limb_39_col112)
                        - (input_limb_55_col56));
                let y_diff_12_tmp_d9509_42 =
                    ((pedersen_points_table_window_bits_18_output_limb_40_col113)
                        - (input_limb_56_col57));
                let y_diff_13_tmp_d9509_43 =
                    ((pedersen_points_table_window_bits_18_output_limb_41_col114)
                        - (input_limb_57_col58));
                let y_diff_14_tmp_d9509_44 =
                    ((pedersen_points_table_window_bits_18_output_limb_42_col115)
                        - (input_limb_58_col59));
                let y_diff_15_tmp_d9509_45 =
                    ((pedersen_points_table_window_bits_18_output_limb_43_col116)
                        - (input_limb_59_col60));
                let y_diff_16_tmp_d9509_46 =
                    ((pedersen_points_table_window_bits_18_output_limb_44_col117)
                        - (input_limb_60_col61));
                let y_diff_17_tmp_d9509_47 =
                    ((pedersen_points_table_window_bits_18_output_limb_45_col118)
                        - (input_limb_61_col62));
                let y_diff_18_tmp_d9509_48 =
                    ((pedersen_points_table_window_bits_18_output_limb_46_col119)
                        - (input_limb_62_col63));
                let y_diff_19_tmp_d9509_49 =
                    ((pedersen_points_table_window_bits_18_output_limb_47_col120)
                        - (input_limb_63_col64));
                let y_diff_20_tmp_d9509_50 =
                    ((pedersen_points_table_window_bits_18_output_limb_48_col121)
                        - (input_limb_64_col65));
                let y_diff_21_tmp_d9509_51 =
                    ((pedersen_points_table_window_bits_18_output_limb_49_col122)
                        - (input_limb_65_col66));
                let y_diff_22_tmp_d9509_52 =
                    ((pedersen_points_table_window_bits_18_output_limb_50_col123)
                        - (input_limb_66_col67));
                let y_diff_23_tmp_d9509_53 =
                    ((pedersen_points_table_window_bits_18_output_limb_51_col124)
                        - (input_limb_67_col68));
                let y_diff_24_tmp_d9509_54 =
                    ((pedersen_points_table_window_bits_18_output_limb_52_col125)
                        - (input_limb_68_col69));
                let y_diff_25_tmp_d9509_55 =
                    ((pedersen_points_table_window_bits_18_output_limb_53_col126)
                        - (input_limb_69_col70));
                let y_diff_26_tmp_d9509_56 =
                    ((pedersen_points_table_window_bits_18_output_limb_54_col127)
                        - (input_limb_70_col71));
                let y_diff_27_tmp_d9509_57 =
                    ((pedersen_points_table_window_bits_18_output_limb_55_col128)
                        - (input_limb_71_col72));

                // Verify Mul 252.

                // Double Karatsuba F 0 Fc 6.

                // Single Karatsuba N 7.

                let z0_tmp_d9509_58 = [
                    ((slope_limb_0_col129) * (x_diff_0_tmp_d9509_2)),
                    (((slope_limb_0_col129) * (x_diff_1_tmp_d9509_3))
                        + ((slope_limb_1_col130) * (x_diff_0_tmp_d9509_2))),
                    ((((slope_limb_0_col129) * (x_diff_2_tmp_d9509_4))
                        + ((slope_limb_1_col130) * (x_diff_1_tmp_d9509_3)))
                        + ((slope_limb_2_col131) * (x_diff_0_tmp_d9509_2))),
                    (((((slope_limb_0_col129) * (x_diff_3_tmp_d9509_5))
                        + ((slope_limb_1_col130) * (x_diff_2_tmp_d9509_4)))
                        + ((slope_limb_2_col131) * (x_diff_1_tmp_d9509_3)))
                        + ((slope_limb_3_col132) * (x_diff_0_tmp_d9509_2))),
                    ((((((slope_limb_0_col129) * (x_diff_4_tmp_d9509_6))
                        + ((slope_limb_1_col130) * (x_diff_3_tmp_d9509_5)))
                        + ((slope_limb_2_col131) * (x_diff_2_tmp_d9509_4)))
                        + ((slope_limb_3_col132) * (x_diff_1_tmp_d9509_3)))
                        + ((slope_limb_4_col133) * (x_diff_0_tmp_d9509_2))),
                    (((((((slope_limb_0_col129) * (x_diff_5_tmp_d9509_7))
                        + ((slope_limb_1_col130) * (x_diff_4_tmp_d9509_6)))
                        + ((slope_limb_2_col131) * (x_diff_3_tmp_d9509_5)))
                        + ((slope_limb_3_col132) * (x_diff_2_tmp_d9509_4)))
                        + ((slope_limb_4_col133) * (x_diff_1_tmp_d9509_3)))
                        + ((slope_limb_5_col134) * (x_diff_0_tmp_d9509_2))),
                    ((((((((slope_limb_0_col129) * (x_diff_6_tmp_d9509_8))
                        + ((slope_limb_1_col130) * (x_diff_5_tmp_d9509_7)))
                        + ((slope_limb_2_col131) * (x_diff_4_tmp_d9509_6)))
                        + ((slope_limb_3_col132) * (x_diff_3_tmp_d9509_5)))
                        + ((slope_limb_4_col133) * (x_diff_2_tmp_d9509_4)))
                        + ((slope_limb_5_col134) * (x_diff_1_tmp_d9509_3)))
                        + ((slope_limb_6_col135) * (x_diff_0_tmp_d9509_2))),
                    (((((((slope_limb_1_col130) * (x_diff_6_tmp_d9509_8))
                        + ((slope_limb_2_col131) * (x_diff_5_tmp_d9509_7)))
                        + ((slope_limb_3_col132) * (x_diff_4_tmp_d9509_6)))
                        + ((slope_limb_4_col133) * (x_diff_3_tmp_d9509_5)))
                        + ((slope_limb_5_col134) * (x_diff_2_tmp_d9509_4)))
                        + ((slope_limb_6_col135) * (x_diff_1_tmp_d9509_3))),
                    ((((((slope_limb_2_col131) * (x_diff_6_tmp_d9509_8))
                        + ((slope_limb_3_col132) * (x_diff_5_tmp_d9509_7)))
                        + ((slope_limb_4_col133) * (x_diff_4_tmp_d9509_6)))
                        + ((slope_limb_5_col134) * (x_diff_3_tmp_d9509_5)))
                        + ((slope_limb_6_col135) * (x_diff_2_tmp_d9509_4))),
                    (((((slope_limb_3_col132) * (x_diff_6_tmp_d9509_8))
                        + ((slope_limb_4_col133) * (x_diff_5_tmp_d9509_7)))
                        + ((slope_limb_5_col134) * (x_diff_4_tmp_d9509_6)))
                        + ((slope_limb_6_col135) * (x_diff_3_tmp_d9509_5))),
                    ((((slope_limb_4_col133) * (x_diff_6_tmp_d9509_8))
                        + ((slope_limb_5_col134) * (x_diff_5_tmp_d9509_7)))
                        + ((slope_limb_6_col135) * (x_diff_4_tmp_d9509_6))),
                    (((slope_limb_5_col134) * (x_diff_6_tmp_d9509_8))
                        + ((slope_limb_6_col135) * (x_diff_5_tmp_d9509_7))),
                    ((slope_limb_6_col135) * (x_diff_6_tmp_d9509_8)),
                ];
                let z2_tmp_d9509_59 = [
                    ((slope_limb_7_col136) * (x_diff_7_tmp_d9509_9)),
                    (((slope_limb_7_col136) * (x_diff_8_tmp_d9509_10))
                        + ((slope_limb_8_col137) * (x_diff_7_tmp_d9509_9))),
                    ((((slope_limb_7_col136) * (x_diff_9_tmp_d9509_11))
                        + ((slope_limb_8_col137) * (x_diff_8_tmp_d9509_10)))
                        + ((slope_limb_9_col138) * (x_diff_7_tmp_d9509_9))),
                    (((((slope_limb_7_col136) * (x_diff_10_tmp_d9509_12))
                        + ((slope_limb_8_col137) * (x_diff_9_tmp_d9509_11)))
                        + ((slope_limb_9_col138) * (x_diff_8_tmp_d9509_10)))
                        + ((slope_limb_10_col139) * (x_diff_7_tmp_d9509_9))),
                    ((((((slope_limb_7_col136) * (x_diff_11_tmp_d9509_13))
                        + ((slope_limb_8_col137) * (x_diff_10_tmp_d9509_12)))
                        + ((slope_limb_9_col138) * (x_diff_9_tmp_d9509_11)))
                        + ((slope_limb_10_col139) * (x_diff_8_tmp_d9509_10)))
                        + ((slope_limb_11_col140) * (x_diff_7_tmp_d9509_9))),
                    (((((((slope_limb_7_col136) * (x_diff_12_tmp_d9509_14))
                        + ((slope_limb_8_col137) * (x_diff_11_tmp_d9509_13)))
                        + ((slope_limb_9_col138) * (x_diff_10_tmp_d9509_12)))
                        + ((slope_limb_10_col139) * (x_diff_9_tmp_d9509_11)))
                        + ((slope_limb_11_col140) * (x_diff_8_tmp_d9509_10)))
                        + ((slope_limb_12_col141) * (x_diff_7_tmp_d9509_9))),
                    ((((((((slope_limb_7_col136) * (x_diff_13_tmp_d9509_15))
                        + ((slope_limb_8_col137) * (x_diff_12_tmp_d9509_14)))
                        + ((slope_limb_9_col138) * (x_diff_11_tmp_d9509_13)))
                        + ((slope_limb_10_col139) * (x_diff_10_tmp_d9509_12)))
                        + ((slope_limb_11_col140) * (x_diff_9_tmp_d9509_11)))
                        + ((slope_limb_12_col141) * (x_diff_8_tmp_d9509_10)))
                        + ((slope_limb_13_col142) * (x_diff_7_tmp_d9509_9))),
                    (((((((slope_limb_8_col137) * (x_diff_13_tmp_d9509_15))
                        + ((slope_limb_9_col138) * (x_diff_12_tmp_d9509_14)))
                        + ((slope_limb_10_col139) * (x_diff_11_tmp_d9509_13)))
                        + ((slope_limb_11_col140) * (x_diff_10_tmp_d9509_12)))
                        + ((slope_limb_12_col141) * (x_diff_9_tmp_d9509_11)))
                        + ((slope_limb_13_col142) * (x_diff_8_tmp_d9509_10))),
                    ((((((slope_limb_9_col138) * (x_diff_13_tmp_d9509_15))
                        + ((slope_limb_10_col139) * (x_diff_12_tmp_d9509_14)))
                        + ((slope_limb_11_col140) * (x_diff_11_tmp_d9509_13)))
                        + ((slope_limb_12_col141) * (x_diff_10_tmp_d9509_12)))
                        + ((slope_limb_13_col142) * (x_diff_9_tmp_d9509_11))),
                    (((((slope_limb_10_col139) * (x_diff_13_tmp_d9509_15))
                        + ((slope_limb_11_col140) * (x_diff_12_tmp_d9509_14)))
                        + ((slope_limb_12_col141) * (x_diff_11_tmp_d9509_13)))
                        + ((slope_limb_13_col142) * (x_diff_10_tmp_d9509_12))),
                    ((((slope_limb_11_col140) * (x_diff_13_tmp_d9509_15))
                        + ((slope_limb_12_col141) * (x_diff_12_tmp_d9509_14)))
                        + ((slope_limb_13_col142) * (x_diff_11_tmp_d9509_13))),
                    (((slope_limb_12_col141) * (x_diff_13_tmp_d9509_15))
                        + ((slope_limb_13_col142) * (x_diff_12_tmp_d9509_14))),
                    ((slope_limb_13_col142) * (x_diff_13_tmp_d9509_15)),
                ];
                let x_sum_tmp_d9509_60 = [
                    ((slope_limb_0_col129) + (slope_limb_7_col136)),
                    ((slope_limb_1_col130) + (slope_limb_8_col137)),
                    ((slope_limb_2_col131) + (slope_limb_9_col138)),
                    ((slope_limb_3_col132) + (slope_limb_10_col139)),
                    ((slope_limb_4_col133) + (slope_limb_11_col140)),
                    ((slope_limb_5_col134) + (slope_limb_12_col141)),
                    ((slope_limb_6_col135) + (slope_limb_13_col142)),
                ];
                let y_sum_tmp_d9509_61 = [
                    ((x_diff_0_tmp_d9509_2) + (x_diff_7_tmp_d9509_9)),
                    ((x_diff_1_tmp_d9509_3) + (x_diff_8_tmp_d9509_10)),
                    ((x_diff_2_tmp_d9509_4) + (x_diff_9_tmp_d9509_11)),
                    ((x_diff_3_tmp_d9509_5) + (x_diff_10_tmp_d9509_12)),
                    ((x_diff_4_tmp_d9509_6) + (x_diff_11_tmp_d9509_13)),
                    ((x_diff_5_tmp_d9509_7) + (x_diff_12_tmp_d9509_14)),
                    ((x_diff_6_tmp_d9509_8) + (x_diff_13_tmp_d9509_15)),
                ];
                let single_karatsuba_n_7_output_tmp_d9509_62 = [
                    z0_tmp_d9509_58[0],
                    z0_tmp_d9509_58[1],
                    z0_tmp_d9509_58[2],
                    z0_tmp_d9509_58[3],
                    z0_tmp_d9509_58[4],
                    z0_tmp_d9509_58[5],
                    z0_tmp_d9509_58[6],
                    ((z0_tmp_d9509_58[7])
                        + ((((x_sum_tmp_d9509_60[0]) * (y_sum_tmp_d9509_61[0]))
                            - (z0_tmp_d9509_58[0]))
                            - (z2_tmp_d9509_59[0]))),
                    ((z0_tmp_d9509_58[8])
                        + (((((x_sum_tmp_d9509_60[0]) * (y_sum_tmp_d9509_61[1]))
                            + ((x_sum_tmp_d9509_60[1]) * (y_sum_tmp_d9509_61[0])))
                            - (z0_tmp_d9509_58[1]))
                            - (z2_tmp_d9509_59[1]))),
                    ((z0_tmp_d9509_58[9])
                        + ((((((x_sum_tmp_d9509_60[0]) * (y_sum_tmp_d9509_61[2]))
                            + ((x_sum_tmp_d9509_60[1]) * (y_sum_tmp_d9509_61[1])))
                            + ((x_sum_tmp_d9509_60[2]) * (y_sum_tmp_d9509_61[0])))
                            - (z0_tmp_d9509_58[2]))
                            - (z2_tmp_d9509_59[2]))),
                    ((z0_tmp_d9509_58[10])
                        + (((((((x_sum_tmp_d9509_60[0]) * (y_sum_tmp_d9509_61[3]))
                            + ((x_sum_tmp_d9509_60[1]) * (y_sum_tmp_d9509_61[2])))
                            + ((x_sum_tmp_d9509_60[2]) * (y_sum_tmp_d9509_61[1])))
                            + ((x_sum_tmp_d9509_60[3]) * (y_sum_tmp_d9509_61[0])))
                            - (z0_tmp_d9509_58[3]))
                            - (z2_tmp_d9509_59[3]))),
                    ((z0_tmp_d9509_58[11])
                        + ((((((((x_sum_tmp_d9509_60[0]) * (y_sum_tmp_d9509_61[4]))
                            + ((x_sum_tmp_d9509_60[1]) * (y_sum_tmp_d9509_61[3])))
                            + ((x_sum_tmp_d9509_60[2]) * (y_sum_tmp_d9509_61[2])))
                            + ((x_sum_tmp_d9509_60[3]) * (y_sum_tmp_d9509_61[1])))
                            + ((x_sum_tmp_d9509_60[4]) * (y_sum_tmp_d9509_61[0])))
                            - (z0_tmp_d9509_58[4]))
                            - (z2_tmp_d9509_59[4]))),
                    ((z0_tmp_d9509_58[12])
                        + (((((((((x_sum_tmp_d9509_60[0]) * (y_sum_tmp_d9509_61[5]))
                            + ((x_sum_tmp_d9509_60[1]) * (y_sum_tmp_d9509_61[4])))
                            + ((x_sum_tmp_d9509_60[2]) * (y_sum_tmp_d9509_61[3])))
                            + ((x_sum_tmp_d9509_60[3]) * (y_sum_tmp_d9509_61[2])))
                            + ((x_sum_tmp_d9509_60[4]) * (y_sum_tmp_d9509_61[1])))
                            + ((x_sum_tmp_d9509_60[5]) * (y_sum_tmp_d9509_61[0])))
                            - (z0_tmp_d9509_58[5]))
                            - (z2_tmp_d9509_59[5]))),
                    ((((((((((x_sum_tmp_d9509_60[0]) * (y_sum_tmp_d9509_61[6]))
                        + ((x_sum_tmp_d9509_60[1]) * (y_sum_tmp_d9509_61[5])))
                        + ((x_sum_tmp_d9509_60[2]) * (y_sum_tmp_d9509_61[4])))
                        + ((x_sum_tmp_d9509_60[3]) * (y_sum_tmp_d9509_61[3])))
                        + ((x_sum_tmp_d9509_60[4]) * (y_sum_tmp_d9509_61[2])))
                        + ((x_sum_tmp_d9509_60[5]) * (y_sum_tmp_d9509_61[1])))
                        + ((x_sum_tmp_d9509_60[6]) * (y_sum_tmp_d9509_61[0])))
                        - (z0_tmp_d9509_58[6]))
                        - (z2_tmp_d9509_59[6])),
                    ((z2_tmp_d9509_59[0])
                        + (((((((((x_sum_tmp_d9509_60[1]) * (y_sum_tmp_d9509_61[6]))
                            + ((x_sum_tmp_d9509_60[2]) * (y_sum_tmp_d9509_61[5])))
                            + ((x_sum_tmp_d9509_60[3]) * (y_sum_tmp_d9509_61[4])))
                            + ((x_sum_tmp_d9509_60[4]) * (y_sum_tmp_d9509_61[3])))
                            + ((x_sum_tmp_d9509_60[5]) * (y_sum_tmp_d9509_61[2])))
                            + ((x_sum_tmp_d9509_60[6]) * (y_sum_tmp_d9509_61[1])))
                            - (z0_tmp_d9509_58[7]))
                            - (z2_tmp_d9509_59[7]))),
                    ((z2_tmp_d9509_59[1])
                        + ((((((((x_sum_tmp_d9509_60[2]) * (y_sum_tmp_d9509_61[6]))
                            + ((x_sum_tmp_d9509_60[3]) * (y_sum_tmp_d9509_61[5])))
                            + ((x_sum_tmp_d9509_60[4]) * (y_sum_tmp_d9509_61[4])))
                            + ((x_sum_tmp_d9509_60[5]) * (y_sum_tmp_d9509_61[3])))
                            + ((x_sum_tmp_d9509_60[6]) * (y_sum_tmp_d9509_61[2])))
                            - (z0_tmp_d9509_58[8]))
                            - (z2_tmp_d9509_59[8]))),
                    ((z2_tmp_d9509_59[2])
                        + (((((((x_sum_tmp_d9509_60[3]) * (y_sum_tmp_d9509_61[6]))
                            + ((x_sum_tmp_d9509_60[4]) * (y_sum_tmp_d9509_61[5])))
                            + ((x_sum_tmp_d9509_60[5]) * (y_sum_tmp_d9509_61[4])))
                            + ((x_sum_tmp_d9509_60[6]) * (y_sum_tmp_d9509_61[3])))
                            - (z0_tmp_d9509_58[9]))
                            - (z2_tmp_d9509_59[9]))),
                    ((z2_tmp_d9509_59[3])
                        + ((((((x_sum_tmp_d9509_60[4]) * (y_sum_tmp_d9509_61[6]))
                            + ((x_sum_tmp_d9509_60[5]) * (y_sum_tmp_d9509_61[5])))
                            + ((x_sum_tmp_d9509_60[6]) * (y_sum_tmp_d9509_61[4])))
                            - (z0_tmp_d9509_58[10]))
                            - (z2_tmp_d9509_59[10]))),
                    ((z2_tmp_d9509_59[4])
                        + (((((x_sum_tmp_d9509_60[5]) * (y_sum_tmp_d9509_61[6]))
                            + ((x_sum_tmp_d9509_60[6]) * (y_sum_tmp_d9509_61[5])))
                            - (z0_tmp_d9509_58[11]))
                            - (z2_tmp_d9509_59[11]))),
                    ((z2_tmp_d9509_59[5])
                        + ((((x_sum_tmp_d9509_60[6]) * (y_sum_tmp_d9509_61[6]))
                            - (z0_tmp_d9509_58[12]))
                            - (z2_tmp_d9509_59[12]))),
                    z2_tmp_d9509_59[6],
                    z2_tmp_d9509_59[7],
                    z2_tmp_d9509_59[8],
                    z2_tmp_d9509_59[9],
                    z2_tmp_d9509_59[10],
                    z2_tmp_d9509_59[11],
                    z2_tmp_d9509_59[12],
                ];

                // Single Karatsuba N 7.

                let z0_tmp_d9509_63 = [
                    ((slope_limb_14_col143) * (x_diff_14_tmp_d9509_16)),
                    (((slope_limb_14_col143) * (x_diff_15_tmp_d9509_17))
                        + ((slope_limb_15_col144) * (x_diff_14_tmp_d9509_16))),
                    ((((slope_limb_14_col143) * (x_diff_16_tmp_d9509_18))
                        + ((slope_limb_15_col144) * (x_diff_15_tmp_d9509_17)))
                        + ((slope_limb_16_col145) * (x_diff_14_tmp_d9509_16))),
                    (((((slope_limb_14_col143) * (x_diff_17_tmp_d9509_19))
                        + ((slope_limb_15_col144) * (x_diff_16_tmp_d9509_18)))
                        + ((slope_limb_16_col145) * (x_diff_15_tmp_d9509_17)))
                        + ((slope_limb_17_col146) * (x_diff_14_tmp_d9509_16))),
                    ((((((slope_limb_14_col143) * (x_diff_18_tmp_d9509_20))
                        + ((slope_limb_15_col144) * (x_diff_17_tmp_d9509_19)))
                        + ((slope_limb_16_col145) * (x_diff_16_tmp_d9509_18)))
                        + ((slope_limb_17_col146) * (x_diff_15_tmp_d9509_17)))
                        + ((slope_limb_18_col147) * (x_diff_14_tmp_d9509_16))),
                    (((((((slope_limb_14_col143) * (x_diff_19_tmp_d9509_21))
                        + ((slope_limb_15_col144) * (x_diff_18_tmp_d9509_20)))
                        + ((slope_limb_16_col145) * (x_diff_17_tmp_d9509_19)))
                        + ((slope_limb_17_col146) * (x_diff_16_tmp_d9509_18)))
                        + ((slope_limb_18_col147) * (x_diff_15_tmp_d9509_17)))
                        + ((slope_limb_19_col148) * (x_diff_14_tmp_d9509_16))),
                    ((((((((slope_limb_14_col143) * (x_diff_20_tmp_d9509_22))
                        + ((slope_limb_15_col144) * (x_diff_19_tmp_d9509_21)))
                        + ((slope_limb_16_col145) * (x_diff_18_tmp_d9509_20)))
                        + ((slope_limb_17_col146) * (x_diff_17_tmp_d9509_19)))
                        + ((slope_limb_18_col147) * (x_diff_16_tmp_d9509_18)))
                        + ((slope_limb_19_col148) * (x_diff_15_tmp_d9509_17)))
                        + ((slope_limb_20_col149) * (x_diff_14_tmp_d9509_16))),
                    (((((((slope_limb_15_col144) * (x_diff_20_tmp_d9509_22))
                        + ((slope_limb_16_col145) * (x_diff_19_tmp_d9509_21)))
                        + ((slope_limb_17_col146) * (x_diff_18_tmp_d9509_20)))
                        + ((slope_limb_18_col147) * (x_diff_17_tmp_d9509_19)))
                        + ((slope_limb_19_col148) * (x_diff_16_tmp_d9509_18)))
                        + ((slope_limb_20_col149) * (x_diff_15_tmp_d9509_17))),
                    ((((((slope_limb_16_col145) * (x_diff_20_tmp_d9509_22))
                        + ((slope_limb_17_col146) * (x_diff_19_tmp_d9509_21)))
                        + ((slope_limb_18_col147) * (x_diff_18_tmp_d9509_20)))
                        + ((slope_limb_19_col148) * (x_diff_17_tmp_d9509_19)))
                        + ((slope_limb_20_col149) * (x_diff_16_tmp_d9509_18))),
                    (((((slope_limb_17_col146) * (x_diff_20_tmp_d9509_22))
                        + ((slope_limb_18_col147) * (x_diff_19_tmp_d9509_21)))
                        + ((slope_limb_19_col148) * (x_diff_18_tmp_d9509_20)))
                        + ((slope_limb_20_col149) * (x_diff_17_tmp_d9509_19))),
                    ((((slope_limb_18_col147) * (x_diff_20_tmp_d9509_22))
                        + ((slope_limb_19_col148) * (x_diff_19_tmp_d9509_21)))
                        + ((slope_limb_20_col149) * (x_diff_18_tmp_d9509_20))),
                    (((slope_limb_19_col148) * (x_diff_20_tmp_d9509_22))
                        + ((slope_limb_20_col149) * (x_diff_19_tmp_d9509_21))),
                    ((slope_limb_20_col149) * (x_diff_20_tmp_d9509_22)),
                ];
                let z2_tmp_d9509_64 = [
                    ((slope_limb_21_col150) * (x_diff_21_tmp_d9509_23)),
                    (((slope_limb_21_col150) * (x_diff_22_tmp_d9509_24))
                        + ((slope_limb_22_col151) * (x_diff_21_tmp_d9509_23))),
                    ((((slope_limb_21_col150) * (x_diff_23_tmp_d9509_25))
                        + ((slope_limb_22_col151) * (x_diff_22_tmp_d9509_24)))
                        + ((slope_limb_23_col152) * (x_diff_21_tmp_d9509_23))),
                    (((((slope_limb_21_col150) * (x_diff_24_tmp_d9509_26))
                        + ((slope_limb_22_col151) * (x_diff_23_tmp_d9509_25)))
                        + ((slope_limb_23_col152) * (x_diff_22_tmp_d9509_24)))
                        + ((slope_limb_24_col153) * (x_diff_21_tmp_d9509_23))),
                    ((((((slope_limb_21_col150) * (x_diff_25_tmp_d9509_27))
                        + ((slope_limb_22_col151) * (x_diff_24_tmp_d9509_26)))
                        + ((slope_limb_23_col152) * (x_diff_23_tmp_d9509_25)))
                        + ((slope_limb_24_col153) * (x_diff_22_tmp_d9509_24)))
                        + ((slope_limb_25_col154) * (x_diff_21_tmp_d9509_23))),
                    (((((((slope_limb_21_col150) * (x_diff_26_tmp_d9509_28))
                        + ((slope_limb_22_col151) * (x_diff_25_tmp_d9509_27)))
                        + ((slope_limb_23_col152) * (x_diff_24_tmp_d9509_26)))
                        + ((slope_limb_24_col153) * (x_diff_23_tmp_d9509_25)))
                        + ((slope_limb_25_col154) * (x_diff_22_tmp_d9509_24)))
                        + ((slope_limb_26_col155) * (x_diff_21_tmp_d9509_23))),
                    ((((((((slope_limb_21_col150) * (x_diff_27_tmp_d9509_29))
                        + ((slope_limb_22_col151) * (x_diff_26_tmp_d9509_28)))
                        + ((slope_limb_23_col152) * (x_diff_25_tmp_d9509_27)))
                        + ((slope_limb_24_col153) * (x_diff_24_tmp_d9509_26)))
                        + ((slope_limb_25_col154) * (x_diff_23_tmp_d9509_25)))
                        + ((slope_limb_26_col155) * (x_diff_22_tmp_d9509_24)))
                        + ((slope_limb_27_col156) * (x_diff_21_tmp_d9509_23))),
                    (((((((slope_limb_22_col151) * (x_diff_27_tmp_d9509_29))
                        + ((slope_limb_23_col152) * (x_diff_26_tmp_d9509_28)))
                        + ((slope_limb_24_col153) * (x_diff_25_tmp_d9509_27)))
                        + ((slope_limb_25_col154) * (x_diff_24_tmp_d9509_26)))
                        + ((slope_limb_26_col155) * (x_diff_23_tmp_d9509_25)))
                        + ((slope_limb_27_col156) * (x_diff_22_tmp_d9509_24))),
                    ((((((slope_limb_23_col152) * (x_diff_27_tmp_d9509_29))
                        + ((slope_limb_24_col153) * (x_diff_26_tmp_d9509_28)))
                        + ((slope_limb_25_col154) * (x_diff_25_tmp_d9509_27)))
                        + ((slope_limb_26_col155) * (x_diff_24_tmp_d9509_26)))
                        + ((slope_limb_27_col156) * (x_diff_23_tmp_d9509_25))),
                    (((((slope_limb_24_col153) * (x_diff_27_tmp_d9509_29))
                        + ((slope_limb_25_col154) * (x_diff_26_tmp_d9509_28)))
                        + ((slope_limb_26_col155) * (x_diff_25_tmp_d9509_27)))
                        + ((slope_limb_27_col156) * (x_diff_24_tmp_d9509_26))),
                    ((((slope_limb_25_col154) * (x_diff_27_tmp_d9509_29))
                        + ((slope_limb_26_col155) * (x_diff_26_tmp_d9509_28)))
                        + ((slope_limb_27_col156) * (x_diff_25_tmp_d9509_27))),
                    (((slope_limb_26_col155) * (x_diff_27_tmp_d9509_29))
                        + ((slope_limb_27_col156) * (x_diff_26_tmp_d9509_28))),
                    ((slope_limb_27_col156) * (x_diff_27_tmp_d9509_29)),
                ];
                let x_sum_tmp_d9509_65 = [
                    ((slope_limb_14_col143) + (slope_limb_21_col150)),
                    ((slope_limb_15_col144) + (slope_limb_22_col151)),
                    ((slope_limb_16_col145) + (slope_limb_23_col152)),
                    ((slope_limb_17_col146) + (slope_limb_24_col153)),
                    ((slope_limb_18_col147) + (slope_limb_25_col154)),
                    ((slope_limb_19_col148) + (slope_limb_26_col155)),
                    ((slope_limb_20_col149) + (slope_limb_27_col156)),
                ];
                let y_sum_tmp_d9509_66 = [
                    ((x_diff_14_tmp_d9509_16) + (x_diff_21_tmp_d9509_23)),
                    ((x_diff_15_tmp_d9509_17) + (x_diff_22_tmp_d9509_24)),
                    ((x_diff_16_tmp_d9509_18) + (x_diff_23_tmp_d9509_25)),
                    ((x_diff_17_tmp_d9509_19) + (x_diff_24_tmp_d9509_26)),
                    ((x_diff_18_tmp_d9509_20) + (x_diff_25_tmp_d9509_27)),
                    ((x_diff_19_tmp_d9509_21) + (x_diff_26_tmp_d9509_28)),
                    ((x_diff_20_tmp_d9509_22) + (x_diff_27_tmp_d9509_29)),
                ];
                let single_karatsuba_n_7_output_tmp_d9509_67 = [
                    z0_tmp_d9509_63[0],
                    z0_tmp_d9509_63[1],
                    z0_tmp_d9509_63[2],
                    z0_tmp_d9509_63[3],
                    z0_tmp_d9509_63[4],
                    z0_tmp_d9509_63[5],
                    z0_tmp_d9509_63[6],
                    ((z0_tmp_d9509_63[7])
                        + ((((x_sum_tmp_d9509_65[0]) * (y_sum_tmp_d9509_66[0]))
                            - (z0_tmp_d9509_63[0]))
                            - (z2_tmp_d9509_64[0]))),
                    ((z0_tmp_d9509_63[8])
                        + (((((x_sum_tmp_d9509_65[0]) * (y_sum_tmp_d9509_66[1]))
                            + ((x_sum_tmp_d9509_65[1]) * (y_sum_tmp_d9509_66[0])))
                            - (z0_tmp_d9509_63[1]))
                            - (z2_tmp_d9509_64[1]))),
                    ((z0_tmp_d9509_63[9])
                        + ((((((x_sum_tmp_d9509_65[0]) * (y_sum_tmp_d9509_66[2]))
                            + ((x_sum_tmp_d9509_65[1]) * (y_sum_tmp_d9509_66[1])))
                            + ((x_sum_tmp_d9509_65[2]) * (y_sum_tmp_d9509_66[0])))
                            - (z0_tmp_d9509_63[2]))
                            - (z2_tmp_d9509_64[2]))),
                    ((z0_tmp_d9509_63[10])
                        + (((((((x_sum_tmp_d9509_65[0]) * (y_sum_tmp_d9509_66[3]))
                            + ((x_sum_tmp_d9509_65[1]) * (y_sum_tmp_d9509_66[2])))
                            + ((x_sum_tmp_d9509_65[2]) * (y_sum_tmp_d9509_66[1])))
                            + ((x_sum_tmp_d9509_65[3]) * (y_sum_tmp_d9509_66[0])))
                            - (z0_tmp_d9509_63[3]))
                            - (z2_tmp_d9509_64[3]))),
                    ((z0_tmp_d9509_63[11])
                        + ((((((((x_sum_tmp_d9509_65[0]) * (y_sum_tmp_d9509_66[4]))
                            + ((x_sum_tmp_d9509_65[1]) * (y_sum_tmp_d9509_66[3])))
                            + ((x_sum_tmp_d9509_65[2]) * (y_sum_tmp_d9509_66[2])))
                            + ((x_sum_tmp_d9509_65[3]) * (y_sum_tmp_d9509_66[1])))
                            + ((x_sum_tmp_d9509_65[4]) * (y_sum_tmp_d9509_66[0])))
                            - (z0_tmp_d9509_63[4]))
                            - (z2_tmp_d9509_64[4]))),
                    ((z0_tmp_d9509_63[12])
                        + (((((((((x_sum_tmp_d9509_65[0]) * (y_sum_tmp_d9509_66[5]))
                            + ((x_sum_tmp_d9509_65[1]) * (y_sum_tmp_d9509_66[4])))
                            + ((x_sum_tmp_d9509_65[2]) * (y_sum_tmp_d9509_66[3])))
                            + ((x_sum_tmp_d9509_65[3]) * (y_sum_tmp_d9509_66[2])))
                            + ((x_sum_tmp_d9509_65[4]) * (y_sum_tmp_d9509_66[1])))
                            + ((x_sum_tmp_d9509_65[5]) * (y_sum_tmp_d9509_66[0])))
                            - (z0_tmp_d9509_63[5]))
                            - (z2_tmp_d9509_64[5]))),
                    ((((((((((x_sum_tmp_d9509_65[0]) * (y_sum_tmp_d9509_66[6]))
                        + ((x_sum_tmp_d9509_65[1]) * (y_sum_tmp_d9509_66[5])))
                        + ((x_sum_tmp_d9509_65[2]) * (y_sum_tmp_d9509_66[4])))
                        + ((x_sum_tmp_d9509_65[3]) * (y_sum_tmp_d9509_66[3])))
                        + ((x_sum_tmp_d9509_65[4]) * (y_sum_tmp_d9509_66[2])))
                        + ((x_sum_tmp_d9509_65[5]) * (y_sum_tmp_d9509_66[1])))
                        + ((x_sum_tmp_d9509_65[6]) * (y_sum_tmp_d9509_66[0])))
                        - (z0_tmp_d9509_63[6]))
                        - (z2_tmp_d9509_64[6])),
                    ((z2_tmp_d9509_64[0])
                        + (((((((((x_sum_tmp_d9509_65[1]) * (y_sum_tmp_d9509_66[6]))
                            + ((x_sum_tmp_d9509_65[2]) * (y_sum_tmp_d9509_66[5])))
                            + ((x_sum_tmp_d9509_65[3]) * (y_sum_tmp_d9509_66[4])))
                            + ((x_sum_tmp_d9509_65[4]) * (y_sum_tmp_d9509_66[3])))
                            + ((x_sum_tmp_d9509_65[5]) * (y_sum_tmp_d9509_66[2])))
                            + ((x_sum_tmp_d9509_65[6]) * (y_sum_tmp_d9509_66[1])))
                            - (z0_tmp_d9509_63[7]))
                            - (z2_tmp_d9509_64[7]))),
                    ((z2_tmp_d9509_64[1])
                        + ((((((((x_sum_tmp_d9509_65[2]) * (y_sum_tmp_d9509_66[6]))
                            + ((x_sum_tmp_d9509_65[3]) * (y_sum_tmp_d9509_66[5])))
                            + ((x_sum_tmp_d9509_65[4]) * (y_sum_tmp_d9509_66[4])))
                            + ((x_sum_tmp_d9509_65[5]) * (y_sum_tmp_d9509_66[3])))
                            + ((x_sum_tmp_d9509_65[6]) * (y_sum_tmp_d9509_66[2])))
                            - (z0_tmp_d9509_63[8]))
                            - (z2_tmp_d9509_64[8]))),
                    ((z2_tmp_d9509_64[2])
                        + (((((((x_sum_tmp_d9509_65[3]) * (y_sum_tmp_d9509_66[6]))
                            + ((x_sum_tmp_d9509_65[4]) * (y_sum_tmp_d9509_66[5])))
                            + ((x_sum_tmp_d9509_65[5]) * (y_sum_tmp_d9509_66[4])))
                            + ((x_sum_tmp_d9509_65[6]) * (y_sum_tmp_d9509_66[3])))
                            - (z0_tmp_d9509_63[9]))
                            - (z2_tmp_d9509_64[9]))),
                    ((z2_tmp_d9509_64[3])
                        + ((((((x_sum_tmp_d9509_65[4]) * (y_sum_tmp_d9509_66[6]))
                            + ((x_sum_tmp_d9509_65[5]) * (y_sum_tmp_d9509_66[5])))
                            + ((x_sum_tmp_d9509_65[6]) * (y_sum_tmp_d9509_66[4])))
                            - (z0_tmp_d9509_63[10]))
                            - (z2_tmp_d9509_64[10]))),
                    ((z2_tmp_d9509_64[4])
                        + (((((x_sum_tmp_d9509_65[5]) * (y_sum_tmp_d9509_66[6]))
                            + ((x_sum_tmp_d9509_65[6]) * (y_sum_tmp_d9509_66[5])))
                            - (z0_tmp_d9509_63[11]))
                            - (z2_tmp_d9509_64[11]))),
                    ((z2_tmp_d9509_64[5])
                        + ((((x_sum_tmp_d9509_65[6]) * (y_sum_tmp_d9509_66[6]))
                            - (z0_tmp_d9509_63[12]))
                            - (z2_tmp_d9509_64[12]))),
                    z2_tmp_d9509_64[6],
                    z2_tmp_d9509_64[7],
                    z2_tmp_d9509_64[8],
                    z2_tmp_d9509_64[9],
                    z2_tmp_d9509_64[10],
                    z2_tmp_d9509_64[11],
                    z2_tmp_d9509_64[12],
                ];

                let x_sum_tmp_d9509_68 = [
                    ((slope_limb_0_col129) + (slope_limb_14_col143)),
                    ((slope_limb_1_col130) + (slope_limb_15_col144)),
                    ((slope_limb_2_col131) + (slope_limb_16_col145)),
                    ((slope_limb_3_col132) + (slope_limb_17_col146)),
                    ((slope_limb_4_col133) + (slope_limb_18_col147)),
                    ((slope_limb_5_col134) + (slope_limb_19_col148)),
                    ((slope_limb_6_col135) + (slope_limb_20_col149)),
                    ((slope_limb_7_col136) + (slope_limb_21_col150)),
                    ((slope_limb_8_col137) + (slope_limb_22_col151)),
                    ((slope_limb_9_col138) + (slope_limb_23_col152)),
                    ((slope_limb_10_col139) + (slope_limb_24_col153)),
                    ((slope_limb_11_col140) + (slope_limb_25_col154)),
                    ((slope_limb_12_col141) + (slope_limb_26_col155)),
                    ((slope_limb_13_col142) + (slope_limb_27_col156)),
                ];
                let y_sum_tmp_d9509_69 = [
                    ((x_diff_0_tmp_d9509_2) + (x_diff_14_tmp_d9509_16)),
                    ((x_diff_1_tmp_d9509_3) + (x_diff_15_tmp_d9509_17)),
                    ((x_diff_2_tmp_d9509_4) + (x_diff_16_tmp_d9509_18)),
                    ((x_diff_3_tmp_d9509_5) + (x_diff_17_tmp_d9509_19)),
                    ((x_diff_4_tmp_d9509_6) + (x_diff_18_tmp_d9509_20)),
                    ((x_diff_5_tmp_d9509_7) + (x_diff_19_tmp_d9509_21)),
                    ((x_diff_6_tmp_d9509_8) + (x_diff_20_tmp_d9509_22)),
                    ((x_diff_7_tmp_d9509_9) + (x_diff_21_tmp_d9509_23)),
                    ((x_diff_8_tmp_d9509_10) + (x_diff_22_tmp_d9509_24)),
                    ((x_diff_9_tmp_d9509_11) + (x_diff_23_tmp_d9509_25)),
                    ((x_diff_10_tmp_d9509_12) + (x_diff_24_tmp_d9509_26)),
                    ((x_diff_11_tmp_d9509_13) + (x_diff_25_tmp_d9509_27)),
                    ((x_diff_12_tmp_d9509_14) + (x_diff_26_tmp_d9509_28)),
                    ((x_diff_13_tmp_d9509_15) + (x_diff_27_tmp_d9509_29)),
                ];

                // Single Karatsuba N 7.

                let z0_tmp_d9509_70 = [
                    ((x_sum_tmp_d9509_68[0]) * (y_sum_tmp_d9509_69[0])),
                    (((x_sum_tmp_d9509_68[0]) * (y_sum_tmp_d9509_69[1]))
                        + ((x_sum_tmp_d9509_68[1]) * (y_sum_tmp_d9509_69[0]))),
                    ((((x_sum_tmp_d9509_68[0]) * (y_sum_tmp_d9509_69[2]))
                        + ((x_sum_tmp_d9509_68[1]) * (y_sum_tmp_d9509_69[1])))
                        + ((x_sum_tmp_d9509_68[2]) * (y_sum_tmp_d9509_69[0]))),
                    (((((x_sum_tmp_d9509_68[0]) * (y_sum_tmp_d9509_69[3]))
                        + ((x_sum_tmp_d9509_68[1]) * (y_sum_tmp_d9509_69[2])))
                        + ((x_sum_tmp_d9509_68[2]) * (y_sum_tmp_d9509_69[1])))
                        + ((x_sum_tmp_d9509_68[3]) * (y_sum_tmp_d9509_69[0]))),
                    ((((((x_sum_tmp_d9509_68[0]) * (y_sum_tmp_d9509_69[4]))
                        + ((x_sum_tmp_d9509_68[1]) * (y_sum_tmp_d9509_69[3])))
                        + ((x_sum_tmp_d9509_68[2]) * (y_sum_tmp_d9509_69[2])))
                        + ((x_sum_tmp_d9509_68[3]) * (y_sum_tmp_d9509_69[1])))
                        + ((x_sum_tmp_d9509_68[4]) * (y_sum_tmp_d9509_69[0]))),
                    (((((((x_sum_tmp_d9509_68[0]) * (y_sum_tmp_d9509_69[5]))
                        + ((x_sum_tmp_d9509_68[1]) * (y_sum_tmp_d9509_69[4])))
                        + ((x_sum_tmp_d9509_68[2]) * (y_sum_tmp_d9509_69[3])))
                        + ((x_sum_tmp_d9509_68[3]) * (y_sum_tmp_d9509_69[2])))
                        + ((x_sum_tmp_d9509_68[4]) * (y_sum_tmp_d9509_69[1])))
                        + ((x_sum_tmp_d9509_68[5]) * (y_sum_tmp_d9509_69[0]))),
                    ((((((((x_sum_tmp_d9509_68[0]) * (y_sum_tmp_d9509_69[6]))
                        + ((x_sum_tmp_d9509_68[1]) * (y_sum_tmp_d9509_69[5])))
                        + ((x_sum_tmp_d9509_68[2]) * (y_sum_tmp_d9509_69[4])))
                        + ((x_sum_tmp_d9509_68[3]) * (y_sum_tmp_d9509_69[3])))
                        + ((x_sum_tmp_d9509_68[4]) * (y_sum_tmp_d9509_69[2])))
                        + ((x_sum_tmp_d9509_68[5]) * (y_sum_tmp_d9509_69[1])))
                        + ((x_sum_tmp_d9509_68[6]) * (y_sum_tmp_d9509_69[0]))),
                    (((((((x_sum_tmp_d9509_68[1]) * (y_sum_tmp_d9509_69[6]))
                        + ((x_sum_tmp_d9509_68[2]) * (y_sum_tmp_d9509_69[5])))
                        + ((x_sum_tmp_d9509_68[3]) * (y_sum_tmp_d9509_69[4])))
                        + ((x_sum_tmp_d9509_68[4]) * (y_sum_tmp_d9509_69[3])))
                        + ((x_sum_tmp_d9509_68[5]) * (y_sum_tmp_d9509_69[2])))
                        + ((x_sum_tmp_d9509_68[6]) * (y_sum_tmp_d9509_69[1]))),
                    ((((((x_sum_tmp_d9509_68[2]) * (y_sum_tmp_d9509_69[6]))
                        + ((x_sum_tmp_d9509_68[3]) * (y_sum_tmp_d9509_69[5])))
                        + ((x_sum_tmp_d9509_68[4]) * (y_sum_tmp_d9509_69[4])))
                        + ((x_sum_tmp_d9509_68[5]) * (y_sum_tmp_d9509_69[3])))
                        + ((x_sum_tmp_d9509_68[6]) * (y_sum_tmp_d9509_69[2]))),
                    (((((x_sum_tmp_d9509_68[3]) * (y_sum_tmp_d9509_69[6]))
                        + ((x_sum_tmp_d9509_68[4]) * (y_sum_tmp_d9509_69[5])))
                        + ((x_sum_tmp_d9509_68[5]) * (y_sum_tmp_d9509_69[4])))
                        + ((x_sum_tmp_d9509_68[6]) * (y_sum_tmp_d9509_69[3]))),
                    ((((x_sum_tmp_d9509_68[4]) * (y_sum_tmp_d9509_69[6]))
                        + ((x_sum_tmp_d9509_68[5]) * (y_sum_tmp_d9509_69[5])))
                        + ((x_sum_tmp_d9509_68[6]) * (y_sum_tmp_d9509_69[4]))),
                    (((x_sum_tmp_d9509_68[5]) * (y_sum_tmp_d9509_69[6]))
                        + ((x_sum_tmp_d9509_68[6]) * (y_sum_tmp_d9509_69[5]))),
                    ((x_sum_tmp_d9509_68[6]) * (y_sum_tmp_d9509_69[6])),
                ];
                let z2_tmp_d9509_71 = [
                    ((x_sum_tmp_d9509_68[7]) * (y_sum_tmp_d9509_69[7])),
                    (((x_sum_tmp_d9509_68[7]) * (y_sum_tmp_d9509_69[8]))
                        + ((x_sum_tmp_d9509_68[8]) * (y_sum_tmp_d9509_69[7]))),
                    ((((x_sum_tmp_d9509_68[7]) * (y_sum_tmp_d9509_69[9]))
                        + ((x_sum_tmp_d9509_68[8]) * (y_sum_tmp_d9509_69[8])))
                        + ((x_sum_tmp_d9509_68[9]) * (y_sum_tmp_d9509_69[7]))),
                    (((((x_sum_tmp_d9509_68[7]) * (y_sum_tmp_d9509_69[10]))
                        + ((x_sum_tmp_d9509_68[8]) * (y_sum_tmp_d9509_69[9])))
                        + ((x_sum_tmp_d9509_68[9]) * (y_sum_tmp_d9509_69[8])))
                        + ((x_sum_tmp_d9509_68[10]) * (y_sum_tmp_d9509_69[7]))),
                    ((((((x_sum_tmp_d9509_68[7]) * (y_sum_tmp_d9509_69[11]))
                        + ((x_sum_tmp_d9509_68[8]) * (y_sum_tmp_d9509_69[10])))
                        + ((x_sum_tmp_d9509_68[9]) * (y_sum_tmp_d9509_69[9])))
                        + ((x_sum_tmp_d9509_68[10]) * (y_sum_tmp_d9509_69[8])))
                        + ((x_sum_tmp_d9509_68[11]) * (y_sum_tmp_d9509_69[7]))),
                    (((((((x_sum_tmp_d9509_68[7]) * (y_sum_tmp_d9509_69[12]))
                        + ((x_sum_tmp_d9509_68[8]) * (y_sum_tmp_d9509_69[11])))
                        + ((x_sum_tmp_d9509_68[9]) * (y_sum_tmp_d9509_69[10])))
                        + ((x_sum_tmp_d9509_68[10]) * (y_sum_tmp_d9509_69[9])))
                        + ((x_sum_tmp_d9509_68[11]) * (y_sum_tmp_d9509_69[8])))
                        + ((x_sum_tmp_d9509_68[12]) * (y_sum_tmp_d9509_69[7]))),
                    ((((((((x_sum_tmp_d9509_68[7]) * (y_sum_tmp_d9509_69[13]))
                        + ((x_sum_tmp_d9509_68[8]) * (y_sum_tmp_d9509_69[12])))
                        + ((x_sum_tmp_d9509_68[9]) * (y_sum_tmp_d9509_69[11])))
                        + ((x_sum_tmp_d9509_68[10]) * (y_sum_tmp_d9509_69[10])))
                        + ((x_sum_tmp_d9509_68[11]) * (y_sum_tmp_d9509_69[9])))
                        + ((x_sum_tmp_d9509_68[12]) * (y_sum_tmp_d9509_69[8])))
                        + ((x_sum_tmp_d9509_68[13]) * (y_sum_tmp_d9509_69[7]))),
                    (((((((x_sum_tmp_d9509_68[8]) * (y_sum_tmp_d9509_69[13]))
                        + ((x_sum_tmp_d9509_68[9]) * (y_sum_tmp_d9509_69[12])))
                        + ((x_sum_tmp_d9509_68[10]) * (y_sum_tmp_d9509_69[11])))
                        + ((x_sum_tmp_d9509_68[11]) * (y_sum_tmp_d9509_69[10])))
                        + ((x_sum_tmp_d9509_68[12]) * (y_sum_tmp_d9509_69[9])))
                        + ((x_sum_tmp_d9509_68[13]) * (y_sum_tmp_d9509_69[8]))),
                    ((((((x_sum_tmp_d9509_68[9]) * (y_sum_tmp_d9509_69[13]))
                        + ((x_sum_tmp_d9509_68[10]) * (y_sum_tmp_d9509_69[12])))
                        + ((x_sum_tmp_d9509_68[11]) * (y_sum_tmp_d9509_69[11])))
                        + ((x_sum_tmp_d9509_68[12]) * (y_sum_tmp_d9509_69[10])))
                        + ((x_sum_tmp_d9509_68[13]) * (y_sum_tmp_d9509_69[9]))),
                    (((((x_sum_tmp_d9509_68[10]) * (y_sum_tmp_d9509_69[13]))
                        + ((x_sum_tmp_d9509_68[11]) * (y_sum_tmp_d9509_69[12])))
                        + ((x_sum_tmp_d9509_68[12]) * (y_sum_tmp_d9509_69[11])))
                        + ((x_sum_tmp_d9509_68[13]) * (y_sum_tmp_d9509_69[10]))),
                    ((((x_sum_tmp_d9509_68[11]) * (y_sum_tmp_d9509_69[13]))
                        + ((x_sum_tmp_d9509_68[12]) * (y_sum_tmp_d9509_69[12])))
                        + ((x_sum_tmp_d9509_68[13]) * (y_sum_tmp_d9509_69[11]))),
                    (((x_sum_tmp_d9509_68[12]) * (y_sum_tmp_d9509_69[13]))
                        + ((x_sum_tmp_d9509_68[13]) * (y_sum_tmp_d9509_69[12]))),
                    ((x_sum_tmp_d9509_68[13]) * (y_sum_tmp_d9509_69[13])),
                ];
                let x_sum_tmp_d9509_72 = [
                    ((x_sum_tmp_d9509_68[0]) + (x_sum_tmp_d9509_68[7])),
                    ((x_sum_tmp_d9509_68[1]) + (x_sum_tmp_d9509_68[8])),
                    ((x_sum_tmp_d9509_68[2]) + (x_sum_tmp_d9509_68[9])),
                    ((x_sum_tmp_d9509_68[3]) + (x_sum_tmp_d9509_68[10])),
                    ((x_sum_tmp_d9509_68[4]) + (x_sum_tmp_d9509_68[11])),
                    ((x_sum_tmp_d9509_68[5]) + (x_sum_tmp_d9509_68[12])),
                    ((x_sum_tmp_d9509_68[6]) + (x_sum_tmp_d9509_68[13])),
                ];
                let y_sum_tmp_d9509_73 = [
                    ((y_sum_tmp_d9509_69[0]) + (y_sum_tmp_d9509_69[7])),
                    ((y_sum_tmp_d9509_69[1]) + (y_sum_tmp_d9509_69[8])),
                    ((y_sum_tmp_d9509_69[2]) + (y_sum_tmp_d9509_69[9])),
                    ((y_sum_tmp_d9509_69[3]) + (y_sum_tmp_d9509_69[10])),
                    ((y_sum_tmp_d9509_69[4]) + (y_sum_tmp_d9509_69[11])),
                    ((y_sum_tmp_d9509_69[5]) + (y_sum_tmp_d9509_69[12])),
                    ((y_sum_tmp_d9509_69[6]) + (y_sum_tmp_d9509_69[13])),
                ];
                let single_karatsuba_n_7_output_tmp_d9509_74 = [
                    z0_tmp_d9509_70[0],
                    z0_tmp_d9509_70[1],
                    z0_tmp_d9509_70[2],
                    z0_tmp_d9509_70[3],
                    z0_tmp_d9509_70[4],
                    z0_tmp_d9509_70[5],
                    z0_tmp_d9509_70[6],
                    ((z0_tmp_d9509_70[7])
                        + ((((x_sum_tmp_d9509_72[0]) * (y_sum_tmp_d9509_73[0]))
                            - (z0_tmp_d9509_70[0]))
                            - (z2_tmp_d9509_71[0]))),
                    ((z0_tmp_d9509_70[8])
                        + (((((x_sum_tmp_d9509_72[0]) * (y_sum_tmp_d9509_73[1]))
                            + ((x_sum_tmp_d9509_72[1]) * (y_sum_tmp_d9509_73[0])))
                            - (z0_tmp_d9509_70[1]))
                            - (z2_tmp_d9509_71[1]))),
                    ((z0_tmp_d9509_70[9])
                        + ((((((x_sum_tmp_d9509_72[0]) * (y_sum_tmp_d9509_73[2]))
                            + ((x_sum_tmp_d9509_72[1]) * (y_sum_tmp_d9509_73[1])))
                            + ((x_sum_tmp_d9509_72[2]) * (y_sum_tmp_d9509_73[0])))
                            - (z0_tmp_d9509_70[2]))
                            - (z2_tmp_d9509_71[2]))),
                    ((z0_tmp_d9509_70[10])
                        + (((((((x_sum_tmp_d9509_72[0]) * (y_sum_tmp_d9509_73[3]))
                            + ((x_sum_tmp_d9509_72[1]) * (y_sum_tmp_d9509_73[2])))
                            + ((x_sum_tmp_d9509_72[2]) * (y_sum_tmp_d9509_73[1])))
                            + ((x_sum_tmp_d9509_72[3]) * (y_sum_tmp_d9509_73[0])))
                            - (z0_tmp_d9509_70[3]))
                            - (z2_tmp_d9509_71[3]))),
                    ((z0_tmp_d9509_70[11])
                        + ((((((((x_sum_tmp_d9509_72[0]) * (y_sum_tmp_d9509_73[4]))
                            + ((x_sum_tmp_d9509_72[1]) * (y_sum_tmp_d9509_73[3])))
                            + ((x_sum_tmp_d9509_72[2]) * (y_sum_tmp_d9509_73[2])))
                            + ((x_sum_tmp_d9509_72[3]) * (y_sum_tmp_d9509_73[1])))
                            + ((x_sum_tmp_d9509_72[4]) * (y_sum_tmp_d9509_73[0])))
                            - (z0_tmp_d9509_70[4]))
                            - (z2_tmp_d9509_71[4]))),
                    ((z0_tmp_d9509_70[12])
                        + (((((((((x_sum_tmp_d9509_72[0]) * (y_sum_tmp_d9509_73[5]))
                            + ((x_sum_tmp_d9509_72[1]) * (y_sum_tmp_d9509_73[4])))
                            + ((x_sum_tmp_d9509_72[2]) * (y_sum_tmp_d9509_73[3])))
                            + ((x_sum_tmp_d9509_72[3]) * (y_sum_tmp_d9509_73[2])))
                            + ((x_sum_tmp_d9509_72[4]) * (y_sum_tmp_d9509_73[1])))
                            + ((x_sum_tmp_d9509_72[5]) * (y_sum_tmp_d9509_73[0])))
                            - (z0_tmp_d9509_70[5]))
                            - (z2_tmp_d9509_71[5]))),
                    ((((((((((x_sum_tmp_d9509_72[0]) * (y_sum_tmp_d9509_73[6]))
                        + ((x_sum_tmp_d9509_72[1]) * (y_sum_tmp_d9509_73[5])))
                        + ((x_sum_tmp_d9509_72[2]) * (y_sum_tmp_d9509_73[4])))
                        + ((x_sum_tmp_d9509_72[3]) * (y_sum_tmp_d9509_73[3])))
                        + ((x_sum_tmp_d9509_72[4]) * (y_sum_tmp_d9509_73[2])))
                        + ((x_sum_tmp_d9509_72[5]) * (y_sum_tmp_d9509_73[1])))
                        + ((x_sum_tmp_d9509_72[6]) * (y_sum_tmp_d9509_73[0])))
                        - (z0_tmp_d9509_70[6]))
                        - (z2_tmp_d9509_71[6])),
                    ((z2_tmp_d9509_71[0])
                        + (((((((((x_sum_tmp_d9509_72[1]) * (y_sum_tmp_d9509_73[6]))
                            + ((x_sum_tmp_d9509_72[2]) * (y_sum_tmp_d9509_73[5])))
                            + ((x_sum_tmp_d9509_72[3]) * (y_sum_tmp_d9509_73[4])))
                            + ((x_sum_tmp_d9509_72[4]) * (y_sum_tmp_d9509_73[3])))
                            + ((x_sum_tmp_d9509_72[5]) * (y_sum_tmp_d9509_73[2])))
                            + ((x_sum_tmp_d9509_72[6]) * (y_sum_tmp_d9509_73[1])))
                            - (z0_tmp_d9509_70[7]))
                            - (z2_tmp_d9509_71[7]))),
                    ((z2_tmp_d9509_71[1])
                        + ((((((((x_sum_tmp_d9509_72[2]) * (y_sum_tmp_d9509_73[6]))
                            + ((x_sum_tmp_d9509_72[3]) * (y_sum_tmp_d9509_73[5])))
                            + ((x_sum_tmp_d9509_72[4]) * (y_sum_tmp_d9509_73[4])))
                            + ((x_sum_tmp_d9509_72[5]) * (y_sum_tmp_d9509_73[3])))
                            + ((x_sum_tmp_d9509_72[6]) * (y_sum_tmp_d9509_73[2])))
                            - (z0_tmp_d9509_70[8]))
                            - (z2_tmp_d9509_71[8]))),
                    ((z2_tmp_d9509_71[2])
                        + (((((((x_sum_tmp_d9509_72[3]) * (y_sum_tmp_d9509_73[6]))
                            + ((x_sum_tmp_d9509_72[4]) * (y_sum_tmp_d9509_73[5])))
                            + ((x_sum_tmp_d9509_72[5]) * (y_sum_tmp_d9509_73[4])))
                            + ((x_sum_tmp_d9509_72[6]) * (y_sum_tmp_d9509_73[3])))
                            - (z0_tmp_d9509_70[9]))
                            - (z2_tmp_d9509_71[9]))),
                    ((z2_tmp_d9509_71[3])
                        + ((((((x_sum_tmp_d9509_72[4]) * (y_sum_tmp_d9509_73[6]))
                            + ((x_sum_tmp_d9509_72[5]) * (y_sum_tmp_d9509_73[5])))
                            + ((x_sum_tmp_d9509_72[6]) * (y_sum_tmp_d9509_73[4])))
                            - (z0_tmp_d9509_70[10]))
                            - (z2_tmp_d9509_71[10]))),
                    ((z2_tmp_d9509_71[4])
                        + (((((x_sum_tmp_d9509_72[5]) * (y_sum_tmp_d9509_73[6]))
                            + ((x_sum_tmp_d9509_72[6]) * (y_sum_tmp_d9509_73[5])))
                            - (z0_tmp_d9509_70[11]))
                            - (z2_tmp_d9509_71[11]))),
                    ((z2_tmp_d9509_71[5])
                        + ((((x_sum_tmp_d9509_72[6]) * (y_sum_tmp_d9509_73[6]))
                            - (z0_tmp_d9509_70[12]))
                            - (z2_tmp_d9509_71[12]))),
                    z2_tmp_d9509_71[6],
                    z2_tmp_d9509_71[7],
                    z2_tmp_d9509_71[8],
                    z2_tmp_d9509_71[9],
                    z2_tmp_d9509_71[10],
                    z2_tmp_d9509_71[11],
                    z2_tmp_d9509_71[12],
                ];

                let double_karatsuba_f0fc6_output_tmp_d9509_75 = [
                    single_karatsuba_n_7_output_tmp_d9509_62[0],
                    single_karatsuba_n_7_output_tmp_d9509_62[1],
                    single_karatsuba_n_7_output_tmp_d9509_62[2],
                    single_karatsuba_n_7_output_tmp_d9509_62[3],
                    single_karatsuba_n_7_output_tmp_d9509_62[4],
                    single_karatsuba_n_7_output_tmp_d9509_62[5],
                    single_karatsuba_n_7_output_tmp_d9509_62[6],
                    single_karatsuba_n_7_output_tmp_d9509_62[7],
                    single_karatsuba_n_7_output_tmp_d9509_62[8],
                    single_karatsuba_n_7_output_tmp_d9509_62[9],
                    single_karatsuba_n_7_output_tmp_d9509_62[10],
                    single_karatsuba_n_7_output_tmp_d9509_62[11],
                    single_karatsuba_n_7_output_tmp_d9509_62[12],
                    single_karatsuba_n_7_output_tmp_d9509_62[13],
                    ((single_karatsuba_n_7_output_tmp_d9509_62[14])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[0])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[0]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[0]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_62[15])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[1])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[1]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[1]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_62[16])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[2])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[2]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[2]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_62[17])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[3])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[3]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[3]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_62[18])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[4])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[4]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[4]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_62[19])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[5])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[5]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[5]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_62[20])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[6])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[6]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[6]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_62[21])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[7])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[7]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[7]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_62[22])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[8])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[8]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[8]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_62[23])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[9])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[9]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[9]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_62[24])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[10])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[10]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[10]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_62[25])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[11])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[11]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[11]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_62[26])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[12])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[12]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[12]))),
                    (((single_karatsuba_n_7_output_tmp_d9509_74[13])
                        - (single_karatsuba_n_7_output_tmp_d9509_62[13]))
                        - (single_karatsuba_n_7_output_tmp_d9509_67[13])),
                    ((single_karatsuba_n_7_output_tmp_d9509_67[0])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[14])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[14]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[14]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_67[1])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[15])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[15]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[15]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_67[2])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[16])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[16]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[16]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_67[3])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[17])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[17]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[17]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_67[4])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[18])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[18]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[18]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_67[5])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[19])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[19]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[19]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_67[6])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[20])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[20]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[20]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_67[7])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[21])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[21]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[21]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_67[8])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[22])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[22]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[22]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_67[9])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[23])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[23]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[23]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_67[10])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[24])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[24]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[24]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_67[11])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[25])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[25]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[25]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_67[12])
                        + (((single_karatsuba_n_7_output_tmp_d9509_74[26])
                            - (single_karatsuba_n_7_output_tmp_d9509_62[26]))
                            - (single_karatsuba_n_7_output_tmp_d9509_67[26]))),
                    single_karatsuba_n_7_output_tmp_d9509_67[13],
                    single_karatsuba_n_7_output_tmp_d9509_67[14],
                    single_karatsuba_n_7_output_tmp_d9509_67[15],
                    single_karatsuba_n_7_output_tmp_d9509_67[16],
                    single_karatsuba_n_7_output_tmp_d9509_67[17],
                    single_karatsuba_n_7_output_tmp_d9509_67[18],
                    single_karatsuba_n_7_output_tmp_d9509_67[19],
                    single_karatsuba_n_7_output_tmp_d9509_67[20],
                    single_karatsuba_n_7_output_tmp_d9509_67[21],
                    single_karatsuba_n_7_output_tmp_d9509_67[22],
                    single_karatsuba_n_7_output_tmp_d9509_67[23],
                    single_karatsuba_n_7_output_tmp_d9509_67[24],
                    single_karatsuba_n_7_output_tmp_d9509_67[25],
                    single_karatsuba_n_7_output_tmp_d9509_67[26],
                ];

                let conv_tmp_d9509_76 = [
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[0]) - (y_diff_0_tmp_d9509_30)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[1]) - (y_diff_1_tmp_d9509_31)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[2]) - (y_diff_2_tmp_d9509_32)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[3]) - (y_diff_3_tmp_d9509_33)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[4]) - (y_diff_4_tmp_d9509_34)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[5]) - (y_diff_5_tmp_d9509_35)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[6]) - (y_diff_6_tmp_d9509_36)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[7]) - (y_diff_7_tmp_d9509_37)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[8]) - (y_diff_8_tmp_d9509_38)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[9]) - (y_diff_9_tmp_d9509_39)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[10]) - (y_diff_10_tmp_d9509_40)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[11]) - (y_diff_11_tmp_d9509_41)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[12]) - (y_diff_12_tmp_d9509_42)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[13]) - (y_diff_13_tmp_d9509_43)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[14]) - (y_diff_14_tmp_d9509_44)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[15]) - (y_diff_15_tmp_d9509_45)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[16]) - (y_diff_16_tmp_d9509_46)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[17]) - (y_diff_17_tmp_d9509_47)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[18]) - (y_diff_18_tmp_d9509_48)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[19]) - (y_diff_19_tmp_d9509_49)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[20]) - (y_diff_20_tmp_d9509_50)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[21]) - (y_diff_21_tmp_d9509_51)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[22]) - (y_diff_22_tmp_d9509_52)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[23]) - (y_diff_23_tmp_d9509_53)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[24]) - (y_diff_24_tmp_d9509_54)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[25]) - (y_diff_25_tmp_d9509_55)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[26]) - (y_diff_26_tmp_d9509_56)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_75[27]) - (y_diff_27_tmp_d9509_57)),
                    double_karatsuba_f0fc6_output_tmp_d9509_75[28],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[29],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[30],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[31],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[32],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[33],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[34],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[35],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[36],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[37],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[38],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[39],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[40],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[41],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[42],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[43],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[44],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[45],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[46],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[47],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[48],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[49],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[50],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[51],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[52],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[53],
                    double_karatsuba_f0fc6_output_tmp_d9509_75[54],
                ];
                let conv_mod_tmp_d9509_77 = [
                    ((((M31_32) * (conv_tmp_d9509_76[0])) - ((M31_4) * (conv_tmp_d9509_76[21])))
                        + ((M31_8) * (conv_tmp_d9509_76[49]))),
                    ((((conv_tmp_d9509_76[0]) + ((M31_32) * (conv_tmp_d9509_76[1])))
                        - ((M31_4) * (conv_tmp_d9509_76[22])))
                        + ((M31_8) * (conv_tmp_d9509_76[50]))),
                    ((((conv_tmp_d9509_76[1]) + ((M31_32) * (conv_tmp_d9509_76[2])))
                        - ((M31_4) * (conv_tmp_d9509_76[23])))
                        + ((M31_8) * (conv_tmp_d9509_76[51]))),
                    ((((conv_tmp_d9509_76[2]) + ((M31_32) * (conv_tmp_d9509_76[3])))
                        - ((M31_4) * (conv_tmp_d9509_76[24])))
                        + ((M31_8) * (conv_tmp_d9509_76[52]))),
                    ((((conv_tmp_d9509_76[3]) + ((M31_32) * (conv_tmp_d9509_76[4])))
                        - ((M31_4) * (conv_tmp_d9509_76[25])))
                        + ((M31_8) * (conv_tmp_d9509_76[53]))),
                    ((((conv_tmp_d9509_76[4]) + ((M31_32) * (conv_tmp_d9509_76[5])))
                        - ((M31_4) * (conv_tmp_d9509_76[26])))
                        + ((M31_8) * (conv_tmp_d9509_76[54]))),
                    (((conv_tmp_d9509_76[5]) + ((M31_32) * (conv_tmp_d9509_76[6])))
                        - ((M31_4) * (conv_tmp_d9509_76[27]))),
                    (((((M31_2) * (conv_tmp_d9509_76[0])) + (conv_tmp_d9509_76[6]))
                        + ((M31_32) * (conv_tmp_d9509_76[7])))
                        - ((M31_4) * (conv_tmp_d9509_76[28]))),
                    (((((M31_2) * (conv_tmp_d9509_76[1])) + (conv_tmp_d9509_76[7]))
                        + ((M31_32) * (conv_tmp_d9509_76[8])))
                        - ((M31_4) * (conv_tmp_d9509_76[29]))),
                    (((((M31_2) * (conv_tmp_d9509_76[2])) + (conv_tmp_d9509_76[8]))
                        + ((M31_32) * (conv_tmp_d9509_76[9])))
                        - ((M31_4) * (conv_tmp_d9509_76[30]))),
                    (((((M31_2) * (conv_tmp_d9509_76[3])) + (conv_tmp_d9509_76[9]))
                        + ((M31_32) * (conv_tmp_d9509_76[10])))
                        - ((M31_4) * (conv_tmp_d9509_76[31]))),
                    (((((M31_2) * (conv_tmp_d9509_76[4])) + (conv_tmp_d9509_76[10]))
                        + ((M31_32) * (conv_tmp_d9509_76[11])))
                        - ((M31_4) * (conv_tmp_d9509_76[32]))),
                    (((((M31_2) * (conv_tmp_d9509_76[5])) + (conv_tmp_d9509_76[11]))
                        + ((M31_32) * (conv_tmp_d9509_76[12])))
                        - ((M31_4) * (conv_tmp_d9509_76[33]))),
                    (((((M31_2) * (conv_tmp_d9509_76[6])) + (conv_tmp_d9509_76[12]))
                        + ((M31_32) * (conv_tmp_d9509_76[13])))
                        - ((M31_4) * (conv_tmp_d9509_76[34]))),
                    (((((M31_2) * (conv_tmp_d9509_76[7])) + (conv_tmp_d9509_76[13]))
                        + ((M31_32) * (conv_tmp_d9509_76[14])))
                        - ((M31_4) * (conv_tmp_d9509_76[35]))),
                    (((((M31_2) * (conv_tmp_d9509_76[8])) + (conv_tmp_d9509_76[14]))
                        + ((M31_32) * (conv_tmp_d9509_76[15])))
                        - ((M31_4) * (conv_tmp_d9509_76[36]))),
                    (((((M31_2) * (conv_tmp_d9509_76[9])) + (conv_tmp_d9509_76[15]))
                        + ((M31_32) * (conv_tmp_d9509_76[16])))
                        - ((M31_4) * (conv_tmp_d9509_76[37]))),
                    (((((M31_2) * (conv_tmp_d9509_76[10])) + (conv_tmp_d9509_76[16]))
                        + ((M31_32) * (conv_tmp_d9509_76[17])))
                        - ((M31_4) * (conv_tmp_d9509_76[38]))),
                    (((((M31_2) * (conv_tmp_d9509_76[11])) + (conv_tmp_d9509_76[17]))
                        + ((M31_32) * (conv_tmp_d9509_76[18])))
                        - ((M31_4) * (conv_tmp_d9509_76[39]))),
                    (((((M31_2) * (conv_tmp_d9509_76[12])) + (conv_tmp_d9509_76[18]))
                        + ((M31_32) * (conv_tmp_d9509_76[19])))
                        - ((M31_4) * (conv_tmp_d9509_76[40]))),
                    (((((M31_2) * (conv_tmp_d9509_76[13])) + (conv_tmp_d9509_76[19]))
                        + ((M31_32) * (conv_tmp_d9509_76[20])))
                        - ((M31_4) * (conv_tmp_d9509_76[41]))),
                    (((((M31_2) * (conv_tmp_d9509_76[14])) + (conv_tmp_d9509_76[20]))
                        - ((M31_4) * (conv_tmp_d9509_76[42])))
                        + ((M31_64) * (conv_tmp_d9509_76[49]))),
                    (((((M31_2) * (conv_tmp_d9509_76[15])) - ((M31_4) * (conv_tmp_d9509_76[43])))
                        + ((M31_2) * (conv_tmp_d9509_76[49])))
                        + ((M31_64) * (conv_tmp_d9509_76[50]))),
                    (((((M31_2) * (conv_tmp_d9509_76[16])) - ((M31_4) * (conv_tmp_d9509_76[44])))
                        + ((M31_2) * (conv_tmp_d9509_76[50])))
                        + ((M31_64) * (conv_tmp_d9509_76[51]))),
                    (((((M31_2) * (conv_tmp_d9509_76[17])) - ((M31_4) * (conv_tmp_d9509_76[45])))
                        + ((M31_2) * (conv_tmp_d9509_76[51])))
                        + ((M31_64) * (conv_tmp_d9509_76[52]))),
                    (((((M31_2) * (conv_tmp_d9509_76[18])) - ((M31_4) * (conv_tmp_d9509_76[46])))
                        + ((M31_2) * (conv_tmp_d9509_76[52])))
                        + ((M31_64) * (conv_tmp_d9509_76[53]))),
                    (((((M31_2) * (conv_tmp_d9509_76[19])) - ((M31_4) * (conv_tmp_d9509_76[47])))
                        + ((M31_2) * (conv_tmp_d9509_76[53])))
                        + ((M31_64) * (conv_tmp_d9509_76[54]))),
                    ((((M31_2) * (conv_tmp_d9509_76[20])) - ((M31_4) * (conv_tmp_d9509_76[48])))
                        + ((M31_2) * (conv_tmp_d9509_76[54]))),
                ];
                let k_mod_2_18_biased_tmp_d9509_78 =
                    ((((PackedUInt32::from_m31(((conv_mod_tmp_d9509_77[0]) + (M31_134217728))))
                        + (((PackedUInt32::from_m31(
                            ((conv_mod_tmp_d9509_77[1]) + (M31_134217728)),
                        )) & (UInt32_511))
                            << (UInt32_9)))
                        + (UInt32_131072))
                        & (UInt32_262143));
                let k_col157 = ((k_mod_2_18_biased_tmp_d9509_78.low().as_m31())
                    + (((k_mod_2_18_biased_tmp_d9509_78.high().as_m31()) - (M31_2)) * (M31_65536)));
                *row[157] = k_col157;
                *sub_component_inputs.range_check_20[0] = [((k_col157) + (M31_524288))];
                *lookup_data.range_check_20_15 = [M31_1410849886, ((k_col157) + (M31_524288))];
                let carry_0_col158 = (((conv_mod_tmp_d9509_77[0]) - (k_col157)) * (M31_4194304));
                *row[158] = carry_0_col158;
                *sub_component_inputs.range_check_20_b[0] = [((carry_0_col158) + (M31_524288))];
                *lookup_data.range_check_20_b_16 =
                    [M31_514232941, ((carry_0_col158) + (M31_524288))];
                let carry_1_col159 =
                    (((conv_mod_tmp_d9509_77[1]) + (carry_0_col158)) * (M31_4194304));
                *row[159] = carry_1_col159;
                *sub_component_inputs.range_check_20_c[0] = [((carry_1_col159) + (M31_524288))];
                *lookup_data.range_check_20_c_17 =
                    [M31_531010560, ((carry_1_col159) + (M31_524288))];
                let carry_2_col160 =
                    (((conv_mod_tmp_d9509_77[2]) + (carry_1_col159)) * (M31_4194304));
                *row[160] = carry_2_col160;
                *sub_component_inputs.range_check_20_d[0] = [((carry_2_col160) + (M31_524288))];
                *lookup_data.range_check_20_d_18 =
                    [M31_480677703, ((carry_2_col160) + (M31_524288))];
                let carry_3_col161 =
                    (((conv_mod_tmp_d9509_77[3]) + (carry_2_col160)) * (M31_4194304));
                *row[161] = carry_3_col161;
                *sub_component_inputs.range_check_20_e[0] = [((carry_3_col161) + (M31_524288))];
                *lookup_data.range_check_20_e_19 =
                    [M31_497455322, ((carry_3_col161) + (M31_524288))];
                let carry_4_col162 =
                    (((conv_mod_tmp_d9509_77[4]) + (carry_3_col161)) * (M31_4194304));
                *row[162] = carry_4_col162;
                *sub_component_inputs.range_check_20_f[0] = [((carry_4_col162) + (M31_524288))];
                *lookup_data.range_check_20_f_20 =
                    [M31_447122465, ((carry_4_col162) + (M31_524288))];
                let carry_5_col163 =
                    (((conv_mod_tmp_d9509_77[5]) + (carry_4_col162)) * (M31_4194304));
                *row[163] = carry_5_col163;
                *sub_component_inputs.range_check_20_g[0] = [((carry_5_col163) + (M31_524288))];
                *lookup_data.range_check_20_g_21 =
                    [M31_463900084, ((carry_5_col163) + (M31_524288))];
                let carry_6_col164 =
                    (((conv_mod_tmp_d9509_77[6]) + (carry_5_col163)) * (M31_4194304));
                *row[164] = carry_6_col164;
                *sub_component_inputs.range_check_20_h[0] = [((carry_6_col164) + (M31_524288))];
                *lookup_data.range_check_20_h_22 =
                    [M31_682009131, ((carry_6_col164) + (M31_524288))];
                let carry_7_col165 =
                    (((conv_mod_tmp_d9509_77[7]) + (carry_6_col164)) * (M31_4194304));
                *row[165] = carry_7_col165;
                *sub_component_inputs.range_check_20[1] = [((carry_7_col165) + (M31_524288))];
                *lookup_data.range_check_20_23 =
                    [M31_1410849886, ((carry_7_col165) + (M31_524288))];
                let carry_8_col166 =
                    (((conv_mod_tmp_d9509_77[8]) + (carry_7_col165)) * (M31_4194304));
                *row[166] = carry_8_col166;
                *sub_component_inputs.range_check_20_b[1] = [((carry_8_col166) + (M31_524288))];
                *lookup_data.range_check_20_b_24 =
                    [M31_514232941, ((carry_8_col166) + (M31_524288))];
                let carry_9_col167 =
                    (((conv_mod_tmp_d9509_77[9]) + (carry_8_col166)) * (M31_4194304));
                *row[167] = carry_9_col167;
                *sub_component_inputs.range_check_20_c[1] = [((carry_9_col167) + (M31_524288))];
                *lookup_data.range_check_20_c_25 =
                    [M31_531010560, ((carry_9_col167) + (M31_524288))];
                let carry_10_col168 =
                    (((conv_mod_tmp_d9509_77[10]) + (carry_9_col167)) * (M31_4194304));
                *row[168] = carry_10_col168;
                *sub_component_inputs.range_check_20_d[1] = [((carry_10_col168) + (M31_524288))];
                *lookup_data.range_check_20_d_26 =
                    [M31_480677703, ((carry_10_col168) + (M31_524288))];
                let carry_11_col169 =
                    (((conv_mod_tmp_d9509_77[11]) + (carry_10_col168)) * (M31_4194304));
                *row[169] = carry_11_col169;
                *sub_component_inputs.range_check_20_e[1] = [((carry_11_col169) + (M31_524288))];
                *lookup_data.range_check_20_e_27 =
                    [M31_497455322, ((carry_11_col169) + (M31_524288))];
                let carry_12_col170 =
                    (((conv_mod_tmp_d9509_77[12]) + (carry_11_col169)) * (M31_4194304));
                *row[170] = carry_12_col170;
                *sub_component_inputs.range_check_20_f[1] = [((carry_12_col170) + (M31_524288))];
                *lookup_data.range_check_20_f_28 =
                    [M31_447122465, ((carry_12_col170) + (M31_524288))];
                let carry_13_col171 =
                    (((conv_mod_tmp_d9509_77[13]) + (carry_12_col170)) * (M31_4194304));
                *row[171] = carry_13_col171;
                *sub_component_inputs.range_check_20_g[1] = [((carry_13_col171) + (M31_524288))];
                *lookup_data.range_check_20_g_29 =
                    [M31_463900084, ((carry_13_col171) + (M31_524288))];
                let carry_14_col172 =
                    (((conv_mod_tmp_d9509_77[14]) + (carry_13_col171)) * (M31_4194304));
                *row[172] = carry_14_col172;
                *sub_component_inputs.range_check_20_h[1] = [((carry_14_col172) + (M31_524288))];
                *lookup_data.range_check_20_h_30 =
                    [M31_682009131, ((carry_14_col172) + (M31_524288))];
                let carry_15_col173 =
                    (((conv_mod_tmp_d9509_77[15]) + (carry_14_col172)) * (M31_4194304));
                *row[173] = carry_15_col173;
                *sub_component_inputs.range_check_20[2] = [((carry_15_col173) + (M31_524288))];
                *lookup_data.range_check_20_31 =
                    [M31_1410849886, ((carry_15_col173) + (M31_524288))];
                let carry_16_col174 =
                    (((conv_mod_tmp_d9509_77[16]) + (carry_15_col173)) * (M31_4194304));
                *row[174] = carry_16_col174;
                *sub_component_inputs.range_check_20_b[2] = [((carry_16_col174) + (M31_524288))];
                *lookup_data.range_check_20_b_32 =
                    [M31_514232941, ((carry_16_col174) + (M31_524288))];
                let carry_17_col175 =
                    (((conv_mod_tmp_d9509_77[17]) + (carry_16_col174)) * (M31_4194304));
                *row[175] = carry_17_col175;
                *sub_component_inputs.range_check_20_c[2] = [((carry_17_col175) + (M31_524288))];
                *lookup_data.range_check_20_c_33 =
                    [M31_531010560, ((carry_17_col175) + (M31_524288))];
                let carry_18_col176 =
                    (((conv_mod_tmp_d9509_77[18]) + (carry_17_col175)) * (M31_4194304));
                *row[176] = carry_18_col176;
                *sub_component_inputs.range_check_20_d[2] = [((carry_18_col176) + (M31_524288))];
                *lookup_data.range_check_20_d_34 =
                    [M31_480677703, ((carry_18_col176) + (M31_524288))];
                let carry_19_col177 =
                    (((conv_mod_tmp_d9509_77[19]) + (carry_18_col176)) * (M31_4194304));
                *row[177] = carry_19_col177;
                *sub_component_inputs.range_check_20_e[2] = [((carry_19_col177) + (M31_524288))];
                *lookup_data.range_check_20_e_35 =
                    [M31_497455322, ((carry_19_col177) + (M31_524288))];
                let carry_20_col178 =
                    (((conv_mod_tmp_d9509_77[20]) + (carry_19_col177)) * (M31_4194304));
                *row[178] = carry_20_col178;
                *sub_component_inputs.range_check_20_f[2] = [((carry_20_col178) + (M31_524288))];
                *lookup_data.range_check_20_f_36 =
                    [M31_447122465, ((carry_20_col178) + (M31_524288))];
                let carry_21_col179 = ((((conv_mod_tmp_d9509_77[21]) - ((M31_136) * (k_col157)))
                    + (carry_20_col178))
                    * (M31_4194304));
                *row[179] = carry_21_col179;
                *sub_component_inputs.range_check_20_g[2] = [((carry_21_col179) + (M31_524288))];
                *lookup_data.range_check_20_g_37 =
                    [M31_463900084, ((carry_21_col179) + (M31_524288))];
                let carry_22_col180 =
                    (((conv_mod_tmp_d9509_77[22]) + (carry_21_col179)) * (M31_4194304));
                *row[180] = carry_22_col180;
                *sub_component_inputs.range_check_20_h[2] = [((carry_22_col180) + (M31_524288))];
                *lookup_data.range_check_20_h_38 =
                    [M31_682009131, ((carry_22_col180) + (M31_524288))];
                let carry_23_col181 =
                    (((conv_mod_tmp_d9509_77[23]) + (carry_22_col180)) * (M31_4194304));
                *row[181] = carry_23_col181;
                *sub_component_inputs.range_check_20[3] = [((carry_23_col181) + (M31_524288))];
                *lookup_data.range_check_20_39 =
                    [M31_1410849886, ((carry_23_col181) + (M31_524288))];
                let carry_24_col182 =
                    (((conv_mod_tmp_d9509_77[24]) + (carry_23_col181)) * (M31_4194304));
                *row[182] = carry_24_col182;
                *sub_component_inputs.range_check_20_b[3] = [((carry_24_col182) + (M31_524288))];
                *lookup_data.range_check_20_b_40 =
                    [M31_514232941, ((carry_24_col182) + (M31_524288))];
                let carry_25_col183 =
                    (((conv_mod_tmp_d9509_77[25]) + (carry_24_col182)) * (M31_4194304));
                *row[183] = carry_25_col183;
                *sub_component_inputs.range_check_20_c[3] = [((carry_25_col183) + (M31_524288))];
                *lookup_data.range_check_20_c_41 =
                    [M31_531010560, ((carry_25_col183) + (M31_524288))];
                let carry_26_col184 =
                    (((conv_mod_tmp_d9509_77[26]) + (carry_25_col183)) * (M31_4194304));
                *row[184] = carry_26_col184;
                *sub_component_inputs.range_check_20_d[3] = [((carry_26_col184) + (M31_524288))];
                *lookup_data.range_check_20_d_42 =
                    [M31_480677703, ((carry_26_col184) + (M31_524288))];

                let result_x_tmp_d9509_79 = ((((slope_tmp_d9509_1) * (slope_tmp_d9509_1))
                    - (partial_ec_mul_window_bits_18_input.2 .1[0]))
                    - (pedersen_points_table_window_bits_18_output_tmp_d9509_0[0]));
                let result_x_limb_0_col185 = result_x_tmp_d9509_79.get_m31(0);
                *row[185] = result_x_limb_0_col185;
                let result_x_limb_1_col186 = result_x_tmp_d9509_79.get_m31(1);
                *row[186] = result_x_limb_1_col186;
                let result_x_limb_2_col187 = result_x_tmp_d9509_79.get_m31(2);
                *row[187] = result_x_limb_2_col187;
                let result_x_limb_3_col188 = result_x_tmp_d9509_79.get_m31(3);
                *row[188] = result_x_limb_3_col188;
                let result_x_limb_4_col189 = result_x_tmp_d9509_79.get_m31(4);
                *row[189] = result_x_limb_4_col189;
                let result_x_limb_5_col190 = result_x_tmp_d9509_79.get_m31(5);
                *row[190] = result_x_limb_5_col190;
                let result_x_limb_6_col191 = result_x_tmp_d9509_79.get_m31(6);
                *row[191] = result_x_limb_6_col191;
                let result_x_limb_7_col192 = result_x_tmp_d9509_79.get_m31(7);
                *row[192] = result_x_limb_7_col192;
                let result_x_limb_8_col193 = result_x_tmp_d9509_79.get_m31(8);
                *row[193] = result_x_limb_8_col193;
                let result_x_limb_9_col194 = result_x_tmp_d9509_79.get_m31(9);
                *row[194] = result_x_limb_9_col194;
                let result_x_limb_10_col195 = result_x_tmp_d9509_79.get_m31(10);
                *row[195] = result_x_limb_10_col195;
                let result_x_limb_11_col196 = result_x_tmp_d9509_79.get_m31(11);
                *row[196] = result_x_limb_11_col196;
                let result_x_limb_12_col197 = result_x_tmp_d9509_79.get_m31(12);
                *row[197] = result_x_limb_12_col197;
                let result_x_limb_13_col198 = result_x_tmp_d9509_79.get_m31(13);
                *row[198] = result_x_limb_13_col198;
                let result_x_limb_14_col199 = result_x_tmp_d9509_79.get_m31(14);
                *row[199] = result_x_limb_14_col199;
                let result_x_limb_15_col200 = result_x_tmp_d9509_79.get_m31(15);
                *row[200] = result_x_limb_15_col200;
                let result_x_limb_16_col201 = result_x_tmp_d9509_79.get_m31(16);
                *row[201] = result_x_limb_16_col201;
                let result_x_limb_17_col202 = result_x_tmp_d9509_79.get_m31(17);
                *row[202] = result_x_limb_17_col202;
                let result_x_limb_18_col203 = result_x_tmp_d9509_79.get_m31(18);
                *row[203] = result_x_limb_18_col203;
                let result_x_limb_19_col204 = result_x_tmp_d9509_79.get_m31(19);
                *row[204] = result_x_limb_19_col204;
                let result_x_limb_20_col205 = result_x_tmp_d9509_79.get_m31(20);
                *row[205] = result_x_limb_20_col205;
                let result_x_limb_21_col206 = result_x_tmp_d9509_79.get_m31(21);
                *row[206] = result_x_limb_21_col206;
                let result_x_limb_22_col207 = result_x_tmp_d9509_79.get_m31(22);
                *row[207] = result_x_limb_22_col207;
                let result_x_limb_23_col208 = result_x_tmp_d9509_79.get_m31(23);
                *row[208] = result_x_limb_23_col208;
                let result_x_limb_24_col209 = result_x_tmp_d9509_79.get_m31(24);
                *row[209] = result_x_limb_24_col209;
                let result_x_limb_25_col210 = result_x_tmp_d9509_79.get_m31(25);
                *row[210] = result_x_limb_25_col210;
                let result_x_limb_26_col211 = result_x_tmp_d9509_79.get_m31(26);
                *row[211] = result_x_limb_26_col211;
                let result_x_limb_27_col212 = result_x_tmp_d9509_79.get_m31(27);
                *row[212] = result_x_limb_27_col212;

                // Range Check Mem Value N 28.

                *sub_component_inputs.range_check_9_9[2] =
                    [result_x_limb_0_col185, result_x_limb_1_col186];
                *lookup_data.range_check_9_9_43 = [
                    M31_517791011,
                    result_x_limb_0_col185,
                    result_x_limb_1_col186,
                ];
                *sub_component_inputs.range_check_9_9_b[2] =
                    [result_x_limb_2_col187, result_x_limb_3_col188];
                *lookup_data.range_check_9_9_b_44 = [
                    M31_1897792095,
                    result_x_limb_2_col187,
                    result_x_limb_3_col188,
                ];
                *sub_component_inputs.range_check_9_9_c[2] =
                    [result_x_limb_4_col189, result_x_limb_5_col190];
                *lookup_data.range_check_9_9_c_45 = [
                    M31_1881014476,
                    result_x_limb_4_col189,
                    result_x_limb_5_col190,
                ];
                *sub_component_inputs.range_check_9_9_d[2] =
                    [result_x_limb_6_col191, result_x_limb_7_col192];
                *lookup_data.range_check_9_9_d_46 = [
                    M31_1864236857,
                    result_x_limb_6_col191,
                    result_x_limb_7_col192,
                ];
                *sub_component_inputs.range_check_9_9_e[2] =
                    [result_x_limb_8_col193, result_x_limb_9_col194];
                *lookup_data.range_check_9_9_e_47 = [
                    M31_1847459238,
                    result_x_limb_8_col193,
                    result_x_limb_9_col194,
                ];
                *sub_component_inputs.range_check_9_9_f[2] =
                    [result_x_limb_10_col195, result_x_limb_11_col196];
                *lookup_data.range_check_9_9_f_48 = [
                    M31_1830681619,
                    result_x_limb_10_col195,
                    result_x_limb_11_col196,
                ];
                *sub_component_inputs.range_check_9_9_g[1] =
                    [result_x_limb_12_col197, result_x_limb_13_col198];
                *lookup_data.range_check_9_9_g_49 = [
                    M31_1813904000,
                    result_x_limb_12_col197,
                    result_x_limb_13_col198,
                ];
                *sub_component_inputs.range_check_9_9_h[1] =
                    [result_x_limb_14_col199, result_x_limb_15_col200];
                *lookup_data.range_check_9_9_h_50 = [
                    M31_2065568285,
                    result_x_limb_14_col199,
                    result_x_limb_15_col200,
                ];
                *sub_component_inputs.range_check_9_9[3] =
                    [result_x_limb_16_col201, result_x_limb_17_col202];
                *lookup_data.range_check_9_9_51 = [
                    M31_517791011,
                    result_x_limb_16_col201,
                    result_x_limb_17_col202,
                ];
                *sub_component_inputs.range_check_9_9_b[3] =
                    [result_x_limb_18_col203, result_x_limb_19_col204];
                *lookup_data.range_check_9_9_b_52 = [
                    M31_1897792095,
                    result_x_limb_18_col203,
                    result_x_limb_19_col204,
                ];
                *sub_component_inputs.range_check_9_9_c[3] =
                    [result_x_limb_20_col205, result_x_limb_21_col206];
                *lookup_data.range_check_9_9_c_53 = [
                    M31_1881014476,
                    result_x_limb_20_col205,
                    result_x_limb_21_col206,
                ];
                *sub_component_inputs.range_check_9_9_d[3] =
                    [result_x_limb_22_col207, result_x_limb_23_col208];
                *lookup_data.range_check_9_9_d_54 = [
                    M31_1864236857,
                    result_x_limb_22_col207,
                    result_x_limb_23_col208,
                ];
                *sub_component_inputs.range_check_9_9_e[3] =
                    [result_x_limb_24_col209, result_x_limb_25_col210];
                *lookup_data.range_check_9_9_e_55 = [
                    M31_1847459238,
                    result_x_limb_24_col209,
                    result_x_limb_25_col210,
                ];
                *sub_component_inputs.range_check_9_9_f[3] =
                    [result_x_limb_26_col211, result_x_limb_27_col212];
                *lookup_data.range_check_9_9_f_56 = [
                    M31_1830681619,
                    result_x_limb_26_col211,
                    result_x_limb_27_col212,
                ];

                let x_sum_0_tmp_d9509_80 = (((input_limb_16_col17)
                    + (pedersen_points_table_window_bits_18_output_limb_0_col73))
                    + (result_x_limb_0_col185));
                let x_sum_1_tmp_d9509_81 = (((input_limb_17_col18)
                    + (pedersen_points_table_window_bits_18_output_limb_1_col74))
                    + (result_x_limb_1_col186));
                let x_sum_2_tmp_d9509_82 = (((input_limb_18_col19)
                    + (pedersen_points_table_window_bits_18_output_limb_2_col75))
                    + (result_x_limb_2_col187));
                let x_sum_3_tmp_d9509_83 = (((input_limb_19_col20)
                    + (pedersen_points_table_window_bits_18_output_limb_3_col76))
                    + (result_x_limb_3_col188));
                let x_sum_4_tmp_d9509_84 = (((input_limb_20_col21)
                    + (pedersen_points_table_window_bits_18_output_limb_4_col77))
                    + (result_x_limb_4_col189));
                let x_sum_5_tmp_d9509_85 = (((input_limb_21_col22)
                    + (pedersen_points_table_window_bits_18_output_limb_5_col78))
                    + (result_x_limb_5_col190));
                let x_sum_6_tmp_d9509_86 = (((input_limb_22_col23)
                    + (pedersen_points_table_window_bits_18_output_limb_6_col79))
                    + (result_x_limb_6_col191));
                let x_sum_7_tmp_d9509_87 = (((input_limb_23_col24)
                    + (pedersen_points_table_window_bits_18_output_limb_7_col80))
                    + (result_x_limb_7_col192));
                let x_sum_8_tmp_d9509_88 = (((input_limb_24_col25)
                    + (pedersen_points_table_window_bits_18_output_limb_8_col81))
                    + (result_x_limb_8_col193));
                let x_sum_9_tmp_d9509_89 = (((input_limb_25_col26)
                    + (pedersen_points_table_window_bits_18_output_limb_9_col82))
                    + (result_x_limb_9_col194));
                let x_sum_10_tmp_d9509_90 = (((input_limb_26_col27)
                    + (pedersen_points_table_window_bits_18_output_limb_10_col83))
                    + (result_x_limb_10_col195));
                let x_sum_11_tmp_d9509_91 = (((input_limb_27_col28)
                    + (pedersen_points_table_window_bits_18_output_limb_11_col84))
                    + (result_x_limb_11_col196));
                let x_sum_12_tmp_d9509_92 = (((input_limb_28_col29)
                    + (pedersen_points_table_window_bits_18_output_limb_12_col85))
                    + (result_x_limb_12_col197));
                let x_sum_13_tmp_d9509_93 = (((input_limb_29_col30)
                    + (pedersen_points_table_window_bits_18_output_limb_13_col86))
                    + (result_x_limb_13_col198));
                let x_sum_14_tmp_d9509_94 = (((input_limb_30_col31)
                    + (pedersen_points_table_window_bits_18_output_limb_14_col87))
                    + (result_x_limb_14_col199));
                let x_sum_15_tmp_d9509_95 = (((input_limb_31_col32)
                    + (pedersen_points_table_window_bits_18_output_limb_15_col88))
                    + (result_x_limb_15_col200));
                let x_sum_16_tmp_d9509_96 = (((input_limb_32_col33)
                    + (pedersen_points_table_window_bits_18_output_limb_16_col89))
                    + (result_x_limb_16_col201));
                let x_sum_17_tmp_d9509_97 = (((input_limb_33_col34)
                    + (pedersen_points_table_window_bits_18_output_limb_17_col90))
                    + (result_x_limb_17_col202));
                let x_sum_18_tmp_d9509_98 = (((input_limb_34_col35)
                    + (pedersen_points_table_window_bits_18_output_limb_18_col91))
                    + (result_x_limb_18_col203));
                let x_sum_19_tmp_d9509_99 = (((input_limb_35_col36)
                    + (pedersen_points_table_window_bits_18_output_limb_19_col92))
                    + (result_x_limb_19_col204));
                let x_sum_20_tmp_d9509_100 = (((input_limb_36_col37)
                    + (pedersen_points_table_window_bits_18_output_limb_20_col93))
                    + (result_x_limb_20_col205));
                let x_sum_21_tmp_d9509_101 = (((input_limb_37_col38)
                    + (pedersen_points_table_window_bits_18_output_limb_21_col94))
                    + (result_x_limb_21_col206));
                let x_sum_22_tmp_d9509_102 = (((input_limb_38_col39)
                    + (pedersen_points_table_window_bits_18_output_limb_22_col95))
                    + (result_x_limb_22_col207));
                let x_sum_23_tmp_d9509_103 = (((input_limb_39_col40)
                    + (pedersen_points_table_window_bits_18_output_limb_23_col96))
                    + (result_x_limb_23_col208));
                let x_sum_24_tmp_d9509_104 = (((input_limb_40_col41)
                    + (pedersen_points_table_window_bits_18_output_limb_24_col97))
                    + (result_x_limb_24_col209));
                let x_sum_25_tmp_d9509_105 = (((input_limb_41_col42)
                    + (pedersen_points_table_window_bits_18_output_limb_25_col98))
                    + (result_x_limb_25_col210));
                let x_sum_26_tmp_d9509_106 = (((input_limb_42_col43)
                    + (pedersen_points_table_window_bits_18_output_limb_26_col99))
                    + (result_x_limb_26_col211));
                let x_sum_27_tmp_d9509_107 = (((input_limb_43_col44)
                    + (pedersen_points_table_window_bits_18_output_limb_27_col100))
                    + (result_x_limb_27_col212));

                // Verify Mul 252.

                // Double Karatsuba F 0 Fc 6.

                // Single Karatsuba N 7.

                let z0_tmp_d9509_108 = [
                    ((slope_limb_0_col129) * (slope_limb_0_col129)),
                    (((slope_limb_0_col129) * (slope_limb_1_col130))
                        + ((slope_limb_1_col130) * (slope_limb_0_col129))),
                    ((((slope_limb_0_col129) * (slope_limb_2_col131))
                        + ((slope_limb_1_col130) * (slope_limb_1_col130)))
                        + ((slope_limb_2_col131) * (slope_limb_0_col129))),
                    (((((slope_limb_0_col129) * (slope_limb_3_col132))
                        + ((slope_limb_1_col130) * (slope_limb_2_col131)))
                        + ((slope_limb_2_col131) * (slope_limb_1_col130)))
                        + ((slope_limb_3_col132) * (slope_limb_0_col129))),
                    ((((((slope_limb_0_col129) * (slope_limb_4_col133))
                        + ((slope_limb_1_col130) * (slope_limb_3_col132)))
                        + ((slope_limb_2_col131) * (slope_limb_2_col131)))
                        + ((slope_limb_3_col132) * (slope_limb_1_col130)))
                        + ((slope_limb_4_col133) * (slope_limb_0_col129))),
                    (((((((slope_limb_0_col129) * (slope_limb_5_col134))
                        + ((slope_limb_1_col130) * (slope_limb_4_col133)))
                        + ((slope_limb_2_col131) * (slope_limb_3_col132)))
                        + ((slope_limb_3_col132) * (slope_limb_2_col131)))
                        + ((slope_limb_4_col133) * (slope_limb_1_col130)))
                        + ((slope_limb_5_col134) * (slope_limb_0_col129))),
                    ((((((((slope_limb_0_col129) * (slope_limb_6_col135))
                        + ((slope_limb_1_col130) * (slope_limb_5_col134)))
                        + ((slope_limb_2_col131) * (slope_limb_4_col133)))
                        + ((slope_limb_3_col132) * (slope_limb_3_col132)))
                        + ((slope_limb_4_col133) * (slope_limb_2_col131)))
                        + ((slope_limb_5_col134) * (slope_limb_1_col130)))
                        + ((slope_limb_6_col135) * (slope_limb_0_col129))),
                    (((((((slope_limb_1_col130) * (slope_limb_6_col135))
                        + ((slope_limb_2_col131) * (slope_limb_5_col134)))
                        + ((slope_limb_3_col132) * (slope_limb_4_col133)))
                        + ((slope_limb_4_col133) * (slope_limb_3_col132)))
                        + ((slope_limb_5_col134) * (slope_limb_2_col131)))
                        + ((slope_limb_6_col135) * (slope_limb_1_col130))),
                    ((((((slope_limb_2_col131) * (slope_limb_6_col135))
                        + ((slope_limb_3_col132) * (slope_limb_5_col134)))
                        + ((slope_limb_4_col133) * (slope_limb_4_col133)))
                        + ((slope_limb_5_col134) * (slope_limb_3_col132)))
                        + ((slope_limb_6_col135) * (slope_limb_2_col131))),
                    (((((slope_limb_3_col132) * (slope_limb_6_col135))
                        + ((slope_limb_4_col133) * (slope_limb_5_col134)))
                        + ((slope_limb_5_col134) * (slope_limb_4_col133)))
                        + ((slope_limb_6_col135) * (slope_limb_3_col132))),
                    ((((slope_limb_4_col133) * (slope_limb_6_col135))
                        + ((slope_limb_5_col134) * (slope_limb_5_col134)))
                        + ((slope_limb_6_col135) * (slope_limb_4_col133))),
                    (((slope_limb_5_col134) * (slope_limb_6_col135))
                        + ((slope_limb_6_col135) * (slope_limb_5_col134))),
                    ((slope_limb_6_col135) * (slope_limb_6_col135)),
                ];
                let z2_tmp_d9509_109 = [
                    ((slope_limb_7_col136) * (slope_limb_7_col136)),
                    (((slope_limb_7_col136) * (slope_limb_8_col137))
                        + ((slope_limb_8_col137) * (slope_limb_7_col136))),
                    ((((slope_limb_7_col136) * (slope_limb_9_col138))
                        + ((slope_limb_8_col137) * (slope_limb_8_col137)))
                        + ((slope_limb_9_col138) * (slope_limb_7_col136))),
                    (((((slope_limb_7_col136) * (slope_limb_10_col139))
                        + ((slope_limb_8_col137) * (slope_limb_9_col138)))
                        + ((slope_limb_9_col138) * (slope_limb_8_col137)))
                        + ((slope_limb_10_col139) * (slope_limb_7_col136))),
                    ((((((slope_limb_7_col136) * (slope_limb_11_col140))
                        + ((slope_limb_8_col137) * (slope_limb_10_col139)))
                        + ((slope_limb_9_col138) * (slope_limb_9_col138)))
                        + ((slope_limb_10_col139) * (slope_limb_8_col137)))
                        + ((slope_limb_11_col140) * (slope_limb_7_col136))),
                    (((((((slope_limb_7_col136) * (slope_limb_12_col141))
                        + ((slope_limb_8_col137) * (slope_limb_11_col140)))
                        + ((slope_limb_9_col138) * (slope_limb_10_col139)))
                        + ((slope_limb_10_col139) * (slope_limb_9_col138)))
                        + ((slope_limb_11_col140) * (slope_limb_8_col137)))
                        + ((slope_limb_12_col141) * (slope_limb_7_col136))),
                    ((((((((slope_limb_7_col136) * (slope_limb_13_col142))
                        + ((slope_limb_8_col137) * (slope_limb_12_col141)))
                        + ((slope_limb_9_col138) * (slope_limb_11_col140)))
                        + ((slope_limb_10_col139) * (slope_limb_10_col139)))
                        + ((slope_limb_11_col140) * (slope_limb_9_col138)))
                        + ((slope_limb_12_col141) * (slope_limb_8_col137)))
                        + ((slope_limb_13_col142) * (slope_limb_7_col136))),
                    (((((((slope_limb_8_col137) * (slope_limb_13_col142))
                        + ((slope_limb_9_col138) * (slope_limb_12_col141)))
                        + ((slope_limb_10_col139) * (slope_limb_11_col140)))
                        + ((slope_limb_11_col140) * (slope_limb_10_col139)))
                        + ((slope_limb_12_col141) * (slope_limb_9_col138)))
                        + ((slope_limb_13_col142) * (slope_limb_8_col137))),
                    ((((((slope_limb_9_col138) * (slope_limb_13_col142))
                        + ((slope_limb_10_col139) * (slope_limb_12_col141)))
                        + ((slope_limb_11_col140) * (slope_limb_11_col140)))
                        + ((slope_limb_12_col141) * (slope_limb_10_col139)))
                        + ((slope_limb_13_col142) * (slope_limb_9_col138))),
                    (((((slope_limb_10_col139) * (slope_limb_13_col142))
                        + ((slope_limb_11_col140) * (slope_limb_12_col141)))
                        + ((slope_limb_12_col141) * (slope_limb_11_col140)))
                        + ((slope_limb_13_col142) * (slope_limb_10_col139))),
                    ((((slope_limb_11_col140) * (slope_limb_13_col142))
                        + ((slope_limb_12_col141) * (slope_limb_12_col141)))
                        + ((slope_limb_13_col142) * (slope_limb_11_col140))),
                    (((slope_limb_12_col141) * (slope_limb_13_col142))
                        + ((slope_limb_13_col142) * (slope_limb_12_col141))),
                    ((slope_limb_13_col142) * (slope_limb_13_col142)),
                ];
                let x_sum_tmp_d9509_110 = [
                    ((slope_limb_0_col129) + (slope_limb_7_col136)),
                    ((slope_limb_1_col130) + (slope_limb_8_col137)),
                    ((slope_limb_2_col131) + (slope_limb_9_col138)),
                    ((slope_limb_3_col132) + (slope_limb_10_col139)),
                    ((slope_limb_4_col133) + (slope_limb_11_col140)),
                    ((slope_limb_5_col134) + (slope_limb_12_col141)),
                    ((slope_limb_6_col135) + (slope_limb_13_col142)),
                ];
                let y_sum_tmp_d9509_111 = [
                    ((slope_limb_0_col129) + (slope_limb_7_col136)),
                    ((slope_limb_1_col130) + (slope_limb_8_col137)),
                    ((slope_limb_2_col131) + (slope_limb_9_col138)),
                    ((slope_limb_3_col132) + (slope_limb_10_col139)),
                    ((slope_limb_4_col133) + (slope_limb_11_col140)),
                    ((slope_limb_5_col134) + (slope_limb_12_col141)),
                    ((slope_limb_6_col135) + (slope_limb_13_col142)),
                ];
                let single_karatsuba_n_7_output_tmp_d9509_112 = [
                    z0_tmp_d9509_108[0],
                    z0_tmp_d9509_108[1],
                    z0_tmp_d9509_108[2],
                    z0_tmp_d9509_108[3],
                    z0_tmp_d9509_108[4],
                    z0_tmp_d9509_108[5],
                    z0_tmp_d9509_108[6],
                    ((z0_tmp_d9509_108[7])
                        + ((((x_sum_tmp_d9509_110[0]) * (y_sum_tmp_d9509_111[0]))
                            - (z0_tmp_d9509_108[0]))
                            - (z2_tmp_d9509_109[0]))),
                    ((z0_tmp_d9509_108[8])
                        + (((((x_sum_tmp_d9509_110[0]) * (y_sum_tmp_d9509_111[1]))
                            + ((x_sum_tmp_d9509_110[1]) * (y_sum_tmp_d9509_111[0])))
                            - (z0_tmp_d9509_108[1]))
                            - (z2_tmp_d9509_109[1]))),
                    ((z0_tmp_d9509_108[9])
                        + ((((((x_sum_tmp_d9509_110[0]) * (y_sum_tmp_d9509_111[2]))
                            + ((x_sum_tmp_d9509_110[1]) * (y_sum_tmp_d9509_111[1])))
                            + ((x_sum_tmp_d9509_110[2]) * (y_sum_tmp_d9509_111[0])))
                            - (z0_tmp_d9509_108[2]))
                            - (z2_tmp_d9509_109[2]))),
                    ((z0_tmp_d9509_108[10])
                        + (((((((x_sum_tmp_d9509_110[0]) * (y_sum_tmp_d9509_111[3]))
                            + ((x_sum_tmp_d9509_110[1]) * (y_sum_tmp_d9509_111[2])))
                            + ((x_sum_tmp_d9509_110[2]) * (y_sum_tmp_d9509_111[1])))
                            + ((x_sum_tmp_d9509_110[3]) * (y_sum_tmp_d9509_111[0])))
                            - (z0_tmp_d9509_108[3]))
                            - (z2_tmp_d9509_109[3]))),
                    ((z0_tmp_d9509_108[11])
                        + ((((((((x_sum_tmp_d9509_110[0]) * (y_sum_tmp_d9509_111[4]))
                            + ((x_sum_tmp_d9509_110[1]) * (y_sum_tmp_d9509_111[3])))
                            + ((x_sum_tmp_d9509_110[2]) * (y_sum_tmp_d9509_111[2])))
                            + ((x_sum_tmp_d9509_110[3]) * (y_sum_tmp_d9509_111[1])))
                            + ((x_sum_tmp_d9509_110[4]) * (y_sum_tmp_d9509_111[0])))
                            - (z0_tmp_d9509_108[4]))
                            - (z2_tmp_d9509_109[4]))),
                    ((z0_tmp_d9509_108[12])
                        + (((((((((x_sum_tmp_d9509_110[0]) * (y_sum_tmp_d9509_111[5]))
                            + ((x_sum_tmp_d9509_110[1]) * (y_sum_tmp_d9509_111[4])))
                            + ((x_sum_tmp_d9509_110[2]) * (y_sum_tmp_d9509_111[3])))
                            + ((x_sum_tmp_d9509_110[3]) * (y_sum_tmp_d9509_111[2])))
                            + ((x_sum_tmp_d9509_110[4]) * (y_sum_tmp_d9509_111[1])))
                            + ((x_sum_tmp_d9509_110[5]) * (y_sum_tmp_d9509_111[0])))
                            - (z0_tmp_d9509_108[5]))
                            - (z2_tmp_d9509_109[5]))),
                    ((((((((((x_sum_tmp_d9509_110[0]) * (y_sum_tmp_d9509_111[6]))
                        + ((x_sum_tmp_d9509_110[1]) * (y_sum_tmp_d9509_111[5])))
                        + ((x_sum_tmp_d9509_110[2]) * (y_sum_tmp_d9509_111[4])))
                        + ((x_sum_tmp_d9509_110[3]) * (y_sum_tmp_d9509_111[3])))
                        + ((x_sum_tmp_d9509_110[4]) * (y_sum_tmp_d9509_111[2])))
                        + ((x_sum_tmp_d9509_110[5]) * (y_sum_tmp_d9509_111[1])))
                        + ((x_sum_tmp_d9509_110[6]) * (y_sum_tmp_d9509_111[0])))
                        - (z0_tmp_d9509_108[6]))
                        - (z2_tmp_d9509_109[6])),
                    ((z2_tmp_d9509_109[0])
                        + (((((((((x_sum_tmp_d9509_110[1]) * (y_sum_tmp_d9509_111[6]))
                            + ((x_sum_tmp_d9509_110[2]) * (y_sum_tmp_d9509_111[5])))
                            + ((x_sum_tmp_d9509_110[3]) * (y_sum_tmp_d9509_111[4])))
                            + ((x_sum_tmp_d9509_110[4]) * (y_sum_tmp_d9509_111[3])))
                            + ((x_sum_tmp_d9509_110[5]) * (y_sum_tmp_d9509_111[2])))
                            + ((x_sum_tmp_d9509_110[6]) * (y_sum_tmp_d9509_111[1])))
                            - (z0_tmp_d9509_108[7]))
                            - (z2_tmp_d9509_109[7]))),
                    ((z2_tmp_d9509_109[1])
                        + ((((((((x_sum_tmp_d9509_110[2]) * (y_sum_tmp_d9509_111[6]))
                            + ((x_sum_tmp_d9509_110[3]) * (y_sum_tmp_d9509_111[5])))
                            + ((x_sum_tmp_d9509_110[4]) * (y_sum_tmp_d9509_111[4])))
                            + ((x_sum_tmp_d9509_110[5]) * (y_sum_tmp_d9509_111[3])))
                            + ((x_sum_tmp_d9509_110[6]) * (y_sum_tmp_d9509_111[2])))
                            - (z0_tmp_d9509_108[8]))
                            - (z2_tmp_d9509_109[8]))),
                    ((z2_tmp_d9509_109[2])
                        + (((((((x_sum_tmp_d9509_110[3]) * (y_sum_tmp_d9509_111[6]))
                            + ((x_sum_tmp_d9509_110[4]) * (y_sum_tmp_d9509_111[5])))
                            + ((x_sum_tmp_d9509_110[5]) * (y_sum_tmp_d9509_111[4])))
                            + ((x_sum_tmp_d9509_110[6]) * (y_sum_tmp_d9509_111[3])))
                            - (z0_tmp_d9509_108[9]))
                            - (z2_tmp_d9509_109[9]))),
                    ((z2_tmp_d9509_109[3])
                        + ((((((x_sum_tmp_d9509_110[4]) * (y_sum_tmp_d9509_111[6]))
                            + ((x_sum_tmp_d9509_110[5]) * (y_sum_tmp_d9509_111[5])))
                            + ((x_sum_tmp_d9509_110[6]) * (y_sum_tmp_d9509_111[4])))
                            - (z0_tmp_d9509_108[10]))
                            - (z2_tmp_d9509_109[10]))),
                    ((z2_tmp_d9509_109[4])
                        + (((((x_sum_tmp_d9509_110[5]) * (y_sum_tmp_d9509_111[6]))
                            + ((x_sum_tmp_d9509_110[6]) * (y_sum_tmp_d9509_111[5])))
                            - (z0_tmp_d9509_108[11]))
                            - (z2_tmp_d9509_109[11]))),
                    ((z2_tmp_d9509_109[5])
                        + ((((x_sum_tmp_d9509_110[6]) * (y_sum_tmp_d9509_111[6]))
                            - (z0_tmp_d9509_108[12]))
                            - (z2_tmp_d9509_109[12]))),
                    z2_tmp_d9509_109[6],
                    z2_tmp_d9509_109[7],
                    z2_tmp_d9509_109[8],
                    z2_tmp_d9509_109[9],
                    z2_tmp_d9509_109[10],
                    z2_tmp_d9509_109[11],
                    z2_tmp_d9509_109[12],
                ];

                // Single Karatsuba N 7.

                let z0_tmp_d9509_113 = [
                    ((slope_limb_14_col143) * (slope_limb_14_col143)),
                    (((slope_limb_14_col143) * (slope_limb_15_col144))
                        + ((slope_limb_15_col144) * (slope_limb_14_col143))),
                    ((((slope_limb_14_col143) * (slope_limb_16_col145))
                        + ((slope_limb_15_col144) * (slope_limb_15_col144)))
                        + ((slope_limb_16_col145) * (slope_limb_14_col143))),
                    (((((slope_limb_14_col143) * (slope_limb_17_col146))
                        + ((slope_limb_15_col144) * (slope_limb_16_col145)))
                        + ((slope_limb_16_col145) * (slope_limb_15_col144)))
                        + ((slope_limb_17_col146) * (slope_limb_14_col143))),
                    ((((((slope_limb_14_col143) * (slope_limb_18_col147))
                        + ((slope_limb_15_col144) * (slope_limb_17_col146)))
                        + ((slope_limb_16_col145) * (slope_limb_16_col145)))
                        + ((slope_limb_17_col146) * (slope_limb_15_col144)))
                        + ((slope_limb_18_col147) * (slope_limb_14_col143))),
                    (((((((slope_limb_14_col143) * (slope_limb_19_col148))
                        + ((slope_limb_15_col144) * (slope_limb_18_col147)))
                        + ((slope_limb_16_col145) * (slope_limb_17_col146)))
                        + ((slope_limb_17_col146) * (slope_limb_16_col145)))
                        + ((slope_limb_18_col147) * (slope_limb_15_col144)))
                        + ((slope_limb_19_col148) * (slope_limb_14_col143))),
                    ((((((((slope_limb_14_col143) * (slope_limb_20_col149))
                        + ((slope_limb_15_col144) * (slope_limb_19_col148)))
                        + ((slope_limb_16_col145) * (slope_limb_18_col147)))
                        + ((slope_limb_17_col146) * (slope_limb_17_col146)))
                        + ((slope_limb_18_col147) * (slope_limb_16_col145)))
                        + ((slope_limb_19_col148) * (slope_limb_15_col144)))
                        + ((slope_limb_20_col149) * (slope_limb_14_col143))),
                    (((((((slope_limb_15_col144) * (slope_limb_20_col149))
                        + ((slope_limb_16_col145) * (slope_limb_19_col148)))
                        + ((slope_limb_17_col146) * (slope_limb_18_col147)))
                        + ((slope_limb_18_col147) * (slope_limb_17_col146)))
                        + ((slope_limb_19_col148) * (slope_limb_16_col145)))
                        + ((slope_limb_20_col149) * (slope_limb_15_col144))),
                    ((((((slope_limb_16_col145) * (slope_limb_20_col149))
                        + ((slope_limb_17_col146) * (slope_limb_19_col148)))
                        + ((slope_limb_18_col147) * (slope_limb_18_col147)))
                        + ((slope_limb_19_col148) * (slope_limb_17_col146)))
                        + ((slope_limb_20_col149) * (slope_limb_16_col145))),
                    (((((slope_limb_17_col146) * (slope_limb_20_col149))
                        + ((slope_limb_18_col147) * (slope_limb_19_col148)))
                        + ((slope_limb_19_col148) * (slope_limb_18_col147)))
                        + ((slope_limb_20_col149) * (slope_limb_17_col146))),
                    ((((slope_limb_18_col147) * (slope_limb_20_col149))
                        + ((slope_limb_19_col148) * (slope_limb_19_col148)))
                        + ((slope_limb_20_col149) * (slope_limb_18_col147))),
                    (((slope_limb_19_col148) * (slope_limb_20_col149))
                        + ((slope_limb_20_col149) * (slope_limb_19_col148))),
                    ((slope_limb_20_col149) * (slope_limb_20_col149)),
                ];
                let z2_tmp_d9509_114 = [
                    ((slope_limb_21_col150) * (slope_limb_21_col150)),
                    (((slope_limb_21_col150) * (slope_limb_22_col151))
                        + ((slope_limb_22_col151) * (slope_limb_21_col150))),
                    ((((slope_limb_21_col150) * (slope_limb_23_col152))
                        + ((slope_limb_22_col151) * (slope_limb_22_col151)))
                        + ((slope_limb_23_col152) * (slope_limb_21_col150))),
                    (((((slope_limb_21_col150) * (slope_limb_24_col153))
                        + ((slope_limb_22_col151) * (slope_limb_23_col152)))
                        + ((slope_limb_23_col152) * (slope_limb_22_col151)))
                        + ((slope_limb_24_col153) * (slope_limb_21_col150))),
                    ((((((slope_limb_21_col150) * (slope_limb_25_col154))
                        + ((slope_limb_22_col151) * (slope_limb_24_col153)))
                        + ((slope_limb_23_col152) * (slope_limb_23_col152)))
                        + ((slope_limb_24_col153) * (slope_limb_22_col151)))
                        + ((slope_limb_25_col154) * (slope_limb_21_col150))),
                    (((((((slope_limb_21_col150) * (slope_limb_26_col155))
                        + ((slope_limb_22_col151) * (slope_limb_25_col154)))
                        + ((slope_limb_23_col152) * (slope_limb_24_col153)))
                        + ((slope_limb_24_col153) * (slope_limb_23_col152)))
                        + ((slope_limb_25_col154) * (slope_limb_22_col151)))
                        + ((slope_limb_26_col155) * (slope_limb_21_col150))),
                    ((((((((slope_limb_21_col150) * (slope_limb_27_col156))
                        + ((slope_limb_22_col151) * (slope_limb_26_col155)))
                        + ((slope_limb_23_col152) * (slope_limb_25_col154)))
                        + ((slope_limb_24_col153) * (slope_limb_24_col153)))
                        + ((slope_limb_25_col154) * (slope_limb_23_col152)))
                        + ((slope_limb_26_col155) * (slope_limb_22_col151)))
                        + ((slope_limb_27_col156) * (slope_limb_21_col150))),
                    (((((((slope_limb_22_col151) * (slope_limb_27_col156))
                        + ((slope_limb_23_col152) * (slope_limb_26_col155)))
                        + ((slope_limb_24_col153) * (slope_limb_25_col154)))
                        + ((slope_limb_25_col154) * (slope_limb_24_col153)))
                        + ((slope_limb_26_col155) * (slope_limb_23_col152)))
                        + ((slope_limb_27_col156) * (slope_limb_22_col151))),
                    ((((((slope_limb_23_col152) * (slope_limb_27_col156))
                        + ((slope_limb_24_col153) * (slope_limb_26_col155)))
                        + ((slope_limb_25_col154) * (slope_limb_25_col154)))
                        + ((slope_limb_26_col155) * (slope_limb_24_col153)))
                        + ((slope_limb_27_col156) * (slope_limb_23_col152))),
                    (((((slope_limb_24_col153) * (slope_limb_27_col156))
                        + ((slope_limb_25_col154) * (slope_limb_26_col155)))
                        + ((slope_limb_26_col155) * (slope_limb_25_col154)))
                        + ((slope_limb_27_col156) * (slope_limb_24_col153))),
                    ((((slope_limb_25_col154) * (slope_limb_27_col156))
                        + ((slope_limb_26_col155) * (slope_limb_26_col155)))
                        + ((slope_limb_27_col156) * (slope_limb_25_col154))),
                    (((slope_limb_26_col155) * (slope_limb_27_col156))
                        + ((slope_limb_27_col156) * (slope_limb_26_col155))),
                    ((slope_limb_27_col156) * (slope_limb_27_col156)),
                ];
                let x_sum_tmp_d9509_115 = [
                    ((slope_limb_14_col143) + (slope_limb_21_col150)),
                    ((slope_limb_15_col144) + (slope_limb_22_col151)),
                    ((slope_limb_16_col145) + (slope_limb_23_col152)),
                    ((slope_limb_17_col146) + (slope_limb_24_col153)),
                    ((slope_limb_18_col147) + (slope_limb_25_col154)),
                    ((slope_limb_19_col148) + (slope_limb_26_col155)),
                    ((slope_limb_20_col149) + (slope_limb_27_col156)),
                ];
                let y_sum_tmp_d9509_116 = [
                    ((slope_limb_14_col143) + (slope_limb_21_col150)),
                    ((slope_limb_15_col144) + (slope_limb_22_col151)),
                    ((slope_limb_16_col145) + (slope_limb_23_col152)),
                    ((slope_limb_17_col146) + (slope_limb_24_col153)),
                    ((slope_limb_18_col147) + (slope_limb_25_col154)),
                    ((slope_limb_19_col148) + (slope_limb_26_col155)),
                    ((slope_limb_20_col149) + (slope_limb_27_col156)),
                ];
                let single_karatsuba_n_7_output_tmp_d9509_117 = [
                    z0_tmp_d9509_113[0],
                    z0_tmp_d9509_113[1],
                    z0_tmp_d9509_113[2],
                    z0_tmp_d9509_113[3],
                    z0_tmp_d9509_113[4],
                    z0_tmp_d9509_113[5],
                    z0_tmp_d9509_113[6],
                    ((z0_tmp_d9509_113[7])
                        + ((((x_sum_tmp_d9509_115[0]) * (y_sum_tmp_d9509_116[0]))
                            - (z0_tmp_d9509_113[0]))
                            - (z2_tmp_d9509_114[0]))),
                    ((z0_tmp_d9509_113[8])
                        + (((((x_sum_tmp_d9509_115[0]) * (y_sum_tmp_d9509_116[1]))
                            + ((x_sum_tmp_d9509_115[1]) * (y_sum_tmp_d9509_116[0])))
                            - (z0_tmp_d9509_113[1]))
                            - (z2_tmp_d9509_114[1]))),
                    ((z0_tmp_d9509_113[9])
                        + ((((((x_sum_tmp_d9509_115[0]) * (y_sum_tmp_d9509_116[2]))
                            + ((x_sum_tmp_d9509_115[1]) * (y_sum_tmp_d9509_116[1])))
                            + ((x_sum_tmp_d9509_115[2]) * (y_sum_tmp_d9509_116[0])))
                            - (z0_tmp_d9509_113[2]))
                            - (z2_tmp_d9509_114[2]))),
                    ((z0_tmp_d9509_113[10])
                        + (((((((x_sum_tmp_d9509_115[0]) * (y_sum_tmp_d9509_116[3]))
                            + ((x_sum_tmp_d9509_115[1]) * (y_sum_tmp_d9509_116[2])))
                            + ((x_sum_tmp_d9509_115[2]) * (y_sum_tmp_d9509_116[1])))
                            + ((x_sum_tmp_d9509_115[3]) * (y_sum_tmp_d9509_116[0])))
                            - (z0_tmp_d9509_113[3]))
                            - (z2_tmp_d9509_114[3]))),
                    ((z0_tmp_d9509_113[11])
                        + ((((((((x_sum_tmp_d9509_115[0]) * (y_sum_tmp_d9509_116[4]))
                            + ((x_sum_tmp_d9509_115[1]) * (y_sum_tmp_d9509_116[3])))
                            + ((x_sum_tmp_d9509_115[2]) * (y_sum_tmp_d9509_116[2])))
                            + ((x_sum_tmp_d9509_115[3]) * (y_sum_tmp_d9509_116[1])))
                            + ((x_sum_tmp_d9509_115[4]) * (y_sum_tmp_d9509_116[0])))
                            - (z0_tmp_d9509_113[4]))
                            - (z2_tmp_d9509_114[4]))),
                    ((z0_tmp_d9509_113[12])
                        + (((((((((x_sum_tmp_d9509_115[0]) * (y_sum_tmp_d9509_116[5]))
                            + ((x_sum_tmp_d9509_115[1]) * (y_sum_tmp_d9509_116[4])))
                            + ((x_sum_tmp_d9509_115[2]) * (y_sum_tmp_d9509_116[3])))
                            + ((x_sum_tmp_d9509_115[3]) * (y_sum_tmp_d9509_116[2])))
                            + ((x_sum_tmp_d9509_115[4]) * (y_sum_tmp_d9509_116[1])))
                            + ((x_sum_tmp_d9509_115[5]) * (y_sum_tmp_d9509_116[0])))
                            - (z0_tmp_d9509_113[5]))
                            - (z2_tmp_d9509_114[5]))),
                    ((((((((((x_sum_tmp_d9509_115[0]) * (y_sum_tmp_d9509_116[6]))
                        + ((x_sum_tmp_d9509_115[1]) * (y_sum_tmp_d9509_116[5])))
                        + ((x_sum_tmp_d9509_115[2]) * (y_sum_tmp_d9509_116[4])))
                        + ((x_sum_tmp_d9509_115[3]) * (y_sum_tmp_d9509_116[3])))
                        + ((x_sum_tmp_d9509_115[4]) * (y_sum_tmp_d9509_116[2])))
                        + ((x_sum_tmp_d9509_115[5]) * (y_sum_tmp_d9509_116[1])))
                        + ((x_sum_tmp_d9509_115[6]) * (y_sum_tmp_d9509_116[0])))
                        - (z0_tmp_d9509_113[6]))
                        - (z2_tmp_d9509_114[6])),
                    ((z2_tmp_d9509_114[0])
                        + (((((((((x_sum_tmp_d9509_115[1]) * (y_sum_tmp_d9509_116[6]))
                            + ((x_sum_tmp_d9509_115[2]) * (y_sum_tmp_d9509_116[5])))
                            + ((x_sum_tmp_d9509_115[3]) * (y_sum_tmp_d9509_116[4])))
                            + ((x_sum_tmp_d9509_115[4]) * (y_sum_tmp_d9509_116[3])))
                            + ((x_sum_tmp_d9509_115[5]) * (y_sum_tmp_d9509_116[2])))
                            + ((x_sum_tmp_d9509_115[6]) * (y_sum_tmp_d9509_116[1])))
                            - (z0_tmp_d9509_113[7]))
                            - (z2_tmp_d9509_114[7]))),
                    ((z2_tmp_d9509_114[1])
                        + ((((((((x_sum_tmp_d9509_115[2]) * (y_sum_tmp_d9509_116[6]))
                            + ((x_sum_tmp_d9509_115[3]) * (y_sum_tmp_d9509_116[5])))
                            + ((x_sum_tmp_d9509_115[4]) * (y_sum_tmp_d9509_116[4])))
                            + ((x_sum_tmp_d9509_115[5]) * (y_sum_tmp_d9509_116[3])))
                            + ((x_sum_tmp_d9509_115[6]) * (y_sum_tmp_d9509_116[2])))
                            - (z0_tmp_d9509_113[8]))
                            - (z2_tmp_d9509_114[8]))),
                    ((z2_tmp_d9509_114[2])
                        + (((((((x_sum_tmp_d9509_115[3]) * (y_sum_tmp_d9509_116[6]))
                            + ((x_sum_tmp_d9509_115[4]) * (y_sum_tmp_d9509_116[5])))
                            + ((x_sum_tmp_d9509_115[5]) * (y_sum_tmp_d9509_116[4])))
                            + ((x_sum_tmp_d9509_115[6]) * (y_sum_tmp_d9509_116[3])))
                            - (z0_tmp_d9509_113[9]))
                            - (z2_tmp_d9509_114[9]))),
                    ((z2_tmp_d9509_114[3])
                        + ((((((x_sum_tmp_d9509_115[4]) * (y_sum_tmp_d9509_116[6]))
                            + ((x_sum_tmp_d9509_115[5]) * (y_sum_tmp_d9509_116[5])))
                            + ((x_sum_tmp_d9509_115[6]) * (y_sum_tmp_d9509_116[4])))
                            - (z0_tmp_d9509_113[10]))
                            - (z2_tmp_d9509_114[10]))),
                    ((z2_tmp_d9509_114[4])
                        + (((((x_sum_tmp_d9509_115[5]) * (y_sum_tmp_d9509_116[6]))
                            + ((x_sum_tmp_d9509_115[6]) * (y_sum_tmp_d9509_116[5])))
                            - (z0_tmp_d9509_113[11]))
                            - (z2_tmp_d9509_114[11]))),
                    ((z2_tmp_d9509_114[5])
                        + ((((x_sum_tmp_d9509_115[6]) * (y_sum_tmp_d9509_116[6]))
                            - (z0_tmp_d9509_113[12]))
                            - (z2_tmp_d9509_114[12]))),
                    z2_tmp_d9509_114[6],
                    z2_tmp_d9509_114[7],
                    z2_tmp_d9509_114[8],
                    z2_tmp_d9509_114[9],
                    z2_tmp_d9509_114[10],
                    z2_tmp_d9509_114[11],
                    z2_tmp_d9509_114[12],
                ];

                let x_sum_tmp_d9509_118 = [
                    ((slope_limb_0_col129) + (slope_limb_14_col143)),
                    ((slope_limb_1_col130) + (slope_limb_15_col144)),
                    ((slope_limb_2_col131) + (slope_limb_16_col145)),
                    ((slope_limb_3_col132) + (slope_limb_17_col146)),
                    ((slope_limb_4_col133) + (slope_limb_18_col147)),
                    ((slope_limb_5_col134) + (slope_limb_19_col148)),
                    ((slope_limb_6_col135) + (slope_limb_20_col149)),
                    ((slope_limb_7_col136) + (slope_limb_21_col150)),
                    ((slope_limb_8_col137) + (slope_limb_22_col151)),
                    ((slope_limb_9_col138) + (slope_limb_23_col152)),
                    ((slope_limb_10_col139) + (slope_limb_24_col153)),
                    ((slope_limb_11_col140) + (slope_limb_25_col154)),
                    ((slope_limb_12_col141) + (slope_limb_26_col155)),
                    ((slope_limb_13_col142) + (slope_limb_27_col156)),
                ];
                let y_sum_tmp_d9509_119 = [
                    ((slope_limb_0_col129) + (slope_limb_14_col143)),
                    ((slope_limb_1_col130) + (slope_limb_15_col144)),
                    ((slope_limb_2_col131) + (slope_limb_16_col145)),
                    ((slope_limb_3_col132) + (slope_limb_17_col146)),
                    ((slope_limb_4_col133) + (slope_limb_18_col147)),
                    ((slope_limb_5_col134) + (slope_limb_19_col148)),
                    ((slope_limb_6_col135) + (slope_limb_20_col149)),
                    ((slope_limb_7_col136) + (slope_limb_21_col150)),
                    ((slope_limb_8_col137) + (slope_limb_22_col151)),
                    ((slope_limb_9_col138) + (slope_limb_23_col152)),
                    ((slope_limb_10_col139) + (slope_limb_24_col153)),
                    ((slope_limb_11_col140) + (slope_limb_25_col154)),
                    ((slope_limb_12_col141) + (slope_limb_26_col155)),
                    ((slope_limb_13_col142) + (slope_limb_27_col156)),
                ];

                // Single Karatsuba N 7.

                let z0_tmp_d9509_120 = [
                    ((x_sum_tmp_d9509_118[0]) * (y_sum_tmp_d9509_119[0])),
                    (((x_sum_tmp_d9509_118[0]) * (y_sum_tmp_d9509_119[1]))
                        + ((x_sum_tmp_d9509_118[1]) * (y_sum_tmp_d9509_119[0]))),
                    ((((x_sum_tmp_d9509_118[0]) * (y_sum_tmp_d9509_119[2]))
                        + ((x_sum_tmp_d9509_118[1]) * (y_sum_tmp_d9509_119[1])))
                        + ((x_sum_tmp_d9509_118[2]) * (y_sum_tmp_d9509_119[0]))),
                    (((((x_sum_tmp_d9509_118[0]) * (y_sum_tmp_d9509_119[3]))
                        + ((x_sum_tmp_d9509_118[1]) * (y_sum_tmp_d9509_119[2])))
                        + ((x_sum_tmp_d9509_118[2]) * (y_sum_tmp_d9509_119[1])))
                        + ((x_sum_tmp_d9509_118[3]) * (y_sum_tmp_d9509_119[0]))),
                    ((((((x_sum_tmp_d9509_118[0]) * (y_sum_tmp_d9509_119[4]))
                        + ((x_sum_tmp_d9509_118[1]) * (y_sum_tmp_d9509_119[3])))
                        + ((x_sum_tmp_d9509_118[2]) * (y_sum_tmp_d9509_119[2])))
                        + ((x_sum_tmp_d9509_118[3]) * (y_sum_tmp_d9509_119[1])))
                        + ((x_sum_tmp_d9509_118[4]) * (y_sum_tmp_d9509_119[0]))),
                    (((((((x_sum_tmp_d9509_118[0]) * (y_sum_tmp_d9509_119[5]))
                        + ((x_sum_tmp_d9509_118[1]) * (y_sum_tmp_d9509_119[4])))
                        + ((x_sum_tmp_d9509_118[2]) * (y_sum_tmp_d9509_119[3])))
                        + ((x_sum_tmp_d9509_118[3]) * (y_sum_tmp_d9509_119[2])))
                        + ((x_sum_tmp_d9509_118[4]) * (y_sum_tmp_d9509_119[1])))
                        + ((x_sum_tmp_d9509_118[5]) * (y_sum_tmp_d9509_119[0]))),
                    ((((((((x_sum_tmp_d9509_118[0]) * (y_sum_tmp_d9509_119[6]))
                        + ((x_sum_tmp_d9509_118[1]) * (y_sum_tmp_d9509_119[5])))
                        + ((x_sum_tmp_d9509_118[2]) * (y_sum_tmp_d9509_119[4])))
                        + ((x_sum_tmp_d9509_118[3]) * (y_sum_tmp_d9509_119[3])))
                        + ((x_sum_tmp_d9509_118[4]) * (y_sum_tmp_d9509_119[2])))
                        + ((x_sum_tmp_d9509_118[5]) * (y_sum_tmp_d9509_119[1])))
                        + ((x_sum_tmp_d9509_118[6]) * (y_sum_tmp_d9509_119[0]))),
                    (((((((x_sum_tmp_d9509_118[1]) * (y_sum_tmp_d9509_119[6]))
                        + ((x_sum_tmp_d9509_118[2]) * (y_sum_tmp_d9509_119[5])))
                        + ((x_sum_tmp_d9509_118[3]) * (y_sum_tmp_d9509_119[4])))
                        + ((x_sum_tmp_d9509_118[4]) * (y_sum_tmp_d9509_119[3])))
                        + ((x_sum_tmp_d9509_118[5]) * (y_sum_tmp_d9509_119[2])))
                        + ((x_sum_tmp_d9509_118[6]) * (y_sum_tmp_d9509_119[1]))),
                    ((((((x_sum_tmp_d9509_118[2]) * (y_sum_tmp_d9509_119[6]))
                        + ((x_sum_tmp_d9509_118[3]) * (y_sum_tmp_d9509_119[5])))
                        + ((x_sum_tmp_d9509_118[4]) * (y_sum_tmp_d9509_119[4])))
                        + ((x_sum_tmp_d9509_118[5]) * (y_sum_tmp_d9509_119[3])))
                        + ((x_sum_tmp_d9509_118[6]) * (y_sum_tmp_d9509_119[2]))),
                    (((((x_sum_tmp_d9509_118[3]) * (y_sum_tmp_d9509_119[6]))
                        + ((x_sum_tmp_d9509_118[4]) * (y_sum_tmp_d9509_119[5])))
                        + ((x_sum_tmp_d9509_118[5]) * (y_sum_tmp_d9509_119[4])))
                        + ((x_sum_tmp_d9509_118[6]) * (y_sum_tmp_d9509_119[3]))),
                    ((((x_sum_tmp_d9509_118[4]) * (y_sum_tmp_d9509_119[6]))
                        + ((x_sum_tmp_d9509_118[5]) * (y_sum_tmp_d9509_119[5])))
                        + ((x_sum_tmp_d9509_118[6]) * (y_sum_tmp_d9509_119[4]))),
                    (((x_sum_tmp_d9509_118[5]) * (y_sum_tmp_d9509_119[6]))
                        + ((x_sum_tmp_d9509_118[6]) * (y_sum_tmp_d9509_119[5]))),
                    ((x_sum_tmp_d9509_118[6]) * (y_sum_tmp_d9509_119[6])),
                ];
                let z2_tmp_d9509_121 = [
                    ((x_sum_tmp_d9509_118[7]) * (y_sum_tmp_d9509_119[7])),
                    (((x_sum_tmp_d9509_118[7]) * (y_sum_tmp_d9509_119[8]))
                        + ((x_sum_tmp_d9509_118[8]) * (y_sum_tmp_d9509_119[7]))),
                    ((((x_sum_tmp_d9509_118[7]) * (y_sum_tmp_d9509_119[9]))
                        + ((x_sum_tmp_d9509_118[8]) * (y_sum_tmp_d9509_119[8])))
                        + ((x_sum_tmp_d9509_118[9]) * (y_sum_tmp_d9509_119[7]))),
                    (((((x_sum_tmp_d9509_118[7]) * (y_sum_tmp_d9509_119[10]))
                        + ((x_sum_tmp_d9509_118[8]) * (y_sum_tmp_d9509_119[9])))
                        + ((x_sum_tmp_d9509_118[9]) * (y_sum_tmp_d9509_119[8])))
                        + ((x_sum_tmp_d9509_118[10]) * (y_sum_tmp_d9509_119[7]))),
                    ((((((x_sum_tmp_d9509_118[7]) * (y_sum_tmp_d9509_119[11]))
                        + ((x_sum_tmp_d9509_118[8]) * (y_sum_tmp_d9509_119[10])))
                        + ((x_sum_tmp_d9509_118[9]) * (y_sum_tmp_d9509_119[9])))
                        + ((x_sum_tmp_d9509_118[10]) * (y_sum_tmp_d9509_119[8])))
                        + ((x_sum_tmp_d9509_118[11]) * (y_sum_tmp_d9509_119[7]))),
                    (((((((x_sum_tmp_d9509_118[7]) * (y_sum_tmp_d9509_119[12]))
                        + ((x_sum_tmp_d9509_118[8]) * (y_sum_tmp_d9509_119[11])))
                        + ((x_sum_tmp_d9509_118[9]) * (y_sum_tmp_d9509_119[10])))
                        + ((x_sum_tmp_d9509_118[10]) * (y_sum_tmp_d9509_119[9])))
                        + ((x_sum_tmp_d9509_118[11]) * (y_sum_tmp_d9509_119[8])))
                        + ((x_sum_tmp_d9509_118[12]) * (y_sum_tmp_d9509_119[7]))),
                    ((((((((x_sum_tmp_d9509_118[7]) * (y_sum_tmp_d9509_119[13]))
                        + ((x_sum_tmp_d9509_118[8]) * (y_sum_tmp_d9509_119[12])))
                        + ((x_sum_tmp_d9509_118[9]) * (y_sum_tmp_d9509_119[11])))
                        + ((x_sum_tmp_d9509_118[10]) * (y_sum_tmp_d9509_119[10])))
                        + ((x_sum_tmp_d9509_118[11]) * (y_sum_tmp_d9509_119[9])))
                        + ((x_sum_tmp_d9509_118[12]) * (y_sum_tmp_d9509_119[8])))
                        + ((x_sum_tmp_d9509_118[13]) * (y_sum_tmp_d9509_119[7]))),
                    (((((((x_sum_tmp_d9509_118[8]) * (y_sum_tmp_d9509_119[13]))
                        + ((x_sum_tmp_d9509_118[9]) * (y_sum_tmp_d9509_119[12])))
                        + ((x_sum_tmp_d9509_118[10]) * (y_sum_tmp_d9509_119[11])))
                        + ((x_sum_tmp_d9509_118[11]) * (y_sum_tmp_d9509_119[10])))
                        + ((x_sum_tmp_d9509_118[12]) * (y_sum_tmp_d9509_119[9])))
                        + ((x_sum_tmp_d9509_118[13]) * (y_sum_tmp_d9509_119[8]))),
                    ((((((x_sum_tmp_d9509_118[9]) * (y_sum_tmp_d9509_119[13]))
                        + ((x_sum_tmp_d9509_118[10]) * (y_sum_tmp_d9509_119[12])))
                        + ((x_sum_tmp_d9509_118[11]) * (y_sum_tmp_d9509_119[11])))
                        + ((x_sum_tmp_d9509_118[12]) * (y_sum_tmp_d9509_119[10])))
                        + ((x_sum_tmp_d9509_118[13]) * (y_sum_tmp_d9509_119[9]))),
                    (((((x_sum_tmp_d9509_118[10]) * (y_sum_tmp_d9509_119[13]))
                        + ((x_sum_tmp_d9509_118[11]) * (y_sum_tmp_d9509_119[12])))
                        + ((x_sum_tmp_d9509_118[12]) * (y_sum_tmp_d9509_119[11])))
                        + ((x_sum_tmp_d9509_118[13]) * (y_sum_tmp_d9509_119[10]))),
                    ((((x_sum_tmp_d9509_118[11]) * (y_sum_tmp_d9509_119[13]))
                        + ((x_sum_tmp_d9509_118[12]) * (y_sum_tmp_d9509_119[12])))
                        + ((x_sum_tmp_d9509_118[13]) * (y_sum_tmp_d9509_119[11]))),
                    (((x_sum_tmp_d9509_118[12]) * (y_sum_tmp_d9509_119[13]))
                        + ((x_sum_tmp_d9509_118[13]) * (y_sum_tmp_d9509_119[12]))),
                    ((x_sum_tmp_d9509_118[13]) * (y_sum_tmp_d9509_119[13])),
                ];
                let x_sum_tmp_d9509_122 = [
                    ((x_sum_tmp_d9509_118[0]) + (x_sum_tmp_d9509_118[7])),
                    ((x_sum_tmp_d9509_118[1]) + (x_sum_tmp_d9509_118[8])),
                    ((x_sum_tmp_d9509_118[2]) + (x_sum_tmp_d9509_118[9])),
                    ((x_sum_tmp_d9509_118[3]) + (x_sum_tmp_d9509_118[10])),
                    ((x_sum_tmp_d9509_118[4]) + (x_sum_tmp_d9509_118[11])),
                    ((x_sum_tmp_d9509_118[5]) + (x_sum_tmp_d9509_118[12])),
                    ((x_sum_tmp_d9509_118[6]) + (x_sum_tmp_d9509_118[13])),
                ];
                let y_sum_tmp_d9509_123 = [
                    ((y_sum_tmp_d9509_119[0]) + (y_sum_tmp_d9509_119[7])),
                    ((y_sum_tmp_d9509_119[1]) + (y_sum_tmp_d9509_119[8])),
                    ((y_sum_tmp_d9509_119[2]) + (y_sum_tmp_d9509_119[9])),
                    ((y_sum_tmp_d9509_119[3]) + (y_sum_tmp_d9509_119[10])),
                    ((y_sum_tmp_d9509_119[4]) + (y_sum_tmp_d9509_119[11])),
                    ((y_sum_tmp_d9509_119[5]) + (y_sum_tmp_d9509_119[12])),
                    ((y_sum_tmp_d9509_119[6]) + (y_sum_tmp_d9509_119[13])),
                ];
                let single_karatsuba_n_7_output_tmp_d9509_124 = [
                    z0_tmp_d9509_120[0],
                    z0_tmp_d9509_120[1],
                    z0_tmp_d9509_120[2],
                    z0_tmp_d9509_120[3],
                    z0_tmp_d9509_120[4],
                    z0_tmp_d9509_120[5],
                    z0_tmp_d9509_120[6],
                    ((z0_tmp_d9509_120[7])
                        + ((((x_sum_tmp_d9509_122[0]) * (y_sum_tmp_d9509_123[0]))
                            - (z0_tmp_d9509_120[0]))
                            - (z2_tmp_d9509_121[0]))),
                    ((z0_tmp_d9509_120[8])
                        + (((((x_sum_tmp_d9509_122[0]) * (y_sum_tmp_d9509_123[1]))
                            + ((x_sum_tmp_d9509_122[1]) * (y_sum_tmp_d9509_123[0])))
                            - (z0_tmp_d9509_120[1]))
                            - (z2_tmp_d9509_121[1]))),
                    ((z0_tmp_d9509_120[9])
                        + ((((((x_sum_tmp_d9509_122[0]) * (y_sum_tmp_d9509_123[2]))
                            + ((x_sum_tmp_d9509_122[1]) * (y_sum_tmp_d9509_123[1])))
                            + ((x_sum_tmp_d9509_122[2]) * (y_sum_tmp_d9509_123[0])))
                            - (z0_tmp_d9509_120[2]))
                            - (z2_tmp_d9509_121[2]))),
                    ((z0_tmp_d9509_120[10])
                        + (((((((x_sum_tmp_d9509_122[0]) * (y_sum_tmp_d9509_123[3]))
                            + ((x_sum_tmp_d9509_122[1]) * (y_sum_tmp_d9509_123[2])))
                            + ((x_sum_tmp_d9509_122[2]) * (y_sum_tmp_d9509_123[1])))
                            + ((x_sum_tmp_d9509_122[3]) * (y_sum_tmp_d9509_123[0])))
                            - (z0_tmp_d9509_120[3]))
                            - (z2_tmp_d9509_121[3]))),
                    ((z0_tmp_d9509_120[11])
                        + ((((((((x_sum_tmp_d9509_122[0]) * (y_sum_tmp_d9509_123[4]))
                            + ((x_sum_tmp_d9509_122[1]) * (y_sum_tmp_d9509_123[3])))
                            + ((x_sum_tmp_d9509_122[2]) * (y_sum_tmp_d9509_123[2])))
                            + ((x_sum_tmp_d9509_122[3]) * (y_sum_tmp_d9509_123[1])))
                            + ((x_sum_tmp_d9509_122[4]) * (y_sum_tmp_d9509_123[0])))
                            - (z0_tmp_d9509_120[4]))
                            - (z2_tmp_d9509_121[4]))),
                    ((z0_tmp_d9509_120[12])
                        + (((((((((x_sum_tmp_d9509_122[0]) * (y_sum_tmp_d9509_123[5]))
                            + ((x_sum_tmp_d9509_122[1]) * (y_sum_tmp_d9509_123[4])))
                            + ((x_sum_tmp_d9509_122[2]) * (y_sum_tmp_d9509_123[3])))
                            + ((x_sum_tmp_d9509_122[3]) * (y_sum_tmp_d9509_123[2])))
                            + ((x_sum_tmp_d9509_122[4]) * (y_sum_tmp_d9509_123[1])))
                            + ((x_sum_tmp_d9509_122[5]) * (y_sum_tmp_d9509_123[0])))
                            - (z0_tmp_d9509_120[5]))
                            - (z2_tmp_d9509_121[5]))),
                    ((((((((((x_sum_tmp_d9509_122[0]) * (y_sum_tmp_d9509_123[6]))
                        + ((x_sum_tmp_d9509_122[1]) * (y_sum_tmp_d9509_123[5])))
                        + ((x_sum_tmp_d9509_122[2]) * (y_sum_tmp_d9509_123[4])))
                        + ((x_sum_tmp_d9509_122[3]) * (y_sum_tmp_d9509_123[3])))
                        + ((x_sum_tmp_d9509_122[4]) * (y_sum_tmp_d9509_123[2])))
                        + ((x_sum_tmp_d9509_122[5]) * (y_sum_tmp_d9509_123[1])))
                        + ((x_sum_tmp_d9509_122[6]) * (y_sum_tmp_d9509_123[0])))
                        - (z0_tmp_d9509_120[6]))
                        - (z2_tmp_d9509_121[6])),
                    ((z2_tmp_d9509_121[0])
                        + (((((((((x_sum_tmp_d9509_122[1]) * (y_sum_tmp_d9509_123[6]))
                            + ((x_sum_tmp_d9509_122[2]) * (y_sum_tmp_d9509_123[5])))
                            + ((x_sum_tmp_d9509_122[3]) * (y_sum_tmp_d9509_123[4])))
                            + ((x_sum_tmp_d9509_122[4]) * (y_sum_tmp_d9509_123[3])))
                            + ((x_sum_tmp_d9509_122[5]) * (y_sum_tmp_d9509_123[2])))
                            + ((x_sum_tmp_d9509_122[6]) * (y_sum_tmp_d9509_123[1])))
                            - (z0_tmp_d9509_120[7]))
                            - (z2_tmp_d9509_121[7]))),
                    ((z2_tmp_d9509_121[1])
                        + ((((((((x_sum_tmp_d9509_122[2]) * (y_sum_tmp_d9509_123[6]))
                            + ((x_sum_tmp_d9509_122[3]) * (y_sum_tmp_d9509_123[5])))
                            + ((x_sum_tmp_d9509_122[4]) * (y_sum_tmp_d9509_123[4])))
                            + ((x_sum_tmp_d9509_122[5]) * (y_sum_tmp_d9509_123[3])))
                            + ((x_sum_tmp_d9509_122[6]) * (y_sum_tmp_d9509_123[2])))
                            - (z0_tmp_d9509_120[8]))
                            - (z2_tmp_d9509_121[8]))),
                    ((z2_tmp_d9509_121[2])
                        + (((((((x_sum_tmp_d9509_122[3]) * (y_sum_tmp_d9509_123[6]))
                            + ((x_sum_tmp_d9509_122[4]) * (y_sum_tmp_d9509_123[5])))
                            + ((x_sum_tmp_d9509_122[5]) * (y_sum_tmp_d9509_123[4])))
                            + ((x_sum_tmp_d9509_122[6]) * (y_sum_tmp_d9509_123[3])))
                            - (z0_tmp_d9509_120[9]))
                            - (z2_tmp_d9509_121[9]))),
                    ((z2_tmp_d9509_121[3])
                        + ((((((x_sum_tmp_d9509_122[4]) * (y_sum_tmp_d9509_123[6]))
                            + ((x_sum_tmp_d9509_122[5]) * (y_sum_tmp_d9509_123[5])))
                            + ((x_sum_tmp_d9509_122[6]) * (y_sum_tmp_d9509_123[4])))
                            - (z0_tmp_d9509_120[10]))
                            - (z2_tmp_d9509_121[10]))),
                    ((z2_tmp_d9509_121[4])
                        + (((((x_sum_tmp_d9509_122[5]) * (y_sum_tmp_d9509_123[6]))
                            + ((x_sum_tmp_d9509_122[6]) * (y_sum_tmp_d9509_123[5])))
                            - (z0_tmp_d9509_120[11]))
                            - (z2_tmp_d9509_121[11]))),
                    ((z2_tmp_d9509_121[5])
                        + ((((x_sum_tmp_d9509_122[6]) * (y_sum_tmp_d9509_123[6]))
                            - (z0_tmp_d9509_120[12]))
                            - (z2_tmp_d9509_121[12]))),
                    z2_tmp_d9509_121[6],
                    z2_tmp_d9509_121[7],
                    z2_tmp_d9509_121[8],
                    z2_tmp_d9509_121[9],
                    z2_tmp_d9509_121[10],
                    z2_tmp_d9509_121[11],
                    z2_tmp_d9509_121[12],
                ];

                let double_karatsuba_f0fc6_output_tmp_d9509_125 = [
                    single_karatsuba_n_7_output_tmp_d9509_112[0],
                    single_karatsuba_n_7_output_tmp_d9509_112[1],
                    single_karatsuba_n_7_output_tmp_d9509_112[2],
                    single_karatsuba_n_7_output_tmp_d9509_112[3],
                    single_karatsuba_n_7_output_tmp_d9509_112[4],
                    single_karatsuba_n_7_output_tmp_d9509_112[5],
                    single_karatsuba_n_7_output_tmp_d9509_112[6],
                    single_karatsuba_n_7_output_tmp_d9509_112[7],
                    single_karatsuba_n_7_output_tmp_d9509_112[8],
                    single_karatsuba_n_7_output_tmp_d9509_112[9],
                    single_karatsuba_n_7_output_tmp_d9509_112[10],
                    single_karatsuba_n_7_output_tmp_d9509_112[11],
                    single_karatsuba_n_7_output_tmp_d9509_112[12],
                    single_karatsuba_n_7_output_tmp_d9509_112[13],
                    ((single_karatsuba_n_7_output_tmp_d9509_112[14])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[0])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[0]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[0]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_112[15])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[1])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[1]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[1]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_112[16])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[2])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[2]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[2]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_112[17])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[3])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[3]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[3]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_112[18])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[4])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[4]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[4]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_112[19])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[5])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[5]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[5]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_112[20])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[6])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[6]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[6]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_112[21])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[7])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[7]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[7]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_112[22])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[8])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[8]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[8]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_112[23])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[9])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[9]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[9]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_112[24])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[10])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[10]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[10]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_112[25])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[11])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[11]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[11]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_112[26])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[12])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[12]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[12]))),
                    (((single_karatsuba_n_7_output_tmp_d9509_124[13])
                        - (single_karatsuba_n_7_output_tmp_d9509_112[13]))
                        - (single_karatsuba_n_7_output_tmp_d9509_117[13])),
                    ((single_karatsuba_n_7_output_tmp_d9509_117[0])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[14])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[14]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[14]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_117[1])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[15])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[15]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[15]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_117[2])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[16])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[16]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[16]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_117[3])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[17])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[17]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[17]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_117[4])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[18])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[18]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[18]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_117[5])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[19])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[19]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[19]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_117[6])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[20])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[20]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[20]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_117[7])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[21])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[21]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[21]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_117[8])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[22])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[22]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[22]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_117[9])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[23])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[23]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[23]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_117[10])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[24])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[24]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[24]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_117[11])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[25])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[25]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[25]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_117[12])
                        + (((single_karatsuba_n_7_output_tmp_d9509_124[26])
                            - (single_karatsuba_n_7_output_tmp_d9509_112[26]))
                            - (single_karatsuba_n_7_output_tmp_d9509_117[26]))),
                    single_karatsuba_n_7_output_tmp_d9509_117[13],
                    single_karatsuba_n_7_output_tmp_d9509_117[14],
                    single_karatsuba_n_7_output_tmp_d9509_117[15],
                    single_karatsuba_n_7_output_tmp_d9509_117[16],
                    single_karatsuba_n_7_output_tmp_d9509_117[17],
                    single_karatsuba_n_7_output_tmp_d9509_117[18],
                    single_karatsuba_n_7_output_tmp_d9509_117[19],
                    single_karatsuba_n_7_output_tmp_d9509_117[20],
                    single_karatsuba_n_7_output_tmp_d9509_117[21],
                    single_karatsuba_n_7_output_tmp_d9509_117[22],
                    single_karatsuba_n_7_output_tmp_d9509_117[23],
                    single_karatsuba_n_7_output_tmp_d9509_117[24],
                    single_karatsuba_n_7_output_tmp_d9509_117[25],
                    single_karatsuba_n_7_output_tmp_d9509_117[26],
                ];

                let conv_tmp_d9509_126 = [
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[0]) - (x_sum_0_tmp_d9509_80)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[1]) - (x_sum_1_tmp_d9509_81)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[2]) - (x_sum_2_tmp_d9509_82)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[3]) - (x_sum_3_tmp_d9509_83)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[4]) - (x_sum_4_tmp_d9509_84)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[5]) - (x_sum_5_tmp_d9509_85)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[6]) - (x_sum_6_tmp_d9509_86)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[7]) - (x_sum_7_tmp_d9509_87)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[8]) - (x_sum_8_tmp_d9509_88)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[9]) - (x_sum_9_tmp_d9509_89)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[10]) - (x_sum_10_tmp_d9509_90)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[11]) - (x_sum_11_tmp_d9509_91)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[12]) - (x_sum_12_tmp_d9509_92)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[13]) - (x_sum_13_tmp_d9509_93)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[14]) - (x_sum_14_tmp_d9509_94)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[15]) - (x_sum_15_tmp_d9509_95)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[16]) - (x_sum_16_tmp_d9509_96)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[17]) - (x_sum_17_tmp_d9509_97)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[18]) - (x_sum_18_tmp_d9509_98)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[19]) - (x_sum_19_tmp_d9509_99)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[20]) - (x_sum_20_tmp_d9509_100)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[21]) - (x_sum_21_tmp_d9509_101)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[22]) - (x_sum_22_tmp_d9509_102)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[23]) - (x_sum_23_tmp_d9509_103)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[24]) - (x_sum_24_tmp_d9509_104)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[25]) - (x_sum_25_tmp_d9509_105)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[26]) - (x_sum_26_tmp_d9509_106)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_125[27]) - (x_sum_27_tmp_d9509_107)),
                    double_karatsuba_f0fc6_output_tmp_d9509_125[28],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[29],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[30],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[31],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[32],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[33],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[34],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[35],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[36],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[37],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[38],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[39],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[40],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[41],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[42],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[43],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[44],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[45],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[46],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[47],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[48],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[49],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[50],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[51],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[52],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[53],
                    double_karatsuba_f0fc6_output_tmp_d9509_125[54],
                ];
                let conv_mod_tmp_d9509_127 = [
                    ((((M31_32) * (conv_tmp_d9509_126[0])) - ((M31_4) * (conv_tmp_d9509_126[21])))
                        + ((M31_8) * (conv_tmp_d9509_126[49]))),
                    ((((conv_tmp_d9509_126[0]) + ((M31_32) * (conv_tmp_d9509_126[1])))
                        - ((M31_4) * (conv_tmp_d9509_126[22])))
                        + ((M31_8) * (conv_tmp_d9509_126[50]))),
                    ((((conv_tmp_d9509_126[1]) + ((M31_32) * (conv_tmp_d9509_126[2])))
                        - ((M31_4) * (conv_tmp_d9509_126[23])))
                        + ((M31_8) * (conv_tmp_d9509_126[51]))),
                    ((((conv_tmp_d9509_126[2]) + ((M31_32) * (conv_tmp_d9509_126[3])))
                        - ((M31_4) * (conv_tmp_d9509_126[24])))
                        + ((M31_8) * (conv_tmp_d9509_126[52]))),
                    ((((conv_tmp_d9509_126[3]) + ((M31_32) * (conv_tmp_d9509_126[4])))
                        - ((M31_4) * (conv_tmp_d9509_126[25])))
                        + ((M31_8) * (conv_tmp_d9509_126[53]))),
                    ((((conv_tmp_d9509_126[4]) + ((M31_32) * (conv_tmp_d9509_126[5])))
                        - ((M31_4) * (conv_tmp_d9509_126[26])))
                        + ((M31_8) * (conv_tmp_d9509_126[54]))),
                    (((conv_tmp_d9509_126[5]) + ((M31_32) * (conv_tmp_d9509_126[6])))
                        - ((M31_4) * (conv_tmp_d9509_126[27]))),
                    (((((M31_2) * (conv_tmp_d9509_126[0])) + (conv_tmp_d9509_126[6]))
                        + ((M31_32) * (conv_tmp_d9509_126[7])))
                        - ((M31_4) * (conv_tmp_d9509_126[28]))),
                    (((((M31_2) * (conv_tmp_d9509_126[1])) + (conv_tmp_d9509_126[7]))
                        + ((M31_32) * (conv_tmp_d9509_126[8])))
                        - ((M31_4) * (conv_tmp_d9509_126[29]))),
                    (((((M31_2) * (conv_tmp_d9509_126[2])) + (conv_tmp_d9509_126[8]))
                        + ((M31_32) * (conv_tmp_d9509_126[9])))
                        - ((M31_4) * (conv_tmp_d9509_126[30]))),
                    (((((M31_2) * (conv_tmp_d9509_126[3])) + (conv_tmp_d9509_126[9]))
                        + ((M31_32) * (conv_tmp_d9509_126[10])))
                        - ((M31_4) * (conv_tmp_d9509_126[31]))),
                    (((((M31_2) * (conv_tmp_d9509_126[4])) + (conv_tmp_d9509_126[10]))
                        + ((M31_32) * (conv_tmp_d9509_126[11])))
                        - ((M31_4) * (conv_tmp_d9509_126[32]))),
                    (((((M31_2) * (conv_tmp_d9509_126[5])) + (conv_tmp_d9509_126[11]))
                        + ((M31_32) * (conv_tmp_d9509_126[12])))
                        - ((M31_4) * (conv_tmp_d9509_126[33]))),
                    (((((M31_2) * (conv_tmp_d9509_126[6])) + (conv_tmp_d9509_126[12]))
                        + ((M31_32) * (conv_tmp_d9509_126[13])))
                        - ((M31_4) * (conv_tmp_d9509_126[34]))),
                    (((((M31_2) * (conv_tmp_d9509_126[7])) + (conv_tmp_d9509_126[13]))
                        + ((M31_32) * (conv_tmp_d9509_126[14])))
                        - ((M31_4) * (conv_tmp_d9509_126[35]))),
                    (((((M31_2) * (conv_tmp_d9509_126[8])) + (conv_tmp_d9509_126[14]))
                        + ((M31_32) * (conv_tmp_d9509_126[15])))
                        - ((M31_4) * (conv_tmp_d9509_126[36]))),
                    (((((M31_2) * (conv_tmp_d9509_126[9])) + (conv_tmp_d9509_126[15]))
                        + ((M31_32) * (conv_tmp_d9509_126[16])))
                        - ((M31_4) * (conv_tmp_d9509_126[37]))),
                    (((((M31_2) * (conv_tmp_d9509_126[10])) + (conv_tmp_d9509_126[16]))
                        + ((M31_32) * (conv_tmp_d9509_126[17])))
                        - ((M31_4) * (conv_tmp_d9509_126[38]))),
                    (((((M31_2) * (conv_tmp_d9509_126[11])) + (conv_tmp_d9509_126[17]))
                        + ((M31_32) * (conv_tmp_d9509_126[18])))
                        - ((M31_4) * (conv_tmp_d9509_126[39]))),
                    (((((M31_2) * (conv_tmp_d9509_126[12])) + (conv_tmp_d9509_126[18]))
                        + ((M31_32) * (conv_tmp_d9509_126[19])))
                        - ((M31_4) * (conv_tmp_d9509_126[40]))),
                    (((((M31_2) * (conv_tmp_d9509_126[13])) + (conv_tmp_d9509_126[19]))
                        + ((M31_32) * (conv_tmp_d9509_126[20])))
                        - ((M31_4) * (conv_tmp_d9509_126[41]))),
                    (((((M31_2) * (conv_tmp_d9509_126[14])) + (conv_tmp_d9509_126[20]))
                        - ((M31_4) * (conv_tmp_d9509_126[42])))
                        + ((M31_64) * (conv_tmp_d9509_126[49]))),
                    (((((M31_2) * (conv_tmp_d9509_126[15]))
                        - ((M31_4) * (conv_tmp_d9509_126[43])))
                        + ((M31_2) * (conv_tmp_d9509_126[49])))
                        + ((M31_64) * (conv_tmp_d9509_126[50]))),
                    (((((M31_2) * (conv_tmp_d9509_126[16]))
                        - ((M31_4) * (conv_tmp_d9509_126[44])))
                        + ((M31_2) * (conv_tmp_d9509_126[50])))
                        + ((M31_64) * (conv_tmp_d9509_126[51]))),
                    (((((M31_2) * (conv_tmp_d9509_126[17]))
                        - ((M31_4) * (conv_tmp_d9509_126[45])))
                        + ((M31_2) * (conv_tmp_d9509_126[51])))
                        + ((M31_64) * (conv_tmp_d9509_126[52]))),
                    (((((M31_2) * (conv_tmp_d9509_126[18]))
                        - ((M31_4) * (conv_tmp_d9509_126[46])))
                        + ((M31_2) * (conv_tmp_d9509_126[52])))
                        + ((M31_64) * (conv_tmp_d9509_126[53]))),
                    (((((M31_2) * (conv_tmp_d9509_126[19]))
                        - ((M31_4) * (conv_tmp_d9509_126[47])))
                        + ((M31_2) * (conv_tmp_d9509_126[53])))
                        + ((M31_64) * (conv_tmp_d9509_126[54]))),
                    ((((M31_2) * (conv_tmp_d9509_126[20])) - ((M31_4) * (conv_tmp_d9509_126[48])))
                        + ((M31_2) * (conv_tmp_d9509_126[54]))),
                ];
                let k_mod_2_18_biased_tmp_d9509_128 =
                    ((((PackedUInt32::from_m31(((conv_mod_tmp_d9509_127[0]) + (M31_134217728))))
                        + (((PackedUInt32::from_m31(
                            ((conv_mod_tmp_d9509_127[1]) + (M31_134217728)),
                        )) & (UInt32_511))
                            << (UInt32_9)))
                        + (UInt32_131072))
                        & (UInt32_262143));
                let k_col213 = ((k_mod_2_18_biased_tmp_d9509_128.low().as_m31())
                    + (((k_mod_2_18_biased_tmp_d9509_128.high().as_m31()) - (M31_2))
                        * (M31_65536)));
                *row[213] = k_col213;
                *sub_component_inputs.range_check_20[4] = [((k_col213) + (M31_524288))];
                *lookup_data.range_check_20_57 = [M31_1410849886, ((k_col213) + (M31_524288))];
                let carry_0_col214 = (((conv_mod_tmp_d9509_127[0]) - (k_col213)) * (M31_4194304));
                *row[214] = carry_0_col214;
                *sub_component_inputs.range_check_20_b[4] = [((carry_0_col214) + (M31_524288))];
                *lookup_data.range_check_20_b_58 =
                    [M31_514232941, ((carry_0_col214) + (M31_524288))];
                let carry_1_col215 =
                    (((conv_mod_tmp_d9509_127[1]) + (carry_0_col214)) * (M31_4194304));
                *row[215] = carry_1_col215;
                *sub_component_inputs.range_check_20_c[4] = [((carry_1_col215) + (M31_524288))];
                *lookup_data.range_check_20_c_59 =
                    [M31_531010560, ((carry_1_col215) + (M31_524288))];
                let carry_2_col216 =
                    (((conv_mod_tmp_d9509_127[2]) + (carry_1_col215)) * (M31_4194304));
                *row[216] = carry_2_col216;
                *sub_component_inputs.range_check_20_d[4] = [((carry_2_col216) + (M31_524288))];
                *lookup_data.range_check_20_d_60 =
                    [M31_480677703, ((carry_2_col216) + (M31_524288))];
                let carry_3_col217 =
                    (((conv_mod_tmp_d9509_127[3]) + (carry_2_col216)) * (M31_4194304));
                *row[217] = carry_3_col217;
                *sub_component_inputs.range_check_20_e[3] = [((carry_3_col217) + (M31_524288))];
                *lookup_data.range_check_20_e_61 =
                    [M31_497455322, ((carry_3_col217) + (M31_524288))];
                let carry_4_col218 =
                    (((conv_mod_tmp_d9509_127[4]) + (carry_3_col217)) * (M31_4194304));
                *row[218] = carry_4_col218;
                *sub_component_inputs.range_check_20_f[3] = [((carry_4_col218) + (M31_524288))];
                *lookup_data.range_check_20_f_62 =
                    [M31_447122465, ((carry_4_col218) + (M31_524288))];
                let carry_5_col219 =
                    (((conv_mod_tmp_d9509_127[5]) + (carry_4_col218)) * (M31_4194304));
                *row[219] = carry_5_col219;
                *sub_component_inputs.range_check_20_g[3] = [((carry_5_col219) + (M31_524288))];
                *lookup_data.range_check_20_g_63 =
                    [M31_463900084, ((carry_5_col219) + (M31_524288))];
                let carry_6_col220 =
                    (((conv_mod_tmp_d9509_127[6]) + (carry_5_col219)) * (M31_4194304));
                *row[220] = carry_6_col220;
                *sub_component_inputs.range_check_20_h[3] = [((carry_6_col220) + (M31_524288))];
                *lookup_data.range_check_20_h_64 =
                    [M31_682009131, ((carry_6_col220) + (M31_524288))];
                let carry_7_col221 =
                    (((conv_mod_tmp_d9509_127[7]) + (carry_6_col220)) * (M31_4194304));
                *row[221] = carry_7_col221;
                *sub_component_inputs.range_check_20[5] = [((carry_7_col221) + (M31_524288))];
                *lookup_data.range_check_20_65 =
                    [M31_1410849886, ((carry_7_col221) + (M31_524288))];
                let carry_8_col222 =
                    (((conv_mod_tmp_d9509_127[8]) + (carry_7_col221)) * (M31_4194304));
                *row[222] = carry_8_col222;
                *sub_component_inputs.range_check_20_b[5] = [((carry_8_col222) + (M31_524288))];
                *lookup_data.range_check_20_b_66 =
                    [M31_514232941, ((carry_8_col222) + (M31_524288))];
                let carry_9_col223 =
                    (((conv_mod_tmp_d9509_127[9]) + (carry_8_col222)) * (M31_4194304));
                *row[223] = carry_9_col223;
                *sub_component_inputs.range_check_20_c[5] = [((carry_9_col223) + (M31_524288))];
                *lookup_data.range_check_20_c_67 =
                    [M31_531010560, ((carry_9_col223) + (M31_524288))];
                let carry_10_col224 =
                    (((conv_mod_tmp_d9509_127[10]) + (carry_9_col223)) * (M31_4194304));
                *row[224] = carry_10_col224;
                *sub_component_inputs.range_check_20_d[5] = [((carry_10_col224) + (M31_524288))];
                *lookup_data.range_check_20_d_68 =
                    [M31_480677703, ((carry_10_col224) + (M31_524288))];
                let carry_11_col225 =
                    (((conv_mod_tmp_d9509_127[11]) + (carry_10_col224)) * (M31_4194304));
                *row[225] = carry_11_col225;
                *sub_component_inputs.range_check_20_e[4] = [((carry_11_col225) + (M31_524288))];
                *lookup_data.range_check_20_e_69 =
                    [M31_497455322, ((carry_11_col225) + (M31_524288))];
                let carry_12_col226 =
                    (((conv_mod_tmp_d9509_127[12]) + (carry_11_col225)) * (M31_4194304));
                *row[226] = carry_12_col226;
                *sub_component_inputs.range_check_20_f[4] = [((carry_12_col226) + (M31_524288))];
                *lookup_data.range_check_20_f_70 =
                    [M31_447122465, ((carry_12_col226) + (M31_524288))];
                let carry_13_col227 =
                    (((conv_mod_tmp_d9509_127[13]) + (carry_12_col226)) * (M31_4194304));
                *row[227] = carry_13_col227;
                *sub_component_inputs.range_check_20_g[4] = [((carry_13_col227) + (M31_524288))];
                *lookup_data.range_check_20_g_71 =
                    [M31_463900084, ((carry_13_col227) + (M31_524288))];
                let carry_14_col228 =
                    (((conv_mod_tmp_d9509_127[14]) + (carry_13_col227)) * (M31_4194304));
                *row[228] = carry_14_col228;
                *sub_component_inputs.range_check_20_h[4] = [((carry_14_col228) + (M31_524288))];
                *lookup_data.range_check_20_h_72 =
                    [M31_682009131, ((carry_14_col228) + (M31_524288))];
                let carry_15_col229 =
                    (((conv_mod_tmp_d9509_127[15]) + (carry_14_col228)) * (M31_4194304));
                *row[229] = carry_15_col229;
                *sub_component_inputs.range_check_20[6] = [((carry_15_col229) + (M31_524288))];
                *lookup_data.range_check_20_73 =
                    [M31_1410849886, ((carry_15_col229) + (M31_524288))];
                let carry_16_col230 =
                    (((conv_mod_tmp_d9509_127[16]) + (carry_15_col229)) * (M31_4194304));
                *row[230] = carry_16_col230;
                *sub_component_inputs.range_check_20_b[6] = [((carry_16_col230) + (M31_524288))];
                *lookup_data.range_check_20_b_74 =
                    [M31_514232941, ((carry_16_col230) + (M31_524288))];
                let carry_17_col231 =
                    (((conv_mod_tmp_d9509_127[17]) + (carry_16_col230)) * (M31_4194304));
                *row[231] = carry_17_col231;
                *sub_component_inputs.range_check_20_c[6] = [((carry_17_col231) + (M31_524288))];
                *lookup_data.range_check_20_c_75 =
                    [M31_531010560, ((carry_17_col231) + (M31_524288))];
                let carry_18_col232 =
                    (((conv_mod_tmp_d9509_127[18]) + (carry_17_col231)) * (M31_4194304));
                *row[232] = carry_18_col232;
                *sub_component_inputs.range_check_20_d[6] = [((carry_18_col232) + (M31_524288))];
                *lookup_data.range_check_20_d_76 =
                    [M31_480677703, ((carry_18_col232) + (M31_524288))];
                let carry_19_col233 =
                    (((conv_mod_tmp_d9509_127[19]) + (carry_18_col232)) * (M31_4194304));
                *row[233] = carry_19_col233;
                *sub_component_inputs.range_check_20_e[5] = [((carry_19_col233) + (M31_524288))];
                *lookup_data.range_check_20_e_77 =
                    [M31_497455322, ((carry_19_col233) + (M31_524288))];
                let carry_20_col234 =
                    (((conv_mod_tmp_d9509_127[20]) + (carry_19_col233)) * (M31_4194304));
                *row[234] = carry_20_col234;
                *sub_component_inputs.range_check_20_f[5] = [((carry_20_col234) + (M31_524288))];
                *lookup_data.range_check_20_f_78 =
                    [M31_447122465, ((carry_20_col234) + (M31_524288))];
                let carry_21_col235 = ((((conv_mod_tmp_d9509_127[21]) - ((M31_136) * (k_col213)))
                    + (carry_20_col234))
                    * (M31_4194304));
                *row[235] = carry_21_col235;
                *sub_component_inputs.range_check_20_g[5] = [((carry_21_col235) + (M31_524288))];
                *lookup_data.range_check_20_g_79 =
                    [M31_463900084, ((carry_21_col235) + (M31_524288))];
                let carry_22_col236 =
                    (((conv_mod_tmp_d9509_127[22]) + (carry_21_col235)) * (M31_4194304));
                *row[236] = carry_22_col236;
                *sub_component_inputs.range_check_20_h[5] = [((carry_22_col236) + (M31_524288))];
                *lookup_data.range_check_20_h_80 =
                    [M31_682009131, ((carry_22_col236) + (M31_524288))];
                let carry_23_col237 =
                    (((conv_mod_tmp_d9509_127[23]) + (carry_22_col236)) * (M31_4194304));
                *row[237] = carry_23_col237;
                *sub_component_inputs.range_check_20[7] = [((carry_23_col237) + (M31_524288))];
                *lookup_data.range_check_20_81 =
                    [M31_1410849886, ((carry_23_col237) + (M31_524288))];
                let carry_24_col238 =
                    (((conv_mod_tmp_d9509_127[24]) + (carry_23_col237)) * (M31_4194304));
                *row[238] = carry_24_col238;
                *sub_component_inputs.range_check_20_b[7] = [((carry_24_col238) + (M31_524288))];
                *lookup_data.range_check_20_b_82 =
                    [M31_514232941, ((carry_24_col238) + (M31_524288))];
                let carry_25_col239 =
                    (((conv_mod_tmp_d9509_127[25]) + (carry_24_col238)) * (M31_4194304));
                *row[239] = carry_25_col239;
                *sub_component_inputs.range_check_20_c[7] = [((carry_25_col239) + (M31_524288))];
                *lookup_data.range_check_20_c_83 =
                    [M31_531010560, ((carry_25_col239) + (M31_524288))];
                let carry_26_col240 =
                    (((conv_mod_tmp_d9509_127[26]) + (carry_25_col239)) * (M31_4194304));
                *row[240] = carry_26_col240;
                *sub_component_inputs.range_check_20_d[7] = [((carry_26_col240) + (M31_524288))];
                *lookup_data.range_check_20_d_84 =
                    [M31_480677703, ((carry_26_col240) + (M31_524288))];

                let result_y_tmp_d9509_129 = (((slope_tmp_d9509_1)
                    * ((partial_ec_mul_window_bits_18_input.2 .1[0]) - (result_x_tmp_d9509_79)))
                    - (partial_ec_mul_window_bits_18_input.2 .1[1]));
                let result_y_limb_0_col241 = result_y_tmp_d9509_129.get_m31(0);
                *row[241] = result_y_limb_0_col241;
                let result_y_limb_1_col242 = result_y_tmp_d9509_129.get_m31(1);
                *row[242] = result_y_limb_1_col242;
                let result_y_limb_2_col243 = result_y_tmp_d9509_129.get_m31(2);
                *row[243] = result_y_limb_2_col243;
                let result_y_limb_3_col244 = result_y_tmp_d9509_129.get_m31(3);
                *row[244] = result_y_limb_3_col244;
                let result_y_limb_4_col245 = result_y_tmp_d9509_129.get_m31(4);
                *row[245] = result_y_limb_4_col245;
                let result_y_limb_5_col246 = result_y_tmp_d9509_129.get_m31(5);
                *row[246] = result_y_limb_5_col246;
                let result_y_limb_6_col247 = result_y_tmp_d9509_129.get_m31(6);
                *row[247] = result_y_limb_6_col247;
                let result_y_limb_7_col248 = result_y_tmp_d9509_129.get_m31(7);
                *row[248] = result_y_limb_7_col248;
                let result_y_limb_8_col249 = result_y_tmp_d9509_129.get_m31(8);
                *row[249] = result_y_limb_8_col249;
                let result_y_limb_9_col250 = result_y_tmp_d9509_129.get_m31(9);
                *row[250] = result_y_limb_9_col250;
                let result_y_limb_10_col251 = result_y_tmp_d9509_129.get_m31(10);
                *row[251] = result_y_limb_10_col251;
                let result_y_limb_11_col252 = result_y_tmp_d9509_129.get_m31(11);
                *row[252] = result_y_limb_11_col252;
                let result_y_limb_12_col253 = result_y_tmp_d9509_129.get_m31(12);
                *row[253] = result_y_limb_12_col253;
                let result_y_limb_13_col254 = result_y_tmp_d9509_129.get_m31(13);
                *row[254] = result_y_limb_13_col254;
                let result_y_limb_14_col255 = result_y_tmp_d9509_129.get_m31(14);
                *row[255] = result_y_limb_14_col255;
                let result_y_limb_15_col256 = result_y_tmp_d9509_129.get_m31(15);
                *row[256] = result_y_limb_15_col256;
                let result_y_limb_16_col257 = result_y_tmp_d9509_129.get_m31(16);
                *row[257] = result_y_limb_16_col257;
                let result_y_limb_17_col258 = result_y_tmp_d9509_129.get_m31(17);
                *row[258] = result_y_limb_17_col258;
                let result_y_limb_18_col259 = result_y_tmp_d9509_129.get_m31(18);
                *row[259] = result_y_limb_18_col259;
                let result_y_limb_19_col260 = result_y_tmp_d9509_129.get_m31(19);
                *row[260] = result_y_limb_19_col260;
                let result_y_limb_20_col261 = result_y_tmp_d9509_129.get_m31(20);
                *row[261] = result_y_limb_20_col261;
                let result_y_limb_21_col262 = result_y_tmp_d9509_129.get_m31(21);
                *row[262] = result_y_limb_21_col262;
                let result_y_limb_22_col263 = result_y_tmp_d9509_129.get_m31(22);
                *row[263] = result_y_limb_22_col263;
                let result_y_limb_23_col264 = result_y_tmp_d9509_129.get_m31(23);
                *row[264] = result_y_limb_23_col264;
                let result_y_limb_24_col265 = result_y_tmp_d9509_129.get_m31(24);
                *row[265] = result_y_limb_24_col265;
                let result_y_limb_25_col266 = result_y_tmp_d9509_129.get_m31(25);
                *row[266] = result_y_limb_25_col266;
                let result_y_limb_26_col267 = result_y_tmp_d9509_129.get_m31(26);
                *row[267] = result_y_limb_26_col267;
                let result_y_limb_27_col268 = result_y_tmp_d9509_129.get_m31(27);
                *row[268] = result_y_limb_27_col268;

                // Range Check Mem Value N 28.

                *sub_component_inputs.range_check_9_9[4] =
                    [result_y_limb_0_col241, result_y_limb_1_col242];
                *lookup_data.range_check_9_9_85 = [
                    M31_517791011,
                    result_y_limb_0_col241,
                    result_y_limb_1_col242,
                ];
                *sub_component_inputs.range_check_9_9_b[4] =
                    [result_y_limb_2_col243, result_y_limb_3_col244];
                *lookup_data.range_check_9_9_b_86 = [
                    M31_1897792095,
                    result_y_limb_2_col243,
                    result_y_limb_3_col244,
                ];
                *sub_component_inputs.range_check_9_9_c[4] =
                    [result_y_limb_4_col245, result_y_limb_5_col246];
                *lookup_data.range_check_9_9_c_87 = [
                    M31_1881014476,
                    result_y_limb_4_col245,
                    result_y_limb_5_col246,
                ];
                *sub_component_inputs.range_check_9_9_d[4] =
                    [result_y_limb_6_col247, result_y_limb_7_col248];
                *lookup_data.range_check_9_9_d_88 = [
                    M31_1864236857,
                    result_y_limb_6_col247,
                    result_y_limb_7_col248,
                ];
                *sub_component_inputs.range_check_9_9_e[4] =
                    [result_y_limb_8_col249, result_y_limb_9_col250];
                *lookup_data.range_check_9_9_e_89 = [
                    M31_1847459238,
                    result_y_limb_8_col249,
                    result_y_limb_9_col250,
                ];
                *sub_component_inputs.range_check_9_9_f[4] =
                    [result_y_limb_10_col251, result_y_limb_11_col252];
                *lookup_data.range_check_9_9_f_90 = [
                    M31_1830681619,
                    result_y_limb_10_col251,
                    result_y_limb_11_col252,
                ];
                *sub_component_inputs.range_check_9_9_g[2] =
                    [result_y_limb_12_col253, result_y_limb_13_col254];
                *lookup_data.range_check_9_9_g_91 = [
                    M31_1813904000,
                    result_y_limb_12_col253,
                    result_y_limb_13_col254,
                ];
                *sub_component_inputs.range_check_9_9_h[2] =
                    [result_y_limb_14_col255, result_y_limb_15_col256];
                *lookup_data.range_check_9_9_h_92 = [
                    M31_2065568285,
                    result_y_limb_14_col255,
                    result_y_limb_15_col256,
                ];
                *sub_component_inputs.range_check_9_9[5] =
                    [result_y_limb_16_col257, result_y_limb_17_col258];
                *lookup_data.range_check_9_9_93 = [
                    M31_517791011,
                    result_y_limb_16_col257,
                    result_y_limb_17_col258,
                ];
                *sub_component_inputs.range_check_9_9_b[5] =
                    [result_y_limb_18_col259, result_y_limb_19_col260];
                *lookup_data.range_check_9_9_b_94 = [
                    M31_1897792095,
                    result_y_limb_18_col259,
                    result_y_limb_19_col260,
                ];
                *sub_component_inputs.range_check_9_9_c[5] =
                    [result_y_limb_20_col261, result_y_limb_21_col262];
                *lookup_data.range_check_9_9_c_95 = [
                    M31_1881014476,
                    result_y_limb_20_col261,
                    result_y_limb_21_col262,
                ];
                *sub_component_inputs.range_check_9_9_d[5] =
                    [result_y_limb_22_col263, result_y_limb_23_col264];
                *lookup_data.range_check_9_9_d_96 = [
                    M31_1864236857,
                    result_y_limb_22_col263,
                    result_y_limb_23_col264,
                ];
                *sub_component_inputs.range_check_9_9_e[5] =
                    [result_y_limb_24_col265, result_y_limb_25_col266];
                *lookup_data.range_check_9_9_e_97 = [
                    M31_1847459238,
                    result_y_limb_24_col265,
                    result_y_limb_25_col266,
                ];
                *sub_component_inputs.range_check_9_9_f[5] =
                    [result_y_limb_26_col267, result_y_limb_27_col268];
                *lookup_data.range_check_9_9_f_98 = [
                    M31_1830681619,
                    result_y_limb_26_col267,
                    result_y_limb_27_col268,
                ];

                let x_diff2_0_tmp_d9509_130 = ((input_limb_16_col17) - (result_x_limb_0_col185));
                let x_diff2_1_tmp_d9509_131 = ((input_limb_17_col18) - (result_x_limb_1_col186));
                let x_diff2_2_tmp_d9509_132 = ((input_limb_18_col19) - (result_x_limb_2_col187));
                let x_diff2_3_tmp_d9509_133 = ((input_limb_19_col20) - (result_x_limb_3_col188));
                let x_diff2_4_tmp_d9509_134 = ((input_limb_20_col21) - (result_x_limb_4_col189));
                let x_diff2_5_tmp_d9509_135 = ((input_limb_21_col22) - (result_x_limb_5_col190));
                let x_diff2_6_tmp_d9509_136 = ((input_limb_22_col23) - (result_x_limb_6_col191));
                let x_diff2_7_tmp_d9509_137 = ((input_limb_23_col24) - (result_x_limb_7_col192));
                let x_diff2_8_tmp_d9509_138 = ((input_limb_24_col25) - (result_x_limb_8_col193));
                let x_diff2_9_tmp_d9509_139 = ((input_limb_25_col26) - (result_x_limb_9_col194));
                let x_diff2_10_tmp_d9509_140 = ((input_limb_26_col27) - (result_x_limb_10_col195));
                let x_diff2_11_tmp_d9509_141 = ((input_limb_27_col28) - (result_x_limb_11_col196));
                let x_diff2_12_tmp_d9509_142 = ((input_limb_28_col29) - (result_x_limb_12_col197));
                let x_diff2_13_tmp_d9509_143 = ((input_limb_29_col30) - (result_x_limb_13_col198));
                let x_diff2_14_tmp_d9509_144 = ((input_limb_30_col31) - (result_x_limb_14_col199));
                let x_diff2_15_tmp_d9509_145 = ((input_limb_31_col32) - (result_x_limb_15_col200));
                let x_diff2_16_tmp_d9509_146 = ((input_limb_32_col33) - (result_x_limb_16_col201));
                let x_diff2_17_tmp_d9509_147 = ((input_limb_33_col34) - (result_x_limb_17_col202));
                let x_diff2_18_tmp_d9509_148 = ((input_limb_34_col35) - (result_x_limb_18_col203));
                let x_diff2_19_tmp_d9509_149 = ((input_limb_35_col36) - (result_x_limb_19_col204));
                let x_diff2_20_tmp_d9509_150 = ((input_limb_36_col37) - (result_x_limb_20_col205));
                let x_diff2_21_tmp_d9509_151 = ((input_limb_37_col38) - (result_x_limb_21_col206));
                let x_diff2_22_tmp_d9509_152 = ((input_limb_38_col39) - (result_x_limb_22_col207));
                let x_diff2_23_tmp_d9509_153 = ((input_limb_39_col40) - (result_x_limb_23_col208));
                let x_diff2_24_tmp_d9509_154 = ((input_limb_40_col41) - (result_x_limb_24_col209));
                let x_diff2_25_tmp_d9509_155 = ((input_limb_41_col42) - (result_x_limb_25_col210));
                let x_diff2_26_tmp_d9509_156 = ((input_limb_42_col43) - (result_x_limb_26_col211));
                let x_diff2_27_tmp_d9509_157 = ((input_limb_43_col44) - (result_x_limb_27_col212));
                let y_sum_0_tmp_d9509_158 = ((input_limb_44_col45) + (result_y_limb_0_col241));
                let y_sum_1_tmp_d9509_159 = ((input_limb_45_col46) + (result_y_limb_1_col242));
                let y_sum_2_tmp_d9509_160 = ((input_limb_46_col47) + (result_y_limb_2_col243));
                let y_sum_3_tmp_d9509_161 = ((input_limb_47_col48) + (result_y_limb_3_col244));
                let y_sum_4_tmp_d9509_162 = ((input_limb_48_col49) + (result_y_limb_4_col245));
                let y_sum_5_tmp_d9509_163 = ((input_limb_49_col50) + (result_y_limb_5_col246));
                let y_sum_6_tmp_d9509_164 = ((input_limb_50_col51) + (result_y_limb_6_col247));
                let y_sum_7_tmp_d9509_165 = ((input_limb_51_col52) + (result_y_limb_7_col248));
                let y_sum_8_tmp_d9509_166 = ((input_limb_52_col53) + (result_y_limb_8_col249));
                let y_sum_9_tmp_d9509_167 = ((input_limb_53_col54) + (result_y_limb_9_col250));
                let y_sum_10_tmp_d9509_168 = ((input_limb_54_col55) + (result_y_limb_10_col251));
                let y_sum_11_tmp_d9509_169 = ((input_limb_55_col56) + (result_y_limb_11_col252));
                let y_sum_12_tmp_d9509_170 = ((input_limb_56_col57) + (result_y_limb_12_col253));
                let y_sum_13_tmp_d9509_171 = ((input_limb_57_col58) + (result_y_limb_13_col254));
                let y_sum_14_tmp_d9509_172 = ((input_limb_58_col59) + (result_y_limb_14_col255));
                let y_sum_15_tmp_d9509_173 = ((input_limb_59_col60) + (result_y_limb_15_col256));
                let y_sum_16_tmp_d9509_174 = ((input_limb_60_col61) + (result_y_limb_16_col257));
                let y_sum_17_tmp_d9509_175 = ((input_limb_61_col62) + (result_y_limb_17_col258));
                let y_sum_18_tmp_d9509_176 = ((input_limb_62_col63) + (result_y_limb_18_col259));
                let y_sum_19_tmp_d9509_177 = ((input_limb_63_col64) + (result_y_limb_19_col260));
                let y_sum_20_tmp_d9509_178 = ((input_limb_64_col65) + (result_y_limb_20_col261));
                let y_sum_21_tmp_d9509_179 = ((input_limb_65_col66) + (result_y_limb_21_col262));
                let y_sum_22_tmp_d9509_180 = ((input_limb_66_col67) + (result_y_limb_22_col263));
                let y_sum_23_tmp_d9509_181 = ((input_limb_67_col68) + (result_y_limb_23_col264));
                let y_sum_24_tmp_d9509_182 = ((input_limb_68_col69) + (result_y_limb_24_col265));
                let y_sum_25_tmp_d9509_183 = ((input_limb_69_col70) + (result_y_limb_25_col266));
                let y_sum_26_tmp_d9509_184 = ((input_limb_70_col71) + (result_y_limb_26_col267));
                let y_sum_27_tmp_d9509_185 = ((input_limb_71_col72) + (result_y_limb_27_col268));

                // Verify Mul 252.

                // Double Karatsuba F 0 Fc 6.

                // Single Karatsuba N 7.

                let z0_tmp_d9509_186 = [
                    ((slope_limb_0_col129) * (x_diff2_0_tmp_d9509_130)),
                    (((slope_limb_0_col129) * (x_diff2_1_tmp_d9509_131))
                        + ((slope_limb_1_col130) * (x_diff2_0_tmp_d9509_130))),
                    ((((slope_limb_0_col129) * (x_diff2_2_tmp_d9509_132))
                        + ((slope_limb_1_col130) * (x_diff2_1_tmp_d9509_131)))
                        + ((slope_limb_2_col131) * (x_diff2_0_tmp_d9509_130))),
                    (((((slope_limb_0_col129) * (x_diff2_3_tmp_d9509_133))
                        + ((slope_limb_1_col130) * (x_diff2_2_tmp_d9509_132)))
                        + ((slope_limb_2_col131) * (x_diff2_1_tmp_d9509_131)))
                        + ((slope_limb_3_col132) * (x_diff2_0_tmp_d9509_130))),
                    ((((((slope_limb_0_col129) * (x_diff2_4_tmp_d9509_134))
                        + ((slope_limb_1_col130) * (x_diff2_3_tmp_d9509_133)))
                        + ((slope_limb_2_col131) * (x_diff2_2_tmp_d9509_132)))
                        + ((slope_limb_3_col132) * (x_diff2_1_tmp_d9509_131)))
                        + ((slope_limb_4_col133) * (x_diff2_0_tmp_d9509_130))),
                    (((((((slope_limb_0_col129) * (x_diff2_5_tmp_d9509_135))
                        + ((slope_limb_1_col130) * (x_diff2_4_tmp_d9509_134)))
                        + ((slope_limb_2_col131) * (x_diff2_3_tmp_d9509_133)))
                        + ((slope_limb_3_col132) * (x_diff2_2_tmp_d9509_132)))
                        + ((slope_limb_4_col133) * (x_diff2_1_tmp_d9509_131)))
                        + ((slope_limb_5_col134) * (x_diff2_0_tmp_d9509_130))),
                    ((((((((slope_limb_0_col129) * (x_diff2_6_tmp_d9509_136))
                        + ((slope_limb_1_col130) * (x_diff2_5_tmp_d9509_135)))
                        + ((slope_limb_2_col131) * (x_diff2_4_tmp_d9509_134)))
                        + ((slope_limb_3_col132) * (x_diff2_3_tmp_d9509_133)))
                        + ((slope_limb_4_col133) * (x_diff2_2_tmp_d9509_132)))
                        + ((slope_limb_5_col134) * (x_diff2_1_tmp_d9509_131)))
                        + ((slope_limb_6_col135) * (x_diff2_0_tmp_d9509_130))),
                    (((((((slope_limb_1_col130) * (x_diff2_6_tmp_d9509_136))
                        + ((slope_limb_2_col131) * (x_diff2_5_tmp_d9509_135)))
                        + ((slope_limb_3_col132) * (x_diff2_4_tmp_d9509_134)))
                        + ((slope_limb_4_col133) * (x_diff2_3_tmp_d9509_133)))
                        + ((slope_limb_5_col134) * (x_diff2_2_tmp_d9509_132)))
                        + ((slope_limb_6_col135) * (x_diff2_1_tmp_d9509_131))),
                    ((((((slope_limb_2_col131) * (x_diff2_6_tmp_d9509_136))
                        + ((slope_limb_3_col132) * (x_diff2_5_tmp_d9509_135)))
                        + ((slope_limb_4_col133) * (x_diff2_4_tmp_d9509_134)))
                        + ((slope_limb_5_col134) * (x_diff2_3_tmp_d9509_133)))
                        + ((slope_limb_6_col135) * (x_diff2_2_tmp_d9509_132))),
                    (((((slope_limb_3_col132) * (x_diff2_6_tmp_d9509_136))
                        + ((slope_limb_4_col133) * (x_diff2_5_tmp_d9509_135)))
                        + ((slope_limb_5_col134) * (x_diff2_4_tmp_d9509_134)))
                        + ((slope_limb_6_col135) * (x_diff2_3_tmp_d9509_133))),
                    ((((slope_limb_4_col133) * (x_diff2_6_tmp_d9509_136))
                        + ((slope_limb_5_col134) * (x_diff2_5_tmp_d9509_135)))
                        + ((slope_limb_6_col135) * (x_diff2_4_tmp_d9509_134))),
                    (((slope_limb_5_col134) * (x_diff2_6_tmp_d9509_136))
                        + ((slope_limb_6_col135) * (x_diff2_5_tmp_d9509_135))),
                    ((slope_limb_6_col135) * (x_diff2_6_tmp_d9509_136)),
                ];
                let z2_tmp_d9509_187 = [
                    ((slope_limb_7_col136) * (x_diff2_7_tmp_d9509_137)),
                    (((slope_limb_7_col136) * (x_diff2_8_tmp_d9509_138))
                        + ((slope_limb_8_col137) * (x_diff2_7_tmp_d9509_137))),
                    ((((slope_limb_7_col136) * (x_diff2_9_tmp_d9509_139))
                        + ((slope_limb_8_col137) * (x_diff2_8_tmp_d9509_138)))
                        + ((slope_limb_9_col138) * (x_diff2_7_tmp_d9509_137))),
                    (((((slope_limb_7_col136) * (x_diff2_10_tmp_d9509_140))
                        + ((slope_limb_8_col137) * (x_diff2_9_tmp_d9509_139)))
                        + ((slope_limb_9_col138) * (x_diff2_8_tmp_d9509_138)))
                        + ((slope_limb_10_col139) * (x_diff2_7_tmp_d9509_137))),
                    ((((((slope_limb_7_col136) * (x_diff2_11_tmp_d9509_141))
                        + ((slope_limb_8_col137) * (x_diff2_10_tmp_d9509_140)))
                        + ((slope_limb_9_col138) * (x_diff2_9_tmp_d9509_139)))
                        + ((slope_limb_10_col139) * (x_diff2_8_tmp_d9509_138)))
                        + ((slope_limb_11_col140) * (x_diff2_7_tmp_d9509_137))),
                    (((((((slope_limb_7_col136) * (x_diff2_12_tmp_d9509_142))
                        + ((slope_limb_8_col137) * (x_diff2_11_tmp_d9509_141)))
                        + ((slope_limb_9_col138) * (x_diff2_10_tmp_d9509_140)))
                        + ((slope_limb_10_col139) * (x_diff2_9_tmp_d9509_139)))
                        + ((slope_limb_11_col140) * (x_diff2_8_tmp_d9509_138)))
                        + ((slope_limb_12_col141) * (x_diff2_7_tmp_d9509_137))),
                    ((((((((slope_limb_7_col136) * (x_diff2_13_tmp_d9509_143))
                        + ((slope_limb_8_col137) * (x_diff2_12_tmp_d9509_142)))
                        + ((slope_limb_9_col138) * (x_diff2_11_tmp_d9509_141)))
                        + ((slope_limb_10_col139) * (x_diff2_10_tmp_d9509_140)))
                        + ((slope_limb_11_col140) * (x_diff2_9_tmp_d9509_139)))
                        + ((slope_limb_12_col141) * (x_diff2_8_tmp_d9509_138)))
                        + ((slope_limb_13_col142) * (x_diff2_7_tmp_d9509_137))),
                    (((((((slope_limb_8_col137) * (x_diff2_13_tmp_d9509_143))
                        + ((slope_limb_9_col138) * (x_diff2_12_tmp_d9509_142)))
                        + ((slope_limb_10_col139) * (x_diff2_11_tmp_d9509_141)))
                        + ((slope_limb_11_col140) * (x_diff2_10_tmp_d9509_140)))
                        + ((slope_limb_12_col141) * (x_diff2_9_tmp_d9509_139)))
                        + ((slope_limb_13_col142) * (x_diff2_8_tmp_d9509_138))),
                    ((((((slope_limb_9_col138) * (x_diff2_13_tmp_d9509_143))
                        + ((slope_limb_10_col139) * (x_diff2_12_tmp_d9509_142)))
                        + ((slope_limb_11_col140) * (x_diff2_11_tmp_d9509_141)))
                        + ((slope_limb_12_col141) * (x_diff2_10_tmp_d9509_140)))
                        + ((slope_limb_13_col142) * (x_diff2_9_tmp_d9509_139))),
                    (((((slope_limb_10_col139) * (x_diff2_13_tmp_d9509_143))
                        + ((slope_limb_11_col140) * (x_diff2_12_tmp_d9509_142)))
                        + ((slope_limb_12_col141) * (x_diff2_11_tmp_d9509_141)))
                        + ((slope_limb_13_col142) * (x_diff2_10_tmp_d9509_140))),
                    ((((slope_limb_11_col140) * (x_diff2_13_tmp_d9509_143))
                        + ((slope_limb_12_col141) * (x_diff2_12_tmp_d9509_142)))
                        + ((slope_limb_13_col142) * (x_diff2_11_tmp_d9509_141))),
                    (((slope_limb_12_col141) * (x_diff2_13_tmp_d9509_143))
                        + ((slope_limb_13_col142) * (x_diff2_12_tmp_d9509_142))),
                    ((slope_limb_13_col142) * (x_diff2_13_tmp_d9509_143)),
                ];
                let x_sum_tmp_d9509_188 = [
                    ((slope_limb_0_col129) + (slope_limb_7_col136)),
                    ((slope_limb_1_col130) + (slope_limb_8_col137)),
                    ((slope_limb_2_col131) + (slope_limb_9_col138)),
                    ((slope_limb_3_col132) + (slope_limb_10_col139)),
                    ((slope_limb_4_col133) + (slope_limb_11_col140)),
                    ((slope_limb_5_col134) + (slope_limb_12_col141)),
                    ((slope_limb_6_col135) + (slope_limb_13_col142)),
                ];
                let y_sum_tmp_d9509_189 = [
                    ((x_diff2_0_tmp_d9509_130) + (x_diff2_7_tmp_d9509_137)),
                    ((x_diff2_1_tmp_d9509_131) + (x_diff2_8_tmp_d9509_138)),
                    ((x_diff2_2_tmp_d9509_132) + (x_diff2_9_tmp_d9509_139)),
                    ((x_diff2_3_tmp_d9509_133) + (x_diff2_10_tmp_d9509_140)),
                    ((x_diff2_4_tmp_d9509_134) + (x_diff2_11_tmp_d9509_141)),
                    ((x_diff2_5_tmp_d9509_135) + (x_diff2_12_tmp_d9509_142)),
                    ((x_diff2_6_tmp_d9509_136) + (x_diff2_13_tmp_d9509_143)),
                ];
                let single_karatsuba_n_7_output_tmp_d9509_190 = [
                    z0_tmp_d9509_186[0],
                    z0_tmp_d9509_186[1],
                    z0_tmp_d9509_186[2],
                    z0_tmp_d9509_186[3],
                    z0_tmp_d9509_186[4],
                    z0_tmp_d9509_186[5],
                    z0_tmp_d9509_186[6],
                    ((z0_tmp_d9509_186[7])
                        + ((((x_sum_tmp_d9509_188[0]) * (y_sum_tmp_d9509_189[0]))
                            - (z0_tmp_d9509_186[0]))
                            - (z2_tmp_d9509_187[0]))),
                    ((z0_tmp_d9509_186[8])
                        + (((((x_sum_tmp_d9509_188[0]) * (y_sum_tmp_d9509_189[1]))
                            + ((x_sum_tmp_d9509_188[1]) * (y_sum_tmp_d9509_189[0])))
                            - (z0_tmp_d9509_186[1]))
                            - (z2_tmp_d9509_187[1]))),
                    ((z0_tmp_d9509_186[9])
                        + ((((((x_sum_tmp_d9509_188[0]) * (y_sum_tmp_d9509_189[2]))
                            + ((x_sum_tmp_d9509_188[1]) * (y_sum_tmp_d9509_189[1])))
                            + ((x_sum_tmp_d9509_188[2]) * (y_sum_tmp_d9509_189[0])))
                            - (z0_tmp_d9509_186[2]))
                            - (z2_tmp_d9509_187[2]))),
                    ((z0_tmp_d9509_186[10])
                        + (((((((x_sum_tmp_d9509_188[0]) * (y_sum_tmp_d9509_189[3]))
                            + ((x_sum_tmp_d9509_188[1]) * (y_sum_tmp_d9509_189[2])))
                            + ((x_sum_tmp_d9509_188[2]) * (y_sum_tmp_d9509_189[1])))
                            + ((x_sum_tmp_d9509_188[3]) * (y_sum_tmp_d9509_189[0])))
                            - (z0_tmp_d9509_186[3]))
                            - (z2_tmp_d9509_187[3]))),
                    ((z0_tmp_d9509_186[11])
                        + ((((((((x_sum_tmp_d9509_188[0]) * (y_sum_tmp_d9509_189[4]))
                            + ((x_sum_tmp_d9509_188[1]) * (y_sum_tmp_d9509_189[3])))
                            + ((x_sum_tmp_d9509_188[2]) * (y_sum_tmp_d9509_189[2])))
                            + ((x_sum_tmp_d9509_188[3]) * (y_sum_tmp_d9509_189[1])))
                            + ((x_sum_tmp_d9509_188[4]) * (y_sum_tmp_d9509_189[0])))
                            - (z0_tmp_d9509_186[4]))
                            - (z2_tmp_d9509_187[4]))),
                    ((z0_tmp_d9509_186[12])
                        + (((((((((x_sum_tmp_d9509_188[0]) * (y_sum_tmp_d9509_189[5]))
                            + ((x_sum_tmp_d9509_188[1]) * (y_sum_tmp_d9509_189[4])))
                            + ((x_sum_tmp_d9509_188[2]) * (y_sum_tmp_d9509_189[3])))
                            + ((x_sum_tmp_d9509_188[3]) * (y_sum_tmp_d9509_189[2])))
                            + ((x_sum_tmp_d9509_188[4]) * (y_sum_tmp_d9509_189[1])))
                            + ((x_sum_tmp_d9509_188[5]) * (y_sum_tmp_d9509_189[0])))
                            - (z0_tmp_d9509_186[5]))
                            - (z2_tmp_d9509_187[5]))),
                    ((((((((((x_sum_tmp_d9509_188[0]) * (y_sum_tmp_d9509_189[6]))
                        + ((x_sum_tmp_d9509_188[1]) * (y_sum_tmp_d9509_189[5])))
                        + ((x_sum_tmp_d9509_188[2]) * (y_sum_tmp_d9509_189[4])))
                        + ((x_sum_tmp_d9509_188[3]) * (y_sum_tmp_d9509_189[3])))
                        + ((x_sum_tmp_d9509_188[4]) * (y_sum_tmp_d9509_189[2])))
                        + ((x_sum_tmp_d9509_188[5]) * (y_sum_tmp_d9509_189[1])))
                        + ((x_sum_tmp_d9509_188[6]) * (y_sum_tmp_d9509_189[0])))
                        - (z0_tmp_d9509_186[6]))
                        - (z2_tmp_d9509_187[6])),
                    ((z2_tmp_d9509_187[0])
                        + (((((((((x_sum_tmp_d9509_188[1]) * (y_sum_tmp_d9509_189[6]))
                            + ((x_sum_tmp_d9509_188[2]) * (y_sum_tmp_d9509_189[5])))
                            + ((x_sum_tmp_d9509_188[3]) * (y_sum_tmp_d9509_189[4])))
                            + ((x_sum_tmp_d9509_188[4]) * (y_sum_tmp_d9509_189[3])))
                            + ((x_sum_tmp_d9509_188[5]) * (y_sum_tmp_d9509_189[2])))
                            + ((x_sum_tmp_d9509_188[6]) * (y_sum_tmp_d9509_189[1])))
                            - (z0_tmp_d9509_186[7]))
                            - (z2_tmp_d9509_187[7]))),
                    ((z2_tmp_d9509_187[1])
                        + ((((((((x_sum_tmp_d9509_188[2]) * (y_sum_tmp_d9509_189[6]))
                            + ((x_sum_tmp_d9509_188[3]) * (y_sum_tmp_d9509_189[5])))
                            + ((x_sum_tmp_d9509_188[4]) * (y_sum_tmp_d9509_189[4])))
                            + ((x_sum_tmp_d9509_188[5]) * (y_sum_tmp_d9509_189[3])))
                            + ((x_sum_tmp_d9509_188[6]) * (y_sum_tmp_d9509_189[2])))
                            - (z0_tmp_d9509_186[8]))
                            - (z2_tmp_d9509_187[8]))),
                    ((z2_tmp_d9509_187[2])
                        + (((((((x_sum_tmp_d9509_188[3]) * (y_sum_tmp_d9509_189[6]))
                            + ((x_sum_tmp_d9509_188[4]) * (y_sum_tmp_d9509_189[5])))
                            + ((x_sum_tmp_d9509_188[5]) * (y_sum_tmp_d9509_189[4])))
                            + ((x_sum_tmp_d9509_188[6]) * (y_sum_tmp_d9509_189[3])))
                            - (z0_tmp_d9509_186[9]))
                            - (z2_tmp_d9509_187[9]))),
                    ((z2_tmp_d9509_187[3])
                        + ((((((x_sum_tmp_d9509_188[4]) * (y_sum_tmp_d9509_189[6]))
                            + ((x_sum_tmp_d9509_188[5]) * (y_sum_tmp_d9509_189[5])))
                            + ((x_sum_tmp_d9509_188[6]) * (y_sum_tmp_d9509_189[4])))
                            - (z0_tmp_d9509_186[10]))
                            - (z2_tmp_d9509_187[10]))),
                    ((z2_tmp_d9509_187[4])
                        + (((((x_sum_tmp_d9509_188[5]) * (y_sum_tmp_d9509_189[6]))
                            + ((x_sum_tmp_d9509_188[6]) * (y_sum_tmp_d9509_189[5])))
                            - (z0_tmp_d9509_186[11]))
                            - (z2_tmp_d9509_187[11]))),
                    ((z2_tmp_d9509_187[5])
                        + ((((x_sum_tmp_d9509_188[6]) * (y_sum_tmp_d9509_189[6]))
                            - (z0_tmp_d9509_186[12]))
                            - (z2_tmp_d9509_187[12]))),
                    z2_tmp_d9509_187[6],
                    z2_tmp_d9509_187[7],
                    z2_tmp_d9509_187[8],
                    z2_tmp_d9509_187[9],
                    z2_tmp_d9509_187[10],
                    z2_tmp_d9509_187[11],
                    z2_tmp_d9509_187[12],
                ];

                // Single Karatsuba N 7.

                let z0_tmp_d9509_191 = [
                    ((slope_limb_14_col143) * (x_diff2_14_tmp_d9509_144)),
                    (((slope_limb_14_col143) * (x_diff2_15_tmp_d9509_145))
                        + ((slope_limb_15_col144) * (x_diff2_14_tmp_d9509_144))),
                    ((((slope_limb_14_col143) * (x_diff2_16_tmp_d9509_146))
                        + ((slope_limb_15_col144) * (x_diff2_15_tmp_d9509_145)))
                        + ((slope_limb_16_col145) * (x_diff2_14_tmp_d9509_144))),
                    (((((slope_limb_14_col143) * (x_diff2_17_tmp_d9509_147))
                        + ((slope_limb_15_col144) * (x_diff2_16_tmp_d9509_146)))
                        + ((slope_limb_16_col145) * (x_diff2_15_tmp_d9509_145)))
                        + ((slope_limb_17_col146) * (x_diff2_14_tmp_d9509_144))),
                    ((((((slope_limb_14_col143) * (x_diff2_18_tmp_d9509_148))
                        + ((slope_limb_15_col144) * (x_diff2_17_tmp_d9509_147)))
                        + ((slope_limb_16_col145) * (x_diff2_16_tmp_d9509_146)))
                        + ((slope_limb_17_col146) * (x_diff2_15_tmp_d9509_145)))
                        + ((slope_limb_18_col147) * (x_diff2_14_tmp_d9509_144))),
                    (((((((slope_limb_14_col143) * (x_diff2_19_tmp_d9509_149))
                        + ((slope_limb_15_col144) * (x_diff2_18_tmp_d9509_148)))
                        + ((slope_limb_16_col145) * (x_diff2_17_tmp_d9509_147)))
                        + ((slope_limb_17_col146) * (x_diff2_16_tmp_d9509_146)))
                        + ((slope_limb_18_col147) * (x_diff2_15_tmp_d9509_145)))
                        + ((slope_limb_19_col148) * (x_diff2_14_tmp_d9509_144))),
                    ((((((((slope_limb_14_col143) * (x_diff2_20_tmp_d9509_150))
                        + ((slope_limb_15_col144) * (x_diff2_19_tmp_d9509_149)))
                        + ((slope_limb_16_col145) * (x_diff2_18_tmp_d9509_148)))
                        + ((slope_limb_17_col146) * (x_diff2_17_tmp_d9509_147)))
                        + ((slope_limb_18_col147) * (x_diff2_16_tmp_d9509_146)))
                        + ((slope_limb_19_col148) * (x_diff2_15_tmp_d9509_145)))
                        + ((slope_limb_20_col149) * (x_diff2_14_tmp_d9509_144))),
                    (((((((slope_limb_15_col144) * (x_diff2_20_tmp_d9509_150))
                        + ((slope_limb_16_col145) * (x_diff2_19_tmp_d9509_149)))
                        + ((slope_limb_17_col146) * (x_diff2_18_tmp_d9509_148)))
                        + ((slope_limb_18_col147) * (x_diff2_17_tmp_d9509_147)))
                        + ((slope_limb_19_col148) * (x_diff2_16_tmp_d9509_146)))
                        + ((slope_limb_20_col149) * (x_diff2_15_tmp_d9509_145))),
                    ((((((slope_limb_16_col145) * (x_diff2_20_tmp_d9509_150))
                        + ((slope_limb_17_col146) * (x_diff2_19_tmp_d9509_149)))
                        + ((slope_limb_18_col147) * (x_diff2_18_tmp_d9509_148)))
                        + ((slope_limb_19_col148) * (x_diff2_17_tmp_d9509_147)))
                        + ((slope_limb_20_col149) * (x_diff2_16_tmp_d9509_146))),
                    (((((slope_limb_17_col146) * (x_diff2_20_tmp_d9509_150))
                        + ((slope_limb_18_col147) * (x_diff2_19_tmp_d9509_149)))
                        + ((slope_limb_19_col148) * (x_diff2_18_tmp_d9509_148)))
                        + ((slope_limb_20_col149) * (x_diff2_17_tmp_d9509_147))),
                    ((((slope_limb_18_col147) * (x_diff2_20_tmp_d9509_150))
                        + ((slope_limb_19_col148) * (x_diff2_19_tmp_d9509_149)))
                        + ((slope_limb_20_col149) * (x_diff2_18_tmp_d9509_148))),
                    (((slope_limb_19_col148) * (x_diff2_20_tmp_d9509_150))
                        + ((slope_limb_20_col149) * (x_diff2_19_tmp_d9509_149))),
                    ((slope_limb_20_col149) * (x_diff2_20_tmp_d9509_150)),
                ];
                let z2_tmp_d9509_192 = [
                    ((slope_limb_21_col150) * (x_diff2_21_tmp_d9509_151)),
                    (((slope_limb_21_col150) * (x_diff2_22_tmp_d9509_152))
                        + ((slope_limb_22_col151) * (x_diff2_21_tmp_d9509_151))),
                    ((((slope_limb_21_col150) * (x_diff2_23_tmp_d9509_153))
                        + ((slope_limb_22_col151) * (x_diff2_22_tmp_d9509_152)))
                        + ((slope_limb_23_col152) * (x_diff2_21_tmp_d9509_151))),
                    (((((slope_limb_21_col150) * (x_diff2_24_tmp_d9509_154))
                        + ((slope_limb_22_col151) * (x_diff2_23_tmp_d9509_153)))
                        + ((slope_limb_23_col152) * (x_diff2_22_tmp_d9509_152)))
                        + ((slope_limb_24_col153) * (x_diff2_21_tmp_d9509_151))),
                    ((((((slope_limb_21_col150) * (x_diff2_25_tmp_d9509_155))
                        + ((slope_limb_22_col151) * (x_diff2_24_tmp_d9509_154)))
                        + ((slope_limb_23_col152) * (x_diff2_23_tmp_d9509_153)))
                        + ((slope_limb_24_col153) * (x_diff2_22_tmp_d9509_152)))
                        + ((slope_limb_25_col154) * (x_diff2_21_tmp_d9509_151))),
                    (((((((slope_limb_21_col150) * (x_diff2_26_tmp_d9509_156))
                        + ((slope_limb_22_col151) * (x_diff2_25_tmp_d9509_155)))
                        + ((slope_limb_23_col152) * (x_diff2_24_tmp_d9509_154)))
                        + ((slope_limb_24_col153) * (x_diff2_23_tmp_d9509_153)))
                        + ((slope_limb_25_col154) * (x_diff2_22_tmp_d9509_152)))
                        + ((slope_limb_26_col155) * (x_diff2_21_tmp_d9509_151))),
                    ((((((((slope_limb_21_col150) * (x_diff2_27_tmp_d9509_157))
                        + ((slope_limb_22_col151) * (x_diff2_26_tmp_d9509_156)))
                        + ((slope_limb_23_col152) * (x_diff2_25_tmp_d9509_155)))
                        + ((slope_limb_24_col153) * (x_diff2_24_tmp_d9509_154)))
                        + ((slope_limb_25_col154) * (x_diff2_23_tmp_d9509_153)))
                        + ((slope_limb_26_col155) * (x_diff2_22_tmp_d9509_152)))
                        + ((slope_limb_27_col156) * (x_diff2_21_tmp_d9509_151))),
                    (((((((slope_limb_22_col151) * (x_diff2_27_tmp_d9509_157))
                        + ((slope_limb_23_col152) * (x_diff2_26_tmp_d9509_156)))
                        + ((slope_limb_24_col153) * (x_diff2_25_tmp_d9509_155)))
                        + ((slope_limb_25_col154) * (x_diff2_24_tmp_d9509_154)))
                        + ((slope_limb_26_col155) * (x_diff2_23_tmp_d9509_153)))
                        + ((slope_limb_27_col156) * (x_diff2_22_tmp_d9509_152))),
                    ((((((slope_limb_23_col152) * (x_diff2_27_tmp_d9509_157))
                        + ((slope_limb_24_col153) * (x_diff2_26_tmp_d9509_156)))
                        + ((slope_limb_25_col154) * (x_diff2_25_tmp_d9509_155)))
                        + ((slope_limb_26_col155) * (x_diff2_24_tmp_d9509_154)))
                        + ((slope_limb_27_col156) * (x_diff2_23_tmp_d9509_153))),
                    (((((slope_limb_24_col153) * (x_diff2_27_tmp_d9509_157))
                        + ((slope_limb_25_col154) * (x_diff2_26_tmp_d9509_156)))
                        + ((slope_limb_26_col155) * (x_diff2_25_tmp_d9509_155)))
                        + ((slope_limb_27_col156) * (x_diff2_24_tmp_d9509_154))),
                    ((((slope_limb_25_col154) * (x_diff2_27_tmp_d9509_157))
                        + ((slope_limb_26_col155) * (x_diff2_26_tmp_d9509_156)))
                        + ((slope_limb_27_col156) * (x_diff2_25_tmp_d9509_155))),
                    (((slope_limb_26_col155) * (x_diff2_27_tmp_d9509_157))
                        + ((slope_limb_27_col156) * (x_diff2_26_tmp_d9509_156))),
                    ((slope_limb_27_col156) * (x_diff2_27_tmp_d9509_157)),
                ];
                let x_sum_tmp_d9509_193 = [
                    ((slope_limb_14_col143) + (slope_limb_21_col150)),
                    ((slope_limb_15_col144) + (slope_limb_22_col151)),
                    ((slope_limb_16_col145) + (slope_limb_23_col152)),
                    ((slope_limb_17_col146) + (slope_limb_24_col153)),
                    ((slope_limb_18_col147) + (slope_limb_25_col154)),
                    ((slope_limb_19_col148) + (slope_limb_26_col155)),
                    ((slope_limb_20_col149) + (slope_limb_27_col156)),
                ];
                let y_sum_tmp_d9509_194 = [
                    ((x_diff2_14_tmp_d9509_144) + (x_diff2_21_tmp_d9509_151)),
                    ((x_diff2_15_tmp_d9509_145) + (x_diff2_22_tmp_d9509_152)),
                    ((x_diff2_16_tmp_d9509_146) + (x_diff2_23_tmp_d9509_153)),
                    ((x_diff2_17_tmp_d9509_147) + (x_diff2_24_tmp_d9509_154)),
                    ((x_diff2_18_tmp_d9509_148) + (x_diff2_25_tmp_d9509_155)),
                    ((x_diff2_19_tmp_d9509_149) + (x_diff2_26_tmp_d9509_156)),
                    ((x_diff2_20_tmp_d9509_150) + (x_diff2_27_tmp_d9509_157)),
                ];
                let single_karatsuba_n_7_output_tmp_d9509_195 = [
                    z0_tmp_d9509_191[0],
                    z0_tmp_d9509_191[1],
                    z0_tmp_d9509_191[2],
                    z0_tmp_d9509_191[3],
                    z0_tmp_d9509_191[4],
                    z0_tmp_d9509_191[5],
                    z0_tmp_d9509_191[6],
                    ((z0_tmp_d9509_191[7])
                        + ((((x_sum_tmp_d9509_193[0]) * (y_sum_tmp_d9509_194[0]))
                            - (z0_tmp_d9509_191[0]))
                            - (z2_tmp_d9509_192[0]))),
                    ((z0_tmp_d9509_191[8])
                        + (((((x_sum_tmp_d9509_193[0]) * (y_sum_tmp_d9509_194[1]))
                            + ((x_sum_tmp_d9509_193[1]) * (y_sum_tmp_d9509_194[0])))
                            - (z0_tmp_d9509_191[1]))
                            - (z2_tmp_d9509_192[1]))),
                    ((z0_tmp_d9509_191[9])
                        + ((((((x_sum_tmp_d9509_193[0]) * (y_sum_tmp_d9509_194[2]))
                            + ((x_sum_tmp_d9509_193[1]) * (y_sum_tmp_d9509_194[1])))
                            + ((x_sum_tmp_d9509_193[2]) * (y_sum_tmp_d9509_194[0])))
                            - (z0_tmp_d9509_191[2]))
                            - (z2_tmp_d9509_192[2]))),
                    ((z0_tmp_d9509_191[10])
                        + (((((((x_sum_tmp_d9509_193[0]) * (y_sum_tmp_d9509_194[3]))
                            + ((x_sum_tmp_d9509_193[1]) * (y_sum_tmp_d9509_194[2])))
                            + ((x_sum_tmp_d9509_193[2]) * (y_sum_tmp_d9509_194[1])))
                            + ((x_sum_tmp_d9509_193[3]) * (y_sum_tmp_d9509_194[0])))
                            - (z0_tmp_d9509_191[3]))
                            - (z2_tmp_d9509_192[3]))),
                    ((z0_tmp_d9509_191[11])
                        + ((((((((x_sum_tmp_d9509_193[0]) * (y_sum_tmp_d9509_194[4]))
                            + ((x_sum_tmp_d9509_193[1]) * (y_sum_tmp_d9509_194[3])))
                            + ((x_sum_tmp_d9509_193[2]) * (y_sum_tmp_d9509_194[2])))
                            + ((x_sum_tmp_d9509_193[3]) * (y_sum_tmp_d9509_194[1])))
                            + ((x_sum_tmp_d9509_193[4]) * (y_sum_tmp_d9509_194[0])))
                            - (z0_tmp_d9509_191[4]))
                            - (z2_tmp_d9509_192[4]))),
                    ((z0_tmp_d9509_191[12])
                        + (((((((((x_sum_tmp_d9509_193[0]) * (y_sum_tmp_d9509_194[5]))
                            + ((x_sum_tmp_d9509_193[1]) * (y_sum_tmp_d9509_194[4])))
                            + ((x_sum_tmp_d9509_193[2]) * (y_sum_tmp_d9509_194[3])))
                            + ((x_sum_tmp_d9509_193[3]) * (y_sum_tmp_d9509_194[2])))
                            + ((x_sum_tmp_d9509_193[4]) * (y_sum_tmp_d9509_194[1])))
                            + ((x_sum_tmp_d9509_193[5]) * (y_sum_tmp_d9509_194[0])))
                            - (z0_tmp_d9509_191[5]))
                            - (z2_tmp_d9509_192[5]))),
                    ((((((((((x_sum_tmp_d9509_193[0]) * (y_sum_tmp_d9509_194[6]))
                        + ((x_sum_tmp_d9509_193[1]) * (y_sum_tmp_d9509_194[5])))
                        + ((x_sum_tmp_d9509_193[2]) * (y_sum_tmp_d9509_194[4])))
                        + ((x_sum_tmp_d9509_193[3]) * (y_sum_tmp_d9509_194[3])))
                        + ((x_sum_tmp_d9509_193[4]) * (y_sum_tmp_d9509_194[2])))
                        + ((x_sum_tmp_d9509_193[5]) * (y_sum_tmp_d9509_194[1])))
                        + ((x_sum_tmp_d9509_193[6]) * (y_sum_tmp_d9509_194[0])))
                        - (z0_tmp_d9509_191[6]))
                        - (z2_tmp_d9509_192[6])),
                    ((z2_tmp_d9509_192[0])
                        + (((((((((x_sum_tmp_d9509_193[1]) * (y_sum_tmp_d9509_194[6]))
                            + ((x_sum_tmp_d9509_193[2]) * (y_sum_tmp_d9509_194[5])))
                            + ((x_sum_tmp_d9509_193[3]) * (y_sum_tmp_d9509_194[4])))
                            + ((x_sum_tmp_d9509_193[4]) * (y_sum_tmp_d9509_194[3])))
                            + ((x_sum_tmp_d9509_193[5]) * (y_sum_tmp_d9509_194[2])))
                            + ((x_sum_tmp_d9509_193[6]) * (y_sum_tmp_d9509_194[1])))
                            - (z0_tmp_d9509_191[7]))
                            - (z2_tmp_d9509_192[7]))),
                    ((z2_tmp_d9509_192[1])
                        + ((((((((x_sum_tmp_d9509_193[2]) * (y_sum_tmp_d9509_194[6]))
                            + ((x_sum_tmp_d9509_193[3]) * (y_sum_tmp_d9509_194[5])))
                            + ((x_sum_tmp_d9509_193[4]) * (y_sum_tmp_d9509_194[4])))
                            + ((x_sum_tmp_d9509_193[5]) * (y_sum_tmp_d9509_194[3])))
                            + ((x_sum_tmp_d9509_193[6]) * (y_sum_tmp_d9509_194[2])))
                            - (z0_tmp_d9509_191[8]))
                            - (z2_tmp_d9509_192[8]))),
                    ((z2_tmp_d9509_192[2])
                        + (((((((x_sum_tmp_d9509_193[3]) * (y_sum_tmp_d9509_194[6]))
                            + ((x_sum_tmp_d9509_193[4]) * (y_sum_tmp_d9509_194[5])))
                            + ((x_sum_tmp_d9509_193[5]) * (y_sum_tmp_d9509_194[4])))
                            + ((x_sum_tmp_d9509_193[6]) * (y_sum_tmp_d9509_194[3])))
                            - (z0_tmp_d9509_191[9]))
                            - (z2_tmp_d9509_192[9]))),
                    ((z2_tmp_d9509_192[3])
                        + ((((((x_sum_tmp_d9509_193[4]) * (y_sum_tmp_d9509_194[6]))
                            + ((x_sum_tmp_d9509_193[5]) * (y_sum_tmp_d9509_194[5])))
                            + ((x_sum_tmp_d9509_193[6]) * (y_sum_tmp_d9509_194[4])))
                            - (z0_tmp_d9509_191[10]))
                            - (z2_tmp_d9509_192[10]))),
                    ((z2_tmp_d9509_192[4])
                        + (((((x_sum_tmp_d9509_193[5]) * (y_sum_tmp_d9509_194[6]))
                            + ((x_sum_tmp_d9509_193[6]) * (y_sum_tmp_d9509_194[5])))
                            - (z0_tmp_d9509_191[11]))
                            - (z2_tmp_d9509_192[11]))),
                    ((z2_tmp_d9509_192[5])
                        + ((((x_sum_tmp_d9509_193[6]) * (y_sum_tmp_d9509_194[6]))
                            - (z0_tmp_d9509_191[12]))
                            - (z2_tmp_d9509_192[12]))),
                    z2_tmp_d9509_192[6],
                    z2_tmp_d9509_192[7],
                    z2_tmp_d9509_192[8],
                    z2_tmp_d9509_192[9],
                    z2_tmp_d9509_192[10],
                    z2_tmp_d9509_192[11],
                    z2_tmp_d9509_192[12],
                ];

                let x_sum_tmp_d9509_196 = [
                    ((slope_limb_0_col129) + (slope_limb_14_col143)),
                    ((slope_limb_1_col130) + (slope_limb_15_col144)),
                    ((slope_limb_2_col131) + (slope_limb_16_col145)),
                    ((slope_limb_3_col132) + (slope_limb_17_col146)),
                    ((slope_limb_4_col133) + (slope_limb_18_col147)),
                    ((slope_limb_5_col134) + (slope_limb_19_col148)),
                    ((slope_limb_6_col135) + (slope_limb_20_col149)),
                    ((slope_limb_7_col136) + (slope_limb_21_col150)),
                    ((slope_limb_8_col137) + (slope_limb_22_col151)),
                    ((slope_limb_9_col138) + (slope_limb_23_col152)),
                    ((slope_limb_10_col139) + (slope_limb_24_col153)),
                    ((slope_limb_11_col140) + (slope_limb_25_col154)),
                    ((slope_limb_12_col141) + (slope_limb_26_col155)),
                    ((slope_limb_13_col142) + (slope_limb_27_col156)),
                ];
                let y_sum_tmp_d9509_197 = [
                    ((x_diff2_0_tmp_d9509_130) + (x_diff2_14_tmp_d9509_144)),
                    ((x_diff2_1_tmp_d9509_131) + (x_diff2_15_tmp_d9509_145)),
                    ((x_diff2_2_tmp_d9509_132) + (x_diff2_16_tmp_d9509_146)),
                    ((x_diff2_3_tmp_d9509_133) + (x_diff2_17_tmp_d9509_147)),
                    ((x_diff2_4_tmp_d9509_134) + (x_diff2_18_tmp_d9509_148)),
                    ((x_diff2_5_tmp_d9509_135) + (x_diff2_19_tmp_d9509_149)),
                    ((x_diff2_6_tmp_d9509_136) + (x_diff2_20_tmp_d9509_150)),
                    ((x_diff2_7_tmp_d9509_137) + (x_diff2_21_tmp_d9509_151)),
                    ((x_diff2_8_tmp_d9509_138) + (x_diff2_22_tmp_d9509_152)),
                    ((x_diff2_9_tmp_d9509_139) + (x_diff2_23_tmp_d9509_153)),
                    ((x_diff2_10_tmp_d9509_140) + (x_diff2_24_tmp_d9509_154)),
                    ((x_diff2_11_tmp_d9509_141) + (x_diff2_25_tmp_d9509_155)),
                    ((x_diff2_12_tmp_d9509_142) + (x_diff2_26_tmp_d9509_156)),
                    ((x_diff2_13_tmp_d9509_143) + (x_diff2_27_tmp_d9509_157)),
                ];

                // Single Karatsuba N 7.

                let z0_tmp_d9509_198 = [
                    ((x_sum_tmp_d9509_196[0]) * (y_sum_tmp_d9509_197[0])),
                    (((x_sum_tmp_d9509_196[0]) * (y_sum_tmp_d9509_197[1]))
                        + ((x_sum_tmp_d9509_196[1]) * (y_sum_tmp_d9509_197[0]))),
                    ((((x_sum_tmp_d9509_196[0]) * (y_sum_tmp_d9509_197[2]))
                        + ((x_sum_tmp_d9509_196[1]) * (y_sum_tmp_d9509_197[1])))
                        + ((x_sum_tmp_d9509_196[2]) * (y_sum_tmp_d9509_197[0]))),
                    (((((x_sum_tmp_d9509_196[0]) * (y_sum_tmp_d9509_197[3]))
                        + ((x_sum_tmp_d9509_196[1]) * (y_sum_tmp_d9509_197[2])))
                        + ((x_sum_tmp_d9509_196[2]) * (y_sum_tmp_d9509_197[1])))
                        + ((x_sum_tmp_d9509_196[3]) * (y_sum_tmp_d9509_197[0]))),
                    ((((((x_sum_tmp_d9509_196[0]) * (y_sum_tmp_d9509_197[4]))
                        + ((x_sum_tmp_d9509_196[1]) * (y_sum_tmp_d9509_197[3])))
                        + ((x_sum_tmp_d9509_196[2]) * (y_sum_tmp_d9509_197[2])))
                        + ((x_sum_tmp_d9509_196[3]) * (y_sum_tmp_d9509_197[1])))
                        + ((x_sum_tmp_d9509_196[4]) * (y_sum_tmp_d9509_197[0]))),
                    (((((((x_sum_tmp_d9509_196[0]) * (y_sum_tmp_d9509_197[5]))
                        + ((x_sum_tmp_d9509_196[1]) * (y_sum_tmp_d9509_197[4])))
                        + ((x_sum_tmp_d9509_196[2]) * (y_sum_tmp_d9509_197[3])))
                        + ((x_sum_tmp_d9509_196[3]) * (y_sum_tmp_d9509_197[2])))
                        + ((x_sum_tmp_d9509_196[4]) * (y_sum_tmp_d9509_197[1])))
                        + ((x_sum_tmp_d9509_196[5]) * (y_sum_tmp_d9509_197[0]))),
                    ((((((((x_sum_tmp_d9509_196[0]) * (y_sum_tmp_d9509_197[6]))
                        + ((x_sum_tmp_d9509_196[1]) * (y_sum_tmp_d9509_197[5])))
                        + ((x_sum_tmp_d9509_196[2]) * (y_sum_tmp_d9509_197[4])))
                        + ((x_sum_tmp_d9509_196[3]) * (y_sum_tmp_d9509_197[3])))
                        + ((x_sum_tmp_d9509_196[4]) * (y_sum_tmp_d9509_197[2])))
                        + ((x_sum_tmp_d9509_196[5]) * (y_sum_tmp_d9509_197[1])))
                        + ((x_sum_tmp_d9509_196[6]) * (y_sum_tmp_d9509_197[0]))),
                    (((((((x_sum_tmp_d9509_196[1]) * (y_sum_tmp_d9509_197[6]))
                        + ((x_sum_tmp_d9509_196[2]) * (y_sum_tmp_d9509_197[5])))
                        + ((x_sum_tmp_d9509_196[3]) * (y_sum_tmp_d9509_197[4])))
                        + ((x_sum_tmp_d9509_196[4]) * (y_sum_tmp_d9509_197[3])))
                        + ((x_sum_tmp_d9509_196[5]) * (y_sum_tmp_d9509_197[2])))
                        + ((x_sum_tmp_d9509_196[6]) * (y_sum_tmp_d9509_197[1]))),
                    ((((((x_sum_tmp_d9509_196[2]) * (y_sum_tmp_d9509_197[6]))
                        + ((x_sum_tmp_d9509_196[3]) * (y_sum_tmp_d9509_197[5])))
                        + ((x_sum_tmp_d9509_196[4]) * (y_sum_tmp_d9509_197[4])))
                        + ((x_sum_tmp_d9509_196[5]) * (y_sum_tmp_d9509_197[3])))
                        + ((x_sum_tmp_d9509_196[6]) * (y_sum_tmp_d9509_197[2]))),
                    (((((x_sum_tmp_d9509_196[3]) * (y_sum_tmp_d9509_197[6]))
                        + ((x_sum_tmp_d9509_196[4]) * (y_sum_tmp_d9509_197[5])))
                        + ((x_sum_tmp_d9509_196[5]) * (y_sum_tmp_d9509_197[4])))
                        + ((x_sum_tmp_d9509_196[6]) * (y_sum_tmp_d9509_197[3]))),
                    ((((x_sum_tmp_d9509_196[4]) * (y_sum_tmp_d9509_197[6]))
                        + ((x_sum_tmp_d9509_196[5]) * (y_sum_tmp_d9509_197[5])))
                        + ((x_sum_tmp_d9509_196[6]) * (y_sum_tmp_d9509_197[4]))),
                    (((x_sum_tmp_d9509_196[5]) * (y_sum_tmp_d9509_197[6]))
                        + ((x_sum_tmp_d9509_196[6]) * (y_sum_tmp_d9509_197[5]))),
                    ((x_sum_tmp_d9509_196[6]) * (y_sum_tmp_d9509_197[6])),
                ];
                let z2_tmp_d9509_199 = [
                    ((x_sum_tmp_d9509_196[7]) * (y_sum_tmp_d9509_197[7])),
                    (((x_sum_tmp_d9509_196[7]) * (y_sum_tmp_d9509_197[8]))
                        + ((x_sum_tmp_d9509_196[8]) * (y_sum_tmp_d9509_197[7]))),
                    ((((x_sum_tmp_d9509_196[7]) * (y_sum_tmp_d9509_197[9]))
                        + ((x_sum_tmp_d9509_196[8]) * (y_sum_tmp_d9509_197[8])))
                        + ((x_sum_tmp_d9509_196[9]) * (y_sum_tmp_d9509_197[7]))),
                    (((((x_sum_tmp_d9509_196[7]) * (y_sum_tmp_d9509_197[10]))
                        + ((x_sum_tmp_d9509_196[8]) * (y_sum_tmp_d9509_197[9])))
                        + ((x_sum_tmp_d9509_196[9]) * (y_sum_tmp_d9509_197[8])))
                        + ((x_sum_tmp_d9509_196[10]) * (y_sum_tmp_d9509_197[7]))),
                    ((((((x_sum_tmp_d9509_196[7]) * (y_sum_tmp_d9509_197[11]))
                        + ((x_sum_tmp_d9509_196[8]) * (y_sum_tmp_d9509_197[10])))
                        + ((x_sum_tmp_d9509_196[9]) * (y_sum_tmp_d9509_197[9])))
                        + ((x_sum_tmp_d9509_196[10]) * (y_sum_tmp_d9509_197[8])))
                        + ((x_sum_tmp_d9509_196[11]) * (y_sum_tmp_d9509_197[7]))),
                    (((((((x_sum_tmp_d9509_196[7]) * (y_sum_tmp_d9509_197[12]))
                        + ((x_sum_tmp_d9509_196[8]) * (y_sum_tmp_d9509_197[11])))
                        + ((x_sum_tmp_d9509_196[9]) * (y_sum_tmp_d9509_197[10])))
                        + ((x_sum_tmp_d9509_196[10]) * (y_sum_tmp_d9509_197[9])))
                        + ((x_sum_tmp_d9509_196[11]) * (y_sum_tmp_d9509_197[8])))
                        + ((x_sum_tmp_d9509_196[12]) * (y_sum_tmp_d9509_197[7]))),
                    ((((((((x_sum_tmp_d9509_196[7]) * (y_sum_tmp_d9509_197[13]))
                        + ((x_sum_tmp_d9509_196[8]) * (y_sum_tmp_d9509_197[12])))
                        + ((x_sum_tmp_d9509_196[9]) * (y_sum_tmp_d9509_197[11])))
                        + ((x_sum_tmp_d9509_196[10]) * (y_sum_tmp_d9509_197[10])))
                        + ((x_sum_tmp_d9509_196[11]) * (y_sum_tmp_d9509_197[9])))
                        + ((x_sum_tmp_d9509_196[12]) * (y_sum_tmp_d9509_197[8])))
                        + ((x_sum_tmp_d9509_196[13]) * (y_sum_tmp_d9509_197[7]))),
                    (((((((x_sum_tmp_d9509_196[8]) * (y_sum_tmp_d9509_197[13]))
                        + ((x_sum_tmp_d9509_196[9]) * (y_sum_tmp_d9509_197[12])))
                        + ((x_sum_tmp_d9509_196[10]) * (y_sum_tmp_d9509_197[11])))
                        + ((x_sum_tmp_d9509_196[11]) * (y_sum_tmp_d9509_197[10])))
                        + ((x_sum_tmp_d9509_196[12]) * (y_sum_tmp_d9509_197[9])))
                        + ((x_sum_tmp_d9509_196[13]) * (y_sum_tmp_d9509_197[8]))),
                    ((((((x_sum_tmp_d9509_196[9]) * (y_sum_tmp_d9509_197[13]))
                        + ((x_sum_tmp_d9509_196[10]) * (y_sum_tmp_d9509_197[12])))
                        + ((x_sum_tmp_d9509_196[11]) * (y_sum_tmp_d9509_197[11])))
                        + ((x_sum_tmp_d9509_196[12]) * (y_sum_tmp_d9509_197[10])))
                        + ((x_sum_tmp_d9509_196[13]) * (y_sum_tmp_d9509_197[9]))),
                    (((((x_sum_tmp_d9509_196[10]) * (y_sum_tmp_d9509_197[13]))
                        + ((x_sum_tmp_d9509_196[11]) * (y_sum_tmp_d9509_197[12])))
                        + ((x_sum_tmp_d9509_196[12]) * (y_sum_tmp_d9509_197[11])))
                        + ((x_sum_tmp_d9509_196[13]) * (y_sum_tmp_d9509_197[10]))),
                    ((((x_sum_tmp_d9509_196[11]) * (y_sum_tmp_d9509_197[13]))
                        + ((x_sum_tmp_d9509_196[12]) * (y_sum_tmp_d9509_197[12])))
                        + ((x_sum_tmp_d9509_196[13]) * (y_sum_tmp_d9509_197[11]))),
                    (((x_sum_tmp_d9509_196[12]) * (y_sum_tmp_d9509_197[13]))
                        + ((x_sum_tmp_d9509_196[13]) * (y_sum_tmp_d9509_197[12]))),
                    ((x_sum_tmp_d9509_196[13]) * (y_sum_tmp_d9509_197[13])),
                ];
                let x_sum_tmp_d9509_200 = [
                    ((x_sum_tmp_d9509_196[0]) + (x_sum_tmp_d9509_196[7])),
                    ((x_sum_tmp_d9509_196[1]) + (x_sum_tmp_d9509_196[8])),
                    ((x_sum_tmp_d9509_196[2]) + (x_sum_tmp_d9509_196[9])),
                    ((x_sum_tmp_d9509_196[3]) + (x_sum_tmp_d9509_196[10])),
                    ((x_sum_tmp_d9509_196[4]) + (x_sum_tmp_d9509_196[11])),
                    ((x_sum_tmp_d9509_196[5]) + (x_sum_tmp_d9509_196[12])),
                    ((x_sum_tmp_d9509_196[6]) + (x_sum_tmp_d9509_196[13])),
                ];
                let y_sum_tmp_d9509_201 = [
                    ((y_sum_tmp_d9509_197[0]) + (y_sum_tmp_d9509_197[7])),
                    ((y_sum_tmp_d9509_197[1]) + (y_sum_tmp_d9509_197[8])),
                    ((y_sum_tmp_d9509_197[2]) + (y_sum_tmp_d9509_197[9])),
                    ((y_sum_tmp_d9509_197[3]) + (y_sum_tmp_d9509_197[10])),
                    ((y_sum_tmp_d9509_197[4]) + (y_sum_tmp_d9509_197[11])),
                    ((y_sum_tmp_d9509_197[5]) + (y_sum_tmp_d9509_197[12])),
                    ((y_sum_tmp_d9509_197[6]) + (y_sum_tmp_d9509_197[13])),
                ];
                let single_karatsuba_n_7_output_tmp_d9509_202 = [
                    z0_tmp_d9509_198[0],
                    z0_tmp_d9509_198[1],
                    z0_tmp_d9509_198[2],
                    z0_tmp_d9509_198[3],
                    z0_tmp_d9509_198[4],
                    z0_tmp_d9509_198[5],
                    z0_tmp_d9509_198[6],
                    ((z0_tmp_d9509_198[7])
                        + ((((x_sum_tmp_d9509_200[0]) * (y_sum_tmp_d9509_201[0]))
                            - (z0_tmp_d9509_198[0]))
                            - (z2_tmp_d9509_199[0]))),
                    ((z0_tmp_d9509_198[8])
                        + (((((x_sum_tmp_d9509_200[0]) * (y_sum_tmp_d9509_201[1]))
                            + ((x_sum_tmp_d9509_200[1]) * (y_sum_tmp_d9509_201[0])))
                            - (z0_tmp_d9509_198[1]))
                            - (z2_tmp_d9509_199[1]))),
                    ((z0_tmp_d9509_198[9])
                        + ((((((x_sum_tmp_d9509_200[0]) * (y_sum_tmp_d9509_201[2]))
                            + ((x_sum_tmp_d9509_200[1]) * (y_sum_tmp_d9509_201[1])))
                            + ((x_sum_tmp_d9509_200[2]) * (y_sum_tmp_d9509_201[0])))
                            - (z0_tmp_d9509_198[2]))
                            - (z2_tmp_d9509_199[2]))),
                    ((z0_tmp_d9509_198[10])
                        + (((((((x_sum_tmp_d9509_200[0]) * (y_sum_tmp_d9509_201[3]))
                            + ((x_sum_tmp_d9509_200[1]) * (y_sum_tmp_d9509_201[2])))
                            + ((x_sum_tmp_d9509_200[2]) * (y_sum_tmp_d9509_201[1])))
                            + ((x_sum_tmp_d9509_200[3]) * (y_sum_tmp_d9509_201[0])))
                            - (z0_tmp_d9509_198[3]))
                            - (z2_tmp_d9509_199[3]))),
                    ((z0_tmp_d9509_198[11])
                        + ((((((((x_sum_tmp_d9509_200[0]) * (y_sum_tmp_d9509_201[4]))
                            + ((x_sum_tmp_d9509_200[1]) * (y_sum_tmp_d9509_201[3])))
                            + ((x_sum_tmp_d9509_200[2]) * (y_sum_tmp_d9509_201[2])))
                            + ((x_sum_tmp_d9509_200[3]) * (y_sum_tmp_d9509_201[1])))
                            + ((x_sum_tmp_d9509_200[4]) * (y_sum_tmp_d9509_201[0])))
                            - (z0_tmp_d9509_198[4]))
                            - (z2_tmp_d9509_199[4]))),
                    ((z0_tmp_d9509_198[12])
                        + (((((((((x_sum_tmp_d9509_200[0]) * (y_sum_tmp_d9509_201[5]))
                            + ((x_sum_tmp_d9509_200[1]) * (y_sum_tmp_d9509_201[4])))
                            + ((x_sum_tmp_d9509_200[2]) * (y_sum_tmp_d9509_201[3])))
                            + ((x_sum_tmp_d9509_200[3]) * (y_sum_tmp_d9509_201[2])))
                            + ((x_sum_tmp_d9509_200[4]) * (y_sum_tmp_d9509_201[1])))
                            + ((x_sum_tmp_d9509_200[5]) * (y_sum_tmp_d9509_201[0])))
                            - (z0_tmp_d9509_198[5]))
                            - (z2_tmp_d9509_199[5]))),
                    ((((((((((x_sum_tmp_d9509_200[0]) * (y_sum_tmp_d9509_201[6]))
                        + ((x_sum_tmp_d9509_200[1]) * (y_sum_tmp_d9509_201[5])))
                        + ((x_sum_tmp_d9509_200[2]) * (y_sum_tmp_d9509_201[4])))
                        + ((x_sum_tmp_d9509_200[3]) * (y_sum_tmp_d9509_201[3])))
                        + ((x_sum_tmp_d9509_200[4]) * (y_sum_tmp_d9509_201[2])))
                        + ((x_sum_tmp_d9509_200[5]) * (y_sum_tmp_d9509_201[1])))
                        + ((x_sum_tmp_d9509_200[6]) * (y_sum_tmp_d9509_201[0])))
                        - (z0_tmp_d9509_198[6]))
                        - (z2_tmp_d9509_199[6])),
                    ((z2_tmp_d9509_199[0])
                        + (((((((((x_sum_tmp_d9509_200[1]) * (y_sum_tmp_d9509_201[6]))
                            + ((x_sum_tmp_d9509_200[2]) * (y_sum_tmp_d9509_201[5])))
                            + ((x_sum_tmp_d9509_200[3]) * (y_sum_tmp_d9509_201[4])))
                            + ((x_sum_tmp_d9509_200[4]) * (y_sum_tmp_d9509_201[3])))
                            + ((x_sum_tmp_d9509_200[5]) * (y_sum_tmp_d9509_201[2])))
                            + ((x_sum_tmp_d9509_200[6]) * (y_sum_tmp_d9509_201[1])))
                            - (z0_tmp_d9509_198[7]))
                            - (z2_tmp_d9509_199[7]))),
                    ((z2_tmp_d9509_199[1])
                        + ((((((((x_sum_tmp_d9509_200[2]) * (y_sum_tmp_d9509_201[6]))
                            + ((x_sum_tmp_d9509_200[3]) * (y_sum_tmp_d9509_201[5])))
                            + ((x_sum_tmp_d9509_200[4]) * (y_sum_tmp_d9509_201[4])))
                            + ((x_sum_tmp_d9509_200[5]) * (y_sum_tmp_d9509_201[3])))
                            + ((x_sum_tmp_d9509_200[6]) * (y_sum_tmp_d9509_201[2])))
                            - (z0_tmp_d9509_198[8]))
                            - (z2_tmp_d9509_199[8]))),
                    ((z2_tmp_d9509_199[2])
                        + (((((((x_sum_tmp_d9509_200[3]) * (y_sum_tmp_d9509_201[6]))
                            + ((x_sum_tmp_d9509_200[4]) * (y_sum_tmp_d9509_201[5])))
                            + ((x_sum_tmp_d9509_200[5]) * (y_sum_tmp_d9509_201[4])))
                            + ((x_sum_tmp_d9509_200[6]) * (y_sum_tmp_d9509_201[3])))
                            - (z0_tmp_d9509_198[9]))
                            - (z2_tmp_d9509_199[9]))),
                    ((z2_tmp_d9509_199[3])
                        + ((((((x_sum_tmp_d9509_200[4]) * (y_sum_tmp_d9509_201[6]))
                            + ((x_sum_tmp_d9509_200[5]) * (y_sum_tmp_d9509_201[5])))
                            + ((x_sum_tmp_d9509_200[6]) * (y_sum_tmp_d9509_201[4])))
                            - (z0_tmp_d9509_198[10]))
                            - (z2_tmp_d9509_199[10]))),
                    ((z2_tmp_d9509_199[4])
                        + (((((x_sum_tmp_d9509_200[5]) * (y_sum_tmp_d9509_201[6]))
                            + ((x_sum_tmp_d9509_200[6]) * (y_sum_tmp_d9509_201[5])))
                            - (z0_tmp_d9509_198[11]))
                            - (z2_tmp_d9509_199[11]))),
                    ((z2_tmp_d9509_199[5])
                        + ((((x_sum_tmp_d9509_200[6]) * (y_sum_tmp_d9509_201[6]))
                            - (z0_tmp_d9509_198[12]))
                            - (z2_tmp_d9509_199[12]))),
                    z2_tmp_d9509_199[6],
                    z2_tmp_d9509_199[7],
                    z2_tmp_d9509_199[8],
                    z2_tmp_d9509_199[9],
                    z2_tmp_d9509_199[10],
                    z2_tmp_d9509_199[11],
                    z2_tmp_d9509_199[12],
                ];

                let double_karatsuba_f0fc6_output_tmp_d9509_203 = [
                    single_karatsuba_n_7_output_tmp_d9509_190[0],
                    single_karatsuba_n_7_output_tmp_d9509_190[1],
                    single_karatsuba_n_7_output_tmp_d9509_190[2],
                    single_karatsuba_n_7_output_tmp_d9509_190[3],
                    single_karatsuba_n_7_output_tmp_d9509_190[4],
                    single_karatsuba_n_7_output_tmp_d9509_190[5],
                    single_karatsuba_n_7_output_tmp_d9509_190[6],
                    single_karatsuba_n_7_output_tmp_d9509_190[7],
                    single_karatsuba_n_7_output_tmp_d9509_190[8],
                    single_karatsuba_n_7_output_tmp_d9509_190[9],
                    single_karatsuba_n_7_output_tmp_d9509_190[10],
                    single_karatsuba_n_7_output_tmp_d9509_190[11],
                    single_karatsuba_n_7_output_tmp_d9509_190[12],
                    single_karatsuba_n_7_output_tmp_d9509_190[13],
                    ((single_karatsuba_n_7_output_tmp_d9509_190[14])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[0])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[0]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[0]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_190[15])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[1])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[1]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[1]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_190[16])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[2])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[2]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[2]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_190[17])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[3])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[3]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[3]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_190[18])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[4])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[4]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[4]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_190[19])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[5])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[5]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[5]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_190[20])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[6])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[6]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[6]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_190[21])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[7])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[7]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[7]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_190[22])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[8])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[8]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[8]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_190[23])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[9])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[9]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[9]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_190[24])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[10])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[10]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[10]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_190[25])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[11])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[11]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[11]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_190[26])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[12])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[12]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[12]))),
                    (((single_karatsuba_n_7_output_tmp_d9509_202[13])
                        - (single_karatsuba_n_7_output_tmp_d9509_190[13]))
                        - (single_karatsuba_n_7_output_tmp_d9509_195[13])),
                    ((single_karatsuba_n_7_output_tmp_d9509_195[0])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[14])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[14]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[14]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_195[1])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[15])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[15]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[15]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_195[2])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[16])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[16]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[16]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_195[3])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[17])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[17]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[17]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_195[4])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[18])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[18]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[18]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_195[5])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[19])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[19]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[19]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_195[6])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[20])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[20]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[20]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_195[7])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[21])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[21]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[21]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_195[8])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[22])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[22]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[22]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_195[9])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[23])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[23]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[23]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_195[10])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[24])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[24]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[24]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_195[11])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[25])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[25]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[25]))),
                    ((single_karatsuba_n_7_output_tmp_d9509_195[12])
                        + (((single_karatsuba_n_7_output_tmp_d9509_202[26])
                            - (single_karatsuba_n_7_output_tmp_d9509_190[26]))
                            - (single_karatsuba_n_7_output_tmp_d9509_195[26]))),
                    single_karatsuba_n_7_output_tmp_d9509_195[13],
                    single_karatsuba_n_7_output_tmp_d9509_195[14],
                    single_karatsuba_n_7_output_tmp_d9509_195[15],
                    single_karatsuba_n_7_output_tmp_d9509_195[16],
                    single_karatsuba_n_7_output_tmp_d9509_195[17],
                    single_karatsuba_n_7_output_tmp_d9509_195[18],
                    single_karatsuba_n_7_output_tmp_d9509_195[19],
                    single_karatsuba_n_7_output_tmp_d9509_195[20],
                    single_karatsuba_n_7_output_tmp_d9509_195[21],
                    single_karatsuba_n_7_output_tmp_d9509_195[22],
                    single_karatsuba_n_7_output_tmp_d9509_195[23],
                    single_karatsuba_n_7_output_tmp_d9509_195[24],
                    single_karatsuba_n_7_output_tmp_d9509_195[25],
                    single_karatsuba_n_7_output_tmp_d9509_195[26],
                ];

                let conv_tmp_d9509_204 = [
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[0]) - (y_sum_0_tmp_d9509_158)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[1]) - (y_sum_1_tmp_d9509_159)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[2]) - (y_sum_2_tmp_d9509_160)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[3]) - (y_sum_3_tmp_d9509_161)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[4]) - (y_sum_4_tmp_d9509_162)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[5]) - (y_sum_5_tmp_d9509_163)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[6]) - (y_sum_6_tmp_d9509_164)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[7]) - (y_sum_7_tmp_d9509_165)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[8]) - (y_sum_8_tmp_d9509_166)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[9]) - (y_sum_9_tmp_d9509_167)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[10]) - (y_sum_10_tmp_d9509_168)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[11]) - (y_sum_11_tmp_d9509_169)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[12]) - (y_sum_12_tmp_d9509_170)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[13]) - (y_sum_13_tmp_d9509_171)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[14]) - (y_sum_14_tmp_d9509_172)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[15]) - (y_sum_15_tmp_d9509_173)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[16]) - (y_sum_16_tmp_d9509_174)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[17]) - (y_sum_17_tmp_d9509_175)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[18]) - (y_sum_18_tmp_d9509_176)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[19]) - (y_sum_19_tmp_d9509_177)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[20]) - (y_sum_20_tmp_d9509_178)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[21]) - (y_sum_21_tmp_d9509_179)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[22]) - (y_sum_22_tmp_d9509_180)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[23]) - (y_sum_23_tmp_d9509_181)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[24]) - (y_sum_24_tmp_d9509_182)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[25]) - (y_sum_25_tmp_d9509_183)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[26]) - (y_sum_26_tmp_d9509_184)),
                    ((double_karatsuba_f0fc6_output_tmp_d9509_203[27]) - (y_sum_27_tmp_d9509_185)),
                    double_karatsuba_f0fc6_output_tmp_d9509_203[28],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[29],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[30],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[31],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[32],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[33],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[34],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[35],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[36],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[37],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[38],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[39],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[40],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[41],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[42],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[43],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[44],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[45],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[46],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[47],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[48],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[49],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[50],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[51],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[52],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[53],
                    double_karatsuba_f0fc6_output_tmp_d9509_203[54],
                ];
                let conv_mod_tmp_d9509_205 = [
                    ((((M31_32) * (conv_tmp_d9509_204[0])) - ((M31_4) * (conv_tmp_d9509_204[21])))
                        + ((M31_8) * (conv_tmp_d9509_204[49]))),
                    ((((conv_tmp_d9509_204[0]) + ((M31_32) * (conv_tmp_d9509_204[1])))
                        - ((M31_4) * (conv_tmp_d9509_204[22])))
                        + ((M31_8) * (conv_tmp_d9509_204[50]))),
                    ((((conv_tmp_d9509_204[1]) + ((M31_32) * (conv_tmp_d9509_204[2])))
                        - ((M31_4) * (conv_tmp_d9509_204[23])))
                        + ((M31_8) * (conv_tmp_d9509_204[51]))),
                    ((((conv_tmp_d9509_204[2]) + ((M31_32) * (conv_tmp_d9509_204[3])))
                        - ((M31_4) * (conv_tmp_d9509_204[24])))
                        + ((M31_8) * (conv_tmp_d9509_204[52]))),
                    ((((conv_tmp_d9509_204[3]) + ((M31_32) * (conv_tmp_d9509_204[4])))
                        - ((M31_4) * (conv_tmp_d9509_204[25])))
                        + ((M31_8) * (conv_tmp_d9509_204[53]))),
                    ((((conv_tmp_d9509_204[4]) + ((M31_32) * (conv_tmp_d9509_204[5])))
                        - ((M31_4) * (conv_tmp_d9509_204[26])))
                        + ((M31_8) * (conv_tmp_d9509_204[54]))),
                    (((conv_tmp_d9509_204[5]) + ((M31_32) * (conv_tmp_d9509_204[6])))
                        - ((M31_4) * (conv_tmp_d9509_204[27]))),
                    (((((M31_2) * (conv_tmp_d9509_204[0])) + (conv_tmp_d9509_204[6]))
                        + ((M31_32) * (conv_tmp_d9509_204[7])))
                        - ((M31_4) * (conv_tmp_d9509_204[28]))),
                    (((((M31_2) * (conv_tmp_d9509_204[1])) + (conv_tmp_d9509_204[7]))
                        + ((M31_32) * (conv_tmp_d9509_204[8])))
                        - ((M31_4) * (conv_tmp_d9509_204[29]))),
                    (((((M31_2) * (conv_tmp_d9509_204[2])) + (conv_tmp_d9509_204[8]))
                        + ((M31_32) * (conv_tmp_d9509_204[9])))
                        - ((M31_4) * (conv_tmp_d9509_204[30]))),
                    (((((M31_2) * (conv_tmp_d9509_204[3])) + (conv_tmp_d9509_204[9]))
                        + ((M31_32) * (conv_tmp_d9509_204[10])))
                        - ((M31_4) * (conv_tmp_d9509_204[31]))),
                    (((((M31_2) * (conv_tmp_d9509_204[4])) + (conv_tmp_d9509_204[10]))
                        + ((M31_32) * (conv_tmp_d9509_204[11])))
                        - ((M31_4) * (conv_tmp_d9509_204[32]))),
                    (((((M31_2) * (conv_tmp_d9509_204[5])) + (conv_tmp_d9509_204[11]))
                        + ((M31_32) * (conv_tmp_d9509_204[12])))
                        - ((M31_4) * (conv_tmp_d9509_204[33]))),
                    (((((M31_2) * (conv_tmp_d9509_204[6])) + (conv_tmp_d9509_204[12]))
                        + ((M31_32) * (conv_tmp_d9509_204[13])))
                        - ((M31_4) * (conv_tmp_d9509_204[34]))),
                    (((((M31_2) * (conv_tmp_d9509_204[7])) + (conv_tmp_d9509_204[13]))
                        + ((M31_32) * (conv_tmp_d9509_204[14])))
                        - ((M31_4) * (conv_tmp_d9509_204[35]))),
                    (((((M31_2) * (conv_tmp_d9509_204[8])) + (conv_tmp_d9509_204[14]))
                        + ((M31_32) * (conv_tmp_d9509_204[15])))
                        - ((M31_4) * (conv_tmp_d9509_204[36]))),
                    (((((M31_2) * (conv_tmp_d9509_204[9])) + (conv_tmp_d9509_204[15]))
                        + ((M31_32) * (conv_tmp_d9509_204[16])))
                        - ((M31_4) * (conv_tmp_d9509_204[37]))),
                    (((((M31_2) * (conv_tmp_d9509_204[10])) + (conv_tmp_d9509_204[16]))
                        + ((M31_32) * (conv_tmp_d9509_204[17])))
                        - ((M31_4) * (conv_tmp_d9509_204[38]))),
                    (((((M31_2) * (conv_tmp_d9509_204[11])) + (conv_tmp_d9509_204[17]))
                        + ((M31_32) * (conv_tmp_d9509_204[18])))
                        - ((M31_4) * (conv_tmp_d9509_204[39]))),
                    (((((M31_2) * (conv_tmp_d9509_204[12])) + (conv_tmp_d9509_204[18]))
                        + ((M31_32) * (conv_tmp_d9509_204[19])))
                        - ((M31_4) * (conv_tmp_d9509_204[40]))),
                    (((((M31_2) * (conv_tmp_d9509_204[13])) + (conv_tmp_d9509_204[19]))
                        + ((M31_32) * (conv_tmp_d9509_204[20])))
                        - ((M31_4) * (conv_tmp_d9509_204[41]))),
                    (((((M31_2) * (conv_tmp_d9509_204[14])) + (conv_tmp_d9509_204[20]))
                        - ((M31_4) * (conv_tmp_d9509_204[42])))
                        + ((M31_64) * (conv_tmp_d9509_204[49]))),
                    (((((M31_2) * (conv_tmp_d9509_204[15]))
                        - ((M31_4) * (conv_tmp_d9509_204[43])))
                        + ((M31_2) * (conv_tmp_d9509_204[49])))
                        + ((M31_64) * (conv_tmp_d9509_204[50]))),
                    (((((M31_2) * (conv_tmp_d9509_204[16]))
                        - ((M31_4) * (conv_tmp_d9509_204[44])))
                        + ((M31_2) * (conv_tmp_d9509_204[50])))
                        + ((M31_64) * (conv_tmp_d9509_204[51]))),
                    (((((M31_2) * (conv_tmp_d9509_204[17]))
                        - ((M31_4) * (conv_tmp_d9509_204[45])))
                        + ((M31_2) * (conv_tmp_d9509_204[51])))
                        + ((M31_64) * (conv_tmp_d9509_204[52]))),
                    (((((M31_2) * (conv_tmp_d9509_204[18]))
                        - ((M31_4) * (conv_tmp_d9509_204[46])))
                        + ((M31_2) * (conv_tmp_d9509_204[52])))
                        + ((M31_64) * (conv_tmp_d9509_204[53]))),
                    (((((M31_2) * (conv_tmp_d9509_204[19]))
                        - ((M31_4) * (conv_tmp_d9509_204[47])))
                        + ((M31_2) * (conv_tmp_d9509_204[53])))
                        + ((M31_64) * (conv_tmp_d9509_204[54]))),
                    ((((M31_2) * (conv_tmp_d9509_204[20])) - ((M31_4) * (conv_tmp_d9509_204[48])))
                        + ((M31_2) * (conv_tmp_d9509_204[54]))),
                ];
                let k_mod_2_18_biased_tmp_d9509_206 =
                    ((((PackedUInt32::from_m31(((conv_mod_tmp_d9509_205[0]) + (M31_134217728))))
                        + (((PackedUInt32::from_m31(
                            ((conv_mod_tmp_d9509_205[1]) + (M31_134217728)),
                        )) & (UInt32_511))
                            << (UInt32_9)))
                        + (UInt32_131072))
                        & (UInt32_262143));
                let k_col269 = ((k_mod_2_18_biased_tmp_d9509_206.low().as_m31())
                    + (((k_mod_2_18_biased_tmp_d9509_206.high().as_m31()) - (M31_2))
                        * (M31_65536)));
                *row[269] = k_col269;
                *sub_component_inputs.range_check_20[8] = [((k_col269) + (M31_524288))];
                *lookup_data.range_check_20_99 = [M31_1410849886, ((k_col269) + (M31_524288))];
                let carry_0_col270 = (((conv_mod_tmp_d9509_205[0]) - (k_col269)) * (M31_4194304));
                *row[270] = carry_0_col270;
                *sub_component_inputs.range_check_20_b[8] = [((carry_0_col270) + (M31_524288))];
                *lookup_data.range_check_20_b_100 =
                    [M31_514232941, ((carry_0_col270) + (M31_524288))];
                let carry_1_col271 =
                    (((conv_mod_tmp_d9509_205[1]) + (carry_0_col270)) * (M31_4194304));
                *row[271] = carry_1_col271;
                *sub_component_inputs.range_check_20_c[8] = [((carry_1_col271) + (M31_524288))];
                *lookup_data.range_check_20_c_101 =
                    [M31_531010560, ((carry_1_col271) + (M31_524288))];
                let carry_2_col272 =
                    (((conv_mod_tmp_d9509_205[2]) + (carry_1_col271)) * (M31_4194304));
                *row[272] = carry_2_col272;
                *sub_component_inputs.range_check_20_d[8] = [((carry_2_col272) + (M31_524288))];
                *lookup_data.range_check_20_d_102 =
                    [M31_480677703, ((carry_2_col272) + (M31_524288))];
                let carry_3_col273 =
                    (((conv_mod_tmp_d9509_205[3]) + (carry_2_col272)) * (M31_4194304));
                *row[273] = carry_3_col273;
                *sub_component_inputs.range_check_20_e[6] = [((carry_3_col273) + (M31_524288))];
                *lookup_data.range_check_20_e_103 =
                    [M31_497455322, ((carry_3_col273) + (M31_524288))];
                let carry_4_col274 =
                    (((conv_mod_tmp_d9509_205[4]) + (carry_3_col273)) * (M31_4194304));
                *row[274] = carry_4_col274;
                *sub_component_inputs.range_check_20_f[6] = [((carry_4_col274) + (M31_524288))];
                *lookup_data.range_check_20_f_104 =
                    [M31_447122465, ((carry_4_col274) + (M31_524288))];
                let carry_5_col275 =
                    (((conv_mod_tmp_d9509_205[5]) + (carry_4_col274)) * (M31_4194304));
                *row[275] = carry_5_col275;
                *sub_component_inputs.range_check_20_g[6] = [((carry_5_col275) + (M31_524288))];
                *lookup_data.range_check_20_g_105 =
                    [M31_463900084, ((carry_5_col275) + (M31_524288))];
                let carry_6_col276 =
                    (((conv_mod_tmp_d9509_205[6]) + (carry_5_col275)) * (M31_4194304));
                *row[276] = carry_6_col276;
                *sub_component_inputs.range_check_20_h[6] = [((carry_6_col276) + (M31_524288))];
                *lookup_data.range_check_20_h_106 =
                    [M31_682009131, ((carry_6_col276) + (M31_524288))];
                let carry_7_col277 =
                    (((conv_mod_tmp_d9509_205[7]) + (carry_6_col276)) * (M31_4194304));
                *row[277] = carry_7_col277;
                *sub_component_inputs.range_check_20[9] = [((carry_7_col277) + (M31_524288))];
                *lookup_data.range_check_20_107 =
                    [M31_1410849886, ((carry_7_col277) + (M31_524288))];
                let carry_8_col278 =
                    (((conv_mod_tmp_d9509_205[8]) + (carry_7_col277)) * (M31_4194304));
                *row[278] = carry_8_col278;
                *sub_component_inputs.range_check_20_b[9] = [((carry_8_col278) + (M31_524288))];
                *lookup_data.range_check_20_b_108 =
                    [M31_514232941, ((carry_8_col278) + (M31_524288))];
                let carry_9_col279 =
                    (((conv_mod_tmp_d9509_205[9]) + (carry_8_col278)) * (M31_4194304));
                *row[279] = carry_9_col279;
                *sub_component_inputs.range_check_20_c[9] = [((carry_9_col279) + (M31_524288))];
                *lookup_data.range_check_20_c_109 =
                    [M31_531010560, ((carry_9_col279) + (M31_524288))];
                let carry_10_col280 =
                    (((conv_mod_tmp_d9509_205[10]) + (carry_9_col279)) * (M31_4194304));
                *row[280] = carry_10_col280;
                *sub_component_inputs.range_check_20_d[9] = [((carry_10_col280) + (M31_524288))];
                *lookup_data.range_check_20_d_110 =
                    [M31_480677703, ((carry_10_col280) + (M31_524288))];
                let carry_11_col281 =
                    (((conv_mod_tmp_d9509_205[11]) + (carry_10_col280)) * (M31_4194304));
                *row[281] = carry_11_col281;
                *sub_component_inputs.range_check_20_e[7] = [((carry_11_col281) + (M31_524288))];
                *lookup_data.range_check_20_e_111 =
                    [M31_497455322, ((carry_11_col281) + (M31_524288))];
                let carry_12_col282 =
                    (((conv_mod_tmp_d9509_205[12]) + (carry_11_col281)) * (M31_4194304));
                *row[282] = carry_12_col282;
                *sub_component_inputs.range_check_20_f[7] = [((carry_12_col282) + (M31_524288))];
                *lookup_data.range_check_20_f_112 =
                    [M31_447122465, ((carry_12_col282) + (M31_524288))];
                let carry_13_col283 =
                    (((conv_mod_tmp_d9509_205[13]) + (carry_12_col282)) * (M31_4194304));
                *row[283] = carry_13_col283;
                *sub_component_inputs.range_check_20_g[7] = [((carry_13_col283) + (M31_524288))];
                *lookup_data.range_check_20_g_113 =
                    [M31_463900084, ((carry_13_col283) + (M31_524288))];
                let carry_14_col284 =
                    (((conv_mod_tmp_d9509_205[14]) + (carry_13_col283)) * (M31_4194304));
                *row[284] = carry_14_col284;
                *sub_component_inputs.range_check_20_h[7] = [((carry_14_col284) + (M31_524288))];
                *lookup_data.range_check_20_h_114 =
                    [M31_682009131, ((carry_14_col284) + (M31_524288))];
                let carry_15_col285 =
                    (((conv_mod_tmp_d9509_205[15]) + (carry_14_col284)) * (M31_4194304));
                *row[285] = carry_15_col285;
                *sub_component_inputs.range_check_20[10] = [((carry_15_col285) + (M31_524288))];
                *lookup_data.range_check_20_115 =
                    [M31_1410849886, ((carry_15_col285) + (M31_524288))];
                let carry_16_col286 =
                    (((conv_mod_tmp_d9509_205[16]) + (carry_15_col285)) * (M31_4194304));
                *row[286] = carry_16_col286;
                *sub_component_inputs.range_check_20_b[10] = [((carry_16_col286) + (M31_524288))];
                *lookup_data.range_check_20_b_116 =
                    [M31_514232941, ((carry_16_col286) + (M31_524288))];
                let carry_17_col287 =
                    (((conv_mod_tmp_d9509_205[17]) + (carry_16_col286)) * (M31_4194304));
                *row[287] = carry_17_col287;
                *sub_component_inputs.range_check_20_c[10] = [((carry_17_col287) + (M31_524288))];
                *lookup_data.range_check_20_c_117 =
                    [M31_531010560, ((carry_17_col287) + (M31_524288))];
                let carry_18_col288 =
                    (((conv_mod_tmp_d9509_205[18]) + (carry_17_col287)) * (M31_4194304));
                *row[288] = carry_18_col288;
                *sub_component_inputs.range_check_20_d[10] = [((carry_18_col288) + (M31_524288))];
                *lookup_data.range_check_20_d_118 =
                    [M31_480677703, ((carry_18_col288) + (M31_524288))];
                let carry_19_col289 =
                    (((conv_mod_tmp_d9509_205[19]) + (carry_18_col288)) * (M31_4194304));
                *row[289] = carry_19_col289;
                *sub_component_inputs.range_check_20_e[8] = [((carry_19_col289) + (M31_524288))];
                *lookup_data.range_check_20_e_119 =
                    [M31_497455322, ((carry_19_col289) + (M31_524288))];
                let carry_20_col290 =
                    (((conv_mod_tmp_d9509_205[20]) + (carry_19_col289)) * (M31_4194304));
                *row[290] = carry_20_col290;
                *sub_component_inputs.range_check_20_f[8] = [((carry_20_col290) + (M31_524288))];
                *lookup_data.range_check_20_f_120 =
                    [M31_447122465, ((carry_20_col290) + (M31_524288))];
                let carry_21_col291 = ((((conv_mod_tmp_d9509_205[21]) - ((M31_136) * (k_col269)))
                    + (carry_20_col290))
                    * (M31_4194304));
                *row[291] = carry_21_col291;
                *sub_component_inputs.range_check_20_g[8] = [((carry_21_col291) + (M31_524288))];
                *lookup_data.range_check_20_g_121 =
                    [M31_463900084, ((carry_21_col291) + (M31_524288))];
                let carry_22_col292 =
                    (((conv_mod_tmp_d9509_205[22]) + (carry_21_col291)) * (M31_4194304));
                *row[292] = carry_22_col292;
                *sub_component_inputs.range_check_20_h[8] = [((carry_22_col292) + (M31_524288))];
                *lookup_data.range_check_20_h_122 =
                    [M31_682009131, ((carry_22_col292) + (M31_524288))];
                let carry_23_col293 =
                    (((conv_mod_tmp_d9509_205[23]) + (carry_22_col292)) * (M31_4194304));
                *row[293] = carry_23_col293;
                *sub_component_inputs.range_check_20[11] = [((carry_23_col293) + (M31_524288))];
                *lookup_data.range_check_20_123 =
                    [M31_1410849886, ((carry_23_col293) + (M31_524288))];
                let carry_24_col294 =
                    (((conv_mod_tmp_d9509_205[24]) + (carry_23_col293)) * (M31_4194304));
                *row[294] = carry_24_col294;
                *sub_component_inputs.range_check_20_b[11] = [((carry_24_col294) + (M31_524288))];
                *lookup_data.range_check_20_b_124 =
                    [M31_514232941, ((carry_24_col294) + (M31_524288))];
                let carry_25_col295 =
                    (((conv_mod_tmp_d9509_205[25]) + (carry_24_col294)) * (M31_4194304));
                *row[295] = carry_25_col295;
                *sub_component_inputs.range_check_20_c[11] = [((carry_25_col295) + (M31_524288))];
                *lookup_data.range_check_20_c_125 =
                    [M31_531010560, ((carry_25_col295) + (M31_524288))];
                let carry_26_col296 =
                    (((conv_mod_tmp_d9509_205[26]) + (carry_25_col295)) * (M31_4194304));
                *row[296] = carry_26_col296;
                *sub_component_inputs.range_check_20_d[11] = [((carry_26_col296) + (M31_524288))];
                *lookup_data.range_check_20_d_126 =
                    [M31_480677703, ((carry_26_col296) + (M31_524288))];

                let ec_add_output_tmp_d9509_207 = [result_x_tmp_d9509_79, result_y_tmp_d9509_129];

                *lookup_data.partial_ec_mul_window_bits_18_127 = [
                    M31_1621226978,
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
                    input_limb_10_col11,
                    input_limb_11_col12,
                    input_limb_12_col13,
                    input_limb_13_col14,
                    input_limb_14_col15,
                    input_limb_15_col16,
                    input_limb_16_col17,
                    input_limb_17_col18,
                    input_limb_18_col19,
                    input_limb_19_col20,
                    input_limb_20_col21,
                    input_limb_21_col22,
                    input_limb_22_col23,
                    input_limb_23_col24,
                    input_limb_24_col25,
                    input_limb_25_col26,
                    input_limb_26_col27,
                    input_limb_27_col28,
                    input_limb_28_col29,
                    input_limb_29_col30,
                    input_limb_30_col31,
                    input_limb_31_col32,
                    input_limb_32_col33,
                    input_limb_33_col34,
                    input_limb_34_col35,
                    input_limb_35_col36,
                    input_limb_36_col37,
                    input_limb_37_col38,
                    input_limb_38_col39,
                    input_limb_39_col40,
                    input_limb_40_col41,
                    input_limb_41_col42,
                    input_limb_42_col43,
                    input_limb_43_col44,
                    input_limb_44_col45,
                    input_limb_45_col46,
                    input_limb_46_col47,
                    input_limb_47_col48,
                    input_limb_48_col49,
                    input_limb_49_col50,
                    input_limb_50_col51,
                    input_limb_51_col52,
                    input_limb_52_col53,
                    input_limb_53_col54,
                    input_limb_54_col55,
                    input_limb_55_col56,
                    input_limb_56_col57,
                    input_limb_57_col58,
                    input_limb_58_col59,
                    input_limb_59_col60,
                    input_limb_60_col61,
                    input_limb_61_col62,
                    input_limb_62_col63,
                    input_limb_63_col64,
                    input_limb_64_col65,
                    input_limb_65_col66,
                    input_limb_66_col67,
                    input_limb_67_col68,
                    input_limb_68_col69,
                    input_limb_69_col70,
                    input_limb_70_col71,
                    input_limb_71_col72,
                ];
                *lookup_data.partial_ec_mul_window_bits_18_128 = [
                    M31_1621226978,
                    input_limb_0_col1,
                    ((input_limb_1_col2) + (M31_1)),
                    input_limb_3_col4,
                    input_limb_4_col5,
                    input_limb_5_col6,
                    input_limb_6_col7,
                    input_limb_7_col8,
                    input_limb_8_col9,
                    input_limb_9_col10,
                    input_limb_10_col11,
                    input_limb_11_col12,
                    input_limb_12_col13,
                    input_limb_13_col14,
                    input_limb_14_col15,
                    input_limb_15_col16,
                    M31_0,
                    result_x_limb_0_col185,
                    result_x_limb_1_col186,
                    result_x_limb_2_col187,
                    result_x_limb_3_col188,
                    result_x_limb_4_col189,
                    result_x_limb_5_col190,
                    result_x_limb_6_col191,
                    result_x_limb_7_col192,
                    result_x_limb_8_col193,
                    result_x_limb_9_col194,
                    result_x_limb_10_col195,
                    result_x_limb_11_col196,
                    result_x_limb_12_col197,
                    result_x_limb_13_col198,
                    result_x_limb_14_col199,
                    result_x_limb_15_col200,
                    result_x_limb_16_col201,
                    result_x_limb_17_col202,
                    result_x_limb_18_col203,
                    result_x_limb_19_col204,
                    result_x_limb_20_col205,
                    result_x_limb_21_col206,
                    result_x_limb_22_col207,
                    result_x_limb_23_col208,
                    result_x_limb_24_col209,
                    result_x_limb_25_col210,
                    result_x_limb_26_col211,
                    result_x_limb_27_col212,
                    result_y_limb_0_col241,
                    result_y_limb_1_col242,
                    result_y_limb_2_col243,
                    result_y_limb_3_col244,
                    result_y_limb_4_col245,
                    result_y_limb_5_col246,
                    result_y_limb_6_col247,
                    result_y_limb_7_col248,
                    result_y_limb_8_col249,
                    result_y_limb_9_col250,
                    result_y_limb_10_col251,
                    result_y_limb_11_col252,
                    result_y_limb_12_col253,
                    result_y_limb_13_col254,
                    result_y_limb_14_col255,
                    result_y_limb_15_col256,
                    result_y_limb_16_col257,
                    result_y_limb_17_col258,
                    result_y_limb_18_col259,
                    result_y_limb_19_col260,
                    result_y_limb_20_col261,
                    result_y_limb_21_col262,
                    result_y_limb_22_col263,
                    result_y_limb_23_col264,
                    result_y_limb_24_col265,
                    result_y_limb_25_col266,
                    result_y_limb_26_col267,
                    result_y_limb_27_col268,
                ];
                *lookup_data.mults_0 = enabler_col0;
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    pedersen_points_table_window_bits_18_0: Vec<[PackedM31; 58]>,
    range_check_9_9_1: Vec<[PackedM31; 3]>,
    range_check_9_9_b_2: Vec<[PackedM31; 3]>,
    range_check_9_9_c_3: Vec<[PackedM31; 3]>,
    range_check_9_9_d_4: Vec<[PackedM31; 3]>,
    range_check_9_9_e_5: Vec<[PackedM31; 3]>,
    range_check_9_9_f_6: Vec<[PackedM31; 3]>,
    range_check_9_9_g_7: Vec<[PackedM31; 3]>,
    range_check_9_9_h_8: Vec<[PackedM31; 3]>,
    range_check_9_9_9: Vec<[PackedM31; 3]>,
    range_check_9_9_b_10: Vec<[PackedM31; 3]>,
    range_check_9_9_c_11: Vec<[PackedM31; 3]>,
    range_check_9_9_d_12: Vec<[PackedM31; 3]>,
    range_check_9_9_e_13: Vec<[PackedM31; 3]>,
    range_check_9_9_f_14: Vec<[PackedM31; 3]>,
    range_check_20_15: Vec<[PackedM31; 2]>,
    range_check_20_b_16: Vec<[PackedM31; 2]>,
    range_check_20_c_17: Vec<[PackedM31; 2]>,
    range_check_20_d_18: Vec<[PackedM31; 2]>,
    range_check_20_e_19: Vec<[PackedM31; 2]>,
    range_check_20_f_20: Vec<[PackedM31; 2]>,
    range_check_20_g_21: Vec<[PackedM31; 2]>,
    range_check_20_h_22: Vec<[PackedM31; 2]>,
    range_check_20_23: Vec<[PackedM31; 2]>,
    range_check_20_b_24: Vec<[PackedM31; 2]>,
    range_check_20_c_25: Vec<[PackedM31; 2]>,
    range_check_20_d_26: Vec<[PackedM31; 2]>,
    range_check_20_e_27: Vec<[PackedM31; 2]>,
    range_check_20_f_28: Vec<[PackedM31; 2]>,
    range_check_20_g_29: Vec<[PackedM31; 2]>,
    range_check_20_h_30: Vec<[PackedM31; 2]>,
    range_check_20_31: Vec<[PackedM31; 2]>,
    range_check_20_b_32: Vec<[PackedM31; 2]>,
    range_check_20_c_33: Vec<[PackedM31; 2]>,
    range_check_20_d_34: Vec<[PackedM31; 2]>,
    range_check_20_e_35: Vec<[PackedM31; 2]>,
    range_check_20_f_36: Vec<[PackedM31; 2]>,
    range_check_20_g_37: Vec<[PackedM31; 2]>,
    range_check_20_h_38: Vec<[PackedM31; 2]>,
    range_check_20_39: Vec<[PackedM31; 2]>,
    range_check_20_b_40: Vec<[PackedM31; 2]>,
    range_check_20_c_41: Vec<[PackedM31; 2]>,
    range_check_20_d_42: Vec<[PackedM31; 2]>,
    range_check_9_9_43: Vec<[PackedM31; 3]>,
    range_check_9_9_b_44: Vec<[PackedM31; 3]>,
    range_check_9_9_c_45: Vec<[PackedM31; 3]>,
    range_check_9_9_d_46: Vec<[PackedM31; 3]>,
    range_check_9_9_e_47: Vec<[PackedM31; 3]>,
    range_check_9_9_f_48: Vec<[PackedM31; 3]>,
    range_check_9_9_g_49: Vec<[PackedM31; 3]>,
    range_check_9_9_h_50: Vec<[PackedM31; 3]>,
    range_check_9_9_51: Vec<[PackedM31; 3]>,
    range_check_9_9_b_52: Vec<[PackedM31; 3]>,
    range_check_9_9_c_53: Vec<[PackedM31; 3]>,
    range_check_9_9_d_54: Vec<[PackedM31; 3]>,
    range_check_9_9_e_55: Vec<[PackedM31; 3]>,
    range_check_9_9_f_56: Vec<[PackedM31; 3]>,
    range_check_20_57: Vec<[PackedM31; 2]>,
    range_check_20_b_58: Vec<[PackedM31; 2]>,
    range_check_20_c_59: Vec<[PackedM31; 2]>,
    range_check_20_d_60: Vec<[PackedM31; 2]>,
    range_check_20_e_61: Vec<[PackedM31; 2]>,
    range_check_20_f_62: Vec<[PackedM31; 2]>,
    range_check_20_g_63: Vec<[PackedM31; 2]>,
    range_check_20_h_64: Vec<[PackedM31; 2]>,
    range_check_20_65: Vec<[PackedM31; 2]>,
    range_check_20_b_66: Vec<[PackedM31; 2]>,
    range_check_20_c_67: Vec<[PackedM31; 2]>,
    range_check_20_d_68: Vec<[PackedM31; 2]>,
    range_check_20_e_69: Vec<[PackedM31; 2]>,
    range_check_20_f_70: Vec<[PackedM31; 2]>,
    range_check_20_g_71: Vec<[PackedM31; 2]>,
    range_check_20_h_72: Vec<[PackedM31; 2]>,
    range_check_20_73: Vec<[PackedM31; 2]>,
    range_check_20_b_74: Vec<[PackedM31; 2]>,
    range_check_20_c_75: Vec<[PackedM31; 2]>,
    range_check_20_d_76: Vec<[PackedM31; 2]>,
    range_check_20_e_77: Vec<[PackedM31; 2]>,
    range_check_20_f_78: Vec<[PackedM31; 2]>,
    range_check_20_g_79: Vec<[PackedM31; 2]>,
    range_check_20_h_80: Vec<[PackedM31; 2]>,
    range_check_20_81: Vec<[PackedM31; 2]>,
    range_check_20_b_82: Vec<[PackedM31; 2]>,
    range_check_20_c_83: Vec<[PackedM31; 2]>,
    range_check_20_d_84: Vec<[PackedM31; 2]>,
    range_check_9_9_85: Vec<[PackedM31; 3]>,
    range_check_9_9_b_86: Vec<[PackedM31; 3]>,
    range_check_9_9_c_87: Vec<[PackedM31; 3]>,
    range_check_9_9_d_88: Vec<[PackedM31; 3]>,
    range_check_9_9_e_89: Vec<[PackedM31; 3]>,
    range_check_9_9_f_90: Vec<[PackedM31; 3]>,
    range_check_9_9_g_91: Vec<[PackedM31; 3]>,
    range_check_9_9_h_92: Vec<[PackedM31; 3]>,
    range_check_9_9_93: Vec<[PackedM31; 3]>,
    range_check_9_9_b_94: Vec<[PackedM31; 3]>,
    range_check_9_9_c_95: Vec<[PackedM31; 3]>,
    range_check_9_9_d_96: Vec<[PackedM31; 3]>,
    range_check_9_9_e_97: Vec<[PackedM31; 3]>,
    range_check_9_9_f_98: Vec<[PackedM31; 3]>,
    range_check_20_99: Vec<[PackedM31; 2]>,
    range_check_20_b_100: Vec<[PackedM31; 2]>,
    range_check_20_c_101: Vec<[PackedM31; 2]>,
    range_check_20_d_102: Vec<[PackedM31; 2]>,
    range_check_20_e_103: Vec<[PackedM31; 2]>,
    range_check_20_f_104: Vec<[PackedM31; 2]>,
    range_check_20_g_105: Vec<[PackedM31; 2]>,
    range_check_20_h_106: Vec<[PackedM31; 2]>,
    range_check_20_107: Vec<[PackedM31; 2]>,
    range_check_20_b_108: Vec<[PackedM31; 2]>,
    range_check_20_c_109: Vec<[PackedM31; 2]>,
    range_check_20_d_110: Vec<[PackedM31; 2]>,
    range_check_20_e_111: Vec<[PackedM31; 2]>,
    range_check_20_f_112: Vec<[PackedM31; 2]>,
    range_check_20_g_113: Vec<[PackedM31; 2]>,
    range_check_20_h_114: Vec<[PackedM31; 2]>,
    range_check_20_115: Vec<[PackedM31; 2]>,
    range_check_20_b_116: Vec<[PackedM31; 2]>,
    range_check_20_c_117: Vec<[PackedM31; 2]>,
    range_check_20_d_118: Vec<[PackedM31; 2]>,
    range_check_20_e_119: Vec<[PackedM31; 2]>,
    range_check_20_f_120: Vec<[PackedM31; 2]>,
    range_check_20_g_121: Vec<[PackedM31; 2]>,
    range_check_20_h_122: Vec<[PackedM31; 2]>,
    range_check_20_123: Vec<[PackedM31; 2]>,
    range_check_20_b_124: Vec<[PackedM31; 2]>,
    range_check_20_c_125: Vec<[PackedM31; 2]>,
    range_check_20_d_126: Vec<[PackedM31; 2]>,
    partial_ec_mul_window_bits_18_127: Vec<[PackedM31; 73]>,
    partial_ec_mul_window_bits_18_128: Vec<[PackedM31; 73]>,
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
    ) -> (
        Vec<CircleEvaluation<SimdBackend, M31, BitReversedOrder>>,
        InteractionClaim,
    ) {
        let mut logup_gen = unsafe { LogupTraceGenerator::uninitialized(self.log_size) };

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.pedersen_points_table_window_bits_18_0,
            &self.lookup_data.range_check_9_9_1,
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
            &self.lookup_data.range_check_9_9_b_2,
            &self.lookup_data.range_check_9_9_c_3,
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
            &self.lookup_data.range_check_9_9_d_4,
            &self.lookup_data.range_check_9_9_e_5,
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
            &self.lookup_data.range_check_9_9_f_6,
            &self.lookup_data.range_check_9_9_g_7,
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
            &self.lookup_data.range_check_9_9_h_8,
            &self.lookup_data.range_check_9_9_9,
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
            &self.lookup_data.range_check_9_9_b_10,
            &self.lookup_data.range_check_9_9_c_11,
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
            &self.lookup_data.range_check_9_9_d_12,
            &self.lookup_data.range_check_9_9_e_13,
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
            &self.lookup_data.range_check_9_9_f_14,
            &self.lookup_data.range_check_20_15,
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
            &self.lookup_data.range_check_20_b_16,
            &self.lookup_data.range_check_20_c_17,
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
            &self.lookup_data.range_check_20_d_18,
            &self.lookup_data.range_check_20_e_19,
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
            &self.lookup_data.range_check_20_f_20,
            &self.lookup_data.range_check_20_g_21,
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
            &self.lookup_data.range_check_20_h_22,
            &self.lookup_data.range_check_20_23,
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
            &self.lookup_data.range_check_20_b_24,
            &self.lookup_data.range_check_20_c_25,
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
            &self.lookup_data.range_check_20_d_26,
            &self.lookup_data.range_check_20_e_27,
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
            &self.lookup_data.range_check_20_f_28,
            &self.lookup_data.range_check_20_g_29,
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
            &self.lookup_data.range_check_20_h_30,
            &self.lookup_data.range_check_20_31,
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
            &self.lookup_data.range_check_20_b_32,
            &self.lookup_data.range_check_20_c_33,
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
            &self.lookup_data.range_check_20_d_34,
            &self.lookup_data.range_check_20_e_35,
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
            &self.lookup_data.range_check_20_f_36,
            &self.lookup_data.range_check_20_g_37,
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
            &self.lookup_data.range_check_20_h_38,
            &self.lookup_data.range_check_20_39,
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
            &self.lookup_data.range_check_20_b_40,
            &self.lookup_data.range_check_20_c_41,
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
            &self.lookup_data.range_check_20_d_42,
            &self.lookup_data.range_check_9_9_43,
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
            &self.lookup_data.range_check_9_9_b_44,
            &self.lookup_data.range_check_9_9_c_45,
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
            &self.lookup_data.range_check_9_9_d_46,
            &self.lookup_data.range_check_9_9_e_47,
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
            &self.lookup_data.range_check_9_9_f_48,
            &self.lookup_data.range_check_9_9_g_49,
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
            &self.lookup_data.range_check_9_9_h_50,
            &self.lookup_data.range_check_9_9_51,
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
            &self.lookup_data.range_check_9_9_b_52,
            &self.lookup_data.range_check_9_9_c_53,
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
            &self.lookup_data.range_check_9_9_d_54,
            &self.lookup_data.range_check_9_9_e_55,
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
            &self.lookup_data.range_check_9_9_f_56,
            &self.lookup_data.range_check_20_57,
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
            &self.lookup_data.range_check_20_b_58,
            &self.lookup_data.range_check_20_c_59,
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
            &self.lookup_data.range_check_20_d_60,
            &self.lookup_data.range_check_20_e_61,
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
            &self.lookup_data.range_check_20_f_62,
            &self.lookup_data.range_check_20_g_63,
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
            &self.lookup_data.range_check_20_h_64,
            &self.lookup_data.range_check_20_65,
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
            &self.lookup_data.range_check_20_b_66,
            &self.lookup_data.range_check_20_c_67,
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
            &self.lookup_data.range_check_20_d_68,
            &self.lookup_data.range_check_20_e_69,
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
            &self.lookup_data.range_check_20_f_70,
            &self.lookup_data.range_check_20_g_71,
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
            &self.lookup_data.range_check_20_h_72,
            &self.lookup_data.range_check_20_73,
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
            &self.lookup_data.range_check_20_b_74,
            &self.lookup_data.range_check_20_c_75,
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
            &self.lookup_data.range_check_20_d_76,
            &self.lookup_data.range_check_20_e_77,
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
            &self.lookup_data.range_check_20_f_78,
            &self.lookup_data.range_check_20_g_79,
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
            &self.lookup_data.range_check_20_h_80,
            &self.lookup_data.range_check_20_81,
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
            &self.lookup_data.range_check_20_b_82,
            &self.lookup_data.range_check_20_c_83,
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
            &self.lookup_data.range_check_20_d_84,
            &self.lookup_data.range_check_9_9_85,
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
            &self.lookup_data.range_check_9_9_b_86,
            &self.lookup_data.range_check_9_9_c_87,
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
            &self.lookup_data.range_check_9_9_d_88,
            &self.lookup_data.range_check_9_9_e_89,
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
            &self.lookup_data.range_check_9_9_f_90,
            &self.lookup_data.range_check_9_9_g_91,
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
            &self.lookup_data.range_check_9_9_h_92,
            &self.lookup_data.range_check_9_9_93,
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
            &self.lookup_data.range_check_9_9_b_94,
            &self.lookup_data.range_check_9_9_c_95,
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
            &self.lookup_data.range_check_9_9_d_96,
            &self.lookup_data.range_check_9_9_e_97,
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
            &self.lookup_data.range_check_9_9_f_98,
            &self.lookup_data.range_check_20_99,
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
            &self.lookup_data.range_check_20_b_100,
            &self.lookup_data.range_check_20_c_101,
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
            &self.lookup_data.range_check_20_d_102,
            &self.lookup_data.range_check_20_e_103,
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
            &self.lookup_data.range_check_20_f_104,
            &self.lookup_data.range_check_20_g_105,
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
            &self.lookup_data.range_check_20_h_106,
            &self.lookup_data.range_check_20_107,
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
            &self.lookup_data.range_check_20_b_108,
            &self.lookup_data.range_check_20_c_109,
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
            &self.lookup_data.range_check_20_d_110,
            &self.lookup_data.range_check_20_e_111,
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
            &self.lookup_data.range_check_20_f_112,
            &self.lookup_data.range_check_20_g_113,
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
            &self.lookup_data.range_check_20_h_114,
            &self.lookup_data.range_check_20_115,
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
            &self.lookup_data.range_check_20_b_116,
            &self.lookup_data.range_check_20_c_117,
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
            &self.lookup_data.range_check_20_d_118,
            &self.lookup_data.range_check_20_e_119,
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
            &self.lookup_data.range_check_20_f_120,
            &self.lookup_data.range_check_20_g_121,
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
            &self.lookup_data.range_check_20_h_122,
            &self.lookup_data.range_check_20_123,
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
            &self.lookup_data.range_check_20_b_124,
            &self.lookup_data.range_check_20_c_125,
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
            &self.lookup_data.range_check_20_d_126,
            &self.lookup_data.partial_ec_mul_window_bits_18_127,
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
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.partial_ec_mul_window_bits_18_128,
            self.lookup_data.mults_0,
        )
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
