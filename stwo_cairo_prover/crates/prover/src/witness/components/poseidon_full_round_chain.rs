// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::poseidon_full_round_chain::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{cube_252, poseidon_round_keys, range_check_3_3_3_3_3};
use crate::witness::prelude::*;

pub type PackedInputType = (PackedM31, PackedM31, [PackedFelt252Width27; 3]);

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
        cube_252_state: &mut cube_252::ClaimGenerator,
        poseidon_round_keys_state: &poseidon_round_keys::ClaimGenerator,
        range_check_3_3_3_3_3_state: &range_check_3_3_3_3_3::ClaimGenerator,
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
            cube_252_state,
            poseidon_round_keys_state,
            range_check_3_3_3_3_3_state,
        );
        sub_component_inputs.cube_252.iter().for_each(|inputs| {
            cube_252_state.add_packed_inputs(inputs);
        });
        sub_component_inputs
            .poseidon_round_keys
            .iter()
            .for_each(|inputs| {
                poseidon_round_keys_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_3_3_3_3_3
            .iter()
            .for_each(|inputs| {
                range_check_3_3_3_3_3_state.add_packed_inputs(inputs);
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
    cube_252: [Vec<cube_252::PackedInputType>; 3],
    poseidon_round_keys: [Vec<poseidon_round_keys::PackedInputType>; 1],
    range_check_3_3_3_3_3: [Vec<range_check_3_3_3_3_3::PackedInputType>; 6],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    n_rows: usize,
    cube_252_state: &mut cube_252::ClaimGenerator,
    poseidon_round_keys_state: &poseidon_round_keys::ClaimGenerator,
    range_check_3_3_3_3_3_state: &range_check_3_3_3_3_3::ClaimGenerator,
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
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_134217729 = PackedM31::broadcast(M31::from(134217729));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_16 = PackedM31::broadcast(M31::from(16));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_268435458 = PackedM31::broadcast(M31::from(268435458));
    let M31_3 = PackedM31::broadcast(M31::from(3));
    let M31_402653187 = PackedM31::broadcast(M31::from(402653187));
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
                (row, lookup_data, sub_component_inputs, poseidon_full_round_chain_input),
            )| {
                let input_limb_0_col0 = poseidon_full_round_chain_input.0;
                *row[0] = input_limb_0_col0;
                let input_limb_1_col1 = poseidon_full_round_chain_input.1;
                *row[1] = input_limb_1_col1;
                let input_limb_2_col2 = poseidon_full_round_chain_input.2[0].get_m31(0);
                *row[2] = input_limb_2_col2;
                let input_limb_3_col3 = poseidon_full_round_chain_input.2[0].get_m31(1);
                *row[3] = input_limb_3_col3;
                let input_limb_4_col4 = poseidon_full_round_chain_input.2[0].get_m31(2);
                *row[4] = input_limb_4_col4;
                let input_limb_5_col5 = poseidon_full_round_chain_input.2[0].get_m31(3);
                *row[5] = input_limb_5_col5;
                let input_limb_6_col6 = poseidon_full_round_chain_input.2[0].get_m31(4);
                *row[6] = input_limb_6_col6;
                let input_limb_7_col7 = poseidon_full_round_chain_input.2[0].get_m31(5);
                *row[7] = input_limb_7_col7;
                let input_limb_8_col8 = poseidon_full_round_chain_input.2[0].get_m31(6);
                *row[8] = input_limb_8_col8;
                let input_limb_9_col9 = poseidon_full_round_chain_input.2[0].get_m31(7);
                *row[9] = input_limb_9_col9;
                let input_limb_10_col10 = poseidon_full_round_chain_input.2[0].get_m31(8);
                *row[10] = input_limb_10_col10;
                let input_limb_11_col11 = poseidon_full_round_chain_input.2[0].get_m31(9);
                *row[11] = input_limb_11_col11;
                let input_limb_12_col12 = poseidon_full_round_chain_input.2[1].get_m31(0);
                *row[12] = input_limb_12_col12;
                let input_limb_13_col13 = poseidon_full_round_chain_input.2[1].get_m31(1);
                *row[13] = input_limb_13_col13;
                let input_limb_14_col14 = poseidon_full_round_chain_input.2[1].get_m31(2);
                *row[14] = input_limb_14_col14;
                let input_limb_15_col15 = poseidon_full_round_chain_input.2[1].get_m31(3);
                *row[15] = input_limb_15_col15;
                let input_limb_16_col16 = poseidon_full_round_chain_input.2[1].get_m31(4);
                *row[16] = input_limb_16_col16;
                let input_limb_17_col17 = poseidon_full_round_chain_input.2[1].get_m31(5);
                *row[17] = input_limb_17_col17;
                let input_limb_18_col18 = poseidon_full_round_chain_input.2[1].get_m31(6);
                *row[18] = input_limb_18_col18;
                let input_limb_19_col19 = poseidon_full_round_chain_input.2[1].get_m31(7);
                *row[19] = input_limb_19_col19;
                let input_limb_20_col20 = poseidon_full_round_chain_input.2[1].get_m31(8);
                *row[20] = input_limb_20_col20;
                let input_limb_21_col21 = poseidon_full_round_chain_input.2[1].get_m31(9);
                *row[21] = input_limb_21_col21;
                let input_limb_22_col22 = poseidon_full_round_chain_input.2[2].get_m31(0);
                *row[22] = input_limb_22_col22;
                let input_limb_23_col23 = poseidon_full_round_chain_input.2[2].get_m31(1);
                *row[23] = input_limb_23_col23;
                let input_limb_24_col24 = poseidon_full_round_chain_input.2[2].get_m31(2);
                *row[24] = input_limb_24_col24;
                let input_limb_25_col25 = poseidon_full_round_chain_input.2[2].get_m31(3);
                *row[25] = input_limb_25_col25;
                let input_limb_26_col26 = poseidon_full_round_chain_input.2[2].get_m31(4);
                *row[26] = input_limb_26_col26;
                let input_limb_27_col27 = poseidon_full_round_chain_input.2[2].get_m31(5);
                *row[27] = input_limb_27_col27;
                let input_limb_28_col28 = poseidon_full_round_chain_input.2[2].get_m31(6);
                *row[28] = input_limb_28_col28;
                let input_limb_29_col29 = poseidon_full_round_chain_input.2[2].get_m31(7);
                *row[29] = input_limb_29_col29;
                let input_limb_30_col30 = poseidon_full_round_chain_input.2[2].get_m31(8);
                *row[30] = input_limb_30_col30;
                let input_limb_31_col31 = poseidon_full_round_chain_input.2[2].get_m31(9);
                *row[31] = input_limb_31_col31;
                *sub_component_inputs.cube_252[0] = poseidon_full_round_chain_input.2[0];
                let cube_252_output_tmp_f9fbc_0 =
                    PackedCube252::deduce_output(poseidon_full_round_chain_input.2[0]);
                let cube_252_output_limb_0_col32 = cube_252_output_tmp_f9fbc_0.get_m31(0);
                *row[32] = cube_252_output_limb_0_col32;
                let cube_252_output_limb_1_col33 = cube_252_output_tmp_f9fbc_0.get_m31(1);
                *row[33] = cube_252_output_limb_1_col33;
                let cube_252_output_limb_2_col34 = cube_252_output_tmp_f9fbc_0.get_m31(2);
                *row[34] = cube_252_output_limb_2_col34;
                let cube_252_output_limb_3_col35 = cube_252_output_tmp_f9fbc_0.get_m31(3);
                *row[35] = cube_252_output_limb_3_col35;
                let cube_252_output_limb_4_col36 = cube_252_output_tmp_f9fbc_0.get_m31(4);
                *row[36] = cube_252_output_limb_4_col36;
                let cube_252_output_limb_5_col37 = cube_252_output_tmp_f9fbc_0.get_m31(5);
                *row[37] = cube_252_output_limb_5_col37;
                let cube_252_output_limb_6_col38 = cube_252_output_tmp_f9fbc_0.get_m31(6);
                *row[38] = cube_252_output_limb_6_col38;
                let cube_252_output_limb_7_col39 = cube_252_output_tmp_f9fbc_0.get_m31(7);
                *row[39] = cube_252_output_limb_7_col39;
                let cube_252_output_limb_8_col40 = cube_252_output_tmp_f9fbc_0.get_m31(8);
                *row[40] = cube_252_output_limb_8_col40;
                let cube_252_output_limb_9_col41 = cube_252_output_tmp_f9fbc_0.get_m31(9);
                *row[41] = cube_252_output_limb_9_col41;
                *lookup_data.cube_252_0 = [
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
                    cube_252_output_limb_0_col32,
                    cube_252_output_limb_1_col33,
                    cube_252_output_limb_2_col34,
                    cube_252_output_limb_3_col35,
                    cube_252_output_limb_4_col36,
                    cube_252_output_limb_5_col37,
                    cube_252_output_limb_6_col38,
                    cube_252_output_limb_7_col39,
                    cube_252_output_limb_8_col40,
                    cube_252_output_limb_9_col41,
                ];
                *sub_component_inputs.cube_252[1] = poseidon_full_round_chain_input.2[1];
                let cube_252_output_tmp_f9fbc_1 =
                    PackedCube252::deduce_output(poseidon_full_round_chain_input.2[1]);
                let cube_252_output_limb_0_col42 = cube_252_output_tmp_f9fbc_1.get_m31(0);
                *row[42] = cube_252_output_limb_0_col42;
                let cube_252_output_limb_1_col43 = cube_252_output_tmp_f9fbc_1.get_m31(1);
                *row[43] = cube_252_output_limb_1_col43;
                let cube_252_output_limb_2_col44 = cube_252_output_tmp_f9fbc_1.get_m31(2);
                *row[44] = cube_252_output_limb_2_col44;
                let cube_252_output_limb_3_col45 = cube_252_output_tmp_f9fbc_1.get_m31(3);
                *row[45] = cube_252_output_limb_3_col45;
                let cube_252_output_limb_4_col46 = cube_252_output_tmp_f9fbc_1.get_m31(4);
                *row[46] = cube_252_output_limb_4_col46;
                let cube_252_output_limb_5_col47 = cube_252_output_tmp_f9fbc_1.get_m31(5);
                *row[47] = cube_252_output_limb_5_col47;
                let cube_252_output_limb_6_col48 = cube_252_output_tmp_f9fbc_1.get_m31(6);
                *row[48] = cube_252_output_limb_6_col48;
                let cube_252_output_limb_7_col49 = cube_252_output_tmp_f9fbc_1.get_m31(7);
                *row[49] = cube_252_output_limb_7_col49;
                let cube_252_output_limb_8_col50 = cube_252_output_tmp_f9fbc_1.get_m31(8);
                *row[50] = cube_252_output_limb_8_col50;
                let cube_252_output_limb_9_col51 = cube_252_output_tmp_f9fbc_1.get_m31(9);
                *row[51] = cube_252_output_limb_9_col51;
                *lookup_data.cube_252_1 = [
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
                    cube_252_output_limb_0_col42,
                    cube_252_output_limb_1_col43,
                    cube_252_output_limb_2_col44,
                    cube_252_output_limb_3_col45,
                    cube_252_output_limb_4_col46,
                    cube_252_output_limb_5_col47,
                    cube_252_output_limb_6_col48,
                    cube_252_output_limb_7_col49,
                    cube_252_output_limb_8_col50,
                    cube_252_output_limb_9_col51,
                ];
                *sub_component_inputs.cube_252[2] = poseidon_full_round_chain_input.2[2];
                let cube_252_output_tmp_f9fbc_2 =
                    PackedCube252::deduce_output(poseidon_full_round_chain_input.2[2]);
                let cube_252_output_limb_0_col52 = cube_252_output_tmp_f9fbc_2.get_m31(0);
                *row[52] = cube_252_output_limb_0_col52;
                let cube_252_output_limb_1_col53 = cube_252_output_tmp_f9fbc_2.get_m31(1);
                *row[53] = cube_252_output_limb_1_col53;
                let cube_252_output_limb_2_col54 = cube_252_output_tmp_f9fbc_2.get_m31(2);
                *row[54] = cube_252_output_limb_2_col54;
                let cube_252_output_limb_3_col55 = cube_252_output_tmp_f9fbc_2.get_m31(3);
                *row[55] = cube_252_output_limb_3_col55;
                let cube_252_output_limb_4_col56 = cube_252_output_tmp_f9fbc_2.get_m31(4);
                *row[56] = cube_252_output_limb_4_col56;
                let cube_252_output_limb_5_col57 = cube_252_output_tmp_f9fbc_2.get_m31(5);
                *row[57] = cube_252_output_limb_5_col57;
                let cube_252_output_limb_6_col58 = cube_252_output_tmp_f9fbc_2.get_m31(6);
                *row[58] = cube_252_output_limb_6_col58;
                let cube_252_output_limb_7_col59 = cube_252_output_tmp_f9fbc_2.get_m31(7);
                *row[59] = cube_252_output_limb_7_col59;
                let cube_252_output_limb_8_col60 = cube_252_output_tmp_f9fbc_2.get_m31(8);
                *row[60] = cube_252_output_limb_8_col60;
                let cube_252_output_limb_9_col61 = cube_252_output_tmp_f9fbc_2.get_m31(9);
                *row[61] = cube_252_output_limb_9_col61;
                *lookup_data.cube_252_2 = [
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
                    cube_252_output_limb_0_col52,
                    cube_252_output_limb_1_col53,
                    cube_252_output_limb_2_col54,
                    cube_252_output_limb_3_col55,
                    cube_252_output_limb_4_col56,
                    cube_252_output_limb_5_col57,
                    cube_252_output_limb_6_col58,
                    cube_252_output_limb_7_col59,
                    cube_252_output_limb_8_col60,
                    cube_252_output_limb_9_col61,
                ];
                *sub_component_inputs.poseidon_round_keys[0] = [input_limb_1_col1];
                let poseidon_round_keys_output_tmp_f9fbc_3 =
                    PackedPoseidonRoundKeys::deduce_output([input_limb_1_col1]);
                let poseidon_round_keys_output_limb_0_col62 =
                    poseidon_round_keys_output_tmp_f9fbc_3[0].get_m31(0);
                *row[62] = poseidon_round_keys_output_limb_0_col62;
                let poseidon_round_keys_output_limb_1_col63 =
                    poseidon_round_keys_output_tmp_f9fbc_3[0].get_m31(1);
                *row[63] = poseidon_round_keys_output_limb_1_col63;
                let poseidon_round_keys_output_limb_2_col64 =
                    poseidon_round_keys_output_tmp_f9fbc_3[0].get_m31(2);
                *row[64] = poseidon_round_keys_output_limb_2_col64;
                let poseidon_round_keys_output_limb_3_col65 =
                    poseidon_round_keys_output_tmp_f9fbc_3[0].get_m31(3);
                *row[65] = poseidon_round_keys_output_limb_3_col65;
                let poseidon_round_keys_output_limb_4_col66 =
                    poseidon_round_keys_output_tmp_f9fbc_3[0].get_m31(4);
                *row[66] = poseidon_round_keys_output_limb_4_col66;
                let poseidon_round_keys_output_limb_5_col67 =
                    poseidon_round_keys_output_tmp_f9fbc_3[0].get_m31(5);
                *row[67] = poseidon_round_keys_output_limb_5_col67;
                let poseidon_round_keys_output_limb_6_col68 =
                    poseidon_round_keys_output_tmp_f9fbc_3[0].get_m31(6);
                *row[68] = poseidon_round_keys_output_limb_6_col68;
                let poseidon_round_keys_output_limb_7_col69 =
                    poseidon_round_keys_output_tmp_f9fbc_3[0].get_m31(7);
                *row[69] = poseidon_round_keys_output_limb_7_col69;
                let poseidon_round_keys_output_limb_8_col70 =
                    poseidon_round_keys_output_tmp_f9fbc_3[0].get_m31(8);
                *row[70] = poseidon_round_keys_output_limb_8_col70;
                let poseidon_round_keys_output_limb_9_col71 =
                    poseidon_round_keys_output_tmp_f9fbc_3[0].get_m31(9);
                *row[71] = poseidon_round_keys_output_limb_9_col71;
                let poseidon_round_keys_output_limb_10_col72 =
                    poseidon_round_keys_output_tmp_f9fbc_3[1].get_m31(0);
                *row[72] = poseidon_round_keys_output_limb_10_col72;
                let poseidon_round_keys_output_limb_11_col73 =
                    poseidon_round_keys_output_tmp_f9fbc_3[1].get_m31(1);
                *row[73] = poseidon_round_keys_output_limb_11_col73;
                let poseidon_round_keys_output_limb_12_col74 =
                    poseidon_round_keys_output_tmp_f9fbc_3[1].get_m31(2);
                *row[74] = poseidon_round_keys_output_limb_12_col74;
                let poseidon_round_keys_output_limb_13_col75 =
                    poseidon_round_keys_output_tmp_f9fbc_3[1].get_m31(3);
                *row[75] = poseidon_round_keys_output_limb_13_col75;
                let poseidon_round_keys_output_limb_14_col76 =
                    poseidon_round_keys_output_tmp_f9fbc_3[1].get_m31(4);
                *row[76] = poseidon_round_keys_output_limb_14_col76;
                let poseidon_round_keys_output_limb_15_col77 =
                    poseidon_round_keys_output_tmp_f9fbc_3[1].get_m31(5);
                *row[77] = poseidon_round_keys_output_limb_15_col77;
                let poseidon_round_keys_output_limb_16_col78 =
                    poseidon_round_keys_output_tmp_f9fbc_3[1].get_m31(6);
                *row[78] = poseidon_round_keys_output_limb_16_col78;
                let poseidon_round_keys_output_limb_17_col79 =
                    poseidon_round_keys_output_tmp_f9fbc_3[1].get_m31(7);
                *row[79] = poseidon_round_keys_output_limb_17_col79;
                let poseidon_round_keys_output_limb_18_col80 =
                    poseidon_round_keys_output_tmp_f9fbc_3[1].get_m31(8);
                *row[80] = poseidon_round_keys_output_limb_18_col80;
                let poseidon_round_keys_output_limb_19_col81 =
                    poseidon_round_keys_output_tmp_f9fbc_3[1].get_m31(9);
                *row[81] = poseidon_round_keys_output_limb_19_col81;
                let poseidon_round_keys_output_limb_20_col82 =
                    poseidon_round_keys_output_tmp_f9fbc_3[2].get_m31(0);
                *row[82] = poseidon_round_keys_output_limb_20_col82;
                let poseidon_round_keys_output_limb_21_col83 =
                    poseidon_round_keys_output_tmp_f9fbc_3[2].get_m31(1);
                *row[83] = poseidon_round_keys_output_limb_21_col83;
                let poseidon_round_keys_output_limb_22_col84 =
                    poseidon_round_keys_output_tmp_f9fbc_3[2].get_m31(2);
                *row[84] = poseidon_round_keys_output_limb_22_col84;
                let poseidon_round_keys_output_limb_23_col85 =
                    poseidon_round_keys_output_tmp_f9fbc_3[2].get_m31(3);
                *row[85] = poseidon_round_keys_output_limb_23_col85;
                let poseidon_round_keys_output_limb_24_col86 =
                    poseidon_round_keys_output_tmp_f9fbc_3[2].get_m31(4);
                *row[86] = poseidon_round_keys_output_limb_24_col86;
                let poseidon_round_keys_output_limb_25_col87 =
                    poseidon_round_keys_output_tmp_f9fbc_3[2].get_m31(5);
                *row[87] = poseidon_round_keys_output_limb_25_col87;
                let poseidon_round_keys_output_limb_26_col88 =
                    poseidon_round_keys_output_tmp_f9fbc_3[2].get_m31(6);
                *row[88] = poseidon_round_keys_output_limb_26_col88;
                let poseidon_round_keys_output_limb_27_col89 =
                    poseidon_round_keys_output_tmp_f9fbc_3[2].get_m31(7);
                *row[89] = poseidon_round_keys_output_limb_27_col89;
                let poseidon_round_keys_output_limb_28_col90 =
                    poseidon_round_keys_output_tmp_f9fbc_3[2].get_m31(8);
                *row[90] = poseidon_round_keys_output_limb_28_col90;
                let poseidon_round_keys_output_limb_29_col91 =
                    poseidon_round_keys_output_tmp_f9fbc_3[2].get_m31(9);
                *row[91] = poseidon_round_keys_output_limb_29_col91;
                *lookup_data.poseidon_round_keys_0 = [
                    input_limb_1_col1,
                    poseidon_round_keys_output_limb_0_col62,
                    poseidon_round_keys_output_limb_1_col63,
                    poseidon_round_keys_output_limb_2_col64,
                    poseidon_round_keys_output_limb_3_col65,
                    poseidon_round_keys_output_limb_4_col66,
                    poseidon_round_keys_output_limb_5_col67,
                    poseidon_round_keys_output_limb_6_col68,
                    poseidon_round_keys_output_limb_7_col69,
                    poseidon_round_keys_output_limb_8_col70,
                    poseidon_round_keys_output_limb_9_col71,
                    poseidon_round_keys_output_limb_10_col72,
                    poseidon_round_keys_output_limb_11_col73,
                    poseidon_round_keys_output_limb_12_col74,
                    poseidon_round_keys_output_limb_13_col75,
                    poseidon_round_keys_output_limb_14_col76,
                    poseidon_round_keys_output_limb_15_col77,
                    poseidon_round_keys_output_limb_16_col78,
                    poseidon_round_keys_output_limb_17_col79,
                    poseidon_round_keys_output_limb_18_col80,
                    poseidon_round_keys_output_limb_19_col81,
                    poseidon_round_keys_output_limb_20_col82,
                    poseidon_round_keys_output_limb_21_col83,
                    poseidon_round_keys_output_limb_22_col84,
                    poseidon_round_keys_output_limb_23_col85,
                    poseidon_round_keys_output_limb_24_col86,
                    poseidon_round_keys_output_limb_25_col87,
                    poseidon_round_keys_output_limb_26_col88,
                    poseidon_round_keys_output_limb_27_col89,
                    poseidon_round_keys_output_limb_28_col90,
                    poseidon_round_keys_output_limb_29_col91,
                ];

                // Linear Combination N 4 Coefs 3 1 1 1.

                let combination_tmp_f9fbc_4 = PackedFelt252Width27::from_packed_felt252(
                    (((((Felt252_0_0_0_0)
                        + ((Felt252_3_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_f9fbc_0,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_f9fbc_1,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_f9fbc_2,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_round_keys_output_tmp_f9fbc_3[0],
                            )))),
                );
                let combination_limb_0_col92 = combination_tmp_f9fbc_4.get_m31(0);
                *row[92] = combination_limb_0_col92;
                let combination_limb_1_col93 = combination_tmp_f9fbc_4.get_m31(1);
                *row[93] = combination_limb_1_col93;
                let combination_limb_2_col94 = combination_tmp_f9fbc_4.get_m31(2);
                *row[94] = combination_limb_2_col94;
                let combination_limb_3_col95 = combination_tmp_f9fbc_4.get_m31(3);
                *row[95] = combination_limb_3_col95;
                let combination_limb_4_col96 = combination_tmp_f9fbc_4.get_m31(4);
                *row[96] = combination_limb_4_col96;
                let combination_limb_5_col97 = combination_tmp_f9fbc_4.get_m31(5);
                *row[97] = combination_limb_5_col97;
                let combination_limb_6_col98 = combination_tmp_f9fbc_4.get_m31(6);
                *row[98] = combination_limb_6_col98;
                let combination_limb_7_col99 = combination_tmp_f9fbc_4.get_m31(7);
                *row[99] = combination_limb_7_col99;
                let combination_limb_8_col100 = combination_tmp_f9fbc_4.get_m31(8);
                *row[100] = combination_limb_8_col100;
                let combination_limb_9_col101 = combination_tmp_f9fbc_4.get_m31(9);
                *row[101] = combination_limb_9_col101;
                let biased_limb_accumulator_u32_tmp_f9fbc_5 = PackedUInt32::from_m31(
                    (((((((M31_3) * (cube_252_output_limb_0_col32))
                        + (cube_252_output_limb_0_col42))
                        + (cube_252_output_limb_0_col52))
                        + (poseidon_round_keys_output_limb_0_col62))
                        - (combination_limb_0_col92))
                        + (M31_134217729)),
                );
                let p_coef_col102 =
                    ((biased_limb_accumulator_u32_tmp_f9fbc_5.low().as_m31()) - (M31_1));
                *row[102] = p_coef_col102;
                let carry_0_tmp_f9fbc_6 = ((((((((M31_3) * (cube_252_output_limb_0_col32))
                    + (cube_252_output_limb_0_col42))
                    + (cube_252_output_limb_0_col52))
                    + (poseidon_round_keys_output_limb_0_col62))
                    - (combination_limb_0_col92))
                    - (p_coef_col102))
                    * (M31_16));
                let carry_1_tmp_f9fbc_7 = (((((((carry_0_tmp_f9fbc_6)
                    + ((M31_3) * (cube_252_output_limb_1_col33)))
                    + (cube_252_output_limb_1_col43))
                    + (cube_252_output_limb_1_col53))
                    + (poseidon_round_keys_output_limb_1_col63))
                    - (combination_limb_1_col93))
                    * (M31_16));
                let carry_2_tmp_f9fbc_8 = (((((((carry_1_tmp_f9fbc_7)
                    + ((M31_3) * (cube_252_output_limb_2_col34)))
                    + (cube_252_output_limb_2_col44))
                    + (cube_252_output_limb_2_col54))
                    + (poseidon_round_keys_output_limb_2_col64))
                    - (combination_limb_2_col94))
                    * (M31_16));
                let carry_3_tmp_f9fbc_9 = (((((((carry_2_tmp_f9fbc_8)
                    + ((M31_3) * (cube_252_output_limb_3_col35)))
                    + (cube_252_output_limb_3_col45))
                    + (cube_252_output_limb_3_col55))
                    + (poseidon_round_keys_output_limb_3_col65))
                    - (combination_limb_3_col95))
                    * (M31_16));
                let carry_4_tmp_f9fbc_10 = (((((((carry_3_tmp_f9fbc_9)
                    + ((M31_3) * (cube_252_output_limb_4_col36)))
                    + (cube_252_output_limb_4_col46))
                    + (cube_252_output_limb_4_col56))
                    + (poseidon_round_keys_output_limb_4_col66))
                    - (combination_limb_4_col96))
                    * (M31_16));
                let carry_5_tmp_f9fbc_11 = (((((((carry_4_tmp_f9fbc_10)
                    + ((M31_3) * (cube_252_output_limb_5_col37)))
                    + (cube_252_output_limb_5_col47))
                    + (cube_252_output_limb_5_col57))
                    + (poseidon_round_keys_output_limb_5_col67))
                    - (combination_limb_5_col97))
                    * (M31_16));
                let carry_6_tmp_f9fbc_12 = (((((((carry_5_tmp_f9fbc_11)
                    + ((M31_3) * (cube_252_output_limb_6_col38)))
                    + (cube_252_output_limb_6_col48))
                    + (cube_252_output_limb_6_col58))
                    + (poseidon_round_keys_output_limb_6_col68))
                    - (combination_limb_6_col98))
                    * (M31_16));
                let carry_7_tmp_f9fbc_13 = ((((((((carry_6_tmp_f9fbc_12)
                    + ((M31_3) * (cube_252_output_limb_7_col39)))
                    + (cube_252_output_limb_7_col49))
                    + (cube_252_output_limb_7_col59))
                    + (poseidon_round_keys_output_limb_7_col69))
                    - (combination_limb_7_col99))
                    - ((p_coef_col102) * (M31_136)))
                    * (M31_16));
                let carry_8_tmp_f9fbc_14 = (((((((carry_7_tmp_f9fbc_13)
                    + ((M31_3) * (cube_252_output_limb_8_col40)))
                    + (cube_252_output_limb_8_col50))
                    + (cube_252_output_limb_8_col60))
                    + (poseidon_round_keys_output_limb_8_col70))
                    - (combination_limb_8_col100))
                    * (M31_16));
                *sub_component_inputs.range_check_3_3_3_3_3[0] = [
                    ((p_coef_col102) + (M31_1)),
                    ((carry_0_tmp_f9fbc_6) + (M31_1)),
                    ((carry_1_tmp_f9fbc_7) + (M31_1)),
                    ((carry_2_tmp_f9fbc_8) + (M31_1)),
                    ((carry_3_tmp_f9fbc_9) + (M31_1)),
                ];
                *lookup_data.range_check_3_3_3_3_3_0 = [
                    ((p_coef_col102) + (M31_1)),
                    ((carry_0_tmp_f9fbc_6) + (M31_1)),
                    ((carry_1_tmp_f9fbc_7) + (M31_1)),
                    ((carry_2_tmp_f9fbc_8) + (M31_1)),
                    ((carry_3_tmp_f9fbc_9) + (M31_1)),
                ];
                *sub_component_inputs.range_check_3_3_3_3_3[1] = [
                    ((carry_4_tmp_f9fbc_10) + (M31_1)),
                    ((carry_5_tmp_f9fbc_11) + (M31_1)),
                    ((carry_6_tmp_f9fbc_12) + (M31_1)),
                    ((carry_7_tmp_f9fbc_13) + (M31_1)),
                    ((carry_8_tmp_f9fbc_14) + (M31_1)),
                ];
                *lookup_data.range_check_3_3_3_3_3_1 = [
                    ((carry_4_tmp_f9fbc_10) + (M31_1)),
                    ((carry_5_tmp_f9fbc_11) + (M31_1)),
                    ((carry_6_tmp_f9fbc_12) + (M31_1)),
                    ((carry_7_tmp_f9fbc_13) + (M31_1)),
                    ((carry_8_tmp_f9fbc_14) + (M31_1)),
                ];
                let linear_combination_n_4_coefs_3_1_1_1_output_tmp_f9fbc_15 =
                    combination_tmp_f9fbc_4;

                // Linear Combination N 4 Coefs 1 M 1 1 1.

                let combination_tmp_f9fbc_16 = PackedFelt252Width27::from_packed_felt252(
                    (((((Felt252_0_0_0_0)
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_f9fbc_0,
                            ))))
                        - ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_f9fbc_1,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_f9fbc_2,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_round_keys_output_tmp_f9fbc_3[1],
                            )))),
                );
                let combination_limb_0_col103 = combination_tmp_f9fbc_16.get_m31(0);
                *row[103] = combination_limb_0_col103;
                let combination_limb_1_col104 = combination_tmp_f9fbc_16.get_m31(1);
                *row[104] = combination_limb_1_col104;
                let combination_limb_2_col105 = combination_tmp_f9fbc_16.get_m31(2);
                *row[105] = combination_limb_2_col105;
                let combination_limb_3_col106 = combination_tmp_f9fbc_16.get_m31(3);
                *row[106] = combination_limb_3_col106;
                let combination_limb_4_col107 = combination_tmp_f9fbc_16.get_m31(4);
                *row[107] = combination_limb_4_col107;
                let combination_limb_5_col108 = combination_tmp_f9fbc_16.get_m31(5);
                *row[108] = combination_limb_5_col108;
                let combination_limb_6_col109 = combination_tmp_f9fbc_16.get_m31(6);
                *row[109] = combination_limb_6_col109;
                let combination_limb_7_col110 = combination_tmp_f9fbc_16.get_m31(7);
                *row[110] = combination_limb_7_col110;
                let combination_limb_8_col111 = combination_tmp_f9fbc_16.get_m31(8);
                *row[111] = combination_limb_8_col111;
                let combination_limb_9_col112 = combination_tmp_f9fbc_16.get_m31(9);
                *row[112] = combination_limb_9_col112;
                let biased_limb_accumulator_u32_tmp_f9fbc_17 = PackedUInt32::from_m31(
                    ((((((cube_252_output_limb_0_col32) - (cube_252_output_limb_0_col42))
                        + (cube_252_output_limb_0_col52))
                        + (poseidon_round_keys_output_limb_10_col72))
                        - (combination_limb_0_col103))
                        + (M31_268435458)),
                );
                let p_coef_col113 =
                    ((biased_limb_accumulator_u32_tmp_f9fbc_17.low().as_m31()) - (M31_2));
                *row[113] = p_coef_col113;
                let carry_0_tmp_f9fbc_18 = (((((((cube_252_output_limb_0_col32)
                    - (cube_252_output_limb_0_col42))
                    + (cube_252_output_limb_0_col52))
                    + (poseidon_round_keys_output_limb_10_col72))
                    - (combination_limb_0_col103))
                    - (p_coef_col113))
                    * (M31_16));
                let carry_1_tmp_f9fbc_19 = (((((((carry_0_tmp_f9fbc_18)
                    + (cube_252_output_limb_1_col33))
                    - (cube_252_output_limb_1_col43))
                    + (cube_252_output_limb_1_col53))
                    + (poseidon_round_keys_output_limb_11_col73))
                    - (combination_limb_1_col104))
                    * (M31_16));
                let carry_2_tmp_f9fbc_20 = (((((((carry_1_tmp_f9fbc_19)
                    + (cube_252_output_limb_2_col34))
                    - (cube_252_output_limb_2_col44))
                    + (cube_252_output_limb_2_col54))
                    + (poseidon_round_keys_output_limb_12_col74))
                    - (combination_limb_2_col105))
                    * (M31_16));
                let carry_3_tmp_f9fbc_21 = (((((((carry_2_tmp_f9fbc_20)
                    + (cube_252_output_limb_3_col35))
                    - (cube_252_output_limb_3_col45))
                    + (cube_252_output_limb_3_col55))
                    + (poseidon_round_keys_output_limb_13_col75))
                    - (combination_limb_3_col106))
                    * (M31_16));
                let carry_4_tmp_f9fbc_22 = (((((((carry_3_tmp_f9fbc_21)
                    + (cube_252_output_limb_4_col36))
                    - (cube_252_output_limb_4_col46))
                    + (cube_252_output_limb_4_col56))
                    + (poseidon_round_keys_output_limb_14_col76))
                    - (combination_limb_4_col107))
                    * (M31_16));
                let carry_5_tmp_f9fbc_23 = (((((((carry_4_tmp_f9fbc_22)
                    + (cube_252_output_limb_5_col37))
                    - (cube_252_output_limb_5_col47))
                    + (cube_252_output_limb_5_col57))
                    + (poseidon_round_keys_output_limb_15_col77))
                    - (combination_limb_5_col108))
                    * (M31_16));
                let carry_6_tmp_f9fbc_24 = (((((((carry_5_tmp_f9fbc_23)
                    + (cube_252_output_limb_6_col38))
                    - (cube_252_output_limb_6_col48))
                    + (cube_252_output_limb_6_col58))
                    + (poseidon_round_keys_output_limb_16_col78))
                    - (combination_limb_6_col109))
                    * (M31_16));
                let carry_7_tmp_f9fbc_25 = ((((((((carry_6_tmp_f9fbc_24)
                    + (cube_252_output_limb_7_col39))
                    - (cube_252_output_limb_7_col49))
                    + (cube_252_output_limb_7_col59))
                    + (poseidon_round_keys_output_limb_17_col79))
                    - (combination_limb_7_col110))
                    - ((p_coef_col113) * (M31_136)))
                    * (M31_16));
                let carry_8_tmp_f9fbc_26 = (((((((carry_7_tmp_f9fbc_25)
                    + (cube_252_output_limb_8_col40))
                    - (cube_252_output_limb_8_col50))
                    + (cube_252_output_limb_8_col60))
                    + (poseidon_round_keys_output_limb_18_col80))
                    - (combination_limb_8_col111))
                    * (M31_16));
                *sub_component_inputs.range_check_3_3_3_3_3[2] = [
                    ((p_coef_col113) + (M31_2)),
                    ((carry_0_tmp_f9fbc_18) + (M31_2)),
                    ((carry_1_tmp_f9fbc_19) + (M31_2)),
                    ((carry_2_tmp_f9fbc_20) + (M31_2)),
                    ((carry_3_tmp_f9fbc_21) + (M31_2)),
                ];
                *lookup_data.range_check_3_3_3_3_3_2 = [
                    ((p_coef_col113) + (M31_2)),
                    ((carry_0_tmp_f9fbc_18) + (M31_2)),
                    ((carry_1_tmp_f9fbc_19) + (M31_2)),
                    ((carry_2_tmp_f9fbc_20) + (M31_2)),
                    ((carry_3_tmp_f9fbc_21) + (M31_2)),
                ];
                *sub_component_inputs.range_check_3_3_3_3_3[3] = [
                    ((carry_4_tmp_f9fbc_22) + (M31_2)),
                    ((carry_5_tmp_f9fbc_23) + (M31_2)),
                    ((carry_6_tmp_f9fbc_24) + (M31_2)),
                    ((carry_7_tmp_f9fbc_25) + (M31_2)),
                    ((carry_8_tmp_f9fbc_26) + (M31_2)),
                ];
                *lookup_data.range_check_3_3_3_3_3_3 = [
                    ((carry_4_tmp_f9fbc_22) + (M31_2)),
                    ((carry_5_tmp_f9fbc_23) + (M31_2)),
                    ((carry_6_tmp_f9fbc_24) + (M31_2)),
                    ((carry_7_tmp_f9fbc_25) + (M31_2)),
                    ((carry_8_tmp_f9fbc_26) + (M31_2)),
                ];
                let linear_combination_n_4_coefs_1_m1_1_1_output_tmp_f9fbc_27 =
                    combination_tmp_f9fbc_16;

                // Linear Combination N 4 Coefs 1 1 M 2 1.

                let combination_tmp_f9fbc_28 = PackedFelt252Width27::from_packed_felt252(
                    (((((Felt252_0_0_0_0)
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_f9fbc_0,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_f9fbc_1,
                            ))))
                        - ((Felt252_2_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_f9fbc_2,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_round_keys_output_tmp_f9fbc_3[2],
                            )))),
                );
                let combination_limb_0_col114 = combination_tmp_f9fbc_28.get_m31(0);
                *row[114] = combination_limb_0_col114;
                let combination_limb_1_col115 = combination_tmp_f9fbc_28.get_m31(1);
                *row[115] = combination_limb_1_col115;
                let combination_limb_2_col116 = combination_tmp_f9fbc_28.get_m31(2);
                *row[116] = combination_limb_2_col116;
                let combination_limb_3_col117 = combination_tmp_f9fbc_28.get_m31(3);
                *row[117] = combination_limb_3_col117;
                let combination_limb_4_col118 = combination_tmp_f9fbc_28.get_m31(4);
                *row[118] = combination_limb_4_col118;
                let combination_limb_5_col119 = combination_tmp_f9fbc_28.get_m31(5);
                *row[119] = combination_limb_5_col119;
                let combination_limb_6_col120 = combination_tmp_f9fbc_28.get_m31(6);
                *row[120] = combination_limb_6_col120;
                let combination_limb_7_col121 = combination_tmp_f9fbc_28.get_m31(7);
                *row[121] = combination_limb_7_col121;
                let combination_limb_8_col122 = combination_tmp_f9fbc_28.get_m31(8);
                *row[122] = combination_limb_8_col122;
                let combination_limb_9_col123 = combination_tmp_f9fbc_28.get_m31(9);
                *row[123] = combination_limb_9_col123;
                let biased_limb_accumulator_u32_tmp_f9fbc_29 = PackedUInt32::from_m31(
                    ((((((cube_252_output_limb_0_col32) + (cube_252_output_limb_0_col42))
                        - ((M31_2) * (cube_252_output_limb_0_col52)))
                        + (poseidon_round_keys_output_limb_20_col82))
                        - (combination_limb_0_col114))
                        + (M31_402653187)),
                );
                let p_coef_col124 =
                    ((biased_limb_accumulator_u32_tmp_f9fbc_29.low().as_m31()) - (M31_3));
                *row[124] = p_coef_col124;
                let carry_0_tmp_f9fbc_30 = (((((((cube_252_output_limb_0_col32)
                    + (cube_252_output_limb_0_col42))
                    - ((M31_2) * (cube_252_output_limb_0_col52)))
                    + (poseidon_round_keys_output_limb_20_col82))
                    - (combination_limb_0_col114))
                    - (p_coef_col124))
                    * (M31_16));
                let carry_1_tmp_f9fbc_31 = (((((((carry_0_tmp_f9fbc_30)
                    + (cube_252_output_limb_1_col33))
                    + (cube_252_output_limb_1_col43))
                    - ((M31_2) * (cube_252_output_limb_1_col53)))
                    + (poseidon_round_keys_output_limb_21_col83))
                    - (combination_limb_1_col115))
                    * (M31_16));
                let carry_2_tmp_f9fbc_32 = (((((((carry_1_tmp_f9fbc_31)
                    + (cube_252_output_limb_2_col34))
                    + (cube_252_output_limb_2_col44))
                    - ((M31_2) * (cube_252_output_limb_2_col54)))
                    + (poseidon_round_keys_output_limb_22_col84))
                    - (combination_limb_2_col116))
                    * (M31_16));
                let carry_3_tmp_f9fbc_33 = (((((((carry_2_tmp_f9fbc_32)
                    + (cube_252_output_limb_3_col35))
                    + (cube_252_output_limb_3_col45))
                    - ((M31_2) * (cube_252_output_limb_3_col55)))
                    + (poseidon_round_keys_output_limb_23_col85))
                    - (combination_limb_3_col117))
                    * (M31_16));
                let carry_4_tmp_f9fbc_34 = (((((((carry_3_tmp_f9fbc_33)
                    + (cube_252_output_limb_4_col36))
                    + (cube_252_output_limb_4_col46))
                    - ((M31_2) * (cube_252_output_limb_4_col56)))
                    + (poseidon_round_keys_output_limb_24_col86))
                    - (combination_limb_4_col118))
                    * (M31_16));
                let carry_5_tmp_f9fbc_35 = (((((((carry_4_tmp_f9fbc_34)
                    + (cube_252_output_limb_5_col37))
                    + (cube_252_output_limb_5_col47))
                    - ((M31_2) * (cube_252_output_limb_5_col57)))
                    + (poseidon_round_keys_output_limb_25_col87))
                    - (combination_limb_5_col119))
                    * (M31_16));
                let carry_6_tmp_f9fbc_36 = (((((((carry_5_tmp_f9fbc_35)
                    + (cube_252_output_limb_6_col38))
                    + (cube_252_output_limb_6_col48))
                    - ((M31_2) * (cube_252_output_limb_6_col58)))
                    + (poseidon_round_keys_output_limb_26_col88))
                    - (combination_limb_6_col120))
                    * (M31_16));
                let carry_7_tmp_f9fbc_37 = ((((((((carry_6_tmp_f9fbc_36)
                    + (cube_252_output_limb_7_col39))
                    + (cube_252_output_limb_7_col49))
                    - ((M31_2) * (cube_252_output_limb_7_col59)))
                    + (poseidon_round_keys_output_limb_27_col89))
                    - (combination_limb_7_col121))
                    - ((p_coef_col124) * (M31_136)))
                    * (M31_16));
                let carry_8_tmp_f9fbc_38 = (((((((carry_7_tmp_f9fbc_37)
                    + (cube_252_output_limb_8_col40))
                    + (cube_252_output_limb_8_col50))
                    - ((M31_2) * (cube_252_output_limb_8_col60)))
                    + (poseidon_round_keys_output_limb_28_col90))
                    - (combination_limb_8_col122))
                    * (M31_16));
                *sub_component_inputs.range_check_3_3_3_3_3[4] = [
                    ((p_coef_col124) + (M31_3)),
                    ((carry_0_tmp_f9fbc_30) + (M31_3)),
                    ((carry_1_tmp_f9fbc_31) + (M31_3)),
                    ((carry_2_tmp_f9fbc_32) + (M31_3)),
                    ((carry_3_tmp_f9fbc_33) + (M31_3)),
                ];
                *lookup_data.range_check_3_3_3_3_3_4 = [
                    ((p_coef_col124) + (M31_3)),
                    ((carry_0_tmp_f9fbc_30) + (M31_3)),
                    ((carry_1_tmp_f9fbc_31) + (M31_3)),
                    ((carry_2_tmp_f9fbc_32) + (M31_3)),
                    ((carry_3_tmp_f9fbc_33) + (M31_3)),
                ];
                *sub_component_inputs.range_check_3_3_3_3_3[5] = [
                    ((carry_4_tmp_f9fbc_34) + (M31_3)),
                    ((carry_5_tmp_f9fbc_35) + (M31_3)),
                    ((carry_6_tmp_f9fbc_36) + (M31_3)),
                    ((carry_7_tmp_f9fbc_37) + (M31_3)),
                    ((carry_8_tmp_f9fbc_38) + (M31_3)),
                ];
                *lookup_data.range_check_3_3_3_3_3_5 = [
                    ((carry_4_tmp_f9fbc_34) + (M31_3)),
                    ((carry_5_tmp_f9fbc_35) + (M31_3)),
                    ((carry_6_tmp_f9fbc_36) + (M31_3)),
                    ((carry_7_tmp_f9fbc_37) + (M31_3)),
                    ((carry_8_tmp_f9fbc_38) + (M31_3)),
                ];
                let linear_combination_n_4_coefs_1_1_m2_1_output_tmp_f9fbc_39 =
                    combination_tmp_f9fbc_28;

                *lookup_data.poseidon_full_round_chain_0 = [
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
                ];
                *lookup_data.poseidon_full_round_chain_1 = [
                    input_limb_0_col0,
                    ((input_limb_1_col1) + (M31_1)),
                    combination_limb_0_col92,
                    combination_limb_1_col93,
                    combination_limb_2_col94,
                    combination_limb_3_col95,
                    combination_limb_4_col96,
                    combination_limb_5_col97,
                    combination_limb_6_col98,
                    combination_limb_7_col99,
                    combination_limb_8_col100,
                    combination_limb_9_col101,
                    combination_limb_0_col103,
                    combination_limb_1_col104,
                    combination_limb_2_col105,
                    combination_limb_3_col106,
                    combination_limb_4_col107,
                    combination_limb_5_col108,
                    combination_limb_6_col109,
                    combination_limb_7_col110,
                    combination_limb_8_col111,
                    combination_limb_9_col112,
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
                *row[125] = enabler_col.packed_at(row_index);
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    cube_252_0: Vec<[PackedM31; 20]>,
    cube_252_1: Vec<[PackedM31; 20]>,
    cube_252_2: Vec<[PackedM31; 20]>,
    poseidon_full_round_chain_0: Vec<[PackedM31; 32]>,
    poseidon_full_round_chain_1: Vec<[PackedM31; 32]>,
    poseidon_round_keys_0: Vec<[PackedM31; 31]>,
    range_check_3_3_3_3_3_0: Vec<[PackedM31; 5]>,
    range_check_3_3_3_3_3_1: Vec<[PackedM31; 5]>,
    range_check_3_3_3_3_3_2: Vec<[PackedM31; 5]>,
    range_check_3_3_3_3_3_3: Vec<[PackedM31; 5]>,
    range_check_3_3_3_3_3_4: Vec<[PackedM31; 5]>,
    range_check_3_3_3_3_3_5: Vec<[PackedM31; 5]>,
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
        poseidon_round_keys: &relations::PoseidonRoundKeys,
        range_check_3_3_3_3_3: &relations::RangeCheck_3_3_3_3_3,
        poseidon_full_round_chain: &relations::PoseidonFullRoundChain,
    ) -> InteractionClaim {
        let enabler_col = Enabler::new(self.n_rows);
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.cube_252_0,
            &self.lookup_data.cube_252_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = cube_252.combine(values0);
                let denom1: PackedQM31 = cube_252.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.cube_252_2,
            &self.lookup_data.poseidon_round_keys_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = cube_252.combine(values0);
                let denom1: PackedQM31 = poseidon_round_keys.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_3_3_3_3_3_0,
            &self.lookup_data.range_check_3_3_3_3_3_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_3_3_3_3_3.combine(values0);
                let denom1: PackedQM31 = range_check_3_3_3_3_3.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_3_3_3_3_3_2,
            &self.lookup_data.range_check_3_3_3_3_3_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_3_3_3_3_3.combine(values0);
                let denom1: PackedQM31 = range_check_3_3_3_3_3.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_3_3_3_3_3_4,
            &self.lookup_data.range_check_3_3_3_3_3_5,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_3_3_3_3_3.combine(values0);
                let denom1: PackedQM31 = range_check_3_3_3_3_3.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.poseidon_full_round_chain_0,
            &self.lookup_data.poseidon_full_round_chain_1,
        )
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (writer, values0, values1))| {
                let denom0: PackedQM31 = poseidon_full_round_chain.combine(values0);
                let denom1: PackedQM31 = poseidon_full_round_chain.combine(values1);
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
