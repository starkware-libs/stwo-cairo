// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::jnz_opcode_taken::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{memory_address_to_id, memory_id_to_big, verify_instruction};
use crate::witness::prelude::*;

pub type InputType = CasmState;
pub type PackedInputType = PackedCasmState;

#[derive(Default)]
pub struct ClaimGenerator {
    pub inputs: Vec<InputType>,
}

impl ClaimGenerator {
    pub fn new(inputs: Vec<InputType>) -> Self {
        Self { inputs }
    }

    pub fn write_trace(
        mut self,
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
        verify_instruction_state: &verify_instruction::ClaimGenerator,
    ) -> (ComponentTrace<N_TRACE_COLUMNS>, Claim, InteractionClaimGenerator) {
        let n_active_rows = self.inputs.len();
        assert_ne!(n_active_rows, 0);
        let size = std::cmp::max(n_active_rows.next_power_of_two(), N_LANES);
        let log_size = size.ilog2();
        self.inputs.resize(size, *self.inputs.first().unwrap());
        let packed_inputs = pack_values(&self.inputs);

        let (trace, lookup_data, sub_component_inputs) = write_trace_simd(
            packed_inputs,
            n_active_rows,
            memory_address_to_id_state,
            memory_id_to_big_state,
            verify_instruction_state,
        );
        for inputs in sub_component_inputs.verify_instruction {
            add_inputs(verify_instruction_state, &inputs, n_active_rows, 0);
        }
        for inputs in sub_component_inputs.memory_address_to_id {
            add_inputs(memory_address_to_id_state, &inputs, n_active_rows, 0);
        }
        for inputs in sub_component_inputs.memory_id_to_big {
            add_inputs(memory_id_to_big_state, &inputs, n_active_rows, 0);
        }

        (trace, Claim { log_size }, InteractionClaimGenerator { log_size, lookup_data })
    }
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct SubComponentInputs {
    verify_instruction: [Vec<verify_instruction::PackedInputType>; 1],
    memory_address_to_id: [Vec<memory_address_to_id::PackedInputType>; 2],
    memory_id_to_big: [Vec<memory_id_to_big::PackedInputType>; 2],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    n_rows: usize,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
    verify_instruction_state: &verify_instruction::ClaimGenerator,
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
    let M31_134217728 = PackedM31::broadcast(M31::from(134217728));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_1444891767 = PackedM31::broadcast(M31::from(1444891767));
    let M31_16 = PackedM31::broadcast(M31::from(16));
    let M31_1662111297 = PackedM31::broadcast(M31::from(1662111297));
    let M31_1719106205 = PackedM31::broadcast(M31::from(1719106205));
    let M31_2147483646 = PackedM31::broadcast(M31::from(2147483646));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_32767 = PackedM31::broadcast(M31::from(32767));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_32769 = PackedM31::broadcast(M31::from(32769));
    let M31_428564188 = PackedM31::broadcast(M31::from(428564188));
    let M31_508 = PackedM31::broadcast(M31::from(508));
    let M31_511 = PackedM31::broadcast(M31::from(511));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let M31_536870912 = PackedM31::broadcast(M31::from(536870912));
    let M31_8 = PackedM31::broadcast(M31::from(8));
    let UInt16_0 = PackedUInt16::broadcast(UInt16::from(0));
    let UInt16_1 = PackedUInt16::broadcast(UInt16::from(1));
    let UInt16_11 = PackedUInt16::broadcast(UInt16::from(11));
    let UInt16_127 = PackedUInt16::broadcast(UInt16::from(127));
    let UInt16_2 = PackedUInt16::broadcast(UInt16::from(2));
    let UInt16_3 = PackedUInt16::broadcast(UInt16::from(3));
    let UInt16_6 = PackedUInt16::broadcast(UInt16::from(6));
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
            |(row_index, (row, lookup_data, sub_component_inputs, jnz_opcode_taken_input))| {
                let enabler_col0 = enabler_col.packed_at(row_index);
                *row[0] = enabler_col0;
                let input_pc_col1 = jnz_opcode_taken_input.pc;
                *row[1] = input_pc_col1;
                let input_ap_col2 = jnz_opcode_taken_input.ap;
                *row[2] = input_ap_col2;
                let input_fp_col3 = jnz_opcode_taken_input.fp;
                *row[3] = input_fp_col3;

                // Decode Instruction.

                let memory_address_to_id_value_tmp_7f087_0 =
                    memory_address_to_id_state.deduce_output(input_pc_col1);
                let memory_id_to_big_value_tmp_7f087_1 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_7f087_0);
                let offset0_tmp_7f087_2 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_7f087_1.get_m31(0)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_7f087_1.get_m31(1),
                        )) & (UInt16_127))
                            << (UInt16_9)));
                let offset0_col4 = offset0_tmp_7f087_2.as_m31();
                *row[4] = offset0_col4;
                let dst_base_fp_tmp_7f087_3 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_7f087_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_7f087_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_0))
                        & (UInt16_1));
                let dst_base_fp_col5 = dst_base_fp_tmp_7f087_3.as_m31();
                *row[5] = dst_base_fp_col5;
                let ap_update_add_1_tmp_7f087_4 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_7f087_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_7f087_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_11))
                        & (UInt16_1));
                let ap_update_add_1_col6 = ap_update_add_1_tmp_7f087_4.as_m31();
                *row[6] = ap_update_add_1_col6;
                *sub_component_inputs.verify_instruction[0] = (
                    input_pc_col1,
                    [offset0_col4, M31_32767, M31_32769],
                    [
                        ((((dst_base_fp_col5) * (M31_8)) + (M31_16)) + (M31_32)),
                        ((M31_8) + ((ap_update_add_1_col6) * (M31_32))),
                    ],
                    M31_0,
                );
                *lookup_data.verify_instruction_0 = [
                    M31_1719106205,
                    input_pc_col1,
                    offset0_col4,
                    M31_32767,
                    M31_32769,
                    ((((dst_base_fp_col5) * (M31_8)) + (M31_16)) + (M31_32)),
                    ((M31_8) + ((ap_update_add_1_col6) * (M31_32))),
                    M31_0,
                ];
                let decode_instruction_ad440_output_tmp_7f087_5 = (
                    [((offset0_col4) - (M31_32768)), M31_2147483646, M31_1],
                    [
                        dst_base_fp_col5,
                        M31_1,
                        M31_1,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_1,
                        M31_0,
                        ap_update_add_1_col6,
                        M31_0,
                        M31_0,
                        M31_0,
                    ],
                    M31_0,
                );

                let mem_dst_base_col7 = (((dst_base_fp_col5) * (input_fp_col3))
                    + (((M31_1) - (dst_base_fp_col5)) * (input_ap_col2)));
                *row[7] = mem_dst_base_col7;

                // Read Positive Num Bits 252.

                // Read Id.

                let memory_address_to_id_value_tmp_7f087_6 = memory_address_to_id_state
                    .deduce_output(
                        ((mem_dst_base_col7) + (decode_instruction_ad440_output_tmp_7f087_5.0[0])),
                    );
                let dst_id_col8 = memory_address_to_id_value_tmp_7f087_6;
                *row[8] = dst_id_col8;
                *sub_component_inputs.memory_address_to_id[0] =
                    ((mem_dst_base_col7) + (decode_instruction_ad440_output_tmp_7f087_5.0[0]));
                *lookup_data.memory_address_to_id_1 = [
                    M31_1444891767,
                    ((mem_dst_base_col7) + (decode_instruction_ad440_output_tmp_7f087_5.0[0])),
                    dst_id_col8,
                ];

                // Read Positive Known Id Num Bits 252.

                let memory_id_to_big_value_tmp_7f087_8 =
                    memory_id_to_big_state.deduce_output(dst_id_col8);
                let dst_limb_0_col9 = memory_id_to_big_value_tmp_7f087_8.get_m31(0);
                *row[9] = dst_limb_0_col9;
                let dst_limb_1_col10 = memory_id_to_big_value_tmp_7f087_8.get_m31(1);
                *row[10] = dst_limb_1_col10;
                let dst_limb_2_col11 = memory_id_to_big_value_tmp_7f087_8.get_m31(2);
                *row[11] = dst_limb_2_col11;
                let dst_limb_3_col12 = memory_id_to_big_value_tmp_7f087_8.get_m31(3);
                *row[12] = dst_limb_3_col12;
                let dst_limb_4_col13 = memory_id_to_big_value_tmp_7f087_8.get_m31(4);
                *row[13] = dst_limb_4_col13;
                let dst_limb_5_col14 = memory_id_to_big_value_tmp_7f087_8.get_m31(5);
                *row[14] = dst_limb_5_col14;
                let dst_limb_6_col15 = memory_id_to_big_value_tmp_7f087_8.get_m31(6);
                *row[15] = dst_limb_6_col15;
                let dst_limb_7_col16 = memory_id_to_big_value_tmp_7f087_8.get_m31(7);
                *row[16] = dst_limb_7_col16;
                let dst_limb_8_col17 = memory_id_to_big_value_tmp_7f087_8.get_m31(8);
                *row[17] = dst_limb_8_col17;
                let dst_limb_9_col18 = memory_id_to_big_value_tmp_7f087_8.get_m31(9);
                *row[18] = dst_limb_9_col18;
                let dst_limb_10_col19 = memory_id_to_big_value_tmp_7f087_8.get_m31(10);
                *row[19] = dst_limb_10_col19;
                let dst_limb_11_col20 = memory_id_to_big_value_tmp_7f087_8.get_m31(11);
                *row[20] = dst_limb_11_col20;
                let dst_limb_12_col21 = memory_id_to_big_value_tmp_7f087_8.get_m31(12);
                *row[21] = dst_limb_12_col21;
                let dst_limb_13_col22 = memory_id_to_big_value_tmp_7f087_8.get_m31(13);
                *row[22] = dst_limb_13_col22;
                let dst_limb_14_col23 = memory_id_to_big_value_tmp_7f087_8.get_m31(14);
                *row[23] = dst_limb_14_col23;
                let dst_limb_15_col24 = memory_id_to_big_value_tmp_7f087_8.get_m31(15);
                *row[24] = dst_limb_15_col24;
                let dst_limb_16_col25 = memory_id_to_big_value_tmp_7f087_8.get_m31(16);
                *row[25] = dst_limb_16_col25;
                let dst_limb_17_col26 = memory_id_to_big_value_tmp_7f087_8.get_m31(17);
                *row[26] = dst_limb_17_col26;
                let dst_limb_18_col27 = memory_id_to_big_value_tmp_7f087_8.get_m31(18);
                *row[27] = dst_limb_18_col27;
                let dst_limb_19_col28 = memory_id_to_big_value_tmp_7f087_8.get_m31(19);
                *row[28] = dst_limb_19_col28;
                let dst_limb_20_col29 = memory_id_to_big_value_tmp_7f087_8.get_m31(20);
                *row[29] = dst_limb_20_col29;
                let dst_limb_21_col30 = memory_id_to_big_value_tmp_7f087_8.get_m31(21);
                *row[30] = dst_limb_21_col30;
                let dst_limb_22_col31 = memory_id_to_big_value_tmp_7f087_8.get_m31(22);
                *row[31] = dst_limb_22_col31;
                let dst_limb_23_col32 = memory_id_to_big_value_tmp_7f087_8.get_m31(23);
                *row[32] = dst_limb_23_col32;
                let dst_limb_24_col33 = memory_id_to_big_value_tmp_7f087_8.get_m31(24);
                *row[33] = dst_limb_24_col33;
                let dst_limb_25_col34 = memory_id_to_big_value_tmp_7f087_8.get_m31(25);
                *row[34] = dst_limb_25_col34;
                let dst_limb_26_col35 = memory_id_to_big_value_tmp_7f087_8.get_m31(26);
                *row[35] = dst_limb_26_col35;
                let dst_limb_27_col36 = memory_id_to_big_value_tmp_7f087_8.get_m31(27);
                *row[36] = dst_limb_27_col36;
                *sub_component_inputs.memory_id_to_big[0] = dst_id_col8;
                *lookup_data.memory_id_to_big_2 = [
                    M31_1662111297,
                    dst_id_col8,
                    dst_limb_0_col9,
                    dst_limb_1_col10,
                    dst_limb_2_col11,
                    dst_limb_3_col12,
                    dst_limb_4_col13,
                    dst_limb_5_col14,
                    dst_limb_6_col15,
                    dst_limb_7_col16,
                    dst_limb_8_col17,
                    dst_limb_9_col18,
                    dst_limb_10_col19,
                    dst_limb_11_col20,
                    dst_limb_12_col21,
                    dst_limb_13_col22,
                    dst_limb_14_col23,
                    dst_limb_15_col24,
                    dst_limb_16_col25,
                    dst_limb_17_col26,
                    dst_limb_18_col27,
                    dst_limb_19_col28,
                    dst_limb_20_col29,
                    dst_limb_21_col30,
                    dst_limb_22_col31,
                    dst_limb_23_col32,
                    dst_limb_24_col33,
                    dst_limb_25_col34,
                    dst_limb_26_col35,
                    dst_limb_27_col36,
                ];
                let read_positive_known_id_num_bits_252_output_tmp_7f087_9 =
                    PackedFelt252::from_limbs([
                        dst_limb_0_col9,
                        dst_limb_1_col10,
                        dst_limb_2_col11,
                        dst_limb_3_col12,
                        dst_limb_4_col13,
                        dst_limb_5_col14,
                        dst_limb_6_col15,
                        dst_limb_7_col16,
                        dst_limb_8_col17,
                        dst_limb_9_col18,
                        dst_limb_10_col19,
                        dst_limb_11_col20,
                        dst_limb_12_col21,
                        dst_limb_13_col22,
                        dst_limb_14_col23,
                        dst_limb_15_col24,
                        dst_limb_16_col25,
                        dst_limb_17_col26,
                        dst_limb_18_col27,
                        dst_limb_19_col28,
                        dst_limb_20_col29,
                        dst_limb_21_col30,
                        dst_limb_22_col31,
                        dst_limb_23_col32,
                        dst_limb_24_col33,
                        dst_limb_25_col34,
                        dst_limb_26_col35,
                        dst_limb_27_col36,
                    ]);

                let read_positive_num_bits_252_output_tmp_7f087_10 =
                    (read_positive_known_id_num_bits_252_output_tmp_7f087_9, dst_id_col8);

                let dst_sum_p_zero_tmp_7f087_11 = (((((((((((((((((((((((((dst_limb_1_col10)
                    + (dst_limb_2_col11))
                    + (dst_limb_3_col12))
                    + (dst_limb_4_col13))
                    + (dst_limb_5_col14))
                    + (dst_limb_6_col15))
                    + (dst_limb_7_col16))
                    + (dst_limb_8_col17))
                    + (dst_limb_9_col18))
                    + (dst_limb_10_col19))
                    + (dst_limb_11_col20))
                    + (dst_limb_12_col21))
                    + (dst_limb_13_col22))
                    + (dst_limb_14_col23))
                    + (dst_limb_15_col24))
                    + (dst_limb_16_col25))
                    + (dst_limb_17_col26))
                    + (dst_limb_18_col27))
                    + (dst_limb_19_col28))
                    + (dst_limb_20_col29))
                    + (dst_limb_22_col31))
                    + (dst_limb_23_col32))
                    + (dst_limb_24_col33))
                    + (dst_limb_25_col34))
                    + (dst_limb_26_col35));
                let dst_sum_inv_col37 = ((dst_sum_p_zero_tmp_7f087_11)
                    + (((dst_limb_0_col9) + (dst_limb_21_col30)) + (dst_limb_27_col36)))
                    .inverse();
                *row[37] = dst_sum_inv_col37;
                let diff_from_p_tmp_7f087_12 = ((dst_limb_0_col9) - (M31_1));
                let diff_from_p_tmp_7f087_13 = ((dst_limb_21_col30) - (M31_136));
                let diff_from_p_tmp_7f087_14 = ((dst_limb_27_col36) - (M31_256));
                let dst_sum_squares_inv_col38 = ((dst_sum_p_zero_tmp_7f087_11)
                    + ((((diff_from_p_tmp_7f087_12) * (diff_from_p_tmp_7f087_12))
                        + ((diff_from_p_tmp_7f087_13) * (diff_from_p_tmp_7f087_13)))
                        + ((diff_from_p_tmp_7f087_14) * (diff_from_p_tmp_7f087_14))))
                    .inverse();
                *row[38] = dst_sum_squares_inv_col38;

                // Read Small.

                // Read Id.

                let memory_address_to_id_value_tmp_7f087_15 =
                    memory_address_to_id_state.deduce_output(((input_pc_col1) + (M31_1)));
                let next_pc_id_col39 = memory_address_to_id_value_tmp_7f087_15;
                *row[39] = next_pc_id_col39;
                *sub_component_inputs.memory_address_to_id[1] = ((input_pc_col1) + (M31_1));
                *lookup_data.memory_address_to_id_3 =
                    [M31_1444891767, ((input_pc_col1) + (M31_1)), next_pc_id_col39];

                let memory_id_to_big_value_tmp_7f087_17 =
                    memory_id_to_big_state.deduce_output(next_pc_id_col39);

                // Decode Small Sign.

                let msb_tmp_7f087_18 = memory_id_to_big_value_tmp_7f087_17.get_m31(27).eq(M31_256);
                let msb_col40 = msb_tmp_7f087_18.as_m31();
                *row[40] = msb_col40;
                let mid_limbs_set_tmp_7f087_19 =
                    ((memory_id_to_big_value_tmp_7f087_17.get_m31(20).eq(M31_511))
                        & (msb_tmp_7f087_18));
                let mid_limbs_set_col41 = mid_limbs_set_tmp_7f087_19.as_m31();
                *row[41] = mid_limbs_set_col41;
                let decode_small_sign_output_tmp_7f087_20 = [
                    msb_col40,
                    mid_limbs_set_col41,
                    ((mid_limbs_set_col41) * (M31_508)),
                    ((mid_limbs_set_col41) * (M31_511)),
                    (((msb_col40) * (M31_136)) - (mid_limbs_set_col41)),
                    ((msb_col40) * (M31_256)),
                ];

                let next_pc_limb_0_col42 = memory_id_to_big_value_tmp_7f087_17.get_m31(0);
                *row[42] = next_pc_limb_0_col42;
                let next_pc_limb_1_col43 = memory_id_to_big_value_tmp_7f087_17.get_m31(1);
                *row[43] = next_pc_limb_1_col43;
                let next_pc_limb_2_col44 = memory_id_to_big_value_tmp_7f087_17.get_m31(2);
                *row[44] = next_pc_limb_2_col44;
                let remainder_bits_tmp_7f087_21 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_7f087_17.get_m31(3)))
                        & (UInt16_3));
                let remainder_bits_col45 = remainder_bits_tmp_7f087_21.as_m31();
                *row[45] = remainder_bits_col45;

                // Cond Range Check 2.

                let partial_limb_msb_tmp_7f087_22 =
                    (((PackedUInt16::from_m31(remainder_bits_col45)) & (UInt16_2)) >> (UInt16_1));
                let partial_limb_msb_col46 = partial_limb_msb_tmp_7f087_22.as_m31();
                *row[46] = partial_limb_msb_col46;

                *sub_component_inputs.memory_id_to_big[1] = next_pc_id_col39;
                *lookup_data.memory_id_to_big_4 = [
                    M31_1662111297,
                    next_pc_id_col39,
                    next_pc_limb_0_col42,
                    next_pc_limb_1_col43,
                    next_pc_limb_2_col44,
                    ((remainder_bits_col45) + (decode_small_sign_output_tmp_7f087_20[2])),
                    decode_small_sign_output_tmp_7f087_20[3],
                    decode_small_sign_output_tmp_7f087_20[3],
                    decode_small_sign_output_tmp_7f087_20[3],
                    decode_small_sign_output_tmp_7f087_20[3],
                    decode_small_sign_output_tmp_7f087_20[3],
                    decode_small_sign_output_tmp_7f087_20[3],
                    decode_small_sign_output_tmp_7f087_20[3],
                    decode_small_sign_output_tmp_7f087_20[3],
                    decode_small_sign_output_tmp_7f087_20[3],
                    decode_small_sign_output_tmp_7f087_20[3],
                    decode_small_sign_output_tmp_7f087_20[3],
                    decode_small_sign_output_tmp_7f087_20[3],
                    decode_small_sign_output_tmp_7f087_20[3],
                    decode_small_sign_output_tmp_7f087_20[3],
                    decode_small_sign_output_tmp_7f087_20[3],
                    decode_small_sign_output_tmp_7f087_20[3],
                    decode_small_sign_output_tmp_7f087_20[3],
                    decode_small_sign_output_tmp_7f087_20[4],
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    decode_small_sign_output_tmp_7f087_20[5],
                ];
                let read_small_output_tmp_7f087_24 = (
                    ((((((next_pc_limb_0_col42) + ((next_pc_limb_1_col43) * (M31_512)))
                        + ((next_pc_limb_2_col44) * (M31_262144)))
                        + ((remainder_bits_col45) * (M31_134217728)))
                        - (msb_col40))
                        - ((M31_536870912) * (mid_limbs_set_col41))),
                    next_pc_id_col39,
                );

                *lookup_data.opcodes_5 =
                    [M31_428564188, input_pc_col1, input_ap_col2, input_fp_col3];
                *lookup_data.opcodes_6 = [
                    M31_428564188,
                    ((input_pc_col1) + (read_small_output_tmp_7f087_24.0)),
                    ((input_ap_col2) + (ap_update_add_1_col6)),
                    input_fp_col3,
                ];
                *lookup_data.mults_0 = enabler_col0;
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    verify_instruction_0: Vec<[PackedM31; 8]>,
    memory_address_to_id_1: Vec<[PackedM31; 3]>,
    memory_id_to_big_2: Vec<[PackedM31; 30]>,
    memory_address_to_id_3: Vec<[PackedM31; 3]>,
    memory_id_to_big_4: Vec<[PackedM31; 30]>,
    opcodes_5: Vec<[PackedM31; 4]>,
    opcodes_6: Vec<[PackedM31; 4]>,
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
            &self.lookup_data.verify_instruction_0,
            &self.lookup_data.memory_address_to_id_1,
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
            &self.lookup_data.memory_id_to_big_2,
            &self.lookup_data.memory_address_to_id_3,
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
            &self.lookup_data.memory_id_to_big_4,
            &self.lookup_data.opcodes_5,
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
        (col_gen.par_iter_mut(), &self.lookup_data.opcodes_6, self.lookup_data.mults_0)
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
