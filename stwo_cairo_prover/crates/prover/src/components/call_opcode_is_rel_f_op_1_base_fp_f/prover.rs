#![allow(unused_parens)]
#![allow(unused_imports)]
use std::iter::zip;

use air_structs_derive::SubComponentInputs;
use itertools::{chain, zip_eq, Itertools};
use num_traits::{One, Zero};
use prover_types::cpu::*;
use prover_types::simd::*;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use stwo_air_utils::trace::component_trace::ComponentTrace;
use stwo_air_utils_derive::{IterMut, ParIterMut, Uninitialized};
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::constraint_framework::Relation;
use stwo_prover::core::air::Component;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::conversion::Unpack;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{BackendForChannel, Col, Column};
use stwo_prover::core::channel::{Channel, MerkleChannel};
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::utils::bit_reverse_coset_to_circle_domain_order;

use super::component::{Claim, InteractionClaim};
use crate::components::{memory_address_to_id, memory_id_to_big, pack_values, verify_instruction};
use crate::relations;

pub type InputType = CasmState;
pub type PackedInputType = PackedCasmState;
const N_TRACE_COLUMNS: usize = 16;

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
        verify_instruction_state: &mut verify_instruction::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let n_calls = self.inputs.len();
        assert_ne!(n_calls, 0);
        let size = std::cmp::max(n_calls.next_power_of_two(), N_LANES);
        let need_padding = n_calls != size;

        if need_padding {
            self.inputs.resize(size, *self.inputs.first().unwrap());
            bit_reverse_coset_to_circle_domain_order(&mut self.inputs);
        }

        let packed_inputs = pack_values(&self.inputs);
        let (trace, mut sub_components_inputs, lookup_data) = write_trace_simd(
            packed_inputs,
            memory_address_to_id_state,
            memory_id_to_big_state,
        );

        if need_padding {
            sub_components_inputs.bit_reverse_coset_to_circle_domain_order();
        }
        sub_components_inputs
            .memory_address_to_id_inputs
            .iter()
            .for_each(|inputs| {
                memory_address_to_id_state.add_inputs(&inputs[..n_calls]);
            });
        sub_components_inputs
            .memory_id_to_big_inputs
            .iter()
            .for_each(|inputs| {
                memory_id_to_big_state.add_inputs(&inputs[..n_calls]);
            });
        sub_components_inputs
            .verify_instruction_inputs
            .iter()
            .for_each(|inputs| {
                verify_instruction_state.add_inputs(&inputs[..n_calls]);
            });

        tree_builder.extend_evals(trace.to_evals());

        (
            Claim { n_calls },
            InteractionClaimGenerator {
                n_calls,
                lookup_data,
            },
        )
    }

    pub fn add_inputs(&mut self, inputs: &[InputType]) {
        self.inputs.extend(inputs);
    }
}

#[derive(SubComponentInputs, Uninitialized, IterMut, ParIterMut)]
pub struct SubComponentInputs {
    pub memory_address_to_id_inputs: [Vec<memory_address_to_id::InputType>; 3],
    pub memory_id_to_big_inputs: [Vec<memory_id_to_big::InputType>; 3],
    pub verify_instruction_inputs: [Vec<verify_instruction::InputType>; 1],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
) -> (
    ComponentTrace<N_TRACE_COLUMNS>,
    SubComponentInputs,
    LookupData,
) {
    let log_n_packed_rows = inputs.len().ilog2();
    let log_size = log_n_packed_rows + LOG_N_LANES;
    let (mut trace, mut lookup_data, mut sub_components_inputs) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
            SubComponentInputs::uninitialized(log_size),
        )
    };

    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_32769 = PackedM31::broadcast(M31::from(32769));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let UInt16_13 = PackedUInt16::broadcast(UInt16::from(13));
    let UInt16_4 = PackedUInt16::broadcast(UInt16::from(4));
    let UInt16_5 = PackedUInt16::broadcast(UInt16::from(5));
    let UInt16_7 = PackedUInt16::broadcast(UInt16::from(7));

    trace
        .par_iter_mut()
        .zip(inputs.par_iter())
        .zip(lookup_data.par_iter_mut())
        .zip(sub_components_inputs.par_iter_mut().chunks(N_LANES))
        .for_each(
            |(
                ((row, call_opcode_is_rel_f_op_1_base_fp_f_input), lookup_data),
                mut sub_components_inputs,
            )| {
                let input_tmp_eefa0_0 = call_opcode_is_rel_f_op_1_base_fp_f_input;
                let input_pc_col0 = input_tmp_eefa0_0.pc;
                *row[0] = input_pc_col0;
                let input_ap_col1 = input_tmp_eefa0_0.ap;
                *row[1] = input_ap_col1;
                let input_fp_col2 = input_tmp_eefa0_0.fp;
                *row[2] = input_fp_col2;

                // Decode Instruction.

                let memory_address_to_id_value_tmp_eefa0_1 =
                    memory_address_to_id_state.deduce_output(input_pc_col0);
                let memory_id_to_big_value_tmp_eefa0_2 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_eefa0_1);
                let offset2_tmp_eefa0_3 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_eefa0_2.get_m31(3)))
                        >> (UInt16_5))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_eefa0_2.get_m31(4),
                        )) << (UInt16_4)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_eefa0_2.get_m31(5),
                        )) & (UInt16_7))
                            << (UInt16_13)));
                let offset2_col3 = offset2_tmp_eefa0_3.as_m31();
                *row[3] = offset2_col3;

                for (i, &input) in (
                    input_pc_col0,
                    [M31_32768, M31_32769, offset2_col3],
                    [
                        M31_0, M31_0, M31_0, M31_0, M31_1, M31_0, M31_0, M31_1, M31_0, M31_0,
                        M31_0, M31_0, M31_1, M31_0, M31_0,
                    ],
                )
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].verify_instruction_inputs[0] = input;
                }
                *lookup_data.verify_instruction_0 = [
                    input_pc_col0,
                    M31_32768,
                    M31_32769,
                    offset2_col3,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_1,
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
                ];

                // Read Positive Num Bits 27.

                let memory_address_to_id_value_tmp_eefa0_4 =
                    memory_address_to_id_state.deduce_output(input_ap_col1);
                let memory_id_to_big_value_tmp_eefa0_5 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_eefa0_4);
                let stored_fp_id_col4 = memory_address_to_id_value_tmp_eefa0_4;
                *row[4] = stored_fp_id_col4;
                for (i, &input) in input_ap_col1.unpack().iter().enumerate() {
                    *sub_components_inputs[i].memory_address_to_id_inputs[0] = input;
                }
                *lookup_data.memory_address_to_id_0 = [input_ap_col1, stored_fp_id_col4];
                let stored_fp_limb_0_col5 = memory_id_to_big_value_tmp_eefa0_5.get_m31(0);
                *row[5] = stored_fp_limb_0_col5;
                let stored_fp_limb_1_col6 = memory_id_to_big_value_tmp_eefa0_5.get_m31(1);
                *row[6] = stored_fp_limb_1_col6;
                let stored_fp_limb_2_col7 = memory_id_to_big_value_tmp_eefa0_5.get_m31(2);
                *row[7] = stored_fp_limb_2_col7;
                for (i, &input) in stored_fp_id_col4.unpack().iter().enumerate() {
                    *sub_components_inputs[i].memory_id_to_big_inputs[0] = input;
                }
                *lookup_data.memory_id_to_big_0 = [
                    stored_fp_id_col4,
                    stored_fp_limb_0_col5,
                    stored_fp_limb_1_col6,
                    stored_fp_limb_2_col7,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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

                // Read Positive Num Bits 27.

                let memory_address_to_id_value_tmp_eefa0_6 =
                    memory_address_to_id_state.deduce_output(((input_ap_col1) + (M31_1)));
                let memory_id_to_big_value_tmp_eefa0_7 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_eefa0_6);
                let stored_ret_pc_id_col8 = memory_address_to_id_value_tmp_eefa0_6;
                *row[8] = stored_ret_pc_id_col8;
                for (i, &input) in ((input_ap_col1) + (M31_1)).unpack().iter().enumerate() {
                    *sub_components_inputs[i].memory_address_to_id_inputs[1] = input;
                }
                *lookup_data.memory_address_to_id_1 =
                    [((input_ap_col1) + (M31_1)), stored_ret_pc_id_col8];
                let stored_ret_pc_limb_0_col9 = memory_id_to_big_value_tmp_eefa0_7.get_m31(0);
                *row[9] = stored_ret_pc_limb_0_col9;
                let stored_ret_pc_limb_1_col10 = memory_id_to_big_value_tmp_eefa0_7.get_m31(1);
                *row[10] = stored_ret_pc_limb_1_col10;
                let stored_ret_pc_limb_2_col11 = memory_id_to_big_value_tmp_eefa0_7.get_m31(2);
                *row[11] = stored_ret_pc_limb_2_col11;
                for (i, &input) in stored_ret_pc_id_col8.unpack().iter().enumerate() {
                    *sub_components_inputs[i].memory_id_to_big_inputs[1] = input;
                }
                *lookup_data.memory_id_to_big_1 = [
                    stored_ret_pc_id_col8,
                    stored_ret_pc_limb_0_col9,
                    stored_ret_pc_limb_1_col10,
                    stored_ret_pc_limb_2_col11,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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

                // Read Positive Num Bits 27.

                let memory_address_to_id_value_tmp_eefa0_8 = memory_address_to_id_state
                    .deduce_output(((input_ap_col1) + ((offset2_col3) - (M31_32768))));
                let memory_id_to_big_value_tmp_eefa0_9 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_eefa0_8);
                let next_pc_id_col12 = memory_address_to_id_value_tmp_eefa0_8;
                *row[12] = next_pc_id_col12;
                for (i, &input) in ((input_ap_col1) + ((offset2_col3) - (M31_32768)))
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].memory_address_to_id_inputs[2] = input;
                }
                *lookup_data.memory_address_to_id_2 = [
                    ((input_ap_col1) + ((offset2_col3) - (M31_32768))),
                    next_pc_id_col12,
                ];
                let next_pc_limb_0_col13 = memory_id_to_big_value_tmp_eefa0_9.get_m31(0);
                *row[13] = next_pc_limb_0_col13;
                let next_pc_limb_1_col14 = memory_id_to_big_value_tmp_eefa0_9.get_m31(1);
                *row[14] = next_pc_limb_1_col14;
                let next_pc_limb_2_col15 = memory_id_to_big_value_tmp_eefa0_9.get_m31(2);
                *row[15] = next_pc_limb_2_col15;
                for (i, &input) in next_pc_id_col12.unpack().iter().enumerate() {
                    *sub_components_inputs[i].memory_id_to_big_inputs[2] = input;
                }
                *lookup_data.memory_id_to_big_2 = [
                    next_pc_id_col12,
                    next_pc_limb_0_col13,
                    next_pc_limb_1_col14,
                    next_pc_limb_2_col15,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
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
                    (((next_pc_limb_0_col13) + ((next_pc_limb_1_col14) * (M31_512)))
                        + ((next_pc_limb_2_col15) * (M31_262144))),
                    ((input_ap_col1) + (M31_2)),
                    ((input_ap_col1) + (M31_2)),
                ];
            },
        );

    (trace, sub_components_inputs, lookup_data)
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
    verify_instruction_0: Vec<[PackedM31; 19]>,
}

pub struct InteractionClaimGenerator {
    n_calls: usize,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        memory_address_to_id: &relations::MemoryAddressToId,
        memory_id_to_big: &relations::MemoryIdToBig,
        opcodes: &relations::Opcodes,
        verify_instruction: &relations::VerifyInstruction,
    ) -> InteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let log_size = std::cmp::max(self.n_calls.next_power_of_two().ilog2(), LOG_N_LANES);
        let mut logup_gen = LogupTraceGenerator::new(log_size);

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
            &self.lookup_data.opcodes_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = opcodes.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        for (i, values) in self.lookup_data.opcodes_1.iter().enumerate() {
            let denom = opcodes.combine(values);
            col_gen.write_frac(i, -PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let (trace, total_sum, claimed_sum) = if self.n_calls == 1 << log_size {
            let (trace, claimed_sum) = logup_gen.finalize_last();
            (trace, claimed_sum, None)
        } else {
            let (trace, [total_sum, claimed_sum]) =
                logup_gen.finalize_at([(1 << log_size) - 1, self.n_calls - 1]);
            (trace, total_sum, Some((claimed_sum, self.n_calls - 1)))
        };
        tree_builder.extend_evals(trace);

        InteractionClaim {
            logup_sums: (total_sum, claimed_sum),
        }
    }
}
