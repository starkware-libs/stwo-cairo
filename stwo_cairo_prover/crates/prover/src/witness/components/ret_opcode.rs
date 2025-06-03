#![allow(unused_parens)]
use cairo_air::components::ret_opcode::{Claim, InteractionClaim, N_TRACE_COLUMNS};

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
    memory_address_to_id: [Vec<PackedRelocatable>; 2],
    memory_id_to_big: [Vec<memory_id_to_big::PackedInputType>; 2],
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
    let M31_130 = PackedM31::broadcast(M31::from(130));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_2147483645 = PackedM31::broadcast(M31::from(2147483645));
    let M31_2147483646 = PackedM31::broadcast(M31::from(2147483646));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32766 = PackedM31::broadcast(M31::from(32766));
    let M31_32767 = PackedM31::broadcast(M31::from(32767));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let M31_88 = PackedM31::broadcast(M31::from(88));
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
            |(row_index, (mut row, lookup_data, sub_component_inputs, ret_opcode_input))| {
                let input_pc_col0 = ret_opcode_input.pc;
                *row[0] = input_pc_col0;
                let input_ap_col1 = ret_opcode_input.ap;
                *row[1] = input_ap_col1;
                let input_fp_col2 = ret_opcode_input.fp;
                *row[2] = input_fp_col2;

                // Decode Instruction.

                let memory_address_to_id_value_tmp_e23a5_0 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_pc_m31(input_pc_col0));
                let memory_id_to_big_value_tmp_e23a5_1 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_e23a5_0);
                *sub_component_inputs.verify_instruction[0] = (
                    input_pc_col0,
                    [M31_32766, M31_32767, M31_32767],
                    [M31_88, M31_130],
                    M31_0,
                );
                *lookup_data.verify_instruction_0 = [
                    input_pc_col0,
                    M31_32766,
                    M31_32767,
                    M31_32767,
                    M31_88,
                    M31_130,
                    M31_0,
                ];
                let decode_instruction_e46923e076e93350_output_tmp_e23a5_2 = (
                    [M31_2147483645, M31_2147483646, M31_2147483646],
                    [
                        M31_1, M31_1, M31_0, M31_1, M31_0, M31_0, M31_0, M31_1, M31_0, M31_0,
                        M31_0, M31_0, M31_0, M31_1, M31_0,
                    ],
                    M31_0,
                );

                // Read Positive Num Bits 27.

                let memory_address_to_id_value_tmp_e23a5_3 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(((input_fp_col2) - (M31_1))));
                let memory_id_to_big_value_tmp_e23a5_4 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_e23a5_3);
                let next_pc_id_col3 = memory_address_to_id_value_tmp_e23a5_3;
                *row[3] = next_pc_id_col3;
                *sub_component_inputs.memory_address_to_id[0] =
                    PackedRelocatable::from_ap_m31(((input_fp_col2) - (M31_1)));
                *lookup_data.memory_address_to_id_0 =
                    [M31_1, ((input_fp_col2) - (M31_1)), next_pc_id_col3];
                let next_pc_limb_0_col4 = memory_id_to_big_value_tmp_e23a5_4.get_m31(0);
                *row[4] = next_pc_limb_0_col4;
                let next_pc_limb_1_col5 = memory_id_to_big_value_tmp_e23a5_4.get_m31(1);
                *row[5] = next_pc_limb_1_col5;
                let next_pc_limb_2_col6 = memory_id_to_big_value_tmp_e23a5_4.get_m31(2);
                *row[6] = next_pc_limb_2_col6;
                *sub_component_inputs.memory_id_to_big[0] = next_pc_id_col3;
                *lookup_data.memory_id_to_big_0 = [
                    next_pc_id_col3,
                    next_pc_limb_0_col4,
                    next_pc_limb_1_col5,
                    next_pc_limb_2_col6,
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
                    M31_0,
                ];
                let read_positive_num_bits_27_output_tmp_e23a5_5 = (
                    PackedFelt252::from_limbs([
                        next_pc_limb_0_col4,
                        next_pc_limb_1_col5,
                        next_pc_limb_2_col6,
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
                        M31_0,
                    ]),
                    next_pc_id_col3,
                );

                // Read Positive Num Bits 27.

                let memory_address_to_id_value_tmp_e23a5_6 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable::from_ap_m31(((input_fp_col2) - (M31_2))));
                let memory_id_to_big_value_tmp_e23a5_7 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_e23a5_6);
                let next_fp_id_col7 = memory_address_to_id_value_tmp_e23a5_6;
                *row[7] = next_fp_id_col7;
                *sub_component_inputs.memory_address_to_id[1] =
                    PackedRelocatable::from_ap_m31(((input_fp_col2) - (M31_2)));
                *lookup_data.memory_address_to_id_1 =
                    [M31_1, ((input_fp_col2) - (M31_2)), next_fp_id_col7];
                let next_fp_limb_0_col8 = memory_id_to_big_value_tmp_e23a5_7.get_m31(0);
                *row[8] = next_fp_limb_0_col8;
                let next_fp_limb_1_col9 = memory_id_to_big_value_tmp_e23a5_7.get_m31(1);
                *row[9] = next_fp_limb_1_col9;
                let next_fp_limb_2_col10 = memory_id_to_big_value_tmp_e23a5_7.get_m31(2);
                *row[10] = next_fp_limb_2_col10;
                let next_fp_limb_3_col11 = memory_id_to_big_value_tmp_e23a5_7.get_m31(3);
                *row[12] = next_fp_limb_3_col11;
                let next_fp_limb_4_col12 = memory_id_to_big_value_tmp_e23a5_7.get_m31(4);
                *row[13] = next_fp_limb_4_col12;
                let next_fp_limb_5_col13 = memory_id_to_big_value_tmp_e23a5_7.get_m31(5);
                *row[14] = next_fp_limb_5_col13;
                let next_fp_limb_6_col14 = memory_id_to_big_value_tmp_e23a5_7.get_m31(6);
                *row[15] = next_fp_limb_6_col14;
                let next_fp_limb_7_col15 = memory_id_to_big_value_tmp_e23a5_7.get_m31(7);
                *row[16] = next_fp_limb_7_col15;
                *sub_component_inputs.memory_id_to_big[1] = next_fp_id_col7;
                *lookup_data.memory_id_to_big_1 = [
                    next_fp_id_col7,
                    next_fp_limb_0_col8,
                    next_fp_limb_1_col9,
                    next_fp_limb_2_col10,
                    next_fp_limb_3_col11,
                    next_fp_limb_4_col12,
                    next_fp_limb_5_col13,
                    next_fp_limb_6_col14,
                    next_fp_limb_7_col15,
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
                let read_positive_num_bits_27_output_tmp_e23a5_8 = (
                    PackedFelt252::from_limbs([
                        next_fp_limb_0_col8,
                        next_fp_limb_1_col9,
                        next_fp_limb_2_col10,
                        next_fp_limb_3_col11,
                        next_fp_limb_4_col12,
                        next_fp_limb_5_col13,
                        next_fp_limb_6_col14,
                        next_fp_limb_7_col15,
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
                    ]),
                    next_fp_id_col7,
                );

                *lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];
                *lookup_data.opcodes_1 = [
                    (((next_pc_limb_0_col4) + ((next_pc_limb_1_col5) * (M31_512)))
                        + ((next_pc_limb_2_col6) * (M31_262144))),
                    input_ap_col1,
                    (((next_fp_limb_0_col8) + ((next_fp_limb_1_col9) * (M31_512)))
                        + ((next_fp_limb_2_col10) * (M31_262144))),
                ];
                *row[11] = enabler_col.packed_at(row_index);
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 3]>,
    memory_address_to_id_1: Vec<[PackedM31; 3]>,
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
