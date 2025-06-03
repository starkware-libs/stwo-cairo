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
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        blake_round_state: &mut blake_round::ClaimGenerator,
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
        range_check_7_2_5_state: &range_check_7_2_5::ClaimGenerator,
        triple_xor_32_state: &mut triple_xor_32::ClaimGenerator,
        verify_bitwise_xor_8_state: &verify_bitwise_xor_8::ClaimGenerator,
        verify_instruction_state: &verify_instruction::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator) {
        let n_rows = self.inputs.len();
        assert_ne!(n_rows, 0);
        let size = std::cmp::max(n_rows.next_power_of_two(), N_LANES);
        let log_size = size.ilog2();
        self.inputs.resize(size, *self.inputs.first().unwrap());
        let packed_inputs = pack_values(&self.inputs);

        let (trace, lookup_data, sub_component_inputs) = write_trace_simd(
            packed_inputs,
            n_rows,
            blake_round_state,
            memory_address_to_id_state,
            memory_id_to_big_state,
            range_check_7_2_5_state,
            triple_xor_32_state,
            verify_bitwise_xor_8_state,
            verify_instruction_state,
        );
        sub_component_inputs
            .verify_instruction
            .iter()
            .for_each(|inputs| {
                verify_instruction_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .memory_address_to_id
            .iter()
            .for_each(|inputs| {
                memory_address_to_id_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .memory_id_to_big
            .iter()
            .for_each(|inputs| {
                memory_id_to_big_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_7_2_5
            .iter()
            .for_each(|inputs| {
                range_check_7_2_5_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .verify_bitwise_xor_8
            .iter()
            .for_each(|inputs| {
                verify_bitwise_xor_8_state.add_packed_inputs(inputs);
            });
        sub_component_inputs.blake_round.iter().for_each(|inputs| {
            blake_round_state.add_packed_inputs(inputs);
        });
        sub_component_inputs
            .triple_xor_32
            .iter()
            .for_each(|inputs| {
                triple_xor_32_state.add_packed_inputs(inputs);
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
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct SubComponentInputs {
    verify_instruction: [Vec<verify_instruction::PackedInputType>; 1],
    memory_address_to_id: [Vec<PackedRelocatable>; 20],
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
    blake_round_state: &blake_round::ClaimGenerator,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
    range_check_7_2_5_state: &range_check_7_2_5::ClaimGenerator,
    triple_xor_32_state: &triple_xor_32::ClaimGenerator,
    verify_bitwise_xor_8_state: &verify_bitwise_xor_8::ClaimGenerator,
    verify_instruction_state: &verify_instruction::ClaimGenerator,
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
    let M31_127 = PackedM31::broadcast(M31::from(127));
    let M31_128 = PackedM31::broadcast(M31::from(128));
    let M31_14 = PackedM31::broadcast(M31::from(14));
    let M31_15470 = PackedM31::broadcast(M31::from(15470));
    let M31_16 = PackedM31::broadcast(M31::from(16));
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
    let M31_39685 = PackedM31::broadcast(M31::from(39685));
    let M31_4 = PackedM31::broadcast(M31::from(4));
    let M31_42319 = PackedM31::broadcast(M31::from(42319));
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
            |(
                row_index,
                (mut row, lookup_data, sub_component_inputs, blake_compress_opcode_input),
            )| {
                let seq = seq.packed_at(row_index);
                let input_pc_col0 = blake_compress_opcode_input.pc;
                *row[0] = input_pc_col0;
                let input_ap_col1 = blake_compress_opcode_input.ap;
                *row[1] = input_ap_col1;
                let input_fp_col2 = blake_compress_opcode_input.fp;
                *row[2] = input_fp_col2;

                // Decode Blake Opcode.

                // Decode Instruction.

                let memory_address_to_id_value_tmp_53f39_0 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_pc_m31(input_pc_col0));
                let memory_id_to_big_value_tmp_53f39_1 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_53f39_0);
                let offset0_tmp_53f39_2 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_53f39_1.get_m31(0)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_53f39_1.get_m31(1),
                        )) & (UInt16_127))
                            << (UInt16_9)));
                let offset0_col3 = offset0_tmp_53f39_2.as_m31();
                *row[3] = offset0_col3;
                let offset1_tmp_53f39_3 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_53f39_1.get_m31(1)))
                        >> (UInt16_7))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_53f39_1.get_m31(2),
                        )) << (UInt16_2)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_53f39_1.get_m31(3),
                        )) & (UInt16_31))
                            << (UInt16_11)));
                let offset1_col4 = offset1_tmp_53f39_3.as_m31();
                *row[4] = offset1_col4;
                let offset2_tmp_53f39_4 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_53f39_1.get_m31(3)))
                        >> (UInt16_5))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_53f39_1.get_m31(4),
                        )) << (UInt16_4)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_53f39_1.get_m31(5),
                        )) & (UInt16_7))
                            << (UInt16_13)));
                let offset2_col5 = offset2_tmp_53f39_4.as_m31();
                *row[5] = offset2_col5;
                let dst_base_fp_tmp_53f39_5 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_53f39_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_53f39_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_0))
                        & (UInt16_1));
                let dst_base_fp_col6 = dst_base_fp_tmp_53f39_5.as_m31();
                *row[6] = dst_base_fp_col6;
                let op0_base_fp_tmp_53f39_6 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_53f39_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_53f39_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_1))
                        & (UInt16_1));
                let op0_base_fp_col7 = op0_base_fp_tmp_53f39_6.as_m31();
                *row[7] = op0_base_fp_col7;
                let op1_base_fp_tmp_53f39_7 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_53f39_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_53f39_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_3))
                        & (UInt16_1));
                let op1_base_fp_col8 = op1_base_fp_tmp_53f39_7.as_m31();
                *row[8] = op1_base_fp_col8;
                let op1_base_ap_tmp_53f39_8 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_53f39_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_53f39_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_4))
                        & (UInt16_1));
                let op1_base_ap_col9 = op1_base_ap_tmp_53f39_8.as_m31();
                *row[9] = op1_base_ap_col9;
                let ap_update_add_1_tmp_53f39_9 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_53f39_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_53f39_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_11))
                        & (UInt16_1));
                let ap_update_add_1_col10 = ap_update_add_1_tmp_53f39_9.as_m31();
                *row[10] = ap_update_add_1_col10;
                let opcode_extension_col11 = memory_id_to_big_value_tmp_53f39_1.get_m31(7);
                *row[11] = opcode_extension_col11;
                *sub_component_inputs.verify_instruction[0] = (
                    input_pc_col0,
                    [offset0_col3, offset1_col4, offset2_col5],
                    [
                        (((((dst_base_fp_col6) * (M31_8)) + ((op0_base_fp_col7) * (M31_16)))
                            + ((op1_base_fp_col8) * (M31_64)))
                            + ((op1_base_ap_col9) * (M31_128))),
                        ((ap_update_add_1_col10) * (M31_32)),
                    ],
                    opcode_extension_col11,
                );
                *lookup_data.verify_instruction_0 = [
                    input_pc_col0,
                    offset0_col3,
                    offset1_col4,
                    offset2_col5,
                    (((((dst_base_fp_col6) * (M31_8)) + ((op0_base_fp_col7) * (M31_16)))
                        + ((op1_base_fp_col8) * (M31_64)))
                        + ((op1_base_ap_col9) * (M31_128))),
                    ((ap_update_add_1_col10) * (M31_32)),
                    opcode_extension_col11,
                ];
                let decode_instruction_64420_output_tmp_53f39_10 = (
                    [
                        ((offset0_col3) - (M31_32768)),
                        ((offset1_col4) - (M31_32768)),
                        ((offset2_col5) - (M31_32768)),
                    ],
                    [
                        dst_base_fp_col6,
                        op0_base_fp_col7,
                        M31_0,
                        op1_base_fp_col8,
                        op1_base_ap_col9,
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

                let mem0_base_col12 = (((op0_base_fp_col7) * (input_fp_col2))
                    + (((M31_1) - (op0_base_fp_col7)) * (input_ap_col1)));
                *row[12] = mem0_base_col12;

                // Read Positive Num Bits 27.

                let memory_address_to_id_value_tmp_53f39_11 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((mem0_base_col12) + (decode_instruction_64420_output_tmp_53f39_10.0[1])),
                    ));
                let memory_id_to_big_value_tmp_53f39_12 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_53f39_11);
                let op0_id_col13 = memory_address_to_id_value_tmp_53f39_11;
                *row[13] = op0_id_col13;
                *sub_component_inputs.memory_address_to_id[0] = PackedRelocatable::from_ap_m31(
                    ((mem0_base_col12) + (decode_instruction_64420_output_tmp_53f39_10.0[1])),
                );
                *lookup_data.memory_address_to_id_0 = [
                    M31_1,
                    ((mem0_base_col12) + (decode_instruction_64420_output_tmp_53f39_10.0[1])),
                    op0_id_col13,
                ];
                let op0_limb_0_col14 = memory_id_to_big_value_tmp_53f39_12.get_m31(0);
                *row[14] = op0_limb_0_col14;
                let op0_limb_1_col15 = memory_id_to_big_value_tmp_53f39_12.get_m31(1);
                *row[15] = op0_limb_1_col15;
                let op0_limb_2_col16 = memory_id_to_big_value_tmp_53f39_12.get_m31(2);
                *row[16] = op0_limb_2_col16;
                *sub_component_inputs.memory_id_to_big[0] = op0_id_col13;
                *lookup_data.memory_id_to_big_0 = [
                    op0_id_col13,
                    op0_limb_0_col14,
                    op0_limb_1_col15,
                    op0_limb_2_col16,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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
                let read_positive_num_bits_27_output_tmp_53f39_13 = (
                    PackedFelt252::from_limbs([
                        op0_limb_0_col14,
                        op0_limb_1_col15,
                        op0_limb_2_col16,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                    ]),
                    op0_id_col13,
                );

                let mem1_base_col17 = (((op1_base_fp_col8) * (input_fp_col2))
                    + ((op1_base_ap_col9) * (input_ap_col1)));
                *row[17] = mem1_base_col17;

                // Read Positive Num Bits 27.

                let memory_address_to_id_value_tmp_53f39_14 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((mem1_base_col17) + (decode_instruction_64420_output_tmp_53f39_10.0[2])),
                    ));
                let memory_id_to_big_value_tmp_53f39_15 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_53f39_14);
                let op1_id_col18 = memory_address_to_id_value_tmp_53f39_14;
                *row[18] = op1_id_col18;
                *sub_component_inputs.memory_address_to_id[1] = PackedRelocatable::from_ap_m31(
                    ((mem1_base_col17) + (decode_instruction_64420_output_tmp_53f39_10.0[2])),
                );
                *lookup_data.memory_address_to_id_1 = [
                    M31_1,
                    ((mem1_base_col17) + (decode_instruction_64420_output_tmp_53f39_10.0[2])),
                    op1_id_col18,
                ];
                let op1_limb_0_col19 = memory_id_to_big_value_tmp_53f39_15.get_m31(0);
                *row[19] = op1_limb_0_col19;
                let op1_limb_1_col20 = memory_id_to_big_value_tmp_53f39_15.get_m31(1);
                *row[20] = op1_limb_1_col20;
                let op1_limb_2_col21 = memory_id_to_big_value_tmp_53f39_15.get_m31(2);
                *row[21] = op1_limb_2_col21;
                *sub_component_inputs.memory_id_to_big[1] = op1_id_col18;
                *lookup_data.memory_id_to_big_1 = [
                    op1_id_col18,
                    op1_limb_0_col19,
                    op1_limb_1_col20,
                    op1_limb_2_col21,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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
                let read_positive_num_bits_27_output_tmp_53f39_16 = (
                    PackedFelt252::from_limbs([
                        op1_limb_0_col19,
                        op1_limb_1_col20,
                        op1_limb_2_col21,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                    ]),
                    op1_id_col18,
                );

                // Read Positive Num Bits 27.

                let memory_address_to_id_value_tmp_53f39_17 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(input_ap_col1));
                let memory_id_to_big_value_tmp_53f39_18 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_53f39_17);
                let ap_id_col22 = memory_address_to_id_value_tmp_53f39_17;
                *row[22] = ap_id_col22;
                *sub_component_inputs.memory_address_to_id[2] =
                    PackedRelocatable::from_ap_m31(input_ap_col1);
                *lookup_data.memory_address_to_id_2 = [M31_1, input_ap_col1, ap_id_col22];
                let ap_limb_0_col23 = memory_id_to_big_value_tmp_53f39_18.get_m31(0);
                *row[23] = ap_limb_0_col23;
                let ap_limb_1_col24 = memory_id_to_big_value_tmp_53f39_18.get_m31(1);
                *row[24] = ap_limb_1_col24;
                let ap_limb_2_col25 = memory_id_to_big_value_tmp_53f39_18.get_m31(2);
                *row[25] = ap_limb_2_col25;
                *sub_component_inputs.memory_id_to_big[2] = ap_id_col22;
                *lookup_data.memory_id_to_big_2 = [
                    ap_id_col22,
                    ap_limb_0_col23,
                    ap_limb_1_col24,
                    ap_limb_2_col25,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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
                let read_positive_num_bits_27_output_tmp_53f39_19 = (
                    PackedFelt252::from_limbs([
                        ap_limb_0_col23,
                        ap_limb_1_col24,
                        ap_limb_2_col25,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                    ]),
                    ap_id_col22,
                );

                let mem_dst_base_col26 = (((dst_base_fp_col6) * (input_fp_col2))
                    + (((M31_1) - (dst_base_fp_col6)) * (input_ap_col1)));
                *row[26] = mem_dst_base_col26;

                // Read Blake Word.

                let memory_address_to_id_value_tmp_53f39_20 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((mem_dst_base_col26)
                            + (decode_instruction_64420_output_tmp_53f39_10.0[0])),
                    ));
                let memory_id_to_big_value_tmp_53f39_21 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_53f39_20);
                let tmp_53f39_22 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_53f39_21.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col27 = ((((memory_id_to_big_value_tmp_53f39_21.get_m31(1))
                    - ((tmp_53f39_22.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_53f39_21.get_m31(0)));
                *row[27] = low_16_bits_col27;
                let high_16_bits_col28 = ((((memory_id_to_big_value_tmp_53f39_21.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_53f39_21.get_m31(2)) * (M31_4)))
                    + (tmp_53f39_22.as_m31()));
                *row[28] = high_16_bits_col28;
                let expected_word_tmp_53f39_23 =
                    PackedUInt32::from_limbs([low_16_bits_col27, high_16_bits_col28]);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_53f39_24 = ((expected_word_tmp_53f39_23.low()) >> (UInt16_9));
                let low_7_ms_bits_col29 = low_7_ms_bits_tmp_53f39_24.as_m31();
                *row[29] = low_7_ms_bits_col29;
                let high_14_ms_bits_tmp_53f39_25 =
                    ((expected_word_tmp_53f39_23.high()) >> (UInt16_2));
                let high_14_ms_bits_col30 = high_14_ms_bits_tmp_53f39_25.as_m31();
                *row[30] = high_14_ms_bits_col30;
                let high_5_ms_bits_tmp_53f39_26 = ((high_14_ms_bits_tmp_53f39_25) >> (UInt16_9));
                let high_5_ms_bits_col31 = high_5_ms_bits_tmp_53f39_26.as_m31();
                *row[31] = high_5_ms_bits_col31;
                *sub_component_inputs.range_check_7_2_5[0] = [
                    low_7_ms_bits_col29,
                    ((high_16_bits_col28) - ((high_14_ms_bits_col30) * (M31_4))),
                    high_5_ms_bits_col31,
                ];
                *lookup_data.range_check_7_2_5_0 = [
                    low_7_ms_bits_col29,
                    ((high_16_bits_col28) - ((high_14_ms_bits_col30) * (M31_4))),
                    high_5_ms_bits_col31,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_53f39_27 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((mem_dst_base_col26)
                            + (decode_instruction_64420_output_tmp_53f39_10.0[0])),
                    ));
                let dst_id_col32 = memory_address_to_id_value_tmp_53f39_27;
                *row[32] = dst_id_col32;
                *sub_component_inputs.memory_address_to_id[3] = PackedRelocatable::from_ap_m31(
                    ((mem_dst_base_col26) + (decode_instruction_64420_output_tmp_53f39_10.0[0])),
                );
                *lookup_data.memory_address_to_id_3 = [
                    M31_1,
                    ((mem_dst_base_col26) + (decode_instruction_64420_output_tmp_53f39_10.0[0])),
                    dst_id_col32,
                ];
                *sub_component_inputs.memory_id_to_big[3] = dst_id_col32;
                *lookup_data.memory_id_to_big_3 = [
                    dst_id_col32,
                    ((low_16_bits_col27) - ((low_7_ms_bits_col29) * (M31_512))),
                    ((low_7_ms_bits_col29)
                        + (((high_16_bits_col28) - ((high_14_ms_bits_col30) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col30) - ((high_5_ms_bits_col31) * (M31_512))),
                    high_5_ms_bits_col31,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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

                let read_blake_word_output_tmp_53f39_28 = expected_word_tmp_53f39_23;

                let decode_blake_opcode_output_tmp_53f39_29 = (
                    [
                        (((op0_limb_0_col14) + ((op0_limb_1_col15) * (M31_512)))
                            + ((op0_limb_2_col16) * (M31_262144))),
                        (((op1_limb_0_col19) + ((op1_limb_1_col20) * (M31_512)))
                            + ((op1_limb_2_col21) * (M31_262144))),
                        (((ap_limb_0_col23) + ((ap_limb_1_col24) * (M31_512)))
                            + ((ap_limb_2_col25) * (M31_262144))),
                    ],
                    read_blake_word_output_tmp_53f39_28,
                    [
                        PackedBool::from_m31(ap_update_add_1_col10),
                        PackedBool::from_m31(((opcode_extension_col11) - (M31_1))),
                    ],
                );

                // Create Blake Round Input.

                // Read Blake Word.

                let memory_address_to_id_value_tmp_53f39_30 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        decode_blake_opcode_output_tmp_53f39_29.0[0],
                    ));
                let memory_id_to_big_value_tmp_53f39_31 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_53f39_30);
                let tmp_53f39_32 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_53f39_31.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col33 = ((((memory_id_to_big_value_tmp_53f39_31.get_m31(1))
                    - ((tmp_53f39_32.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_53f39_31.get_m31(0)));
                *row[33] = low_16_bits_col33;
                let high_16_bits_col34 = ((((memory_id_to_big_value_tmp_53f39_31.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_53f39_31.get_m31(2)) * (M31_4)))
                    + (tmp_53f39_32.as_m31()));
                *row[34] = high_16_bits_col34;
                let expected_word_tmp_53f39_33 =
                    PackedUInt32::from_limbs([low_16_bits_col33, high_16_bits_col34]);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_53f39_34 = ((expected_word_tmp_53f39_33.low()) >> (UInt16_9));
                let low_7_ms_bits_col35 = low_7_ms_bits_tmp_53f39_34.as_m31();
                *row[35] = low_7_ms_bits_col35;
                let high_14_ms_bits_tmp_53f39_35 =
                    ((expected_word_tmp_53f39_33.high()) >> (UInt16_2));
                let high_14_ms_bits_col36 = high_14_ms_bits_tmp_53f39_35.as_m31();
                *row[36] = high_14_ms_bits_col36;
                let high_5_ms_bits_tmp_53f39_36 = ((high_14_ms_bits_tmp_53f39_35) >> (UInt16_9));
                let high_5_ms_bits_col37 = high_5_ms_bits_tmp_53f39_36.as_m31();
                *row[37] = high_5_ms_bits_col37;
                *sub_component_inputs.range_check_7_2_5[1] = [
                    low_7_ms_bits_col35,
                    ((high_16_bits_col34) - ((high_14_ms_bits_col36) * (M31_4))),
                    high_5_ms_bits_col37,
                ];
                *lookup_data.range_check_7_2_5_1 = [
                    low_7_ms_bits_col35,
                    ((high_16_bits_col34) - ((high_14_ms_bits_col36) * (M31_4))),
                    high_5_ms_bits_col37,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_53f39_37 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        decode_blake_opcode_output_tmp_53f39_29.0[0],
                    ));
                let state_0_id_col38 = memory_address_to_id_value_tmp_53f39_37;
                *row[38] = state_0_id_col38;
                *sub_component_inputs.memory_address_to_id[4] =
                    PackedRelocatable::from_ap_m31(decode_blake_opcode_output_tmp_53f39_29.0[0]);
                *lookup_data.memory_address_to_id_4 = [
                    M31_1,
                    decode_blake_opcode_output_tmp_53f39_29.0[0],
                    state_0_id_col38,
                ];
                *sub_component_inputs.memory_id_to_big[4] = state_0_id_col38;
                *lookup_data.memory_id_to_big_4 = [
                    state_0_id_col38,
                    ((low_16_bits_col33) - ((low_7_ms_bits_col35) * (M31_512))),
                    ((low_7_ms_bits_col35)
                        + (((high_16_bits_col34) - ((high_14_ms_bits_col36) * (M31_4)))
                            * (M31_128))),
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

                let read_blake_word_output_tmp_53f39_38 = expected_word_tmp_53f39_33;

                // Read Blake Word.

                let memory_address_to_id_value_tmp_53f39_39 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_1)),
                    ));
                let memory_id_to_big_value_tmp_53f39_40 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_53f39_39);
                let tmp_53f39_41 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_53f39_40.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col39 = ((((memory_id_to_big_value_tmp_53f39_40.get_m31(1))
                    - ((tmp_53f39_41.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_53f39_40.get_m31(0)));
                *row[39] = low_16_bits_col39;
                let high_16_bits_col40 = ((((memory_id_to_big_value_tmp_53f39_40.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_53f39_40.get_m31(2)) * (M31_4)))
                    + (tmp_53f39_41.as_m31()));
                *row[40] = high_16_bits_col40;
                let expected_word_tmp_53f39_42 =
                    PackedUInt32::from_limbs([low_16_bits_col39, high_16_bits_col40]);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_53f39_43 = ((expected_word_tmp_53f39_42.low()) >> (UInt16_9));
                let low_7_ms_bits_col41 = low_7_ms_bits_tmp_53f39_43.as_m31();
                *row[41] = low_7_ms_bits_col41;
                let high_14_ms_bits_tmp_53f39_44 =
                    ((expected_word_tmp_53f39_42.high()) >> (UInt16_2));
                let high_14_ms_bits_col42 = high_14_ms_bits_tmp_53f39_44.as_m31();
                *row[42] = high_14_ms_bits_col42;
                let high_5_ms_bits_tmp_53f39_45 = ((high_14_ms_bits_tmp_53f39_44) >> (UInt16_9));
                let high_5_ms_bits_col43 = high_5_ms_bits_tmp_53f39_45.as_m31();
                *row[43] = high_5_ms_bits_col43;
                *sub_component_inputs.range_check_7_2_5[2] = [
                    low_7_ms_bits_col41,
                    ((high_16_bits_col40) - ((high_14_ms_bits_col42) * (M31_4))),
                    high_5_ms_bits_col43,
                ];
                *lookup_data.range_check_7_2_5_2 = [
                    low_7_ms_bits_col41,
                    ((high_16_bits_col40) - ((high_14_ms_bits_col42) * (M31_4))),
                    high_5_ms_bits_col43,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_53f39_46 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_1)),
                    ));
                let state_1_id_col44 = memory_address_to_id_value_tmp_53f39_46;
                *row[44] = state_1_id_col44;
                *sub_component_inputs.memory_address_to_id[5] = PackedRelocatable::from_ap_m31(
                    ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_1)),
                );
                *lookup_data.memory_address_to_id_5 = [
                    M31_1,
                    ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_1)),
                    state_1_id_col44,
                ];
                *sub_component_inputs.memory_id_to_big[5] = state_1_id_col44;
                *lookup_data.memory_id_to_big_5 = [
                    state_1_id_col44,
                    ((low_16_bits_col39) - ((low_7_ms_bits_col41) * (M31_512))),
                    ((low_7_ms_bits_col41)
                        + (((high_16_bits_col40) - ((high_14_ms_bits_col42) * (M31_4)))
                            * (M31_128))),
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

                let read_blake_word_output_tmp_53f39_47 = expected_word_tmp_53f39_42;

                // Read Blake Word.

                let memory_address_to_id_value_tmp_53f39_48 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_2)),
                    ));
                let memory_id_to_big_value_tmp_53f39_49 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_53f39_48);
                let tmp_53f39_50 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_53f39_49.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col45 = ((((memory_id_to_big_value_tmp_53f39_49.get_m31(1))
                    - ((tmp_53f39_50.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_53f39_49.get_m31(0)));
                *row[45] = low_16_bits_col45;
                let high_16_bits_col46 = ((((memory_id_to_big_value_tmp_53f39_49.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_53f39_49.get_m31(2)) * (M31_4)))
                    + (tmp_53f39_50.as_m31()));
                *row[46] = high_16_bits_col46;
                let expected_word_tmp_53f39_51 =
                    PackedUInt32::from_limbs([low_16_bits_col45, high_16_bits_col46]);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_53f39_52 = ((expected_word_tmp_53f39_51.low()) >> (UInt16_9));
                let low_7_ms_bits_col47 = low_7_ms_bits_tmp_53f39_52.as_m31();
                *row[47] = low_7_ms_bits_col47;
                let high_14_ms_bits_tmp_53f39_53 =
                    ((expected_word_tmp_53f39_51.high()) >> (UInt16_2));
                let high_14_ms_bits_col48 = high_14_ms_bits_tmp_53f39_53.as_m31();
                *row[48] = high_14_ms_bits_col48;
                let high_5_ms_bits_tmp_53f39_54 = ((high_14_ms_bits_tmp_53f39_53) >> (UInt16_9));
                let high_5_ms_bits_col49 = high_5_ms_bits_tmp_53f39_54.as_m31();
                *row[49] = high_5_ms_bits_col49;
                *sub_component_inputs.range_check_7_2_5[3] = [
                    low_7_ms_bits_col47,
                    ((high_16_bits_col46) - ((high_14_ms_bits_col48) * (M31_4))),
                    high_5_ms_bits_col49,
                ];
                *lookup_data.range_check_7_2_5_3 = [
                    low_7_ms_bits_col47,
                    ((high_16_bits_col46) - ((high_14_ms_bits_col48) * (M31_4))),
                    high_5_ms_bits_col49,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_53f39_55 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_2)),
                    ));
                let state_2_id_col50 = memory_address_to_id_value_tmp_53f39_55;
                *row[50] = state_2_id_col50;
                *sub_component_inputs.memory_address_to_id[6] = PackedRelocatable::from_ap_m31(
                    ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_2)),
                );
                *lookup_data.memory_address_to_id_6 = [
                    M31_1,
                    ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_2)),
                    state_2_id_col50,
                ];
                *sub_component_inputs.memory_id_to_big[6] = state_2_id_col50;
                *lookup_data.memory_id_to_big_6 = [
                    state_2_id_col50,
                    ((low_16_bits_col45) - ((low_7_ms_bits_col47) * (M31_512))),
                    ((low_7_ms_bits_col47)
                        + (((high_16_bits_col46) - ((high_14_ms_bits_col48) * (M31_4)))
                            * (M31_128))),
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

                let read_blake_word_output_tmp_53f39_56 = expected_word_tmp_53f39_51;

                // Read Blake Word.

                let memory_address_to_id_value_tmp_53f39_57 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_3)),
                    ));
                let memory_id_to_big_value_tmp_53f39_58 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_53f39_57);
                let tmp_53f39_59 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_53f39_58.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col51 = ((((memory_id_to_big_value_tmp_53f39_58.get_m31(1))
                    - ((tmp_53f39_59.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_53f39_58.get_m31(0)));
                *row[51] = low_16_bits_col51;
                let high_16_bits_col52 = ((((memory_id_to_big_value_tmp_53f39_58.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_53f39_58.get_m31(2)) * (M31_4)))
                    + (tmp_53f39_59.as_m31()));
                *row[52] = high_16_bits_col52;
                let expected_word_tmp_53f39_60 =
                    PackedUInt32::from_limbs([low_16_bits_col51, high_16_bits_col52]);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_53f39_61 = ((expected_word_tmp_53f39_60.low()) >> (UInt16_9));
                let low_7_ms_bits_col53 = low_7_ms_bits_tmp_53f39_61.as_m31();
                *row[53] = low_7_ms_bits_col53;
                let high_14_ms_bits_tmp_53f39_62 =
                    ((expected_word_tmp_53f39_60.high()) >> (UInt16_2));
                let high_14_ms_bits_col54 = high_14_ms_bits_tmp_53f39_62.as_m31();
                *row[54] = high_14_ms_bits_col54;
                let high_5_ms_bits_tmp_53f39_63 = ((high_14_ms_bits_tmp_53f39_62) >> (UInt16_9));
                let high_5_ms_bits_col55 = high_5_ms_bits_tmp_53f39_63.as_m31();
                *row[55] = high_5_ms_bits_col55;
                *sub_component_inputs.range_check_7_2_5[4] = [
                    low_7_ms_bits_col53,
                    ((high_16_bits_col52) - ((high_14_ms_bits_col54) * (M31_4))),
                    high_5_ms_bits_col55,
                ];
                *lookup_data.range_check_7_2_5_4 = [
                    low_7_ms_bits_col53,
                    ((high_16_bits_col52) - ((high_14_ms_bits_col54) * (M31_4))),
                    high_5_ms_bits_col55,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_53f39_64 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_3)),
                    ));
                let state_3_id_col56 = memory_address_to_id_value_tmp_53f39_64;
                *row[56] = state_3_id_col56;
                *sub_component_inputs.memory_address_to_id[7] = PackedRelocatable::from_ap_m31(
                    ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_3)),
                );
                *lookup_data.memory_address_to_id_7 = [
                    M31_1,
                    ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_3)),
                    state_3_id_col56,
                ];
                *sub_component_inputs.memory_id_to_big[7] = state_3_id_col56;
                *lookup_data.memory_id_to_big_7 = [
                    state_3_id_col56,
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

                let read_blake_word_output_tmp_53f39_65 = expected_word_tmp_53f39_60;

                // Read Blake Word.

                let memory_address_to_id_value_tmp_53f39_66 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_4)),
                    ));
                let memory_id_to_big_value_tmp_53f39_67 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_53f39_66);
                let tmp_53f39_68 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_53f39_67.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col57 = ((((memory_id_to_big_value_tmp_53f39_67.get_m31(1))
                    - ((tmp_53f39_68.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_53f39_67.get_m31(0)));
                *row[57] = low_16_bits_col57;
                let high_16_bits_col58 = ((((memory_id_to_big_value_tmp_53f39_67.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_53f39_67.get_m31(2)) * (M31_4)))
                    + (tmp_53f39_68.as_m31()));
                *row[58] = high_16_bits_col58;
                let expected_word_tmp_53f39_69 =
                    PackedUInt32::from_limbs([low_16_bits_col57, high_16_bits_col58]);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_53f39_70 = ((expected_word_tmp_53f39_69.low()) >> (UInt16_9));
                let low_7_ms_bits_col59 = low_7_ms_bits_tmp_53f39_70.as_m31();
                *row[59] = low_7_ms_bits_col59;
                let high_14_ms_bits_tmp_53f39_71 =
                    ((expected_word_tmp_53f39_69.high()) >> (UInt16_2));
                let high_14_ms_bits_col60 = high_14_ms_bits_tmp_53f39_71.as_m31();
                *row[60] = high_14_ms_bits_col60;
                let high_5_ms_bits_tmp_53f39_72 = ((high_14_ms_bits_tmp_53f39_71) >> (UInt16_9));
                let high_5_ms_bits_col61 = high_5_ms_bits_tmp_53f39_72.as_m31();
                *row[61] = high_5_ms_bits_col61;
                *sub_component_inputs.range_check_7_2_5[5] = [
                    low_7_ms_bits_col59,
                    ((high_16_bits_col58) - ((high_14_ms_bits_col60) * (M31_4))),
                    high_5_ms_bits_col61,
                ];
                *lookup_data.range_check_7_2_5_5 = [
                    low_7_ms_bits_col59,
                    ((high_16_bits_col58) - ((high_14_ms_bits_col60) * (M31_4))),
                    high_5_ms_bits_col61,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_53f39_73 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_4)),
                    ));
                let state_4_id_col62 = memory_address_to_id_value_tmp_53f39_73;
                *row[62] = state_4_id_col62;
                *sub_component_inputs.memory_address_to_id[8] = PackedRelocatable::from_ap_m31(
                    ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_4)),
                );
                *lookup_data.memory_address_to_id_8 = [
                    M31_1,
                    ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_4)),
                    state_4_id_col62,
                ];
                *sub_component_inputs.memory_id_to_big[8] = state_4_id_col62;
                *lookup_data.memory_id_to_big_8 = [
                    state_4_id_col62,
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

                let read_blake_word_output_tmp_53f39_74 = expected_word_tmp_53f39_69;

                // Read Blake Word.

                let memory_address_to_id_value_tmp_53f39_75 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_5)),
                    ));
                let memory_id_to_big_value_tmp_53f39_76 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_53f39_75);
                let tmp_53f39_77 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_53f39_76.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col63 = ((((memory_id_to_big_value_tmp_53f39_76.get_m31(1))
                    - ((tmp_53f39_77.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_53f39_76.get_m31(0)));
                *row[63] = low_16_bits_col63;
                let high_16_bits_col64 = ((((memory_id_to_big_value_tmp_53f39_76.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_53f39_76.get_m31(2)) * (M31_4)))
                    + (tmp_53f39_77.as_m31()));
                *row[64] = high_16_bits_col64;
                let expected_word_tmp_53f39_78 =
                    PackedUInt32::from_limbs([low_16_bits_col63, high_16_bits_col64]);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_53f39_79 = ((expected_word_tmp_53f39_78.low()) >> (UInt16_9));
                let low_7_ms_bits_col65 = low_7_ms_bits_tmp_53f39_79.as_m31();
                *row[65] = low_7_ms_bits_col65;
                let high_14_ms_bits_tmp_53f39_80 =
                    ((expected_word_tmp_53f39_78.high()) >> (UInt16_2));
                let high_14_ms_bits_col66 = high_14_ms_bits_tmp_53f39_80.as_m31();
                *row[66] = high_14_ms_bits_col66;
                let high_5_ms_bits_tmp_53f39_81 = ((high_14_ms_bits_tmp_53f39_80) >> (UInt16_9));
                let high_5_ms_bits_col67 = high_5_ms_bits_tmp_53f39_81.as_m31();
                *row[67] = high_5_ms_bits_col67;
                *sub_component_inputs.range_check_7_2_5[6] = [
                    low_7_ms_bits_col65,
                    ((high_16_bits_col64) - ((high_14_ms_bits_col66) * (M31_4))),
                    high_5_ms_bits_col67,
                ];
                *lookup_data.range_check_7_2_5_6 = [
                    low_7_ms_bits_col65,
                    ((high_16_bits_col64) - ((high_14_ms_bits_col66) * (M31_4))),
                    high_5_ms_bits_col67,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_53f39_82 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_5)),
                    ));
                let state_5_id_col68 = memory_address_to_id_value_tmp_53f39_82;
                *row[68] = state_5_id_col68;
                *sub_component_inputs.memory_address_to_id[9] = PackedRelocatable::from_ap_m31(
                    ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_5)),
                );
                *lookup_data.memory_address_to_id_9 = [
                    M31_1,
                    ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_5)),
                    state_5_id_col68,
                ];
                *sub_component_inputs.memory_id_to_big[9] = state_5_id_col68;
                *lookup_data.memory_id_to_big_9 = [
                    state_5_id_col68,
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

                let read_blake_word_output_tmp_53f39_83 = expected_word_tmp_53f39_78;

                // Read Blake Word.

                let memory_address_to_id_value_tmp_53f39_84 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_6)),
                    ));
                let memory_id_to_big_value_tmp_53f39_85 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_53f39_84);
                let tmp_53f39_86 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_53f39_85.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col69 = ((((memory_id_to_big_value_tmp_53f39_85.get_m31(1))
                    - ((tmp_53f39_86.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_53f39_85.get_m31(0)));
                *row[69] = low_16_bits_col69;
                let high_16_bits_col70 = ((((memory_id_to_big_value_tmp_53f39_85.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_53f39_85.get_m31(2)) * (M31_4)))
                    + (tmp_53f39_86.as_m31()));
                *row[70] = high_16_bits_col70;
                let expected_word_tmp_53f39_87 =
                    PackedUInt32::from_limbs([low_16_bits_col69, high_16_bits_col70]);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_53f39_88 = ((expected_word_tmp_53f39_87.low()) >> (UInt16_9));
                let low_7_ms_bits_col71 = low_7_ms_bits_tmp_53f39_88.as_m31();
                *row[71] = low_7_ms_bits_col71;
                let high_14_ms_bits_tmp_53f39_89 =
                    ((expected_word_tmp_53f39_87.high()) >> (UInt16_2));
                let high_14_ms_bits_col72 = high_14_ms_bits_tmp_53f39_89.as_m31();
                *row[72] = high_14_ms_bits_col72;
                let high_5_ms_bits_tmp_53f39_90 = ((high_14_ms_bits_tmp_53f39_89) >> (UInt16_9));
                let high_5_ms_bits_col73 = high_5_ms_bits_tmp_53f39_90.as_m31();
                *row[73] = high_5_ms_bits_col73;
                *sub_component_inputs.range_check_7_2_5[7] = [
                    low_7_ms_bits_col71,
                    ((high_16_bits_col70) - ((high_14_ms_bits_col72) * (M31_4))),
                    high_5_ms_bits_col73,
                ];
                *lookup_data.range_check_7_2_5_7 = [
                    low_7_ms_bits_col71,
                    ((high_16_bits_col70) - ((high_14_ms_bits_col72) * (M31_4))),
                    high_5_ms_bits_col73,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_53f39_91 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_6)),
                    ));
                let state_6_id_col74 = memory_address_to_id_value_tmp_53f39_91;
                *row[74] = state_6_id_col74;
                *sub_component_inputs.memory_address_to_id[10] = PackedRelocatable::from_ap_m31(
                    ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_6)),
                );
                *lookup_data.memory_address_to_id_10 = [
                    M31_1,
                    ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_6)),
                    state_6_id_col74,
                ];
                *sub_component_inputs.memory_id_to_big[10] = state_6_id_col74;
                *lookup_data.memory_id_to_big_10 = [
                    state_6_id_col74,
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

                let read_blake_word_output_tmp_53f39_92 = expected_word_tmp_53f39_87;

                // Read Blake Word.

                let memory_address_to_id_value_tmp_53f39_93 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_7)),
                    ));
                let memory_id_to_big_value_tmp_53f39_94 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_53f39_93);
                let tmp_53f39_95 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_53f39_94.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col75 = ((((memory_id_to_big_value_tmp_53f39_94.get_m31(1))
                    - ((tmp_53f39_95.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_53f39_94.get_m31(0)));
                *row[75] = low_16_bits_col75;
                let high_16_bits_col76 = ((((memory_id_to_big_value_tmp_53f39_94.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_53f39_94.get_m31(2)) * (M31_4)))
                    + (tmp_53f39_95.as_m31()));
                *row[76] = high_16_bits_col76;
                let expected_word_tmp_53f39_96 =
                    PackedUInt32::from_limbs([low_16_bits_col75, high_16_bits_col76]);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_53f39_97 = ((expected_word_tmp_53f39_96.low()) >> (UInt16_9));
                let low_7_ms_bits_col77 = low_7_ms_bits_tmp_53f39_97.as_m31();
                *row[77] = low_7_ms_bits_col77;
                let high_14_ms_bits_tmp_53f39_98 =
                    ((expected_word_tmp_53f39_96.high()) >> (UInt16_2));
                let high_14_ms_bits_col78 = high_14_ms_bits_tmp_53f39_98.as_m31();
                *row[78] = high_14_ms_bits_col78;
                let high_5_ms_bits_tmp_53f39_99 = ((high_14_ms_bits_tmp_53f39_98) >> (UInt16_9));
                let high_5_ms_bits_col79 = high_5_ms_bits_tmp_53f39_99.as_m31();
                *row[79] = high_5_ms_bits_col79;
                *sub_component_inputs.range_check_7_2_5[8] = [
                    low_7_ms_bits_col77,
                    ((high_16_bits_col76) - ((high_14_ms_bits_col78) * (M31_4))),
                    high_5_ms_bits_col79,
                ];
                *lookup_data.range_check_7_2_5_8 = [
                    low_7_ms_bits_col77,
                    ((high_16_bits_col76) - ((high_14_ms_bits_col78) * (M31_4))),
                    high_5_ms_bits_col79,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_53f39_100 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_7)),
                    ));
                let state_7_id_col80 = memory_address_to_id_value_tmp_53f39_100;
                *row[80] = state_7_id_col80;
                *sub_component_inputs.memory_address_to_id[11] = PackedRelocatable::from_ap_m31(
                    ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_7)),
                );
                *lookup_data.memory_address_to_id_11 = [
                    M31_1,
                    ((decode_blake_opcode_output_tmp_53f39_29.0[0]) + (M31_7)),
                    state_7_id_col80,
                ];
                *sub_component_inputs.memory_id_to_big[11] = state_7_id_col80;
                *lookup_data.memory_id_to_big_11 = [
                    state_7_id_col80,
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

                let read_blake_word_output_tmp_53f39_101 = expected_word_tmp_53f39_96;

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_53f39_102 =
                    ((decode_blake_opcode_output_tmp_53f39_29.1.low()) >> (UInt16_8));
                let ms_8_bits_col81 = ms_8_bits_tmp_53f39_102.as_m31();
                *row[81] = ms_8_bits_col81;
                let split_16_low_part_size_8_output_tmp_53f39_103 = [
                    ((low_16_bits_col27) - ((ms_8_bits_col81) * (M31_256))),
                    ms_8_bits_col81,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_53f39_104 =
                    ((decode_blake_opcode_output_tmp_53f39_29.1.high()) >> (UInt16_8));
                let ms_8_bits_col82 = ms_8_bits_tmp_53f39_104.as_m31();
                *row[82] = ms_8_bits_col82;
                let split_16_low_part_size_8_output_tmp_53f39_105 = [
                    ((high_16_bits_col28) - ((ms_8_bits_col82) * (M31_256))),
                    ms_8_bits_col82,
                ];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_53f39_106 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_53f39_103[0]))
                        ^ (UInt16_127));
                let xor_col83 = xor_tmp_53f39_106.as_m31();
                *row[83] = xor_col83;
                *sub_component_inputs.verify_bitwise_xor_8[0] = [
                    split_16_low_part_size_8_output_tmp_53f39_103[0],
                    M31_127,
                    xor_col83,
                ];
                *lookup_data.verify_bitwise_xor_8_0 = [
                    split_16_low_part_size_8_output_tmp_53f39_103[0],
                    M31_127,
                    xor_col83,
                ];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_53f39_108 = ((PackedUInt16::from_m31(ms_8_bits_col81)) ^ (UInt16_82));
                let xor_col84 = xor_tmp_53f39_108.as_m31();
                *row[84] = xor_col84;
                *sub_component_inputs.verify_bitwise_xor_8[1] =
                    [ms_8_bits_col81, M31_82, xor_col84];
                *lookup_data.verify_bitwise_xor_8_1 = [ms_8_bits_col81, M31_82, xor_col84];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_53f39_110 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_53f39_105[0]))
                        ^ (UInt16_14));
                let xor_col85 = xor_tmp_53f39_110.as_m31();
                *row[85] = xor_col85;
                *sub_component_inputs.verify_bitwise_xor_8[2] = [
                    split_16_low_part_size_8_output_tmp_53f39_105[0],
                    M31_14,
                    xor_col85,
                ];
                *lookup_data.verify_bitwise_xor_8_2 = [
                    split_16_low_part_size_8_output_tmp_53f39_105[0],
                    M31_14,
                    xor_col85,
                ];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_53f39_112 = ((PackedUInt16::from_m31(ms_8_bits_col82)) ^ (UInt16_81));
                let xor_col86 = xor_tmp_53f39_112.as_m31();
                *row[86] = xor_col86;
                *sub_component_inputs.verify_bitwise_xor_8[3] =
                    [ms_8_bits_col82, M31_81, xor_col86];
                *lookup_data.verify_bitwise_xor_8_3 = [ms_8_bits_col82, M31_81, xor_col86];

                let create_blake_round_input_output_tmp_53f39_114 = [
                    read_blake_word_output_tmp_53f39_38,
                    read_blake_word_output_tmp_53f39_47,
                    read_blake_word_output_tmp_53f39_56,
                    read_blake_word_output_tmp_53f39_65,
                    read_blake_word_output_tmp_53f39_74,
                    read_blake_word_output_tmp_53f39_83,
                    read_blake_word_output_tmp_53f39_92,
                    read_blake_word_output_tmp_53f39_101,
                    UInt32_1779033703,
                    UInt32_3144134277,
                    UInt32_1013904242,
                    UInt32_2773480762,
                    PackedUInt32::from_limbs([
                        ((xor_col83) + ((xor_col84) * (M31_256))),
                        ((xor_col85) + ((xor_col86) * (M31_256))),
                    ]),
                    UInt32_2600822924,
                    PackedUInt32::from_limbs([
                        (((decode_blake_opcode_output_tmp_53f39_29.2[1].as_m31()) * (M31_9812))
                            + (((M31_1)
                                - (decode_blake_opcode_output_tmp_53f39_29.2[1].as_m31()))
                                * (M31_55723))),
                        (((decode_blake_opcode_output_tmp_53f39_29.2[1].as_m31()) * (M31_57468))
                            + (((M31_1)
                                - (decode_blake_opcode_output_tmp_53f39_29.2[1].as_m31()))
                                * (M31_8067))),
                    ]),
                    UInt32_1541459225,
                ];

                *lookup_data.blake_round_0 = [
                    seq,
                    M31_0,
                    low_16_bits_col33,
                    high_16_bits_col34,
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
                    M31_58983,
                    M31_27145,
                    M31_44677,
                    M31_47975,
                    M31_62322,
                    M31_15470,
                    M31_62778,
                    M31_42319,
                    create_blake_round_input_output_tmp_53f39_114[12]
                        .low()
                        .as_m31(),
                    create_blake_round_input_output_tmp_53f39_114[12]
                        .high()
                        .as_m31(),
                    M31_26764,
                    M31_39685,
                    create_blake_round_input_output_tmp_53f39_114[14]
                        .low()
                        .as_m31(),
                    create_blake_round_input_output_tmp_53f39_114[14]
                        .high()
                        .as_m31(),
                    M31_52505,
                    M31_23520,
                    decode_blake_opcode_output_tmp_53f39_29.0[1],
                ];
                *sub_component_inputs.blake_round[0] = (
                    seq,
                    M31_0,
                    (
                        [
                            create_blake_round_input_output_tmp_53f39_114[0],
                            create_blake_round_input_output_tmp_53f39_114[1],
                            create_blake_round_input_output_tmp_53f39_114[2],
                            create_blake_round_input_output_tmp_53f39_114[3],
                            create_blake_round_input_output_tmp_53f39_114[4],
                            create_blake_round_input_output_tmp_53f39_114[5],
                            create_blake_round_input_output_tmp_53f39_114[6],
                            create_blake_round_input_output_tmp_53f39_114[7],
                            UInt32_1779033703,
                            UInt32_3144134277,
                            UInt32_1013904242,
                            UInt32_2773480762,
                            create_blake_round_input_output_tmp_53f39_114[12],
                            UInt32_2600822924,
                            create_blake_round_input_output_tmp_53f39_114[14],
                            UInt32_1541459225,
                        ],
                        decode_blake_opcode_output_tmp_53f39_29.0[1],
                    ),
                );
                let blake_round_output_round_0_tmp_53f39_115 = blake_round_state.deduce_output((
                    seq,
                    M31_0,
                    (
                        [
                            create_blake_round_input_output_tmp_53f39_114[0],
                            create_blake_round_input_output_tmp_53f39_114[1],
                            create_blake_round_input_output_tmp_53f39_114[2],
                            create_blake_round_input_output_tmp_53f39_114[3],
                            create_blake_round_input_output_tmp_53f39_114[4],
                            create_blake_round_input_output_tmp_53f39_114[5],
                            create_blake_round_input_output_tmp_53f39_114[6],
                            create_blake_round_input_output_tmp_53f39_114[7],
                            UInt32_1779033703,
                            UInt32_3144134277,
                            UInt32_1013904242,
                            UInt32_2773480762,
                            create_blake_round_input_output_tmp_53f39_114[12],
                            UInt32_2600822924,
                            create_blake_round_input_output_tmp_53f39_114[14],
                            UInt32_1541459225,
                        ],
                        decode_blake_opcode_output_tmp_53f39_29.0[1],
                    ),
                ));
                *sub_component_inputs.blake_round[1] = (
                    seq,
                    M31_1,
                    (
                        [
                            blake_round_output_round_0_tmp_53f39_115.2 .0[0],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[1],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[2],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[3],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[4],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[5],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[6],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[7],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[8],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[9],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[10],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[11],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[12],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[13],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[14],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[15],
                        ],
                        blake_round_output_round_0_tmp_53f39_115.2 .1,
                    ),
                );
                let blake_round_output_round_1_tmp_53f39_116 = blake_round_state.deduce_output((
                    seq,
                    M31_1,
                    (
                        [
                            blake_round_output_round_0_tmp_53f39_115.2 .0[0],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[1],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[2],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[3],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[4],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[5],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[6],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[7],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[8],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[9],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[10],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[11],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[12],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[13],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[14],
                            blake_round_output_round_0_tmp_53f39_115.2 .0[15],
                        ],
                        blake_round_output_round_0_tmp_53f39_115.2 .1,
                    ),
                ));
                *sub_component_inputs.blake_round[2] = (
                    seq,
                    M31_2,
                    (
                        [
                            blake_round_output_round_1_tmp_53f39_116.2 .0[0],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[1],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[2],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[3],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[4],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[5],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[6],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[7],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[8],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[9],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[10],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[11],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[12],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[13],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[14],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[15],
                        ],
                        blake_round_output_round_1_tmp_53f39_116.2 .1,
                    ),
                );
                let blake_round_output_round_2_tmp_53f39_117 = blake_round_state.deduce_output((
                    seq,
                    M31_2,
                    (
                        [
                            blake_round_output_round_1_tmp_53f39_116.2 .0[0],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[1],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[2],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[3],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[4],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[5],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[6],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[7],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[8],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[9],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[10],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[11],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[12],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[13],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[14],
                            blake_round_output_round_1_tmp_53f39_116.2 .0[15],
                        ],
                        blake_round_output_round_1_tmp_53f39_116.2 .1,
                    ),
                ));
                *sub_component_inputs.blake_round[3] = (
                    seq,
                    M31_3,
                    (
                        [
                            blake_round_output_round_2_tmp_53f39_117.2 .0[0],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[1],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[2],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[3],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[4],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[5],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[6],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[7],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[8],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[9],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[10],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[11],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[12],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[13],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[14],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[15],
                        ],
                        blake_round_output_round_2_tmp_53f39_117.2 .1,
                    ),
                );
                let blake_round_output_round_3_tmp_53f39_118 = blake_round_state.deduce_output((
                    seq,
                    M31_3,
                    (
                        [
                            blake_round_output_round_2_tmp_53f39_117.2 .0[0],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[1],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[2],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[3],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[4],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[5],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[6],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[7],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[8],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[9],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[10],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[11],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[12],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[13],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[14],
                            blake_round_output_round_2_tmp_53f39_117.2 .0[15],
                        ],
                        blake_round_output_round_2_tmp_53f39_117.2 .1,
                    ),
                ));
                *sub_component_inputs.blake_round[4] = (
                    seq,
                    M31_4,
                    (
                        [
                            blake_round_output_round_3_tmp_53f39_118.2 .0[0],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[1],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[2],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[3],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[4],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[5],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[6],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[7],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[8],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[9],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[10],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[11],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[12],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[13],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[14],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[15],
                        ],
                        blake_round_output_round_3_tmp_53f39_118.2 .1,
                    ),
                );
                let blake_round_output_round_4_tmp_53f39_119 = blake_round_state.deduce_output((
                    seq,
                    M31_4,
                    (
                        [
                            blake_round_output_round_3_tmp_53f39_118.2 .0[0],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[1],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[2],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[3],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[4],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[5],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[6],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[7],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[8],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[9],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[10],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[11],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[12],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[13],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[14],
                            blake_round_output_round_3_tmp_53f39_118.2 .0[15],
                        ],
                        blake_round_output_round_3_tmp_53f39_118.2 .1,
                    ),
                ));
                *sub_component_inputs.blake_round[5] = (
                    seq,
                    M31_5,
                    (
                        [
                            blake_round_output_round_4_tmp_53f39_119.2 .0[0],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[1],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[2],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[3],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[4],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[5],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[6],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[7],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[8],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[9],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[10],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[11],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[12],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[13],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[14],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[15],
                        ],
                        blake_round_output_round_4_tmp_53f39_119.2 .1,
                    ),
                );
                let blake_round_output_round_5_tmp_53f39_120 = blake_round_state.deduce_output((
                    seq,
                    M31_5,
                    (
                        [
                            blake_round_output_round_4_tmp_53f39_119.2 .0[0],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[1],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[2],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[3],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[4],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[5],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[6],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[7],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[8],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[9],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[10],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[11],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[12],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[13],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[14],
                            blake_round_output_round_4_tmp_53f39_119.2 .0[15],
                        ],
                        blake_round_output_round_4_tmp_53f39_119.2 .1,
                    ),
                ));
                *sub_component_inputs.blake_round[6] = (
                    seq,
                    M31_6,
                    (
                        [
                            blake_round_output_round_5_tmp_53f39_120.2 .0[0],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[1],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[2],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[3],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[4],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[5],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[6],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[7],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[8],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[9],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[10],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[11],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[12],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[13],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[14],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[15],
                        ],
                        blake_round_output_round_5_tmp_53f39_120.2 .1,
                    ),
                );
                let blake_round_output_round_6_tmp_53f39_121 = blake_round_state.deduce_output((
                    seq,
                    M31_6,
                    (
                        [
                            blake_round_output_round_5_tmp_53f39_120.2 .0[0],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[1],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[2],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[3],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[4],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[5],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[6],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[7],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[8],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[9],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[10],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[11],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[12],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[13],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[14],
                            blake_round_output_round_5_tmp_53f39_120.2 .0[15],
                        ],
                        blake_round_output_round_5_tmp_53f39_120.2 .1,
                    ),
                ));
                *sub_component_inputs.blake_round[7] = (
                    seq,
                    M31_7,
                    (
                        [
                            blake_round_output_round_6_tmp_53f39_121.2 .0[0],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[1],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[2],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[3],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[4],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[5],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[6],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[7],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[8],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[9],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[10],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[11],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[12],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[13],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[14],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[15],
                        ],
                        blake_round_output_round_6_tmp_53f39_121.2 .1,
                    ),
                );
                let blake_round_output_round_7_tmp_53f39_122 = blake_round_state.deduce_output((
                    seq,
                    M31_7,
                    (
                        [
                            blake_round_output_round_6_tmp_53f39_121.2 .0[0],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[1],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[2],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[3],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[4],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[5],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[6],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[7],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[8],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[9],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[10],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[11],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[12],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[13],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[14],
                            blake_round_output_round_6_tmp_53f39_121.2 .0[15],
                        ],
                        blake_round_output_round_6_tmp_53f39_121.2 .1,
                    ),
                ));
                *sub_component_inputs.blake_round[8] = (
                    seq,
                    M31_8,
                    (
                        [
                            blake_round_output_round_7_tmp_53f39_122.2 .0[0],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[1],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[2],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[3],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[4],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[5],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[6],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[7],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[8],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[9],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[10],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[11],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[12],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[13],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[14],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[15],
                        ],
                        blake_round_output_round_7_tmp_53f39_122.2 .1,
                    ),
                );
                let blake_round_output_round_8_tmp_53f39_123 = blake_round_state.deduce_output((
                    seq,
                    M31_8,
                    (
                        [
                            blake_round_output_round_7_tmp_53f39_122.2 .0[0],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[1],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[2],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[3],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[4],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[5],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[6],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[7],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[8],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[9],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[10],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[11],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[12],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[13],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[14],
                            blake_round_output_round_7_tmp_53f39_122.2 .0[15],
                        ],
                        blake_round_output_round_7_tmp_53f39_122.2 .1,
                    ),
                ));
                *sub_component_inputs.blake_round[9] = (
                    seq,
                    M31_9,
                    (
                        [
                            blake_round_output_round_8_tmp_53f39_123.2 .0[0],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[1],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[2],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[3],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[4],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[5],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[6],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[7],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[8],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[9],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[10],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[11],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[12],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[13],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[14],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[15],
                        ],
                        blake_round_output_round_8_tmp_53f39_123.2 .1,
                    ),
                );
                let blake_round_output_round_9_tmp_53f39_124 = blake_round_state.deduce_output((
                    seq,
                    M31_9,
                    (
                        [
                            blake_round_output_round_8_tmp_53f39_123.2 .0[0],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[1],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[2],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[3],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[4],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[5],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[6],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[7],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[8],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[9],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[10],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[11],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[12],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[13],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[14],
                            blake_round_output_round_8_tmp_53f39_123.2 .0[15],
                        ],
                        blake_round_output_round_8_tmp_53f39_123.2 .1,
                    ),
                ));
                let blake_round_output_limb_0_col87 = blake_round_output_round_9_tmp_53f39_124.2 .0
                    [0]
                .low()
                .as_m31();
                *row[87] = blake_round_output_limb_0_col87;
                let blake_round_output_limb_1_col88 = blake_round_output_round_9_tmp_53f39_124.2 .0
                    [0]
                .high()
                .as_m31();
                *row[88] = blake_round_output_limb_1_col88;
                let blake_round_output_limb_2_col89 = blake_round_output_round_9_tmp_53f39_124.2 .0
                    [1]
                .low()
                .as_m31();
                *row[89] = blake_round_output_limb_2_col89;
                let blake_round_output_limb_3_col90 = blake_round_output_round_9_tmp_53f39_124.2 .0
                    [1]
                .high()
                .as_m31();
                *row[90] = blake_round_output_limb_3_col90;
                let blake_round_output_limb_4_col91 = blake_round_output_round_9_tmp_53f39_124.2 .0
                    [2]
                .low()
                .as_m31();
                *row[91] = blake_round_output_limb_4_col91;
                let blake_round_output_limb_5_col92 = blake_round_output_round_9_tmp_53f39_124.2 .0
                    [2]
                .high()
                .as_m31();
                *row[92] = blake_round_output_limb_5_col92;
                let blake_round_output_limb_6_col93 = blake_round_output_round_9_tmp_53f39_124.2 .0
                    [3]
                .low()
                .as_m31();
                *row[93] = blake_round_output_limb_6_col93;
                let blake_round_output_limb_7_col94 = blake_round_output_round_9_tmp_53f39_124.2 .0
                    [3]
                .high()
                .as_m31();
                *row[94] = blake_round_output_limb_7_col94;
                let blake_round_output_limb_8_col95 = blake_round_output_round_9_tmp_53f39_124.2 .0
                    [4]
                .low()
                .as_m31();
                *row[95] = blake_round_output_limb_8_col95;
                let blake_round_output_limb_9_col96 = blake_round_output_round_9_tmp_53f39_124.2 .0
                    [4]
                .high()
                .as_m31();
                *row[96] = blake_round_output_limb_9_col96;
                let blake_round_output_limb_10_col97 =
                    blake_round_output_round_9_tmp_53f39_124.2 .0[5]
                        .low()
                        .as_m31();
                *row[97] = blake_round_output_limb_10_col97;
                let blake_round_output_limb_11_col98 =
                    blake_round_output_round_9_tmp_53f39_124.2 .0[5]
                        .high()
                        .as_m31();
                *row[98] = blake_round_output_limb_11_col98;
                let blake_round_output_limb_12_col99 =
                    blake_round_output_round_9_tmp_53f39_124.2 .0[6]
                        .low()
                        .as_m31();
                *row[99] = blake_round_output_limb_12_col99;
                let blake_round_output_limb_13_col100 =
                    blake_round_output_round_9_tmp_53f39_124.2 .0[6]
                        .high()
                        .as_m31();
                *row[100] = blake_round_output_limb_13_col100;
                let blake_round_output_limb_14_col101 =
                    blake_round_output_round_9_tmp_53f39_124.2 .0[7]
                        .low()
                        .as_m31();
                *row[101] = blake_round_output_limb_14_col101;
                let blake_round_output_limb_15_col102 =
                    blake_round_output_round_9_tmp_53f39_124.2 .0[7]
                        .high()
                        .as_m31();
                *row[102] = blake_round_output_limb_15_col102;
                let blake_round_output_limb_16_col103 =
                    blake_round_output_round_9_tmp_53f39_124.2 .0[8]
                        .low()
                        .as_m31();
                *row[103] = blake_round_output_limb_16_col103;
                let blake_round_output_limb_17_col104 =
                    blake_round_output_round_9_tmp_53f39_124.2 .0[8]
                        .high()
                        .as_m31();
                *row[104] = blake_round_output_limb_17_col104;
                let blake_round_output_limb_18_col105 =
                    blake_round_output_round_9_tmp_53f39_124.2 .0[9]
                        .low()
                        .as_m31();
                *row[105] = blake_round_output_limb_18_col105;
                let blake_round_output_limb_19_col106 =
                    blake_round_output_round_9_tmp_53f39_124.2 .0[9]
                        .high()
                        .as_m31();
                *row[106] = blake_round_output_limb_19_col106;
                let blake_round_output_limb_20_col107 =
                    blake_round_output_round_9_tmp_53f39_124.2 .0[10]
                        .low()
                        .as_m31();
                *row[107] = blake_round_output_limb_20_col107;
                let blake_round_output_limb_21_col108 =
                    blake_round_output_round_9_tmp_53f39_124.2 .0[10]
                        .high()
                        .as_m31();
                *row[108] = blake_round_output_limb_21_col108;
                let blake_round_output_limb_22_col109 =
                    blake_round_output_round_9_tmp_53f39_124.2 .0[11]
                        .low()
                        .as_m31();
                *row[109] = blake_round_output_limb_22_col109;
                let blake_round_output_limb_23_col110 =
                    blake_round_output_round_9_tmp_53f39_124.2 .0[11]
                        .high()
                        .as_m31();
                *row[110] = blake_round_output_limb_23_col110;
                let blake_round_output_limb_24_col111 =
                    blake_round_output_round_9_tmp_53f39_124.2 .0[12]
                        .low()
                        .as_m31();
                *row[111] = blake_round_output_limb_24_col111;
                let blake_round_output_limb_25_col112 =
                    blake_round_output_round_9_tmp_53f39_124.2 .0[12]
                        .high()
                        .as_m31();
                *row[112] = blake_round_output_limb_25_col112;
                let blake_round_output_limb_26_col113 =
                    blake_round_output_round_9_tmp_53f39_124.2 .0[13]
                        .low()
                        .as_m31();
                *row[113] = blake_round_output_limb_26_col113;
                let blake_round_output_limb_27_col114 =
                    blake_round_output_round_9_tmp_53f39_124.2 .0[13]
                        .high()
                        .as_m31();
                *row[114] = blake_round_output_limb_27_col114;
                let blake_round_output_limb_28_col115 =
                    blake_round_output_round_9_tmp_53f39_124.2 .0[14]
                        .low()
                        .as_m31();
                *row[115] = blake_round_output_limb_28_col115;
                let blake_round_output_limb_29_col116 =
                    blake_round_output_round_9_tmp_53f39_124.2 .0[14]
                        .high()
                        .as_m31();
                *row[116] = blake_round_output_limb_29_col116;
                let blake_round_output_limb_30_col117 =
                    blake_round_output_round_9_tmp_53f39_124.2 .0[15]
                        .low()
                        .as_m31();
                *row[117] = blake_round_output_limb_30_col117;
                let blake_round_output_limb_31_col118 =
                    blake_round_output_round_9_tmp_53f39_124.2 .0[15]
                        .high()
                        .as_m31();
                *row[118] = blake_round_output_limb_31_col118;
                let blake_round_output_limb_32_col119 =
                    blake_round_output_round_9_tmp_53f39_124.2 .1;
                *row[119] = blake_round_output_limb_32_col119;
                *lookup_data.blake_round_1 = [
                    seq,
                    M31_10,
                    blake_round_output_limb_0_col87,
                    blake_round_output_limb_1_col88,
                    blake_round_output_limb_2_col89,
                    blake_round_output_limb_3_col90,
                    blake_round_output_limb_4_col91,
                    blake_round_output_limb_5_col92,
                    blake_round_output_limb_6_col93,
                    blake_round_output_limb_7_col94,
                    blake_round_output_limb_8_col95,
                    blake_round_output_limb_9_col96,
                    blake_round_output_limb_10_col97,
                    blake_round_output_limb_11_col98,
                    blake_round_output_limb_12_col99,
                    blake_round_output_limb_13_col100,
                    blake_round_output_limb_14_col101,
                    blake_round_output_limb_15_col102,
                    blake_round_output_limb_16_col103,
                    blake_round_output_limb_17_col104,
                    blake_round_output_limb_18_col105,
                    blake_round_output_limb_19_col106,
                    blake_round_output_limb_20_col107,
                    blake_round_output_limb_21_col108,
                    blake_round_output_limb_22_col109,
                    blake_round_output_limb_23_col110,
                    blake_round_output_limb_24_col111,
                    blake_round_output_limb_25_col112,
                    blake_round_output_limb_26_col113,
                    blake_round_output_limb_27_col114,
                    blake_round_output_limb_28_col115,
                    blake_round_output_limb_29_col116,
                    blake_round_output_limb_30_col117,
                    blake_round_output_limb_31_col118,
                    blake_round_output_limb_32_col119,
                ];

                // Create Blake Output.

                *sub_component_inputs.triple_xor_32[0] = [
                    blake_round_output_round_9_tmp_53f39_124.2 .0[0],
                    blake_round_output_round_9_tmp_53f39_124.2 .0[8],
                    create_blake_round_input_output_tmp_53f39_114[0],
                ];
                let triple_xor_32_output_tmp_53f39_125 = PackedTripleXor32::deduce_output([
                    blake_round_output_round_9_tmp_53f39_124.2 .0[0],
                    blake_round_output_round_9_tmp_53f39_124.2 .0[8],
                    create_blake_round_input_output_tmp_53f39_114[0],
                ]);
                let triple_xor_32_output_limb_0_col120 =
                    triple_xor_32_output_tmp_53f39_125.low().as_m31();
                *row[120] = triple_xor_32_output_limb_0_col120;
                let triple_xor_32_output_limb_1_col121 =
                    triple_xor_32_output_tmp_53f39_125.high().as_m31();
                *row[121] = triple_xor_32_output_limb_1_col121;
                *lookup_data.triple_xor_32_0 = [
                    blake_round_output_limb_0_col87,
                    blake_round_output_limb_1_col88,
                    blake_round_output_limb_16_col103,
                    blake_round_output_limb_17_col104,
                    low_16_bits_col33,
                    high_16_bits_col34,
                    triple_xor_32_output_limb_0_col120,
                    triple_xor_32_output_limb_1_col121,
                ];
                *sub_component_inputs.triple_xor_32[1] = [
                    blake_round_output_round_9_tmp_53f39_124.2 .0[1],
                    blake_round_output_round_9_tmp_53f39_124.2 .0[9],
                    create_blake_round_input_output_tmp_53f39_114[1],
                ];
                let triple_xor_32_output_tmp_53f39_126 = PackedTripleXor32::deduce_output([
                    blake_round_output_round_9_tmp_53f39_124.2 .0[1],
                    blake_round_output_round_9_tmp_53f39_124.2 .0[9],
                    create_blake_round_input_output_tmp_53f39_114[1],
                ]);
                let triple_xor_32_output_limb_0_col122 =
                    triple_xor_32_output_tmp_53f39_126.low().as_m31();
                *row[122] = triple_xor_32_output_limb_0_col122;
                let triple_xor_32_output_limb_1_col123 =
                    triple_xor_32_output_tmp_53f39_126.high().as_m31();
                *row[123] = triple_xor_32_output_limb_1_col123;
                *lookup_data.triple_xor_32_1 = [
                    blake_round_output_limb_2_col89,
                    blake_round_output_limb_3_col90,
                    blake_round_output_limb_18_col105,
                    blake_round_output_limb_19_col106,
                    low_16_bits_col39,
                    high_16_bits_col40,
                    triple_xor_32_output_limb_0_col122,
                    triple_xor_32_output_limb_1_col123,
                ];
                *sub_component_inputs.triple_xor_32[2] = [
                    blake_round_output_round_9_tmp_53f39_124.2 .0[2],
                    blake_round_output_round_9_tmp_53f39_124.2 .0[10],
                    create_blake_round_input_output_tmp_53f39_114[2],
                ];
                let triple_xor_32_output_tmp_53f39_127 = PackedTripleXor32::deduce_output([
                    blake_round_output_round_9_tmp_53f39_124.2 .0[2],
                    blake_round_output_round_9_tmp_53f39_124.2 .0[10],
                    create_blake_round_input_output_tmp_53f39_114[2],
                ]);
                let triple_xor_32_output_limb_0_col124 =
                    triple_xor_32_output_tmp_53f39_127.low().as_m31();
                *row[124] = triple_xor_32_output_limb_0_col124;
                let triple_xor_32_output_limb_1_col125 =
                    triple_xor_32_output_tmp_53f39_127.high().as_m31();
                *row[125] = triple_xor_32_output_limb_1_col125;
                *lookup_data.triple_xor_32_2 = [
                    blake_round_output_limb_4_col91,
                    blake_round_output_limb_5_col92,
                    blake_round_output_limb_20_col107,
                    blake_round_output_limb_21_col108,
                    low_16_bits_col45,
                    high_16_bits_col46,
                    triple_xor_32_output_limb_0_col124,
                    triple_xor_32_output_limb_1_col125,
                ];
                *sub_component_inputs.triple_xor_32[3] = [
                    blake_round_output_round_9_tmp_53f39_124.2 .0[3],
                    blake_round_output_round_9_tmp_53f39_124.2 .0[11],
                    create_blake_round_input_output_tmp_53f39_114[3],
                ];
                let triple_xor_32_output_tmp_53f39_128 = PackedTripleXor32::deduce_output([
                    blake_round_output_round_9_tmp_53f39_124.2 .0[3],
                    blake_round_output_round_9_tmp_53f39_124.2 .0[11],
                    create_blake_round_input_output_tmp_53f39_114[3],
                ]);
                let triple_xor_32_output_limb_0_col126 =
                    triple_xor_32_output_tmp_53f39_128.low().as_m31();
                *row[126] = triple_xor_32_output_limb_0_col126;
                let triple_xor_32_output_limb_1_col127 =
                    triple_xor_32_output_tmp_53f39_128.high().as_m31();
                *row[127] = triple_xor_32_output_limb_1_col127;
                *lookup_data.triple_xor_32_3 = [
                    blake_round_output_limb_6_col93,
                    blake_round_output_limb_7_col94,
                    blake_round_output_limb_22_col109,
                    blake_round_output_limb_23_col110,
                    low_16_bits_col51,
                    high_16_bits_col52,
                    triple_xor_32_output_limb_0_col126,
                    triple_xor_32_output_limb_1_col127,
                ];
                *sub_component_inputs.triple_xor_32[4] = [
                    blake_round_output_round_9_tmp_53f39_124.2 .0[4],
                    blake_round_output_round_9_tmp_53f39_124.2 .0[12],
                    create_blake_round_input_output_tmp_53f39_114[4],
                ];
                let triple_xor_32_output_tmp_53f39_129 = PackedTripleXor32::deduce_output([
                    blake_round_output_round_9_tmp_53f39_124.2 .0[4],
                    blake_round_output_round_9_tmp_53f39_124.2 .0[12],
                    create_blake_round_input_output_tmp_53f39_114[4],
                ]);
                let triple_xor_32_output_limb_0_col128 =
                    triple_xor_32_output_tmp_53f39_129.low().as_m31();
                *row[128] = triple_xor_32_output_limb_0_col128;
                let triple_xor_32_output_limb_1_col129 =
                    triple_xor_32_output_tmp_53f39_129.high().as_m31();
                *row[129] = triple_xor_32_output_limb_1_col129;
                *lookup_data.triple_xor_32_4 = [
                    blake_round_output_limb_8_col95,
                    blake_round_output_limb_9_col96,
                    blake_round_output_limb_24_col111,
                    blake_round_output_limb_25_col112,
                    low_16_bits_col57,
                    high_16_bits_col58,
                    triple_xor_32_output_limb_0_col128,
                    triple_xor_32_output_limb_1_col129,
                ];
                *sub_component_inputs.triple_xor_32[5] = [
                    blake_round_output_round_9_tmp_53f39_124.2 .0[5],
                    blake_round_output_round_9_tmp_53f39_124.2 .0[13],
                    create_blake_round_input_output_tmp_53f39_114[5],
                ];
                let triple_xor_32_output_tmp_53f39_130 = PackedTripleXor32::deduce_output([
                    blake_round_output_round_9_tmp_53f39_124.2 .0[5],
                    blake_round_output_round_9_tmp_53f39_124.2 .0[13],
                    create_blake_round_input_output_tmp_53f39_114[5],
                ]);
                let triple_xor_32_output_limb_0_col130 =
                    triple_xor_32_output_tmp_53f39_130.low().as_m31();
                *row[130] = triple_xor_32_output_limb_0_col130;
                let triple_xor_32_output_limb_1_col131 =
                    triple_xor_32_output_tmp_53f39_130.high().as_m31();
                *row[131] = triple_xor_32_output_limb_1_col131;
                *lookup_data.triple_xor_32_5 = [
                    blake_round_output_limb_10_col97,
                    blake_round_output_limb_11_col98,
                    blake_round_output_limb_26_col113,
                    blake_round_output_limb_27_col114,
                    low_16_bits_col63,
                    high_16_bits_col64,
                    triple_xor_32_output_limb_0_col130,
                    triple_xor_32_output_limb_1_col131,
                ];
                *sub_component_inputs.triple_xor_32[6] = [
                    blake_round_output_round_9_tmp_53f39_124.2 .0[6],
                    blake_round_output_round_9_tmp_53f39_124.2 .0[14],
                    create_blake_round_input_output_tmp_53f39_114[6],
                ];
                let triple_xor_32_output_tmp_53f39_131 = PackedTripleXor32::deduce_output([
                    blake_round_output_round_9_tmp_53f39_124.2 .0[6],
                    blake_round_output_round_9_tmp_53f39_124.2 .0[14],
                    create_blake_round_input_output_tmp_53f39_114[6],
                ]);
                let triple_xor_32_output_limb_0_col132 =
                    triple_xor_32_output_tmp_53f39_131.low().as_m31();
                *row[132] = triple_xor_32_output_limb_0_col132;
                let triple_xor_32_output_limb_1_col133 =
                    triple_xor_32_output_tmp_53f39_131.high().as_m31();
                *row[133] = triple_xor_32_output_limb_1_col133;
                *lookup_data.triple_xor_32_6 = [
                    blake_round_output_limb_12_col99,
                    blake_round_output_limb_13_col100,
                    blake_round_output_limb_28_col115,
                    blake_round_output_limb_29_col116,
                    low_16_bits_col69,
                    high_16_bits_col70,
                    triple_xor_32_output_limb_0_col132,
                    triple_xor_32_output_limb_1_col133,
                ];
                *sub_component_inputs.triple_xor_32[7] = [
                    blake_round_output_round_9_tmp_53f39_124.2 .0[7],
                    blake_round_output_round_9_tmp_53f39_124.2 .0[15],
                    create_blake_round_input_output_tmp_53f39_114[7],
                ];
                let triple_xor_32_output_tmp_53f39_132 = PackedTripleXor32::deduce_output([
                    blake_round_output_round_9_tmp_53f39_124.2 .0[7],
                    blake_round_output_round_9_tmp_53f39_124.2 .0[15],
                    create_blake_round_input_output_tmp_53f39_114[7],
                ]);
                let triple_xor_32_output_limb_0_col134 =
                    triple_xor_32_output_tmp_53f39_132.low().as_m31();
                *row[134] = triple_xor_32_output_limb_0_col134;
                let triple_xor_32_output_limb_1_col135 =
                    triple_xor_32_output_tmp_53f39_132.high().as_m31();
                *row[135] = triple_xor_32_output_limb_1_col135;
                *lookup_data.triple_xor_32_7 = [
                    blake_round_output_limb_14_col101,
                    blake_round_output_limb_15_col102,
                    blake_round_output_limb_30_col117,
                    blake_round_output_limb_31_col118,
                    low_16_bits_col75,
                    high_16_bits_col76,
                    triple_xor_32_output_limb_0_col134,
                    triple_xor_32_output_limb_1_col135,
                ];
                let create_blake_output_output_tmp_53f39_133 = [
                    triple_xor_32_output_tmp_53f39_125,
                    triple_xor_32_output_tmp_53f39_126,
                    triple_xor_32_output_tmp_53f39_127,
                    triple_xor_32_output_tmp_53f39_128,
                    triple_xor_32_output_tmp_53f39_129,
                    triple_xor_32_output_tmp_53f39_130,
                    triple_xor_32_output_tmp_53f39_131,
                    triple_xor_32_output_tmp_53f39_132,
                ];

                // Verify Blake Word.

                let low_7_ms_bits_tmp_53f39_134 =
                    ((create_blake_output_output_tmp_53f39_133[0].low()) >> (UInt16_9));
                let low_7_ms_bits_col136 = low_7_ms_bits_tmp_53f39_134.as_m31();
                *row[136] = low_7_ms_bits_col136;
                let high_14_ms_bits_tmp_53f39_135 =
                    ((create_blake_output_output_tmp_53f39_133[0].high()) >> (UInt16_2));
                let high_14_ms_bits_col137 = high_14_ms_bits_tmp_53f39_135.as_m31();
                *row[137] = high_14_ms_bits_col137;
                let high_5_ms_bits_tmp_53f39_136 = ((high_14_ms_bits_tmp_53f39_135) >> (UInt16_9));
                let high_5_ms_bits_col138 = high_5_ms_bits_tmp_53f39_136.as_m31();
                *row[138] = high_5_ms_bits_col138;
                *sub_component_inputs.range_check_7_2_5[9] = [
                    low_7_ms_bits_col136,
                    ((triple_xor_32_output_limb_1_col121) - ((high_14_ms_bits_col137) * (M31_4))),
                    high_5_ms_bits_col138,
                ];
                *lookup_data.range_check_7_2_5_9 = [
                    low_7_ms_bits_col136,
                    ((triple_xor_32_output_limb_1_col121) - ((high_14_ms_bits_col137) * (M31_4))),
                    high_5_ms_bits_col138,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_53f39_137 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        decode_blake_opcode_output_tmp_53f39_29.0[2],
                    ));
                let new_state_0_id_col139 = memory_address_to_id_value_tmp_53f39_137;
                *row[139] = new_state_0_id_col139;
                *sub_component_inputs.memory_address_to_id[12] =
                    PackedRelocatable::from_ap_m31(decode_blake_opcode_output_tmp_53f39_29.0[2]);
                *lookup_data.memory_address_to_id_12 = [
                    M31_1,
                    decode_blake_opcode_output_tmp_53f39_29.0[2],
                    new_state_0_id_col139,
                ];
                *sub_component_inputs.memory_id_to_big[12] = new_state_0_id_col139;
                *lookup_data.memory_id_to_big_12 = [
                    new_state_0_id_col139,
                    ((triple_xor_32_output_limb_0_col120) - ((low_7_ms_bits_col136) * (M31_512))),
                    ((low_7_ms_bits_col136)
                        + (((triple_xor_32_output_limb_1_col121)
                            - ((high_14_ms_bits_col137) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col137) - ((high_5_ms_bits_col138) * (M31_512))),
                    high_5_ms_bits_col138,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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

                // Verify Blake Word.

                let low_7_ms_bits_tmp_53f39_138 =
                    ((create_blake_output_output_tmp_53f39_133[1].low()) >> (UInt16_9));
                let low_7_ms_bits_col140 = low_7_ms_bits_tmp_53f39_138.as_m31();
                *row[140] = low_7_ms_bits_col140;
                let high_14_ms_bits_tmp_53f39_139 =
                    ((create_blake_output_output_tmp_53f39_133[1].high()) >> (UInt16_2));
                let high_14_ms_bits_col141 = high_14_ms_bits_tmp_53f39_139.as_m31();
                *row[141] = high_14_ms_bits_col141;
                let high_5_ms_bits_tmp_53f39_140 = ((high_14_ms_bits_tmp_53f39_139) >> (UInt16_9));
                let high_5_ms_bits_col142 = high_5_ms_bits_tmp_53f39_140.as_m31();
                *row[142] = high_5_ms_bits_col142;
                *sub_component_inputs.range_check_7_2_5[10] = [
                    low_7_ms_bits_col140,
                    ((triple_xor_32_output_limb_1_col123) - ((high_14_ms_bits_col141) * (M31_4))),
                    high_5_ms_bits_col142,
                ];
                *lookup_data.range_check_7_2_5_10 = [
                    low_7_ms_bits_col140,
                    ((triple_xor_32_output_limb_1_col123) - ((high_14_ms_bits_col141) * (M31_4))),
                    high_5_ms_bits_col142,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_53f39_141 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((decode_blake_opcode_output_tmp_53f39_29.0[2]) + (M31_1)),
                    ));
                let new_state_1_id_col143 = memory_address_to_id_value_tmp_53f39_141;
                *row[143] = new_state_1_id_col143;
                *sub_component_inputs.memory_address_to_id[13] = PackedRelocatable::from_ap_m31(
                    ((decode_blake_opcode_output_tmp_53f39_29.0[2]) + (M31_1)),
                );
                *lookup_data.memory_address_to_id_13 = [
                    M31_1,
                    ((decode_blake_opcode_output_tmp_53f39_29.0[2]) + (M31_1)),
                    new_state_1_id_col143,
                ];
                *sub_component_inputs.memory_id_to_big[13] = new_state_1_id_col143;
                *lookup_data.memory_id_to_big_13 = [
                    new_state_1_id_col143,
                    ((triple_xor_32_output_limb_0_col122) - ((low_7_ms_bits_col140) * (M31_512))),
                    ((low_7_ms_bits_col140)
                        + (((triple_xor_32_output_limb_1_col123)
                            - ((high_14_ms_bits_col141) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col141) - ((high_5_ms_bits_col142) * (M31_512))),
                    high_5_ms_bits_col142,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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

                // Verify Blake Word.

                let low_7_ms_bits_tmp_53f39_142 =
                    ((create_blake_output_output_tmp_53f39_133[2].low()) >> (UInt16_9));
                let low_7_ms_bits_col144 = low_7_ms_bits_tmp_53f39_142.as_m31();
                *row[144] = low_7_ms_bits_col144;
                let high_14_ms_bits_tmp_53f39_143 =
                    ((create_blake_output_output_tmp_53f39_133[2].high()) >> (UInt16_2));
                let high_14_ms_bits_col145 = high_14_ms_bits_tmp_53f39_143.as_m31();
                *row[145] = high_14_ms_bits_col145;
                let high_5_ms_bits_tmp_53f39_144 = ((high_14_ms_bits_tmp_53f39_143) >> (UInt16_9));
                let high_5_ms_bits_col146 = high_5_ms_bits_tmp_53f39_144.as_m31();
                *row[146] = high_5_ms_bits_col146;
                *sub_component_inputs.range_check_7_2_5[11] = [
                    low_7_ms_bits_col144,
                    ((triple_xor_32_output_limb_1_col125) - ((high_14_ms_bits_col145) * (M31_4))),
                    high_5_ms_bits_col146,
                ];
                *lookup_data.range_check_7_2_5_11 = [
                    low_7_ms_bits_col144,
                    ((triple_xor_32_output_limb_1_col125) - ((high_14_ms_bits_col145) * (M31_4))),
                    high_5_ms_bits_col146,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_53f39_145 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((decode_blake_opcode_output_tmp_53f39_29.0[2]) + (M31_2)),
                    ));
                let new_state_2_id_col147 = memory_address_to_id_value_tmp_53f39_145;
                *row[147] = new_state_2_id_col147;
                *sub_component_inputs.memory_address_to_id[14] = PackedRelocatable::from_ap_m31(
                    ((decode_blake_opcode_output_tmp_53f39_29.0[2]) + (M31_2)),
                );
                *lookup_data.memory_address_to_id_14 = [
                    M31_1,
                    ((decode_blake_opcode_output_tmp_53f39_29.0[2]) + (M31_2)),
                    new_state_2_id_col147,
                ];
                *sub_component_inputs.memory_id_to_big[14] = new_state_2_id_col147;
                *lookup_data.memory_id_to_big_14 = [
                    new_state_2_id_col147,
                    ((triple_xor_32_output_limb_0_col124) - ((low_7_ms_bits_col144) * (M31_512))),
                    ((low_7_ms_bits_col144)
                        + (((triple_xor_32_output_limb_1_col125)
                            - ((high_14_ms_bits_col145) * (M31_4)))
                            * (M31_128))),
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

                // Verify Blake Word.

                let low_7_ms_bits_tmp_53f39_146 =
                    ((create_blake_output_output_tmp_53f39_133[3].low()) >> (UInt16_9));
                let low_7_ms_bits_col148 = low_7_ms_bits_tmp_53f39_146.as_m31();
                *row[148] = low_7_ms_bits_col148;
                let high_14_ms_bits_tmp_53f39_147 =
                    ((create_blake_output_output_tmp_53f39_133[3].high()) >> (UInt16_2));
                let high_14_ms_bits_col149 = high_14_ms_bits_tmp_53f39_147.as_m31();
                *row[149] = high_14_ms_bits_col149;
                let high_5_ms_bits_tmp_53f39_148 = ((high_14_ms_bits_tmp_53f39_147) >> (UInt16_9));
                let high_5_ms_bits_col150 = high_5_ms_bits_tmp_53f39_148.as_m31();
                *row[150] = high_5_ms_bits_col150;
                *sub_component_inputs.range_check_7_2_5[12] = [
                    low_7_ms_bits_col148,
                    ((triple_xor_32_output_limb_1_col127) - ((high_14_ms_bits_col149) * (M31_4))),
                    high_5_ms_bits_col150,
                ];
                *lookup_data.range_check_7_2_5_12 = [
                    low_7_ms_bits_col148,
                    ((triple_xor_32_output_limb_1_col127) - ((high_14_ms_bits_col149) * (M31_4))),
                    high_5_ms_bits_col150,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_53f39_149 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((decode_blake_opcode_output_tmp_53f39_29.0[2]) + (M31_3)),
                    ));
                let new_state_3_id_col151 = memory_address_to_id_value_tmp_53f39_149;
                *row[151] = new_state_3_id_col151;
                *sub_component_inputs.memory_address_to_id[15] = PackedRelocatable::from_ap_m31(
                    ((decode_blake_opcode_output_tmp_53f39_29.0[2]) + (M31_3)),
                );
                *lookup_data.memory_address_to_id_15 = [
                    M31_1,
                    ((decode_blake_opcode_output_tmp_53f39_29.0[2]) + (M31_3)),
                    new_state_3_id_col151,
                ];
                *sub_component_inputs.memory_id_to_big[15] = new_state_3_id_col151;
                *lookup_data.memory_id_to_big_15 = [
                    new_state_3_id_col151,
                    ((triple_xor_32_output_limb_0_col126) - ((low_7_ms_bits_col148) * (M31_512))),
                    ((low_7_ms_bits_col148)
                        + (((triple_xor_32_output_limb_1_col127)
                            - ((high_14_ms_bits_col149) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col149) - ((high_5_ms_bits_col150) * (M31_512))),
                    high_5_ms_bits_col150,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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

                // Verify Blake Word.

                let low_7_ms_bits_tmp_53f39_150 =
                    ((create_blake_output_output_tmp_53f39_133[4].low()) >> (UInt16_9));
                let low_7_ms_bits_col152 = low_7_ms_bits_tmp_53f39_150.as_m31();
                *row[152] = low_7_ms_bits_col152;
                let high_14_ms_bits_tmp_53f39_151 =
                    ((create_blake_output_output_tmp_53f39_133[4].high()) >> (UInt16_2));
                let high_14_ms_bits_col153 = high_14_ms_bits_tmp_53f39_151.as_m31();
                *row[153] = high_14_ms_bits_col153;
                let high_5_ms_bits_tmp_53f39_152 = ((high_14_ms_bits_tmp_53f39_151) >> (UInt16_9));
                let high_5_ms_bits_col154 = high_5_ms_bits_tmp_53f39_152.as_m31();
                *row[154] = high_5_ms_bits_col154;
                *sub_component_inputs.range_check_7_2_5[13] = [
                    low_7_ms_bits_col152,
                    ((triple_xor_32_output_limb_1_col129) - ((high_14_ms_bits_col153) * (M31_4))),
                    high_5_ms_bits_col154,
                ];
                *lookup_data.range_check_7_2_5_13 = [
                    low_7_ms_bits_col152,
                    ((triple_xor_32_output_limb_1_col129) - ((high_14_ms_bits_col153) * (M31_4))),
                    high_5_ms_bits_col154,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_53f39_153 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((decode_blake_opcode_output_tmp_53f39_29.0[2]) + (M31_4)),
                    ));
                let new_state_4_id_col155 = memory_address_to_id_value_tmp_53f39_153;
                *row[155] = new_state_4_id_col155;
                *sub_component_inputs.memory_address_to_id[16] = PackedRelocatable::from_ap_m31(
                    ((decode_blake_opcode_output_tmp_53f39_29.0[2]) + (M31_4)),
                );
                *lookup_data.memory_address_to_id_16 = [
                    M31_1,
                    ((decode_blake_opcode_output_tmp_53f39_29.0[2]) + (M31_4)),
                    new_state_4_id_col155,
                ];
                *sub_component_inputs.memory_id_to_big[16] = new_state_4_id_col155;
                *lookup_data.memory_id_to_big_16 = [
                    new_state_4_id_col155,
                    ((triple_xor_32_output_limb_0_col128) - ((low_7_ms_bits_col152) * (M31_512))),
                    ((low_7_ms_bits_col152)
                        + (((triple_xor_32_output_limb_1_col129)
                            - ((high_14_ms_bits_col153) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col153) - ((high_5_ms_bits_col154) * (M31_512))),
                    high_5_ms_bits_col154,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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

                // Verify Blake Word.

                let low_7_ms_bits_tmp_53f39_154 =
                    ((create_blake_output_output_tmp_53f39_133[5].low()) >> (UInt16_9));
                let low_7_ms_bits_col156 = low_7_ms_bits_tmp_53f39_154.as_m31();
                *row[156] = low_7_ms_bits_col156;
                let high_14_ms_bits_tmp_53f39_155 =
                    ((create_blake_output_output_tmp_53f39_133[5].high()) >> (UInt16_2));
                let high_14_ms_bits_col157 = high_14_ms_bits_tmp_53f39_155.as_m31();
                *row[157] = high_14_ms_bits_col157;
                let high_5_ms_bits_tmp_53f39_156 = ((high_14_ms_bits_tmp_53f39_155) >> (UInt16_9));
                let high_5_ms_bits_col158 = high_5_ms_bits_tmp_53f39_156.as_m31();
                *row[158] = high_5_ms_bits_col158;
                *sub_component_inputs.range_check_7_2_5[14] = [
                    low_7_ms_bits_col156,
                    ((triple_xor_32_output_limb_1_col131) - ((high_14_ms_bits_col157) * (M31_4))),
                    high_5_ms_bits_col158,
                ];
                *lookup_data.range_check_7_2_5_14 = [
                    low_7_ms_bits_col156,
                    ((triple_xor_32_output_limb_1_col131) - ((high_14_ms_bits_col157) * (M31_4))),
                    high_5_ms_bits_col158,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_53f39_157 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((decode_blake_opcode_output_tmp_53f39_29.0[2]) + (M31_5)),
                    ));
                let new_state_5_id_col159 = memory_address_to_id_value_tmp_53f39_157;
                *row[159] = new_state_5_id_col159;
                *sub_component_inputs.memory_address_to_id[17] = PackedRelocatable::from_ap_m31(
                    ((decode_blake_opcode_output_tmp_53f39_29.0[2]) + (M31_5)),
                );
                *lookup_data.memory_address_to_id_17 = [
                    M31_1,
                    ((decode_blake_opcode_output_tmp_53f39_29.0[2]) + (M31_5)),
                    new_state_5_id_col159,
                ];
                *sub_component_inputs.memory_id_to_big[17] = new_state_5_id_col159;
                *lookup_data.memory_id_to_big_17 = [
                    new_state_5_id_col159,
                    ((triple_xor_32_output_limb_0_col130) - ((low_7_ms_bits_col156) * (M31_512))),
                    ((low_7_ms_bits_col156)
                        + (((triple_xor_32_output_limb_1_col131)
                            - ((high_14_ms_bits_col157) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col157) - ((high_5_ms_bits_col158) * (M31_512))),
                    high_5_ms_bits_col158,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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

                // Verify Blake Word.

                let low_7_ms_bits_tmp_53f39_158 =
                    ((create_blake_output_output_tmp_53f39_133[6].low()) >> (UInt16_9));
                let low_7_ms_bits_col160 = low_7_ms_bits_tmp_53f39_158.as_m31();
                *row[160] = low_7_ms_bits_col160;
                let high_14_ms_bits_tmp_53f39_159 =
                    ((create_blake_output_output_tmp_53f39_133[6].high()) >> (UInt16_2));
                let high_14_ms_bits_col161 = high_14_ms_bits_tmp_53f39_159.as_m31();
                *row[161] = high_14_ms_bits_col161;
                let high_5_ms_bits_tmp_53f39_160 = ((high_14_ms_bits_tmp_53f39_159) >> (UInt16_9));
                let high_5_ms_bits_col162 = high_5_ms_bits_tmp_53f39_160.as_m31();
                *row[162] = high_5_ms_bits_col162;
                *sub_component_inputs.range_check_7_2_5[15] = [
                    low_7_ms_bits_col160,
                    ((triple_xor_32_output_limb_1_col133) - ((high_14_ms_bits_col161) * (M31_4))),
                    high_5_ms_bits_col162,
                ];
                *lookup_data.range_check_7_2_5_15 = [
                    low_7_ms_bits_col160,
                    ((triple_xor_32_output_limb_1_col133) - ((high_14_ms_bits_col161) * (M31_4))),
                    high_5_ms_bits_col162,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_53f39_161 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((decode_blake_opcode_output_tmp_53f39_29.0[2]) + (M31_6)),
                    ));
                let new_state_6_id_col163 = memory_address_to_id_value_tmp_53f39_161;
                *row[163] = new_state_6_id_col163;
                *sub_component_inputs.memory_address_to_id[18] = PackedRelocatable::from_ap_m31(
                    ((decode_blake_opcode_output_tmp_53f39_29.0[2]) + (M31_6)),
                );
                *lookup_data.memory_address_to_id_18 = [
                    M31_1,
                    ((decode_blake_opcode_output_tmp_53f39_29.0[2]) + (M31_6)),
                    new_state_6_id_col163,
                ];
                *sub_component_inputs.memory_id_to_big[18] = new_state_6_id_col163;
                *lookup_data.memory_id_to_big_18 = [
                    new_state_6_id_col163,
                    ((triple_xor_32_output_limb_0_col132) - ((low_7_ms_bits_col160) * (M31_512))),
                    ((low_7_ms_bits_col160)
                        + (((triple_xor_32_output_limb_1_col133)
                            - ((high_14_ms_bits_col161) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col161) - ((high_5_ms_bits_col162) * (M31_512))),
                    high_5_ms_bits_col162,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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

                // Verify Blake Word.

                let low_7_ms_bits_tmp_53f39_162 =
                    ((create_blake_output_output_tmp_53f39_133[7].low()) >> (UInt16_9));
                let low_7_ms_bits_col164 = low_7_ms_bits_tmp_53f39_162.as_m31();
                *row[164] = low_7_ms_bits_col164;
                let high_14_ms_bits_tmp_53f39_163 =
                    ((create_blake_output_output_tmp_53f39_133[7].high()) >> (UInt16_2));
                let high_14_ms_bits_col165 = high_14_ms_bits_tmp_53f39_163.as_m31();
                *row[165] = high_14_ms_bits_col165;
                let high_5_ms_bits_tmp_53f39_164 = ((high_14_ms_bits_tmp_53f39_163) >> (UInt16_9));
                let high_5_ms_bits_col166 = high_5_ms_bits_tmp_53f39_164.as_m31();
                *row[166] = high_5_ms_bits_col166;
                *sub_component_inputs.range_check_7_2_5[16] = [
                    low_7_ms_bits_col164,
                    ((triple_xor_32_output_limb_1_col135) - ((high_14_ms_bits_col165) * (M31_4))),
                    high_5_ms_bits_col166,
                ];
                *lookup_data.range_check_7_2_5_16 = [
                    low_7_ms_bits_col164,
                    ((triple_xor_32_output_limb_1_col135) - ((high_14_ms_bits_col165) * (M31_4))),
                    high_5_ms_bits_col166,
                ];

                // Mem Verify.

                let memory_address_to_id_value_tmp_53f39_165 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((decode_blake_opcode_output_tmp_53f39_29.0[2]) + (M31_7)),
                    ));
                let new_state_7_id_col167 = memory_address_to_id_value_tmp_53f39_165;
                *row[167] = new_state_7_id_col167;
                *sub_component_inputs.memory_address_to_id[19] = PackedRelocatable::from_ap_m31(
                    ((decode_blake_opcode_output_tmp_53f39_29.0[2]) + (M31_7)),
                );
                *lookup_data.memory_address_to_id_19 = [
                    M31_1,
                    ((decode_blake_opcode_output_tmp_53f39_29.0[2]) + (M31_7)),
                    new_state_7_id_col167,
                ];
                *sub_component_inputs.memory_id_to_big[19] = new_state_7_id_col167;
                *lookup_data.memory_id_to_big_19 = [
                    new_state_7_id_col167,
                    ((triple_xor_32_output_limb_0_col134) - ((low_7_ms_bits_col164) * (M31_512))),
                    ((low_7_ms_bits_col164)
                        + (((triple_xor_32_output_limb_1_col135)
                            - ((high_14_ms_bits_col165) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col165) - ((high_5_ms_bits_col166) * (M31_512))),
                    high_5_ms_bits_col166,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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

                *lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];
                *lookup_data.opcodes_1 = [
                    ((input_pc_col0) + (M31_1)),
                    ((input_ap_col1) + (ap_update_add_1_col10)),
                    input_fp_col2,
                ];
                *row[168] = enabler_col.packed_at(row_index);
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    blake_round_0: Vec<[PackedM31; 35]>,
    blake_round_1: Vec<[PackedM31; 35]>,
    memory_address_to_id_0: Vec<[PackedM31; 3]>,
    memory_address_to_id_1: Vec<[PackedM31; 3]>,
    memory_address_to_id_2: Vec<[PackedM31; 3]>,
    memory_address_to_id_3: Vec<[PackedM31; 3]>,
    memory_address_to_id_4: Vec<[PackedM31; 3]>,
    memory_address_to_id_5: Vec<[PackedM31; 3]>,
    memory_address_to_id_6: Vec<[PackedM31; 3]>,
    memory_address_to_id_7: Vec<[PackedM31; 3]>,
    memory_address_to_id_8: Vec<[PackedM31; 3]>,
    memory_address_to_id_9: Vec<[PackedM31; 3]>,
    memory_address_to_id_10: Vec<[PackedM31; 3]>,
    memory_address_to_id_11: Vec<[PackedM31; 3]>,
    memory_address_to_id_12: Vec<[PackedM31; 3]>,
    memory_address_to_id_13: Vec<[PackedM31; 3]>,
    memory_address_to_id_14: Vec<[PackedM31; 3]>,
    memory_address_to_id_15: Vec<[PackedM31; 3]>,
    memory_address_to_id_16: Vec<[PackedM31; 3]>,
    memory_address_to_id_17: Vec<[PackedM31; 3]>,
    memory_address_to_id_18: Vec<[PackedM31; 3]>,
    memory_address_to_id_19: Vec<[PackedM31; 3]>,
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
    memory_id_to_big_16: Vec<[PackedM31; 29]>,
    memory_id_to_big_17: Vec<[PackedM31; 29]>,
    memory_id_to_big_18: Vec<[PackedM31; 29]>,
    memory_id_to_big_19: Vec<[PackedM31; 29]>,
    opcodes_0: Vec<[PackedM31; 3]>,
    opcodes_1: Vec<[PackedM31; 3]>,
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
    range_check_7_2_5_16: Vec<[PackedM31; 3]>,
    triple_xor_32_0: Vec<[PackedM31; 8]>,
    triple_xor_32_1: Vec<[PackedM31; 8]>,
    triple_xor_32_2: Vec<[PackedM31; 8]>,
    triple_xor_32_3: Vec<[PackedM31; 8]>,
    triple_xor_32_4: Vec<[PackedM31; 8]>,
    triple_xor_32_5: Vec<[PackedM31; 8]>,
    triple_xor_32_6: Vec<[PackedM31; 8]>,
    triple_xor_32_7: Vec<[PackedM31; 8]>,
    verify_bitwise_xor_8_0: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_1: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_2: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_3: Vec<[PackedM31; 3]>,
    verify_instruction_0: Vec<[PackedM31; 7]>,
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
        blake_round: &relations::BlakeRound,
        memory_address_to_id: &relations::MemoryAddressToId,
        memory_id_to_big: &relations::MemoryIdToBig,
        opcodes: &relations::Opcodes,
        range_check_7_2_5: &relations::RangeCheck_7_2_5,
        triple_xor_32: &relations::TripleXor32,
        verify_bitwise_xor_8: &relations::VerifyBitwiseXor_8,
        verify_instruction: &relations::VerifyInstruction,
    ) -> InteractionClaim {
        let enabler_col = Enabler::new(self.n_rows);
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_instruction_0,
            &self.lookup_data.memory_address_to_id_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_instruction.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_0,
            &self.lookup_data.memory_address_to_id_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_1,
            &self.lookup_data.memory_address_to_id_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_2,
            &self.lookup_data.range_check_7_2_5_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = range_check_7_2_5.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_3,
            &self.lookup_data.memory_id_to_big_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_7_2_5_1,
            &self.lookup_data.memory_address_to_id_4,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_7_2_5.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_4,
            &self.lookup_data.range_check_7_2_5_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = range_check_7_2_5.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_5,
            &self.lookup_data.memory_id_to_big_5,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_7_2_5_3,
            &self.lookup_data.memory_address_to_id_6,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_7_2_5.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_6,
            &self.lookup_data.range_check_7_2_5_4,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = range_check_7_2_5.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_7,
            &self.lookup_data.memory_id_to_big_7,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_7_2_5_5,
            &self.lookup_data.memory_address_to_id_8,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_7_2_5.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_8,
            &self.lookup_data.range_check_7_2_5_6,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = range_check_7_2_5.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_9,
            &self.lookup_data.memory_id_to_big_9,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_7_2_5_7,
            &self.lookup_data.memory_address_to_id_10,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_7_2_5.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_10,
            &self.lookup_data.range_check_7_2_5_8,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = range_check_7_2_5.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_11,
            &self.lookup_data.memory_id_to_big_11,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_xor_8_0,
            &self.lookup_data.verify_bitwise_xor_8_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_xor_8.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_xor_8.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_xor_8_2,
            &self.lookup_data.verify_bitwise_xor_8_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_xor_8.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_xor_8.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.blake_round_0,
            &self.lookup_data.blake_round_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = blake_round.combine(values0);
                let denom1: PackedQM31 = blake_round.combine(values1);
                writer.write_frac(denom0 - denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.triple_xor_32_0,
            &self.lookup_data.triple_xor_32_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = triple_xor_32.combine(values0);
                let denom1: PackedQM31 = triple_xor_32.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.triple_xor_32_2,
            &self.lookup_data.triple_xor_32_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = triple_xor_32.combine(values0);
                let denom1: PackedQM31 = triple_xor_32.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.triple_xor_32_4,
            &self.lookup_data.triple_xor_32_5,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = triple_xor_32.combine(values0);
                let denom1: PackedQM31 = triple_xor_32.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.triple_xor_32_6,
            &self.lookup_data.triple_xor_32_7,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = triple_xor_32.combine(values0);
                let denom1: PackedQM31 = triple_xor_32.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_7_2_5_9,
            &self.lookup_data.memory_address_to_id_12,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_7_2_5.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_12,
            &self.lookup_data.range_check_7_2_5_10,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = range_check_7_2_5.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_13,
            &self.lookup_data.memory_id_to_big_13,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_7_2_5_11,
            &self.lookup_data.memory_address_to_id_14,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_7_2_5.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_14,
            &self.lookup_data.range_check_7_2_5_12,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = range_check_7_2_5.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_15,
            &self.lookup_data.memory_id_to_big_15,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_7_2_5_13,
            &self.lookup_data.memory_address_to_id_16,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_7_2_5.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_16,
            &self.lookup_data.range_check_7_2_5_14,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = range_check_7_2_5.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_17,
            &self.lookup_data.memory_id_to_big_17,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_7_2_5_15,
            &self.lookup_data.memory_address_to_id_18,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_7_2_5.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_18,
            &self.lookup_data.range_check_7_2_5_16,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = range_check_7_2_5.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_19,
            &self.lookup_data.memory_id_to_big_19,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.opcodes_0,
            &self.lookup_data.opcodes_1,
        )
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (writer, values0, values1))| {
                let denom0: PackedQM31 = opcodes.combine(values0);
                let denom1: PackedQM31 = opcodes.combine(values1);
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
