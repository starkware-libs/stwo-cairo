#![allow(unused_parens)]
use super::component::{Claim, InteractionClaim};
use crate::components::prelude::proving::*;
use crate::components::{
    memory_address_to_id, memory_id_to_big, range_check_19, verify_instruction,
};

pub type InputType = CasmState;
pub type PackedInputType = PackedCasmState;
const N_TRACE_COLUMNS: usize = 130;

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
        range_check_19_state: &range_check_19::ClaimGenerator,
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
            range_check_19_state,
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
    range_check_19_state: &range_check_19::ClaimGenerator,
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

    trace
        .par_iter_mut()
        .enumerate()
        .zip(inputs.into_par_iter())
        .zip(lookup_data.par_iter_mut())
        .for_each(|(((row_index, mut row), mul_opcode_input), lookup_data)| {
            let input_tmp_42314_0 = mul_opcode_input;
            let input_pc_col0 = input_tmp_42314_0.pc;
            *row[0] = input_pc_col0;
            let input_ap_col1 = input_tmp_42314_0.ap;
            *row[1] = input_ap_col1;
            let input_fp_col2 = input_tmp_42314_0.fp;
            *row[2] = input_fp_col2;

            // Decode Instruction.

            let memory_address_to_id_value_tmp_42314_1 =
                memory_address_to_id_state.deduce_output(input_pc_col0);
            let memory_id_to_big_value_tmp_42314_2 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_42314_1);
            let offset0_tmp_42314_3 =
                ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_2.get_m31(0)))
                    + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_2.get_m31(1)))
                        & (UInt16_127))
                        << (UInt16_9)));
            let offset0_col3 = offset0_tmp_42314_3.as_m31();
            *row[3] = offset0_col3;
            let offset1_tmp_42314_4 =
                ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_2.get_m31(1)))
                    >> (UInt16_7))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_2.get_m31(2)))
                        << (UInt16_2)))
                    + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_2.get_m31(3)))
                        & (UInt16_31))
                        << (UInt16_11)));
            let offset1_col4 = offset1_tmp_42314_4.as_m31();
            *row[4] = offset1_col4;
            let offset2_tmp_42314_5 =
                ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_2.get_m31(3)))
                    >> (UInt16_5))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_2.get_m31(4)))
                        << (UInt16_4)))
                    + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_2.get_m31(5)))
                        & (UInt16_7))
                        << (UInt16_13)));
            let offset2_col5 = offset2_tmp_42314_5.as_m31();
            *row[5] = offset2_col5;
            let dst_base_fp_tmp_42314_6 =
                (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_0))
                    & (UInt16_1));
            let dst_base_fp_col6 = dst_base_fp_tmp_42314_6.as_m31();
            *row[6] = dst_base_fp_col6;
            let op0_base_fp_tmp_42314_7 =
                (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_1))
                    & (UInt16_1));
            let op0_base_fp_col7 = op0_base_fp_tmp_42314_7.as_m31();
            *row[7] = op0_base_fp_col7;
            let op1_base_fp_tmp_42314_8 =
                (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_3))
                    & (UInt16_1));
            let op1_base_fp_col8 = op1_base_fp_tmp_42314_8.as_m31();
            *row[8] = op1_base_fp_col8;
            let op1_base_ap_tmp_42314_9 =
                (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_4))
                    & (UInt16_1));
            let op1_base_ap_col9 = op1_base_ap_tmp_42314_9.as_m31();
            *row[9] = op1_base_ap_col9;
            let ap_update_add_1_tmp_42314_10 =
                (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_42314_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_11))
                    & (UInt16_1));
            let ap_update_add_1_col10 = ap_update_add_1_tmp_42314_10.as_m31();
            *row[10] = ap_update_add_1_col10;
            let verify_instruction_inputs_0 = (
                input_pc_col0,
                [offset0_col3, offset1_col4, offset2_col5],
                [
                    (((((((M31_0) + ((dst_base_fp_col6) * (M31_8)))
                        + ((op0_base_fp_col7) * (M31_16)))
                        + (M31_0))
                        + ((op1_base_fp_col8) * (M31_64)))
                        + ((op1_base_ap_col9) * (M31_128)))
                        + (M31_0)),
                    (((((M31_1) + ((ap_update_add_1_col10) * (M31_32))) + (M31_0)) + (M31_0))
                        + (M31_256)),
                ],
                M31_0,
            )
                .unpack();
            *lookup_data.verify_instruction_0 = [
                input_pc_col0,
                offset0_col3,
                offset1_col4,
                offset2_col5,
                (((((((M31_0) + ((dst_base_fp_col6) * (M31_8)))
                    + ((op0_base_fp_col7) * (M31_16)))
                    + (M31_0))
                    + ((op1_base_fp_col8) * (M31_64)))
                    + ((op1_base_ap_col9) * (M31_128)))
                    + (M31_0)),
                (((((M31_1) + ((ap_update_add_1_col10) * (M31_32))) + (M31_0)) + (M31_0))
                    + (M31_256)),
                M31_0,
            ];

            let mem_dst_base_col11 = (((dst_base_fp_col6) * (input_fp_col2))
                + (((M31_1) - (dst_base_fp_col6)) * (input_ap_col1)));
            *row[11] = mem_dst_base_col11;
            let mem0_base_col12 = (((op0_base_fp_col7) * (input_fp_col2))
                + (((M31_1) - (op0_base_fp_col7)) * (input_ap_col1)));
            *row[12] = mem0_base_col12;
            let mem1_base_col13 =
                (((op1_base_fp_col8) * (input_fp_col2)) + ((op1_base_ap_col9) * (input_ap_col1)));
            *row[13] = mem1_base_col13;

            // Read Positive Num Bits 252.

            let memory_address_to_id_value_tmp_42314_11 = memory_address_to_id_state
                .deduce_output(((mem_dst_base_col11) + ((offset0_col3) - (M31_32768))));
            let memory_id_to_big_value_tmp_42314_12 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_42314_11);
            let dst_id_col14 = memory_address_to_id_value_tmp_42314_11;
            *row[14] = dst_id_col14;
            let memory_address_to_id_inputs_0 =
                ((mem_dst_base_col11) + ((offset0_col3) - (M31_32768))).unpack();
            *lookup_data.memory_address_to_id_0 = [
                ((mem_dst_base_col11) + ((offset0_col3) - (M31_32768))),
                dst_id_col14,
            ];
            let dst_limb_0_col15 = memory_id_to_big_value_tmp_42314_12.get_m31(0);
            *row[15] = dst_limb_0_col15;
            let dst_limb_1_col16 = memory_id_to_big_value_tmp_42314_12.get_m31(1);
            *row[16] = dst_limb_1_col16;
            let dst_limb_2_col17 = memory_id_to_big_value_tmp_42314_12.get_m31(2);
            *row[17] = dst_limb_2_col17;
            let dst_limb_3_col18 = memory_id_to_big_value_tmp_42314_12.get_m31(3);
            *row[18] = dst_limb_3_col18;
            let dst_limb_4_col19 = memory_id_to_big_value_tmp_42314_12.get_m31(4);
            *row[19] = dst_limb_4_col19;
            let dst_limb_5_col20 = memory_id_to_big_value_tmp_42314_12.get_m31(5);
            *row[20] = dst_limb_5_col20;
            let dst_limb_6_col21 = memory_id_to_big_value_tmp_42314_12.get_m31(6);
            *row[21] = dst_limb_6_col21;
            let dst_limb_7_col22 = memory_id_to_big_value_tmp_42314_12.get_m31(7);
            *row[22] = dst_limb_7_col22;
            let dst_limb_8_col23 = memory_id_to_big_value_tmp_42314_12.get_m31(8);
            *row[23] = dst_limb_8_col23;
            let dst_limb_9_col24 = memory_id_to_big_value_tmp_42314_12.get_m31(9);
            *row[24] = dst_limb_9_col24;
            let dst_limb_10_col25 = memory_id_to_big_value_tmp_42314_12.get_m31(10);
            *row[25] = dst_limb_10_col25;
            let dst_limb_11_col26 = memory_id_to_big_value_tmp_42314_12.get_m31(11);
            *row[26] = dst_limb_11_col26;
            let dst_limb_12_col27 = memory_id_to_big_value_tmp_42314_12.get_m31(12);
            *row[27] = dst_limb_12_col27;
            let dst_limb_13_col28 = memory_id_to_big_value_tmp_42314_12.get_m31(13);
            *row[28] = dst_limb_13_col28;
            let dst_limb_14_col29 = memory_id_to_big_value_tmp_42314_12.get_m31(14);
            *row[29] = dst_limb_14_col29;
            let dst_limb_15_col30 = memory_id_to_big_value_tmp_42314_12.get_m31(15);
            *row[30] = dst_limb_15_col30;
            let dst_limb_16_col31 = memory_id_to_big_value_tmp_42314_12.get_m31(16);
            *row[31] = dst_limb_16_col31;
            let dst_limb_17_col32 = memory_id_to_big_value_tmp_42314_12.get_m31(17);
            *row[32] = dst_limb_17_col32;
            let dst_limb_18_col33 = memory_id_to_big_value_tmp_42314_12.get_m31(18);
            *row[33] = dst_limb_18_col33;
            let dst_limb_19_col34 = memory_id_to_big_value_tmp_42314_12.get_m31(19);
            *row[34] = dst_limb_19_col34;
            let dst_limb_20_col35 = memory_id_to_big_value_tmp_42314_12.get_m31(20);
            *row[35] = dst_limb_20_col35;
            let dst_limb_21_col36 = memory_id_to_big_value_tmp_42314_12.get_m31(21);
            *row[36] = dst_limb_21_col36;
            let dst_limb_22_col37 = memory_id_to_big_value_tmp_42314_12.get_m31(22);
            *row[37] = dst_limb_22_col37;
            let dst_limb_23_col38 = memory_id_to_big_value_tmp_42314_12.get_m31(23);
            *row[38] = dst_limb_23_col38;
            let dst_limb_24_col39 = memory_id_to_big_value_tmp_42314_12.get_m31(24);
            *row[39] = dst_limb_24_col39;
            let dst_limb_25_col40 = memory_id_to_big_value_tmp_42314_12.get_m31(25);
            *row[40] = dst_limb_25_col40;
            let dst_limb_26_col41 = memory_id_to_big_value_tmp_42314_12.get_m31(26);
            *row[41] = dst_limb_26_col41;
            let dst_limb_27_col42 = memory_id_to_big_value_tmp_42314_12.get_m31(27);
            *row[42] = dst_limb_27_col42;
            let memory_id_to_big_inputs_0 = dst_id_col14.unpack();
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

            // Read Positive Num Bits 252.

            let memory_address_to_id_value_tmp_42314_13 = memory_address_to_id_state
                .deduce_output(((mem0_base_col12) + ((offset1_col4) - (M31_32768))));
            let memory_id_to_big_value_tmp_42314_14 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_42314_13);
            let op0_id_col43 = memory_address_to_id_value_tmp_42314_13;
            *row[43] = op0_id_col43;
            let memory_address_to_id_inputs_1 =
                ((mem0_base_col12) + ((offset1_col4) - (M31_32768))).unpack();
            *lookup_data.memory_address_to_id_1 = [
                ((mem0_base_col12) + ((offset1_col4) - (M31_32768))),
                op0_id_col43,
            ];
            let op0_limb_0_col44 = memory_id_to_big_value_tmp_42314_14.get_m31(0);
            *row[44] = op0_limb_0_col44;
            let op0_limb_1_col45 = memory_id_to_big_value_tmp_42314_14.get_m31(1);
            *row[45] = op0_limb_1_col45;
            let op0_limb_2_col46 = memory_id_to_big_value_tmp_42314_14.get_m31(2);
            *row[46] = op0_limb_2_col46;
            let op0_limb_3_col47 = memory_id_to_big_value_tmp_42314_14.get_m31(3);
            *row[47] = op0_limb_3_col47;
            let op0_limb_4_col48 = memory_id_to_big_value_tmp_42314_14.get_m31(4);
            *row[48] = op0_limb_4_col48;
            let op0_limb_5_col49 = memory_id_to_big_value_tmp_42314_14.get_m31(5);
            *row[49] = op0_limb_5_col49;
            let op0_limb_6_col50 = memory_id_to_big_value_tmp_42314_14.get_m31(6);
            *row[50] = op0_limb_6_col50;
            let op0_limb_7_col51 = memory_id_to_big_value_tmp_42314_14.get_m31(7);
            *row[51] = op0_limb_7_col51;
            let op0_limb_8_col52 = memory_id_to_big_value_tmp_42314_14.get_m31(8);
            *row[52] = op0_limb_8_col52;
            let op0_limb_9_col53 = memory_id_to_big_value_tmp_42314_14.get_m31(9);
            *row[53] = op0_limb_9_col53;
            let op0_limb_10_col54 = memory_id_to_big_value_tmp_42314_14.get_m31(10);
            *row[54] = op0_limb_10_col54;
            let op0_limb_11_col55 = memory_id_to_big_value_tmp_42314_14.get_m31(11);
            *row[55] = op0_limb_11_col55;
            let op0_limb_12_col56 = memory_id_to_big_value_tmp_42314_14.get_m31(12);
            *row[56] = op0_limb_12_col56;
            let op0_limb_13_col57 = memory_id_to_big_value_tmp_42314_14.get_m31(13);
            *row[57] = op0_limb_13_col57;
            let op0_limb_14_col58 = memory_id_to_big_value_tmp_42314_14.get_m31(14);
            *row[58] = op0_limb_14_col58;
            let op0_limb_15_col59 = memory_id_to_big_value_tmp_42314_14.get_m31(15);
            *row[59] = op0_limb_15_col59;
            let op0_limb_16_col60 = memory_id_to_big_value_tmp_42314_14.get_m31(16);
            *row[60] = op0_limb_16_col60;
            let op0_limb_17_col61 = memory_id_to_big_value_tmp_42314_14.get_m31(17);
            *row[61] = op0_limb_17_col61;
            let op0_limb_18_col62 = memory_id_to_big_value_tmp_42314_14.get_m31(18);
            *row[62] = op0_limb_18_col62;
            let op0_limb_19_col63 = memory_id_to_big_value_tmp_42314_14.get_m31(19);
            *row[63] = op0_limb_19_col63;
            let op0_limb_20_col64 = memory_id_to_big_value_tmp_42314_14.get_m31(20);
            *row[64] = op0_limb_20_col64;
            let op0_limb_21_col65 = memory_id_to_big_value_tmp_42314_14.get_m31(21);
            *row[65] = op0_limb_21_col65;
            let op0_limb_22_col66 = memory_id_to_big_value_tmp_42314_14.get_m31(22);
            *row[66] = op0_limb_22_col66;
            let op0_limb_23_col67 = memory_id_to_big_value_tmp_42314_14.get_m31(23);
            *row[67] = op0_limb_23_col67;
            let op0_limb_24_col68 = memory_id_to_big_value_tmp_42314_14.get_m31(24);
            *row[68] = op0_limb_24_col68;
            let op0_limb_25_col69 = memory_id_to_big_value_tmp_42314_14.get_m31(25);
            *row[69] = op0_limb_25_col69;
            let op0_limb_26_col70 = memory_id_to_big_value_tmp_42314_14.get_m31(26);
            *row[70] = op0_limb_26_col70;
            let op0_limb_27_col71 = memory_id_to_big_value_tmp_42314_14.get_m31(27);
            *row[71] = op0_limb_27_col71;
            let memory_id_to_big_inputs_1 = op0_id_col43.unpack();
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

            // Read Positive Num Bits 252.

            let memory_address_to_id_value_tmp_42314_15 = memory_address_to_id_state
                .deduce_output(((mem1_base_col13) + ((offset2_col5) - (M31_32768))));
            let memory_id_to_big_value_tmp_42314_16 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_42314_15);
            let op1_id_col72 = memory_address_to_id_value_tmp_42314_15;
            *row[72] = op1_id_col72;
            let memory_address_to_id_inputs_2 =
                ((mem1_base_col13) + ((offset2_col5) - (M31_32768))).unpack();
            *lookup_data.memory_address_to_id_2 = [
                ((mem1_base_col13) + ((offset2_col5) - (M31_32768))),
                op1_id_col72,
            ];
            let op1_limb_0_col73 = memory_id_to_big_value_tmp_42314_16.get_m31(0);
            *row[73] = op1_limb_0_col73;
            let op1_limb_1_col74 = memory_id_to_big_value_tmp_42314_16.get_m31(1);
            *row[74] = op1_limb_1_col74;
            let op1_limb_2_col75 = memory_id_to_big_value_tmp_42314_16.get_m31(2);
            *row[75] = op1_limb_2_col75;
            let op1_limb_3_col76 = memory_id_to_big_value_tmp_42314_16.get_m31(3);
            *row[76] = op1_limb_3_col76;
            let op1_limb_4_col77 = memory_id_to_big_value_tmp_42314_16.get_m31(4);
            *row[77] = op1_limb_4_col77;
            let op1_limb_5_col78 = memory_id_to_big_value_tmp_42314_16.get_m31(5);
            *row[78] = op1_limb_5_col78;
            let op1_limb_6_col79 = memory_id_to_big_value_tmp_42314_16.get_m31(6);
            *row[79] = op1_limb_6_col79;
            let op1_limb_7_col80 = memory_id_to_big_value_tmp_42314_16.get_m31(7);
            *row[80] = op1_limb_7_col80;
            let op1_limb_8_col81 = memory_id_to_big_value_tmp_42314_16.get_m31(8);
            *row[81] = op1_limb_8_col81;
            let op1_limb_9_col82 = memory_id_to_big_value_tmp_42314_16.get_m31(9);
            *row[82] = op1_limb_9_col82;
            let op1_limb_10_col83 = memory_id_to_big_value_tmp_42314_16.get_m31(10);
            *row[83] = op1_limb_10_col83;
            let op1_limb_11_col84 = memory_id_to_big_value_tmp_42314_16.get_m31(11);
            *row[84] = op1_limb_11_col84;
            let op1_limb_12_col85 = memory_id_to_big_value_tmp_42314_16.get_m31(12);
            *row[85] = op1_limb_12_col85;
            let op1_limb_13_col86 = memory_id_to_big_value_tmp_42314_16.get_m31(13);
            *row[86] = op1_limb_13_col86;
            let op1_limb_14_col87 = memory_id_to_big_value_tmp_42314_16.get_m31(14);
            *row[87] = op1_limb_14_col87;
            let op1_limb_15_col88 = memory_id_to_big_value_tmp_42314_16.get_m31(15);
            *row[88] = op1_limb_15_col88;
            let op1_limb_16_col89 = memory_id_to_big_value_tmp_42314_16.get_m31(16);
            *row[89] = op1_limb_16_col89;
            let op1_limb_17_col90 = memory_id_to_big_value_tmp_42314_16.get_m31(17);
            *row[90] = op1_limb_17_col90;
            let op1_limb_18_col91 = memory_id_to_big_value_tmp_42314_16.get_m31(18);
            *row[91] = op1_limb_18_col91;
            let op1_limb_19_col92 = memory_id_to_big_value_tmp_42314_16.get_m31(19);
            *row[92] = op1_limb_19_col92;
            let op1_limb_20_col93 = memory_id_to_big_value_tmp_42314_16.get_m31(20);
            *row[93] = op1_limb_20_col93;
            let op1_limb_21_col94 = memory_id_to_big_value_tmp_42314_16.get_m31(21);
            *row[94] = op1_limb_21_col94;
            let op1_limb_22_col95 = memory_id_to_big_value_tmp_42314_16.get_m31(22);
            *row[95] = op1_limb_22_col95;
            let op1_limb_23_col96 = memory_id_to_big_value_tmp_42314_16.get_m31(23);
            *row[96] = op1_limb_23_col96;
            let op1_limb_24_col97 = memory_id_to_big_value_tmp_42314_16.get_m31(24);
            *row[97] = op1_limb_24_col97;
            let op1_limb_25_col98 = memory_id_to_big_value_tmp_42314_16.get_m31(25);
            *row[98] = op1_limb_25_col98;
            let op1_limb_26_col99 = memory_id_to_big_value_tmp_42314_16.get_m31(26);
            *row[99] = op1_limb_26_col99;
            let op1_limb_27_col100 = memory_id_to_big_value_tmp_42314_16.get_m31(27);
            *row[100] = op1_limb_27_col100;
            let memory_id_to_big_inputs_2 = op1_id_col72.unpack();
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

            // Verify Mul 252.

            let conv_tmp_42314_17 =
                (((M31_0) - (dst_limb_0_col15)) + ((op0_limb_0_col44) * (op1_limb_0_col73)));
            let conv_tmp_42314_18 = ((((M31_0) - (dst_limb_1_col16))
                + ((op0_limb_0_col44) * (op1_limb_1_col74)))
                + ((op0_limb_1_col45) * (op1_limb_0_col73)));
            let conv_tmp_42314_19 = (((((M31_0) - (dst_limb_2_col17))
                + ((op0_limb_0_col44) * (op1_limb_2_col75)))
                + ((op0_limb_1_col45) * (op1_limb_1_col74)))
                + ((op0_limb_2_col46) * (op1_limb_0_col73)));
            let conv_tmp_42314_20 = ((((((M31_0) - (dst_limb_3_col18))
                + ((op0_limb_0_col44) * (op1_limb_3_col76)))
                + ((op0_limb_1_col45) * (op1_limb_2_col75)))
                + ((op0_limb_2_col46) * (op1_limb_1_col74)))
                + ((op0_limb_3_col47) * (op1_limb_0_col73)));
            let conv_tmp_42314_21 = (((((((M31_0) - (dst_limb_4_col19))
                + ((op0_limb_0_col44) * (op1_limb_4_col77)))
                + ((op0_limb_1_col45) * (op1_limb_3_col76)))
                + ((op0_limb_2_col46) * (op1_limb_2_col75)))
                + ((op0_limb_3_col47) * (op1_limb_1_col74)))
                + ((op0_limb_4_col48) * (op1_limb_0_col73)));
            let conv_tmp_42314_22 = ((((((((M31_0) - (dst_limb_5_col20))
                + ((op0_limb_0_col44) * (op1_limb_5_col78)))
                + ((op0_limb_1_col45) * (op1_limb_4_col77)))
                + ((op0_limb_2_col46) * (op1_limb_3_col76)))
                + ((op0_limb_3_col47) * (op1_limb_2_col75)))
                + ((op0_limb_4_col48) * (op1_limb_1_col74)))
                + ((op0_limb_5_col49) * (op1_limb_0_col73)));
            let conv_tmp_42314_23 = (((((((((M31_0) - (dst_limb_6_col21))
                + ((op0_limb_0_col44) * (op1_limb_6_col79)))
                + ((op0_limb_1_col45) * (op1_limb_5_col78)))
                + ((op0_limb_2_col46) * (op1_limb_4_col77)))
                + ((op0_limb_3_col47) * (op1_limb_3_col76)))
                + ((op0_limb_4_col48) * (op1_limb_2_col75)))
                + ((op0_limb_5_col49) * (op1_limb_1_col74)))
                + ((op0_limb_6_col50) * (op1_limb_0_col73)));
            let conv_tmp_42314_24 = ((((((((((M31_0) - (dst_limb_7_col22))
                + ((op0_limb_0_col44) * (op1_limb_7_col80)))
                + ((op0_limb_1_col45) * (op1_limb_6_col79)))
                + ((op0_limb_2_col46) * (op1_limb_5_col78)))
                + ((op0_limb_3_col47) * (op1_limb_4_col77)))
                + ((op0_limb_4_col48) * (op1_limb_3_col76)))
                + ((op0_limb_5_col49) * (op1_limb_2_col75)))
                + ((op0_limb_6_col50) * (op1_limb_1_col74)))
                + ((op0_limb_7_col51) * (op1_limb_0_col73)));
            let conv_tmp_42314_25 = (((((((((((M31_0) - (dst_limb_8_col23))
                + ((op0_limb_0_col44) * (op1_limb_8_col81)))
                + ((op0_limb_1_col45) * (op1_limb_7_col80)))
                + ((op0_limb_2_col46) * (op1_limb_6_col79)))
                + ((op0_limb_3_col47) * (op1_limb_5_col78)))
                + ((op0_limb_4_col48) * (op1_limb_4_col77)))
                + ((op0_limb_5_col49) * (op1_limb_3_col76)))
                + ((op0_limb_6_col50) * (op1_limb_2_col75)))
                + ((op0_limb_7_col51) * (op1_limb_1_col74)))
                + ((op0_limb_8_col52) * (op1_limb_0_col73)));
            let conv_tmp_42314_26 = ((((((((((((M31_0) - (dst_limb_9_col24))
                + ((op0_limb_0_col44) * (op1_limb_9_col82)))
                + ((op0_limb_1_col45) * (op1_limb_8_col81)))
                + ((op0_limb_2_col46) * (op1_limb_7_col80)))
                + ((op0_limb_3_col47) * (op1_limb_6_col79)))
                + ((op0_limb_4_col48) * (op1_limb_5_col78)))
                + ((op0_limb_5_col49) * (op1_limb_4_col77)))
                + ((op0_limb_6_col50) * (op1_limb_3_col76)))
                + ((op0_limb_7_col51) * (op1_limb_2_col75)))
                + ((op0_limb_8_col52) * (op1_limb_1_col74)))
                + ((op0_limb_9_col53) * (op1_limb_0_col73)));
            let conv_tmp_42314_27 = (((((((((((((M31_0) - (dst_limb_10_col25))
                + ((op0_limb_0_col44) * (op1_limb_10_col83)))
                + ((op0_limb_1_col45) * (op1_limb_9_col82)))
                + ((op0_limb_2_col46) * (op1_limb_8_col81)))
                + ((op0_limb_3_col47) * (op1_limb_7_col80)))
                + ((op0_limb_4_col48) * (op1_limb_6_col79)))
                + ((op0_limb_5_col49) * (op1_limb_5_col78)))
                + ((op0_limb_6_col50) * (op1_limb_4_col77)))
                + ((op0_limb_7_col51) * (op1_limb_3_col76)))
                + ((op0_limb_8_col52) * (op1_limb_2_col75)))
                + ((op0_limb_9_col53) * (op1_limb_1_col74)))
                + ((op0_limb_10_col54) * (op1_limb_0_col73)));
            let conv_tmp_42314_28 = ((((((((((((((M31_0) - (dst_limb_11_col26))
                + ((op0_limb_0_col44) * (op1_limb_11_col84)))
                + ((op0_limb_1_col45) * (op1_limb_10_col83)))
                + ((op0_limb_2_col46) * (op1_limb_9_col82)))
                + ((op0_limb_3_col47) * (op1_limb_8_col81)))
                + ((op0_limb_4_col48) * (op1_limb_7_col80)))
                + ((op0_limb_5_col49) * (op1_limb_6_col79)))
                + ((op0_limb_6_col50) * (op1_limb_5_col78)))
                + ((op0_limb_7_col51) * (op1_limb_4_col77)))
                + ((op0_limb_8_col52) * (op1_limb_3_col76)))
                + ((op0_limb_9_col53) * (op1_limb_2_col75)))
                + ((op0_limb_10_col54) * (op1_limb_1_col74)))
                + ((op0_limb_11_col55) * (op1_limb_0_col73)));
            let conv_tmp_42314_29 = (((((((((((((((M31_0) - (dst_limb_12_col27))
                + ((op0_limb_0_col44) * (op1_limb_12_col85)))
                + ((op0_limb_1_col45) * (op1_limb_11_col84)))
                + ((op0_limb_2_col46) * (op1_limb_10_col83)))
                + ((op0_limb_3_col47) * (op1_limb_9_col82)))
                + ((op0_limb_4_col48) * (op1_limb_8_col81)))
                + ((op0_limb_5_col49) * (op1_limb_7_col80)))
                + ((op0_limb_6_col50) * (op1_limb_6_col79)))
                + ((op0_limb_7_col51) * (op1_limb_5_col78)))
                + ((op0_limb_8_col52) * (op1_limb_4_col77)))
                + ((op0_limb_9_col53) * (op1_limb_3_col76)))
                + ((op0_limb_10_col54) * (op1_limb_2_col75)))
                + ((op0_limb_11_col55) * (op1_limb_1_col74)))
                + ((op0_limb_12_col56) * (op1_limb_0_col73)));
            let conv_tmp_42314_30 = ((((((((((((((((M31_0) - (dst_limb_13_col28))
                + ((op0_limb_0_col44) * (op1_limb_13_col86)))
                + ((op0_limb_1_col45) * (op1_limb_12_col85)))
                + ((op0_limb_2_col46) * (op1_limb_11_col84)))
                + ((op0_limb_3_col47) * (op1_limb_10_col83)))
                + ((op0_limb_4_col48) * (op1_limb_9_col82)))
                + ((op0_limb_5_col49) * (op1_limb_8_col81)))
                + ((op0_limb_6_col50) * (op1_limb_7_col80)))
                + ((op0_limb_7_col51) * (op1_limb_6_col79)))
                + ((op0_limb_8_col52) * (op1_limb_5_col78)))
                + ((op0_limb_9_col53) * (op1_limb_4_col77)))
                + ((op0_limb_10_col54) * (op1_limb_3_col76)))
                + ((op0_limb_11_col55) * (op1_limb_2_col75)))
                + ((op0_limb_12_col56) * (op1_limb_1_col74)))
                + ((op0_limb_13_col57) * (op1_limb_0_col73)));
            let conv_tmp_42314_31 = (((((((((((((((((M31_0) - (dst_limb_14_col29))
                + ((op0_limb_0_col44) * (op1_limb_14_col87)))
                + ((op0_limb_1_col45) * (op1_limb_13_col86)))
                + ((op0_limb_2_col46) * (op1_limb_12_col85)))
                + ((op0_limb_3_col47) * (op1_limb_11_col84)))
                + ((op0_limb_4_col48) * (op1_limb_10_col83)))
                + ((op0_limb_5_col49) * (op1_limb_9_col82)))
                + ((op0_limb_6_col50) * (op1_limb_8_col81)))
                + ((op0_limb_7_col51) * (op1_limb_7_col80)))
                + ((op0_limb_8_col52) * (op1_limb_6_col79)))
                + ((op0_limb_9_col53) * (op1_limb_5_col78)))
                + ((op0_limb_10_col54) * (op1_limb_4_col77)))
                + ((op0_limb_11_col55) * (op1_limb_3_col76)))
                + ((op0_limb_12_col56) * (op1_limb_2_col75)))
                + ((op0_limb_13_col57) * (op1_limb_1_col74)))
                + ((op0_limb_14_col58) * (op1_limb_0_col73)));
            let conv_tmp_42314_32 = ((((((((((((((((((M31_0) - (dst_limb_15_col30))
                + ((op0_limb_0_col44) * (op1_limb_15_col88)))
                + ((op0_limb_1_col45) * (op1_limb_14_col87)))
                + ((op0_limb_2_col46) * (op1_limb_13_col86)))
                + ((op0_limb_3_col47) * (op1_limb_12_col85)))
                + ((op0_limb_4_col48) * (op1_limb_11_col84)))
                + ((op0_limb_5_col49) * (op1_limb_10_col83)))
                + ((op0_limb_6_col50) * (op1_limb_9_col82)))
                + ((op0_limb_7_col51) * (op1_limb_8_col81)))
                + ((op0_limb_8_col52) * (op1_limb_7_col80)))
                + ((op0_limb_9_col53) * (op1_limb_6_col79)))
                + ((op0_limb_10_col54) * (op1_limb_5_col78)))
                + ((op0_limb_11_col55) * (op1_limb_4_col77)))
                + ((op0_limb_12_col56) * (op1_limb_3_col76)))
                + ((op0_limb_13_col57) * (op1_limb_2_col75)))
                + ((op0_limb_14_col58) * (op1_limb_1_col74)))
                + ((op0_limb_15_col59) * (op1_limb_0_col73)));
            let conv_tmp_42314_33 = (((((((((((((((((((M31_0)
                - (dst_limb_16_col31))
                + ((op0_limb_0_col44) * (op1_limb_16_col89)))
                + ((op0_limb_1_col45) * (op1_limb_15_col88)))
                + ((op0_limb_2_col46) * (op1_limb_14_col87)))
                + ((op0_limb_3_col47) * (op1_limb_13_col86)))
                + ((op0_limb_4_col48) * (op1_limb_12_col85)))
                + ((op0_limb_5_col49) * (op1_limb_11_col84)))
                + ((op0_limb_6_col50) * (op1_limb_10_col83)))
                + ((op0_limb_7_col51) * (op1_limb_9_col82)))
                + ((op0_limb_8_col52) * (op1_limb_8_col81)))
                + ((op0_limb_9_col53) * (op1_limb_7_col80)))
                + ((op0_limb_10_col54) * (op1_limb_6_col79)))
                + ((op0_limb_11_col55) * (op1_limb_5_col78)))
                + ((op0_limb_12_col56) * (op1_limb_4_col77)))
                + ((op0_limb_13_col57) * (op1_limb_3_col76)))
                + ((op0_limb_14_col58) * (op1_limb_2_col75)))
                + ((op0_limb_15_col59) * (op1_limb_1_col74)))
                + ((op0_limb_16_col60) * (op1_limb_0_col73)));
            let conv_tmp_42314_34 = ((((((((((((((((((((M31_0)
                - (dst_limb_17_col32))
                + ((op0_limb_0_col44) * (op1_limb_17_col90)))
                + ((op0_limb_1_col45) * (op1_limb_16_col89)))
                + ((op0_limb_2_col46) * (op1_limb_15_col88)))
                + ((op0_limb_3_col47) * (op1_limb_14_col87)))
                + ((op0_limb_4_col48) * (op1_limb_13_col86)))
                + ((op0_limb_5_col49) * (op1_limb_12_col85)))
                + ((op0_limb_6_col50) * (op1_limb_11_col84)))
                + ((op0_limb_7_col51) * (op1_limb_10_col83)))
                + ((op0_limb_8_col52) * (op1_limb_9_col82)))
                + ((op0_limb_9_col53) * (op1_limb_8_col81)))
                + ((op0_limb_10_col54) * (op1_limb_7_col80)))
                + ((op0_limb_11_col55) * (op1_limb_6_col79)))
                + ((op0_limb_12_col56) * (op1_limb_5_col78)))
                + ((op0_limb_13_col57) * (op1_limb_4_col77)))
                + ((op0_limb_14_col58) * (op1_limb_3_col76)))
                + ((op0_limb_15_col59) * (op1_limb_2_col75)))
                + ((op0_limb_16_col60) * (op1_limb_1_col74)))
                + ((op0_limb_17_col61) * (op1_limb_0_col73)));
            let conv_tmp_42314_35 = (((((((((((((((((((((M31_0)
                - (dst_limb_18_col33))
                + ((op0_limb_0_col44) * (op1_limb_18_col91)))
                + ((op0_limb_1_col45) * (op1_limb_17_col90)))
                + ((op0_limb_2_col46) * (op1_limb_16_col89)))
                + ((op0_limb_3_col47) * (op1_limb_15_col88)))
                + ((op0_limb_4_col48) * (op1_limb_14_col87)))
                + ((op0_limb_5_col49) * (op1_limb_13_col86)))
                + ((op0_limb_6_col50) * (op1_limb_12_col85)))
                + ((op0_limb_7_col51) * (op1_limb_11_col84)))
                + ((op0_limb_8_col52) * (op1_limb_10_col83)))
                + ((op0_limb_9_col53) * (op1_limb_9_col82)))
                + ((op0_limb_10_col54) * (op1_limb_8_col81)))
                + ((op0_limb_11_col55) * (op1_limb_7_col80)))
                + ((op0_limb_12_col56) * (op1_limb_6_col79)))
                + ((op0_limb_13_col57) * (op1_limb_5_col78)))
                + ((op0_limb_14_col58) * (op1_limb_4_col77)))
                + ((op0_limb_15_col59) * (op1_limb_3_col76)))
                + ((op0_limb_16_col60) * (op1_limb_2_col75)))
                + ((op0_limb_17_col61) * (op1_limb_1_col74)))
                + ((op0_limb_18_col62) * (op1_limb_0_col73)));
            let conv_tmp_42314_36 = ((((((((((((((((((((((M31_0)
                - (dst_limb_19_col34))
                + ((op0_limb_0_col44) * (op1_limb_19_col92)))
                + ((op0_limb_1_col45) * (op1_limb_18_col91)))
                + ((op0_limb_2_col46) * (op1_limb_17_col90)))
                + ((op0_limb_3_col47) * (op1_limb_16_col89)))
                + ((op0_limb_4_col48) * (op1_limb_15_col88)))
                + ((op0_limb_5_col49) * (op1_limb_14_col87)))
                + ((op0_limb_6_col50) * (op1_limb_13_col86)))
                + ((op0_limb_7_col51) * (op1_limb_12_col85)))
                + ((op0_limb_8_col52) * (op1_limb_11_col84)))
                + ((op0_limb_9_col53) * (op1_limb_10_col83)))
                + ((op0_limb_10_col54) * (op1_limb_9_col82)))
                + ((op0_limb_11_col55) * (op1_limb_8_col81)))
                + ((op0_limb_12_col56) * (op1_limb_7_col80)))
                + ((op0_limb_13_col57) * (op1_limb_6_col79)))
                + ((op0_limb_14_col58) * (op1_limb_5_col78)))
                + ((op0_limb_15_col59) * (op1_limb_4_col77)))
                + ((op0_limb_16_col60) * (op1_limb_3_col76)))
                + ((op0_limb_17_col61) * (op1_limb_2_col75)))
                + ((op0_limb_18_col62) * (op1_limb_1_col74)))
                + ((op0_limb_19_col63) * (op1_limb_0_col73)));
            let conv_tmp_42314_37 = (((((((((((((((((((((((M31_0)
                - (dst_limb_20_col35))
                + ((op0_limb_0_col44) * (op1_limb_20_col93)))
                + ((op0_limb_1_col45) * (op1_limb_19_col92)))
                + ((op0_limb_2_col46) * (op1_limb_18_col91)))
                + ((op0_limb_3_col47) * (op1_limb_17_col90)))
                + ((op0_limb_4_col48) * (op1_limb_16_col89)))
                + ((op0_limb_5_col49) * (op1_limb_15_col88)))
                + ((op0_limb_6_col50) * (op1_limb_14_col87)))
                + ((op0_limb_7_col51) * (op1_limb_13_col86)))
                + ((op0_limb_8_col52) * (op1_limb_12_col85)))
                + ((op0_limb_9_col53) * (op1_limb_11_col84)))
                + ((op0_limb_10_col54) * (op1_limb_10_col83)))
                + ((op0_limb_11_col55) * (op1_limb_9_col82)))
                + ((op0_limb_12_col56) * (op1_limb_8_col81)))
                + ((op0_limb_13_col57) * (op1_limb_7_col80)))
                + ((op0_limb_14_col58) * (op1_limb_6_col79)))
                + ((op0_limb_15_col59) * (op1_limb_5_col78)))
                + ((op0_limb_16_col60) * (op1_limb_4_col77)))
                + ((op0_limb_17_col61) * (op1_limb_3_col76)))
                + ((op0_limb_18_col62) * (op1_limb_2_col75)))
                + ((op0_limb_19_col63) * (op1_limb_1_col74)))
                + ((op0_limb_20_col64) * (op1_limb_0_col73)));
            let conv_tmp_42314_38 = ((((((((((((((((((((((((M31_0)
                - (dst_limb_21_col36))
                + ((op0_limb_0_col44) * (op1_limb_21_col94)))
                + ((op0_limb_1_col45) * (op1_limb_20_col93)))
                + ((op0_limb_2_col46) * (op1_limb_19_col92)))
                + ((op0_limb_3_col47) * (op1_limb_18_col91)))
                + ((op0_limb_4_col48) * (op1_limb_17_col90)))
                + ((op0_limb_5_col49) * (op1_limb_16_col89)))
                + ((op0_limb_6_col50) * (op1_limb_15_col88)))
                + ((op0_limb_7_col51) * (op1_limb_14_col87)))
                + ((op0_limb_8_col52) * (op1_limb_13_col86)))
                + ((op0_limb_9_col53) * (op1_limb_12_col85)))
                + ((op0_limb_10_col54) * (op1_limb_11_col84)))
                + ((op0_limb_11_col55) * (op1_limb_10_col83)))
                + ((op0_limb_12_col56) * (op1_limb_9_col82)))
                + ((op0_limb_13_col57) * (op1_limb_8_col81)))
                + ((op0_limb_14_col58) * (op1_limb_7_col80)))
                + ((op0_limb_15_col59) * (op1_limb_6_col79)))
                + ((op0_limb_16_col60) * (op1_limb_5_col78)))
                + ((op0_limb_17_col61) * (op1_limb_4_col77)))
                + ((op0_limb_18_col62) * (op1_limb_3_col76)))
                + ((op0_limb_19_col63) * (op1_limb_2_col75)))
                + ((op0_limb_20_col64) * (op1_limb_1_col74)))
                + ((op0_limb_21_col65) * (op1_limb_0_col73)));
            let conv_tmp_42314_39 = (((((((((((((((((((((((((M31_0)
                - (dst_limb_22_col37))
                + ((op0_limb_0_col44) * (op1_limb_22_col95)))
                + ((op0_limb_1_col45) * (op1_limb_21_col94)))
                + ((op0_limb_2_col46) * (op1_limb_20_col93)))
                + ((op0_limb_3_col47) * (op1_limb_19_col92)))
                + ((op0_limb_4_col48) * (op1_limb_18_col91)))
                + ((op0_limb_5_col49) * (op1_limb_17_col90)))
                + ((op0_limb_6_col50) * (op1_limb_16_col89)))
                + ((op0_limb_7_col51) * (op1_limb_15_col88)))
                + ((op0_limb_8_col52) * (op1_limb_14_col87)))
                + ((op0_limb_9_col53) * (op1_limb_13_col86)))
                + ((op0_limb_10_col54) * (op1_limb_12_col85)))
                + ((op0_limb_11_col55) * (op1_limb_11_col84)))
                + ((op0_limb_12_col56) * (op1_limb_10_col83)))
                + ((op0_limb_13_col57) * (op1_limb_9_col82)))
                + ((op0_limb_14_col58) * (op1_limb_8_col81)))
                + ((op0_limb_15_col59) * (op1_limb_7_col80)))
                + ((op0_limb_16_col60) * (op1_limb_6_col79)))
                + ((op0_limb_17_col61) * (op1_limb_5_col78)))
                + ((op0_limb_18_col62) * (op1_limb_4_col77)))
                + ((op0_limb_19_col63) * (op1_limb_3_col76)))
                + ((op0_limb_20_col64) * (op1_limb_2_col75)))
                + ((op0_limb_21_col65) * (op1_limb_1_col74)))
                + ((op0_limb_22_col66) * (op1_limb_0_col73)));
            let conv_tmp_42314_40 = ((((((((((((((((((((((((((M31_0)
                - (dst_limb_23_col38))
                + ((op0_limb_0_col44) * (op1_limb_23_col96)))
                + ((op0_limb_1_col45) * (op1_limb_22_col95)))
                + ((op0_limb_2_col46) * (op1_limb_21_col94)))
                + ((op0_limb_3_col47) * (op1_limb_20_col93)))
                + ((op0_limb_4_col48) * (op1_limb_19_col92)))
                + ((op0_limb_5_col49) * (op1_limb_18_col91)))
                + ((op0_limb_6_col50) * (op1_limb_17_col90)))
                + ((op0_limb_7_col51) * (op1_limb_16_col89)))
                + ((op0_limb_8_col52) * (op1_limb_15_col88)))
                + ((op0_limb_9_col53) * (op1_limb_14_col87)))
                + ((op0_limb_10_col54) * (op1_limb_13_col86)))
                + ((op0_limb_11_col55) * (op1_limb_12_col85)))
                + ((op0_limb_12_col56) * (op1_limb_11_col84)))
                + ((op0_limb_13_col57) * (op1_limb_10_col83)))
                + ((op0_limb_14_col58) * (op1_limb_9_col82)))
                + ((op0_limb_15_col59) * (op1_limb_8_col81)))
                + ((op0_limb_16_col60) * (op1_limb_7_col80)))
                + ((op0_limb_17_col61) * (op1_limb_6_col79)))
                + ((op0_limb_18_col62) * (op1_limb_5_col78)))
                + ((op0_limb_19_col63) * (op1_limb_4_col77)))
                + ((op0_limb_20_col64) * (op1_limb_3_col76)))
                + ((op0_limb_21_col65) * (op1_limb_2_col75)))
                + ((op0_limb_22_col66) * (op1_limb_1_col74)))
                + ((op0_limb_23_col67) * (op1_limb_0_col73)));
            let conv_tmp_42314_41 = (((((((((((((((((((((((((((M31_0)
                - (dst_limb_24_col39))
                + ((op0_limb_0_col44) * (op1_limb_24_col97)))
                + ((op0_limb_1_col45) * (op1_limb_23_col96)))
                + ((op0_limb_2_col46) * (op1_limb_22_col95)))
                + ((op0_limb_3_col47) * (op1_limb_21_col94)))
                + ((op0_limb_4_col48) * (op1_limb_20_col93)))
                + ((op0_limb_5_col49) * (op1_limb_19_col92)))
                + ((op0_limb_6_col50) * (op1_limb_18_col91)))
                + ((op0_limb_7_col51) * (op1_limb_17_col90)))
                + ((op0_limb_8_col52) * (op1_limb_16_col89)))
                + ((op0_limb_9_col53) * (op1_limb_15_col88)))
                + ((op0_limb_10_col54) * (op1_limb_14_col87)))
                + ((op0_limb_11_col55) * (op1_limb_13_col86)))
                + ((op0_limb_12_col56) * (op1_limb_12_col85)))
                + ((op0_limb_13_col57) * (op1_limb_11_col84)))
                + ((op0_limb_14_col58) * (op1_limb_10_col83)))
                + ((op0_limb_15_col59) * (op1_limb_9_col82)))
                + ((op0_limb_16_col60) * (op1_limb_8_col81)))
                + ((op0_limb_17_col61) * (op1_limb_7_col80)))
                + ((op0_limb_18_col62) * (op1_limb_6_col79)))
                + ((op0_limb_19_col63) * (op1_limb_5_col78)))
                + ((op0_limb_20_col64) * (op1_limb_4_col77)))
                + ((op0_limb_21_col65) * (op1_limb_3_col76)))
                + ((op0_limb_22_col66) * (op1_limb_2_col75)))
                + ((op0_limb_23_col67) * (op1_limb_1_col74)))
                + ((op0_limb_24_col68) * (op1_limb_0_col73)));
            let conv_tmp_42314_42 = ((((((((((((((((((((((((((((M31_0)
                - (dst_limb_25_col40))
                + ((op0_limb_0_col44) * (op1_limb_25_col98)))
                + ((op0_limb_1_col45) * (op1_limb_24_col97)))
                + ((op0_limb_2_col46) * (op1_limb_23_col96)))
                + ((op0_limb_3_col47) * (op1_limb_22_col95)))
                + ((op0_limb_4_col48) * (op1_limb_21_col94)))
                + ((op0_limb_5_col49) * (op1_limb_20_col93)))
                + ((op0_limb_6_col50) * (op1_limb_19_col92)))
                + ((op0_limb_7_col51) * (op1_limb_18_col91)))
                + ((op0_limb_8_col52) * (op1_limb_17_col90)))
                + ((op0_limb_9_col53) * (op1_limb_16_col89)))
                + ((op0_limb_10_col54) * (op1_limb_15_col88)))
                + ((op0_limb_11_col55) * (op1_limb_14_col87)))
                + ((op0_limb_12_col56) * (op1_limb_13_col86)))
                + ((op0_limb_13_col57) * (op1_limb_12_col85)))
                + ((op0_limb_14_col58) * (op1_limb_11_col84)))
                + ((op0_limb_15_col59) * (op1_limb_10_col83)))
                + ((op0_limb_16_col60) * (op1_limb_9_col82)))
                + ((op0_limb_17_col61) * (op1_limb_8_col81)))
                + ((op0_limb_18_col62) * (op1_limb_7_col80)))
                + ((op0_limb_19_col63) * (op1_limb_6_col79)))
                + ((op0_limb_20_col64) * (op1_limb_5_col78)))
                + ((op0_limb_21_col65) * (op1_limb_4_col77)))
                + ((op0_limb_22_col66) * (op1_limb_3_col76)))
                + ((op0_limb_23_col67) * (op1_limb_2_col75)))
                + ((op0_limb_24_col68) * (op1_limb_1_col74)))
                + ((op0_limb_25_col69) * (op1_limb_0_col73)));
            let conv_tmp_42314_43 = (((((((((((((((((((((((((((((M31_0)
                - (dst_limb_26_col41))
                + ((op0_limb_0_col44) * (op1_limb_26_col99)))
                + ((op0_limb_1_col45) * (op1_limb_25_col98)))
                + ((op0_limb_2_col46) * (op1_limb_24_col97)))
                + ((op0_limb_3_col47) * (op1_limb_23_col96)))
                + ((op0_limb_4_col48) * (op1_limb_22_col95)))
                + ((op0_limb_5_col49) * (op1_limb_21_col94)))
                + ((op0_limb_6_col50) * (op1_limb_20_col93)))
                + ((op0_limb_7_col51) * (op1_limb_19_col92)))
                + ((op0_limb_8_col52) * (op1_limb_18_col91)))
                + ((op0_limb_9_col53) * (op1_limb_17_col90)))
                + ((op0_limb_10_col54) * (op1_limb_16_col89)))
                + ((op0_limb_11_col55) * (op1_limb_15_col88)))
                + ((op0_limb_12_col56) * (op1_limb_14_col87)))
                + ((op0_limb_13_col57) * (op1_limb_13_col86)))
                + ((op0_limb_14_col58) * (op1_limb_12_col85)))
                + ((op0_limb_15_col59) * (op1_limb_11_col84)))
                + ((op0_limb_16_col60) * (op1_limb_10_col83)))
                + ((op0_limb_17_col61) * (op1_limb_9_col82)))
                + ((op0_limb_18_col62) * (op1_limb_8_col81)))
                + ((op0_limb_19_col63) * (op1_limb_7_col80)))
                + ((op0_limb_20_col64) * (op1_limb_6_col79)))
                + ((op0_limb_21_col65) * (op1_limb_5_col78)))
                + ((op0_limb_22_col66) * (op1_limb_4_col77)))
                + ((op0_limb_23_col67) * (op1_limb_3_col76)))
                + ((op0_limb_24_col68) * (op1_limb_2_col75)))
                + ((op0_limb_25_col69) * (op1_limb_1_col74)))
                + ((op0_limb_26_col70) * (op1_limb_0_col73)));
            let conv_tmp_42314_44 = ((((((((((((((((((((((((((((((M31_0)
                - (dst_limb_27_col42))
                + ((op0_limb_0_col44) * (op1_limb_27_col100)))
                + ((op0_limb_1_col45) * (op1_limb_26_col99)))
                + ((op0_limb_2_col46) * (op1_limb_25_col98)))
                + ((op0_limb_3_col47) * (op1_limb_24_col97)))
                + ((op0_limb_4_col48) * (op1_limb_23_col96)))
                + ((op0_limb_5_col49) * (op1_limb_22_col95)))
                + ((op0_limb_6_col50) * (op1_limb_21_col94)))
                + ((op0_limb_7_col51) * (op1_limb_20_col93)))
                + ((op0_limb_8_col52) * (op1_limb_19_col92)))
                + ((op0_limb_9_col53) * (op1_limb_18_col91)))
                + ((op0_limb_10_col54) * (op1_limb_17_col90)))
                + ((op0_limb_11_col55) * (op1_limb_16_col89)))
                + ((op0_limb_12_col56) * (op1_limb_15_col88)))
                + ((op0_limb_13_col57) * (op1_limb_14_col87)))
                + ((op0_limb_14_col58) * (op1_limb_13_col86)))
                + ((op0_limb_15_col59) * (op1_limb_12_col85)))
                + ((op0_limb_16_col60) * (op1_limb_11_col84)))
                + ((op0_limb_17_col61) * (op1_limb_10_col83)))
                + ((op0_limb_18_col62) * (op1_limb_9_col82)))
                + ((op0_limb_19_col63) * (op1_limb_8_col81)))
                + ((op0_limb_20_col64) * (op1_limb_7_col80)))
                + ((op0_limb_21_col65) * (op1_limb_6_col79)))
                + ((op0_limb_22_col66) * (op1_limb_5_col78)))
                + ((op0_limb_23_col67) * (op1_limb_4_col77)))
                + ((op0_limb_24_col68) * (op1_limb_3_col76)))
                + ((op0_limb_25_col69) * (op1_limb_2_col75)))
                + ((op0_limb_26_col70) * (op1_limb_1_col74)))
                + ((op0_limb_27_col71) * (op1_limb_0_col73)));
            let conv_tmp_42314_45 = ((((((((((((((((((((((((((((M31_0)
                + ((op0_limb_1_col45) * (op1_limb_27_col100)))
                + ((op0_limb_2_col46) * (op1_limb_26_col99)))
                + ((op0_limb_3_col47) * (op1_limb_25_col98)))
                + ((op0_limb_4_col48) * (op1_limb_24_col97)))
                + ((op0_limb_5_col49) * (op1_limb_23_col96)))
                + ((op0_limb_6_col50) * (op1_limb_22_col95)))
                + ((op0_limb_7_col51) * (op1_limb_21_col94)))
                + ((op0_limb_8_col52) * (op1_limb_20_col93)))
                + ((op0_limb_9_col53) * (op1_limb_19_col92)))
                + ((op0_limb_10_col54) * (op1_limb_18_col91)))
                + ((op0_limb_11_col55) * (op1_limb_17_col90)))
                + ((op0_limb_12_col56) * (op1_limb_16_col89)))
                + ((op0_limb_13_col57) * (op1_limb_15_col88)))
                + ((op0_limb_14_col58) * (op1_limb_14_col87)))
                + ((op0_limb_15_col59) * (op1_limb_13_col86)))
                + ((op0_limb_16_col60) * (op1_limb_12_col85)))
                + ((op0_limb_17_col61) * (op1_limb_11_col84)))
                + ((op0_limb_18_col62) * (op1_limb_10_col83)))
                + ((op0_limb_19_col63) * (op1_limb_9_col82)))
                + ((op0_limb_20_col64) * (op1_limb_8_col81)))
                + ((op0_limb_21_col65) * (op1_limb_7_col80)))
                + ((op0_limb_22_col66) * (op1_limb_6_col79)))
                + ((op0_limb_23_col67) * (op1_limb_5_col78)))
                + ((op0_limb_24_col68) * (op1_limb_4_col77)))
                + ((op0_limb_25_col69) * (op1_limb_3_col76)))
                + ((op0_limb_26_col70) * (op1_limb_2_col75)))
                + ((op0_limb_27_col71) * (op1_limb_1_col74)));
            let conv_tmp_42314_46 = (((((((((((((((((((((((((((M31_0)
                + ((op0_limb_2_col46) * (op1_limb_27_col100)))
                + ((op0_limb_3_col47) * (op1_limb_26_col99)))
                + ((op0_limb_4_col48) * (op1_limb_25_col98)))
                + ((op0_limb_5_col49) * (op1_limb_24_col97)))
                + ((op0_limb_6_col50) * (op1_limb_23_col96)))
                + ((op0_limb_7_col51) * (op1_limb_22_col95)))
                + ((op0_limb_8_col52) * (op1_limb_21_col94)))
                + ((op0_limb_9_col53) * (op1_limb_20_col93)))
                + ((op0_limb_10_col54) * (op1_limb_19_col92)))
                + ((op0_limb_11_col55) * (op1_limb_18_col91)))
                + ((op0_limb_12_col56) * (op1_limb_17_col90)))
                + ((op0_limb_13_col57) * (op1_limb_16_col89)))
                + ((op0_limb_14_col58) * (op1_limb_15_col88)))
                + ((op0_limb_15_col59) * (op1_limb_14_col87)))
                + ((op0_limb_16_col60) * (op1_limb_13_col86)))
                + ((op0_limb_17_col61) * (op1_limb_12_col85)))
                + ((op0_limb_18_col62) * (op1_limb_11_col84)))
                + ((op0_limb_19_col63) * (op1_limb_10_col83)))
                + ((op0_limb_20_col64) * (op1_limb_9_col82)))
                + ((op0_limb_21_col65) * (op1_limb_8_col81)))
                + ((op0_limb_22_col66) * (op1_limb_7_col80)))
                + ((op0_limb_23_col67) * (op1_limb_6_col79)))
                + ((op0_limb_24_col68) * (op1_limb_5_col78)))
                + ((op0_limb_25_col69) * (op1_limb_4_col77)))
                + ((op0_limb_26_col70) * (op1_limb_3_col76)))
                + ((op0_limb_27_col71) * (op1_limb_2_col75)));
            let conv_tmp_42314_47 = ((((((((((((((((((((((((((M31_0)
                + ((op0_limb_3_col47) * (op1_limb_27_col100)))
                + ((op0_limb_4_col48) * (op1_limb_26_col99)))
                + ((op0_limb_5_col49) * (op1_limb_25_col98)))
                + ((op0_limb_6_col50) * (op1_limb_24_col97)))
                + ((op0_limb_7_col51) * (op1_limb_23_col96)))
                + ((op0_limb_8_col52) * (op1_limb_22_col95)))
                + ((op0_limb_9_col53) * (op1_limb_21_col94)))
                + ((op0_limb_10_col54) * (op1_limb_20_col93)))
                + ((op0_limb_11_col55) * (op1_limb_19_col92)))
                + ((op0_limb_12_col56) * (op1_limb_18_col91)))
                + ((op0_limb_13_col57) * (op1_limb_17_col90)))
                + ((op0_limb_14_col58) * (op1_limb_16_col89)))
                + ((op0_limb_15_col59) * (op1_limb_15_col88)))
                + ((op0_limb_16_col60) * (op1_limb_14_col87)))
                + ((op0_limb_17_col61) * (op1_limb_13_col86)))
                + ((op0_limb_18_col62) * (op1_limb_12_col85)))
                + ((op0_limb_19_col63) * (op1_limb_11_col84)))
                + ((op0_limb_20_col64) * (op1_limb_10_col83)))
                + ((op0_limb_21_col65) * (op1_limb_9_col82)))
                + ((op0_limb_22_col66) * (op1_limb_8_col81)))
                + ((op0_limb_23_col67) * (op1_limb_7_col80)))
                + ((op0_limb_24_col68) * (op1_limb_6_col79)))
                + ((op0_limb_25_col69) * (op1_limb_5_col78)))
                + ((op0_limb_26_col70) * (op1_limb_4_col77)))
                + ((op0_limb_27_col71) * (op1_limb_3_col76)));
            let conv_tmp_42314_48 = (((((((((((((((((((((((((M31_0)
                + ((op0_limb_4_col48) * (op1_limb_27_col100)))
                + ((op0_limb_5_col49) * (op1_limb_26_col99)))
                + ((op0_limb_6_col50) * (op1_limb_25_col98)))
                + ((op0_limb_7_col51) * (op1_limb_24_col97)))
                + ((op0_limb_8_col52) * (op1_limb_23_col96)))
                + ((op0_limb_9_col53) * (op1_limb_22_col95)))
                + ((op0_limb_10_col54) * (op1_limb_21_col94)))
                + ((op0_limb_11_col55) * (op1_limb_20_col93)))
                + ((op0_limb_12_col56) * (op1_limb_19_col92)))
                + ((op0_limb_13_col57) * (op1_limb_18_col91)))
                + ((op0_limb_14_col58) * (op1_limb_17_col90)))
                + ((op0_limb_15_col59) * (op1_limb_16_col89)))
                + ((op0_limb_16_col60) * (op1_limb_15_col88)))
                + ((op0_limb_17_col61) * (op1_limb_14_col87)))
                + ((op0_limb_18_col62) * (op1_limb_13_col86)))
                + ((op0_limb_19_col63) * (op1_limb_12_col85)))
                + ((op0_limb_20_col64) * (op1_limb_11_col84)))
                + ((op0_limb_21_col65) * (op1_limb_10_col83)))
                + ((op0_limb_22_col66) * (op1_limb_9_col82)))
                + ((op0_limb_23_col67) * (op1_limb_8_col81)))
                + ((op0_limb_24_col68) * (op1_limb_7_col80)))
                + ((op0_limb_25_col69) * (op1_limb_6_col79)))
                + ((op0_limb_26_col70) * (op1_limb_5_col78)))
                + ((op0_limb_27_col71) * (op1_limb_4_col77)));
            let conv_tmp_42314_49 = ((((((((((((((((((((((((M31_0)
                + ((op0_limb_5_col49) * (op1_limb_27_col100)))
                + ((op0_limb_6_col50) * (op1_limb_26_col99)))
                + ((op0_limb_7_col51) * (op1_limb_25_col98)))
                + ((op0_limb_8_col52) * (op1_limb_24_col97)))
                + ((op0_limb_9_col53) * (op1_limb_23_col96)))
                + ((op0_limb_10_col54) * (op1_limb_22_col95)))
                + ((op0_limb_11_col55) * (op1_limb_21_col94)))
                + ((op0_limb_12_col56) * (op1_limb_20_col93)))
                + ((op0_limb_13_col57) * (op1_limb_19_col92)))
                + ((op0_limb_14_col58) * (op1_limb_18_col91)))
                + ((op0_limb_15_col59) * (op1_limb_17_col90)))
                + ((op0_limb_16_col60) * (op1_limb_16_col89)))
                + ((op0_limb_17_col61) * (op1_limb_15_col88)))
                + ((op0_limb_18_col62) * (op1_limb_14_col87)))
                + ((op0_limb_19_col63) * (op1_limb_13_col86)))
                + ((op0_limb_20_col64) * (op1_limb_12_col85)))
                + ((op0_limb_21_col65) * (op1_limb_11_col84)))
                + ((op0_limb_22_col66) * (op1_limb_10_col83)))
                + ((op0_limb_23_col67) * (op1_limb_9_col82)))
                + ((op0_limb_24_col68) * (op1_limb_8_col81)))
                + ((op0_limb_25_col69) * (op1_limb_7_col80)))
                + ((op0_limb_26_col70) * (op1_limb_6_col79)))
                + ((op0_limb_27_col71) * (op1_limb_5_col78)));
            let conv_tmp_42314_50 = (((((((((((((((((((((((M31_0)
                + ((op0_limb_6_col50) * (op1_limb_27_col100)))
                + ((op0_limb_7_col51) * (op1_limb_26_col99)))
                + ((op0_limb_8_col52) * (op1_limb_25_col98)))
                + ((op0_limb_9_col53) * (op1_limb_24_col97)))
                + ((op0_limb_10_col54) * (op1_limb_23_col96)))
                + ((op0_limb_11_col55) * (op1_limb_22_col95)))
                + ((op0_limb_12_col56) * (op1_limb_21_col94)))
                + ((op0_limb_13_col57) * (op1_limb_20_col93)))
                + ((op0_limb_14_col58) * (op1_limb_19_col92)))
                + ((op0_limb_15_col59) * (op1_limb_18_col91)))
                + ((op0_limb_16_col60) * (op1_limb_17_col90)))
                + ((op0_limb_17_col61) * (op1_limb_16_col89)))
                + ((op0_limb_18_col62) * (op1_limb_15_col88)))
                + ((op0_limb_19_col63) * (op1_limb_14_col87)))
                + ((op0_limb_20_col64) * (op1_limb_13_col86)))
                + ((op0_limb_21_col65) * (op1_limb_12_col85)))
                + ((op0_limb_22_col66) * (op1_limb_11_col84)))
                + ((op0_limb_23_col67) * (op1_limb_10_col83)))
                + ((op0_limb_24_col68) * (op1_limb_9_col82)))
                + ((op0_limb_25_col69) * (op1_limb_8_col81)))
                + ((op0_limb_26_col70) * (op1_limb_7_col80)))
                + ((op0_limb_27_col71) * (op1_limb_6_col79)));
            let conv_tmp_42314_51 = ((((((((((((((((((((((M31_0)
                + ((op0_limb_7_col51) * (op1_limb_27_col100)))
                + ((op0_limb_8_col52) * (op1_limb_26_col99)))
                + ((op0_limb_9_col53) * (op1_limb_25_col98)))
                + ((op0_limb_10_col54) * (op1_limb_24_col97)))
                + ((op0_limb_11_col55) * (op1_limb_23_col96)))
                + ((op0_limb_12_col56) * (op1_limb_22_col95)))
                + ((op0_limb_13_col57) * (op1_limb_21_col94)))
                + ((op0_limb_14_col58) * (op1_limb_20_col93)))
                + ((op0_limb_15_col59) * (op1_limb_19_col92)))
                + ((op0_limb_16_col60) * (op1_limb_18_col91)))
                + ((op0_limb_17_col61) * (op1_limb_17_col90)))
                + ((op0_limb_18_col62) * (op1_limb_16_col89)))
                + ((op0_limb_19_col63) * (op1_limb_15_col88)))
                + ((op0_limb_20_col64) * (op1_limb_14_col87)))
                + ((op0_limb_21_col65) * (op1_limb_13_col86)))
                + ((op0_limb_22_col66) * (op1_limb_12_col85)))
                + ((op0_limb_23_col67) * (op1_limb_11_col84)))
                + ((op0_limb_24_col68) * (op1_limb_10_col83)))
                + ((op0_limb_25_col69) * (op1_limb_9_col82)))
                + ((op0_limb_26_col70) * (op1_limb_8_col81)))
                + ((op0_limb_27_col71) * (op1_limb_7_col80)));
            let conv_tmp_42314_52 = (((((((((((((((((((((M31_0)
                + ((op0_limb_8_col52) * (op1_limb_27_col100)))
                + ((op0_limb_9_col53) * (op1_limb_26_col99)))
                + ((op0_limb_10_col54) * (op1_limb_25_col98)))
                + ((op0_limb_11_col55) * (op1_limb_24_col97)))
                + ((op0_limb_12_col56) * (op1_limb_23_col96)))
                + ((op0_limb_13_col57) * (op1_limb_22_col95)))
                + ((op0_limb_14_col58) * (op1_limb_21_col94)))
                + ((op0_limb_15_col59) * (op1_limb_20_col93)))
                + ((op0_limb_16_col60) * (op1_limb_19_col92)))
                + ((op0_limb_17_col61) * (op1_limb_18_col91)))
                + ((op0_limb_18_col62) * (op1_limb_17_col90)))
                + ((op0_limb_19_col63) * (op1_limb_16_col89)))
                + ((op0_limb_20_col64) * (op1_limb_15_col88)))
                + ((op0_limb_21_col65) * (op1_limb_14_col87)))
                + ((op0_limb_22_col66) * (op1_limb_13_col86)))
                + ((op0_limb_23_col67) * (op1_limb_12_col85)))
                + ((op0_limb_24_col68) * (op1_limb_11_col84)))
                + ((op0_limb_25_col69) * (op1_limb_10_col83)))
                + ((op0_limb_26_col70) * (op1_limb_9_col82)))
                + ((op0_limb_27_col71) * (op1_limb_8_col81)));
            let conv_tmp_42314_53 = ((((((((((((((((((((M31_0)
                + ((op0_limb_9_col53) * (op1_limb_27_col100)))
                + ((op0_limb_10_col54) * (op1_limb_26_col99)))
                + ((op0_limb_11_col55) * (op1_limb_25_col98)))
                + ((op0_limb_12_col56) * (op1_limb_24_col97)))
                + ((op0_limb_13_col57) * (op1_limb_23_col96)))
                + ((op0_limb_14_col58) * (op1_limb_22_col95)))
                + ((op0_limb_15_col59) * (op1_limb_21_col94)))
                + ((op0_limb_16_col60) * (op1_limb_20_col93)))
                + ((op0_limb_17_col61) * (op1_limb_19_col92)))
                + ((op0_limb_18_col62) * (op1_limb_18_col91)))
                + ((op0_limb_19_col63) * (op1_limb_17_col90)))
                + ((op0_limb_20_col64) * (op1_limb_16_col89)))
                + ((op0_limb_21_col65) * (op1_limb_15_col88)))
                + ((op0_limb_22_col66) * (op1_limb_14_col87)))
                + ((op0_limb_23_col67) * (op1_limb_13_col86)))
                + ((op0_limb_24_col68) * (op1_limb_12_col85)))
                + ((op0_limb_25_col69) * (op1_limb_11_col84)))
                + ((op0_limb_26_col70) * (op1_limb_10_col83)))
                + ((op0_limb_27_col71) * (op1_limb_9_col82)));
            let conv_tmp_42314_54 = (((((((((((((((((((M31_0)
                + ((op0_limb_10_col54) * (op1_limb_27_col100)))
                + ((op0_limb_11_col55) * (op1_limb_26_col99)))
                + ((op0_limb_12_col56) * (op1_limb_25_col98)))
                + ((op0_limb_13_col57) * (op1_limb_24_col97)))
                + ((op0_limb_14_col58) * (op1_limb_23_col96)))
                + ((op0_limb_15_col59) * (op1_limb_22_col95)))
                + ((op0_limb_16_col60) * (op1_limb_21_col94)))
                + ((op0_limb_17_col61) * (op1_limb_20_col93)))
                + ((op0_limb_18_col62) * (op1_limb_19_col92)))
                + ((op0_limb_19_col63) * (op1_limb_18_col91)))
                + ((op0_limb_20_col64) * (op1_limb_17_col90)))
                + ((op0_limb_21_col65) * (op1_limb_16_col89)))
                + ((op0_limb_22_col66) * (op1_limb_15_col88)))
                + ((op0_limb_23_col67) * (op1_limb_14_col87)))
                + ((op0_limb_24_col68) * (op1_limb_13_col86)))
                + ((op0_limb_25_col69) * (op1_limb_12_col85)))
                + ((op0_limb_26_col70) * (op1_limb_11_col84)))
                + ((op0_limb_27_col71) * (op1_limb_10_col83)));
            let conv_tmp_42314_55 = ((((((((((((((((((M31_0)
                + ((op0_limb_11_col55) * (op1_limb_27_col100)))
                + ((op0_limb_12_col56) * (op1_limb_26_col99)))
                + ((op0_limb_13_col57) * (op1_limb_25_col98)))
                + ((op0_limb_14_col58) * (op1_limb_24_col97)))
                + ((op0_limb_15_col59) * (op1_limb_23_col96)))
                + ((op0_limb_16_col60) * (op1_limb_22_col95)))
                + ((op0_limb_17_col61) * (op1_limb_21_col94)))
                + ((op0_limb_18_col62) * (op1_limb_20_col93)))
                + ((op0_limb_19_col63) * (op1_limb_19_col92)))
                + ((op0_limb_20_col64) * (op1_limb_18_col91)))
                + ((op0_limb_21_col65) * (op1_limb_17_col90)))
                + ((op0_limb_22_col66) * (op1_limb_16_col89)))
                + ((op0_limb_23_col67) * (op1_limb_15_col88)))
                + ((op0_limb_24_col68) * (op1_limb_14_col87)))
                + ((op0_limb_25_col69) * (op1_limb_13_col86)))
                + ((op0_limb_26_col70) * (op1_limb_12_col85)))
                + ((op0_limb_27_col71) * (op1_limb_11_col84)));
            let conv_tmp_42314_56 = (((((((((((((((((M31_0)
                + ((op0_limb_12_col56) * (op1_limb_27_col100)))
                + ((op0_limb_13_col57) * (op1_limb_26_col99)))
                + ((op0_limb_14_col58) * (op1_limb_25_col98)))
                + ((op0_limb_15_col59) * (op1_limb_24_col97)))
                + ((op0_limb_16_col60) * (op1_limb_23_col96)))
                + ((op0_limb_17_col61) * (op1_limb_22_col95)))
                + ((op0_limb_18_col62) * (op1_limb_21_col94)))
                + ((op0_limb_19_col63) * (op1_limb_20_col93)))
                + ((op0_limb_20_col64) * (op1_limb_19_col92)))
                + ((op0_limb_21_col65) * (op1_limb_18_col91)))
                + ((op0_limb_22_col66) * (op1_limb_17_col90)))
                + ((op0_limb_23_col67) * (op1_limb_16_col89)))
                + ((op0_limb_24_col68) * (op1_limb_15_col88)))
                + ((op0_limb_25_col69) * (op1_limb_14_col87)))
                + ((op0_limb_26_col70) * (op1_limb_13_col86)))
                + ((op0_limb_27_col71) * (op1_limb_12_col85)));
            let conv_tmp_42314_57 = ((((((((((((((((M31_0)
                + ((op0_limb_13_col57) * (op1_limb_27_col100)))
                + ((op0_limb_14_col58) * (op1_limb_26_col99)))
                + ((op0_limb_15_col59) * (op1_limb_25_col98)))
                + ((op0_limb_16_col60) * (op1_limb_24_col97)))
                + ((op0_limb_17_col61) * (op1_limb_23_col96)))
                + ((op0_limb_18_col62) * (op1_limb_22_col95)))
                + ((op0_limb_19_col63) * (op1_limb_21_col94)))
                + ((op0_limb_20_col64) * (op1_limb_20_col93)))
                + ((op0_limb_21_col65) * (op1_limb_19_col92)))
                + ((op0_limb_22_col66) * (op1_limb_18_col91)))
                + ((op0_limb_23_col67) * (op1_limb_17_col90)))
                + ((op0_limb_24_col68) * (op1_limb_16_col89)))
                + ((op0_limb_25_col69) * (op1_limb_15_col88)))
                + ((op0_limb_26_col70) * (op1_limb_14_col87)))
                + ((op0_limb_27_col71) * (op1_limb_13_col86)));
            let conv_tmp_42314_58 = (((((((((((((((M31_0)
                + ((op0_limb_14_col58) * (op1_limb_27_col100)))
                + ((op0_limb_15_col59) * (op1_limb_26_col99)))
                + ((op0_limb_16_col60) * (op1_limb_25_col98)))
                + ((op0_limb_17_col61) * (op1_limb_24_col97)))
                + ((op0_limb_18_col62) * (op1_limb_23_col96)))
                + ((op0_limb_19_col63) * (op1_limb_22_col95)))
                + ((op0_limb_20_col64) * (op1_limb_21_col94)))
                + ((op0_limb_21_col65) * (op1_limb_20_col93)))
                + ((op0_limb_22_col66) * (op1_limb_19_col92)))
                + ((op0_limb_23_col67) * (op1_limb_18_col91)))
                + ((op0_limb_24_col68) * (op1_limb_17_col90)))
                + ((op0_limb_25_col69) * (op1_limb_16_col89)))
                + ((op0_limb_26_col70) * (op1_limb_15_col88)))
                + ((op0_limb_27_col71) * (op1_limb_14_col87)));
            let conv_tmp_42314_59 = ((((((((((((((M31_0)
                + ((op0_limb_15_col59) * (op1_limb_27_col100)))
                + ((op0_limb_16_col60) * (op1_limb_26_col99)))
                + ((op0_limb_17_col61) * (op1_limb_25_col98)))
                + ((op0_limb_18_col62) * (op1_limb_24_col97)))
                + ((op0_limb_19_col63) * (op1_limb_23_col96)))
                + ((op0_limb_20_col64) * (op1_limb_22_col95)))
                + ((op0_limb_21_col65) * (op1_limb_21_col94)))
                + ((op0_limb_22_col66) * (op1_limb_20_col93)))
                + ((op0_limb_23_col67) * (op1_limb_19_col92)))
                + ((op0_limb_24_col68) * (op1_limb_18_col91)))
                + ((op0_limb_25_col69) * (op1_limb_17_col90)))
                + ((op0_limb_26_col70) * (op1_limb_16_col89)))
                + ((op0_limb_27_col71) * (op1_limb_15_col88)));
            let conv_tmp_42314_60 = (((((((((((((M31_0)
                + ((op0_limb_16_col60) * (op1_limb_27_col100)))
                + ((op0_limb_17_col61) * (op1_limb_26_col99)))
                + ((op0_limb_18_col62) * (op1_limb_25_col98)))
                + ((op0_limb_19_col63) * (op1_limb_24_col97)))
                + ((op0_limb_20_col64) * (op1_limb_23_col96)))
                + ((op0_limb_21_col65) * (op1_limb_22_col95)))
                + ((op0_limb_22_col66) * (op1_limb_21_col94)))
                + ((op0_limb_23_col67) * (op1_limb_20_col93)))
                + ((op0_limb_24_col68) * (op1_limb_19_col92)))
                + ((op0_limb_25_col69) * (op1_limb_18_col91)))
                + ((op0_limb_26_col70) * (op1_limb_17_col90)))
                + ((op0_limb_27_col71) * (op1_limb_16_col89)));
            let conv_tmp_42314_61 = ((((((((((((M31_0)
                + ((op0_limb_17_col61) * (op1_limb_27_col100)))
                + ((op0_limb_18_col62) * (op1_limb_26_col99)))
                + ((op0_limb_19_col63) * (op1_limb_25_col98)))
                + ((op0_limb_20_col64) * (op1_limb_24_col97)))
                + ((op0_limb_21_col65) * (op1_limb_23_col96)))
                + ((op0_limb_22_col66) * (op1_limb_22_col95)))
                + ((op0_limb_23_col67) * (op1_limb_21_col94)))
                + ((op0_limb_24_col68) * (op1_limb_20_col93)))
                + ((op0_limb_25_col69) * (op1_limb_19_col92)))
                + ((op0_limb_26_col70) * (op1_limb_18_col91)))
                + ((op0_limb_27_col71) * (op1_limb_17_col90)));
            let conv_tmp_42314_62 = (((((((((((M31_0)
                + ((op0_limb_18_col62) * (op1_limb_27_col100)))
                + ((op0_limb_19_col63) * (op1_limb_26_col99)))
                + ((op0_limb_20_col64) * (op1_limb_25_col98)))
                + ((op0_limb_21_col65) * (op1_limb_24_col97)))
                + ((op0_limb_22_col66) * (op1_limb_23_col96)))
                + ((op0_limb_23_col67) * (op1_limb_22_col95)))
                + ((op0_limb_24_col68) * (op1_limb_21_col94)))
                + ((op0_limb_25_col69) * (op1_limb_20_col93)))
                + ((op0_limb_26_col70) * (op1_limb_19_col92)))
                + ((op0_limb_27_col71) * (op1_limb_18_col91)));
            let conv_tmp_42314_63 = ((((((((((M31_0)
                + ((op0_limb_19_col63) * (op1_limb_27_col100)))
                + ((op0_limb_20_col64) * (op1_limb_26_col99)))
                + ((op0_limb_21_col65) * (op1_limb_25_col98)))
                + ((op0_limb_22_col66) * (op1_limb_24_col97)))
                + ((op0_limb_23_col67) * (op1_limb_23_col96)))
                + ((op0_limb_24_col68) * (op1_limb_22_col95)))
                + ((op0_limb_25_col69) * (op1_limb_21_col94)))
                + ((op0_limb_26_col70) * (op1_limb_20_col93)))
                + ((op0_limb_27_col71) * (op1_limb_19_col92)));
            let conv_tmp_42314_64 = (((((((((M31_0)
                + ((op0_limb_20_col64) * (op1_limb_27_col100)))
                + ((op0_limb_21_col65) * (op1_limb_26_col99)))
                + ((op0_limb_22_col66) * (op1_limb_25_col98)))
                + ((op0_limb_23_col67) * (op1_limb_24_col97)))
                + ((op0_limb_24_col68) * (op1_limb_23_col96)))
                + ((op0_limb_25_col69) * (op1_limb_22_col95)))
                + ((op0_limb_26_col70) * (op1_limb_21_col94)))
                + ((op0_limb_27_col71) * (op1_limb_20_col93)));
            let conv_tmp_42314_65 = ((((((((M31_0)
                + ((op0_limb_21_col65) * (op1_limb_27_col100)))
                + ((op0_limb_22_col66) * (op1_limb_26_col99)))
                + ((op0_limb_23_col67) * (op1_limb_25_col98)))
                + ((op0_limb_24_col68) * (op1_limb_24_col97)))
                + ((op0_limb_25_col69) * (op1_limb_23_col96)))
                + ((op0_limb_26_col70) * (op1_limb_22_col95)))
                + ((op0_limb_27_col71) * (op1_limb_21_col94)));
            let conv_tmp_42314_66 = (((((((M31_0)
                + ((op0_limb_22_col66) * (op1_limb_27_col100)))
                + ((op0_limb_23_col67) * (op1_limb_26_col99)))
                + ((op0_limb_24_col68) * (op1_limb_25_col98)))
                + ((op0_limb_25_col69) * (op1_limb_24_col97)))
                + ((op0_limb_26_col70) * (op1_limb_23_col96)))
                + ((op0_limb_27_col71) * (op1_limb_22_col95)));
            let conv_tmp_42314_67 = ((((((M31_0)
                + ((op0_limb_23_col67) * (op1_limb_27_col100)))
                + ((op0_limb_24_col68) * (op1_limb_26_col99)))
                + ((op0_limb_25_col69) * (op1_limb_25_col98)))
                + ((op0_limb_26_col70) * (op1_limb_24_col97)))
                + ((op0_limb_27_col71) * (op1_limb_23_col96)));
            let conv_tmp_42314_68 = (((((M31_0) + ((op0_limb_24_col68) * (op1_limb_27_col100)))
                + ((op0_limb_25_col69) * (op1_limb_26_col99)))
                + ((op0_limb_26_col70) * (op1_limb_25_col98)))
                + ((op0_limb_27_col71) * (op1_limb_24_col97)));
            let conv_tmp_42314_69 = ((((M31_0) + ((op0_limb_25_col69) * (op1_limb_27_col100)))
                + ((op0_limb_26_col70) * (op1_limb_26_col99)))
                + ((op0_limb_27_col71) * (op1_limb_25_col98)));
            let conv_tmp_42314_70 = (((M31_0) + ((op0_limb_26_col70) * (op1_limb_27_col100)))
                + ((op0_limb_27_col71) * (op1_limb_26_col99)));
            let conv_tmp_42314_71 = ((M31_0) + ((op0_limb_27_col71) * (op1_limb_27_col100)));
            let conv_mod_tmp_42314_72 = ((((M31_0) + ((M31_32) * (conv_tmp_42314_17)))
                - ((M31_4) * (conv_tmp_42314_38)))
                + ((M31_8) * (conv_tmp_42314_66)));
            let conv_mod_tmp_42314_73 = (((((M31_0) + ((M31_1) * (conv_tmp_42314_17)))
                + ((M31_32) * (conv_tmp_42314_18)))
                - ((M31_4) * (conv_tmp_42314_39)))
                + ((M31_8) * (conv_tmp_42314_67)));
            let conv_mod_tmp_42314_74 = (((((M31_0) + ((M31_1) * (conv_tmp_42314_18)))
                + ((M31_32) * (conv_tmp_42314_19)))
                - ((M31_4) * (conv_tmp_42314_40)))
                + ((M31_8) * (conv_tmp_42314_68)));
            let conv_mod_tmp_42314_75 = (((((M31_0) + ((M31_1) * (conv_tmp_42314_19)))
                + ((M31_32) * (conv_tmp_42314_20)))
                - ((M31_4) * (conv_tmp_42314_41)))
                + ((M31_8) * (conv_tmp_42314_69)));
            let conv_mod_tmp_42314_76 = (((((M31_0) + ((M31_1) * (conv_tmp_42314_20)))
                + ((M31_32) * (conv_tmp_42314_21)))
                - ((M31_4) * (conv_tmp_42314_42)))
                + ((M31_8) * (conv_tmp_42314_70)));
            let conv_mod_tmp_42314_77 = (((((M31_0) + ((M31_1) * (conv_tmp_42314_21)))
                + ((M31_32) * (conv_tmp_42314_22)))
                - ((M31_4) * (conv_tmp_42314_43)))
                + ((M31_8) * (conv_tmp_42314_71)));
            let conv_mod_tmp_42314_78 = ((((M31_0) + ((M31_1) * (conv_tmp_42314_22)))
                + ((M31_32) * (conv_tmp_42314_23)))
                - ((M31_4) * (conv_tmp_42314_44)));
            let conv_mod_tmp_42314_79 = (((((M31_0) + ((M31_2) * (conv_tmp_42314_17)))
                + ((M31_1) * (conv_tmp_42314_23)))
                + ((M31_32) * (conv_tmp_42314_24)))
                - ((M31_4) * (conv_tmp_42314_45)));
            let conv_mod_tmp_42314_80 = (((((M31_0) + ((M31_2) * (conv_tmp_42314_18)))
                + ((M31_1) * (conv_tmp_42314_24)))
                + ((M31_32) * (conv_tmp_42314_25)))
                - ((M31_4) * (conv_tmp_42314_46)));
            let conv_mod_tmp_42314_81 = (((((M31_0) + ((M31_2) * (conv_tmp_42314_19)))
                + ((M31_1) * (conv_tmp_42314_25)))
                + ((M31_32) * (conv_tmp_42314_26)))
                - ((M31_4) * (conv_tmp_42314_47)));
            let conv_mod_tmp_42314_82 = (((((M31_0) + ((M31_2) * (conv_tmp_42314_20)))
                + ((M31_1) * (conv_tmp_42314_26)))
                + ((M31_32) * (conv_tmp_42314_27)))
                - ((M31_4) * (conv_tmp_42314_48)));
            let conv_mod_tmp_42314_83 = (((((M31_0) + ((M31_2) * (conv_tmp_42314_21)))
                + ((M31_1) * (conv_tmp_42314_27)))
                + ((M31_32) * (conv_tmp_42314_28)))
                - ((M31_4) * (conv_tmp_42314_49)));
            let conv_mod_tmp_42314_84 = (((((M31_0) + ((M31_2) * (conv_tmp_42314_22)))
                + ((M31_1) * (conv_tmp_42314_28)))
                + ((M31_32) * (conv_tmp_42314_29)))
                - ((M31_4) * (conv_tmp_42314_50)));
            let conv_mod_tmp_42314_85 = (((((M31_0) + ((M31_2) * (conv_tmp_42314_23)))
                + ((M31_1) * (conv_tmp_42314_29)))
                + ((M31_32) * (conv_tmp_42314_30)))
                - ((M31_4) * (conv_tmp_42314_51)));
            let conv_mod_tmp_42314_86 = (((((M31_0) + ((M31_2) * (conv_tmp_42314_24)))
                + ((M31_1) * (conv_tmp_42314_30)))
                + ((M31_32) * (conv_tmp_42314_31)))
                - ((M31_4) * (conv_tmp_42314_52)));
            let conv_mod_tmp_42314_87 = (((((M31_0) + ((M31_2) * (conv_tmp_42314_25)))
                + ((M31_1) * (conv_tmp_42314_31)))
                + ((M31_32) * (conv_tmp_42314_32)))
                - ((M31_4) * (conv_tmp_42314_53)));
            let conv_mod_tmp_42314_88 = (((((M31_0) + ((M31_2) * (conv_tmp_42314_26)))
                + ((M31_1) * (conv_tmp_42314_32)))
                + ((M31_32) * (conv_tmp_42314_33)))
                - ((M31_4) * (conv_tmp_42314_54)));
            let conv_mod_tmp_42314_89 = (((((M31_0) + ((M31_2) * (conv_tmp_42314_27)))
                + ((M31_1) * (conv_tmp_42314_33)))
                + ((M31_32) * (conv_tmp_42314_34)))
                - ((M31_4) * (conv_tmp_42314_55)));
            let conv_mod_tmp_42314_90 = (((((M31_0) + ((M31_2) * (conv_tmp_42314_28)))
                + ((M31_1) * (conv_tmp_42314_34)))
                + ((M31_32) * (conv_tmp_42314_35)))
                - ((M31_4) * (conv_tmp_42314_56)));
            let conv_mod_tmp_42314_91 = (((((M31_0) + ((M31_2) * (conv_tmp_42314_29)))
                + ((M31_1) * (conv_tmp_42314_35)))
                + ((M31_32) * (conv_tmp_42314_36)))
                - ((M31_4) * (conv_tmp_42314_57)));
            let conv_mod_tmp_42314_92 = (((((M31_0) + ((M31_2) * (conv_tmp_42314_30)))
                + ((M31_1) * (conv_tmp_42314_36)))
                + ((M31_32) * (conv_tmp_42314_37)))
                - ((M31_4) * (conv_tmp_42314_58)));
            let conv_mod_tmp_42314_93 = (((((M31_0) + ((M31_2) * (conv_tmp_42314_31)))
                + ((M31_1) * (conv_tmp_42314_37)))
                - ((M31_4) * (conv_tmp_42314_59)))
                + ((M31_64) * (conv_tmp_42314_66)));
            let conv_mod_tmp_42314_94 = (((((M31_0) + ((M31_2) * (conv_tmp_42314_32)))
                - ((M31_4) * (conv_tmp_42314_60)))
                + ((M31_2) * (conv_tmp_42314_66)))
                + ((M31_64) * (conv_tmp_42314_67)));
            let conv_mod_tmp_42314_95 = (((((M31_0) + ((M31_2) * (conv_tmp_42314_33)))
                - ((M31_4) * (conv_tmp_42314_61)))
                + ((M31_2) * (conv_tmp_42314_67)))
                + ((M31_64) * (conv_tmp_42314_68)));
            let conv_mod_tmp_42314_96 = (((((M31_0) + ((M31_2) * (conv_tmp_42314_34)))
                - ((M31_4) * (conv_tmp_42314_62)))
                + ((M31_2) * (conv_tmp_42314_68)))
                + ((M31_64) * (conv_tmp_42314_69)));
            let conv_mod_tmp_42314_97 = (((((M31_0) + ((M31_2) * (conv_tmp_42314_35)))
                - ((M31_4) * (conv_tmp_42314_63)))
                + ((M31_2) * (conv_tmp_42314_69)))
                + ((M31_64) * (conv_tmp_42314_70)));
            let conv_mod_tmp_42314_98 = (((((M31_0) + ((M31_2) * (conv_tmp_42314_36)))
                - ((M31_4) * (conv_tmp_42314_64)))
                + ((M31_2) * (conv_tmp_42314_70)))
                + ((M31_64) * (conv_tmp_42314_71)));
            let conv_mod_tmp_42314_99 = ((((M31_0) + ((M31_2) * (conv_tmp_42314_37)))
                - ((M31_4) * (conv_tmp_42314_65)))
                + ((M31_2) * (conv_tmp_42314_71)));
            let k_mod_2_18_biased_tmp_42314_100 =
                ((((PackedUInt32::from_m31(((conv_mod_tmp_42314_72) + (M31_134217728))))
                    + (((PackedUInt32::from_m31(((conv_mod_tmp_42314_73) + (M31_134217728))))
                        & (UInt32_511))
                        << (UInt32_9)))
                    + (UInt32_65536))
                    & (UInt32_262143));
            let k_col101 = ((k_mod_2_18_biased_tmp_42314_100.low().as_m31())
                + (((k_mod_2_18_biased_tmp_42314_100.high().as_m31()) - (M31_1)) * (M31_65536)));
            *row[101] = k_col101;
            let range_check_19_inputs_0 = [((k_col101) + (M31_262144))].unpack();
            *lookup_data.range_check_19_0 = [((k_col101) + (M31_262144))];
            let carry_0_col102 =
                ((((conv_mod_tmp_42314_72) - ((M31_1) * (k_col101))) + (M31_0)) * (M31_4194304));
            *row[102] = carry_0_col102;
            let range_check_19_inputs_1 = [((carry_0_col102) + (M31_131072))].unpack();
            *lookup_data.range_check_19_1 = [((carry_0_col102) + (M31_131072))];
            let carry_1_col103 = (((conv_mod_tmp_42314_73) + (carry_0_col102)) * (M31_4194304));
            *row[103] = carry_1_col103;
            let range_check_19_inputs_2 = [((carry_1_col103) + (M31_131072))].unpack();
            *lookup_data.range_check_19_2 = [((carry_1_col103) + (M31_131072))];
            let carry_2_col104 = (((conv_mod_tmp_42314_74) + (carry_1_col103)) * (M31_4194304));
            *row[104] = carry_2_col104;
            let range_check_19_inputs_3 = [((carry_2_col104) + (M31_131072))].unpack();
            *lookup_data.range_check_19_3 = [((carry_2_col104) + (M31_131072))];
            let carry_3_col105 = (((conv_mod_tmp_42314_75) + (carry_2_col104)) * (M31_4194304));
            *row[105] = carry_3_col105;
            let range_check_19_inputs_4 = [((carry_3_col105) + (M31_131072))].unpack();
            *lookup_data.range_check_19_4 = [((carry_3_col105) + (M31_131072))];
            let carry_4_col106 = (((conv_mod_tmp_42314_76) + (carry_3_col105)) * (M31_4194304));
            *row[106] = carry_4_col106;
            let range_check_19_inputs_5 = [((carry_4_col106) + (M31_131072))].unpack();
            *lookup_data.range_check_19_5 = [((carry_4_col106) + (M31_131072))];
            let carry_5_col107 = (((conv_mod_tmp_42314_77) + (carry_4_col106)) * (M31_4194304));
            *row[107] = carry_5_col107;
            let range_check_19_inputs_6 = [((carry_5_col107) + (M31_131072))].unpack();
            *lookup_data.range_check_19_6 = [((carry_5_col107) + (M31_131072))];
            let carry_6_col108 = (((conv_mod_tmp_42314_78) + (carry_5_col107)) * (M31_4194304));
            *row[108] = carry_6_col108;
            let range_check_19_inputs_7 = [((carry_6_col108) + (M31_131072))].unpack();
            *lookup_data.range_check_19_7 = [((carry_6_col108) + (M31_131072))];
            let carry_7_col109 = (((conv_mod_tmp_42314_79) + (carry_6_col108)) * (M31_4194304));
            *row[109] = carry_7_col109;
            let range_check_19_inputs_8 = [((carry_7_col109) + (M31_131072))].unpack();
            *lookup_data.range_check_19_8 = [((carry_7_col109) + (M31_131072))];
            let carry_8_col110 = (((conv_mod_tmp_42314_80) + (carry_7_col109)) * (M31_4194304));
            *row[110] = carry_8_col110;
            let range_check_19_inputs_9 = [((carry_8_col110) + (M31_131072))].unpack();
            *lookup_data.range_check_19_9 = [((carry_8_col110) + (M31_131072))];
            let carry_9_col111 = (((conv_mod_tmp_42314_81) + (carry_8_col110)) * (M31_4194304));
            *row[111] = carry_9_col111;
            let range_check_19_inputs_10 = [((carry_9_col111) + (M31_131072))].unpack();
            *lookup_data.range_check_19_10 = [((carry_9_col111) + (M31_131072))];
            let carry_10_col112 = (((conv_mod_tmp_42314_82) + (carry_9_col111)) * (M31_4194304));
            *row[112] = carry_10_col112;
            let range_check_19_inputs_11 = [((carry_10_col112) + (M31_131072))].unpack();
            *lookup_data.range_check_19_11 = [((carry_10_col112) + (M31_131072))];
            let carry_11_col113 = (((conv_mod_tmp_42314_83) + (carry_10_col112)) * (M31_4194304));
            *row[113] = carry_11_col113;
            let range_check_19_inputs_12 = [((carry_11_col113) + (M31_131072))].unpack();
            *lookup_data.range_check_19_12 = [((carry_11_col113) + (M31_131072))];
            let carry_12_col114 = (((conv_mod_tmp_42314_84) + (carry_11_col113)) * (M31_4194304));
            *row[114] = carry_12_col114;
            let range_check_19_inputs_13 = [((carry_12_col114) + (M31_131072))].unpack();
            *lookup_data.range_check_19_13 = [((carry_12_col114) + (M31_131072))];
            let carry_13_col115 = (((conv_mod_tmp_42314_85) + (carry_12_col114)) * (M31_4194304));
            *row[115] = carry_13_col115;
            let range_check_19_inputs_14 = [((carry_13_col115) + (M31_131072))].unpack();
            *lookup_data.range_check_19_14 = [((carry_13_col115) + (M31_131072))];
            let carry_14_col116 = (((conv_mod_tmp_42314_86) + (carry_13_col115)) * (M31_4194304));
            *row[116] = carry_14_col116;
            let range_check_19_inputs_15 = [((carry_14_col116) + (M31_131072))].unpack();
            *lookup_data.range_check_19_15 = [((carry_14_col116) + (M31_131072))];
            let carry_15_col117 = (((conv_mod_tmp_42314_87) + (carry_14_col116)) * (M31_4194304));
            *row[117] = carry_15_col117;
            let range_check_19_inputs_16 = [((carry_15_col117) + (M31_131072))].unpack();
            *lookup_data.range_check_19_16 = [((carry_15_col117) + (M31_131072))];
            let carry_16_col118 = (((conv_mod_tmp_42314_88) + (carry_15_col117)) * (M31_4194304));
            *row[118] = carry_16_col118;
            let range_check_19_inputs_17 = [((carry_16_col118) + (M31_131072))].unpack();
            *lookup_data.range_check_19_17 = [((carry_16_col118) + (M31_131072))];
            let carry_17_col119 = (((conv_mod_tmp_42314_89) + (carry_16_col118)) * (M31_4194304));
            *row[119] = carry_17_col119;
            let range_check_19_inputs_18 = [((carry_17_col119) + (M31_131072))].unpack();
            *lookup_data.range_check_19_18 = [((carry_17_col119) + (M31_131072))];
            let carry_18_col120 = (((conv_mod_tmp_42314_90) + (carry_17_col119)) * (M31_4194304));
            *row[120] = carry_18_col120;
            let range_check_19_inputs_19 = [((carry_18_col120) + (M31_131072))].unpack();
            *lookup_data.range_check_19_19 = [((carry_18_col120) + (M31_131072))];
            let carry_19_col121 = (((conv_mod_tmp_42314_91) + (carry_18_col120)) * (M31_4194304));
            *row[121] = carry_19_col121;
            let range_check_19_inputs_20 = [((carry_19_col121) + (M31_131072))].unpack();
            *lookup_data.range_check_19_20 = [((carry_19_col121) + (M31_131072))];
            let carry_20_col122 = (((conv_mod_tmp_42314_92) + (carry_19_col121)) * (M31_4194304));
            *row[122] = carry_20_col122;
            let range_check_19_inputs_21 = [((carry_20_col122) + (M31_131072))].unpack();
            *lookup_data.range_check_19_21 = [((carry_20_col122) + (M31_131072))];
            let carry_21_col123 = ((((conv_mod_tmp_42314_93) - ((M31_136) * (k_col101)))
                + (carry_20_col122))
                * (M31_4194304));
            *row[123] = carry_21_col123;
            let range_check_19_inputs_22 = [((carry_21_col123) + (M31_131072))].unpack();
            *lookup_data.range_check_19_22 = [((carry_21_col123) + (M31_131072))];
            let carry_22_col124 = (((conv_mod_tmp_42314_94) + (carry_21_col123)) * (M31_4194304));
            *row[124] = carry_22_col124;
            let range_check_19_inputs_23 = [((carry_22_col124) + (M31_131072))].unpack();
            *lookup_data.range_check_19_23 = [((carry_22_col124) + (M31_131072))];
            let carry_23_col125 = (((conv_mod_tmp_42314_95) + (carry_22_col124)) * (M31_4194304));
            *row[125] = carry_23_col125;
            let range_check_19_inputs_24 = [((carry_23_col125) + (M31_131072))].unpack();
            *lookup_data.range_check_19_24 = [((carry_23_col125) + (M31_131072))];
            let carry_24_col126 = (((conv_mod_tmp_42314_96) + (carry_23_col125)) * (M31_4194304));
            *row[126] = carry_24_col126;
            let range_check_19_inputs_25 = [((carry_24_col126) + (M31_131072))].unpack();
            *lookup_data.range_check_19_25 = [((carry_24_col126) + (M31_131072))];
            let carry_25_col127 = (((conv_mod_tmp_42314_97) + (carry_24_col126)) * (M31_4194304));
            *row[127] = carry_25_col127;
            let range_check_19_inputs_26 = [((carry_25_col127) + (M31_131072))].unpack();
            *lookup_data.range_check_19_26 = [((carry_25_col127) + (M31_131072))];
            let carry_26_col128 = (((conv_mod_tmp_42314_98) + (carry_25_col127)) * (M31_4194304));
            *row[128] = carry_26_col128;
            let range_check_19_inputs_27 = [((carry_26_col128) + (M31_131072))].unpack();
            *lookup_data.range_check_19_27 = [((carry_26_col128) + (M31_131072))];

            *lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];
            *lookup_data.opcodes_1 = [
                ((input_pc_col0) + (M31_1)),
                ((input_ap_col1) + (ap_update_add_1_col10)),
                input_fp_col2,
            ];
            *row[129] = padding_col.packed_at(row_index);

            // Add sub-components inputs.
            verify_instruction_state.add_inputs(&verify_instruction_inputs_0);
            memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_0);
            memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_0);
            memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_1);
            memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_1);
            memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_2);
            memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_2);
            range_check_19_state.add_inputs(&range_check_19_inputs_0);
            range_check_19_state.add_inputs(&range_check_19_inputs_1);
            range_check_19_state.add_inputs(&range_check_19_inputs_2);
            range_check_19_state.add_inputs(&range_check_19_inputs_3);
            range_check_19_state.add_inputs(&range_check_19_inputs_4);
            range_check_19_state.add_inputs(&range_check_19_inputs_5);
            range_check_19_state.add_inputs(&range_check_19_inputs_6);
            range_check_19_state.add_inputs(&range_check_19_inputs_7);
            range_check_19_state.add_inputs(&range_check_19_inputs_8);
            range_check_19_state.add_inputs(&range_check_19_inputs_9);
            range_check_19_state.add_inputs(&range_check_19_inputs_10);
            range_check_19_state.add_inputs(&range_check_19_inputs_11);
            range_check_19_state.add_inputs(&range_check_19_inputs_12);
            range_check_19_state.add_inputs(&range_check_19_inputs_13);
            range_check_19_state.add_inputs(&range_check_19_inputs_14);
            range_check_19_state.add_inputs(&range_check_19_inputs_15);
            range_check_19_state.add_inputs(&range_check_19_inputs_16);
            range_check_19_state.add_inputs(&range_check_19_inputs_17);
            range_check_19_state.add_inputs(&range_check_19_inputs_18);
            range_check_19_state.add_inputs(&range_check_19_inputs_19);
            range_check_19_state.add_inputs(&range_check_19_inputs_20);
            range_check_19_state.add_inputs(&range_check_19_inputs_21);
            range_check_19_state.add_inputs(&range_check_19_inputs_22);
            range_check_19_state.add_inputs(&range_check_19_inputs_23);
            range_check_19_state.add_inputs(&range_check_19_inputs_24);
            range_check_19_state.add_inputs(&range_check_19_inputs_25);
            range_check_19_state.add_inputs(&range_check_19_inputs_26);
            range_check_19_state.add_inputs(&range_check_19_inputs_27);
        });

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
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        memory_address_to_id: &relations::MemoryAddressToId,
        memory_id_to_big: &relations::MemoryIdToBig,
        opcodes: &relations::Opcodes,
        range_check_19: &relations::RangeCheck_19,
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
