#![allow(unused_parens)]
#![allow(unused_imports)]
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
const N_TRACE_COLUMNS: usize = 26;

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
    pub memory_id_to_big_inputs: [Vec<memory_id_to_big::InputType>; 3],
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
    const N_TRACE_COLUMNS: usize = 26;
    let mut trace: [_; N_TRACE_COLUMNS] =
        std::array::from_fn(|_| Col::<SimdBackend, M31>::zeros(inputs.len() * N_LANES));

    let mut lookup_data = LookupData::with_capacity(inputs.len());
    #[allow(unused_mut)]
    let mut sub_components_inputs = SubComponentInputs::with_capacity(inputs.len());

    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_32769 = PackedM31::broadcast(M31::from(32769));
    let M31_511 = PackedM31::broadcast(M31::from(511));
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

    inputs
        .into_iter()
        .enumerate()
        .for_each(|(row_index, add_opcode_is_small_t_is_imm_t_input)| {
            let input_tmp_c5e3_0 = add_opcode_is_small_t_is_imm_t_input;
            let input_pc_col0 = input_tmp_c5e3_0.pc;
            trace[0].data[row_index] = input_pc_col0;
            let input_ap_col1 = input_tmp_c5e3_0.ap;
            trace[1].data[row_index] = input_ap_col1;
            let input_fp_col2 = input_tmp_c5e3_0.fp;
            trace[2].data[row_index] = input_fp_col2;

            // DecodeInstruction_9aed6a790187299c.

            let memoryaddresstoid_value_tmp_c5e3_1 =
                memory_address_to_id_state.deduce_output(input_pc_col0);
            let memoryidtobig_value_tmp_c5e3_2 =
                memory_id_to_big_state.deduce_output(memoryaddresstoid_value_tmp_c5e3_1);
            let offset0_tmp_c5e3_3 =
                ((PackedUInt16::from_m31(memoryidtobig_value_tmp_c5e3_2.get_m31(0)))
                    + (((PackedUInt16::from_m31(memoryidtobig_value_tmp_c5e3_2.get_m31(1)))
                        & (UInt16_127))
                        << (UInt16_9)));
            let offset0_col3 = offset0_tmp_c5e3_3.as_m31();
            trace[3].data[row_index] = offset0_col3;
            let offset1_tmp_c5e3_4 =
                ((((PackedUInt16::from_m31(memoryidtobig_value_tmp_c5e3_2.get_m31(1)))
                    >> (UInt16_7))
                    + ((PackedUInt16::from_m31(memoryidtobig_value_tmp_c5e3_2.get_m31(2)))
                        << (UInt16_2)))
                    + (((PackedUInt16::from_m31(memoryidtobig_value_tmp_c5e3_2.get_m31(3)))
                        & (UInt16_31))
                        << (UInt16_11)));
            let offset1_col4 = offset1_tmp_c5e3_4.as_m31();
            trace[4].data[row_index] = offset1_col4;
            let dst_base_fp_tmp_c5e3_5 =
                (((((PackedUInt16::from_m31(memoryidtobig_value_tmp_c5e3_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memoryidtobig_value_tmp_c5e3_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_0))
                    & (UInt16_1));
            let dst_base_fp_col5 = dst_base_fp_tmp_c5e3_5.as_m31();
            trace[5].data[row_index] = dst_base_fp_col5;
            let op0_base_fp_tmp_c5e3_6 =
                (((((PackedUInt16::from_m31(memoryidtobig_value_tmp_c5e3_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memoryidtobig_value_tmp_c5e3_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_1))
                    & (UInt16_1));
            let op0_base_fp_col6 = op0_base_fp_tmp_c5e3_6.as_m31();
            trace[6].data[row_index] = op0_base_fp_col6;
            let ap_update_add_1_tmp_c5e3_7 =
                (((((PackedUInt16::from_m31(memoryidtobig_value_tmp_c5e3_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memoryidtobig_value_tmp_c5e3_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_11))
                    & (UInt16_1));
            let ap_update_add_1_col7 = ap_update_add_1_tmp_c5e3_7.as_m31();
            trace[7].data[row_index] = ap_update_add_1_col7;

            sub_components_inputs.verify_instruction_inputs[0].extend(
                (
                    input_pc_col0,
                    [offset0_col3, offset1_col4, M31_32769],
                    [
                        dst_base_fp_col5,
                        op0_base_fp_col6,
                        M31_1,
                        M31_0,
                        M31_0,
                        M31_1,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        ap_update_add_1_col7,
                        M31_0,
                        M31_0,
                        M31_1,
                    ],
                )
                    .unpack(),
            );

            lookup_data.verifyinstruction[0].push([
                input_pc_col0,
                offset0_col3,
                offset1_col4,
                M31_32769,
                dst_base_fp_col5,
                op0_base_fp_col6,
                M31_1,
                M31_0,
                M31_0,
                M31_1,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                ap_update_add_1_col7,
                M31_0,
                M31_0,
                M31_1,
            ]);

            // ReadSmall.

            let memoryaddresstoid_value_tmp_c5e3_8 = memory_address_to_id_state.deduce_output(
                ((((dst_base_fp_col5) * (input_fp_col2))
                    + (((M31_1) - (dst_base_fp_col5)) * (input_ap_col1)))
                    + ((offset0_col3) - (M31_32768))),
            );
            let memoryidtobig_value_tmp_c5e3_9 =
                memory_id_to_big_state.deduce_output(memoryaddresstoid_value_tmp_c5e3_8);
            let dst_id_col8 = memoryaddresstoid_value_tmp_c5e3_8;
            trace[8].data[row_index] = dst_id_col8;
            sub_components_inputs.memory_address_to_id_inputs[0].extend(
                ((((dst_base_fp_col5) * (input_fp_col2))
                    + (((M31_1) - (dst_base_fp_col5)) * (input_ap_col1)))
                    + ((offset0_col3) - (M31_32768)))
                    .unpack(),
            );

            lookup_data.memoryaddresstoid[0].push([
                ((((dst_base_fp_col5) * (input_fp_col2))
                    + (((M31_1) - (dst_base_fp_col5)) * (input_ap_col1)))
                    + ((offset0_col3) - (M31_32768))),
                dst_id_col8,
            ]);

            // CondDecodeSmallSign.

            let msb_tmp_c5e3_10 = memoryidtobig_value_tmp_c5e3_9.get_m31(27).eq(M31_256);
            let msb_col9 = msb_tmp_c5e3_10.as_m31();
            trace[9].data[row_index] = msb_col9;
            let mid_limbs_set_tmp_c5e3_11 = memoryidtobig_value_tmp_c5e3_9.get_m31(20).eq(M31_511);
            let mid_limbs_set_col10 = mid_limbs_set_tmp_c5e3_11.as_m31();
            trace[10].data[row_index] = mid_limbs_set_col10;

            let dst_limb_0_col11 = memoryidtobig_value_tmp_c5e3_9.get_m31(0);
            trace[11].data[row_index] = dst_limb_0_col11;
            let dst_limb_1_col12 = memoryidtobig_value_tmp_c5e3_9.get_m31(1);
            trace[12].data[row_index] = dst_limb_1_col12;
            let dst_limb_2_col13 = memoryidtobig_value_tmp_c5e3_9.get_m31(2);
            trace[13].data[row_index] = dst_limb_2_col13;
            sub_components_inputs.memory_id_to_big_inputs[0].extend(dst_id_col8.unpack());

            lookup_data.memoryidtobig[0].push([
                dst_id_col8,
                dst_limb_0_col11,
                dst_limb_1_col12,
                dst_limb_2_col13,
                ((mid_limbs_set_col10) * (M31_511)),
                ((mid_limbs_set_col10) * (M31_511)),
                ((mid_limbs_set_col10) * (M31_511)),
                ((mid_limbs_set_col10) * (M31_511)),
                ((mid_limbs_set_col10) * (M31_511)),
                ((mid_limbs_set_col10) * (M31_511)),
                ((mid_limbs_set_col10) * (M31_511)),
                ((mid_limbs_set_col10) * (M31_511)),
                ((mid_limbs_set_col10) * (M31_511)),
                ((mid_limbs_set_col10) * (M31_511)),
                ((mid_limbs_set_col10) * (M31_511)),
                ((mid_limbs_set_col10) * (M31_511)),
                ((mid_limbs_set_col10) * (M31_511)),
                ((mid_limbs_set_col10) * (M31_511)),
                ((mid_limbs_set_col10) * (M31_511)),
                ((mid_limbs_set_col10) * (M31_511)),
                ((mid_limbs_set_col10) * (M31_511)),
                ((mid_limbs_set_col10) * (M31_511)),
                (((M31_136) * (msb_col9)) - (mid_limbs_set_col10)),
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                ((msb_col9) * (M31_256)),
            ]);

            // ReadSmall.

            let memoryaddresstoid_value_tmp_c5e3_12 = memory_address_to_id_state.deduce_output(
                ((((op0_base_fp_col6) * (input_fp_col2))
                    + (((M31_1) - (op0_base_fp_col6)) * (input_ap_col1)))
                    + ((offset1_col4) - (M31_32768))),
            );
            let memoryidtobig_value_tmp_c5e3_13 =
                memory_id_to_big_state.deduce_output(memoryaddresstoid_value_tmp_c5e3_12);
            let op0_id_col14 = memoryaddresstoid_value_tmp_c5e3_12;
            trace[14].data[row_index] = op0_id_col14;
            sub_components_inputs.memory_address_to_id_inputs[1].extend(
                ((((op0_base_fp_col6) * (input_fp_col2))
                    + (((M31_1) - (op0_base_fp_col6)) * (input_ap_col1)))
                    + ((offset1_col4) - (M31_32768)))
                    .unpack(),
            );

            lookup_data.memoryaddresstoid[1].push([
                ((((op0_base_fp_col6) * (input_fp_col2))
                    + (((M31_1) - (op0_base_fp_col6)) * (input_ap_col1)))
                    + ((offset1_col4) - (M31_32768))),
                op0_id_col14,
            ]);

            // CondDecodeSmallSign.

            let msb_tmp_c5e3_14 = memoryidtobig_value_tmp_c5e3_13.get_m31(27).eq(M31_256);
            let msb_col15 = msb_tmp_c5e3_14.as_m31();
            trace[15].data[row_index] = msb_col15;
            let mid_limbs_set_tmp_c5e3_15 = memoryidtobig_value_tmp_c5e3_13.get_m31(20).eq(M31_511);
            let mid_limbs_set_col16 = mid_limbs_set_tmp_c5e3_15.as_m31();
            trace[16].data[row_index] = mid_limbs_set_col16;

            let op0_limb_0_col17 = memoryidtobig_value_tmp_c5e3_13.get_m31(0);
            trace[17].data[row_index] = op0_limb_0_col17;
            let op0_limb_1_col18 = memoryidtobig_value_tmp_c5e3_13.get_m31(1);
            trace[18].data[row_index] = op0_limb_1_col18;
            let op0_limb_2_col19 = memoryidtobig_value_tmp_c5e3_13.get_m31(2);
            trace[19].data[row_index] = op0_limb_2_col19;
            sub_components_inputs.memory_id_to_big_inputs[1].extend(op0_id_col14.unpack());

            lookup_data.memoryidtobig[1].push([
                op0_id_col14,
                op0_limb_0_col17,
                op0_limb_1_col18,
                op0_limb_2_col19,
                ((mid_limbs_set_col16) * (M31_511)),
                ((mid_limbs_set_col16) * (M31_511)),
                ((mid_limbs_set_col16) * (M31_511)),
                ((mid_limbs_set_col16) * (M31_511)),
                ((mid_limbs_set_col16) * (M31_511)),
                ((mid_limbs_set_col16) * (M31_511)),
                ((mid_limbs_set_col16) * (M31_511)),
                ((mid_limbs_set_col16) * (M31_511)),
                ((mid_limbs_set_col16) * (M31_511)),
                ((mid_limbs_set_col16) * (M31_511)),
                ((mid_limbs_set_col16) * (M31_511)),
                ((mid_limbs_set_col16) * (M31_511)),
                ((mid_limbs_set_col16) * (M31_511)),
                ((mid_limbs_set_col16) * (M31_511)),
                ((mid_limbs_set_col16) * (M31_511)),
                ((mid_limbs_set_col16) * (M31_511)),
                ((mid_limbs_set_col16) * (M31_511)),
                ((mid_limbs_set_col16) * (M31_511)),
                (((M31_136) * (msb_col15)) - (mid_limbs_set_col16)),
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                ((msb_col15) * (M31_256)),
            ]);

            // ReadSmall.

            let memoryaddresstoid_value_tmp_c5e3_16 =
                memory_address_to_id_state.deduce_output(((input_pc_col0) + (M31_1)));
            let memoryidtobig_value_tmp_c5e3_17 =
                memory_id_to_big_state.deduce_output(memoryaddresstoid_value_tmp_c5e3_16);
            let op1_id_col20 = memoryaddresstoid_value_tmp_c5e3_16;
            trace[20].data[row_index] = op1_id_col20;
            sub_components_inputs.memory_address_to_id_inputs[2]
                .extend(((input_pc_col0) + (M31_1)).unpack());

            lookup_data.memoryaddresstoid[2].push([((input_pc_col0) + (M31_1)), op1_id_col20]);

            // CondDecodeSmallSign.

            let msb_tmp_c5e3_18 = memoryidtobig_value_tmp_c5e3_17.get_m31(27).eq(M31_256);
            let msb_col21 = msb_tmp_c5e3_18.as_m31();
            trace[21].data[row_index] = msb_col21;
            let mid_limbs_set_tmp_c5e3_19 = memoryidtobig_value_tmp_c5e3_17.get_m31(20).eq(M31_511);
            let mid_limbs_set_col22 = mid_limbs_set_tmp_c5e3_19.as_m31();
            trace[22].data[row_index] = mid_limbs_set_col22;

            let op1_limb_0_col23 = memoryidtobig_value_tmp_c5e3_17.get_m31(0);
            trace[23].data[row_index] = op1_limb_0_col23;
            let op1_limb_1_col24 = memoryidtobig_value_tmp_c5e3_17.get_m31(1);
            trace[24].data[row_index] = op1_limb_1_col24;
            let op1_limb_2_col25 = memoryidtobig_value_tmp_c5e3_17.get_m31(2);
            trace[25].data[row_index] = op1_limb_2_col25;
            sub_components_inputs.memory_id_to_big_inputs[2].extend(op1_id_col20.unpack());

            lookup_data.memoryidtobig[2].push([
                op1_id_col20,
                op1_limb_0_col23,
                op1_limb_1_col24,
                op1_limb_2_col25,
                ((mid_limbs_set_col22) * (M31_511)),
                ((mid_limbs_set_col22) * (M31_511)),
                ((mid_limbs_set_col22) * (M31_511)),
                ((mid_limbs_set_col22) * (M31_511)),
                ((mid_limbs_set_col22) * (M31_511)),
                ((mid_limbs_set_col22) * (M31_511)),
                ((mid_limbs_set_col22) * (M31_511)),
                ((mid_limbs_set_col22) * (M31_511)),
                ((mid_limbs_set_col22) * (M31_511)),
                ((mid_limbs_set_col22) * (M31_511)),
                ((mid_limbs_set_col22) * (M31_511)),
                ((mid_limbs_set_col22) * (M31_511)),
                ((mid_limbs_set_col22) * (M31_511)),
                ((mid_limbs_set_col22) * (M31_511)),
                ((mid_limbs_set_col22) * (M31_511)),
                ((mid_limbs_set_col22) * (M31_511)),
                ((mid_limbs_set_col22) * (M31_511)),
                ((mid_limbs_set_col22) * (M31_511)),
                (((M31_136) * (msb_col21)) - (mid_limbs_set_col22)),
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                ((msb_col21) * (M31_256)),
            ]);

            lookup_data.opcodes[0].push([input_pc_col0, input_ap_col1, input_fp_col2]);
            lookup_data.opcodes[1].push([
                ((input_pc_col0) + (M31_2)),
                ((input_ap_col1) + (ap_update_add_1_col7)),
                input_fp_col2,
            ]);
        });

    (trace, sub_components_inputs, lookup_data)
}

pub struct LookupData {
    pub memoryaddresstoid: [Vec<[PackedM31; 2]>; 3],
    pub memoryidtobig: [Vec<[PackedM31; 29]>; 3],
    pub opcodes: [Vec<[PackedM31; 3]>; 2],
    pub verifyinstruction: [Vec<[PackedM31; 19]>; 1],
}
impl LookupData {
    #[allow(unused_variables)]
    fn with_capacity(capacity: usize) -> Self {
        Self {
            memoryaddresstoid: [
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
            ],
            memoryidtobig: [
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
            ],
            opcodes: [Vec::with_capacity(capacity), Vec::with_capacity(capacity)],
            verifyinstruction: [Vec::with_capacity(capacity)],
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
        let lookup_row = &self.lookup_data.memoryaddresstoid[1];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = memoryaddresstoid_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.memoryidtobig[1];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = memoryidtobig_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.memoryaddresstoid[2];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = memoryaddresstoid_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.memoryidtobig[2];
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
