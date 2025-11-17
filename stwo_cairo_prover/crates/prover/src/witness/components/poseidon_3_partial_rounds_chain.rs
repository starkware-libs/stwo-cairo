// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::poseidon_3_partial_rounds_chain::{
    Claim, InteractionClaim, N_TRACE_COLUMNS,
};

use crate::witness::components::{
    cube_252, poseidon_round_keys, range_check_252_width_27, range_check_4_4, range_check_4_4_4_4,
};
use crate::witness::prelude::*;

pub type PackedInputType = (PackedM31, PackedM31, [PackedFelt252Width27; 4]);

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
        poseidon_round_keys_state: &poseidon_round_keys::ClaimGenerator,
        cube_252_state: &mut cube_252::ClaimGenerator,
        range_check_4_4_4_4_state: &range_check_4_4_4_4::ClaimGenerator,
        range_check_4_4_state: &range_check_4_4::ClaimGenerator,
        range_check_252_width_27_state: &mut range_check_252_width_27::ClaimGenerator,
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
            poseidon_round_keys_state,
            cube_252_state,
            range_check_4_4_4_4_state,
            range_check_4_4_state,
            range_check_252_width_27_state,
        );
        sub_component_inputs
            .poseidon_round_keys
            .iter()
            .for_each(|inputs| {
                poseidon_round_keys_state.add_packed_inputs(inputs);
            });
        sub_component_inputs.cube_252.iter().for_each(|inputs| {
            cube_252_state.add_packed_inputs(inputs);
        });
        sub_component_inputs
            .range_check_4_4_4_4
            .iter()
            .for_each(|inputs| {
                range_check_4_4_4_4_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_4_4
            .iter()
            .for_each(|inputs| {
                range_check_4_4_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_252_width_27
            .iter()
            .for_each(|inputs| {
                range_check_252_width_27_state.add_packed_inputs(inputs);
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
    poseidon_round_keys: [Vec<poseidon_round_keys::PackedInputType>; 1],
    cube_252: [Vec<cube_252::PackedInputType>; 3],
    range_check_4_4_4_4: [Vec<range_check_4_4_4_4::PackedInputType>; 6],
    range_check_4_4: [Vec<range_check_4_4::PackedInputType>; 3],
    range_check_252_width_27: [Vec<range_check_252_width_27::PackedInputType>; 3],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    n_rows: usize,
    poseidon_round_keys_state: &poseidon_round_keys::ClaimGenerator,
    cube_252_state: &mut cube_252::ClaimGenerator,
    range_check_4_4_4_4_state: &range_check_4_4_4_4::ClaimGenerator,
    range_check_4_4_state: &range_check_4_4::ClaimGenerator,
    range_check_252_width_27_state: &mut range_check_252_width_27::ClaimGenerator,
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

    let Felt252_0_0_0_0 = PackedFelt252::broadcast(Felt252::from([0, 0, 0, 0]));
    let Felt252_1_0_0_0 = PackedFelt252::broadcast(Felt252::from([1, 0, 0, 0]));
    let Felt252_2_0_0_0 = PackedFelt252::broadcast(Felt252::from([2, 0, 0, 0]));
    let Felt252_3_0_0_0 = PackedFelt252::broadcast(Felt252::from([3, 0, 0, 0]));
    let Felt252_4_0_0_0 = PackedFelt252::broadcast(Felt252::from([4, 0, 0, 0]));
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_134217729 = PackedM31::broadcast(M31::from(134217729));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_16 = PackedM31::broadcast(M31::from(16));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_268435458 = PackedM31::broadcast(M31::from(268435458));
    let M31_3 = PackedM31::broadcast(M31::from(3));
    let M31_4 = PackedM31::broadcast(M31::from(4));
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
                (row, lookup_data, sub_component_inputs, poseidon_3_partial_rounds_chain_input),
            )| {
                let input_limb_0_col0 = poseidon_3_partial_rounds_chain_input.0;
                *row[0] = input_limb_0_col0;
                let input_limb_1_col1 = poseidon_3_partial_rounds_chain_input.1;
                *row[1] = input_limb_1_col1;
                let input_limb_2_col2 = poseidon_3_partial_rounds_chain_input.2[0].get_m31(0);
                *row[2] = input_limb_2_col2;
                let input_limb_3_col3 = poseidon_3_partial_rounds_chain_input.2[0].get_m31(1);
                *row[3] = input_limb_3_col3;
                let input_limb_4_col4 = poseidon_3_partial_rounds_chain_input.2[0].get_m31(2);
                *row[4] = input_limb_4_col4;
                let input_limb_5_col5 = poseidon_3_partial_rounds_chain_input.2[0].get_m31(3);
                *row[5] = input_limb_5_col5;
                let input_limb_6_col6 = poseidon_3_partial_rounds_chain_input.2[0].get_m31(4);
                *row[6] = input_limb_6_col6;
                let input_limb_7_col7 = poseidon_3_partial_rounds_chain_input.2[0].get_m31(5);
                *row[7] = input_limb_7_col7;
                let input_limb_8_col8 = poseidon_3_partial_rounds_chain_input.2[0].get_m31(6);
                *row[8] = input_limb_8_col8;
                let input_limb_9_col9 = poseidon_3_partial_rounds_chain_input.2[0].get_m31(7);
                *row[9] = input_limb_9_col9;
                let input_limb_10_col10 = poseidon_3_partial_rounds_chain_input.2[0].get_m31(8);
                *row[10] = input_limb_10_col10;
                let input_limb_11_col11 = poseidon_3_partial_rounds_chain_input.2[0].get_m31(9);
                *row[11] = input_limb_11_col11;
                let input_limb_12_col12 = poseidon_3_partial_rounds_chain_input.2[1].get_m31(0);
                *row[12] = input_limb_12_col12;
                let input_limb_13_col13 = poseidon_3_partial_rounds_chain_input.2[1].get_m31(1);
                *row[13] = input_limb_13_col13;
                let input_limb_14_col14 = poseidon_3_partial_rounds_chain_input.2[1].get_m31(2);
                *row[14] = input_limb_14_col14;
                let input_limb_15_col15 = poseidon_3_partial_rounds_chain_input.2[1].get_m31(3);
                *row[15] = input_limb_15_col15;
                let input_limb_16_col16 = poseidon_3_partial_rounds_chain_input.2[1].get_m31(4);
                *row[16] = input_limb_16_col16;
                let input_limb_17_col17 = poseidon_3_partial_rounds_chain_input.2[1].get_m31(5);
                *row[17] = input_limb_17_col17;
                let input_limb_18_col18 = poseidon_3_partial_rounds_chain_input.2[1].get_m31(6);
                *row[18] = input_limb_18_col18;
                let input_limb_19_col19 = poseidon_3_partial_rounds_chain_input.2[1].get_m31(7);
                *row[19] = input_limb_19_col19;
                let input_limb_20_col20 = poseidon_3_partial_rounds_chain_input.2[1].get_m31(8);
                *row[20] = input_limb_20_col20;
                let input_limb_21_col21 = poseidon_3_partial_rounds_chain_input.2[1].get_m31(9);
                *row[21] = input_limb_21_col21;
                let input_limb_22_col22 = poseidon_3_partial_rounds_chain_input.2[2].get_m31(0);
                *row[22] = input_limb_22_col22;
                let input_limb_23_col23 = poseidon_3_partial_rounds_chain_input.2[2].get_m31(1);
                *row[23] = input_limb_23_col23;
                let input_limb_24_col24 = poseidon_3_partial_rounds_chain_input.2[2].get_m31(2);
                *row[24] = input_limb_24_col24;
                let input_limb_25_col25 = poseidon_3_partial_rounds_chain_input.2[2].get_m31(3);
                *row[25] = input_limb_25_col25;
                let input_limb_26_col26 = poseidon_3_partial_rounds_chain_input.2[2].get_m31(4);
                *row[26] = input_limb_26_col26;
                let input_limb_27_col27 = poseidon_3_partial_rounds_chain_input.2[2].get_m31(5);
                *row[27] = input_limb_27_col27;
                let input_limb_28_col28 = poseidon_3_partial_rounds_chain_input.2[2].get_m31(6);
                *row[28] = input_limb_28_col28;
                let input_limb_29_col29 = poseidon_3_partial_rounds_chain_input.2[2].get_m31(7);
                *row[29] = input_limb_29_col29;
                let input_limb_30_col30 = poseidon_3_partial_rounds_chain_input.2[2].get_m31(8);
                *row[30] = input_limb_30_col30;
                let input_limb_31_col31 = poseidon_3_partial_rounds_chain_input.2[2].get_m31(9);
                *row[31] = input_limb_31_col31;
                let input_limb_32_col32 = poseidon_3_partial_rounds_chain_input.2[3].get_m31(0);
                *row[32] = input_limb_32_col32;
                let input_limb_33_col33 = poseidon_3_partial_rounds_chain_input.2[3].get_m31(1);
                *row[33] = input_limb_33_col33;
                let input_limb_34_col34 = poseidon_3_partial_rounds_chain_input.2[3].get_m31(2);
                *row[34] = input_limb_34_col34;
                let input_limb_35_col35 = poseidon_3_partial_rounds_chain_input.2[3].get_m31(3);
                *row[35] = input_limb_35_col35;
                let input_limb_36_col36 = poseidon_3_partial_rounds_chain_input.2[3].get_m31(4);
                *row[36] = input_limb_36_col36;
                let input_limb_37_col37 = poseidon_3_partial_rounds_chain_input.2[3].get_m31(5);
                *row[37] = input_limb_37_col37;
                let input_limb_38_col38 = poseidon_3_partial_rounds_chain_input.2[3].get_m31(6);
                *row[38] = input_limb_38_col38;
                let input_limb_39_col39 = poseidon_3_partial_rounds_chain_input.2[3].get_m31(7);
                *row[39] = input_limb_39_col39;
                let input_limb_40_col40 = poseidon_3_partial_rounds_chain_input.2[3].get_m31(8);
                *row[40] = input_limb_40_col40;
                let input_limb_41_col41 = poseidon_3_partial_rounds_chain_input.2[3].get_m31(9);
                *row[41] = input_limb_41_col41;
                *sub_component_inputs.poseidon_round_keys[0] = [input_limb_1_col1];
                let poseidon_round_keys_output_tmp_44f04_0 =
                    PackedPoseidonRoundKeys::deduce_output([input_limb_1_col1]);
                let poseidon_round_keys_output_limb_0_col42 =
                    poseidon_round_keys_output_tmp_44f04_0[0].get_m31(0);
                *row[42] = poseidon_round_keys_output_limb_0_col42;
                let poseidon_round_keys_output_limb_1_col43 =
                    poseidon_round_keys_output_tmp_44f04_0[0].get_m31(1);
                *row[43] = poseidon_round_keys_output_limb_1_col43;
                let poseidon_round_keys_output_limb_2_col44 =
                    poseidon_round_keys_output_tmp_44f04_0[0].get_m31(2);
                *row[44] = poseidon_round_keys_output_limb_2_col44;
                let poseidon_round_keys_output_limb_3_col45 =
                    poseidon_round_keys_output_tmp_44f04_0[0].get_m31(3);
                *row[45] = poseidon_round_keys_output_limb_3_col45;
                let poseidon_round_keys_output_limb_4_col46 =
                    poseidon_round_keys_output_tmp_44f04_0[0].get_m31(4);
                *row[46] = poseidon_round_keys_output_limb_4_col46;
                let poseidon_round_keys_output_limb_5_col47 =
                    poseidon_round_keys_output_tmp_44f04_0[0].get_m31(5);
                *row[47] = poseidon_round_keys_output_limb_5_col47;
                let poseidon_round_keys_output_limb_6_col48 =
                    poseidon_round_keys_output_tmp_44f04_0[0].get_m31(6);
                *row[48] = poseidon_round_keys_output_limb_6_col48;
                let poseidon_round_keys_output_limb_7_col49 =
                    poseidon_round_keys_output_tmp_44f04_0[0].get_m31(7);
                *row[49] = poseidon_round_keys_output_limb_7_col49;
                let poseidon_round_keys_output_limb_8_col50 =
                    poseidon_round_keys_output_tmp_44f04_0[0].get_m31(8);
                *row[50] = poseidon_round_keys_output_limb_8_col50;
                let poseidon_round_keys_output_limb_9_col51 =
                    poseidon_round_keys_output_tmp_44f04_0[0].get_m31(9);
                *row[51] = poseidon_round_keys_output_limb_9_col51;
                let poseidon_round_keys_output_limb_10_col52 =
                    poseidon_round_keys_output_tmp_44f04_0[1].get_m31(0);
                *row[52] = poseidon_round_keys_output_limb_10_col52;
                let poseidon_round_keys_output_limb_11_col53 =
                    poseidon_round_keys_output_tmp_44f04_0[1].get_m31(1);
                *row[53] = poseidon_round_keys_output_limb_11_col53;
                let poseidon_round_keys_output_limb_12_col54 =
                    poseidon_round_keys_output_tmp_44f04_0[1].get_m31(2);
                *row[54] = poseidon_round_keys_output_limb_12_col54;
                let poseidon_round_keys_output_limb_13_col55 =
                    poseidon_round_keys_output_tmp_44f04_0[1].get_m31(3);
                *row[55] = poseidon_round_keys_output_limb_13_col55;
                let poseidon_round_keys_output_limb_14_col56 =
                    poseidon_round_keys_output_tmp_44f04_0[1].get_m31(4);
                *row[56] = poseidon_round_keys_output_limb_14_col56;
                let poseidon_round_keys_output_limb_15_col57 =
                    poseidon_round_keys_output_tmp_44f04_0[1].get_m31(5);
                *row[57] = poseidon_round_keys_output_limb_15_col57;
                let poseidon_round_keys_output_limb_16_col58 =
                    poseidon_round_keys_output_tmp_44f04_0[1].get_m31(6);
                *row[58] = poseidon_round_keys_output_limb_16_col58;
                let poseidon_round_keys_output_limb_17_col59 =
                    poseidon_round_keys_output_tmp_44f04_0[1].get_m31(7);
                *row[59] = poseidon_round_keys_output_limb_17_col59;
                let poseidon_round_keys_output_limb_18_col60 =
                    poseidon_round_keys_output_tmp_44f04_0[1].get_m31(8);
                *row[60] = poseidon_round_keys_output_limb_18_col60;
                let poseidon_round_keys_output_limb_19_col61 =
                    poseidon_round_keys_output_tmp_44f04_0[1].get_m31(9);
                *row[61] = poseidon_round_keys_output_limb_19_col61;
                let poseidon_round_keys_output_limb_20_col62 =
                    poseidon_round_keys_output_tmp_44f04_0[2].get_m31(0);
                *row[62] = poseidon_round_keys_output_limb_20_col62;
                let poseidon_round_keys_output_limb_21_col63 =
                    poseidon_round_keys_output_tmp_44f04_0[2].get_m31(1);
                *row[63] = poseidon_round_keys_output_limb_21_col63;
                let poseidon_round_keys_output_limb_22_col64 =
                    poseidon_round_keys_output_tmp_44f04_0[2].get_m31(2);
                *row[64] = poseidon_round_keys_output_limb_22_col64;
                let poseidon_round_keys_output_limb_23_col65 =
                    poseidon_round_keys_output_tmp_44f04_0[2].get_m31(3);
                *row[65] = poseidon_round_keys_output_limb_23_col65;
                let poseidon_round_keys_output_limb_24_col66 =
                    poseidon_round_keys_output_tmp_44f04_0[2].get_m31(4);
                *row[66] = poseidon_round_keys_output_limb_24_col66;
                let poseidon_round_keys_output_limb_25_col67 =
                    poseidon_round_keys_output_tmp_44f04_0[2].get_m31(5);
                *row[67] = poseidon_round_keys_output_limb_25_col67;
                let poseidon_round_keys_output_limb_26_col68 =
                    poseidon_round_keys_output_tmp_44f04_0[2].get_m31(6);
                *row[68] = poseidon_round_keys_output_limb_26_col68;
                let poseidon_round_keys_output_limb_27_col69 =
                    poseidon_round_keys_output_tmp_44f04_0[2].get_m31(7);
                *row[69] = poseidon_round_keys_output_limb_27_col69;
                let poseidon_round_keys_output_limb_28_col70 =
                    poseidon_round_keys_output_tmp_44f04_0[2].get_m31(8);
                *row[70] = poseidon_round_keys_output_limb_28_col70;
                let poseidon_round_keys_output_limb_29_col71 =
                    poseidon_round_keys_output_tmp_44f04_0[2].get_m31(9);
                *row[71] = poseidon_round_keys_output_limb_29_col71;
                *lookup_data.poseidon_round_keys_0 = [
                    input_limb_1_col1,
                    poseidon_round_keys_output_limb_0_col42,
                    poseidon_round_keys_output_limb_1_col43,
                    poseidon_round_keys_output_limb_2_col44,
                    poseidon_round_keys_output_limb_3_col45,
                    poseidon_round_keys_output_limb_4_col46,
                    poseidon_round_keys_output_limb_5_col47,
                    poseidon_round_keys_output_limb_6_col48,
                    poseidon_round_keys_output_limb_7_col49,
                    poseidon_round_keys_output_limb_8_col50,
                    poseidon_round_keys_output_limb_9_col51,
                    poseidon_round_keys_output_limb_10_col52,
                    poseidon_round_keys_output_limb_11_col53,
                    poseidon_round_keys_output_limb_12_col54,
                    poseidon_round_keys_output_limb_13_col55,
                    poseidon_round_keys_output_limb_14_col56,
                    poseidon_round_keys_output_limb_15_col57,
                    poseidon_round_keys_output_limb_16_col58,
                    poseidon_round_keys_output_limb_17_col59,
                    poseidon_round_keys_output_limb_18_col60,
                    poseidon_round_keys_output_limb_19_col61,
                    poseidon_round_keys_output_limb_20_col62,
                    poseidon_round_keys_output_limb_21_col63,
                    poseidon_round_keys_output_limb_22_col64,
                    poseidon_round_keys_output_limb_23_col65,
                    poseidon_round_keys_output_limb_24_col66,
                    poseidon_round_keys_output_limb_25_col67,
                    poseidon_round_keys_output_limb_26_col68,
                    poseidon_round_keys_output_limb_27_col69,
                    poseidon_round_keys_output_limb_28_col70,
                    poseidon_round_keys_output_limb_29_col71,
                ];

                // Poseidon Partial Round.

                *sub_component_inputs.cube_252[0] = poseidon_3_partial_rounds_chain_input.2[3];
                let cube_252_output_tmp_44f04_1 =
                    PackedCube252::deduce_output(poseidon_3_partial_rounds_chain_input.2[3]);
                let cube_252_output_limb_0_col72 = cube_252_output_tmp_44f04_1.get_m31(0);
                *row[72] = cube_252_output_limb_0_col72;
                let cube_252_output_limb_1_col73 = cube_252_output_tmp_44f04_1.get_m31(1);
                *row[73] = cube_252_output_limb_1_col73;
                let cube_252_output_limb_2_col74 = cube_252_output_tmp_44f04_1.get_m31(2);
                *row[74] = cube_252_output_limb_2_col74;
                let cube_252_output_limb_3_col75 = cube_252_output_tmp_44f04_1.get_m31(3);
                *row[75] = cube_252_output_limb_3_col75;
                let cube_252_output_limb_4_col76 = cube_252_output_tmp_44f04_1.get_m31(4);
                *row[76] = cube_252_output_limb_4_col76;
                let cube_252_output_limb_5_col77 = cube_252_output_tmp_44f04_1.get_m31(5);
                *row[77] = cube_252_output_limb_5_col77;
                let cube_252_output_limb_6_col78 = cube_252_output_tmp_44f04_1.get_m31(6);
                *row[78] = cube_252_output_limb_6_col78;
                let cube_252_output_limb_7_col79 = cube_252_output_tmp_44f04_1.get_m31(7);
                *row[79] = cube_252_output_limb_7_col79;
                let cube_252_output_limb_8_col80 = cube_252_output_tmp_44f04_1.get_m31(8);
                *row[80] = cube_252_output_limb_8_col80;
                let cube_252_output_limb_9_col81 = cube_252_output_tmp_44f04_1.get_m31(9);
                *row[81] = cube_252_output_limb_9_col81;
                *lookup_data.cube_252_0 = [
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
                    cube_252_output_limb_0_col72,
                    cube_252_output_limb_1_col73,
                    cube_252_output_limb_2_col74,
                    cube_252_output_limb_3_col75,
                    cube_252_output_limb_4_col76,
                    cube_252_output_limb_5_col77,
                    cube_252_output_limb_6_col78,
                    cube_252_output_limb_7_col79,
                    cube_252_output_limb_8_col80,
                    cube_252_output_limb_9_col81,
                ];

                // Linear Combination N 6 Coefs 4 2 3 1 M 1 1.

                let combination_tmp_44f04_2 = PackedFelt252Width27::from_packed_felt252(
                    (((((((Felt252_0_0_0_0)
                        + ((Felt252_4_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_3_partial_rounds_chain_input.2[0],
                            ))))
                        + ((Felt252_2_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_3_partial_rounds_chain_input.2[1],
                            ))))
                        + ((Felt252_3_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_3_partial_rounds_chain_input.2[2],
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_3_partial_rounds_chain_input.2[3],
                            ))))
                        - ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_44f04_1,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_round_keys_output_tmp_44f04_0[0],
                            )))),
                );
                let combination_limb_0_col82 = combination_tmp_44f04_2.get_m31(0);
                *row[82] = combination_limb_0_col82;
                let combination_limb_1_col83 = combination_tmp_44f04_2.get_m31(1);
                *row[83] = combination_limb_1_col83;
                let combination_limb_2_col84 = combination_tmp_44f04_2.get_m31(2);
                *row[84] = combination_limb_2_col84;
                let combination_limb_3_col85 = combination_tmp_44f04_2.get_m31(3);
                *row[85] = combination_limb_3_col85;
                let combination_limb_4_col86 = combination_tmp_44f04_2.get_m31(4);
                *row[86] = combination_limb_4_col86;
                let combination_limb_5_col87 = combination_tmp_44f04_2.get_m31(5);
                *row[87] = combination_limb_5_col87;
                let combination_limb_6_col88 = combination_tmp_44f04_2.get_m31(6);
                *row[88] = combination_limb_6_col88;
                let combination_limb_7_col89 = combination_tmp_44f04_2.get_m31(7);
                *row[89] = combination_limb_7_col89;
                let combination_limb_8_col90 = combination_tmp_44f04_2.get_m31(8);
                *row[90] = combination_limb_8_col90;
                let combination_limb_9_col91 = combination_tmp_44f04_2.get_m31(9);
                *row[91] = combination_limb_9_col91;
                let biased_limb_accumulator_u32_tmp_44f04_3 = PackedUInt32::from_m31(
                    (((((((((M31_4) * (input_limb_2_col2))
                        + ((M31_2) * (input_limb_12_col12)))
                        + ((M31_3) * (input_limb_22_col22)))
                        + (input_limb_32_col32))
                        - (cube_252_output_limb_0_col72))
                        + (poseidon_round_keys_output_limb_0_col42))
                        - (combination_limb_0_col82))
                        + (M31_268435458)),
                );
                let p_coef_col92 =
                    ((biased_limb_accumulator_u32_tmp_44f04_3.low().as_m31()) - (M31_2));
                *row[92] = p_coef_col92;
                let carry_0_tmp_44f04_4 = ((((((((((M31_4) * (input_limb_2_col2))
                    + ((M31_2) * (input_limb_12_col12)))
                    + ((M31_3) * (input_limb_22_col22)))
                    + (input_limb_32_col32))
                    - (cube_252_output_limb_0_col72))
                    + (poseidon_round_keys_output_limb_0_col42))
                    - (combination_limb_0_col82))
                    - (p_coef_col92))
                    * (M31_16));
                let carry_1_tmp_44f04_5 = (((((((((carry_0_tmp_44f04_4)
                    + ((M31_4) * (input_limb_3_col3)))
                    + ((M31_2) * (input_limb_13_col13)))
                    + ((M31_3) * (input_limb_23_col23)))
                    + (input_limb_33_col33))
                    - (cube_252_output_limb_1_col73))
                    + (poseidon_round_keys_output_limb_1_col43))
                    - (combination_limb_1_col83))
                    * (M31_16));
                let carry_2_tmp_44f04_6 = (((((((((carry_1_tmp_44f04_5)
                    + ((M31_4) * (input_limb_4_col4)))
                    + ((M31_2) * (input_limb_14_col14)))
                    + ((M31_3) * (input_limb_24_col24)))
                    + (input_limb_34_col34))
                    - (cube_252_output_limb_2_col74))
                    + (poseidon_round_keys_output_limb_2_col44))
                    - (combination_limb_2_col84))
                    * (M31_16));
                let carry_3_tmp_44f04_7 = (((((((((carry_2_tmp_44f04_6)
                    + ((M31_4) * (input_limb_5_col5)))
                    + ((M31_2) * (input_limb_15_col15)))
                    + ((M31_3) * (input_limb_25_col25)))
                    + (input_limb_35_col35))
                    - (cube_252_output_limb_3_col75))
                    + (poseidon_round_keys_output_limb_3_col45))
                    - (combination_limb_3_col85))
                    * (M31_16));
                let carry_4_tmp_44f04_8 = (((((((((carry_3_tmp_44f04_7)
                    + ((M31_4) * (input_limb_6_col6)))
                    + ((M31_2) * (input_limb_16_col16)))
                    + ((M31_3) * (input_limb_26_col26)))
                    + (input_limb_36_col36))
                    - (cube_252_output_limb_4_col76))
                    + (poseidon_round_keys_output_limb_4_col46))
                    - (combination_limb_4_col86))
                    * (M31_16));
                let carry_5_tmp_44f04_9 = (((((((((carry_4_tmp_44f04_8)
                    + ((M31_4) * (input_limb_7_col7)))
                    + ((M31_2) * (input_limb_17_col17)))
                    + ((M31_3) * (input_limb_27_col27)))
                    + (input_limb_37_col37))
                    - (cube_252_output_limb_5_col77))
                    + (poseidon_round_keys_output_limb_5_col47))
                    - (combination_limb_5_col87))
                    * (M31_16));
                let carry_6_tmp_44f04_10 = (((((((((carry_5_tmp_44f04_9)
                    + ((M31_4) * (input_limb_8_col8)))
                    + ((M31_2) * (input_limb_18_col18)))
                    + ((M31_3) * (input_limb_28_col28)))
                    + (input_limb_38_col38))
                    - (cube_252_output_limb_6_col78))
                    + (poseidon_round_keys_output_limb_6_col48))
                    - (combination_limb_6_col88))
                    * (M31_16));
                let carry_7_tmp_44f04_11 = ((((((((((carry_6_tmp_44f04_10)
                    + ((M31_4) * (input_limb_9_col9)))
                    + ((M31_2) * (input_limb_19_col19)))
                    + ((M31_3) * (input_limb_29_col29)))
                    + (input_limb_39_col39))
                    - (cube_252_output_limb_7_col79))
                    + (poseidon_round_keys_output_limb_7_col49))
                    - (combination_limb_7_col89))
                    - ((p_coef_col92) * (M31_136)))
                    * (M31_16));
                let carry_8_tmp_44f04_12 = (((((((((carry_7_tmp_44f04_11)
                    + ((M31_4) * (input_limb_10_col10)))
                    + ((M31_2) * (input_limb_20_col20)))
                    + ((M31_3) * (input_limb_30_col30)))
                    + (input_limb_40_col40))
                    - (cube_252_output_limb_8_col80))
                    + (poseidon_round_keys_output_limb_8_col50))
                    - (combination_limb_8_col90))
                    * (M31_16));
                *sub_component_inputs.range_check_4_4_4_4[0] = [
                    ((p_coef_col92) + (M31_2)),
                    ((carry_0_tmp_44f04_4) + (M31_2)),
                    ((carry_1_tmp_44f04_5) + (M31_2)),
                    ((carry_2_tmp_44f04_6) + (M31_2)),
                ];
                *lookup_data.range_check_4_4_4_4_0 = [
                    ((p_coef_col92) + (M31_2)),
                    ((carry_0_tmp_44f04_4) + (M31_2)),
                    ((carry_1_tmp_44f04_5) + (M31_2)),
                    ((carry_2_tmp_44f04_6) + (M31_2)),
                ];
                *sub_component_inputs.range_check_4_4_4_4[1] = [
                    ((carry_3_tmp_44f04_7) + (M31_2)),
                    ((carry_4_tmp_44f04_8) + (M31_2)),
                    ((carry_5_tmp_44f04_9) + (M31_2)),
                    ((carry_6_tmp_44f04_10) + (M31_2)),
                ];
                *lookup_data.range_check_4_4_4_4_1 = [
                    ((carry_3_tmp_44f04_7) + (M31_2)),
                    ((carry_4_tmp_44f04_8) + (M31_2)),
                    ((carry_5_tmp_44f04_9) + (M31_2)),
                    ((carry_6_tmp_44f04_10) + (M31_2)),
                ];
                *sub_component_inputs.range_check_4_4[0] = [
                    ((carry_7_tmp_44f04_11) + (M31_2)),
                    ((carry_8_tmp_44f04_12) + (M31_2)),
                ];
                *lookup_data.range_check_4_4_0 = [
                    ((carry_7_tmp_44f04_11) + (M31_2)),
                    ((carry_8_tmp_44f04_12) + (M31_2)),
                ];
                let linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_44f04_13 =
                    combination_tmp_44f04_2;

                *sub_component_inputs.range_check_252_width_27[0] =
                    linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_44f04_13;
                *lookup_data.range_check_252_width_27_0 = [
                    combination_limb_0_col82,
                    combination_limb_1_col83,
                    combination_limb_2_col84,
                    combination_limb_3_col85,
                    combination_limb_4_col86,
                    combination_limb_5_col87,
                    combination_limb_6_col88,
                    combination_limb_7_col89,
                    combination_limb_8_col90,
                    combination_limb_9_col91,
                ];

                // Linear Combination N 1 Coefs 2.

                let combination_tmp_44f04_14 = PackedFelt252Width27::from_packed_felt252(
                    ((Felt252_0_0_0_0)
                        + ((Felt252_2_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_44f04_13,
                            )))),
                );
                let combination_limb_0_col93 = combination_tmp_44f04_14.get_m31(0);
                *row[93] = combination_limb_0_col93;
                let combination_limb_1_col94 = combination_tmp_44f04_14.get_m31(1);
                *row[94] = combination_limb_1_col94;
                let combination_limb_2_col95 = combination_tmp_44f04_14.get_m31(2);
                *row[95] = combination_limb_2_col95;
                let combination_limb_3_col96 = combination_tmp_44f04_14.get_m31(3);
                *row[96] = combination_limb_3_col96;
                let combination_limb_4_col97 = combination_tmp_44f04_14.get_m31(4);
                *row[97] = combination_limb_4_col97;
                let combination_limb_5_col98 = combination_tmp_44f04_14.get_m31(5);
                *row[98] = combination_limb_5_col98;
                let combination_limb_6_col99 = combination_tmp_44f04_14.get_m31(6);
                *row[99] = combination_limb_6_col99;
                let combination_limb_7_col100 = combination_tmp_44f04_14.get_m31(7);
                *row[100] = combination_limb_7_col100;
                let combination_limb_8_col101 = combination_tmp_44f04_14.get_m31(8);
                *row[101] = combination_limb_8_col101;
                let combination_limb_9_col102 = combination_tmp_44f04_14.get_m31(9);
                *row[102] = combination_limb_9_col102;
                let biased_limb_accumulator_u32_tmp_44f04_15 = PackedUInt32::from_m31(
                    ((((M31_2) * (combination_limb_0_col82)) - (combination_limb_0_col93))
                        + (M31_134217729)),
                );
                let p_coef_col103 =
                    ((biased_limb_accumulator_u32_tmp_44f04_15.low().as_m31()) - (M31_1));
                *row[103] = p_coef_col103;
                let carry_0_tmp_44f04_16 = (((((M31_2) * (combination_limb_0_col82))
                    - (combination_limb_0_col93))
                    - (p_coef_col103))
                    * (M31_16));
                let carry_1_tmp_44f04_17 = ((((carry_0_tmp_44f04_16)
                    + ((M31_2) * (combination_limb_1_col83)))
                    - (combination_limb_1_col94))
                    * (M31_16));
                let carry_2_tmp_44f04_18 = ((((carry_1_tmp_44f04_17)
                    + ((M31_2) * (combination_limb_2_col84)))
                    - (combination_limb_2_col95))
                    * (M31_16));
                let carry_3_tmp_44f04_19 = ((((carry_2_tmp_44f04_18)
                    + ((M31_2) * (combination_limb_3_col85)))
                    - (combination_limb_3_col96))
                    * (M31_16));
                let carry_4_tmp_44f04_20 = ((((carry_3_tmp_44f04_19)
                    + ((M31_2) * (combination_limb_4_col86)))
                    - (combination_limb_4_col97))
                    * (M31_16));
                let carry_5_tmp_44f04_21 = ((((carry_4_tmp_44f04_20)
                    + ((M31_2) * (combination_limb_5_col87)))
                    - (combination_limb_5_col98))
                    * (M31_16));
                let carry_6_tmp_44f04_22 = ((((carry_5_tmp_44f04_21)
                    + ((M31_2) * (combination_limb_6_col88)))
                    - (combination_limb_6_col99))
                    * (M31_16));
                let carry_7_tmp_44f04_23 = (((((carry_6_tmp_44f04_22)
                    + ((M31_2) * (combination_limb_7_col89)))
                    - (combination_limb_7_col100))
                    - ((p_coef_col103) * (M31_136)))
                    * (M31_16));
                let carry_8_tmp_44f04_24 = ((((carry_7_tmp_44f04_23)
                    + ((M31_2) * (combination_limb_8_col90)))
                    - (combination_limb_8_col101))
                    * (M31_16));
                let linear_combination_n_1_coefs_2_output_tmp_44f04_34 = combination_tmp_44f04_14;

                let poseidon_partial_round_output_tmp_44f04_35 = [
                    cube_252_output_tmp_44f04_1,
                    linear_combination_n_1_coefs_2_output_tmp_44f04_34,
                ];

                // Poseidon Partial Round.

                *sub_component_inputs.cube_252[1] = poseidon_partial_round_output_tmp_44f04_35[1];
                let cube_252_output_tmp_44f04_36 =
                    PackedCube252::deduce_output(poseidon_partial_round_output_tmp_44f04_35[1]);
                let cube_252_output_limb_0_col104 = cube_252_output_tmp_44f04_36.get_m31(0);
                *row[104] = cube_252_output_limb_0_col104;
                let cube_252_output_limb_1_col105 = cube_252_output_tmp_44f04_36.get_m31(1);
                *row[105] = cube_252_output_limb_1_col105;
                let cube_252_output_limb_2_col106 = cube_252_output_tmp_44f04_36.get_m31(2);
                *row[106] = cube_252_output_limb_2_col106;
                let cube_252_output_limb_3_col107 = cube_252_output_tmp_44f04_36.get_m31(3);
                *row[107] = cube_252_output_limb_3_col107;
                let cube_252_output_limb_4_col108 = cube_252_output_tmp_44f04_36.get_m31(4);
                *row[108] = cube_252_output_limb_4_col108;
                let cube_252_output_limb_5_col109 = cube_252_output_tmp_44f04_36.get_m31(5);
                *row[109] = cube_252_output_limb_5_col109;
                let cube_252_output_limb_6_col110 = cube_252_output_tmp_44f04_36.get_m31(6);
                *row[110] = cube_252_output_limb_6_col110;
                let cube_252_output_limb_7_col111 = cube_252_output_tmp_44f04_36.get_m31(7);
                *row[111] = cube_252_output_limb_7_col111;
                let cube_252_output_limb_8_col112 = cube_252_output_tmp_44f04_36.get_m31(8);
                *row[112] = cube_252_output_limb_8_col112;
                let cube_252_output_limb_9_col113 = cube_252_output_tmp_44f04_36.get_m31(9);
                *row[113] = cube_252_output_limb_9_col113;
                *lookup_data.cube_252_1 = [
                    combination_limb_0_col93,
                    combination_limb_1_col94,
                    combination_limb_2_col95,
                    combination_limb_3_col96,
                    combination_limb_4_col97,
                    combination_limb_5_col98,
                    combination_limb_6_col99,
                    combination_limb_7_col100,
                    combination_limb_8_col101,
                    combination_limb_9_col102,
                    cube_252_output_limb_0_col104,
                    cube_252_output_limb_1_col105,
                    cube_252_output_limb_2_col106,
                    cube_252_output_limb_3_col107,
                    cube_252_output_limb_4_col108,
                    cube_252_output_limb_5_col109,
                    cube_252_output_limb_6_col110,
                    cube_252_output_limb_7_col111,
                    cube_252_output_limb_8_col112,
                    cube_252_output_limb_9_col113,
                ];

                // Linear Combination N 6 Coefs 4 2 3 1 M 1 1.

                let combination_tmp_44f04_37 = PackedFelt252Width27::from_packed_felt252(
                    (((((((Felt252_0_0_0_0)
                        + ((Felt252_4_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_3_partial_rounds_chain_input.2[2],
                            ))))
                        + ((Felt252_2_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_3_partial_rounds_chain_input.2[3],
                            ))))
                        + ((Felt252_3_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_partial_round_output_tmp_44f04_35[0],
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_partial_round_output_tmp_44f04_35[1],
                            ))))
                        - ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_44f04_36,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_round_keys_output_tmp_44f04_0[1],
                            )))),
                );
                let combination_limb_0_col114 = combination_tmp_44f04_37.get_m31(0);
                *row[114] = combination_limb_0_col114;
                let combination_limb_1_col115 = combination_tmp_44f04_37.get_m31(1);
                *row[115] = combination_limb_1_col115;
                let combination_limb_2_col116 = combination_tmp_44f04_37.get_m31(2);
                *row[116] = combination_limb_2_col116;
                let combination_limb_3_col117 = combination_tmp_44f04_37.get_m31(3);
                *row[117] = combination_limb_3_col117;
                let combination_limb_4_col118 = combination_tmp_44f04_37.get_m31(4);
                *row[118] = combination_limb_4_col118;
                let combination_limb_5_col119 = combination_tmp_44f04_37.get_m31(5);
                *row[119] = combination_limb_5_col119;
                let combination_limb_6_col120 = combination_tmp_44f04_37.get_m31(6);
                *row[120] = combination_limb_6_col120;
                let combination_limb_7_col121 = combination_tmp_44f04_37.get_m31(7);
                *row[121] = combination_limb_7_col121;
                let combination_limb_8_col122 = combination_tmp_44f04_37.get_m31(8);
                *row[122] = combination_limb_8_col122;
                let combination_limb_9_col123 = combination_tmp_44f04_37.get_m31(9);
                *row[123] = combination_limb_9_col123;
                let biased_limb_accumulator_u32_tmp_44f04_38 = PackedUInt32::from_m31(
                    (((((((((M31_4) * (input_limb_22_col22))
                        + ((M31_2) * (input_limb_32_col32)))
                        + ((M31_3) * (cube_252_output_limb_0_col72)))
                        + (combination_limb_0_col93))
                        - (cube_252_output_limb_0_col104))
                        + (poseidon_round_keys_output_limb_10_col52))
                        - (combination_limb_0_col114))
                        + (M31_268435458)),
                );
                let p_coef_col124 =
                    ((biased_limb_accumulator_u32_tmp_44f04_38.low().as_m31()) - (M31_2));
                *row[124] = p_coef_col124;
                let carry_0_tmp_44f04_39 = ((((((((((M31_4) * (input_limb_22_col22))
                    + ((M31_2) * (input_limb_32_col32)))
                    + ((M31_3) * (cube_252_output_limb_0_col72)))
                    + (combination_limb_0_col93))
                    - (cube_252_output_limb_0_col104))
                    + (poseidon_round_keys_output_limb_10_col52))
                    - (combination_limb_0_col114))
                    - (p_coef_col124))
                    * (M31_16));
                let carry_1_tmp_44f04_40 = (((((((((carry_0_tmp_44f04_39)
                    + ((M31_4) * (input_limb_23_col23)))
                    + ((M31_2) * (input_limb_33_col33)))
                    + ((M31_3) * (cube_252_output_limb_1_col73)))
                    + (combination_limb_1_col94))
                    - (cube_252_output_limb_1_col105))
                    + (poseidon_round_keys_output_limb_11_col53))
                    - (combination_limb_1_col115))
                    * (M31_16));
                let carry_2_tmp_44f04_41 = (((((((((carry_1_tmp_44f04_40)
                    + ((M31_4) * (input_limb_24_col24)))
                    + ((M31_2) * (input_limb_34_col34)))
                    + ((M31_3) * (cube_252_output_limb_2_col74)))
                    + (combination_limb_2_col95))
                    - (cube_252_output_limb_2_col106))
                    + (poseidon_round_keys_output_limb_12_col54))
                    - (combination_limb_2_col116))
                    * (M31_16));
                let carry_3_tmp_44f04_42 = (((((((((carry_2_tmp_44f04_41)
                    + ((M31_4) * (input_limb_25_col25)))
                    + ((M31_2) * (input_limb_35_col35)))
                    + ((M31_3) * (cube_252_output_limb_3_col75)))
                    + (combination_limb_3_col96))
                    - (cube_252_output_limb_3_col107))
                    + (poseidon_round_keys_output_limb_13_col55))
                    - (combination_limb_3_col117))
                    * (M31_16));
                let carry_4_tmp_44f04_43 = (((((((((carry_3_tmp_44f04_42)
                    + ((M31_4) * (input_limb_26_col26)))
                    + ((M31_2) * (input_limb_36_col36)))
                    + ((M31_3) * (cube_252_output_limb_4_col76)))
                    + (combination_limb_4_col97))
                    - (cube_252_output_limb_4_col108))
                    + (poseidon_round_keys_output_limb_14_col56))
                    - (combination_limb_4_col118))
                    * (M31_16));
                let carry_5_tmp_44f04_44 = (((((((((carry_4_tmp_44f04_43)
                    + ((M31_4) * (input_limb_27_col27)))
                    + ((M31_2) * (input_limb_37_col37)))
                    + ((M31_3) * (cube_252_output_limb_5_col77)))
                    + (combination_limb_5_col98))
                    - (cube_252_output_limb_5_col109))
                    + (poseidon_round_keys_output_limb_15_col57))
                    - (combination_limb_5_col119))
                    * (M31_16));
                let carry_6_tmp_44f04_45 = (((((((((carry_5_tmp_44f04_44)
                    + ((M31_4) * (input_limb_28_col28)))
                    + ((M31_2) * (input_limb_38_col38)))
                    + ((M31_3) * (cube_252_output_limb_6_col78)))
                    + (combination_limb_6_col99))
                    - (cube_252_output_limb_6_col110))
                    + (poseidon_round_keys_output_limb_16_col58))
                    - (combination_limb_6_col120))
                    * (M31_16));
                let carry_7_tmp_44f04_46 = ((((((((((carry_6_tmp_44f04_45)
                    + ((M31_4) * (input_limb_29_col29)))
                    + ((M31_2) * (input_limb_39_col39)))
                    + ((M31_3) * (cube_252_output_limb_7_col79)))
                    + (combination_limb_7_col100))
                    - (cube_252_output_limb_7_col111))
                    + (poseidon_round_keys_output_limb_17_col59))
                    - (combination_limb_7_col121))
                    - ((p_coef_col124) * (M31_136)))
                    * (M31_16));
                let carry_8_tmp_44f04_47 = (((((((((carry_7_tmp_44f04_46)
                    + ((M31_4) * (input_limb_30_col30)))
                    + ((M31_2) * (input_limb_40_col40)))
                    + ((M31_3) * (cube_252_output_limb_8_col80)))
                    + (combination_limb_8_col101))
                    - (cube_252_output_limb_8_col112))
                    + (poseidon_round_keys_output_limb_18_col60))
                    - (combination_limb_8_col122))
                    * (M31_16));
                *sub_component_inputs.range_check_4_4_4_4[2] = [
                    ((p_coef_col124) + (M31_2)),
                    ((carry_0_tmp_44f04_39) + (M31_2)),
                    ((carry_1_tmp_44f04_40) + (M31_2)),
                    ((carry_2_tmp_44f04_41) + (M31_2)),
                ];
                *lookup_data.range_check_4_4_4_4_2 = [
                    ((p_coef_col124) + (M31_2)),
                    ((carry_0_tmp_44f04_39) + (M31_2)),
                    ((carry_1_tmp_44f04_40) + (M31_2)),
                    ((carry_2_tmp_44f04_41) + (M31_2)),
                ];
                *sub_component_inputs.range_check_4_4_4_4[3] = [
                    ((carry_3_tmp_44f04_42) + (M31_2)),
                    ((carry_4_tmp_44f04_43) + (M31_2)),
                    ((carry_5_tmp_44f04_44) + (M31_2)),
                    ((carry_6_tmp_44f04_45) + (M31_2)),
                ];
                *lookup_data.range_check_4_4_4_4_3 = [
                    ((carry_3_tmp_44f04_42) + (M31_2)),
                    ((carry_4_tmp_44f04_43) + (M31_2)),
                    ((carry_5_tmp_44f04_44) + (M31_2)),
                    ((carry_6_tmp_44f04_45) + (M31_2)),
                ];
                *sub_component_inputs.range_check_4_4[1] = [
                    ((carry_7_tmp_44f04_46) + (M31_2)),
                    ((carry_8_tmp_44f04_47) + (M31_2)),
                ];
                *lookup_data.range_check_4_4_1 = [
                    ((carry_7_tmp_44f04_46) + (M31_2)),
                    ((carry_8_tmp_44f04_47) + (M31_2)),
                ];
                let linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_44f04_48 =
                    combination_tmp_44f04_37;

                *sub_component_inputs.range_check_252_width_27[1] =
                    linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_44f04_48;
                *lookup_data.range_check_252_width_27_1 = [
                    combination_limb_0_col114,
                    combination_limb_1_col115,
                    combination_limb_2_col116,
                    combination_limb_3_col117,
                    combination_limb_4_col118,
                    combination_limb_5_col119,
                    combination_limb_6_col120,
                    combination_limb_7_col121,
                    combination_limb_8_col122,
                    combination_limb_9_col123,
                ];

                // Linear Combination N 1 Coefs 2.

                let combination_tmp_44f04_49 = PackedFelt252Width27::from_packed_felt252(
                    ((Felt252_0_0_0_0)
                        + ((Felt252_2_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_44f04_48,
                            )))),
                );
                let combination_limb_0_col125 = combination_tmp_44f04_49.get_m31(0);
                *row[125] = combination_limb_0_col125;
                let combination_limb_1_col126 = combination_tmp_44f04_49.get_m31(1);
                *row[126] = combination_limb_1_col126;
                let combination_limb_2_col127 = combination_tmp_44f04_49.get_m31(2);
                *row[127] = combination_limb_2_col127;
                let combination_limb_3_col128 = combination_tmp_44f04_49.get_m31(3);
                *row[128] = combination_limb_3_col128;
                let combination_limb_4_col129 = combination_tmp_44f04_49.get_m31(4);
                *row[129] = combination_limb_4_col129;
                let combination_limb_5_col130 = combination_tmp_44f04_49.get_m31(5);
                *row[130] = combination_limb_5_col130;
                let combination_limb_6_col131 = combination_tmp_44f04_49.get_m31(6);
                *row[131] = combination_limb_6_col131;
                let combination_limb_7_col132 = combination_tmp_44f04_49.get_m31(7);
                *row[132] = combination_limb_7_col132;
                let combination_limb_8_col133 = combination_tmp_44f04_49.get_m31(8);
                *row[133] = combination_limb_8_col133;
                let combination_limb_9_col134 = combination_tmp_44f04_49.get_m31(9);
                *row[134] = combination_limb_9_col134;
                let biased_limb_accumulator_u32_tmp_44f04_50 = PackedUInt32::from_m31(
                    ((((M31_2) * (combination_limb_0_col114)) - (combination_limb_0_col125))
                        + (M31_134217729)),
                );
                let p_coef_col135 =
                    ((biased_limb_accumulator_u32_tmp_44f04_50.low().as_m31()) - (M31_1));
                *row[135] = p_coef_col135;
                let carry_0_tmp_44f04_51 = (((((M31_2) * (combination_limb_0_col114))
                    - (combination_limb_0_col125))
                    - (p_coef_col135))
                    * (M31_16));
                let carry_1_tmp_44f04_52 = ((((carry_0_tmp_44f04_51)
                    + ((M31_2) * (combination_limb_1_col115)))
                    - (combination_limb_1_col126))
                    * (M31_16));
                let carry_2_tmp_44f04_53 = ((((carry_1_tmp_44f04_52)
                    + ((M31_2) * (combination_limb_2_col116)))
                    - (combination_limb_2_col127))
                    * (M31_16));
                let carry_3_tmp_44f04_54 = ((((carry_2_tmp_44f04_53)
                    + ((M31_2) * (combination_limb_3_col117)))
                    - (combination_limb_3_col128))
                    * (M31_16));
                let carry_4_tmp_44f04_55 = ((((carry_3_tmp_44f04_54)
                    + ((M31_2) * (combination_limb_4_col118)))
                    - (combination_limb_4_col129))
                    * (M31_16));
                let carry_5_tmp_44f04_56 = ((((carry_4_tmp_44f04_55)
                    + ((M31_2) * (combination_limb_5_col119)))
                    - (combination_limb_5_col130))
                    * (M31_16));
                let carry_6_tmp_44f04_57 = ((((carry_5_tmp_44f04_56)
                    + ((M31_2) * (combination_limb_6_col120)))
                    - (combination_limb_6_col131))
                    * (M31_16));
                let carry_7_tmp_44f04_58 = (((((carry_6_tmp_44f04_57)
                    + ((M31_2) * (combination_limb_7_col121)))
                    - (combination_limb_7_col132))
                    - ((p_coef_col135) * (M31_136)))
                    * (M31_16));
                let carry_8_tmp_44f04_59 = ((((carry_7_tmp_44f04_58)
                    + ((M31_2) * (combination_limb_8_col122)))
                    - (combination_limb_8_col133))
                    * (M31_16));
                let linear_combination_n_1_coefs_2_output_tmp_44f04_69 = combination_tmp_44f04_49;

                let poseidon_partial_round_output_tmp_44f04_70 = [
                    cube_252_output_tmp_44f04_36,
                    linear_combination_n_1_coefs_2_output_tmp_44f04_69,
                ];

                // Poseidon Partial Round.

                *sub_component_inputs.cube_252[2] = poseidon_partial_round_output_tmp_44f04_70[1];
                let cube_252_output_tmp_44f04_71 =
                    PackedCube252::deduce_output(poseidon_partial_round_output_tmp_44f04_70[1]);
                let cube_252_output_limb_0_col136 = cube_252_output_tmp_44f04_71.get_m31(0);
                *row[136] = cube_252_output_limb_0_col136;
                let cube_252_output_limb_1_col137 = cube_252_output_tmp_44f04_71.get_m31(1);
                *row[137] = cube_252_output_limb_1_col137;
                let cube_252_output_limb_2_col138 = cube_252_output_tmp_44f04_71.get_m31(2);
                *row[138] = cube_252_output_limb_2_col138;
                let cube_252_output_limb_3_col139 = cube_252_output_tmp_44f04_71.get_m31(3);
                *row[139] = cube_252_output_limb_3_col139;
                let cube_252_output_limb_4_col140 = cube_252_output_tmp_44f04_71.get_m31(4);
                *row[140] = cube_252_output_limb_4_col140;
                let cube_252_output_limb_5_col141 = cube_252_output_tmp_44f04_71.get_m31(5);
                *row[141] = cube_252_output_limb_5_col141;
                let cube_252_output_limb_6_col142 = cube_252_output_tmp_44f04_71.get_m31(6);
                *row[142] = cube_252_output_limb_6_col142;
                let cube_252_output_limb_7_col143 = cube_252_output_tmp_44f04_71.get_m31(7);
                *row[143] = cube_252_output_limb_7_col143;
                let cube_252_output_limb_8_col144 = cube_252_output_tmp_44f04_71.get_m31(8);
                *row[144] = cube_252_output_limb_8_col144;
                let cube_252_output_limb_9_col145 = cube_252_output_tmp_44f04_71.get_m31(9);
                *row[145] = cube_252_output_limb_9_col145;
                *lookup_data.cube_252_2 = [
                    combination_limb_0_col125,
                    combination_limb_1_col126,
                    combination_limb_2_col127,
                    combination_limb_3_col128,
                    combination_limb_4_col129,
                    combination_limb_5_col130,
                    combination_limb_6_col131,
                    combination_limb_7_col132,
                    combination_limb_8_col133,
                    combination_limb_9_col134,
                    cube_252_output_limb_0_col136,
                    cube_252_output_limb_1_col137,
                    cube_252_output_limb_2_col138,
                    cube_252_output_limb_3_col139,
                    cube_252_output_limb_4_col140,
                    cube_252_output_limb_5_col141,
                    cube_252_output_limb_6_col142,
                    cube_252_output_limb_7_col143,
                    cube_252_output_limb_8_col144,
                    cube_252_output_limb_9_col145,
                ];

                // Linear Combination N 6 Coefs 4 2 3 1 M 1 1.

                let combination_tmp_44f04_72 = PackedFelt252Width27::from_packed_felt252(
                    (((((((Felt252_0_0_0_0)
                        + ((Felt252_4_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_partial_round_output_tmp_44f04_35[0],
                            ))))
                        + ((Felt252_2_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_partial_round_output_tmp_44f04_35[1],
                            ))))
                        + ((Felt252_3_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_partial_round_output_tmp_44f04_70[0],
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_partial_round_output_tmp_44f04_70[1],
                            ))))
                        - ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_44f04_71,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_round_keys_output_tmp_44f04_0[2],
                            )))),
                );
                let combination_limb_0_col146 = combination_tmp_44f04_72.get_m31(0);
                *row[146] = combination_limb_0_col146;
                let combination_limb_1_col147 = combination_tmp_44f04_72.get_m31(1);
                *row[147] = combination_limb_1_col147;
                let combination_limb_2_col148 = combination_tmp_44f04_72.get_m31(2);
                *row[148] = combination_limb_2_col148;
                let combination_limb_3_col149 = combination_tmp_44f04_72.get_m31(3);
                *row[149] = combination_limb_3_col149;
                let combination_limb_4_col150 = combination_tmp_44f04_72.get_m31(4);
                *row[150] = combination_limb_4_col150;
                let combination_limb_5_col151 = combination_tmp_44f04_72.get_m31(5);
                *row[151] = combination_limb_5_col151;
                let combination_limb_6_col152 = combination_tmp_44f04_72.get_m31(6);
                *row[152] = combination_limb_6_col152;
                let combination_limb_7_col153 = combination_tmp_44f04_72.get_m31(7);
                *row[153] = combination_limb_7_col153;
                let combination_limb_8_col154 = combination_tmp_44f04_72.get_m31(8);
                *row[154] = combination_limb_8_col154;
                let combination_limb_9_col155 = combination_tmp_44f04_72.get_m31(9);
                *row[155] = combination_limb_9_col155;
                let biased_limb_accumulator_u32_tmp_44f04_73 = PackedUInt32::from_m31(
                    (((((((((M31_4) * (cube_252_output_limb_0_col72))
                        + ((M31_2) * (combination_limb_0_col93)))
                        + ((M31_3) * (cube_252_output_limb_0_col104)))
                        + (combination_limb_0_col125))
                        - (cube_252_output_limb_0_col136))
                        + (poseidon_round_keys_output_limb_20_col62))
                        - (combination_limb_0_col146))
                        + (M31_268435458)),
                );
                let p_coef_col156 =
                    ((biased_limb_accumulator_u32_tmp_44f04_73.low().as_m31()) - (M31_2));
                *row[156] = p_coef_col156;
                let carry_0_tmp_44f04_74 = ((((((((((M31_4)
                    * (cube_252_output_limb_0_col72))
                    + ((M31_2) * (combination_limb_0_col93)))
                    + ((M31_3) * (cube_252_output_limb_0_col104)))
                    + (combination_limb_0_col125))
                    - (cube_252_output_limb_0_col136))
                    + (poseidon_round_keys_output_limb_20_col62))
                    - (combination_limb_0_col146))
                    - (p_coef_col156))
                    * (M31_16));
                let carry_1_tmp_44f04_75 = (((((((((carry_0_tmp_44f04_74)
                    + ((M31_4) * (cube_252_output_limb_1_col73)))
                    + ((M31_2) * (combination_limb_1_col94)))
                    + ((M31_3) * (cube_252_output_limb_1_col105)))
                    + (combination_limb_1_col126))
                    - (cube_252_output_limb_1_col137))
                    + (poseidon_round_keys_output_limb_21_col63))
                    - (combination_limb_1_col147))
                    * (M31_16));
                let carry_2_tmp_44f04_76 = (((((((((carry_1_tmp_44f04_75)
                    + ((M31_4) * (cube_252_output_limb_2_col74)))
                    + ((M31_2) * (combination_limb_2_col95)))
                    + ((M31_3) * (cube_252_output_limb_2_col106)))
                    + (combination_limb_2_col127))
                    - (cube_252_output_limb_2_col138))
                    + (poseidon_round_keys_output_limb_22_col64))
                    - (combination_limb_2_col148))
                    * (M31_16));
                let carry_3_tmp_44f04_77 = (((((((((carry_2_tmp_44f04_76)
                    + ((M31_4) * (cube_252_output_limb_3_col75)))
                    + ((M31_2) * (combination_limb_3_col96)))
                    + ((M31_3) * (cube_252_output_limb_3_col107)))
                    + (combination_limb_3_col128))
                    - (cube_252_output_limb_3_col139))
                    + (poseidon_round_keys_output_limb_23_col65))
                    - (combination_limb_3_col149))
                    * (M31_16));
                let carry_4_tmp_44f04_78 = (((((((((carry_3_tmp_44f04_77)
                    + ((M31_4) * (cube_252_output_limb_4_col76)))
                    + ((M31_2) * (combination_limb_4_col97)))
                    + ((M31_3) * (cube_252_output_limb_4_col108)))
                    + (combination_limb_4_col129))
                    - (cube_252_output_limb_4_col140))
                    + (poseidon_round_keys_output_limb_24_col66))
                    - (combination_limb_4_col150))
                    * (M31_16));
                let carry_5_tmp_44f04_79 = (((((((((carry_4_tmp_44f04_78)
                    + ((M31_4) * (cube_252_output_limb_5_col77)))
                    + ((M31_2) * (combination_limb_5_col98)))
                    + ((M31_3) * (cube_252_output_limb_5_col109)))
                    + (combination_limb_5_col130))
                    - (cube_252_output_limb_5_col141))
                    + (poseidon_round_keys_output_limb_25_col67))
                    - (combination_limb_5_col151))
                    * (M31_16));
                let carry_6_tmp_44f04_80 = (((((((((carry_5_tmp_44f04_79)
                    + ((M31_4) * (cube_252_output_limb_6_col78)))
                    + ((M31_2) * (combination_limb_6_col99)))
                    + ((M31_3) * (cube_252_output_limb_6_col110)))
                    + (combination_limb_6_col131))
                    - (cube_252_output_limb_6_col142))
                    + (poseidon_round_keys_output_limb_26_col68))
                    - (combination_limb_6_col152))
                    * (M31_16));
                let carry_7_tmp_44f04_81 = ((((((((((carry_6_tmp_44f04_80)
                    + ((M31_4) * (cube_252_output_limb_7_col79)))
                    + ((M31_2) * (combination_limb_7_col100)))
                    + ((M31_3) * (cube_252_output_limb_7_col111)))
                    + (combination_limb_7_col132))
                    - (cube_252_output_limb_7_col143))
                    + (poseidon_round_keys_output_limb_27_col69))
                    - (combination_limb_7_col153))
                    - ((p_coef_col156) * (M31_136)))
                    * (M31_16));
                let carry_8_tmp_44f04_82 = (((((((((carry_7_tmp_44f04_81)
                    + ((M31_4) * (cube_252_output_limb_8_col80)))
                    + ((M31_2) * (combination_limb_8_col101)))
                    + ((M31_3) * (cube_252_output_limb_8_col112)))
                    + (combination_limb_8_col133))
                    - (cube_252_output_limb_8_col144))
                    + (poseidon_round_keys_output_limb_28_col70))
                    - (combination_limb_8_col154))
                    * (M31_16));
                *sub_component_inputs.range_check_4_4_4_4[4] = [
                    ((p_coef_col156) + (M31_2)),
                    ((carry_0_tmp_44f04_74) + (M31_2)),
                    ((carry_1_tmp_44f04_75) + (M31_2)),
                    ((carry_2_tmp_44f04_76) + (M31_2)),
                ];
                *lookup_data.range_check_4_4_4_4_4 = [
                    ((p_coef_col156) + (M31_2)),
                    ((carry_0_tmp_44f04_74) + (M31_2)),
                    ((carry_1_tmp_44f04_75) + (M31_2)),
                    ((carry_2_tmp_44f04_76) + (M31_2)),
                ];
                *sub_component_inputs.range_check_4_4_4_4[5] = [
                    ((carry_3_tmp_44f04_77) + (M31_2)),
                    ((carry_4_tmp_44f04_78) + (M31_2)),
                    ((carry_5_tmp_44f04_79) + (M31_2)),
                    ((carry_6_tmp_44f04_80) + (M31_2)),
                ];
                *lookup_data.range_check_4_4_4_4_5 = [
                    ((carry_3_tmp_44f04_77) + (M31_2)),
                    ((carry_4_tmp_44f04_78) + (M31_2)),
                    ((carry_5_tmp_44f04_79) + (M31_2)),
                    ((carry_6_tmp_44f04_80) + (M31_2)),
                ];
                *sub_component_inputs.range_check_4_4[2] = [
                    ((carry_7_tmp_44f04_81) + (M31_2)),
                    ((carry_8_tmp_44f04_82) + (M31_2)),
                ];
                *lookup_data.range_check_4_4_2 = [
                    ((carry_7_tmp_44f04_81) + (M31_2)),
                    ((carry_8_tmp_44f04_82) + (M31_2)),
                ];
                let linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_44f04_83 =
                    combination_tmp_44f04_72;

                *sub_component_inputs.range_check_252_width_27[2] =
                    linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_44f04_83;
                *lookup_data.range_check_252_width_27_2 = [
                    combination_limb_0_col146,
                    combination_limb_1_col147,
                    combination_limb_2_col148,
                    combination_limb_3_col149,
                    combination_limb_4_col150,
                    combination_limb_5_col151,
                    combination_limb_6_col152,
                    combination_limb_7_col153,
                    combination_limb_8_col154,
                    combination_limb_9_col155,
                ];

                // Linear Combination N 1 Coefs 2.

                let combination_tmp_44f04_84 = PackedFelt252Width27::from_packed_felt252(
                    ((Felt252_0_0_0_0)
                        + ((Felt252_2_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_44f04_83,
                            )))),
                );
                let combination_limb_0_col157 = combination_tmp_44f04_84.get_m31(0);
                *row[157] = combination_limb_0_col157;
                let combination_limb_1_col158 = combination_tmp_44f04_84.get_m31(1);
                *row[158] = combination_limb_1_col158;
                let combination_limb_2_col159 = combination_tmp_44f04_84.get_m31(2);
                *row[159] = combination_limb_2_col159;
                let combination_limb_3_col160 = combination_tmp_44f04_84.get_m31(3);
                *row[160] = combination_limb_3_col160;
                let combination_limb_4_col161 = combination_tmp_44f04_84.get_m31(4);
                *row[161] = combination_limb_4_col161;
                let combination_limb_5_col162 = combination_tmp_44f04_84.get_m31(5);
                *row[162] = combination_limb_5_col162;
                let combination_limb_6_col163 = combination_tmp_44f04_84.get_m31(6);
                *row[163] = combination_limb_6_col163;
                let combination_limb_7_col164 = combination_tmp_44f04_84.get_m31(7);
                *row[164] = combination_limb_7_col164;
                let combination_limb_8_col165 = combination_tmp_44f04_84.get_m31(8);
                *row[165] = combination_limb_8_col165;
                let combination_limb_9_col166 = combination_tmp_44f04_84.get_m31(9);
                *row[166] = combination_limb_9_col166;
                let biased_limb_accumulator_u32_tmp_44f04_85 = PackedUInt32::from_m31(
                    ((((M31_2) * (combination_limb_0_col146)) - (combination_limb_0_col157))
                        + (M31_134217729)),
                );
                let p_coef_col167 =
                    ((biased_limb_accumulator_u32_tmp_44f04_85.low().as_m31()) - (M31_1));
                *row[167] = p_coef_col167;
                let carry_0_tmp_44f04_86 = (((((M31_2) * (combination_limb_0_col146))
                    - (combination_limb_0_col157))
                    - (p_coef_col167))
                    * (M31_16));
                let carry_1_tmp_44f04_87 = ((((carry_0_tmp_44f04_86)
                    + ((M31_2) * (combination_limb_1_col147)))
                    - (combination_limb_1_col158))
                    * (M31_16));
                let carry_2_tmp_44f04_88 = ((((carry_1_tmp_44f04_87)
                    + ((M31_2) * (combination_limb_2_col148)))
                    - (combination_limb_2_col159))
                    * (M31_16));
                let carry_3_tmp_44f04_89 = ((((carry_2_tmp_44f04_88)
                    + ((M31_2) * (combination_limb_3_col149)))
                    - (combination_limb_3_col160))
                    * (M31_16));
                let carry_4_tmp_44f04_90 = ((((carry_3_tmp_44f04_89)
                    + ((M31_2) * (combination_limb_4_col150)))
                    - (combination_limb_4_col161))
                    * (M31_16));
                let carry_5_tmp_44f04_91 = ((((carry_4_tmp_44f04_90)
                    + ((M31_2) * (combination_limb_5_col151)))
                    - (combination_limb_5_col162))
                    * (M31_16));
                let carry_6_tmp_44f04_92 = ((((carry_5_tmp_44f04_91)
                    + ((M31_2) * (combination_limb_6_col152)))
                    - (combination_limb_6_col163))
                    * (M31_16));
                let carry_7_tmp_44f04_93 = (((((carry_6_tmp_44f04_92)
                    + ((M31_2) * (combination_limb_7_col153)))
                    - (combination_limb_7_col164))
                    - ((p_coef_col167) * (M31_136)))
                    * (M31_16));
                let carry_8_tmp_44f04_94 = ((((carry_7_tmp_44f04_93)
                    + ((M31_2) * (combination_limb_8_col154)))
                    - (combination_limb_8_col165))
                    * (M31_16));
                let linear_combination_n_1_coefs_2_output_tmp_44f04_104 = combination_tmp_44f04_84;

                let poseidon_partial_round_output_tmp_44f04_105 = [
                    cube_252_output_tmp_44f04_71,
                    linear_combination_n_1_coefs_2_output_tmp_44f04_104,
                ];

                *lookup_data.poseidon_3_partial_rounds_chain_0 = [
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
                ];
                *lookup_data.poseidon_3_partial_rounds_chain_1 = [
                    input_limb_0_col0,
                    ((input_limb_1_col1) + (M31_1)),
                    cube_252_output_limb_0_col104,
                    cube_252_output_limb_1_col105,
                    cube_252_output_limb_2_col106,
                    cube_252_output_limb_3_col107,
                    cube_252_output_limb_4_col108,
                    cube_252_output_limb_5_col109,
                    cube_252_output_limb_6_col110,
                    cube_252_output_limb_7_col111,
                    cube_252_output_limb_8_col112,
                    cube_252_output_limb_9_col113,
                    combination_limb_0_col125,
                    combination_limb_1_col126,
                    combination_limb_2_col127,
                    combination_limb_3_col128,
                    combination_limb_4_col129,
                    combination_limb_5_col130,
                    combination_limb_6_col131,
                    combination_limb_7_col132,
                    combination_limb_8_col133,
                    combination_limb_9_col134,
                    cube_252_output_limb_0_col136,
                    cube_252_output_limb_1_col137,
                    cube_252_output_limb_2_col138,
                    cube_252_output_limb_3_col139,
                    cube_252_output_limb_4_col140,
                    cube_252_output_limb_5_col141,
                    cube_252_output_limb_6_col142,
                    cube_252_output_limb_7_col143,
                    cube_252_output_limb_8_col144,
                    cube_252_output_limb_9_col145,
                    combination_limb_0_col157,
                    combination_limb_1_col158,
                    combination_limb_2_col159,
                    combination_limb_3_col160,
                    combination_limb_4_col161,
                    combination_limb_5_col162,
                    combination_limb_6_col163,
                    combination_limb_7_col164,
                    combination_limb_8_col165,
                    combination_limb_9_col166,
                ];
                *row[168] = enabler_col.packed_at(row_index);
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    cube_252_0: Vec<[PackedM31; 20]>,
    cube_252_1: Vec<[PackedM31; 20]>,
    cube_252_2: Vec<[PackedM31; 20]>,
    poseidon_3_partial_rounds_chain_0: Vec<[PackedM31; 42]>,
    poseidon_3_partial_rounds_chain_1: Vec<[PackedM31; 42]>,
    poseidon_round_keys_0: Vec<[PackedM31; 31]>,
    range_check_252_width_27_0: Vec<[PackedM31; 10]>,
    range_check_252_width_27_1: Vec<[PackedM31; 10]>,
    range_check_252_width_27_2: Vec<[PackedM31; 10]>,
    range_check_4_4_0: Vec<[PackedM31; 2]>,
    range_check_4_4_1: Vec<[PackedM31; 2]>,
    range_check_4_4_2: Vec<[PackedM31; 2]>,
    range_check_4_4_4_4_0: Vec<[PackedM31; 4]>,
    range_check_4_4_4_4_1: Vec<[PackedM31; 4]>,
    range_check_4_4_4_4_2: Vec<[PackedM31; 4]>,
    range_check_4_4_4_4_3: Vec<[PackedM31; 4]>,
    range_check_4_4_4_4_4: Vec<[PackedM31; 4]>,
    range_check_4_4_4_4_5: Vec<[PackedM31; 4]>,
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
        poseidon_round_keys: &relations::PoseidonRoundKeys,
        cube_252: &relations::Cube252,
        range_check_4_4_4_4: &relations::RangeCheck_4_4_4_4,
        range_check_4_4: &relations::RangeCheck_4_4,
        range_check_252_width_27: &relations::RangeCheck252Width27,
        poseidon_3_partial_rounds_chain: &relations::Poseidon3PartialRoundsChain,
    ) -> InteractionClaim {
        let enabler_col = Enabler::new(self.n_rows);
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.poseidon_round_keys_0,
            &self.lookup_data.cube_252_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = poseidon_round_keys.combine(values0);
                let denom1: PackedQM31 = cube_252.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_4_4_4_4_0,
            &self.lookup_data.range_check_4_4_4_4_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_4_4_4_4.combine(values0);
                let denom1: PackedQM31 = range_check_4_4_4_4.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_4_4_0,
            &self.lookup_data.range_check_252_width_27_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_4_4.combine(values0);
                let denom1: PackedQM31 = range_check_252_width_27.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.cube_252_1,
            &self.lookup_data.range_check_4_4_4_4_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = cube_252.combine(values0);
                let denom1: PackedQM31 = range_check_4_4_4_4.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_4_4_4_4_3,
            &self.lookup_data.range_check_4_4_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_4_4_4_4.combine(values0);
                let denom1: PackedQM31 = range_check_4_4.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_252_width_27_1,
            &self.lookup_data.cube_252_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_252_width_27.combine(values0);
                let denom1: PackedQM31 = cube_252.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_4_4_4_4_4,
            &self.lookup_data.range_check_4_4_4_4_5,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_4_4_4_4.combine(values0);
                let denom1: PackedQM31 = range_check_4_4_4_4.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_4_4_2,
            &self.lookup_data.range_check_252_width_27_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_4_4.combine(values0);
                let denom1: PackedQM31 = range_check_252_width_27.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.poseidon_3_partial_rounds_chain_0,
            &self.lookup_data.poseidon_3_partial_rounds_chain_1,
        )
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (writer, values0, values1))| {
                let denom0: PackedQM31 = poseidon_3_partial_rounds_chain.combine(values0);
                let denom1: PackedQM31 = poseidon_3_partial_rounds_chain.combine(values1);
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
