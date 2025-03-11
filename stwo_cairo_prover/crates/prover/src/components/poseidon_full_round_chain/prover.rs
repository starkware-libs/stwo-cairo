#![allow(unused_parens)]
use itertools::Itertools;
use stwo_prover::core::backend::simd::conversion::Pack;

use super::component::{Claim, InteractionClaim};
use crate::components::prelude::proving::*;
use crate::components::range_check_vector::range_check_3_3_3_3_3;
use crate::components::{cube_252, poseidon_round_keys};

pub type InputType = (M31, M31, [Felt252Width27; 3]);
pub type PackedInputType = (PackedM31, PackedM31, [PackedFelt252Width27; 3]);
const N_TRACE_COLUMNS: usize = 126;

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
        cube_252_state: &mut cube_252::ClaimGenerator,
        poseidon_round_keys_state: &poseidon_round_keys::ClaimGenerator,
        range_check_3_3_3_3_3_state: &range_check_3_3_3_3_3::ClaimGenerator,
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
            cube_252_state,
            poseidon_round_keys_state,
            range_check_3_3_3_3_3_state,
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
    cube_252_state: &mut cube_252::ClaimGenerator,
    poseidon_round_keys_state: &poseidon_round_keys::ClaimGenerator,
    range_check_3_3_3_3_3_state: &range_check_3_3_3_3_3::ClaimGenerator,
) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData) {
    let log_n_packed_rows = inputs.len().ilog2();
    let log_size = log_n_packed_rows + LOG_N_LANES;
    let (mut trace, mut lookup_data) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
        )
    };

    let Felt252_0_0_0_0 = PackedFelt252::broadcast(Felt252::from([0, 0, 0, 0]));
    let Felt252_1_0_0_0 = PackedFelt252::broadcast(Felt252::from([1, 0, 0, 0]));
    let Felt252_2_0_0_0 = PackedFelt252::broadcast(Felt252::from([2, 0, 0, 0]));
    let Felt252_3_0_0_0 = PackedFelt252::broadcast(Felt252::from([3, 0, 0, 0]));
    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_134217729 = PackedM31::broadcast(M31::from(134217729));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_16 = PackedM31::broadcast(M31::from(16));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_268435458 = PackedM31::broadcast(M31::from(268435458));
    let M31_3 = PackedM31::broadcast(M31::from(3));
    let M31_402653187 = PackedM31::broadcast(M31::from(402653187));
    let padding_col = Enabler::new(n_rows);

    let mut cube_252_inputs_vec = vec![[[Felt252Width27::default(); 16]; 3]; n_rows];

    trace
        .par_iter_mut()
        .enumerate()
        .zip(inputs.into_par_iter())
        .zip(lookup_data.par_iter_mut())
        .zip(cube_252_inputs_vec.par_iter_mut())
        .for_each(
            |(
                (((row_index, mut row), poseidon_full_round_chain_input), lookup_data),
                cube_252_input,
            )| {
                let input_tmp_f9fbc_0 = (
                    poseidon_full_round_chain_input.0,
                    poseidon_full_round_chain_input.1,
                    [
                        poseidon_full_round_chain_input.2[0],
                        poseidon_full_round_chain_input.2[1],
                        poseidon_full_round_chain_input.2[2],
                    ],
                );
                let input_limb_0_col0 = input_tmp_f9fbc_0.0;
                *row[0] = input_limb_0_col0;
                let input_limb_1_col1 = input_tmp_f9fbc_0.1;
                *row[1] = input_limb_1_col1;
                let input_limb_2_col2 = input_tmp_f9fbc_0.2[0].get_m31(0);
                *row[2] = input_limb_2_col2;
                let input_limb_3_col3 = input_tmp_f9fbc_0.2[0].get_m31(1);
                *row[3] = input_limb_3_col3;
                let input_limb_4_col4 = input_tmp_f9fbc_0.2[0].get_m31(2);
                *row[4] = input_limb_4_col4;
                let input_limb_5_col5 = input_tmp_f9fbc_0.2[0].get_m31(3);
                *row[5] = input_limb_5_col5;
                let input_limb_6_col6 = input_tmp_f9fbc_0.2[0].get_m31(4);
                *row[6] = input_limb_6_col6;
                let input_limb_7_col7 = input_tmp_f9fbc_0.2[0].get_m31(5);
                *row[7] = input_limb_7_col7;
                let input_limb_8_col8 = input_tmp_f9fbc_0.2[0].get_m31(6);
                *row[8] = input_limb_8_col8;
                let input_limb_9_col9 = input_tmp_f9fbc_0.2[0].get_m31(7);
                *row[9] = input_limb_9_col9;
                let input_limb_10_col10 = input_tmp_f9fbc_0.2[0].get_m31(8);
                *row[10] = input_limb_10_col10;
                let input_limb_11_col11 = input_tmp_f9fbc_0.2[0].get_m31(9);
                *row[11] = input_limb_11_col11;
                let input_limb_12_col12 = input_tmp_f9fbc_0.2[1].get_m31(0);
                *row[12] = input_limb_12_col12;
                let input_limb_13_col13 = input_tmp_f9fbc_0.2[1].get_m31(1);
                *row[13] = input_limb_13_col13;
                let input_limb_14_col14 = input_tmp_f9fbc_0.2[1].get_m31(2);
                *row[14] = input_limb_14_col14;
                let input_limb_15_col15 = input_tmp_f9fbc_0.2[1].get_m31(3);
                *row[15] = input_limb_15_col15;
                let input_limb_16_col16 = input_tmp_f9fbc_0.2[1].get_m31(4);
                *row[16] = input_limb_16_col16;
                let input_limb_17_col17 = input_tmp_f9fbc_0.2[1].get_m31(5);
                *row[17] = input_limb_17_col17;
                let input_limb_18_col18 = input_tmp_f9fbc_0.2[1].get_m31(6);
                *row[18] = input_limb_18_col18;
                let input_limb_19_col19 = input_tmp_f9fbc_0.2[1].get_m31(7);
                *row[19] = input_limb_19_col19;
                let input_limb_20_col20 = input_tmp_f9fbc_0.2[1].get_m31(8);
                *row[20] = input_limb_20_col20;
                let input_limb_21_col21 = input_tmp_f9fbc_0.2[1].get_m31(9);
                *row[21] = input_limb_21_col21;
                let input_limb_22_col22 = input_tmp_f9fbc_0.2[2].get_m31(0);
                *row[22] = input_limb_22_col22;
                let input_limb_23_col23 = input_tmp_f9fbc_0.2[2].get_m31(1);
                *row[23] = input_limb_23_col23;
                let input_limb_24_col24 = input_tmp_f9fbc_0.2[2].get_m31(2);
                *row[24] = input_limb_24_col24;
                let input_limb_25_col25 = input_tmp_f9fbc_0.2[2].get_m31(3);
                *row[25] = input_limb_25_col25;
                let input_limb_26_col26 = input_tmp_f9fbc_0.2[2].get_m31(4);
                *row[26] = input_limb_26_col26;
                let input_limb_27_col27 = input_tmp_f9fbc_0.2[2].get_m31(5);
                *row[27] = input_limb_27_col27;
                let input_limb_28_col28 = input_tmp_f9fbc_0.2[2].get_m31(6);
                *row[28] = input_limb_28_col28;
                let input_limb_29_col29 = input_tmp_f9fbc_0.2[2].get_m31(7);
                *row[29] = input_limb_29_col29;
                let input_limb_30_col30 = input_tmp_f9fbc_0.2[2].get_m31(8);
                *row[30] = input_limb_30_col30;
                let input_limb_31_col31 = input_tmp_f9fbc_0.2[2].get_m31(9);
                *row[31] = input_limb_31_col31;
                let cube_252_inputs_0 = input_tmp_f9fbc_0.2[0].unpack();
                let cube_252_output_tmp_f9fbc_1 =
                    cube_252_state.deduce_output(input_tmp_f9fbc_0.2[0]);
                let cube_252_output_limb_0_col32 = cube_252_output_tmp_f9fbc_1.get_m31(0);
                *row[32] = cube_252_output_limb_0_col32;
                let cube_252_output_limb_1_col33 = cube_252_output_tmp_f9fbc_1.get_m31(1);
                *row[33] = cube_252_output_limb_1_col33;
                let cube_252_output_limb_2_col34 = cube_252_output_tmp_f9fbc_1.get_m31(2);
                *row[34] = cube_252_output_limb_2_col34;
                let cube_252_output_limb_3_col35 = cube_252_output_tmp_f9fbc_1.get_m31(3);
                *row[35] = cube_252_output_limb_3_col35;
                let cube_252_output_limb_4_col36 = cube_252_output_tmp_f9fbc_1.get_m31(4);
                *row[36] = cube_252_output_limb_4_col36;
                let cube_252_output_limb_5_col37 = cube_252_output_tmp_f9fbc_1.get_m31(5);
                *row[37] = cube_252_output_limb_5_col37;
                let cube_252_output_limb_6_col38 = cube_252_output_tmp_f9fbc_1.get_m31(6);
                *row[38] = cube_252_output_limb_6_col38;
                let cube_252_output_limb_7_col39 = cube_252_output_tmp_f9fbc_1.get_m31(7);
                *row[39] = cube_252_output_limb_7_col39;
                let cube_252_output_limb_8_col40 = cube_252_output_tmp_f9fbc_1.get_m31(8);
                *row[40] = cube_252_output_limb_8_col40;
                let cube_252_output_limb_9_col41 = cube_252_output_tmp_f9fbc_1.get_m31(9);
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
                let cube_252_inputs_1 = input_tmp_f9fbc_0.2[1].unpack();
                let cube_252_output_tmp_f9fbc_2 =
                    cube_252_state.deduce_output(input_tmp_f9fbc_0.2[1]);
                let cube_252_output_limb_0_col42 = cube_252_output_tmp_f9fbc_2.get_m31(0);
                *row[42] = cube_252_output_limb_0_col42;
                let cube_252_output_limb_1_col43 = cube_252_output_tmp_f9fbc_2.get_m31(1);
                *row[43] = cube_252_output_limb_1_col43;
                let cube_252_output_limb_2_col44 = cube_252_output_tmp_f9fbc_2.get_m31(2);
                *row[44] = cube_252_output_limb_2_col44;
                let cube_252_output_limb_3_col45 = cube_252_output_tmp_f9fbc_2.get_m31(3);
                *row[45] = cube_252_output_limb_3_col45;
                let cube_252_output_limb_4_col46 = cube_252_output_tmp_f9fbc_2.get_m31(4);
                *row[46] = cube_252_output_limb_4_col46;
                let cube_252_output_limb_5_col47 = cube_252_output_tmp_f9fbc_2.get_m31(5);
                *row[47] = cube_252_output_limb_5_col47;
                let cube_252_output_limb_6_col48 = cube_252_output_tmp_f9fbc_2.get_m31(6);
                *row[48] = cube_252_output_limb_6_col48;
                let cube_252_output_limb_7_col49 = cube_252_output_tmp_f9fbc_2.get_m31(7);
                *row[49] = cube_252_output_limb_7_col49;
                let cube_252_output_limb_8_col50 = cube_252_output_tmp_f9fbc_2.get_m31(8);
                *row[50] = cube_252_output_limb_8_col50;
                let cube_252_output_limb_9_col51 = cube_252_output_tmp_f9fbc_2.get_m31(9);
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
                let cube_252_inputs_2 = input_tmp_f9fbc_0.2[2].unpack();
                let cube_252_output_tmp_f9fbc_3 =
                    cube_252_state.deduce_output(input_tmp_f9fbc_0.2[2]);
                let cube_252_output_limb_0_col52 = cube_252_output_tmp_f9fbc_3.get_m31(0);
                *row[52] = cube_252_output_limb_0_col52;
                let cube_252_output_limb_1_col53 = cube_252_output_tmp_f9fbc_3.get_m31(1);
                *row[53] = cube_252_output_limb_1_col53;
                let cube_252_output_limb_2_col54 = cube_252_output_tmp_f9fbc_3.get_m31(2);
                *row[54] = cube_252_output_limb_2_col54;
                let cube_252_output_limb_3_col55 = cube_252_output_tmp_f9fbc_3.get_m31(3);
                *row[55] = cube_252_output_limb_3_col55;
                let cube_252_output_limb_4_col56 = cube_252_output_tmp_f9fbc_3.get_m31(4);
                *row[56] = cube_252_output_limb_4_col56;
                let cube_252_output_limb_5_col57 = cube_252_output_tmp_f9fbc_3.get_m31(5);
                *row[57] = cube_252_output_limb_5_col57;
                let cube_252_output_limb_6_col58 = cube_252_output_tmp_f9fbc_3.get_m31(6);
                *row[58] = cube_252_output_limb_6_col58;
                let cube_252_output_limb_7_col59 = cube_252_output_tmp_f9fbc_3.get_m31(7);
                *row[59] = cube_252_output_limb_7_col59;
                let cube_252_output_limb_8_col60 = cube_252_output_tmp_f9fbc_3.get_m31(8);
                *row[60] = cube_252_output_limb_8_col60;
                let cube_252_output_limb_9_col61 = cube_252_output_tmp_f9fbc_3.get_m31(9);
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
                let poseidon_round_keys_inputs_0 = [input_limb_1_col1].unpack();
                let poseidon_round_keys_output_tmp_f9fbc_4 =
                    poseidon_round_keys_state.deduce_output([input_limb_1_col1]);
                let poseidon_round_keys_output_limb_0_col62 =
                    poseidon_round_keys_output_tmp_f9fbc_4[0].get_m31(0);
                *row[62] = poseidon_round_keys_output_limb_0_col62;
                let poseidon_round_keys_output_limb_1_col63 =
                    poseidon_round_keys_output_tmp_f9fbc_4[0].get_m31(1);
                *row[63] = poseidon_round_keys_output_limb_1_col63;
                let poseidon_round_keys_output_limb_2_col64 =
                    poseidon_round_keys_output_tmp_f9fbc_4[0].get_m31(2);
                *row[64] = poseidon_round_keys_output_limb_2_col64;
                let poseidon_round_keys_output_limb_3_col65 =
                    poseidon_round_keys_output_tmp_f9fbc_4[0].get_m31(3);
                *row[65] = poseidon_round_keys_output_limb_3_col65;
                let poseidon_round_keys_output_limb_4_col66 =
                    poseidon_round_keys_output_tmp_f9fbc_4[0].get_m31(4);
                *row[66] = poseidon_round_keys_output_limb_4_col66;
                let poseidon_round_keys_output_limb_5_col67 =
                    poseidon_round_keys_output_tmp_f9fbc_4[0].get_m31(5);
                *row[67] = poseidon_round_keys_output_limb_5_col67;
                let poseidon_round_keys_output_limb_6_col68 =
                    poseidon_round_keys_output_tmp_f9fbc_4[0].get_m31(6);
                *row[68] = poseidon_round_keys_output_limb_6_col68;
                let poseidon_round_keys_output_limb_7_col69 =
                    poseidon_round_keys_output_tmp_f9fbc_4[0].get_m31(7);
                *row[69] = poseidon_round_keys_output_limb_7_col69;
                let poseidon_round_keys_output_limb_8_col70 =
                    poseidon_round_keys_output_tmp_f9fbc_4[0].get_m31(8);
                *row[70] = poseidon_round_keys_output_limb_8_col70;
                let poseidon_round_keys_output_limb_9_col71 =
                    poseidon_round_keys_output_tmp_f9fbc_4[0].get_m31(9);
                *row[71] = poseidon_round_keys_output_limb_9_col71;
                let poseidon_round_keys_output_limb_10_col72 =
                    poseidon_round_keys_output_tmp_f9fbc_4[1].get_m31(0);
                *row[72] = poseidon_round_keys_output_limb_10_col72;
                let poseidon_round_keys_output_limb_11_col73 =
                    poseidon_round_keys_output_tmp_f9fbc_4[1].get_m31(1);
                *row[73] = poseidon_round_keys_output_limb_11_col73;
                let poseidon_round_keys_output_limb_12_col74 =
                    poseidon_round_keys_output_tmp_f9fbc_4[1].get_m31(2);
                *row[74] = poseidon_round_keys_output_limb_12_col74;
                let poseidon_round_keys_output_limb_13_col75 =
                    poseidon_round_keys_output_tmp_f9fbc_4[1].get_m31(3);
                *row[75] = poseidon_round_keys_output_limb_13_col75;
                let poseidon_round_keys_output_limb_14_col76 =
                    poseidon_round_keys_output_tmp_f9fbc_4[1].get_m31(4);
                *row[76] = poseidon_round_keys_output_limb_14_col76;
                let poseidon_round_keys_output_limb_15_col77 =
                    poseidon_round_keys_output_tmp_f9fbc_4[1].get_m31(5);
                *row[77] = poseidon_round_keys_output_limb_15_col77;
                let poseidon_round_keys_output_limb_16_col78 =
                    poseidon_round_keys_output_tmp_f9fbc_4[1].get_m31(6);
                *row[78] = poseidon_round_keys_output_limb_16_col78;
                let poseidon_round_keys_output_limb_17_col79 =
                    poseidon_round_keys_output_tmp_f9fbc_4[1].get_m31(7);
                *row[79] = poseidon_round_keys_output_limb_17_col79;
                let poseidon_round_keys_output_limb_18_col80 =
                    poseidon_round_keys_output_tmp_f9fbc_4[1].get_m31(8);
                *row[80] = poseidon_round_keys_output_limb_18_col80;
                let poseidon_round_keys_output_limb_19_col81 =
                    poseidon_round_keys_output_tmp_f9fbc_4[1].get_m31(9);
                *row[81] = poseidon_round_keys_output_limb_19_col81;
                let poseidon_round_keys_output_limb_20_col82 =
                    poseidon_round_keys_output_tmp_f9fbc_4[2].get_m31(0);
                *row[82] = poseidon_round_keys_output_limb_20_col82;
                let poseidon_round_keys_output_limb_21_col83 =
                    poseidon_round_keys_output_tmp_f9fbc_4[2].get_m31(1);
                *row[83] = poseidon_round_keys_output_limb_21_col83;
                let poseidon_round_keys_output_limb_22_col84 =
                    poseidon_round_keys_output_tmp_f9fbc_4[2].get_m31(2);
                *row[84] = poseidon_round_keys_output_limb_22_col84;
                let poseidon_round_keys_output_limb_23_col85 =
                    poseidon_round_keys_output_tmp_f9fbc_4[2].get_m31(3);
                *row[85] = poseidon_round_keys_output_limb_23_col85;
                let poseidon_round_keys_output_limb_24_col86 =
                    poseidon_round_keys_output_tmp_f9fbc_4[2].get_m31(4);
                *row[86] = poseidon_round_keys_output_limb_24_col86;
                let poseidon_round_keys_output_limb_25_col87 =
                    poseidon_round_keys_output_tmp_f9fbc_4[2].get_m31(5);
                *row[87] = poseidon_round_keys_output_limb_25_col87;
                let poseidon_round_keys_output_limb_26_col88 =
                    poseidon_round_keys_output_tmp_f9fbc_4[2].get_m31(6);
                *row[88] = poseidon_round_keys_output_limb_26_col88;
                let poseidon_round_keys_output_limb_27_col89 =
                    poseidon_round_keys_output_tmp_f9fbc_4[2].get_m31(7);
                *row[89] = poseidon_round_keys_output_limb_27_col89;
                let poseidon_round_keys_output_limb_28_col90 =
                    poseidon_round_keys_output_tmp_f9fbc_4[2].get_m31(8);
                *row[90] = poseidon_round_keys_output_limb_28_col90;
                let poseidon_round_keys_output_limb_29_col91 =
                    poseidon_round_keys_output_tmp_f9fbc_4[2].get_m31(9);
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

                let combination_result_tmp_f9fbc_5 = PackedFelt252Width27::from_packed_felt252(
                    (((((Felt252_0_0_0_0)
                        + ((Felt252_3_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_f9fbc_1,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_f9fbc_2,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_f9fbc_3,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_round_keys_output_tmp_f9fbc_4[0],
                            )))),
                );
                let combination_limb_0_col92 = combination_result_tmp_f9fbc_5.get_m31(0);
                *row[92] = combination_limb_0_col92;
                let combination_limb_1_col93 = combination_result_tmp_f9fbc_5.get_m31(1);
                *row[93] = combination_limb_1_col93;
                let combination_limb_2_col94 = combination_result_tmp_f9fbc_5.get_m31(2);
                *row[94] = combination_limb_2_col94;
                let combination_limb_3_col95 = combination_result_tmp_f9fbc_5.get_m31(3);
                *row[95] = combination_limb_3_col95;
                let combination_limb_4_col96 = combination_result_tmp_f9fbc_5.get_m31(4);
                *row[96] = combination_limb_4_col96;
                let combination_limb_5_col97 = combination_result_tmp_f9fbc_5.get_m31(5);
                *row[97] = combination_limb_5_col97;
                let combination_limb_6_col98 = combination_result_tmp_f9fbc_5.get_m31(6);
                *row[98] = combination_limb_6_col98;
                let combination_limb_7_col99 = combination_result_tmp_f9fbc_5.get_m31(7);
                *row[99] = combination_limb_7_col99;
                let combination_limb_8_col100 = combination_result_tmp_f9fbc_5.get_m31(8);
                *row[100] = combination_limb_8_col100;
                let combination_limb_9_col101 = combination_result_tmp_f9fbc_5.get_m31(9);
                *row[101] = combination_limb_9_col101;
                let biased_limb_accumulator_u32_tmp_f9fbc_6 = PackedUInt32::from_m31(
                    (((((((M31_0) - (combination_limb_0_col92))
                        + ((M31_3) * (cube_252_output_limb_0_col32)))
                        + ((M31_1) * (cube_252_output_limb_0_col42)))
                        + ((M31_1) * (cube_252_output_limb_0_col52)))
                        + ((M31_1) * (poseidon_round_keys_output_limb_0_col62)))
                        + (M31_134217729)),
                );
                let p_coef_col102 =
                    ((biased_limb_accumulator_u32_tmp_f9fbc_6.low().as_m31()) - (M31_1));
                *row[102] = p_coef_col102;
                let carry_0_tmp_f9fbc_7 = ((((((((M31_0) - (combination_limb_0_col92))
                    + ((M31_3) * (cube_252_output_limb_0_col32)))
                    + ((M31_1) * (cube_252_output_limb_0_col42)))
                    + ((M31_1) * (cube_252_output_limb_0_col52)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_0_col62)))
                    - ((p_coef_col102) * (M31_1)))
                    * (M31_16));
                let carry_1_tmp_f9fbc_8 = ((((((((carry_0_tmp_f9fbc_7)
                    - (combination_limb_1_col93))
                    + ((M31_3) * (cube_252_output_limb_1_col33)))
                    + ((M31_1) * (cube_252_output_limb_1_col43)))
                    + ((M31_1) * (cube_252_output_limb_1_col53)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_1_col63)))
                    - ((p_coef_col102) * (M31_0)))
                    * (M31_16));
                let carry_2_tmp_f9fbc_9 = ((((((((carry_1_tmp_f9fbc_8)
                    - (combination_limb_2_col94))
                    + ((M31_3) * (cube_252_output_limb_2_col34)))
                    + ((M31_1) * (cube_252_output_limb_2_col44)))
                    + ((M31_1) * (cube_252_output_limb_2_col54)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_2_col64)))
                    - ((p_coef_col102) * (M31_0)))
                    * (M31_16));
                let carry_3_tmp_f9fbc_10 = ((((((((carry_2_tmp_f9fbc_9)
                    - (combination_limb_3_col95))
                    + ((M31_3) * (cube_252_output_limb_3_col35)))
                    + ((M31_1) * (cube_252_output_limb_3_col45)))
                    + ((M31_1) * (cube_252_output_limb_3_col55)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_3_col65)))
                    - ((p_coef_col102) * (M31_0)))
                    * (M31_16));
                let carry_4_tmp_f9fbc_11 = ((((((((carry_3_tmp_f9fbc_10)
                    - (combination_limb_4_col96))
                    + ((M31_3) * (cube_252_output_limb_4_col36)))
                    + ((M31_1) * (cube_252_output_limb_4_col46)))
                    + ((M31_1) * (cube_252_output_limb_4_col56)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_4_col66)))
                    - ((p_coef_col102) * (M31_0)))
                    * (M31_16));
                let carry_5_tmp_f9fbc_12 = ((((((((carry_4_tmp_f9fbc_11)
                    - (combination_limb_5_col97))
                    + ((M31_3) * (cube_252_output_limb_5_col37)))
                    + ((M31_1) * (cube_252_output_limb_5_col47)))
                    + ((M31_1) * (cube_252_output_limb_5_col57)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_5_col67)))
                    - ((p_coef_col102) * (M31_0)))
                    * (M31_16));
                let carry_6_tmp_f9fbc_13 = ((((((((carry_5_tmp_f9fbc_12)
                    - (combination_limb_6_col98))
                    + ((M31_3) * (cube_252_output_limb_6_col38)))
                    + ((M31_1) * (cube_252_output_limb_6_col48)))
                    + ((M31_1) * (cube_252_output_limb_6_col58)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_6_col68)))
                    - ((p_coef_col102) * (M31_0)))
                    * (M31_16));
                let carry_7_tmp_f9fbc_14 = ((((((((carry_6_tmp_f9fbc_13)
                    - (combination_limb_7_col99))
                    + ((M31_3) * (cube_252_output_limb_7_col39)))
                    + ((M31_1) * (cube_252_output_limb_7_col49)))
                    + ((M31_1) * (cube_252_output_limb_7_col59)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_7_col69)))
                    - ((p_coef_col102) * (M31_136)))
                    * (M31_16));
                let carry_8_tmp_f9fbc_15 = ((((((((carry_7_tmp_f9fbc_14)
                    - (combination_limb_8_col100))
                    + ((M31_3) * (cube_252_output_limb_8_col40)))
                    + ((M31_1) * (cube_252_output_limb_8_col50)))
                    + ((M31_1) * (cube_252_output_limb_8_col60)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_8_col70)))
                    - ((p_coef_col102) * (M31_0)))
                    * (M31_16));
                let range_check_3_3_3_3_3_inputs_0 = [
                    ((p_coef_col102) + (M31_1)),
                    ((carry_0_tmp_f9fbc_7) + (M31_1)),
                    ((carry_1_tmp_f9fbc_8) + (M31_1)),
                    ((carry_2_tmp_f9fbc_9) + (M31_1)),
                    ((carry_3_tmp_f9fbc_10) + (M31_1)),
                ]
                .unpack();
                *lookup_data.range_check_3_3_3_3_3_0 = [
                    ((p_coef_col102) + (M31_1)),
                    ((carry_0_tmp_f9fbc_7) + (M31_1)),
                    ((carry_1_tmp_f9fbc_8) + (M31_1)),
                    ((carry_2_tmp_f9fbc_9) + (M31_1)),
                    ((carry_3_tmp_f9fbc_10) + (M31_1)),
                ];
                let range_check_3_3_3_3_3_inputs_1 = [
                    ((carry_4_tmp_f9fbc_11) + (M31_1)),
                    ((carry_5_tmp_f9fbc_12) + (M31_1)),
                    ((carry_6_tmp_f9fbc_13) + (M31_1)),
                    ((carry_7_tmp_f9fbc_14) + (M31_1)),
                    ((carry_8_tmp_f9fbc_15) + (M31_1)),
                ]
                .unpack();
                *lookup_data.range_check_3_3_3_3_3_1 = [
                    ((carry_4_tmp_f9fbc_11) + (M31_1)),
                    ((carry_5_tmp_f9fbc_12) + (M31_1)),
                    ((carry_6_tmp_f9fbc_13) + (M31_1)),
                    ((carry_7_tmp_f9fbc_14) + (M31_1)),
                    ((carry_8_tmp_f9fbc_15) + (M31_1)),
                ];

                // Linear Combination N 4 Coefs 1 M 1 1 1.

                let combination_result_tmp_f9fbc_16 = PackedFelt252Width27::from_packed_felt252(
                    (((((Felt252_0_0_0_0)
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_f9fbc_1,
                            ))))
                        - ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_f9fbc_2,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_f9fbc_3,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_round_keys_output_tmp_f9fbc_4[1],
                            )))),
                );
                let combination_limb_0_col103 = combination_result_tmp_f9fbc_16.get_m31(0);
                *row[103] = combination_limb_0_col103;
                let combination_limb_1_col104 = combination_result_tmp_f9fbc_16.get_m31(1);
                *row[104] = combination_limb_1_col104;
                let combination_limb_2_col105 = combination_result_tmp_f9fbc_16.get_m31(2);
                *row[105] = combination_limb_2_col105;
                let combination_limb_3_col106 = combination_result_tmp_f9fbc_16.get_m31(3);
                *row[106] = combination_limb_3_col106;
                let combination_limb_4_col107 = combination_result_tmp_f9fbc_16.get_m31(4);
                *row[107] = combination_limb_4_col107;
                let combination_limb_5_col108 = combination_result_tmp_f9fbc_16.get_m31(5);
                *row[108] = combination_limb_5_col108;
                let combination_limb_6_col109 = combination_result_tmp_f9fbc_16.get_m31(6);
                *row[109] = combination_limb_6_col109;
                let combination_limb_7_col110 = combination_result_tmp_f9fbc_16.get_m31(7);
                *row[110] = combination_limb_7_col110;
                let combination_limb_8_col111 = combination_result_tmp_f9fbc_16.get_m31(8);
                *row[111] = combination_limb_8_col111;
                let combination_limb_9_col112 = combination_result_tmp_f9fbc_16.get_m31(9);
                *row[112] = combination_limb_9_col112;
                let biased_limb_accumulator_u32_tmp_f9fbc_17 = PackedUInt32::from_m31(
                    (((((((M31_0) - (combination_limb_0_col103))
                        + ((M31_1) * (cube_252_output_limb_0_col32)))
                        - ((M31_1) * (cube_252_output_limb_0_col42)))
                        + ((M31_1) * (cube_252_output_limb_0_col52)))
                        + ((M31_1) * (poseidon_round_keys_output_limb_10_col72)))
                        + (M31_268435458)),
                );
                let p_coef_col113 =
                    ((biased_limb_accumulator_u32_tmp_f9fbc_17.low().as_m31()) - (M31_2));
                *row[113] = p_coef_col113;
                let carry_0_tmp_f9fbc_18 = ((((((((M31_0) - (combination_limb_0_col103))
                    + ((M31_1) * (cube_252_output_limb_0_col32)))
                    - ((M31_1) * (cube_252_output_limb_0_col42)))
                    + ((M31_1) * (cube_252_output_limb_0_col52)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_10_col72)))
                    - ((p_coef_col113) * (M31_1)))
                    * (M31_16));
                let carry_1_tmp_f9fbc_19 = ((((((((carry_0_tmp_f9fbc_18)
                    - (combination_limb_1_col104))
                    + ((M31_1) * (cube_252_output_limb_1_col33)))
                    - ((M31_1) * (cube_252_output_limb_1_col43)))
                    + ((M31_1) * (cube_252_output_limb_1_col53)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_11_col73)))
                    - ((p_coef_col113) * (M31_0)))
                    * (M31_16));
                let carry_2_tmp_f9fbc_20 = ((((((((carry_1_tmp_f9fbc_19)
                    - (combination_limb_2_col105))
                    + ((M31_1) * (cube_252_output_limb_2_col34)))
                    - ((M31_1) * (cube_252_output_limb_2_col44)))
                    + ((M31_1) * (cube_252_output_limb_2_col54)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_12_col74)))
                    - ((p_coef_col113) * (M31_0)))
                    * (M31_16));
                let carry_3_tmp_f9fbc_21 = ((((((((carry_2_tmp_f9fbc_20)
                    - (combination_limb_3_col106))
                    + ((M31_1) * (cube_252_output_limb_3_col35)))
                    - ((M31_1) * (cube_252_output_limb_3_col45)))
                    + ((M31_1) * (cube_252_output_limb_3_col55)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_13_col75)))
                    - ((p_coef_col113) * (M31_0)))
                    * (M31_16));
                let carry_4_tmp_f9fbc_22 = ((((((((carry_3_tmp_f9fbc_21)
                    - (combination_limb_4_col107))
                    + ((M31_1) * (cube_252_output_limb_4_col36)))
                    - ((M31_1) * (cube_252_output_limb_4_col46)))
                    + ((M31_1) * (cube_252_output_limb_4_col56)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_14_col76)))
                    - ((p_coef_col113) * (M31_0)))
                    * (M31_16));
                let carry_5_tmp_f9fbc_23 = ((((((((carry_4_tmp_f9fbc_22)
                    - (combination_limb_5_col108))
                    + ((M31_1) * (cube_252_output_limb_5_col37)))
                    - ((M31_1) * (cube_252_output_limb_5_col47)))
                    + ((M31_1) * (cube_252_output_limb_5_col57)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_15_col77)))
                    - ((p_coef_col113) * (M31_0)))
                    * (M31_16));
                let carry_6_tmp_f9fbc_24 = ((((((((carry_5_tmp_f9fbc_23)
                    - (combination_limb_6_col109))
                    + ((M31_1) * (cube_252_output_limb_6_col38)))
                    - ((M31_1) * (cube_252_output_limb_6_col48)))
                    + ((M31_1) * (cube_252_output_limb_6_col58)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_16_col78)))
                    - ((p_coef_col113) * (M31_0)))
                    * (M31_16));
                let carry_7_tmp_f9fbc_25 = ((((((((carry_6_tmp_f9fbc_24)
                    - (combination_limb_7_col110))
                    + ((M31_1) * (cube_252_output_limb_7_col39)))
                    - ((M31_1) * (cube_252_output_limb_7_col49)))
                    + ((M31_1) * (cube_252_output_limb_7_col59)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_17_col79)))
                    - ((p_coef_col113) * (M31_136)))
                    * (M31_16));
                let carry_8_tmp_f9fbc_26 = ((((((((carry_7_tmp_f9fbc_25)
                    - (combination_limb_8_col111))
                    + ((M31_1) * (cube_252_output_limb_8_col40)))
                    - ((M31_1) * (cube_252_output_limb_8_col50)))
                    + ((M31_1) * (cube_252_output_limb_8_col60)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_18_col80)))
                    - ((p_coef_col113) * (M31_0)))
                    * (M31_16));
                let range_check_3_3_3_3_3_inputs_2 = [
                    ((p_coef_col113) + (M31_2)),
                    ((carry_0_tmp_f9fbc_18) + (M31_2)),
                    ((carry_1_tmp_f9fbc_19) + (M31_2)),
                    ((carry_2_tmp_f9fbc_20) + (M31_2)),
                    ((carry_3_tmp_f9fbc_21) + (M31_2)),
                ]
                .unpack();
                *lookup_data.range_check_3_3_3_3_3_2 = [
                    ((p_coef_col113) + (M31_2)),
                    ((carry_0_tmp_f9fbc_18) + (M31_2)),
                    ((carry_1_tmp_f9fbc_19) + (M31_2)),
                    ((carry_2_tmp_f9fbc_20) + (M31_2)),
                    ((carry_3_tmp_f9fbc_21) + (M31_2)),
                ];
                let range_check_3_3_3_3_3_inputs_3 = [
                    ((carry_4_tmp_f9fbc_22) + (M31_2)),
                    ((carry_5_tmp_f9fbc_23) + (M31_2)),
                    ((carry_6_tmp_f9fbc_24) + (M31_2)),
                    ((carry_7_tmp_f9fbc_25) + (M31_2)),
                    ((carry_8_tmp_f9fbc_26) + (M31_2)),
                ]
                .unpack();
                *lookup_data.range_check_3_3_3_3_3_3 = [
                    ((carry_4_tmp_f9fbc_22) + (M31_2)),
                    ((carry_5_tmp_f9fbc_23) + (M31_2)),
                    ((carry_6_tmp_f9fbc_24) + (M31_2)),
                    ((carry_7_tmp_f9fbc_25) + (M31_2)),
                    ((carry_8_tmp_f9fbc_26) + (M31_2)),
                ];

                // Linear Combination N 4 Coefs 1 1 M 2 1.

                let combination_result_tmp_f9fbc_27 = PackedFelt252Width27::from_packed_felt252(
                    (((((Felt252_0_0_0_0)
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_f9fbc_1,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_f9fbc_2,
                            ))))
                        - ((Felt252_2_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                cube_252_output_tmp_f9fbc_3,
                            ))))
                        + ((Felt252_1_0_0_0)
                            * (PackedFelt252::from_packed_felt252width27(
                                poseidon_round_keys_output_tmp_f9fbc_4[2],
                            )))),
                );
                let combination_limb_0_col114 = combination_result_tmp_f9fbc_27.get_m31(0);
                *row[114] = combination_limb_0_col114;
                let combination_limb_1_col115 = combination_result_tmp_f9fbc_27.get_m31(1);
                *row[115] = combination_limb_1_col115;
                let combination_limb_2_col116 = combination_result_tmp_f9fbc_27.get_m31(2);
                *row[116] = combination_limb_2_col116;
                let combination_limb_3_col117 = combination_result_tmp_f9fbc_27.get_m31(3);
                *row[117] = combination_limb_3_col117;
                let combination_limb_4_col118 = combination_result_tmp_f9fbc_27.get_m31(4);
                *row[118] = combination_limb_4_col118;
                let combination_limb_5_col119 = combination_result_tmp_f9fbc_27.get_m31(5);
                *row[119] = combination_limb_5_col119;
                let combination_limb_6_col120 = combination_result_tmp_f9fbc_27.get_m31(6);
                *row[120] = combination_limb_6_col120;
                let combination_limb_7_col121 = combination_result_tmp_f9fbc_27.get_m31(7);
                *row[121] = combination_limb_7_col121;
                let combination_limb_8_col122 = combination_result_tmp_f9fbc_27.get_m31(8);
                *row[122] = combination_limb_8_col122;
                let combination_limb_9_col123 = combination_result_tmp_f9fbc_27.get_m31(9);
                *row[123] = combination_limb_9_col123;
                let biased_limb_accumulator_u32_tmp_f9fbc_28 = PackedUInt32::from_m31(
                    (((((((M31_0) - (combination_limb_0_col114))
                        + ((M31_1) * (cube_252_output_limb_0_col32)))
                        + ((M31_1) * (cube_252_output_limb_0_col42)))
                        - ((M31_2) * (cube_252_output_limb_0_col52)))
                        + ((M31_1) * (poseidon_round_keys_output_limb_20_col82)))
                        + (M31_402653187)),
                );
                let p_coef_col124 =
                    ((biased_limb_accumulator_u32_tmp_f9fbc_28.low().as_m31()) - (M31_3));
                *row[124] = p_coef_col124;
                let carry_0_tmp_f9fbc_29 = ((((((((M31_0) - (combination_limb_0_col114))
                    + ((M31_1) * (cube_252_output_limb_0_col32)))
                    + ((M31_1) * (cube_252_output_limb_0_col42)))
                    - ((M31_2) * (cube_252_output_limb_0_col52)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_20_col82)))
                    - ((p_coef_col124) * (M31_1)))
                    * (M31_16));
                let carry_1_tmp_f9fbc_30 = ((((((((carry_0_tmp_f9fbc_29)
                    - (combination_limb_1_col115))
                    + ((M31_1) * (cube_252_output_limb_1_col33)))
                    + ((M31_1) * (cube_252_output_limb_1_col43)))
                    - ((M31_2) * (cube_252_output_limb_1_col53)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_21_col83)))
                    - ((p_coef_col124) * (M31_0)))
                    * (M31_16));
                let carry_2_tmp_f9fbc_31 = ((((((((carry_1_tmp_f9fbc_30)
                    - (combination_limb_2_col116))
                    + ((M31_1) * (cube_252_output_limb_2_col34)))
                    + ((M31_1) * (cube_252_output_limb_2_col44)))
                    - ((M31_2) * (cube_252_output_limb_2_col54)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_22_col84)))
                    - ((p_coef_col124) * (M31_0)))
                    * (M31_16));
                let carry_3_tmp_f9fbc_32 = ((((((((carry_2_tmp_f9fbc_31)
                    - (combination_limb_3_col117))
                    + ((M31_1) * (cube_252_output_limb_3_col35)))
                    + ((M31_1) * (cube_252_output_limb_3_col45)))
                    - ((M31_2) * (cube_252_output_limb_3_col55)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_23_col85)))
                    - ((p_coef_col124) * (M31_0)))
                    * (M31_16));
                let carry_4_tmp_f9fbc_33 = ((((((((carry_3_tmp_f9fbc_32)
                    - (combination_limb_4_col118))
                    + ((M31_1) * (cube_252_output_limb_4_col36)))
                    + ((M31_1) * (cube_252_output_limb_4_col46)))
                    - ((M31_2) * (cube_252_output_limb_4_col56)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_24_col86)))
                    - ((p_coef_col124) * (M31_0)))
                    * (M31_16));
                let carry_5_tmp_f9fbc_34 = ((((((((carry_4_tmp_f9fbc_33)
                    - (combination_limb_5_col119))
                    + ((M31_1) * (cube_252_output_limb_5_col37)))
                    + ((M31_1) * (cube_252_output_limb_5_col47)))
                    - ((M31_2) * (cube_252_output_limb_5_col57)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_25_col87)))
                    - ((p_coef_col124) * (M31_0)))
                    * (M31_16));
                let carry_6_tmp_f9fbc_35 = ((((((((carry_5_tmp_f9fbc_34)
                    - (combination_limb_6_col120))
                    + ((M31_1) * (cube_252_output_limb_6_col38)))
                    + ((M31_1) * (cube_252_output_limb_6_col48)))
                    - ((M31_2) * (cube_252_output_limb_6_col58)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_26_col88)))
                    - ((p_coef_col124) * (M31_0)))
                    * (M31_16));
                let carry_7_tmp_f9fbc_36 = ((((((((carry_6_tmp_f9fbc_35)
                    - (combination_limb_7_col121))
                    + ((M31_1) * (cube_252_output_limb_7_col39)))
                    + ((M31_1) * (cube_252_output_limb_7_col49)))
                    - ((M31_2) * (cube_252_output_limb_7_col59)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_27_col89)))
                    - ((p_coef_col124) * (M31_136)))
                    * (M31_16));
                let carry_8_tmp_f9fbc_37 = ((((((((carry_7_tmp_f9fbc_36)
                    - (combination_limb_8_col122))
                    + ((M31_1) * (cube_252_output_limb_8_col40)))
                    + ((M31_1) * (cube_252_output_limb_8_col50)))
                    - ((M31_2) * (cube_252_output_limb_8_col60)))
                    + ((M31_1) * (poseidon_round_keys_output_limb_28_col90)))
                    - ((p_coef_col124) * (M31_0)))
                    * (M31_16));
                let range_check_3_3_3_3_3_inputs_4 = [
                    ((p_coef_col124) + (M31_3)),
                    ((carry_0_tmp_f9fbc_29) + (M31_3)),
                    ((carry_1_tmp_f9fbc_30) + (M31_3)),
                    ((carry_2_tmp_f9fbc_31) + (M31_3)),
                    ((carry_3_tmp_f9fbc_32) + (M31_3)),
                ]
                .unpack();
                *lookup_data.range_check_3_3_3_3_3_4 = [
                    ((p_coef_col124) + (M31_3)),
                    ((carry_0_tmp_f9fbc_29) + (M31_3)),
                    ((carry_1_tmp_f9fbc_30) + (M31_3)),
                    ((carry_2_tmp_f9fbc_31) + (M31_3)),
                    ((carry_3_tmp_f9fbc_32) + (M31_3)),
                ];
                let range_check_3_3_3_3_3_inputs_5 = [
                    ((carry_4_tmp_f9fbc_33) + (M31_3)),
                    ((carry_5_tmp_f9fbc_34) + (M31_3)),
                    ((carry_6_tmp_f9fbc_35) + (M31_3)),
                    ((carry_7_tmp_f9fbc_36) + (M31_3)),
                    ((carry_8_tmp_f9fbc_37) + (M31_3)),
                ]
                .unpack();
                *lookup_data.range_check_3_3_3_3_3_5 = [
                    ((carry_4_tmp_f9fbc_33) + (M31_3)),
                    ((carry_5_tmp_f9fbc_34) + (M31_3)),
                    ((carry_6_tmp_f9fbc_35) + (M31_3)),
                    ((carry_7_tmp_f9fbc_36) + (M31_3)),
                    ((carry_8_tmp_f9fbc_37) + (M31_3)),
                ];

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
                *row[125] = padding_col.packed_at(row_index);

                // Add sub-components inputs.
                *cube_252_input = [cube_252_inputs_0, cube_252_inputs_1, cube_252_inputs_2];
                poseidon_round_keys_state.add_inputs(&poseidon_round_keys_inputs_0);
                range_check_3_3_3_3_3_state.add_inputs(&range_check_3_3_3_3_3_inputs_0);
                range_check_3_3_3_3_3_state.add_inputs(&range_check_3_3_3_3_3_inputs_1);
                range_check_3_3_3_3_3_state.add_inputs(&range_check_3_3_3_3_3_inputs_2);
                range_check_3_3_3_3_3_state.add_inputs(&range_check_3_3_3_3_3_inputs_3);
                range_check_3_3_3_3_3_state.add_inputs(&range_check_3_3_3_3_3_inputs_4);
                range_check_3_3_3_3_3_state.add_inputs(&range_check_3_3_3_3_3_inputs_5);
            },
        );

    cube_252_state.add_inputs(
        &cube_252_inputs_vec
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
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        cube_252: &relations::Cube252,
        poseidon_full_round_chain: &relations::PoseidonFullRoundChain,
        poseidon_round_keys: &relations::PoseidonRoundKeys,
        range_check_3_3_3_3_3: &relations::RangeCheck_3_3_3_3_3,
    ) -> InteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let log_size = std::cmp::max(self.n_rows.next_power_of_two().ilog2(), LOG_N_LANES);
        let padding_col = Enabler::new(self.n_rows);
        let mut logup_gen = LogupTraceGenerator::new(log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in
            zip(&self.lookup_data.cube_252_0, &self.lookup_data.cube_252_1).enumerate()
        {
            let denom0: PackedQM31 = cube_252.combine(values0);
            let denom1: PackedQM31 = cube_252.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.cube_252_2,
            &self.lookup_data.poseidon_round_keys_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = cube_252.combine(values0);
            let denom1: PackedQM31 = poseidon_round_keys.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_3_3_3_3_3_0,
            &self.lookup_data.range_check_3_3_3_3_3_1,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_3_3_3_3_3.combine(values0);
            let denom1: PackedQM31 = range_check_3_3_3_3_3.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_3_3_3_3_3_2,
            &self.lookup_data.range_check_3_3_3_3_3_3,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_3_3_3_3_3.combine(values0);
            let denom1: PackedQM31 = range_check_3_3_3_3_3.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_3_3_3_3_3_4,
            &self.lookup_data.range_check_3_3_3_3_3_5,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_3_3_3_3_3.combine(values0);
            let denom1: PackedQM31 = range_check_3_3_3_3_3.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.poseidon_full_round_chain_0,
            &self.lookup_data.poseidon_full_round_chain_1,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = poseidon_full_round_chain.combine(values0);
            let denom1: PackedQM31 = poseidon_full_round_chain.combine(values1);
            col_gen.write_frac(
                i,
                (denom1 - denom0) * padding_col.packed_at(i),
                denom0 * denom1,
            );
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
