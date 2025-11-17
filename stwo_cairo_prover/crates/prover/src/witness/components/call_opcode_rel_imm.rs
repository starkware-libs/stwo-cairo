// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::call_opcode_rel_imm::{Claim, InteractionClaim, N_TRACE_COLUMNS};

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
    let M31_134217728 = PackedM31::broadcast(M31::from(134217728));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_32769 = PackedM31::broadcast(M31::from(32769));
    let M31_508 = PackedM31::broadcast(M31::from(508));
    let M31_511 = PackedM31::broadcast(M31::from(511));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let M31_536870912 = PackedM31::broadcast(M31::from(536870912));
    let M31_68 = PackedM31::broadcast(M31::from(68));
    let UInt16_1 = PackedUInt16::broadcast(UInt16::from(1));
    let UInt16_2 = PackedUInt16::broadcast(UInt16::from(2));
    let UInt16_3 = PackedUInt16::broadcast(UInt16::from(3));
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
            |(row_index, (row, lookup_data, sub_component_inputs, call_opcode_rel_imm_input))| {
                let input_pc_col0 = call_opcode_rel_imm_input.pc;
                *row[0] = input_pc_col0;
                let input_ap_col1 = call_opcode_rel_imm_input.ap;
                *row[1] = input_ap_col1;
                let input_fp_col2 = call_opcode_rel_imm_input.fp;
                *row[2] = input_fp_col2;

                // Decode Instruction.

                let memory_address_to_id_value_tmp_9db06_0 =
                    memory_address_to_id_state.deduce_output(input_pc_col0);
                let memory_id_to_big_value_tmp_9db06_1 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_9db06_0);
                *sub_component_inputs.verify_instruction[0] = (
                    input_pc_col0,
                    [M31_32768, M31_32769, M31_32769],
                    [M31_32, M31_68],
                    M31_0,
                );
                *lookup_data.verify_instruction_0 = [
                    input_pc_col0,
                    M31_32768,
                    M31_32769,
                    M31_32769,
                    M31_32,
                    M31_68,
                    M31_0,
                ];
                let decode_instruction_2a7a2_output_tmp_9db06_2 = (
                    [M31_0, M31_1, M31_1],
                    [
                        M31_0, M31_0, M31_1, M31_0, M31_0, M31_0, M31_0, M31_0, M31_1, M31_0,
                        M31_0, M31_0, M31_1, M31_0, M31_0,
                    ],
                    M31_0,
                );

                // Read Positive Num Bits 29.

                // Read Id.

                let memory_address_to_id_value_tmp_9db06_3 =
                    memory_address_to_id_state.deduce_output(input_ap_col1);
                let stored_fp_id_col3 = memory_address_to_id_value_tmp_9db06_3;
                *row[3] = stored_fp_id_col3;
                *sub_component_inputs.memory_address_to_id[0] = input_ap_col1;
                *lookup_data.memory_address_to_id_0 = [input_ap_col1, stored_fp_id_col3];

                // Read Positive Known Id Num Bits 29.

                let memory_id_to_big_value_tmp_9db06_5 =
                    memory_id_to_big_state.deduce_output(stored_fp_id_col3);
                let stored_fp_limb_0_col4 = memory_id_to_big_value_tmp_9db06_5.get_m31(0);
                *row[4] = stored_fp_limb_0_col4;
                let stored_fp_limb_1_col5 = memory_id_to_big_value_tmp_9db06_5.get_m31(1);
                *row[5] = stored_fp_limb_1_col5;
                let stored_fp_limb_2_col6 = memory_id_to_big_value_tmp_9db06_5.get_m31(2);
                *row[6] = stored_fp_limb_2_col6;
                let stored_fp_limb_3_col7 = memory_id_to_big_value_tmp_9db06_5.get_m31(3);
                *row[7] = stored_fp_limb_3_col7;

                // Range Check Last Limb Bits In Ms Limb 2.

                // Cond Range Check 2.

                let partial_limb_msb_tmp_9db06_6 =
                    (((PackedUInt16::from_m31(stored_fp_limb_3_col7)) & (UInt16_2)) >> (UInt16_1));
                let partial_limb_msb_col8 = partial_limb_msb_tmp_9db06_6.as_m31();
                *row[8] = partial_limb_msb_col8;

                *sub_component_inputs.memory_id_to_big[0] = stored_fp_id_col3;
                *lookup_data.memory_id_to_big_0 = [
                    stored_fp_id_col3,
                    stored_fp_limb_0_col4,
                    stored_fp_limb_1_col5,
                    stored_fp_limb_2_col6,
                    stored_fp_limb_3_col7,
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
                let read_positive_known_id_num_bits_29_output_tmp_9db06_8 =
                    PackedFelt252::from_limbs([
                        stored_fp_limb_0_col4,
                        stored_fp_limb_1_col5,
                        stored_fp_limb_2_col6,
                        stored_fp_limb_3_col7,
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

                let read_positive_num_bits_29_output_tmp_9db06_9 = (
                    read_positive_known_id_num_bits_29_output_tmp_9db06_8,
                    stored_fp_id_col3,
                );

                // Read Positive Num Bits 29.

                // Read Id.

                let memory_address_to_id_value_tmp_9db06_10 =
                    memory_address_to_id_state.deduce_output(((input_ap_col1) + (M31_1)));
                let stored_ret_pc_id_col9 = memory_address_to_id_value_tmp_9db06_10;
                *row[9] = stored_ret_pc_id_col9;
                *sub_component_inputs.memory_address_to_id[1] = ((input_ap_col1) + (M31_1));
                *lookup_data.memory_address_to_id_1 =
                    [((input_ap_col1) + (M31_1)), stored_ret_pc_id_col9];

                // Read Positive Known Id Num Bits 29.

                let memory_id_to_big_value_tmp_9db06_12 =
                    memory_id_to_big_state.deduce_output(stored_ret_pc_id_col9);
                let stored_ret_pc_limb_0_col10 = memory_id_to_big_value_tmp_9db06_12.get_m31(0);
                *row[10] = stored_ret_pc_limb_0_col10;
                let stored_ret_pc_limb_1_col11 = memory_id_to_big_value_tmp_9db06_12.get_m31(1);
                *row[11] = stored_ret_pc_limb_1_col11;
                let stored_ret_pc_limb_2_col12 = memory_id_to_big_value_tmp_9db06_12.get_m31(2);
                *row[12] = stored_ret_pc_limb_2_col12;
                let stored_ret_pc_limb_3_col13 = memory_id_to_big_value_tmp_9db06_12.get_m31(3);
                *row[13] = stored_ret_pc_limb_3_col13;

                // Range Check Last Limb Bits In Ms Limb 2.

                // Cond Range Check 2.

                let partial_limb_msb_tmp_9db06_13 =
                    (((PackedUInt16::from_m31(stored_ret_pc_limb_3_col13)) & (UInt16_2))
                        >> (UInt16_1));
                let partial_limb_msb_col14 = partial_limb_msb_tmp_9db06_13.as_m31();
                *row[14] = partial_limb_msb_col14;

                *sub_component_inputs.memory_id_to_big[1] = stored_ret_pc_id_col9;
                *lookup_data.memory_id_to_big_1 = [
                    stored_ret_pc_id_col9,
                    stored_ret_pc_limb_0_col10,
                    stored_ret_pc_limb_1_col11,
                    stored_ret_pc_limb_2_col12,
                    stored_ret_pc_limb_3_col13,
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
                let read_positive_known_id_num_bits_29_output_tmp_9db06_15 =
                    PackedFelt252::from_limbs([
                        stored_ret_pc_limb_0_col10,
                        stored_ret_pc_limb_1_col11,
                        stored_ret_pc_limb_2_col12,
                        stored_ret_pc_limb_3_col13,
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

                let read_positive_num_bits_29_output_tmp_9db06_16 = (
                    read_positive_known_id_num_bits_29_output_tmp_9db06_15,
                    stored_ret_pc_id_col9,
                );

                // Read Small.

                // Read Id.

                let memory_address_to_id_value_tmp_9db06_17 =
                    memory_address_to_id_state.deduce_output(((input_pc_col0) + (M31_1)));
                let distance_to_next_pc_id_col15 = memory_address_to_id_value_tmp_9db06_17;
                *row[15] = distance_to_next_pc_id_col15;
                *sub_component_inputs.memory_address_to_id[2] = ((input_pc_col0) + (M31_1));
                *lookup_data.memory_address_to_id_2 =
                    [((input_pc_col0) + (M31_1)), distance_to_next_pc_id_col15];

                let memory_id_to_big_value_tmp_9db06_19 =
                    memory_id_to_big_state.deduce_output(distance_to_next_pc_id_col15);

                // Decode Small Sign.

                let msb_tmp_9db06_20 = memory_id_to_big_value_tmp_9db06_19.get_m31(27).eq(M31_256);
                let msb_col16 = msb_tmp_9db06_20.as_m31();
                *row[16] = msb_col16;
                let mid_limbs_set_tmp_9db06_21 =
                    ((memory_id_to_big_value_tmp_9db06_19.get_m31(20).eq(M31_511))
                        & (msb_tmp_9db06_20));
                let mid_limbs_set_col17 = mid_limbs_set_tmp_9db06_21.as_m31();
                *row[17] = mid_limbs_set_col17;
                let decode_small_sign_output_tmp_9db06_22 = [msb_col16, mid_limbs_set_col17];

                let distance_to_next_pc_limb_0_col18 =
                    memory_id_to_big_value_tmp_9db06_19.get_m31(0);
                *row[18] = distance_to_next_pc_limb_0_col18;
                let distance_to_next_pc_limb_1_col19 =
                    memory_id_to_big_value_tmp_9db06_19.get_m31(1);
                *row[19] = distance_to_next_pc_limb_1_col19;
                let distance_to_next_pc_limb_2_col20 =
                    memory_id_to_big_value_tmp_9db06_19.get_m31(2);
                *row[20] = distance_to_next_pc_limb_2_col20;
                let remainder_bits_tmp_9db06_23 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_9db06_19.get_m31(3)))
                        & (UInt16_3));
                let remainder_bits_col21 = remainder_bits_tmp_9db06_23.as_m31();
                *row[21] = remainder_bits_col21;

                // Cond Range Check 2.

                let partial_limb_msb_tmp_9db06_24 =
                    (((PackedUInt16::from_m31(remainder_bits_col21)) & (UInt16_2)) >> (UInt16_1));
                let partial_limb_msb_col22 = partial_limb_msb_tmp_9db06_24.as_m31();
                *row[22] = partial_limb_msb_col22;

                *sub_component_inputs.memory_id_to_big[2] = distance_to_next_pc_id_col15;
                *lookup_data.memory_id_to_big_2 = [
                    distance_to_next_pc_id_col15,
                    distance_to_next_pc_limb_0_col18,
                    distance_to_next_pc_limb_1_col19,
                    distance_to_next_pc_limb_2_col20,
                    ((remainder_bits_col21) + ((mid_limbs_set_col17) * (M31_508))),
                    ((mid_limbs_set_col17) * (M31_511)),
                    ((mid_limbs_set_col17) * (M31_511)),
                    ((mid_limbs_set_col17) * (M31_511)),
                    ((mid_limbs_set_col17) * (M31_511)),
                    ((mid_limbs_set_col17) * (M31_511)),
                    ((mid_limbs_set_col17) * (M31_511)),
                    ((mid_limbs_set_col17) * (M31_511)),
                    ((mid_limbs_set_col17) * (M31_511)),
                    ((mid_limbs_set_col17) * (M31_511)),
                    ((mid_limbs_set_col17) * (M31_511)),
                    ((mid_limbs_set_col17) * (M31_511)),
                    ((mid_limbs_set_col17) * (M31_511)),
                    ((mid_limbs_set_col17) * (M31_511)),
                    ((mid_limbs_set_col17) * (M31_511)),
                    ((mid_limbs_set_col17) * (M31_511)),
                    ((mid_limbs_set_col17) * (M31_511)),
                    ((mid_limbs_set_col17) * (M31_511)),
                    (((M31_136) * (msb_col16)) - (mid_limbs_set_col17)),
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    ((msb_col16) * (M31_256)),
                ];
                let read_small_output_tmp_9db06_26 = (
                    ((((((distance_to_next_pc_limb_0_col18)
                        + ((distance_to_next_pc_limb_1_col19) * (M31_512)))
                        + ((distance_to_next_pc_limb_2_col20) * (M31_262144)))
                        + ((remainder_bits_col21) * (M31_134217728)))
                        - (msb_col16))
                        - ((M31_536870912) * (mid_limbs_set_col17))),
                    distance_to_next_pc_id_col15,
                );

                *lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];
                *lookup_data.opcodes_1 = [
                    ((input_pc_col0) + (read_small_output_tmp_9db06_26.0)),
                    ((input_ap_col1) + (M31_2)),
                    ((input_ap_col1) + (M31_2)),
                ];
                *row[23] = enabler_col.packed_at(row_index);
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
