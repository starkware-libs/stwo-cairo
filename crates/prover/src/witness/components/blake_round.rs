#![allow(unused_parens)]
use cairo_air::components::blake_round::{Claim, InteractionClaim, N_TRACE_COLUMNS};
use stwo_cairo_adapter::memory::Memory;

use crate::witness::components::{
    blake_g, blake_round_sigma, memory_address_to_id, memory_id_to_big, range_check_7_2_5,
};
use crate::witness::prelude::*;

pub type InputType = (M31, M31, ([UInt32; 16], M31));
pub type PackedInputType = (PackedM31, PackedM31, ([PackedUInt32; 16], PackedM31));

pub struct ClaimGenerator {
    state: BlakeRound,
    pub packed_inputs: Mutex<Vec<PackedInputType>>,
    pub remainder_inputs: Mutex<Vec<InputType>>,
}

impl ClaimGenerator {
    pub fn new(memory: Arc<Memory>) -> Self {
        let state = BlakeRound::new(memory);
        Self { packed_inputs: Mutex::new(vec![]), remainder_inputs: Mutex::new(vec![]), state }
    }

    pub fn write_trace(
        self,
        blake_round_sigma_state: &blake_round_sigma::ClaimGenerator,
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
        range_check_7_2_5_state: &range_check_7_2_5::ClaimGenerator,
        blake_g_state: &blake_g::ClaimGenerator,
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
            blake_round_sigma_state,
            memory_address_to_id_state,
            memory_id_to_big_state,
            range_check_7_2_5_state,
            blake_g_state,
        );
        for inputs in sub_component_inputs.blake_round_sigma {
            add_inputs(blake_round_sigma_state, &inputs, n_active_rows, 0);
        }
        for inputs in sub_component_inputs.range_check_7_2_5 {
            add_inputs(range_check_7_2_5_state, &inputs, n_active_rows, 0);
        }
        for inputs in sub_component_inputs.memory_address_to_id {
            add_inputs(memory_address_to_id_state, &inputs, n_active_rows, 0);
        }
        for inputs in sub_component_inputs.memory_id_to_big {
            add_inputs(memory_id_to_big_state, &inputs, n_active_rows, 0);
        }
        for inputs in sub_component_inputs.blake_g {
            add_inputs(blake_g_state, &inputs, n_active_rows, 0);
        }

        (trace, Claim { log_size }, InteractionClaimGenerator { log_size, lookup_data })
    }

    pub fn deduce_output(
        &self,
        input: (PackedM31, PackedM31, ([PackedUInt32; 16], PackedM31)),
    ) -> (PackedM31, PackedM31, ([PackedUInt32; 16], PackedM31)) {
        self.state.deduce_output(input.0, input.1, input.2)
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
    blake_round_sigma: [Vec<blake_round_sigma::PackedInputType>; 1],
    range_check_7_2_5: [Vec<range_check_7_2_5::PackedInputType>; 16],
    memory_address_to_id: [Vec<memory_address_to_id::PackedInputType>; 16],
    memory_id_to_big: [Vec<memory_id_to_big::PackedInputType>; 16],
    blake_g: [Vec<blake_g::PackedInputType>; 8],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    n_rows: usize,
    blake_round_sigma_state: &blake_round_sigma::ClaimGenerator,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
    range_check_7_2_5_state: &range_check_7_2_5::ClaimGenerator,
    blake_g_state: &blake_g::ClaimGenerator,
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

    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_1139985212 = PackedM31::broadcast(M31::from(1139985212));
    let M31_128 = PackedM31::broadcast(M31::from(128));
    let M31_1444891767 = PackedM31::broadcast(M31::from(1444891767));
    let M31_1662111297 = PackedM31::broadcast(M31::from(1662111297));
    let M31_1805967942 = PackedM31::broadcast(M31::from(1805967942));
    let M31_2048 = PackedM31::broadcast(M31::from(2048));
    let M31_371240602 = PackedM31::broadcast(M31::from(371240602));
    let M31_4 = PackedM31::broadcast(M31::from(4));
    let M31_40528774 = PackedM31::broadcast(M31::from(40528774));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let UInt16_2 = PackedUInt16::broadcast(UInt16::from(2));
    let UInt16_7 = PackedUInt16::broadcast(UInt16::from(7));
    let UInt16_9 = PackedUInt16::broadcast(UInt16::from(9));
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
            |(row_index, (row, lookup_data, sub_component_inputs, blake_round_input))| {
                let enabler_col0 = enabler_col.packed_at(row_index);
                *row[0] = enabler_col0;
                let input_limb_0_col1 = blake_round_input.0;
                *row[1] = input_limb_0_col1;
                let input_limb_1_col2 = blake_round_input.1;
                *row[2] = input_limb_1_col2;
                let input_limb_2_col3 = blake_round_input.2.0[0].low().as_m31();
                *row[3] = input_limb_2_col3;
                let input_limb_3_col4 = blake_round_input.2.0[0].high().as_m31();
                *row[4] = input_limb_3_col4;
                let input_limb_4_col5 = blake_round_input.2.0[1].low().as_m31();
                *row[5] = input_limb_4_col5;
                let input_limb_5_col6 = blake_round_input.2.0[1].high().as_m31();
                *row[6] = input_limb_5_col6;
                let input_limb_6_col7 = blake_round_input.2.0[2].low().as_m31();
                *row[7] = input_limb_6_col7;
                let input_limb_7_col8 = blake_round_input.2.0[2].high().as_m31();
                *row[8] = input_limb_7_col8;
                let input_limb_8_col9 = blake_round_input.2.0[3].low().as_m31();
                *row[9] = input_limb_8_col9;
                let input_limb_9_col10 = blake_round_input.2.0[3].high().as_m31();
                *row[10] = input_limb_9_col10;
                let input_limb_10_col11 = blake_round_input.2.0[4].low().as_m31();
                *row[11] = input_limb_10_col11;
                let input_limb_11_col12 = blake_round_input.2.0[4].high().as_m31();
                *row[12] = input_limb_11_col12;
                let input_limb_12_col13 = blake_round_input.2.0[5].low().as_m31();
                *row[13] = input_limb_12_col13;
                let input_limb_13_col14 = blake_round_input.2.0[5].high().as_m31();
                *row[14] = input_limb_13_col14;
                let input_limb_14_col15 = blake_round_input.2.0[6].low().as_m31();
                *row[15] = input_limb_14_col15;
                let input_limb_15_col16 = blake_round_input.2.0[6].high().as_m31();
                *row[16] = input_limb_15_col16;
                let input_limb_16_col17 = blake_round_input.2.0[7].low().as_m31();
                *row[17] = input_limb_16_col17;
                let input_limb_17_col18 = blake_round_input.2.0[7].high().as_m31();
                *row[18] = input_limb_17_col18;
                let input_limb_18_col19 = blake_round_input.2.0[8].low().as_m31();
                *row[19] = input_limb_18_col19;
                let input_limb_19_col20 = blake_round_input.2.0[8].high().as_m31();
                *row[20] = input_limb_19_col20;
                let input_limb_20_col21 = blake_round_input.2.0[9].low().as_m31();
                *row[21] = input_limb_20_col21;
                let input_limb_21_col22 = blake_round_input.2.0[9].high().as_m31();
                *row[22] = input_limb_21_col22;
                let input_limb_22_col23 = blake_round_input.2.0[10].low().as_m31();
                *row[23] = input_limb_22_col23;
                let input_limb_23_col24 = blake_round_input.2.0[10].high().as_m31();
                *row[24] = input_limb_23_col24;
                let input_limb_24_col25 = blake_round_input.2.0[11].low().as_m31();
                *row[25] = input_limb_24_col25;
                let input_limb_25_col26 = blake_round_input.2.0[11].high().as_m31();
                *row[26] = input_limb_25_col26;
                let input_limb_26_col27 = blake_round_input.2.0[12].low().as_m31();
                *row[27] = input_limb_26_col27;
                let input_limb_27_col28 = blake_round_input.2.0[12].high().as_m31();
                *row[28] = input_limb_27_col28;
                let input_limb_28_col29 = blake_round_input.2.0[13].low().as_m31();
                *row[29] = input_limb_28_col29;
                let input_limb_29_col30 = blake_round_input.2.0[13].high().as_m31();
                *row[30] = input_limb_29_col30;
                let input_limb_30_col31 = blake_round_input.2.0[14].low().as_m31();
                *row[31] = input_limb_30_col31;
                let input_limb_31_col32 = blake_round_input.2.0[14].high().as_m31();
                *row[32] = input_limb_31_col32;
                let input_limb_32_col33 = blake_round_input.2.0[15].low().as_m31();
                *row[33] = input_limb_32_col33;
                let input_limb_33_col34 = blake_round_input.2.0[15].high().as_m31();
                *row[34] = input_limb_33_col34;
                let input_limb_34_col35 = blake_round_input.2.1;
                *row[35] = input_limb_34_col35;
                *sub_component_inputs.blake_round_sigma[0] = [input_limb_1_col2];
                let blake_round_sigma_output_tmp_9aaf1_0 =
                    PackedBlakeRoundSigma::deduce_output(input_limb_1_col2);
                let blake_round_sigma_output_limb_0_col36 = blake_round_sigma_output_tmp_9aaf1_0[0];
                *row[36] = blake_round_sigma_output_limb_0_col36;
                let blake_round_sigma_output_limb_1_col37 = blake_round_sigma_output_tmp_9aaf1_0[1];
                *row[37] = blake_round_sigma_output_limb_1_col37;
                let blake_round_sigma_output_limb_2_col38 = blake_round_sigma_output_tmp_9aaf1_0[2];
                *row[38] = blake_round_sigma_output_limb_2_col38;
                let blake_round_sigma_output_limb_3_col39 = blake_round_sigma_output_tmp_9aaf1_0[3];
                *row[39] = blake_round_sigma_output_limb_3_col39;
                let blake_round_sigma_output_limb_4_col40 = blake_round_sigma_output_tmp_9aaf1_0[4];
                *row[40] = blake_round_sigma_output_limb_4_col40;
                let blake_round_sigma_output_limb_5_col41 = blake_round_sigma_output_tmp_9aaf1_0[5];
                *row[41] = blake_round_sigma_output_limb_5_col41;
                let blake_round_sigma_output_limb_6_col42 = blake_round_sigma_output_tmp_9aaf1_0[6];
                *row[42] = blake_round_sigma_output_limb_6_col42;
                let blake_round_sigma_output_limb_7_col43 = blake_round_sigma_output_tmp_9aaf1_0[7];
                *row[43] = blake_round_sigma_output_limb_7_col43;
                let blake_round_sigma_output_limb_8_col44 = blake_round_sigma_output_tmp_9aaf1_0[8];
                *row[44] = blake_round_sigma_output_limb_8_col44;
                let blake_round_sigma_output_limb_9_col45 = blake_round_sigma_output_tmp_9aaf1_0[9];
                *row[45] = blake_round_sigma_output_limb_9_col45;
                let blake_round_sigma_output_limb_10_col46 =
                    blake_round_sigma_output_tmp_9aaf1_0[10];
                *row[46] = blake_round_sigma_output_limb_10_col46;
                let blake_round_sigma_output_limb_11_col47 =
                    blake_round_sigma_output_tmp_9aaf1_0[11];
                *row[47] = blake_round_sigma_output_limb_11_col47;
                let blake_round_sigma_output_limb_12_col48 =
                    blake_round_sigma_output_tmp_9aaf1_0[12];
                *row[48] = blake_round_sigma_output_limb_12_col48;
                let blake_round_sigma_output_limb_13_col49 =
                    blake_round_sigma_output_tmp_9aaf1_0[13];
                *row[49] = blake_round_sigma_output_limb_13_col49;
                let blake_round_sigma_output_limb_14_col50 =
                    blake_round_sigma_output_tmp_9aaf1_0[14];
                *row[50] = blake_round_sigma_output_limb_14_col50;
                let blake_round_sigma_output_limb_15_col51 =
                    blake_round_sigma_output_tmp_9aaf1_0[15];
                *row[51] = blake_round_sigma_output_limb_15_col51;
                *lookup_data.blake_round_sigma_0 = [
                    M31_1805967942,
                    input_limb_1_col2,
                    blake_round_sigma_output_limb_0_col36,
                    blake_round_sigma_output_limb_1_col37,
                    blake_round_sigma_output_limb_2_col38,
                    blake_round_sigma_output_limb_3_col39,
                    blake_round_sigma_output_limb_4_col40,
                    blake_round_sigma_output_limb_5_col41,
                    blake_round_sigma_output_limb_6_col42,
                    blake_round_sigma_output_limb_7_col43,
                    blake_round_sigma_output_limb_8_col44,
                    blake_round_sigma_output_limb_9_col45,
                    blake_round_sigma_output_limb_10_col46,
                    blake_round_sigma_output_limb_11_col47,
                    blake_round_sigma_output_limb_12_col48,
                    blake_round_sigma_output_limb_13_col49,
                    blake_round_sigma_output_limb_14_col50,
                    blake_round_sigma_output_limb_15_col51,
                ];

                // Read U 32.

                let memory_address_to_id_value_tmp_9aaf1_1 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_0_col36)),
                    );
                let memory_id_to_big_value_tmp_9aaf1_2 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_9aaf1_1);
                let tmp_9aaf1_3 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9aaf1_2.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col52 = ((((memory_id_to_big_value_tmp_9aaf1_2.get_m31(1))
                    - ((tmp_9aaf1_3.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_9aaf1_2.get_m31(0)));
                *row[52] = low_16_bits_col52;
                let high_16_bits_col53 = ((((memory_id_to_big_value_tmp_9aaf1_2.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_9aaf1_2.get_m31(2)) * (M31_4)))
                    + (tmp_9aaf1_3.as_m31()));
                *row[53] = high_16_bits_col53;
                let expected_word_tmp_9aaf1_4 =
                    PackedUInt32::from_limbs([low_16_bits_col52, high_16_bits_col53]);

                // Verify U 32.

                let low_7_ms_bits_tmp_9aaf1_5 = ((expected_word_tmp_9aaf1_4.low()) >> (UInt16_9));
                let low_7_ms_bits_col54 = low_7_ms_bits_tmp_9aaf1_5.as_m31();
                *row[54] = low_7_ms_bits_col54;
                let high_14_ms_bits_tmp_9aaf1_6 =
                    ((expected_word_tmp_9aaf1_4.high()) >> (UInt16_2));
                let high_14_ms_bits_col55 = high_14_ms_bits_tmp_9aaf1_6.as_m31();
                *row[55] = high_14_ms_bits_col55;
                let high_2_ls_bits_tmp_9aaf1_7 =
                    ((high_16_bits_col53) - ((high_14_ms_bits_col55) * (M31_4)));
                let high_5_ms_bits_tmp_9aaf1_8 = ((high_14_ms_bits_tmp_9aaf1_6) >> (UInt16_9));
                let high_5_ms_bits_col56 = high_5_ms_bits_tmp_9aaf1_8.as_m31();
                *row[56] = high_5_ms_bits_col56;
                *sub_component_inputs.range_check_7_2_5[0] =
                    [low_7_ms_bits_col54, high_2_ls_bits_tmp_9aaf1_7, high_5_ms_bits_col56];
                *lookup_data.range_check_7_2_5_1 = [
                    M31_371240602,
                    low_7_ms_bits_col54,
                    high_2_ls_bits_tmp_9aaf1_7,
                    high_5_ms_bits_col56,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_9aaf1_9 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_0_col36)),
                    );
                let message_word_0_id_col57 = memory_address_to_id_value_tmp_9aaf1_9;
                *row[57] = message_word_0_id_col57;
                *sub_component_inputs.memory_address_to_id[0] =
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_0_col36));
                *lookup_data.memory_address_to_id_2 = [
                    M31_1444891767,
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_0_col36)),
                    message_word_0_id_col57,
                ];

                *sub_component_inputs.memory_id_to_big[0] = message_word_0_id_col57;
                *lookup_data.memory_id_to_big_3 = [
                    M31_1662111297,
                    message_word_0_id_col57,
                    ((low_16_bits_col52) - ((low_7_ms_bits_col54) * (M31_512))),
                    ((low_7_ms_bits_col54) + ((high_2_ls_bits_tmp_9aaf1_7) * (M31_128))),
                    ((high_14_ms_bits_col55) - ((high_5_ms_bits_col56) * (M31_512))),
                    high_5_ms_bits_col56,
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

                let read_u_32_output_tmp_9aaf1_11 = expected_word_tmp_9aaf1_4;

                // Read U 32.

                let memory_address_to_id_value_tmp_9aaf1_12 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_1_col37)),
                    );
                let memory_id_to_big_value_tmp_9aaf1_13 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_9aaf1_12);
                let tmp_9aaf1_14 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9aaf1_13.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col58 = ((((memory_id_to_big_value_tmp_9aaf1_13.get_m31(1))
                    - ((tmp_9aaf1_14.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_9aaf1_13.get_m31(0)));
                *row[58] = low_16_bits_col58;
                let high_16_bits_col59 = ((((memory_id_to_big_value_tmp_9aaf1_13.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_9aaf1_13.get_m31(2)) * (M31_4)))
                    + (tmp_9aaf1_14.as_m31()));
                *row[59] = high_16_bits_col59;
                let expected_word_tmp_9aaf1_15 =
                    PackedUInt32::from_limbs([low_16_bits_col58, high_16_bits_col59]);

                // Verify U 32.

                let low_7_ms_bits_tmp_9aaf1_16 = ((expected_word_tmp_9aaf1_15.low()) >> (UInt16_9));
                let low_7_ms_bits_col60 = low_7_ms_bits_tmp_9aaf1_16.as_m31();
                *row[60] = low_7_ms_bits_col60;
                let high_14_ms_bits_tmp_9aaf1_17 =
                    ((expected_word_tmp_9aaf1_15.high()) >> (UInt16_2));
                let high_14_ms_bits_col61 = high_14_ms_bits_tmp_9aaf1_17.as_m31();
                *row[61] = high_14_ms_bits_col61;
                let high_2_ls_bits_tmp_9aaf1_18 =
                    ((high_16_bits_col59) - ((high_14_ms_bits_col61) * (M31_4)));
                let high_5_ms_bits_tmp_9aaf1_19 = ((high_14_ms_bits_tmp_9aaf1_17) >> (UInt16_9));
                let high_5_ms_bits_col62 = high_5_ms_bits_tmp_9aaf1_19.as_m31();
                *row[62] = high_5_ms_bits_col62;
                *sub_component_inputs.range_check_7_2_5[1] =
                    [low_7_ms_bits_col60, high_2_ls_bits_tmp_9aaf1_18, high_5_ms_bits_col62];
                *lookup_data.range_check_7_2_5_4 = [
                    M31_371240602,
                    low_7_ms_bits_col60,
                    high_2_ls_bits_tmp_9aaf1_18,
                    high_5_ms_bits_col62,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_9aaf1_20 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_1_col37)),
                    );
                let message_word_1_id_col63 = memory_address_to_id_value_tmp_9aaf1_20;
                *row[63] = message_word_1_id_col63;
                *sub_component_inputs.memory_address_to_id[1] =
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_1_col37));
                *lookup_data.memory_address_to_id_5 = [
                    M31_1444891767,
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_1_col37)),
                    message_word_1_id_col63,
                ];

                *sub_component_inputs.memory_id_to_big[1] = message_word_1_id_col63;
                *lookup_data.memory_id_to_big_6 = [
                    M31_1662111297,
                    message_word_1_id_col63,
                    ((low_16_bits_col58) - ((low_7_ms_bits_col60) * (M31_512))),
                    ((low_7_ms_bits_col60) + ((high_2_ls_bits_tmp_9aaf1_18) * (M31_128))),
                    ((high_14_ms_bits_col61) - ((high_5_ms_bits_col62) * (M31_512))),
                    high_5_ms_bits_col62,
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

                let read_u_32_output_tmp_9aaf1_22 = expected_word_tmp_9aaf1_15;

                // Read U 32.

                let memory_address_to_id_value_tmp_9aaf1_23 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_2_col38)),
                    );
                let memory_id_to_big_value_tmp_9aaf1_24 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_9aaf1_23);
                let tmp_9aaf1_25 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9aaf1_24.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col64 = ((((memory_id_to_big_value_tmp_9aaf1_24.get_m31(1))
                    - ((tmp_9aaf1_25.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_9aaf1_24.get_m31(0)));
                *row[64] = low_16_bits_col64;
                let high_16_bits_col65 = ((((memory_id_to_big_value_tmp_9aaf1_24.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_9aaf1_24.get_m31(2)) * (M31_4)))
                    + (tmp_9aaf1_25.as_m31()));
                *row[65] = high_16_bits_col65;
                let expected_word_tmp_9aaf1_26 =
                    PackedUInt32::from_limbs([low_16_bits_col64, high_16_bits_col65]);

                // Verify U 32.

                let low_7_ms_bits_tmp_9aaf1_27 = ((expected_word_tmp_9aaf1_26.low()) >> (UInt16_9));
                let low_7_ms_bits_col66 = low_7_ms_bits_tmp_9aaf1_27.as_m31();
                *row[66] = low_7_ms_bits_col66;
                let high_14_ms_bits_tmp_9aaf1_28 =
                    ((expected_word_tmp_9aaf1_26.high()) >> (UInt16_2));
                let high_14_ms_bits_col67 = high_14_ms_bits_tmp_9aaf1_28.as_m31();
                *row[67] = high_14_ms_bits_col67;
                let high_2_ls_bits_tmp_9aaf1_29 =
                    ((high_16_bits_col65) - ((high_14_ms_bits_col67) * (M31_4)));
                let high_5_ms_bits_tmp_9aaf1_30 = ((high_14_ms_bits_tmp_9aaf1_28) >> (UInt16_9));
                let high_5_ms_bits_col68 = high_5_ms_bits_tmp_9aaf1_30.as_m31();
                *row[68] = high_5_ms_bits_col68;
                *sub_component_inputs.range_check_7_2_5[2] =
                    [low_7_ms_bits_col66, high_2_ls_bits_tmp_9aaf1_29, high_5_ms_bits_col68];
                *lookup_data.range_check_7_2_5_7 = [
                    M31_371240602,
                    low_7_ms_bits_col66,
                    high_2_ls_bits_tmp_9aaf1_29,
                    high_5_ms_bits_col68,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_9aaf1_31 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_2_col38)),
                    );
                let message_word_2_id_col69 = memory_address_to_id_value_tmp_9aaf1_31;
                *row[69] = message_word_2_id_col69;
                *sub_component_inputs.memory_address_to_id[2] =
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_2_col38));
                *lookup_data.memory_address_to_id_8 = [
                    M31_1444891767,
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_2_col38)),
                    message_word_2_id_col69,
                ];

                *sub_component_inputs.memory_id_to_big[2] = message_word_2_id_col69;
                *lookup_data.memory_id_to_big_9 = [
                    M31_1662111297,
                    message_word_2_id_col69,
                    ((low_16_bits_col64) - ((low_7_ms_bits_col66) * (M31_512))),
                    ((low_7_ms_bits_col66) + ((high_2_ls_bits_tmp_9aaf1_29) * (M31_128))),
                    ((high_14_ms_bits_col67) - ((high_5_ms_bits_col68) * (M31_512))),
                    high_5_ms_bits_col68,
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

                let read_u_32_output_tmp_9aaf1_33 = expected_word_tmp_9aaf1_26;

                // Read U 32.

                let memory_address_to_id_value_tmp_9aaf1_34 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_3_col39)),
                    );
                let memory_id_to_big_value_tmp_9aaf1_35 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_9aaf1_34);
                let tmp_9aaf1_36 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9aaf1_35.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col70 = ((((memory_id_to_big_value_tmp_9aaf1_35.get_m31(1))
                    - ((tmp_9aaf1_36.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_9aaf1_35.get_m31(0)));
                *row[70] = low_16_bits_col70;
                let high_16_bits_col71 = ((((memory_id_to_big_value_tmp_9aaf1_35.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_9aaf1_35.get_m31(2)) * (M31_4)))
                    + (tmp_9aaf1_36.as_m31()));
                *row[71] = high_16_bits_col71;
                let expected_word_tmp_9aaf1_37 =
                    PackedUInt32::from_limbs([low_16_bits_col70, high_16_bits_col71]);

                // Verify U 32.

                let low_7_ms_bits_tmp_9aaf1_38 = ((expected_word_tmp_9aaf1_37.low()) >> (UInt16_9));
                let low_7_ms_bits_col72 = low_7_ms_bits_tmp_9aaf1_38.as_m31();
                *row[72] = low_7_ms_bits_col72;
                let high_14_ms_bits_tmp_9aaf1_39 =
                    ((expected_word_tmp_9aaf1_37.high()) >> (UInt16_2));
                let high_14_ms_bits_col73 = high_14_ms_bits_tmp_9aaf1_39.as_m31();
                *row[73] = high_14_ms_bits_col73;
                let high_2_ls_bits_tmp_9aaf1_40 =
                    ((high_16_bits_col71) - ((high_14_ms_bits_col73) * (M31_4)));
                let high_5_ms_bits_tmp_9aaf1_41 = ((high_14_ms_bits_tmp_9aaf1_39) >> (UInt16_9));
                let high_5_ms_bits_col74 = high_5_ms_bits_tmp_9aaf1_41.as_m31();
                *row[74] = high_5_ms_bits_col74;
                *sub_component_inputs.range_check_7_2_5[3] =
                    [low_7_ms_bits_col72, high_2_ls_bits_tmp_9aaf1_40, high_5_ms_bits_col74];
                *lookup_data.range_check_7_2_5_10 = [
                    M31_371240602,
                    low_7_ms_bits_col72,
                    high_2_ls_bits_tmp_9aaf1_40,
                    high_5_ms_bits_col74,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_9aaf1_42 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_3_col39)),
                    );
                let message_word_3_id_col75 = memory_address_to_id_value_tmp_9aaf1_42;
                *row[75] = message_word_3_id_col75;
                *sub_component_inputs.memory_address_to_id[3] =
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_3_col39));
                *lookup_data.memory_address_to_id_11 = [
                    M31_1444891767,
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_3_col39)),
                    message_word_3_id_col75,
                ];

                *sub_component_inputs.memory_id_to_big[3] = message_word_3_id_col75;
                *lookup_data.memory_id_to_big_12 = [
                    M31_1662111297,
                    message_word_3_id_col75,
                    ((low_16_bits_col70) - ((low_7_ms_bits_col72) * (M31_512))),
                    ((low_7_ms_bits_col72) + ((high_2_ls_bits_tmp_9aaf1_40) * (M31_128))),
                    ((high_14_ms_bits_col73) - ((high_5_ms_bits_col74) * (M31_512))),
                    high_5_ms_bits_col74,
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

                let read_u_32_output_tmp_9aaf1_44 = expected_word_tmp_9aaf1_37;

                // Read U 32.

                let memory_address_to_id_value_tmp_9aaf1_45 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_4_col40)),
                    );
                let memory_id_to_big_value_tmp_9aaf1_46 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_9aaf1_45);
                let tmp_9aaf1_47 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9aaf1_46.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col76 = ((((memory_id_to_big_value_tmp_9aaf1_46.get_m31(1))
                    - ((tmp_9aaf1_47.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_9aaf1_46.get_m31(0)));
                *row[76] = low_16_bits_col76;
                let high_16_bits_col77 = ((((memory_id_to_big_value_tmp_9aaf1_46.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_9aaf1_46.get_m31(2)) * (M31_4)))
                    + (tmp_9aaf1_47.as_m31()));
                *row[77] = high_16_bits_col77;
                let expected_word_tmp_9aaf1_48 =
                    PackedUInt32::from_limbs([low_16_bits_col76, high_16_bits_col77]);

                // Verify U 32.

                let low_7_ms_bits_tmp_9aaf1_49 = ((expected_word_tmp_9aaf1_48.low()) >> (UInt16_9));
                let low_7_ms_bits_col78 = low_7_ms_bits_tmp_9aaf1_49.as_m31();
                *row[78] = low_7_ms_bits_col78;
                let high_14_ms_bits_tmp_9aaf1_50 =
                    ((expected_word_tmp_9aaf1_48.high()) >> (UInt16_2));
                let high_14_ms_bits_col79 = high_14_ms_bits_tmp_9aaf1_50.as_m31();
                *row[79] = high_14_ms_bits_col79;
                let high_2_ls_bits_tmp_9aaf1_51 =
                    ((high_16_bits_col77) - ((high_14_ms_bits_col79) * (M31_4)));
                let high_5_ms_bits_tmp_9aaf1_52 = ((high_14_ms_bits_tmp_9aaf1_50) >> (UInt16_9));
                let high_5_ms_bits_col80 = high_5_ms_bits_tmp_9aaf1_52.as_m31();
                *row[80] = high_5_ms_bits_col80;
                *sub_component_inputs.range_check_7_2_5[4] =
                    [low_7_ms_bits_col78, high_2_ls_bits_tmp_9aaf1_51, high_5_ms_bits_col80];
                *lookup_data.range_check_7_2_5_13 = [
                    M31_371240602,
                    low_7_ms_bits_col78,
                    high_2_ls_bits_tmp_9aaf1_51,
                    high_5_ms_bits_col80,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_9aaf1_53 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_4_col40)),
                    );
                let message_word_4_id_col81 = memory_address_to_id_value_tmp_9aaf1_53;
                *row[81] = message_word_4_id_col81;
                *sub_component_inputs.memory_address_to_id[4] =
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_4_col40));
                *lookup_data.memory_address_to_id_14 = [
                    M31_1444891767,
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_4_col40)),
                    message_word_4_id_col81,
                ];

                *sub_component_inputs.memory_id_to_big[4] = message_word_4_id_col81;
                *lookup_data.memory_id_to_big_15 = [
                    M31_1662111297,
                    message_word_4_id_col81,
                    ((low_16_bits_col76) - ((low_7_ms_bits_col78) * (M31_512))),
                    ((low_7_ms_bits_col78) + ((high_2_ls_bits_tmp_9aaf1_51) * (M31_128))),
                    ((high_14_ms_bits_col79) - ((high_5_ms_bits_col80) * (M31_512))),
                    high_5_ms_bits_col80,
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

                let read_u_32_output_tmp_9aaf1_55 = expected_word_tmp_9aaf1_48;

                // Read U 32.

                let memory_address_to_id_value_tmp_9aaf1_56 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_5_col41)),
                    );
                let memory_id_to_big_value_tmp_9aaf1_57 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_9aaf1_56);
                let tmp_9aaf1_58 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9aaf1_57.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col82 = ((((memory_id_to_big_value_tmp_9aaf1_57.get_m31(1))
                    - ((tmp_9aaf1_58.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_9aaf1_57.get_m31(0)));
                *row[82] = low_16_bits_col82;
                let high_16_bits_col83 = ((((memory_id_to_big_value_tmp_9aaf1_57.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_9aaf1_57.get_m31(2)) * (M31_4)))
                    + (tmp_9aaf1_58.as_m31()));
                *row[83] = high_16_bits_col83;
                let expected_word_tmp_9aaf1_59 =
                    PackedUInt32::from_limbs([low_16_bits_col82, high_16_bits_col83]);

                // Verify U 32.

                let low_7_ms_bits_tmp_9aaf1_60 = ((expected_word_tmp_9aaf1_59.low()) >> (UInt16_9));
                let low_7_ms_bits_col84 = low_7_ms_bits_tmp_9aaf1_60.as_m31();
                *row[84] = low_7_ms_bits_col84;
                let high_14_ms_bits_tmp_9aaf1_61 =
                    ((expected_word_tmp_9aaf1_59.high()) >> (UInt16_2));
                let high_14_ms_bits_col85 = high_14_ms_bits_tmp_9aaf1_61.as_m31();
                *row[85] = high_14_ms_bits_col85;
                let high_2_ls_bits_tmp_9aaf1_62 =
                    ((high_16_bits_col83) - ((high_14_ms_bits_col85) * (M31_4)));
                let high_5_ms_bits_tmp_9aaf1_63 = ((high_14_ms_bits_tmp_9aaf1_61) >> (UInt16_9));
                let high_5_ms_bits_col86 = high_5_ms_bits_tmp_9aaf1_63.as_m31();
                *row[86] = high_5_ms_bits_col86;
                *sub_component_inputs.range_check_7_2_5[5] =
                    [low_7_ms_bits_col84, high_2_ls_bits_tmp_9aaf1_62, high_5_ms_bits_col86];
                *lookup_data.range_check_7_2_5_16 = [
                    M31_371240602,
                    low_7_ms_bits_col84,
                    high_2_ls_bits_tmp_9aaf1_62,
                    high_5_ms_bits_col86,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_9aaf1_64 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_5_col41)),
                    );
                let message_word_5_id_col87 = memory_address_to_id_value_tmp_9aaf1_64;
                *row[87] = message_word_5_id_col87;
                *sub_component_inputs.memory_address_to_id[5] =
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_5_col41));
                *lookup_data.memory_address_to_id_17 = [
                    M31_1444891767,
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_5_col41)),
                    message_word_5_id_col87,
                ];

                *sub_component_inputs.memory_id_to_big[5] = message_word_5_id_col87;
                *lookup_data.memory_id_to_big_18 = [
                    M31_1662111297,
                    message_word_5_id_col87,
                    ((low_16_bits_col82) - ((low_7_ms_bits_col84) * (M31_512))),
                    ((low_7_ms_bits_col84) + ((high_2_ls_bits_tmp_9aaf1_62) * (M31_128))),
                    ((high_14_ms_bits_col85) - ((high_5_ms_bits_col86) * (M31_512))),
                    high_5_ms_bits_col86,
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

                let read_u_32_output_tmp_9aaf1_66 = expected_word_tmp_9aaf1_59;

                // Read U 32.

                let memory_address_to_id_value_tmp_9aaf1_67 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_6_col42)),
                    );
                let memory_id_to_big_value_tmp_9aaf1_68 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_9aaf1_67);
                let tmp_9aaf1_69 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9aaf1_68.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col88 = ((((memory_id_to_big_value_tmp_9aaf1_68.get_m31(1))
                    - ((tmp_9aaf1_69.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_9aaf1_68.get_m31(0)));
                *row[88] = low_16_bits_col88;
                let high_16_bits_col89 = ((((memory_id_to_big_value_tmp_9aaf1_68.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_9aaf1_68.get_m31(2)) * (M31_4)))
                    + (tmp_9aaf1_69.as_m31()));
                *row[89] = high_16_bits_col89;
                let expected_word_tmp_9aaf1_70 =
                    PackedUInt32::from_limbs([low_16_bits_col88, high_16_bits_col89]);

                // Verify U 32.

                let low_7_ms_bits_tmp_9aaf1_71 = ((expected_word_tmp_9aaf1_70.low()) >> (UInt16_9));
                let low_7_ms_bits_col90 = low_7_ms_bits_tmp_9aaf1_71.as_m31();
                *row[90] = low_7_ms_bits_col90;
                let high_14_ms_bits_tmp_9aaf1_72 =
                    ((expected_word_tmp_9aaf1_70.high()) >> (UInt16_2));
                let high_14_ms_bits_col91 = high_14_ms_bits_tmp_9aaf1_72.as_m31();
                *row[91] = high_14_ms_bits_col91;
                let high_2_ls_bits_tmp_9aaf1_73 =
                    ((high_16_bits_col89) - ((high_14_ms_bits_col91) * (M31_4)));
                let high_5_ms_bits_tmp_9aaf1_74 = ((high_14_ms_bits_tmp_9aaf1_72) >> (UInt16_9));
                let high_5_ms_bits_col92 = high_5_ms_bits_tmp_9aaf1_74.as_m31();
                *row[92] = high_5_ms_bits_col92;
                *sub_component_inputs.range_check_7_2_5[6] =
                    [low_7_ms_bits_col90, high_2_ls_bits_tmp_9aaf1_73, high_5_ms_bits_col92];
                *lookup_data.range_check_7_2_5_19 = [
                    M31_371240602,
                    low_7_ms_bits_col90,
                    high_2_ls_bits_tmp_9aaf1_73,
                    high_5_ms_bits_col92,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_9aaf1_75 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_6_col42)),
                    );
                let message_word_6_id_col93 = memory_address_to_id_value_tmp_9aaf1_75;
                *row[93] = message_word_6_id_col93;
                *sub_component_inputs.memory_address_to_id[6] =
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_6_col42));
                *lookup_data.memory_address_to_id_20 = [
                    M31_1444891767,
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_6_col42)),
                    message_word_6_id_col93,
                ];

                *sub_component_inputs.memory_id_to_big[6] = message_word_6_id_col93;
                *lookup_data.memory_id_to_big_21 = [
                    M31_1662111297,
                    message_word_6_id_col93,
                    ((low_16_bits_col88) - ((low_7_ms_bits_col90) * (M31_512))),
                    ((low_7_ms_bits_col90) + ((high_2_ls_bits_tmp_9aaf1_73) * (M31_128))),
                    ((high_14_ms_bits_col91) - ((high_5_ms_bits_col92) * (M31_512))),
                    high_5_ms_bits_col92,
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

                let read_u_32_output_tmp_9aaf1_77 = expected_word_tmp_9aaf1_70;

                // Read U 32.

                let memory_address_to_id_value_tmp_9aaf1_78 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_7_col43)),
                    );
                let memory_id_to_big_value_tmp_9aaf1_79 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_9aaf1_78);
                let tmp_9aaf1_80 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9aaf1_79.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col94 = ((((memory_id_to_big_value_tmp_9aaf1_79.get_m31(1))
                    - ((tmp_9aaf1_80.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_9aaf1_79.get_m31(0)));
                *row[94] = low_16_bits_col94;
                let high_16_bits_col95 = ((((memory_id_to_big_value_tmp_9aaf1_79.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_9aaf1_79.get_m31(2)) * (M31_4)))
                    + (tmp_9aaf1_80.as_m31()));
                *row[95] = high_16_bits_col95;
                let expected_word_tmp_9aaf1_81 =
                    PackedUInt32::from_limbs([low_16_bits_col94, high_16_bits_col95]);

                // Verify U 32.

                let low_7_ms_bits_tmp_9aaf1_82 = ((expected_word_tmp_9aaf1_81.low()) >> (UInt16_9));
                let low_7_ms_bits_col96 = low_7_ms_bits_tmp_9aaf1_82.as_m31();
                *row[96] = low_7_ms_bits_col96;
                let high_14_ms_bits_tmp_9aaf1_83 =
                    ((expected_word_tmp_9aaf1_81.high()) >> (UInt16_2));
                let high_14_ms_bits_col97 = high_14_ms_bits_tmp_9aaf1_83.as_m31();
                *row[97] = high_14_ms_bits_col97;
                let high_2_ls_bits_tmp_9aaf1_84 =
                    ((high_16_bits_col95) - ((high_14_ms_bits_col97) * (M31_4)));
                let high_5_ms_bits_tmp_9aaf1_85 = ((high_14_ms_bits_tmp_9aaf1_83) >> (UInt16_9));
                let high_5_ms_bits_col98 = high_5_ms_bits_tmp_9aaf1_85.as_m31();
                *row[98] = high_5_ms_bits_col98;
                *sub_component_inputs.range_check_7_2_5[7] =
                    [low_7_ms_bits_col96, high_2_ls_bits_tmp_9aaf1_84, high_5_ms_bits_col98];
                *lookup_data.range_check_7_2_5_22 = [
                    M31_371240602,
                    low_7_ms_bits_col96,
                    high_2_ls_bits_tmp_9aaf1_84,
                    high_5_ms_bits_col98,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_9aaf1_86 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_7_col43)),
                    );
                let message_word_7_id_col99 = memory_address_to_id_value_tmp_9aaf1_86;
                *row[99] = message_word_7_id_col99;
                *sub_component_inputs.memory_address_to_id[7] =
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_7_col43));
                *lookup_data.memory_address_to_id_23 = [
                    M31_1444891767,
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_7_col43)),
                    message_word_7_id_col99,
                ];

                *sub_component_inputs.memory_id_to_big[7] = message_word_7_id_col99;
                *lookup_data.memory_id_to_big_24 = [
                    M31_1662111297,
                    message_word_7_id_col99,
                    ((low_16_bits_col94) - ((low_7_ms_bits_col96) * (M31_512))),
                    ((low_7_ms_bits_col96) + ((high_2_ls_bits_tmp_9aaf1_84) * (M31_128))),
                    ((high_14_ms_bits_col97) - ((high_5_ms_bits_col98) * (M31_512))),
                    high_5_ms_bits_col98,
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

                let read_u_32_output_tmp_9aaf1_88 = expected_word_tmp_9aaf1_81;

                // Read U 32.

                let memory_address_to_id_value_tmp_9aaf1_89 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_8_col44)),
                    );
                let memory_id_to_big_value_tmp_9aaf1_90 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_9aaf1_89);
                let tmp_9aaf1_91 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9aaf1_90.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col100 = ((((memory_id_to_big_value_tmp_9aaf1_90.get_m31(1))
                    - ((tmp_9aaf1_91.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_9aaf1_90.get_m31(0)));
                *row[100] = low_16_bits_col100;
                let high_16_bits_col101 = ((((memory_id_to_big_value_tmp_9aaf1_90.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_9aaf1_90.get_m31(2)) * (M31_4)))
                    + (tmp_9aaf1_91.as_m31()));
                *row[101] = high_16_bits_col101;
                let expected_word_tmp_9aaf1_92 =
                    PackedUInt32::from_limbs([low_16_bits_col100, high_16_bits_col101]);

                // Verify U 32.

                let low_7_ms_bits_tmp_9aaf1_93 = ((expected_word_tmp_9aaf1_92.low()) >> (UInt16_9));
                let low_7_ms_bits_col102 = low_7_ms_bits_tmp_9aaf1_93.as_m31();
                *row[102] = low_7_ms_bits_col102;
                let high_14_ms_bits_tmp_9aaf1_94 =
                    ((expected_word_tmp_9aaf1_92.high()) >> (UInt16_2));
                let high_14_ms_bits_col103 = high_14_ms_bits_tmp_9aaf1_94.as_m31();
                *row[103] = high_14_ms_bits_col103;
                let high_2_ls_bits_tmp_9aaf1_95 =
                    ((high_16_bits_col101) - ((high_14_ms_bits_col103) * (M31_4)));
                let high_5_ms_bits_tmp_9aaf1_96 = ((high_14_ms_bits_tmp_9aaf1_94) >> (UInt16_9));
                let high_5_ms_bits_col104 = high_5_ms_bits_tmp_9aaf1_96.as_m31();
                *row[104] = high_5_ms_bits_col104;
                *sub_component_inputs.range_check_7_2_5[8] =
                    [low_7_ms_bits_col102, high_2_ls_bits_tmp_9aaf1_95, high_5_ms_bits_col104];
                *lookup_data.range_check_7_2_5_25 = [
                    M31_371240602,
                    low_7_ms_bits_col102,
                    high_2_ls_bits_tmp_9aaf1_95,
                    high_5_ms_bits_col104,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_9aaf1_97 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_8_col44)),
                    );
                let message_word_8_id_col105 = memory_address_to_id_value_tmp_9aaf1_97;
                *row[105] = message_word_8_id_col105;
                *sub_component_inputs.memory_address_to_id[8] =
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_8_col44));
                *lookup_data.memory_address_to_id_26 = [
                    M31_1444891767,
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_8_col44)),
                    message_word_8_id_col105,
                ];

                *sub_component_inputs.memory_id_to_big[8] = message_word_8_id_col105;
                *lookup_data.memory_id_to_big_27 = [
                    M31_1662111297,
                    message_word_8_id_col105,
                    ((low_16_bits_col100) - ((low_7_ms_bits_col102) * (M31_512))),
                    ((low_7_ms_bits_col102) + ((high_2_ls_bits_tmp_9aaf1_95) * (M31_128))),
                    ((high_14_ms_bits_col103) - ((high_5_ms_bits_col104) * (M31_512))),
                    high_5_ms_bits_col104,
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

                let read_u_32_output_tmp_9aaf1_99 = expected_word_tmp_9aaf1_92;

                // Read U 32.

                let memory_address_to_id_value_tmp_9aaf1_100 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_9_col45)),
                    );
                let memory_id_to_big_value_tmp_9aaf1_101 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_9aaf1_100);
                let tmp_9aaf1_102 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9aaf1_101.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col106 = ((((memory_id_to_big_value_tmp_9aaf1_101.get_m31(1))
                    - ((tmp_9aaf1_102.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_9aaf1_101.get_m31(0)));
                *row[106] = low_16_bits_col106;
                let high_16_bits_col107 = ((((memory_id_to_big_value_tmp_9aaf1_101.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_9aaf1_101.get_m31(2)) * (M31_4)))
                    + (tmp_9aaf1_102.as_m31()));
                *row[107] = high_16_bits_col107;
                let expected_word_tmp_9aaf1_103 =
                    PackedUInt32::from_limbs([low_16_bits_col106, high_16_bits_col107]);

                // Verify U 32.

                let low_7_ms_bits_tmp_9aaf1_104 =
                    ((expected_word_tmp_9aaf1_103.low()) >> (UInt16_9));
                let low_7_ms_bits_col108 = low_7_ms_bits_tmp_9aaf1_104.as_m31();
                *row[108] = low_7_ms_bits_col108;
                let high_14_ms_bits_tmp_9aaf1_105 =
                    ((expected_word_tmp_9aaf1_103.high()) >> (UInt16_2));
                let high_14_ms_bits_col109 = high_14_ms_bits_tmp_9aaf1_105.as_m31();
                *row[109] = high_14_ms_bits_col109;
                let high_2_ls_bits_tmp_9aaf1_106 =
                    ((high_16_bits_col107) - ((high_14_ms_bits_col109) * (M31_4)));
                let high_5_ms_bits_tmp_9aaf1_107 = ((high_14_ms_bits_tmp_9aaf1_105) >> (UInt16_9));
                let high_5_ms_bits_col110 = high_5_ms_bits_tmp_9aaf1_107.as_m31();
                *row[110] = high_5_ms_bits_col110;
                *sub_component_inputs.range_check_7_2_5[9] =
                    [low_7_ms_bits_col108, high_2_ls_bits_tmp_9aaf1_106, high_5_ms_bits_col110];
                *lookup_data.range_check_7_2_5_28 = [
                    M31_371240602,
                    low_7_ms_bits_col108,
                    high_2_ls_bits_tmp_9aaf1_106,
                    high_5_ms_bits_col110,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_9aaf1_108 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_9_col45)),
                    );
                let message_word_9_id_col111 = memory_address_to_id_value_tmp_9aaf1_108;
                *row[111] = message_word_9_id_col111;
                *sub_component_inputs.memory_address_to_id[9] =
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_9_col45));
                *lookup_data.memory_address_to_id_29 = [
                    M31_1444891767,
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_9_col45)),
                    message_word_9_id_col111,
                ];

                *sub_component_inputs.memory_id_to_big[9] = message_word_9_id_col111;
                *lookup_data.memory_id_to_big_30 = [
                    M31_1662111297,
                    message_word_9_id_col111,
                    ((low_16_bits_col106) - ((low_7_ms_bits_col108) * (M31_512))),
                    ((low_7_ms_bits_col108) + ((high_2_ls_bits_tmp_9aaf1_106) * (M31_128))),
                    ((high_14_ms_bits_col109) - ((high_5_ms_bits_col110) * (M31_512))),
                    high_5_ms_bits_col110,
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

                let read_u_32_output_tmp_9aaf1_110 = expected_word_tmp_9aaf1_103;

                // Read U 32.

                let memory_address_to_id_value_tmp_9aaf1_111 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_10_col46)),
                    );
                let memory_id_to_big_value_tmp_9aaf1_112 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_9aaf1_111);
                let tmp_9aaf1_113 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9aaf1_112.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col112 = ((((memory_id_to_big_value_tmp_9aaf1_112.get_m31(1))
                    - ((tmp_9aaf1_113.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_9aaf1_112.get_m31(0)));
                *row[112] = low_16_bits_col112;
                let high_16_bits_col113 = ((((memory_id_to_big_value_tmp_9aaf1_112.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_9aaf1_112.get_m31(2)) * (M31_4)))
                    + (tmp_9aaf1_113.as_m31()));
                *row[113] = high_16_bits_col113;
                let expected_word_tmp_9aaf1_114 =
                    PackedUInt32::from_limbs([low_16_bits_col112, high_16_bits_col113]);

                // Verify U 32.

                let low_7_ms_bits_tmp_9aaf1_115 =
                    ((expected_word_tmp_9aaf1_114.low()) >> (UInt16_9));
                let low_7_ms_bits_col114 = low_7_ms_bits_tmp_9aaf1_115.as_m31();
                *row[114] = low_7_ms_bits_col114;
                let high_14_ms_bits_tmp_9aaf1_116 =
                    ((expected_word_tmp_9aaf1_114.high()) >> (UInt16_2));
                let high_14_ms_bits_col115 = high_14_ms_bits_tmp_9aaf1_116.as_m31();
                *row[115] = high_14_ms_bits_col115;
                let high_2_ls_bits_tmp_9aaf1_117 =
                    ((high_16_bits_col113) - ((high_14_ms_bits_col115) * (M31_4)));
                let high_5_ms_bits_tmp_9aaf1_118 = ((high_14_ms_bits_tmp_9aaf1_116) >> (UInt16_9));
                let high_5_ms_bits_col116 = high_5_ms_bits_tmp_9aaf1_118.as_m31();
                *row[116] = high_5_ms_bits_col116;
                *sub_component_inputs.range_check_7_2_5[10] =
                    [low_7_ms_bits_col114, high_2_ls_bits_tmp_9aaf1_117, high_5_ms_bits_col116];
                *lookup_data.range_check_7_2_5_31 = [
                    M31_371240602,
                    low_7_ms_bits_col114,
                    high_2_ls_bits_tmp_9aaf1_117,
                    high_5_ms_bits_col116,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_9aaf1_119 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_10_col46)),
                    );
                let message_word_10_id_col117 = memory_address_to_id_value_tmp_9aaf1_119;
                *row[117] = message_word_10_id_col117;
                *sub_component_inputs.memory_address_to_id[10] =
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_10_col46));
                *lookup_data.memory_address_to_id_32 = [
                    M31_1444891767,
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_10_col46)),
                    message_word_10_id_col117,
                ];

                *sub_component_inputs.memory_id_to_big[10] = message_word_10_id_col117;
                *lookup_data.memory_id_to_big_33 = [
                    M31_1662111297,
                    message_word_10_id_col117,
                    ((low_16_bits_col112) - ((low_7_ms_bits_col114) * (M31_512))),
                    ((low_7_ms_bits_col114) + ((high_2_ls_bits_tmp_9aaf1_117) * (M31_128))),
                    ((high_14_ms_bits_col115) - ((high_5_ms_bits_col116) * (M31_512))),
                    high_5_ms_bits_col116,
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

                let read_u_32_output_tmp_9aaf1_121 = expected_word_tmp_9aaf1_114;

                // Read U 32.

                let memory_address_to_id_value_tmp_9aaf1_122 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_11_col47)),
                    );
                let memory_id_to_big_value_tmp_9aaf1_123 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_9aaf1_122);
                let tmp_9aaf1_124 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9aaf1_123.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col118 = ((((memory_id_to_big_value_tmp_9aaf1_123.get_m31(1))
                    - ((tmp_9aaf1_124.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_9aaf1_123.get_m31(0)));
                *row[118] = low_16_bits_col118;
                let high_16_bits_col119 = ((((memory_id_to_big_value_tmp_9aaf1_123.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_9aaf1_123.get_m31(2)) * (M31_4)))
                    + (tmp_9aaf1_124.as_m31()));
                *row[119] = high_16_bits_col119;
                let expected_word_tmp_9aaf1_125 =
                    PackedUInt32::from_limbs([low_16_bits_col118, high_16_bits_col119]);

                // Verify U 32.

                let low_7_ms_bits_tmp_9aaf1_126 =
                    ((expected_word_tmp_9aaf1_125.low()) >> (UInt16_9));
                let low_7_ms_bits_col120 = low_7_ms_bits_tmp_9aaf1_126.as_m31();
                *row[120] = low_7_ms_bits_col120;
                let high_14_ms_bits_tmp_9aaf1_127 =
                    ((expected_word_tmp_9aaf1_125.high()) >> (UInt16_2));
                let high_14_ms_bits_col121 = high_14_ms_bits_tmp_9aaf1_127.as_m31();
                *row[121] = high_14_ms_bits_col121;
                let high_2_ls_bits_tmp_9aaf1_128 =
                    ((high_16_bits_col119) - ((high_14_ms_bits_col121) * (M31_4)));
                let high_5_ms_bits_tmp_9aaf1_129 = ((high_14_ms_bits_tmp_9aaf1_127) >> (UInt16_9));
                let high_5_ms_bits_col122 = high_5_ms_bits_tmp_9aaf1_129.as_m31();
                *row[122] = high_5_ms_bits_col122;
                *sub_component_inputs.range_check_7_2_5[11] =
                    [low_7_ms_bits_col120, high_2_ls_bits_tmp_9aaf1_128, high_5_ms_bits_col122];
                *lookup_data.range_check_7_2_5_34 = [
                    M31_371240602,
                    low_7_ms_bits_col120,
                    high_2_ls_bits_tmp_9aaf1_128,
                    high_5_ms_bits_col122,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_9aaf1_130 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_11_col47)),
                    );
                let message_word_11_id_col123 = memory_address_to_id_value_tmp_9aaf1_130;
                *row[123] = message_word_11_id_col123;
                *sub_component_inputs.memory_address_to_id[11] =
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_11_col47));
                *lookup_data.memory_address_to_id_35 = [
                    M31_1444891767,
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_11_col47)),
                    message_word_11_id_col123,
                ];

                *sub_component_inputs.memory_id_to_big[11] = message_word_11_id_col123;
                *lookup_data.memory_id_to_big_36 = [
                    M31_1662111297,
                    message_word_11_id_col123,
                    ((low_16_bits_col118) - ((low_7_ms_bits_col120) * (M31_512))),
                    ((low_7_ms_bits_col120) + ((high_2_ls_bits_tmp_9aaf1_128) * (M31_128))),
                    ((high_14_ms_bits_col121) - ((high_5_ms_bits_col122) * (M31_512))),
                    high_5_ms_bits_col122,
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

                let read_u_32_output_tmp_9aaf1_132 = expected_word_tmp_9aaf1_125;

                // Read U 32.

                let memory_address_to_id_value_tmp_9aaf1_133 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_12_col48)),
                    );
                let memory_id_to_big_value_tmp_9aaf1_134 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_9aaf1_133);
                let tmp_9aaf1_135 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9aaf1_134.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col124 = ((((memory_id_to_big_value_tmp_9aaf1_134.get_m31(1))
                    - ((tmp_9aaf1_135.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_9aaf1_134.get_m31(0)));
                *row[124] = low_16_bits_col124;
                let high_16_bits_col125 = ((((memory_id_to_big_value_tmp_9aaf1_134.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_9aaf1_134.get_m31(2)) * (M31_4)))
                    + (tmp_9aaf1_135.as_m31()));
                *row[125] = high_16_bits_col125;
                let expected_word_tmp_9aaf1_136 =
                    PackedUInt32::from_limbs([low_16_bits_col124, high_16_bits_col125]);

                // Verify U 32.

                let low_7_ms_bits_tmp_9aaf1_137 =
                    ((expected_word_tmp_9aaf1_136.low()) >> (UInt16_9));
                let low_7_ms_bits_col126 = low_7_ms_bits_tmp_9aaf1_137.as_m31();
                *row[126] = low_7_ms_bits_col126;
                let high_14_ms_bits_tmp_9aaf1_138 =
                    ((expected_word_tmp_9aaf1_136.high()) >> (UInt16_2));
                let high_14_ms_bits_col127 = high_14_ms_bits_tmp_9aaf1_138.as_m31();
                *row[127] = high_14_ms_bits_col127;
                let high_2_ls_bits_tmp_9aaf1_139 =
                    ((high_16_bits_col125) - ((high_14_ms_bits_col127) * (M31_4)));
                let high_5_ms_bits_tmp_9aaf1_140 = ((high_14_ms_bits_tmp_9aaf1_138) >> (UInt16_9));
                let high_5_ms_bits_col128 = high_5_ms_bits_tmp_9aaf1_140.as_m31();
                *row[128] = high_5_ms_bits_col128;
                *sub_component_inputs.range_check_7_2_5[12] =
                    [low_7_ms_bits_col126, high_2_ls_bits_tmp_9aaf1_139, high_5_ms_bits_col128];
                *lookup_data.range_check_7_2_5_37 = [
                    M31_371240602,
                    low_7_ms_bits_col126,
                    high_2_ls_bits_tmp_9aaf1_139,
                    high_5_ms_bits_col128,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_9aaf1_141 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_12_col48)),
                    );
                let message_word_12_id_col129 = memory_address_to_id_value_tmp_9aaf1_141;
                *row[129] = message_word_12_id_col129;
                *sub_component_inputs.memory_address_to_id[12] =
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_12_col48));
                *lookup_data.memory_address_to_id_38 = [
                    M31_1444891767,
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_12_col48)),
                    message_word_12_id_col129,
                ];

                *sub_component_inputs.memory_id_to_big[12] = message_word_12_id_col129;
                *lookup_data.memory_id_to_big_39 = [
                    M31_1662111297,
                    message_word_12_id_col129,
                    ((low_16_bits_col124) - ((low_7_ms_bits_col126) * (M31_512))),
                    ((low_7_ms_bits_col126) + ((high_2_ls_bits_tmp_9aaf1_139) * (M31_128))),
                    ((high_14_ms_bits_col127) - ((high_5_ms_bits_col128) * (M31_512))),
                    high_5_ms_bits_col128,
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

                let read_u_32_output_tmp_9aaf1_143 = expected_word_tmp_9aaf1_136;

                // Read U 32.

                let memory_address_to_id_value_tmp_9aaf1_144 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_13_col49)),
                    );
                let memory_id_to_big_value_tmp_9aaf1_145 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_9aaf1_144);
                let tmp_9aaf1_146 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9aaf1_145.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col130 = ((((memory_id_to_big_value_tmp_9aaf1_145.get_m31(1))
                    - ((tmp_9aaf1_146.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_9aaf1_145.get_m31(0)));
                *row[130] = low_16_bits_col130;
                let high_16_bits_col131 = ((((memory_id_to_big_value_tmp_9aaf1_145.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_9aaf1_145.get_m31(2)) * (M31_4)))
                    + (tmp_9aaf1_146.as_m31()));
                *row[131] = high_16_bits_col131;
                let expected_word_tmp_9aaf1_147 =
                    PackedUInt32::from_limbs([low_16_bits_col130, high_16_bits_col131]);

                // Verify U 32.

                let low_7_ms_bits_tmp_9aaf1_148 =
                    ((expected_word_tmp_9aaf1_147.low()) >> (UInt16_9));
                let low_7_ms_bits_col132 = low_7_ms_bits_tmp_9aaf1_148.as_m31();
                *row[132] = low_7_ms_bits_col132;
                let high_14_ms_bits_tmp_9aaf1_149 =
                    ((expected_word_tmp_9aaf1_147.high()) >> (UInt16_2));
                let high_14_ms_bits_col133 = high_14_ms_bits_tmp_9aaf1_149.as_m31();
                *row[133] = high_14_ms_bits_col133;
                let high_2_ls_bits_tmp_9aaf1_150 =
                    ((high_16_bits_col131) - ((high_14_ms_bits_col133) * (M31_4)));
                let high_5_ms_bits_tmp_9aaf1_151 = ((high_14_ms_bits_tmp_9aaf1_149) >> (UInt16_9));
                let high_5_ms_bits_col134 = high_5_ms_bits_tmp_9aaf1_151.as_m31();
                *row[134] = high_5_ms_bits_col134;
                *sub_component_inputs.range_check_7_2_5[13] =
                    [low_7_ms_bits_col132, high_2_ls_bits_tmp_9aaf1_150, high_5_ms_bits_col134];
                *lookup_data.range_check_7_2_5_40 = [
                    M31_371240602,
                    low_7_ms_bits_col132,
                    high_2_ls_bits_tmp_9aaf1_150,
                    high_5_ms_bits_col134,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_9aaf1_152 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_13_col49)),
                    );
                let message_word_13_id_col135 = memory_address_to_id_value_tmp_9aaf1_152;
                *row[135] = message_word_13_id_col135;
                *sub_component_inputs.memory_address_to_id[13] =
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_13_col49));
                *lookup_data.memory_address_to_id_41 = [
                    M31_1444891767,
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_13_col49)),
                    message_word_13_id_col135,
                ];

                *sub_component_inputs.memory_id_to_big[13] = message_word_13_id_col135;
                *lookup_data.memory_id_to_big_42 = [
                    M31_1662111297,
                    message_word_13_id_col135,
                    ((low_16_bits_col130) - ((low_7_ms_bits_col132) * (M31_512))),
                    ((low_7_ms_bits_col132) + ((high_2_ls_bits_tmp_9aaf1_150) * (M31_128))),
                    ((high_14_ms_bits_col133) - ((high_5_ms_bits_col134) * (M31_512))),
                    high_5_ms_bits_col134,
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

                let read_u_32_output_tmp_9aaf1_154 = expected_word_tmp_9aaf1_147;

                // Read U 32.

                let memory_address_to_id_value_tmp_9aaf1_155 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_14_col50)),
                    );
                let memory_id_to_big_value_tmp_9aaf1_156 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_9aaf1_155);
                let tmp_9aaf1_157 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9aaf1_156.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col136 = ((((memory_id_to_big_value_tmp_9aaf1_156.get_m31(1))
                    - ((tmp_9aaf1_157.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_9aaf1_156.get_m31(0)));
                *row[136] = low_16_bits_col136;
                let high_16_bits_col137 = ((((memory_id_to_big_value_tmp_9aaf1_156.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_9aaf1_156.get_m31(2)) * (M31_4)))
                    + (tmp_9aaf1_157.as_m31()));
                *row[137] = high_16_bits_col137;
                let expected_word_tmp_9aaf1_158 =
                    PackedUInt32::from_limbs([low_16_bits_col136, high_16_bits_col137]);

                // Verify U 32.

                let low_7_ms_bits_tmp_9aaf1_159 =
                    ((expected_word_tmp_9aaf1_158.low()) >> (UInt16_9));
                let low_7_ms_bits_col138 = low_7_ms_bits_tmp_9aaf1_159.as_m31();
                *row[138] = low_7_ms_bits_col138;
                let high_14_ms_bits_tmp_9aaf1_160 =
                    ((expected_word_tmp_9aaf1_158.high()) >> (UInt16_2));
                let high_14_ms_bits_col139 = high_14_ms_bits_tmp_9aaf1_160.as_m31();
                *row[139] = high_14_ms_bits_col139;
                let high_2_ls_bits_tmp_9aaf1_161 =
                    ((high_16_bits_col137) - ((high_14_ms_bits_col139) * (M31_4)));
                let high_5_ms_bits_tmp_9aaf1_162 = ((high_14_ms_bits_tmp_9aaf1_160) >> (UInt16_9));
                let high_5_ms_bits_col140 = high_5_ms_bits_tmp_9aaf1_162.as_m31();
                *row[140] = high_5_ms_bits_col140;
                *sub_component_inputs.range_check_7_2_5[14] =
                    [low_7_ms_bits_col138, high_2_ls_bits_tmp_9aaf1_161, high_5_ms_bits_col140];
                *lookup_data.range_check_7_2_5_43 = [
                    M31_371240602,
                    low_7_ms_bits_col138,
                    high_2_ls_bits_tmp_9aaf1_161,
                    high_5_ms_bits_col140,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_9aaf1_163 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_14_col50)),
                    );
                let message_word_14_id_col141 = memory_address_to_id_value_tmp_9aaf1_163;
                *row[141] = message_word_14_id_col141;
                *sub_component_inputs.memory_address_to_id[14] =
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_14_col50));
                *lookup_data.memory_address_to_id_44 = [
                    M31_1444891767,
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_14_col50)),
                    message_word_14_id_col141,
                ];

                *sub_component_inputs.memory_id_to_big[14] = message_word_14_id_col141;
                *lookup_data.memory_id_to_big_45 = [
                    M31_1662111297,
                    message_word_14_id_col141,
                    ((low_16_bits_col136) - ((low_7_ms_bits_col138) * (M31_512))),
                    ((low_7_ms_bits_col138) + ((high_2_ls_bits_tmp_9aaf1_161) * (M31_128))),
                    ((high_14_ms_bits_col139) - ((high_5_ms_bits_col140) * (M31_512))),
                    high_5_ms_bits_col140,
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

                let read_u_32_output_tmp_9aaf1_165 = expected_word_tmp_9aaf1_158;

                // Read U 32.

                let memory_address_to_id_value_tmp_9aaf1_166 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_15_col51)),
                    );
                let memory_id_to_big_value_tmp_9aaf1_167 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_9aaf1_166);
                let tmp_9aaf1_168 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9aaf1_167.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col142 = ((((memory_id_to_big_value_tmp_9aaf1_167.get_m31(1))
                    - ((tmp_9aaf1_168.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_9aaf1_167.get_m31(0)));
                *row[142] = low_16_bits_col142;
                let high_16_bits_col143 = ((((memory_id_to_big_value_tmp_9aaf1_167.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_9aaf1_167.get_m31(2)) * (M31_4)))
                    + (tmp_9aaf1_168.as_m31()));
                *row[143] = high_16_bits_col143;
                let expected_word_tmp_9aaf1_169 =
                    PackedUInt32::from_limbs([low_16_bits_col142, high_16_bits_col143]);

                // Verify U 32.

                let low_7_ms_bits_tmp_9aaf1_170 =
                    ((expected_word_tmp_9aaf1_169.low()) >> (UInt16_9));
                let low_7_ms_bits_col144 = low_7_ms_bits_tmp_9aaf1_170.as_m31();
                *row[144] = low_7_ms_bits_col144;
                let high_14_ms_bits_tmp_9aaf1_171 =
                    ((expected_word_tmp_9aaf1_169.high()) >> (UInt16_2));
                let high_14_ms_bits_col145 = high_14_ms_bits_tmp_9aaf1_171.as_m31();
                *row[145] = high_14_ms_bits_col145;
                let high_2_ls_bits_tmp_9aaf1_172 =
                    ((high_16_bits_col143) - ((high_14_ms_bits_col145) * (M31_4)));
                let high_5_ms_bits_tmp_9aaf1_173 = ((high_14_ms_bits_tmp_9aaf1_171) >> (UInt16_9));
                let high_5_ms_bits_col146 = high_5_ms_bits_tmp_9aaf1_173.as_m31();
                *row[146] = high_5_ms_bits_col146;
                *sub_component_inputs.range_check_7_2_5[15] =
                    [low_7_ms_bits_col144, high_2_ls_bits_tmp_9aaf1_172, high_5_ms_bits_col146];
                *lookup_data.range_check_7_2_5_46 = [
                    M31_371240602,
                    low_7_ms_bits_col144,
                    high_2_ls_bits_tmp_9aaf1_172,
                    high_5_ms_bits_col146,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_9aaf1_174 = memory_address_to_id_state
                    .deduce_output(
                        ((input_limb_34_col35) + (blake_round_sigma_output_limb_15_col51)),
                    );
                let message_word_15_id_col147 = memory_address_to_id_value_tmp_9aaf1_174;
                *row[147] = message_word_15_id_col147;
                *sub_component_inputs.memory_address_to_id[15] =
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_15_col51));
                *lookup_data.memory_address_to_id_47 = [
                    M31_1444891767,
                    ((input_limb_34_col35) + (blake_round_sigma_output_limb_15_col51)),
                    message_word_15_id_col147,
                ];

                *sub_component_inputs.memory_id_to_big[15] = message_word_15_id_col147;
                *lookup_data.memory_id_to_big_48 = [
                    M31_1662111297,
                    message_word_15_id_col147,
                    ((low_16_bits_col142) - ((low_7_ms_bits_col144) * (M31_512))),
                    ((low_7_ms_bits_col144) + ((high_2_ls_bits_tmp_9aaf1_172) * (M31_128))),
                    ((high_14_ms_bits_col145) - ((high_5_ms_bits_col146) * (M31_512))),
                    high_5_ms_bits_col146,
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

                let read_u_32_output_tmp_9aaf1_176 = expected_word_tmp_9aaf1_169;

                *sub_component_inputs.blake_g[0] = [
                    blake_round_input.2.0[0],
                    blake_round_input.2.0[4],
                    blake_round_input.2.0[8],
                    blake_round_input.2.0[12],
                    read_u_32_output_tmp_9aaf1_11,
                    read_u_32_output_tmp_9aaf1_22,
                ];
                let blake_g_output_tmp_9aaf1_177 = PackedBlakeG::deduce_output([
                    blake_round_input.2.0[0],
                    blake_round_input.2.0[4],
                    blake_round_input.2.0[8],
                    blake_round_input.2.0[12],
                    read_u_32_output_tmp_9aaf1_11,
                    read_u_32_output_tmp_9aaf1_22,
                ]);
                let blake_g_output_limb_0_col148 = blake_g_output_tmp_9aaf1_177[0].low().as_m31();
                *row[148] = blake_g_output_limb_0_col148;
                let blake_g_output_limb_1_col149 = blake_g_output_tmp_9aaf1_177[0].high().as_m31();
                *row[149] = blake_g_output_limb_1_col149;
                let blake_g_output_limb_2_col150 = blake_g_output_tmp_9aaf1_177[1].low().as_m31();
                *row[150] = blake_g_output_limb_2_col150;
                let blake_g_output_limb_3_col151 = blake_g_output_tmp_9aaf1_177[1].high().as_m31();
                *row[151] = blake_g_output_limb_3_col151;
                let blake_g_output_limb_4_col152 = blake_g_output_tmp_9aaf1_177[2].low().as_m31();
                *row[152] = blake_g_output_limb_4_col152;
                let blake_g_output_limb_5_col153 = blake_g_output_tmp_9aaf1_177[2].high().as_m31();
                *row[153] = blake_g_output_limb_5_col153;
                let blake_g_output_limb_6_col154 = blake_g_output_tmp_9aaf1_177[3].low().as_m31();
                *row[154] = blake_g_output_limb_6_col154;
                let blake_g_output_limb_7_col155 = blake_g_output_tmp_9aaf1_177[3].high().as_m31();
                *row[155] = blake_g_output_limb_7_col155;
                *lookup_data.blake_g_49 = [
                    M31_1139985212,
                    input_limb_2_col3,
                    input_limb_3_col4,
                    input_limb_10_col11,
                    input_limb_11_col12,
                    input_limb_18_col19,
                    input_limb_19_col20,
                    input_limb_26_col27,
                    input_limb_27_col28,
                    low_16_bits_col52,
                    high_16_bits_col53,
                    low_16_bits_col58,
                    high_16_bits_col59,
                    blake_g_output_limb_0_col148,
                    blake_g_output_limb_1_col149,
                    blake_g_output_limb_2_col150,
                    blake_g_output_limb_3_col151,
                    blake_g_output_limb_4_col152,
                    blake_g_output_limb_5_col153,
                    blake_g_output_limb_6_col154,
                    blake_g_output_limb_7_col155,
                ];
                *sub_component_inputs.blake_g[1] = [
                    blake_round_input.2.0[1],
                    blake_round_input.2.0[5],
                    blake_round_input.2.0[9],
                    blake_round_input.2.0[13],
                    read_u_32_output_tmp_9aaf1_33,
                    read_u_32_output_tmp_9aaf1_44,
                ];
                let blake_g_output_tmp_9aaf1_178 = PackedBlakeG::deduce_output([
                    blake_round_input.2.0[1],
                    blake_round_input.2.0[5],
                    blake_round_input.2.0[9],
                    blake_round_input.2.0[13],
                    read_u_32_output_tmp_9aaf1_33,
                    read_u_32_output_tmp_9aaf1_44,
                ]);
                let blake_g_output_limb_0_col156 = blake_g_output_tmp_9aaf1_178[0].low().as_m31();
                *row[156] = blake_g_output_limb_0_col156;
                let blake_g_output_limb_1_col157 = blake_g_output_tmp_9aaf1_178[0].high().as_m31();
                *row[157] = blake_g_output_limb_1_col157;
                let blake_g_output_limb_2_col158 = blake_g_output_tmp_9aaf1_178[1].low().as_m31();
                *row[158] = blake_g_output_limb_2_col158;
                let blake_g_output_limb_3_col159 = blake_g_output_tmp_9aaf1_178[1].high().as_m31();
                *row[159] = blake_g_output_limb_3_col159;
                let blake_g_output_limb_4_col160 = blake_g_output_tmp_9aaf1_178[2].low().as_m31();
                *row[160] = blake_g_output_limb_4_col160;
                let blake_g_output_limb_5_col161 = blake_g_output_tmp_9aaf1_178[2].high().as_m31();
                *row[161] = blake_g_output_limb_5_col161;
                let blake_g_output_limb_6_col162 = blake_g_output_tmp_9aaf1_178[3].low().as_m31();
                *row[162] = blake_g_output_limb_6_col162;
                let blake_g_output_limb_7_col163 = blake_g_output_tmp_9aaf1_178[3].high().as_m31();
                *row[163] = blake_g_output_limb_7_col163;
                *lookup_data.blake_g_50 = [
                    M31_1139985212,
                    input_limb_4_col5,
                    input_limb_5_col6,
                    input_limb_12_col13,
                    input_limb_13_col14,
                    input_limb_20_col21,
                    input_limb_21_col22,
                    input_limb_28_col29,
                    input_limb_29_col30,
                    low_16_bits_col64,
                    high_16_bits_col65,
                    low_16_bits_col70,
                    high_16_bits_col71,
                    blake_g_output_limb_0_col156,
                    blake_g_output_limb_1_col157,
                    blake_g_output_limb_2_col158,
                    blake_g_output_limb_3_col159,
                    blake_g_output_limb_4_col160,
                    blake_g_output_limb_5_col161,
                    blake_g_output_limb_6_col162,
                    blake_g_output_limb_7_col163,
                ];
                *sub_component_inputs.blake_g[2] = [
                    blake_round_input.2.0[2],
                    blake_round_input.2.0[6],
                    blake_round_input.2.0[10],
                    blake_round_input.2.0[14],
                    read_u_32_output_tmp_9aaf1_55,
                    read_u_32_output_tmp_9aaf1_66,
                ];
                let blake_g_output_tmp_9aaf1_179 = PackedBlakeG::deduce_output([
                    blake_round_input.2.0[2],
                    blake_round_input.2.0[6],
                    blake_round_input.2.0[10],
                    blake_round_input.2.0[14],
                    read_u_32_output_tmp_9aaf1_55,
                    read_u_32_output_tmp_9aaf1_66,
                ]);
                let blake_g_output_limb_0_col164 = blake_g_output_tmp_9aaf1_179[0].low().as_m31();
                *row[164] = blake_g_output_limb_0_col164;
                let blake_g_output_limb_1_col165 = blake_g_output_tmp_9aaf1_179[0].high().as_m31();
                *row[165] = blake_g_output_limb_1_col165;
                let blake_g_output_limb_2_col166 = blake_g_output_tmp_9aaf1_179[1].low().as_m31();
                *row[166] = blake_g_output_limb_2_col166;
                let blake_g_output_limb_3_col167 = blake_g_output_tmp_9aaf1_179[1].high().as_m31();
                *row[167] = blake_g_output_limb_3_col167;
                let blake_g_output_limb_4_col168 = blake_g_output_tmp_9aaf1_179[2].low().as_m31();
                *row[168] = blake_g_output_limb_4_col168;
                let blake_g_output_limb_5_col169 = blake_g_output_tmp_9aaf1_179[2].high().as_m31();
                *row[169] = blake_g_output_limb_5_col169;
                let blake_g_output_limb_6_col170 = blake_g_output_tmp_9aaf1_179[3].low().as_m31();
                *row[170] = blake_g_output_limb_6_col170;
                let blake_g_output_limb_7_col171 = blake_g_output_tmp_9aaf1_179[3].high().as_m31();
                *row[171] = blake_g_output_limb_7_col171;
                *lookup_data.blake_g_51 = [
                    M31_1139985212,
                    input_limb_6_col7,
                    input_limb_7_col8,
                    input_limb_14_col15,
                    input_limb_15_col16,
                    input_limb_22_col23,
                    input_limb_23_col24,
                    input_limb_30_col31,
                    input_limb_31_col32,
                    low_16_bits_col76,
                    high_16_bits_col77,
                    low_16_bits_col82,
                    high_16_bits_col83,
                    blake_g_output_limb_0_col164,
                    blake_g_output_limb_1_col165,
                    blake_g_output_limb_2_col166,
                    blake_g_output_limb_3_col167,
                    blake_g_output_limb_4_col168,
                    blake_g_output_limb_5_col169,
                    blake_g_output_limb_6_col170,
                    blake_g_output_limb_7_col171,
                ];
                *sub_component_inputs.blake_g[3] = [
                    blake_round_input.2.0[3],
                    blake_round_input.2.0[7],
                    blake_round_input.2.0[11],
                    blake_round_input.2.0[15],
                    read_u_32_output_tmp_9aaf1_77,
                    read_u_32_output_tmp_9aaf1_88,
                ];
                let blake_g_output_tmp_9aaf1_180 = PackedBlakeG::deduce_output([
                    blake_round_input.2.0[3],
                    blake_round_input.2.0[7],
                    blake_round_input.2.0[11],
                    blake_round_input.2.0[15],
                    read_u_32_output_tmp_9aaf1_77,
                    read_u_32_output_tmp_9aaf1_88,
                ]);
                let blake_g_output_limb_0_col172 = blake_g_output_tmp_9aaf1_180[0].low().as_m31();
                *row[172] = blake_g_output_limb_0_col172;
                let blake_g_output_limb_1_col173 = blake_g_output_tmp_9aaf1_180[0].high().as_m31();
                *row[173] = blake_g_output_limb_1_col173;
                let blake_g_output_limb_2_col174 = blake_g_output_tmp_9aaf1_180[1].low().as_m31();
                *row[174] = blake_g_output_limb_2_col174;
                let blake_g_output_limb_3_col175 = blake_g_output_tmp_9aaf1_180[1].high().as_m31();
                *row[175] = blake_g_output_limb_3_col175;
                let blake_g_output_limb_4_col176 = blake_g_output_tmp_9aaf1_180[2].low().as_m31();
                *row[176] = blake_g_output_limb_4_col176;
                let blake_g_output_limb_5_col177 = blake_g_output_tmp_9aaf1_180[2].high().as_m31();
                *row[177] = blake_g_output_limb_5_col177;
                let blake_g_output_limb_6_col178 = blake_g_output_tmp_9aaf1_180[3].low().as_m31();
                *row[178] = blake_g_output_limb_6_col178;
                let blake_g_output_limb_7_col179 = blake_g_output_tmp_9aaf1_180[3].high().as_m31();
                *row[179] = blake_g_output_limb_7_col179;
                *lookup_data.blake_g_52 = [
                    M31_1139985212,
                    input_limb_8_col9,
                    input_limb_9_col10,
                    input_limb_16_col17,
                    input_limb_17_col18,
                    input_limb_24_col25,
                    input_limb_25_col26,
                    input_limb_32_col33,
                    input_limb_33_col34,
                    low_16_bits_col88,
                    high_16_bits_col89,
                    low_16_bits_col94,
                    high_16_bits_col95,
                    blake_g_output_limb_0_col172,
                    blake_g_output_limb_1_col173,
                    blake_g_output_limb_2_col174,
                    blake_g_output_limb_3_col175,
                    blake_g_output_limb_4_col176,
                    blake_g_output_limb_5_col177,
                    blake_g_output_limb_6_col178,
                    blake_g_output_limb_7_col179,
                ];
                *sub_component_inputs.blake_g[4] = [
                    blake_g_output_tmp_9aaf1_177[0],
                    blake_g_output_tmp_9aaf1_178[1],
                    blake_g_output_tmp_9aaf1_179[2],
                    blake_g_output_tmp_9aaf1_180[3],
                    read_u_32_output_tmp_9aaf1_99,
                    read_u_32_output_tmp_9aaf1_110,
                ];
                let blake_g_output_tmp_9aaf1_181 = PackedBlakeG::deduce_output([
                    blake_g_output_tmp_9aaf1_177[0],
                    blake_g_output_tmp_9aaf1_178[1],
                    blake_g_output_tmp_9aaf1_179[2],
                    blake_g_output_tmp_9aaf1_180[3],
                    read_u_32_output_tmp_9aaf1_99,
                    read_u_32_output_tmp_9aaf1_110,
                ]);
                let blake_g_output_limb_0_col180 = blake_g_output_tmp_9aaf1_181[0].low().as_m31();
                *row[180] = blake_g_output_limb_0_col180;
                let blake_g_output_limb_1_col181 = blake_g_output_tmp_9aaf1_181[0].high().as_m31();
                *row[181] = blake_g_output_limb_1_col181;
                let blake_g_output_limb_2_col182 = blake_g_output_tmp_9aaf1_181[1].low().as_m31();
                *row[182] = blake_g_output_limb_2_col182;
                let blake_g_output_limb_3_col183 = blake_g_output_tmp_9aaf1_181[1].high().as_m31();
                *row[183] = blake_g_output_limb_3_col183;
                let blake_g_output_limb_4_col184 = blake_g_output_tmp_9aaf1_181[2].low().as_m31();
                *row[184] = blake_g_output_limb_4_col184;
                let blake_g_output_limb_5_col185 = blake_g_output_tmp_9aaf1_181[2].high().as_m31();
                *row[185] = blake_g_output_limb_5_col185;
                let blake_g_output_limb_6_col186 = blake_g_output_tmp_9aaf1_181[3].low().as_m31();
                *row[186] = blake_g_output_limb_6_col186;
                let blake_g_output_limb_7_col187 = blake_g_output_tmp_9aaf1_181[3].high().as_m31();
                *row[187] = blake_g_output_limb_7_col187;
                *lookup_data.blake_g_53 = [
                    M31_1139985212,
                    blake_g_output_limb_0_col148,
                    blake_g_output_limb_1_col149,
                    blake_g_output_limb_2_col158,
                    blake_g_output_limb_3_col159,
                    blake_g_output_limb_4_col168,
                    blake_g_output_limb_5_col169,
                    blake_g_output_limb_6_col178,
                    blake_g_output_limb_7_col179,
                    low_16_bits_col100,
                    high_16_bits_col101,
                    low_16_bits_col106,
                    high_16_bits_col107,
                    blake_g_output_limb_0_col180,
                    blake_g_output_limb_1_col181,
                    blake_g_output_limb_2_col182,
                    blake_g_output_limb_3_col183,
                    blake_g_output_limb_4_col184,
                    blake_g_output_limb_5_col185,
                    blake_g_output_limb_6_col186,
                    blake_g_output_limb_7_col187,
                ];
                *sub_component_inputs.blake_g[5] = [
                    blake_g_output_tmp_9aaf1_178[0],
                    blake_g_output_tmp_9aaf1_179[1],
                    blake_g_output_tmp_9aaf1_180[2],
                    blake_g_output_tmp_9aaf1_177[3],
                    read_u_32_output_tmp_9aaf1_121,
                    read_u_32_output_tmp_9aaf1_132,
                ];
                let blake_g_output_tmp_9aaf1_182 = PackedBlakeG::deduce_output([
                    blake_g_output_tmp_9aaf1_178[0],
                    blake_g_output_tmp_9aaf1_179[1],
                    blake_g_output_tmp_9aaf1_180[2],
                    blake_g_output_tmp_9aaf1_177[3],
                    read_u_32_output_tmp_9aaf1_121,
                    read_u_32_output_tmp_9aaf1_132,
                ]);
                let blake_g_output_limb_0_col188 = blake_g_output_tmp_9aaf1_182[0].low().as_m31();
                *row[188] = blake_g_output_limb_0_col188;
                let blake_g_output_limb_1_col189 = blake_g_output_tmp_9aaf1_182[0].high().as_m31();
                *row[189] = blake_g_output_limb_1_col189;
                let blake_g_output_limb_2_col190 = blake_g_output_tmp_9aaf1_182[1].low().as_m31();
                *row[190] = blake_g_output_limb_2_col190;
                let blake_g_output_limb_3_col191 = blake_g_output_tmp_9aaf1_182[1].high().as_m31();
                *row[191] = blake_g_output_limb_3_col191;
                let blake_g_output_limb_4_col192 = blake_g_output_tmp_9aaf1_182[2].low().as_m31();
                *row[192] = blake_g_output_limb_4_col192;
                let blake_g_output_limb_5_col193 = blake_g_output_tmp_9aaf1_182[2].high().as_m31();
                *row[193] = blake_g_output_limb_5_col193;
                let blake_g_output_limb_6_col194 = blake_g_output_tmp_9aaf1_182[3].low().as_m31();
                *row[194] = blake_g_output_limb_6_col194;
                let blake_g_output_limb_7_col195 = blake_g_output_tmp_9aaf1_182[3].high().as_m31();
                *row[195] = blake_g_output_limb_7_col195;
                *lookup_data.blake_g_54 = [
                    M31_1139985212,
                    blake_g_output_limb_0_col156,
                    blake_g_output_limb_1_col157,
                    blake_g_output_limb_2_col166,
                    blake_g_output_limb_3_col167,
                    blake_g_output_limb_4_col176,
                    blake_g_output_limb_5_col177,
                    blake_g_output_limb_6_col154,
                    blake_g_output_limb_7_col155,
                    low_16_bits_col112,
                    high_16_bits_col113,
                    low_16_bits_col118,
                    high_16_bits_col119,
                    blake_g_output_limb_0_col188,
                    blake_g_output_limb_1_col189,
                    blake_g_output_limb_2_col190,
                    blake_g_output_limb_3_col191,
                    blake_g_output_limb_4_col192,
                    blake_g_output_limb_5_col193,
                    blake_g_output_limb_6_col194,
                    blake_g_output_limb_7_col195,
                ];
                *sub_component_inputs.blake_g[6] = [
                    blake_g_output_tmp_9aaf1_179[0],
                    blake_g_output_tmp_9aaf1_180[1],
                    blake_g_output_tmp_9aaf1_177[2],
                    blake_g_output_tmp_9aaf1_178[3],
                    read_u_32_output_tmp_9aaf1_143,
                    read_u_32_output_tmp_9aaf1_154,
                ];
                let blake_g_output_tmp_9aaf1_183 = PackedBlakeG::deduce_output([
                    blake_g_output_tmp_9aaf1_179[0],
                    blake_g_output_tmp_9aaf1_180[1],
                    blake_g_output_tmp_9aaf1_177[2],
                    blake_g_output_tmp_9aaf1_178[3],
                    read_u_32_output_tmp_9aaf1_143,
                    read_u_32_output_tmp_9aaf1_154,
                ]);
                let blake_g_output_limb_0_col196 = blake_g_output_tmp_9aaf1_183[0].low().as_m31();
                *row[196] = blake_g_output_limb_0_col196;
                let blake_g_output_limb_1_col197 = blake_g_output_tmp_9aaf1_183[0].high().as_m31();
                *row[197] = blake_g_output_limb_1_col197;
                let blake_g_output_limb_2_col198 = blake_g_output_tmp_9aaf1_183[1].low().as_m31();
                *row[198] = blake_g_output_limb_2_col198;
                let blake_g_output_limb_3_col199 = blake_g_output_tmp_9aaf1_183[1].high().as_m31();
                *row[199] = blake_g_output_limb_3_col199;
                let blake_g_output_limb_4_col200 = blake_g_output_tmp_9aaf1_183[2].low().as_m31();
                *row[200] = blake_g_output_limb_4_col200;
                let blake_g_output_limb_5_col201 = blake_g_output_tmp_9aaf1_183[2].high().as_m31();
                *row[201] = blake_g_output_limb_5_col201;
                let blake_g_output_limb_6_col202 = blake_g_output_tmp_9aaf1_183[3].low().as_m31();
                *row[202] = blake_g_output_limb_6_col202;
                let blake_g_output_limb_7_col203 = blake_g_output_tmp_9aaf1_183[3].high().as_m31();
                *row[203] = blake_g_output_limb_7_col203;
                *lookup_data.blake_g_55 = [
                    M31_1139985212,
                    blake_g_output_limb_0_col164,
                    blake_g_output_limb_1_col165,
                    blake_g_output_limb_2_col174,
                    blake_g_output_limb_3_col175,
                    blake_g_output_limb_4_col152,
                    blake_g_output_limb_5_col153,
                    blake_g_output_limb_6_col162,
                    blake_g_output_limb_7_col163,
                    low_16_bits_col124,
                    high_16_bits_col125,
                    low_16_bits_col130,
                    high_16_bits_col131,
                    blake_g_output_limb_0_col196,
                    blake_g_output_limb_1_col197,
                    blake_g_output_limb_2_col198,
                    blake_g_output_limb_3_col199,
                    blake_g_output_limb_4_col200,
                    blake_g_output_limb_5_col201,
                    blake_g_output_limb_6_col202,
                    blake_g_output_limb_7_col203,
                ];
                *sub_component_inputs.blake_g[7] = [
                    blake_g_output_tmp_9aaf1_180[0],
                    blake_g_output_tmp_9aaf1_177[1],
                    blake_g_output_tmp_9aaf1_178[2],
                    blake_g_output_tmp_9aaf1_179[3],
                    read_u_32_output_tmp_9aaf1_165,
                    read_u_32_output_tmp_9aaf1_176,
                ];
                let blake_g_output_tmp_9aaf1_184 = PackedBlakeG::deduce_output([
                    blake_g_output_tmp_9aaf1_180[0],
                    blake_g_output_tmp_9aaf1_177[1],
                    blake_g_output_tmp_9aaf1_178[2],
                    blake_g_output_tmp_9aaf1_179[3],
                    read_u_32_output_tmp_9aaf1_165,
                    read_u_32_output_tmp_9aaf1_176,
                ]);
                let blake_g_output_limb_0_col204 = blake_g_output_tmp_9aaf1_184[0].low().as_m31();
                *row[204] = blake_g_output_limb_0_col204;
                let blake_g_output_limb_1_col205 = blake_g_output_tmp_9aaf1_184[0].high().as_m31();
                *row[205] = blake_g_output_limb_1_col205;
                let blake_g_output_limb_2_col206 = blake_g_output_tmp_9aaf1_184[1].low().as_m31();
                *row[206] = blake_g_output_limb_2_col206;
                let blake_g_output_limb_3_col207 = blake_g_output_tmp_9aaf1_184[1].high().as_m31();
                *row[207] = blake_g_output_limb_3_col207;
                let blake_g_output_limb_4_col208 = blake_g_output_tmp_9aaf1_184[2].low().as_m31();
                *row[208] = blake_g_output_limb_4_col208;
                let blake_g_output_limb_5_col209 = blake_g_output_tmp_9aaf1_184[2].high().as_m31();
                *row[209] = blake_g_output_limb_5_col209;
                let blake_g_output_limb_6_col210 = blake_g_output_tmp_9aaf1_184[3].low().as_m31();
                *row[210] = blake_g_output_limb_6_col210;
                let blake_g_output_limb_7_col211 = blake_g_output_tmp_9aaf1_184[3].high().as_m31();
                *row[211] = blake_g_output_limb_7_col211;
                *lookup_data.blake_g_56 = [
                    M31_1139985212,
                    blake_g_output_limb_0_col172,
                    blake_g_output_limb_1_col173,
                    blake_g_output_limb_2_col150,
                    blake_g_output_limb_3_col151,
                    blake_g_output_limb_4_col160,
                    blake_g_output_limb_5_col161,
                    blake_g_output_limb_6_col170,
                    blake_g_output_limb_7_col171,
                    low_16_bits_col136,
                    high_16_bits_col137,
                    low_16_bits_col142,
                    high_16_bits_col143,
                    blake_g_output_limb_0_col204,
                    blake_g_output_limb_1_col205,
                    blake_g_output_limb_2_col206,
                    blake_g_output_limb_3_col207,
                    blake_g_output_limb_4_col208,
                    blake_g_output_limb_5_col209,
                    blake_g_output_limb_6_col210,
                    blake_g_output_limb_7_col211,
                ];
                *lookup_data.blake_round_57 = [
                    M31_40528774,
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
                ];
                *lookup_data.blake_round_58 = [
                    M31_40528774,
                    input_limb_0_col1,
                    ((input_limb_1_col2) + (M31_1)),
                    blake_g_output_limb_0_col180,
                    blake_g_output_limb_1_col181,
                    blake_g_output_limb_0_col188,
                    blake_g_output_limb_1_col189,
                    blake_g_output_limb_0_col196,
                    blake_g_output_limb_1_col197,
                    blake_g_output_limb_0_col204,
                    blake_g_output_limb_1_col205,
                    blake_g_output_limb_2_col206,
                    blake_g_output_limb_3_col207,
                    blake_g_output_limb_2_col182,
                    blake_g_output_limb_3_col183,
                    blake_g_output_limb_2_col190,
                    blake_g_output_limb_3_col191,
                    blake_g_output_limb_2_col198,
                    blake_g_output_limb_3_col199,
                    blake_g_output_limb_4_col200,
                    blake_g_output_limb_5_col201,
                    blake_g_output_limb_4_col208,
                    blake_g_output_limb_5_col209,
                    blake_g_output_limb_4_col184,
                    blake_g_output_limb_5_col185,
                    blake_g_output_limb_4_col192,
                    blake_g_output_limb_5_col193,
                    blake_g_output_limb_6_col194,
                    blake_g_output_limb_7_col195,
                    blake_g_output_limb_6_col202,
                    blake_g_output_limb_7_col203,
                    blake_g_output_limb_6_col210,
                    blake_g_output_limb_7_col211,
                    blake_g_output_limb_6_col186,
                    blake_g_output_limb_7_col187,
                    input_limb_34_col35,
                ];
                *lookup_data.mults_0 = enabler_col0;
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    blake_round_sigma_0: Vec<[PackedM31; 18]>,
    range_check_7_2_5_1: Vec<[PackedM31; 4]>,
    memory_address_to_id_2: Vec<[PackedM31; 3]>,
    memory_id_to_big_3: Vec<[PackedM31; 30]>,
    range_check_7_2_5_4: Vec<[PackedM31; 4]>,
    memory_address_to_id_5: Vec<[PackedM31; 3]>,
    memory_id_to_big_6: Vec<[PackedM31; 30]>,
    range_check_7_2_5_7: Vec<[PackedM31; 4]>,
    memory_address_to_id_8: Vec<[PackedM31; 3]>,
    memory_id_to_big_9: Vec<[PackedM31; 30]>,
    range_check_7_2_5_10: Vec<[PackedM31; 4]>,
    memory_address_to_id_11: Vec<[PackedM31; 3]>,
    memory_id_to_big_12: Vec<[PackedM31; 30]>,
    range_check_7_2_5_13: Vec<[PackedM31; 4]>,
    memory_address_to_id_14: Vec<[PackedM31; 3]>,
    memory_id_to_big_15: Vec<[PackedM31; 30]>,
    range_check_7_2_5_16: Vec<[PackedM31; 4]>,
    memory_address_to_id_17: Vec<[PackedM31; 3]>,
    memory_id_to_big_18: Vec<[PackedM31; 30]>,
    range_check_7_2_5_19: Vec<[PackedM31; 4]>,
    memory_address_to_id_20: Vec<[PackedM31; 3]>,
    memory_id_to_big_21: Vec<[PackedM31; 30]>,
    range_check_7_2_5_22: Vec<[PackedM31; 4]>,
    memory_address_to_id_23: Vec<[PackedM31; 3]>,
    memory_id_to_big_24: Vec<[PackedM31; 30]>,
    range_check_7_2_5_25: Vec<[PackedM31; 4]>,
    memory_address_to_id_26: Vec<[PackedM31; 3]>,
    memory_id_to_big_27: Vec<[PackedM31; 30]>,
    range_check_7_2_5_28: Vec<[PackedM31; 4]>,
    memory_address_to_id_29: Vec<[PackedM31; 3]>,
    memory_id_to_big_30: Vec<[PackedM31; 30]>,
    range_check_7_2_5_31: Vec<[PackedM31; 4]>,
    memory_address_to_id_32: Vec<[PackedM31; 3]>,
    memory_id_to_big_33: Vec<[PackedM31; 30]>,
    range_check_7_2_5_34: Vec<[PackedM31; 4]>,
    memory_address_to_id_35: Vec<[PackedM31; 3]>,
    memory_id_to_big_36: Vec<[PackedM31; 30]>,
    range_check_7_2_5_37: Vec<[PackedM31; 4]>,
    memory_address_to_id_38: Vec<[PackedM31; 3]>,
    memory_id_to_big_39: Vec<[PackedM31; 30]>,
    range_check_7_2_5_40: Vec<[PackedM31; 4]>,
    memory_address_to_id_41: Vec<[PackedM31; 3]>,
    memory_id_to_big_42: Vec<[PackedM31; 30]>,
    range_check_7_2_5_43: Vec<[PackedM31; 4]>,
    memory_address_to_id_44: Vec<[PackedM31; 3]>,
    memory_id_to_big_45: Vec<[PackedM31; 30]>,
    range_check_7_2_5_46: Vec<[PackedM31; 4]>,
    memory_address_to_id_47: Vec<[PackedM31; 3]>,
    memory_id_to_big_48: Vec<[PackedM31; 30]>,
    blake_g_49: Vec<[PackedM31; 21]>,
    blake_g_50: Vec<[PackedM31; 21]>,
    blake_g_51: Vec<[PackedM31; 21]>,
    blake_g_52: Vec<[PackedM31; 21]>,
    blake_g_53: Vec<[PackedM31; 21]>,
    blake_g_54: Vec<[PackedM31; 21]>,
    blake_g_55: Vec<[PackedM31; 21]>,
    blake_g_56: Vec<[PackedM31; 21]>,
    blake_round_57: Vec<[PackedM31; 36]>,
    blake_round_58: Vec<[PackedM31; 36]>,
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
            &self.lookup_data.blake_round_sigma_0,
            &self.lookup_data.range_check_7_2_5_1,
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
            &self.lookup_data.memory_address_to_id_2,
            &self.lookup_data.memory_id_to_big_3,
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
            &self.lookup_data.range_check_7_2_5_4,
            &self.lookup_data.memory_address_to_id_5,
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
            &self.lookup_data.memory_id_to_big_6,
            &self.lookup_data.range_check_7_2_5_7,
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
            &self.lookup_data.memory_address_to_id_8,
            &self.lookup_data.memory_id_to_big_9,
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
            &self.lookup_data.range_check_7_2_5_10,
            &self.lookup_data.memory_address_to_id_11,
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
            &self.lookup_data.memory_id_to_big_12,
            &self.lookup_data.range_check_7_2_5_13,
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
            &self.lookup_data.memory_address_to_id_14,
            &self.lookup_data.memory_id_to_big_15,
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
            &self.lookup_data.range_check_7_2_5_16,
            &self.lookup_data.memory_address_to_id_17,
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
            &self.lookup_data.memory_id_to_big_18,
            &self.lookup_data.range_check_7_2_5_19,
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
            &self.lookup_data.memory_address_to_id_20,
            &self.lookup_data.memory_id_to_big_21,
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
            &self.lookup_data.range_check_7_2_5_22,
            &self.lookup_data.memory_address_to_id_23,
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
            &self.lookup_data.memory_id_to_big_24,
            &self.lookup_data.range_check_7_2_5_25,
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
            &self.lookup_data.memory_address_to_id_26,
            &self.lookup_data.memory_id_to_big_27,
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
            &self.lookup_data.range_check_7_2_5_28,
            &self.lookup_data.memory_address_to_id_29,
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
            &self.lookup_data.memory_id_to_big_30,
            &self.lookup_data.range_check_7_2_5_31,
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
            &self.lookup_data.memory_address_to_id_32,
            &self.lookup_data.memory_id_to_big_33,
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
            &self.lookup_data.range_check_7_2_5_34,
            &self.lookup_data.memory_address_to_id_35,
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
            &self.lookup_data.memory_id_to_big_36,
            &self.lookup_data.range_check_7_2_5_37,
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
            &self.lookup_data.memory_address_to_id_38,
            &self.lookup_data.memory_id_to_big_39,
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
            &self.lookup_data.range_check_7_2_5_40,
            &self.lookup_data.memory_address_to_id_41,
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
            &self.lookup_data.memory_id_to_big_42,
            &self.lookup_data.range_check_7_2_5_43,
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
            &self.lookup_data.memory_address_to_id_44,
            &self.lookup_data.memory_id_to_big_45,
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
            &self.lookup_data.range_check_7_2_5_46,
            &self.lookup_data.memory_address_to_id_47,
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
            &self.lookup_data.memory_id_to_big_48,
            &self.lookup_data.blake_g_49,
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
            &self.lookup_data.blake_g_50,
            &self.lookup_data.blake_g_51,
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
            &self.lookup_data.blake_g_52,
            &self.lookup_data.blake_g_53,
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
            &self.lookup_data.blake_g_54,
            &self.lookup_data.blake_g_55,
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
            &self.lookup_data.blake_g_56,
            &self.lookup_data.blake_round_57,
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
        (col_gen.par_iter_mut(), &self.lookup_data.blake_round_58, self.lookup_data.mults_0)
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
