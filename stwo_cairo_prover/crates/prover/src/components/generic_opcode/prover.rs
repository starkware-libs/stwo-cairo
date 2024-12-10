#![cfg_attr(rustfmt, rustfmt_skip)]#![allow(unused_parens)]
#![allow(unused_imports)]
use std::cell::UnsafeCell;
use std::rc::Rc;

use itertools::{chain, zip_eq, Itertools};
use num_traits::{One, Zero};
use prover_types::cpu::*;
use prover_types::simd::*;
#[cfg(feature = "parallel")]
use rayon::prelude::*;
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::constraint_framework::Relation;
use stwo_prover::core::air::Component;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::conversion::Unpack;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::utils::bit_reverse_coset_to_circle_domain_order;
use stwo_prover::core::vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher};

use super::component::{Claim, InteractionClaim};
use crate::components::pack_values;
use crate::components::ThreadSafeTrace;
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

    pub fn write_trace(
        mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        memory_address_to_id_state: &mut memory_address_to_id::ClaimGenerator,memory_id_to_big_state: &mut memory_id_to_big::ClaimGenerator,range_check_19_state: &mut range_check_19::ClaimGenerator,range_check_9_9_state: &mut range_check_9_9::ClaimGenerator,verify_instruction_state: &mut verify_instruction::ClaimGenerator,
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
        let (trace, mut sub_components_inputs, lookup_data) =
            write_trace_simd(packed_inputs, memory_address_to_id_state,memory_id_to_big_state,);

        if need_padding {
            sub_components_inputs.bit_reverse_coset_to_circle_domain_order();
        }
        sub_components_inputs.memory_address_to_id_inputs.iter_mut().for_each(|inputs| {
            memory_address_to_id_state.add_inputs(&inputs.get_mut()[..n_calls]);
        });sub_components_inputs.memory_id_to_big_inputs.iter_mut().for_each(|inputs| {
            memory_id_to_big_state.add_inputs(&inputs.get_mut()[..n_calls]);
        });sub_components_inputs.range_check_19_inputs.iter_mut().for_each(|inputs| {
            range_check_19_state.add_inputs(&inputs.get_mut()[..n_calls]);
        });sub_components_inputs.range_check_9_9_inputs.iter_mut().for_each(|inputs| {
            range_check_9_9_state.add_inputs(&inputs.get_mut()[..n_calls]);
        });sub_components_inputs.verify_instruction_inputs.iter_mut().for_each(|inputs| {
            verify_instruction_state.add_inputs(&inputs.get_mut()[..n_calls]);
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

pub struct SubComponentInputs
{pub memory_address_to_id_inputs: [UnsafeCell<Vec<memory_address_to_id::InputType>> ;3],pub memory_id_to_big_inputs: [UnsafeCell<Vec<memory_id_to_big::InputType>> ;3],pub range_check_19_inputs: [UnsafeCell<Vec<range_check_19::InputType>> ;28],pub range_check_9_9_inputs: [UnsafeCell<Vec<range_check_9_9::InputType>> ;28],pub verify_instruction_inputs: [UnsafeCell<Vec<verify_instruction::InputType>> ;1],}
impl SubComponentInputs {
    #[allow(unused_variables)]
    unsafe fn uninitialized(len: usize) -> Self {
        let sub_component_inputs = Self {memory_address_to_id_inputs: [Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),],memory_id_to_big_inputs: [Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),],range_check_19_inputs: [Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),],range_check_9_9_inputs: [Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),],verify_instruction_inputs: [Vec::with_capacity(len).into(),],};
        sub_component_inputs.memory_address_to_id_inputs
        .iter()
        .for_each(|vec| {
            (*vec.get()).set_len(len);
        });sub_component_inputs.memory_id_to_big_inputs
        .iter()
        .for_each(|vec| {
            (*vec.get()).set_len(len);
        });sub_component_inputs.range_check_19_inputs
        .iter()
        .for_each(|vec| {
            (*vec.get()).set_len(len);
        });sub_component_inputs.range_check_9_9_inputs
        .iter()
        .for_each(|vec| {
            (*vec.get()).set_len(len);
        });sub_component_inputs.verify_instruction_inputs
        .iter()
        .for_each(|vec| {
            (*vec.get()).set_len(len);
        });
        sub_component_inputs
    }

    fn bit_reverse_coset_to_circle_domain_order(&mut self) {
        self.memory_address_to_id_inputs
            .iter_mut()
            .for_each(|vec| bit_reverse_coset_to_circle_domain_order(vec.get_mut()));
        self.memory_id_to_big_inputs
            .iter_mut()
            .for_each(|vec| bit_reverse_coset_to_circle_domain_order(vec.get_mut()));
        self.range_check_19_inputs
            .iter_mut()
            .for_each(|vec| bit_reverse_coset_to_circle_domain_order(vec.get_mut()));
        self.range_check_9_9_inputs
            .iter_mut()
            .for_each(|vec| bit_reverse_coset_to_circle_domain_order(vec.get_mut()));
        self.verify_instruction_inputs
            .iter_mut()
            .for_each(|vec| bit_reverse_coset_to_circle_domain_order(vec.get_mut()));
    }
}

#[allow(clippy::useless_conversion)]

#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
pub fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    memory_address_to_id_state: &mut memory_address_to_id::ClaimGenerator,memory_id_to_big_state: &mut memory_id_to_big::ClaimGenerator,
) -> ([BaseColumn; N_TRACE_COLUMNS],
    SubComponentInputs,
    LookupData
) {
    let len = inputs.len();
    let cpu_len = len * N_LANES;
    let trace = Rc::new(ThreadSafeTrace::<N_TRACE_COLUMNS>::new(cpu_len));
    let lookup_data = Rc::new(unsafe { LookupData::uninitialized(len) });
    let sub_components_inputs = Rc::new(unsafe { SubComponentInputs::uninitialized(cpu_len) });

    let M31_0 = PackedM31::broadcast(M31::from(0));let M31_1 = PackedM31::broadcast(M31::from(1));let M31_131072 = PackedM31::broadcast(M31::from(131072));let M31_134217728 = PackedM31::broadcast(M31::from(134217728));let M31_136 = PackedM31::broadcast(M31::from(136));let M31_2 = PackedM31::broadcast(M31::from(2));let M31_256 = PackedM31::broadcast(M31::from(256));let M31_262144 = PackedM31::broadcast(M31::from(262144));let M31_32 = PackedM31::broadcast(M31::from(32));let M31_32768 = PackedM31::broadcast(M31::from(32768));let M31_4 = PackedM31::broadcast(M31::from(4));let M31_4194304 = PackedM31::broadcast(M31::from(4194304));let M31_511 = PackedM31::broadcast(M31::from(511));let M31_512 = PackedM31::broadcast(M31::from(512));let M31_64 = PackedM31::broadcast(M31::from(64));let M31_65536 = PackedM31::broadcast(M31::from(65536));let M31_8 = PackedM31::broadcast(M31::from(8));let UInt16_0 = PackedUInt16::broadcast(UInt16::from(0));let UInt16_1 = PackedUInt16::broadcast(UInt16::from(1));let UInt16_10 = PackedUInt16::broadcast(UInt16::from(10));let UInt16_11 = PackedUInt16::broadcast(UInt16::from(11));let UInt16_12 = PackedUInt16::broadcast(UInt16::from(12));let UInt16_127 = PackedUInt16::broadcast(UInt16::from(127));let UInt16_13 = PackedUInt16::broadcast(UInt16::from(13));let UInt16_14 = PackedUInt16::broadcast(UInt16::from(14));let UInt16_2 = PackedUInt16::broadcast(UInt16::from(2));let UInt16_3 = PackedUInt16::broadcast(UInt16::from(3));let UInt16_31 = PackedUInt16::broadcast(UInt16::from(31));let UInt16_4 = PackedUInt16::broadcast(UInt16::from(4));let UInt16_5 = PackedUInt16::broadcast(UInt16::from(5));let UInt16_6 = PackedUInt16::broadcast(UInt16::from(6));let UInt16_7 = PackedUInt16::broadcast(UInt16::from(7));let UInt16_8 = PackedUInt16::broadcast(UInt16::from(8));let UInt16_9 = PackedUInt16::broadcast(UInt16::from(9));let UInt32_262143 = PackedUInt32::broadcast(UInt32::from(262143));let UInt32_511 = PackedUInt32::broadcast(UInt32::from(511));let UInt32_65536 = PackedUInt32::broadcast(UInt32::from(65536));let UInt32_9 = PackedUInt32::broadcast(UInt32::from(9));

    #[cfg(not(feature = "parallel"))]
    let iter = inputs.into_iter().enumerate();

    #[cfg(feature = "parallel")]
    let iter = inputs.into_par_iter().enumerate();

    iter.for_each(|(row_index, generic_opcode_input)| {
        unsafe {
            let input_tmp_5745_0 = generic_opcode_input;
let input_pc_col0 = input_tmp_5745_0.pc;
            (*trace.data[0].get()).data[row_index] = input_pc_col0;
let input_ap_col1 = input_tmp_5745_0.ap;
            (*trace.data[1].get()).data[row_index] = input_ap_col1;
let input_fp_col2 = input_tmp_5745_0.fp;
            (*trace.data[2].get()).data[row_index] = input_fp_col2;


            //DecodeGenericInstruction.

            


            //DecodeInstruction_337193008ebaa578.

            
let memoryaddresstoid_value_tmp_5745_1 = memory_address_to_id_state.deduce_output(
                input_pc_col0
            );
let memoryidtobig_value_tmp_5745_2 = memory_id_to_big_state.deduce_output(
                memoryaddresstoid_value_tmp_5745_1
            );
let offset0_tmp_5745_3 = ((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(0))) + (((((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(1))) & (UInt16_127))) << (UInt16_9))));
let offset0_col3 = offset0_tmp_5745_3.as_m31();
            (*trace.data[3].get()).data[row_index] = offset0_col3;
let offset1_tmp_5745_4 = ((((((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(1))) >> (UInt16_7))) + (((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(2))) << (UInt16_2))))) + (((((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(3))) & (UInt16_31))) << (UInt16_11))));
let offset1_col4 = offset1_tmp_5745_4.as_m31();
            (*trace.data[4].get()).data[row_index] = offset1_col4;
let offset2_tmp_5745_5 = ((((((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(3))) >> (UInt16_5))) + (((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(4))) << (UInt16_4))))) + (((((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(5))) & (UInt16_7))) << (UInt16_13))));
let offset2_col5 = offset2_tmp_5745_5.as_m31();
            (*trace.data[5].get()).data[row_index] = offset2_col5;
let dst_base_fp_tmp_5745_6 = ((((((((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_0))) & (UInt16_1));
let dst_base_fp_col6 = dst_base_fp_tmp_5745_6.as_m31();
            (*trace.data[6].get()).data[row_index] = dst_base_fp_col6;
let op0_base_fp_tmp_5745_7 = ((((((((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_1))) & (UInt16_1));
let op0_base_fp_col7 = op0_base_fp_tmp_5745_7.as_m31();
            (*trace.data[7].get()).data[row_index] = op0_base_fp_col7;
let op1_imm_tmp_5745_8 = ((((((((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_2))) & (UInt16_1));
let op1_imm_col8 = op1_imm_tmp_5745_8.as_m31();
            (*trace.data[8].get()).data[row_index] = op1_imm_col8;
let op1_base_fp_tmp_5745_9 = ((((((((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_3))) & (UInt16_1));
let op1_base_fp_col9 = op1_base_fp_tmp_5745_9.as_m31();
            (*trace.data[9].get()).data[row_index] = op1_base_fp_col9;
let op1_base_ap_tmp_5745_10 = ((((((((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_4))) & (UInt16_1));
let op1_base_ap_col10 = op1_base_ap_tmp_5745_10.as_m31();
            (*trace.data[10].get()).data[row_index] = op1_base_ap_col10;
let res_add_tmp_5745_11 = ((((((((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_5))) & (UInt16_1));
let res_add_col11 = res_add_tmp_5745_11.as_m31();
            (*trace.data[11].get()).data[row_index] = res_add_col11;
let res_mul_tmp_5745_12 = ((((((((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_6))) & (UInt16_1));
let res_mul_col12 = res_mul_tmp_5745_12.as_m31();
            (*trace.data[12].get()).data[row_index] = res_mul_col12;
let pc_update_jump_tmp_5745_13 = ((((((((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_7))) & (UInt16_1));
let pc_update_jump_col13 = pc_update_jump_tmp_5745_13.as_m31();
            (*trace.data[13].get()).data[row_index] = pc_update_jump_col13;
let pc_update_jump_rel_tmp_5745_14 = ((((((((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_8))) & (UInt16_1));
let pc_update_jump_rel_col14 = pc_update_jump_rel_tmp_5745_14.as_m31();
            (*trace.data[14].get()).data[row_index] = pc_update_jump_rel_col14;
let pc_update_jnz_tmp_5745_15 = ((((((((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_9))) & (UInt16_1));
let pc_update_jnz_col15 = pc_update_jnz_tmp_5745_15.as_m31();
            (*trace.data[15].get()).data[row_index] = pc_update_jnz_col15;
let ap_update_add_tmp_5745_16 = ((((((((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_10))) & (UInt16_1));
let ap_update_add_col16 = ap_update_add_tmp_5745_16.as_m31();
            (*trace.data[16].get()).data[row_index] = ap_update_add_col16;
let ap_update_add_1_tmp_5745_17 = ((((((((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_11))) & (UInt16_1));
let ap_update_add_1_col17 = ap_update_add_1_tmp_5745_17.as_m31();
            (*trace.data[17].get()).data[row_index] = ap_update_add_1_col17;
let opcode_call_tmp_5745_18 = ((((((((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_12))) & (UInt16_1));
let opcode_call_col18 = opcode_call_tmp_5745_18.as_m31();
            (*trace.data[18].get()).data[row_index] = opcode_call_col18;
let opcode_ret_tmp_5745_19 = ((((((((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_13))) & (UInt16_1));
let opcode_ret_col19 = opcode_ret_tmp_5745_19.as_m31();
            (*trace.data[19].get()).data[row_index] = opcode_ret_col19;
let opcode_assert_eq_tmp_5745_20 = ((((((((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memoryidtobig_value_tmp_5745_2.get_m31(6))) << (UInt16_6))))) >> (UInt16_14))) & (UInt16_1));
let opcode_assert_eq_col20 = opcode_assert_eq_tmp_5745_20.as_m31();
            (*trace.data[20].get()).data[row_index] = opcode_assert_eq_col20;

(*sub_components_inputs
                .verify_instruction_inputs[0].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&(input_pc_col0, [offset0_col3, offset1_col4, offset2_col5], [dst_base_fp_col6, op0_base_fp_col7, op1_imm_col8, op1_base_fp_col9, op1_base_ap_col10, res_add_col11, res_mul_col12, pc_update_jump_col13, pc_update_jump_rel_col14, pc_update_jnz_col15, ap_update_add_col16, ap_update_add_1_col17, opcode_call_col18, opcode_ret_col19, opcode_assert_eq_col20]).unpack());
            
(*lookup_data.verifyinstruction[0].get())[row_index] = [input_pc_col0, offset0_col3, offset1_col4, offset2_col5, dst_base_fp_col6, op0_base_fp_col7, op1_imm_col8, op1_base_fp_col9, op1_base_ap_col10, res_add_col11, res_mul_col12, pc_update_jump_col13, pc_update_jump_rel_col14, pc_update_jnz_col15, ap_update_add_col16, ap_update_add_1_col17, opcode_call_col18, opcode_ret_col19, opcode_assert_eq_col20];


            
let op1_base_op0_tmp_5745_21 = ((((((M31_1) - (op1_imm_col8))) - (op1_base_fp_col9))) - (op1_base_ap_col10));
let res_op1_tmp_5745_22 = ((((((M31_1) - (res_add_col11))) - (res_mul_col12))) - (pc_update_jnz_col15));
let pc_update_regular_tmp_5745_23 = ((((((M31_1) - (pc_update_jump_col13))) - (pc_update_jump_rel_col14))) - (pc_update_jnz_col15));
let ap_update_regular_tmp_5745_24 = ((((((M31_1) - (ap_update_add_col16))) - (ap_update_add_1_col17))) - (opcode_call_col18));
let fp_update_regular_tmp_5745_25 = ((((M31_1) - (opcode_call_col18))) - (opcode_ret_col19));


            


            //EvalOperands.

            


            //ReadPositive_num_bits_252.

            
let memoryaddresstoid_value_tmp_5745_26 = memory_address_to_id_state.deduce_output(
                ((((((dst_base_fp_col6) * (input_fp_col2))) + (((((M31_1) - (dst_base_fp_col6))) * (input_ap_col1))))) + (((offset0_col3) - (M31_32768))))
            );
let memoryidtobig_value_tmp_5745_27 = memory_id_to_big_state.deduce_output(
                memoryaddresstoid_value_tmp_5745_26
            );
let dst_id_col21 = memoryaddresstoid_value_tmp_5745_26;
            (*trace.data[21].get()).data[row_index] = dst_id_col21;
(*sub_components_inputs
                .memory_address_to_id_inputs[0].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&((((((dst_base_fp_col6) * (input_fp_col2))) + (((((M31_1) - (dst_base_fp_col6))) * (input_ap_col1))))) + (((offset0_col3) - (M31_32768)))).unpack());
            
(*lookup_data.memoryaddresstoid[0].get())[row_index] = [((((((dst_base_fp_col6) * (input_fp_col2))) + (((((M31_1) - (dst_base_fp_col6))) * (input_ap_col1))))) + (((offset0_col3) - (M31_32768)))), dst_id_col21];
let dst_limb_0_col22 = memoryidtobig_value_tmp_5745_27.get_m31(0);
            (*trace.data[22].get()).data[row_index] = dst_limb_0_col22;
let dst_limb_1_col23 = memoryidtobig_value_tmp_5745_27.get_m31(1);
            (*trace.data[23].get()).data[row_index] = dst_limb_1_col23;
let dst_limb_2_col24 = memoryidtobig_value_tmp_5745_27.get_m31(2);
            (*trace.data[24].get()).data[row_index] = dst_limb_2_col24;
let dst_limb_3_col25 = memoryidtobig_value_tmp_5745_27.get_m31(3);
            (*trace.data[25].get()).data[row_index] = dst_limb_3_col25;
let dst_limb_4_col26 = memoryidtobig_value_tmp_5745_27.get_m31(4);
            (*trace.data[26].get()).data[row_index] = dst_limb_4_col26;
let dst_limb_5_col27 = memoryidtobig_value_tmp_5745_27.get_m31(5);
            (*trace.data[27].get()).data[row_index] = dst_limb_5_col27;
let dst_limb_6_col28 = memoryidtobig_value_tmp_5745_27.get_m31(6);
            (*trace.data[28].get()).data[row_index] = dst_limb_6_col28;
let dst_limb_7_col29 = memoryidtobig_value_tmp_5745_27.get_m31(7);
            (*trace.data[29].get()).data[row_index] = dst_limb_7_col29;
let dst_limb_8_col30 = memoryidtobig_value_tmp_5745_27.get_m31(8);
            (*trace.data[30].get()).data[row_index] = dst_limb_8_col30;
let dst_limb_9_col31 = memoryidtobig_value_tmp_5745_27.get_m31(9);
            (*trace.data[31].get()).data[row_index] = dst_limb_9_col31;
let dst_limb_10_col32 = memoryidtobig_value_tmp_5745_27.get_m31(10);
            (*trace.data[32].get()).data[row_index] = dst_limb_10_col32;
let dst_limb_11_col33 = memoryidtobig_value_tmp_5745_27.get_m31(11);
            (*trace.data[33].get()).data[row_index] = dst_limb_11_col33;
let dst_limb_12_col34 = memoryidtobig_value_tmp_5745_27.get_m31(12);
            (*trace.data[34].get()).data[row_index] = dst_limb_12_col34;
let dst_limb_13_col35 = memoryidtobig_value_tmp_5745_27.get_m31(13);
            (*trace.data[35].get()).data[row_index] = dst_limb_13_col35;
let dst_limb_14_col36 = memoryidtobig_value_tmp_5745_27.get_m31(14);
            (*trace.data[36].get()).data[row_index] = dst_limb_14_col36;
let dst_limb_15_col37 = memoryidtobig_value_tmp_5745_27.get_m31(15);
            (*trace.data[37].get()).data[row_index] = dst_limb_15_col37;
let dst_limb_16_col38 = memoryidtobig_value_tmp_5745_27.get_m31(16);
            (*trace.data[38].get()).data[row_index] = dst_limb_16_col38;
let dst_limb_17_col39 = memoryidtobig_value_tmp_5745_27.get_m31(17);
            (*trace.data[39].get()).data[row_index] = dst_limb_17_col39;
let dst_limb_18_col40 = memoryidtobig_value_tmp_5745_27.get_m31(18);
            (*trace.data[40].get()).data[row_index] = dst_limb_18_col40;
let dst_limb_19_col41 = memoryidtobig_value_tmp_5745_27.get_m31(19);
            (*trace.data[41].get()).data[row_index] = dst_limb_19_col41;
let dst_limb_20_col42 = memoryidtobig_value_tmp_5745_27.get_m31(20);
            (*trace.data[42].get()).data[row_index] = dst_limb_20_col42;
let dst_limb_21_col43 = memoryidtobig_value_tmp_5745_27.get_m31(21);
            (*trace.data[43].get()).data[row_index] = dst_limb_21_col43;
let dst_limb_22_col44 = memoryidtobig_value_tmp_5745_27.get_m31(22);
            (*trace.data[44].get()).data[row_index] = dst_limb_22_col44;
let dst_limb_23_col45 = memoryidtobig_value_tmp_5745_27.get_m31(23);
            (*trace.data[45].get()).data[row_index] = dst_limb_23_col45;
let dst_limb_24_col46 = memoryidtobig_value_tmp_5745_27.get_m31(24);
            (*trace.data[46].get()).data[row_index] = dst_limb_24_col46;
let dst_limb_25_col47 = memoryidtobig_value_tmp_5745_27.get_m31(25);
            (*trace.data[47].get()).data[row_index] = dst_limb_25_col47;
let dst_limb_26_col48 = memoryidtobig_value_tmp_5745_27.get_m31(26);
            (*trace.data[48].get()).data[row_index] = dst_limb_26_col48;
let dst_limb_27_col49 = memoryidtobig_value_tmp_5745_27.get_m31(27);
            (*trace.data[49].get()).data[row_index] = dst_limb_27_col49;
(*sub_components_inputs
                .memory_id_to_big_inputs[0].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&dst_id_col21.unpack());
            
(*lookup_data.memoryidtobig[0].get())[row_index] = [dst_id_col21, dst_limb_0_col22, dst_limb_1_col23, dst_limb_2_col24, dst_limb_3_col25, dst_limb_4_col26, dst_limb_5_col27, dst_limb_6_col28, dst_limb_7_col29, dst_limb_8_col30, dst_limb_9_col31, dst_limb_10_col32, dst_limb_11_col33, dst_limb_12_col34, dst_limb_13_col35, dst_limb_14_col36, dst_limb_15_col37, dst_limb_16_col38, dst_limb_17_col39, dst_limb_18_col40, dst_limb_19_col41, dst_limb_20_col42, dst_limb_21_col43, dst_limb_22_col44, dst_limb_23_col45, dst_limb_24_col46, dst_limb_25_col47, dst_limb_26_col48, dst_limb_27_col49];


            


            //ReadPositive_num_bits_252.

            
let memoryaddresstoid_value_tmp_5745_28 = memory_address_to_id_state.deduce_output(
                ((((((op0_base_fp_col7) * (input_fp_col2))) + (((((M31_1) - (op0_base_fp_col7))) * (input_ap_col1))))) + (((offset1_col4) - (M31_32768))))
            );
let memoryidtobig_value_tmp_5745_29 = memory_id_to_big_state.deduce_output(
                memoryaddresstoid_value_tmp_5745_28
            );
let op0_id_col50 = memoryaddresstoid_value_tmp_5745_28;
            (*trace.data[50].get()).data[row_index] = op0_id_col50;
(*sub_components_inputs
                .memory_address_to_id_inputs[1].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&((((((op0_base_fp_col7) * (input_fp_col2))) + (((((M31_1) - (op0_base_fp_col7))) * (input_ap_col1))))) + (((offset1_col4) - (M31_32768)))).unpack());
            
(*lookup_data.memoryaddresstoid[1].get())[row_index] = [((((((op0_base_fp_col7) * (input_fp_col2))) + (((((M31_1) - (op0_base_fp_col7))) * (input_ap_col1))))) + (((offset1_col4) - (M31_32768)))), op0_id_col50];
let op0_limb_0_col51 = memoryidtobig_value_tmp_5745_29.get_m31(0);
            (*trace.data[51].get()).data[row_index] = op0_limb_0_col51;
let op0_limb_1_col52 = memoryidtobig_value_tmp_5745_29.get_m31(1);
            (*trace.data[52].get()).data[row_index] = op0_limb_1_col52;
let op0_limb_2_col53 = memoryidtobig_value_tmp_5745_29.get_m31(2);
            (*trace.data[53].get()).data[row_index] = op0_limb_2_col53;
let op0_limb_3_col54 = memoryidtobig_value_tmp_5745_29.get_m31(3);
            (*trace.data[54].get()).data[row_index] = op0_limb_3_col54;
let op0_limb_4_col55 = memoryidtobig_value_tmp_5745_29.get_m31(4);
            (*trace.data[55].get()).data[row_index] = op0_limb_4_col55;
let op0_limb_5_col56 = memoryidtobig_value_tmp_5745_29.get_m31(5);
            (*trace.data[56].get()).data[row_index] = op0_limb_5_col56;
let op0_limb_6_col57 = memoryidtobig_value_tmp_5745_29.get_m31(6);
            (*trace.data[57].get()).data[row_index] = op0_limb_6_col57;
let op0_limb_7_col58 = memoryidtobig_value_tmp_5745_29.get_m31(7);
            (*trace.data[58].get()).data[row_index] = op0_limb_7_col58;
let op0_limb_8_col59 = memoryidtobig_value_tmp_5745_29.get_m31(8);
            (*trace.data[59].get()).data[row_index] = op0_limb_8_col59;
let op0_limb_9_col60 = memoryidtobig_value_tmp_5745_29.get_m31(9);
            (*trace.data[60].get()).data[row_index] = op0_limb_9_col60;
let op0_limb_10_col61 = memoryidtobig_value_tmp_5745_29.get_m31(10);
            (*trace.data[61].get()).data[row_index] = op0_limb_10_col61;
let op0_limb_11_col62 = memoryidtobig_value_tmp_5745_29.get_m31(11);
            (*trace.data[62].get()).data[row_index] = op0_limb_11_col62;
let op0_limb_12_col63 = memoryidtobig_value_tmp_5745_29.get_m31(12);
            (*trace.data[63].get()).data[row_index] = op0_limb_12_col63;
let op0_limb_13_col64 = memoryidtobig_value_tmp_5745_29.get_m31(13);
            (*trace.data[64].get()).data[row_index] = op0_limb_13_col64;
let op0_limb_14_col65 = memoryidtobig_value_tmp_5745_29.get_m31(14);
            (*trace.data[65].get()).data[row_index] = op0_limb_14_col65;
let op0_limb_15_col66 = memoryidtobig_value_tmp_5745_29.get_m31(15);
            (*trace.data[66].get()).data[row_index] = op0_limb_15_col66;
let op0_limb_16_col67 = memoryidtobig_value_tmp_5745_29.get_m31(16);
            (*trace.data[67].get()).data[row_index] = op0_limb_16_col67;
let op0_limb_17_col68 = memoryidtobig_value_tmp_5745_29.get_m31(17);
            (*trace.data[68].get()).data[row_index] = op0_limb_17_col68;
let op0_limb_18_col69 = memoryidtobig_value_tmp_5745_29.get_m31(18);
            (*trace.data[69].get()).data[row_index] = op0_limb_18_col69;
let op0_limb_19_col70 = memoryidtobig_value_tmp_5745_29.get_m31(19);
            (*trace.data[70].get()).data[row_index] = op0_limb_19_col70;
let op0_limb_20_col71 = memoryidtobig_value_tmp_5745_29.get_m31(20);
            (*trace.data[71].get()).data[row_index] = op0_limb_20_col71;
let op0_limb_21_col72 = memoryidtobig_value_tmp_5745_29.get_m31(21);
            (*trace.data[72].get()).data[row_index] = op0_limb_21_col72;
let op0_limb_22_col73 = memoryidtobig_value_tmp_5745_29.get_m31(22);
            (*trace.data[73].get()).data[row_index] = op0_limb_22_col73;
let op0_limb_23_col74 = memoryidtobig_value_tmp_5745_29.get_m31(23);
            (*trace.data[74].get()).data[row_index] = op0_limb_23_col74;
let op0_limb_24_col75 = memoryidtobig_value_tmp_5745_29.get_m31(24);
            (*trace.data[75].get()).data[row_index] = op0_limb_24_col75;
let op0_limb_25_col76 = memoryidtobig_value_tmp_5745_29.get_m31(25);
            (*trace.data[76].get()).data[row_index] = op0_limb_25_col76;
let op0_limb_26_col77 = memoryidtobig_value_tmp_5745_29.get_m31(26);
            (*trace.data[77].get()).data[row_index] = op0_limb_26_col77;
let op0_limb_27_col78 = memoryidtobig_value_tmp_5745_29.get_m31(27);
            (*trace.data[78].get()).data[row_index] = op0_limb_27_col78;
(*sub_components_inputs
                .memory_id_to_big_inputs[1].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&op0_id_col50.unpack());
            
(*lookup_data.memoryidtobig[1].get())[row_index] = [op0_id_col50, op0_limb_0_col51, op0_limb_1_col52, op0_limb_2_col53, op0_limb_3_col54, op0_limb_4_col55, op0_limb_5_col56, op0_limb_6_col57, op0_limb_7_col58, op0_limb_8_col59, op0_limb_9_col60, op0_limb_10_col61, op0_limb_11_col62, op0_limb_12_col63, op0_limb_13_col64, op0_limb_14_col65, op0_limb_15_col66, op0_limb_16_col67, op0_limb_17_col68, op0_limb_18_col69, op0_limb_19_col70, op0_limb_20_col71, op0_limb_21_col72, op0_limb_22_col73, op0_limb_23_col74, op0_limb_24_col75, op0_limb_25_col76, op0_limb_26_col77, op0_limb_27_col78];


            


            //ReadPositive_num_bits_252.

            
let memoryaddresstoid_value_tmp_5745_30 = memory_address_to_id_state.deduce_output(
                ((((((((((op1_base_fp_col9) * (input_fp_col2))) + (((op1_base_ap_col10) * (input_ap_col1))))) + (((op1_imm_col8) * (input_pc_col0))))) + (((op1_base_op0_tmp_5745_21) * (((((op0_limb_0_col51) + (((op0_limb_1_col52) * (M31_512))))) + (((op0_limb_2_col53) * (M31_262144))))))))) + (((offset2_col5) - (M31_32768))))
            );
let memoryidtobig_value_tmp_5745_31 = memory_id_to_big_state.deduce_output(
                memoryaddresstoid_value_tmp_5745_30
            );
let op1_id_col79 = memoryaddresstoid_value_tmp_5745_30;
            (*trace.data[79].get()).data[row_index] = op1_id_col79;
(*sub_components_inputs
                .memory_address_to_id_inputs[2].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&((((((((((op1_base_fp_col9) * (input_fp_col2))) + (((op1_base_ap_col10) * (input_ap_col1))))) + (((op1_imm_col8) * (input_pc_col0))))) + (((op1_base_op0_tmp_5745_21) * (((((op0_limb_0_col51) + (((op0_limb_1_col52) * (M31_512))))) + (((op0_limb_2_col53) * (M31_262144))))))))) + (((offset2_col5) - (M31_32768)))).unpack());
            
(*lookup_data.memoryaddresstoid[2].get())[row_index] = [((((((((((op1_base_fp_col9) * (input_fp_col2))) + (((op1_base_ap_col10) * (input_ap_col1))))) + (((op1_imm_col8) * (input_pc_col0))))) + (((op1_base_op0_tmp_5745_21) * (((((op0_limb_0_col51) + (((op0_limb_1_col52) * (M31_512))))) + (((op0_limb_2_col53) * (M31_262144))))))))) + (((offset2_col5) - (M31_32768)))), op1_id_col79];
let op1_limb_0_col80 = memoryidtobig_value_tmp_5745_31.get_m31(0);
            (*trace.data[80].get()).data[row_index] = op1_limb_0_col80;
let op1_limb_1_col81 = memoryidtobig_value_tmp_5745_31.get_m31(1);
            (*trace.data[81].get()).data[row_index] = op1_limb_1_col81;
let op1_limb_2_col82 = memoryidtobig_value_tmp_5745_31.get_m31(2);
            (*trace.data[82].get()).data[row_index] = op1_limb_2_col82;
let op1_limb_3_col83 = memoryidtobig_value_tmp_5745_31.get_m31(3);
            (*trace.data[83].get()).data[row_index] = op1_limb_3_col83;
let op1_limb_4_col84 = memoryidtobig_value_tmp_5745_31.get_m31(4);
            (*trace.data[84].get()).data[row_index] = op1_limb_4_col84;
let op1_limb_5_col85 = memoryidtobig_value_tmp_5745_31.get_m31(5);
            (*trace.data[85].get()).data[row_index] = op1_limb_5_col85;
let op1_limb_6_col86 = memoryidtobig_value_tmp_5745_31.get_m31(6);
            (*trace.data[86].get()).data[row_index] = op1_limb_6_col86;
let op1_limb_7_col87 = memoryidtobig_value_tmp_5745_31.get_m31(7);
            (*trace.data[87].get()).data[row_index] = op1_limb_7_col87;
let op1_limb_8_col88 = memoryidtobig_value_tmp_5745_31.get_m31(8);
            (*trace.data[88].get()).data[row_index] = op1_limb_8_col88;
let op1_limb_9_col89 = memoryidtobig_value_tmp_5745_31.get_m31(9);
            (*trace.data[89].get()).data[row_index] = op1_limb_9_col89;
let op1_limb_10_col90 = memoryidtobig_value_tmp_5745_31.get_m31(10);
            (*trace.data[90].get()).data[row_index] = op1_limb_10_col90;
let op1_limb_11_col91 = memoryidtobig_value_tmp_5745_31.get_m31(11);
            (*trace.data[91].get()).data[row_index] = op1_limb_11_col91;
let op1_limb_12_col92 = memoryidtobig_value_tmp_5745_31.get_m31(12);
            (*trace.data[92].get()).data[row_index] = op1_limb_12_col92;
let op1_limb_13_col93 = memoryidtobig_value_tmp_5745_31.get_m31(13);
            (*trace.data[93].get()).data[row_index] = op1_limb_13_col93;
let op1_limb_14_col94 = memoryidtobig_value_tmp_5745_31.get_m31(14);
            (*trace.data[94].get()).data[row_index] = op1_limb_14_col94;
let op1_limb_15_col95 = memoryidtobig_value_tmp_5745_31.get_m31(15);
            (*trace.data[95].get()).data[row_index] = op1_limb_15_col95;
let op1_limb_16_col96 = memoryidtobig_value_tmp_5745_31.get_m31(16);
            (*trace.data[96].get()).data[row_index] = op1_limb_16_col96;
let op1_limb_17_col97 = memoryidtobig_value_tmp_5745_31.get_m31(17);
            (*trace.data[97].get()).data[row_index] = op1_limb_17_col97;
let op1_limb_18_col98 = memoryidtobig_value_tmp_5745_31.get_m31(18);
            (*trace.data[98].get()).data[row_index] = op1_limb_18_col98;
let op1_limb_19_col99 = memoryidtobig_value_tmp_5745_31.get_m31(19);
            (*trace.data[99].get()).data[row_index] = op1_limb_19_col99;
let op1_limb_20_col100 = memoryidtobig_value_tmp_5745_31.get_m31(20);
            (*trace.data[100].get()).data[row_index] = op1_limb_20_col100;
let op1_limb_21_col101 = memoryidtobig_value_tmp_5745_31.get_m31(21);
            (*trace.data[101].get()).data[row_index] = op1_limb_21_col101;
let op1_limb_22_col102 = memoryidtobig_value_tmp_5745_31.get_m31(22);
            (*trace.data[102].get()).data[row_index] = op1_limb_22_col102;
let op1_limb_23_col103 = memoryidtobig_value_tmp_5745_31.get_m31(23);
            (*trace.data[103].get()).data[row_index] = op1_limb_23_col103;
let op1_limb_24_col104 = memoryidtobig_value_tmp_5745_31.get_m31(24);
            (*trace.data[104].get()).data[row_index] = op1_limb_24_col104;
let op1_limb_25_col105 = memoryidtobig_value_tmp_5745_31.get_m31(25);
            (*trace.data[105].get()).data[row_index] = op1_limb_25_col105;
let op1_limb_26_col106 = memoryidtobig_value_tmp_5745_31.get_m31(26);
            (*trace.data[106].get()).data[row_index] = op1_limb_26_col106;
let op1_limb_27_col107 = memoryidtobig_value_tmp_5745_31.get_m31(27);
            (*trace.data[107].get()).data[row_index] = op1_limb_27_col107;
(*sub_components_inputs
                .memory_id_to_big_inputs[2].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&op1_id_col79.unpack());
            
(*lookup_data.memoryidtobig[2].get())[row_index] = [op1_id_col79, op1_limb_0_col80, op1_limb_1_col81, op1_limb_2_col82, op1_limb_3_col83, op1_limb_4_col84, op1_limb_5_col85, op1_limb_6_col86, op1_limb_7_col87, op1_limb_8_col88, op1_limb_9_col89, op1_limb_10_col90, op1_limb_11_col91, op1_limb_12_col92, op1_limb_13_col93, op1_limb_14_col94, op1_limb_15_col95, op1_limb_16_col96, op1_limb_17_col97, op1_limb_18_col98, op1_limb_19_col99, op1_limb_20_col100, op1_limb_21_col101, op1_limb_22_col102, op1_limb_23_col103, op1_limb_24_col104, op1_limb_25_col105, op1_limb_26_col106, op1_limb_27_col107];


            


            //Add252.

            
let add_res_tmp_5745_32 = ((PackedFelt252::from_limbs([op0_limb_0_col51, op0_limb_1_col52, op0_limb_2_col53, op0_limb_3_col54, op0_limb_4_col55, op0_limb_5_col56, op0_limb_6_col57, op0_limb_7_col58, op0_limb_8_col59, op0_limb_9_col60, op0_limb_10_col61, op0_limb_11_col62, op0_limb_12_col63, op0_limb_13_col64, op0_limb_14_col65, op0_limb_15_col66, op0_limb_16_col67, op0_limb_17_col68, op0_limb_18_col69, op0_limb_19_col70, op0_limb_20_col71, op0_limb_21_col72, op0_limb_22_col73, op0_limb_23_col74, op0_limb_24_col75, op0_limb_25_col76, op0_limb_26_col77, op0_limb_27_col78])) + (PackedFelt252::from_limbs([op1_limb_0_col80, op1_limb_1_col81, op1_limb_2_col82, op1_limb_3_col83, op1_limb_4_col84, op1_limb_5_col85, op1_limb_6_col86, op1_limb_7_col87, op1_limb_8_col88, op1_limb_9_col89, op1_limb_10_col90, op1_limb_11_col91, op1_limb_12_col92, op1_limb_13_col93, op1_limb_14_col94, op1_limb_15_col95, op1_limb_16_col96, op1_limb_17_col97, op1_limb_18_col98, op1_limb_19_col99, op1_limb_20_col100, op1_limb_21_col101, op1_limb_22_col102, op1_limb_23_col103, op1_limb_24_col104, op1_limb_25_col105, op1_limb_26_col106, op1_limb_27_col107])));
let add_res_limb_0_col108 = add_res_tmp_5745_32.get_m31(0);
            (*trace.data[108].get()).data[row_index] = add_res_limb_0_col108;
let add_res_limb_1_col109 = add_res_tmp_5745_32.get_m31(1);
            (*trace.data[109].get()).data[row_index] = add_res_limb_1_col109;
let add_res_limb_2_col110 = add_res_tmp_5745_32.get_m31(2);
            (*trace.data[110].get()).data[row_index] = add_res_limb_2_col110;
let add_res_limb_3_col111 = add_res_tmp_5745_32.get_m31(3);
            (*trace.data[111].get()).data[row_index] = add_res_limb_3_col111;
let add_res_limb_4_col112 = add_res_tmp_5745_32.get_m31(4);
            (*trace.data[112].get()).data[row_index] = add_res_limb_4_col112;
let add_res_limb_5_col113 = add_res_tmp_5745_32.get_m31(5);
            (*trace.data[113].get()).data[row_index] = add_res_limb_5_col113;
let add_res_limb_6_col114 = add_res_tmp_5745_32.get_m31(6);
            (*trace.data[114].get()).data[row_index] = add_res_limb_6_col114;
let add_res_limb_7_col115 = add_res_tmp_5745_32.get_m31(7);
            (*trace.data[115].get()).data[row_index] = add_res_limb_7_col115;
let add_res_limb_8_col116 = add_res_tmp_5745_32.get_m31(8);
            (*trace.data[116].get()).data[row_index] = add_res_limb_8_col116;
let add_res_limb_9_col117 = add_res_tmp_5745_32.get_m31(9);
            (*trace.data[117].get()).data[row_index] = add_res_limb_9_col117;
let add_res_limb_10_col118 = add_res_tmp_5745_32.get_m31(10);
            (*trace.data[118].get()).data[row_index] = add_res_limb_10_col118;
let add_res_limb_11_col119 = add_res_tmp_5745_32.get_m31(11);
            (*trace.data[119].get()).data[row_index] = add_res_limb_11_col119;
let add_res_limb_12_col120 = add_res_tmp_5745_32.get_m31(12);
            (*trace.data[120].get()).data[row_index] = add_res_limb_12_col120;
let add_res_limb_13_col121 = add_res_tmp_5745_32.get_m31(13);
            (*trace.data[121].get()).data[row_index] = add_res_limb_13_col121;
let add_res_limb_14_col122 = add_res_tmp_5745_32.get_m31(14);
            (*trace.data[122].get()).data[row_index] = add_res_limb_14_col122;
let add_res_limb_15_col123 = add_res_tmp_5745_32.get_m31(15);
            (*trace.data[123].get()).data[row_index] = add_res_limb_15_col123;
let add_res_limb_16_col124 = add_res_tmp_5745_32.get_m31(16);
            (*trace.data[124].get()).data[row_index] = add_res_limb_16_col124;
let add_res_limb_17_col125 = add_res_tmp_5745_32.get_m31(17);
            (*trace.data[125].get()).data[row_index] = add_res_limb_17_col125;
let add_res_limb_18_col126 = add_res_tmp_5745_32.get_m31(18);
            (*trace.data[126].get()).data[row_index] = add_res_limb_18_col126;
let add_res_limb_19_col127 = add_res_tmp_5745_32.get_m31(19);
            (*trace.data[127].get()).data[row_index] = add_res_limb_19_col127;
let add_res_limb_20_col128 = add_res_tmp_5745_32.get_m31(20);
            (*trace.data[128].get()).data[row_index] = add_res_limb_20_col128;
let add_res_limb_21_col129 = add_res_tmp_5745_32.get_m31(21);
            (*trace.data[129].get()).data[row_index] = add_res_limb_21_col129;
let add_res_limb_22_col130 = add_res_tmp_5745_32.get_m31(22);
            (*trace.data[130].get()).data[row_index] = add_res_limb_22_col130;
let add_res_limb_23_col131 = add_res_tmp_5745_32.get_m31(23);
            (*trace.data[131].get()).data[row_index] = add_res_limb_23_col131;
let add_res_limb_24_col132 = add_res_tmp_5745_32.get_m31(24);
            (*trace.data[132].get()).data[row_index] = add_res_limb_24_col132;
let add_res_limb_25_col133 = add_res_tmp_5745_32.get_m31(25);
            (*trace.data[133].get()).data[row_index] = add_res_limb_25_col133;
let add_res_limb_26_col134 = add_res_tmp_5745_32.get_m31(26);
            (*trace.data[134].get()).data[row_index] = add_res_limb_26_col134;
let add_res_limb_27_col135 = add_res_tmp_5745_32.get_m31(27);
            (*trace.data[135].get()).data[row_index] = add_res_limb_27_col135;


            //RangeCheckBigValue.

            

(*sub_components_inputs
                .range_check_9_9_inputs[0].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[add_res_limb_0_col108, add_res_limb_1_col109].unpack());
            
(*lookup_data.rangecheck_9_9[0].get())[row_index] = [add_res_limb_0_col108, add_res_limb_1_col109];

(*sub_components_inputs
                .range_check_9_9_inputs[1].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[add_res_limb_2_col110, add_res_limb_3_col111].unpack());
            
(*lookup_data.rangecheck_9_9[1].get())[row_index] = [add_res_limb_2_col110, add_res_limb_3_col111];

(*sub_components_inputs
                .range_check_9_9_inputs[2].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[add_res_limb_4_col112, add_res_limb_5_col113].unpack());
            
(*lookup_data.rangecheck_9_9[2].get())[row_index] = [add_res_limb_4_col112, add_res_limb_5_col113];

(*sub_components_inputs
                .range_check_9_9_inputs[3].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[add_res_limb_6_col114, add_res_limb_7_col115].unpack());
            
(*lookup_data.rangecheck_9_9[3].get())[row_index] = [add_res_limb_6_col114, add_res_limb_7_col115];

(*sub_components_inputs
                .range_check_9_9_inputs[4].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[add_res_limb_8_col116, add_res_limb_9_col117].unpack());
            
(*lookup_data.rangecheck_9_9[4].get())[row_index] = [add_res_limb_8_col116, add_res_limb_9_col117];

(*sub_components_inputs
                .range_check_9_9_inputs[5].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[add_res_limb_10_col118, add_res_limb_11_col119].unpack());
            
(*lookup_data.rangecheck_9_9[5].get())[row_index] = [add_res_limb_10_col118, add_res_limb_11_col119];

(*sub_components_inputs
                .range_check_9_9_inputs[6].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[add_res_limb_12_col120, add_res_limb_13_col121].unpack());
            
(*lookup_data.rangecheck_9_9[6].get())[row_index] = [add_res_limb_12_col120, add_res_limb_13_col121];

(*sub_components_inputs
                .range_check_9_9_inputs[7].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[add_res_limb_14_col122, add_res_limb_15_col123].unpack());
            
(*lookup_data.rangecheck_9_9[7].get())[row_index] = [add_res_limb_14_col122, add_res_limb_15_col123];

(*sub_components_inputs
                .range_check_9_9_inputs[8].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[add_res_limb_16_col124, add_res_limb_17_col125].unpack());
            
(*lookup_data.rangecheck_9_9[8].get())[row_index] = [add_res_limb_16_col124, add_res_limb_17_col125];

(*sub_components_inputs
                .range_check_9_9_inputs[9].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[add_res_limb_18_col126, add_res_limb_19_col127].unpack());
            
(*lookup_data.rangecheck_9_9[9].get())[row_index] = [add_res_limb_18_col126, add_res_limb_19_col127];

(*sub_components_inputs
                .range_check_9_9_inputs[10].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[add_res_limb_20_col128, add_res_limb_21_col129].unpack());
            
(*lookup_data.rangecheck_9_9[10].get())[row_index] = [add_res_limb_20_col128, add_res_limb_21_col129];

(*sub_components_inputs
                .range_check_9_9_inputs[11].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[add_res_limb_22_col130, add_res_limb_23_col131].unpack());
            
(*lookup_data.rangecheck_9_9[11].get())[row_index] = [add_res_limb_22_col130, add_res_limb_23_col131];

(*sub_components_inputs
                .range_check_9_9_inputs[12].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[add_res_limb_24_col132, add_res_limb_25_col133].unpack());
            
(*lookup_data.rangecheck_9_9[12].get())[row_index] = [add_res_limb_24_col132, add_res_limb_25_col133];

(*sub_components_inputs
                .range_check_9_9_inputs[13].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[add_res_limb_26_col134, add_res_limb_27_col135].unpack());
            
(*lookup_data.rangecheck_9_9[13].get())[row_index] = [add_res_limb_26_col134, add_res_limb_27_col135];


            


            //VerifyAdd252.

            
let sub_p_bit_tmp_5745_33 = ((UInt16_1) & (((((PackedUInt16::from_m31(op0_limb_0_col51)) ^ (PackedUInt16::from_m31(op1_limb_0_col80)))) ^ (PackedUInt16::from_m31(add_res_limb_0_col108)))));
let sub_p_bit_col136 = sub_p_bit_tmp_5745_33.as_m31();
            (*trace.data[136].get()).data[row_index] = sub_p_bit_col136;


            


            


            //Mul252.

            
let mul_res_tmp_5745_61 = ((PackedFelt252::from_limbs([op0_limb_0_col51, op0_limb_1_col52, op0_limb_2_col53, op0_limb_3_col54, op0_limb_4_col55, op0_limb_5_col56, op0_limb_6_col57, op0_limb_7_col58, op0_limb_8_col59, op0_limb_9_col60, op0_limb_10_col61, op0_limb_11_col62, op0_limb_12_col63, op0_limb_13_col64, op0_limb_14_col65, op0_limb_15_col66, op0_limb_16_col67, op0_limb_17_col68, op0_limb_18_col69, op0_limb_19_col70, op0_limb_20_col71, op0_limb_21_col72, op0_limb_22_col73, op0_limb_23_col74, op0_limb_24_col75, op0_limb_25_col76, op0_limb_26_col77, op0_limb_27_col78])) * (PackedFelt252::from_limbs([op1_limb_0_col80, op1_limb_1_col81, op1_limb_2_col82, op1_limb_3_col83, op1_limb_4_col84, op1_limb_5_col85, op1_limb_6_col86, op1_limb_7_col87, op1_limb_8_col88, op1_limb_9_col89, op1_limb_10_col90, op1_limb_11_col91, op1_limb_12_col92, op1_limb_13_col93, op1_limb_14_col94, op1_limb_15_col95, op1_limb_16_col96, op1_limb_17_col97, op1_limb_18_col98, op1_limb_19_col99, op1_limb_20_col100, op1_limb_21_col101, op1_limb_22_col102, op1_limb_23_col103, op1_limb_24_col104, op1_limb_25_col105, op1_limb_26_col106, op1_limb_27_col107])));
let mul_res_limb_0_col137 = mul_res_tmp_5745_61.get_m31(0);
            (*trace.data[137].get()).data[row_index] = mul_res_limb_0_col137;
let mul_res_limb_1_col138 = mul_res_tmp_5745_61.get_m31(1);
            (*trace.data[138].get()).data[row_index] = mul_res_limb_1_col138;
let mul_res_limb_2_col139 = mul_res_tmp_5745_61.get_m31(2);
            (*trace.data[139].get()).data[row_index] = mul_res_limb_2_col139;
let mul_res_limb_3_col140 = mul_res_tmp_5745_61.get_m31(3);
            (*trace.data[140].get()).data[row_index] = mul_res_limb_3_col140;
let mul_res_limb_4_col141 = mul_res_tmp_5745_61.get_m31(4);
            (*trace.data[141].get()).data[row_index] = mul_res_limb_4_col141;
let mul_res_limb_5_col142 = mul_res_tmp_5745_61.get_m31(5);
            (*trace.data[142].get()).data[row_index] = mul_res_limb_5_col142;
let mul_res_limb_6_col143 = mul_res_tmp_5745_61.get_m31(6);
            (*trace.data[143].get()).data[row_index] = mul_res_limb_6_col143;
let mul_res_limb_7_col144 = mul_res_tmp_5745_61.get_m31(7);
            (*trace.data[144].get()).data[row_index] = mul_res_limb_7_col144;
let mul_res_limb_8_col145 = mul_res_tmp_5745_61.get_m31(8);
            (*trace.data[145].get()).data[row_index] = mul_res_limb_8_col145;
let mul_res_limb_9_col146 = mul_res_tmp_5745_61.get_m31(9);
            (*trace.data[146].get()).data[row_index] = mul_res_limb_9_col146;
let mul_res_limb_10_col147 = mul_res_tmp_5745_61.get_m31(10);
            (*trace.data[147].get()).data[row_index] = mul_res_limb_10_col147;
let mul_res_limb_11_col148 = mul_res_tmp_5745_61.get_m31(11);
            (*trace.data[148].get()).data[row_index] = mul_res_limb_11_col148;
let mul_res_limb_12_col149 = mul_res_tmp_5745_61.get_m31(12);
            (*trace.data[149].get()).data[row_index] = mul_res_limb_12_col149;
let mul_res_limb_13_col150 = mul_res_tmp_5745_61.get_m31(13);
            (*trace.data[150].get()).data[row_index] = mul_res_limb_13_col150;
let mul_res_limb_14_col151 = mul_res_tmp_5745_61.get_m31(14);
            (*trace.data[151].get()).data[row_index] = mul_res_limb_14_col151;
let mul_res_limb_15_col152 = mul_res_tmp_5745_61.get_m31(15);
            (*trace.data[152].get()).data[row_index] = mul_res_limb_15_col152;
let mul_res_limb_16_col153 = mul_res_tmp_5745_61.get_m31(16);
            (*trace.data[153].get()).data[row_index] = mul_res_limb_16_col153;
let mul_res_limb_17_col154 = mul_res_tmp_5745_61.get_m31(17);
            (*trace.data[154].get()).data[row_index] = mul_res_limb_17_col154;
let mul_res_limb_18_col155 = mul_res_tmp_5745_61.get_m31(18);
            (*trace.data[155].get()).data[row_index] = mul_res_limb_18_col155;
let mul_res_limb_19_col156 = mul_res_tmp_5745_61.get_m31(19);
            (*trace.data[156].get()).data[row_index] = mul_res_limb_19_col156;
let mul_res_limb_20_col157 = mul_res_tmp_5745_61.get_m31(20);
            (*trace.data[157].get()).data[row_index] = mul_res_limb_20_col157;
let mul_res_limb_21_col158 = mul_res_tmp_5745_61.get_m31(21);
            (*trace.data[158].get()).data[row_index] = mul_res_limb_21_col158;
let mul_res_limb_22_col159 = mul_res_tmp_5745_61.get_m31(22);
            (*trace.data[159].get()).data[row_index] = mul_res_limb_22_col159;
let mul_res_limb_23_col160 = mul_res_tmp_5745_61.get_m31(23);
            (*trace.data[160].get()).data[row_index] = mul_res_limb_23_col160;
let mul_res_limb_24_col161 = mul_res_tmp_5745_61.get_m31(24);
            (*trace.data[161].get()).data[row_index] = mul_res_limb_24_col161;
let mul_res_limb_25_col162 = mul_res_tmp_5745_61.get_m31(25);
            (*trace.data[162].get()).data[row_index] = mul_res_limb_25_col162;
let mul_res_limb_26_col163 = mul_res_tmp_5745_61.get_m31(26);
            (*trace.data[163].get()).data[row_index] = mul_res_limb_26_col163;
let mul_res_limb_27_col164 = mul_res_tmp_5745_61.get_m31(27);
            (*trace.data[164].get()).data[row_index] = mul_res_limb_27_col164;


            //RangeCheckBigValue.

            

(*sub_components_inputs
                .range_check_9_9_inputs[14].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[mul_res_limb_0_col137, mul_res_limb_1_col138].unpack());
            
(*lookup_data.rangecheck_9_9[14].get())[row_index] = [mul_res_limb_0_col137, mul_res_limb_1_col138];

(*sub_components_inputs
                .range_check_9_9_inputs[15].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[mul_res_limb_2_col139, mul_res_limb_3_col140].unpack());
            
(*lookup_data.rangecheck_9_9[15].get())[row_index] = [mul_res_limb_2_col139, mul_res_limb_3_col140];

(*sub_components_inputs
                .range_check_9_9_inputs[16].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[mul_res_limb_4_col141, mul_res_limb_5_col142].unpack());
            
(*lookup_data.rangecheck_9_9[16].get())[row_index] = [mul_res_limb_4_col141, mul_res_limb_5_col142];

(*sub_components_inputs
                .range_check_9_9_inputs[17].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[mul_res_limb_6_col143, mul_res_limb_7_col144].unpack());
            
(*lookup_data.rangecheck_9_9[17].get())[row_index] = [mul_res_limb_6_col143, mul_res_limb_7_col144];

(*sub_components_inputs
                .range_check_9_9_inputs[18].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[mul_res_limb_8_col145, mul_res_limb_9_col146].unpack());
            
(*lookup_data.rangecheck_9_9[18].get())[row_index] = [mul_res_limb_8_col145, mul_res_limb_9_col146];

(*sub_components_inputs
                .range_check_9_9_inputs[19].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[mul_res_limb_10_col147, mul_res_limb_11_col148].unpack());
            
(*lookup_data.rangecheck_9_9[19].get())[row_index] = [mul_res_limb_10_col147, mul_res_limb_11_col148];

(*sub_components_inputs
                .range_check_9_9_inputs[20].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[mul_res_limb_12_col149, mul_res_limb_13_col150].unpack());
            
(*lookup_data.rangecheck_9_9[20].get())[row_index] = [mul_res_limb_12_col149, mul_res_limb_13_col150];

(*sub_components_inputs
                .range_check_9_9_inputs[21].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[mul_res_limb_14_col151, mul_res_limb_15_col152].unpack());
            
(*lookup_data.rangecheck_9_9[21].get())[row_index] = [mul_res_limb_14_col151, mul_res_limb_15_col152];

(*sub_components_inputs
                .range_check_9_9_inputs[22].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[mul_res_limb_16_col153, mul_res_limb_17_col154].unpack());
            
(*lookup_data.rangecheck_9_9[22].get())[row_index] = [mul_res_limb_16_col153, mul_res_limb_17_col154];

(*sub_components_inputs
                .range_check_9_9_inputs[23].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[mul_res_limb_18_col155, mul_res_limb_19_col156].unpack());
            
(*lookup_data.rangecheck_9_9[23].get())[row_index] = [mul_res_limb_18_col155, mul_res_limb_19_col156];

(*sub_components_inputs
                .range_check_9_9_inputs[24].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[mul_res_limb_20_col157, mul_res_limb_21_col158].unpack());
            
(*lookup_data.rangecheck_9_9[24].get())[row_index] = [mul_res_limb_20_col157, mul_res_limb_21_col158];

(*sub_components_inputs
                .range_check_9_9_inputs[25].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[mul_res_limb_22_col159, mul_res_limb_23_col160].unpack());
            
(*lookup_data.rangecheck_9_9[25].get())[row_index] = [mul_res_limb_22_col159, mul_res_limb_23_col160];

(*sub_components_inputs
                .range_check_9_9_inputs[26].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[mul_res_limb_24_col161, mul_res_limb_25_col162].unpack());
            
(*lookup_data.rangecheck_9_9[26].get())[row_index] = [mul_res_limb_24_col161, mul_res_limb_25_col162];

(*sub_components_inputs
                .range_check_9_9_inputs[27].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[mul_res_limb_26_col163, mul_res_limb_27_col164].unpack());
            
(*lookup_data.rangecheck_9_9[27].get())[row_index] = [mul_res_limb_26_col163, mul_res_limb_27_col164];


            


            //VerifyMul252.

            
let conv_tmp_5745_62 = ((((M31_0) - (mul_res_limb_0_col137))) + (((op0_limb_0_col51) * (op1_limb_0_col80))));
let conv_tmp_5745_63 = ((((((M31_0) - (mul_res_limb_1_col138))) + (((op0_limb_0_col51) * (op1_limb_1_col81))))) + (((op0_limb_1_col52) * (op1_limb_0_col80))));
let conv_tmp_5745_64 = ((((((((M31_0) - (mul_res_limb_2_col139))) + (((op0_limb_0_col51) * (op1_limb_2_col82))))) + (((op0_limb_1_col52) * (op1_limb_1_col81))))) + (((op0_limb_2_col53) * (op1_limb_0_col80))));
let conv_tmp_5745_65 = ((((((((((M31_0) - (mul_res_limb_3_col140))) + (((op0_limb_0_col51) * (op1_limb_3_col83))))) + (((op0_limb_1_col52) * (op1_limb_2_col82))))) + (((op0_limb_2_col53) * (op1_limb_1_col81))))) + (((op0_limb_3_col54) * (op1_limb_0_col80))));
let conv_tmp_5745_66 = ((((((((((((M31_0) - (mul_res_limb_4_col141))) + (((op0_limb_0_col51) * (op1_limb_4_col84))))) + (((op0_limb_1_col52) * (op1_limb_3_col83))))) + (((op0_limb_2_col53) * (op1_limb_2_col82))))) + (((op0_limb_3_col54) * (op1_limb_1_col81))))) + (((op0_limb_4_col55) * (op1_limb_0_col80))));
let conv_tmp_5745_67 = ((((((((((((((M31_0) - (mul_res_limb_5_col142))) + (((op0_limb_0_col51) * (op1_limb_5_col85))))) + (((op0_limb_1_col52) * (op1_limb_4_col84))))) + (((op0_limb_2_col53) * (op1_limb_3_col83))))) + (((op0_limb_3_col54) * (op1_limb_2_col82))))) + (((op0_limb_4_col55) * (op1_limb_1_col81))))) + (((op0_limb_5_col56) * (op1_limb_0_col80))));
let conv_tmp_5745_68 = ((((((((((((((((M31_0) - (mul_res_limb_6_col143))) + (((op0_limb_0_col51) * (op1_limb_6_col86))))) + (((op0_limb_1_col52) * (op1_limb_5_col85))))) + (((op0_limb_2_col53) * (op1_limb_4_col84))))) + (((op0_limb_3_col54) * (op1_limb_3_col83))))) + (((op0_limb_4_col55) * (op1_limb_2_col82))))) + (((op0_limb_5_col56) * (op1_limb_1_col81))))) + (((op0_limb_6_col57) * (op1_limb_0_col80))));
let conv_tmp_5745_69 = ((((((((((((((((((M31_0) - (mul_res_limb_7_col144))) + (((op0_limb_0_col51) * (op1_limb_7_col87))))) + (((op0_limb_1_col52) * (op1_limb_6_col86))))) + (((op0_limb_2_col53) * (op1_limb_5_col85))))) + (((op0_limb_3_col54) * (op1_limb_4_col84))))) + (((op0_limb_4_col55) * (op1_limb_3_col83))))) + (((op0_limb_5_col56) * (op1_limb_2_col82))))) + (((op0_limb_6_col57) * (op1_limb_1_col81))))) + (((op0_limb_7_col58) * (op1_limb_0_col80))));
let conv_tmp_5745_70 = ((((((((((((((((((((M31_0) - (mul_res_limb_8_col145))) + (((op0_limb_0_col51) * (op1_limb_8_col88))))) + (((op0_limb_1_col52) * (op1_limb_7_col87))))) + (((op0_limb_2_col53) * (op1_limb_6_col86))))) + (((op0_limb_3_col54) * (op1_limb_5_col85))))) + (((op0_limb_4_col55) * (op1_limb_4_col84))))) + (((op0_limb_5_col56) * (op1_limb_3_col83))))) + (((op0_limb_6_col57) * (op1_limb_2_col82))))) + (((op0_limb_7_col58) * (op1_limb_1_col81))))) + (((op0_limb_8_col59) * (op1_limb_0_col80))));
let conv_tmp_5745_71 = ((((((((((((((((((((((M31_0) - (mul_res_limb_9_col146))) + (((op0_limb_0_col51) * (op1_limb_9_col89))))) + (((op0_limb_1_col52) * (op1_limb_8_col88))))) + (((op0_limb_2_col53) * (op1_limb_7_col87))))) + (((op0_limb_3_col54) * (op1_limb_6_col86))))) + (((op0_limb_4_col55) * (op1_limb_5_col85))))) + (((op0_limb_5_col56) * (op1_limb_4_col84))))) + (((op0_limb_6_col57) * (op1_limb_3_col83))))) + (((op0_limb_7_col58) * (op1_limb_2_col82))))) + (((op0_limb_8_col59) * (op1_limb_1_col81))))) + (((op0_limb_9_col60) * (op1_limb_0_col80))));
let conv_tmp_5745_72 = ((((((((((((((((((((((((M31_0) - (mul_res_limb_10_col147))) + (((op0_limb_0_col51) * (op1_limb_10_col90))))) + (((op0_limb_1_col52) * (op1_limb_9_col89))))) + (((op0_limb_2_col53) * (op1_limb_8_col88))))) + (((op0_limb_3_col54) * (op1_limb_7_col87))))) + (((op0_limb_4_col55) * (op1_limb_6_col86))))) + (((op0_limb_5_col56) * (op1_limb_5_col85))))) + (((op0_limb_6_col57) * (op1_limb_4_col84))))) + (((op0_limb_7_col58) * (op1_limb_3_col83))))) + (((op0_limb_8_col59) * (op1_limb_2_col82))))) + (((op0_limb_9_col60) * (op1_limb_1_col81))))) + (((op0_limb_10_col61) * (op1_limb_0_col80))));
let conv_tmp_5745_73 = ((((((((((((((((((((((((((M31_0) - (mul_res_limb_11_col148))) + (((op0_limb_0_col51) * (op1_limb_11_col91))))) + (((op0_limb_1_col52) * (op1_limb_10_col90))))) + (((op0_limb_2_col53) * (op1_limb_9_col89))))) + (((op0_limb_3_col54) * (op1_limb_8_col88))))) + (((op0_limb_4_col55) * (op1_limb_7_col87))))) + (((op0_limb_5_col56) * (op1_limb_6_col86))))) + (((op0_limb_6_col57) * (op1_limb_5_col85))))) + (((op0_limb_7_col58) * (op1_limb_4_col84))))) + (((op0_limb_8_col59) * (op1_limb_3_col83))))) + (((op0_limb_9_col60) * (op1_limb_2_col82))))) + (((op0_limb_10_col61) * (op1_limb_1_col81))))) + (((op0_limb_11_col62) * (op1_limb_0_col80))));
let conv_tmp_5745_74 = ((((((((((((((((((((((((((((M31_0) - (mul_res_limb_12_col149))) + (((op0_limb_0_col51) * (op1_limb_12_col92))))) + (((op0_limb_1_col52) * (op1_limb_11_col91))))) + (((op0_limb_2_col53) * (op1_limb_10_col90))))) + (((op0_limb_3_col54) * (op1_limb_9_col89))))) + (((op0_limb_4_col55) * (op1_limb_8_col88))))) + (((op0_limb_5_col56) * (op1_limb_7_col87))))) + (((op0_limb_6_col57) * (op1_limb_6_col86))))) + (((op0_limb_7_col58) * (op1_limb_5_col85))))) + (((op0_limb_8_col59) * (op1_limb_4_col84))))) + (((op0_limb_9_col60) * (op1_limb_3_col83))))) + (((op0_limb_10_col61) * (op1_limb_2_col82))))) + (((op0_limb_11_col62) * (op1_limb_1_col81))))) + (((op0_limb_12_col63) * (op1_limb_0_col80))));
let conv_tmp_5745_75 = ((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_13_col150))) + (((op0_limb_0_col51) * (op1_limb_13_col93))))) + (((op0_limb_1_col52) * (op1_limb_12_col92))))) + (((op0_limb_2_col53) * (op1_limb_11_col91))))) + (((op0_limb_3_col54) * (op1_limb_10_col90))))) + (((op0_limb_4_col55) * (op1_limb_9_col89))))) + (((op0_limb_5_col56) * (op1_limb_8_col88))))) + (((op0_limb_6_col57) * (op1_limb_7_col87))))) + (((op0_limb_7_col58) * (op1_limb_6_col86))))) + (((op0_limb_8_col59) * (op1_limb_5_col85))))) + (((op0_limb_9_col60) * (op1_limb_4_col84))))) + (((op0_limb_10_col61) * (op1_limb_3_col83))))) + (((op0_limb_11_col62) * (op1_limb_2_col82))))) + (((op0_limb_12_col63) * (op1_limb_1_col81))))) + (((op0_limb_13_col64) * (op1_limb_0_col80))));
let conv_tmp_5745_76 = ((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_14_col151))) + (((op0_limb_0_col51) * (op1_limb_14_col94))))) + (((op0_limb_1_col52) * (op1_limb_13_col93))))) + (((op0_limb_2_col53) * (op1_limb_12_col92))))) + (((op0_limb_3_col54) * (op1_limb_11_col91))))) + (((op0_limb_4_col55) * (op1_limb_10_col90))))) + (((op0_limb_5_col56) * (op1_limb_9_col89))))) + (((op0_limb_6_col57) * (op1_limb_8_col88))))) + (((op0_limb_7_col58) * (op1_limb_7_col87))))) + (((op0_limb_8_col59) * (op1_limb_6_col86))))) + (((op0_limb_9_col60) * (op1_limb_5_col85))))) + (((op0_limb_10_col61) * (op1_limb_4_col84))))) + (((op0_limb_11_col62) * (op1_limb_3_col83))))) + (((op0_limb_12_col63) * (op1_limb_2_col82))))) + (((op0_limb_13_col64) * (op1_limb_1_col81))))) + (((op0_limb_14_col65) * (op1_limb_0_col80))));
let conv_tmp_5745_77 = ((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_15_col152))) + (((op0_limb_0_col51) * (op1_limb_15_col95))))) + (((op0_limb_1_col52) * (op1_limb_14_col94))))) + (((op0_limb_2_col53) * (op1_limb_13_col93))))) + (((op0_limb_3_col54) * (op1_limb_12_col92))))) + (((op0_limb_4_col55) * (op1_limb_11_col91))))) + (((op0_limb_5_col56) * (op1_limb_10_col90))))) + (((op0_limb_6_col57) * (op1_limb_9_col89))))) + (((op0_limb_7_col58) * (op1_limb_8_col88))))) + (((op0_limb_8_col59) * (op1_limb_7_col87))))) + (((op0_limb_9_col60) * (op1_limb_6_col86))))) + (((op0_limb_10_col61) * (op1_limb_5_col85))))) + (((op0_limb_11_col62) * (op1_limb_4_col84))))) + (((op0_limb_12_col63) * (op1_limb_3_col83))))) + (((op0_limb_13_col64) * (op1_limb_2_col82))))) + (((op0_limb_14_col65) * (op1_limb_1_col81))))) + (((op0_limb_15_col66) * (op1_limb_0_col80))));
let conv_tmp_5745_78 = ((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_16_col153))) + (((op0_limb_0_col51) * (op1_limb_16_col96))))) + (((op0_limb_1_col52) * (op1_limb_15_col95))))) + (((op0_limb_2_col53) * (op1_limb_14_col94))))) + (((op0_limb_3_col54) * (op1_limb_13_col93))))) + (((op0_limb_4_col55) * (op1_limb_12_col92))))) + (((op0_limb_5_col56) * (op1_limb_11_col91))))) + (((op0_limb_6_col57) * (op1_limb_10_col90))))) + (((op0_limb_7_col58) * (op1_limb_9_col89))))) + (((op0_limb_8_col59) * (op1_limb_8_col88))))) + (((op0_limb_9_col60) * (op1_limb_7_col87))))) + (((op0_limb_10_col61) * (op1_limb_6_col86))))) + (((op0_limb_11_col62) * (op1_limb_5_col85))))) + (((op0_limb_12_col63) * (op1_limb_4_col84))))) + (((op0_limb_13_col64) * (op1_limb_3_col83))))) + (((op0_limb_14_col65) * (op1_limb_2_col82))))) + (((op0_limb_15_col66) * (op1_limb_1_col81))))) + (((op0_limb_16_col67) * (op1_limb_0_col80))));
let conv_tmp_5745_79 = ((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_17_col154))) + (((op0_limb_0_col51) * (op1_limb_17_col97))))) + (((op0_limb_1_col52) * (op1_limb_16_col96))))) + (((op0_limb_2_col53) * (op1_limb_15_col95))))) + (((op0_limb_3_col54) * (op1_limb_14_col94))))) + (((op0_limb_4_col55) * (op1_limb_13_col93))))) + (((op0_limb_5_col56) * (op1_limb_12_col92))))) + (((op0_limb_6_col57) * (op1_limb_11_col91))))) + (((op0_limb_7_col58) * (op1_limb_10_col90))))) + (((op0_limb_8_col59) * (op1_limb_9_col89))))) + (((op0_limb_9_col60) * (op1_limb_8_col88))))) + (((op0_limb_10_col61) * (op1_limb_7_col87))))) + (((op0_limb_11_col62) * (op1_limb_6_col86))))) + (((op0_limb_12_col63) * (op1_limb_5_col85))))) + (((op0_limb_13_col64) * (op1_limb_4_col84))))) + (((op0_limb_14_col65) * (op1_limb_3_col83))))) + (((op0_limb_15_col66) * (op1_limb_2_col82))))) + (((op0_limb_16_col67) * (op1_limb_1_col81))))) + (((op0_limb_17_col68) * (op1_limb_0_col80))));
let conv_tmp_5745_80 = ((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_18_col155))) + (((op0_limb_0_col51) * (op1_limb_18_col98))))) + (((op0_limb_1_col52) * (op1_limb_17_col97))))) + (((op0_limb_2_col53) * (op1_limb_16_col96))))) + (((op0_limb_3_col54) * (op1_limb_15_col95))))) + (((op0_limb_4_col55) * (op1_limb_14_col94))))) + (((op0_limb_5_col56) * (op1_limb_13_col93))))) + (((op0_limb_6_col57) * (op1_limb_12_col92))))) + (((op0_limb_7_col58) * (op1_limb_11_col91))))) + (((op0_limb_8_col59) * (op1_limb_10_col90))))) + (((op0_limb_9_col60) * (op1_limb_9_col89))))) + (((op0_limb_10_col61) * (op1_limb_8_col88))))) + (((op0_limb_11_col62) * (op1_limb_7_col87))))) + (((op0_limb_12_col63) * (op1_limb_6_col86))))) + (((op0_limb_13_col64) * (op1_limb_5_col85))))) + (((op0_limb_14_col65) * (op1_limb_4_col84))))) + (((op0_limb_15_col66) * (op1_limb_3_col83))))) + (((op0_limb_16_col67) * (op1_limb_2_col82))))) + (((op0_limb_17_col68) * (op1_limb_1_col81))))) + (((op0_limb_18_col69) * (op1_limb_0_col80))));
let conv_tmp_5745_81 = ((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_19_col156))) + (((op0_limb_0_col51) * (op1_limb_19_col99))))) + (((op0_limb_1_col52) * (op1_limb_18_col98))))) + (((op0_limb_2_col53) * (op1_limb_17_col97))))) + (((op0_limb_3_col54) * (op1_limb_16_col96))))) + (((op0_limb_4_col55) * (op1_limb_15_col95))))) + (((op0_limb_5_col56) * (op1_limb_14_col94))))) + (((op0_limb_6_col57) * (op1_limb_13_col93))))) + (((op0_limb_7_col58) * (op1_limb_12_col92))))) + (((op0_limb_8_col59) * (op1_limb_11_col91))))) + (((op0_limb_9_col60) * (op1_limb_10_col90))))) + (((op0_limb_10_col61) * (op1_limb_9_col89))))) + (((op0_limb_11_col62) * (op1_limb_8_col88))))) + (((op0_limb_12_col63) * (op1_limb_7_col87))))) + (((op0_limb_13_col64) * (op1_limb_6_col86))))) + (((op0_limb_14_col65) * (op1_limb_5_col85))))) + (((op0_limb_15_col66) * (op1_limb_4_col84))))) + (((op0_limb_16_col67) * (op1_limb_3_col83))))) + (((op0_limb_17_col68) * (op1_limb_2_col82))))) + (((op0_limb_18_col69) * (op1_limb_1_col81))))) + (((op0_limb_19_col70) * (op1_limb_0_col80))));
let conv_tmp_5745_82 = ((((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_20_col157))) + (((op0_limb_0_col51) * (op1_limb_20_col100))))) + (((op0_limb_1_col52) * (op1_limb_19_col99))))) + (((op0_limb_2_col53) * (op1_limb_18_col98))))) + (((op0_limb_3_col54) * (op1_limb_17_col97))))) + (((op0_limb_4_col55) * (op1_limb_16_col96))))) + (((op0_limb_5_col56) * (op1_limb_15_col95))))) + (((op0_limb_6_col57) * (op1_limb_14_col94))))) + (((op0_limb_7_col58) * (op1_limb_13_col93))))) + (((op0_limb_8_col59) * (op1_limb_12_col92))))) + (((op0_limb_9_col60) * (op1_limb_11_col91))))) + (((op0_limb_10_col61) * (op1_limb_10_col90))))) + (((op0_limb_11_col62) * (op1_limb_9_col89))))) + (((op0_limb_12_col63) * (op1_limb_8_col88))))) + (((op0_limb_13_col64) * (op1_limb_7_col87))))) + (((op0_limb_14_col65) * (op1_limb_6_col86))))) + (((op0_limb_15_col66) * (op1_limb_5_col85))))) + (((op0_limb_16_col67) * (op1_limb_4_col84))))) + (((op0_limb_17_col68) * (op1_limb_3_col83))))) + (((op0_limb_18_col69) * (op1_limb_2_col82))))) + (((op0_limb_19_col70) * (op1_limb_1_col81))))) + (((op0_limb_20_col71) * (op1_limb_0_col80))));
let conv_tmp_5745_83 = ((((((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_21_col158))) + (((op0_limb_0_col51) * (op1_limb_21_col101))))) + (((op0_limb_1_col52) * (op1_limb_20_col100))))) + (((op0_limb_2_col53) * (op1_limb_19_col99))))) + (((op0_limb_3_col54) * (op1_limb_18_col98))))) + (((op0_limb_4_col55) * (op1_limb_17_col97))))) + (((op0_limb_5_col56) * (op1_limb_16_col96))))) + (((op0_limb_6_col57) * (op1_limb_15_col95))))) + (((op0_limb_7_col58) * (op1_limb_14_col94))))) + (((op0_limb_8_col59) * (op1_limb_13_col93))))) + (((op0_limb_9_col60) * (op1_limb_12_col92))))) + (((op0_limb_10_col61) * (op1_limb_11_col91))))) + (((op0_limb_11_col62) * (op1_limb_10_col90))))) + (((op0_limb_12_col63) * (op1_limb_9_col89))))) + (((op0_limb_13_col64) * (op1_limb_8_col88))))) + (((op0_limb_14_col65) * (op1_limb_7_col87))))) + (((op0_limb_15_col66) * (op1_limb_6_col86))))) + (((op0_limb_16_col67) * (op1_limb_5_col85))))) + (((op0_limb_17_col68) * (op1_limb_4_col84))))) + (((op0_limb_18_col69) * (op1_limb_3_col83))))) + (((op0_limb_19_col70) * (op1_limb_2_col82))))) + (((op0_limb_20_col71) * (op1_limb_1_col81))))) + (((op0_limb_21_col72) * (op1_limb_0_col80))));
let conv_tmp_5745_84 = ((((((((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_22_col159))) + (((op0_limb_0_col51) * (op1_limb_22_col102))))) + (((op0_limb_1_col52) * (op1_limb_21_col101))))) + (((op0_limb_2_col53) * (op1_limb_20_col100))))) + (((op0_limb_3_col54) * (op1_limb_19_col99))))) + (((op0_limb_4_col55) * (op1_limb_18_col98))))) + (((op0_limb_5_col56) * (op1_limb_17_col97))))) + (((op0_limb_6_col57) * (op1_limb_16_col96))))) + (((op0_limb_7_col58) * (op1_limb_15_col95))))) + (((op0_limb_8_col59) * (op1_limb_14_col94))))) + (((op0_limb_9_col60) * (op1_limb_13_col93))))) + (((op0_limb_10_col61) * (op1_limb_12_col92))))) + (((op0_limb_11_col62) * (op1_limb_11_col91))))) + (((op0_limb_12_col63) * (op1_limb_10_col90))))) + (((op0_limb_13_col64) * (op1_limb_9_col89))))) + (((op0_limb_14_col65) * (op1_limb_8_col88))))) + (((op0_limb_15_col66) * (op1_limb_7_col87))))) + (((op0_limb_16_col67) * (op1_limb_6_col86))))) + (((op0_limb_17_col68) * (op1_limb_5_col85))))) + (((op0_limb_18_col69) * (op1_limb_4_col84))))) + (((op0_limb_19_col70) * (op1_limb_3_col83))))) + (((op0_limb_20_col71) * (op1_limb_2_col82))))) + (((op0_limb_21_col72) * (op1_limb_1_col81))))) + (((op0_limb_22_col73) * (op1_limb_0_col80))));
let conv_tmp_5745_85 = ((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_23_col160))) + (((op0_limb_0_col51) * (op1_limb_23_col103))))) + (((op0_limb_1_col52) * (op1_limb_22_col102))))) + (((op0_limb_2_col53) * (op1_limb_21_col101))))) + (((op0_limb_3_col54) * (op1_limb_20_col100))))) + (((op0_limb_4_col55) * (op1_limb_19_col99))))) + (((op0_limb_5_col56) * (op1_limb_18_col98))))) + (((op0_limb_6_col57) * (op1_limb_17_col97))))) + (((op0_limb_7_col58) * (op1_limb_16_col96))))) + (((op0_limb_8_col59) * (op1_limb_15_col95))))) + (((op0_limb_9_col60) * (op1_limb_14_col94))))) + (((op0_limb_10_col61) * (op1_limb_13_col93))))) + (((op0_limb_11_col62) * (op1_limb_12_col92))))) + (((op0_limb_12_col63) * (op1_limb_11_col91))))) + (((op0_limb_13_col64) * (op1_limb_10_col90))))) + (((op0_limb_14_col65) * (op1_limb_9_col89))))) + (((op0_limb_15_col66) * (op1_limb_8_col88))))) + (((op0_limb_16_col67) * (op1_limb_7_col87))))) + (((op0_limb_17_col68) * (op1_limb_6_col86))))) + (((op0_limb_18_col69) * (op1_limb_5_col85))))) + (((op0_limb_19_col70) * (op1_limb_4_col84))))) + (((op0_limb_20_col71) * (op1_limb_3_col83))))) + (((op0_limb_21_col72) * (op1_limb_2_col82))))) + (((op0_limb_22_col73) * (op1_limb_1_col81))))) + (((op0_limb_23_col74) * (op1_limb_0_col80))));
let conv_tmp_5745_86 = ((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_24_col161))) + (((op0_limb_0_col51) * (op1_limb_24_col104))))) + (((op0_limb_1_col52) * (op1_limb_23_col103))))) + (((op0_limb_2_col53) * (op1_limb_22_col102))))) + (((op0_limb_3_col54) * (op1_limb_21_col101))))) + (((op0_limb_4_col55) * (op1_limb_20_col100))))) + (((op0_limb_5_col56) * (op1_limb_19_col99))))) + (((op0_limb_6_col57) * (op1_limb_18_col98))))) + (((op0_limb_7_col58) * (op1_limb_17_col97))))) + (((op0_limb_8_col59) * (op1_limb_16_col96))))) + (((op0_limb_9_col60) * (op1_limb_15_col95))))) + (((op0_limb_10_col61) * (op1_limb_14_col94))))) + (((op0_limb_11_col62) * (op1_limb_13_col93))))) + (((op0_limb_12_col63) * (op1_limb_12_col92))))) + (((op0_limb_13_col64) * (op1_limb_11_col91))))) + (((op0_limb_14_col65) * (op1_limb_10_col90))))) + (((op0_limb_15_col66) * (op1_limb_9_col89))))) + (((op0_limb_16_col67) * (op1_limb_8_col88))))) + (((op0_limb_17_col68) * (op1_limb_7_col87))))) + (((op0_limb_18_col69) * (op1_limb_6_col86))))) + (((op0_limb_19_col70) * (op1_limb_5_col85))))) + (((op0_limb_20_col71) * (op1_limb_4_col84))))) + (((op0_limb_21_col72) * (op1_limb_3_col83))))) + (((op0_limb_22_col73) * (op1_limb_2_col82))))) + (((op0_limb_23_col74) * (op1_limb_1_col81))))) + (((op0_limb_24_col75) * (op1_limb_0_col80))));
let conv_tmp_5745_87 = ((((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_25_col162))) + (((op0_limb_0_col51) * (op1_limb_25_col105))))) + (((op0_limb_1_col52) * (op1_limb_24_col104))))) + (((op0_limb_2_col53) * (op1_limb_23_col103))))) + (((op0_limb_3_col54) * (op1_limb_22_col102))))) + (((op0_limb_4_col55) * (op1_limb_21_col101))))) + (((op0_limb_5_col56) * (op1_limb_20_col100))))) + (((op0_limb_6_col57) * (op1_limb_19_col99))))) + (((op0_limb_7_col58) * (op1_limb_18_col98))))) + (((op0_limb_8_col59) * (op1_limb_17_col97))))) + (((op0_limb_9_col60) * (op1_limb_16_col96))))) + (((op0_limb_10_col61) * (op1_limb_15_col95))))) + (((op0_limb_11_col62) * (op1_limb_14_col94))))) + (((op0_limb_12_col63) * (op1_limb_13_col93))))) + (((op0_limb_13_col64) * (op1_limb_12_col92))))) + (((op0_limb_14_col65) * (op1_limb_11_col91))))) + (((op0_limb_15_col66) * (op1_limb_10_col90))))) + (((op0_limb_16_col67) * (op1_limb_9_col89))))) + (((op0_limb_17_col68) * (op1_limb_8_col88))))) + (((op0_limb_18_col69) * (op1_limb_7_col87))))) + (((op0_limb_19_col70) * (op1_limb_6_col86))))) + (((op0_limb_20_col71) * (op1_limb_5_col85))))) + (((op0_limb_21_col72) * (op1_limb_4_col84))))) + (((op0_limb_22_col73) * (op1_limb_3_col83))))) + (((op0_limb_23_col74) * (op1_limb_2_col82))))) + (((op0_limb_24_col75) * (op1_limb_1_col81))))) + (((op0_limb_25_col76) * (op1_limb_0_col80))));
let conv_tmp_5745_88 = ((((((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_26_col163))) + (((op0_limb_0_col51) * (op1_limb_26_col106))))) + (((op0_limb_1_col52) * (op1_limb_25_col105))))) + (((op0_limb_2_col53) * (op1_limb_24_col104))))) + (((op0_limb_3_col54) * (op1_limb_23_col103))))) + (((op0_limb_4_col55) * (op1_limb_22_col102))))) + (((op0_limb_5_col56) * (op1_limb_21_col101))))) + (((op0_limb_6_col57) * (op1_limb_20_col100))))) + (((op0_limb_7_col58) * (op1_limb_19_col99))))) + (((op0_limb_8_col59) * (op1_limb_18_col98))))) + (((op0_limb_9_col60) * (op1_limb_17_col97))))) + (((op0_limb_10_col61) * (op1_limb_16_col96))))) + (((op0_limb_11_col62) * (op1_limb_15_col95))))) + (((op0_limb_12_col63) * (op1_limb_14_col94))))) + (((op0_limb_13_col64) * (op1_limb_13_col93))))) + (((op0_limb_14_col65) * (op1_limb_12_col92))))) + (((op0_limb_15_col66) * (op1_limb_11_col91))))) + (((op0_limb_16_col67) * (op1_limb_10_col90))))) + (((op0_limb_17_col68) * (op1_limb_9_col89))))) + (((op0_limb_18_col69) * (op1_limb_8_col88))))) + (((op0_limb_19_col70) * (op1_limb_7_col87))))) + (((op0_limb_20_col71) * (op1_limb_6_col86))))) + (((op0_limb_21_col72) * (op1_limb_5_col85))))) + (((op0_limb_22_col73) * (op1_limb_4_col84))))) + (((op0_limb_23_col74) * (op1_limb_3_col83))))) + (((op0_limb_24_col75) * (op1_limb_2_col82))))) + (((op0_limb_25_col76) * (op1_limb_1_col81))))) + (((op0_limb_26_col77) * (op1_limb_0_col80))));
let conv_tmp_5745_89 = ((((((((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_27_col164))) + (((op0_limb_0_col51) * (op1_limb_27_col107))))) + (((op0_limb_1_col52) * (op1_limb_26_col106))))) + (((op0_limb_2_col53) * (op1_limb_25_col105))))) + (((op0_limb_3_col54) * (op1_limb_24_col104))))) + (((op0_limb_4_col55) * (op1_limb_23_col103))))) + (((op0_limb_5_col56) * (op1_limb_22_col102))))) + (((op0_limb_6_col57) * (op1_limb_21_col101))))) + (((op0_limb_7_col58) * (op1_limb_20_col100))))) + (((op0_limb_8_col59) * (op1_limb_19_col99))))) + (((op0_limb_9_col60) * (op1_limb_18_col98))))) + (((op0_limb_10_col61) * (op1_limb_17_col97))))) + (((op0_limb_11_col62) * (op1_limb_16_col96))))) + (((op0_limb_12_col63) * (op1_limb_15_col95))))) + (((op0_limb_13_col64) * (op1_limb_14_col94))))) + (((op0_limb_14_col65) * (op1_limb_13_col93))))) + (((op0_limb_15_col66) * (op1_limb_12_col92))))) + (((op0_limb_16_col67) * (op1_limb_11_col91))))) + (((op0_limb_17_col68) * (op1_limb_10_col90))))) + (((op0_limb_18_col69) * (op1_limb_9_col89))))) + (((op0_limb_19_col70) * (op1_limb_8_col88))))) + (((op0_limb_20_col71) * (op1_limb_7_col87))))) + (((op0_limb_21_col72) * (op1_limb_6_col86))))) + (((op0_limb_22_col73) * (op1_limb_5_col85))))) + (((op0_limb_23_col74) * (op1_limb_4_col84))))) + (((op0_limb_24_col75) * (op1_limb_3_col83))))) + (((op0_limb_25_col76) * (op1_limb_2_col82))))) + (((op0_limb_26_col77) * (op1_limb_1_col81))))) + (((op0_limb_27_col78) * (op1_limb_0_col80))));
let conv_tmp_5745_90 = ((((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_1_col52) * (op1_limb_27_col107))))) + (((op0_limb_2_col53) * (op1_limb_26_col106))))) + (((op0_limb_3_col54) * (op1_limb_25_col105))))) + (((op0_limb_4_col55) * (op1_limb_24_col104))))) + (((op0_limb_5_col56) * (op1_limb_23_col103))))) + (((op0_limb_6_col57) * (op1_limb_22_col102))))) + (((op0_limb_7_col58) * (op1_limb_21_col101))))) + (((op0_limb_8_col59) * (op1_limb_20_col100))))) + (((op0_limb_9_col60) * (op1_limb_19_col99))))) + (((op0_limb_10_col61) * (op1_limb_18_col98))))) + (((op0_limb_11_col62) * (op1_limb_17_col97))))) + (((op0_limb_12_col63) * (op1_limb_16_col96))))) + (((op0_limb_13_col64) * (op1_limb_15_col95))))) + (((op0_limb_14_col65) * (op1_limb_14_col94))))) + (((op0_limb_15_col66) * (op1_limb_13_col93))))) + (((op0_limb_16_col67) * (op1_limb_12_col92))))) + (((op0_limb_17_col68) * (op1_limb_11_col91))))) + (((op0_limb_18_col69) * (op1_limb_10_col90))))) + (((op0_limb_19_col70) * (op1_limb_9_col89))))) + (((op0_limb_20_col71) * (op1_limb_8_col88))))) + (((op0_limb_21_col72) * (op1_limb_7_col87))))) + (((op0_limb_22_col73) * (op1_limb_6_col86))))) + (((op0_limb_23_col74) * (op1_limb_5_col85))))) + (((op0_limb_24_col75) * (op1_limb_4_col84))))) + (((op0_limb_25_col76) * (op1_limb_3_col83))))) + (((op0_limb_26_col77) * (op1_limb_2_col82))))) + (((op0_limb_27_col78) * (op1_limb_1_col81))));
let conv_tmp_5745_91 = ((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_2_col53) * (op1_limb_27_col107))))) + (((op0_limb_3_col54) * (op1_limb_26_col106))))) + (((op0_limb_4_col55) * (op1_limb_25_col105))))) + (((op0_limb_5_col56) * (op1_limb_24_col104))))) + (((op0_limb_6_col57) * (op1_limb_23_col103))))) + (((op0_limb_7_col58) * (op1_limb_22_col102))))) + (((op0_limb_8_col59) * (op1_limb_21_col101))))) + (((op0_limb_9_col60) * (op1_limb_20_col100))))) + (((op0_limb_10_col61) * (op1_limb_19_col99))))) + (((op0_limb_11_col62) * (op1_limb_18_col98))))) + (((op0_limb_12_col63) * (op1_limb_17_col97))))) + (((op0_limb_13_col64) * (op1_limb_16_col96))))) + (((op0_limb_14_col65) * (op1_limb_15_col95))))) + (((op0_limb_15_col66) * (op1_limb_14_col94))))) + (((op0_limb_16_col67) * (op1_limb_13_col93))))) + (((op0_limb_17_col68) * (op1_limb_12_col92))))) + (((op0_limb_18_col69) * (op1_limb_11_col91))))) + (((op0_limb_19_col70) * (op1_limb_10_col90))))) + (((op0_limb_20_col71) * (op1_limb_9_col89))))) + (((op0_limb_21_col72) * (op1_limb_8_col88))))) + (((op0_limb_22_col73) * (op1_limb_7_col87))))) + (((op0_limb_23_col74) * (op1_limb_6_col86))))) + (((op0_limb_24_col75) * (op1_limb_5_col85))))) + (((op0_limb_25_col76) * (op1_limb_4_col84))))) + (((op0_limb_26_col77) * (op1_limb_3_col83))))) + (((op0_limb_27_col78) * (op1_limb_2_col82))));
let conv_tmp_5745_92 = ((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_3_col54) * (op1_limb_27_col107))))) + (((op0_limb_4_col55) * (op1_limb_26_col106))))) + (((op0_limb_5_col56) * (op1_limb_25_col105))))) + (((op0_limb_6_col57) * (op1_limb_24_col104))))) + (((op0_limb_7_col58) * (op1_limb_23_col103))))) + (((op0_limb_8_col59) * (op1_limb_22_col102))))) + (((op0_limb_9_col60) * (op1_limb_21_col101))))) + (((op0_limb_10_col61) * (op1_limb_20_col100))))) + (((op0_limb_11_col62) * (op1_limb_19_col99))))) + (((op0_limb_12_col63) * (op1_limb_18_col98))))) + (((op0_limb_13_col64) * (op1_limb_17_col97))))) + (((op0_limb_14_col65) * (op1_limb_16_col96))))) + (((op0_limb_15_col66) * (op1_limb_15_col95))))) + (((op0_limb_16_col67) * (op1_limb_14_col94))))) + (((op0_limb_17_col68) * (op1_limb_13_col93))))) + (((op0_limb_18_col69) * (op1_limb_12_col92))))) + (((op0_limb_19_col70) * (op1_limb_11_col91))))) + (((op0_limb_20_col71) * (op1_limb_10_col90))))) + (((op0_limb_21_col72) * (op1_limb_9_col89))))) + (((op0_limb_22_col73) * (op1_limb_8_col88))))) + (((op0_limb_23_col74) * (op1_limb_7_col87))))) + (((op0_limb_24_col75) * (op1_limb_6_col86))))) + (((op0_limb_25_col76) * (op1_limb_5_col85))))) + (((op0_limb_26_col77) * (op1_limb_4_col84))))) + (((op0_limb_27_col78) * (op1_limb_3_col83))));
let conv_tmp_5745_93 = ((((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_4_col55) * (op1_limb_27_col107))))) + (((op0_limb_5_col56) * (op1_limb_26_col106))))) + (((op0_limb_6_col57) * (op1_limb_25_col105))))) + (((op0_limb_7_col58) * (op1_limb_24_col104))))) + (((op0_limb_8_col59) * (op1_limb_23_col103))))) + (((op0_limb_9_col60) * (op1_limb_22_col102))))) + (((op0_limb_10_col61) * (op1_limb_21_col101))))) + (((op0_limb_11_col62) * (op1_limb_20_col100))))) + (((op0_limb_12_col63) * (op1_limb_19_col99))))) + (((op0_limb_13_col64) * (op1_limb_18_col98))))) + (((op0_limb_14_col65) * (op1_limb_17_col97))))) + (((op0_limb_15_col66) * (op1_limb_16_col96))))) + (((op0_limb_16_col67) * (op1_limb_15_col95))))) + (((op0_limb_17_col68) * (op1_limb_14_col94))))) + (((op0_limb_18_col69) * (op1_limb_13_col93))))) + (((op0_limb_19_col70) * (op1_limb_12_col92))))) + (((op0_limb_20_col71) * (op1_limb_11_col91))))) + (((op0_limb_21_col72) * (op1_limb_10_col90))))) + (((op0_limb_22_col73) * (op1_limb_9_col89))))) + (((op0_limb_23_col74) * (op1_limb_8_col88))))) + (((op0_limb_24_col75) * (op1_limb_7_col87))))) + (((op0_limb_25_col76) * (op1_limb_6_col86))))) + (((op0_limb_26_col77) * (op1_limb_5_col85))))) + (((op0_limb_27_col78) * (op1_limb_4_col84))));
let conv_tmp_5745_94 = ((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_5_col56) * (op1_limb_27_col107))))) + (((op0_limb_6_col57) * (op1_limb_26_col106))))) + (((op0_limb_7_col58) * (op1_limb_25_col105))))) + (((op0_limb_8_col59) * (op1_limb_24_col104))))) + (((op0_limb_9_col60) * (op1_limb_23_col103))))) + (((op0_limb_10_col61) * (op1_limb_22_col102))))) + (((op0_limb_11_col62) * (op1_limb_21_col101))))) + (((op0_limb_12_col63) * (op1_limb_20_col100))))) + (((op0_limb_13_col64) * (op1_limb_19_col99))))) + (((op0_limb_14_col65) * (op1_limb_18_col98))))) + (((op0_limb_15_col66) * (op1_limb_17_col97))))) + (((op0_limb_16_col67) * (op1_limb_16_col96))))) + (((op0_limb_17_col68) * (op1_limb_15_col95))))) + (((op0_limb_18_col69) * (op1_limb_14_col94))))) + (((op0_limb_19_col70) * (op1_limb_13_col93))))) + (((op0_limb_20_col71) * (op1_limb_12_col92))))) + (((op0_limb_21_col72) * (op1_limb_11_col91))))) + (((op0_limb_22_col73) * (op1_limb_10_col90))))) + (((op0_limb_23_col74) * (op1_limb_9_col89))))) + (((op0_limb_24_col75) * (op1_limb_8_col88))))) + (((op0_limb_25_col76) * (op1_limb_7_col87))))) + (((op0_limb_26_col77) * (op1_limb_6_col86))))) + (((op0_limb_27_col78) * (op1_limb_5_col85))));
let conv_tmp_5745_95 = ((((((((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_6_col57) * (op1_limb_27_col107))))) + (((op0_limb_7_col58) * (op1_limb_26_col106))))) + (((op0_limb_8_col59) * (op1_limb_25_col105))))) + (((op0_limb_9_col60) * (op1_limb_24_col104))))) + (((op0_limb_10_col61) * (op1_limb_23_col103))))) + (((op0_limb_11_col62) * (op1_limb_22_col102))))) + (((op0_limb_12_col63) * (op1_limb_21_col101))))) + (((op0_limb_13_col64) * (op1_limb_20_col100))))) + (((op0_limb_14_col65) * (op1_limb_19_col99))))) + (((op0_limb_15_col66) * (op1_limb_18_col98))))) + (((op0_limb_16_col67) * (op1_limb_17_col97))))) + (((op0_limb_17_col68) * (op1_limb_16_col96))))) + (((op0_limb_18_col69) * (op1_limb_15_col95))))) + (((op0_limb_19_col70) * (op1_limb_14_col94))))) + (((op0_limb_20_col71) * (op1_limb_13_col93))))) + (((op0_limb_21_col72) * (op1_limb_12_col92))))) + (((op0_limb_22_col73) * (op1_limb_11_col91))))) + (((op0_limb_23_col74) * (op1_limb_10_col90))))) + (((op0_limb_24_col75) * (op1_limb_9_col89))))) + (((op0_limb_25_col76) * (op1_limb_8_col88))))) + (((op0_limb_26_col77) * (op1_limb_7_col87))))) + (((op0_limb_27_col78) * (op1_limb_6_col86))));
let conv_tmp_5745_96 = ((((((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_7_col58) * (op1_limb_27_col107))))) + (((op0_limb_8_col59) * (op1_limb_26_col106))))) + (((op0_limb_9_col60) * (op1_limb_25_col105))))) + (((op0_limb_10_col61) * (op1_limb_24_col104))))) + (((op0_limb_11_col62) * (op1_limb_23_col103))))) + (((op0_limb_12_col63) * (op1_limb_22_col102))))) + (((op0_limb_13_col64) * (op1_limb_21_col101))))) + (((op0_limb_14_col65) * (op1_limb_20_col100))))) + (((op0_limb_15_col66) * (op1_limb_19_col99))))) + (((op0_limb_16_col67) * (op1_limb_18_col98))))) + (((op0_limb_17_col68) * (op1_limb_17_col97))))) + (((op0_limb_18_col69) * (op1_limb_16_col96))))) + (((op0_limb_19_col70) * (op1_limb_15_col95))))) + (((op0_limb_20_col71) * (op1_limb_14_col94))))) + (((op0_limb_21_col72) * (op1_limb_13_col93))))) + (((op0_limb_22_col73) * (op1_limb_12_col92))))) + (((op0_limb_23_col74) * (op1_limb_11_col91))))) + (((op0_limb_24_col75) * (op1_limb_10_col90))))) + (((op0_limb_25_col76) * (op1_limb_9_col89))))) + (((op0_limb_26_col77) * (op1_limb_8_col88))))) + (((op0_limb_27_col78) * (op1_limb_7_col87))));
let conv_tmp_5745_97 = ((((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_8_col59) * (op1_limb_27_col107))))) + (((op0_limb_9_col60) * (op1_limb_26_col106))))) + (((op0_limb_10_col61) * (op1_limb_25_col105))))) + (((op0_limb_11_col62) * (op1_limb_24_col104))))) + (((op0_limb_12_col63) * (op1_limb_23_col103))))) + (((op0_limb_13_col64) * (op1_limb_22_col102))))) + (((op0_limb_14_col65) * (op1_limb_21_col101))))) + (((op0_limb_15_col66) * (op1_limb_20_col100))))) + (((op0_limb_16_col67) * (op1_limb_19_col99))))) + (((op0_limb_17_col68) * (op1_limb_18_col98))))) + (((op0_limb_18_col69) * (op1_limb_17_col97))))) + (((op0_limb_19_col70) * (op1_limb_16_col96))))) + (((op0_limb_20_col71) * (op1_limb_15_col95))))) + (((op0_limb_21_col72) * (op1_limb_14_col94))))) + (((op0_limb_22_col73) * (op1_limb_13_col93))))) + (((op0_limb_23_col74) * (op1_limb_12_col92))))) + (((op0_limb_24_col75) * (op1_limb_11_col91))))) + (((op0_limb_25_col76) * (op1_limb_10_col90))))) + (((op0_limb_26_col77) * (op1_limb_9_col89))))) + (((op0_limb_27_col78) * (op1_limb_8_col88))));
let conv_tmp_5745_98 = ((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_9_col60) * (op1_limb_27_col107))))) + (((op0_limb_10_col61) * (op1_limb_26_col106))))) + (((op0_limb_11_col62) * (op1_limb_25_col105))))) + (((op0_limb_12_col63) * (op1_limb_24_col104))))) + (((op0_limb_13_col64) * (op1_limb_23_col103))))) + (((op0_limb_14_col65) * (op1_limb_22_col102))))) + (((op0_limb_15_col66) * (op1_limb_21_col101))))) + (((op0_limb_16_col67) * (op1_limb_20_col100))))) + (((op0_limb_17_col68) * (op1_limb_19_col99))))) + (((op0_limb_18_col69) * (op1_limb_18_col98))))) + (((op0_limb_19_col70) * (op1_limb_17_col97))))) + (((op0_limb_20_col71) * (op1_limb_16_col96))))) + (((op0_limb_21_col72) * (op1_limb_15_col95))))) + (((op0_limb_22_col73) * (op1_limb_14_col94))))) + (((op0_limb_23_col74) * (op1_limb_13_col93))))) + (((op0_limb_24_col75) * (op1_limb_12_col92))))) + (((op0_limb_25_col76) * (op1_limb_11_col91))))) + (((op0_limb_26_col77) * (op1_limb_10_col90))))) + (((op0_limb_27_col78) * (op1_limb_9_col89))));
let conv_tmp_5745_99 = ((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_10_col61) * (op1_limb_27_col107))))) + (((op0_limb_11_col62) * (op1_limb_26_col106))))) + (((op0_limb_12_col63) * (op1_limb_25_col105))))) + (((op0_limb_13_col64) * (op1_limb_24_col104))))) + (((op0_limb_14_col65) * (op1_limb_23_col103))))) + (((op0_limb_15_col66) * (op1_limb_22_col102))))) + (((op0_limb_16_col67) * (op1_limb_21_col101))))) + (((op0_limb_17_col68) * (op1_limb_20_col100))))) + (((op0_limb_18_col69) * (op1_limb_19_col99))))) + (((op0_limb_19_col70) * (op1_limb_18_col98))))) + (((op0_limb_20_col71) * (op1_limb_17_col97))))) + (((op0_limb_21_col72) * (op1_limb_16_col96))))) + (((op0_limb_22_col73) * (op1_limb_15_col95))))) + (((op0_limb_23_col74) * (op1_limb_14_col94))))) + (((op0_limb_24_col75) * (op1_limb_13_col93))))) + (((op0_limb_25_col76) * (op1_limb_12_col92))))) + (((op0_limb_26_col77) * (op1_limb_11_col91))))) + (((op0_limb_27_col78) * (op1_limb_10_col90))));
let conv_tmp_5745_100 = ((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_11_col62) * (op1_limb_27_col107))))) + (((op0_limb_12_col63) * (op1_limb_26_col106))))) + (((op0_limb_13_col64) * (op1_limb_25_col105))))) + (((op0_limb_14_col65) * (op1_limb_24_col104))))) + (((op0_limb_15_col66) * (op1_limb_23_col103))))) + (((op0_limb_16_col67) * (op1_limb_22_col102))))) + (((op0_limb_17_col68) * (op1_limb_21_col101))))) + (((op0_limb_18_col69) * (op1_limb_20_col100))))) + (((op0_limb_19_col70) * (op1_limb_19_col99))))) + (((op0_limb_20_col71) * (op1_limb_18_col98))))) + (((op0_limb_21_col72) * (op1_limb_17_col97))))) + (((op0_limb_22_col73) * (op1_limb_16_col96))))) + (((op0_limb_23_col74) * (op1_limb_15_col95))))) + (((op0_limb_24_col75) * (op1_limb_14_col94))))) + (((op0_limb_25_col76) * (op1_limb_13_col93))))) + (((op0_limb_26_col77) * (op1_limb_12_col92))))) + (((op0_limb_27_col78) * (op1_limb_11_col91))));
let conv_tmp_5745_101 = ((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_12_col63) * (op1_limb_27_col107))))) + (((op0_limb_13_col64) * (op1_limb_26_col106))))) + (((op0_limb_14_col65) * (op1_limb_25_col105))))) + (((op0_limb_15_col66) * (op1_limb_24_col104))))) + (((op0_limb_16_col67) * (op1_limb_23_col103))))) + (((op0_limb_17_col68) * (op1_limb_22_col102))))) + (((op0_limb_18_col69) * (op1_limb_21_col101))))) + (((op0_limb_19_col70) * (op1_limb_20_col100))))) + (((op0_limb_20_col71) * (op1_limb_19_col99))))) + (((op0_limb_21_col72) * (op1_limb_18_col98))))) + (((op0_limb_22_col73) * (op1_limb_17_col97))))) + (((op0_limb_23_col74) * (op1_limb_16_col96))))) + (((op0_limb_24_col75) * (op1_limb_15_col95))))) + (((op0_limb_25_col76) * (op1_limb_14_col94))))) + (((op0_limb_26_col77) * (op1_limb_13_col93))))) + (((op0_limb_27_col78) * (op1_limb_12_col92))));
let conv_tmp_5745_102 = ((((((((((((((((((((((((((((((M31_0) + (((op0_limb_13_col64) * (op1_limb_27_col107))))) + (((op0_limb_14_col65) * (op1_limb_26_col106))))) + (((op0_limb_15_col66) * (op1_limb_25_col105))))) + (((op0_limb_16_col67) * (op1_limb_24_col104))))) + (((op0_limb_17_col68) * (op1_limb_23_col103))))) + (((op0_limb_18_col69) * (op1_limb_22_col102))))) + (((op0_limb_19_col70) * (op1_limb_21_col101))))) + (((op0_limb_20_col71) * (op1_limb_20_col100))))) + (((op0_limb_21_col72) * (op1_limb_19_col99))))) + (((op0_limb_22_col73) * (op1_limb_18_col98))))) + (((op0_limb_23_col74) * (op1_limb_17_col97))))) + (((op0_limb_24_col75) * (op1_limb_16_col96))))) + (((op0_limb_25_col76) * (op1_limb_15_col95))))) + (((op0_limb_26_col77) * (op1_limb_14_col94))))) + (((op0_limb_27_col78) * (op1_limb_13_col93))));
let conv_tmp_5745_103 = ((((((((((((((((((((((((((((M31_0) + (((op0_limb_14_col65) * (op1_limb_27_col107))))) + (((op0_limb_15_col66) * (op1_limb_26_col106))))) + (((op0_limb_16_col67) * (op1_limb_25_col105))))) + (((op0_limb_17_col68) * (op1_limb_24_col104))))) + (((op0_limb_18_col69) * (op1_limb_23_col103))))) + (((op0_limb_19_col70) * (op1_limb_22_col102))))) + (((op0_limb_20_col71) * (op1_limb_21_col101))))) + (((op0_limb_21_col72) * (op1_limb_20_col100))))) + (((op0_limb_22_col73) * (op1_limb_19_col99))))) + (((op0_limb_23_col74) * (op1_limb_18_col98))))) + (((op0_limb_24_col75) * (op1_limb_17_col97))))) + (((op0_limb_25_col76) * (op1_limb_16_col96))))) + (((op0_limb_26_col77) * (op1_limb_15_col95))))) + (((op0_limb_27_col78) * (op1_limb_14_col94))));
let conv_tmp_5745_104 = ((((((((((((((((((((((((((M31_0) + (((op0_limb_15_col66) * (op1_limb_27_col107))))) + (((op0_limb_16_col67) * (op1_limb_26_col106))))) + (((op0_limb_17_col68) * (op1_limb_25_col105))))) + (((op0_limb_18_col69) * (op1_limb_24_col104))))) + (((op0_limb_19_col70) * (op1_limb_23_col103))))) + (((op0_limb_20_col71) * (op1_limb_22_col102))))) + (((op0_limb_21_col72) * (op1_limb_21_col101))))) + (((op0_limb_22_col73) * (op1_limb_20_col100))))) + (((op0_limb_23_col74) * (op1_limb_19_col99))))) + (((op0_limb_24_col75) * (op1_limb_18_col98))))) + (((op0_limb_25_col76) * (op1_limb_17_col97))))) + (((op0_limb_26_col77) * (op1_limb_16_col96))))) + (((op0_limb_27_col78) * (op1_limb_15_col95))));
let conv_tmp_5745_105 = ((((((((((((((((((((((((M31_0) + (((op0_limb_16_col67) * (op1_limb_27_col107))))) + (((op0_limb_17_col68) * (op1_limb_26_col106))))) + (((op0_limb_18_col69) * (op1_limb_25_col105))))) + (((op0_limb_19_col70) * (op1_limb_24_col104))))) + (((op0_limb_20_col71) * (op1_limb_23_col103))))) + (((op0_limb_21_col72) * (op1_limb_22_col102))))) + (((op0_limb_22_col73) * (op1_limb_21_col101))))) + (((op0_limb_23_col74) * (op1_limb_20_col100))))) + (((op0_limb_24_col75) * (op1_limb_19_col99))))) + (((op0_limb_25_col76) * (op1_limb_18_col98))))) + (((op0_limb_26_col77) * (op1_limb_17_col97))))) + (((op0_limb_27_col78) * (op1_limb_16_col96))));
let conv_tmp_5745_106 = ((((((((((((((((((((((M31_0) + (((op0_limb_17_col68) * (op1_limb_27_col107))))) + (((op0_limb_18_col69) * (op1_limb_26_col106))))) + (((op0_limb_19_col70) * (op1_limb_25_col105))))) + (((op0_limb_20_col71) * (op1_limb_24_col104))))) + (((op0_limb_21_col72) * (op1_limb_23_col103))))) + (((op0_limb_22_col73) * (op1_limb_22_col102))))) + (((op0_limb_23_col74) * (op1_limb_21_col101))))) + (((op0_limb_24_col75) * (op1_limb_20_col100))))) + (((op0_limb_25_col76) * (op1_limb_19_col99))))) + (((op0_limb_26_col77) * (op1_limb_18_col98))))) + (((op0_limb_27_col78) * (op1_limb_17_col97))));
let conv_tmp_5745_107 = ((((((((((((((((((((M31_0) + (((op0_limb_18_col69) * (op1_limb_27_col107))))) + (((op0_limb_19_col70) * (op1_limb_26_col106))))) + (((op0_limb_20_col71) * (op1_limb_25_col105))))) + (((op0_limb_21_col72) * (op1_limb_24_col104))))) + (((op0_limb_22_col73) * (op1_limb_23_col103))))) + (((op0_limb_23_col74) * (op1_limb_22_col102))))) + (((op0_limb_24_col75) * (op1_limb_21_col101))))) + (((op0_limb_25_col76) * (op1_limb_20_col100))))) + (((op0_limb_26_col77) * (op1_limb_19_col99))))) + (((op0_limb_27_col78) * (op1_limb_18_col98))));
let conv_tmp_5745_108 = ((((((((((((((((((M31_0) + (((op0_limb_19_col70) * (op1_limb_27_col107))))) + (((op0_limb_20_col71) * (op1_limb_26_col106))))) + (((op0_limb_21_col72) * (op1_limb_25_col105))))) + (((op0_limb_22_col73) * (op1_limb_24_col104))))) + (((op0_limb_23_col74) * (op1_limb_23_col103))))) + (((op0_limb_24_col75) * (op1_limb_22_col102))))) + (((op0_limb_25_col76) * (op1_limb_21_col101))))) + (((op0_limb_26_col77) * (op1_limb_20_col100))))) + (((op0_limb_27_col78) * (op1_limb_19_col99))));
let conv_tmp_5745_109 = ((((((((((((((((M31_0) + (((op0_limb_20_col71) * (op1_limb_27_col107))))) + (((op0_limb_21_col72) * (op1_limb_26_col106))))) + (((op0_limb_22_col73) * (op1_limb_25_col105))))) + (((op0_limb_23_col74) * (op1_limb_24_col104))))) + (((op0_limb_24_col75) * (op1_limb_23_col103))))) + (((op0_limb_25_col76) * (op1_limb_22_col102))))) + (((op0_limb_26_col77) * (op1_limb_21_col101))))) + (((op0_limb_27_col78) * (op1_limb_20_col100))));
let conv_tmp_5745_110 = ((((((((((((((M31_0) + (((op0_limb_21_col72) * (op1_limb_27_col107))))) + (((op0_limb_22_col73) * (op1_limb_26_col106))))) + (((op0_limb_23_col74) * (op1_limb_25_col105))))) + (((op0_limb_24_col75) * (op1_limb_24_col104))))) + (((op0_limb_25_col76) * (op1_limb_23_col103))))) + (((op0_limb_26_col77) * (op1_limb_22_col102))))) + (((op0_limb_27_col78) * (op1_limb_21_col101))));
let conv_tmp_5745_111 = ((((((((((((M31_0) + (((op0_limb_22_col73) * (op1_limb_27_col107))))) + (((op0_limb_23_col74) * (op1_limb_26_col106))))) + (((op0_limb_24_col75) * (op1_limb_25_col105))))) + (((op0_limb_25_col76) * (op1_limb_24_col104))))) + (((op0_limb_26_col77) * (op1_limb_23_col103))))) + (((op0_limb_27_col78) * (op1_limb_22_col102))));
let conv_tmp_5745_112 = ((((((((((M31_0) + (((op0_limb_23_col74) * (op1_limb_27_col107))))) + (((op0_limb_24_col75) * (op1_limb_26_col106))))) + (((op0_limb_25_col76) * (op1_limb_25_col105))))) + (((op0_limb_26_col77) * (op1_limb_24_col104))))) + (((op0_limb_27_col78) * (op1_limb_23_col103))));
let conv_tmp_5745_113 = ((((((((M31_0) + (((op0_limb_24_col75) * (op1_limb_27_col107))))) + (((op0_limb_25_col76) * (op1_limb_26_col106))))) + (((op0_limb_26_col77) * (op1_limb_25_col105))))) + (((op0_limb_27_col78) * (op1_limb_24_col104))));
let conv_tmp_5745_114 = ((((((M31_0) + (((op0_limb_25_col76) * (op1_limb_27_col107))))) + (((op0_limb_26_col77) * (op1_limb_26_col106))))) + (((op0_limb_27_col78) * (op1_limb_25_col105))));
let conv_tmp_5745_115 = ((((M31_0) + (((op0_limb_26_col77) * (op1_limb_27_col107))))) + (((op0_limb_27_col78) * (op1_limb_26_col106))));
let conv_tmp_5745_116 = ((M31_0) + (((op0_limb_27_col78) * (op1_limb_27_col107))));
let conv_mod_tmp_5745_117 = ((((((M31_0) + (((M31_32) * (conv_tmp_5745_62))))) - (((M31_4) * (conv_tmp_5745_83))))) + (((M31_8) * (conv_tmp_5745_111))));
let conv_mod_tmp_5745_118 = ((((((((M31_0) + (((M31_1) * (conv_tmp_5745_62))))) + (((M31_32) * (conv_tmp_5745_63))))) - (((M31_4) * (conv_tmp_5745_84))))) + (((M31_8) * (conv_tmp_5745_112))));
let conv_mod_tmp_5745_119 = ((((((((M31_0) + (((M31_1) * (conv_tmp_5745_63))))) + (((M31_32) * (conv_tmp_5745_64))))) - (((M31_4) * (conv_tmp_5745_85))))) + (((M31_8) * (conv_tmp_5745_113))));
let conv_mod_tmp_5745_120 = ((((((((M31_0) + (((M31_1) * (conv_tmp_5745_64))))) + (((M31_32) * (conv_tmp_5745_65))))) - (((M31_4) * (conv_tmp_5745_86))))) + (((M31_8) * (conv_tmp_5745_114))));
let conv_mod_tmp_5745_121 = ((((((((M31_0) + (((M31_1) * (conv_tmp_5745_65))))) + (((M31_32) * (conv_tmp_5745_66))))) - (((M31_4) * (conv_tmp_5745_87))))) + (((M31_8) * (conv_tmp_5745_115))));
let conv_mod_tmp_5745_122 = ((((((((M31_0) + (((M31_1) * (conv_tmp_5745_66))))) + (((M31_32) * (conv_tmp_5745_67))))) - (((M31_4) * (conv_tmp_5745_88))))) + (((M31_8) * (conv_tmp_5745_116))));
let conv_mod_tmp_5745_123 = ((((((M31_0) + (((M31_1) * (conv_tmp_5745_67))))) + (((M31_32) * (conv_tmp_5745_68))))) - (((M31_4) * (conv_tmp_5745_89))));
let conv_mod_tmp_5745_124 = ((((((((M31_0) + (((M31_2) * (conv_tmp_5745_62))))) + (((M31_1) * (conv_tmp_5745_68))))) + (((M31_32) * (conv_tmp_5745_69))))) - (((M31_4) * (conv_tmp_5745_90))));
let conv_mod_tmp_5745_125 = ((((((((M31_0) + (((M31_2) * (conv_tmp_5745_63))))) + (((M31_1) * (conv_tmp_5745_69))))) + (((M31_32) * (conv_tmp_5745_70))))) - (((M31_4) * (conv_tmp_5745_91))));
let conv_mod_tmp_5745_126 = ((((((((M31_0) + (((M31_2) * (conv_tmp_5745_64))))) + (((M31_1) * (conv_tmp_5745_70))))) + (((M31_32) * (conv_tmp_5745_71))))) - (((M31_4) * (conv_tmp_5745_92))));
let conv_mod_tmp_5745_127 = ((((((((M31_0) + (((M31_2) * (conv_tmp_5745_65))))) + (((M31_1) * (conv_tmp_5745_71))))) + (((M31_32) * (conv_tmp_5745_72))))) - (((M31_4) * (conv_tmp_5745_93))));
let conv_mod_tmp_5745_128 = ((((((((M31_0) + (((M31_2) * (conv_tmp_5745_66))))) + (((M31_1) * (conv_tmp_5745_72))))) + (((M31_32) * (conv_tmp_5745_73))))) - (((M31_4) * (conv_tmp_5745_94))));
let conv_mod_tmp_5745_129 = ((((((((M31_0) + (((M31_2) * (conv_tmp_5745_67))))) + (((M31_1) * (conv_tmp_5745_73))))) + (((M31_32) * (conv_tmp_5745_74))))) - (((M31_4) * (conv_tmp_5745_95))));
let conv_mod_tmp_5745_130 = ((((((((M31_0) + (((M31_2) * (conv_tmp_5745_68))))) + (((M31_1) * (conv_tmp_5745_74))))) + (((M31_32) * (conv_tmp_5745_75))))) - (((M31_4) * (conv_tmp_5745_96))));
let conv_mod_tmp_5745_131 = ((((((((M31_0) + (((M31_2) * (conv_tmp_5745_69))))) + (((M31_1) * (conv_tmp_5745_75))))) + (((M31_32) * (conv_tmp_5745_76))))) - (((M31_4) * (conv_tmp_5745_97))));
let conv_mod_tmp_5745_132 = ((((((((M31_0) + (((M31_2) * (conv_tmp_5745_70))))) + (((M31_1) * (conv_tmp_5745_76))))) + (((M31_32) * (conv_tmp_5745_77))))) - (((M31_4) * (conv_tmp_5745_98))));
let conv_mod_tmp_5745_133 = ((((((((M31_0) + (((M31_2) * (conv_tmp_5745_71))))) + (((M31_1) * (conv_tmp_5745_77))))) + (((M31_32) * (conv_tmp_5745_78))))) - (((M31_4) * (conv_tmp_5745_99))));
let conv_mod_tmp_5745_134 = ((((((((M31_0) + (((M31_2) * (conv_tmp_5745_72))))) + (((M31_1) * (conv_tmp_5745_78))))) + (((M31_32) * (conv_tmp_5745_79))))) - (((M31_4) * (conv_tmp_5745_100))));
let conv_mod_tmp_5745_135 = ((((((((M31_0) + (((M31_2) * (conv_tmp_5745_73))))) + (((M31_1) * (conv_tmp_5745_79))))) + (((M31_32) * (conv_tmp_5745_80))))) - (((M31_4) * (conv_tmp_5745_101))));
let conv_mod_tmp_5745_136 = ((((((((M31_0) + (((M31_2) * (conv_tmp_5745_74))))) + (((M31_1) * (conv_tmp_5745_80))))) + (((M31_32) * (conv_tmp_5745_81))))) - (((M31_4) * (conv_tmp_5745_102))));
let conv_mod_tmp_5745_137 = ((((((((M31_0) + (((M31_2) * (conv_tmp_5745_75))))) + (((M31_1) * (conv_tmp_5745_81))))) + (((M31_32) * (conv_tmp_5745_82))))) - (((M31_4) * (conv_tmp_5745_103))));
let conv_mod_tmp_5745_138 = ((((((((M31_0) + (((M31_2) * (conv_tmp_5745_76))))) + (((M31_1) * (conv_tmp_5745_82))))) - (((M31_4) * (conv_tmp_5745_104))))) + (((M31_64) * (conv_tmp_5745_111))));
let conv_mod_tmp_5745_139 = ((((((((M31_0) + (((M31_2) * (conv_tmp_5745_77))))) - (((M31_4) * (conv_tmp_5745_105))))) + (((M31_2) * (conv_tmp_5745_111))))) + (((M31_64) * (conv_tmp_5745_112))));
let conv_mod_tmp_5745_140 = ((((((((M31_0) + (((M31_2) * (conv_tmp_5745_78))))) - (((M31_4) * (conv_tmp_5745_106))))) + (((M31_2) * (conv_tmp_5745_112))))) + (((M31_64) * (conv_tmp_5745_113))));
let conv_mod_tmp_5745_141 = ((((((((M31_0) + (((M31_2) * (conv_tmp_5745_79))))) - (((M31_4) * (conv_tmp_5745_107))))) + (((M31_2) * (conv_tmp_5745_113))))) + (((M31_64) * (conv_tmp_5745_114))));
let conv_mod_tmp_5745_142 = ((((((((M31_0) + (((M31_2) * (conv_tmp_5745_80))))) - (((M31_4) * (conv_tmp_5745_108))))) + (((M31_2) * (conv_tmp_5745_114))))) + (((M31_64) * (conv_tmp_5745_115))));
let conv_mod_tmp_5745_143 = ((((((((M31_0) + (((M31_2) * (conv_tmp_5745_81))))) - (((M31_4) * (conv_tmp_5745_109))))) + (((M31_2) * (conv_tmp_5745_115))))) + (((M31_64) * (conv_tmp_5745_116))));
let conv_mod_tmp_5745_144 = ((((((M31_0) + (((M31_2) * (conv_tmp_5745_82))))) - (((M31_4) * (conv_tmp_5745_110))))) + (((M31_2) * (conv_tmp_5745_116))));
let k_mod_2_18_biased_tmp_5745_145 = ((((((PackedUInt32::from_m31(((conv_mod_tmp_5745_117) + (M31_134217728)))) + (((((PackedUInt32::from_m31(((conv_mod_tmp_5745_118) + (M31_134217728)))) & (UInt32_511))) << (UInt32_9))))) + (UInt32_65536))) & (UInt32_262143));
let k_col165 = ((k_mod_2_18_biased_tmp_5745_145.low().as_m31()) + (((((k_mod_2_18_biased_tmp_5745_145.high().as_m31()) - (M31_1))) * (M31_65536))));
            (*trace.data[165].get()).data[row_index] = k_col165;

(*sub_components_inputs
                .range_check_19_inputs[0].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((k_col165) + (M31_262144))].unpack());
            
(*lookup_data.rangecheck_19[0].get())[row_index] = [((k_col165) + (M31_262144))];
let carry_0_col166 = ((((((conv_mod_tmp_5745_117) - (((M31_1) * (k_col165))))) + (M31_0))) * (M31_4194304));
            (*trace.data[166].get()).data[row_index] = carry_0_col166;

(*sub_components_inputs
                .range_check_19_inputs[1].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_0_col166) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[1].get())[row_index] = [((carry_0_col166) + (M31_131072))];
let carry_1_col167 = ((((conv_mod_tmp_5745_118) + (carry_0_col166))) * (M31_4194304));
            (*trace.data[167].get()).data[row_index] = carry_1_col167;

(*sub_components_inputs
                .range_check_19_inputs[2].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_1_col167) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[2].get())[row_index] = [((carry_1_col167) + (M31_131072))];
let carry_2_col168 = ((((conv_mod_tmp_5745_119) + (carry_1_col167))) * (M31_4194304));
            (*trace.data[168].get()).data[row_index] = carry_2_col168;

(*sub_components_inputs
                .range_check_19_inputs[3].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_2_col168) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[3].get())[row_index] = [((carry_2_col168) + (M31_131072))];
let carry_3_col169 = ((((conv_mod_tmp_5745_120) + (carry_2_col168))) * (M31_4194304));
            (*trace.data[169].get()).data[row_index] = carry_3_col169;

(*sub_components_inputs
                .range_check_19_inputs[4].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_3_col169) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[4].get())[row_index] = [((carry_3_col169) + (M31_131072))];
let carry_4_col170 = ((((conv_mod_tmp_5745_121) + (carry_3_col169))) * (M31_4194304));
            (*trace.data[170].get()).data[row_index] = carry_4_col170;

(*sub_components_inputs
                .range_check_19_inputs[5].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_4_col170) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[5].get())[row_index] = [((carry_4_col170) + (M31_131072))];
let carry_5_col171 = ((((conv_mod_tmp_5745_122) + (carry_4_col170))) * (M31_4194304));
            (*trace.data[171].get()).data[row_index] = carry_5_col171;

(*sub_components_inputs
                .range_check_19_inputs[6].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_5_col171) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[6].get())[row_index] = [((carry_5_col171) + (M31_131072))];
let carry_6_col172 = ((((conv_mod_tmp_5745_123) + (carry_5_col171))) * (M31_4194304));
            (*trace.data[172].get()).data[row_index] = carry_6_col172;

(*sub_components_inputs
                .range_check_19_inputs[7].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_6_col172) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[7].get())[row_index] = [((carry_6_col172) + (M31_131072))];
let carry_7_col173 = ((((conv_mod_tmp_5745_124) + (carry_6_col172))) * (M31_4194304));
            (*trace.data[173].get()).data[row_index] = carry_7_col173;

(*sub_components_inputs
                .range_check_19_inputs[8].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_7_col173) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[8].get())[row_index] = [((carry_7_col173) + (M31_131072))];
let carry_8_col174 = ((((conv_mod_tmp_5745_125) + (carry_7_col173))) * (M31_4194304));
            (*trace.data[174].get()).data[row_index] = carry_8_col174;

(*sub_components_inputs
                .range_check_19_inputs[9].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_8_col174) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[9].get())[row_index] = [((carry_8_col174) + (M31_131072))];
let carry_9_col175 = ((((conv_mod_tmp_5745_126) + (carry_8_col174))) * (M31_4194304));
            (*trace.data[175].get()).data[row_index] = carry_9_col175;

(*sub_components_inputs
                .range_check_19_inputs[10].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_9_col175) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[10].get())[row_index] = [((carry_9_col175) + (M31_131072))];
let carry_10_col176 = ((((conv_mod_tmp_5745_127) + (carry_9_col175))) * (M31_4194304));
            (*trace.data[176].get()).data[row_index] = carry_10_col176;

(*sub_components_inputs
                .range_check_19_inputs[11].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_10_col176) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[11].get())[row_index] = [((carry_10_col176) + (M31_131072))];
let carry_11_col177 = ((((conv_mod_tmp_5745_128) + (carry_10_col176))) * (M31_4194304));
            (*trace.data[177].get()).data[row_index] = carry_11_col177;

(*sub_components_inputs
                .range_check_19_inputs[12].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_11_col177) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[12].get())[row_index] = [((carry_11_col177) + (M31_131072))];
let carry_12_col178 = ((((conv_mod_tmp_5745_129) + (carry_11_col177))) * (M31_4194304));
            (*trace.data[178].get()).data[row_index] = carry_12_col178;

(*sub_components_inputs
                .range_check_19_inputs[13].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_12_col178) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[13].get())[row_index] = [((carry_12_col178) + (M31_131072))];
let carry_13_col179 = ((((conv_mod_tmp_5745_130) + (carry_12_col178))) * (M31_4194304));
            (*trace.data[179].get()).data[row_index] = carry_13_col179;

(*sub_components_inputs
                .range_check_19_inputs[14].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_13_col179) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[14].get())[row_index] = [((carry_13_col179) + (M31_131072))];
let carry_14_col180 = ((((conv_mod_tmp_5745_131) + (carry_13_col179))) * (M31_4194304));
            (*trace.data[180].get()).data[row_index] = carry_14_col180;

(*sub_components_inputs
                .range_check_19_inputs[15].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_14_col180) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[15].get())[row_index] = [((carry_14_col180) + (M31_131072))];
let carry_15_col181 = ((((conv_mod_tmp_5745_132) + (carry_14_col180))) * (M31_4194304));
            (*trace.data[181].get()).data[row_index] = carry_15_col181;

(*sub_components_inputs
                .range_check_19_inputs[16].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_15_col181) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[16].get())[row_index] = [((carry_15_col181) + (M31_131072))];
let carry_16_col182 = ((((conv_mod_tmp_5745_133) + (carry_15_col181))) * (M31_4194304));
            (*trace.data[182].get()).data[row_index] = carry_16_col182;

(*sub_components_inputs
                .range_check_19_inputs[17].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_16_col182) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[17].get())[row_index] = [((carry_16_col182) + (M31_131072))];
let carry_17_col183 = ((((conv_mod_tmp_5745_134) + (carry_16_col182))) * (M31_4194304));
            (*trace.data[183].get()).data[row_index] = carry_17_col183;

(*sub_components_inputs
                .range_check_19_inputs[18].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_17_col183) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[18].get())[row_index] = [((carry_17_col183) + (M31_131072))];
let carry_18_col184 = ((((conv_mod_tmp_5745_135) + (carry_17_col183))) * (M31_4194304));
            (*trace.data[184].get()).data[row_index] = carry_18_col184;

(*sub_components_inputs
                .range_check_19_inputs[19].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_18_col184) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[19].get())[row_index] = [((carry_18_col184) + (M31_131072))];
let carry_19_col185 = ((((conv_mod_tmp_5745_136) + (carry_18_col184))) * (M31_4194304));
            (*trace.data[185].get()).data[row_index] = carry_19_col185;

(*sub_components_inputs
                .range_check_19_inputs[20].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_19_col185) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[20].get())[row_index] = [((carry_19_col185) + (M31_131072))];
let carry_20_col186 = ((((conv_mod_tmp_5745_137) + (carry_19_col185))) * (M31_4194304));
            (*trace.data[186].get()).data[row_index] = carry_20_col186;

(*sub_components_inputs
                .range_check_19_inputs[21].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_20_col186) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[21].get())[row_index] = [((carry_20_col186) + (M31_131072))];
let carry_21_col187 = ((((((conv_mod_tmp_5745_138) - (((M31_136) * (k_col165))))) + (carry_20_col186))) * (M31_4194304));
            (*trace.data[187].get()).data[row_index] = carry_21_col187;

(*sub_components_inputs
                .range_check_19_inputs[22].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_21_col187) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[22].get())[row_index] = [((carry_21_col187) + (M31_131072))];
let carry_22_col188 = ((((conv_mod_tmp_5745_139) + (carry_21_col187))) * (M31_4194304));
            (*trace.data[188].get()).data[row_index] = carry_22_col188;

(*sub_components_inputs
                .range_check_19_inputs[23].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_22_col188) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[23].get())[row_index] = [((carry_22_col188) + (M31_131072))];
let carry_23_col189 = ((((conv_mod_tmp_5745_140) + (carry_22_col188))) * (M31_4194304));
            (*trace.data[189].get()).data[row_index] = carry_23_col189;

(*sub_components_inputs
                .range_check_19_inputs[24].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_23_col189) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[24].get())[row_index] = [((carry_23_col189) + (M31_131072))];
let carry_24_col190 = ((((conv_mod_tmp_5745_141) + (carry_23_col189))) * (M31_4194304));
            (*trace.data[190].get()).data[row_index] = carry_24_col190;

(*sub_components_inputs
                .range_check_19_inputs[25].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_24_col190) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[25].get())[row_index] = [((carry_24_col190) + (M31_131072))];
let carry_25_col191 = ((((conv_mod_tmp_5745_142) + (carry_24_col190))) * (M31_4194304));
            (*trace.data[191].get()).data[row_index] = carry_25_col191;

(*sub_components_inputs
                .range_check_19_inputs[26].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_25_col191) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[26].get())[row_index] = [((carry_25_col191) + (M31_131072))];
let carry_26_col192 = ((((conv_mod_tmp_5745_143) + (carry_25_col191))) * (M31_4194304));
            (*trace.data[192].get()).data[row_index] = carry_26_col192;

(*sub_components_inputs
                .range_check_19_inputs[27].get())
                    [row_index * N_LANES..(row_index+1) * N_LANES]
                .copy_from_slice(&[((carry_26_col192) + (M31_131072))].unpack());
            
(*lookup_data.rangecheck_19[27].get())[row_index] = [((carry_26_col192) + (M31_131072))];


            


            
let res_tmp_5745_146 = ((((((PackedFelt252::from_m31(res_op1_tmp_5745_22)) * (PackedFelt252::from_limbs([op1_limb_0_col80, op1_limb_1_col81, op1_limb_2_col82, op1_limb_3_col83, op1_limb_4_col84, op1_limb_5_col85, op1_limb_6_col86, op1_limb_7_col87, op1_limb_8_col88, op1_limb_9_col89, op1_limb_10_col90, op1_limb_11_col91, op1_limb_12_col92, op1_limb_13_col93, op1_limb_14_col94, op1_limb_15_col95, op1_limb_16_col96, op1_limb_17_col97, op1_limb_18_col98, op1_limb_19_col99, op1_limb_20_col100, op1_limb_21_col101, op1_limb_22_col102, op1_limb_23_col103, op1_limb_24_col104, op1_limb_25_col105, op1_limb_26_col106, op1_limb_27_col107])))) + (((PackedFelt252::from_m31(res_mul_col12)) * (mul_res_tmp_5745_61))))) + (((PackedFelt252::from_m31(res_add_col11)) * (add_res_tmp_5745_32))));
let res_limb_0_col193 = res_tmp_5745_146.get_m31(0);
            (*trace.data[193].get()).data[row_index] = res_limb_0_col193;
let res_limb_1_col194 = res_tmp_5745_146.get_m31(1);
            (*trace.data[194].get()).data[row_index] = res_limb_1_col194;
let res_limb_2_col195 = res_tmp_5745_146.get_m31(2);
            (*trace.data[195].get()).data[row_index] = res_limb_2_col195;
let res_limb_3_col196 = res_tmp_5745_146.get_m31(3);
            (*trace.data[196].get()).data[row_index] = res_limb_3_col196;
let res_limb_4_col197 = res_tmp_5745_146.get_m31(4);
            (*trace.data[197].get()).data[row_index] = res_limb_4_col197;
let res_limb_5_col198 = res_tmp_5745_146.get_m31(5);
            (*trace.data[198].get()).data[row_index] = res_limb_5_col198;
let res_limb_6_col199 = res_tmp_5745_146.get_m31(6);
            (*trace.data[199].get()).data[row_index] = res_limb_6_col199;
let res_limb_7_col200 = res_tmp_5745_146.get_m31(7);
            (*trace.data[200].get()).data[row_index] = res_limb_7_col200;
let res_limb_8_col201 = res_tmp_5745_146.get_m31(8);
            (*trace.data[201].get()).data[row_index] = res_limb_8_col201;
let res_limb_9_col202 = res_tmp_5745_146.get_m31(9);
            (*trace.data[202].get()).data[row_index] = res_limb_9_col202;
let res_limb_10_col203 = res_tmp_5745_146.get_m31(10);
            (*trace.data[203].get()).data[row_index] = res_limb_10_col203;
let res_limb_11_col204 = res_tmp_5745_146.get_m31(11);
            (*trace.data[204].get()).data[row_index] = res_limb_11_col204;
let res_limb_12_col205 = res_tmp_5745_146.get_m31(12);
            (*trace.data[205].get()).data[row_index] = res_limb_12_col205;
let res_limb_13_col206 = res_tmp_5745_146.get_m31(13);
            (*trace.data[206].get()).data[row_index] = res_limb_13_col206;
let res_limb_14_col207 = res_tmp_5745_146.get_m31(14);
            (*trace.data[207].get()).data[row_index] = res_limb_14_col207;
let res_limb_15_col208 = res_tmp_5745_146.get_m31(15);
            (*trace.data[208].get()).data[row_index] = res_limb_15_col208;
let res_limb_16_col209 = res_tmp_5745_146.get_m31(16);
            (*trace.data[209].get()).data[row_index] = res_limb_16_col209;
let res_limb_17_col210 = res_tmp_5745_146.get_m31(17);
            (*trace.data[210].get()).data[row_index] = res_limb_17_col210;
let res_limb_18_col211 = res_tmp_5745_146.get_m31(18);
            (*trace.data[211].get()).data[row_index] = res_limb_18_col211;
let res_limb_19_col212 = res_tmp_5745_146.get_m31(19);
            (*trace.data[212].get()).data[row_index] = res_limb_19_col212;
let res_limb_20_col213 = res_tmp_5745_146.get_m31(20);
            (*trace.data[213].get()).data[row_index] = res_limb_20_col213;
let res_limb_21_col214 = res_tmp_5745_146.get_m31(21);
            (*trace.data[214].get()).data[row_index] = res_limb_21_col214;
let res_limb_22_col215 = res_tmp_5745_146.get_m31(22);
            (*trace.data[215].get()).data[row_index] = res_limb_22_col215;
let res_limb_23_col216 = res_tmp_5745_146.get_m31(23);
            (*trace.data[216].get()).data[row_index] = res_limb_23_col216;
let res_limb_24_col217 = res_tmp_5745_146.get_m31(24);
            (*trace.data[217].get()).data[row_index] = res_limb_24_col217;
let res_limb_25_col218 = res_tmp_5745_146.get_m31(25);
            (*trace.data[218].get()).data[row_index] = res_limb_25_col218;
let res_limb_26_col219 = res_tmp_5745_146.get_m31(26);
            (*trace.data[219].get()).data[row_index] = res_limb_26_col219;
let res_limb_27_col220 = res_tmp_5745_146.get_m31(27);
            (*trace.data[220].get()).data[row_index] = res_limb_27_col220;


            


            //UpdateRegisters.

            


            //CondFelt252AsRelImm.

            


            //CondDecodeSmallSign.

            
let msb_tmp_5745_148 = res_limb_27_col220.eq(M31_256);
let msb_col221 = msb_tmp_5745_148.as_m31();
            (*trace.data[221].get()).data[row_index] = msb_col221;
let mid_limbs_set_tmp_5745_149 = res_limb_20_col213.eq(M31_511);
let mid_limbs_set_col222 = mid_limbs_set_tmp_5745_149.as_m31();
            (*trace.data[222].get()).data[row_index] = mid_limbs_set_col222;


            


            
let diff_from_p_tmp_5745_150 = ((dst_limb_0_col22) - (M31_1));
let diff_from_p_tmp_5745_151 = ((dst_limb_21_col43) - (M31_136));
let diff_from_p_tmp_5745_152 = ((dst_limb_27_col49) - (M31_256));
let dst_sum_squares_inv_col223 = ((M31_1) .div (((((((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (((diff_from_p_tmp_5745_150) * (diff_from_p_tmp_5745_150))))) + (dst_limb_1_col23))) + (dst_limb_2_col24))) + (dst_limb_3_col25))) + (dst_limb_4_col26))) + (dst_limb_5_col27))) + (dst_limb_6_col28))) + (dst_limb_7_col29))) + (dst_limb_8_col30))) + (dst_limb_9_col31))) + (dst_limb_10_col32))) + (dst_limb_11_col33))) + (dst_limb_12_col34))) + (dst_limb_13_col35))) + (dst_limb_14_col36))) + (dst_limb_15_col37))) + (dst_limb_16_col38))) + (dst_limb_17_col39))) + (dst_limb_18_col40))) + (dst_limb_19_col41))) + (dst_limb_20_col42))) + (((diff_from_p_tmp_5745_151) * (diff_from_p_tmp_5745_151))))) + (dst_limb_22_col44))) + (dst_limb_23_col45))) + (dst_limb_24_col46))) + (dst_limb_25_col47))) + (dst_limb_26_col48))) + (((diff_from_p_tmp_5745_152) * (diff_from_p_tmp_5745_152))))));
            (*trace.data[223].get()).data[row_index] = dst_sum_squares_inv_col223;
let dst_is_zero_tmp_5745_153 = ((((((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (dst_limb_0_col22))) + (dst_limb_1_col23))) + (dst_limb_2_col24))) + (dst_limb_3_col25))) + (dst_limb_4_col26))) + (dst_limb_5_col27))) + (dst_limb_6_col28))) + (dst_limb_7_col29))) + (dst_limb_8_col30))) + (dst_limb_9_col31))) + (dst_limb_10_col32))) + (dst_limb_11_col33))) + (dst_limb_12_col34))) + (dst_limb_13_col35))) + (dst_limb_14_col36))) + (dst_limb_15_col37))) + (dst_limb_16_col38))) + (dst_limb_17_col39))) + (dst_limb_18_col40))) + (dst_limb_19_col41))) + (dst_limb_20_col42))) + (dst_limb_21_col43))) + (dst_limb_22_col44))) + (dst_limb_23_col45))) + (dst_limb_24_col46))) + (dst_limb_25_col47))) + (dst_limb_26_col48))) + (dst_limb_27_col49)).eq(M31_0);
let dst_sum_inv_col224 = ((M31_1) .div (((((((((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (dst_limb_0_col22))) + (dst_limb_1_col23))) + (dst_limb_2_col24))) + (dst_limb_3_col25))) + (dst_limb_4_col26))) + (dst_limb_5_col27))) + (dst_limb_6_col28))) + (dst_limb_7_col29))) + (dst_limb_8_col30))) + (dst_limb_9_col31))) + (dst_limb_10_col32))) + (dst_limb_11_col33))) + (dst_limb_12_col34))) + (dst_limb_13_col35))) + (dst_limb_14_col36))) + (dst_limb_15_col37))) + (dst_limb_16_col38))) + (dst_limb_17_col39))) + (dst_limb_18_col40))) + (dst_limb_19_col41))) + (dst_limb_20_col42))) + (dst_limb_21_col43))) + (dst_limb_22_col44))) + (dst_limb_23_col45))) + (dst_limb_24_col46))) + (dst_limb_25_col47))) + (dst_limb_26_col48))) + (dst_limb_27_col49))) + (dst_is_zero_tmp_5745_153.as_m31()))));
            (*trace.data[224].get()).data[row_index] = dst_sum_inv_col224;
let op1_as_rel_imm_cond_col225 = ((pc_update_jnz_col15) * (((((((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (dst_limb_0_col22))) + (dst_limb_1_col23))) + (dst_limb_2_col24))) + (dst_limb_3_col25))) + (dst_limb_4_col26))) + (dst_limb_5_col27))) + (dst_limb_6_col28))) + (dst_limb_7_col29))) + (dst_limb_8_col30))) + (dst_limb_9_col31))) + (dst_limb_10_col32))) + (dst_limb_11_col33))) + (dst_limb_12_col34))) + (dst_limb_13_col35))) + (dst_limb_14_col36))) + (dst_limb_15_col37))) + (dst_limb_16_col38))) + (dst_limb_17_col39))) + (dst_limb_18_col40))) + (dst_limb_19_col41))) + (dst_limb_20_col42))) + (dst_limb_21_col43))) + (dst_limb_22_col44))) + (dst_limb_23_col45))) + (dst_limb_24_col46))) + (dst_limb_25_col47))) + (dst_limb_26_col48))) + (dst_limb_27_col49))));
            (*trace.data[225].get()).data[row_index] = op1_as_rel_imm_cond_col225;


            //CondFelt252AsRelImm.

            


            //CondDecodeSmallSign.

            
let msb_tmp_5745_154 = op1_limb_27_col107.eq(M31_256);
let msb_col226 = msb_tmp_5745_154.as_m31();
            (*trace.data[226].get()).data[row_index] = msb_col226;
let mid_limbs_set_tmp_5745_155 = op1_limb_20_col100.eq(M31_511);
let mid_limbs_set_col227 = mid_limbs_set_tmp_5745_155.as_m31();
            (*trace.data[227].get()).data[row_index] = mid_limbs_set_col227;


            


            
let next_pc_jnz_col228 = ((((dst_is_zero_tmp_5745_153.as_m31()) * (((input_pc_col0) + (((M31_1) + (op1_imm_col8))))))) + (((((M31_1) - (dst_is_zero_tmp_5745_153.as_m31()))) * (((input_pc_col0) + (((((((((op1_limb_0_col80) + (((op1_limb_1_col81) * (M31_512))))) + (((op1_limb_2_col82) * (M31_262144))))) - (msb_col226))) - (((M31_134217728) * (mid_limbs_set_col227))))))))));
            (*trace.data[228].get()).data[row_index] = next_pc_jnz_col228;


            
(*lookup_data.opcodes[0].get())[row_index] = [input_pc_col0, input_ap_col1, input_fp_col2];
(*lookup_data.opcodes[1].get())[row_index] = [((((((((pc_update_regular_tmp_5745_23) * (((input_pc_col0) + (((M31_1) + (op1_imm_col8))))))) + (((pc_update_jump_col13) * (((((res_limb_0_col193) + (((res_limb_1_col194) * (M31_512))))) + (((res_limb_2_col195) * (M31_262144))))))))) + (((pc_update_jump_rel_col14) * (((input_pc_col0) + (((((((((res_limb_0_col193) + (((res_limb_1_col194) * (M31_512))))) + (((res_limb_2_col195) * (M31_262144))))) - (msb_col221))) - (((M31_134217728) * (mid_limbs_set_col222))))))))))) + (((pc_update_jnz_col15) * (next_pc_jnz_col228)))), ((((((input_ap_col1) + (((ap_update_add_col16) * (((((((((res_limb_0_col193) + (((res_limb_1_col194) * (M31_512))))) + (((res_limb_2_col195) * (M31_262144))))) - (msb_col221))) - (((M31_134217728) * (mid_limbs_set_col222))))))))) + (((ap_update_add_1_col17) * (M31_1))))) + (((opcode_call_col18) * (M31_2)))), ((((((fp_update_regular_tmp_5745_25) * (input_fp_col2))) + (((opcode_ret_col19) * (((((dst_limb_0_col22) + (((dst_limb_1_col23) * (M31_512))))) + (((dst_limb_2_col24) * (M31_262144))))))))) + (((opcode_call_col18) * (((input_ap_col1) + (M31_2))))))];

        }
    });

    let trace = match Rc::try_unwrap(trace) {
        Ok(trace) => trace.inner(),
        Err(_) => panic!("could not unwrap trace"),
    };
    let sub_components_inputs = match Rc::try_unwrap(sub_components_inputs) {
        Ok(sub_components_inputs) => sub_components_inputs,
        Err(_) => panic!("could not unwrap sub_components_inputs"),
    };
    let lookup_data = match Rc::try_unwrap(lookup_data) {
        Ok(lookup_data) => lookup_data,
        Err(_) => panic!("could not unwrap lookup_data"),
    };
    (trace, sub_components_inputs, lookup_data)
}

pub struct LookupData
{pub memoryaddresstoid: [UnsafeCell<Vec<[PackedM31; 2]>>; 3],pub memoryidtobig: [UnsafeCell<Vec<[PackedM31; 29]>>; 3],pub opcodes: [UnsafeCell<Vec<[PackedM31; 3]>>; 2],pub rangecheck_19: [UnsafeCell<Vec<[PackedM31; 1]>>; 28],pub rangecheck_9_9: [UnsafeCell<Vec<[PackedM31; 2]>>; 28],pub verifyinstruction: [UnsafeCell<Vec<[PackedM31; 19]>>; 1],}
impl LookupData {
    unsafe fn uninitialized(len: usize) -> Self {
        let lookup_data = Self {memoryaddresstoid: [Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),],memoryidtobig: [Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),],opcodes: [Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),],rangecheck_19: [Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),],rangecheck_9_9: [Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),Vec::with_capacity(len).into(),],verifyinstruction: [Vec::with_capacity(len).into(),],};
        lookup_data.memoryaddresstoid
        .iter()
        .for_each(|vec| {
            (*vec.get()).set_len(len);
        });lookup_data.memoryidtobig
        .iter()
        .for_each(|vec| {
            (*vec.get()).set_len(len);
        });lookup_data.opcodes
        .iter()
        .for_each(|vec| {
            (*vec.get()).set_len(len);
        });lookup_data.rangecheck_19
        .iter()
        .for_each(|vec| {
            (*vec.get()).set_len(len);
        });lookup_data.rangecheck_9_9
        .iter()
        .for_each(|vec| {
            (*vec.get()).set_len(len);
        });lookup_data.verifyinstruction
        .iter()
        .for_each(|vec| {
            (*vec.get()).set_len(len);
        });
        lookup_data
    }
}

pub struct InteractionClaimGenerator {
    pub n_calls: usize,
    pub lookup_data: LookupData,
}
impl InteractionClaimGenerator {

    pub fn write_interaction_trace(
        mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        memoryaddresstoid_lookup_elements:
            &relations::MemoryAddressToId,
        memoryidtobig_lookup_elements:
            &relations::MemoryIdToBig,
        opcodes_lookup_elements:
            &relations::Opcodes,
        rangecheck_19_lookup_elements:
            &relations::RangeCheck_19,
        rangecheck_9_9_lookup_elements:
            &relations::RangeCheck_9_9,
        verifyinstruction_lookup_elements:
            &relations::VerifyInstruction,
    ) -> InteractionClaim {
        let log_size = std::cmp::max(self.n_calls.next_power_of_two().ilog2(), LOG_N_LANES);
        let mut logup_gen = LogupTraceGenerator::new(log_size);

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .verifyinstruction[0];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                verifyinstruction_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .memoryaddresstoid[0];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                memoryaddresstoid_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .memoryidtobig[0];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                memoryidtobig_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .memoryaddresstoid[1];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                memoryaddresstoid_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .memoryidtobig[1];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                memoryidtobig_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .memoryaddresstoid[2];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                memoryaddresstoid_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .memoryidtobig[2];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                memoryidtobig_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[0];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[1];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[2];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[3];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[4];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[5];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[6];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[7];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[8];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[9];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[10];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[11];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[12];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[13];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[14];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[15];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[16];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[17];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[18];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[19];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[20];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[21];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[22];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[23];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[24];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[25];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[26];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_9_9[27];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[0];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[1];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[2];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[3];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[4];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[5];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[6];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[7];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[8];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[9];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[10];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[11];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[12];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[13];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[14];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[15];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[16];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[17];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[18];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[19];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[20];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[21];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[22];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[23];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[24];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[25];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[26];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .rangecheck_19[27];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                rangecheck_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .opcodes[0];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                opcodes_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data
            .opcodes[1];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom =
                opcodes_lookup_elements.combine(lookup_values);
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
