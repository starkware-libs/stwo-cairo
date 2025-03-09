#![allow(unused_parens)]
#![allow(unused_imports)]
use std::mem::transmute;

use itertools::Itertools;
use stwo_cairo_adapter::memory::Memory;

use super::component::{Claim, InteractionClaim};
use crate::cairo_air::blake::deduce_output::{BlakeRound, BlakeRoundSigma};
use crate::components::prelude::proving::*;
use crate::components::{
    blake_g, blake_round_sigma, memory_address_to_id, memory_id_to_big, range_check_7_2_5,
};

pub type InputType = (M31, M31, ([UInt32; 16], M31));
pub type PackedInputType = (PackedM31, PackedM31, ([PackedUInt32; 16], PackedM31));
const N_TRACE_COLUMNS: usize = 211 + 1;

pub struct ClaimGenerator {
    inputs: Vec<InputType>,
    state: BlakeRound,
}
impl ClaimGenerator {
    pub fn new(memory: Memory) -> Self {
        let inputs = vec![];
        let state = BlakeRound::new(memory);
        Self { inputs, state }
    }

    pub fn write_trace<MC: MerkleChannel>(
        mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        blake_g_state: &mut blake_g::ClaimGenerator,
        blake_round_sigma_state: &blake_round_sigma::ClaimGenerator,
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
        range_check_7_2_5_state: &range_check_7_2_5::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let n_rows = self.inputs.len();
        let size = std::cmp::max(n_rows.next_power_of_two(), N_LANES);
        let log_size = size.ilog2();
        self.inputs.resize(size, *self.inputs.first().unwrap());
        let packed_inputs = pack_values(&self.inputs);

        let (trace, lookup_data) = write_trace_simd(
            n_rows,
            packed_inputs,
            blake_g_state,
            blake_round_sigma_state,
            memory_address_to_id_state,
            memory_id_to_big_state,
            range_check_7_2_5_state,
        );

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
        let inputs = inputs
            .iter()
            .cloned()
            .flat_map(Unpack::unpack)
            .collect_vec();
        self.add_inputs(&inputs);
    }

    pub fn add_inputs(&mut self, inputs: &[InputType]) {
        self.inputs.extend(inputs);
    }

    pub fn deduce_output(
        &self,
        input: (PackedM31, PackedM31, ([PackedUInt32; 16], PackedM31)),
    ) -> (PackedM31, PackedM31, ([PackedUInt32; 16], PackedM31)) {
        self.state.deduce_output(input.0, input.1, input.2)
    }
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct BlakeGInput {
    inputs: [Vec<blake_g::PackedInputType>; 8],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    n_rows: usize,
    inputs: Vec<PackedInputType>,
    blake_g_state: &mut blake_g::ClaimGenerator,
    blake_round_sigma_state: &blake_round_sigma::ClaimGenerator,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
    range_check_7_2_5_state: &range_check_7_2_5::ClaimGenerator,
) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData) {
    let log_n_packed_rows = inputs.len().ilog2();
    let log_size = log_n_packed_rows + LOG_N_LANES;
    let (mut trace, mut lookup_data, mut blake_g_inputs) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
            BlakeGInput::uninitialized(log_n_packed_rows),
        )
    };

    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_128 = PackedM31::broadcast(M31::from(128));
    let M31_2048 = PackedM31::broadcast(M31::from(2048));
    let M31_4 = PackedM31::broadcast(M31::from(4));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let UInt16_2 = PackedUInt16::broadcast(UInt16::from(2));
    let UInt16_7 = PackedUInt16::broadcast(UInt16::from(7));
    let UInt16_9 = PackedUInt16::broadcast(UInt16::from(9));
    let enabler = Enabler::new(n_rows);

    trace
        .par_iter_mut()
        .enumerate()
        .zip(inputs.into_par_iter())
        .zip(lookup_data.par_iter_mut())
        .zip(blake_g_inputs.par_iter_mut())
        .for_each(
            |((((row_index, mut row), blake_round_input), lookup_data), blake_g_input)| {
                let input_tmp_92ff8_0 = (
                    blake_round_input.0,
                    blake_round_input.1,
                    (
                        [
                            blake_round_input.2 .0[0],
                            blake_round_input.2 .0[1],
                            blake_round_input.2 .0[2],
                            blake_round_input.2 .0[3],
                            blake_round_input.2 .0[4],
                            blake_round_input.2 .0[5],
                            blake_round_input.2 .0[6],
                            blake_round_input.2 .0[7],
                            blake_round_input.2 .0[8],
                            blake_round_input.2 .0[9],
                            blake_round_input.2 .0[10],
                            blake_round_input.2 .0[11],
                            blake_round_input.2 .0[12],
                            blake_round_input.2 .0[13],
                            blake_round_input.2 .0[14],
                            blake_round_input.2 .0[15],
                        ],
                        blake_round_input.2 .1,
                    ),
                );
                let input_limb_0_col0 = input_tmp_92ff8_0.0;
                *row[0] = input_limb_0_col0;
                let input_limb_1_col1 = input_tmp_92ff8_0.1;
                *row[1] = input_limb_1_col1;
                let input_limb_2_col2 = input_tmp_92ff8_0.2 .0[0].low().as_m31();
                *row[2] = input_limb_2_col2;
                let input_limb_3_col3 = input_tmp_92ff8_0.2 .0[0].high().as_m31();
                *row[3] = input_limb_3_col3;
                let input_limb_4_col4 = input_tmp_92ff8_0.2 .0[1].low().as_m31();
                *row[4] = input_limb_4_col4;
                let input_limb_5_col5 = input_tmp_92ff8_0.2 .0[1].high().as_m31();
                *row[5] = input_limb_5_col5;
                let input_limb_6_col6 = input_tmp_92ff8_0.2 .0[2].low().as_m31();
                *row[6] = input_limb_6_col6;
                let input_limb_7_col7 = input_tmp_92ff8_0.2 .0[2].high().as_m31();
                *row[7] = input_limb_7_col7;
                let input_limb_8_col8 = input_tmp_92ff8_0.2 .0[3].low().as_m31();
                *row[8] = input_limb_8_col8;
                let input_limb_9_col9 = input_tmp_92ff8_0.2 .0[3].high().as_m31();
                *row[9] = input_limb_9_col9;
                let input_limb_10_col10 = input_tmp_92ff8_0.2 .0[4].low().as_m31();
                *row[10] = input_limb_10_col10;
                let input_limb_11_col11 = input_tmp_92ff8_0.2 .0[4].high().as_m31();
                *row[11] = input_limb_11_col11;
                let input_limb_12_col12 = input_tmp_92ff8_0.2 .0[5].low().as_m31();
                *row[12] = input_limb_12_col12;
                let input_limb_13_col13 = input_tmp_92ff8_0.2 .0[5].high().as_m31();
                *row[13] = input_limb_13_col13;
                let input_limb_14_col14 = input_tmp_92ff8_0.2 .0[6].low().as_m31();
                *row[14] = input_limb_14_col14;
                let input_limb_15_col15 = input_tmp_92ff8_0.2 .0[6].high().as_m31();
                *row[15] = input_limb_15_col15;
                let input_limb_16_col16 = input_tmp_92ff8_0.2 .0[7].low().as_m31();
                *row[16] = input_limb_16_col16;
                let input_limb_17_col17 = input_tmp_92ff8_0.2 .0[7].high().as_m31();
                *row[17] = input_limb_17_col17;
                let input_limb_18_col18 = input_tmp_92ff8_0.2 .0[8].low().as_m31();
                *row[18] = input_limb_18_col18;
                let input_limb_19_col19 = input_tmp_92ff8_0.2 .0[8].high().as_m31();
                *row[19] = input_limb_19_col19;
                let input_limb_20_col20 = input_tmp_92ff8_0.2 .0[9].low().as_m31();
                *row[20] = input_limb_20_col20;
                let input_limb_21_col21 = input_tmp_92ff8_0.2 .0[9].high().as_m31();
                *row[21] = input_limb_21_col21;
                let input_limb_22_col22 = input_tmp_92ff8_0.2 .0[10].low().as_m31();
                *row[22] = input_limb_22_col22;
                let input_limb_23_col23 = input_tmp_92ff8_0.2 .0[10].high().as_m31();
                *row[23] = input_limb_23_col23;
                let input_limb_24_col24 = input_tmp_92ff8_0.2 .0[11].low().as_m31();
                *row[24] = input_limb_24_col24;
                let input_limb_25_col25 = input_tmp_92ff8_0.2 .0[11].high().as_m31();
                *row[25] = input_limb_25_col25;
                let input_limb_26_col26 = input_tmp_92ff8_0.2 .0[12].low().as_m31();
                *row[26] = input_limb_26_col26;
                let input_limb_27_col27 = input_tmp_92ff8_0.2 .0[12].high().as_m31();
                *row[27] = input_limb_27_col27;
                let input_limb_28_col28 = input_tmp_92ff8_0.2 .0[13].low().as_m31();
                *row[28] = input_limb_28_col28;
                let input_limb_29_col29 = input_tmp_92ff8_0.2 .0[13].high().as_m31();
                *row[29] = input_limb_29_col29;
                let input_limb_30_col30 = input_tmp_92ff8_0.2 .0[14].low().as_m31();
                *row[30] = input_limb_30_col30;
                let input_limb_31_col31 = input_tmp_92ff8_0.2 .0[14].high().as_m31();
                *row[31] = input_limb_31_col31;
                let input_limb_32_col32 = input_tmp_92ff8_0.2 .0[15].low().as_m31();
                *row[32] = input_limb_32_col32;
                let input_limb_33_col33 = input_tmp_92ff8_0.2 .0[15].high().as_m31();
                *row[33] = input_limb_33_col33;
                let input_limb_34_col34 = input_tmp_92ff8_0.2 .1;
                *row[34] = input_limb_34_col34;
                let blake_round_sigma_inputs_0 = [input_limb_1_col1].unpack();
                let blake_round_sigma_output_tmp_92ff8_1 =
                    BlakeRoundSigma::deduce_output(input_limb_1_col1);
                let blake_round_sigma_output_limb_0_col35 = blake_round_sigma_output_tmp_92ff8_1[0];
                *row[35] = blake_round_sigma_output_limb_0_col35;
                let blake_round_sigma_output_limb_1_col36 = blake_round_sigma_output_tmp_92ff8_1[1];
                *row[36] = blake_round_sigma_output_limb_1_col36;
                let blake_round_sigma_output_limb_2_col37 = blake_round_sigma_output_tmp_92ff8_1[2];
                *row[37] = blake_round_sigma_output_limb_2_col37;
                let blake_round_sigma_output_limb_3_col38 = blake_round_sigma_output_tmp_92ff8_1[3];
                *row[38] = blake_round_sigma_output_limb_3_col38;
                let blake_round_sigma_output_limb_4_col39 = blake_round_sigma_output_tmp_92ff8_1[4];
                *row[39] = blake_round_sigma_output_limb_4_col39;
                let blake_round_sigma_output_limb_5_col40 = blake_round_sigma_output_tmp_92ff8_1[5];
                *row[40] = blake_round_sigma_output_limb_5_col40;
                let blake_round_sigma_output_limb_6_col41 = blake_round_sigma_output_tmp_92ff8_1[6];
                *row[41] = blake_round_sigma_output_limb_6_col41;
                let blake_round_sigma_output_limb_7_col42 = blake_round_sigma_output_tmp_92ff8_1[7];
                *row[42] = blake_round_sigma_output_limb_7_col42;
                let blake_round_sigma_output_limb_8_col43 = blake_round_sigma_output_tmp_92ff8_1[8];
                *row[43] = blake_round_sigma_output_limb_8_col43;
                let blake_round_sigma_output_limb_9_col44 = blake_round_sigma_output_tmp_92ff8_1[9];
                *row[44] = blake_round_sigma_output_limb_9_col44;
                let blake_round_sigma_output_limb_10_col45 =
                    blake_round_sigma_output_tmp_92ff8_1[10];
                *row[45] = blake_round_sigma_output_limb_10_col45;
                let blake_round_sigma_output_limb_11_col46 =
                    blake_round_sigma_output_tmp_92ff8_1[11];
                *row[46] = blake_round_sigma_output_limb_11_col46;
                let blake_round_sigma_output_limb_12_col47 =
                    blake_round_sigma_output_tmp_92ff8_1[12];
                *row[47] = blake_round_sigma_output_limb_12_col47;
                let blake_round_sigma_output_limb_13_col48 =
                    blake_round_sigma_output_tmp_92ff8_1[13];
                *row[48] = blake_round_sigma_output_limb_13_col48;
                let blake_round_sigma_output_limb_14_col49 =
                    blake_round_sigma_output_tmp_92ff8_1[14];
                *row[49] = blake_round_sigma_output_limb_14_col49;
                let blake_round_sigma_output_limb_15_col50 =
                    blake_round_sigma_output_tmp_92ff8_1[15];
                *row[50] = blake_round_sigma_output_limb_15_col50;
                *lookup_data.blake_round_sigma_0 = [
                    input_limb_1_col1,
                    blake_round_sigma_output_limb_0_col35,
                    blake_round_sigma_output_limb_1_col36,
                    blake_round_sigma_output_limb_2_col37,
                    blake_round_sigma_output_limb_3_col38,
                    blake_round_sigma_output_limb_4_col39,
                    blake_round_sigma_output_limb_5_col40,
                    blake_round_sigma_output_limb_6_col41,
                    blake_round_sigma_output_limb_7_col42,
                    blake_round_sigma_output_limb_8_col43,
                    blake_round_sigma_output_limb_9_col44,
                    blake_round_sigma_output_limb_10_col45,
                    blake_round_sigma_output_limb_11_col46,
                    blake_round_sigma_output_limb_12_col47,
                    blake_round_sigma_output_limb_13_col48,
                    blake_round_sigma_output_limb_14_col49,
                    blake_round_sigma_output_limb_15_col50,
                ];

                // Read Blake Word.

                let memory_address_to_id_value_tmp_92ff8_2 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_0_col35)),
                    );
                let memory_id_to_big_value_tmp_92ff8_3 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_92ff8_2);
                let tmp_92ff8_4 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_92ff8_3.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col51 = ((((memory_id_to_big_value_tmp_92ff8_3.get_m31(1))
                    - ((tmp_92ff8_4.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_92ff8_3.get_m31(0)));
                *row[51] = low_16_bits_col51;
                let high_16_bits_col52 = ((((memory_id_to_big_value_tmp_92ff8_3.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_92ff8_3.get_m31(2)) * (M31_4)))
                    + (tmp_92ff8_4.as_m31()));
                *row[52] = high_16_bits_col52;
                let expected_word_tmp_92ff8_5 =
                    PackedUInt32::from_limbs(low_16_bits_col51, high_16_bits_col52);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_92ff8_6 = ((expected_word_tmp_92ff8_5.low()) >> (UInt16_9));
                let low_7_ms_bits_col53 = low_7_ms_bits_tmp_92ff8_6.as_m31();
                *row[53] = low_7_ms_bits_col53;
                let high_14_ms_bits_tmp_92ff8_7 =
                    ((expected_word_tmp_92ff8_5.high()) >> (UInt16_2));
                let high_14_ms_bits_col54 = high_14_ms_bits_tmp_92ff8_7.as_m31();
                *row[54] = high_14_ms_bits_col54;
                let high_5_ms_bits_tmp_92ff8_8 = ((high_14_ms_bits_tmp_92ff8_7) >> (UInt16_9));
                let high_5_ms_bits_col55 = high_5_ms_bits_tmp_92ff8_8.as_m31();
                *row[55] = high_5_ms_bits_col55;
                let range_check_7_2_5_inputs_0 = [
                    low_7_ms_bits_col53,
                    ((high_16_bits_col52) - ((high_14_ms_bits_col54) * (M31_4))),
                    high_5_ms_bits_col55,
                ]
                .unpack();
                *lookup_data.range_check_7_2_5_0 = [
                    low_7_ms_bits_col53,
                    ((high_16_bits_col52) - ((high_14_ms_bits_col54) * (M31_4))),
                    high_5_ms_bits_col55,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_92ff8_9 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_0_col35)),
                    );
                let message_word_0_id_col56 = memory_address_to_id_value_tmp_92ff8_9;
                *row[56] = message_word_0_id_col56;
                let memory_address_to_id_inputs_0 =
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_0_col35)).unpack();
                *lookup_data.memory_address_to_id_0 = [
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_0_col35)),
                    message_word_0_id_col56,
                ];
                let memory_id_to_big_inputs_0 = message_word_0_id_col56.unpack();
                *lookup_data.memory_id_to_big_0 = [
                    message_word_0_id_col56,
                    ((low_16_bits_col51) - ((low_7_ms_bits_col53) * (M31_512))),
                    ((low_7_ms_bits_col53)
                        + (((high_16_bits_col52) - ((high_14_ms_bits_col54) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col54) - ((high_5_ms_bits_col55) * (M31_512))),
                    high_5_ms_bits_col55,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                ];

                // Read Blake Word.

                let memory_address_to_id_value_tmp_92ff8_10 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_1_col36)),
                    );
                let memory_id_to_big_value_tmp_92ff8_11 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_92ff8_10);
                let tmp_92ff8_12 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_92ff8_11.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col57 = ((((memory_id_to_big_value_tmp_92ff8_11.get_m31(1))
                    - ((tmp_92ff8_12.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_92ff8_11.get_m31(0)));
                *row[57] = low_16_bits_col57;
                let high_16_bits_col58 = ((((memory_id_to_big_value_tmp_92ff8_11.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_92ff8_11.get_m31(2)) * (M31_4)))
                    + (tmp_92ff8_12.as_m31()));
                *row[58] = high_16_bits_col58;
                let expected_word_tmp_92ff8_13 =
                    PackedUInt32::from_limbs(low_16_bits_col57, high_16_bits_col58);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_92ff8_14 = ((expected_word_tmp_92ff8_13.low()) >> (UInt16_9));
                let low_7_ms_bits_col59 = low_7_ms_bits_tmp_92ff8_14.as_m31();
                *row[59] = low_7_ms_bits_col59;
                let high_14_ms_bits_tmp_92ff8_15 =
                    ((expected_word_tmp_92ff8_13.high()) >> (UInt16_2));
                let high_14_ms_bits_col60 = high_14_ms_bits_tmp_92ff8_15.as_m31();
                *row[60] = high_14_ms_bits_col60;
                let high_5_ms_bits_tmp_92ff8_16 = ((high_14_ms_bits_tmp_92ff8_15) >> (UInt16_9));
                let high_5_ms_bits_col61 = high_5_ms_bits_tmp_92ff8_16.as_m31();
                *row[61] = high_5_ms_bits_col61;
                let range_check_7_2_5_inputs_1 = [
                    low_7_ms_bits_col59,
                    ((high_16_bits_col58) - ((high_14_ms_bits_col60) * (M31_4))),
                    high_5_ms_bits_col61,
                ]
                .unpack();
                *lookup_data.range_check_7_2_5_1 = [
                    low_7_ms_bits_col59,
                    ((high_16_bits_col58) - ((high_14_ms_bits_col60) * (M31_4))),
                    high_5_ms_bits_col61,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_92ff8_17 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_1_col36)),
                    );
                let message_word_1_id_col62 = memory_address_to_id_value_tmp_92ff8_17;
                *row[62] = message_word_1_id_col62;
                let memory_address_to_id_inputs_1 =
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_1_col36)).unpack();
                *lookup_data.memory_address_to_id_1 = [
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_1_col36)),
                    message_word_1_id_col62,
                ];
                let memory_id_to_big_inputs_1 = message_word_1_id_col62.unpack();
                *lookup_data.memory_id_to_big_1 = [
                    message_word_1_id_col62,
                    ((low_16_bits_col57) - ((low_7_ms_bits_col59) * (M31_512))),
                    ((low_7_ms_bits_col59)
                        + (((high_16_bits_col58) - ((high_14_ms_bits_col60) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col60) - ((high_5_ms_bits_col61) * (M31_512))),
                    high_5_ms_bits_col61,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                ];

                // Read Blake Word.

                let memory_address_to_id_value_tmp_92ff8_18 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_2_col37)),
                    );
                let memory_id_to_big_value_tmp_92ff8_19 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_92ff8_18);
                let tmp_92ff8_20 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_92ff8_19.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col63 = ((((memory_id_to_big_value_tmp_92ff8_19.get_m31(1))
                    - ((tmp_92ff8_20.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_92ff8_19.get_m31(0)));
                *row[63] = low_16_bits_col63;
                let high_16_bits_col64 = ((((memory_id_to_big_value_tmp_92ff8_19.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_92ff8_19.get_m31(2)) * (M31_4)))
                    + (tmp_92ff8_20.as_m31()));
                *row[64] = high_16_bits_col64;
                let expected_word_tmp_92ff8_21 =
                    PackedUInt32::from_limbs(low_16_bits_col63, high_16_bits_col64);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_92ff8_22 = ((expected_word_tmp_92ff8_21.low()) >> (UInt16_9));
                let low_7_ms_bits_col65 = low_7_ms_bits_tmp_92ff8_22.as_m31();
                *row[65] = low_7_ms_bits_col65;
                let high_14_ms_bits_tmp_92ff8_23 =
                    ((expected_word_tmp_92ff8_21.high()) >> (UInt16_2));
                let high_14_ms_bits_col66 = high_14_ms_bits_tmp_92ff8_23.as_m31();
                *row[66] = high_14_ms_bits_col66;
                let high_5_ms_bits_tmp_92ff8_24 = ((high_14_ms_bits_tmp_92ff8_23) >> (UInt16_9));
                let high_5_ms_bits_col67 = high_5_ms_bits_tmp_92ff8_24.as_m31();
                *row[67] = high_5_ms_bits_col67;
                let range_check_7_2_5_inputs_2 = [
                    low_7_ms_bits_col65,
                    ((high_16_bits_col64) - ((high_14_ms_bits_col66) * (M31_4))),
                    high_5_ms_bits_col67,
                ]
                .unpack();
                *lookup_data.range_check_7_2_5_2 = [
                    low_7_ms_bits_col65,
                    ((high_16_bits_col64) - ((high_14_ms_bits_col66) * (M31_4))),
                    high_5_ms_bits_col67,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_92ff8_25 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_2_col37)),
                    );
                let message_word_2_id_col68 = memory_address_to_id_value_tmp_92ff8_25;
                *row[68] = message_word_2_id_col68;
                let memory_address_to_id_inputs_2 =
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_2_col37)).unpack();
                *lookup_data.memory_address_to_id_2 = [
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_2_col37)),
                    message_word_2_id_col68,
                ];
                let memory_id_to_big_inputs_2 = message_word_2_id_col68.unpack();
                *lookup_data.memory_id_to_big_2 = [
                    message_word_2_id_col68,
                    ((low_16_bits_col63) - ((low_7_ms_bits_col65) * (M31_512))),
                    ((low_7_ms_bits_col65)
                        + (((high_16_bits_col64) - ((high_14_ms_bits_col66) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col66) - ((high_5_ms_bits_col67) * (M31_512))),
                    high_5_ms_bits_col67,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                ];

                // Read Blake Word.

                let memory_address_to_id_value_tmp_92ff8_26 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_3_col38)),
                    );
                let memory_id_to_big_value_tmp_92ff8_27 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_92ff8_26);
                let tmp_92ff8_28 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_92ff8_27.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col69 = ((((memory_id_to_big_value_tmp_92ff8_27.get_m31(1))
                    - ((tmp_92ff8_28.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_92ff8_27.get_m31(0)));
                *row[69] = low_16_bits_col69;
                let high_16_bits_col70 = ((((memory_id_to_big_value_tmp_92ff8_27.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_92ff8_27.get_m31(2)) * (M31_4)))
                    + (tmp_92ff8_28.as_m31()));
                *row[70] = high_16_bits_col70;
                let expected_word_tmp_92ff8_29 =
                    PackedUInt32::from_limbs(low_16_bits_col69, high_16_bits_col70);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_92ff8_30 = ((expected_word_tmp_92ff8_29.low()) >> (UInt16_9));
                let low_7_ms_bits_col71 = low_7_ms_bits_tmp_92ff8_30.as_m31();
                *row[71] = low_7_ms_bits_col71;
                let high_14_ms_bits_tmp_92ff8_31 =
                    ((expected_word_tmp_92ff8_29.high()) >> (UInt16_2));
                let high_14_ms_bits_col72 = high_14_ms_bits_tmp_92ff8_31.as_m31();
                *row[72] = high_14_ms_bits_col72;
                let high_5_ms_bits_tmp_92ff8_32 = ((high_14_ms_bits_tmp_92ff8_31) >> (UInt16_9));
                let high_5_ms_bits_col73 = high_5_ms_bits_tmp_92ff8_32.as_m31();
                *row[73] = high_5_ms_bits_col73;
                let range_check_7_2_5_inputs_3 = [
                    low_7_ms_bits_col71,
                    ((high_16_bits_col70) - ((high_14_ms_bits_col72) * (M31_4))),
                    high_5_ms_bits_col73,
                ]
                .unpack();
                *lookup_data.range_check_7_2_5_3 = [
                    low_7_ms_bits_col71,
                    ((high_16_bits_col70) - ((high_14_ms_bits_col72) * (M31_4))),
                    high_5_ms_bits_col73,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_92ff8_33 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_3_col38)),
                    );
                let message_word_3_id_col74 = memory_address_to_id_value_tmp_92ff8_33;
                *row[74] = message_word_3_id_col74;
                let memory_address_to_id_inputs_3 =
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_3_col38)).unpack();
                *lookup_data.memory_address_to_id_3 = [
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_3_col38)),
                    message_word_3_id_col74,
                ];
                let memory_id_to_big_inputs_3 = message_word_3_id_col74.unpack();
                *lookup_data.memory_id_to_big_3 = [
                    message_word_3_id_col74,
                    ((low_16_bits_col69) - ((low_7_ms_bits_col71) * (M31_512))),
                    ((low_7_ms_bits_col71)
                        + (((high_16_bits_col70) - ((high_14_ms_bits_col72) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col72) - ((high_5_ms_bits_col73) * (M31_512))),
                    high_5_ms_bits_col73,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                ];

                // Read Blake Word.

                let memory_address_to_id_value_tmp_92ff8_34 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_4_col39)),
                    );
                let memory_id_to_big_value_tmp_92ff8_35 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_92ff8_34);
                let tmp_92ff8_36 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_92ff8_35.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col75 = ((((memory_id_to_big_value_tmp_92ff8_35.get_m31(1))
                    - ((tmp_92ff8_36.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_92ff8_35.get_m31(0)));
                *row[75] = low_16_bits_col75;
                let high_16_bits_col76 = ((((memory_id_to_big_value_tmp_92ff8_35.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_92ff8_35.get_m31(2)) * (M31_4)))
                    + (tmp_92ff8_36.as_m31()));
                *row[76] = high_16_bits_col76;
                let expected_word_tmp_92ff8_37 =
                    PackedUInt32::from_limbs(low_16_bits_col75, high_16_bits_col76);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_92ff8_38 = ((expected_word_tmp_92ff8_37.low()) >> (UInt16_9));
                let low_7_ms_bits_col77 = low_7_ms_bits_tmp_92ff8_38.as_m31();
                *row[77] = low_7_ms_bits_col77;
                let high_14_ms_bits_tmp_92ff8_39 =
                    ((expected_word_tmp_92ff8_37.high()) >> (UInt16_2));
                let high_14_ms_bits_col78 = high_14_ms_bits_tmp_92ff8_39.as_m31();
                *row[78] = high_14_ms_bits_col78;
                let high_5_ms_bits_tmp_92ff8_40 = ((high_14_ms_bits_tmp_92ff8_39) >> (UInt16_9));
                let high_5_ms_bits_col79 = high_5_ms_bits_tmp_92ff8_40.as_m31();
                *row[79] = high_5_ms_bits_col79;
                let range_check_7_2_5_inputs_4 = [
                    low_7_ms_bits_col77,
                    ((high_16_bits_col76) - ((high_14_ms_bits_col78) * (M31_4))),
                    high_5_ms_bits_col79,
                ]
                .unpack();
                *lookup_data.range_check_7_2_5_4 = [
                    low_7_ms_bits_col77,
                    ((high_16_bits_col76) - ((high_14_ms_bits_col78) * (M31_4))),
                    high_5_ms_bits_col79,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_92ff8_41 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_4_col39)),
                    );
                let message_word_4_id_col80 = memory_address_to_id_value_tmp_92ff8_41;
                *row[80] = message_word_4_id_col80;
                let memory_address_to_id_inputs_4 =
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_4_col39)).unpack();
                *lookup_data.memory_address_to_id_4 = [
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_4_col39)),
                    message_word_4_id_col80,
                ];
                let memory_id_to_big_inputs_4 = message_word_4_id_col80.unpack();
                *lookup_data.memory_id_to_big_4 = [
                    message_word_4_id_col80,
                    ((low_16_bits_col75) - ((low_7_ms_bits_col77) * (M31_512))),
                    ((low_7_ms_bits_col77)
                        + (((high_16_bits_col76) - ((high_14_ms_bits_col78) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col78) - ((high_5_ms_bits_col79) * (M31_512))),
                    high_5_ms_bits_col79,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                ];

                // Read Blake Word.

                let memory_address_to_id_value_tmp_92ff8_42 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_5_col40)),
                    );
                let memory_id_to_big_value_tmp_92ff8_43 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_92ff8_42);
                let tmp_92ff8_44 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_92ff8_43.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col81 = ((((memory_id_to_big_value_tmp_92ff8_43.get_m31(1))
                    - ((tmp_92ff8_44.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_92ff8_43.get_m31(0)));
                *row[81] = low_16_bits_col81;
                let high_16_bits_col82 = ((((memory_id_to_big_value_tmp_92ff8_43.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_92ff8_43.get_m31(2)) * (M31_4)))
                    + (tmp_92ff8_44.as_m31()));
                *row[82] = high_16_bits_col82;
                let expected_word_tmp_92ff8_45 =
                    PackedUInt32::from_limbs(low_16_bits_col81, high_16_bits_col82);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_92ff8_46 = ((expected_word_tmp_92ff8_45.low()) >> (UInt16_9));
                let low_7_ms_bits_col83 = low_7_ms_bits_tmp_92ff8_46.as_m31();
                *row[83] = low_7_ms_bits_col83;
                let high_14_ms_bits_tmp_92ff8_47 =
                    ((expected_word_tmp_92ff8_45.high()) >> (UInt16_2));
                let high_14_ms_bits_col84 = high_14_ms_bits_tmp_92ff8_47.as_m31();
                *row[84] = high_14_ms_bits_col84;
                let high_5_ms_bits_tmp_92ff8_48 = ((high_14_ms_bits_tmp_92ff8_47) >> (UInt16_9));
                let high_5_ms_bits_col85 = high_5_ms_bits_tmp_92ff8_48.as_m31();
                *row[85] = high_5_ms_bits_col85;
                let range_check_7_2_5_inputs_5 = [
                    low_7_ms_bits_col83,
                    ((high_16_bits_col82) - ((high_14_ms_bits_col84) * (M31_4))),
                    high_5_ms_bits_col85,
                ]
                .unpack();
                *lookup_data.range_check_7_2_5_5 = [
                    low_7_ms_bits_col83,
                    ((high_16_bits_col82) - ((high_14_ms_bits_col84) * (M31_4))),
                    high_5_ms_bits_col85,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_92ff8_49 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_5_col40)),
                    );
                let message_word_5_id_col86 = memory_address_to_id_value_tmp_92ff8_49;
                *row[86] = message_word_5_id_col86;
                let memory_address_to_id_inputs_5 =
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_5_col40)).unpack();
                *lookup_data.memory_address_to_id_5 = [
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_5_col40)),
                    message_word_5_id_col86,
                ];
                let memory_id_to_big_inputs_5 = message_word_5_id_col86.unpack();
                *lookup_data.memory_id_to_big_5 = [
                    message_word_5_id_col86,
                    ((low_16_bits_col81) - ((low_7_ms_bits_col83) * (M31_512))),
                    ((low_7_ms_bits_col83)
                        + (((high_16_bits_col82) - ((high_14_ms_bits_col84) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col84) - ((high_5_ms_bits_col85) * (M31_512))),
                    high_5_ms_bits_col85,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                ];

                // Read Blake Word.

                let memory_address_to_id_value_tmp_92ff8_50 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_6_col41)),
                    );
                let memory_id_to_big_value_tmp_92ff8_51 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_92ff8_50);
                let tmp_92ff8_52 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_92ff8_51.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col87 = ((((memory_id_to_big_value_tmp_92ff8_51.get_m31(1))
                    - ((tmp_92ff8_52.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_92ff8_51.get_m31(0)));
                *row[87] = low_16_bits_col87;
                let high_16_bits_col88 = ((((memory_id_to_big_value_tmp_92ff8_51.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_92ff8_51.get_m31(2)) * (M31_4)))
                    + (tmp_92ff8_52.as_m31()));
                *row[88] = high_16_bits_col88;
                let expected_word_tmp_92ff8_53 =
                    PackedUInt32::from_limbs(low_16_bits_col87, high_16_bits_col88);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_92ff8_54 = ((expected_word_tmp_92ff8_53.low()) >> (UInt16_9));
                let low_7_ms_bits_col89 = low_7_ms_bits_tmp_92ff8_54.as_m31();
                *row[89] = low_7_ms_bits_col89;
                let high_14_ms_bits_tmp_92ff8_55 =
                    ((expected_word_tmp_92ff8_53.high()) >> (UInt16_2));
                let high_14_ms_bits_col90 = high_14_ms_bits_tmp_92ff8_55.as_m31();
                *row[90] = high_14_ms_bits_col90;
                let high_5_ms_bits_tmp_92ff8_56 = ((high_14_ms_bits_tmp_92ff8_55) >> (UInt16_9));
                let high_5_ms_bits_col91 = high_5_ms_bits_tmp_92ff8_56.as_m31();
                *row[91] = high_5_ms_bits_col91;
                let range_check_7_2_5_inputs_6 = [
                    low_7_ms_bits_col89,
                    ((high_16_bits_col88) - ((high_14_ms_bits_col90) * (M31_4))),
                    high_5_ms_bits_col91,
                ]
                .unpack();
                *lookup_data.range_check_7_2_5_6 = [
                    low_7_ms_bits_col89,
                    ((high_16_bits_col88) - ((high_14_ms_bits_col90) * (M31_4))),
                    high_5_ms_bits_col91,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_92ff8_57 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_6_col41)),
                    );
                let message_word_6_id_col92 = memory_address_to_id_value_tmp_92ff8_57;
                *row[92] = message_word_6_id_col92;
                let memory_address_to_id_inputs_6 =
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_6_col41)).unpack();
                *lookup_data.memory_address_to_id_6 = [
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_6_col41)),
                    message_word_6_id_col92,
                ];
                let memory_id_to_big_inputs_6 = message_word_6_id_col92.unpack();
                *lookup_data.memory_id_to_big_6 = [
                    message_word_6_id_col92,
                    ((low_16_bits_col87) - ((low_7_ms_bits_col89) * (M31_512))),
                    ((low_7_ms_bits_col89)
                        + (((high_16_bits_col88) - ((high_14_ms_bits_col90) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col90) - ((high_5_ms_bits_col91) * (M31_512))),
                    high_5_ms_bits_col91,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                ];

                // Read Blake Word.

                let memory_address_to_id_value_tmp_92ff8_58 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_7_col42)),
                    );
                let memory_id_to_big_value_tmp_92ff8_59 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_92ff8_58);
                let tmp_92ff8_60 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_92ff8_59.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col93 = ((((memory_id_to_big_value_tmp_92ff8_59.get_m31(1))
                    - ((tmp_92ff8_60.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_92ff8_59.get_m31(0)));
                *row[93] = low_16_bits_col93;
                let high_16_bits_col94 = ((((memory_id_to_big_value_tmp_92ff8_59.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_92ff8_59.get_m31(2)) * (M31_4)))
                    + (tmp_92ff8_60.as_m31()));
                *row[94] = high_16_bits_col94;
                let expected_word_tmp_92ff8_61 =
                    PackedUInt32::from_limbs(low_16_bits_col93, high_16_bits_col94);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_92ff8_62 = ((expected_word_tmp_92ff8_61.low()) >> (UInt16_9));
                let low_7_ms_bits_col95 = low_7_ms_bits_tmp_92ff8_62.as_m31();
                *row[95] = low_7_ms_bits_col95;
                let high_14_ms_bits_tmp_92ff8_63 =
                    ((expected_word_tmp_92ff8_61.high()) >> (UInt16_2));
                let high_14_ms_bits_col96 = high_14_ms_bits_tmp_92ff8_63.as_m31();
                *row[96] = high_14_ms_bits_col96;
                let high_5_ms_bits_tmp_92ff8_64 = ((high_14_ms_bits_tmp_92ff8_63) >> (UInt16_9));
                let high_5_ms_bits_col97 = high_5_ms_bits_tmp_92ff8_64.as_m31();
                *row[97] = high_5_ms_bits_col97;
                let range_check_7_2_5_inputs_7 = [
                    low_7_ms_bits_col95,
                    ((high_16_bits_col94) - ((high_14_ms_bits_col96) * (M31_4))),
                    high_5_ms_bits_col97,
                ]
                .unpack();
                *lookup_data.range_check_7_2_5_7 = [
                    low_7_ms_bits_col95,
                    ((high_16_bits_col94) - ((high_14_ms_bits_col96) * (M31_4))),
                    high_5_ms_bits_col97,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_92ff8_65 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_7_col42)),
                    );
                let message_word_7_id_col98 = memory_address_to_id_value_tmp_92ff8_65;
                *row[98] = message_word_7_id_col98;
                let memory_address_to_id_inputs_7 =
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_7_col42)).unpack();
                *lookup_data.memory_address_to_id_7 = [
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_7_col42)),
                    message_word_7_id_col98,
                ];
                let memory_id_to_big_inputs_7 = message_word_7_id_col98.unpack();
                *lookup_data.memory_id_to_big_7 = [
                    message_word_7_id_col98,
                    ((low_16_bits_col93) - ((low_7_ms_bits_col95) * (M31_512))),
                    ((low_7_ms_bits_col95)
                        + (((high_16_bits_col94) - ((high_14_ms_bits_col96) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col96) - ((high_5_ms_bits_col97) * (M31_512))),
                    high_5_ms_bits_col97,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                ];

                // Read Blake Word.

                let memory_address_to_id_value_tmp_92ff8_66 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_8_col43)),
                    );
                let memory_id_to_big_value_tmp_92ff8_67 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_92ff8_66);
                let tmp_92ff8_68 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_92ff8_67.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col99 = ((((memory_id_to_big_value_tmp_92ff8_67.get_m31(1))
                    - ((tmp_92ff8_68.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_92ff8_67.get_m31(0)));
                *row[99] = low_16_bits_col99;
                let high_16_bits_col100 = ((((memory_id_to_big_value_tmp_92ff8_67.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_92ff8_67.get_m31(2)) * (M31_4)))
                    + (tmp_92ff8_68.as_m31()));
                *row[100] = high_16_bits_col100;
                let expected_word_tmp_92ff8_69 =
                    PackedUInt32::from_limbs(low_16_bits_col99, high_16_bits_col100);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_92ff8_70 = ((expected_word_tmp_92ff8_69.low()) >> (UInt16_9));
                let low_7_ms_bits_col101 = low_7_ms_bits_tmp_92ff8_70.as_m31();
                *row[101] = low_7_ms_bits_col101;
                let high_14_ms_bits_tmp_92ff8_71 =
                    ((expected_word_tmp_92ff8_69.high()) >> (UInt16_2));
                let high_14_ms_bits_col102 = high_14_ms_bits_tmp_92ff8_71.as_m31();
                *row[102] = high_14_ms_bits_col102;
                let high_5_ms_bits_tmp_92ff8_72 = ((high_14_ms_bits_tmp_92ff8_71) >> (UInt16_9));
                let high_5_ms_bits_col103 = high_5_ms_bits_tmp_92ff8_72.as_m31();
                *row[103] = high_5_ms_bits_col103;
                let range_check_7_2_5_inputs_8 = [
                    low_7_ms_bits_col101,
                    ((high_16_bits_col100) - ((high_14_ms_bits_col102) * (M31_4))),
                    high_5_ms_bits_col103,
                ]
                .unpack();
                *lookup_data.range_check_7_2_5_8 = [
                    low_7_ms_bits_col101,
                    ((high_16_bits_col100) - ((high_14_ms_bits_col102) * (M31_4))),
                    high_5_ms_bits_col103,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_92ff8_73 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_8_col43)),
                    );
                let message_word_8_id_col104 = memory_address_to_id_value_tmp_92ff8_73;
                *row[104] = message_word_8_id_col104;
                let memory_address_to_id_inputs_8 =
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_8_col43)).unpack();
                *lookup_data.memory_address_to_id_8 = [
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_8_col43)),
                    message_word_8_id_col104,
                ];
                let memory_id_to_big_inputs_8 = message_word_8_id_col104.unpack();
                *lookup_data.memory_id_to_big_8 = [
                    message_word_8_id_col104,
                    ((low_16_bits_col99) - ((low_7_ms_bits_col101) * (M31_512))),
                    ((low_7_ms_bits_col101)
                        + (((high_16_bits_col100) - ((high_14_ms_bits_col102) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col102) - ((high_5_ms_bits_col103) * (M31_512))),
                    high_5_ms_bits_col103,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                ];

                // Read Blake Word.

                let memory_address_to_id_value_tmp_92ff8_74 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_9_col44)),
                    );
                let memory_id_to_big_value_tmp_92ff8_75 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_92ff8_74);
                let tmp_92ff8_76 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_92ff8_75.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col105 = ((((memory_id_to_big_value_tmp_92ff8_75.get_m31(1))
                    - ((tmp_92ff8_76.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_92ff8_75.get_m31(0)));
                *row[105] = low_16_bits_col105;
                let high_16_bits_col106 = ((((memory_id_to_big_value_tmp_92ff8_75.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_92ff8_75.get_m31(2)) * (M31_4)))
                    + (tmp_92ff8_76.as_m31()));
                *row[106] = high_16_bits_col106;
                let expected_word_tmp_92ff8_77 =
                    PackedUInt32::from_limbs(low_16_bits_col105, high_16_bits_col106);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_92ff8_78 = ((expected_word_tmp_92ff8_77.low()) >> (UInt16_9));
                let low_7_ms_bits_col107 = low_7_ms_bits_tmp_92ff8_78.as_m31();
                *row[107] = low_7_ms_bits_col107;
                let high_14_ms_bits_tmp_92ff8_79 =
                    ((expected_word_tmp_92ff8_77.high()) >> (UInt16_2));
                let high_14_ms_bits_col108 = high_14_ms_bits_tmp_92ff8_79.as_m31();
                *row[108] = high_14_ms_bits_col108;
                let high_5_ms_bits_tmp_92ff8_80 = ((high_14_ms_bits_tmp_92ff8_79) >> (UInt16_9));
                let high_5_ms_bits_col109 = high_5_ms_bits_tmp_92ff8_80.as_m31();
                *row[109] = high_5_ms_bits_col109;
                let range_check_7_2_5_inputs_9 = [
                    low_7_ms_bits_col107,
                    ((high_16_bits_col106) - ((high_14_ms_bits_col108) * (M31_4))),
                    high_5_ms_bits_col109,
                ]
                .unpack();
                *lookup_data.range_check_7_2_5_9 = [
                    low_7_ms_bits_col107,
                    ((high_16_bits_col106) - ((high_14_ms_bits_col108) * (M31_4))),
                    high_5_ms_bits_col109,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_92ff8_81 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_9_col44)),
                    );
                let message_word_9_id_col110 = memory_address_to_id_value_tmp_92ff8_81;
                *row[110] = message_word_9_id_col110;
                let memory_address_to_id_inputs_9 =
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_9_col44)).unpack();
                *lookup_data.memory_address_to_id_9 = [
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_9_col44)),
                    message_word_9_id_col110,
                ];
                let memory_id_to_big_inputs_9 = message_word_9_id_col110.unpack();
                *lookup_data.memory_id_to_big_9 = [
                    message_word_9_id_col110,
                    ((low_16_bits_col105) - ((low_7_ms_bits_col107) * (M31_512))),
                    ((low_7_ms_bits_col107)
                        + (((high_16_bits_col106) - ((high_14_ms_bits_col108) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col108) - ((high_5_ms_bits_col109) * (M31_512))),
                    high_5_ms_bits_col109,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                ];

                // Read Blake Word.

                let memory_address_to_id_value_tmp_92ff8_82 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_10_col45)),
                    );
                let memory_id_to_big_value_tmp_92ff8_83 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_92ff8_82);
                let tmp_92ff8_84 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_92ff8_83.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col111 = ((((memory_id_to_big_value_tmp_92ff8_83.get_m31(1))
                    - ((tmp_92ff8_84.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_92ff8_83.get_m31(0)));
                *row[111] = low_16_bits_col111;
                let high_16_bits_col112 = ((((memory_id_to_big_value_tmp_92ff8_83.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_92ff8_83.get_m31(2)) * (M31_4)))
                    + (tmp_92ff8_84.as_m31()));
                *row[112] = high_16_bits_col112;
                let expected_word_tmp_92ff8_85 =
                    PackedUInt32::from_limbs(low_16_bits_col111, high_16_bits_col112);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_92ff8_86 = ((expected_word_tmp_92ff8_85.low()) >> (UInt16_9));
                let low_7_ms_bits_col113 = low_7_ms_bits_tmp_92ff8_86.as_m31();
                *row[113] = low_7_ms_bits_col113;
                let high_14_ms_bits_tmp_92ff8_87 =
                    ((expected_word_tmp_92ff8_85.high()) >> (UInt16_2));
                let high_14_ms_bits_col114 = high_14_ms_bits_tmp_92ff8_87.as_m31();
                *row[114] = high_14_ms_bits_col114;
                let high_5_ms_bits_tmp_92ff8_88 = ((high_14_ms_bits_tmp_92ff8_87) >> (UInt16_9));
                let high_5_ms_bits_col115 = high_5_ms_bits_tmp_92ff8_88.as_m31();
                *row[115] = high_5_ms_bits_col115;
                let range_check_7_2_5_inputs_10 = [
                    low_7_ms_bits_col113,
                    ((high_16_bits_col112) - ((high_14_ms_bits_col114) * (M31_4))),
                    high_5_ms_bits_col115,
                ]
                .unpack();
                *lookup_data.range_check_7_2_5_10 = [
                    low_7_ms_bits_col113,
                    ((high_16_bits_col112) - ((high_14_ms_bits_col114) * (M31_4))),
                    high_5_ms_bits_col115,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_92ff8_89 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_10_col45)),
                    );
                let message_word_10_id_col116 = memory_address_to_id_value_tmp_92ff8_89;
                *row[116] = message_word_10_id_col116;
                let memory_address_to_id_inputs_10 =
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_10_col45)).unpack();
                *lookup_data.memory_address_to_id_10 = [
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_10_col45)),
                    message_word_10_id_col116,
                ];
                let memory_id_to_big_inputs_10 = message_word_10_id_col116.unpack();
                *lookup_data.memory_id_to_big_10 = [
                    message_word_10_id_col116,
                    ((low_16_bits_col111) - ((low_7_ms_bits_col113) * (M31_512))),
                    ((low_7_ms_bits_col113)
                        + (((high_16_bits_col112) - ((high_14_ms_bits_col114) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col114) - ((high_5_ms_bits_col115) * (M31_512))),
                    high_5_ms_bits_col115,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                ];

                // Read Blake Word.

                let memory_address_to_id_value_tmp_92ff8_90 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_11_col46)),
                    );
                let memory_id_to_big_value_tmp_92ff8_91 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_92ff8_90);
                let tmp_92ff8_92 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_92ff8_91.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col117 = ((((memory_id_to_big_value_tmp_92ff8_91.get_m31(1))
                    - ((tmp_92ff8_92.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_92ff8_91.get_m31(0)));
                *row[117] = low_16_bits_col117;
                let high_16_bits_col118 = ((((memory_id_to_big_value_tmp_92ff8_91.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_92ff8_91.get_m31(2)) * (M31_4)))
                    + (tmp_92ff8_92.as_m31()));
                *row[118] = high_16_bits_col118;
                let expected_word_tmp_92ff8_93 =
                    PackedUInt32::from_limbs(low_16_bits_col117, high_16_bits_col118);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_92ff8_94 = ((expected_word_tmp_92ff8_93.low()) >> (UInt16_9));
                let low_7_ms_bits_col119 = low_7_ms_bits_tmp_92ff8_94.as_m31();
                *row[119] = low_7_ms_bits_col119;
                let high_14_ms_bits_tmp_92ff8_95 =
                    ((expected_word_tmp_92ff8_93.high()) >> (UInt16_2));
                let high_14_ms_bits_col120 = high_14_ms_bits_tmp_92ff8_95.as_m31();
                *row[120] = high_14_ms_bits_col120;
                let high_5_ms_bits_tmp_92ff8_96 = ((high_14_ms_bits_tmp_92ff8_95) >> (UInt16_9));
                let high_5_ms_bits_col121 = high_5_ms_bits_tmp_92ff8_96.as_m31();
                *row[121] = high_5_ms_bits_col121;
                let range_check_7_2_5_inputs_11 = [
                    low_7_ms_bits_col119,
                    ((high_16_bits_col118) - ((high_14_ms_bits_col120) * (M31_4))),
                    high_5_ms_bits_col121,
                ]
                .unpack();
                *lookup_data.range_check_7_2_5_11 = [
                    low_7_ms_bits_col119,
                    ((high_16_bits_col118) - ((high_14_ms_bits_col120) * (M31_4))),
                    high_5_ms_bits_col121,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_92ff8_97 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_11_col46)),
                    );
                let message_word_11_id_col122 = memory_address_to_id_value_tmp_92ff8_97;
                *row[122] = message_word_11_id_col122;
                let memory_address_to_id_inputs_11 =
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_11_col46)).unpack();
                *lookup_data.memory_address_to_id_11 = [
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_11_col46)),
                    message_word_11_id_col122,
                ];
                let memory_id_to_big_inputs_11 = message_word_11_id_col122.unpack();
                *lookup_data.memory_id_to_big_11 = [
                    message_word_11_id_col122,
                    ((low_16_bits_col117) - ((low_7_ms_bits_col119) * (M31_512))),
                    ((low_7_ms_bits_col119)
                        + (((high_16_bits_col118) - ((high_14_ms_bits_col120) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col120) - ((high_5_ms_bits_col121) * (M31_512))),
                    high_5_ms_bits_col121,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                ];

                // Read Blake Word.

                let memory_address_to_id_value_tmp_92ff8_98 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_12_col47)),
                    );
                let memory_id_to_big_value_tmp_92ff8_99 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_92ff8_98);
                let tmp_92ff8_100 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_92ff8_99.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col123 = ((((memory_id_to_big_value_tmp_92ff8_99.get_m31(1))
                    - ((tmp_92ff8_100.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_92ff8_99.get_m31(0)));
                *row[123] = low_16_bits_col123;
                let high_16_bits_col124 = ((((memory_id_to_big_value_tmp_92ff8_99.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_92ff8_99.get_m31(2)) * (M31_4)))
                    + (tmp_92ff8_100.as_m31()));
                *row[124] = high_16_bits_col124;
                let expected_word_tmp_92ff8_101 =
                    PackedUInt32::from_limbs(low_16_bits_col123, high_16_bits_col124);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_92ff8_102 =
                    ((expected_word_tmp_92ff8_101.low()) >> (UInt16_9));
                let low_7_ms_bits_col125 = low_7_ms_bits_tmp_92ff8_102.as_m31();
                *row[125] = low_7_ms_bits_col125;
                let high_14_ms_bits_tmp_92ff8_103 =
                    ((expected_word_tmp_92ff8_101.high()) >> (UInt16_2));
                let high_14_ms_bits_col126 = high_14_ms_bits_tmp_92ff8_103.as_m31();
                *row[126] = high_14_ms_bits_col126;
                let high_5_ms_bits_tmp_92ff8_104 = ((high_14_ms_bits_tmp_92ff8_103) >> (UInt16_9));
                let high_5_ms_bits_col127 = high_5_ms_bits_tmp_92ff8_104.as_m31();
                *row[127] = high_5_ms_bits_col127;
                let range_check_7_2_5_inputs_12 = [
                    low_7_ms_bits_col125,
                    ((high_16_bits_col124) - ((high_14_ms_bits_col126) * (M31_4))),
                    high_5_ms_bits_col127,
                ]
                .unpack();
                *lookup_data.range_check_7_2_5_12 = [
                    low_7_ms_bits_col125,
                    ((high_16_bits_col124) - ((high_14_ms_bits_col126) * (M31_4))),
                    high_5_ms_bits_col127,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_92ff8_105 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_12_col47)),
                    );
                let message_word_12_id_col128 = memory_address_to_id_value_tmp_92ff8_105;
                *row[128] = message_word_12_id_col128;
                let memory_address_to_id_inputs_12 =
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_12_col47)).unpack();
                *lookup_data.memory_address_to_id_12 = [
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_12_col47)),
                    message_word_12_id_col128,
                ];
                let memory_id_to_big_inputs_12 = message_word_12_id_col128.unpack();
                *lookup_data.memory_id_to_big_12 = [
                    message_word_12_id_col128,
                    ((low_16_bits_col123) - ((low_7_ms_bits_col125) * (M31_512))),
                    ((low_7_ms_bits_col125)
                        + (((high_16_bits_col124) - ((high_14_ms_bits_col126) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col126) - ((high_5_ms_bits_col127) * (M31_512))),
                    high_5_ms_bits_col127,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                ];

                // Read Blake Word.

                let memory_address_to_id_value_tmp_92ff8_106 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_13_col48)),
                    );
                let memory_id_to_big_value_tmp_92ff8_107 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_92ff8_106);
                let tmp_92ff8_108 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_92ff8_107.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col129 = ((((memory_id_to_big_value_tmp_92ff8_107.get_m31(1))
                    - ((tmp_92ff8_108.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_92ff8_107.get_m31(0)));
                *row[129] = low_16_bits_col129;
                let high_16_bits_col130 = ((((memory_id_to_big_value_tmp_92ff8_107.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_92ff8_107.get_m31(2)) * (M31_4)))
                    + (tmp_92ff8_108.as_m31()));
                *row[130] = high_16_bits_col130;
                let expected_word_tmp_92ff8_109 =
                    PackedUInt32::from_limbs(low_16_bits_col129, high_16_bits_col130);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_92ff8_110 =
                    ((expected_word_tmp_92ff8_109.low()) >> (UInt16_9));
                let low_7_ms_bits_col131 = low_7_ms_bits_tmp_92ff8_110.as_m31();
                *row[131] = low_7_ms_bits_col131;
                let high_14_ms_bits_tmp_92ff8_111 =
                    ((expected_word_tmp_92ff8_109.high()) >> (UInt16_2));
                let high_14_ms_bits_col132 = high_14_ms_bits_tmp_92ff8_111.as_m31();
                *row[132] = high_14_ms_bits_col132;
                let high_5_ms_bits_tmp_92ff8_112 = ((high_14_ms_bits_tmp_92ff8_111) >> (UInt16_9));
                let high_5_ms_bits_col133 = high_5_ms_bits_tmp_92ff8_112.as_m31();
                *row[133] = high_5_ms_bits_col133;
                let range_check_7_2_5_inputs_13 = [
                    low_7_ms_bits_col131,
                    ((high_16_bits_col130) - ((high_14_ms_bits_col132) * (M31_4))),
                    high_5_ms_bits_col133,
                ]
                .unpack();
                *lookup_data.range_check_7_2_5_13 = [
                    low_7_ms_bits_col131,
                    ((high_16_bits_col130) - ((high_14_ms_bits_col132) * (M31_4))),
                    high_5_ms_bits_col133,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_92ff8_113 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_13_col48)),
                    );
                let message_word_13_id_col134 = memory_address_to_id_value_tmp_92ff8_113;
                *row[134] = message_word_13_id_col134;
                let memory_address_to_id_inputs_13 =
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_13_col48)).unpack();
                *lookup_data.memory_address_to_id_13 = [
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_13_col48)),
                    message_word_13_id_col134,
                ];
                let memory_id_to_big_inputs_13 = message_word_13_id_col134.unpack();
                *lookup_data.memory_id_to_big_13 = [
                    message_word_13_id_col134,
                    ((low_16_bits_col129) - ((low_7_ms_bits_col131) * (M31_512))),
                    ((low_7_ms_bits_col131)
                        + (((high_16_bits_col130) - ((high_14_ms_bits_col132) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col132) - ((high_5_ms_bits_col133) * (M31_512))),
                    high_5_ms_bits_col133,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                ];

                // Read Blake Word.

                let memory_address_to_id_value_tmp_92ff8_114 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_14_col49)),
                    );
                let memory_id_to_big_value_tmp_92ff8_115 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_92ff8_114);
                let tmp_92ff8_116 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_92ff8_115.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col135 = ((((memory_id_to_big_value_tmp_92ff8_115.get_m31(1))
                    - ((tmp_92ff8_116.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_92ff8_115.get_m31(0)));
                *row[135] = low_16_bits_col135;
                let high_16_bits_col136 = ((((memory_id_to_big_value_tmp_92ff8_115.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_92ff8_115.get_m31(2)) * (M31_4)))
                    + (tmp_92ff8_116.as_m31()));
                *row[136] = high_16_bits_col136;
                let expected_word_tmp_92ff8_117 =
                    PackedUInt32::from_limbs(low_16_bits_col135, high_16_bits_col136);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_92ff8_118 =
                    ((expected_word_tmp_92ff8_117.low()) >> (UInt16_9));
                let low_7_ms_bits_col137 = low_7_ms_bits_tmp_92ff8_118.as_m31();
                *row[137] = low_7_ms_bits_col137;
                let high_14_ms_bits_tmp_92ff8_119 =
                    ((expected_word_tmp_92ff8_117.high()) >> (UInt16_2));
                let high_14_ms_bits_col138 = high_14_ms_bits_tmp_92ff8_119.as_m31();
                *row[138] = high_14_ms_bits_col138;
                let high_5_ms_bits_tmp_92ff8_120 = ((high_14_ms_bits_tmp_92ff8_119) >> (UInt16_9));
                let high_5_ms_bits_col139 = high_5_ms_bits_tmp_92ff8_120.as_m31();
                *row[139] = high_5_ms_bits_col139;
                let range_check_7_2_5_inputs_14 = [
                    low_7_ms_bits_col137,
                    ((high_16_bits_col136) - ((high_14_ms_bits_col138) * (M31_4))),
                    high_5_ms_bits_col139,
                ]
                .unpack();
                *lookup_data.range_check_7_2_5_14 = [
                    low_7_ms_bits_col137,
                    ((high_16_bits_col136) - ((high_14_ms_bits_col138) * (M31_4))),
                    high_5_ms_bits_col139,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_92ff8_121 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_14_col49)),
                    );
                let message_word_14_id_col140 = memory_address_to_id_value_tmp_92ff8_121;
                *row[140] = message_word_14_id_col140;
                let memory_address_to_id_inputs_14 =
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_14_col49)).unpack();
                *lookup_data.memory_address_to_id_14 = [
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_14_col49)),
                    message_word_14_id_col140,
                ];
                let memory_id_to_big_inputs_14 = message_word_14_id_col140.unpack();
                *lookup_data.memory_id_to_big_14 = [
                    message_word_14_id_col140,
                    ((low_16_bits_col135) - ((low_7_ms_bits_col137) * (M31_512))),
                    ((low_7_ms_bits_col137)
                        + (((high_16_bits_col136) - ((high_14_ms_bits_col138) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col138) - ((high_5_ms_bits_col139) * (M31_512))),
                    high_5_ms_bits_col139,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                ];

                // Read Blake Word.

                let memory_address_to_id_value_tmp_92ff8_122 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_15_col50)),
                    );
                let memory_id_to_big_value_tmp_92ff8_123 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_92ff8_122);
                let tmp_92ff8_124 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_92ff8_123.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col141 = ((((memory_id_to_big_value_tmp_92ff8_123.get_m31(1))
                    - ((tmp_92ff8_124.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_92ff8_123.get_m31(0)));
                *row[141] = low_16_bits_col141;
                let high_16_bits_col142 = ((((memory_id_to_big_value_tmp_92ff8_123.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_92ff8_123.get_m31(2)) * (M31_4)))
                    + (tmp_92ff8_124.as_m31()));
                *row[142] = high_16_bits_col142;
                let expected_word_tmp_92ff8_125 =
                    PackedUInt32::from_limbs(low_16_bits_col141, high_16_bits_col142);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_92ff8_126 =
                    ((expected_word_tmp_92ff8_125.low()) >> (UInt16_9));
                let low_7_ms_bits_col143 = low_7_ms_bits_tmp_92ff8_126.as_m31();
                *row[143] = low_7_ms_bits_col143;
                let high_14_ms_bits_tmp_92ff8_127 =
                    ((expected_word_tmp_92ff8_125.high()) >> (UInt16_2));
                let high_14_ms_bits_col144 = high_14_ms_bits_tmp_92ff8_127.as_m31();
                *row[144] = high_14_ms_bits_col144;
                let high_5_ms_bits_tmp_92ff8_128 = ((high_14_ms_bits_tmp_92ff8_127) >> (UInt16_9));
                let high_5_ms_bits_col145 = high_5_ms_bits_tmp_92ff8_128.as_m31();
                *row[145] = high_5_ms_bits_col145;
                let range_check_7_2_5_inputs_15 = [
                    low_7_ms_bits_col143,
                    ((high_16_bits_col142) - ((high_14_ms_bits_col144) * (M31_4))),
                    high_5_ms_bits_col145,
                ]
                .unpack();
                *lookup_data.range_check_7_2_5_15 = [
                    low_7_ms_bits_col143,
                    ((high_16_bits_col142) - ((high_14_ms_bits_col144) * (M31_4))),
                    high_5_ms_bits_col145,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_92ff8_129 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col34) + (blake_round_sigma_output_limb_15_col50)),
                    );
                let message_word_15_id_col146 = memory_address_to_id_value_tmp_92ff8_129;
                *row[146] = message_word_15_id_col146;
                let memory_address_to_id_inputs_15 =
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_15_col50)).unpack();
                *lookup_data.memory_address_to_id_15 = [
                    ((input_limb_34_col34) + (blake_round_sigma_output_limb_15_col50)),
                    message_word_15_id_col146,
                ];
                let memory_id_to_big_inputs_15 = message_word_15_id_col146.unpack();
                *lookup_data.memory_id_to_big_15 = [
                    message_word_15_id_col146,
                    ((low_16_bits_col141) - ((low_7_ms_bits_col143) * (M31_512))),
                    ((low_7_ms_bits_col143)
                        + (((high_16_bits_col142) - ((high_14_ms_bits_col144) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col144) - ((high_5_ms_bits_col145) * (M31_512))),
                    high_5_ms_bits_col145,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                ];

                let blake_g_inputs_0 = [
                    input_tmp_92ff8_0.2 .0[0],
                    input_tmp_92ff8_0.2 .0[4],
                    input_tmp_92ff8_0.2 .0[8],
                    input_tmp_92ff8_0.2 .0[12],
                    expected_word_tmp_92ff8_5,
                    expected_word_tmp_92ff8_13,
                ];
                let blake_g_output_tmp_92ff8_130 = BlakeG::deduce_output([
                    input_tmp_92ff8_0.2 .0[0],
                    input_tmp_92ff8_0.2 .0[4],
                    input_tmp_92ff8_0.2 .0[8],
                    input_tmp_92ff8_0.2 .0[12],
                    expected_word_tmp_92ff8_5,
                    expected_word_tmp_92ff8_13,
                ]);
                let blake_g_output_limb_0_col147 = blake_g_output_tmp_92ff8_130[0].low().as_m31();
                *row[147] = blake_g_output_limb_0_col147;
                let blake_g_output_limb_1_col148 = blake_g_output_tmp_92ff8_130[0].high().as_m31();
                *row[148] = blake_g_output_limb_1_col148;
                let blake_g_output_limb_2_col149 = blake_g_output_tmp_92ff8_130[1].low().as_m31();
                *row[149] = blake_g_output_limb_2_col149;
                let blake_g_output_limb_3_col150 = blake_g_output_tmp_92ff8_130[1].high().as_m31();
                *row[150] = blake_g_output_limb_3_col150;
                let blake_g_output_limb_4_col151 = blake_g_output_tmp_92ff8_130[2].low().as_m31();
                *row[151] = blake_g_output_limb_4_col151;
                let blake_g_output_limb_5_col152 = blake_g_output_tmp_92ff8_130[2].high().as_m31();
                *row[152] = blake_g_output_limb_5_col152;
                let blake_g_output_limb_6_col153 = blake_g_output_tmp_92ff8_130[3].low().as_m31();
                *row[153] = blake_g_output_limb_6_col153;
                let blake_g_output_limb_7_col154 = blake_g_output_tmp_92ff8_130[3].high().as_m31();
                *row[154] = blake_g_output_limb_7_col154;
                *lookup_data.blake_g_0 = [
                    input_limb_2_col2,
                    input_limb_3_col3,
                    input_limb_10_col10,
                    input_limb_11_col11,
                    input_limb_18_col18,
                    input_limb_19_col19,
                    input_limb_26_col26,
                    input_limb_27_col27,
                    low_16_bits_col51,
                    high_16_bits_col52,
                    low_16_bits_col57,
                    high_16_bits_col58,
                    blake_g_output_limb_0_col147,
                    blake_g_output_limb_1_col148,
                    blake_g_output_limb_2_col149,
                    blake_g_output_limb_3_col150,
                    blake_g_output_limb_4_col151,
                    blake_g_output_limb_5_col152,
                    blake_g_output_limb_6_col153,
                    blake_g_output_limb_7_col154,
                ];
                let blake_g_inputs_1 = [
                    input_tmp_92ff8_0.2 .0[1],
                    input_tmp_92ff8_0.2 .0[5],
                    input_tmp_92ff8_0.2 .0[9],
                    input_tmp_92ff8_0.2 .0[13],
                    expected_word_tmp_92ff8_21,
                    expected_word_tmp_92ff8_29,
                ];
                let blake_g_output_tmp_92ff8_131 = BlakeG::deduce_output([
                    input_tmp_92ff8_0.2 .0[1],
                    input_tmp_92ff8_0.2 .0[5],
                    input_tmp_92ff8_0.2 .0[9],
                    input_tmp_92ff8_0.2 .0[13],
                    expected_word_tmp_92ff8_21,
                    expected_word_tmp_92ff8_29,
                ]);
                let blake_g_output_limb_0_col155 = blake_g_output_tmp_92ff8_131[0].low().as_m31();
                *row[155] = blake_g_output_limb_0_col155;
                let blake_g_output_limb_1_col156 = blake_g_output_tmp_92ff8_131[0].high().as_m31();
                *row[156] = blake_g_output_limb_1_col156;
                let blake_g_output_limb_2_col157 = blake_g_output_tmp_92ff8_131[1].low().as_m31();
                *row[157] = blake_g_output_limb_2_col157;
                let blake_g_output_limb_3_col158 = blake_g_output_tmp_92ff8_131[1].high().as_m31();
                *row[158] = blake_g_output_limb_3_col158;
                let blake_g_output_limb_4_col159 = blake_g_output_tmp_92ff8_131[2].low().as_m31();
                *row[159] = blake_g_output_limb_4_col159;
                let blake_g_output_limb_5_col160 = blake_g_output_tmp_92ff8_131[2].high().as_m31();
                *row[160] = blake_g_output_limb_5_col160;
                let blake_g_output_limb_6_col161 = blake_g_output_tmp_92ff8_131[3].low().as_m31();
                *row[161] = blake_g_output_limb_6_col161;
                let blake_g_output_limb_7_col162 = blake_g_output_tmp_92ff8_131[3].high().as_m31();
                *row[162] = blake_g_output_limb_7_col162;
                *lookup_data.blake_g_1 = [
                    input_limb_4_col4,
                    input_limb_5_col5,
                    input_limb_12_col12,
                    input_limb_13_col13,
                    input_limb_20_col20,
                    input_limb_21_col21,
                    input_limb_28_col28,
                    input_limb_29_col29,
                    low_16_bits_col63,
                    high_16_bits_col64,
                    low_16_bits_col69,
                    high_16_bits_col70,
                    blake_g_output_limb_0_col155,
                    blake_g_output_limb_1_col156,
                    blake_g_output_limb_2_col157,
                    blake_g_output_limb_3_col158,
                    blake_g_output_limb_4_col159,
                    blake_g_output_limb_5_col160,
                    blake_g_output_limb_6_col161,
                    blake_g_output_limb_7_col162,
                ];
                let blake_g_inputs_2 = [
                    input_tmp_92ff8_0.2 .0[2],
                    input_tmp_92ff8_0.2 .0[6],
                    input_tmp_92ff8_0.2 .0[10],
                    input_tmp_92ff8_0.2 .0[14],
                    expected_word_tmp_92ff8_37,
                    expected_word_tmp_92ff8_45,
                ];
                let blake_g_output_tmp_92ff8_132 = BlakeG::deduce_output([
                    input_tmp_92ff8_0.2 .0[2],
                    input_tmp_92ff8_0.2 .0[6],
                    input_tmp_92ff8_0.2 .0[10],
                    input_tmp_92ff8_0.2 .0[14],
                    expected_word_tmp_92ff8_37,
                    expected_word_tmp_92ff8_45,
                ]);
                let blake_g_output_limb_0_col163 = blake_g_output_tmp_92ff8_132[0].low().as_m31();
                *row[163] = blake_g_output_limb_0_col163;
                let blake_g_output_limb_1_col164 = blake_g_output_tmp_92ff8_132[0].high().as_m31();
                *row[164] = blake_g_output_limb_1_col164;
                let blake_g_output_limb_2_col165 = blake_g_output_tmp_92ff8_132[1].low().as_m31();
                *row[165] = blake_g_output_limb_2_col165;
                let blake_g_output_limb_3_col166 = blake_g_output_tmp_92ff8_132[1].high().as_m31();
                *row[166] = blake_g_output_limb_3_col166;
                let blake_g_output_limb_4_col167 = blake_g_output_tmp_92ff8_132[2].low().as_m31();
                *row[167] = blake_g_output_limb_4_col167;
                let blake_g_output_limb_5_col168 = blake_g_output_tmp_92ff8_132[2].high().as_m31();
                *row[168] = blake_g_output_limb_5_col168;
                let blake_g_output_limb_6_col169 = blake_g_output_tmp_92ff8_132[3].low().as_m31();
                *row[169] = blake_g_output_limb_6_col169;
                let blake_g_output_limb_7_col170 = blake_g_output_tmp_92ff8_132[3].high().as_m31();
                *row[170] = blake_g_output_limb_7_col170;
                *lookup_data.blake_g_2 = [
                    input_limb_6_col6,
                    input_limb_7_col7,
                    input_limb_14_col14,
                    input_limb_15_col15,
                    input_limb_22_col22,
                    input_limb_23_col23,
                    input_limb_30_col30,
                    input_limb_31_col31,
                    low_16_bits_col75,
                    high_16_bits_col76,
                    low_16_bits_col81,
                    high_16_bits_col82,
                    blake_g_output_limb_0_col163,
                    blake_g_output_limb_1_col164,
                    blake_g_output_limb_2_col165,
                    blake_g_output_limb_3_col166,
                    blake_g_output_limb_4_col167,
                    blake_g_output_limb_5_col168,
                    blake_g_output_limb_6_col169,
                    blake_g_output_limb_7_col170,
                ];
                let blake_g_inputs_3 = [
                    input_tmp_92ff8_0.2 .0[3],
                    input_tmp_92ff8_0.2 .0[7],
                    input_tmp_92ff8_0.2 .0[11],
                    input_tmp_92ff8_0.2 .0[15],
                    expected_word_tmp_92ff8_53,
                    expected_word_tmp_92ff8_61,
                ];
                let blake_g_output_tmp_92ff8_133 = BlakeG::deduce_output([
                    input_tmp_92ff8_0.2 .0[3],
                    input_tmp_92ff8_0.2 .0[7],
                    input_tmp_92ff8_0.2 .0[11],
                    input_tmp_92ff8_0.2 .0[15],
                    expected_word_tmp_92ff8_53,
                    expected_word_tmp_92ff8_61,
                ]);
                let blake_g_output_limb_0_col171 = blake_g_output_tmp_92ff8_133[0].low().as_m31();
                *row[171] = blake_g_output_limb_0_col171;
                let blake_g_output_limb_1_col172 = blake_g_output_tmp_92ff8_133[0].high().as_m31();
                *row[172] = blake_g_output_limb_1_col172;
                let blake_g_output_limb_2_col173 = blake_g_output_tmp_92ff8_133[1].low().as_m31();
                *row[173] = blake_g_output_limb_2_col173;
                let blake_g_output_limb_3_col174 = blake_g_output_tmp_92ff8_133[1].high().as_m31();
                *row[174] = blake_g_output_limb_3_col174;
                let blake_g_output_limb_4_col175 = blake_g_output_tmp_92ff8_133[2].low().as_m31();
                *row[175] = blake_g_output_limb_4_col175;
                let blake_g_output_limb_5_col176 = blake_g_output_tmp_92ff8_133[2].high().as_m31();
                *row[176] = blake_g_output_limb_5_col176;
                let blake_g_output_limb_6_col177 = blake_g_output_tmp_92ff8_133[3].low().as_m31();
                *row[177] = blake_g_output_limb_6_col177;
                let blake_g_output_limb_7_col178 = blake_g_output_tmp_92ff8_133[3].high().as_m31();
                *row[178] = blake_g_output_limb_7_col178;
                *lookup_data.blake_g_3 = [
                    input_limb_8_col8,
                    input_limb_9_col9,
                    input_limb_16_col16,
                    input_limb_17_col17,
                    input_limb_24_col24,
                    input_limb_25_col25,
                    input_limb_32_col32,
                    input_limb_33_col33,
                    low_16_bits_col87,
                    high_16_bits_col88,
                    low_16_bits_col93,
                    high_16_bits_col94,
                    blake_g_output_limb_0_col171,
                    blake_g_output_limb_1_col172,
                    blake_g_output_limb_2_col173,
                    blake_g_output_limb_3_col174,
                    blake_g_output_limb_4_col175,
                    blake_g_output_limb_5_col176,
                    blake_g_output_limb_6_col177,
                    blake_g_output_limb_7_col178,
                ];
                let blake_g_inputs_4 = [
                    blake_g_output_tmp_92ff8_130[0],
                    blake_g_output_tmp_92ff8_131[1],
                    blake_g_output_tmp_92ff8_132[2],
                    blake_g_output_tmp_92ff8_133[3],
                    expected_word_tmp_92ff8_69,
                    expected_word_tmp_92ff8_77,
                ];
                let blake_g_output_tmp_92ff8_134 = BlakeG::deduce_output([
                    blake_g_output_tmp_92ff8_130[0],
                    blake_g_output_tmp_92ff8_131[1],
                    blake_g_output_tmp_92ff8_132[2],
                    blake_g_output_tmp_92ff8_133[3],
                    expected_word_tmp_92ff8_69,
                    expected_word_tmp_92ff8_77,
                ]);
                let blake_g_output_limb_0_col179 = blake_g_output_tmp_92ff8_134[0].low().as_m31();
                *row[179] = blake_g_output_limb_0_col179;
                let blake_g_output_limb_1_col180 = blake_g_output_tmp_92ff8_134[0].high().as_m31();
                *row[180] = blake_g_output_limb_1_col180;
                let blake_g_output_limb_2_col181 = blake_g_output_tmp_92ff8_134[1].low().as_m31();
                *row[181] = blake_g_output_limb_2_col181;
                let blake_g_output_limb_3_col182 = blake_g_output_tmp_92ff8_134[1].high().as_m31();
                *row[182] = blake_g_output_limb_3_col182;
                let blake_g_output_limb_4_col183 = blake_g_output_tmp_92ff8_134[2].low().as_m31();
                *row[183] = blake_g_output_limb_4_col183;
                let blake_g_output_limb_5_col184 = blake_g_output_tmp_92ff8_134[2].high().as_m31();
                *row[184] = blake_g_output_limb_5_col184;
                let blake_g_output_limb_6_col185 = blake_g_output_tmp_92ff8_134[3].low().as_m31();
                *row[185] = blake_g_output_limb_6_col185;
                let blake_g_output_limb_7_col186 = blake_g_output_tmp_92ff8_134[3].high().as_m31();
                *row[186] = blake_g_output_limb_7_col186;
                *lookup_data.blake_g_4 = [
                    blake_g_output_limb_0_col147,
                    blake_g_output_limb_1_col148,
                    blake_g_output_limb_2_col157,
                    blake_g_output_limb_3_col158,
                    blake_g_output_limb_4_col167,
                    blake_g_output_limb_5_col168,
                    blake_g_output_limb_6_col177,
                    blake_g_output_limb_7_col178,
                    low_16_bits_col99,
                    high_16_bits_col100,
                    low_16_bits_col105,
                    high_16_bits_col106,
                    blake_g_output_limb_0_col179,
                    blake_g_output_limb_1_col180,
                    blake_g_output_limb_2_col181,
                    blake_g_output_limb_3_col182,
                    blake_g_output_limb_4_col183,
                    blake_g_output_limb_5_col184,
                    blake_g_output_limb_6_col185,
                    blake_g_output_limb_7_col186,
                ];
                let blake_g_inputs_5 = [
                    blake_g_output_tmp_92ff8_131[0],
                    blake_g_output_tmp_92ff8_132[1],
                    blake_g_output_tmp_92ff8_133[2],
                    blake_g_output_tmp_92ff8_130[3],
                    expected_word_tmp_92ff8_85,
                    expected_word_tmp_92ff8_93,
                ];
                let blake_g_output_tmp_92ff8_135 = BlakeG::deduce_output([
                    blake_g_output_tmp_92ff8_131[0],
                    blake_g_output_tmp_92ff8_132[1],
                    blake_g_output_tmp_92ff8_133[2],
                    blake_g_output_tmp_92ff8_130[3],
                    expected_word_tmp_92ff8_85,
                    expected_word_tmp_92ff8_93,
                ]);
                let blake_g_output_limb_0_col187 = blake_g_output_tmp_92ff8_135[0].low().as_m31();
                *row[187] = blake_g_output_limb_0_col187;
                let blake_g_output_limb_1_col188 = blake_g_output_tmp_92ff8_135[0].high().as_m31();
                *row[188] = blake_g_output_limb_1_col188;
                let blake_g_output_limb_2_col189 = blake_g_output_tmp_92ff8_135[1].low().as_m31();
                *row[189] = blake_g_output_limb_2_col189;
                let blake_g_output_limb_3_col190 = blake_g_output_tmp_92ff8_135[1].high().as_m31();
                *row[190] = blake_g_output_limb_3_col190;
                let blake_g_output_limb_4_col191 = blake_g_output_tmp_92ff8_135[2].low().as_m31();
                *row[191] = blake_g_output_limb_4_col191;
                let blake_g_output_limb_5_col192 = blake_g_output_tmp_92ff8_135[2].high().as_m31();
                *row[192] = blake_g_output_limb_5_col192;
                let blake_g_output_limb_6_col193 = blake_g_output_tmp_92ff8_135[3].low().as_m31();
                *row[193] = blake_g_output_limb_6_col193;
                let blake_g_output_limb_7_col194 = blake_g_output_tmp_92ff8_135[3].high().as_m31();
                *row[194] = blake_g_output_limb_7_col194;
                *lookup_data.blake_g_5 = [
                    blake_g_output_limb_0_col155,
                    blake_g_output_limb_1_col156,
                    blake_g_output_limb_2_col165,
                    blake_g_output_limb_3_col166,
                    blake_g_output_limb_4_col175,
                    blake_g_output_limb_5_col176,
                    blake_g_output_limb_6_col153,
                    blake_g_output_limb_7_col154,
                    low_16_bits_col111,
                    high_16_bits_col112,
                    low_16_bits_col117,
                    high_16_bits_col118,
                    blake_g_output_limb_0_col187,
                    blake_g_output_limb_1_col188,
                    blake_g_output_limb_2_col189,
                    blake_g_output_limb_3_col190,
                    blake_g_output_limb_4_col191,
                    blake_g_output_limb_5_col192,
                    blake_g_output_limb_6_col193,
                    blake_g_output_limb_7_col194,
                ];
                let blake_g_inputs_6 = [
                    blake_g_output_tmp_92ff8_132[0],
                    blake_g_output_tmp_92ff8_133[1],
                    blake_g_output_tmp_92ff8_130[2],
                    blake_g_output_tmp_92ff8_131[3],
                    expected_word_tmp_92ff8_101,
                    expected_word_tmp_92ff8_109,
                ];
                let blake_g_output_tmp_92ff8_136 = BlakeG::deduce_output([
                    blake_g_output_tmp_92ff8_132[0],
                    blake_g_output_tmp_92ff8_133[1],
                    blake_g_output_tmp_92ff8_130[2],
                    blake_g_output_tmp_92ff8_131[3],
                    expected_word_tmp_92ff8_101,
                    expected_word_tmp_92ff8_109,
                ]);
                let blake_g_output_limb_0_col195 = blake_g_output_tmp_92ff8_136[0].low().as_m31();
                *row[195] = blake_g_output_limb_0_col195;
                let blake_g_output_limb_1_col196 = blake_g_output_tmp_92ff8_136[0].high().as_m31();
                *row[196] = blake_g_output_limb_1_col196;
                let blake_g_output_limb_2_col197 = blake_g_output_tmp_92ff8_136[1].low().as_m31();
                *row[197] = blake_g_output_limb_2_col197;
                let blake_g_output_limb_3_col198 = blake_g_output_tmp_92ff8_136[1].high().as_m31();
                *row[198] = blake_g_output_limb_3_col198;
                let blake_g_output_limb_4_col199 = blake_g_output_tmp_92ff8_136[2].low().as_m31();
                *row[199] = blake_g_output_limb_4_col199;
                let blake_g_output_limb_5_col200 = blake_g_output_tmp_92ff8_136[2].high().as_m31();
                *row[200] = blake_g_output_limb_5_col200;
                let blake_g_output_limb_6_col201 = blake_g_output_tmp_92ff8_136[3].low().as_m31();
                *row[201] = blake_g_output_limb_6_col201;
                let blake_g_output_limb_7_col202 = blake_g_output_tmp_92ff8_136[3].high().as_m31();
                *row[202] = blake_g_output_limb_7_col202;
                *lookup_data.blake_g_6 = [
                    blake_g_output_limb_0_col163,
                    blake_g_output_limb_1_col164,
                    blake_g_output_limb_2_col173,
                    blake_g_output_limb_3_col174,
                    blake_g_output_limb_4_col151,
                    blake_g_output_limb_5_col152,
                    blake_g_output_limb_6_col161,
                    blake_g_output_limb_7_col162,
                    low_16_bits_col123,
                    high_16_bits_col124,
                    low_16_bits_col129,
                    high_16_bits_col130,
                    blake_g_output_limb_0_col195,
                    blake_g_output_limb_1_col196,
                    blake_g_output_limb_2_col197,
                    blake_g_output_limb_3_col198,
                    blake_g_output_limb_4_col199,
                    blake_g_output_limb_5_col200,
                    blake_g_output_limb_6_col201,
                    blake_g_output_limb_7_col202,
                ];
                let blake_g_inputs_7 = [
                    blake_g_output_tmp_92ff8_133[0],
                    blake_g_output_tmp_92ff8_130[1],
                    blake_g_output_tmp_92ff8_131[2],
                    blake_g_output_tmp_92ff8_132[3],
                    expected_word_tmp_92ff8_117,
                    expected_word_tmp_92ff8_125,
                ];
                let blake_g_output_tmp_92ff8_137 = BlakeG::deduce_output([
                    blake_g_output_tmp_92ff8_133[0],
                    blake_g_output_tmp_92ff8_130[1],
                    blake_g_output_tmp_92ff8_131[2],
                    blake_g_output_tmp_92ff8_132[3],
                    expected_word_tmp_92ff8_117,
                    expected_word_tmp_92ff8_125,
                ]);
                let blake_g_output_limb_0_col203 = blake_g_output_tmp_92ff8_137[0].low().as_m31();
                *row[203] = blake_g_output_limb_0_col203;
                let blake_g_output_limb_1_col204 = blake_g_output_tmp_92ff8_137[0].high().as_m31();
                *row[204] = blake_g_output_limb_1_col204;
                let blake_g_output_limb_2_col205 = blake_g_output_tmp_92ff8_137[1].low().as_m31();
                *row[205] = blake_g_output_limb_2_col205;
                let blake_g_output_limb_3_col206 = blake_g_output_tmp_92ff8_137[1].high().as_m31();
                *row[206] = blake_g_output_limb_3_col206;
                let blake_g_output_limb_4_col207 = blake_g_output_tmp_92ff8_137[2].low().as_m31();
                *row[207] = blake_g_output_limb_4_col207;
                let blake_g_output_limb_5_col208 = blake_g_output_tmp_92ff8_137[2].high().as_m31();
                *row[208] = blake_g_output_limb_5_col208;
                let blake_g_output_limb_6_col209 = blake_g_output_tmp_92ff8_137[3].low().as_m31();
                *row[209] = blake_g_output_limb_6_col209;
                let blake_g_output_limb_7_col210 = blake_g_output_tmp_92ff8_137[3].high().as_m31();
                *row[210] = blake_g_output_limb_7_col210;
                *lookup_data.blake_g_7 = [
                    blake_g_output_limb_0_col171,
                    blake_g_output_limb_1_col172,
                    blake_g_output_limb_2_col149,
                    blake_g_output_limb_3_col150,
                    blake_g_output_limb_4_col159,
                    blake_g_output_limb_5_col160,
                    blake_g_output_limb_6_col169,
                    blake_g_output_limb_7_col170,
                    low_16_bits_col135,
                    high_16_bits_col136,
                    low_16_bits_col141,
                    high_16_bits_col142,
                    blake_g_output_limb_0_col203,
                    blake_g_output_limb_1_col204,
                    blake_g_output_limb_2_col205,
                    blake_g_output_limb_3_col206,
                    blake_g_output_limb_4_col207,
                    blake_g_output_limb_5_col208,
                    blake_g_output_limb_6_col209,
                    blake_g_output_limb_7_col210,
                ];
                *lookup_data.blake_round_0 = [
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
                ];
                *lookup_data.blake_round_1 = [
                    input_limb_0_col0,
                    ((input_limb_1_col1) + (M31_1)),
                    blake_g_output_limb_0_col179,
                    blake_g_output_limb_1_col180,
                    blake_g_output_limb_0_col187,
                    blake_g_output_limb_1_col188,
                    blake_g_output_limb_0_col195,
                    blake_g_output_limb_1_col196,
                    blake_g_output_limb_0_col203,
                    blake_g_output_limb_1_col204,
                    blake_g_output_limb_2_col205,
                    blake_g_output_limb_3_col206,
                    blake_g_output_limb_2_col181,
                    blake_g_output_limb_3_col182,
                    blake_g_output_limb_2_col189,
                    blake_g_output_limb_3_col190,
                    blake_g_output_limb_2_col197,
                    blake_g_output_limb_3_col198,
                    blake_g_output_limb_4_col199,
                    blake_g_output_limb_5_col200,
                    blake_g_output_limb_4_col207,
                    blake_g_output_limb_5_col208,
                    blake_g_output_limb_4_col183,
                    blake_g_output_limb_5_col184,
                    blake_g_output_limb_4_col191,
                    blake_g_output_limb_5_col192,
                    blake_g_output_limb_6_col193,
                    blake_g_output_limb_7_col194,
                    blake_g_output_limb_6_col201,
                    blake_g_output_limb_7_col202,
                    blake_g_output_limb_6_col209,
                    blake_g_output_limb_7_col210,
                    blake_g_output_limb_6_col185,
                    blake_g_output_limb_7_col186,
                    input_limb_34_col34,
                ];

                *row[211] = enabler.packed_at(row_index);

                // Add sub-components inputs.
                blake_round_sigma_state.add_inputs(&blake_round_sigma_inputs_0);
                range_check_7_2_5_state.add_inputs(&range_check_7_2_5_inputs_0);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_0);
                memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_0);
                range_check_7_2_5_state.add_inputs(&range_check_7_2_5_inputs_1);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_1);
                memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_1);
                range_check_7_2_5_state.add_inputs(&range_check_7_2_5_inputs_2);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_2);
                memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_2);
                range_check_7_2_5_state.add_inputs(&range_check_7_2_5_inputs_3);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_3);
                memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_3);
                range_check_7_2_5_state.add_inputs(&range_check_7_2_5_inputs_4);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_4);
                memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_4);
                range_check_7_2_5_state.add_inputs(&range_check_7_2_5_inputs_5);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_5);
                memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_5);
                range_check_7_2_5_state.add_inputs(&range_check_7_2_5_inputs_6);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_6);
                memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_6);
                range_check_7_2_5_state.add_inputs(&range_check_7_2_5_inputs_7);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_7);
                memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_7);
                range_check_7_2_5_state.add_inputs(&range_check_7_2_5_inputs_8);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_8);
                memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_8);
                range_check_7_2_5_state.add_inputs(&range_check_7_2_5_inputs_9);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_9);
                memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_9);
                range_check_7_2_5_state.add_inputs(&range_check_7_2_5_inputs_10);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_10);
                memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_10);
                range_check_7_2_5_state.add_inputs(&range_check_7_2_5_inputs_11);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_11);
                memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_11);
                range_check_7_2_5_state.add_inputs(&range_check_7_2_5_inputs_12);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_12);
                memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_12);
                range_check_7_2_5_state.add_inputs(&range_check_7_2_5_inputs_13);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_13);
                memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_13);
                range_check_7_2_5_state.add_inputs(&range_check_7_2_5_inputs_14);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_14);
                memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_14);
                range_check_7_2_5_state.add_inputs(&range_check_7_2_5_inputs_15);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_15);
                memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_15);
                *blake_g_input.inputs[0] = blake_g_inputs_0;
                *blake_g_input.inputs[1] = blake_g_inputs_1;
                *blake_g_input.inputs[2] = blake_g_inputs_2;
                *blake_g_input.inputs[3] = blake_g_inputs_3;
                *blake_g_input.inputs[4] = blake_g_inputs_4;
                *blake_g_input.inputs[5] = blake_g_inputs_5;
                *blake_g_input.inputs[6] = blake_g_inputs_6;
                *blake_g_input.inputs[7] = blake_g_inputs_7;
            },
        );

    blake_g_state.add_packed_inputs(&blake_g_inputs.inputs[0]);
    blake_g_state.add_packed_inputs(&blake_g_inputs.inputs[1]);
    blake_g_state.add_packed_inputs(&blake_g_inputs.inputs[2]);
    blake_g_state.add_packed_inputs(&blake_g_inputs.inputs[3]);
    blake_g_state.add_packed_inputs(&blake_g_inputs.inputs[4]);
    blake_g_state.add_packed_inputs(&blake_g_inputs.inputs[5]);
    blake_g_state.add_packed_inputs(&blake_g_inputs.inputs[6]);
    blake_g_state.add_packed_inputs(&blake_g_inputs.inputs[7]);

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    blake_g_0: Vec<[PackedM31; 20]>,
    blake_g_1: Vec<[PackedM31; 20]>,
    blake_g_2: Vec<[PackedM31; 20]>,
    blake_g_3: Vec<[PackedM31; 20]>,
    blake_g_4: Vec<[PackedM31; 20]>,
    blake_g_5: Vec<[PackedM31; 20]>,
    blake_g_6: Vec<[PackedM31; 20]>,
    blake_g_7: Vec<[PackedM31; 20]>,
    blake_round_0: Vec<[PackedM31; 35]>,
    blake_round_1: Vec<[PackedM31; 35]>,
    blake_round_sigma_0: Vec<[PackedM31; 17]>,
    memory_address_to_id_0: Vec<[PackedM31; 2]>,
    memory_address_to_id_1: Vec<[PackedM31; 2]>,
    memory_address_to_id_2: Vec<[PackedM31; 2]>,
    memory_address_to_id_3: Vec<[PackedM31; 2]>,
    memory_address_to_id_4: Vec<[PackedM31; 2]>,
    memory_address_to_id_5: Vec<[PackedM31; 2]>,
    memory_address_to_id_6: Vec<[PackedM31; 2]>,
    memory_address_to_id_7: Vec<[PackedM31; 2]>,
    memory_address_to_id_8: Vec<[PackedM31; 2]>,
    memory_address_to_id_9: Vec<[PackedM31; 2]>,
    memory_address_to_id_10: Vec<[PackedM31; 2]>,
    memory_address_to_id_11: Vec<[PackedM31; 2]>,
    memory_address_to_id_12: Vec<[PackedM31; 2]>,
    memory_address_to_id_13: Vec<[PackedM31; 2]>,
    memory_address_to_id_14: Vec<[PackedM31; 2]>,
    memory_address_to_id_15: Vec<[PackedM31; 2]>,
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
    memory_id_to_big_1: Vec<[PackedM31; 29]>,
    memory_id_to_big_2: Vec<[PackedM31; 29]>,
    memory_id_to_big_3: Vec<[PackedM31; 29]>,
    memory_id_to_big_4: Vec<[PackedM31; 29]>,
    memory_id_to_big_5: Vec<[PackedM31; 29]>,
    memory_id_to_big_6: Vec<[PackedM31; 29]>,
    memory_id_to_big_7: Vec<[PackedM31; 29]>,
    memory_id_to_big_8: Vec<[PackedM31; 29]>,
    memory_id_to_big_9: Vec<[PackedM31; 29]>,
    memory_id_to_big_10: Vec<[PackedM31; 29]>,
    memory_id_to_big_11: Vec<[PackedM31; 29]>,
    memory_id_to_big_12: Vec<[PackedM31; 29]>,
    memory_id_to_big_13: Vec<[PackedM31; 29]>,
    memory_id_to_big_14: Vec<[PackedM31; 29]>,
    memory_id_to_big_15: Vec<[PackedM31; 29]>,
    range_check_7_2_5_0: Vec<[PackedM31; 3]>,
    range_check_7_2_5_1: Vec<[PackedM31; 3]>,
    range_check_7_2_5_2: Vec<[PackedM31; 3]>,
    range_check_7_2_5_3: Vec<[PackedM31; 3]>,
    range_check_7_2_5_4: Vec<[PackedM31; 3]>,
    range_check_7_2_5_5: Vec<[PackedM31; 3]>,
    range_check_7_2_5_6: Vec<[PackedM31; 3]>,
    range_check_7_2_5_7: Vec<[PackedM31; 3]>,
    range_check_7_2_5_8: Vec<[PackedM31; 3]>,
    range_check_7_2_5_9: Vec<[PackedM31; 3]>,
    range_check_7_2_5_10: Vec<[PackedM31; 3]>,
    range_check_7_2_5_11: Vec<[PackedM31; 3]>,
    range_check_7_2_5_12: Vec<[PackedM31; 3]>,
    range_check_7_2_5_13: Vec<[PackedM31; 3]>,
    range_check_7_2_5_14: Vec<[PackedM31; 3]>,
    range_check_7_2_5_15: Vec<[PackedM31; 3]>,
}

pub struct InteractionClaimGenerator {
    n_rows: usize,
    log_size: u32,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        blake_g: &relations::BlakeG,
        blake_round: &relations::BlakeRound,
        blake_round_sigma: &relations::BlakeRoundSigma,
        memory_address_to_id: &relations::MemoryAddressToId,
        memory_id_to_big: &relations::MemoryIdToBig,
        range_check_7_2_5: &relations::RangeCheck_7_2_5,
    ) -> InteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);
        let enabler = Enabler::new(self.n_rows);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.blake_round_sigma_0,
            &self.lookup_data.range_check_7_2_5_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = blake_round_sigma.combine(values0);
            let denom1: PackedQM31 = range_check_7_2_5.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_address_to_id_0,
            &self.lookup_data.memory_id_to_big_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_address_to_id.combine(values0);
            let denom1: PackedQM31 = memory_id_to_big.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_7_2_5_1,
            &self.lookup_data.memory_address_to_id_1,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_7_2_5.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_id_to_big_1,
            &self.lookup_data.range_check_7_2_5_2,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = range_check_7_2_5.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_address_to_id_2,
            &self.lookup_data.memory_id_to_big_2,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_address_to_id.combine(values0);
            let denom1: PackedQM31 = memory_id_to_big.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_7_2_5_3,
            &self.lookup_data.memory_address_to_id_3,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_7_2_5.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_id_to_big_3,
            &self.lookup_data.range_check_7_2_5_4,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = range_check_7_2_5.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_address_to_id_4,
            &self.lookup_data.memory_id_to_big_4,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_address_to_id.combine(values0);
            let denom1: PackedQM31 = memory_id_to_big.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_7_2_5_5,
            &self.lookup_data.memory_address_to_id_5,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_7_2_5.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_id_to_big_5,
            &self.lookup_data.range_check_7_2_5_6,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = range_check_7_2_5.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_address_to_id_6,
            &self.lookup_data.memory_id_to_big_6,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_address_to_id.combine(values0);
            let denom1: PackedQM31 = memory_id_to_big.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_7_2_5_7,
            &self.lookup_data.memory_address_to_id_7,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_7_2_5.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_id_to_big_7,
            &self.lookup_data.range_check_7_2_5_8,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = range_check_7_2_5.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_address_to_id_8,
            &self.lookup_data.memory_id_to_big_8,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_address_to_id.combine(values0);
            let denom1: PackedQM31 = memory_id_to_big.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_7_2_5_9,
            &self.lookup_data.memory_address_to_id_9,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_7_2_5.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_id_to_big_9,
            &self.lookup_data.range_check_7_2_5_10,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = range_check_7_2_5.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_address_to_id_10,
            &self.lookup_data.memory_id_to_big_10,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_address_to_id.combine(values0);
            let denom1: PackedQM31 = memory_id_to_big.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_7_2_5_11,
            &self.lookup_data.memory_address_to_id_11,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_7_2_5.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_id_to_big_11,
            &self.lookup_data.range_check_7_2_5_12,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = range_check_7_2_5.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_address_to_id_12,
            &self.lookup_data.memory_id_to_big_12,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_address_to_id.combine(values0);
            let denom1: PackedQM31 = memory_id_to_big.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_7_2_5_13,
            &self.lookup_data.memory_address_to_id_13,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_7_2_5.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_id_to_big_13,
            &self.lookup_data.range_check_7_2_5_14,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = range_check_7_2_5.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_address_to_id_14,
            &self.lookup_data.memory_id_to_big_14,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_address_to_id.combine(values0);
            let denom1: PackedQM31 = memory_id_to_big.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_7_2_5_15,
            &self.lookup_data.memory_address_to_id_15,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_7_2_5.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_id_to_big_15,
            &self.lookup_data.blake_g_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = blake_g.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in
            zip(&self.lookup_data.blake_g_1, &self.lookup_data.blake_g_2).enumerate()
        {
            let denom0: PackedQM31 = blake_g.combine(values0);
            let denom1: PackedQM31 = blake_g.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in
            zip(&self.lookup_data.blake_g_3, &self.lookup_data.blake_g_4).enumerate()
        {
            let denom0: PackedQM31 = blake_g.combine(values0);
            let denom1: PackedQM31 = blake_g.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in
            zip(&self.lookup_data.blake_g_5, &self.lookup_data.blake_g_6).enumerate()
        {
            let denom0: PackedQM31 = blake_g.combine(values0);
            let denom1: PackedQM31 = blake_g.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in
            zip(&self.lookup_data.blake_g_7, &self.lookup_data.blake_round_0).enumerate()
        {
            let denom0: PackedQM31 = blake_g.combine(values0);
            let denom1: PackedQM31 = blake_round.combine(values1);
            col_gen.write_frac(i, denom0 * enabler.packed_at(i) + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        for (i, values) in self.lookup_data.blake_round_1.iter().enumerate() {
            let denom = blake_round.combine(values);
            col_gen.write_frac(i, PackedQM31::from(-enabler.packed_at(i)), denom);
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
