#![allow(unused_parens)]
#![allow(unused_imports)]
use std::iter::zip;

use air_structs_derive::SubComponentInputs;
use itertools::{chain, zip_eq, Itertools};
use num_traits::{One, Zero};
use prover_types::cpu::*;
use prover_types::simd::*;
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
        memory_address_to_id_state: &mut memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &mut memory_id_to_big::ClaimGenerator,
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

        tree_builder.extend_evals(
            trace
                .into_iter()
                .map(|eval| {
                    let domain = CanonicCoset::new(
                        eval.len()
                            .checked_ilog2()
                            .expect("Input is not a power of 2!"),
                    )
                    .circle_domain();
                    CircleEvaluation::<SimdBackend, M31, BitReversedOrder>::new(domain, eval)
                })
                .collect_vec(),
        );

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

#[derive(SubComponentInputs)]
pub struct SubComponentInputs {
    pub memory_address_to_id_inputs: [Vec<memory_address_to_id::InputType>; 3],
    pub memory_id_to_big_inputs: [Vec<memory_id_to_big::InputType>; 1],
    pub verify_instruction_inputs: [Vec<verify_instruction::InputType>; 1],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
pub fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    memory_address_to_id_state: &mut memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &mut memory_id_to_big::ClaimGenerator,
) -> (
    [BaseColumn; N_TRACE_COLUMNS],
    SubComponentInputs,
    LookupData,
) {
    const N_TRACE_COLUMNS: usize = 16;
    let mut trace: [_; N_TRACE_COLUMNS] =
        std::array::from_fn(|_| Col::<SimdBackend, M31>::zeros(inputs.len() * N_LANES));

    let mut lookup_data = LookupData::with_capacity(inputs.len());
    #[allow(unused_mut)]
    let mut sub_components_inputs = SubComponentInputs::with_capacity(inputs.len());

    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_512 = PackedM31::broadcast(M31::from(512));
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

    inputs.into_iter().enumerate().for_each(
        |(row_index, assert_eq_opcode_is_double_deref_t_is_imm_f_input)| {
            let input_tmp_d488_0 = assert_eq_opcode_is_double_deref_t_is_imm_f_input;
            let input_pc_col0 = input_tmp_d488_0.pc;
            trace[0].data[row_index] = input_pc_col0;
            let input_ap_col1 = input_tmp_d488_0.ap;
            trace[1].data[row_index] = input_ap_col1;
            let input_fp_col2 = input_tmp_d488_0.fp;
            trace[2].data[row_index] = input_fp_col2;

            // Decode Instruction.

            let memory_address_to_id_value_tmp_d488_1 =
                memory_address_to_id_state.deduce_output(input_pc_col0);
            let memory_id_to_big_value_tmp_d488_2 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_d488_1);
            let offset0_tmp_d488_3 =
                ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_d488_2.get_m31(0)))
                    + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_d488_2.get_m31(1)))
                        & (UInt16_127))
                        << (UInt16_9)));
            let offset0_col3 = offset0_tmp_d488_3.as_m31();
            trace[3].data[row_index] = offset0_col3;
            let offset1_tmp_d488_4 =
                ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_d488_2.get_m31(1)))
                    >> (UInt16_7))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_d488_2.get_m31(2)))
                        << (UInt16_2)))
                    + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_d488_2.get_m31(3)))
                        & (UInt16_31))
                        << (UInt16_11)));
            let offset1_col4 = offset1_tmp_d488_4.as_m31();
            trace[4].data[row_index] = offset1_col4;
            let offset2_tmp_d488_5 =
                ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_d488_2.get_m31(3)))
                    >> (UInt16_5))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_d488_2.get_m31(4)))
                        << (UInt16_4)))
                    + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_d488_2.get_m31(5)))
                        & (UInt16_7))
                        << (UInt16_13)));
            let offset2_col5 = offset2_tmp_d488_5.as_m31();
            trace[5].data[row_index] = offset2_col5;
            let dst_base_fp_tmp_d488_6 =
                (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_d488_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_d488_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_0))
                    & (UInt16_1));
            let dst_base_fp_col6 = dst_base_fp_tmp_d488_6.as_m31();
            trace[6].data[row_index] = dst_base_fp_col6;
            let op0_base_fp_tmp_d488_7 =
                (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_d488_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_d488_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_1))
                    & (UInt16_1));
            let op0_base_fp_col7 = op0_base_fp_tmp_d488_7.as_m31();
            trace[7].data[row_index] = op0_base_fp_col7;
            let ap_update_add_1_tmp_d488_8 =
                (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_d488_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_d488_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_11))
                    & (UInt16_1));
            let ap_update_add_1_col8 = ap_update_add_1_tmp_d488_8.as_m31();
            trace[8].data[row_index] = ap_update_add_1_col8;

            sub_components_inputs.verify_instruction_inputs[0].extend(
                (
                    input_pc_col0,
                    [offset0_col3, offset1_col4, offset2_col5],
                    [
                        dst_base_fp_col6,
                        op0_base_fp_col7,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        ap_update_add_1_col8,
                        M31_0,
                        M31_0,
                        M31_1,
                    ],
                )
                    .unpack(),
            );

            lookup_data.verify_instruction_0.push([
                input_pc_col0,
                offset0_col3,
                offset1_col4,
                offset2_col5,
                dst_base_fp_col6,
                op0_base_fp_col7,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                ap_update_add_1_col8,
                M31_0,
                M31_0,
                M31_1,
            ]);

            let mem_dst_base_col9 = (((dst_base_fp_col6) * (input_fp_col2))
                + (((M31_1) - (dst_base_fp_col6)) * (input_ap_col1)));
            trace[9].data[row_index] = mem_dst_base_col9;
            let mem0_base_col10 = (((op0_base_fp_col7) * (input_fp_col2))
                + (((M31_1) - (op0_base_fp_col7)) * (input_ap_col1)));
            trace[10].data[row_index] = mem0_base_col10;

            // Read Positive Num Bits 27.

            let memory_address_to_id_value_tmp_d488_9 = memory_address_to_id_state
                .deduce_output(((mem0_base_col10) + ((offset1_col4) - (M31_32768))));
            let memory_id_to_big_value_tmp_d488_10 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_d488_9);
            let mem1_base_id_col11 = memory_address_to_id_value_tmp_d488_9;
            trace[11].data[row_index] = mem1_base_id_col11;
            sub_components_inputs.memory_address_to_id_inputs[0]
                .extend(((mem0_base_col10) + ((offset1_col4) - (M31_32768))).unpack());

            lookup_data.memory_address_to_id_0.push([
                ((mem0_base_col10) + ((offset1_col4) - (M31_32768))),
                mem1_base_id_col11,
            ]);
            let mem1_base_limb_0_col12 = memory_id_to_big_value_tmp_d488_10.get_m31(0);
            trace[12].data[row_index] = mem1_base_limb_0_col12;
            let mem1_base_limb_1_col13 = memory_id_to_big_value_tmp_d488_10.get_m31(1);
            trace[13].data[row_index] = mem1_base_limb_1_col13;
            let mem1_base_limb_2_col14 = memory_id_to_big_value_tmp_d488_10.get_m31(2);
            trace[14].data[row_index] = mem1_base_limb_2_col14;
            sub_components_inputs.memory_id_to_big_inputs[0].extend(mem1_base_id_col11.unpack());

            lookup_data.memory_id_to_big_0.push([
                mem1_base_id_col11,
                mem1_base_limb_0_col12,
                mem1_base_limb_1_col13,
                mem1_base_limb_2_col14,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
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

            // Mem Verify Equal.

            let memory_address_to_id_value_tmp_d488_11 = memory_address_to_id_state
                .deduce_output(((mem_dst_base_col9) + ((offset0_col3) - (M31_32768))));
            let dst_id_col15 = memory_address_to_id_value_tmp_d488_11;
            trace[15].data[row_index] = dst_id_col15;
            sub_components_inputs.memory_address_to_id_inputs[1]
                .extend(((mem_dst_base_col9) + ((offset0_col3) - (M31_32768))).unpack());

            lookup_data.memory_address_to_id_1.push([
                ((mem_dst_base_col9) + ((offset0_col3) - (M31_32768))),
                dst_id_col15,
            ]);
            sub_components_inputs.memory_address_to_id_inputs[2].extend(
                ((((mem1_base_limb_0_col12) + ((mem1_base_limb_1_col13) * (M31_512)))
                    + ((mem1_base_limb_2_col14) * (M31_262144)))
                    + ((offset2_col5) - (M31_32768)))
                    .unpack(),
            );

            lookup_data.memory_address_to_id_2.push([
                ((((mem1_base_limb_0_col12) + ((mem1_base_limb_1_col13) * (M31_512)))
                    + ((mem1_base_limb_2_col14) * (M31_262144)))
                    + ((offset2_col5) - (M31_32768))),
                dst_id_col15,
            ]);

            lookup_data
                .opcodes_0
                .push([input_pc_col0, input_ap_col1, input_fp_col2]);
            lookup_data.opcodes_1.push([
                ((input_pc_col0) + (M31_1)),
                ((input_ap_col1) + (ap_update_add_1_col8)),
                input_fp_col2,
            ]);
        },
    );

    (trace, sub_components_inputs, lookup_data)
}

pub struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 2]>,
    memory_address_to_id_1: Vec<[PackedM31; 2]>,
    memory_address_to_id_2: Vec<[PackedM31; 2]>,
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
    opcodes_0: Vec<[PackedM31; 3]>,
    opcodes_1: Vec<[PackedM31; 3]>,
    verify_instruction_0: Vec<[PackedM31; 19]>,
}
impl LookupData {
    #[allow(unused_variables)]
    fn with_capacity(capacity: usize) -> Self {
        Self {
            memory_address_to_id_0: Vec::with_capacity(capacity),
            memory_address_to_id_1: Vec::with_capacity(capacity),
            memory_address_to_id_2: Vec::with_capacity(capacity),
            memory_id_to_big_0: Vec::with_capacity(capacity),
            opcodes_0: Vec::with_capacity(capacity),
            opcodes_1: Vec::with_capacity(capacity),
            verify_instruction_0: Vec::with_capacity(capacity),
        }
    }
}

pub struct InteractionClaimGenerator {
    pub n_calls: usize,
    pub lookup_data: LookupData,
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

        let mut col_gen = logup_gen.new_col();
        for (i, (v0, v1)) in zip(
            &self.lookup_data.verify_instruction_0,
            &self.lookup_data.memory_address_to_id_0,
        )
        .enumerate()
        {
            let p0: PackedQM31 = verify_instruction.combine(v0);
            let p1: PackedQM31 = memory_address_to_id.combine(v1);
            col_gen.write_frac(i, p0 + p1, p0 * p1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (v0, v1)) in zip(
            &self.lookup_data.memory_id_to_big_0,
            &self.lookup_data.memory_address_to_id_1,
        )
        .enumerate()
        {
            let p0: PackedQM31 = memory_id_to_big.combine(v0);
            let p1: PackedQM31 = memory_address_to_id.combine(v1);
            col_gen.write_frac(i, p0 + p1, p0 * p1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (v0, v1)) in zip(
            &self.lookup_data.memory_address_to_id_2,
            &self.lookup_data.opcodes_0,
        )
        .enumerate()
        {
            let p0: PackedQM31 = memory_address_to_id.combine(v0);
            let p1: PackedQM31 = opcodes.combine(v1);
            col_gen.write_frac(i, p0 + p1, p0 * p1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, v0) in self.lookup_data.opcodes_1.iter().enumerate() {
            let p0 = opcodes.combine(v0);
            col_gen.write_frac(i, -PackedQM31::one(), p0);
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
