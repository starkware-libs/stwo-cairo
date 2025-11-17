// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::mul_opcode_small::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{
    memory_address_to_id, memory_id_to_big, range_check_11, verify_instruction,
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
        verify_instruction_state: &verify_instruction::ClaimGenerator,
        range_check_11_state: &range_check_11::ClaimGenerator,
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
            memory_address_to_id_state,
            memory_id_to_big_state,
            verify_instruction_state,
            range_check_11_state,
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
            .range_check_11
            .iter()
            .for_each(|inputs| {
                range_check_11_state.add_packed_inputs(inputs);
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
    memory_address_to_id: [Vec<memory_address_to_id::PackedInputType>; 3],
    memory_id_to_big: [Vec<memory_id_to_big::PackedInputType>; 3],
    range_check_11: [Vec<range_check_11::PackedInputType>; 3],
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
    range_check_11_state: &range_check_11::ClaimGenerator,
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
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let M31_64 = PackedM31::broadcast(M31::from(64));
    let M31_8 = PackedM31::broadcast(M31::from(8));
    let M31_8192 = PackedM31::broadcast(M31::from(8192));
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
            |(row_index, (row, lookup_data, sub_component_inputs, mul_opcode_small_input))| {
                let input_pc_col0 = mul_opcode_small_input.pc;
                *row[0] = input_pc_col0;
                let input_ap_col1 = mul_opcode_small_input.ap;
                *row[1] = input_ap_col1;
                let input_fp_col2 = mul_opcode_small_input.fp;
                *row[2] = input_fp_col2;

                // Decode Instruction.

                let memory_address_to_id_value_tmp_9d1ad_0 =
                    memory_address_to_id_state.deduce_output(input_pc_col0);
                let memory_id_to_big_value_tmp_9d1ad_1 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_9d1ad_0);
                let offset0_tmp_9d1ad_2 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9d1ad_1.get_m31(0)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_9d1ad_1.get_m31(1),
                        )) & (UInt16_127))
                            << (UInt16_9)));
                let offset0_col3 = offset0_tmp_9d1ad_2.as_m31();
                *row[3] = offset0_col3;
                let offset1_tmp_9d1ad_3 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9d1ad_1.get_m31(1)))
                        >> (UInt16_7))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_9d1ad_1.get_m31(2),
                        )) << (UInt16_2)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_9d1ad_1.get_m31(3),
                        )) & (UInt16_31))
                            << (UInt16_11)));
                let offset1_col4 = offset1_tmp_9d1ad_3.as_m31();
                *row[4] = offset1_col4;
                let offset2_tmp_9d1ad_4 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9d1ad_1.get_m31(3)))
                        >> (UInt16_5))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_9d1ad_1.get_m31(4),
                        )) << (UInt16_4)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_9d1ad_1.get_m31(5),
                        )) & (UInt16_7))
                            << (UInt16_13)));
                let offset2_col5 = offset2_tmp_9d1ad_4.as_m31();
                *row[5] = offset2_col5;
                let dst_base_fp_tmp_9d1ad_5 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9d1ad_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_9d1ad_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_0))
                        & (UInt16_1));
                let dst_base_fp_col6 = dst_base_fp_tmp_9d1ad_5.as_m31();
                *row[6] = dst_base_fp_col6;
                let op0_base_fp_tmp_9d1ad_6 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9d1ad_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_9d1ad_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_1))
                        & (UInt16_1));
                let op0_base_fp_col7 = op0_base_fp_tmp_9d1ad_6.as_m31();
                *row[7] = op0_base_fp_col7;
                let op1_imm_tmp_9d1ad_7 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9d1ad_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_9d1ad_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_2))
                        & (UInt16_1));
                let op1_imm_col8 = op1_imm_tmp_9d1ad_7.as_m31();
                *row[8] = op1_imm_col8;
                let op1_base_fp_tmp_9d1ad_8 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9d1ad_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_9d1ad_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_3))
                        & (UInt16_1));
                let op1_base_fp_col9 = op1_base_fp_tmp_9d1ad_8.as_m31();
                *row[9] = op1_base_fp_col9;
                let op1_base_ap_tmp_9d1ad_9 = (((M31_1) - (op1_imm_col8)) - (op1_base_fp_col9));
                let ap_update_add_1_tmp_9d1ad_10 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9d1ad_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_9d1ad_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_11))
                        & (UInt16_1));
                let ap_update_add_1_col10 = ap_update_add_1_tmp_9d1ad_10.as_m31();
                *row[10] = ap_update_add_1_col10;
                *sub_component_inputs.verify_instruction[0] = (
                    input_pc_col0,
                    [offset0_col3, offset1_col4, offset2_col5],
                    [
                        ((((((dst_base_fp_col6) * (M31_8)) + ((op0_base_fp_col7) * (M31_16)))
                            + ((op1_imm_col8) * (M31_32)))
                            + ((op1_base_fp_col9) * (M31_64)))
                            + ((op1_base_ap_tmp_9d1ad_9) * (M31_128))),
                        (((M31_1) + ((ap_update_add_1_col10) * (M31_32))) + (M31_256)),
                    ],
                    M31_0,
                );
                *lookup_data.verify_instruction_0 = [
                    input_pc_col0,
                    offset0_col3,
                    offset1_col4,
                    offset2_col5,
                    ((((((dst_base_fp_col6) * (M31_8)) + ((op0_base_fp_col7) * (M31_16)))
                        + ((op1_imm_col8) * (M31_32)))
                        + ((op1_base_fp_col9) * (M31_64)))
                        + ((op1_base_ap_tmp_9d1ad_9) * (M31_128))),
                    (((M31_1) + ((ap_update_add_1_col10) * (M31_32))) + (M31_256)),
                    M31_0,
                ];
                let decode_instruction_4b8cf_output_tmp_9d1ad_11 = (
                    [
                        ((offset0_col3) - (M31_32768)),
                        ((offset1_col4) - (M31_32768)),
                        ((offset2_col5) - (M31_32768)),
                    ],
                    [
                        dst_base_fp_col6,
                        op0_base_fp_col7,
                        op1_imm_col8,
                        op1_base_fp_col9,
                        op1_base_ap_tmp_9d1ad_9,
                        M31_0,
                        M31_1,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        ap_update_add_1_col10,
                        M31_0,
                        M31_0,
                        M31_1,
                    ],
                    M31_0,
                );

                let mem_dst_base_col11 = (((dst_base_fp_col6) * (input_fp_col2))
                    + (((M31_1) - (dst_base_fp_col6)) * (input_ap_col1)));
                *row[11] = mem_dst_base_col11;
                let mem0_base_col12 = (((op0_base_fp_col7) * (input_fp_col2))
                    + (((M31_1) - (op0_base_fp_col7)) * (input_ap_col1)));
                *row[12] = mem0_base_col12;
                let mem1_base_col13 = ((((op1_imm_col8) * (input_pc_col0))
                    + ((op1_base_fp_col9) * (input_fp_col2)))
                    + ((decode_instruction_4b8cf_output_tmp_9d1ad_11.1[4]) * (input_ap_col1)));
                *row[13] = mem1_base_col13;

                // Read Positive Num Bits 72.

                // Read Id.

                let memory_address_to_id_value_tmp_9d1ad_12 = memory_address_to_id_state
                    .deduce_output(
                        ((mem_dst_base_col11)
                            + (decode_instruction_4b8cf_output_tmp_9d1ad_11.0[0])),
                    );
                let dst_id_col14 = memory_address_to_id_value_tmp_9d1ad_12;
                *row[14] = dst_id_col14;
                *sub_component_inputs.memory_address_to_id[0] =
                    ((mem_dst_base_col11) + (decode_instruction_4b8cf_output_tmp_9d1ad_11.0[0]));
                *lookup_data.memory_address_to_id_0 = [
                    ((mem_dst_base_col11) + (decode_instruction_4b8cf_output_tmp_9d1ad_11.0[0])),
                    dst_id_col14,
                ];

                // Read Positive Known Id Num Bits 72.

                let memory_id_to_big_value_tmp_9d1ad_14 =
                    memory_id_to_big_state.deduce_output(dst_id_col14);
                let dst_limb_0_col15 = memory_id_to_big_value_tmp_9d1ad_14.get_m31(0);
                *row[15] = dst_limb_0_col15;
                let dst_limb_1_col16 = memory_id_to_big_value_tmp_9d1ad_14.get_m31(1);
                *row[16] = dst_limb_1_col16;
                let dst_limb_2_col17 = memory_id_to_big_value_tmp_9d1ad_14.get_m31(2);
                *row[17] = dst_limb_2_col17;
                let dst_limb_3_col18 = memory_id_to_big_value_tmp_9d1ad_14.get_m31(3);
                *row[18] = dst_limb_3_col18;
                let dst_limb_4_col19 = memory_id_to_big_value_tmp_9d1ad_14.get_m31(4);
                *row[19] = dst_limb_4_col19;
                let dst_limb_5_col20 = memory_id_to_big_value_tmp_9d1ad_14.get_m31(5);
                *row[20] = dst_limb_5_col20;
                let dst_limb_6_col21 = memory_id_to_big_value_tmp_9d1ad_14.get_m31(6);
                *row[21] = dst_limb_6_col21;
                let dst_limb_7_col22 = memory_id_to_big_value_tmp_9d1ad_14.get_m31(7);
                *row[22] = dst_limb_7_col22;
                *sub_component_inputs.memory_id_to_big[0] = dst_id_col14;
                *lookup_data.memory_id_to_big_0 = [
                    dst_id_col14,
                    dst_limb_0_col15,
                    dst_limb_1_col16,
                    dst_limb_2_col17,
                    dst_limb_3_col18,
                    dst_limb_4_col19,
                    dst_limb_5_col20,
                    dst_limb_6_col21,
                    dst_limb_7_col22,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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
                let read_positive_known_id_num_bits_72_output_tmp_9d1ad_15 =
                    PackedFelt252::from_limbs([
                        dst_limb_0_col15,
                        dst_limb_1_col16,
                        dst_limb_2_col17,
                        dst_limb_3_col18,
                        dst_limb_4_col19,
                        dst_limb_5_col20,
                        dst_limb_6_col21,
                        dst_limb_7_col22,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
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

                let read_positive_num_bits_72_output_tmp_9d1ad_16 = (
                    read_positive_known_id_num_bits_72_output_tmp_9d1ad_15,
                    dst_id_col14,
                );

                // Read Positive Num Bits 36.

                // Read Id.

                let memory_address_to_id_value_tmp_9d1ad_17 = memory_address_to_id_state
                    .deduce_output(
                        ((mem0_base_col12) + (decode_instruction_4b8cf_output_tmp_9d1ad_11.0[1])),
                    );
                let op0_id_col23 = memory_address_to_id_value_tmp_9d1ad_17;
                *row[23] = op0_id_col23;
                *sub_component_inputs.memory_address_to_id[1] =
                    ((mem0_base_col12) + (decode_instruction_4b8cf_output_tmp_9d1ad_11.0[1]));
                *lookup_data.memory_address_to_id_1 = [
                    ((mem0_base_col12) + (decode_instruction_4b8cf_output_tmp_9d1ad_11.0[1])),
                    op0_id_col23,
                ];

                // Read Positive Known Id Num Bits 36.

                let memory_id_to_big_value_tmp_9d1ad_19 =
                    memory_id_to_big_state.deduce_output(op0_id_col23);
                let op0_limb_0_col24 = memory_id_to_big_value_tmp_9d1ad_19.get_m31(0);
                *row[24] = op0_limb_0_col24;
                let op0_limb_1_col25 = memory_id_to_big_value_tmp_9d1ad_19.get_m31(1);
                *row[25] = op0_limb_1_col25;
                let op0_limb_2_col26 = memory_id_to_big_value_tmp_9d1ad_19.get_m31(2);
                *row[26] = op0_limb_2_col26;
                let op0_limb_3_col27 = memory_id_to_big_value_tmp_9d1ad_19.get_m31(3);
                *row[27] = op0_limb_3_col27;
                *sub_component_inputs.memory_id_to_big[1] = op0_id_col23;
                *lookup_data.memory_id_to_big_1 = [
                    op0_id_col23,
                    op0_limb_0_col24,
                    op0_limb_1_col25,
                    op0_limb_2_col26,
                    op0_limb_3_col27,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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
                let read_positive_known_id_num_bits_36_output_tmp_9d1ad_20 =
                    PackedFelt252::from_limbs([
                        op0_limb_0_col24,
                        op0_limb_1_col25,
                        op0_limb_2_col26,
                        op0_limb_3_col27,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
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

                let read_positive_num_bits_36_output_tmp_9d1ad_21 = (
                    read_positive_known_id_num_bits_36_output_tmp_9d1ad_20,
                    op0_id_col23,
                );

                // Read Positive Num Bits 36.

                // Read Id.

                let memory_address_to_id_value_tmp_9d1ad_22 = memory_address_to_id_state
                    .deduce_output(
                        ((mem1_base_col13) + (decode_instruction_4b8cf_output_tmp_9d1ad_11.0[2])),
                    );
                let op1_id_col28 = memory_address_to_id_value_tmp_9d1ad_22;
                *row[28] = op1_id_col28;
                *sub_component_inputs.memory_address_to_id[2] =
                    ((mem1_base_col13) + (decode_instruction_4b8cf_output_tmp_9d1ad_11.0[2]));
                *lookup_data.memory_address_to_id_2 = [
                    ((mem1_base_col13) + (decode_instruction_4b8cf_output_tmp_9d1ad_11.0[2])),
                    op1_id_col28,
                ];

                // Read Positive Known Id Num Bits 36.

                let memory_id_to_big_value_tmp_9d1ad_24 =
                    memory_id_to_big_state.deduce_output(op1_id_col28);
                let op1_limb_0_col29 = memory_id_to_big_value_tmp_9d1ad_24.get_m31(0);
                *row[29] = op1_limb_0_col29;
                let op1_limb_1_col30 = memory_id_to_big_value_tmp_9d1ad_24.get_m31(1);
                *row[30] = op1_limb_1_col30;
                let op1_limb_2_col31 = memory_id_to_big_value_tmp_9d1ad_24.get_m31(2);
                *row[31] = op1_limb_2_col31;
                let op1_limb_3_col32 = memory_id_to_big_value_tmp_9d1ad_24.get_m31(3);
                *row[32] = op1_limb_3_col32;
                *sub_component_inputs.memory_id_to_big[2] = op1_id_col28;
                *lookup_data.memory_id_to_big_2 = [
                    op1_id_col28,
                    op1_limb_0_col29,
                    op1_limb_1_col30,
                    op1_limb_2_col31,
                    op1_limb_3_col32,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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
                let read_positive_known_id_num_bits_36_output_tmp_9d1ad_25 =
                    PackedFelt252::from_limbs([
                        op1_limb_0_col29,
                        op1_limb_1_col30,
                        op1_limb_2_col31,
                        op1_limb_3_col32,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
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

                let read_positive_num_bits_36_output_tmp_9d1ad_26 = (
                    read_positive_known_id_num_bits_36_output_tmp_9d1ad_25,
                    op1_id_col28,
                );

                // Verify Mul Small.

                let carry_1_col33 = (((((op0_limb_0_col24) * (op1_limb_0_col29))
                    - (dst_limb_0_col15))
                    + (((((op0_limb_0_col24) * (op1_limb_1_col30))
                        + ((op0_limb_1_col25) * (op1_limb_0_col29)))
                        - (dst_limb_1_col16))
                        * (M31_512)))
                    * (M31_8192));
                *row[33] = carry_1_col33;
                *sub_component_inputs.range_check_11[0] = [carry_1_col33];
                *lookup_data.range_check_11_0 = [carry_1_col33];
                let carry_3_col34 = ((((carry_1_col33)
                    + (((((op0_limb_0_col24) * (op1_limb_2_col31))
                        + ((op0_limb_1_col25) * (op1_limb_1_col30)))
                        + ((op0_limb_2_col26) * (op1_limb_0_col29)))
                        - (dst_limb_2_col17)))
                    + (((((((op0_limb_0_col24) * (op1_limb_3_col32))
                        + ((op0_limb_1_col25) * (op1_limb_2_col31)))
                        + ((op0_limb_2_col26) * (op1_limb_1_col30)))
                        + ((op0_limb_3_col27) * (op1_limb_0_col29)))
                        - (dst_limb_3_col18))
                        * (M31_512)))
                    * (M31_8192));
                *row[34] = carry_3_col34;
                *sub_component_inputs.range_check_11[1] = [carry_3_col34];
                *lookup_data.range_check_11_1 = [carry_3_col34];
                let carry_5_col35 = ((((carry_3_col34)
                    + (((((op0_limb_1_col25) * (op1_limb_3_col32))
                        + ((op0_limb_2_col26) * (op1_limb_2_col31)))
                        + ((op0_limb_3_col27) * (op1_limb_1_col30)))
                        - (dst_limb_4_col19)))
                    + (((((op0_limb_2_col26) * (op1_limb_3_col32))
                        + ((op0_limb_3_col27) * (op1_limb_2_col31)))
                        - (dst_limb_5_col20))
                        * (M31_512)))
                    * (M31_8192));
                *row[35] = carry_5_col35;
                *sub_component_inputs.range_check_11[2] = [carry_5_col35];
                *lookup_data.range_check_11_2 = [carry_5_col35];

                *lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];
                *lookup_data.opcodes_1 = [
                    (((input_pc_col0) + (M31_1)) + (op1_imm_col8)),
                    ((input_ap_col1) + (ap_update_add_1_col10)),
                    input_fp_col2,
                ];
                *row[36] = enabler_col.packed_at(row_index);
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 2]>,
    memory_address_to_id_1: Vec<[PackedM31; 2]>,
    memory_address_to_id_2: Vec<[PackedM31; 2]>,
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
    memory_id_to_big_1: Vec<[PackedM31; 29]>,
    memory_id_to_big_2: Vec<[PackedM31; 29]>,
    opcodes_0: Vec<[PackedM31; 3]>,
    opcodes_1: Vec<[PackedM31; 3]>,
    range_check_11_0: Vec<[PackedM31; 1]>,
    range_check_11_1: Vec<[PackedM31; 1]>,
    range_check_11_2: Vec<[PackedM31; 1]>,
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
        verify_instruction: &relations::VerifyInstruction,
        memory_address_to_id: &relations::MemoryAddressToId,
        memory_id_to_big: &relations::MemoryIdToBig,
        range_check_11: &relations::RangeCheck_11,
        opcodes: &relations::Opcodes,
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
            &self.lookup_data.range_check_11_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = range_check_11.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_11_1,
            &self.lookup_data.range_check_11_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_11.combine(values0);
                let denom1: PackedQM31 = range_check_11.combine(values1);
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
