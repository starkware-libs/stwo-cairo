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
const N_TRACE_COLUMNS: usize = 98;

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
    const N_TRACE_COLUMNS: usize = 98;
    let mut trace: [_; N_TRACE_COLUMNS] =
        std::array::from_fn(|_| Col::<SimdBackend, M31>::zeros(inputs.len() * N_LANES));

    let mut lookup_data = LookupData::with_capacity(inputs.len());
    #[allow(unused_mut)]
    let mut sub_components_inputs = SubComponentInputs::with_capacity(inputs.len());

    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_32769 = PackedM31::broadcast(M31::from(32769));
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
        .for_each(|(row_index, add_opcode_is_small_f_is_imm_t_input)| {
            let input_tmp_f0ae_0 = add_opcode_is_small_f_is_imm_t_input;
            let input_pc_col0 = input_tmp_f0ae_0.pc;
            trace[0].data[row_index] = input_pc_col0;
            let input_ap_col1 = input_tmp_f0ae_0.ap;
            trace[1].data[row_index] = input_ap_col1;
            let input_fp_col2 = input_tmp_f0ae_0.fp;
            trace[2].data[row_index] = input_fp_col2;

            // Decode Instruction.

            let memory_address_to_id_value_tmp_f0ae_1 =
                memory_address_to_id_state.deduce_output(input_pc_col0);
            let memory_id_to_big_value_tmp_f0ae_2 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_f0ae_1);
            let offset0_tmp_f0ae_3 =
                ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_f0ae_2.get_m31(0)))
                    + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_f0ae_2.get_m31(1)))
                        & (UInt16_127))
                        << (UInt16_9)));
            let offset0_col3 = offset0_tmp_f0ae_3.as_m31();
            trace[3].data[row_index] = offset0_col3;
            let offset1_tmp_f0ae_4 =
                ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_f0ae_2.get_m31(1)))
                    >> (UInt16_7))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_f0ae_2.get_m31(2)))
                        << (UInt16_2)))
                    + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_f0ae_2.get_m31(3)))
                        & (UInt16_31))
                        << (UInt16_11)));
            let offset1_col4 = offset1_tmp_f0ae_4.as_m31();
            trace[4].data[row_index] = offset1_col4;
            let dst_base_fp_tmp_f0ae_5 =
                (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_f0ae_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_f0ae_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_0))
                    & (UInt16_1));
            let dst_base_fp_col5 = dst_base_fp_tmp_f0ae_5.as_m31();
            trace[5].data[row_index] = dst_base_fp_col5;
            let op0_base_fp_tmp_f0ae_6 =
                (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_f0ae_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_f0ae_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_1))
                    & (UInt16_1));
            let op0_base_fp_col6 = op0_base_fp_tmp_f0ae_6.as_m31();
            trace[6].data[row_index] = op0_base_fp_col6;
            let ap_update_add_1_tmp_f0ae_7 =
                (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_f0ae_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_f0ae_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_11))
                    & (UInt16_1));
            let ap_update_add_1_col7 = ap_update_add_1_tmp_f0ae_7.as_m31();
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

            lookup_data.verify_instruction_0.push([
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

            let mem_dst_base_col8 = (((dst_base_fp_col5) * (input_fp_col2))
                + (((M31_1) - (dst_base_fp_col5)) * (input_ap_col1)));
            trace[8].data[row_index] = mem_dst_base_col8;
            let mem0_base_col9 = (((op0_base_fp_col6) * (input_fp_col2))
                + (((M31_1) - (op0_base_fp_col6)) * (input_ap_col1)));
            trace[9].data[row_index] = mem0_base_col9;

            // Read Positive Num Bits 252.

            let memory_address_to_id_value_tmp_f0ae_8 = memory_address_to_id_state
                .deduce_output(((mem_dst_base_col8) + ((offset0_col3) - (M31_32768))));
            let memory_id_to_big_value_tmp_f0ae_9 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_f0ae_8);
            let dst_id_col10 = memory_address_to_id_value_tmp_f0ae_8;
            trace[10].data[row_index] = dst_id_col10;
            sub_components_inputs.memory_address_to_id_inputs[0]
                .extend(((mem_dst_base_col8) + ((offset0_col3) - (M31_32768))).unpack());

            lookup_data.memory_address_to_id_0.push([
                ((mem_dst_base_col8) + ((offset0_col3) - (M31_32768))),
                dst_id_col10,
            ]);
            let dst_limb_0_col11 = memory_id_to_big_value_tmp_f0ae_9.get_m31(0);
            trace[11].data[row_index] = dst_limb_0_col11;
            let dst_limb_1_col12 = memory_id_to_big_value_tmp_f0ae_9.get_m31(1);
            trace[12].data[row_index] = dst_limb_1_col12;
            let dst_limb_2_col13 = memory_id_to_big_value_tmp_f0ae_9.get_m31(2);
            trace[13].data[row_index] = dst_limb_2_col13;
            let dst_limb_3_col14 = memory_id_to_big_value_tmp_f0ae_9.get_m31(3);
            trace[14].data[row_index] = dst_limb_3_col14;
            let dst_limb_4_col15 = memory_id_to_big_value_tmp_f0ae_9.get_m31(4);
            trace[15].data[row_index] = dst_limb_4_col15;
            let dst_limb_5_col16 = memory_id_to_big_value_tmp_f0ae_9.get_m31(5);
            trace[16].data[row_index] = dst_limb_5_col16;
            let dst_limb_6_col17 = memory_id_to_big_value_tmp_f0ae_9.get_m31(6);
            trace[17].data[row_index] = dst_limb_6_col17;
            let dst_limb_7_col18 = memory_id_to_big_value_tmp_f0ae_9.get_m31(7);
            trace[18].data[row_index] = dst_limb_7_col18;
            let dst_limb_8_col19 = memory_id_to_big_value_tmp_f0ae_9.get_m31(8);
            trace[19].data[row_index] = dst_limb_8_col19;
            let dst_limb_9_col20 = memory_id_to_big_value_tmp_f0ae_9.get_m31(9);
            trace[20].data[row_index] = dst_limb_9_col20;
            let dst_limb_10_col21 = memory_id_to_big_value_tmp_f0ae_9.get_m31(10);
            trace[21].data[row_index] = dst_limb_10_col21;
            let dst_limb_11_col22 = memory_id_to_big_value_tmp_f0ae_9.get_m31(11);
            trace[22].data[row_index] = dst_limb_11_col22;
            let dst_limb_12_col23 = memory_id_to_big_value_tmp_f0ae_9.get_m31(12);
            trace[23].data[row_index] = dst_limb_12_col23;
            let dst_limb_13_col24 = memory_id_to_big_value_tmp_f0ae_9.get_m31(13);
            trace[24].data[row_index] = dst_limb_13_col24;
            let dst_limb_14_col25 = memory_id_to_big_value_tmp_f0ae_9.get_m31(14);
            trace[25].data[row_index] = dst_limb_14_col25;
            let dst_limb_15_col26 = memory_id_to_big_value_tmp_f0ae_9.get_m31(15);
            trace[26].data[row_index] = dst_limb_15_col26;
            let dst_limb_16_col27 = memory_id_to_big_value_tmp_f0ae_9.get_m31(16);
            trace[27].data[row_index] = dst_limb_16_col27;
            let dst_limb_17_col28 = memory_id_to_big_value_tmp_f0ae_9.get_m31(17);
            trace[28].data[row_index] = dst_limb_17_col28;
            let dst_limb_18_col29 = memory_id_to_big_value_tmp_f0ae_9.get_m31(18);
            trace[29].data[row_index] = dst_limb_18_col29;
            let dst_limb_19_col30 = memory_id_to_big_value_tmp_f0ae_9.get_m31(19);
            trace[30].data[row_index] = dst_limb_19_col30;
            let dst_limb_20_col31 = memory_id_to_big_value_tmp_f0ae_9.get_m31(20);
            trace[31].data[row_index] = dst_limb_20_col31;
            let dst_limb_21_col32 = memory_id_to_big_value_tmp_f0ae_9.get_m31(21);
            trace[32].data[row_index] = dst_limb_21_col32;
            let dst_limb_22_col33 = memory_id_to_big_value_tmp_f0ae_9.get_m31(22);
            trace[33].data[row_index] = dst_limb_22_col33;
            let dst_limb_23_col34 = memory_id_to_big_value_tmp_f0ae_9.get_m31(23);
            trace[34].data[row_index] = dst_limb_23_col34;
            let dst_limb_24_col35 = memory_id_to_big_value_tmp_f0ae_9.get_m31(24);
            trace[35].data[row_index] = dst_limb_24_col35;
            let dst_limb_25_col36 = memory_id_to_big_value_tmp_f0ae_9.get_m31(25);
            trace[36].data[row_index] = dst_limb_25_col36;
            let dst_limb_26_col37 = memory_id_to_big_value_tmp_f0ae_9.get_m31(26);
            trace[37].data[row_index] = dst_limb_26_col37;
            let dst_limb_27_col38 = memory_id_to_big_value_tmp_f0ae_9.get_m31(27);
            trace[38].data[row_index] = dst_limb_27_col38;
            sub_components_inputs.memory_id_to_big_inputs[0].extend(dst_id_col10.unpack());

            lookup_data.memory_id_to_big_0.push([
                dst_id_col10,
                dst_limb_0_col11,
                dst_limb_1_col12,
                dst_limb_2_col13,
                dst_limb_3_col14,
                dst_limb_4_col15,
                dst_limb_5_col16,
                dst_limb_6_col17,
                dst_limb_7_col18,
                dst_limb_8_col19,
                dst_limb_9_col20,
                dst_limb_10_col21,
                dst_limb_11_col22,
                dst_limb_12_col23,
                dst_limb_13_col24,
                dst_limb_14_col25,
                dst_limb_15_col26,
                dst_limb_16_col27,
                dst_limb_17_col28,
                dst_limb_18_col29,
                dst_limb_19_col30,
                dst_limb_20_col31,
                dst_limb_21_col32,
                dst_limb_22_col33,
                dst_limb_23_col34,
                dst_limb_24_col35,
                dst_limb_25_col36,
                dst_limb_26_col37,
                dst_limb_27_col38,
            ]);

            // Read Positive Num Bits 252.

            let memory_address_to_id_value_tmp_f0ae_10 = memory_address_to_id_state
                .deduce_output(((mem0_base_col9) + ((offset1_col4) - (M31_32768))));
            let memory_id_to_big_value_tmp_f0ae_11 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_f0ae_10);
            let op0_id_col39 = memory_address_to_id_value_tmp_f0ae_10;
            trace[39].data[row_index] = op0_id_col39;
            sub_components_inputs.memory_address_to_id_inputs[1]
                .extend(((mem0_base_col9) + ((offset1_col4) - (M31_32768))).unpack());

            lookup_data.memory_address_to_id_1.push([
                ((mem0_base_col9) + ((offset1_col4) - (M31_32768))),
                op0_id_col39,
            ]);
            let op0_limb_0_col40 = memory_id_to_big_value_tmp_f0ae_11.get_m31(0);
            trace[40].data[row_index] = op0_limb_0_col40;
            let op0_limb_1_col41 = memory_id_to_big_value_tmp_f0ae_11.get_m31(1);
            trace[41].data[row_index] = op0_limb_1_col41;
            let op0_limb_2_col42 = memory_id_to_big_value_tmp_f0ae_11.get_m31(2);
            trace[42].data[row_index] = op0_limb_2_col42;
            let op0_limb_3_col43 = memory_id_to_big_value_tmp_f0ae_11.get_m31(3);
            trace[43].data[row_index] = op0_limb_3_col43;
            let op0_limb_4_col44 = memory_id_to_big_value_tmp_f0ae_11.get_m31(4);
            trace[44].data[row_index] = op0_limb_4_col44;
            let op0_limb_5_col45 = memory_id_to_big_value_tmp_f0ae_11.get_m31(5);
            trace[45].data[row_index] = op0_limb_5_col45;
            let op0_limb_6_col46 = memory_id_to_big_value_tmp_f0ae_11.get_m31(6);
            trace[46].data[row_index] = op0_limb_6_col46;
            let op0_limb_7_col47 = memory_id_to_big_value_tmp_f0ae_11.get_m31(7);
            trace[47].data[row_index] = op0_limb_7_col47;
            let op0_limb_8_col48 = memory_id_to_big_value_tmp_f0ae_11.get_m31(8);
            trace[48].data[row_index] = op0_limb_8_col48;
            let op0_limb_9_col49 = memory_id_to_big_value_tmp_f0ae_11.get_m31(9);
            trace[49].data[row_index] = op0_limb_9_col49;
            let op0_limb_10_col50 = memory_id_to_big_value_tmp_f0ae_11.get_m31(10);
            trace[50].data[row_index] = op0_limb_10_col50;
            let op0_limb_11_col51 = memory_id_to_big_value_tmp_f0ae_11.get_m31(11);
            trace[51].data[row_index] = op0_limb_11_col51;
            let op0_limb_12_col52 = memory_id_to_big_value_tmp_f0ae_11.get_m31(12);
            trace[52].data[row_index] = op0_limb_12_col52;
            let op0_limb_13_col53 = memory_id_to_big_value_tmp_f0ae_11.get_m31(13);
            trace[53].data[row_index] = op0_limb_13_col53;
            let op0_limb_14_col54 = memory_id_to_big_value_tmp_f0ae_11.get_m31(14);
            trace[54].data[row_index] = op0_limb_14_col54;
            let op0_limb_15_col55 = memory_id_to_big_value_tmp_f0ae_11.get_m31(15);
            trace[55].data[row_index] = op0_limb_15_col55;
            let op0_limb_16_col56 = memory_id_to_big_value_tmp_f0ae_11.get_m31(16);
            trace[56].data[row_index] = op0_limb_16_col56;
            let op0_limb_17_col57 = memory_id_to_big_value_tmp_f0ae_11.get_m31(17);
            trace[57].data[row_index] = op0_limb_17_col57;
            let op0_limb_18_col58 = memory_id_to_big_value_tmp_f0ae_11.get_m31(18);
            trace[58].data[row_index] = op0_limb_18_col58;
            let op0_limb_19_col59 = memory_id_to_big_value_tmp_f0ae_11.get_m31(19);
            trace[59].data[row_index] = op0_limb_19_col59;
            let op0_limb_20_col60 = memory_id_to_big_value_tmp_f0ae_11.get_m31(20);
            trace[60].data[row_index] = op0_limb_20_col60;
            let op0_limb_21_col61 = memory_id_to_big_value_tmp_f0ae_11.get_m31(21);
            trace[61].data[row_index] = op0_limb_21_col61;
            let op0_limb_22_col62 = memory_id_to_big_value_tmp_f0ae_11.get_m31(22);
            trace[62].data[row_index] = op0_limb_22_col62;
            let op0_limb_23_col63 = memory_id_to_big_value_tmp_f0ae_11.get_m31(23);
            trace[63].data[row_index] = op0_limb_23_col63;
            let op0_limb_24_col64 = memory_id_to_big_value_tmp_f0ae_11.get_m31(24);
            trace[64].data[row_index] = op0_limb_24_col64;
            let op0_limb_25_col65 = memory_id_to_big_value_tmp_f0ae_11.get_m31(25);
            trace[65].data[row_index] = op0_limb_25_col65;
            let op0_limb_26_col66 = memory_id_to_big_value_tmp_f0ae_11.get_m31(26);
            trace[66].data[row_index] = op0_limb_26_col66;
            let op0_limb_27_col67 = memory_id_to_big_value_tmp_f0ae_11.get_m31(27);
            trace[67].data[row_index] = op0_limb_27_col67;
            sub_components_inputs.memory_id_to_big_inputs[1].extend(op0_id_col39.unpack());

            lookup_data.memory_id_to_big_1.push([
                op0_id_col39,
                op0_limb_0_col40,
                op0_limb_1_col41,
                op0_limb_2_col42,
                op0_limb_3_col43,
                op0_limb_4_col44,
                op0_limb_5_col45,
                op0_limb_6_col46,
                op0_limb_7_col47,
                op0_limb_8_col48,
                op0_limb_9_col49,
                op0_limb_10_col50,
                op0_limb_11_col51,
                op0_limb_12_col52,
                op0_limb_13_col53,
                op0_limb_14_col54,
                op0_limb_15_col55,
                op0_limb_16_col56,
                op0_limb_17_col57,
                op0_limb_18_col58,
                op0_limb_19_col59,
                op0_limb_20_col60,
                op0_limb_21_col61,
                op0_limb_22_col62,
                op0_limb_23_col63,
                op0_limb_24_col64,
                op0_limb_25_col65,
                op0_limb_26_col66,
                op0_limb_27_col67,
            ]);

            // Read Positive Num Bits 252.

            let memory_address_to_id_value_tmp_f0ae_12 =
                memory_address_to_id_state.deduce_output(((input_pc_col0) + (M31_1)));
            let memory_id_to_big_value_tmp_f0ae_13 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_f0ae_12);
            let op1_id_col68 = memory_address_to_id_value_tmp_f0ae_12;
            trace[68].data[row_index] = op1_id_col68;
            sub_components_inputs.memory_address_to_id_inputs[2]
                .extend(((input_pc_col0) + (M31_1)).unpack());

            lookup_data
                .memory_address_to_id_2
                .push([((input_pc_col0) + (M31_1)), op1_id_col68]);
            let op1_limb_0_col69 = memory_id_to_big_value_tmp_f0ae_13.get_m31(0);
            trace[69].data[row_index] = op1_limb_0_col69;
            let op1_limb_1_col70 = memory_id_to_big_value_tmp_f0ae_13.get_m31(1);
            trace[70].data[row_index] = op1_limb_1_col70;
            let op1_limb_2_col71 = memory_id_to_big_value_tmp_f0ae_13.get_m31(2);
            trace[71].data[row_index] = op1_limb_2_col71;
            let op1_limb_3_col72 = memory_id_to_big_value_tmp_f0ae_13.get_m31(3);
            trace[72].data[row_index] = op1_limb_3_col72;
            let op1_limb_4_col73 = memory_id_to_big_value_tmp_f0ae_13.get_m31(4);
            trace[73].data[row_index] = op1_limb_4_col73;
            let op1_limb_5_col74 = memory_id_to_big_value_tmp_f0ae_13.get_m31(5);
            trace[74].data[row_index] = op1_limb_5_col74;
            let op1_limb_6_col75 = memory_id_to_big_value_tmp_f0ae_13.get_m31(6);
            trace[75].data[row_index] = op1_limb_6_col75;
            let op1_limb_7_col76 = memory_id_to_big_value_tmp_f0ae_13.get_m31(7);
            trace[76].data[row_index] = op1_limb_7_col76;
            let op1_limb_8_col77 = memory_id_to_big_value_tmp_f0ae_13.get_m31(8);
            trace[77].data[row_index] = op1_limb_8_col77;
            let op1_limb_9_col78 = memory_id_to_big_value_tmp_f0ae_13.get_m31(9);
            trace[78].data[row_index] = op1_limb_9_col78;
            let op1_limb_10_col79 = memory_id_to_big_value_tmp_f0ae_13.get_m31(10);
            trace[79].data[row_index] = op1_limb_10_col79;
            let op1_limb_11_col80 = memory_id_to_big_value_tmp_f0ae_13.get_m31(11);
            trace[80].data[row_index] = op1_limb_11_col80;
            let op1_limb_12_col81 = memory_id_to_big_value_tmp_f0ae_13.get_m31(12);
            trace[81].data[row_index] = op1_limb_12_col81;
            let op1_limb_13_col82 = memory_id_to_big_value_tmp_f0ae_13.get_m31(13);
            trace[82].data[row_index] = op1_limb_13_col82;
            let op1_limb_14_col83 = memory_id_to_big_value_tmp_f0ae_13.get_m31(14);
            trace[83].data[row_index] = op1_limb_14_col83;
            let op1_limb_15_col84 = memory_id_to_big_value_tmp_f0ae_13.get_m31(15);
            trace[84].data[row_index] = op1_limb_15_col84;
            let op1_limb_16_col85 = memory_id_to_big_value_tmp_f0ae_13.get_m31(16);
            trace[85].data[row_index] = op1_limb_16_col85;
            let op1_limb_17_col86 = memory_id_to_big_value_tmp_f0ae_13.get_m31(17);
            trace[86].data[row_index] = op1_limb_17_col86;
            let op1_limb_18_col87 = memory_id_to_big_value_tmp_f0ae_13.get_m31(18);
            trace[87].data[row_index] = op1_limb_18_col87;
            let op1_limb_19_col88 = memory_id_to_big_value_tmp_f0ae_13.get_m31(19);
            trace[88].data[row_index] = op1_limb_19_col88;
            let op1_limb_20_col89 = memory_id_to_big_value_tmp_f0ae_13.get_m31(20);
            trace[89].data[row_index] = op1_limb_20_col89;
            let op1_limb_21_col90 = memory_id_to_big_value_tmp_f0ae_13.get_m31(21);
            trace[90].data[row_index] = op1_limb_21_col90;
            let op1_limb_22_col91 = memory_id_to_big_value_tmp_f0ae_13.get_m31(22);
            trace[91].data[row_index] = op1_limb_22_col91;
            let op1_limb_23_col92 = memory_id_to_big_value_tmp_f0ae_13.get_m31(23);
            trace[92].data[row_index] = op1_limb_23_col92;
            let op1_limb_24_col93 = memory_id_to_big_value_tmp_f0ae_13.get_m31(24);
            trace[93].data[row_index] = op1_limb_24_col93;
            let op1_limb_25_col94 = memory_id_to_big_value_tmp_f0ae_13.get_m31(25);
            trace[94].data[row_index] = op1_limb_25_col94;
            let op1_limb_26_col95 = memory_id_to_big_value_tmp_f0ae_13.get_m31(26);
            trace[95].data[row_index] = op1_limb_26_col95;
            let op1_limb_27_col96 = memory_id_to_big_value_tmp_f0ae_13.get_m31(27);
            trace[96].data[row_index] = op1_limb_27_col96;
            sub_components_inputs.memory_id_to_big_inputs[2].extend(op1_id_col68.unpack());

            lookup_data.memory_id_to_big_2.push([
                op1_id_col68,
                op1_limb_0_col69,
                op1_limb_1_col70,
                op1_limb_2_col71,
                op1_limb_3_col72,
                op1_limb_4_col73,
                op1_limb_5_col74,
                op1_limb_6_col75,
                op1_limb_7_col76,
                op1_limb_8_col77,
                op1_limb_9_col78,
                op1_limb_10_col79,
                op1_limb_11_col80,
                op1_limb_12_col81,
                op1_limb_13_col82,
                op1_limb_14_col83,
                op1_limb_15_col84,
                op1_limb_16_col85,
                op1_limb_17_col86,
                op1_limb_18_col87,
                op1_limb_19_col88,
                op1_limb_20_col89,
                op1_limb_21_col90,
                op1_limb_22_col91,
                op1_limb_23_col92,
                op1_limb_24_col93,
                op1_limb_25_col94,
                op1_limb_26_col95,
                op1_limb_27_col96,
            ]);

            // Verify Add 252.

            let sub_p_bit_tmp_f0ae_14 = ((UInt16_1)
                & (((PackedUInt16::from_m31(op0_limb_0_col40))
                    ^ (PackedUInt16::from_m31(op1_limb_0_col69)))
                    ^ (PackedUInt16::from_m31(dst_limb_0_col11))));
            let sub_p_bit_col97 = sub_p_bit_tmp_f0ae_14.as_m31();
            trace[97].data[row_index] = sub_p_bit_col97;

            lookup_data
                .opcodes_0
                .push([input_pc_col0, input_ap_col1, input_fp_col2]);
            lookup_data.opcodes_1.push([
                ((input_pc_col0) + (M31_2)),
                ((input_ap_col1) + (ap_update_add_1_col7)),
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
            &self.lookup_data.opcodes_0,
        )
        .enumerate()
        {
            let p0: PackedQM31 = memory_id_to_big.combine(v0);
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
