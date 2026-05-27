// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::blake_compress_opcode::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{
    blake_round, memory_address_to_id, memory_id_to_big, range_check_7_2_5, triple_xor_32,
    verify_bitwise_xor_8, verify_instruction,
};
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
        range_check_7_2_5_state: &range_check_7_2_5::ClaimGenerator,
        verify_bitwise_xor_8_state: &verify_bitwise_xor_8::ClaimGenerator,
        blake_round_state: &blake_round::ClaimGenerator,
        triple_xor_32_state: &triple_xor_32::ClaimGenerator,
    ) -> (
        ComponentTrace<N_TRACE_COLUMNS>,
        Claim,
        InteractionClaimGenerator,
    ) {
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
            range_check_7_2_5_state,
            verify_bitwise_xor_8_state,
            blake_round_state,
            triple_xor_32_state,
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
        for inputs in sub_component_inputs.range_check_7_2_5 {
            add_inputs(range_check_7_2_5_state, &inputs, n_active_rows, 0);
        }
        for inputs in sub_component_inputs.verify_bitwise_xor_8 {
            add_inputs(verify_bitwise_xor_8_state, &inputs, n_active_rows, 0);
        }
        for inputs in sub_component_inputs.blake_round {
            add_inputs(blake_round_state, &inputs, n_active_rows, 0);
        }
        for inputs in sub_component_inputs.triple_xor_32 {
            add_inputs(triple_xor_32_state, &inputs, n_active_rows, 0);
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

#[derive(Uninitialized, IterMut, ParIterMut)]
struct SubComponentInputs {
    verify_instruction: [Vec<verify_instruction::PackedInputType>; 1],
    memory_address_to_id: [Vec<memory_address_to_id::PackedInputType>; 20],
    memory_id_to_big: [Vec<memory_id_to_big::PackedInputType>; 20],
    range_check_7_2_5: [Vec<range_check_7_2_5::PackedInputType>; 17],
    verify_bitwise_xor_8: [Vec<verify_bitwise_xor_8::PackedInputType>; 4],
    blake_round: [Vec<blake_round::PackedInputType>; 10],
    triple_xor_32: [Vec<triple_xor_32::PackedInputType>; 8],
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
    range_check_7_2_5_state: &range_check_7_2_5::ClaimGenerator,
    verify_bitwise_xor_8_state: &verify_bitwise_xor_8::ClaimGenerator,
    blake_round_state: &blake_round::ClaimGenerator,
    triple_xor_32_state: &triple_xor_32::ClaimGenerator,
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
    let M31_10 = PackedM31::broadcast(M31::from(10));
    let M31_112558620 = PackedM31::broadcast(M31::from(112558620));
    let M31_127 = PackedM31::broadcast(M31::from(127));
    let M31_128 = PackedM31::broadcast(M31::from(128));
    let M31_134217728 = PackedM31::broadcast(M31::from(134217728));
    let M31_14 = PackedM31::broadcast(M31::from(14));
    let M31_1444891767 = PackedM31::broadcast(M31::from(1444891767));
    let M31_15470 = PackedM31::broadcast(M31::from(15470));
    let M31_16 = PackedM31::broadcast(M31::from(16));
    let M31_1662111297 = PackedM31::broadcast(M31::from(1662111297));
    let M31_1719106205 = PackedM31::broadcast(M31::from(1719106205));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_2048 = PackedM31::broadcast(M31::from(2048));
    let M31_23520 = PackedM31::broadcast(M31::from(23520));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_26764 = PackedM31::broadcast(M31::from(26764));
    let M31_27145 = PackedM31::broadcast(M31::from(27145));
    let M31_3 = PackedM31::broadcast(M31::from(3));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_371240602 = PackedM31::broadcast(M31::from(371240602));
    let M31_39685 = PackedM31::broadcast(M31::from(39685));
    let M31_4 = PackedM31::broadcast(M31::from(4));
    let M31_40528774 = PackedM31::broadcast(M31::from(40528774));
    let M31_42319 = PackedM31::broadcast(M31::from(42319));
    let M31_428564188 = PackedM31::broadcast(M31::from(428564188));
    let M31_44677 = PackedM31::broadcast(M31::from(44677));
    let M31_47975 = PackedM31::broadcast(M31::from(47975));
    let M31_5 = PackedM31::broadcast(M31::from(5));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let M31_52505 = PackedM31::broadcast(M31::from(52505));
    let M31_55723 = PackedM31::broadcast(M31::from(55723));
    let M31_57468 = PackedM31::broadcast(M31::from(57468));
    let M31_58983 = PackedM31::broadcast(M31::from(58983));
    let M31_6 = PackedM31::broadcast(M31::from(6));
    let M31_62322 = PackedM31::broadcast(M31::from(62322));
    let M31_62778 = PackedM31::broadcast(M31::from(62778));
    let M31_64 = PackedM31::broadcast(M31::from(64));
    let M31_7 = PackedM31::broadcast(M31::from(7));
    let M31_8 = PackedM31::broadcast(M31::from(8));
    let M31_8067 = PackedM31::broadcast(M31::from(8067));
    let M31_81 = PackedM31::broadcast(M31::from(81));
    let M31_82 = PackedM31::broadcast(M31::from(82));
    let M31_9 = PackedM31::broadcast(M31::from(9));
    let M31_9812 = PackedM31::broadcast(M31::from(9812));
    let M31_990559919 = PackedM31::broadcast(M31::from(990559919));
    let UInt16_0 = PackedUInt16::broadcast(UInt16::from(0));
    let UInt16_1 = PackedUInt16::broadcast(UInt16::from(1));
    let UInt16_11 = PackedUInt16::broadcast(UInt16::from(11));
    let UInt16_127 = PackedUInt16::broadcast(UInt16::from(127));
    let UInt16_13 = PackedUInt16::broadcast(UInt16::from(13));
    let UInt16_14 = PackedUInt16::broadcast(UInt16::from(14));
    let UInt16_2 = PackedUInt16::broadcast(UInt16::from(2));
    let UInt16_3 = PackedUInt16::broadcast(UInt16::from(3));
    let UInt16_31 = PackedUInt16::broadcast(UInt16::from(31));
    let UInt16_4 = PackedUInt16::broadcast(UInt16::from(4));
    let UInt16_5 = PackedUInt16::broadcast(UInt16::from(5));
    let UInt16_6 = PackedUInt16::broadcast(UInt16::from(6));
    let UInt16_7 = PackedUInt16::broadcast(UInt16::from(7));
    let UInt16_8 = PackedUInt16::broadcast(UInt16::from(8));
    let UInt16_81 = PackedUInt16::broadcast(UInt16::from(81));
    let UInt16_82 = PackedUInt16::broadcast(UInt16::from(82));
    let UInt16_9 = PackedUInt16::broadcast(UInt16::from(9));
    let UInt32_1013904242 = PackedUInt32::broadcast(UInt32::from(1013904242));
    let UInt32_1541459225 = PackedUInt32::broadcast(UInt32::from(1541459225));
    let UInt32_1779033703 = PackedUInt32::broadcast(UInt32::from(1779033703));
    let UInt32_2600822924 = PackedUInt32::broadcast(UInt32::from(2600822924));
    let UInt32_2773480762 = PackedUInt32::broadcast(UInt32::from(2773480762));
    let UInt32_3144134277 = PackedUInt32::broadcast(UInt32::from(3144134277));
    let seq = Seq::new(log_size);
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
            |(row_index, (row, lookup_data, sub_component_inputs, blake_compress_opcode_input))| {
                let seq = seq.packed_at(row_index);
                let enabler_col0 = enabler_col.packed_at(row_index);
                *row[0] = enabler_col0;
                let input_pc_col1 = blake_compress_opcode_input.pc;
                *row[1] = input_pc_col1;
                let input_ap_col2 = blake_compress_opcode_input.ap;
                *row[2] = input_ap_col2;
                let input_fp_col3 = blake_compress_opcode_input.fp;
                *row[3] = input_fp_col3;

                // Decode Blake Opcode.

                // Decode Instruction.

                let memory_address_to_id_value_tmp_40cd9_0 =
                    memory_address_to_id_state.deduce_output(input_pc_col1);
                let memory_id_to_big_value_tmp_40cd9_1 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_40cd9_0);
                let offset0_tmp_40cd9_2 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_40cd9_1.get_m31(0)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_40cd9_1.get_m31(1),
                        )) & (UInt16_127))
                            << (UInt16_9)));
                let offset0_col4 = offset0_tmp_40cd9_2.as_m31();
                *row[4] = offset0_col4;
                let offset1_tmp_40cd9_3 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_40cd9_1.get_m31(1)))
                        >> (UInt16_7))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_40cd9_1.get_m31(2),
                        )) << (UInt16_2)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_40cd9_1.get_m31(3),
                        )) & (UInt16_31))
                            << (UInt16_11)));
                let offset1_col5 = offset1_tmp_40cd9_3.as_m31();
                *row[5] = offset1_col5;
                let offset2_tmp_40cd9_4 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_40cd9_1.get_m31(3)))
                        >> (UInt16_5))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_40cd9_1.get_m31(4),
                        )) << (UInt16_4)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_40cd9_1.get_m31(5),
                        )) & (UInt16_7))
                            << (UInt16_13)));
                let offset2_col6 = offset2_tmp_40cd9_4.as_m31();
                *row[6] = offset2_col6;
                let dst_base_fp_tmp_40cd9_5 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_40cd9_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_40cd9_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_0))
                        & (UInt16_1));
                let dst_base_fp_col7 = dst_base_fp_tmp_40cd9_5.as_m31();
                *row[7] = dst_base_fp_col7;
                let op0_base_fp_tmp_40cd9_6 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_40cd9_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_40cd9_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_1))
                        & (UInt16_1));
                let op0_base_fp_col8 = op0_base_fp_tmp_40cd9_6.as_m31();
                *row[8] = op0_base_fp_col8;
                let op1_base_fp_tmp_40cd9_7 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_40cd9_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_40cd9_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_3))
                        & (UInt16_1));
                let op1_base_fp_col9 = op1_base_fp_tmp_40cd9_7.as_m31();
                *row[9] = op1_base_fp_col9;
                let ap_update_add_1_tmp_40cd9_8 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_40cd9_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_40cd9_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_11))
                        & (UInt16_1));
                let ap_update_add_1_col10 = ap_update_add_1_tmp_40cd9_8.as_m31();
                *row[10] = ap_update_add_1_col10;
                let opcode_extension_col11 = memory_id_to_big_value_tmp_40cd9_1.get_m31(7);
                *row[11] = opcode_extension_col11;
                *sub_component_inputs.verify_instruction[0] = (
                    input_pc_col1,
                    [offset0_col4, offset1_col5, offset2_col6],
                    [
                        (((((dst_base_fp_col7) * (M31_8)) + ((op0_base_fp_col8) * (M31_16)))
                            + ((op1_base_fp_col9) * (M31_64)))
                            + (((M31_1) - (op1_base_fp_col9)) * (M31_128))),
                        ((ap_update_add_1_col10) * (M31_32)),
                    ],
                    opcode_extension_col11,
                );
                *lookup_data.verify_instruction_0 = [
                    M31_1719106205,
                    input_pc_col1,
                    offset0_col4,
                    offset1_col5,
                    offset2_col6,
                    (((((dst_base_fp_col7) * (M31_8)) + ((op0_base_fp_col8) * (M31_16)))
                        + ((op1_base_fp_col9) * (M31_64)))
                        + (((M31_1) - (op1_base_fp_col9)) * (M31_128))),
                    ((ap_update_add_1_col10) * (M31_32)),
                    opcode_extension_col11,
                ];
                let decode_instruction_30129_output_tmp_40cd9_9 = (
                    [
                        ((offset0_col4) - (M31_32768)),
                        ((offset1_col5) - (M31_32768)),
                        ((offset2_col6) - (M31_32768)),
                    ],
                    [
                        dst_base_fp_col7,
                        op0_base_fp_col8,
                        M31_0,
                        op1_base_fp_col9,
                        ((M31_1) - (op1_base_fp_col9)),
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        ap_update_add_1_col10,
                        M31_0,
                        M31_0,
                        M31_0,
                    ],
                    opcode_extension_col11,
                );

                let mem0_base_col12 = (((op0_base_fp_col8) * (input_fp_col3))
                    + (((M31_1) - (op0_base_fp_col8)) * (input_ap_col2)));
                *row[12] = mem0_base_col12;

                // Read Positive Num Bits 29.

                // Read Id.

                let memory_address_to_id_value_tmp_40cd9_10 = memory_address_to_id_state
                    .deduce_output(
                        ((mem0_base_col12) + (decode_instruction_30129_output_tmp_40cd9_9.0[1])),
                    );
                let op0_id_col13 = memory_address_to_id_value_tmp_40cd9_10;
                *row[13] = op0_id_col13;
                *sub_component_inputs.memory_address_to_id[0] =
                    ((mem0_base_col12) + (decode_instruction_30129_output_tmp_40cd9_9.0[1]));
                *lookup_data.memory_address_to_id_1 = [
                    M31_1444891767,
                    ((mem0_base_col12) + (decode_instruction_30129_output_tmp_40cd9_9.0[1])),
                    op0_id_col13,
                ];

                // Read Positive Known Id Num Bits 29.

                let memory_id_to_big_value_tmp_40cd9_12 =
                    memory_id_to_big_state.deduce_output(op0_id_col13);
                let op0_limb_0_col14 = memory_id_to_big_value_tmp_40cd9_12.get_m31(0);
                *row[14] = op0_limb_0_col14;
                let op0_limb_1_col15 = memory_id_to_big_value_tmp_40cd9_12.get_m31(1);
                *row[15] = op0_limb_1_col15;
                let op0_limb_2_col16 = memory_id_to_big_value_tmp_40cd9_12.get_m31(2);
                *row[16] = op0_limb_2_col16;
                let op0_limb_3_col17 = memory_id_to_big_value_tmp_40cd9_12.get_m31(3);
                *row[17] = op0_limb_3_col17;

                // Range Check Last Limb Bits In Ms Limb 2.

                // Cond Range Check 2.

                let partial_limb_msb_tmp_40cd9_13 =
                    (((PackedUInt16::from_m31(op0_limb_3_col17)) & (UInt16_2)) >> (UInt16_1));
                let partial_limb_msb_col18 = partial_limb_msb_tmp_40cd9_13.as_m31();
                *row[18] = partial_limb_msb_col18;

                *sub_component_inputs.memory_id_to_big[0] = op0_id_col13;
                *lookup_data.memory_id_to_big_2 = [
                    M31_1662111297,
                    op0_id_col13,
                    op0_limb_0_col14,
                    op0_limb_1_col15,
                    op0_limb_2_col16,
                    op0_limb_3_col17,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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
                let read_positive_known_id_num_bits_29_output_tmp_40cd9_15 =
                    PackedFelt252::from_limbs([
                        op0_limb_0_col14,
                        op0_limb_1_col15,
                        op0_limb_2_col16,
                        op0_limb_3_col17,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                    ]);

                let read_positive_num_bits_29_output_tmp_40cd9_16 = (
                    read_positive_known_id_num_bits_29_output_tmp_40cd9_15,
                    op0_id_col13,
                );

                let mem1_base_col19 = (((op1_base_fp_col9) * (input_fp_col3))
                    + ((decode_instruction_30129_output_tmp_40cd9_9.1[4]) * (input_ap_col2)));
                *row[19] = mem1_base_col19;

                // Read Positive Num Bits 29.

                // Read Id.

                let memory_address_to_id_value_tmp_40cd9_17 = memory_address_to_id_state
                    .deduce_output(
                        ((mem1_base_col19) + (decode_instruction_30129_output_tmp_40cd9_9.0[2])),
                    );
                let op1_id_col20 = memory_address_to_id_value_tmp_40cd9_17;
                *row[20] = op1_id_col20;
                *sub_component_inputs.memory_address_to_id[1] =
                    ((mem1_base_col19) + (decode_instruction_30129_output_tmp_40cd9_9.0[2]));
                *lookup_data.memory_address_to_id_3 = [
                    M31_1444891767,
                    ((mem1_base_col19) + (decode_instruction_30129_output_tmp_40cd9_9.0[2])),
                    op1_id_col20,
                ];

                // Read Positive Known Id Num Bits 29.

                let memory_id_to_big_value_tmp_40cd9_19 =
                    memory_id_to_big_state.deduce_output(op1_id_col20);
                let op1_limb_0_col21 = memory_id_to_big_value_tmp_40cd9_19.get_m31(0);
                *row[21] = op1_limb_0_col21;
                let op1_limb_1_col22 = memory_id_to_big_value_tmp_40cd9_19.get_m31(1);
                *row[22] = op1_limb_1_col22;
                let op1_limb_2_col23 = memory_id_to_big_value_tmp_40cd9_19.get_m31(2);
                *row[23] = op1_limb_2_col23;
                let op1_limb_3_col24 = memory_id_to_big_value_tmp_40cd9_19.get_m31(3);
                *row[24] = op1_limb_3_col24;

                // Range Check Last Limb Bits In Ms Limb 2.

                // Cond Range Check 2.

                let partial_limb_msb_tmp_40cd9_20 =
                    (((PackedUInt16::from_m31(op1_limb_3_col24)) & (UInt16_2)) >> (UInt16_1));
                let partial_limb_msb_col25 = partial_limb_msb_tmp_40cd9_20.as_m31();
                *row[25] = partial_limb_msb_col25;

                *sub_component_inputs.memory_id_to_big[1] = op1_id_col20;
                *lookup_data.memory_id_to_big_4 = [
                    M31_1662111297,
                    op1_id_col20,
                    op1_limb_0_col21,
                    op1_limb_1_col22,
                    op1_limb_2_col23,
                    op1_limb_3_col24,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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
                let read_positive_known_id_num_bits_29_output_tmp_40cd9_22 =
                    PackedFelt252::from_limbs([
                        op1_limb_0_col21,
                        op1_limb_1_col22,
                        op1_limb_2_col23,
                        op1_limb_3_col24,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                    ]);

                let read_positive_num_bits_29_output_tmp_40cd9_23 = (
                    read_positive_known_id_num_bits_29_output_tmp_40cd9_22,
                    op1_id_col20,
                );

                // Read Positive Num Bits 29.

                // Read Id.

                let memory_address_to_id_value_tmp_40cd9_24 =
                    memory_address_to_id_state.deduce_output(input_ap_col2);
                let ap_id_col26 = memory_address_to_id_value_tmp_40cd9_24;
                *row[26] = ap_id_col26;
                *sub_component_inputs.memory_address_to_id[2] = input_ap_col2;
                *lookup_data.memory_address_to_id_5 = [M31_1444891767, input_ap_col2, ap_id_col26];

                // Read Positive Known Id Num Bits 29.

                let memory_id_to_big_value_tmp_40cd9_26 =
                    memory_id_to_big_state.deduce_output(ap_id_col26);
                let ap_limb_0_col27 = memory_id_to_big_value_tmp_40cd9_26.get_m31(0);
                *row[27] = ap_limb_0_col27;
                let ap_limb_1_col28 = memory_id_to_big_value_tmp_40cd9_26.get_m31(1);
                *row[28] = ap_limb_1_col28;
                let ap_limb_2_col29 = memory_id_to_big_value_tmp_40cd9_26.get_m31(2);
                *row[29] = ap_limb_2_col29;
                let ap_limb_3_col30 = memory_id_to_big_value_tmp_40cd9_26.get_m31(3);
                *row[30] = ap_limb_3_col30;

                // Range Check Last Limb Bits In Ms Limb 2.

                // Cond Range Check 2.

                let partial_limb_msb_tmp_40cd9_27 =
                    (((PackedUInt16::from_m31(ap_limb_3_col30)) & (UInt16_2)) >> (UInt16_1));
                let partial_limb_msb_col31 = partial_limb_msb_tmp_40cd9_27.as_m31();
                *row[31] = partial_limb_msb_col31;

                *sub_component_inputs.memory_id_to_big[2] = ap_id_col26;
                *lookup_data.memory_id_to_big_6 = [
                    M31_1662111297,
                    ap_id_col26,
                    ap_limb_0_col27,
                    ap_limb_1_col28,
                    ap_limb_2_col29,
                    ap_limb_3_col30,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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
                let read_positive_known_id_num_bits_29_output_tmp_40cd9_29 =
                    PackedFelt252::from_limbs([
                        ap_limb_0_col27,
                        ap_limb_1_col28,
                        ap_limb_2_col29,
                        ap_limb_3_col30,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                    ]);

                let read_positive_num_bits_29_output_tmp_40cd9_30 = (
                    read_positive_known_id_num_bits_29_output_tmp_40cd9_29,
                    ap_id_col26,
                );

                let mem_dst_base_col32 = (((dst_base_fp_col7) * (input_fp_col3))
                    + (((M31_1) - (dst_base_fp_col7)) * (input_ap_col2)));
                *row[32] = mem_dst_base_col32;

                // Read U 32.

                let memory_address_to_id_value_tmp_40cd9_31 = memory_address_to_id_state
                    .deduce_output(
                        ((mem_dst_base_col32) + (decode_instruction_30129_output_tmp_40cd9_9.0[0])),
                    );
                let memory_id_to_big_value_tmp_40cd9_32 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_40cd9_31);
                let tmp_40cd9_33 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_40cd9_32.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col33 = ((((memory_id_to_big_value_tmp_40cd9_32.get_m31(1))
                    - ((tmp_40cd9_33.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_40cd9_32.get_m31(0)));
                *row[33] = low_16_bits_col33;
                let high_16_bits_col34 = ((((memory_id_to_big_value_tmp_40cd9_32.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_40cd9_32.get_m31(2)) * (M31_4)))
                    + (tmp_40cd9_33.as_m31()));
                *row[34] = high_16_bits_col34;
                let expected_word_tmp_40cd9_34 =
                    PackedUInt32::from_limbs([low_16_bits_col33, high_16_bits_col34]);

                // Verify U 32.

                let low_7_ms_bits_tmp_40cd9_35 = ((expected_word_tmp_40cd9_34.low()) >> (UInt16_9));
                let low_7_ms_bits_col35 = low_7_ms_bits_tmp_40cd9_35.as_m31();
                *row[35] = low_7_ms_bits_col35;
                let high_14_ms_bits_tmp_40cd9_36 =
                    ((expected_word_tmp_40cd9_34.high()) >> (UInt16_2));
                let high_14_ms_bits_col36 = high_14_ms_bits_tmp_40cd9_36.as_m31();
                *row[36] = high_14_ms_bits_col36;
                let high_2_ls_bits_tmp_40cd9_37 =
                    ((high_16_bits_col34) - ((high_14_ms_bits_col36) * (M31_4)));
                let high_5_ms_bits_tmp_40cd9_38 = ((high_14_ms_bits_tmp_40cd9_36) >> (UInt16_9));
                let high_5_ms_bits_col37 = high_5_ms_bits_tmp_40cd9_38.as_m31();
                *row[37] = high_5_ms_bits_col37;
                *sub_component_inputs.range_check_7_2_5[0] = [
                    low_7_ms_bits_col35,
                    high_2_ls_bits_tmp_40cd9_37,
                    high_5_ms_bits_col37,
                ];
                *lookup_data.range_check_7_2_5_7 = [
                    M31_371240602,
                    low_7_ms_bits_col35,
                    high_2_ls_bits_tmp_40cd9_37,
                    high_5_ms_bits_col37,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_40cd9_39 = memory_address_to_id_state
                    .deduce_output(
                        ((mem_dst_base_col32) + (decode_instruction_30129_output_tmp_40cd9_9.0[0])),
                    );
                let dst_id_col38 = memory_address_to_id_value_tmp_40cd9_39;
                *row[38] = dst_id_col38;
                *sub_component_inputs.memory_address_to_id[3] =
                    ((mem_dst_base_col32) + (decode_instruction_30129_output_tmp_40cd9_9.0[0]));
                *lookup_data.memory_address_to_id_8 = [
                    M31_1444891767,
                    ((mem_dst_base_col32) + (decode_instruction_30129_output_tmp_40cd9_9.0[0])),
                    dst_id_col38,
                ];

                *sub_component_inputs.memory_id_to_big[3] = dst_id_col38;
                *lookup_data.memory_id_to_big_9 = [
                    M31_1662111297,
                    dst_id_col38,
                    ((low_16_bits_col33) - ((low_7_ms_bits_col35) * (M31_512))),
                    ((low_7_ms_bits_col35) + ((high_2_ls_bits_tmp_40cd9_37) * (M31_128))),
                    ((high_14_ms_bits_col36) - ((high_5_ms_bits_col37) * (M31_512))),
                    high_5_ms_bits_col37,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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

                let read_u_32_output_tmp_40cd9_41 = expected_word_tmp_40cd9_34;

                let decode_blake_opcode_output_tmp_40cd9_42 = (
                    [
                        ((((op0_limb_0_col14) + ((op0_limb_1_col15) * (M31_512)))
                            + ((op0_limb_2_col16) * (M31_262144)))
                            + ((op0_limb_3_col17) * (M31_134217728))),
                        ((((op1_limb_0_col21) + ((op1_limb_1_col22) * (M31_512)))
                            + ((op1_limb_2_col23) * (M31_262144)))
                            + ((op1_limb_3_col24) * (M31_134217728))),
                        ((((ap_limb_0_col27) + ((ap_limb_1_col28) * (M31_512)))
                            + ((ap_limb_2_col29) * (M31_262144)))
                            + ((ap_limb_3_col30) * (M31_134217728))),
                    ],
                    read_u_32_output_tmp_40cd9_41,
                    [
                        PackedBool::from_m31(ap_update_add_1_col10),
                        PackedBool::from_m31(((opcode_extension_col11) - (M31_1))),
                    ],
                );

                // Create Blake Round Input.

                // Read U 32.

                let memory_address_to_id_value_tmp_40cd9_43 = memory_address_to_id_state
                    .deduce_output(decode_blake_opcode_output_tmp_40cd9_42.0[0]);
                let memory_id_to_big_value_tmp_40cd9_44 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_40cd9_43);
                let tmp_40cd9_45 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_40cd9_44.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col39 = ((((memory_id_to_big_value_tmp_40cd9_44.get_m31(1))
                    - ((tmp_40cd9_45.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_40cd9_44.get_m31(0)));
                *row[39] = low_16_bits_col39;
                let high_16_bits_col40 = ((((memory_id_to_big_value_tmp_40cd9_44.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_40cd9_44.get_m31(2)) * (M31_4)))
                    + (tmp_40cd9_45.as_m31()));
                *row[40] = high_16_bits_col40;
                let expected_word_tmp_40cd9_46 =
                    PackedUInt32::from_limbs([low_16_bits_col39, high_16_bits_col40]);

                // Verify U 32.

                let low_7_ms_bits_tmp_40cd9_47 = ((expected_word_tmp_40cd9_46.low()) >> (UInt16_9));
                let low_7_ms_bits_col41 = low_7_ms_bits_tmp_40cd9_47.as_m31();
                *row[41] = low_7_ms_bits_col41;
                let high_14_ms_bits_tmp_40cd9_48 =
                    ((expected_word_tmp_40cd9_46.high()) >> (UInt16_2));
                let high_14_ms_bits_col42 = high_14_ms_bits_tmp_40cd9_48.as_m31();
                *row[42] = high_14_ms_bits_col42;
                let high_2_ls_bits_tmp_40cd9_49 =
                    ((high_16_bits_col40) - ((high_14_ms_bits_col42) * (M31_4)));
                let high_5_ms_bits_tmp_40cd9_50 = ((high_14_ms_bits_tmp_40cd9_48) >> (UInt16_9));
                let high_5_ms_bits_col43 = high_5_ms_bits_tmp_40cd9_50.as_m31();
                *row[43] = high_5_ms_bits_col43;
                *sub_component_inputs.range_check_7_2_5[1] = [
                    low_7_ms_bits_col41,
                    high_2_ls_bits_tmp_40cd9_49,
                    high_5_ms_bits_col43,
                ];
                *lookup_data.range_check_7_2_5_10 = [
                    M31_371240602,
                    low_7_ms_bits_col41,
                    high_2_ls_bits_tmp_40cd9_49,
                    high_5_ms_bits_col43,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_40cd9_51 = memory_address_to_id_state
                    .deduce_output(decode_blake_opcode_output_tmp_40cd9_42.0[0]);
                let state_0_id_col44 = memory_address_to_id_value_tmp_40cd9_51;
                *row[44] = state_0_id_col44;
                *sub_component_inputs.memory_address_to_id[4] =
                    decode_blake_opcode_output_tmp_40cd9_42.0[0];
                *lookup_data.memory_address_to_id_11 = [
                    M31_1444891767,
                    decode_blake_opcode_output_tmp_40cd9_42.0[0],
                    state_0_id_col44,
                ];

                *sub_component_inputs.memory_id_to_big[4] = state_0_id_col44;
                *lookup_data.memory_id_to_big_12 = [
                    M31_1662111297,
                    state_0_id_col44,
                    ((low_16_bits_col39) - ((low_7_ms_bits_col41) * (M31_512))),
                    ((low_7_ms_bits_col41) + ((high_2_ls_bits_tmp_40cd9_49) * (M31_128))),
                    ((high_14_ms_bits_col42) - ((high_5_ms_bits_col43) * (M31_512))),
                    high_5_ms_bits_col43,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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

                let read_u_32_output_tmp_40cd9_53 = expected_word_tmp_40cd9_46;

                // Read U 32.

                let memory_address_to_id_value_tmp_40cd9_54 = memory_address_to_id_state
                    .deduce_output(((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_1)));
                let memory_id_to_big_value_tmp_40cd9_55 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_40cd9_54);
                let tmp_40cd9_56 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_40cd9_55.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col45 = ((((memory_id_to_big_value_tmp_40cd9_55.get_m31(1))
                    - ((tmp_40cd9_56.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_40cd9_55.get_m31(0)));
                *row[45] = low_16_bits_col45;
                let high_16_bits_col46 = ((((memory_id_to_big_value_tmp_40cd9_55.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_40cd9_55.get_m31(2)) * (M31_4)))
                    + (tmp_40cd9_56.as_m31()));
                *row[46] = high_16_bits_col46;
                let expected_word_tmp_40cd9_57 =
                    PackedUInt32::from_limbs([low_16_bits_col45, high_16_bits_col46]);

                // Verify U 32.

                let low_7_ms_bits_tmp_40cd9_58 = ((expected_word_tmp_40cd9_57.low()) >> (UInt16_9));
                let low_7_ms_bits_col47 = low_7_ms_bits_tmp_40cd9_58.as_m31();
                *row[47] = low_7_ms_bits_col47;
                let high_14_ms_bits_tmp_40cd9_59 =
                    ((expected_word_tmp_40cd9_57.high()) >> (UInt16_2));
                let high_14_ms_bits_col48 = high_14_ms_bits_tmp_40cd9_59.as_m31();
                *row[48] = high_14_ms_bits_col48;
                let high_2_ls_bits_tmp_40cd9_60 =
                    ((high_16_bits_col46) - ((high_14_ms_bits_col48) * (M31_4)));
                let high_5_ms_bits_tmp_40cd9_61 = ((high_14_ms_bits_tmp_40cd9_59) >> (UInt16_9));
                let high_5_ms_bits_col49 = high_5_ms_bits_tmp_40cd9_61.as_m31();
                *row[49] = high_5_ms_bits_col49;
                *sub_component_inputs.range_check_7_2_5[2] = [
                    low_7_ms_bits_col47,
                    high_2_ls_bits_tmp_40cd9_60,
                    high_5_ms_bits_col49,
                ];
                *lookup_data.range_check_7_2_5_13 = [
                    M31_371240602,
                    low_7_ms_bits_col47,
                    high_2_ls_bits_tmp_40cd9_60,
                    high_5_ms_bits_col49,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_40cd9_62 = memory_address_to_id_state
                    .deduce_output(((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_1)));
                let state_1_id_col50 = memory_address_to_id_value_tmp_40cd9_62;
                *row[50] = state_1_id_col50;
                *sub_component_inputs.memory_address_to_id[5] =
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_1));
                *lookup_data.memory_address_to_id_14 = [
                    M31_1444891767,
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_1)),
                    state_1_id_col50,
                ];

                *sub_component_inputs.memory_id_to_big[5] = state_1_id_col50;
                *lookup_data.memory_id_to_big_15 = [
                    M31_1662111297,
                    state_1_id_col50,
                    ((low_16_bits_col45) - ((low_7_ms_bits_col47) * (M31_512))),
                    ((low_7_ms_bits_col47) + ((high_2_ls_bits_tmp_40cd9_60) * (M31_128))),
                    ((high_14_ms_bits_col48) - ((high_5_ms_bits_col49) * (M31_512))),
                    high_5_ms_bits_col49,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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

                let read_u_32_output_tmp_40cd9_64 = expected_word_tmp_40cd9_57;

                // Read U 32.

                let memory_address_to_id_value_tmp_40cd9_65 = memory_address_to_id_state
                    .deduce_output(((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_2)));
                let memory_id_to_big_value_tmp_40cd9_66 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_40cd9_65);
                let tmp_40cd9_67 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_40cd9_66.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col51 = ((((memory_id_to_big_value_tmp_40cd9_66.get_m31(1))
                    - ((tmp_40cd9_67.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_40cd9_66.get_m31(0)));
                *row[51] = low_16_bits_col51;
                let high_16_bits_col52 = ((((memory_id_to_big_value_tmp_40cd9_66.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_40cd9_66.get_m31(2)) * (M31_4)))
                    + (tmp_40cd9_67.as_m31()));
                *row[52] = high_16_bits_col52;
                let expected_word_tmp_40cd9_68 =
                    PackedUInt32::from_limbs([low_16_bits_col51, high_16_bits_col52]);

                // Verify U 32.

                let low_7_ms_bits_tmp_40cd9_69 = ((expected_word_tmp_40cd9_68.low()) >> (UInt16_9));
                let low_7_ms_bits_col53 = low_7_ms_bits_tmp_40cd9_69.as_m31();
                *row[53] = low_7_ms_bits_col53;
                let high_14_ms_bits_tmp_40cd9_70 =
                    ((expected_word_tmp_40cd9_68.high()) >> (UInt16_2));
                let high_14_ms_bits_col54 = high_14_ms_bits_tmp_40cd9_70.as_m31();
                *row[54] = high_14_ms_bits_col54;
                let high_2_ls_bits_tmp_40cd9_71 =
                    ((high_16_bits_col52) - ((high_14_ms_bits_col54) * (M31_4)));
                let high_5_ms_bits_tmp_40cd9_72 = ((high_14_ms_bits_tmp_40cd9_70) >> (UInt16_9));
                let high_5_ms_bits_col55 = high_5_ms_bits_tmp_40cd9_72.as_m31();
                *row[55] = high_5_ms_bits_col55;
                *sub_component_inputs.range_check_7_2_5[3] = [
                    low_7_ms_bits_col53,
                    high_2_ls_bits_tmp_40cd9_71,
                    high_5_ms_bits_col55,
                ];
                *lookup_data.range_check_7_2_5_16 = [
                    M31_371240602,
                    low_7_ms_bits_col53,
                    high_2_ls_bits_tmp_40cd9_71,
                    high_5_ms_bits_col55,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_40cd9_73 = memory_address_to_id_state
                    .deduce_output(((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_2)));
                let state_2_id_col56 = memory_address_to_id_value_tmp_40cd9_73;
                *row[56] = state_2_id_col56;
                *sub_component_inputs.memory_address_to_id[6] =
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_2));
                *lookup_data.memory_address_to_id_17 = [
                    M31_1444891767,
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_2)),
                    state_2_id_col56,
                ];

                *sub_component_inputs.memory_id_to_big[6] = state_2_id_col56;
                *lookup_data.memory_id_to_big_18 = [
                    M31_1662111297,
                    state_2_id_col56,
                    ((low_16_bits_col51) - ((low_7_ms_bits_col53) * (M31_512))),
                    ((low_7_ms_bits_col53) + ((high_2_ls_bits_tmp_40cd9_71) * (M31_128))),
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

                let read_u_32_output_tmp_40cd9_75 = expected_word_tmp_40cd9_68;

                // Read U 32.

                let memory_address_to_id_value_tmp_40cd9_76 = memory_address_to_id_state
                    .deduce_output(((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_3)));
                let memory_id_to_big_value_tmp_40cd9_77 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_40cd9_76);
                let tmp_40cd9_78 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_40cd9_77.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col57 = ((((memory_id_to_big_value_tmp_40cd9_77.get_m31(1))
                    - ((tmp_40cd9_78.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_40cd9_77.get_m31(0)));
                *row[57] = low_16_bits_col57;
                let high_16_bits_col58 = ((((memory_id_to_big_value_tmp_40cd9_77.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_40cd9_77.get_m31(2)) * (M31_4)))
                    + (tmp_40cd9_78.as_m31()));
                *row[58] = high_16_bits_col58;
                let expected_word_tmp_40cd9_79 =
                    PackedUInt32::from_limbs([low_16_bits_col57, high_16_bits_col58]);

                // Verify U 32.

                let low_7_ms_bits_tmp_40cd9_80 = ((expected_word_tmp_40cd9_79.low()) >> (UInt16_9));
                let low_7_ms_bits_col59 = low_7_ms_bits_tmp_40cd9_80.as_m31();
                *row[59] = low_7_ms_bits_col59;
                let high_14_ms_bits_tmp_40cd9_81 =
                    ((expected_word_tmp_40cd9_79.high()) >> (UInt16_2));
                let high_14_ms_bits_col60 = high_14_ms_bits_tmp_40cd9_81.as_m31();
                *row[60] = high_14_ms_bits_col60;
                let high_2_ls_bits_tmp_40cd9_82 =
                    ((high_16_bits_col58) - ((high_14_ms_bits_col60) * (M31_4)));
                let high_5_ms_bits_tmp_40cd9_83 = ((high_14_ms_bits_tmp_40cd9_81) >> (UInt16_9));
                let high_5_ms_bits_col61 = high_5_ms_bits_tmp_40cd9_83.as_m31();
                *row[61] = high_5_ms_bits_col61;
                *sub_component_inputs.range_check_7_2_5[4] = [
                    low_7_ms_bits_col59,
                    high_2_ls_bits_tmp_40cd9_82,
                    high_5_ms_bits_col61,
                ];
                *lookup_data.range_check_7_2_5_19 = [
                    M31_371240602,
                    low_7_ms_bits_col59,
                    high_2_ls_bits_tmp_40cd9_82,
                    high_5_ms_bits_col61,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_40cd9_84 = memory_address_to_id_state
                    .deduce_output(((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_3)));
                let state_3_id_col62 = memory_address_to_id_value_tmp_40cd9_84;
                *row[62] = state_3_id_col62;
                *sub_component_inputs.memory_address_to_id[7] =
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_3));
                *lookup_data.memory_address_to_id_20 = [
                    M31_1444891767,
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_3)),
                    state_3_id_col62,
                ];

                *sub_component_inputs.memory_id_to_big[7] = state_3_id_col62;
                *lookup_data.memory_id_to_big_21 = [
                    M31_1662111297,
                    state_3_id_col62,
                    ((low_16_bits_col57) - ((low_7_ms_bits_col59) * (M31_512))),
                    ((low_7_ms_bits_col59) + ((high_2_ls_bits_tmp_40cd9_82) * (M31_128))),
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

                let read_u_32_output_tmp_40cd9_86 = expected_word_tmp_40cd9_79;

                // Read U 32.

                let memory_address_to_id_value_tmp_40cd9_87 = memory_address_to_id_state
                    .deduce_output(((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_4)));
                let memory_id_to_big_value_tmp_40cd9_88 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_40cd9_87);
                let tmp_40cd9_89 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_40cd9_88.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col63 = ((((memory_id_to_big_value_tmp_40cd9_88.get_m31(1))
                    - ((tmp_40cd9_89.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_40cd9_88.get_m31(0)));
                *row[63] = low_16_bits_col63;
                let high_16_bits_col64 = ((((memory_id_to_big_value_tmp_40cd9_88.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_40cd9_88.get_m31(2)) * (M31_4)))
                    + (tmp_40cd9_89.as_m31()));
                *row[64] = high_16_bits_col64;
                let expected_word_tmp_40cd9_90 =
                    PackedUInt32::from_limbs([low_16_bits_col63, high_16_bits_col64]);

                // Verify U 32.

                let low_7_ms_bits_tmp_40cd9_91 = ((expected_word_tmp_40cd9_90.low()) >> (UInt16_9));
                let low_7_ms_bits_col65 = low_7_ms_bits_tmp_40cd9_91.as_m31();
                *row[65] = low_7_ms_bits_col65;
                let high_14_ms_bits_tmp_40cd9_92 =
                    ((expected_word_tmp_40cd9_90.high()) >> (UInt16_2));
                let high_14_ms_bits_col66 = high_14_ms_bits_tmp_40cd9_92.as_m31();
                *row[66] = high_14_ms_bits_col66;
                let high_2_ls_bits_tmp_40cd9_93 =
                    ((high_16_bits_col64) - ((high_14_ms_bits_col66) * (M31_4)));
                let high_5_ms_bits_tmp_40cd9_94 = ((high_14_ms_bits_tmp_40cd9_92) >> (UInt16_9));
                let high_5_ms_bits_col67 = high_5_ms_bits_tmp_40cd9_94.as_m31();
                *row[67] = high_5_ms_bits_col67;
                *sub_component_inputs.range_check_7_2_5[5] = [
                    low_7_ms_bits_col65,
                    high_2_ls_bits_tmp_40cd9_93,
                    high_5_ms_bits_col67,
                ];
                *lookup_data.range_check_7_2_5_22 = [
                    M31_371240602,
                    low_7_ms_bits_col65,
                    high_2_ls_bits_tmp_40cd9_93,
                    high_5_ms_bits_col67,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_40cd9_95 = memory_address_to_id_state
                    .deduce_output(((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_4)));
                let state_4_id_col68 = memory_address_to_id_value_tmp_40cd9_95;
                *row[68] = state_4_id_col68;
                *sub_component_inputs.memory_address_to_id[8] =
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_4));
                *lookup_data.memory_address_to_id_23 = [
                    M31_1444891767,
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_4)),
                    state_4_id_col68,
                ];

                *sub_component_inputs.memory_id_to_big[8] = state_4_id_col68;
                *lookup_data.memory_id_to_big_24 = [
                    M31_1662111297,
                    state_4_id_col68,
                    ((low_16_bits_col63) - ((low_7_ms_bits_col65) * (M31_512))),
                    ((low_7_ms_bits_col65) + ((high_2_ls_bits_tmp_40cd9_93) * (M31_128))),
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

                let read_u_32_output_tmp_40cd9_97 = expected_word_tmp_40cd9_90;

                // Read U 32.

                let memory_address_to_id_value_tmp_40cd9_98 = memory_address_to_id_state
                    .deduce_output(((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_5)));
                let memory_id_to_big_value_tmp_40cd9_99 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_40cd9_98);
                let tmp_40cd9_100 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_40cd9_99.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col69 = ((((memory_id_to_big_value_tmp_40cd9_99.get_m31(1))
                    - ((tmp_40cd9_100.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_40cd9_99.get_m31(0)));
                *row[69] = low_16_bits_col69;
                let high_16_bits_col70 = ((((memory_id_to_big_value_tmp_40cd9_99.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_40cd9_99.get_m31(2)) * (M31_4)))
                    + (tmp_40cd9_100.as_m31()));
                *row[70] = high_16_bits_col70;
                let expected_word_tmp_40cd9_101 =
                    PackedUInt32::from_limbs([low_16_bits_col69, high_16_bits_col70]);

                // Verify U 32.

                let low_7_ms_bits_tmp_40cd9_102 =
                    ((expected_word_tmp_40cd9_101.low()) >> (UInt16_9));
                let low_7_ms_bits_col71 = low_7_ms_bits_tmp_40cd9_102.as_m31();
                *row[71] = low_7_ms_bits_col71;
                let high_14_ms_bits_tmp_40cd9_103 =
                    ((expected_word_tmp_40cd9_101.high()) >> (UInt16_2));
                let high_14_ms_bits_col72 = high_14_ms_bits_tmp_40cd9_103.as_m31();
                *row[72] = high_14_ms_bits_col72;
                let high_2_ls_bits_tmp_40cd9_104 =
                    ((high_16_bits_col70) - ((high_14_ms_bits_col72) * (M31_4)));
                let high_5_ms_bits_tmp_40cd9_105 = ((high_14_ms_bits_tmp_40cd9_103) >> (UInt16_9));
                let high_5_ms_bits_col73 = high_5_ms_bits_tmp_40cd9_105.as_m31();
                *row[73] = high_5_ms_bits_col73;
                *sub_component_inputs.range_check_7_2_5[6] = [
                    low_7_ms_bits_col71,
                    high_2_ls_bits_tmp_40cd9_104,
                    high_5_ms_bits_col73,
                ];
                *lookup_data.range_check_7_2_5_25 = [
                    M31_371240602,
                    low_7_ms_bits_col71,
                    high_2_ls_bits_tmp_40cd9_104,
                    high_5_ms_bits_col73,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_40cd9_106 = memory_address_to_id_state
                    .deduce_output(((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_5)));
                let state_5_id_col74 = memory_address_to_id_value_tmp_40cd9_106;
                *row[74] = state_5_id_col74;
                *sub_component_inputs.memory_address_to_id[9] =
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_5));
                *lookup_data.memory_address_to_id_26 = [
                    M31_1444891767,
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_5)),
                    state_5_id_col74,
                ];

                *sub_component_inputs.memory_id_to_big[9] = state_5_id_col74;
                *lookup_data.memory_id_to_big_27 = [
                    M31_1662111297,
                    state_5_id_col74,
                    ((low_16_bits_col69) - ((low_7_ms_bits_col71) * (M31_512))),
                    ((low_7_ms_bits_col71) + ((high_2_ls_bits_tmp_40cd9_104) * (M31_128))),
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

                let read_u_32_output_tmp_40cd9_108 = expected_word_tmp_40cd9_101;

                // Read U 32.

                let memory_address_to_id_value_tmp_40cd9_109 = memory_address_to_id_state
                    .deduce_output(((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_6)));
                let memory_id_to_big_value_tmp_40cd9_110 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_40cd9_109);
                let tmp_40cd9_111 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_40cd9_110.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col75 = ((((memory_id_to_big_value_tmp_40cd9_110.get_m31(1))
                    - ((tmp_40cd9_111.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_40cd9_110.get_m31(0)));
                *row[75] = low_16_bits_col75;
                let high_16_bits_col76 = ((((memory_id_to_big_value_tmp_40cd9_110.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_40cd9_110.get_m31(2)) * (M31_4)))
                    + (tmp_40cd9_111.as_m31()));
                *row[76] = high_16_bits_col76;
                let expected_word_tmp_40cd9_112 =
                    PackedUInt32::from_limbs([low_16_bits_col75, high_16_bits_col76]);

                // Verify U 32.

                let low_7_ms_bits_tmp_40cd9_113 =
                    ((expected_word_tmp_40cd9_112.low()) >> (UInt16_9));
                let low_7_ms_bits_col77 = low_7_ms_bits_tmp_40cd9_113.as_m31();
                *row[77] = low_7_ms_bits_col77;
                let high_14_ms_bits_tmp_40cd9_114 =
                    ((expected_word_tmp_40cd9_112.high()) >> (UInt16_2));
                let high_14_ms_bits_col78 = high_14_ms_bits_tmp_40cd9_114.as_m31();
                *row[78] = high_14_ms_bits_col78;
                let high_2_ls_bits_tmp_40cd9_115 =
                    ((high_16_bits_col76) - ((high_14_ms_bits_col78) * (M31_4)));
                let high_5_ms_bits_tmp_40cd9_116 = ((high_14_ms_bits_tmp_40cd9_114) >> (UInt16_9));
                let high_5_ms_bits_col79 = high_5_ms_bits_tmp_40cd9_116.as_m31();
                *row[79] = high_5_ms_bits_col79;
                *sub_component_inputs.range_check_7_2_5[7] = [
                    low_7_ms_bits_col77,
                    high_2_ls_bits_tmp_40cd9_115,
                    high_5_ms_bits_col79,
                ];
                *lookup_data.range_check_7_2_5_28 = [
                    M31_371240602,
                    low_7_ms_bits_col77,
                    high_2_ls_bits_tmp_40cd9_115,
                    high_5_ms_bits_col79,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_40cd9_117 = memory_address_to_id_state
                    .deduce_output(((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_6)));
                let state_6_id_col80 = memory_address_to_id_value_tmp_40cd9_117;
                *row[80] = state_6_id_col80;
                *sub_component_inputs.memory_address_to_id[10] =
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_6));
                *lookup_data.memory_address_to_id_29 = [
                    M31_1444891767,
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_6)),
                    state_6_id_col80,
                ];

                *sub_component_inputs.memory_id_to_big[10] = state_6_id_col80;
                *lookup_data.memory_id_to_big_30 = [
                    M31_1662111297,
                    state_6_id_col80,
                    ((low_16_bits_col75) - ((low_7_ms_bits_col77) * (M31_512))),
                    ((low_7_ms_bits_col77) + ((high_2_ls_bits_tmp_40cd9_115) * (M31_128))),
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

                let read_u_32_output_tmp_40cd9_119 = expected_word_tmp_40cd9_112;

                // Read U 32.

                let memory_address_to_id_value_tmp_40cd9_120 = memory_address_to_id_state
                    .deduce_output(((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_7)));
                let memory_id_to_big_value_tmp_40cd9_121 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_40cd9_120);
                let tmp_40cd9_122 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_40cd9_121.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col81 = ((((memory_id_to_big_value_tmp_40cd9_121.get_m31(1))
                    - ((tmp_40cd9_122.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_40cd9_121.get_m31(0)));
                *row[81] = low_16_bits_col81;
                let high_16_bits_col82 = ((((memory_id_to_big_value_tmp_40cd9_121.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_40cd9_121.get_m31(2)) * (M31_4)))
                    + (tmp_40cd9_122.as_m31()));
                *row[82] = high_16_bits_col82;
                let expected_word_tmp_40cd9_123 =
                    PackedUInt32::from_limbs([low_16_bits_col81, high_16_bits_col82]);

                // Verify U 32.

                let low_7_ms_bits_tmp_40cd9_124 =
                    ((expected_word_tmp_40cd9_123.low()) >> (UInt16_9));
                let low_7_ms_bits_col83 = low_7_ms_bits_tmp_40cd9_124.as_m31();
                *row[83] = low_7_ms_bits_col83;
                let high_14_ms_bits_tmp_40cd9_125 =
                    ((expected_word_tmp_40cd9_123.high()) >> (UInt16_2));
                let high_14_ms_bits_col84 = high_14_ms_bits_tmp_40cd9_125.as_m31();
                *row[84] = high_14_ms_bits_col84;
                let high_2_ls_bits_tmp_40cd9_126 =
                    ((high_16_bits_col82) - ((high_14_ms_bits_col84) * (M31_4)));
                let high_5_ms_bits_tmp_40cd9_127 = ((high_14_ms_bits_tmp_40cd9_125) >> (UInt16_9));
                let high_5_ms_bits_col85 = high_5_ms_bits_tmp_40cd9_127.as_m31();
                *row[85] = high_5_ms_bits_col85;
                *sub_component_inputs.range_check_7_2_5[8] = [
                    low_7_ms_bits_col83,
                    high_2_ls_bits_tmp_40cd9_126,
                    high_5_ms_bits_col85,
                ];
                *lookup_data.range_check_7_2_5_31 = [
                    M31_371240602,
                    low_7_ms_bits_col83,
                    high_2_ls_bits_tmp_40cd9_126,
                    high_5_ms_bits_col85,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_40cd9_128 = memory_address_to_id_state
                    .deduce_output(((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_7)));
                let state_7_id_col86 = memory_address_to_id_value_tmp_40cd9_128;
                *row[86] = state_7_id_col86;
                *sub_component_inputs.memory_address_to_id[11] =
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_7));
                *lookup_data.memory_address_to_id_32 = [
                    M31_1444891767,
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[0]) + (M31_7)),
                    state_7_id_col86,
                ];

                *sub_component_inputs.memory_id_to_big[11] = state_7_id_col86;
                *lookup_data.memory_id_to_big_33 = [
                    M31_1662111297,
                    state_7_id_col86,
                    ((low_16_bits_col81) - ((low_7_ms_bits_col83) * (M31_512))),
                    ((low_7_ms_bits_col83) + ((high_2_ls_bits_tmp_40cd9_126) * (M31_128))),
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

                let read_u_32_output_tmp_40cd9_130 = expected_word_tmp_40cd9_123;

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_40cd9_131 =
                    ((decode_blake_opcode_output_tmp_40cd9_42.1.low()) >> (UInt16_8));
                let ms_8_bits_col87 = ms_8_bits_tmp_40cd9_131.as_m31();
                *row[87] = ms_8_bits_col87;
                let split_16_low_part_size_8_output_tmp_40cd9_132 = [
                    ((low_16_bits_col33) - ((ms_8_bits_col87) * (M31_256))),
                    ms_8_bits_col87,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_40cd9_133 =
                    ((decode_blake_opcode_output_tmp_40cd9_42.1.high()) >> (UInt16_8));
                let ms_8_bits_col88 = ms_8_bits_tmp_40cd9_133.as_m31();
                *row[88] = ms_8_bits_col88;
                let split_16_low_part_size_8_output_tmp_40cd9_134 = [
                    ((high_16_bits_col34) - ((ms_8_bits_col88) * (M31_256))),
                    ms_8_bits_col88,
                ];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_40cd9_135 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_40cd9_132[0]))
                        ^ (UInt16_127));
                let xor_col89 = xor_tmp_40cd9_135.as_m31();
                *row[89] = xor_col89;
                *sub_component_inputs.verify_bitwise_xor_8[0] = [
                    split_16_low_part_size_8_output_tmp_40cd9_132[0],
                    M31_127,
                    xor_col89,
                ];
                *lookup_data.verify_bitwise_xor_8_34 = [
                    M31_112558620,
                    split_16_low_part_size_8_output_tmp_40cd9_132[0],
                    M31_127,
                    xor_col89,
                ];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_40cd9_137 = ((PackedUInt16::from_m31(ms_8_bits_col87)) ^ (UInt16_82));
                let xor_col90 = xor_tmp_40cd9_137.as_m31();
                *row[90] = xor_col90;
                *sub_component_inputs.verify_bitwise_xor_8[1] =
                    [ms_8_bits_col87, M31_82, xor_col90];
                *lookup_data.verify_bitwise_xor_8_35 =
                    [M31_112558620, ms_8_bits_col87, M31_82, xor_col90];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_40cd9_139 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_40cd9_134[0]))
                        ^ (UInt16_14));
                let xor_col91 = xor_tmp_40cd9_139.as_m31();
                *row[91] = xor_col91;
                *sub_component_inputs.verify_bitwise_xor_8[2] = [
                    split_16_low_part_size_8_output_tmp_40cd9_134[0],
                    M31_14,
                    xor_col91,
                ];
                *lookup_data.verify_bitwise_xor_8_36 = [
                    M31_112558620,
                    split_16_low_part_size_8_output_tmp_40cd9_134[0],
                    M31_14,
                    xor_col91,
                ];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_40cd9_141 = ((PackedUInt16::from_m31(ms_8_bits_col88)) ^ (UInt16_81));
                let xor_col92 = xor_tmp_40cd9_141.as_m31();
                *row[92] = xor_col92;
                *sub_component_inputs.verify_bitwise_xor_8[3] =
                    [ms_8_bits_col88, M31_81, xor_col92];
                *lookup_data.verify_bitwise_xor_8_37 =
                    [M31_112558620, ms_8_bits_col88, M31_81, xor_col92];

                let create_blake_round_input_output_tmp_40cd9_143 = [
                    read_u_32_output_tmp_40cd9_53,
                    read_u_32_output_tmp_40cd9_64,
                    read_u_32_output_tmp_40cd9_75,
                    read_u_32_output_tmp_40cd9_86,
                    read_u_32_output_tmp_40cd9_97,
                    read_u_32_output_tmp_40cd9_108,
                    read_u_32_output_tmp_40cd9_119,
                    read_u_32_output_tmp_40cd9_130,
                    UInt32_1779033703,
                    UInt32_3144134277,
                    UInt32_1013904242,
                    UInt32_2773480762,
                    PackedUInt32::from_limbs([
                        ((xor_col89) + ((xor_col90) * (M31_256))),
                        ((xor_col91) + ((xor_col92) * (M31_256))),
                    ]),
                    UInt32_2600822924,
                    PackedUInt32::from_limbs([
                        (((decode_blake_opcode_output_tmp_40cd9_42.2[1].as_m31()) * (M31_9812))
                            + (((M31_1)
                                - (decode_blake_opcode_output_tmp_40cd9_42.2[1].as_m31()))
                                * (M31_55723))),
                        (((decode_blake_opcode_output_tmp_40cd9_42.2[1].as_m31()) * (M31_57468))
                            + (((M31_1)
                                - (decode_blake_opcode_output_tmp_40cd9_42.2[1].as_m31()))
                                * (M31_8067))),
                    ]),
                    UInt32_1541459225,
                ];

                *lookup_data.blake_round_38 = [
                    M31_40528774,
                    seq,
                    M31_0,
                    low_16_bits_col39,
                    high_16_bits_col40,
                    low_16_bits_col45,
                    high_16_bits_col46,
                    low_16_bits_col51,
                    high_16_bits_col52,
                    low_16_bits_col57,
                    high_16_bits_col58,
                    low_16_bits_col63,
                    high_16_bits_col64,
                    low_16_bits_col69,
                    high_16_bits_col70,
                    low_16_bits_col75,
                    high_16_bits_col76,
                    low_16_bits_col81,
                    high_16_bits_col82,
                    M31_58983,
                    M31_27145,
                    M31_44677,
                    M31_47975,
                    M31_62322,
                    M31_15470,
                    M31_62778,
                    M31_42319,
                    create_blake_round_input_output_tmp_40cd9_143[12]
                        .low()
                        .as_m31(),
                    create_blake_round_input_output_tmp_40cd9_143[12]
                        .high()
                        .as_m31(),
                    M31_26764,
                    M31_39685,
                    create_blake_round_input_output_tmp_40cd9_143[14]
                        .low()
                        .as_m31(),
                    create_blake_round_input_output_tmp_40cd9_143[14]
                        .high()
                        .as_m31(),
                    M31_52505,
                    M31_23520,
                    decode_blake_opcode_output_tmp_40cd9_42.0[1],
                ];
                *sub_component_inputs.blake_round[0] = (
                    seq,
                    M31_0,
                    (
                        [
                            create_blake_round_input_output_tmp_40cd9_143[0],
                            create_blake_round_input_output_tmp_40cd9_143[1],
                            create_blake_round_input_output_tmp_40cd9_143[2],
                            create_blake_round_input_output_tmp_40cd9_143[3],
                            create_blake_round_input_output_tmp_40cd9_143[4],
                            create_blake_round_input_output_tmp_40cd9_143[5],
                            create_blake_round_input_output_tmp_40cd9_143[6],
                            create_blake_round_input_output_tmp_40cd9_143[7],
                            UInt32_1779033703,
                            UInt32_3144134277,
                            UInt32_1013904242,
                            UInt32_2773480762,
                            create_blake_round_input_output_tmp_40cd9_143[12],
                            UInt32_2600822924,
                            create_blake_round_input_output_tmp_40cd9_143[14],
                            UInt32_1541459225,
                        ],
                        decode_blake_opcode_output_tmp_40cd9_42.0[1],
                    ),
                );
                let blake_round_output_round_0_tmp_40cd9_145 = blake_round_state.deduce_output((
                    seq,
                    M31_0,
                    (
                        [
                            create_blake_round_input_output_tmp_40cd9_143[0],
                            create_blake_round_input_output_tmp_40cd9_143[1],
                            create_blake_round_input_output_tmp_40cd9_143[2],
                            create_blake_round_input_output_tmp_40cd9_143[3],
                            create_blake_round_input_output_tmp_40cd9_143[4],
                            create_blake_round_input_output_tmp_40cd9_143[5],
                            create_blake_round_input_output_tmp_40cd9_143[6],
                            create_blake_round_input_output_tmp_40cd9_143[7],
                            UInt32_1779033703,
                            UInt32_3144134277,
                            UInt32_1013904242,
                            UInt32_2773480762,
                            create_blake_round_input_output_tmp_40cd9_143[12],
                            UInt32_2600822924,
                            create_blake_round_input_output_tmp_40cd9_143[14],
                            UInt32_1541459225,
                        ],
                        decode_blake_opcode_output_tmp_40cd9_42.0[1],
                    ),
                ));
                *sub_component_inputs.blake_round[1] = (
                    seq,
                    M31_1,
                    (
                        [
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[0],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[1],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[2],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[3],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[4],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[5],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[6],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[7],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[8],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[9],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[10],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[11],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[12],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[13],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[14],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[15],
                        ],
                        blake_round_output_round_0_tmp_40cd9_145.2 .1,
                    ),
                );
                let blake_round_output_round_1_tmp_40cd9_146 = blake_round_state.deduce_output((
                    seq,
                    M31_1,
                    (
                        [
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[0],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[1],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[2],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[3],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[4],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[5],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[6],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[7],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[8],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[9],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[10],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[11],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[12],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[13],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[14],
                            blake_round_output_round_0_tmp_40cd9_145.2 .0[15],
                        ],
                        blake_round_output_round_0_tmp_40cd9_145.2 .1,
                    ),
                ));
                *sub_component_inputs.blake_round[2] = (
                    seq,
                    M31_2,
                    (
                        [
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[0],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[1],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[2],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[3],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[4],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[5],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[6],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[7],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[8],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[9],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[10],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[11],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[12],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[13],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[14],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[15],
                        ],
                        blake_round_output_round_1_tmp_40cd9_146.2 .1,
                    ),
                );
                let blake_round_output_round_2_tmp_40cd9_147 = blake_round_state.deduce_output((
                    seq,
                    M31_2,
                    (
                        [
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[0],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[1],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[2],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[3],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[4],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[5],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[6],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[7],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[8],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[9],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[10],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[11],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[12],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[13],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[14],
                            blake_round_output_round_1_tmp_40cd9_146.2 .0[15],
                        ],
                        blake_round_output_round_1_tmp_40cd9_146.2 .1,
                    ),
                ));
                *sub_component_inputs.blake_round[3] = (
                    seq,
                    M31_3,
                    (
                        [
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[0],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[1],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[2],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[3],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[4],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[5],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[6],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[7],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[8],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[9],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[10],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[11],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[12],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[13],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[14],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[15],
                        ],
                        blake_round_output_round_2_tmp_40cd9_147.2 .1,
                    ),
                );
                let blake_round_output_round_3_tmp_40cd9_148 = blake_round_state.deduce_output((
                    seq,
                    M31_3,
                    (
                        [
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[0],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[1],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[2],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[3],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[4],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[5],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[6],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[7],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[8],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[9],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[10],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[11],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[12],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[13],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[14],
                            blake_round_output_round_2_tmp_40cd9_147.2 .0[15],
                        ],
                        blake_round_output_round_2_tmp_40cd9_147.2 .1,
                    ),
                ));
                *sub_component_inputs.blake_round[4] = (
                    seq,
                    M31_4,
                    (
                        [
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[0],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[1],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[2],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[3],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[4],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[5],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[6],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[7],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[8],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[9],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[10],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[11],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[12],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[13],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[14],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[15],
                        ],
                        blake_round_output_round_3_tmp_40cd9_148.2 .1,
                    ),
                );
                let blake_round_output_round_4_tmp_40cd9_149 = blake_round_state.deduce_output((
                    seq,
                    M31_4,
                    (
                        [
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[0],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[1],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[2],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[3],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[4],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[5],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[6],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[7],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[8],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[9],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[10],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[11],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[12],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[13],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[14],
                            blake_round_output_round_3_tmp_40cd9_148.2 .0[15],
                        ],
                        blake_round_output_round_3_tmp_40cd9_148.2 .1,
                    ),
                ));
                *sub_component_inputs.blake_round[5] = (
                    seq,
                    M31_5,
                    (
                        [
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[0],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[1],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[2],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[3],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[4],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[5],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[6],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[7],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[8],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[9],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[10],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[11],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[12],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[13],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[14],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[15],
                        ],
                        blake_round_output_round_4_tmp_40cd9_149.2 .1,
                    ),
                );
                let blake_round_output_round_5_tmp_40cd9_150 = blake_round_state.deduce_output((
                    seq,
                    M31_5,
                    (
                        [
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[0],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[1],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[2],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[3],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[4],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[5],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[6],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[7],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[8],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[9],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[10],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[11],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[12],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[13],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[14],
                            blake_round_output_round_4_tmp_40cd9_149.2 .0[15],
                        ],
                        blake_round_output_round_4_tmp_40cd9_149.2 .1,
                    ),
                ));
                *sub_component_inputs.blake_round[6] = (
                    seq,
                    M31_6,
                    (
                        [
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[0],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[1],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[2],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[3],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[4],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[5],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[6],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[7],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[8],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[9],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[10],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[11],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[12],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[13],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[14],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[15],
                        ],
                        blake_round_output_round_5_tmp_40cd9_150.2 .1,
                    ),
                );
                let blake_round_output_round_6_tmp_40cd9_151 = blake_round_state.deduce_output((
                    seq,
                    M31_6,
                    (
                        [
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[0],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[1],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[2],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[3],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[4],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[5],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[6],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[7],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[8],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[9],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[10],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[11],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[12],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[13],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[14],
                            blake_round_output_round_5_tmp_40cd9_150.2 .0[15],
                        ],
                        blake_round_output_round_5_tmp_40cd9_150.2 .1,
                    ),
                ));
                *sub_component_inputs.blake_round[7] = (
                    seq,
                    M31_7,
                    (
                        [
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[0],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[1],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[2],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[3],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[4],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[5],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[6],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[7],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[8],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[9],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[10],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[11],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[12],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[13],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[14],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[15],
                        ],
                        blake_round_output_round_6_tmp_40cd9_151.2 .1,
                    ),
                );
                let blake_round_output_round_7_tmp_40cd9_152 = blake_round_state.deduce_output((
                    seq,
                    M31_7,
                    (
                        [
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[0],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[1],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[2],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[3],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[4],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[5],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[6],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[7],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[8],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[9],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[10],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[11],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[12],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[13],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[14],
                            blake_round_output_round_6_tmp_40cd9_151.2 .0[15],
                        ],
                        blake_round_output_round_6_tmp_40cd9_151.2 .1,
                    ),
                ));
                *sub_component_inputs.blake_round[8] = (
                    seq,
                    M31_8,
                    (
                        [
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[0],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[1],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[2],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[3],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[4],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[5],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[6],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[7],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[8],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[9],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[10],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[11],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[12],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[13],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[14],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[15],
                        ],
                        blake_round_output_round_7_tmp_40cd9_152.2 .1,
                    ),
                );
                let blake_round_output_round_8_tmp_40cd9_153 = blake_round_state.deduce_output((
                    seq,
                    M31_8,
                    (
                        [
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[0],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[1],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[2],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[3],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[4],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[5],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[6],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[7],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[8],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[9],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[10],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[11],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[12],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[13],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[14],
                            blake_round_output_round_7_tmp_40cd9_152.2 .0[15],
                        ],
                        blake_round_output_round_7_tmp_40cd9_152.2 .1,
                    ),
                ));
                *sub_component_inputs.blake_round[9] = (
                    seq,
                    M31_9,
                    (
                        [
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[0],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[1],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[2],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[3],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[4],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[5],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[6],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[7],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[8],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[9],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[10],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[11],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[12],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[13],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[14],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[15],
                        ],
                        blake_round_output_round_8_tmp_40cd9_153.2 .1,
                    ),
                );
                let blake_round_output_round_9_tmp_40cd9_154 = blake_round_state.deduce_output((
                    seq,
                    M31_9,
                    (
                        [
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[0],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[1],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[2],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[3],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[4],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[5],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[6],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[7],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[8],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[9],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[10],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[11],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[12],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[13],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[14],
                            blake_round_output_round_8_tmp_40cd9_153.2 .0[15],
                        ],
                        blake_round_output_round_8_tmp_40cd9_153.2 .1,
                    ),
                ));
                let blake_round_output_limb_0_col93 = blake_round_output_round_9_tmp_40cd9_154.2 .0
                    [0]
                .low()
                .as_m31();
                *row[93] = blake_round_output_limb_0_col93;
                let blake_round_output_limb_1_col94 = blake_round_output_round_9_tmp_40cd9_154.2 .0
                    [0]
                .high()
                .as_m31();
                *row[94] = blake_round_output_limb_1_col94;
                let blake_round_output_limb_2_col95 = blake_round_output_round_9_tmp_40cd9_154.2 .0
                    [1]
                .low()
                .as_m31();
                *row[95] = blake_round_output_limb_2_col95;
                let blake_round_output_limb_3_col96 = blake_round_output_round_9_tmp_40cd9_154.2 .0
                    [1]
                .high()
                .as_m31();
                *row[96] = blake_round_output_limb_3_col96;
                let blake_round_output_limb_4_col97 = blake_round_output_round_9_tmp_40cd9_154.2 .0
                    [2]
                .low()
                .as_m31();
                *row[97] = blake_round_output_limb_4_col97;
                let blake_round_output_limb_5_col98 = blake_round_output_round_9_tmp_40cd9_154.2 .0
                    [2]
                .high()
                .as_m31();
                *row[98] = blake_round_output_limb_5_col98;
                let blake_round_output_limb_6_col99 = blake_round_output_round_9_tmp_40cd9_154.2 .0
                    [3]
                .low()
                .as_m31();
                *row[99] = blake_round_output_limb_6_col99;
                let blake_round_output_limb_7_col100 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[3]
                        .high()
                        .as_m31();
                *row[100] = blake_round_output_limb_7_col100;
                let blake_round_output_limb_8_col101 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[4]
                        .low()
                        .as_m31();
                *row[101] = blake_round_output_limb_8_col101;
                let blake_round_output_limb_9_col102 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[4]
                        .high()
                        .as_m31();
                *row[102] = blake_round_output_limb_9_col102;
                let blake_round_output_limb_10_col103 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[5]
                        .low()
                        .as_m31();
                *row[103] = blake_round_output_limb_10_col103;
                let blake_round_output_limb_11_col104 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[5]
                        .high()
                        .as_m31();
                *row[104] = blake_round_output_limb_11_col104;
                let blake_round_output_limb_12_col105 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[6]
                        .low()
                        .as_m31();
                *row[105] = blake_round_output_limb_12_col105;
                let blake_round_output_limb_13_col106 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[6]
                        .high()
                        .as_m31();
                *row[106] = blake_round_output_limb_13_col106;
                let blake_round_output_limb_14_col107 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[7]
                        .low()
                        .as_m31();
                *row[107] = blake_round_output_limb_14_col107;
                let blake_round_output_limb_15_col108 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[7]
                        .high()
                        .as_m31();
                *row[108] = blake_round_output_limb_15_col108;
                let blake_round_output_limb_16_col109 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[8]
                        .low()
                        .as_m31();
                *row[109] = blake_round_output_limb_16_col109;
                let blake_round_output_limb_17_col110 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[8]
                        .high()
                        .as_m31();
                *row[110] = blake_round_output_limb_17_col110;
                let blake_round_output_limb_18_col111 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[9]
                        .low()
                        .as_m31();
                *row[111] = blake_round_output_limb_18_col111;
                let blake_round_output_limb_19_col112 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[9]
                        .high()
                        .as_m31();
                *row[112] = blake_round_output_limb_19_col112;
                let blake_round_output_limb_20_col113 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[10]
                        .low()
                        .as_m31();
                *row[113] = blake_round_output_limb_20_col113;
                let blake_round_output_limb_21_col114 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[10]
                        .high()
                        .as_m31();
                *row[114] = blake_round_output_limb_21_col114;
                let blake_round_output_limb_22_col115 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[11]
                        .low()
                        .as_m31();
                *row[115] = blake_round_output_limb_22_col115;
                let blake_round_output_limb_23_col116 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[11]
                        .high()
                        .as_m31();
                *row[116] = blake_round_output_limb_23_col116;
                let blake_round_output_limb_24_col117 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[12]
                        .low()
                        .as_m31();
                *row[117] = blake_round_output_limb_24_col117;
                let blake_round_output_limb_25_col118 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[12]
                        .high()
                        .as_m31();
                *row[118] = blake_round_output_limb_25_col118;
                let blake_round_output_limb_26_col119 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[13]
                        .low()
                        .as_m31();
                *row[119] = blake_round_output_limb_26_col119;
                let blake_round_output_limb_27_col120 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[13]
                        .high()
                        .as_m31();
                *row[120] = blake_round_output_limb_27_col120;
                let blake_round_output_limb_28_col121 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[14]
                        .low()
                        .as_m31();
                *row[121] = blake_round_output_limb_28_col121;
                let blake_round_output_limb_29_col122 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[14]
                        .high()
                        .as_m31();
                *row[122] = blake_round_output_limb_29_col122;
                let blake_round_output_limb_30_col123 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[15]
                        .low()
                        .as_m31();
                *row[123] = blake_round_output_limb_30_col123;
                let blake_round_output_limb_31_col124 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[15]
                        .high()
                        .as_m31();
                *row[124] = blake_round_output_limb_31_col124;
                let blake_round_output_limb_32_col125 =
                    blake_round_output_round_9_tmp_40cd9_154.2 .1;
                *row[125] = blake_round_output_limb_32_col125;
                *lookup_data.blake_round_39 = [
                    M31_40528774,
                    seq,
                    M31_10,
                    blake_round_output_limb_0_col93,
                    blake_round_output_limb_1_col94,
                    blake_round_output_limb_2_col95,
                    blake_round_output_limb_3_col96,
                    blake_round_output_limb_4_col97,
                    blake_round_output_limb_5_col98,
                    blake_round_output_limb_6_col99,
                    blake_round_output_limb_7_col100,
                    blake_round_output_limb_8_col101,
                    blake_round_output_limb_9_col102,
                    blake_round_output_limb_10_col103,
                    blake_round_output_limb_11_col104,
                    blake_round_output_limb_12_col105,
                    blake_round_output_limb_13_col106,
                    blake_round_output_limb_14_col107,
                    blake_round_output_limb_15_col108,
                    blake_round_output_limb_16_col109,
                    blake_round_output_limb_17_col110,
                    blake_round_output_limb_18_col111,
                    blake_round_output_limb_19_col112,
                    blake_round_output_limb_20_col113,
                    blake_round_output_limb_21_col114,
                    blake_round_output_limb_22_col115,
                    blake_round_output_limb_23_col116,
                    blake_round_output_limb_24_col117,
                    blake_round_output_limb_25_col118,
                    blake_round_output_limb_26_col119,
                    blake_round_output_limb_27_col120,
                    blake_round_output_limb_28_col121,
                    blake_round_output_limb_29_col122,
                    blake_round_output_limb_30_col123,
                    blake_round_output_limb_31_col124,
                    blake_round_output_limb_32_col125,
                ];

                // Create Blake Output.

                *sub_component_inputs.triple_xor_32[0] = [
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[0],
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[8],
                    create_blake_round_input_output_tmp_40cd9_143[0],
                ];
                let triple_xor_32_output_tmp_40cd9_155 = PackedTripleXor32::deduce_output([
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[0],
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[8],
                    create_blake_round_input_output_tmp_40cd9_143[0],
                ]);
                let triple_xor_32_output_limb_0_col126 =
                    triple_xor_32_output_tmp_40cd9_155.low().as_m31();
                *row[126] = triple_xor_32_output_limb_0_col126;
                let triple_xor_32_output_limb_1_col127 =
                    triple_xor_32_output_tmp_40cd9_155.high().as_m31();
                *row[127] = triple_xor_32_output_limb_1_col127;
                *lookup_data.triple_xor_32_40 = [
                    M31_990559919,
                    blake_round_output_limb_0_col93,
                    blake_round_output_limb_1_col94,
                    blake_round_output_limb_16_col109,
                    blake_round_output_limb_17_col110,
                    low_16_bits_col39,
                    high_16_bits_col40,
                    triple_xor_32_output_limb_0_col126,
                    triple_xor_32_output_limb_1_col127,
                ];
                *sub_component_inputs.triple_xor_32[1] = [
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[1],
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[9],
                    create_blake_round_input_output_tmp_40cd9_143[1],
                ];
                let triple_xor_32_output_tmp_40cd9_156 = PackedTripleXor32::deduce_output([
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[1],
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[9],
                    create_blake_round_input_output_tmp_40cd9_143[1],
                ]);
                let triple_xor_32_output_limb_0_col128 =
                    triple_xor_32_output_tmp_40cd9_156.low().as_m31();
                *row[128] = triple_xor_32_output_limb_0_col128;
                let triple_xor_32_output_limb_1_col129 =
                    triple_xor_32_output_tmp_40cd9_156.high().as_m31();
                *row[129] = triple_xor_32_output_limb_1_col129;
                *lookup_data.triple_xor_32_41 = [
                    M31_990559919,
                    blake_round_output_limb_2_col95,
                    blake_round_output_limb_3_col96,
                    blake_round_output_limb_18_col111,
                    blake_round_output_limb_19_col112,
                    low_16_bits_col45,
                    high_16_bits_col46,
                    triple_xor_32_output_limb_0_col128,
                    triple_xor_32_output_limb_1_col129,
                ];
                *sub_component_inputs.triple_xor_32[2] = [
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[2],
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[10],
                    create_blake_round_input_output_tmp_40cd9_143[2],
                ];
                let triple_xor_32_output_tmp_40cd9_157 = PackedTripleXor32::deduce_output([
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[2],
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[10],
                    create_blake_round_input_output_tmp_40cd9_143[2],
                ]);
                let triple_xor_32_output_limb_0_col130 =
                    triple_xor_32_output_tmp_40cd9_157.low().as_m31();
                *row[130] = triple_xor_32_output_limb_0_col130;
                let triple_xor_32_output_limb_1_col131 =
                    triple_xor_32_output_tmp_40cd9_157.high().as_m31();
                *row[131] = triple_xor_32_output_limb_1_col131;
                *lookup_data.triple_xor_32_42 = [
                    M31_990559919,
                    blake_round_output_limb_4_col97,
                    blake_round_output_limb_5_col98,
                    blake_round_output_limb_20_col113,
                    blake_round_output_limb_21_col114,
                    low_16_bits_col51,
                    high_16_bits_col52,
                    triple_xor_32_output_limb_0_col130,
                    triple_xor_32_output_limb_1_col131,
                ];
                *sub_component_inputs.triple_xor_32[3] = [
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[3],
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[11],
                    create_blake_round_input_output_tmp_40cd9_143[3],
                ];
                let triple_xor_32_output_tmp_40cd9_158 = PackedTripleXor32::deduce_output([
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[3],
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[11],
                    create_blake_round_input_output_tmp_40cd9_143[3],
                ]);
                let triple_xor_32_output_limb_0_col132 =
                    triple_xor_32_output_tmp_40cd9_158.low().as_m31();
                *row[132] = triple_xor_32_output_limb_0_col132;
                let triple_xor_32_output_limb_1_col133 =
                    triple_xor_32_output_tmp_40cd9_158.high().as_m31();
                *row[133] = triple_xor_32_output_limb_1_col133;
                *lookup_data.triple_xor_32_43 = [
                    M31_990559919,
                    blake_round_output_limb_6_col99,
                    blake_round_output_limb_7_col100,
                    blake_round_output_limb_22_col115,
                    blake_round_output_limb_23_col116,
                    low_16_bits_col57,
                    high_16_bits_col58,
                    triple_xor_32_output_limb_0_col132,
                    triple_xor_32_output_limb_1_col133,
                ];
                *sub_component_inputs.triple_xor_32[4] = [
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[4],
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[12],
                    create_blake_round_input_output_tmp_40cd9_143[4],
                ];
                let triple_xor_32_output_tmp_40cd9_159 = PackedTripleXor32::deduce_output([
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[4],
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[12],
                    create_blake_round_input_output_tmp_40cd9_143[4],
                ]);
                let triple_xor_32_output_limb_0_col134 =
                    triple_xor_32_output_tmp_40cd9_159.low().as_m31();
                *row[134] = triple_xor_32_output_limb_0_col134;
                let triple_xor_32_output_limb_1_col135 =
                    triple_xor_32_output_tmp_40cd9_159.high().as_m31();
                *row[135] = triple_xor_32_output_limb_1_col135;
                *lookup_data.triple_xor_32_44 = [
                    M31_990559919,
                    blake_round_output_limb_8_col101,
                    blake_round_output_limb_9_col102,
                    blake_round_output_limb_24_col117,
                    blake_round_output_limb_25_col118,
                    low_16_bits_col63,
                    high_16_bits_col64,
                    triple_xor_32_output_limb_0_col134,
                    triple_xor_32_output_limb_1_col135,
                ];
                *sub_component_inputs.triple_xor_32[5] = [
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[5],
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[13],
                    create_blake_round_input_output_tmp_40cd9_143[5],
                ];
                let triple_xor_32_output_tmp_40cd9_160 = PackedTripleXor32::deduce_output([
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[5],
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[13],
                    create_blake_round_input_output_tmp_40cd9_143[5],
                ]);
                let triple_xor_32_output_limb_0_col136 =
                    triple_xor_32_output_tmp_40cd9_160.low().as_m31();
                *row[136] = triple_xor_32_output_limb_0_col136;
                let triple_xor_32_output_limb_1_col137 =
                    triple_xor_32_output_tmp_40cd9_160.high().as_m31();
                *row[137] = triple_xor_32_output_limb_1_col137;
                *lookup_data.triple_xor_32_45 = [
                    M31_990559919,
                    blake_round_output_limb_10_col103,
                    blake_round_output_limb_11_col104,
                    blake_round_output_limb_26_col119,
                    blake_round_output_limb_27_col120,
                    low_16_bits_col69,
                    high_16_bits_col70,
                    triple_xor_32_output_limb_0_col136,
                    triple_xor_32_output_limb_1_col137,
                ];
                *sub_component_inputs.triple_xor_32[6] = [
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[6],
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[14],
                    create_blake_round_input_output_tmp_40cd9_143[6],
                ];
                let triple_xor_32_output_tmp_40cd9_161 = PackedTripleXor32::deduce_output([
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[6],
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[14],
                    create_blake_round_input_output_tmp_40cd9_143[6],
                ]);
                let triple_xor_32_output_limb_0_col138 =
                    triple_xor_32_output_tmp_40cd9_161.low().as_m31();
                *row[138] = triple_xor_32_output_limb_0_col138;
                let triple_xor_32_output_limb_1_col139 =
                    triple_xor_32_output_tmp_40cd9_161.high().as_m31();
                *row[139] = triple_xor_32_output_limb_1_col139;
                *lookup_data.triple_xor_32_46 = [
                    M31_990559919,
                    blake_round_output_limb_12_col105,
                    blake_round_output_limb_13_col106,
                    blake_round_output_limb_28_col121,
                    blake_round_output_limb_29_col122,
                    low_16_bits_col75,
                    high_16_bits_col76,
                    triple_xor_32_output_limb_0_col138,
                    triple_xor_32_output_limb_1_col139,
                ];
                *sub_component_inputs.triple_xor_32[7] = [
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[7],
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[15],
                    create_blake_round_input_output_tmp_40cd9_143[7],
                ];
                let triple_xor_32_output_tmp_40cd9_162 = PackedTripleXor32::deduce_output([
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[7],
                    blake_round_output_round_9_tmp_40cd9_154.2 .0[15],
                    create_blake_round_input_output_tmp_40cd9_143[7],
                ]);
                let triple_xor_32_output_limb_0_col140 =
                    triple_xor_32_output_tmp_40cd9_162.low().as_m31();
                *row[140] = triple_xor_32_output_limb_0_col140;
                let triple_xor_32_output_limb_1_col141 =
                    triple_xor_32_output_tmp_40cd9_162.high().as_m31();
                *row[141] = triple_xor_32_output_limb_1_col141;
                *lookup_data.triple_xor_32_47 = [
                    M31_990559919,
                    blake_round_output_limb_14_col107,
                    blake_round_output_limb_15_col108,
                    blake_round_output_limb_30_col123,
                    blake_round_output_limb_31_col124,
                    low_16_bits_col81,
                    high_16_bits_col82,
                    triple_xor_32_output_limb_0_col140,
                    triple_xor_32_output_limb_1_col141,
                ];
                let create_blake_output_output_tmp_40cd9_163 = [
                    triple_xor_32_output_tmp_40cd9_155,
                    triple_xor_32_output_tmp_40cd9_156,
                    triple_xor_32_output_tmp_40cd9_157,
                    triple_xor_32_output_tmp_40cd9_158,
                    triple_xor_32_output_tmp_40cd9_159,
                    triple_xor_32_output_tmp_40cd9_160,
                    triple_xor_32_output_tmp_40cd9_161,
                    triple_xor_32_output_tmp_40cd9_162,
                ];

                // Verify U 32.

                let low_7_ms_bits_tmp_40cd9_164 =
                    ((create_blake_output_output_tmp_40cd9_163[0].low()) >> (UInt16_9));
                let low_7_ms_bits_col142 = low_7_ms_bits_tmp_40cd9_164.as_m31();
                *row[142] = low_7_ms_bits_col142;
                let high_14_ms_bits_tmp_40cd9_165 =
                    ((create_blake_output_output_tmp_40cd9_163[0].high()) >> (UInt16_2));
                let high_14_ms_bits_col143 = high_14_ms_bits_tmp_40cd9_165.as_m31();
                *row[143] = high_14_ms_bits_col143;
                let high_2_ls_bits_tmp_40cd9_166 =
                    ((triple_xor_32_output_limb_1_col127) - ((high_14_ms_bits_col143) * (M31_4)));
                let high_5_ms_bits_tmp_40cd9_167 = ((high_14_ms_bits_tmp_40cd9_165) >> (UInt16_9));
                let high_5_ms_bits_col144 = high_5_ms_bits_tmp_40cd9_167.as_m31();
                *row[144] = high_5_ms_bits_col144;
                *sub_component_inputs.range_check_7_2_5[9] = [
                    low_7_ms_bits_col142,
                    high_2_ls_bits_tmp_40cd9_166,
                    high_5_ms_bits_col144,
                ];
                *lookup_data.range_check_7_2_5_48 = [
                    M31_371240602,
                    low_7_ms_bits_col142,
                    high_2_ls_bits_tmp_40cd9_166,
                    high_5_ms_bits_col144,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_40cd9_168 = memory_address_to_id_state
                    .deduce_output(decode_blake_opcode_output_tmp_40cd9_42.0[2]);
                let new_state_0_id_col145 = memory_address_to_id_value_tmp_40cd9_168;
                *row[145] = new_state_0_id_col145;
                *sub_component_inputs.memory_address_to_id[12] =
                    decode_blake_opcode_output_tmp_40cd9_42.0[2];
                *lookup_data.memory_address_to_id_49 = [
                    M31_1444891767,
                    decode_blake_opcode_output_tmp_40cd9_42.0[2],
                    new_state_0_id_col145,
                ];

                *sub_component_inputs.memory_id_to_big[12] = new_state_0_id_col145;
                *lookup_data.memory_id_to_big_50 = [
                    M31_1662111297,
                    new_state_0_id_col145,
                    ((triple_xor_32_output_limb_0_col126) - ((low_7_ms_bits_col142) * (M31_512))),
                    ((low_7_ms_bits_col142) + ((high_2_ls_bits_tmp_40cd9_166) * (M31_128))),
                    ((high_14_ms_bits_col143) - ((high_5_ms_bits_col144) * (M31_512))),
                    high_5_ms_bits_col144,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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

                // Verify U 32.

                let low_7_ms_bits_tmp_40cd9_170 =
                    ((create_blake_output_output_tmp_40cd9_163[1].low()) >> (UInt16_9));
                let low_7_ms_bits_col146 = low_7_ms_bits_tmp_40cd9_170.as_m31();
                *row[146] = low_7_ms_bits_col146;
                let high_14_ms_bits_tmp_40cd9_171 =
                    ((create_blake_output_output_tmp_40cd9_163[1].high()) >> (UInt16_2));
                let high_14_ms_bits_col147 = high_14_ms_bits_tmp_40cd9_171.as_m31();
                *row[147] = high_14_ms_bits_col147;
                let high_2_ls_bits_tmp_40cd9_172 =
                    ((triple_xor_32_output_limb_1_col129) - ((high_14_ms_bits_col147) * (M31_4)));
                let high_5_ms_bits_tmp_40cd9_173 = ((high_14_ms_bits_tmp_40cd9_171) >> (UInt16_9));
                let high_5_ms_bits_col148 = high_5_ms_bits_tmp_40cd9_173.as_m31();
                *row[148] = high_5_ms_bits_col148;
                *sub_component_inputs.range_check_7_2_5[10] = [
                    low_7_ms_bits_col146,
                    high_2_ls_bits_tmp_40cd9_172,
                    high_5_ms_bits_col148,
                ];
                *lookup_data.range_check_7_2_5_51 = [
                    M31_371240602,
                    low_7_ms_bits_col146,
                    high_2_ls_bits_tmp_40cd9_172,
                    high_5_ms_bits_col148,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_40cd9_174 = memory_address_to_id_state
                    .deduce_output(((decode_blake_opcode_output_tmp_40cd9_42.0[2]) + (M31_1)));
                let new_state_1_id_col149 = memory_address_to_id_value_tmp_40cd9_174;
                *row[149] = new_state_1_id_col149;
                *sub_component_inputs.memory_address_to_id[13] =
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[2]) + (M31_1));
                *lookup_data.memory_address_to_id_52 = [
                    M31_1444891767,
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[2]) + (M31_1)),
                    new_state_1_id_col149,
                ];

                *sub_component_inputs.memory_id_to_big[13] = new_state_1_id_col149;
                *lookup_data.memory_id_to_big_53 = [
                    M31_1662111297,
                    new_state_1_id_col149,
                    ((triple_xor_32_output_limb_0_col128) - ((low_7_ms_bits_col146) * (M31_512))),
                    ((low_7_ms_bits_col146) + ((high_2_ls_bits_tmp_40cd9_172) * (M31_128))),
                    ((high_14_ms_bits_col147) - ((high_5_ms_bits_col148) * (M31_512))),
                    high_5_ms_bits_col148,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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

                // Verify U 32.

                let low_7_ms_bits_tmp_40cd9_176 =
                    ((create_blake_output_output_tmp_40cd9_163[2].low()) >> (UInt16_9));
                let low_7_ms_bits_col150 = low_7_ms_bits_tmp_40cd9_176.as_m31();
                *row[150] = low_7_ms_bits_col150;
                let high_14_ms_bits_tmp_40cd9_177 =
                    ((create_blake_output_output_tmp_40cd9_163[2].high()) >> (UInt16_2));
                let high_14_ms_bits_col151 = high_14_ms_bits_tmp_40cd9_177.as_m31();
                *row[151] = high_14_ms_bits_col151;
                let high_2_ls_bits_tmp_40cd9_178 =
                    ((triple_xor_32_output_limb_1_col131) - ((high_14_ms_bits_col151) * (M31_4)));
                let high_5_ms_bits_tmp_40cd9_179 = ((high_14_ms_bits_tmp_40cd9_177) >> (UInt16_9));
                let high_5_ms_bits_col152 = high_5_ms_bits_tmp_40cd9_179.as_m31();
                *row[152] = high_5_ms_bits_col152;
                *sub_component_inputs.range_check_7_2_5[11] = [
                    low_7_ms_bits_col150,
                    high_2_ls_bits_tmp_40cd9_178,
                    high_5_ms_bits_col152,
                ];
                *lookup_data.range_check_7_2_5_54 = [
                    M31_371240602,
                    low_7_ms_bits_col150,
                    high_2_ls_bits_tmp_40cd9_178,
                    high_5_ms_bits_col152,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_40cd9_180 = memory_address_to_id_state
                    .deduce_output(((decode_blake_opcode_output_tmp_40cd9_42.0[2]) + (M31_2)));
                let new_state_2_id_col153 = memory_address_to_id_value_tmp_40cd9_180;
                *row[153] = new_state_2_id_col153;
                *sub_component_inputs.memory_address_to_id[14] =
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[2]) + (M31_2));
                *lookup_data.memory_address_to_id_55 = [
                    M31_1444891767,
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[2]) + (M31_2)),
                    new_state_2_id_col153,
                ];

                *sub_component_inputs.memory_id_to_big[14] = new_state_2_id_col153;
                *lookup_data.memory_id_to_big_56 = [
                    M31_1662111297,
                    new_state_2_id_col153,
                    ((triple_xor_32_output_limb_0_col130) - ((low_7_ms_bits_col150) * (M31_512))),
                    ((low_7_ms_bits_col150) + ((high_2_ls_bits_tmp_40cd9_178) * (M31_128))),
                    ((high_14_ms_bits_col151) - ((high_5_ms_bits_col152) * (M31_512))),
                    high_5_ms_bits_col152,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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

                // Verify U 32.

                let low_7_ms_bits_tmp_40cd9_182 =
                    ((create_blake_output_output_tmp_40cd9_163[3].low()) >> (UInt16_9));
                let low_7_ms_bits_col154 = low_7_ms_bits_tmp_40cd9_182.as_m31();
                *row[154] = low_7_ms_bits_col154;
                let high_14_ms_bits_tmp_40cd9_183 =
                    ((create_blake_output_output_tmp_40cd9_163[3].high()) >> (UInt16_2));
                let high_14_ms_bits_col155 = high_14_ms_bits_tmp_40cd9_183.as_m31();
                *row[155] = high_14_ms_bits_col155;
                let high_2_ls_bits_tmp_40cd9_184 =
                    ((triple_xor_32_output_limb_1_col133) - ((high_14_ms_bits_col155) * (M31_4)));
                let high_5_ms_bits_tmp_40cd9_185 = ((high_14_ms_bits_tmp_40cd9_183) >> (UInt16_9));
                let high_5_ms_bits_col156 = high_5_ms_bits_tmp_40cd9_185.as_m31();
                *row[156] = high_5_ms_bits_col156;
                *sub_component_inputs.range_check_7_2_5[12] = [
                    low_7_ms_bits_col154,
                    high_2_ls_bits_tmp_40cd9_184,
                    high_5_ms_bits_col156,
                ];
                *lookup_data.range_check_7_2_5_57 = [
                    M31_371240602,
                    low_7_ms_bits_col154,
                    high_2_ls_bits_tmp_40cd9_184,
                    high_5_ms_bits_col156,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_40cd9_186 = memory_address_to_id_state
                    .deduce_output(((decode_blake_opcode_output_tmp_40cd9_42.0[2]) + (M31_3)));
                let new_state_3_id_col157 = memory_address_to_id_value_tmp_40cd9_186;
                *row[157] = new_state_3_id_col157;
                *sub_component_inputs.memory_address_to_id[15] =
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[2]) + (M31_3));
                *lookup_data.memory_address_to_id_58 = [
                    M31_1444891767,
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[2]) + (M31_3)),
                    new_state_3_id_col157,
                ];

                *sub_component_inputs.memory_id_to_big[15] = new_state_3_id_col157;
                *lookup_data.memory_id_to_big_59 = [
                    M31_1662111297,
                    new_state_3_id_col157,
                    ((triple_xor_32_output_limb_0_col132) - ((low_7_ms_bits_col154) * (M31_512))),
                    ((low_7_ms_bits_col154) + ((high_2_ls_bits_tmp_40cd9_184) * (M31_128))),
                    ((high_14_ms_bits_col155) - ((high_5_ms_bits_col156) * (M31_512))),
                    high_5_ms_bits_col156,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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

                // Verify U 32.

                let low_7_ms_bits_tmp_40cd9_188 =
                    ((create_blake_output_output_tmp_40cd9_163[4].low()) >> (UInt16_9));
                let low_7_ms_bits_col158 = low_7_ms_bits_tmp_40cd9_188.as_m31();
                *row[158] = low_7_ms_bits_col158;
                let high_14_ms_bits_tmp_40cd9_189 =
                    ((create_blake_output_output_tmp_40cd9_163[4].high()) >> (UInt16_2));
                let high_14_ms_bits_col159 = high_14_ms_bits_tmp_40cd9_189.as_m31();
                *row[159] = high_14_ms_bits_col159;
                let high_2_ls_bits_tmp_40cd9_190 =
                    ((triple_xor_32_output_limb_1_col135) - ((high_14_ms_bits_col159) * (M31_4)));
                let high_5_ms_bits_tmp_40cd9_191 = ((high_14_ms_bits_tmp_40cd9_189) >> (UInt16_9));
                let high_5_ms_bits_col160 = high_5_ms_bits_tmp_40cd9_191.as_m31();
                *row[160] = high_5_ms_bits_col160;
                *sub_component_inputs.range_check_7_2_5[13] = [
                    low_7_ms_bits_col158,
                    high_2_ls_bits_tmp_40cd9_190,
                    high_5_ms_bits_col160,
                ];
                *lookup_data.range_check_7_2_5_60 = [
                    M31_371240602,
                    low_7_ms_bits_col158,
                    high_2_ls_bits_tmp_40cd9_190,
                    high_5_ms_bits_col160,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_40cd9_192 = memory_address_to_id_state
                    .deduce_output(((decode_blake_opcode_output_tmp_40cd9_42.0[2]) + (M31_4)));
                let new_state_4_id_col161 = memory_address_to_id_value_tmp_40cd9_192;
                *row[161] = new_state_4_id_col161;
                *sub_component_inputs.memory_address_to_id[16] =
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[2]) + (M31_4));
                *lookup_data.memory_address_to_id_61 = [
                    M31_1444891767,
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[2]) + (M31_4)),
                    new_state_4_id_col161,
                ];

                *sub_component_inputs.memory_id_to_big[16] = new_state_4_id_col161;
                *lookup_data.memory_id_to_big_62 = [
                    M31_1662111297,
                    new_state_4_id_col161,
                    ((triple_xor_32_output_limb_0_col134) - ((low_7_ms_bits_col158) * (M31_512))),
                    ((low_7_ms_bits_col158) + ((high_2_ls_bits_tmp_40cd9_190) * (M31_128))),
                    ((high_14_ms_bits_col159) - ((high_5_ms_bits_col160) * (M31_512))),
                    high_5_ms_bits_col160,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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

                // Verify U 32.

                let low_7_ms_bits_tmp_40cd9_194 =
                    ((create_blake_output_output_tmp_40cd9_163[5].low()) >> (UInt16_9));
                let low_7_ms_bits_col162 = low_7_ms_bits_tmp_40cd9_194.as_m31();
                *row[162] = low_7_ms_bits_col162;
                let high_14_ms_bits_tmp_40cd9_195 =
                    ((create_blake_output_output_tmp_40cd9_163[5].high()) >> (UInt16_2));
                let high_14_ms_bits_col163 = high_14_ms_bits_tmp_40cd9_195.as_m31();
                *row[163] = high_14_ms_bits_col163;
                let high_2_ls_bits_tmp_40cd9_196 =
                    ((triple_xor_32_output_limb_1_col137) - ((high_14_ms_bits_col163) * (M31_4)));
                let high_5_ms_bits_tmp_40cd9_197 = ((high_14_ms_bits_tmp_40cd9_195) >> (UInt16_9));
                let high_5_ms_bits_col164 = high_5_ms_bits_tmp_40cd9_197.as_m31();
                *row[164] = high_5_ms_bits_col164;
                *sub_component_inputs.range_check_7_2_5[14] = [
                    low_7_ms_bits_col162,
                    high_2_ls_bits_tmp_40cd9_196,
                    high_5_ms_bits_col164,
                ];
                *lookup_data.range_check_7_2_5_63 = [
                    M31_371240602,
                    low_7_ms_bits_col162,
                    high_2_ls_bits_tmp_40cd9_196,
                    high_5_ms_bits_col164,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_40cd9_198 = memory_address_to_id_state
                    .deduce_output(((decode_blake_opcode_output_tmp_40cd9_42.0[2]) + (M31_5)));
                let new_state_5_id_col165 = memory_address_to_id_value_tmp_40cd9_198;
                *row[165] = new_state_5_id_col165;
                *sub_component_inputs.memory_address_to_id[17] =
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[2]) + (M31_5));
                *lookup_data.memory_address_to_id_64 = [
                    M31_1444891767,
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[2]) + (M31_5)),
                    new_state_5_id_col165,
                ];

                *sub_component_inputs.memory_id_to_big[17] = new_state_5_id_col165;
                *lookup_data.memory_id_to_big_65 = [
                    M31_1662111297,
                    new_state_5_id_col165,
                    ((triple_xor_32_output_limb_0_col136) - ((low_7_ms_bits_col162) * (M31_512))),
                    ((low_7_ms_bits_col162) + ((high_2_ls_bits_tmp_40cd9_196) * (M31_128))),
                    ((high_14_ms_bits_col163) - ((high_5_ms_bits_col164) * (M31_512))),
                    high_5_ms_bits_col164,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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

                // Verify U 32.

                let low_7_ms_bits_tmp_40cd9_200 =
                    ((create_blake_output_output_tmp_40cd9_163[6].low()) >> (UInt16_9));
                let low_7_ms_bits_col166 = low_7_ms_bits_tmp_40cd9_200.as_m31();
                *row[166] = low_7_ms_bits_col166;
                let high_14_ms_bits_tmp_40cd9_201 =
                    ((create_blake_output_output_tmp_40cd9_163[6].high()) >> (UInt16_2));
                let high_14_ms_bits_col167 = high_14_ms_bits_tmp_40cd9_201.as_m31();
                *row[167] = high_14_ms_bits_col167;
                let high_2_ls_bits_tmp_40cd9_202 =
                    ((triple_xor_32_output_limb_1_col139) - ((high_14_ms_bits_col167) * (M31_4)));
                let high_5_ms_bits_tmp_40cd9_203 = ((high_14_ms_bits_tmp_40cd9_201) >> (UInt16_9));
                let high_5_ms_bits_col168 = high_5_ms_bits_tmp_40cd9_203.as_m31();
                *row[168] = high_5_ms_bits_col168;
                *sub_component_inputs.range_check_7_2_5[15] = [
                    low_7_ms_bits_col166,
                    high_2_ls_bits_tmp_40cd9_202,
                    high_5_ms_bits_col168,
                ];
                *lookup_data.range_check_7_2_5_66 = [
                    M31_371240602,
                    low_7_ms_bits_col166,
                    high_2_ls_bits_tmp_40cd9_202,
                    high_5_ms_bits_col168,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_40cd9_204 = memory_address_to_id_state
                    .deduce_output(((decode_blake_opcode_output_tmp_40cd9_42.0[2]) + (M31_6)));
                let new_state_6_id_col169 = memory_address_to_id_value_tmp_40cd9_204;
                *row[169] = new_state_6_id_col169;
                *sub_component_inputs.memory_address_to_id[18] =
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[2]) + (M31_6));
                *lookup_data.memory_address_to_id_67 = [
                    M31_1444891767,
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[2]) + (M31_6)),
                    new_state_6_id_col169,
                ];

                *sub_component_inputs.memory_id_to_big[18] = new_state_6_id_col169;
                *lookup_data.memory_id_to_big_68 = [
                    M31_1662111297,
                    new_state_6_id_col169,
                    ((triple_xor_32_output_limb_0_col138) - ((low_7_ms_bits_col166) * (M31_512))),
                    ((low_7_ms_bits_col166) + ((high_2_ls_bits_tmp_40cd9_202) * (M31_128))),
                    ((high_14_ms_bits_col167) - ((high_5_ms_bits_col168) * (M31_512))),
                    high_5_ms_bits_col168,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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

                // Verify U 32.

                let low_7_ms_bits_tmp_40cd9_206 =
                    ((create_blake_output_output_tmp_40cd9_163[7].low()) >> (UInt16_9));
                let low_7_ms_bits_col170 = low_7_ms_bits_tmp_40cd9_206.as_m31();
                *row[170] = low_7_ms_bits_col170;
                let high_14_ms_bits_tmp_40cd9_207 =
                    ((create_blake_output_output_tmp_40cd9_163[7].high()) >> (UInt16_2));
                let high_14_ms_bits_col171 = high_14_ms_bits_tmp_40cd9_207.as_m31();
                *row[171] = high_14_ms_bits_col171;
                let high_2_ls_bits_tmp_40cd9_208 =
                    ((triple_xor_32_output_limb_1_col141) - ((high_14_ms_bits_col171) * (M31_4)));
                let high_5_ms_bits_tmp_40cd9_209 = ((high_14_ms_bits_tmp_40cd9_207) >> (UInt16_9));
                let high_5_ms_bits_col172 = high_5_ms_bits_tmp_40cd9_209.as_m31();
                *row[172] = high_5_ms_bits_col172;
                *sub_component_inputs.range_check_7_2_5[16] = [
                    low_7_ms_bits_col170,
                    high_2_ls_bits_tmp_40cd9_208,
                    high_5_ms_bits_col172,
                ];
                *lookup_data.range_check_7_2_5_69 = [
                    M31_371240602,
                    low_7_ms_bits_col170,
                    high_2_ls_bits_tmp_40cd9_208,
                    high_5_ms_bits_col172,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_40cd9_210 = memory_address_to_id_state
                    .deduce_output(((decode_blake_opcode_output_tmp_40cd9_42.0[2]) + (M31_7)));
                let new_state_7_id_col173 = memory_address_to_id_value_tmp_40cd9_210;
                *row[173] = new_state_7_id_col173;
                *sub_component_inputs.memory_address_to_id[19] =
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[2]) + (M31_7));
                *lookup_data.memory_address_to_id_70 = [
                    M31_1444891767,
                    ((decode_blake_opcode_output_tmp_40cd9_42.0[2]) + (M31_7)),
                    new_state_7_id_col173,
                ];

                *sub_component_inputs.memory_id_to_big[19] = new_state_7_id_col173;
                *lookup_data.memory_id_to_big_71 = [
                    M31_1662111297,
                    new_state_7_id_col173,
                    ((triple_xor_32_output_limb_0_col140) - ((low_7_ms_bits_col170) * (M31_512))),
                    ((low_7_ms_bits_col170) + ((high_2_ls_bits_tmp_40cd9_208) * (M31_128))),
                    ((high_14_ms_bits_col171) - ((high_5_ms_bits_col172) * (M31_512))),
                    high_5_ms_bits_col172,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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

                *lookup_data.opcodes_72 =
                    [M31_428564188, input_pc_col1, input_ap_col2, input_fp_col3];
                *lookup_data.opcodes_73 = [
                    M31_428564188,
                    ((input_pc_col1) + (M31_1)),
                    ((input_ap_col2) + (ap_update_add_1_col10)),
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
    verify_bitwise_xor_8_34: Vec<[PackedM31; 4]>,
    verify_bitwise_xor_8_35: Vec<[PackedM31; 4]>,
    verify_bitwise_xor_8_36: Vec<[PackedM31; 4]>,
    verify_bitwise_xor_8_37: Vec<[PackedM31; 4]>,
    blake_round_38: Vec<[PackedM31; 36]>,
    blake_round_39: Vec<[PackedM31; 36]>,
    triple_xor_32_40: Vec<[PackedM31; 9]>,
    triple_xor_32_41: Vec<[PackedM31; 9]>,
    triple_xor_32_42: Vec<[PackedM31; 9]>,
    triple_xor_32_43: Vec<[PackedM31; 9]>,
    triple_xor_32_44: Vec<[PackedM31; 9]>,
    triple_xor_32_45: Vec<[PackedM31; 9]>,
    triple_xor_32_46: Vec<[PackedM31; 9]>,
    triple_xor_32_47: Vec<[PackedM31; 9]>,
    range_check_7_2_5_48: Vec<[PackedM31; 4]>,
    memory_address_to_id_49: Vec<[PackedM31; 3]>,
    memory_id_to_big_50: Vec<[PackedM31; 30]>,
    range_check_7_2_5_51: Vec<[PackedM31; 4]>,
    memory_address_to_id_52: Vec<[PackedM31; 3]>,
    memory_id_to_big_53: Vec<[PackedM31; 30]>,
    range_check_7_2_5_54: Vec<[PackedM31; 4]>,
    memory_address_to_id_55: Vec<[PackedM31; 3]>,
    memory_id_to_big_56: Vec<[PackedM31; 30]>,
    range_check_7_2_5_57: Vec<[PackedM31; 4]>,
    memory_address_to_id_58: Vec<[PackedM31; 3]>,
    memory_id_to_big_59: Vec<[PackedM31; 30]>,
    range_check_7_2_5_60: Vec<[PackedM31; 4]>,
    memory_address_to_id_61: Vec<[PackedM31; 3]>,
    memory_id_to_big_62: Vec<[PackedM31; 30]>,
    range_check_7_2_5_63: Vec<[PackedM31; 4]>,
    memory_address_to_id_64: Vec<[PackedM31; 3]>,
    memory_id_to_big_65: Vec<[PackedM31; 30]>,
    range_check_7_2_5_66: Vec<[PackedM31; 4]>,
    memory_address_to_id_67: Vec<[PackedM31; 3]>,
    memory_id_to_big_68: Vec<[PackedM31; 30]>,
    range_check_7_2_5_69: Vec<[PackedM31; 4]>,
    memory_address_to_id_70: Vec<[PackedM31; 3]>,
    memory_id_to_big_71: Vec<[PackedM31; 30]>,
    opcodes_72: Vec<[PackedM31; 4]>,
    opcodes_73: Vec<[PackedM31; 4]>,
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
            &self.lookup_data.verify_bitwise_xor_8_34,
            &self.lookup_data.verify_bitwise_xor_8_35,
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
            &self.lookup_data.verify_bitwise_xor_8_36,
            &self.lookup_data.verify_bitwise_xor_8_37,
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
            &self.lookup_data.blake_round_38,
            &self.lookup_data.blake_round_39,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 * *mult1 - denom1 * *mult0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.triple_xor_32_40,
            &self.lookup_data.triple_xor_32_41,
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
            &self.lookup_data.triple_xor_32_42,
            &self.lookup_data.triple_xor_32_43,
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
            &self.lookup_data.triple_xor_32_44,
            &self.lookup_data.triple_xor_32_45,
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
            &self.lookup_data.triple_xor_32_46,
            &self.lookup_data.triple_xor_32_47,
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
            &self.lookup_data.range_check_7_2_5_48,
            &self.lookup_data.memory_address_to_id_49,
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
            &self.lookup_data.memory_id_to_big_50,
            &self.lookup_data.range_check_7_2_5_51,
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
            &self.lookup_data.memory_address_to_id_52,
            &self.lookup_data.memory_id_to_big_53,
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
            &self.lookup_data.range_check_7_2_5_54,
            &self.lookup_data.memory_address_to_id_55,
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
            &self.lookup_data.memory_id_to_big_56,
            &self.lookup_data.range_check_7_2_5_57,
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
            &self.lookup_data.memory_address_to_id_58,
            &self.lookup_data.memory_id_to_big_59,
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
            &self.lookup_data.range_check_7_2_5_60,
            &self.lookup_data.memory_address_to_id_61,
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
            &self.lookup_data.memory_id_to_big_62,
            &self.lookup_data.range_check_7_2_5_63,
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
            &self.lookup_data.memory_address_to_id_64,
            &self.lookup_data.memory_id_to_big_65,
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
            &self.lookup_data.range_check_7_2_5_66,
            &self.lookup_data.memory_address_to_id_67,
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
            &self.lookup_data.memory_id_to_big_68,
            &self.lookup_data.range_check_7_2_5_69,
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
            &self.lookup_data.memory_address_to_id_70,
            &self.lookup_data.memory_id_to_big_71,
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
            &self.lookup_data.opcodes_72,
            &self.lookup_data.opcodes_73,
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
