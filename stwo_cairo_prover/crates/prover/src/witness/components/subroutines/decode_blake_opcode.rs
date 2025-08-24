// AIR version 97774321-dirty
#![allow(unused_parens)]
use cairo_air::components::decode_blake_opcode::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{
    memory_address_to_id, memory_id_to_big, range_check_7_2_5, verify_instruction,
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
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
        range_check_7_2_5_state: &range_check_7_2_5::ClaimGenerator,
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
            memory_address_to_id_state,
            memory_id_to_big_state,
            range_check_7_2_5_state,
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
        tree_builder.extend_evals(trace.to_evals());

        (
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
    memory_address_to_id: [Vec<memory_address_to_id::PackedInputType>; 4],
    memory_id_to_big: [Vec<memory_id_to_big::PackedInputType>; 4],
    range_check_7_2_5: [Vec<range_check_7_2_5::PackedInputType>; 1],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
    range_check_7_2_5_state: &range_check_7_2_5::ClaimGenerator,
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
    let M31_128 = PackedM31::broadcast(M31::from(128));
    let M31_16 = PackedM31::broadcast(M31::from(16));
    let M31_2048 = PackedM31::broadcast(M31::from(2048));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_4 = PackedM31::broadcast(M31::from(4));
    let M31_512 = PackedM31::broadcast(M31::from(512));
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
                (mut row, lookup_data, sub_component_inputs, decode_blake_opcode_input),
            )| {
                // Decode Instruction.

                let memory_address_to_id_value_tmp_47e62_0 =
                    memory_address_to_id_state.deduce_output(decode_blake_opcode_input.pc);
                let memory_id_to_big_value_tmp_47e62_1 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_47e62_0);
                let offset0_tmp_47e62_2 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_47e62_1.get_m31(0)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_47e62_1.get_m31(1),
                        )) & (UInt16_127))
                            << (UInt16_9)));
                let offset0_col0 = offset0_tmp_47e62_2.as_m31();
                *row[0] = offset0_col0;
                let offset1_tmp_47e62_3 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_47e62_1.get_m31(1)))
                        >> (UInt16_7))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_47e62_1.get_m31(2),
                        )) << (UInt16_2)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_47e62_1.get_m31(3),
                        )) & (UInt16_31))
                            << (UInt16_11)));
                let offset1_col1 = offset1_tmp_47e62_3.as_m31();
                *row[1] = offset1_col1;
                let offset2_tmp_47e62_4 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_47e62_1.get_m31(3)))
                        >> (UInt16_5))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_47e62_1.get_m31(4),
                        )) << (UInt16_4)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_47e62_1.get_m31(5),
                        )) & (UInt16_7))
                            << (UInt16_13)));
                let offset2_col2 = offset2_tmp_47e62_4.as_m31();
                *row[2] = offset2_col2;
                let dst_base_fp_tmp_47e62_5 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_47e62_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_47e62_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_0))
                        & (UInt16_1));
                let dst_base_fp_col3 = dst_base_fp_tmp_47e62_5.as_m31();
                *row[3] = dst_base_fp_col3;
                let op0_base_fp_tmp_47e62_6 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_47e62_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_47e62_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_1))
                        & (UInt16_1));
                let op0_base_fp_col4 = op0_base_fp_tmp_47e62_6.as_m31();
                *row[4] = op0_base_fp_col4;
                let op1_base_fp_tmp_47e62_7 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_47e62_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_47e62_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_3))
                        & (UInt16_1));
                let op1_base_fp_col5 = op1_base_fp_tmp_47e62_7.as_m31();
                *row[5] = op1_base_fp_col5;
                let op1_base_ap_tmp_47e62_8 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_47e62_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_47e62_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_4))
                        & (UInt16_1));
                let op1_base_ap_col6 = op1_base_ap_tmp_47e62_8.as_m31();
                *row[6] = op1_base_ap_col6;
                let ap_update_add_1_tmp_47e62_9 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_47e62_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_47e62_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_11))
                        & (UInt16_1));
                let ap_update_add_1_col7 = ap_update_add_1_tmp_47e62_9.as_m31();
                *row[7] = ap_update_add_1_col7;
                let opcode_extension_col8 = memory_id_to_big_value_tmp_47e62_1.get_m31(7);
                *row[8] = opcode_extension_col8;
                *sub_component_inputs.verify_instruction[0] = (
                    decode_blake_opcode_input.pc,
                    [offset0_col0, offset1_col1, offset2_col2],
                    [
                        (((((dst_base_fp_col3) * (M31_8)) + ((op0_base_fp_col4) * (M31_16)))
                            + ((op1_base_fp_col5) * (M31_64)))
                            + ((op1_base_ap_col6) * (M31_128))),
                        ((ap_update_add_1_col7) * (M31_32)),
                    ],
                    opcode_extension_col8,
                );
                *lookup_data.verify_instruction_0 = [
                    decode_blake_opcode_input.pc,
                    offset0_col0,
                    offset1_col1,
                    offset2_col2,
                    (((((dst_base_fp_col3) * (M31_8)) + ((op0_base_fp_col4) * (M31_16)))
                        + ((op1_base_fp_col5) * (M31_64)))
                        + ((op1_base_ap_col6) * (M31_128))),
                    ((ap_update_add_1_col7) * (M31_32)),
                    opcode_extension_col8,
                ];
                let decode_instruction_64420_output_tmp_47e62_10 = (
                    [
                        ((offset0_col0) - (M31_32768)),
                        ((offset1_col1) - (M31_32768)),
                        ((offset2_col2) - (M31_32768)),
                    ],
                    [
                        dst_base_fp_col3,
                        op0_base_fp_col4,
                        M31_0,
                        op1_base_fp_col5,
                        op1_base_ap_col6,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        ap_update_add_1_col7,
                        M31_0,
                        M31_0,
                        M31_0,
                    ],
                    opcode_extension_col8,
                );

                let mem0_base_col9 = (((op0_base_fp_col4) * (decode_blake_opcode_input.fp))
                    + (((M31_1) - (op0_base_fp_col4)) * (decode_blake_opcode_input.ap)));
                *row[9] = mem0_base_col9;

                // Read Positive Num Bits 27.

                // Read Id.

                let memory_address_to_id_value_tmp_47e62_11 = memory_address_to_id_state
                    .deduce_output(
                        ((mem0_base_col9) + (decode_instruction_64420_output_tmp_47e62_10.0[1])),
                    );
                let op0_id_col10 = memory_address_to_id_value_tmp_47e62_11;
                *row[10] = op0_id_col10;
                *sub_component_inputs.memory_address_to_id[0] =
                    ((mem0_base_col9) + (decode_instruction_64420_output_tmp_47e62_10.0[1]));
                *lookup_data.memory_address_to_id_0 = [
                    ((mem0_base_col9) + (decode_instruction_64420_output_tmp_47e62_10.0[1])),
                    op0_id_col10,
                ];

                // Read Positive known Id Num Bits 27.

                let memory_id_to_big_value_tmp_47e62_13 =
                    memory_id_to_big_state.deduce_output(op0_id_col10);
                let op0_limb_0_col11 = memory_id_to_big_value_tmp_47e62_13.get_m31(0);
                *row[11] = op0_limb_0_col11;
                let op0_limb_1_col12 = memory_id_to_big_value_tmp_47e62_13.get_m31(1);
                *row[12] = op0_limb_1_col12;
                let op0_limb_2_col13 = memory_id_to_big_value_tmp_47e62_13.get_m31(2);
                *row[13] = op0_limb_2_col13;
                *sub_component_inputs.memory_id_to_big[0] = op0_id_col10;
                *lookup_data.memory_id_to_big_0 = [
                    op0_id_col10,
                    op0_limb_0_col11,
                    op0_limb_1_col12,
                    op0_limb_2_col13,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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
                let read_positive_known_id_num_bits_27_output_tmp_47e62_14 =
                    PackedFelt252::from_limbs([
                        op0_limb_0_col11,
                        op0_limb_1_col12,
                        op0_limb_2_col13,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
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

                let read_positive_num_bits_27_output_tmp_47e62_15 = (
                    read_positive_known_id_num_bits_27_output_tmp_47e62_14,
                    op0_id_col10,
                );

                let mem1_base_col14 = (((op1_base_fp_col5) * (decode_blake_opcode_input.fp))
                    + ((op1_base_ap_col6) * (decode_blake_opcode_input.ap)));
                *row[14] = mem1_base_col14;

                // Read Positive Num Bits 27.

                // Read Id.

                let memory_address_to_id_value_tmp_47e62_16 = memory_address_to_id_state
                    .deduce_output(
                        ((mem1_base_col14) + (decode_instruction_64420_output_tmp_47e62_10.0[2])),
                    );
                let op1_id_col15 = memory_address_to_id_value_tmp_47e62_16;
                *row[15] = op1_id_col15;
                *sub_component_inputs.memory_address_to_id[1] =
                    ((mem1_base_col14) + (decode_instruction_64420_output_tmp_47e62_10.0[2]));
                *lookup_data.memory_address_to_id_1 = [
                    ((mem1_base_col14) + (decode_instruction_64420_output_tmp_47e62_10.0[2])),
                    op1_id_col15,
                ];

                // Read Positive known Id Num Bits 27.

                let memory_id_to_big_value_tmp_47e62_18 =
                    memory_id_to_big_state.deduce_output(op1_id_col15);
                let op1_limb_0_col16 = memory_id_to_big_value_tmp_47e62_18.get_m31(0);
                *row[16] = op1_limb_0_col16;
                let op1_limb_1_col17 = memory_id_to_big_value_tmp_47e62_18.get_m31(1);
                *row[17] = op1_limb_1_col17;
                let op1_limb_2_col18 = memory_id_to_big_value_tmp_47e62_18.get_m31(2);
                *row[18] = op1_limb_2_col18;
                *sub_component_inputs.memory_id_to_big[1] = op1_id_col15;
                *lookup_data.memory_id_to_big_1 = [
                    op1_id_col15,
                    op1_limb_0_col16,
                    op1_limb_1_col17,
                    op1_limb_2_col18,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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
                let read_positive_known_id_num_bits_27_output_tmp_47e62_19 =
                    PackedFelt252::from_limbs([
                        op1_limb_0_col16,
                        op1_limb_1_col17,
                        op1_limb_2_col18,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
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

                let read_positive_num_bits_27_output_tmp_47e62_20 = (
                    read_positive_known_id_num_bits_27_output_tmp_47e62_19,
                    op1_id_col15,
                );

                // Read Positive Num Bits 27.

                // Read Id.

                let memory_address_to_id_value_tmp_47e62_21 =
                    memory_address_to_id_state.deduce_output(decode_blake_opcode_input.ap);
                let ap_id_col19 = memory_address_to_id_value_tmp_47e62_21;
                *row[19] = ap_id_col19;
                *sub_component_inputs.memory_address_to_id[2] = decode_blake_opcode_input.ap;
                *lookup_data.memory_address_to_id_2 = [decode_blake_opcode_input.ap, ap_id_col19];

                // Read Positive known Id Num Bits 27.

                let memory_id_to_big_value_tmp_47e62_23 =
                    memory_id_to_big_state.deduce_output(ap_id_col19);
                let ap_limb_0_col20 = memory_id_to_big_value_tmp_47e62_23.get_m31(0);
                *row[20] = ap_limb_0_col20;
                let ap_limb_1_col21 = memory_id_to_big_value_tmp_47e62_23.get_m31(1);
                *row[21] = ap_limb_1_col21;
                let ap_limb_2_col22 = memory_id_to_big_value_tmp_47e62_23.get_m31(2);
                *row[22] = ap_limb_2_col22;
                *sub_component_inputs.memory_id_to_big[2] = ap_id_col19;
                *lookup_data.memory_id_to_big_2 = [
                    ap_id_col19,
                    ap_limb_0_col20,
                    ap_limb_1_col21,
                    ap_limb_2_col22,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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
                let read_positive_known_id_num_bits_27_output_tmp_47e62_24 =
                    PackedFelt252::from_limbs([
                        ap_limb_0_col20,
                        ap_limb_1_col21,
                        ap_limb_2_col22,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
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

                let read_positive_num_bits_27_output_tmp_47e62_25 = (
                    read_positive_known_id_num_bits_27_output_tmp_47e62_24,
                    ap_id_col19,
                );

                let mem_dst_base_col23 = (((dst_base_fp_col3) * (decode_blake_opcode_input.fp))
                    + (((M31_1) - (dst_base_fp_col3)) * (decode_blake_opcode_input.ap)));
                *row[23] = mem_dst_base_col23;

                // Read Blake Word.

                let memory_address_to_id_value_tmp_47e62_26 = memory_address_to_id_state
                    .deduce_output(
                        ((mem_dst_base_col23)
                            + (decode_instruction_64420_output_tmp_47e62_10.0[0])),
                    );
                let memory_id_to_big_value_tmp_47e62_27 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_47e62_26);
                let tmp_47e62_28 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_47e62_27.get_m31(1)))
                        >> (UInt16_7));
                let low_16_bits_col24 = ((((memory_id_to_big_value_tmp_47e62_27.get_m31(1))
                    - ((tmp_47e62_28.as_m31()) * (M31_128)))
                    * (M31_512))
                    + (memory_id_to_big_value_tmp_47e62_27.get_m31(0)));
                *row[24] = low_16_bits_col24;
                let high_16_bits_col25 = ((((memory_id_to_big_value_tmp_47e62_27.get_m31(3))
                    * (M31_2048))
                    + ((memory_id_to_big_value_tmp_47e62_27.get_m31(2)) * (M31_4)))
                    + (tmp_47e62_28.as_m31()));
                *row[25] = high_16_bits_col25;
                let expected_word_tmp_47e62_29 =
                    PackedUInt32::from_limbs([low_16_bits_col24, high_16_bits_col25]);

                // Verify Blake Word.

                let low_7_ms_bits_tmp_47e62_30 = ((expected_word_tmp_47e62_29.low()) >> (UInt16_9));
                let low_7_ms_bits_col26 = low_7_ms_bits_tmp_47e62_30.as_m31();
                *row[26] = low_7_ms_bits_col26;
                let high_14_ms_bits_tmp_47e62_31 =
                    ((expected_word_tmp_47e62_29.high()) >> (UInt16_2));
                let high_14_ms_bits_col27 = high_14_ms_bits_tmp_47e62_31.as_m31();
                *row[27] = high_14_ms_bits_col27;
                let high_5_ms_bits_tmp_47e62_32 = ((high_14_ms_bits_tmp_47e62_31) >> (UInt16_9));
                let high_5_ms_bits_col28 = high_5_ms_bits_tmp_47e62_32.as_m31();
                *row[28] = high_5_ms_bits_col28;
                *sub_component_inputs.range_check_7_2_5[0] = [
                    low_7_ms_bits_col26,
                    ((high_16_bits_col25) - ((high_14_ms_bits_col27) * (M31_4))),
                    high_5_ms_bits_col28,
                ];
                *lookup_data.range_check_7_2_5_0 = [
                    low_7_ms_bits_col26,
                    ((high_16_bits_col25) - ((high_14_ms_bits_col27) * (M31_4))),
                    high_5_ms_bits_col28,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_47e62_33 = memory_address_to_id_state
                    .deduce_output(
                        ((mem_dst_base_col23)
                            + (decode_instruction_64420_output_tmp_47e62_10.0[0])),
                    );
                let dst_id_col29 = memory_address_to_id_value_tmp_47e62_33;
                *row[29] = dst_id_col29;
                *sub_component_inputs.memory_address_to_id[3] =
                    ((mem_dst_base_col23) + (decode_instruction_64420_output_tmp_47e62_10.0[0]));
                *lookup_data.memory_address_to_id_3 = [
                    ((mem_dst_base_col23) + (decode_instruction_64420_output_tmp_47e62_10.0[0])),
                    dst_id_col29,
                ];

                *sub_component_inputs.memory_id_to_big[3] = dst_id_col29;
                *lookup_data.memory_id_to_big_3 = [
                    dst_id_col29,
                    ((low_16_bits_col24) - ((low_7_ms_bits_col26) * (M31_512))),
                    ((low_7_ms_bits_col26)
                        + (((high_16_bits_col25) - ((high_14_ms_bits_col27) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col27) - ((high_5_ms_bits_col28) * (M31_512))),
                    high_5_ms_bits_col28,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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

                let read_blake_word_output_tmp_47e62_35 = expected_word_tmp_47e62_29;
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 2]>,
    memory_address_to_id_1: Vec<[PackedM31; 2]>,
    memory_address_to_id_2: Vec<[PackedM31; 2]>,
    memory_address_to_id_3: Vec<[PackedM31; 2]>,
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
    memory_id_to_big_1: Vec<[PackedM31; 29]>,
    memory_id_to_big_2: Vec<[PackedM31; 29]>,
    memory_id_to_big_3: Vec<[PackedM31; 29]>,
    range_check_7_2_5_0: Vec<[PackedM31; 3]>,
    verify_instruction_0: Vec<[PackedM31; 7]>,
}

pub struct InteractionClaimGenerator {
    log_size: u32,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        verify_instruction: &relations::VerifyInstruction,
        memory_address_to_id: &relations::MemoryAddressToId,
        memory_id_to_big: &relations::MemoryIdToBig,
        range_check_7_2_5: &relations::RangeCheck_7_2_5,
    ) -> InteractionClaim {
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

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
