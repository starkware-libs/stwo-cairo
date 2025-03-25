#![allow(unused_parens)]
use super::component::{Claim, InteractionClaim, N_TRACE_COLUMNS};
use crate::components::prelude::proving::*;
use crate::components::{memory_address_to_id, memory_id_to_big, verify_instruction};

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
    memory_address_to_id: [Vec<memory_address_to_id::PackedInputType>; 1],
    memory_id_to_big: [Vec<memory_id_to_big::PackedInputType>; 1],
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
    let M31_128 = PackedM31::broadcast(M31::from(128));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_24 = PackedM31::broadcast(M31::from(24));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_32767 = PackedM31::broadcast(M31::from(32767));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let M31_64 = PackedM31::broadcast(M31::from(64));
    let UInt16_1 = PackedUInt16::broadcast(UInt16::from(1));
    let UInt16_11 = PackedUInt16::broadcast(UInt16::from(11));
    let UInt16_13 = PackedUInt16::broadcast(UInt16::from(13));
    let UInt16_3 = PackedUInt16::broadcast(UInt16::from(3));
    let UInt16_4 = PackedUInt16::broadcast(UInt16::from(4));
    let UInt16_5 = PackedUInt16::broadcast(UInt16::from(5));
    let UInt16_6 = PackedUInt16::broadcast(UInt16::from(6));
    let UInt16_7 = PackedUInt16::broadcast(UInt16::from(7));
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
            |(row_index, (mut row, lookup_data, sub_component_inputs, jump_opcode_input))| {
                let input_pc_col0 = jump_opcode_input.pc;
                *row[0] = input_pc_col0;
                let input_ap_col1 = jump_opcode_input.ap;
                *row[1] = input_ap_col1;
                let input_fp_col2 = jump_opcode_input.fp;
                *row[2] = input_fp_col2;

                // Decode Instruction.

                let memory_address_to_id_value_tmp_39ce3_0 =
                    memory_address_to_id_state.deduce_output(input_pc_col0);
                let memory_id_to_big_value_tmp_39ce3_1 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_39ce3_0);
                let offset2_tmp_39ce3_2 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_39ce3_1.get_m31(3)))
                        >> (UInt16_5))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_39ce3_1.get_m31(4),
                        )) << (UInt16_4)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_39ce3_1.get_m31(5),
                        )) & (UInt16_7))
                            << (UInt16_13)));
                let offset2_col3 = offset2_tmp_39ce3_2.as_m31();
                *row[3] = offset2_col3;
                let op1_base_fp_tmp_39ce3_3 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_39ce3_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_39ce3_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_3))
                        & (UInt16_1));
                let op1_base_fp_col4 = op1_base_fp_tmp_39ce3_3.as_m31();
                *row[4] = op1_base_fp_col4;
                let op1_base_ap_tmp_39ce3_4 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_39ce3_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_39ce3_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_4))
                        & (UInt16_1));
                let op1_base_ap_col5 = op1_base_ap_tmp_39ce3_4.as_m31();
                *row[5] = op1_base_ap_col5;
                let ap_update_add_1_tmp_39ce3_5 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_39ce3_1.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_39ce3_1.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_11))
                        & (UInt16_1));
                let ap_update_add_1_col6 = ap_update_add_1_tmp_39ce3_5.as_m31();
                *row[6] = ap_update_add_1_col6;
                *sub_component_inputs.verify_instruction[0] = (
                    input_pc_col0,
                    [M31_32767, M31_32767, offset2_col3],
                    [
                        (((M31_24) + ((op1_base_fp_col4) * (M31_64)))
                            + ((op1_base_ap_col5) * (M31_128))),
                        ((M31_2) + ((ap_update_add_1_col6) * (M31_32))),
                    ],
                    M31_0,
                );
                *lookup_data.verify_instruction_0 = [
                    input_pc_col0,
                    M31_32767,
                    M31_32767,
                    offset2_col3,
                    (((M31_24) + ((op1_base_fp_col4) * (M31_64)))
                        + ((op1_base_ap_col5) * (M31_128))),
                    ((M31_2) + ((ap_update_add_1_col6) * (M31_32))),
                    M31_0,
                ];

                let mem1_base_col7 = (((op1_base_fp_col4) * (input_fp_col2))
                    + ((op1_base_ap_col5) * (input_ap_col1)));
                *row[7] = mem1_base_col7;

                // Read Positive Num Bits 27.

                let memory_address_to_id_value_tmp_39ce3_6 = memory_address_to_id_state
                    .deduce_output(((mem1_base_col7) + ((offset2_col3) - (M31_32768))));
                let memory_id_to_big_value_tmp_39ce3_7 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_39ce3_6);
                let next_pc_id_col8 = memory_address_to_id_value_tmp_39ce3_6;
                *row[8] = next_pc_id_col8;
                *sub_component_inputs.memory_address_to_id[0] =
                    ((mem1_base_col7) + ((offset2_col3) - (M31_32768)));
                *lookup_data.memory_address_to_id_0 = [
                    ((mem1_base_col7) + ((offset2_col3) - (M31_32768))),
                    next_pc_id_col8,
                ];
                let next_pc_limb_0_col9 = memory_id_to_big_value_tmp_39ce3_7.get_m31(0);
                *row[9] = next_pc_limb_0_col9;
                let next_pc_limb_1_col10 = memory_id_to_big_value_tmp_39ce3_7.get_m31(1);
                *row[10] = next_pc_limb_1_col10;
                let next_pc_limb_2_col11 = memory_id_to_big_value_tmp_39ce3_7.get_m31(2);
                *row[11] = next_pc_limb_2_col11;
                *sub_component_inputs.memory_id_to_big[0] = next_pc_id_col8;
                *lookup_data.memory_id_to_big_0 = [
                    next_pc_id_col8,
                    next_pc_limb_0_col9,
                    next_pc_limb_1_col10,
                    next_pc_limb_2_col11,
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

                *lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];
                *lookup_data.opcodes_1 = [
                    (((next_pc_limb_0_col9) + ((next_pc_limb_1_col10) * (M31_512)))
                        + ((next_pc_limb_2_col11) * (M31_262144))),
                    ((input_ap_col1) + (ap_update_add_1_col6)),
                    input_fp_col2,
                ];
                *row[12] = padding_col.packed_at(row_index);
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
