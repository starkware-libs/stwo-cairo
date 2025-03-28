#![allow(unused_parens)]
use super::component::{Claim, InteractionClaim, N_TRACE_COLUMNS};
use crate::components::prelude::proving::*;
use crate::components::{
    memory_address_to_id, memory_id_to_big, range_check_11, verify_instruction,
};

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
        range_check_11_state: &range_check_11::ClaimGenerator,
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
            range_check_11_state,
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
    n_rows: usize,
    inputs: Vec<PackedInputType>,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
    range_check_11_state: &range_check_11::ClaimGenerator,
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
    let M31_16 = PackedM31::broadcast(M31::from(16));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_32769 = PackedM31::broadcast(M31::from(32769));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let M31_8 = PackedM31::broadcast(M31::from(8));
    let M31_8192 = PackedM31::broadcast(M31::from(8192));
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
            |(
                row_index,
                (mut row, lookup_data, sub_component_inputs, mul_opcode_small_imm_input),
            )| {
                let input_pc_col0 = mul_opcode_small_imm_input.pc;
                *row[0] = input_pc_col0;
                let input_ap_col1 = mul_opcode_small_imm_input.ap;
                *row[1] = input_ap_col1;
                let input_fp_col2 = mul_opcode_small_imm_input.fp;
                *row[2] = input_fp_col2;

                // Decode Instruction.

                let memory_address_to_id_value_tmp_3f47f_0 =
                    memory_address_to_id_state.deduce_output(input_pc_col0);
                let memory_id_to_big_value_tmp_3f47f_1 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_3f47f_0);
                let offset0_tmp_3f47f_2 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_3f47f_1.get_m31(0)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_3f47f_1.get_m31(1),
                        )) & (UInt16_127))
                            << (UInt16_9)));
                let offset0_col3 = offset0_tmp_3f47f_2.as_m31();
                *row[3] = offset0_col3;
                let offset1_tmp_3f47f_3 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_3f47f_1.get_m31(1)))
                        >> (UInt16_7))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_3f47f_1.get_m31(2),
                        )) << (UInt16_2)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_3f47f_1.get_m31(3),
                        )) & (UInt16_31))
                            << (UInt16_11)));
                let offset1_col4 = offset1_tmp_3f47f_3.as_m31();
                *row[4] = offset1_col4;
                let dst_base_fp_tmp_3f47f_4 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_3f47f_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_3f47f_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_0))
                        & (UInt16_1));
                let dst_base_fp_col5 = dst_base_fp_tmp_3f47f_4.as_m31();
                *row[5] = dst_base_fp_col5;
                let op0_base_fp_tmp_3f47f_5 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_3f47f_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_3f47f_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_1))
                        & (UInt16_1));
                let op0_base_fp_col6 = op0_base_fp_tmp_3f47f_5.as_m31();
                *row[6] = op0_base_fp_col6;
                let ap_update_add_1_tmp_3f47f_6 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_3f47f_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_3f47f_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_11))
                        & (UInt16_1));
                let ap_update_add_1_col7 = ap_update_add_1_tmp_3f47f_6.as_m31();
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

                let mem_dst_base_col8 = (((dst_base_fp_col5) * (input_fp_col2))
                    + (((M31_1) - (dst_base_fp_col5)) * (input_ap_col1)));
                *row[8] = mem_dst_base_col8;
                let mem0_base_col9 = (((op0_base_fp_col6) * (input_fp_col2))
                    + (((M31_1) - (op0_base_fp_col6)) * (input_ap_col1)));
                *row[9] = mem0_base_col9;

                // Read Positive Num Bits 72.

                let memory_address_to_id_value_tmp_3f47f_7 = memory_address_to_id_state
                    .deduce_output(((mem_dst_base_col8) + ((offset0_col3) - (M31_32768))));
                let memory_id_to_big_value_tmp_3f47f_8 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_3f47f_7);
                let dst_id_col10 = memory_address_to_id_value_tmp_3f47f_7;
                *row[10] = dst_id_col10;
                *sub_component_inputs.memory_address_to_id[0] =
                    ((mem_dst_base_col8) + ((offset0_col3) - (M31_32768)));
                *lookup_data.memory_address_to_id_0 = [
                    ((mem_dst_base_col8) + ((offset0_col3) - (M31_32768))),
                    dst_id_col10,
                ];
                let dst_limb_0_col11 = memory_id_to_big_value_tmp_3f47f_8.get_m31(0);
                *row[11] = dst_limb_0_col11;
                let dst_limb_1_col12 = memory_id_to_big_value_tmp_3f47f_8.get_m31(1);
                *row[12] = dst_limb_1_col12;
                let dst_limb_2_col13 = memory_id_to_big_value_tmp_3f47f_8.get_m31(2);
                *row[13] = dst_limb_2_col13;
                let dst_limb_3_col14 = memory_id_to_big_value_tmp_3f47f_8.get_m31(3);
                *row[14] = dst_limb_3_col14;
                let dst_limb_4_col15 = memory_id_to_big_value_tmp_3f47f_8.get_m31(4);
                *row[15] = dst_limb_4_col15;
                let dst_limb_5_col16 = memory_id_to_big_value_tmp_3f47f_8.get_m31(5);
                *row[16] = dst_limb_5_col16;
                let dst_limb_6_col17 = memory_id_to_big_value_tmp_3f47f_8.get_m31(6);
                *row[17] = dst_limb_6_col17;
                let dst_limb_7_col18 = memory_id_to_big_value_tmp_3f47f_8.get_m31(7);
                *row[18] = dst_limb_7_col18;
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

                // Read Positive Num Bits 36.

                let memory_address_to_id_value_tmp_3f47f_9 = memory_address_to_id_state
                    .deduce_output(((mem0_base_col9) + ((offset1_col4) - (M31_32768))));
                let memory_id_to_big_value_tmp_3f47f_10 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_3f47f_9);
                let op0_id_col19 = memory_address_to_id_value_tmp_3f47f_9;
                *row[19] = op0_id_col19;
                *sub_component_inputs.memory_address_to_id[1] =
                    ((mem0_base_col9) + ((offset1_col4) - (M31_32768)));
                *lookup_data.memory_address_to_id_1 = [
                    ((mem0_base_col9) + ((offset1_col4) - (M31_32768))),
                    op0_id_col19,
                ];
                let op0_limb_0_col20 = memory_id_to_big_value_tmp_3f47f_10.get_m31(0);
                *row[20] = op0_limb_0_col20;
                let op0_limb_1_col21 = memory_id_to_big_value_tmp_3f47f_10.get_m31(1);
                *row[21] = op0_limb_1_col21;
                let op0_limb_2_col22 = memory_id_to_big_value_tmp_3f47f_10.get_m31(2);
                *row[22] = op0_limb_2_col22;
                let op0_limb_3_col23 = memory_id_to_big_value_tmp_3f47f_10.get_m31(3);
                *row[23] = op0_limb_3_col23;
                *sub_component_inputs.memory_id_to_big[1] = op0_id_col19;
                *lookup_data.memory_id_to_big_1 = [
                    op0_id_col19,
                    op0_limb_0_col20,
                    op0_limb_1_col21,
                    op0_limb_2_col22,
                    op0_limb_3_col23,
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

                // Read Positive Num Bits 36.

                let memory_address_to_id_value_tmp_3f47f_11 =
                    memory_address_to_id_state.deduce_output(((input_pc_col0) + (M31_1)));
                let memory_id_to_big_value_tmp_3f47f_12 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_3f47f_11);
                let op1_id_col24 = memory_address_to_id_value_tmp_3f47f_11;
                *row[24] = op1_id_col24;
                *sub_component_inputs.memory_address_to_id[2] = ((input_pc_col0) + (M31_1));
                *lookup_data.memory_address_to_id_2 = [((input_pc_col0) + (M31_1)), op1_id_col24];
                let op1_limb_0_col25 = memory_id_to_big_value_tmp_3f47f_12.get_m31(0);
                *row[25] = op1_limb_0_col25;
                let op1_limb_1_col26 = memory_id_to_big_value_tmp_3f47f_12.get_m31(1);
                *row[26] = op1_limb_1_col26;
                let op1_limb_2_col27 = memory_id_to_big_value_tmp_3f47f_12.get_m31(2);
                *row[27] = op1_limb_2_col27;
                let op1_limb_3_col28 = memory_id_to_big_value_tmp_3f47f_12.get_m31(3);
                *row[28] = op1_limb_3_col28;
                *sub_component_inputs.memory_id_to_big[2] = op1_id_col24;
                *lookup_data.memory_id_to_big_2 = [
                    op1_id_col24,
                    op1_limb_0_col25,
                    op1_limb_1_col26,
                    op1_limb_2_col27,
                    op1_limb_3_col28,
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

                // Verify Mul Small.

                let carry_1_col29 = (((((((op0_limb_0_col20) * (op1_limb_0_col25))
                    - (dst_limb_0_col11))
                    + (((op0_limb_0_col20) * (op1_limb_1_col26)) * (M31_512)))
                    + (((op0_limb_1_col21) * (op1_limb_0_col25)) * (M31_512)))
                    - ((dst_limb_1_col12) * (M31_512)))
                    * (M31_8192));
                *row[29] = carry_1_col29;
                *sub_component_inputs.range_check_11[0] = [carry_1_col29];
                *lookup_data.range_check_11_0 = [carry_1_col29];
                let carry_3_col30 = (((((((((((carry_1_col29)
                    + ((op0_limb_0_col20) * (op1_limb_2_col27)))
                    + ((op0_limb_1_col21) * (op1_limb_1_col26)))
                    + ((op0_limb_2_col22) * (op1_limb_0_col25)))
                    - (dst_limb_2_col13))
                    + (((op0_limb_0_col20) * (op1_limb_3_col28)) * (M31_512)))
                    + (((op0_limb_1_col21) * (op1_limb_2_col27)) * (M31_512)))
                    + (((op0_limb_2_col22) * (op1_limb_1_col26)) * (M31_512)))
                    + (((op0_limb_3_col23) * (op1_limb_0_col25)) * (M31_512)))
                    - ((dst_limb_3_col14) * (M31_512)))
                    * (M31_8192));
                *row[30] = carry_3_col30;
                *sub_component_inputs.range_check_11[1] = [carry_3_col30];
                *lookup_data.range_check_11_1 = [carry_3_col30];
                let carry_5_col31 = (((((((((carry_3_col30)
                    + ((op0_limb_1_col21) * (op1_limb_3_col28)))
                    + ((op0_limb_2_col22) * (op1_limb_2_col27)))
                    + ((op0_limb_3_col23) * (op1_limb_1_col26)))
                    - (dst_limb_4_col15))
                    + (((op0_limb_2_col22) * (op1_limb_3_col28)) * (M31_512)))
                    + (((op0_limb_3_col23) * (op1_limb_2_col27)) * (M31_512)))
                    - ((dst_limb_5_col16) * (M31_512)))
                    * (M31_8192));
                *row[31] = carry_5_col31;
                *sub_component_inputs.range_check_11[2] = [carry_5_col31];
                *lookup_data.range_check_11_2 = [carry_5_col31];

                *lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];
                *lookup_data.opcodes_1 = [
                    ((input_pc_col0) + (M31_2)),
                    ((input_ap_col1) + (ap_update_add_1_col7)),
                    input_fp_col2,
                ];
                *row[32] = padding_col.packed_at(row_index);
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
        memory_address_to_id: &relations::MemoryAddressToId,
        memory_id_to_big: &relations::MemoryIdToBig,
        opcodes: &relations::Opcodes,
        range_check_11: &relations::RangeCheck_11,
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
            &self.lookup_data.range_check_11_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = range_check_11.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_11_1,
            &self.lookup_data.range_check_11_2,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_11.combine(values0);
            let denom1: PackedQM31 = range_check_11.combine(values1);
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
