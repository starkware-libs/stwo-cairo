#![allow(unused_parens)]
use cairo_air::components::add_ap_opcode::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{
    memory_address_to_id, memory_id_to_big, range_check_19, range_check_8, verify_instruction,
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
        range_check_8_state: &range_check_8::ClaimGenerator,
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
            range_check_8_state,
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
        sub_component_inputs
            .range_check_8
            .iter()
            .for_each(|inputs| {
                range_check_8_state.add_packed_inputs(inputs);
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
    memory_address_to_id: [Vec<memory_address_to_id::PackedInputType>; 1],
    memory_id_to_big: [Vec<memory_id_to_big::PackedInputType>; 1],
    range_check_19: [Vec<range_check_19::PackedInputType>; 1],
    range_check_8: [Vec<range_check_8::PackedInputType>; 1],
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
    range_check_8_state: &range_check_8::ClaimGenerator,
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
    let M31_2147483646 = PackedM31::broadcast(M31::from(2147483646));
    let M31_24 = PackedM31::broadcast(M31::from(24));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_32767 = PackedM31::broadcast(M31::from(32767));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_511 = PackedM31::broadcast(M31::from(511));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let M31_64 = PackedM31::broadcast(M31::from(64));
    let M31_8388608 = PackedM31::broadcast(M31::from(8388608));
    let UInt16_1 = PackedUInt16::broadcast(UInt16::from(1));
    let UInt16_13 = PackedUInt16::broadcast(UInt16::from(13));
    let UInt16_2 = PackedUInt16::broadcast(UInt16::from(2));
    let UInt16_3 = PackedUInt16::broadcast(UInt16::from(3));
    let UInt16_4 = PackedUInt16::broadcast(UInt16::from(4));
    let UInt16_5 = PackedUInt16::broadcast(UInt16::from(5));
    let UInt16_6 = PackedUInt16::broadcast(UInt16::from(6));
    let UInt16_7 = PackedUInt16::broadcast(UInt16::from(7));
    let UInt32_255 = PackedUInt32::broadcast(UInt32::from(255));
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
            |(row_index, (mut row, lookup_data, sub_component_inputs, add_ap_opcode_input))| {
                let input_pc_col0 = add_ap_opcode_input.pc;
                *row[0] = input_pc_col0;
                let input_ap_col1 = add_ap_opcode_input.ap;
                *row[1] = input_ap_col1;
                let input_fp_col2 = add_ap_opcode_input.fp;
                *row[2] = input_fp_col2;

                // Decode Instruction.

                let memory_address_to_id_value_tmp_c921e_0 =
                    memory_address_to_id_state.deduce_output(input_pc_col0);
                let memory_id_to_big_value_tmp_c921e_1 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c921e_0);
                let offset2_tmp_c921e_2 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_c921e_1.get_m31(3)))
                        >> (UInt16_5))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_c921e_1.get_m31(4),
                        )) << (UInt16_4)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_c921e_1.get_m31(5),
                        )) & (UInt16_7))
                            << (UInt16_13)));
                let offset2_col3 = offset2_tmp_c921e_2.as_m31();
                *row[3] = offset2_col3;
                let op1_imm_tmp_c921e_3 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_c921e_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_c921e_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_2))
                        & (UInt16_1));
                let op1_imm_col4 = op1_imm_tmp_c921e_3.as_m31();
                *row[4] = op1_imm_col4;
                let op1_base_fp_tmp_c921e_4 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_c921e_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_c921e_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_3))
                        & (UInt16_1));
                let op1_base_fp_col5 = op1_base_fp_tmp_c921e_4.as_m31();
                *row[5] = op1_base_fp_col5;
                *sub_component_inputs.verify_instruction[0] = (
                    input_pc_col0,
                    [M31_32767, M31_32767, offset2_col3],
                    [
                        ((((M31_24) + ((op1_imm_col4) * (M31_32)))
                            + ((op1_base_fp_col5) * (M31_64)))
                            + ((((M31_1) - (op1_imm_col4)) - (op1_base_fp_col5)) * (M31_128))),
                        M31_16,
                    ],
                    M31_0,
                );
                *lookup_data.verify_instruction_0 = [
                    input_pc_col0,
                    M31_32767,
                    M31_32767,
                    offset2_col3,
                    ((((M31_24) + ((op1_imm_col4) * (M31_32))) + ((op1_base_fp_col5) * (M31_64)))
                        + ((((M31_1) - (op1_imm_col4)) - (op1_base_fp_col5)) * (M31_128))),
                    M31_16,
                    M31_0,
                ];
                let decode_instruction_d2a10_output_tmp_c921e_5 = (
                    [
                        M31_2147483646,
                        M31_2147483646,
                        ((offset2_col3) - (M31_32768)),
                    ],
                    [
                        M31_1,
                        M31_1,
                        op1_imm_col4,
                        op1_base_fp_col5,
                        (((M31_1) - (op1_imm_col4)) - (op1_base_fp_col5)),
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_1,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                    ],
                    M31_0,
                );

                let mem1_base_col6 = ((((op1_imm_col4) * (input_pc_col0))
                    + ((op1_base_fp_col5) * (input_fp_col2)))
                    + ((decode_instruction_d2a10_output_tmp_c921e_5.1[4]) * (input_ap_col1)));
                *row[6] = mem1_base_col6;

                // Read Small.

                let memory_address_to_id_value_tmp_c921e_6 = memory_address_to_id_state
                    .deduce_output(
                        ((mem1_base_col6) + (decode_instruction_d2a10_output_tmp_c921e_5.0[2])),
                    );
                let memory_id_to_big_value_tmp_c921e_7 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c921e_6);
                let op1_id_col7 = memory_address_to_id_value_tmp_c921e_6;
                *row[7] = op1_id_col7;
                *sub_component_inputs.memory_address_to_id[0] =
                    ((mem1_base_col6) + (decode_instruction_d2a10_output_tmp_c921e_5.0[2]));
                *lookup_data.memory_address_to_id_0 = [
                    ((mem1_base_col6) + (decode_instruction_d2a10_output_tmp_c921e_5.0[2])),
                    op1_id_col7,
                ];

                // Cond Decode Small Sign.

                let msb_tmp_c921e_8 = memory_id_to_big_value_tmp_c921e_7.get_m31(27).eq(M31_256);
                let msb_col8 = msb_tmp_c921e_8.as_m31();
                *row[8] = msb_col8;
                let mid_limbs_set_tmp_c921e_9 =
                    memory_id_to_big_value_tmp_c921e_7.get_m31(20).eq(M31_511);
                let mid_limbs_set_col9 = mid_limbs_set_tmp_c921e_9.as_m31();
                *row[9] = mid_limbs_set_col9;
                let cond_decode_small_sign_output_tmp_c921e_10 = [msb_col8, mid_limbs_set_col9];

                let op1_limb_0_col10 = memory_id_to_big_value_tmp_c921e_7.get_m31(0);
                *row[10] = op1_limb_0_col10;
                let op1_limb_1_col11 = memory_id_to_big_value_tmp_c921e_7.get_m31(1);
                *row[11] = op1_limb_1_col11;
                let op1_limb_2_col12 = memory_id_to_big_value_tmp_c921e_7.get_m31(2);
                *row[12] = op1_limb_2_col12;
                *sub_component_inputs.memory_id_to_big[0] = op1_id_col7;
                *lookup_data.memory_id_to_big_0 = [
                    op1_id_col7,
                    op1_limb_0_col10,
                    op1_limb_1_col11,
                    op1_limb_2_col12,
                    ((mid_limbs_set_col9) * (M31_511)),
                    ((mid_limbs_set_col9) * (M31_511)),
                    ((mid_limbs_set_col9) * (M31_511)),
                    ((mid_limbs_set_col9) * (M31_511)),
                    ((mid_limbs_set_col9) * (M31_511)),
                    ((mid_limbs_set_col9) * (M31_511)),
                    ((mid_limbs_set_col9) * (M31_511)),
                    ((mid_limbs_set_col9) * (M31_511)),
                    ((mid_limbs_set_col9) * (M31_511)),
                    ((mid_limbs_set_col9) * (M31_511)),
                    ((mid_limbs_set_col9) * (M31_511)),
                    ((mid_limbs_set_col9) * (M31_511)),
                    ((mid_limbs_set_col9) * (M31_511)),
                    ((mid_limbs_set_col9) * (M31_511)),
                    ((mid_limbs_set_col9) * (M31_511)),
                    ((mid_limbs_set_col9) * (M31_511)),
                    ((mid_limbs_set_col9) * (M31_511)),
                    ((mid_limbs_set_col9) * (M31_511)),
                    (((M31_136) * (msb_col8)) - (mid_limbs_set_col9)),
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    ((msb_col8) * (M31_256)),
                ];
                let read_small_output_tmp_c921e_11 = (
                    (((((op1_limb_0_col10) + ((op1_limb_1_col11) * (M31_512)))
                        + ((op1_limb_2_col12) * (M31_262144)))
                        - (msb_col8))
                        - ((M31_134217728) * (mid_limbs_set_col9))),
                    op1_id_col7,
                );

                let next_ap_tmp_c921e_12 = ((input_ap_col1) + (read_small_output_tmp_c921e_11.0));
                let next_ap_bot8bits_u32_tmp_c921e_13 =
                    ((PackedUInt32::from_m31(next_ap_tmp_c921e_12)) & (UInt32_255));
                let next_ap_bot8bits_col13 = next_ap_bot8bits_u32_tmp_c921e_13.low().as_m31();
                *row[13] = next_ap_bot8bits_col13;
                *sub_component_inputs.range_check_19[0] =
                    [(((next_ap_tmp_c921e_12) - (next_ap_bot8bits_col13)) * (M31_8388608))];
                *lookup_data.range_check_19_0 =
                    [(((next_ap_tmp_c921e_12) - (next_ap_bot8bits_col13)) * (M31_8388608))];
                *sub_component_inputs.range_check_8[0] = [next_ap_bot8bits_col13];
                *lookup_data.range_check_8_0 = [next_ap_bot8bits_col13];
                *lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];
                *lookup_data.opcodes_1 = [
                    ((input_pc_col0) + ((M31_1) + (op1_imm_col4))),
                    next_ap_tmp_c921e_12,
                    input_fp_col2,
                ];
                *row[14] = enabler_col.packed_at(row_index);
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 2]>,
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
    opcodes_0: Vec<[PackedM31; 3]>,
    opcodes_1: Vec<[PackedM31; 3]>,
    range_check_19_0: Vec<[PackedM31; 1]>,
    range_check_8_0: Vec<[PackedM31; 1]>,
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
        range_check_19: &relations::RangeCheck_19,
        range_check_8: &relations::RangeCheck_8,
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
            &self.lookup_data.range_check_8_0,
            &self.lookup_data.opcodes_0,
        )
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (writer, values0, values1))| {
                let denom0: PackedQM31 = range_check_8.combine(values0);
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
