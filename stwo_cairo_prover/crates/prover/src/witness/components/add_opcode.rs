// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::add_opcode::{Claim, InteractionClaim, N_TRACE_COLUMNS};

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
            |(row_index, (row, lookup_data, sub_component_inputs, add_opcode_input))| {
                let input_pc_col0 = add_opcode_input.pc;
                *row[0] = input_pc_col0;
                let input_ap_col1 = add_opcode_input.ap;
                *row[1] = input_ap_col1;
                let input_fp_col2 = add_opcode_input.fp;
                *row[2] = input_fp_col2;

                // Decode Instruction.

                let memory_address_to_id_value_tmp_3fa46_0 =
                    memory_address_to_id_state.deduce_output(input_pc_col0);
                let memory_id_to_big_value_tmp_3fa46_1 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_3fa46_0);
                let offset0_tmp_3fa46_2 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_3fa46_1.get_m31(0)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_3fa46_1.get_m31(1),
                        )) & (UInt16_127))
                            << (UInt16_9)));
                let offset0_col3 = offset0_tmp_3fa46_2.as_m31();
                *row[3] = offset0_col3;
                let offset1_tmp_3fa46_3 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_3fa46_1.get_m31(1)))
                        >> (UInt16_7))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_3fa46_1.get_m31(2),
                        )) << (UInt16_2)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_3fa46_1.get_m31(3),
                        )) & (UInt16_31))
                            << (UInt16_11)));
                let offset1_col4 = offset1_tmp_3fa46_3.as_m31();
                *row[4] = offset1_col4;
                let offset2_tmp_3fa46_4 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_3fa46_1.get_m31(3)))
                        >> (UInt16_5))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_3fa46_1.get_m31(4),
                        )) << (UInt16_4)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_3fa46_1.get_m31(5),
                        )) & (UInt16_7))
                            << (UInt16_13)));
                let offset2_col5 = offset2_tmp_3fa46_4.as_m31();
                *row[5] = offset2_col5;
                let dst_base_fp_tmp_3fa46_5 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_3fa46_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_3fa46_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_0))
                        & (UInt16_1));
                let dst_base_fp_col6 = dst_base_fp_tmp_3fa46_5.as_m31();
                *row[6] = dst_base_fp_col6;
                let op0_base_fp_tmp_3fa46_6 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_3fa46_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_3fa46_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_1))
                        & (UInt16_1));
                let op0_base_fp_col7 = op0_base_fp_tmp_3fa46_6.as_m31();
                *row[7] = op0_base_fp_col7;
                let op1_imm_tmp_3fa46_7 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_3fa46_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_3fa46_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_2))
                        & (UInt16_1));
                let op1_imm_col8 = op1_imm_tmp_3fa46_7.as_m31();
                *row[8] = op1_imm_col8;
                let op1_base_fp_tmp_3fa46_8 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_3fa46_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_3fa46_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_3))
                        & (UInt16_1));
                let op1_base_fp_col9 = op1_base_fp_tmp_3fa46_8.as_m31();
                *row[9] = op1_base_fp_col9;
                let op1_base_ap_tmp_3fa46_9 = (((M31_1) - (op1_imm_col8)) - (op1_base_fp_col9));
                let ap_update_add_1_tmp_3fa46_10 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_3fa46_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_3fa46_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_11))
                        & (UInt16_1));
                let ap_update_add_1_col10 = ap_update_add_1_tmp_3fa46_10.as_m31();
                *row[10] = ap_update_add_1_col10;
                *sub_component_inputs.verify_instruction[0] = (
                    input_pc_col0,
                    [offset0_col3, offset1_col4, offset2_col5],
                    [
                        (((((((dst_base_fp_col6) * (M31_8)) + ((op0_base_fp_col7) * (M31_16)))
                            + ((op1_imm_col8) * (M31_32)))
                            + ((op1_base_fp_col9) * (M31_64)))
                            + ((op1_base_ap_tmp_3fa46_9) * (M31_128)))
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
                        + ((op1_base_ap_tmp_3fa46_9) * (M31_128)))
                        + (M31_256)),
                    (((ap_update_add_1_col10) * (M31_32)) + (M31_256)),
                    M31_0,
                ];
                let decode_instruction_bc3cd_output_tmp_3fa46_11 = (
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
                        op1_base_ap_tmp_3fa46_9,
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
                    + ((decode_instruction_bc3cd_output_tmp_3fa46_11.1[4]) * (input_ap_col1)));
                *row[13] = mem1_base_col13;

                // Read Positive Num Bits 252.

                // Read Id.

                let memory_address_to_id_value_tmp_3fa46_12 = memory_address_to_id_state
                    .deduce_output(
                        ((mem_dst_base_col11)
                            + (decode_instruction_bc3cd_output_tmp_3fa46_11.0[0])),
                    );
                let dst_id_col14 = memory_address_to_id_value_tmp_3fa46_12;
                *row[14] = dst_id_col14;
                *sub_component_inputs.memory_address_to_id[0] =
                    ((mem_dst_base_col11) + (decode_instruction_bc3cd_output_tmp_3fa46_11.0[0]));
                *lookup_data.memory_address_to_id_0 = [
                    ((mem_dst_base_col11) + (decode_instruction_bc3cd_output_tmp_3fa46_11.0[0])),
                    dst_id_col14,
                ];

                // Read Positive Known Id Num Bits 252.

                let memory_id_to_big_value_tmp_3fa46_14 =
                    memory_id_to_big_state.deduce_output(dst_id_col14);
                let dst_limb_0_col15 = memory_id_to_big_value_tmp_3fa46_14.get_m31(0);
                *row[15] = dst_limb_0_col15;
                let dst_limb_1_col16 = memory_id_to_big_value_tmp_3fa46_14.get_m31(1);
                *row[16] = dst_limb_1_col16;
                let dst_limb_2_col17 = memory_id_to_big_value_tmp_3fa46_14.get_m31(2);
                *row[17] = dst_limb_2_col17;
                let dst_limb_3_col18 = memory_id_to_big_value_tmp_3fa46_14.get_m31(3);
                *row[18] = dst_limb_3_col18;
                let dst_limb_4_col19 = memory_id_to_big_value_tmp_3fa46_14.get_m31(4);
                *row[19] = dst_limb_4_col19;
                let dst_limb_5_col20 = memory_id_to_big_value_tmp_3fa46_14.get_m31(5);
                *row[20] = dst_limb_5_col20;
                let dst_limb_6_col21 = memory_id_to_big_value_tmp_3fa46_14.get_m31(6);
                *row[21] = dst_limb_6_col21;
                let dst_limb_7_col22 = memory_id_to_big_value_tmp_3fa46_14.get_m31(7);
                *row[22] = dst_limb_7_col22;
                let dst_limb_8_col23 = memory_id_to_big_value_tmp_3fa46_14.get_m31(8);
                *row[23] = dst_limb_8_col23;
                let dst_limb_9_col24 = memory_id_to_big_value_tmp_3fa46_14.get_m31(9);
                *row[24] = dst_limb_9_col24;
                let dst_limb_10_col25 = memory_id_to_big_value_tmp_3fa46_14.get_m31(10);
                *row[25] = dst_limb_10_col25;
                let dst_limb_11_col26 = memory_id_to_big_value_tmp_3fa46_14.get_m31(11);
                *row[26] = dst_limb_11_col26;
                let dst_limb_12_col27 = memory_id_to_big_value_tmp_3fa46_14.get_m31(12);
                *row[27] = dst_limb_12_col27;
                let dst_limb_13_col28 = memory_id_to_big_value_tmp_3fa46_14.get_m31(13);
                *row[28] = dst_limb_13_col28;
                let dst_limb_14_col29 = memory_id_to_big_value_tmp_3fa46_14.get_m31(14);
                *row[29] = dst_limb_14_col29;
                let dst_limb_15_col30 = memory_id_to_big_value_tmp_3fa46_14.get_m31(15);
                *row[30] = dst_limb_15_col30;
                let dst_limb_16_col31 = memory_id_to_big_value_tmp_3fa46_14.get_m31(16);
                *row[31] = dst_limb_16_col31;
                let dst_limb_17_col32 = memory_id_to_big_value_tmp_3fa46_14.get_m31(17);
                *row[32] = dst_limb_17_col32;
                let dst_limb_18_col33 = memory_id_to_big_value_tmp_3fa46_14.get_m31(18);
                *row[33] = dst_limb_18_col33;
                let dst_limb_19_col34 = memory_id_to_big_value_tmp_3fa46_14.get_m31(19);
                *row[34] = dst_limb_19_col34;
                let dst_limb_20_col35 = memory_id_to_big_value_tmp_3fa46_14.get_m31(20);
                *row[35] = dst_limb_20_col35;
                let dst_limb_21_col36 = memory_id_to_big_value_tmp_3fa46_14.get_m31(21);
                *row[36] = dst_limb_21_col36;
                let dst_limb_22_col37 = memory_id_to_big_value_tmp_3fa46_14.get_m31(22);
                *row[37] = dst_limb_22_col37;
                let dst_limb_23_col38 = memory_id_to_big_value_tmp_3fa46_14.get_m31(23);
                *row[38] = dst_limb_23_col38;
                let dst_limb_24_col39 = memory_id_to_big_value_tmp_3fa46_14.get_m31(24);
                *row[39] = dst_limb_24_col39;
                let dst_limb_25_col40 = memory_id_to_big_value_tmp_3fa46_14.get_m31(25);
                *row[40] = dst_limb_25_col40;
                let dst_limb_26_col41 = memory_id_to_big_value_tmp_3fa46_14.get_m31(26);
                *row[41] = dst_limb_26_col41;
                let dst_limb_27_col42 = memory_id_to_big_value_tmp_3fa46_14.get_m31(27);
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
                let read_positive_known_id_num_bits_252_output_tmp_3fa46_15 =
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

                let read_positive_num_bits_252_output_tmp_3fa46_16 = (
                    read_positive_known_id_num_bits_252_output_tmp_3fa46_15,
                    dst_id_col14,
                );

                // Read Positive Num Bits 252.

                // Read Id.

                let memory_address_to_id_value_tmp_3fa46_17 = memory_address_to_id_state
                    .deduce_output(
                        ((mem0_base_col12) + (decode_instruction_bc3cd_output_tmp_3fa46_11.0[1])),
                    );
                let op0_id_col43 = memory_address_to_id_value_tmp_3fa46_17;
                *row[43] = op0_id_col43;
                *sub_component_inputs.memory_address_to_id[1] =
                    ((mem0_base_col12) + (decode_instruction_bc3cd_output_tmp_3fa46_11.0[1]));
                *lookup_data.memory_address_to_id_1 = [
                    ((mem0_base_col12) + (decode_instruction_bc3cd_output_tmp_3fa46_11.0[1])),
                    op0_id_col43,
                ];

                // Read Positive Known Id Num Bits 252.

                let memory_id_to_big_value_tmp_3fa46_19 =
                    memory_id_to_big_state.deduce_output(op0_id_col43);
                let op0_limb_0_col44 = memory_id_to_big_value_tmp_3fa46_19.get_m31(0);
                *row[44] = op0_limb_0_col44;
                let op0_limb_1_col45 = memory_id_to_big_value_tmp_3fa46_19.get_m31(1);
                *row[45] = op0_limb_1_col45;
                let op0_limb_2_col46 = memory_id_to_big_value_tmp_3fa46_19.get_m31(2);
                *row[46] = op0_limb_2_col46;
                let op0_limb_3_col47 = memory_id_to_big_value_tmp_3fa46_19.get_m31(3);
                *row[47] = op0_limb_3_col47;
                let op0_limb_4_col48 = memory_id_to_big_value_tmp_3fa46_19.get_m31(4);
                *row[48] = op0_limb_4_col48;
                let op0_limb_5_col49 = memory_id_to_big_value_tmp_3fa46_19.get_m31(5);
                *row[49] = op0_limb_5_col49;
                let op0_limb_6_col50 = memory_id_to_big_value_tmp_3fa46_19.get_m31(6);
                *row[50] = op0_limb_6_col50;
                let op0_limb_7_col51 = memory_id_to_big_value_tmp_3fa46_19.get_m31(7);
                *row[51] = op0_limb_7_col51;
                let op0_limb_8_col52 = memory_id_to_big_value_tmp_3fa46_19.get_m31(8);
                *row[52] = op0_limb_8_col52;
                let op0_limb_9_col53 = memory_id_to_big_value_tmp_3fa46_19.get_m31(9);
                *row[53] = op0_limb_9_col53;
                let op0_limb_10_col54 = memory_id_to_big_value_tmp_3fa46_19.get_m31(10);
                *row[54] = op0_limb_10_col54;
                let op0_limb_11_col55 = memory_id_to_big_value_tmp_3fa46_19.get_m31(11);
                *row[55] = op0_limb_11_col55;
                let op0_limb_12_col56 = memory_id_to_big_value_tmp_3fa46_19.get_m31(12);
                *row[56] = op0_limb_12_col56;
                let op0_limb_13_col57 = memory_id_to_big_value_tmp_3fa46_19.get_m31(13);
                *row[57] = op0_limb_13_col57;
                let op0_limb_14_col58 = memory_id_to_big_value_tmp_3fa46_19.get_m31(14);
                *row[58] = op0_limb_14_col58;
                let op0_limb_15_col59 = memory_id_to_big_value_tmp_3fa46_19.get_m31(15);
                *row[59] = op0_limb_15_col59;
                let op0_limb_16_col60 = memory_id_to_big_value_tmp_3fa46_19.get_m31(16);
                *row[60] = op0_limb_16_col60;
                let op0_limb_17_col61 = memory_id_to_big_value_tmp_3fa46_19.get_m31(17);
                *row[61] = op0_limb_17_col61;
                let op0_limb_18_col62 = memory_id_to_big_value_tmp_3fa46_19.get_m31(18);
                *row[62] = op0_limb_18_col62;
                let op0_limb_19_col63 = memory_id_to_big_value_tmp_3fa46_19.get_m31(19);
                *row[63] = op0_limb_19_col63;
                let op0_limb_20_col64 = memory_id_to_big_value_tmp_3fa46_19.get_m31(20);
                *row[64] = op0_limb_20_col64;
                let op0_limb_21_col65 = memory_id_to_big_value_tmp_3fa46_19.get_m31(21);
                *row[65] = op0_limb_21_col65;
                let op0_limb_22_col66 = memory_id_to_big_value_tmp_3fa46_19.get_m31(22);
                *row[66] = op0_limb_22_col66;
                let op0_limb_23_col67 = memory_id_to_big_value_tmp_3fa46_19.get_m31(23);
                *row[67] = op0_limb_23_col67;
                let op0_limb_24_col68 = memory_id_to_big_value_tmp_3fa46_19.get_m31(24);
                *row[68] = op0_limb_24_col68;
                let op0_limb_25_col69 = memory_id_to_big_value_tmp_3fa46_19.get_m31(25);
                *row[69] = op0_limb_25_col69;
                let op0_limb_26_col70 = memory_id_to_big_value_tmp_3fa46_19.get_m31(26);
                *row[70] = op0_limb_26_col70;
                let op0_limb_27_col71 = memory_id_to_big_value_tmp_3fa46_19.get_m31(27);
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
                let read_positive_known_id_num_bits_252_output_tmp_3fa46_20 =
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

                let read_positive_num_bits_252_output_tmp_3fa46_21 = (
                    read_positive_known_id_num_bits_252_output_tmp_3fa46_20,
                    op0_id_col43,
                );

                // Read Positive Num Bits 252.

                // Read Id.

                let memory_address_to_id_value_tmp_3fa46_22 = memory_address_to_id_state
                    .deduce_output(
                        ((mem1_base_col13) + (decode_instruction_bc3cd_output_tmp_3fa46_11.0[2])),
                    );
                let op1_id_col72 = memory_address_to_id_value_tmp_3fa46_22;
                *row[72] = op1_id_col72;
                *sub_component_inputs.memory_address_to_id[2] =
                    ((mem1_base_col13) + (decode_instruction_bc3cd_output_tmp_3fa46_11.0[2]));
                *lookup_data.memory_address_to_id_2 = [
                    ((mem1_base_col13) + (decode_instruction_bc3cd_output_tmp_3fa46_11.0[2])),
                    op1_id_col72,
                ];

                // Read Positive Known Id Num Bits 252.

                let memory_id_to_big_value_tmp_3fa46_24 =
                    memory_id_to_big_state.deduce_output(op1_id_col72);
                let op1_limb_0_col73 = memory_id_to_big_value_tmp_3fa46_24.get_m31(0);
                *row[73] = op1_limb_0_col73;
                let op1_limb_1_col74 = memory_id_to_big_value_tmp_3fa46_24.get_m31(1);
                *row[74] = op1_limb_1_col74;
                let op1_limb_2_col75 = memory_id_to_big_value_tmp_3fa46_24.get_m31(2);
                *row[75] = op1_limb_2_col75;
                let op1_limb_3_col76 = memory_id_to_big_value_tmp_3fa46_24.get_m31(3);
                *row[76] = op1_limb_3_col76;
                let op1_limb_4_col77 = memory_id_to_big_value_tmp_3fa46_24.get_m31(4);
                *row[77] = op1_limb_4_col77;
                let op1_limb_5_col78 = memory_id_to_big_value_tmp_3fa46_24.get_m31(5);
                *row[78] = op1_limb_5_col78;
                let op1_limb_6_col79 = memory_id_to_big_value_tmp_3fa46_24.get_m31(6);
                *row[79] = op1_limb_6_col79;
                let op1_limb_7_col80 = memory_id_to_big_value_tmp_3fa46_24.get_m31(7);
                *row[80] = op1_limb_7_col80;
                let op1_limb_8_col81 = memory_id_to_big_value_tmp_3fa46_24.get_m31(8);
                *row[81] = op1_limb_8_col81;
                let op1_limb_9_col82 = memory_id_to_big_value_tmp_3fa46_24.get_m31(9);
                *row[82] = op1_limb_9_col82;
                let op1_limb_10_col83 = memory_id_to_big_value_tmp_3fa46_24.get_m31(10);
                *row[83] = op1_limb_10_col83;
                let op1_limb_11_col84 = memory_id_to_big_value_tmp_3fa46_24.get_m31(11);
                *row[84] = op1_limb_11_col84;
                let op1_limb_12_col85 = memory_id_to_big_value_tmp_3fa46_24.get_m31(12);
                *row[85] = op1_limb_12_col85;
                let op1_limb_13_col86 = memory_id_to_big_value_tmp_3fa46_24.get_m31(13);
                *row[86] = op1_limb_13_col86;
                let op1_limb_14_col87 = memory_id_to_big_value_tmp_3fa46_24.get_m31(14);
                *row[87] = op1_limb_14_col87;
                let op1_limb_15_col88 = memory_id_to_big_value_tmp_3fa46_24.get_m31(15);
                *row[88] = op1_limb_15_col88;
                let op1_limb_16_col89 = memory_id_to_big_value_tmp_3fa46_24.get_m31(16);
                *row[89] = op1_limb_16_col89;
                let op1_limb_17_col90 = memory_id_to_big_value_tmp_3fa46_24.get_m31(17);
                *row[90] = op1_limb_17_col90;
                let op1_limb_18_col91 = memory_id_to_big_value_tmp_3fa46_24.get_m31(18);
                *row[91] = op1_limb_18_col91;
                let op1_limb_19_col92 = memory_id_to_big_value_tmp_3fa46_24.get_m31(19);
                *row[92] = op1_limb_19_col92;
                let op1_limb_20_col93 = memory_id_to_big_value_tmp_3fa46_24.get_m31(20);
                *row[93] = op1_limb_20_col93;
                let op1_limb_21_col94 = memory_id_to_big_value_tmp_3fa46_24.get_m31(21);
                *row[94] = op1_limb_21_col94;
                let op1_limb_22_col95 = memory_id_to_big_value_tmp_3fa46_24.get_m31(22);
                *row[95] = op1_limb_22_col95;
                let op1_limb_23_col96 = memory_id_to_big_value_tmp_3fa46_24.get_m31(23);
                *row[96] = op1_limb_23_col96;
                let op1_limb_24_col97 = memory_id_to_big_value_tmp_3fa46_24.get_m31(24);
                *row[97] = op1_limb_24_col97;
                let op1_limb_25_col98 = memory_id_to_big_value_tmp_3fa46_24.get_m31(25);
                *row[98] = op1_limb_25_col98;
                let op1_limb_26_col99 = memory_id_to_big_value_tmp_3fa46_24.get_m31(26);
                *row[99] = op1_limb_26_col99;
                let op1_limb_27_col100 = memory_id_to_big_value_tmp_3fa46_24.get_m31(27);
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
                let read_positive_known_id_num_bits_252_output_tmp_3fa46_25 =
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

                let read_positive_num_bits_252_output_tmp_3fa46_26 = (
                    read_positive_known_id_num_bits_252_output_tmp_3fa46_25,
                    op1_id_col72,
                );

                // Verify Add 252.

                let sub_p_bit_tmp_3fa46_27 = ((UInt16_1)
                    & (((PackedUInt16::from_m31(op0_limb_0_col44))
                        ^ (PackedUInt16::from_m31(op1_limb_0_col73)))
                        ^ (PackedUInt16::from_m31(dst_limb_0_col15))));
                let sub_p_bit_col101 = sub_p_bit_tmp_3fa46_27.as_m31();
                *row[101] = sub_p_bit_col101;

                *lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];
                *lookup_data.opcodes_1 = [
                    (((input_pc_col0) + (M31_1)) + (op1_imm_col8)),
                    ((input_ap_col1) + (ap_update_add_1_col10)),
                    input_fp_col2,
                ];
                *row[102] = enabler_col.packed_at(row_index);
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
