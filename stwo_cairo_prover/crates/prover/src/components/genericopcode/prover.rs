#![cfg_attr(rustfmt, rustfmt_skip)]
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
use stwo_prover::core::backend::simd::conversion::{Pack, Unpack};
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::QM31;
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::utils::bit_reverse_coset_to_circle_domain_order;
use stwo_prover::core::vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher};

use super::component::{Claim, InteractionClaim, RelationElements};
use crate::components::memory::{addr_to_id, id_to_f252};
use crate::relations;
use crate::components::{pack_values, verifyinstruction};
use crate::input::instructions::VmState;
use crate::components::range_check_vector::range_check_9_9;
use crate::components::range_check_vector::range_check_19;
use crate::relations::AddrToId;

pub type InputType = CasmState;
pub type PackedInputType = PackedCasmState;
const N_TRACE_COLUMNS: usize = 229;

#[derive(Default)]
pub struct ClaimGenerator {
    pub inputs: Vec<InputType>,
}
impl ClaimGenerator {
    pub fn new(cpu_inputs: Vec<VmState>) -> Self {
        let cpu_inputs = cpu_inputs
            .into_iter()
            .map(|VmState { pc, ap, fp }| CasmState {
                pc: M31(pc),
                ap: M31(ap),
                fp: M31(fp),
            })
            .collect();
        Self { inputs: cpu_inputs }
    }
    pub fn write_trace(
        mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        addr_to_id_state: &mut addr_to_id::ClaimGenerator,id_to_f252_state: &mut id_to_f252::ClaimGenerator,range_check_19_state: &mut range_check_19::ClaimGenerator,range_check_9_9_state: &mut range_check_9_9::ClaimGenerator,verifyinstruction_state: &mut verifyinstruction::ClaimGenerator,
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
            write_trace_simd(packed_inputs, addr_to_id_state,id_to_f252_state,);

        if need_padding {
            sub_components_inputs.bit_reverse_coset_to_circle_domain_order();
        }
        sub_components_inputs.addr_to_id_inputs.iter().for_each(|inputs| {
            addr_to_id_state.add_inputs(&inputs[..n_calls]);
        });sub_components_inputs.id_to_f252_inputs.iter().for_each(|inputs| {
            id_to_f252_state.add_inputs(&inputs[..n_calls]);
        });sub_components_inputs.range_check_19_inputs.iter().for_each(|inputs| {
            range_check_19_state.add_inputs(&inputs[..n_calls]);
        });sub_components_inputs.range_check_9_9_inputs.iter().for_each(|inputs| {
            range_check_9_9_state.add_inputs(&inputs[..n_calls]);
        });sub_components_inputs.verifyinstruction_inputs.iter().for_each(|inputs| {
            verifyinstruction_state.add_inputs(&inputs[..n_calls]);
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
{pub addr_to_id_inputs: [Vec<addr_to_id::InputType>; 4],pub id_to_f252_inputs: [Vec<id_to_f252::InputType>; 4],pub range_check_19_inputs: [Vec<range_check_19::InputType>; 28],pub range_check_9_9_inputs: [Vec<range_check_9_9::InputType>; 28],pub verifyinstruction_inputs: [Vec<verifyinstruction::InputType>; 1],}
impl SubComponentInputs {
    #[allow(unused_variables)]
    fn with_capacity(capacity: usize) -> Self {
        Self {addr_to_id_inputs: [Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),],id_to_f252_inputs: [Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),],range_check_19_inputs: [Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),],range_check_9_9_inputs: [Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),],verifyinstruction_inputs: [Vec::with_capacity(capacity),],}

    }

    fn bit_reverse_coset_to_circle_domain_order(&mut self) {
        self.addr_to_id_inputs
            .iter_mut()
            .for_each(|vec| bit_reverse_coset_to_circle_domain_order(vec));
        self.id_to_f252_inputs
            .iter_mut()
            .for_each(|vec| bit_reverse_coset_to_circle_domain_order(vec));
        self.range_check_19_inputs
            .iter_mut()
            .for_each(|vec| bit_reverse_coset_to_circle_domain_order(vec));
        self.range_check_9_9_inputs
            .iter_mut()
            .for_each(|vec| bit_reverse_coset_to_circle_domain_order(vec));
        self.verifyinstruction_inputs
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
    addr_to_id_state: &mut addr_to_id::ClaimGenerator,id_to_f252_state: &mut id_to_f252::ClaimGenerator,
) -> ([BaseColumn; N_TRACE_COLUMNS],
    SubComponentInputs,
    LookupData) {
    let mut trace: [_ ;N_TRACE_COLUMNS]= std::array::from_fn
        (|_| Col::<SimdBackend, M31>::zeros(inputs.len() * N_LANES));

    let mut lookup_data = LookupData::with_capacity(inputs.len());
    #[allow(unused_mut)]
    let mut sub_components_inputs = SubComponentInputs::with_capacity(inputs.len());

    let M31_0 = PackedM31::broadcast(M31::from(0));let M31_1 = PackedM31::broadcast(M31::from(1));let M31_131072 = PackedM31::broadcast(M31::from(131072));let M31_134217728 = PackedM31::broadcast(M31::from(134217728));let M31_136 = PackedM31::broadcast(M31::from(136));let M31_2 = PackedM31::broadcast(M31::from(2));let M31_256 = PackedM31::broadcast(M31::from(256));let M31_262144 = PackedM31::broadcast(M31::from(262144));let M31_32 = PackedM31::broadcast(M31::from(32));let M31_32768 = PackedM31::broadcast(M31::from(32768));let M31_4 = PackedM31::broadcast(M31::from(4));let M31_4194304 = PackedM31::broadcast(M31::from(4194304));let M31_511 = PackedM31::broadcast(M31::from(511));let M31_512 = PackedM31::broadcast(M31::from(512));let M31_64 = PackedM31::broadcast(M31::from(64));let M31_65536 = PackedM31::broadcast(M31::from(65536));let M31_8 = PackedM31::broadcast(M31::from(8));let UInt16_0 = PackedUInt16::broadcast(UInt16::from(0));let UInt16_1 = PackedUInt16::broadcast(UInt16::from(1));let UInt16_10 = PackedUInt16::broadcast(UInt16::from(10));let UInt16_11 = PackedUInt16::broadcast(UInt16::from(11));let UInt16_12 = PackedUInt16::broadcast(UInt16::from(12));let UInt16_127 = PackedUInt16::broadcast(UInt16::from(127));let UInt16_13 = PackedUInt16::broadcast(UInt16::from(13));let UInt16_14 = PackedUInt16::broadcast(UInt16::from(14));let UInt16_2 = PackedUInt16::broadcast(UInt16::from(2));let UInt16_3 = PackedUInt16::broadcast(UInt16::from(3));let UInt16_31 = PackedUInt16::broadcast(UInt16::from(31));let UInt16_4 = PackedUInt16::broadcast(UInt16::from(4));let UInt16_5 = PackedUInt16::broadcast(UInt16::from(5));let UInt16_6 = PackedUInt16::broadcast(UInt16::from(6));let UInt16_7 = PackedUInt16::broadcast(UInt16::from(7));let UInt16_8 = PackedUInt16::broadcast(UInt16::from(8));let UInt16_9 = PackedUInt16::broadcast(UInt16::from(9));let UInt32_262143 = PackedUInt32::broadcast(UInt32::from(262143));let UInt32_511 = PackedUInt32::broadcast(UInt32::from(511));let UInt32_65536 = PackedUInt32::broadcast(UInt32::from(65536));let UInt32_9 = PackedUInt32::broadcast(UInt32::from(9));

    inputs.into_iter()
        .enumerate().for_each(|(row_index, genericopcode_input)| {
        let input_tmp_138 = genericopcode_input;
let input_pc_col0 = input_tmp_138.pc;
        trace[0].data[row_index] = input_pc_col0;
let input_ap_col1 = input_tmp_138.ap;
        trace[1].data[row_index] = input_ap_col1;
let input_fp_col2 = input_tmp_138.fp;
        trace[2].data[row_index] = input_fp_col2;


        //DecodeGenericInstruction.

        


        //DecodeInstruction_337193008ebaa578.

        
sub_components_inputs
            .addr_to_id_inputs[0]
            .extend(input_pc_col0.unpack());
        let addr_to_id_value_tmp_209 = addr_to_id_state.deduce_output(
            input_pc_col0
        );
sub_components_inputs
            .id_to_f252_inputs[0]
            .extend(addr_to_id_value_tmp_209.unpack());
        let id_to_f252_value_tmp_210 = id_to_f252_state.deduce_output(
            addr_to_id_value_tmp_209
        );
let offset0_tmp_211 = ((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(0))) + (((((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(1))) & (UInt16_127))) << (UInt16_9))));
let offset0_col3 = offset0_tmp_211.as_m31();
        trace[3].data[row_index] = offset0_col3;
let offset1_tmp_212 = ((((((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(1))) >> (UInt16_7))) + (((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(2))) << (UInt16_2))))) + (((((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(3))) & (UInt16_31))) << (UInt16_11))));
let offset1_col4 = offset1_tmp_212.as_m31();
        trace[4].data[row_index] = offset1_col4;
let offset2_tmp_213 = ((((((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(3))) >> (UInt16_5))) + (((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(4))) << (UInt16_4))))) + (((((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(5))) & (UInt16_7))) << (UInt16_13))));
let offset2_col5 = offset2_tmp_213.as_m31();
        trace[5].data[row_index] = offset2_col5;
let dst_base_fp_tmp_214 = ((((((((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(6))) << (UInt16_6))))) >> (UInt16_0))) & (UInt16_1));
let dst_base_fp_col6 = dst_base_fp_tmp_214.as_m31();
        trace[6].data[row_index] = dst_base_fp_col6;
let op0_base_fp_tmp_215 = ((((((((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(6))) << (UInt16_6))))) >> (UInt16_1))) & (UInt16_1));
let op0_base_fp_col7 = op0_base_fp_tmp_215.as_m31();
        trace[7].data[row_index] = op0_base_fp_col7;
let op1_imm_tmp_216 = ((((((((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(6))) << (UInt16_6))))) >> (UInt16_2))) & (UInt16_1));
let op1_imm_col8 = op1_imm_tmp_216.as_m31();
        trace[8].data[row_index] = op1_imm_col8;
let op1_base_fp_tmp_217 = ((((((((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(6))) << (UInt16_6))))) >> (UInt16_3))) & (UInt16_1));
let op1_base_fp_col9 = op1_base_fp_tmp_217.as_m31();
        trace[9].data[row_index] = op1_base_fp_col9;
let op1_base_ap_tmp_218 = ((((((((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(6))) << (UInt16_6))))) >> (UInt16_4))) & (UInt16_1));
let op1_base_ap_col10 = op1_base_ap_tmp_218.as_m31();
        trace[10].data[row_index] = op1_base_ap_col10;
let res_add_tmp_219 = ((((((((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(6))) << (UInt16_6))))) >> (UInt16_5))) & (UInt16_1));
let res_add_col11 = res_add_tmp_219.as_m31();
        trace[11].data[row_index] = res_add_col11;
let res_mul_tmp_220 = ((((((((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(6))) << (UInt16_6))))) >> (UInt16_6))) & (UInt16_1));
let res_mul_col12 = res_mul_tmp_220.as_m31();
        trace[12].data[row_index] = res_mul_col12;
let pc_update_jump_tmp_221 = ((((((((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(6))) << (UInt16_6))))) >> (UInt16_7))) & (UInt16_1));
let pc_update_jump_col13 = pc_update_jump_tmp_221.as_m31();
        trace[13].data[row_index] = pc_update_jump_col13;
let pc_update_jump_rel_tmp_222 = ((((((((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(6))) << (UInt16_6))))) >> (UInt16_8))) & (UInt16_1));
let pc_update_jump_rel_col14 = pc_update_jump_rel_tmp_222.as_m31();
        trace[14].data[row_index] = pc_update_jump_rel_col14;
let pc_update_jnz_tmp_223 = ((((((((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(6))) << (UInt16_6))))) >> (UInt16_9))) & (UInt16_1));
let pc_update_jnz_col15 = pc_update_jnz_tmp_223.as_m31();
        trace[15].data[row_index] = pc_update_jnz_col15;
let ap_update_add_tmp_224 = ((((((((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(6))) << (UInt16_6))))) >> (UInt16_10))) & (UInt16_1));
let ap_update_add_col16 = ap_update_add_tmp_224.as_m31();
        trace[16].data[row_index] = ap_update_add_col16;
let ap_update_add_1_tmp_225 = ((((((((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(6))) << (UInt16_6))))) >> (UInt16_11))) & (UInt16_1));
let ap_update_add_1_col17 = ap_update_add_1_tmp_225.as_m31();
        trace[17].data[row_index] = ap_update_add_1_col17;
let opcode_call_tmp_226 = ((((((((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(6))) << (UInt16_6))))) >> (UInt16_12))) & (UInt16_1));
let opcode_call_col18 = opcode_call_tmp_226.as_m31();
        trace[18].data[row_index] = opcode_call_col18;
let opcode_ret_tmp_227 = ((((((((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(6))) << (UInt16_6))))) >> (UInt16_13))) & (UInt16_1));
let opcode_ret_col19 = opcode_ret_tmp_227.as_m31();
        trace[19].data[row_index] = opcode_ret_col19;
let opcode_assert_eq_tmp_228 = ((((((((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(id_to_f252_value_tmp_210.get_m31(6))) << (UInt16_6))))) >> (UInt16_14))) & (UInt16_1));
let opcode_assert_eq_col20 = opcode_assert_eq_tmp_228.as_m31();
        trace[20].data[row_index] = opcode_assert_eq_col20;
sub_components_inputs
            .verifyinstruction_inputs[0]
            .extend((input_pc_col0, [offset0_col3, offset1_col4, offset2_col5], [dst_base_fp_col6, op0_base_fp_col7, op1_imm_col8, op1_base_fp_col9, op1_base_ap_col10, res_add_col11, res_mul_col12, pc_update_jump_col13, pc_update_jump_rel_col14, pc_update_jnz_col15, ap_update_add_col16, ap_update_add_1_col17, opcode_call_col18, opcode_ret_col19, opcode_assert_eq_col20]).unpack());
        
lookup_data.verifyinstruction[0].push([input_pc_col0, offset0_col3, offset1_col4, offset2_col5, dst_base_fp_col6, op0_base_fp_col7, op1_imm_col8, op1_base_fp_col9, op1_base_ap_col10, res_add_col11, res_mul_col12, pc_update_jump_col13, pc_update_jump_rel_col14, pc_update_jnz_col15, ap_update_add_col16, ap_update_add_1_col17, opcode_call_col18, opcode_ret_col19, opcode_assert_eq_col20]);


        
let op1_base_op0_tmp_230 = ((((((M31_1) - (op1_imm_col8))) - (op1_base_fp_col9))) - (op1_base_ap_col10));
let res_op1_tmp_231 = ((((((M31_1) - (res_add_col11))) - (res_mul_col12))) - (pc_update_jnz_col15));
let pc_update_regular_tmp_232 = ((((((M31_1) - (pc_update_jump_col13))) - (pc_update_jump_rel_col14))) - (pc_update_jnz_col15));
let ap_update_regular_tmp_233 = ((((((M31_1) - (ap_update_add_col16))) - (ap_update_add_1_col17))) - (opcode_call_col18));
let fp_update_regular_tmp_234 = ((((M31_1) - (opcode_call_col18))) - (opcode_ret_col19));


        


        //EvalOperands.

        


        //ReadPositive_num_bits_252.

        
sub_components_inputs
            .addr_to_id_inputs[1]
            .extend(((((((dst_base_fp_col6) * (input_fp_col2))) + (((((M31_1) - (dst_base_fp_col6))) * (input_ap_col1))))) + (((offset0_col3) - (M31_32768)))).unpack());
        let addr_to_id_value_tmp_723 = addr_to_id_state.deduce_output(
            ((((((dst_base_fp_col6) * (input_fp_col2))) + (((((M31_1) - (dst_base_fp_col6))) * (input_ap_col1))))) + (((offset0_col3) - (M31_32768))))
        );
let dst_id_col21 = addr_to_id_value_tmp_723;
        trace[21].data[row_index] = dst_id_col21;
lookup_data.addr_to_id[0].push([((((((dst_base_fp_col6) * (input_fp_col2))) + (((((M31_1) - (dst_base_fp_col6))) * (input_ap_col1))))) + (((offset0_col3) - (M31_32768)))), dst_id_col21]);
sub_components_inputs
            .id_to_f252_inputs[1]
            .extend(dst_id_col21.unpack());
        let id_to_f252_value_tmp_724 = id_to_f252_state.deduce_output(
            dst_id_col21
        );
let dst_limb_0_col22 = id_to_f252_value_tmp_724.get_m31(0);
        trace[22].data[row_index] = dst_limb_0_col22;
let dst_limb_1_col23 = id_to_f252_value_tmp_724.get_m31(1);
        trace[23].data[row_index] = dst_limb_1_col23;
let dst_limb_2_col24 = id_to_f252_value_tmp_724.get_m31(2);
        trace[24].data[row_index] = dst_limb_2_col24;
let dst_limb_3_col25 = id_to_f252_value_tmp_724.get_m31(3);
        trace[25].data[row_index] = dst_limb_3_col25;
let dst_limb_4_col26 = id_to_f252_value_tmp_724.get_m31(4);
        trace[26].data[row_index] = dst_limb_4_col26;
let dst_limb_5_col27 = id_to_f252_value_tmp_724.get_m31(5);
        trace[27].data[row_index] = dst_limb_5_col27;
let dst_limb_6_col28 = id_to_f252_value_tmp_724.get_m31(6);
        trace[28].data[row_index] = dst_limb_6_col28;
let dst_limb_7_col29 = id_to_f252_value_tmp_724.get_m31(7);
        trace[29].data[row_index] = dst_limb_7_col29;
let dst_limb_8_col30 = id_to_f252_value_tmp_724.get_m31(8);
        trace[30].data[row_index] = dst_limb_8_col30;
let dst_limb_9_col31 = id_to_f252_value_tmp_724.get_m31(9);
        trace[31].data[row_index] = dst_limb_9_col31;
let dst_limb_10_col32 = id_to_f252_value_tmp_724.get_m31(10);
        trace[32].data[row_index] = dst_limb_10_col32;
let dst_limb_11_col33 = id_to_f252_value_tmp_724.get_m31(11);
        trace[33].data[row_index] = dst_limb_11_col33;
let dst_limb_12_col34 = id_to_f252_value_tmp_724.get_m31(12);
        trace[34].data[row_index] = dst_limb_12_col34;
let dst_limb_13_col35 = id_to_f252_value_tmp_724.get_m31(13);
        trace[35].data[row_index] = dst_limb_13_col35;
let dst_limb_14_col36 = id_to_f252_value_tmp_724.get_m31(14);
        trace[36].data[row_index] = dst_limb_14_col36;
let dst_limb_15_col37 = id_to_f252_value_tmp_724.get_m31(15);
        trace[37].data[row_index] = dst_limb_15_col37;
let dst_limb_16_col38 = id_to_f252_value_tmp_724.get_m31(16);
        trace[38].data[row_index] = dst_limb_16_col38;
let dst_limb_17_col39 = id_to_f252_value_tmp_724.get_m31(17);
        trace[39].data[row_index] = dst_limb_17_col39;
let dst_limb_18_col40 = id_to_f252_value_tmp_724.get_m31(18);
        trace[40].data[row_index] = dst_limb_18_col40;
let dst_limb_19_col41 = id_to_f252_value_tmp_724.get_m31(19);
        trace[41].data[row_index] = dst_limb_19_col41;
let dst_limb_20_col42 = id_to_f252_value_tmp_724.get_m31(20);
        trace[42].data[row_index] = dst_limb_20_col42;
let dst_limb_21_col43 = id_to_f252_value_tmp_724.get_m31(21);
        trace[43].data[row_index] = dst_limb_21_col43;
let dst_limb_22_col44 = id_to_f252_value_tmp_724.get_m31(22);
        trace[44].data[row_index] = dst_limb_22_col44;
let dst_limb_23_col45 = id_to_f252_value_tmp_724.get_m31(23);
        trace[45].data[row_index] = dst_limb_23_col45;
let dst_limb_24_col46 = id_to_f252_value_tmp_724.get_m31(24);
        trace[46].data[row_index] = dst_limb_24_col46;
let dst_limb_25_col47 = id_to_f252_value_tmp_724.get_m31(25);
        trace[47].data[row_index] = dst_limb_25_col47;
let dst_limb_26_col48 = id_to_f252_value_tmp_724.get_m31(26);
        trace[48].data[row_index] = dst_limb_26_col48;
let dst_limb_27_col49 = id_to_f252_value_tmp_724.get_m31(27);
        trace[49].data[row_index] = dst_limb_27_col49;
lookup_data.id_to_f252[0].push([dst_id_col21, dst_limb_0_col22, dst_limb_1_col23, dst_limb_2_col24, dst_limb_3_col25, dst_limb_4_col26, dst_limb_5_col27, dst_limb_6_col28, dst_limb_7_col29, dst_limb_8_col30, dst_limb_9_col31, dst_limb_10_col32, dst_limb_11_col33, dst_limb_12_col34, dst_limb_13_col35, dst_limb_14_col36, dst_limb_15_col37, dst_limb_16_col38, dst_limb_17_col39, dst_limb_18_col40, dst_limb_19_col41, dst_limb_20_col42, dst_limb_21_col43, dst_limb_22_col44, dst_limb_23_col45, dst_limb_24_col46, dst_limb_25_col47, dst_limb_26_col48, dst_limb_27_col49]);


        


        //ReadPositive_num_bits_252.

        
sub_components_inputs
            .addr_to_id_inputs[2]
            .extend(((((((op0_base_fp_col7) * (input_fp_col2))) + (((((M31_1) - (op0_base_fp_col7))) * (input_ap_col1))))) + (((offset1_col4) - (M31_32768)))).unpack());
        let addr_to_id_value_tmp_725 = addr_to_id_state.deduce_output(
            ((((((op0_base_fp_col7) * (input_fp_col2))) + (((((M31_1) - (op0_base_fp_col7))) * (input_ap_col1))))) + (((offset1_col4) - (M31_32768))))
        );
let op0_id_col50 = addr_to_id_value_tmp_725;
        trace[50].data[row_index] = op0_id_col50;
lookup_data.addr_to_id[1].push([((((((op0_base_fp_col7) * (input_fp_col2))) + (((((M31_1) - (op0_base_fp_col7))) * (input_ap_col1))))) + (((offset1_col4) - (M31_32768)))), op0_id_col50]);
sub_components_inputs
            .id_to_f252_inputs[2]
            .extend(op0_id_col50.unpack());
        let id_to_f252_value_tmp_726 = id_to_f252_state.deduce_output(
            op0_id_col50
        );
let op0_limb_0_col51 = id_to_f252_value_tmp_726.get_m31(0);
        trace[51].data[row_index] = op0_limb_0_col51;
let op0_limb_1_col52 = id_to_f252_value_tmp_726.get_m31(1);
        trace[52].data[row_index] = op0_limb_1_col52;
let op0_limb_2_col53 = id_to_f252_value_tmp_726.get_m31(2);
        trace[53].data[row_index] = op0_limb_2_col53;
let op0_limb_3_col54 = id_to_f252_value_tmp_726.get_m31(3);
        trace[54].data[row_index] = op0_limb_3_col54;
let op0_limb_4_col55 = id_to_f252_value_tmp_726.get_m31(4);
        trace[55].data[row_index] = op0_limb_4_col55;
let op0_limb_5_col56 = id_to_f252_value_tmp_726.get_m31(5);
        trace[56].data[row_index] = op0_limb_5_col56;
let op0_limb_6_col57 = id_to_f252_value_tmp_726.get_m31(6);
        trace[57].data[row_index] = op0_limb_6_col57;
let op0_limb_7_col58 = id_to_f252_value_tmp_726.get_m31(7);
        trace[58].data[row_index] = op0_limb_7_col58;
let op0_limb_8_col59 = id_to_f252_value_tmp_726.get_m31(8);
        trace[59].data[row_index] = op0_limb_8_col59;
let op0_limb_9_col60 = id_to_f252_value_tmp_726.get_m31(9);
        trace[60].data[row_index] = op0_limb_9_col60;
let op0_limb_10_col61 = id_to_f252_value_tmp_726.get_m31(10);
        trace[61].data[row_index] = op0_limb_10_col61;
let op0_limb_11_col62 = id_to_f252_value_tmp_726.get_m31(11);
        trace[62].data[row_index] = op0_limb_11_col62;
let op0_limb_12_col63 = id_to_f252_value_tmp_726.get_m31(12);
        trace[63].data[row_index] = op0_limb_12_col63;
let op0_limb_13_col64 = id_to_f252_value_tmp_726.get_m31(13);
        trace[64].data[row_index] = op0_limb_13_col64;
let op0_limb_14_col65 = id_to_f252_value_tmp_726.get_m31(14);
        trace[65].data[row_index] = op0_limb_14_col65;
let op0_limb_15_col66 = id_to_f252_value_tmp_726.get_m31(15);
        trace[66].data[row_index] = op0_limb_15_col66;
let op0_limb_16_col67 = id_to_f252_value_tmp_726.get_m31(16);
        trace[67].data[row_index] = op0_limb_16_col67;
let op0_limb_17_col68 = id_to_f252_value_tmp_726.get_m31(17);
        trace[68].data[row_index] = op0_limb_17_col68;
let op0_limb_18_col69 = id_to_f252_value_tmp_726.get_m31(18);
        trace[69].data[row_index] = op0_limb_18_col69;
let op0_limb_19_col70 = id_to_f252_value_tmp_726.get_m31(19);
        trace[70].data[row_index] = op0_limb_19_col70;
let op0_limb_20_col71 = id_to_f252_value_tmp_726.get_m31(20);
        trace[71].data[row_index] = op0_limb_20_col71;
let op0_limb_21_col72 = id_to_f252_value_tmp_726.get_m31(21);
        trace[72].data[row_index] = op0_limb_21_col72;
let op0_limb_22_col73 = id_to_f252_value_tmp_726.get_m31(22);
        trace[73].data[row_index] = op0_limb_22_col73;
let op0_limb_23_col74 = id_to_f252_value_tmp_726.get_m31(23);
        trace[74].data[row_index] = op0_limb_23_col74;
let op0_limb_24_col75 = id_to_f252_value_tmp_726.get_m31(24);
        trace[75].data[row_index] = op0_limb_24_col75;
let op0_limb_25_col76 = id_to_f252_value_tmp_726.get_m31(25);
        trace[76].data[row_index] = op0_limb_25_col76;
let op0_limb_26_col77 = id_to_f252_value_tmp_726.get_m31(26);
        trace[77].data[row_index] = op0_limb_26_col77;
let op0_limb_27_col78 = id_to_f252_value_tmp_726.get_m31(27);
        trace[78].data[row_index] = op0_limb_27_col78;
lookup_data.id_to_f252[1].push([op0_id_col50, op0_limb_0_col51, op0_limb_1_col52, op0_limb_2_col53, op0_limb_3_col54, op0_limb_4_col55, op0_limb_5_col56, op0_limb_6_col57, op0_limb_7_col58, op0_limb_8_col59, op0_limb_9_col60, op0_limb_10_col61, op0_limb_11_col62, op0_limb_12_col63, op0_limb_13_col64, op0_limb_14_col65, op0_limb_15_col66, op0_limb_16_col67, op0_limb_17_col68, op0_limb_18_col69, op0_limb_19_col70, op0_limb_20_col71, op0_limb_21_col72, op0_limb_22_col73, op0_limb_23_col74, op0_limb_24_col75, op0_limb_25_col76, op0_limb_26_col77, op0_limb_27_col78]);


        


        //CondFelt252AsAddr.

        


        


        //ReadPositive_num_bits_252.

        
sub_components_inputs
            .addr_to_id_inputs[3]
            .extend(((((((((((op1_base_fp_col9) * (input_fp_col2))) + (((op1_base_ap_col10) * (input_ap_col1))))) + (((op1_imm_col8) * (input_pc_col0))))) + (((op1_base_op0_tmp_230) * (((((op0_limb_0_col51) + (((op0_limb_1_col52) * (M31_512))))) + (((op0_limb_2_col53) * (M31_262144))))))))) + (((offset2_col5) - (M31_32768)))).unpack());
        let addr_to_id_value_tmp_727 = addr_to_id_state.deduce_output(
            ((((((((((op1_base_fp_col9) * (input_fp_col2))) + (((op1_base_ap_col10) * (input_ap_col1))))) + (((op1_imm_col8) * (input_pc_col0))))) + (((op1_base_op0_tmp_230) * (((((op0_limb_0_col51) + (((op0_limb_1_col52) * (M31_512))))) + (((op0_limb_2_col53) * (M31_262144))))))))) + (((offset2_col5) - (M31_32768))))
        );
let op1_id_col79 = addr_to_id_value_tmp_727;
        trace[79].data[row_index] = op1_id_col79;
lookup_data.addr_to_id[2].push([((((((((((op1_base_fp_col9) * (input_fp_col2))) + (((op1_base_ap_col10) * (input_ap_col1))))) + (((op1_imm_col8) * (input_pc_col0))))) + (((op1_base_op0_tmp_230) * (((((op0_limb_0_col51) + (((op0_limb_1_col52) * (M31_512))))) + (((op0_limb_2_col53) * (M31_262144))))))))) + (((offset2_col5) - (M31_32768)))), op1_id_col79]);
sub_components_inputs
            .id_to_f252_inputs[3]
            .extend(op1_id_col79.unpack());
        let id_to_f252_value_tmp_728 = id_to_f252_state.deduce_output(
            op1_id_col79
        );
let op1_limb_0_col80 = id_to_f252_value_tmp_728.get_m31(0);
        trace[80].data[row_index] = op1_limb_0_col80;
let op1_limb_1_col81 = id_to_f252_value_tmp_728.get_m31(1);
        trace[81].data[row_index] = op1_limb_1_col81;
let op1_limb_2_col82 = id_to_f252_value_tmp_728.get_m31(2);
        trace[82].data[row_index] = op1_limb_2_col82;
let op1_limb_3_col83 = id_to_f252_value_tmp_728.get_m31(3);
        trace[83].data[row_index] = op1_limb_3_col83;
let op1_limb_4_col84 = id_to_f252_value_tmp_728.get_m31(4);
        trace[84].data[row_index] = op1_limb_4_col84;
let op1_limb_5_col85 = id_to_f252_value_tmp_728.get_m31(5);
        trace[85].data[row_index] = op1_limb_5_col85;
let op1_limb_6_col86 = id_to_f252_value_tmp_728.get_m31(6);
        trace[86].data[row_index] = op1_limb_6_col86;
let op1_limb_7_col87 = id_to_f252_value_tmp_728.get_m31(7);
        trace[87].data[row_index] = op1_limb_7_col87;
let op1_limb_8_col88 = id_to_f252_value_tmp_728.get_m31(8);
        trace[88].data[row_index] = op1_limb_8_col88;
let op1_limb_9_col89 = id_to_f252_value_tmp_728.get_m31(9);
        trace[89].data[row_index] = op1_limb_9_col89;
let op1_limb_10_col90 = id_to_f252_value_tmp_728.get_m31(10);
        trace[90].data[row_index] = op1_limb_10_col90;
let op1_limb_11_col91 = id_to_f252_value_tmp_728.get_m31(11);
        trace[91].data[row_index] = op1_limb_11_col91;
let op1_limb_12_col92 = id_to_f252_value_tmp_728.get_m31(12);
        trace[92].data[row_index] = op1_limb_12_col92;
let op1_limb_13_col93 = id_to_f252_value_tmp_728.get_m31(13);
        trace[93].data[row_index] = op1_limb_13_col93;
let op1_limb_14_col94 = id_to_f252_value_tmp_728.get_m31(14);
        trace[94].data[row_index] = op1_limb_14_col94;
let op1_limb_15_col95 = id_to_f252_value_tmp_728.get_m31(15);
        trace[95].data[row_index] = op1_limb_15_col95;
let op1_limb_16_col96 = id_to_f252_value_tmp_728.get_m31(16);
        trace[96].data[row_index] = op1_limb_16_col96;
let op1_limb_17_col97 = id_to_f252_value_tmp_728.get_m31(17);
        trace[97].data[row_index] = op1_limb_17_col97;
let op1_limb_18_col98 = id_to_f252_value_tmp_728.get_m31(18);
        trace[98].data[row_index] = op1_limb_18_col98;
let op1_limb_19_col99 = id_to_f252_value_tmp_728.get_m31(19);
        trace[99].data[row_index] = op1_limb_19_col99;
let op1_limb_20_col100 = id_to_f252_value_tmp_728.get_m31(20);
        trace[100].data[row_index] = op1_limb_20_col100;
let op1_limb_21_col101 = id_to_f252_value_tmp_728.get_m31(21);
        trace[101].data[row_index] = op1_limb_21_col101;
let op1_limb_22_col102 = id_to_f252_value_tmp_728.get_m31(22);
        trace[102].data[row_index] = op1_limb_22_col102;
let op1_limb_23_col103 = id_to_f252_value_tmp_728.get_m31(23);
        trace[103].data[row_index] = op1_limb_23_col103;
let op1_limb_24_col104 = id_to_f252_value_tmp_728.get_m31(24);
        trace[104].data[row_index] = op1_limb_24_col104;
let op1_limb_25_col105 = id_to_f252_value_tmp_728.get_m31(25);
        trace[105].data[row_index] = op1_limb_25_col105;
let op1_limb_26_col106 = id_to_f252_value_tmp_728.get_m31(26);
        trace[106].data[row_index] = op1_limb_26_col106;
let op1_limb_27_col107 = id_to_f252_value_tmp_728.get_m31(27);
        trace[107].data[row_index] = op1_limb_27_col107;
lookup_data.id_to_f252[2].push([op1_id_col79, op1_limb_0_col80, op1_limb_1_col81, op1_limb_2_col82, op1_limb_3_col83, op1_limb_4_col84, op1_limb_5_col85, op1_limb_6_col86, op1_limb_7_col87, op1_limb_8_col88, op1_limb_9_col89, op1_limb_10_col90, op1_limb_11_col91, op1_limb_12_col92, op1_limb_13_col93, op1_limb_14_col94, op1_limb_15_col95, op1_limb_16_col96, op1_limb_17_col97, op1_limb_18_col98, op1_limb_19_col99, op1_limb_20_col100, op1_limb_21_col101, op1_limb_22_col102, op1_limb_23_col103, op1_limb_24_col104, op1_limb_25_col105, op1_limb_26_col106, op1_limb_27_col107]);


        


        //Add252.

        
let add_res_tmp_729 = ((PackedFelt252::from_limbs([op0_limb_0_col51, op0_limb_1_col52, op0_limb_2_col53, op0_limb_3_col54, op0_limb_4_col55, op0_limb_5_col56, op0_limb_6_col57, op0_limb_7_col58, op0_limb_8_col59, op0_limb_9_col60, op0_limb_10_col61, op0_limb_11_col62, op0_limb_12_col63, op0_limb_13_col64, op0_limb_14_col65, op0_limb_15_col66, op0_limb_16_col67, op0_limb_17_col68, op0_limb_18_col69, op0_limb_19_col70, op0_limb_20_col71, op0_limb_21_col72, op0_limb_22_col73, op0_limb_23_col74, op0_limb_24_col75, op0_limb_25_col76, op0_limb_26_col77, op0_limb_27_col78])) + (PackedFelt252::from_limbs([op1_limb_0_col80, op1_limb_1_col81, op1_limb_2_col82, op1_limb_3_col83, op1_limb_4_col84, op1_limb_5_col85, op1_limb_6_col86, op1_limb_7_col87, op1_limb_8_col88, op1_limb_9_col89, op1_limb_10_col90, op1_limb_11_col91, op1_limb_12_col92, op1_limb_13_col93, op1_limb_14_col94, op1_limb_15_col95, op1_limb_16_col96, op1_limb_17_col97, op1_limb_18_col98, op1_limb_19_col99, op1_limb_20_col100, op1_limb_21_col101, op1_limb_22_col102, op1_limb_23_col103, op1_limb_24_col104, op1_limb_25_col105, op1_limb_26_col106, op1_limb_27_col107])));
let add_res_limb_0_col108 = add_res_tmp_729.get_m31(0);
        trace[108].data[row_index] = add_res_limb_0_col108;
let add_res_limb_1_col109 = add_res_tmp_729.get_m31(1);
        trace[109].data[row_index] = add_res_limb_1_col109;
let add_res_limb_2_col110 = add_res_tmp_729.get_m31(2);
        trace[110].data[row_index] = add_res_limb_2_col110;
let add_res_limb_3_col111 = add_res_tmp_729.get_m31(3);
        trace[111].data[row_index] = add_res_limb_3_col111;
let add_res_limb_4_col112 = add_res_tmp_729.get_m31(4);
        trace[112].data[row_index] = add_res_limb_4_col112;
let add_res_limb_5_col113 = add_res_tmp_729.get_m31(5);
        trace[113].data[row_index] = add_res_limb_5_col113;
let add_res_limb_6_col114 = add_res_tmp_729.get_m31(6);
        trace[114].data[row_index] = add_res_limb_6_col114;
let add_res_limb_7_col115 = add_res_tmp_729.get_m31(7);
        trace[115].data[row_index] = add_res_limb_7_col115;
let add_res_limb_8_col116 = add_res_tmp_729.get_m31(8);
        trace[116].data[row_index] = add_res_limb_8_col116;
let add_res_limb_9_col117 = add_res_tmp_729.get_m31(9);
        trace[117].data[row_index] = add_res_limb_9_col117;
let add_res_limb_10_col118 = add_res_tmp_729.get_m31(10);
        trace[118].data[row_index] = add_res_limb_10_col118;
let add_res_limb_11_col119 = add_res_tmp_729.get_m31(11);
        trace[119].data[row_index] = add_res_limb_11_col119;
let add_res_limb_12_col120 = add_res_tmp_729.get_m31(12);
        trace[120].data[row_index] = add_res_limb_12_col120;
let add_res_limb_13_col121 = add_res_tmp_729.get_m31(13);
        trace[121].data[row_index] = add_res_limb_13_col121;
let add_res_limb_14_col122 = add_res_tmp_729.get_m31(14);
        trace[122].data[row_index] = add_res_limb_14_col122;
let add_res_limb_15_col123 = add_res_tmp_729.get_m31(15);
        trace[123].data[row_index] = add_res_limb_15_col123;
let add_res_limb_16_col124 = add_res_tmp_729.get_m31(16);
        trace[124].data[row_index] = add_res_limb_16_col124;
let add_res_limb_17_col125 = add_res_tmp_729.get_m31(17);
        trace[125].data[row_index] = add_res_limb_17_col125;
let add_res_limb_18_col126 = add_res_tmp_729.get_m31(18);
        trace[126].data[row_index] = add_res_limb_18_col126;
let add_res_limb_19_col127 = add_res_tmp_729.get_m31(19);
        trace[127].data[row_index] = add_res_limb_19_col127;
let add_res_limb_20_col128 = add_res_tmp_729.get_m31(20);
        trace[128].data[row_index] = add_res_limb_20_col128;
let add_res_limb_21_col129 = add_res_tmp_729.get_m31(21);
        trace[129].data[row_index] = add_res_limb_21_col129;
let add_res_limb_22_col130 = add_res_tmp_729.get_m31(22);
        trace[130].data[row_index] = add_res_limb_22_col130;
let add_res_limb_23_col131 = add_res_tmp_729.get_m31(23);
        trace[131].data[row_index] = add_res_limb_23_col131;
let add_res_limb_24_col132 = add_res_tmp_729.get_m31(24);
        trace[132].data[row_index] = add_res_limb_24_col132;
let add_res_limb_25_col133 = add_res_tmp_729.get_m31(25);
        trace[133].data[row_index] = add_res_limb_25_col133;
let add_res_limb_26_col134 = add_res_tmp_729.get_m31(26);
        trace[134].data[row_index] = add_res_limb_26_col134;
let add_res_limb_27_col135 = add_res_tmp_729.get_m31(27);
        trace[135].data[row_index] = add_res_limb_27_col135;


        //RangeCheckBigValue.

        
sub_components_inputs
            .range_check_9_9_inputs[0]
            .extend([add_res_limb_0_col108, add_res_limb_1_col109].unpack());
        
lookup_data.range_check_9_9[0].push([add_res_limb_0_col108, add_res_limb_1_col109]);
sub_components_inputs
            .range_check_9_9_inputs[1]
            .extend([add_res_limb_2_col110, add_res_limb_3_col111].unpack());
        
lookup_data.range_check_9_9[1].push([add_res_limb_2_col110, add_res_limb_3_col111]);
sub_components_inputs
            .range_check_9_9_inputs[2]
            .extend([add_res_limb_4_col112, add_res_limb_5_col113].unpack());
        
lookup_data.range_check_9_9[2].push([add_res_limb_4_col112, add_res_limb_5_col113]);
sub_components_inputs
            .range_check_9_9_inputs[3]
            .extend([add_res_limb_6_col114, add_res_limb_7_col115].unpack());
        
lookup_data.range_check_9_9[3].push([add_res_limb_6_col114, add_res_limb_7_col115]);
sub_components_inputs
            .range_check_9_9_inputs[4]
            .extend([add_res_limb_8_col116, add_res_limb_9_col117].unpack());
        
lookup_data.range_check_9_9[4].push([add_res_limb_8_col116, add_res_limb_9_col117]);
sub_components_inputs
            .range_check_9_9_inputs[5]
            .extend([add_res_limb_10_col118, add_res_limb_11_col119].unpack());
        
lookup_data.range_check_9_9[5].push([add_res_limb_10_col118, add_res_limb_11_col119]);
sub_components_inputs
            .range_check_9_9_inputs[6]
            .extend([add_res_limb_12_col120, add_res_limb_13_col121].unpack());
        
lookup_data.range_check_9_9[6].push([add_res_limb_12_col120, add_res_limb_13_col121]);
sub_components_inputs
            .range_check_9_9_inputs[7]
            .extend([add_res_limb_14_col122, add_res_limb_15_col123].unpack());
        
lookup_data.range_check_9_9[7].push([add_res_limb_14_col122, add_res_limb_15_col123]);
sub_components_inputs
            .range_check_9_9_inputs[8]
            .extend([add_res_limb_16_col124, add_res_limb_17_col125].unpack());
        
lookup_data.range_check_9_9[8].push([add_res_limb_16_col124, add_res_limb_17_col125]);
sub_components_inputs
            .range_check_9_9_inputs[9]
            .extend([add_res_limb_18_col126, add_res_limb_19_col127].unpack());
        
lookup_data.range_check_9_9[9].push([add_res_limb_18_col126, add_res_limb_19_col127]);
sub_components_inputs
            .range_check_9_9_inputs[10]
            .extend([add_res_limb_20_col128, add_res_limb_21_col129].unpack());
        
lookup_data.range_check_9_9[10].push([add_res_limb_20_col128, add_res_limb_21_col129]);
sub_components_inputs
            .range_check_9_9_inputs[11]
            .extend([add_res_limb_22_col130, add_res_limb_23_col131].unpack());
        
lookup_data.range_check_9_9[11].push([add_res_limb_22_col130, add_res_limb_23_col131]);
sub_components_inputs
            .range_check_9_9_inputs[12]
            .extend([add_res_limb_24_col132, add_res_limb_25_col133].unpack());
        
lookup_data.range_check_9_9[12].push([add_res_limb_24_col132, add_res_limb_25_col133]);
sub_components_inputs
            .range_check_9_9_inputs[13]
            .extend([add_res_limb_26_col134, add_res_limb_27_col135].unpack());
        
lookup_data.range_check_9_9[13].push([add_res_limb_26_col134, add_res_limb_27_col135]);


        


        //VerifyAdd252.

        
let sub_p_bit_tmp_744 = ((UInt16_1) & (((((PackedUInt16::from_m31(op0_limb_0_col51)) ^ (PackedUInt16::from_m31(op1_limb_0_col80)))) ^ (PackedUInt16::from_m31(add_res_limb_0_col108)))));
let sub_p_bit_col136 = sub_p_bit_tmp_744.as_m31();
        trace[136].data[row_index] = sub_p_bit_col136;


        


        


        //Mul252.

        
let mul_res_tmp_772 = ((PackedFelt252::from_limbs([op0_limb_0_col51, op0_limb_1_col52, op0_limb_2_col53, op0_limb_3_col54, op0_limb_4_col55, op0_limb_5_col56, op0_limb_6_col57, op0_limb_7_col58, op0_limb_8_col59, op0_limb_9_col60, op0_limb_10_col61, op0_limb_11_col62, op0_limb_12_col63, op0_limb_13_col64, op0_limb_14_col65, op0_limb_15_col66, op0_limb_16_col67, op0_limb_17_col68, op0_limb_18_col69, op0_limb_19_col70, op0_limb_20_col71, op0_limb_21_col72, op0_limb_22_col73, op0_limb_23_col74, op0_limb_24_col75, op0_limb_25_col76, op0_limb_26_col77, op0_limb_27_col78])) * (PackedFelt252::from_limbs([op1_limb_0_col80, op1_limb_1_col81, op1_limb_2_col82, op1_limb_3_col83, op1_limb_4_col84, op1_limb_5_col85, op1_limb_6_col86, op1_limb_7_col87, op1_limb_8_col88, op1_limb_9_col89, op1_limb_10_col90, op1_limb_11_col91, op1_limb_12_col92, op1_limb_13_col93, op1_limb_14_col94, op1_limb_15_col95, op1_limb_16_col96, op1_limb_17_col97, op1_limb_18_col98, op1_limb_19_col99, op1_limb_20_col100, op1_limb_21_col101, op1_limb_22_col102, op1_limb_23_col103, op1_limb_24_col104, op1_limb_25_col105, op1_limb_26_col106, op1_limb_27_col107])));
let mul_res_limb_0_col137 = mul_res_tmp_772.get_m31(0);
        trace[137].data[row_index] = mul_res_limb_0_col137;
let mul_res_limb_1_col138 = mul_res_tmp_772.get_m31(1);
        trace[138].data[row_index] = mul_res_limb_1_col138;
let mul_res_limb_2_col139 = mul_res_tmp_772.get_m31(2);
        trace[139].data[row_index] = mul_res_limb_2_col139;
let mul_res_limb_3_col140 = mul_res_tmp_772.get_m31(3);
        trace[140].data[row_index] = mul_res_limb_3_col140;
let mul_res_limb_4_col141 = mul_res_tmp_772.get_m31(4);
        trace[141].data[row_index] = mul_res_limb_4_col141;
let mul_res_limb_5_col142 = mul_res_tmp_772.get_m31(5);
        trace[142].data[row_index] = mul_res_limb_5_col142;
let mul_res_limb_6_col143 = mul_res_tmp_772.get_m31(6);
        trace[143].data[row_index] = mul_res_limb_6_col143;
let mul_res_limb_7_col144 = mul_res_tmp_772.get_m31(7);
        trace[144].data[row_index] = mul_res_limb_7_col144;
let mul_res_limb_8_col145 = mul_res_tmp_772.get_m31(8);
        trace[145].data[row_index] = mul_res_limb_8_col145;
let mul_res_limb_9_col146 = mul_res_tmp_772.get_m31(9);
        trace[146].data[row_index] = mul_res_limb_9_col146;
let mul_res_limb_10_col147 = mul_res_tmp_772.get_m31(10);
        trace[147].data[row_index] = mul_res_limb_10_col147;
let mul_res_limb_11_col148 = mul_res_tmp_772.get_m31(11);
        trace[148].data[row_index] = mul_res_limb_11_col148;
let mul_res_limb_12_col149 = mul_res_tmp_772.get_m31(12);
        trace[149].data[row_index] = mul_res_limb_12_col149;
let mul_res_limb_13_col150 = mul_res_tmp_772.get_m31(13);
        trace[150].data[row_index] = mul_res_limb_13_col150;
let mul_res_limb_14_col151 = mul_res_tmp_772.get_m31(14);
        trace[151].data[row_index] = mul_res_limb_14_col151;
let mul_res_limb_15_col152 = mul_res_tmp_772.get_m31(15);
        trace[152].data[row_index] = mul_res_limb_15_col152;
let mul_res_limb_16_col153 = mul_res_tmp_772.get_m31(16);
        trace[153].data[row_index] = mul_res_limb_16_col153;
let mul_res_limb_17_col154 = mul_res_tmp_772.get_m31(17);
        trace[154].data[row_index] = mul_res_limb_17_col154;
let mul_res_limb_18_col155 = mul_res_tmp_772.get_m31(18);
        trace[155].data[row_index] = mul_res_limb_18_col155;
let mul_res_limb_19_col156 = mul_res_tmp_772.get_m31(19);
        trace[156].data[row_index] = mul_res_limb_19_col156;
let mul_res_limb_20_col157 = mul_res_tmp_772.get_m31(20);
        trace[157].data[row_index] = mul_res_limb_20_col157;
let mul_res_limb_21_col158 = mul_res_tmp_772.get_m31(21);
        trace[158].data[row_index] = mul_res_limb_21_col158;
let mul_res_limb_22_col159 = mul_res_tmp_772.get_m31(22);
        trace[159].data[row_index] = mul_res_limb_22_col159;
let mul_res_limb_23_col160 = mul_res_tmp_772.get_m31(23);
        trace[160].data[row_index] = mul_res_limb_23_col160;
let mul_res_limb_24_col161 = mul_res_tmp_772.get_m31(24);
        trace[161].data[row_index] = mul_res_limb_24_col161;
let mul_res_limb_25_col162 = mul_res_tmp_772.get_m31(25);
        trace[162].data[row_index] = mul_res_limb_25_col162;
let mul_res_limb_26_col163 = mul_res_tmp_772.get_m31(26);
        trace[163].data[row_index] = mul_res_limb_26_col163;
let mul_res_limb_27_col164 = mul_res_tmp_772.get_m31(27);
        trace[164].data[row_index] = mul_res_limb_27_col164;


        //RangeCheckBigValue.

        
sub_components_inputs
            .range_check_9_9_inputs[14]
            .extend([mul_res_limb_0_col137, mul_res_limb_1_col138].unpack());
        
lookup_data.range_check_9_9[14].push([mul_res_limb_0_col137, mul_res_limb_1_col138]);
sub_components_inputs
            .range_check_9_9_inputs[15]
            .extend([mul_res_limb_2_col139, mul_res_limb_3_col140].unpack());
        
lookup_data.range_check_9_9[15].push([mul_res_limb_2_col139, mul_res_limb_3_col140]);
sub_components_inputs
            .range_check_9_9_inputs[16]
            .extend([mul_res_limb_4_col141, mul_res_limb_5_col142].unpack());
        
lookup_data.range_check_9_9[16].push([mul_res_limb_4_col141, mul_res_limb_5_col142]);
sub_components_inputs
            .range_check_9_9_inputs[17]
            .extend([mul_res_limb_6_col143, mul_res_limb_7_col144].unpack());
        
lookup_data.range_check_9_9[17].push([mul_res_limb_6_col143, mul_res_limb_7_col144]);
sub_components_inputs
            .range_check_9_9_inputs[18]
            .extend([mul_res_limb_8_col145, mul_res_limb_9_col146].unpack());
        
lookup_data.range_check_9_9[18].push([mul_res_limb_8_col145, mul_res_limb_9_col146]);
sub_components_inputs
            .range_check_9_9_inputs[19]
            .extend([mul_res_limb_10_col147, mul_res_limb_11_col148].unpack());
        
lookup_data.range_check_9_9[19].push([mul_res_limb_10_col147, mul_res_limb_11_col148]);
sub_components_inputs
            .range_check_9_9_inputs[20]
            .extend([mul_res_limb_12_col149, mul_res_limb_13_col150].unpack());
        
lookup_data.range_check_9_9[20].push([mul_res_limb_12_col149, mul_res_limb_13_col150]);
sub_components_inputs
            .range_check_9_9_inputs[21]
            .extend([mul_res_limb_14_col151, mul_res_limb_15_col152].unpack());
        
lookup_data.range_check_9_9[21].push([mul_res_limb_14_col151, mul_res_limb_15_col152]);
sub_components_inputs
            .range_check_9_9_inputs[22]
            .extend([mul_res_limb_16_col153, mul_res_limb_17_col154].unpack());
        
lookup_data.range_check_9_9[22].push([mul_res_limb_16_col153, mul_res_limb_17_col154]);
sub_components_inputs
            .range_check_9_9_inputs[23]
            .extend([mul_res_limb_18_col155, mul_res_limb_19_col156].unpack());
        
lookup_data.range_check_9_9[23].push([mul_res_limb_18_col155, mul_res_limb_19_col156]);
sub_components_inputs
            .range_check_9_9_inputs[24]
            .extend([mul_res_limb_20_col157, mul_res_limb_21_col158].unpack());
        
lookup_data.range_check_9_9[24].push([mul_res_limb_20_col157, mul_res_limb_21_col158]);
sub_components_inputs
            .range_check_9_9_inputs[25]
            .extend([mul_res_limb_22_col159, mul_res_limb_23_col160].unpack());
        
lookup_data.range_check_9_9[25].push([mul_res_limb_22_col159, mul_res_limb_23_col160]);
sub_components_inputs
            .range_check_9_9_inputs[26]
            .extend([mul_res_limb_24_col161, mul_res_limb_25_col162].unpack());
        
lookup_data.range_check_9_9[26].push([mul_res_limb_24_col161, mul_res_limb_25_col162]);
sub_components_inputs
            .range_check_9_9_inputs[27]
            .extend([mul_res_limb_26_col163, mul_res_limb_27_col164].unpack());
        
lookup_data.range_check_9_9[27].push([mul_res_limb_26_col163, mul_res_limb_27_col164]);


        


        //VerifyMul252.

        
let conv_tmp_787 = ((((M31_0) - (mul_res_limb_0_col137))) + (((op0_limb_0_col51) * (op1_limb_0_col80))));
let conv_tmp_788 = ((((((M31_0) - (mul_res_limb_1_col138))) + (((op0_limb_0_col51) * (op1_limb_1_col81))))) + (((op0_limb_1_col52) * (op1_limb_0_col80))));
let conv_tmp_789 = ((((((((M31_0) - (mul_res_limb_2_col139))) + (((op0_limb_0_col51) * (op1_limb_2_col82))))) + (((op0_limb_1_col52) * (op1_limb_1_col81))))) + (((op0_limb_2_col53) * (op1_limb_0_col80))));
let conv_tmp_790 = ((((((((((M31_0) - (mul_res_limb_3_col140))) + (((op0_limb_0_col51) * (op1_limb_3_col83))))) + (((op0_limb_1_col52) * (op1_limb_2_col82))))) + (((op0_limb_2_col53) * (op1_limb_1_col81))))) + (((op0_limb_3_col54) * (op1_limb_0_col80))));
let conv_tmp_791 = ((((((((((((M31_0) - (mul_res_limb_4_col141))) + (((op0_limb_0_col51) * (op1_limb_4_col84))))) + (((op0_limb_1_col52) * (op1_limb_3_col83))))) + (((op0_limb_2_col53) * (op1_limb_2_col82))))) + (((op0_limb_3_col54) * (op1_limb_1_col81))))) + (((op0_limb_4_col55) * (op1_limb_0_col80))));
let conv_tmp_792 = ((((((((((((((M31_0) - (mul_res_limb_5_col142))) + (((op0_limb_0_col51) * (op1_limb_5_col85))))) + (((op0_limb_1_col52) * (op1_limb_4_col84))))) + (((op0_limb_2_col53) * (op1_limb_3_col83))))) + (((op0_limb_3_col54) * (op1_limb_2_col82))))) + (((op0_limb_4_col55) * (op1_limb_1_col81))))) + (((op0_limb_5_col56) * (op1_limb_0_col80))));
let conv_tmp_793 = ((((((((((((((((M31_0) - (mul_res_limb_6_col143))) + (((op0_limb_0_col51) * (op1_limb_6_col86))))) + (((op0_limb_1_col52) * (op1_limb_5_col85))))) + (((op0_limb_2_col53) * (op1_limb_4_col84))))) + (((op0_limb_3_col54) * (op1_limb_3_col83))))) + (((op0_limb_4_col55) * (op1_limb_2_col82))))) + (((op0_limb_5_col56) * (op1_limb_1_col81))))) + (((op0_limb_6_col57) * (op1_limb_0_col80))));
let conv_tmp_794 = ((((((((((((((((((M31_0) - (mul_res_limb_7_col144))) + (((op0_limb_0_col51) * (op1_limb_7_col87))))) + (((op0_limb_1_col52) * (op1_limb_6_col86))))) + (((op0_limb_2_col53) * (op1_limb_5_col85))))) + (((op0_limb_3_col54) * (op1_limb_4_col84))))) + (((op0_limb_4_col55) * (op1_limb_3_col83))))) + (((op0_limb_5_col56) * (op1_limb_2_col82))))) + (((op0_limb_6_col57) * (op1_limb_1_col81))))) + (((op0_limb_7_col58) * (op1_limb_0_col80))));
let conv_tmp_795 = ((((((((((((((((((((M31_0) - (mul_res_limb_8_col145))) + (((op0_limb_0_col51) * (op1_limb_8_col88))))) + (((op0_limb_1_col52) * (op1_limb_7_col87))))) + (((op0_limb_2_col53) * (op1_limb_6_col86))))) + (((op0_limb_3_col54) * (op1_limb_5_col85))))) + (((op0_limb_4_col55) * (op1_limb_4_col84))))) + (((op0_limb_5_col56) * (op1_limb_3_col83))))) + (((op0_limb_6_col57) * (op1_limb_2_col82))))) + (((op0_limb_7_col58) * (op1_limb_1_col81))))) + (((op0_limb_8_col59) * (op1_limb_0_col80))));
let conv_tmp_796 = ((((((((((((((((((((((M31_0) - (mul_res_limb_9_col146))) + (((op0_limb_0_col51) * (op1_limb_9_col89))))) + (((op0_limb_1_col52) * (op1_limb_8_col88))))) + (((op0_limb_2_col53) * (op1_limb_7_col87))))) + (((op0_limb_3_col54) * (op1_limb_6_col86))))) + (((op0_limb_4_col55) * (op1_limb_5_col85))))) + (((op0_limb_5_col56) * (op1_limb_4_col84))))) + (((op0_limb_6_col57) * (op1_limb_3_col83))))) + (((op0_limb_7_col58) * (op1_limb_2_col82))))) + (((op0_limb_8_col59) * (op1_limb_1_col81))))) + (((op0_limb_9_col60) * (op1_limb_0_col80))));
let conv_tmp_797 = ((((((((((((((((((((((((M31_0) - (mul_res_limb_10_col147))) + (((op0_limb_0_col51) * (op1_limb_10_col90))))) + (((op0_limb_1_col52) * (op1_limb_9_col89))))) + (((op0_limb_2_col53) * (op1_limb_8_col88))))) + (((op0_limb_3_col54) * (op1_limb_7_col87))))) + (((op0_limb_4_col55) * (op1_limb_6_col86))))) + (((op0_limb_5_col56) * (op1_limb_5_col85))))) + (((op0_limb_6_col57) * (op1_limb_4_col84))))) + (((op0_limb_7_col58) * (op1_limb_3_col83))))) + (((op0_limb_8_col59) * (op1_limb_2_col82))))) + (((op0_limb_9_col60) * (op1_limb_1_col81))))) + (((op0_limb_10_col61) * (op1_limb_0_col80))));
let conv_tmp_798 = ((((((((((((((((((((((((((M31_0) - (mul_res_limb_11_col148))) + (((op0_limb_0_col51) * (op1_limb_11_col91))))) + (((op0_limb_1_col52) * (op1_limb_10_col90))))) + (((op0_limb_2_col53) * (op1_limb_9_col89))))) + (((op0_limb_3_col54) * (op1_limb_8_col88))))) + (((op0_limb_4_col55) * (op1_limb_7_col87))))) + (((op0_limb_5_col56) * (op1_limb_6_col86))))) + (((op0_limb_6_col57) * (op1_limb_5_col85))))) + (((op0_limb_7_col58) * (op1_limb_4_col84))))) + (((op0_limb_8_col59) * (op1_limb_3_col83))))) + (((op0_limb_9_col60) * (op1_limb_2_col82))))) + (((op0_limb_10_col61) * (op1_limb_1_col81))))) + (((op0_limb_11_col62) * (op1_limb_0_col80))));
let conv_tmp_799 = ((((((((((((((((((((((((((((M31_0) - (mul_res_limb_12_col149))) + (((op0_limb_0_col51) * (op1_limb_12_col92))))) + (((op0_limb_1_col52) * (op1_limb_11_col91))))) + (((op0_limb_2_col53) * (op1_limb_10_col90))))) + (((op0_limb_3_col54) * (op1_limb_9_col89))))) + (((op0_limb_4_col55) * (op1_limb_8_col88))))) + (((op0_limb_5_col56) * (op1_limb_7_col87))))) + (((op0_limb_6_col57) * (op1_limb_6_col86))))) + (((op0_limb_7_col58) * (op1_limb_5_col85))))) + (((op0_limb_8_col59) * (op1_limb_4_col84))))) + (((op0_limb_9_col60) * (op1_limb_3_col83))))) + (((op0_limb_10_col61) * (op1_limb_2_col82))))) + (((op0_limb_11_col62) * (op1_limb_1_col81))))) + (((op0_limb_12_col63) * (op1_limb_0_col80))));
let conv_tmp_800 = ((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_13_col150))) + (((op0_limb_0_col51) * (op1_limb_13_col93))))) + (((op0_limb_1_col52) * (op1_limb_12_col92))))) + (((op0_limb_2_col53) * (op1_limb_11_col91))))) + (((op0_limb_3_col54) * (op1_limb_10_col90))))) + (((op0_limb_4_col55) * (op1_limb_9_col89))))) + (((op0_limb_5_col56) * (op1_limb_8_col88))))) + (((op0_limb_6_col57) * (op1_limb_7_col87))))) + (((op0_limb_7_col58) * (op1_limb_6_col86))))) + (((op0_limb_8_col59) * (op1_limb_5_col85))))) + (((op0_limb_9_col60) * (op1_limb_4_col84))))) + (((op0_limb_10_col61) * (op1_limb_3_col83))))) + (((op0_limb_11_col62) * (op1_limb_2_col82))))) + (((op0_limb_12_col63) * (op1_limb_1_col81))))) + (((op0_limb_13_col64) * (op1_limb_0_col80))));
let conv_tmp_801 = ((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_14_col151))) + (((op0_limb_0_col51) * (op1_limb_14_col94))))) + (((op0_limb_1_col52) * (op1_limb_13_col93))))) + (((op0_limb_2_col53) * (op1_limb_12_col92))))) + (((op0_limb_3_col54) * (op1_limb_11_col91))))) + (((op0_limb_4_col55) * (op1_limb_10_col90))))) + (((op0_limb_5_col56) * (op1_limb_9_col89))))) + (((op0_limb_6_col57) * (op1_limb_8_col88))))) + (((op0_limb_7_col58) * (op1_limb_7_col87))))) + (((op0_limb_8_col59) * (op1_limb_6_col86))))) + (((op0_limb_9_col60) * (op1_limb_5_col85))))) + (((op0_limb_10_col61) * (op1_limb_4_col84))))) + (((op0_limb_11_col62) * (op1_limb_3_col83))))) + (((op0_limb_12_col63) * (op1_limb_2_col82))))) + (((op0_limb_13_col64) * (op1_limb_1_col81))))) + (((op0_limb_14_col65) * (op1_limb_0_col80))));
let conv_tmp_802 = ((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_15_col152))) + (((op0_limb_0_col51) * (op1_limb_15_col95))))) + (((op0_limb_1_col52) * (op1_limb_14_col94))))) + (((op0_limb_2_col53) * (op1_limb_13_col93))))) + (((op0_limb_3_col54) * (op1_limb_12_col92))))) + (((op0_limb_4_col55) * (op1_limb_11_col91))))) + (((op0_limb_5_col56) * (op1_limb_10_col90))))) + (((op0_limb_6_col57) * (op1_limb_9_col89))))) + (((op0_limb_7_col58) * (op1_limb_8_col88))))) + (((op0_limb_8_col59) * (op1_limb_7_col87))))) + (((op0_limb_9_col60) * (op1_limb_6_col86))))) + (((op0_limb_10_col61) * (op1_limb_5_col85))))) + (((op0_limb_11_col62) * (op1_limb_4_col84))))) + (((op0_limb_12_col63) * (op1_limb_3_col83))))) + (((op0_limb_13_col64) * (op1_limb_2_col82))))) + (((op0_limb_14_col65) * (op1_limb_1_col81))))) + (((op0_limb_15_col66) * (op1_limb_0_col80))));
let conv_tmp_803 = ((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_16_col153))) + (((op0_limb_0_col51) * (op1_limb_16_col96))))) + (((op0_limb_1_col52) * (op1_limb_15_col95))))) + (((op0_limb_2_col53) * (op1_limb_14_col94))))) + (((op0_limb_3_col54) * (op1_limb_13_col93))))) + (((op0_limb_4_col55) * (op1_limb_12_col92))))) + (((op0_limb_5_col56) * (op1_limb_11_col91))))) + (((op0_limb_6_col57) * (op1_limb_10_col90))))) + (((op0_limb_7_col58) * (op1_limb_9_col89))))) + (((op0_limb_8_col59) * (op1_limb_8_col88))))) + (((op0_limb_9_col60) * (op1_limb_7_col87))))) + (((op0_limb_10_col61) * (op1_limb_6_col86))))) + (((op0_limb_11_col62) * (op1_limb_5_col85))))) + (((op0_limb_12_col63) * (op1_limb_4_col84))))) + (((op0_limb_13_col64) * (op1_limb_3_col83))))) + (((op0_limb_14_col65) * (op1_limb_2_col82))))) + (((op0_limb_15_col66) * (op1_limb_1_col81))))) + (((op0_limb_16_col67) * (op1_limb_0_col80))));
let conv_tmp_804 = ((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_17_col154))) + (((op0_limb_0_col51) * (op1_limb_17_col97))))) + (((op0_limb_1_col52) * (op1_limb_16_col96))))) + (((op0_limb_2_col53) * (op1_limb_15_col95))))) + (((op0_limb_3_col54) * (op1_limb_14_col94))))) + (((op0_limb_4_col55) * (op1_limb_13_col93))))) + (((op0_limb_5_col56) * (op1_limb_12_col92))))) + (((op0_limb_6_col57) * (op1_limb_11_col91))))) + (((op0_limb_7_col58) * (op1_limb_10_col90))))) + (((op0_limb_8_col59) * (op1_limb_9_col89))))) + (((op0_limb_9_col60) * (op1_limb_8_col88))))) + (((op0_limb_10_col61) * (op1_limb_7_col87))))) + (((op0_limb_11_col62) * (op1_limb_6_col86))))) + (((op0_limb_12_col63) * (op1_limb_5_col85))))) + (((op0_limb_13_col64) * (op1_limb_4_col84))))) + (((op0_limb_14_col65) * (op1_limb_3_col83))))) + (((op0_limb_15_col66) * (op1_limb_2_col82))))) + (((op0_limb_16_col67) * (op1_limb_1_col81))))) + (((op0_limb_17_col68) * (op1_limb_0_col80))));
let conv_tmp_805 = ((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_18_col155))) + (((op0_limb_0_col51) * (op1_limb_18_col98))))) + (((op0_limb_1_col52) * (op1_limb_17_col97))))) + (((op0_limb_2_col53) * (op1_limb_16_col96))))) + (((op0_limb_3_col54) * (op1_limb_15_col95))))) + (((op0_limb_4_col55) * (op1_limb_14_col94))))) + (((op0_limb_5_col56) * (op1_limb_13_col93))))) + (((op0_limb_6_col57) * (op1_limb_12_col92))))) + (((op0_limb_7_col58) * (op1_limb_11_col91))))) + (((op0_limb_8_col59) * (op1_limb_10_col90))))) + (((op0_limb_9_col60) * (op1_limb_9_col89))))) + (((op0_limb_10_col61) * (op1_limb_8_col88))))) + (((op0_limb_11_col62) * (op1_limb_7_col87))))) + (((op0_limb_12_col63) * (op1_limb_6_col86))))) + (((op0_limb_13_col64) * (op1_limb_5_col85))))) + (((op0_limb_14_col65) * (op1_limb_4_col84))))) + (((op0_limb_15_col66) * (op1_limb_3_col83))))) + (((op0_limb_16_col67) * (op1_limb_2_col82))))) + (((op0_limb_17_col68) * (op1_limb_1_col81))))) + (((op0_limb_18_col69) * (op1_limb_0_col80))));
let conv_tmp_806 = ((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_19_col156))) + (((op0_limb_0_col51) * (op1_limb_19_col99))))) + (((op0_limb_1_col52) * (op1_limb_18_col98))))) + (((op0_limb_2_col53) * (op1_limb_17_col97))))) + (((op0_limb_3_col54) * (op1_limb_16_col96))))) + (((op0_limb_4_col55) * (op1_limb_15_col95))))) + (((op0_limb_5_col56) * (op1_limb_14_col94))))) + (((op0_limb_6_col57) * (op1_limb_13_col93))))) + (((op0_limb_7_col58) * (op1_limb_12_col92))))) + (((op0_limb_8_col59) * (op1_limb_11_col91))))) + (((op0_limb_9_col60) * (op1_limb_10_col90))))) + (((op0_limb_10_col61) * (op1_limb_9_col89))))) + (((op0_limb_11_col62) * (op1_limb_8_col88))))) + (((op0_limb_12_col63) * (op1_limb_7_col87))))) + (((op0_limb_13_col64) * (op1_limb_6_col86))))) + (((op0_limb_14_col65) * (op1_limb_5_col85))))) + (((op0_limb_15_col66) * (op1_limb_4_col84))))) + (((op0_limb_16_col67) * (op1_limb_3_col83))))) + (((op0_limb_17_col68) * (op1_limb_2_col82))))) + (((op0_limb_18_col69) * (op1_limb_1_col81))))) + (((op0_limb_19_col70) * (op1_limb_0_col80))));
let conv_tmp_807 = ((((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_20_col157))) + (((op0_limb_0_col51) * (op1_limb_20_col100))))) + (((op0_limb_1_col52) * (op1_limb_19_col99))))) + (((op0_limb_2_col53) * (op1_limb_18_col98))))) + (((op0_limb_3_col54) * (op1_limb_17_col97))))) + (((op0_limb_4_col55) * (op1_limb_16_col96))))) + (((op0_limb_5_col56) * (op1_limb_15_col95))))) + (((op0_limb_6_col57) * (op1_limb_14_col94))))) + (((op0_limb_7_col58) * (op1_limb_13_col93))))) + (((op0_limb_8_col59) * (op1_limb_12_col92))))) + (((op0_limb_9_col60) * (op1_limb_11_col91))))) + (((op0_limb_10_col61) * (op1_limb_10_col90))))) + (((op0_limb_11_col62) * (op1_limb_9_col89))))) + (((op0_limb_12_col63) * (op1_limb_8_col88))))) + (((op0_limb_13_col64) * (op1_limb_7_col87))))) + (((op0_limb_14_col65) * (op1_limb_6_col86))))) + (((op0_limb_15_col66) * (op1_limb_5_col85))))) + (((op0_limb_16_col67) * (op1_limb_4_col84))))) + (((op0_limb_17_col68) * (op1_limb_3_col83))))) + (((op0_limb_18_col69) * (op1_limb_2_col82))))) + (((op0_limb_19_col70) * (op1_limb_1_col81))))) + (((op0_limb_20_col71) * (op1_limb_0_col80))));
let conv_tmp_808 = ((((((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_21_col158))) + (((op0_limb_0_col51) * (op1_limb_21_col101))))) + (((op0_limb_1_col52) * (op1_limb_20_col100))))) + (((op0_limb_2_col53) * (op1_limb_19_col99))))) + (((op0_limb_3_col54) * (op1_limb_18_col98))))) + (((op0_limb_4_col55) * (op1_limb_17_col97))))) + (((op0_limb_5_col56) * (op1_limb_16_col96))))) + (((op0_limb_6_col57) * (op1_limb_15_col95))))) + (((op0_limb_7_col58) * (op1_limb_14_col94))))) + (((op0_limb_8_col59) * (op1_limb_13_col93))))) + (((op0_limb_9_col60) * (op1_limb_12_col92))))) + (((op0_limb_10_col61) * (op1_limb_11_col91))))) + (((op0_limb_11_col62) * (op1_limb_10_col90))))) + (((op0_limb_12_col63) * (op1_limb_9_col89))))) + (((op0_limb_13_col64) * (op1_limb_8_col88))))) + (((op0_limb_14_col65) * (op1_limb_7_col87))))) + (((op0_limb_15_col66) * (op1_limb_6_col86))))) + (((op0_limb_16_col67) * (op1_limb_5_col85))))) + (((op0_limb_17_col68) * (op1_limb_4_col84))))) + (((op0_limb_18_col69) * (op1_limb_3_col83))))) + (((op0_limb_19_col70) * (op1_limb_2_col82))))) + (((op0_limb_20_col71) * (op1_limb_1_col81))))) + (((op0_limb_21_col72) * (op1_limb_0_col80))));
let conv_tmp_809 = ((((((((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_22_col159))) + (((op0_limb_0_col51) * (op1_limb_22_col102))))) + (((op0_limb_1_col52) * (op1_limb_21_col101))))) + (((op0_limb_2_col53) * (op1_limb_20_col100))))) + (((op0_limb_3_col54) * (op1_limb_19_col99))))) + (((op0_limb_4_col55) * (op1_limb_18_col98))))) + (((op0_limb_5_col56) * (op1_limb_17_col97))))) + (((op0_limb_6_col57) * (op1_limb_16_col96))))) + (((op0_limb_7_col58) * (op1_limb_15_col95))))) + (((op0_limb_8_col59) * (op1_limb_14_col94))))) + (((op0_limb_9_col60) * (op1_limb_13_col93))))) + (((op0_limb_10_col61) * (op1_limb_12_col92))))) + (((op0_limb_11_col62) * (op1_limb_11_col91))))) + (((op0_limb_12_col63) * (op1_limb_10_col90))))) + (((op0_limb_13_col64) * (op1_limb_9_col89))))) + (((op0_limb_14_col65) * (op1_limb_8_col88))))) + (((op0_limb_15_col66) * (op1_limb_7_col87))))) + (((op0_limb_16_col67) * (op1_limb_6_col86))))) + (((op0_limb_17_col68) * (op1_limb_5_col85))))) + (((op0_limb_18_col69) * (op1_limb_4_col84))))) + (((op0_limb_19_col70) * (op1_limb_3_col83))))) + (((op0_limb_20_col71) * (op1_limb_2_col82))))) + (((op0_limb_21_col72) * (op1_limb_1_col81))))) + (((op0_limb_22_col73) * (op1_limb_0_col80))));
let conv_tmp_810 = ((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_23_col160))) + (((op0_limb_0_col51) * (op1_limb_23_col103))))) + (((op0_limb_1_col52) * (op1_limb_22_col102))))) + (((op0_limb_2_col53) * (op1_limb_21_col101))))) + (((op0_limb_3_col54) * (op1_limb_20_col100))))) + (((op0_limb_4_col55) * (op1_limb_19_col99))))) + (((op0_limb_5_col56) * (op1_limb_18_col98))))) + (((op0_limb_6_col57) * (op1_limb_17_col97))))) + (((op0_limb_7_col58) * (op1_limb_16_col96))))) + (((op0_limb_8_col59) * (op1_limb_15_col95))))) + (((op0_limb_9_col60) * (op1_limb_14_col94))))) + (((op0_limb_10_col61) * (op1_limb_13_col93))))) + (((op0_limb_11_col62) * (op1_limb_12_col92))))) + (((op0_limb_12_col63) * (op1_limb_11_col91))))) + (((op0_limb_13_col64) * (op1_limb_10_col90))))) + (((op0_limb_14_col65) * (op1_limb_9_col89))))) + (((op0_limb_15_col66) * (op1_limb_8_col88))))) + (((op0_limb_16_col67) * (op1_limb_7_col87))))) + (((op0_limb_17_col68) * (op1_limb_6_col86))))) + (((op0_limb_18_col69) * (op1_limb_5_col85))))) + (((op0_limb_19_col70) * (op1_limb_4_col84))))) + (((op0_limb_20_col71) * (op1_limb_3_col83))))) + (((op0_limb_21_col72) * (op1_limb_2_col82))))) + (((op0_limb_22_col73) * (op1_limb_1_col81))))) + (((op0_limb_23_col74) * (op1_limb_0_col80))));
let conv_tmp_811 = ((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_24_col161))) + (((op0_limb_0_col51) * (op1_limb_24_col104))))) + (((op0_limb_1_col52) * (op1_limb_23_col103))))) + (((op0_limb_2_col53) * (op1_limb_22_col102))))) + (((op0_limb_3_col54) * (op1_limb_21_col101))))) + (((op0_limb_4_col55) * (op1_limb_20_col100))))) + (((op0_limb_5_col56) * (op1_limb_19_col99))))) + (((op0_limb_6_col57) * (op1_limb_18_col98))))) + (((op0_limb_7_col58) * (op1_limb_17_col97))))) + (((op0_limb_8_col59) * (op1_limb_16_col96))))) + (((op0_limb_9_col60) * (op1_limb_15_col95))))) + (((op0_limb_10_col61) * (op1_limb_14_col94))))) + (((op0_limb_11_col62) * (op1_limb_13_col93))))) + (((op0_limb_12_col63) * (op1_limb_12_col92))))) + (((op0_limb_13_col64) * (op1_limb_11_col91))))) + (((op0_limb_14_col65) * (op1_limb_10_col90))))) + (((op0_limb_15_col66) * (op1_limb_9_col89))))) + (((op0_limb_16_col67) * (op1_limb_8_col88))))) + (((op0_limb_17_col68) * (op1_limb_7_col87))))) + (((op0_limb_18_col69) * (op1_limb_6_col86))))) + (((op0_limb_19_col70) * (op1_limb_5_col85))))) + (((op0_limb_20_col71) * (op1_limb_4_col84))))) + (((op0_limb_21_col72) * (op1_limb_3_col83))))) + (((op0_limb_22_col73) * (op1_limb_2_col82))))) + (((op0_limb_23_col74) * (op1_limb_1_col81))))) + (((op0_limb_24_col75) * (op1_limb_0_col80))));
let conv_tmp_812 = ((((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_25_col162))) + (((op0_limb_0_col51) * (op1_limb_25_col105))))) + (((op0_limb_1_col52) * (op1_limb_24_col104))))) + (((op0_limb_2_col53) * (op1_limb_23_col103))))) + (((op0_limb_3_col54) * (op1_limb_22_col102))))) + (((op0_limb_4_col55) * (op1_limb_21_col101))))) + (((op0_limb_5_col56) * (op1_limb_20_col100))))) + (((op0_limb_6_col57) * (op1_limb_19_col99))))) + (((op0_limb_7_col58) * (op1_limb_18_col98))))) + (((op0_limb_8_col59) * (op1_limb_17_col97))))) + (((op0_limb_9_col60) * (op1_limb_16_col96))))) + (((op0_limb_10_col61) * (op1_limb_15_col95))))) + (((op0_limb_11_col62) * (op1_limb_14_col94))))) + (((op0_limb_12_col63) * (op1_limb_13_col93))))) + (((op0_limb_13_col64) * (op1_limb_12_col92))))) + (((op0_limb_14_col65) * (op1_limb_11_col91))))) + (((op0_limb_15_col66) * (op1_limb_10_col90))))) + (((op0_limb_16_col67) * (op1_limb_9_col89))))) + (((op0_limb_17_col68) * (op1_limb_8_col88))))) + (((op0_limb_18_col69) * (op1_limb_7_col87))))) + (((op0_limb_19_col70) * (op1_limb_6_col86))))) + (((op0_limb_20_col71) * (op1_limb_5_col85))))) + (((op0_limb_21_col72) * (op1_limb_4_col84))))) + (((op0_limb_22_col73) * (op1_limb_3_col83))))) + (((op0_limb_23_col74) * (op1_limb_2_col82))))) + (((op0_limb_24_col75) * (op1_limb_1_col81))))) + (((op0_limb_25_col76) * (op1_limb_0_col80))));
let conv_tmp_813 = ((((((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_26_col163))) + (((op0_limb_0_col51) * (op1_limb_26_col106))))) + (((op0_limb_1_col52) * (op1_limb_25_col105))))) + (((op0_limb_2_col53) * (op1_limb_24_col104))))) + (((op0_limb_3_col54) * (op1_limb_23_col103))))) + (((op0_limb_4_col55) * (op1_limb_22_col102))))) + (((op0_limb_5_col56) * (op1_limb_21_col101))))) + (((op0_limb_6_col57) * (op1_limb_20_col100))))) + (((op0_limb_7_col58) * (op1_limb_19_col99))))) + (((op0_limb_8_col59) * (op1_limb_18_col98))))) + (((op0_limb_9_col60) * (op1_limb_17_col97))))) + (((op0_limb_10_col61) * (op1_limb_16_col96))))) + (((op0_limb_11_col62) * (op1_limb_15_col95))))) + (((op0_limb_12_col63) * (op1_limb_14_col94))))) + (((op0_limb_13_col64) * (op1_limb_13_col93))))) + (((op0_limb_14_col65) * (op1_limb_12_col92))))) + (((op0_limb_15_col66) * (op1_limb_11_col91))))) + (((op0_limb_16_col67) * (op1_limb_10_col90))))) + (((op0_limb_17_col68) * (op1_limb_9_col89))))) + (((op0_limb_18_col69) * (op1_limb_8_col88))))) + (((op0_limb_19_col70) * (op1_limb_7_col87))))) + (((op0_limb_20_col71) * (op1_limb_6_col86))))) + (((op0_limb_21_col72) * (op1_limb_5_col85))))) + (((op0_limb_22_col73) * (op1_limb_4_col84))))) + (((op0_limb_23_col74) * (op1_limb_3_col83))))) + (((op0_limb_24_col75) * (op1_limb_2_col82))))) + (((op0_limb_25_col76) * (op1_limb_1_col81))))) + (((op0_limb_26_col77) * (op1_limb_0_col80))));
let conv_tmp_814 = ((((((((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) - (mul_res_limb_27_col164))) + (((op0_limb_0_col51) * (op1_limb_27_col107))))) + (((op0_limb_1_col52) * (op1_limb_26_col106))))) + (((op0_limb_2_col53) * (op1_limb_25_col105))))) + (((op0_limb_3_col54) * (op1_limb_24_col104))))) + (((op0_limb_4_col55) * (op1_limb_23_col103))))) + (((op0_limb_5_col56) * (op1_limb_22_col102))))) + (((op0_limb_6_col57) * (op1_limb_21_col101))))) + (((op0_limb_7_col58) * (op1_limb_20_col100))))) + (((op0_limb_8_col59) * (op1_limb_19_col99))))) + (((op0_limb_9_col60) * (op1_limb_18_col98))))) + (((op0_limb_10_col61) * (op1_limb_17_col97))))) + (((op0_limb_11_col62) * (op1_limb_16_col96))))) + (((op0_limb_12_col63) * (op1_limb_15_col95))))) + (((op0_limb_13_col64) * (op1_limb_14_col94))))) + (((op0_limb_14_col65) * (op1_limb_13_col93))))) + (((op0_limb_15_col66) * (op1_limb_12_col92))))) + (((op0_limb_16_col67) * (op1_limb_11_col91))))) + (((op0_limb_17_col68) * (op1_limb_10_col90))))) + (((op0_limb_18_col69) * (op1_limb_9_col89))))) + (((op0_limb_19_col70) * (op1_limb_8_col88))))) + (((op0_limb_20_col71) * (op1_limb_7_col87))))) + (((op0_limb_21_col72) * (op1_limb_6_col86))))) + (((op0_limb_22_col73) * (op1_limb_5_col85))))) + (((op0_limb_23_col74) * (op1_limb_4_col84))))) + (((op0_limb_24_col75) * (op1_limb_3_col83))))) + (((op0_limb_25_col76) * (op1_limb_2_col82))))) + (((op0_limb_26_col77) * (op1_limb_1_col81))))) + (((op0_limb_27_col78) * (op1_limb_0_col80))));
let conv_tmp_815 = ((((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_1_col52) * (op1_limb_27_col107))))) + (((op0_limb_2_col53) * (op1_limb_26_col106))))) + (((op0_limb_3_col54) * (op1_limb_25_col105))))) + (((op0_limb_4_col55) * (op1_limb_24_col104))))) + (((op0_limb_5_col56) * (op1_limb_23_col103))))) + (((op0_limb_6_col57) * (op1_limb_22_col102))))) + (((op0_limb_7_col58) * (op1_limb_21_col101))))) + (((op0_limb_8_col59) * (op1_limb_20_col100))))) + (((op0_limb_9_col60) * (op1_limb_19_col99))))) + (((op0_limb_10_col61) * (op1_limb_18_col98))))) + (((op0_limb_11_col62) * (op1_limb_17_col97))))) + (((op0_limb_12_col63) * (op1_limb_16_col96))))) + (((op0_limb_13_col64) * (op1_limb_15_col95))))) + (((op0_limb_14_col65) * (op1_limb_14_col94))))) + (((op0_limb_15_col66) * (op1_limb_13_col93))))) + (((op0_limb_16_col67) * (op1_limb_12_col92))))) + (((op0_limb_17_col68) * (op1_limb_11_col91))))) + (((op0_limb_18_col69) * (op1_limb_10_col90))))) + (((op0_limb_19_col70) * (op1_limb_9_col89))))) + (((op0_limb_20_col71) * (op1_limb_8_col88))))) + (((op0_limb_21_col72) * (op1_limb_7_col87))))) + (((op0_limb_22_col73) * (op1_limb_6_col86))))) + (((op0_limb_23_col74) * (op1_limb_5_col85))))) + (((op0_limb_24_col75) * (op1_limb_4_col84))))) + (((op0_limb_25_col76) * (op1_limb_3_col83))))) + (((op0_limb_26_col77) * (op1_limb_2_col82))))) + (((op0_limb_27_col78) * (op1_limb_1_col81))));
let conv_tmp_816 = ((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_2_col53) * (op1_limb_27_col107))))) + (((op0_limb_3_col54) * (op1_limb_26_col106))))) + (((op0_limb_4_col55) * (op1_limb_25_col105))))) + (((op0_limb_5_col56) * (op1_limb_24_col104))))) + (((op0_limb_6_col57) * (op1_limb_23_col103))))) + (((op0_limb_7_col58) * (op1_limb_22_col102))))) + (((op0_limb_8_col59) * (op1_limb_21_col101))))) + (((op0_limb_9_col60) * (op1_limb_20_col100))))) + (((op0_limb_10_col61) * (op1_limb_19_col99))))) + (((op0_limb_11_col62) * (op1_limb_18_col98))))) + (((op0_limb_12_col63) * (op1_limb_17_col97))))) + (((op0_limb_13_col64) * (op1_limb_16_col96))))) + (((op0_limb_14_col65) * (op1_limb_15_col95))))) + (((op0_limb_15_col66) * (op1_limb_14_col94))))) + (((op0_limb_16_col67) * (op1_limb_13_col93))))) + (((op0_limb_17_col68) * (op1_limb_12_col92))))) + (((op0_limb_18_col69) * (op1_limb_11_col91))))) + (((op0_limb_19_col70) * (op1_limb_10_col90))))) + (((op0_limb_20_col71) * (op1_limb_9_col89))))) + (((op0_limb_21_col72) * (op1_limb_8_col88))))) + (((op0_limb_22_col73) * (op1_limb_7_col87))))) + (((op0_limb_23_col74) * (op1_limb_6_col86))))) + (((op0_limb_24_col75) * (op1_limb_5_col85))))) + (((op0_limb_25_col76) * (op1_limb_4_col84))))) + (((op0_limb_26_col77) * (op1_limb_3_col83))))) + (((op0_limb_27_col78) * (op1_limb_2_col82))));
let conv_tmp_817 = ((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_3_col54) * (op1_limb_27_col107))))) + (((op0_limb_4_col55) * (op1_limb_26_col106))))) + (((op0_limb_5_col56) * (op1_limb_25_col105))))) + (((op0_limb_6_col57) * (op1_limb_24_col104))))) + (((op0_limb_7_col58) * (op1_limb_23_col103))))) + (((op0_limb_8_col59) * (op1_limb_22_col102))))) + (((op0_limb_9_col60) * (op1_limb_21_col101))))) + (((op0_limb_10_col61) * (op1_limb_20_col100))))) + (((op0_limb_11_col62) * (op1_limb_19_col99))))) + (((op0_limb_12_col63) * (op1_limb_18_col98))))) + (((op0_limb_13_col64) * (op1_limb_17_col97))))) + (((op0_limb_14_col65) * (op1_limb_16_col96))))) + (((op0_limb_15_col66) * (op1_limb_15_col95))))) + (((op0_limb_16_col67) * (op1_limb_14_col94))))) + (((op0_limb_17_col68) * (op1_limb_13_col93))))) + (((op0_limb_18_col69) * (op1_limb_12_col92))))) + (((op0_limb_19_col70) * (op1_limb_11_col91))))) + (((op0_limb_20_col71) * (op1_limb_10_col90))))) + (((op0_limb_21_col72) * (op1_limb_9_col89))))) + (((op0_limb_22_col73) * (op1_limb_8_col88))))) + (((op0_limb_23_col74) * (op1_limb_7_col87))))) + (((op0_limb_24_col75) * (op1_limb_6_col86))))) + (((op0_limb_25_col76) * (op1_limb_5_col85))))) + (((op0_limb_26_col77) * (op1_limb_4_col84))))) + (((op0_limb_27_col78) * (op1_limb_3_col83))));
let conv_tmp_818 = ((((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_4_col55) * (op1_limb_27_col107))))) + (((op0_limb_5_col56) * (op1_limb_26_col106))))) + (((op0_limb_6_col57) * (op1_limb_25_col105))))) + (((op0_limb_7_col58) * (op1_limb_24_col104))))) + (((op0_limb_8_col59) * (op1_limb_23_col103))))) + (((op0_limb_9_col60) * (op1_limb_22_col102))))) + (((op0_limb_10_col61) * (op1_limb_21_col101))))) + (((op0_limb_11_col62) * (op1_limb_20_col100))))) + (((op0_limb_12_col63) * (op1_limb_19_col99))))) + (((op0_limb_13_col64) * (op1_limb_18_col98))))) + (((op0_limb_14_col65) * (op1_limb_17_col97))))) + (((op0_limb_15_col66) * (op1_limb_16_col96))))) + (((op0_limb_16_col67) * (op1_limb_15_col95))))) + (((op0_limb_17_col68) * (op1_limb_14_col94))))) + (((op0_limb_18_col69) * (op1_limb_13_col93))))) + (((op0_limb_19_col70) * (op1_limb_12_col92))))) + (((op0_limb_20_col71) * (op1_limb_11_col91))))) + (((op0_limb_21_col72) * (op1_limb_10_col90))))) + (((op0_limb_22_col73) * (op1_limb_9_col89))))) + (((op0_limb_23_col74) * (op1_limb_8_col88))))) + (((op0_limb_24_col75) * (op1_limb_7_col87))))) + (((op0_limb_25_col76) * (op1_limb_6_col86))))) + (((op0_limb_26_col77) * (op1_limb_5_col85))))) + (((op0_limb_27_col78) * (op1_limb_4_col84))));
let conv_tmp_819 = ((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_5_col56) * (op1_limb_27_col107))))) + (((op0_limb_6_col57) * (op1_limb_26_col106))))) + (((op0_limb_7_col58) * (op1_limb_25_col105))))) + (((op0_limb_8_col59) * (op1_limb_24_col104))))) + (((op0_limb_9_col60) * (op1_limb_23_col103))))) + (((op0_limb_10_col61) * (op1_limb_22_col102))))) + (((op0_limb_11_col62) * (op1_limb_21_col101))))) + (((op0_limb_12_col63) * (op1_limb_20_col100))))) + (((op0_limb_13_col64) * (op1_limb_19_col99))))) + (((op0_limb_14_col65) * (op1_limb_18_col98))))) + (((op0_limb_15_col66) * (op1_limb_17_col97))))) + (((op0_limb_16_col67) * (op1_limb_16_col96))))) + (((op0_limb_17_col68) * (op1_limb_15_col95))))) + (((op0_limb_18_col69) * (op1_limb_14_col94))))) + (((op0_limb_19_col70) * (op1_limb_13_col93))))) + (((op0_limb_20_col71) * (op1_limb_12_col92))))) + (((op0_limb_21_col72) * (op1_limb_11_col91))))) + (((op0_limb_22_col73) * (op1_limb_10_col90))))) + (((op0_limb_23_col74) * (op1_limb_9_col89))))) + (((op0_limb_24_col75) * (op1_limb_8_col88))))) + (((op0_limb_25_col76) * (op1_limb_7_col87))))) + (((op0_limb_26_col77) * (op1_limb_6_col86))))) + (((op0_limb_27_col78) * (op1_limb_5_col85))));
let conv_tmp_820 = ((((((((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_6_col57) * (op1_limb_27_col107))))) + (((op0_limb_7_col58) * (op1_limb_26_col106))))) + (((op0_limb_8_col59) * (op1_limb_25_col105))))) + (((op0_limb_9_col60) * (op1_limb_24_col104))))) + (((op0_limb_10_col61) * (op1_limb_23_col103))))) + (((op0_limb_11_col62) * (op1_limb_22_col102))))) + (((op0_limb_12_col63) * (op1_limb_21_col101))))) + (((op0_limb_13_col64) * (op1_limb_20_col100))))) + (((op0_limb_14_col65) * (op1_limb_19_col99))))) + (((op0_limb_15_col66) * (op1_limb_18_col98))))) + (((op0_limb_16_col67) * (op1_limb_17_col97))))) + (((op0_limb_17_col68) * (op1_limb_16_col96))))) + (((op0_limb_18_col69) * (op1_limb_15_col95))))) + (((op0_limb_19_col70) * (op1_limb_14_col94))))) + (((op0_limb_20_col71) * (op1_limb_13_col93))))) + (((op0_limb_21_col72) * (op1_limb_12_col92))))) + (((op0_limb_22_col73) * (op1_limb_11_col91))))) + (((op0_limb_23_col74) * (op1_limb_10_col90))))) + (((op0_limb_24_col75) * (op1_limb_9_col89))))) + (((op0_limb_25_col76) * (op1_limb_8_col88))))) + (((op0_limb_26_col77) * (op1_limb_7_col87))))) + (((op0_limb_27_col78) * (op1_limb_6_col86))));
let conv_tmp_821 = ((((((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_7_col58) * (op1_limb_27_col107))))) + (((op0_limb_8_col59) * (op1_limb_26_col106))))) + (((op0_limb_9_col60) * (op1_limb_25_col105))))) + (((op0_limb_10_col61) * (op1_limb_24_col104))))) + (((op0_limb_11_col62) * (op1_limb_23_col103))))) + (((op0_limb_12_col63) * (op1_limb_22_col102))))) + (((op0_limb_13_col64) * (op1_limb_21_col101))))) + (((op0_limb_14_col65) * (op1_limb_20_col100))))) + (((op0_limb_15_col66) * (op1_limb_19_col99))))) + (((op0_limb_16_col67) * (op1_limb_18_col98))))) + (((op0_limb_17_col68) * (op1_limb_17_col97))))) + (((op0_limb_18_col69) * (op1_limb_16_col96))))) + (((op0_limb_19_col70) * (op1_limb_15_col95))))) + (((op0_limb_20_col71) * (op1_limb_14_col94))))) + (((op0_limb_21_col72) * (op1_limb_13_col93))))) + (((op0_limb_22_col73) * (op1_limb_12_col92))))) + (((op0_limb_23_col74) * (op1_limb_11_col91))))) + (((op0_limb_24_col75) * (op1_limb_10_col90))))) + (((op0_limb_25_col76) * (op1_limb_9_col89))))) + (((op0_limb_26_col77) * (op1_limb_8_col88))))) + (((op0_limb_27_col78) * (op1_limb_7_col87))));
let conv_tmp_822 = ((((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_8_col59) * (op1_limb_27_col107))))) + (((op0_limb_9_col60) * (op1_limb_26_col106))))) + (((op0_limb_10_col61) * (op1_limb_25_col105))))) + (((op0_limb_11_col62) * (op1_limb_24_col104))))) + (((op0_limb_12_col63) * (op1_limb_23_col103))))) + (((op0_limb_13_col64) * (op1_limb_22_col102))))) + (((op0_limb_14_col65) * (op1_limb_21_col101))))) + (((op0_limb_15_col66) * (op1_limb_20_col100))))) + (((op0_limb_16_col67) * (op1_limb_19_col99))))) + (((op0_limb_17_col68) * (op1_limb_18_col98))))) + (((op0_limb_18_col69) * (op1_limb_17_col97))))) + (((op0_limb_19_col70) * (op1_limb_16_col96))))) + (((op0_limb_20_col71) * (op1_limb_15_col95))))) + (((op0_limb_21_col72) * (op1_limb_14_col94))))) + (((op0_limb_22_col73) * (op1_limb_13_col93))))) + (((op0_limb_23_col74) * (op1_limb_12_col92))))) + (((op0_limb_24_col75) * (op1_limb_11_col91))))) + (((op0_limb_25_col76) * (op1_limb_10_col90))))) + (((op0_limb_26_col77) * (op1_limb_9_col89))))) + (((op0_limb_27_col78) * (op1_limb_8_col88))));
let conv_tmp_823 = ((((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_9_col60) * (op1_limb_27_col107))))) + (((op0_limb_10_col61) * (op1_limb_26_col106))))) + (((op0_limb_11_col62) * (op1_limb_25_col105))))) + (((op0_limb_12_col63) * (op1_limb_24_col104))))) + (((op0_limb_13_col64) * (op1_limb_23_col103))))) + (((op0_limb_14_col65) * (op1_limb_22_col102))))) + (((op0_limb_15_col66) * (op1_limb_21_col101))))) + (((op0_limb_16_col67) * (op1_limb_20_col100))))) + (((op0_limb_17_col68) * (op1_limb_19_col99))))) + (((op0_limb_18_col69) * (op1_limb_18_col98))))) + (((op0_limb_19_col70) * (op1_limb_17_col97))))) + (((op0_limb_20_col71) * (op1_limb_16_col96))))) + (((op0_limb_21_col72) * (op1_limb_15_col95))))) + (((op0_limb_22_col73) * (op1_limb_14_col94))))) + (((op0_limb_23_col74) * (op1_limb_13_col93))))) + (((op0_limb_24_col75) * (op1_limb_12_col92))))) + (((op0_limb_25_col76) * (op1_limb_11_col91))))) + (((op0_limb_26_col77) * (op1_limb_10_col90))))) + (((op0_limb_27_col78) * (op1_limb_9_col89))));
let conv_tmp_824 = ((((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_10_col61) * (op1_limb_27_col107))))) + (((op0_limb_11_col62) * (op1_limb_26_col106))))) + (((op0_limb_12_col63) * (op1_limb_25_col105))))) + (((op0_limb_13_col64) * (op1_limb_24_col104))))) + (((op0_limb_14_col65) * (op1_limb_23_col103))))) + (((op0_limb_15_col66) * (op1_limb_22_col102))))) + (((op0_limb_16_col67) * (op1_limb_21_col101))))) + (((op0_limb_17_col68) * (op1_limb_20_col100))))) + (((op0_limb_18_col69) * (op1_limb_19_col99))))) + (((op0_limb_19_col70) * (op1_limb_18_col98))))) + (((op0_limb_20_col71) * (op1_limb_17_col97))))) + (((op0_limb_21_col72) * (op1_limb_16_col96))))) + (((op0_limb_22_col73) * (op1_limb_15_col95))))) + (((op0_limb_23_col74) * (op1_limb_14_col94))))) + (((op0_limb_24_col75) * (op1_limb_13_col93))))) + (((op0_limb_25_col76) * (op1_limb_12_col92))))) + (((op0_limb_26_col77) * (op1_limb_11_col91))))) + (((op0_limb_27_col78) * (op1_limb_10_col90))));
let conv_tmp_825 = ((((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_11_col62) * (op1_limb_27_col107))))) + (((op0_limb_12_col63) * (op1_limb_26_col106))))) + (((op0_limb_13_col64) * (op1_limb_25_col105))))) + (((op0_limb_14_col65) * (op1_limb_24_col104))))) + (((op0_limb_15_col66) * (op1_limb_23_col103))))) + (((op0_limb_16_col67) * (op1_limb_22_col102))))) + (((op0_limb_17_col68) * (op1_limb_21_col101))))) + (((op0_limb_18_col69) * (op1_limb_20_col100))))) + (((op0_limb_19_col70) * (op1_limb_19_col99))))) + (((op0_limb_20_col71) * (op1_limb_18_col98))))) + (((op0_limb_21_col72) * (op1_limb_17_col97))))) + (((op0_limb_22_col73) * (op1_limb_16_col96))))) + (((op0_limb_23_col74) * (op1_limb_15_col95))))) + (((op0_limb_24_col75) * (op1_limb_14_col94))))) + (((op0_limb_25_col76) * (op1_limb_13_col93))))) + (((op0_limb_26_col77) * (op1_limb_12_col92))))) + (((op0_limb_27_col78) * (op1_limb_11_col91))));
let conv_tmp_826 = ((((((((((((((((((((((((((((((((M31_0) + (((op0_limb_12_col63) * (op1_limb_27_col107))))) + (((op0_limb_13_col64) * (op1_limb_26_col106))))) + (((op0_limb_14_col65) * (op1_limb_25_col105))))) + (((op0_limb_15_col66) * (op1_limb_24_col104))))) + (((op0_limb_16_col67) * (op1_limb_23_col103))))) + (((op0_limb_17_col68) * (op1_limb_22_col102))))) + (((op0_limb_18_col69) * (op1_limb_21_col101))))) + (((op0_limb_19_col70) * (op1_limb_20_col100))))) + (((op0_limb_20_col71) * (op1_limb_19_col99))))) + (((op0_limb_21_col72) * (op1_limb_18_col98))))) + (((op0_limb_22_col73) * (op1_limb_17_col97))))) + (((op0_limb_23_col74) * (op1_limb_16_col96))))) + (((op0_limb_24_col75) * (op1_limb_15_col95))))) + (((op0_limb_25_col76) * (op1_limb_14_col94))))) + (((op0_limb_26_col77) * (op1_limb_13_col93))))) + (((op0_limb_27_col78) * (op1_limb_12_col92))));
let conv_tmp_827 = ((((((((((((((((((((((((((((((M31_0) + (((op0_limb_13_col64) * (op1_limb_27_col107))))) + (((op0_limb_14_col65) * (op1_limb_26_col106))))) + (((op0_limb_15_col66) * (op1_limb_25_col105))))) + (((op0_limb_16_col67) * (op1_limb_24_col104))))) + (((op0_limb_17_col68) * (op1_limb_23_col103))))) + (((op0_limb_18_col69) * (op1_limb_22_col102))))) + (((op0_limb_19_col70) * (op1_limb_21_col101))))) + (((op0_limb_20_col71) * (op1_limb_20_col100))))) + (((op0_limb_21_col72) * (op1_limb_19_col99))))) + (((op0_limb_22_col73) * (op1_limb_18_col98))))) + (((op0_limb_23_col74) * (op1_limb_17_col97))))) + (((op0_limb_24_col75) * (op1_limb_16_col96))))) + (((op0_limb_25_col76) * (op1_limb_15_col95))))) + (((op0_limb_26_col77) * (op1_limb_14_col94))))) + (((op0_limb_27_col78) * (op1_limb_13_col93))));
let conv_tmp_828 = ((((((((((((((((((((((((((((M31_0) + (((op0_limb_14_col65) * (op1_limb_27_col107))))) + (((op0_limb_15_col66) * (op1_limb_26_col106))))) + (((op0_limb_16_col67) * (op1_limb_25_col105))))) + (((op0_limb_17_col68) * (op1_limb_24_col104))))) + (((op0_limb_18_col69) * (op1_limb_23_col103))))) + (((op0_limb_19_col70) * (op1_limb_22_col102))))) + (((op0_limb_20_col71) * (op1_limb_21_col101))))) + (((op0_limb_21_col72) * (op1_limb_20_col100))))) + (((op0_limb_22_col73) * (op1_limb_19_col99))))) + (((op0_limb_23_col74) * (op1_limb_18_col98))))) + (((op0_limb_24_col75) * (op1_limb_17_col97))))) + (((op0_limb_25_col76) * (op1_limb_16_col96))))) + (((op0_limb_26_col77) * (op1_limb_15_col95))))) + (((op0_limb_27_col78) * (op1_limb_14_col94))));
let conv_tmp_829 = ((((((((((((((((((((((((((M31_0) + (((op0_limb_15_col66) * (op1_limb_27_col107))))) + (((op0_limb_16_col67) * (op1_limb_26_col106))))) + (((op0_limb_17_col68) * (op1_limb_25_col105))))) + (((op0_limb_18_col69) * (op1_limb_24_col104))))) + (((op0_limb_19_col70) * (op1_limb_23_col103))))) + (((op0_limb_20_col71) * (op1_limb_22_col102))))) + (((op0_limb_21_col72) * (op1_limb_21_col101))))) + (((op0_limb_22_col73) * (op1_limb_20_col100))))) + (((op0_limb_23_col74) * (op1_limb_19_col99))))) + (((op0_limb_24_col75) * (op1_limb_18_col98))))) + (((op0_limb_25_col76) * (op1_limb_17_col97))))) + (((op0_limb_26_col77) * (op1_limb_16_col96))))) + (((op0_limb_27_col78) * (op1_limb_15_col95))));
let conv_tmp_830 = ((((((((((((((((((((((((M31_0) + (((op0_limb_16_col67) * (op1_limb_27_col107))))) + (((op0_limb_17_col68) * (op1_limb_26_col106))))) + (((op0_limb_18_col69) * (op1_limb_25_col105))))) + (((op0_limb_19_col70) * (op1_limb_24_col104))))) + (((op0_limb_20_col71) * (op1_limb_23_col103))))) + (((op0_limb_21_col72) * (op1_limb_22_col102))))) + (((op0_limb_22_col73) * (op1_limb_21_col101))))) + (((op0_limb_23_col74) * (op1_limb_20_col100))))) + (((op0_limb_24_col75) * (op1_limb_19_col99))))) + (((op0_limb_25_col76) * (op1_limb_18_col98))))) + (((op0_limb_26_col77) * (op1_limb_17_col97))))) + (((op0_limb_27_col78) * (op1_limb_16_col96))));
let conv_tmp_831 = ((((((((((((((((((((((M31_0) + (((op0_limb_17_col68) * (op1_limb_27_col107))))) + (((op0_limb_18_col69) * (op1_limb_26_col106))))) + (((op0_limb_19_col70) * (op1_limb_25_col105))))) + (((op0_limb_20_col71) * (op1_limb_24_col104))))) + (((op0_limb_21_col72) * (op1_limb_23_col103))))) + (((op0_limb_22_col73) * (op1_limb_22_col102))))) + (((op0_limb_23_col74) * (op1_limb_21_col101))))) + (((op0_limb_24_col75) * (op1_limb_20_col100))))) + (((op0_limb_25_col76) * (op1_limb_19_col99))))) + (((op0_limb_26_col77) * (op1_limb_18_col98))))) + (((op0_limb_27_col78) * (op1_limb_17_col97))));
let conv_tmp_832 = ((((((((((((((((((((M31_0) + (((op0_limb_18_col69) * (op1_limb_27_col107))))) + (((op0_limb_19_col70) * (op1_limb_26_col106))))) + (((op0_limb_20_col71) * (op1_limb_25_col105))))) + (((op0_limb_21_col72) * (op1_limb_24_col104))))) + (((op0_limb_22_col73) * (op1_limb_23_col103))))) + (((op0_limb_23_col74) * (op1_limb_22_col102))))) + (((op0_limb_24_col75) * (op1_limb_21_col101))))) + (((op0_limb_25_col76) * (op1_limb_20_col100))))) + (((op0_limb_26_col77) * (op1_limb_19_col99))))) + (((op0_limb_27_col78) * (op1_limb_18_col98))));
let conv_tmp_833 = ((((((((((((((((((M31_0) + (((op0_limb_19_col70) * (op1_limb_27_col107))))) + (((op0_limb_20_col71) * (op1_limb_26_col106))))) + (((op0_limb_21_col72) * (op1_limb_25_col105))))) + (((op0_limb_22_col73) * (op1_limb_24_col104))))) + (((op0_limb_23_col74) * (op1_limb_23_col103))))) + (((op0_limb_24_col75) * (op1_limb_22_col102))))) + (((op0_limb_25_col76) * (op1_limb_21_col101))))) + (((op0_limb_26_col77) * (op1_limb_20_col100))))) + (((op0_limb_27_col78) * (op1_limb_19_col99))));
let conv_tmp_834 = ((((((((((((((((M31_0) + (((op0_limb_20_col71) * (op1_limb_27_col107))))) + (((op0_limb_21_col72) * (op1_limb_26_col106))))) + (((op0_limb_22_col73) * (op1_limb_25_col105))))) + (((op0_limb_23_col74) * (op1_limb_24_col104))))) + (((op0_limb_24_col75) * (op1_limb_23_col103))))) + (((op0_limb_25_col76) * (op1_limb_22_col102))))) + (((op0_limb_26_col77) * (op1_limb_21_col101))))) + (((op0_limb_27_col78) * (op1_limb_20_col100))));
let conv_tmp_835 = ((((((((((((((M31_0) + (((op0_limb_21_col72) * (op1_limb_27_col107))))) + (((op0_limb_22_col73) * (op1_limb_26_col106))))) + (((op0_limb_23_col74) * (op1_limb_25_col105))))) + (((op0_limb_24_col75) * (op1_limb_24_col104))))) + (((op0_limb_25_col76) * (op1_limb_23_col103))))) + (((op0_limb_26_col77) * (op1_limb_22_col102))))) + (((op0_limb_27_col78) * (op1_limb_21_col101))));
let conv_tmp_836 = ((((((((((((M31_0) + (((op0_limb_22_col73) * (op1_limb_27_col107))))) + (((op0_limb_23_col74) * (op1_limb_26_col106))))) + (((op0_limb_24_col75) * (op1_limb_25_col105))))) + (((op0_limb_25_col76) * (op1_limb_24_col104))))) + (((op0_limb_26_col77) * (op1_limb_23_col103))))) + (((op0_limb_27_col78) * (op1_limb_22_col102))));
let conv_tmp_837 = ((((((((((M31_0) + (((op0_limb_23_col74) * (op1_limb_27_col107))))) + (((op0_limb_24_col75) * (op1_limb_26_col106))))) + (((op0_limb_25_col76) * (op1_limb_25_col105))))) + (((op0_limb_26_col77) * (op1_limb_24_col104))))) + (((op0_limb_27_col78) * (op1_limb_23_col103))));
let conv_tmp_838 = ((((((((M31_0) + (((op0_limb_24_col75) * (op1_limb_27_col107))))) + (((op0_limb_25_col76) * (op1_limb_26_col106))))) + (((op0_limb_26_col77) * (op1_limb_25_col105))))) + (((op0_limb_27_col78) * (op1_limb_24_col104))));
let conv_tmp_839 = ((((((M31_0) + (((op0_limb_25_col76) * (op1_limb_27_col107))))) + (((op0_limb_26_col77) * (op1_limb_26_col106))))) + (((op0_limb_27_col78) * (op1_limb_25_col105))));
let conv_tmp_840 = ((((M31_0) + (((op0_limb_26_col77) * (op1_limb_27_col107))))) + (((op0_limb_27_col78) * (op1_limb_26_col106))));
let conv_tmp_841 = ((M31_0) + (((op0_limb_27_col78) * (op1_limb_27_col107))));
let conv_mod_tmp_842 = ((((((M31_0) + (((M31_32) * (conv_tmp_787))))) - (((M31_4) * (conv_tmp_808))))) + (((M31_8) * (conv_tmp_836))));
let conv_mod_tmp_843 = ((((((((M31_0) + (((M31_1) * (conv_tmp_787))))) + (((M31_32) * (conv_tmp_788))))) - (((M31_4) * (conv_tmp_809))))) + (((M31_8) * (conv_tmp_837))));
let conv_mod_tmp_844 = ((((((((M31_0) + (((M31_1) * (conv_tmp_788))))) + (((M31_32) * (conv_tmp_789))))) - (((M31_4) * (conv_tmp_810))))) + (((M31_8) * (conv_tmp_838))));
let conv_mod_tmp_845 = ((((((((M31_0) + (((M31_1) * (conv_tmp_789))))) + (((M31_32) * (conv_tmp_790))))) - (((M31_4) * (conv_tmp_811))))) + (((M31_8) * (conv_tmp_839))));
let conv_mod_tmp_846 = ((((((((M31_0) + (((M31_1) * (conv_tmp_790))))) + (((M31_32) * (conv_tmp_791))))) - (((M31_4) * (conv_tmp_812))))) + (((M31_8) * (conv_tmp_840))));
let conv_mod_tmp_847 = ((((((((M31_0) + (((M31_1) * (conv_tmp_791))))) + (((M31_32) * (conv_tmp_792))))) - (((M31_4) * (conv_tmp_813))))) + (((M31_8) * (conv_tmp_841))));
let conv_mod_tmp_848 = ((((((M31_0) + (((M31_1) * (conv_tmp_792))))) + (((M31_32) * (conv_tmp_793))))) - (((M31_4) * (conv_tmp_814))));
let conv_mod_tmp_849 = ((((((((M31_0) + (((M31_2) * (conv_tmp_787))))) + (((M31_1) * (conv_tmp_793))))) + (((M31_32) * (conv_tmp_794))))) - (((M31_4) * (conv_tmp_815))));
let conv_mod_tmp_850 = ((((((((M31_0) + (((M31_2) * (conv_tmp_788))))) + (((M31_1) * (conv_tmp_794))))) + (((M31_32) * (conv_tmp_795))))) - (((M31_4) * (conv_tmp_816))));
let conv_mod_tmp_851 = ((((((((M31_0) + (((M31_2) * (conv_tmp_789))))) + (((M31_1) * (conv_tmp_795))))) + (((M31_32) * (conv_tmp_796))))) - (((M31_4) * (conv_tmp_817))));
let conv_mod_tmp_852 = ((((((((M31_0) + (((M31_2) * (conv_tmp_790))))) + (((M31_1) * (conv_tmp_796))))) + (((M31_32) * (conv_tmp_797))))) - (((M31_4) * (conv_tmp_818))));
let conv_mod_tmp_853 = ((((((((M31_0) + (((M31_2) * (conv_tmp_791))))) + (((M31_1) * (conv_tmp_797))))) + (((M31_32) * (conv_tmp_798))))) - (((M31_4) * (conv_tmp_819))));
let conv_mod_tmp_854 = ((((((((M31_0) + (((M31_2) * (conv_tmp_792))))) + (((M31_1) * (conv_tmp_798))))) + (((M31_32) * (conv_tmp_799))))) - (((M31_4) * (conv_tmp_820))));
let conv_mod_tmp_855 = ((((((((M31_0) + (((M31_2) * (conv_tmp_793))))) + (((M31_1) * (conv_tmp_799))))) + (((M31_32) * (conv_tmp_800))))) - (((M31_4) * (conv_tmp_821))));
let conv_mod_tmp_856 = ((((((((M31_0) + (((M31_2) * (conv_tmp_794))))) + (((M31_1) * (conv_tmp_800))))) + (((M31_32) * (conv_tmp_801))))) - (((M31_4) * (conv_tmp_822))));
let conv_mod_tmp_857 = ((((((((M31_0) + (((M31_2) * (conv_tmp_795))))) + (((M31_1) * (conv_tmp_801))))) + (((M31_32) * (conv_tmp_802))))) - (((M31_4) * (conv_tmp_823))));
let conv_mod_tmp_858 = ((((((((M31_0) + (((M31_2) * (conv_tmp_796))))) + (((M31_1) * (conv_tmp_802))))) + (((M31_32) * (conv_tmp_803))))) - (((M31_4) * (conv_tmp_824))));
let conv_mod_tmp_859 = ((((((((M31_0) + (((M31_2) * (conv_tmp_797))))) + (((M31_1) * (conv_tmp_803))))) + (((M31_32) * (conv_tmp_804))))) - (((M31_4) * (conv_tmp_825))));
let conv_mod_tmp_860 = ((((((((M31_0) + (((M31_2) * (conv_tmp_798))))) + (((M31_1) * (conv_tmp_804))))) + (((M31_32) * (conv_tmp_805))))) - (((M31_4) * (conv_tmp_826))));
let conv_mod_tmp_861 = ((((((((M31_0) + (((M31_2) * (conv_tmp_799))))) + (((M31_1) * (conv_tmp_805))))) + (((M31_32) * (conv_tmp_806))))) - (((M31_4) * (conv_tmp_827))));
let conv_mod_tmp_862 = ((((((((M31_0) + (((M31_2) * (conv_tmp_800))))) + (((M31_1) * (conv_tmp_806))))) + (((M31_32) * (conv_tmp_807))))) - (((M31_4) * (conv_tmp_828))));
let conv_mod_tmp_863 = ((((((((M31_0) + (((M31_2) * (conv_tmp_801))))) + (((M31_1) * (conv_tmp_807))))) - (((M31_4) * (conv_tmp_829))))) + (((M31_64) * (conv_tmp_836))));
let conv_mod_tmp_864 = ((((((((M31_0) + (((M31_2) * (conv_tmp_802))))) - (((M31_4) * (conv_tmp_830))))) + (((M31_2) * (conv_tmp_836))))) + (((M31_64) * (conv_tmp_837))));
let conv_mod_tmp_865 = ((((((((M31_0) + (((M31_2) * (conv_tmp_803))))) - (((M31_4) * (conv_tmp_831))))) + (((M31_2) * (conv_tmp_837))))) + (((M31_64) * (conv_tmp_838))));
let conv_mod_tmp_866 = ((((((((M31_0) + (((M31_2) * (conv_tmp_804))))) - (((M31_4) * (conv_tmp_832))))) + (((M31_2) * (conv_tmp_838))))) + (((M31_64) * (conv_tmp_839))));
let conv_mod_tmp_867 = ((((((((M31_0) + (((M31_2) * (conv_tmp_805))))) - (((M31_4) * (conv_tmp_833))))) + (((M31_2) * (conv_tmp_839))))) + (((M31_64) * (conv_tmp_840))));
let conv_mod_tmp_868 = ((((((((M31_0) + (((M31_2) * (conv_tmp_806))))) - (((M31_4) * (conv_tmp_834))))) + (((M31_2) * (conv_tmp_840))))) + (((M31_64) * (conv_tmp_841))));
let conv_mod_tmp_869 = ((((((M31_0) + (((M31_2) * (conv_tmp_807))))) - (((M31_4) * (conv_tmp_835))))) + (((M31_2) * (conv_tmp_841))));
let k_mod_2_18_biased_tmp_870 = ((((((PackedUInt32::from_m31(((conv_mod_tmp_842) + (M31_134217728)))) + (((((PackedUInt32::from_m31(((conv_mod_tmp_843) + (M31_134217728)))) & (UInt32_511))) << (UInt32_9))))) + (UInt32_65536))) & (UInt32_262143));
let k_col165 = ((k_mod_2_18_biased_tmp_870.low().as_m31()) + (((((k_mod_2_18_biased_tmp_870.high().as_m31()) - (M31_1))) * (M31_65536))));
        trace[165].data[row_index] = k_col165;
sub_components_inputs
            .range_check_19_inputs[0]
            .extend([((k_col165) + (M31_262144))].unpack());
        
lookup_data.range_check_19[0].push([((k_col165) + (M31_262144))]);
let carry_0_col166 = ((((((conv_mod_tmp_842) - (((M31_1) * (k_col165))))) + (M31_0))) * (M31_4194304));
        trace[166].data[row_index] = carry_0_col166;
sub_components_inputs
            .range_check_19_inputs[1]
            .extend([((carry_0_col166) + (M31_131072))].unpack());
        
lookup_data.range_check_19[1].push([((carry_0_col166) + (M31_131072))]);
let carry_1_col167 = ((((conv_mod_tmp_843) + (carry_0_col166))) * (M31_4194304));
        trace[167].data[row_index] = carry_1_col167;
sub_components_inputs
            .range_check_19_inputs[2]
            .extend([((carry_1_col167) + (M31_131072))].unpack());
        
lookup_data.range_check_19[2].push([((carry_1_col167) + (M31_131072))]);
let carry_2_col168 = ((((conv_mod_tmp_844) + (carry_1_col167))) * (M31_4194304));
        trace[168].data[row_index] = carry_2_col168;
sub_components_inputs
            .range_check_19_inputs[3]
            .extend([((carry_2_col168) + (M31_131072))].unpack());
        
lookup_data.range_check_19[3].push([((carry_2_col168) + (M31_131072))]);
let carry_3_col169 = ((((conv_mod_tmp_845) + (carry_2_col168))) * (M31_4194304));
        trace[169].data[row_index] = carry_3_col169;
sub_components_inputs
            .range_check_19_inputs[4]
            .extend([((carry_3_col169) + (M31_131072))].unpack());
        
lookup_data.range_check_19[4].push([((carry_3_col169) + (M31_131072))]);
let carry_4_col170 = ((((conv_mod_tmp_846) + (carry_3_col169))) * (M31_4194304));
        trace[170].data[row_index] = carry_4_col170;
sub_components_inputs
            .range_check_19_inputs[5]
            .extend([((carry_4_col170) + (M31_131072))].unpack());
        
lookup_data.range_check_19[5].push([((carry_4_col170) + (M31_131072))]);
let carry_5_col171 = ((((conv_mod_tmp_847) + (carry_4_col170))) * (M31_4194304));
        trace[171].data[row_index] = carry_5_col171;
sub_components_inputs
            .range_check_19_inputs[6]
            .extend([((carry_5_col171) + (M31_131072))].unpack());
        
lookup_data.range_check_19[6].push([((carry_5_col171) + (M31_131072))]);
let carry_6_col172 = ((((conv_mod_tmp_848) + (carry_5_col171))) * (M31_4194304));
        trace[172].data[row_index] = carry_6_col172;
sub_components_inputs
            .range_check_19_inputs[7]
            .extend([((carry_6_col172) + (M31_131072))].unpack());
        
lookup_data.range_check_19[7].push([((carry_6_col172) + (M31_131072))]);
let carry_7_col173 = ((((conv_mod_tmp_849) + (carry_6_col172))) * (M31_4194304));
        trace[173].data[row_index] = carry_7_col173;
sub_components_inputs
            .range_check_19_inputs[8]
            .extend([((carry_7_col173) + (M31_131072))].unpack());
        
lookup_data.range_check_19[8].push([((carry_7_col173) + (M31_131072))]);
let carry_8_col174 = ((((conv_mod_tmp_850) + (carry_7_col173))) * (M31_4194304));
        trace[174].data[row_index] = carry_8_col174;
sub_components_inputs
            .range_check_19_inputs[9]
            .extend([((carry_8_col174) + (M31_131072))].unpack());
        
lookup_data.range_check_19[9].push([((carry_8_col174) + (M31_131072))]);
let carry_9_col175 = ((((conv_mod_tmp_851) + (carry_8_col174))) * (M31_4194304));
        trace[175].data[row_index] = carry_9_col175;
sub_components_inputs
            .range_check_19_inputs[10]
            .extend([((carry_9_col175) + (M31_131072))].unpack());
        
lookup_data.range_check_19[10].push([((carry_9_col175) + (M31_131072))]);
let carry_10_col176 = ((((conv_mod_tmp_852) + (carry_9_col175))) * (M31_4194304));
        trace[176].data[row_index] = carry_10_col176;
sub_components_inputs
            .range_check_19_inputs[11]
            .extend([((carry_10_col176) + (M31_131072))].unpack());
        
lookup_data.range_check_19[11].push([((carry_10_col176) + (M31_131072))]);
let carry_11_col177 = ((((conv_mod_tmp_853) + (carry_10_col176))) * (M31_4194304));
        trace[177].data[row_index] = carry_11_col177;
sub_components_inputs
            .range_check_19_inputs[12]
            .extend([((carry_11_col177) + (M31_131072))].unpack());
        
lookup_data.range_check_19[12].push([((carry_11_col177) + (M31_131072))]);
let carry_12_col178 = ((((conv_mod_tmp_854) + (carry_11_col177))) * (M31_4194304));
        trace[178].data[row_index] = carry_12_col178;
sub_components_inputs
            .range_check_19_inputs[13]
            .extend([((carry_12_col178) + (M31_131072))].unpack());
        
lookup_data.range_check_19[13].push([((carry_12_col178) + (M31_131072))]);
let carry_13_col179 = ((((conv_mod_tmp_855) + (carry_12_col178))) * (M31_4194304));
        trace[179].data[row_index] = carry_13_col179;
sub_components_inputs
            .range_check_19_inputs[14]
            .extend([((carry_13_col179) + (M31_131072))].unpack());
        
lookup_data.range_check_19[14].push([((carry_13_col179) + (M31_131072))]);
let carry_14_col180 = ((((conv_mod_tmp_856) + (carry_13_col179))) * (M31_4194304));
        trace[180].data[row_index] = carry_14_col180;
sub_components_inputs
            .range_check_19_inputs[15]
            .extend([((carry_14_col180) + (M31_131072))].unpack());
        
lookup_data.range_check_19[15].push([((carry_14_col180) + (M31_131072))]);
let carry_15_col181 = ((((conv_mod_tmp_857) + (carry_14_col180))) * (M31_4194304));
        trace[181].data[row_index] = carry_15_col181;
sub_components_inputs
            .range_check_19_inputs[16]
            .extend([((carry_15_col181) + (M31_131072))].unpack());
        
lookup_data.range_check_19[16].push([((carry_15_col181) + (M31_131072))]);
let carry_16_col182 = ((((conv_mod_tmp_858) + (carry_15_col181))) * (M31_4194304));
        trace[182].data[row_index] = carry_16_col182;
sub_components_inputs
            .range_check_19_inputs[17]
            .extend([((carry_16_col182) + (M31_131072))].unpack());
        
lookup_data.range_check_19[17].push([((carry_16_col182) + (M31_131072))]);
let carry_17_col183 = ((((conv_mod_tmp_859) + (carry_16_col182))) * (M31_4194304));
        trace[183].data[row_index] = carry_17_col183;
sub_components_inputs
            .range_check_19_inputs[18]
            .extend([((carry_17_col183) + (M31_131072))].unpack());
        
lookup_data.range_check_19[18].push([((carry_17_col183) + (M31_131072))]);
let carry_18_col184 = ((((conv_mod_tmp_860) + (carry_17_col183))) * (M31_4194304));
        trace[184].data[row_index] = carry_18_col184;
sub_components_inputs
            .range_check_19_inputs[19]
            .extend([((carry_18_col184) + (M31_131072))].unpack());
        
lookup_data.range_check_19[19].push([((carry_18_col184) + (M31_131072))]);
let carry_19_col185 = ((((conv_mod_tmp_861) + (carry_18_col184))) * (M31_4194304));
        trace[185].data[row_index] = carry_19_col185;
sub_components_inputs
            .range_check_19_inputs[20]
            .extend([((carry_19_col185) + (M31_131072))].unpack());
        
lookup_data.range_check_19[20].push([((carry_19_col185) + (M31_131072))]);
let carry_20_col186 = ((((conv_mod_tmp_862) + (carry_19_col185))) * (M31_4194304));
        trace[186].data[row_index] = carry_20_col186;
sub_components_inputs
            .range_check_19_inputs[21]
            .extend([((carry_20_col186) + (M31_131072))].unpack());
        
lookup_data.range_check_19[21].push([((carry_20_col186) + (M31_131072))]);
let carry_21_col187 = ((((((conv_mod_tmp_863) - (((M31_136) * (k_col165))))) + (carry_20_col186))) * (M31_4194304));
        trace[187].data[row_index] = carry_21_col187;
sub_components_inputs
            .range_check_19_inputs[22]
            .extend([((carry_21_col187) + (M31_131072))].unpack());
        
lookup_data.range_check_19[22].push([((carry_21_col187) + (M31_131072))]);
let carry_22_col188 = ((((conv_mod_tmp_864) + (carry_21_col187))) * (M31_4194304));
        trace[188].data[row_index] = carry_22_col188;
sub_components_inputs
            .range_check_19_inputs[23]
            .extend([((carry_22_col188) + (M31_131072))].unpack());
        
lookup_data.range_check_19[23].push([((carry_22_col188) + (M31_131072))]);
let carry_23_col189 = ((((conv_mod_tmp_865) + (carry_22_col188))) * (M31_4194304));
        trace[189].data[row_index] = carry_23_col189;
sub_components_inputs
            .range_check_19_inputs[24]
            .extend([((carry_23_col189) + (M31_131072))].unpack());
        
lookup_data.range_check_19[24].push([((carry_23_col189) + (M31_131072))]);
let carry_24_col190 = ((((conv_mod_tmp_866) + (carry_23_col189))) * (M31_4194304));
        trace[190].data[row_index] = carry_24_col190;
sub_components_inputs
            .range_check_19_inputs[25]
            .extend([((carry_24_col190) + (M31_131072))].unpack());
        
lookup_data.range_check_19[25].push([((carry_24_col190) + (M31_131072))]);
let carry_25_col191 = ((((conv_mod_tmp_867) + (carry_24_col190))) * (M31_4194304));
        trace[191].data[row_index] = carry_25_col191;
sub_components_inputs
            .range_check_19_inputs[26]
            .extend([((carry_25_col191) + (M31_131072))].unpack());
        
lookup_data.range_check_19[26].push([((carry_25_col191) + (M31_131072))]);
let carry_26_col192 = ((((conv_mod_tmp_868) + (carry_25_col191))) * (M31_4194304));
        trace[192].data[row_index] = carry_26_col192;
sub_components_inputs
            .range_check_19_inputs[27]
            .extend([((carry_26_col192) + (M31_131072))].unpack());
        
lookup_data.range_check_19[27].push([((carry_26_col192) + (M31_131072))]);


        


        
let res_tmp_899 = ((((((PackedFelt252::from_m31(res_op1_tmp_231)) * (PackedFelt252::from_limbs([op1_limb_0_col80, op1_limb_1_col81, op1_limb_2_col82, op1_limb_3_col83, op1_limb_4_col84, op1_limb_5_col85, op1_limb_6_col86, op1_limb_7_col87, op1_limb_8_col88, op1_limb_9_col89, op1_limb_10_col90, op1_limb_11_col91, op1_limb_12_col92, op1_limb_13_col93, op1_limb_14_col94, op1_limb_15_col95, op1_limb_16_col96, op1_limb_17_col97, op1_limb_18_col98, op1_limb_19_col99, op1_limb_20_col100, op1_limb_21_col101, op1_limb_22_col102, op1_limb_23_col103, op1_limb_24_col104, op1_limb_25_col105, op1_limb_26_col106, op1_limb_27_col107])))) + (((PackedFelt252::from_m31(res_mul_col12)) * (mul_res_tmp_772))))) + (((PackedFelt252::from_m31(res_add_col11)) * (add_res_tmp_729))));
let res_limb_0_col193 = res_tmp_899.get_m31(0);
        trace[193].data[row_index] = res_limb_0_col193;
let res_limb_1_col194 = res_tmp_899.get_m31(1);
        trace[194].data[row_index] = res_limb_1_col194;
let res_limb_2_col195 = res_tmp_899.get_m31(2);
        trace[195].data[row_index] = res_limb_2_col195;
let res_limb_3_col196 = res_tmp_899.get_m31(3);
        trace[196].data[row_index] = res_limb_3_col196;
let res_limb_4_col197 = res_tmp_899.get_m31(4);
        trace[197].data[row_index] = res_limb_4_col197;
let res_limb_5_col198 = res_tmp_899.get_m31(5);
        trace[198].data[row_index] = res_limb_5_col198;
let res_limb_6_col199 = res_tmp_899.get_m31(6);
        trace[199].data[row_index] = res_limb_6_col199;
let res_limb_7_col200 = res_tmp_899.get_m31(7);
        trace[200].data[row_index] = res_limb_7_col200;
let res_limb_8_col201 = res_tmp_899.get_m31(8);
        trace[201].data[row_index] = res_limb_8_col201;
let res_limb_9_col202 = res_tmp_899.get_m31(9);
        trace[202].data[row_index] = res_limb_9_col202;
let res_limb_10_col203 = res_tmp_899.get_m31(10);
        trace[203].data[row_index] = res_limb_10_col203;
let res_limb_11_col204 = res_tmp_899.get_m31(11);
        trace[204].data[row_index] = res_limb_11_col204;
let res_limb_12_col205 = res_tmp_899.get_m31(12);
        trace[205].data[row_index] = res_limb_12_col205;
let res_limb_13_col206 = res_tmp_899.get_m31(13);
        trace[206].data[row_index] = res_limb_13_col206;
let res_limb_14_col207 = res_tmp_899.get_m31(14);
        trace[207].data[row_index] = res_limb_14_col207;
let res_limb_15_col208 = res_tmp_899.get_m31(15);
        trace[208].data[row_index] = res_limb_15_col208;
let res_limb_16_col209 = res_tmp_899.get_m31(16);
        trace[209].data[row_index] = res_limb_16_col209;
let res_limb_17_col210 = res_tmp_899.get_m31(17);
        trace[210].data[row_index] = res_limb_17_col210;
let res_limb_18_col211 = res_tmp_899.get_m31(18);
        trace[211].data[row_index] = res_limb_18_col211;
let res_limb_19_col212 = res_tmp_899.get_m31(19);
        trace[212].data[row_index] = res_limb_19_col212;
let res_limb_20_col213 = res_tmp_899.get_m31(20);
        trace[213].data[row_index] = res_limb_20_col213;
let res_limb_21_col214 = res_tmp_899.get_m31(21);
        trace[214].data[row_index] = res_limb_21_col214;
let res_limb_22_col215 = res_tmp_899.get_m31(22);
        trace[215].data[row_index] = res_limb_22_col215;
let res_limb_23_col216 = res_tmp_899.get_m31(23);
        trace[216].data[row_index] = res_limb_23_col216;
let res_limb_24_col217 = res_tmp_899.get_m31(24);
        trace[217].data[row_index] = res_limb_24_col217;
let res_limb_25_col218 = res_tmp_899.get_m31(25);
        trace[218].data[row_index] = res_limb_25_col218;
let res_limb_26_col219 = res_tmp_899.get_m31(26);
        trace[219].data[row_index] = res_limb_26_col219;
let res_limb_27_col220 = res_tmp_899.get_m31(27);
        trace[220].data[row_index] = res_limb_27_col220;


        


        //HandleOpcodes.

        


        //CondFelt252AsAddr.

        


        


        //CondFelt252AsAddr.

        


        


        


        //UpdateRegisters.

        


        //CondFelt252AsAddr.

        


        


        //CondFelt252AsAddr.

        


        


        //CondFelt252AsRelImm.

        


        //CondDecodeSmallSign.

        
let msb_tmp_913 = res_limb_27_col220.eq(M31_256);
let msb_col221 = msb_tmp_913.as_m31();
        trace[221].data[row_index] = msb_col221;
let mid_limbs_set_tmp_914 = res_limb_20_col213.eq(M31_511);
let mid_limbs_set_col222 = mid_limbs_set_tmp_914.as_m31();
        trace[222].data[row_index] = mid_limbs_set_col222;


        


        
let diff_from_p_tmp_915 = ((dst_limb_0_col22) - (M31_1));
let diff_from_p_tmp_916 = ((dst_limb_21_col43) - (M31_136));
let diff_from_p_tmp_917 = ((dst_limb_27_col49) - (M31_256));
let dst_sum_squares_inv_col223 = ((M31_1).div(((((((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (((diff_from_p_tmp_915) * (diff_from_p_tmp_915))))) + (dst_limb_1_col23))) + (dst_limb_2_col24))) + (dst_limb_3_col25))) + (dst_limb_4_col26))) + (dst_limb_5_col27))) + (dst_limb_6_col28))) + (dst_limb_7_col29))) + (dst_limb_8_col30))) + (dst_limb_9_col31))) + (dst_limb_10_col32))) + (dst_limb_11_col33))) + (dst_limb_12_col34))) + (dst_limb_13_col35))) + (dst_limb_14_col36))) + (dst_limb_15_col37))) + (dst_limb_16_col38))) + (dst_limb_17_col39))) + (dst_limb_18_col40))) + (dst_limb_19_col41))) + (dst_limb_20_col42))) + (((diff_from_p_tmp_916) * (diff_from_p_tmp_916))))) + (dst_limb_22_col44))) + (dst_limb_23_col45))) + (dst_limb_24_col46))) + (dst_limb_25_col47))) + (dst_limb_26_col48))) + (((diff_from_p_tmp_917) * (diff_from_p_tmp_917))))));
        trace[223].data[row_index] = dst_sum_squares_inv_col223;
let dst_is_zero_tmp_918 = ((((((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (dst_limb_0_col22))) + (dst_limb_1_col23))) + (dst_limb_2_col24))) + (dst_limb_3_col25))) + (dst_limb_4_col26))) + (dst_limb_5_col27))) + (dst_limb_6_col28))) + (dst_limb_7_col29))) + (dst_limb_8_col30))) + (dst_limb_9_col31))) + (dst_limb_10_col32))) + (dst_limb_11_col33))) + (dst_limb_12_col34))) + (dst_limb_13_col35))) + (dst_limb_14_col36))) + (dst_limb_15_col37))) + (dst_limb_16_col38))) + (dst_limb_17_col39))) + (dst_limb_18_col40))) + (dst_limb_19_col41))) + (dst_limb_20_col42))) + (dst_limb_21_col43))) + (dst_limb_22_col44))) + (dst_limb_23_col45))) + (dst_limb_24_col46))) + (dst_limb_25_col47))) + (dst_limb_26_col48))) + (dst_limb_27_col49)).eq(M31_0);
let dst_sum_inv_col224 = ((M31_1).div(((((((((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (dst_limb_0_col22))) + (dst_limb_1_col23))) + (dst_limb_2_col24))) + (dst_limb_3_col25))) + (dst_limb_4_col26))) + (dst_limb_5_col27))) + (dst_limb_6_col28))) + (dst_limb_7_col29))) + (dst_limb_8_col30))) + (dst_limb_9_col31))) + (dst_limb_10_col32))) + (dst_limb_11_col33))) + (dst_limb_12_col34))) + (dst_limb_13_col35))) + (dst_limb_14_col36))) + (dst_limb_15_col37))) + (dst_limb_16_col38))) + (dst_limb_17_col39))) + (dst_limb_18_col40))) + (dst_limb_19_col41))) + (dst_limb_20_col42))) + (dst_limb_21_col43))) + (dst_limb_22_col44))) + (dst_limb_23_col45))) + (dst_limb_24_col46))) + (dst_limb_25_col47))) + (dst_limb_26_col48))) + (dst_limb_27_col49))) + (dst_is_zero_tmp_918.as_m31()))));
        trace[224].data[row_index] = dst_sum_inv_col224;
let op1_as_rel_imm_cond_col225 = ((pc_update_jnz_col15) * (((((((((((((((((((((((((((((((((((((((((((((((((((((((((M31_0) + (dst_limb_0_col22))) + (dst_limb_1_col23))) + (dst_limb_2_col24))) + (dst_limb_3_col25))) + (dst_limb_4_col26))) + (dst_limb_5_col27))) + (dst_limb_6_col28))) + (dst_limb_7_col29))) + (dst_limb_8_col30))) + (dst_limb_9_col31))) + (dst_limb_10_col32))) + (dst_limb_11_col33))) + (dst_limb_12_col34))) + (dst_limb_13_col35))) + (dst_limb_14_col36))) + (dst_limb_15_col37))) + (dst_limb_16_col38))) + (dst_limb_17_col39))) + (dst_limb_18_col40))) + (dst_limb_19_col41))) + (dst_limb_20_col42))) + (dst_limb_21_col43))) + (dst_limb_22_col44))) + (dst_limb_23_col45))) + (dst_limb_24_col46))) + (dst_limb_25_col47))) + (dst_limb_26_col48))) + (dst_limb_27_col49))));
        trace[225].data[row_index] = op1_as_rel_imm_cond_col225;


        //CondFelt252AsRelImm.

        


        //CondDecodeSmallSign.

        
let msb_tmp_919 = op1_limb_27_col107.eq(M31_256);
let msb_col226 = msb_tmp_919.as_m31();
        trace[226].data[row_index] = msb_col226;
let mid_limbs_set_tmp_920 = op1_limb_20_col100.eq(M31_511);
let mid_limbs_set_col227 = mid_limbs_set_tmp_920.as_m31();
        trace[227].data[row_index] = mid_limbs_set_col227;


        


        
let next_pc_jnz_col228 = ((((dst_is_zero_tmp_918.as_m31()) * (((input_pc_col0) + (((M31_1) + (op1_imm_col8))))))) + (((((M31_1) - (dst_is_zero_tmp_918.as_m31()))) * (((input_pc_col0) + (((((((((op1_limb_0_col80) + (((op1_limb_1_col81) * (M31_512))))) + (((op1_limb_2_col82) * (M31_262144))))) - (msb_col226))) - (((M31_134217728) * (mid_limbs_set_col227))))))))));
        trace[228].data[row_index] = next_pc_jnz_col228;


        
lookup_data.opcodes[0].push([input_pc_col0, input_ap_col1, input_fp_col2]);
lookup_data.opcodes[1].push([((((((((pc_update_regular_tmp_232) * (((input_pc_col0) + (((M31_1) + (op1_imm_col8))))))) + (((pc_update_jump_col13) * (((((res_limb_0_col193) + (((res_limb_1_col194) * (M31_512))))) + (((res_limb_2_col195) * (M31_262144))))))))) + (((pc_update_jump_rel_col14) * (((input_pc_col0) + (((((((((res_limb_0_col193) + (((res_limb_1_col194) * (M31_512))))) + (((res_limb_2_col195) * (M31_262144))))) - (msb_col221))) - (((M31_134217728) * (mid_limbs_set_col222))))))))))) + (((pc_update_jnz_col15) * (next_pc_jnz_col228)))), ((((((input_ap_col1) + (((ap_update_add_col16) * (((((((((res_limb_0_col193) + (((res_limb_1_col194) * (M31_512))))) + (((res_limb_2_col195) * (M31_262144))))) - (msb_col221))) - (((M31_134217728) * (mid_limbs_set_col222))))))))) + (((ap_update_add_1_col17) * (M31_1))))) + (((opcode_call_col18) * (M31_2)))), ((((((fp_update_regular_tmp_234) * (input_fp_col2))) + (((opcode_ret_col19) * (((((dst_limb_0_col22) + (((dst_limb_1_col23) * (M31_512))))) + (((dst_limb_2_col24) * (M31_262144))))))))) + (((opcode_call_col18) * (((input_ap_col1) + (M31_2))))))]);

    });

    (trace, sub_components_inputs, lookup_data)
}

#[derive(Default)]
pub struct LookupData
{pub addr_to_id: [Vec<[PackedM31; 2]>; 3],pub id_to_f252: [Vec<[PackedM31; 29]>; 3],pub range_check_19: [Vec<[PackedM31; 1]>; 28],pub range_check_9_9: [Vec<[PackedM31; 2]>; 28],pub verifyinstruction: [Vec<[PackedM31; 19]>; 1],pub opcodes: [Vec<[PackedM31; 3]>; 2],}
impl LookupData {
    #[allow(unused_variables)]
    fn with_capacity(capacity: usize) -> Self {
        Self {addr_to_id: [Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),],id_to_f252: [Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),],range_check_19: [Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),],range_check_9_9: [Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),Vec::with_capacity(capacity),],verifyinstruction: [Vec::with_capacity(capacity),],opcodes: [Vec::with_capacity(capacity),Vec::with_capacity(capacity),],}

    }
}

#[derive(Default)]
pub struct InteractionClaimGenerator {
    pub n_calls: usize,
    pub lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        memoryaddresstoid_lookup_elements:
            &relations::AddrToId,
        memoryidtobig_lookup_elements:
            &relations::IdToValue,
        range_check_19_lookup_elements:
            &relations::RangeCheck_19,
        range_check_9_9_lookup_elements:
            &relations::RangeCheck_9_9,
        verifyinstruction_lookup_elements:
            &relations::VerifyInstruction,
        opcodes_lookup_elements:
            &relations::Vm,
    ) -> InteractionClaim {
        let log_size = std::cmp::max(self.n_calls.next_power_of_two().ilog2(), LOG_N_LANES);
        let mut logup_gen = LogupTraceGenerator::new(log_size);

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .verifyinstruction[0];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                verifyinstruction_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .addr_to_id[0];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                addr_to_id_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .id_to_f252[0];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                id_to_f252_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .addr_to_id[1];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                addr_to_id_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .id_to_f252[1];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                id_to_f252_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .addr_to_id[2];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                addr_to_id_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .id_to_f252[2];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                id_to_f252_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[0];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[1];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[2];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[3];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[4];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[5];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[6];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[7];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[8];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[9];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[10];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[11];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[12];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[13];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[14];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[15];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[16];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[17];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[18];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[19];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[20];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[21];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[22];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[23];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[24];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[25];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[26];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_9_9[27];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_9_9_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[0];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[1];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[2];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[3];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[4];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[5];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[6];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[7];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[8];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[9];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[10];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[11];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[12];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[13];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[14];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[15];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[16];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[17];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[18];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[19];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[20];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[21];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[22];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[23];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[24];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[25];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[26];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .range_check_19[27];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                range_check_19_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .opcodes[0];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom =
                opcodes_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data
            .opcodes[1];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
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
            claimed_sum,
            total_sum,
        }
    }
}