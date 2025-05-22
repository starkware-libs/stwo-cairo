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
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
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
            memory_address_to_id_state,
            memory_id_to_big_state,
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
    memory_address_to_id: [Vec<PackedRelocatable>; 3],
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
    let M31_134217728 = PackedM31::broadcast(M31::from(134217728));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_16 = PackedM31::broadcast(M31::from(16));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_511 = PackedM31::broadcast(M31::from(511));
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
            |(row_index, (mut row, lookup_data, sub_component_inputs, add_opcode_small_input))| {
                let input_pc_col0 = add_opcode_small_input.pc;
                *row[0] = input_pc_col0;
                let input_ap_col1 = add_opcode_small_input.ap;
                *row[1] = input_ap_col1;
                let input_fp_col2 = add_opcode_small_input.fp;
                *row[2] = input_fp_col2;

                // Decode Instruction.

                let memory_address_to_id_value_tmp_8d553_0 =
                    memory_address_to_id_state.deduce_output(PackedRelocatable::from_pc_m31(input_pc_col0));
                let memory_id_to_big_value_tmp_8d553_1 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_8d553_0);
                let offset0_tmp_8d553_2 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_8d553_1.get_m31(0)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_8d553_1.get_m31(1),
                        )) & (UInt16_127))
                            << (UInt16_9)));
                let offset0_col3 = offset0_tmp_8d553_2.as_m31();
                *row[3] = offset0_col3;
                let offset1_tmp_8d553_3 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_8d553_1.get_m31(1)))
                        >> (UInt16_7))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_8d553_1.get_m31(2),
                        )) << (UInt16_2)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_8d553_1.get_m31(3),
                        )) & (UInt16_31))
                            << (UInt16_11)));
                let offset1_col4 = offset1_tmp_8d553_3.as_m31();
                *row[4] = offset1_col4;
                let offset2_tmp_8d553_4 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_8d553_1.get_m31(3)))
                        >> (UInt16_5))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_8d553_1.get_m31(4),
                        )) << (UInt16_4)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_8d553_1.get_m31(5),
                        )) & (UInt16_7))
                            << (UInt16_13)));
                let offset2_col5 = offset2_tmp_8d553_4.as_m31();
                *row[5] = offset2_col5;
                let dst_base_fp_tmp_8d553_5 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_8d553_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_8d553_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_0))
                        & (UInt16_1));
                let dst_base_fp_col6 = dst_base_fp_tmp_8d553_5.as_m31();
                *row[6] = dst_base_fp_col6;
                let op0_base_fp_tmp_8d553_6 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_8d553_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_8d553_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_1))
                        & (UInt16_1));
                let op0_base_fp_col7 = op0_base_fp_tmp_8d553_6.as_m31();
                *row[7] = op0_base_fp_col7;
                let op1_imm_tmp_8d553_7 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_8d553_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_8d553_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_2))
                        & (UInt16_1));
                let op1_imm_col8 = op1_imm_tmp_8d553_7.as_m31();
                *row[8] = op1_imm_col8;
                let op1_base_fp_tmp_8d553_8 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_8d553_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_8d553_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_3))
                        & (UInt16_1));
                let op1_base_fp_col9 = op1_base_fp_tmp_8d553_8.as_m31();
                *row[9] = op1_base_fp_col9;
                let ap_update_add_1_tmp_8d553_9 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_8d553_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_8d553_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_11))
                        & (UInt16_1));
                let ap_update_add_1_col10 = ap_update_add_1_tmp_8d553_9.as_m31();
                *row[10] = ap_update_add_1_col10;
                *sub_component_inputs.verify_instruction[0] = (
                    input_pc_col0,
                    [offset0_col3, offset1_col4, offset2_col5],
                    [
                        (((((((dst_base_fp_col6) * (M31_8)) + ((op0_base_fp_col7) * (M31_16)))
                            + ((op1_imm_col8) * (M31_32)))
                            + ((op1_base_fp_col9) * (M31_64)))
                            + ((((M31_1) - (op1_imm_col8)) - (op1_base_fp_col9)) * (M31_128)))
                            + (M31_256)),
                        (((ap_update_add_1_col10) * (M31_32)) + (M31_256)),
                    ],
                    M31_0,
                );
                *lookup_data.verify_instruction_0 = [
                    input_pc_col0,
                    offset0_col3,
                    offset1_col4,
                    offset2_col5,
                    (((((((dst_base_fp_col6) * (M31_8)) + ((op0_base_fp_col7) * (M31_16)))
                        + ((op1_imm_col8) * (M31_32)))
                        + ((op1_base_fp_col9) * (M31_64)))
                        + ((((M31_1) - (op1_imm_col8)) - (op1_base_fp_col9)) * (M31_128)))
                        + (M31_256)),
                    (((ap_update_add_1_col10) * (M31_32)) + (M31_256)),
                    M31_0,
                ];
                let decode_instruction_5738317e55ddadf7_output_tmp_8d553_10 = (
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
                        (((M31_1) - (op1_imm_col8)) - (op1_base_fp_col9)),
                        M31_1,
                        M31_0,
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
                    + ((decode_instruction_5738317e55ddadf7_output_tmp_8d553_10.1[4])
                        * (input_ap_col1)));
                *row[13] = mem1_base_col13;

                // Read Small.

                let memory_address_to_id_value_tmp_8d553_11 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((mem_dst_base_col11)
                            + (decode_instruction_5738317e55ddadf7_output_tmp_8d553_10.0[0])),
                    ));
                let memory_id_to_big_value_tmp_8d553_12 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_8d553_11);
                let dst_id_col14 = memory_address_to_id_value_tmp_8d553_11;
                *row[14] = dst_id_col14;
                *sub_component_inputs.memory_address_to_id[0] = PackedRelocatable::from_ap_m31(
                    ((mem_dst_base_col11)
                        + (decode_instruction_5738317e55ddadf7_output_tmp_8d553_10.0[0])),
                );
                *lookup_data.memory_address_to_id_0 = [
                    M31_1,
                    ((mem_dst_base_col11)
                        + (decode_instruction_5738317e55ddadf7_output_tmp_8d553_10.0[0])),
                    dst_id_col14,
                ];

                // Cond Decode Small Sign.

                let msb_tmp_8d553_13 = memory_id_to_big_value_tmp_8d553_12.get_m31(27).eq(M31_256);
                let msb_col15 = msb_tmp_8d553_13.as_m31();
                *row[15] = msb_col15;
                let mid_limbs_set_tmp_8d553_14 =
                    memory_id_to_big_value_tmp_8d553_12.get_m31(20).eq(M31_511);
                let mid_limbs_set_col16 = mid_limbs_set_tmp_8d553_14.as_m31();
                *row[16] = mid_limbs_set_col16;
                let cond_decode_small_sign_output_tmp_8d553_15 = [msb_col15, mid_limbs_set_col16];

                let dst_limb_0_col17 = memory_id_to_big_value_tmp_8d553_12.get_m31(0);
                *row[17] = dst_limb_0_col17;
                let dst_limb_1_col18 = memory_id_to_big_value_tmp_8d553_12.get_m31(1);
                *row[18] = dst_limb_1_col18;
                let dst_limb_2_col19 = memory_id_to_big_value_tmp_8d553_12.get_m31(2);
                *row[19] = dst_limb_2_col19;
                *sub_component_inputs.memory_id_to_big[0] = dst_id_col14;
                *lookup_data.memory_id_to_big_0 = [
                    dst_id_col14,
                    dst_limb_0_col17,
                    dst_limb_1_col18,
                    dst_limb_2_col19,
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    (((M31_136) * (msb_col15)) - (mid_limbs_set_col16)),
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    ((msb_col15) * (M31_256)),
                ];
                let read_small_output_tmp_8d553_16 = (
                    (((((dst_limb_0_col17) + ((dst_limb_1_col18) * (M31_512)))
                        + ((dst_limb_2_col19) * (M31_262144)))
                        - (msb_col15))
                        - ((M31_134217728) * (mid_limbs_set_col16))),
                    dst_id_col14,
                );

                // Read Small.

                let memory_address_to_id_value_tmp_8d553_17 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(
                        ((mem0_base_col12)
                            + (decode_instruction_5738317e55ddadf7_output_tmp_8d553_10.0[1])),
                    ));
                let memory_id_to_big_value_tmp_8d553_18 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_8d553_17);
                let op0_id_col20 = memory_address_to_id_value_tmp_8d553_17;
                *row[20] = op0_id_col20;
                *sub_component_inputs.memory_address_to_id[1] = PackedRelocatable::from_ap_m31(
                    ((mem0_base_col12)
                        + (decode_instruction_5738317e55ddadf7_output_tmp_8d553_10.0[1])),
                );
                *lookup_data.memory_address_to_id_1 = [
                    M31_1,
                    ((mem0_base_col12)
                        + (decode_instruction_5738317e55ddadf7_output_tmp_8d553_10.0[1])),
                    op0_id_col20,
                ];

                // Cond Decode Small Sign.

                let msb_tmp_8d553_19 = memory_id_to_big_value_tmp_8d553_18.get_m31(27).eq(M31_256);
                let msb_col21 = msb_tmp_8d553_19.as_m31();
                *row[21] = msb_col21;
                let mid_limbs_set_tmp_8d553_20 =
                    memory_id_to_big_value_tmp_8d553_18.get_m31(20).eq(M31_511);
                let mid_limbs_set_col22 = mid_limbs_set_tmp_8d553_20.as_m31();
                *row[22] = mid_limbs_set_col22;
                let cond_decode_small_sign_output_tmp_8d553_21 = [msb_col21, mid_limbs_set_col22];

                let op0_limb_0_col23 = memory_id_to_big_value_tmp_8d553_18.get_m31(0);
                *row[23] = op0_limb_0_col23;
                let op0_limb_1_col24 = memory_id_to_big_value_tmp_8d553_18.get_m31(1);
                *row[24] = op0_limb_1_col24;
                let op0_limb_2_col25 = memory_id_to_big_value_tmp_8d553_18.get_m31(2);
                *row[25] = op0_limb_2_col25;
                *sub_component_inputs.memory_id_to_big[1] = op0_id_col20;
                *lookup_data.memory_id_to_big_1 = [
                    op0_id_col20,
                    op0_limb_0_col23,
                    op0_limb_1_col24,
                    op0_limb_2_col25,
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    (((M31_136) * (msb_col21)) - (mid_limbs_set_col22)),
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    ((msb_col21) * (M31_256)),
                ];
                let read_small_output_tmp_8d553_22 = (
                    (((((op0_limb_0_col23) + ((op0_limb_1_col24) * (M31_512)))
                        + ((op0_limb_2_col25) * (M31_262144)))
                        - (msb_col21))
                        - ((M31_134217728) * (mid_limbs_set_col22))),
                    op0_id_col20,
                );

                // Read Small.

                let memory_address_to_id_value_tmp_8d553_23 = memory_address_to_id_state
                    .deduce_output(
                        PackedRelocatable {
                            // If op1_imm_col8 is 1, use segment 0 (PC), otherwise use segment 1 (memory)
                            segment_index: M31_1 * (M31_1 - op1_imm_col8),
                            offset: (mem1_base_col13)
                                + (decode_instruction_5738317e55ddadf7_output_tmp_8d553_10.0[2])
                        }
                    );
                let memory_id_to_big_value_tmp_8d553_24 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_8d553_23);
                let op1_id_col26 = memory_address_to_id_value_tmp_8d553_23;
                *row[26] = op1_id_col26;
                *sub_component_inputs.memory_address_to_id[2] = PackedRelocatable{
                    segment_index: M31_1 * (M31_1 - op1_imm_col8),
                    offset: ((mem1_base_col13)
                        + (decode_instruction_5738317e55ddadf7_output_tmp_8d553_10.0[2])),
                };
                *lookup_data.memory_address_to_id_2 = [
                    M31_1 * (M31_1 - op1_imm_col8),
                    ((mem1_base_col13)
                        + (decode_instruction_5738317e55ddadf7_output_tmp_8d553_10.0[2])),
                    op1_id_col26,
                ];

                // Cond Decode Small Sign.

                let msb_tmp_8d553_25 = memory_id_to_big_value_tmp_8d553_24.get_m31(27).eq(M31_256);
                let msb_col27 = msb_tmp_8d553_25.as_m31();
                *row[27] = msb_col27;
                let mid_limbs_set_tmp_8d553_26 =
                    memory_id_to_big_value_tmp_8d553_24.get_m31(20).eq(M31_511);
                let mid_limbs_set_col28 = mid_limbs_set_tmp_8d553_26.as_m31();
                *row[28] = mid_limbs_set_col28;
                let cond_decode_small_sign_output_tmp_8d553_27 = [msb_col27, mid_limbs_set_col28];

                let op1_limb_0_col29 = memory_id_to_big_value_tmp_8d553_24.get_m31(0);
                *row[29] = op1_limb_0_col29;
                let op1_limb_1_col30 = memory_id_to_big_value_tmp_8d553_24.get_m31(1);
                *row[30] = op1_limb_1_col30;
                let op1_limb_2_col31 = memory_id_to_big_value_tmp_8d553_24.get_m31(2);
                *row[31] = op1_limb_2_col31;
                *sub_component_inputs.memory_id_to_big[2] = op1_id_col26;
                *lookup_data.memory_id_to_big_2 = [
                    op1_id_col26,
                    op1_limb_0_col29,
                    op1_limb_1_col30,
                    op1_limb_2_col31,
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    (((M31_136) * (msb_col27)) - (mid_limbs_set_col28)),
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    ((msb_col27) * (M31_256)),
                ];
                let read_small_output_tmp_8d553_28 = (
                    (((((op1_limb_0_col29) + ((op1_limb_1_col30) * (M31_512)))
                        + ((op1_limb_2_col31) * (M31_262144)))
                        - (msb_col27))
                        - ((M31_134217728) * (mid_limbs_set_col28))),
                    op1_id_col26,
                );

                *lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];
                *lookup_data.opcodes_1 = [
                    (((input_pc_col0) + (M31_1)) + (op1_imm_col8)),
                    ((input_ap_col1) + (ap_update_add_1_col10)),
                    input_fp_col2,
                ];
                *row[32] = enabler_col.packed_at(row_index);
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 3]>,
    memory_address_to_id_1: Vec<[PackedM31; 3]>,
    memory_address_to_id_2: Vec<[PackedM31; 3]>,
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
    memory_id_to_big_1: Vec<[PackedM31; 29]>,
    memory_id_to_big_2: Vec<[PackedM31; 29]>,
    opcodes_0: Vec<[PackedM31; 3]>,
    opcodes_1: Vec<[PackedM31; 3]>,
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
        memory_address_to_id: &relations::MemoryAddressToId,
        memory_id_to_big: &relations::MemoryIdToBig,
        opcodes: &relations::Opcodes,
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
            &self.lookup_data.opcodes_0,
        )
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (writer, values0, values1))| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = opcodes.combine(values1);
                writer.write_frac(denom0 * enabler_col.packed_at(i) + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(), &self.lookup_data.opcodes_1)
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (writer, values))| {
                let denom = opcodes.combine(values);
                writer.write_frac(-PackedQM31::one() * enabler_col.packed_at(i), denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
