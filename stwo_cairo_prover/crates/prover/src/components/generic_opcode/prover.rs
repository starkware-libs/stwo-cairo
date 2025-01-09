#![cfg_attr(rustfmt, rustfmt_skip)]#![allow(unused_parens)]
#![allow(unused_imports)]
use std::iter::zip;

use air_structs_derive::SubComponentInputs;
use itertools::{chain, zip_eq, Itertools};
use num_traits::{One, Zero};
use prover_types::cpu::*;
use prover_types::simd::*;
use rayon::iter::ParallelIterator;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator};
use stwo_air_utils_derive::{Uninitialized, IterMut, ParIterMut};
use stwo_air_utils::trace::component_trace::ComponentTrace;
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::constraint_framework::Relation;
use stwo_prover::core::air::Component;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::backend::BackendForChannel;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::conversion::Unpack;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::{Channel, MerkleChannel};
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::utils::bit_reverse_coset_to_circle_domain_order;

use super::component::{Claim, InteractionClaim};
use crate::components::pack_values;
use crate::relations;
use crate::components::memory_address_to_id;use crate::components::memory_id_to_big;use crate::components::verify_instruction;use crate::components::range_check_9_9;use crate::components::range_check_19;

pub type InputType = CasmState;
pub type PackedInputType = PackedCasmState;
const N_TRACE_COLUMNS: usize = 229;

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
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,range_check_19_state: &range_check_19::ClaimGenerator,range_check_9_9_state: &range_check_9_9::ClaimGenerator,verify_instruction_state: &mut verify_instruction::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>
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
        let (trace, mut sub_components_inputs, lookup_data) =
            write_trace_simd(packed_inputs, memory_address_to_id_state,memory_id_to_big_state,);

        if need_padding {
            sub_components_inputs.bit_reverse_coset_to_circle_domain_order();
        }
        sub_components_inputs.memory_address_to_id_inputs.iter().for_each(|inputs| {
            memory_address_to_id_state.add_inputs(&inputs[..n_calls]);
        });sub_components_inputs.memory_id_to_big_inputs.iter().for_each(|inputs| {
            memory_id_to_big_state.add_inputs(&inputs[..n_calls]);
        });sub_components_inputs.range_check_19_inputs.iter().for_each(|inputs| {
            range_check_19_state.add_inputs(&inputs[..n_calls]);
        });sub_components_inputs.range_check_9_9_inputs.iter().for_each(|inputs| {
            range_check_9_9_state.add_inputs(&inputs[..n_calls]);
        });sub_components_inputs.verify_instruction_inputs.iter().for_each(|inputs| {
            verify_instruction_state.add_inputs(&inputs[..n_calls]);
        });

        tree_builder.extend_evals(trace.to_evals());

        (
        Claim {
            n_calls
        },
        InteractionClaimGenerator {
            n_calls,
            lookup_data,
        },
        )
    }

    pub fn add_inputs(
        &mut self,
        inputs: &[InputType],
    ) {
        self.inputs.extend(inputs);
    }
}

#[derive(SubComponentInputs,Uninitialized,IterMut, ParIterMut)]
pub struct SubComponentInputs
{pub memory_address_to_id_inputs: [Vec<memory_address_to_id::InputType>; 3],pub memory_id_to_big_inputs: [Vec<memory_id_to_big::InputType>; 3],pub range_check_19_inputs: [Vec<range_check_19::InputType>; 28],pub range_check_9_9_inputs: [Vec<range_check_9_9::InputType>; 28],pub verify_instruction_inputs: [Vec<verify_instruction::InputType>; 1],}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
) -> (ComponentTrace<N_TRACE_COLUMNS>,
    SubComponentInputs,
    LookupData) {
        let log_n_packed_rows = inputs.len().ilog2();
        let log_size = log_n_packed_rows + LOG_N_LANES;
        let (mut trace, mut lookup_data, mut sub_components_inputs) = unsafe {
            (
                ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
                LookupData::uninitialized(log_n_packed_rows),
                SubComponentInputs::uninitialized(log_size),
            )
        };

    let M31_0 = PackedM31::broadcast(M31::from(0));let M31_1 = PackedM31::broadcast(M31::from(1));let M31_131072 = PackedM31::broadcast(M31::from(131072));let M31_134217728 = PackedM31::broadcast(M31::from(134217728));let M31_136 = PackedM31::broadcast(M31::from(136));let M31_2 = PackedM31::broadcast(M31::from(2));let M31_256 = PackedM31::broadcast(M31::from(256));let M31_262144 = PackedM31::broadcast(M31::from(262144));let M31_32 = PackedM31::broadcast(M31::from(32));let M31_32768 = PackedM31::broadcast(M31::from(32768));let M31_4 = PackedM31::broadcast(M31::from(4));let M31_4194304 = PackedM31::broadcast(M31::from(4194304));let M31_511 = PackedM31::broadcast(M31::from(511));let M31_512 = PackedM31::broadcast(M31::from(512));let M31_64 = PackedM31::broadcast(M31::from(64));let M31_65536 = PackedM31::broadcast(M31::from(65536));let M31_8 = PackedM31::broadcast(M31::from(8));let UInt16_0 = PackedUInt16::broadcast(UInt16::from(0));let UInt16_1 = PackedUInt16::broadcast(UInt16::from(1));let UInt16_10 = PackedUInt16::broadcast(UInt16::from(10));let UInt16_11 = PackedUInt16::broadcast(UInt16::from(11));let UInt16_12 = PackedUInt16::broadcast(UInt16::from(12));let UInt16_127 = PackedUInt16::broadcast(UInt16::from(127));let UInt16_13 = PackedUInt16::broadcast(UInt16::from(13));let UInt16_14 = PackedUInt16::broadcast(UInt16::from(14));let UInt16_2 = PackedUInt16::broadcast(UInt16::from(2));let UInt16_3 = PackedUInt16::broadcast(UInt16::from(3));let UInt16_31 = PackedUInt16::broadcast(UInt16::from(31));let UInt16_4 = PackedUInt16::broadcast(UInt16::from(4));let UInt16_5 = PackedUInt16::broadcast(UInt16::from(5));let UInt16_6 = PackedUInt16::broadcast(UInt16::from(6));let UInt16_7 = PackedUInt16::broadcast(UInt16::from(7));let UInt16_8 = PackedUInt16::broadcast(UInt16::from(8));let UInt16_9 = PackedUInt16::broadcast(UInt16::from(9));let UInt32_262143 = PackedUInt32::broadcast(UInt32::from(262143));let UInt32_511 = PackedUInt32::broadcast(UInt32::from(511));let UInt32_65536 = PackedUInt32::broadcast(UInt32::from(65536));let UInt32_9 = PackedUInt32::broadcast(UInt32::from(9));

    trace
    .par_iter_mut()
    .zip(inputs.par_iter())
    .zip(lookup_data.par_iter_mut())
    .zip(sub_components_inputs.par_iter_mut().chunks(N_LANES))
    .for_each(
        |(((row, generic_opcode_input), lookup_data), mut sub_components_inputs)| {
            let input_tmp_57455_0 = generic_opcode_input;
let input_pc_col0 = input_tmp_57455_0.pc;
            *row[0] = input_pc_col0;
let input_ap_col1 = input_tmp_57455_0.ap;
            *row[1] = input_ap_col1;
let input_fp_col2 = input_tmp_57455_0.fp;
            *row[2] = input_fp_col2;


            //Decode Generic Instruction.

            


            //Decode Instruction.

            
let memory_address_to_id_value_tmp_57455_1 = memory_address_to_id_state.deduce_output(
                input_pc_col0
            );
let memory_id_to_big_value_tmp_57455_2 = memory_id_to_big_state.deduce_output(
                memory_address_to_id_value_tmp_57455_1
            );
let offset0_tmp_57455_3 = ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(0))) + (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(1))) & (UInt16_127))) << (UInt16_9))));
let offset0_col3 = offset0_tmp_57455_3.as_m31();
            *row[3] = offset0_col3;
let offset1_tmp_57455_4 = ((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(1))) >> (UInt16_7))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(2))) << (UInt16_2))))) + (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(3))) & (UInt16_31))) << (UInt16_11))));
let offset1_col4 = offset1_tmp_57455_4.as_m31();
            *row[4] = offset1_col4;
let offset2_tmp_57455_5 = ((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(3))) >> (UInt16_5))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(4))) << (UInt16_4))))) + (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(5))) & (UInt16_7))) << (UInt16_13))));
let offset2_col5 = offset2_tmp_57455_5.as_m31();
            *row[5] = offset2_col5;
let dst_base_fp_tmp_57455_6 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_0))) & (UInt16_1));
let dst_base_fp_col6 = dst_base_fp_tmp_57455_6.as_m31();
            *row[6] = dst_base_fp_col6;
let op0_base_fp_tmp_57455_7 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_1))) & (UInt16_1));
let op0_base_fp_col7 = op0_base_fp_tmp_57455_7.as_m31();
            *row[7] = op0_base_fp_col7;
let op1_imm_tmp_57455_8 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_2))) & (UInt16_1));
let op1_imm_col8 = op1_imm_tmp_57455_8.as_m31();
            *row[8] = op1_imm_col8;
let op1_base_fp_tmp_57455_9 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_3))) & (UInt16_1));
let op1_base_fp_col9 = op1_base_fp_tmp_57455_9.as_m31();
            *row[9] = op1_base_fp_col9;
let op1_base_ap_tmp_57455_10 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_4))) & (UInt16_1));
let op1_base_ap_col10 = op1_base_ap_tmp_57455_10.as_m31();
            *row[10] = op1_base_ap_col10;
let res_add_tmp_57455_11 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_5))) & (UInt16_1));
let res_add_col11 = res_add_tmp_57455_11.as_m31();
            *row[11] = res_add_col11;
let res_mul_tmp_57455_12 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_6))) & (UInt16_1));
let res_mul_col12 = res_mul_tmp_57455_12.as_m31();
            *row[12] = res_mul_col12;
let pc_update_jump_tmp_57455_13 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_7))) & (UInt16_1));
let pc_update_jump_col13 = pc_update_jump_tmp_57455_13.as_m31();
            *row[13] = pc_update_jump_col13;
let pc_update_jump_rel_tmp_57455_14 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_8))) & (UInt16_1));
let pc_update_jump_rel_col14 = pc_update_jump_rel_tmp_57455_14.as_m31();
            *row[14] = pc_update_jump_rel_col14;
let pc_update_jnz_tmp_57455_15 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_9))) & (UInt16_1));
let pc_update_jnz_col15 = pc_update_jnz_tmp_57455_15.as_m31();
            *row[15] = pc_update_jnz_col15;
let ap_update_add_tmp_57455_16 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_10))) & (UInt16_1));
let ap_update_add_col16 = ap_update_add_tmp_57455_16.as_m31();
            *row[16] = ap_update_add_col16;
let ap_update_add_1_tmp_57455_17 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_11))) & (UInt16_1));
let ap_update_add_1_col17 = ap_update_add_1_tmp_57455_17.as_m31();
            *row[17] = ap_update_add_1_col17;
let opcode_call_tmp_57455_18 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_12))) & (UInt16_1));
let opcode_call_col18 = opcode_call_tmp_57455_18.as_m31();
            *row[18] = opcode_call_col18;
let opcode_ret_tmp_57455_19 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_13))) & (UInt16_1));
let opcode_ret_col19 = opcode_ret_tmp_57455_19.as_m31();
            *row[19] = opcode_ret_col19;
let opcode_assert_eq_tmp_57455_20 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_14))) & (UInt16_1));
let opcode_assert_eq_col20 = opcode_assert_eq_tmp_57455_20.as_m31();
            *row[20] = opcode_assert_eq_col20;

for (i, &input) in (input_pc_col0, [offset0_col3, offset1_col4, offset2_col5], [dst_base_fp_col6, op0_base_fp_col7, op1_imm_col8, op1_base_fp_col9, op1_base_ap_col10, res_add_col11, res_mul_col12, pc_update_jump_col13, pc_update_jump_rel_col14, pc_update_jnz_col15, ap_update_add_col16, ap_update_add_1_col17, opcode_call_col18, opcode_ret_col19, opcode_assert_eq_col20]).unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .verify_instruction_inputs[0] = input;
            }
*lookup_data.verify_instruction_0 = [input_pc_col0, offset0_col3, offset1_col4, offset2_col5, dst_base_fp_col6, op0_base_fp_col7, op1_imm_col8, op1_base_fp_col9, op1_base_ap_col10, res_add_col11, res_mul_col12, pc_update_jump_col13, pc_update_jump_rel_col14, pc_update_jnz_col15, ap_update_add_col16, ap_update_add_1_col17, opcode_call_col18, opcode_ret_col19, opcode_assert_eq_col20];


            
let op1_base_op0_tmp_57455_21 = ((((((M31_1) - (op1_imm_col8))) - (op1_base_fp_col9))) - (op1_base_ap_col10));
let res_op1_tmp_57455_22 = ((((((M31_1) - (res_add_col11))) - (res_mul_col12))) - (pc_update_jnz_col15));
let pc_update_regular_tmp_57455_23 = ((((((M31_1) - (pc_update_jump_col13))) - (pc_update_jump_rel_col14))) - (pc_update_jnz_col15));
let ap_update_regular_tmp_57455_24 = ((((((M31_1) - (ap_update_add_col16))) - (ap_update_add_1_col17))) - (opcode_call_col18));
let fp_update_regular_tmp_57455_25 = ((((M31_1) - (opcode_call_col18))) - (opcode_ret_col19));


            


            //Eval Operands.

            


            //Read Positive Num Bits 252.

            
let memory_address_to_id_value_tmp_57455_26 = memory_address_to_id_state.deduce_output(
                ((((((dst_base_fp_col6) * (input_fp_col2))) + (((((M31_1) - (dst_base_fp_col6))) * (input_ap_col1))))) + (((offset0_col3) - (M31_32768))))
            );
let memory_id_to_big_value_tmp_57455_27 = memory_id_to_big_state.deduce_output(
                memory_address_to_id_value_tmp_57455_26
            );
let dst_id_col21 = memory_address_to_id_value_tmp_57455_26;
            *row[21] = dst_id_col21;
for (i, &input) in ((((((dst_base_fp_col6) * (input_fp_col2))) + (((((M31_1) - (dst_base_fp_col6))) * (input_ap_col1))))) + (((offset0_col3) - (M31_32768)))).unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .memory_address_to_id_inputs[0] = input;
            }
*lookup_data.memory_address_to_id_0 = [((((((dst_base_fp_col6) * (input_fp_col2))) + (((((M31_1) - (dst_base_fp_col6))) * (input_ap_col1))))) + (((offset0_col3) - (M31_32768)))), dst_id_col21];
let dst_limb_0_col22 = memory_id_to_big_value_tmp_57455_27.get_m31(0);
            *row[22] = dst_limb_0_col22;
let dst_limb_1_col23 = memory_id_to_big_value_tmp_57455_27.get_m31(1);
            *row[23] = dst_limb_1_col23;
let dst_limb_2_col24 = memory_id_to_big_value_tmp_57455_27.get_m31(2);
            *row[24] = dst_limb_2_col24;
let dst_limb_3_col25 = memory_id_to_big_value_tmp_57455_27.get_m31(3);
            *row[25] = dst_limb_3_col25;
let dst_limb_4_col26 = memory_id_to_big_value_tmp_57455_27.get_m31(4);
            *row[26] = dst_limb_4_col26;
let dst_limb_5_col27 = memory_id_to_big_value_tmp_57455_27.get_m31(5);
            *row[27] = dst_limb_5_col27;
let dst_limb_6_col28 = memory_id_to_big_value_tmp_57455_27.get_m31(6);
            *row[28] = dst_limb_6_col28;
let dst_limb_7_col29 = memory_id_to_big_value_tmp_57455_27.get_m31(7);
            *row[29] = dst_limb_7_col29;
let dst_limb_8_col30 = memory_id_to_big_value_tmp_57455_27.get_m31(8);
            *row[30] = dst_limb_8_col30;
let dst_limb_9_col31 = memory_id_to_big_value_tmp_57455_27.get_m31(9);
            *row[31] = dst_limb_9_col31;
let dst_limb_10_col32 = memory_id_to_big_value_tmp_57455_27.get_m31(10);
            *row[32] = dst_limb_10_col32;
let dst_limb_11_col33 = memory_id_to_big_value_tmp_57455_27.get_m31(11);
            *row[33] = dst_limb_11_col33;
let dst_limb_12_col34 = memory_id_to_big_value_tmp_57455_27.get_m31(12);
            *row[34] = dst_limb_12_col34;
let dst_limb_13_col35 = memory_id_to_big_value_tmp_57455_27.get_m31(13);
            *row[35] = dst_limb_13_col35;
let dst_limb_14_col36 = memory_id_to_big_value_tmp_57455_27.get_m31(14);
            *row[36] = dst_limb_14_col36;
let dst_limb_15_col37 = memory_id_to_big_value_tmp_57455_27.get_m31(15);
            *row[37] = dst_limb_15_col37;
let dst_limb_16_col38 = memory_id_to_big_value_tmp_57455_27.get_m31(16);
            *row[38] = dst_limb_16_col38;
let dst_limb_17_col39 = memory_id_to_big_value_tmp_57455_27.get_m31(17);
            *row[39] = dst_limb_17_col39;
let dst_limb_18_col40 = memory_id_to_big_value_tmp_57455_27.get_m31(18);
            *row[40] = dst_limb_18_col40;
let dst_limb_19_col41 = memory_id_to_big_value_tmp_57455_27.get_m31(19);
            *row[41] = dst_limb_19_col41;
let dst_limb_20_col42 = memory_id_to_big_value_tmp_57455_27.get_m31(20);
            *row[42] = dst_limb_20_col42;
let dst_limb_21_col43 = memory_id_to_big_value_tmp_57455_27.get_m31(21);
            *row[43] = dst_limb_21_col43;
let dst_limb_22_col44 = memory_id_to_big_value_tmp_57455_27.get_m31(22);
            *row[44] = dst_limb_22_col44;
let dst_limb_23_col45 = memory_id_to_big_value_tmp_57455_27.get_m31(23);
            *row[45] = dst_limb_23_col45;
let dst_limb_24_col46 = memory_id_to_big_value_tmp_57455_27.get_m31(24);
            *row[46] = dst_limb_24_col46;
let dst_limb_25_col47 = memory_id_to_big_value_tmp_57455_27.get_m31(25);
            *row[47] = dst_limb_25_col47;
let dst_limb_26_col48 = memory_id_to_big_value_tmp_57455_27.get_m31(26);
            *row[48] = dst_limb_26_col48;
let dst_limb_27_col49 = memory_id_to_big_value_tmp_57455_27.get_m31(27);
            *row[49] = dst_limb_27_col49;
for (i, &input) in dst_id_col21.unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .memory_id_to_big_inputs[0] = input;
            }
*lookup_data.memory_id_to_big_0 = [dst_id_col21, dst_limb_0_col22, dst_limb_1_col23, dst_limb_2_col24, dst_limb_3_col25, dst_limb_4_col26, dst_limb_5_col27, dst_limb_6_col28, dst_limb_7_col29, dst_limb_8_col30, dst_limb_9_col31, dst_limb_10_col32, dst_limb_11_col33, dst_limb_12_col34, dst_limb_13_col35, dst_limb_14_col36, dst_limb_15_col37, dst_limb_16_col38, dst_limb_17_col39, dst_limb_18_col40, dst_limb_19_col41, dst_limb_20_col42, dst_limb_21_col43, dst_limb_22_col44, dst_limb_23_col45, dst_limb_24_col46, dst_limb_25_col47, dst_limb_26_col48, dst_limb_27_col49];


            


            //Read Positive Num Bits 252.

            
let memory_address_to_id_value_tmp_57455_28 = memory_address_to_id_state.deduce_output(
                ((((((op0_base_fp_col7) * (input_fp_col2))) + (((((M31_1) - (op0_base_fp_col7))) * (input_ap_col1))))) + (((offset1_col4) - (M31_32768))))
            );
let memory_id_to_big_value_tmp_57455_29 = memory_id_to_big_state.deduce_output(
                memory_address_to_id_value_tmp_57455_28
            );
let op0_id_col50 = memory_address_to_id_value_tmp_57455_28;
            *row[50] = op0_id_col50;
for (i, &input) in ((((((op0_base_fp_col7) * (input_fp_col2))) + (((((M31_1) - (op0_base_fp_col7))) * (input_ap_col1))))) + (((offset1_col4) - (M31_32768)))).unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .memory_address_to_id_inputs[1] = input;
            }
*lookup_data.memory_address_to_id_1 = [((((((op0_base_fp_col7) * (input_fp_col2))) + (((((M31_1) - (op0_base_fp_col7))) * (input_ap_col1))))) + (((offset1_col4) - (M31_32768)))), op0_id_col50];
let op0_limb_0_col51 = memory_id_to_big_value_tmp_57455_29.get_m31(0);
            *row[51] = op0_limb_0_col51;
let op0_limb_1_col52 = memory_id_to_big_value_tmp_57455_29.get_m31(1);
            *row[52] = op0_limb_1_col52;
let op0_limb_2_col53 = memory_id_to_big_value_tmp_57455_29.get_m31(2);
            *row[53] = op0_limb_2_col53;
let op0_limb_3_col54 = memory_id_to_big_value_tmp_57455_29.get_m31(3);
            *row[54] = op0_limb_3_col54;
let op0_limb_4_col55 = memory_id_to_big_value_tmp_57455_29.get_m31(4);
            *row[55] = op0_limb_4_col55;
let op0_limb_5_col56 = memory_id_to_big_value_tmp_57455_29.get_m31(5);
            *row[56] = op0_limb_5_col56;
let op0_limb_6_col57 = memory_id_to_big_value_tmp_57455_29.get_m31(6);
            *row[57] = op0_limb_6_col57;
let op0_limb_7_col58 = memory_id_to_big_value_tmp_57455_29.get_m31(7);
            *row[58] = op0_limb_7_col58;
let op0_limb_8_col59 = memory_id_to_big_value_tmp_57455_29.get_m31(8);
            *row[59] = op0_limb_8_col59;
let op0_limb_9_col60 = memory_id_to_big_value_tmp_57455_29.get_m31(9);
            *row[60] = op0_limb_9_col60;
let op0_limb_10_col61 = memory_id_to_big_value_tmp_57455_29.get_m31(10);
            *row[61] = op0_limb_10_col61;
let op0_limb_11_col62 = memory_id_to_big_value_tmp_57455_29.get_m31(11);
            *row[62] = op0_limb_11_col62;
let op0_limb_12_col63 = memory_id_to_big_value_tmp_57455_29.get_m31(12);
            *row[63] = op0_limb_12_col63;
let op0_limb_13_col64 = memory_id_to_big_value_tmp_57455_29.get_m31(13);
            *row[64] = op0_limb_13_col64;
let op0_limb_14_col65 = memory_id_to_big_value_tmp_57455_29.get_m31(14);
            *row[65] = op0_limb_14_col65;
let op0_limb_15_col66 = memory_id_to_big_value_tmp_57455_29.get_m31(15);
            *row[66] = op0_limb_15_col66;
let op0_limb_16_col67 = memory_id_to_big_value_tmp_57455_29.get_m31(16);
            *row[67] = op0_limb_16_col67;
let op0_limb_17_col68 = memory_id_to_big_value_tmp_57455_29.get_m31(17);
            *row[68] = op0_limb_17_col68;
let op0_limb_18_col69 = memory_id_to_big_value_tmp_57455_29.get_m31(18);
            *row[69] = op0_limb_18_col69;
let op0_limb_19_col70 = memory_id_to_big_value_tmp_57455_29.get_m31(19);
            *row[70] = op0_limb_19_col70;
let op0_limb_20_col71 = memory_id_to_big_value_tmp_57455_29.get_m31(20);
            *row[71] = op0_limb_20_col71;
let op0_limb_21_col72 = memory_id_to_big_value_tmp_57455_29.get_m31(21);
            *row[72] = op0_limb_21_col72;
let op0_limb_22_col73 = memory_id_to_big_value_tmp_57455_29.get_m31(22);
            *row[73] = op0_limb_22_col73;
let op0_limb_23_col74 = memory_id_to_big_value_tmp_57455_29.get_m31(23);
            *row[74] = op0_limb_23_col74;
let op0_limb_24_col75 = memory_id_to_big_value_tmp_57455_29.get_m31(24);
            *row[75] = op0_limb_24_col75;
let op0_limb_25_col76 = memory_id_to_big_value_tmp_57455_29.get_m31(25);
            *row[76] = op0_limb_25_col76;
let op0_limb_26_col77 = memory_id_to_big_value_tmp_57455_29.get_m31(26);
            *row[77] = op0_limb_26_col77;
let op0_limb_27_col78 = memory_id_to_big_value_tmp_57455_29.get_m31(27);
            *row[78] = op0_limb_27_col78;
for (i, &input) in op0_id_col50.unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .memory_id_to_big_inputs[1] = input;
            }
*lookup_data.memory_id_to_big_1 = [op0_id_col50, op0_limb_0_col51, op0_limb_1_col52, op0_limb_2_col53, op0_limb_3_col54, op0_limb_4_col55, op0_limb_5_col56, op0_limb_6_col57, op0_limb_7_col58, op0_limb_8_col59, op0_limb_9_col60, op0_limb_10_col61, op0_limb_11_col62, op0_limb_12_col63, op0_limb_13_col64, op0_limb_14_col65, op0_limb_15_col66, op0_limb_16_col67, op0_limb_17_col68, op0_limb_18_col69, op0_limb_19_col70, op0_limb_20_col71, op0_limb_21_col72, op0_limb_22_col73, op0_limb_23_col74, op0_limb_24_col75, op0_limb_25_col76, op0_limb_26_col77, op0_limb_27_col78];


            


            //Read Positive Num Bits 252.

            
let memory_address_to_id_value_tmp_57455_30 = memory_address_to_id_state.deduce_output(
                ((((((((((op1_base_fp_col9) * (input_fp_col2))) + (((op1_base_ap_col10) * (input_ap_col1))))) + (((op1_imm_col8) * (input_pc_col0))))) + (((op1_base_op0_tmp_57455_21) * (((((op0_limb_0_col51) + (((op0_limb_1_col52) * (M31_512))))) + (((op0_limb_2_col53) * (M31_262144))))))))) + (((offset2_col5) - (M31_32768))))
            );
let memory_id_to_big_value_tmp_57455_31 = memory_id_to_big_state.deduce_output(
                memory_address_to_id_value_tmp_57455_30
            );
let op1_id_col79 = memory_address_to_id_value_tmp_57455_30;
            *row[79] = op1_id_col79;
for (i, &input) in ((((((((((op1_base_fp_col9) * (input_fp_col2))) + (((op1_base_ap_col10) * (input_ap_col1))))) + (((op1_imm_col8) * (input_pc_col0))))) + (((op1_base_op0_tmp_57455_21) * (((((op0_limb_0_col51) + (((op0_limb_1_col52) * (M31_512))))) + (((op0_limb_2_col53) * (M31_262144))))))))) + (((offset2_col5) - (M31_32768)))).unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .memory_address_to_id_inputs[2] = input;
            }
*lookup_data.memory_address_to_id_2 = [((((((((((op1_base_fp_col9) * (input_fp_col2))) + (((op1_base_ap_col10) * (input_ap_col1))))) + (((op1_imm_col8) * (input_pc_col0))))) + (((op1_base_op0_tmp_57455_21) * (((((op0_limb_0_col51) + (((op0_limb_1_col52) * (M31_512))))) + (((op0_limb_2_col53) * (M31_262144))))))))) + (((offset2_col5) - (M31_32768)))), op1_id_col79];
let op1_limb_0_col80 = memory_id_to_big_value_tmp_57455_31.get_m31(0);
            *row[80] = op1_limb_0_col80;
let op1_limb_1_col81 = memory_id_to_big_value_tmp_57455_31.get_m31(1);
            *row[81] = op1_limb_1_col81;
let op1_limb_2_col82 = memory_id_to_big_value_tmp_57455_31.get_m31(2);
            *row[82] = op1_limb_2_col82;
let op1_limb_3_col83 = memory_id_to_big_value_tmp_57455_31.get_m31(3);
            *row[83] = op1_limb_3_col83;
let op1_limb_4_col84 = memory_id_to_big_value_tmp_57455_31.get_m31(4);
            *row[84] = op1_limb_4_col84;
let op1_limb_5_col85 = memory_id_to_big_value_tmp_57455_31.get_m31(5);
            *row[85] = op1_limb_5_col85;
let op1_limb_6_col86 = memory_id_to_big_value_tmp_57455_31.get_m31(6);
            *row[86] = op1_limb_6_col86;
let op1_limb_7_col87 = memory_id_to_big_value_tmp_57455_31.get_m31(7);
            *row[87] = op1_limb_7_col87;
let op1_limb_8_col88 = memory_id_to_big_value_tmp_57455_31.get_m31(8);
            *row[88] = op1_limb_8_col88;
let op1_limb_9_col89 = memory_id_to_big_value_tmp_57455_31.get_m31(9);
            *row[89] = op1_limb_9_col89;
let op1_limb_10_col90 = memory_id_to_big_value_tmp_57455_31.get_m31(10);
            *row[90] = op1_limb_10_col90;
let op1_limb_11_col91 = memory_id_to_big_value_tmp_57455_31.get_m31(11);
            *row[91] = op1_limb_11_col91;
let op1_limb_12_col92 = memory_id_to_big_value_tmp_57455_31.get_m31(12);
            *row[92] = op1_limb_12_col92;
let op1_limb_13_col93 = memory_id_to_big_value_tmp_57455_31.get_m31(13);
            *row[93] = op1_limb_13_col93;
let op1_limb_14_col94 = memory_id_to_big_value_tmp_57455_31.get_m31(14);
            *row[94] = op1_limb_14_col94;
let op1_limb_15_col95 = memory_id_to_big_value_tmp_57455_31.get_m31(15);
            *row[95] = op1_limb_15_col95;
let op1_limb_16_col96 = memory_id_to_big_value_tmp_57455_31.get_m31(16);
            *row[96] = op1_limb_16_col96;
let op1_limb_17_col97 = memory_id_to_big_value_tmp_57455_31.get_m31(17);
            *row[97] = op1_limb_17_col97;
let op1_limb_18_col98 = memory_id_to_big_value_tmp_57455_31.get_m31(18);
            *row[98] = op1_limb_18_col98;
let op1_limb_19_col99 = memory_id_to_big_value_tmp_57455_31.get_m31(19);
            *row[99] = op1_limb_19_col99;
let op1_limb_20_col100 = memory_id_to_big_value_tmp_57455_31.get_m31(20);
            *row[100] = op1_limb_20_col100;
let op1_limb_21_col101 = memory_id_to_big_value_tmp_57455_31.get_m31(21);
            *row[101] = op1_limb_21_col101;
let op1_limb_22_col102 = memory_id_to_big_value_tmp_57455_31.get_m31(22);
            *row[102] = op1_limb_22_col102;
let op1_limb_23_col103 = memory_id_to_big_value_tmp_57455_31.get_m31(23);
            *row[103] = op1_limb_23_col103;
let op1_limb_24_col104 = memory_id_to_big_value_tmp_57455_31.get_m31(24);
            *row[104] = op1_limb_24_col104;
let op1_limb_25_col105 = memory_id_to_big_value_tmp_57455_31.get_m31(25);
            *row[105] = op1_limb_25_col105;
let op1_limb_26_col106 = memory_id_to_big_value_tmp_57455_31.get_m31(26);
            *row[106] = op1_limb_26_col106;
let op1_limb_27_col107 = memory_id_to_big_value_tmp_57455_31.get_m31(27);
            *row[107] = op1_limb_27_col107;
for (i, &input) in op1_id_col79.unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .memory_id_to_big_inputs[2] = input;
            }
*lookup_data.memory_id_to_big_2 = [op1_id_col79, op1_limb_0_col80, op1_limb_1_col81, op1_limb_2_col82, op1_limb_3_col83, op1_limb_4_col84, op1_limb_5_col85, op1_limb_6_col86, op1_limb_7_col87, op1_limb_8_col88, op1_limb_9_col89, op1_limb_10_col90, op1_limb_11_col91, op1_limb_12_col92, op1_limb_13_col93, op1_limb_14_col94, op1_limb_15_col95, op1_limb_16_col96, op1_limb_17_col97, op1_limb_18_col98, op1_limb_19_col99, op1_limb_20_col100, op1_limb_21_col101, op1_limb_22_col102, op1_limb_23_col103, op1_limb_24_col104, op1_limb_25_col105, op1_limb_26_col106, op1_limb_27_col107];


            


            //Add 252.

            
let add_res_tmp_57455_32 = ((PackedFelt252::from_limbs([op0_limb_0_col51, op0_limb_1_col52, op0_limb_2_col53, op0_limb_3_col54, op0_limb_4_col55, op0_limb_5_col56, op0_limb_6_col57, op0_limb_7_col58, op0_limb_8_col59, op0_limb_9_col60, op0_limb_10_col61, op0_limb_11_col62, op0_limb_12_col63, op0_limb_13_col64, op0_limb_14_col65, op0_limb_15_col66, op0_limb_16_col67, op0_limb_17_col68, op0_limb_18_col69, op0_limb_19_col70, op0_limb_20_col71, op0_limb_21_col72, op0_limb_22_col73, op0_limb_23_col74, op0_limb_24_col75, op0_limb_25_col76, op0_limb_26_col77, op0_limb_27_col78])) + (PackedFelt252::from_limbs([op1_limb_0_col80, op1_limb_1_col81, op1_limb_2_col82, op1_limb_3_col83, op1_limb_4_col84, op1_limb_5_col85, op1_limb_6_col86, op1_limb_7_col87, op1_limb_8_col88, op1_limb_9_col89, op1_limb_10_col90, op1_limb_11_col91, op1_limb_12_col92, op1_limb_13_col93, op1_limb_14_col94, op1_limb_15_col95, op1_limb_16_col96, op1_limb_17_col97, op1_limb_18_col98, op1_limb_19_col99, op1_limb_20_col100, op1_limb_21_col101, op1_limb_22_col102, op1_limb_23_col103, op1_limb_24_col104, op1_limb_25_col105, op1_limb_26_col106, op1_limb_27_col107])));
let add_res_limb_0_col108 = add_res_tmp_57455_32.get_m31(0);
            *row[108] = add_res_limb_0_col108;
let add_res_limb_1_col109 = add_res_tmp_57455_32.get_m31(1);
            *row[109] = add_res_limb_1_col109;
let add_res_limb_2_col110 = add_res_tmp_57455_32.get_m31(2);
            *row[110] = add_res_limb_2_col110;
let add_res_limb_3_col111 = add_res_tmp_57455_32.get_m31(3);
            *row[111] = add_res_limb_3_col111;
let add_res_limb_4_col112 = add_res_tmp_57455_32.get_m31(4);
            *row[112] = add_res_limb_4_col112;
let add_res_limb_5_col113 = add_res_tmp_57455_32.get_m31(5);
            *row[113] = add_res_limb_5_col113;
let add_res_limb_6_col114 = add_res_tmp_57455_32.get_m31(6);
            *row[114] = add_res_limb_6_col114;
let add_res_limb_7_col115 = add_res_tmp_57455_32.get_m31(7);
            *row[115] = add_res_limb_7_col115;
let add_res_limb_8_col116 = add_res_tmp_57455_32.get_m31(8);
            *row[116] = add_res_limb_8_col116;
let add_res_limb_9_col117 = add_res_tmp_57455_32.get_m31(9);
            *row[117] = add_res_limb_9_col117;
let add_res_limb_10_col118 = add_res_tmp_57455_32.get_m31(10);
            *row[118] = add_res_limb_10_col118;
let add_res_limb_11_col119 = add_res_tmp_57455_32.get_m31(11);
            *row[119] = add_res_limb_11_col119;
let add_res_limb_12_col120 = add_res_tmp_57455_32.get_m31(12);
            *row[120] = add_res_limb_12_col120;
let add_res_limb_13_col121 = add_res_tmp_57455_32.get_m31(13);
            *row[121] = add_res_limb_13_col121;
let add_res_limb_14_col122 = add_res_tmp_57455_32.get_m31(14);
            *row[122] = add_res_limb_14_col122;
let add_res_limb_15_col123 = add_res_tmp_57455_32.get_m31(15);
            *row[123] = add_res_limb_15_col123;
let add_res_limb_16_col124 = add_res_tmp_57455_32.get_m31(16);
            *row[124] = add_res_limb_16_col124;
let add_res_limb_17_col125 = add_res_tmp_57455_32.get_m31(17);
            *row[125] = add_res_limb_17_col125;
let add_res_limb_18_col126 = add_res_tmp_57455_32.get_m31(18);
            *row[126] = add_res_limb_18_col126;
let add_res_limb_19_col127 = add_res_tmp_57455_32.get_m31(19);
            *row[127] = add_res_limb_19_col127;
let add_res_limb_20_col128 = add_res_tmp_57455_32.get_m31(20);
            *row[128] = add_res_limb_20_col128;
let add_res_limb_21_col129 = add_res_tmp_57455_32.get_m31(21);
            *row[129] = add_res_limb_21_col129;
let add_res_limb_22_col130 = add_res_tmp_57455_32.get_m31(22);
            *row[130] = add_res_limb_22_col130;
let add_res_limb_23_col131 = add_res_tmp_57455_32.get_m31(23);
            *row[131] = add_res_limb_23_col131;
let add_res_limb_24_col132 = add_res_tmp_57455_32.get_m31(24);
            *row[132] = add_res_limb_24_col132;
let add_res_limb_25_col133 = add_res_tmp_57455_32.get_m31(25);
            *row[133] = add_res_limb_25_col133;
let add_res_limb_26_col134 = add_res_tmp_57455_32.get_m31(26);
            *row[134] = add_res_limb_26_col134;
let add_res_limb_27_col135 = add_res_tmp_57455_32.get_m31(27);
            *row[135] = add_res_limb_27_col135;


            //Range Check Big Value.

            

for (i, &input) in [add_res_limb_0_col108, add_res_limb_1_col109].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[0] = input;
            }
*lookup_data.range_check_9_9_0 = [add_res_limb_0_col108, add_res_limb_1_col109];

for (i, &input) in [add_res_limb_2_col110, add_res_limb_3_col111].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[1] = input;
            }
*lookup_data.range_check_9_9_1 = [add_res_limb_2_col110, add_res_limb_3_col111];

for (i, &input) in [add_res_limb_4_col112, add_res_limb_5_col113].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[2] = input;
            }
*lookup_data.range_check_9_9_2 = [add_res_limb_4_col112, add_res_limb_5_col113];

for (i, &input) in [add_res_limb_6_col114, add_res_limb_7_col115].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[3] = input;
            }
*lookup_data.range_check_9_9_3 = [add_res_limb_6_col114, add_res_limb_7_col115];

for (i, &input) in [add_res_limb_8_col116, add_res_limb_9_col117].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[4] = input;
            }
*lookup_data.range_check_9_9_4 = [add_res_limb_8_col116, add_res_limb_9_col117];

for (i, &input) in [add_res_limb_10_col118, add_res_limb_11_col119].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[5] = input;
            }
*lookup_data.range_check_9_9_5 = [add_res_limb_10_col118, add_res_limb_11_col119];

for (i, &input) in [add_res_limb_12_col120, add_res_limb_13_col121].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[6] = input;
            }
*lookup_data.range_check_9_9_6 = [add_res_limb_12_col120, add_res_limb_13_col121];

for (i, &input) in [add_res_limb_14_col122, add_res_limb_15_col123].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[7] = input;
            }
*lookup_data.range_check_9_9_7 = [add_res_limb_14_col122, add_res_limb_15_col123];

for (i, &input) in [add_res_limb_16_col124, add_res_limb_17_col125].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[8] = input;
            }
*lookup_data.range_check_9_9_8 = [add_res_limb_16_col124, add_res_limb_17_col125];

for (i, &input) in [add_res_limb_18_col126, add_res_limb_19_col127].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[9] = input;
            }
*lookup_data.range_check_9_9_9 = [add_res_limb_18_col126, add_res_limb_19_col127];

for (i, &input) in [add_res_limb_20_col128, add_res_limb_21_col129].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[10] = input;
            }
*lookup_data.range_check_9_9_10 = [add_res_limb_20_col128, add_res_limb_21_col129];

for (i, &input) in [add_res_limb_22_col130, add_res_limb_23_col131].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[11] = input;
            }
*lookup_data.range_check_9_9_11 = [add_res_limb_22_col130, add_res_limb_23_col131];

for (i, &input) in [add_res_limb_24_col132, add_res_limb_25_col133].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[12] = input;
            }
*lookup_data.range_check_9_9_12 = [add_res_limb_24_col132, add_res_limb_25_col133];

for (i, &input) in [add_res_limb_26_col134, add_res_limb_27_col135].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[13] = input;
            }
*lookup_data.range_check_9_9_13 = [add_res_limb_26_col134, add_res_limb_27_col135];


            


            //Verify Add 252.

            
let sub_p_bit_tmp_57455_33 = ((UInt16_1) & (((((PackedUInt16::from_m31(op0_limb_0_col51)) ^ (PackedUInt16::from_m31(op1_limb_0_col80)))) ^ (PackedUInt16::from_m31(add_res_limb_0_col108)))));
let sub_p_bit_col136 = sub_p_bit_tmp_57455_33.as_m31();
            *row[136] = sub_p_bit_col136;


            


            


            //Mul 252.

            
let mul_res_tmp_57455_61 = ((PackedFelt252::from_limbs([op0_limb_0_col51, op0_limb_1_col52, op0_limb_2_col53, op0_limb_3_col54, op0_limb_4_col55, op0_limb_5_col56, op0_limb_6_col57, op0_limb_7_col58, op0_limb_8_col59, op0_limb_9_col60, op0_limb_10_col61, op0_limb_11_col62, op0_limb_12_col63, op0_limb_13_col64, op0_limb_14_col65, op0_limb_15_col66, op0_limb_16_col67, op0_limb_17_col68, op0_limb_18_col69, op0_limb_19_col70, op0_limb_20_col71, op0_limb_21_col72, op0_limb_22_col73, op0_limb_23_col74, op0_limb_24_col75, op0_limb_25_col76, op0_limb_26_col77, op0_limb_27_col78])) * (PackedFelt252::from_limbs([op1_limb_0_col80, op1_limb_1_col81, op1_limb_2_col82, op1_limb_3_col83, op1_limb_4_col84, op1_limb_5_col85, op1_limb_6_col86, op1_limb_7_col87, op1_limb_8_col88, op1_limb_9_col89, op1_limb_10_col90, op1_limb_11_col91, op1_limb_12_col92, op1_limb_13_col93, op1_limb_14_col94, op1_limb_15_col95, op1_limb_16_col96, op1_limb_17_col97, op1_limb_18_col98, op1_limb_19_col99, op1_limb_20_col100, op1_limb_21_col101, op1_limb_22_col102, op1_limb_23_col103, op1_limb_24_col104, op1_limb_25_col105, op1_limb_26_col106, op1_limb_27_col107])));
let mul_res_limb_0_col137 = mul_res_tmp_57455_61.get_m31(0);
            *row[137] = mul_res_limb_0_col137;
let mul_res_limb_1_col138 = mul_res_tmp_57455_61.get_m31(1);
            *row[138] = mul_res_limb_1_col138;
let mul_res_limb_2_col139 = mul_res_tmp_57455_61.get_m31(2);
            *row[139] = mul_res_limb_2_col139;
let mul_res_limb_3_col140 = mul_res_tmp_57455_61.get_m31(3);
            *row[140] = mul_res_limb_3_col140;
let mul_res_limb_4_col141 = mul_res_tmp_57455_61.get_m31(4);
            *row[141] = mul_res_limb_4_col141;
let mul_res_limb_5_col142 = mul_res_tmp_57455_61.get_m31(5);
            *row[142] = mul_res_limb_5_col142;
let mul_res_limb_6_col143 = mul_res_tmp_57455_61.get_m31(6);
            *row[143] = mul_res_limb_6_col143;
let mul_res_limb_7_col144 = mul_res_tmp_57455_61.get_m31(7);
            *row[144] = mul_res_limb_7_col144;
let mul_res_limb_8_col145 = mul_res_tmp_57455_61.get_m31(8);
            *row[145] = mul_res_limb_8_col145;
let mul_res_limb_9_col146 = mul_res_tmp_57455_61.get_m31(9);
            *row[146] = mul_res_limb_9_col146;
let mul_res_limb_10_col147 = mul_res_tmp_57455_61.get_m31(10);
            *row[147] = mul_res_limb_10_col147;
let mul_res_limb_11_col148 = mul_res_tmp_57455_61.get_m31(11);
            *row[148] = mul_res_limb_11_col148;
let mul_res_limb_12_col149 = mul_res_tmp_57455_61.get_m31(12);
            *row[149] = mul_res_limb_12_col149;
let mul_res_limb_13_col150 = mul_res_tmp_57455_61.get_m31(13);
            *row[150] = mul_res_limb_13_col150;
let mul_res_limb_14_col151 = mul_res_tmp_57455_61.get_m31(14);
            *row[151] = mul_res_limb_14_col151;
let mul_res_limb_15_col152 = mul_res_tmp_57455_61.get_m31(15);
            *row[152] = mul_res_limb_15_col152;
let mul_res_limb_16_col153 = mul_res_tmp_57455_61.get_m31(16);
            *row[153] = mul_res_limb_16_col153;
let mul_res_limb_17_col154 = mul_res_tmp_57455_61.get_m31(17);
            *row[154] = mul_res_limb_17_col154;
let mul_res_limb_18_col155 = mul_res_tmp_57455_61.get_m31(18);
            *row[155] = mul_res_limb_18_col155;
let mul_res_limb_19_col156 = mul_res_tmp_57455_61.get_m31(19);
            *row[156] = mul_res_limb_19_col156;
let mul_res_limb_20_col157 = mul_res_tmp_57455_61.get_m31(20);
            *row[157] = mul_res_limb_20_col157;
let mul_res_limb_21_col158 = mul_res_tmp_57455_61.get_m31(21);
            *row[158] = mul_res_limb_21_col158;
let mul_res_limb_22_col159 = mul_res_tmp_57455_61.get_m31(22);
            *row[159] = mul_res_limb_22_col159;
let mul_res_limb_23_col160 = mul_res_tmp_57455_61.get_m31(23);
            *row[160] = mul_res_limb_23_col160;
let mul_res_limb_24_col161 = mul_res_tmp_57455_61.get_m31(24);
            *row[161] = mul_res_limb_24_col161;
let mul_res_limb_25_col162 = mul_res_tmp_57455_61.get_m31(25);
            *row[162] = mul_res_limb_25_col162;
let mul_res_limb_26_col163 = mul_res_tmp_57455_61.get_m31(26);
            *row[163] = mul_res_limb_26_col163;
let mul_res_limb_27_col164 = mul_res_tmp_57455_61.get_m31(27);
            *row[164] = mul_res_limb_27_col164;


            //Range Check Big Value.

            

for (i, &input) in [mul_res_limb_0_col137, mul_res_limb_1_col138].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[14] = input;
            }
*lookup_data.range_check_9_9_14 = [mul_res_limb_0_col137, mul_res_limb_1_col138];

for (i, &input) in [mul_res_limb_2_col139, mul_res_limb_3_col140].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[15] = input;
            }
*lookup_data.range_check_9_9_15 = [mul_res_limb_2_col139, mul_res_limb_3_col140];

for (i, &input) in [mul_res_limb_4_col141, mul_res_limb_5_col142].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[16] = input;
            }
*lookup_data.range_check_9_9_16 = [mul_res_limb_4_col141, mul_res_limb_5_col142];

for (i, &input) in [mul_res_limb_6_col143, mul_res_limb_7_col144].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[17] = input;
            }
*lookup_data.range_check_9_9_17 = [mul_res_limb_6_col143, mul_res_limb_7_col144];

for (i, &input) in [mul_res_limb_8_col145, mul_res_limb_9_col146].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[18] = input;
            }
*lookup_data.range_check_9_9_18 = [mul_res_limb_8_col145, mul_res_limb_9_col146];

for (i, &input) in [mul_res_limb_10_col147, mul_res_limb_11_col148].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[19] = input;
            }
*lookup_data.range_check_9_9_19 = [mul_res_limb_10_col147, mul_res_limb_11_col148];

for (i, &input) in [mul_res_limb_12_col149, mul_res_limb_13_col150].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[20] = input;
            }
*lookup_data.range_check_9_9_20 = [mul_res_limb_12_col149, mul_res_limb_13_col150];

for (i, &input) in [mul_res_limb_14_col151, mul_res_limb_15_col152].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[21] = input;
            }
*lookup_data.range_check_9_9_21 = [mul_res_limb_14_col151, mul_res_limb_15_col152];

for (i, &input) in [mul_res_limb_16_col153, mul_res_limb_17_col154].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[22] = input;
            }
*lookup_data.range_check_9_9_22 = [mul_res_limb_16_col153, mul_res_limb_17_col154];

for (i, &input) in [mul_res_limb_18_col155, mul_res_limb_19_col156].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[23] = input;
            }
*lookup_data.range_check_9_9_23 = [mul_res_limb_18_col155, mul_res_limb_19_col156];

for (i, &input) in [mul_res_limb_20_col157, mul_res_limb_21_col158].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[24] = input;
            }
*lookup_data.range_check_9_9_24 = [mul_res_limb_20_col157, mul_res_limb_21_col158];

for (i, &input) in [mul_res_limb_22_col159, mul_res_limb_23_col160].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[25] = input;
            }
*lookup_data.range_check_9_9_25 = [mul_res_limb_22_col159, mul_res_limb_23_col160];

for (i, &input) in [mul_res_limb_24_col161, mul_res_limb_25_col162].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[26] = input;
            }
*lookup_data.range_check_9_9_26 = [mul_res_limb_24_col161, mul_res_limb_25_col162];

for (i, &input) in [mul_res_limb_26_col163, mul_res_limb_27_col164].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_9_9_inputs[27] = input;
            }
*lookup_data.range_check_9_9_27 = [mul_res_limb_26_col163, mul_res_limb_27_col164];


            


            //Verify Mul 252.

            
let conv_tmp_57455_62 = ((((M31_0) - (mul_res_limb_0_col137))) + (((op0_limb_0_col51) * (op1_limb_0_col80))));
let conv_tmp_57455_63 = ((((((M31_0) - (mul_res_limb_1_col138))) + (((op0_limb_0_col51) * (op1_limb_1_col81))))) + (((op0_limb_1_col52) * (op1_limb_0_col80))));
let conv_tmp_57455_64 = ((((((((M31_0) - (mul_res_limb_2_col139))) + (((op0_limb_0_col51) * (op1_limb_2_col82))))) + (((op0_limb_1_col52) * (op1_limb_1_col81))))) + (((op0_limb_2_col53) * (op1_limb_0_col80))));
let conv_tmp_57455_65 = ((((((((((M31_0) - (mul_res_limb_3_col140))) + (((op0_limb_0_col51) * (op1_limb_3_col83))))) + (((op0_limb_1_col52) * (op1_limb_2_col82))))) + (((op0_limb_2_col53) * (op1_limb_1_col81))))) + (((op0_limb_3_col54) * (op1_limb_0_col80))));
let conv_tmp_57455_66 = ((((((((((((M31_0) - (mul_res_limb_4_col141))) + (((op0_limb_0_col51) * (op1_limb_4_col84))))) + (((op0_limb_1_col52) * (op1_limb_3_col83))))) + (((op0_limb_2_col53) * (op1_limb_2_col82))))) + (((op0_limb_3_col54) * (op1_limb_1_col81))))) + (((op0_limb_4_col55) * (op1_limb_0_col80))));
let conv_tmp_57455_67 = ((((((((((((((M31_0) - (mul_res_limb_5_col142))) + (((op0_limb_0_col51) * (op1_limb_5_col85))))) + (((op0_limb_1_col52) * (op1_limb_4_col84))))) + (((op0_limb_2_col53) * (op1_limb_3_col83))))) + (((op0_limb_3_col54) * (op1_limb_2_col82))))) + (((op0_limb_4_col55) * (op1_limb_1_col81))))) + (((op0_limb_5_col56) * (op1_limb_0_col80))));
let conv_tmp_57455_68 = ((((((((((((((((M31_0) - (mul_res_limb_6_col143))) + (((op0_limb_0_col51) * (op1_limb_6_col86))))) + (((op0_limb_1_col52) * (op1_limb_5_col85))))) + (((op0_limb_2_col53) * (op1_limb_4_col84))))) + (((op0_limb_3_col54) * (op1_limb_3_col83))))) + (((op0_limb_4_col55) * (op1_limb_2_col82))))) + (((op0_limb_5_col56) * (op1_limb_1_col81))))) + (((op0_limb_6_col57) * (op1_limb_0_col80))));
let conv_tmp_57455_69 = ((((((((((((((((((M31_0) - (mul_res_limb_7_col144))) + (((op0_limb_0_col51) * (op1_limb_7_col87))))) + (((op0_limb_1_col52) * (op1_limb_6_col86))))) + (((op0_limb_2_col53) * (op1_limb_5_col85))))) + (((op0_limb_3_col54) * (op1_limb_4_col84))))) + (((op0_limb_4_col55) * (op1_limb_3_col83))))) + (((op0_limb_5_col56) * (op1_limb_2_col82))))) + (((op0_limb_6_col57) * (op1_limb_1_col81))))) + (((op0_limb_7_col58) * (op1_limb_0_col80))));
let conv_tmp_57455_70 = ((((((((((((((((((((M31_0) - (mul_res_limb_8_col145))) + (((op0_limb_0_col51) * (op1_limb_8_col88))))) + (((op0_limb_1_col52) * (op1_limb_7_col87))))) + (((op0_limb_2_col53) * (op1_limb_6_col86))))) + (((op0_limb_3_col54) * (op1_limb_5_col85))))) + (((op0_limb_4_col55) * (op1_limb_4_col84))))) + (((op0_limb_5_col56) * (op1_limb_3_col83))))) + (((op0_limb_6_col57) * (op1_limb_2_col82))))) + (((op0_limb_7_col58) * (op1_limb_1_col81))))) + (((op0_limb_8_col59) * (op1_limb_0_col80))));
let conv_tmp_57455_71 = ((((((((((((((((((((((M31_0) - (mul_res_limb_9_col146))) + (((op0_limb_0_col51) * (op1_limb_9_col89))))) + (((op0_limb_1_col52) * (op1_limb_8_col88))))) + (((op0_limb_2_col53) * (op1_limb_7_col87))))) + (((op0_limb_3_col54) * (op1_limb_6_col86))))) + (((op0_limb_4_col55) * (op1_limb_5_col85))))) + (((op0_limb_5_col56) * (op1_limb_4_col84))))) + (((op0_limb_6_col57) * (op1_limb_3_col83))))) + (((op0_limb_7_col58) * (op1_limb_2_col82))))) + (((op0_limb_8_col59) * (op1_limb_1_col81))))) + (((op0_limb_9_col60) * (op1_limb_0_col80))));
let conv_tmp_57455_72 = ((((((((((((((((((((((((M31_0) - (mul_res_limb_10_col147))) + (((op0_limb_0_col51) * (op1_limb_10_col90))))) + (((op0_limb_1_col52) * (op1_limb_9_col89))))) + (((op0_limb_2_col53) * (op1_limb_8_col88))))) + (((op0_limb_3_col54) * (op1_limb_7_col87))))) + (((op0_limb_4_col55) * (op1_limb_6_col86))))) + (((op0_limb_5_col56) * (op1_limb_5_col85))))) + (((op0_limb_6_col57) * (op1_limb_4_col84))))) + (((op0_limb_7_col58) * (op1_limb_3_col83))))) + (((op0_limb_8_col59) * (op1_limb_2_col82))))) + (((op0_limb_9_col60) * (op1_limb_1_col81))))) + (((op0_limb_10_col61) * (op1_limb_0_col80))));
let conv_tmp_57455_73 = ((((((((((((((((((((((((((M31_0) - (mul_res_limb_11_col148))) + (((op0_limb_0_col51) * (op1_limb_11_col91))))) + (((op0_limb_1_col52) * (op1_limb_10_col90))))) + (((op0_limb_2_col53) * (op1_limb_9_col89))))) + (((op0_limb_3_col54) * (op1_limb_8_col88))))) + (((op0_limb_4_col55) * (op1_limb_7_col87))))) + (((op0_limb_5_col56) * (op1_limb_6_col86))))) + (((op0_limb_6_col57) * (op1_limb_5_col85))))) + (((op0_limb_7_col58) * (op1_limb_4_col84))))) + (((op0_limb_8_col59) * (op1_limb_3_col83))))) + (((op0_limb_9_col60) * (op1_limb_2_col82))))) + (((op0_limb_10_col61) * (op1_limb_1_col81))))) + (((op0_limb_11_col62) * (op1_limb_0_col80))));
let conv_tmp_57455_74 = ((((((((((((((((((((((((((((M31_0) - (mul_res_limb_12_col149))) + (((op0_limb_0_col51) * (op1_limb_12_col92))))) + (((op0_limb_1_col52) * (op1_limb_11_col91))))) + (((op0_limb_2_col53) * (op1_limb_10_col90))))) + (((op0_limb_3_col54) * (op1_limb_9_col89))))) + (((op0_limb_4_col55) * (op1_limb_8_col88))))) + (((op0_limb_5_col56) * (op1_limb_7_col87))))) + (((op0_limb_6_col57) * (op1_limb_6_col86))))) + (((op0_limb_7_col58) * (op1_limb_5_col85))))) + (((op0_limb_8_col59) * (op1_limb_4_col84))))) + (((op0_limb_9_col60) * (op1_limb_3_col83))))) + (((op0_limb_10_col61) * (op1_limb_2_col82))))) + (((op0_limb_11_col62) * (op1_limb_1_col81))))) + (((op0_limb_12_col63) * (op1_limb_0_col80))));
let conv_tmp_57455_75 = ((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_13_col150))) + (((op0_limb_0_col51) * (op1_limb_13_col93))))) + (((op0_limb_1_col52) * (op1_limb_12_col92))))) + (((op0_limb_2_col53) * (op1_limb_11_col91))))) + (((op0_limb_3_col54) * (op1_limb_10_col90))))) + (((op0_limb_4_col55) * (op1_limb_9_col89))))) + (((op0_limb_5_col56) * (op1_limb_8_col88))))) + (((op0_limb_6_col57) * (op1_limb_7_col87))))) + (((op0_limb_7_col58) * (op1_limb_6_col86))))) + (((op0_limb_8_col59) * (op1_limb_5_col85))))) + (((op0_limb_9_col60) * (op1_limb_4_col84))))) + (((op0_limb_10_col61) * (op1_limb_3_col83))))) + (((op0_limb_11_col62) * (op1_limb_2_col82))))) + (((op0_limb_12_col63) * (op1_limb_1_col81))))) + (((op0_limb_13_col64) * (op1_limb_0_col80))));
let conv_tmp_57455_76 = ((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_14_col151))) + (((op0_limb_0_col51) * (op1_limb_14_col94))))) + (((op0_limb_1_col52) * (op1_limb_13_col93))))) + (((op0_limb_2_col53) * (op1_limb_12_col92))))) + (((op0_limb_3_col54) * (op1_limb_11_col91))))) + (((op0_limb_4_col55) * (op1_limb_10_col90))))) + (((op0_limb_5_col56) * (op1_limb_9_col89))))) + (((op0_limb_6_col57) * (op1_limb_8_col88))))) + (((op0_limb_7_col58) * (op1_limb_7_col87))))) + (((op0_limb_8_col59) * (op1_limb_6_col86))))) + (((op0_limb_9_col60) * (op1_limb_5_col85))))) + (((op0_limb_10_col61) * (op1_limb_4_col84))))) + (((op0_limb_11_col62) * (op1_limb_3_col83))))) + (((op0_limb_12_col63) * (op1_limb_2_col82))))) + (((op0_limb_13_col64) * (op1_limb_1_col81))))) + (((op0_limb_14_col65) * (op1_limb_0_col80))));
let conv_tmp_57455_77 = ((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_15_col152))) + (((op0_limb_0_col51) * (op1_limb_15_col95))))) + (((op0_limb_1_col52) * (op1_limb_14_col94))))) + (((op0_limb_2_col53) * (op1_limb_13_col93))))) + (((op0_limb_3_col54) * (op1_limb_12_col92))))) + (((op0_limb_4_col55) * (op1_limb_11_col91))))) + (((op0_limb_5_col56) * (op1_limb_10_col90))))) + (((op0_limb_6_col57) * (op1_limb_9_col89))))) + (((op0_limb_7_col58) * (op1_limb_8_col88))))) + (((op0_limb_8_col59) * (op1_limb_7_col87))))) + (((op0_limb_9_col60) * (op1_limb_6_col86))))) + (((op0_limb_10_col61) * (op1_limb_5_col85))))) + (((op0_limb_11_col62) * (op1_limb_4_col84))))) + (((op0_limb_12_col63) * (op1_limb_3_col83))))) + (((op0_limb_13_col64) * (op1_limb_2_col82))))) + (((op0_limb_14_col65) * (op1_limb_1_col81))))) + (((op0_limb_15_col66) * (op1_limb_0_col80))));
let conv_tmp_57455_78 = ((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_16_col153))) + (((op0_limb_0_col51) * (op1_limb_16_col96))))) + (((op0_limb_1_col52) * (op1_limb_15_col95))))) + (((op0_limb_2_col53) * (op1_limb_14_col94))))) + (((op0_limb_3_col54) * (op1_limb_13_col93))))) + (((op0_limb_4_col55) * (op1_limb_12_col92))))) + (((op0_limb_5_col56) * (op1_limb_11_col91))))) + (((op0_limb_6_col57) * (op1_limb_10_col90))))) + (((op0_limb_7_col58) * (op1_limb_9_col89))))) + (((op0_limb_8_col59) * (op1_limb_8_col88))))) + (((op0_limb_9_col60) * (op1_limb_7_col87))))) + (((op0_limb_10_col61) * (op1_limb_6_col86))))) + (((op0_limb_11_col62) * (op1_limb_5_col85))))) + (((op0_limb_12_col63) * (op1_limb_4_col84))))) + (((op0_limb_13_col64) * (op1_limb_3_col83))))) + (((op0_limb_14_col65) * (op1_limb_2_col82))))) + (((op0_limb_15_col66) * (op1_limb_1_col81))))) + (((op0_limb_16_col67) * (op1_limb_0_col80))));
let conv_tmp_57455_79 = ((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_17_col154))) + (((op0_limb_0_col51) * (op1_limb_17_col97))))) + (((op0_limb_1_col52) * (op1_limb_16_col96))))) + (((op0_limb_2_col53) * (op1_limb_15_col95))))) + (((op0_limb_3_col54) * (op1_limb_14_col94))))) + (((op0_limb_4_col55) * (op1_limb_13_col93))))) + (((op0_limb_5_col56) * (op1_limb_12_col92))))) + (((op0_limb_6_col57) * (op1_limb_11_col91))))) + (((op0_limb_7_col58) * (op1_limb_10_col90))))) + (((op0_limb_8_col59) * (op1_limb_9_col89))))) + (((op0_limb_9_col60) * (op1_limb_8_col88))))) + (((op0_limb_10_col61) * (op1_limb_7_col87))))) + (((op0_limb_11_col62) * (op1_limb_6_col86))))) + (((op0_limb_12_col63) * (op1_limb_5_col85))))) + (((op0_limb_13_col64) * (op1_limb_4_col84))))) + (((op0_limb_14_col65) * (op1_limb_3_col83))))) + (((op0_limb_15_col66) * (op1_limb_2_col82))))) + (((op0_limb_16_col67) * (op1_limb_1_col81))))) + (((op0_limb_17_col68) * (op1_limb_0_col80))));
let conv_tmp_57455_80 = ((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_18_col155))) + (((op0_limb_0_col51) * (op1_limb_18_col98))))) + (((op0_limb_1_col52) * (op1_limb_17_col97))))) + (((op0_limb_2_col53) * (op1_limb_16_col96))))) + (((op0_limb_3_col54) * (op1_limb_15_col95))))) + (((op0_limb_4_col55) * (op1_limb_14_col94))))) + (((op0_limb_5_col56) * (op1_limb_13_col93))))) + (((op0_limb_6_col57) * (op1_limb_12_col92))))) + (((op0_limb_7_col58) * (op1_limb_11_col91))))) + (((op0_limb_8_col59) * (op1_limb_10_col90))))) + (((op0_limb_9_col60) * (op1_limb_9_col89))))) + (((op0_limb_10_col61) * (op1_limb_8_col88))))) + (((op0_limb_11_col62) * (op1_limb_7_col87))))) + (((op0_limb_12_col63) * (op1_limb_6_col86))))) + (((op0_limb_13_col64) * (op1_limb_5_col85))))) + (((op0_limb_14_col65) * (op1_limb_4_col84))))) + (((op0_limb_15_col66) * (op1_limb_3_col83))))) + (((op0_limb_16_col67) * (op1_limb_2_col82))))) + (((op0_limb_17_col68) * (op1_limb_1_col81))))) + (((op0_limb_18_col69) * (op1_limb_0_col80))));
let conv_tmp_57455_81 = ((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_19_col156))) + (((op0_limb_0_col51) * (op1_limb_19_col99))))) + (((op0_limb_1_col52) * (op1_limb_18_col98))))) + (((op0_limb_2_col53) * (op1_limb_17_col97))))) + (((op0_limb_3_col54) * (op1_limb_16_col96))))) + (((op0_limb_4_col55) * (op1_limb_15_col95))))) + (((op0_limb_5_col56) * (op1_limb_14_col94))))) + (((op0_limb_6_col57) * (op1_limb_13_col93))))) + (((op0_limb_7_col58) * (op1_limb_12_col92))))) + (((op0_limb_8_col59) * (op1_limb_11_col91))))) + (((op0_limb_9_col60) * (op1_limb_10_col90))))) + (((op0_limb_10_col61) * (op1_limb_9_col89))))) + (((op0_limb_11_col62) * (op1_limb_8_col88))))) + (((op0_limb_12_col63) * (op1_limb_7_col87))))) + (((op0_limb_13_col64) * (op1_limb_6_col86))))) + (((op0_limb_14_col65) * (op1_limb_5_col85))))) + (((op0_limb_15_col66) * (op1_limb_4_col84))))) + (((op0_limb_16_col67) * (op1_limb_3_col83))))) + (((op0_limb_17_col68) * (op1_limb_2_col82))))) + (((op0_limb_18_col69) * (op1_limb_1_col81))))) + (((op0_limb_19_col70) * (op1_limb_0_col80))));
let conv_tmp_57455_82 = ((((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_20_col157))) + (((op0_limb_0_col51) * (op1_limb_20_col100))))) + (((op0_limb_1_col52) * (op1_limb_19_col99))))) + (((op0_limb_2_col53) * (op1_limb_18_col98))))) + (((op0_limb_3_col54) * (op1_limb_17_col97))))) + (((op0_limb_4_col55) * (op1_limb_16_col96))))) + (((op0_limb_5_col56) * (op1_limb_15_col95))))) + (((op0_limb_6_col57) * (op1_limb_14_col94))))) + (((op0_limb_7_col58) * (op1_limb_13_col93))))) + (((op0_limb_8_col59) * (op1_limb_12_col92))))) + (((op0_limb_9_col60) * (op1_limb_11_col91))))) + (((op0_limb_10_col61) * (op1_limb_10_col90))))) + (((op0_limb_11_col62) * (op1_limb_9_col89))))) + (((op0_limb_12_col63) * (op1_limb_8_col88))))) + (((op0_limb_13_col64) * (op1_limb_7_col87))))) + (((op0_limb_14_col65) * (op1_limb_6_col86))))) + (((op0_limb_15_col66) * (op1_limb_5_col85))))) + (((op0_limb_16_col67) * (op1_limb_4_col84))))) + (((op0_limb_17_col68) * (op1_limb_3_col83))))) + (((op0_limb_18_col69) * (op1_limb_2_col82))))) + (((op0_limb_19_col70) * (op1_limb_1_col81))))) + (((op0_limb_20_col71) * (op1_limb_0_col80))));
let conv_tmp_57455_83 = ((((((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_21_col158))) + (((op0_limb_0_col51) * (op1_limb_21_col101))))) + (((op0_limb_1_col52) * (op1_limb_20_col100))))) + (((op0_limb_2_col53) * (op1_limb_19_col99))))) + (((op0_limb_3_col54) * (op1_limb_18_col98))))) + (((op0_limb_4_col55) * (op1_limb_17_col97))))) + (((op0_limb_5_col56) * (op1_limb_16_col96))))) + (((op0_limb_6_col57) * (op1_limb_15_col95))))) + (((op0_limb_7_col58) * (op1_limb_14_col94))))) + (((op0_limb_8_col59) * (op1_limb_13_col93))))) + (((op0_limb_9_col60) * (op1_limb_12_col92))))) + (((op0_limb_10_col61) * (op1_limb_11_col91))))) + (((op0_limb_11_col62) * (op1_limb_10_col90))))) + (((op0_limb_12_col63) * (op1_limb_9_col89))))) + (((op0_limb_13_col64) * (op1_limb_8_col88))))) + (((op0_limb_14_col65) * (op1_limb_7_col87))))) + (((op0_limb_15_col66) * (op1_limb_6_col86))))) + (((op0_limb_16_col67) * (op1_limb_5_col85))))) + (((op0_limb_17_col68) * (op1_limb_4_col84))))) + (((op0_limb_18_col69) * (op1_limb_3_col83))))) + (((op0_limb_19_col70) * (op1_limb_2_col82))))) + (((op0_limb_20_col71) * (op1_limb_1_col81))))) + (((op0_limb_21_col72) * (op1_limb_0_col80))));
let conv_tmp_57455_84 = ((((((((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_22_col159))) + (((op0_limb_0_col51) * (op1_limb_22_col102))))) + (((op0_limb_1_col52) * (op1_limb_21_col101))))) + (((op0_limb_2_col53) * (op1_limb_20_col100))))) + (((op0_limb_3_col54) * (op1_limb_19_col99))))) + (((op0_limb_4_col55) * (op1_limb_18_col98))))) + (((op0_limb_5_col56) * (op1_limb_17_col97))))) + (((op0_limb_6_col57) * (op1_limb_16_col96))))) + (((op0_limb_7_col58) * (op1_limb_15_col95))))) + (((op0_limb_8_col59) * (op1_limb_14_col94))))) + (((op0_limb_9_col60) * (op1_limb_13_col93))))) + (((op0_limb_10_col61) * (op1_limb_12_col92))))) + (((op0_limb_11_col62) * (op1_limb_11_col91))))) + (((op0_limb_12_col63) * (op1_limb_10_col90))))) + (((op0_limb_13_col64) * (op1_limb_9_col89))))) + (((op0_limb_14_col65) * (op1_limb_8_col88))))) + (((op0_limb_15_col66) * (op1_limb_7_col87))))) + (((op0_limb_16_col67) * (op1_limb_6_col86))))) + (((op0_limb_17_col68) * (op1_limb_5_col85))))) + (((op0_limb_18_col69) * (op1_limb_4_col84))))) + (((op0_limb_19_col70) * (op1_limb_3_col83))))) + (((op0_limb_20_col71) * (op1_limb_2_col82))))) + (((op0_limb_21_col72) * (op1_limb_1_col81))))) + (((op0_limb_22_col73) * (op1_limb_0_col80))));
let conv_tmp_57455_85 = ((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_23_col160))) + (((op0_limb_0_col51) * (op1_limb_23_col103))))) + (((op0_limb_1_col52) * (op1_limb_22_col102))))) + (((op0_limb_2_col53) * (op1_limb_21_col101))))) + (((op0_limb_3_col54) * (op1_limb_20_col100))))) + (((op0_limb_4_col55) * (op1_limb_19_col99))))) + (((op0_limb_5_col56) * (op1_limb_18_col98))))) + (((op0_limb_6_col57) * (op1_limb_17_col97))))) + (((op0_limb_7_col58) * (op1_limb_16_col96))))) + (((op0_limb_8_col59) * (op1_limb_15_col95))))) + (((op0_limb_9_col60) * (op1_limb_14_col94))))) + (((op0_limb_10_col61) * (op1_limb_13_col93))))) + (((op0_limb_11_col62) * (op1_limb_12_col92))))) + (((op0_limb_12_col63) * (op1_limb_11_col91))))) + (((op0_limb_13_col64) * (op1_limb_10_col90))))) + (((op0_limb_14_col65) * (op1_limb_9_col89))))) + (((op0_limb_15_col66) * (op1_limb_8_col88))))) + (((op0_limb_16_col67) * (op1_limb_7_col87))))) + (((op0_limb_17_col68) * (op1_limb_6_col86))))) + (((op0_limb_18_col69) * (op1_limb_5_col85))))) + (((op0_limb_19_col70) * (op1_limb_4_col84))))) + (((op0_limb_20_col71) * (op1_limb_3_col83))))) + (((op0_limb_21_col72) * (op1_limb_2_col82))))) + (((op0_limb_22_col73) * (op1_limb_1_col81))))) + (((op0_limb_23_col74) * (op1_limb_0_col80))));
let conv_tmp_57455_86 = ((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_24_col161))) + (((op0_limb_0_col51) * (op1_limb_24_col104))))) + (((op0_limb_1_col52) * (op1_limb_23_col103))))) + (((op0_limb_2_col53) * (op1_limb_22_col102))))) + (((op0_limb_3_col54) * (op1_limb_21_col101))))) + (((op0_limb_4_col55) * (op1_limb_20_col100))))) + (((op0_limb_5_col56) * (op1_limb_19_col99))))) + (((op0_limb_6_col57) * (op1_limb_18_col98))))) + (((op0_limb_7_col58) * (op1_limb_17_col97))))) + (((op0_limb_8_col59) * (op1_limb_16_col96))))) + (((op0_limb_9_col60) * (op1_limb_15_col95))))) + (((op0_limb_10_col61) * (op1_limb_14_col94))))) + (((op0_limb_11_col62) * (op1_limb_13_col93))))) + (((op0_limb_12_col63) * (op1_limb_12_col92))))) + (((op0_limb_13_col64) * (op1_limb_11_col91))))) + (((op0_limb_14_col65) * (op1_limb_10_col90))))) + (((op0_limb_15_col66) * (op1_limb_9_col89))))) + (((op0_limb_16_col67) * (op1_limb_8_col88))))) + (((op0_limb_17_col68) * (op1_limb_7_col87))))) + (((op0_limb_18_col69) * (op1_limb_6_col86))))) + (((op0_limb_19_col70) * (op1_limb_5_col85))))) + (((op0_limb_20_col71) * (op1_limb_4_col84))))) + (((op0_limb_21_col72) * (op1_limb_3_col83))))) + (((op0_limb_22_col73) * (op1_limb_2_col82))))) + (((op0_limb_23_col74) * (op1_limb_1_col81))))) + (((op0_limb_24_col75) * (op1_limb_0_col80))));
let conv_tmp_57455_87 = ((((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_25_col162))) + (((op0_limb_0_col51) * (op1_limb_25_col105))))) + (((op0_limb_1_col52) * (op1_limb_24_col104))))) + (((op0_limb_2_col53) * (op1_limb_23_col103))))) + (((op0_limb_3_col54) * (op1_limb_22_col102))))) + (((op0_limb_4_col55) * (op1_limb_21_col101))))) + (((op0_limb_5_col56) * (op1_limb_20_col100))))) + (((op0_limb_6_col57) * (op1_limb_19_col99))))) + (((op0_limb_7_col58) * (op1_limb_18_col98))))) + (((op0_limb_8_col59) * (op1_limb_17_col97))))) + (((op0_limb_9_col60) * (op1_limb_16_col96))))) + (((op0_limb_10_col61) * (op1_limb_15_col95))))) + (((op0_limb_11_col62) * (op1_limb_14_col94))))) + (((op0_limb_12_col63) * (op1_limb_13_col93))))) + (((op0_limb_13_col64) * (op1_limb_12_col92))))) + (((op0_limb_14_col65) * (op1_limb_11_col91))))) + (((op0_limb_15_col66) * (op1_limb_10_col90))))) + (((op0_limb_16_col67) * (op1_limb_9_col89))))) + (((op0_limb_17_col68) * (op1_limb_8_col88))))) + (((op0_limb_18_col69) * (op1_limb_7_col87))))) + (((op0_limb_19_col70) * (op1_limb_6_col86))))) + (((op0_limb_20_col71) * (op1_limb_5_col85))))) + (((op0_limb_21_col72) * (op1_limb_4_col84))))) + (((op0_limb_22_col73) * (op1_limb_3_col83))))) + (((op0_limb_23_col74) * (op1_limb_2_col82))))) + (((op0_limb_24_col75) * (op1_limb_1_col81))))) + (((op0_limb_25_col76) * (op1_limb_0_col80))));
let conv_tmp_57455_88 = ((((((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_26_col163))) + (((op0_limb_0_col51) * (op1_limb_26_col106))))) + (((op0_limb_1_col52) * (op1_limb_25_col105))))) + (((op0_limb_2_col53) * (op1_limb_24_col104))))) + (((op0_limb_3_col54) * (op1_limb_23_col103))))) + (((op0_limb_4_col55) * (op1_limb_22_col102))))) + (((op0_limb_5_col56) * (op1_limb_21_col101))))) + (((op0_limb_6_col57) * (op1_limb_20_col100))))) + (((op0_limb_7_col58) * (op1_limb_19_col99))))) + (((op0_limb_8_col59) * (op1_limb_18_col98))))) + (((op0_limb_9_col60) * (op1_limb_17_col97))))) + (((op0_limb_10_col61) * (op1_limb_16_col96))))) + (((op0_limb_11_col62) * (op1_limb_15_col95))))) + (((op0_limb_12_col63) * (op1_limb_14_col94))))) + (((op0_limb_13_col64) * (op1_limb_13_col93))))) + (((op0_limb_14_col65) * (op1_limb_12_col92))))) + (((op0_limb_15_col66) * (op1_limb_11_col91))))) + (((op0_limb_16_col67) * (op1_limb_10_col90))))) + (((op0_limb_17_col68) * (op1_limb_9_col89))))) + (((op0_limb_18_col69) * (op1_limb_8_col88))))) + (((op0_limb_19_col70) * (op1_limb_7_col87))))) + (((op0_limb_20_col71) * (op1_limb_6_col86))))) + (((op0_limb_21_col72) * (op1_limb_5_col85))))) + (((op0_limb_22_col73) * (op1_limb_4_col84))))) + (((op0_limb_23_col74) * (op1_limb_3_col83))))) + (((op0_limb_24_col75) * (op1_limb_2_col82))))) + (((op0_limb_25_col76) * (op1_limb_1_col81))))) + (((op0_limb_26_col77) * (op1_limb_0_col80))));
let conv_tmp_57455_89 = ((((((((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_27_col164))) + (((op0_limb_0_col51) * (op1_limb_27_col107))))) + (((op0_limb_1_col52) * (op1_limb_26_col106))))) + (((op0_limb_2_col53) * (op1_limb_25_col105))))) + (((op0_limb_3_col54) * (op1_limb_24_col104))))) + (((op0_limb_4_col55) * (op1_limb_23_col103))))) + (((op0_limb_5_col56) * (op1_limb_22_col102))))) + (((op0_limb_6_col57) * (op1_limb_21_col101))))) + (((op0_limb_7_col58) * (op1_limb_20_col100))))) + (((op0_limb_8_col59) * (op1_limb_19_col99))))) + (((op0_limb_9_col60) * (op1_limb_18_col98))))) + (((op0_limb_10_col61) * (op1_limb_17_col97))))) + (((op0_limb_11_col62) * (op1_limb_16_col96))))) + (((op0_limb_12_col63) * (op1_limb_15_col95))))) + (((op0_limb_13_col64) * (op1_limb_14_col94))))) + (((op0_limb_14_col65) * (op1_limb_13_col93))))) + (((op0_limb_15_col66) * (op1_limb_12_col92))))) + (((op0_limb_16_col67) * (op1_limb_11_col91))))) + (((op0_limb_17_col68) * (op1_limb_10_col90))))) + (((op0_limb_18_col69) * (op1_limb_9_col89))))) + (((op0_limb_19_col70) * (op1_limb_8_col88))))) + (((op0_limb_20_col71) * (op1_limb_7_col87))))) + (((op0_limb_21_col72) * (op1_limb_6_col86))))) + (((op0_limb_22_col73) * (op1_limb_5_col85))))) + (((op0_limb_23_col74) * (op1_limb_4_col84))))) + (((op0_limb_24_col75) * (op1_limb_3_col83))))) + (((op0_limb_25_col76) * (op1_limb_2_col82))))) + (((op0_limb_26_col77) * (op1_limb_1_col81))))) + (((op0_limb_27_col78) * (op1_limb_0_col80))));
let conv_tmp_57455_90 = ((((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_1_col52) * (op1_limb_27_col107))))) + (((op0_limb_2_col53) * (op1_limb_26_col106))))) + (((op0_limb_3_col54) * (op1_limb_25_col105))))) + (((op0_limb_4_col55) * (op1_limb_24_col104))))) + (((op0_limb_5_col56) * (op1_limb_23_col103))))) + (((op0_limb_6_col57) * (op1_limb_22_col102))))) + (((op0_limb_7_col58) * (op1_limb_21_col101))))) + (((op0_limb_8_col59) * (op1_limb_20_col100))))) + (((op0_limb_9_col60) * (op1_limb_19_col99))))) + (((op0_limb_10_col61) * (op1_limb_18_col98))))) + (((op0_limb_11_col62) * (op1_limb_17_col97))))) + (((op0_limb_12_col63) * (op1_limb_16_col96))))) + (((op0_limb_13_col64) * (op1_limb_15_col95))))) + (((op0_limb_14_col65) * (op1_limb_14_col94))))) + (((op0_limb_15_col66) * (op1_limb_13_col93))))) + (((op0_limb_16_col67) * (op1_limb_12_col92))))) + (((op0_limb_17_col68) * (op1_limb_11_col91))))) + (((op0_limb_18_col69) * (op1_limb_10_col90))))) + (((op0_limb_19_col70) * (op1_limb_9_col89))))) + (((op0_limb_20_col71) * (op1_limb_8_col88))))) + (((op0_limb_21_col72) * (op1_limb_7_col87))))) + (((op0_limb_22_col73) * (op1_limb_6_col86))))) + (((op0_limb_23_col74) * (op1_limb_5_col85))))) + (((op0_limb_24_col75) * (op1_limb_4_col84))))) + (((op0_limb_25_col76) * (op1_limb_3_col83))))) + (((op0_limb_26_col77) * (op1_limb_2_col82))))) + (((op0_limb_27_col78) * (op1_limb_1_col81))));
let conv_tmp_57455_91 = ((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_2_col53) * (op1_limb_27_col107))))) + (((op0_limb_3_col54) * (op1_limb_26_col106))))) + (((op0_limb_4_col55) * (op1_limb_25_col105))))) + (((op0_limb_5_col56) * (op1_limb_24_col104))))) + (((op0_limb_6_col57) * (op1_limb_23_col103))))) + (((op0_limb_7_col58) * (op1_limb_22_col102))))) + (((op0_limb_8_col59) * (op1_limb_21_col101))))) + (((op0_limb_9_col60) * (op1_limb_20_col100))))) + (((op0_limb_10_col61) * (op1_limb_19_col99))))) + (((op0_limb_11_col62) * (op1_limb_18_col98))))) + (((op0_limb_12_col63) * (op1_limb_17_col97))))) + (((op0_limb_13_col64) * (op1_limb_16_col96))))) + (((op0_limb_14_col65) * (op1_limb_15_col95))))) + (((op0_limb_15_col66) * (op1_limb_14_col94))))) + (((op0_limb_16_col67) * (op1_limb_13_col93))))) + (((op0_limb_17_col68) * (op1_limb_12_col92))))) + (((op0_limb_18_col69) * (op1_limb_11_col91))))) + (((op0_limb_19_col70) * (op1_limb_10_col90))))) + (((op0_limb_20_col71) * (op1_limb_9_col89))))) + (((op0_limb_21_col72) * (op1_limb_8_col88))))) + (((op0_limb_22_col73) * (op1_limb_7_col87))))) + (((op0_limb_23_col74) * (op1_limb_6_col86))))) + (((op0_limb_24_col75) * (op1_limb_5_col85))))) + (((op0_limb_25_col76) * (op1_limb_4_col84))))) + (((op0_limb_26_col77) * (op1_limb_3_col83))))) + (((op0_limb_27_col78) * (op1_limb_2_col82))));
let conv_tmp_57455_92 = ((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_3_col54) * (op1_limb_27_col107))))) + (((op0_limb_4_col55) * (op1_limb_26_col106))))) + (((op0_limb_5_col56) * (op1_limb_25_col105))))) + (((op0_limb_6_col57) * (op1_limb_24_col104))))) + (((op0_limb_7_col58) * (op1_limb_23_col103))))) + (((op0_limb_8_col59) * (op1_limb_22_col102))))) + (((op0_limb_9_col60) * (op1_limb_21_col101))))) + (((op0_limb_10_col61) * (op1_limb_20_col100))))) + (((op0_limb_11_col62) * (op1_limb_19_col99))))) + (((op0_limb_12_col63) * (op1_limb_18_col98))))) + (((op0_limb_13_col64) * (op1_limb_17_col97))))) + (((op0_limb_14_col65) * (op1_limb_16_col96))))) + (((op0_limb_15_col66) * (op1_limb_15_col95))))) + (((op0_limb_16_col67) * (op1_limb_14_col94))))) + (((op0_limb_17_col68) * (op1_limb_13_col93))))) + (((op0_limb_18_col69) * (op1_limb_12_col92))))) + (((op0_limb_19_col70) * (op1_limb_11_col91))))) + (((op0_limb_20_col71) * (op1_limb_10_col90))))) + (((op0_limb_21_col72) * (op1_limb_9_col89))))) + (((op0_limb_22_col73) * (op1_limb_8_col88))))) + (((op0_limb_23_col74) * (op1_limb_7_col87))))) + (((op0_limb_24_col75) * (op1_limb_6_col86))))) + (((op0_limb_25_col76) * (op1_limb_5_col85))))) + (((op0_limb_26_col77) * (op1_limb_4_col84))))) + (((op0_limb_27_col78) * (op1_limb_3_col83))));
let conv_tmp_57455_93 = ((((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_4_col55) * (op1_limb_27_col107))))) + (((op0_limb_5_col56) * (op1_limb_26_col106))))) + (((op0_limb_6_col57) * (op1_limb_25_col105))))) + (((op0_limb_7_col58) * (op1_limb_24_col104))))) + (((op0_limb_8_col59) * (op1_limb_23_col103))))) + (((op0_limb_9_col60) * (op1_limb_22_col102))))) + (((op0_limb_10_col61) * (op1_limb_21_col101))))) + (((op0_limb_11_col62) * (op1_limb_20_col100))))) + (((op0_limb_12_col63) * (op1_limb_19_col99))))) + (((op0_limb_13_col64) * (op1_limb_18_col98))))) + (((op0_limb_14_col65) * (op1_limb_17_col97))))) + (((op0_limb_15_col66) * (op1_limb_16_col96))))) + (((op0_limb_16_col67) * (op1_limb_15_col95))))) + (((op0_limb_17_col68) * (op1_limb_14_col94))))) + (((op0_limb_18_col69) * (op1_limb_13_col93))))) + (((op0_limb_19_col70) * (op1_limb_12_col92))))) + (((op0_limb_20_col71) * (op1_limb_11_col91))))) + (((op0_limb_21_col72) * (op1_limb_10_col90))))) + (((op0_limb_22_col73) * (op1_limb_9_col89))))) + (((op0_limb_23_col74) * (op1_limb_8_col88))))) + (((op0_limb_24_col75) * (op1_limb_7_col87))))) + (((op0_limb_25_col76) * (op1_limb_6_col86))))) + (((op0_limb_26_col77) * (op1_limb_5_col85))))) + (((op0_limb_27_col78) * (op1_limb_4_col84))));
let conv_tmp_57455_94 = ((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_5_col56) * (op1_limb_27_col107))))) + (((op0_limb_6_col57) * (op1_limb_26_col106))))) + (((op0_limb_7_col58) * (op1_limb_25_col105))))) + (((op0_limb_8_col59) * (op1_limb_24_col104))))) + (((op0_limb_9_col60) * (op1_limb_23_col103))))) + (((op0_limb_10_col61) * (op1_limb_22_col102))))) + (((op0_limb_11_col62) * (op1_limb_21_col101))))) + (((op0_limb_12_col63) * (op1_limb_20_col100))))) + (((op0_limb_13_col64) * (op1_limb_19_col99))))) + (((op0_limb_14_col65) * (op1_limb_18_col98))))) + (((op0_limb_15_col66) * (op1_limb_17_col97))))) + (((op0_limb_16_col67) * (op1_limb_16_col96))))) + (((op0_limb_17_col68) * (op1_limb_15_col95))))) + (((op0_limb_18_col69) * (op1_limb_14_col94))))) + (((op0_limb_19_col70) * (op1_limb_13_col93))))) + (((op0_limb_20_col71) * (op1_limb_12_col92))))) + (((op0_limb_21_col72) * (op1_limb_11_col91))))) + (((op0_limb_22_col73) * (op1_limb_10_col90))))) + (((op0_limb_23_col74) * (op1_limb_9_col89))))) + (((op0_limb_24_col75) * (op1_limb_8_col88))))) + (((op0_limb_25_col76) * (op1_limb_7_col87))))) + (((op0_limb_26_col77) * (op1_limb_6_col86))))) + (((op0_limb_27_col78) * (op1_limb_5_col85))));
let conv_tmp_57455_95 = ((((((((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_6_col57) * (op1_limb_27_col107))))) + (((op0_limb_7_col58) * (op1_limb_26_col106))))) + (((op0_limb_8_col59) * (op1_limb_25_col105))))) + (((op0_limb_9_col60) * (op1_limb_24_col104))))) + (((op0_limb_10_col61) * (op1_limb_23_col103))))) + (((op0_limb_11_col62) * (op1_limb_22_col102))))) + (((op0_limb_12_col63) * (op1_limb_21_col101))))) + (((op0_limb_13_col64) * (op1_limb_20_col100))))) + (((op0_limb_14_col65) * (op1_limb_19_col99))))) + (((op0_limb_15_col66) * (op1_limb_18_col98))))) + (((op0_limb_16_col67) * (op1_limb_17_col97))))) + (((op0_limb_17_col68) * (op1_limb_16_col96))))) + (((op0_limb_18_col69) * (op1_limb_15_col95))))) + (((op0_limb_19_col70) * (op1_limb_14_col94))))) + (((op0_limb_20_col71) * (op1_limb_13_col93))))) + (((op0_limb_21_col72) * (op1_limb_12_col92))))) + (((op0_limb_22_col73) * (op1_limb_11_col91))))) + (((op0_limb_23_col74) * (op1_limb_10_col90))))) + (((op0_limb_24_col75) * (op1_limb_9_col89))))) + (((op0_limb_25_col76) * (op1_limb_8_col88))))) + (((op0_limb_26_col77) * (op1_limb_7_col87))))) + (((op0_limb_27_col78) * (op1_limb_6_col86))));
let conv_tmp_57455_96 = ((((((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_7_col58) * (op1_limb_27_col107))))) + (((op0_limb_8_col59) * (op1_limb_26_col106))))) + (((op0_limb_9_col60) * (op1_limb_25_col105))))) + (((op0_limb_10_col61) * (op1_limb_24_col104))))) + (((op0_limb_11_col62) * (op1_limb_23_col103))))) + (((op0_limb_12_col63) * (op1_limb_22_col102))))) + (((op0_limb_13_col64) * (op1_limb_21_col101))))) + (((op0_limb_14_col65) * (op1_limb_20_col100))))) + (((op0_limb_15_col66) * (op1_limb_19_col99))))) + (((op0_limb_16_col67) * (op1_limb_18_col98))))) + (((op0_limb_17_col68) * (op1_limb_17_col97))))) + (((op0_limb_18_col69) * (op1_limb_16_col96))))) + (((op0_limb_19_col70) * (op1_limb_15_col95))))) + (((op0_limb_20_col71) * (op1_limb_14_col94))))) + (((op0_limb_21_col72) * (op1_limb_13_col93))))) + (((op0_limb_22_col73) * (op1_limb_12_col92))))) + (((op0_limb_23_col74) * (op1_limb_11_col91))))) + (((op0_limb_24_col75) * (op1_limb_10_col90))))) + (((op0_limb_25_col76) * (op1_limb_9_col89))))) + (((op0_limb_26_col77) * (op1_limb_8_col88))))) + (((op0_limb_27_col78) * (op1_limb_7_col87))));
let conv_tmp_57455_97 = ((((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_8_col59) * (op1_limb_27_col107))))) + (((op0_limb_9_col60) * (op1_limb_26_col106))))) + (((op0_limb_10_col61) * (op1_limb_25_col105))))) + (((op0_limb_11_col62) * (op1_limb_24_col104))))) + (((op0_limb_12_col63) * (op1_limb_23_col103))))) + (((op0_limb_13_col64) * (op1_limb_22_col102))))) + (((op0_limb_14_col65) * (op1_limb_21_col101))))) + (((op0_limb_15_col66) * (op1_limb_20_col100))))) + (((op0_limb_16_col67) * (op1_limb_19_col99))))) + (((op0_limb_17_col68) * (op1_limb_18_col98))))) + (((op0_limb_18_col69) * (op1_limb_17_col97))))) + (((op0_limb_19_col70) * (op1_limb_16_col96))))) + (((op0_limb_20_col71) * (op1_limb_15_col95))))) + (((op0_limb_21_col72) * (op1_limb_14_col94))))) + (((op0_limb_22_col73) * (op1_limb_13_col93))))) + (((op0_limb_23_col74) * (op1_limb_12_col92))))) + (((op0_limb_24_col75) * (op1_limb_11_col91))))) + (((op0_limb_25_col76) * (op1_limb_10_col90))))) + (((op0_limb_26_col77) * (op1_limb_9_col89))))) + (((op0_limb_27_col78) * (op1_limb_8_col88))));
let conv_tmp_57455_98 = ((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_9_col60) * (op1_limb_27_col107))))) + (((op0_limb_10_col61) * (op1_limb_26_col106))))) + (((op0_limb_11_col62) * (op1_limb_25_col105))))) + (((op0_limb_12_col63) * (op1_limb_24_col104))))) + (((op0_limb_13_col64) * (op1_limb_23_col103))))) + (((op0_limb_14_col65) * (op1_limb_22_col102))))) + (((op0_limb_15_col66) * (op1_limb_21_col101))))) + (((op0_limb_16_col67) * (op1_limb_20_col100))))) + (((op0_limb_17_col68) * (op1_limb_19_col99))))) + (((op0_limb_18_col69) * (op1_limb_18_col98))))) + (((op0_limb_19_col70) * (op1_limb_17_col97))))) + (((op0_limb_20_col71) * (op1_limb_16_col96))))) + (((op0_limb_21_col72) * (op1_limb_15_col95))))) + (((op0_limb_22_col73) * (op1_limb_14_col94))))) + (((op0_limb_23_col74) * (op1_limb_13_col93))))) + (((op0_limb_24_col75) * (op1_limb_12_col92))))) + (((op0_limb_25_col76) * (op1_limb_11_col91))))) + (((op0_limb_26_col77) * (op1_limb_10_col90))))) + (((op0_limb_27_col78) * (op1_limb_9_col89))));
let conv_tmp_57455_99 = ((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_10_col61) * (op1_limb_27_col107))))) + (((op0_limb_11_col62) * (op1_limb_26_col106))))) + (((op0_limb_12_col63) * (op1_limb_25_col105))))) + (((op0_limb_13_col64) * (op1_limb_24_col104))))) + (((op0_limb_14_col65) * (op1_limb_23_col103))))) + (((op0_limb_15_col66) * (op1_limb_22_col102))))) + (((op0_limb_16_col67) * (op1_limb_21_col101))))) + (((op0_limb_17_col68) * (op1_limb_20_col100))))) + (((op0_limb_18_col69) * (op1_limb_19_col99))))) + (((op0_limb_19_col70) * (op1_limb_18_col98))))) + (((op0_limb_20_col71) * (op1_limb_17_col97))))) + (((op0_limb_21_col72) * (op1_limb_16_col96))))) + (((op0_limb_22_col73) * (op1_limb_15_col95))))) + (((op0_limb_23_col74) * (op1_limb_14_col94))))) + (((op0_limb_24_col75) * (op1_limb_13_col93))))) + (((op0_limb_25_col76) * (op1_limb_12_col92))))) + (((op0_limb_26_col77) * (op1_limb_11_col91))))) + (((op0_limb_27_col78) * (op1_limb_10_col90))));
let conv_tmp_57455_100 = ((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_11_col62) * (op1_limb_27_col107))))) + (((op0_limb_12_col63) * (op1_limb_26_col106))))) + (((op0_limb_13_col64) * (op1_limb_25_col105))))) + (((op0_limb_14_col65) * (op1_limb_24_col104))))) + (((op0_limb_15_col66) * (op1_limb_23_col103))))) + (((op0_limb_16_col67) * (op1_limb_22_col102))))) + (((op0_limb_17_col68) * (op1_limb_21_col101))))) + (((op0_limb_18_col69) * (op1_limb_20_col100))))) + (((op0_limb_19_col70) * (op1_limb_19_col99))))) + (((op0_limb_20_col71) * (op1_limb_18_col98))))) + (((op0_limb_21_col72) * (op1_limb_17_col97))))) + (((op0_limb_22_col73) * (op1_limb_16_col96))))) + (((op0_limb_23_col74) * (op1_limb_15_col95))))) + (((op0_limb_24_col75) * (op1_limb_14_col94))))) + (((op0_limb_25_col76) * (op1_limb_13_col93))))) + (((op0_limb_26_col77) * (op1_limb_12_col92))))) + (((op0_limb_27_col78) * (op1_limb_11_col91))));
let conv_tmp_57455_101 = ((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_12_col63) * (op1_limb_27_col107))))) + (((op0_limb_13_col64) * (op1_limb_26_col106))))) + (((op0_limb_14_col65) * (op1_limb_25_col105))))) + (((op0_limb_15_col66) * (op1_limb_24_col104))))) + (((op0_limb_16_col67) * (op1_limb_23_col103))))) + (((op0_limb_17_col68) * (op1_limb_22_col102))))) + (((op0_limb_18_col69) * (op1_limb_21_col101))))) + (((op0_limb_19_col70) * (op1_limb_20_col100))))) + (((op0_limb_20_col71) * (op1_limb_19_col99))))) + (((op0_limb_21_col72) * (op1_limb_18_col98))))) + (((op0_limb_22_col73) * (op1_limb_17_col97))))) + (((op0_limb_23_col74) * (op1_limb_16_col96))))) + (((op0_limb_24_col75) * (op1_limb_15_col95))))) + (((op0_limb_25_col76) * (op1_limb_14_col94))))) + (((op0_limb_26_col77) * (op1_limb_13_col93))))) + (((op0_limb_27_col78) * (op1_limb_12_col92))));
let conv_tmp_57455_102 = ((((((((((((((((((((((((((((((M31_0) + (((op0_limb_13_col64) * (op1_limb_27_col107))))) + (((op0_limb_14_col65) * (op1_limb_26_col106))))) + (((op0_limb_15_col66) * (op1_limb_25_col105))))) + (((op0_limb_16_col67) * (op1_limb_24_col104))))) + (((op0_limb_17_col68) * (op1_limb_23_col103))))) + (((op0_limb_18_col69) * (op1_limb_22_col102))))) + (((op0_limb_19_col70) * (op1_limb_21_col101))))) + (((op0_limb_20_col71) * (op1_limb_20_col100))))) + (((op0_limb_21_col72) * (op1_limb_19_col99))))) + (((op0_limb_22_col73) * (op1_limb_18_col98))))) + (((op0_limb_23_col74) * (op1_limb_17_col97))))) + (((op0_limb_24_col75) * (op1_limb_16_col96))))) + (((op0_limb_25_col76) * (op1_limb_15_col95))))) + (((op0_limb_26_col77) * (op1_limb_14_col94))))) + (((op0_limb_27_col78) * (op1_limb_13_col93))));
let conv_tmp_57455_103 = ((((((((((((((((((((((((((((M31_0) + (((op0_limb_14_col65) * (op1_limb_27_col107))))) + (((op0_limb_15_col66) * (op1_limb_26_col106))))) + (((op0_limb_16_col67) * (op1_limb_25_col105))))) + (((op0_limb_17_col68) * (op1_limb_24_col104))))) + (((op0_limb_18_col69) * (op1_limb_23_col103))))) + (((op0_limb_19_col70) * (op1_limb_22_col102))))) + (((op0_limb_20_col71) * (op1_limb_21_col101))))) + (((op0_limb_21_col72) * (op1_limb_20_col100))))) + (((op0_limb_22_col73) * (op1_limb_19_col99))))) + (((op0_limb_23_col74) * (op1_limb_18_col98))))) + (((op0_limb_24_col75) * (op1_limb_17_col97))))) + (((op0_limb_25_col76) * (op1_limb_16_col96))))) + (((op0_limb_26_col77) * (op1_limb_15_col95))))) + (((op0_limb_27_col78) * (op1_limb_14_col94))));
let conv_tmp_57455_104 = ((((((((((((((((((((((((((M31_0) + (((op0_limb_15_col66) * (op1_limb_27_col107))))) + (((op0_limb_16_col67) * (op1_limb_26_col106))))) + (((op0_limb_17_col68) * (op1_limb_25_col105))))) + (((op0_limb_18_col69) * (op1_limb_24_col104))))) + (((op0_limb_19_col70) * (op1_limb_23_col103))))) + (((op0_limb_20_col71) * (op1_limb_22_col102))))) + (((op0_limb_21_col72) * (op1_limb_21_col101))))) + (((op0_limb_22_col73) * (op1_limb_20_col100))))) + (((op0_limb_23_col74) * (op1_limb_19_col99))))) + (((op0_limb_24_col75) * (op1_limb_18_col98))))) + (((op0_limb_25_col76) * (op1_limb_17_col97))))) + (((op0_limb_26_col77) * (op1_limb_16_col96))))) + (((op0_limb_27_col78) * (op1_limb_15_col95))));
let conv_tmp_57455_105 = ((((((((((((((((((((((((M31_0) + (((op0_limb_16_col67) * (op1_limb_27_col107))))) + (((op0_limb_17_col68) * (op1_limb_26_col106))))) + (((op0_limb_18_col69) * (op1_limb_25_col105))))) + (((op0_limb_19_col70) * (op1_limb_24_col104))))) + (((op0_limb_20_col71) * (op1_limb_23_col103))))) + (((op0_limb_21_col72) * (op1_limb_22_col102))))) + (((op0_limb_22_col73) * (op1_limb_21_col101))))) + (((op0_limb_23_col74) * (op1_limb_20_col100))))) + (((op0_limb_24_col75) * (op1_limb_19_col99))))) + (((op0_limb_25_col76) * (op1_limb_18_col98))))) + (((op0_limb_26_col77) * (op1_limb_17_col97))))) + (((op0_limb_27_col78) * (op1_limb_16_col96))));
let conv_tmp_57455_106 = ((((((((((((((((((((((M31_0) + (((op0_limb_17_col68) * (op1_limb_27_col107))))) + (((op0_limb_18_col69) * (op1_limb_26_col106))))) + (((op0_limb_19_col70) * (op1_limb_25_col105))))) + (((op0_limb_20_col71) * (op1_limb_24_col104))))) + (((op0_limb_21_col72) * (op1_limb_23_col103))))) + (((op0_limb_22_col73) * (op1_limb_22_col102))))) + (((op0_limb_23_col74) * (op1_limb_21_col101))))) + (((op0_limb_24_col75) * (op1_limb_20_col100))))) + (((op0_limb_25_col76) * (op1_limb_19_col99))))) + (((op0_limb_26_col77) * (op1_limb_18_col98))))) + (((op0_limb_27_col78) * (op1_limb_17_col97))));
let conv_tmp_57455_107 = ((((((((((((((((((((M31_0) + (((op0_limb_18_col69) * (op1_limb_27_col107))))) + (((op0_limb_19_col70) * (op1_limb_26_col106))))) + (((op0_limb_20_col71) * (op1_limb_25_col105))))) + (((op0_limb_21_col72) * (op1_limb_24_col104))))) + (((op0_limb_22_col73) * (op1_limb_23_col103))))) + (((op0_limb_23_col74) * (op1_limb_22_col102))))) + (((op0_limb_24_col75) * (op1_limb_21_col101))))) + (((op0_limb_25_col76) * (op1_limb_20_col100))))) + (((op0_limb_26_col77) * (op1_limb_19_col99))))) + (((op0_limb_27_col78) * (op1_limb_18_col98))));
let conv_tmp_57455_108 = ((((((((((((((((((M31_0) + (((op0_limb_19_col70) * (op1_limb_27_col107))))) + (((op0_limb_20_col71) * (op1_limb_26_col106))))) + (((op0_limb_21_col72) * (op1_limb_25_col105))))) + (((op0_limb_22_col73) * (op1_limb_24_col104))))) + (((op0_limb_23_col74) * (op1_limb_23_col103))))) + (((op0_limb_24_col75) * (op1_limb_22_col102))))) + (((op0_limb_25_col76) * (op1_limb_21_col101))))) + (((op0_limb_26_col77) * (op1_limb_20_col100))))) + (((op0_limb_27_col78) * (op1_limb_19_col99))));
let conv_tmp_57455_109 = ((((((((((((((((M31_0) + (((op0_limb_20_col71) * (op1_limb_27_col107))))) + (((op0_limb_21_col72) * (op1_limb_26_col106))))) + (((op0_limb_22_col73) * (op1_limb_25_col105))))) + (((op0_limb_23_col74) * (op1_limb_24_col104))))) + (((op0_limb_24_col75) * (op1_limb_23_col103))))) + (((op0_limb_25_col76) * (op1_limb_22_col102))))) + (((op0_limb_26_col77) * (op1_limb_21_col101))))) + (((op0_limb_27_col78) * (op1_limb_20_col100))));
let conv_tmp_57455_110 = ((((((((((((((M31_0) + (((op0_limb_21_col72) * (op1_limb_27_col107))))) + (((op0_limb_22_col73) * (op1_limb_26_col106))))) + (((op0_limb_23_col74) * (op1_limb_25_col105))))) + (((op0_limb_24_col75) * (op1_limb_24_col104))))) + (((op0_limb_25_col76) * (op1_limb_23_col103))))) + (((op0_limb_26_col77) * (op1_limb_22_col102))))) + (((op0_limb_27_col78) * (op1_limb_21_col101))));
let conv_tmp_57455_111 = ((((((((((((M31_0) + (((op0_limb_22_col73) * (op1_limb_27_col107))))) + (((op0_limb_23_col74) * (op1_limb_26_col106))))) + (((op0_limb_24_col75) * (op1_limb_25_col105))))) + (((op0_limb_25_col76) * (op1_limb_24_col104))))) + (((op0_limb_26_col77) * (op1_limb_23_col103))))) + (((op0_limb_27_col78) * (op1_limb_22_col102))));
let conv_tmp_57455_112 = ((((((((((M31_0) + (((op0_limb_23_col74) * (op1_limb_27_col107))))) + (((op0_limb_24_col75) * (op1_limb_26_col106))))) + (((op0_limb_25_col76) * (op1_limb_25_col105))))) + (((op0_limb_26_col77) * (op1_limb_24_col104))))) + (((op0_limb_27_col78) * (op1_limb_23_col103))));
let conv_tmp_57455_113 = ((((((((M31_0) + (((op0_limb_24_col75) * (op1_limb_27_col107))))) + (((op0_limb_25_col76) * (op1_limb_26_col106))))) + (((op0_limb_26_col77) * (op1_limb_25_col105))))) + (((op0_limb_27_col78) * (op1_limb_24_col104))));
let conv_tmp_57455_114 = ((((((M31_0) + (((op0_limb_25_col76) * (op1_limb_27_col107))))) + (((op0_limb_26_col77) * (op1_limb_26_col106))))) + (((op0_limb_27_col78) * (op1_limb_25_col105))));
let conv_tmp_57455_115 = ((((M31_0) + (((op0_limb_26_col77) * (op1_limb_27_col107))))) + (((op0_limb_27_col78) * (op1_limb_26_col106))));
let conv_tmp_57455_116 = ((M31_0) + (((op0_limb_27_col78) * (op1_limb_27_col107))));
let conv_mod_tmp_57455_117 = ((((((M31_0) + (((M31_32) * (conv_tmp_57455_62))))) - (((M31_4) * (conv_tmp_57455_83))))) + (((M31_8) * (conv_tmp_57455_111))));
let conv_mod_tmp_57455_118 = ((((((((M31_0) + (((M31_1) * (conv_tmp_57455_62))))) + (((M31_32) * (conv_tmp_57455_63))))) - (((M31_4) * (conv_tmp_57455_84))))) + (((M31_8) * (conv_tmp_57455_112))));
let conv_mod_tmp_57455_119 = ((((((((M31_0) + (((M31_1) * (conv_tmp_57455_63))))) + (((M31_32) * (conv_tmp_57455_64))))) - (((M31_4) * (conv_tmp_57455_85))))) + (((M31_8) * (conv_tmp_57455_113))));
let conv_mod_tmp_57455_120 = ((((((((M31_0) + (((M31_1) * (conv_tmp_57455_64))))) + (((M31_32) * (conv_tmp_57455_65))))) - (((M31_4) * (conv_tmp_57455_86))))) + (((M31_8) * (conv_tmp_57455_114))));
let conv_mod_tmp_57455_121 = ((((((((M31_0) + (((M31_1) * (conv_tmp_57455_65))))) + (((M31_32) * (conv_tmp_57455_66))))) - (((M31_4) * (conv_tmp_57455_87))))) + (((M31_8) * (conv_tmp_57455_115))));
let conv_mod_tmp_57455_122 = ((((((((M31_0) + (((M31_1) * (conv_tmp_57455_66))))) + (((M31_32) * (conv_tmp_57455_67))))) - (((M31_4) * (conv_tmp_57455_88))))) + (((M31_8) * (conv_tmp_57455_116))));
let conv_mod_tmp_57455_123 = ((((((M31_0) + (((M31_1) * (conv_tmp_57455_67))))) + (((M31_32) * (conv_tmp_57455_68))))) - (((M31_4) * (conv_tmp_57455_89))));
let conv_mod_tmp_57455_124 = ((((((((M31_0) + (((M31_2) * (conv_tmp_57455_62))))) + (((M31_1) * (conv_tmp_57455_68))))) + (((M31_32) * (conv_tmp_57455_69))))) - (((M31_4) * (conv_tmp_57455_90))));
let conv_mod_tmp_57455_125 = ((((((((M31_0) + (((M31_2) * (conv_tmp_57455_63))))) + (((M31_1) * (conv_tmp_57455_69))))) + (((M31_32) * (conv_tmp_57455_70))))) - (((M31_4) * (conv_tmp_57455_91))));
let conv_mod_tmp_57455_126 = ((((((((M31_0) + (((M31_2) * (conv_tmp_57455_64))))) + (((M31_1) * (conv_tmp_57455_70))))) + (((M31_32) * (conv_tmp_57455_71))))) - (((M31_4) * (conv_tmp_57455_92))));
let conv_mod_tmp_57455_127 = ((((((((M31_0) + (((M31_2) * (conv_tmp_57455_65))))) + (((M31_1) * (conv_tmp_57455_71))))) + (((M31_32) * (conv_tmp_57455_72))))) - (((M31_4) * (conv_tmp_57455_93))));
let conv_mod_tmp_57455_128 = ((((((((M31_0) + (((M31_2) * (conv_tmp_57455_66))))) + (((M31_1) * (conv_tmp_57455_72))))) + (((M31_32) * (conv_tmp_57455_73))))) - (((M31_4) * (conv_tmp_57455_94))));
let conv_mod_tmp_57455_129 = ((((((((M31_0) + (((M31_2) * (conv_tmp_57455_67))))) + (((M31_1) * (conv_tmp_57455_73))))) + (((M31_32) * (conv_tmp_57455_74))))) - (((M31_4) * (conv_tmp_57455_95))));
let conv_mod_tmp_57455_130 = ((((((((M31_0) + (((M31_2) * (conv_tmp_57455_68))))) + (((M31_1) * (conv_tmp_57455_74))))) + (((M31_32) * (conv_tmp_57455_75))))) - (((M31_4) * (conv_tmp_57455_96))));
let conv_mod_tmp_57455_131 = ((((((((M31_0) + (((M31_2) * (conv_tmp_57455_69))))) + (((M31_1) * (conv_tmp_57455_75))))) + (((M31_32) * (conv_tmp_57455_76))))) - (((M31_4) * (conv_tmp_57455_97))));
let conv_mod_tmp_57455_132 = ((((((((M31_0) + (((M31_2) * (conv_tmp_57455_70))))) + (((M31_1) * (conv_tmp_57455_76))))) + (((M31_32) * (conv_tmp_57455_77))))) - (((M31_4) * (conv_tmp_57455_98))));
let conv_mod_tmp_57455_133 = ((((((((M31_0) + (((M31_2) * (conv_tmp_57455_71))))) + (((M31_1) * (conv_tmp_57455_77))))) + (((M31_32) * (conv_tmp_57455_78))))) - (((M31_4) * (conv_tmp_57455_99))));
let conv_mod_tmp_57455_134 = ((((((((M31_0) + (((M31_2) * (conv_tmp_57455_72))))) + (((M31_1) * (conv_tmp_57455_78))))) + (((M31_32) * (conv_tmp_57455_79))))) - (((M31_4) * (conv_tmp_57455_100))));
let conv_mod_tmp_57455_135 = ((((((((M31_0) + (((M31_2) * (conv_tmp_57455_73))))) + (((M31_1) * (conv_tmp_57455_79))))) + (((M31_32) * (conv_tmp_57455_80))))) - (((M31_4) * (conv_tmp_57455_101))));
let conv_mod_tmp_57455_136 = ((((((((M31_0) + (((M31_2) * (conv_tmp_57455_74))))) + (((M31_1) * (conv_tmp_57455_80))))) + (((M31_32) * (conv_tmp_57455_81))))) - (((M31_4) * (conv_tmp_57455_102))));
let conv_mod_tmp_57455_137 = ((((((((M31_0) + (((M31_2) * (conv_tmp_57455_75))))) + (((M31_1) * (conv_tmp_57455_81))))) + (((M31_32) * (conv_tmp_57455_82))))) - (((M31_4) * (conv_tmp_57455_103))));
let conv_mod_tmp_57455_138 = ((((((((M31_0) + (((M31_2) * (conv_tmp_57455_76))))) + (((M31_1) * (conv_tmp_57455_82))))) - (((M31_4) * (conv_tmp_57455_104))))) + (((M31_64) * (conv_tmp_57455_111))));
let conv_mod_tmp_57455_139 = ((((((((M31_0) + (((M31_2) * (conv_tmp_57455_77))))) - (((M31_4) * (conv_tmp_57455_105))))) + (((M31_2) * (conv_tmp_57455_111))))) + (((M31_64) * (conv_tmp_57455_112))));
let conv_mod_tmp_57455_140 = ((((((((M31_0) + (((M31_2) * (conv_tmp_57455_78))))) - (((M31_4) * (conv_tmp_57455_106))))) + (((M31_2) * (conv_tmp_57455_112))))) + (((M31_64) * (conv_tmp_57455_113))));
let conv_mod_tmp_57455_141 = ((((((((M31_0) + (((M31_2) * (conv_tmp_57455_79))))) - (((M31_4) * (conv_tmp_57455_107))))) + (((M31_2) * (conv_tmp_57455_113))))) + (((M31_64) * (conv_tmp_57455_114))));
let conv_mod_tmp_57455_142 = ((((((((M31_0) + (((M31_2) * (conv_tmp_57455_80))))) - (((M31_4) * (conv_tmp_57455_108))))) + (((M31_2) * (conv_tmp_57455_114))))) + (((M31_64) * (conv_tmp_57455_115))));
let conv_mod_tmp_57455_143 = ((((((((M31_0) + (((M31_2) * (conv_tmp_57455_81))))) - (((M31_4) * (conv_tmp_57455_109))))) + (((M31_2) * (conv_tmp_57455_115))))) + (((M31_64) * (conv_tmp_57455_116))));
let conv_mod_tmp_57455_144 = ((((((M31_0) + (((M31_2) * (conv_tmp_57455_82))))) - (((M31_4) * (conv_tmp_57455_110))))) + (((M31_2) * (conv_tmp_57455_116))));
let k_mod_2_18_biased_tmp_57455_145 = ((((((PackedUInt32::from_m31(((conv_mod_tmp_57455_117) + (M31_134217728)))) + (((((PackedUInt32::from_m31(((conv_mod_tmp_57455_118) + (M31_134217728)))) & (UInt32_511))) << (UInt32_9))))) + (UInt32_65536))) & (UInt32_262143));
let k_col165 = ((k_mod_2_18_biased_tmp_57455_145.low().as_m31()) + (((((k_mod_2_18_biased_tmp_57455_145.high().as_m31()) - (M31_1))) * (M31_65536))));
            *row[165] = k_col165;

for (i, &input) in [((k_col165) + (M31_262144))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[0] = input;
            }
*lookup_data.range_check_19_0 = [((k_col165) + (M31_262144))];
let carry_0_col166 = ((((((conv_mod_tmp_57455_117) - (((M31_1) * (k_col165))))) + (M31_0))) * (M31_4194304));
            *row[166] = carry_0_col166;

for (i, &input) in [((carry_0_col166) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[1] = input;
            }
*lookup_data.range_check_19_1 = [((carry_0_col166) + (M31_131072))];
let carry_1_col167 = ((((conv_mod_tmp_57455_118) + (carry_0_col166))) * (M31_4194304));
            *row[167] = carry_1_col167;

for (i, &input) in [((carry_1_col167) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[2] = input;
            }
*lookup_data.range_check_19_2 = [((carry_1_col167) + (M31_131072))];
let carry_2_col168 = ((((conv_mod_tmp_57455_119) + (carry_1_col167))) * (M31_4194304));
            *row[168] = carry_2_col168;

for (i, &input) in [((carry_2_col168) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[3] = input;
            }
*lookup_data.range_check_19_3 = [((carry_2_col168) + (M31_131072))];
let carry_3_col169 = ((((conv_mod_tmp_57455_120) + (carry_2_col168))) * (M31_4194304));
            *row[169] = carry_3_col169;

for (i, &input) in [((carry_3_col169) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[4] = input;
            }
*lookup_data.range_check_19_4 = [((carry_3_col169) + (M31_131072))];
let carry_4_col170 = ((((conv_mod_tmp_57455_121) + (carry_3_col169))) * (M31_4194304));
            *row[170] = carry_4_col170;

for (i, &input) in [((carry_4_col170) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[5] = input;
            }
*lookup_data.range_check_19_5 = [((carry_4_col170) + (M31_131072))];
let carry_5_col171 = ((((conv_mod_tmp_57455_122) + (carry_4_col170))) * (M31_4194304));
            *row[171] = carry_5_col171;

for (i, &input) in [((carry_5_col171) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[6] = input;
            }
*lookup_data.range_check_19_6 = [((carry_5_col171) + (M31_131072))];
let carry_6_col172 = ((((conv_mod_tmp_57455_123) + (carry_5_col171))) * (M31_4194304));
            *row[172] = carry_6_col172;

for (i, &input) in [((carry_6_col172) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[7] = input;
            }
*lookup_data.range_check_19_7 = [((carry_6_col172) + (M31_131072))];
let carry_7_col173 = ((((conv_mod_tmp_57455_124) + (carry_6_col172))) * (M31_4194304));
            *row[173] = carry_7_col173;

for (i, &input) in [((carry_7_col173) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[8] = input;
            }
*lookup_data.range_check_19_8 = [((carry_7_col173) + (M31_131072))];
let carry_8_col174 = ((((conv_mod_tmp_57455_125) + (carry_7_col173))) * (M31_4194304));
            *row[174] = carry_8_col174;

for (i, &input) in [((carry_8_col174) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[9] = input;
            }
*lookup_data.range_check_19_9 = [((carry_8_col174) + (M31_131072))];
let carry_9_col175 = ((((conv_mod_tmp_57455_126) + (carry_8_col174))) * (M31_4194304));
            *row[175] = carry_9_col175;

for (i, &input) in [((carry_9_col175) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[10] = input;
            }
*lookup_data.range_check_19_10 = [((carry_9_col175) + (M31_131072))];
let carry_10_col176 = ((((conv_mod_tmp_57455_127) + (carry_9_col175))) * (M31_4194304));
            *row[176] = carry_10_col176;

for (i, &input) in [((carry_10_col176) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[11] = input;
            }
*lookup_data.range_check_19_11 = [((carry_10_col176) + (M31_131072))];
let carry_11_col177 = ((((conv_mod_tmp_57455_128) + (carry_10_col176))) * (M31_4194304));
            *row[177] = carry_11_col177;

for (i, &input) in [((carry_11_col177) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[12] = input;
            }
*lookup_data.range_check_19_12 = [((carry_11_col177) + (M31_131072))];
let carry_12_col178 = ((((conv_mod_tmp_57455_129) + (carry_11_col177))) * (M31_4194304));
            *row[178] = carry_12_col178;

for (i, &input) in [((carry_12_col178) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[13] = input;
            }
*lookup_data.range_check_19_13 = [((carry_12_col178) + (M31_131072))];
let carry_13_col179 = ((((conv_mod_tmp_57455_130) + (carry_12_col178))) * (M31_4194304));
            *row[179] = carry_13_col179;

for (i, &input) in [((carry_13_col179) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[14] = input;
            }
*lookup_data.range_check_19_14 = [((carry_13_col179) + (M31_131072))];
let carry_14_col180 = ((((conv_mod_tmp_57455_131) + (carry_13_col179))) * (M31_4194304));
            *row[180] = carry_14_col180;

for (i, &input) in [((carry_14_col180) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[15] = input;
            }
*lookup_data.range_check_19_15 = [((carry_14_col180) + (M31_131072))];
let carry_15_col181 = ((((conv_mod_tmp_57455_132) + (carry_14_col180))) * (M31_4194304));
            *row[181] = carry_15_col181;

for (i, &input) in [((carry_15_col181) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[16] = input;
            }
*lookup_data.range_check_19_16 = [((carry_15_col181) + (M31_131072))];
let carry_16_col182 = ((((conv_mod_tmp_57455_133) + (carry_15_col181))) * (M31_4194304));
            *row[182] = carry_16_col182;

for (i, &input) in [((carry_16_col182) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[17] = input;
            }
*lookup_data.range_check_19_17 = [((carry_16_col182) + (M31_131072))];
let carry_17_col183 = ((((conv_mod_tmp_57455_134) + (carry_16_col182))) * (M31_4194304));
            *row[183] = carry_17_col183;

for (i, &input) in [((carry_17_col183) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[18] = input;
            }
*lookup_data.range_check_19_18 = [((carry_17_col183) + (M31_131072))];
let carry_18_col184 = ((((conv_mod_tmp_57455_135) + (carry_17_col183))) * (M31_4194304));
            *row[184] = carry_18_col184;

for (i, &input) in [((carry_18_col184) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[19] = input;
            }
*lookup_data.range_check_19_19 = [((carry_18_col184) + (M31_131072))];
let carry_19_col185 = ((((conv_mod_tmp_57455_136) + (carry_18_col184))) * (M31_4194304));
            *row[185] = carry_19_col185;

for (i, &input) in [((carry_19_col185) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[20] = input;
            }
*lookup_data.range_check_19_20 = [((carry_19_col185) + (M31_131072))];
let carry_20_col186 = ((((conv_mod_tmp_57455_137) + (carry_19_col185))) * (M31_4194304));
            *row[186] = carry_20_col186;

for (i, &input) in [((carry_20_col186) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[21] = input;
            }
*lookup_data.range_check_19_21 = [((carry_20_col186) + (M31_131072))];
let carry_21_col187 = ((((((conv_mod_tmp_57455_138) - (((M31_136) * (k_col165))))) + (carry_20_col186))) * (M31_4194304));
            *row[187] = carry_21_col187;

for (i, &input) in [((carry_21_col187) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[22] = input;
            }
*lookup_data.range_check_19_22 = [((carry_21_col187) + (M31_131072))];
let carry_22_col188 = ((((conv_mod_tmp_57455_139) + (carry_21_col187))) * (M31_4194304));
            *row[188] = carry_22_col188;

for (i, &input) in [((carry_22_col188) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[23] = input;
            }
*lookup_data.range_check_19_23 = [((carry_22_col188) + (M31_131072))];
let carry_23_col189 = ((((conv_mod_tmp_57455_140) + (carry_22_col188))) * (M31_4194304));
            *row[189] = carry_23_col189;

for (i, &input) in [((carry_23_col189) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[24] = input;
            }
*lookup_data.range_check_19_24 = [((carry_23_col189) + (M31_131072))];
let carry_24_col190 = ((((conv_mod_tmp_57455_141) + (carry_23_col189))) * (M31_4194304));
            *row[190] = carry_24_col190;

for (i, &input) in [((carry_24_col190) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[25] = input;
            }
*lookup_data.range_check_19_25 = [((carry_24_col190) + (M31_131072))];
let carry_25_col191 = ((((conv_mod_tmp_57455_142) + (carry_24_col190))) * (M31_4194304));
            *row[191] = carry_25_col191;

for (i, &input) in [((carry_25_col191) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[26] = input;
            }
*lookup_data.range_check_19_26 = [((carry_25_col191) + (M31_131072))];
let carry_26_col192 = ((((conv_mod_tmp_57455_143) + (carry_25_col191))) * (M31_4194304));
            *row[192] = carry_26_col192;

for (i, &input) in [((carry_26_col192) + (M31_131072))].unpack().iter().enumerate() {
                *sub_components_inputs[i]
                    .range_check_19_inputs[27] = input;
            }
*lookup_data.range_check_19_27 = [((carry_26_col192) + (M31_131072))];


            


            
let res_tmp_57455_146 = ((((((PackedFelt252::from_m31(res_op1_tmp_57455_22)) * (PackedFelt252::from_limbs([op1_limb_0_col80, op1_limb_1_col81, op1_limb_2_col82, op1_limb_3_col83, op1_limb_4_col84, op1_limb_5_col85, op1_limb_6_col86, op1_limb_7_col87, op1_limb_8_col88, op1_limb_9_col89, op1_limb_10_col90, op1_limb_11_col91, op1_limb_12_col92, op1_limb_13_col93, op1_limb_14_col94, op1_limb_15_col95, op1_limb_16_col96, op1_limb_17_col97, op1_limb_18_col98, op1_limb_19_col99, op1_limb_20_col100, op1_limb_21_col101, op1_limb_22_col102, op1_limb_23_col103, op1_limb_24_col104, op1_limb_25_col105, op1_limb_26_col106, op1_limb_27_col107])))) + (((PackedFelt252::from_m31(res_mul_col12)) * (mul_res_tmp_57455_61))))) + (((PackedFelt252::from_m31(res_add_col11)) * (add_res_tmp_57455_32))));
let res_limb_0_col193 = res_tmp_57455_146.get_m31(0);
            *row[193] = res_limb_0_col193;
let res_limb_1_col194 = res_tmp_57455_146.get_m31(1);
            *row[194] = res_limb_1_col194;
let res_limb_2_col195 = res_tmp_57455_146.get_m31(2);
            *row[195] = res_limb_2_col195;
let res_limb_3_col196 = res_tmp_57455_146.get_m31(3);
            *row[196] = res_limb_3_col196;
let res_limb_4_col197 = res_tmp_57455_146.get_m31(4);
            *row[197] = res_limb_4_col197;
let res_limb_5_col198 = res_tmp_57455_146.get_m31(5);
            *row[198] = res_limb_5_col198;
let res_limb_6_col199 = res_tmp_57455_146.get_m31(6);
            *row[199] = res_limb_6_col199;
let res_limb_7_col200 = res_tmp_57455_146.get_m31(7);
            *row[200] = res_limb_7_col200;
let res_limb_8_col201 = res_tmp_57455_146.get_m31(8);
            *row[201] = res_limb_8_col201;
let res_limb_9_col202 = res_tmp_57455_146.get_m31(9);
            *row[202] = res_limb_9_col202;
let res_limb_10_col203 = res_tmp_57455_146.get_m31(10);
            *row[203] = res_limb_10_col203;
let res_limb_11_col204 = res_tmp_57455_146.get_m31(11);
            *row[204] = res_limb_11_col204;
let res_limb_12_col205 = res_tmp_57455_146.get_m31(12);
            *row[205] = res_limb_12_col205;
let res_limb_13_col206 = res_tmp_57455_146.get_m31(13);
            *row[206] = res_limb_13_col206;
let res_limb_14_col207 = res_tmp_57455_146.get_m31(14);
            *row[207] = res_limb_14_col207;
let res_limb_15_col208 = res_tmp_57455_146.get_m31(15);
            *row[208] = res_limb_15_col208;
let res_limb_16_col209 = res_tmp_57455_146.get_m31(16);
            *row[209] = res_limb_16_col209;
let res_limb_17_col210 = res_tmp_57455_146.get_m31(17);
            *row[210] = res_limb_17_col210;
let res_limb_18_col211 = res_tmp_57455_146.get_m31(18);
            *row[211] = res_limb_18_col211;
let res_limb_19_col212 = res_tmp_57455_146.get_m31(19);
            *row[212] = res_limb_19_col212;
let res_limb_20_col213 = res_tmp_57455_146.get_m31(20);
            *row[213] = res_limb_20_col213;
let res_limb_21_col214 = res_tmp_57455_146.get_m31(21);
            *row[214] = res_limb_21_col214;
let res_limb_22_col215 = res_tmp_57455_146.get_m31(22);
            *row[215] = res_limb_22_col215;
let res_limb_23_col216 = res_tmp_57455_146.get_m31(23);
            *row[216] = res_limb_23_col216;
let res_limb_24_col217 = res_tmp_57455_146.get_m31(24);
            *row[217] = res_limb_24_col217;
let res_limb_25_col218 = res_tmp_57455_146.get_m31(25);
            *row[218] = res_limb_25_col218;
let res_limb_26_col219 = res_tmp_57455_146.get_m31(26);
            *row[219] = res_limb_26_col219;
let res_limb_27_col220 = res_tmp_57455_146.get_m31(27);
            *row[220] = res_limb_27_col220;


            


            //Update Registers.

            


            //Cond Felt 252 As Rel Imm.

            


            //Cond Decode Small Sign.

            
let msb_tmp_57455_148 = res_limb_27_col220.eq(M31_256);
let msb_col221 = msb_tmp_57455_148.as_m31();
            *row[221] = msb_col221;
let mid_limbs_set_tmp_57455_149 = res_limb_20_col213.eq(M31_511);
let mid_limbs_set_col222 = mid_limbs_set_tmp_57455_149.as_m31();
            *row[222] = mid_limbs_set_col222;


            


            
let diff_from_p_tmp_57455_150 = ((dst_limb_0_col22) - (M31_1));
let diff_from_p_tmp_57455_151 = ((dst_limb_21_col43) - (M31_136));
let diff_from_p_tmp_57455_152 = ((dst_limb_27_col49) - (M31_256));
let dst_sum_squares_inv_col223 = ((M31_1) .div (((((((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (((diff_from_p_tmp_57455_150) * (diff_from_p_tmp_57455_150))))) + (dst_limb_1_col23))) + (dst_limb_2_col24))) + (dst_limb_3_col25))) + (dst_limb_4_col26))) + (dst_limb_5_col27))) + (dst_limb_6_col28))) + (dst_limb_7_col29))) + (dst_limb_8_col30))) + (dst_limb_9_col31))) + (dst_limb_10_col32))) + (dst_limb_11_col33))) + (dst_limb_12_col34))) + (dst_limb_13_col35))) + (dst_limb_14_col36))) + (dst_limb_15_col37))) + (dst_limb_16_col38))) + (dst_limb_17_col39))) + (dst_limb_18_col40))) + (dst_limb_19_col41))) + (dst_limb_20_col42))) + (((diff_from_p_tmp_57455_151) * (diff_from_p_tmp_57455_151))))) + (dst_limb_22_col44))) + (dst_limb_23_col45))) + (dst_limb_24_col46))) + (dst_limb_25_col47))) + (dst_limb_26_col48))) + (((diff_from_p_tmp_57455_152) * (diff_from_p_tmp_57455_152))))));
            *row[223] = dst_sum_squares_inv_col223;
let dst_is_zero_tmp_57455_153 = ((((((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (dst_limb_0_col22))) + (dst_limb_1_col23))) + (dst_limb_2_col24))) + (dst_limb_3_col25))) + (dst_limb_4_col26))) + (dst_limb_5_col27))) + (dst_limb_6_col28))) + (dst_limb_7_col29))) + (dst_limb_8_col30))) + (dst_limb_9_col31))) + (dst_limb_10_col32))) + (dst_limb_11_col33))) + (dst_limb_12_col34))) + (dst_limb_13_col35))) + (dst_limb_14_col36))) + (dst_limb_15_col37))) + (dst_limb_16_col38))) + (dst_limb_17_col39))) + (dst_limb_18_col40))) + (dst_limb_19_col41))) + (dst_limb_20_col42))) + (dst_limb_21_col43))) + (dst_limb_22_col44))) + (dst_limb_23_col45))) + (dst_limb_24_col46))) + (dst_limb_25_col47))) + (dst_limb_26_col48))) + (dst_limb_27_col49)).eq(M31_0);
let dst_sum_inv_col224 = ((M31_1) .div (((((((((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (dst_limb_0_col22))) + (dst_limb_1_col23))) + (dst_limb_2_col24))) + (dst_limb_3_col25))) + (dst_limb_4_col26))) + (dst_limb_5_col27))) + (dst_limb_6_col28))) + (dst_limb_7_col29))) + (dst_limb_8_col30))) + (dst_limb_9_col31))) + (dst_limb_10_col32))) + (dst_limb_11_col33))) + (dst_limb_12_col34))) + (dst_limb_13_col35))) + (dst_limb_14_col36))) + (dst_limb_15_col37))) + (dst_limb_16_col38))) + (dst_limb_17_col39))) + (dst_limb_18_col40))) + (dst_limb_19_col41))) + (dst_limb_20_col42))) + (dst_limb_21_col43))) + (dst_limb_22_col44))) + (dst_limb_23_col45))) + (dst_limb_24_col46))) + (dst_limb_25_col47))) + (dst_limb_26_col48))) + (dst_limb_27_col49))) + (dst_is_zero_tmp_57455_153.as_m31()))));
            *row[224] = dst_sum_inv_col224;
let op1_as_rel_imm_cond_col225 = ((pc_update_jnz_col15) * (((((((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (dst_limb_0_col22))) + (dst_limb_1_col23))) + (dst_limb_2_col24))) + (dst_limb_3_col25))) + (dst_limb_4_col26))) + (dst_limb_5_col27))) + (dst_limb_6_col28))) + (dst_limb_7_col29))) + (dst_limb_8_col30))) + (dst_limb_9_col31))) + (dst_limb_10_col32))) + (dst_limb_11_col33))) + (dst_limb_12_col34))) + (dst_limb_13_col35))) + (dst_limb_14_col36))) + (dst_limb_15_col37))) + (dst_limb_16_col38))) + (dst_limb_17_col39))) + (dst_limb_18_col40))) + (dst_limb_19_col41))) + (dst_limb_20_col42))) + (dst_limb_21_col43))) + (dst_limb_22_col44))) + (dst_limb_23_col45))) + (dst_limb_24_col46))) + (dst_limb_25_col47))) + (dst_limb_26_col48))) + (dst_limb_27_col49))));
            *row[225] = op1_as_rel_imm_cond_col225;


            //Cond Felt 252 As Rel Imm.

            


            //Cond Decode Small Sign.

            
let msb_tmp_57455_154 = op1_limb_27_col107.eq(M31_256);
let msb_col226 = msb_tmp_57455_154.as_m31();
            *row[226] = msb_col226;
let mid_limbs_set_tmp_57455_155 = op1_limb_20_col100.eq(M31_511);
let mid_limbs_set_col227 = mid_limbs_set_tmp_57455_155.as_m31();
            *row[227] = mid_limbs_set_col227;


            


            
let next_pc_jnz_col228 = ((((dst_is_zero_tmp_57455_153.as_m31()) * (((input_pc_col0) + (((M31_1) + (op1_imm_col8))))))) + (((((M31_1) - (dst_is_zero_tmp_57455_153.as_m31()))) * (((input_pc_col0) + (((((((((op1_limb_0_col80) + (((op1_limb_1_col81) * (M31_512))))) + (((op1_limb_2_col82) * (M31_262144))))) - (msb_col226))) - (((M31_134217728) * (mid_limbs_set_col227))))))))));
            *row[228] = next_pc_jnz_col228;


            
*lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];
*lookup_data.opcodes_1 = [((((((((pc_update_regular_tmp_57455_23) * (((input_pc_col0) + (((M31_1) + (op1_imm_col8))))))) + (((pc_update_jump_col13) * (((((res_limb_0_col193) + (((res_limb_1_col194) * (M31_512))))) + (((res_limb_2_col195) * (M31_262144))))))))) + (((pc_update_jump_rel_col14) * (((input_pc_col0) + (((((((((res_limb_0_col193) + (((res_limb_1_col194) * (M31_512))))) + (((res_limb_2_col195) * (M31_262144))))) - (msb_col221))) - (((M31_134217728) * (mid_limbs_set_col222))))))))))) + (((pc_update_jnz_col15) * (next_pc_jnz_col228)))), ((((((input_ap_col1) + (((ap_update_add_col16) * (((((((((res_limb_0_col193) + (((res_limb_1_col194) * (M31_512))))) + (((res_limb_2_col195) * (M31_262144))))) - (msb_col221))) - (((M31_134217728) * (mid_limbs_set_col222))))))))) + (((ap_update_add_1_col17) * (M31_1))))) + (((opcode_call_col18) * (M31_2)))), ((((((fp_update_regular_tmp_57455_25) * (input_fp_col2))) + (((opcode_ret_col19) * (((((dst_limb_0_col22) + (((dst_limb_1_col23) * (M31_512))))) + (((dst_limb_2_col24) * (M31_262144))))))))) + (((opcode_call_col18) * (((input_ap_col1) + (M31_2))))))];

        });

    (trace, sub_components_inputs, lookup_data)
}

#[derive(Uninitialized,IterMut, ParIterMut)]
struct LookupData
{memory_address_to_id_0: Vec<[PackedM31; 2]>,memory_address_to_id_1: Vec<[PackedM31; 2]>,memory_address_to_id_2: Vec<[PackedM31; 2]>,memory_id_to_big_0: Vec<[PackedM31; 29]>,memory_id_to_big_1: Vec<[PackedM31; 29]>,memory_id_to_big_2: Vec<[PackedM31; 29]>,opcodes_0: Vec<[PackedM31; 3]>,opcodes_1: Vec<[PackedM31; 3]>,range_check_19_0: Vec<[PackedM31; 1]>,range_check_19_1: Vec<[PackedM31; 1]>,range_check_19_2: Vec<[PackedM31; 1]>,range_check_19_3: Vec<[PackedM31; 1]>,range_check_19_4: Vec<[PackedM31; 1]>,range_check_19_5: Vec<[PackedM31; 1]>,range_check_19_6: Vec<[PackedM31; 1]>,range_check_19_7: Vec<[PackedM31; 1]>,range_check_19_8: Vec<[PackedM31; 1]>,range_check_19_9: Vec<[PackedM31; 1]>,range_check_19_10: Vec<[PackedM31; 1]>,range_check_19_11: Vec<[PackedM31; 1]>,range_check_19_12: Vec<[PackedM31; 1]>,range_check_19_13: Vec<[PackedM31; 1]>,range_check_19_14: Vec<[PackedM31; 1]>,range_check_19_15: Vec<[PackedM31; 1]>,range_check_19_16: Vec<[PackedM31; 1]>,range_check_19_17: Vec<[PackedM31; 1]>,range_check_19_18: Vec<[PackedM31; 1]>,range_check_19_19: Vec<[PackedM31; 1]>,range_check_19_20: Vec<[PackedM31; 1]>,range_check_19_21: Vec<[PackedM31; 1]>,range_check_19_22: Vec<[PackedM31; 1]>,range_check_19_23: Vec<[PackedM31; 1]>,range_check_19_24: Vec<[PackedM31; 1]>,range_check_19_25: Vec<[PackedM31; 1]>,range_check_19_26: Vec<[PackedM31; 1]>,range_check_19_27: Vec<[PackedM31; 1]>,range_check_9_9_0: Vec<[PackedM31; 2]>,range_check_9_9_1: Vec<[PackedM31; 2]>,range_check_9_9_2: Vec<[PackedM31; 2]>,range_check_9_9_3: Vec<[PackedM31; 2]>,range_check_9_9_4: Vec<[PackedM31; 2]>,range_check_9_9_5: Vec<[PackedM31; 2]>,range_check_9_9_6: Vec<[PackedM31; 2]>,range_check_9_9_7: Vec<[PackedM31; 2]>,range_check_9_9_8: Vec<[PackedM31; 2]>,range_check_9_9_9: Vec<[PackedM31; 2]>,range_check_9_9_10: Vec<[PackedM31; 2]>,range_check_9_9_11: Vec<[PackedM31; 2]>,range_check_9_9_12: Vec<[PackedM31; 2]>,range_check_9_9_13: Vec<[PackedM31; 2]>,range_check_9_9_14: Vec<[PackedM31; 2]>,range_check_9_9_15: Vec<[PackedM31; 2]>,range_check_9_9_16: Vec<[PackedM31; 2]>,range_check_9_9_17: Vec<[PackedM31; 2]>,range_check_9_9_18: Vec<[PackedM31; 2]>,range_check_9_9_19: Vec<[PackedM31; 2]>,range_check_9_9_20: Vec<[PackedM31; 2]>,range_check_9_9_21: Vec<[PackedM31; 2]>,range_check_9_9_22: Vec<[PackedM31; 2]>,range_check_9_9_23: Vec<[PackedM31; 2]>,range_check_9_9_24: Vec<[PackedM31; 2]>,range_check_9_9_25: Vec<[PackedM31; 2]>,range_check_9_9_26: Vec<[PackedM31; 2]>,range_check_9_9_27: Vec<[PackedM31; 2]>,verify_instruction_0: Vec<[PackedM31; 19]>,}

pub struct InteractionClaimGenerator {
    n_calls: usize,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {

    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        memory_address_to_id:
            &relations::MemoryAddressToId,
        memory_id_to_big:
            &relations::MemoryIdToBig,
        opcodes:
            &relations::Opcodes,
        range_check_19:
            &relations::RangeCheck_19,
        range_check_9_9:
            &relations::RangeCheck_9_9,
        verify_instruction:
            &relations::VerifyInstruction,
    ) -> InteractionClaim
    where
        SimdBackend: BackendForChannel<MC>
    {
        let log_size = std::cmp::max(self.n_calls.next_power_of_two().ilog2(), LOG_N_LANES);
        let mut logup_gen = LogupTraceGenerator::new(log_size);

        //Sum logup terms in pairs.
let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .verify_instruction_0,
            &self.lookup_data
                .memory_address_to_id_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_instruction.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .memory_id_to_big_0,
            &self.lookup_data
                .memory_address_to_id_1,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .memory_id_to_big_1,
            &self.lookup_data
                .memory_address_to_id_2,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .memory_id_to_big_2,
            &self.lookup_data
                .range_check_9_9_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_9_9_1,
            &self.lookup_data
                .range_check_9_9_2,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_9_9_3,
            &self.lookup_data
                .range_check_9_9_4,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_9_9_5,
            &self.lookup_data
                .range_check_9_9_6,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_9_9_7,
            &self.lookup_data
                .range_check_9_9_8,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_9_9_9,
            &self.lookup_data
                .range_check_9_9_10,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_9_9_11,
            &self.lookup_data
                .range_check_9_9_12,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_9_9_13,
            &self.lookup_data
                .range_check_9_9_14,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_9_9_15,
            &self.lookup_data
                .range_check_9_9_16,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_9_9_17,
            &self.lookup_data
                .range_check_9_9_18,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_9_9_19,
            &self.lookup_data
                .range_check_9_9_20,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_9_9_21,
            &self.lookup_data
                .range_check_9_9_22,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_9_9_23,
            &self.lookup_data
                .range_check_9_9_24,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_9_9_25,
            &self.lookup_data
                .range_check_9_9_26,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_9_9_27,
            &self.lookup_data
                .range_check_19_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_19_1,
            &self.lookup_data
                .range_check_19_2,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_19_3,
            &self.lookup_data
                .range_check_19_4,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_19_5,
            &self.lookup_data
                .range_check_19_6,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_19_7,
            &self.lookup_data
                .range_check_19_8,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_19_9,
            &self.lookup_data
                .range_check_19_10,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_19_11,
            &self.lookup_data
                .range_check_19_12,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_19_13,
            &self.lookup_data
                .range_check_19_14,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_19_15,
            &self.lookup_data
                .range_check_19_16,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_19_17,
            &self.lookup_data
                .range_check_19_18,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_19_19,
            &self.lookup_data
                .range_check_19_20,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_19_21,
            &self.lookup_data
                .range_check_19_22,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_19_23,
            &self.lookup_data
                .range_check_19_24,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_19_25,
            &self.lookup_data
                .range_check_19_26,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data
                .range_check_19_27,
            &self.lookup_data
                .opcodes_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = opcodes.combine(values1);
            col_gen.write_frac(i,denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        //Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        for (i, values) in self.lookup_data
            .opcodes_1.iter().enumerate() {
            let denom =
                opcodes.combine(values);
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
            logup_sums: (total_sum,claimed_sum)
        }
    }
}
