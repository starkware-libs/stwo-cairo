#![allow(unused_parens)]
use cairo_air::components::mul_opcode_imm::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{
    memory_address_to_id, memory_id_to_big, range_check_19, verify_instruction,
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
        range_check_19_state: &range_check_19::ClaimGenerator,
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
            range_check_19_state,
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
            .range_check_19
            .iter()
            .for_each(|inputs| {
                range_check_19_state.add_packed_inputs(inputs);
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
    range_check_19: [Vec<range_check_19::PackedInputType>; 28],
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
    range_check_19_state: &range_check_19::ClaimGenerator,
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
    let M31_131072 = PackedM31::broadcast(M31::from(131072));
    let M31_134217728 = PackedM31::broadcast(M31::from(134217728));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_16 = PackedM31::broadcast(M31::from(16));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_32769 = PackedM31::broadcast(M31::from(32769));
    let M31_4 = PackedM31::broadcast(M31::from(4));
    let M31_4194304 = PackedM31::broadcast(M31::from(4194304));
    let M31_64 = PackedM31::broadcast(M31::from(64));
    let M31_65536 = PackedM31::broadcast(M31::from(65536));
    let M31_8 = PackedM31::broadcast(M31::from(8));
    let UInt16_0 = PackedUInt16::broadcast(UInt16::from(0));
    let UInt16_1 = PackedUInt16::broadcast(UInt16::from(1));
    let UInt16_11 = PackedUInt16::broadcast(UInt16::from(11));
    let UInt16_127 = PackedUInt16::broadcast(UInt16::from(127));
    let UInt16_2 = PackedUInt16::broadcast(UInt16::from(2));
    let UInt16_3 = PackedUInt16::broadcast(UInt16::from(3));
    let UInt16_31 = PackedUInt16::broadcast(UInt16::from(31));
    let UInt16_6 = PackedUInt16::broadcast(UInt16::from(6));
    let UInt16_7 = PackedUInt16::broadcast(UInt16::from(7));
    let UInt16_9 = PackedUInt16::broadcast(UInt16::from(9));
    let UInt32_262143 = PackedUInt32::broadcast(UInt32::from(262143));
    let UInt32_511 = PackedUInt32::broadcast(UInt32::from(511));
    let UInt32_65536 = PackedUInt32::broadcast(UInt32::from(65536));
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
            |(row_index, (mut row, lookup_data, sub_component_inputs, mul_opcode_imm_input))| {
                let input_pc_col0 = mul_opcode_imm_input.pc;
                *row[0] = input_pc_col0;
                let input_ap_col1 = mul_opcode_imm_input.ap;
                *row[1] = input_ap_col1;
                let input_fp_col2 = mul_opcode_imm_input.fp;
                *row[2] = input_fp_col2;

                // Decode Instruction.

                let memory_address_to_id_value_tmp_48d52_0 =
                    memory_address_to_id_state.deduce_output(input_pc_col0);
                let memory_id_to_big_value_tmp_48d52_1 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_48d52_0);
                let offset0_tmp_48d52_2 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_48d52_1.get_m31(0)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_48d52_1.get_m31(1),
                        )) & (UInt16_127))
                            << (UInt16_9)));
                let offset0_col3 = offset0_tmp_48d52_2.as_m31();
                *row[3] = offset0_col3;
                let offset1_tmp_48d52_3 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_48d52_1.get_m31(1)))
                        >> (UInt16_7))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_48d52_1.get_m31(2),
                        )) << (UInt16_2)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_48d52_1.get_m31(3),
                        )) & (UInt16_31))
                            << (UInt16_11)));
                let offset1_col4 = offset1_tmp_48d52_3.as_m31();
                *row[4] = offset1_col4;
                let dst_base_fp_tmp_48d52_4 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_48d52_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_48d52_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_0))
                        & (UInt16_1));
                let dst_base_fp_col5 = dst_base_fp_tmp_48d52_4.as_m31();
                *row[5] = dst_base_fp_col5;
                let op0_base_fp_tmp_48d52_5 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_48d52_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_48d52_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_1))
                        & (UInt16_1));
                let op0_base_fp_col6 = op0_base_fp_tmp_48d52_5.as_m31();
                *row[6] = op0_base_fp_col6;
                let ap_update_add_1_tmp_48d52_6 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_48d52_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_48d52_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_11))
                        & (UInt16_1));
                let ap_update_add_1_col7 = ap_update_add_1_tmp_48d52_6.as_m31();
                *row[7] = ap_update_add_1_col7;
                *sub_component_inputs.verify_instruction[0] = (
                    input_pc_col0,
                    [offset0_col3, offset1_col4, M31_32769],
                    [
                        ((((dst_base_fp_col5) * (M31_8)) + ((op0_base_fp_col6) * (M31_16)))
                            + (M31_32)),
                        (((M31_1) + ((ap_update_add_1_col7) * (M31_32))) + (M31_256)),
                    ],
                    M31_0,
                );
                *lookup_data.verify_instruction_0 = [
                    input_pc_col0,
                    offset0_col3,
                    offset1_col4,
                    M31_32769,
                    ((((dst_base_fp_col5) * (M31_8)) + ((op0_base_fp_col6) * (M31_16))) + (M31_32)),
                    (((M31_1) + ((ap_update_add_1_col7) * (M31_32))) + (M31_256)),
                    M31_0,
                ];
                let decode_instruction_db26c85482ebf3d9_output_tmp_48d52_7 = (
                    [
                        ((offset0_col3) - (M31_32768)),
                        ((offset1_col4) - (M31_32768)),
                        M31_1,
                    ],
                    [
                        dst_base_fp_col5,
                        op0_base_fp_col6,
                        M31_1,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_1,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        ap_update_add_1_col7,
                        M31_0,
                        M31_0,
                        M31_1,
                    ],
                    M31_0,
                );

                let mem_dst_base_col8 = (((dst_base_fp_col5) * (input_fp_col2))
                    + (((M31_1) - (dst_base_fp_col5)) * (input_ap_col1)));
                *row[8] = mem_dst_base_col8;
                let mem0_base_col9 = (((op0_base_fp_col6) * (input_fp_col2))
                    + (((M31_1) - (op0_base_fp_col6)) * (input_ap_col1)));
                *row[9] = mem0_base_col9;

                // Read Positive Num Bits 252.

                let memory_address_to_id_value_tmp_48d52_8 = memory_address_to_id_state
                    .deduce_output(
                        ((mem_dst_base_col8)
                            + (decode_instruction_db26c85482ebf3d9_output_tmp_48d52_7.0[0])),
                    );
                let memory_id_to_big_value_tmp_48d52_9 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_48d52_8);
                let dst_id_col10 = memory_address_to_id_value_tmp_48d52_8;
                *row[10] = dst_id_col10;
                *sub_component_inputs.memory_address_to_id[0] = ((mem_dst_base_col8)
                    + (decode_instruction_db26c85482ebf3d9_output_tmp_48d52_7.0[0]));
                *lookup_data.memory_address_to_id_0 = [
                    ((mem_dst_base_col8)
                        + (decode_instruction_db26c85482ebf3d9_output_tmp_48d52_7.0[0])),
                    dst_id_col10,
                ];
                let dst_limb_0_col11 = memory_id_to_big_value_tmp_48d52_9.get_m31(0);
                *row[11] = dst_limb_0_col11;
                let dst_limb_1_col12 = memory_id_to_big_value_tmp_48d52_9.get_m31(1);
                *row[12] = dst_limb_1_col12;
                let dst_limb_2_col13 = memory_id_to_big_value_tmp_48d52_9.get_m31(2);
                *row[13] = dst_limb_2_col13;
                let dst_limb_3_col14 = memory_id_to_big_value_tmp_48d52_9.get_m31(3);
                *row[14] = dst_limb_3_col14;
                let dst_limb_4_col15 = memory_id_to_big_value_tmp_48d52_9.get_m31(4);
                *row[15] = dst_limb_4_col15;
                let dst_limb_5_col16 = memory_id_to_big_value_tmp_48d52_9.get_m31(5);
                *row[16] = dst_limb_5_col16;
                let dst_limb_6_col17 = memory_id_to_big_value_tmp_48d52_9.get_m31(6);
                *row[17] = dst_limb_6_col17;
                let dst_limb_7_col18 = memory_id_to_big_value_tmp_48d52_9.get_m31(7);
                *row[18] = dst_limb_7_col18;
                let dst_limb_8_col19 = memory_id_to_big_value_tmp_48d52_9.get_m31(8);
                *row[19] = dst_limb_8_col19;
                let dst_limb_9_col20 = memory_id_to_big_value_tmp_48d52_9.get_m31(9);
                *row[20] = dst_limb_9_col20;
                let dst_limb_10_col21 = memory_id_to_big_value_tmp_48d52_9.get_m31(10);
                *row[21] = dst_limb_10_col21;
                let dst_limb_11_col22 = memory_id_to_big_value_tmp_48d52_9.get_m31(11);
                *row[22] = dst_limb_11_col22;
                let dst_limb_12_col23 = memory_id_to_big_value_tmp_48d52_9.get_m31(12);
                *row[23] = dst_limb_12_col23;
                let dst_limb_13_col24 = memory_id_to_big_value_tmp_48d52_9.get_m31(13);
                *row[24] = dst_limb_13_col24;
                let dst_limb_14_col25 = memory_id_to_big_value_tmp_48d52_9.get_m31(14);
                *row[25] = dst_limb_14_col25;
                let dst_limb_15_col26 = memory_id_to_big_value_tmp_48d52_9.get_m31(15);
                *row[26] = dst_limb_15_col26;
                let dst_limb_16_col27 = memory_id_to_big_value_tmp_48d52_9.get_m31(16);
                *row[27] = dst_limb_16_col27;
                let dst_limb_17_col28 = memory_id_to_big_value_tmp_48d52_9.get_m31(17);
                *row[28] = dst_limb_17_col28;
                let dst_limb_18_col29 = memory_id_to_big_value_tmp_48d52_9.get_m31(18);
                *row[29] = dst_limb_18_col29;
                let dst_limb_19_col30 = memory_id_to_big_value_tmp_48d52_9.get_m31(19);
                *row[30] = dst_limb_19_col30;
                let dst_limb_20_col31 = memory_id_to_big_value_tmp_48d52_9.get_m31(20);
                *row[31] = dst_limb_20_col31;
                let dst_limb_21_col32 = memory_id_to_big_value_tmp_48d52_9.get_m31(21);
                *row[32] = dst_limb_21_col32;
                let dst_limb_22_col33 = memory_id_to_big_value_tmp_48d52_9.get_m31(22);
                *row[33] = dst_limb_22_col33;
                let dst_limb_23_col34 = memory_id_to_big_value_tmp_48d52_9.get_m31(23);
                *row[34] = dst_limb_23_col34;
                let dst_limb_24_col35 = memory_id_to_big_value_tmp_48d52_9.get_m31(24);
                *row[35] = dst_limb_24_col35;
                let dst_limb_25_col36 = memory_id_to_big_value_tmp_48d52_9.get_m31(25);
                *row[36] = dst_limb_25_col36;
                let dst_limb_26_col37 = memory_id_to_big_value_tmp_48d52_9.get_m31(26);
                *row[37] = dst_limb_26_col37;
                let dst_limb_27_col38 = memory_id_to_big_value_tmp_48d52_9.get_m31(27);
                *row[38] = dst_limb_27_col38;
                *sub_component_inputs.memory_id_to_big[0] = dst_id_col10;
                *lookup_data.memory_id_to_big_0 = [
                    dst_id_col10,
                    dst_limb_0_col11,
                    dst_limb_1_col12,
                    dst_limb_2_col13,
                    dst_limb_3_col14,
                    dst_limb_4_col15,
                    dst_limb_5_col16,
                    dst_limb_6_col17,
                    dst_limb_7_col18,
                    dst_limb_8_col19,
                    dst_limb_9_col20,
                    dst_limb_10_col21,
                    dst_limb_11_col22,
                    dst_limb_12_col23,
                    dst_limb_13_col24,
                    dst_limb_14_col25,
                    dst_limb_15_col26,
                    dst_limb_16_col27,
                    dst_limb_17_col28,
                    dst_limb_18_col29,
                    dst_limb_19_col30,
                    dst_limb_20_col31,
                    dst_limb_21_col32,
                    dst_limb_22_col33,
                    dst_limb_23_col34,
                    dst_limb_24_col35,
                    dst_limb_25_col36,
                    dst_limb_26_col37,
                    dst_limb_27_col38,
                ];
                let read_positive_num_bits_252_output_tmp_48d52_10 = (
                    PackedFelt252::from_limbs([
                        dst_limb_0_col11,
                        dst_limb_1_col12,
                        dst_limb_2_col13,
                        dst_limb_3_col14,
                        dst_limb_4_col15,
                        dst_limb_5_col16,
                        dst_limb_6_col17,
                        dst_limb_7_col18,
                        dst_limb_8_col19,
                        dst_limb_9_col20,
                        dst_limb_10_col21,
                        dst_limb_11_col22,
                        dst_limb_12_col23,
                        dst_limb_13_col24,
                        dst_limb_14_col25,
                        dst_limb_15_col26,
                        dst_limb_16_col27,
                        dst_limb_17_col28,
                        dst_limb_18_col29,
                        dst_limb_19_col30,
                        dst_limb_20_col31,
                        dst_limb_21_col32,
                        dst_limb_22_col33,
                        dst_limb_23_col34,
                        dst_limb_24_col35,
                        dst_limb_25_col36,
                        dst_limb_26_col37,
                        dst_limb_27_col38,
                    ]),
                    dst_id_col10,
                );

                // Read Positive Num Bits 252.

                let memory_address_to_id_value_tmp_48d52_11 = memory_address_to_id_state
                    .deduce_output(
                        ((mem0_base_col9)
                            + (decode_instruction_db26c85482ebf3d9_output_tmp_48d52_7.0[1])),
                    );
                let memory_id_to_big_value_tmp_48d52_12 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_48d52_11);
                let op0_id_col39 = memory_address_to_id_value_tmp_48d52_11;
                *row[39] = op0_id_col39;
                *sub_component_inputs.memory_address_to_id[1] = ((mem0_base_col9)
                    + (decode_instruction_db26c85482ebf3d9_output_tmp_48d52_7.0[1]));
                *lookup_data.memory_address_to_id_1 = [
                    ((mem0_base_col9)
                        + (decode_instruction_db26c85482ebf3d9_output_tmp_48d52_7.0[1])),
                    op0_id_col39,
                ];
                let op0_limb_0_col40 = memory_id_to_big_value_tmp_48d52_12.get_m31(0);
                *row[40] = op0_limb_0_col40;
                let op0_limb_1_col41 = memory_id_to_big_value_tmp_48d52_12.get_m31(1);
                *row[41] = op0_limb_1_col41;
                let op0_limb_2_col42 = memory_id_to_big_value_tmp_48d52_12.get_m31(2);
                *row[42] = op0_limb_2_col42;
                let op0_limb_3_col43 = memory_id_to_big_value_tmp_48d52_12.get_m31(3);
                *row[43] = op0_limb_3_col43;
                let op0_limb_4_col44 = memory_id_to_big_value_tmp_48d52_12.get_m31(4);
                *row[44] = op0_limb_4_col44;
                let op0_limb_5_col45 = memory_id_to_big_value_tmp_48d52_12.get_m31(5);
                *row[45] = op0_limb_5_col45;
                let op0_limb_6_col46 = memory_id_to_big_value_tmp_48d52_12.get_m31(6);
                *row[46] = op0_limb_6_col46;
                let op0_limb_7_col47 = memory_id_to_big_value_tmp_48d52_12.get_m31(7);
                *row[47] = op0_limb_7_col47;
                let op0_limb_8_col48 = memory_id_to_big_value_tmp_48d52_12.get_m31(8);
                *row[48] = op0_limb_8_col48;
                let op0_limb_9_col49 = memory_id_to_big_value_tmp_48d52_12.get_m31(9);
                *row[49] = op0_limb_9_col49;
                let op0_limb_10_col50 = memory_id_to_big_value_tmp_48d52_12.get_m31(10);
                *row[50] = op0_limb_10_col50;
                let op0_limb_11_col51 = memory_id_to_big_value_tmp_48d52_12.get_m31(11);
                *row[51] = op0_limb_11_col51;
                let op0_limb_12_col52 = memory_id_to_big_value_tmp_48d52_12.get_m31(12);
                *row[52] = op0_limb_12_col52;
                let op0_limb_13_col53 = memory_id_to_big_value_tmp_48d52_12.get_m31(13);
                *row[53] = op0_limb_13_col53;
                let op0_limb_14_col54 = memory_id_to_big_value_tmp_48d52_12.get_m31(14);
                *row[54] = op0_limb_14_col54;
                let op0_limb_15_col55 = memory_id_to_big_value_tmp_48d52_12.get_m31(15);
                *row[55] = op0_limb_15_col55;
                let op0_limb_16_col56 = memory_id_to_big_value_tmp_48d52_12.get_m31(16);
                *row[56] = op0_limb_16_col56;
                let op0_limb_17_col57 = memory_id_to_big_value_tmp_48d52_12.get_m31(17);
                *row[57] = op0_limb_17_col57;
                let op0_limb_18_col58 = memory_id_to_big_value_tmp_48d52_12.get_m31(18);
                *row[58] = op0_limb_18_col58;
                let op0_limb_19_col59 = memory_id_to_big_value_tmp_48d52_12.get_m31(19);
                *row[59] = op0_limb_19_col59;
                let op0_limb_20_col60 = memory_id_to_big_value_tmp_48d52_12.get_m31(20);
                *row[60] = op0_limb_20_col60;
                let op0_limb_21_col61 = memory_id_to_big_value_tmp_48d52_12.get_m31(21);
                *row[61] = op0_limb_21_col61;
                let op0_limb_22_col62 = memory_id_to_big_value_tmp_48d52_12.get_m31(22);
                *row[62] = op0_limb_22_col62;
                let op0_limb_23_col63 = memory_id_to_big_value_tmp_48d52_12.get_m31(23);
                *row[63] = op0_limb_23_col63;
                let op0_limb_24_col64 = memory_id_to_big_value_tmp_48d52_12.get_m31(24);
                *row[64] = op0_limb_24_col64;
                let op0_limb_25_col65 = memory_id_to_big_value_tmp_48d52_12.get_m31(25);
                *row[65] = op0_limb_25_col65;
                let op0_limb_26_col66 = memory_id_to_big_value_tmp_48d52_12.get_m31(26);
                *row[66] = op0_limb_26_col66;
                let op0_limb_27_col67 = memory_id_to_big_value_tmp_48d52_12.get_m31(27);
                *row[67] = op0_limb_27_col67;
                *sub_component_inputs.memory_id_to_big[1] = op0_id_col39;
                *lookup_data.memory_id_to_big_1 = [
                    op0_id_col39,
                    op0_limb_0_col40,
                    op0_limb_1_col41,
                    op0_limb_2_col42,
                    op0_limb_3_col43,
                    op0_limb_4_col44,
                    op0_limb_5_col45,
                    op0_limb_6_col46,
                    op0_limb_7_col47,
                    op0_limb_8_col48,
                    op0_limb_9_col49,
                    op0_limb_10_col50,
                    op0_limb_11_col51,
                    op0_limb_12_col52,
                    op0_limb_13_col53,
                    op0_limb_14_col54,
                    op0_limb_15_col55,
                    op0_limb_16_col56,
                    op0_limb_17_col57,
                    op0_limb_18_col58,
                    op0_limb_19_col59,
                    op0_limb_20_col60,
                    op0_limb_21_col61,
                    op0_limb_22_col62,
                    op0_limb_23_col63,
                    op0_limb_24_col64,
                    op0_limb_25_col65,
                    op0_limb_26_col66,
                    op0_limb_27_col67,
                ];
                let read_positive_num_bits_252_output_tmp_48d52_13 = (
                    PackedFelt252::from_limbs([
                        op0_limb_0_col40,
                        op0_limb_1_col41,
                        op0_limb_2_col42,
                        op0_limb_3_col43,
                        op0_limb_4_col44,
                        op0_limb_5_col45,
                        op0_limb_6_col46,
                        op0_limb_7_col47,
                        op0_limb_8_col48,
                        op0_limb_9_col49,
                        op0_limb_10_col50,
                        op0_limb_11_col51,
                        op0_limb_12_col52,
                        op0_limb_13_col53,
                        op0_limb_14_col54,
                        op0_limb_15_col55,
                        op0_limb_16_col56,
                        op0_limb_17_col57,
                        op0_limb_18_col58,
                        op0_limb_19_col59,
                        op0_limb_20_col60,
                        op0_limb_21_col61,
                        op0_limb_22_col62,
                        op0_limb_23_col63,
                        op0_limb_24_col64,
                        op0_limb_25_col65,
                        op0_limb_26_col66,
                        op0_limb_27_col67,
                    ]),
                    op0_id_col39,
                );

                // Read Positive Num Bits 252.

                let memory_address_to_id_value_tmp_48d52_14 =
                    memory_address_to_id_state.deduce_output(((input_pc_col0) + (M31_1)));
                let memory_id_to_big_value_tmp_48d52_15 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_48d52_14);
                let op1_id_col68 = memory_address_to_id_value_tmp_48d52_14;
                *row[68] = op1_id_col68;
                *sub_component_inputs.memory_address_to_id[2] = ((input_pc_col0) + (M31_1));
                *lookup_data.memory_address_to_id_2 = [((input_pc_col0) + (M31_1)), op1_id_col68];
                let op1_limb_0_col69 = memory_id_to_big_value_tmp_48d52_15.get_m31(0);
                *row[69] = op1_limb_0_col69;
                let op1_limb_1_col70 = memory_id_to_big_value_tmp_48d52_15.get_m31(1);
                *row[70] = op1_limb_1_col70;
                let op1_limb_2_col71 = memory_id_to_big_value_tmp_48d52_15.get_m31(2);
                *row[71] = op1_limb_2_col71;
                let op1_limb_3_col72 = memory_id_to_big_value_tmp_48d52_15.get_m31(3);
                *row[72] = op1_limb_3_col72;
                let op1_limb_4_col73 = memory_id_to_big_value_tmp_48d52_15.get_m31(4);
                *row[73] = op1_limb_4_col73;
                let op1_limb_5_col74 = memory_id_to_big_value_tmp_48d52_15.get_m31(5);
                *row[74] = op1_limb_5_col74;
                let op1_limb_6_col75 = memory_id_to_big_value_tmp_48d52_15.get_m31(6);
                *row[75] = op1_limb_6_col75;
                let op1_limb_7_col76 = memory_id_to_big_value_tmp_48d52_15.get_m31(7);
                *row[76] = op1_limb_7_col76;
                let op1_limb_8_col77 = memory_id_to_big_value_tmp_48d52_15.get_m31(8);
                *row[77] = op1_limb_8_col77;
                let op1_limb_9_col78 = memory_id_to_big_value_tmp_48d52_15.get_m31(9);
                *row[78] = op1_limb_9_col78;
                let op1_limb_10_col79 = memory_id_to_big_value_tmp_48d52_15.get_m31(10);
                *row[79] = op1_limb_10_col79;
                let op1_limb_11_col80 = memory_id_to_big_value_tmp_48d52_15.get_m31(11);
                *row[80] = op1_limb_11_col80;
                let op1_limb_12_col81 = memory_id_to_big_value_tmp_48d52_15.get_m31(12);
                *row[81] = op1_limb_12_col81;
                let op1_limb_13_col82 = memory_id_to_big_value_tmp_48d52_15.get_m31(13);
                *row[82] = op1_limb_13_col82;
                let op1_limb_14_col83 = memory_id_to_big_value_tmp_48d52_15.get_m31(14);
                *row[83] = op1_limb_14_col83;
                let op1_limb_15_col84 = memory_id_to_big_value_tmp_48d52_15.get_m31(15);
                *row[84] = op1_limb_15_col84;
                let op1_limb_16_col85 = memory_id_to_big_value_tmp_48d52_15.get_m31(16);
                *row[85] = op1_limb_16_col85;
                let op1_limb_17_col86 = memory_id_to_big_value_tmp_48d52_15.get_m31(17);
                *row[86] = op1_limb_17_col86;
                let op1_limb_18_col87 = memory_id_to_big_value_tmp_48d52_15.get_m31(18);
                *row[87] = op1_limb_18_col87;
                let op1_limb_19_col88 = memory_id_to_big_value_tmp_48d52_15.get_m31(19);
                *row[88] = op1_limb_19_col88;
                let op1_limb_20_col89 = memory_id_to_big_value_tmp_48d52_15.get_m31(20);
                *row[89] = op1_limb_20_col89;
                let op1_limb_21_col90 = memory_id_to_big_value_tmp_48d52_15.get_m31(21);
                *row[90] = op1_limb_21_col90;
                let op1_limb_22_col91 = memory_id_to_big_value_tmp_48d52_15.get_m31(22);
                *row[91] = op1_limb_22_col91;
                let op1_limb_23_col92 = memory_id_to_big_value_tmp_48d52_15.get_m31(23);
                *row[92] = op1_limb_23_col92;
                let op1_limb_24_col93 = memory_id_to_big_value_tmp_48d52_15.get_m31(24);
                *row[93] = op1_limb_24_col93;
                let op1_limb_25_col94 = memory_id_to_big_value_tmp_48d52_15.get_m31(25);
                *row[94] = op1_limb_25_col94;
                let op1_limb_26_col95 = memory_id_to_big_value_tmp_48d52_15.get_m31(26);
                *row[95] = op1_limb_26_col95;
                let op1_limb_27_col96 = memory_id_to_big_value_tmp_48d52_15.get_m31(27);
                *row[96] = op1_limb_27_col96;
                *sub_component_inputs.memory_id_to_big[2] = op1_id_col68;
                *lookup_data.memory_id_to_big_2 = [
                    op1_id_col68,
                    op1_limb_0_col69,
                    op1_limb_1_col70,
                    op1_limb_2_col71,
                    op1_limb_3_col72,
                    op1_limb_4_col73,
                    op1_limb_5_col74,
                    op1_limb_6_col75,
                    op1_limb_7_col76,
                    op1_limb_8_col77,
                    op1_limb_9_col78,
                    op1_limb_10_col79,
                    op1_limb_11_col80,
                    op1_limb_12_col81,
                    op1_limb_13_col82,
                    op1_limb_14_col83,
                    op1_limb_15_col84,
                    op1_limb_16_col85,
                    op1_limb_17_col86,
                    op1_limb_18_col87,
                    op1_limb_19_col88,
                    op1_limb_20_col89,
                    op1_limb_21_col90,
                    op1_limb_22_col91,
                    op1_limb_23_col92,
                    op1_limb_24_col93,
                    op1_limb_25_col94,
                    op1_limb_26_col95,
                    op1_limb_27_col96,
                ];
                let read_positive_num_bits_252_output_tmp_48d52_16 = (
                    PackedFelt252::from_limbs([
                        op1_limb_0_col69,
                        op1_limb_1_col70,
                        op1_limb_2_col71,
                        op1_limb_3_col72,
                        op1_limb_4_col73,
                        op1_limb_5_col74,
                        op1_limb_6_col75,
                        op1_limb_7_col76,
                        op1_limb_8_col77,
                        op1_limb_9_col78,
                        op1_limb_10_col79,
                        op1_limb_11_col80,
                        op1_limb_12_col81,
                        op1_limb_13_col82,
                        op1_limb_14_col83,
                        op1_limb_15_col84,
                        op1_limb_16_col85,
                        op1_limb_17_col86,
                        op1_limb_18_col87,
                        op1_limb_19_col88,
                        op1_limb_20_col89,
                        op1_limb_21_col90,
                        op1_limb_22_col91,
                        op1_limb_23_col92,
                        op1_limb_24_col93,
                        op1_limb_25_col94,
                        op1_limb_26_col95,
                        op1_limb_27_col96,
                    ]),
                    op1_id_col68,
                );

                // Verify Mul 252.

                // Double Karatsuba N 7 Limb Max Bound 511.

                // Single Karatsuba N 7.

                let z0_tmp_48d52_17 = [
                    ((op0_limb_0_col40) * (op1_limb_0_col69)),
                    (((op0_limb_0_col40) * (op1_limb_1_col70))
                        + ((op0_limb_1_col41) * (op1_limb_0_col69))),
                    ((((op0_limb_0_col40) * (op1_limb_2_col71))
                        + ((op0_limb_1_col41) * (op1_limb_1_col70)))
                        + ((op0_limb_2_col42) * (op1_limb_0_col69))),
                    (((((op0_limb_0_col40) * (op1_limb_3_col72))
                        + ((op0_limb_1_col41) * (op1_limb_2_col71)))
                        + ((op0_limb_2_col42) * (op1_limb_1_col70)))
                        + ((op0_limb_3_col43) * (op1_limb_0_col69))),
                    ((((((op0_limb_0_col40) * (op1_limb_4_col73))
                        + ((op0_limb_1_col41) * (op1_limb_3_col72)))
                        + ((op0_limb_2_col42) * (op1_limb_2_col71)))
                        + ((op0_limb_3_col43) * (op1_limb_1_col70)))
                        + ((op0_limb_4_col44) * (op1_limb_0_col69))),
                    (((((((op0_limb_0_col40) * (op1_limb_5_col74))
                        + ((op0_limb_1_col41) * (op1_limb_4_col73)))
                        + ((op0_limb_2_col42) * (op1_limb_3_col72)))
                        + ((op0_limb_3_col43) * (op1_limb_2_col71)))
                        + ((op0_limb_4_col44) * (op1_limb_1_col70)))
                        + ((op0_limb_5_col45) * (op1_limb_0_col69))),
                    ((((((((op0_limb_0_col40) * (op1_limb_6_col75))
                        + ((op0_limb_1_col41) * (op1_limb_5_col74)))
                        + ((op0_limb_2_col42) * (op1_limb_4_col73)))
                        + ((op0_limb_3_col43) * (op1_limb_3_col72)))
                        + ((op0_limb_4_col44) * (op1_limb_2_col71)))
                        + ((op0_limb_5_col45) * (op1_limb_1_col70)))
                        + ((op0_limb_6_col46) * (op1_limb_0_col69))),
                    (((((((op0_limb_1_col41) * (op1_limb_6_col75))
                        + ((op0_limb_2_col42) * (op1_limb_5_col74)))
                        + ((op0_limb_3_col43) * (op1_limb_4_col73)))
                        + ((op0_limb_4_col44) * (op1_limb_3_col72)))
                        + ((op0_limb_5_col45) * (op1_limb_2_col71)))
                        + ((op0_limb_6_col46) * (op1_limb_1_col70))),
                    ((((((op0_limb_2_col42) * (op1_limb_6_col75))
                        + ((op0_limb_3_col43) * (op1_limb_5_col74)))
                        + ((op0_limb_4_col44) * (op1_limb_4_col73)))
                        + ((op0_limb_5_col45) * (op1_limb_3_col72)))
                        + ((op0_limb_6_col46) * (op1_limb_2_col71))),
                    (((((op0_limb_3_col43) * (op1_limb_6_col75))
                        + ((op0_limb_4_col44) * (op1_limb_5_col74)))
                        + ((op0_limb_5_col45) * (op1_limb_4_col73)))
                        + ((op0_limb_6_col46) * (op1_limb_3_col72))),
                    ((((op0_limb_4_col44) * (op1_limb_6_col75))
                        + ((op0_limb_5_col45) * (op1_limb_5_col74)))
                        + ((op0_limb_6_col46) * (op1_limb_4_col73))),
                    (((op0_limb_5_col45) * (op1_limb_6_col75))
                        + ((op0_limb_6_col46) * (op1_limb_5_col74))),
                    ((op0_limb_6_col46) * (op1_limb_6_col75)),
                ];
                let z2_tmp_48d52_18 = [
                    ((op0_limb_7_col47) * (op1_limb_7_col76)),
                    (((op0_limb_7_col47) * (op1_limb_8_col77))
                        + ((op0_limb_8_col48) * (op1_limb_7_col76))),
                    ((((op0_limb_7_col47) * (op1_limb_9_col78))
                        + ((op0_limb_8_col48) * (op1_limb_8_col77)))
                        + ((op0_limb_9_col49) * (op1_limb_7_col76))),
                    (((((op0_limb_7_col47) * (op1_limb_10_col79))
                        + ((op0_limb_8_col48) * (op1_limb_9_col78)))
                        + ((op0_limb_9_col49) * (op1_limb_8_col77)))
                        + ((op0_limb_10_col50) * (op1_limb_7_col76))),
                    ((((((op0_limb_7_col47) * (op1_limb_11_col80))
                        + ((op0_limb_8_col48) * (op1_limb_10_col79)))
                        + ((op0_limb_9_col49) * (op1_limb_9_col78)))
                        + ((op0_limb_10_col50) * (op1_limb_8_col77)))
                        + ((op0_limb_11_col51) * (op1_limb_7_col76))),
                    (((((((op0_limb_7_col47) * (op1_limb_12_col81))
                        + ((op0_limb_8_col48) * (op1_limb_11_col80)))
                        + ((op0_limb_9_col49) * (op1_limb_10_col79)))
                        + ((op0_limb_10_col50) * (op1_limb_9_col78)))
                        + ((op0_limb_11_col51) * (op1_limb_8_col77)))
                        + ((op0_limb_12_col52) * (op1_limb_7_col76))),
                    ((((((((op0_limb_7_col47) * (op1_limb_13_col82))
                        + ((op0_limb_8_col48) * (op1_limb_12_col81)))
                        + ((op0_limb_9_col49) * (op1_limb_11_col80)))
                        + ((op0_limb_10_col50) * (op1_limb_10_col79)))
                        + ((op0_limb_11_col51) * (op1_limb_9_col78)))
                        + ((op0_limb_12_col52) * (op1_limb_8_col77)))
                        + ((op0_limb_13_col53) * (op1_limb_7_col76))),
                    (((((((op0_limb_8_col48) * (op1_limb_13_col82))
                        + ((op0_limb_9_col49) * (op1_limb_12_col81)))
                        + ((op0_limb_10_col50) * (op1_limb_11_col80)))
                        + ((op0_limb_11_col51) * (op1_limb_10_col79)))
                        + ((op0_limb_12_col52) * (op1_limb_9_col78)))
                        + ((op0_limb_13_col53) * (op1_limb_8_col77))),
                    ((((((op0_limb_9_col49) * (op1_limb_13_col82))
                        + ((op0_limb_10_col50) * (op1_limb_12_col81)))
                        + ((op0_limb_11_col51) * (op1_limb_11_col80)))
                        + ((op0_limb_12_col52) * (op1_limb_10_col79)))
                        + ((op0_limb_13_col53) * (op1_limb_9_col78))),
                    (((((op0_limb_10_col50) * (op1_limb_13_col82))
                        + ((op0_limb_11_col51) * (op1_limb_12_col81)))
                        + ((op0_limb_12_col52) * (op1_limb_11_col80)))
                        + ((op0_limb_13_col53) * (op1_limb_10_col79))),
                    ((((op0_limb_11_col51) * (op1_limb_13_col82))
                        + ((op0_limb_12_col52) * (op1_limb_12_col81)))
                        + ((op0_limb_13_col53) * (op1_limb_11_col80))),
                    (((op0_limb_12_col52) * (op1_limb_13_col82))
                        + ((op0_limb_13_col53) * (op1_limb_12_col81))),
                    ((op0_limb_13_col53) * (op1_limb_13_col82)),
                ];
                let x_sum_tmp_48d52_19 = [
                    ((op0_limb_0_col40) + (op0_limb_7_col47)),
                    ((op0_limb_1_col41) + (op0_limb_8_col48)),
                    ((op0_limb_2_col42) + (op0_limb_9_col49)),
                    ((op0_limb_3_col43) + (op0_limb_10_col50)),
                    ((op0_limb_4_col44) + (op0_limb_11_col51)),
                    ((op0_limb_5_col45) + (op0_limb_12_col52)),
                    ((op0_limb_6_col46) + (op0_limb_13_col53)),
                ];
                let y_sum_tmp_48d52_20 = [
                    ((op1_limb_0_col69) + (op1_limb_7_col76)),
                    ((op1_limb_1_col70) + (op1_limb_8_col77)),
                    ((op1_limb_2_col71) + (op1_limb_9_col78)),
                    ((op1_limb_3_col72) + (op1_limb_10_col79)),
                    ((op1_limb_4_col73) + (op1_limb_11_col80)),
                    ((op1_limb_5_col74) + (op1_limb_12_col81)),
                    ((op1_limb_6_col75) + (op1_limb_13_col82)),
                ];
                let single_karatsuba_n_7_output_tmp_48d52_21 = [
                    z0_tmp_48d52_17[0],
                    z0_tmp_48d52_17[1],
                    z0_tmp_48d52_17[2],
                    z0_tmp_48d52_17[3],
                    z0_tmp_48d52_17[4],
                    z0_tmp_48d52_17[5],
                    z0_tmp_48d52_17[6],
                    ((z0_tmp_48d52_17[7])
                        + ((((x_sum_tmp_48d52_19[0]) * (y_sum_tmp_48d52_20[0]))
                            - (z0_tmp_48d52_17[0]))
                            - (z2_tmp_48d52_18[0]))),
                    ((z0_tmp_48d52_17[8])
                        + (((((x_sum_tmp_48d52_19[0]) * (y_sum_tmp_48d52_20[1]))
                            + ((x_sum_tmp_48d52_19[1]) * (y_sum_tmp_48d52_20[0])))
                            - (z0_tmp_48d52_17[1]))
                            - (z2_tmp_48d52_18[1]))),
                    ((z0_tmp_48d52_17[9])
                        + ((((((x_sum_tmp_48d52_19[0]) * (y_sum_tmp_48d52_20[2]))
                            + ((x_sum_tmp_48d52_19[1]) * (y_sum_tmp_48d52_20[1])))
                            + ((x_sum_tmp_48d52_19[2]) * (y_sum_tmp_48d52_20[0])))
                            - (z0_tmp_48d52_17[2]))
                            - (z2_tmp_48d52_18[2]))),
                    ((z0_tmp_48d52_17[10])
                        + (((((((x_sum_tmp_48d52_19[0]) * (y_sum_tmp_48d52_20[3]))
                            + ((x_sum_tmp_48d52_19[1]) * (y_sum_tmp_48d52_20[2])))
                            + ((x_sum_tmp_48d52_19[2]) * (y_sum_tmp_48d52_20[1])))
                            + ((x_sum_tmp_48d52_19[3]) * (y_sum_tmp_48d52_20[0])))
                            - (z0_tmp_48d52_17[3]))
                            - (z2_tmp_48d52_18[3]))),
                    ((z0_tmp_48d52_17[11])
                        + ((((((((x_sum_tmp_48d52_19[0]) * (y_sum_tmp_48d52_20[4]))
                            + ((x_sum_tmp_48d52_19[1]) * (y_sum_tmp_48d52_20[3])))
                            + ((x_sum_tmp_48d52_19[2]) * (y_sum_tmp_48d52_20[2])))
                            + ((x_sum_tmp_48d52_19[3]) * (y_sum_tmp_48d52_20[1])))
                            + ((x_sum_tmp_48d52_19[4]) * (y_sum_tmp_48d52_20[0])))
                            - (z0_tmp_48d52_17[4]))
                            - (z2_tmp_48d52_18[4]))),
                    ((z0_tmp_48d52_17[12])
                        + (((((((((x_sum_tmp_48d52_19[0]) * (y_sum_tmp_48d52_20[5]))
                            + ((x_sum_tmp_48d52_19[1]) * (y_sum_tmp_48d52_20[4])))
                            + ((x_sum_tmp_48d52_19[2]) * (y_sum_tmp_48d52_20[3])))
                            + ((x_sum_tmp_48d52_19[3]) * (y_sum_tmp_48d52_20[2])))
                            + ((x_sum_tmp_48d52_19[4]) * (y_sum_tmp_48d52_20[1])))
                            + ((x_sum_tmp_48d52_19[5]) * (y_sum_tmp_48d52_20[0])))
                            - (z0_tmp_48d52_17[5]))
                            - (z2_tmp_48d52_18[5]))),
                    ((((((((((x_sum_tmp_48d52_19[0]) * (y_sum_tmp_48d52_20[6]))
                        + ((x_sum_tmp_48d52_19[1]) * (y_sum_tmp_48d52_20[5])))
                        + ((x_sum_tmp_48d52_19[2]) * (y_sum_tmp_48d52_20[4])))
                        + ((x_sum_tmp_48d52_19[3]) * (y_sum_tmp_48d52_20[3])))
                        + ((x_sum_tmp_48d52_19[4]) * (y_sum_tmp_48d52_20[2])))
                        + ((x_sum_tmp_48d52_19[5]) * (y_sum_tmp_48d52_20[1])))
                        + ((x_sum_tmp_48d52_19[6]) * (y_sum_tmp_48d52_20[0])))
                        - (z0_tmp_48d52_17[6]))
                        - (z2_tmp_48d52_18[6])),
                    ((z2_tmp_48d52_18[0])
                        + (((((((((x_sum_tmp_48d52_19[1]) * (y_sum_tmp_48d52_20[6]))
                            + ((x_sum_tmp_48d52_19[2]) * (y_sum_tmp_48d52_20[5])))
                            + ((x_sum_tmp_48d52_19[3]) * (y_sum_tmp_48d52_20[4])))
                            + ((x_sum_tmp_48d52_19[4]) * (y_sum_tmp_48d52_20[3])))
                            + ((x_sum_tmp_48d52_19[5]) * (y_sum_tmp_48d52_20[2])))
                            + ((x_sum_tmp_48d52_19[6]) * (y_sum_tmp_48d52_20[1])))
                            - (z0_tmp_48d52_17[7]))
                            - (z2_tmp_48d52_18[7]))),
                    ((z2_tmp_48d52_18[1])
                        + ((((((((x_sum_tmp_48d52_19[2]) * (y_sum_tmp_48d52_20[6]))
                            + ((x_sum_tmp_48d52_19[3]) * (y_sum_tmp_48d52_20[5])))
                            + ((x_sum_tmp_48d52_19[4]) * (y_sum_tmp_48d52_20[4])))
                            + ((x_sum_tmp_48d52_19[5]) * (y_sum_tmp_48d52_20[3])))
                            + ((x_sum_tmp_48d52_19[6]) * (y_sum_tmp_48d52_20[2])))
                            - (z0_tmp_48d52_17[8]))
                            - (z2_tmp_48d52_18[8]))),
                    ((z2_tmp_48d52_18[2])
                        + (((((((x_sum_tmp_48d52_19[3]) * (y_sum_tmp_48d52_20[6]))
                            + ((x_sum_tmp_48d52_19[4]) * (y_sum_tmp_48d52_20[5])))
                            + ((x_sum_tmp_48d52_19[5]) * (y_sum_tmp_48d52_20[4])))
                            + ((x_sum_tmp_48d52_19[6]) * (y_sum_tmp_48d52_20[3])))
                            - (z0_tmp_48d52_17[9]))
                            - (z2_tmp_48d52_18[9]))),
                    ((z2_tmp_48d52_18[3])
                        + ((((((x_sum_tmp_48d52_19[4]) * (y_sum_tmp_48d52_20[6]))
                            + ((x_sum_tmp_48d52_19[5]) * (y_sum_tmp_48d52_20[5])))
                            + ((x_sum_tmp_48d52_19[6]) * (y_sum_tmp_48d52_20[4])))
                            - (z0_tmp_48d52_17[10]))
                            - (z2_tmp_48d52_18[10]))),
                    ((z2_tmp_48d52_18[4])
                        + (((((x_sum_tmp_48d52_19[5]) * (y_sum_tmp_48d52_20[6]))
                            + ((x_sum_tmp_48d52_19[6]) * (y_sum_tmp_48d52_20[5])))
                            - (z0_tmp_48d52_17[11]))
                            - (z2_tmp_48d52_18[11]))),
                    ((z2_tmp_48d52_18[5])
                        + ((((x_sum_tmp_48d52_19[6]) * (y_sum_tmp_48d52_20[6]))
                            - (z0_tmp_48d52_17[12]))
                            - (z2_tmp_48d52_18[12]))),
                    z2_tmp_48d52_18[6],
                    z2_tmp_48d52_18[7],
                    z2_tmp_48d52_18[8],
                    z2_tmp_48d52_18[9],
                    z2_tmp_48d52_18[10],
                    z2_tmp_48d52_18[11],
                    z2_tmp_48d52_18[12],
                ];

                // Single Karatsuba N 7.

                let z0_tmp_48d52_22 = [
                    ((op0_limb_14_col54) * (op1_limb_14_col83)),
                    (((op0_limb_14_col54) * (op1_limb_15_col84))
                        + ((op0_limb_15_col55) * (op1_limb_14_col83))),
                    ((((op0_limb_14_col54) * (op1_limb_16_col85))
                        + ((op0_limb_15_col55) * (op1_limb_15_col84)))
                        + ((op0_limb_16_col56) * (op1_limb_14_col83))),
                    (((((op0_limb_14_col54) * (op1_limb_17_col86))
                        + ((op0_limb_15_col55) * (op1_limb_16_col85)))
                        + ((op0_limb_16_col56) * (op1_limb_15_col84)))
                        + ((op0_limb_17_col57) * (op1_limb_14_col83))),
                    ((((((op0_limb_14_col54) * (op1_limb_18_col87))
                        + ((op0_limb_15_col55) * (op1_limb_17_col86)))
                        + ((op0_limb_16_col56) * (op1_limb_16_col85)))
                        + ((op0_limb_17_col57) * (op1_limb_15_col84)))
                        + ((op0_limb_18_col58) * (op1_limb_14_col83))),
                    (((((((op0_limb_14_col54) * (op1_limb_19_col88))
                        + ((op0_limb_15_col55) * (op1_limb_18_col87)))
                        + ((op0_limb_16_col56) * (op1_limb_17_col86)))
                        + ((op0_limb_17_col57) * (op1_limb_16_col85)))
                        + ((op0_limb_18_col58) * (op1_limb_15_col84)))
                        + ((op0_limb_19_col59) * (op1_limb_14_col83))),
                    ((((((((op0_limb_14_col54) * (op1_limb_20_col89))
                        + ((op0_limb_15_col55) * (op1_limb_19_col88)))
                        + ((op0_limb_16_col56) * (op1_limb_18_col87)))
                        + ((op0_limb_17_col57) * (op1_limb_17_col86)))
                        + ((op0_limb_18_col58) * (op1_limb_16_col85)))
                        + ((op0_limb_19_col59) * (op1_limb_15_col84)))
                        + ((op0_limb_20_col60) * (op1_limb_14_col83))),
                    (((((((op0_limb_15_col55) * (op1_limb_20_col89))
                        + ((op0_limb_16_col56) * (op1_limb_19_col88)))
                        + ((op0_limb_17_col57) * (op1_limb_18_col87)))
                        + ((op0_limb_18_col58) * (op1_limb_17_col86)))
                        + ((op0_limb_19_col59) * (op1_limb_16_col85)))
                        + ((op0_limb_20_col60) * (op1_limb_15_col84))),
                    ((((((op0_limb_16_col56) * (op1_limb_20_col89))
                        + ((op0_limb_17_col57) * (op1_limb_19_col88)))
                        + ((op0_limb_18_col58) * (op1_limb_18_col87)))
                        + ((op0_limb_19_col59) * (op1_limb_17_col86)))
                        + ((op0_limb_20_col60) * (op1_limb_16_col85))),
                    (((((op0_limb_17_col57) * (op1_limb_20_col89))
                        + ((op0_limb_18_col58) * (op1_limb_19_col88)))
                        + ((op0_limb_19_col59) * (op1_limb_18_col87)))
                        + ((op0_limb_20_col60) * (op1_limb_17_col86))),
                    ((((op0_limb_18_col58) * (op1_limb_20_col89))
                        + ((op0_limb_19_col59) * (op1_limb_19_col88)))
                        + ((op0_limb_20_col60) * (op1_limb_18_col87))),
                    (((op0_limb_19_col59) * (op1_limb_20_col89))
                        + ((op0_limb_20_col60) * (op1_limb_19_col88))),
                    ((op0_limb_20_col60) * (op1_limb_20_col89)),
                ];
                let z2_tmp_48d52_23 = [
                    ((op0_limb_21_col61) * (op1_limb_21_col90)),
                    (((op0_limb_21_col61) * (op1_limb_22_col91))
                        + ((op0_limb_22_col62) * (op1_limb_21_col90))),
                    ((((op0_limb_21_col61) * (op1_limb_23_col92))
                        + ((op0_limb_22_col62) * (op1_limb_22_col91)))
                        + ((op0_limb_23_col63) * (op1_limb_21_col90))),
                    (((((op0_limb_21_col61) * (op1_limb_24_col93))
                        + ((op0_limb_22_col62) * (op1_limb_23_col92)))
                        + ((op0_limb_23_col63) * (op1_limb_22_col91)))
                        + ((op0_limb_24_col64) * (op1_limb_21_col90))),
                    ((((((op0_limb_21_col61) * (op1_limb_25_col94))
                        + ((op0_limb_22_col62) * (op1_limb_24_col93)))
                        + ((op0_limb_23_col63) * (op1_limb_23_col92)))
                        + ((op0_limb_24_col64) * (op1_limb_22_col91)))
                        + ((op0_limb_25_col65) * (op1_limb_21_col90))),
                    (((((((op0_limb_21_col61) * (op1_limb_26_col95))
                        + ((op0_limb_22_col62) * (op1_limb_25_col94)))
                        + ((op0_limb_23_col63) * (op1_limb_24_col93)))
                        + ((op0_limb_24_col64) * (op1_limb_23_col92)))
                        + ((op0_limb_25_col65) * (op1_limb_22_col91)))
                        + ((op0_limb_26_col66) * (op1_limb_21_col90))),
                    ((((((((op0_limb_21_col61) * (op1_limb_27_col96))
                        + ((op0_limb_22_col62) * (op1_limb_26_col95)))
                        + ((op0_limb_23_col63) * (op1_limb_25_col94)))
                        + ((op0_limb_24_col64) * (op1_limb_24_col93)))
                        + ((op0_limb_25_col65) * (op1_limb_23_col92)))
                        + ((op0_limb_26_col66) * (op1_limb_22_col91)))
                        + ((op0_limb_27_col67) * (op1_limb_21_col90))),
                    (((((((op0_limb_22_col62) * (op1_limb_27_col96))
                        + ((op0_limb_23_col63) * (op1_limb_26_col95)))
                        + ((op0_limb_24_col64) * (op1_limb_25_col94)))
                        + ((op0_limb_25_col65) * (op1_limb_24_col93)))
                        + ((op0_limb_26_col66) * (op1_limb_23_col92)))
                        + ((op0_limb_27_col67) * (op1_limb_22_col91))),
                    ((((((op0_limb_23_col63) * (op1_limb_27_col96))
                        + ((op0_limb_24_col64) * (op1_limb_26_col95)))
                        + ((op0_limb_25_col65) * (op1_limb_25_col94)))
                        + ((op0_limb_26_col66) * (op1_limb_24_col93)))
                        + ((op0_limb_27_col67) * (op1_limb_23_col92))),
                    (((((op0_limb_24_col64) * (op1_limb_27_col96))
                        + ((op0_limb_25_col65) * (op1_limb_26_col95)))
                        + ((op0_limb_26_col66) * (op1_limb_25_col94)))
                        + ((op0_limb_27_col67) * (op1_limb_24_col93))),
                    ((((op0_limb_25_col65) * (op1_limb_27_col96))
                        + ((op0_limb_26_col66) * (op1_limb_26_col95)))
                        + ((op0_limb_27_col67) * (op1_limb_25_col94))),
                    (((op0_limb_26_col66) * (op1_limb_27_col96))
                        + ((op0_limb_27_col67) * (op1_limb_26_col95))),
                    ((op0_limb_27_col67) * (op1_limb_27_col96)),
                ];
                let x_sum_tmp_48d52_24 = [
                    ((op0_limb_14_col54) + (op0_limb_21_col61)),
                    ((op0_limb_15_col55) + (op0_limb_22_col62)),
                    ((op0_limb_16_col56) + (op0_limb_23_col63)),
                    ((op0_limb_17_col57) + (op0_limb_24_col64)),
                    ((op0_limb_18_col58) + (op0_limb_25_col65)),
                    ((op0_limb_19_col59) + (op0_limb_26_col66)),
                    ((op0_limb_20_col60) + (op0_limb_27_col67)),
                ];
                let y_sum_tmp_48d52_25 = [
                    ((op1_limb_14_col83) + (op1_limb_21_col90)),
                    ((op1_limb_15_col84) + (op1_limb_22_col91)),
                    ((op1_limb_16_col85) + (op1_limb_23_col92)),
                    ((op1_limb_17_col86) + (op1_limb_24_col93)),
                    ((op1_limb_18_col87) + (op1_limb_25_col94)),
                    ((op1_limb_19_col88) + (op1_limb_26_col95)),
                    ((op1_limb_20_col89) + (op1_limb_27_col96)),
                ];
                let single_karatsuba_n_7_output_tmp_48d52_26 = [
                    z0_tmp_48d52_22[0],
                    z0_tmp_48d52_22[1],
                    z0_tmp_48d52_22[2],
                    z0_tmp_48d52_22[3],
                    z0_tmp_48d52_22[4],
                    z0_tmp_48d52_22[5],
                    z0_tmp_48d52_22[6],
                    ((z0_tmp_48d52_22[7])
                        + ((((x_sum_tmp_48d52_24[0]) * (y_sum_tmp_48d52_25[0]))
                            - (z0_tmp_48d52_22[0]))
                            - (z2_tmp_48d52_23[0]))),
                    ((z0_tmp_48d52_22[8])
                        + (((((x_sum_tmp_48d52_24[0]) * (y_sum_tmp_48d52_25[1]))
                            + ((x_sum_tmp_48d52_24[1]) * (y_sum_tmp_48d52_25[0])))
                            - (z0_tmp_48d52_22[1]))
                            - (z2_tmp_48d52_23[1]))),
                    ((z0_tmp_48d52_22[9])
                        + ((((((x_sum_tmp_48d52_24[0]) * (y_sum_tmp_48d52_25[2]))
                            + ((x_sum_tmp_48d52_24[1]) * (y_sum_tmp_48d52_25[1])))
                            + ((x_sum_tmp_48d52_24[2]) * (y_sum_tmp_48d52_25[0])))
                            - (z0_tmp_48d52_22[2]))
                            - (z2_tmp_48d52_23[2]))),
                    ((z0_tmp_48d52_22[10])
                        + (((((((x_sum_tmp_48d52_24[0]) * (y_sum_tmp_48d52_25[3]))
                            + ((x_sum_tmp_48d52_24[1]) * (y_sum_tmp_48d52_25[2])))
                            + ((x_sum_tmp_48d52_24[2]) * (y_sum_tmp_48d52_25[1])))
                            + ((x_sum_tmp_48d52_24[3]) * (y_sum_tmp_48d52_25[0])))
                            - (z0_tmp_48d52_22[3]))
                            - (z2_tmp_48d52_23[3]))),
                    ((z0_tmp_48d52_22[11])
                        + ((((((((x_sum_tmp_48d52_24[0]) * (y_sum_tmp_48d52_25[4]))
                            + ((x_sum_tmp_48d52_24[1]) * (y_sum_tmp_48d52_25[3])))
                            + ((x_sum_tmp_48d52_24[2]) * (y_sum_tmp_48d52_25[2])))
                            + ((x_sum_tmp_48d52_24[3]) * (y_sum_tmp_48d52_25[1])))
                            + ((x_sum_tmp_48d52_24[4]) * (y_sum_tmp_48d52_25[0])))
                            - (z0_tmp_48d52_22[4]))
                            - (z2_tmp_48d52_23[4]))),
                    ((z0_tmp_48d52_22[12])
                        + (((((((((x_sum_tmp_48d52_24[0]) * (y_sum_tmp_48d52_25[5]))
                            + ((x_sum_tmp_48d52_24[1]) * (y_sum_tmp_48d52_25[4])))
                            + ((x_sum_tmp_48d52_24[2]) * (y_sum_tmp_48d52_25[3])))
                            + ((x_sum_tmp_48d52_24[3]) * (y_sum_tmp_48d52_25[2])))
                            + ((x_sum_tmp_48d52_24[4]) * (y_sum_tmp_48d52_25[1])))
                            + ((x_sum_tmp_48d52_24[5]) * (y_sum_tmp_48d52_25[0])))
                            - (z0_tmp_48d52_22[5]))
                            - (z2_tmp_48d52_23[5]))),
                    ((((((((((x_sum_tmp_48d52_24[0]) * (y_sum_tmp_48d52_25[6]))
                        + ((x_sum_tmp_48d52_24[1]) * (y_sum_tmp_48d52_25[5])))
                        + ((x_sum_tmp_48d52_24[2]) * (y_sum_tmp_48d52_25[4])))
                        + ((x_sum_tmp_48d52_24[3]) * (y_sum_tmp_48d52_25[3])))
                        + ((x_sum_tmp_48d52_24[4]) * (y_sum_tmp_48d52_25[2])))
                        + ((x_sum_tmp_48d52_24[5]) * (y_sum_tmp_48d52_25[1])))
                        + ((x_sum_tmp_48d52_24[6]) * (y_sum_tmp_48d52_25[0])))
                        - (z0_tmp_48d52_22[6]))
                        - (z2_tmp_48d52_23[6])),
                    ((z2_tmp_48d52_23[0])
                        + (((((((((x_sum_tmp_48d52_24[1]) * (y_sum_tmp_48d52_25[6]))
                            + ((x_sum_tmp_48d52_24[2]) * (y_sum_tmp_48d52_25[5])))
                            + ((x_sum_tmp_48d52_24[3]) * (y_sum_tmp_48d52_25[4])))
                            + ((x_sum_tmp_48d52_24[4]) * (y_sum_tmp_48d52_25[3])))
                            + ((x_sum_tmp_48d52_24[5]) * (y_sum_tmp_48d52_25[2])))
                            + ((x_sum_tmp_48d52_24[6]) * (y_sum_tmp_48d52_25[1])))
                            - (z0_tmp_48d52_22[7]))
                            - (z2_tmp_48d52_23[7]))),
                    ((z2_tmp_48d52_23[1])
                        + ((((((((x_sum_tmp_48d52_24[2]) * (y_sum_tmp_48d52_25[6]))
                            + ((x_sum_tmp_48d52_24[3]) * (y_sum_tmp_48d52_25[5])))
                            + ((x_sum_tmp_48d52_24[4]) * (y_sum_tmp_48d52_25[4])))
                            + ((x_sum_tmp_48d52_24[5]) * (y_sum_tmp_48d52_25[3])))
                            + ((x_sum_tmp_48d52_24[6]) * (y_sum_tmp_48d52_25[2])))
                            - (z0_tmp_48d52_22[8]))
                            - (z2_tmp_48d52_23[8]))),
                    ((z2_tmp_48d52_23[2])
                        + (((((((x_sum_tmp_48d52_24[3]) * (y_sum_tmp_48d52_25[6]))
                            + ((x_sum_tmp_48d52_24[4]) * (y_sum_tmp_48d52_25[5])))
                            + ((x_sum_tmp_48d52_24[5]) * (y_sum_tmp_48d52_25[4])))
                            + ((x_sum_tmp_48d52_24[6]) * (y_sum_tmp_48d52_25[3])))
                            - (z0_tmp_48d52_22[9]))
                            - (z2_tmp_48d52_23[9]))),
                    ((z2_tmp_48d52_23[3])
                        + ((((((x_sum_tmp_48d52_24[4]) * (y_sum_tmp_48d52_25[6]))
                            + ((x_sum_tmp_48d52_24[5]) * (y_sum_tmp_48d52_25[5])))
                            + ((x_sum_tmp_48d52_24[6]) * (y_sum_tmp_48d52_25[4])))
                            - (z0_tmp_48d52_22[10]))
                            - (z2_tmp_48d52_23[10]))),
                    ((z2_tmp_48d52_23[4])
                        + (((((x_sum_tmp_48d52_24[5]) * (y_sum_tmp_48d52_25[6]))
                            + ((x_sum_tmp_48d52_24[6]) * (y_sum_tmp_48d52_25[5])))
                            - (z0_tmp_48d52_22[11]))
                            - (z2_tmp_48d52_23[11]))),
                    ((z2_tmp_48d52_23[5])
                        + ((((x_sum_tmp_48d52_24[6]) * (y_sum_tmp_48d52_25[6]))
                            - (z0_tmp_48d52_22[12]))
                            - (z2_tmp_48d52_23[12]))),
                    z2_tmp_48d52_23[6],
                    z2_tmp_48d52_23[7],
                    z2_tmp_48d52_23[8],
                    z2_tmp_48d52_23[9],
                    z2_tmp_48d52_23[10],
                    z2_tmp_48d52_23[11],
                    z2_tmp_48d52_23[12],
                ];

                let x_sum_tmp_48d52_27 = [
                    ((op0_limb_0_col40) + (op0_limb_14_col54)),
                    ((op0_limb_1_col41) + (op0_limb_15_col55)),
                    ((op0_limb_2_col42) + (op0_limb_16_col56)),
                    ((op0_limb_3_col43) + (op0_limb_17_col57)),
                    ((op0_limb_4_col44) + (op0_limb_18_col58)),
                    ((op0_limb_5_col45) + (op0_limb_19_col59)),
                    ((op0_limb_6_col46) + (op0_limb_20_col60)),
                    ((op0_limb_7_col47) + (op0_limb_21_col61)),
                    ((op0_limb_8_col48) + (op0_limb_22_col62)),
                    ((op0_limb_9_col49) + (op0_limb_23_col63)),
                    ((op0_limb_10_col50) + (op0_limb_24_col64)),
                    ((op0_limb_11_col51) + (op0_limb_25_col65)),
                    ((op0_limb_12_col52) + (op0_limb_26_col66)),
                    ((op0_limb_13_col53) + (op0_limb_27_col67)),
                ];
                let y_sum_tmp_48d52_28 = [
                    ((op1_limb_0_col69) + (op1_limb_14_col83)),
                    ((op1_limb_1_col70) + (op1_limb_15_col84)),
                    ((op1_limb_2_col71) + (op1_limb_16_col85)),
                    ((op1_limb_3_col72) + (op1_limb_17_col86)),
                    ((op1_limb_4_col73) + (op1_limb_18_col87)),
                    ((op1_limb_5_col74) + (op1_limb_19_col88)),
                    ((op1_limb_6_col75) + (op1_limb_20_col89)),
                    ((op1_limb_7_col76) + (op1_limb_21_col90)),
                    ((op1_limb_8_col77) + (op1_limb_22_col91)),
                    ((op1_limb_9_col78) + (op1_limb_23_col92)),
                    ((op1_limb_10_col79) + (op1_limb_24_col93)),
                    ((op1_limb_11_col80) + (op1_limb_25_col94)),
                    ((op1_limb_12_col81) + (op1_limb_26_col95)),
                    ((op1_limb_13_col82) + (op1_limb_27_col96)),
                ];

                // Single Karatsuba N 7.

                let z0_tmp_48d52_29 = [
                    ((x_sum_tmp_48d52_27[0]) * (y_sum_tmp_48d52_28[0])),
                    (((x_sum_tmp_48d52_27[0]) * (y_sum_tmp_48d52_28[1]))
                        + ((x_sum_tmp_48d52_27[1]) * (y_sum_tmp_48d52_28[0]))),
                    ((((x_sum_tmp_48d52_27[0]) * (y_sum_tmp_48d52_28[2]))
                        + ((x_sum_tmp_48d52_27[1]) * (y_sum_tmp_48d52_28[1])))
                        + ((x_sum_tmp_48d52_27[2]) * (y_sum_tmp_48d52_28[0]))),
                    (((((x_sum_tmp_48d52_27[0]) * (y_sum_tmp_48d52_28[3]))
                        + ((x_sum_tmp_48d52_27[1]) * (y_sum_tmp_48d52_28[2])))
                        + ((x_sum_tmp_48d52_27[2]) * (y_sum_tmp_48d52_28[1])))
                        + ((x_sum_tmp_48d52_27[3]) * (y_sum_tmp_48d52_28[0]))),
                    ((((((x_sum_tmp_48d52_27[0]) * (y_sum_tmp_48d52_28[4]))
                        + ((x_sum_tmp_48d52_27[1]) * (y_sum_tmp_48d52_28[3])))
                        + ((x_sum_tmp_48d52_27[2]) * (y_sum_tmp_48d52_28[2])))
                        + ((x_sum_tmp_48d52_27[3]) * (y_sum_tmp_48d52_28[1])))
                        + ((x_sum_tmp_48d52_27[4]) * (y_sum_tmp_48d52_28[0]))),
                    (((((((x_sum_tmp_48d52_27[0]) * (y_sum_tmp_48d52_28[5]))
                        + ((x_sum_tmp_48d52_27[1]) * (y_sum_tmp_48d52_28[4])))
                        + ((x_sum_tmp_48d52_27[2]) * (y_sum_tmp_48d52_28[3])))
                        + ((x_sum_tmp_48d52_27[3]) * (y_sum_tmp_48d52_28[2])))
                        + ((x_sum_tmp_48d52_27[4]) * (y_sum_tmp_48d52_28[1])))
                        + ((x_sum_tmp_48d52_27[5]) * (y_sum_tmp_48d52_28[0]))),
                    ((((((((x_sum_tmp_48d52_27[0]) * (y_sum_tmp_48d52_28[6]))
                        + ((x_sum_tmp_48d52_27[1]) * (y_sum_tmp_48d52_28[5])))
                        + ((x_sum_tmp_48d52_27[2]) * (y_sum_tmp_48d52_28[4])))
                        + ((x_sum_tmp_48d52_27[3]) * (y_sum_tmp_48d52_28[3])))
                        + ((x_sum_tmp_48d52_27[4]) * (y_sum_tmp_48d52_28[2])))
                        + ((x_sum_tmp_48d52_27[5]) * (y_sum_tmp_48d52_28[1])))
                        + ((x_sum_tmp_48d52_27[6]) * (y_sum_tmp_48d52_28[0]))),
                    (((((((x_sum_tmp_48d52_27[1]) * (y_sum_tmp_48d52_28[6]))
                        + ((x_sum_tmp_48d52_27[2]) * (y_sum_tmp_48d52_28[5])))
                        + ((x_sum_tmp_48d52_27[3]) * (y_sum_tmp_48d52_28[4])))
                        + ((x_sum_tmp_48d52_27[4]) * (y_sum_tmp_48d52_28[3])))
                        + ((x_sum_tmp_48d52_27[5]) * (y_sum_tmp_48d52_28[2])))
                        + ((x_sum_tmp_48d52_27[6]) * (y_sum_tmp_48d52_28[1]))),
                    ((((((x_sum_tmp_48d52_27[2]) * (y_sum_tmp_48d52_28[6]))
                        + ((x_sum_tmp_48d52_27[3]) * (y_sum_tmp_48d52_28[5])))
                        + ((x_sum_tmp_48d52_27[4]) * (y_sum_tmp_48d52_28[4])))
                        + ((x_sum_tmp_48d52_27[5]) * (y_sum_tmp_48d52_28[3])))
                        + ((x_sum_tmp_48d52_27[6]) * (y_sum_tmp_48d52_28[2]))),
                    (((((x_sum_tmp_48d52_27[3]) * (y_sum_tmp_48d52_28[6]))
                        + ((x_sum_tmp_48d52_27[4]) * (y_sum_tmp_48d52_28[5])))
                        + ((x_sum_tmp_48d52_27[5]) * (y_sum_tmp_48d52_28[4])))
                        + ((x_sum_tmp_48d52_27[6]) * (y_sum_tmp_48d52_28[3]))),
                    ((((x_sum_tmp_48d52_27[4]) * (y_sum_tmp_48d52_28[6]))
                        + ((x_sum_tmp_48d52_27[5]) * (y_sum_tmp_48d52_28[5])))
                        + ((x_sum_tmp_48d52_27[6]) * (y_sum_tmp_48d52_28[4]))),
                    (((x_sum_tmp_48d52_27[5]) * (y_sum_tmp_48d52_28[6]))
                        + ((x_sum_tmp_48d52_27[6]) * (y_sum_tmp_48d52_28[5]))),
                    ((x_sum_tmp_48d52_27[6]) * (y_sum_tmp_48d52_28[6])),
                ];
                let z2_tmp_48d52_30 = [
                    ((x_sum_tmp_48d52_27[7]) * (y_sum_tmp_48d52_28[7])),
                    (((x_sum_tmp_48d52_27[7]) * (y_sum_tmp_48d52_28[8]))
                        + ((x_sum_tmp_48d52_27[8]) * (y_sum_tmp_48d52_28[7]))),
                    ((((x_sum_tmp_48d52_27[7]) * (y_sum_tmp_48d52_28[9]))
                        + ((x_sum_tmp_48d52_27[8]) * (y_sum_tmp_48d52_28[8])))
                        + ((x_sum_tmp_48d52_27[9]) * (y_sum_tmp_48d52_28[7]))),
                    (((((x_sum_tmp_48d52_27[7]) * (y_sum_tmp_48d52_28[10]))
                        + ((x_sum_tmp_48d52_27[8]) * (y_sum_tmp_48d52_28[9])))
                        + ((x_sum_tmp_48d52_27[9]) * (y_sum_tmp_48d52_28[8])))
                        + ((x_sum_tmp_48d52_27[10]) * (y_sum_tmp_48d52_28[7]))),
                    ((((((x_sum_tmp_48d52_27[7]) * (y_sum_tmp_48d52_28[11]))
                        + ((x_sum_tmp_48d52_27[8]) * (y_sum_tmp_48d52_28[10])))
                        + ((x_sum_tmp_48d52_27[9]) * (y_sum_tmp_48d52_28[9])))
                        + ((x_sum_tmp_48d52_27[10]) * (y_sum_tmp_48d52_28[8])))
                        + ((x_sum_tmp_48d52_27[11]) * (y_sum_tmp_48d52_28[7]))),
                    (((((((x_sum_tmp_48d52_27[7]) * (y_sum_tmp_48d52_28[12]))
                        + ((x_sum_tmp_48d52_27[8]) * (y_sum_tmp_48d52_28[11])))
                        + ((x_sum_tmp_48d52_27[9]) * (y_sum_tmp_48d52_28[10])))
                        + ((x_sum_tmp_48d52_27[10]) * (y_sum_tmp_48d52_28[9])))
                        + ((x_sum_tmp_48d52_27[11]) * (y_sum_tmp_48d52_28[8])))
                        + ((x_sum_tmp_48d52_27[12]) * (y_sum_tmp_48d52_28[7]))),
                    ((((((((x_sum_tmp_48d52_27[7]) * (y_sum_tmp_48d52_28[13]))
                        + ((x_sum_tmp_48d52_27[8]) * (y_sum_tmp_48d52_28[12])))
                        + ((x_sum_tmp_48d52_27[9]) * (y_sum_tmp_48d52_28[11])))
                        + ((x_sum_tmp_48d52_27[10]) * (y_sum_tmp_48d52_28[10])))
                        + ((x_sum_tmp_48d52_27[11]) * (y_sum_tmp_48d52_28[9])))
                        + ((x_sum_tmp_48d52_27[12]) * (y_sum_tmp_48d52_28[8])))
                        + ((x_sum_tmp_48d52_27[13]) * (y_sum_tmp_48d52_28[7]))),
                    (((((((x_sum_tmp_48d52_27[8]) * (y_sum_tmp_48d52_28[13]))
                        + ((x_sum_tmp_48d52_27[9]) * (y_sum_tmp_48d52_28[12])))
                        + ((x_sum_tmp_48d52_27[10]) * (y_sum_tmp_48d52_28[11])))
                        + ((x_sum_tmp_48d52_27[11]) * (y_sum_tmp_48d52_28[10])))
                        + ((x_sum_tmp_48d52_27[12]) * (y_sum_tmp_48d52_28[9])))
                        + ((x_sum_tmp_48d52_27[13]) * (y_sum_tmp_48d52_28[8]))),
                    ((((((x_sum_tmp_48d52_27[9]) * (y_sum_tmp_48d52_28[13]))
                        + ((x_sum_tmp_48d52_27[10]) * (y_sum_tmp_48d52_28[12])))
                        + ((x_sum_tmp_48d52_27[11]) * (y_sum_tmp_48d52_28[11])))
                        + ((x_sum_tmp_48d52_27[12]) * (y_sum_tmp_48d52_28[10])))
                        + ((x_sum_tmp_48d52_27[13]) * (y_sum_tmp_48d52_28[9]))),
                    (((((x_sum_tmp_48d52_27[10]) * (y_sum_tmp_48d52_28[13]))
                        + ((x_sum_tmp_48d52_27[11]) * (y_sum_tmp_48d52_28[12])))
                        + ((x_sum_tmp_48d52_27[12]) * (y_sum_tmp_48d52_28[11])))
                        + ((x_sum_tmp_48d52_27[13]) * (y_sum_tmp_48d52_28[10]))),
                    ((((x_sum_tmp_48d52_27[11]) * (y_sum_tmp_48d52_28[13]))
                        + ((x_sum_tmp_48d52_27[12]) * (y_sum_tmp_48d52_28[12])))
                        + ((x_sum_tmp_48d52_27[13]) * (y_sum_tmp_48d52_28[11]))),
                    (((x_sum_tmp_48d52_27[12]) * (y_sum_tmp_48d52_28[13]))
                        + ((x_sum_tmp_48d52_27[13]) * (y_sum_tmp_48d52_28[12]))),
                    ((x_sum_tmp_48d52_27[13]) * (y_sum_tmp_48d52_28[13])),
                ];
                let x_sum_tmp_48d52_31 = [
                    ((x_sum_tmp_48d52_27[0]) + (x_sum_tmp_48d52_27[7])),
                    ((x_sum_tmp_48d52_27[1]) + (x_sum_tmp_48d52_27[8])),
                    ((x_sum_tmp_48d52_27[2]) + (x_sum_tmp_48d52_27[9])),
                    ((x_sum_tmp_48d52_27[3]) + (x_sum_tmp_48d52_27[10])),
                    ((x_sum_tmp_48d52_27[4]) + (x_sum_tmp_48d52_27[11])),
                    ((x_sum_tmp_48d52_27[5]) + (x_sum_tmp_48d52_27[12])),
                    ((x_sum_tmp_48d52_27[6]) + (x_sum_tmp_48d52_27[13])),
                ];
                let y_sum_tmp_48d52_32 = [
                    ((y_sum_tmp_48d52_28[0]) + (y_sum_tmp_48d52_28[7])),
                    ((y_sum_tmp_48d52_28[1]) + (y_sum_tmp_48d52_28[8])),
                    ((y_sum_tmp_48d52_28[2]) + (y_sum_tmp_48d52_28[9])),
                    ((y_sum_tmp_48d52_28[3]) + (y_sum_tmp_48d52_28[10])),
                    ((y_sum_tmp_48d52_28[4]) + (y_sum_tmp_48d52_28[11])),
                    ((y_sum_tmp_48d52_28[5]) + (y_sum_tmp_48d52_28[12])),
                    ((y_sum_tmp_48d52_28[6]) + (y_sum_tmp_48d52_28[13])),
                ];
                let single_karatsuba_n_7_output_tmp_48d52_33 = [
                    z0_tmp_48d52_29[0],
                    z0_tmp_48d52_29[1],
                    z0_tmp_48d52_29[2],
                    z0_tmp_48d52_29[3],
                    z0_tmp_48d52_29[4],
                    z0_tmp_48d52_29[5],
                    z0_tmp_48d52_29[6],
                    ((z0_tmp_48d52_29[7])
                        + ((((x_sum_tmp_48d52_31[0]) * (y_sum_tmp_48d52_32[0]))
                            - (z0_tmp_48d52_29[0]))
                            - (z2_tmp_48d52_30[0]))),
                    ((z0_tmp_48d52_29[8])
                        + (((((x_sum_tmp_48d52_31[0]) * (y_sum_tmp_48d52_32[1]))
                            + ((x_sum_tmp_48d52_31[1]) * (y_sum_tmp_48d52_32[0])))
                            - (z0_tmp_48d52_29[1]))
                            - (z2_tmp_48d52_30[1]))),
                    ((z0_tmp_48d52_29[9])
                        + ((((((x_sum_tmp_48d52_31[0]) * (y_sum_tmp_48d52_32[2]))
                            + ((x_sum_tmp_48d52_31[1]) * (y_sum_tmp_48d52_32[1])))
                            + ((x_sum_tmp_48d52_31[2]) * (y_sum_tmp_48d52_32[0])))
                            - (z0_tmp_48d52_29[2]))
                            - (z2_tmp_48d52_30[2]))),
                    ((z0_tmp_48d52_29[10])
                        + (((((((x_sum_tmp_48d52_31[0]) * (y_sum_tmp_48d52_32[3]))
                            + ((x_sum_tmp_48d52_31[1]) * (y_sum_tmp_48d52_32[2])))
                            + ((x_sum_tmp_48d52_31[2]) * (y_sum_tmp_48d52_32[1])))
                            + ((x_sum_tmp_48d52_31[3]) * (y_sum_tmp_48d52_32[0])))
                            - (z0_tmp_48d52_29[3]))
                            - (z2_tmp_48d52_30[3]))),
                    ((z0_tmp_48d52_29[11])
                        + ((((((((x_sum_tmp_48d52_31[0]) * (y_sum_tmp_48d52_32[4]))
                            + ((x_sum_tmp_48d52_31[1]) * (y_sum_tmp_48d52_32[3])))
                            + ((x_sum_tmp_48d52_31[2]) * (y_sum_tmp_48d52_32[2])))
                            + ((x_sum_tmp_48d52_31[3]) * (y_sum_tmp_48d52_32[1])))
                            + ((x_sum_tmp_48d52_31[4]) * (y_sum_tmp_48d52_32[0])))
                            - (z0_tmp_48d52_29[4]))
                            - (z2_tmp_48d52_30[4]))),
                    ((z0_tmp_48d52_29[12])
                        + (((((((((x_sum_tmp_48d52_31[0]) * (y_sum_tmp_48d52_32[5]))
                            + ((x_sum_tmp_48d52_31[1]) * (y_sum_tmp_48d52_32[4])))
                            + ((x_sum_tmp_48d52_31[2]) * (y_sum_tmp_48d52_32[3])))
                            + ((x_sum_tmp_48d52_31[3]) * (y_sum_tmp_48d52_32[2])))
                            + ((x_sum_tmp_48d52_31[4]) * (y_sum_tmp_48d52_32[1])))
                            + ((x_sum_tmp_48d52_31[5]) * (y_sum_tmp_48d52_32[0])))
                            - (z0_tmp_48d52_29[5]))
                            - (z2_tmp_48d52_30[5]))),
                    ((((((((((x_sum_tmp_48d52_31[0]) * (y_sum_tmp_48d52_32[6]))
                        + ((x_sum_tmp_48d52_31[1]) * (y_sum_tmp_48d52_32[5])))
                        + ((x_sum_tmp_48d52_31[2]) * (y_sum_tmp_48d52_32[4])))
                        + ((x_sum_tmp_48d52_31[3]) * (y_sum_tmp_48d52_32[3])))
                        + ((x_sum_tmp_48d52_31[4]) * (y_sum_tmp_48d52_32[2])))
                        + ((x_sum_tmp_48d52_31[5]) * (y_sum_tmp_48d52_32[1])))
                        + ((x_sum_tmp_48d52_31[6]) * (y_sum_tmp_48d52_32[0])))
                        - (z0_tmp_48d52_29[6]))
                        - (z2_tmp_48d52_30[6])),
                    ((z2_tmp_48d52_30[0])
                        + (((((((((x_sum_tmp_48d52_31[1]) * (y_sum_tmp_48d52_32[6]))
                            + ((x_sum_tmp_48d52_31[2]) * (y_sum_tmp_48d52_32[5])))
                            + ((x_sum_tmp_48d52_31[3]) * (y_sum_tmp_48d52_32[4])))
                            + ((x_sum_tmp_48d52_31[4]) * (y_sum_tmp_48d52_32[3])))
                            + ((x_sum_tmp_48d52_31[5]) * (y_sum_tmp_48d52_32[2])))
                            + ((x_sum_tmp_48d52_31[6]) * (y_sum_tmp_48d52_32[1])))
                            - (z0_tmp_48d52_29[7]))
                            - (z2_tmp_48d52_30[7]))),
                    ((z2_tmp_48d52_30[1])
                        + ((((((((x_sum_tmp_48d52_31[2]) * (y_sum_tmp_48d52_32[6]))
                            + ((x_sum_tmp_48d52_31[3]) * (y_sum_tmp_48d52_32[5])))
                            + ((x_sum_tmp_48d52_31[4]) * (y_sum_tmp_48d52_32[4])))
                            + ((x_sum_tmp_48d52_31[5]) * (y_sum_tmp_48d52_32[3])))
                            + ((x_sum_tmp_48d52_31[6]) * (y_sum_tmp_48d52_32[2])))
                            - (z0_tmp_48d52_29[8]))
                            - (z2_tmp_48d52_30[8]))),
                    ((z2_tmp_48d52_30[2])
                        + (((((((x_sum_tmp_48d52_31[3]) * (y_sum_tmp_48d52_32[6]))
                            + ((x_sum_tmp_48d52_31[4]) * (y_sum_tmp_48d52_32[5])))
                            + ((x_sum_tmp_48d52_31[5]) * (y_sum_tmp_48d52_32[4])))
                            + ((x_sum_tmp_48d52_31[6]) * (y_sum_tmp_48d52_32[3])))
                            - (z0_tmp_48d52_29[9]))
                            - (z2_tmp_48d52_30[9]))),
                    ((z2_tmp_48d52_30[3])
                        + ((((((x_sum_tmp_48d52_31[4]) * (y_sum_tmp_48d52_32[6]))
                            + ((x_sum_tmp_48d52_31[5]) * (y_sum_tmp_48d52_32[5])))
                            + ((x_sum_tmp_48d52_31[6]) * (y_sum_tmp_48d52_32[4])))
                            - (z0_tmp_48d52_29[10]))
                            - (z2_tmp_48d52_30[10]))),
                    ((z2_tmp_48d52_30[4])
                        + (((((x_sum_tmp_48d52_31[5]) * (y_sum_tmp_48d52_32[6]))
                            + ((x_sum_tmp_48d52_31[6]) * (y_sum_tmp_48d52_32[5])))
                            - (z0_tmp_48d52_29[11]))
                            - (z2_tmp_48d52_30[11]))),
                    ((z2_tmp_48d52_30[5])
                        + ((((x_sum_tmp_48d52_31[6]) * (y_sum_tmp_48d52_32[6]))
                            - (z0_tmp_48d52_29[12]))
                            - (z2_tmp_48d52_30[12]))),
                    z2_tmp_48d52_30[6],
                    z2_tmp_48d52_30[7],
                    z2_tmp_48d52_30[8],
                    z2_tmp_48d52_30[9],
                    z2_tmp_48d52_30[10],
                    z2_tmp_48d52_30[11],
                    z2_tmp_48d52_30[12],
                ];

                let double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34 = [
                    single_karatsuba_n_7_output_tmp_48d52_21[0],
                    single_karatsuba_n_7_output_tmp_48d52_21[1],
                    single_karatsuba_n_7_output_tmp_48d52_21[2],
                    single_karatsuba_n_7_output_tmp_48d52_21[3],
                    single_karatsuba_n_7_output_tmp_48d52_21[4],
                    single_karatsuba_n_7_output_tmp_48d52_21[5],
                    single_karatsuba_n_7_output_tmp_48d52_21[6],
                    single_karatsuba_n_7_output_tmp_48d52_21[7],
                    single_karatsuba_n_7_output_tmp_48d52_21[8],
                    single_karatsuba_n_7_output_tmp_48d52_21[9],
                    single_karatsuba_n_7_output_tmp_48d52_21[10],
                    single_karatsuba_n_7_output_tmp_48d52_21[11],
                    single_karatsuba_n_7_output_tmp_48d52_21[12],
                    single_karatsuba_n_7_output_tmp_48d52_21[13],
                    ((single_karatsuba_n_7_output_tmp_48d52_21[14])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[0])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[0]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[0]))),
                    ((single_karatsuba_n_7_output_tmp_48d52_21[15])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[1])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[1]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[1]))),
                    ((single_karatsuba_n_7_output_tmp_48d52_21[16])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[2])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[2]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[2]))),
                    ((single_karatsuba_n_7_output_tmp_48d52_21[17])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[3])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[3]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[3]))),
                    ((single_karatsuba_n_7_output_tmp_48d52_21[18])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[4])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[4]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[4]))),
                    ((single_karatsuba_n_7_output_tmp_48d52_21[19])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[5])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[5]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[5]))),
                    ((single_karatsuba_n_7_output_tmp_48d52_21[20])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[6])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[6]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[6]))),
                    ((single_karatsuba_n_7_output_tmp_48d52_21[21])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[7])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[7]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[7]))),
                    ((single_karatsuba_n_7_output_tmp_48d52_21[22])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[8])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[8]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[8]))),
                    ((single_karatsuba_n_7_output_tmp_48d52_21[23])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[9])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[9]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[9]))),
                    ((single_karatsuba_n_7_output_tmp_48d52_21[24])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[10])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[10]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[10]))),
                    ((single_karatsuba_n_7_output_tmp_48d52_21[25])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[11])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[11]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[11]))),
                    ((single_karatsuba_n_7_output_tmp_48d52_21[26])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[12])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[12]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[12]))),
                    (((single_karatsuba_n_7_output_tmp_48d52_33[13])
                        - (single_karatsuba_n_7_output_tmp_48d52_21[13]))
                        - (single_karatsuba_n_7_output_tmp_48d52_26[13])),
                    ((single_karatsuba_n_7_output_tmp_48d52_26[0])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[14])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[14]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[14]))),
                    ((single_karatsuba_n_7_output_tmp_48d52_26[1])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[15])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[15]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[15]))),
                    ((single_karatsuba_n_7_output_tmp_48d52_26[2])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[16])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[16]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[16]))),
                    ((single_karatsuba_n_7_output_tmp_48d52_26[3])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[17])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[17]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[17]))),
                    ((single_karatsuba_n_7_output_tmp_48d52_26[4])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[18])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[18]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[18]))),
                    ((single_karatsuba_n_7_output_tmp_48d52_26[5])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[19])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[19]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[19]))),
                    ((single_karatsuba_n_7_output_tmp_48d52_26[6])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[20])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[20]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[20]))),
                    ((single_karatsuba_n_7_output_tmp_48d52_26[7])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[21])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[21]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[21]))),
                    ((single_karatsuba_n_7_output_tmp_48d52_26[8])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[22])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[22]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[22]))),
                    ((single_karatsuba_n_7_output_tmp_48d52_26[9])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[23])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[23]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[23]))),
                    ((single_karatsuba_n_7_output_tmp_48d52_26[10])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[24])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[24]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[24]))),
                    ((single_karatsuba_n_7_output_tmp_48d52_26[11])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[25])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[25]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[25]))),
                    ((single_karatsuba_n_7_output_tmp_48d52_26[12])
                        + (((single_karatsuba_n_7_output_tmp_48d52_33[26])
                            - (single_karatsuba_n_7_output_tmp_48d52_21[26]))
                            - (single_karatsuba_n_7_output_tmp_48d52_26[26]))),
                    single_karatsuba_n_7_output_tmp_48d52_26[13],
                    single_karatsuba_n_7_output_tmp_48d52_26[14],
                    single_karatsuba_n_7_output_tmp_48d52_26[15],
                    single_karatsuba_n_7_output_tmp_48d52_26[16],
                    single_karatsuba_n_7_output_tmp_48d52_26[17],
                    single_karatsuba_n_7_output_tmp_48d52_26[18],
                    single_karatsuba_n_7_output_tmp_48d52_26[19],
                    single_karatsuba_n_7_output_tmp_48d52_26[20],
                    single_karatsuba_n_7_output_tmp_48d52_26[21],
                    single_karatsuba_n_7_output_tmp_48d52_26[22],
                    single_karatsuba_n_7_output_tmp_48d52_26[23],
                    single_karatsuba_n_7_output_tmp_48d52_26[24],
                    single_karatsuba_n_7_output_tmp_48d52_26[25],
                    single_karatsuba_n_7_output_tmp_48d52_26[26],
                ];

                let conv_tmp_48d52_35 = [
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[0])
                        - (dst_limb_0_col11)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[1])
                        - (dst_limb_1_col12)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[2])
                        - (dst_limb_2_col13)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[3])
                        - (dst_limb_3_col14)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[4])
                        - (dst_limb_4_col15)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[5])
                        - (dst_limb_5_col16)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[6])
                        - (dst_limb_6_col17)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[7])
                        - (dst_limb_7_col18)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[8])
                        - (dst_limb_8_col19)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[9])
                        - (dst_limb_9_col20)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[10])
                        - (dst_limb_10_col21)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[11])
                        - (dst_limb_11_col22)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[12])
                        - (dst_limb_12_col23)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[13])
                        - (dst_limb_13_col24)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[14])
                        - (dst_limb_14_col25)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[15])
                        - (dst_limb_15_col26)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[16])
                        - (dst_limb_16_col27)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[17])
                        - (dst_limb_17_col28)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[18])
                        - (dst_limb_18_col29)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[19])
                        - (dst_limb_19_col30)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[20])
                        - (dst_limb_20_col31)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[21])
                        - (dst_limb_21_col32)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[22])
                        - (dst_limb_22_col33)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[23])
                        - (dst_limb_23_col34)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[24])
                        - (dst_limb_24_col35)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[25])
                        - (dst_limb_25_col36)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[26])
                        - (dst_limb_26_col37)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[27])
                        - (dst_limb_27_col38)),
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[28],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[29],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[30],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[31],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[32],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[33],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[34],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[35],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[36],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[37],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[38],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[39],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[40],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[41],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[42],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[43],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[44],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[45],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[46],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[47],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[48],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[49],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[50],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[51],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[52],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[53],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_48d52_34[54],
                ];
                let conv_mod_tmp_48d52_36 = [
                    ((((M31_32) * (conv_tmp_48d52_35[0])) - ((M31_4) * (conv_tmp_48d52_35[21])))
                        + ((M31_8) * (conv_tmp_48d52_35[49]))),
                    ((((conv_tmp_48d52_35[0]) + ((M31_32) * (conv_tmp_48d52_35[1])))
                        - ((M31_4) * (conv_tmp_48d52_35[22])))
                        + ((M31_8) * (conv_tmp_48d52_35[50]))),
                    ((((conv_tmp_48d52_35[1]) + ((M31_32) * (conv_tmp_48d52_35[2])))
                        - ((M31_4) * (conv_tmp_48d52_35[23])))
                        + ((M31_8) * (conv_tmp_48d52_35[51]))),
                    ((((conv_tmp_48d52_35[2]) + ((M31_32) * (conv_tmp_48d52_35[3])))
                        - ((M31_4) * (conv_tmp_48d52_35[24])))
                        + ((M31_8) * (conv_tmp_48d52_35[52]))),
                    ((((conv_tmp_48d52_35[3]) + ((M31_32) * (conv_tmp_48d52_35[4])))
                        - ((M31_4) * (conv_tmp_48d52_35[25])))
                        + ((M31_8) * (conv_tmp_48d52_35[53]))),
                    ((((conv_tmp_48d52_35[4]) + ((M31_32) * (conv_tmp_48d52_35[5])))
                        - ((M31_4) * (conv_tmp_48d52_35[26])))
                        + ((M31_8) * (conv_tmp_48d52_35[54]))),
                    (((conv_tmp_48d52_35[5]) + ((M31_32) * (conv_tmp_48d52_35[6])))
                        - ((M31_4) * (conv_tmp_48d52_35[27]))),
                    (((((M31_2) * (conv_tmp_48d52_35[0])) + (conv_tmp_48d52_35[6]))
                        + ((M31_32) * (conv_tmp_48d52_35[7])))
                        - ((M31_4) * (conv_tmp_48d52_35[28]))),
                    (((((M31_2) * (conv_tmp_48d52_35[1])) + (conv_tmp_48d52_35[7]))
                        + ((M31_32) * (conv_tmp_48d52_35[8])))
                        - ((M31_4) * (conv_tmp_48d52_35[29]))),
                    (((((M31_2) * (conv_tmp_48d52_35[2])) + (conv_tmp_48d52_35[8]))
                        + ((M31_32) * (conv_tmp_48d52_35[9])))
                        - ((M31_4) * (conv_tmp_48d52_35[30]))),
                    (((((M31_2) * (conv_tmp_48d52_35[3])) + (conv_tmp_48d52_35[9]))
                        + ((M31_32) * (conv_tmp_48d52_35[10])))
                        - ((M31_4) * (conv_tmp_48d52_35[31]))),
                    (((((M31_2) * (conv_tmp_48d52_35[4])) + (conv_tmp_48d52_35[10]))
                        + ((M31_32) * (conv_tmp_48d52_35[11])))
                        - ((M31_4) * (conv_tmp_48d52_35[32]))),
                    (((((M31_2) * (conv_tmp_48d52_35[5])) + (conv_tmp_48d52_35[11]))
                        + ((M31_32) * (conv_tmp_48d52_35[12])))
                        - ((M31_4) * (conv_tmp_48d52_35[33]))),
                    (((((M31_2) * (conv_tmp_48d52_35[6])) + (conv_tmp_48d52_35[12]))
                        + ((M31_32) * (conv_tmp_48d52_35[13])))
                        - ((M31_4) * (conv_tmp_48d52_35[34]))),
                    (((((M31_2) * (conv_tmp_48d52_35[7])) + (conv_tmp_48d52_35[13]))
                        + ((M31_32) * (conv_tmp_48d52_35[14])))
                        - ((M31_4) * (conv_tmp_48d52_35[35]))),
                    (((((M31_2) * (conv_tmp_48d52_35[8])) + (conv_tmp_48d52_35[14]))
                        + ((M31_32) * (conv_tmp_48d52_35[15])))
                        - ((M31_4) * (conv_tmp_48d52_35[36]))),
                    (((((M31_2) * (conv_tmp_48d52_35[9])) + (conv_tmp_48d52_35[15]))
                        + ((M31_32) * (conv_tmp_48d52_35[16])))
                        - ((M31_4) * (conv_tmp_48d52_35[37]))),
                    (((((M31_2) * (conv_tmp_48d52_35[10])) + (conv_tmp_48d52_35[16]))
                        + ((M31_32) * (conv_tmp_48d52_35[17])))
                        - ((M31_4) * (conv_tmp_48d52_35[38]))),
                    (((((M31_2) * (conv_tmp_48d52_35[11])) + (conv_tmp_48d52_35[17]))
                        + ((M31_32) * (conv_tmp_48d52_35[18])))
                        - ((M31_4) * (conv_tmp_48d52_35[39]))),
                    (((((M31_2) * (conv_tmp_48d52_35[12])) + (conv_tmp_48d52_35[18]))
                        + ((M31_32) * (conv_tmp_48d52_35[19])))
                        - ((M31_4) * (conv_tmp_48d52_35[40]))),
                    (((((M31_2) * (conv_tmp_48d52_35[13])) + (conv_tmp_48d52_35[19]))
                        + ((M31_32) * (conv_tmp_48d52_35[20])))
                        - ((M31_4) * (conv_tmp_48d52_35[41]))),
                    (((((M31_2) * (conv_tmp_48d52_35[14])) + (conv_tmp_48d52_35[20]))
                        - ((M31_4) * (conv_tmp_48d52_35[42])))
                        + ((M31_64) * (conv_tmp_48d52_35[49]))),
                    (((((M31_2) * (conv_tmp_48d52_35[15])) - ((M31_4) * (conv_tmp_48d52_35[43])))
                        + ((M31_2) * (conv_tmp_48d52_35[49])))
                        + ((M31_64) * (conv_tmp_48d52_35[50]))),
                    (((((M31_2) * (conv_tmp_48d52_35[16])) - ((M31_4) * (conv_tmp_48d52_35[44])))
                        + ((M31_2) * (conv_tmp_48d52_35[50])))
                        + ((M31_64) * (conv_tmp_48d52_35[51]))),
                    (((((M31_2) * (conv_tmp_48d52_35[17])) - ((M31_4) * (conv_tmp_48d52_35[45])))
                        + ((M31_2) * (conv_tmp_48d52_35[51])))
                        + ((M31_64) * (conv_tmp_48d52_35[52]))),
                    (((((M31_2) * (conv_tmp_48d52_35[18])) - ((M31_4) * (conv_tmp_48d52_35[46])))
                        + ((M31_2) * (conv_tmp_48d52_35[52])))
                        + ((M31_64) * (conv_tmp_48d52_35[53]))),
                    (((((M31_2) * (conv_tmp_48d52_35[19])) - ((M31_4) * (conv_tmp_48d52_35[47])))
                        + ((M31_2) * (conv_tmp_48d52_35[53])))
                        + ((M31_64) * (conv_tmp_48d52_35[54]))),
                    ((((M31_2) * (conv_tmp_48d52_35[20])) - ((M31_4) * (conv_tmp_48d52_35[48])))
                        + ((M31_2) * (conv_tmp_48d52_35[54]))),
                ];
                let k_mod_2_18_biased_tmp_48d52_37 =
                    ((((PackedUInt32::from_m31(((conv_mod_tmp_48d52_36[0]) + (M31_134217728))))
                        + (((PackedUInt32::from_m31(
                            ((conv_mod_tmp_48d52_36[1]) + (M31_134217728)),
                        )) & (UInt32_511))
                            << (UInt32_9)))
                        + (UInt32_65536))
                        & (UInt32_262143));
                let k_col97 = ((k_mod_2_18_biased_tmp_48d52_37.low().as_m31())
                    + (((k_mod_2_18_biased_tmp_48d52_37.high().as_m31()) - (M31_1)) * (M31_65536)));
                *row[97] = k_col97;
                *sub_component_inputs.range_check_19[0] = [((k_col97) + (M31_262144))];
                *lookup_data.range_check_19_0 = [((k_col97) + (M31_262144))];
                let carry_0_col98 = (((conv_mod_tmp_48d52_36[0]) - (k_col97)) * (M31_4194304));
                *row[98] = carry_0_col98;
                *sub_component_inputs.range_check_19[1] = [((carry_0_col98) + (M31_131072))];
                *lookup_data.range_check_19_1 = [((carry_0_col98) + (M31_131072))];
                let carry_1_col99 =
                    (((conv_mod_tmp_48d52_36[1]) + (carry_0_col98)) * (M31_4194304));
                *row[99] = carry_1_col99;
                *sub_component_inputs.range_check_19[2] = [((carry_1_col99) + (M31_131072))];
                *lookup_data.range_check_19_2 = [((carry_1_col99) + (M31_131072))];
                let carry_2_col100 =
                    (((conv_mod_tmp_48d52_36[2]) + (carry_1_col99)) * (M31_4194304));
                *row[100] = carry_2_col100;
                *sub_component_inputs.range_check_19[3] = [((carry_2_col100) + (M31_131072))];
                *lookup_data.range_check_19_3 = [((carry_2_col100) + (M31_131072))];
                let carry_3_col101 =
                    (((conv_mod_tmp_48d52_36[3]) + (carry_2_col100)) * (M31_4194304));
                *row[101] = carry_3_col101;
                *sub_component_inputs.range_check_19[4] = [((carry_3_col101) + (M31_131072))];
                *lookup_data.range_check_19_4 = [((carry_3_col101) + (M31_131072))];
                let carry_4_col102 =
                    (((conv_mod_tmp_48d52_36[4]) + (carry_3_col101)) * (M31_4194304));
                *row[102] = carry_4_col102;
                *sub_component_inputs.range_check_19[5] = [((carry_4_col102) + (M31_131072))];
                *lookup_data.range_check_19_5 = [((carry_4_col102) + (M31_131072))];
                let carry_5_col103 =
                    (((conv_mod_tmp_48d52_36[5]) + (carry_4_col102)) * (M31_4194304));
                *row[103] = carry_5_col103;
                *sub_component_inputs.range_check_19[6] = [((carry_5_col103) + (M31_131072))];
                *lookup_data.range_check_19_6 = [((carry_5_col103) + (M31_131072))];
                let carry_6_col104 =
                    (((conv_mod_tmp_48d52_36[6]) + (carry_5_col103)) * (M31_4194304));
                *row[104] = carry_6_col104;
                *sub_component_inputs.range_check_19[7] = [((carry_6_col104) + (M31_131072))];
                *lookup_data.range_check_19_7 = [((carry_6_col104) + (M31_131072))];
                let carry_7_col105 =
                    (((conv_mod_tmp_48d52_36[7]) + (carry_6_col104)) * (M31_4194304));
                *row[105] = carry_7_col105;
                *sub_component_inputs.range_check_19[8] = [((carry_7_col105) + (M31_131072))];
                *lookup_data.range_check_19_8 = [((carry_7_col105) + (M31_131072))];
                let carry_8_col106 =
                    (((conv_mod_tmp_48d52_36[8]) + (carry_7_col105)) * (M31_4194304));
                *row[106] = carry_8_col106;
                *sub_component_inputs.range_check_19[9] = [((carry_8_col106) + (M31_131072))];
                *lookup_data.range_check_19_9 = [((carry_8_col106) + (M31_131072))];
                let carry_9_col107 =
                    (((conv_mod_tmp_48d52_36[9]) + (carry_8_col106)) * (M31_4194304));
                *row[107] = carry_9_col107;
                *sub_component_inputs.range_check_19[10] = [((carry_9_col107) + (M31_131072))];
                *lookup_data.range_check_19_10 = [((carry_9_col107) + (M31_131072))];
                let carry_10_col108 =
                    (((conv_mod_tmp_48d52_36[10]) + (carry_9_col107)) * (M31_4194304));
                *row[108] = carry_10_col108;
                *sub_component_inputs.range_check_19[11] = [((carry_10_col108) + (M31_131072))];
                *lookup_data.range_check_19_11 = [((carry_10_col108) + (M31_131072))];
                let carry_11_col109 =
                    (((conv_mod_tmp_48d52_36[11]) + (carry_10_col108)) * (M31_4194304));
                *row[109] = carry_11_col109;
                *sub_component_inputs.range_check_19[12] = [((carry_11_col109) + (M31_131072))];
                *lookup_data.range_check_19_12 = [((carry_11_col109) + (M31_131072))];
                let carry_12_col110 =
                    (((conv_mod_tmp_48d52_36[12]) + (carry_11_col109)) * (M31_4194304));
                *row[110] = carry_12_col110;
                *sub_component_inputs.range_check_19[13] = [((carry_12_col110) + (M31_131072))];
                *lookup_data.range_check_19_13 = [((carry_12_col110) + (M31_131072))];
                let carry_13_col111 =
                    (((conv_mod_tmp_48d52_36[13]) + (carry_12_col110)) * (M31_4194304));
                *row[111] = carry_13_col111;
                *sub_component_inputs.range_check_19[14] = [((carry_13_col111) + (M31_131072))];
                *lookup_data.range_check_19_14 = [((carry_13_col111) + (M31_131072))];
                let carry_14_col112 =
                    (((conv_mod_tmp_48d52_36[14]) + (carry_13_col111)) * (M31_4194304));
                *row[112] = carry_14_col112;
                *sub_component_inputs.range_check_19[15] = [((carry_14_col112) + (M31_131072))];
                *lookup_data.range_check_19_15 = [((carry_14_col112) + (M31_131072))];
                let carry_15_col113 =
                    (((conv_mod_tmp_48d52_36[15]) + (carry_14_col112)) * (M31_4194304));
                *row[113] = carry_15_col113;
                *sub_component_inputs.range_check_19[16] = [((carry_15_col113) + (M31_131072))];
                *lookup_data.range_check_19_16 = [((carry_15_col113) + (M31_131072))];
                let carry_16_col114 =
                    (((conv_mod_tmp_48d52_36[16]) + (carry_15_col113)) * (M31_4194304));
                *row[114] = carry_16_col114;
                *sub_component_inputs.range_check_19[17] = [((carry_16_col114) + (M31_131072))];
                *lookup_data.range_check_19_17 = [((carry_16_col114) + (M31_131072))];
                let carry_17_col115 =
                    (((conv_mod_tmp_48d52_36[17]) + (carry_16_col114)) * (M31_4194304));
                *row[115] = carry_17_col115;
                *sub_component_inputs.range_check_19[18] = [((carry_17_col115) + (M31_131072))];
                *lookup_data.range_check_19_18 = [((carry_17_col115) + (M31_131072))];
                let carry_18_col116 =
                    (((conv_mod_tmp_48d52_36[18]) + (carry_17_col115)) * (M31_4194304));
                *row[116] = carry_18_col116;
                *sub_component_inputs.range_check_19[19] = [((carry_18_col116) + (M31_131072))];
                *lookup_data.range_check_19_19 = [((carry_18_col116) + (M31_131072))];
                let carry_19_col117 =
                    (((conv_mod_tmp_48d52_36[19]) + (carry_18_col116)) * (M31_4194304));
                *row[117] = carry_19_col117;
                *sub_component_inputs.range_check_19[20] = [((carry_19_col117) + (M31_131072))];
                *lookup_data.range_check_19_20 = [((carry_19_col117) + (M31_131072))];
                let carry_20_col118 =
                    (((conv_mod_tmp_48d52_36[20]) + (carry_19_col117)) * (M31_4194304));
                *row[118] = carry_20_col118;
                *sub_component_inputs.range_check_19[21] = [((carry_20_col118) + (M31_131072))];
                *lookup_data.range_check_19_21 = [((carry_20_col118) + (M31_131072))];
                let carry_21_col119 = ((((conv_mod_tmp_48d52_36[21]) - ((M31_136) * (k_col97)))
                    + (carry_20_col118))
                    * (M31_4194304));
                *row[119] = carry_21_col119;
                *sub_component_inputs.range_check_19[22] = [((carry_21_col119) + (M31_131072))];
                *lookup_data.range_check_19_22 = [((carry_21_col119) + (M31_131072))];
                let carry_22_col120 =
                    (((conv_mod_tmp_48d52_36[22]) + (carry_21_col119)) * (M31_4194304));
                *row[120] = carry_22_col120;
                *sub_component_inputs.range_check_19[23] = [((carry_22_col120) + (M31_131072))];
                *lookup_data.range_check_19_23 = [((carry_22_col120) + (M31_131072))];
                let carry_23_col121 =
                    (((conv_mod_tmp_48d52_36[23]) + (carry_22_col120)) * (M31_4194304));
                *row[121] = carry_23_col121;
                *sub_component_inputs.range_check_19[24] = [((carry_23_col121) + (M31_131072))];
                *lookup_data.range_check_19_24 = [((carry_23_col121) + (M31_131072))];
                let carry_24_col122 =
                    (((conv_mod_tmp_48d52_36[24]) + (carry_23_col121)) * (M31_4194304));
                *row[122] = carry_24_col122;
                *sub_component_inputs.range_check_19[25] = [((carry_24_col122) + (M31_131072))];
                *lookup_data.range_check_19_25 = [((carry_24_col122) + (M31_131072))];
                let carry_25_col123 =
                    (((conv_mod_tmp_48d52_36[25]) + (carry_24_col122)) * (M31_4194304));
                *row[123] = carry_25_col123;
                *sub_component_inputs.range_check_19[26] = [((carry_25_col123) + (M31_131072))];
                *lookup_data.range_check_19_26 = [((carry_25_col123) + (M31_131072))];
                let carry_26_col124 =
                    (((conv_mod_tmp_48d52_36[26]) + (carry_25_col123)) * (M31_4194304));
                *row[124] = carry_26_col124;
                *sub_component_inputs.range_check_19[27] = [((carry_26_col124) + (M31_131072))];
                *lookup_data.range_check_19_27 = [((carry_26_col124) + (M31_131072))];

                *lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];
                *lookup_data.opcodes_1 = [
                    ((input_pc_col0) + (M31_2)),
                    ((input_ap_col1) + (ap_update_add_1_col7)),
                    input_fp_col2,
                ];
                *row[125] = enabler_col.packed_at(row_index);
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
    range_check_19_0: Vec<[PackedM31; 1]>,
    range_check_19_1: Vec<[PackedM31; 1]>,
    range_check_19_2: Vec<[PackedM31; 1]>,
    range_check_19_3: Vec<[PackedM31; 1]>,
    range_check_19_4: Vec<[PackedM31; 1]>,
    range_check_19_5: Vec<[PackedM31; 1]>,
    range_check_19_6: Vec<[PackedM31; 1]>,
    range_check_19_7: Vec<[PackedM31; 1]>,
    range_check_19_8: Vec<[PackedM31; 1]>,
    range_check_19_9: Vec<[PackedM31; 1]>,
    range_check_19_10: Vec<[PackedM31; 1]>,
    range_check_19_11: Vec<[PackedM31; 1]>,
    range_check_19_12: Vec<[PackedM31; 1]>,
    range_check_19_13: Vec<[PackedM31; 1]>,
    range_check_19_14: Vec<[PackedM31; 1]>,
    range_check_19_15: Vec<[PackedM31; 1]>,
    range_check_19_16: Vec<[PackedM31; 1]>,
    range_check_19_17: Vec<[PackedM31; 1]>,
    range_check_19_18: Vec<[PackedM31; 1]>,
    range_check_19_19: Vec<[PackedM31; 1]>,
    range_check_19_20: Vec<[PackedM31; 1]>,
    range_check_19_21: Vec<[PackedM31; 1]>,
    range_check_19_22: Vec<[PackedM31; 1]>,
    range_check_19_23: Vec<[PackedM31; 1]>,
    range_check_19_24: Vec<[PackedM31; 1]>,
    range_check_19_25: Vec<[PackedM31; 1]>,
    range_check_19_26: Vec<[PackedM31; 1]>,
    range_check_19_27: Vec<[PackedM31; 1]>,
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
        range_check_19: &relations::RangeCheck_19,
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
            &self.lookup_data.range_check_19_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = range_check_19.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_1,
            &self.lookup_data.range_check_19_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19.combine(values0);
                let denom1: PackedQM31 = range_check_19.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_3,
            &self.lookup_data.range_check_19_4,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19.combine(values0);
                let denom1: PackedQM31 = range_check_19.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_5,
            &self.lookup_data.range_check_19_6,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19.combine(values0);
                let denom1: PackedQM31 = range_check_19.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_7,
            &self.lookup_data.range_check_19_8,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19.combine(values0);
                let denom1: PackedQM31 = range_check_19.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_9,
            &self.lookup_data.range_check_19_10,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19.combine(values0);
                let denom1: PackedQM31 = range_check_19.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_11,
            &self.lookup_data.range_check_19_12,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19.combine(values0);
                let denom1: PackedQM31 = range_check_19.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_13,
            &self.lookup_data.range_check_19_14,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19.combine(values0);
                let denom1: PackedQM31 = range_check_19.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_15,
            &self.lookup_data.range_check_19_16,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19.combine(values0);
                let denom1: PackedQM31 = range_check_19.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_17,
            &self.lookup_data.range_check_19_18,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19.combine(values0);
                let denom1: PackedQM31 = range_check_19.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_19,
            &self.lookup_data.range_check_19_20,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19.combine(values0);
                let denom1: PackedQM31 = range_check_19.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_21,
            &self.lookup_data.range_check_19_22,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19.combine(values0);
                let denom1: PackedQM31 = range_check_19.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_23,
            &self.lookup_data.range_check_19_24,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19.combine(values0);
                let denom1: PackedQM31 = range_check_19.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_25,
            &self.lookup_data.range_check_19_26,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19.combine(values0);
                let denom1: PackedQM31 = range_check_19.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_27,
            &self.lookup_data.opcodes_0,
        )
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (writer, values0, values1))| {
                let denom0: PackedQM31 = range_check_19.combine(values0);
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
