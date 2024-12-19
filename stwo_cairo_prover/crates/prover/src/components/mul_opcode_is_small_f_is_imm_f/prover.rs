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
use crate::components::{
    memory_address_to_id, memory_id_to_big, pack_values, range_check_19, verify_instruction,
};
use crate::relations;

pub type InputType = CasmState;
pub type PackedInputType = PackedCasmState;
const N_TRACE_COLUMNS: usize = 129;

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
        range_check_19_state: &mut range_check_19::ClaimGenerator,
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
            .range_check_19_inputs
            .iter()
            .for_each(|inputs| {
                range_check_19_state.add_inputs(&inputs[..n_calls]);
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
    pub range_check_19_inputs: [Vec<range_check_19::InputType>; 28],
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
    const N_TRACE_COLUMNS: usize = 129;
    let mut trace: [_; N_TRACE_COLUMNS] =
        std::array::from_fn(|_| Col::<SimdBackend, M31>::zeros(inputs.len() * N_LANES));

    let mut lookup_data = LookupData::with_capacity(inputs.len());
    #[allow(unused_mut)]
    let mut sub_components_inputs = SubComponentInputs::with_capacity(inputs.len());

    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_131072 = PackedM31::broadcast(M31::from(131072));
    let M31_134217728 = PackedM31::broadcast(M31::from(134217728));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_4 = PackedM31::broadcast(M31::from(4));
    let M31_4194304 = PackedM31::broadcast(M31::from(4194304));
    let M31_64 = PackedM31::broadcast(M31::from(64));
    let M31_65536 = PackedM31::broadcast(M31::from(65536));
    let M31_8 = PackedM31::broadcast(M31::from(8));
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
    let UInt32_262143 = PackedUInt32::broadcast(UInt32::from(262143));
    let UInt32_511 = PackedUInt32::broadcast(UInt32::from(511));
    let UInt32_65536 = PackedUInt32::broadcast(UInt32::from(65536));
    let UInt32_9 = PackedUInt32::broadcast(UInt32::from(9));

    inputs
        .into_iter()
        .enumerate()
        .for_each(|(row_index, mul_opcode_is_small_f_is_imm_f_input)| {
            let input_tmp_5a14_0 = mul_opcode_is_small_f_is_imm_f_input;
            let input_pc_col0 = input_tmp_5a14_0.pc;
            trace[0].data[row_index] = input_pc_col0;
            let input_ap_col1 = input_tmp_5a14_0.ap;
            trace[1].data[row_index] = input_ap_col1;
            let input_fp_col2 = input_tmp_5a14_0.fp;
            trace[2].data[row_index] = input_fp_col2;

            // Decode Instruction.

            let memory_address_to_id_value_tmp_5a14_1 =
                memory_address_to_id_state.deduce_output(input_pc_col0);
            let memory_id_to_big_value_tmp_5a14_2 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_5a14_1);
            let offset0_tmp_5a14_3 =
                ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a14_2.get_m31(0)))
                    + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a14_2.get_m31(1)))
                        & (UInt16_127))
                        << (UInt16_9)));
            let offset0_col3 = offset0_tmp_5a14_3.as_m31();
            trace[3].data[row_index] = offset0_col3;
            let offset1_tmp_5a14_4 =
                ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a14_2.get_m31(1)))
                    >> (UInt16_7))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a14_2.get_m31(2)))
                        << (UInt16_2)))
                    + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a14_2.get_m31(3)))
                        & (UInt16_31))
                        << (UInt16_11)));
            let offset1_col4 = offset1_tmp_5a14_4.as_m31();
            trace[4].data[row_index] = offset1_col4;
            let offset2_tmp_5a14_5 =
                ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a14_2.get_m31(3)))
                    >> (UInt16_5))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a14_2.get_m31(4)))
                        << (UInt16_4)))
                    + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a14_2.get_m31(5)))
                        & (UInt16_7))
                        << (UInt16_13)));
            let offset2_col5 = offset2_tmp_5a14_5.as_m31();
            trace[5].data[row_index] = offset2_col5;
            let dst_base_fp_tmp_5a14_6 =
                (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a14_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a14_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_0))
                    & (UInt16_1));
            let dst_base_fp_col6 = dst_base_fp_tmp_5a14_6.as_m31();
            trace[6].data[row_index] = dst_base_fp_col6;
            let op0_base_fp_tmp_5a14_7 =
                (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a14_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a14_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_1))
                    & (UInt16_1));
            let op0_base_fp_col7 = op0_base_fp_tmp_5a14_7.as_m31();
            trace[7].data[row_index] = op0_base_fp_col7;
            let op1_base_fp_tmp_5a14_8 =
                (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a14_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a14_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_3))
                    & (UInt16_1));
            let op1_base_fp_col8 = op1_base_fp_tmp_5a14_8.as_m31();
            trace[8].data[row_index] = op1_base_fp_col8;
            let op1_base_ap_tmp_5a14_9 =
                (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a14_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a14_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_4))
                    & (UInt16_1));
            let op1_base_ap_col9 = op1_base_ap_tmp_5a14_9.as_m31();
            trace[9].data[row_index] = op1_base_ap_col9;
            let ap_update_add_1_tmp_5a14_10 =
                (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a14_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a14_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_11))
                    & (UInt16_1));
            let ap_update_add_1_col10 = ap_update_add_1_tmp_5a14_10.as_m31();
            trace[10].data[row_index] = ap_update_add_1_col10;

            sub_components_inputs.verify_instruction_inputs[0].extend(
                (
                    input_pc_col0,
                    [offset0_col3, offset1_col4, offset2_col5],
                    [
                        dst_base_fp_col6,
                        op0_base_fp_col7,
                        M31_0,
                        op1_base_fp_col8,
                        op1_base_ap_col9,
                        M31_0,
                        M31_1,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        ap_update_add_1_col10,
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
                op1_base_fp_col8,
                op1_base_ap_col9,
                M31_0,
                M31_1,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                ap_update_add_1_col10,
                M31_0,
                M31_0,
                M31_1,
            ]);

            let mem_dst_base_col11 = (((dst_base_fp_col6) * (input_fp_col2))
                + (((M31_1) - (dst_base_fp_col6)) * (input_ap_col1)));
            trace[11].data[row_index] = mem_dst_base_col11;
            let mem0_base_col12 = (((op0_base_fp_col7) * (input_fp_col2))
                + (((M31_1) - (op0_base_fp_col7)) * (input_ap_col1)));
            trace[12].data[row_index] = mem0_base_col12;
            let mem1_base_col13 =
                (((op1_base_fp_col8) * (input_fp_col2)) + ((op1_base_ap_col9) * (input_ap_col1)));
            trace[13].data[row_index] = mem1_base_col13;

            // Read Positive Num Bits 252.

            let memory_address_to_id_value_tmp_5a14_11 = memory_address_to_id_state
                .deduce_output(((mem_dst_base_col11) + ((offset0_col3) - (M31_32768))));
            let memory_id_to_big_value_tmp_5a14_12 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_5a14_11);
            let dst_id_col14 = memory_address_to_id_value_tmp_5a14_11;
            trace[14].data[row_index] = dst_id_col14;
            sub_components_inputs.memory_address_to_id_inputs[0]
                .extend(((mem_dst_base_col11) + ((offset0_col3) - (M31_32768))).unpack());

            lookup_data.memory_address_to_id_0.push([
                ((mem_dst_base_col11) + ((offset0_col3) - (M31_32768))),
                dst_id_col14,
            ]);
            let dst_limb_0_col15 = memory_id_to_big_value_tmp_5a14_12.get_m31(0);
            trace[15].data[row_index] = dst_limb_0_col15;
            let dst_limb_1_col16 = memory_id_to_big_value_tmp_5a14_12.get_m31(1);
            trace[16].data[row_index] = dst_limb_1_col16;
            let dst_limb_2_col17 = memory_id_to_big_value_tmp_5a14_12.get_m31(2);
            trace[17].data[row_index] = dst_limb_2_col17;
            let dst_limb_3_col18 = memory_id_to_big_value_tmp_5a14_12.get_m31(3);
            trace[18].data[row_index] = dst_limb_3_col18;
            let dst_limb_4_col19 = memory_id_to_big_value_tmp_5a14_12.get_m31(4);
            trace[19].data[row_index] = dst_limb_4_col19;
            let dst_limb_5_col20 = memory_id_to_big_value_tmp_5a14_12.get_m31(5);
            trace[20].data[row_index] = dst_limb_5_col20;
            let dst_limb_6_col21 = memory_id_to_big_value_tmp_5a14_12.get_m31(6);
            trace[21].data[row_index] = dst_limb_6_col21;
            let dst_limb_7_col22 = memory_id_to_big_value_tmp_5a14_12.get_m31(7);
            trace[22].data[row_index] = dst_limb_7_col22;
            let dst_limb_8_col23 = memory_id_to_big_value_tmp_5a14_12.get_m31(8);
            trace[23].data[row_index] = dst_limb_8_col23;
            let dst_limb_9_col24 = memory_id_to_big_value_tmp_5a14_12.get_m31(9);
            trace[24].data[row_index] = dst_limb_9_col24;
            let dst_limb_10_col25 = memory_id_to_big_value_tmp_5a14_12.get_m31(10);
            trace[25].data[row_index] = dst_limb_10_col25;
            let dst_limb_11_col26 = memory_id_to_big_value_tmp_5a14_12.get_m31(11);
            trace[26].data[row_index] = dst_limb_11_col26;
            let dst_limb_12_col27 = memory_id_to_big_value_tmp_5a14_12.get_m31(12);
            trace[27].data[row_index] = dst_limb_12_col27;
            let dst_limb_13_col28 = memory_id_to_big_value_tmp_5a14_12.get_m31(13);
            trace[28].data[row_index] = dst_limb_13_col28;
            let dst_limb_14_col29 = memory_id_to_big_value_tmp_5a14_12.get_m31(14);
            trace[29].data[row_index] = dst_limb_14_col29;
            let dst_limb_15_col30 = memory_id_to_big_value_tmp_5a14_12.get_m31(15);
            trace[30].data[row_index] = dst_limb_15_col30;
            let dst_limb_16_col31 = memory_id_to_big_value_tmp_5a14_12.get_m31(16);
            trace[31].data[row_index] = dst_limb_16_col31;
            let dst_limb_17_col32 = memory_id_to_big_value_tmp_5a14_12.get_m31(17);
            trace[32].data[row_index] = dst_limb_17_col32;
            let dst_limb_18_col33 = memory_id_to_big_value_tmp_5a14_12.get_m31(18);
            trace[33].data[row_index] = dst_limb_18_col33;
            let dst_limb_19_col34 = memory_id_to_big_value_tmp_5a14_12.get_m31(19);
            trace[34].data[row_index] = dst_limb_19_col34;
            let dst_limb_20_col35 = memory_id_to_big_value_tmp_5a14_12.get_m31(20);
            trace[35].data[row_index] = dst_limb_20_col35;
            let dst_limb_21_col36 = memory_id_to_big_value_tmp_5a14_12.get_m31(21);
            trace[36].data[row_index] = dst_limb_21_col36;
            let dst_limb_22_col37 = memory_id_to_big_value_tmp_5a14_12.get_m31(22);
            trace[37].data[row_index] = dst_limb_22_col37;
            let dst_limb_23_col38 = memory_id_to_big_value_tmp_5a14_12.get_m31(23);
            trace[38].data[row_index] = dst_limb_23_col38;
            let dst_limb_24_col39 = memory_id_to_big_value_tmp_5a14_12.get_m31(24);
            trace[39].data[row_index] = dst_limb_24_col39;
            let dst_limb_25_col40 = memory_id_to_big_value_tmp_5a14_12.get_m31(25);
            trace[40].data[row_index] = dst_limb_25_col40;
            let dst_limb_26_col41 = memory_id_to_big_value_tmp_5a14_12.get_m31(26);
            trace[41].data[row_index] = dst_limb_26_col41;
            let dst_limb_27_col42 = memory_id_to_big_value_tmp_5a14_12.get_m31(27);
            trace[42].data[row_index] = dst_limb_27_col42;
            sub_components_inputs.memory_id_to_big_inputs[0].extend(dst_id_col14.unpack());

            lookup_data.memory_id_to_big_0.push([
                dst_id_col14,
                dst_limb_0_col15,
                dst_limb_1_col16,
                dst_limb_2_col17,
                dst_limb_3_col18,
                dst_limb_4_col19,
                dst_limb_5_col20,
                dst_limb_6_col21,
                dst_limb_7_col22,
                dst_limb_8_col23,
                dst_limb_9_col24,
                dst_limb_10_col25,
                dst_limb_11_col26,
                dst_limb_12_col27,
                dst_limb_13_col28,
                dst_limb_14_col29,
                dst_limb_15_col30,
                dst_limb_16_col31,
                dst_limb_17_col32,
                dst_limb_18_col33,
                dst_limb_19_col34,
                dst_limb_20_col35,
                dst_limb_21_col36,
                dst_limb_22_col37,
                dst_limb_23_col38,
                dst_limb_24_col39,
                dst_limb_25_col40,
                dst_limb_26_col41,
                dst_limb_27_col42,
            ]);

            // Read Positive Num Bits 252.

            let memory_address_to_id_value_tmp_5a14_13 = memory_address_to_id_state
                .deduce_output(((mem0_base_col12) + ((offset1_col4) - (M31_32768))));
            let memory_id_to_big_value_tmp_5a14_14 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_5a14_13);
            let op0_id_col43 = memory_address_to_id_value_tmp_5a14_13;
            trace[43].data[row_index] = op0_id_col43;
            sub_components_inputs.memory_address_to_id_inputs[1]
                .extend(((mem0_base_col12) + ((offset1_col4) - (M31_32768))).unpack());

            lookup_data.memory_address_to_id_1.push([
                ((mem0_base_col12) + ((offset1_col4) - (M31_32768))),
                op0_id_col43,
            ]);
            let op0_limb_0_col44 = memory_id_to_big_value_tmp_5a14_14.get_m31(0);
            trace[44].data[row_index] = op0_limb_0_col44;
            let op0_limb_1_col45 = memory_id_to_big_value_tmp_5a14_14.get_m31(1);
            trace[45].data[row_index] = op0_limb_1_col45;
            let op0_limb_2_col46 = memory_id_to_big_value_tmp_5a14_14.get_m31(2);
            trace[46].data[row_index] = op0_limb_2_col46;
            let op0_limb_3_col47 = memory_id_to_big_value_tmp_5a14_14.get_m31(3);
            trace[47].data[row_index] = op0_limb_3_col47;
            let op0_limb_4_col48 = memory_id_to_big_value_tmp_5a14_14.get_m31(4);
            trace[48].data[row_index] = op0_limb_4_col48;
            let op0_limb_5_col49 = memory_id_to_big_value_tmp_5a14_14.get_m31(5);
            trace[49].data[row_index] = op0_limb_5_col49;
            let op0_limb_6_col50 = memory_id_to_big_value_tmp_5a14_14.get_m31(6);
            trace[50].data[row_index] = op0_limb_6_col50;
            let op0_limb_7_col51 = memory_id_to_big_value_tmp_5a14_14.get_m31(7);
            trace[51].data[row_index] = op0_limb_7_col51;
            let op0_limb_8_col52 = memory_id_to_big_value_tmp_5a14_14.get_m31(8);
            trace[52].data[row_index] = op0_limb_8_col52;
            let op0_limb_9_col53 = memory_id_to_big_value_tmp_5a14_14.get_m31(9);
            trace[53].data[row_index] = op0_limb_9_col53;
            let op0_limb_10_col54 = memory_id_to_big_value_tmp_5a14_14.get_m31(10);
            trace[54].data[row_index] = op0_limb_10_col54;
            let op0_limb_11_col55 = memory_id_to_big_value_tmp_5a14_14.get_m31(11);
            trace[55].data[row_index] = op0_limb_11_col55;
            let op0_limb_12_col56 = memory_id_to_big_value_tmp_5a14_14.get_m31(12);
            trace[56].data[row_index] = op0_limb_12_col56;
            let op0_limb_13_col57 = memory_id_to_big_value_tmp_5a14_14.get_m31(13);
            trace[57].data[row_index] = op0_limb_13_col57;
            let op0_limb_14_col58 = memory_id_to_big_value_tmp_5a14_14.get_m31(14);
            trace[58].data[row_index] = op0_limb_14_col58;
            let op0_limb_15_col59 = memory_id_to_big_value_tmp_5a14_14.get_m31(15);
            trace[59].data[row_index] = op0_limb_15_col59;
            let op0_limb_16_col60 = memory_id_to_big_value_tmp_5a14_14.get_m31(16);
            trace[60].data[row_index] = op0_limb_16_col60;
            let op0_limb_17_col61 = memory_id_to_big_value_tmp_5a14_14.get_m31(17);
            trace[61].data[row_index] = op0_limb_17_col61;
            let op0_limb_18_col62 = memory_id_to_big_value_tmp_5a14_14.get_m31(18);
            trace[62].data[row_index] = op0_limb_18_col62;
            let op0_limb_19_col63 = memory_id_to_big_value_tmp_5a14_14.get_m31(19);
            trace[63].data[row_index] = op0_limb_19_col63;
            let op0_limb_20_col64 = memory_id_to_big_value_tmp_5a14_14.get_m31(20);
            trace[64].data[row_index] = op0_limb_20_col64;
            let op0_limb_21_col65 = memory_id_to_big_value_tmp_5a14_14.get_m31(21);
            trace[65].data[row_index] = op0_limb_21_col65;
            let op0_limb_22_col66 = memory_id_to_big_value_tmp_5a14_14.get_m31(22);
            trace[66].data[row_index] = op0_limb_22_col66;
            let op0_limb_23_col67 = memory_id_to_big_value_tmp_5a14_14.get_m31(23);
            trace[67].data[row_index] = op0_limb_23_col67;
            let op0_limb_24_col68 = memory_id_to_big_value_tmp_5a14_14.get_m31(24);
            trace[68].data[row_index] = op0_limb_24_col68;
            let op0_limb_25_col69 = memory_id_to_big_value_tmp_5a14_14.get_m31(25);
            trace[69].data[row_index] = op0_limb_25_col69;
            let op0_limb_26_col70 = memory_id_to_big_value_tmp_5a14_14.get_m31(26);
            trace[70].data[row_index] = op0_limb_26_col70;
            let op0_limb_27_col71 = memory_id_to_big_value_tmp_5a14_14.get_m31(27);
            trace[71].data[row_index] = op0_limb_27_col71;
            sub_components_inputs.memory_id_to_big_inputs[1].extend(op0_id_col43.unpack());

            lookup_data.memory_id_to_big_1.push([
                op0_id_col43,
                op0_limb_0_col44,
                op0_limb_1_col45,
                op0_limb_2_col46,
                op0_limb_3_col47,
                op0_limb_4_col48,
                op0_limb_5_col49,
                op0_limb_6_col50,
                op0_limb_7_col51,
                op0_limb_8_col52,
                op0_limb_9_col53,
                op0_limb_10_col54,
                op0_limb_11_col55,
                op0_limb_12_col56,
                op0_limb_13_col57,
                op0_limb_14_col58,
                op0_limb_15_col59,
                op0_limb_16_col60,
                op0_limb_17_col61,
                op0_limb_18_col62,
                op0_limb_19_col63,
                op0_limb_20_col64,
                op0_limb_21_col65,
                op0_limb_22_col66,
                op0_limb_23_col67,
                op0_limb_24_col68,
                op0_limb_25_col69,
                op0_limb_26_col70,
                op0_limb_27_col71,
            ]);

            // Read Positive Num Bits 252.

            let memory_address_to_id_value_tmp_5a14_15 = memory_address_to_id_state
                .deduce_output(((mem1_base_col13) + ((offset2_col5) - (M31_32768))));
            let memory_id_to_big_value_tmp_5a14_16 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_5a14_15);
            let op1_id_col72 = memory_address_to_id_value_tmp_5a14_15;
            trace[72].data[row_index] = op1_id_col72;
            sub_components_inputs.memory_address_to_id_inputs[2]
                .extend(((mem1_base_col13) + ((offset2_col5) - (M31_32768))).unpack());

            lookup_data.memory_address_to_id_2.push([
                ((mem1_base_col13) + ((offset2_col5) - (M31_32768))),
                op1_id_col72,
            ]);
            let op1_limb_0_col73 = memory_id_to_big_value_tmp_5a14_16.get_m31(0);
            trace[73].data[row_index] = op1_limb_0_col73;
            let op1_limb_1_col74 = memory_id_to_big_value_tmp_5a14_16.get_m31(1);
            trace[74].data[row_index] = op1_limb_1_col74;
            let op1_limb_2_col75 = memory_id_to_big_value_tmp_5a14_16.get_m31(2);
            trace[75].data[row_index] = op1_limb_2_col75;
            let op1_limb_3_col76 = memory_id_to_big_value_tmp_5a14_16.get_m31(3);
            trace[76].data[row_index] = op1_limb_3_col76;
            let op1_limb_4_col77 = memory_id_to_big_value_tmp_5a14_16.get_m31(4);
            trace[77].data[row_index] = op1_limb_4_col77;
            let op1_limb_5_col78 = memory_id_to_big_value_tmp_5a14_16.get_m31(5);
            trace[78].data[row_index] = op1_limb_5_col78;
            let op1_limb_6_col79 = memory_id_to_big_value_tmp_5a14_16.get_m31(6);
            trace[79].data[row_index] = op1_limb_6_col79;
            let op1_limb_7_col80 = memory_id_to_big_value_tmp_5a14_16.get_m31(7);
            trace[80].data[row_index] = op1_limb_7_col80;
            let op1_limb_8_col81 = memory_id_to_big_value_tmp_5a14_16.get_m31(8);
            trace[81].data[row_index] = op1_limb_8_col81;
            let op1_limb_9_col82 = memory_id_to_big_value_tmp_5a14_16.get_m31(9);
            trace[82].data[row_index] = op1_limb_9_col82;
            let op1_limb_10_col83 = memory_id_to_big_value_tmp_5a14_16.get_m31(10);
            trace[83].data[row_index] = op1_limb_10_col83;
            let op1_limb_11_col84 = memory_id_to_big_value_tmp_5a14_16.get_m31(11);
            trace[84].data[row_index] = op1_limb_11_col84;
            let op1_limb_12_col85 = memory_id_to_big_value_tmp_5a14_16.get_m31(12);
            trace[85].data[row_index] = op1_limb_12_col85;
            let op1_limb_13_col86 = memory_id_to_big_value_tmp_5a14_16.get_m31(13);
            trace[86].data[row_index] = op1_limb_13_col86;
            let op1_limb_14_col87 = memory_id_to_big_value_tmp_5a14_16.get_m31(14);
            trace[87].data[row_index] = op1_limb_14_col87;
            let op1_limb_15_col88 = memory_id_to_big_value_tmp_5a14_16.get_m31(15);
            trace[88].data[row_index] = op1_limb_15_col88;
            let op1_limb_16_col89 = memory_id_to_big_value_tmp_5a14_16.get_m31(16);
            trace[89].data[row_index] = op1_limb_16_col89;
            let op1_limb_17_col90 = memory_id_to_big_value_tmp_5a14_16.get_m31(17);
            trace[90].data[row_index] = op1_limb_17_col90;
            let op1_limb_18_col91 = memory_id_to_big_value_tmp_5a14_16.get_m31(18);
            trace[91].data[row_index] = op1_limb_18_col91;
            let op1_limb_19_col92 = memory_id_to_big_value_tmp_5a14_16.get_m31(19);
            trace[92].data[row_index] = op1_limb_19_col92;
            let op1_limb_20_col93 = memory_id_to_big_value_tmp_5a14_16.get_m31(20);
            trace[93].data[row_index] = op1_limb_20_col93;
            let op1_limb_21_col94 = memory_id_to_big_value_tmp_5a14_16.get_m31(21);
            trace[94].data[row_index] = op1_limb_21_col94;
            let op1_limb_22_col95 = memory_id_to_big_value_tmp_5a14_16.get_m31(22);
            trace[95].data[row_index] = op1_limb_22_col95;
            let op1_limb_23_col96 = memory_id_to_big_value_tmp_5a14_16.get_m31(23);
            trace[96].data[row_index] = op1_limb_23_col96;
            let op1_limb_24_col97 = memory_id_to_big_value_tmp_5a14_16.get_m31(24);
            trace[97].data[row_index] = op1_limb_24_col97;
            let op1_limb_25_col98 = memory_id_to_big_value_tmp_5a14_16.get_m31(25);
            trace[98].data[row_index] = op1_limb_25_col98;
            let op1_limb_26_col99 = memory_id_to_big_value_tmp_5a14_16.get_m31(26);
            trace[99].data[row_index] = op1_limb_26_col99;
            let op1_limb_27_col100 = memory_id_to_big_value_tmp_5a14_16.get_m31(27);
            trace[100].data[row_index] = op1_limb_27_col100;
            sub_components_inputs.memory_id_to_big_inputs[2].extend(op1_id_col72.unpack());

            lookup_data.memory_id_to_big_2.push([
                op1_id_col72,
                op1_limb_0_col73,
                op1_limb_1_col74,
                op1_limb_2_col75,
                op1_limb_3_col76,
                op1_limb_4_col77,
                op1_limb_5_col78,
                op1_limb_6_col79,
                op1_limb_7_col80,
                op1_limb_8_col81,
                op1_limb_9_col82,
                op1_limb_10_col83,
                op1_limb_11_col84,
                op1_limb_12_col85,
                op1_limb_13_col86,
                op1_limb_14_col87,
                op1_limb_15_col88,
                op1_limb_16_col89,
                op1_limb_17_col90,
                op1_limb_18_col91,
                op1_limb_19_col92,
                op1_limb_20_col93,
                op1_limb_21_col94,
                op1_limb_22_col95,
                op1_limb_23_col96,
                op1_limb_24_col97,
                op1_limb_25_col98,
                op1_limb_26_col99,
                op1_limb_27_col100,
            ]);

            // Verify Mul 252.

            let conv_tmp_5a14_17 =
                (((M31_0) - (dst_limb_0_col15)) + ((op0_limb_0_col44) * (op1_limb_0_col73)));
            let conv_tmp_5a14_18 = ((((M31_0) - (dst_limb_1_col16))
                + ((op0_limb_0_col44) * (op1_limb_1_col74)))
                + ((op0_limb_1_col45) * (op1_limb_0_col73)));
            let conv_tmp_5a14_19 = (((((M31_0) - (dst_limb_2_col17))
                + ((op0_limb_0_col44) * (op1_limb_2_col75)))
                + ((op0_limb_1_col45) * (op1_limb_1_col74)))
                + ((op0_limb_2_col46) * (op1_limb_0_col73)));
            let conv_tmp_5a14_20 = ((((((M31_0) - (dst_limb_3_col18))
                + ((op0_limb_0_col44) * (op1_limb_3_col76)))
                + ((op0_limb_1_col45) * (op1_limb_2_col75)))
                + ((op0_limb_2_col46) * (op1_limb_1_col74)))
                + ((op0_limb_3_col47) * (op1_limb_0_col73)));
            let conv_tmp_5a14_21 = (((((((M31_0) - (dst_limb_4_col19))
                + ((op0_limb_0_col44) * (op1_limb_4_col77)))
                + ((op0_limb_1_col45) * (op1_limb_3_col76)))
                + ((op0_limb_2_col46) * (op1_limb_2_col75)))
                + ((op0_limb_3_col47) * (op1_limb_1_col74)))
                + ((op0_limb_4_col48) * (op1_limb_0_col73)));
            let conv_tmp_5a14_22 = ((((((((M31_0) - (dst_limb_5_col20))
                + ((op0_limb_0_col44) * (op1_limb_5_col78)))
                + ((op0_limb_1_col45) * (op1_limb_4_col77)))
                + ((op0_limb_2_col46) * (op1_limb_3_col76)))
                + ((op0_limb_3_col47) * (op1_limb_2_col75)))
                + ((op0_limb_4_col48) * (op1_limb_1_col74)))
                + ((op0_limb_5_col49) * (op1_limb_0_col73)));
            let conv_tmp_5a14_23 = (((((((((M31_0) - (dst_limb_6_col21))
                + ((op0_limb_0_col44) * (op1_limb_6_col79)))
                + ((op0_limb_1_col45) * (op1_limb_5_col78)))
                + ((op0_limb_2_col46) * (op1_limb_4_col77)))
                + ((op0_limb_3_col47) * (op1_limb_3_col76)))
                + ((op0_limb_4_col48) * (op1_limb_2_col75)))
                + ((op0_limb_5_col49) * (op1_limb_1_col74)))
                + ((op0_limb_6_col50) * (op1_limb_0_col73)));
            let conv_tmp_5a14_24 = ((((((((((M31_0) - (dst_limb_7_col22))
                + ((op0_limb_0_col44) * (op1_limb_7_col80)))
                + ((op0_limb_1_col45) * (op1_limb_6_col79)))
                + ((op0_limb_2_col46) * (op1_limb_5_col78)))
                + ((op0_limb_3_col47) * (op1_limb_4_col77)))
                + ((op0_limb_4_col48) * (op1_limb_3_col76)))
                + ((op0_limb_5_col49) * (op1_limb_2_col75)))
                + ((op0_limb_6_col50) * (op1_limb_1_col74)))
                + ((op0_limb_7_col51) * (op1_limb_0_col73)));
            let conv_tmp_5a14_25 = (((((((((((M31_0) - (dst_limb_8_col23))
                + ((op0_limb_0_col44) * (op1_limb_8_col81)))
                + ((op0_limb_1_col45) * (op1_limb_7_col80)))
                + ((op0_limb_2_col46) * (op1_limb_6_col79)))
                + ((op0_limb_3_col47) * (op1_limb_5_col78)))
                + ((op0_limb_4_col48) * (op1_limb_4_col77)))
                + ((op0_limb_5_col49) * (op1_limb_3_col76)))
                + ((op0_limb_6_col50) * (op1_limb_2_col75)))
                + ((op0_limb_7_col51) * (op1_limb_1_col74)))
                + ((op0_limb_8_col52) * (op1_limb_0_col73)));
            let conv_tmp_5a14_26 = ((((((((((((M31_0) - (dst_limb_9_col24))
                + ((op0_limb_0_col44) * (op1_limb_9_col82)))
                + ((op0_limb_1_col45) * (op1_limb_8_col81)))
                + ((op0_limb_2_col46) * (op1_limb_7_col80)))
                + ((op0_limb_3_col47) * (op1_limb_6_col79)))
                + ((op0_limb_4_col48) * (op1_limb_5_col78)))
                + ((op0_limb_5_col49) * (op1_limb_4_col77)))
                + ((op0_limb_6_col50) * (op1_limb_3_col76)))
                + ((op0_limb_7_col51) * (op1_limb_2_col75)))
                + ((op0_limb_8_col52) * (op1_limb_1_col74)))
                + ((op0_limb_9_col53) * (op1_limb_0_col73)));
            let conv_tmp_5a14_27 = (((((((((((((M31_0) - (dst_limb_10_col25))
                + ((op0_limb_0_col44) * (op1_limb_10_col83)))
                + ((op0_limb_1_col45) * (op1_limb_9_col82)))
                + ((op0_limb_2_col46) * (op1_limb_8_col81)))
                + ((op0_limb_3_col47) * (op1_limb_7_col80)))
                + ((op0_limb_4_col48) * (op1_limb_6_col79)))
                + ((op0_limb_5_col49) * (op1_limb_5_col78)))
                + ((op0_limb_6_col50) * (op1_limb_4_col77)))
                + ((op0_limb_7_col51) * (op1_limb_3_col76)))
                + ((op0_limb_8_col52) * (op1_limb_2_col75)))
                + ((op0_limb_9_col53) * (op1_limb_1_col74)))
                + ((op0_limb_10_col54) * (op1_limb_0_col73)));
            let conv_tmp_5a14_28 = ((((((((((((((M31_0) - (dst_limb_11_col26))
                + ((op0_limb_0_col44) * (op1_limb_11_col84)))
                + ((op0_limb_1_col45) * (op1_limb_10_col83)))
                + ((op0_limb_2_col46) * (op1_limb_9_col82)))
                + ((op0_limb_3_col47) * (op1_limb_8_col81)))
                + ((op0_limb_4_col48) * (op1_limb_7_col80)))
                + ((op0_limb_5_col49) * (op1_limb_6_col79)))
                + ((op0_limb_6_col50) * (op1_limb_5_col78)))
                + ((op0_limb_7_col51) * (op1_limb_4_col77)))
                + ((op0_limb_8_col52) * (op1_limb_3_col76)))
                + ((op0_limb_9_col53) * (op1_limb_2_col75)))
                + ((op0_limb_10_col54) * (op1_limb_1_col74)))
                + ((op0_limb_11_col55) * (op1_limb_0_col73)));
            let conv_tmp_5a14_29 = (((((((((((((((M31_0) - (dst_limb_12_col27))
                + ((op0_limb_0_col44) * (op1_limb_12_col85)))
                + ((op0_limb_1_col45) * (op1_limb_11_col84)))
                + ((op0_limb_2_col46) * (op1_limb_10_col83)))
                + ((op0_limb_3_col47) * (op1_limb_9_col82)))
                + ((op0_limb_4_col48) * (op1_limb_8_col81)))
                + ((op0_limb_5_col49) * (op1_limb_7_col80)))
                + ((op0_limb_6_col50) * (op1_limb_6_col79)))
                + ((op0_limb_7_col51) * (op1_limb_5_col78)))
                + ((op0_limb_8_col52) * (op1_limb_4_col77)))
                + ((op0_limb_9_col53) * (op1_limb_3_col76)))
                + ((op0_limb_10_col54) * (op1_limb_2_col75)))
                + ((op0_limb_11_col55) * (op1_limb_1_col74)))
                + ((op0_limb_12_col56) * (op1_limb_0_col73)));
            let conv_tmp_5a14_30 = ((((((((((((((((M31_0) - (dst_limb_13_col28))
                + ((op0_limb_0_col44) * (op1_limb_13_col86)))
                + ((op0_limb_1_col45) * (op1_limb_12_col85)))
                + ((op0_limb_2_col46) * (op1_limb_11_col84)))
                + ((op0_limb_3_col47) * (op1_limb_10_col83)))
                + ((op0_limb_4_col48) * (op1_limb_9_col82)))
                + ((op0_limb_5_col49) * (op1_limb_8_col81)))
                + ((op0_limb_6_col50) * (op1_limb_7_col80)))
                + ((op0_limb_7_col51) * (op1_limb_6_col79)))
                + ((op0_limb_8_col52) * (op1_limb_5_col78)))
                + ((op0_limb_9_col53) * (op1_limb_4_col77)))
                + ((op0_limb_10_col54) * (op1_limb_3_col76)))
                + ((op0_limb_11_col55) * (op1_limb_2_col75)))
                + ((op0_limb_12_col56) * (op1_limb_1_col74)))
                + ((op0_limb_13_col57) * (op1_limb_0_col73)));
            let conv_tmp_5a14_31 = (((((((((((((((((M31_0) - (dst_limb_14_col29))
                + ((op0_limb_0_col44) * (op1_limb_14_col87)))
                + ((op0_limb_1_col45) * (op1_limb_13_col86)))
                + ((op0_limb_2_col46) * (op1_limb_12_col85)))
                + ((op0_limb_3_col47) * (op1_limb_11_col84)))
                + ((op0_limb_4_col48) * (op1_limb_10_col83)))
                + ((op0_limb_5_col49) * (op1_limb_9_col82)))
                + ((op0_limb_6_col50) * (op1_limb_8_col81)))
                + ((op0_limb_7_col51) * (op1_limb_7_col80)))
                + ((op0_limb_8_col52) * (op1_limb_6_col79)))
                + ((op0_limb_9_col53) * (op1_limb_5_col78)))
                + ((op0_limb_10_col54) * (op1_limb_4_col77)))
                + ((op0_limb_11_col55) * (op1_limb_3_col76)))
                + ((op0_limb_12_col56) * (op1_limb_2_col75)))
                + ((op0_limb_13_col57) * (op1_limb_1_col74)))
                + ((op0_limb_14_col58) * (op1_limb_0_col73)));
            let conv_tmp_5a14_32 = ((((((((((((((((((M31_0) - (dst_limb_15_col30))
                + ((op0_limb_0_col44) * (op1_limb_15_col88)))
                + ((op0_limb_1_col45) * (op1_limb_14_col87)))
                + ((op0_limb_2_col46) * (op1_limb_13_col86)))
                + ((op0_limb_3_col47) * (op1_limb_12_col85)))
                + ((op0_limb_4_col48) * (op1_limb_11_col84)))
                + ((op0_limb_5_col49) * (op1_limb_10_col83)))
                + ((op0_limb_6_col50) * (op1_limb_9_col82)))
                + ((op0_limb_7_col51) * (op1_limb_8_col81)))
                + ((op0_limb_8_col52) * (op1_limb_7_col80)))
                + ((op0_limb_9_col53) * (op1_limb_6_col79)))
                + ((op0_limb_10_col54) * (op1_limb_5_col78)))
                + ((op0_limb_11_col55) * (op1_limb_4_col77)))
                + ((op0_limb_12_col56) * (op1_limb_3_col76)))
                + ((op0_limb_13_col57) * (op1_limb_2_col75)))
                + ((op0_limb_14_col58) * (op1_limb_1_col74)))
                + ((op0_limb_15_col59) * (op1_limb_0_col73)));
            let conv_tmp_5a14_33 = (((((((((((((((((((M31_0)
                - (dst_limb_16_col31))
                + ((op0_limb_0_col44) * (op1_limb_16_col89)))
                + ((op0_limb_1_col45) * (op1_limb_15_col88)))
                + ((op0_limb_2_col46) * (op1_limb_14_col87)))
                + ((op0_limb_3_col47) * (op1_limb_13_col86)))
                + ((op0_limb_4_col48) * (op1_limb_12_col85)))
                + ((op0_limb_5_col49) * (op1_limb_11_col84)))
                + ((op0_limb_6_col50) * (op1_limb_10_col83)))
                + ((op0_limb_7_col51) * (op1_limb_9_col82)))
                + ((op0_limb_8_col52) * (op1_limb_8_col81)))
                + ((op0_limb_9_col53) * (op1_limb_7_col80)))
                + ((op0_limb_10_col54) * (op1_limb_6_col79)))
                + ((op0_limb_11_col55) * (op1_limb_5_col78)))
                + ((op0_limb_12_col56) * (op1_limb_4_col77)))
                + ((op0_limb_13_col57) * (op1_limb_3_col76)))
                + ((op0_limb_14_col58) * (op1_limb_2_col75)))
                + ((op0_limb_15_col59) * (op1_limb_1_col74)))
                + ((op0_limb_16_col60) * (op1_limb_0_col73)));
            let conv_tmp_5a14_34 = ((((((((((((((((((((M31_0)
                - (dst_limb_17_col32))
                + ((op0_limb_0_col44) * (op1_limb_17_col90)))
                + ((op0_limb_1_col45) * (op1_limb_16_col89)))
                + ((op0_limb_2_col46) * (op1_limb_15_col88)))
                + ((op0_limb_3_col47) * (op1_limb_14_col87)))
                + ((op0_limb_4_col48) * (op1_limb_13_col86)))
                + ((op0_limb_5_col49) * (op1_limb_12_col85)))
                + ((op0_limb_6_col50) * (op1_limb_11_col84)))
                + ((op0_limb_7_col51) * (op1_limb_10_col83)))
                + ((op0_limb_8_col52) * (op1_limb_9_col82)))
                + ((op0_limb_9_col53) * (op1_limb_8_col81)))
                + ((op0_limb_10_col54) * (op1_limb_7_col80)))
                + ((op0_limb_11_col55) * (op1_limb_6_col79)))
                + ((op0_limb_12_col56) * (op1_limb_5_col78)))
                + ((op0_limb_13_col57) * (op1_limb_4_col77)))
                + ((op0_limb_14_col58) * (op1_limb_3_col76)))
                + ((op0_limb_15_col59) * (op1_limb_2_col75)))
                + ((op0_limb_16_col60) * (op1_limb_1_col74)))
                + ((op0_limb_17_col61) * (op1_limb_0_col73)));
            let conv_tmp_5a14_35 = (((((((((((((((((((((M31_0)
                - (dst_limb_18_col33))
                + ((op0_limb_0_col44) * (op1_limb_18_col91)))
                + ((op0_limb_1_col45) * (op1_limb_17_col90)))
                + ((op0_limb_2_col46) * (op1_limb_16_col89)))
                + ((op0_limb_3_col47) * (op1_limb_15_col88)))
                + ((op0_limb_4_col48) * (op1_limb_14_col87)))
                + ((op0_limb_5_col49) * (op1_limb_13_col86)))
                + ((op0_limb_6_col50) * (op1_limb_12_col85)))
                + ((op0_limb_7_col51) * (op1_limb_11_col84)))
                + ((op0_limb_8_col52) * (op1_limb_10_col83)))
                + ((op0_limb_9_col53) * (op1_limb_9_col82)))
                + ((op0_limb_10_col54) * (op1_limb_8_col81)))
                + ((op0_limb_11_col55) * (op1_limb_7_col80)))
                + ((op0_limb_12_col56) * (op1_limb_6_col79)))
                + ((op0_limb_13_col57) * (op1_limb_5_col78)))
                + ((op0_limb_14_col58) * (op1_limb_4_col77)))
                + ((op0_limb_15_col59) * (op1_limb_3_col76)))
                + ((op0_limb_16_col60) * (op1_limb_2_col75)))
                + ((op0_limb_17_col61) * (op1_limb_1_col74)))
                + ((op0_limb_18_col62) * (op1_limb_0_col73)));
            let conv_tmp_5a14_36 = ((((((((((((((((((((((M31_0)
                - (dst_limb_19_col34))
                + ((op0_limb_0_col44) * (op1_limb_19_col92)))
                + ((op0_limb_1_col45) * (op1_limb_18_col91)))
                + ((op0_limb_2_col46) * (op1_limb_17_col90)))
                + ((op0_limb_3_col47) * (op1_limb_16_col89)))
                + ((op0_limb_4_col48) * (op1_limb_15_col88)))
                + ((op0_limb_5_col49) * (op1_limb_14_col87)))
                + ((op0_limb_6_col50) * (op1_limb_13_col86)))
                + ((op0_limb_7_col51) * (op1_limb_12_col85)))
                + ((op0_limb_8_col52) * (op1_limb_11_col84)))
                + ((op0_limb_9_col53) * (op1_limb_10_col83)))
                + ((op0_limb_10_col54) * (op1_limb_9_col82)))
                + ((op0_limb_11_col55) * (op1_limb_8_col81)))
                + ((op0_limb_12_col56) * (op1_limb_7_col80)))
                + ((op0_limb_13_col57) * (op1_limb_6_col79)))
                + ((op0_limb_14_col58) * (op1_limb_5_col78)))
                + ((op0_limb_15_col59) * (op1_limb_4_col77)))
                + ((op0_limb_16_col60) * (op1_limb_3_col76)))
                + ((op0_limb_17_col61) * (op1_limb_2_col75)))
                + ((op0_limb_18_col62) * (op1_limb_1_col74)))
                + ((op0_limb_19_col63) * (op1_limb_0_col73)));
            let conv_tmp_5a14_37 = (((((((((((((((((((((((M31_0)
                - (dst_limb_20_col35))
                + ((op0_limb_0_col44) * (op1_limb_20_col93)))
                + ((op0_limb_1_col45) * (op1_limb_19_col92)))
                + ((op0_limb_2_col46) * (op1_limb_18_col91)))
                + ((op0_limb_3_col47) * (op1_limb_17_col90)))
                + ((op0_limb_4_col48) * (op1_limb_16_col89)))
                + ((op0_limb_5_col49) * (op1_limb_15_col88)))
                + ((op0_limb_6_col50) * (op1_limb_14_col87)))
                + ((op0_limb_7_col51) * (op1_limb_13_col86)))
                + ((op0_limb_8_col52) * (op1_limb_12_col85)))
                + ((op0_limb_9_col53) * (op1_limb_11_col84)))
                + ((op0_limb_10_col54) * (op1_limb_10_col83)))
                + ((op0_limb_11_col55) * (op1_limb_9_col82)))
                + ((op0_limb_12_col56) * (op1_limb_8_col81)))
                + ((op0_limb_13_col57) * (op1_limb_7_col80)))
                + ((op0_limb_14_col58) * (op1_limb_6_col79)))
                + ((op0_limb_15_col59) * (op1_limb_5_col78)))
                + ((op0_limb_16_col60) * (op1_limb_4_col77)))
                + ((op0_limb_17_col61) * (op1_limb_3_col76)))
                + ((op0_limb_18_col62) * (op1_limb_2_col75)))
                + ((op0_limb_19_col63) * (op1_limb_1_col74)))
                + ((op0_limb_20_col64) * (op1_limb_0_col73)));
            let conv_tmp_5a14_38 = ((((((((((((((((((((((((M31_0)
                - (dst_limb_21_col36))
                + ((op0_limb_0_col44) * (op1_limb_21_col94)))
                + ((op0_limb_1_col45) * (op1_limb_20_col93)))
                + ((op0_limb_2_col46) * (op1_limb_19_col92)))
                + ((op0_limb_3_col47) * (op1_limb_18_col91)))
                + ((op0_limb_4_col48) * (op1_limb_17_col90)))
                + ((op0_limb_5_col49) * (op1_limb_16_col89)))
                + ((op0_limb_6_col50) * (op1_limb_15_col88)))
                + ((op0_limb_7_col51) * (op1_limb_14_col87)))
                + ((op0_limb_8_col52) * (op1_limb_13_col86)))
                + ((op0_limb_9_col53) * (op1_limb_12_col85)))
                + ((op0_limb_10_col54) * (op1_limb_11_col84)))
                + ((op0_limb_11_col55) * (op1_limb_10_col83)))
                + ((op0_limb_12_col56) * (op1_limb_9_col82)))
                + ((op0_limb_13_col57) * (op1_limb_8_col81)))
                + ((op0_limb_14_col58) * (op1_limb_7_col80)))
                + ((op0_limb_15_col59) * (op1_limb_6_col79)))
                + ((op0_limb_16_col60) * (op1_limb_5_col78)))
                + ((op0_limb_17_col61) * (op1_limb_4_col77)))
                + ((op0_limb_18_col62) * (op1_limb_3_col76)))
                + ((op0_limb_19_col63) * (op1_limb_2_col75)))
                + ((op0_limb_20_col64) * (op1_limb_1_col74)))
                + ((op0_limb_21_col65) * (op1_limb_0_col73)));
            let conv_tmp_5a14_39 = (((((((((((((((((((((((((M31_0)
                - (dst_limb_22_col37))
                + ((op0_limb_0_col44) * (op1_limb_22_col95)))
                + ((op0_limb_1_col45) * (op1_limb_21_col94)))
                + ((op0_limb_2_col46) * (op1_limb_20_col93)))
                + ((op0_limb_3_col47) * (op1_limb_19_col92)))
                + ((op0_limb_4_col48) * (op1_limb_18_col91)))
                + ((op0_limb_5_col49) * (op1_limb_17_col90)))
                + ((op0_limb_6_col50) * (op1_limb_16_col89)))
                + ((op0_limb_7_col51) * (op1_limb_15_col88)))
                + ((op0_limb_8_col52) * (op1_limb_14_col87)))
                + ((op0_limb_9_col53) * (op1_limb_13_col86)))
                + ((op0_limb_10_col54) * (op1_limb_12_col85)))
                + ((op0_limb_11_col55) * (op1_limb_11_col84)))
                + ((op0_limb_12_col56) * (op1_limb_10_col83)))
                + ((op0_limb_13_col57) * (op1_limb_9_col82)))
                + ((op0_limb_14_col58) * (op1_limb_8_col81)))
                + ((op0_limb_15_col59) * (op1_limb_7_col80)))
                + ((op0_limb_16_col60) * (op1_limb_6_col79)))
                + ((op0_limb_17_col61) * (op1_limb_5_col78)))
                + ((op0_limb_18_col62) * (op1_limb_4_col77)))
                + ((op0_limb_19_col63) * (op1_limb_3_col76)))
                + ((op0_limb_20_col64) * (op1_limb_2_col75)))
                + ((op0_limb_21_col65) * (op1_limb_1_col74)))
                + ((op0_limb_22_col66) * (op1_limb_0_col73)));
            let conv_tmp_5a14_40 = ((((((((((((((((((((((((((M31_0)
                - (dst_limb_23_col38))
                + ((op0_limb_0_col44) * (op1_limb_23_col96)))
                + ((op0_limb_1_col45) * (op1_limb_22_col95)))
                + ((op0_limb_2_col46) * (op1_limb_21_col94)))
                + ((op0_limb_3_col47) * (op1_limb_20_col93)))
                + ((op0_limb_4_col48) * (op1_limb_19_col92)))
                + ((op0_limb_5_col49) * (op1_limb_18_col91)))
                + ((op0_limb_6_col50) * (op1_limb_17_col90)))
                + ((op0_limb_7_col51) * (op1_limb_16_col89)))
                + ((op0_limb_8_col52) * (op1_limb_15_col88)))
                + ((op0_limb_9_col53) * (op1_limb_14_col87)))
                + ((op0_limb_10_col54) * (op1_limb_13_col86)))
                + ((op0_limb_11_col55) * (op1_limb_12_col85)))
                + ((op0_limb_12_col56) * (op1_limb_11_col84)))
                + ((op0_limb_13_col57) * (op1_limb_10_col83)))
                + ((op0_limb_14_col58) * (op1_limb_9_col82)))
                + ((op0_limb_15_col59) * (op1_limb_8_col81)))
                + ((op0_limb_16_col60) * (op1_limb_7_col80)))
                + ((op0_limb_17_col61) * (op1_limb_6_col79)))
                + ((op0_limb_18_col62) * (op1_limb_5_col78)))
                + ((op0_limb_19_col63) * (op1_limb_4_col77)))
                + ((op0_limb_20_col64) * (op1_limb_3_col76)))
                + ((op0_limb_21_col65) * (op1_limb_2_col75)))
                + ((op0_limb_22_col66) * (op1_limb_1_col74)))
                + ((op0_limb_23_col67) * (op1_limb_0_col73)));
            let conv_tmp_5a14_41 = (((((((((((((((((((((((((((M31_0)
                - (dst_limb_24_col39))
                + ((op0_limb_0_col44) * (op1_limb_24_col97)))
                + ((op0_limb_1_col45) * (op1_limb_23_col96)))
                + ((op0_limb_2_col46) * (op1_limb_22_col95)))
                + ((op0_limb_3_col47) * (op1_limb_21_col94)))
                + ((op0_limb_4_col48) * (op1_limb_20_col93)))
                + ((op0_limb_5_col49) * (op1_limb_19_col92)))
                + ((op0_limb_6_col50) * (op1_limb_18_col91)))
                + ((op0_limb_7_col51) * (op1_limb_17_col90)))
                + ((op0_limb_8_col52) * (op1_limb_16_col89)))
                + ((op0_limb_9_col53) * (op1_limb_15_col88)))
                + ((op0_limb_10_col54) * (op1_limb_14_col87)))
                + ((op0_limb_11_col55) * (op1_limb_13_col86)))
                + ((op0_limb_12_col56) * (op1_limb_12_col85)))
                + ((op0_limb_13_col57) * (op1_limb_11_col84)))
                + ((op0_limb_14_col58) * (op1_limb_10_col83)))
                + ((op0_limb_15_col59) * (op1_limb_9_col82)))
                + ((op0_limb_16_col60) * (op1_limb_8_col81)))
                + ((op0_limb_17_col61) * (op1_limb_7_col80)))
                + ((op0_limb_18_col62) * (op1_limb_6_col79)))
                + ((op0_limb_19_col63) * (op1_limb_5_col78)))
                + ((op0_limb_20_col64) * (op1_limb_4_col77)))
                + ((op0_limb_21_col65) * (op1_limb_3_col76)))
                + ((op0_limb_22_col66) * (op1_limb_2_col75)))
                + ((op0_limb_23_col67) * (op1_limb_1_col74)))
                + ((op0_limb_24_col68) * (op1_limb_0_col73)));
            let conv_tmp_5a14_42 = ((((((((((((((((((((((((((((M31_0)
                - (dst_limb_25_col40))
                + ((op0_limb_0_col44) * (op1_limb_25_col98)))
                + ((op0_limb_1_col45) * (op1_limb_24_col97)))
                + ((op0_limb_2_col46) * (op1_limb_23_col96)))
                + ((op0_limb_3_col47) * (op1_limb_22_col95)))
                + ((op0_limb_4_col48) * (op1_limb_21_col94)))
                + ((op0_limb_5_col49) * (op1_limb_20_col93)))
                + ((op0_limb_6_col50) * (op1_limb_19_col92)))
                + ((op0_limb_7_col51) * (op1_limb_18_col91)))
                + ((op0_limb_8_col52) * (op1_limb_17_col90)))
                + ((op0_limb_9_col53) * (op1_limb_16_col89)))
                + ((op0_limb_10_col54) * (op1_limb_15_col88)))
                + ((op0_limb_11_col55) * (op1_limb_14_col87)))
                + ((op0_limb_12_col56) * (op1_limb_13_col86)))
                + ((op0_limb_13_col57) * (op1_limb_12_col85)))
                + ((op0_limb_14_col58) * (op1_limb_11_col84)))
                + ((op0_limb_15_col59) * (op1_limb_10_col83)))
                + ((op0_limb_16_col60) * (op1_limb_9_col82)))
                + ((op0_limb_17_col61) * (op1_limb_8_col81)))
                + ((op0_limb_18_col62) * (op1_limb_7_col80)))
                + ((op0_limb_19_col63) * (op1_limb_6_col79)))
                + ((op0_limb_20_col64) * (op1_limb_5_col78)))
                + ((op0_limb_21_col65) * (op1_limb_4_col77)))
                + ((op0_limb_22_col66) * (op1_limb_3_col76)))
                + ((op0_limb_23_col67) * (op1_limb_2_col75)))
                + ((op0_limb_24_col68) * (op1_limb_1_col74)))
                + ((op0_limb_25_col69) * (op1_limb_0_col73)));
            let conv_tmp_5a14_43 = (((((((((((((((((((((((((((((M31_0)
                - (dst_limb_26_col41))
                + ((op0_limb_0_col44) * (op1_limb_26_col99)))
                + ((op0_limb_1_col45) * (op1_limb_25_col98)))
                + ((op0_limb_2_col46) * (op1_limb_24_col97)))
                + ((op0_limb_3_col47) * (op1_limb_23_col96)))
                + ((op0_limb_4_col48) * (op1_limb_22_col95)))
                + ((op0_limb_5_col49) * (op1_limb_21_col94)))
                + ((op0_limb_6_col50) * (op1_limb_20_col93)))
                + ((op0_limb_7_col51) * (op1_limb_19_col92)))
                + ((op0_limb_8_col52) * (op1_limb_18_col91)))
                + ((op0_limb_9_col53) * (op1_limb_17_col90)))
                + ((op0_limb_10_col54) * (op1_limb_16_col89)))
                + ((op0_limb_11_col55) * (op1_limb_15_col88)))
                + ((op0_limb_12_col56) * (op1_limb_14_col87)))
                + ((op0_limb_13_col57) * (op1_limb_13_col86)))
                + ((op0_limb_14_col58) * (op1_limb_12_col85)))
                + ((op0_limb_15_col59) * (op1_limb_11_col84)))
                + ((op0_limb_16_col60) * (op1_limb_10_col83)))
                + ((op0_limb_17_col61) * (op1_limb_9_col82)))
                + ((op0_limb_18_col62) * (op1_limb_8_col81)))
                + ((op0_limb_19_col63) * (op1_limb_7_col80)))
                + ((op0_limb_20_col64) * (op1_limb_6_col79)))
                + ((op0_limb_21_col65) * (op1_limb_5_col78)))
                + ((op0_limb_22_col66) * (op1_limb_4_col77)))
                + ((op0_limb_23_col67) * (op1_limb_3_col76)))
                + ((op0_limb_24_col68) * (op1_limb_2_col75)))
                + ((op0_limb_25_col69) * (op1_limb_1_col74)))
                + ((op0_limb_26_col70) * (op1_limb_0_col73)));
            let conv_tmp_5a14_44 = ((((((((((((((((((((((((((((((M31_0)
                - (dst_limb_27_col42))
                + ((op0_limb_0_col44) * (op1_limb_27_col100)))
                + ((op0_limb_1_col45) * (op1_limb_26_col99)))
                + ((op0_limb_2_col46) * (op1_limb_25_col98)))
                + ((op0_limb_3_col47) * (op1_limb_24_col97)))
                + ((op0_limb_4_col48) * (op1_limb_23_col96)))
                + ((op0_limb_5_col49) * (op1_limb_22_col95)))
                + ((op0_limb_6_col50) * (op1_limb_21_col94)))
                + ((op0_limb_7_col51) * (op1_limb_20_col93)))
                + ((op0_limb_8_col52) * (op1_limb_19_col92)))
                + ((op0_limb_9_col53) * (op1_limb_18_col91)))
                + ((op0_limb_10_col54) * (op1_limb_17_col90)))
                + ((op0_limb_11_col55) * (op1_limb_16_col89)))
                + ((op0_limb_12_col56) * (op1_limb_15_col88)))
                + ((op0_limb_13_col57) * (op1_limb_14_col87)))
                + ((op0_limb_14_col58) * (op1_limb_13_col86)))
                + ((op0_limb_15_col59) * (op1_limb_12_col85)))
                + ((op0_limb_16_col60) * (op1_limb_11_col84)))
                + ((op0_limb_17_col61) * (op1_limb_10_col83)))
                + ((op0_limb_18_col62) * (op1_limb_9_col82)))
                + ((op0_limb_19_col63) * (op1_limb_8_col81)))
                + ((op0_limb_20_col64) * (op1_limb_7_col80)))
                + ((op0_limb_21_col65) * (op1_limb_6_col79)))
                + ((op0_limb_22_col66) * (op1_limb_5_col78)))
                + ((op0_limb_23_col67) * (op1_limb_4_col77)))
                + ((op0_limb_24_col68) * (op1_limb_3_col76)))
                + ((op0_limb_25_col69) * (op1_limb_2_col75)))
                + ((op0_limb_26_col70) * (op1_limb_1_col74)))
                + ((op0_limb_27_col71) * (op1_limb_0_col73)));
            let conv_tmp_5a14_45 = ((((((((((((((((((((((((((((M31_0)
                + ((op0_limb_1_col45) * (op1_limb_27_col100)))
                + ((op0_limb_2_col46) * (op1_limb_26_col99)))
                + ((op0_limb_3_col47) * (op1_limb_25_col98)))
                + ((op0_limb_4_col48) * (op1_limb_24_col97)))
                + ((op0_limb_5_col49) * (op1_limb_23_col96)))
                + ((op0_limb_6_col50) * (op1_limb_22_col95)))
                + ((op0_limb_7_col51) * (op1_limb_21_col94)))
                + ((op0_limb_8_col52) * (op1_limb_20_col93)))
                + ((op0_limb_9_col53) * (op1_limb_19_col92)))
                + ((op0_limb_10_col54) * (op1_limb_18_col91)))
                + ((op0_limb_11_col55) * (op1_limb_17_col90)))
                + ((op0_limb_12_col56) * (op1_limb_16_col89)))
                + ((op0_limb_13_col57) * (op1_limb_15_col88)))
                + ((op0_limb_14_col58) * (op1_limb_14_col87)))
                + ((op0_limb_15_col59) * (op1_limb_13_col86)))
                + ((op0_limb_16_col60) * (op1_limb_12_col85)))
                + ((op0_limb_17_col61) * (op1_limb_11_col84)))
                + ((op0_limb_18_col62) * (op1_limb_10_col83)))
                + ((op0_limb_19_col63) * (op1_limb_9_col82)))
                + ((op0_limb_20_col64) * (op1_limb_8_col81)))
                + ((op0_limb_21_col65) * (op1_limb_7_col80)))
                + ((op0_limb_22_col66) * (op1_limb_6_col79)))
                + ((op0_limb_23_col67) * (op1_limb_5_col78)))
                + ((op0_limb_24_col68) * (op1_limb_4_col77)))
                + ((op0_limb_25_col69) * (op1_limb_3_col76)))
                + ((op0_limb_26_col70) * (op1_limb_2_col75)))
                + ((op0_limb_27_col71) * (op1_limb_1_col74)));
            let conv_tmp_5a14_46 = (((((((((((((((((((((((((((M31_0)
                + ((op0_limb_2_col46) * (op1_limb_27_col100)))
                + ((op0_limb_3_col47) * (op1_limb_26_col99)))
                + ((op0_limb_4_col48) * (op1_limb_25_col98)))
                + ((op0_limb_5_col49) * (op1_limb_24_col97)))
                + ((op0_limb_6_col50) * (op1_limb_23_col96)))
                + ((op0_limb_7_col51) * (op1_limb_22_col95)))
                + ((op0_limb_8_col52) * (op1_limb_21_col94)))
                + ((op0_limb_9_col53) * (op1_limb_20_col93)))
                + ((op0_limb_10_col54) * (op1_limb_19_col92)))
                + ((op0_limb_11_col55) * (op1_limb_18_col91)))
                + ((op0_limb_12_col56) * (op1_limb_17_col90)))
                + ((op0_limb_13_col57) * (op1_limb_16_col89)))
                + ((op0_limb_14_col58) * (op1_limb_15_col88)))
                + ((op0_limb_15_col59) * (op1_limb_14_col87)))
                + ((op0_limb_16_col60) * (op1_limb_13_col86)))
                + ((op0_limb_17_col61) * (op1_limb_12_col85)))
                + ((op0_limb_18_col62) * (op1_limb_11_col84)))
                + ((op0_limb_19_col63) * (op1_limb_10_col83)))
                + ((op0_limb_20_col64) * (op1_limb_9_col82)))
                + ((op0_limb_21_col65) * (op1_limb_8_col81)))
                + ((op0_limb_22_col66) * (op1_limb_7_col80)))
                + ((op0_limb_23_col67) * (op1_limb_6_col79)))
                + ((op0_limb_24_col68) * (op1_limb_5_col78)))
                + ((op0_limb_25_col69) * (op1_limb_4_col77)))
                + ((op0_limb_26_col70) * (op1_limb_3_col76)))
                + ((op0_limb_27_col71) * (op1_limb_2_col75)));
            let conv_tmp_5a14_47 = ((((((((((((((((((((((((((M31_0)
                + ((op0_limb_3_col47) * (op1_limb_27_col100)))
                + ((op0_limb_4_col48) * (op1_limb_26_col99)))
                + ((op0_limb_5_col49) * (op1_limb_25_col98)))
                + ((op0_limb_6_col50) * (op1_limb_24_col97)))
                + ((op0_limb_7_col51) * (op1_limb_23_col96)))
                + ((op0_limb_8_col52) * (op1_limb_22_col95)))
                + ((op0_limb_9_col53) * (op1_limb_21_col94)))
                + ((op0_limb_10_col54) * (op1_limb_20_col93)))
                + ((op0_limb_11_col55) * (op1_limb_19_col92)))
                + ((op0_limb_12_col56) * (op1_limb_18_col91)))
                + ((op0_limb_13_col57) * (op1_limb_17_col90)))
                + ((op0_limb_14_col58) * (op1_limb_16_col89)))
                + ((op0_limb_15_col59) * (op1_limb_15_col88)))
                + ((op0_limb_16_col60) * (op1_limb_14_col87)))
                + ((op0_limb_17_col61) * (op1_limb_13_col86)))
                + ((op0_limb_18_col62) * (op1_limb_12_col85)))
                + ((op0_limb_19_col63) * (op1_limb_11_col84)))
                + ((op0_limb_20_col64) * (op1_limb_10_col83)))
                + ((op0_limb_21_col65) * (op1_limb_9_col82)))
                + ((op0_limb_22_col66) * (op1_limb_8_col81)))
                + ((op0_limb_23_col67) * (op1_limb_7_col80)))
                + ((op0_limb_24_col68) * (op1_limb_6_col79)))
                + ((op0_limb_25_col69) * (op1_limb_5_col78)))
                + ((op0_limb_26_col70) * (op1_limb_4_col77)))
                + ((op0_limb_27_col71) * (op1_limb_3_col76)));
            let conv_tmp_5a14_48 = (((((((((((((((((((((((((M31_0)
                + ((op0_limb_4_col48) * (op1_limb_27_col100)))
                + ((op0_limb_5_col49) * (op1_limb_26_col99)))
                + ((op0_limb_6_col50) * (op1_limb_25_col98)))
                + ((op0_limb_7_col51) * (op1_limb_24_col97)))
                + ((op0_limb_8_col52) * (op1_limb_23_col96)))
                + ((op0_limb_9_col53) * (op1_limb_22_col95)))
                + ((op0_limb_10_col54) * (op1_limb_21_col94)))
                + ((op0_limb_11_col55) * (op1_limb_20_col93)))
                + ((op0_limb_12_col56) * (op1_limb_19_col92)))
                + ((op0_limb_13_col57) * (op1_limb_18_col91)))
                + ((op0_limb_14_col58) * (op1_limb_17_col90)))
                + ((op0_limb_15_col59) * (op1_limb_16_col89)))
                + ((op0_limb_16_col60) * (op1_limb_15_col88)))
                + ((op0_limb_17_col61) * (op1_limb_14_col87)))
                + ((op0_limb_18_col62) * (op1_limb_13_col86)))
                + ((op0_limb_19_col63) * (op1_limb_12_col85)))
                + ((op0_limb_20_col64) * (op1_limb_11_col84)))
                + ((op0_limb_21_col65) * (op1_limb_10_col83)))
                + ((op0_limb_22_col66) * (op1_limb_9_col82)))
                + ((op0_limb_23_col67) * (op1_limb_8_col81)))
                + ((op0_limb_24_col68) * (op1_limb_7_col80)))
                + ((op0_limb_25_col69) * (op1_limb_6_col79)))
                + ((op0_limb_26_col70) * (op1_limb_5_col78)))
                + ((op0_limb_27_col71) * (op1_limb_4_col77)));
            let conv_tmp_5a14_49 = ((((((((((((((((((((((((M31_0)
                + ((op0_limb_5_col49) * (op1_limb_27_col100)))
                + ((op0_limb_6_col50) * (op1_limb_26_col99)))
                + ((op0_limb_7_col51) * (op1_limb_25_col98)))
                + ((op0_limb_8_col52) * (op1_limb_24_col97)))
                + ((op0_limb_9_col53) * (op1_limb_23_col96)))
                + ((op0_limb_10_col54) * (op1_limb_22_col95)))
                + ((op0_limb_11_col55) * (op1_limb_21_col94)))
                + ((op0_limb_12_col56) * (op1_limb_20_col93)))
                + ((op0_limb_13_col57) * (op1_limb_19_col92)))
                + ((op0_limb_14_col58) * (op1_limb_18_col91)))
                + ((op0_limb_15_col59) * (op1_limb_17_col90)))
                + ((op0_limb_16_col60) * (op1_limb_16_col89)))
                + ((op0_limb_17_col61) * (op1_limb_15_col88)))
                + ((op0_limb_18_col62) * (op1_limb_14_col87)))
                + ((op0_limb_19_col63) * (op1_limb_13_col86)))
                + ((op0_limb_20_col64) * (op1_limb_12_col85)))
                + ((op0_limb_21_col65) * (op1_limb_11_col84)))
                + ((op0_limb_22_col66) * (op1_limb_10_col83)))
                + ((op0_limb_23_col67) * (op1_limb_9_col82)))
                + ((op0_limb_24_col68) * (op1_limb_8_col81)))
                + ((op0_limb_25_col69) * (op1_limb_7_col80)))
                + ((op0_limb_26_col70) * (op1_limb_6_col79)))
                + ((op0_limb_27_col71) * (op1_limb_5_col78)));
            let conv_tmp_5a14_50 = (((((((((((((((((((((((M31_0)
                + ((op0_limb_6_col50) * (op1_limb_27_col100)))
                + ((op0_limb_7_col51) * (op1_limb_26_col99)))
                + ((op0_limb_8_col52) * (op1_limb_25_col98)))
                + ((op0_limb_9_col53) * (op1_limb_24_col97)))
                + ((op0_limb_10_col54) * (op1_limb_23_col96)))
                + ((op0_limb_11_col55) * (op1_limb_22_col95)))
                + ((op0_limb_12_col56) * (op1_limb_21_col94)))
                + ((op0_limb_13_col57) * (op1_limb_20_col93)))
                + ((op0_limb_14_col58) * (op1_limb_19_col92)))
                + ((op0_limb_15_col59) * (op1_limb_18_col91)))
                + ((op0_limb_16_col60) * (op1_limb_17_col90)))
                + ((op0_limb_17_col61) * (op1_limb_16_col89)))
                + ((op0_limb_18_col62) * (op1_limb_15_col88)))
                + ((op0_limb_19_col63) * (op1_limb_14_col87)))
                + ((op0_limb_20_col64) * (op1_limb_13_col86)))
                + ((op0_limb_21_col65) * (op1_limb_12_col85)))
                + ((op0_limb_22_col66) * (op1_limb_11_col84)))
                + ((op0_limb_23_col67) * (op1_limb_10_col83)))
                + ((op0_limb_24_col68) * (op1_limb_9_col82)))
                + ((op0_limb_25_col69) * (op1_limb_8_col81)))
                + ((op0_limb_26_col70) * (op1_limb_7_col80)))
                + ((op0_limb_27_col71) * (op1_limb_6_col79)));
            let conv_tmp_5a14_51 = ((((((((((((((((((((((M31_0)
                + ((op0_limb_7_col51) * (op1_limb_27_col100)))
                + ((op0_limb_8_col52) * (op1_limb_26_col99)))
                + ((op0_limb_9_col53) * (op1_limb_25_col98)))
                + ((op0_limb_10_col54) * (op1_limb_24_col97)))
                + ((op0_limb_11_col55) * (op1_limb_23_col96)))
                + ((op0_limb_12_col56) * (op1_limb_22_col95)))
                + ((op0_limb_13_col57) * (op1_limb_21_col94)))
                + ((op0_limb_14_col58) * (op1_limb_20_col93)))
                + ((op0_limb_15_col59) * (op1_limb_19_col92)))
                + ((op0_limb_16_col60) * (op1_limb_18_col91)))
                + ((op0_limb_17_col61) * (op1_limb_17_col90)))
                + ((op0_limb_18_col62) * (op1_limb_16_col89)))
                + ((op0_limb_19_col63) * (op1_limb_15_col88)))
                + ((op0_limb_20_col64) * (op1_limb_14_col87)))
                + ((op0_limb_21_col65) * (op1_limb_13_col86)))
                + ((op0_limb_22_col66) * (op1_limb_12_col85)))
                + ((op0_limb_23_col67) * (op1_limb_11_col84)))
                + ((op0_limb_24_col68) * (op1_limb_10_col83)))
                + ((op0_limb_25_col69) * (op1_limb_9_col82)))
                + ((op0_limb_26_col70) * (op1_limb_8_col81)))
                + ((op0_limb_27_col71) * (op1_limb_7_col80)));
            let conv_tmp_5a14_52 = (((((((((((((((((((((M31_0)
                + ((op0_limb_8_col52) * (op1_limb_27_col100)))
                + ((op0_limb_9_col53) * (op1_limb_26_col99)))
                + ((op0_limb_10_col54) * (op1_limb_25_col98)))
                + ((op0_limb_11_col55) * (op1_limb_24_col97)))
                + ((op0_limb_12_col56) * (op1_limb_23_col96)))
                + ((op0_limb_13_col57) * (op1_limb_22_col95)))
                + ((op0_limb_14_col58) * (op1_limb_21_col94)))
                + ((op0_limb_15_col59) * (op1_limb_20_col93)))
                + ((op0_limb_16_col60) * (op1_limb_19_col92)))
                + ((op0_limb_17_col61) * (op1_limb_18_col91)))
                + ((op0_limb_18_col62) * (op1_limb_17_col90)))
                + ((op0_limb_19_col63) * (op1_limb_16_col89)))
                + ((op0_limb_20_col64) * (op1_limb_15_col88)))
                + ((op0_limb_21_col65) * (op1_limb_14_col87)))
                + ((op0_limb_22_col66) * (op1_limb_13_col86)))
                + ((op0_limb_23_col67) * (op1_limb_12_col85)))
                + ((op0_limb_24_col68) * (op1_limb_11_col84)))
                + ((op0_limb_25_col69) * (op1_limb_10_col83)))
                + ((op0_limb_26_col70) * (op1_limb_9_col82)))
                + ((op0_limb_27_col71) * (op1_limb_8_col81)));
            let conv_tmp_5a14_53 = ((((((((((((((((((((M31_0)
                + ((op0_limb_9_col53) * (op1_limb_27_col100)))
                + ((op0_limb_10_col54) * (op1_limb_26_col99)))
                + ((op0_limb_11_col55) * (op1_limb_25_col98)))
                + ((op0_limb_12_col56) * (op1_limb_24_col97)))
                + ((op0_limb_13_col57) * (op1_limb_23_col96)))
                + ((op0_limb_14_col58) * (op1_limb_22_col95)))
                + ((op0_limb_15_col59) * (op1_limb_21_col94)))
                + ((op0_limb_16_col60) * (op1_limb_20_col93)))
                + ((op0_limb_17_col61) * (op1_limb_19_col92)))
                + ((op0_limb_18_col62) * (op1_limb_18_col91)))
                + ((op0_limb_19_col63) * (op1_limb_17_col90)))
                + ((op0_limb_20_col64) * (op1_limb_16_col89)))
                + ((op0_limb_21_col65) * (op1_limb_15_col88)))
                + ((op0_limb_22_col66) * (op1_limb_14_col87)))
                + ((op0_limb_23_col67) * (op1_limb_13_col86)))
                + ((op0_limb_24_col68) * (op1_limb_12_col85)))
                + ((op0_limb_25_col69) * (op1_limb_11_col84)))
                + ((op0_limb_26_col70) * (op1_limb_10_col83)))
                + ((op0_limb_27_col71) * (op1_limb_9_col82)));
            let conv_tmp_5a14_54 = (((((((((((((((((((M31_0)
                + ((op0_limb_10_col54) * (op1_limb_27_col100)))
                + ((op0_limb_11_col55) * (op1_limb_26_col99)))
                + ((op0_limb_12_col56) * (op1_limb_25_col98)))
                + ((op0_limb_13_col57) * (op1_limb_24_col97)))
                + ((op0_limb_14_col58) * (op1_limb_23_col96)))
                + ((op0_limb_15_col59) * (op1_limb_22_col95)))
                + ((op0_limb_16_col60) * (op1_limb_21_col94)))
                + ((op0_limb_17_col61) * (op1_limb_20_col93)))
                + ((op0_limb_18_col62) * (op1_limb_19_col92)))
                + ((op0_limb_19_col63) * (op1_limb_18_col91)))
                + ((op0_limb_20_col64) * (op1_limb_17_col90)))
                + ((op0_limb_21_col65) * (op1_limb_16_col89)))
                + ((op0_limb_22_col66) * (op1_limb_15_col88)))
                + ((op0_limb_23_col67) * (op1_limb_14_col87)))
                + ((op0_limb_24_col68) * (op1_limb_13_col86)))
                + ((op0_limb_25_col69) * (op1_limb_12_col85)))
                + ((op0_limb_26_col70) * (op1_limb_11_col84)))
                + ((op0_limb_27_col71) * (op1_limb_10_col83)));
            let conv_tmp_5a14_55 = ((((((((((((((((((M31_0)
                + ((op0_limb_11_col55) * (op1_limb_27_col100)))
                + ((op0_limb_12_col56) * (op1_limb_26_col99)))
                + ((op0_limb_13_col57) * (op1_limb_25_col98)))
                + ((op0_limb_14_col58) * (op1_limb_24_col97)))
                + ((op0_limb_15_col59) * (op1_limb_23_col96)))
                + ((op0_limb_16_col60) * (op1_limb_22_col95)))
                + ((op0_limb_17_col61) * (op1_limb_21_col94)))
                + ((op0_limb_18_col62) * (op1_limb_20_col93)))
                + ((op0_limb_19_col63) * (op1_limb_19_col92)))
                + ((op0_limb_20_col64) * (op1_limb_18_col91)))
                + ((op0_limb_21_col65) * (op1_limb_17_col90)))
                + ((op0_limb_22_col66) * (op1_limb_16_col89)))
                + ((op0_limb_23_col67) * (op1_limb_15_col88)))
                + ((op0_limb_24_col68) * (op1_limb_14_col87)))
                + ((op0_limb_25_col69) * (op1_limb_13_col86)))
                + ((op0_limb_26_col70) * (op1_limb_12_col85)))
                + ((op0_limb_27_col71) * (op1_limb_11_col84)));
            let conv_tmp_5a14_56 = (((((((((((((((((M31_0)
                + ((op0_limb_12_col56) * (op1_limb_27_col100)))
                + ((op0_limb_13_col57) * (op1_limb_26_col99)))
                + ((op0_limb_14_col58) * (op1_limb_25_col98)))
                + ((op0_limb_15_col59) * (op1_limb_24_col97)))
                + ((op0_limb_16_col60) * (op1_limb_23_col96)))
                + ((op0_limb_17_col61) * (op1_limb_22_col95)))
                + ((op0_limb_18_col62) * (op1_limb_21_col94)))
                + ((op0_limb_19_col63) * (op1_limb_20_col93)))
                + ((op0_limb_20_col64) * (op1_limb_19_col92)))
                + ((op0_limb_21_col65) * (op1_limb_18_col91)))
                + ((op0_limb_22_col66) * (op1_limb_17_col90)))
                + ((op0_limb_23_col67) * (op1_limb_16_col89)))
                + ((op0_limb_24_col68) * (op1_limb_15_col88)))
                + ((op0_limb_25_col69) * (op1_limb_14_col87)))
                + ((op0_limb_26_col70) * (op1_limb_13_col86)))
                + ((op0_limb_27_col71) * (op1_limb_12_col85)));
            let conv_tmp_5a14_57 = ((((((((((((((((M31_0)
                + ((op0_limb_13_col57) * (op1_limb_27_col100)))
                + ((op0_limb_14_col58) * (op1_limb_26_col99)))
                + ((op0_limb_15_col59) * (op1_limb_25_col98)))
                + ((op0_limb_16_col60) * (op1_limb_24_col97)))
                + ((op0_limb_17_col61) * (op1_limb_23_col96)))
                + ((op0_limb_18_col62) * (op1_limb_22_col95)))
                + ((op0_limb_19_col63) * (op1_limb_21_col94)))
                + ((op0_limb_20_col64) * (op1_limb_20_col93)))
                + ((op0_limb_21_col65) * (op1_limb_19_col92)))
                + ((op0_limb_22_col66) * (op1_limb_18_col91)))
                + ((op0_limb_23_col67) * (op1_limb_17_col90)))
                + ((op0_limb_24_col68) * (op1_limb_16_col89)))
                + ((op0_limb_25_col69) * (op1_limb_15_col88)))
                + ((op0_limb_26_col70) * (op1_limb_14_col87)))
                + ((op0_limb_27_col71) * (op1_limb_13_col86)));
            let conv_tmp_5a14_58 = (((((((((((((((M31_0)
                + ((op0_limb_14_col58) * (op1_limb_27_col100)))
                + ((op0_limb_15_col59) * (op1_limb_26_col99)))
                + ((op0_limb_16_col60) * (op1_limb_25_col98)))
                + ((op0_limb_17_col61) * (op1_limb_24_col97)))
                + ((op0_limb_18_col62) * (op1_limb_23_col96)))
                + ((op0_limb_19_col63) * (op1_limb_22_col95)))
                + ((op0_limb_20_col64) * (op1_limb_21_col94)))
                + ((op0_limb_21_col65) * (op1_limb_20_col93)))
                + ((op0_limb_22_col66) * (op1_limb_19_col92)))
                + ((op0_limb_23_col67) * (op1_limb_18_col91)))
                + ((op0_limb_24_col68) * (op1_limb_17_col90)))
                + ((op0_limb_25_col69) * (op1_limb_16_col89)))
                + ((op0_limb_26_col70) * (op1_limb_15_col88)))
                + ((op0_limb_27_col71) * (op1_limb_14_col87)));
            let conv_tmp_5a14_59 = ((((((((((((((M31_0)
                + ((op0_limb_15_col59) * (op1_limb_27_col100)))
                + ((op0_limb_16_col60) * (op1_limb_26_col99)))
                + ((op0_limb_17_col61) * (op1_limb_25_col98)))
                + ((op0_limb_18_col62) * (op1_limb_24_col97)))
                + ((op0_limb_19_col63) * (op1_limb_23_col96)))
                + ((op0_limb_20_col64) * (op1_limb_22_col95)))
                + ((op0_limb_21_col65) * (op1_limb_21_col94)))
                + ((op0_limb_22_col66) * (op1_limb_20_col93)))
                + ((op0_limb_23_col67) * (op1_limb_19_col92)))
                + ((op0_limb_24_col68) * (op1_limb_18_col91)))
                + ((op0_limb_25_col69) * (op1_limb_17_col90)))
                + ((op0_limb_26_col70) * (op1_limb_16_col89)))
                + ((op0_limb_27_col71) * (op1_limb_15_col88)));
            let conv_tmp_5a14_60 = (((((((((((((M31_0)
                + ((op0_limb_16_col60) * (op1_limb_27_col100)))
                + ((op0_limb_17_col61) * (op1_limb_26_col99)))
                + ((op0_limb_18_col62) * (op1_limb_25_col98)))
                + ((op0_limb_19_col63) * (op1_limb_24_col97)))
                + ((op0_limb_20_col64) * (op1_limb_23_col96)))
                + ((op0_limb_21_col65) * (op1_limb_22_col95)))
                + ((op0_limb_22_col66) * (op1_limb_21_col94)))
                + ((op0_limb_23_col67) * (op1_limb_20_col93)))
                + ((op0_limb_24_col68) * (op1_limb_19_col92)))
                + ((op0_limb_25_col69) * (op1_limb_18_col91)))
                + ((op0_limb_26_col70) * (op1_limb_17_col90)))
                + ((op0_limb_27_col71) * (op1_limb_16_col89)));
            let conv_tmp_5a14_61 = ((((((((((((M31_0)
                + ((op0_limb_17_col61) * (op1_limb_27_col100)))
                + ((op0_limb_18_col62) * (op1_limb_26_col99)))
                + ((op0_limb_19_col63) * (op1_limb_25_col98)))
                + ((op0_limb_20_col64) * (op1_limb_24_col97)))
                + ((op0_limb_21_col65) * (op1_limb_23_col96)))
                + ((op0_limb_22_col66) * (op1_limb_22_col95)))
                + ((op0_limb_23_col67) * (op1_limb_21_col94)))
                + ((op0_limb_24_col68) * (op1_limb_20_col93)))
                + ((op0_limb_25_col69) * (op1_limb_19_col92)))
                + ((op0_limb_26_col70) * (op1_limb_18_col91)))
                + ((op0_limb_27_col71) * (op1_limb_17_col90)));
            let conv_tmp_5a14_62 = (((((((((((M31_0)
                + ((op0_limb_18_col62) * (op1_limb_27_col100)))
                + ((op0_limb_19_col63) * (op1_limb_26_col99)))
                + ((op0_limb_20_col64) * (op1_limb_25_col98)))
                + ((op0_limb_21_col65) * (op1_limb_24_col97)))
                + ((op0_limb_22_col66) * (op1_limb_23_col96)))
                + ((op0_limb_23_col67) * (op1_limb_22_col95)))
                + ((op0_limb_24_col68) * (op1_limb_21_col94)))
                + ((op0_limb_25_col69) * (op1_limb_20_col93)))
                + ((op0_limb_26_col70) * (op1_limb_19_col92)))
                + ((op0_limb_27_col71) * (op1_limb_18_col91)));
            let conv_tmp_5a14_63 = ((((((((((M31_0)
                + ((op0_limb_19_col63) * (op1_limb_27_col100)))
                + ((op0_limb_20_col64) * (op1_limb_26_col99)))
                + ((op0_limb_21_col65) * (op1_limb_25_col98)))
                + ((op0_limb_22_col66) * (op1_limb_24_col97)))
                + ((op0_limb_23_col67) * (op1_limb_23_col96)))
                + ((op0_limb_24_col68) * (op1_limb_22_col95)))
                + ((op0_limb_25_col69) * (op1_limb_21_col94)))
                + ((op0_limb_26_col70) * (op1_limb_20_col93)))
                + ((op0_limb_27_col71) * (op1_limb_19_col92)));
            let conv_tmp_5a14_64 = (((((((((M31_0)
                + ((op0_limb_20_col64) * (op1_limb_27_col100)))
                + ((op0_limb_21_col65) * (op1_limb_26_col99)))
                + ((op0_limb_22_col66) * (op1_limb_25_col98)))
                + ((op0_limb_23_col67) * (op1_limb_24_col97)))
                + ((op0_limb_24_col68) * (op1_limb_23_col96)))
                + ((op0_limb_25_col69) * (op1_limb_22_col95)))
                + ((op0_limb_26_col70) * (op1_limb_21_col94)))
                + ((op0_limb_27_col71) * (op1_limb_20_col93)));
            let conv_tmp_5a14_65 = ((((((((M31_0)
                + ((op0_limb_21_col65) * (op1_limb_27_col100)))
                + ((op0_limb_22_col66) * (op1_limb_26_col99)))
                + ((op0_limb_23_col67) * (op1_limb_25_col98)))
                + ((op0_limb_24_col68) * (op1_limb_24_col97)))
                + ((op0_limb_25_col69) * (op1_limb_23_col96)))
                + ((op0_limb_26_col70) * (op1_limb_22_col95)))
                + ((op0_limb_27_col71) * (op1_limb_21_col94)));
            let conv_tmp_5a14_66 = (((((((M31_0)
                + ((op0_limb_22_col66) * (op1_limb_27_col100)))
                + ((op0_limb_23_col67) * (op1_limb_26_col99)))
                + ((op0_limb_24_col68) * (op1_limb_25_col98)))
                + ((op0_limb_25_col69) * (op1_limb_24_col97)))
                + ((op0_limb_26_col70) * (op1_limb_23_col96)))
                + ((op0_limb_27_col71) * (op1_limb_22_col95)));
            let conv_tmp_5a14_67 = ((((((M31_0) + ((op0_limb_23_col67) * (op1_limb_27_col100)))
                + ((op0_limb_24_col68) * (op1_limb_26_col99)))
                + ((op0_limb_25_col69) * (op1_limb_25_col98)))
                + ((op0_limb_26_col70) * (op1_limb_24_col97)))
                + ((op0_limb_27_col71) * (op1_limb_23_col96)));
            let conv_tmp_5a14_68 = (((((M31_0) + ((op0_limb_24_col68) * (op1_limb_27_col100)))
                + ((op0_limb_25_col69) * (op1_limb_26_col99)))
                + ((op0_limb_26_col70) * (op1_limb_25_col98)))
                + ((op0_limb_27_col71) * (op1_limb_24_col97)));
            let conv_tmp_5a14_69 = ((((M31_0) + ((op0_limb_25_col69) * (op1_limb_27_col100)))
                + ((op0_limb_26_col70) * (op1_limb_26_col99)))
                + ((op0_limb_27_col71) * (op1_limb_25_col98)));
            let conv_tmp_5a14_70 = (((M31_0) + ((op0_limb_26_col70) * (op1_limb_27_col100)))
                + ((op0_limb_27_col71) * (op1_limb_26_col99)));
            let conv_tmp_5a14_71 = ((M31_0) + ((op0_limb_27_col71) * (op1_limb_27_col100)));
            let conv_mod_tmp_5a14_72 = ((((M31_0) + ((M31_32) * (conv_tmp_5a14_17)))
                - ((M31_4) * (conv_tmp_5a14_38)))
                + ((M31_8) * (conv_tmp_5a14_66)));
            let conv_mod_tmp_5a14_73 = (((((M31_0) + ((M31_1) * (conv_tmp_5a14_17)))
                + ((M31_32) * (conv_tmp_5a14_18)))
                - ((M31_4) * (conv_tmp_5a14_39)))
                + ((M31_8) * (conv_tmp_5a14_67)));
            let conv_mod_tmp_5a14_74 = (((((M31_0) + ((M31_1) * (conv_tmp_5a14_18)))
                + ((M31_32) * (conv_tmp_5a14_19)))
                - ((M31_4) * (conv_tmp_5a14_40)))
                + ((M31_8) * (conv_tmp_5a14_68)));
            let conv_mod_tmp_5a14_75 = (((((M31_0) + ((M31_1) * (conv_tmp_5a14_19)))
                + ((M31_32) * (conv_tmp_5a14_20)))
                - ((M31_4) * (conv_tmp_5a14_41)))
                + ((M31_8) * (conv_tmp_5a14_69)));
            let conv_mod_tmp_5a14_76 = (((((M31_0) + ((M31_1) * (conv_tmp_5a14_20)))
                + ((M31_32) * (conv_tmp_5a14_21)))
                - ((M31_4) * (conv_tmp_5a14_42)))
                + ((M31_8) * (conv_tmp_5a14_70)));
            let conv_mod_tmp_5a14_77 = (((((M31_0) + ((M31_1) * (conv_tmp_5a14_21)))
                + ((M31_32) * (conv_tmp_5a14_22)))
                - ((M31_4) * (conv_tmp_5a14_43)))
                + ((M31_8) * (conv_tmp_5a14_71)));
            let conv_mod_tmp_5a14_78 = ((((M31_0) + ((M31_1) * (conv_tmp_5a14_22)))
                + ((M31_32) * (conv_tmp_5a14_23)))
                - ((M31_4) * (conv_tmp_5a14_44)));
            let conv_mod_tmp_5a14_79 = (((((M31_0) + ((M31_2) * (conv_tmp_5a14_17)))
                + ((M31_1) * (conv_tmp_5a14_23)))
                + ((M31_32) * (conv_tmp_5a14_24)))
                - ((M31_4) * (conv_tmp_5a14_45)));
            let conv_mod_tmp_5a14_80 = (((((M31_0) + ((M31_2) * (conv_tmp_5a14_18)))
                + ((M31_1) * (conv_tmp_5a14_24)))
                + ((M31_32) * (conv_tmp_5a14_25)))
                - ((M31_4) * (conv_tmp_5a14_46)));
            let conv_mod_tmp_5a14_81 = (((((M31_0) + ((M31_2) * (conv_tmp_5a14_19)))
                + ((M31_1) * (conv_tmp_5a14_25)))
                + ((M31_32) * (conv_tmp_5a14_26)))
                - ((M31_4) * (conv_tmp_5a14_47)));
            let conv_mod_tmp_5a14_82 = (((((M31_0) + ((M31_2) * (conv_tmp_5a14_20)))
                + ((M31_1) * (conv_tmp_5a14_26)))
                + ((M31_32) * (conv_tmp_5a14_27)))
                - ((M31_4) * (conv_tmp_5a14_48)));
            let conv_mod_tmp_5a14_83 = (((((M31_0) + ((M31_2) * (conv_tmp_5a14_21)))
                + ((M31_1) * (conv_tmp_5a14_27)))
                + ((M31_32) * (conv_tmp_5a14_28)))
                - ((M31_4) * (conv_tmp_5a14_49)));
            let conv_mod_tmp_5a14_84 = (((((M31_0) + ((M31_2) * (conv_tmp_5a14_22)))
                + ((M31_1) * (conv_tmp_5a14_28)))
                + ((M31_32) * (conv_tmp_5a14_29)))
                - ((M31_4) * (conv_tmp_5a14_50)));
            let conv_mod_tmp_5a14_85 = (((((M31_0) + ((M31_2) * (conv_tmp_5a14_23)))
                + ((M31_1) * (conv_tmp_5a14_29)))
                + ((M31_32) * (conv_tmp_5a14_30)))
                - ((M31_4) * (conv_tmp_5a14_51)));
            let conv_mod_tmp_5a14_86 = (((((M31_0) + ((M31_2) * (conv_tmp_5a14_24)))
                + ((M31_1) * (conv_tmp_5a14_30)))
                + ((M31_32) * (conv_tmp_5a14_31)))
                - ((M31_4) * (conv_tmp_5a14_52)));
            let conv_mod_tmp_5a14_87 = (((((M31_0) + ((M31_2) * (conv_tmp_5a14_25)))
                + ((M31_1) * (conv_tmp_5a14_31)))
                + ((M31_32) * (conv_tmp_5a14_32)))
                - ((M31_4) * (conv_tmp_5a14_53)));
            let conv_mod_tmp_5a14_88 = (((((M31_0) + ((M31_2) * (conv_tmp_5a14_26)))
                + ((M31_1) * (conv_tmp_5a14_32)))
                + ((M31_32) * (conv_tmp_5a14_33)))
                - ((M31_4) * (conv_tmp_5a14_54)));
            let conv_mod_tmp_5a14_89 = (((((M31_0) + ((M31_2) * (conv_tmp_5a14_27)))
                + ((M31_1) * (conv_tmp_5a14_33)))
                + ((M31_32) * (conv_tmp_5a14_34)))
                - ((M31_4) * (conv_tmp_5a14_55)));
            let conv_mod_tmp_5a14_90 = (((((M31_0) + ((M31_2) * (conv_tmp_5a14_28)))
                + ((M31_1) * (conv_tmp_5a14_34)))
                + ((M31_32) * (conv_tmp_5a14_35)))
                - ((M31_4) * (conv_tmp_5a14_56)));
            let conv_mod_tmp_5a14_91 = (((((M31_0) + ((M31_2) * (conv_tmp_5a14_29)))
                + ((M31_1) * (conv_tmp_5a14_35)))
                + ((M31_32) * (conv_tmp_5a14_36)))
                - ((M31_4) * (conv_tmp_5a14_57)));
            let conv_mod_tmp_5a14_92 = (((((M31_0) + ((M31_2) * (conv_tmp_5a14_30)))
                + ((M31_1) * (conv_tmp_5a14_36)))
                + ((M31_32) * (conv_tmp_5a14_37)))
                - ((M31_4) * (conv_tmp_5a14_58)));
            let conv_mod_tmp_5a14_93 = (((((M31_0) + ((M31_2) * (conv_tmp_5a14_31)))
                + ((M31_1) * (conv_tmp_5a14_37)))
                - ((M31_4) * (conv_tmp_5a14_59)))
                + ((M31_64) * (conv_tmp_5a14_66)));
            let conv_mod_tmp_5a14_94 = (((((M31_0) + ((M31_2) * (conv_tmp_5a14_32)))
                - ((M31_4) * (conv_tmp_5a14_60)))
                + ((M31_2) * (conv_tmp_5a14_66)))
                + ((M31_64) * (conv_tmp_5a14_67)));
            let conv_mod_tmp_5a14_95 = (((((M31_0) + ((M31_2) * (conv_tmp_5a14_33)))
                - ((M31_4) * (conv_tmp_5a14_61)))
                + ((M31_2) * (conv_tmp_5a14_67)))
                + ((M31_64) * (conv_tmp_5a14_68)));
            let conv_mod_tmp_5a14_96 = (((((M31_0) + ((M31_2) * (conv_tmp_5a14_34)))
                - ((M31_4) * (conv_tmp_5a14_62)))
                + ((M31_2) * (conv_tmp_5a14_68)))
                + ((M31_64) * (conv_tmp_5a14_69)));
            let conv_mod_tmp_5a14_97 = (((((M31_0) + ((M31_2) * (conv_tmp_5a14_35)))
                - ((M31_4) * (conv_tmp_5a14_63)))
                + ((M31_2) * (conv_tmp_5a14_69)))
                + ((M31_64) * (conv_tmp_5a14_70)));
            let conv_mod_tmp_5a14_98 = (((((M31_0) + ((M31_2) * (conv_tmp_5a14_36)))
                - ((M31_4) * (conv_tmp_5a14_64)))
                + ((M31_2) * (conv_tmp_5a14_70)))
                + ((M31_64) * (conv_tmp_5a14_71)));
            let conv_mod_tmp_5a14_99 = ((((M31_0) + ((M31_2) * (conv_tmp_5a14_37)))
                - ((M31_4) * (conv_tmp_5a14_65)))
                + ((M31_2) * (conv_tmp_5a14_71)));
            let k_mod_2_18_biased_tmp_5a14_100 =
                ((((PackedUInt32::from_m31(((conv_mod_tmp_5a14_72) + (M31_134217728))))
                    + (((PackedUInt32::from_m31(((conv_mod_tmp_5a14_73) + (M31_134217728))))
                        & (UInt32_511))
                        << (UInt32_9)))
                    + (UInt32_65536))
                    & (UInt32_262143));
            let k_col101 = ((k_mod_2_18_biased_tmp_5a14_100.low().as_m31())
                + (((k_mod_2_18_biased_tmp_5a14_100.high().as_m31()) - (M31_1)) * (M31_65536)));
            trace[101].data[row_index] = k_col101;

            sub_components_inputs.range_check_19_inputs[0]
                .extend([((k_col101) + (M31_262144))].unpack());

            lookup_data
                .range_check_19_0
                .push([((k_col101) + (M31_262144))]);
            let carry_0_col102 =
                ((((conv_mod_tmp_5a14_72) - ((M31_1) * (k_col101))) + (M31_0)) * (M31_4194304));
            trace[102].data[row_index] = carry_0_col102;

            sub_components_inputs.range_check_19_inputs[1]
                .extend([((carry_0_col102) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_1
                .push([((carry_0_col102) + (M31_131072))]);
            let carry_1_col103 = (((conv_mod_tmp_5a14_73) + (carry_0_col102)) * (M31_4194304));
            trace[103].data[row_index] = carry_1_col103;

            sub_components_inputs.range_check_19_inputs[2]
                .extend([((carry_1_col103) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_2
                .push([((carry_1_col103) + (M31_131072))]);
            let carry_2_col104 = (((conv_mod_tmp_5a14_74) + (carry_1_col103)) * (M31_4194304));
            trace[104].data[row_index] = carry_2_col104;

            sub_components_inputs.range_check_19_inputs[3]
                .extend([((carry_2_col104) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_3
                .push([((carry_2_col104) + (M31_131072))]);
            let carry_3_col105 = (((conv_mod_tmp_5a14_75) + (carry_2_col104)) * (M31_4194304));
            trace[105].data[row_index] = carry_3_col105;

            sub_components_inputs.range_check_19_inputs[4]
                .extend([((carry_3_col105) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_4
                .push([((carry_3_col105) + (M31_131072))]);
            let carry_4_col106 = (((conv_mod_tmp_5a14_76) + (carry_3_col105)) * (M31_4194304));
            trace[106].data[row_index] = carry_4_col106;

            sub_components_inputs.range_check_19_inputs[5]
                .extend([((carry_4_col106) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_5
                .push([((carry_4_col106) + (M31_131072))]);
            let carry_5_col107 = (((conv_mod_tmp_5a14_77) + (carry_4_col106)) * (M31_4194304));
            trace[107].data[row_index] = carry_5_col107;

            sub_components_inputs.range_check_19_inputs[6]
                .extend([((carry_5_col107) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_6
                .push([((carry_5_col107) + (M31_131072))]);
            let carry_6_col108 = (((conv_mod_tmp_5a14_78) + (carry_5_col107)) * (M31_4194304));
            trace[108].data[row_index] = carry_6_col108;

            sub_components_inputs.range_check_19_inputs[7]
                .extend([((carry_6_col108) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_7
                .push([((carry_6_col108) + (M31_131072))]);
            let carry_7_col109 = (((conv_mod_tmp_5a14_79) + (carry_6_col108)) * (M31_4194304));
            trace[109].data[row_index] = carry_7_col109;

            sub_components_inputs.range_check_19_inputs[8]
                .extend([((carry_7_col109) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_8
                .push([((carry_7_col109) + (M31_131072))]);
            let carry_8_col110 = (((conv_mod_tmp_5a14_80) + (carry_7_col109)) * (M31_4194304));
            trace[110].data[row_index] = carry_8_col110;

            sub_components_inputs.range_check_19_inputs[9]
                .extend([((carry_8_col110) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_9
                .push([((carry_8_col110) + (M31_131072))]);
            let carry_9_col111 = (((conv_mod_tmp_5a14_81) + (carry_8_col110)) * (M31_4194304));
            trace[111].data[row_index] = carry_9_col111;

            sub_components_inputs.range_check_19_inputs[10]
                .extend([((carry_9_col111) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_10
                .push([((carry_9_col111) + (M31_131072))]);
            let carry_10_col112 = (((conv_mod_tmp_5a14_82) + (carry_9_col111)) * (M31_4194304));
            trace[112].data[row_index] = carry_10_col112;

            sub_components_inputs.range_check_19_inputs[11]
                .extend([((carry_10_col112) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_11
                .push([((carry_10_col112) + (M31_131072))]);
            let carry_11_col113 = (((conv_mod_tmp_5a14_83) + (carry_10_col112)) * (M31_4194304));
            trace[113].data[row_index] = carry_11_col113;

            sub_components_inputs.range_check_19_inputs[12]
                .extend([((carry_11_col113) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_12
                .push([((carry_11_col113) + (M31_131072))]);
            let carry_12_col114 = (((conv_mod_tmp_5a14_84) + (carry_11_col113)) * (M31_4194304));
            trace[114].data[row_index] = carry_12_col114;

            sub_components_inputs.range_check_19_inputs[13]
                .extend([((carry_12_col114) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_13
                .push([((carry_12_col114) + (M31_131072))]);
            let carry_13_col115 = (((conv_mod_tmp_5a14_85) + (carry_12_col114)) * (M31_4194304));
            trace[115].data[row_index] = carry_13_col115;

            sub_components_inputs.range_check_19_inputs[14]
                .extend([((carry_13_col115) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_14
                .push([((carry_13_col115) + (M31_131072))]);
            let carry_14_col116 = (((conv_mod_tmp_5a14_86) + (carry_13_col115)) * (M31_4194304));
            trace[116].data[row_index] = carry_14_col116;

            sub_components_inputs.range_check_19_inputs[15]
                .extend([((carry_14_col116) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_15
                .push([((carry_14_col116) + (M31_131072))]);
            let carry_15_col117 = (((conv_mod_tmp_5a14_87) + (carry_14_col116)) * (M31_4194304));
            trace[117].data[row_index] = carry_15_col117;

            sub_components_inputs.range_check_19_inputs[16]
                .extend([((carry_15_col117) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_16
                .push([((carry_15_col117) + (M31_131072))]);
            let carry_16_col118 = (((conv_mod_tmp_5a14_88) + (carry_15_col117)) * (M31_4194304));
            trace[118].data[row_index] = carry_16_col118;

            sub_components_inputs.range_check_19_inputs[17]
                .extend([((carry_16_col118) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_17
                .push([((carry_16_col118) + (M31_131072))]);
            let carry_17_col119 = (((conv_mod_tmp_5a14_89) + (carry_16_col118)) * (M31_4194304));
            trace[119].data[row_index] = carry_17_col119;

            sub_components_inputs.range_check_19_inputs[18]
                .extend([((carry_17_col119) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_18
                .push([((carry_17_col119) + (M31_131072))]);
            let carry_18_col120 = (((conv_mod_tmp_5a14_90) + (carry_17_col119)) * (M31_4194304));
            trace[120].data[row_index] = carry_18_col120;

            sub_components_inputs.range_check_19_inputs[19]
                .extend([((carry_18_col120) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_19
                .push([((carry_18_col120) + (M31_131072))]);
            let carry_19_col121 = (((conv_mod_tmp_5a14_91) + (carry_18_col120)) * (M31_4194304));
            trace[121].data[row_index] = carry_19_col121;

            sub_components_inputs.range_check_19_inputs[20]
                .extend([((carry_19_col121) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_20
                .push([((carry_19_col121) + (M31_131072))]);
            let carry_20_col122 = (((conv_mod_tmp_5a14_92) + (carry_19_col121)) * (M31_4194304));
            trace[122].data[row_index] = carry_20_col122;

            sub_components_inputs.range_check_19_inputs[21]
                .extend([((carry_20_col122) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_21
                .push([((carry_20_col122) + (M31_131072))]);
            let carry_21_col123 = ((((conv_mod_tmp_5a14_93) - ((M31_136) * (k_col101)))
                + (carry_20_col122))
                * (M31_4194304));
            trace[123].data[row_index] = carry_21_col123;

            sub_components_inputs.range_check_19_inputs[22]
                .extend([((carry_21_col123) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_22
                .push([((carry_21_col123) + (M31_131072))]);
            let carry_22_col124 = (((conv_mod_tmp_5a14_94) + (carry_21_col123)) * (M31_4194304));
            trace[124].data[row_index] = carry_22_col124;

            sub_components_inputs.range_check_19_inputs[23]
                .extend([((carry_22_col124) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_23
                .push([((carry_22_col124) + (M31_131072))]);
            let carry_23_col125 = (((conv_mod_tmp_5a14_95) + (carry_22_col124)) * (M31_4194304));
            trace[125].data[row_index] = carry_23_col125;

            sub_components_inputs.range_check_19_inputs[24]
                .extend([((carry_23_col125) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_24
                .push([((carry_23_col125) + (M31_131072))]);
            let carry_24_col126 = (((conv_mod_tmp_5a14_96) + (carry_23_col125)) * (M31_4194304));
            trace[126].data[row_index] = carry_24_col126;

            sub_components_inputs.range_check_19_inputs[25]
                .extend([((carry_24_col126) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_25
                .push([((carry_24_col126) + (M31_131072))]);
            let carry_25_col127 = (((conv_mod_tmp_5a14_97) + (carry_24_col126)) * (M31_4194304));
            trace[127].data[row_index] = carry_25_col127;

            sub_components_inputs.range_check_19_inputs[26]
                .extend([((carry_25_col127) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_26
                .push([((carry_25_col127) + (M31_131072))]);
            let carry_26_col128 = (((conv_mod_tmp_5a14_98) + (carry_25_col127)) * (M31_4194304));
            trace[128].data[row_index] = carry_26_col128;

            sub_components_inputs.range_check_19_inputs[27]
                .extend([((carry_26_col128) + (M31_131072))].unpack());

            lookup_data
                .range_check_19_27
                .push([((carry_26_col128) + (M31_131072))]);

            lookup_data
                .opcodes_0
                .push([input_pc_col0, input_ap_col1, input_fp_col2]);
            lookup_data.opcodes_1.push([
                ((input_pc_col0) + (M31_1)),
                ((input_ap_col1) + (ap_update_add_1_col10)),
                input_fp_col2,
            ]);
        });

    (trace, sub_components_inputs, lookup_data)
}

pub struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 2]>,
    memory_address_to_id_1: Vec<[PackedM31; 2]>,
    memory_address_to_id_2: Vec<[PackedM31; 2]>,
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
    memory_id_to_big_1: Vec<[PackedM31; 29]>,
    memory_id_to_big_2: Vec<[PackedM31; 29]>,
    opcodes_0: Vec<[PackedM31; 3]>,
    opcodes_1: Vec<[PackedM31; 3]>,
    range_check_19_0: Vec<[PackedM31; 1]>,
    range_check_19_1: Vec<[PackedM31; 1]>,
    range_check_19_2: Vec<[PackedM31; 1]>,
    range_check_19_3: Vec<[PackedM31; 1]>,
    range_check_19_4: Vec<[PackedM31; 1]>,
    range_check_19_5: Vec<[PackedM31; 1]>,
    range_check_19_6: Vec<[PackedM31; 1]>,
    range_check_19_7: Vec<[PackedM31; 1]>,
    range_check_19_8: Vec<[PackedM31; 1]>,
    range_check_19_9: Vec<[PackedM31; 1]>,
    range_check_19_10: Vec<[PackedM31; 1]>,
    range_check_19_11: Vec<[PackedM31; 1]>,
    range_check_19_12: Vec<[PackedM31; 1]>,
    range_check_19_13: Vec<[PackedM31; 1]>,
    range_check_19_14: Vec<[PackedM31; 1]>,
    range_check_19_15: Vec<[PackedM31; 1]>,
    range_check_19_16: Vec<[PackedM31; 1]>,
    range_check_19_17: Vec<[PackedM31; 1]>,
    range_check_19_18: Vec<[PackedM31; 1]>,
    range_check_19_19: Vec<[PackedM31; 1]>,
    range_check_19_20: Vec<[PackedM31; 1]>,
    range_check_19_21: Vec<[PackedM31; 1]>,
    range_check_19_22: Vec<[PackedM31; 1]>,
    range_check_19_23: Vec<[PackedM31; 1]>,
    range_check_19_24: Vec<[PackedM31; 1]>,
    range_check_19_25: Vec<[PackedM31; 1]>,
    range_check_19_26: Vec<[PackedM31; 1]>,
    range_check_19_27: Vec<[PackedM31; 1]>,
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
            memory_id_to_big_1: Vec::with_capacity(capacity),
            memory_id_to_big_2: Vec::with_capacity(capacity),
            opcodes_0: Vec::with_capacity(capacity),
            opcodes_1: Vec::with_capacity(capacity),
            range_check_19_0: Vec::with_capacity(capacity),
            range_check_19_1: Vec::with_capacity(capacity),
            range_check_19_2: Vec::with_capacity(capacity),
            range_check_19_3: Vec::with_capacity(capacity),
            range_check_19_4: Vec::with_capacity(capacity),
            range_check_19_5: Vec::with_capacity(capacity),
            range_check_19_6: Vec::with_capacity(capacity),
            range_check_19_7: Vec::with_capacity(capacity),
            range_check_19_8: Vec::with_capacity(capacity),
            range_check_19_9: Vec::with_capacity(capacity),
            range_check_19_10: Vec::with_capacity(capacity),
            range_check_19_11: Vec::with_capacity(capacity),
            range_check_19_12: Vec::with_capacity(capacity),
            range_check_19_13: Vec::with_capacity(capacity),
            range_check_19_14: Vec::with_capacity(capacity),
            range_check_19_15: Vec::with_capacity(capacity),
            range_check_19_16: Vec::with_capacity(capacity),
            range_check_19_17: Vec::with_capacity(capacity),
            range_check_19_18: Vec::with_capacity(capacity),
            range_check_19_19: Vec::with_capacity(capacity),
            range_check_19_20: Vec::with_capacity(capacity),
            range_check_19_21: Vec::with_capacity(capacity),
            range_check_19_22: Vec::with_capacity(capacity),
            range_check_19_23: Vec::with_capacity(capacity),
            range_check_19_24: Vec::with_capacity(capacity),
            range_check_19_25: Vec::with_capacity(capacity),
            range_check_19_26: Vec::with_capacity(capacity),
            range_check_19_27: Vec::with_capacity(capacity),
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
        range_check_19: &relations::RangeCheck_19,
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
            &self.lookup_data.memory_id_to_big_1,
            &self.lookup_data.memory_address_to_id_2,
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
            &self.lookup_data.memory_id_to_big_2,
            &self.lookup_data.range_check_19_0,
        )
        .enumerate()
        {
            let p0: PackedQM31 = memory_id_to_big.combine(v0);
            let p1: PackedQM31 = range_check_19.combine(v1);
            col_gen.write_frac(i, p0 + p1, p0 * p1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (v0, v1)) in zip(
            &self.lookup_data.range_check_19_1,
            &self.lookup_data.range_check_19_2,
        )
        .enumerate()
        {
            let p0: PackedQM31 = range_check_19.combine(v0);
            let p1: PackedQM31 = range_check_19.combine(v1);
            col_gen.write_frac(i, p0 + p1, p0 * p1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (v0, v1)) in zip(
            &self.lookup_data.range_check_19_3,
            &self.lookup_data.range_check_19_4,
        )
        .enumerate()
        {
            let p0: PackedQM31 = range_check_19.combine(v0);
            let p1: PackedQM31 = range_check_19.combine(v1);
            col_gen.write_frac(i, p0 + p1, p0 * p1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (v0, v1)) in zip(
            &self.lookup_data.range_check_19_5,
            &self.lookup_data.range_check_19_6,
        )
        .enumerate()
        {
            let p0: PackedQM31 = range_check_19.combine(v0);
            let p1: PackedQM31 = range_check_19.combine(v1);
            col_gen.write_frac(i, p0 + p1, p0 * p1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (v0, v1)) in zip(
            &self.lookup_data.range_check_19_7,
            &self.lookup_data.range_check_19_8,
        )
        .enumerate()
        {
            let p0: PackedQM31 = range_check_19.combine(v0);
            let p1: PackedQM31 = range_check_19.combine(v1);
            col_gen.write_frac(i, p0 + p1, p0 * p1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (v0, v1)) in zip(
            &self.lookup_data.range_check_19_9,
            &self.lookup_data.range_check_19_10,
        )
        .enumerate()
        {
            let p0: PackedQM31 = range_check_19.combine(v0);
            let p1: PackedQM31 = range_check_19.combine(v1);
            col_gen.write_frac(i, p0 + p1, p0 * p1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (v0, v1)) in zip(
            &self.lookup_data.range_check_19_11,
            &self.lookup_data.range_check_19_12,
        )
        .enumerate()
        {
            let p0: PackedQM31 = range_check_19.combine(v0);
            let p1: PackedQM31 = range_check_19.combine(v1);
            col_gen.write_frac(i, p0 + p1, p0 * p1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (v0, v1)) in zip(
            &self.lookup_data.range_check_19_13,
            &self.lookup_data.range_check_19_14,
        )
        .enumerate()
        {
            let p0: PackedQM31 = range_check_19.combine(v0);
            let p1: PackedQM31 = range_check_19.combine(v1);
            col_gen.write_frac(i, p0 + p1, p0 * p1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (v0, v1)) in zip(
            &self.lookup_data.range_check_19_15,
            &self.lookup_data.range_check_19_16,
        )
        .enumerate()
        {
            let p0: PackedQM31 = range_check_19.combine(v0);
            let p1: PackedQM31 = range_check_19.combine(v1);
            col_gen.write_frac(i, p0 + p1, p0 * p1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (v0, v1)) in zip(
            &self.lookup_data.range_check_19_17,
            &self.lookup_data.range_check_19_18,
        )
        .enumerate()
        {
            let p0: PackedQM31 = range_check_19.combine(v0);
            let p1: PackedQM31 = range_check_19.combine(v1);
            col_gen.write_frac(i, p0 + p1, p0 * p1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (v0, v1)) in zip(
            &self.lookup_data.range_check_19_19,
            &self.lookup_data.range_check_19_20,
        )
        .enumerate()
        {
            let p0: PackedQM31 = range_check_19.combine(v0);
            let p1: PackedQM31 = range_check_19.combine(v1);
            col_gen.write_frac(i, p0 + p1, p0 * p1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (v0, v1)) in zip(
            &self.lookup_data.range_check_19_21,
            &self.lookup_data.range_check_19_22,
        )
        .enumerate()
        {
            let p0: PackedQM31 = range_check_19.combine(v0);
            let p1: PackedQM31 = range_check_19.combine(v1);
            col_gen.write_frac(i, p0 + p1, p0 * p1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (v0, v1)) in zip(
            &self.lookup_data.range_check_19_23,
            &self.lookup_data.range_check_19_24,
        )
        .enumerate()
        {
            let p0: PackedQM31 = range_check_19.combine(v0);
            let p1: PackedQM31 = range_check_19.combine(v1);
            col_gen.write_frac(i, p0 + p1, p0 * p1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (v0, v1)) in zip(
            &self.lookup_data.range_check_19_25,
            &self.lookup_data.range_check_19_26,
        )
        .enumerate()
        {
            let p0: PackedQM31 = range_check_19.combine(v0);
            let p1: PackedQM31 = range_check_19.combine(v1);
            col_gen.write_frac(i, p0 + p1, p0 * p1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (v0, v1)) in zip(
            &self.lookup_data.range_check_19_27,
            &self.lookup_data.opcodes_0,
        )
        .enumerate()
        {
            let p0: PackedQM31 = range_check_19.combine(v0);
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
