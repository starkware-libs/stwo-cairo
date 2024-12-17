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
const N_TRACE_COLUMNS: usize = 42;

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
    pub memory_address_to_id_inputs: [Vec<memory_address_to_id::InputType>; 2],
    pub memory_id_to_big_inputs: [Vec<memory_id_to_big::InputType>; 2],
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
    const N_TRACE_COLUMNS: usize = 42;
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
    let M31_32769 = PackedM31::broadcast(M31::from(32769));
    let M31_511 = PackedM31::broadcast(M31::from(511));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let UInt16_1 = PackedUInt16::broadcast(UInt16::from(1));
    let UInt16_11 = PackedUInt16::broadcast(UInt16::from(11));
    let UInt16_127 = PackedUInt16::broadcast(UInt16::from(127));
    let UInt16_3 = PackedUInt16::broadcast(UInt16::from(3));
    let UInt16_6 = PackedUInt16::broadcast(UInt16::from(6));
    let UInt16_9 = PackedUInt16::broadcast(UInt16::from(9));

    inputs.into_iter().enumerate().for_each(
        |(row_index, jnz_opcode_is_taken_t_dst_base_fp_f_input)| {
            let input_tmp_490d_0 = jnz_opcode_is_taken_t_dst_base_fp_f_input;
            let input_pc_col0 = input_tmp_490d_0.pc;
            trace[0].data[row_index] = input_pc_col0;
            let input_ap_col1 = input_tmp_490d_0.ap;
            trace[1].data[row_index] = input_ap_col1;
            let input_fp_col2 = input_tmp_490d_0.fp;
            trace[2].data[row_index] = input_fp_col2;

            // DecodeInstruction_d2b4cd588a3e2a7b.

            let memoryaddresstoid_value_tmp_490d_1 =
                memory_address_to_id_state.deduce_output(input_pc_col0);
            let memoryidtobig_value_tmp_490d_2 =
                memory_id_to_big_state.deduce_output(memoryaddresstoid_value_tmp_490d_1);
            let offset0_tmp_490d_3 =
                ((PackedUInt16::from_m31(memoryidtobig_value_tmp_490d_2.get_m31(0)))
                    + (((PackedUInt16::from_m31(memoryidtobig_value_tmp_490d_2.get_m31(1)))
                        & (UInt16_127))
                        << (UInt16_9)));
            let offset0_col3 = offset0_tmp_490d_3.as_m31();
            trace[3].data[row_index] = offset0_col3;
            let ap_update_add_1_tmp_490d_4 =
                (((((PackedUInt16::from_m31(memoryidtobig_value_tmp_490d_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memoryidtobig_value_tmp_490d_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_11))
                    & (UInt16_1));
            let ap_update_add_1_col4 = ap_update_add_1_tmp_490d_4.as_m31();
            trace[4].data[row_index] = ap_update_add_1_col4;

            sub_components_inputs.verify_instruction_inputs[0].extend(
                (
                    input_pc_col0,
                    [offset0_col3, M31_32767, M31_32769],
                    [
                        M31_0,
                        M31_1,
                        M31_1,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_1,
                        M31_0,
                        ap_update_add_1_col4,
                        M31_0,
                        M31_0,
                        M31_0,
                    ],
                )
                    .unpack(),
            );

            lookup_data.verifyinstruction[0].push([
                input_pc_col0,
                offset0_col3,
                M31_32767,
                M31_32769,
                M31_0,
                M31_1,
                M31_1,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_1,
                M31_0,
                ap_update_add_1_col4,
                M31_0,
                M31_0,
                M31_0,
            ]);

            // ReadPositive_num_bits_252.

            let memoryaddresstoid_value_tmp_490d_5 = memory_address_to_id_state
                .deduce_output(((input_ap_col1) + ((offset0_col3) - (M31_32768))));
            let memoryidtobig_value_tmp_490d_6 =
                memory_id_to_big_state.deduce_output(memoryaddresstoid_value_tmp_490d_5);
            let dst_id_col5 = memoryaddresstoid_value_tmp_490d_5;
            trace[5].data[row_index] = dst_id_col5;
            sub_components_inputs.memory_address_to_id_inputs[0]
                .extend(((input_ap_col1) + ((offset0_col3) - (M31_32768))).unpack());

            lookup_data.memoryaddresstoid[0].push([
                ((input_ap_col1) + ((offset0_col3) - (M31_32768))),
                dst_id_col5,
            ]);
            let dst_limb_0_col6 = memoryidtobig_value_tmp_490d_6.get_m31(0);
            trace[6].data[row_index] = dst_limb_0_col6;
            let dst_limb_1_col7 = memoryidtobig_value_tmp_490d_6.get_m31(1);
            trace[7].data[row_index] = dst_limb_1_col7;
            let dst_limb_2_col8 = memoryidtobig_value_tmp_490d_6.get_m31(2);
            trace[8].data[row_index] = dst_limb_2_col8;
            let dst_limb_3_col9 = memoryidtobig_value_tmp_490d_6.get_m31(3);
            trace[9].data[row_index] = dst_limb_3_col9;
            let dst_limb_4_col10 = memoryidtobig_value_tmp_490d_6.get_m31(4);
            trace[10].data[row_index] = dst_limb_4_col10;
            let dst_limb_5_col11 = memoryidtobig_value_tmp_490d_6.get_m31(5);
            trace[11].data[row_index] = dst_limb_5_col11;
            let dst_limb_6_col12 = memoryidtobig_value_tmp_490d_6.get_m31(6);
            trace[12].data[row_index] = dst_limb_6_col12;
            let dst_limb_7_col13 = memoryidtobig_value_tmp_490d_6.get_m31(7);
            trace[13].data[row_index] = dst_limb_7_col13;
            let dst_limb_8_col14 = memoryidtobig_value_tmp_490d_6.get_m31(8);
            trace[14].data[row_index] = dst_limb_8_col14;
            let dst_limb_9_col15 = memoryidtobig_value_tmp_490d_6.get_m31(9);
            trace[15].data[row_index] = dst_limb_9_col15;
            let dst_limb_10_col16 = memoryidtobig_value_tmp_490d_6.get_m31(10);
            trace[16].data[row_index] = dst_limb_10_col16;
            let dst_limb_11_col17 = memoryidtobig_value_tmp_490d_6.get_m31(11);
            trace[17].data[row_index] = dst_limb_11_col17;
            let dst_limb_12_col18 = memoryidtobig_value_tmp_490d_6.get_m31(12);
            trace[18].data[row_index] = dst_limb_12_col18;
            let dst_limb_13_col19 = memoryidtobig_value_tmp_490d_6.get_m31(13);
            trace[19].data[row_index] = dst_limb_13_col19;
            let dst_limb_14_col20 = memoryidtobig_value_tmp_490d_6.get_m31(14);
            trace[20].data[row_index] = dst_limb_14_col20;
            let dst_limb_15_col21 = memoryidtobig_value_tmp_490d_6.get_m31(15);
            trace[21].data[row_index] = dst_limb_15_col21;
            let dst_limb_16_col22 = memoryidtobig_value_tmp_490d_6.get_m31(16);
            trace[22].data[row_index] = dst_limb_16_col22;
            let dst_limb_17_col23 = memoryidtobig_value_tmp_490d_6.get_m31(17);
            trace[23].data[row_index] = dst_limb_17_col23;
            let dst_limb_18_col24 = memoryidtobig_value_tmp_490d_6.get_m31(18);
            trace[24].data[row_index] = dst_limb_18_col24;
            let dst_limb_19_col25 = memoryidtobig_value_tmp_490d_6.get_m31(19);
            trace[25].data[row_index] = dst_limb_19_col25;
            let dst_limb_20_col26 = memoryidtobig_value_tmp_490d_6.get_m31(20);
            trace[26].data[row_index] = dst_limb_20_col26;
            let dst_limb_21_col27 = memoryidtobig_value_tmp_490d_6.get_m31(21);
            trace[27].data[row_index] = dst_limb_21_col27;
            let dst_limb_22_col28 = memoryidtobig_value_tmp_490d_6.get_m31(22);
            trace[28].data[row_index] = dst_limb_22_col28;
            let dst_limb_23_col29 = memoryidtobig_value_tmp_490d_6.get_m31(23);
            trace[29].data[row_index] = dst_limb_23_col29;
            let dst_limb_24_col30 = memoryidtobig_value_tmp_490d_6.get_m31(24);
            trace[30].data[row_index] = dst_limb_24_col30;
            let dst_limb_25_col31 = memoryidtobig_value_tmp_490d_6.get_m31(25);
            trace[31].data[row_index] = dst_limb_25_col31;
            let dst_limb_26_col32 = memoryidtobig_value_tmp_490d_6.get_m31(26);
            trace[32].data[row_index] = dst_limb_26_col32;
            let dst_limb_27_col33 = memoryidtobig_value_tmp_490d_6.get_m31(27);
            trace[33].data[row_index] = dst_limb_27_col33;
            sub_components_inputs.memory_id_to_big_inputs[0].extend(dst_id_col5.unpack());

            lookup_data.memoryidtobig[0].push([
                dst_id_col5,
                dst_limb_0_col6,
                dst_limb_1_col7,
                dst_limb_2_col8,
                dst_limb_3_col9,
                dst_limb_4_col10,
                dst_limb_5_col11,
                dst_limb_6_col12,
                dst_limb_7_col13,
                dst_limb_8_col14,
                dst_limb_9_col15,
                dst_limb_10_col16,
                dst_limb_11_col17,
                dst_limb_12_col18,
                dst_limb_13_col19,
                dst_limb_14_col20,
                dst_limb_15_col21,
                dst_limb_16_col22,
                dst_limb_17_col23,
                dst_limb_18_col24,
                dst_limb_19_col25,
                dst_limb_20_col26,
                dst_limb_21_col27,
                dst_limb_22_col28,
                dst_limb_23_col29,
                dst_limb_24_col30,
                dst_limb_25_col31,
                dst_limb_26_col32,
                dst_limb_27_col33,
            ]);

            let res_col34 = ((M31_1).div(
                (((((((((((((((((((((((((((((M31_0) + (dst_limb_0_col6))
                    + (dst_limb_1_col7))
                    + (dst_limb_2_col8))
                    + (dst_limb_3_col9))
                    + (dst_limb_4_col10))
                    + (dst_limb_5_col11))
                    + (dst_limb_6_col12))
                    + (dst_limb_7_col13))
                    + (dst_limb_8_col14))
                    + (dst_limb_9_col15))
                    + (dst_limb_10_col16))
                    + (dst_limb_11_col17))
                    + (dst_limb_12_col18))
                    + (dst_limb_13_col19))
                    + (dst_limb_14_col20))
                    + (dst_limb_15_col21))
                    + (dst_limb_16_col22))
                    + (dst_limb_17_col23))
                    + (dst_limb_18_col24))
                    + (dst_limb_19_col25))
                    + (dst_limb_20_col26))
                    + (dst_limb_21_col27))
                    + (dst_limb_22_col28))
                    + (dst_limb_23_col29))
                    + (dst_limb_24_col30))
                    + (dst_limb_25_col31))
                    + (dst_limb_26_col32))
                    + (dst_limb_27_col33)),
            ));
            trace[34].data[row_index] = res_col34;
            let diff_from_p_tmp_490d_7 = ((dst_limb_0_col6) - (M31_1));
            let diff_from_p_tmp_490d_8 = ((dst_limb_21_col27) - (M31_136));
            let diff_from_p_tmp_490d_9 = ((dst_limb_27_col33) - (M31_256));
            let res_squares_col35 = ((M31_1).div(
                (((((((((((((((((((((((((((((M31_0)
                    + ((diff_from_p_tmp_490d_7)
                        * (diff_from_p_tmp_490d_7)))
                    + (dst_limb_1_col7))
                    + (dst_limb_2_col8))
                    + (dst_limb_3_col9))
                    + (dst_limb_4_col10))
                    + (dst_limb_5_col11))
                    + (dst_limb_6_col12))
                    + (dst_limb_7_col13))
                    + (dst_limb_8_col14))
                    + (dst_limb_9_col15))
                    + (dst_limb_10_col16))
                    + (dst_limb_11_col17))
                    + (dst_limb_12_col18))
                    + (dst_limb_13_col19))
                    + (dst_limb_14_col20))
                    + (dst_limb_15_col21))
                    + (dst_limb_16_col22))
                    + (dst_limb_17_col23))
                    + (dst_limb_18_col24))
                    + (dst_limb_19_col25))
                    + (dst_limb_20_col26))
                    + ((diff_from_p_tmp_490d_8) * (diff_from_p_tmp_490d_8)))
                    + (dst_limb_22_col28))
                    + (dst_limb_23_col29))
                    + (dst_limb_24_col30))
                    + (dst_limb_25_col31))
                    + (dst_limb_26_col32))
                    + ((diff_from_p_tmp_490d_9) * (diff_from_p_tmp_490d_9))),
            ));
            trace[35].data[row_index] = res_squares_col35;

            // ReadSmall.

            let memoryaddresstoid_value_tmp_490d_10 =
                memory_address_to_id_state.deduce_output(((input_pc_col0) + (M31_1)));
            let memoryidtobig_value_tmp_490d_11 =
                memory_id_to_big_state.deduce_output(memoryaddresstoid_value_tmp_490d_10);
            let next_pc_id_col36 = memoryaddresstoid_value_tmp_490d_10;
            trace[36].data[row_index] = next_pc_id_col36;
            sub_components_inputs.memory_address_to_id_inputs[1]
                .extend(((input_pc_col0) + (M31_1)).unpack());

            lookup_data.memoryaddresstoid[1].push([((input_pc_col0) + (M31_1)), next_pc_id_col36]);

            // CondDecodeSmallSign.

            let msb_tmp_490d_12 = memoryidtobig_value_tmp_490d_11.get_m31(27).eq(M31_256);
            let msb_col37 = msb_tmp_490d_12.as_m31();
            trace[37].data[row_index] = msb_col37;
            let mid_limbs_set_tmp_490d_13 = memoryidtobig_value_tmp_490d_11.get_m31(20).eq(M31_511);
            let mid_limbs_set_col38 = mid_limbs_set_tmp_490d_13.as_m31();
            trace[38].data[row_index] = mid_limbs_set_col38;

            let next_pc_limb_0_col39 = memoryidtobig_value_tmp_490d_11.get_m31(0);
            trace[39].data[row_index] = next_pc_limb_0_col39;
            let next_pc_limb_1_col40 = memoryidtobig_value_tmp_490d_11.get_m31(1);
            trace[40].data[row_index] = next_pc_limb_1_col40;
            let next_pc_limb_2_col41 = memoryidtobig_value_tmp_490d_11.get_m31(2);
            trace[41].data[row_index] = next_pc_limb_2_col41;
            sub_components_inputs.memory_id_to_big_inputs[1].extend(next_pc_id_col36.unpack());

            lookup_data.memoryidtobig[1].push([
                next_pc_id_col36,
                next_pc_limb_0_col39,
                next_pc_limb_1_col40,
                next_pc_limb_2_col41,
                ((mid_limbs_set_col38) * (M31_511)),
                ((mid_limbs_set_col38) * (M31_511)),
                ((mid_limbs_set_col38) * (M31_511)),
                ((mid_limbs_set_col38) * (M31_511)),
                ((mid_limbs_set_col38) * (M31_511)),
                ((mid_limbs_set_col38) * (M31_511)),
                ((mid_limbs_set_col38) * (M31_511)),
                ((mid_limbs_set_col38) * (M31_511)),
                ((mid_limbs_set_col38) * (M31_511)),
                ((mid_limbs_set_col38) * (M31_511)),
                ((mid_limbs_set_col38) * (M31_511)),
                ((mid_limbs_set_col38) * (M31_511)),
                ((mid_limbs_set_col38) * (M31_511)),
                ((mid_limbs_set_col38) * (M31_511)),
                ((mid_limbs_set_col38) * (M31_511)),
                ((mid_limbs_set_col38) * (M31_511)),
                ((mid_limbs_set_col38) * (M31_511)),
                ((mid_limbs_set_col38) * (M31_511)),
                (((M31_136) * (msb_col37)) - (mid_limbs_set_col38)),
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                ((msb_col37) * (M31_256)),
            ]);

            lookup_data.opcodes[0].push([input_pc_col0, input_ap_col1, input_fp_col2]);
            lookup_data.opcodes[1].push([
                ((input_pc_col0)
                    + (((((next_pc_limb_0_col39) + ((next_pc_limb_1_col40) * (M31_512)))
                        + ((next_pc_limb_2_col41) * (M31_262144)))
                        - (msb_col37))
                        - ((M31_134217728) * (mid_limbs_set_col38)))),
                ((input_ap_col1) + (ap_update_add_1_col4)),
                input_fp_col2,
            ]);
        },
    );

    (trace, sub_components_inputs, lookup_data)
}

pub struct LookupData {
    pub memoryaddresstoid: [Vec<[PackedM31; 2]>; 2],
    pub memoryidtobig: [Vec<[PackedM31; 29]>; 2],
    pub opcodes: [Vec<[PackedM31; 3]>; 2],
    pub verifyinstruction: [Vec<[PackedM31; 19]>; 1],
}
impl LookupData {
    #[allow(unused_variables)]
    fn with_capacity(capacity: usize) -> Self {
        Self {
            memoryaddresstoid: [Vec::with_capacity(capacity), Vec::with_capacity(capacity)],
            memoryidtobig: [Vec::with_capacity(capacity), Vec::with_capacity(capacity)],
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
