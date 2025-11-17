// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::mul_opcode::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{
    memory_address_to_id, memory_id_to_big, range_check_20, range_check_20_b, range_check_20_c,
    range_check_20_d, range_check_20_e, range_check_20_f, range_check_20_g, range_check_20_h,
    verify_instruction,
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
        range_check_20_state: &range_check_20::ClaimGenerator,
        range_check_20_b_state: &range_check_20_b::ClaimGenerator,
        range_check_20_c_state: &range_check_20_c::ClaimGenerator,
        range_check_20_d_state: &range_check_20_d::ClaimGenerator,
        range_check_20_e_state: &range_check_20_e::ClaimGenerator,
        range_check_20_f_state: &range_check_20_f::ClaimGenerator,
        range_check_20_g_state: &range_check_20_g::ClaimGenerator,
        range_check_20_h_state: &range_check_20_h::ClaimGenerator,
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
            range_check_20_state,
            range_check_20_b_state,
            range_check_20_c_state,
            range_check_20_d_state,
            range_check_20_e_state,
            range_check_20_f_state,
            range_check_20_g_state,
            range_check_20_h_state,
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
            .range_check_20
            .iter()
            .for_each(|inputs| {
                range_check_20_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_20_b
            .iter()
            .for_each(|inputs| {
                range_check_20_b_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_20_c
            .iter()
            .for_each(|inputs| {
                range_check_20_c_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_20_d
            .iter()
            .for_each(|inputs| {
                range_check_20_d_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_20_e
            .iter()
            .for_each(|inputs| {
                range_check_20_e_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_20_f
            .iter()
            .for_each(|inputs| {
                range_check_20_f_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_20_g
            .iter()
            .for_each(|inputs| {
                range_check_20_g_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_20_h
            .iter()
            .for_each(|inputs| {
                range_check_20_h_state.add_packed_inputs(inputs);
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
    range_check_20: [Vec<range_check_20::PackedInputType>; 4],
    range_check_20_b: [Vec<range_check_20_b::PackedInputType>; 4],
    range_check_20_c: [Vec<range_check_20_c::PackedInputType>; 4],
    range_check_20_d: [Vec<range_check_20_d::PackedInputType>; 4],
    range_check_20_e: [Vec<range_check_20_e::PackedInputType>; 3],
    range_check_20_f: [Vec<range_check_20_f::PackedInputType>; 3],
    range_check_20_g: [Vec<range_check_20_g::PackedInputType>; 3],
    range_check_20_h: [Vec<range_check_20_h::PackedInputType>; 3],
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
    range_check_20_state: &range_check_20::ClaimGenerator,
    range_check_20_b_state: &range_check_20_b::ClaimGenerator,
    range_check_20_c_state: &range_check_20_c::ClaimGenerator,
    range_check_20_d_state: &range_check_20_d::ClaimGenerator,
    range_check_20_e_state: &range_check_20_e::ClaimGenerator,
    range_check_20_f_state: &range_check_20_f::ClaimGenerator,
    range_check_20_g_state: &range_check_20_g::ClaimGenerator,
    range_check_20_h_state: &range_check_20_h::ClaimGenerator,
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
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_4 = PackedM31::broadcast(M31::from(4));
    let M31_4194304 = PackedM31::broadcast(M31::from(4194304));
    let M31_524288 = PackedM31::broadcast(M31::from(524288));
    let M31_64 = PackedM31::broadcast(M31::from(64));
    let M31_65536 = PackedM31::broadcast(M31::from(65536));
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
    let UInt32_131072 = PackedUInt32::broadcast(UInt32::from(131072));
    let UInt32_262143 = PackedUInt32::broadcast(UInt32::from(262143));
    let UInt32_511 = PackedUInt32::broadcast(UInt32::from(511));
    let UInt32_9 = PackedUInt32::broadcast(UInt32::from(9));
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
            |(row_index, (row, lookup_data, sub_component_inputs, mul_opcode_input))| {
                let input_pc_col0 = mul_opcode_input.pc;
                *row[0] = input_pc_col0;
                let input_ap_col1 = mul_opcode_input.ap;
                *row[1] = input_ap_col1;
                let input_fp_col2 = mul_opcode_input.fp;
                *row[2] = input_fp_col2;

                // Decode Instruction.

                let memory_address_to_id_value_tmp_42314_0 =
                    memory_address_to_id_state.deduce_output(input_pc_col0);
                let memory_id_to_big_value_tmp_42314_1 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_42314_0);
                let offset0_tmp_42314_2 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_1.get_m31(0)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_42314_1.get_m31(1),
                        )) & (UInt16_127))
                            << (UInt16_9)));
                let offset0_col3 = offset0_tmp_42314_2.as_m31();
                *row[3] = offset0_col3;
                let offset1_tmp_42314_3 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_1.get_m31(1)))
                        >> (UInt16_7))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_42314_1.get_m31(2),
                        )) << (UInt16_2)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_42314_1.get_m31(3),
                        )) & (UInt16_31))
                            << (UInt16_11)));
                let offset1_col4 = offset1_tmp_42314_3.as_m31();
                *row[4] = offset1_col4;
                let offset2_tmp_42314_4 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_1.get_m31(3)))
                        >> (UInt16_5))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_42314_1.get_m31(4),
                        )) << (UInt16_4)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_42314_1.get_m31(5),
                        )) & (UInt16_7))
                            << (UInt16_13)));
                let offset2_col5 = offset2_tmp_42314_4.as_m31();
                *row[5] = offset2_col5;
                let dst_base_fp_tmp_42314_5 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_42314_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_0))
                        & (UInt16_1));
                let dst_base_fp_col6 = dst_base_fp_tmp_42314_5.as_m31();
                *row[6] = dst_base_fp_col6;
                let op0_base_fp_tmp_42314_6 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_42314_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_1))
                        & (UInt16_1));
                let op0_base_fp_col7 = op0_base_fp_tmp_42314_6.as_m31();
                *row[7] = op0_base_fp_col7;
                let op1_imm_tmp_42314_7 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_42314_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_2))
                        & (UInt16_1));
                let op1_imm_col8 = op1_imm_tmp_42314_7.as_m31();
                *row[8] = op1_imm_col8;
                let op1_base_fp_tmp_42314_8 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_42314_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_3))
                        & (UInt16_1));
                let op1_base_fp_col9 = op1_base_fp_tmp_42314_8.as_m31();
                *row[9] = op1_base_fp_col9;
                let op1_base_ap_tmp_42314_9 = (((M31_1) - (op1_imm_col8)) - (op1_base_fp_col9));
                let ap_update_add_1_tmp_42314_10 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_42314_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_11))
                        & (UInt16_1));
                let ap_update_add_1_col10 = ap_update_add_1_tmp_42314_10.as_m31();
                *row[10] = ap_update_add_1_col10;
                *sub_component_inputs.verify_instruction[0] = (
                    input_pc_col0,
                    [offset0_col3, offset1_col4, offset2_col5],
                    [
                        ((((((dst_base_fp_col6) * (M31_8)) + ((op0_base_fp_col7) * (M31_16)))
                            + ((op1_imm_col8) * (M31_32)))
                            + ((op1_base_fp_col9) * (M31_64)))
                            + ((op1_base_ap_tmp_42314_9) * (M31_128))),
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
                        + ((op1_base_ap_tmp_42314_9) * (M31_128))),
                    (((M31_1) + ((ap_update_add_1_col10) * (M31_32))) + (M31_256)),
                    M31_0,
                ];
                let decode_instruction_4b8cf_output_tmp_42314_11 = (
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
                        op1_base_ap_tmp_42314_9,
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
                    + ((decode_instruction_4b8cf_output_tmp_42314_11.1[4]) * (input_ap_col1)));
                *row[13] = mem1_base_col13;

                // Read Positive Num Bits 252.

                // Read Id.

                let memory_address_to_id_value_tmp_42314_12 = memory_address_to_id_state
                    .deduce_output(
                        ((mem_dst_base_col11)
                            + (decode_instruction_4b8cf_output_tmp_42314_11.0[0])),
                    );
                let dst_id_col14 = memory_address_to_id_value_tmp_42314_12;
                *row[14] = dst_id_col14;
                *sub_component_inputs.memory_address_to_id[0] =
                    ((mem_dst_base_col11) + (decode_instruction_4b8cf_output_tmp_42314_11.0[0]));
                *lookup_data.memory_address_to_id_0 = [
                    ((mem_dst_base_col11) + (decode_instruction_4b8cf_output_tmp_42314_11.0[0])),
                    dst_id_col14,
                ];

                // Read Positive Known Id Num Bits 252.

                let memory_id_to_big_value_tmp_42314_14 =
                    memory_id_to_big_state.deduce_output(dst_id_col14);
                let dst_limb_0_col15 = memory_id_to_big_value_tmp_42314_14.get_m31(0);
                *row[15] = dst_limb_0_col15;
                let dst_limb_1_col16 = memory_id_to_big_value_tmp_42314_14.get_m31(1);
                *row[16] = dst_limb_1_col16;
                let dst_limb_2_col17 = memory_id_to_big_value_tmp_42314_14.get_m31(2);
                *row[17] = dst_limb_2_col17;
                let dst_limb_3_col18 = memory_id_to_big_value_tmp_42314_14.get_m31(3);
                *row[18] = dst_limb_3_col18;
                let dst_limb_4_col19 = memory_id_to_big_value_tmp_42314_14.get_m31(4);
                *row[19] = dst_limb_4_col19;
                let dst_limb_5_col20 = memory_id_to_big_value_tmp_42314_14.get_m31(5);
                *row[20] = dst_limb_5_col20;
                let dst_limb_6_col21 = memory_id_to_big_value_tmp_42314_14.get_m31(6);
                *row[21] = dst_limb_6_col21;
                let dst_limb_7_col22 = memory_id_to_big_value_tmp_42314_14.get_m31(7);
                *row[22] = dst_limb_7_col22;
                let dst_limb_8_col23 = memory_id_to_big_value_tmp_42314_14.get_m31(8);
                *row[23] = dst_limb_8_col23;
                let dst_limb_9_col24 = memory_id_to_big_value_tmp_42314_14.get_m31(9);
                *row[24] = dst_limb_9_col24;
                let dst_limb_10_col25 = memory_id_to_big_value_tmp_42314_14.get_m31(10);
                *row[25] = dst_limb_10_col25;
                let dst_limb_11_col26 = memory_id_to_big_value_tmp_42314_14.get_m31(11);
                *row[26] = dst_limb_11_col26;
                let dst_limb_12_col27 = memory_id_to_big_value_tmp_42314_14.get_m31(12);
                *row[27] = dst_limb_12_col27;
                let dst_limb_13_col28 = memory_id_to_big_value_tmp_42314_14.get_m31(13);
                *row[28] = dst_limb_13_col28;
                let dst_limb_14_col29 = memory_id_to_big_value_tmp_42314_14.get_m31(14);
                *row[29] = dst_limb_14_col29;
                let dst_limb_15_col30 = memory_id_to_big_value_tmp_42314_14.get_m31(15);
                *row[30] = dst_limb_15_col30;
                let dst_limb_16_col31 = memory_id_to_big_value_tmp_42314_14.get_m31(16);
                *row[31] = dst_limb_16_col31;
                let dst_limb_17_col32 = memory_id_to_big_value_tmp_42314_14.get_m31(17);
                *row[32] = dst_limb_17_col32;
                let dst_limb_18_col33 = memory_id_to_big_value_tmp_42314_14.get_m31(18);
                *row[33] = dst_limb_18_col33;
                let dst_limb_19_col34 = memory_id_to_big_value_tmp_42314_14.get_m31(19);
                *row[34] = dst_limb_19_col34;
                let dst_limb_20_col35 = memory_id_to_big_value_tmp_42314_14.get_m31(20);
                *row[35] = dst_limb_20_col35;
                let dst_limb_21_col36 = memory_id_to_big_value_tmp_42314_14.get_m31(21);
                *row[36] = dst_limb_21_col36;
                let dst_limb_22_col37 = memory_id_to_big_value_tmp_42314_14.get_m31(22);
                *row[37] = dst_limb_22_col37;
                let dst_limb_23_col38 = memory_id_to_big_value_tmp_42314_14.get_m31(23);
                *row[38] = dst_limb_23_col38;
                let dst_limb_24_col39 = memory_id_to_big_value_tmp_42314_14.get_m31(24);
                *row[39] = dst_limb_24_col39;
                let dst_limb_25_col40 = memory_id_to_big_value_tmp_42314_14.get_m31(25);
                *row[40] = dst_limb_25_col40;
                let dst_limb_26_col41 = memory_id_to_big_value_tmp_42314_14.get_m31(26);
                *row[41] = dst_limb_26_col41;
                let dst_limb_27_col42 = memory_id_to_big_value_tmp_42314_14.get_m31(27);
                *row[42] = dst_limb_27_col42;
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
                    dst_limb_8_col23,
                    dst_limb_9_col24,
                    dst_limb_10_col25,
                    dst_limb_11_col26,
                    dst_limb_12_col27,
                    dst_limb_13_col28,
                    dst_limb_14_col29,
                    dst_limb_15_col30,
                    dst_limb_16_col31,
                    dst_limb_17_col32,
                    dst_limb_18_col33,
                    dst_limb_19_col34,
                    dst_limb_20_col35,
                    dst_limb_21_col36,
                    dst_limb_22_col37,
                    dst_limb_23_col38,
                    dst_limb_24_col39,
                    dst_limb_25_col40,
                    dst_limb_26_col41,
                    dst_limb_27_col42,
                ];
                let read_positive_known_id_num_bits_252_output_tmp_42314_15 =
                    PackedFelt252::from_limbs([
                        dst_limb_0_col15,
                        dst_limb_1_col16,
                        dst_limb_2_col17,
                        dst_limb_3_col18,
                        dst_limb_4_col19,
                        dst_limb_5_col20,
                        dst_limb_6_col21,
                        dst_limb_7_col22,
                        dst_limb_8_col23,
                        dst_limb_9_col24,
                        dst_limb_10_col25,
                        dst_limb_11_col26,
                        dst_limb_12_col27,
                        dst_limb_13_col28,
                        dst_limb_14_col29,
                        dst_limb_15_col30,
                        dst_limb_16_col31,
                        dst_limb_17_col32,
                        dst_limb_18_col33,
                        dst_limb_19_col34,
                        dst_limb_20_col35,
                        dst_limb_21_col36,
                        dst_limb_22_col37,
                        dst_limb_23_col38,
                        dst_limb_24_col39,
                        dst_limb_25_col40,
                        dst_limb_26_col41,
                        dst_limb_27_col42,
                    ]);

                let read_positive_num_bits_252_output_tmp_42314_16 = (
                    read_positive_known_id_num_bits_252_output_tmp_42314_15,
                    dst_id_col14,
                );

                // Read Positive Num Bits 252.

                // Read Id.

                let memory_address_to_id_value_tmp_42314_17 = memory_address_to_id_state
                    .deduce_output(
                        ((mem0_base_col12) + (decode_instruction_4b8cf_output_tmp_42314_11.0[1])),
                    );
                let op0_id_col43 = memory_address_to_id_value_tmp_42314_17;
                *row[43] = op0_id_col43;
                *sub_component_inputs.memory_address_to_id[1] =
                    ((mem0_base_col12) + (decode_instruction_4b8cf_output_tmp_42314_11.0[1]));
                *lookup_data.memory_address_to_id_1 = [
                    ((mem0_base_col12) + (decode_instruction_4b8cf_output_tmp_42314_11.0[1])),
                    op0_id_col43,
                ];

                // Read Positive Known Id Num Bits 252.

                let memory_id_to_big_value_tmp_42314_19 =
                    memory_id_to_big_state.deduce_output(op0_id_col43);
                let op0_limb_0_col44 = memory_id_to_big_value_tmp_42314_19.get_m31(0);
                *row[44] = op0_limb_0_col44;
                let op0_limb_1_col45 = memory_id_to_big_value_tmp_42314_19.get_m31(1);
                *row[45] = op0_limb_1_col45;
                let op0_limb_2_col46 = memory_id_to_big_value_tmp_42314_19.get_m31(2);
                *row[46] = op0_limb_2_col46;
                let op0_limb_3_col47 = memory_id_to_big_value_tmp_42314_19.get_m31(3);
                *row[47] = op0_limb_3_col47;
                let op0_limb_4_col48 = memory_id_to_big_value_tmp_42314_19.get_m31(4);
                *row[48] = op0_limb_4_col48;
                let op0_limb_5_col49 = memory_id_to_big_value_tmp_42314_19.get_m31(5);
                *row[49] = op0_limb_5_col49;
                let op0_limb_6_col50 = memory_id_to_big_value_tmp_42314_19.get_m31(6);
                *row[50] = op0_limb_6_col50;
                let op0_limb_7_col51 = memory_id_to_big_value_tmp_42314_19.get_m31(7);
                *row[51] = op0_limb_7_col51;
                let op0_limb_8_col52 = memory_id_to_big_value_tmp_42314_19.get_m31(8);
                *row[52] = op0_limb_8_col52;
                let op0_limb_9_col53 = memory_id_to_big_value_tmp_42314_19.get_m31(9);
                *row[53] = op0_limb_9_col53;
                let op0_limb_10_col54 = memory_id_to_big_value_tmp_42314_19.get_m31(10);
                *row[54] = op0_limb_10_col54;
                let op0_limb_11_col55 = memory_id_to_big_value_tmp_42314_19.get_m31(11);
                *row[55] = op0_limb_11_col55;
                let op0_limb_12_col56 = memory_id_to_big_value_tmp_42314_19.get_m31(12);
                *row[56] = op0_limb_12_col56;
                let op0_limb_13_col57 = memory_id_to_big_value_tmp_42314_19.get_m31(13);
                *row[57] = op0_limb_13_col57;
                let op0_limb_14_col58 = memory_id_to_big_value_tmp_42314_19.get_m31(14);
                *row[58] = op0_limb_14_col58;
                let op0_limb_15_col59 = memory_id_to_big_value_tmp_42314_19.get_m31(15);
                *row[59] = op0_limb_15_col59;
                let op0_limb_16_col60 = memory_id_to_big_value_tmp_42314_19.get_m31(16);
                *row[60] = op0_limb_16_col60;
                let op0_limb_17_col61 = memory_id_to_big_value_tmp_42314_19.get_m31(17);
                *row[61] = op0_limb_17_col61;
                let op0_limb_18_col62 = memory_id_to_big_value_tmp_42314_19.get_m31(18);
                *row[62] = op0_limb_18_col62;
                let op0_limb_19_col63 = memory_id_to_big_value_tmp_42314_19.get_m31(19);
                *row[63] = op0_limb_19_col63;
                let op0_limb_20_col64 = memory_id_to_big_value_tmp_42314_19.get_m31(20);
                *row[64] = op0_limb_20_col64;
                let op0_limb_21_col65 = memory_id_to_big_value_tmp_42314_19.get_m31(21);
                *row[65] = op0_limb_21_col65;
                let op0_limb_22_col66 = memory_id_to_big_value_tmp_42314_19.get_m31(22);
                *row[66] = op0_limb_22_col66;
                let op0_limb_23_col67 = memory_id_to_big_value_tmp_42314_19.get_m31(23);
                *row[67] = op0_limb_23_col67;
                let op0_limb_24_col68 = memory_id_to_big_value_tmp_42314_19.get_m31(24);
                *row[68] = op0_limb_24_col68;
                let op0_limb_25_col69 = memory_id_to_big_value_tmp_42314_19.get_m31(25);
                *row[69] = op0_limb_25_col69;
                let op0_limb_26_col70 = memory_id_to_big_value_tmp_42314_19.get_m31(26);
                *row[70] = op0_limb_26_col70;
                let op0_limb_27_col71 = memory_id_to_big_value_tmp_42314_19.get_m31(27);
                *row[71] = op0_limb_27_col71;
                *sub_component_inputs.memory_id_to_big[1] = op0_id_col43;
                *lookup_data.memory_id_to_big_1 = [
                    op0_id_col43,
                    op0_limb_0_col44,
                    op0_limb_1_col45,
                    op0_limb_2_col46,
                    op0_limb_3_col47,
                    op0_limb_4_col48,
                    op0_limb_5_col49,
                    op0_limb_6_col50,
                    op0_limb_7_col51,
                    op0_limb_8_col52,
                    op0_limb_9_col53,
                    op0_limb_10_col54,
                    op0_limb_11_col55,
                    op0_limb_12_col56,
                    op0_limb_13_col57,
                    op0_limb_14_col58,
                    op0_limb_15_col59,
                    op0_limb_16_col60,
                    op0_limb_17_col61,
                    op0_limb_18_col62,
                    op0_limb_19_col63,
                    op0_limb_20_col64,
                    op0_limb_21_col65,
                    op0_limb_22_col66,
                    op0_limb_23_col67,
                    op0_limb_24_col68,
                    op0_limb_25_col69,
                    op0_limb_26_col70,
                    op0_limb_27_col71,
                ];
                let read_positive_known_id_num_bits_252_output_tmp_42314_20 =
                    PackedFelt252::from_limbs([
                        op0_limb_0_col44,
                        op0_limb_1_col45,
                        op0_limb_2_col46,
                        op0_limb_3_col47,
                        op0_limb_4_col48,
                        op0_limb_5_col49,
                        op0_limb_6_col50,
                        op0_limb_7_col51,
                        op0_limb_8_col52,
                        op0_limb_9_col53,
                        op0_limb_10_col54,
                        op0_limb_11_col55,
                        op0_limb_12_col56,
                        op0_limb_13_col57,
                        op0_limb_14_col58,
                        op0_limb_15_col59,
                        op0_limb_16_col60,
                        op0_limb_17_col61,
                        op0_limb_18_col62,
                        op0_limb_19_col63,
                        op0_limb_20_col64,
                        op0_limb_21_col65,
                        op0_limb_22_col66,
                        op0_limb_23_col67,
                        op0_limb_24_col68,
                        op0_limb_25_col69,
                        op0_limb_26_col70,
                        op0_limb_27_col71,
                    ]);

                let read_positive_num_bits_252_output_tmp_42314_21 = (
                    read_positive_known_id_num_bits_252_output_tmp_42314_20,
                    op0_id_col43,
                );

                // Read Positive Num Bits 252.

                // Read Id.

                let memory_address_to_id_value_tmp_42314_22 = memory_address_to_id_state
                    .deduce_output(
                        ((mem1_base_col13) + (decode_instruction_4b8cf_output_tmp_42314_11.0[2])),
                    );
                let op1_id_col72 = memory_address_to_id_value_tmp_42314_22;
                *row[72] = op1_id_col72;
                *sub_component_inputs.memory_address_to_id[2] =
                    ((mem1_base_col13) + (decode_instruction_4b8cf_output_tmp_42314_11.0[2]));
                *lookup_data.memory_address_to_id_2 = [
                    ((mem1_base_col13) + (decode_instruction_4b8cf_output_tmp_42314_11.0[2])),
                    op1_id_col72,
                ];

                // Read Positive Known Id Num Bits 252.

                let memory_id_to_big_value_tmp_42314_24 =
                    memory_id_to_big_state.deduce_output(op1_id_col72);
                let op1_limb_0_col73 = memory_id_to_big_value_tmp_42314_24.get_m31(0);
                *row[73] = op1_limb_0_col73;
                let op1_limb_1_col74 = memory_id_to_big_value_tmp_42314_24.get_m31(1);
                *row[74] = op1_limb_1_col74;
                let op1_limb_2_col75 = memory_id_to_big_value_tmp_42314_24.get_m31(2);
                *row[75] = op1_limb_2_col75;
                let op1_limb_3_col76 = memory_id_to_big_value_tmp_42314_24.get_m31(3);
                *row[76] = op1_limb_3_col76;
                let op1_limb_4_col77 = memory_id_to_big_value_tmp_42314_24.get_m31(4);
                *row[77] = op1_limb_4_col77;
                let op1_limb_5_col78 = memory_id_to_big_value_tmp_42314_24.get_m31(5);
                *row[78] = op1_limb_5_col78;
                let op1_limb_6_col79 = memory_id_to_big_value_tmp_42314_24.get_m31(6);
                *row[79] = op1_limb_6_col79;
                let op1_limb_7_col80 = memory_id_to_big_value_tmp_42314_24.get_m31(7);
                *row[80] = op1_limb_7_col80;
                let op1_limb_8_col81 = memory_id_to_big_value_tmp_42314_24.get_m31(8);
                *row[81] = op1_limb_8_col81;
                let op1_limb_9_col82 = memory_id_to_big_value_tmp_42314_24.get_m31(9);
                *row[82] = op1_limb_9_col82;
                let op1_limb_10_col83 = memory_id_to_big_value_tmp_42314_24.get_m31(10);
                *row[83] = op1_limb_10_col83;
                let op1_limb_11_col84 = memory_id_to_big_value_tmp_42314_24.get_m31(11);
                *row[84] = op1_limb_11_col84;
                let op1_limb_12_col85 = memory_id_to_big_value_tmp_42314_24.get_m31(12);
                *row[85] = op1_limb_12_col85;
                let op1_limb_13_col86 = memory_id_to_big_value_tmp_42314_24.get_m31(13);
                *row[86] = op1_limb_13_col86;
                let op1_limb_14_col87 = memory_id_to_big_value_tmp_42314_24.get_m31(14);
                *row[87] = op1_limb_14_col87;
                let op1_limb_15_col88 = memory_id_to_big_value_tmp_42314_24.get_m31(15);
                *row[88] = op1_limb_15_col88;
                let op1_limb_16_col89 = memory_id_to_big_value_tmp_42314_24.get_m31(16);
                *row[89] = op1_limb_16_col89;
                let op1_limb_17_col90 = memory_id_to_big_value_tmp_42314_24.get_m31(17);
                *row[90] = op1_limb_17_col90;
                let op1_limb_18_col91 = memory_id_to_big_value_tmp_42314_24.get_m31(18);
                *row[91] = op1_limb_18_col91;
                let op1_limb_19_col92 = memory_id_to_big_value_tmp_42314_24.get_m31(19);
                *row[92] = op1_limb_19_col92;
                let op1_limb_20_col93 = memory_id_to_big_value_tmp_42314_24.get_m31(20);
                *row[93] = op1_limb_20_col93;
                let op1_limb_21_col94 = memory_id_to_big_value_tmp_42314_24.get_m31(21);
                *row[94] = op1_limb_21_col94;
                let op1_limb_22_col95 = memory_id_to_big_value_tmp_42314_24.get_m31(22);
                *row[95] = op1_limb_22_col95;
                let op1_limb_23_col96 = memory_id_to_big_value_tmp_42314_24.get_m31(23);
                *row[96] = op1_limb_23_col96;
                let op1_limb_24_col97 = memory_id_to_big_value_tmp_42314_24.get_m31(24);
                *row[97] = op1_limb_24_col97;
                let op1_limb_25_col98 = memory_id_to_big_value_tmp_42314_24.get_m31(25);
                *row[98] = op1_limb_25_col98;
                let op1_limb_26_col99 = memory_id_to_big_value_tmp_42314_24.get_m31(26);
                *row[99] = op1_limb_26_col99;
                let op1_limb_27_col100 = memory_id_to_big_value_tmp_42314_24.get_m31(27);
                *row[100] = op1_limb_27_col100;
                *sub_component_inputs.memory_id_to_big[2] = op1_id_col72;
                *lookup_data.memory_id_to_big_2 = [
                    op1_id_col72,
                    op1_limb_0_col73,
                    op1_limb_1_col74,
                    op1_limb_2_col75,
                    op1_limb_3_col76,
                    op1_limb_4_col77,
                    op1_limb_5_col78,
                    op1_limb_6_col79,
                    op1_limb_7_col80,
                    op1_limb_8_col81,
                    op1_limb_9_col82,
                    op1_limb_10_col83,
                    op1_limb_11_col84,
                    op1_limb_12_col85,
                    op1_limb_13_col86,
                    op1_limb_14_col87,
                    op1_limb_15_col88,
                    op1_limb_16_col89,
                    op1_limb_17_col90,
                    op1_limb_18_col91,
                    op1_limb_19_col92,
                    op1_limb_20_col93,
                    op1_limb_21_col94,
                    op1_limb_22_col95,
                    op1_limb_23_col96,
                    op1_limb_24_col97,
                    op1_limb_25_col98,
                    op1_limb_26_col99,
                    op1_limb_27_col100,
                ];
                let read_positive_known_id_num_bits_252_output_tmp_42314_25 =
                    PackedFelt252::from_limbs([
                        op1_limb_0_col73,
                        op1_limb_1_col74,
                        op1_limb_2_col75,
                        op1_limb_3_col76,
                        op1_limb_4_col77,
                        op1_limb_5_col78,
                        op1_limb_6_col79,
                        op1_limb_7_col80,
                        op1_limb_8_col81,
                        op1_limb_9_col82,
                        op1_limb_10_col83,
                        op1_limb_11_col84,
                        op1_limb_12_col85,
                        op1_limb_13_col86,
                        op1_limb_14_col87,
                        op1_limb_15_col88,
                        op1_limb_16_col89,
                        op1_limb_17_col90,
                        op1_limb_18_col91,
                        op1_limb_19_col92,
                        op1_limb_20_col93,
                        op1_limb_21_col94,
                        op1_limb_22_col95,
                        op1_limb_23_col96,
                        op1_limb_24_col97,
                        op1_limb_25_col98,
                        op1_limb_26_col99,
                        op1_limb_27_col100,
                    ]);

                let read_positive_num_bits_252_output_tmp_42314_26 = (
                    read_positive_known_id_num_bits_252_output_tmp_42314_25,
                    op1_id_col72,
                );

                // Verify Mul 252.

                // Double Karatsuba 1454 B.

                // Single Karatsuba N 7.

                let z0_tmp_42314_27 = [
                    ((op0_limb_0_col44) * (op1_limb_0_col73)),
                    (((op0_limb_0_col44) * (op1_limb_1_col74))
                        + ((op0_limb_1_col45) * (op1_limb_0_col73))),
                    ((((op0_limb_0_col44) * (op1_limb_2_col75))
                        + ((op0_limb_1_col45) * (op1_limb_1_col74)))
                        + ((op0_limb_2_col46) * (op1_limb_0_col73))),
                    (((((op0_limb_0_col44) * (op1_limb_3_col76))
                        + ((op0_limb_1_col45) * (op1_limb_2_col75)))
                        + ((op0_limb_2_col46) * (op1_limb_1_col74)))
                        + ((op0_limb_3_col47) * (op1_limb_0_col73))),
                    ((((((op0_limb_0_col44) * (op1_limb_4_col77))
                        + ((op0_limb_1_col45) * (op1_limb_3_col76)))
                        + ((op0_limb_2_col46) * (op1_limb_2_col75)))
                        + ((op0_limb_3_col47) * (op1_limb_1_col74)))
                        + ((op0_limb_4_col48) * (op1_limb_0_col73))),
                    (((((((op0_limb_0_col44) * (op1_limb_5_col78))
                        + ((op0_limb_1_col45) * (op1_limb_4_col77)))
                        + ((op0_limb_2_col46) * (op1_limb_3_col76)))
                        + ((op0_limb_3_col47) * (op1_limb_2_col75)))
                        + ((op0_limb_4_col48) * (op1_limb_1_col74)))
                        + ((op0_limb_5_col49) * (op1_limb_0_col73))),
                    ((((((((op0_limb_0_col44) * (op1_limb_6_col79))
                        + ((op0_limb_1_col45) * (op1_limb_5_col78)))
                        + ((op0_limb_2_col46) * (op1_limb_4_col77)))
                        + ((op0_limb_3_col47) * (op1_limb_3_col76)))
                        + ((op0_limb_4_col48) * (op1_limb_2_col75)))
                        + ((op0_limb_5_col49) * (op1_limb_1_col74)))
                        + ((op0_limb_6_col50) * (op1_limb_0_col73))),
                    (((((((op0_limb_1_col45) * (op1_limb_6_col79))
                        + ((op0_limb_2_col46) * (op1_limb_5_col78)))
                        + ((op0_limb_3_col47) * (op1_limb_4_col77)))
                        + ((op0_limb_4_col48) * (op1_limb_3_col76)))
                        + ((op0_limb_5_col49) * (op1_limb_2_col75)))
                        + ((op0_limb_6_col50) * (op1_limb_1_col74))),
                    ((((((op0_limb_2_col46) * (op1_limb_6_col79))
                        + ((op0_limb_3_col47) * (op1_limb_5_col78)))
                        + ((op0_limb_4_col48) * (op1_limb_4_col77)))
                        + ((op0_limb_5_col49) * (op1_limb_3_col76)))
                        + ((op0_limb_6_col50) * (op1_limb_2_col75))),
                    (((((op0_limb_3_col47) * (op1_limb_6_col79))
                        + ((op0_limb_4_col48) * (op1_limb_5_col78)))
                        + ((op0_limb_5_col49) * (op1_limb_4_col77)))
                        + ((op0_limb_6_col50) * (op1_limb_3_col76))),
                    ((((op0_limb_4_col48) * (op1_limb_6_col79))
                        + ((op0_limb_5_col49) * (op1_limb_5_col78)))
                        + ((op0_limb_6_col50) * (op1_limb_4_col77))),
                    (((op0_limb_5_col49) * (op1_limb_6_col79))
                        + ((op0_limb_6_col50) * (op1_limb_5_col78))),
                    ((op0_limb_6_col50) * (op1_limb_6_col79)),
                ];
                let z2_tmp_42314_28 = [
                    ((op0_limb_7_col51) * (op1_limb_7_col80)),
                    (((op0_limb_7_col51) * (op1_limb_8_col81))
                        + ((op0_limb_8_col52) * (op1_limb_7_col80))),
                    ((((op0_limb_7_col51) * (op1_limb_9_col82))
                        + ((op0_limb_8_col52) * (op1_limb_8_col81)))
                        + ((op0_limb_9_col53) * (op1_limb_7_col80))),
                    (((((op0_limb_7_col51) * (op1_limb_10_col83))
                        + ((op0_limb_8_col52) * (op1_limb_9_col82)))
                        + ((op0_limb_9_col53) * (op1_limb_8_col81)))
                        + ((op0_limb_10_col54) * (op1_limb_7_col80))),
                    ((((((op0_limb_7_col51) * (op1_limb_11_col84))
                        + ((op0_limb_8_col52) * (op1_limb_10_col83)))
                        + ((op0_limb_9_col53) * (op1_limb_9_col82)))
                        + ((op0_limb_10_col54) * (op1_limb_8_col81)))
                        + ((op0_limb_11_col55) * (op1_limb_7_col80))),
                    (((((((op0_limb_7_col51) * (op1_limb_12_col85))
                        + ((op0_limb_8_col52) * (op1_limb_11_col84)))
                        + ((op0_limb_9_col53) * (op1_limb_10_col83)))
                        + ((op0_limb_10_col54) * (op1_limb_9_col82)))
                        + ((op0_limb_11_col55) * (op1_limb_8_col81)))
                        + ((op0_limb_12_col56) * (op1_limb_7_col80))),
                    ((((((((op0_limb_7_col51) * (op1_limb_13_col86))
                        + ((op0_limb_8_col52) * (op1_limb_12_col85)))
                        + ((op0_limb_9_col53) * (op1_limb_11_col84)))
                        + ((op0_limb_10_col54) * (op1_limb_10_col83)))
                        + ((op0_limb_11_col55) * (op1_limb_9_col82)))
                        + ((op0_limb_12_col56) * (op1_limb_8_col81)))
                        + ((op0_limb_13_col57) * (op1_limb_7_col80))),
                    (((((((op0_limb_8_col52) * (op1_limb_13_col86))
                        + ((op0_limb_9_col53) * (op1_limb_12_col85)))
                        + ((op0_limb_10_col54) * (op1_limb_11_col84)))
                        + ((op0_limb_11_col55) * (op1_limb_10_col83)))
                        + ((op0_limb_12_col56) * (op1_limb_9_col82)))
                        + ((op0_limb_13_col57) * (op1_limb_8_col81))),
                    ((((((op0_limb_9_col53) * (op1_limb_13_col86))
                        + ((op0_limb_10_col54) * (op1_limb_12_col85)))
                        + ((op0_limb_11_col55) * (op1_limb_11_col84)))
                        + ((op0_limb_12_col56) * (op1_limb_10_col83)))
                        + ((op0_limb_13_col57) * (op1_limb_9_col82))),
                    (((((op0_limb_10_col54) * (op1_limb_13_col86))
                        + ((op0_limb_11_col55) * (op1_limb_12_col85)))
                        + ((op0_limb_12_col56) * (op1_limb_11_col84)))
                        + ((op0_limb_13_col57) * (op1_limb_10_col83))),
                    ((((op0_limb_11_col55) * (op1_limb_13_col86))
                        + ((op0_limb_12_col56) * (op1_limb_12_col85)))
                        + ((op0_limb_13_col57) * (op1_limb_11_col84))),
                    (((op0_limb_12_col56) * (op1_limb_13_col86))
                        + ((op0_limb_13_col57) * (op1_limb_12_col85))),
                    ((op0_limb_13_col57) * (op1_limb_13_col86)),
                ];
                let x_sum_tmp_42314_29 = [
                    ((op0_limb_0_col44) + (op0_limb_7_col51)),
                    ((op0_limb_1_col45) + (op0_limb_8_col52)),
                    ((op0_limb_2_col46) + (op0_limb_9_col53)),
                    ((op0_limb_3_col47) + (op0_limb_10_col54)),
                    ((op0_limb_4_col48) + (op0_limb_11_col55)),
                    ((op0_limb_5_col49) + (op0_limb_12_col56)),
                    ((op0_limb_6_col50) + (op0_limb_13_col57)),
                ];
                let y_sum_tmp_42314_30 = [
                    ((op1_limb_0_col73) + (op1_limb_7_col80)),
                    ((op1_limb_1_col74) + (op1_limb_8_col81)),
                    ((op1_limb_2_col75) + (op1_limb_9_col82)),
                    ((op1_limb_3_col76) + (op1_limb_10_col83)),
                    ((op1_limb_4_col77) + (op1_limb_11_col84)),
                    ((op1_limb_5_col78) + (op1_limb_12_col85)),
                    ((op1_limb_6_col79) + (op1_limb_13_col86)),
                ];
                let single_karatsuba_n_7_output_tmp_42314_31 = [
                    z0_tmp_42314_27[0],
                    z0_tmp_42314_27[1],
                    z0_tmp_42314_27[2],
                    z0_tmp_42314_27[3],
                    z0_tmp_42314_27[4],
                    z0_tmp_42314_27[5],
                    z0_tmp_42314_27[6],
                    ((z0_tmp_42314_27[7])
                        + ((((x_sum_tmp_42314_29[0]) * (y_sum_tmp_42314_30[0]))
                            - (z0_tmp_42314_27[0]))
                            - (z2_tmp_42314_28[0]))),
                    ((z0_tmp_42314_27[8])
                        + (((((x_sum_tmp_42314_29[0]) * (y_sum_tmp_42314_30[1]))
                            + ((x_sum_tmp_42314_29[1]) * (y_sum_tmp_42314_30[0])))
                            - (z0_tmp_42314_27[1]))
                            - (z2_tmp_42314_28[1]))),
                    ((z0_tmp_42314_27[9])
                        + ((((((x_sum_tmp_42314_29[0]) * (y_sum_tmp_42314_30[2]))
                            + ((x_sum_tmp_42314_29[1]) * (y_sum_tmp_42314_30[1])))
                            + ((x_sum_tmp_42314_29[2]) * (y_sum_tmp_42314_30[0])))
                            - (z0_tmp_42314_27[2]))
                            - (z2_tmp_42314_28[2]))),
                    ((z0_tmp_42314_27[10])
                        + (((((((x_sum_tmp_42314_29[0]) * (y_sum_tmp_42314_30[3]))
                            + ((x_sum_tmp_42314_29[1]) * (y_sum_tmp_42314_30[2])))
                            + ((x_sum_tmp_42314_29[2]) * (y_sum_tmp_42314_30[1])))
                            + ((x_sum_tmp_42314_29[3]) * (y_sum_tmp_42314_30[0])))
                            - (z0_tmp_42314_27[3]))
                            - (z2_tmp_42314_28[3]))),
                    ((z0_tmp_42314_27[11])
                        + ((((((((x_sum_tmp_42314_29[0]) * (y_sum_tmp_42314_30[4]))
                            + ((x_sum_tmp_42314_29[1]) * (y_sum_tmp_42314_30[3])))
                            + ((x_sum_tmp_42314_29[2]) * (y_sum_tmp_42314_30[2])))
                            + ((x_sum_tmp_42314_29[3]) * (y_sum_tmp_42314_30[1])))
                            + ((x_sum_tmp_42314_29[4]) * (y_sum_tmp_42314_30[0])))
                            - (z0_tmp_42314_27[4]))
                            - (z2_tmp_42314_28[4]))),
                    ((z0_tmp_42314_27[12])
                        + (((((((((x_sum_tmp_42314_29[0]) * (y_sum_tmp_42314_30[5]))
                            + ((x_sum_tmp_42314_29[1]) * (y_sum_tmp_42314_30[4])))
                            + ((x_sum_tmp_42314_29[2]) * (y_sum_tmp_42314_30[3])))
                            + ((x_sum_tmp_42314_29[3]) * (y_sum_tmp_42314_30[2])))
                            + ((x_sum_tmp_42314_29[4]) * (y_sum_tmp_42314_30[1])))
                            + ((x_sum_tmp_42314_29[5]) * (y_sum_tmp_42314_30[0])))
                            - (z0_tmp_42314_27[5]))
                            - (z2_tmp_42314_28[5]))),
                    ((((((((((x_sum_tmp_42314_29[0]) * (y_sum_tmp_42314_30[6]))
                        + ((x_sum_tmp_42314_29[1]) * (y_sum_tmp_42314_30[5])))
                        + ((x_sum_tmp_42314_29[2]) * (y_sum_tmp_42314_30[4])))
                        + ((x_sum_tmp_42314_29[3]) * (y_sum_tmp_42314_30[3])))
                        + ((x_sum_tmp_42314_29[4]) * (y_sum_tmp_42314_30[2])))
                        + ((x_sum_tmp_42314_29[5]) * (y_sum_tmp_42314_30[1])))
                        + ((x_sum_tmp_42314_29[6]) * (y_sum_tmp_42314_30[0])))
                        - (z0_tmp_42314_27[6]))
                        - (z2_tmp_42314_28[6])),
                    ((z2_tmp_42314_28[0])
                        + (((((((((x_sum_tmp_42314_29[1]) * (y_sum_tmp_42314_30[6]))
                            + ((x_sum_tmp_42314_29[2]) * (y_sum_tmp_42314_30[5])))
                            + ((x_sum_tmp_42314_29[3]) * (y_sum_tmp_42314_30[4])))
                            + ((x_sum_tmp_42314_29[4]) * (y_sum_tmp_42314_30[3])))
                            + ((x_sum_tmp_42314_29[5]) * (y_sum_tmp_42314_30[2])))
                            + ((x_sum_tmp_42314_29[6]) * (y_sum_tmp_42314_30[1])))
                            - (z0_tmp_42314_27[7]))
                            - (z2_tmp_42314_28[7]))),
                    ((z2_tmp_42314_28[1])
                        + ((((((((x_sum_tmp_42314_29[2]) * (y_sum_tmp_42314_30[6]))
                            + ((x_sum_tmp_42314_29[3]) * (y_sum_tmp_42314_30[5])))
                            + ((x_sum_tmp_42314_29[4]) * (y_sum_tmp_42314_30[4])))
                            + ((x_sum_tmp_42314_29[5]) * (y_sum_tmp_42314_30[3])))
                            + ((x_sum_tmp_42314_29[6]) * (y_sum_tmp_42314_30[2])))
                            - (z0_tmp_42314_27[8]))
                            - (z2_tmp_42314_28[8]))),
                    ((z2_tmp_42314_28[2])
                        + (((((((x_sum_tmp_42314_29[3]) * (y_sum_tmp_42314_30[6]))
                            + ((x_sum_tmp_42314_29[4]) * (y_sum_tmp_42314_30[5])))
                            + ((x_sum_tmp_42314_29[5]) * (y_sum_tmp_42314_30[4])))
                            + ((x_sum_tmp_42314_29[6]) * (y_sum_tmp_42314_30[3])))
                            - (z0_tmp_42314_27[9]))
                            - (z2_tmp_42314_28[9]))),
                    ((z2_tmp_42314_28[3])
                        + ((((((x_sum_tmp_42314_29[4]) * (y_sum_tmp_42314_30[6]))
                            + ((x_sum_tmp_42314_29[5]) * (y_sum_tmp_42314_30[5])))
                            + ((x_sum_tmp_42314_29[6]) * (y_sum_tmp_42314_30[4])))
                            - (z0_tmp_42314_27[10]))
                            - (z2_tmp_42314_28[10]))),
                    ((z2_tmp_42314_28[4])
                        + (((((x_sum_tmp_42314_29[5]) * (y_sum_tmp_42314_30[6]))
                            + ((x_sum_tmp_42314_29[6]) * (y_sum_tmp_42314_30[5])))
                            - (z0_tmp_42314_27[11]))
                            - (z2_tmp_42314_28[11]))),
                    ((z2_tmp_42314_28[5])
                        + ((((x_sum_tmp_42314_29[6]) * (y_sum_tmp_42314_30[6]))
                            - (z0_tmp_42314_27[12]))
                            - (z2_tmp_42314_28[12]))),
                    z2_tmp_42314_28[6],
                    z2_tmp_42314_28[7],
                    z2_tmp_42314_28[8],
                    z2_tmp_42314_28[9],
                    z2_tmp_42314_28[10],
                    z2_tmp_42314_28[11],
                    z2_tmp_42314_28[12],
                ];

                // Single Karatsuba N 7.

                let z0_tmp_42314_32 = [
                    ((op0_limb_14_col58) * (op1_limb_14_col87)),
                    (((op0_limb_14_col58) * (op1_limb_15_col88))
                        + ((op0_limb_15_col59) * (op1_limb_14_col87))),
                    ((((op0_limb_14_col58) * (op1_limb_16_col89))
                        + ((op0_limb_15_col59) * (op1_limb_15_col88)))
                        + ((op0_limb_16_col60) * (op1_limb_14_col87))),
                    (((((op0_limb_14_col58) * (op1_limb_17_col90))
                        + ((op0_limb_15_col59) * (op1_limb_16_col89)))
                        + ((op0_limb_16_col60) * (op1_limb_15_col88)))
                        + ((op0_limb_17_col61) * (op1_limb_14_col87))),
                    ((((((op0_limb_14_col58) * (op1_limb_18_col91))
                        + ((op0_limb_15_col59) * (op1_limb_17_col90)))
                        + ((op0_limb_16_col60) * (op1_limb_16_col89)))
                        + ((op0_limb_17_col61) * (op1_limb_15_col88)))
                        + ((op0_limb_18_col62) * (op1_limb_14_col87))),
                    (((((((op0_limb_14_col58) * (op1_limb_19_col92))
                        + ((op0_limb_15_col59) * (op1_limb_18_col91)))
                        + ((op0_limb_16_col60) * (op1_limb_17_col90)))
                        + ((op0_limb_17_col61) * (op1_limb_16_col89)))
                        + ((op0_limb_18_col62) * (op1_limb_15_col88)))
                        + ((op0_limb_19_col63) * (op1_limb_14_col87))),
                    ((((((((op0_limb_14_col58) * (op1_limb_20_col93))
                        + ((op0_limb_15_col59) * (op1_limb_19_col92)))
                        + ((op0_limb_16_col60) * (op1_limb_18_col91)))
                        + ((op0_limb_17_col61) * (op1_limb_17_col90)))
                        + ((op0_limb_18_col62) * (op1_limb_16_col89)))
                        + ((op0_limb_19_col63) * (op1_limb_15_col88)))
                        + ((op0_limb_20_col64) * (op1_limb_14_col87))),
                    (((((((op0_limb_15_col59) * (op1_limb_20_col93))
                        + ((op0_limb_16_col60) * (op1_limb_19_col92)))
                        + ((op0_limb_17_col61) * (op1_limb_18_col91)))
                        + ((op0_limb_18_col62) * (op1_limb_17_col90)))
                        + ((op0_limb_19_col63) * (op1_limb_16_col89)))
                        + ((op0_limb_20_col64) * (op1_limb_15_col88))),
                    ((((((op0_limb_16_col60) * (op1_limb_20_col93))
                        + ((op0_limb_17_col61) * (op1_limb_19_col92)))
                        + ((op0_limb_18_col62) * (op1_limb_18_col91)))
                        + ((op0_limb_19_col63) * (op1_limb_17_col90)))
                        + ((op0_limb_20_col64) * (op1_limb_16_col89))),
                    (((((op0_limb_17_col61) * (op1_limb_20_col93))
                        + ((op0_limb_18_col62) * (op1_limb_19_col92)))
                        + ((op0_limb_19_col63) * (op1_limb_18_col91)))
                        + ((op0_limb_20_col64) * (op1_limb_17_col90))),
                    ((((op0_limb_18_col62) * (op1_limb_20_col93))
                        + ((op0_limb_19_col63) * (op1_limb_19_col92)))
                        + ((op0_limb_20_col64) * (op1_limb_18_col91))),
                    (((op0_limb_19_col63) * (op1_limb_20_col93))
                        + ((op0_limb_20_col64) * (op1_limb_19_col92))),
                    ((op0_limb_20_col64) * (op1_limb_20_col93)),
                ];
                let z2_tmp_42314_33 = [
                    ((op0_limb_21_col65) * (op1_limb_21_col94)),
                    (((op0_limb_21_col65) * (op1_limb_22_col95))
                        + ((op0_limb_22_col66) * (op1_limb_21_col94))),
                    ((((op0_limb_21_col65) * (op1_limb_23_col96))
                        + ((op0_limb_22_col66) * (op1_limb_22_col95)))
                        + ((op0_limb_23_col67) * (op1_limb_21_col94))),
                    (((((op0_limb_21_col65) * (op1_limb_24_col97))
                        + ((op0_limb_22_col66) * (op1_limb_23_col96)))
                        + ((op0_limb_23_col67) * (op1_limb_22_col95)))
                        + ((op0_limb_24_col68) * (op1_limb_21_col94))),
                    ((((((op0_limb_21_col65) * (op1_limb_25_col98))
                        + ((op0_limb_22_col66) * (op1_limb_24_col97)))
                        + ((op0_limb_23_col67) * (op1_limb_23_col96)))
                        + ((op0_limb_24_col68) * (op1_limb_22_col95)))
                        + ((op0_limb_25_col69) * (op1_limb_21_col94))),
                    (((((((op0_limb_21_col65) * (op1_limb_26_col99))
                        + ((op0_limb_22_col66) * (op1_limb_25_col98)))
                        + ((op0_limb_23_col67) * (op1_limb_24_col97)))
                        + ((op0_limb_24_col68) * (op1_limb_23_col96)))
                        + ((op0_limb_25_col69) * (op1_limb_22_col95)))
                        + ((op0_limb_26_col70) * (op1_limb_21_col94))),
                    ((((((((op0_limb_21_col65) * (op1_limb_27_col100))
                        + ((op0_limb_22_col66) * (op1_limb_26_col99)))
                        + ((op0_limb_23_col67) * (op1_limb_25_col98)))
                        + ((op0_limb_24_col68) * (op1_limb_24_col97)))
                        + ((op0_limb_25_col69) * (op1_limb_23_col96)))
                        + ((op0_limb_26_col70) * (op1_limb_22_col95)))
                        + ((op0_limb_27_col71) * (op1_limb_21_col94))),
                    (((((((op0_limb_22_col66) * (op1_limb_27_col100))
                        + ((op0_limb_23_col67) * (op1_limb_26_col99)))
                        + ((op0_limb_24_col68) * (op1_limb_25_col98)))
                        + ((op0_limb_25_col69) * (op1_limb_24_col97)))
                        + ((op0_limb_26_col70) * (op1_limb_23_col96)))
                        + ((op0_limb_27_col71) * (op1_limb_22_col95))),
                    ((((((op0_limb_23_col67) * (op1_limb_27_col100))
                        + ((op0_limb_24_col68) * (op1_limb_26_col99)))
                        + ((op0_limb_25_col69) * (op1_limb_25_col98)))
                        + ((op0_limb_26_col70) * (op1_limb_24_col97)))
                        + ((op0_limb_27_col71) * (op1_limb_23_col96))),
                    (((((op0_limb_24_col68) * (op1_limb_27_col100))
                        + ((op0_limb_25_col69) * (op1_limb_26_col99)))
                        + ((op0_limb_26_col70) * (op1_limb_25_col98)))
                        + ((op0_limb_27_col71) * (op1_limb_24_col97))),
                    ((((op0_limb_25_col69) * (op1_limb_27_col100))
                        + ((op0_limb_26_col70) * (op1_limb_26_col99)))
                        + ((op0_limb_27_col71) * (op1_limb_25_col98))),
                    (((op0_limb_26_col70) * (op1_limb_27_col100))
                        + ((op0_limb_27_col71) * (op1_limb_26_col99))),
                    ((op0_limb_27_col71) * (op1_limb_27_col100)),
                ];
                let x_sum_tmp_42314_34 = [
                    ((op0_limb_14_col58) + (op0_limb_21_col65)),
                    ((op0_limb_15_col59) + (op0_limb_22_col66)),
                    ((op0_limb_16_col60) + (op0_limb_23_col67)),
                    ((op0_limb_17_col61) + (op0_limb_24_col68)),
                    ((op0_limb_18_col62) + (op0_limb_25_col69)),
                    ((op0_limb_19_col63) + (op0_limb_26_col70)),
                    ((op0_limb_20_col64) + (op0_limb_27_col71)),
                ];
                let y_sum_tmp_42314_35 = [
                    ((op1_limb_14_col87) + (op1_limb_21_col94)),
                    ((op1_limb_15_col88) + (op1_limb_22_col95)),
                    ((op1_limb_16_col89) + (op1_limb_23_col96)),
                    ((op1_limb_17_col90) + (op1_limb_24_col97)),
                    ((op1_limb_18_col91) + (op1_limb_25_col98)),
                    ((op1_limb_19_col92) + (op1_limb_26_col99)),
                    ((op1_limb_20_col93) + (op1_limb_27_col100)),
                ];
                let single_karatsuba_n_7_output_tmp_42314_36 = [
                    z0_tmp_42314_32[0],
                    z0_tmp_42314_32[1],
                    z0_tmp_42314_32[2],
                    z0_tmp_42314_32[3],
                    z0_tmp_42314_32[4],
                    z0_tmp_42314_32[5],
                    z0_tmp_42314_32[6],
                    ((z0_tmp_42314_32[7])
                        + ((((x_sum_tmp_42314_34[0]) * (y_sum_tmp_42314_35[0]))
                            - (z0_tmp_42314_32[0]))
                            - (z2_tmp_42314_33[0]))),
                    ((z0_tmp_42314_32[8])
                        + (((((x_sum_tmp_42314_34[0]) * (y_sum_tmp_42314_35[1]))
                            + ((x_sum_tmp_42314_34[1]) * (y_sum_tmp_42314_35[0])))
                            - (z0_tmp_42314_32[1]))
                            - (z2_tmp_42314_33[1]))),
                    ((z0_tmp_42314_32[9])
                        + ((((((x_sum_tmp_42314_34[0]) * (y_sum_tmp_42314_35[2]))
                            + ((x_sum_tmp_42314_34[1]) * (y_sum_tmp_42314_35[1])))
                            + ((x_sum_tmp_42314_34[2]) * (y_sum_tmp_42314_35[0])))
                            - (z0_tmp_42314_32[2]))
                            - (z2_tmp_42314_33[2]))),
                    ((z0_tmp_42314_32[10])
                        + (((((((x_sum_tmp_42314_34[0]) * (y_sum_tmp_42314_35[3]))
                            + ((x_sum_tmp_42314_34[1]) * (y_sum_tmp_42314_35[2])))
                            + ((x_sum_tmp_42314_34[2]) * (y_sum_tmp_42314_35[1])))
                            + ((x_sum_tmp_42314_34[3]) * (y_sum_tmp_42314_35[0])))
                            - (z0_tmp_42314_32[3]))
                            - (z2_tmp_42314_33[3]))),
                    ((z0_tmp_42314_32[11])
                        + ((((((((x_sum_tmp_42314_34[0]) * (y_sum_tmp_42314_35[4]))
                            + ((x_sum_tmp_42314_34[1]) * (y_sum_tmp_42314_35[3])))
                            + ((x_sum_tmp_42314_34[2]) * (y_sum_tmp_42314_35[2])))
                            + ((x_sum_tmp_42314_34[3]) * (y_sum_tmp_42314_35[1])))
                            + ((x_sum_tmp_42314_34[4]) * (y_sum_tmp_42314_35[0])))
                            - (z0_tmp_42314_32[4]))
                            - (z2_tmp_42314_33[4]))),
                    ((z0_tmp_42314_32[12])
                        + (((((((((x_sum_tmp_42314_34[0]) * (y_sum_tmp_42314_35[5]))
                            + ((x_sum_tmp_42314_34[1]) * (y_sum_tmp_42314_35[4])))
                            + ((x_sum_tmp_42314_34[2]) * (y_sum_tmp_42314_35[3])))
                            + ((x_sum_tmp_42314_34[3]) * (y_sum_tmp_42314_35[2])))
                            + ((x_sum_tmp_42314_34[4]) * (y_sum_tmp_42314_35[1])))
                            + ((x_sum_tmp_42314_34[5]) * (y_sum_tmp_42314_35[0])))
                            - (z0_tmp_42314_32[5]))
                            - (z2_tmp_42314_33[5]))),
                    ((((((((((x_sum_tmp_42314_34[0]) * (y_sum_tmp_42314_35[6]))
                        + ((x_sum_tmp_42314_34[1]) * (y_sum_tmp_42314_35[5])))
                        + ((x_sum_tmp_42314_34[2]) * (y_sum_tmp_42314_35[4])))
                        + ((x_sum_tmp_42314_34[3]) * (y_sum_tmp_42314_35[3])))
                        + ((x_sum_tmp_42314_34[4]) * (y_sum_tmp_42314_35[2])))
                        + ((x_sum_tmp_42314_34[5]) * (y_sum_tmp_42314_35[1])))
                        + ((x_sum_tmp_42314_34[6]) * (y_sum_tmp_42314_35[0])))
                        - (z0_tmp_42314_32[6]))
                        - (z2_tmp_42314_33[6])),
                    ((z2_tmp_42314_33[0])
                        + (((((((((x_sum_tmp_42314_34[1]) * (y_sum_tmp_42314_35[6]))
                            + ((x_sum_tmp_42314_34[2]) * (y_sum_tmp_42314_35[5])))
                            + ((x_sum_tmp_42314_34[3]) * (y_sum_tmp_42314_35[4])))
                            + ((x_sum_tmp_42314_34[4]) * (y_sum_tmp_42314_35[3])))
                            + ((x_sum_tmp_42314_34[5]) * (y_sum_tmp_42314_35[2])))
                            + ((x_sum_tmp_42314_34[6]) * (y_sum_tmp_42314_35[1])))
                            - (z0_tmp_42314_32[7]))
                            - (z2_tmp_42314_33[7]))),
                    ((z2_tmp_42314_33[1])
                        + ((((((((x_sum_tmp_42314_34[2]) * (y_sum_tmp_42314_35[6]))
                            + ((x_sum_tmp_42314_34[3]) * (y_sum_tmp_42314_35[5])))
                            + ((x_sum_tmp_42314_34[4]) * (y_sum_tmp_42314_35[4])))
                            + ((x_sum_tmp_42314_34[5]) * (y_sum_tmp_42314_35[3])))
                            + ((x_sum_tmp_42314_34[6]) * (y_sum_tmp_42314_35[2])))
                            - (z0_tmp_42314_32[8]))
                            - (z2_tmp_42314_33[8]))),
                    ((z2_tmp_42314_33[2])
                        + (((((((x_sum_tmp_42314_34[3]) * (y_sum_tmp_42314_35[6]))
                            + ((x_sum_tmp_42314_34[4]) * (y_sum_tmp_42314_35[5])))
                            + ((x_sum_tmp_42314_34[5]) * (y_sum_tmp_42314_35[4])))
                            + ((x_sum_tmp_42314_34[6]) * (y_sum_tmp_42314_35[3])))
                            - (z0_tmp_42314_32[9]))
                            - (z2_tmp_42314_33[9]))),
                    ((z2_tmp_42314_33[3])
                        + ((((((x_sum_tmp_42314_34[4]) * (y_sum_tmp_42314_35[6]))
                            + ((x_sum_tmp_42314_34[5]) * (y_sum_tmp_42314_35[5])))
                            + ((x_sum_tmp_42314_34[6]) * (y_sum_tmp_42314_35[4])))
                            - (z0_tmp_42314_32[10]))
                            - (z2_tmp_42314_33[10]))),
                    ((z2_tmp_42314_33[4])
                        + (((((x_sum_tmp_42314_34[5]) * (y_sum_tmp_42314_35[6]))
                            + ((x_sum_tmp_42314_34[6]) * (y_sum_tmp_42314_35[5])))
                            - (z0_tmp_42314_32[11]))
                            - (z2_tmp_42314_33[11]))),
                    ((z2_tmp_42314_33[5])
                        + ((((x_sum_tmp_42314_34[6]) * (y_sum_tmp_42314_35[6]))
                            - (z0_tmp_42314_32[12]))
                            - (z2_tmp_42314_33[12]))),
                    z2_tmp_42314_33[6],
                    z2_tmp_42314_33[7],
                    z2_tmp_42314_33[8],
                    z2_tmp_42314_33[9],
                    z2_tmp_42314_33[10],
                    z2_tmp_42314_33[11],
                    z2_tmp_42314_33[12],
                ];

                let x_sum_tmp_42314_37 = [
                    ((op0_limb_0_col44) + (op0_limb_14_col58)),
                    ((op0_limb_1_col45) + (op0_limb_15_col59)),
                    ((op0_limb_2_col46) + (op0_limb_16_col60)),
                    ((op0_limb_3_col47) + (op0_limb_17_col61)),
                    ((op0_limb_4_col48) + (op0_limb_18_col62)),
                    ((op0_limb_5_col49) + (op0_limb_19_col63)),
                    ((op0_limb_6_col50) + (op0_limb_20_col64)),
                    ((op0_limb_7_col51) + (op0_limb_21_col65)),
                    ((op0_limb_8_col52) + (op0_limb_22_col66)),
                    ((op0_limb_9_col53) + (op0_limb_23_col67)),
                    ((op0_limb_10_col54) + (op0_limb_24_col68)),
                    ((op0_limb_11_col55) + (op0_limb_25_col69)),
                    ((op0_limb_12_col56) + (op0_limb_26_col70)),
                    ((op0_limb_13_col57) + (op0_limb_27_col71)),
                ];
                let y_sum_tmp_42314_38 = [
                    ((op1_limb_0_col73) + (op1_limb_14_col87)),
                    ((op1_limb_1_col74) + (op1_limb_15_col88)),
                    ((op1_limb_2_col75) + (op1_limb_16_col89)),
                    ((op1_limb_3_col76) + (op1_limb_17_col90)),
                    ((op1_limb_4_col77) + (op1_limb_18_col91)),
                    ((op1_limb_5_col78) + (op1_limb_19_col92)),
                    ((op1_limb_6_col79) + (op1_limb_20_col93)),
                    ((op1_limb_7_col80) + (op1_limb_21_col94)),
                    ((op1_limb_8_col81) + (op1_limb_22_col95)),
                    ((op1_limb_9_col82) + (op1_limb_23_col96)),
                    ((op1_limb_10_col83) + (op1_limb_24_col97)),
                    ((op1_limb_11_col84) + (op1_limb_25_col98)),
                    ((op1_limb_12_col85) + (op1_limb_26_col99)),
                    ((op1_limb_13_col86) + (op1_limb_27_col100)),
                ];

                // Single Karatsuba N 7.

                let z0_tmp_42314_39 = [
                    ((x_sum_tmp_42314_37[0]) * (y_sum_tmp_42314_38[0])),
                    (((x_sum_tmp_42314_37[0]) * (y_sum_tmp_42314_38[1]))
                        + ((x_sum_tmp_42314_37[1]) * (y_sum_tmp_42314_38[0]))),
                    ((((x_sum_tmp_42314_37[0]) * (y_sum_tmp_42314_38[2]))
                        + ((x_sum_tmp_42314_37[1]) * (y_sum_tmp_42314_38[1])))
                        + ((x_sum_tmp_42314_37[2]) * (y_sum_tmp_42314_38[0]))),
                    (((((x_sum_tmp_42314_37[0]) * (y_sum_tmp_42314_38[3]))
                        + ((x_sum_tmp_42314_37[1]) * (y_sum_tmp_42314_38[2])))
                        + ((x_sum_tmp_42314_37[2]) * (y_sum_tmp_42314_38[1])))
                        + ((x_sum_tmp_42314_37[3]) * (y_sum_tmp_42314_38[0]))),
                    ((((((x_sum_tmp_42314_37[0]) * (y_sum_tmp_42314_38[4]))
                        + ((x_sum_tmp_42314_37[1]) * (y_sum_tmp_42314_38[3])))
                        + ((x_sum_tmp_42314_37[2]) * (y_sum_tmp_42314_38[2])))
                        + ((x_sum_tmp_42314_37[3]) * (y_sum_tmp_42314_38[1])))
                        + ((x_sum_tmp_42314_37[4]) * (y_sum_tmp_42314_38[0]))),
                    (((((((x_sum_tmp_42314_37[0]) * (y_sum_tmp_42314_38[5]))
                        + ((x_sum_tmp_42314_37[1]) * (y_sum_tmp_42314_38[4])))
                        + ((x_sum_tmp_42314_37[2]) * (y_sum_tmp_42314_38[3])))
                        + ((x_sum_tmp_42314_37[3]) * (y_sum_tmp_42314_38[2])))
                        + ((x_sum_tmp_42314_37[4]) * (y_sum_tmp_42314_38[1])))
                        + ((x_sum_tmp_42314_37[5]) * (y_sum_tmp_42314_38[0]))),
                    ((((((((x_sum_tmp_42314_37[0]) * (y_sum_tmp_42314_38[6]))
                        + ((x_sum_tmp_42314_37[1]) * (y_sum_tmp_42314_38[5])))
                        + ((x_sum_tmp_42314_37[2]) * (y_sum_tmp_42314_38[4])))
                        + ((x_sum_tmp_42314_37[3]) * (y_sum_tmp_42314_38[3])))
                        + ((x_sum_tmp_42314_37[4]) * (y_sum_tmp_42314_38[2])))
                        + ((x_sum_tmp_42314_37[5]) * (y_sum_tmp_42314_38[1])))
                        + ((x_sum_tmp_42314_37[6]) * (y_sum_tmp_42314_38[0]))),
                    (((((((x_sum_tmp_42314_37[1]) * (y_sum_tmp_42314_38[6]))
                        + ((x_sum_tmp_42314_37[2]) * (y_sum_tmp_42314_38[5])))
                        + ((x_sum_tmp_42314_37[3]) * (y_sum_tmp_42314_38[4])))
                        + ((x_sum_tmp_42314_37[4]) * (y_sum_tmp_42314_38[3])))
                        + ((x_sum_tmp_42314_37[5]) * (y_sum_tmp_42314_38[2])))
                        + ((x_sum_tmp_42314_37[6]) * (y_sum_tmp_42314_38[1]))),
                    ((((((x_sum_tmp_42314_37[2]) * (y_sum_tmp_42314_38[6]))
                        + ((x_sum_tmp_42314_37[3]) * (y_sum_tmp_42314_38[5])))
                        + ((x_sum_tmp_42314_37[4]) * (y_sum_tmp_42314_38[4])))
                        + ((x_sum_tmp_42314_37[5]) * (y_sum_tmp_42314_38[3])))
                        + ((x_sum_tmp_42314_37[6]) * (y_sum_tmp_42314_38[2]))),
                    (((((x_sum_tmp_42314_37[3]) * (y_sum_tmp_42314_38[6]))
                        + ((x_sum_tmp_42314_37[4]) * (y_sum_tmp_42314_38[5])))
                        + ((x_sum_tmp_42314_37[5]) * (y_sum_tmp_42314_38[4])))
                        + ((x_sum_tmp_42314_37[6]) * (y_sum_tmp_42314_38[3]))),
                    ((((x_sum_tmp_42314_37[4]) * (y_sum_tmp_42314_38[6]))
                        + ((x_sum_tmp_42314_37[5]) * (y_sum_tmp_42314_38[5])))
                        + ((x_sum_tmp_42314_37[6]) * (y_sum_tmp_42314_38[4]))),
                    (((x_sum_tmp_42314_37[5]) * (y_sum_tmp_42314_38[6]))
                        + ((x_sum_tmp_42314_37[6]) * (y_sum_tmp_42314_38[5]))),
                    ((x_sum_tmp_42314_37[6]) * (y_sum_tmp_42314_38[6])),
                ];
                let z2_tmp_42314_40 = [
                    ((x_sum_tmp_42314_37[7]) * (y_sum_tmp_42314_38[7])),
                    (((x_sum_tmp_42314_37[7]) * (y_sum_tmp_42314_38[8]))
                        + ((x_sum_tmp_42314_37[8]) * (y_sum_tmp_42314_38[7]))),
                    ((((x_sum_tmp_42314_37[7]) * (y_sum_tmp_42314_38[9]))
                        + ((x_sum_tmp_42314_37[8]) * (y_sum_tmp_42314_38[8])))
                        + ((x_sum_tmp_42314_37[9]) * (y_sum_tmp_42314_38[7]))),
                    (((((x_sum_tmp_42314_37[7]) * (y_sum_tmp_42314_38[10]))
                        + ((x_sum_tmp_42314_37[8]) * (y_sum_tmp_42314_38[9])))
                        + ((x_sum_tmp_42314_37[9]) * (y_sum_tmp_42314_38[8])))
                        + ((x_sum_tmp_42314_37[10]) * (y_sum_tmp_42314_38[7]))),
                    ((((((x_sum_tmp_42314_37[7]) * (y_sum_tmp_42314_38[11]))
                        + ((x_sum_tmp_42314_37[8]) * (y_sum_tmp_42314_38[10])))
                        + ((x_sum_tmp_42314_37[9]) * (y_sum_tmp_42314_38[9])))
                        + ((x_sum_tmp_42314_37[10]) * (y_sum_tmp_42314_38[8])))
                        + ((x_sum_tmp_42314_37[11]) * (y_sum_tmp_42314_38[7]))),
                    (((((((x_sum_tmp_42314_37[7]) * (y_sum_tmp_42314_38[12]))
                        + ((x_sum_tmp_42314_37[8]) * (y_sum_tmp_42314_38[11])))
                        + ((x_sum_tmp_42314_37[9]) * (y_sum_tmp_42314_38[10])))
                        + ((x_sum_tmp_42314_37[10]) * (y_sum_tmp_42314_38[9])))
                        + ((x_sum_tmp_42314_37[11]) * (y_sum_tmp_42314_38[8])))
                        + ((x_sum_tmp_42314_37[12]) * (y_sum_tmp_42314_38[7]))),
                    ((((((((x_sum_tmp_42314_37[7]) * (y_sum_tmp_42314_38[13]))
                        + ((x_sum_tmp_42314_37[8]) * (y_sum_tmp_42314_38[12])))
                        + ((x_sum_tmp_42314_37[9]) * (y_sum_tmp_42314_38[11])))
                        + ((x_sum_tmp_42314_37[10]) * (y_sum_tmp_42314_38[10])))
                        + ((x_sum_tmp_42314_37[11]) * (y_sum_tmp_42314_38[9])))
                        + ((x_sum_tmp_42314_37[12]) * (y_sum_tmp_42314_38[8])))
                        + ((x_sum_tmp_42314_37[13]) * (y_sum_tmp_42314_38[7]))),
                    (((((((x_sum_tmp_42314_37[8]) * (y_sum_tmp_42314_38[13]))
                        + ((x_sum_tmp_42314_37[9]) * (y_sum_tmp_42314_38[12])))
                        + ((x_sum_tmp_42314_37[10]) * (y_sum_tmp_42314_38[11])))
                        + ((x_sum_tmp_42314_37[11]) * (y_sum_tmp_42314_38[10])))
                        + ((x_sum_tmp_42314_37[12]) * (y_sum_tmp_42314_38[9])))
                        + ((x_sum_tmp_42314_37[13]) * (y_sum_tmp_42314_38[8]))),
                    ((((((x_sum_tmp_42314_37[9]) * (y_sum_tmp_42314_38[13]))
                        + ((x_sum_tmp_42314_37[10]) * (y_sum_tmp_42314_38[12])))
                        + ((x_sum_tmp_42314_37[11]) * (y_sum_tmp_42314_38[11])))
                        + ((x_sum_tmp_42314_37[12]) * (y_sum_tmp_42314_38[10])))
                        + ((x_sum_tmp_42314_37[13]) * (y_sum_tmp_42314_38[9]))),
                    (((((x_sum_tmp_42314_37[10]) * (y_sum_tmp_42314_38[13]))
                        + ((x_sum_tmp_42314_37[11]) * (y_sum_tmp_42314_38[12])))
                        + ((x_sum_tmp_42314_37[12]) * (y_sum_tmp_42314_38[11])))
                        + ((x_sum_tmp_42314_37[13]) * (y_sum_tmp_42314_38[10]))),
                    ((((x_sum_tmp_42314_37[11]) * (y_sum_tmp_42314_38[13]))
                        + ((x_sum_tmp_42314_37[12]) * (y_sum_tmp_42314_38[12])))
                        + ((x_sum_tmp_42314_37[13]) * (y_sum_tmp_42314_38[11]))),
                    (((x_sum_tmp_42314_37[12]) * (y_sum_tmp_42314_38[13]))
                        + ((x_sum_tmp_42314_37[13]) * (y_sum_tmp_42314_38[12]))),
                    ((x_sum_tmp_42314_37[13]) * (y_sum_tmp_42314_38[13])),
                ];
                let x_sum_tmp_42314_41 = [
                    ((x_sum_tmp_42314_37[0]) + (x_sum_tmp_42314_37[7])),
                    ((x_sum_tmp_42314_37[1]) + (x_sum_tmp_42314_37[8])),
                    ((x_sum_tmp_42314_37[2]) + (x_sum_tmp_42314_37[9])),
                    ((x_sum_tmp_42314_37[3]) + (x_sum_tmp_42314_37[10])),
                    ((x_sum_tmp_42314_37[4]) + (x_sum_tmp_42314_37[11])),
                    ((x_sum_tmp_42314_37[5]) + (x_sum_tmp_42314_37[12])),
                    ((x_sum_tmp_42314_37[6]) + (x_sum_tmp_42314_37[13])),
                ];
                let y_sum_tmp_42314_42 = [
                    ((y_sum_tmp_42314_38[0]) + (y_sum_tmp_42314_38[7])),
                    ((y_sum_tmp_42314_38[1]) + (y_sum_tmp_42314_38[8])),
                    ((y_sum_tmp_42314_38[2]) + (y_sum_tmp_42314_38[9])),
                    ((y_sum_tmp_42314_38[3]) + (y_sum_tmp_42314_38[10])),
                    ((y_sum_tmp_42314_38[4]) + (y_sum_tmp_42314_38[11])),
                    ((y_sum_tmp_42314_38[5]) + (y_sum_tmp_42314_38[12])),
                    ((y_sum_tmp_42314_38[6]) + (y_sum_tmp_42314_38[13])),
                ];
                let single_karatsuba_n_7_output_tmp_42314_43 = [
                    z0_tmp_42314_39[0],
                    z0_tmp_42314_39[1],
                    z0_tmp_42314_39[2],
                    z0_tmp_42314_39[3],
                    z0_tmp_42314_39[4],
                    z0_tmp_42314_39[5],
                    z0_tmp_42314_39[6],
                    ((z0_tmp_42314_39[7])
                        + ((((x_sum_tmp_42314_41[0]) * (y_sum_tmp_42314_42[0]))
                            - (z0_tmp_42314_39[0]))
                            - (z2_tmp_42314_40[0]))),
                    ((z0_tmp_42314_39[8])
                        + (((((x_sum_tmp_42314_41[0]) * (y_sum_tmp_42314_42[1]))
                            + ((x_sum_tmp_42314_41[1]) * (y_sum_tmp_42314_42[0])))
                            - (z0_tmp_42314_39[1]))
                            - (z2_tmp_42314_40[1]))),
                    ((z0_tmp_42314_39[9])
                        + ((((((x_sum_tmp_42314_41[0]) * (y_sum_tmp_42314_42[2]))
                            + ((x_sum_tmp_42314_41[1]) * (y_sum_tmp_42314_42[1])))
                            + ((x_sum_tmp_42314_41[2]) * (y_sum_tmp_42314_42[0])))
                            - (z0_tmp_42314_39[2]))
                            - (z2_tmp_42314_40[2]))),
                    ((z0_tmp_42314_39[10])
                        + (((((((x_sum_tmp_42314_41[0]) * (y_sum_tmp_42314_42[3]))
                            + ((x_sum_tmp_42314_41[1]) * (y_sum_tmp_42314_42[2])))
                            + ((x_sum_tmp_42314_41[2]) * (y_sum_tmp_42314_42[1])))
                            + ((x_sum_tmp_42314_41[3]) * (y_sum_tmp_42314_42[0])))
                            - (z0_tmp_42314_39[3]))
                            - (z2_tmp_42314_40[3]))),
                    ((z0_tmp_42314_39[11])
                        + ((((((((x_sum_tmp_42314_41[0]) * (y_sum_tmp_42314_42[4]))
                            + ((x_sum_tmp_42314_41[1]) * (y_sum_tmp_42314_42[3])))
                            + ((x_sum_tmp_42314_41[2]) * (y_sum_tmp_42314_42[2])))
                            + ((x_sum_tmp_42314_41[3]) * (y_sum_tmp_42314_42[1])))
                            + ((x_sum_tmp_42314_41[4]) * (y_sum_tmp_42314_42[0])))
                            - (z0_tmp_42314_39[4]))
                            - (z2_tmp_42314_40[4]))),
                    ((z0_tmp_42314_39[12])
                        + (((((((((x_sum_tmp_42314_41[0]) * (y_sum_tmp_42314_42[5]))
                            + ((x_sum_tmp_42314_41[1]) * (y_sum_tmp_42314_42[4])))
                            + ((x_sum_tmp_42314_41[2]) * (y_sum_tmp_42314_42[3])))
                            + ((x_sum_tmp_42314_41[3]) * (y_sum_tmp_42314_42[2])))
                            + ((x_sum_tmp_42314_41[4]) * (y_sum_tmp_42314_42[1])))
                            + ((x_sum_tmp_42314_41[5]) * (y_sum_tmp_42314_42[0])))
                            - (z0_tmp_42314_39[5]))
                            - (z2_tmp_42314_40[5]))),
                    ((((((((((x_sum_tmp_42314_41[0]) * (y_sum_tmp_42314_42[6]))
                        + ((x_sum_tmp_42314_41[1]) * (y_sum_tmp_42314_42[5])))
                        + ((x_sum_tmp_42314_41[2]) * (y_sum_tmp_42314_42[4])))
                        + ((x_sum_tmp_42314_41[3]) * (y_sum_tmp_42314_42[3])))
                        + ((x_sum_tmp_42314_41[4]) * (y_sum_tmp_42314_42[2])))
                        + ((x_sum_tmp_42314_41[5]) * (y_sum_tmp_42314_42[1])))
                        + ((x_sum_tmp_42314_41[6]) * (y_sum_tmp_42314_42[0])))
                        - (z0_tmp_42314_39[6]))
                        - (z2_tmp_42314_40[6])),
                    ((z2_tmp_42314_40[0])
                        + (((((((((x_sum_tmp_42314_41[1]) * (y_sum_tmp_42314_42[6]))
                            + ((x_sum_tmp_42314_41[2]) * (y_sum_tmp_42314_42[5])))
                            + ((x_sum_tmp_42314_41[3]) * (y_sum_tmp_42314_42[4])))
                            + ((x_sum_tmp_42314_41[4]) * (y_sum_tmp_42314_42[3])))
                            + ((x_sum_tmp_42314_41[5]) * (y_sum_tmp_42314_42[2])))
                            + ((x_sum_tmp_42314_41[6]) * (y_sum_tmp_42314_42[1])))
                            - (z0_tmp_42314_39[7]))
                            - (z2_tmp_42314_40[7]))),
                    ((z2_tmp_42314_40[1])
                        + ((((((((x_sum_tmp_42314_41[2]) * (y_sum_tmp_42314_42[6]))
                            + ((x_sum_tmp_42314_41[3]) * (y_sum_tmp_42314_42[5])))
                            + ((x_sum_tmp_42314_41[4]) * (y_sum_tmp_42314_42[4])))
                            + ((x_sum_tmp_42314_41[5]) * (y_sum_tmp_42314_42[3])))
                            + ((x_sum_tmp_42314_41[6]) * (y_sum_tmp_42314_42[2])))
                            - (z0_tmp_42314_39[8]))
                            - (z2_tmp_42314_40[8]))),
                    ((z2_tmp_42314_40[2])
                        + (((((((x_sum_tmp_42314_41[3]) * (y_sum_tmp_42314_42[6]))
                            + ((x_sum_tmp_42314_41[4]) * (y_sum_tmp_42314_42[5])))
                            + ((x_sum_tmp_42314_41[5]) * (y_sum_tmp_42314_42[4])))
                            + ((x_sum_tmp_42314_41[6]) * (y_sum_tmp_42314_42[3])))
                            - (z0_tmp_42314_39[9]))
                            - (z2_tmp_42314_40[9]))),
                    ((z2_tmp_42314_40[3])
                        + ((((((x_sum_tmp_42314_41[4]) * (y_sum_tmp_42314_42[6]))
                            + ((x_sum_tmp_42314_41[5]) * (y_sum_tmp_42314_42[5])))
                            + ((x_sum_tmp_42314_41[6]) * (y_sum_tmp_42314_42[4])))
                            - (z0_tmp_42314_39[10]))
                            - (z2_tmp_42314_40[10]))),
                    ((z2_tmp_42314_40[4])
                        + (((((x_sum_tmp_42314_41[5]) * (y_sum_tmp_42314_42[6]))
                            + ((x_sum_tmp_42314_41[6]) * (y_sum_tmp_42314_42[5])))
                            - (z0_tmp_42314_39[11]))
                            - (z2_tmp_42314_40[11]))),
                    ((z2_tmp_42314_40[5])
                        + ((((x_sum_tmp_42314_41[6]) * (y_sum_tmp_42314_42[6]))
                            - (z0_tmp_42314_39[12]))
                            - (z2_tmp_42314_40[12]))),
                    z2_tmp_42314_40[6],
                    z2_tmp_42314_40[7],
                    z2_tmp_42314_40[8],
                    z2_tmp_42314_40[9],
                    z2_tmp_42314_40[10],
                    z2_tmp_42314_40[11],
                    z2_tmp_42314_40[12],
                ];

                let double_karatsuba_1454b_output_tmp_42314_44 = [
                    single_karatsuba_n_7_output_tmp_42314_31[0],
                    single_karatsuba_n_7_output_tmp_42314_31[1],
                    single_karatsuba_n_7_output_tmp_42314_31[2],
                    single_karatsuba_n_7_output_tmp_42314_31[3],
                    single_karatsuba_n_7_output_tmp_42314_31[4],
                    single_karatsuba_n_7_output_tmp_42314_31[5],
                    single_karatsuba_n_7_output_tmp_42314_31[6],
                    single_karatsuba_n_7_output_tmp_42314_31[7],
                    single_karatsuba_n_7_output_tmp_42314_31[8],
                    single_karatsuba_n_7_output_tmp_42314_31[9],
                    single_karatsuba_n_7_output_tmp_42314_31[10],
                    single_karatsuba_n_7_output_tmp_42314_31[11],
                    single_karatsuba_n_7_output_tmp_42314_31[12],
                    single_karatsuba_n_7_output_tmp_42314_31[13],
                    ((single_karatsuba_n_7_output_tmp_42314_31[14])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[0])
                            - (single_karatsuba_n_7_output_tmp_42314_31[0]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[0]))),
                    ((single_karatsuba_n_7_output_tmp_42314_31[15])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[1])
                            - (single_karatsuba_n_7_output_tmp_42314_31[1]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[1]))),
                    ((single_karatsuba_n_7_output_tmp_42314_31[16])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[2])
                            - (single_karatsuba_n_7_output_tmp_42314_31[2]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[2]))),
                    ((single_karatsuba_n_7_output_tmp_42314_31[17])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[3])
                            - (single_karatsuba_n_7_output_tmp_42314_31[3]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[3]))),
                    ((single_karatsuba_n_7_output_tmp_42314_31[18])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[4])
                            - (single_karatsuba_n_7_output_tmp_42314_31[4]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[4]))),
                    ((single_karatsuba_n_7_output_tmp_42314_31[19])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[5])
                            - (single_karatsuba_n_7_output_tmp_42314_31[5]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[5]))),
                    ((single_karatsuba_n_7_output_tmp_42314_31[20])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[6])
                            - (single_karatsuba_n_7_output_tmp_42314_31[6]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[6]))),
                    ((single_karatsuba_n_7_output_tmp_42314_31[21])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[7])
                            - (single_karatsuba_n_7_output_tmp_42314_31[7]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[7]))),
                    ((single_karatsuba_n_7_output_tmp_42314_31[22])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[8])
                            - (single_karatsuba_n_7_output_tmp_42314_31[8]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[8]))),
                    ((single_karatsuba_n_7_output_tmp_42314_31[23])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[9])
                            - (single_karatsuba_n_7_output_tmp_42314_31[9]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[9]))),
                    ((single_karatsuba_n_7_output_tmp_42314_31[24])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[10])
                            - (single_karatsuba_n_7_output_tmp_42314_31[10]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[10]))),
                    ((single_karatsuba_n_7_output_tmp_42314_31[25])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[11])
                            - (single_karatsuba_n_7_output_tmp_42314_31[11]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[11]))),
                    ((single_karatsuba_n_7_output_tmp_42314_31[26])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[12])
                            - (single_karatsuba_n_7_output_tmp_42314_31[12]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[12]))),
                    (((single_karatsuba_n_7_output_tmp_42314_43[13])
                        - (single_karatsuba_n_7_output_tmp_42314_31[13]))
                        - (single_karatsuba_n_7_output_tmp_42314_36[13])),
                    ((single_karatsuba_n_7_output_tmp_42314_36[0])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[14])
                            - (single_karatsuba_n_7_output_tmp_42314_31[14]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[14]))),
                    ((single_karatsuba_n_7_output_tmp_42314_36[1])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[15])
                            - (single_karatsuba_n_7_output_tmp_42314_31[15]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[15]))),
                    ((single_karatsuba_n_7_output_tmp_42314_36[2])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[16])
                            - (single_karatsuba_n_7_output_tmp_42314_31[16]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[16]))),
                    ((single_karatsuba_n_7_output_tmp_42314_36[3])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[17])
                            - (single_karatsuba_n_7_output_tmp_42314_31[17]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[17]))),
                    ((single_karatsuba_n_7_output_tmp_42314_36[4])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[18])
                            - (single_karatsuba_n_7_output_tmp_42314_31[18]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[18]))),
                    ((single_karatsuba_n_7_output_tmp_42314_36[5])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[19])
                            - (single_karatsuba_n_7_output_tmp_42314_31[19]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[19]))),
                    ((single_karatsuba_n_7_output_tmp_42314_36[6])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[20])
                            - (single_karatsuba_n_7_output_tmp_42314_31[20]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[20]))),
                    ((single_karatsuba_n_7_output_tmp_42314_36[7])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[21])
                            - (single_karatsuba_n_7_output_tmp_42314_31[21]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[21]))),
                    ((single_karatsuba_n_7_output_tmp_42314_36[8])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[22])
                            - (single_karatsuba_n_7_output_tmp_42314_31[22]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[22]))),
                    ((single_karatsuba_n_7_output_tmp_42314_36[9])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[23])
                            - (single_karatsuba_n_7_output_tmp_42314_31[23]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[23]))),
                    ((single_karatsuba_n_7_output_tmp_42314_36[10])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[24])
                            - (single_karatsuba_n_7_output_tmp_42314_31[24]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[24]))),
                    ((single_karatsuba_n_7_output_tmp_42314_36[11])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[25])
                            - (single_karatsuba_n_7_output_tmp_42314_31[25]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[25]))),
                    ((single_karatsuba_n_7_output_tmp_42314_36[12])
                        + (((single_karatsuba_n_7_output_tmp_42314_43[26])
                            - (single_karatsuba_n_7_output_tmp_42314_31[26]))
                            - (single_karatsuba_n_7_output_tmp_42314_36[26]))),
                    single_karatsuba_n_7_output_tmp_42314_36[13],
                    single_karatsuba_n_7_output_tmp_42314_36[14],
                    single_karatsuba_n_7_output_tmp_42314_36[15],
                    single_karatsuba_n_7_output_tmp_42314_36[16],
                    single_karatsuba_n_7_output_tmp_42314_36[17],
                    single_karatsuba_n_7_output_tmp_42314_36[18],
                    single_karatsuba_n_7_output_tmp_42314_36[19],
                    single_karatsuba_n_7_output_tmp_42314_36[20],
                    single_karatsuba_n_7_output_tmp_42314_36[21],
                    single_karatsuba_n_7_output_tmp_42314_36[22],
                    single_karatsuba_n_7_output_tmp_42314_36[23],
                    single_karatsuba_n_7_output_tmp_42314_36[24],
                    single_karatsuba_n_7_output_tmp_42314_36[25],
                    single_karatsuba_n_7_output_tmp_42314_36[26],
                ];

                let conv_tmp_42314_45 = [
                    ((double_karatsuba_1454b_output_tmp_42314_44[0]) - (dst_limb_0_col15)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[1]) - (dst_limb_1_col16)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[2]) - (dst_limb_2_col17)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[3]) - (dst_limb_3_col18)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[4]) - (dst_limb_4_col19)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[5]) - (dst_limb_5_col20)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[6]) - (dst_limb_6_col21)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[7]) - (dst_limb_7_col22)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[8]) - (dst_limb_8_col23)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[9]) - (dst_limb_9_col24)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[10]) - (dst_limb_10_col25)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[11]) - (dst_limb_11_col26)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[12]) - (dst_limb_12_col27)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[13]) - (dst_limb_13_col28)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[14]) - (dst_limb_14_col29)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[15]) - (dst_limb_15_col30)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[16]) - (dst_limb_16_col31)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[17]) - (dst_limb_17_col32)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[18]) - (dst_limb_18_col33)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[19]) - (dst_limb_19_col34)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[20]) - (dst_limb_20_col35)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[21]) - (dst_limb_21_col36)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[22]) - (dst_limb_22_col37)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[23]) - (dst_limb_23_col38)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[24]) - (dst_limb_24_col39)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[25]) - (dst_limb_25_col40)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[26]) - (dst_limb_26_col41)),
                    ((double_karatsuba_1454b_output_tmp_42314_44[27]) - (dst_limb_27_col42)),
                    double_karatsuba_1454b_output_tmp_42314_44[28],
                    double_karatsuba_1454b_output_tmp_42314_44[29],
                    double_karatsuba_1454b_output_tmp_42314_44[30],
                    double_karatsuba_1454b_output_tmp_42314_44[31],
                    double_karatsuba_1454b_output_tmp_42314_44[32],
                    double_karatsuba_1454b_output_tmp_42314_44[33],
                    double_karatsuba_1454b_output_tmp_42314_44[34],
                    double_karatsuba_1454b_output_tmp_42314_44[35],
                    double_karatsuba_1454b_output_tmp_42314_44[36],
                    double_karatsuba_1454b_output_tmp_42314_44[37],
                    double_karatsuba_1454b_output_tmp_42314_44[38],
                    double_karatsuba_1454b_output_tmp_42314_44[39],
                    double_karatsuba_1454b_output_tmp_42314_44[40],
                    double_karatsuba_1454b_output_tmp_42314_44[41],
                    double_karatsuba_1454b_output_tmp_42314_44[42],
                    double_karatsuba_1454b_output_tmp_42314_44[43],
                    double_karatsuba_1454b_output_tmp_42314_44[44],
                    double_karatsuba_1454b_output_tmp_42314_44[45],
                    double_karatsuba_1454b_output_tmp_42314_44[46],
                    double_karatsuba_1454b_output_tmp_42314_44[47],
                    double_karatsuba_1454b_output_tmp_42314_44[48],
                    double_karatsuba_1454b_output_tmp_42314_44[49],
                    double_karatsuba_1454b_output_tmp_42314_44[50],
                    double_karatsuba_1454b_output_tmp_42314_44[51],
                    double_karatsuba_1454b_output_tmp_42314_44[52],
                    double_karatsuba_1454b_output_tmp_42314_44[53],
                    double_karatsuba_1454b_output_tmp_42314_44[54],
                ];
                let conv_mod_tmp_42314_46 = [
                    ((((M31_32) * (conv_tmp_42314_45[0])) - ((M31_4) * (conv_tmp_42314_45[21])))
                        + ((M31_8) * (conv_tmp_42314_45[49]))),
                    ((((conv_tmp_42314_45[0]) + ((M31_32) * (conv_tmp_42314_45[1])))
                        - ((M31_4) * (conv_tmp_42314_45[22])))
                        + ((M31_8) * (conv_tmp_42314_45[50]))),
                    ((((conv_tmp_42314_45[1]) + ((M31_32) * (conv_tmp_42314_45[2])))
                        - ((M31_4) * (conv_tmp_42314_45[23])))
                        + ((M31_8) * (conv_tmp_42314_45[51]))),
                    ((((conv_tmp_42314_45[2]) + ((M31_32) * (conv_tmp_42314_45[3])))
                        - ((M31_4) * (conv_tmp_42314_45[24])))
                        + ((M31_8) * (conv_tmp_42314_45[52]))),
                    ((((conv_tmp_42314_45[3]) + ((M31_32) * (conv_tmp_42314_45[4])))
                        - ((M31_4) * (conv_tmp_42314_45[25])))
                        + ((M31_8) * (conv_tmp_42314_45[53]))),
                    ((((conv_tmp_42314_45[4]) + ((M31_32) * (conv_tmp_42314_45[5])))
                        - ((M31_4) * (conv_tmp_42314_45[26])))
                        + ((M31_8) * (conv_tmp_42314_45[54]))),
                    (((conv_tmp_42314_45[5]) + ((M31_32) * (conv_tmp_42314_45[6])))
                        - ((M31_4) * (conv_tmp_42314_45[27]))),
                    (((((M31_2) * (conv_tmp_42314_45[0])) + (conv_tmp_42314_45[6]))
                        + ((M31_32) * (conv_tmp_42314_45[7])))
                        - ((M31_4) * (conv_tmp_42314_45[28]))),
                    (((((M31_2) * (conv_tmp_42314_45[1])) + (conv_tmp_42314_45[7]))
                        + ((M31_32) * (conv_tmp_42314_45[8])))
                        - ((M31_4) * (conv_tmp_42314_45[29]))),
                    (((((M31_2) * (conv_tmp_42314_45[2])) + (conv_tmp_42314_45[8]))
                        + ((M31_32) * (conv_tmp_42314_45[9])))
                        - ((M31_4) * (conv_tmp_42314_45[30]))),
                    (((((M31_2) * (conv_tmp_42314_45[3])) + (conv_tmp_42314_45[9]))
                        + ((M31_32) * (conv_tmp_42314_45[10])))
                        - ((M31_4) * (conv_tmp_42314_45[31]))),
                    (((((M31_2) * (conv_tmp_42314_45[4])) + (conv_tmp_42314_45[10]))
                        + ((M31_32) * (conv_tmp_42314_45[11])))
                        - ((M31_4) * (conv_tmp_42314_45[32]))),
                    (((((M31_2) * (conv_tmp_42314_45[5])) + (conv_tmp_42314_45[11]))
                        + ((M31_32) * (conv_tmp_42314_45[12])))
                        - ((M31_4) * (conv_tmp_42314_45[33]))),
                    (((((M31_2) * (conv_tmp_42314_45[6])) + (conv_tmp_42314_45[12]))
                        + ((M31_32) * (conv_tmp_42314_45[13])))
                        - ((M31_4) * (conv_tmp_42314_45[34]))),
                    (((((M31_2) * (conv_tmp_42314_45[7])) + (conv_tmp_42314_45[13]))
                        + ((M31_32) * (conv_tmp_42314_45[14])))
                        - ((M31_4) * (conv_tmp_42314_45[35]))),
                    (((((M31_2) * (conv_tmp_42314_45[8])) + (conv_tmp_42314_45[14]))
                        + ((M31_32) * (conv_tmp_42314_45[15])))
                        - ((M31_4) * (conv_tmp_42314_45[36]))),
                    (((((M31_2) * (conv_tmp_42314_45[9])) + (conv_tmp_42314_45[15]))
                        + ((M31_32) * (conv_tmp_42314_45[16])))
                        - ((M31_4) * (conv_tmp_42314_45[37]))),
                    (((((M31_2) * (conv_tmp_42314_45[10])) + (conv_tmp_42314_45[16]))
                        + ((M31_32) * (conv_tmp_42314_45[17])))
                        - ((M31_4) * (conv_tmp_42314_45[38]))),
                    (((((M31_2) * (conv_tmp_42314_45[11])) + (conv_tmp_42314_45[17]))
                        + ((M31_32) * (conv_tmp_42314_45[18])))
                        - ((M31_4) * (conv_tmp_42314_45[39]))),
                    (((((M31_2) * (conv_tmp_42314_45[12])) + (conv_tmp_42314_45[18]))
                        + ((M31_32) * (conv_tmp_42314_45[19])))
                        - ((M31_4) * (conv_tmp_42314_45[40]))),
                    (((((M31_2) * (conv_tmp_42314_45[13])) + (conv_tmp_42314_45[19]))
                        + ((M31_32) * (conv_tmp_42314_45[20])))
                        - ((M31_4) * (conv_tmp_42314_45[41]))),
                    (((((M31_2) * (conv_tmp_42314_45[14])) + (conv_tmp_42314_45[20]))
                        - ((M31_4) * (conv_tmp_42314_45[42])))
                        + ((M31_64) * (conv_tmp_42314_45[49]))),
                    (((((M31_2) * (conv_tmp_42314_45[15])) - ((M31_4) * (conv_tmp_42314_45[43])))
                        + ((M31_2) * (conv_tmp_42314_45[49])))
                        + ((M31_64) * (conv_tmp_42314_45[50]))),
                    (((((M31_2) * (conv_tmp_42314_45[16])) - ((M31_4) * (conv_tmp_42314_45[44])))
                        + ((M31_2) * (conv_tmp_42314_45[50])))
                        + ((M31_64) * (conv_tmp_42314_45[51]))),
                    (((((M31_2) * (conv_tmp_42314_45[17])) - ((M31_4) * (conv_tmp_42314_45[45])))
                        + ((M31_2) * (conv_tmp_42314_45[51])))
                        + ((M31_64) * (conv_tmp_42314_45[52]))),
                    (((((M31_2) * (conv_tmp_42314_45[18])) - ((M31_4) * (conv_tmp_42314_45[46])))
                        + ((M31_2) * (conv_tmp_42314_45[52])))
                        + ((M31_64) * (conv_tmp_42314_45[53]))),
                    (((((M31_2) * (conv_tmp_42314_45[19])) - ((M31_4) * (conv_tmp_42314_45[47])))
                        + ((M31_2) * (conv_tmp_42314_45[53])))
                        + ((M31_64) * (conv_tmp_42314_45[54]))),
                    ((((M31_2) * (conv_tmp_42314_45[20])) - ((M31_4) * (conv_tmp_42314_45[48])))
                        + ((M31_2) * (conv_tmp_42314_45[54]))),
                ];
                let k_mod_2_18_biased_tmp_42314_47 =
                    ((((PackedUInt32::from_m31(((conv_mod_tmp_42314_46[0]) + (M31_134217728))))
                        + (((PackedUInt32::from_m31(
                            ((conv_mod_tmp_42314_46[1]) + (M31_134217728)),
                        )) & (UInt32_511))
                            << (UInt32_9)))
                        + (UInt32_131072))
                        & (UInt32_262143));
                let k_col101 = ((k_mod_2_18_biased_tmp_42314_47.low().as_m31())
                    + (((k_mod_2_18_biased_tmp_42314_47.high().as_m31()) - (M31_2)) * (M31_65536)));
                *row[101] = k_col101;
                *sub_component_inputs.range_check_20[0] = [((k_col101) + (M31_524288))];
                *lookup_data.range_check_20_0 = [((k_col101) + (M31_524288))];
                let carry_0_col102 = (((conv_mod_tmp_42314_46[0]) - (k_col101)) * (M31_4194304));
                *row[102] = carry_0_col102;
                *sub_component_inputs.range_check_20_b[0] = [((carry_0_col102) + (M31_524288))];
                *lookup_data.range_check_20_b_0 = [((carry_0_col102) + (M31_524288))];
                let carry_1_col103 =
                    (((conv_mod_tmp_42314_46[1]) + (carry_0_col102)) * (M31_4194304));
                *row[103] = carry_1_col103;
                *sub_component_inputs.range_check_20_c[0] = [((carry_1_col103) + (M31_524288))];
                *lookup_data.range_check_20_c_0 = [((carry_1_col103) + (M31_524288))];
                let carry_2_col104 =
                    (((conv_mod_tmp_42314_46[2]) + (carry_1_col103)) * (M31_4194304));
                *row[104] = carry_2_col104;
                *sub_component_inputs.range_check_20_d[0] = [((carry_2_col104) + (M31_524288))];
                *lookup_data.range_check_20_d_0 = [((carry_2_col104) + (M31_524288))];
                let carry_3_col105 =
                    (((conv_mod_tmp_42314_46[3]) + (carry_2_col104)) * (M31_4194304));
                *row[105] = carry_3_col105;
                *sub_component_inputs.range_check_20_e[0] = [((carry_3_col105) + (M31_524288))];
                *lookup_data.range_check_20_e_0 = [((carry_3_col105) + (M31_524288))];
                let carry_4_col106 =
                    (((conv_mod_tmp_42314_46[4]) + (carry_3_col105)) * (M31_4194304));
                *row[106] = carry_4_col106;
                *sub_component_inputs.range_check_20_f[0] = [((carry_4_col106) + (M31_524288))];
                *lookup_data.range_check_20_f_0 = [((carry_4_col106) + (M31_524288))];
                let carry_5_col107 =
                    (((conv_mod_tmp_42314_46[5]) + (carry_4_col106)) * (M31_4194304));
                *row[107] = carry_5_col107;
                *sub_component_inputs.range_check_20_g[0] = [((carry_5_col107) + (M31_524288))];
                *lookup_data.range_check_20_g_0 = [((carry_5_col107) + (M31_524288))];
                let carry_6_col108 =
                    (((conv_mod_tmp_42314_46[6]) + (carry_5_col107)) * (M31_4194304));
                *row[108] = carry_6_col108;
                *sub_component_inputs.range_check_20_h[0] = [((carry_6_col108) + (M31_524288))];
                *lookup_data.range_check_20_h_0 = [((carry_6_col108) + (M31_524288))];
                let carry_7_col109 =
                    (((conv_mod_tmp_42314_46[7]) + (carry_6_col108)) * (M31_4194304));
                *row[109] = carry_7_col109;
                *sub_component_inputs.range_check_20[1] = [((carry_7_col109) + (M31_524288))];
                *lookup_data.range_check_20_1 = [((carry_7_col109) + (M31_524288))];
                let carry_8_col110 =
                    (((conv_mod_tmp_42314_46[8]) + (carry_7_col109)) * (M31_4194304));
                *row[110] = carry_8_col110;
                *sub_component_inputs.range_check_20_b[1] = [((carry_8_col110) + (M31_524288))];
                *lookup_data.range_check_20_b_1 = [((carry_8_col110) + (M31_524288))];
                let carry_9_col111 =
                    (((conv_mod_tmp_42314_46[9]) + (carry_8_col110)) * (M31_4194304));
                *row[111] = carry_9_col111;
                *sub_component_inputs.range_check_20_c[1] = [((carry_9_col111) + (M31_524288))];
                *lookup_data.range_check_20_c_1 = [((carry_9_col111) + (M31_524288))];
                let carry_10_col112 =
                    (((conv_mod_tmp_42314_46[10]) + (carry_9_col111)) * (M31_4194304));
                *row[112] = carry_10_col112;
                *sub_component_inputs.range_check_20_d[1] = [((carry_10_col112) + (M31_524288))];
                *lookup_data.range_check_20_d_1 = [((carry_10_col112) + (M31_524288))];
                let carry_11_col113 =
                    (((conv_mod_tmp_42314_46[11]) + (carry_10_col112)) * (M31_4194304));
                *row[113] = carry_11_col113;
                *sub_component_inputs.range_check_20_e[1] = [((carry_11_col113) + (M31_524288))];
                *lookup_data.range_check_20_e_1 = [((carry_11_col113) + (M31_524288))];
                let carry_12_col114 =
                    (((conv_mod_tmp_42314_46[12]) + (carry_11_col113)) * (M31_4194304));
                *row[114] = carry_12_col114;
                *sub_component_inputs.range_check_20_f[1] = [((carry_12_col114) + (M31_524288))];
                *lookup_data.range_check_20_f_1 = [((carry_12_col114) + (M31_524288))];
                let carry_13_col115 =
                    (((conv_mod_tmp_42314_46[13]) + (carry_12_col114)) * (M31_4194304));
                *row[115] = carry_13_col115;
                *sub_component_inputs.range_check_20_g[1] = [((carry_13_col115) + (M31_524288))];
                *lookup_data.range_check_20_g_1 = [((carry_13_col115) + (M31_524288))];
                let carry_14_col116 =
                    (((conv_mod_tmp_42314_46[14]) + (carry_13_col115)) * (M31_4194304));
                *row[116] = carry_14_col116;
                *sub_component_inputs.range_check_20_h[1] = [((carry_14_col116) + (M31_524288))];
                *lookup_data.range_check_20_h_1 = [((carry_14_col116) + (M31_524288))];
                let carry_15_col117 =
                    (((conv_mod_tmp_42314_46[15]) + (carry_14_col116)) * (M31_4194304));
                *row[117] = carry_15_col117;
                *sub_component_inputs.range_check_20[2] = [((carry_15_col117) + (M31_524288))];
                *lookup_data.range_check_20_2 = [((carry_15_col117) + (M31_524288))];
                let carry_16_col118 =
                    (((conv_mod_tmp_42314_46[16]) + (carry_15_col117)) * (M31_4194304));
                *row[118] = carry_16_col118;
                *sub_component_inputs.range_check_20_b[2] = [((carry_16_col118) + (M31_524288))];
                *lookup_data.range_check_20_b_2 = [((carry_16_col118) + (M31_524288))];
                let carry_17_col119 =
                    (((conv_mod_tmp_42314_46[17]) + (carry_16_col118)) * (M31_4194304));
                *row[119] = carry_17_col119;
                *sub_component_inputs.range_check_20_c[2] = [((carry_17_col119) + (M31_524288))];
                *lookup_data.range_check_20_c_2 = [((carry_17_col119) + (M31_524288))];
                let carry_18_col120 =
                    (((conv_mod_tmp_42314_46[18]) + (carry_17_col119)) * (M31_4194304));
                *row[120] = carry_18_col120;
                *sub_component_inputs.range_check_20_d[2] = [((carry_18_col120) + (M31_524288))];
                *lookup_data.range_check_20_d_2 = [((carry_18_col120) + (M31_524288))];
                let carry_19_col121 =
                    (((conv_mod_tmp_42314_46[19]) + (carry_18_col120)) * (M31_4194304));
                *row[121] = carry_19_col121;
                *sub_component_inputs.range_check_20_e[2] = [((carry_19_col121) + (M31_524288))];
                *lookup_data.range_check_20_e_2 = [((carry_19_col121) + (M31_524288))];
                let carry_20_col122 =
                    (((conv_mod_tmp_42314_46[20]) + (carry_19_col121)) * (M31_4194304));
                *row[122] = carry_20_col122;
                *sub_component_inputs.range_check_20_f[2] = [((carry_20_col122) + (M31_524288))];
                *lookup_data.range_check_20_f_2 = [((carry_20_col122) + (M31_524288))];
                let carry_21_col123 = ((((conv_mod_tmp_42314_46[21]) - ((M31_136) * (k_col101)))
                    + (carry_20_col122))
                    * (M31_4194304));
                *row[123] = carry_21_col123;
                *sub_component_inputs.range_check_20_g[2] = [((carry_21_col123) + (M31_524288))];
                *lookup_data.range_check_20_g_2 = [((carry_21_col123) + (M31_524288))];
                let carry_22_col124 =
                    (((conv_mod_tmp_42314_46[22]) + (carry_21_col123)) * (M31_4194304));
                *row[124] = carry_22_col124;
                *sub_component_inputs.range_check_20_h[2] = [((carry_22_col124) + (M31_524288))];
                *lookup_data.range_check_20_h_2 = [((carry_22_col124) + (M31_524288))];
                let carry_23_col125 =
                    (((conv_mod_tmp_42314_46[23]) + (carry_22_col124)) * (M31_4194304));
                *row[125] = carry_23_col125;
                *sub_component_inputs.range_check_20[3] = [((carry_23_col125) + (M31_524288))];
                *lookup_data.range_check_20_3 = [((carry_23_col125) + (M31_524288))];
                let carry_24_col126 =
                    (((conv_mod_tmp_42314_46[24]) + (carry_23_col125)) * (M31_4194304));
                *row[126] = carry_24_col126;
                *sub_component_inputs.range_check_20_b[3] = [((carry_24_col126) + (M31_524288))];
                *lookup_data.range_check_20_b_3 = [((carry_24_col126) + (M31_524288))];
                let carry_25_col127 =
                    (((conv_mod_tmp_42314_46[25]) + (carry_24_col126)) * (M31_4194304));
                *row[127] = carry_25_col127;
                *sub_component_inputs.range_check_20_c[3] = [((carry_25_col127) + (M31_524288))];
                *lookup_data.range_check_20_c_3 = [((carry_25_col127) + (M31_524288))];
                let carry_26_col128 =
                    (((conv_mod_tmp_42314_46[26]) + (carry_25_col127)) * (M31_4194304));
                *row[128] = carry_26_col128;
                *sub_component_inputs.range_check_20_d[3] = [((carry_26_col128) + (M31_524288))];
                *lookup_data.range_check_20_d_3 = [((carry_26_col128) + (M31_524288))];

                *lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];
                *lookup_data.opcodes_1 = [
                    (((input_pc_col0) + (M31_1)) + (op1_imm_col8)),
                    ((input_ap_col1) + (ap_update_add_1_col10)),
                    input_fp_col2,
                ];
                *row[129] = enabler_col.packed_at(row_index);
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
    range_check_20_0: Vec<[PackedM31; 1]>,
    range_check_20_1: Vec<[PackedM31; 1]>,
    range_check_20_2: Vec<[PackedM31; 1]>,
    range_check_20_3: Vec<[PackedM31; 1]>,
    range_check_20_b_0: Vec<[PackedM31; 1]>,
    range_check_20_b_1: Vec<[PackedM31; 1]>,
    range_check_20_b_2: Vec<[PackedM31; 1]>,
    range_check_20_b_3: Vec<[PackedM31; 1]>,
    range_check_20_c_0: Vec<[PackedM31; 1]>,
    range_check_20_c_1: Vec<[PackedM31; 1]>,
    range_check_20_c_2: Vec<[PackedM31; 1]>,
    range_check_20_c_3: Vec<[PackedM31; 1]>,
    range_check_20_d_0: Vec<[PackedM31; 1]>,
    range_check_20_d_1: Vec<[PackedM31; 1]>,
    range_check_20_d_2: Vec<[PackedM31; 1]>,
    range_check_20_d_3: Vec<[PackedM31; 1]>,
    range_check_20_e_0: Vec<[PackedM31; 1]>,
    range_check_20_e_1: Vec<[PackedM31; 1]>,
    range_check_20_e_2: Vec<[PackedM31; 1]>,
    range_check_20_f_0: Vec<[PackedM31; 1]>,
    range_check_20_f_1: Vec<[PackedM31; 1]>,
    range_check_20_f_2: Vec<[PackedM31; 1]>,
    range_check_20_g_0: Vec<[PackedM31; 1]>,
    range_check_20_g_1: Vec<[PackedM31; 1]>,
    range_check_20_g_2: Vec<[PackedM31; 1]>,
    range_check_20_h_0: Vec<[PackedM31; 1]>,
    range_check_20_h_1: Vec<[PackedM31; 1]>,
    range_check_20_h_2: Vec<[PackedM31; 1]>,
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
        range_check_20: &relations::RangeCheck_20,
        range_check_20_b: &relations::RangeCheck_20_B,
        range_check_20_c: &relations::RangeCheck_20_C,
        range_check_20_d: &relations::RangeCheck_20_D,
        range_check_20_e: &relations::RangeCheck_20_E,
        range_check_20_f: &relations::RangeCheck_20_F,
        range_check_20_g: &relations::RangeCheck_20_G,
        range_check_20_h: &relations::RangeCheck_20_H,
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
            &self.lookup_data.range_check_20_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = range_check_20.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_b_0,
            &self.lookup_data.range_check_20_c_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_b.combine(values0);
                let denom1: PackedQM31 = range_check_20_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_d_0,
            &self.lookup_data.range_check_20_e_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_d.combine(values0);
                let denom1: PackedQM31 = range_check_20_e.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_f_0,
            &self.lookup_data.range_check_20_g_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_f.combine(values0);
                let denom1: PackedQM31 = range_check_20_g.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_h_0,
            &self.lookup_data.range_check_20_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_h.combine(values0);
                let denom1: PackedQM31 = range_check_20.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_b_1,
            &self.lookup_data.range_check_20_c_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_b.combine(values0);
                let denom1: PackedQM31 = range_check_20_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_d_1,
            &self.lookup_data.range_check_20_e_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_d.combine(values0);
                let denom1: PackedQM31 = range_check_20_e.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_f_1,
            &self.lookup_data.range_check_20_g_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_f.combine(values0);
                let denom1: PackedQM31 = range_check_20_g.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_h_1,
            &self.lookup_data.range_check_20_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_h.combine(values0);
                let denom1: PackedQM31 = range_check_20.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_b_2,
            &self.lookup_data.range_check_20_c_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_b.combine(values0);
                let denom1: PackedQM31 = range_check_20_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_d_2,
            &self.lookup_data.range_check_20_e_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_d.combine(values0);
                let denom1: PackedQM31 = range_check_20_e.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_f_2,
            &self.lookup_data.range_check_20_g_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_f.combine(values0);
                let denom1: PackedQM31 = range_check_20_g.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_h_2,
            &self.lookup_data.range_check_20_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_h.combine(values0);
                let denom1: PackedQM31 = range_check_20.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_b_3,
            &self.lookup_data.range_check_20_c_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_20_b.combine(values0);
                let denom1: PackedQM31 = range_check_20_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_20_d_3,
            &self.lookup_data.opcodes_0,
        )
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (writer, values0, values1))| {
                let denom0: PackedQM31 = range_check_20_d.combine(values0);
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
