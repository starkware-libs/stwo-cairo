#![allow(unused_parens)]
use crate::cairo_air::components::jnz_opcode_taken::{Claim, InteractionClaim, N_TRACE_COLUMNS};
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
            n_rows,
            packed_inputs,
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
    memory_address_to_id: [Vec<memory_address_to_id::PackedInputType>; 2],
    memory_id_to_big: [Vec<memory_id_to_big::PackedInputType>; 2],
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
    let M31_134217728 = PackedM31::broadcast(M31::from(134217728));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_32767 = PackedM31::broadcast(M31::from(32767));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_32769 = PackedM31::broadcast(M31::from(32769));
    let M31_48 = PackedM31::broadcast(M31::from(48));
    let M31_511 = PackedM31::broadcast(M31::from(511));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let M31_8 = PackedM31::broadcast(M31::from(8));
    let UInt16_1 = PackedUInt16::broadcast(UInt16::from(1));
    let UInt16_11 = PackedUInt16::broadcast(UInt16::from(11));
    let UInt16_127 = PackedUInt16::broadcast(UInt16::from(127));
    let UInt16_3 = PackedUInt16::broadcast(UInt16::from(3));
    let UInt16_6 = PackedUInt16::broadcast(UInt16::from(6));
    let UInt16_9 = PackedUInt16::broadcast(UInt16::from(9));
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
            |(row_index, (mut row, lookup_data, sub_component_inputs, jnz_opcode_taken_input))| {
                let input_pc_col0 = jnz_opcode_taken_input.pc;
                *row[0] = input_pc_col0;
                let input_ap_col1 = jnz_opcode_taken_input.ap;
                *row[1] = input_ap_col1;
                let input_fp_col2 = jnz_opcode_taken_input.fp;
                *row[2] = input_fp_col2;

                // Decode Instruction.

                let memory_address_to_id_value_tmp_65b05_0 =
                    memory_address_to_id_state.deduce_output(input_pc_col0);
                let memory_id_to_big_value_tmp_65b05_1 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_65b05_0);
                let offset0_tmp_65b05_2 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_65b05_1.get_m31(0)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_65b05_1.get_m31(1),
                        )) & (UInt16_127))
                            << (UInt16_9)));
                let offset0_col3 = offset0_tmp_65b05_2.as_m31();
                *row[3] = offset0_col3;
                let ap_update_add_1_tmp_65b05_3 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_65b05_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_65b05_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_11))
                        & (UInt16_1));
                let ap_update_add_1_col4 = ap_update_add_1_tmp_65b05_3.as_m31();
                *row[4] = ap_update_add_1_col4;
                *sub_component_inputs.verify_instruction[0] = (
                    input_pc_col0,
                    [offset0_col3, M31_32767, M31_32769],
                    [M31_48, ((M31_8) + ((ap_update_add_1_col4) * (M31_32)))],
                    M31_0,
                );
                *lookup_data.verify_instruction_0 = [
                    input_pc_col0,
                    offset0_col3,
                    M31_32767,
                    M31_32769,
                    M31_48,
                    ((M31_8) + ((ap_update_add_1_col4) * (M31_32))),
                    M31_0,
                ];

                // Read Positive Num Bits 252.

                let memory_address_to_id_value_tmp_65b05_4 = memory_address_to_id_state
                    .deduce_output(((input_ap_col1) + ((offset0_col3) - (M31_32768))));
                let memory_id_to_big_value_tmp_65b05_5 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_65b05_4);
                let dst_id_col5 = memory_address_to_id_value_tmp_65b05_4;
                *row[5] = dst_id_col5;
                *sub_component_inputs.memory_address_to_id[0] =
                    ((input_ap_col1) + ((offset0_col3) - (M31_32768)));
                *lookup_data.memory_address_to_id_0 = [
                    ((input_ap_col1) + ((offset0_col3) - (M31_32768))),
                    dst_id_col5,
                ];
                let dst_limb_0_col6 = memory_id_to_big_value_tmp_65b05_5.get_m31(0);
                *row[6] = dst_limb_0_col6;
                let dst_limb_1_col7 = memory_id_to_big_value_tmp_65b05_5.get_m31(1);
                *row[7] = dst_limb_1_col7;
                let dst_limb_2_col8 = memory_id_to_big_value_tmp_65b05_5.get_m31(2);
                *row[8] = dst_limb_2_col8;
                let dst_limb_3_col9 = memory_id_to_big_value_tmp_65b05_5.get_m31(3);
                *row[9] = dst_limb_3_col9;
                let dst_limb_4_col10 = memory_id_to_big_value_tmp_65b05_5.get_m31(4);
                *row[10] = dst_limb_4_col10;
                let dst_limb_5_col11 = memory_id_to_big_value_tmp_65b05_5.get_m31(5);
                *row[11] = dst_limb_5_col11;
                let dst_limb_6_col12 = memory_id_to_big_value_tmp_65b05_5.get_m31(6);
                *row[12] = dst_limb_6_col12;
                let dst_limb_7_col13 = memory_id_to_big_value_tmp_65b05_5.get_m31(7);
                *row[13] = dst_limb_7_col13;
                let dst_limb_8_col14 = memory_id_to_big_value_tmp_65b05_5.get_m31(8);
                *row[14] = dst_limb_8_col14;
                let dst_limb_9_col15 = memory_id_to_big_value_tmp_65b05_5.get_m31(9);
                *row[15] = dst_limb_9_col15;
                let dst_limb_10_col16 = memory_id_to_big_value_tmp_65b05_5.get_m31(10);
                *row[16] = dst_limb_10_col16;
                let dst_limb_11_col17 = memory_id_to_big_value_tmp_65b05_5.get_m31(11);
                *row[17] = dst_limb_11_col17;
                let dst_limb_12_col18 = memory_id_to_big_value_tmp_65b05_5.get_m31(12);
                *row[18] = dst_limb_12_col18;
                let dst_limb_13_col19 = memory_id_to_big_value_tmp_65b05_5.get_m31(13);
                *row[19] = dst_limb_13_col19;
                let dst_limb_14_col20 = memory_id_to_big_value_tmp_65b05_5.get_m31(14);
                *row[20] = dst_limb_14_col20;
                let dst_limb_15_col21 = memory_id_to_big_value_tmp_65b05_5.get_m31(15);
                *row[21] = dst_limb_15_col21;
                let dst_limb_16_col22 = memory_id_to_big_value_tmp_65b05_5.get_m31(16);
                *row[22] = dst_limb_16_col22;
                let dst_limb_17_col23 = memory_id_to_big_value_tmp_65b05_5.get_m31(17);
                *row[23] = dst_limb_17_col23;
                let dst_limb_18_col24 = memory_id_to_big_value_tmp_65b05_5.get_m31(18);
                *row[24] = dst_limb_18_col24;
                let dst_limb_19_col25 = memory_id_to_big_value_tmp_65b05_5.get_m31(19);
                *row[25] = dst_limb_19_col25;
                let dst_limb_20_col26 = memory_id_to_big_value_tmp_65b05_5.get_m31(20);
                *row[26] = dst_limb_20_col26;
                let dst_limb_21_col27 = memory_id_to_big_value_tmp_65b05_5.get_m31(21);
                *row[27] = dst_limb_21_col27;
                let dst_limb_22_col28 = memory_id_to_big_value_tmp_65b05_5.get_m31(22);
                *row[28] = dst_limb_22_col28;
                let dst_limb_23_col29 = memory_id_to_big_value_tmp_65b05_5.get_m31(23);
                *row[29] = dst_limb_23_col29;
                let dst_limb_24_col30 = memory_id_to_big_value_tmp_65b05_5.get_m31(24);
                *row[30] = dst_limb_24_col30;
                let dst_limb_25_col31 = memory_id_to_big_value_tmp_65b05_5.get_m31(25);
                *row[31] = dst_limb_25_col31;
                let dst_limb_26_col32 = memory_id_to_big_value_tmp_65b05_5.get_m31(26);
                *row[32] = dst_limb_26_col32;
                let dst_limb_27_col33 = memory_id_to_big_value_tmp_65b05_5.get_m31(27);
                *row[33] = dst_limb_27_col33;
                *sub_component_inputs.memory_id_to_big[0] = dst_id_col5;
                *lookup_data.memory_id_to_big_0 = [
                    dst_id_col5,
                    dst_limb_0_col6,
                    dst_limb_1_col7,
                    dst_limb_2_col8,
                    dst_limb_3_col9,
                    dst_limb_4_col10,
                    dst_limb_5_col11,
                    dst_limb_6_col12,
                    dst_limb_7_col13,
                    dst_limb_8_col14,
                    dst_limb_9_col15,
                    dst_limb_10_col16,
                    dst_limb_11_col17,
                    dst_limb_12_col18,
                    dst_limb_13_col19,
                    dst_limb_14_col20,
                    dst_limb_15_col21,
                    dst_limb_16_col22,
                    dst_limb_17_col23,
                    dst_limb_18_col24,
                    dst_limb_19_col25,
                    dst_limb_20_col26,
                    dst_limb_21_col27,
                    dst_limb_22_col28,
                    dst_limb_23_col29,
                    dst_limb_24_col30,
                    dst_limb_25_col31,
                    dst_limb_26_col32,
                    dst_limb_27_col33,
                ];

                let res_col34 = ((((((((((((((((((((((((((((dst_limb_0_col6)
                    + (dst_limb_1_col7))
                    + (dst_limb_2_col8))
                    + (dst_limb_3_col9))
                    + (dst_limb_4_col10))
                    + (dst_limb_5_col11))
                    + (dst_limb_6_col12))
                    + (dst_limb_7_col13))
                    + (dst_limb_8_col14))
                    + (dst_limb_9_col15))
                    + (dst_limb_10_col16))
                    + (dst_limb_11_col17))
                    + (dst_limb_12_col18))
                    + (dst_limb_13_col19))
                    + (dst_limb_14_col20))
                    + (dst_limb_15_col21))
                    + (dst_limb_16_col22))
                    + (dst_limb_17_col23))
                    + (dst_limb_18_col24))
                    + (dst_limb_19_col25))
                    + (dst_limb_20_col26))
                    + (dst_limb_21_col27))
                    + (dst_limb_22_col28))
                    + (dst_limb_23_col29))
                    + (dst_limb_24_col30))
                    + (dst_limb_25_col31))
                    + (dst_limb_26_col32))
                    + (dst_limb_27_col33))
                    .inverse();
                *row[34] = res_col34;
                let diff_from_p_tmp_65b05_6 = ((dst_limb_0_col6) - (M31_1));
                let diff_from_p_tmp_65b05_7 = ((dst_limb_21_col27) - (M31_136));
                let diff_from_p_tmp_65b05_8 = ((dst_limb_27_col33) - (M31_256));
                let res_squares_col35 = (((((((((((((((((((((((((((((diff_from_p_tmp_65b05_6)
                    * (diff_from_p_tmp_65b05_6))
                    + (dst_limb_1_col7))
                    + (dst_limb_2_col8))
                    + (dst_limb_3_col9))
                    + (dst_limb_4_col10))
                    + (dst_limb_5_col11))
                    + (dst_limb_6_col12))
                    + (dst_limb_7_col13))
                    + (dst_limb_8_col14))
                    + (dst_limb_9_col15))
                    + (dst_limb_10_col16))
                    + (dst_limb_11_col17))
                    + (dst_limb_12_col18))
                    + (dst_limb_13_col19))
                    + (dst_limb_14_col20))
                    + (dst_limb_15_col21))
                    + (dst_limb_16_col22))
                    + (dst_limb_17_col23))
                    + (dst_limb_18_col24))
                    + (dst_limb_19_col25))
                    + (dst_limb_20_col26))
                    + ((diff_from_p_tmp_65b05_7) * (diff_from_p_tmp_65b05_7)))
                    + (dst_limb_22_col28))
                    + (dst_limb_23_col29))
                    + (dst_limb_24_col30))
                    + (dst_limb_25_col31))
                    + (dst_limb_26_col32))
                    + ((diff_from_p_tmp_65b05_8) * (diff_from_p_tmp_65b05_8)))
                    .inverse();
                *row[35] = res_squares_col35;

                // Read Small.

                let memory_address_to_id_value_tmp_65b05_9 =
                    memory_address_to_id_state.deduce_output(((input_pc_col0) + (M31_1)));
                let memory_id_to_big_value_tmp_65b05_10 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_65b05_9);
                let next_pc_id_col36 = memory_address_to_id_value_tmp_65b05_9;
                *row[36] = next_pc_id_col36;
                *sub_component_inputs.memory_address_to_id[1] = ((input_pc_col0) + (M31_1));
                *lookup_data.memory_address_to_id_1 =
                    [((input_pc_col0) + (M31_1)), next_pc_id_col36];

                // Cond Decode Small Sign.

                let msb_tmp_65b05_11 = memory_id_to_big_value_tmp_65b05_10.get_m31(27).eq(M31_256);
                let msb_col37 = msb_tmp_65b05_11.as_m31();
                *row[37] = msb_col37;
                let mid_limbs_set_tmp_65b05_12 =
                    memory_id_to_big_value_tmp_65b05_10.get_m31(20).eq(M31_511);
                let mid_limbs_set_col38 = mid_limbs_set_tmp_65b05_12.as_m31();
                *row[38] = mid_limbs_set_col38;

                let next_pc_limb_0_col39 = memory_id_to_big_value_tmp_65b05_10.get_m31(0);
                *row[39] = next_pc_limb_0_col39;
                let next_pc_limb_1_col40 = memory_id_to_big_value_tmp_65b05_10.get_m31(1);
                *row[40] = next_pc_limb_1_col40;
                let next_pc_limb_2_col41 = memory_id_to_big_value_tmp_65b05_10.get_m31(2);
                *row[41] = next_pc_limb_2_col41;
                *sub_component_inputs.memory_id_to_big[1] = next_pc_id_col36;
                *lookup_data.memory_id_to_big_1 = [
                    next_pc_id_col36,
                    next_pc_limb_0_col39,
                    next_pc_limb_1_col40,
                    next_pc_limb_2_col41,
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    (((M31_136) * (msb_col37)) - (mid_limbs_set_col38)),
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    ((msb_col37) * (M31_256)),
                ];

                *lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];
                *lookup_data.opcodes_1 = [
                    ((input_pc_col0)
                        + (((((next_pc_limb_0_col39) + ((next_pc_limb_1_col40) * (M31_512)))
                            + ((next_pc_limb_2_col41) * (M31_262144)))
                            - (msb_col37))
                            - ((M31_134217728) * (mid_limbs_set_col38)))),
                    ((input_ap_col1) + (ap_update_add_1_col4)),
                    input_fp_col2,
                ];
                *row[42] = padding_col.packed_at(row_index);
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 2]>,
    memory_address_to_id_1: Vec<[PackedM31; 2]>,
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
    memory_id_to_big_1: Vec<[PackedM31; 29]>,
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
            &self.lookup_data.opcodes_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
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
