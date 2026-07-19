// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::add_opcode_small::{Claim, InteractionClaim, N_TRACE_COLUMNS};

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
    memory_address_to_id: [Vec<memory_address_to_id::PackedInputType>; 3],
    memory_id_to_big: [Vec<memory_id_to_big::PackedInputType>; 3],
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
    let M31_128 = PackedM31::broadcast(M31::from(128));
    let M31_134217728 = PackedM31::broadcast(M31::from(134217728));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_1444891767 = PackedM31::broadcast(M31::from(1444891767));
    let M31_16 = PackedM31::broadcast(M31::from(16));
    let M31_1662111297 = PackedM31::broadcast(M31::from(1662111297));
    let M31_1719106205 = PackedM31::broadcast(M31::from(1719106205));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_428564188 = PackedM31::broadcast(M31::from(428564188));
    let M31_508 = PackedM31::broadcast(M31::from(508));
    let M31_511 = PackedM31::broadcast(M31::from(511));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let M31_536870912 = PackedM31::broadcast(M31::from(536870912));
    let M31_64 = PackedM31::broadcast(M31::from(64));
    let M31_8 = PackedM31::broadcast(M31::from(8));
    let UInt16_0 = PackedUInt16::broadcast(UInt16::from(0));
    let UInt16_1 = PackedUInt16::broadcast(UInt16::from(1));
    let UInt16_11 = PackedUInt16::broadcast(UInt16::from(11));
    let UInt16_127 = PackedUInt16::broadcast(UInt16::from(127));
    let UInt16_13 = PackedUInt16::broadcast(UInt16::from(13));
    let UInt16_2 = PackedUInt16::broadcast(UInt16::from(2));
    let UInt16_3 = PackedUInt16::broadcast(UInt16::from(3));
    let UInt16_31 = PackedUInt16::broadcast(UInt16::from(31));
    let UInt16_4 = PackedUInt16::broadcast(UInt16::from(4));
    let UInt16_5 = PackedUInt16::broadcast(UInt16::from(5));
    let UInt16_6 = PackedUInt16::broadcast(UInt16::from(6));
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
            |(row_index, (row, lookup_data, sub_component_inputs, add_opcode_small_input))| {
                let enabler_col0 = enabler_col.packed_at(row_index);
                *row[0] = enabler_col0;
                let input_pc_col1 = add_opcode_small_input.pc;
                *row[1] = input_pc_col1;
                let input_ap_col2 = add_opcode_small_input.ap;
                *row[2] = input_ap_col2;
                let input_fp_col3 = add_opcode_small_input.fp;
                *row[3] = input_fp_col3;

                // Decode Instruction.

                let memory_address_to_id_value_tmp_e5099_0 =
                    memory_address_to_id_state.deduce_output(input_pc_col1);
                let memory_id_to_big_value_tmp_e5099_1 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_e5099_0);
                let offset0_tmp_e5099_2 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_e5099_1.get_m31(0)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_e5099_1.get_m31(1),
                        )) & (UInt16_127))
                            << (UInt16_9)));
                let offset0_col4 = offset0_tmp_e5099_2.as_m31();
                *row[4] = offset0_col4;
                let offset1_tmp_e5099_3 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_e5099_1.get_m31(1)))
                        >> (UInt16_7))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_e5099_1.get_m31(2),
                        )) << (UInt16_2)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_e5099_1.get_m31(3),
                        )) & (UInt16_31))
                            << (UInt16_11)));
                let offset1_col5 = offset1_tmp_e5099_3.as_m31();
                *row[5] = offset1_col5;
                let offset2_tmp_e5099_4 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_e5099_1.get_m31(3)))
                        >> (UInt16_5))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_e5099_1.get_m31(4),
                        )) << (UInt16_4)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_e5099_1.get_m31(5),
                        )) & (UInt16_7))
                            << (UInt16_13)));
                let offset2_col6 = offset2_tmp_e5099_4.as_m31();
                *row[6] = offset2_col6;
                let dst_base_fp_tmp_e5099_5 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_e5099_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_e5099_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_0))
                        & (UInt16_1));
                let dst_base_fp_col7 = dst_base_fp_tmp_e5099_5.as_m31();
                *row[7] = dst_base_fp_col7;
                let op0_base_fp_tmp_e5099_6 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_e5099_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_e5099_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_1))
                        & (UInt16_1));
                let op0_base_fp_col8 = op0_base_fp_tmp_e5099_6.as_m31();
                *row[8] = op0_base_fp_col8;
                let op1_imm_tmp_e5099_7 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_e5099_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_e5099_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_2))
                        & (UInt16_1));
                let op1_imm_col9 = op1_imm_tmp_e5099_7.as_m31();
                *row[9] = op1_imm_col9;
                let op1_base_fp_tmp_e5099_8 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_e5099_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_e5099_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_3))
                        & (UInt16_1));
                let op1_base_fp_col10 = op1_base_fp_tmp_e5099_8.as_m31();
                *row[10] = op1_base_fp_col10;
                let op1_base_ap_tmp_e5099_9 = (((M31_1) - (op1_imm_col9)) - (op1_base_fp_col10));
                let ap_update_add_1_tmp_e5099_10 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_e5099_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_e5099_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_11))
                        & (UInt16_1));
                let ap_update_add_1_col11 = ap_update_add_1_tmp_e5099_10.as_m31();
                *row[11] = ap_update_add_1_col11;
                *sub_component_inputs.verify_instruction[0] = (
                    input_pc_col1,
                    [offset0_col4, offset1_col5, offset2_col6],
                    [
                        (((((((dst_base_fp_col7) * (M31_8)) + ((op0_base_fp_col8) * (M31_16)))
                            + ((op1_imm_col9) * (M31_32)))
                            + ((op1_base_fp_col10) * (M31_64)))
                            + ((op1_base_ap_tmp_e5099_9) * (M31_128)))
                            + (M31_256)),
                        (((ap_update_add_1_col11) * (M31_32)) + (M31_256)),
                    ],
                    M31_0,
                );
                *lookup_data.verify_instruction_0 = [
                    M31_1719106205,
                    input_pc_col1,
                    offset0_col4,
                    offset1_col5,
                    offset2_col6,
                    (((((((dst_base_fp_col7) * (M31_8)) + ((op0_base_fp_col8) * (M31_16)))
                        + ((op1_imm_col9) * (M31_32)))
                        + ((op1_base_fp_col10) * (M31_64)))
                        + ((op1_base_ap_tmp_e5099_9) * (M31_128)))
                        + (M31_256)),
                    (((ap_update_add_1_col11) * (M31_32)) + (M31_256)),
                    M31_0,
                ];
                let decode_instruction_7785f_output_tmp_e5099_11 = (
                    [
                        ((offset0_col4) - (M31_32768)),
                        ((offset1_col5) - (M31_32768)),
                        ((offset2_col6) - (M31_32768)),
                    ],
                    [
                        dst_base_fp_col7,
                        op0_base_fp_col8,
                        op1_imm_col9,
                        op1_base_fp_col10,
                        op1_base_ap_tmp_e5099_9,
                        M31_1,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        ap_update_add_1_col11,
                        M31_0,
                        M31_0,
                        M31_1,
                    ],
                    M31_0,
                );

                let mem_dst_base_col12 = (((dst_base_fp_col7) * (input_fp_col3))
                    + (((M31_1) - (dst_base_fp_col7)) * (input_ap_col2)));
                *row[12] = mem_dst_base_col12;
                let mem0_base_col13 = (((op0_base_fp_col8) * (input_fp_col3))
                    + (((M31_1) - (op0_base_fp_col8)) * (input_ap_col2)));
                *row[13] = mem0_base_col13;
                let mem1_base_col14 = ((((op1_imm_col9) * (input_pc_col1))
                    + ((op1_base_fp_col10) * (input_fp_col3)))
                    + ((decode_instruction_7785f_output_tmp_e5099_11.1[4]) * (input_ap_col2)));
                *row[14] = mem1_base_col14;

                // Read Small.

                // Read Id.

                let memory_address_to_id_value_tmp_e5099_12 = memory_address_to_id_state
                    .deduce_output(
                        ((mem_dst_base_col12)
                            + (decode_instruction_7785f_output_tmp_e5099_11.0[0])),
                    );
                let dst_id_col15 = memory_address_to_id_value_tmp_e5099_12;
                *row[15] = dst_id_col15;
                *sub_component_inputs.memory_address_to_id[0] =
                    ((mem_dst_base_col12) + (decode_instruction_7785f_output_tmp_e5099_11.0[0]));
                *lookup_data.memory_address_to_id_1 = [
                    M31_1444891767,
                    ((mem_dst_base_col12) + (decode_instruction_7785f_output_tmp_e5099_11.0[0])),
                    dst_id_col15,
                ];

                let memory_id_to_big_value_tmp_e5099_14 =
                    memory_id_to_big_state.deduce_output(dst_id_col15);

                // Decode Small Sign.

                let msb_tmp_e5099_15 = memory_id_to_big_value_tmp_e5099_14.get_m31(27).eq(M31_256);
                let msb_col16 = msb_tmp_e5099_15.as_m31();
                *row[16] = msb_col16;
                let mid_limbs_set_tmp_e5099_16 =
                    ((memory_id_to_big_value_tmp_e5099_14.get_m31(20).eq(M31_511))
                        & (msb_tmp_e5099_15));
                let mid_limbs_set_col17 = mid_limbs_set_tmp_e5099_16.as_m31();
                *row[17] = mid_limbs_set_col17;
                let decode_small_sign_output_tmp_e5099_17 = [
                    msb_col16,
                    mid_limbs_set_col17,
                    ((mid_limbs_set_col17) * (M31_508)),
                    ((mid_limbs_set_col17) * (M31_511)),
                    (((msb_col16) * (M31_136)) - (mid_limbs_set_col17)),
                    ((msb_col16) * (M31_256)),
                ];

                let dst_limb_0_col18 = memory_id_to_big_value_tmp_e5099_14.get_m31(0);
                *row[18] = dst_limb_0_col18;
                let dst_limb_1_col19 = memory_id_to_big_value_tmp_e5099_14.get_m31(1);
                *row[19] = dst_limb_1_col19;
                let dst_limb_2_col20 = memory_id_to_big_value_tmp_e5099_14.get_m31(2);
                *row[20] = dst_limb_2_col20;
                let remainder_bits_tmp_e5099_18 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_e5099_14.get_m31(3)))
                        & (UInt16_3));
                let remainder_bits_col21 = remainder_bits_tmp_e5099_18.as_m31();
                *row[21] = remainder_bits_col21;

                // Cond Range Check 2.

                let partial_limb_msb_tmp_e5099_19 =
                    (((PackedUInt16::from_m31(remainder_bits_col21)) & (UInt16_2)) >> (UInt16_1));
                let partial_limb_msb_col22 = partial_limb_msb_tmp_e5099_19.as_m31();
                *row[22] = partial_limb_msb_col22;

                *sub_component_inputs.memory_id_to_big[0] = dst_id_col15;
                *lookup_data.memory_id_to_big_2 = [
                    M31_1662111297,
                    dst_id_col15,
                    dst_limb_0_col18,
                    dst_limb_1_col19,
                    dst_limb_2_col20,
                    ((remainder_bits_col21) + (decode_small_sign_output_tmp_e5099_17[2])),
                    decode_small_sign_output_tmp_e5099_17[3],
                    decode_small_sign_output_tmp_e5099_17[3],
                    decode_small_sign_output_tmp_e5099_17[3],
                    decode_small_sign_output_tmp_e5099_17[3],
                    decode_small_sign_output_tmp_e5099_17[3],
                    decode_small_sign_output_tmp_e5099_17[3],
                    decode_small_sign_output_tmp_e5099_17[3],
                    decode_small_sign_output_tmp_e5099_17[3],
                    decode_small_sign_output_tmp_e5099_17[3],
                    decode_small_sign_output_tmp_e5099_17[3],
                    decode_small_sign_output_tmp_e5099_17[3],
                    decode_small_sign_output_tmp_e5099_17[3],
                    decode_small_sign_output_tmp_e5099_17[3],
                    decode_small_sign_output_tmp_e5099_17[3],
                    decode_small_sign_output_tmp_e5099_17[3],
                    decode_small_sign_output_tmp_e5099_17[3],
                    decode_small_sign_output_tmp_e5099_17[3],
                    decode_small_sign_output_tmp_e5099_17[4],
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    decode_small_sign_output_tmp_e5099_17[5],
                ];
                let read_small_output_tmp_e5099_21 = (
                    ((((((dst_limb_0_col18) + ((dst_limb_1_col19) * (M31_512)))
                        + ((dst_limb_2_col20) * (M31_262144)))
                        + ((remainder_bits_col21) * (M31_134217728)))
                        - (msb_col16))
                        - ((M31_536870912) * (mid_limbs_set_col17))),
                    dst_id_col15,
                );

                // Read Small.

                // Read Id.

                let memory_address_to_id_value_tmp_e5099_22 = memory_address_to_id_state
                    .deduce_output(
                        ((mem0_base_col13) + (decode_instruction_7785f_output_tmp_e5099_11.0[1])),
                    );
                let op0_id_col23 = memory_address_to_id_value_tmp_e5099_22;
                *row[23] = op0_id_col23;
                *sub_component_inputs.memory_address_to_id[1] =
                    ((mem0_base_col13) + (decode_instruction_7785f_output_tmp_e5099_11.0[1]));
                *lookup_data.memory_address_to_id_3 = [
                    M31_1444891767,
                    ((mem0_base_col13) + (decode_instruction_7785f_output_tmp_e5099_11.0[1])),
                    op0_id_col23,
                ];

                let memory_id_to_big_value_tmp_e5099_24 =
                    memory_id_to_big_state.deduce_output(op0_id_col23);

                // Decode Small Sign.

                let msb_tmp_e5099_25 = memory_id_to_big_value_tmp_e5099_24.get_m31(27).eq(M31_256);
                let msb_col24 = msb_tmp_e5099_25.as_m31();
                *row[24] = msb_col24;
                let mid_limbs_set_tmp_e5099_26 =
                    ((memory_id_to_big_value_tmp_e5099_24.get_m31(20).eq(M31_511))
                        & (msb_tmp_e5099_25));
                let mid_limbs_set_col25 = mid_limbs_set_tmp_e5099_26.as_m31();
                *row[25] = mid_limbs_set_col25;
                let decode_small_sign_output_tmp_e5099_27 = [
                    msb_col24,
                    mid_limbs_set_col25,
                    ((mid_limbs_set_col25) * (M31_508)),
                    ((mid_limbs_set_col25) * (M31_511)),
                    (((msb_col24) * (M31_136)) - (mid_limbs_set_col25)),
                    ((msb_col24) * (M31_256)),
                ];

                let op0_limb_0_col26 = memory_id_to_big_value_tmp_e5099_24.get_m31(0);
                *row[26] = op0_limb_0_col26;
                let op0_limb_1_col27 = memory_id_to_big_value_tmp_e5099_24.get_m31(1);
                *row[27] = op0_limb_1_col27;
                let op0_limb_2_col28 = memory_id_to_big_value_tmp_e5099_24.get_m31(2);
                *row[28] = op0_limb_2_col28;
                let remainder_bits_tmp_e5099_28 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_e5099_24.get_m31(3)))
                        & (UInt16_3));
                let remainder_bits_col29 = remainder_bits_tmp_e5099_28.as_m31();
                *row[29] = remainder_bits_col29;

                // Cond Range Check 2.

                let partial_limb_msb_tmp_e5099_29 =
                    (((PackedUInt16::from_m31(remainder_bits_col29)) & (UInt16_2)) >> (UInt16_1));
                let partial_limb_msb_col30 = partial_limb_msb_tmp_e5099_29.as_m31();
                *row[30] = partial_limb_msb_col30;

                *sub_component_inputs.memory_id_to_big[1] = op0_id_col23;
                *lookup_data.memory_id_to_big_4 = [
                    M31_1662111297,
                    op0_id_col23,
                    op0_limb_0_col26,
                    op0_limb_1_col27,
                    op0_limb_2_col28,
                    ((remainder_bits_col29) + (decode_small_sign_output_tmp_e5099_27[2])),
                    decode_small_sign_output_tmp_e5099_27[3],
                    decode_small_sign_output_tmp_e5099_27[3],
                    decode_small_sign_output_tmp_e5099_27[3],
                    decode_small_sign_output_tmp_e5099_27[3],
                    decode_small_sign_output_tmp_e5099_27[3],
                    decode_small_sign_output_tmp_e5099_27[3],
                    decode_small_sign_output_tmp_e5099_27[3],
                    decode_small_sign_output_tmp_e5099_27[3],
                    decode_small_sign_output_tmp_e5099_27[3],
                    decode_small_sign_output_tmp_e5099_27[3],
                    decode_small_sign_output_tmp_e5099_27[3],
                    decode_small_sign_output_tmp_e5099_27[3],
                    decode_small_sign_output_tmp_e5099_27[3],
                    decode_small_sign_output_tmp_e5099_27[3],
                    decode_small_sign_output_tmp_e5099_27[3],
                    decode_small_sign_output_tmp_e5099_27[3],
                    decode_small_sign_output_tmp_e5099_27[3],
                    decode_small_sign_output_tmp_e5099_27[4],
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    decode_small_sign_output_tmp_e5099_27[5],
                ];
                let read_small_output_tmp_e5099_31 = (
                    ((((((op0_limb_0_col26) + ((op0_limb_1_col27) * (M31_512)))
                        + ((op0_limb_2_col28) * (M31_262144)))
                        + ((remainder_bits_col29) * (M31_134217728)))
                        - (msb_col24))
                        - ((M31_536870912) * (mid_limbs_set_col25))),
                    op0_id_col23,
                );

                // Read Small.

                // Read Id.

                let memory_address_to_id_value_tmp_e5099_32 = memory_address_to_id_state
                    .deduce_output(
                        ((mem1_base_col14) + (decode_instruction_7785f_output_tmp_e5099_11.0[2])),
                    );
                let op1_id_col31 = memory_address_to_id_value_tmp_e5099_32;
                *row[31] = op1_id_col31;
                *sub_component_inputs.memory_address_to_id[2] =
                    ((mem1_base_col14) + (decode_instruction_7785f_output_tmp_e5099_11.0[2]));
                *lookup_data.memory_address_to_id_5 = [
                    M31_1444891767,
                    ((mem1_base_col14) + (decode_instruction_7785f_output_tmp_e5099_11.0[2])),
                    op1_id_col31,
                ];

                let memory_id_to_big_value_tmp_e5099_34 =
                    memory_id_to_big_state.deduce_output(op1_id_col31);

                // Decode Small Sign.

                let msb_tmp_e5099_35 = memory_id_to_big_value_tmp_e5099_34.get_m31(27).eq(M31_256);
                let msb_col32 = msb_tmp_e5099_35.as_m31();
                *row[32] = msb_col32;
                let mid_limbs_set_tmp_e5099_36 =
                    ((memory_id_to_big_value_tmp_e5099_34.get_m31(20).eq(M31_511))
                        & (msb_tmp_e5099_35));
                let mid_limbs_set_col33 = mid_limbs_set_tmp_e5099_36.as_m31();
                *row[33] = mid_limbs_set_col33;
                let decode_small_sign_output_tmp_e5099_37 = [
                    msb_col32,
                    mid_limbs_set_col33,
                    ((mid_limbs_set_col33) * (M31_508)),
                    ((mid_limbs_set_col33) * (M31_511)),
                    (((msb_col32) * (M31_136)) - (mid_limbs_set_col33)),
                    ((msb_col32) * (M31_256)),
                ];

                let op1_limb_0_col34 = memory_id_to_big_value_tmp_e5099_34.get_m31(0);
                *row[34] = op1_limb_0_col34;
                let op1_limb_1_col35 = memory_id_to_big_value_tmp_e5099_34.get_m31(1);
                *row[35] = op1_limb_1_col35;
                let op1_limb_2_col36 = memory_id_to_big_value_tmp_e5099_34.get_m31(2);
                *row[36] = op1_limb_2_col36;
                let remainder_bits_tmp_e5099_38 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_e5099_34.get_m31(3)))
                        & (UInt16_3));
                let remainder_bits_col37 = remainder_bits_tmp_e5099_38.as_m31();
                *row[37] = remainder_bits_col37;

                // Cond Range Check 2.

                let partial_limb_msb_tmp_e5099_39 =
                    (((PackedUInt16::from_m31(remainder_bits_col37)) & (UInt16_2)) >> (UInt16_1));
                let partial_limb_msb_col38 = partial_limb_msb_tmp_e5099_39.as_m31();
                *row[38] = partial_limb_msb_col38;

                *sub_component_inputs.memory_id_to_big[2] = op1_id_col31;
                *lookup_data.memory_id_to_big_6 = [
                    M31_1662111297,
                    op1_id_col31,
                    op1_limb_0_col34,
                    op1_limb_1_col35,
                    op1_limb_2_col36,
                    ((remainder_bits_col37) + (decode_small_sign_output_tmp_e5099_37[2])),
                    decode_small_sign_output_tmp_e5099_37[3],
                    decode_small_sign_output_tmp_e5099_37[3],
                    decode_small_sign_output_tmp_e5099_37[3],
                    decode_small_sign_output_tmp_e5099_37[3],
                    decode_small_sign_output_tmp_e5099_37[3],
                    decode_small_sign_output_tmp_e5099_37[3],
                    decode_small_sign_output_tmp_e5099_37[3],
                    decode_small_sign_output_tmp_e5099_37[3],
                    decode_small_sign_output_tmp_e5099_37[3],
                    decode_small_sign_output_tmp_e5099_37[3],
                    decode_small_sign_output_tmp_e5099_37[3],
                    decode_small_sign_output_tmp_e5099_37[3],
                    decode_small_sign_output_tmp_e5099_37[3],
                    decode_small_sign_output_tmp_e5099_37[3],
                    decode_small_sign_output_tmp_e5099_37[3],
                    decode_small_sign_output_tmp_e5099_37[3],
                    decode_small_sign_output_tmp_e5099_37[3],
                    decode_small_sign_output_tmp_e5099_37[4],
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    decode_small_sign_output_tmp_e5099_37[5],
                ];
                let read_small_output_tmp_e5099_41 = (
                    ((((((op1_limb_0_col34) + ((op1_limb_1_col35) * (M31_512)))
                        + ((op1_limb_2_col36) * (M31_262144)))
                        + ((remainder_bits_col37) * (M31_134217728)))
                        - (msb_col32))
                        - ((M31_536870912) * (mid_limbs_set_col33))),
                    op1_id_col31,
                );

                *lookup_data.opcodes_7 =
                    [M31_428564188, input_pc_col1, input_ap_col2, input_fp_col3];
                *lookup_data.opcodes_8 = [
                    M31_428564188,
                    (((input_pc_col1) + (M31_1)) + (op1_imm_col9)),
                    ((input_ap_col2) + (ap_update_add_1_col11)),
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
    memory_address_to_id_5: Vec<[PackedM31; 3]>,
    memory_id_to_big_6: Vec<[PackedM31; 30]>,
    opcodes_7: Vec<[PackedM31; 4]>,
    opcodes_8: Vec<[PackedM31; 4]>,
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
            &self.lookup_data.opcodes_7,
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
        (col_gen.par_iter_mut(), &self.lookup_data.opcodes_8, self.lookup_data.mults_0)
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
