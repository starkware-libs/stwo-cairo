#![allow(unused_parens)]
#![allow(unused_imports)]
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
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::utils::bit_reverse_coset_to_circle_domain_order;
use stwo_prover::core::vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher};

use super::component::{Claim, InteractionClaim};
use crate::components::{
    memory_address_to_id, memory_id_to_big, pack_values, range_check_19, verify_instruction,
};
use crate::relations;

pub type InputType = CasmState;
pub type PackedInputType = PackedCasmState;
const N_TRACE_COLUMNS: usize = 123;

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
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        memory_address_to_id_state: &mut memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &mut memory_id_to_big::ClaimGenerator,
        range_check_19_state: &mut range_check_19::ClaimGenerator,
        verify_instruction_state: &mut verify_instruction::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator) {
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
            .range_check_19_inputs
            .iter()
            .for_each(|inputs| {
                range_check_19_state.add_inputs(&inputs[..n_calls]);
            });
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

pub struct SubComponentInputs {
    pub range_check_19_inputs: [Vec<range_check_19::InputType>; 28],
    pub memory_address_to_id_inputs: [Vec<memory_address_to_id::InputType>; 3],
    pub memory_id_to_big_inputs: [Vec<memory_id_to_big::InputType>; 3],
    pub verify_instruction_inputs: [Vec<verify_instruction::InputType>; 1],
}
impl SubComponentInputs {
    #[allow(unused_variables)]
    fn with_capacity(capacity: usize) -> Self {
        Self {
            range_check_19_inputs: [
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
            ],
            memory_address_to_id_inputs: [
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
            ],
            memory_id_to_big_inputs: [
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
            ],
            verify_instruction_inputs: [Vec::with_capacity(capacity)],
        }
    }

    fn bit_reverse_coset_to_circle_domain_order(&mut self) {
        self.range_check_19_inputs
            .iter_mut()
            .for_each(|vec| bit_reverse_coset_to_circle_domain_order(vec));
        self.memory_address_to_id_inputs
            .iter_mut()
            .for_each(|vec| bit_reverse_coset_to_circle_domain_order(vec));
        self.memory_id_to_big_inputs
            .iter_mut()
            .for_each(|vec| bit_reverse_coset_to_circle_domain_order(vec));
        self.verify_instruction_inputs
            .iter_mut()
            .for_each(|vec| bit_reverse_coset_to_circle_domain_order(vec));
    }
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
    const N_TRACE_COLUMNS: usize = 123;
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
    let M31_32769 = PackedM31::broadcast(M31::from(32769));
    let M31_4 = PackedM31::broadcast(M31::from(4));
    let M31_4194304 = PackedM31::broadcast(M31::from(4194304));
    let M31_64 = PackedM31::broadcast(M31::from(64));
    let M31_65536 = PackedM31::broadcast(M31::from(65536));
    let M31_8 = PackedM31::broadcast(M31::from(8));
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
    let UInt32_262143 = PackedUInt32::broadcast(UInt32::from(262143));
    let UInt32_511 = PackedUInt32::broadcast(UInt32::from(511));
    let UInt32_65536 = PackedUInt32::broadcast(UInt32::from(65536));
    let UInt32_9 = PackedUInt32::broadcast(UInt32::from(9));

    inputs
        .into_iter()
        .enumerate()
        .for_each(|(row_index, mul_opcode_is_small_f_is_imm_t_input)| {
            let input_tmp_1528 = mul_opcode_is_small_f_is_imm_t_input;
            let input_pc_col0 = input_tmp_1528.pc;
            trace[0].data[row_index] = input_pc_col0;
            let input_ap_col1 = input_tmp_1528.ap;
            trace[1].data[row_index] = input_ap_col1;
            let input_fp_col2 = input_tmp_1528.fp;
            trace[2].data[row_index] = input_fp_col2;

            // decode_instruction_cea21b812a0ef1a0.

            let memory_address_to_id_value_tmp_1529 =
                memory_address_to_id_state.deduce_output(input_pc_col0);
            let memory_id_to_big_value_tmp_1530 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_1529);
            let offset0_tmp_1531 =
                ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_1530.get_m31(0)))
                    + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_1530.get_m31(1)))
                        & (UInt16_127))
                        << (UInt16_9)));
            let offset0_col3 = offset0_tmp_1531.as_m31();
            trace[3].data[row_index] = offset0_col3;
            let offset1_tmp_1532 =
                ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_1530.get_m31(1)))
                    >> (UInt16_7))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_1530.get_m31(2)))
                        << (UInt16_2)))
                    + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_1530.get_m31(3)))
                        & (UInt16_31))
                        << (UInt16_11)));
            let offset1_col4 = offset1_tmp_1532.as_m31();
            trace[4].data[row_index] = offset1_col4;
            let dst_base_fp_tmp_1533 =
                (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_1530.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_1530.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_0))
                    & (UInt16_1));
            let dst_base_fp_col5 = dst_base_fp_tmp_1533.as_m31();
            trace[5].data[row_index] = dst_base_fp_col5;
            let op0_base_fp_tmp_1534 =
                (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_1530.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_1530.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_1))
                    & (UInt16_1));
            let op0_base_fp_col6 = op0_base_fp_tmp_1534.as_m31();
            trace[6].data[row_index] = op0_base_fp_col6;
            let ap_update_add_1_tmp_1535 =
                (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_1530.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_1530.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_11))
                    & (UInt16_1));
            let ap_update_add_1_col7 = ap_update_add_1_tmp_1535.as_m31();
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
                        M31_0,
                        M31_1,
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
                M31_0,
                M31_1,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                ap_update_add_1_col7,
                M31_0,
                M31_0,
                M31_1,
            ]);

            // read_positive_num_bits_252.

            let memory_address_to_id_value_tmp_1537 = memory_address_to_id_state.deduce_output(
                ((((dst_base_fp_col5) * (input_fp_col2))
                    + (((M31_1) - (dst_base_fp_col5)) * (input_ap_col1)))
                    + ((offset0_col3) - (M31_32768))),
            );
            let memory_id_to_big_value_tmp_1538 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_1537);
            let dst_id_col8 = memory_address_to_id_value_tmp_1537;
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
            let dst_limb_0_col9 = memory_id_to_big_value_tmp_1538.get_m31(0);
            trace[9].data[row_index] = dst_limb_0_col9;
            let dst_limb_1_col10 = memory_id_to_big_value_tmp_1538.get_m31(1);
            trace[10].data[row_index] = dst_limb_1_col10;
            let dst_limb_2_col11 = memory_id_to_big_value_tmp_1538.get_m31(2);
            trace[11].data[row_index] = dst_limb_2_col11;
            let dst_limb_3_col12 = memory_id_to_big_value_tmp_1538.get_m31(3);
            trace[12].data[row_index] = dst_limb_3_col12;
            let dst_limb_4_col13 = memory_id_to_big_value_tmp_1538.get_m31(4);
            trace[13].data[row_index] = dst_limb_4_col13;
            let dst_limb_5_col14 = memory_id_to_big_value_tmp_1538.get_m31(5);
            trace[14].data[row_index] = dst_limb_5_col14;
            let dst_limb_6_col15 = memory_id_to_big_value_tmp_1538.get_m31(6);
            trace[15].data[row_index] = dst_limb_6_col15;
            let dst_limb_7_col16 = memory_id_to_big_value_tmp_1538.get_m31(7);
            trace[16].data[row_index] = dst_limb_7_col16;
            let dst_limb_8_col17 = memory_id_to_big_value_tmp_1538.get_m31(8);
            trace[17].data[row_index] = dst_limb_8_col17;
            let dst_limb_9_col18 = memory_id_to_big_value_tmp_1538.get_m31(9);
            trace[18].data[row_index] = dst_limb_9_col18;
            let dst_limb_10_col19 = memory_id_to_big_value_tmp_1538.get_m31(10);
            trace[19].data[row_index] = dst_limb_10_col19;
            let dst_limb_11_col20 = memory_id_to_big_value_tmp_1538.get_m31(11);
            trace[20].data[row_index] = dst_limb_11_col20;
            let dst_limb_12_col21 = memory_id_to_big_value_tmp_1538.get_m31(12);
            trace[21].data[row_index] = dst_limb_12_col21;
            let dst_limb_13_col22 = memory_id_to_big_value_tmp_1538.get_m31(13);
            trace[22].data[row_index] = dst_limb_13_col22;
            let dst_limb_14_col23 = memory_id_to_big_value_tmp_1538.get_m31(14);
            trace[23].data[row_index] = dst_limb_14_col23;
            let dst_limb_15_col24 = memory_id_to_big_value_tmp_1538.get_m31(15);
            trace[24].data[row_index] = dst_limb_15_col24;
            let dst_limb_16_col25 = memory_id_to_big_value_tmp_1538.get_m31(16);
            trace[25].data[row_index] = dst_limb_16_col25;
            let dst_limb_17_col26 = memory_id_to_big_value_tmp_1538.get_m31(17);
            trace[26].data[row_index] = dst_limb_17_col26;
            let dst_limb_18_col27 = memory_id_to_big_value_tmp_1538.get_m31(18);
            trace[27].data[row_index] = dst_limb_18_col27;
            let dst_limb_19_col28 = memory_id_to_big_value_tmp_1538.get_m31(19);
            trace[28].data[row_index] = dst_limb_19_col28;
            let dst_limb_20_col29 = memory_id_to_big_value_tmp_1538.get_m31(20);
            trace[29].data[row_index] = dst_limb_20_col29;
            let dst_limb_21_col30 = memory_id_to_big_value_tmp_1538.get_m31(21);
            trace[30].data[row_index] = dst_limb_21_col30;
            let dst_limb_22_col31 = memory_id_to_big_value_tmp_1538.get_m31(22);
            trace[31].data[row_index] = dst_limb_22_col31;
            let dst_limb_23_col32 = memory_id_to_big_value_tmp_1538.get_m31(23);
            trace[32].data[row_index] = dst_limb_23_col32;
            let dst_limb_24_col33 = memory_id_to_big_value_tmp_1538.get_m31(24);
            trace[33].data[row_index] = dst_limb_24_col33;
            let dst_limb_25_col34 = memory_id_to_big_value_tmp_1538.get_m31(25);
            trace[34].data[row_index] = dst_limb_25_col34;
            let dst_limb_26_col35 = memory_id_to_big_value_tmp_1538.get_m31(26);
            trace[35].data[row_index] = dst_limb_26_col35;
            let dst_limb_27_col36 = memory_id_to_big_value_tmp_1538.get_m31(27);
            trace[36].data[row_index] = dst_limb_27_col36;
            sub_components_inputs.memory_id_to_big_inputs[0].extend(dst_id_col8.unpack());

            lookup_data.memoryidtobig[0].push([
                dst_id_col8,
                dst_limb_0_col9,
                dst_limb_1_col10,
                dst_limb_2_col11,
                dst_limb_3_col12,
                dst_limb_4_col13,
                dst_limb_5_col14,
                dst_limb_6_col15,
                dst_limb_7_col16,
                dst_limb_8_col17,
                dst_limb_9_col18,
                dst_limb_10_col19,
                dst_limb_11_col20,
                dst_limb_12_col21,
                dst_limb_13_col22,
                dst_limb_14_col23,
                dst_limb_15_col24,
                dst_limb_16_col25,
                dst_limb_17_col26,
                dst_limb_18_col27,
                dst_limb_19_col28,
                dst_limb_20_col29,
                dst_limb_21_col30,
                dst_limb_22_col31,
                dst_limb_23_col32,
                dst_limb_24_col33,
                dst_limb_25_col34,
                dst_limb_26_col35,
                dst_limb_27_col36,
            ]);

            // read_positive_num_bits_252.

            let memory_address_to_id_value_tmp_1539 = memory_address_to_id_state.deduce_output(
                ((((op0_base_fp_col6) * (input_fp_col2))
                    + (((M31_1) - (op0_base_fp_col6)) * (input_ap_col1)))
                    + ((offset1_col4) - (M31_32768))),
            );
            let memory_id_to_big_value_tmp_1540 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_1539);
            let op0_id_col37 = memory_address_to_id_value_tmp_1539;
            trace[37].data[row_index] = op0_id_col37;
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
                op0_id_col37,
            ]);
            let op0_limb_0_col38 = memory_id_to_big_value_tmp_1540.get_m31(0);
            trace[38].data[row_index] = op0_limb_0_col38;
            let op0_limb_1_col39 = memory_id_to_big_value_tmp_1540.get_m31(1);
            trace[39].data[row_index] = op0_limb_1_col39;
            let op0_limb_2_col40 = memory_id_to_big_value_tmp_1540.get_m31(2);
            trace[40].data[row_index] = op0_limb_2_col40;
            let op0_limb_3_col41 = memory_id_to_big_value_tmp_1540.get_m31(3);
            trace[41].data[row_index] = op0_limb_3_col41;
            let op0_limb_4_col42 = memory_id_to_big_value_tmp_1540.get_m31(4);
            trace[42].data[row_index] = op0_limb_4_col42;
            let op0_limb_5_col43 = memory_id_to_big_value_tmp_1540.get_m31(5);
            trace[43].data[row_index] = op0_limb_5_col43;
            let op0_limb_6_col44 = memory_id_to_big_value_tmp_1540.get_m31(6);
            trace[44].data[row_index] = op0_limb_6_col44;
            let op0_limb_7_col45 = memory_id_to_big_value_tmp_1540.get_m31(7);
            trace[45].data[row_index] = op0_limb_7_col45;
            let op0_limb_8_col46 = memory_id_to_big_value_tmp_1540.get_m31(8);
            trace[46].data[row_index] = op0_limb_8_col46;
            let op0_limb_9_col47 = memory_id_to_big_value_tmp_1540.get_m31(9);
            trace[47].data[row_index] = op0_limb_9_col47;
            let op0_limb_10_col48 = memory_id_to_big_value_tmp_1540.get_m31(10);
            trace[48].data[row_index] = op0_limb_10_col48;
            let op0_limb_11_col49 = memory_id_to_big_value_tmp_1540.get_m31(11);
            trace[49].data[row_index] = op0_limb_11_col49;
            let op0_limb_12_col50 = memory_id_to_big_value_tmp_1540.get_m31(12);
            trace[50].data[row_index] = op0_limb_12_col50;
            let op0_limb_13_col51 = memory_id_to_big_value_tmp_1540.get_m31(13);
            trace[51].data[row_index] = op0_limb_13_col51;
            let op0_limb_14_col52 = memory_id_to_big_value_tmp_1540.get_m31(14);
            trace[52].data[row_index] = op0_limb_14_col52;
            let op0_limb_15_col53 = memory_id_to_big_value_tmp_1540.get_m31(15);
            trace[53].data[row_index] = op0_limb_15_col53;
            let op0_limb_16_col54 = memory_id_to_big_value_tmp_1540.get_m31(16);
            trace[54].data[row_index] = op0_limb_16_col54;
            let op0_limb_17_col55 = memory_id_to_big_value_tmp_1540.get_m31(17);
            trace[55].data[row_index] = op0_limb_17_col55;
            let op0_limb_18_col56 = memory_id_to_big_value_tmp_1540.get_m31(18);
            trace[56].data[row_index] = op0_limb_18_col56;
            let op0_limb_19_col57 = memory_id_to_big_value_tmp_1540.get_m31(19);
            trace[57].data[row_index] = op0_limb_19_col57;
            let op0_limb_20_col58 = memory_id_to_big_value_tmp_1540.get_m31(20);
            trace[58].data[row_index] = op0_limb_20_col58;
            let op0_limb_21_col59 = memory_id_to_big_value_tmp_1540.get_m31(21);
            trace[59].data[row_index] = op0_limb_21_col59;
            let op0_limb_22_col60 = memory_id_to_big_value_tmp_1540.get_m31(22);
            trace[60].data[row_index] = op0_limb_22_col60;
            let op0_limb_23_col61 = memory_id_to_big_value_tmp_1540.get_m31(23);
            trace[61].data[row_index] = op0_limb_23_col61;
            let op0_limb_24_col62 = memory_id_to_big_value_tmp_1540.get_m31(24);
            trace[62].data[row_index] = op0_limb_24_col62;
            let op0_limb_25_col63 = memory_id_to_big_value_tmp_1540.get_m31(25);
            trace[63].data[row_index] = op0_limb_25_col63;
            let op0_limb_26_col64 = memory_id_to_big_value_tmp_1540.get_m31(26);
            trace[64].data[row_index] = op0_limb_26_col64;
            let op0_limb_27_col65 = memory_id_to_big_value_tmp_1540.get_m31(27);
            trace[65].data[row_index] = op0_limb_27_col65;
            sub_components_inputs.memory_id_to_big_inputs[1].extend(op0_id_col37.unpack());

            lookup_data.memoryidtobig[1].push([
                op0_id_col37,
                op0_limb_0_col38,
                op0_limb_1_col39,
                op0_limb_2_col40,
                op0_limb_3_col41,
                op0_limb_4_col42,
                op0_limb_5_col43,
                op0_limb_6_col44,
                op0_limb_7_col45,
                op0_limb_8_col46,
                op0_limb_9_col47,
                op0_limb_10_col48,
                op0_limb_11_col49,
                op0_limb_12_col50,
                op0_limb_13_col51,
                op0_limb_14_col52,
                op0_limb_15_col53,
                op0_limb_16_col54,
                op0_limb_17_col55,
                op0_limb_18_col56,
                op0_limb_19_col57,
                op0_limb_20_col58,
                op0_limb_21_col59,
                op0_limb_22_col60,
                op0_limb_23_col61,
                op0_limb_24_col62,
                op0_limb_25_col63,
                op0_limb_26_col64,
                op0_limb_27_col65,
            ]);

            // read_positive_num_bits_252.

            let memory_address_to_id_value_tmp_1541 =
                memory_address_to_id_state.deduce_output(((input_pc_col0) + (M31_1)));
            let memory_id_to_big_value_tmp_1542 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_1541);
            let op1_id_col66 = memory_address_to_id_value_tmp_1541;
            trace[66].data[row_index] = op1_id_col66;
            sub_components_inputs.memory_address_to_id_inputs[2]
                .extend(((input_pc_col0) + (M31_1)).unpack());

            lookup_data.memoryaddresstoid[2].push([((input_pc_col0) + (M31_1)), op1_id_col66]);
            let op1_limb_0_col67 = memory_id_to_big_value_tmp_1542.get_m31(0);
            trace[67].data[row_index] = op1_limb_0_col67;
            let op1_limb_1_col68 = memory_id_to_big_value_tmp_1542.get_m31(1);
            trace[68].data[row_index] = op1_limb_1_col68;
            let op1_limb_2_col69 = memory_id_to_big_value_tmp_1542.get_m31(2);
            trace[69].data[row_index] = op1_limb_2_col69;
            let op1_limb_3_col70 = memory_id_to_big_value_tmp_1542.get_m31(3);
            trace[70].data[row_index] = op1_limb_3_col70;
            let op1_limb_4_col71 = memory_id_to_big_value_tmp_1542.get_m31(4);
            trace[71].data[row_index] = op1_limb_4_col71;
            let op1_limb_5_col72 = memory_id_to_big_value_tmp_1542.get_m31(5);
            trace[72].data[row_index] = op1_limb_5_col72;
            let op1_limb_6_col73 = memory_id_to_big_value_tmp_1542.get_m31(6);
            trace[73].data[row_index] = op1_limb_6_col73;
            let op1_limb_7_col74 = memory_id_to_big_value_tmp_1542.get_m31(7);
            trace[74].data[row_index] = op1_limb_7_col74;
            let op1_limb_8_col75 = memory_id_to_big_value_tmp_1542.get_m31(8);
            trace[75].data[row_index] = op1_limb_8_col75;
            let op1_limb_9_col76 = memory_id_to_big_value_tmp_1542.get_m31(9);
            trace[76].data[row_index] = op1_limb_9_col76;
            let op1_limb_10_col77 = memory_id_to_big_value_tmp_1542.get_m31(10);
            trace[77].data[row_index] = op1_limb_10_col77;
            let op1_limb_11_col78 = memory_id_to_big_value_tmp_1542.get_m31(11);
            trace[78].data[row_index] = op1_limb_11_col78;
            let op1_limb_12_col79 = memory_id_to_big_value_tmp_1542.get_m31(12);
            trace[79].data[row_index] = op1_limb_12_col79;
            let op1_limb_13_col80 = memory_id_to_big_value_tmp_1542.get_m31(13);
            trace[80].data[row_index] = op1_limb_13_col80;
            let op1_limb_14_col81 = memory_id_to_big_value_tmp_1542.get_m31(14);
            trace[81].data[row_index] = op1_limb_14_col81;
            let op1_limb_15_col82 = memory_id_to_big_value_tmp_1542.get_m31(15);
            trace[82].data[row_index] = op1_limb_15_col82;
            let op1_limb_16_col83 = memory_id_to_big_value_tmp_1542.get_m31(16);
            trace[83].data[row_index] = op1_limb_16_col83;
            let op1_limb_17_col84 = memory_id_to_big_value_tmp_1542.get_m31(17);
            trace[84].data[row_index] = op1_limb_17_col84;
            let op1_limb_18_col85 = memory_id_to_big_value_tmp_1542.get_m31(18);
            trace[85].data[row_index] = op1_limb_18_col85;
            let op1_limb_19_col86 = memory_id_to_big_value_tmp_1542.get_m31(19);
            trace[86].data[row_index] = op1_limb_19_col86;
            let op1_limb_20_col87 = memory_id_to_big_value_tmp_1542.get_m31(20);
            trace[87].data[row_index] = op1_limb_20_col87;
            let op1_limb_21_col88 = memory_id_to_big_value_tmp_1542.get_m31(21);
            trace[88].data[row_index] = op1_limb_21_col88;
            let op1_limb_22_col89 = memory_id_to_big_value_tmp_1542.get_m31(22);
            trace[89].data[row_index] = op1_limb_22_col89;
            let op1_limb_23_col90 = memory_id_to_big_value_tmp_1542.get_m31(23);
            trace[90].data[row_index] = op1_limb_23_col90;
            let op1_limb_24_col91 = memory_id_to_big_value_tmp_1542.get_m31(24);
            trace[91].data[row_index] = op1_limb_24_col91;
            let op1_limb_25_col92 = memory_id_to_big_value_tmp_1542.get_m31(25);
            trace[92].data[row_index] = op1_limb_25_col92;
            let op1_limb_26_col93 = memory_id_to_big_value_tmp_1542.get_m31(26);
            trace[93].data[row_index] = op1_limb_26_col93;
            let op1_limb_27_col94 = memory_id_to_big_value_tmp_1542.get_m31(27);
            trace[94].data[row_index] = op1_limb_27_col94;
            sub_components_inputs.memory_id_to_big_inputs[2].extend(op1_id_col66.unpack());

            lookup_data.memoryidtobig[2].push([
                op1_id_col66,
                op1_limb_0_col67,
                op1_limb_1_col68,
                op1_limb_2_col69,
                op1_limb_3_col70,
                op1_limb_4_col71,
                op1_limb_5_col72,
                op1_limb_6_col73,
                op1_limb_7_col74,
                op1_limb_8_col75,
                op1_limb_9_col76,
                op1_limb_10_col77,
                op1_limb_11_col78,
                op1_limb_12_col79,
                op1_limb_13_col80,
                op1_limb_14_col81,
                op1_limb_15_col82,
                op1_limb_16_col83,
                op1_limb_17_col84,
                op1_limb_18_col85,
                op1_limb_19_col86,
                op1_limb_20_col87,
                op1_limb_21_col88,
                op1_limb_22_col89,
                op1_limb_23_col90,
                op1_limb_24_col91,
                op1_limb_25_col92,
                op1_limb_26_col93,
                op1_limb_27_col94,
            ]);

            // verify_mul252.

            let conv_tmp_1543 =
                (((M31_0) - (dst_limb_0_col9)) + ((op0_limb_0_col38) * (op1_limb_0_col67)));
            let conv_tmp_1544 = ((((M31_0) - (dst_limb_1_col10))
                + ((op0_limb_0_col38) * (op1_limb_1_col68)))
                + ((op0_limb_1_col39) * (op1_limb_0_col67)));
            let conv_tmp_1545 = (((((M31_0) - (dst_limb_2_col11))
                + ((op0_limb_0_col38) * (op1_limb_2_col69)))
                + ((op0_limb_1_col39) * (op1_limb_1_col68)))
                + ((op0_limb_2_col40) * (op1_limb_0_col67)));
            let conv_tmp_1546 = ((((((M31_0) - (dst_limb_3_col12))
                + ((op0_limb_0_col38) * (op1_limb_3_col70)))
                + ((op0_limb_1_col39) * (op1_limb_2_col69)))
                + ((op0_limb_2_col40) * (op1_limb_1_col68)))
                + ((op0_limb_3_col41) * (op1_limb_0_col67)));
            let conv_tmp_1547 = (((((((M31_0) - (dst_limb_4_col13))
                + ((op0_limb_0_col38) * (op1_limb_4_col71)))
                + ((op0_limb_1_col39) * (op1_limb_3_col70)))
                + ((op0_limb_2_col40) * (op1_limb_2_col69)))
                + ((op0_limb_3_col41) * (op1_limb_1_col68)))
                + ((op0_limb_4_col42) * (op1_limb_0_col67)));
            let conv_tmp_1548 = ((((((((M31_0) - (dst_limb_5_col14))
                + ((op0_limb_0_col38) * (op1_limb_5_col72)))
                + ((op0_limb_1_col39) * (op1_limb_4_col71)))
                + ((op0_limb_2_col40) * (op1_limb_3_col70)))
                + ((op0_limb_3_col41) * (op1_limb_2_col69)))
                + ((op0_limb_4_col42) * (op1_limb_1_col68)))
                + ((op0_limb_5_col43) * (op1_limb_0_col67)));
            let conv_tmp_1549 = (((((((((M31_0) - (dst_limb_6_col15))
                + ((op0_limb_0_col38) * (op1_limb_6_col73)))
                + ((op0_limb_1_col39) * (op1_limb_5_col72)))
                + ((op0_limb_2_col40) * (op1_limb_4_col71)))
                + ((op0_limb_3_col41) * (op1_limb_3_col70)))
                + ((op0_limb_4_col42) * (op1_limb_2_col69)))
                + ((op0_limb_5_col43) * (op1_limb_1_col68)))
                + ((op0_limb_6_col44) * (op1_limb_0_col67)));
            let conv_tmp_1550 = ((((((((((M31_0) - (dst_limb_7_col16))
                + ((op0_limb_0_col38) * (op1_limb_7_col74)))
                + ((op0_limb_1_col39) * (op1_limb_6_col73)))
                + ((op0_limb_2_col40) * (op1_limb_5_col72)))
                + ((op0_limb_3_col41) * (op1_limb_4_col71)))
                + ((op0_limb_4_col42) * (op1_limb_3_col70)))
                + ((op0_limb_5_col43) * (op1_limb_2_col69)))
                + ((op0_limb_6_col44) * (op1_limb_1_col68)))
                + ((op0_limb_7_col45) * (op1_limb_0_col67)));
            let conv_tmp_1551 = (((((((((((M31_0) - (dst_limb_8_col17))
                + ((op0_limb_0_col38) * (op1_limb_8_col75)))
                + ((op0_limb_1_col39) * (op1_limb_7_col74)))
                + ((op0_limb_2_col40) * (op1_limb_6_col73)))
                + ((op0_limb_3_col41) * (op1_limb_5_col72)))
                + ((op0_limb_4_col42) * (op1_limb_4_col71)))
                + ((op0_limb_5_col43) * (op1_limb_3_col70)))
                + ((op0_limb_6_col44) * (op1_limb_2_col69)))
                + ((op0_limb_7_col45) * (op1_limb_1_col68)))
                + ((op0_limb_8_col46) * (op1_limb_0_col67)));
            let conv_tmp_1552 = ((((((((((((M31_0) - (dst_limb_9_col18))
                + ((op0_limb_0_col38) * (op1_limb_9_col76)))
                + ((op0_limb_1_col39) * (op1_limb_8_col75)))
                + ((op0_limb_2_col40) * (op1_limb_7_col74)))
                + ((op0_limb_3_col41) * (op1_limb_6_col73)))
                + ((op0_limb_4_col42) * (op1_limb_5_col72)))
                + ((op0_limb_5_col43) * (op1_limb_4_col71)))
                + ((op0_limb_6_col44) * (op1_limb_3_col70)))
                + ((op0_limb_7_col45) * (op1_limb_2_col69)))
                + ((op0_limb_8_col46) * (op1_limb_1_col68)))
                + ((op0_limb_9_col47) * (op1_limb_0_col67)));
            let conv_tmp_1553 = (((((((((((((M31_0) - (dst_limb_10_col19))
                + ((op0_limb_0_col38) * (op1_limb_10_col77)))
                + ((op0_limb_1_col39) * (op1_limb_9_col76)))
                + ((op0_limb_2_col40) * (op1_limb_8_col75)))
                + ((op0_limb_3_col41) * (op1_limb_7_col74)))
                + ((op0_limb_4_col42) * (op1_limb_6_col73)))
                + ((op0_limb_5_col43) * (op1_limb_5_col72)))
                + ((op0_limb_6_col44) * (op1_limb_4_col71)))
                + ((op0_limb_7_col45) * (op1_limb_3_col70)))
                + ((op0_limb_8_col46) * (op1_limb_2_col69)))
                + ((op0_limb_9_col47) * (op1_limb_1_col68)))
                + ((op0_limb_10_col48) * (op1_limb_0_col67)));
            let conv_tmp_1554 = ((((((((((((((M31_0) - (dst_limb_11_col20))
                + ((op0_limb_0_col38) * (op1_limb_11_col78)))
                + ((op0_limb_1_col39) * (op1_limb_10_col77)))
                + ((op0_limb_2_col40) * (op1_limb_9_col76)))
                + ((op0_limb_3_col41) * (op1_limb_8_col75)))
                + ((op0_limb_4_col42) * (op1_limb_7_col74)))
                + ((op0_limb_5_col43) * (op1_limb_6_col73)))
                + ((op0_limb_6_col44) * (op1_limb_5_col72)))
                + ((op0_limb_7_col45) * (op1_limb_4_col71)))
                + ((op0_limb_8_col46) * (op1_limb_3_col70)))
                + ((op0_limb_9_col47) * (op1_limb_2_col69)))
                + ((op0_limb_10_col48) * (op1_limb_1_col68)))
                + ((op0_limb_11_col49) * (op1_limb_0_col67)));
            let conv_tmp_1555 = (((((((((((((((M31_0) - (dst_limb_12_col21))
                + ((op0_limb_0_col38) * (op1_limb_12_col79)))
                + ((op0_limb_1_col39) * (op1_limb_11_col78)))
                + ((op0_limb_2_col40) * (op1_limb_10_col77)))
                + ((op0_limb_3_col41) * (op1_limb_9_col76)))
                + ((op0_limb_4_col42) * (op1_limb_8_col75)))
                + ((op0_limb_5_col43) * (op1_limb_7_col74)))
                + ((op0_limb_6_col44) * (op1_limb_6_col73)))
                + ((op0_limb_7_col45) * (op1_limb_5_col72)))
                + ((op0_limb_8_col46) * (op1_limb_4_col71)))
                + ((op0_limb_9_col47) * (op1_limb_3_col70)))
                + ((op0_limb_10_col48) * (op1_limb_2_col69)))
                + ((op0_limb_11_col49) * (op1_limb_1_col68)))
                + ((op0_limb_12_col50) * (op1_limb_0_col67)));
            let conv_tmp_1556 = ((((((((((((((((M31_0) - (dst_limb_13_col22))
                + ((op0_limb_0_col38) * (op1_limb_13_col80)))
                + ((op0_limb_1_col39) * (op1_limb_12_col79)))
                + ((op0_limb_2_col40) * (op1_limb_11_col78)))
                + ((op0_limb_3_col41) * (op1_limb_10_col77)))
                + ((op0_limb_4_col42) * (op1_limb_9_col76)))
                + ((op0_limb_5_col43) * (op1_limb_8_col75)))
                + ((op0_limb_6_col44) * (op1_limb_7_col74)))
                + ((op0_limb_7_col45) * (op1_limb_6_col73)))
                + ((op0_limb_8_col46) * (op1_limb_5_col72)))
                + ((op0_limb_9_col47) * (op1_limb_4_col71)))
                + ((op0_limb_10_col48) * (op1_limb_3_col70)))
                + ((op0_limb_11_col49) * (op1_limb_2_col69)))
                + ((op0_limb_12_col50) * (op1_limb_1_col68)))
                + ((op0_limb_13_col51) * (op1_limb_0_col67)));
            let conv_tmp_1557 = (((((((((((((((((M31_0) - (dst_limb_14_col23))
                + ((op0_limb_0_col38) * (op1_limb_14_col81)))
                + ((op0_limb_1_col39) * (op1_limb_13_col80)))
                + ((op0_limb_2_col40) * (op1_limb_12_col79)))
                + ((op0_limb_3_col41) * (op1_limb_11_col78)))
                + ((op0_limb_4_col42) * (op1_limb_10_col77)))
                + ((op0_limb_5_col43) * (op1_limb_9_col76)))
                + ((op0_limb_6_col44) * (op1_limb_8_col75)))
                + ((op0_limb_7_col45) * (op1_limb_7_col74)))
                + ((op0_limb_8_col46) * (op1_limb_6_col73)))
                + ((op0_limb_9_col47) * (op1_limb_5_col72)))
                + ((op0_limb_10_col48) * (op1_limb_4_col71)))
                + ((op0_limb_11_col49) * (op1_limb_3_col70)))
                + ((op0_limb_12_col50) * (op1_limb_2_col69)))
                + ((op0_limb_13_col51) * (op1_limb_1_col68)))
                + ((op0_limb_14_col52) * (op1_limb_0_col67)));
            let conv_tmp_1558 = ((((((((((((((((((M31_0) - (dst_limb_15_col24))
                + ((op0_limb_0_col38) * (op1_limb_15_col82)))
                + ((op0_limb_1_col39) * (op1_limb_14_col81)))
                + ((op0_limb_2_col40) * (op1_limb_13_col80)))
                + ((op0_limb_3_col41) * (op1_limb_12_col79)))
                + ((op0_limb_4_col42) * (op1_limb_11_col78)))
                + ((op0_limb_5_col43) * (op1_limb_10_col77)))
                + ((op0_limb_6_col44) * (op1_limb_9_col76)))
                + ((op0_limb_7_col45) * (op1_limb_8_col75)))
                + ((op0_limb_8_col46) * (op1_limb_7_col74)))
                + ((op0_limb_9_col47) * (op1_limb_6_col73)))
                + ((op0_limb_10_col48) * (op1_limb_5_col72)))
                + ((op0_limb_11_col49) * (op1_limb_4_col71)))
                + ((op0_limb_12_col50) * (op1_limb_3_col70)))
                + ((op0_limb_13_col51) * (op1_limb_2_col69)))
                + ((op0_limb_14_col52) * (op1_limb_1_col68)))
                + ((op0_limb_15_col53) * (op1_limb_0_col67)));
            let conv_tmp_1559 = (((((((((((((((((((M31_0) - (dst_limb_16_col25))
                + ((op0_limb_0_col38) * (op1_limb_16_col83)))
                + ((op0_limb_1_col39) * (op1_limb_15_col82)))
                + ((op0_limb_2_col40) * (op1_limb_14_col81)))
                + ((op0_limb_3_col41) * (op1_limb_13_col80)))
                + ((op0_limb_4_col42) * (op1_limb_12_col79)))
                + ((op0_limb_5_col43) * (op1_limb_11_col78)))
                + ((op0_limb_6_col44) * (op1_limb_10_col77)))
                + ((op0_limb_7_col45) * (op1_limb_9_col76)))
                + ((op0_limb_8_col46) * (op1_limb_8_col75)))
                + ((op0_limb_9_col47) * (op1_limb_7_col74)))
                + ((op0_limb_10_col48) * (op1_limb_6_col73)))
                + ((op0_limb_11_col49) * (op1_limb_5_col72)))
                + ((op0_limb_12_col50) * (op1_limb_4_col71)))
                + ((op0_limb_13_col51) * (op1_limb_3_col70)))
                + ((op0_limb_14_col52) * (op1_limb_2_col69)))
                + ((op0_limb_15_col53) * (op1_limb_1_col68)))
                + ((op0_limb_16_col54) * (op1_limb_0_col67)));
            let conv_tmp_1560 = ((((((((((((((((((((M31_0) - (dst_limb_17_col26))
                + ((op0_limb_0_col38) * (op1_limb_17_col84)))
                + ((op0_limb_1_col39) * (op1_limb_16_col83)))
                + ((op0_limb_2_col40) * (op1_limb_15_col82)))
                + ((op0_limb_3_col41) * (op1_limb_14_col81)))
                + ((op0_limb_4_col42) * (op1_limb_13_col80)))
                + ((op0_limb_5_col43) * (op1_limb_12_col79)))
                + ((op0_limb_6_col44) * (op1_limb_11_col78)))
                + ((op0_limb_7_col45) * (op1_limb_10_col77)))
                + ((op0_limb_8_col46) * (op1_limb_9_col76)))
                + ((op0_limb_9_col47) * (op1_limb_8_col75)))
                + ((op0_limb_10_col48) * (op1_limb_7_col74)))
                + ((op0_limb_11_col49) * (op1_limb_6_col73)))
                + ((op0_limb_12_col50) * (op1_limb_5_col72)))
                + ((op0_limb_13_col51) * (op1_limb_4_col71)))
                + ((op0_limb_14_col52) * (op1_limb_3_col70)))
                + ((op0_limb_15_col53) * (op1_limb_2_col69)))
                + ((op0_limb_16_col54) * (op1_limb_1_col68)))
                + ((op0_limb_17_col55) * (op1_limb_0_col67)));
            let conv_tmp_1561 = (((((((((((((((((((((M31_0)
                - (dst_limb_18_col27))
                + ((op0_limb_0_col38) * (op1_limb_18_col85)))
                + ((op0_limb_1_col39) * (op1_limb_17_col84)))
                + ((op0_limb_2_col40) * (op1_limb_16_col83)))
                + ((op0_limb_3_col41) * (op1_limb_15_col82)))
                + ((op0_limb_4_col42) * (op1_limb_14_col81)))
                + ((op0_limb_5_col43) * (op1_limb_13_col80)))
                + ((op0_limb_6_col44) * (op1_limb_12_col79)))
                + ((op0_limb_7_col45) * (op1_limb_11_col78)))
                + ((op0_limb_8_col46) * (op1_limb_10_col77)))
                + ((op0_limb_9_col47) * (op1_limb_9_col76)))
                + ((op0_limb_10_col48) * (op1_limb_8_col75)))
                + ((op0_limb_11_col49) * (op1_limb_7_col74)))
                + ((op0_limb_12_col50) * (op1_limb_6_col73)))
                + ((op0_limb_13_col51) * (op1_limb_5_col72)))
                + ((op0_limb_14_col52) * (op1_limb_4_col71)))
                + ((op0_limb_15_col53) * (op1_limb_3_col70)))
                + ((op0_limb_16_col54) * (op1_limb_2_col69)))
                + ((op0_limb_17_col55) * (op1_limb_1_col68)))
                + ((op0_limb_18_col56) * (op1_limb_0_col67)));
            let conv_tmp_1562 = ((((((((((((((((((((((M31_0)
                - (dst_limb_19_col28))
                + ((op0_limb_0_col38) * (op1_limb_19_col86)))
                + ((op0_limb_1_col39) * (op1_limb_18_col85)))
                + ((op0_limb_2_col40) * (op1_limb_17_col84)))
                + ((op0_limb_3_col41) * (op1_limb_16_col83)))
                + ((op0_limb_4_col42) * (op1_limb_15_col82)))
                + ((op0_limb_5_col43) * (op1_limb_14_col81)))
                + ((op0_limb_6_col44) * (op1_limb_13_col80)))
                + ((op0_limb_7_col45) * (op1_limb_12_col79)))
                + ((op0_limb_8_col46) * (op1_limb_11_col78)))
                + ((op0_limb_9_col47) * (op1_limb_10_col77)))
                + ((op0_limb_10_col48) * (op1_limb_9_col76)))
                + ((op0_limb_11_col49) * (op1_limb_8_col75)))
                + ((op0_limb_12_col50) * (op1_limb_7_col74)))
                + ((op0_limb_13_col51) * (op1_limb_6_col73)))
                + ((op0_limb_14_col52) * (op1_limb_5_col72)))
                + ((op0_limb_15_col53) * (op1_limb_4_col71)))
                + ((op0_limb_16_col54) * (op1_limb_3_col70)))
                + ((op0_limb_17_col55) * (op1_limb_2_col69)))
                + ((op0_limb_18_col56) * (op1_limb_1_col68)))
                + ((op0_limb_19_col57) * (op1_limb_0_col67)));
            let conv_tmp_1563 = (((((((((((((((((((((((M31_0)
                - (dst_limb_20_col29))
                + ((op0_limb_0_col38) * (op1_limb_20_col87)))
                + ((op0_limb_1_col39) * (op1_limb_19_col86)))
                + ((op0_limb_2_col40) * (op1_limb_18_col85)))
                + ((op0_limb_3_col41) * (op1_limb_17_col84)))
                + ((op0_limb_4_col42) * (op1_limb_16_col83)))
                + ((op0_limb_5_col43) * (op1_limb_15_col82)))
                + ((op0_limb_6_col44) * (op1_limb_14_col81)))
                + ((op0_limb_7_col45) * (op1_limb_13_col80)))
                + ((op0_limb_8_col46) * (op1_limb_12_col79)))
                + ((op0_limb_9_col47) * (op1_limb_11_col78)))
                + ((op0_limb_10_col48) * (op1_limb_10_col77)))
                + ((op0_limb_11_col49) * (op1_limb_9_col76)))
                + ((op0_limb_12_col50) * (op1_limb_8_col75)))
                + ((op0_limb_13_col51) * (op1_limb_7_col74)))
                + ((op0_limb_14_col52) * (op1_limb_6_col73)))
                + ((op0_limb_15_col53) * (op1_limb_5_col72)))
                + ((op0_limb_16_col54) * (op1_limb_4_col71)))
                + ((op0_limb_17_col55) * (op1_limb_3_col70)))
                + ((op0_limb_18_col56) * (op1_limb_2_col69)))
                + ((op0_limb_19_col57) * (op1_limb_1_col68)))
                + ((op0_limb_20_col58) * (op1_limb_0_col67)));
            let conv_tmp_1564 = ((((((((((((((((((((((((M31_0)
                - (dst_limb_21_col30))
                + ((op0_limb_0_col38) * (op1_limb_21_col88)))
                + ((op0_limb_1_col39) * (op1_limb_20_col87)))
                + ((op0_limb_2_col40) * (op1_limb_19_col86)))
                + ((op0_limb_3_col41) * (op1_limb_18_col85)))
                + ((op0_limb_4_col42) * (op1_limb_17_col84)))
                + ((op0_limb_5_col43) * (op1_limb_16_col83)))
                + ((op0_limb_6_col44) * (op1_limb_15_col82)))
                + ((op0_limb_7_col45) * (op1_limb_14_col81)))
                + ((op0_limb_8_col46) * (op1_limb_13_col80)))
                + ((op0_limb_9_col47) * (op1_limb_12_col79)))
                + ((op0_limb_10_col48) * (op1_limb_11_col78)))
                + ((op0_limb_11_col49) * (op1_limb_10_col77)))
                + ((op0_limb_12_col50) * (op1_limb_9_col76)))
                + ((op0_limb_13_col51) * (op1_limb_8_col75)))
                + ((op0_limb_14_col52) * (op1_limb_7_col74)))
                + ((op0_limb_15_col53) * (op1_limb_6_col73)))
                + ((op0_limb_16_col54) * (op1_limb_5_col72)))
                + ((op0_limb_17_col55) * (op1_limb_4_col71)))
                + ((op0_limb_18_col56) * (op1_limb_3_col70)))
                + ((op0_limb_19_col57) * (op1_limb_2_col69)))
                + ((op0_limb_20_col58) * (op1_limb_1_col68)))
                + ((op0_limb_21_col59) * (op1_limb_0_col67)));
            let conv_tmp_1565 = (((((((((((((((((((((((((M31_0)
                - (dst_limb_22_col31))
                + ((op0_limb_0_col38) * (op1_limb_22_col89)))
                + ((op0_limb_1_col39) * (op1_limb_21_col88)))
                + ((op0_limb_2_col40) * (op1_limb_20_col87)))
                + ((op0_limb_3_col41) * (op1_limb_19_col86)))
                + ((op0_limb_4_col42) * (op1_limb_18_col85)))
                + ((op0_limb_5_col43) * (op1_limb_17_col84)))
                + ((op0_limb_6_col44) * (op1_limb_16_col83)))
                + ((op0_limb_7_col45) * (op1_limb_15_col82)))
                + ((op0_limb_8_col46) * (op1_limb_14_col81)))
                + ((op0_limb_9_col47) * (op1_limb_13_col80)))
                + ((op0_limb_10_col48) * (op1_limb_12_col79)))
                + ((op0_limb_11_col49) * (op1_limb_11_col78)))
                + ((op0_limb_12_col50) * (op1_limb_10_col77)))
                + ((op0_limb_13_col51) * (op1_limb_9_col76)))
                + ((op0_limb_14_col52) * (op1_limb_8_col75)))
                + ((op0_limb_15_col53) * (op1_limb_7_col74)))
                + ((op0_limb_16_col54) * (op1_limb_6_col73)))
                + ((op0_limb_17_col55) * (op1_limb_5_col72)))
                + ((op0_limb_18_col56) * (op1_limb_4_col71)))
                + ((op0_limb_19_col57) * (op1_limb_3_col70)))
                + ((op0_limb_20_col58) * (op1_limb_2_col69)))
                + ((op0_limb_21_col59) * (op1_limb_1_col68)))
                + ((op0_limb_22_col60) * (op1_limb_0_col67)));
            let conv_tmp_1566 = ((((((((((((((((((((((((((M31_0)
                - (dst_limb_23_col32))
                + ((op0_limb_0_col38) * (op1_limb_23_col90)))
                + ((op0_limb_1_col39) * (op1_limb_22_col89)))
                + ((op0_limb_2_col40) * (op1_limb_21_col88)))
                + ((op0_limb_3_col41) * (op1_limb_20_col87)))
                + ((op0_limb_4_col42) * (op1_limb_19_col86)))
                + ((op0_limb_5_col43) * (op1_limb_18_col85)))
                + ((op0_limb_6_col44) * (op1_limb_17_col84)))
                + ((op0_limb_7_col45) * (op1_limb_16_col83)))
                + ((op0_limb_8_col46) * (op1_limb_15_col82)))
                + ((op0_limb_9_col47) * (op1_limb_14_col81)))
                + ((op0_limb_10_col48) * (op1_limb_13_col80)))
                + ((op0_limb_11_col49) * (op1_limb_12_col79)))
                + ((op0_limb_12_col50) * (op1_limb_11_col78)))
                + ((op0_limb_13_col51) * (op1_limb_10_col77)))
                + ((op0_limb_14_col52) * (op1_limb_9_col76)))
                + ((op0_limb_15_col53) * (op1_limb_8_col75)))
                + ((op0_limb_16_col54) * (op1_limb_7_col74)))
                + ((op0_limb_17_col55) * (op1_limb_6_col73)))
                + ((op0_limb_18_col56) * (op1_limb_5_col72)))
                + ((op0_limb_19_col57) * (op1_limb_4_col71)))
                + ((op0_limb_20_col58) * (op1_limb_3_col70)))
                + ((op0_limb_21_col59) * (op1_limb_2_col69)))
                + ((op0_limb_22_col60) * (op1_limb_1_col68)))
                + ((op0_limb_23_col61) * (op1_limb_0_col67)));
            let conv_tmp_1567 = (((((((((((((((((((((((((((M31_0)
                - (dst_limb_24_col33))
                + ((op0_limb_0_col38) * (op1_limb_24_col91)))
                + ((op0_limb_1_col39) * (op1_limb_23_col90)))
                + ((op0_limb_2_col40) * (op1_limb_22_col89)))
                + ((op0_limb_3_col41) * (op1_limb_21_col88)))
                + ((op0_limb_4_col42) * (op1_limb_20_col87)))
                + ((op0_limb_5_col43) * (op1_limb_19_col86)))
                + ((op0_limb_6_col44) * (op1_limb_18_col85)))
                + ((op0_limb_7_col45) * (op1_limb_17_col84)))
                + ((op0_limb_8_col46) * (op1_limb_16_col83)))
                + ((op0_limb_9_col47) * (op1_limb_15_col82)))
                + ((op0_limb_10_col48) * (op1_limb_14_col81)))
                + ((op0_limb_11_col49) * (op1_limb_13_col80)))
                + ((op0_limb_12_col50) * (op1_limb_12_col79)))
                + ((op0_limb_13_col51) * (op1_limb_11_col78)))
                + ((op0_limb_14_col52) * (op1_limb_10_col77)))
                + ((op0_limb_15_col53) * (op1_limb_9_col76)))
                + ((op0_limb_16_col54) * (op1_limb_8_col75)))
                + ((op0_limb_17_col55) * (op1_limb_7_col74)))
                + ((op0_limb_18_col56) * (op1_limb_6_col73)))
                + ((op0_limb_19_col57) * (op1_limb_5_col72)))
                + ((op0_limb_20_col58) * (op1_limb_4_col71)))
                + ((op0_limb_21_col59) * (op1_limb_3_col70)))
                + ((op0_limb_22_col60) * (op1_limb_2_col69)))
                + ((op0_limb_23_col61) * (op1_limb_1_col68)))
                + ((op0_limb_24_col62) * (op1_limb_0_col67)));
            let conv_tmp_1568 = ((((((((((((((((((((((((((((M31_0)
                - (dst_limb_25_col34))
                + ((op0_limb_0_col38) * (op1_limb_25_col92)))
                + ((op0_limb_1_col39) * (op1_limb_24_col91)))
                + ((op0_limb_2_col40) * (op1_limb_23_col90)))
                + ((op0_limb_3_col41) * (op1_limb_22_col89)))
                + ((op0_limb_4_col42) * (op1_limb_21_col88)))
                + ((op0_limb_5_col43) * (op1_limb_20_col87)))
                + ((op0_limb_6_col44) * (op1_limb_19_col86)))
                + ((op0_limb_7_col45) * (op1_limb_18_col85)))
                + ((op0_limb_8_col46) * (op1_limb_17_col84)))
                + ((op0_limb_9_col47) * (op1_limb_16_col83)))
                + ((op0_limb_10_col48) * (op1_limb_15_col82)))
                + ((op0_limb_11_col49) * (op1_limb_14_col81)))
                + ((op0_limb_12_col50) * (op1_limb_13_col80)))
                + ((op0_limb_13_col51) * (op1_limb_12_col79)))
                + ((op0_limb_14_col52) * (op1_limb_11_col78)))
                + ((op0_limb_15_col53) * (op1_limb_10_col77)))
                + ((op0_limb_16_col54) * (op1_limb_9_col76)))
                + ((op0_limb_17_col55) * (op1_limb_8_col75)))
                + ((op0_limb_18_col56) * (op1_limb_7_col74)))
                + ((op0_limb_19_col57) * (op1_limb_6_col73)))
                + ((op0_limb_20_col58) * (op1_limb_5_col72)))
                + ((op0_limb_21_col59) * (op1_limb_4_col71)))
                + ((op0_limb_22_col60) * (op1_limb_3_col70)))
                + ((op0_limb_23_col61) * (op1_limb_2_col69)))
                + ((op0_limb_24_col62) * (op1_limb_1_col68)))
                + ((op0_limb_25_col63) * (op1_limb_0_col67)));
            let conv_tmp_1569 = (((((((((((((((((((((((((((((M31_0)
                - (dst_limb_26_col35))
                + ((op0_limb_0_col38) * (op1_limb_26_col93)))
                + ((op0_limb_1_col39) * (op1_limb_25_col92)))
                + ((op0_limb_2_col40) * (op1_limb_24_col91)))
                + ((op0_limb_3_col41) * (op1_limb_23_col90)))
                + ((op0_limb_4_col42) * (op1_limb_22_col89)))
                + ((op0_limb_5_col43) * (op1_limb_21_col88)))
                + ((op0_limb_6_col44) * (op1_limb_20_col87)))
                + ((op0_limb_7_col45) * (op1_limb_19_col86)))
                + ((op0_limb_8_col46) * (op1_limb_18_col85)))
                + ((op0_limb_9_col47) * (op1_limb_17_col84)))
                + ((op0_limb_10_col48) * (op1_limb_16_col83)))
                + ((op0_limb_11_col49) * (op1_limb_15_col82)))
                + ((op0_limb_12_col50) * (op1_limb_14_col81)))
                + ((op0_limb_13_col51) * (op1_limb_13_col80)))
                + ((op0_limb_14_col52) * (op1_limb_12_col79)))
                + ((op0_limb_15_col53) * (op1_limb_11_col78)))
                + ((op0_limb_16_col54) * (op1_limb_10_col77)))
                + ((op0_limb_17_col55) * (op1_limb_9_col76)))
                + ((op0_limb_18_col56) * (op1_limb_8_col75)))
                + ((op0_limb_19_col57) * (op1_limb_7_col74)))
                + ((op0_limb_20_col58) * (op1_limb_6_col73)))
                + ((op0_limb_21_col59) * (op1_limb_5_col72)))
                + ((op0_limb_22_col60) * (op1_limb_4_col71)))
                + ((op0_limb_23_col61) * (op1_limb_3_col70)))
                + ((op0_limb_24_col62) * (op1_limb_2_col69)))
                + ((op0_limb_25_col63) * (op1_limb_1_col68)))
                + ((op0_limb_26_col64) * (op1_limb_0_col67)));
            let conv_tmp_1570 = ((((((((((((((((((((((((((((((M31_0)
                - (dst_limb_27_col36))
                + ((op0_limb_0_col38) * (op1_limb_27_col94)))
                + ((op0_limb_1_col39) * (op1_limb_26_col93)))
                + ((op0_limb_2_col40) * (op1_limb_25_col92)))
                + ((op0_limb_3_col41) * (op1_limb_24_col91)))
                + ((op0_limb_4_col42) * (op1_limb_23_col90)))
                + ((op0_limb_5_col43) * (op1_limb_22_col89)))
                + ((op0_limb_6_col44) * (op1_limb_21_col88)))
                + ((op0_limb_7_col45) * (op1_limb_20_col87)))
                + ((op0_limb_8_col46) * (op1_limb_19_col86)))
                + ((op0_limb_9_col47) * (op1_limb_18_col85)))
                + ((op0_limb_10_col48) * (op1_limb_17_col84)))
                + ((op0_limb_11_col49) * (op1_limb_16_col83)))
                + ((op0_limb_12_col50) * (op1_limb_15_col82)))
                + ((op0_limb_13_col51) * (op1_limb_14_col81)))
                + ((op0_limb_14_col52) * (op1_limb_13_col80)))
                + ((op0_limb_15_col53) * (op1_limb_12_col79)))
                + ((op0_limb_16_col54) * (op1_limb_11_col78)))
                + ((op0_limb_17_col55) * (op1_limb_10_col77)))
                + ((op0_limb_18_col56) * (op1_limb_9_col76)))
                + ((op0_limb_19_col57) * (op1_limb_8_col75)))
                + ((op0_limb_20_col58) * (op1_limb_7_col74)))
                + ((op0_limb_21_col59) * (op1_limb_6_col73)))
                + ((op0_limb_22_col60) * (op1_limb_5_col72)))
                + ((op0_limb_23_col61) * (op1_limb_4_col71)))
                + ((op0_limb_24_col62) * (op1_limb_3_col70)))
                + ((op0_limb_25_col63) * (op1_limb_2_col69)))
                + ((op0_limb_26_col64) * (op1_limb_1_col68)))
                + ((op0_limb_27_col65) * (op1_limb_0_col67)));
            let conv_tmp_1571 = ((((((((((((((((((((((((((((M31_0)
                + ((op0_limb_1_col39) * (op1_limb_27_col94)))
                + ((op0_limb_2_col40) * (op1_limb_26_col93)))
                + ((op0_limb_3_col41) * (op1_limb_25_col92)))
                + ((op0_limb_4_col42) * (op1_limb_24_col91)))
                + ((op0_limb_5_col43) * (op1_limb_23_col90)))
                + ((op0_limb_6_col44) * (op1_limb_22_col89)))
                + ((op0_limb_7_col45) * (op1_limb_21_col88)))
                + ((op0_limb_8_col46) * (op1_limb_20_col87)))
                + ((op0_limb_9_col47) * (op1_limb_19_col86)))
                + ((op0_limb_10_col48) * (op1_limb_18_col85)))
                + ((op0_limb_11_col49) * (op1_limb_17_col84)))
                + ((op0_limb_12_col50) * (op1_limb_16_col83)))
                + ((op0_limb_13_col51) * (op1_limb_15_col82)))
                + ((op0_limb_14_col52) * (op1_limb_14_col81)))
                + ((op0_limb_15_col53) * (op1_limb_13_col80)))
                + ((op0_limb_16_col54) * (op1_limb_12_col79)))
                + ((op0_limb_17_col55) * (op1_limb_11_col78)))
                + ((op0_limb_18_col56) * (op1_limb_10_col77)))
                + ((op0_limb_19_col57) * (op1_limb_9_col76)))
                + ((op0_limb_20_col58) * (op1_limb_8_col75)))
                + ((op0_limb_21_col59) * (op1_limb_7_col74)))
                + ((op0_limb_22_col60) * (op1_limb_6_col73)))
                + ((op0_limb_23_col61) * (op1_limb_5_col72)))
                + ((op0_limb_24_col62) * (op1_limb_4_col71)))
                + ((op0_limb_25_col63) * (op1_limb_3_col70)))
                + ((op0_limb_26_col64) * (op1_limb_2_col69)))
                + ((op0_limb_27_col65) * (op1_limb_1_col68)));
            let conv_tmp_1572 = (((((((((((((((((((((((((((M31_0)
                + ((op0_limb_2_col40) * (op1_limb_27_col94)))
                + ((op0_limb_3_col41) * (op1_limb_26_col93)))
                + ((op0_limb_4_col42) * (op1_limb_25_col92)))
                + ((op0_limb_5_col43) * (op1_limb_24_col91)))
                + ((op0_limb_6_col44) * (op1_limb_23_col90)))
                + ((op0_limb_7_col45) * (op1_limb_22_col89)))
                + ((op0_limb_8_col46) * (op1_limb_21_col88)))
                + ((op0_limb_9_col47) * (op1_limb_20_col87)))
                + ((op0_limb_10_col48) * (op1_limb_19_col86)))
                + ((op0_limb_11_col49) * (op1_limb_18_col85)))
                + ((op0_limb_12_col50) * (op1_limb_17_col84)))
                + ((op0_limb_13_col51) * (op1_limb_16_col83)))
                + ((op0_limb_14_col52) * (op1_limb_15_col82)))
                + ((op0_limb_15_col53) * (op1_limb_14_col81)))
                + ((op0_limb_16_col54) * (op1_limb_13_col80)))
                + ((op0_limb_17_col55) * (op1_limb_12_col79)))
                + ((op0_limb_18_col56) * (op1_limb_11_col78)))
                + ((op0_limb_19_col57) * (op1_limb_10_col77)))
                + ((op0_limb_20_col58) * (op1_limb_9_col76)))
                + ((op0_limb_21_col59) * (op1_limb_8_col75)))
                + ((op0_limb_22_col60) * (op1_limb_7_col74)))
                + ((op0_limb_23_col61) * (op1_limb_6_col73)))
                + ((op0_limb_24_col62) * (op1_limb_5_col72)))
                + ((op0_limb_25_col63) * (op1_limb_4_col71)))
                + ((op0_limb_26_col64) * (op1_limb_3_col70)))
                + ((op0_limb_27_col65) * (op1_limb_2_col69)));
            let conv_tmp_1573 = ((((((((((((((((((((((((((M31_0)
                + ((op0_limb_3_col41) * (op1_limb_27_col94)))
                + ((op0_limb_4_col42) * (op1_limb_26_col93)))
                + ((op0_limb_5_col43) * (op1_limb_25_col92)))
                + ((op0_limb_6_col44) * (op1_limb_24_col91)))
                + ((op0_limb_7_col45) * (op1_limb_23_col90)))
                + ((op0_limb_8_col46) * (op1_limb_22_col89)))
                + ((op0_limb_9_col47) * (op1_limb_21_col88)))
                + ((op0_limb_10_col48) * (op1_limb_20_col87)))
                + ((op0_limb_11_col49) * (op1_limb_19_col86)))
                + ((op0_limb_12_col50) * (op1_limb_18_col85)))
                + ((op0_limb_13_col51) * (op1_limb_17_col84)))
                + ((op0_limb_14_col52) * (op1_limb_16_col83)))
                + ((op0_limb_15_col53) * (op1_limb_15_col82)))
                + ((op0_limb_16_col54) * (op1_limb_14_col81)))
                + ((op0_limb_17_col55) * (op1_limb_13_col80)))
                + ((op0_limb_18_col56) * (op1_limb_12_col79)))
                + ((op0_limb_19_col57) * (op1_limb_11_col78)))
                + ((op0_limb_20_col58) * (op1_limb_10_col77)))
                + ((op0_limb_21_col59) * (op1_limb_9_col76)))
                + ((op0_limb_22_col60) * (op1_limb_8_col75)))
                + ((op0_limb_23_col61) * (op1_limb_7_col74)))
                + ((op0_limb_24_col62) * (op1_limb_6_col73)))
                + ((op0_limb_25_col63) * (op1_limb_5_col72)))
                + ((op0_limb_26_col64) * (op1_limb_4_col71)))
                + ((op0_limb_27_col65) * (op1_limb_3_col70)));
            let conv_tmp_1574 = (((((((((((((((((((((((((M31_0)
                + ((op0_limb_4_col42) * (op1_limb_27_col94)))
                + ((op0_limb_5_col43) * (op1_limb_26_col93)))
                + ((op0_limb_6_col44) * (op1_limb_25_col92)))
                + ((op0_limb_7_col45) * (op1_limb_24_col91)))
                + ((op0_limb_8_col46) * (op1_limb_23_col90)))
                + ((op0_limb_9_col47) * (op1_limb_22_col89)))
                + ((op0_limb_10_col48) * (op1_limb_21_col88)))
                + ((op0_limb_11_col49) * (op1_limb_20_col87)))
                + ((op0_limb_12_col50) * (op1_limb_19_col86)))
                + ((op0_limb_13_col51) * (op1_limb_18_col85)))
                + ((op0_limb_14_col52) * (op1_limb_17_col84)))
                + ((op0_limb_15_col53) * (op1_limb_16_col83)))
                + ((op0_limb_16_col54) * (op1_limb_15_col82)))
                + ((op0_limb_17_col55) * (op1_limb_14_col81)))
                + ((op0_limb_18_col56) * (op1_limb_13_col80)))
                + ((op0_limb_19_col57) * (op1_limb_12_col79)))
                + ((op0_limb_20_col58) * (op1_limb_11_col78)))
                + ((op0_limb_21_col59) * (op1_limb_10_col77)))
                + ((op0_limb_22_col60) * (op1_limb_9_col76)))
                + ((op0_limb_23_col61) * (op1_limb_8_col75)))
                + ((op0_limb_24_col62) * (op1_limb_7_col74)))
                + ((op0_limb_25_col63) * (op1_limb_6_col73)))
                + ((op0_limb_26_col64) * (op1_limb_5_col72)))
                + ((op0_limb_27_col65) * (op1_limb_4_col71)));
            let conv_tmp_1575 = ((((((((((((((((((((((((M31_0)
                + ((op0_limb_5_col43) * (op1_limb_27_col94)))
                + ((op0_limb_6_col44) * (op1_limb_26_col93)))
                + ((op0_limb_7_col45) * (op1_limb_25_col92)))
                + ((op0_limb_8_col46) * (op1_limb_24_col91)))
                + ((op0_limb_9_col47) * (op1_limb_23_col90)))
                + ((op0_limb_10_col48) * (op1_limb_22_col89)))
                + ((op0_limb_11_col49) * (op1_limb_21_col88)))
                + ((op0_limb_12_col50) * (op1_limb_20_col87)))
                + ((op0_limb_13_col51) * (op1_limb_19_col86)))
                + ((op0_limb_14_col52) * (op1_limb_18_col85)))
                + ((op0_limb_15_col53) * (op1_limb_17_col84)))
                + ((op0_limb_16_col54) * (op1_limb_16_col83)))
                + ((op0_limb_17_col55) * (op1_limb_15_col82)))
                + ((op0_limb_18_col56) * (op1_limb_14_col81)))
                + ((op0_limb_19_col57) * (op1_limb_13_col80)))
                + ((op0_limb_20_col58) * (op1_limb_12_col79)))
                + ((op0_limb_21_col59) * (op1_limb_11_col78)))
                + ((op0_limb_22_col60) * (op1_limb_10_col77)))
                + ((op0_limb_23_col61) * (op1_limb_9_col76)))
                + ((op0_limb_24_col62) * (op1_limb_8_col75)))
                + ((op0_limb_25_col63) * (op1_limb_7_col74)))
                + ((op0_limb_26_col64) * (op1_limb_6_col73)))
                + ((op0_limb_27_col65) * (op1_limb_5_col72)));
            let conv_tmp_1576 = (((((((((((((((((((((((M31_0)
                + ((op0_limb_6_col44) * (op1_limb_27_col94)))
                + ((op0_limb_7_col45) * (op1_limb_26_col93)))
                + ((op0_limb_8_col46) * (op1_limb_25_col92)))
                + ((op0_limb_9_col47) * (op1_limb_24_col91)))
                + ((op0_limb_10_col48) * (op1_limb_23_col90)))
                + ((op0_limb_11_col49) * (op1_limb_22_col89)))
                + ((op0_limb_12_col50) * (op1_limb_21_col88)))
                + ((op0_limb_13_col51) * (op1_limb_20_col87)))
                + ((op0_limb_14_col52) * (op1_limb_19_col86)))
                + ((op0_limb_15_col53) * (op1_limb_18_col85)))
                + ((op0_limb_16_col54) * (op1_limb_17_col84)))
                + ((op0_limb_17_col55) * (op1_limb_16_col83)))
                + ((op0_limb_18_col56) * (op1_limb_15_col82)))
                + ((op0_limb_19_col57) * (op1_limb_14_col81)))
                + ((op0_limb_20_col58) * (op1_limb_13_col80)))
                + ((op0_limb_21_col59) * (op1_limb_12_col79)))
                + ((op0_limb_22_col60) * (op1_limb_11_col78)))
                + ((op0_limb_23_col61) * (op1_limb_10_col77)))
                + ((op0_limb_24_col62) * (op1_limb_9_col76)))
                + ((op0_limb_25_col63) * (op1_limb_8_col75)))
                + ((op0_limb_26_col64) * (op1_limb_7_col74)))
                + ((op0_limb_27_col65) * (op1_limb_6_col73)));
            let conv_tmp_1577 = ((((((((((((((((((((((M31_0)
                + ((op0_limb_7_col45) * (op1_limb_27_col94)))
                + ((op0_limb_8_col46) * (op1_limb_26_col93)))
                + ((op0_limb_9_col47) * (op1_limb_25_col92)))
                + ((op0_limb_10_col48) * (op1_limb_24_col91)))
                + ((op0_limb_11_col49) * (op1_limb_23_col90)))
                + ((op0_limb_12_col50) * (op1_limb_22_col89)))
                + ((op0_limb_13_col51) * (op1_limb_21_col88)))
                + ((op0_limb_14_col52) * (op1_limb_20_col87)))
                + ((op0_limb_15_col53) * (op1_limb_19_col86)))
                + ((op0_limb_16_col54) * (op1_limb_18_col85)))
                + ((op0_limb_17_col55) * (op1_limb_17_col84)))
                + ((op0_limb_18_col56) * (op1_limb_16_col83)))
                + ((op0_limb_19_col57) * (op1_limb_15_col82)))
                + ((op0_limb_20_col58) * (op1_limb_14_col81)))
                + ((op0_limb_21_col59) * (op1_limb_13_col80)))
                + ((op0_limb_22_col60) * (op1_limb_12_col79)))
                + ((op0_limb_23_col61) * (op1_limb_11_col78)))
                + ((op0_limb_24_col62) * (op1_limb_10_col77)))
                + ((op0_limb_25_col63) * (op1_limb_9_col76)))
                + ((op0_limb_26_col64) * (op1_limb_8_col75)))
                + ((op0_limb_27_col65) * (op1_limb_7_col74)));
            let conv_tmp_1578 = (((((((((((((((((((((M31_0)
                + ((op0_limb_8_col46) * (op1_limb_27_col94)))
                + ((op0_limb_9_col47) * (op1_limb_26_col93)))
                + ((op0_limb_10_col48) * (op1_limb_25_col92)))
                + ((op0_limb_11_col49) * (op1_limb_24_col91)))
                + ((op0_limb_12_col50) * (op1_limb_23_col90)))
                + ((op0_limb_13_col51) * (op1_limb_22_col89)))
                + ((op0_limb_14_col52) * (op1_limb_21_col88)))
                + ((op0_limb_15_col53) * (op1_limb_20_col87)))
                + ((op0_limb_16_col54) * (op1_limb_19_col86)))
                + ((op0_limb_17_col55) * (op1_limb_18_col85)))
                + ((op0_limb_18_col56) * (op1_limb_17_col84)))
                + ((op0_limb_19_col57) * (op1_limb_16_col83)))
                + ((op0_limb_20_col58) * (op1_limb_15_col82)))
                + ((op0_limb_21_col59) * (op1_limb_14_col81)))
                + ((op0_limb_22_col60) * (op1_limb_13_col80)))
                + ((op0_limb_23_col61) * (op1_limb_12_col79)))
                + ((op0_limb_24_col62) * (op1_limb_11_col78)))
                + ((op0_limb_25_col63) * (op1_limb_10_col77)))
                + ((op0_limb_26_col64) * (op1_limb_9_col76)))
                + ((op0_limb_27_col65) * (op1_limb_8_col75)));
            let conv_tmp_1579 = ((((((((((((((((((((M31_0)
                + ((op0_limb_9_col47) * (op1_limb_27_col94)))
                + ((op0_limb_10_col48) * (op1_limb_26_col93)))
                + ((op0_limb_11_col49) * (op1_limb_25_col92)))
                + ((op0_limb_12_col50) * (op1_limb_24_col91)))
                + ((op0_limb_13_col51) * (op1_limb_23_col90)))
                + ((op0_limb_14_col52) * (op1_limb_22_col89)))
                + ((op0_limb_15_col53) * (op1_limb_21_col88)))
                + ((op0_limb_16_col54) * (op1_limb_20_col87)))
                + ((op0_limb_17_col55) * (op1_limb_19_col86)))
                + ((op0_limb_18_col56) * (op1_limb_18_col85)))
                + ((op0_limb_19_col57) * (op1_limb_17_col84)))
                + ((op0_limb_20_col58) * (op1_limb_16_col83)))
                + ((op0_limb_21_col59) * (op1_limb_15_col82)))
                + ((op0_limb_22_col60) * (op1_limb_14_col81)))
                + ((op0_limb_23_col61) * (op1_limb_13_col80)))
                + ((op0_limb_24_col62) * (op1_limb_12_col79)))
                + ((op0_limb_25_col63) * (op1_limb_11_col78)))
                + ((op0_limb_26_col64) * (op1_limb_10_col77)))
                + ((op0_limb_27_col65) * (op1_limb_9_col76)));
            let conv_tmp_1580 = (((((((((((((((((((M31_0)
                + ((op0_limb_10_col48) * (op1_limb_27_col94)))
                + ((op0_limb_11_col49) * (op1_limb_26_col93)))
                + ((op0_limb_12_col50) * (op1_limb_25_col92)))
                + ((op0_limb_13_col51) * (op1_limb_24_col91)))
                + ((op0_limb_14_col52) * (op1_limb_23_col90)))
                + ((op0_limb_15_col53) * (op1_limb_22_col89)))
                + ((op0_limb_16_col54) * (op1_limb_21_col88)))
                + ((op0_limb_17_col55) * (op1_limb_20_col87)))
                + ((op0_limb_18_col56) * (op1_limb_19_col86)))
                + ((op0_limb_19_col57) * (op1_limb_18_col85)))
                + ((op0_limb_20_col58) * (op1_limb_17_col84)))
                + ((op0_limb_21_col59) * (op1_limb_16_col83)))
                + ((op0_limb_22_col60) * (op1_limb_15_col82)))
                + ((op0_limb_23_col61) * (op1_limb_14_col81)))
                + ((op0_limb_24_col62) * (op1_limb_13_col80)))
                + ((op0_limb_25_col63) * (op1_limb_12_col79)))
                + ((op0_limb_26_col64) * (op1_limb_11_col78)))
                + ((op0_limb_27_col65) * (op1_limb_10_col77)));
            let conv_tmp_1581 = ((((((((((((((((((M31_0)
                + ((op0_limb_11_col49) * (op1_limb_27_col94)))
                + ((op0_limb_12_col50) * (op1_limb_26_col93)))
                + ((op0_limb_13_col51) * (op1_limb_25_col92)))
                + ((op0_limb_14_col52) * (op1_limb_24_col91)))
                + ((op0_limb_15_col53) * (op1_limb_23_col90)))
                + ((op0_limb_16_col54) * (op1_limb_22_col89)))
                + ((op0_limb_17_col55) * (op1_limb_21_col88)))
                + ((op0_limb_18_col56) * (op1_limb_20_col87)))
                + ((op0_limb_19_col57) * (op1_limb_19_col86)))
                + ((op0_limb_20_col58) * (op1_limb_18_col85)))
                + ((op0_limb_21_col59) * (op1_limb_17_col84)))
                + ((op0_limb_22_col60) * (op1_limb_16_col83)))
                + ((op0_limb_23_col61) * (op1_limb_15_col82)))
                + ((op0_limb_24_col62) * (op1_limb_14_col81)))
                + ((op0_limb_25_col63) * (op1_limb_13_col80)))
                + ((op0_limb_26_col64) * (op1_limb_12_col79)))
                + ((op0_limb_27_col65) * (op1_limb_11_col78)));
            let conv_tmp_1582 = (((((((((((((((((M31_0)
                + ((op0_limb_12_col50) * (op1_limb_27_col94)))
                + ((op0_limb_13_col51) * (op1_limb_26_col93)))
                + ((op0_limb_14_col52) * (op1_limb_25_col92)))
                + ((op0_limb_15_col53) * (op1_limb_24_col91)))
                + ((op0_limb_16_col54) * (op1_limb_23_col90)))
                + ((op0_limb_17_col55) * (op1_limb_22_col89)))
                + ((op0_limb_18_col56) * (op1_limb_21_col88)))
                + ((op0_limb_19_col57) * (op1_limb_20_col87)))
                + ((op0_limb_20_col58) * (op1_limb_19_col86)))
                + ((op0_limb_21_col59) * (op1_limb_18_col85)))
                + ((op0_limb_22_col60) * (op1_limb_17_col84)))
                + ((op0_limb_23_col61) * (op1_limb_16_col83)))
                + ((op0_limb_24_col62) * (op1_limb_15_col82)))
                + ((op0_limb_25_col63) * (op1_limb_14_col81)))
                + ((op0_limb_26_col64) * (op1_limb_13_col80)))
                + ((op0_limb_27_col65) * (op1_limb_12_col79)));
            let conv_tmp_1583 = ((((((((((((((((M31_0)
                + ((op0_limb_13_col51) * (op1_limb_27_col94)))
                + ((op0_limb_14_col52) * (op1_limb_26_col93)))
                + ((op0_limb_15_col53) * (op1_limb_25_col92)))
                + ((op0_limb_16_col54) * (op1_limb_24_col91)))
                + ((op0_limb_17_col55) * (op1_limb_23_col90)))
                + ((op0_limb_18_col56) * (op1_limb_22_col89)))
                + ((op0_limb_19_col57) * (op1_limb_21_col88)))
                + ((op0_limb_20_col58) * (op1_limb_20_col87)))
                + ((op0_limb_21_col59) * (op1_limb_19_col86)))
                + ((op0_limb_22_col60) * (op1_limb_18_col85)))
                + ((op0_limb_23_col61) * (op1_limb_17_col84)))
                + ((op0_limb_24_col62) * (op1_limb_16_col83)))
                + ((op0_limb_25_col63) * (op1_limb_15_col82)))
                + ((op0_limb_26_col64) * (op1_limb_14_col81)))
                + ((op0_limb_27_col65) * (op1_limb_13_col80)));
            let conv_tmp_1584 = (((((((((((((((M31_0)
                + ((op0_limb_14_col52) * (op1_limb_27_col94)))
                + ((op0_limb_15_col53) * (op1_limb_26_col93)))
                + ((op0_limb_16_col54) * (op1_limb_25_col92)))
                + ((op0_limb_17_col55) * (op1_limb_24_col91)))
                + ((op0_limb_18_col56) * (op1_limb_23_col90)))
                + ((op0_limb_19_col57) * (op1_limb_22_col89)))
                + ((op0_limb_20_col58) * (op1_limb_21_col88)))
                + ((op0_limb_21_col59) * (op1_limb_20_col87)))
                + ((op0_limb_22_col60) * (op1_limb_19_col86)))
                + ((op0_limb_23_col61) * (op1_limb_18_col85)))
                + ((op0_limb_24_col62) * (op1_limb_17_col84)))
                + ((op0_limb_25_col63) * (op1_limb_16_col83)))
                + ((op0_limb_26_col64) * (op1_limb_15_col82)))
                + ((op0_limb_27_col65) * (op1_limb_14_col81)));
            let conv_tmp_1585 = ((((((((((((((M31_0)
                + ((op0_limb_15_col53) * (op1_limb_27_col94)))
                + ((op0_limb_16_col54) * (op1_limb_26_col93)))
                + ((op0_limb_17_col55) * (op1_limb_25_col92)))
                + ((op0_limb_18_col56) * (op1_limb_24_col91)))
                + ((op0_limb_19_col57) * (op1_limb_23_col90)))
                + ((op0_limb_20_col58) * (op1_limb_22_col89)))
                + ((op0_limb_21_col59) * (op1_limb_21_col88)))
                + ((op0_limb_22_col60) * (op1_limb_20_col87)))
                + ((op0_limb_23_col61) * (op1_limb_19_col86)))
                + ((op0_limb_24_col62) * (op1_limb_18_col85)))
                + ((op0_limb_25_col63) * (op1_limb_17_col84)))
                + ((op0_limb_26_col64) * (op1_limb_16_col83)))
                + ((op0_limb_27_col65) * (op1_limb_15_col82)));
            let conv_tmp_1586 = (((((((((((((M31_0)
                + ((op0_limb_16_col54) * (op1_limb_27_col94)))
                + ((op0_limb_17_col55) * (op1_limb_26_col93)))
                + ((op0_limb_18_col56) * (op1_limb_25_col92)))
                + ((op0_limb_19_col57) * (op1_limb_24_col91)))
                + ((op0_limb_20_col58) * (op1_limb_23_col90)))
                + ((op0_limb_21_col59) * (op1_limb_22_col89)))
                + ((op0_limb_22_col60) * (op1_limb_21_col88)))
                + ((op0_limb_23_col61) * (op1_limb_20_col87)))
                + ((op0_limb_24_col62) * (op1_limb_19_col86)))
                + ((op0_limb_25_col63) * (op1_limb_18_col85)))
                + ((op0_limb_26_col64) * (op1_limb_17_col84)))
                + ((op0_limb_27_col65) * (op1_limb_16_col83)));
            let conv_tmp_1587 = ((((((((((((M31_0)
                + ((op0_limb_17_col55) * (op1_limb_27_col94)))
                + ((op0_limb_18_col56) * (op1_limb_26_col93)))
                + ((op0_limb_19_col57) * (op1_limb_25_col92)))
                + ((op0_limb_20_col58) * (op1_limb_24_col91)))
                + ((op0_limb_21_col59) * (op1_limb_23_col90)))
                + ((op0_limb_22_col60) * (op1_limb_22_col89)))
                + ((op0_limb_23_col61) * (op1_limb_21_col88)))
                + ((op0_limb_24_col62) * (op1_limb_20_col87)))
                + ((op0_limb_25_col63) * (op1_limb_19_col86)))
                + ((op0_limb_26_col64) * (op1_limb_18_col85)))
                + ((op0_limb_27_col65) * (op1_limb_17_col84)));
            let conv_tmp_1588 = (((((((((((M31_0)
                + ((op0_limb_18_col56) * (op1_limb_27_col94)))
                + ((op0_limb_19_col57) * (op1_limb_26_col93)))
                + ((op0_limb_20_col58) * (op1_limb_25_col92)))
                + ((op0_limb_21_col59) * (op1_limb_24_col91)))
                + ((op0_limb_22_col60) * (op1_limb_23_col90)))
                + ((op0_limb_23_col61) * (op1_limb_22_col89)))
                + ((op0_limb_24_col62) * (op1_limb_21_col88)))
                + ((op0_limb_25_col63) * (op1_limb_20_col87)))
                + ((op0_limb_26_col64) * (op1_limb_19_col86)))
                + ((op0_limb_27_col65) * (op1_limb_18_col85)));
            let conv_tmp_1589 = ((((((((((M31_0)
                + ((op0_limb_19_col57) * (op1_limb_27_col94)))
                + ((op0_limb_20_col58) * (op1_limb_26_col93)))
                + ((op0_limb_21_col59) * (op1_limb_25_col92)))
                + ((op0_limb_22_col60) * (op1_limb_24_col91)))
                + ((op0_limb_23_col61) * (op1_limb_23_col90)))
                + ((op0_limb_24_col62) * (op1_limb_22_col89)))
                + ((op0_limb_25_col63) * (op1_limb_21_col88)))
                + ((op0_limb_26_col64) * (op1_limb_20_col87)))
                + ((op0_limb_27_col65) * (op1_limb_19_col86)));
            let conv_tmp_1590 = (((((((((M31_0)
                + ((op0_limb_20_col58) * (op1_limb_27_col94)))
                + ((op0_limb_21_col59) * (op1_limb_26_col93)))
                + ((op0_limb_22_col60) * (op1_limb_25_col92)))
                + ((op0_limb_23_col61) * (op1_limb_24_col91)))
                + ((op0_limb_24_col62) * (op1_limb_23_col90)))
                + ((op0_limb_25_col63) * (op1_limb_22_col89)))
                + ((op0_limb_26_col64) * (op1_limb_21_col88)))
                + ((op0_limb_27_col65) * (op1_limb_20_col87)));
            let conv_tmp_1591 = ((((((((M31_0) + ((op0_limb_21_col59) * (op1_limb_27_col94)))
                + ((op0_limb_22_col60) * (op1_limb_26_col93)))
                + ((op0_limb_23_col61) * (op1_limb_25_col92)))
                + ((op0_limb_24_col62) * (op1_limb_24_col91)))
                + ((op0_limb_25_col63) * (op1_limb_23_col90)))
                + ((op0_limb_26_col64) * (op1_limb_22_col89)))
                + ((op0_limb_27_col65) * (op1_limb_21_col88)));
            let conv_tmp_1592 = (((((((M31_0) + ((op0_limb_22_col60) * (op1_limb_27_col94)))
                + ((op0_limb_23_col61) * (op1_limb_26_col93)))
                + ((op0_limb_24_col62) * (op1_limb_25_col92)))
                + ((op0_limb_25_col63) * (op1_limb_24_col91)))
                + ((op0_limb_26_col64) * (op1_limb_23_col90)))
                + ((op0_limb_27_col65) * (op1_limb_22_col89)));
            let conv_tmp_1593 = ((((((M31_0) + ((op0_limb_23_col61) * (op1_limb_27_col94)))
                + ((op0_limb_24_col62) * (op1_limb_26_col93)))
                + ((op0_limb_25_col63) * (op1_limb_25_col92)))
                + ((op0_limb_26_col64) * (op1_limb_24_col91)))
                + ((op0_limb_27_col65) * (op1_limb_23_col90)));
            let conv_tmp_1594 = (((((M31_0) + ((op0_limb_24_col62) * (op1_limb_27_col94)))
                + ((op0_limb_25_col63) * (op1_limb_26_col93)))
                + ((op0_limb_26_col64) * (op1_limb_25_col92)))
                + ((op0_limb_27_col65) * (op1_limb_24_col91)));
            let conv_tmp_1595 = ((((M31_0) + ((op0_limb_25_col63) * (op1_limb_27_col94)))
                + ((op0_limb_26_col64) * (op1_limb_26_col93)))
                + ((op0_limb_27_col65) * (op1_limb_25_col92)));
            let conv_tmp_1596 = (((M31_0) + ((op0_limb_26_col64) * (op1_limb_27_col94)))
                + ((op0_limb_27_col65) * (op1_limb_26_col93)));
            let conv_tmp_1597 = ((M31_0) + ((op0_limb_27_col65) * (op1_limb_27_col94)));
            let conv_mod_tmp_1598 = ((((M31_0) + ((M31_32) * (conv_tmp_1543)))
                - ((M31_4) * (conv_tmp_1564)))
                + ((M31_8) * (conv_tmp_1592)));
            let conv_mod_tmp_1599 = (((((M31_0) + ((M31_1) * (conv_tmp_1543)))
                + ((M31_32) * (conv_tmp_1544)))
                - ((M31_4) * (conv_tmp_1565)))
                + ((M31_8) * (conv_tmp_1593)));
            let conv_mod_tmp_1600 = (((((M31_0) + ((M31_1) * (conv_tmp_1544)))
                + ((M31_32) * (conv_tmp_1545)))
                - ((M31_4) * (conv_tmp_1566)))
                + ((M31_8) * (conv_tmp_1594)));
            let conv_mod_tmp_1601 = (((((M31_0) + ((M31_1) * (conv_tmp_1545)))
                + ((M31_32) * (conv_tmp_1546)))
                - ((M31_4) * (conv_tmp_1567)))
                + ((M31_8) * (conv_tmp_1595)));
            let conv_mod_tmp_1602 = (((((M31_0) + ((M31_1) * (conv_tmp_1546)))
                + ((M31_32) * (conv_tmp_1547)))
                - ((M31_4) * (conv_tmp_1568)))
                + ((M31_8) * (conv_tmp_1596)));
            let conv_mod_tmp_1603 = (((((M31_0) + ((M31_1) * (conv_tmp_1547)))
                + ((M31_32) * (conv_tmp_1548)))
                - ((M31_4) * (conv_tmp_1569)))
                + ((M31_8) * (conv_tmp_1597)));
            let conv_mod_tmp_1604 = ((((M31_0) + ((M31_1) * (conv_tmp_1548)))
                + ((M31_32) * (conv_tmp_1549)))
                - ((M31_4) * (conv_tmp_1570)));
            let conv_mod_tmp_1605 = (((((M31_0) + ((M31_2) * (conv_tmp_1543)))
                + ((M31_1) * (conv_tmp_1549)))
                + ((M31_32) * (conv_tmp_1550)))
                - ((M31_4) * (conv_tmp_1571)));
            let conv_mod_tmp_1606 = (((((M31_0) + ((M31_2) * (conv_tmp_1544)))
                + ((M31_1) * (conv_tmp_1550)))
                + ((M31_32) * (conv_tmp_1551)))
                - ((M31_4) * (conv_tmp_1572)));
            let conv_mod_tmp_1607 = (((((M31_0) + ((M31_2) * (conv_tmp_1545)))
                + ((M31_1) * (conv_tmp_1551)))
                + ((M31_32) * (conv_tmp_1552)))
                - ((M31_4) * (conv_tmp_1573)));
            let conv_mod_tmp_1608 = (((((M31_0) + ((M31_2) * (conv_tmp_1546)))
                + ((M31_1) * (conv_tmp_1552)))
                + ((M31_32) * (conv_tmp_1553)))
                - ((M31_4) * (conv_tmp_1574)));
            let conv_mod_tmp_1609 = (((((M31_0) + ((M31_2) * (conv_tmp_1547)))
                + ((M31_1) * (conv_tmp_1553)))
                + ((M31_32) * (conv_tmp_1554)))
                - ((M31_4) * (conv_tmp_1575)));
            let conv_mod_tmp_1610 = (((((M31_0) + ((M31_2) * (conv_tmp_1548)))
                + ((M31_1) * (conv_tmp_1554)))
                + ((M31_32) * (conv_tmp_1555)))
                - ((M31_4) * (conv_tmp_1576)));
            let conv_mod_tmp_1611 = (((((M31_0) + ((M31_2) * (conv_tmp_1549)))
                + ((M31_1) * (conv_tmp_1555)))
                + ((M31_32) * (conv_tmp_1556)))
                - ((M31_4) * (conv_tmp_1577)));
            let conv_mod_tmp_1612 = (((((M31_0) + ((M31_2) * (conv_tmp_1550)))
                + ((M31_1) * (conv_tmp_1556)))
                + ((M31_32) * (conv_tmp_1557)))
                - ((M31_4) * (conv_tmp_1578)));
            let conv_mod_tmp_1613 = (((((M31_0) + ((M31_2) * (conv_tmp_1551)))
                + ((M31_1) * (conv_tmp_1557)))
                + ((M31_32) * (conv_tmp_1558)))
                - ((M31_4) * (conv_tmp_1579)));
            let conv_mod_tmp_1614 = (((((M31_0) + ((M31_2) * (conv_tmp_1552)))
                + ((M31_1) * (conv_tmp_1558)))
                + ((M31_32) * (conv_tmp_1559)))
                - ((M31_4) * (conv_tmp_1580)));
            let conv_mod_tmp_1615 = (((((M31_0) + ((M31_2) * (conv_tmp_1553)))
                + ((M31_1) * (conv_tmp_1559)))
                + ((M31_32) * (conv_tmp_1560)))
                - ((M31_4) * (conv_tmp_1581)));
            let conv_mod_tmp_1616 = (((((M31_0) + ((M31_2) * (conv_tmp_1554)))
                + ((M31_1) * (conv_tmp_1560)))
                + ((M31_32) * (conv_tmp_1561)))
                - ((M31_4) * (conv_tmp_1582)));
            let conv_mod_tmp_1617 = (((((M31_0) + ((M31_2) * (conv_tmp_1555)))
                + ((M31_1) * (conv_tmp_1561)))
                + ((M31_32) * (conv_tmp_1562)))
                - ((M31_4) * (conv_tmp_1583)));
            let conv_mod_tmp_1618 = (((((M31_0) + ((M31_2) * (conv_tmp_1556)))
                + ((M31_1) * (conv_tmp_1562)))
                + ((M31_32) * (conv_tmp_1563)))
                - ((M31_4) * (conv_tmp_1584)));
            let conv_mod_tmp_1619 = (((((M31_0) + ((M31_2) * (conv_tmp_1557)))
                + ((M31_1) * (conv_tmp_1563)))
                - ((M31_4) * (conv_tmp_1585)))
                + ((M31_64) * (conv_tmp_1592)));
            let conv_mod_tmp_1620 = (((((M31_0) + ((M31_2) * (conv_tmp_1558)))
                - ((M31_4) * (conv_tmp_1586)))
                + ((M31_2) * (conv_tmp_1592)))
                + ((M31_64) * (conv_tmp_1593)));
            let conv_mod_tmp_1621 = (((((M31_0) + ((M31_2) * (conv_tmp_1559)))
                - ((M31_4) * (conv_tmp_1587)))
                + ((M31_2) * (conv_tmp_1593)))
                + ((M31_64) * (conv_tmp_1594)));
            let conv_mod_tmp_1622 = (((((M31_0) + ((M31_2) * (conv_tmp_1560)))
                - ((M31_4) * (conv_tmp_1588)))
                + ((M31_2) * (conv_tmp_1594)))
                + ((M31_64) * (conv_tmp_1595)));
            let conv_mod_tmp_1623 = (((((M31_0) + ((M31_2) * (conv_tmp_1561)))
                - ((M31_4) * (conv_tmp_1589)))
                + ((M31_2) * (conv_tmp_1595)))
                + ((M31_64) * (conv_tmp_1596)));
            let conv_mod_tmp_1624 = (((((M31_0) + ((M31_2) * (conv_tmp_1562)))
                - ((M31_4) * (conv_tmp_1590)))
                + ((M31_2) * (conv_tmp_1596)))
                + ((M31_64) * (conv_tmp_1597)));
            let conv_mod_tmp_1625 = ((((M31_0) + ((M31_2) * (conv_tmp_1563)))
                - ((M31_4) * (conv_tmp_1591)))
                + ((M31_2) * (conv_tmp_1597)));
            let k_mod_2_18_biased_tmp_1626 =
                ((((PackedUInt32::from_m31(((conv_mod_tmp_1598) + (M31_134217728))))
                    + (((PackedUInt32::from_m31(((conv_mod_tmp_1599) + (M31_134217728))))
                        & (UInt32_511))
                        << (UInt32_9)))
                    + (UInt32_65536))
                    & (UInt32_262143));
            let k_col95 = ((k_mod_2_18_biased_tmp_1626.low().as_m31())
                + (((k_mod_2_18_biased_tmp_1626.high().as_m31()) - (M31_1)) * (M31_65536)));
            trace[95].data[row_index] = k_col95;

            sub_components_inputs.range_check_19_inputs[0]
                .extend([((k_col95) + (M31_262144))].unpack());

            lookup_data.rangecheck_19[0].push([((k_col95) + (M31_262144))]);
            let carry_0_col96 =
                ((((conv_mod_tmp_1598) - ((M31_1) * (k_col95))) + (M31_0)) * (M31_4194304));
            trace[96].data[row_index] = carry_0_col96;

            sub_components_inputs.range_check_19_inputs[1]
                .extend([((carry_0_col96) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[1].push([((carry_0_col96) + (M31_131072))]);
            let carry_1_col97 = (((conv_mod_tmp_1599) + (carry_0_col96)) * (M31_4194304));
            trace[97].data[row_index] = carry_1_col97;

            sub_components_inputs.range_check_19_inputs[2]
                .extend([((carry_1_col97) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[2].push([((carry_1_col97) + (M31_131072))]);
            let carry_2_col98 = (((conv_mod_tmp_1600) + (carry_1_col97)) * (M31_4194304));
            trace[98].data[row_index] = carry_2_col98;

            sub_components_inputs.range_check_19_inputs[3]
                .extend([((carry_2_col98) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[3].push([((carry_2_col98) + (M31_131072))]);
            let carry_3_col99 = (((conv_mod_tmp_1601) + (carry_2_col98)) * (M31_4194304));
            trace[99].data[row_index] = carry_3_col99;

            sub_components_inputs.range_check_19_inputs[4]
                .extend([((carry_3_col99) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[4].push([((carry_3_col99) + (M31_131072))]);
            let carry_4_col100 = (((conv_mod_tmp_1602) + (carry_3_col99)) * (M31_4194304));
            trace[100].data[row_index] = carry_4_col100;

            sub_components_inputs.range_check_19_inputs[5]
                .extend([((carry_4_col100) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[5].push([((carry_4_col100) + (M31_131072))]);
            let carry_5_col101 = (((conv_mod_tmp_1603) + (carry_4_col100)) * (M31_4194304));
            trace[101].data[row_index] = carry_5_col101;

            sub_components_inputs.range_check_19_inputs[6]
                .extend([((carry_5_col101) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[6].push([((carry_5_col101) + (M31_131072))]);
            let carry_6_col102 = (((conv_mod_tmp_1604) + (carry_5_col101)) * (M31_4194304));
            trace[102].data[row_index] = carry_6_col102;

            sub_components_inputs.range_check_19_inputs[7]
                .extend([((carry_6_col102) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[7].push([((carry_6_col102) + (M31_131072))]);
            let carry_7_col103 = (((conv_mod_tmp_1605) + (carry_6_col102)) * (M31_4194304));
            trace[103].data[row_index] = carry_7_col103;

            sub_components_inputs.range_check_19_inputs[8]
                .extend([((carry_7_col103) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[8].push([((carry_7_col103) + (M31_131072))]);
            let carry_8_col104 = (((conv_mod_tmp_1606) + (carry_7_col103)) * (M31_4194304));
            trace[104].data[row_index] = carry_8_col104;

            sub_components_inputs.range_check_19_inputs[9]
                .extend([((carry_8_col104) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[9].push([((carry_8_col104) + (M31_131072))]);
            let carry_9_col105 = (((conv_mod_tmp_1607) + (carry_8_col104)) * (M31_4194304));
            trace[105].data[row_index] = carry_9_col105;

            sub_components_inputs.range_check_19_inputs[10]
                .extend([((carry_9_col105) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[10].push([((carry_9_col105) + (M31_131072))]);
            let carry_10_col106 = (((conv_mod_tmp_1608) + (carry_9_col105)) * (M31_4194304));
            trace[106].data[row_index] = carry_10_col106;

            sub_components_inputs.range_check_19_inputs[11]
                .extend([((carry_10_col106) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[11].push([((carry_10_col106) + (M31_131072))]);
            let carry_11_col107 = (((conv_mod_tmp_1609) + (carry_10_col106)) * (M31_4194304));
            trace[107].data[row_index] = carry_11_col107;

            sub_components_inputs.range_check_19_inputs[12]
                .extend([((carry_11_col107) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[12].push([((carry_11_col107) + (M31_131072))]);
            let carry_12_col108 = (((conv_mod_tmp_1610) + (carry_11_col107)) * (M31_4194304));
            trace[108].data[row_index] = carry_12_col108;

            sub_components_inputs.range_check_19_inputs[13]
                .extend([((carry_12_col108) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[13].push([((carry_12_col108) + (M31_131072))]);
            let carry_13_col109 = (((conv_mod_tmp_1611) + (carry_12_col108)) * (M31_4194304));
            trace[109].data[row_index] = carry_13_col109;

            sub_components_inputs.range_check_19_inputs[14]
                .extend([((carry_13_col109) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[14].push([((carry_13_col109) + (M31_131072))]);
            let carry_14_col110 = (((conv_mod_tmp_1612) + (carry_13_col109)) * (M31_4194304));
            trace[110].data[row_index] = carry_14_col110;

            sub_components_inputs.range_check_19_inputs[15]
                .extend([((carry_14_col110) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[15].push([((carry_14_col110) + (M31_131072))]);
            let carry_15_col111 = (((conv_mod_tmp_1613) + (carry_14_col110)) * (M31_4194304));
            trace[111].data[row_index] = carry_15_col111;

            sub_components_inputs.range_check_19_inputs[16]
                .extend([((carry_15_col111) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[16].push([((carry_15_col111) + (M31_131072))]);
            let carry_16_col112 = (((conv_mod_tmp_1614) + (carry_15_col111)) * (M31_4194304));
            trace[112].data[row_index] = carry_16_col112;

            sub_components_inputs.range_check_19_inputs[17]
                .extend([((carry_16_col112) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[17].push([((carry_16_col112) + (M31_131072))]);
            let carry_17_col113 = (((conv_mod_tmp_1615) + (carry_16_col112)) * (M31_4194304));
            trace[113].data[row_index] = carry_17_col113;

            sub_components_inputs.range_check_19_inputs[18]
                .extend([((carry_17_col113) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[18].push([((carry_17_col113) + (M31_131072))]);
            let carry_18_col114 = (((conv_mod_tmp_1616) + (carry_17_col113)) * (M31_4194304));
            trace[114].data[row_index] = carry_18_col114;

            sub_components_inputs.range_check_19_inputs[19]
                .extend([((carry_18_col114) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[19].push([((carry_18_col114) + (M31_131072))]);
            let carry_19_col115 = (((conv_mod_tmp_1617) + (carry_18_col114)) * (M31_4194304));
            trace[115].data[row_index] = carry_19_col115;

            sub_components_inputs.range_check_19_inputs[20]
                .extend([((carry_19_col115) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[20].push([((carry_19_col115) + (M31_131072))]);
            let carry_20_col116 = (((conv_mod_tmp_1618) + (carry_19_col115)) * (M31_4194304));
            trace[116].data[row_index] = carry_20_col116;

            sub_components_inputs.range_check_19_inputs[21]
                .extend([((carry_20_col116) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[21].push([((carry_20_col116) + (M31_131072))]);
            let carry_21_col117 = ((((conv_mod_tmp_1619) - ((M31_136) * (k_col95)))
                + (carry_20_col116))
                * (M31_4194304));
            trace[117].data[row_index] = carry_21_col117;

            sub_components_inputs.range_check_19_inputs[22]
                .extend([((carry_21_col117) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[22].push([((carry_21_col117) + (M31_131072))]);
            let carry_22_col118 = (((conv_mod_tmp_1620) + (carry_21_col117)) * (M31_4194304));
            trace[118].data[row_index] = carry_22_col118;

            sub_components_inputs.range_check_19_inputs[23]
                .extend([((carry_22_col118) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[23].push([((carry_22_col118) + (M31_131072))]);
            let carry_23_col119 = (((conv_mod_tmp_1621) + (carry_22_col118)) * (M31_4194304));
            trace[119].data[row_index] = carry_23_col119;

            sub_components_inputs.range_check_19_inputs[24]
                .extend([((carry_23_col119) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[24].push([((carry_23_col119) + (M31_131072))]);
            let carry_24_col120 = (((conv_mod_tmp_1622) + (carry_23_col119)) * (M31_4194304));
            trace[120].data[row_index] = carry_24_col120;

            sub_components_inputs.range_check_19_inputs[25]
                .extend([((carry_24_col120) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[25].push([((carry_24_col120) + (M31_131072))]);
            let carry_25_col121 = (((conv_mod_tmp_1623) + (carry_24_col120)) * (M31_4194304));
            trace[121].data[row_index] = carry_25_col121;

            sub_components_inputs.range_check_19_inputs[26]
                .extend([((carry_25_col121) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[26].push([((carry_25_col121) + (M31_131072))]);
            let carry_26_col122 = (((conv_mod_tmp_1624) + (carry_25_col121)) * (M31_4194304));
            trace[122].data[row_index] = carry_26_col122;

            sub_components_inputs.range_check_19_inputs[27]
                .extend([((carry_26_col122) + (M31_131072))].unpack());

            lookup_data.rangecheck_19[27].push([((carry_26_col122) + (M31_131072))]);

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
    pub rangecheck_19: [Vec<[PackedM31; 1]>; 28],
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
            rangecheck_19: [
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
            ],
            verifyinstruction: [Vec::with_capacity(capacity)],
        }
    }
}

pub struct InteractionClaimGenerator {
    pub n_calls: usize,
    pub lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        memoryaddresstoid_lookup_elements: &relations::MemoryAddressToId,
        memoryidtobig_lookup_elements: &relations::MemoryIdToBig,
        opcodes_lookup_elements: &relations::Opcodes,
        rangecheck_19_lookup_elements: &relations::RangeCheck_19,
        verifyinstruction_lookup_elements: &relations::VerifyInstruction,
    ) -> InteractionClaim {
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
        let lookup_row = &self.lookup_data.rangecheck_19[0];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[1];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[2];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[3];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[4];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[5];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[6];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[7];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[8];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[9];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[10];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[11];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[12];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[13];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[14];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[15];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[16];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[17];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[18];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[19];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[20];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[21];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[22];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[23];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[24];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[25];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[26];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_19[27];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_19_lookup_elements.combine(lookup_values);
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
