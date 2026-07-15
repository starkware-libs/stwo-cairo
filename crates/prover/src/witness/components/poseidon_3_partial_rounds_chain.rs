// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::poseidon_3_partial_rounds_chain::{
    Claim, InteractionClaim, N_TRACE_COLUMNS,
};

use crate::witness::components::{
    cube_252, poseidon_round_keys, range_check_252_width_27, range_check_4_4, range_check_4_4_4_4,
};
use crate::witness::prelude::*;

pub type InputType = (M31, M31, [Felt252Width27; 4]);
pub type PackedInputType = (PackedM31, PackedM31, [PackedFelt252Width27; 4]);

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
        poseidon_round_keys_state: &poseidon_round_keys::ClaimGenerator,
        cube_252_state: &cube_252::ClaimGenerator,
        range_check_4_4_4_4_state: &range_check_4_4_4_4::ClaimGenerator,
        range_check_4_4_state: &range_check_4_4::ClaimGenerator,
        range_check_252_width_27_state: &range_check_252_width_27::ClaimGenerator,
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

        let (trace, lookup_data, sub_component_inputs) = write_trace_simd(
            packed_inputs,
            n_active_rows,
            poseidon_round_keys_state,
            cube_252_state,
            range_check_4_4_4_4_state,
            range_check_4_4_state,
            range_check_252_width_27_state,
        );
        for inputs in sub_component_inputs.poseidon_round_keys {
            add_inputs(poseidon_round_keys_state, &inputs, n_active_rows, 0);
        }
        for inputs in sub_component_inputs.cube_252 {
            add_inputs(cube_252_state, &inputs, n_active_rows, 0);
        }
        for inputs in sub_component_inputs.range_check_4_4_4_4 {
            add_inputs(range_check_4_4_4_4_state, &inputs, n_active_rows, 0);
        }
        for inputs in sub_component_inputs.range_check_4_4 {
            add_inputs(range_check_4_4_state, &inputs, n_active_rows, 0);
        }
        for inputs in sub_component_inputs.range_check_252_width_27 {
            add_inputs(range_check_252_width_27_state, &inputs, n_active_rows, 0);
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
    cube_252_state: &cube_252::ClaimGenerator,
    range_check_4_4_4_4_state: &range_check_4_4_4_4::ClaimGenerator,
    range_check_4_4_state: &range_check_4_4::ClaimGenerator,
    range_check_252_width_27_state: &range_check_252_width_27::ClaimGenerator,
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
    let M31_1024310512 = PackedM31::broadcast(M31::from(1024310512));
    let M31_1027333874 = PackedM31::broadcast(M31::from(1027333874));
    let M31_1090315331 = PackedM31::broadcast(M31::from(1090315331));
    let M31_134217729 = PackedM31::broadcast(M31::from(134217729));
    let M31_1343313504 = PackedM31::broadcast(M31::from(1343313504));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_16 = PackedM31::broadcast(M31::from(16));
    let M31_1651211826 = PackedM31::broadcast(M31::from(1651211826));
    let M31_1987997202 = PackedM31::broadcast(M31::from(1987997202));
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
                let enabler_col0 = enabler_col.packed_at(row_index);
                *row[0] = enabler_col0;
                let input_limb_0_col1 = poseidon_3_partial_rounds_chain_input.0;
                *row[1] = input_limb_0_col1;
                let input_limb_1_col2 = poseidon_3_partial_rounds_chain_input.1;
                *row[2] = input_limb_1_col2;
                let input_limb_2_col3 = poseidon_3_partial_rounds_chain_input.2[0].get_m31(0);
                *row[3] = input_limb_2_col3;
                let input_limb_3_col4 = poseidon_3_partial_rounds_chain_input.2[0].get_m31(1);
                *row[4] = input_limb_3_col4;
                let input_limb_4_col5 = poseidon_3_partial_rounds_chain_input.2[0].get_m31(2);
                *row[5] = input_limb_4_col5;
                let input_limb_5_col6 = poseidon_3_partial_rounds_chain_input.2[0].get_m31(3);
                *row[6] = input_limb_5_col6;
                let input_limb_6_col7 = poseidon_3_partial_rounds_chain_input.2[0].get_m31(4);
                *row[7] = input_limb_6_col7;
                let input_limb_7_col8 = poseidon_3_partial_rounds_chain_input.2[0].get_m31(5);
                *row[8] = input_limb_7_col8;
                let input_limb_8_col9 = poseidon_3_partial_rounds_chain_input.2[0].get_m31(6);
                *row[9] = input_limb_8_col9;
                let input_limb_9_col10 = poseidon_3_partial_rounds_chain_input.2[0].get_m31(7);
                *row[10] = input_limb_9_col10;
                let input_limb_10_col11 = poseidon_3_partial_rounds_chain_input.2[0].get_m31(8);
                *row[11] = input_limb_10_col11;
                let input_limb_11_col12 = poseidon_3_partial_rounds_chain_input.2[0].get_m31(9);
                *row[12] = input_limb_11_col12;
                let input_limb_12_col13 = poseidon_3_partial_rounds_chain_input.2[1].get_m31(0);
                *row[13] = input_limb_12_col13;
                let input_limb_13_col14 = poseidon_3_partial_rounds_chain_input.2[1].get_m31(1);
                *row[14] = input_limb_13_col14;
                let input_limb_14_col15 = poseidon_3_partial_rounds_chain_input.2[1].get_m31(2);
                *row[15] = input_limb_14_col15;
                let input_limb_15_col16 = poseidon_3_partial_rounds_chain_input.2[1].get_m31(3);
                *row[16] = input_limb_15_col16;
                let input_limb_16_col17 = poseidon_3_partial_rounds_chain_input.2[1].get_m31(4);
                *row[17] = input_limb_16_col17;
                let input_limb_17_col18 = poseidon_3_partial_rounds_chain_input.2[1].get_m31(5);
                *row[18] = input_limb_17_col18;
                let input_limb_18_col19 = poseidon_3_partial_rounds_chain_input.2[1].get_m31(6);
                *row[19] = input_limb_18_col19;
                let input_limb_19_col20 = poseidon_3_partial_rounds_chain_input.2[1].get_m31(7);
                *row[20] = input_limb_19_col20;
                let input_limb_20_col21 = poseidon_3_partial_rounds_chain_input.2[1].get_m31(8);
                *row[21] = input_limb_20_col21;
                let input_limb_21_col22 = poseidon_3_partial_rounds_chain_input.2[1].get_m31(9);
                *row[22] = input_limb_21_col22;
                let input_limb_22_col23 = poseidon_3_partial_rounds_chain_input.2[2].get_m31(0);
                *row[23] = input_limb_22_col23;
                let input_limb_23_col24 = poseidon_3_partial_rounds_chain_input.2[2].get_m31(1);
                *row[24] = input_limb_23_col24;
                let input_limb_24_col25 = poseidon_3_partial_rounds_chain_input.2[2].get_m31(2);
                *row[25] = input_limb_24_col25;
                let input_limb_25_col26 = poseidon_3_partial_rounds_chain_input.2[2].get_m31(3);
                *row[26] = input_limb_25_col26;
                let input_limb_26_col27 = poseidon_3_partial_rounds_chain_input.2[2].get_m31(4);
                *row[27] = input_limb_26_col27;
                let input_limb_27_col28 = poseidon_3_partial_rounds_chain_input.2[2].get_m31(5);
                *row[28] = input_limb_27_col28;
                let input_limb_28_col29 = poseidon_3_partial_rounds_chain_input.2[2].get_m31(6);
                *row[29] = input_limb_28_col29;
                let input_limb_29_col30 = poseidon_3_partial_rounds_chain_input.2[2].get_m31(7);
                *row[30] = input_limb_29_col30;
                let input_limb_30_col31 = poseidon_3_partial_rounds_chain_input.2[2].get_m31(8);
                *row[31] = input_limb_30_col31;
                let input_limb_31_col32 = poseidon_3_partial_rounds_chain_input.2[2].get_m31(9);
                *row[32] = input_limb_31_col32;
                let input_limb_32_col33 = poseidon_3_partial_rounds_chain_input.2[3].get_m31(0);
                *row[33] = input_limb_32_col33;
                let input_limb_33_col34 = poseidon_3_partial_rounds_chain_input.2[3].get_m31(1);
                *row[34] = input_limb_33_col34;
                let input_limb_34_col35 = poseidon_3_partial_rounds_chain_input.2[3].get_m31(2);
                *row[35] = input_limb_34_col35;
                let input_limb_35_col36 = poseidon_3_partial_rounds_chain_input.2[3].get_m31(3);
                *row[36] = input_limb_35_col36;
                let input_limb_36_col37 = poseidon_3_partial_rounds_chain_input.2[3].get_m31(4);
                *row[37] = input_limb_36_col37;
                let input_limb_37_col38 = poseidon_3_partial_rounds_chain_input.2[3].get_m31(5);
                *row[38] = input_limb_37_col38;
                let input_limb_38_col39 = poseidon_3_partial_rounds_chain_input.2[3].get_m31(6);
                *row[39] = input_limb_38_col39;
                let input_limb_39_col40 = poseidon_3_partial_rounds_chain_input.2[3].get_m31(7);
                *row[40] = input_limb_39_col40;
                let input_limb_40_col41 = poseidon_3_partial_rounds_chain_input.2[3].get_m31(8);
                *row[41] = input_limb_40_col41;
                let input_limb_41_col42 = poseidon_3_partial_rounds_chain_input.2[3].get_m31(9);
                *row[42] = input_limb_41_col42;
                *sub_component_inputs.poseidon_round_keys[0] = [input_limb_1_col2];
                let poseidon_round_keys_output_tmp_8c14f_0 =
                    PackedPoseidonRoundKeys::deduce_output([input_limb_1_col2]);
                let poseidon_round_keys_output_limb_0_col43 =
                    poseidon_round_keys_output_tmp_8c14f_0[0].get_m31(0);
                *row[43] = poseidon_round_keys_output_limb_0_col43;
                let poseidon_round_keys_output_limb_1_col44 =
                    poseidon_round_keys_output_tmp_8c14f_0[0].get_m31(1);
                *row[44] = poseidon_round_keys_output_limb_1_col44;
                let poseidon_round_keys_output_limb_2_col45 =
                    poseidon_round_keys_output_tmp_8c14f_0[0].get_m31(2);
                *row[45] = poseidon_round_keys_output_limb_2_col45;
                let poseidon_round_keys_output_limb_3_col46 =
                    poseidon_round_keys_output_tmp_8c14f_0[0].get_m31(3);
                *row[46] = poseidon_round_keys_output_limb_3_col46;
                let poseidon_round_keys_output_limb_4_col47 =
                    poseidon_round_keys_output_tmp_8c14f_0[0].get_m31(4);
                *row[47] = poseidon_round_keys_output_limb_4_col47;
                let poseidon_round_keys_output_limb_5_col48 =
                    poseidon_round_keys_output_tmp_8c14f_0[0].get_m31(5);
                *row[48] = poseidon_round_keys_output_limb_5_col48;
                let poseidon_round_keys_output_limb_6_col49 =
                    poseidon_round_keys_output_tmp_8c14f_0[0].get_m31(6);
                *row[49] = poseidon_round_keys_output_limb_6_col49;
                let poseidon_round_keys_output_limb_7_col50 =
                    poseidon_round_keys_output_tmp_8c14f_0[0].get_m31(7);
                *row[50] = poseidon_round_keys_output_limb_7_col50;
                let poseidon_round_keys_output_limb_8_col51 =
                    poseidon_round_keys_output_tmp_8c14f_0[0].get_m31(8);
                *row[51] = poseidon_round_keys_output_limb_8_col51;
                let poseidon_round_keys_output_limb_9_col52 =
                    poseidon_round_keys_output_tmp_8c14f_0[0].get_m31(9);
                *row[52] = poseidon_round_keys_output_limb_9_col52;
                let poseidon_round_keys_output_limb_10_col53 =
                    poseidon_round_keys_output_tmp_8c14f_0[1].get_m31(0);
                *row[53] = poseidon_round_keys_output_limb_10_col53;
                let poseidon_round_keys_output_limb_11_col54 =
                    poseidon_round_keys_output_tmp_8c14f_0[1].get_m31(1);
                *row[54] = poseidon_round_keys_output_limb_11_col54;
                let poseidon_round_keys_output_limb_12_col55 =
                    poseidon_round_keys_output_tmp_8c14f_0[1].get_m31(2);
                *row[55] = poseidon_round_keys_output_limb_12_col55;
                let poseidon_round_keys_output_limb_13_col56 =
                    poseidon_round_keys_output_tmp_8c14f_0[1].get_m31(3);
                *row[56] = poseidon_round_keys_output_limb_13_col56;
                let poseidon_round_keys_output_limb_14_col57 =
                    poseidon_round_keys_output_tmp_8c14f_0[1].get_m31(4);
                *row[57] = poseidon_round_keys_output_limb_14_col57;
                let poseidon_round_keys_output_limb_15_col58 =
                    poseidon_round_keys_output_tmp_8c14f_0[1].get_m31(5);
                *row[58] = poseidon_round_keys_output_limb_15_col58;
                let poseidon_round_keys_output_limb_16_col59 =
                    poseidon_round_keys_output_tmp_8c14f_0[1].get_m31(6);
                *row[59] = poseidon_round_keys_output_limb_16_col59;
                let poseidon_round_keys_output_limb_17_col60 =
                    poseidon_round_keys_output_tmp_8c14f_0[1].get_m31(7);
                *row[60] = poseidon_round_keys_output_limb_17_col60;
                let poseidon_round_keys_output_limb_18_col61 =
                    poseidon_round_keys_output_tmp_8c14f_0[1].get_m31(8);
                *row[61] = poseidon_round_keys_output_limb_18_col61;
                let poseidon_round_keys_output_limb_19_col62 =
                    poseidon_round_keys_output_tmp_8c14f_0[1].get_m31(9);
                *row[62] = poseidon_round_keys_output_limb_19_col62;
                let poseidon_round_keys_output_limb_20_col63 =
                    poseidon_round_keys_output_tmp_8c14f_0[2].get_m31(0);
                *row[63] = poseidon_round_keys_output_limb_20_col63;
                let poseidon_round_keys_output_limb_21_col64 =
                    poseidon_round_keys_output_tmp_8c14f_0[2].get_m31(1);
                *row[64] = poseidon_round_keys_output_limb_21_col64;
                let poseidon_round_keys_output_limb_22_col65 =
                    poseidon_round_keys_output_tmp_8c14f_0[2].get_m31(2);
                *row[65] = poseidon_round_keys_output_limb_22_col65;
                let poseidon_round_keys_output_limb_23_col66 =
                    poseidon_round_keys_output_tmp_8c14f_0[2].get_m31(3);
                *row[66] = poseidon_round_keys_output_limb_23_col66;
                let poseidon_round_keys_output_limb_24_col67 =
                    poseidon_round_keys_output_tmp_8c14f_0[2].get_m31(4);
                *row[67] = poseidon_round_keys_output_limb_24_col67;
                let poseidon_round_keys_output_limb_25_col68 =
                    poseidon_round_keys_output_tmp_8c14f_0[2].get_m31(5);
                *row[68] = poseidon_round_keys_output_limb_25_col68;
                let poseidon_round_keys_output_limb_26_col69 =
                    poseidon_round_keys_output_tmp_8c14f_0[2].get_m31(6);
                *row[69] = poseidon_round_keys_output_limb_26_col69;
                let poseidon_round_keys_output_limb_27_col70 =
                    poseidon_round_keys_output_tmp_8c14f_0[2].get_m31(7);
                *row[70] = poseidon_round_keys_output_limb_27_col70;
                let poseidon_round_keys_output_limb_28_col71 =
                    poseidon_round_keys_output_tmp_8c14f_0[2].get_m31(8);
                *row[71] = poseidon_round_keys_output_limb_28_col71;
                let poseidon_round_keys_output_limb_29_col72 =
                    poseidon_round_keys_output_tmp_8c14f_0[2].get_m31(9);
                *row[72] = poseidon_round_keys_output_limb_29_col72;
                *lookup_data.poseidon_round_keys_0 = [
                    M31_1024310512,
                    input_limb_1_col2,
                    poseidon_round_keys_output_limb_0_col43,
                    poseidon_round_keys_output_limb_1_col44,
                    poseidon_round_keys_output_limb_2_col45,
                    poseidon_round_keys_output_limb_3_col46,
                    poseidon_round_keys_output_limb_4_col47,
                    poseidon_round_keys_output_limb_5_col48,
                    poseidon_round_keys_output_limb_6_col49,
                    poseidon_round_keys_output_limb_7_col50,
                    poseidon_round_keys_output_limb_8_col51,
                    poseidon_round_keys_output_limb_9_col52,
                    poseidon_round_keys_output_limb_10_col53,
                    poseidon_round_keys_output_limb_11_col54,
                    poseidon_round_keys_output_limb_12_col55,
                    poseidon_round_keys_output_limb_13_col56,
                    poseidon_round_keys_output_limb_14_col57,
                    poseidon_round_keys_output_limb_15_col58,
                    poseidon_round_keys_output_limb_16_col59,
                    poseidon_round_keys_output_limb_17_col60,
                    poseidon_round_keys_output_limb_18_col61,
                    poseidon_round_keys_output_limb_19_col62,
                    poseidon_round_keys_output_limb_20_col63,
                    poseidon_round_keys_output_limb_21_col64,
                    poseidon_round_keys_output_limb_22_col65,
                    poseidon_round_keys_output_limb_23_col66,
                    poseidon_round_keys_output_limb_24_col67,
                    poseidon_round_keys_output_limb_25_col68,
                    poseidon_round_keys_output_limb_26_col69,
                    poseidon_round_keys_output_limb_27_col70,
                    poseidon_round_keys_output_limb_28_col71,
                    poseidon_round_keys_output_limb_29_col72,
                ];

                // Poseidon Partial Round.

                *sub_component_inputs.cube_252[0] = poseidon_3_partial_rounds_chain_input.2[3];
                let cube_252_output_tmp_8c14f_1 =
                    PackedCube252::deduce_output(poseidon_3_partial_rounds_chain_input.2[3]);
                let cube_252_output_limb_0_col73 = cube_252_output_tmp_8c14f_1.get_m31(0);
                *row[73] = cube_252_output_limb_0_col73;
                let cube_252_output_limb_1_col74 = cube_252_output_tmp_8c14f_1.get_m31(1);
                *row[74] = cube_252_output_limb_1_col74;
                let cube_252_output_limb_2_col75 = cube_252_output_tmp_8c14f_1.get_m31(2);
                *row[75] = cube_252_output_limb_2_col75;
                let cube_252_output_limb_3_col76 = cube_252_output_tmp_8c14f_1.get_m31(3);
                *row[76] = cube_252_output_limb_3_col76;
                let cube_252_output_limb_4_col77 = cube_252_output_tmp_8c14f_1.get_m31(4);
                *row[77] = cube_252_output_limb_4_col77;
                let cube_252_output_limb_5_col78 = cube_252_output_tmp_8c14f_1.get_m31(5);
                *row[78] = cube_252_output_limb_5_col78;
                let cube_252_output_limb_6_col79 = cube_252_output_tmp_8c14f_1.get_m31(6);
                *row[79] = cube_252_output_limb_6_col79;
                let cube_252_output_limb_7_col80 = cube_252_output_tmp_8c14f_1.get_m31(7);
                *row[80] = cube_252_output_limb_7_col80;
                let cube_252_output_limb_8_col81 = cube_252_output_tmp_8c14f_1.get_m31(8);
                *row[81] = cube_252_output_limb_8_col81;
                let cube_252_output_limb_9_col82 = cube_252_output_tmp_8c14f_1.get_m31(9);
                *row[82] = cube_252_output_limb_9_col82;
                *lookup_data.cube_252_1 = [
                    M31_1987997202,
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
                    cube_252_output_limb_0_col73,
                    cube_252_output_limb_1_col74,
                    cube_252_output_limb_2_col75,
                    cube_252_output_limb_3_col76,
                    cube_252_output_limb_4_col77,
                    cube_252_output_limb_5_col78,
                    cube_252_output_limb_6_col79,
                    cube_252_output_limb_7_col80,
                    cube_252_output_limb_8_col81,
                    cube_252_output_limb_9_col82,
                ];

                // Linear Combination N 6 Coefs 4 2 3 1 M 1 1.

                let combination_tmp_8c14f_2 = PackedFelt252Width27::from_packed_felt252(
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
                                cube_252_output_tmp_8c14f_1,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_round_keys_output_tmp_8c14f_0[0],
                            )))),
                );
                let combination_limb_0_col83 = combination_tmp_8c14f_2.get_m31(0);
                *row[83] = combination_limb_0_col83;
                let combination_limb_1_col84 = combination_tmp_8c14f_2.get_m31(1);
                *row[84] = combination_limb_1_col84;
                let combination_limb_2_col85 = combination_tmp_8c14f_2.get_m31(2);
                *row[85] = combination_limb_2_col85;
                let combination_limb_3_col86 = combination_tmp_8c14f_2.get_m31(3);
                *row[86] = combination_limb_3_col86;
                let combination_limb_4_col87 = combination_tmp_8c14f_2.get_m31(4);
                *row[87] = combination_limb_4_col87;
                let combination_limb_5_col88 = combination_tmp_8c14f_2.get_m31(5);
                *row[88] = combination_limb_5_col88;
                let combination_limb_6_col89 = combination_tmp_8c14f_2.get_m31(6);
                *row[89] = combination_limb_6_col89;
                let combination_limb_7_col90 = combination_tmp_8c14f_2.get_m31(7);
                *row[90] = combination_limb_7_col90;
                let combination_limb_8_col91 = combination_tmp_8c14f_2.get_m31(8);
                *row[91] = combination_limb_8_col91;
                let combination_limb_9_col92 = combination_tmp_8c14f_2.get_m31(9);
                *row[92] = combination_limb_9_col92;
                let biased_limb_accumulator_u32_tmp_8c14f_3 = PackedUInt32::from_m31(
                    (((((((((M31_4) * (input_limb_2_col3))
                        + ((M31_2) * (input_limb_12_col13)))
                        + ((M31_3) * (input_limb_22_col23)))
                        + (input_limb_32_col33))
                        - (cube_252_output_limb_0_col73))
                        + (poseidon_round_keys_output_limb_0_col43))
                        - (combination_limb_0_col83))
                        + (M31_268435458)),
                );
                let p_coef_col93 =
                    ((biased_limb_accumulator_u32_tmp_8c14f_3.low().as_m31()) - (M31_2));
                *row[93] = p_coef_col93;
                let carry_0_tmp_8c14f_4 = ((((((((((M31_4) * (input_limb_2_col3))
                    + ((M31_2) * (input_limb_12_col13)))
                    + ((M31_3) * (input_limb_22_col23)))
                    + (input_limb_32_col33))
                    - (cube_252_output_limb_0_col73))
                    + (poseidon_round_keys_output_limb_0_col43))
                    - (combination_limb_0_col83))
                    - (p_coef_col93))
                    * (M31_16));
                let carry_1_tmp_8c14f_5 = (((((((((carry_0_tmp_8c14f_4)
                    + ((M31_4) * (input_limb_3_col4)))
                    + ((M31_2) * (input_limb_13_col14)))
                    + ((M31_3) * (input_limb_23_col24)))
                    + (input_limb_33_col34))
                    - (cube_252_output_limb_1_col74))
                    + (poseidon_round_keys_output_limb_1_col44))
                    - (combination_limb_1_col84))
                    * (M31_16));
                let carry_2_tmp_8c14f_6 = (((((((((carry_1_tmp_8c14f_5)
                    + ((M31_4) * (input_limb_4_col5)))
                    + ((M31_2) * (input_limb_14_col15)))
                    + ((M31_3) * (input_limb_24_col25)))
                    + (input_limb_34_col35))
                    - (cube_252_output_limb_2_col75))
                    + (poseidon_round_keys_output_limb_2_col45))
                    - (combination_limb_2_col85))
                    * (M31_16));
                let carry_3_tmp_8c14f_7 = (((((((((carry_2_tmp_8c14f_6)
                    + ((M31_4) * (input_limb_5_col6)))
                    + ((M31_2) * (input_limb_15_col16)))
                    + ((M31_3) * (input_limb_25_col26)))
                    + (input_limb_35_col36))
                    - (cube_252_output_limb_3_col76))
                    + (poseidon_round_keys_output_limb_3_col46))
                    - (combination_limb_3_col86))
                    * (M31_16));
                let carry_4_tmp_8c14f_8 = (((((((((carry_3_tmp_8c14f_7)
                    + ((M31_4) * (input_limb_6_col7)))
                    + ((M31_2) * (input_limb_16_col17)))
                    + ((M31_3) * (input_limb_26_col27)))
                    + (input_limb_36_col37))
                    - (cube_252_output_limb_4_col77))
                    + (poseidon_round_keys_output_limb_4_col47))
                    - (combination_limb_4_col87))
                    * (M31_16));
                let carry_5_tmp_8c14f_9 = (((((((((carry_4_tmp_8c14f_8)
                    + ((M31_4) * (input_limb_7_col8)))
                    + ((M31_2) * (input_limb_17_col18)))
                    + ((M31_3) * (input_limb_27_col28)))
                    + (input_limb_37_col38))
                    - (cube_252_output_limb_5_col78))
                    + (poseidon_round_keys_output_limb_5_col48))
                    - (combination_limb_5_col88))
                    * (M31_16));
                let carry_6_tmp_8c14f_10 = (((((((((carry_5_tmp_8c14f_9)
                    + ((M31_4) * (input_limb_8_col9)))
                    + ((M31_2) * (input_limb_18_col19)))
                    + ((M31_3) * (input_limb_28_col29)))
                    + (input_limb_38_col39))
                    - (cube_252_output_limb_6_col79))
                    + (poseidon_round_keys_output_limb_6_col49))
                    - (combination_limb_6_col89))
                    * (M31_16));
                let carry_7_tmp_8c14f_11 = ((((((((((carry_6_tmp_8c14f_10)
                    + ((M31_4) * (input_limb_9_col10)))
                    + ((M31_2) * (input_limb_19_col20)))
                    + ((M31_3) * (input_limb_29_col30)))
                    + (input_limb_39_col40))
                    - (cube_252_output_limb_7_col80))
                    + (poseidon_round_keys_output_limb_7_col50))
                    - (combination_limb_7_col90))
                    - ((p_coef_col93) * (M31_136)))
                    * (M31_16));
                let carry_8_tmp_8c14f_12 = (((((((((carry_7_tmp_8c14f_11)
                    + ((M31_4) * (input_limb_10_col11)))
                    + ((M31_2) * (input_limb_20_col21)))
                    + ((M31_3) * (input_limb_30_col31)))
                    + (input_limb_40_col41))
                    - (cube_252_output_limb_8_col81))
                    + (poseidon_round_keys_output_limb_8_col51))
                    - (combination_limb_8_col91))
                    * (M31_16));
                *sub_component_inputs.range_check_4_4_4_4[0] = [
                    ((p_coef_col93) + (M31_2)),
                    ((carry_0_tmp_8c14f_4) + (M31_2)),
                    ((carry_1_tmp_8c14f_5) + (M31_2)),
                    ((carry_2_tmp_8c14f_6) + (M31_2)),
                ];
                *lookup_data.range_check_4_4_4_4_2 = [
                    M31_1027333874,
                    ((p_coef_col93) + (M31_2)),
                    ((carry_0_tmp_8c14f_4) + (M31_2)),
                    ((carry_1_tmp_8c14f_5) + (M31_2)),
                    ((carry_2_tmp_8c14f_6) + (M31_2)),
                ];
                *sub_component_inputs.range_check_4_4_4_4[1] = [
                    ((carry_3_tmp_8c14f_7) + (M31_2)),
                    ((carry_4_tmp_8c14f_8) + (M31_2)),
                    ((carry_5_tmp_8c14f_9) + (M31_2)),
                    ((carry_6_tmp_8c14f_10) + (M31_2)),
                ];
                *lookup_data.range_check_4_4_4_4_3 = [
                    M31_1027333874,
                    ((carry_3_tmp_8c14f_7) + (M31_2)),
                    ((carry_4_tmp_8c14f_8) + (M31_2)),
                    ((carry_5_tmp_8c14f_9) + (M31_2)),
                    ((carry_6_tmp_8c14f_10) + (M31_2)),
                ];
                *sub_component_inputs.range_check_4_4[0] = [
                    ((carry_7_tmp_8c14f_11) + (M31_2)),
                    ((carry_8_tmp_8c14f_12) + (M31_2)),
                ];
                *lookup_data.range_check_4_4_4 = [
                    M31_1651211826,
                    ((carry_7_tmp_8c14f_11) + (M31_2)),
                    ((carry_8_tmp_8c14f_12) + (M31_2)),
                ];
                let linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_8c14f_13 =
                    combination_tmp_8c14f_2;

                *sub_component_inputs.range_check_252_width_27[0] =
                    linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_8c14f_13;
                *lookup_data.range_check_252_width_27_5 = [
                    M31_1090315331,
                    combination_limb_0_col83,
                    combination_limb_1_col84,
                    combination_limb_2_col85,
                    combination_limb_3_col86,
                    combination_limb_4_col87,
                    combination_limb_5_col88,
                    combination_limb_6_col89,
                    combination_limb_7_col90,
                    combination_limb_8_col91,
                    combination_limb_9_col92,
                ];

                // Linear Combination N 1 Coefs 2.

                let combination_tmp_8c14f_14 = PackedFelt252Width27::from_packed_felt252(
                    ((Felt252_0_0_0_0)
                        + ((Felt252_2_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_8c14f_13,
                            )))),
                );
                let combination_limb_0_col94 = combination_tmp_8c14f_14.get_m31(0);
                *row[94] = combination_limb_0_col94;
                let combination_limb_1_col95 = combination_tmp_8c14f_14.get_m31(1);
                *row[95] = combination_limb_1_col95;
                let combination_limb_2_col96 = combination_tmp_8c14f_14.get_m31(2);
                *row[96] = combination_limb_2_col96;
                let combination_limb_3_col97 = combination_tmp_8c14f_14.get_m31(3);
                *row[97] = combination_limb_3_col97;
                let combination_limb_4_col98 = combination_tmp_8c14f_14.get_m31(4);
                *row[98] = combination_limb_4_col98;
                let combination_limb_5_col99 = combination_tmp_8c14f_14.get_m31(5);
                *row[99] = combination_limb_5_col99;
                let combination_limb_6_col100 = combination_tmp_8c14f_14.get_m31(6);
                *row[100] = combination_limb_6_col100;
                let combination_limb_7_col101 = combination_tmp_8c14f_14.get_m31(7);
                *row[101] = combination_limb_7_col101;
                let combination_limb_8_col102 = combination_tmp_8c14f_14.get_m31(8);
                *row[102] = combination_limb_8_col102;
                let combination_limb_9_col103 = combination_tmp_8c14f_14.get_m31(9);
                *row[103] = combination_limb_9_col103;
                let biased_limb_accumulator_u32_tmp_8c14f_15 = PackedUInt32::from_m31(
                    ((((M31_2) * (combination_limb_0_col83)) - (combination_limb_0_col94))
                        + (M31_134217729)),
                );
                let p_coef_col104 =
                    ((biased_limb_accumulator_u32_tmp_8c14f_15.low().as_m31()) - (M31_1));
                *row[104] = p_coef_col104;
                let carry_0_tmp_8c14f_16 = (((((M31_2) * (combination_limb_0_col83))
                    - (combination_limb_0_col94))
                    - (p_coef_col104))
                    * (M31_16));
                let carry_1_tmp_8c14f_17 = ((((carry_0_tmp_8c14f_16)
                    + ((M31_2) * (combination_limb_1_col84)))
                    - (combination_limb_1_col95))
                    * (M31_16));
                let carry_2_tmp_8c14f_18 = ((((carry_1_tmp_8c14f_17)
                    + ((M31_2) * (combination_limb_2_col85)))
                    - (combination_limb_2_col96))
                    * (M31_16));
                let carry_3_tmp_8c14f_19 = ((((carry_2_tmp_8c14f_18)
                    + ((M31_2) * (combination_limb_3_col86)))
                    - (combination_limb_3_col97))
                    * (M31_16));
                let carry_4_tmp_8c14f_20 = ((((carry_3_tmp_8c14f_19)
                    + ((M31_2) * (combination_limb_4_col87)))
                    - (combination_limb_4_col98))
                    * (M31_16));
                let carry_5_tmp_8c14f_21 = ((((carry_4_tmp_8c14f_20)
                    + ((M31_2) * (combination_limb_5_col88)))
                    - (combination_limb_5_col99))
                    * (M31_16));
                let carry_6_tmp_8c14f_22 = ((((carry_5_tmp_8c14f_21)
                    + ((M31_2) * (combination_limb_6_col89)))
                    - (combination_limb_6_col100))
                    * (M31_16));
                let carry_7_tmp_8c14f_23 = (((((carry_6_tmp_8c14f_22)
                    + ((M31_2) * (combination_limb_7_col90)))
                    - (combination_limb_7_col101))
                    - ((p_coef_col104) * (M31_136)))
                    * (M31_16));
                let carry_8_tmp_8c14f_24 = ((((carry_7_tmp_8c14f_23)
                    + ((M31_2) * (combination_limb_8_col91)))
                    - (combination_limb_8_col102))
                    * (M31_16));
                let linear_combination_n_1_coefs_2_output_tmp_8c14f_34 = combination_tmp_8c14f_14;

                let poseidon_partial_round_output_tmp_8c14f_35 = [
                    cube_252_output_tmp_8c14f_1,
                    linear_combination_n_1_coefs_2_output_tmp_8c14f_34,
                ];

                // Poseidon Partial Round.

                *sub_component_inputs.cube_252[1] = poseidon_partial_round_output_tmp_8c14f_35[1];
                let cube_252_output_tmp_8c14f_36 =
                    PackedCube252::deduce_output(poseidon_partial_round_output_tmp_8c14f_35[1]);
                let cube_252_output_limb_0_col105 = cube_252_output_tmp_8c14f_36.get_m31(0);
                *row[105] = cube_252_output_limb_0_col105;
                let cube_252_output_limb_1_col106 = cube_252_output_tmp_8c14f_36.get_m31(1);
                *row[106] = cube_252_output_limb_1_col106;
                let cube_252_output_limb_2_col107 = cube_252_output_tmp_8c14f_36.get_m31(2);
                *row[107] = cube_252_output_limb_2_col107;
                let cube_252_output_limb_3_col108 = cube_252_output_tmp_8c14f_36.get_m31(3);
                *row[108] = cube_252_output_limb_3_col108;
                let cube_252_output_limb_4_col109 = cube_252_output_tmp_8c14f_36.get_m31(4);
                *row[109] = cube_252_output_limb_4_col109;
                let cube_252_output_limb_5_col110 = cube_252_output_tmp_8c14f_36.get_m31(5);
                *row[110] = cube_252_output_limb_5_col110;
                let cube_252_output_limb_6_col111 = cube_252_output_tmp_8c14f_36.get_m31(6);
                *row[111] = cube_252_output_limb_6_col111;
                let cube_252_output_limb_7_col112 = cube_252_output_tmp_8c14f_36.get_m31(7);
                *row[112] = cube_252_output_limb_7_col112;
                let cube_252_output_limb_8_col113 = cube_252_output_tmp_8c14f_36.get_m31(8);
                *row[113] = cube_252_output_limb_8_col113;
                let cube_252_output_limb_9_col114 = cube_252_output_tmp_8c14f_36.get_m31(9);
                *row[114] = cube_252_output_limb_9_col114;
                *lookup_data.cube_252_6 = [
                    M31_1987997202,
                    combination_limb_0_col94,
                    combination_limb_1_col95,
                    combination_limb_2_col96,
                    combination_limb_3_col97,
                    combination_limb_4_col98,
                    combination_limb_5_col99,
                    combination_limb_6_col100,
                    combination_limb_7_col101,
                    combination_limb_8_col102,
                    combination_limb_9_col103,
                    cube_252_output_limb_0_col105,
                    cube_252_output_limb_1_col106,
                    cube_252_output_limb_2_col107,
                    cube_252_output_limb_3_col108,
                    cube_252_output_limb_4_col109,
                    cube_252_output_limb_5_col110,
                    cube_252_output_limb_6_col111,
                    cube_252_output_limb_7_col112,
                    cube_252_output_limb_8_col113,
                    cube_252_output_limb_9_col114,
                ];

                // Linear Combination N 6 Coefs 4 2 3 1 M 1 1.

                let combination_tmp_8c14f_37 = PackedFelt252Width27::from_packed_felt252(
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
                                poseidon_partial_round_output_tmp_8c14f_35[0],
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_partial_round_output_tmp_8c14f_35[1],
                            ))))
                        - ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_8c14f_36,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_round_keys_output_tmp_8c14f_0[1],
                            )))),
                );
                let combination_limb_0_col115 = combination_tmp_8c14f_37.get_m31(0);
                *row[115] = combination_limb_0_col115;
                let combination_limb_1_col116 = combination_tmp_8c14f_37.get_m31(1);
                *row[116] = combination_limb_1_col116;
                let combination_limb_2_col117 = combination_tmp_8c14f_37.get_m31(2);
                *row[117] = combination_limb_2_col117;
                let combination_limb_3_col118 = combination_tmp_8c14f_37.get_m31(3);
                *row[118] = combination_limb_3_col118;
                let combination_limb_4_col119 = combination_tmp_8c14f_37.get_m31(4);
                *row[119] = combination_limb_4_col119;
                let combination_limb_5_col120 = combination_tmp_8c14f_37.get_m31(5);
                *row[120] = combination_limb_5_col120;
                let combination_limb_6_col121 = combination_tmp_8c14f_37.get_m31(6);
                *row[121] = combination_limb_6_col121;
                let combination_limb_7_col122 = combination_tmp_8c14f_37.get_m31(7);
                *row[122] = combination_limb_7_col122;
                let combination_limb_8_col123 = combination_tmp_8c14f_37.get_m31(8);
                *row[123] = combination_limb_8_col123;
                let combination_limb_9_col124 = combination_tmp_8c14f_37.get_m31(9);
                *row[124] = combination_limb_9_col124;
                let biased_limb_accumulator_u32_tmp_8c14f_38 = PackedUInt32::from_m31(
                    (((((((((M31_4) * (input_limb_22_col23))
                        + ((M31_2) * (input_limb_32_col33)))
                        + ((M31_3) * (cube_252_output_limb_0_col73)))
                        + (combination_limb_0_col94))
                        - (cube_252_output_limb_0_col105))
                        + (poseidon_round_keys_output_limb_10_col53))
                        - (combination_limb_0_col115))
                        + (M31_268435458)),
                );
                let p_coef_col125 =
                    ((biased_limb_accumulator_u32_tmp_8c14f_38.low().as_m31()) - (M31_2));
                *row[125] = p_coef_col125;
                let carry_0_tmp_8c14f_39 = ((((((((((M31_4) * (input_limb_22_col23))
                    + ((M31_2) * (input_limb_32_col33)))
                    + ((M31_3) * (cube_252_output_limb_0_col73)))
                    + (combination_limb_0_col94))
                    - (cube_252_output_limb_0_col105))
                    + (poseidon_round_keys_output_limb_10_col53))
                    - (combination_limb_0_col115))
                    - (p_coef_col125))
                    * (M31_16));
                let carry_1_tmp_8c14f_40 = (((((((((carry_0_tmp_8c14f_39)
                    + ((M31_4) * (input_limb_23_col24)))
                    + ((M31_2) * (input_limb_33_col34)))
                    + ((M31_3) * (cube_252_output_limb_1_col74)))
                    + (combination_limb_1_col95))
                    - (cube_252_output_limb_1_col106))
                    + (poseidon_round_keys_output_limb_11_col54))
                    - (combination_limb_1_col116))
                    * (M31_16));
                let carry_2_tmp_8c14f_41 = (((((((((carry_1_tmp_8c14f_40)
                    + ((M31_4) * (input_limb_24_col25)))
                    + ((M31_2) * (input_limb_34_col35)))
                    + ((M31_3) * (cube_252_output_limb_2_col75)))
                    + (combination_limb_2_col96))
                    - (cube_252_output_limb_2_col107))
                    + (poseidon_round_keys_output_limb_12_col55))
                    - (combination_limb_2_col117))
                    * (M31_16));
                let carry_3_tmp_8c14f_42 = (((((((((carry_2_tmp_8c14f_41)
                    + ((M31_4) * (input_limb_25_col26)))
                    + ((M31_2) * (input_limb_35_col36)))
                    + ((M31_3) * (cube_252_output_limb_3_col76)))
                    + (combination_limb_3_col97))
                    - (cube_252_output_limb_3_col108))
                    + (poseidon_round_keys_output_limb_13_col56))
                    - (combination_limb_3_col118))
                    * (M31_16));
                let carry_4_tmp_8c14f_43 = (((((((((carry_3_tmp_8c14f_42)
                    + ((M31_4) * (input_limb_26_col27)))
                    + ((M31_2) * (input_limb_36_col37)))
                    + ((M31_3) * (cube_252_output_limb_4_col77)))
                    + (combination_limb_4_col98))
                    - (cube_252_output_limb_4_col109))
                    + (poseidon_round_keys_output_limb_14_col57))
                    - (combination_limb_4_col119))
                    * (M31_16));
                let carry_5_tmp_8c14f_44 = (((((((((carry_4_tmp_8c14f_43)
                    + ((M31_4) * (input_limb_27_col28)))
                    + ((M31_2) * (input_limb_37_col38)))
                    + ((M31_3) * (cube_252_output_limb_5_col78)))
                    + (combination_limb_5_col99))
                    - (cube_252_output_limb_5_col110))
                    + (poseidon_round_keys_output_limb_15_col58))
                    - (combination_limb_5_col120))
                    * (M31_16));
                let carry_6_tmp_8c14f_45 = (((((((((carry_5_tmp_8c14f_44)
                    + ((M31_4) * (input_limb_28_col29)))
                    + ((M31_2) * (input_limb_38_col39)))
                    + ((M31_3) * (cube_252_output_limb_6_col79)))
                    + (combination_limb_6_col100))
                    - (cube_252_output_limb_6_col111))
                    + (poseidon_round_keys_output_limb_16_col59))
                    - (combination_limb_6_col121))
                    * (M31_16));
                let carry_7_tmp_8c14f_46 = ((((((((((carry_6_tmp_8c14f_45)
                    + ((M31_4) * (input_limb_29_col30)))
                    + ((M31_2) * (input_limb_39_col40)))
                    + ((M31_3) * (cube_252_output_limb_7_col80)))
                    + (combination_limb_7_col101))
                    - (cube_252_output_limb_7_col112))
                    + (poseidon_round_keys_output_limb_17_col60))
                    - (combination_limb_7_col122))
                    - ((p_coef_col125) * (M31_136)))
                    * (M31_16));
                let carry_8_tmp_8c14f_47 = (((((((((carry_7_tmp_8c14f_46)
                    + ((M31_4) * (input_limb_30_col31)))
                    + ((M31_2) * (input_limb_40_col41)))
                    + ((M31_3) * (cube_252_output_limb_8_col81)))
                    + (combination_limb_8_col102))
                    - (cube_252_output_limb_8_col113))
                    + (poseidon_round_keys_output_limb_18_col61))
                    - (combination_limb_8_col123))
                    * (M31_16));
                *sub_component_inputs.range_check_4_4_4_4[2] = [
                    ((p_coef_col125) + (M31_2)),
                    ((carry_0_tmp_8c14f_39) + (M31_2)),
                    ((carry_1_tmp_8c14f_40) + (M31_2)),
                    ((carry_2_tmp_8c14f_41) + (M31_2)),
                ];
                *lookup_data.range_check_4_4_4_4_7 = [
                    M31_1027333874,
                    ((p_coef_col125) + (M31_2)),
                    ((carry_0_tmp_8c14f_39) + (M31_2)),
                    ((carry_1_tmp_8c14f_40) + (M31_2)),
                    ((carry_2_tmp_8c14f_41) + (M31_2)),
                ];
                *sub_component_inputs.range_check_4_4_4_4[3] = [
                    ((carry_3_tmp_8c14f_42) + (M31_2)),
                    ((carry_4_tmp_8c14f_43) + (M31_2)),
                    ((carry_5_tmp_8c14f_44) + (M31_2)),
                    ((carry_6_tmp_8c14f_45) + (M31_2)),
                ];
                *lookup_data.range_check_4_4_4_4_8 = [
                    M31_1027333874,
                    ((carry_3_tmp_8c14f_42) + (M31_2)),
                    ((carry_4_tmp_8c14f_43) + (M31_2)),
                    ((carry_5_tmp_8c14f_44) + (M31_2)),
                    ((carry_6_tmp_8c14f_45) + (M31_2)),
                ];
                *sub_component_inputs.range_check_4_4[1] = [
                    ((carry_7_tmp_8c14f_46) + (M31_2)),
                    ((carry_8_tmp_8c14f_47) + (M31_2)),
                ];
                *lookup_data.range_check_4_4_9 = [
                    M31_1651211826,
                    ((carry_7_tmp_8c14f_46) + (M31_2)),
                    ((carry_8_tmp_8c14f_47) + (M31_2)),
                ];
                let linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_8c14f_48 =
                    combination_tmp_8c14f_37;

                *sub_component_inputs.range_check_252_width_27[1] =
                    linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_8c14f_48;
                *lookup_data.range_check_252_width_27_10 = [
                    M31_1090315331,
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

                // Linear Combination N 1 Coefs 2.

                let combination_tmp_8c14f_49 = PackedFelt252Width27::from_packed_felt252(
                    ((Felt252_0_0_0_0)
                        + ((Felt252_2_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_8c14f_48,
                            )))),
                );
                let combination_limb_0_col126 = combination_tmp_8c14f_49.get_m31(0);
                *row[126] = combination_limb_0_col126;
                let combination_limb_1_col127 = combination_tmp_8c14f_49.get_m31(1);
                *row[127] = combination_limb_1_col127;
                let combination_limb_2_col128 = combination_tmp_8c14f_49.get_m31(2);
                *row[128] = combination_limb_2_col128;
                let combination_limb_3_col129 = combination_tmp_8c14f_49.get_m31(3);
                *row[129] = combination_limb_3_col129;
                let combination_limb_4_col130 = combination_tmp_8c14f_49.get_m31(4);
                *row[130] = combination_limb_4_col130;
                let combination_limb_5_col131 = combination_tmp_8c14f_49.get_m31(5);
                *row[131] = combination_limb_5_col131;
                let combination_limb_6_col132 = combination_tmp_8c14f_49.get_m31(6);
                *row[132] = combination_limb_6_col132;
                let combination_limb_7_col133 = combination_tmp_8c14f_49.get_m31(7);
                *row[133] = combination_limb_7_col133;
                let combination_limb_8_col134 = combination_tmp_8c14f_49.get_m31(8);
                *row[134] = combination_limb_8_col134;
                let combination_limb_9_col135 = combination_tmp_8c14f_49.get_m31(9);
                *row[135] = combination_limb_9_col135;
                let biased_limb_accumulator_u32_tmp_8c14f_50 = PackedUInt32::from_m31(
                    ((((M31_2) * (combination_limb_0_col115)) - (combination_limb_0_col126))
                        + (M31_134217729)),
                );
                let p_coef_col136 =
                    ((biased_limb_accumulator_u32_tmp_8c14f_50.low().as_m31()) - (M31_1));
                *row[136] = p_coef_col136;
                let carry_0_tmp_8c14f_51 = (((((M31_2) * (combination_limb_0_col115))
                    - (combination_limb_0_col126))
                    - (p_coef_col136))
                    * (M31_16));
                let carry_1_tmp_8c14f_52 = ((((carry_0_tmp_8c14f_51)
                    + ((M31_2) * (combination_limb_1_col116)))
                    - (combination_limb_1_col127))
                    * (M31_16));
                let carry_2_tmp_8c14f_53 = ((((carry_1_tmp_8c14f_52)
                    + ((M31_2) * (combination_limb_2_col117)))
                    - (combination_limb_2_col128))
                    * (M31_16));
                let carry_3_tmp_8c14f_54 = ((((carry_2_tmp_8c14f_53)
                    + ((M31_2) * (combination_limb_3_col118)))
                    - (combination_limb_3_col129))
                    * (M31_16));
                let carry_4_tmp_8c14f_55 = ((((carry_3_tmp_8c14f_54)
                    + ((M31_2) * (combination_limb_4_col119)))
                    - (combination_limb_4_col130))
                    * (M31_16));
                let carry_5_tmp_8c14f_56 = ((((carry_4_tmp_8c14f_55)
                    + ((M31_2) * (combination_limb_5_col120)))
                    - (combination_limb_5_col131))
                    * (M31_16));
                let carry_6_tmp_8c14f_57 = ((((carry_5_tmp_8c14f_56)
                    + ((M31_2) * (combination_limb_6_col121)))
                    - (combination_limb_6_col132))
                    * (M31_16));
                let carry_7_tmp_8c14f_58 = (((((carry_6_tmp_8c14f_57)
                    + ((M31_2) * (combination_limb_7_col122)))
                    - (combination_limb_7_col133))
                    - ((p_coef_col136) * (M31_136)))
                    * (M31_16));
                let carry_8_tmp_8c14f_59 = ((((carry_7_tmp_8c14f_58)
                    + ((M31_2) * (combination_limb_8_col123)))
                    - (combination_limb_8_col134))
                    * (M31_16));
                let linear_combination_n_1_coefs_2_output_tmp_8c14f_69 = combination_tmp_8c14f_49;

                let poseidon_partial_round_output_tmp_8c14f_70 = [
                    cube_252_output_tmp_8c14f_36,
                    linear_combination_n_1_coefs_2_output_tmp_8c14f_69,
                ];

                // Poseidon Partial Round.

                *sub_component_inputs.cube_252[2] = poseidon_partial_round_output_tmp_8c14f_70[1];
                let cube_252_output_tmp_8c14f_71 =
                    PackedCube252::deduce_output(poseidon_partial_round_output_tmp_8c14f_70[1]);
                let cube_252_output_limb_0_col137 = cube_252_output_tmp_8c14f_71.get_m31(0);
                *row[137] = cube_252_output_limb_0_col137;
                let cube_252_output_limb_1_col138 = cube_252_output_tmp_8c14f_71.get_m31(1);
                *row[138] = cube_252_output_limb_1_col138;
                let cube_252_output_limb_2_col139 = cube_252_output_tmp_8c14f_71.get_m31(2);
                *row[139] = cube_252_output_limb_2_col139;
                let cube_252_output_limb_3_col140 = cube_252_output_tmp_8c14f_71.get_m31(3);
                *row[140] = cube_252_output_limb_3_col140;
                let cube_252_output_limb_4_col141 = cube_252_output_tmp_8c14f_71.get_m31(4);
                *row[141] = cube_252_output_limb_4_col141;
                let cube_252_output_limb_5_col142 = cube_252_output_tmp_8c14f_71.get_m31(5);
                *row[142] = cube_252_output_limb_5_col142;
                let cube_252_output_limb_6_col143 = cube_252_output_tmp_8c14f_71.get_m31(6);
                *row[143] = cube_252_output_limb_6_col143;
                let cube_252_output_limb_7_col144 = cube_252_output_tmp_8c14f_71.get_m31(7);
                *row[144] = cube_252_output_limb_7_col144;
                let cube_252_output_limb_8_col145 = cube_252_output_tmp_8c14f_71.get_m31(8);
                *row[145] = cube_252_output_limb_8_col145;
                let cube_252_output_limb_9_col146 = cube_252_output_tmp_8c14f_71.get_m31(9);
                *row[146] = cube_252_output_limb_9_col146;
                *lookup_data.cube_252_11 = [
                    M31_1987997202,
                    combination_limb_0_col126,
                    combination_limb_1_col127,
                    combination_limb_2_col128,
                    combination_limb_3_col129,
                    combination_limb_4_col130,
                    combination_limb_5_col131,
                    combination_limb_6_col132,
                    combination_limb_7_col133,
                    combination_limb_8_col134,
                    combination_limb_9_col135,
                    cube_252_output_limb_0_col137,
                    cube_252_output_limb_1_col138,
                    cube_252_output_limb_2_col139,
                    cube_252_output_limb_3_col140,
                    cube_252_output_limb_4_col141,
                    cube_252_output_limb_5_col142,
                    cube_252_output_limb_6_col143,
                    cube_252_output_limb_7_col144,
                    cube_252_output_limb_8_col145,
                    cube_252_output_limb_9_col146,
                ];

                // Linear Combination N 6 Coefs 4 2 3 1 M 1 1.

                let combination_tmp_8c14f_72 = PackedFelt252Width27::from_packed_felt252(
                    (((((((Felt252_0_0_0_0)
                        + ((Felt252_4_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_partial_round_output_tmp_8c14f_35[0],
                            ))))
                        + ((Felt252_2_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_partial_round_output_tmp_8c14f_35[1],
                            ))))
                        + ((Felt252_3_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_partial_round_output_tmp_8c14f_70[0],
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_partial_round_output_tmp_8c14f_70[1],
                            ))))
                        - ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_8c14f_71,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_round_keys_output_tmp_8c14f_0[2],
                            )))),
                );
                let combination_limb_0_col147 = combination_tmp_8c14f_72.get_m31(0);
                *row[147] = combination_limb_0_col147;
                let combination_limb_1_col148 = combination_tmp_8c14f_72.get_m31(1);
                *row[148] = combination_limb_1_col148;
                let combination_limb_2_col149 = combination_tmp_8c14f_72.get_m31(2);
                *row[149] = combination_limb_2_col149;
                let combination_limb_3_col150 = combination_tmp_8c14f_72.get_m31(3);
                *row[150] = combination_limb_3_col150;
                let combination_limb_4_col151 = combination_tmp_8c14f_72.get_m31(4);
                *row[151] = combination_limb_4_col151;
                let combination_limb_5_col152 = combination_tmp_8c14f_72.get_m31(5);
                *row[152] = combination_limb_5_col152;
                let combination_limb_6_col153 = combination_tmp_8c14f_72.get_m31(6);
                *row[153] = combination_limb_6_col153;
                let combination_limb_7_col154 = combination_tmp_8c14f_72.get_m31(7);
                *row[154] = combination_limb_7_col154;
                let combination_limb_8_col155 = combination_tmp_8c14f_72.get_m31(8);
                *row[155] = combination_limb_8_col155;
                let combination_limb_9_col156 = combination_tmp_8c14f_72.get_m31(9);
                *row[156] = combination_limb_9_col156;
                let biased_limb_accumulator_u32_tmp_8c14f_73 = PackedUInt32::from_m31(
                    (((((((((M31_4) * (cube_252_output_limb_0_col73))
                        + ((M31_2) * (combination_limb_0_col94)))
                        + ((M31_3) * (cube_252_output_limb_0_col105)))
                        + (combination_limb_0_col126))
                        - (cube_252_output_limb_0_col137))
                        + (poseidon_round_keys_output_limb_20_col63))
                        - (combination_limb_0_col147))
                        + (M31_268435458)),
                );
                let p_coef_col157 =
                    ((biased_limb_accumulator_u32_tmp_8c14f_73.low().as_m31()) - (M31_2));
                *row[157] = p_coef_col157;
                let carry_0_tmp_8c14f_74 = ((((((((((M31_4)
                    * (cube_252_output_limb_0_col73))
                    + ((M31_2) * (combination_limb_0_col94)))
                    + ((M31_3) * (cube_252_output_limb_0_col105)))
                    + (combination_limb_0_col126))
                    - (cube_252_output_limb_0_col137))
                    + (poseidon_round_keys_output_limb_20_col63))
                    - (combination_limb_0_col147))
                    - (p_coef_col157))
                    * (M31_16));
                let carry_1_tmp_8c14f_75 = (((((((((carry_0_tmp_8c14f_74)
                    + ((M31_4) * (cube_252_output_limb_1_col74)))
                    + ((M31_2) * (combination_limb_1_col95)))
                    + ((M31_3) * (cube_252_output_limb_1_col106)))
                    + (combination_limb_1_col127))
                    - (cube_252_output_limb_1_col138))
                    + (poseidon_round_keys_output_limb_21_col64))
                    - (combination_limb_1_col148))
                    * (M31_16));
                let carry_2_tmp_8c14f_76 = (((((((((carry_1_tmp_8c14f_75)
                    + ((M31_4) * (cube_252_output_limb_2_col75)))
                    + ((M31_2) * (combination_limb_2_col96)))
                    + ((M31_3) * (cube_252_output_limb_2_col107)))
                    + (combination_limb_2_col128))
                    - (cube_252_output_limb_2_col139))
                    + (poseidon_round_keys_output_limb_22_col65))
                    - (combination_limb_2_col149))
                    * (M31_16));
                let carry_3_tmp_8c14f_77 = (((((((((carry_2_tmp_8c14f_76)
                    + ((M31_4) * (cube_252_output_limb_3_col76)))
                    + ((M31_2) * (combination_limb_3_col97)))
                    + ((M31_3) * (cube_252_output_limb_3_col108)))
                    + (combination_limb_3_col129))
                    - (cube_252_output_limb_3_col140))
                    + (poseidon_round_keys_output_limb_23_col66))
                    - (combination_limb_3_col150))
                    * (M31_16));
                let carry_4_tmp_8c14f_78 = (((((((((carry_3_tmp_8c14f_77)
                    + ((M31_4) * (cube_252_output_limb_4_col77)))
                    + ((M31_2) * (combination_limb_4_col98)))
                    + ((M31_3) * (cube_252_output_limb_4_col109)))
                    + (combination_limb_4_col130))
                    - (cube_252_output_limb_4_col141))
                    + (poseidon_round_keys_output_limb_24_col67))
                    - (combination_limb_4_col151))
                    * (M31_16));
                let carry_5_tmp_8c14f_79 = (((((((((carry_4_tmp_8c14f_78)
                    + ((M31_4) * (cube_252_output_limb_5_col78)))
                    + ((M31_2) * (combination_limb_5_col99)))
                    + ((M31_3) * (cube_252_output_limb_5_col110)))
                    + (combination_limb_5_col131))
                    - (cube_252_output_limb_5_col142))
                    + (poseidon_round_keys_output_limb_25_col68))
                    - (combination_limb_5_col152))
                    * (M31_16));
                let carry_6_tmp_8c14f_80 = (((((((((carry_5_tmp_8c14f_79)
                    + ((M31_4) * (cube_252_output_limb_6_col79)))
                    + ((M31_2) * (combination_limb_6_col100)))
                    + ((M31_3) * (cube_252_output_limb_6_col111)))
                    + (combination_limb_6_col132))
                    - (cube_252_output_limb_6_col143))
                    + (poseidon_round_keys_output_limb_26_col69))
                    - (combination_limb_6_col153))
                    * (M31_16));
                let carry_7_tmp_8c14f_81 = ((((((((((carry_6_tmp_8c14f_80)
                    + ((M31_4) * (cube_252_output_limb_7_col80)))
                    + ((M31_2) * (combination_limb_7_col101)))
                    + ((M31_3) * (cube_252_output_limb_7_col112)))
                    + (combination_limb_7_col133))
                    - (cube_252_output_limb_7_col144))
                    + (poseidon_round_keys_output_limb_27_col70))
                    - (combination_limb_7_col154))
                    - ((p_coef_col157) * (M31_136)))
                    * (M31_16));
                let carry_8_tmp_8c14f_82 = (((((((((carry_7_tmp_8c14f_81)
                    + ((M31_4) * (cube_252_output_limb_8_col81)))
                    + ((M31_2) * (combination_limb_8_col102)))
                    + ((M31_3) * (cube_252_output_limb_8_col113)))
                    + (combination_limb_8_col134))
                    - (cube_252_output_limb_8_col145))
                    + (poseidon_round_keys_output_limb_28_col71))
                    - (combination_limb_8_col155))
                    * (M31_16));
                *sub_component_inputs.range_check_4_4_4_4[4] = [
                    ((p_coef_col157) + (M31_2)),
                    ((carry_0_tmp_8c14f_74) + (M31_2)),
                    ((carry_1_tmp_8c14f_75) + (M31_2)),
                    ((carry_2_tmp_8c14f_76) + (M31_2)),
                ];
                *lookup_data.range_check_4_4_4_4_12 = [
                    M31_1027333874,
                    ((p_coef_col157) + (M31_2)),
                    ((carry_0_tmp_8c14f_74) + (M31_2)),
                    ((carry_1_tmp_8c14f_75) + (M31_2)),
                    ((carry_2_tmp_8c14f_76) + (M31_2)),
                ];
                *sub_component_inputs.range_check_4_4_4_4[5] = [
                    ((carry_3_tmp_8c14f_77) + (M31_2)),
                    ((carry_4_tmp_8c14f_78) + (M31_2)),
                    ((carry_5_tmp_8c14f_79) + (M31_2)),
                    ((carry_6_tmp_8c14f_80) + (M31_2)),
                ];
                *lookup_data.range_check_4_4_4_4_13 = [
                    M31_1027333874,
                    ((carry_3_tmp_8c14f_77) + (M31_2)),
                    ((carry_4_tmp_8c14f_78) + (M31_2)),
                    ((carry_5_tmp_8c14f_79) + (M31_2)),
                    ((carry_6_tmp_8c14f_80) + (M31_2)),
                ];
                *sub_component_inputs.range_check_4_4[2] = [
                    ((carry_7_tmp_8c14f_81) + (M31_2)),
                    ((carry_8_tmp_8c14f_82) + (M31_2)),
                ];
                *lookup_data.range_check_4_4_14 = [
                    M31_1651211826,
                    ((carry_7_tmp_8c14f_81) + (M31_2)),
                    ((carry_8_tmp_8c14f_82) + (M31_2)),
                ];
                let linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_8c14f_83 =
                    combination_tmp_8c14f_72;

                *sub_component_inputs.range_check_252_width_27[2] =
                    linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_8c14f_83;
                *lookup_data.range_check_252_width_27_15 = [
                    M31_1090315331,
                    combination_limb_0_col147,
                    combination_limb_1_col148,
                    combination_limb_2_col149,
                    combination_limb_3_col150,
                    combination_limb_4_col151,
                    combination_limb_5_col152,
                    combination_limb_6_col153,
                    combination_limb_7_col154,
                    combination_limb_8_col155,
                    combination_limb_9_col156,
                ];

                // Linear Combination N 1 Coefs 2.

                let combination_tmp_8c14f_84 = PackedFelt252Width27::from_packed_felt252(
                    ((Felt252_0_0_0_0)
                        + ((Felt252_2_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_8c14f_83,
                            )))),
                );
                let combination_limb_0_col158 = combination_tmp_8c14f_84.get_m31(0);
                *row[158] = combination_limb_0_col158;
                let combination_limb_1_col159 = combination_tmp_8c14f_84.get_m31(1);
                *row[159] = combination_limb_1_col159;
                let combination_limb_2_col160 = combination_tmp_8c14f_84.get_m31(2);
                *row[160] = combination_limb_2_col160;
                let combination_limb_3_col161 = combination_tmp_8c14f_84.get_m31(3);
                *row[161] = combination_limb_3_col161;
                let combination_limb_4_col162 = combination_tmp_8c14f_84.get_m31(4);
                *row[162] = combination_limb_4_col162;
                let combination_limb_5_col163 = combination_tmp_8c14f_84.get_m31(5);
                *row[163] = combination_limb_5_col163;
                let combination_limb_6_col164 = combination_tmp_8c14f_84.get_m31(6);
                *row[164] = combination_limb_6_col164;
                let combination_limb_7_col165 = combination_tmp_8c14f_84.get_m31(7);
                *row[165] = combination_limb_7_col165;
                let combination_limb_8_col166 = combination_tmp_8c14f_84.get_m31(8);
                *row[166] = combination_limb_8_col166;
                let combination_limb_9_col167 = combination_tmp_8c14f_84.get_m31(9);
                *row[167] = combination_limb_9_col167;
                let biased_limb_accumulator_u32_tmp_8c14f_85 = PackedUInt32::from_m31(
                    ((((M31_2) * (combination_limb_0_col147)) - (combination_limb_0_col158))
                        + (M31_134217729)),
                );
                let p_coef_col168 =
                    ((biased_limb_accumulator_u32_tmp_8c14f_85.low().as_m31()) - (M31_1));
                *row[168] = p_coef_col168;
                let carry_0_tmp_8c14f_86 = (((((M31_2) * (combination_limb_0_col147))
                    - (combination_limb_0_col158))
                    - (p_coef_col168))
                    * (M31_16));
                let carry_1_tmp_8c14f_87 = ((((carry_0_tmp_8c14f_86)
                    + ((M31_2) * (combination_limb_1_col148)))
                    - (combination_limb_1_col159))
                    * (M31_16));
                let carry_2_tmp_8c14f_88 = ((((carry_1_tmp_8c14f_87)
                    + ((M31_2) * (combination_limb_2_col149)))
                    - (combination_limb_2_col160))
                    * (M31_16));
                let carry_3_tmp_8c14f_89 = ((((carry_2_tmp_8c14f_88)
                    + ((M31_2) * (combination_limb_3_col150)))
                    - (combination_limb_3_col161))
                    * (M31_16));
                let carry_4_tmp_8c14f_90 = ((((carry_3_tmp_8c14f_89)
                    + ((M31_2) * (combination_limb_4_col151)))
                    - (combination_limb_4_col162))
                    * (M31_16));
                let carry_5_tmp_8c14f_91 = ((((carry_4_tmp_8c14f_90)
                    + ((M31_2) * (combination_limb_5_col152)))
                    - (combination_limb_5_col163))
                    * (M31_16));
                let carry_6_tmp_8c14f_92 = ((((carry_5_tmp_8c14f_91)
                    + ((M31_2) * (combination_limb_6_col153)))
                    - (combination_limb_6_col164))
                    * (M31_16));
                let carry_7_tmp_8c14f_93 = (((((carry_6_tmp_8c14f_92)
                    + ((M31_2) * (combination_limb_7_col154)))
                    - (combination_limb_7_col165))
                    - ((p_coef_col168) * (M31_136)))
                    * (M31_16));
                let carry_8_tmp_8c14f_94 = ((((carry_7_tmp_8c14f_93)
                    + ((M31_2) * (combination_limb_8_col155)))
                    - (combination_limb_8_col166))
                    * (M31_16));
                let linear_combination_n_1_coefs_2_output_tmp_8c14f_104 = combination_tmp_8c14f_84;

                let poseidon_partial_round_output_tmp_8c14f_105 = [
                    cube_252_output_tmp_8c14f_71,
                    linear_combination_n_1_coefs_2_output_tmp_8c14f_104,
                ];

                *lookup_data.poseidon_3_partial_rounds_chain_16 = [
                    M31_1343313504,
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
                ];
                *lookup_data.poseidon_3_partial_rounds_chain_17 = [
                    M31_1343313504,
                    input_limb_0_col1,
                    ((input_limb_1_col2) + (M31_1)),
                    cube_252_output_limb_0_col105,
                    cube_252_output_limb_1_col106,
                    cube_252_output_limb_2_col107,
                    cube_252_output_limb_3_col108,
                    cube_252_output_limb_4_col109,
                    cube_252_output_limb_5_col110,
                    cube_252_output_limb_6_col111,
                    cube_252_output_limb_7_col112,
                    cube_252_output_limb_8_col113,
                    cube_252_output_limb_9_col114,
                    combination_limb_0_col126,
                    combination_limb_1_col127,
                    combination_limb_2_col128,
                    combination_limb_3_col129,
                    combination_limb_4_col130,
                    combination_limb_5_col131,
                    combination_limb_6_col132,
                    combination_limb_7_col133,
                    combination_limb_8_col134,
                    combination_limb_9_col135,
                    cube_252_output_limb_0_col137,
                    cube_252_output_limb_1_col138,
                    cube_252_output_limb_2_col139,
                    cube_252_output_limb_3_col140,
                    cube_252_output_limb_4_col141,
                    cube_252_output_limb_5_col142,
                    cube_252_output_limb_6_col143,
                    cube_252_output_limb_7_col144,
                    cube_252_output_limb_8_col145,
                    cube_252_output_limb_9_col146,
                    combination_limb_0_col158,
                    combination_limb_1_col159,
                    combination_limb_2_col160,
                    combination_limb_3_col161,
                    combination_limb_4_col162,
                    combination_limb_5_col163,
                    combination_limb_6_col164,
                    combination_limb_7_col165,
                    combination_limb_8_col166,
                    combination_limb_9_col167,
                ];
                *lookup_data.mults_0 = enabler_col0;
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    poseidon_round_keys_0: Vec<[PackedM31; 32]>,
    cube_252_1: Vec<[PackedM31; 21]>,
    range_check_4_4_4_4_2: Vec<[PackedM31; 5]>,
    range_check_4_4_4_4_3: Vec<[PackedM31; 5]>,
    range_check_4_4_4: Vec<[PackedM31; 3]>,
    range_check_252_width_27_5: Vec<[PackedM31; 11]>,
    cube_252_6: Vec<[PackedM31; 21]>,
    range_check_4_4_4_4_7: Vec<[PackedM31; 5]>,
    range_check_4_4_4_4_8: Vec<[PackedM31; 5]>,
    range_check_4_4_9: Vec<[PackedM31; 3]>,
    range_check_252_width_27_10: Vec<[PackedM31; 11]>,
    cube_252_11: Vec<[PackedM31; 21]>,
    range_check_4_4_4_4_12: Vec<[PackedM31; 5]>,
    range_check_4_4_4_4_13: Vec<[PackedM31; 5]>,
    range_check_4_4_14: Vec<[PackedM31; 3]>,
    range_check_252_width_27_15: Vec<[PackedM31; 11]>,
    poseidon_3_partial_rounds_chain_16: Vec<[PackedM31; 43]>,
    poseidon_3_partial_rounds_chain_17: Vec<[PackedM31; 43]>,
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
            &self.lookup_data.poseidon_round_keys_0,
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
            &self.lookup_data.range_check_4_4_4_4_2,
            &self.lookup_data.range_check_4_4_4_4_3,
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
            &self.lookup_data.range_check_4_4_4,
            &self.lookup_data.range_check_252_width_27_5,
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
            &self.lookup_data.cube_252_6,
            &self.lookup_data.range_check_4_4_4_4_7,
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
            &self.lookup_data.range_check_4_4_4_4_8,
            &self.lookup_data.range_check_4_4_9,
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
            &self.lookup_data.range_check_252_width_27_10,
            &self.lookup_data.cube_252_11,
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
            &self.lookup_data.range_check_4_4_4_4_12,
            &self.lookup_data.range_check_4_4_4_4_13,
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
            &self.lookup_data.range_check_4_4_14,
            &self.lookup_data.range_check_252_width_27_15,
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
            &self.lookup_data.poseidon_3_partial_rounds_chain_16,
            &self.lookup_data.poseidon_3_partial_rounds_chain_17,
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
