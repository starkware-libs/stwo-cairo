#![allow(unused_parens)]
#![allow(unused_imports)]
use air_structs_derive::{LookupData, SubComponentInputs};
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
const N_TRACE_COLUMNS: usize = 13;

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
    pub memory_address_to_id_inputs: [Vec<memory_address_to_id::InputType>; 1],
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
    const N_TRACE_COLUMNS: usize = 13;
    let mut trace: [_; N_TRACE_COLUMNS] =
        std::array::from_fn(|_| Col::<SimdBackend, M31>::zeros(inputs.len() * N_LANES));

    let mut lookup_data = LookupData::with_capacity(inputs.len());
    #[allow(unused_mut)]
    let mut sub_components_inputs = SubComponentInputs::with_capacity(inputs.len());

    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_134217728 = PackedM31::broadcast(M31::from(134217728));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32767 = PackedM31::broadcast(M31::from(32767));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_511 = PackedM31::broadcast(M31::from(511));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let UInt16_1 = PackedUInt16::broadcast(UInt16::from(1));
    let UInt16_11 = PackedUInt16::broadcast(UInt16::from(11));
    let UInt16_13 = PackedUInt16::broadcast(UInt16::from(13));
    let UInt16_3 = PackedUInt16::broadcast(UInt16::from(3));
    let UInt16_4 = PackedUInt16::broadcast(UInt16::from(4));
    let UInt16_5 = PackedUInt16::broadcast(UInt16::from(5));
    let UInt16_6 = PackedUInt16::broadcast(UInt16::from(6));
    let UInt16_7 = PackedUInt16::broadcast(UInt16::from(7));

    inputs.into_iter().enumerate().for_each(
        |(row_index, jump_opcode_is_rel_t_is_imm_f_is_double_deref_f_input)| {
            let input_tmp_1409_0 = jump_opcode_is_rel_t_is_imm_f_is_double_deref_f_input;
            let input_pc_col0 = input_tmp_1409_0.pc;
            trace[0].data[row_index] = input_pc_col0;
            let input_ap_col1 = input_tmp_1409_0.ap;
            trace[1].data[row_index] = input_ap_col1;
            let input_fp_col2 = input_tmp_1409_0.fp;
            trace[2].data[row_index] = input_fp_col2;

            // DecodeInstruction_2cd1887fb89e83e3.

            let memoryaddresstoid_value_tmp_1409_1 =
                memory_address_to_id_state.deduce_output(input_pc_col0);
            let memoryidtobig_value_tmp_1409_2 =
                memory_id_to_big_state.deduce_output(memoryaddresstoid_value_tmp_1409_1);
            let offset2_tmp_1409_3 =
                ((((PackedUInt16::from_m31(memoryidtobig_value_tmp_1409_2.get_m31(3)))
                    >> (UInt16_5))
                    + ((PackedUInt16::from_m31(memoryidtobig_value_tmp_1409_2.get_m31(4)))
                        << (UInt16_4)))
                    + (((PackedUInt16::from_m31(memoryidtobig_value_tmp_1409_2.get_m31(5)))
                        & (UInt16_7))
                        << (UInt16_13)));
            let offset2_col3 = offset2_tmp_1409_3.as_m31();
            trace[3].data[row_index] = offset2_col3;
            let op1_base_fp_tmp_1409_4 =
                (((((PackedUInt16::from_m31(memoryidtobig_value_tmp_1409_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memoryidtobig_value_tmp_1409_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_3))
                    & (UInt16_1));
            let op1_base_fp_col4 = op1_base_fp_tmp_1409_4.as_m31();
            trace[4].data[row_index] = op1_base_fp_col4;
            let op1_base_ap_tmp_1409_5 =
                (((((PackedUInt16::from_m31(memoryidtobig_value_tmp_1409_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memoryidtobig_value_tmp_1409_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_4))
                    & (UInt16_1));
            let op1_base_ap_col5 = op1_base_ap_tmp_1409_5.as_m31();
            trace[5].data[row_index] = op1_base_ap_col5;
            let ap_update_add_1_tmp_1409_6 =
                (((((PackedUInt16::from_m31(memoryidtobig_value_tmp_1409_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memoryidtobig_value_tmp_1409_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_11))
                    & (UInt16_1));
            let ap_update_add_1_col6 = ap_update_add_1_tmp_1409_6.as_m31();
            trace[6].data[row_index] = ap_update_add_1_col6;

            sub_components_inputs.verify_instruction_inputs[0].extend(
                (
                    input_pc_col0,
                    [M31_32767, M31_32767, offset2_col3],
                    [
                        M31_1,
                        M31_1,
                        M31_0,
                        op1_base_fp_col4,
                        op1_base_ap_col5,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_1,
                        M31_0,
                        M31_0,
                        ap_update_add_1_col6,
                        M31_0,
                        M31_0,
                        M31_0,
                    ],
                )
                    .unpack(),
            );

            lookup_data.verifyinstruction[0].push([
                input_pc_col0,
                M31_32767,
                M31_32767,
                offset2_col3,
                M31_1,
                M31_1,
                M31_0,
                op1_base_fp_col4,
                op1_base_ap_col5,
                M31_0,
                M31_0,
                M31_0,
                M31_1,
                M31_0,
                M31_0,
                ap_update_add_1_col6,
                M31_0,
                M31_0,
                M31_0,
            ]);

            // ReadSmall.

            let memoryaddresstoid_value_tmp_1409_7 = memory_address_to_id_state.deduce_output(
                ((((op1_base_fp_col4) * (input_fp_col2)) + ((op1_base_ap_col5) * (input_ap_col1)))
                    + ((offset2_col3) - (M31_32768))),
            );
            let memoryidtobig_value_tmp_1409_8 =
                memory_id_to_big_state.deduce_output(memoryaddresstoid_value_tmp_1409_7);
            let next_pc_id_col7 = memoryaddresstoid_value_tmp_1409_7;
            trace[7].data[row_index] = next_pc_id_col7;
            sub_components_inputs.memory_address_to_id_inputs[0].extend(
                ((((op1_base_fp_col4) * (input_fp_col2)) + ((op1_base_ap_col5) * (input_ap_col1)))
                    + ((offset2_col3) - (M31_32768)))
                    .unpack(),
            );

            lookup_data.memoryaddresstoid[0].push([
                ((((op1_base_fp_col4) * (input_fp_col2)) + ((op1_base_ap_col5) * (input_ap_col1)))
                    + ((offset2_col3) - (M31_32768))),
                next_pc_id_col7,
            ]);

            // CondDecodeSmallSign.

            let msb_tmp_1409_9 = memoryidtobig_value_tmp_1409_8.get_m31(27).eq(M31_256);
            let msb_col8 = msb_tmp_1409_9.as_m31();
            trace[8].data[row_index] = msb_col8;
            let mid_limbs_set_tmp_1409_10 = memoryidtobig_value_tmp_1409_8.get_m31(20).eq(M31_511);
            let mid_limbs_set_col9 = mid_limbs_set_tmp_1409_10.as_m31();
            trace[9].data[row_index] = mid_limbs_set_col9;

            let next_pc_limb_0_col10 = memoryidtobig_value_tmp_1409_8.get_m31(0);
            trace[10].data[row_index] = next_pc_limb_0_col10;
            let next_pc_limb_1_col11 = memoryidtobig_value_tmp_1409_8.get_m31(1);
            trace[11].data[row_index] = next_pc_limb_1_col11;
            let next_pc_limb_2_col12 = memoryidtobig_value_tmp_1409_8.get_m31(2);
            trace[12].data[row_index] = next_pc_limb_2_col12;
            sub_components_inputs.memory_id_to_big_inputs[0].extend(next_pc_id_col7.unpack());

            lookup_data.memoryidtobig[0].push([
                next_pc_id_col7,
                next_pc_limb_0_col10,
                next_pc_limb_1_col11,
                next_pc_limb_2_col12,
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
            ]);

            lookup_data.opcodes[0].push([input_pc_col0, input_ap_col1, input_fp_col2]);
            lookup_data.opcodes[1].push([
                ((input_pc_col0)
                    + (((((next_pc_limb_0_col10) + ((next_pc_limb_1_col11) * (M31_512)))
                        + ((next_pc_limb_2_col12) * (M31_262144)))
                        - (msb_col8))
                        - ((M31_134217728) * (mid_limbs_set_col9)))),
                ((input_ap_col1) + (ap_update_add_1_col6)),
                input_fp_col2,
            ]);
        },
    );

    (trace, sub_components_inputs, lookup_data)
}

#[derive(LookupData)]
pub struct LookupData {
    pub memoryaddresstoid: [Vec<[PackedM31; 2]>; 1],
    pub memoryidtobig: [Vec<[PackedM31; 29]>; 1],
    pub opcodes: [Vec<[PackedM31; 3]>; 2],
    pub verifyinstruction: [Vec<[PackedM31; 19]>; 1],
}

pub struct InteractionClaimGenerator {
    pub n_calls: usize,
    pub lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        memoryaddresstoid_lookup_elements: &relations::MemoryAddressToId,
        memoryidtobig_lookup_elements: &relations::MemoryIdToBig,
        opcodes_lookup_elements: &relations::Opcodes,
        verifyinstruction_lookup_elements: &relations::VerifyInstruction,
    ) -> InteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let log_size = std::cmp::max(self.n_calls.next_power_of_two().ilog2(), LOG_N_LANES);
        let mut logup_gen = LogupTraceGenerator::new(log_size);

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.verifyinstruction[0];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = verifyinstruction_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.memoryaddresstoid[0];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = memoryaddresstoid_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.memoryidtobig[0];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = memoryidtobig_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.opcodes[0];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = opcodes_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.opcodes[1];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = opcodes_lookup_elements.combine(lookup_values);
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
