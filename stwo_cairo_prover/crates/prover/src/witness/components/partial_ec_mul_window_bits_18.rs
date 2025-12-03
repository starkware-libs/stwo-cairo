#![allow(unused_parens)]
use cairo_air::components::partial_ec_mul_window_bits_18::{
    Claim, InteractionClaim, N_TRACE_COLUMNS,
};

use crate::witness::components::{
    pedersen_points_table_window_bits_18, range_check_20, range_check_9_9,
};
use crate::witness::prelude::*;

pub type PackedInputType = (PackedM31, PackedM31, ([PackedM31; 14], [PackedFelt252; 2]));

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
        pedersen_points_table_window_bits_18_state: &pedersen_points_table_window_bits_18::ClaimGenerator,
        range_check_9_9_state: &range_check_9_9::ClaimGenerator,
        range_check_20_state: &range_check_20::ClaimGenerator,
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
                pedersen_points_table_window_bits_18_state,
                range_check_9_9_state,
                range_check_20_state,
            )
        });
        sub_component_inputs
            .pedersen_points_table_window_bits_18
            .iter()
            .for_each(|inputs| {
                pedersen_points_table_window_bits_18_state.add_packed_inputs(inputs, 0);
            });
        sub_component_inputs
            .range_check_9_9
            .iter()
            .for_each(|inputs| {
                range_check_9_9_state.add_packed_inputs(inputs, 0);
            });
        sub_component_inputs
            .range_check_9_9_b
            .iter()
            .for_each(|inputs| {
                range_check_9_9_state.add_packed_inputs(inputs, 1);
            });
        sub_component_inputs
            .range_check_9_9_c
            .iter()
            .for_each(|inputs| {
                range_check_9_9_state.add_packed_inputs(inputs, 2);
            });
        sub_component_inputs
            .range_check_9_9_d
            .iter()
            .for_each(|inputs| {
                range_check_9_9_state.add_packed_inputs(inputs, 3);
            });
        sub_component_inputs
            .range_check_9_9_e
            .iter()
            .for_each(|inputs| {
                range_check_9_9_state.add_packed_inputs(inputs, 4);
            });
        sub_component_inputs
            .range_check_9_9_f
            .iter()
            .for_each(|inputs| {
                range_check_9_9_state.add_packed_inputs(inputs, 5);
            });
        sub_component_inputs
            .range_check_9_9_g
            .iter()
            .for_each(|inputs| {
                range_check_9_9_state.add_packed_inputs(inputs, 6);
            });
        sub_component_inputs
            .range_check_9_9_h
            .iter()
            .for_each(|inputs| {
                range_check_9_9_state.add_packed_inputs(inputs, 7);
            });
        sub_component_inputs
            .range_check_20
            .iter()
            .for_each(|inputs| {
                range_check_20_state.add_packed_inputs(inputs, 0);
            });
        sub_component_inputs
            .range_check_20_b
            .iter()
            .for_each(|inputs| {
                range_check_20_state.add_packed_inputs(inputs, 1);
            });
        sub_component_inputs
            .range_check_20_c
            .iter()
            .for_each(|inputs| {
                range_check_20_state.add_packed_inputs(inputs, 2);
            });
        sub_component_inputs
            .range_check_20_d
            .iter()
            .for_each(|inputs| {
                range_check_20_state.add_packed_inputs(inputs, 3);
            });
        sub_component_inputs
            .range_check_20_e
            .iter()
            .for_each(|inputs| {
                range_check_20_state.add_packed_inputs(inputs, 4);
            });
        sub_component_inputs
            .range_check_20_f
            .iter()
            .for_each(|inputs| {
                range_check_20_state.add_packed_inputs(inputs, 5);
            });
        sub_component_inputs
            .range_check_20_g
            .iter()
            .for_each(|inputs| {
                range_check_20_state.add_packed_inputs(inputs, 6);
            });
        sub_component_inputs
            .range_check_20_h
            .iter()
            .for_each(|inputs| {
                range_check_20_state.add_packed_inputs(inputs, 7);
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

    pub fn add_packed_inputs(&mut self, inputs: &[PackedInputType], _relation_index: usize) {
        self.packed_inputs.extend(inputs);
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
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_4 = PackedM31::broadcast(M31::from(4));
    let M31_4194304 = PackedM31::broadcast(M31::from(4194304));
    let M31_524288 = PackedM31::broadcast(M31::from(524288));
    let M31_64 = PackedM31::broadcast(M31::from(64));
    let M31_65536 = PackedM31::broadcast(M31::from(65536));
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
                let input_limb_0_col0 = partial_ec_mul_window_bits_18_input.0;
                *row[0] = input_limb_0_col0;
                let input_limb_1_col1 = partial_ec_mul_window_bits_18_input.1;
                *row[1] = input_limb_1_col1;
                let input_limb_2_col2 = partial_ec_mul_window_bits_18_input.2 .0[0];
                *row[2] = input_limb_2_col2;
                let input_limb_3_col3 = partial_ec_mul_window_bits_18_input.2 .0[1];
                *row[3] = input_limb_3_col3;
                let input_limb_4_col4 = partial_ec_mul_window_bits_18_input.2 .0[2];
                *row[4] = input_limb_4_col4;
                let input_limb_5_col5 = partial_ec_mul_window_bits_18_input.2 .0[3];
                *row[5] = input_limb_5_col5;
                let input_limb_6_col6 = partial_ec_mul_window_bits_18_input.2 .0[4];
                *row[6] = input_limb_6_col6;
                let input_limb_7_col7 = partial_ec_mul_window_bits_18_input.2 .0[5];
                *row[7] = input_limb_7_col7;
                let input_limb_8_col8 = partial_ec_mul_window_bits_18_input.2 .0[6];
                *row[8] = input_limb_8_col8;
                let input_limb_9_col9 = partial_ec_mul_window_bits_18_input.2 .0[7];
                *row[9] = input_limb_9_col9;
                let input_limb_10_col10 = partial_ec_mul_window_bits_18_input.2 .0[8];
                *row[10] = input_limb_10_col10;
                let input_limb_11_col11 = partial_ec_mul_window_bits_18_input.2 .0[9];
                *row[11] = input_limb_11_col11;
                let input_limb_12_col12 = partial_ec_mul_window_bits_18_input.2 .0[10];
                *row[12] = input_limb_12_col12;
                let input_limb_13_col13 = partial_ec_mul_window_bits_18_input.2 .0[11];
                *row[13] = input_limb_13_col13;
                let input_limb_14_col14 = partial_ec_mul_window_bits_18_input.2 .0[12];
                *row[14] = input_limb_14_col14;
                let input_limb_15_col15 = partial_ec_mul_window_bits_18_input.2 .0[13];
                *row[15] = input_limb_15_col15;
                let input_limb_16_col16 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(0);
                *row[16] = input_limb_16_col16;
                let input_limb_17_col17 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(1);
                *row[17] = input_limb_17_col17;
                let input_limb_18_col18 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(2);
                *row[18] = input_limb_18_col18;
                let input_limb_19_col19 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(3);
                *row[19] = input_limb_19_col19;
                let input_limb_20_col20 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(4);
                *row[20] = input_limb_20_col20;
                let input_limb_21_col21 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(5);
                *row[21] = input_limb_21_col21;
                let input_limb_22_col22 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(6);
                *row[22] = input_limb_22_col22;
                let input_limb_23_col23 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(7);
                *row[23] = input_limb_23_col23;
                let input_limb_24_col24 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(8);
                *row[24] = input_limb_24_col24;
                let input_limb_25_col25 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(9);
                *row[25] = input_limb_25_col25;
                let input_limb_26_col26 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(10);
                *row[26] = input_limb_26_col26;
                let input_limb_27_col27 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(11);
                *row[27] = input_limb_27_col27;
                let input_limb_28_col28 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(12);
                *row[28] = input_limb_28_col28;
                let input_limb_29_col29 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(13);
                *row[29] = input_limb_29_col29;
                let input_limb_30_col30 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(14);
                *row[30] = input_limb_30_col30;
                let input_limb_31_col31 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(15);
                *row[31] = input_limb_31_col31;
                let input_limb_32_col32 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(16);
                *row[32] = input_limb_32_col32;
                let input_limb_33_col33 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(17);
                *row[33] = input_limb_33_col33;
                let input_limb_34_col34 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(18);
                *row[34] = input_limb_34_col34;
                let input_limb_35_col35 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(19);
                *row[35] = input_limb_35_col35;
                let input_limb_36_col36 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(20);
                *row[36] = input_limb_36_col36;
                let input_limb_37_col37 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(21);
                *row[37] = input_limb_37_col37;
                let input_limb_38_col38 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(22);
                *row[38] = input_limb_38_col38;
                let input_limb_39_col39 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(23);
                *row[39] = input_limb_39_col39;
                let input_limb_40_col40 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(24);
                *row[40] = input_limb_40_col40;
                let input_limb_41_col41 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(25);
                *row[41] = input_limb_41_col41;
                let input_limb_42_col42 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(26);
                *row[42] = input_limb_42_col42;
                let input_limb_43_col43 = partial_ec_mul_window_bits_18_input.2 .1[0].get_m31(27);
                *row[43] = input_limb_43_col43;
                let input_limb_44_col44 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(0);
                *row[44] = input_limb_44_col44;
                let input_limb_45_col45 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(1);
                *row[45] = input_limb_45_col45;
                let input_limb_46_col46 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(2);
                *row[46] = input_limb_46_col46;
                let input_limb_47_col47 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(3);
                *row[47] = input_limb_47_col47;
                let input_limb_48_col48 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(4);
                *row[48] = input_limb_48_col48;
                let input_limb_49_col49 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(5);
                *row[49] = input_limb_49_col49;
                let input_limb_50_col50 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(6);
                *row[50] = input_limb_50_col50;
                let input_limb_51_col51 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(7);
                *row[51] = input_limb_51_col51;
                let input_limb_52_col52 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(8);
                *row[52] = input_limb_52_col52;
                let input_limb_53_col53 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(9);
                *row[53] = input_limb_53_col53;
                let input_limb_54_col54 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(10);
                *row[54] = input_limb_54_col54;
                let input_limb_55_col55 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(11);
                *row[55] = input_limb_55_col55;
                let input_limb_56_col56 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(12);
                *row[56] = input_limb_56_col56;
                let input_limb_57_col57 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(13);
                *row[57] = input_limb_57_col57;
                let input_limb_58_col58 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(14);
                *row[58] = input_limb_58_col58;
                let input_limb_59_col59 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(15);
                *row[59] = input_limb_59_col59;
                let input_limb_60_col60 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(16);
                *row[60] = input_limb_60_col60;
                let input_limb_61_col61 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(17);
                *row[61] = input_limb_61_col61;
                let input_limb_62_col62 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(18);
                *row[62] = input_limb_62_col62;
                let input_limb_63_col63 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(19);
                *row[63] = input_limb_63_col63;
                let input_limb_64_col64 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(20);
                *row[64] = input_limb_64_col64;
                let input_limb_65_col65 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(21);
                *row[65] = input_limb_65_col65;
                let input_limb_66_col66 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(22);
                *row[66] = input_limb_66_col66;
                let input_limb_67_col67 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(23);
                *row[67] = input_limb_67_col67;
                let input_limb_68_col68 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(24);
                *row[68] = input_limb_68_col68;
                let input_limb_69_col69 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(25);
                *row[69] = input_limb_69_col69;
                let input_limb_70_col70 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(26);
                *row[70] = input_limb_70_col70;
                let input_limb_71_col71 = partial_ec_mul_window_bits_18_input.2 .1[1].get_m31(27);
                *row[71] = input_limb_71_col71;
                *sub_component_inputs.pedersen_points_table_window_bits_18[0] =
                    [(((M31_262144) * (input_limb_1_col1)) + (input_limb_2_col2))];
                let pedersen_points_table_window_bits_18_output_tmp_3cd4f_0 =
                    PackedPedersenPointsTableWindowBits18::deduce_output([(((M31_262144)
                        * (input_limb_1_col1))
                        + (input_limb_2_col2))]);
                let pedersen_points_table_window_bits_18_output_limb_0_col72 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(0);
                *row[72] = pedersen_points_table_window_bits_18_output_limb_0_col72;
                let pedersen_points_table_window_bits_18_output_limb_1_col73 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(1);
                *row[73] = pedersen_points_table_window_bits_18_output_limb_1_col73;
                let pedersen_points_table_window_bits_18_output_limb_2_col74 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(2);
                *row[74] = pedersen_points_table_window_bits_18_output_limb_2_col74;
                let pedersen_points_table_window_bits_18_output_limb_3_col75 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(3);
                *row[75] = pedersen_points_table_window_bits_18_output_limb_3_col75;
                let pedersen_points_table_window_bits_18_output_limb_4_col76 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(4);
                *row[76] = pedersen_points_table_window_bits_18_output_limb_4_col76;
                let pedersen_points_table_window_bits_18_output_limb_5_col77 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(5);
                *row[77] = pedersen_points_table_window_bits_18_output_limb_5_col77;
                let pedersen_points_table_window_bits_18_output_limb_6_col78 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(6);
                *row[78] = pedersen_points_table_window_bits_18_output_limb_6_col78;
                let pedersen_points_table_window_bits_18_output_limb_7_col79 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(7);
                *row[79] = pedersen_points_table_window_bits_18_output_limb_7_col79;
                let pedersen_points_table_window_bits_18_output_limb_8_col80 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(8);
                *row[80] = pedersen_points_table_window_bits_18_output_limb_8_col80;
                let pedersen_points_table_window_bits_18_output_limb_9_col81 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(9);
                *row[81] = pedersen_points_table_window_bits_18_output_limb_9_col81;
                let pedersen_points_table_window_bits_18_output_limb_10_col82 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(10);
                *row[82] = pedersen_points_table_window_bits_18_output_limb_10_col82;
                let pedersen_points_table_window_bits_18_output_limb_11_col83 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(11);
                *row[83] = pedersen_points_table_window_bits_18_output_limb_11_col83;
                let pedersen_points_table_window_bits_18_output_limb_12_col84 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(12);
                *row[84] = pedersen_points_table_window_bits_18_output_limb_12_col84;
                let pedersen_points_table_window_bits_18_output_limb_13_col85 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(13);
                *row[85] = pedersen_points_table_window_bits_18_output_limb_13_col85;
                let pedersen_points_table_window_bits_18_output_limb_14_col86 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(14);
                *row[86] = pedersen_points_table_window_bits_18_output_limb_14_col86;
                let pedersen_points_table_window_bits_18_output_limb_15_col87 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(15);
                *row[87] = pedersen_points_table_window_bits_18_output_limb_15_col87;
                let pedersen_points_table_window_bits_18_output_limb_16_col88 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(16);
                *row[88] = pedersen_points_table_window_bits_18_output_limb_16_col88;
                let pedersen_points_table_window_bits_18_output_limb_17_col89 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(17);
                *row[89] = pedersen_points_table_window_bits_18_output_limb_17_col89;
                let pedersen_points_table_window_bits_18_output_limb_18_col90 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(18);
                *row[90] = pedersen_points_table_window_bits_18_output_limb_18_col90;
                let pedersen_points_table_window_bits_18_output_limb_19_col91 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(19);
                *row[91] = pedersen_points_table_window_bits_18_output_limb_19_col91;
                let pedersen_points_table_window_bits_18_output_limb_20_col92 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(20);
                *row[92] = pedersen_points_table_window_bits_18_output_limb_20_col92;
                let pedersen_points_table_window_bits_18_output_limb_21_col93 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(21);
                *row[93] = pedersen_points_table_window_bits_18_output_limb_21_col93;
                let pedersen_points_table_window_bits_18_output_limb_22_col94 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(22);
                *row[94] = pedersen_points_table_window_bits_18_output_limb_22_col94;
                let pedersen_points_table_window_bits_18_output_limb_23_col95 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(23);
                *row[95] = pedersen_points_table_window_bits_18_output_limb_23_col95;
                let pedersen_points_table_window_bits_18_output_limb_24_col96 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(24);
                *row[96] = pedersen_points_table_window_bits_18_output_limb_24_col96;
                let pedersen_points_table_window_bits_18_output_limb_25_col97 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(25);
                *row[97] = pedersen_points_table_window_bits_18_output_limb_25_col97;
                let pedersen_points_table_window_bits_18_output_limb_26_col98 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(26);
                *row[98] = pedersen_points_table_window_bits_18_output_limb_26_col98;
                let pedersen_points_table_window_bits_18_output_limb_27_col99 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0].get_m31(27);
                *row[99] = pedersen_points_table_window_bits_18_output_limb_27_col99;
                let pedersen_points_table_window_bits_18_output_limb_28_col100 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(0);
                *row[100] = pedersen_points_table_window_bits_18_output_limb_28_col100;
                let pedersen_points_table_window_bits_18_output_limb_29_col101 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(1);
                *row[101] = pedersen_points_table_window_bits_18_output_limb_29_col101;
                let pedersen_points_table_window_bits_18_output_limb_30_col102 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(2);
                *row[102] = pedersen_points_table_window_bits_18_output_limb_30_col102;
                let pedersen_points_table_window_bits_18_output_limb_31_col103 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(3);
                *row[103] = pedersen_points_table_window_bits_18_output_limb_31_col103;
                let pedersen_points_table_window_bits_18_output_limb_32_col104 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(4);
                *row[104] = pedersen_points_table_window_bits_18_output_limb_32_col104;
                let pedersen_points_table_window_bits_18_output_limb_33_col105 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(5);
                *row[105] = pedersen_points_table_window_bits_18_output_limb_33_col105;
                let pedersen_points_table_window_bits_18_output_limb_34_col106 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(6);
                *row[106] = pedersen_points_table_window_bits_18_output_limb_34_col106;
                let pedersen_points_table_window_bits_18_output_limb_35_col107 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(7);
                *row[107] = pedersen_points_table_window_bits_18_output_limb_35_col107;
                let pedersen_points_table_window_bits_18_output_limb_36_col108 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(8);
                *row[108] = pedersen_points_table_window_bits_18_output_limb_36_col108;
                let pedersen_points_table_window_bits_18_output_limb_37_col109 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(9);
                *row[109] = pedersen_points_table_window_bits_18_output_limb_37_col109;
                let pedersen_points_table_window_bits_18_output_limb_38_col110 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(10);
                *row[110] = pedersen_points_table_window_bits_18_output_limb_38_col110;
                let pedersen_points_table_window_bits_18_output_limb_39_col111 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(11);
                *row[111] = pedersen_points_table_window_bits_18_output_limb_39_col111;
                let pedersen_points_table_window_bits_18_output_limb_40_col112 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(12);
                *row[112] = pedersen_points_table_window_bits_18_output_limb_40_col112;
                let pedersen_points_table_window_bits_18_output_limb_41_col113 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(13);
                *row[113] = pedersen_points_table_window_bits_18_output_limb_41_col113;
                let pedersen_points_table_window_bits_18_output_limb_42_col114 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(14);
                *row[114] = pedersen_points_table_window_bits_18_output_limb_42_col114;
                let pedersen_points_table_window_bits_18_output_limb_43_col115 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(15);
                *row[115] = pedersen_points_table_window_bits_18_output_limb_43_col115;
                let pedersen_points_table_window_bits_18_output_limb_44_col116 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(16);
                *row[116] = pedersen_points_table_window_bits_18_output_limb_44_col116;
                let pedersen_points_table_window_bits_18_output_limb_45_col117 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(17);
                *row[117] = pedersen_points_table_window_bits_18_output_limb_45_col117;
                let pedersen_points_table_window_bits_18_output_limb_46_col118 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(18);
                *row[118] = pedersen_points_table_window_bits_18_output_limb_46_col118;
                let pedersen_points_table_window_bits_18_output_limb_47_col119 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(19);
                *row[119] = pedersen_points_table_window_bits_18_output_limb_47_col119;
                let pedersen_points_table_window_bits_18_output_limb_48_col120 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(20);
                *row[120] = pedersen_points_table_window_bits_18_output_limb_48_col120;
                let pedersen_points_table_window_bits_18_output_limb_49_col121 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(21);
                *row[121] = pedersen_points_table_window_bits_18_output_limb_49_col121;
                let pedersen_points_table_window_bits_18_output_limb_50_col122 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(22);
                *row[122] = pedersen_points_table_window_bits_18_output_limb_50_col122;
                let pedersen_points_table_window_bits_18_output_limb_51_col123 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(23);
                *row[123] = pedersen_points_table_window_bits_18_output_limb_51_col123;
                let pedersen_points_table_window_bits_18_output_limb_52_col124 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(24);
                *row[124] = pedersen_points_table_window_bits_18_output_limb_52_col124;
                let pedersen_points_table_window_bits_18_output_limb_53_col125 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(25);
                *row[125] = pedersen_points_table_window_bits_18_output_limb_53_col125;
                let pedersen_points_table_window_bits_18_output_limb_54_col126 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(26);
                *row[126] = pedersen_points_table_window_bits_18_output_limb_54_col126;
                let pedersen_points_table_window_bits_18_output_limb_55_col127 =
                    pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[1].get_m31(27);
                *row[127] = pedersen_points_table_window_bits_18_output_limb_55_col127;
                *lookup_data.pedersen_points_table_window_bits_18_0 = [
                    (((M31_262144) * (input_limb_1_col1)) + (input_limb_2_col2)),
                    pedersen_points_table_window_bits_18_output_limb_0_col72,
                    pedersen_points_table_window_bits_18_output_limb_1_col73,
                    pedersen_points_table_window_bits_18_output_limb_2_col74,
                    pedersen_points_table_window_bits_18_output_limb_3_col75,
                    pedersen_points_table_window_bits_18_output_limb_4_col76,
                    pedersen_points_table_window_bits_18_output_limb_5_col77,
                    pedersen_points_table_window_bits_18_output_limb_6_col78,
                    pedersen_points_table_window_bits_18_output_limb_7_col79,
                    pedersen_points_table_window_bits_18_output_limb_8_col80,
                    pedersen_points_table_window_bits_18_output_limb_9_col81,
                    pedersen_points_table_window_bits_18_output_limb_10_col82,
                    pedersen_points_table_window_bits_18_output_limb_11_col83,
                    pedersen_points_table_window_bits_18_output_limb_12_col84,
                    pedersen_points_table_window_bits_18_output_limb_13_col85,
                    pedersen_points_table_window_bits_18_output_limb_14_col86,
                    pedersen_points_table_window_bits_18_output_limb_15_col87,
                    pedersen_points_table_window_bits_18_output_limb_16_col88,
                    pedersen_points_table_window_bits_18_output_limb_17_col89,
                    pedersen_points_table_window_bits_18_output_limb_18_col90,
                    pedersen_points_table_window_bits_18_output_limb_19_col91,
                    pedersen_points_table_window_bits_18_output_limb_20_col92,
                    pedersen_points_table_window_bits_18_output_limb_21_col93,
                    pedersen_points_table_window_bits_18_output_limb_22_col94,
                    pedersen_points_table_window_bits_18_output_limb_23_col95,
                    pedersen_points_table_window_bits_18_output_limb_24_col96,
                    pedersen_points_table_window_bits_18_output_limb_25_col97,
                    pedersen_points_table_window_bits_18_output_limb_26_col98,
                    pedersen_points_table_window_bits_18_output_limb_27_col99,
                    pedersen_points_table_window_bits_18_output_limb_28_col100,
                    pedersen_points_table_window_bits_18_output_limb_29_col101,
                    pedersen_points_table_window_bits_18_output_limb_30_col102,
                    pedersen_points_table_window_bits_18_output_limb_31_col103,
                    pedersen_points_table_window_bits_18_output_limb_32_col104,
                    pedersen_points_table_window_bits_18_output_limb_33_col105,
                    pedersen_points_table_window_bits_18_output_limb_34_col106,
                    pedersen_points_table_window_bits_18_output_limb_35_col107,
                    pedersen_points_table_window_bits_18_output_limb_36_col108,
                    pedersen_points_table_window_bits_18_output_limb_37_col109,
                    pedersen_points_table_window_bits_18_output_limb_38_col110,
                    pedersen_points_table_window_bits_18_output_limb_39_col111,
                    pedersen_points_table_window_bits_18_output_limb_40_col112,
                    pedersen_points_table_window_bits_18_output_limb_41_col113,
                    pedersen_points_table_window_bits_18_output_limb_42_col114,
                    pedersen_points_table_window_bits_18_output_limb_43_col115,
                    pedersen_points_table_window_bits_18_output_limb_44_col116,
                    pedersen_points_table_window_bits_18_output_limb_45_col117,
                    pedersen_points_table_window_bits_18_output_limb_46_col118,
                    pedersen_points_table_window_bits_18_output_limb_47_col119,
                    pedersen_points_table_window_bits_18_output_limb_48_col120,
                    pedersen_points_table_window_bits_18_output_limb_49_col121,
                    pedersen_points_table_window_bits_18_output_limb_50_col122,
                    pedersen_points_table_window_bits_18_output_limb_51_col123,
                    pedersen_points_table_window_bits_18_output_limb_52_col124,
                    pedersen_points_table_window_bits_18_output_limb_53_col125,
                    pedersen_points_table_window_bits_18_output_limb_54_col126,
                    pedersen_points_table_window_bits_18_output_limb_55_col127,
                ];

                // Ec Add.

                let slope_tmp_3cd4f_1 = (((pedersen_points_table_window_bits_18_output_tmp_3cd4f_0
                    [1])
                    - (partial_ec_mul_window_bits_18_input.2 .1[1]))
                    / ((pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0])
                        - (partial_ec_mul_window_bits_18_input.2 .1[0])));
                let slope_limb_0_col128 = slope_tmp_3cd4f_1.get_m31(0);
                *row[128] = slope_limb_0_col128;
                let slope_limb_1_col129 = slope_tmp_3cd4f_1.get_m31(1);
                *row[129] = slope_limb_1_col129;
                let slope_limb_2_col130 = slope_tmp_3cd4f_1.get_m31(2);
                *row[130] = slope_limb_2_col130;
                let slope_limb_3_col131 = slope_tmp_3cd4f_1.get_m31(3);
                *row[131] = slope_limb_3_col131;
                let slope_limb_4_col132 = slope_tmp_3cd4f_1.get_m31(4);
                *row[132] = slope_limb_4_col132;
                let slope_limb_5_col133 = slope_tmp_3cd4f_1.get_m31(5);
                *row[133] = slope_limb_5_col133;
                let slope_limb_6_col134 = slope_tmp_3cd4f_1.get_m31(6);
                *row[134] = slope_limb_6_col134;
                let slope_limb_7_col135 = slope_tmp_3cd4f_1.get_m31(7);
                *row[135] = slope_limb_7_col135;
                let slope_limb_8_col136 = slope_tmp_3cd4f_1.get_m31(8);
                *row[136] = slope_limb_8_col136;
                let slope_limb_9_col137 = slope_tmp_3cd4f_1.get_m31(9);
                *row[137] = slope_limb_9_col137;
                let slope_limb_10_col138 = slope_tmp_3cd4f_1.get_m31(10);
                *row[138] = slope_limb_10_col138;
                let slope_limb_11_col139 = slope_tmp_3cd4f_1.get_m31(11);
                *row[139] = slope_limb_11_col139;
                let slope_limb_12_col140 = slope_tmp_3cd4f_1.get_m31(12);
                *row[140] = slope_limb_12_col140;
                let slope_limb_13_col141 = slope_tmp_3cd4f_1.get_m31(13);
                *row[141] = slope_limb_13_col141;
                let slope_limb_14_col142 = slope_tmp_3cd4f_1.get_m31(14);
                *row[142] = slope_limb_14_col142;
                let slope_limb_15_col143 = slope_tmp_3cd4f_1.get_m31(15);
                *row[143] = slope_limb_15_col143;
                let slope_limb_16_col144 = slope_tmp_3cd4f_1.get_m31(16);
                *row[144] = slope_limb_16_col144;
                let slope_limb_17_col145 = slope_tmp_3cd4f_1.get_m31(17);
                *row[145] = slope_limb_17_col145;
                let slope_limb_18_col146 = slope_tmp_3cd4f_1.get_m31(18);
                *row[146] = slope_limb_18_col146;
                let slope_limb_19_col147 = slope_tmp_3cd4f_1.get_m31(19);
                *row[147] = slope_limb_19_col147;
                let slope_limb_20_col148 = slope_tmp_3cd4f_1.get_m31(20);
                *row[148] = slope_limb_20_col148;
                let slope_limb_21_col149 = slope_tmp_3cd4f_1.get_m31(21);
                *row[149] = slope_limb_21_col149;
                let slope_limb_22_col150 = slope_tmp_3cd4f_1.get_m31(22);
                *row[150] = slope_limb_22_col150;
                let slope_limb_23_col151 = slope_tmp_3cd4f_1.get_m31(23);
                *row[151] = slope_limb_23_col151;
                let slope_limb_24_col152 = slope_tmp_3cd4f_1.get_m31(24);
                *row[152] = slope_limb_24_col152;
                let slope_limb_25_col153 = slope_tmp_3cd4f_1.get_m31(25);
                *row[153] = slope_limb_25_col153;
                let slope_limb_26_col154 = slope_tmp_3cd4f_1.get_m31(26);
                *row[154] = slope_limb_26_col154;
                let slope_limb_27_col155 = slope_tmp_3cd4f_1.get_m31(27);
                *row[155] = slope_limb_27_col155;

                // Range Check Mem Value N 28.

                *sub_component_inputs.range_check_9_9[0] =
                    [slope_limb_0_col128, slope_limb_1_col129];
                *lookup_data.range_check_9_9_0 = [slope_limb_0_col128, slope_limb_1_col129];
                *sub_component_inputs.range_check_9_9_b[0] =
                    [slope_limb_2_col130, slope_limb_3_col131];
                *lookup_data.range_check_9_9_b_0 = [slope_limb_2_col130, slope_limb_3_col131];
                *sub_component_inputs.range_check_9_9_c[0] =
                    [slope_limb_4_col132, slope_limb_5_col133];
                *lookup_data.range_check_9_9_c_0 = [slope_limb_4_col132, slope_limb_5_col133];
                *sub_component_inputs.range_check_9_9_d[0] =
                    [slope_limb_6_col134, slope_limb_7_col135];
                *lookup_data.range_check_9_9_d_0 = [slope_limb_6_col134, slope_limb_7_col135];
                *sub_component_inputs.range_check_9_9_e[0] =
                    [slope_limb_8_col136, slope_limb_9_col137];
                *lookup_data.range_check_9_9_e_0 = [slope_limb_8_col136, slope_limb_9_col137];
                *sub_component_inputs.range_check_9_9_f[0] =
                    [slope_limb_10_col138, slope_limb_11_col139];
                *lookup_data.range_check_9_9_f_0 = [slope_limb_10_col138, slope_limb_11_col139];
                *sub_component_inputs.range_check_9_9_g[0] =
                    [slope_limb_12_col140, slope_limb_13_col141];
                *lookup_data.range_check_9_9_g_0 = [slope_limb_12_col140, slope_limb_13_col141];
                *sub_component_inputs.range_check_9_9_h[0] =
                    [slope_limb_14_col142, slope_limb_15_col143];
                *lookup_data.range_check_9_9_h_0 = [slope_limb_14_col142, slope_limb_15_col143];
                *sub_component_inputs.range_check_9_9[1] =
                    [slope_limb_16_col144, slope_limb_17_col145];
                *lookup_data.range_check_9_9_1 = [slope_limb_16_col144, slope_limb_17_col145];
                *sub_component_inputs.range_check_9_9_b[1] =
                    [slope_limb_18_col146, slope_limb_19_col147];
                *lookup_data.range_check_9_9_b_1 = [slope_limb_18_col146, slope_limb_19_col147];
                *sub_component_inputs.range_check_9_9_c[1] =
                    [slope_limb_20_col148, slope_limb_21_col149];
                *lookup_data.range_check_9_9_c_1 = [slope_limb_20_col148, slope_limb_21_col149];
                *sub_component_inputs.range_check_9_9_d[1] =
                    [slope_limb_22_col150, slope_limb_23_col151];
                *lookup_data.range_check_9_9_d_1 = [slope_limb_22_col150, slope_limb_23_col151];
                *sub_component_inputs.range_check_9_9_e[1] =
                    [slope_limb_24_col152, slope_limb_25_col153];
                *lookup_data.range_check_9_9_e_1 = [slope_limb_24_col152, slope_limb_25_col153];
                *sub_component_inputs.range_check_9_9_f[1] =
                    [slope_limb_26_col154, slope_limb_27_col155];
                *lookup_data.range_check_9_9_f_1 = [slope_limb_26_col154, slope_limb_27_col155];

                // Verify Mul 252.

                // Double Karatsuba 1454 B.

                // Single Karatsuba N 7.

                let z0_tmp_3cd4f_2 = [
                    ((slope_limb_0_col128)
                        * ((pedersen_points_table_window_bits_18_output_limb_0_col72)
                            - (input_limb_16_col16))),
                    (((slope_limb_0_col128)
                        * ((pedersen_points_table_window_bits_18_output_limb_1_col73)
                            - (input_limb_17_col17)))
                        + ((slope_limb_1_col129)
                            * ((pedersen_points_table_window_bits_18_output_limb_0_col72)
                                - (input_limb_16_col16)))),
                    ((((slope_limb_0_col128)
                        * ((pedersen_points_table_window_bits_18_output_limb_2_col74)
                            - (input_limb_18_col18)))
                        + ((slope_limb_1_col129)
                            * ((pedersen_points_table_window_bits_18_output_limb_1_col73)
                                - (input_limb_17_col17))))
                        + ((slope_limb_2_col130)
                            * ((pedersen_points_table_window_bits_18_output_limb_0_col72)
                                - (input_limb_16_col16)))),
                    (((((slope_limb_0_col128)
                        * ((pedersen_points_table_window_bits_18_output_limb_3_col75)
                            - (input_limb_19_col19)))
                        + ((slope_limb_1_col129)
                            * ((pedersen_points_table_window_bits_18_output_limb_2_col74)
                                - (input_limb_18_col18))))
                        + ((slope_limb_2_col130)
                            * ((pedersen_points_table_window_bits_18_output_limb_1_col73)
                                - (input_limb_17_col17))))
                        + ((slope_limb_3_col131)
                            * ((pedersen_points_table_window_bits_18_output_limb_0_col72)
                                - (input_limb_16_col16)))),
                    ((((((slope_limb_0_col128)
                        * ((pedersen_points_table_window_bits_18_output_limb_4_col76)
                            - (input_limb_20_col20)))
                        + ((slope_limb_1_col129)
                            * ((pedersen_points_table_window_bits_18_output_limb_3_col75)
                                - (input_limb_19_col19))))
                        + ((slope_limb_2_col130)
                            * ((pedersen_points_table_window_bits_18_output_limb_2_col74)
                                - (input_limb_18_col18))))
                        + ((slope_limb_3_col131)
                            * ((pedersen_points_table_window_bits_18_output_limb_1_col73)
                                - (input_limb_17_col17))))
                        + ((slope_limb_4_col132)
                            * ((pedersen_points_table_window_bits_18_output_limb_0_col72)
                                - (input_limb_16_col16)))),
                    (((((((slope_limb_0_col128)
                        * ((pedersen_points_table_window_bits_18_output_limb_5_col77)
                            - (input_limb_21_col21)))
                        + ((slope_limb_1_col129)
                            * ((pedersen_points_table_window_bits_18_output_limb_4_col76)
                                - (input_limb_20_col20))))
                        + ((slope_limb_2_col130)
                            * ((pedersen_points_table_window_bits_18_output_limb_3_col75)
                                - (input_limb_19_col19))))
                        + ((slope_limb_3_col131)
                            * ((pedersen_points_table_window_bits_18_output_limb_2_col74)
                                - (input_limb_18_col18))))
                        + ((slope_limb_4_col132)
                            * ((pedersen_points_table_window_bits_18_output_limb_1_col73)
                                - (input_limb_17_col17))))
                        + ((slope_limb_5_col133)
                            * ((pedersen_points_table_window_bits_18_output_limb_0_col72)
                                - (input_limb_16_col16)))),
                    ((((((((slope_limb_0_col128)
                        * ((pedersen_points_table_window_bits_18_output_limb_6_col78)
                            - (input_limb_22_col22)))
                        + ((slope_limb_1_col129)
                            * ((pedersen_points_table_window_bits_18_output_limb_5_col77)
                                - (input_limb_21_col21))))
                        + ((slope_limb_2_col130)
                            * ((pedersen_points_table_window_bits_18_output_limb_4_col76)
                                - (input_limb_20_col20))))
                        + ((slope_limb_3_col131)
                            * ((pedersen_points_table_window_bits_18_output_limb_3_col75)
                                - (input_limb_19_col19))))
                        + ((slope_limb_4_col132)
                            * ((pedersen_points_table_window_bits_18_output_limb_2_col74)
                                - (input_limb_18_col18))))
                        + ((slope_limb_5_col133)
                            * ((pedersen_points_table_window_bits_18_output_limb_1_col73)
                                - (input_limb_17_col17))))
                        + ((slope_limb_6_col134)
                            * ((pedersen_points_table_window_bits_18_output_limb_0_col72)
                                - (input_limb_16_col16)))),
                    (((((((slope_limb_1_col129)
                        * ((pedersen_points_table_window_bits_18_output_limb_6_col78)
                            - (input_limb_22_col22)))
                        + ((slope_limb_2_col130)
                            * ((pedersen_points_table_window_bits_18_output_limb_5_col77)
                                - (input_limb_21_col21))))
                        + ((slope_limb_3_col131)
                            * ((pedersen_points_table_window_bits_18_output_limb_4_col76)
                                - (input_limb_20_col20))))
                        + ((slope_limb_4_col132)
                            * ((pedersen_points_table_window_bits_18_output_limb_3_col75)
                                - (input_limb_19_col19))))
                        + ((slope_limb_5_col133)
                            * ((pedersen_points_table_window_bits_18_output_limb_2_col74)
                                - (input_limb_18_col18))))
                        + ((slope_limb_6_col134)
                            * ((pedersen_points_table_window_bits_18_output_limb_1_col73)
                                - (input_limb_17_col17)))),
                    ((((((slope_limb_2_col130)
                        * ((pedersen_points_table_window_bits_18_output_limb_6_col78)
                            - (input_limb_22_col22)))
                        + ((slope_limb_3_col131)
                            * ((pedersen_points_table_window_bits_18_output_limb_5_col77)
                                - (input_limb_21_col21))))
                        + ((slope_limb_4_col132)
                            * ((pedersen_points_table_window_bits_18_output_limb_4_col76)
                                - (input_limb_20_col20))))
                        + ((slope_limb_5_col133)
                            * ((pedersen_points_table_window_bits_18_output_limb_3_col75)
                                - (input_limb_19_col19))))
                        + ((slope_limb_6_col134)
                            * ((pedersen_points_table_window_bits_18_output_limb_2_col74)
                                - (input_limb_18_col18)))),
                    (((((slope_limb_3_col131)
                        * ((pedersen_points_table_window_bits_18_output_limb_6_col78)
                            - (input_limb_22_col22)))
                        + ((slope_limb_4_col132)
                            * ((pedersen_points_table_window_bits_18_output_limb_5_col77)
                                - (input_limb_21_col21))))
                        + ((slope_limb_5_col133)
                            * ((pedersen_points_table_window_bits_18_output_limb_4_col76)
                                - (input_limb_20_col20))))
                        + ((slope_limb_6_col134)
                            * ((pedersen_points_table_window_bits_18_output_limb_3_col75)
                                - (input_limb_19_col19)))),
                    ((((slope_limb_4_col132)
                        * ((pedersen_points_table_window_bits_18_output_limb_6_col78)
                            - (input_limb_22_col22)))
                        + ((slope_limb_5_col133)
                            * ((pedersen_points_table_window_bits_18_output_limb_5_col77)
                                - (input_limb_21_col21))))
                        + ((slope_limb_6_col134)
                            * ((pedersen_points_table_window_bits_18_output_limb_4_col76)
                                - (input_limb_20_col20)))),
                    (((slope_limb_5_col133)
                        * ((pedersen_points_table_window_bits_18_output_limb_6_col78)
                            - (input_limb_22_col22)))
                        + ((slope_limb_6_col134)
                            * ((pedersen_points_table_window_bits_18_output_limb_5_col77)
                                - (input_limb_21_col21)))),
                    ((slope_limb_6_col134)
                        * ((pedersen_points_table_window_bits_18_output_limb_6_col78)
                            - (input_limb_22_col22))),
                ];
                let z2_tmp_3cd4f_3 = [
                    ((slope_limb_7_col135)
                        * ((pedersen_points_table_window_bits_18_output_limb_7_col79)
                            - (input_limb_23_col23))),
                    (((slope_limb_7_col135)
                        * ((pedersen_points_table_window_bits_18_output_limb_8_col80)
                            - (input_limb_24_col24)))
                        + ((slope_limb_8_col136)
                            * ((pedersen_points_table_window_bits_18_output_limb_7_col79)
                                - (input_limb_23_col23)))),
                    ((((slope_limb_7_col135)
                        * ((pedersen_points_table_window_bits_18_output_limb_9_col81)
                            - (input_limb_25_col25)))
                        + ((slope_limb_8_col136)
                            * ((pedersen_points_table_window_bits_18_output_limb_8_col80)
                                - (input_limb_24_col24))))
                        + ((slope_limb_9_col137)
                            * ((pedersen_points_table_window_bits_18_output_limb_7_col79)
                                - (input_limb_23_col23)))),
                    (((((slope_limb_7_col135)
                        * ((pedersen_points_table_window_bits_18_output_limb_10_col82)
                            - (input_limb_26_col26)))
                        + ((slope_limb_8_col136)
                            * ((pedersen_points_table_window_bits_18_output_limb_9_col81)
                                - (input_limb_25_col25))))
                        + ((slope_limb_9_col137)
                            * ((pedersen_points_table_window_bits_18_output_limb_8_col80)
                                - (input_limb_24_col24))))
                        + ((slope_limb_10_col138)
                            * ((pedersen_points_table_window_bits_18_output_limb_7_col79)
                                - (input_limb_23_col23)))),
                    ((((((slope_limb_7_col135)
                        * ((pedersen_points_table_window_bits_18_output_limb_11_col83)
                            - (input_limb_27_col27)))
                        + ((slope_limb_8_col136)
                            * ((pedersen_points_table_window_bits_18_output_limb_10_col82)
                                - (input_limb_26_col26))))
                        + ((slope_limb_9_col137)
                            * ((pedersen_points_table_window_bits_18_output_limb_9_col81)
                                - (input_limb_25_col25))))
                        + ((slope_limb_10_col138)
                            * ((pedersen_points_table_window_bits_18_output_limb_8_col80)
                                - (input_limb_24_col24))))
                        + ((slope_limb_11_col139)
                            * ((pedersen_points_table_window_bits_18_output_limb_7_col79)
                                - (input_limb_23_col23)))),
                    (((((((slope_limb_7_col135)
                        * ((pedersen_points_table_window_bits_18_output_limb_12_col84)
                            - (input_limb_28_col28)))
                        + ((slope_limb_8_col136)
                            * ((pedersen_points_table_window_bits_18_output_limb_11_col83)
                                - (input_limb_27_col27))))
                        + ((slope_limb_9_col137)
                            * ((pedersen_points_table_window_bits_18_output_limb_10_col82)
                                - (input_limb_26_col26))))
                        + ((slope_limb_10_col138)
                            * ((pedersen_points_table_window_bits_18_output_limb_9_col81)
                                - (input_limb_25_col25))))
                        + ((slope_limb_11_col139)
                            * ((pedersen_points_table_window_bits_18_output_limb_8_col80)
                                - (input_limb_24_col24))))
                        + ((slope_limb_12_col140)
                            * ((pedersen_points_table_window_bits_18_output_limb_7_col79)
                                - (input_limb_23_col23)))),
                    ((((((((slope_limb_7_col135)
                        * ((pedersen_points_table_window_bits_18_output_limb_13_col85)
                            - (input_limb_29_col29)))
                        + ((slope_limb_8_col136)
                            * ((pedersen_points_table_window_bits_18_output_limb_12_col84)
                                - (input_limb_28_col28))))
                        + ((slope_limb_9_col137)
                            * ((pedersen_points_table_window_bits_18_output_limb_11_col83)
                                - (input_limb_27_col27))))
                        + ((slope_limb_10_col138)
                            * ((pedersen_points_table_window_bits_18_output_limb_10_col82)
                                - (input_limb_26_col26))))
                        + ((slope_limb_11_col139)
                            * ((pedersen_points_table_window_bits_18_output_limb_9_col81)
                                - (input_limb_25_col25))))
                        + ((slope_limb_12_col140)
                            * ((pedersen_points_table_window_bits_18_output_limb_8_col80)
                                - (input_limb_24_col24))))
                        + ((slope_limb_13_col141)
                            * ((pedersen_points_table_window_bits_18_output_limb_7_col79)
                                - (input_limb_23_col23)))),
                    (((((((slope_limb_8_col136)
                        * ((pedersen_points_table_window_bits_18_output_limb_13_col85)
                            - (input_limb_29_col29)))
                        + ((slope_limb_9_col137)
                            * ((pedersen_points_table_window_bits_18_output_limb_12_col84)
                                - (input_limb_28_col28))))
                        + ((slope_limb_10_col138)
                            * ((pedersen_points_table_window_bits_18_output_limb_11_col83)
                                - (input_limb_27_col27))))
                        + ((slope_limb_11_col139)
                            * ((pedersen_points_table_window_bits_18_output_limb_10_col82)
                                - (input_limb_26_col26))))
                        + ((slope_limb_12_col140)
                            * ((pedersen_points_table_window_bits_18_output_limb_9_col81)
                                - (input_limb_25_col25))))
                        + ((slope_limb_13_col141)
                            * ((pedersen_points_table_window_bits_18_output_limb_8_col80)
                                - (input_limb_24_col24)))),
                    ((((((slope_limb_9_col137)
                        * ((pedersen_points_table_window_bits_18_output_limb_13_col85)
                            - (input_limb_29_col29)))
                        + ((slope_limb_10_col138)
                            * ((pedersen_points_table_window_bits_18_output_limb_12_col84)
                                - (input_limb_28_col28))))
                        + ((slope_limb_11_col139)
                            * ((pedersen_points_table_window_bits_18_output_limb_11_col83)
                                - (input_limb_27_col27))))
                        + ((slope_limb_12_col140)
                            * ((pedersen_points_table_window_bits_18_output_limb_10_col82)
                                - (input_limb_26_col26))))
                        + ((slope_limb_13_col141)
                            * ((pedersen_points_table_window_bits_18_output_limb_9_col81)
                                - (input_limb_25_col25)))),
                    (((((slope_limb_10_col138)
                        * ((pedersen_points_table_window_bits_18_output_limb_13_col85)
                            - (input_limb_29_col29)))
                        + ((slope_limb_11_col139)
                            * ((pedersen_points_table_window_bits_18_output_limb_12_col84)
                                - (input_limb_28_col28))))
                        + ((slope_limb_12_col140)
                            * ((pedersen_points_table_window_bits_18_output_limb_11_col83)
                                - (input_limb_27_col27))))
                        + ((slope_limb_13_col141)
                            * ((pedersen_points_table_window_bits_18_output_limb_10_col82)
                                - (input_limb_26_col26)))),
                    ((((slope_limb_11_col139)
                        * ((pedersen_points_table_window_bits_18_output_limb_13_col85)
                            - (input_limb_29_col29)))
                        + ((slope_limb_12_col140)
                            * ((pedersen_points_table_window_bits_18_output_limb_12_col84)
                                - (input_limb_28_col28))))
                        + ((slope_limb_13_col141)
                            * ((pedersen_points_table_window_bits_18_output_limb_11_col83)
                                - (input_limb_27_col27)))),
                    (((slope_limb_12_col140)
                        * ((pedersen_points_table_window_bits_18_output_limb_13_col85)
                            - (input_limb_29_col29)))
                        + ((slope_limb_13_col141)
                            * ((pedersen_points_table_window_bits_18_output_limb_12_col84)
                                - (input_limb_28_col28)))),
                    ((slope_limb_13_col141)
                        * ((pedersen_points_table_window_bits_18_output_limb_13_col85)
                            - (input_limb_29_col29))),
                ];
                let x_sum_tmp_3cd4f_4 = [
                    ((slope_limb_0_col128) + (slope_limb_7_col135)),
                    ((slope_limb_1_col129) + (slope_limb_8_col136)),
                    ((slope_limb_2_col130) + (slope_limb_9_col137)),
                    ((slope_limb_3_col131) + (slope_limb_10_col138)),
                    ((slope_limb_4_col132) + (slope_limb_11_col139)),
                    ((slope_limb_5_col133) + (slope_limb_12_col140)),
                    ((slope_limb_6_col134) + (slope_limb_13_col141)),
                ];
                let y_sum_tmp_3cd4f_5 = [
                    (((pedersen_points_table_window_bits_18_output_limb_0_col72)
                        - (input_limb_16_col16))
                        + ((pedersen_points_table_window_bits_18_output_limb_7_col79)
                            - (input_limb_23_col23))),
                    (((pedersen_points_table_window_bits_18_output_limb_1_col73)
                        - (input_limb_17_col17))
                        + ((pedersen_points_table_window_bits_18_output_limb_8_col80)
                            - (input_limb_24_col24))),
                    (((pedersen_points_table_window_bits_18_output_limb_2_col74)
                        - (input_limb_18_col18))
                        + ((pedersen_points_table_window_bits_18_output_limb_9_col81)
                            - (input_limb_25_col25))),
                    (((pedersen_points_table_window_bits_18_output_limb_3_col75)
                        - (input_limb_19_col19))
                        + ((pedersen_points_table_window_bits_18_output_limb_10_col82)
                            - (input_limb_26_col26))),
                    (((pedersen_points_table_window_bits_18_output_limb_4_col76)
                        - (input_limb_20_col20))
                        + ((pedersen_points_table_window_bits_18_output_limb_11_col83)
                            - (input_limb_27_col27))),
                    (((pedersen_points_table_window_bits_18_output_limb_5_col77)
                        - (input_limb_21_col21))
                        + ((pedersen_points_table_window_bits_18_output_limb_12_col84)
                            - (input_limb_28_col28))),
                    (((pedersen_points_table_window_bits_18_output_limb_6_col78)
                        - (input_limb_22_col22))
                        + ((pedersen_points_table_window_bits_18_output_limb_13_col85)
                            - (input_limb_29_col29))),
                ];
                let single_karatsuba_n_7_output_tmp_3cd4f_6 = [
                    z0_tmp_3cd4f_2[0],
                    z0_tmp_3cd4f_2[1],
                    z0_tmp_3cd4f_2[2],
                    z0_tmp_3cd4f_2[3],
                    z0_tmp_3cd4f_2[4],
                    z0_tmp_3cd4f_2[5],
                    z0_tmp_3cd4f_2[6],
                    ((z0_tmp_3cd4f_2[7])
                        + ((((x_sum_tmp_3cd4f_4[0]) * (y_sum_tmp_3cd4f_5[0]))
                            - (z0_tmp_3cd4f_2[0]))
                            - (z2_tmp_3cd4f_3[0]))),
                    ((z0_tmp_3cd4f_2[8])
                        + (((((x_sum_tmp_3cd4f_4[0]) * (y_sum_tmp_3cd4f_5[1]))
                            + ((x_sum_tmp_3cd4f_4[1]) * (y_sum_tmp_3cd4f_5[0])))
                            - (z0_tmp_3cd4f_2[1]))
                            - (z2_tmp_3cd4f_3[1]))),
                    ((z0_tmp_3cd4f_2[9])
                        + ((((((x_sum_tmp_3cd4f_4[0]) * (y_sum_tmp_3cd4f_5[2]))
                            + ((x_sum_tmp_3cd4f_4[1]) * (y_sum_tmp_3cd4f_5[1])))
                            + ((x_sum_tmp_3cd4f_4[2]) * (y_sum_tmp_3cd4f_5[0])))
                            - (z0_tmp_3cd4f_2[2]))
                            - (z2_tmp_3cd4f_3[2]))),
                    ((z0_tmp_3cd4f_2[10])
                        + (((((((x_sum_tmp_3cd4f_4[0]) * (y_sum_tmp_3cd4f_5[3]))
                            + ((x_sum_tmp_3cd4f_4[1]) * (y_sum_tmp_3cd4f_5[2])))
                            + ((x_sum_tmp_3cd4f_4[2]) * (y_sum_tmp_3cd4f_5[1])))
                            + ((x_sum_tmp_3cd4f_4[3]) * (y_sum_tmp_3cd4f_5[0])))
                            - (z0_tmp_3cd4f_2[3]))
                            - (z2_tmp_3cd4f_3[3]))),
                    ((z0_tmp_3cd4f_2[11])
                        + ((((((((x_sum_tmp_3cd4f_4[0]) * (y_sum_tmp_3cd4f_5[4]))
                            + ((x_sum_tmp_3cd4f_4[1]) * (y_sum_tmp_3cd4f_5[3])))
                            + ((x_sum_tmp_3cd4f_4[2]) * (y_sum_tmp_3cd4f_5[2])))
                            + ((x_sum_tmp_3cd4f_4[3]) * (y_sum_tmp_3cd4f_5[1])))
                            + ((x_sum_tmp_3cd4f_4[4]) * (y_sum_tmp_3cd4f_5[0])))
                            - (z0_tmp_3cd4f_2[4]))
                            - (z2_tmp_3cd4f_3[4]))),
                    ((z0_tmp_3cd4f_2[12])
                        + (((((((((x_sum_tmp_3cd4f_4[0]) * (y_sum_tmp_3cd4f_5[5]))
                            + ((x_sum_tmp_3cd4f_4[1]) * (y_sum_tmp_3cd4f_5[4])))
                            + ((x_sum_tmp_3cd4f_4[2]) * (y_sum_tmp_3cd4f_5[3])))
                            + ((x_sum_tmp_3cd4f_4[3]) * (y_sum_tmp_3cd4f_5[2])))
                            + ((x_sum_tmp_3cd4f_4[4]) * (y_sum_tmp_3cd4f_5[1])))
                            + ((x_sum_tmp_3cd4f_4[5]) * (y_sum_tmp_3cd4f_5[0])))
                            - (z0_tmp_3cd4f_2[5]))
                            - (z2_tmp_3cd4f_3[5]))),
                    ((((((((((x_sum_tmp_3cd4f_4[0]) * (y_sum_tmp_3cd4f_5[6]))
                        + ((x_sum_tmp_3cd4f_4[1]) * (y_sum_tmp_3cd4f_5[5])))
                        + ((x_sum_tmp_3cd4f_4[2]) * (y_sum_tmp_3cd4f_5[4])))
                        + ((x_sum_tmp_3cd4f_4[3]) * (y_sum_tmp_3cd4f_5[3])))
                        + ((x_sum_tmp_3cd4f_4[4]) * (y_sum_tmp_3cd4f_5[2])))
                        + ((x_sum_tmp_3cd4f_4[5]) * (y_sum_tmp_3cd4f_5[1])))
                        + ((x_sum_tmp_3cd4f_4[6]) * (y_sum_tmp_3cd4f_5[0])))
                        - (z0_tmp_3cd4f_2[6]))
                        - (z2_tmp_3cd4f_3[6])),
                    ((z2_tmp_3cd4f_3[0])
                        + (((((((((x_sum_tmp_3cd4f_4[1]) * (y_sum_tmp_3cd4f_5[6]))
                            + ((x_sum_tmp_3cd4f_4[2]) * (y_sum_tmp_3cd4f_5[5])))
                            + ((x_sum_tmp_3cd4f_4[3]) * (y_sum_tmp_3cd4f_5[4])))
                            + ((x_sum_tmp_3cd4f_4[4]) * (y_sum_tmp_3cd4f_5[3])))
                            + ((x_sum_tmp_3cd4f_4[5]) * (y_sum_tmp_3cd4f_5[2])))
                            + ((x_sum_tmp_3cd4f_4[6]) * (y_sum_tmp_3cd4f_5[1])))
                            - (z0_tmp_3cd4f_2[7]))
                            - (z2_tmp_3cd4f_3[7]))),
                    ((z2_tmp_3cd4f_3[1])
                        + ((((((((x_sum_tmp_3cd4f_4[2]) * (y_sum_tmp_3cd4f_5[6]))
                            + ((x_sum_tmp_3cd4f_4[3]) * (y_sum_tmp_3cd4f_5[5])))
                            + ((x_sum_tmp_3cd4f_4[4]) * (y_sum_tmp_3cd4f_5[4])))
                            + ((x_sum_tmp_3cd4f_4[5]) * (y_sum_tmp_3cd4f_5[3])))
                            + ((x_sum_tmp_3cd4f_4[6]) * (y_sum_tmp_3cd4f_5[2])))
                            - (z0_tmp_3cd4f_2[8]))
                            - (z2_tmp_3cd4f_3[8]))),
                    ((z2_tmp_3cd4f_3[2])
                        + (((((((x_sum_tmp_3cd4f_4[3]) * (y_sum_tmp_3cd4f_5[6]))
                            + ((x_sum_tmp_3cd4f_4[4]) * (y_sum_tmp_3cd4f_5[5])))
                            + ((x_sum_tmp_3cd4f_4[5]) * (y_sum_tmp_3cd4f_5[4])))
                            + ((x_sum_tmp_3cd4f_4[6]) * (y_sum_tmp_3cd4f_5[3])))
                            - (z0_tmp_3cd4f_2[9]))
                            - (z2_tmp_3cd4f_3[9]))),
                    ((z2_tmp_3cd4f_3[3])
                        + ((((((x_sum_tmp_3cd4f_4[4]) * (y_sum_tmp_3cd4f_5[6]))
                            + ((x_sum_tmp_3cd4f_4[5]) * (y_sum_tmp_3cd4f_5[5])))
                            + ((x_sum_tmp_3cd4f_4[6]) * (y_sum_tmp_3cd4f_5[4])))
                            - (z0_tmp_3cd4f_2[10]))
                            - (z2_tmp_3cd4f_3[10]))),
                    ((z2_tmp_3cd4f_3[4])
                        + (((((x_sum_tmp_3cd4f_4[5]) * (y_sum_tmp_3cd4f_5[6]))
                            + ((x_sum_tmp_3cd4f_4[6]) * (y_sum_tmp_3cd4f_5[5])))
                            - (z0_tmp_3cd4f_2[11]))
                            - (z2_tmp_3cd4f_3[11]))),
                    ((z2_tmp_3cd4f_3[5])
                        + ((((x_sum_tmp_3cd4f_4[6]) * (y_sum_tmp_3cd4f_5[6]))
                            - (z0_tmp_3cd4f_2[12]))
                            - (z2_tmp_3cd4f_3[12]))),
                    z2_tmp_3cd4f_3[6],
                    z2_tmp_3cd4f_3[7],
                    z2_tmp_3cd4f_3[8],
                    z2_tmp_3cd4f_3[9],
                    z2_tmp_3cd4f_3[10],
                    z2_tmp_3cd4f_3[11],
                    z2_tmp_3cd4f_3[12],
                ];

                // Single Karatsuba N 7.

                let z0_tmp_3cd4f_7 = [
                    ((slope_limb_14_col142)
                        * ((pedersen_points_table_window_bits_18_output_limb_14_col86)
                            - (input_limb_30_col30))),
                    (((slope_limb_14_col142)
                        * ((pedersen_points_table_window_bits_18_output_limb_15_col87)
                            - (input_limb_31_col31)))
                        + ((slope_limb_15_col143)
                            * ((pedersen_points_table_window_bits_18_output_limb_14_col86)
                                - (input_limb_30_col30)))),
                    ((((slope_limb_14_col142)
                        * ((pedersen_points_table_window_bits_18_output_limb_16_col88)
                            - (input_limb_32_col32)))
                        + ((slope_limb_15_col143)
                            * ((pedersen_points_table_window_bits_18_output_limb_15_col87)
                                - (input_limb_31_col31))))
                        + ((slope_limb_16_col144)
                            * ((pedersen_points_table_window_bits_18_output_limb_14_col86)
                                - (input_limb_30_col30)))),
                    (((((slope_limb_14_col142)
                        * ((pedersen_points_table_window_bits_18_output_limb_17_col89)
                            - (input_limb_33_col33)))
                        + ((slope_limb_15_col143)
                            * ((pedersen_points_table_window_bits_18_output_limb_16_col88)
                                - (input_limb_32_col32))))
                        + ((slope_limb_16_col144)
                            * ((pedersen_points_table_window_bits_18_output_limb_15_col87)
                                - (input_limb_31_col31))))
                        + ((slope_limb_17_col145)
                            * ((pedersen_points_table_window_bits_18_output_limb_14_col86)
                                - (input_limb_30_col30)))),
                    ((((((slope_limb_14_col142)
                        * ((pedersen_points_table_window_bits_18_output_limb_18_col90)
                            - (input_limb_34_col34)))
                        + ((slope_limb_15_col143)
                            * ((pedersen_points_table_window_bits_18_output_limb_17_col89)
                                - (input_limb_33_col33))))
                        + ((slope_limb_16_col144)
                            * ((pedersen_points_table_window_bits_18_output_limb_16_col88)
                                - (input_limb_32_col32))))
                        + ((slope_limb_17_col145)
                            * ((pedersen_points_table_window_bits_18_output_limb_15_col87)
                                - (input_limb_31_col31))))
                        + ((slope_limb_18_col146)
                            * ((pedersen_points_table_window_bits_18_output_limb_14_col86)
                                - (input_limb_30_col30)))),
                    (((((((slope_limb_14_col142)
                        * ((pedersen_points_table_window_bits_18_output_limb_19_col91)
                            - (input_limb_35_col35)))
                        + ((slope_limb_15_col143)
                            * ((pedersen_points_table_window_bits_18_output_limb_18_col90)
                                - (input_limb_34_col34))))
                        + ((slope_limb_16_col144)
                            * ((pedersen_points_table_window_bits_18_output_limb_17_col89)
                                - (input_limb_33_col33))))
                        + ((slope_limb_17_col145)
                            * ((pedersen_points_table_window_bits_18_output_limb_16_col88)
                                - (input_limb_32_col32))))
                        + ((slope_limb_18_col146)
                            * ((pedersen_points_table_window_bits_18_output_limb_15_col87)
                                - (input_limb_31_col31))))
                        + ((slope_limb_19_col147)
                            * ((pedersen_points_table_window_bits_18_output_limb_14_col86)
                                - (input_limb_30_col30)))),
                    ((((((((slope_limb_14_col142)
                        * ((pedersen_points_table_window_bits_18_output_limb_20_col92)
                            - (input_limb_36_col36)))
                        + ((slope_limb_15_col143)
                            * ((pedersen_points_table_window_bits_18_output_limb_19_col91)
                                - (input_limb_35_col35))))
                        + ((slope_limb_16_col144)
                            * ((pedersen_points_table_window_bits_18_output_limb_18_col90)
                                - (input_limb_34_col34))))
                        + ((slope_limb_17_col145)
                            * ((pedersen_points_table_window_bits_18_output_limb_17_col89)
                                - (input_limb_33_col33))))
                        + ((slope_limb_18_col146)
                            * ((pedersen_points_table_window_bits_18_output_limb_16_col88)
                                - (input_limb_32_col32))))
                        + ((slope_limb_19_col147)
                            * ((pedersen_points_table_window_bits_18_output_limb_15_col87)
                                - (input_limb_31_col31))))
                        + ((slope_limb_20_col148)
                            * ((pedersen_points_table_window_bits_18_output_limb_14_col86)
                                - (input_limb_30_col30)))),
                    (((((((slope_limb_15_col143)
                        * ((pedersen_points_table_window_bits_18_output_limb_20_col92)
                            - (input_limb_36_col36)))
                        + ((slope_limb_16_col144)
                            * ((pedersen_points_table_window_bits_18_output_limb_19_col91)
                                - (input_limb_35_col35))))
                        + ((slope_limb_17_col145)
                            * ((pedersen_points_table_window_bits_18_output_limb_18_col90)
                                - (input_limb_34_col34))))
                        + ((slope_limb_18_col146)
                            * ((pedersen_points_table_window_bits_18_output_limb_17_col89)
                                - (input_limb_33_col33))))
                        + ((slope_limb_19_col147)
                            * ((pedersen_points_table_window_bits_18_output_limb_16_col88)
                                - (input_limb_32_col32))))
                        + ((slope_limb_20_col148)
                            * ((pedersen_points_table_window_bits_18_output_limb_15_col87)
                                - (input_limb_31_col31)))),
                    ((((((slope_limb_16_col144)
                        * ((pedersen_points_table_window_bits_18_output_limb_20_col92)
                            - (input_limb_36_col36)))
                        + ((slope_limb_17_col145)
                            * ((pedersen_points_table_window_bits_18_output_limb_19_col91)
                                - (input_limb_35_col35))))
                        + ((slope_limb_18_col146)
                            * ((pedersen_points_table_window_bits_18_output_limb_18_col90)
                                - (input_limb_34_col34))))
                        + ((slope_limb_19_col147)
                            * ((pedersen_points_table_window_bits_18_output_limb_17_col89)
                                - (input_limb_33_col33))))
                        + ((slope_limb_20_col148)
                            * ((pedersen_points_table_window_bits_18_output_limb_16_col88)
                                - (input_limb_32_col32)))),
                    (((((slope_limb_17_col145)
                        * ((pedersen_points_table_window_bits_18_output_limb_20_col92)
                            - (input_limb_36_col36)))
                        + ((slope_limb_18_col146)
                            * ((pedersen_points_table_window_bits_18_output_limb_19_col91)
                                - (input_limb_35_col35))))
                        + ((slope_limb_19_col147)
                            * ((pedersen_points_table_window_bits_18_output_limb_18_col90)
                                - (input_limb_34_col34))))
                        + ((slope_limb_20_col148)
                            * ((pedersen_points_table_window_bits_18_output_limb_17_col89)
                                - (input_limb_33_col33)))),
                    ((((slope_limb_18_col146)
                        * ((pedersen_points_table_window_bits_18_output_limb_20_col92)
                            - (input_limb_36_col36)))
                        + ((slope_limb_19_col147)
                            * ((pedersen_points_table_window_bits_18_output_limb_19_col91)
                                - (input_limb_35_col35))))
                        + ((slope_limb_20_col148)
                            * ((pedersen_points_table_window_bits_18_output_limb_18_col90)
                                - (input_limb_34_col34)))),
                    (((slope_limb_19_col147)
                        * ((pedersen_points_table_window_bits_18_output_limb_20_col92)
                            - (input_limb_36_col36)))
                        + ((slope_limb_20_col148)
                            * ((pedersen_points_table_window_bits_18_output_limb_19_col91)
                                - (input_limb_35_col35)))),
                    ((slope_limb_20_col148)
                        * ((pedersen_points_table_window_bits_18_output_limb_20_col92)
                            - (input_limb_36_col36))),
                ];
                let z2_tmp_3cd4f_8 = [
                    ((slope_limb_21_col149)
                        * ((pedersen_points_table_window_bits_18_output_limb_21_col93)
                            - (input_limb_37_col37))),
                    (((slope_limb_21_col149)
                        * ((pedersen_points_table_window_bits_18_output_limb_22_col94)
                            - (input_limb_38_col38)))
                        + ((slope_limb_22_col150)
                            * ((pedersen_points_table_window_bits_18_output_limb_21_col93)
                                - (input_limb_37_col37)))),
                    ((((slope_limb_21_col149)
                        * ((pedersen_points_table_window_bits_18_output_limb_23_col95)
                            - (input_limb_39_col39)))
                        + ((slope_limb_22_col150)
                            * ((pedersen_points_table_window_bits_18_output_limb_22_col94)
                                - (input_limb_38_col38))))
                        + ((slope_limb_23_col151)
                            * ((pedersen_points_table_window_bits_18_output_limb_21_col93)
                                - (input_limb_37_col37)))),
                    (((((slope_limb_21_col149)
                        * ((pedersen_points_table_window_bits_18_output_limb_24_col96)
                            - (input_limb_40_col40)))
                        + ((slope_limb_22_col150)
                            * ((pedersen_points_table_window_bits_18_output_limb_23_col95)
                                - (input_limb_39_col39))))
                        + ((slope_limb_23_col151)
                            * ((pedersen_points_table_window_bits_18_output_limb_22_col94)
                                - (input_limb_38_col38))))
                        + ((slope_limb_24_col152)
                            * ((pedersen_points_table_window_bits_18_output_limb_21_col93)
                                - (input_limb_37_col37)))),
                    ((((((slope_limb_21_col149)
                        * ((pedersen_points_table_window_bits_18_output_limb_25_col97)
                            - (input_limb_41_col41)))
                        + ((slope_limb_22_col150)
                            * ((pedersen_points_table_window_bits_18_output_limb_24_col96)
                                - (input_limb_40_col40))))
                        + ((slope_limb_23_col151)
                            * ((pedersen_points_table_window_bits_18_output_limb_23_col95)
                                - (input_limb_39_col39))))
                        + ((slope_limb_24_col152)
                            * ((pedersen_points_table_window_bits_18_output_limb_22_col94)
                                - (input_limb_38_col38))))
                        + ((slope_limb_25_col153)
                            * ((pedersen_points_table_window_bits_18_output_limb_21_col93)
                                - (input_limb_37_col37)))),
                    (((((((slope_limb_21_col149)
                        * ((pedersen_points_table_window_bits_18_output_limb_26_col98)
                            - (input_limb_42_col42)))
                        + ((slope_limb_22_col150)
                            * ((pedersen_points_table_window_bits_18_output_limb_25_col97)
                                - (input_limb_41_col41))))
                        + ((slope_limb_23_col151)
                            * ((pedersen_points_table_window_bits_18_output_limb_24_col96)
                                - (input_limb_40_col40))))
                        + ((slope_limb_24_col152)
                            * ((pedersen_points_table_window_bits_18_output_limb_23_col95)
                                - (input_limb_39_col39))))
                        + ((slope_limb_25_col153)
                            * ((pedersen_points_table_window_bits_18_output_limb_22_col94)
                                - (input_limb_38_col38))))
                        + ((slope_limb_26_col154)
                            * ((pedersen_points_table_window_bits_18_output_limb_21_col93)
                                - (input_limb_37_col37)))),
                    ((((((((slope_limb_21_col149)
                        * ((pedersen_points_table_window_bits_18_output_limb_27_col99)
                            - (input_limb_43_col43)))
                        + ((slope_limb_22_col150)
                            * ((pedersen_points_table_window_bits_18_output_limb_26_col98)
                                - (input_limb_42_col42))))
                        + ((slope_limb_23_col151)
                            * ((pedersen_points_table_window_bits_18_output_limb_25_col97)
                                - (input_limb_41_col41))))
                        + ((slope_limb_24_col152)
                            * ((pedersen_points_table_window_bits_18_output_limb_24_col96)
                                - (input_limb_40_col40))))
                        + ((slope_limb_25_col153)
                            * ((pedersen_points_table_window_bits_18_output_limb_23_col95)
                                - (input_limb_39_col39))))
                        + ((slope_limb_26_col154)
                            * ((pedersen_points_table_window_bits_18_output_limb_22_col94)
                                - (input_limb_38_col38))))
                        + ((slope_limb_27_col155)
                            * ((pedersen_points_table_window_bits_18_output_limb_21_col93)
                                - (input_limb_37_col37)))),
                    (((((((slope_limb_22_col150)
                        * ((pedersen_points_table_window_bits_18_output_limb_27_col99)
                            - (input_limb_43_col43)))
                        + ((slope_limb_23_col151)
                            * ((pedersen_points_table_window_bits_18_output_limb_26_col98)
                                - (input_limb_42_col42))))
                        + ((slope_limb_24_col152)
                            * ((pedersen_points_table_window_bits_18_output_limb_25_col97)
                                - (input_limb_41_col41))))
                        + ((slope_limb_25_col153)
                            * ((pedersen_points_table_window_bits_18_output_limb_24_col96)
                                - (input_limb_40_col40))))
                        + ((slope_limb_26_col154)
                            * ((pedersen_points_table_window_bits_18_output_limb_23_col95)
                                - (input_limb_39_col39))))
                        + ((slope_limb_27_col155)
                            * ((pedersen_points_table_window_bits_18_output_limb_22_col94)
                                - (input_limb_38_col38)))),
                    ((((((slope_limb_23_col151)
                        * ((pedersen_points_table_window_bits_18_output_limb_27_col99)
                            - (input_limb_43_col43)))
                        + ((slope_limb_24_col152)
                            * ((pedersen_points_table_window_bits_18_output_limb_26_col98)
                                - (input_limb_42_col42))))
                        + ((slope_limb_25_col153)
                            * ((pedersen_points_table_window_bits_18_output_limb_25_col97)
                                - (input_limb_41_col41))))
                        + ((slope_limb_26_col154)
                            * ((pedersen_points_table_window_bits_18_output_limb_24_col96)
                                - (input_limb_40_col40))))
                        + ((slope_limb_27_col155)
                            * ((pedersen_points_table_window_bits_18_output_limb_23_col95)
                                - (input_limb_39_col39)))),
                    (((((slope_limb_24_col152)
                        * ((pedersen_points_table_window_bits_18_output_limb_27_col99)
                            - (input_limb_43_col43)))
                        + ((slope_limb_25_col153)
                            * ((pedersen_points_table_window_bits_18_output_limb_26_col98)
                                - (input_limb_42_col42))))
                        + ((slope_limb_26_col154)
                            * ((pedersen_points_table_window_bits_18_output_limb_25_col97)
                                - (input_limb_41_col41))))
                        + ((slope_limb_27_col155)
                            * ((pedersen_points_table_window_bits_18_output_limb_24_col96)
                                - (input_limb_40_col40)))),
                    ((((slope_limb_25_col153)
                        * ((pedersen_points_table_window_bits_18_output_limb_27_col99)
                            - (input_limb_43_col43)))
                        + ((slope_limb_26_col154)
                            * ((pedersen_points_table_window_bits_18_output_limb_26_col98)
                                - (input_limb_42_col42))))
                        + ((slope_limb_27_col155)
                            * ((pedersen_points_table_window_bits_18_output_limb_25_col97)
                                - (input_limb_41_col41)))),
                    (((slope_limb_26_col154)
                        * ((pedersen_points_table_window_bits_18_output_limb_27_col99)
                            - (input_limb_43_col43)))
                        + ((slope_limb_27_col155)
                            * ((pedersen_points_table_window_bits_18_output_limb_26_col98)
                                - (input_limb_42_col42)))),
                    ((slope_limb_27_col155)
                        * ((pedersen_points_table_window_bits_18_output_limb_27_col99)
                            - (input_limb_43_col43))),
                ];
                let x_sum_tmp_3cd4f_9 = [
                    ((slope_limb_14_col142) + (slope_limb_21_col149)),
                    ((slope_limb_15_col143) + (slope_limb_22_col150)),
                    ((slope_limb_16_col144) + (slope_limb_23_col151)),
                    ((slope_limb_17_col145) + (slope_limb_24_col152)),
                    ((slope_limb_18_col146) + (slope_limb_25_col153)),
                    ((slope_limb_19_col147) + (slope_limb_26_col154)),
                    ((slope_limb_20_col148) + (slope_limb_27_col155)),
                ];
                let y_sum_tmp_3cd4f_10 = [
                    (((pedersen_points_table_window_bits_18_output_limb_14_col86)
                        - (input_limb_30_col30))
                        + ((pedersen_points_table_window_bits_18_output_limb_21_col93)
                            - (input_limb_37_col37))),
                    (((pedersen_points_table_window_bits_18_output_limb_15_col87)
                        - (input_limb_31_col31))
                        + ((pedersen_points_table_window_bits_18_output_limb_22_col94)
                            - (input_limb_38_col38))),
                    (((pedersen_points_table_window_bits_18_output_limb_16_col88)
                        - (input_limb_32_col32))
                        + ((pedersen_points_table_window_bits_18_output_limb_23_col95)
                            - (input_limb_39_col39))),
                    (((pedersen_points_table_window_bits_18_output_limb_17_col89)
                        - (input_limb_33_col33))
                        + ((pedersen_points_table_window_bits_18_output_limb_24_col96)
                            - (input_limb_40_col40))),
                    (((pedersen_points_table_window_bits_18_output_limb_18_col90)
                        - (input_limb_34_col34))
                        + ((pedersen_points_table_window_bits_18_output_limb_25_col97)
                            - (input_limb_41_col41))),
                    (((pedersen_points_table_window_bits_18_output_limb_19_col91)
                        - (input_limb_35_col35))
                        + ((pedersen_points_table_window_bits_18_output_limb_26_col98)
                            - (input_limb_42_col42))),
                    (((pedersen_points_table_window_bits_18_output_limb_20_col92)
                        - (input_limb_36_col36))
                        + ((pedersen_points_table_window_bits_18_output_limb_27_col99)
                            - (input_limb_43_col43))),
                ];
                let single_karatsuba_n_7_output_tmp_3cd4f_11 = [
                    z0_tmp_3cd4f_7[0],
                    z0_tmp_3cd4f_7[1],
                    z0_tmp_3cd4f_7[2],
                    z0_tmp_3cd4f_7[3],
                    z0_tmp_3cd4f_7[4],
                    z0_tmp_3cd4f_7[5],
                    z0_tmp_3cd4f_7[6],
                    ((z0_tmp_3cd4f_7[7])
                        + ((((x_sum_tmp_3cd4f_9[0]) * (y_sum_tmp_3cd4f_10[0]))
                            - (z0_tmp_3cd4f_7[0]))
                            - (z2_tmp_3cd4f_8[0]))),
                    ((z0_tmp_3cd4f_7[8])
                        + (((((x_sum_tmp_3cd4f_9[0]) * (y_sum_tmp_3cd4f_10[1]))
                            + ((x_sum_tmp_3cd4f_9[1]) * (y_sum_tmp_3cd4f_10[0])))
                            - (z0_tmp_3cd4f_7[1]))
                            - (z2_tmp_3cd4f_8[1]))),
                    ((z0_tmp_3cd4f_7[9])
                        + ((((((x_sum_tmp_3cd4f_9[0]) * (y_sum_tmp_3cd4f_10[2]))
                            + ((x_sum_tmp_3cd4f_9[1]) * (y_sum_tmp_3cd4f_10[1])))
                            + ((x_sum_tmp_3cd4f_9[2]) * (y_sum_tmp_3cd4f_10[0])))
                            - (z0_tmp_3cd4f_7[2]))
                            - (z2_tmp_3cd4f_8[2]))),
                    ((z0_tmp_3cd4f_7[10])
                        + (((((((x_sum_tmp_3cd4f_9[0]) * (y_sum_tmp_3cd4f_10[3]))
                            + ((x_sum_tmp_3cd4f_9[1]) * (y_sum_tmp_3cd4f_10[2])))
                            + ((x_sum_tmp_3cd4f_9[2]) * (y_sum_tmp_3cd4f_10[1])))
                            + ((x_sum_tmp_3cd4f_9[3]) * (y_sum_tmp_3cd4f_10[0])))
                            - (z0_tmp_3cd4f_7[3]))
                            - (z2_tmp_3cd4f_8[3]))),
                    ((z0_tmp_3cd4f_7[11])
                        + ((((((((x_sum_tmp_3cd4f_9[0]) * (y_sum_tmp_3cd4f_10[4]))
                            + ((x_sum_tmp_3cd4f_9[1]) * (y_sum_tmp_3cd4f_10[3])))
                            + ((x_sum_tmp_3cd4f_9[2]) * (y_sum_tmp_3cd4f_10[2])))
                            + ((x_sum_tmp_3cd4f_9[3]) * (y_sum_tmp_3cd4f_10[1])))
                            + ((x_sum_tmp_3cd4f_9[4]) * (y_sum_tmp_3cd4f_10[0])))
                            - (z0_tmp_3cd4f_7[4]))
                            - (z2_tmp_3cd4f_8[4]))),
                    ((z0_tmp_3cd4f_7[12])
                        + (((((((((x_sum_tmp_3cd4f_9[0]) * (y_sum_tmp_3cd4f_10[5]))
                            + ((x_sum_tmp_3cd4f_9[1]) * (y_sum_tmp_3cd4f_10[4])))
                            + ((x_sum_tmp_3cd4f_9[2]) * (y_sum_tmp_3cd4f_10[3])))
                            + ((x_sum_tmp_3cd4f_9[3]) * (y_sum_tmp_3cd4f_10[2])))
                            + ((x_sum_tmp_3cd4f_9[4]) * (y_sum_tmp_3cd4f_10[1])))
                            + ((x_sum_tmp_3cd4f_9[5]) * (y_sum_tmp_3cd4f_10[0])))
                            - (z0_tmp_3cd4f_7[5]))
                            - (z2_tmp_3cd4f_8[5]))),
                    ((((((((((x_sum_tmp_3cd4f_9[0]) * (y_sum_tmp_3cd4f_10[6]))
                        + ((x_sum_tmp_3cd4f_9[1]) * (y_sum_tmp_3cd4f_10[5])))
                        + ((x_sum_tmp_3cd4f_9[2]) * (y_sum_tmp_3cd4f_10[4])))
                        + ((x_sum_tmp_3cd4f_9[3]) * (y_sum_tmp_3cd4f_10[3])))
                        + ((x_sum_tmp_3cd4f_9[4]) * (y_sum_tmp_3cd4f_10[2])))
                        + ((x_sum_tmp_3cd4f_9[5]) * (y_sum_tmp_3cd4f_10[1])))
                        + ((x_sum_tmp_3cd4f_9[6]) * (y_sum_tmp_3cd4f_10[0])))
                        - (z0_tmp_3cd4f_7[6]))
                        - (z2_tmp_3cd4f_8[6])),
                    ((z2_tmp_3cd4f_8[0])
                        + (((((((((x_sum_tmp_3cd4f_9[1]) * (y_sum_tmp_3cd4f_10[6]))
                            + ((x_sum_tmp_3cd4f_9[2]) * (y_sum_tmp_3cd4f_10[5])))
                            + ((x_sum_tmp_3cd4f_9[3]) * (y_sum_tmp_3cd4f_10[4])))
                            + ((x_sum_tmp_3cd4f_9[4]) * (y_sum_tmp_3cd4f_10[3])))
                            + ((x_sum_tmp_3cd4f_9[5]) * (y_sum_tmp_3cd4f_10[2])))
                            + ((x_sum_tmp_3cd4f_9[6]) * (y_sum_tmp_3cd4f_10[1])))
                            - (z0_tmp_3cd4f_7[7]))
                            - (z2_tmp_3cd4f_8[7]))),
                    ((z2_tmp_3cd4f_8[1])
                        + ((((((((x_sum_tmp_3cd4f_9[2]) * (y_sum_tmp_3cd4f_10[6]))
                            + ((x_sum_tmp_3cd4f_9[3]) * (y_sum_tmp_3cd4f_10[5])))
                            + ((x_sum_tmp_3cd4f_9[4]) * (y_sum_tmp_3cd4f_10[4])))
                            + ((x_sum_tmp_3cd4f_9[5]) * (y_sum_tmp_3cd4f_10[3])))
                            + ((x_sum_tmp_3cd4f_9[6]) * (y_sum_tmp_3cd4f_10[2])))
                            - (z0_tmp_3cd4f_7[8]))
                            - (z2_tmp_3cd4f_8[8]))),
                    ((z2_tmp_3cd4f_8[2])
                        + (((((((x_sum_tmp_3cd4f_9[3]) * (y_sum_tmp_3cd4f_10[6]))
                            + ((x_sum_tmp_3cd4f_9[4]) * (y_sum_tmp_3cd4f_10[5])))
                            + ((x_sum_tmp_3cd4f_9[5]) * (y_sum_tmp_3cd4f_10[4])))
                            + ((x_sum_tmp_3cd4f_9[6]) * (y_sum_tmp_3cd4f_10[3])))
                            - (z0_tmp_3cd4f_7[9]))
                            - (z2_tmp_3cd4f_8[9]))),
                    ((z2_tmp_3cd4f_8[3])
                        + ((((((x_sum_tmp_3cd4f_9[4]) * (y_sum_tmp_3cd4f_10[6]))
                            + ((x_sum_tmp_3cd4f_9[5]) * (y_sum_tmp_3cd4f_10[5])))
                            + ((x_sum_tmp_3cd4f_9[6]) * (y_sum_tmp_3cd4f_10[4])))
                            - (z0_tmp_3cd4f_7[10]))
                            - (z2_tmp_3cd4f_8[10]))),
                    ((z2_tmp_3cd4f_8[4])
                        + (((((x_sum_tmp_3cd4f_9[5]) * (y_sum_tmp_3cd4f_10[6]))
                            + ((x_sum_tmp_3cd4f_9[6]) * (y_sum_tmp_3cd4f_10[5])))
                            - (z0_tmp_3cd4f_7[11]))
                            - (z2_tmp_3cd4f_8[11]))),
                    ((z2_tmp_3cd4f_8[5])
                        + ((((x_sum_tmp_3cd4f_9[6]) * (y_sum_tmp_3cd4f_10[6]))
                            - (z0_tmp_3cd4f_7[12]))
                            - (z2_tmp_3cd4f_8[12]))),
                    z2_tmp_3cd4f_8[6],
                    z2_tmp_3cd4f_8[7],
                    z2_tmp_3cd4f_8[8],
                    z2_tmp_3cd4f_8[9],
                    z2_tmp_3cd4f_8[10],
                    z2_tmp_3cd4f_8[11],
                    z2_tmp_3cd4f_8[12],
                ];

                let x_sum_tmp_3cd4f_12 = [
                    ((slope_limb_0_col128) + (slope_limb_14_col142)),
                    ((slope_limb_1_col129) + (slope_limb_15_col143)),
                    ((slope_limb_2_col130) + (slope_limb_16_col144)),
                    ((slope_limb_3_col131) + (slope_limb_17_col145)),
                    ((slope_limb_4_col132) + (slope_limb_18_col146)),
                    ((slope_limb_5_col133) + (slope_limb_19_col147)),
                    ((slope_limb_6_col134) + (slope_limb_20_col148)),
                    ((slope_limb_7_col135) + (slope_limb_21_col149)),
                    ((slope_limb_8_col136) + (slope_limb_22_col150)),
                    ((slope_limb_9_col137) + (slope_limb_23_col151)),
                    ((slope_limb_10_col138) + (slope_limb_24_col152)),
                    ((slope_limb_11_col139) + (slope_limb_25_col153)),
                    ((slope_limb_12_col140) + (slope_limb_26_col154)),
                    ((slope_limb_13_col141) + (slope_limb_27_col155)),
                ];
                let y_sum_tmp_3cd4f_13 = [
                    (((pedersen_points_table_window_bits_18_output_limb_0_col72)
                        - (input_limb_16_col16))
                        + ((pedersen_points_table_window_bits_18_output_limb_14_col86)
                            - (input_limb_30_col30))),
                    (((pedersen_points_table_window_bits_18_output_limb_1_col73)
                        - (input_limb_17_col17))
                        + ((pedersen_points_table_window_bits_18_output_limb_15_col87)
                            - (input_limb_31_col31))),
                    (((pedersen_points_table_window_bits_18_output_limb_2_col74)
                        - (input_limb_18_col18))
                        + ((pedersen_points_table_window_bits_18_output_limb_16_col88)
                            - (input_limb_32_col32))),
                    (((pedersen_points_table_window_bits_18_output_limb_3_col75)
                        - (input_limb_19_col19))
                        + ((pedersen_points_table_window_bits_18_output_limb_17_col89)
                            - (input_limb_33_col33))),
                    (((pedersen_points_table_window_bits_18_output_limb_4_col76)
                        - (input_limb_20_col20))
                        + ((pedersen_points_table_window_bits_18_output_limb_18_col90)
                            - (input_limb_34_col34))),
                    (((pedersen_points_table_window_bits_18_output_limb_5_col77)
                        - (input_limb_21_col21))
                        + ((pedersen_points_table_window_bits_18_output_limb_19_col91)
                            - (input_limb_35_col35))),
                    (((pedersen_points_table_window_bits_18_output_limb_6_col78)
                        - (input_limb_22_col22))
                        + ((pedersen_points_table_window_bits_18_output_limb_20_col92)
                            - (input_limb_36_col36))),
                    (((pedersen_points_table_window_bits_18_output_limb_7_col79)
                        - (input_limb_23_col23))
                        + ((pedersen_points_table_window_bits_18_output_limb_21_col93)
                            - (input_limb_37_col37))),
                    (((pedersen_points_table_window_bits_18_output_limb_8_col80)
                        - (input_limb_24_col24))
                        + ((pedersen_points_table_window_bits_18_output_limb_22_col94)
                            - (input_limb_38_col38))),
                    (((pedersen_points_table_window_bits_18_output_limb_9_col81)
                        - (input_limb_25_col25))
                        + ((pedersen_points_table_window_bits_18_output_limb_23_col95)
                            - (input_limb_39_col39))),
                    (((pedersen_points_table_window_bits_18_output_limb_10_col82)
                        - (input_limb_26_col26))
                        + ((pedersen_points_table_window_bits_18_output_limb_24_col96)
                            - (input_limb_40_col40))),
                    (((pedersen_points_table_window_bits_18_output_limb_11_col83)
                        - (input_limb_27_col27))
                        + ((pedersen_points_table_window_bits_18_output_limb_25_col97)
                            - (input_limb_41_col41))),
                    (((pedersen_points_table_window_bits_18_output_limb_12_col84)
                        - (input_limb_28_col28))
                        + ((pedersen_points_table_window_bits_18_output_limb_26_col98)
                            - (input_limb_42_col42))),
                    (((pedersen_points_table_window_bits_18_output_limb_13_col85)
                        - (input_limb_29_col29))
                        + ((pedersen_points_table_window_bits_18_output_limb_27_col99)
                            - (input_limb_43_col43))),
                ];

                // Single Karatsuba N 7.

                let z0_tmp_3cd4f_14 = [
                    ((x_sum_tmp_3cd4f_12[0]) * (y_sum_tmp_3cd4f_13[0])),
                    (((x_sum_tmp_3cd4f_12[0]) * (y_sum_tmp_3cd4f_13[1]))
                        + ((x_sum_tmp_3cd4f_12[1]) * (y_sum_tmp_3cd4f_13[0]))),
                    ((((x_sum_tmp_3cd4f_12[0]) * (y_sum_tmp_3cd4f_13[2]))
                        + ((x_sum_tmp_3cd4f_12[1]) * (y_sum_tmp_3cd4f_13[1])))
                        + ((x_sum_tmp_3cd4f_12[2]) * (y_sum_tmp_3cd4f_13[0]))),
                    (((((x_sum_tmp_3cd4f_12[0]) * (y_sum_tmp_3cd4f_13[3]))
                        + ((x_sum_tmp_3cd4f_12[1]) * (y_sum_tmp_3cd4f_13[2])))
                        + ((x_sum_tmp_3cd4f_12[2]) * (y_sum_tmp_3cd4f_13[1])))
                        + ((x_sum_tmp_3cd4f_12[3]) * (y_sum_tmp_3cd4f_13[0]))),
                    ((((((x_sum_tmp_3cd4f_12[0]) * (y_sum_tmp_3cd4f_13[4]))
                        + ((x_sum_tmp_3cd4f_12[1]) * (y_sum_tmp_3cd4f_13[3])))
                        + ((x_sum_tmp_3cd4f_12[2]) * (y_sum_tmp_3cd4f_13[2])))
                        + ((x_sum_tmp_3cd4f_12[3]) * (y_sum_tmp_3cd4f_13[1])))
                        + ((x_sum_tmp_3cd4f_12[4]) * (y_sum_tmp_3cd4f_13[0]))),
                    (((((((x_sum_tmp_3cd4f_12[0]) * (y_sum_tmp_3cd4f_13[5]))
                        + ((x_sum_tmp_3cd4f_12[1]) * (y_sum_tmp_3cd4f_13[4])))
                        + ((x_sum_tmp_3cd4f_12[2]) * (y_sum_tmp_3cd4f_13[3])))
                        + ((x_sum_tmp_3cd4f_12[3]) * (y_sum_tmp_3cd4f_13[2])))
                        + ((x_sum_tmp_3cd4f_12[4]) * (y_sum_tmp_3cd4f_13[1])))
                        + ((x_sum_tmp_3cd4f_12[5]) * (y_sum_tmp_3cd4f_13[0]))),
                    ((((((((x_sum_tmp_3cd4f_12[0]) * (y_sum_tmp_3cd4f_13[6]))
                        + ((x_sum_tmp_3cd4f_12[1]) * (y_sum_tmp_3cd4f_13[5])))
                        + ((x_sum_tmp_3cd4f_12[2]) * (y_sum_tmp_3cd4f_13[4])))
                        + ((x_sum_tmp_3cd4f_12[3]) * (y_sum_tmp_3cd4f_13[3])))
                        + ((x_sum_tmp_3cd4f_12[4]) * (y_sum_tmp_3cd4f_13[2])))
                        + ((x_sum_tmp_3cd4f_12[5]) * (y_sum_tmp_3cd4f_13[1])))
                        + ((x_sum_tmp_3cd4f_12[6]) * (y_sum_tmp_3cd4f_13[0]))),
                    (((((((x_sum_tmp_3cd4f_12[1]) * (y_sum_tmp_3cd4f_13[6]))
                        + ((x_sum_tmp_3cd4f_12[2]) * (y_sum_tmp_3cd4f_13[5])))
                        + ((x_sum_tmp_3cd4f_12[3]) * (y_sum_tmp_3cd4f_13[4])))
                        + ((x_sum_tmp_3cd4f_12[4]) * (y_sum_tmp_3cd4f_13[3])))
                        + ((x_sum_tmp_3cd4f_12[5]) * (y_sum_tmp_3cd4f_13[2])))
                        + ((x_sum_tmp_3cd4f_12[6]) * (y_sum_tmp_3cd4f_13[1]))),
                    ((((((x_sum_tmp_3cd4f_12[2]) * (y_sum_tmp_3cd4f_13[6]))
                        + ((x_sum_tmp_3cd4f_12[3]) * (y_sum_tmp_3cd4f_13[5])))
                        + ((x_sum_tmp_3cd4f_12[4]) * (y_sum_tmp_3cd4f_13[4])))
                        + ((x_sum_tmp_3cd4f_12[5]) * (y_sum_tmp_3cd4f_13[3])))
                        + ((x_sum_tmp_3cd4f_12[6]) * (y_sum_tmp_3cd4f_13[2]))),
                    (((((x_sum_tmp_3cd4f_12[3]) * (y_sum_tmp_3cd4f_13[6]))
                        + ((x_sum_tmp_3cd4f_12[4]) * (y_sum_tmp_3cd4f_13[5])))
                        + ((x_sum_tmp_3cd4f_12[5]) * (y_sum_tmp_3cd4f_13[4])))
                        + ((x_sum_tmp_3cd4f_12[6]) * (y_sum_tmp_3cd4f_13[3]))),
                    ((((x_sum_tmp_3cd4f_12[4]) * (y_sum_tmp_3cd4f_13[6]))
                        + ((x_sum_tmp_3cd4f_12[5]) * (y_sum_tmp_3cd4f_13[5])))
                        + ((x_sum_tmp_3cd4f_12[6]) * (y_sum_tmp_3cd4f_13[4]))),
                    (((x_sum_tmp_3cd4f_12[5]) * (y_sum_tmp_3cd4f_13[6]))
                        + ((x_sum_tmp_3cd4f_12[6]) * (y_sum_tmp_3cd4f_13[5]))),
                    ((x_sum_tmp_3cd4f_12[6]) * (y_sum_tmp_3cd4f_13[6])),
                ];
                let z2_tmp_3cd4f_15 = [
                    ((x_sum_tmp_3cd4f_12[7]) * (y_sum_tmp_3cd4f_13[7])),
                    (((x_sum_tmp_3cd4f_12[7]) * (y_sum_tmp_3cd4f_13[8]))
                        + ((x_sum_tmp_3cd4f_12[8]) * (y_sum_tmp_3cd4f_13[7]))),
                    ((((x_sum_tmp_3cd4f_12[7]) * (y_sum_tmp_3cd4f_13[9]))
                        + ((x_sum_tmp_3cd4f_12[8]) * (y_sum_tmp_3cd4f_13[8])))
                        + ((x_sum_tmp_3cd4f_12[9]) * (y_sum_tmp_3cd4f_13[7]))),
                    (((((x_sum_tmp_3cd4f_12[7]) * (y_sum_tmp_3cd4f_13[10]))
                        + ((x_sum_tmp_3cd4f_12[8]) * (y_sum_tmp_3cd4f_13[9])))
                        + ((x_sum_tmp_3cd4f_12[9]) * (y_sum_tmp_3cd4f_13[8])))
                        + ((x_sum_tmp_3cd4f_12[10]) * (y_sum_tmp_3cd4f_13[7]))),
                    ((((((x_sum_tmp_3cd4f_12[7]) * (y_sum_tmp_3cd4f_13[11]))
                        + ((x_sum_tmp_3cd4f_12[8]) * (y_sum_tmp_3cd4f_13[10])))
                        + ((x_sum_tmp_3cd4f_12[9]) * (y_sum_tmp_3cd4f_13[9])))
                        + ((x_sum_tmp_3cd4f_12[10]) * (y_sum_tmp_3cd4f_13[8])))
                        + ((x_sum_tmp_3cd4f_12[11]) * (y_sum_tmp_3cd4f_13[7]))),
                    (((((((x_sum_tmp_3cd4f_12[7]) * (y_sum_tmp_3cd4f_13[12]))
                        + ((x_sum_tmp_3cd4f_12[8]) * (y_sum_tmp_3cd4f_13[11])))
                        + ((x_sum_tmp_3cd4f_12[9]) * (y_sum_tmp_3cd4f_13[10])))
                        + ((x_sum_tmp_3cd4f_12[10]) * (y_sum_tmp_3cd4f_13[9])))
                        + ((x_sum_tmp_3cd4f_12[11]) * (y_sum_tmp_3cd4f_13[8])))
                        + ((x_sum_tmp_3cd4f_12[12]) * (y_sum_tmp_3cd4f_13[7]))),
                    ((((((((x_sum_tmp_3cd4f_12[7]) * (y_sum_tmp_3cd4f_13[13]))
                        + ((x_sum_tmp_3cd4f_12[8]) * (y_sum_tmp_3cd4f_13[12])))
                        + ((x_sum_tmp_3cd4f_12[9]) * (y_sum_tmp_3cd4f_13[11])))
                        + ((x_sum_tmp_3cd4f_12[10]) * (y_sum_tmp_3cd4f_13[10])))
                        + ((x_sum_tmp_3cd4f_12[11]) * (y_sum_tmp_3cd4f_13[9])))
                        + ((x_sum_tmp_3cd4f_12[12]) * (y_sum_tmp_3cd4f_13[8])))
                        + ((x_sum_tmp_3cd4f_12[13]) * (y_sum_tmp_3cd4f_13[7]))),
                    (((((((x_sum_tmp_3cd4f_12[8]) * (y_sum_tmp_3cd4f_13[13]))
                        + ((x_sum_tmp_3cd4f_12[9]) * (y_sum_tmp_3cd4f_13[12])))
                        + ((x_sum_tmp_3cd4f_12[10]) * (y_sum_tmp_3cd4f_13[11])))
                        + ((x_sum_tmp_3cd4f_12[11]) * (y_sum_tmp_3cd4f_13[10])))
                        + ((x_sum_tmp_3cd4f_12[12]) * (y_sum_tmp_3cd4f_13[9])))
                        + ((x_sum_tmp_3cd4f_12[13]) * (y_sum_tmp_3cd4f_13[8]))),
                    ((((((x_sum_tmp_3cd4f_12[9]) * (y_sum_tmp_3cd4f_13[13]))
                        + ((x_sum_tmp_3cd4f_12[10]) * (y_sum_tmp_3cd4f_13[12])))
                        + ((x_sum_tmp_3cd4f_12[11]) * (y_sum_tmp_3cd4f_13[11])))
                        + ((x_sum_tmp_3cd4f_12[12]) * (y_sum_tmp_3cd4f_13[10])))
                        + ((x_sum_tmp_3cd4f_12[13]) * (y_sum_tmp_3cd4f_13[9]))),
                    (((((x_sum_tmp_3cd4f_12[10]) * (y_sum_tmp_3cd4f_13[13]))
                        + ((x_sum_tmp_3cd4f_12[11]) * (y_sum_tmp_3cd4f_13[12])))
                        + ((x_sum_tmp_3cd4f_12[12]) * (y_sum_tmp_3cd4f_13[11])))
                        + ((x_sum_tmp_3cd4f_12[13]) * (y_sum_tmp_3cd4f_13[10]))),
                    ((((x_sum_tmp_3cd4f_12[11]) * (y_sum_tmp_3cd4f_13[13]))
                        + ((x_sum_tmp_3cd4f_12[12]) * (y_sum_tmp_3cd4f_13[12])))
                        + ((x_sum_tmp_3cd4f_12[13]) * (y_sum_tmp_3cd4f_13[11]))),
                    (((x_sum_tmp_3cd4f_12[12]) * (y_sum_tmp_3cd4f_13[13]))
                        + ((x_sum_tmp_3cd4f_12[13]) * (y_sum_tmp_3cd4f_13[12]))),
                    ((x_sum_tmp_3cd4f_12[13]) * (y_sum_tmp_3cd4f_13[13])),
                ];
                let x_sum_tmp_3cd4f_16 = [
                    ((x_sum_tmp_3cd4f_12[0]) + (x_sum_tmp_3cd4f_12[7])),
                    ((x_sum_tmp_3cd4f_12[1]) + (x_sum_tmp_3cd4f_12[8])),
                    ((x_sum_tmp_3cd4f_12[2]) + (x_sum_tmp_3cd4f_12[9])),
                    ((x_sum_tmp_3cd4f_12[3]) + (x_sum_tmp_3cd4f_12[10])),
                    ((x_sum_tmp_3cd4f_12[4]) + (x_sum_tmp_3cd4f_12[11])),
                    ((x_sum_tmp_3cd4f_12[5]) + (x_sum_tmp_3cd4f_12[12])),
                    ((x_sum_tmp_3cd4f_12[6]) + (x_sum_tmp_3cd4f_12[13])),
                ];
                let y_sum_tmp_3cd4f_17 = [
                    ((y_sum_tmp_3cd4f_13[0]) + (y_sum_tmp_3cd4f_13[7])),
                    ((y_sum_tmp_3cd4f_13[1]) + (y_sum_tmp_3cd4f_13[8])),
                    ((y_sum_tmp_3cd4f_13[2]) + (y_sum_tmp_3cd4f_13[9])),
                    ((y_sum_tmp_3cd4f_13[3]) + (y_sum_tmp_3cd4f_13[10])),
                    ((y_sum_tmp_3cd4f_13[4]) + (y_sum_tmp_3cd4f_13[11])),
                    ((y_sum_tmp_3cd4f_13[5]) + (y_sum_tmp_3cd4f_13[12])),
                    ((y_sum_tmp_3cd4f_13[6]) + (y_sum_tmp_3cd4f_13[13])),
                ];
                let single_karatsuba_n_7_output_tmp_3cd4f_18 = [
                    z0_tmp_3cd4f_14[0],
                    z0_tmp_3cd4f_14[1],
                    z0_tmp_3cd4f_14[2],
                    z0_tmp_3cd4f_14[3],
                    z0_tmp_3cd4f_14[4],
                    z0_tmp_3cd4f_14[5],
                    z0_tmp_3cd4f_14[6],
                    ((z0_tmp_3cd4f_14[7])
                        + ((((x_sum_tmp_3cd4f_16[0]) * (y_sum_tmp_3cd4f_17[0]))
                            - (z0_tmp_3cd4f_14[0]))
                            - (z2_tmp_3cd4f_15[0]))),
                    ((z0_tmp_3cd4f_14[8])
                        + (((((x_sum_tmp_3cd4f_16[0]) * (y_sum_tmp_3cd4f_17[1]))
                            + ((x_sum_tmp_3cd4f_16[1]) * (y_sum_tmp_3cd4f_17[0])))
                            - (z0_tmp_3cd4f_14[1]))
                            - (z2_tmp_3cd4f_15[1]))),
                    ((z0_tmp_3cd4f_14[9])
                        + ((((((x_sum_tmp_3cd4f_16[0]) * (y_sum_tmp_3cd4f_17[2]))
                            + ((x_sum_tmp_3cd4f_16[1]) * (y_sum_tmp_3cd4f_17[1])))
                            + ((x_sum_tmp_3cd4f_16[2]) * (y_sum_tmp_3cd4f_17[0])))
                            - (z0_tmp_3cd4f_14[2]))
                            - (z2_tmp_3cd4f_15[2]))),
                    ((z0_tmp_3cd4f_14[10])
                        + (((((((x_sum_tmp_3cd4f_16[0]) * (y_sum_tmp_3cd4f_17[3]))
                            + ((x_sum_tmp_3cd4f_16[1]) * (y_sum_tmp_3cd4f_17[2])))
                            + ((x_sum_tmp_3cd4f_16[2]) * (y_sum_tmp_3cd4f_17[1])))
                            + ((x_sum_tmp_3cd4f_16[3]) * (y_sum_tmp_3cd4f_17[0])))
                            - (z0_tmp_3cd4f_14[3]))
                            - (z2_tmp_3cd4f_15[3]))),
                    ((z0_tmp_3cd4f_14[11])
                        + ((((((((x_sum_tmp_3cd4f_16[0]) * (y_sum_tmp_3cd4f_17[4]))
                            + ((x_sum_tmp_3cd4f_16[1]) * (y_sum_tmp_3cd4f_17[3])))
                            + ((x_sum_tmp_3cd4f_16[2]) * (y_sum_tmp_3cd4f_17[2])))
                            + ((x_sum_tmp_3cd4f_16[3]) * (y_sum_tmp_3cd4f_17[1])))
                            + ((x_sum_tmp_3cd4f_16[4]) * (y_sum_tmp_3cd4f_17[0])))
                            - (z0_tmp_3cd4f_14[4]))
                            - (z2_tmp_3cd4f_15[4]))),
                    ((z0_tmp_3cd4f_14[12])
                        + (((((((((x_sum_tmp_3cd4f_16[0]) * (y_sum_tmp_3cd4f_17[5]))
                            + ((x_sum_tmp_3cd4f_16[1]) * (y_sum_tmp_3cd4f_17[4])))
                            + ((x_sum_tmp_3cd4f_16[2]) * (y_sum_tmp_3cd4f_17[3])))
                            + ((x_sum_tmp_3cd4f_16[3]) * (y_sum_tmp_3cd4f_17[2])))
                            + ((x_sum_tmp_3cd4f_16[4]) * (y_sum_tmp_3cd4f_17[1])))
                            + ((x_sum_tmp_3cd4f_16[5]) * (y_sum_tmp_3cd4f_17[0])))
                            - (z0_tmp_3cd4f_14[5]))
                            - (z2_tmp_3cd4f_15[5]))),
                    ((((((((((x_sum_tmp_3cd4f_16[0]) * (y_sum_tmp_3cd4f_17[6]))
                        + ((x_sum_tmp_3cd4f_16[1]) * (y_sum_tmp_3cd4f_17[5])))
                        + ((x_sum_tmp_3cd4f_16[2]) * (y_sum_tmp_3cd4f_17[4])))
                        + ((x_sum_tmp_3cd4f_16[3]) * (y_sum_tmp_3cd4f_17[3])))
                        + ((x_sum_tmp_3cd4f_16[4]) * (y_sum_tmp_3cd4f_17[2])))
                        + ((x_sum_tmp_3cd4f_16[5]) * (y_sum_tmp_3cd4f_17[1])))
                        + ((x_sum_tmp_3cd4f_16[6]) * (y_sum_tmp_3cd4f_17[0])))
                        - (z0_tmp_3cd4f_14[6]))
                        - (z2_tmp_3cd4f_15[6])),
                    ((z2_tmp_3cd4f_15[0])
                        + (((((((((x_sum_tmp_3cd4f_16[1]) * (y_sum_tmp_3cd4f_17[6]))
                            + ((x_sum_tmp_3cd4f_16[2]) * (y_sum_tmp_3cd4f_17[5])))
                            + ((x_sum_tmp_3cd4f_16[3]) * (y_sum_tmp_3cd4f_17[4])))
                            + ((x_sum_tmp_3cd4f_16[4]) * (y_sum_tmp_3cd4f_17[3])))
                            + ((x_sum_tmp_3cd4f_16[5]) * (y_sum_tmp_3cd4f_17[2])))
                            + ((x_sum_tmp_3cd4f_16[6]) * (y_sum_tmp_3cd4f_17[1])))
                            - (z0_tmp_3cd4f_14[7]))
                            - (z2_tmp_3cd4f_15[7]))),
                    ((z2_tmp_3cd4f_15[1])
                        + ((((((((x_sum_tmp_3cd4f_16[2]) * (y_sum_tmp_3cd4f_17[6]))
                            + ((x_sum_tmp_3cd4f_16[3]) * (y_sum_tmp_3cd4f_17[5])))
                            + ((x_sum_tmp_3cd4f_16[4]) * (y_sum_tmp_3cd4f_17[4])))
                            + ((x_sum_tmp_3cd4f_16[5]) * (y_sum_tmp_3cd4f_17[3])))
                            + ((x_sum_tmp_3cd4f_16[6]) * (y_sum_tmp_3cd4f_17[2])))
                            - (z0_tmp_3cd4f_14[8]))
                            - (z2_tmp_3cd4f_15[8]))),
                    ((z2_tmp_3cd4f_15[2])
                        + (((((((x_sum_tmp_3cd4f_16[3]) * (y_sum_tmp_3cd4f_17[6]))
                            + ((x_sum_tmp_3cd4f_16[4]) * (y_sum_tmp_3cd4f_17[5])))
                            + ((x_sum_tmp_3cd4f_16[5]) * (y_sum_tmp_3cd4f_17[4])))
                            + ((x_sum_tmp_3cd4f_16[6]) * (y_sum_tmp_3cd4f_17[3])))
                            - (z0_tmp_3cd4f_14[9]))
                            - (z2_tmp_3cd4f_15[9]))),
                    ((z2_tmp_3cd4f_15[3])
                        + ((((((x_sum_tmp_3cd4f_16[4]) * (y_sum_tmp_3cd4f_17[6]))
                            + ((x_sum_tmp_3cd4f_16[5]) * (y_sum_tmp_3cd4f_17[5])))
                            + ((x_sum_tmp_3cd4f_16[6]) * (y_sum_tmp_3cd4f_17[4])))
                            - (z0_tmp_3cd4f_14[10]))
                            - (z2_tmp_3cd4f_15[10]))),
                    ((z2_tmp_3cd4f_15[4])
                        + (((((x_sum_tmp_3cd4f_16[5]) * (y_sum_tmp_3cd4f_17[6]))
                            + ((x_sum_tmp_3cd4f_16[6]) * (y_sum_tmp_3cd4f_17[5])))
                            - (z0_tmp_3cd4f_14[11]))
                            - (z2_tmp_3cd4f_15[11]))),
                    ((z2_tmp_3cd4f_15[5])
                        + ((((x_sum_tmp_3cd4f_16[6]) * (y_sum_tmp_3cd4f_17[6]))
                            - (z0_tmp_3cd4f_14[12]))
                            - (z2_tmp_3cd4f_15[12]))),
                    z2_tmp_3cd4f_15[6],
                    z2_tmp_3cd4f_15[7],
                    z2_tmp_3cd4f_15[8],
                    z2_tmp_3cd4f_15[9],
                    z2_tmp_3cd4f_15[10],
                    z2_tmp_3cd4f_15[11],
                    z2_tmp_3cd4f_15[12],
                ];

                let double_karatsuba_1454b_output_tmp_3cd4f_19 = [
                    single_karatsuba_n_7_output_tmp_3cd4f_6[0],
                    single_karatsuba_n_7_output_tmp_3cd4f_6[1],
                    single_karatsuba_n_7_output_tmp_3cd4f_6[2],
                    single_karatsuba_n_7_output_tmp_3cd4f_6[3],
                    single_karatsuba_n_7_output_tmp_3cd4f_6[4],
                    single_karatsuba_n_7_output_tmp_3cd4f_6[5],
                    single_karatsuba_n_7_output_tmp_3cd4f_6[6],
                    single_karatsuba_n_7_output_tmp_3cd4f_6[7],
                    single_karatsuba_n_7_output_tmp_3cd4f_6[8],
                    single_karatsuba_n_7_output_tmp_3cd4f_6[9],
                    single_karatsuba_n_7_output_tmp_3cd4f_6[10],
                    single_karatsuba_n_7_output_tmp_3cd4f_6[11],
                    single_karatsuba_n_7_output_tmp_3cd4f_6[12],
                    single_karatsuba_n_7_output_tmp_3cd4f_6[13],
                    ((single_karatsuba_n_7_output_tmp_3cd4f_6[14])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[0])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[0]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[0]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_6[15])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[1])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[1]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[1]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_6[16])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[2])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[2]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[2]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_6[17])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[3])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[3]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[3]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_6[18])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[4])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[4]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[4]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_6[19])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[5])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[5]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[5]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_6[20])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[6])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[6]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[6]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_6[21])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[7])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[7]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[7]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_6[22])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[8])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[8]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[8]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_6[23])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[9])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[9]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[9]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_6[24])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[10])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[10]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[10]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_6[25])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[11])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[11]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[11]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_6[26])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[12])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[12]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[12]))),
                    (((single_karatsuba_n_7_output_tmp_3cd4f_18[13])
                        - (single_karatsuba_n_7_output_tmp_3cd4f_6[13]))
                        - (single_karatsuba_n_7_output_tmp_3cd4f_11[13])),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_11[0])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[14])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[14]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[14]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_11[1])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[15])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[15]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[15]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_11[2])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[16])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[16]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[16]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_11[3])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[17])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[17]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[17]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_11[4])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[18])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[18]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[18]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_11[5])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[19])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[19]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[19]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_11[6])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[20])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[20]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[20]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_11[7])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[21])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[21]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[21]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_11[8])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[22])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[22]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[22]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_11[9])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[23])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[23]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[23]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_11[10])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[24])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[24]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[24]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_11[11])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[25])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[25]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[25]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_11[12])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_18[26])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_6[26]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_11[26]))),
                    single_karatsuba_n_7_output_tmp_3cd4f_11[13],
                    single_karatsuba_n_7_output_tmp_3cd4f_11[14],
                    single_karatsuba_n_7_output_tmp_3cd4f_11[15],
                    single_karatsuba_n_7_output_tmp_3cd4f_11[16],
                    single_karatsuba_n_7_output_tmp_3cd4f_11[17],
                    single_karatsuba_n_7_output_tmp_3cd4f_11[18],
                    single_karatsuba_n_7_output_tmp_3cd4f_11[19],
                    single_karatsuba_n_7_output_tmp_3cd4f_11[20],
                    single_karatsuba_n_7_output_tmp_3cd4f_11[21],
                    single_karatsuba_n_7_output_tmp_3cd4f_11[22],
                    single_karatsuba_n_7_output_tmp_3cd4f_11[23],
                    single_karatsuba_n_7_output_tmp_3cd4f_11[24],
                    single_karatsuba_n_7_output_tmp_3cd4f_11[25],
                    single_karatsuba_n_7_output_tmp_3cd4f_11[26],
                ];

                let conv_tmp_3cd4f_20 = [
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[0])
                        - ((pedersen_points_table_window_bits_18_output_limb_28_col100)
                            - (input_limb_44_col44))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[1])
                        - ((pedersen_points_table_window_bits_18_output_limb_29_col101)
                            - (input_limb_45_col45))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[2])
                        - ((pedersen_points_table_window_bits_18_output_limb_30_col102)
                            - (input_limb_46_col46))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[3])
                        - ((pedersen_points_table_window_bits_18_output_limb_31_col103)
                            - (input_limb_47_col47))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[4])
                        - ((pedersen_points_table_window_bits_18_output_limb_32_col104)
                            - (input_limb_48_col48))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[5])
                        - ((pedersen_points_table_window_bits_18_output_limb_33_col105)
                            - (input_limb_49_col49))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[6])
                        - ((pedersen_points_table_window_bits_18_output_limb_34_col106)
                            - (input_limb_50_col50))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[7])
                        - ((pedersen_points_table_window_bits_18_output_limb_35_col107)
                            - (input_limb_51_col51))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[8])
                        - ((pedersen_points_table_window_bits_18_output_limb_36_col108)
                            - (input_limb_52_col52))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[9])
                        - ((pedersen_points_table_window_bits_18_output_limb_37_col109)
                            - (input_limb_53_col53))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[10])
                        - ((pedersen_points_table_window_bits_18_output_limb_38_col110)
                            - (input_limb_54_col54))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[11])
                        - ((pedersen_points_table_window_bits_18_output_limb_39_col111)
                            - (input_limb_55_col55))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[12])
                        - ((pedersen_points_table_window_bits_18_output_limb_40_col112)
                            - (input_limb_56_col56))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[13])
                        - ((pedersen_points_table_window_bits_18_output_limb_41_col113)
                            - (input_limb_57_col57))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[14])
                        - ((pedersen_points_table_window_bits_18_output_limb_42_col114)
                            - (input_limb_58_col58))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[15])
                        - ((pedersen_points_table_window_bits_18_output_limb_43_col115)
                            - (input_limb_59_col59))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[16])
                        - ((pedersen_points_table_window_bits_18_output_limb_44_col116)
                            - (input_limb_60_col60))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[17])
                        - ((pedersen_points_table_window_bits_18_output_limb_45_col117)
                            - (input_limb_61_col61))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[18])
                        - ((pedersen_points_table_window_bits_18_output_limb_46_col118)
                            - (input_limb_62_col62))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[19])
                        - ((pedersen_points_table_window_bits_18_output_limb_47_col119)
                            - (input_limb_63_col63))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[20])
                        - ((pedersen_points_table_window_bits_18_output_limb_48_col120)
                            - (input_limb_64_col64))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[21])
                        - ((pedersen_points_table_window_bits_18_output_limb_49_col121)
                            - (input_limb_65_col65))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[22])
                        - ((pedersen_points_table_window_bits_18_output_limb_50_col122)
                            - (input_limb_66_col66))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[23])
                        - ((pedersen_points_table_window_bits_18_output_limb_51_col123)
                            - (input_limb_67_col67))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[24])
                        - ((pedersen_points_table_window_bits_18_output_limb_52_col124)
                            - (input_limb_68_col68))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[25])
                        - ((pedersen_points_table_window_bits_18_output_limb_53_col125)
                            - (input_limb_69_col69))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[26])
                        - ((pedersen_points_table_window_bits_18_output_limb_54_col126)
                            - (input_limb_70_col70))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_19[27])
                        - ((pedersen_points_table_window_bits_18_output_limb_55_col127)
                            - (input_limb_71_col71))),
                    double_karatsuba_1454b_output_tmp_3cd4f_19[28],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[29],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[30],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[31],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[32],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[33],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[34],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[35],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[36],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[37],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[38],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[39],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[40],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[41],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[42],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[43],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[44],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[45],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[46],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[47],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[48],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[49],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[50],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[51],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[52],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[53],
                    double_karatsuba_1454b_output_tmp_3cd4f_19[54],
                ];
                let conv_mod_tmp_3cd4f_21 = [
                    ((((M31_32) * (conv_tmp_3cd4f_20[0])) - ((M31_4) * (conv_tmp_3cd4f_20[21])))
                        + ((M31_8) * (conv_tmp_3cd4f_20[49]))),
                    ((((conv_tmp_3cd4f_20[0]) + ((M31_32) * (conv_tmp_3cd4f_20[1])))
                        - ((M31_4) * (conv_tmp_3cd4f_20[22])))
                        + ((M31_8) * (conv_tmp_3cd4f_20[50]))),
                    ((((conv_tmp_3cd4f_20[1]) + ((M31_32) * (conv_tmp_3cd4f_20[2])))
                        - ((M31_4) * (conv_tmp_3cd4f_20[23])))
                        + ((M31_8) * (conv_tmp_3cd4f_20[51]))),
                    ((((conv_tmp_3cd4f_20[2]) + ((M31_32) * (conv_tmp_3cd4f_20[3])))
                        - ((M31_4) * (conv_tmp_3cd4f_20[24])))
                        + ((M31_8) * (conv_tmp_3cd4f_20[52]))),
                    ((((conv_tmp_3cd4f_20[3]) + ((M31_32) * (conv_tmp_3cd4f_20[4])))
                        - ((M31_4) * (conv_tmp_3cd4f_20[25])))
                        + ((M31_8) * (conv_tmp_3cd4f_20[53]))),
                    ((((conv_tmp_3cd4f_20[4]) + ((M31_32) * (conv_tmp_3cd4f_20[5])))
                        - ((M31_4) * (conv_tmp_3cd4f_20[26])))
                        + ((M31_8) * (conv_tmp_3cd4f_20[54]))),
                    (((conv_tmp_3cd4f_20[5]) + ((M31_32) * (conv_tmp_3cd4f_20[6])))
                        - ((M31_4) * (conv_tmp_3cd4f_20[27]))),
                    (((((M31_2) * (conv_tmp_3cd4f_20[0])) + (conv_tmp_3cd4f_20[6]))
                        + ((M31_32) * (conv_tmp_3cd4f_20[7])))
                        - ((M31_4) * (conv_tmp_3cd4f_20[28]))),
                    (((((M31_2) * (conv_tmp_3cd4f_20[1])) + (conv_tmp_3cd4f_20[7]))
                        + ((M31_32) * (conv_tmp_3cd4f_20[8])))
                        - ((M31_4) * (conv_tmp_3cd4f_20[29]))),
                    (((((M31_2) * (conv_tmp_3cd4f_20[2])) + (conv_tmp_3cd4f_20[8]))
                        + ((M31_32) * (conv_tmp_3cd4f_20[9])))
                        - ((M31_4) * (conv_tmp_3cd4f_20[30]))),
                    (((((M31_2) * (conv_tmp_3cd4f_20[3])) + (conv_tmp_3cd4f_20[9]))
                        + ((M31_32) * (conv_tmp_3cd4f_20[10])))
                        - ((M31_4) * (conv_tmp_3cd4f_20[31]))),
                    (((((M31_2) * (conv_tmp_3cd4f_20[4])) + (conv_tmp_3cd4f_20[10]))
                        + ((M31_32) * (conv_tmp_3cd4f_20[11])))
                        - ((M31_4) * (conv_tmp_3cd4f_20[32]))),
                    (((((M31_2) * (conv_tmp_3cd4f_20[5])) + (conv_tmp_3cd4f_20[11]))
                        + ((M31_32) * (conv_tmp_3cd4f_20[12])))
                        - ((M31_4) * (conv_tmp_3cd4f_20[33]))),
                    (((((M31_2) * (conv_tmp_3cd4f_20[6])) + (conv_tmp_3cd4f_20[12]))
                        + ((M31_32) * (conv_tmp_3cd4f_20[13])))
                        - ((M31_4) * (conv_tmp_3cd4f_20[34]))),
                    (((((M31_2) * (conv_tmp_3cd4f_20[7])) + (conv_tmp_3cd4f_20[13]))
                        + ((M31_32) * (conv_tmp_3cd4f_20[14])))
                        - ((M31_4) * (conv_tmp_3cd4f_20[35]))),
                    (((((M31_2) * (conv_tmp_3cd4f_20[8])) + (conv_tmp_3cd4f_20[14]))
                        + ((M31_32) * (conv_tmp_3cd4f_20[15])))
                        - ((M31_4) * (conv_tmp_3cd4f_20[36]))),
                    (((((M31_2) * (conv_tmp_3cd4f_20[9])) + (conv_tmp_3cd4f_20[15]))
                        + ((M31_32) * (conv_tmp_3cd4f_20[16])))
                        - ((M31_4) * (conv_tmp_3cd4f_20[37]))),
                    (((((M31_2) * (conv_tmp_3cd4f_20[10])) + (conv_tmp_3cd4f_20[16]))
                        + ((M31_32) * (conv_tmp_3cd4f_20[17])))
                        - ((M31_4) * (conv_tmp_3cd4f_20[38]))),
                    (((((M31_2) * (conv_tmp_3cd4f_20[11])) + (conv_tmp_3cd4f_20[17]))
                        + ((M31_32) * (conv_tmp_3cd4f_20[18])))
                        - ((M31_4) * (conv_tmp_3cd4f_20[39]))),
                    (((((M31_2) * (conv_tmp_3cd4f_20[12])) + (conv_tmp_3cd4f_20[18]))
                        + ((M31_32) * (conv_tmp_3cd4f_20[19])))
                        - ((M31_4) * (conv_tmp_3cd4f_20[40]))),
                    (((((M31_2) * (conv_tmp_3cd4f_20[13])) + (conv_tmp_3cd4f_20[19]))
                        + ((M31_32) * (conv_tmp_3cd4f_20[20])))
                        - ((M31_4) * (conv_tmp_3cd4f_20[41]))),
                    (((((M31_2) * (conv_tmp_3cd4f_20[14])) + (conv_tmp_3cd4f_20[20]))
                        - ((M31_4) * (conv_tmp_3cd4f_20[42])))
                        + ((M31_64) * (conv_tmp_3cd4f_20[49]))),
                    (((((M31_2) * (conv_tmp_3cd4f_20[15])) - ((M31_4) * (conv_tmp_3cd4f_20[43])))
                        + ((M31_2) * (conv_tmp_3cd4f_20[49])))
                        + ((M31_64) * (conv_tmp_3cd4f_20[50]))),
                    (((((M31_2) * (conv_tmp_3cd4f_20[16])) - ((M31_4) * (conv_tmp_3cd4f_20[44])))
                        + ((M31_2) * (conv_tmp_3cd4f_20[50])))
                        + ((M31_64) * (conv_tmp_3cd4f_20[51]))),
                    (((((M31_2) * (conv_tmp_3cd4f_20[17])) - ((M31_4) * (conv_tmp_3cd4f_20[45])))
                        + ((M31_2) * (conv_tmp_3cd4f_20[51])))
                        + ((M31_64) * (conv_tmp_3cd4f_20[52]))),
                    (((((M31_2) * (conv_tmp_3cd4f_20[18])) - ((M31_4) * (conv_tmp_3cd4f_20[46])))
                        + ((M31_2) * (conv_tmp_3cd4f_20[52])))
                        + ((M31_64) * (conv_tmp_3cd4f_20[53]))),
                    (((((M31_2) * (conv_tmp_3cd4f_20[19])) - ((M31_4) * (conv_tmp_3cd4f_20[47])))
                        + ((M31_2) * (conv_tmp_3cd4f_20[53])))
                        + ((M31_64) * (conv_tmp_3cd4f_20[54]))),
                    ((((M31_2) * (conv_tmp_3cd4f_20[20])) - ((M31_4) * (conv_tmp_3cd4f_20[48])))
                        + ((M31_2) * (conv_tmp_3cd4f_20[54]))),
                ];
                let k_mod_2_18_biased_tmp_3cd4f_22 =
                    ((((PackedUInt32::from_m31(((conv_mod_tmp_3cd4f_21[0]) + (M31_134217728))))
                        + (((PackedUInt32::from_m31(
                            ((conv_mod_tmp_3cd4f_21[1]) + (M31_134217728)),
                        )) & (UInt32_511))
                            << (UInt32_9)))
                        + (UInt32_131072))
                        & (UInt32_262143));
                let k_col156 = ((k_mod_2_18_biased_tmp_3cd4f_22.low().as_m31())
                    + (((k_mod_2_18_biased_tmp_3cd4f_22.high().as_m31()) - (M31_2)) * (M31_65536)));
                *row[156] = k_col156;
                *sub_component_inputs.range_check_20[0] = [((k_col156) + (M31_524288))];
                *lookup_data.range_check_20_0 = [((k_col156) + (M31_524288))];
                let carry_0_col157 = (((conv_mod_tmp_3cd4f_21[0]) - (k_col156)) * (M31_4194304));
                *row[157] = carry_0_col157;
                *sub_component_inputs.range_check_20_b[0] = [((carry_0_col157) + (M31_524288))];
                *lookup_data.range_check_20_b_0 = [((carry_0_col157) + (M31_524288))];
                let carry_1_col158 =
                    (((conv_mod_tmp_3cd4f_21[1]) + (carry_0_col157)) * (M31_4194304));
                *row[158] = carry_1_col158;
                *sub_component_inputs.range_check_20_c[0] = [((carry_1_col158) + (M31_524288))];
                *lookup_data.range_check_20_c_0 = [((carry_1_col158) + (M31_524288))];
                let carry_2_col159 =
                    (((conv_mod_tmp_3cd4f_21[2]) + (carry_1_col158)) * (M31_4194304));
                *row[159] = carry_2_col159;
                *sub_component_inputs.range_check_20_d[0] = [((carry_2_col159) + (M31_524288))];
                *lookup_data.range_check_20_d_0 = [((carry_2_col159) + (M31_524288))];
                let carry_3_col160 =
                    (((conv_mod_tmp_3cd4f_21[3]) + (carry_2_col159)) * (M31_4194304));
                *row[160] = carry_3_col160;
                *sub_component_inputs.range_check_20_e[0] = [((carry_3_col160) + (M31_524288))];
                *lookup_data.range_check_20_e_0 = [((carry_3_col160) + (M31_524288))];
                let carry_4_col161 =
                    (((conv_mod_tmp_3cd4f_21[4]) + (carry_3_col160)) * (M31_4194304));
                *row[161] = carry_4_col161;
                *sub_component_inputs.range_check_20_f[0] = [((carry_4_col161) + (M31_524288))];
                *lookup_data.range_check_20_f_0 = [((carry_4_col161) + (M31_524288))];
                let carry_5_col162 =
                    (((conv_mod_tmp_3cd4f_21[5]) + (carry_4_col161)) * (M31_4194304));
                *row[162] = carry_5_col162;
                *sub_component_inputs.range_check_20_g[0] = [((carry_5_col162) + (M31_524288))];
                *lookup_data.range_check_20_g_0 = [((carry_5_col162) + (M31_524288))];
                let carry_6_col163 =
                    (((conv_mod_tmp_3cd4f_21[6]) + (carry_5_col162)) * (M31_4194304));
                *row[163] = carry_6_col163;
                *sub_component_inputs.range_check_20_h[0] = [((carry_6_col163) + (M31_524288))];
                *lookup_data.range_check_20_h_0 = [((carry_6_col163) + (M31_524288))];
                let carry_7_col164 =
                    (((conv_mod_tmp_3cd4f_21[7]) + (carry_6_col163)) * (M31_4194304));
                *row[164] = carry_7_col164;
                *sub_component_inputs.range_check_20[1] = [((carry_7_col164) + (M31_524288))];
                *lookup_data.range_check_20_1 = [((carry_7_col164) + (M31_524288))];
                let carry_8_col165 =
                    (((conv_mod_tmp_3cd4f_21[8]) + (carry_7_col164)) * (M31_4194304));
                *row[165] = carry_8_col165;
                *sub_component_inputs.range_check_20_b[1] = [((carry_8_col165) + (M31_524288))];
                *lookup_data.range_check_20_b_1 = [((carry_8_col165) + (M31_524288))];
                let carry_9_col166 =
                    (((conv_mod_tmp_3cd4f_21[9]) + (carry_8_col165)) * (M31_4194304));
                *row[166] = carry_9_col166;
                *sub_component_inputs.range_check_20_c[1] = [((carry_9_col166) + (M31_524288))];
                *lookup_data.range_check_20_c_1 = [((carry_9_col166) + (M31_524288))];
                let carry_10_col167 =
                    (((conv_mod_tmp_3cd4f_21[10]) + (carry_9_col166)) * (M31_4194304));
                *row[167] = carry_10_col167;
                *sub_component_inputs.range_check_20_d[1] = [((carry_10_col167) + (M31_524288))];
                *lookup_data.range_check_20_d_1 = [((carry_10_col167) + (M31_524288))];
                let carry_11_col168 =
                    (((conv_mod_tmp_3cd4f_21[11]) + (carry_10_col167)) * (M31_4194304));
                *row[168] = carry_11_col168;
                *sub_component_inputs.range_check_20_e[1] = [((carry_11_col168) + (M31_524288))];
                *lookup_data.range_check_20_e_1 = [((carry_11_col168) + (M31_524288))];
                let carry_12_col169 =
                    (((conv_mod_tmp_3cd4f_21[12]) + (carry_11_col168)) * (M31_4194304));
                *row[169] = carry_12_col169;
                *sub_component_inputs.range_check_20_f[1] = [((carry_12_col169) + (M31_524288))];
                *lookup_data.range_check_20_f_1 = [((carry_12_col169) + (M31_524288))];
                let carry_13_col170 =
                    (((conv_mod_tmp_3cd4f_21[13]) + (carry_12_col169)) * (M31_4194304));
                *row[170] = carry_13_col170;
                *sub_component_inputs.range_check_20_g[1] = [((carry_13_col170) + (M31_524288))];
                *lookup_data.range_check_20_g_1 = [((carry_13_col170) + (M31_524288))];
                let carry_14_col171 =
                    (((conv_mod_tmp_3cd4f_21[14]) + (carry_13_col170)) * (M31_4194304));
                *row[171] = carry_14_col171;
                *sub_component_inputs.range_check_20_h[1] = [((carry_14_col171) + (M31_524288))];
                *lookup_data.range_check_20_h_1 = [((carry_14_col171) + (M31_524288))];
                let carry_15_col172 =
                    (((conv_mod_tmp_3cd4f_21[15]) + (carry_14_col171)) * (M31_4194304));
                *row[172] = carry_15_col172;
                *sub_component_inputs.range_check_20[2] = [((carry_15_col172) + (M31_524288))];
                *lookup_data.range_check_20_2 = [((carry_15_col172) + (M31_524288))];
                let carry_16_col173 =
                    (((conv_mod_tmp_3cd4f_21[16]) + (carry_15_col172)) * (M31_4194304));
                *row[173] = carry_16_col173;
                *sub_component_inputs.range_check_20_b[2] = [((carry_16_col173) + (M31_524288))];
                *lookup_data.range_check_20_b_2 = [((carry_16_col173) + (M31_524288))];
                let carry_17_col174 =
                    (((conv_mod_tmp_3cd4f_21[17]) + (carry_16_col173)) * (M31_4194304));
                *row[174] = carry_17_col174;
                *sub_component_inputs.range_check_20_c[2] = [((carry_17_col174) + (M31_524288))];
                *lookup_data.range_check_20_c_2 = [((carry_17_col174) + (M31_524288))];
                let carry_18_col175 =
                    (((conv_mod_tmp_3cd4f_21[18]) + (carry_17_col174)) * (M31_4194304));
                *row[175] = carry_18_col175;
                *sub_component_inputs.range_check_20_d[2] = [((carry_18_col175) + (M31_524288))];
                *lookup_data.range_check_20_d_2 = [((carry_18_col175) + (M31_524288))];
                let carry_19_col176 =
                    (((conv_mod_tmp_3cd4f_21[19]) + (carry_18_col175)) * (M31_4194304));
                *row[176] = carry_19_col176;
                *sub_component_inputs.range_check_20_e[2] = [((carry_19_col176) + (M31_524288))];
                *lookup_data.range_check_20_e_2 = [((carry_19_col176) + (M31_524288))];
                let carry_20_col177 =
                    (((conv_mod_tmp_3cd4f_21[20]) + (carry_19_col176)) * (M31_4194304));
                *row[177] = carry_20_col177;
                *sub_component_inputs.range_check_20_f[2] = [((carry_20_col177) + (M31_524288))];
                *lookup_data.range_check_20_f_2 = [((carry_20_col177) + (M31_524288))];
                let carry_21_col178 = ((((conv_mod_tmp_3cd4f_21[21]) - ((M31_136) * (k_col156)))
                    + (carry_20_col177))
                    * (M31_4194304));
                *row[178] = carry_21_col178;
                *sub_component_inputs.range_check_20_g[2] = [((carry_21_col178) + (M31_524288))];
                *lookup_data.range_check_20_g_2 = [((carry_21_col178) + (M31_524288))];
                let carry_22_col179 =
                    (((conv_mod_tmp_3cd4f_21[22]) + (carry_21_col178)) * (M31_4194304));
                *row[179] = carry_22_col179;
                *sub_component_inputs.range_check_20_h[2] = [((carry_22_col179) + (M31_524288))];
                *lookup_data.range_check_20_h_2 = [((carry_22_col179) + (M31_524288))];
                let carry_23_col180 =
                    (((conv_mod_tmp_3cd4f_21[23]) + (carry_22_col179)) * (M31_4194304));
                *row[180] = carry_23_col180;
                *sub_component_inputs.range_check_20[3] = [((carry_23_col180) + (M31_524288))];
                *lookup_data.range_check_20_3 = [((carry_23_col180) + (M31_524288))];
                let carry_24_col181 =
                    (((conv_mod_tmp_3cd4f_21[24]) + (carry_23_col180)) * (M31_4194304));
                *row[181] = carry_24_col181;
                *sub_component_inputs.range_check_20_b[3] = [((carry_24_col181) + (M31_524288))];
                *lookup_data.range_check_20_b_3 = [((carry_24_col181) + (M31_524288))];
                let carry_25_col182 =
                    (((conv_mod_tmp_3cd4f_21[25]) + (carry_24_col181)) * (M31_4194304));
                *row[182] = carry_25_col182;
                *sub_component_inputs.range_check_20_c[3] = [((carry_25_col182) + (M31_524288))];
                *lookup_data.range_check_20_c_3 = [((carry_25_col182) + (M31_524288))];
                let carry_26_col183 =
                    (((conv_mod_tmp_3cd4f_21[26]) + (carry_25_col182)) * (M31_4194304));
                *row[183] = carry_26_col183;
                *sub_component_inputs.range_check_20_d[3] = [((carry_26_col183) + (M31_524288))];
                *lookup_data.range_check_20_d_3 = [((carry_26_col183) + (M31_524288))];

                let result_x_tmp_3cd4f_23 = ((((slope_tmp_3cd4f_1) * (slope_tmp_3cd4f_1))
                    - (partial_ec_mul_window_bits_18_input.2 .1[0]))
                    - (pedersen_points_table_window_bits_18_output_tmp_3cd4f_0[0]));
                let result_x_limb_0_col184 = result_x_tmp_3cd4f_23.get_m31(0);
                *row[184] = result_x_limb_0_col184;
                let result_x_limb_1_col185 = result_x_tmp_3cd4f_23.get_m31(1);
                *row[185] = result_x_limb_1_col185;
                let result_x_limb_2_col186 = result_x_tmp_3cd4f_23.get_m31(2);
                *row[186] = result_x_limb_2_col186;
                let result_x_limb_3_col187 = result_x_tmp_3cd4f_23.get_m31(3);
                *row[187] = result_x_limb_3_col187;
                let result_x_limb_4_col188 = result_x_tmp_3cd4f_23.get_m31(4);
                *row[188] = result_x_limb_4_col188;
                let result_x_limb_5_col189 = result_x_tmp_3cd4f_23.get_m31(5);
                *row[189] = result_x_limb_5_col189;
                let result_x_limb_6_col190 = result_x_tmp_3cd4f_23.get_m31(6);
                *row[190] = result_x_limb_6_col190;
                let result_x_limb_7_col191 = result_x_tmp_3cd4f_23.get_m31(7);
                *row[191] = result_x_limb_7_col191;
                let result_x_limb_8_col192 = result_x_tmp_3cd4f_23.get_m31(8);
                *row[192] = result_x_limb_8_col192;
                let result_x_limb_9_col193 = result_x_tmp_3cd4f_23.get_m31(9);
                *row[193] = result_x_limb_9_col193;
                let result_x_limb_10_col194 = result_x_tmp_3cd4f_23.get_m31(10);
                *row[194] = result_x_limb_10_col194;
                let result_x_limb_11_col195 = result_x_tmp_3cd4f_23.get_m31(11);
                *row[195] = result_x_limb_11_col195;
                let result_x_limb_12_col196 = result_x_tmp_3cd4f_23.get_m31(12);
                *row[196] = result_x_limb_12_col196;
                let result_x_limb_13_col197 = result_x_tmp_3cd4f_23.get_m31(13);
                *row[197] = result_x_limb_13_col197;
                let result_x_limb_14_col198 = result_x_tmp_3cd4f_23.get_m31(14);
                *row[198] = result_x_limb_14_col198;
                let result_x_limb_15_col199 = result_x_tmp_3cd4f_23.get_m31(15);
                *row[199] = result_x_limb_15_col199;
                let result_x_limb_16_col200 = result_x_tmp_3cd4f_23.get_m31(16);
                *row[200] = result_x_limb_16_col200;
                let result_x_limb_17_col201 = result_x_tmp_3cd4f_23.get_m31(17);
                *row[201] = result_x_limb_17_col201;
                let result_x_limb_18_col202 = result_x_tmp_3cd4f_23.get_m31(18);
                *row[202] = result_x_limb_18_col202;
                let result_x_limb_19_col203 = result_x_tmp_3cd4f_23.get_m31(19);
                *row[203] = result_x_limb_19_col203;
                let result_x_limb_20_col204 = result_x_tmp_3cd4f_23.get_m31(20);
                *row[204] = result_x_limb_20_col204;
                let result_x_limb_21_col205 = result_x_tmp_3cd4f_23.get_m31(21);
                *row[205] = result_x_limb_21_col205;
                let result_x_limb_22_col206 = result_x_tmp_3cd4f_23.get_m31(22);
                *row[206] = result_x_limb_22_col206;
                let result_x_limb_23_col207 = result_x_tmp_3cd4f_23.get_m31(23);
                *row[207] = result_x_limb_23_col207;
                let result_x_limb_24_col208 = result_x_tmp_3cd4f_23.get_m31(24);
                *row[208] = result_x_limb_24_col208;
                let result_x_limb_25_col209 = result_x_tmp_3cd4f_23.get_m31(25);
                *row[209] = result_x_limb_25_col209;
                let result_x_limb_26_col210 = result_x_tmp_3cd4f_23.get_m31(26);
                *row[210] = result_x_limb_26_col210;
                let result_x_limb_27_col211 = result_x_tmp_3cd4f_23.get_m31(27);
                *row[211] = result_x_limb_27_col211;

                // Range Check Mem Value N 28.

                *sub_component_inputs.range_check_9_9[2] =
                    [result_x_limb_0_col184, result_x_limb_1_col185];
                *lookup_data.range_check_9_9_2 = [result_x_limb_0_col184, result_x_limb_1_col185];
                *sub_component_inputs.range_check_9_9_b[2] =
                    [result_x_limb_2_col186, result_x_limb_3_col187];
                *lookup_data.range_check_9_9_b_2 = [result_x_limb_2_col186, result_x_limb_3_col187];
                *sub_component_inputs.range_check_9_9_c[2] =
                    [result_x_limb_4_col188, result_x_limb_5_col189];
                *lookup_data.range_check_9_9_c_2 = [result_x_limb_4_col188, result_x_limb_5_col189];
                *sub_component_inputs.range_check_9_9_d[2] =
                    [result_x_limb_6_col190, result_x_limb_7_col191];
                *lookup_data.range_check_9_9_d_2 = [result_x_limb_6_col190, result_x_limb_7_col191];
                *sub_component_inputs.range_check_9_9_e[2] =
                    [result_x_limb_8_col192, result_x_limb_9_col193];
                *lookup_data.range_check_9_9_e_2 = [result_x_limb_8_col192, result_x_limb_9_col193];
                *sub_component_inputs.range_check_9_9_f[2] =
                    [result_x_limb_10_col194, result_x_limb_11_col195];
                *lookup_data.range_check_9_9_f_2 =
                    [result_x_limb_10_col194, result_x_limb_11_col195];
                *sub_component_inputs.range_check_9_9_g[1] =
                    [result_x_limb_12_col196, result_x_limb_13_col197];
                *lookup_data.range_check_9_9_g_1 =
                    [result_x_limb_12_col196, result_x_limb_13_col197];
                *sub_component_inputs.range_check_9_9_h[1] =
                    [result_x_limb_14_col198, result_x_limb_15_col199];
                *lookup_data.range_check_9_9_h_1 =
                    [result_x_limb_14_col198, result_x_limb_15_col199];
                *sub_component_inputs.range_check_9_9[3] =
                    [result_x_limb_16_col200, result_x_limb_17_col201];
                *lookup_data.range_check_9_9_3 = [result_x_limb_16_col200, result_x_limb_17_col201];
                *sub_component_inputs.range_check_9_9_b[3] =
                    [result_x_limb_18_col202, result_x_limb_19_col203];
                *lookup_data.range_check_9_9_b_3 =
                    [result_x_limb_18_col202, result_x_limb_19_col203];
                *sub_component_inputs.range_check_9_9_c[3] =
                    [result_x_limb_20_col204, result_x_limb_21_col205];
                *lookup_data.range_check_9_9_c_3 =
                    [result_x_limb_20_col204, result_x_limb_21_col205];
                *sub_component_inputs.range_check_9_9_d[3] =
                    [result_x_limb_22_col206, result_x_limb_23_col207];
                *lookup_data.range_check_9_9_d_3 =
                    [result_x_limb_22_col206, result_x_limb_23_col207];
                *sub_component_inputs.range_check_9_9_e[3] =
                    [result_x_limb_24_col208, result_x_limb_25_col209];
                *lookup_data.range_check_9_9_e_3 =
                    [result_x_limb_24_col208, result_x_limb_25_col209];
                *sub_component_inputs.range_check_9_9_f[3] =
                    [result_x_limb_26_col210, result_x_limb_27_col211];
                *lookup_data.range_check_9_9_f_3 =
                    [result_x_limb_26_col210, result_x_limb_27_col211];

                // Verify Mul 252.

                // Double Karatsuba 1454 B.

                // Single Karatsuba N 7.

                let z0_tmp_3cd4f_24 = [
                    ((slope_limb_0_col128) * (slope_limb_0_col128)),
                    (((slope_limb_0_col128) * (slope_limb_1_col129))
                        + ((slope_limb_1_col129) * (slope_limb_0_col128))),
                    ((((slope_limb_0_col128) * (slope_limb_2_col130))
                        + ((slope_limb_1_col129) * (slope_limb_1_col129)))
                        + ((slope_limb_2_col130) * (slope_limb_0_col128))),
                    (((((slope_limb_0_col128) * (slope_limb_3_col131))
                        + ((slope_limb_1_col129) * (slope_limb_2_col130)))
                        + ((slope_limb_2_col130) * (slope_limb_1_col129)))
                        + ((slope_limb_3_col131) * (slope_limb_0_col128))),
                    ((((((slope_limb_0_col128) * (slope_limb_4_col132))
                        + ((slope_limb_1_col129) * (slope_limb_3_col131)))
                        + ((slope_limb_2_col130) * (slope_limb_2_col130)))
                        + ((slope_limb_3_col131) * (slope_limb_1_col129)))
                        + ((slope_limb_4_col132) * (slope_limb_0_col128))),
                    (((((((slope_limb_0_col128) * (slope_limb_5_col133))
                        + ((slope_limb_1_col129) * (slope_limb_4_col132)))
                        + ((slope_limb_2_col130) * (slope_limb_3_col131)))
                        + ((slope_limb_3_col131) * (slope_limb_2_col130)))
                        + ((slope_limb_4_col132) * (slope_limb_1_col129)))
                        + ((slope_limb_5_col133) * (slope_limb_0_col128))),
                    ((((((((slope_limb_0_col128) * (slope_limb_6_col134))
                        + ((slope_limb_1_col129) * (slope_limb_5_col133)))
                        + ((slope_limb_2_col130) * (slope_limb_4_col132)))
                        + ((slope_limb_3_col131) * (slope_limb_3_col131)))
                        + ((slope_limb_4_col132) * (slope_limb_2_col130)))
                        + ((slope_limb_5_col133) * (slope_limb_1_col129)))
                        + ((slope_limb_6_col134) * (slope_limb_0_col128))),
                    (((((((slope_limb_1_col129) * (slope_limb_6_col134))
                        + ((slope_limb_2_col130) * (slope_limb_5_col133)))
                        + ((slope_limb_3_col131) * (slope_limb_4_col132)))
                        + ((slope_limb_4_col132) * (slope_limb_3_col131)))
                        + ((slope_limb_5_col133) * (slope_limb_2_col130)))
                        + ((slope_limb_6_col134) * (slope_limb_1_col129))),
                    ((((((slope_limb_2_col130) * (slope_limb_6_col134))
                        + ((slope_limb_3_col131) * (slope_limb_5_col133)))
                        + ((slope_limb_4_col132) * (slope_limb_4_col132)))
                        + ((slope_limb_5_col133) * (slope_limb_3_col131)))
                        + ((slope_limb_6_col134) * (slope_limb_2_col130))),
                    (((((slope_limb_3_col131) * (slope_limb_6_col134))
                        + ((slope_limb_4_col132) * (slope_limb_5_col133)))
                        + ((slope_limb_5_col133) * (slope_limb_4_col132)))
                        + ((slope_limb_6_col134) * (slope_limb_3_col131))),
                    ((((slope_limb_4_col132) * (slope_limb_6_col134))
                        + ((slope_limb_5_col133) * (slope_limb_5_col133)))
                        + ((slope_limb_6_col134) * (slope_limb_4_col132))),
                    (((slope_limb_5_col133) * (slope_limb_6_col134))
                        + ((slope_limb_6_col134) * (slope_limb_5_col133))),
                    ((slope_limb_6_col134) * (slope_limb_6_col134)),
                ];
                let z2_tmp_3cd4f_25 = [
                    ((slope_limb_7_col135) * (slope_limb_7_col135)),
                    (((slope_limb_7_col135) * (slope_limb_8_col136))
                        + ((slope_limb_8_col136) * (slope_limb_7_col135))),
                    ((((slope_limb_7_col135) * (slope_limb_9_col137))
                        + ((slope_limb_8_col136) * (slope_limb_8_col136)))
                        + ((slope_limb_9_col137) * (slope_limb_7_col135))),
                    (((((slope_limb_7_col135) * (slope_limb_10_col138))
                        + ((slope_limb_8_col136) * (slope_limb_9_col137)))
                        + ((slope_limb_9_col137) * (slope_limb_8_col136)))
                        + ((slope_limb_10_col138) * (slope_limb_7_col135))),
                    ((((((slope_limb_7_col135) * (slope_limb_11_col139))
                        + ((slope_limb_8_col136) * (slope_limb_10_col138)))
                        + ((slope_limb_9_col137) * (slope_limb_9_col137)))
                        + ((slope_limb_10_col138) * (slope_limb_8_col136)))
                        + ((slope_limb_11_col139) * (slope_limb_7_col135))),
                    (((((((slope_limb_7_col135) * (slope_limb_12_col140))
                        + ((slope_limb_8_col136) * (slope_limb_11_col139)))
                        + ((slope_limb_9_col137) * (slope_limb_10_col138)))
                        + ((slope_limb_10_col138) * (slope_limb_9_col137)))
                        + ((slope_limb_11_col139) * (slope_limb_8_col136)))
                        + ((slope_limb_12_col140) * (slope_limb_7_col135))),
                    ((((((((slope_limb_7_col135) * (slope_limb_13_col141))
                        + ((slope_limb_8_col136) * (slope_limb_12_col140)))
                        + ((slope_limb_9_col137) * (slope_limb_11_col139)))
                        + ((slope_limb_10_col138) * (slope_limb_10_col138)))
                        + ((slope_limb_11_col139) * (slope_limb_9_col137)))
                        + ((slope_limb_12_col140) * (slope_limb_8_col136)))
                        + ((slope_limb_13_col141) * (slope_limb_7_col135))),
                    (((((((slope_limb_8_col136) * (slope_limb_13_col141))
                        + ((slope_limb_9_col137) * (slope_limb_12_col140)))
                        + ((slope_limb_10_col138) * (slope_limb_11_col139)))
                        + ((slope_limb_11_col139) * (slope_limb_10_col138)))
                        + ((slope_limb_12_col140) * (slope_limb_9_col137)))
                        + ((slope_limb_13_col141) * (slope_limb_8_col136))),
                    ((((((slope_limb_9_col137) * (slope_limb_13_col141))
                        + ((slope_limb_10_col138) * (slope_limb_12_col140)))
                        + ((slope_limb_11_col139) * (slope_limb_11_col139)))
                        + ((slope_limb_12_col140) * (slope_limb_10_col138)))
                        + ((slope_limb_13_col141) * (slope_limb_9_col137))),
                    (((((slope_limb_10_col138) * (slope_limb_13_col141))
                        + ((slope_limb_11_col139) * (slope_limb_12_col140)))
                        + ((slope_limb_12_col140) * (slope_limb_11_col139)))
                        + ((slope_limb_13_col141) * (slope_limb_10_col138))),
                    ((((slope_limb_11_col139) * (slope_limb_13_col141))
                        + ((slope_limb_12_col140) * (slope_limb_12_col140)))
                        + ((slope_limb_13_col141) * (slope_limb_11_col139))),
                    (((slope_limb_12_col140) * (slope_limb_13_col141))
                        + ((slope_limb_13_col141) * (slope_limb_12_col140))),
                    ((slope_limb_13_col141) * (slope_limb_13_col141)),
                ];
                let x_sum_tmp_3cd4f_26 = [
                    ((slope_limb_0_col128) + (slope_limb_7_col135)),
                    ((slope_limb_1_col129) + (slope_limb_8_col136)),
                    ((slope_limb_2_col130) + (slope_limb_9_col137)),
                    ((slope_limb_3_col131) + (slope_limb_10_col138)),
                    ((slope_limb_4_col132) + (slope_limb_11_col139)),
                    ((slope_limb_5_col133) + (slope_limb_12_col140)),
                    ((slope_limb_6_col134) + (slope_limb_13_col141)),
                ];
                let y_sum_tmp_3cd4f_27 = [
                    ((slope_limb_0_col128) + (slope_limb_7_col135)),
                    ((slope_limb_1_col129) + (slope_limb_8_col136)),
                    ((slope_limb_2_col130) + (slope_limb_9_col137)),
                    ((slope_limb_3_col131) + (slope_limb_10_col138)),
                    ((slope_limb_4_col132) + (slope_limb_11_col139)),
                    ((slope_limb_5_col133) + (slope_limb_12_col140)),
                    ((slope_limb_6_col134) + (slope_limb_13_col141)),
                ];
                let single_karatsuba_n_7_output_tmp_3cd4f_28 = [
                    z0_tmp_3cd4f_24[0],
                    z0_tmp_3cd4f_24[1],
                    z0_tmp_3cd4f_24[2],
                    z0_tmp_3cd4f_24[3],
                    z0_tmp_3cd4f_24[4],
                    z0_tmp_3cd4f_24[5],
                    z0_tmp_3cd4f_24[6],
                    ((z0_tmp_3cd4f_24[7])
                        + ((((x_sum_tmp_3cd4f_26[0]) * (y_sum_tmp_3cd4f_27[0]))
                            - (z0_tmp_3cd4f_24[0]))
                            - (z2_tmp_3cd4f_25[0]))),
                    ((z0_tmp_3cd4f_24[8])
                        + (((((x_sum_tmp_3cd4f_26[0]) * (y_sum_tmp_3cd4f_27[1]))
                            + ((x_sum_tmp_3cd4f_26[1]) * (y_sum_tmp_3cd4f_27[0])))
                            - (z0_tmp_3cd4f_24[1]))
                            - (z2_tmp_3cd4f_25[1]))),
                    ((z0_tmp_3cd4f_24[9])
                        + ((((((x_sum_tmp_3cd4f_26[0]) * (y_sum_tmp_3cd4f_27[2]))
                            + ((x_sum_tmp_3cd4f_26[1]) * (y_sum_tmp_3cd4f_27[1])))
                            + ((x_sum_tmp_3cd4f_26[2]) * (y_sum_tmp_3cd4f_27[0])))
                            - (z0_tmp_3cd4f_24[2]))
                            - (z2_tmp_3cd4f_25[2]))),
                    ((z0_tmp_3cd4f_24[10])
                        + (((((((x_sum_tmp_3cd4f_26[0]) * (y_sum_tmp_3cd4f_27[3]))
                            + ((x_sum_tmp_3cd4f_26[1]) * (y_sum_tmp_3cd4f_27[2])))
                            + ((x_sum_tmp_3cd4f_26[2]) * (y_sum_tmp_3cd4f_27[1])))
                            + ((x_sum_tmp_3cd4f_26[3]) * (y_sum_tmp_3cd4f_27[0])))
                            - (z0_tmp_3cd4f_24[3]))
                            - (z2_tmp_3cd4f_25[3]))),
                    ((z0_tmp_3cd4f_24[11])
                        + ((((((((x_sum_tmp_3cd4f_26[0]) * (y_sum_tmp_3cd4f_27[4]))
                            + ((x_sum_tmp_3cd4f_26[1]) * (y_sum_tmp_3cd4f_27[3])))
                            + ((x_sum_tmp_3cd4f_26[2]) * (y_sum_tmp_3cd4f_27[2])))
                            + ((x_sum_tmp_3cd4f_26[3]) * (y_sum_tmp_3cd4f_27[1])))
                            + ((x_sum_tmp_3cd4f_26[4]) * (y_sum_tmp_3cd4f_27[0])))
                            - (z0_tmp_3cd4f_24[4]))
                            - (z2_tmp_3cd4f_25[4]))),
                    ((z0_tmp_3cd4f_24[12])
                        + (((((((((x_sum_tmp_3cd4f_26[0]) * (y_sum_tmp_3cd4f_27[5]))
                            + ((x_sum_tmp_3cd4f_26[1]) * (y_sum_tmp_3cd4f_27[4])))
                            + ((x_sum_tmp_3cd4f_26[2]) * (y_sum_tmp_3cd4f_27[3])))
                            + ((x_sum_tmp_3cd4f_26[3]) * (y_sum_tmp_3cd4f_27[2])))
                            + ((x_sum_tmp_3cd4f_26[4]) * (y_sum_tmp_3cd4f_27[1])))
                            + ((x_sum_tmp_3cd4f_26[5]) * (y_sum_tmp_3cd4f_27[0])))
                            - (z0_tmp_3cd4f_24[5]))
                            - (z2_tmp_3cd4f_25[5]))),
                    ((((((((((x_sum_tmp_3cd4f_26[0]) * (y_sum_tmp_3cd4f_27[6]))
                        + ((x_sum_tmp_3cd4f_26[1]) * (y_sum_tmp_3cd4f_27[5])))
                        + ((x_sum_tmp_3cd4f_26[2]) * (y_sum_tmp_3cd4f_27[4])))
                        + ((x_sum_tmp_3cd4f_26[3]) * (y_sum_tmp_3cd4f_27[3])))
                        + ((x_sum_tmp_3cd4f_26[4]) * (y_sum_tmp_3cd4f_27[2])))
                        + ((x_sum_tmp_3cd4f_26[5]) * (y_sum_tmp_3cd4f_27[1])))
                        + ((x_sum_tmp_3cd4f_26[6]) * (y_sum_tmp_3cd4f_27[0])))
                        - (z0_tmp_3cd4f_24[6]))
                        - (z2_tmp_3cd4f_25[6])),
                    ((z2_tmp_3cd4f_25[0])
                        + (((((((((x_sum_tmp_3cd4f_26[1]) * (y_sum_tmp_3cd4f_27[6]))
                            + ((x_sum_tmp_3cd4f_26[2]) * (y_sum_tmp_3cd4f_27[5])))
                            + ((x_sum_tmp_3cd4f_26[3]) * (y_sum_tmp_3cd4f_27[4])))
                            + ((x_sum_tmp_3cd4f_26[4]) * (y_sum_tmp_3cd4f_27[3])))
                            + ((x_sum_tmp_3cd4f_26[5]) * (y_sum_tmp_3cd4f_27[2])))
                            + ((x_sum_tmp_3cd4f_26[6]) * (y_sum_tmp_3cd4f_27[1])))
                            - (z0_tmp_3cd4f_24[7]))
                            - (z2_tmp_3cd4f_25[7]))),
                    ((z2_tmp_3cd4f_25[1])
                        + ((((((((x_sum_tmp_3cd4f_26[2]) * (y_sum_tmp_3cd4f_27[6]))
                            + ((x_sum_tmp_3cd4f_26[3]) * (y_sum_tmp_3cd4f_27[5])))
                            + ((x_sum_tmp_3cd4f_26[4]) * (y_sum_tmp_3cd4f_27[4])))
                            + ((x_sum_tmp_3cd4f_26[5]) * (y_sum_tmp_3cd4f_27[3])))
                            + ((x_sum_tmp_3cd4f_26[6]) * (y_sum_tmp_3cd4f_27[2])))
                            - (z0_tmp_3cd4f_24[8]))
                            - (z2_tmp_3cd4f_25[8]))),
                    ((z2_tmp_3cd4f_25[2])
                        + (((((((x_sum_tmp_3cd4f_26[3]) * (y_sum_tmp_3cd4f_27[6]))
                            + ((x_sum_tmp_3cd4f_26[4]) * (y_sum_tmp_3cd4f_27[5])))
                            + ((x_sum_tmp_3cd4f_26[5]) * (y_sum_tmp_3cd4f_27[4])))
                            + ((x_sum_tmp_3cd4f_26[6]) * (y_sum_tmp_3cd4f_27[3])))
                            - (z0_tmp_3cd4f_24[9]))
                            - (z2_tmp_3cd4f_25[9]))),
                    ((z2_tmp_3cd4f_25[3])
                        + ((((((x_sum_tmp_3cd4f_26[4]) * (y_sum_tmp_3cd4f_27[6]))
                            + ((x_sum_tmp_3cd4f_26[5]) * (y_sum_tmp_3cd4f_27[5])))
                            + ((x_sum_tmp_3cd4f_26[6]) * (y_sum_tmp_3cd4f_27[4])))
                            - (z0_tmp_3cd4f_24[10]))
                            - (z2_tmp_3cd4f_25[10]))),
                    ((z2_tmp_3cd4f_25[4])
                        + (((((x_sum_tmp_3cd4f_26[5]) * (y_sum_tmp_3cd4f_27[6]))
                            + ((x_sum_tmp_3cd4f_26[6]) * (y_sum_tmp_3cd4f_27[5])))
                            - (z0_tmp_3cd4f_24[11]))
                            - (z2_tmp_3cd4f_25[11]))),
                    ((z2_tmp_3cd4f_25[5])
                        + ((((x_sum_tmp_3cd4f_26[6]) * (y_sum_tmp_3cd4f_27[6]))
                            - (z0_tmp_3cd4f_24[12]))
                            - (z2_tmp_3cd4f_25[12]))),
                    z2_tmp_3cd4f_25[6],
                    z2_tmp_3cd4f_25[7],
                    z2_tmp_3cd4f_25[8],
                    z2_tmp_3cd4f_25[9],
                    z2_tmp_3cd4f_25[10],
                    z2_tmp_3cd4f_25[11],
                    z2_tmp_3cd4f_25[12],
                ];

                // Single Karatsuba N 7.

                let z0_tmp_3cd4f_29 = [
                    ((slope_limb_14_col142) * (slope_limb_14_col142)),
                    (((slope_limb_14_col142) * (slope_limb_15_col143))
                        + ((slope_limb_15_col143) * (slope_limb_14_col142))),
                    ((((slope_limb_14_col142) * (slope_limb_16_col144))
                        + ((slope_limb_15_col143) * (slope_limb_15_col143)))
                        + ((slope_limb_16_col144) * (slope_limb_14_col142))),
                    (((((slope_limb_14_col142) * (slope_limb_17_col145))
                        + ((slope_limb_15_col143) * (slope_limb_16_col144)))
                        + ((slope_limb_16_col144) * (slope_limb_15_col143)))
                        + ((slope_limb_17_col145) * (slope_limb_14_col142))),
                    ((((((slope_limb_14_col142) * (slope_limb_18_col146))
                        + ((slope_limb_15_col143) * (slope_limb_17_col145)))
                        + ((slope_limb_16_col144) * (slope_limb_16_col144)))
                        + ((slope_limb_17_col145) * (slope_limb_15_col143)))
                        + ((slope_limb_18_col146) * (slope_limb_14_col142))),
                    (((((((slope_limb_14_col142) * (slope_limb_19_col147))
                        + ((slope_limb_15_col143) * (slope_limb_18_col146)))
                        + ((slope_limb_16_col144) * (slope_limb_17_col145)))
                        + ((slope_limb_17_col145) * (slope_limb_16_col144)))
                        + ((slope_limb_18_col146) * (slope_limb_15_col143)))
                        + ((slope_limb_19_col147) * (slope_limb_14_col142))),
                    ((((((((slope_limb_14_col142) * (slope_limb_20_col148))
                        + ((slope_limb_15_col143) * (slope_limb_19_col147)))
                        + ((slope_limb_16_col144) * (slope_limb_18_col146)))
                        + ((slope_limb_17_col145) * (slope_limb_17_col145)))
                        + ((slope_limb_18_col146) * (slope_limb_16_col144)))
                        + ((slope_limb_19_col147) * (slope_limb_15_col143)))
                        + ((slope_limb_20_col148) * (slope_limb_14_col142))),
                    (((((((slope_limb_15_col143) * (slope_limb_20_col148))
                        + ((slope_limb_16_col144) * (slope_limb_19_col147)))
                        + ((slope_limb_17_col145) * (slope_limb_18_col146)))
                        + ((slope_limb_18_col146) * (slope_limb_17_col145)))
                        + ((slope_limb_19_col147) * (slope_limb_16_col144)))
                        + ((slope_limb_20_col148) * (slope_limb_15_col143))),
                    ((((((slope_limb_16_col144) * (slope_limb_20_col148))
                        + ((slope_limb_17_col145) * (slope_limb_19_col147)))
                        + ((slope_limb_18_col146) * (slope_limb_18_col146)))
                        + ((slope_limb_19_col147) * (slope_limb_17_col145)))
                        + ((slope_limb_20_col148) * (slope_limb_16_col144))),
                    (((((slope_limb_17_col145) * (slope_limb_20_col148))
                        + ((slope_limb_18_col146) * (slope_limb_19_col147)))
                        + ((slope_limb_19_col147) * (slope_limb_18_col146)))
                        + ((slope_limb_20_col148) * (slope_limb_17_col145))),
                    ((((slope_limb_18_col146) * (slope_limb_20_col148))
                        + ((slope_limb_19_col147) * (slope_limb_19_col147)))
                        + ((slope_limb_20_col148) * (slope_limb_18_col146))),
                    (((slope_limb_19_col147) * (slope_limb_20_col148))
                        + ((slope_limb_20_col148) * (slope_limb_19_col147))),
                    ((slope_limb_20_col148) * (slope_limb_20_col148)),
                ];
                let z2_tmp_3cd4f_30 = [
                    ((slope_limb_21_col149) * (slope_limb_21_col149)),
                    (((slope_limb_21_col149) * (slope_limb_22_col150))
                        + ((slope_limb_22_col150) * (slope_limb_21_col149))),
                    ((((slope_limb_21_col149) * (slope_limb_23_col151))
                        + ((slope_limb_22_col150) * (slope_limb_22_col150)))
                        + ((slope_limb_23_col151) * (slope_limb_21_col149))),
                    (((((slope_limb_21_col149) * (slope_limb_24_col152))
                        + ((slope_limb_22_col150) * (slope_limb_23_col151)))
                        + ((slope_limb_23_col151) * (slope_limb_22_col150)))
                        + ((slope_limb_24_col152) * (slope_limb_21_col149))),
                    ((((((slope_limb_21_col149) * (slope_limb_25_col153))
                        + ((slope_limb_22_col150) * (slope_limb_24_col152)))
                        + ((slope_limb_23_col151) * (slope_limb_23_col151)))
                        + ((slope_limb_24_col152) * (slope_limb_22_col150)))
                        + ((slope_limb_25_col153) * (slope_limb_21_col149))),
                    (((((((slope_limb_21_col149) * (slope_limb_26_col154))
                        + ((slope_limb_22_col150) * (slope_limb_25_col153)))
                        + ((slope_limb_23_col151) * (slope_limb_24_col152)))
                        + ((slope_limb_24_col152) * (slope_limb_23_col151)))
                        + ((slope_limb_25_col153) * (slope_limb_22_col150)))
                        + ((slope_limb_26_col154) * (slope_limb_21_col149))),
                    ((((((((slope_limb_21_col149) * (slope_limb_27_col155))
                        + ((slope_limb_22_col150) * (slope_limb_26_col154)))
                        + ((slope_limb_23_col151) * (slope_limb_25_col153)))
                        + ((slope_limb_24_col152) * (slope_limb_24_col152)))
                        + ((slope_limb_25_col153) * (slope_limb_23_col151)))
                        + ((slope_limb_26_col154) * (slope_limb_22_col150)))
                        + ((slope_limb_27_col155) * (slope_limb_21_col149))),
                    (((((((slope_limb_22_col150) * (slope_limb_27_col155))
                        + ((slope_limb_23_col151) * (slope_limb_26_col154)))
                        + ((slope_limb_24_col152) * (slope_limb_25_col153)))
                        + ((slope_limb_25_col153) * (slope_limb_24_col152)))
                        + ((slope_limb_26_col154) * (slope_limb_23_col151)))
                        + ((slope_limb_27_col155) * (slope_limb_22_col150))),
                    ((((((slope_limb_23_col151) * (slope_limb_27_col155))
                        + ((slope_limb_24_col152) * (slope_limb_26_col154)))
                        + ((slope_limb_25_col153) * (slope_limb_25_col153)))
                        + ((slope_limb_26_col154) * (slope_limb_24_col152)))
                        + ((slope_limb_27_col155) * (slope_limb_23_col151))),
                    (((((slope_limb_24_col152) * (slope_limb_27_col155))
                        + ((slope_limb_25_col153) * (slope_limb_26_col154)))
                        + ((slope_limb_26_col154) * (slope_limb_25_col153)))
                        + ((slope_limb_27_col155) * (slope_limb_24_col152))),
                    ((((slope_limb_25_col153) * (slope_limb_27_col155))
                        + ((slope_limb_26_col154) * (slope_limb_26_col154)))
                        + ((slope_limb_27_col155) * (slope_limb_25_col153))),
                    (((slope_limb_26_col154) * (slope_limb_27_col155))
                        + ((slope_limb_27_col155) * (slope_limb_26_col154))),
                    ((slope_limb_27_col155) * (slope_limb_27_col155)),
                ];
                let x_sum_tmp_3cd4f_31 = [
                    ((slope_limb_14_col142) + (slope_limb_21_col149)),
                    ((slope_limb_15_col143) + (slope_limb_22_col150)),
                    ((slope_limb_16_col144) + (slope_limb_23_col151)),
                    ((slope_limb_17_col145) + (slope_limb_24_col152)),
                    ((slope_limb_18_col146) + (slope_limb_25_col153)),
                    ((slope_limb_19_col147) + (slope_limb_26_col154)),
                    ((slope_limb_20_col148) + (slope_limb_27_col155)),
                ];
                let y_sum_tmp_3cd4f_32 = [
                    ((slope_limb_14_col142) + (slope_limb_21_col149)),
                    ((slope_limb_15_col143) + (slope_limb_22_col150)),
                    ((slope_limb_16_col144) + (slope_limb_23_col151)),
                    ((slope_limb_17_col145) + (slope_limb_24_col152)),
                    ((slope_limb_18_col146) + (slope_limb_25_col153)),
                    ((slope_limb_19_col147) + (slope_limb_26_col154)),
                    ((slope_limb_20_col148) + (slope_limb_27_col155)),
                ];
                let single_karatsuba_n_7_output_tmp_3cd4f_33 = [
                    z0_tmp_3cd4f_29[0],
                    z0_tmp_3cd4f_29[1],
                    z0_tmp_3cd4f_29[2],
                    z0_tmp_3cd4f_29[3],
                    z0_tmp_3cd4f_29[4],
                    z0_tmp_3cd4f_29[5],
                    z0_tmp_3cd4f_29[6],
                    ((z0_tmp_3cd4f_29[7])
                        + ((((x_sum_tmp_3cd4f_31[0]) * (y_sum_tmp_3cd4f_32[0]))
                            - (z0_tmp_3cd4f_29[0]))
                            - (z2_tmp_3cd4f_30[0]))),
                    ((z0_tmp_3cd4f_29[8])
                        + (((((x_sum_tmp_3cd4f_31[0]) * (y_sum_tmp_3cd4f_32[1]))
                            + ((x_sum_tmp_3cd4f_31[1]) * (y_sum_tmp_3cd4f_32[0])))
                            - (z0_tmp_3cd4f_29[1]))
                            - (z2_tmp_3cd4f_30[1]))),
                    ((z0_tmp_3cd4f_29[9])
                        + ((((((x_sum_tmp_3cd4f_31[0]) * (y_sum_tmp_3cd4f_32[2]))
                            + ((x_sum_tmp_3cd4f_31[1]) * (y_sum_tmp_3cd4f_32[1])))
                            + ((x_sum_tmp_3cd4f_31[2]) * (y_sum_tmp_3cd4f_32[0])))
                            - (z0_tmp_3cd4f_29[2]))
                            - (z2_tmp_3cd4f_30[2]))),
                    ((z0_tmp_3cd4f_29[10])
                        + (((((((x_sum_tmp_3cd4f_31[0]) * (y_sum_tmp_3cd4f_32[3]))
                            + ((x_sum_tmp_3cd4f_31[1]) * (y_sum_tmp_3cd4f_32[2])))
                            + ((x_sum_tmp_3cd4f_31[2]) * (y_sum_tmp_3cd4f_32[1])))
                            + ((x_sum_tmp_3cd4f_31[3]) * (y_sum_tmp_3cd4f_32[0])))
                            - (z0_tmp_3cd4f_29[3]))
                            - (z2_tmp_3cd4f_30[3]))),
                    ((z0_tmp_3cd4f_29[11])
                        + ((((((((x_sum_tmp_3cd4f_31[0]) * (y_sum_tmp_3cd4f_32[4]))
                            + ((x_sum_tmp_3cd4f_31[1]) * (y_sum_tmp_3cd4f_32[3])))
                            + ((x_sum_tmp_3cd4f_31[2]) * (y_sum_tmp_3cd4f_32[2])))
                            + ((x_sum_tmp_3cd4f_31[3]) * (y_sum_tmp_3cd4f_32[1])))
                            + ((x_sum_tmp_3cd4f_31[4]) * (y_sum_tmp_3cd4f_32[0])))
                            - (z0_tmp_3cd4f_29[4]))
                            - (z2_tmp_3cd4f_30[4]))),
                    ((z0_tmp_3cd4f_29[12])
                        + (((((((((x_sum_tmp_3cd4f_31[0]) * (y_sum_tmp_3cd4f_32[5]))
                            + ((x_sum_tmp_3cd4f_31[1]) * (y_sum_tmp_3cd4f_32[4])))
                            + ((x_sum_tmp_3cd4f_31[2]) * (y_sum_tmp_3cd4f_32[3])))
                            + ((x_sum_tmp_3cd4f_31[3]) * (y_sum_tmp_3cd4f_32[2])))
                            + ((x_sum_tmp_3cd4f_31[4]) * (y_sum_tmp_3cd4f_32[1])))
                            + ((x_sum_tmp_3cd4f_31[5]) * (y_sum_tmp_3cd4f_32[0])))
                            - (z0_tmp_3cd4f_29[5]))
                            - (z2_tmp_3cd4f_30[5]))),
                    ((((((((((x_sum_tmp_3cd4f_31[0]) * (y_sum_tmp_3cd4f_32[6]))
                        + ((x_sum_tmp_3cd4f_31[1]) * (y_sum_tmp_3cd4f_32[5])))
                        + ((x_sum_tmp_3cd4f_31[2]) * (y_sum_tmp_3cd4f_32[4])))
                        + ((x_sum_tmp_3cd4f_31[3]) * (y_sum_tmp_3cd4f_32[3])))
                        + ((x_sum_tmp_3cd4f_31[4]) * (y_sum_tmp_3cd4f_32[2])))
                        + ((x_sum_tmp_3cd4f_31[5]) * (y_sum_tmp_3cd4f_32[1])))
                        + ((x_sum_tmp_3cd4f_31[6]) * (y_sum_tmp_3cd4f_32[0])))
                        - (z0_tmp_3cd4f_29[6]))
                        - (z2_tmp_3cd4f_30[6])),
                    ((z2_tmp_3cd4f_30[0])
                        + (((((((((x_sum_tmp_3cd4f_31[1]) * (y_sum_tmp_3cd4f_32[6]))
                            + ((x_sum_tmp_3cd4f_31[2]) * (y_sum_tmp_3cd4f_32[5])))
                            + ((x_sum_tmp_3cd4f_31[3]) * (y_sum_tmp_3cd4f_32[4])))
                            + ((x_sum_tmp_3cd4f_31[4]) * (y_sum_tmp_3cd4f_32[3])))
                            + ((x_sum_tmp_3cd4f_31[5]) * (y_sum_tmp_3cd4f_32[2])))
                            + ((x_sum_tmp_3cd4f_31[6]) * (y_sum_tmp_3cd4f_32[1])))
                            - (z0_tmp_3cd4f_29[7]))
                            - (z2_tmp_3cd4f_30[7]))),
                    ((z2_tmp_3cd4f_30[1])
                        + ((((((((x_sum_tmp_3cd4f_31[2]) * (y_sum_tmp_3cd4f_32[6]))
                            + ((x_sum_tmp_3cd4f_31[3]) * (y_sum_tmp_3cd4f_32[5])))
                            + ((x_sum_tmp_3cd4f_31[4]) * (y_sum_tmp_3cd4f_32[4])))
                            + ((x_sum_tmp_3cd4f_31[5]) * (y_sum_tmp_3cd4f_32[3])))
                            + ((x_sum_tmp_3cd4f_31[6]) * (y_sum_tmp_3cd4f_32[2])))
                            - (z0_tmp_3cd4f_29[8]))
                            - (z2_tmp_3cd4f_30[8]))),
                    ((z2_tmp_3cd4f_30[2])
                        + (((((((x_sum_tmp_3cd4f_31[3]) * (y_sum_tmp_3cd4f_32[6]))
                            + ((x_sum_tmp_3cd4f_31[4]) * (y_sum_tmp_3cd4f_32[5])))
                            + ((x_sum_tmp_3cd4f_31[5]) * (y_sum_tmp_3cd4f_32[4])))
                            + ((x_sum_tmp_3cd4f_31[6]) * (y_sum_tmp_3cd4f_32[3])))
                            - (z0_tmp_3cd4f_29[9]))
                            - (z2_tmp_3cd4f_30[9]))),
                    ((z2_tmp_3cd4f_30[3])
                        + ((((((x_sum_tmp_3cd4f_31[4]) * (y_sum_tmp_3cd4f_32[6]))
                            + ((x_sum_tmp_3cd4f_31[5]) * (y_sum_tmp_3cd4f_32[5])))
                            + ((x_sum_tmp_3cd4f_31[6]) * (y_sum_tmp_3cd4f_32[4])))
                            - (z0_tmp_3cd4f_29[10]))
                            - (z2_tmp_3cd4f_30[10]))),
                    ((z2_tmp_3cd4f_30[4])
                        + (((((x_sum_tmp_3cd4f_31[5]) * (y_sum_tmp_3cd4f_32[6]))
                            + ((x_sum_tmp_3cd4f_31[6]) * (y_sum_tmp_3cd4f_32[5])))
                            - (z0_tmp_3cd4f_29[11]))
                            - (z2_tmp_3cd4f_30[11]))),
                    ((z2_tmp_3cd4f_30[5])
                        + ((((x_sum_tmp_3cd4f_31[6]) * (y_sum_tmp_3cd4f_32[6]))
                            - (z0_tmp_3cd4f_29[12]))
                            - (z2_tmp_3cd4f_30[12]))),
                    z2_tmp_3cd4f_30[6],
                    z2_tmp_3cd4f_30[7],
                    z2_tmp_3cd4f_30[8],
                    z2_tmp_3cd4f_30[9],
                    z2_tmp_3cd4f_30[10],
                    z2_tmp_3cd4f_30[11],
                    z2_tmp_3cd4f_30[12],
                ];

                let x_sum_tmp_3cd4f_34 = [
                    ((slope_limb_0_col128) + (slope_limb_14_col142)),
                    ((slope_limb_1_col129) + (slope_limb_15_col143)),
                    ((slope_limb_2_col130) + (slope_limb_16_col144)),
                    ((slope_limb_3_col131) + (slope_limb_17_col145)),
                    ((slope_limb_4_col132) + (slope_limb_18_col146)),
                    ((slope_limb_5_col133) + (slope_limb_19_col147)),
                    ((slope_limb_6_col134) + (slope_limb_20_col148)),
                    ((slope_limb_7_col135) + (slope_limb_21_col149)),
                    ((slope_limb_8_col136) + (slope_limb_22_col150)),
                    ((slope_limb_9_col137) + (slope_limb_23_col151)),
                    ((slope_limb_10_col138) + (slope_limb_24_col152)),
                    ((slope_limb_11_col139) + (slope_limb_25_col153)),
                    ((slope_limb_12_col140) + (slope_limb_26_col154)),
                    ((slope_limb_13_col141) + (slope_limb_27_col155)),
                ];
                let y_sum_tmp_3cd4f_35 = [
                    ((slope_limb_0_col128) + (slope_limb_14_col142)),
                    ((slope_limb_1_col129) + (slope_limb_15_col143)),
                    ((slope_limb_2_col130) + (slope_limb_16_col144)),
                    ((slope_limb_3_col131) + (slope_limb_17_col145)),
                    ((slope_limb_4_col132) + (slope_limb_18_col146)),
                    ((slope_limb_5_col133) + (slope_limb_19_col147)),
                    ((slope_limb_6_col134) + (slope_limb_20_col148)),
                    ((slope_limb_7_col135) + (slope_limb_21_col149)),
                    ((slope_limb_8_col136) + (slope_limb_22_col150)),
                    ((slope_limb_9_col137) + (slope_limb_23_col151)),
                    ((slope_limb_10_col138) + (slope_limb_24_col152)),
                    ((slope_limb_11_col139) + (slope_limb_25_col153)),
                    ((slope_limb_12_col140) + (slope_limb_26_col154)),
                    ((slope_limb_13_col141) + (slope_limb_27_col155)),
                ];

                // Single Karatsuba N 7.

                let z0_tmp_3cd4f_36 = [
                    ((x_sum_tmp_3cd4f_34[0]) * (y_sum_tmp_3cd4f_35[0])),
                    (((x_sum_tmp_3cd4f_34[0]) * (y_sum_tmp_3cd4f_35[1]))
                        + ((x_sum_tmp_3cd4f_34[1]) * (y_sum_tmp_3cd4f_35[0]))),
                    ((((x_sum_tmp_3cd4f_34[0]) * (y_sum_tmp_3cd4f_35[2]))
                        + ((x_sum_tmp_3cd4f_34[1]) * (y_sum_tmp_3cd4f_35[1])))
                        + ((x_sum_tmp_3cd4f_34[2]) * (y_sum_tmp_3cd4f_35[0]))),
                    (((((x_sum_tmp_3cd4f_34[0]) * (y_sum_tmp_3cd4f_35[3]))
                        + ((x_sum_tmp_3cd4f_34[1]) * (y_sum_tmp_3cd4f_35[2])))
                        + ((x_sum_tmp_3cd4f_34[2]) * (y_sum_tmp_3cd4f_35[1])))
                        + ((x_sum_tmp_3cd4f_34[3]) * (y_sum_tmp_3cd4f_35[0]))),
                    ((((((x_sum_tmp_3cd4f_34[0]) * (y_sum_tmp_3cd4f_35[4]))
                        + ((x_sum_tmp_3cd4f_34[1]) * (y_sum_tmp_3cd4f_35[3])))
                        + ((x_sum_tmp_3cd4f_34[2]) * (y_sum_tmp_3cd4f_35[2])))
                        + ((x_sum_tmp_3cd4f_34[3]) * (y_sum_tmp_3cd4f_35[1])))
                        + ((x_sum_tmp_3cd4f_34[4]) * (y_sum_tmp_3cd4f_35[0]))),
                    (((((((x_sum_tmp_3cd4f_34[0]) * (y_sum_tmp_3cd4f_35[5]))
                        + ((x_sum_tmp_3cd4f_34[1]) * (y_sum_tmp_3cd4f_35[4])))
                        + ((x_sum_tmp_3cd4f_34[2]) * (y_sum_tmp_3cd4f_35[3])))
                        + ((x_sum_tmp_3cd4f_34[3]) * (y_sum_tmp_3cd4f_35[2])))
                        + ((x_sum_tmp_3cd4f_34[4]) * (y_sum_tmp_3cd4f_35[1])))
                        + ((x_sum_tmp_3cd4f_34[5]) * (y_sum_tmp_3cd4f_35[0]))),
                    ((((((((x_sum_tmp_3cd4f_34[0]) * (y_sum_tmp_3cd4f_35[6]))
                        + ((x_sum_tmp_3cd4f_34[1]) * (y_sum_tmp_3cd4f_35[5])))
                        + ((x_sum_tmp_3cd4f_34[2]) * (y_sum_tmp_3cd4f_35[4])))
                        + ((x_sum_tmp_3cd4f_34[3]) * (y_sum_tmp_3cd4f_35[3])))
                        + ((x_sum_tmp_3cd4f_34[4]) * (y_sum_tmp_3cd4f_35[2])))
                        + ((x_sum_tmp_3cd4f_34[5]) * (y_sum_tmp_3cd4f_35[1])))
                        + ((x_sum_tmp_3cd4f_34[6]) * (y_sum_tmp_3cd4f_35[0]))),
                    (((((((x_sum_tmp_3cd4f_34[1]) * (y_sum_tmp_3cd4f_35[6]))
                        + ((x_sum_tmp_3cd4f_34[2]) * (y_sum_tmp_3cd4f_35[5])))
                        + ((x_sum_tmp_3cd4f_34[3]) * (y_sum_tmp_3cd4f_35[4])))
                        + ((x_sum_tmp_3cd4f_34[4]) * (y_sum_tmp_3cd4f_35[3])))
                        + ((x_sum_tmp_3cd4f_34[5]) * (y_sum_tmp_3cd4f_35[2])))
                        + ((x_sum_tmp_3cd4f_34[6]) * (y_sum_tmp_3cd4f_35[1]))),
                    ((((((x_sum_tmp_3cd4f_34[2]) * (y_sum_tmp_3cd4f_35[6]))
                        + ((x_sum_tmp_3cd4f_34[3]) * (y_sum_tmp_3cd4f_35[5])))
                        + ((x_sum_tmp_3cd4f_34[4]) * (y_sum_tmp_3cd4f_35[4])))
                        + ((x_sum_tmp_3cd4f_34[5]) * (y_sum_tmp_3cd4f_35[3])))
                        + ((x_sum_tmp_3cd4f_34[6]) * (y_sum_tmp_3cd4f_35[2]))),
                    (((((x_sum_tmp_3cd4f_34[3]) * (y_sum_tmp_3cd4f_35[6]))
                        + ((x_sum_tmp_3cd4f_34[4]) * (y_sum_tmp_3cd4f_35[5])))
                        + ((x_sum_tmp_3cd4f_34[5]) * (y_sum_tmp_3cd4f_35[4])))
                        + ((x_sum_tmp_3cd4f_34[6]) * (y_sum_tmp_3cd4f_35[3]))),
                    ((((x_sum_tmp_3cd4f_34[4]) * (y_sum_tmp_3cd4f_35[6]))
                        + ((x_sum_tmp_3cd4f_34[5]) * (y_sum_tmp_3cd4f_35[5])))
                        + ((x_sum_tmp_3cd4f_34[6]) * (y_sum_tmp_3cd4f_35[4]))),
                    (((x_sum_tmp_3cd4f_34[5]) * (y_sum_tmp_3cd4f_35[6]))
                        + ((x_sum_tmp_3cd4f_34[6]) * (y_sum_tmp_3cd4f_35[5]))),
                    ((x_sum_tmp_3cd4f_34[6]) * (y_sum_tmp_3cd4f_35[6])),
                ];
                let z2_tmp_3cd4f_37 = [
                    ((x_sum_tmp_3cd4f_34[7]) * (y_sum_tmp_3cd4f_35[7])),
                    (((x_sum_tmp_3cd4f_34[7]) * (y_sum_tmp_3cd4f_35[8]))
                        + ((x_sum_tmp_3cd4f_34[8]) * (y_sum_tmp_3cd4f_35[7]))),
                    ((((x_sum_tmp_3cd4f_34[7]) * (y_sum_tmp_3cd4f_35[9]))
                        + ((x_sum_tmp_3cd4f_34[8]) * (y_sum_tmp_3cd4f_35[8])))
                        + ((x_sum_tmp_3cd4f_34[9]) * (y_sum_tmp_3cd4f_35[7]))),
                    (((((x_sum_tmp_3cd4f_34[7]) * (y_sum_tmp_3cd4f_35[10]))
                        + ((x_sum_tmp_3cd4f_34[8]) * (y_sum_tmp_3cd4f_35[9])))
                        + ((x_sum_tmp_3cd4f_34[9]) * (y_sum_tmp_3cd4f_35[8])))
                        + ((x_sum_tmp_3cd4f_34[10]) * (y_sum_tmp_3cd4f_35[7]))),
                    ((((((x_sum_tmp_3cd4f_34[7]) * (y_sum_tmp_3cd4f_35[11]))
                        + ((x_sum_tmp_3cd4f_34[8]) * (y_sum_tmp_3cd4f_35[10])))
                        + ((x_sum_tmp_3cd4f_34[9]) * (y_sum_tmp_3cd4f_35[9])))
                        + ((x_sum_tmp_3cd4f_34[10]) * (y_sum_tmp_3cd4f_35[8])))
                        + ((x_sum_tmp_3cd4f_34[11]) * (y_sum_tmp_3cd4f_35[7]))),
                    (((((((x_sum_tmp_3cd4f_34[7]) * (y_sum_tmp_3cd4f_35[12]))
                        + ((x_sum_tmp_3cd4f_34[8]) * (y_sum_tmp_3cd4f_35[11])))
                        + ((x_sum_tmp_3cd4f_34[9]) * (y_sum_tmp_3cd4f_35[10])))
                        + ((x_sum_tmp_3cd4f_34[10]) * (y_sum_tmp_3cd4f_35[9])))
                        + ((x_sum_tmp_3cd4f_34[11]) * (y_sum_tmp_3cd4f_35[8])))
                        + ((x_sum_tmp_3cd4f_34[12]) * (y_sum_tmp_3cd4f_35[7]))),
                    ((((((((x_sum_tmp_3cd4f_34[7]) * (y_sum_tmp_3cd4f_35[13]))
                        + ((x_sum_tmp_3cd4f_34[8]) * (y_sum_tmp_3cd4f_35[12])))
                        + ((x_sum_tmp_3cd4f_34[9]) * (y_sum_tmp_3cd4f_35[11])))
                        + ((x_sum_tmp_3cd4f_34[10]) * (y_sum_tmp_3cd4f_35[10])))
                        + ((x_sum_tmp_3cd4f_34[11]) * (y_sum_tmp_3cd4f_35[9])))
                        + ((x_sum_tmp_3cd4f_34[12]) * (y_sum_tmp_3cd4f_35[8])))
                        + ((x_sum_tmp_3cd4f_34[13]) * (y_sum_tmp_3cd4f_35[7]))),
                    (((((((x_sum_tmp_3cd4f_34[8]) * (y_sum_tmp_3cd4f_35[13]))
                        + ((x_sum_tmp_3cd4f_34[9]) * (y_sum_tmp_3cd4f_35[12])))
                        + ((x_sum_tmp_3cd4f_34[10]) * (y_sum_tmp_3cd4f_35[11])))
                        + ((x_sum_tmp_3cd4f_34[11]) * (y_sum_tmp_3cd4f_35[10])))
                        + ((x_sum_tmp_3cd4f_34[12]) * (y_sum_tmp_3cd4f_35[9])))
                        + ((x_sum_tmp_3cd4f_34[13]) * (y_sum_tmp_3cd4f_35[8]))),
                    ((((((x_sum_tmp_3cd4f_34[9]) * (y_sum_tmp_3cd4f_35[13]))
                        + ((x_sum_tmp_3cd4f_34[10]) * (y_sum_tmp_3cd4f_35[12])))
                        + ((x_sum_tmp_3cd4f_34[11]) * (y_sum_tmp_3cd4f_35[11])))
                        + ((x_sum_tmp_3cd4f_34[12]) * (y_sum_tmp_3cd4f_35[10])))
                        + ((x_sum_tmp_3cd4f_34[13]) * (y_sum_tmp_3cd4f_35[9]))),
                    (((((x_sum_tmp_3cd4f_34[10]) * (y_sum_tmp_3cd4f_35[13]))
                        + ((x_sum_tmp_3cd4f_34[11]) * (y_sum_tmp_3cd4f_35[12])))
                        + ((x_sum_tmp_3cd4f_34[12]) * (y_sum_tmp_3cd4f_35[11])))
                        + ((x_sum_tmp_3cd4f_34[13]) * (y_sum_tmp_3cd4f_35[10]))),
                    ((((x_sum_tmp_3cd4f_34[11]) * (y_sum_tmp_3cd4f_35[13]))
                        + ((x_sum_tmp_3cd4f_34[12]) * (y_sum_tmp_3cd4f_35[12])))
                        + ((x_sum_tmp_3cd4f_34[13]) * (y_sum_tmp_3cd4f_35[11]))),
                    (((x_sum_tmp_3cd4f_34[12]) * (y_sum_tmp_3cd4f_35[13]))
                        + ((x_sum_tmp_3cd4f_34[13]) * (y_sum_tmp_3cd4f_35[12]))),
                    ((x_sum_tmp_3cd4f_34[13]) * (y_sum_tmp_3cd4f_35[13])),
                ];
                let x_sum_tmp_3cd4f_38 = [
                    ((x_sum_tmp_3cd4f_34[0]) + (x_sum_tmp_3cd4f_34[7])),
                    ((x_sum_tmp_3cd4f_34[1]) + (x_sum_tmp_3cd4f_34[8])),
                    ((x_sum_tmp_3cd4f_34[2]) + (x_sum_tmp_3cd4f_34[9])),
                    ((x_sum_tmp_3cd4f_34[3]) + (x_sum_tmp_3cd4f_34[10])),
                    ((x_sum_tmp_3cd4f_34[4]) + (x_sum_tmp_3cd4f_34[11])),
                    ((x_sum_tmp_3cd4f_34[5]) + (x_sum_tmp_3cd4f_34[12])),
                    ((x_sum_tmp_3cd4f_34[6]) + (x_sum_tmp_3cd4f_34[13])),
                ];
                let y_sum_tmp_3cd4f_39 = [
                    ((y_sum_tmp_3cd4f_35[0]) + (y_sum_tmp_3cd4f_35[7])),
                    ((y_sum_tmp_3cd4f_35[1]) + (y_sum_tmp_3cd4f_35[8])),
                    ((y_sum_tmp_3cd4f_35[2]) + (y_sum_tmp_3cd4f_35[9])),
                    ((y_sum_tmp_3cd4f_35[3]) + (y_sum_tmp_3cd4f_35[10])),
                    ((y_sum_tmp_3cd4f_35[4]) + (y_sum_tmp_3cd4f_35[11])),
                    ((y_sum_tmp_3cd4f_35[5]) + (y_sum_tmp_3cd4f_35[12])),
                    ((y_sum_tmp_3cd4f_35[6]) + (y_sum_tmp_3cd4f_35[13])),
                ];
                let single_karatsuba_n_7_output_tmp_3cd4f_40 = [
                    z0_tmp_3cd4f_36[0],
                    z0_tmp_3cd4f_36[1],
                    z0_tmp_3cd4f_36[2],
                    z0_tmp_3cd4f_36[3],
                    z0_tmp_3cd4f_36[4],
                    z0_tmp_3cd4f_36[5],
                    z0_tmp_3cd4f_36[6],
                    ((z0_tmp_3cd4f_36[7])
                        + ((((x_sum_tmp_3cd4f_38[0]) * (y_sum_tmp_3cd4f_39[0]))
                            - (z0_tmp_3cd4f_36[0]))
                            - (z2_tmp_3cd4f_37[0]))),
                    ((z0_tmp_3cd4f_36[8])
                        + (((((x_sum_tmp_3cd4f_38[0]) * (y_sum_tmp_3cd4f_39[1]))
                            + ((x_sum_tmp_3cd4f_38[1]) * (y_sum_tmp_3cd4f_39[0])))
                            - (z0_tmp_3cd4f_36[1]))
                            - (z2_tmp_3cd4f_37[1]))),
                    ((z0_tmp_3cd4f_36[9])
                        + ((((((x_sum_tmp_3cd4f_38[0]) * (y_sum_tmp_3cd4f_39[2]))
                            + ((x_sum_tmp_3cd4f_38[1]) * (y_sum_tmp_3cd4f_39[1])))
                            + ((x_sum_tmp_3cd4f_38[2]) * (y_sum_tmp_3cd4f_39[0])))
                            - (z0_tmp_3cd4f_36[2]))
                            - (z2_tmp_3cd4f_37[2]))),
                    ((z0_tmp_3cd4f_36[10])
                        + (((((((x_sum_tmp_3cd4f_38[0]) * (y_sum_tmp_3cd4f_39[3]))
                            + ((x_sum_tmp_3cd4f_38[1]) * (y_sum_tmp_3cd4f_39[2])))
                            + ((x_sum_tmp_3cd4f_38[2]) * (y_sum_tmp_3cd4f_39[1])))
                            + ((x_sum_tmp_3cd4f_38[3]) * (y_sum_tmp_3cd4f_39[0])))
                            - (z0_tmp_3cd4f_36[3]))
                            - (z2_tmp_3cd4f_37[3]))),
                    ((z0_tmp_3cd4f_36[11])
                        + ((((((((x_sum_tmp_3cd4f_38[0]) * (y_sum_tmp_3cd4f_39[4]))
                            + ((x_sum_tmp_3cd4f_38[1]) * (y_sum_tmp_3cd4f_39[3])))
                            + ((x_sum_tmp_3cd4f_38[2]) * (y_sum_tmp_3cd4f_39[2])))
                            + ((x_sum_tmp_3cd4f_38[3]) * (y_sum_tmp_3cd4f_39[1])))
                            + ((x_sum_tmp_3cd4f_38[4]) * (y_sum_tmp_3cd4f_39[0])))
                            - (z0_tmp_3cd4f_36[4]))
                            - (z2_tmp_3cd4f_37[4]))),
                    ((z0_tmp_3cd4f_36[12])
                        + (((((((((x_sum_tmp_3cd4f_38[0]) * (y_sum_tmp_3cd4f_39[5]))
                            + ((x_sum_tmp_3cd4f_38[1]) * (y_sum_tmp_3cd4f_39[4])))
                            + ((x_sum_tmp_3cd4f_38[2]) * (y_sum_tmp_3cd4f_39[3])))
                            + ((x_sum_tmp_3cd4f_38[3]) * (y_sum_tmp_3cd4f_39[2])))
                            + ((x_sum_tmp_3cd4f_38[4]) * (y_sum_tmp_3cd4f_39[1])))
                            + ((x_sum_tmp_3cd4f_38[5]) * (y_sum_tmp_3cd4f_39[0])))
                            - (z0_tmp_3cd4f_36[5]))
                            - (z2_tmp_3cd4f_37[5]))),
                    ((((((((((x_sum_tmp_3cd4f_38[0]) * (y_sum_tmp_3cd4f_39[6]))
                        + ((x_sum_tmp_3cd4f_38[1]) * (y_sum_tmp_3cd4f_39[5])))
                        + ((x_sum_tmp_3cd4f_38[2]) * (y_sum_tmp_3cd4f_39[4])))
                        + ((x_sum_tmp_3cd4f_38[3]) * (y_sum_tmp_3cd4f_39[3])))
                        + ((x_sum_tmp_3cd4f_38[4]) * (y_sum_tmp_3cd4f_39[2])))
                        + ((x_sum_tmp_3cd4f_38[5]) * (y_sum_tmp_3cd4f_39[1])))
                        + ((x_sum_tmp_3cd4f_38[6]) * (y_sum_tmp_3cd4f_39[0])))
                        - (z0_tmp_3cd4f_36[6]))
                        - (z2_tmp_3cd4f_37[6])),
                    ((z2_tmp_3cd4f_37[0])
                        + (((((((((x_sum_tmp_3cd4f_38[1]) * (y_sum_tmp_3cd4f_39[6]))
                            + ((x_sum_tmp_3cd4f_38[2]) * (y_sum_tmp_3cd4f_39[5])))
                            + ((x_sum_tmp_3cd4f_38[3]) * (y_sum_tmp_3cd4f_39[4])))
                            + ((x_sum_tmp_3cd4f_38[4]) * (y_sum_tmp_3cd4f_39[3])))
                            + ((x_sum_tmp_3cd4f_38[5]) * (y_sum_tmp_3cd4f_39[2])))
                            + ((x_sum_tmp_3cd4f_38[6]) * (y_sum_tmp_3cd4f_39[1])))
                            - (z0_tmp_3cd4f_36[7]))
                            - (z2_tmp_3cd4f_37[7]))),
                    ((z2_tmp_3cd4f_37[1])
                        + ((((((((x_sum_tmp_3cd4f_38[2]) * (y_sum_tmp_3cd4f_39[6]))
                            + ((x_sum_tmp_3cd4f_38[3]) * (y_sum_tmp_3cd4f_39[5])))
                            + ((x_sum_tmp_3cd4f_38[4]) * (y_sum_tmp_3cd4f_39[4])))
                            + ((x_sum_tmp_3cd4f_38[5]) * (y_sum_tmp_3cd4f_39[3])))
                            + ((x_sum_tmp_3cd4f_38[6]) * (y_sum_tmp_3cd4f_39[2])))
                            - (z0_tmp_3cd4f_36[8]))
                            - (z2_tmp_3cd4f_37[8]))),
                    ((z2_tmp_3cd4f_37[2])
                        + (((((((x_sum_tmp_3cd4f_38[3]) * (y_sum_tmp_3cd4f_39[6]))
                            + ((x_sum_tmp_3cd4f_38[4]) * (y_sum_tmp_3cd4f_39[5])))
                            + ((x_sum_tmp_3cd4f_38[5]) * (y_sum_tmp_3cd4f_39[4])))
                            + ((x_sum_tmp_3cd4f_38[6]) * (y_sum_tmp_3cd4f_39[3])))
                            - (z0_tmp_3cd4f_36[9]))
                            - (z2_tmp_3cd4f_37[9]))),
                    ((z2_tmp_3cd4f_37[3])
                        + ((((((x_sum_tmp_3cd4f_38[4]) * (y_sum_tmp_3cd4f_39[6]))
                            + ((x_sum_tmp_3cd4f_38[5]) * (y_sum_tmp_3cd4f_39[5])))
                            + ((x_sum_tmp_3cd4f_38[6]) * (y_sum_tmp_3cd4f_39[4])))
                            - (z0_tmp_3cd4f_36[10]))
                            - (z2_tmp_3cd4f_37[10]))),
                    ((z2_tmp_3cd4f_37[4])
                        + (((((x_sum_tmp_3cd4f_38[5]) * (y_sum_tmp_3cd4f_39[6]))
                            + ((x_sum_tmp_3cd4f_38[6]) * (y_sum_tmp_3cd4f_39[5])))
                            - (z0_tmp_3cd4f_36[11]))
                            - (z2_tmp_3cd4f_37[11]))),
                    ((z2_tmp_3cd4f_37[5])
                        + ((((x_sum_tmp_3cd4f_38[6]) * (y_sum_tmp_3cd4f_39[6]))
                            - (z0_tmp_3cd4f_36[12]))
                            - (z2_tmp_3cd4f_37[12]))),
                    z2_tmp_3cd4f_37[6],
                    z2_tmp_3cd4f_37[7],
                    z2_tmp_3cd4f_37[8],
                    z2_tmp_3cd4f_37[9],
                    z2_tmp_3cd4f_37[10],
                    z2_tmp_3cd4f_37[11],
                    z2_tmp_3cd4f_37[12],
                ];

                let double_karatsuba_1454b_output_tmp_3cd4f_41 = [
                    single_karatsuba_n_7_output_tmp_3cd4f_28[0],
                    single_karatsuba_n_7_output_tmp_3cd4f_28[1],
                    single_karatsuba_n_7_output_tmp_3cd4f_28[2],
                    single_karatsuba_n_7_output_tmp_3cd4f_28[3],
                    single_karatsuba_n_7_output_tmp_3cd4f_28[4],
                    single_karatsuba_n_7_output_tmp_3cd4f_28[5],
                    single_karatsuba_n_7_output_tmp_3cd4f_28[6],
                    single_karatsuba_n_7_output_tmp_3cd4f_28[7],
                    single_karatsuba_n_7_output_tmp_3cd4f_28[8],
                    single_karatsuba_n_7_output_tmp_3cd4f_28[9],
                    single_karatsuba_n_7_output_tmp_3cd4f_28[10],
                    single_karatsuba_n_7_output_tmp_3cd4f_28[11],
                    single_karatsuba_n_7_output_tmp_3cd4f_28[12],
                    single_karatsuba_n_7_output_tmp_3cd4f_28[13],
                    ((single_karatsuba_n_7_output_tmp_3cd4f_28[14])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[0])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[0]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[0]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_28[15])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[1])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[1]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[1]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_28[16])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[2])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[2]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[2]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_28[17])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[3])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[3]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[3]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_28[18])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[4])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[4]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[4]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_28[19])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[5])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[5]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[5]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_28[20])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[6])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[6]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[6]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_28[21])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[7])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[7]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[7]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_28[22])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[8])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[8]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[8]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_28[23])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[9])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[9]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[9]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_28[24])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[10])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[10]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[10]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_28[25])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[11])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[11]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[11]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_28[26])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[12])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[12]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[12]))),
                    (((single_karatsuba_n_7_output_tmp_3cd4f_40[13])
                        - (single_karatsuba_n_7_output_tmp_3cd4f_28[13]))
                        - (single_karatsuba_n_7_output_tmp_3cd4f_33[13])),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_33[0])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[14])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[14]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[14]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_33[1])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[15])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[15]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[15]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_33[2])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[16])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[16]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[16]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_33[3])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[17])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[17]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[17]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_33[4])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[18])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[18]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[18]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_33[5])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[19])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[19]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[19]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_33[6])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[20])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[20]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[20]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_33[7])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[21])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[21]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[21]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_33[8])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[22])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[22]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[22]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_33[9])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[23])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[23]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[23]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_33[10])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[24])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[24]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[24]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_33[11])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[25])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[25]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[25]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_33[12])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_40[26])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_28[26]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_33[26]))),
                    single_karatsuba_n_7_output_tmp_3cd4f_33[13],
                    single_karatsuba_n_7_output_tmp_3cd4f_33[14],
                    single_karatsuba_n_7_output_tmp_3cd4f_33[15],
                    single_karatsuba_n_7_output_tmp_3cd4f_33[16],
                    single_karatsuba_n_7_output_tmp_3cd4f_33[17],
                    single_karatsuba_n_7_output_tmp_3cd4f_33[18],
                    single_karatsuba_n_7_output_tmp_3cd4f_33[19],
                    single_karatsuba_n_7_output_tmp_3cd4f_33[20],
                    single_karatsuba_n_7_output_tmp_3cd4f_33[21],
                    single_karatsuba_n_7_output_tmp_3cd4f_33[22],
                    single_karatsuba_n_7_output_tmp_3cd4f_33[23],
                    single_karatsuba_n_7_output_tmp_3cd4f_33[24],
                    single_karatsuba_n_7_output_tmp_3cd4f_33[25],
                    single_karatsuba_n_7_output_tmp_3cd4f_33[26],
                ];

                let conv_tmp_3cd4f_42 = [
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[0])
                        - (((input_limb_16_col16)
                            + (pedersen_points_table_window_bits_18_output_limb_0_col72))
                            + (result_x_limb_0_col184))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[1])
                        - (((input_limb_17_col17)
                            + (pedersen_points_table_window_bits_18_output_limb_1_col73))
                            + (result_x_limb_1_col185))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[2])
                        - (((input_limb_18_col18)
                            + (pedersen_points_table_window_bits_18_output_limb_2_col74))
                            + (result_x_limb_2_col186))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[3])
                        - (((input_limb_19_col19)
                            + (pedersen_points_table_window_bits_18_output_limb_3_col75))
                            + (result_x_limb_3_col187))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[4])
                        - (((input_limb_20_col20)
                            + (pedersen_points_table_window_bits_18_output_limb_4_col76))
                            + (result_x_limb_4_col188))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[5])
                        - (((input_limb_21_col21)
                            + (pedersen_points_table_window_bits_18_output_limb_5_col77))
                            + (result_x_limb_5_col189))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[6])
                        - (((input_limb_22_col22)
                            + (pedersen_points_table_window_bits_18_output_limb_6_col78))
                            + (result_x_limb_6_col190))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[7])
                        - (((input_limb_23_col23)
                            + (pedersen_points_table_window_bits_18_output_limb_7_col79))
                            + (result_x_limb_7_col191))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[8])
                        - (((input_limb_24_col24)
                            + (pedersen_points_table_window_bits_18_output_limb_8_col80))
                            + (result_x_limb_8_col192))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[9])
                        - (((input_limb_25_col25)
                            + (pedersen_points_table_window_bits_18_output_limb_9_col81))
                            + (result_x_limb_9_col193))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[10])
                        - (((input_limb_26_col26)
                            + (pedersen_points_table_window_bits_18_output_limb_10_col82))
                            + (result_x_limb_10_col194))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[11])
                        - (((input_limb_27_col27)
                            + (pedersen_points_table_window_bits_18_output_limb_11_col83))
                            + (result_x_limb_11_col195))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[12])
                        - (((input_limb_28_col28)
                            + (pedersen_points_table_window_bits_18_output_limb_12_col84))
                            + (result_x_limb_12_col196))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[13])
                        - (((input_limb_29_col29)
                            + (pedersen_points_table_window_bits_18_output_limb_13_col85))
                            + (result_x_limb_13_col197))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[14])
                        - (((input_limb_30_col30)
                            + (pedersen_points_table_window_bits_18_output_limb_14_col86))
                            + (result_x_limb_14_col198))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[15])
                        - (((input_limb_31_col31)
                            + (pedersen_points_table_window_bits_18_output_limb_15_col87))
                            + (result_x_limb_15_col199))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[16])
                        - (((input_limb_32_col32)
                            + (pedersen_points_table_window_bits_18_output_limb_16_col88))
                            + (result_x_limb_16_col200))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[17])
                        - (((input_limb_33_col33)
                            + (pedersen_points_table_window_bits_18_output_limb_17_col89))
                            + (result_x_limb_17_col201))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[18])
                        - (((input_limb_34_col34)
                            + (pedersen_points_table_window_bits_18_output_limb_18_col90))
                            + (result_x_limb_18_col202))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[19])
                        - (((input_limb_35_col35)
                            + (pedersen_points_table_window_bits_18_output_limb_19_col91))
                            + (result_x_limb_19_col203))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[20])
                        - (((input_limb_36_col36)
                            + (pedersen_points_table_window_bits_18_output_limb_20_col92))
                            + (result_x_limb_20_col204))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[21])
                        - (((input_limb_37_col37)
                            + (pedersen_points_table_window_bits_18_output_limb_21_col93))
                            + (result_x_limb_21_col205))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[22])
                        - (((input_limb_38_col38)
                            + (pedersen_points_table_window_bits_18_output_limb_22_col94))
                            + (result_x_limb_22_col206))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[23])
                        - (((input_limb_39_col39)
                            + (pedersen_points_table_window_bits_18_output_limb_23_col95))
                            + (result_x_limb_23_col207))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[24])
                        - (((input_limb_40_col40)
                            + (pedersen_points_table_window_bits_18_output_limb_24_col96))
                            + (result_x_limb_24_col208))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[25])
                        - (((input_limb_41_col41)
                            + (pedersen_points_table_window_bits_18_output_limb_25_col97))
                            + (result_x_limb_25_col209))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[26])
                        - (((input_limb_42_col42)
                            + (pedersen_points_table_window_bits_18_output_limb_26_col98))
                            + (result_x_limb_26_col210))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_41[27])
                        - (((input_limb_43_col43)
                            + (pedersen_points_table_window_bits_18_output_limb_27_col99))
                            + (result_x_limb_27_col211))),
                    double_karatsuba_1454b_output_tmp_3cd4f_41[28],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[29],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[30],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[31],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[32],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[33],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[34],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[35],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[36],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[37],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[38],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[39],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[40],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[41],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[42],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[43],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[44],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[45],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[46],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[47],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[48],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[49],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[50],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[51],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[52],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[53],
                    double_karatsuba_1454b_output_tmp_3cd4f_41[54],
                ];
                let conv_mod_tmp_3cd4f_43 = [
                    ((((M31_32) * (conv_tmp_3cd4f_42[0])) - ((M31_4) * (conv_tmp_3cd4f_42[21])))
                        + ((M31_8) * (conv_tmp_3cd4f_42[49]))),
                    ((((conv_tmp_3cd4f_42[0]) + ((M31_32) * (conv_tmp_3cd4f_42[1])))
                        - ((M31_4) * (conv_tmp_3cd4f_42[22])))
                        + ((M31_8) * (conv_tmp_3cd4f_42[50]))),
                    ((((conv_tmp_3cd4f_42[1]) + ((M31_32) * (conv_tmp_3cd4f_42[2])))
                        - ((M31_4) * (conv_tmp_3cd4f_42[23])))
                        + ((M31_8) * (conv_tmp_3cd4f_42[51]))),
                    ((((conv_tmp_3cd4f_42[2]) + ((M31_32) * (conv_tmp_3cd4f_42[3])))
                        - ((M31_4) * (conv_tmp_3cd4f_42[24])))
                        + ((M31_8) * (conv_tmp_3cd4f_42[52]))),
                    ((((conv_tmp_3cd4f_42[3]) + ((M31_32) * (conv_tmp_3cd4f_42[4])))
                        - ((M31_4) * (conv_tmp_3cd4f_42[25])))
                        + ((M31_8) * (conv_tmp_3cd4f_42[53]))),
                    ((((conv_tmp_3cd4f_42[4]) + ((M31_32) * (conv_tmp_3cd4f_42[5])))
                        - ((M31_4) * (conv_tmp_3cd4f_42[26])))
                        + ((M31_8) * (conv_tmp_3cd4f_42[54]))),
                    (((conv_tmp_3cd4f_42[5]) + ((M31_32) * (conv_tmp_3cd4f_42[6])))
                        - ((M31_4) * (conv_tmp_3cd4f_42[27]))),
                    (((((M31_2) * (conv_tmp_3cd4f_42[0])) + (conv_tmp_3cd4f_42[6]))
                        + ((M31_32) * (conv_tmp_3cd4f_42[7])))
                        - ((M31_4) * (conv_tmp_3cd4f_42[28]))),
                    (((((M31_2) * (conv_tmp_3cd4f_42[1])) + (conv_tmp_3cd4f_42[7]))
                        + ((M31_32) * (conv_tmp_3cd4f_42[8])))
                        - ((M31_4) * (conv_tmp_3cd4f_42[29]))),
                    (((((M31_2) * (conv_tmp_3cd4f_42[2])) + (conv_tmp_3cd4f_42[8]))
                        + ((M31_32) * (conv_tmp_3cd4f_42[9])))
                        - ((M31_4) * (conv_tmp_3cd4f_42[30]))),
                    (((((M31_2) * (conv_tmp_3cd4f_42[3])) + (conv_tmp_3cd4f_42[9]))
                        + ((M31_32) * (conv_tmp_3cd4f_42[10])))
                        - ((M31_4) * (conv_tmp_3cd4f_42[31]))),
                    (((((M31_2) * (conv_tmp_3cd4f_42[4])) + (conv_tmp_3cd4f_42[10]))
                        + ((M31_32) * (conv_tmp_3cd4f_42[11])))
                        - ((M31_4) * (conv_tmp_3cd4f_42[32]))),
                    (((((M31_2) * (conv_tmp_3cd4f_42[5])) + (conv_tmp_3cd4f_42[11]))
                        + ((M31_32) * (conv_tmp_3cd4f_42[12])))
                        - ((M31_4) * (conv_tmp_3cd4f_42[33]))),
                    (((((M31_2) * (conv_tmp_3cd4f_42[6])) + (conv_tmp_3cd4f_42[12]))
                        + ((M31_32) * (conv_tmp_3cd4f_42[13])))
                        - ((M31_4) * (conv_tmp_3cd4f_42[34]))),
                    (((((M31_2) * (conv_tmp_3cd4f_42[7])) + (conv_tmp_3cd4f_42[13]))
                        + ((M31_32) * (conv_tmp_3cd4f_42[14])))
                        - ((M31_4) * (conv_tmp_3cd4f_42[35]))),
                    (((((M31_2) * (conv_tmp_3cd4f_42[8])) + (conv_tmp_3cd4f_42[14]))
                        + ((M31_32) * (conv_tmp_3cd4f_42[15])))
                        - ((M31_4) * (conv_tmp_3cd4f_42[36]))),
                    (((((M31_2) * (conv_tmp_3cd4f_42[9])) + (conv_tmp_3cd4f_42[15]))
                        + ((M31_32) * (conv_tmp_3cd4f_42[16])))
                        - ((M31_4) * (conv_tmp_3cd4f_42[37]))),
                    (((((M31_2) * (conv_tmp_3cd4f_42[10])) + (conv_tmp_3cd4f_42[16]))
                        + ((M31_32) * (conv_tmp_3cd4f_42[17])))
                        - ((M31_4) * (conv_tmp_3cd4f_42[38]))),
                    (((((M31_2) * (conv_tmp_3cd4f_42[11])) + (conv_tmp_3cd4f_42[17]))
                        + ((M31_32) * (conv_tmp_3cd4f_42[18])))
                        - ((M31_4) * (conv_tmp_3cd4f_42[39]))),
                    (((((M31_2) * (conv_tmp_3cd4f_42[12])) + (conv_tmp_3cd4f_42[18]))
                        + ((M31_32) * (conv_tmp_3cd4f_42[19])))
                        - ((M31_4) * (conv_tmp_3cd4f_42[40]))),
                    (((((M31_2) * (conv_tmp_3cd4f_42[13])) + (conv_tmp_3cd4f_42[19]))
                        + ((M31_32) * (conv_tmp_3cd4f_42[20])))
                        - ((M31_4) * (conv_tmp_3cd4f_42[41]))),
                    (((((M31_2) * (conv_tmp_3cd4f_42[14])) + (conv_tmp_3cd4f_42[20]))
                        - ((M31_4) * (conv_tmp_3cd4f_42[42])))
                        + ((M31_64) * (conv_tmp_3cd4f_42[49]))),
                    (((((M31_2) * (conv_tmp_3cd4f_42[15])) - ((M31_4) * (conv_tmp_3cd4f_42[43])))
                        + ((M31_2) * (conv_tmp_3cd4f_42[49])))
                        + ((M31_64) * (conv_tmp_3cd4f_42[50]))),
                    (((((M31_2) * (conv_tmp_3cd4f_42[16])) - ((M31_4) * (conv_tmp_3cd4f_42[44])))
                        + ((M31_2) * (conv_tmp_3cd4f_42[50])))
                        + ((M31_64) * (conv_tmp_3cd4f_42[51]))),
                    (((((M31_2) * (conv_tmp_3cd4f_42[17])) - ((M31_4) * (conv_tmp_3cd4f_42[45])))
                        + ((M31_2) * (conv_tmp_3cd4f_42[51])))
                        + ((M31_64) * (conv_tmp_3cd4f_42[52]))),
                    (((((M31_2) * (conv_tmp_3cd4f_42[18])) - ((M31_4) * (conv_tmp_3cd4f_42[46])))
                        + ((M31_2) * (conv_tmp_3cd4f_42[52])))
                        + ((M31_64) * (conv_tmp_3cd4f_42[53]))),
                    (((((M31_2) * (conv_tmp_3cd4f_42[19])) - ((M31_4) * (conv_tmp_3cd4f_42[47])))
                        + ((M31_2) * (conv_tmp_3cd4f_42[53])))
                        + ((M31_64) * (conv_tmp_3cd4f_42[54]))),
                    ((((M31_2) * (conv_tmp_3cd4f_42[20])) - ((M31_4) * (conv_tmp_3cd4f_42[48])))
                        + ((M31_2) * (conv_tmp_3cd4f_42[54]))),
                ];
                let k_mod_2_18_biased_tmp_3cd4f_44 =
                    ((((PackedUInt32::from_m31(((conv_mod_tmp_3cd4f_43[0]) + (M31_134217728))))
                        + (((PackedUInt32::from_m31(
                            ((conv_mod_tmp_3cd4f_43[1]) + (M31_134217728)),
                        )) & (UInt32_511))
                            << (UInt32_9)))
                        + (UInt32_131072))
                        & (UInt32_262143));
                let k_col212 = ((k_mod_2_18_biased_tmp_3cd4f_44.low().as_m31())
                    + (((k_mod_2_18_biased_tmp_3cd4f_44.high().as_m31()) - (M31_2)) * (M31_65536)));
                *row[212] = k_col212;
                *sub_component_inputs.range_check_20[4] = [((k_col212) + (M31_524288))];
                *lookup_data.range_check_20_4 = [((k_col212) + (M31_524288))];
                let carry_0_col213 = (((conv_mod_tmp_3cd4f_43[0]) - (k_col212)) * (M31_4194304));
                *row[213] = carry_0_col213;
                *sub_component_inputs.range_check_20_b[4] = [((carry_0_col213) + (M31_524288))];
                *lookup_data.range_check_20_b_4 = [((carry_0_col213) + (M31_524288))];
                let carry_1_col214 =
                    (((conv_mod_tmp_3cd4f_43[1]) + (carry_0_col213)) * (M31_4194304));
                *row[214] = carry_1_col214;
                *sub_component_inputs.range_check_20_c[4] = [((carry_1_col214) + (M31_524288))];
                *lookup_data.range_check_20_c_4 = [((carry_1_col214) + (M31_524288))];
                let carry_2_col215 =
                    (((conv_mod_tmp_3cd4f_43[2]) + (carry_1_col214)) * (M31_4194304));
                *row[215] = carry_2_col215;
                *sub_component_inputs.range_check_20_d[4] = [((carry_2_col215) + (M31_524288))];
                *lookup_data.range_check_20_d_4 = [((carry_2_col215) + (M31_524288))];
                let carry_3_col216 =
                    (((conv_mod_tmp_3cd4f_43[3]) + (carry_2_col215)) * (M31_4194304));
                *row[216] = carry_3_col216;
                *sub_component_inputs.range_check_20_e[3] = [((carry_3_col216) + (M31_524288))];
                *lookup_data.range_check_20_e_3 = [((carry_3_col216) + (M31_524288))];
                let carry_4_col217 =
                    (((conv_mod_tmp_3cd4f_43[4]) + (carry_3_col216)) * (M31_4194304));
                *row[217] = carry_4_col217;
                *sub_component_inputs.range_check_20_f[3] = [((carry_4_col217) + (M31_524288))];
                *lookup_data.range_check_20_f_3 = [((carry_4_col217) + (M31_524288))];
                let carry_5_col218 =
                    (((conv_mod_tmp_3cd4f_43[5]) + (carry_4_col217)) * (M31_4194304));
                *row[218] = carry_5_col218;
                *sub_component_inputs.range_check_20_g[3] = [((carry_5_col218) + (M31_524288))];
                *lookup_data.range_check_20_g_3 = [((carry_5_col218) + (M31_524288))];
                let carry_6_col219 =
                    (((conv_mod_tmp_3cd4f_43[6]) + (carry_5_col218)) * (M31_4194304));
                *row[219] = carry_6_col219;
                *sub_component_inputs.range_check_20_h[3] = [((carry_6_col219) + (M31_524288))];
                *lookup_data.range_check_20_h_3 = [((carry_6_col219) + (M31_524288))];
                let carry_7_col220 =
                    (((conv_mod_tmp_3cd4f_43[7]) + (carry_6_col219)) * (M31_4194304));
                *row[220] = carry_7_col220;
                *sub_component_inputs.range_check_20[5] = [((carry_7_col220) + (M31_524288))];
                *lookup_data.range_check_20_5 = [((carry_7_col220) + (M31_524288))];
                let carry_8_col221 =
                    (((conv_mod_tmp_3cd4f_43[8]) + (carry_7_col220)) * (M31_4194304));
                *row[221] = carry_8_col221;
                *sub_component_inputs.range_check_20_b[5] = [((carry_8_col221) + (M31_524288))];
                *lookup_data.range_check_20_b_5 = [((carry_8_col221) + (M31_524288))];
                let carry_9_col222 =
                    (((conv_mod_tmp_3cd4f_43[9]) + (carry_8_col221)) * (M31_4194304));
                *row[222] = carry_9_col222;
                *sub_component_inputs.range_check_20_c[5] = [((carry_9_col222) + (M31_524288))];
                *lookup_data.range_check_20_c_5 = [((carry_9_col222) + (M31_524288))];
                let carry_10_col223 =
                    (((conv_mod_tmp_3cd4f_43[10]) + (carry_9_col222)) * (M31_4194304));
                *row[223] = carry_10_col223;
                *sub_component_inputs.range_check_20_d[5] = [((carry_10_col223) + (M31_524288))];
                *lookup_data.range_check_20_d_5 = [((carry_10_col223) + (M31_524288))];
                let carry_11_col224 =
                    (((conv_mod_tmp_3cd4f_43[11]) + (carry_10_col223)) * (M31_4194304));
                *row[224] = carry_11_col224;
                *sub_component_inputs.range_check_20_e[4] = [((carry_11_col224) + (M31_524288))];
                *lookup_data.range_check_20_e_4 = [((carry_11_col224) + (M31_524288))];
                let carry_12_col225 =
                    (((conv_mod_tmp_3cd4f_43[12]) + (carry_11_col224)) * (M31_4194304));
                *row[225] = carry_12_col225;
                *sub_component_inputs.range_check_20_f[4] = [((carry_12_col225) + (M31_524288))];
                *lookup_data.range_check_20_f_4 = [((carry_12_col225) + (M31_524288))];
                let carry_13_col226 =
                    (((conv_mod_tmp_3cd4f_43[13]) + (carry_12_col225)) * (M31_4194304));
                *row[226] = carry_13_col226;
                *sub_component_inputs.range_check_20_g[4] = [((carry_13_col226) + (M31_524288))];
                *lookup_data.range_check_20_g_4 = [((carry_13_col226) + (M31_524288))];
                let carry_14_col227 =
                    (((conv_mod_tmp_3cd4f_43[14]) + (carry_13_col226)) * (M31_4194304));
                *row[227] = carry_14_col227;
                *sub_component_inputs.range_check_20_h[4] = [((carry_14_col227) + (M31_524288))];
                *lookup_data.range_check_20_h_4 = [((carry_14_col227) + (M31_524288))];
                let carry_15_col228 =
                    (((conv_mod_tmp_3cd4f_43[15]) + (carry_14_col227)) * (M31_4194304));
                *row[228] = carry_15_col228;
                *sub_component_inputs.range_check_20[6] = [((carry_15_col228) + (M31_524288))];
                *lookup_data.range_check_20_6 = [((carry_15_col228) + (M31_524288))];
                let carry_16_col229 =
                    (((conv_mod_tmp_3cd4f_43[16]) + (carry_15_col228)) * (M31_4194304));
                *row[229] = carry_16_col229;
                *sub_component_inputs.range_check_20_b[6] = [((carry_16_col229) + (M31_524288))];
                *lookup_data.range_check_20_b_6 = [((carry_16_col229) + (M31_524288))];
                let carry_17_col230 =
                    (((conv_mod_tmp_3cd4f_43[17]) + (carry_16_col229)) * (M31_4194304));
                *row[230] = carry_17_col230;
                *sub_component_inputs.range_check_20_c[6] = [((carry_17_col230) + (M31_524288))];
                *lookup_data.range_check_20_c_6 = [((carry_17_col230) + (M31_524288))];
                let carry_18_col231 =
                    (((conv_mod_tmp_3cd4f_43[18]) + (carry_17_col230)) * (M31_4194304));
                *row[231] = carry_18_col231;
                *sub_component_inputs.range_check_20_d[6] = [((carry_18_col231) + (M31_524288))];
                *lookup_data.range_check_20_d_6 = [((carry_18_col231) + (M31_524288))];
                let carry_19_col232 =
                    (((conv_mod_tmp_3cd4f_43[19]) + (carry_18_col231)) * (M31_4194304));
                *row[232] = carry_19_col232;
                *sub_component_inputs.range_check_20_e[5] = [((carry_19_col232) + (M31_524288))];
                *lookup_data.range_check_20_e_5 = [((carry_19_col232) + (M31_524288))];
                let carry_20_col233 =
                    (((conv_mod_tmp_3cd4f_43[20]) + (carry_19_col232)) * (M31_4194304));
                *row[233] = carry_20_col233;
                *sub_component_inputs.range_check_20_f[5] = [((carry_20_col233) + (M31_524288))];
                *lookup_data.range_check_20_f_5 = [((carry_20_col233) + (M31_524288))];
                let carry_21_col234 = ((((conv_mod_tmp_3cd4f_43[21]) - ((M31_136) * (k_col212)))
                    + (carry_20_col233))
                    * (M31_4194304));
                *row[234] = carry_21_col234;
                *sub_component_inputs.range_check_20_g[5] = [((carry_21_col234) + (M31_524288))];
                *lookup_data.range_check_20_g_5 = [((carry_21_col234) + (M31_524288))];
                let carry_22_col235 =
                    (((conv_mod_tmp_3cd4f_43[22]) + (carry_21_col234)) * (M31_4194304));
                *row[235] = carry_22_col235;
                *sub_component_inputs.range_check_20_h[5] = [((carry_22_col235) + (M31_524288))];
                *lookup_data.range_check_20_h_5 = [((carry_22_col235) + (M31_524288))];
                let carry_23_col236 =
                    (((conv_mod_tmp_3cd4f_43[23]) + (carry_22_col235)) * (M31_4194304));
                *row[236] = carry_23_col236;
                *sub_component_inputs.range_check_20[7] = [((carry_23_col236) + (M31_524288))];
                *lookup_data.range_check_20_7 = [((carry_23_col236) + (M31_524288))];
                let carry_24_col237 =
                    (((conv_mod_tmp_3cd4f_43[24]) + (carry_23_col236)) * (M31_4194304));
                *row[237] = carry_24_col237;
                *sub_component_inputs.range_check_20_b[7] = [((carry_24_col237) + (M31_524288))];
                *lookup_data.range_check_20_b_7 = [((carry_24_col237) + (M31_524288))];
                let carry_25_col238 =
                    (((conv_mod_tmp_3cd4f_43[25]) + (carry_24_col237)) * (M31_4194304));
                *row[238] = carry_25_col238;
                *sub_component_inputs.range_check_20_c[7] = [((carry_25_col238) + (M31_524288))];
                *lookup_data.range_check_20_c_7 = [((carry_25_col238) + (M31_524288))];
                let carry_26_col239 =
                    (((conv_mod_tmp_3cd4f_43[26]) + (carry_25_col238)) * (M31_4194304));
                *row[239] = carry_26_col239;
                *sub_component_inputs.range_check_20_d[7] = [((carry_26_col239) + (M31_524288))];
                *lookup_data.range_check_20_d_7 = [((carry_26_col239) + (M31_524288))];

                let result_y_tmp_3cd4f_45 = (((slope_tmp_3cd4f_1)
                    * ((partial_ec_mul_window_bits_18_input.2 .1[0]) - (result_x_tmp_3cd4f_23)))
                    - (partial_ec_mul_window_bits_18_input.2 .1[1]));
                let result_y_limb_0_col240 = result_y_tmp_3cd4f_45.get_m31(0);
                *row[240] = result_y_limb_0_col240;
                let result_y_limb_1_col241 = result_y_tmp_3cd4f_45.get_m31(1);
                *row[241] = result_y_limb_1_col241;
                let result_y_limb_2_col242 = result_y_tmp_3cd4f_45.get_m31(2);
                *row[242] = result_y_limb_2_col242;
                let result_y_limb_3_col243 = result_y_tmp_3cd4f_45.get_m31(3);
                *row[243] = result_y_limb_3_col243;
                let result_y_limb_4_col244 = result_y_tmp_3cd4f_45.get_m31(4);
                *row[244] = result_y_limb_4_col244;
                let result_y_limb_5_col245 = result_y_tmp_3cd4f_45.get_m31(5);
                *row[245] = result_y_limb_5_col245;
                let result_y_limb_6_col246 = result_y_tmp_3cd4f_45.get_m31(6);
                *row[246] = result_y_limb_6_col246;
                let result_y_limb_7_col247 = result_y_tmp_3cd4f_45.get_m31(7);
                *row[247] = result_y_limb_7_col247;
                let result_y_limb_8_col248 = result_y_tmp_3cd4f_45.get_m31(8);
                *row[248] = result_y_limb_8_col248;
                let result_y_limb_9_col249 = result_y_tmp_3cd4f_45.get_m31(9);
                *row[249] = result_y_limb_9_col249;
                let result_y_limb_10_col250 = result_y_tmp_3cd4f_45.get_m31(10);
                *row[250] = result_y_limb_10_col250;
                let result_y_limb_11_col251 = result_y_tmp_3cd4f_45.get_m31(11);
                *row[251] = result_y_limb_11_col251;
                let result_y_limb_12_col252 = result_y_tmp_3cd4f_45.get_m31(12);
                *row[252] = result_y_limb_12_col252;
                let result_y_limb_13_col253 = result_y_tmp_3cd4f_45.get_m31(13);
                *row[253] = result_y_limb_13_col253;
                let result_y_limb_14_col254 = result_y_tmp_3cd4f_45.get_m31(14);
                *row[254] = result_y_limb_14_col254;
                let result_y_limb_15_col255 = result_y_tmp_3cd4f_45.get_m31(15);
                *row[255] = result_y_limb_15_col255;
                let result_y_limb_16_col256 = result_y_tmp_3cd4f_45.get_m31(16);
                *row[256] = result_y_limb_16_col256;
                let result_y_limb_17_col257 = result_y_tmp_3cd4f_45.get_m31(17);
                *row[257] = result_y_limb_17_col257;
                let result_y_limb_18_col258 = result_y_tmp_3cd4f_45.get_m31(18);
                *row[258] = result_y_limb_18_col258;
                let result_y_limb_19_col259 = result_y_tmp_3cd4f_45.get_m31(19);
                *row[259] = result_y_limb_19_col259;
                let result_y_limb_20_col260 = result_y_tmp_3cd4f_45.get_m31(20);
                *row[260] = result_y_limb_20_col260;
                let result_y_limb_21_col261 = result_y_tmp_3cd4f_45.get_m31(21);
                *row[261] = result_y_limb_21_col261;
                let result_y_limb_22_col262 = result_y_tmp_3cd4f_45.get_m31(22);
                *row[262] = result_y_limb_22_col262;
                let result_y_limb_23_col263 = result_y_tmp_3cd4f_45.get_m31(23);
                *row[263] = result_y_limb_23_col263;
                let result_y_limb_24_col264 = result_y_tmp_3cd4f_45.get_m31(24);
                *row[264] = result_y_limb_24_col264;
                let result_y_limb_25_col265 = result_y_tmp_3cd4f_45.get_m31(25);
                *row[265] = result_y_limb_25_col265;
                let result_y_limb_26_col266 = result_y_tmp_3cd4f_45.get_m31(26);
                *row[266] = result_y_limb_26_col266;
                let result_y_limb_27_col267 = result_y_tmp_3cd4f_45.get_m31(27);
                *row[267] = result_y_limb_27_col267;

                // Range Check Mem Value N 28.

                *sub_component_inputs.range_check_9_9[4] =
                    [result_y_limb_0_col240, result_y_limb_1_col241];
                *lookup_data.range_check_9_9_4 = [result_y_limb_0_col240, result_y_limb_1_col241];
                *sub_component_inputs.range_check_9_9_b[4] =
                    [result_y_limb_2_col242, result_y_limb_3_col243];
                *lookup_data.range_check_9_9_b_4 = [result_y_limb_2_col242, result_y_limb_3_col243];
                *sub_component_inputs.range_check_9_9_c[4] =
                    [result_y_limb_4_col244, result_y_limb_5_col245];
                *lookup_data.range_check_9_9_c_4 = [result_y_limb_4_col244, result_y_limb_5_col245];
                *sub_component_inputs.range_check_9_9_d[4] =
                    [result_y_limb_6_col246, result_y_limb_7_col247];
                *lookup_data.range_check_9_9_d_4 = [result_y_limb_6_col246, result_y_limb_7_col247];
                *sub_component_inputs.range_check_9_9_e[4] =
                    [result_y_limb_8_col248, result_y_limb_9_col249];
                *lookup_data.range_check_9_9_e_4 = [result_y_limb_8_col248, result_y_limb_9_col249];
                *sub_component_inputs.range_check_9_9_f[4] =
                    [result_y_limb_10_col250, result_y_limb_11_col251];
                *lookup_data.range_check_9_9_f_4 =
                    [result_y_limb_10_col250, result_y_limb_11_col251];
                *sub_component_inputs.range_check_9_9_g[2] =
                    [result_y_limb_12_col252, result_y_limb_13_col253];
                *lookup_data.range_check_9_9_g_2 =
                    [result_y_limb_12_col252, result_y_limb_13_col253];
                *sub_component_inputs.range_check_9_9_h[2] =
                    [result_y_limb_14_col254, result_y_limb_15_col255];
                *lookup_data.range_check_9_9_h_2 =
                    [result_y_limb_14_col254, result_y_limb_15_col255];
                *sub_component_inputs.range_check_9_9[5] =
                    [result_y_limb_16_col256, result_y_limb_17_col257];
                *lookup_data.range_check_9_9_5 = [result_y_limb_16_col256, result_y_limb_17_col257];
                *sub_component_inputs.range_check_9_9_b[5] =
                    [result_y_limb_18_col258, result_y_limb_19_col259];
                *lookup_data.range_check_9_9_b_5 =
                    [result_y_limb_18_col258, result_y_limb_19_col259];
                *sub_component_inputs.range_check_9_9_c[5] =
                    [result_y_limb_20_col260, result_y_limb_21_col261];
                *lookup_data.range_check_9_9_c_5 =
                    [result_y_limb_20_col260, result_y_limb_21_col261];
                *sub_component_inputs.range_check_9_9_d[5] =
                    [result_y_limb_22_col262, result_y_limb_23_col263];
                *lookup_data.range_check_9_9_d_5 =
                    [result_y_limb_22_col262, result_y_limb_23_col263];
                *sub_component_inputs.range_check_9_9_e[5] =
                    [result_y_limb_24_col264, result_y_limb_25_col265];
                *lookup_data.range_check_9_9_e_5 =
                    [result_y_limb_24_col264, result_y_limb_25_col265];
                *sub_component_inputs.range_check_9_9_f[5] =
                    [result_y_limb_26_col266, result_y_limb_27_col267];
                *lookup_data.range_check_9_9_f_5 =
                    [result_y_limb_26_col266, result_y_limb_27_col267];

                // Verify Mul 252.

                // Double Karatsuba 1454 B.

                // Single Karatsuba N 7.

                let z0_tmp_3cd4f_46 = [
                    ((slope_limb_0_col128) * ((input_limb_16_col16) - (result_x_limb_0_col184))),
                    (((slope_limb_0_col128) * ((input_limb_17_col17) - (result_x_limb_1_col185)))
                        + ((slope_limb_1_col129)
                            * ((input_limb_16_col16) - (result_x_limb_0_col184)))),
                    ((((slope_limb_0_col128)
                        * ((input_limb_18_col18) - (result_x_limb_2_col186)))
                        + ((slope_limb_1_col129)
                            * ((input_limb_17_col17) - (result_x_limb_1_col185))))
                        + ((slope_limb_2_col130)
                            * ((input_limb_16_col16) - (result_x_limb_0_col184)))),
                    (((((slope_limb_0_col128)
                        * ((input_limb_19_col19) - (result_x_limb_3_col187)))
                        + ((slope_limb_1_col129)
                            * ((input_limb_18_col18) - (result_x_limb_2_col186))))
                        + ((slope_limb_2_col130)
                            * ((input_limb_17_col17) - (result_x_limb_1_col185))))
                        + ((slope_limb_3_col131)
                            * ((input_limb_16_col16) - (result_x_limb_0_col184)))),
                    ((((((slope_limb_0_col128)
                        * ((input_limb_20_col20) - (result_x_limb_4_col188)))
                        + ((slope_limb_1_col129)
                            * ((input_limb_19_col19) - (result_x_limb_3_col187))))
                        + ((slope_limb_2_col130)
                            * ((input_limb_18_col18) - (result_x_limb_2_col186))))
                        + ((slope_limb_3_col131)
                            * ((input_limb_17_col17) - (result_x_limb_1_col185))))
                        + ((slope_limb_4_col132)
                            * ((input_limb_16_col16) - (result_x_limb_0_col184)))),
                    (((((((slope_limb_0_col128)
                        * ((input_limb_21_col21) - (result_x_limb_5_col189)))
                        + ((slope_limb_1_col129)
                            * ((input_limb_20_col20) - (result_x_limb_4_col188))))
                        + ((slope_limb_2_col130)
                            * ((input_limb_19_col19) - (result_x_limb_3_col187))))
                        + ((slope_limb_3_col131)
                            * ((input_limb_18_col18) - (result_x_limb_2_col186))))
                        + ((slope_limb_4_col132)
                            * ((input_limb_17_col17) - (result_x_limb_1_col185))))
                        + ((slope_limb_5_col133)
                            * ((input_limb_16_col16) - (result_x_limb_0_col184)))),
                    ((((((((slope_limb_0_col128)
                        * ((input_limb_22_col22) - (result_x_limb_6_col190)))
                        + ((slope_limb_1_col129)
                            * ((input_limb_21_col21) - (result_x_limb_5_col189))))
                        + ((slope_limb_2_col130)
                            * ((input_limb_20_col20) - (result_x_limb_4_col188))))
                        + ((slope_limb_3_col131)
                            * ((input_limb_19_col19) - (result_x_limb_3_col187))))
                        + ((slope_limb_4_col132)
                            * ((input_limb_18_col18) - (result_x_limb_2_col186))))
                        + ((slope_limb_5_col133)
                            * ((input_limb_17_col17) - (result_x_limb_1_col185))))
                        + ((slope_limb_6_col134)
                            * ((input_limb_16_col16) - (result_x_limb_0_col184)))),
                    (((((((slope_limb_1_col129)
                        * ((input_limb_22_col22) - (result_x_limb_6_col190)))
                        + ((slope_limb_2_col130)
                            * ((input_limb_21_col21) - (result_x_limb_5_col189))))
                        + ((slope_limb_3_col131)
                            * ((input_limb_20_col20) - (result_x_limb_4_col188))))
                        + ((slope_limb_4_col132)
                            * ((input_limb_19_col19) - (result_x_limb_3_col187))))
                        + ((slope_limb_5_col133)
                            * ((input_limb_18_col18) - (result_x_limb_2_col186))))
                        + ((slope_limb_6_col134)
                            * ((input_limb_17_col17) - (result_x_limb_1_col185)))),
                    ((((((slope_limb_2_col130)
                        * ((input_limb_22_col22) - (result_x_limb_6_col190)))
                        + ((slope_limb_3_col131)
                            * ((input_limb_21_col21) - (result_x_limb_5_col189))))
                        + ((slope_limb_4_col132)
                            * ((input_limb_20_col20) - (result_x_limb_4_col188))))
                        + ((slope_limb_5_col133)
                            * ((input_limb_19_col19) - (result_x_limb_3_col187))))
                        + ((slope_limb_6_col134)
                            * ((input_limb_18_col18) - (result_x_limb_2_col186)))),
                    (((((slope_limb_3_col131)
                        * ((input_limb_22_col22) - (result_x_limb_6_col190)))
                        + ((slope_limb_4_col132)
                            * ((input_limb_21_col21) - (result_x_limb_5_col189))))
                        + ((slope_limb_5_col133)
                            * ((input_limb_20_col20) - (result_x_limb_4_col188))))
                        + ((slope_limb_6_col134)
                            * ((input_limb_19_col19) - (result_x_limb_3_col187)))),
                    ((((slope_limb_4_col132)
                        * ((input_limb_22_col22) - (result_x_limb_6_col190)))
                        + ((slope_limb_5_col133)
                            * ((input_limb_21_col21) - (result_x_limb_5_col189))))
                        + ((slope_limb_6_col134)
                            * ((input_limb_20_col20) - (result_x_limb_4_col188)))),
                    (((slope_limb_5_col133) * ((input_limb_22_col22) - (result_x_limb_6_col190)))
                        + ((slope_limb_6_col134)
                            * ((input_limb_21_col21) - (result_x_limb_5_col189)))),
                    ((slope_limb_6_col134) * ((input_limb_22_col22) - (result_x_limb_6_col190))),
                ];
                let z2_tmp_3cd4f_47 = [
                    ((slope_limb_7_col135) * ((input_limb_23_col23) - (result_x_limb_7_col191))),
                    (((slope_limb_7_col135) * ((input_limb_24_col24) - (result_x_limb_8_col192)))
                        + ((slope_limb_8_col136)
                            * ((input_limb_23_col23) - (result_x_limb_7_col191)))),
                    ((((slope_limb_7_col135)
                        * ((input_limb_25_col25) - (result_x_limb_9_col193)))
                        + ((slope_limb_8_col136)
                            * ((input_limb_24_col24) - (result_x_limb_8_col192))))
                        + ((slope_limb_9_col137)
                            * ((input_limb_23_col23) - (result_x_limb_7_col191)))),
                    (((((slope_limb_7_col135)
                        * ((input_limb_26_col26) - (result_x_limb_10_col194)))
                        + ((slope_limb_8_col136)
                            * ((input_limb_25_col25) - (result_x_limb_9_col193))))
                        + ((slope_limb_9_col137)
                            * ((input_limb_24_col24) - (result_x_limb_8_col192))))
                        + ((slope_limb_10_col138)
                            * ((input_limb_23_col23) - (result_x_limb_7_col191)))),
                    ((((((slope_limb_7_col135)
                        * ((input_limb_27_col27) - (result_x_limb_11_col195)))
                        + ((slope_limb_8_col136)
                            * ((input_limb_26_col26) - (result_x_limb_10_col194))))
                        + ((slope_limb_9_col137)
                            * ((input_limb_25_col25) - (result_x_limb_9_col193))))
                        + ((slope_limb_10_col138)
                            * ((input_limb_24_col24) - (result_x_limb_8_col192))))
                        + ((slope_limb_11_col139)
                            * ((input_limb_23_col23) - (result_x_limb_7_col191)))),
                    (((((((slope_limb_7_col135)
                        * ((input_limb_28_col28) - (result_x_limb_12_col196)))
                        + ((slope_limb_8_col136)
                            * ((input_limb_27_col27) - (result_x_limb_11_col195))))
                        + ((slope_limb_9_col137)
                            * ((input_limb_26_col26) - (result_x_limb_10_col194))))
                        + ((slope_limb_10_col138)
                            * ((input_limb_25_col25) - (result_x_limb_9_col193))))
                        + ((slope_limb_11_col139)
                            * ((input_limb_24_col24) - (result_x_limb_8_col192))))
                        + ((slope_limb_12_col140)
                            * ((input_limb_23_col23) - (result_x_limb_7_col191)))),
                    ((((((((slope_limb_7_col135)
                        * ((input_limb_29_col29) - (result_x_limb_13_col197)))
                        + ((slope_limb_8_col136)
                            * ((input_limb_28_col28) - (result_x_limb_12_col196))))
                        + ((slope_limb_9_col137)
                            * ((input_limb_27_col27) - (result_x_limb_11_col195))))
                        + ((slope_limb_10_col138)
                            * ((input_limb_26_col26) - (result_x_limb_10_col194))))
                        + ((slope_limb_11_col139)
                            * ((input_limb_25_col25) - (result_x_limb_9_col193))))
                        + ((slope_limb_12_col140)
                            * ((input_limb_24_col24) - (result_x_limb_8_col192))))
                        + ((slope_limb_13_col141)
                            * ((input_limb_23_col23) - (result_x_limb_7_col191)))),
                    (((((((slope_limb_8_col136)
                        * ((input_limb_29_col29) - (result_x_limb_13_col197)))
                        + ((slope_limb_9_col137)
                            * ((input_limb_28_col28) - (result_x_limb_12_col196))))
                        + ((slope_limb_10_col138)
                            * ((input_limb_27_col27) - (result_x_limb_11_col195))))
                        + ((slope_limb_11_col139)
                            * ((input_limb_26_col26) - (result_x_limb_10_col194))))
                        + ((slope_limb_12_col140)
                            * ((input_limb_25_col25) - (result_x_limb_9_col193))))
                        + ((slope_limb_13_col141)
                            * ((input_limb_24_col24) - (result_x_limb_8_col192)))),
                    ((((((slope_limb_9_col137)
                        * ((input_limb_29_col29) - (result_x_limb_13_col197)))
                        + ((slope_limb_10_col138)
                            * ((input_limb_28_col28) - (result_x_limb_12_col196))))
                        + ((slope_limb_11_col139)
                            * ((input_limb_27_col27) - (result_x_limb_11_col195))))
                        + ((slope_limb_12_col140)
                            * ((input_limb_26_col26) - (result_x_limb_10_col194))))
                        + ((slope_limb_13_col141)
                            * ((input_limb_25_col25) - (result_x_limb_9_col193)))),
                    (((((slope_limb_10_col138)
                        * ((input_limb_29_col29) - (result_x_limb_13_col197)))
                        + ((slope_limb_11_col139)
                            * ((input_limb_28_col28) - (result_x_limb_12_col196))))
                        + ((slope_limb_12_col140)
                            * ((input_limb_27_col27) - (result_x_limb_11_col195))))
                        + ((slope_limb_13_col141)
                            * ((input_limb_26_col26) - (result_x_limb_10_col194)))),
                    ((((slope_limb_11_col139)
                        * ((input_limb_29_col29) - (result_x_limb_13_col197)))
                        + ((slope_limb_12_col140)
                            * ((input_limb_28_col28) - (result_x_limb_12_col196))))
                        + ((slope_limb_13_col141)
                            * ((input_limb_27_col27) - (result_x_limb_11_col195)))),
                    (((slope_limb_12_col140)
                        * ((input_limb_29_col29) - (result_x_limb_13_col197)))
                        + ((slope_limb_13_col141)
                            * ((input_limb_28_col28) - (result_x_limb_12_col196)))),
                    ((slope_limb_13_col141) * ((input_limb_29_col29) - (result_x_limb_13_col197))),
                ];
                let x_sum_tmp_3cd4f_48 = [
                    ((slope_limb_0_col128) + (slope_limb_7_col135)),
                    ((slope_limb_1_col129) + (slope_limb_8_col136)),
                    ((slope_limb_2_col130) + (slope_limb_9_col137)),
                    ((slope_limb_3_col131) + (slope_limb_10_col138)),
                    ((slope_limb_4_col132) + (slope_limb_11_col139)),
                    ((slope_limb_5_col133) + (slope_limb_12_col140)),
                    ((slope_limb_6_col134) + (slope_limb_13_col141)),
                ];
                let y_sum_tmp_3cd4f_49 = [
                    (((input_limb_16_col16) - (result_x_limb_0_col184))
                        + ((input_limb_23_col23) - (result_x_limb_7_col191))),
                    (((input_limb_17_col17) - (result_x_limb_1_col185))
                        + ((input_limb_24_col24) - (result_x_limb_8_col192))),
                    (((input_limb_18_col18) - (result_x_limb_2_col186))
                        + ((input_limb_25_col25) - (result_x_limb_9_col193))),
                    (((input_limb_19_col19) - (result_x_limb_3_col187))
                        + ((input_limb_26_col26) - (result_x_limb_10_col194))),
                    (((input_limb_20_col20) - (result_x_limb_4_col188))
                        + ((input_limb_27_col27) - (result_x_limb_11_col195))),
                    (((input_limb_21_col21) - (result_x_limb_5_col189))
                        + ((input_limb_28_col28) - (result_x_limb_12_col196))),
                    (((input_limb_22_col22) - (result_x_limb_6_col190))
                        + ((input_limb_29_col29) - (result_x_limb_13_col197))),
                ];
                let single_karatsuba_n_7_output_tmp_3cd4f_50 = [
                    z0_tmp_3cd4f_46[0],
                    z0_tmp_3cd4f_46[1],
                    z0_tmp_3cd4f_46[2],
                    z0_tmp_3cd4f_46[3],
                    z0_tmp_3cd4f_46[4],
                    z0_tmp_3cd4f_46[5],
                    z0_tmp_3cd4f_46[6],
                    ((z0_tmp_3cd4f_46[7])
                        + ((((x_sum_tmp_3cd4f_48[0]) * (y_sum_tmp_3cd4f_49[0]))
                            - (z0_tmp_3cd4f_46[0]))
                            - (z2_tmp_3cd4f_47[0]))),
                    ((z0_tmp_3cd4f_46[8])
                        + (((((x_sum_tmp_3cd4f_48[0]) * (y_sum_tmp_3cd4f_49[1]))
                            + ((x_sum_tmp_3cd4f_48[1]) * (y_sum_tmp_3cd4f_49[0])))
                            - (z0_tmp_3cd4f_46[1]))
                            - (z2_tmp_3cd4f_47[1]))),
                    ((z0_tmp_3cd4f_46[9])
                        + ((((((x_sum_tmp_3cd4f_48[0]) * (y_sum_tmp_3cd4f_49[2]))
                            + ((x_sum_tmp_3cd4f_48[1]) * (y_sum_tmp_3cd4f_49[1])))
                            + ((x_sum_tmp_3cd4f_48[2]) * (y_sum_tmp_3cd4f_49[0])))
                            - (z0_tmp_3cd4f_46[2]))
                            - (z2_tmp_3cd4f_47[2]))),
                    ((z0_tmp_3cd4f_46[10])
                        + (((((((x_sum_tmp_3cd4f_48[0]) * (y_sum_tmp_3cd4f_49[3]))
                            + ((x_sum_tmp_3cd4f_48[1]) * (y_sum_tmp_3cd4f_49[2])))
                            + ((x_sum_tmp_3cd4f_48[2]) * (y_sum_tmp_3cd4f_49[1])))
                            + ((x_sum_tmp_3cd4f_48[3]) * (y_sum_tmp_3cd4f_49[0])))
                            - (z0_tmp_3cd4f_46[3]))
                            - (z2_tmp_3cd4f_47[3]))),
                    ((z0_tmp_3cd4f_46[11])
                        + ((((((((x_sum_tmp_3cd4f_48[0]) * (y_sum_tmp_3cd4f_49[4]))
                            + ((x_sum_tmp_3cd4f_48[1]) * (y_sum_tmp_3cd4f_49[3])))
                            + ((x_sum_tmp_3cd4f_48[2]) * (y_sum_tmp_3cd4f_49[2])))
                            + ((x_sum_tmp_3cd4f_48[3]) * (y_sum_tmp_3cd4f_49[1])))
                            + ((x_sum_tmp_3cd4f_48[4]) * (y_sum_tmp_3cd4f_49[0])))
                            - (z0_tmp_3cd4f_46[4]))
                            - (z2_tmp_3cd4f_47[4]))),
                    ((z0_tmp_3cd4f_46[12])
                        + (((((((((x_sum_tmp_3cd4f_48[0]) * (y_sum_tmp_3cd4f_49[5]))
                            + ((x_sum_tmp_3cd4f_48[1]) * (y_sum_tmp_3cd4f_49[4])))
                            + ((x_sum_tmp_3cd4f_48[2]) * (y_sum_tmp_3cd4f_49[3])))
                            + ((x_sum_tmp_3cd4f_48[3]) * (y_sum_tmp_3cd4f_49[2])))
                            + ((x_sum_tmp_3cd4f_48[4]) * (y_sum_tmp_3cd4f_49[1])))
                            + ((x_sum_tmp_3cd4f_48[5]) * (y_sum_tmp_3cd4f_49[0])))
                            - (z0_tmp_3cd4f_46[5]))
                            - (z2_tmp_3cd4f_47[5]))),
                    ((((((((((x_sum_tmp_3cd4f_48[0]) * (y_sum_tmp_3cd4f_49[6]))
                        + ((x_sum_tmp_3cd4f_48[1]) * (y_sum_tmp_3cd4f_49[5])))
                        + ((x_sum_tmp_3cd4f_48[2]) * (y_sum_tmp_3cd4f_49[4])))
                        + ((x_sum_tmp_3cd4f_48[3]) * (y_sum_tmp_3cd4f_49[3])))
                        + ((x_sum_tmp_3cd4f_48[4]) * (y_sum_tmp_3cd4f_49[2])))
                        + ((x_sum_tmp_3cd4f_48[5]) * (y_sum_tmp_3cd4f_49[1])))
                        + ((x_sum_tmp_3cd4f_48[6]) * (y_sum_tmp_3cd4f_49[0])))
                        - (z0_tmp_3cd4f_46[6]))
                        - (z2_tmp_3cd4f_47[6])),
                    ((z2_tmp_3cd4f_47[0])
                        + (((((((((x_sum_tmp_3cd4f_48[1]) * (y_sum_tmp_3cd4f_49[6]))
                            + ((x_sum_tmp_3cd4f_48[2]) * (y_sum_tmp_3cd4f_49[5])))
                            + ((x_sum_tmp_3cd4f_48[3]) * (y_sum_tmp_3cd4f_49[4])))
                            + ((x_sum_tmp_3cd4f_48[4]) * (y_sum_tmp_3cd4f_49[3])))
                            + ((x_sum_tmp_3cd4f_48[5]) * (y_sum_tmp_3cd4f_49[2])))
                            + ((x_sum_tmp_3cd4f_48[6]) * (y_sum_tmp_3cd4f_49[1])))
                            - (z0_tmp_3cd4f_46[7]))
                            - (z2_tmp_3cd4f_47[7]))),
                    ((z2_tmp_3cd4f_47[1])
                        + ((((((((x_sum_tmp_3cd4f_48[2]) * (y_sum_tmp_3cd4f_49[6]))
                            + ((x_sum_tmp_3cd4f_48[3]) * (y_sum_tmp_3cd4f_49[5])))
                            + ((x_sum_tmp_3cd4f_48[4]) * (y_sum_tmp_3cd4f_49[4])))
                            + ((x_sum_tmp_3cd4f_48[5]) * (y_sum_tmp_3cd4f_49[3])))
                            + ((x_sum_tmp_3cd4f_48[6]) * (y_sum_tmp_3cd4f_49[2])))
                            - (z0_tmp_3cd4f_46[8]))
                            - (z2_tmp_3cd4f_47[8]))),
                    ((z2_tmp_3cd4f_47[2])
                        + (((((((x_sum_tmp_3cd4f_48[3]) * (y_sum_tmp_3cd4f_49[6]))
                            + ((x_sum_tmp_3cd4f_48[4]) * (y_sum_tmp_3cd4f_49[5])))
                            + ((x_sum_tmp_3cd4f_48[5]) * (y_sum_tmp_3cd4f_49[4])))
                            + ((x_sum_tmp_3cd4f_48[6]) * (y_sum_tmp_3cd4f_49[3])))
                            - (z0_tmp_3cd4f_46[9]))
                            - (z2_tmp_3cd4f_47[9]))),
                    ((z2_tmp_3cd4f_47[3])
                        + ((((((x_sum_tmp_3cd4f_48[4]) * (y_sum_tmp_3cd4f_49[6]))
                            + ((x_sum_tmp_3cd4f_48[5]) * (y_sum_tmp_3cd4f_49[5])))
                            + ((x_sum_tmp_3cd4f_48[6]) * (y_sum_tmp_3cd4f_49[4])))
                            - (z0_tmp_3cd4f_46[10]))
                            - (z2_tmp_3cd4f_47[10]))),
                    ((z2_tmp_3cd4f_47[4])
                        + (((((x_sum_tmp_3cd4f_48[5]) * (y_sum_tmp_3cd4f_49[6]))
                            + ((x_sum_tmp_3cd4f_48[6]) * (y_sum_tmp_3cd4f_49[5])))
                            - (z0_tmp_3cd4f_46[11]))
                            - (z2_tmp_3cd4f_47[11]))),
                    ((z2_tmp_3cd4f_47[5])
                        + ((((x_sum_tmp_3cd4f_48[6]) * (y_sum_tmp_3cd4f_49[6]))
                            - (z0_tmp_3cd4f_46[12]))
                            - (z2_tmp_3cd4f_47[12]))),
                    z2_tmp_3cd4f_47[6],
                    z2_tmp_3cd4f_47[7],
                    z2_tmp_3cd4f_47[8],
                    z2_tmp_3cd4f_47[9],
                    z2_tmp_3cd4f_47[10],
                    z2_tmp_3cd4f_47[11],
                    z2_tmp_3cd4f_47[12],
                ];

                // Single Karatsuba N 7.

                let z0_tmp_3cd4f_51 = [
                    ((slope_limb_14_col142) * ((input_limb_30_col30) - (result_x_limb_14_col198))),
                    (((slope_limb_14_col142)
                        * ((input_limb_31_col31) - (result_x_limb_15_col199)))
                        + ((slope_limb_15_col143)
                            * ((input_limb_30_col30) - (result_x_limb_14_col198)))),
                    ((((slope_limb_14_col142)
                        * ((input_limb_32_col32) - (result_x_limb_16_col200)))
                        + ((slope_limb_15_col143)
                            * ((input_limb_31_col31) - (result_x_limb_15_col199))))
                        + ((slope_limb_16_col144)
                            * ((input_limb_30_col30) - (result_x_limb_14_col198)))),
                    (((((slope_limb_14_col142)
                        * ((input_limb_33_col33) - (result_x_limb_17_col201)))
                        + ((slope_limb_15_col143)
                            * ((input_limb_32_col32) - (result_x_limb_16_col200))))
                        + ((slope_limb_16_col144)
                            * ((input_limb_31_col31) - (result_x_limb_15_col199))))
                        + ((slope_limb_17_col145)
                            * ((input_limb_30_col30) - (result_x_limb_14_col198)))),
                    ((((((slope_limb_14_col142)
                        * ((input_limb_34_col34) - (result_x_limb_18_col202)))
                        + ((slope_limb_15_col143)
                            * ((input_limb_33_col33) - (result_x_limb_17_col201))))
                        + ((slope_limb_16_col144)
                            * ((input_limb_32_col32) - (result_x_limb_16_col200))))
                        + ((slope_limb_17_col145)
                            * ((input_limb_31_col31) - (result_x_limb_15_col199))))
                        + ((slope_limb_18_col146)
                            * ((input_limb_30_col30) - (result_x_limb_14_col198)))),
                    (((((((slope_limb_14_col142)
                        * ((input_limb_35_col35) - (result_x_limb_19_col203)))
                        + ((slope_limb_15_col143)
                            * ((input_limb_34_col34) - (result_x_limb_18_col202))))
                        + ((slope_limb_16_col144)
                            * ((input_limb_33_col33) - (result_x_limb_17_col201))))
                        + ((slope_limb_17_col145)
                            * ((input_limb_32_col32) - (result_x_limb_16_col200))))
                        + ((slope_limb_18_col146)
                            * ((input_limb_31_col31) - (result_x_limb_15_col199))))
                        + ((slope_limb_19_col147)
                            * ((input_limb_30_col30) - (result_x_limb_14_col198)))),
                    ((((((((slope_limb_14_col142)
                        * ((input_limb_36_col36) - (result_x_limb_20_col204)))
                        + ((slope_limb_15_col143)
                            * ((input_limb_35_col35) - (result_x_limb_19_col203))))
                        + ((slope_limb_16_col144)
                            * ((input_limb_34_col34) - (result_x_limb_18_col202))))
                        + ((slope_limb_17_col145)
                            * ((input_limb_33_col33) - (result_x_limb_17_col201))))
                        + ((slope_limb_18_col146)
                            * ((input_limb_32_col32) - (result_x_limb_16_col200))))
                        + ((slope_limb_19_col147)
                            * ((input_limb_31_col31) - (result_x_limb_15_col199))))
                        + ((slope_limb_20_col148)
                            * ((input_limb_30_col30) - (result_x_limb_14_col198)))),
                    (((((((slope_limb_15_col143)
                        * ((input_limb_36_col36) - (result_x_limb_20_col204)))
                        + ((slope_limb_16_col144)
                            * ((input_limb_35_col35) - (result_x_limb_19_col203))))
                        + ((slope_limb_17_col145)
                            * ((input_limb_34_col34) - (result_x_limb_18_col202))))
                        + ((slope_limb_18_col146)
                            * ((input_limb_33_col33) - (result_x_limb_17_col201))))
                        + ((slope_limb_19_col147)
                            * ((input_limb_32_col32) - (result_x_limb_16_col200))))
                        + ((slope_limb_20_col148)
                            * ((input_limb_31_col31) - (result_x_limb_15_col199)))),
                    ((((((slope_limb_16_col144)
                        * ((input_limb_36_col36) - (result_x_limb_20_col204)))
                        + ((slope_limb_17_col145)
                            * ((input_limb_35_col35) - (result_x_limb_19_col203))))
                        + ((slope_limb_18_col146)
                            * ((input_limb_34_col34) - (result_x_limb_18_col202))))
                        + ((slope_limb_19_col147)
                            * ((input_limb_33_col33) - (result_x_limb_17_col201))))
                        + ((slope_limb_20_col148)
                            * ((input_limb_32_col32) - (result_x_limb_16_col200)))),
                    (((((slope_limb_17_col145)
                        * ((input_limb_36_col36) - (result_x_limb_20_col204)))
                        + ((slope_limb_18_col146)
                            * ((input_limb_35_col35) - (result_x_limb_19_col203))))
                        + ((slope_limb_19_col147)
                            * ((input_limb_34_col34) - (result_x_limb_18_col202))))
                        + ((slope_limb_20_col148)
                            * ((input_limb_33_col33) - (result_x_limb_17_col201)))),
                    ((((slope_limb_18_col146)
                        * ((input_limb_36_col36) - (result_x_limb_20_col204)))
                        + ((slope_limb_19_col147)
                            * ((input_limb_35_col35) - (result_x_limb_19_col203))))
                        + ((slope_limb_20_col148)
                            * ((input_limb_34_col34) - (result_x_limb_18_col202)))),
                    (((slope_limb_19_col147)
                        * ((input_limb_36_col36) - (result_x_limb_20_col204)))
                        + ((slope_limb_20_col148)
                            * ((input_limb_35_col35) - (result_x_limb_19_col203)))),
                    ((slope_limb_20_col148) * ((input_limb_36_col36) - (result_x_limb_20_col204))),
                ];
                let z2_tmp_3cd4f_52 = [
                    ((slope_limb_21_col149) * ((input_limb_37_col37) - (result_x_limb_21_col205))),
                    (((slope_limb_21_col149)
                        * ((input_limb_38_col38) - (result_x_limb_22_col206)))
                        + ((slope_limb_22_col150)
                            * ((input_limb_37_col37) - (result_x_limb_21_col205)))),
                    ((((slope_limb_21_col149)
                        * ((input_limb_39_col39) - (result_x_limb_23_col207)))
                        + ((slope_limb_22_col150)
                            * ((input_limb_38_col38) - (result_x_limb_22_col206))))
                        + ((slope_limb_23_col151)
                            * ((input_limb_37_col37) - (result_x_limb_21_col205)))),
                    (((((slope_limb_21_col149)
                        * ((input_limb_40_col40) - (result_x_limb_24_col208)))
                        + ((slope_limb_22_col150)
                            * ((input_limb_39_col39) - (result_x_limb_23_col207))))
                        + ((slope_limb_23_col151)
                            * ((input_limb_38_col38) - (result_x_limb_22_col206))))
                        + ((slope_limb_24_col152)
                            * ((input_limb_37_col37) - (result_x_limb_21_col205)))),
                    ((((((slope_limb_21_col149)
                        * ((input_limb_41_col41) - (result_x_limb_25_col209)))
                        + ((slope_limb_22_col150)
                            * ((input_limb_40_col40) - (result_x_limb_24_col208))))
                        + ((slope_limb_23_col151)
                            * ((input_limb_39_col39) - (result_x_limb_23_col207))))
                        + ((slope_limb_24_col152)
                            * ((input_limb_38_col38) - (result_x_limb_22_col206))))
                        + ((slope_limb_25_col153)
                            * ((input_limb_37_col37) - (result_x_limb_21_col205)))),
                    (((((((slope_limb_21_col149)
                        * ((input_limb_42_col42) - (result_x_limb_26_col210)))
                        + ((slope_limb_22_col150)
                            * ((input_limb_41_col41) - (result_x_limb_25_col209))))
                        + ((slope_limb_23_col151)
                            * ((input_limb_40_col40) - (result_x_limb_24_col208))))
                        + ((slope_limb_24_col152)
                            * ((input_limb_39_col39) - (result_x_limb_23_col207))))
                        + ((slope_limb_25_col153)
                            * ((input_limb_38_col38) - (result_x_limb_22_col206))))
                        + ((slope_limb_26_col154)
                            * ((input_limb_37_col37) - (result_x_limb_21_col205)))),
                    ((((((((slope_limb_21_col149)
                        * ((input_limb_43_col43) - (result_x_limb_27_col211)))
                        + ((slope_limb_22_col150)
                            * ((input_limb_42_col42) - (result_x_limb_26_col210))))
                        + ((slope_limb_23_col151)
                            * ((input_limb_41_col41) - (result_x_limb_25_col209))))
                        + ((slope_limb_24_col152)
                            * ((input_limb_40_col40) - (result_x_limb_24_col208))))
                        + ((slope_limb_25_col153)
                            * ((input_limb_39_col39) - (result_x_limb_23_col207))))
                        + ((slope_limb_26_col154)
                            * ((input_limb_38_col38) - (result_x_limb_22_col206))))
                        + ((slope_limb_27_col155)
                            * ((input_limb_37_col37) - (result_x_limb_21_col205)))),
                    (((((((slope_limb_22_col150)
                        * ((input_limb_43_col43) - (result_x_limb_27_col211)))
                        + ((slope_limb_23_col151)
                            * ((input_limb_42_col42) - (result_x_limb_26_col210))))
                        + ((slope_limb_24_col152)
                            * ((input_limb_41_col41) - (result_x_limb_25_col209))))
                        + ((slope_limb_25_col153)
                            * ((input_limb_40_col40) - (result_x_limb_24_col208))))
                        + ((slope_limb_26_col154)
                            * ((input_limb_39_col39) - (result_x_limb_23_col207))))
                        + ((slope_limb_27_col155)
                            * ((input_limb_38_col38) - (result_x_limb_22_col206)))),
                    ((((((slope_limb_23_col151)
                        * ((input_limb_43_col43) - (result_x_limb_27_col211)))
                        + ((slope_limb_24_col152)
                            * ((input_limb_42_col42) - (result_x_limb_26_col210))))
                        + ((slope_limb_25_col153)
                            * ((input_limb_41_col41) - (result_x_limb_25_col209))))
                        + ((slope_limb_26_col154)
                            * ((input_limb_40_col40) - (result_x_limb_24_col208))))
                        + ((slope_limb_27_col155)
                            * ((input_limb_39_col39) - (result_x_limb_23_col207)))),
                    (((((slope_limb_24_col152)
                        * ((input_limb_43_col43) - (result_x_limb_27_col211)))
                        + ((slope_limb_25_col153)
                            * ((input_limb_42_col42) - (result_x_limb_26_col210))))
                        + ((slope_limb_26_col154)
                            * ((input_limb_41_col41) - (result_x_limb_25_col209))))
                        + ((slope_limb_27_col155)
                            * ((input_limb_40_col40) - (result_x_limb_24_col208)))),
                    ((((slope_limb_25_col153)
                        * ((input_limb_43_col43) - (result_x_limb_27_col211)))
                        + ((slope_limb_26_col154)
                            * ((input_limb_42_col42) - (result_x_limb_26_col210))))
                        + ((slope_limb_27_col155)
                            * ((input_limb_41_col41) - (result_x_limb_25_col209)))),
                    (((slope_limb_26_col154)
                        * ((input_limb_43_col43) - (result_x_limb_27_col211)))
                        + ((slope_limb_27_col155)
                            * ((input_limb_42_col42) - (result_x_limb_26_col210)))),
                    ((slope_limb_27_col155) * ((input_limb_43_col43) - (result_x_limb_27_col211))),
                ];
                let x_sum_tmp_3cd4f_53 = [
                    ((slope_limb_14_col142) + (slope_limb_21_col149)),
                    ((slope_limb_15_col143) + (slope_limb_22_col150)),
                    ((slope_limb_16_col144) + (slope_limb_23_col151)),
                    ((slope_limb_17_col145) + (slope_limb_24_col152)),
                    ((slope_limb_18_col146) + (slope_limb_25_col153)),
                    ((slope_limb_19_col147) + (slope_limb_26_col154)),
                    ((slope_limb_20_col148) + (slope_limb_27_col155)),
                ];
                let y_sum_tmp_3cd4f_54 = [
                    (((input_limb_30_col30) - (result_x_limb_14_col198))
                        + ((input_limb_37_col37) - (result_x_limb_21_col205))),
                    (((input_limb_31_col31) - (result_x_limb_15_col199))
                        + ((input_limb_38_col38) - (result_x_limb_22_col206))),
                    (((input_limb_32_col32) - (result_x_limb_16_col200))
                        + ((input_limb_39_col39) - (result_x_limb_23_col207))),
                    (((input_limb_33_col33) - (result_x_limb_17_col201))
                        + ((input_limb_40_col40) - (result_x_limb_24_col208))),
                    (((input_limb_34_col34) - (result_x_limb_18_col202))
                        + ((input_limb_41_col41) - (result_x_limb_25_col209))),
                    (((input_limb_35_col35) - (result_x_limb_19_col203))
                        + ((input_limb_42_col42) - (result_x_limb_26_col210))),
                    (((input_limb_36_col36) - (result_x_limb_20_col204))
                        + ((input_limb_43_col43) - (result_x_limb_27_col211))),
                ];
                let single_karatsuba_n_7_output_tmp_3cd4f_55 = [
                    z0_tmp_3cd4f_51[0],
                    z0_tmp_3cd4f_51[1],
                    z0_tmp_3cd4f_51[2],
                    z0_tmp_3cd4f_51[3],
                    z0_tmp_3cd4f_51[4],
                    z0_tmp_3cd4f_51[5],
                    z0_tmp_3cd4f_51[6],
                    ((z0_tmp_3cd4f_51[7])
                        + ((((x_sum_tmp_3cd4f_53[0]) * (y_sum_tmp_3cd4f_54[0]))
                            - (z0_tmp_3cd4f_51[0]))
                            - (z2_tmp_3cd4f_52[0]))),
                    ((z0_tmp_3cd4f_51[8])
                        + (((((x_sum_tmp_3cd4f_53[0]) * (y_sum_tmp_3cd4f_54[1]))
                            + ((x_sum_tmp_3cd4f_53[1]) * (y_sum_tmp_3cd4f_54[0])))
                            - (z0_tmp_3cd4f_51[1]))
                            - (z2_tmp_3cd4f_52[1]))),
                    ((z0_tmp_3cd4f_51[9])
                        + ((((((x_sum_tmp_3cd4f_53[0]) * (y_sum_tmp_3cd4f_54[2]))
                            + ((x_sum_tmp_3cd4f_53[1]) * (y_sum_tmp_3cd4f_54[1])))
                            + ((x_sum_tmp_3cd4f_53[2]) * (y_sum_tmp_3cd4f_54[0])))
                            - (z0_tmp_3cd4f_51[2]))
                            - (z2_tmp_3cd4f_52[2]))),
                    ((z0_tmp_3cd4f_51[10])
                        + (((((((x_sum_tmp_3cd4f_53[0]) * (y_sum_tmp_3cd4f_54[3]))
                            + ((x_sum_tmp_3cd4f_53[1]) * (y_sum_tmp_3cd4f_54[2])))
                            + ((x_sum_tmp_3cd4f_53[2]) * (y_sum_tmp_3cd4f_54[1])))
                            + ((x_sum_tmp_3cd4f_53[3]) * (y_sum_tmp_3cd4f_54[0])))
                            - (z0_tmp_3cd4f_51[3]))
                            - (z2_tmp_3cd4f_52[3]))),
                    ((z0_tmp_3cd4f_51[11])
                        + ((((((((x_sum_tmp_3cd4f_53[0]) * (y_sum_tmp_3cd4f_54[4]))
                            + ((x_sum_tmp_3cd4f_53[1]) * (y_sum_tmp_3cd4f_54[3])))
                            + ((x_sum_tmp_3cd4f_53[2]) * (y_sum_tmp_3cd4f_54[2])))
                            + ((x_sum_tmp_3cd4f_53[3]) * (y_sum_tmp_3cd4f_54[1])))
                            + ((x_sum_tmp_3cd4f_53[4]) * (y_sum_tmp_3cd4f_54[0])))
                            - (z0_tmp_3cd4f_51[4]))
                            - (z2_tmp_3cd4f_52[4]))),
                    ((z0_tmp_3cd4f_51[12])
                        + (((((((((x_sum_tmp_3cd4f_53[0]) * (y_sum_tmp_3cd4f_54[5]))
                            + ((x_sum_tmp_3cd4f_53[1]) * (y_sum_tmp_3cd4f_54[4])))
                            + ((x_sum_tmp_3cd4f_53[2]) * (y_sum_tmp_3cd4f_54[3])))
                            + ((x_sum_tmp_3cd4f_53[3]) * (y_sum_tmp_3cd4f_54[2])))
                            + ((x_sum_tmp_3cd4f_53[4]) * (y_sum_tmp_3cd4f_54[1])))
                            + ((x_sum_tmp_3cd4f_53[5]) * (y_sum_tmp_3cd4f_54[0])))
                            - (z0_tmp_3cd4f_51[5]))
                            - (z2_tmp_3cd4f_52[5]))),
                    ((((((((((x_sum_tmp_3cd4f_53[0]) * (y_sum_tmp_3cd4f_54[6]))
                        + ((x_sum_tmp_3cd4f_53[1]) * (y_sum_tmp_3cd4f_54[5])))
                        + ((x_sum_tmp_3cd4f_53[2]) * (y_sum_tmp_3cd4f_54[4])))
                        + ((x_sum_tmp_3cd4f_53[3]) * (y_sum_tmp_3cd4f_54[3])))
                        + ((x_sum_tmp_3cd4f_53[4]) * (y_sum_tmp_3cd4f_54[2])))
                        + ((x_sum_tmp_3cd4f_53[5]) * (y_sum_tmp_3cd4f_54[1])))
                        + ((x_sum_tmp_3cd4f_53[6]) * (y_sum_tmp_3cd4f_54[0])))
                        - (z0_tmp_3cd4f_51[6]))
                        - (z2_tmp_3cd4f_52[6])),
                    ((z2_tmp_3cd4f_52[0])
                        + (((((((((x_sum_tmp_3cd4f_53[1]) * (y_sum_tmp_3cd4f_54[6]))
                            + ((x_sum_tmp_3cd4f_53[2]) * (y_sum_tmp_3cd4f_54[5])))
                            + ((x_sum_tmp_3cd4f_53[3]) * (y_sum_tmp_3cd4f_54[4])))
                            + ((x_sum_tmp_3cd4f_53[4]) * (y_sum_tmp_3cd4f_54[3])))
                            + ((x_sum_tmp_3cd4f_53[5]) * (y_sum_tmp_3cd4f_54[2])))
                            + ((x_sum_tmp_3cd4f_53[6]) * (y_sum_tmp_3cd4f_54[1])))
                            - (z0_tmp_3cd4f_51[7]))
                            - (z2_tmp_3cd4f_52[7]))),
                    ((z2_tmp_3cd4f_52[1])
                        + ((((((((x_sum_tmp_3cd4f_53[2]) * (y_sum_tmp_3cd4f_54[6]))
                            + ((x_sum_tmp_3cd4f_53[3]) * (y_sum_tmp_3cd4f_54[5])))
                            + ((x_sum_tmp_3cd4f_53[4]) * (y_sum_tmp_3cd4f_54[4])))
                            + ((x_sum_tmp_3cd4f_53[5]) * (y_sum_tmp_3cd4f_54[3])))
                            + ((x_sum_tmp_3cd4f_53[6]) * (y_sum_tmp_3cd4f_54[2])))
                            - (z0_tmp_3cd4f_51[8]))
                            - (z2_tmp_3cd4f_52[8]))),
                    ((z2_tmp_3cd4f_52[2])
                        + (((((((x_sum_tmp_3cd4f_53[3]) * (y_sum_tmp_3cd4f_54[6]))
                            + ((x_sum_tmp_3cd4f_53[4]) * (y_sum_tmp_3cd4f_54[5])))
                            + ((x_sum_tmp_3cd4f_53[5]) * (y_sum_tmp_3cd4f_54[4])))
                            + ((x_sum_tmp_3cd4f_53[6]) * (y_sum_tmp_3cd4f_54[3])))
                            - (z0_tmp_3cd4f_51[9]))
                            - (z2_tmp_3cd4f_52[9]))),
                    ((z2_tmp_3cd4f_52[3])
                        + ((((((x_sum_tmp_3cd4f_53[4]) * (y_sum_tmp_3cd4f_54[6]))
                            + ((x_sum_tmp_3cd4f_53[5]) * (y_sum_tmp_3cd4f_54[5])))
                            + ((x_sum_tmp_3cd4f_53[6]) * (y_sum_tmp_3cd4f_54[4])))
                            - (z0_tmp_3cd4f_51[10]))
                            - (z2_tmp_3cd4f_52[10]))),
                    ((z2_tmp_3cd4f_52[4])
                        + (((((x_sum_tmp_3cd4f_53[5]) * (y_sum_tmp_3cd4f_54[6]))
                            + ((x_sum_tmp_3cd4f_53[6]) * (y_sum_tmp_3cd4f_54[5])))
                            - (z0_tmp_3cd4f_51[11]))
                            - (z2_tmp_3cd4f_52[11]))),
                    ((z2_tmp_3cd4f_52[5])
                        + ((((x_sum_tmp_3cd4f_53[6]) * (y_sum_tmp_3cd4f_54[6]))
                            - (z0_tmp_3cd4f_51[12]))
                            - (z2_tmp_3cd4f_52[12]))),
                    z2_tmp_3cd4f_52[6],
                    z2_tmp_3cd4f_52[7],
                    z2_tmp_3cd4f_52[8],
                    z2_tmp_3cd4f_52[9],
                    z2_tmp_3cd4f_52[10],
                    z2_tmp_3cd4f_52[11],
                    z2_tmp_3cd4f_52[12],
                ];

                let x_sum_tmp_3cd4f_56 = [
                    ((slope_limb_0_col128) + (slope_limb_14_col142)),
                    ((slope_limb_1_col129) + (slope_limb_15_col143)),
                    ((slope_limb_2_col130) + (slope_limb_16_col144)),
                    ((slope_limb_3_col131) + (slope_limb_17_col145)),
                    ((slope_limb_4_col132) + (slope_limb_18_col146)),
                    ((slope_limb_5_col133) + (slope_limb_19_col147)),
                    ((slope_limb_6_col134) + (slope_limb_20_col148)),
                    ((slope_limb_7_col135) + (slope_limb_21_col149)),
                    ((slope_limb_8_col136) + (slope_limb_22_col150)),
                    ((slope_limb_9_col137) + (slope_limb_23_col151)),
                    ((slope_limb_10_col138) + (slope_limb_24_col152)),
                    ((slope_limb_11_col139) + (slope_limb_25_col153)),
                    ((slope_limb_12_col140) + (slope_limb_26_col154)),
                    ((slope_limb_13_col141) + (slope_limb_27_col155)),
                ];
                let y_sum_tmp_3cd4f_57 = [
                    (((input_limb_16_col16) - (result_x_limb_0_col184))
                        + ((input_limb_30_col30) - (result_x_limb_14_col198))),
                    (((input_limb_17_col17) - (result_x_limb_1_col185))
                        + ((input_limb_31_col31) - (result_x_limb_15_col199))),
                    (((input_limb_18_col18) - (result_x_limb_2_col186))
                        + ((input_limb_32_col32) - (result_x_limb_16_col200))),
                    (((input_limb_19_col19) - (result_x_limb_3_col187))
                        + ((input_limb_33_col33) - (result_x_limb_17_col201))),
                    (((input_limb_20_col20) - (result_x_limb_4_col188))
                        + ((input_limb_34_col34) - (result_x_limb_18_col202))),
                    (((input_limb_21_col21) - (result_x_limb_5_col189))
                        + ((input_limb_35_col35) - (result_x_limb_19_col203))),
                    (((input_limb_22_col22) - (result_x_limb_6_col190))
                        + ((input_limb_36_col36) - (result_x_limb_20_col204))),
                    (((input_limb_23_col23) - (result_x_limb_7_col191))
                        + ((input_limb_37_col37) - (result_x_limb_21_col205))),
                    (((input_limb_24_col24) - (result_x_limb_8_col192))
                        + ((input_limb_38_col38) - (result_x_limb_22_col206))),
                    (((input_limb_25_col25) - (result_x_limb_9_col193))
                        + ((input_limb_39_col39) - (result_x_limb_23_col207))),
                    (((input_limb_26_col26) - (result_x_limb_10_col194))
                        + ((input_limb_40_col40) - (result_x_limb_24_col208))),
                    (((input_limb_27_col27) - (result_x_limb_11_col195))
                        + ((input_limb_41_col41) - (result_x_limb_25_col209))),
                    (((input_limb_28_col28) - (result_x_limb_12_col196))
                        + ((input_limb_42_col42) - (result_x_limb_26_col210))),
                    (((input_limb_29_col29) - (result_x_limb_13_col197))
                        + ((input_limb_43_col43) - (result_x_limb_27_col211))),
                ];

                // Single Karatsuba N 7.

                let z0_tmp_3cd4f_58 = [
                    ((x_sum_tmp_3cd4f_56[0]) * (y_sum_tmp_3cd4f_57[0])),
                    (((x_sum_tmp_3cd4f_56[0]) * (y_sum_tmp_3cd4f_57[1]))
                        + ((x_sum_tmp_3cd4f_56[1]) * (y_sum_tmp_3cd4f_57[0]))),
                    ((((x_sum_tmp_3cd4f_56[0]) * (y_sum_tmp_3cd4f_57[2]))
                        + ((x_sum_tmp_3cd4f_56[1]) * (y_sum_tmp_3cd4f_57[1])))
                        + ((x_sum_tmp_3cd4f_56[2]) * (y_sum_tmp_3cd4f_57[0]))),
                    (((((x_sum_tmp_3cd4f_56[0]) * (y_sum_tmp_3cd4f_57[3]))
                        + ((x_sum_tmp_3cd4f_56[1]) * (y_sum_tmp_3cd4f_57[2])))
                        + ((x_sum_tmp_3cd4f_56[2]) * (y_sum_tmp_3cd4f_57[1])))
                        + ((x_sum_tmp_3cd4f_56[3]) * (y_sum_tmp_3cd4f_57[0]))),
                    ((((((x_sum_tmp_3cd4f_56[0]) * (y_sum_tmp_3cd4f_57[4]))
                        + ((x_sum_tmp_3cd4f_56[1]) * (y_sum_tmp_3cd4f_57[3])))
                        + ((x_sum_tmp_3cd4f_56[2]) * (y_sum_tmp_3cd4f_57[2])))
                        + ((x_sum_tmp_3cd4f_56[3]) * (y_sum_tmp_3cd4f_57[1])))
                        + ((x_sum_tmp_3cd4f_56[4]) * (y_sum_tmp_3cd4f_57[0]))),
                    (((((((x_sum_tmp_3cd4f_56[0]) * (y_sum_tmp_3cd4f_57[5]))
                        + ((x_sum_tmp_3cd4f_56[1]) * (y_sum_tmp_3cd4f_57[4])))
                        + ((x_sum_tmp_3cd4f_56[2]) * (y_sum_tmp_3cd4f_57[3])))
                        + ((x_sum_tmp_3cd4f_56[3]) * (y_sum_tmp_3cd4f_57[2])))
                        + ((x_sum_tmp_3cd4f_56[4]) * (y_sum_tmp_3cd4f_57[1])))
                        + ((x_sum_tmp_3cd4f_56[5]) * (y_sum_tmp_3cd4f_57[0]))),
                    ((((((((x_sum_tmp_3cd4f_56[0]) * (y_sum_tmp_3cd4f_57[6]))
                        + ((x_sum_tmp_3cd4f_56[1]) * (y_sum_tmp_3cd4f_57[5])))
                        + ((x_sum_tmp_3cd4f_56[2]) * (y_sum_tmp_3cd4f_57[4])))
                        + ((x_sum_tmp_3cd4f_56[3]) * (y_sum_tmp_3cd4f_57[3])))
                        + ((x_sum_tmp_3cd4f_56[4]) * (y_sum_tmp_3cd4f_57[2])))
                        + ((x_sum_tmp_3cd4f_56[5]) * (y_sum_tmp_3cd4f_57[1])))
                        + ((x_sum_tmp_3cd4f_56[6]) * (y_sum_tmp_3cd4f_57[0]))),
                    (((((((x_sum_tmp_3cd4f_56[1]) * (y_sum_tmp_3cd4f_57[6]))
                        + ((x_sum_tmp_3cd4f_56[2]) * (y_sum_tmp_3cd4f_57[5])))
                        + ((x_sum_tmp_3cd4f_56[3]) * (y_sum_tmp_3cd4f_57[4])))
                        + ((x_sum_tmp_3cd4f_56[4]) * (y_sum_tmp_3cd4f_57[3])))
                        + ((x_sum_tmp_3cd4f_56[5]) * (y_sum_tmp_3cd4f_57[2])))
                        + ((x_sum_tmp_3cd4f_56[6]) * (y_sum_tmp_3cd4f_57[1]))),
                    ((((((x_sum_tmp_3cd4f_56[2]) * (y_sum_tmp_3cd4f_57[6]))
                        + ((x_sum_tmp_3cd4f_56[3]) * (y_sum_tmp_3cd4f_57[5])))
                        + ((x_sum_tmp_3cd4f_56[4]) * (y_sum_tmp_3cd4f_57[4])))
                        + ((x_sum_tmp_3cd4f_56[5]) * (y_sum_tmp_3cd4f_57[3])))
                        + ((x_sum_tmp_3cd4f_56[6]) * (y_sum_tmp_3cd4f_57[2]))),
                    (((((x_sum_tmp_3cd4f_56[3]) * (y_sum_tmp_3cd4f_57[6]))
                        + ((x_sum_tmp_3cd4f_56[4]) * (y_sum_tmp_3cd4f_57[5])))
                        + ((x_sum_tmp_3cd4f_56[5]) * (y_sum_tmp_3cd4f_57[4])))
                        + ((x_sum_tmp_3cd4f_56[6]) * (y_sum_tmp_3cd4f_57[3]))),
                    ((((x_sum_tmp_3cd4f_56[4]) * (y_sum_tmp_3cd4f_57[6]))
                        + ((x_sum_tmp_3cd4f_56[5]) * (y_sum_tmp_3cd4f_57[5])))
                        + ((x_sum_tmp_3cd4f_56[6]) * (y_sum_tmp_3cd4f_57[4]))),
                    (((x_sum_tmp_3cd4f_56[5]) * (y_sum_tmp_3cd4f_57[6]))
                        + ((x_sum_tmp_3cd4f_56[6]) * (y_sum_tmp_3cd4f_57[5]))),
                    ((x_sum_tmp_3cd4f_56[6]) * (y_sum_tmp_3cd4f_57[6])),
                ];
                let z2_tmp_3cd4f_59 = [
                    ((x_sum_tmp_3cd4f_56[7]) * (y_sum_tmp_3cd4f_57[7])),
                    (((x_sum_tmp_3cd4f_56[7]) * (y_sum_tmp_3cd4f_57[8]))
                        + ((x_sum_tmp_3cd4f_56[8]) * (y_sum_tmp_3cd4f_57[7]))),
                    ((((x_sum_tmp_3cd4f_56[7]) * (y_sum_tmp_3cd4f_57[9]))
                        + ((x_sum_tmp_3cd4f_56[8]) * (y_sum_tmp_3cd4f_57[8])))
                        + ((x_sum_tmp_3cd4f_56[9]) * (y_sum_tmp_3cd4f_57[7]))),
                    (((((x_sum_tmp_3cd4f_56[7]) * (y_sum_tmp_3cd4f_57[10]))
                        + ((x_sum_tmp_3cd4f_56[8]) * (y_sum_tmp_3cd4f_57[9])))
                        + ((x_sum_tmp_3cd4f_56[9]) * (y_sum_tmp_3cd4f_57[8])))
                        + ((x_sum_tmp_3cd4f_56[10]) * (y_sum_tmp_3cd4f_57[7]))),
                    ((((((x_sum_tmp_3cd4f_56[7]) * (y_sum_tmp_3cd4f_57[11]))
                        + ((x_sum_tmp_3cd4f_56[8]) * (y_sum_tmp_3cd4f_57[10])))
                        + ((x_sum_tmp_3cd4f_56[9]) * (y_sum_tmp_3cd4f_57[9])))
                        + ((x_sum_tmp_3cd4f_56[10]) * (y_sum_tmp_3cd4f_57[8])))
                        + ((x_sum_tmp_3cd4f_56[11]) * (y_sum_tmp_3cd4f_57[7]))),
                    (((((((x_sum_tmp_3cd4f_56[7]) * (y_sum_tmp_3cd4f_57[12]))
                        + ((x_sum_tmp_3cd4f_56[8]) * (y_sum_tmp_3cd4f_57[11])))
                        + ((x_sum_tmp_3cd4f_56[9]) * (y_sum_tmp_3cd4f_57[10])))
                        + ((x_sum_tmp_3cd4f_56[10]) * (y_sum_tmp_3cd4f_57[9])))
                        + ((x_sum_tmp_3cd4f_56[11]) * (y_sum_tmp_3cd4f_57[8])))
                        + ((x_sum_tmp_3cd4f_56[12]) * (y_sum_tmp_3cd4f_57[7]))),
                    ((((((((x_sum_tmp_3cd4f_56[7]) * (y_sum_tmp_3cd4f_57[13]))
                        + ((x_sum_tmp_3cd4f_56[8]) * (y_sum_tmp_3cd4f_57[12])))
                        + ((x_sum_tmp_3cd4f_56[9]) * (y_sum_tmp_3cd4f_57[11])))
                        + ((x_sum_tmp_3cd4f_56[10]) * (y_sum_tmp_3cd4f_57[10])))
                        + ((x_sum_tmp_3cd4f_56[11]) * (y_sum_tmp_3cd4f_57[9])))
                        + ((x_sum_tmp_3cd4f_56[12]) * (y_sum_tmp_3cd4f_57[8])))
                        + ((x_sum_tmp_3cd4f_56[13]) * (y_sum_tmp_3cd4f_57[7]))),
                    (((((((x_sum_tmp_3cd4f_56[8]) * (y_sum_tmp_3cd4f_57[13]))
                        + ((x_sum_tmp_3cd4f_56[9]) * (y_sum_tmp_3cd4f_57[12])))
                        + ((x_sum_tmp_3cd4f_56[10]) * (y_sum_tmp_3cd4f_57[11])))
                        + ((x_sum_tmp_3cd4f_56[11]) * (y_sum_tmp_3cd4f_57[10])))
                        + ((x_sum_tmp_3cd4f_56[12]) * (y_sum_tmp_3cd4f_57[9])))
                        + ((x_sum_tmp_3cd4f_56[13]) * (y_sum_tmp_3cd4f_57[8]))),
                    ((((((x_sum_tmp_3cd4f_56[9]) * (y_sum_tmp_3cd4f_57[13]))
                        + ((x_sum_tmp_3cd4f_56[10]) * (y_sum_tmp_3cd4f_57[12])))
                        + ((x_sum_tmp_3cd4f_56[11]) * (y_sum_tmp_3cd4f_57[11])))
                        + ((x_sum_tmp_3cd4f_56[12]) * (y_sum_tmp_3cd4f_57[10])))
                        + ((x_sum_tmp_3cd4f_56[13]) * (y_sum_tmp_3cd4f_57[9]))),
                    (((((x_sum_tmp_3cd4f_56[10]) * (y_sum_tmp_3cd4f_57[13]))
                        + ((x_sum_tmp_3cd4f_56[11]) * (y_sum_tmp_3cd4f_57[12])))
                        + ((x_sum_tmp_3cd4f_56[12]) * (y_sum_tmp_3cd4f_57[11])))
                        + ((x_sum_tmp_3cd4f_56[13]) * (y_sum_tmp_3cd4f_57[10]))),
                    ((((x_sum_tmp_3cd4f_56[11]) * (y_sum_tmp_3cd4f_57[13]))
                        + ((x_sum_tmp_3cd4f_56[12]) * (y_sum_tmp_3cd4f_57[12])))
                        + ((x_sum_tmp_3cd4f_56[13]) * (y_sum_tmp_3cd4f_57[11]))),
                    (((x_sum_tmp_3cd4f_56[12]) * (y_sum_tmp_3cd4f_57[13]))
                        + ((x_sum_tmp_3cd4f_56[13]) * (y_sum_tmp_3cd4f_57[12]))),
                    ((x_sum_tmp_3cd4f_56[13]) * (y_sum_tmp_3cd4f_57[13])),
                ];
                let x_sum_tmp_3cd4f_60 = [
                    ((x_sum_tmp_3cd4f_56[0]) + (x_sum_tmp_3cd4f_56[7])),
                    ((x_sum_tmp_3cd4f_56[1]) + (x_sum_tmp_3cd4f_56[8])),
                    ((x_sum_tmp_3cd4f_56[2]) + (x_sum_tmp_3cd4f_56[9])),
                    ((x_sum_tmp_3cd4f_56[3]) + (x_sum_tmp_3cd4f_56[10])),
                    ((x_sum_tmp_3cd4f_56[4]) + (x_sum_tmp_3cd4f_56[11])),
                    ((x_sum_tmp_3cd4f_56[5]) + (x_sum_tmp_3cd4f_56[12])),
                    ((x_sum_tmp_3cd4f_56[6]) + (x_sum_tmp_3cd4f_56[13])),
                ];
                let y_sum_tmp_3cd4f_61 = [
                    ((y_sum_tmp_3cd4f_57[0]) + (y_sum_tmp_3cd4f_57[7])),
                    ((y_sum_tmp_3cd4f_57[1]) + (y_sum_tmp_3cd4f_57[8])),
                    ((y_sum_tmp_3cd4f_57[2]) + (y_sum_tmp_3cd4f_57[9])),
                    ((y_sum_tmp_3cd4f_57[3]) + (y_sum_tmp_3cd4f_57[10])),
                    ((y_sum_tmp_3cd4f_57[4]) + (y_sum_tmp_3cd4f_57[11])),
                    ((y_sum_tmp_3cd4f_57[5]) + (y_sum_tmp_3cd4f_57[12])),
                    ((y_sum_tmp_3cd4f_57[6]) + (y_sum_tmp_3cd4f_57[13])),
                ];
                let single_karatsuba_n_7_output_tmp_3cd4f_62 = [
                    z0_tmp_3cd4f_58[0],
                    z0_tmp_3cd4f_58[1],
                    z0_tmp_3cd4f_58[2],
                    z0_tmp_3cd4f_58[3],
                    z0_tmp_3cd4f_58[4],
                    z0_tmp_3cd4f_58[5],
                    z0_tmp_3cd4f_58[6],
                    ((z0_tmp_3cd4f_58[7])
                        + ((((x_sum_tmp_3cd4f_60[0]) * (y_sum_tmp_3cd4f_61[0]))
                            - (z0_tmp_3cd4f_58[0]))
                            - (z2_tmp_3cd4f_59[0]))),
                    ((z0_tmp_3cd4f_58[8])
                        + (((((x_sum_tmp_3cd4f_60[0]) * (y_sum_tmp_3cd4f_61[1]))
                            + ((x_sum_tmp_3cd4f_60[1]) * (y_sum_tmp_3cd4f_61[0])))
                            - (z0_tmp_3cd4f_58[1]))
                            - (z2_tmp_3cd4f_59[1]))),
                    ((z0_tmp_3cd4f_58[9])
                        + ((((((x_sum_tmp_3cd4f_60[0]) * (y_sum_tmp_3cd4f_61[2]))
                            + ((x_sum_tmp_3cd4f_60[1]) * (y_sum_tmp_3cd4f_61[1])))
                            + ((x_sum_tmp_3cd4f_60[2]) * (y_sum_tmp_3cd4f_61[0])))
                            - (z0_tmp_3cd4f_58[2]))
                            - (z2_tmp_3cd4f_59[2]))),
                    ((z0_tmp_3cd4f_58[10])
                        + (((((((x_sum_tmp_3cd4f_60[0]) * (y_sum_tmp_3cd4f_61[3]))
                            + ((x_sum_tmp_3cd4f_60[1]) * (y_sum_tmp_3cd4f_61[2])))
                            + ((x_sum_tmp_3cd4f_60[2]) * (y_sum_tmp_3cd4f_61[1])))
                            + ((x_sum_tmp_3cd4f_60[3]) * (y_sum_tmp_3cd4f_61[0])))
                            - (z0_tmp_3cd4f_58[3]))
                            - (z2_tmp_3cd4f_59[3]))),
                    ((z0_tmp_3cd4f_58[11])
                        + ((((((((x_sum_tmp_3cd4f_60[0]) * (y_sum_tmp_3cd4f_61[4]))
                            + ((x_sum_tmp_3cd4f_60[1]) * (y_sum_tmp_3cd4f_61[3])))
                            + ((x_sum_tmp_3cd4f_60[2]) * (y_sum_tmp_3cd4f_61[2])))
                            + ((x_sum_tmp_3cd4f_60[3]) * (y_sum_tmp_3cd4f_61[1])))
                            + ((x_sum_tmp_3cd4f_60[4]) * (y_sum_tmp_3cd4f_61[0])))
                            - (z0_tmp_3cd4f_58[4]))
                            - (z2_tmp_3cd4f_59[4]))),
                    ((z0_tmp_3cd4f_58[12])
                        + (((((((((x_sum_tmp_3cd4f_60[0]) * (y_sum_tmp_3cd4f_61[5]))
                            + ((x_sum_tmp_3cd4f_60[1]) * (y_sum_tmp_3cd4f_61[4])))
                            + ((x_sum_tmp_3cd4f_60[2]) * (y_sum_tmp_3cd4f_61[3])))
                            + ((x_sum_tmp_3cd4f_60[3]) * (y_sum_tmp_3cd4f_61[2])))
                            + ((x_sum_tmp_3cd4f_60[4]) * (y_sum_tmp_3cd4f_61[1])))
                            + ((x_sum_tmp_3cd4f_60[5]) * (y_sum_tmp_3cd4f_61[0])))
                            - (z0_tmp_3cd4f_58[5]))
                            - (z2_tmp_3cd4f_59[5]))),
                    ((((((((((x_sum_tmp_3cd4f_60[0]) * (y_sum_tmp_3cd4f_61[6]))
                        + ((x_sum_tmp_3cd4f_60[1]) * (y_sum_tmp_3cd4f_61[5])))
                        + ((x_sum_tmp_3cd4f_60[2]) * (y_sum_tmp_3cd4f_61[4])))
                        + ((x_sum_tmp_3cd4f_60[3]) * (y_sum_tmp_3cd4f_61[3])))
                        + ((x_sum_tmp_3cd4f_60[4]) * (y_sum_tmp_3cd4f_61[2])))
                        + ((x_sum_tmp_3cd4f_60[5]) * (y_sum_tmp_3cd4f_61[1])))
                        + ((x_sum_tmp_3cd4f_60[6]) * (y_sum_tmp_3cd4f_61[0])))
                        - (z0_tmp_3cd4f_58[6]))
                        - (z2_tmp_3cd4f_59[6])),
                    ((z2_tmp_3cd4f_59[0])
                        + (((((((((x_sum_tmp_3cd4f_60[1]) * (y_sum_tmp_3cd4f_61[6]))
                            + ((x_sum_tmp_3cd4f_60[2]) * (y_sum_tmp_3cd4f_61[5])))
                            + ((x_sum_tmp_3cd4f_60[3]) * (y_sum_tmp_3cd4f_61[4])))
                            + ((x_sum_tmp_3cd4f_60[4]) * (y_sum_tmp_3cd4f_61[3])))
                            + ((x_sum_tmp_3cd4f_60[5]) * (y_sum_tmp_3cd4f_61[2])))
                            + ((x_sum_tmp_3cd4f_60[6]) * (y_sum_tmp_3cd4f_61[1])))
                            - (z0_tmp_3cd4f_58[7]))
                            - (z2_tmp_3cd4f_59[7]))),
                    ((z2_tmp_3cd4f_59[1])
                        + ((((((((x_sum_tmp_3cd4f_60[2]) * (y_sum_tmp_3cd4f_61[6]))
                            + ((x_sum_tmp_3cd4f_60[3]) * (y_sum_tmp_3cd4f_61[5])))
                            + ((x_sum_tmp_3cd4f_60[4]) * (y_sum_tmp_3cd4f_61[4])))
                            + ((x_sum_tmp_3cd4f_60[5]) * (y_sum_tmp_3cd4f_61[3])))
                            + ((x_sum_tmp_3cd4f_60[6]) * (y_sum_tmp_3cd4f_61[2])))
                            - (z0_tmp_3cd4f_58[8]))
                            - (z2_tmp_3cd4f_59[8]))),
                    ((z2_tmp_3cd4f_59[2])
                        + (((((((x_sum_tmp_3cd4f_60[3]) * (y_sum_tmp_3cd4f_61[6]))
                            + ((x_sum_tmp_3cd4f_60[4]) * (y_sum_tmp_3cd4f_61[5])))
                            + ((x_sum_tmp_3cd4f_60[5]) * (y_sum_tmp_3cd4f_61[4])))
                            + ((x_sum_tmp_3cd4f_60[6]) * (y_sum_tmp_3cd4f_61[3])))
                            - (z0_tmp_3cd4f_58[9]))
                            - (z2_tmp_3cd4f_59[9]))),
                    ((z2_tmp_3cd4f_59[3])
                        + ((((((x_sum_tmp_3cd4f_60[4]) * (y_sum_tmp_3cd4f_61[6]))
                            + ((x_sum_tmp_3cd4f_60[5]) * (y_sum_tmp_3cd4f_61[5])))
                            + ((x_sum_tmp_3cd4f_60[6]) * (y_sum_tmp_3cd4f_61[4])))
                            - (z0_tmp_3cd4f_58[10]))
                            - (z2_tmp_3cd4f_59[10]))),
                    ((z2_tmp_3cd4f_59[4])
                        + (((((x_sum_tmp_3cd4f_60[5]) * (y_sum_tmp_3cd4f_61[6]))
                            + ((x_sum_tmp_3cd4f_60[6]) * (y_sum_tmp_3cd4f_61[5])))
                            - (z0_tmp_3cd4f_58[11]))
                            - (z2_tmp_3cd4f_59[11]))),
                    ((z2_tmp_3cd4f_59[5])
                        + ((((x_sum_tmp_3cd4f_60[6]) * (y_sum_tmp_3cd4f_61[6]))
                            - (z0_tmp_3cd4f_58[12]))
                            - (z2_tmp_3cd4f_59[12]))),
                    z2_tmp_3cd4f_59[6],
                    z2_tmp_3cd4f_59[7],
                    z2_tmp_3cd4f_59[8],
                    z2_tmp_3cd4f_59[9],
                    z2_tmp_3cd4f_59[10],
                    z2_tmp_3cd4f_59[11],
                    z2_tmp_3cd4f_59[12],
                ];

                let double_karatsuba_1454b_output_tmp_3cd4f_63 = [
                    single_karatsuba_n_7_output_tmp_3cd4f_50[0],
                    single_karatsuba_n_7_output_tmp_3cd4f_50[1],
                    single_karatsuba_n_7_output_tmp_3cd4f_50[2],
                    single_karatsuba_n_7_output_tmp_3cd4f_50[3],
                    single_karatsuba_n_7_output_tmp_3cd4f_50[4],
                    single_karatsuba_n_7_output_tmp_3cd4f_50[5],
                    single_karatsuba_n_7_output_tmp_3cd4f_50[6],
                    single_karatsuba_n_7_output_tmp_3cd4f_50[7],
                    single_karatsuba_n_7_output_tmp_3cd4f_50[8],
                    single_karatsuba_n_7_output_tmp_3cd4f_50[9],
                    single_karatsuba_n_7_output_tmp_3cd4f_50[10],
                    single_karatsuba_n_7_output_tmp_3cd4f_50[11],
                    single_karatsuba_n_7_output_tmp_3cd4f_50[12],
                    single_karatsuba_n_7_output_tmp_3cd4f_50[13],
                    ((single_karatsuba_n_7_output_tmp_3cd4f_50[14])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[0])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[0]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[0]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_50[15])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[1])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[1]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[1]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_50[16])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[2])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[2]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[2]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_50[17])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[3])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[3]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[3]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_50[18])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[4])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[4]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[4]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_50[19])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[5])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[5]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[5]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_50[20])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[6])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[6]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[6]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_50[21])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[7])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[7]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[7]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_50[22])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[8])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[8]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[8]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_50[23])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[9])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[9]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[9]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_50[24])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[10])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[10]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[10]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_50[25])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[11])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[11]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[11]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_50[26])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[12])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[12]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[12]))),
                    (((single_karatsuba_n_7_output_tmp_3cd4f_62[13])
                        - (single_karatsuba_n_7_output_tmp_3cd4f_50[13]))
                        - (single_karatsuba_n_7_output_tmp_3cd4f_55[13])),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_55[0])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[14])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[14]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[14]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_55[1])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[15])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[15]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[15]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_55[2])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[16])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[16]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[16]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_55[3])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[17])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[17]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[17]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_55[4])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[18])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[18]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[18]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_55[5])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[19])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[19]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[19]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_55[6])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[20])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[20]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[20]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_55[7])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[21])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[21]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[21]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_55[8])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[22])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[22]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[22]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_55[9])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[23])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[23]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[23]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_55[10])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[24])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[24]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[24]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_55[11])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[25])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[25]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[25]))),
                    ((single_karatsuba_n_7_output_tmp_3cd4f_55[12])
                        + (((single_karatsuba_n_7_output_tmp_3cd4f_62[26])
                            - (single_karatsuba_n_7_output_tmp_3cd4f_50[26]))
                            - (single_karatsuba_n_7_output_tmp_3cd4f_55[26]))),
                    single_karatsuba_n_7_output_tmp_3cd4f_55[13],
                    single_karatsuba_n_7_output_tmp_3cd4f_55[14],
                    single_karatsuba_n_7_output_tmp_3cd4f_55[15],
                    single_karatsuba_n_7_output_tmp_3cd4f_55[16],
                    single_karatsuba_n_7_output_tmp_3cd4f_55[17],
                    single_karatsuba_n_7_output_tmp_3cd4f_55[18],
                    single_karatsuba_n_7_output_tmp_3cd4f_55[19],
                    single_karatsuba_n_7_output_tmp_3cd4f_55[20],
                    single_karatsuba_n_7_output_tmp_3cd4f_55[21],
                    single_karatsuba_n_7_output_tmp_3cd4f_55[22],
                    single_karatsuba_n_7_output_tmp_3cd4f_55[23],
                    single_karatsuba_n_7_output_tmp_3cd4f_55[24],
                    single_karatsuba_n_7_output_tmp_3cd4f_55[25],
                    single_karatsuba_n_7_output_tmp_3cd4f_55[26],
                ];

                let conv_tmp_3cd4f_64 = [
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[0])
                        - ((input_limb_44_col44) + (result_y_limb_0_col240))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[1])
                        - ((input_limb_45_col45) + (result_y_limb_1_col241))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[2])
                        - ((input_limb_46_col46) + (result_y_limb_2_col242))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[3])
                        - ((input_limb_47_col47) + (result_y_limb_3_col243))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[4])
                        - ((input_limb_48_col48) + (result_y_limb_4_col244))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[5])
                        - ((input_limb_49_col49) + (result_y_limb_5_col245))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[6])
                        - ((input_limb_50_col50) + (result_y_limb_6_col246))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[7])
                        - ((input_limb_51_col51) + (result_y_limb_7_col247))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[8])
                        - ((input_limb_52_col52) + (result_y_limb_8_col248))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[9])
                        - ((input_limb_53_col53) + (result_y_limb_9_col249))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[10])
                        - ((input_limb_54_col54) + (result_y_limb_10_col250))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[11])
                        - ((input_limb_55_col55) + (result_y_limb_11_col251))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[12])
                        - ((input_limb_56_col56) + (result_y_limb_12_col252))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[13])
                        - ((input_limb_57_col57) + (result_y_limb_13_col253))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[14])
                        - ((input_limb_58_col58) + (result_y_limb_14_col254))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[15])
                        - ((input_limb_59_col59) + (result_y_limb_15_col255))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[16])
                        - ((input_limb_60_col60) + (result_y_limb_16_col256))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[17])
                        - ((input_limb_61_col61) + (result_y_limb_17_col257))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[18])
                        - ((input_limb_62_col62) + (result_y_limb_18_col258))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[19])
                        - ((input_limb_63_col63) + (result_y_limb_19_col259))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[20])
                        - ((input_limb_64_col64) + (result_y_limb_20_col260))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[21])
                        - ((input_limb_65_col65) + (result_y_limb_21_col261))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[22])
                        - ((input_limb_66_col66) + (result_y_limb_22_col262))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[23])
                        - ((input_limb_67_col67) + (result_y_limb_23_col263))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[24])
                        - ((input_limb_68_col68) + (result_y_limb_24_col264))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[25])
                        - ((input_limb_69_col69) + (result_y_limb_25_col265))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[26])
                        - ((input_limb_70_col70) + (result_y_limb_26_col266))),
                    ((double_karatsuba_1454b_output_tmp_3cd4f_63[27])
                        - ((input_limb_71_col71) + (result_y_limb_27_col267))),
                    double_karatsuba_1454b_output_tmp_3cd4f_63[28],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[29],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[30],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[31],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[32],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[33],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[34],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[35],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[36],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[37],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[38],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[39],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[40],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[41],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[42],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[43],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[44],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[45],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[46],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[47],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[48],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[49],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[50],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[51],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[52],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[53],
                    double_karatsuba_1454b_output_tmp_3cd4f_63[54],
                ];
                let conv_mod_tmp_3cd4f_65 = [
                    ((((M31_32) * (conv_tmp_3cd4f_64[0])) - ((M31_4) * (conv_tmp_3cd4f_64[21])))
                        + ((M31_8) * (conv_tmp_3cd4f_64[49]))),
                    ((((conv_tmp_3cd4f_64[0]) + ((M31_32) * (conv_tmp_3cd4f_64[1])))
                        - ((M31_4) * (conv_tmp_3cd4f_64[22])))
                        + ((M31_8) * (conv_tmp_3cd4f_64[50]))),
                    ((((conv_tmp_3cd4f_64[1]) + ((M31_32) * (conv_tmp_3cd4f_64[2])))
                        - ((M31_4) * (conv_tmp_3cd4f_64[23])))
                        + ((M31_8) * (conv_tmp_3cd4f_64[51]))),
                    ((((conv_tmp_3cd4f_64[2]) + ((M31_32) * (conv_tmp_3cd4f_64[3])))
                        - ((M31_4) * (conv_tmp_3cd4f_64[24])))
                        + ((M31_8) * (conv_tmp_3cd4f_64[52]))),
                    ((((conv_tmp_3cd4f_64[3]) + ((M31_32) * (conv_tmp_3cd4f_64[4])))
                        - ((M31_4) * (conv_tmp_3cd4f_64[25])))
                        + ((M31_8) * (conv_tmp_3cd4f_64[53]))),
                    ((((conv_tmp_3cd4f_64[4]) + ((M31_32) * (conv_tmp_3cd4f_64[5])))
                        - ((M31_4) * (conv_tmp_3cd4f_64[26])))
                        + ((M31_8) * (conv_tmp_3cd4f_64[54]))),
                    (((conv_tmp_3cd4f_64[5]) + ((M31_32) * (conv_tmp_3cd4f_64[6])))
                        - ((M31_4) * (conv_tmp_3cd4f_64[27]))),
                    (((((M31_2) * (conv_tmp_3cd4f_64[0])) + (conv_tmp_3cd4f_64[6]))
                        + ((M31_32) * (conv_tmp_3cd4f_64[7])))
                        - ((M31_4) * (conv_tmp_3cd4f_64[28]))),
                    (((((M31_2) * (conv_tmp_3cd4f_64[1])) + (conv_tmp_3cd4f_64[7]))
                        + ((M31_32) * (conv_tmp_3cd4f_64[8])))
                        - ((M31_4) * (conv_tmp_3cd4f_64[29]))),
                    (((((M31_2) * (conv_tmp_3cd4f_64[2])) + (conv_tmp_3cd4f_64[8]))
                        + ((M31_32) * (conv_tmp_3cd4f_64[9])))
                        - ((M31_4) * (conv_tmp_3cd4f_64[30]))),
                    (((((M31_2) * (conv_tmp_3cd4f_64[3])) + (conv_tmp_3cd4f_64[9]))
                        + ((M31_32) * (conv_tmp_3cd4f_64[10])))
                        - ((M31_4) * (conv_tmp_3cd4f_64[31]))),
                    (((((M31_2) * (conv_tmp_3cd4f_64[4])) + (conv_tmp_3cd4f_64[10]))
                        + ((M31_32) * (conv_tmp_3cd4f_64[11])))
                        - ((M31_4) * (conv_tmp_3cd4f_64[32]))),
                    (((((M31_2) * (conv_tmp_3cd4f_64[5])) + (conv_tmp_3cd4f_64[11]))
                        + ((M31_32) * (conv_tmp_3cd4f_64[12])))
                        - ((M31_4) * (conv_tmp_3cd4f_64[33]))),
                    (((((M31_2) * (conv_tmp_3cd4f_64[6])) + (conv_tmp_3cd4f_64[12]))
                        + ((M31_32) * (conv_tmp_3cd4f_64[13])))
                        - ((M31_4) * (conv_tmp_3cd4f_64[34]))),
                    (((((M31_2) * (conv_tmp_3cd4f_64[7])) + (conv_tmp_3cd4f_64[13]))
                        + ((M31_32) * (conv_tmp_3cd4f_64[14])))
                        - ((M31_4) * (conv_tmp_3cd4f_64[35]))),
                    (((((M31_2) * (conv_tmp_3cd4f_64[8])) + (conv_tmp_3cd4f_64[14]))
                        + ((M31_32) * (conv_tmp_3cd4f_64[15])))
                        - ((M31_4) * (conv_tmp_3cd4f_64[36]))),
                    (((((M31_2) * (conv_tmp_3cd4f_64[9])) + (conv_tmp_3cd4f_64[15]))
                        + ((M31_32) * (conv_tmp_3cd4f_64[16])))
                        - ((M31_4) * (conv_tmp_3cd4f_64[37]))),
                    (((((M31_2) * (conv_tmp_3cd4f_64[10])) + (conv_tmp_3cd4f_64[16]))
                        + ((M31_32) * (conv_tmp_3cd4f_64[17])))
                        - ((M31_4) * (conv_tmp_3cd4f_64[38]))),
                    (((((M31_2) * (conv_tmp_3cd4f_64[11])) + (conv_tmp_3cd4f_64[17]))
                        + ((M31_32) * (conv_tmp_3cd4f_64[18])))
                        - ((M31_4) * (conv_tmp_3cd4f_64[39]))),
                    (((((M31_2) * (conv_tmp_3cd4f_64[12])) + (conv_tmp_3cd4f_64[18]))
                        + ((M31_32) * (conv_tmp_3cd4f_64[19])))
                        - ((M31_4) * (conv_tmp_3cd4f_64[40]))),
                    (((((M31_2) * (conv_tmp_3cd4f_64[13])) + (conv_tmp_3cd4f_64[19]))
                        + ((M31_32) * (conv_tmp_3cd4f_64[20])))
                        - ((M31_4) * (conv_tmp_3cd4f_64[41]))),
                    (((((M31_2) * (conv_tmp_3cd4f_64[14])) + (conv_tmp_3cd4f_64[20]))
                        - ((M31_4) * (conv_tmp_3cd4f_64[42])))
                        + ((M31_64) * (conv_tmp_3cd4f_64[49]))),
                    (((((M31_2) * (conv_tmp_3cd4f_64[15])) - ((M31_4) * (conv_tmp_3cd4f_64[43])))
                        + ((M31_2) * (conv_tmp_3cd4f_64[49])))
                        + ((M31_64) * (conv_tmp_3cd4f_64[50]))),
                    (((((M31_2) * (conv_tmp_3cd4f_64[16])) - ((M31_4) * (conv_tmp_3cd4f_64[44])))
                        + ((M31_2) * (conv_tmp_3cd4f_64[50])))
                        + ((M31_64) * (conv_tmp_3cd4f_64[51]))),
                    (((((M31_2) * (conv_tmp_3cd4f_64[17])) - ((M31_4) * (conv_tmp_3cd4f_64[45])))
                        + ((M31_2) * (conv_tmp_3cd4f_64[51])))
                        + ((M31_64) * (conv_tmp_3cd4f_64[52]))),
                    (((((M31_2) * (conv_tmp_3cd4f_64[18])) - ((M31_4) * (conv_tmp_3cd4f_64[46])))
                        + ((M31_2) * (conv_tmp_3cd4f_64[52])))
                        + ((M31_64) * (conv_tmp_3cd4f_64[53]))),
                    (((((M31_2) * (conv_tmp_3cd4f_64[19])) - ((M31_4) * (conv_tmp_3cd4f_64[47])))
                        + ((M31_2) * (conv_tmp_3cd4f_64[53])))
                        + ((M31_64) * (conv_tmp_3cd4f_64[54]))),
                    ((((M31_2) * (conv_tmp_3cd4f_64[20])) - ((M31_4) * (conv_tmp_3cd4f_64[48])))
                        + ((M31_2) * (conv_tmp_3cd4f_64[54]))),
                ];
                let k_mod_2_18_biased_tmp_3cd4f_66 =
                    ((((PackedUInt32::from_m31(((conv_mod_tmp_3cd4f_65[0]) + (M31_134217728))))
                        + (((PackedUInt32::from_m31(
                            ((conv_mod_tmp_3cd4f_65[1]) + (M31_134217728)),
                        )) & (UInt32_511))
                            << (UInt32_9)))
                        + (UInt32_131072))
                        & (UInt32_262143));
                let k_col268 = ((k_mod_2_18_biased_tmp_3cd4f_66.low().as_m31())
                    + (((k_mod_2_18_biased_tmp_3cd4f_66.high().as_m31()) - (M31_2)) * (M31_65536)));
                *row[268] = k_col268;
                *sub_component_inputs.range_check_20[8] = [((k_col268) + (M31_524288))];
                *lookup_data.range_check_20_8 = [((k_col268) + (M31_524288))];
                let carry_0_col269 = (((conv_mod_tmp_3cd4f_65[0]) - (k_col268)) * (M31_4194304));
                *row[269] = carry_0_col269;
                *sub_component_inputs.range_check_20_b[8] = [((carry_0_col269) + (M31_524288))];
                *lookup_data.range_check_20_b_8 = [((carry_0_col269) + (M31_524288))];
                let carry_1_col270 =
                    (((conv_mod_tmp_3cd4f_65[1]) + (carry_0_col269)) * (M31_4194304));
                *row[270] = carry_1_col270;
                *sub_component_inputs.range_check_20_c[8] = [((carry_1_col270) + (M31_524288))];
                *lookup_data.range_check_20_c_8 = [((carry_1_col270) + (M31_524288))];
                let carry_2_col271 =
                    (((conv_mod_tmp_3cd4f_65[2]) + (carry_1_col270)) * (M31_4194304));
                *row[271] = carry_2_col271;
                *sub_component_inputs.range_check_20_d[8] = [((carry_2_col271) + (M31_524288))];
                *lookup_data.range_check_20_d_8 = [((carry_2_col271) + (M31_524288))];
                let carry_3_col272 =
                    (((conv_mod_tmp_3cd4f_65[3]) + (carry_2_col271)) * (M31_4194304));
                *row[272] = carry_3_col272;
                *sub_component_inputs.range_check_20_e[6] = [((carry_3_col272) + (M31_524288))];
                *lookup_data.range_check_20_e_6 = [((carry_3_col272) + (M31_524288))];
                let carry_4_col273 =
                    (((conv_mod_tmp_3cd4f_65[4]) + (carry_3_col272)) * (M31_4194304));
                *row[273] = carry_4_col273;
                *sub_component_inputs.range_check_20_f[6] = [((carry_4_col273) + (M31_524288))];
                *lookup_data.range_check_20_f_6 = [((carry_4_col273) + (M31_524288))];
                let carry_5_col274 =
                    (((conv_mod_tmp_3cd4f_65[5]) + (carry_4_col273)) * (M31_4194304));
                *row[274] = carry_5_col274;
                *sub_component_inputs.range_check_20_g[6] = [((carry_5_col274) + (M31_524288))];
                *lookup_data.range_check_20_g_6 = [((carry_5_col274) + (M31_524288))];
                let carry_6_col275 =
                    (((conv_mod_tmp_3cd4f_65[6]) + (carry_5_col274)) * (M31_4194304));
                *row[275] = carry_6_col275;
                *sub_component_inputs.range_check_20_h[6] = [((carry_6_col275) + (M31_524288))];
                *lookup_data.range_check_20_h_6 = [((carry_6_col275) + (M31_524288))];
                let carry_7_col276 =
                    (((conv_mod_tmp_3cd4f_65[7]) + (carry_6_col275)) * (M31_4194304));
                *row[276] = carry_7_col276;
                *sub_component_inputs.range_check_20[9] = [((carry_7_col276) + (M31_524288))];
                *lookup_data.range_check_20_9 = [((carry_7_col276) + (M31_524288))];
                let carry_8_col277 =
                    (((conv_mod_tmp_3cd4f_65[8]) + (carry_7_col276)) * (M31_4194304));
                *row[277] = carry_8_col277;
                *sub_component_inputs.range_check_20_b[9] = [((carry_8_col277) + (M31_524288))];
                *lookup_data.range_check_20_b_9 = [((carry_8_col277) + (M31_524288))];
                let carry_9_col278 =
                    (((conv_mod_tmp_3cd4f_65[9]) + (carry_8_col277)) * (M31_4194304));
                *row[278] = carry_9_col278;
                *sub_component_inputs.range_check_20_c[9] = [((carry_9_col278) + (M31_524288))];
                *lookup_data.range_check_20_c_9 = [((carry_9_col278) + (M31_524288))];
                let carry_10_col279 =
                    (((conv_mod_tmp_3cd4f_65[10]) + (carry_9_col278)) * (M31_4194304));
                *row[279] = carry_10_col279;
                *sub_component_inputs.range_check_20_d[9] = [((carry_10_col279) + (M31_524288))];
                *lookup_data.range_check_20_d_9 = [((carry_10_col279) + (M31_524288))];
                let carry_11_col280 =
                    (((conv_mod_tmp_3cd4f_65[11]) + (carry_10_col279)) * (M31_4194304));
                *row[280] = carry_11_col280;
                *sub_component_inputs.range_check_20_e[7] = [((carry_11_col280) + (M31_524288))];
                *lookup_data.range_check_20_e_7 = [((carry_11_col280) + (M31_524288))];
                let carry_12_col281 =
                    (((conv_mod_tmp_3cd4f_65[12]) + (carry_11_col280)) * (M31_4194304));
                *row[281] = carry_12_col281;
                *sub_component_inputs.range_check_20_f[7] = [((carry_12_col281) + (M31_524288))];
                *lookup_data.range_check_20_f_7 = [((carry_12_col281) + (M31_524288))];
                let carry_13_col282 =
                    (((conv_mod_tmp_3cd4f_65[13]) + (carry_12_col281)) * (M31_4194304));
                *row[282] = carry_13_col282;
                *sub_component_inputs.range_check_20_g[7] = [((carry_13_col282) + (M31_524288))];
                *lookup_data.range_check_20_g_7 = [((carry_13_col282) + (M31_524288))];
                let carry_14_col283 =
                    (((conv_mod_tmp_3cd4f_65[14]) + (carry_13_col282)) * (M31_4194304));
                *row[283] = carry_14_col283;
                *sub_component_inputs.range_check_20_h[7] = [((carry_14_col283) + (M31_524288))];
                *lookup_data.range_check_20_h_7 = [((carry_14_col283) + (M31_524288))];
                let carry_15_col284 =
                    (((conv_mod_tmp_3cd4f_65[15]) + (carry_14_col283)) * (M31_4194304));
                *row[284] = carry_15_col284;
                *sub_component_inputs.range_check_20[10] = [((carry_15_col284) + (M31_524288))];
                *lookup_data.range_check_20_10 = [((carry_15_col284) + (M31_524288))];
                let carry_16_col285 =
                    (((conv_mod_tmp_3cd4f_65[16]) + (carry_15_col284)) * (M31_4194304));
                *row[285] = carry_16_col285;
                *sub_component_inputs.range_check_20_b[10] = [((carry_16_col285) + (M31_524288))];
                *lookup_data.range_check_20_b_10 = [((carry_16_col285) + (M31_524288))];
                let carry_17_col286 =
                    (((conv_mod_tmp_3cd4f_65[17]) + (carry_16_col285)) * (M31_4194304));
                *row[286] = carry_17_col286;
                *sub_component_inputs.range_check_20_c[10] = [((carry_17_col286) + (M31_524288))];
                *lookup_data.range_check_20_c_10 = [((carry_17_col286) + (M31_524288))];
                let carry_18_col287 =
                    (((conv_mod_tmp_3cd4f_65[18]) + (carry_17_col286)) * (M31_4194304));
                *row[287] = carry_18_col287;
                *sub_component_inputs.range_check_20_d[10] = [((carry_18_col287) + (M31_524288))];
                *lookup_data.range_check_20_d_10 = [((carry_18_col287) + (M31_524288))];
                let carry_19_col288 =
                    (((conv_mod_tmp_3cd4f_65[19]) + (carry_18_col287)) * (M31_4194304));
                *row[288] = carry_19_col288;
                *sub_component_inputs.range_check_20_e[8] = [((carry_19_col288) + (M31_524288))];
                *lookup_data.range_check_20_e_8 = [((carry_19_col288) + (M31_524288))];
                let carry_20_col289 =
                    (((conv_mod_tmp_3cd4f_65[20]) + (carry_19_col288)) * (M31_4194304));
                *row[289] = carry_20_col289;
                *sub_component_inputs.range_check_20_f[8] = [((carry_20_col289) + (M31_524288))];
                *lookup_data.range_check_20_f_8 = [((carry_20_col289) + (M31_524288))];
                let carry_21_col290 = ((((conv_mod_tmp_3cd4f_65[21]) - ((M31_136) * (k_col268)))
                    + (carry_20_col289))
                    * (M31_4194304));
                *row[290] = carry_21_col290;
                *sub_component_inputs.range_check_20_g[8] = [((carry_21_col290) + (M31_524288))];
                *lookup_data.range_check_20_g_8 = [((carry_21_col290) + (M31_524288))];
                let carry_22_col291 =
                    (((conv_mod_tmp_3cd4f_65[22]) + (carry_21_col290)) * (M31_4194304));
                *row[291] = carry_22_col291;
                *sub_component_inputs.range_check_20_h[8] = [((carry_22_col291) + (M31_524288))];
                *lookup_data.range_check_20_h_8 = [((carry_22_col291) + (M31_524288))];
                let carry_23_col292 =
                    (((conv_mod_tmp_3cd4f_65[23]) + (carry_22_col291)) * (M31_4194304));
                *row[292] = carry_23_col292;
                *sub_component_inputs.range_check_20[11] = [((carry_23_col292) + (M31_524288))];
                *lookup_data.range_check_20_11 = [((carry_23_col292) + (M31_524288))];
                let carry_24_col293 =
                    (((conv_mod_tmp_3cd4f_65[24]) + (carry_23_col292)) * (M31_4194304));
                *row[293] = carry_24_col293;
                *sub_component_inputs.range_check_20_b[11] = [((carry_24_col293) + (M31_524288))];
                *lookup_data.range_check_20_b_11 = [((carry_24_col293) + (M31_524288))];
                let carry_25_col294 =
                    (((conv_mod_tmp_3cd4f_65[25]) + (carry_24_col293)) * (M31_4194304));
                *row[294] = carry_25_col294;
                *sub_component_inputs.range_check_20_c[11] = [((carry_25_col294) + (M31_524288))];
                *lookup_data.range_check_20_c_11 = [((carry_25_col294) + (M31_524288))];
                let carry_26_col295 =
                    (((conv_mod_tmp_3cd4f_65[26]) + (carry_25_col294)) * (M31_4194304));
                *row[295] = carry_26_col295;
                *sub_component_inputs.range_check_20_d[11] = [((carry_26_col295) + (M31_524288))];
                *lookup_data.range_check_20_d_11 = [((carry_26_col295) + (M31_524288))];

                let ec_add_output_tmp_3cd4f_67 = [result_x_tmp_3cd4f_23, result_y_tmp_3cd4f_45];

                *lookup_data.partial_ec_mul_window_bits_18_0 = [
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
                    input_limb_50_col50,
                    input_limb_51_col51,
                    input_limb_52_col52,
                    input_limb_53_col53,
                    input_limb_54_col54,
                    input_limb_55_col55,
                    input_limb_56_col56,
                    input_limb_57_col57,
                    input_limb_58_col58,
                    input_limb_59_col59,
                    input_limb_60_col60,
                    input_limb_61_col61,
                    input_limb_62_col62,
                    input_limb_63_col63,
                    input_limb_64_col64,
                    input_limb_65_col65,
                    input_limb_66_col66,
                    input_limb_67_col67,
                    input_limb_68_col68,
                    input_limb_69_col69,
                    input_limb_70_col70,
                    input_limb_71_col71,
                ];
                *lookup_data.partial_ec_mul_window_bits_18_1 = [
                    input_limb_0_col0,
                    ((input_limb_1_col1) + (M31_1)),
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
                    M31_0,
                    result_x_limb_0_col184,
                    result_x_limb_1_col185,
                    result_x_limb_2_col186,
                    result_x_limb_3_col187,
                    result_x_limb_4_col188,
                    result_x_limb_5_col189,
                    result_x_limb_6_col190,
                    result_x_limb_7_col191,
                    result_x_limb_8_col192,
                    result_x_limb_9_col193,
                    result_x_limb_10_col194,
                    result_x_limb_11_col195,
                    result_x_limb_12_col196,
                    result_x_limb_13_col197,
                    result_x_limb_14_col198,
                    result_x_limb_15_col199,
                    result_x_limb_16_col200,
                    result_x_limb_17_col201,
                    result_x_limb_18_col202,
                    result_x_limb_19_col203,
                    result_x_limb_20_col204,
                    result_x_limb_21_col205,
                    result_x_limb_22_col206,
                    result_x_limb_23_col207,
                    result_x_limb_24_col208,
                    result_x_limb_25_col209,
                    result_x_limb_26_col210,
                    result_x_limb_27_col211,
                    result_y_limb_0_col240,
                    result_y_limb_1_col241,
                    result_y_limb_2_col242,
                    result_y_limb_3_col243,
                    result_y_limb_4_col244,
                    result_y_limb_5_col245,
                    result_y_limb_6_col246,
                    result_y_limb_7_col247,
                    result_y_limb_8_col248,
                    result_y_limb_9_col249,
                    result_y_limb_10_col250,
                    result_y_limb_11_col251,
                    result_y_limb_12_col252,
                    result_y_limb_13_col253,
                    result_y_limb_14_col254,
                    result_y_limb_15_col255,
                    result_y_limb_16_col256,
                    result_y_limb_17_col257,
                    result_y_limb_18_col258,
                    result_y_limb_19_col259,
                    result_y_limb_20_col260,
                    result_y_limb_21_col261,
                    result_y_limb_22_col262,
                    result_y_limb_23_col263,
                    result_y_limb_24_col264,
                    result_y_limb_25_col265,
                    result_y_limb_26_col266,
                    result_y_limb_27_col267,
                ];
                *row[296] = enabler_col.packed_at(row_index);
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    partial_ec_mul_window_bits_18_0: Vec<[PackedM31; 72]>,
    partial_ec_mul_window_bits_18_1: Vec<[PackedM31; 72]>,
    pedersen_points_table_window_bits_18_0: Vec<[PackedM31; 57]>,
    range_check_20_0: Vec<[PackedM31; 1]>,
    range_check_20_1: Vec<[PackedM31; 1]>,
    range_check_20_2: Vec<[PackedM31; 1]>,
    range_check_20_3: Vec<[PackedM31; 1]>,
    range_check_20_4: Vec<[PackedM31; 1]>,
    range_check_20_5: Vec<[PackedM31; 1]>,
    range_check_20_6: Vec<[PackedM31; 1]>,
    range_check_20_7: Vec<[PackedM31; 1]>,
    range_check_20_8: Vec<[PackedM31; 1]>,
    range_check_20_9: Vec<[PackedM31; 1]>,
    range_check_20_10: Vec<[PackedM31; 1]>,
    range_check_20_11: Vec<[PackedM31; 1]>,
    range_check_20_b_0: Vec<[PackedM31; 1]>,
    range_check_20_b_1: Vec<[PackedM31; 1]>,
    range_check_20_b_2: Vec<[PackedM31; 1]>,
    range_check_20_b_3: Vec<[PackedM31; 1]>,
    range_check_20_b_4: Vec<[PackedM31; 1]>,
    range_check_20_b_5: Vec<[PackedM31; 1]>,
    range_check_20_b_6: Vec<[PackedM31; 1]>,
    range_check_20_b_7: Vec<[PackedM31; 1]>,
    range_check_20_b_8: Vec<[PackedM31; 1]>,
    range_check_20_b_9: Vec<[PackedM31; 1]>,
    range_check_20_b_10: Vec<[PackedM31; 1]>,
    range_check_20_b_11: Vec<[PackedM31; 1]>,
    range_check_20_c_0: Vec<[PackedM31; 1]>,
    range_check_20_c_1: Vec<[PackedM31; 1]>,
    range_check_20_c_2: Vec<[PackedM31; 1]>,
    range_check_20_c_3: Vec<[PackedM31; 1]>,
    range_check_20_c_4: Vec<[PackedM31; 1]>,
    range_check_20_c_5: Vec<[PackedM31; 1]>,
    range_check_20_c_6: Vec<[PackedM31; 1]>,
    range_check_20_c_7: Vec<[PackedM31; 1]>,
    range_check_20_c_8: Vec<[PackedM31; 1]>,
    range_check_20_c_9: Vec<[PackedM31; 1]>,
    range_check_20_c_10: Vec<[PackedM31; 1]>,
    range_check_20_c_11: Vec<[PackedM31; 1]>,
    range_check_20_d_0: Vec<[PackedM31; 1]>,
    range_check_20_d_1: Vec<[PackedM31; 1]>,
    range_check_20_d_2: Vec<[PackedM31; 1]>,
    range_check_20_d_3: Vec<[PackedM31; 1]>,
    range_check_20_d_4: Vec<[PackedM31; 1]>,
    range_check_20_d_5: Vec<[PackedM31; 1]>,
    range_check_20_d_6: Vec<[PackedM31; 1]>,
    range_check_20_d_7: Vec<[PackedM31; 1]>,
    range_check_20_d_8: Vec<[PackedM31; 1]>,
    range_check_20_d_9: Vec<[PackedM31; 1]>,
    range_check_20_d_10: Vec<[PackedM31; 1]>,
    range_check_20_d_11: Vec<[PackedM31; 1]>,
    range_check_20_e_0: Vec<[PackedM31; 1]>,
    range_check_20_e_1: Vec<[PackedM31; 1]>,
    range_check_20_e_2: Vec<[PackedM31; 1]>,
    range_check_20_e_3: Vec<[PackedM31; 1]>,
    range_check_20_e_4: Vec<[PackedM31; 1]>,
    range_check_20_e_5: Vec<[PackedM31; 1]>,
    range_check_20_e_6: Vec<[PackedM31; 1]>,
    range_check_20_e_7: Vec<[PackedM31; 1]>,
    range_check_20_e_8: Vec<[PackedM31; 1]>,
    range_check_20_f_0: Vec<[PackedM31; 1]>,
    range_check_20_f_1: Vec<[PackedM31; 1]>,
    range_check_20_f_2: Vec<[PackedM31; 1]>,
    range_check_20_f_3: Vec<[PackedM31; 1]>,
    range_check_20_f_4: Vec<[PackedM31; 1]>,
    range_check_20_f_5: Vec<[PackedM31; 1]>,
    range_check_20_f_6: Vec<[PackedM31; 1]>,
    range_check_20_f_7: Vec<[PackedM31; 1]>,
    range_check_20_f_8: Vec<[PackedM31; 1]>,
    range_check_20_g_0: Vec<[PackedM31; 1]>,
    range_check_20_g_1: Vec<[PackedM31; 1]>,
    range_check_20_g_2: Vec<[PackedM31; 1]>,
    range_check_20_g_3: Vec<[PackedM31; 1]>,
    range_check_20_g_4: Vec<[PackedM31; 1]>,
    range_check_20_g_5: Vec<[PackedM31; 1]>,
    range_check_20_g_6: Vec<[PackedM31; 1]>,
    range_check_20_g_7: Vec<[PackedM31; 1]>,
    range_check_20_g_8: Vec<[PackedM31; 1]>,
    range_check_20_h_0: Vec<[PackedM31; 1]>,
    range_check_20_h_1: Vec<[PackedM31; 1]>,
    range_check_20_h_2: Vec<[PackedM31; 1]>,
    range_check_20_h_3: Vec<[PackedM31; 1]>,
    range_check_20_h_4: Vec<[PackedM31; 1]>,
    range_check_20_h_5: Vec<[PackedM31; 1]>,
    range_check_20_h_6: Vec<[PackedM31; 1]>,
    range_check_20_h_7: Vec<[PackedM31; 1]>,
    range_check_20_h_8: Vec<[PackedM31; 1]>,
    range_check_9_9_0: Vec<[PackedM31; 2]>,
    range_check_9_9_1: Vec<[PackedM31; 2]>,
    range_check_9_9_2: Vec<[PackedM31; 2]>,
    range_check_9_9_3: Vec<[PackedM31; 2]>,
    range_check_9_9_4: Vec<[PackedM31; 2]>,
    range_check_9_9_5: Vec<[PackedM31; 2]>,
    range_check_9_9_b_0: Vec<[PackedM31; 2]>,
    range_check_9_9_b_1: Vec<[PackedM31; 2]>,
    range_check_9_9_b_2: Vec<[PackedM31; 2]>,
    range_check_9_9_b_3: Vec<[PackedM31; 2]>,
    range_check_9_9_b_4: Vec<[PackedM31; 2]>,
    range_check_9_9_b_5: Vec<[PackedM31; 2]>,
    range_check_9_9_c_0: Vec<[PackedM31; 2]>,
    range_check_9_9_c_1: Vec<[PackedM31; 2]>,
    range_check_9_9_c_2: Vec<[PackedM31; 2]>,
    range_check_9_9_c_3: Vec<[PackedM31; 2]>,
    range_check_9_9_c_4: Vec<[PackedM31; 2]>,
    range_check_9_9_c_5: Vec<[PackedM31; 2]>,
    range_check_9_9_d_0: Vec<[PackedM31; 2]>,
    range_check_9_9_d_1: Vec<[PackedM31; 2]>,
    range_check_9_9_d_2: Vec<[PackedM31; 2]>,
    range_check_9_9_d_3: Vec<[PackedM31; 2]>,
    range_check_9_9_d_4: Vec<[PackedM31; 2]>,
    range_check_9_9_d_5: Vec<[PackedM31; 2]>,
    range_check_9_9_e_0: Vec<[PackedM31; 2]>,
    range_check_9_9_e_1: Vec<[PackedM31; 2]>,
    range_check_9_9_e_2: Vec<[PackedM31; 2]>,
    range_check_9_9_e_3: Vec<[PackedM31; 2]>,
    range_check_9_9_e_4: Vec<[PackedM31; 2]>,
    range_check_9_9_e_5: Vec<[PackedM31; 2]>,
    range_check_9_9_f_0: Vec<[PackedM31; 2]>,
    range_check_9_9_f_1: Vec<[PackedM31; 2]>,
    range_check_9_9_f_2: Vec<[PackedM31; 2]>,
    range_check_9_9_f_3: Vec<[PackedM31; 2]>,
    range_check_9_9_f_4: Vec<[PackedM31; 2]>,
    range_check_9_9_f_5: Vec<[PackedM31; 2]>,
    range_check_9_9_g_0: Vec<[PackedM31; 2]>,
    range_check_9_9_g_1: Vec<[PackedM31; 2]>,
    range_check_9_9_g_2: Vec<[PackedM31; 2]>,
    range_check_9_9_h_0: Vec<[PackedM31; 2]>,
    range_check_9_9_h_1: Vec<[PackedM31; 2]>,
    range_check_9_9_h_2: Vec<[PackedM31; 2]>,
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
        pedersen_points_table_window_bits_18: &relations::PedersenPointsTableWindowBits18,
        range_check_9_9: &relations::RangeCheck_9_9,
        range_check_9_9_b: &relations::RangeCheck_9_9_B,
        range_check_9_9_c: &relations::RangeCheck_9_9_C,
        range_check_9_9_d: &relations::RangeCheck_9_9_D,
        range_check_9_9_e: &relations::RangeCheck_9_9_E,
        range_check_9_9_f: &relations::RangeCheck_9_9_F,
        range_check_9_9_g: &relations::RangeCheck_9_9_G,
        range_check_9_9_h: &relations::RangeCheck_9_9_H,
        range_check_20: &relations::RangeCheck_20,
        range_check_20_b: &relations::RangeCheck_20_B,
        range_check_20_c: &relations::RangeCheck_20_C,
        range_check_20_d: &relations::RangeCheck_20_D,
        range_check_20_e: &relations::RangeCheck_20_E,
        range_check_20_f: &relations::RangeCheck_20_F,
        range_check_20_g: &relations::RangeCheck_20_G,
        range_check_20_h: &relations::RangeCheck_20_H,
        partial_ec_mul_window_bits_18: &relations::PartialEcMulWindowBits18,
    ) -> InteractionClaim {
        let enabler_col = Enabler::new(self.n_rows);
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.pedersen_points_table_window_bits_18_0,
            &self.lookup_data.range_check_9_9_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = pedersen_points_table_window_bits_18.combine(values0);
                let denom1: PackedQM31 = range_check_9_9.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_b_0,
            &self.lookup_data.range_check_9_9_c_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_b.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_d_0,
            &self.lookup_data.range_check_9_9_e_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_d.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_e.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_f_0,
            &self.lookup_data.range_check_9_9_g_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_f.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_g.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_h_0,
            &self.lookup_data.range_check_9_9_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_h.combine(values0);
                let denom1: PackedQM31 = range_check_9_9.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_b_1,
            &self.lookup_data.range_check_9_9_c_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_b.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_d_1,
            &self.lookup_data.range_check_9_9_e_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_d.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_e.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_f_1,
            &self.lookup_data.range_check_20_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_f.combine(values0);
                let denom1: PackedQM31 = range_check_20.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_b_0,
            &self.lookup_data.range_check_20_c_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_b.combine(values0);
                let denom1: PackedQM31 = range_check_20_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_d_0,
            &self.lookup_data.range_check_20_e_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_d.combine(values0);
                let denom1: PackedQM31 = range_check_20_e.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_f_0,
            &self.lookup_data.range_check_20_g_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_f.combine(values0);
                let denom1: PackedQM31 = range_check_20_g.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_h_0,
            &self.lookup_data.range_check_20_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_h.combine(values0);
                let denom1: PackedQM31 = range_check_20.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_b_1,
            &self.lookup_data.range_check_20_c_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_b.combine(values0);
                let denom1: PackedQM31 = range_check_20_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_d_1,
            &self.lookup_data.range_check_20_e_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_d.combine(values0);
                let denom1: PackedQM31 = range_check_20_e.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_f_1,
            &self.lookup_data.range_check_20_g_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_f.combine(values0);
                let denom1: PackedQM31 = range_check_20_g.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_h_1,
            &self.lookup_data.range_check_20_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_h.combine(values0);
                let denom1: PackedQM31 = range_check_20.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_b_2,
            &self.lookup_data.range_check_20_c_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_b.combine(values0);
                let denom1: PackedQM31 = range_check_20_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_d_2,
            &self.lookup_data.range_check_20_e_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_d.combine(values0);
                let denom1: PackedQM31 = range_check_20_e.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_f_2,
            &self.lookup_data.range_check_20_g_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_f.combine(values0);
                let denom1: PackedQM31 = range_check_20_g.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_h_2,
            &self.lookup_data.range_check_20_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_h.combine(values0);
                let denom1: PackedQM31 = range_check_20.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_b_3,
            &self.lookup_data.range_check_20_c_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_b.combine(values0);
                let denom1: PackedQM31 = range_check_20_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_d_3,
            &self.lookup_data.range_check_9_9_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_d.combine(values0);
                let denom1: PackedQM31 = range_check_9_9.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_b_2,
            &self.lookup_data.range_check_9_9_c_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_b.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_d_2,
            &self.lookup_data.range_check_9_9_e_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_d.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_e.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_f_2,
            &self.lookup_data.range_check_9_9_g_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_f.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_g.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_h_1,
            &self.lookup_data.range_check_9_9_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_h.combine(values0);
                let denom1: PackedQM31 = range_check_9_9.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_b_3,
            &self.lookup_data.range_check_9_9_c_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_b.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_d_3,
            &self.lookup_data.range_check_9_9_e_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_d.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_e.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_f_3,
            &self.lookup_data.range_check_20_4,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_f.combine(values0);
                let denom1: PackedQM31 = range_check_20.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_b_4,
            &self.lookup_data.range_check_20_c_4,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_b.combine(values0);
                let denom1: PackedQM31 = range_check_20_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_d_4,
            &self.lookup_data.range_check_20_e_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_d.combine(values0);
                let denom1: PackedQM31 = range_check_20_e.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_f_3,
            &self.lookup_data.range_check_20_g_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_f.combine(values0);
                let denom1: PackedQM31 = range_check_20_g.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_h_3,
            &self.lookup_data.range_check_20_5,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_h.combine(values0);
                let denom1: PackedQM31 = range_check_20.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_b_5,
            &self.lookup_data.range_check_20_c_5,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_b.combine(values0);
                let denom1: PackedQM31 = range_check_20_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_d_5,
            &self.lookup_data.range_check_20_e_4,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_d.combine(values0);
                let denom1: PackedQM31 = range_check_20_e.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_f_4,
            &self.lookup_data.range_check_20_g_4,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_f.combine(values0);
                let denom1: PackedQM31 = range_check_20_g.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_h_4,
            &self.lookup_data.range_check_20_6,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_h.combine(values0);
                let denom1: PackedQM31 = range_check_20.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_b_6,
            &self.lookup_data.range_check_20_c_6,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_b.combine(values0);
                let denom1: PackedQM31 = range_check_20_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_d_6,
            &self.lookup_data.range_check_20_e_5,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_d.combine(values0);
                let denom1: PackedQM31 = range_check_20_e.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_f_5,
            &self.lookup_data.range_check_20_g_5,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_f.combine(values0);
                let denom1: PackedQM31 = range_check_20_g.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_h_5,
            &self.lookup_data.range_check_20_7,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_h.combine(values0);
                let denom1: PackedQM31 = range_check_20.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_b_7,
            &self.lookup_data.range_check_20_c_7,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_b.combine(values0);
                let denom1: PackedQM31 = range_check_20_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_d_7,
            &self.lookup_data.range_check_9_9_4,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_d.combine(values0);
                let denom1: PackedQM31 = range_check_9_9.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_b_4,
            &self.lookup_data.range_check_9_9_c_4,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_b.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_d_4,
            &self.lookup_data.range_check_9_9_e_4,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_d.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_e.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_f_4,
            &self.lookup_data.range_check_9_9_g_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_f.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_g.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_h_2,
            &self.lookup_data.range_check_9_9_5,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_h.combine(values0);
                let denom1: PackedQM31 = range_check_9_9.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_b_5,
            &self.lookup_data.range_check_9_9_c_5,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_b.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_d_5,
            &self.lookup_data.range_check_9_9_e_5,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_d.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_e.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_f_5,
            &self.lookup_data.range_check_20_8,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_f.combine(values0);
                let denom1: PackedQM31 = range_check_20.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_b_8,
            &self.lookup_data.range_check_20_c_8,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_b.combine(values0);
                let denom1: PackedQM31 = range_check_20_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_d_8,
            &self.lookup_data.range_check_20_e_6,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_d.combine(values0);
                let denom1: PackedQM31 = range_check_20_e.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_f_6,
            &self.lookup_data.range_check_20_g_6,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_f.combine(values0);
                let denom1: PackedQM31 = range_check_20_g.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_h_6,
            &self.lookup_data.range_check_20_9,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_h.combine(values0);
                let denom1: PackedQM31 = range_check_20.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_b_9,
            &self.lookup_data.range_check_20_c_9,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_b.combine(values0);
                let denom1: PackedQM31 = range_check_20_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_d_9,
            &self.lookup_data.range_check_20_e_7,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_d.combine(values0);
                let denom1: PackedQM31 = range_check_20_e.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_f_7,
            &self.lookup_data.range_check_20_g_7,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_f.combine(values0);
                let denom1: PackedQM31 = range_check_20_g.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_h_7,
            &self.lookup_data.range_check_20_10,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_h.combine(values0);
                let denom1: PackedQM31 = range_check_20.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_b_10,
            &self.lookup_data.range_check_20_c_10,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_b.combine(values0);
                let denom1: PackedQM31 = range_check_20_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_d_10,
            &self.lookup_data.range_check_20_e_8,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_d.combine(values0);
                let denom1: PackedQM31 = range_check_20_e.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_f_8,
            &self.lookup_data.range_check_20_g_8,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_f.combine(values0);
                let denom1: PackedQM31 = range_check_20_g.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_h_8,
            &self.lookup_data.range_check_20_11,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_h.combine(values0);
                let denom1: PackedQM31 = range_check_20.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_b_11,
            &self.lookup_data.range_check_20_c_11,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_b.combine(values0);
                let denom1: PackedQM31 = range_check_20_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_d_11,
            &self.lookup_data.partial_ec_mul_window_bits_18_0,
        )
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (writer, values0, values1))| {
                let denom0: PackedQM31 = range_check_20_d.combine(values0);
                let denom1: PackedQM31 = partial_ec_mul_window_bits_18.combine(values1);
                writer.write_frac(denom0 * enabler_col.packed_at(i) + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.partial_ec_mul_window_bits_18_1,
        )
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (writer, values))| {
                let denom = partial_ec_mul_window_bits_18.combine(values);
                writer.write_frac(-PackedQM31::one() * enabler_col.packed_at(i), denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
