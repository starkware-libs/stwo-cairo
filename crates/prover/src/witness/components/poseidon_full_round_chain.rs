// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::poseidon_full_round_chain::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{cube_252, poseidon_round_keys, range_check_3_3_3_3_3};
use crate::witness::prelude::*;

pub type InputType = (M31, M31, [Felt252Width27; 3]);
pub type PackedInputType = (PackedM31, PackedM31, [PackedFelt252Width27; 3]);

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
        cube_252_state: &cube_252::ClaimGenerator,
        poseidon_round_keys_state: &poseidon_round_keys::ClaimGenerator,
        range_check_3_3_3_3_3_state: &range_check_3_3_3_3_3::ClaimGenerator,
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

        let (trace, lookup_data, sub_component_inputs) = write_trace_simd(
            packed_inputs,
            n_active_rows,
            cube_252_state,
            poseidon_round_keys_state,
            range_check_3_3_3_3_3_state,
        );
        for inputs in sub_component_inputs.cube_252 {
            add_inputs(cube_252_state, &inputs, n_active_rows, 0);
        }
        for inputs in sub_component_inputs.poseidon_round_keys {
            add_inputs(poseidon_round_keys_state, &inputs, n_active_rows, 0);
        }
        for inputs in sub_component_inputs.range_check_3_3_3_3_3 {
            add_inputs(range_check_3_3_3_3_3_state, &inputs, n_active_rows, 0);
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
    cube_252_state: &cube_252::ClaimGenerator,
    poseidon_round_keys_state: &poseidon_round_keys::ClaimGenerator,
    range_check_3_3_3_3_3_state: &range_check_3_3_3_3_3::ClaimGenerator,
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

    let Felt252_0_0_0_0 = PackedFelt252::broadcast(Felt252::from([0, 0, 0, 0]));
    let Felt252_1_0_0_0 = PackedFelt252::broadcast(Felt252::from([1, 0, 0, 0]));
    let Felt252_2_0_0_0 = PackedFelt252::broadcast(Felt252::from([2, 0, 0, 0]));
    let Felt252_3_0_0_0 = PackedFelt252::broadcast(Felt252::from([3, 0, 0, 0]));
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_1024310512 = PackedM31::broadcast(M31::from(1024310512));
    let M31_134217729 = PackedM31::broadcast(M31::from(134217729));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_1480369132 = PackedM31::broadcast(M31::from(1480369132));
    let M31_16 = PackedM31::broadcast(M31::from(16));
    let M31_1987997202 = PackedM31::broadcast(M31::from(1987997202));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_268435458 = PackedM31::broadcast(M31::from(268435458));
    let M31_3 = PackedM31::broadcast(M31::from(3));
    let M31_402653187 = PackedM31::broadcast(M31::from(402653187));
    let M31_502259093 = PackedM31::broadcast(M31::from(502259093));
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
                let enabler_col0 = enabler_col.packed_at(row_index);
                *row[0] = enabler_col0;
                let input_limb_0_col1 = poseidon_full_round_chain_input.0;
                *row[1] = input_limb_0_col1;
                let input_limb_1_col2 = poseidon_full_round_chain_input.1;
                *row[2] = input_limb_1_col2;
                let input_limb_2_col3 = poseidon_full_round_chain_input.2[0].get_m31(0);
                *row[3] = input_limb_2_col3;
                let input_limb_3_col4 = poseidon_full_round_chain_input.2[0].get_m31(1);
                *row[4] = input_limb_3_col4;
                let input_limb_4_col5 = poseidon_full_round_chain_input.2[0].get_m31(2);
                *row[5] = input_limb_4_col5;
                let input_limb_5_col6 = poseidon_full_round_chain_input.2[0].get_m31(3);
                *row[6] = input_limb_5_col6;
                let input_limb_6_col7 = poseidon_full_round_chain_input.2[0].get_m31(4);
                *row[7] = input_limb_6_col7;
                let input_limb_7_col8 = poseidon_full_round_chain_input.2[0].get_m31(5);
                *row[8] = input_limb_7_col8;
                let input_limb_8_col9 = poseidon_full_round_chain_input.2[0].get_m31(6);
                *row[9] = input_limb_8_col9;
                let input_limb_9_col10 = poseidon_full_round_chain_input.2[0].get_m31(7);
                *row[10] = input_limb_9_col10;
                let input_limb_10_col11 = poseidon_full_round_chain_input.2[0].get_m31(8);
                *row[11] = input_limb_10_col11;
                let input_limb_11_col12 = poseidon_full_round_chain_input.2[0].get_m31(9);
                *row[12] = input_limb_11_col12;
                let input_limb_12_col13 = poseidon_full_round_chain_input.2[1].get_m31(0);
                *row[13] = input_limb_12_col13;
                let input_limb_13_col14 = poseidon_full_round_chain_input.2[1].get_m31(1);
                *row[14] = input_limb_13_col14;
                let input_limb_14_col15 = poseidon_full_round_chain_input.2[1].get_m31(2);
                *row[15] = input_limb_14_col15;
                let input_limb_15_col16 = poseidon_full_round_chain_input.2[1].get_m31(3);
                *row[16] = input_limb_15_col16;
                let input_limb_16_col17 = poseidon_full_round_chain_input.2[1].get_m31(4);
                *row[17] = input_limb_16_col17;
                let input_limb_17_col18 = poseidon_full_round_chain_input.2[1].get_m31(5);
                *row[18] = input_limb_17_col18;
                let input_limb_18_col19 = poseidon_full_round_chain_input.2[1].get_m31(6);
                *row[19] = input_limb_18_col19;
                let input_limb_19_col20 = poseidon_full_round_chain_input.2[1].get_m31(7);
                *row[20] = input_limb_19_col20;
                let input_limb_20_col21 = poseidon_full_round_chain_input.2[1].get_m31(8);
                *row[21] = input_limb_20_col21;
                let input_limb_21_col22 = poseidon_full_round_chain_input.2[1].get_m31(9);
                *row[22] = input_limb_21_col22;
                let input_limb_22_col23 = poseidon_full_round_chain_input.2[2].get_m31(0);
                *row[23] = input_limb_22_col23;
                let input_limb_23_col24 = poseidon_full_round_chain_input.2[2].get_m31(1);
                *row[24] = input_limb_23_col24;
                let input_limb_24_col25 = poseidon_full_round_chain_input.2[2].get_m31(2);
                *row[25] = input_limb_24_col25;
                let input_limb_25_col26 = poseidon_full_round_chain_input.2[2].get_m31(3);
                *row[26] = input_limb_25_col26;
                let input_limb_26_col27 = poseidon_full_round_chain_input.2[2].get_m31(4);
                *row[27] = input_limb_26_col27;
                let input_limb_27_col28 = poseidon_full_round_chain_input.2[2].get_m31(5);
                *row[28] = input_limb_27_col28;
                let input_limb_28_col29 = poseidon_full_round_chain_input.2[2].get_m31(6);
                *row[29] = input_limb_28_col29;
                let input_limb_29_col30 = poseidon_full_round_chain_input.2[2].get_m31(7);
                *row[30] = input_limb_29_col30;
                let input_limb_30_col31 = poseidon_full_round_chain_input.2[2].get_m31(8);
                *row[31] = input_limb_30_col31;
                let input_limb_31_col32 = poseidon_full_round_chain_input.2[2].get_m31(9);
                *row[32] = input_limb_31_col32;
                *sub_component_inputs.cube_252[0] = poseidon_full_round_chain_input.2[0];
                let cube_252_output_tmp_1400f_0 =
                    PackedCube252::deduce_output(poseidon_full_round_chain_input.2[0]);
                let cube_252_output_limb_0_col33 = cube_252_output_tmp_1400f_0.get_m31(0);
                *row[33] = cube_252_output_limb_0_col33;
                let cube_252_output_limb_1_col34 = cube_252_output_tmp_1400f_0.get_m31(1);
                *row[34] = cube_252_output_limb_1_col34;
                let cube_252_output_limb_2_col35 = cube_252_output_tmp_1400f_0.get_m31(2);
                *row[35] = cube_252_output_limb_2_col35;
                let cube_252_output_limb_3_col36 = cube_252_output_tmp_1400f_0.get_m31(3);
                *row[36] = cube_252_output_limb_3_col36;
                let cube_252_output_limb_4_col37 = cube_252_output_tmp_1400f_0.get_m31(4);
                *row[37] = cube_252_output_limb_4_col37;
                let cube_252_output_limb_5_col38 = cube_252_output_tmp_1400f_0.get_m31(5);
                *row[38] = cube_252_output_limb_5_col38;
                let cube_252_output_limb_6_col39 = cube_252_output_tmp_1400f_0.get_m31(6);
                *row[39] = cube_252_output_limb_6_col39;
                let cube_252_output_limb_7_col40 = cube_252_output_tmp_1400f_0.get_m31(7);
                *row[40] = cube_252_output_limb_7_col40;
                let cube_252_output_limb_8_col41 = cube_252_output_tmp_1400f_0.get_m31(8);
                *row[41] = cube_252_output_limb_8_col41;
                let cube_252_output_limb_9_col42 = cube_252_output_tmp_1400f_0.get_m31(9);
                *row[42] = cube_252_output_limb_9_col42;
                *lookup_data.cube_252_0 = [
                    M31_1987997202,
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
                    cube_252_output_limb_0_col33,
                    cube_252_output_limb_1_col34,
                    cube_252_output_limb_2_col35,
                    cube_252_output_limb_3_col36,
                    cube_252_output_limb_4_col37,
                    cube_252_output_limb_5_col38,
                    cube_252_output_limb_6_col39,
                    cube_252_output_limb_7_col40,
                    cube_252_output_limb_8_col41,
                    cube_252_output_limb_9_col42,
                ];
                *sub_component_inputs.cube_252[1] = poseidon_full_round_chain_input.2[1];
                let cube_252_output_tmp_1400f_1 =
                    PackedCube252::deduce_output(poseidon_full_round_chain_input.2[1]);
                let cube_252_output_limb_0_col43 = cube_252_output_tmp_1400f_1.get_m31(0);
                *row[43] = cube_252_output_limb_0_col43;
                let cube_252_output_limb_1_col44 = cube_252_output_tmp_1400f_1.get_m31(1);
                *row[44] = cube_252_output_limb_1_col44;
                let cube_252_output_limb_2_col45 = cube_252_output_tmp_1400f_1.get_m31(2);
                *row[45] = cube_252_output_limb_2_col45;
                let cube_252_output_limb_3_col46 = cube_252_output_tmp_1400f_1.get_m31(3);
                *row[46] = cube_252_output_limb_3_col46;
                let cube_252_output_limb_4_col47 = cube_252_output_tmp_1400f_1.get_m31(4);
                *row[47] = cube_252_output_limb_4_col47;
                let cube_252_output_limb_5_col48 = cube_252_output_tmp_1400f_1.get_m31(5);
                *row[48] = cube_252_output_limb_5_col48;
                let cube_252_output_limb_6_col49 = cube_252_output_tmp_1400f_1.get_m31(6);
                *row[49] = cube_252_output_limb_6_col49;
                let cube_252_output_limb_7_col50 = cube_252_output_tmp_1400f_1.get_m31(7);
                *row[50] = cube_252_output_limb_7_col50;
                let cube_252_output_limb_8_col51 = cube_252_output_tmp_1400f_1.get_m31(8);
                *row[51] = cube_252_output_limb_8_col51;
                let cube_252_output_limb_9_col52 = cube_252_output_tmp_1400f_1.get_m31(9);
                *row[52] = cube_252_output_limb_9_col52;
                *lookup_data.cube_252_1 = [
                    M31_1987997202,
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
                    cube_252_output_limb_0_col43,
                    cube_252_output_limb_1_col44,
                    cube_252_output_limb_2_col45,
                    cube_252_output_limb_3_col46,
                    cube_252_output_limb_4_col47,
                    cube_252_output_limb_5_col48,
                    cube_252_output_limb_6_col49,
                    cube_252_output_limb_7_col50,
                    cube_252_output_limb_8_col51,
                    cube_252_output_limb_9_col52,
                ];
                *sub_component_inputs.cube_252[2] = poseidon_full_round_chain_input.2[2];
                let cube_252_output_tmp_1400f_2 =
                    PackedCube252::deduce_output(poseidon_full_round_chain_input.2[2]);
                let cube_252_output_limb_0_col53 = cube_252_output_tmp_1400f_2.get_m31(0);
                *row[53] = cube_252_output_limb_0_col53;
                let cube_252_output_limb_1_col54 = cube_252_output_tmp_1400f_2.get_m31(1);
                *row[54] = cube_252_output_limb_1_col54;
                let cube_252_output_limb_2_col55 = cube_252_output_tmp_1400f_2.get_m31(2);
                *row[55] = cube_252_output_limb_2_col55;
                let cube_252_output_limb_3_col56 = cube_252_output_tmp_1400f_2.get_m31(3);
                *row[56] = cube_252_output_limb_3_col56;
                let cube_252_output_limb_4_col57 = cube_252_output_tmp_1400f_2.get_m31(4);
                *row[57] = cube_252_output_limb_4_col57;
                let cube_252_output_limb_5_col58 = cube_252_output_tmp_1400f_2.get_m31(5);
                *row[58] = cube_252_output_limb_5_col58;
                let cube_252_output_limb_6_col59 = cube_252_output_tmp_1400f_2.get_m31(6);
                *row[59] = cube_252_output_limb_6_col59;
                let cube_252_output_limb_7_col60 = cube_252_output_tmp_1400f_2.get_m31(7);
                *row[60] = cube_252_output_limb_7_col60;
                let cube_252_output_limb_8_col61 = cube_252_output_tmp_1400f_2.get_m31(8);
                *row[61] = cube_252_output_limb_8_col61;
                let cube_252_output_limb_9_col62 = cube_252_output_tmp_1400f_2.get_m31(9);
                *row[62] = cube_252_output_limb_9_col62;
                *lookup_data.cube_252_2 = [
                    M31_1987997202,
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
                    cube_252_output_limb_0_col53,
                    cube_252_output_limb_1_col54,
                    cube_252_output_limb_2_col55,
                    cube_252_output_limb_3_col56,
                    cube_252_output_limb_4_col57,
                    cube_252_output_limb_5_col58,
                    cube_252_output_limb_6_col59,
                    cube_252_output_limb_7_col60,
                    cube_252_output_limb_8_col61,
                    cube_252_output_limb_9_col62,
                ];
                *sub_component_inputs.poseidon_round_keys[0] = [input_limb_1_col2];
                let poseidon_round_keys_output_tmp_1400f_3 =
                    PackedPoseidonRoundKeys::deduce_output([input_limb_1_col2]);
                let poseidon_round_keys_output_limb_0_col63 =
                    poseidon_round_keys_output_tmp_1400f_3[0].get_m31(0);
                *row[63] = poseidon_round_keys_output_limb_0_col63;
                let poseidon_round_keys_output_limb_1_col64 =
                    poseidon_round_keys_output_tmp_1400f_3[0].get_m31(1);
                *row[64] = poseidon_round_keys_output_limb_1_col64;
                let poseidon_round_keys_output_limb_2_col65 =
                    poseidon_round_keys_output_tmp_1400f_3[0].get_m31(2);
                *row[65] = poseidon_round_keys_output_limb_2_col65;
                let poseidon_round_keys_output_limb_3_col66 =
                    poseidon_round_keys_output_tmp_1400f_3[0].get_m31(3);
                *row[66] = poseidon_round_keys_output_limb_3_col66;
                let poseidon_round_keys_output_limb_4_col67 =
                    poseidon_round_keys_output_tmp_1400f_3[0].get_m31(4);
                *row[67] = poseidon_round_keys_output_limb_4_col67;
                let poseidon_round_keys_output_limb_5_col68 =
                    poseidon_round_keys_output_tmp_1400f_3[0].get_m31(5);
                *row[68] = poseidon_round_keys_output_limb_5_col68;
                let poseidon_round_keys_output_limb_6_col69 =
                    poseidon_round_keys_output_tmp_1400f_3[0].get_m31(6);
                *row[69] = poseidon_round_keys_output_limb_6_col69;
                let poseidon_round_keys_output_limb_7_col70 =
                    poseidon_round_keys_output_tmp_1400f_3[0].get_m31(7);
                *row[70] = poseidon_round_keys_output_limb_7_col70;
                let poseidon_round_keys_output_limb_8_col71 =
                    poseidon_round_keys_output_tmp_1400f_3[0].get_m31(8);
                *row[71] = poseidon_round_keys_output_limb_8_col71;
                let poseidon_round_keys_output_limb_9_col72 =
                    poseidon_round_keys_output_tmp_1400f_3[0].get_m31(9);
                *row[72] = poseidon_round_keys_output_limb_9_col72;
                let poseidon_round_keys_output_limb_10_col73 =
                    poseidon_round_keys_output_tmp_1400f_3[1].get_m31(0);
                *row[73] = poseidon_round_keys_output_limb_10_col73;
                let poseidon_round_keys_output_limb_11_col74 =
                    poseidon_round_keys_output_tmp_1400f_3[1].get_m31(1);
                *row[74] = poseidon_round_keys_output_limb_11_col74;
                let poseidon_round_keys_output_limb_12_col75 =
                    poseidon_round_keys_output_tmp_1400f_3[1].get_m31(2);
                *row[75] = poseidon_round_keys_output_limb_12_col75;
                let poseidon_round_keys_output_limb_13_col76 =
                    poseidon_round_keys_output_tmp_1400f_3[1].get_m31(3);
                *row[76] = poseidon_round_keys_output_limb_13_col76;
                let poseidon_round_keys_output_limb_14_col77 =
                    poseidon_round_keys_output_tmp_1400f_3[1].get_m31(4);
                *row[77] = poseidon_round_keys_output_limb_14_col77;
                let poseidon_round_keys_output_limb_15_col78 =
                    poseidon_round_keys_output_tmp_1400f_3[1].get_m31(5);
                *row[78] = poseidon_round_keys_output_limb_15_col78;
                let poseidon_round_keys_output_limb_16_col79 =
                    poseidon_round_keys_output_tmp_1400f_3[1].get_m31(6);
                *row[79] = poseidon_round_keys_output_limb_16_col79;
                let poseidon_round_keys_output_limb_17_col80 =
                    poseidon_round_keys_output_tmp_1400f_3[1].get_m31(7);
                *row[80] = poseidon_round_keys_output_limb_17_col80;
                let poseidon_round_keys_output_limb_18_col81 =
                    poseidon_round_keys_output_tmp_1400f_3[1].get_m31(8);
                *row[81] = poseidon_round_keys_output_limb_18_col81;
                let poseidon_round_keys_output_limb_19_col82 =
                    poseidon_round_keys_output_tmp_1400f_3[1].get_m31(9);
                *row[82] = poseidon_round_keys_output_limb_19_col82;
                let poseidon_round_keys_output_limb_20_col83 =
                    poseidon_round_keys_output_tmp_1400f_3[2].get_m31(0);
                *row[83] = poseidon_round_keys_output_limb_20_col83;
                let poseidon_round_keys_output_limb_21_col84 =
                    poseidon_round_keys_output_tmp_1400f_3[2].get_m31(1);
                *row[84] = poseidon_round_keys_output_limb_21_col84;
                let poseidon_round_keys_output_limb_22_col85 =
                    poseidon_round_keys_output_tmp_1400f_3[2].get_m31(2);
                *row[85] = poseidon_round_keys_output_limb_22_col85;
                let poseidon_round_keys_output_limb_23_col86 =
                    poseidon_round_keys_output_tmp_1400f_3[2].get_m31(3);
                *row[86] = poseidon_round_keys_output_limb_23_col86;
                let poseidon_round_keys_output_limb_24_col87 =
                    poseidon_round_keys_output_tmp_1400f_3[2].get_m31(4);
                *row[87] = poseidon_round_keys_output_limb_24_col87;
                let poseidon_round_keys_output_limb_25_col88 =
                    poseidon_round_keys_output_tmp_1400f_3[2].get_m31(5);
                *row[88] = poseidon_round_keys_output_limb_25_col88;
                let poseidon_round_keys_output_limb_26_col89 =
                    poseidon_round_keys_output_tmp_1400f_3[2].get_m31(6);
                *row[89] = poseidon_round_keys_output_limb_26_col89;
                let poseidon_round_keys_output_limb_27_col90 =
                    poseidon_round_keys_output_tmp_1400f_3[2].get_m31(7);
                *row[90] = poseidon_round_keys_output_limb_27_col90;
                let poseidon_round_keys_output_limb_28_col91 =
                    poseidon_round_keys_output_tmp_1400f_3[2].get_m31(8);
                *row[91] = poseidon_round_keys_output_limb_28_col91;
                let poseidon_round_keys_output_limb_29_col92 =
                    poseidon_round_keys_output_tmp_1400f_3[2].get_m31(9);
                *row[92] = poseidon_round_keys_output_limb_29_col92;
                *lookup_data.poseidon_round_keys_3 = [
                    M31_1024310512,
                    input_limb_1_col2,
                    poseidon_round_keys_output_limb_0_col63,
                    poseidon_round_keys_output_limb_1_col64,
                    poseidon_round_keys_output_limb_2_col65,
                    poseidon_round_keys_output_limb_3_col66,
                    poseidon_round_keys_output_limb_4_col67,
                    poseidon_round_keys_output_limb_5_col68,
                    poseidon_round_keys_output_limb_6_col69,
                    poseidon_round_keys_output_limb_7_col70,
                    poseidon_round_keys_output_limb_8_col71,
                    poseidon_round_keys_output_limb_9_col72,
                    poseidon_round_keys_output_limb_10_col73,
                    poseidon_round_keys_output_limb_11_col74,
                    poseidon_round_keys_output_limb_12_col75,
                    poseidon_round_keys_output_limb_13_col76,
                    poseidon_round_keys_output_limb_14_col77,
                    poseidon_round_keys_output_limb_15_col78,
                    poseidon_round_keys_output_limb_16_col79,
                    poseidon_round_keys_output_limb_17_col80,
                    poseidon_round_keys_output_limb_18_col81,
                    poseidon_round_keys_output_limb_19_col82,
                    poseidon_round_keys_output_limb_20_col83,
                    poseidon_round_keys_output_limb_21_col84,
                    poseidon_round_keys_output_limb_22_col85,
                    poseidon_round_keys_output_limb_23_col86,
                    poseidon_round_keys_output_limb_24_col87,
                    poseidon_round_keys_output_limb_25_col88,
                    poseidon_round_keys_output_limb_26_col89,
                    poseidon_round_keys_output_limb_27_col90,
                    poseidon_round_keys_output_limb_28_col91,
                    poseidon_round_keys_output_limb_29_col92,
                ];

                // Linear Combination N 4 Coefs 3 1 1 1.

                let combination_tmp_1400f_4 = PackedFelt252Width27::from_packed_felt252(
                    (((((Felt252_0_0_0_0)
                        + ((Felt252_3_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_1400f_0,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_1400f_1,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_1400f_2,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_round_keys_output_tmp_1400f_3[0],
                            )))),
                );
                let combination_limb_0_col93 = combination_tmp_1400f_4.get_m31(0);
                *row[93] = combination_limb_0_col93;
                let combination_limb_1_col94 = combination_tmp_1400f_4.get_m31(1);
                *row[94] = combination_limb_1_col94;
                let combination_limb_2_col95 = combination_tmp_1400f_4.get_m31(2);
                *row[95] = combination_limb_2_col95;
                let combination_limb_3_col96 = combination_tmp_1400f_4.get_m31(3);
                *row[96] = combination_limb_3_col96;
                let combination_limb_4_col97 = combination_tmp_1400f_4.get_m31(4);
                *row[97] = combination_limb_4_col97;
                let combination_limb_5_col98 = combination_tmp_1400f_4.get_m31(5);
                *row[98] = combination_limb_5_col98;
                let combination_limb_6_col99 = combination_tmp_1400f_4.get_m31(6);
                *row[99] = combination_limb_6_col99;
                let combination_limb_7_col100 = combination_tmp_1400f_4.get_m31(7);
                *row[100] = combination_limb_7_col100;
                let combination_limb_8_col101 = combination_tmp_1400f_4.get_m31(8);
                *row[101] = combination_limb_8_col101;
                let combination_limb_9_col102 = combination_tmp_1400f_4.get_m31(9);
                *row[102] = combination_limb_9_col102;
                let biased_limb_accumulator_u32_tmp_1400f_5 = PackedUInt32::from_m31(
                    (((((((M31_3) * (cube_252_output_limb_0_col33))
                        + (cube_252_output_limb_0_col43))
                        + (cube_252_output_limb_0_col53))
                        + (poseidon_round_keys_output_limb_0_col63))
                        - (combination_limb_0_col93))
                        + (M31_134217729)),
                );
                let p_coef_col103 =
                    ((biased_limb_accumulator_u32_tmp_1400f_5.low().as_m31()) - (M31_1));
                *row[103] = p_coef_col103;
                let carry_0_tmp_1400f_6 = ((((((((M31_3) * (cube_252_output_limb_0_col33))
                    + (cube_252_output_limb_0_col43))
                    + (cube_252_output_limb_0_col53))
                    + (poseidon_round_keys_output_limb_0_col63))
                    - (combination_limb_0_col93))
                    - (p_coef_col103))
                    * (M31_16));
                let carry_1_tmp_1400f_7 = (((((((carry_0_tmp_1400f_6)
                    + ((M31_3) * (cube_252_output_limb_1_col34)))
                    + (cube_252_output_limb_1_col44))
                    + (cube_252_output_limb_1_col54))
                    + (poseidon_round_keys_output_limb_1_col64))
                    - (combination_limb_1_col94))
                    * (M31_16));
                let carry_2_tmp_1400f_8 = (((((((carry_1_tmp_1400f_7)
                    + ((M31_3) * (cube_252_output_limb_2_col35)))
                    + (cube_252_output_limb_2_col45))
                    + (cube_252_output_limb_2_col55))
                    + (poseidon_round_keys_output_limb_2_col65))
                    - (combination_limb_2_col95))
                    * (M31_16));
                let carry_3_tmp_1400f_9 = (((((((carry_2_tmp_1400f_8)
                    + ((M31_3) * (cube_252_output_limb_3_col36)))
                    + (cube_252_output_limb_3_col46))
                    + (cube_252_output_limb_3_col56))
                    + (poseidon_round_keys_output_limb_3_col66))
                    - (combination_limb_3_col96))
                    * (M31_16));
                let carry_4_tmp_1400f_10 = (((((((carry_3_tmp_1400f_9)
                    + ((M31_3) * (cube_252_output_limb_4_col37)))
                    + (cube_252_output_limb_4_col47))
                    + (cube_252_output_limb_4_col57))
                    + (poseidon_round_keys_output_limb_4_col67))
                    - (combination_limb_4_col97))
                    * (M31_16));
                let carry_5_tmp_1400f_11 = (((((((carry_4_tmp_1400f_10)
                    + ((M31_3) * (cube_252_output_limb_5_col38)))
                    + (cube_252_output_limb_5_col48))
                    + (cube_252_output_limb_5_col58))
                    + (poseidon_round_keys_output_limb_5_col68))
                    - (combination_limb_5_col98))
                    * (M31_16));
                let carry_6_tmp_1400f_12 = (((((((carry_5_tmp_1400f_11)
                    + ((M31_3) * (cube_252_output_limb_6_col39)))
                    + (cube_252_output_limb_6_col49))
                    + (cube_252_output_limb_6_col59))
                    + (poseidon_round_keys_output_limb_6_col69))
                    - (combination_limb_6_col99))
                    * (M31_16));
                let carry_7_tmp_1400f_13 = ((((((((carry_6_tmp_1400f_12)
                    + ((M31_3) * (cube_252_output_limb_7_col40)))
                    + (cube_252_output_limb_7_col50))
                    + (cube_252_output_limb_7_col60))
                    + (poseidon_round_keys_output_limb_7_col70))
                    - (combination_limb_7_col100))
                    - ((p_coef_col103) * (M31_136)))
                    * (M31_16));
                let carry_8_tmp_1400f_14 = (((((((carry_7_tmp_1400f_13)
                    + ((M31_3) * (cube_252_output_limb_8_col41)))
                    + (cube_252_output_limb_8_col51))
                    + (cube_252_output_limb_8_col61))
                    + (poseidon_round_keys_output_limb_8_col71))
                    - (combination_limb_8_col101))
                    * (M31_16));
                *sub_component_inputs.range_check_3_3_3_3_3[0] = [
                    ((p_coef_col103) + (M31_1)),
                    ((carry_0_tmp_1400f_6) + (M31_1)),
                    ((carry_1_tmp_1400f_7) + (M31_1)),
                    ((carry_2_tmp_1400f_8) + (M31_1)),
                    ((carry_3_tmp_1400f_9) + (M31_1)),
                ];
                *lookup_data.range_check_3_3_3_3_3_4 = [
                    M31_502259093,
                    ((p_coef_col103) + (M31_1)),
                    ((carry_0_tmp_1400f_6) + (M31_1)),
                    ((carry_1_tmp_1400f_7) + (M31_1)),
                    ((carry_2_tmp_1400f_8) + (M31_1)),
                    ((carry_3_tmp_1400f_9) + (M31_1)),
                ];
                *sub_component_inputs.range_check_3_3_3_3_3[1] = [
                    ((carry_4_tmp_1400f_10) + (M31_1)),
                    ((carry_5_tmp_1400f_11) + (M31_1)),
                    ((carry_6_tmp_1400f_12) + (M31_1)),
                    ((carry_7_tmp_1400f_13) + (M31_1)),
                    ((carry_8_tmp_1400f_14) + (M31_1)),
                ];
                *lookup_data.range_check_3_3_3_3_3_5 = [
                    M31_502259093,
                    ((carry_4_tmp_1400f_10) + (M31_1)),
                    ((carry_5_tmp_1400f_11) + (M31_1)),
                    ((carry_6_tmp_1400f_12) + (M31_1)),
                    ((carry_7_tmp_1400f_13) + (M31_1)),
                    ((carry_8_tmp_1400f_14) + (M31_1)),
                ];
                let linear_combination_n_4_coefs_3_1_1_1_output_tmp_1400f_15 =
                    combination_tmp_1400f_4;

                // Linear Combination N 4 Coefs 1 M 1 1 1.

                let combination_tmp_1400f_16 = PackedFelt252Width27::from_packed_felt252(
                    (((((Felt252_0_0_0_0)
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_1400f_0,
                            ))))
                        - ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_1400f_1,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_1400f_2,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_round_keys_output_tmp_1400f_3[1],
                            )))),
                );
                let combination_limb_0_col104 = combination_tmp_1400f_16.get_m31(0);
                *row[104] = combination_limb_0_col104;
                let combination_limb_1_col105 = combination_tmp_1400f_16.get_m31(1);
                *row[105] = combination_limb_1_col105;
                let combination_limb_2_col106 = combination_tmp_1400f_16.get_m31(2);
                *row[106] = combination_limb_2_col106;
                let combination_limb_3_col107 = combination_tmp_1400f_16.get_m31(3);
                *row[107] = combination_limb_3_col107;
                let combination_limb_4_col108 = combination_tmp_1400f_16.get_m31(4);
                *row[108] = combination_limb_4_col108;
                let combination_limb_5_col109 = combination_tmp_1400f_16.get_m31(5);
                *row[109] = combination_limb_5_col109;
                let combination_limb_6_col110 = combination_tmp_1400f_16.get_m31(6);
                *row[110] = combination_limb_6_col110;
                let combination_limb_7_col111 = combination_tmp_1400f_16.get_m31(7);
                *row[111] = combination_limb_7_col111;
                let combination_limb_8_col112 = combination_tmp_1400f_16.get_m31(8);
                *row[112] = combination_limb_8_col112;
                let combination_limb_9_col113 = combination_tmp_1400f_16.get_m31(9);
                *row[113] = combination_limb_9_col113;
                let biased_limb_accumulator_u32_tmp_1400f_17 = PackedUInt32::from_m31(
                    ((((((cube_252_output_limb_0_col33) - (cube_252_output_limb_0_col43))
                        + (cube_252_output_limb_0_col53))
                        + (poseidon_round_keys_output_limb_10_col73))
                        - (combination_limb_0_col104))
                        + (M31_268435458)),
                );
                let p_coef_col114 =
                    ((biased_limb_accumulator_u32_tmp_1400f_17.low().as_m31()) - (M31_2));
                *row[114] = p_coef_col114;
                let carry_0_tmp_1400f_18 = (((((((cube_252_output_limb_0_col33)
                    - (cube_252_output_limb_0_col43))
                    + (cube_252_output_limb_0_col53))
                    + (poseidon_round_keys_output_limb_10_col73))
                    - (combination_limb_0_col104))
                    - (p_coef_col114))
                    * (M31_16));
                let carry_1_tmp_1400f_19 = (((((((carry_0_tmp_1400f_18)
                    + (cube_252_output_limb_1_col34))
                    - (cube_252_output_limb_1_col44))
                    + (cube_252_output_limb_1_col54))
                    + (poseidon_round_keys_output_limb_11_col74))
                    - (combination_limb_1_col105))
                    * (M31_16));
                let carry_2_tmp_1400f_20 = (((((((carry_1_tmp_1400f_19)
                    + (cube_252_output_limb_2_col35))
                    - (cube_252_output_limb_2_col45))
                    + (cube_252_output_limb_2_col55))
                    + (poseidon_round_keys_output_limb_12_col75))
                    - (combination_limb_2_col106))
                    * (M31_16));
                let carry_3_tmp_1400f_21 = (((((((carry_2_tmp_1400f_20)
                    + (cube_252_output_limb_3_col36))
                    - (cube_252_output_limb_3_col46))
                    + (cube_252_output_limb_3_col56))
                    + (poseidon_round_keys_output_limb_13_col76))
                    - (combination_limb_3_col107))
                    * (M31_16));
                let carry_4_tmp_1400f_22 = (((((((carry_3_tmp_1400f_21)
                    + (cube_252_output_limb_4_col37))
                    - (cube_252_output_limb_4_col47))
                    + (cube_252_output_limb_4_col57))
                    + (poseidon_round_keys_output_limb_14_col77))
                    - (combination_limb_4_col108))
                    * (M31_16));
                let carry_5_tmp_1400f_23 = (((((((carry_4_tmp_1400f_22)
                    + (cube_252_output_limb_5_col38))
                    - (cube_252_output_limb_5_col48))
                    + (cube_252_output_limb_5_col58))
                    + (poseidon_round_keys_output_limb_15_col78))
                    - (combination_limb_5_col109))
                    * (M31_16));
                let carry_6_tmp_1400f_24 = (((((((carry_5_tmp_1400f_23)
                    + (cube_252_output_limb_6_col39))
                    - (cube_252_output_limb_6_col49))
                    + (cube_252_output_limb_6_col59))
                    + (poseidon_round_keys_output_limb_16_col79))
                    - (combination_limb_6_col110))
                    * (M31_16));
                let carry_7_tmp_1400f_25 = ((((((((carry_6_tmp_1400f_24)
                    + (cube_252_output_limb_7_col40))
                    - (cube_252_output_limb_7_col50))
                    + (cube_252_output_limb_7_col60))
                    + (poseidon_round_keys_output_limb_17_col80))
                    - (combination_limb_7_col111))
                    - ((p_coef_col114) * (M31_136)))
                    * (M31_16));
                let carry_8_tmp_1400f_26 = (((((((carry_7_tmp_1400f_25)
                    + (cube_252_output_limb_8_col41))
                    - (cube_252_output_limb_8_col51))
                    + (cube_252_output_limb_8_col61))
                    + (poseidon_round_keys_output_limb_18_col81))
                    - (combination_limb_8_col112))
                    * (M31_16));
                *sub_component_inputs.range_check_3_3_3_3_3[2] = [
                    ((p_coef_col114) + (M31_2)),
                    ((carry_0_tmp_1400f_18) + (M31_2)),
                    ((carry_1_tmp_1400f_19) + (M31_2)),
                    ((carry_2_tmp_1400f_20) + (M31_2)),
                    ((carry_3_tmp_1400f_21) + (M31_2)),
                ];
                *lookup_data.range_check_3_3_3_3_3_6 = [
                    M31_502259093,
                    ((p_coef_col114) + (M31_2)),
                    ((carry_0_tmp_1400f_18) + (M31_2)),
                    ((carry_1_tmp_1400f_19) + (M31_2)),
                    ((carry_2_tmp_1400f_20) + (M31_2)),
                    ((carry_3_tmp_1400f_21) + (M31_2)),
                ];
                *sub_component_inputs.range_check_3_3_3_3_3[3] = [
                    ((carry_4_tmp_1400f_22) + (M31_2)),
                    ((carry_5_tmp_1400f_23) + (M31_2)),
                    ((carry_6_tmp_1400f_24) + (M31_2)),
                    ((carry_7_tmp_1400f_25) + (M31_2)),
                    ((carry_8_tmp_1400f_26) + (M31_2)),
                ];
                *lookup_data.range_check_3_3_3_3_3_7 = [
                    M31_502259093,
                    ((carry_4_tmp_1400f_22) + (M31_2)),
                    ((carry_5_tmp_1400f_23) + (M31_2)),
                    ((carry_6_tmp_1400f_24) + (M31_2)),
                    ((carry_7_tmp_1400f_25) + (M31_2)),
                    ((carry_8_tmp_1400f_26) + (M31_2)),
                ];
                let linear_combination_n_4_coefs_1_m1_1_1_output_tmp_1400f_27 =
                    combination_tmp_1400f_16;

                // Linear Combination N 4 Coefs 1 1 M 2 1.

                let combination_tmp_1400f_28 = PackedFelt252Width27::from_packed_felt252(
                    (((((Felt252_0_0_0_0)
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_1400f_0,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_1400f_1,
                            ))))
                        - ((Felt252_2_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_1400f_2,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_round_keys_output_tmp_1400f_3[2],
                            )))),
                );
                let combination_limb_0_col115 = combination_tmp_1400f_28.get_m31(0);
                *row[115] = combination_limb_0_col115;
                let combination_limb_1_col116 = combination_tmp_1400f_28.get_m31(1);
                *row[116] = combination_limb_1_col116;
                let combination_limb_2_col117 = combination_tmp_1400f_28.get_m31(2);
                *row[117] = combination_limb_2_col117;
                let combination_limb_3_col118 = combination_tmp_1400f_28.get_m31(3);
                *row[118] = combination_limb_3_col118;
                let combination_limb_4_col119 = combination_tmp_1400f_28.get_m31(4);
                *row[119] = combination_limb_4_col119;
                let combination_limb_5_col120 = combination_tmp_1400f_28.get_m31(5);
                *row[120] = combination_limb_5_col120;
                let combination_limb_6_col121 = combination_tmp_1400f_28.get_m31(6);
                *row[121] = combination_limb_6_col121;
                let combination_limb_7_col122 = combination_tmp_1400f_28.get_m31(7);
                *row[122] = combination_limb_7_col122;
                let combination_limb_8_col123 = combination_tmp_1400f_28.get_m31(8);
                *row[123] = combination_limb_8_col123;
                let combination_limb_9_col124 = combination_tmp_1400f_28.get_m31(9);
                *row[124] = combination_limb_9_col124;
                let biased_limb_accumulator_u32_tmp_1400f_29 = PackedUInt32::from_m31(
                    ((((((cube_252_output_limb_0_col33) + (cube_252_output_limb_0_col43))
                        - ((M31_2) * (cube_252_output_limb_0_col53)))
                        + (poseidon_round_keys_output_limb_20_col83))
                        - (combination_limb_0_col115))
                        + (M31_402653187)),
                );
                let p_coef_col125 =
                    ((biased_limb_accumulator_u32_tmp_1400f_29.low().as_m31()) - (M31_3));
                *row[125] = p_coef_col125;
                let carry_0_tmp_1400f_30 = (((((((cube_252_output_limb_0_col33)
                    + (cube_252_output_limb_0_col43))
                    - ((M31_2) * (cube_252_output_limb_0_col53)))
                    + (poseidon_round_keys_output_limb_20_col83))
                    - (combination_limb_0_col115))
                    - (p_coef_col125))
                    * (M31_16));
                let carry_1_tmp_1400f_31 = (((((((carry_0_tmp_1400f_30)
                    + (cube_252_output_limb_1_col34))
                    + (cube_252_output_limb_1_col44))
                    - ((M31_2) * (cube_252_output_limb_1_col54)))
                    + (poseidon_round_keys_output_limb_21_col84))
                    - (combination_limb_1_col116))
                    * (M31_16));
                let carry_2_tmp_1400f_32 = (((((((carry_1_tmp_1400f_31)
                    + (cube_252_output_limb_2_col35))
                    + (cube_252_output_limb_2_col45))
                    - ((M31_2) * (cube_252_output_limb_2_col55)))
                    + (poseidon_round_keys_output_limb_22_col85))
                    - (combination_limb_2_col117))
                    * (M31_16));
                let carry_3_tmp_1400f_33 = (((((((carry_2_tmp_1400f_32)
                    + (cube_252_output_limb_3_col36))
                    + (cube_252_output_limb_3_col46))
                    - ((M31_2) * (cube_252_output_limb_3_col56)))
                    + (poseidon_round_keys_output_limb_23_col86))
                    - (combination_limb_3_col118))
                    * (M31_16));
                let carry_4_tmp_1400f_34 = (((((((carry_3_tmp_1400f_33)
                    + (cube_252_output_limb_4_col37))
                    + (cube_252_output_limb_4_col47))
                    - ((M31_2) * (cube_252_output_limb_4_col57)))
                    + (poseidon_round_keys_output_limb_24_col87))
                    - (combination_limb_4_col119))
                    * (M31_16));
                let carry_5_tmp_1400f_35 = (((((((carry_4_tmp_1400f_34)
                    + (cube_252_output_limb_5_col38))
                    + (cube_252_output_limb_5_col48))
                    - ((M31_2) * (cube_252_output_limb_5_col58)))
                    + (poseidon_round_keys_output_limb_25_col88))
                    - (combination_limb_5_col120))
                    * (M31_16));
                let carry_6_tmp_1400f_36 = (((((((carry_5_tmp_1400f_35)
                    + (cube_252_output_limb_6_col39))
                    + (cube_252_output_limb_6_col49))
                    - ((M31_2) * (cube_252_output_limb_6_col59)))
                    + (poseidon_round_keys_output_limb_26_col89))
                    - (combination_limb_6_col121))
                    * (M31_16));
                let carry_7_tmp_1400f_37 = ((((((((carry_6_tmp_1400f_36)
                    + (cube_252_output_limb_7_col40))
                    + (cube_252_output_limb_7_col50))
                    - ((M31_2) * (cube_252_output_limb_7_col60)))
                    + (poseidon_round_keys_output_limb_27_col90))
                    - (combination_limb_7_col122))
                    - ((p_coef_col125) * (M31_136)))
                    * (M31_16));
                let carry_8_tmp_1400f_38 = (((((((carry_7_tmp_1400f_37)
                    + (cube_252_output_limb_8_col41))
                    + (cube_252_output_limb_8_col51))
                    - ((M31_2) * (cube_252_output_limb_8_col61)))
                    + (poseidon_round_keys_output_limb_28_col91))
                    - (combination_limb_8_col123))
                    * (M31_16));
                *sub_component_inputs.range_check_3_3_3_3_3[4] = [
                    ((p_coef_col125) + (M31_3)),
                    ((carry_0_tmp_1400f_30) + (M31_3)),
                    ((carry_1_tmp_1400f_31) + (M31_3)),
                    ((carry_2_tmp_1400f_32) + (M31_3)),
                    ((carry_3_tmp_1400f_33) + (M31_3)),
                ];
                *lookup_data.range_check_3_3_3_3_3_8 = [
                    M31_502259093,
                    ((p_coef_col125) + (M31_3)),
                    ((carry_0_tmp_1400f_30) + (M31_3)),
                    ((carry_1_tmp_1400f_31) + (M31_3)),
                    ((carry_2_tmp_1400f_32) + (M31_3)),
                    ((carry_3_tmp_1400f_33) + (M31_3)),
                ];
                *sub_component_inputs.range_check_3_3_3_3_3[5] = [
                    ((carry_4_tmp_1400f_34) + (M31_3)),
                    ((carry_5_tmp_1400f_35) + (M31_3)),
                    ((carry_6_tmp_1400f_36) + (M31_3)),
                    ((carry_7_tmp_1400f_37) + (M31_3)),
                    ((carry_8_tmp_1400f_38) + (M31_3)),
                ];
                *lookup_data.range_check_3_3_3_3_3_9 = [
                    M31_502259093,
                    ((carry_4_tmp_1400f_34) + (M31_3)),
                    ((carry_5_tmp_1400f_35) + (M31_3)),
                    ((carry_6_tmp_1400f_36) + (M31_3)),
                    ((carry_7_tmp_1400f_37) + (M31_3)),
                    ((carry_8_tmp_1400f_38) + (M31_3)),
                ];
                let linear_combination_n_4_coefs_1_1_m2_1_output_tmp_1400f_39 =
                    combination_tmp_1400f_28;

                *lookup_data.poseidon_full_round_chain_10 = [
                    M31_1480369132,
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
                ];
                *lookup_data.poseidon_full_round_chain_11 = [
                    M31_1480369132,
                    input_limb_0_col1,
                    ((input_limb_1_col2) + (M31_1)),
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
                    combination_limb_0_col104,
                    combination_limb_1_col105,
                    combination_limb_2_col106,
                    combination_limb_3_col107,
                    combination_limb_4_col108,
                    combination_limb_5_col109,
                    combination_limb_6_col110,
                    combination_limb_7_col111,
                    combination_limb_8_col112,
                    combination_limb_9_col113,
                    combination_limb_0_col115,
                    combination_limb_1_col116,
                    combination_limb_2_col117,
                    combination_limb_3_col118,
                    combination_limb_4_col119,
                    combination_limb_5_col120,
                    combination_limb_6_col121,
                    combination_limb_7_col122,
                    combination_limb_8_col123,
                    combination_limb_9_col124,
                ];
                *lookup_data.mults_0 = enabler_col0;
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    cube_252_0: Vec<[PackedM31; 21]>,
    cube_252_1: Vec<[PackedM31; 21]>,
    cube_252_2: Vec<[PackedM31; 21]>,
    poseidon_round_keys_3: Vec<[PackedM31; 32]>,
    range_check_3_3_3_3_3_4: Vec<[PackedM31; 6]>,
    range_check_3_3_3_3_3_5: Vec<[PackedM31; 6]>,
    range_check_3_3_3_3_3_6: Vec<[PackedM31; 6]>,
    range_check_3_3_3_3_3_7: Vec<[PackedM31; 6]>,
    range_check_3_3_3_3_3_8: Vec<[PackedM31; 6]>,
    range_check_3_3_3_3_3_9: Vec<[PackedM31; 6]>,
    poseidon_full_round_chain_10: Vec<[PackedM31; 33]>,
    poseidon_full_round_chain_11: Vec<[PackedM31; 33]>,
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
            &self.lookup_data.cube_252_0,
            &self.lookup_data.cube_252_1,
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
            &self.lookup_data.cube_252_2,
            &self.lookup_data.poseidon_round_keys_3,
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
            &self.lookup_data.range_check_3_3_3_3_3_4,
            &self.lookup_data.range_check_3_3_3_3_3_5,
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
            &self.lookup_data.range_check_3_3_3_3_3_6,
            &self.lookup_data.range_check_3_3_3_3_3_7,
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
            &self.lookup_data.range_check_3_3_3_3_3_8,
            &self.lookup_data.range_check_3_3_3_3_3_9,
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
            &self.lookup_data.poseidon_full_round_chain_10,
            &self.lookup_data.poseidon_full_round_chain_11,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom1 * *mult0 - denom0 * *mult1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();

        (trace, InteractionClaim { claimed_sum })
    }
}
