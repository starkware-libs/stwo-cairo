#![allow(unused_parens)]
use super::component::{Claim, InteractionClaim};
use crate::components::prelude::proving::*;
use crate::components::{
    memory_address_to_id, memory_id_to_big, range_check_4_4_4_4, verify_instruction,
};

pub type InputType = CasmState;
pub type PackedInputType = PackedCasmState;
const N_TRACE_COLUMNS: usize = 73;

#[derive(Default)]
pub struct ClaimGenerator {
    pub inputs: Vec<InputType>,
}
impl ClaimGenerator {
    pub fn new(inputs: Vec<InputType>) -> Self {
        Self { inputs }
    }

    pub fn write_trace<MC: MerkleChannel>(
        mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
        range_check_4_4_4_4_state: &range_check_4_4_4_4::ClaimGenerator,
        verify_instruction_state: &verify_instruction::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let n_rows = self.inputs.len();
        assert_ne!(n_rows, 0);
        let size = std::cmp::max(n_rows.next_power_of_two(), N_LANES);
        let log_size = size.ilog2();
        self.inputs.resize(size, *self.inputs.first().unwrap());
        let packed_inputs = pack_values(&self.inputs);

        let (trace, lookup_data) = write_trace_simd(
            n_rows,
            packed_inputs,
            memory_address_to_id_state,
            memory_id_to_big_state,
            range_check_4_4_4_4_state,
            verify_instruction_state,
        );
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

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    n_rows: usize,
    inputs: Vec<PackedInputType>,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
    range_check_4_4_4_4_state: &range_check_4_4_4_4::ClaimGenerator,
    verify_instruction_state: &verify_instruction::ClaimGenerator,
) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData) {
    let log_n_packed_rows = inputs.len().ilog2();
    let log_size = log_n_packed_rows + LOG_N_LANES;
    let (mut trace, mut lookup_data) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
        )
    };

    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_128 = PackedM31::broadcast(M31::from(128));
    let M31_1548 = PackedM31::broadcast(M31::from(1548));
    let M31_16 = PackedM31::broadcast(M31::from(16));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_3 = PackedM31::broadcast(M31::from(3));
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
    let padding_col = Enabler::new(n_rows);

    trace
        .par_iter_mut()
        .enumerate()
        .zip(inputs.into_par_iter())
        .zip(lookup_data.par_iter_mut())
        .for_each(
            |(((row_index, mut row), qm_31_add_mul_opcode_input), lookup_data)| {
                let input_tmp_fa85a_0 = qm_31_add_mul_opcode_input;
                let input_pc_col0 = input_tmp_fa85a_0.pc;
                *row[0] = input_pc_col0;
                let input_ap_col1 = input_tmp_fa85a_0.ap;
                *row[1] = input_ap_col1;
                let input_fp_col2 = input_tmp_fa85a_0.fp;
                *row[2] = input_fp_col2;

                // Decode Instruction.

                let memory_address_to_id_value_tmp_fa85a_1 =
                    memory_address_to_id_state.deduce_output(input_pc_col0);
                let memory_id_to_big_value_tmp_fa85a_2 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_fa85a_1);
                let offset0_tmp_fa85a_3 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_fa85a_2.get_m31(0)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_fa85a_2.get_m31(1),
                        )) & (UInt16_127))
                            << (UInt16_9)));
                let offset0_col3 = offset0_tmp_fa85a_3.as_m31();
                *row[3] = offset0_col3;
                let offset1_tmp_fa85a_4 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_fa85a_2.get_m31(1)))
                        >> (UInt16_7))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_fa85a_2.get_m31(2),
                        )) << (UInt16_2)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_fa85a_2.get_m31(3),
                        )) & (UInt16_31))
                            << (UInt16_11)));
                let offset1_col4 = offset1_tmp_fa85a_4.as_m31();
                *row[4] = offset1_col4;
                let offset2_tmp_fa85a_5 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_fa85a_2.get_m31(3)))
                        >> (UInt16_5))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_fa85a_2.get_m31(4),
                        )) << (UInt16_4)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_fa85a_2.get_m31(5),
                        )) & (UInt16_7))
                            << (UInt16_13)));
                let offset2_col5 = offset2_tmp_fa85a_5.as_m31();
                *row[5] = offset2_col5;
                let dst_base_fp_tmp_fa85a_6 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_fa85a_2.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_fa85a_2.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_0))
                        & (UInt16_1));
                let dst_base_fp_col6 = dst_base_fp_tmp_fa85a_6.as_m31();
                *row[6] = dst_base_fp_col6;
                let op0_base_fp_tmp_fa85a_7 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_fa85a_2.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_fa85a_2.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_1))
                        & (UInt16_1));
                let op0_base_fp_col7 = op0_base_fp_tmp_fa85a_7.as_m31();
                *row[7] = op0_base_fp_col7;
                let op1_imm_tmp_fa85a_8 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_fa85a_2.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_fa85a_2.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_2))
                        & (UInt16_1));
                let op1_imm_col8 = op1_imm_tmp_fa85a_8.as_m31();
                *row[8] = op1_imm_col8;
                let op1_base_fp_tmp_fa85a_9 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_fa85a_2.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_fa85a_2.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_3))
                        & (UInt16_1));
                let op1_base_fp_col9 = op1_base_fp_tmp_fa85a_9.as_m31();
                *row[9] = op1_base_fp_col9;
                let res_add_tmp_fa85a_10 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_fa85a_2.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_fa85a_2.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_5))
                        & (UInt16_1));
                let res_add_col10 = res_add_tmp_fa85a_10.as_m31();
                *row[10] = res_add_col10;
                let ap_update_add_1_tmp_fa85a_11 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_fa85a_2.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_fa85a_2.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_11))
                        & (UInt16_1));
                let ap_update_add_1_col11 = ap_update_add_1_tmp_fa85a_11.as_m31();
                *row[11] = ap_update_add_1_col11;
                let verify_instruction_inputs_0 = (
                    input_pc_col0,
                    [offset0_col3, offset1_col4, offset2_col5],
                    [
                        (((((((M31_0) + ((dst_base_fp_col6) * (M31_8)))
                            + ((op0_base_fp_col7) * (M31_16)))
                            + ((op1_imm_col8) * (M31_32)))
                            + ((op1_base_fp_col9) * (M31_64)))
                            + ((((M31_1) - (op1_imm_col8)) - (op1_base_fp_col9)) * (M31_128)))
                            + ((res_add_col10) * (M31_256))),
                        ((((((((((M31_0) + (((M31_1) - (res_add_col10)) * (M31_1)))
                            + (M31_0))
                            + (M31_0))
                            + (M31_0))
                            + (M31_0))
                            + ((ap_update_add_1_col11) * (M31_32)))
                            + (M31_0))
                            + (M31_0))
                            + (M31_256)),
                    ],
                    M31_3,
                )
                    .unpack();
                *lookup_data.verify_instruction_0 = [
                    input_pc_col0,
                    offset0_col3,
                    offset1_col4,
                    offset2_col5,
                    (((((((M31_0) + ((dst_base_fp_col6) * (M31_8)))
                        + ((op0_base_fp_col7) * (M31_16)))
                        + ((op1_imm_col8) * (M31_32)))
                        + ((op1_base_fp_col9) * (M31_64)))
                        + ((((M31_1) - (op1_imm_col8)) - (op1_base_fp_col9)) * (M31_128)))
                        + ((res_add_col10) * (M31_256))),
                    ((((((((((M31_0) + (((M31_1) - (res_add_col10)) * (M31_1))) + (M31_0))
                        + (M31_0))
                        + (M31_0))
                        + (M31_0))
                        + ((ap_update_add_1_col11) * (M31_32)))
                        + (M31_0))
                        + (M31_0))
                        + (M31_256)),
                    M31_3,
                ];

                let mem_dst_base_col12 = (((dst_base_fp_col6) * (input_fp_col2))
                    + (((M31_1) - (dst_base_fp_col6)) * (input_ap_col1)));
                *row[12] = mem_dst_base_col12;
                let mem0_base_col13 = (((op0_base_fp_col7) * (input_fp_col2))
                    + (((M31_1) - (op0_base_fp_col7)) * (input_ap_col1)));
                *row[13] = mem0_base_col13;
                let mem1_base_col14 = ((((op1_base_fp_col9) * (input_fp_col2))
                    + ((((M31_1) - (op1_imm_col8)) - (op1_base_fp_col9)) * (input_ap_col1)))
                    + ((op1_imm_col8) * (input_pc_col0)));
                *row[14] = mem1_base_col14;

                // Qm 31 Read Reduced.

                // Read Positive Num Bits 144.

                let memory_address_to_id_value_tmp_fa85a_12 = memory_address_to_id_state
                    .deduce_output(((mem_dst_base_col12) + ((offset0_col3) - (M31_32768))));
                let memory_id_to_big_value_tmp_fa85a_13 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_fa85a_12);
                let dst_id_col15 = memory_address_to_id_value_tmp_fa85a_12;
                *row[15] = dst_id_col15;
                let memory_address_to_id_inputs_0 =
                    ((mem_dst_base_col12) + ((offset0_col3) - (M31_32768))).unpack();
                *lookup_data.memory_address_to_id_0 = [
                    ((mem_dst_base_col12) + ((offset0_col3) - (M31_32768))),
                    dst_id_col15,
                ];
                let dst_limb_0_col16 = memory_id_to_big_value_tmp_fa85a_13.get_m31(0);
                *row[16] = dst_limb_0_col16;
                let dst_limb_1_col17 = memory_id_to_big_value_tmp_fa85a_13.get_m31(1);
                *row[17] = dst_limb_1_col17;
                let dst_limb_2_col18 = memory_id_to_big_value_tmp_fa85a_13.get_m31(2);
                *row[18] = dst_limb_2_col18;
                let dst_limb_3_col19 = memory_id_to_big_value_tmp_fa85a_13.get_m31(3);
                *row[19] = dst_limb_3_col19;
                let dst_limb_4_col20 = memory_id_to_big_value_tmp_fa85a_13.get_m31(4);
                *row[20] = dst_limb_4_col20;
                let dst_limb_5_col21 = memory_id_to_big_value_tmp_fa85a_13.get_m31(5);
                *row[21] = dst_limb_5_col21;
                let dst_limb_6_col22 = memory_id_to_big_value_tmp_fa85a_13.get_m31(6);
                *row[22] = dst_limb_6_col22;
                let dst_limb_7_col23 = memory_id_to_big_value_tmp_fa85a_13.get_m31(7);
                *row[23] = dst_limb_7_col23;
                let dst_limb_8_col24 = memory_id_to_big_value_tmp_fa85a_13.get_m31(8);
                *row[24] = dst_limb_8_col24;
                let dst_limb_9_col25 = memory_id_to_big_value_tmp_fa85a_13.get_m31(9);
                *row[25] = dst_limb_9_col25;
                let dst_limb_10_col26 = memory_id_to_big_value_tmp_fa85a_13.get_m31(10);
                *row[26] = dst_limb_10_col26;
                let dst_limb_11_col27 = memory_id_to_big_value_tmp_fa85a_13.get_m31(11);
                *row[27] = dst_limb_11_col27;
                let dst_limb_12_col28 = memory_id_to_big_value_tmp_fa85a_13.get_m31(12);
                *row[28] = dst_limb_12_col28;
                let dst_limb_13_col29 = memory_id_to_big_value_tmp_fa85a_13.get_m31(13);
                *row[29] = dst_limb_13_col29;
                let dst_limb_14_col30 = memory_id_to_big_value_tmp_fa85a_13.get_m31(14);
                *row[30] = dst_limb_14_col30;
                let dst_limb_15_col31 = memory_id_to_big_value_tmp_fa85a_13.get_m31(15);
                *row[31] = dst_limb_15_col31;
                let memory_id_to_big_inputs_0 = dst_id_col15.unpack();
                *lookup_data.memory_id_to_big_0 = [
                    dst_id_col15,
                    dst_limb_0_col16,
                    dst_limb_1_col17,
                    dst_limb_2_col18,
                    dst_limb_3_col19,
                    dst_limb_4_col20,
                    dst_limb_5_col21,
                    dst_limb_6_col22,
                    dst_limb_7_col23,
                    dst_limb_8_col24,
                    dst_limb_9_col25,
                    dst_limb_10_col26,
                    dst_limb_11_col27,
                    dst_limb_12_col28,
                    dst_limb_13_col29,
                    dst_limb_14_col30,
                    dst_limb_15_col31,
                    M31_0,
                    M31_0,
                    M31_0,
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

                let range_check_4_4_4_4_inputs_0 = [
                    dst_limb_3_col19,
                    dst_limb_7_col23,
                    dst_limb_11_col27,
                    dst_limb_15_col31,
                ]
                .unpack();
                *lookup_data.range_check_4_4_4_4_0 = [
                    dst_limb_3_col19,
                    dst_limb_7_col23,
                    dst_limb_11_col27,
                    dst_limb_15_col31,
                ];
                let dst_delta_ab_inv_col32 = ((((((dst_limb_0_col16) + (dst_limb_1_col17))
                    + (dst_limb_2_col18))
                    + (dst_limb_3_col19))
                    - (M31_1548))
                    * (((((dst_limb_4_col20) + (dst_limb_5_col21)) + (dst_limb_6_col22))
                        + (dst_limb_7_col23))
                        - (M31_1548)))
                    .inverse();
                *row[32] = dst_delta_ab_inv_col32;
                let dst_delta_cd_inv_col33 = ((((((dst_limb_8_col24) + (dst_limb_9_col25))
                    + (dst_limb_10_col26))
                    + (dst_limb_11_col27))
                    - (M31_1548))
                    * (((((dst_limb_12_col28) + (dst_limb_13_col29)) + (dst_limb_14_col30))
                        + (dst_limb_15_col31))
                        - (M31_1548)))
                    .inverse();
                *row[33] = dst_delta_cd_inv_col33;

                // Qm 31 Read Reduced.

                // Read Positive Num Bits 144.

                let memory_address_to_id_value_tmp_fa85a_14 = memory_address_to_id_state
                    .deduce_output(((mem0_base_col13) + ((offset1_col4) - (M31_32768))));
                let memory_id_to_big_value_tmp_fa85a_15 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_fa85a_14);
                let op0_id_col34 = memory_address_to_id_value_tmp_fa85a_14;
                *row[34] = op0_id_col34;
                let memory_address_to_id_inputs_1 =
                    ((mem0_base_col13) + ((offset1_col4) - (M31_32768))).unpack();
                *lookup_data.memory_address_to_id_1 = [
                    ((mem0_base_col13) + ((offset1_col4) - (M31_32768))),
                    op0_id_col34,
                ];
                let op0_limb_0_col35 = memory_id_to_big_value_tmp_fa85a_15.get_m31(0);
                *row[35] = op0_limb_0_col35;
                let op0_limb_1_col36 = memory_id_to_big_value_tmp_fa85a_15.get_m31(1);
                *row[36] = op0_limb_1_col36;
                let op0_limb_2_col37 = memory_id_to_big_value_tmp_fa85a_15.get_m31(2);
                *row[37] = op0_limb_2_col37;
                let op0_limb_3_col38 = memory_id_to_big_value_tmp_fa85a_15.get_m31(3);
                *row[38] = op0_limb_3_col38;
                let op0_limb_4_col39 = memory_id_to_big_value_tmp_fa85a_15.get_m31(4);
                *row[39] = op0_limb_4_col39;
                let op0_limb_5_col40 = memory_id_to_big_value_tmp_fa85a_15.get_m31(5);
                *row[40] = op0_limb_5_col40;
                let op0_limb_6_col41 = memory_id_to_big_value_tmp_fa85a_15.get_m31(6);
                *row[41] = op0_limb_6_col41;
                let op0_limb_7_col42 = memory_id_to_big_value_tmp_fa85a_15.get_m31(7);
                *row[42] = op0_limb_7_col42;
                let op0_limb_8_col43 = memory_id_to_big_value_tmp_fa85a_15.get_m31(8);
                *row[43] = op0_limb_8_col43;
                let op0_limb_9_col44 = memory_id_to_big_value_tmp_fa85a_15.get_m31(9);
                *row[44] = op0_limb_9_col44;
                let op0_limb_10_col45 = memory_id_to_big_value_tmp_fa85a_15.get_m31(10);
                *row[45] = op0_limb_10_col45;
                let op0_limb_11_col46 = memory_id_to_big_value_tmp_fa85a_15.get_m31(11);
                *row[46] = op0_limb_11_col46;
                let op0_limb_12_col47 = memory_id_to_big_value_tmp_fa85a_15.get_m31(12);
                *row[47] = op0_limb_12_col47;
                let op0_limb_13_col48 = memory_id_to_big_value_tmp_fa85a_15.get_m31(13);
                *row[48] = op0_limb_13_col48;
                let op0_limb_14_col49 = memory_id_to_big_value_tmp_fa85a_15.get_m31(14);
                *row[49] = op0_limb_14_col49;
                let op0_limb_15_col50 = memory_id_to_big_value_tmp_fa85a_15.get_m31(15);
                *row[50] = op0_limb_15_col50;
                let memory_id_to_big_inputs_1 = op0_id_col34.unpack();
                *lookup_data.memory_id_to_big_1 = [
                    op0_id_col34,
                    op0_limb_0_col35,
                    op0_limb_1_col36,
                    op0_limb_2_col37,
                    op0_limb_3_col38,
                    op0_limb_4_col39,
                    op0_limb_5_col40,
                    op0_limb_6_col41,
                    op0_limb_7_col42,
                    op0_limb_8_col43,
                    op0_limb_9_col44,
                    op0_limb_10_col45,
                    op0_limb_11_col46,
                    op0_limb_12_col47,
                    op0_limb_13_col48,
                    op0_limb_14_col49,
                    op0_limb_15_col50,
                    M31_0,
                    M31_0,
                    M31_0,
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

                let range_check_4_4_4_4_inputs_1 = [
                    op0_limb_3_col38,
                    op0_limb_7_col42,
                    op0_limb_11_col46,
                    op0_limb_15_col50,
                ]
                .unpack();
                *lookup_data.range_check_4_4_4_4_1 = [
                    op0_limb_3_col38,
                    op0_limb_7_col42,
                    op0_limb_11_col46,
                    op0_limb_15_col50,
                ];
                let op0_delta_ab_inv_col51 = ((((((op0_limb_0_col35) + (op0_limb_1_col36))
                    + (op0_limb_2_col37))
                    + (op0_limb_3_col38))
                    - (M31_1548))
                    * (((((op0_limb_4_col39) + (op0_limb_5_col40)) + (op0_limb_6_col41))
                        + (op0_limb_7_col42))
                        - (M31_1548)))
                    .inverse();
                *row[51] = op0_delta_ab_inv_col51;
                let op0_delta_cd_inv_col52 = ((((((op0_limb_8_col43) + (op0_limb_9_col44))
                    + (op0_limb_10_col45))
                    + (op0_limb_11_col46))
                    - (M31_1548))
                    * (((((op0_limb_12_col47) + (op0_limb_13_col48)) + (op0_limb_14_col49))
                        + (op0_limb_15_col50))
                        - (M31_1548)))
                    .inverse();
                *row[52] = op0_delta_cd_inv_col52;

                // Qm 31 Read Reduced.

                // Read Positive Num Bits 144.

                let memory_address_to_id_value_tmp_fa85a_16 = memory_address_to_id_state
                    .deduce_output(((mem1_base_col14) + ((offset2_col5) - (M31_32768))));
                let memory_id_to_big_value_tmp_fa85a_17 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_fa85a_16);
                let op1_id_col53 = memory_address_to_id_value_tmp_fa85a_16;
                *row[53] = op1_id_col53;
                let memory_address_to_id_inputs_2 =
                    ((mem1_base_col14) + ((offset2_col5) - (M31_32768))).unpack();
                *lookup_data.memory_address_to_id_2 = [
                    ((mem1_base_col14) + ((offset2_col5) - (M31_32768))),
                    op1_id_col53,
                ];
                let op1_limb_0_col54 = memory_id_to_big_value_tmp_fa85a_17.get_m31(0);
                *row[54] = op1_limb_0_col54;
                let op1_limb_1_col55 = memory_id_to_big_value_tmp_fa85a_17.get_m31(1);
                *row[55] = op1_limb_1_col55;
                let op1_limb_2_col56 = memory_id_to_big_value_tmp_fa85a_17.get_m31(2);
                *row[56] = op1_limb_2_col56;
                let op1_limb_3_col57 = memory_id_to_big_value_tmp_fa85a_17.get_m31(3);
                *row[57] = op1_limb_3_col57;
                let op1_limb_4_col58 = memory_id_to_big_value_tmp_fa85a_17.get_m31(4);
                *row[58] = op1_limb_4_col58;
                let op1_limb_5_col59 = memory_id_to_big_value_tmp_fa85a_17.get_m31(5);
                *row[59] = op1_limb_5_col59;
                let op1_limb_6_col60 = memory_id_to_big_value_tmp_fa85a_17.get_m31(6);
                *row[60] = op1_limb_6_col60;
                let op1_limb_7_col61 = memory_id_to_big_value_tmp_fa85a_17.get_m31(7);
                *row[61] = op1_limb_7_col61;
                let op1_limb_8_col62 = memory_id_to_big_value_tmp_fa85a_17.get_m31(8);
                *row[62] = op1_limb_8_col62;
                let op1_limb_9_col63 = memory_id_to_big_value_tmp_fa85a_17.get_m31(9);
                *row[63] = op1_limb_9_col63;
                let op1_limb_10_col64 = memory_id_to_big_value_tmp_fa85a_17.get_m31(10);
                *row[64] = op1_limb_10_col64;
                let op1_limb_11_col65 = memory_id_to_big_value_tmp_fa85a_17.get_m31(11);
                *row[65] = op1_limb_11_col65;
                let op1_limb_12_col66 = memory_id_to_big_value_tmp_fa85a_17.get_m31(12);
                *row[66] = op1_limb_12_col66;
                let op1_limb_13_col67 = memory_id_to_big_value_tmp_fa85a_17.get_m31(13);
                *row[67] = op1_limb_13_col67;
                let op1_limb_14_col68 = memory_id_to_big_value_tmp_fa85a_17.get_m31(14);
                *row[68] = op1_limb_14_col68;
                let op1_limb_15_col69 = memory_id_to_big_value_tmp_fa85a_17.get_m31(15);
                *row[69] = op1_limb_15_col69;
                let memory_id_to_big_inputs_2 = op1_id_col53.unpack();
                *lookup_data.memory_id_to_big_2 = [
                    op1_id_col53,
                    op1_limb_0_col54,
                    op1_limb_1_col55,
                    op1_limb_2_col56,
                    op1_limb_3_col57,
                    op1_limb_4_col58,
                    op1_limb_5_col59,
                    op1_limb_6_col60,
                    op1_limb_7_col61,
                    op1_limb_8_col62,
                    op1_limb_9_col63,
                    op1_limb_10_col64,
                    op1_limb_11_col65,
                    op1_limb_12_col66,
                    op1_limb_13_col67,
                    op1_limb_14_col68,
                    op1_limb_15_col69,
                    M31_0,
                    M31_0,
                    M31_0,
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

                let range_check_4_4_4_4_inputs_2 = [
                    op1_limb_3_col57,
                    op1_limb_7_col61,
                    op1_limb_11_col65,
                    op1_limb_15_col69,
                ]
                .unpack();
                *lookup_data.range_check_4_4_4_4_2 = [
                    op1_limb_3_col57,
                    op1_limb_7_col61,
                    op1_limb_11_col65,
                    op1_limb_15_col69,
                ];
                let op1_delta_ab_inv_col70 = ((((((op1_limb_0_col54) + (op1_limb_1_col55))
                    + (op1_limb_2_col56))
                    + (op1_limb_3_col57))
                    - (M31_1548))
                    * (((((op1_limb_4_col58) + (op1_limb_5_col59)) + (op1_limb_6_col60))
                        + (op1_limb_7_col61))
                        - (M31_1548)))
                    .inverse();
                *row[70] = op1_delta_ab_inv_col70;
                let op1_delta_cd_inv_col71 = ((((((op1_limb_8_col62) + (op1_limb_9_col63))
                    + (op1_limb_10_col64))
                    + (op1_limb_11_col65))
                    - (M31_1548))
                    * (((((op1_limb_12_col66) + (op1_limb_13_col67)) + (op1_limb_14_col68))
                        + (op1_limb_15_col69))
                        - (M31_1548)))
                    .inverse();
                *row[71] = op1_delta_cd_inv_col71;

                *lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];
                *lookup_data.opcodes_1 = [
                    (((input_pc_col0) + (M31_1)) + (op1_imm_col8)),
                    ((input_ap_col1) + (ap_update_add_1_col11)),
                    input_fp_col2,
                ];
                *row[72] = padding_col.packed_at(row_index);

                // Add sub-components inputs.
                verify_instruction_state.add_inputs(&verify_instruction_inputs_0);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_0);
                memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_0);
                range_check_4_4_4_4_state.add_inputs(&range_check_4_4_4_4_inputs_0);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_1);
                memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_1);
                range_check_4_4_4_4_state.add_inputs(&range_check_4_4_4_4_inputs_1);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_2);
                memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_2);
                range_check_4_4_4_4_state.add_inputs(&range_check_4_4_4_4_inputs_2);
            },
        );

    (trace, lookup_data)
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
    range_check_4_4_4_4_0: Vec<[PackedM31; 4]>,
    range_check_4_4_4_4_1: Vec<[PackedM31; 4]>,
    range_check_4_4_4_4_2: Vec<[PackedM31; 4]>,
    verify_instruction_0: Vec<[PackedM31; 7]>,
}

pub struct InteractionClaimGenerator {
    n_rows: usize,
    log_size: u32,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        memory_address_to_id: &relations::MemoryAddressToId,
        memory_id_to_big: &relations::MemoryIdToBig,
        opcodes: &relations::Opcodes,
        range_check_4_4_4_4: &relations::RangeCheck_4_4_4_4,
        verify_instruction: &relations::VerifyInstruction,
    ) -> InteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
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
            &self.lookup_data.range_check_4_4_4_4_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = range_check_4_4_4_4.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_address_to_id_1,
            &self.lookup_data.memory_id_to_big_1,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_address_to_id.combine(values0);
            let denom1: PackedQM31 = memory_id_to_big.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_4_4_4_4_1,
            &self.lookup_data.memory_address_to_id_2,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_4_4_4_4.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_id_to_big_2,
            &self.lookup_data.range_check_4_4_4_4_2,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = range_check_4_4_4_4.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in
            zip(&self.lookup_data.opcodes_0, &self.lookup_data.opcodes_1).enumerate()
        {
            let denom0: PackedQM31 = opcodes.combine(values0);
            let denom1: PackedQM31 = opcodes.combine(values1);
            col_gen.write_frac(
                i,
                denom1 * padding_col.packed_at(i) - denom0 * padding_col.packed_at(i),
                denom0 * denom1,
            );
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
