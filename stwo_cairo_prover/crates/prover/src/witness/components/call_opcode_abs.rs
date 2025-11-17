// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::call_opcode_abs::{Claim, InteractionClaim, N_TRACE_COLUMNS};

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
    let M31_134217728 = PackedM31::broadcast(M31::from(134217728));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_32769 = PackedM31::broadcast(M31::from(32769));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let M31_64 = PackedM31::broadcast(M31::from(64));
    let M31_66 = PackedM31::broadcast(M31::from(66));
    let UInt16_1 = PackedUInt16::broadcast(UInt16::from(1));
    let UInt16_13 = PackedUInt16::broadcast(UInt16::from(13));
    let UInt16_2 = PackedUInt16::broadcast(UInt16::from(2));
    let UInt16_3 = PackedUInt16::broadcast(UInt16::from(3));
    let UInt16_4 = PackedUInt16::broadcast(UInt16::from(4));
    let UInt16_5 = PackedUInt16::broadcast(UInt16::from(5));
    let UInt16_6 = PackedUInt16::broadcast(UInt16::from(6));
    let UInt16_7 = PackedUInt16::broadcast(UInt16::from(7));
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
            |(row_index, (row, lookup_data, sub_component_inputs, call_opcode_abs_input))| {
                let input_pc_col0 = call_opcode_abs_input.pc;
                *row[0] = input_pc_col0;
                let input_ap_col1 = call_opcode_abs_input.ap;
                *row[1] = input_ap_col1;
                let input_fp_col2 = call_opcode_abs_input.fp;
                *row[2] = input_fp_col2;

                // Decode Instruction.

                let memory_address_to_id_value_tmp_32b66_0 =
                    memory_address_to_id_state.deduce_output(input_pc_col0);
                let memory_id_to_big_value_tmp_32b66_1 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_32b66_0);
                let offset2_tmp_32b66_2 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_32b66_1.get_m31(3)))
                        >> (UInt16_5))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_32b66_1.get_m31(4),
                        )) << (UInt16_4)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_32b66_1.get_m31(5),
                        )) & (UInt16_7))
                            << (UInt16_13)));
                let offset2_col3 = offset2_tmp_32b66_2.as_m31();
                *row[3] = offset2_col3;
                let op1_base_fp_tmp_32b66_3 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_32b66_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_32b66_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_3))
                        & (UInt16_1));
                let op1_base_fp_col4 = op1_base_fp_tmp_32b66_3.as_m31();
                *row[4] = op1_base_fp_col4;
                *sub_component_inputs.verify_instruction[0] = (
                    input_pc_col0,
                    [M31_32768, M31_32769, offset2_col3],
                    [
                        (((op1_base_fp_col4) * (M31_64))
                            + (((M31_1) - (op1_base_fp_col4)) * (M31_128))),
                        M31_66,
                    ],
                    M31_0,
                );
                *lookup_data.verify_instruction_0 = [
                    input_pc_col0,
                    M31_32768,
                    M31_32769,
                    offset2_col3,
                    (((op1_base_fp_col4) * (M31_64))
                        + (((M31_1) - (op1_base_fp_col4)) * (M31_128))),
                    M31_66,
                    M31_0,
                ];
                let decode_instruction_f1edd_output_tmp_32b66_4 = (
                    [M31_0, M31_1, ((offset2_col3) - (M31_32768))],
                    [
                        M31_0,
                        M31_0,
                        M31_0,
                        op1_base_fp_col4,
                        ((M31_1) - (op1_base_fp_col4)),
                        M31_0,
                        M31_0,
                        M31_1,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_1,
                        M31_0,
                        M31_0,
                    ],
                    M31_0,
                );

                // Read Positive Num Bits 29.

                // Read Id.

                let memory_address_to_id_value_tmp_32b66_5 =
                    memory_address_to_id_state.deduce_output(input_ap_col1);
                let stored_fp_id_col5 = memory_address_to_id_value_tmp_32b66_5;
                *row[5] = stored_fp_id_col5;
                *sub_component_inputs.memory_address_to_id[0] = input_ap_col1;
                *lookup_data.memory_address_to_id_0 = [input_ap_col1, stored_fp_id_col5];

                // Read Positive Known Id Num Bits 29.

                let memory_id_to_big_value_tmp_32b66_7 =
                    memory_id_to_big_state.deduce_output(stored_fp_id_col5);
                let stored_fp_limb_0_col6 = memory_id_to_big_value_tmp_32b66_7.get_m31(0);
                *row[6] = stored_fp_limb_0_col6;
                let stored_fp_limb_1_col7 = memory_id_to_big_value_tmp_32b66_7.get_m31(1);
                *row[7] = stored_fp_limb_1_col7;
                let stored_fp_limb_2_col8 = memory_id_to_big_value_tmp_32b66_7.get_m31(2);
                *row[8] = stored_fp_limb_2_col8;
                let stored_fp_limb_3_col9 = memory_id_to_big_value_tmp_32b66_7.get_m31(3);
                *row[9] = stored_fp_limb_3_col9;

                // Range Check Last Limb Bits In Ms Limb 2.

                // Cond Range Check 2.

                let partial_limb_msb_tmp_32b66_8 =
                    (((PackedUInt16::from_m31(stored_fp_limb_3_col9)) & (UInt16_2)) >> (UInt16_1));
                let partial_limb_msb_col10 = partial_limb_msb_tmp_32b66_8.as_m31();
                *row[10] = partial_limb_msb_col10;

                *sub_component_inputs.memory_id_to_big[0] = stored_fp_id_col5;
                *lookup_data.memory_id_to_big_0 = [
                    stored_fp_id_col5,
                    stored_fp_limb_0_col6,
                    stored_fp_limb_1_col7,
                    stored_fp_limb_2_col8,
                    stored_fp_limb_3_col9,
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
                let read_positive_known_id_num_bits_29_output_tmp_32b66_10 =
                    PackedFelt252::from_limbs([
                        stored_fp_limb_0_col6,
                        stored_fp_limb_1_col7,
                        stored_fp_limb_2_col8,
                        stored_fp_limb_3_col9,
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
                    ]);

                let read_positive_num_bits_29_output_tmp_32b66_11 = (
                    read_positive_known_id_num_bits_29_output_tmp_32b66_10,
                    stored_fp_id_col5,
                );

                // Read Positive Num Bits 29.

                // Read Id.

                let memory_address_to_id_value_tmp_32b66_12 =
                    memory_address_to_id_state.deduce_output(((input_ap_col1) + (M31_1)));
                let stored_ret_pc_id_col11 = memory_address_to_id_value_tmp_32b66_12;
                *row[11] = stored_ret_pc_id_col11;
                *sub_component_inputs.memory_address_to_id[1] = ((input_ap_col1) + (M31_1));
                *lookup_data.memory_address_to_id_1 =
                    [((input_ap_col1) + (M31_1)), stored_ret_pc_id_col11];

                // Read Positive Known Id Num Bits 29.

                let memory_id_to_big_value_tmp_32b66_14 =
                    memory_id_to_big_state.deduce_output(stored_ret_pc_id_col11);
                let stored_ret_pc_limb_0_col12 = memory_id_to_big_value_tmp_32b66_14.get_m31(0);
                *row[12] = stored_ret_pc_limb_0_col12;
                let stored_ret_pc_limb_1_col13 = memory_id_to_big_value_tmp_32b66_14.get_m31(1);
                *row[13] = stored_ret_pc_limb_1_col13;
                let stored_ret_pc_limb_2_col14 = memory_id_to_big_value_tmp_32b66_14.get_m31(2);
                *row[14] = stored_ret_pc_limb_2_col14;
                let stored_ret_pc_limb_3_col15 = memory_id_to_big_value_tmp_32b66_14.get_m31(3);
                *row[15] = stored_ret_pc_limb_3_col15;

                // Range Check Last Limb Bits In Ms Limb 2.

                // Cond Range Check 2.

                let partial_limb_msb_tmp_32b66_15 =
                    (((PackedUInt16::from_m31(stored_ret_pc_limb_3_col15)) & (UInt16_2))
                        >> (UInt16_1));
                let partial_limb_msb_col16 = partial_limb_msb_tmp_32b66_15.as_m31();
                *row[16] = partial_limb_msb_col16;

                *sub_component_inputs.memory_id_to_big[1] = stored_ret_pc_id_col11;
                *lookup_data.memory_id_to_big_1 = [
                    stored_ret_pc_id_col11,
                    stored_ret_pc_limb_0_col12,
                    stored_ret_pc_limb_1_col13,
                    stored_ret_pc_limb_2_col14,
                    stored_ret_pc_limb_3_col15,
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
                let read_positive_known_id_num_bits_29_output_tmp_32b66_17 =
                    PackedFelt252::from_limbs([
                        stored_ret_pc_limb_0_col12,
                        stored_ret_pc_limb_1_col13,
                        stored_ret_pc_limb_2_col14,
                        stored_ret_pc_limb_3_col15,
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
                    ]);

                let read_positive_num_bits_29_output_tmp_32b66_18 = (
                    read_positive_known_id_num_bits_29_output_tmp_32b66_17,
                    stored_ret_pc_id_col11,
                );

                let mem1_base_col17 = (((op1_base_fp_col4) * (input_fp_col2))
                    + ((decode_instruction_f1edd_output_tmp_32b66_4.1[4]) * (input_ap_col1)));
                *row[17] = mem1_base_col17;

                // Read Positive Num Bits 29.

                // Read Id.

                let memory_address_to_id_value_tmp_32b66_19 = memory_address_to_id_state
                    .deduce_output(
                        ((mem1_base_col17) + (decode_instruction_f1edd_output_tmp_32b66_4.0[2])),
                    );
                let next_pc_id_col18 = memory_address_to_id_value_tmp_32b66_19;
                *row[18] = next_pc_id_col18;
                *sub_component_inputs.memory_address_to_id[2] =
                    ((mem1_base_col17) + (decode_instruction_f1edd_output_tmp_32b66_4.0[2]));
                *lookup_data.memory_address_to_id_2 = [
                    ((mem1_base_col17) + (decode_instruction_f1edd_output_tmp_32b66_4.0[2])),
                    next_pc_id_col18,
                ];

                // Read Positive Known Id Num Bits 29.

                let memory_id_to_big_value_tmp_32b66_21 =
                    memory_id_to_big_state.deduce_output(next_pc_id_col18);
                let next_pc_limb_0_col19 = memory_id_to_big_value_tmp_32b66_21.get_m31(0);
                *row[19] = next_pc_limb_0_col19;
                let next_pc_limb_1_col20 = memory_id_to_big_value_tmp_32b66_21.get_m31(1);
                *row[20] = next_pc_limb_1_col20;
                let next_pc_limb_2_col21 = memory_id_to_big_value_tmp_32b66_21.get_m31(2);
                *row[21] = next_pc_limb_2_col21;
                let next_pc_limb_3_col22 = memory_id_to_big_value_tmp_32b66_21.get_m31(3);
                *row[22] = next_pc_limb_3_col22;

                // Range Check Last Limb Bits In Ms Limb 2.

                // Cond Range Check 2.

                let partial_limb_msb_tmp_32b66_22 =
                    (((PackedUInt16::from_m31(next_pc_limb_3_col22)) & (UInt16_2)) >> (UInt16_1));
                let partial_limb_msb_col23 = partial_limb_msb_tmp_32b66_22.as_m31();
                *row[23] = partial_limb_msb_col23;

                *sub_component_inputs.memory_id_to_big[2] = next_pc_id_col18;
                *lookup_data.memory_id_to_big_2 = [
                    next_pc_id_col18,
                    next_pc_limb_0_col19,
                    next_pc_limb_1_col20,
                    next_pc_limb_2_col21,
                    next_pc_limb_3_col22,
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
                let read_positive_known_id_num_bits_29_output_tmp_32b66_24 =
                    PackedFelt252::from_limbs([
                        next_pc_limb_0_col19,
                        next_pc_limb_1_col20,
                        next_pc_limb_2_col21,
                        next_pc_limb_3_col22,
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
                    ]);

                let read_positive_num_bits_29_output_tmp_32b66_25 = (
                    read_positive_known_id_num_bits_29_output_tmp_32b66_24,
                    next_pc_id_col18,
                );

                *lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];
                *lookup_data.opcodes_1 = [
                    ((((next_pc_limb_0_col19) + ((next_pc_limb_1_col20) * (M31_512)))
                        + ((next_pc_limb_2_col21) * (M31_262144)))
                        + ((next_pc_limb_3_col22) * (M31_134217728))),
                    ((input_ap_col1) + (M31_2)),
                    ((input_ap_col1) + (M31_2)),
                ];
                *row[24] = enabler_col.packed_at(row_index);
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
