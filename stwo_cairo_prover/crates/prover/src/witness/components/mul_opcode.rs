#![allow(unused_parens)]
use crate::cairo_air::components::mul_opcode::{Claim, InteractionClaim, N_TRACE_COLUMNS};
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
            n_rows,
            packed_inputs,
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
    n_rows: usize,
    inputs: Vec<PackedInputType>,
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
    let M31_128 = PackedM31::broadcast(M31::from(128));
    let M31_131072 = PackedM31::broadcast(M31::from(131072));
    let M31_134217728 = PackedM31::broadcast(M31::from(134217728));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_16 = PackedM31::broadcast(M31::from(16));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_4 = PackedM31::broadcast(M31::from(4));
    let M31_4194304 = PackedM31::broadcast(M31::from(4194304));
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
    let UInt32_262143 = PackedUInt32::broadcast(UInt32::from(262143));
    let UInt32_511 = PackedUInt32::broadcast(UInt32::from(511));
    let UInt32_65536 = PackedUInt32::broadcast(UInt32::from(65536));
    let UInt32_9 = PackedUInt32::broadcast(UInt32::from(9));
    let padding_col = Enabler::new(n_rows);

    (
        trace.par_iter_mut(),
        lookup_data.par_iter_mut(),
        sub_component_inputs.par_iter_mut(),
        inputs.into_par_iter(),
    )
        .into_par_iter()
        .enumerate()
        .for_each(
            |(row_index, (mut row, lookup_data, sub_component_inputs, mul_opcode_input))| {
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
                let op1_base_fp_tmp_42314_7 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_42314_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_3))
                        & (UInt16_1));
                let op1_base_fp_col8 = op1_base_fp_tmp_42314_7.as_m31();
                *row[8] = op1_base_fp_col8;
                let ap_update_add_1_tmp_42314_8 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_42314_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_11))
                        & (UInt16_1));
                let ap_update_add_1_col9 = ap_update_add_1_tmp_42314_8.as_m31();
                *row[9] = ap_update_add_1_col9;
                *sub_component_inputs.verify_instruction[0] = (
                    input_pc_col0,
                    [offset0_col3, offset1_col4, offset2_col5],
                    [
                        (((((dst_base_fp_col6) * (M31_8)) + ((op0_base_fp_col7) * (M31_16)))
                            + ((op1_base_fp_col8) * (M31_64)))
                            + (((M31_1) - (op1_base_fp_col8)) * (M31_128))),
                        (((M31_1) + ((ap_update_add_1_col9) * (M31_32))) + (M31_256)),
                    ],
                    M31_0,
                );
                *lookup_data.verify_instruction_0 = [
                    input_pc_col0,
                    offset0_col3,
                    offset1_col4,
                    offset2_col5,
                    (((((dst_base_fp_col6) * (M31_8)) + ((op0_base_fp_col7) * (M31_16)))
                        + ((op1_base_fp_col8) * (M31_64)))
                        + (((M31_1) - (op1_base_fp_col8)) * (M31_128))),
                    (((M31_1) + ((ap_update_add_1_col9) * (M31_32))) + (M31_256)),
                    M31_0,
                ];

                let mem_dst_base_col10 = (((dst_base_fp_col6) * (input_fp_col2))
                    + (((M31_1) - (dst_base_fp_col6)) * (input_ap_col1)));
                *row[10] = mem_dst_base_col10;
                let mem0_base_col11 = (((op0_base_fp_col7) * (input_fp_col2))
                    + (((M31_1) - (op0_base_fp_col7)) * (input_ap_col1)));
                *row[11] = mem0_base_col11;
                let mem1_base_col12 = (((op1_base_fp_col8) * (input_fp_col2))
                    + (((M31_1) - (op1_base_fp_col8)) * (input_ap_col1)));
                *row[12] = mem1_base_col12;

                // Read Positive Num Bits 252.

                let memory_address_to_id_value_tmp_42314_9 = memory_address_to_id_state
                    .deduce_output(((mem_dst_base_col10) + ((offset0_col3) - (M31_32768))));
                let memory_id_to_big_value_tmp_42314_10 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_42314_9);
                let dst_id_col13 = memory_address_to_id_value_tmp_42314_9;
                *row[13] = dst_id_col13;
                *sub_component_inputs.memory_address_to_id[0] =
                    ((mem_dst_base_col10) + ((offset0_col3) - (M31_32768)));
                *lookup_data.memory_address_to_id_0 = [
                    ((mem_dst_base_col10) + ((offset0_col3) - (M31_32768))),
                    dst_id_col13,
                ];
                let dst_limb_0_col14 = memory_id_to_big_value_tmp_42314_10.get_m31(0);
                *row[14] = dst_limb_0_col14;
                let dst_limb_1_col15 = memory_id_to_big_value_tmp_42314_10.get_m31(1);
                *row[15] = dst_limb_1_col15;
                let dst_limb_2_col16 = memory_id_to_big_value_tmp_42314_10.get_m31(2);
                *row[16] = dst_limb_2_col16;
                let dst_limb_3_col17 = memory_id_to_big_value_tmp_42314_10.get_m31(3);
                *row[17] = dst_limb_3_col17;
                let dst_limb_4_col18 = memory_id_to_big_value_tmp_42314_10.get_m31(4);
                *row[18] = dst_limb_4_col18;
                let dst_limb_5_col19 = memory_id_to_big_value_tmp_42314_10.get_m31(5);
                *row[19] = dst_limb_5_col19;
                let dst_limb_6_col20 = memory_id_to_big_value_tmp_42314_10.get_m31(6);
                *row[20] = dst_limb_6_col20;
                let dst_limb_7_col21 = memory_id_to_big_value_tmp_42314_10.get_m31(7);
                *row[21] = dst_limb_7_col21;
                let dst_limb_8_col22 = memory_id_to_big_value_tmp_42314_10.get_m31(8);
                *row[22] = dst_limb_8_col22;
                let dst_limb_9_col23 = memory_id_to_big_value_tmp_42314_10.get_m31(9);
                *row[23] = dst_limb_9_col23;
                let dst_limb_10_col24 = memory_id_to_big_value_tmp_42314_10.get_m31(10);
                *row[24] = dst_limb_10_col24;
                let dst_limb_11_col25 = memory_id_to_big_value_tmp_42314_10.get_m31(11);
                *row[25] = dst_limb_11_col25;
                let dst_limb_12_col26 = memory_id_to_big_value_tmp_42314_10.get_m31(12);
                *row[26] = dst_limb_12_col26;
                let dst_limb_13_col27 = memory_id_to_big_value_tmp_42314_10.get_m31(13);
                *row[27] = dst_limb_13_col27;
                let dst_limb_14_col28 = memory_id_to_big_value_tmp_42314_10.get_m31(14);
                *row[28] = dst_limb_14_col28;
                let dst_limb_15_col29 = memory_id_to_big_value_tmp_42314_10.get_m31(15);
                *row[29] = dst_limb_15_col29;
                let dst_limb_16_col30 = memory_id_to_big_value_tmp_42314_10.get_m31(16);
                *row[30] = dst_limb_16_col30;
                let dst_limb_17_col31 = memory_id_to_big_value_tmp_42314_10.get_m31(17);
                *row[31] = dst_limb_17_col31;
                let dst_limb_18_col32 = memory_id_to_big_value_tmp_42314_10.get_m31(18);
                *row[32] = dst_limb_18_col32;
                let dst_limb_19_col33 = memory_id_to_big_value_tmp_42314_10.get_m31(19);
                *row[33] = dst_limb_19_col33;
                let dst_limb_20_col34 = memory_id_to_big_value_tmp_42314_10.get_m31(20);
                *row[34] = dst_limb_20_col34;
                let dst_limb_21_col35 = memory_id_to_big_value_tmp_42314_10.get_m31(21);
                *row[35] = dst_limb_21_col35;
                let dst_limb_22_col36 = memory_id_to_big_value_tmp_42314_10.get_m31(22);
                *row[36] = dst_limb_22_col36;
                let dst_limb_23_col37 = memory_id_to_big_value_tmp_42314_10.get_m31(23);
                *row[37] = dst_limb_23_col37;
                let dst_limb_24_col38 = memory_id_to_big_value_tmp_42314_10.get_m31(24);
                *row[38] = dst_limb_24_col38;
                let dst_limb_25_col39 = memory_id_to_big_value_tmp_42314_10.get_m31(25);
                *row[39] = dst_limb_25_col39;
                let dst_limb_26_col40 = memory_id_to_big_value_tmp_42314_10.get_m31(26);
                *row[40] = dst_limb_26_col40;
                let dst_limb_27_col41 = memory_id_to_big_value_tmp_42314_10.get_m31(27);
                *row[41] = dst_limb_27_col41;
                *sub_component_inputs.memory_id_to_big[0] = dst_id_col13;
                *lookup_data.memory_id_to_big_0 = [
                    dst_id_col13,
                    dst_limb_0_col14,
                    dst_limb_1_col15,
                    dst_limb_2_col16,
                    dst_limb_3_col17,
                    dst_limb_4_col18,
                    dst_limb_5_col19,
                    dst_limb_6_col20,
                    dst_limb_7_col21,
                    dst_limb_8_col22,
                    dst_limb_9_col23,
                    dst_limb_10_col24,
                    dst_limb_11_col25,
                    dst_limb_12_col26,
                    dst_limb_13_col27,
                    dst_limb_14_col28,
                    dst_limb_15_col29,
                    dst_limb_16_col30,
                    dst_limb_17_col31,
                    dst_limb_18_col32,
                    dst_limb_19_col33,
                    dst_limb_20_col34,
                    dst_limb_21_col35,
                    dst_limb_22_col36,
                    dst_limb_23_col37,
                    dst_limb_24_col38,
                    dst_limb_25_col39,
                    dst_limb_26_col40,
                    dst_limb_27_col41,
                ];

                // Read Positive Num Bits 252.

                let memory_address_to_id_value_tmp_42314_11 = memory_address_to_id_state
                    .deduce_output(((mem0_base_col11) + ((offset1_col4) - (M31_32768))));
                let memory_id_to_big_value_tmp_42314_12 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_42314_11);
                let op0_id_col42 = memory_address_to_id_value_tmp_42314_11;
                *row[42] = op0_id_col42;
                *sub_component_inputs.memory_address_to_id[1] =
                    ((mem0_base_col11) + ((offset1_col4) - (M31_32768)));
                *lookup_data.memory_address_to_id_1 = [
                    ((mem0_base_col11) + ((offset1_col4) - (M31_32768))),
                    op0_id_col42,
                ];
                let op0_limb_0_col43 = memory_id_to_big_value_tmp_42314_12.get_m31(0);
                *row[43] = op0_limb_0_col43;
                let op0_limb_1_col44 = memory_id_to_big_value_tmp_42314_12.get_m31(1);
                *row[44] = op0_limb_1_col44;
                let op0_limb_2_col45 = memory_id_to_big_value_tmp_42314_12.get_m31(2);
                *row[45] = op0_limb_2_col45;
                let op0_limb_3_col46 = memory_id_to_big_value_tmp_42314_12.get_m31(3);
                *row[46] = op0_limb_3_col46;
                let op0_limb_4_col47 = memory_id_to_big_value_tmp_42314_12.get_m31(4);
                *row[47] = op0_limb_4_col47;
                let op0_limb_5_col48 = memory_id_to_big_value_tmp_42314_12.get_m31(5);
                *row[48] = op0_limb_5_col48;
                let op0_limb_6_col49 = memory_id_to_big_value_tmp_42314_12.get_m31(6);
                *row[49] = op0_limb_6_col49;
                let op0_limb_7_col50 = memory_id_to_big_value_tmp_42314_12.get_m31(7);
                *row[50] = op0_limb_7_col50;
                let op0_limb_8_col51 = memory_id_to_big_value_tmp_42314_12.get_m31(8);
                *row[51] = op0_limb_8_col51;
                let op0_limb_9_col52 = memory_id_to_big_value_tmp_42314_12.get_m31(9);
                *row[52] = op0_limb_9_col52;
                let op0_limb_10_col53 = memory_id_to_big_value_tmp_42314_12.get_m31(10);
                *row[53] = op0_limb_10_col53;
                let op0_limb_11_col54 = memory_id_to_big_value_tmp_42314_12.get_m31(11);
                *row[54] = op0_limb_11_col54;
                let op0_limb_12_col55 = memory_id_to_big_value_tmp_42314_12.get_m31(12);
                *row[55] = op0_limb_12_col55;
                let op0_limb_13_col56 = memory_id_to_big_value_tmp_42314_12.get_m31(13);
                *row[56] = op0_limb_13_col56;
                let op0_limb_14_col57 = memory_id_to_big_value_tmp_42314_12.get_m31(14);
                *row[57] = op0_limb_14_col57;
                let op0_limb_15_col58 = memory_id_to_big_value_tmp_42314_12.get_m31(15);
                *row[58] = op0_limb_15_col58;
                let op0_limb_16_col59 = memory_id_to_big_value_tmp_42314_12.get_m31(16);
                *row[59] = op0_limb_16_col59;
                let op0_limb_17_col60 = memory_id_to_big_value_tmp_42314_12.get_m31(17);
                *row[60] = op0_limb_17_col60;
                let op0_limb_18_col61 = memory_id_to_big_value_tmp_42314_12.get_m31(18);
                *row[61] = op0_limb_18_col61;
                let op0_limb_19_col62 = memory_id_to_big_value_tmp_42314_12.get_m31(19);
                *row[62] = op0_limb_19_col62;
                let op0_limb_20_col63 = memory_id_to_big_value_tmp_42314_12.get_m31(20);
                *row[63] = op0_limb_20_col63;
                let op0_limb_21_col64 = memory_id_to_big_value_tmp_42314_12.get_m31(21);
                *row[64] = op0_limb_21_col64;
                let op0_limb_22_col65 = memory_id_to_big_value_tmp_42314_12.get_m31(22);
                *row[65] = op0_limb_22_col65;
                let op0_limb_23_col66 = memory_id_to_big_value_tmp_42314_12.get_m31(23);
                *row[66] = op0_limb_23_col66;
                let op0_limb_24_col67 = memory_id_to_big_value_tmp_42314_12.get_m31(24);
                *row[67] = op0_limb_24_col67;
                let op0_limb_25_col68 = memory_id_to_big_value_tmp_42314_12.get_m31(25);
                *row[68] = op0_limb_25_col68;
                let op0_limb_26_col69 = memory_id_to_big_value_tmp_42314_12.get_m31(26);
                *row[69] = op0_limb_26_col69;
                let op0_limb_27_col70 = memory_id_to_big_value_tmp_42314_12.get_m31(27);
                *row[70] = op0_limb_27_col70;
                *sub_component_inputs.memory_id_to_big[1] = op0_id_col42;
                *lookup_data.memory_id_to_big_1 = [
                    op0_id_col42,
                    op0_limb_0_col43,
                    op0_limb_1_col44,
                    op0_limb_2_col45,
                    op0_limb_3_col46,
                    op0_limb_4_col47,
                    op0_limb_5_col48,
                    op0_limb_6_col49,
                    op0_limb_7_col50,
                    op0_limb_8_col51,
                    op0_limb_9_col52,
                    op0_limb_10_col53,
                    op0_limb_11_col54,
                    op0_limb_12_col55,
                    op0_limb_13_col56,
                    op0_limb_14_col57,
                    op0_limb_15_col58,
                    op0_limb_16_col59,
                    op0_limb_17_col60,
                    op0_limb_18_col61,
                    op0_limb_19_col62,
                    op0_limb_20_col63,
                    op0_limb_21_col64,
                    op0_limb_22_col65,
                    op0_limb_23_col66,
                    op0_limb_24_col67,
                    op0_limb_25_col68,
                    op0_limb_26_col69,
                    op0_limb_27_col70,
                ];

                // Read Positive Num Bits 252.

                let memory_address_to_id_value_tmp_42314_13 = memory_address_to_id_state
                    .deduce_output(((mem1_base_col12) + ((offset2_col5) - (M31_32768))));
                let memory_id_to_big_value_tmp_42314_14 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_42314_13);
                let op1_id_col71 = memory_address_to_id_value_tmp_42314_13;
                *row[71] = op1_id_col71;
                *sub_component_inputs.memory_address_to_id[2] =
                    ((mem1_base_col12) + ((offset2_col5) - (M31_32768)));
                *lookup_data.memory_address_to_id_2 = [
                    ((mem1_base_col12) + ((offset2_col5) - (M31_32768))),
                    op1_id_col71,
                ];
                let op1_limb_0_col72 = memory_id_to_big_value_tmp_42314_14.get_m31(0);
                *row[72] = op1_limb_0_col72;
                let op1_limb_1_col73 = memory_id_to_big_value_tmp_42314_14.get_m31(1);
                *row[73] = op1_limb_1_col73;
                let op1_limb_2_col74 = memory_id_to_big_value_tmp_42314_14.get_m31(2);
                *row[74] = op1_limb_2_col74;
                let op1_limb_3_col75 = memory_id_to_big_value_tmp_42314_14.get_m31(3);
                *row[75] = op1_limb_3_col75;
                let op1_limb_4_col76 = memory_id_to_big_value_tmp_42314_14.get_m31(4);
                *row[76] = op1_limb_4_col76;
                let op1_limb_5_col77 = memory_id_to_big_value_tmp_42314_14.get_m31(5);
                *row[77] = op1_limb_5_col77;
                let op1_limb_6_col78 = memory_id_to_big_value_tmp_42314_14.get_m31(6);
                *row[78] = op1_limb_6_col78;
                let op1_limb_7_col79 = memory_id_to_big_value_tmp_42314_14.get_m31(7);
                *row[79] = op1_limb_7_col79;
                let op1_limb_8_col80 = memory_id_to_big_value_tmp_42314_14.get_m31(8);
                *row[80] = op1_limb_8_col80;
                let op1_limb_9_col81 = memory_id_to_big_value_tmp_42314_14.get_m31(9);
                *row[81] = op1_limb_9_col81;
                let op1_limb_10_col82 = memory_id_to_big_value_tmp_42314_14.get_m31(10);
                *row[82] = op1_limb_10_col82;
                let op1_limb_11_col83 = memory_id_to_big_value_tmp_42314_14.get_m31(11);
                *row[83] = op1_limb_11_col83;
                let op1_limb_12_col84 = memory_id_to_big_value_tmp_42314_14.get_m31(12);
                *row[84] = op1_limb_12_col84;
                let op1_limb_13_col85 = memory_id_to_big_value_tmp_42314_14.get_m31(13);
                *row[85] = op1_limb_13_col85;
                let op1_limb_14_col86 = memory_id_to_big_value_tmp_42314_14.get_m31(14);
                *row[86] = op1_limb_14_col86;
                let op1_limb_15_col87 = memory_id_to_big_value_tmp_42314_14.get_m31(15);
                *row[87] = op1_limb_15_col87;
                let op1_limb_16_col88 = memory_id_to_big_value_tmp_42314_14.get_m31(16);
                *row[88] = op1_limb_16_col88;
                let op1_limb_17_col89 = memory_id_to_big_value_tmp_42314_14.get_m31(17);
                *row[89] = op1_limb_17_col89;
                let op1_limb_18_col90 = memory_id_to_big_value_tmp_42314_14.get_m31(18);
                *row[90] = op1_limb_18_col90;
                let op1_limb_19_col91 = memory_id_to_big_value_tmp_42314_14.get_m31(19);
                *row[91] = op1_limb_19_col91;
                let op1_limb_20_col92 = memory_id_to_big_value_tmp_42314_14.get_m31(20);
                *row[92] = op1_limb_20_col92;
                let op1_limb_21_col93 = memory_id_to_big_value_tmp_42314_14.get_m31(21);
                *row[93] = op1_limb_21_col93;
                let op1_limb_22_col94 = memory_id_to_big_value_tmp_42314_14.get_m31(22);
                *row[94] = op1_limb_22_col94;
                let op1_limb_23_col95 = memory_id_to_big_value_tmp_42314_14.get_m31(23);
                *row[95] = op1_limb_23_col95;
                let op1_limb_24_col96 = memory_id_to_big_value_tmp_42314_14.get_m31(24);
                *row[96] = op1_limb_24_col96;
                let op1_limb_25_col97 = memory_id_to_big_value_tmp_42314_14.get_m31(25);
                *row[97] = op1_limb_25_col97;
                let op1_limb_26_col98 = memory_id_to_big_value_tmp_42314_14.get_m31(26);
                *row[98] = op1_limb_26_col98;
                let op1_limb_27_col99 = memory_id_to_big_value_tmp_42314_14.get_m31(27);
                *row[99] = op1_limb_27_col99;
                *sub_component_inputs.memory_id_to_big[2] = op1_id_col71;
                *lookup_data.memory_id_to_big_2 = [
                    op1_id_col71,
                    op1_limb_0_col72,
                    op1_limb_1_col73,
                    op1_limb_2_col74,
                    op1_limb_3_col75,
                    op1_limb_4_col76,
                    op1_limb_5_col77,
                    op1_limb_6_col78,
                    op1_limb_7_col79,
                    op1_limb_8_col80,
                    op1_limb_9_col81,
                    op1_limb_10_col82,
                    op1_limb_11_col83,
                    op1_limb_12_col84,
                    op1_limb_13_col85,
                    op1_limb_14_col86,
                    op1_limb_15_col87,
                    op1_limb_16_col88,
                    op1_limb_17_col89,
                    op1_limb_18_col90,
                    op1_limb_19_col91,
                    op1_limb_20_col92,
                    op1_limb_21_col93,
                    op1_limb_22_col94,
                    op1_limb_23_col95,
                    op1_limb_24_col96,
                    op1_limb_25_col97,
                    op1_limb_26_col98,
                    op1_limb_27_col99,
                ];

                // Verify Mul 252.

                // Double Karatsuba N 7 Limb Max Bound 511.

                // Single Karatsuba N 7.

                let z0_tmp_42314_15 = [
                    ((op0_limb_0_col43) * (op1_limb_0_col72)),
                    (((op0_limb_0_col43) * (op1_limb_1_col73))
                        + ((op0_limb_1_col44) * (op1_limb_0_col72))),
                    ((((op0_limb_0_col43) * (op1_limb_2_col74))
                        + ((op0_limb_1_col44) * (op1_limb_1_col73)))
                        + ((op0_limb_2_col45) * (op1_limb_0_col72))),
                    (((((op0_limb_0_col43) * (op1_limb_3_col75))
                        + ((op0_limb_1_col44) * (op1_limb_2_col74)))
                        + ((op0_limb_2_col45) * (op1_limb_1_col73)))
                        + ((op0_limb_3_col46) * (op1_limb_0_col72))),
                    ((((((op0_limb_0_col43) * (op1_limb_4_col76))
                        + ((op0_limb_1_col44) * (op1_limb_3_col75)))
                        + ((op0_limb_2_col45) * (op1_limb_2_col74)))
                        + ((op0_limb_3_col46) * (op1_limb_1_col73)))
                        + ((op0_limb_4_col47) * (op1_limb_0_col72))),
                    (((((((op0_limb_0_col43) * (op1_limb_5_col77))
                        + ((op0_limb_1_col44) * (op1_limb_4_col76)))
                        + ((op0_limb_2_col45) * (op1_limb_3_col75)))
                        + ((op0_limb_3_col46) * (op1_limb_2_col74)))
                        + ((op0_limb_4_col47) * (op1_limb_1_col73)))
                        + ((op0_limb_5_col48) * (op1_limb_0_col72))),
                    ((((((((op0_limb_0_col43) * (op1_limb_6_col78))
                        + ((op0_limb_1_col44) * (op1_limb_5_col77)))
                        + ((op0_limb_2_col45) * (op1_limb_4_col76)))
                        + ((op0_limb_3_col46) * (op1_limb_3_col75)))
                        + ((op0_limb_4_col47) * (op1_limb_2_col74)))
                        + ((op0_limb_5_col48) * (op1_limb_1_col73)))
                        + ((op0_limb_6_col49) * (op1_limb_0_col72))),
                    (((((((op0_limb_1_col44) * (op1_limb_6_col78))
                        + ((op0_limb_2_col45) * (op1_limb_5_col77)))
                        + ((op0_limb_3_col46) * (op1_limb_4_col76)))
                        + ((op0_limb_4_col47) * (op1_limb_3_col75)))
                        + ((op0_limb_5_col48) * (op1_limb_2_col74)))
                        + ((op0_limb_6_col49) * (op1_limb_1_col73))),
                    ((((((op0_limb_2_col45) * (op1_limb_6_col78))
                        + ((op0_limb_3_col46) * (op1_limb_5_col77)))
                        + ((op0_limb_4_col47) * (op1_limb_4_col76)))
                        + ((op0_limb_5_col48) * (op1_limb_3_col75)))
                        + ((op0_limb_6_col49) * (op1_limb_2_col74))),
                    (((((op0_limb_3_col46) * (op1_limb_6_col78))
                        + ((op0_limb_4_col47) * (op1_limb_5_col77)))
                        + ((op0_limb_5_col48) * (op1_limb_4_col76)))
                        + ((op0_limb_6_col49) * (op1_limb_3_col75))),
                    ((((op0_limb_4_col47) * (op1_limb_6_col78))
                        + ((op0_limb_5_col48) * (op1_limb_5_col77)))
                        + ((op0_limb_6_col49) * (op1_limb_4_col76))),
                    (((op0_limb_5_col48) * (op1_limb_6_col78))
                        + ((op0_limb_6_col49) * (op1_limb_5_col77))),
                    ((op0_limb_6_col49) * (op1_limb_6_col78)),
                ];
                let z2_tmp_42314_16 = [
                    ((op0_limb_7_col50) * (op1_limb_7_col79)),
                    (((op0_limb_7_col50) * (op1_limb_8_col80))
                        + ((op0_limb_8_col51) * (op1_limb_7_col79))),
                    ((((op0_limb_7_col50) * (op1_limb_9_col81))
                        + ((op0_limb_8_col51) * (op1_limb_8_col80)))
                        + ((op0_limb_9_col52) * (op1_limb_7_col79))),
                    (((((op0_limb_7_col50) * (op1_limb_10_col82))
                        + ((op0_limb_8_col51) * (op1_limb_9_col81)))
                        + ((op0_limb_9_col52) * (op1_limb_8_col80)))
                        + ((op0_limb_10_col53) * (op1_limb_7_col79))),
                    ((((((op0_limb_7_col50) * (op1_limb_11_col83))
                        + ((op0_limb_8_col51) * (op1_limb_10_col82)))
                        + ((op0_limb_9_col52) * (op1_limb_9_col81)))
                        + ((op0_limb_10_col53) * (op1_limb_8_col80)))
                        + ((op0_limb_11_col54) * (op1_limb_7_col79))),
                    (((((((op0_limb_7_col50) * (op1_limb_12_col84))
                        + ((op0_limb_8_col51) * (op1_limb_11_col83)))
                        + ((op0_limb_9_col52) * (op1_limb_10_col82)))
                        + ((op0_limb_10_col53) * (op1_limb_9_col81)))
                        + ((op0_limb_11_col54) * (op1_limb_8_col80)))
                        + ((op0_limb_12_col55) * (op1_limb_7_col79))),
                    ((((((((op0_limb_7_col50) * (op1_limb_13_col85))
                        + ((op0_limb_8_col51) * (op1_limb_12_col84)))
                        + ((op0_limb_9_col52) * (op1_limb_11_col83)))
                        + ((op0_limb_10_col53) * (op1_limb_10_col82)))
                        + ((op0_limb_11_col54) * (op1_limb_9_col81)))
                        + ((op0_limb_12_col55) * (op1_limb_8_col80)))
                        + ((op0_limb_13_col56) * (op1_limb_7_col79))),
                    (((((((op0_limb_8_col51) * (op1_limb_13_col85))
                        + ((op0_limb_9_col52) * (op1_limb_12_col84)))
                        + ((op0_limb_10_col53) * (op1_limb_11_col83)))
                        + ((op0_limb_11_col54) * (op1_limb_10_col82)))
                        + ((op0_limb_12_col55) * (op1_limb_9_col81)))
                        + ((op0_limb_13_col56) * (op1_limb_8_col80))),
                    ((((((op0_limb_9_col52) * (op1_limb_13_col85))
                        + ((op0_limb_10_col53) * (op1_limb_12_col84)))
                        + ((op0_limb_11_col54) * (op1_limb_11_col83)))
                        + ((op0_limb_12_col55) * (op1_limb_10_col82)))
                        + ((op0_limb_13_col56) * (op1_limb_9_col81))),
                    (((((op0_limb_10_col53) * (op1_limb_13_col85))
                        + ((op0_limb_11_col54) * (op1_limb_12_col84)))
                        + ((op0_limb_12_col55) * (op1_limb_11_col83)))
                        + ((op0_limb_13_col56) * (op1_limb_10_col82))),
                    ((((op0_limb_11_col54) * (op1_limb_13_col85))
                        + ((op0_limb_12_col55) * (op1_limb_12_col84)))
                        + ((op0_limb_13_col56) * (op1_limb_11_col83))),
                    (((op0_limb_12_col55) * (op1_limb_13_col85))
                        + ((op0_limb_13_col56) * (op1_limb_12_col84))),
                    ((op0_limb_13_col56) * (op1_limb_13_col85)),
                ];
                let x_sum_tmp_42314_17 = [
                    ((op0_limb_0_col43) + (op0_limb_7_col50)),
                    ((op0_limb_1_col44) + (op0_limb_8_col51)),
                    ((op0_limb_2_col45) + (op0_limb_9_col52)),
                    ((op0_limb_3_col46) + (op0_limb_10_col53)),
                    ((op0_limb_4_col47) + (op0_limb_11_col54)),
                    ((op0_limb_5_col48) + (op0_limb_12_col55)),
                    ((op0_limb_6_col49) + (op0_limb_13_col56)),
                ];
                let y_sum_tmp_42314_18 = [
                    ((op1_limb_0_col72) + (op1_limb_7_col79)),
                    ((op1_limb_1_col73) + (op1_limb_8_col80)),
                    ((op1_limb_2_col74) + (op1_limb_9_col81)),
                    ((op1_limb_3_col75) + (op1_limb_10_col82)),
                    ((op1_limb_4_col76) + (op1_limb_11_col83)),
                    ((op1_limb_5_col77) + (op1_limb_12_col84)),
                    ((op1_limb_6_col78) + (op1_limb_13_col85)),
                ];

                // Single Karatsuba N 7.

                let z0_tmp_42314_19 = [
                    ((op0_limb_14_col57) * (op1_limb_14_col86)),
                    (((op0_limb_14_col57) * (op1_limb_15_col87))
                        + ((op0_limb_15_col58) * (op1_limb_14_col86))),
                    ((((op0_limb_14_col57) * (op1_limb_16_col88))
                        + ((op0_limb_15_col58) * (op1_limb_15_col87)))
                        + ((op0_limb_16_col59) * (op1_limb_14_col86))),
                    (((((op0_limb_14_col57) * (op1_limb_17_col89))
                        + ((op0_limb_15_col58) * (op1_limb_16_col88)))
                        + ((op0_limb_16_col59) * (op1_limb_15_col87)))
                        + ((op0_limb_17_col60) * (op1_limb_14_col86))),
                    ((((((op0_limb_14_col57) * (op1_limb_18_col90))
                        + ((op0_limb_15_col58) * (op1_limb_17_col89)))
                        + ((op0_limb_16_col59) * (op1_limb_16_col88)))
                        + ((op0_limb_17_col60) * (op1_limb_15_col87)))
                        + ((op0_limb_18_col61) * (op1_limb_14_col86))),
                    (((((((op0_limb_14_col57) * (op1_limb_19_col91))
                        + ((op0_limb_15_col58) * (op1_limb_18_col90)))
                        + ((op0_limb_16_col59) * (op1_limb_17_col89)))
                        + ((op0_limb_17_col60) * (op1_limb_16_col88)))
                        + ((op0_limb_18_col61) * (op1_limb_15_col87)))
                        + ((op0_limb_19_col62) * (op1_limb_14_col86))),
                    ((((((((op0_limb_14_col57) * (op1_limb_20_col92))
                        + ((op0_limb_15_col58) * (op1_limb_19_col91)))
                        + ((op0_limb_16_col59) * (op1_limb_18_col90)))
                        + ((op0_limb_17_col60) * (op1_limb_17_col89)))
                        + ((op0_limb_18_col61) * (op1_limb_16_col88)))
                        + ((op0_limb_19_col62) * (op1_limb_15_col87)))
                        + ((op0_limb_20_col63) * (op1_limb_14_col86))),
                    (((((((op0_limb_15_col58) * (op1_limb_20_col92))
                        + ((op0_limb_16_col59) * (op1_limb_19_col91)))
                        + ((op0_limb_17_col60) * (op1_limb_18_col90)))
                        + ((op0_limb_18_col61) * (op1_limb_17_col89)))
                        + ((op0_limb_19_col62) * (op1_limb_16_col88)))
                        + ((op0_limb_20_col63) * (op1_limb_15_col87))),
                    ((((((op0_limb_16_col59) * (op1_limb_20_col92))
                        + ((op0_limb_17_col60) * (op1_limb_19_col91)))
                        + ((op0_limb_18_col61) * (op1_limb_18_col90)))
                        + ((op0_limb_19_col62) * (op1_limb_17_col89)))
                        + ((op0_limb_20_col63) * (op1_limb_16_col88))),
                    (((((op0_limb_17_col60) * (op1_limb_20_col92))
                        + ((op0_limb_18_col61) * (op1_limb_19_col91)))
                        + ((op0_limb_19_col62) * (op1_limb_18_col90)))
                        + ((op0_limb_20_col63) * (op1_limb_17_col89))),
                    ((((op0_limb_18_col61) * (op1_limb_20_col92))
                        + ((op0_limb_19_col62) * (op1_limb_19_col91)))
                        + ((op0_limb_20_col63) * (op1_limb_18_col90))),
                    (((op0_limb_19_col62) * (op1_limb_20_col92))
                        + ((op0_limb_20_col63) * (op1_limb_19_col91))),
                    ((op0_limb_20_col63) * (op1_limb_20_col92)),
                ];
                let z2_tmp_42314_20 = [
                    ((op0_limb_21_col64) * (op1_limb_21_col93)),
                    (((op0_limb_21_col64) * (op1_limb_22_col94))
                        + ((op0_limb_22_col65) * (op1_limb_21_col93))),
                    ((((op0_limb_21_col64) * (op1_limb_23_col95))
                        + ((op0_limb_22_col65) * (op1_limb_22_col94)))
                        + ((op0_limb_23_col66) * (op1_limb_21_col93))),
                    (((((op0_limb_21_col64) * (op1_limb_24_col96))
                        + ((op0_limb_22_col65) * (op1_limb_23_col95)))
                        + ((op0_limb_23_col66) * (op1_limb_22_col94)))
                        + ((op0_limb_24_col67) * (op1_limb_21_col93))),
                    ((((((op0_limb_21_col64) * (op1_limb_25_col97))
                        + ((op0_limb_22_col65) * (op1_limb_24_col96)))
                        + ((op0_limb_23_col66) * (op1_limb_23_col95)))
                        + ((op0_limb_24_col67) * (op1_limb_22_col94)))
                        + ((op0_limb_25_col68) * (op1_limb_21_col93))),
                    (((((((op0_limb_21_col64) * (op1_limb_26_col98))
                        + ((op0_limb_22_col65) * (op1_limb_25_col97)))
                        + ((op0_limb_23_col66) * (op1_limb_24_col96)))
                        + ((op0_limb_24_col67) * (op1_limb_23_col95)))
                        + ((op0_limb_25_col68) * (op1_limb_22_col94)))
                        + ((op0_limb_26_col69) * (op1_limb_21_col93))),
                    ((((((((op0_limb_21_col64) * (op1_limb_27_col99))
                        + ((op0_limb_22_col65) * (op1_limb_26_col98)))
                        + ((op0_limb_23_col66) * (op1_limb_25_col97)))
                        + ((op0_limb_24_col67) * (op1_limb_24_col96)))
                        + ((op0_limb_25_col68) * (op1_limb_23_col95)))
                        + ((op0_limb_26_col69) * (op1_limb_22_col94)))
                        + ((op0_limb_27_col70) * (op1_limb_21_col93))),
                    (((((((op0_limb_22_col65) * (op1_limb_27_col99))
                        + ((op0_limb_23_col66) * (op1_limb_26_col98)))
                        + ((op0_limb_24_col67) * (op1_limb_25_col97)))
                        + ((op0_limb_25_col68) * (op1_limb_24_col96)))
                        + ((op0_limb_26_col69) * (op1_limb_23_col95)))
                        + ((op0_limb_27_col70) * (op1_limb_22_col94))),
                    ((((((op0_limb_23_col66) * (op1_limb_27_col99))
                        + ((op0_limb_24_col67) * (op1_limb_26_col98)))
                        + ((op0_limb_25_col68) * (op1_limb_25_col97)))
                        + ((op0_limb_26_col69) * (op1_limb_24_col96)))
                        + ((op0_limb_27_col70) * (op1_limb_23_col95))),
                    (((((op0_limb_24_col67) * (op1_limb_27_col99))
                        + ((op0_limb_25_col68) * (op1_limb_26_col98)))
                        + ((op0_limb_26_col69) * (op1_limb_25_col97)))
                        + ((op0_limb_27_col70) * (op1_limb_24_col96))),
                    ((((op0_limb_25_col68) * (op1_limb_27_col99))
                        + ((op0_limb_26_col69) * (op1_limb_26_col98)))
                        + ((op0_limb_27_col70) * (op1_limb_25_col97))),
                    (((op0_limb_26_col69) * (op1_limb_27_col99))
                        + ((op0_limb_27_col70) * (op1_limb_26_col98))),
                    ((op0_limb_27_col70) * (op1_limb_27_col99)),
                ];
                let x_sum_tmp_42314_21 = [
                    ((op0_limb_14_col57) + (op0_limb_21_col64)),
                    ((op0_limb_15_col58) + (op0_limb_22_col65)),
                    ((op0_limb_16_col59) + (op0_limb_23_col66)),
                    ((op0_limb_17_col60) + (op0_limb_24_col67)),
                    ((op0_limb_18_col61) + (op0_limb_25_col68)),
                    ((op0_limb_19_col62) + (op0_limb_26_col69)),
                    ((op0_limb_20_col63) + (op0_limb_27_col70)),
                ];
                let y_sum_tmp_42314_22 = [
                    ((op1_limb_14_col86) + (op1_limb_21_col93)),
                    ((op1_limb_15_col87) + (op1_limb_22_col94)),
                    ((op1_limb_16_col88) + (op1_limb_23_col95)),
                    ((op1_limb_17_col89) + (op1_limb_24_col96)),
                    ((op1_limb_18_col90) + (op1_limb_25_col97)),
                    ((op1_limb_19_col91) + (op1_limb_26_col98)),
                    ((op1_limb_20_col92) + (op1_limb_27_col99)),
                ];

                let z0_tmp_42314_23 = [
                    z0_tmp_42314_15[0],
                    z0_tmp_42314_15[1],
                    z0_tmp_42314_15[2],
                    z0_tmp_42314_15[3],
                    z0_tmp_42314_15[4],
                    z0_tmp_42314_15[5],
                    z0_tmp_42314_15[6],
                    ((z0_tmp_42314_15[7])
                        + ((((x_sum_tmp_42314_17[0]) * (y_sum_tmp_42314_18[0]))
                            - (z0_tmp_42314_15[0]))
                            - (z2_tmp_42314_16[0]))),
                    ((z0_tmp_42314_15[8])
                        + (((((x_sum_tmp_42314_17[0]) * (y_sum_tmp_42314_18[1]))
                            + ((x_sum_tmp_42314_17[1]) * (y_sum_tmp_42314_18[0])))
                            - (z0_tmp_42314_15[1]))
                            - (z2_tmp_42314_16[1]))),
                    ((z0_tmp_42314_15[9])
                        + ((((((x_sum_tmp_42314_17[0]) * (y_sum_tmp_42314_18[2]))
                            + ((x_sum_tmp_42314_17[1]) * (y_sum_tmp_42314_18[1])))
                            + ((x_sum_tmp_42314_17[2]) * (y_sum_tmp_42314_18[0])))
                            - (z0_tmp_42314_15[2]))
                            - (z2_tmp_42314_16[2]))),
                    ((z0_tmp_42314_15[10])
                        + (((((((x_sum_tmp_42314_17[0]) * (y_sum_tmp_42314_18[3]))
                            + ((x_sum_tmp_42314_17[1]) * (y_sum_tmp_42314_18[2])))
                            + ((x_sum_tmp_42314_17[2]) * (y_sum_tmp_42314_18[1])))
                            + ((x_sum_tmp_42314_17[3]) * (y_sum_tmp_42314_18[0])))
                            - (z0_tmp_42314_15[3]))
                            - (z2_tmp_42314_16[3]))),
                    ((z0_tmp_42314_15[11])
                        + ((((((((x_sum_tmp_42314_17[0]) * (y_sum_tmp_42314_18[4]))
                            + ((x_sum_tmp_42314_17[1]) * (y_sum_tmp_42314_18[3])))
                            + ((x_sum_tmp_42314_17[2]) * (y_sum_tmp_42314_18[2])))
                            + ((x_sum_tmp_42314_17[3]) * (y_sum_tmp_42314_18[1])))
                            + ((x_sum_tmp_42314_17[4]) * (y_sum_tmp_42314_18[0])))
                            - (z0_tmp_42314_15[4]))
                            - (z2_tmp_42314_16[4]))),
                    ((z0_tmp_42314_15[12])
                        + (((((((((x_sum_tmp_42314_17[0]) * (y_sum_tmp_42314_18[5]))
                            + ((x_sum_tmp_42314_17[1]) * (y_sum_tmp_42314_18[4])))
                            + ((x_sum_tmp_42314_17[2]) * (y_sum_tmp_42314_18[3])))
                            + ((x_sum_tmp_42314_17[3]) * (y_sum_tmp_42314_18[2])))
                            + ((x_sum_tmp_42314_17[4]) * (y_sum_tmp_42314_18[1])))
                            + ((x_sum_tmp_42314_17[5]) * (y_sum_tmp_42314_18[0])))
                            - (z0_tmp_42314_15[5]))
                            - (z2_tmp_42314_16[5]))),
                    ((((((((((x_sum_tmp_42314_17[0]) * (y_sum_tmp_42314_18[6]))
                        + ((x_sum_tmp_42314_17[1]) * (y_sum_tmp_42314_18[5])))
                        + ((x_sum_tmp_42314_17[2]) * (y_sum_tmp_42314_18[4])))
                        + ((x_sum_tmp_42314_17[3]) * (y_sum_tmp_42314_18[3])))
                        + ((x_sum_tmp_42314_17[4]) * (y_sum_tmp_42314_18[2])))
                        + ((x_sum_tmp_42314_17[5]) * (y_sum_tmp_42314_18[1])))
                        + ((x_sum_tmp_42314_17[6]) * (y_sum_tmp_42314_18[0])))
                        - (z0_tmp_42314_15[6]))
                        - (z2_tmp_42314_16[6])),
                    ((z2_tmp_42314_16[0])
                        + (((((((((x_sum_tmp_42314_17[1]) * (y_sum_tmp_42314_18[6]))
                            + ((x_sum_tmp_42314_17[2]) * (y_sum_tmp_42314_18[5])))
                            + ((x_sum_tmp_42314_17[3]) * (y_sum_tmp_42314_18[4])))
                            + ((x_sum_tmp_42314_17[4]) * (y_sum_tmp_42314_18[3])))
                            + ((x_sum_tmp_42314_17[5]) * (y_sum_tmp_42314_18[2])))
                            + ((x_sum_tmp_42314_17[6]) * (y_sum_tmp_42314_18[1])))
                            - (z0_tmp_42314_15[7]))
                            - (z2_tmp_42314_16[7]))),
                    ((z2_tmp_42314_16[1])
                        + ((((((((x_sum_tmp_42314_17[2]) * (y_sum_tmp_42314_18[6]))
                            + ((x_sum_tmp_42314_17[3]) * (y_sum_tmp_42314_18[5])))
                            + ((x_sum_tmp_42314_17[4]) * (y_sum_tmp_42314_18[4])))
                            + ((x_sum_tmp_42314_17[5]) * (y_sum_tmp_42314_18[3])))
                            + ((x_sum_tmp_42314_17[6]) * (y_sum_tmp_42314_18[2])))
                            - (z0_tmp_42314_15[8]))
                            - (z2_tmp_42314_16[8]))),
                    ((z2_tmp_42314_16[2])
                        + (((((((x_sum_tmp_42314_17[3]) * (y_sum_tmp_42314_18[6]))
                            + ((x_sum_tmp_42314_17[4]) * (y_sum_tmp_42314_18[5])))
                            + ((x_sum_tmp_42314_17[5]) * (y_sum_tmp_42314_18[4])))
                            + ((x_sum_tmp_42314_17[6]) * (y_sum_tmp_42314_18[3])))
                            - (z0_tmp_42314_15[9]))
                            - (z2_tmp_42314_16[9]))),
                    ((z2_tmp_42314_16[3])
                        + ((((((x_sum_tmp_42314_17[4]) * (y_sum_tmp_42314_18[6]))
                            + ((x_sum_tmp_42314_17[5]) * (y_sum_tmp_42314_18[5])))
                            + ((x_sum_tmp_42314_17[6]) * (y_sum_tmp_42314_18[4])))
                            - (z0_tmp_42314_15[10]))
                            - (z2_tmp_42314_16[10]))),
                    ((z2_tmp_42314_16[4])
                        + (((((x_sum_tmp_42314_17[5]) * (y_sum_tmp_42314_18[6]))
                            + ((x_sum_tmp_42314_17[6]) * (y_sum_tmp_42314_18[5])))
                            - (z0_tmp_42314_15[11]))
                            - (z2_tmp_42314_16[11]))),
                    ((z2_tmp_42314_16[5])
                        + ((((x_sum_tmp_42314_17[6]) * (y_sum_tmp_42314_18[6]))
                            - (z0_tmp_42314_15[12]))
                            - (z2_tmp_42314_16[12]))),
                    z2_tmp_42314_16[6],
                    z2_tmp_42314_16[7],
                    z2_tmp_42314_16[8],
                    z2_tmp_42314_16[9],
                    z2_tmp_42314_16[10],
                    z2_tmp_42314_16[11],
                    z2_tmp_42314_16[12],
                ];
                let z2_tmp_42314_24 = [
                    z0_tmp_42314_19[0],
                    z0_tmp_42314_19[1],
                    z0_tmp_42314_19[2],
                    z0_tmp_42314_19[3],
                    z0_tmp_42314_19[4],
                    z0_tmp_42314_19[5],
                    z0_tmp_42314_19[6],
                    ((z0_tmp_42314_19[7])
                        + ((((x_sum_tmp_42314_21[0]) * (y_sum_tmp_42314_22[0]))
                            - (z0_tmp_42314_19[0]))
                            - (z2_tmp_42314_20[0]))),
                    ((z0_tmp_42314_19[8])
                        + (((((x_sum_tmp_42314_21[0]) * (y_sum_tmp_42314_22[1]))
                            + ((x_sum_tmp_42314_21[1]) * (y_sum_tmp_42314_22[0])))
                            - (z0_tmp_42314_19[1]))
                            - (z2_tmp_42314_20[1]))),
                    ((z0_tmp_42314_19[9])
                        + ((((((x_sum_tmp_42314_21[0]) * (y_sum_tmp_42314_22[2]))
                            + ((x_sum_tmp_42314_21[1]) * (y_sum_tmp_42314_22[1])))
                            + ((x_sum_tmp_42314_21[2]) * (y_sum_tmp_42314_22[0])))
                            - (z0_tmp_42314_19[2]))
                            - (z2_tmp_42314_20[2]))),
                    ((z0_tmp_42314_19[10])
                        + (((((((x_sum_tmp_42314_21[0]) * (y_sum_tmp_42314_22[3]))
                            + ((x_sum_tmp_42314_21[1]) * (y_sum_tmp_42314_22[2])))
                            + ((x_sum_tmp_42314_21[2]) * (y_sum_tmp_42314_22[1])))
                            + ((x_sum_tmp_42314_21[3]) * (y_sum_tmp_42314_22[0])))
                            - (z0_tmp_42314_19[3]))
                            - (z2_tmp_42314_20[3]))),
                    ((z0_tmp_42314_19[11])
                        + ((((((((x_sum_tmp_42314_21[0]) * (y_sum_tmp_42314_22[4]))
                            + ((x_sum_tmp_42314_21[1]) * (y_sum_tmp_42314_22[3])))
                            + ((x_sum_tmp_42314_21[2]) * (y_sum_tmp_42314_22[2])))
                            + ((x_sum_tmp_42314_21[3]) * (y_sum_tmp_42314_22[1])))
                            + ((x_sum_tmp_42314_21[4]) * (y_sum_tmp_42314_22[0])))
                            - (z0_tmp_42314_19[4]))
                            - (z2_tmp_42314_20[4]))),
                    ((z0_tmp_42314_19[12])
                        + (((((((((x_sum_tmp_42314_21[0]) * (y_sum_tmp_42314_22[5]))
                            + ((x_sum_tmp_42314_21[1]) * (y_sum_tmp_42314_22[4])))
                            + ((x_sum_tmp_42314_21[2]) * (y_sum_tmp_42314_22[3])))
                            + ((x_sum_tmp_42314_21[3]) * (y_sum_tmp_42314_22[2])))
                            + ((x_sum_tmp_42314_21[4]) * (y_sum_tmp_42314_22[1])))
                            + ((x_sum_tmp_42314_21[5]) * (y_sum_tmp_42314_22[0])))
                            - (z0_tmp_42314_19[5]))
                            - (z2_tmp_42314_20[5]))),
                    ((((((((((x_sum_tmp_42314_21[0]) * (y_sum_tmp_42314_22[6]))
                        + ((x_sum_tmp_42314_21[1]) * (y_sum_tmp_42314_22[5])))
                        + ((x_sum_tmp_42314_21[2]) * (y_sum_tmp_42314_22[4])))
                        + ((x_sum_tmp_42314_21[3]) * (y_sum_tmp_42314_22[3])))
                        + ((x_sum_tmp_42314_21[4]) * (y_sum_tmp_42314_22[2])))
                        + ((x_sum_tmp_42314_21[5]) * (y_sum_tmp_42314_22[1])))
                        + ((x_sum_tmp_42314_21[6]) * (y_sum_tmp_42314_22[0])))
                        - (z0_tmp_42314_19[6]))
                        - (z2_tmp_42314_20[6])),
                    ((z2_tmp_42314_20[0])
                        + (((((((((x_sum_tmp_42314_21[1]) * (y_sum_tmp_42314_22[6]))
                            + ((x_sum_tmp_42314_21[2]) * (y_sum_tmp_42314_22[5])))
                            + ((x_sum_tmp_42314_21[3]) * (y_sum_tmp_42314_22[4])))
                            + ((x_sum_tmp_42314_21[4]) * (y_sum_tmp_42314_22[3])))
                            + ((x_sum_tmp_42314_21[5]) * (y_sum_tmp_42314_22[2])))
                            + ((x_sum_tmp_42314_21[6]) * (y_sum_tmp_42314_22[1])))
                            - (z0_tmp_42314_19[7]))
                            - (z2_tmp_42314_20[7]))),
                    ((z2_tmp_42314_20[1])
                        + ((((((((x_sum_tmp_42314_21[2]) * (y_sum_tmp_42314_22[6]))
                            + ((x_sum_tmp_42314_21[3]) * (y_sum_tmp_42314_22[5])))
                            + ((x_sum_tmp_42314_21[4]) * (y_sum_tmp_42314_22[4])))
                            + ((x_sum_tmp_42314_21[5]) * (y_sum_tmp_42314_22[3])))
                            + ((x_sum_tmp_42314_21[6]) * (y_sum_tmp_42314_22[2])))
                            - (z0_tmp_42314_19[8]))
                            - (z2_tmp_42314_20[8]))),
                    ((z2_tmp_42314_20[2])
                        + (((((((x_sum_tmp_42314_21[3]) * (y_sum_tmp_42314_22[6]))
                            + ((x_sum_tmp_42314_21[4]) * (y_sum_tmp_42314_22[5])))
                            + ((x_sum_tmp_42314_21[5]) * (y_sum_tmp_42314_22[4])))
                            + ((x_sum_tmp_42314_21[6]) * (y_sum_tmp_42314_22[3])))
                            - (z0_tmp_42314_19[9]))
                            - (z2_tmp_42314_20[9]))),
                    ((z2_tmp_42314_20[3])
                        + ((((((x_sum_tmp_42314_21[4]) * (y_sum_tmp_42314_22[6]))
                            + ((x_sum_tmp_42314_21[5]) * (y_sum_tmp_42314_22[5])))
                            + ((x_sum_tmp_42314_21[6]) * (y_sum_tmp_42314_22[4])))
                            - (z0_tmp_42314_19[10]))
                            - (z2_tmp_42314_20[10]))),
                    ((z2_tmp_42314_20[4])
                        + (((((x_sum_tmp_42314_21[5]) * (y_sum_tmp_42314_22[6]))
                            + ((x_sum_tmp_42314_21[6]) * (y_sum_tmp_42314_22[5])))
                            - (z0_tmp_42314_19[11]))
                            - (z2_tmp_42314_20[11]))),
                    ((z2_tmp_42314_20[5])
                        + ((((x_sum_tmp_42314_21[6]) * (y_sum_tmp_42314_22[6]))
                            - (z0_tmp_42314_19[12]))
                            - (z2_tmp_42314_20[12]))),
                    z2_tmp_42314_20[6],
                    z2_tmp_42314_20[7],
                    z2_tmp_42314_20[8],
                    z2_tmp_42314_20[9],
                    z2_tmp_42314_20[10],
                    z2_tmp_42314_20[11],
                    z2_tmp_42314_20[12],
                ];
                let x_sum_tmp_42314_25 = [
                    ((op0_limb_0_col43) + (op0_limb_14_col57)),
                    ((op0_limb_1_col44) + (op0_limb_15_col58)),
                    ((op0_limb_2_col45) + (op0_limb_16_col59)),
                    ((op0_limb_3_col46) + (op0_limb_17_col60)),
                    ((op0_limb_4_col47) + (op0_limb_18_col61)),
                    ((op0_limb_5_col48) + (op0_limb_19_col62)),
                    ((op0_limb_6_col49) + (op0_limb_20_col63)),
                    ((op0_limb_7_col50) + (op0_limb_21_col64)),
                    ((op0_limb_8_col51) + (op0_limb_22_col65)),
                    ((op0_limb_9_col52) + (op0_limb_23_col66)),
                    ((op0_limb_10_col53) + (op0_limb_24_col67)),
                    ((op0_limb_11_col54) + (op0_limb_25_col68)),
                    ((op0_limb_12_col55) + (op0_limb_26_col69)),
                    ((op0_limb_13_col56) + (op0_limb_27_col70)),
                ];
                let y_sum_tmp_42314_26 = [
                    ((op1_limb_0_col72) + (op1_limb_14_col86)),
                    ((op1_limb_1_col73) + (op1_limb_15_col87)),
                    ((op1_limb_2_col74) + (op1_limb_16_col88)),
                    ((op1_limb_3_col75) + (op1_limb_17_col89)),
                    ((op1_limb_4_col76) + (op1_limb_18_col90)),
                    ((op1_limb_5_col77) + (op1_limb_19_col91)),
                    ((op1_limb_6_col78) + (op1_limb_20_col92)),
                    ((op1_limb_7_col79) + (op1_limb_21_col93)),
                    ((op1_limb_8_col80) + (op1_limb_22_col94)),
                    ((op1_limb_9_col81) + (op1_limb_23_col95)),
                    ((op1_limb_10_col82) + (op1_limb_24_col96)),
                    ((op1_limb_11_col83) + (op1_limb_25_col97)),
                    ((op1_limb_12_col84) + (op1_limb_26_col98)),
                    ((op1_limb_13_col85) + (op1_limb_27_col99)),
                ];

                // Single Karatsuba N 7.

                let z0_tmp_42314_27 = [
                    ((x_sum_tmp_42314_25[0]) * (y_sum_tmp_42314_26[0])),
                    (((x_sum_tmp_42314_25[0]) * (y_sum_tmp_42314_26[1]))
                        + ((x_sum_tmp_42314_25[1]) * (y_sum_tmp_42314_26[0]))),
                    ((((x_sum_tmp_42314_25[0]) * (y_sum_tmp_42314_26[2]))
                        + ((x_sum_tmp_42314_25[1]) * (y_sum_tmp_42314_26[1])))
                        + ((x_sum_tmp_42314_25[2]) * (y_sum_tmp_42314_26[0]))),
                    (((((x_sum_tmp_42314_25[0]) * (y_sum_tmp_42314_26[3]))
                        + ((x_sum_tmp_42314_25[1]) * (y_sum_tmp_42314_26[2])))
                        + ((x_sum_tmp_42314_25[2]) * (y_sum_tmp_42314_26[1])))
                        + ((x_sum_tmp_42314_25[3]) * (y_sum_tmp_42314_26[0]))),
                    ((((((x_sum_tmp_42314_25[0]) * (y_sum_tmp_42314_26[4]))
                        + ((x_sum_tmp_42314_25[1]) * (y_sum_tmp_42314_26[3])))
                        + ((x_sum_tmp_42314_25[2]) * (y_sum_tmp_42314_26[2])))
                        + ((x_sum_tmp_42314_25[3]) * (y_sum_tmp_42314_26[1])))
                        + ((x_sum_tmp_42314_25[4]) * (y_sum_tmp_42314_26[0]))),
                    (((((((x_sum_tmp_42314_25[0]) * (y_sum_tmp_42314_26[5]))
                        + ((x_sum_tmp_42314_25[1]) * (y_sum_tmp_42314_26[4])))
                        + ((x_sum_tmp_42314_25[2]) * (y_sum_tmp_42314_26[3])))
                        + ((x_sum_tmp_42314_25[3]) * (y_sum_tmp_42314_26[2])))
                        + ((x_sum_tmp_42314_25[4]) * (y_sum_tmp_42314_26[1])))
                        + ((x_sum_tmp_42314_25[5]) * (y_sum_tmp_42314_26[0]))),
                    ((((((((x_sum_tmp_42314_25[0]) * (y_sum_tmp_42314_26[6]))
                        + ((x_sum_tmp_42314_25[1]) * (y_sum_tmp_42314_26[5])))
                        + ((x_sum_tmp_42314_25[2]) * (y_sum_tmp_42314_26[4])))
                        + ((x_sum_tmp_42314_25[3]) * (y_sum_tmp_42314_26[3])))
                        + ((x_sum_tmp_42314_25[4]) * (y_sum_tmp_42314_26[2])))
                        + ((x_sum_tmp_42314_25[5]) * (y_sum_tmp_42314_26[1])))
                        + ((x_sum_tmp_42314_25[6]) * (y_sum_tmp_42314_26[0]))),
                    (((((((x_sum_tmp_42314_25[1]) * (y_sum_tmp_42314_26[6]))
                        + ((x_sum_tmp_42314_25[2]) * (y_sum_tmp_42314_26[5])))
                        + ((x_sum_tmp_42314_25[3]) * (y_sum_tmp_42314_26[4])))
                        + ((x_sum_tmp_42314_25[4]) * (y_sum_tmp_42314_26[3])))
                        + ((x_sum_tmp_42314_25[5]) * (y_sum_tmp_42314_26[2])))
                        + ((x_sum_tmp_42314_25[6]) * (y_sum_tmp_42314_26[1]))),
                    ((((((x_sum_tmp_42314_25[2]) * (y_sum_tmp_42314_26[6]))
                        + ((x_sum_tmp_42314_25[3]) * (y_sum_tmp_42314_26[5])))
                        + ((x_sum_tmp_42314_25[4]) * (y_sum_tmp_42314_26[4])))
                        + ((x_sum_tmp_42314_25[5]) * (y_sum_tmp_42314_26[3])))
                        + ((x_sum_tmp_42314_25[6]) * (y_sum_tmp_42314_26[2]))),
                    (((((x_sum_tmp_42314_25[3]) * (y_sum_tmp_42314_26[6]))
                        + ((x_sum_tmp_42314_25[4]) * (y_sum_tmp_42314_26[5])))
                        + ((x_sum_tmp_42314_25[5]) * (y_sum_tmp_42314_26[4])))
                        + ((x_sum_tmp_42314_25[6]) * (y_sum_tmp_42314_26[3]))),
                    ((((x_sum_tmp_42314_25[4]) * (y_sum_tmp_42314_26[6]))
                        + ((x_sum_tmp_42314_25[5]) * (y_sum_tmp_42314_26[5])))
                        + ((x_sum_tmp_42314_25[6]) * (y_sum_tmp_42314_26[4]))),
                    (((x_sum_tmp_42314_25[5]) * (y_sum_tmp_42314_26[6]))
                        + ((x_sum_tmp_42314_25[6]) * (y_sum_tmp_42314_26[5]))),
                    ((x_sum_tmp_42314_25[6]) * (y_sum_tmp_42314_26[6])),
                ];
                let z2_tmp_42314_28 = [
                    ((x_sum_tmp_42314_25[7]) * (y_sum_tmp_42314_26[7])),
                    (((x_sum_tmp_42314_25[7]) * (y_sum_tmp_42314_26[8]))
                        + ((x_sum_tmp_42314_25[8]) * (y_sum_tmp_42314_26[7]))),
                    ((((x_sum_tmp_42314_25[7]) * (y_sum_tmp_42314_26[9]))
                        + ((x_sum_tmp_42314_25[8]) * (y_sum_tmp_42314_26[8])))
                        + ((x_sum_tmp_42314_25[9]) * (y_sum_tmp_42314_26[7]))),
                    (((((x_sum_tmp_42314_25[7]) * (y_sum_tmp_42314_26[10]))
                        + ((x_sum_tmp_42314_25[8]) * (y_sum_tmp_42314_26[9])))
                        + ((x_sum_tmp_42314_25[9]) * (y_sum_tmp_42314_26[8])))
                        + ((x_sum_tmp_42314_25[10]) * (y_sum_tmp_42314_26[7]))),
                    ((((((x_sum_tmp_42314_25[7]) * (y_sum_tmp_42314_26[11]))
                        + ((x_sum_tmp_42314_25[8]) * (y_sum_tmp_42314_26[10])))
                        + ((x_sum_tmp_42314_25[9]) * (y_sum_tmp_42314_26[9])))
                        + ((x_sum_tmp_42314_25[10]) * (y_sum_tmp_42314_26[8])))
                        + ((x_sum_tmp_42314_25[11]) * (y_sum_tmp_42314_26[7]))),
                    (((((((x_sum_tmp_42314_25[7]) * (y_sum_tmp_42314_26[12]))
                        + ((x_sum_tmp_42314_25[8]) * (y_sum_tmp_42314_26[11])))
                        + ((x_sum_tmp_42314_25[9]) * (y_sum_tmp_42314_26[10])))
                        + ((x_sum_tmp_42314_25[10]) * (y_sum_tmp_42314_26[9])))
                        + ((x_sum_tmp_42314_25[11]) * (y_sum_tmp_42314_26[8])))
                        + ((x_sum_tmp_42314_25[12]) * (y_sum_tmp_42314_26[7]))),
                    ((((((((x_sum_tmp_42314_25[7]) * (y_sum_tmp_42314_26[13]))
                        + ((x_sum_tmp_42314_25[8]) * (y_sum_tmp_42314_26[12])))
                        + ((x_sum_tmp_42314_25[9]) * (y_sum_tmp_42314_26[11])))
                        + ((x_sum_tmp_42314_25[10]) * (y_sum_tmp_42314_26[10])))
                        + ((x_sum_tmp_42314_25[11]) * (y_sum_tmp_42314_26[9])))
                        + ((x_sum_tmp_42314_25[12]) * (y_sum_tmp_42314_26[8])))
                        + ((x_sum_tmp_42314_25[13]) * (y_sum_tmp_42314_26[7]))),
                    (((((((x_sum_tmp_42314_25[8]) * (y_sum_tmp_42314_26[13]))
                        + ((x_sum_tmp_42314_25[9]) * (y_sum_tmp_42314_26[12])))
                        + ((x_sum_tmp_42314_25[10]) * (y_sum_tmp_42314_26[11])))
                        + ((x_sum_tmp_42314_25[11]) * (y_sum_tmp_42314_26[10])))
                        + ((x_sum_tmp_42314_25[12]) * (y_sum_tmp_42314_26[9])))
                        + ((x_sum_tmp_42314_25[13]) * (y_sum_tmp_42314_26[8]))),
                    ((((((x_sum_tmp_42314_25[9]) * (y_sum_tmp_42314_26[13]))
                        + ((x_sum_tmp_42314_25[10]) * (y_sum_tmp_42314_26[12])))
                        + ((x_sum_tmp_42314_25[11]) * (y_sum_tmp_42314_26[11])))
                        + ((x_sum_tmp_42314_25[12]) * (y_sum_tmp_42314_26[10])))
                        + ((x_sum_tmp_42314_25[13]) * (y_sum_tmp_42314_26[9]))),
                    (((((x_sum_tmp_42314_25[10]) * (y_sum_tmp_42314_26[13]))
                        + ((x_sum_tmp_42314_25[11]) * (y_sum_tmp_42314_26[12])))
                        + ((x_sum_tmp_42314_25[12]) * (y_sum_tmp_42314_26[11])))
                        + ((x_sum_tmp_42314_25[13]) * (y_sum_tmp_42314_26[10]))),
                    ((((x_sum_tmp_42314_25[11]) * (y_sum_tmp_42314_26[13]))
                        + ((x_sum_tmp_42314_25[12]) * (y_sum_tmp_42314_26[12])))
                        + ((x_sum_tmp_42314_25[13]) * (y_sum_tmp_42314_26[11]))),
                    (((x_sum_tmp_42314_25[12]) * (y_sum_tmp_42314_26[13]))
                        + ((x_sum_tmp_42314_25[13]) * (y_sum_tmp_42314_26[12]))),
                    ((x_sum_tmp_42314_25[13]) * (y_sum_tmp_42314_26[13])),
                ];
                let x_sum_tmp_42314_29 = [
                    ((x_sum_tmp_42314_25[0]) + (x_sum_tmp_42314_25[7])),
                    ((x_sum_tmp_42314_25[1]) + (x_sum_tmp_42314_25[8])),
                    ((x_sum_tmp_42314_25[2]) + (x_sum_tmp_42314_25[9])),
                    ((x_sum_tmp_42314_25[3]) + (x_sum_tmp_42314_25[10])),
                    ((x_sum_tmp_42314_25[4]) + (x_sum_tmp_42314_25[11])),
                    ((x_sum_tmp_42314_25[5]) + (x_sum_tmp_42314_25[12])),
                    ((x_sum_tmp_42314_25[6]) + (x_sum_tmp_42314_25[13])),
                ];
                let y_sum_tmp_42314_30 = [
                    ((y_sum_tmp_42314_26[0]) + (y_sum_tmp_42314_26[7])),
                    ((y_sum_tmp_42314_26[1]) + (y_sum_tmp_42314_26[8])),
                    ((y_sum_tmp_42314_26[2]) + (y_sum_tmp_42314_26[9])),
                    ((y_sum_tmp_42314_26[3]) + (y_sum_tmp_42314_26[10])),
                    ((y_sum_tmp_42314_26[4]) + (y_sum_tmp_42314_26[11])),
                    ((y_sum_tmp_42314_26[5]) + (y_sum_tmp_42314_26[12])),
                    ((y_sum_tmp_42314_26[6]) + (y_sum_tmp_42314_26[13])),
                ];

                let conv_tmp_42314_31 = [
                    ((z0_tmp_42314_23[0]) - (dst_limb_0_col14)),
                    ((z0_tmp_42314_23[1]) - (dst_limb_1_col15)),
                    ((z0_tmp_42314_23[2]) - (dst_limb_2_col16)),
                    ((z0_tmp_42314_23[3]) - (dst_limb_3_col17)),
                    ((z0_tmp_42314_23[4]) - (dst_limb_4_col18)),
                    ((z0_tmp_42314_23[5]) - (dst_limb_5_col19)),
                    ((z0_tmp_42314_23[6]) - (dst_limb_6_col20)),
                    ((z0_tmp_42314_23[7]) - (dst_limb_7_col21)),
                    ((z0_tmp_42314_23[8]) - (dst_limb_8_col22)),
                    ((z0_tmp_42314_23[9]) - (dst_limb_9_col23)),
                    ((z0_tmp_42314_23[10]) - (dst_limb_10_col24)),
                    ((z0_tmp_42314_23[11]) - (dst_limb_11_col25)),
                    ((z0_tmp_42314_23[12]) - (dst_limb_12_col26)),
                    ((z0_tmp_42314_23[13]) - (dst_limb_13_col27)),
                    (((z0_tmp_42314_23[14])
                        + (((z0_tmp_42314_27[0]) - (z0_tmp_42314_23[0])) - (z2_tmp_42314_24[0])))
                        - (dst_limb_14_col28)),
                    (((z0_tmp_42314_23[15])
                        + (((z0_tmp_42314_27[1]) - (z0_tmp_42314_23[1])) - (z2_tmp_42314_24[1])))
                        - (dst_limb_15_col29)),
                    (((z0_tmp_42314_23[16])
                        + (((z0_tmp_42314_27[2]) - (z0_tmp_42314_23[2])) - (z2_tmp_42314_24[2])))
                        - (dst_limb_16_col30)),
                    (((z0_tmp_42314_23[17])
                        + (((z0_tmp_42314_27[3]) - (z0_tmp_42314_23[3])) - (z2_tmp_42314_24[3])))
                        - (dst_limb_17_col31)),
                    (((z0_tmp_42314_23[18])
                        + (((z0_tmp_42314_27[4]) - (z0_tmp_42314_23[4])) - (z2_tmp_42314_24[4])))
                        - (dst_limb_18_col32)),
                    (((z0_tmp_42314_23[19])
                        + (((z0_tmp_42314_27[5]) - (z0_tmp_42314_23[5])) - (z2_tmp_42314_24[5])))
                        - (dst_limb_19_col33)),
                    (((z0_tmp_42314_23[20])
                        + (((z0_tmp_42314_27[6]) - (z0_tmp_42314_23[6])) - (z2_tmp_42314_24[6])))
                        - (dst_limb_20_col34)),
                    (((z0_tmp_42314_23[21])
                        + ((((z0_tmp_42314_27[7])
                            + ((((x_sum_tmp_42314_29[0]) * (y_sum_tmp_42314_30[0]))
                                - (z0_tmp_42314_27[0]))
                                - (z2_tmp_42314_28[0])))
                            - (z0_tmp_42314_23[7]))
                            - (z2_tmp_42314_24[7])))
                        - (dst_limb_21_col35)),
                    (((z0_tmp_42314_23[22])
                        + ((((z0_tmp_42314_27[8])
                            + (((((x_sum_tmp_42314_29[0]) * (y_sum_tmp_42314_30[1]))
                                + ((x_sum_tmp_42314_29[1]) * (y_sum_tmp_42314_30[0])))
                                - (z0_tmp_42314_27[1]))
                                - (z2_tmp_42314_28[1])))
                            - (z0_tmp_42314_23[8]))
                            - (z2_tmp_42314_24[8])))
                        - (dst_limb_22_col36)),
                    (((z0_tmp_42314_23[23])
                        + ((((z0_tmp_42314_27[9])
                            + ((((((x_sum_tmp_42314_29[0]) * (y_sum_tmp_42314_30[2]))
                                + ((x_sum_tmp_42314_29[1]) * (y_sum_tmp_42314_30[1])))
                                + ((x_sum_tmp_42314_29[2]) * (y_sum_tmp_42314_30[0])))
                                - (z0_tmp_42314_27[2]))
                                - (z2_tmp_42314_28[2])))
                            - (z0_tmp_42314_23[9]))
                            - (z2_tmp_42314_24[9])))
                        - (dst_limb_23_col37)),
                    (((z0_tmp_42314_23[24])
                        + ((((z0_tmp_42314_27[10])
                            + (((((((x_sum_tmp_42314_29[0]) * (y_sum_tmp_42314_30[3]))
                                + ((x_sum_tmp_42314_29[1]) * (y_sum_tmp_42314_30[2])))
                                + ((x_sum_tmp_42314_29[2]) * (y_sum_tmp_42314_30[1])))
                                + ((x_sum_tmp_42314_29[3]) * (y_sum_tmp_42314_30[0])))
                                - (z0_tmp_42314_27[3]))
                                - (z2_tmp_42314_28[3])))
                            - (z0_tmp_42314_23[10]))
                            - (z2_tmp_42314_24[10])))
                        - (dst_limb_24_col38)),
                    (((z0_tmp_42314_23[25])
                        + ((((z0_tmp_42314_27[11])
                            + ((((((((x_sum_tmp_42314_29[0]) * (y_sum_tmp_42314_30[4]))
                                + ((x_sum_tmp_42314_29[1]) * (y_sum_tmp_42314_30[3])))
                                + ((x_sum_tmp_42314_29[2]) * (y_sum_tmp_42314_30[2])))
                                + ((x_sum_tmp_42314_29[3]) * (y_sum_tmp_42314_30[1])))
                                + ((x_sum_tmp_42314_29[4]) * (y_sum_tmp_42314_30[0])))
                                - (z0_tmp_42314_27[4]))
                                - (z2_tmp_42314_28[4])))
                            - (z0_tmp_42314_23[11]))
                            - (z2_tmp_42314_24[11])))
                        - (dst_limb_25_col39)),
                    (((z0_tmp_42314_23[26])
                        + ((((z0_tmp_42314_27[12])
                            + (((((((((x_sum_tmp_42314_29[0])
                                * (y_sum_tmp_42314_30[5]))
                                + ((x_sum_tmp_42314_29[1]) * (y_sum_tmp_42314_30[4])))
                                + ((x_sum_tmp_42314_29[2]) * (y_sum_tmp_42314_30[3])))
                                + ((x_sum_tmp_42314_29[3]) * (y_sum_tmp_42314_30[2])))
                                + ((x_sum_tmp_42314_29[4]) * (y_sum_tmp_42314_30[1])))
                                + ((x_sum_tmp_42314_29[5]) * (y_sum_tmp_42314_30[0])))
                                - (z0_tmp_42314_27[5]))
                                - (z2_tmp_42314_28[5])))
                            - (z0_tmp_42314_23[12]))
                            - (z2_tmp_42314_24[12])))
                        - (dst_limb_26_col40)),
                    (((((((((((((x_sum_tmp_42314_29[0]) * (y_sum_tmp_42314_30[6]))
                        + ((x_sum_tmp_42314_29[1]) * (y_sum_tmp_42314_30[5])))
                        + ((x_sum_tmp_42314_29[2]) * (y_sum_tmp_42314_30[4])))
                        + ((x_sum_tmp_42314_29[3]) * (y_sum_tmp_42314_30[3])))
                        + ((x_sum_tmp_42314_29[4]) * (y_sum_tmp_42314_30[2])))
                        + ((x_sum_tmp_42314_29[5]) * (y_sum_tmp_42314_30[1])))
                        + ((x_sum_tmp_42314_29[6]) * (y_sum_tmp_42314_30[0])))
                        - (z0_tmp_42314_27[6]))
                        - (z2_tmp_42314_28[6]))
                        - (z0_tmp_42314_23[13]))
                        - (z2_tmp_42314_24[13]))
                        - (dst_limb_27_col41)),
                    ((z2_tmp_42314_24[0])
                        + ((((z2_tmp_42314_28[0])
                            + (((((((((x_sum_tmp_42314_29[1]) * (y_sum_tmp_42314_30[6]))
                                + ((x_sum_tmp_42314_29[2]) * (y_sum_tmp_42314_30[5])))
                                + ((x_sum_tmp_42314_29[3]) * (y_sum_tmp_42314_30[4])))
                                + ((x_sum_tmp_42314_29[4]) * (y_sum_tmp_42314_30[3])))
                                + ((x_sum_tmp_42314_29[5]) * (y_sum_tmp_42314_30[2])))
                                + ((x_sum_tmp_42314_29[6]) * (y_sum_tmp_42314_30[1])))
                                - (z0_tmp_42314_27[7]))
                                - (z2_tmp_42314_28[7])))
                            - (z0_tmp_42314_23[14]))
                            - (z2_tmp_42314_24[14]))),
                    ((z2_tmp_42314_24[1])
                        + ((((z2_tmp_42314_28[1])
                            + ((((((((x_sum_tmp_42314_29[2]) * (y_sum_tmp_42314_30[6]))
                                + ((x_sum_tmp_42314_29[3]) * (y_sum_tmp_42314_30[5])))
                                + ((x_sum_tmp_42314_29[4]) * (y_sum_tmp_42314_30[4])))
                                + ((x_sum_tmp_42314_29[5]) * (y_sum_tmp_42314_30[3])))
                                + ((x_sum_tmp_42314_29[6]) * (y_sum_tmp_42314_30[2])))
                                - (z0_tmp_42314_27[8]))
                                - (z2_tmp_42314_28[8])))
                            - (z0_tmp_42314_23[15]))
                            - (z2_tmp_42314_24[15]))),
                    ((z2_tmp_42314_24[2])
                        + ((((z2_tmp_42314_28[2])
                            + (((((((x_sum_tmp_42314_29[3]) * (y_sum_tmp_42314_30[6]))
                                + ((x_sum_tmp_42314_29[4]) * (y_sum_tmp_42314_30[5])))
                                + ((x_sum_tmp_42314_29[5]) * (y_sum_tmp_42314_30[4])))
                                + ((x_sum_tmp_42314_29[6]) * (y_sum_tmp_42314_30[3])))
                                - (z0_tmp_42314_27[9]))
                                - (z2_tmp_42314_28[9])))
                            - (z0_tmp_42314_23[16]))
                            - (z2_tmp_42314_24[16]))),
                    ((z2_tmp_42314_24[3])
                        + ((((z2_tmp_42314_28[3])
                            + ((((((x_sum_tmp_42314_29[4]) * (y_sum_tmp_42314_30[6]))
                                + ((x_sum_tmp_42314_29[5]) * (y_sum_tmp_42314_30[5])))
                                + ((x_sum_tmp_42314_29[6]) * (y_sum_tmp_42314_30[4])))
                                - (z0_tmp_42314_27[10]))
                                - (z2_tmp_42314_28[10])))
                            - (z0_tmp_42314_23[17]))
                            - (z2_tmp_42314_24[17]))),
                    ((z2_tmp_42314_24[4])
                        + ((((z2_tmp_42314_28[4])
                            + (((((x_sum_tmp_42314_29[5]) * (y_sum_tmp_42314_30[6]))
                                + ((x_sum_tmp_42314_29[6]) * (y_sum_tmp_42314_30[5])))
                                - (z0_tmp_42314_27[11]))
                                - (z2_tmp_42314_28[11])))
                            - (z0_tmp_42314_23[18]))
                            - (z2_tmp_42314_24[18]))),
                    ((z2_tmp_42314_24[5])
                        + ((((z2_tmp_42314_28[5])
                            + ((((x_sum_tmp_42314_29[6]) * (y_sum_tmp_42314_30[6]))
                                - (z0_tmp_42314_27[12]))
                                - (z2_tmp_42314_28[12])))
                            - (z0_tmp_42314_23[19]))
                            - (z2_tmp_42314_24[19]))),
                    ((z2_tmp_42314_24[6])
                        + (((z2_tmp_42314_28[6]) - (z0_tmp_42314_23[20])) - (z2_tmp_42314_24[20]))),
                    ((z2_tmp_42314_24[7])
                        + (((z2_tmp_42314_28[7]) - (z0_tmp_42314_23[21])) - (z2_tmp_42314_24[21]))),
                    ((z2_tmp_42314_24[8])
                        + (((z2_tmp_42314_28[8]) - (z0_tmp_42314_23[22])) - (z2_tmp_42314_24[22]))),
                    ((z2_tmp_42314_24[9])
                        + (((z2_tmp_42314_28[9]) - (z0_tmp_42314_23[23])) - (z2_tmp_42314_24[23]))),
                    ((z2_tmp_42314_24[10])
                        + (((z2_tmp_42314_28[10]) - (z0_tmp_42314_23[24]))
                            - (z2_tmp_42314_24[24]))),
                    ((z2_tmp_42314_24[11])
                        + (((z2_tmp_42314_28[11]) - (z0_tmp_42314_23[25]))
                            - (z2_tmp_42314_24[25]))),
                    ((z2_tmp_42314_24[12])
                        + (((z2_tmp_42314_28[12]) - (z0_tmp_42314_23[26]))
                            - (z2_tmp_42314_24[26]))),
                    z2_tmp_42314_24[13],
                    z2_tmp_42314_24[14],
                    z2_tmp_42314_24[15],
                    z2_tmp_42314_24[16],
                    z2_tmp_42314_24[17],
                    z2_tmp_42314_24[18],
                    z2_tmp_42314_24[19],
                    z2_tmp_42314_24[20],
                    z2_tmp_42314_24[21],
                    z2_tmp_42314_24[22],
                    z2_tmp_42314_24[23],
                    z2_tmp_42314_24[24],
                    z2_tmp_42314_24[25],
                    z2_tmp_42314_24[26],
                ];
                let conv_mod_tmp_42314_32 = [
                    ((((M31_32) * (conv_tmp_42314_31[0])) - ((M31_4) * (conv_tmp_42314_31[21])))
                        + ((M31_8) * (conv_tmp_42314_31[49]))),
                    ((((conv_tmp_42314_31[0]) + ((M31_32) * (conv_tmp_42314_31[1])))
                        - ((M31_4) * (conv_tmp_42314_31[22])))
                        + ((M31_8) * (conv_tmp_42314_31[50]))),
                    ((((conv_tmp_42314_31[1]) + ((M31_32) * (conv_tmp_42314_31[2])))
                        - ((M31_4) * (conv_tmp_42314_31[23])))
                        + ((M31_8) * (conv_tmp_42314_31[51]))),
                    ((((conv_tmp_42314_31[2]) + ((M31_32) * (conv_tmp_42314_31[3])))
                        - ((M31_4) * (conv_tmp_42314_31[24])))
                        + ((M31_8) * (conv_tmp_42314_31[52]))),
                    ((((conv_tmp_42314_31[3]) + ((M31_32) * (conv_tmp_42314_31[4])))
                        - ((M31_4) * (conv_tmp_42314_31[25])))
                        + ((M31_8) * (conv_tmp_42314_31[53]))),
                    ((((conv_tmp_42314_31[4]) + ((M31_32) * (conv_tmp_42314_31[5])))
                        - ((M31_4) * (conv_tmp_42314_31[26])))
                        + ((M31_8) * (conv_tmp_42314_31[54]))),
                    (((conv_tmp_42314_31[5]) + ((M31_32) * (conv_tmp_42314_31[6])))
                        - ((M31_4) * (conv_tmp_42314_31[27]))),
                    (((((M31_2) * (conv_tmp_42314_31[0])) + (conv_tmp_42314_31[6]))
                        + ((M31_32) * (conv_tmp_42314_31[7])))
                        - ((M31_4) * (conv_tmp_42314_31[28]))),
                    (((((M31_2) * (conv_tmp_42314_31[1])) + (conv_tmp_42314_31[7]))
                        + ((M31_32) * (conv_tmp_42314_31[8])))
                        - ((M31_4) * (conv_tmp_42314_31[29]))),
                    (((((M31_2) * (conv_tmp_42314_31[2])) + (conv_tmp_42314_31[8]))
                        + ((M31_32) * (conv_tmp_42314_31[9])))
                        - ((M31_4) * (conv_tmp_42314_31[30]))),
                    (((((M31_2) * (conv_tmp_42314_31[3])) + (conv_tmp_42314_31[9]))
                        + ((M31_32) * (conv_tmp_42314_31[10])))
                        - ((M31_4) * (conv_tmp_42314_31[31]))),
                    (((((M31_2) * (conv_tmp_42314_31[4])) + (conv_tmp_42314_31[10]))
                        + ((M31_32) * (conv_tmp_42314_31[11])))
                        - ((M31_4) * (conv_tmp_42314_31[32]))),
                    (((((M31_2) * (conv_tmp_42314_31[5])) + (conv_tmp_42314_31[11]))
                        + ((M31_32) * (conv_tmp_42314_31[12])))
                        - ((M31_4) * (conv_tmp_42314_31[33]))),
                    (((((M31_2) * (conv_tmp_42314_31[6])) + (conv_tmp_42314_31[12]))
                        + ((M31_32) * (conv_tmp_42314_31[13])))
                        - ((M31_4) * (conv_tmp_42314_31[34]))),
                    (((((M31_2) * (conv_tmp_42314_31[7])) + (conv_tmp_42314_31[13]))
                        + ((M31_32) * (conv_tmp_42314_31[14])))
                        - ((M31_4) * (conv_tmp_42314_31[35]))),
                    (((((M31_2) * (conv_tmp_42314_31[8])) + (conv_tmp_42314_31[14]))
                        + ((M31_32) * (conv_tmp_42314_31[15])))
                        - ((M31_4) * (conv_tmp_42314_31[36]))),
                    (((((M31_2) * (conv_tmp_42314_31[9])) + (conv_tmp_42314_31[15]))
                        + ((M31_32) * (conv_tmp_42314_31[16])))
                        - ((M31_4) * (conv_tmp_42314_31[37]))),
                    (((((M31_2) * (conv_tmp_42314_31[10])) + (conv_tmp_42314_31[16]))
                        + ((M31_32) * (conv_tmp_42314_31[17])))
                        - ((M31_4) * (conv_tmp_42314_31[38]))),
                    (((((M31_2) * (conv_tmp_42314_31[11])) + (conv_tmp_42314_31[17]))
                        + ((M31_32) * (conv_tmp_42314_31[18])))
                        - ((M31_4) * (conv_tmp_42314_31[39]))),
                    (((((M31_2) * (conv_tmp_42314_31[12])) + (conv_tmp_42314_31[18]))
                        + ((M31_32) * (conv_tmp_42314_31[19])))
                        - ((M31_4) * (conv_tmp_42314_31[40]))),
                    (((((M31_2) * (conv_tmp_42314_31[13])) + (conv_tmp_42314_31[19]))
                        + ((M31_32) * (conv_tmp_42314_31[20])))
                        - ((M31_4) * (conv_tmp_42314_31[41]))),
                    (((((M31_2) * (conv_tmp_42314_31[14])) + (conv_tmp_42314_31[20]))
                        - ((M31_4) * (conv_tmp_42314_31[42])))
                        + ((M31_64) * (conv_tmp_42314_31[49]))),
                    (((((M31_2) * (conv_tmp_42314_31[15])) - ((M31_4) * (conv_tmp_42314_31[43])))
                        + ((M31_2) * (conv_tmp_42314_31[49])))
                        + ((M31_64) * (conv_tmp_42314_31[50]))),
                    (((((M31_2) * (conv_tmp_42314_31[16])) - ((M31_4) * (conv_tmp_42314_31[44])))
                        + ((M31_2) * (conv_tmp_42314_31[50])))
                        + ((M31_64) * (conv_tmp_42314_31[51]))),
                    (((((M31_2) * (conv_tmp_42314_31[17])) - ((M31_4) * (conv_tmp_42314_31[45])))
                        + ((M31_2) * (conv_tmp_42314_31[51])))
                        + ((M31_64) * (conv_tmp_42314_31[52]))),
                    (((((M31_2) * (conv_tmp_42314_31[18])) - ((M31_4) * (conv_tmp_42314_31[46])))
                        + ((M31_2) * (conv_tmp_42314_31[52])))
                        + ((M31_64) * (conv_tmp_42314_31[53]))),
                    (((((M31_2) * (conv_tmp_42314_31[19])) - ((M31_4) * (conv_tmp_42314_31[47])))
                        + ((M31_2) * (conv_tmp_42314_31[53])))
                        + ((M31_64) * (conv_tmp_42314_31[54]))),
                    ((((M31_2) * (conv_tmp_42314_31[20])) - ((M31_4) * (conv_tmp_42314_31[48])))
                        + ((M31_2) * (conv_tmp_42314_31[54]))),
                ];
                let k_mod_2_18_biased_tmp_42314_33 =
                    ((((PackedUInt32::from_m31(((conv_mod_tmp_42314_32[0]) + (M31_134217728))))
                        + (((PackedUInt32::from_m31(
                            ((conv_mod_tmp_42314_32[1]) + (M31_134217728)),
                        )) & (UInt32_511))
                            << (UInt32_9)))
                        + (UInt32_65536))
                        & (UInt32_262143));
                let k_col100 = ((k_mod_2_18_biased_tmp_42314_33.low().as_m31())
                    + (((k_mod_2_18_biased_tmp_42314_33.high().as_m31()) - (M31_1)) * (M31_65536)));
                *row[100] = k_col100;
                *sub_component_inputs.range_check_19[0] = [((k_col100) + (M31_262144))];
                *lookup_data.range_check_19_0 = [((k_col100) + (M31_262144))];
                let carry_0_col101 = (((conv_mod_tmp_42314_32[0]) - (k_col100)) * (M31_4194304));
                *row[101] = carry_0_col101;
                *sub_component_inputs.range_check_19[1] = [((carry_0_col101) + (M31_131072))];
                *lookup_data.range_check_19_1 = [((carry_0_col101) + (M31_131072))];
                let carry_1_col102 =
                    (((conv_mod_tmp_42314_32[1]) + (carry_0_col101)) * (M31_4194304));
                *row[102] = carry_1_col102;
                *sub_component_inputs.range_check_19[2] = [((carry_1_col102) + (M31_131072))];
                *lookup_data.range_check_19_2 = [((carry_1_col102) + (M31_131072))];
                let carry_2_col103 =
                    (((conv_mod_tmp_42314_32[2]) + (carry_1_col102)) * (M31_4194304));
                *row[103] = carry_2_col103;
                *sub_component_inputs.range_check_19[3] = [((carry_2_col103) + (M31_131072))];
                *lookup_data.range_check_19_3 = [((carry_2_col103) + (M31_131072))];
                let carry_3_col104 =
                    (((conv_mod_tmp_42314_32[3]) + (carry_2_col103)) * (M31_4194304));
                *row[104] = carry_3_col104;
                *sub_component_inputs.range_check_19[4] = [((carry_3_col104) + (M31_131072))];
                *lookup_data.range_check_19_4 = [((carry_3_col104) + (M31_131072))];
                let carry_4_col105 =
                    (((conv_mod_tmp_42314_32[4]) + (carry_3_col104)) * (M31_4194304));
                *row[105] = carry_4_col105;
                *sub_component_inputs.range_check_19[5] = [((carry_4_col105) + (M31_131072))];
                *lookup_data.range_check_19_5 = [((carry_4_col105) + (M31_131072))];
                let carry_5_col106 =
                    (((conv_mod_tmp_42314_32[5]) + (carry_4_col105)) * (M31_4194304));
                *row[106] = carry_5_col106;
                *sub_component_inputs.range_check_19[6] = [((carry_5_col106) + (M31_131072))];
                *lookup_data.range_check_19_6 = [((carry_5_col106) + (M31_131072))];
                let carry_6_col107 =
                    (((conv_mod_tmp_42314_32[6]) + (carry_5_col106)) * (M31_4194304));
                *row[107] = carry_6_col107;
                *sub_component_inputs.range_check_19[7] = [((carry_6_col107) + (M31_131072))];
                *lookup_data.range_check_19_7 = [((carry_6_col107) + (M31_131072))];
                let carry_7_col108 =
                    (((conv_mod_tmp_42314_32[7]) + (carry_6_col107)) * (M31_4194304));
                *row[108] = carry_7_col108;
                *sub_component_inputs.range_check_19[8] = [((carry_7_col108) + (M31_131072))];
                *lookup_data.range_check_19_8 = [((carry_7_col108) + (M31_131072))];
                let carry_8_col109 =
                    (((conv_mod_tmp_42314_32[8]) + (carry_7_col108)) * (M31_4194304));
                *row[109] = carry_8_col109;
                *sub_component_inputs.range_check_19[9] = [((carry_8_col109) + (M31_131072))];
                *lookup_data.range_check_19_9 = [((carry_8_col109) + (M31_131072))];
                let carry_9_col110 =
                    (((conv_mod_tmp_42314_32[9]) + (carry_8_col109)) * (M31_4194304));
                *row[110] = carry_9_col110;
                *sub_component_inputs.range_check_19[10] = [((carry_9_col110) + (M31_131072))];
                *lookup_data.range_check_19_10 = [((carry_9_col110) + (M31_131072))];
                let carry_10_col111 =
                    (((conv_mod_tmp_42314_32[10]) + (carry_9_col110)) * (M31_4194304));
                *row[111] = carry_10_col111;
                *sub_component_inputs.range_check_19[11] = [((carry_10_col111) + (M31_131072))];
                *lookup_data.range_check_19_11 = [((carry_10_col111) + (M31_131072))];
                let carry_11_col112 =
                    (((conv_mod_tmp_42314_32[11]) + (carry_10_col111)) * (M31_4194304));
                *row[112] = carry_11_col112;
                *sub_component_inputs.range_check_19[12] = [((carry_11_col112) + (M31_131072))];
                *lookup_data.range_check_19_12 = [((carry_11_col112) + (M31_131072))];
                let carry_12_col113 =
                    (((conv_mod_tmp_42314_32[12]) + (carry_11_col112)) * (M31_4194304));
                *row[113] = carry_12_col113;
                *sub_component_inputs.range_check_19[13] = [((carry_12_col113) + (M31_131072))];
                *lookup_data.range_check_19_13 = [((carry_12_col113) + (M31_131072))];
                let carry_13_col114 =
                    (((conv_mod_tmp_42314_32[13]) + (carry_12_col113)) * (M31_4194304));
                *row[114] = carry_13_col114;
                *sub_component_inputs.range_check_19[14] = [((carry_13_col114) + (M31_131072))];
                *lookup_data.range_check_19_14 = [((carry_13_col114) + (M31_131072))];
                let carry_14_col115 =
                    (((conv_mod_tmp_42314_32[14]) + (carry_13_col114)) * (M31_4194304));
                *row[115] = carry_14_col115;
                *sub_component_inputs.range_check_19[15] = [((carry_14_col115) + (M31_131072))];
                *lookup_data.range_check_19_15 = [((carry_14_col115) + (M31_131072))];
                let carry_15_col116 =
                    (((conv_mod_tmp_42314_32[15]) + (carry_14_col115)) * (M31_4194304));
                *row[116] = carry_15_col116;
                *sub_component_inputs.range_check_19[16] = [((carry_15_col116) + (M31_131072))];
                *lookup_data.range_check_19_16 = [((carry_15_col116) + (M31_131072))];
                let carry_16_col117 =
                    (((conv_mod_tmp_42314_32[16]) + (carry_15_col116)) * (M31_4194304));
                *row[117] = carry_16_col117;
                *sub_component_inputs.range_check_19[17] = [((carry_16_col117) + (M31_131072))];
                *lookup_data.range_check_19_17 = [((carry_16_col117) + (M31_131072))];
                let carry_17_col118 =
                    (((conv_mod_tmp_42314_32[17]) + (carry_16_col117)) * (M31_4194304));
                *row[118] = carry_17_col118;
                *sub_component_inputs.range_check_19[18] = [((carry_17_col118) + (M31_131072))];
                *lookup_data.range_check_19_18 = [((carry_17_col118) + (M31_131072))];
                let carry_18_col119 =
                    (((conv_mod_tmp_42314_32[18]) + (carry_17_col118)) * (M31_4194304));
                *row[119] = carry_18_col119;
                *sub_component_inputs.range_check_19[19] = [((carry_18_col119) + (M31_131072))];
                *lookup_data.range_check_19_19 = [((carry_18_col119) + (M31_131072))];
                let carry_19_col120 =
                    (((conv_mod_tmp_42314_32[19]) + (carry_18_col119)) * (M31_4194304));
                *row[120] = carry_19_col120;
                *sub_component_inputs.range_check_19[20] = [((carry_19_col120) + (M31_131072))];
                *lookup_data.range_check_19_20 = [((carry_19_col120) + (M31_131072))];
                let carry_20_col121 =
                    (((conv_mod_tmp_42314_32[20]) + (carry_19_col120)) * (M31_4194304));
                *row[121] = carry_20_col121;
                *sub_component_inputs.range_check_19[21] = [((carry_20_col121) + (M31_131072))];
                *lookup_data.range_check_19_21 = [((carry_20_col121) + (M31_131072))];
                let carry_21_col122 = ((((conv_mod_tmp_42314_32[21]) - ((M31_136) * (k_col100)))
                    + (carry_20_col121))
                    * (M31_4194304));
                *row[122] = carry_21_col122;
                *sub_component_inputs.range_check_19[22] = [((carry_21_col122) + (M31_131072))];
                *lookup_data.range_check_19_22 = [((carry_21_col122) + (M31_131072))];
                let carry_22_col123 =
                    (((conv_mod_tmp_42314_32[22]) + (carry_21_col122)) * (M31_4194304));
                *row[123] = carry_22_col123;
                *sub_component_inputs.range_check_19[23] = [((carry_22_col123) + (M31_131072))];
                *lookup_data.range_check_19_23 = [((carry_22_col123) + (M31_131072))];
                let carry_23_col124 =
                    (((conv_mod_tmp_42314_32[23]) + (carry_22_col123)) * (M31_4194304));
                *row[124] = carry_23_col124;
                *sub_component_inputs.range_check_19[24] = [((carry_23_col124) + (M31_131072))];
                *lookup_data.range_check_19_24 = [((carry_23_col124) + (M31_131072))];
                let carry_24_col125 =
                    (((conv_mod_tmp_42314_32[24]) + (carry_23_col124)) * (M31_4194304));
                *row[125] = carry_24_col125;
                *sub_component_inputs.range_check_19[25] = [((carry_24_col125) + (M31_131072))];
                *lookup_data.range_check_19_25 = [((carry_24_col125) + (M31_131072))];
                let carry_25_col126 =
                    (((conv_mod_tmp_42314_32[25]) + (carry_24_col125)) * (M31_4194304));
                *row[126] = carry_25_col126;
                *sub_component_inputs.range_check_19[26] = [((carry_25_col126) + (M31_131072))];
                *lookup_data.range_check_19_26 = [((carry_25_col126) + (M31_131072))];
                let carry_26_col127 =
                    (((conv_mod_tmp_42314_32[26]) + (carry_25_col126)) * (M31_4194304));
                *row[127] = carry_26_col127;
                *sub_component_inputs.range_check_19[27] = [((carry_26_col127) + (M31_131072))];
                *lookup_data.range_check_19_27 = [((carry_26_col127) + (M31_131072))];

                *lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];
                *lookup_data.opcodes_1 = [
                    ((input_pc_col0) + (M31_1)),
                    ((input_ap_col1) + (ap_update_add_1_col9)),
                    input_fp_col2,
                ];
                *row[128] = padding_col.packed_at(row_index);
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
        let padding_col = Enabler::new(self.n_rows);
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_instruction_0,
            &self.lookup_data.memory_address_to_id_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_instruction.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_id_to_big_0,
            &self.lookup_data.memory_address_to_id_1,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_id_to_big_1,
            &self.lookup_data.memory_address_to_id_2,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_id_to_big_2,
            &self.lookup_data.range_check_19_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_1,
            &self.lookup_data.range_check_19_2,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_3,
            &self.lookup_data.range_check_19_4,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_5,
            &self.lookup_data.range_check_19_6,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_7,
            &self.lookup_data.range_check_19_8,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_9,
            &self.lookup_data.range_check_19_10,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_11,
            &self.lookup_data.range_check_19_12,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_13,
            &self.lookup_data.range_check_19_14,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_15,
            &self.lookup_data.range_check_19_16,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_17,
            &self.lookup_data.range_check_19_18,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_19,
            &self.lookup_data.range_check_19_20,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_21,
            &self.lookup_data.range_check_19_22,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_23,
            &self.lookup_data.range_check_19_24,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_25,
            &self.lookup_data.range_check_19_26,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_27,
            &self.lookup_data.opcodes_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = opcodes.combine(values1);
            col_gen.write_frac(
                i,
                denom0 * padding_col.packed_at(i) + denom1,
                denom0 * denom1,
            );
        }
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        for (i, values) in self.lookup_data.opcodes_1.iter().enumerate() {
            let denom = opcodes.combine(values);
            col_gen.write_frac(i, -PackedQM31::one() * padding_col.packed_at(i), denom);
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
