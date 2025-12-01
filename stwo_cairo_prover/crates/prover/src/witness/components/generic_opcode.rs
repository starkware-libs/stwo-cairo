// This file was created by the AIR team.

#![allow(unused_parens)]#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::witness::prelude::*;
use cairo_air::components::generic_opcode::{Claim, InteractionClaim, N_TRACE_COLUMNS};
use crate::witness::components::memory_address_to_id;use crate::witness::components::memory_id_to_big;use crate::witness::components::verify_instruction;use crate::witness::components::range_check_9_9;use crate::witness::components::range_check_9_9_b;use crate::witness::components::range_check_9_9_c;use crate::witness::components::range_check_9_9_d;use crate::witness::components::range_check_9_9_e;use crate::witness::components::range_check_9_9_f;use crate::witness::components::range_check_9_9_g;use crate::witness::components::range_check_9_9_h;use crate::witness::components::range_check_20;use crate::witness::components::range_check_20_b;use crate::witness::components::range_check_20_c;use crate::witness::components::range_check_20_d;use crate::witness::components::range_check_20_e;use crate::witness::components::range_check_20_f;use crate::witness::components::range_check_20_g;use crate::witness::components::range_check_20_h;use crate::witness::components::range_check_18;use crate::witness::components::range_check_11;

pub type InputType = CasmState;
pub type PackedInputType = PackedCasmState;

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
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
verify_instruction_state: &verify_instruction::ClaimGenerator,
range_check_9_9_state: &range_check_9_9::ClaimGenerator,
range_check_9_9_b_state: &range_check_9_9_b::ClaimGenerator,
range_check_9_9_c_state: &range_check_9_9_c::ClaimGenerator,
range_check_9_9_d_state: &range_check_9_9_d::ClaimGenerator,
range_check_9_9_e_state: &range_check_9_9_e::ClaimGenerator,
range_check_9_9_f_state: &range_check_9_9_f::ClaimGenerator,
range_check_9_9_g_state: &range_check_9_9_g::ClaimGenerator,
range_check_9_9_h_state: &range_check_9_9_h::ClaimGenerator,
range_check_20_state: &range_check_20::ClaimGenerator,
range_check_20_b_state: &range_check_20_b::ClaimGenerator,
range_check_20_c_state: &range_check_20_c::ClaimGenerator,
range_check_20_d_state: &range_check_20_d::ClaimGenerator,
range_check_20_e_state: &range_check_20_e::ClaimGenerator,
range_check_20_f_state: &range_check_20_f::ClaimGenerator,
range_check_20_g_state: &range_check_20_g::ClaimGenerator,
range_check_20_h_state: &range_check_20_h::ClaimGenerator,
range_check_18_state: &range_check_18::ClaimGenerator,
range_check_11_state: &range_check_11::ClaimGenerator,

    ) -> (Claim, InteractionClaimGenerator)
    {
        let n_rows = self.inputs.len();
        assert_ne!(n_rows, 0);
        let size = std::cmp::max(n_rows.next_power_of_two(), N_LANES);
        let log_size = size.ilog2();
        self.inputs.resize(size, *self.inputs.first().unwrap());
        let packed_inputs = pack_values(&self.inputs);

        let (trace, lookup_data, sub_component_inputs) =
            write_trace_simd(packed_inputs,n_rows,memory_address_to_id_state,memory_id_to_big_state,verify_instruction_state,range_check_9_9_state,range_check_9_9_b_state,range_check_9_9_c_state,range_check_9_9_d_state,range_check_9_9_e_state,range_check_9_9_f_state,range_check_9_9_g_state,range_check_9_9_h_state,range_check_20_state,range_check_20_b_state,range_check_20_c_state,range_check_20_d_state,range_check_20_e_state,range_check_20_f_state,range_check_20_g_state,range_check_20_h_state,range_check_18_state,range_check_11_state,);
        sub_component_inputs.verify_instruction.iter().for_each(|inputs| {verify_instruction_state.add_packed_inputs(inputs);});sub_component_inputs.memory_address_to_id.iter().for_each(|inputs| {memory_address_to_id_state.add_packed_inputs(inputs);});sub_component_inputs.memory_id_to_big.iter().for_each(|inputs| {memory_id_to_big_state.add_packed_inputs(inputs);});sub_component_inputs.range_check_9_9.iter().for_each(|inputs| {range_check_9_9_state.add_packed_inputs(inputs);});sub_component_inputs.range_check_9_9_b.iter().for_each(|inputs| {range_check_9_9_b_state.add_packed_inputs(inputs);});sub_component_inputs.range_check_9_9_c.iter().for_each(|inputs| {range_check_9_9_c_state.add_packed_inputs(inputs);});sub_component_inputs.range_check_9_9_d.iter().for_each(|inputs| {range_check_9_9_d_state.add_packed_inputs(inputs);});sub_component_inputs.range_check_9_9_e.iter().for_each(|inputs| {range_check_9_9_e_state.add_packed_inputs(inputs);});sub_component_inputs.range_check_9_9_f.iter().for_each(|inputs| {range_check_9_9_f_state.add_packed_inputs(inputs);});sub_component_inputs.range_check_9_9_g.iter().for_each(|inputs| {range_check_9_9_g_state.add_packed_inputs(inputs);});sub_component_inputs.range_check_9_9_h.iter().for_each(|inputs| {range_check_9_9_h_state.add_packed_inputs(inputs);});sub_component_inputs.range_check_20.iter().for_each(|inputs| {range_check_20_state.add_packed_inputs(inputs);});sub_component_inputs.range_check_20_b.iter().for_each(|inputs| {range_check_20_b_state.add_packed_inputs(inputs);});sub_component_inputs.range_check_20_c.iter().for_each(|inputs| {range_check_20_c_state.add_packed_inputs(inputs);});sub_component_inputs.range_check_20_d.iter().for_each(|inputs| {range_check_20_d_state.add_packed_inputs(inputs);});sub_component_inputs.range_check_20_e.iter().for_each(|inputs| {range_check_20_e_state.add_packed_inputs(inputs);});sub_component_inputs.range_check_20_f.iter().for_each(|inputs| {range_check_20_f_state.add_packed_inputs(inputs);});sub_component_inputs.range_check_20_g.iter().for_each(|inputs| {range_check_20_g_state.add_packed_inputs(inputs);});sub_component_inputs.range_check_20_h.iter().for_each(|inputs| {range_check_20_h_state.add_packed_inputs(inputs);});sub_component_inputs.range_check_18.iter().for_each(|inputs| {range_check_18_state.add_packed_inputs(inputs);});sub_component_inputs.range_check_11.iter().for_each(|inputs| {range_check_11_state.add_packed_inputs(inputs);});
        tree_builder.extend_evals(trace.to_evals());

        (
        Claim {
            log_size,
        },
        InteractionClaimGenerator {
            n_rows,log_size,
            lookup_data,
        },
        )
    }
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct SubComponentInputs {
    verify_instruction: [Vec<verify_instruction::PackedInputType>; 1],memory_address_to_id: [Vec<memory_address_to_id::PackedInputType>; 3],memory_id_to_big: [Vec<memory_id_to_big::PackedInputType>; 3],range_check_9_9: [Vec<range_check_9_9::PackedInputType>; 4],range_check_9_9_b: [Vec<range_check_9_9_b::PackedInputType>; 4],range_check_9_9_c: [Vec<range_check_9_9_c::PackedInputType>; 4],range_check_9_9_d: [Vec<range_check_9_9_d::PackedInputType>; 4],range_check_9_9_e: [Vec<range_check_9_9_e::PackedInputType>; 4],range_check_9_9_f: [Vec<range_check_9_9_f::PackedInputType>; 4],range_check_9_9_g: [Vec<range_check_9_9_g::PackedInputType>; 2],range_check_9_9_h: [Vec<range_check_9_9_h::PackedInputType>; 2],range_check_20: [Vec<range_check_20::PackedInputType>; 4],range_check_20_b: [Vec<range_check_20_b::PackedInputType>; 4],range_check_20_c: [Vec<range_check_20_c::PackedInputType>; 4],range_check_20_d: [Vec<range_check_20_d::PackedInputType>; 4],range_check_20_e: [Vec<range_check_20_e::PackedInputType>; 3],range_check_20_f: [Vec<range_check_20_f::PackedInputType>; 3],range_check_20_g: [Vec<range_check_20_g::PackedInputType>; 3],range_check_20_h: [Vec<range_check_20_h::PackedInputType>; 3],range_check_18: [Vec<range_check_18::PackedInputType>; 1],range_check_11: [Vec<range_check_11::PackedInputType>; 1],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,n_rows: usize,memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
verify_instruction_state: &verify_instruction::ClaimGenerator,
range_check_9_9_state: &range_check_9_9::ClaimGenerator,
range_check_9_9_b_state: &range_check_9_9_b::ClaimGenerator,
range_check_9_9_c_state: &range_check_9_9_c::ClaimGenerator,
range_check_9_9_d_state: &range_check_9_9_d::ClaimGenerator,
range_check_9_9_e_state: &range_check_9_9_e::ClaimGenerator,
range_check_9_9_f_state: &range_check_9_9_f::ClaimGenerator,
range_check_9_9_g_state: &range_check_9_9_g::ClaimGenerator,
range_check_9_9_h_state: &range_check_9_9_h::ClaimGenerator,
range_check_20_state: &range_check_20::ClaimGenerator,
range_check_20_b_state: &range_check_20_b::ClaimGenerator,
range_check_20_c_state: &range_check_20_c::ClaimGenerator,
range_check_20_d_state: &range_check_20_d::ClaimGenerator,
range_check_20_e_state: &range_check_20_e::ClaimGenerator,
range_check_20_f_state: &range_check_20_f::ClaimGenerator,
range_check_20_g_state: &range_check_20_g::ClaimGenerator,
range_check_20_h_state: &range_check_20_h::ClaimGenerator,
range_check_18_state: &range_check_18::ClaimGenerator,
range_check_11_state: &range_check_11::ClaimGenerator,

) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData,SubComponentInputs,) {
    let log_n_packed_rows = inputs.len().ilog2();
    let log_size = log_n_packed_rows + LOG_N_LANES;
    let (mut trace, mut lookup_data,mut sub_component_inputs,) = unsafe {
        (ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
        LookupData::uninitialized(log_n_packed_rows),SubComponentInputs::uninitialized(log_n_packed_rows),)
    };

    let M31_0 = PackedM31::broadcast(
        M31::from(0)
    );let M31_1 = PackedM31::broadcast(
        M31::from(1)
    );let M31_1048576 = PackedM31::broadcast(
        M31::from(1048576)
    );let M31_128 = PackedM31::broadcast(
        M31::from(128)
    );let M31_134217728 = PackedM31::broadcast(
        M31::from(134217728)
    );let M31_136 = PackedM31::broadcast(
        M31::from(136)
    );let M31_16 = PackedM31::broadcast(
        M31::from(16)
    );let M31_2 = PackedM31::broadcast(
        M31::from(2)
    );let M31_256 = PackedM31::broadcast(
        M31::from(256)
    );let M31_262144 = PackedM31::broadcast(
        M31::from(262144)
    );let M31_32 = PackedM31::broadcast(
        M31::from(32)
    );let M31_32768 = PackedM31::broadcast(
        M31::from(32768)
    );let M31_4 = PackedM31::broadcast(
        M31::from(4)
    );let M31_4194304 = PackedM31::broadcast(
        M31::from(4194304)
    );let M31_508 = PackedM31::broadcast(
        M31::from(508)
    );let M31_511 = PackedM31::broadcast(
        M31::from(511)
    );let M31_512 = PackedM31::broadcast(
        M31::from(512)
    );let M31_524288 = PackedM31::broadcast(
        M31::from(524288)
    );let M31_536870912 = PackedM31::broadcast(
        M31::from(536870912)
    );let M31_64 = PackedM31::broadcast(
        M31::from(64)
    );let M31_65536 = PackedM31::broadcast(
        M31::from(65536)
    );let M31_8 = PackedM31::broadcast(
        M31::from(8)
    );let UInt16_0 = PackedUInt16::broadcast(
        UInt16::from(0)
    );let UInt16_1 = PackedUInt16::broadcast(
        UInt16::from(1)
    );let UInt16_10 = PackedUInt16::broadcast(
        UInt16::from(10)
    );let UInt16_11 = PackedUInt16::broadcast(
        UInt16::from(11)
    );let UInt16_12 = PackedUInt16::broadcast(
        UInt16::from(12)
    );let UInt16_127 = PackedUInt16::broadcast(
        UInt16::from(127)
    );let UInt16_13 = PackedUInt16::broadcast(
        UInt16::from(13)
    );let UInt16_14 = PackedUInt16::broadcast(
        UInt16::from(14)
    );let UInt16_2 = PackedUInt16::broadcast(
        UInt16::from(2)
    );let UInt16_3 = PackedUInt16::broadcast(
        UInt16::from(3)
    );let UInt16_31 = PackedUInt16::broadcast(
        UInt16::from(31)
    );let UInt16_4 = PackedUInt16::broadcast(
        UInt16::from(4)
    );let UInt16_5 = PackedUInt16::broadcast(
        UInt16::from(5)
    );let UInt16_6 = PackedUInt16::broadcast(
        UInt16::from(6)
    );let UInt16_7 = PackedUInt16::broadcast(
        UInt16::from(7)
    );let UInt16_8 = PackedUInt16::broadcast(
        UInt16::from(8)
    );let UInt16_9 = PackedUInt16::broadcast(
        UInt16::from(9)
    );let UInt32_131072 = PackedUInt32::broadcast(
        UInt32::from(131072)
    );let UInt32_2047 = PackedUInt32::broadcast(
        UInt32::from(2047)
    );let UInt32_262143 = PackedUInt32::broadcast(
        UInt32::from(262143)
    );let UInt32_511 = PackedUInt32::broadcast(
        UInt32::from(511)
    );let UInt32_9 = PackedUInt32::broadcast(
        UInt32::from(9)
    );
    let enabler_col = Enabler::new(n_rows);

    (trace.par_iter_mut(),
    lookup_data.par_iter_mut(),sub_component_inputs.par_iter_mut(),inputs.into_par_iter(),)
    .into_par_iter()
    .enumerate()
    .for_each(
        |(row_index,(row, lookup_data,sub_component_inputs,generic_opcode_input,))| {
            let input_pc_col0 = generic_opcode_input.pc;
            *row[0] = input_pc_col0;let input_ap_col1 = generic_opcode_input.ap;
            *row[1] = input_ap_col1;let input_fp_col2 = generic_opcode_input.fp;
            *row[2] = input_fp_col2;

            // Decode Generic Instruction.

            // Decode Instruction.

            let memory_address_to_id_value_tmp_57455_0 = memory_address_to_id_state.deduce_output(input_pc_col0);let memory_id_to_big_value_tmp_57455_1 = memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_57455_0);let offset0_tmp_57455_2 = ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(0))) + (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(1))) & (UInt16_127))) << (UInt16_9))));let offset0_col3 = offset0_tmp_57455_2.as_m31();
            *row[3] = offset0_col3;let offset1_tmp_57455_3 = ((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(1))) >> (UInt16_7))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(2))) << (UInt16_2))))) + (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(3))) & (UInt16_31))) << (UInt16_11))));let offset1_col4 = offset1_tmp_57455_3.as_m31();
            *row[4] = offset1_col4;let offset2_tmp_57455_4 = ((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(3))) >> (UInt16_5))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(4))) << (UInt16_4))))) + (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(5))) & (UInt16_7))) << (UInt16_13))));let offset2_col5 = offset2_tmp_57455_4.as_m31();
            *row[5] = offset2_col5;let dst_base_fp_tmp_57455_5 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(6))) << (UInt16_6))))) >> (UInt16_0))) & (UInt16_1));let dst_base_fp_col6 = dst_base_fp_tmp_57455_5.as_m31();
            *row[6] = dst_base_fp_col6;let op0_base_fp_tmp_57455_6 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(6))) << (UInt16_6))))) >> (UInt16_1))) & (UInt16_1));let op0_base_fp_col7 = op0_base_fp_tmp_57455_6.as_m31();
            *row[7] = op0_base_fp_col7;let op1_imm_tmp_57455_7 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(6))) << (UInt16_6))))) >> (UInt16_2))) & (UInt16_1));let op1_imm_col8 = op1_imm_tmp_57455_7.as_m31();
            *row[8] = op1_imm_col8;let op1_base_fp_tmp_57455_8 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(6))) << (UInt16_6))))) >> (UInt16_3))) & (UInt16_1));let op1_base_fp_col9 = op1_base_fp_tmp_57455_8.as_m31();
            *row[9] = op1_base_fp_col9;let op1_base_ap_tmp_57455_9 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(6))) << (UInt16_6))))) >> (UInt16_4))) & (UInt16_1));let op1_base_ap_col10 = op1_base_ap_tmp_57455_9.as_m31();
            *row[10] = op1_base_ap_col10;let res_add_tmp_57455_10 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(6))) << (UInt16_6))))) >> (UInt16_5))) & (UInt16_1));let res_add_col11 = res_add_tmp_57455_10.as_m31();
            *row[11] = res_add_col11;let res_mul_tmp_57455_11 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(6))) << (UInt16_6))))) >> (UInt16_6))) & (UInt16_1));let res_mul_col12 = res_mul_tmp_57455_11.as_m31();
            *row[12] = res_mul_col12;let pc_update_jump_tmp_57455_12 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(6))) << (UInt16_6))))) >> (UInt16_7))) & (UInt16_1));let pc_update_jump_col13 = pc_update_jump_tmp_57455_12.as_m31();
            *row[13] = pc_update_jump_col13;let pc_update_jump_rel_tmp_57455_13 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(6))) << (UInt16_6))))) >> (UInt16_8))) & (UInt16_1));let pc_update_jump_rel_col14 = pc_update_jump_rel_tmp_57455_13.as_m31();
            *row[14] = pc_update_jump_rel_col14;let pc_update_jnz_tmp_57455_14 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(6))) << (UInt16_6))))) >> (UInt16_9))) & (UInt16_1));let pc_update_jnz_col15 = pc_update_jnz_tmp_57455_14.as_m31();
            *row[15] = pc_update_jnz_col15;let ap_update_add_tmp_57455_15 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(6))) << (UInt16_6))))) >> (UInt16_10))) & (UInt16_1));let ap_update_add_col16 = ap_update_add_tmp_57455_15.as_m31();
            *row[16] = ap_update_add_col16;let ap_update_add_1_tmp_57455_16 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(6))) << (UInt16_6))))) >> (UInt16_11))) & (UInt16_1));let ap_update_add_1_col17 = ap_update_add_1_tmp_57455_16.as_m31();
            *row[17] = ap_update_add_1_col17;let opcode_call_tmp_57455_17 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(6))) << (UInt16_6))))) >> (UInt16_12))) & (UInt16_1));let opcode_call_col18 = opcode_call_tmp_57455_17.as_m31();
            *row[18] = opcode_call_col18;let opcode_ret_tmp_57455_18 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(6))) << (UInt16_6))))) >> (UInt16_13))) & (UInt16_1));let opcode_ret_col19 = opcode_ret_tmp_57455_18.as_m31();
            *row[19] = opcode_ret_col19;let opcode_assert_eq_tmp_57455_19 = ((((((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(5))) >> (UInt16_3))) + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_57455_1.get_m31(6))) << (UInt16_6))))) >> (UInt16_14))) & (UInt16_1));let opcode_assert_eq_col20 = opcode_assert_eq_tmp_57455_19.as_m31();
            *row[20] = opcode_assert_eq_col20;*sub_component_inputs.verify_instruction[0] =
                (input_pc_col0, [offset0_col3, offset1_col4, offset2_col5], [((((((((((((dst_base_fp_col6) * (M31_8))) + (((op0_base_fp_col7) * (M31_16))))) + (((op1_imm_col8) * (M31_32))))) + (((op1_base_fp_col9) * (M31_64))))) + (((op1_base_ap_col10) * (M31_128))))) + (((res_add_col11) * (M31_256)))), ((((((((((((((((res_mul_col12) + (((pc_update_jump_col13) * (M31_2))))) + (((pc_update_jump_rel_col14) * (M31_4))))) + (((pc_update_jnz_col15) * (M31_8))))) + (((ap_update_add_col16) * (M31_16))))) + (((ap_update_add_1_col17) * (M31_32))))) + (((opcode_call_col18) * (M31_64))))) + (((opcode_ret_col19) * (M31_128))))) + (((opcode_assert_eq_col20) * (M31_256))))], M31_0);
            *lookup_data.verify_instruction_0 = [input_pc_col0, offset0_col3, offset1_col4, offset2_col5, ((((((((((((dst_base_fp_col6) * (M31_8))) + (((op0_base_fp_col7) * (M31_16))))) + (((op1_imm_col8) * (M31_32))))) + (((op1_base_fp_col9) * (M31_64))))) + (((op1_base_ap_col10) * (M31_128))))) + (((res_add_col11) * (M31_256)))), ((((((((((((((((res_mul_col12) + (((pc_update_jump_col13) * (M31_2))))) + (((pc_update_jump_rel_col14) * (M31_4))))) + (((pc_update_jnz_col15) * (M31_8))))) + (((ap_update_add_col16) * (M31_16))))) + (((ap_update_add_1_col17) * (M31_32))))) + (((opcode_call_col18) * (M31_64))))) + (((opcode_ret_col19) * (M31_128))))) + (((opcode_assert_eq_col20) * (M31_256)))), M31_0];let decode_instruction_df7a6_output_tmp_57455_20 = ([((offset0_col3) - (M31_32768)), ((offset1_col4) - (M31_32768)), ((offset2_col5) - (M31_32768))], [dst_base_fp_col6, op0_base_fp_col7, op1_imm_col8, op1_base_fp_col9, op1_base_ap_col10, res_add_col11, res_mul_col12, pc_update_jump_col13, pc_update_jump_rel_col14, pc_update_jnz_col15, ap_update_add_col16, ap_update_add_1_col17, opcode_call_col18, opcode_ret_col19, opcode_assert_eq_col20], M31_0);

            let op1_base_op0_tmp_57455_21 = ((((((M31_1) - (op1_imm_col8))) - (op1_base_fp_col9))) - (op1_base_ap_col10));let res_op1_tmp_57455_22 = ((((((M31_1) - (res_add_col11))) - (res_mul_col12))) - (pc_update_jnz_col15));let pc_update_regular_tmp_57455_23 = ((((((M31_1) - (pc_update_jump_col13))) - (pc_update_jump_rel_col14))) - (pc_update_jnz_col15));let ap_update_regular_tmp_57455_24 = ((((((M31_1) - (ap_update_add_col16))) - (ap_update_add_1_col17))) - (opcode_call_col18));let fp_update_regular_tmp_57455_25 = ((((M31_1) - (opcode_call_col18))) - (opcode_ret_col19));let decode_generic_instruction_output_tmp_57455_26 = ([dst_base_fp_col6, op0_base_fp_col7, op1_imm_col8, op1_base_fp_col9, op1_base_ap_col10, res_add_col11, res_mul_col12, pc_update_jump_col13, pc_update_jump_rel_col14, pc_update_jnz_col15, ap_update_add_col16, ap_update_add_1_col17, opcode_call_col18, opcode_ret_col19, opcode_assert_eq_col20, op1_base_op0_tmp_57455_21, res_op1_tmp_57455_22, pc_update_regular_tmp_57455_23, fp_update_regular_tmp_57455_25, ((M31_1) + (op1_imm_col8))], [decode_instruction_df7a6_output_tmp_57455_20.0[0], decode_instruction_df7a6_output_tmp_57455_20.0[1], decode_instruction_df7a6_output_tmp_57455_20.0[2]]);

            // Eval Operands.

            let dst_src_col21 = ((((dst_base_fp_col6) * (input_fp_col2))) + (((((M31_1) - (dst_base_fp_col6))) * (input_ap_col1))));
            *row[21] = dst_src_col21;

            // Read Positive Num Bits 252.

            // Read Id.

            let memory_address_to_id_value_tmp_57455_27 = memory_address_to_id_state.deduce_output(((dst_src_col21) + (decode_generic_instruction_output_tmp_57455_26.1[0])));let dst_id_col22 = memory_address_to_id_value_tmp_57455_27;
            *row[22] = dst_id_col22;*sub_component_inputs.memory_address_to_id[0] =
                ((dst_src_col21) + (decode_generic_instruction_output_tmp_57455_26.1[0]));
            *lookup_data.memory_address_to_id_0 = [((dst_src_col21) + (decode_generic_instruction_output_tmp_57455_26.1[0])), dst_id_col22];

            // Read Positive Known Id Num Bits 252.

            let memory_id_to_big_value_tmp_57455_29 = memory_id_to_big_state.deduce_output(dst_id_col22);let dst_limb_0_col23 = memory_id_to_big_value_tmp_57455_29.get_m31(0);
            *row[23] = dst_limb_0_col23;let dst_limb_1_col24 = memory_id_to_big_value_tmp_57455_29.get_m31(1);
            *row[24] = dst_limb_1_col24;let dst_limb_2_col25 = memory_id_to_big_value_tmp_57455_29.get_m31(2);
            *row[25] = dst_limb_2_col25;let dst_limb_3_col26 = memory_id_to_big_value_tmp_57455_29.get_m31(3);
            *row[26] = dst_limb_3_col26;let dst_limb_4_col27 = memory_id_to_big_value_tmp_57455_29.get_m31(4);
            *row[27] = dst_limb_4_col27;let dst_limb_5_col28 = memory_id_to_big_value_tmp_57455_29.get_m31(5);
            *row[28] = dst_limb_5_col28;let dst_limb_6_col29 = memory_id_to_big_value_tmp_57455_29.get_m31(6);
            *row[29] = dst_limb_6_col29;let dst_limb_7_col30 = memory_id_to_big_value_tmp_57455_29.get_m31(7);
            *row[30] = dst_limb_7_col30;let dst_limb_8_col31 = memory_id_to_big_value_tmp_57455_29.get_m31(8);
            *row[31] = dst_limb_8_col31;let dst_limb_9_col32 = memory_id_to_big_value_tmp_57455_29.get_m31(9);
            *row[32] = dst_limb_9_col32;let dst_limb_10_col33 = memory_id_to_big_value_tmp_57455_29.get_m31(10);
            *row[33] = dst_limb_10_col33;let dst_limb_11_col34 = memory_id_to_big_value_tmp_57455_29.get_m31(11);
            *row[34] = dst_limb_11_col34;let dst_limb_12_col35 = memory_id_to_big_value_tmp_57455_29.get_m31(12);
            *row[35] = dst_limb_12_col35;let dst_limb_13_col36 = memory_id_to_big_value_tmp_57455_29.get_m31(13);
            *row[36] = dst_limb_13_col36;let dst_limb_14_col37 = memory_id_to_big_value_tmp_57455_29.get_m31(14);
            *row[37] = dst_limb_14_col37;let dst_limb_15_col38 = memory_id_to_big_value_tmp_57455_29.get_m31(15);
            *row[38] = dst_limb_15_col38;let dst_limb_16_col39 = memory_id_to_big_value_tmp_57455_29.get_m31(16);
            *row[39] = dst_limb_16_col39;let dst_limb_17_col40 = memory_id_to_big_value_tmp_57455_29.get_m31(17);
            *row[40] = dst_limb_17_col40;let dst_limb_18_col41 = memory_id_to_big_value_tmp_57455_29.get_m31(18);
            *row[41] = dst_limb_18_col41;let dst_limb_19_col42 = memory_id_to_big_value_tmp_57455_29.get_m31(19);
            *row[42] = dst_limb_19_col42;let dst_limb_20_col43 = memory_id_to_big_value_tmp_57455_29.get_m31(20);
            *row[43] = dst_limb_20_col43;let dst_limb_21_col44 = memory_id_to_big_value_tmp_57455_29.get_m31(21);
            *row[44] = dst_limb_21_col44;let dst_limb_22_col45 = memory_id_to_big_value_tmp_57455_29.get_m31(22);
            *row[45] = dst_limb_22_col45;let dst_limb_23_col46 = memory_id_to_big_value_tmp_57455_29.get_m31(23);
            *row[46] = dst_limb_23_col46;let dst_limb_24_col47 = memory_id_to_big_value_tmp_57455_29.get_m31(24);
            *row[47] = dst_limb_24_col47;let dst_limb_25_col48 = memory_id_to_big_value_tmp_57455_29.get_m31(25);
            *row[48] = dst_limb_25_col48;let dst_limb_26_col49 = memory_id_to_big_value_tmp_57455_29.get_m31(26);
            *row[49] = dst_limb_26_col49;let dst_limb_27_col50 = memory_id_to_big_value_tmp_57455_29.get_m31(27);
            *row[50] = dst_limb_27_col50;*sub_component_inputs.memory_id_to_big[0] =
                dst_id_col22;
            *lookup_data.memory_id_to_big_0 = [dst_id_col22, dst_limb_0_col23, dst_limb_1_col24, dst_limb_2_col25, dst_limb_3_col26, dst_limb_4_col27, dst_limb_5_col28, dst_limb_6_col29, dst_limb_7_col30, dst_limb_8_col31, dst_limb_9_col32, dst_limb_10_col33, dst_limb_11_col34, dst_limb_12_col35, dst_limb_13_col36, dst_limb_14_col37, dst_limb_15_col38, dst_limb_16_col39, dst_limb_17_col40, dst_limb_18_col41, dst_limb_19_col42, dst_limb_20_col43, dst_limb_21_col44, dst_limb_22_col45, dst_limb_23_col46, dst_limb_24_col47, dst_limb_25_col48, dst_limb_26_col49, dst_limb_27_col50];let read_positive_known_id_num_bits_252_output_tmp_57455_30 = PackedFelt252::from_limbs([dst_limb_0_col23, dst_limb_1_col24, dst_limb_2_col25, dst_limb_3_col26, dst_limb_4_col27, dst_limb_5_col28, dst_limb_6_col29, dst_limb_7_col30, dst_limb_8_col31, dst_limb_9_col32, dst_limb_10_col33, dst_limb_11_col34, dst_limb_12_col35, dst_limb_13_col36, dst_limb_14_col37, dst_limb_15_col38, dst_limb_16_col39, dst_limb_17_col40, dst_limb_18_col41, dst_limb_19_col42, dst_limb_20_col43, dst_limb_21_col44, dst_limb_22_col45, dst_limb_23_col46, dst_limb_24_col47, dst_limb_25_col48, dst_limb_26_col49, dst_limb_27_col50]);

            let read_positive_num_bits_252_output_tmp_57455_31 = (read_positive_known_id_num_bits_252_output_tmp_57455_30, dst_id_col22);

            let op0_src_col51 = ((((op0_base_fp_col7) * (input_fp_col2))) + (((((M31_1) - (op0_base_fp_col7))) * (input_ap_col1))));
            *row[51] = op0_src_col51;

            // Read Positive Num Bits 252.

            // Read Id.

            let memory_address_to_id_value_tmp_57455_32 = memory_address_to_id_state.deduce_output(((op0_src_col51) + (decode_generic_instruction_output_tmp_57455_26.1[1])));let op0_id_col52 = memory_address_to_id_value_tmp_57455_32;
            *row[52] = op0_id_col52;*sub_component_inputs.memory_address_to_id[1] =
                ((op0_src_col51) + (decode_generic_instruction_output_tmp_57455_26.1[1]));
            *lookup_data.memory_address_to_id_1 = [((op0_src_col51) + (decode_generic_instruction_output_tmp_57455_26.1[1])), op0_id_col52];

            // Read Positive Known Id Num Bits 252.

            let memory_id_to_big_value_tmp_57455_34 = memory_id_to_big_state.deduce_output(op0_id_col52);let op0_limb_0_col53 = memory_id_to_big_value_tmp_57455_34.get_m31(0);
            *row[53] = op0_limb_0_col53;let op0_limb_1_col54 = memory_id_to_big_value_tmp_57455_34.get_m31(1);
            *row[54] = op0_limb_1_col54;let op0_limb_2_col55 = memory_id_to_big_value_tmp_57455_34.get_m31(2);
            *row[55] = op0_limb_2_col55;let op0_limb_3_col56 = memory_id_to_big_value_tmp_57455_34.get_m31(3);
            *row[56] = op0_limb_3_col56;let op0_limb_4_col57 = memory_id_to_big_value_tmp_57455_34.get_m31(4);
            *row[57] = op0_limb_4_col57;let op0_limb_5_col58 = memory_id_to_big_value_tmp_57455_34.get_m31(5);
            *row[58] = op0_limb_5_col58;let op0_limb_6_col59 = memory_id_to_big_value_tmp_57455_34.get_m31(6);
            *row[59] = op0_limb_6_col59;let op0_limb_7_col60 = memory_id_to_big_value_tmp_57455_34.get_m31(7);
            *row[60] = op0_limb_7_col60;let op0_limb_8_col61 = memory_id_to_big_value_tmp_57455_34.get_m31(8);
            *row[61] = op0_limb_8_col61;let op0_limb_9_col62 = memory_id_to_big_value_tmp_57455_34.get_m31(9);
            *row[62] = op0_limb_9_col62;let op0_limb_10_col63 = memory_id_to_big_value_tmp_57455_34.get_m31(10);
            *row[63] = op0_limb_10_col63;let op0_limb_11_col64 = memory_id_to_big_value_tmp_57455_34.get_m31(11);
            *row[64] = op0_limb_11_col64;let op0_limb_12_col65 = memory_id_to_big_value_tmp_57455_34.get_m31(12);
            *row[65] = op0_limb_12_col65;let op0_limb_13_col66 = memory_id_to_big_value_tmp_57455_34.get_m31(13);
            *row[66] = op0_limb_13_col66;let op0_limb_14_col67 = memory_id_to_big_value_tmp_57455_34.get_m31(14);
            *row[67] = op0_limb_14_col67;let op0_limb_15_col68 = memory_id_to_big_value_tmp_57455_34.get_m31(15);
            *row[68] = op0_limb_15_col68;let op0_limb_16_col69 = memory_id_to_big_value_tmp_57455_34.get_m31(16);
            *row[69] = op0_limb_16_col69;let op0_limb_17_col70 = memory_id_to_big_value_tmp_57455_34.get_m31(17);
            *row[70] = op0_limb_17_col70;let op0_limb_18_col71 = memory_id_to_big_value_tmp_57455_34.get_m31(18);
            *row[71] = op0_limb_18_col71;let op0_limb_19_col72 = memory_id_to_big_value_tmp_57455_34.get_m31(19);
            *row[72] = op0_limb_19_col72;let op0_limb_20_col73 = memory_id_to_big_value_tmp_57455_34.get_m31(20);
            *row[73] = op0_limb_20_col73;let op0_limb_21_col74 = memory_id_to_big_value_tmp_57455_34.get_m31(21);
            *row[74] = op0_limb_21_col74;let op0_limb_22_col75 = memory_id_to_big_value_tmp_57455_34.get_m31(22);
            *row[75] = op0_limb_22_col75;let op0_limb_23_col76 = memory_id_to_big_value_tmp_57455_34.get_m31(23);
            *row[76] = op0_limb_23_col76;let op0_limb_24_col77 = memory_id_to_big_value_tmp_57455_34.get_m31(24);
            *row[77] = op0_limb_24_col77;let op0_limb_25_col78 = memory_id_to_big_value_tmp_57455_34.get_m31(25);
            *row[78] = op0_limb_25_col78;let op0_limb_26_col79 = memory_id_to_big_value_tmp_57455_34.get_m31(26);
            *row[79] = op0_limb_26_col79;let op0_limb_27_col80 = memory_id_to_big_value_tmp_57455_34.get_m31(27);
            *row[80] = op0_limb_27_col80;*sub_component_inputs.memory_id_to_big[1] =
                op0_id_col52;
            *lookup_data.memory_id_to_big_1 = [op0_id_col52, op0_limb_0_col53, op0_limb_1_col54, op0_limb_2_col55, op0_limb_3_col56, op0_limb_4_col57, op0_limb_5_col58, op0_limb_6_col59, op0_limb_7_col60, op0_limb_8_col61, op0_limb_9_col62, op0_limb_10_col63, op0_limb_11_col64, op0_limb_12_col65, op0_limb_13_col66, op0_limb_14_col67, op0_limb_15_col68, op0_limb_16_col69, op0_limb_17_col70, op0_limb_18_col71, op0_limb_19_col72, op0_limb_20_col73, op0_limb_21_col74, op0_limb_22_col75, op0_limb_23_col76, op0_limb_24_col77, op0_limb_25_col78, op0_limb_26_col79, op0_limb_27_col80];let read_positive_known_id_num_bits_252_output_tmp_57455_35 = PackedFelt252::from_limbs([op0_limb_0_col53, op0_limb_1_col54, op0_limb_2_col55, op0_limb_3_col56, op0_limb_4_col57, op0_limb_5_col58, op0_limb_6_col59, op0_limb_7_col60, op0_limb_8_col61, op0_limb_9_col62, op0_limb_10_col63, op0_limb_11_col64, op0_limb_12_col65, op0_limb_13_col66, op0_limb_14_col67, op0_limb_15_col68, op0_limb_16_col69, op0_limb_17_col70, op0_limb_18_col71, op0_limb_19_col72, op0_limb_20_col73, op0_limb_21_col74, op0_limb_22_col75, op0_limb_23_col76, op0_limb_24_col77, op0_limb_25_col78, op0_limb_26_col79, op0_limb_27_col80]);

            let read_positive_num_bits_252_output_tmp_57455_36 = (read_positive_known_id_num_bits_252_output_tmp_57455_35, op0_id_col52);

            // Cond Felt 252 As Addr.

            // Cond Range Check 2.

            let partial_limb_msb_tmp_57455_37 = ((((PackedUInt16::from_m31(op0_limb_3_col56)) & (UInt16_2))) >> (UInt16_1));let partial_limb_msb_col81 = partial_limb_msb_tmp_57455_37.as_m31();
            *row[81] = partial_limb_msb_col81;

            let cond_felt_252_as_addr_output_tmp_57455_39 = ((((((op0_limb_0_col53) + (((op0_limb_1_col54) * (M31_512))))) + (((op0_limb_2_col55) * (M31_262144))))) + (((op0_limb_3_col56) * (M31_134217728))));

            let op1_src_col82 = ((((((((op1_base_fp_col9) * (input_fp_col2))) + (((op1_base_ap_col10) * (input_ap_col1))))) + (((op1_imm_col8) * (input_pc_col0))))) + (((decode_generic_instruction_output_tmp_57455_26.0[15]) * (cond_felt_252_as_addr_output_tmp_57455_39))));
            *row[82] = op1_src_col82;

            // Read Positive Num Bits 252.

            // Read Id.

            let memory_address_to_id_value_tmp_57455_40 = memory_address_to_id_state.deduce_output(((op1_src_col82) + (decode_generic_instruction_output_tmp_57455_26.1[2])));let op1_id_col83 = memory_address_to_id_value_tmp_57455_40;
            *row[83] = op1_id_col83;*sub_component_inputs.memory_address_to_id[2] =
                ((op1_src_col82) + (decode_generic_instruction_output_tmp_57455_26.1[2]));
            *lookup_data.memory_address_to_id_2 = [((op1_src_col82) + (decode_generic_instruction_output_tmp_57455_26.1[2])), op1_id_col83];

            // Read Positive Known Id Num Bits 252.

            let memory_id_to_big_value_tmp_57455_42 = memory_id_to_big_state.deduce_output(op1_id_col83);let op1_limb_0_col84 = memory_id_to_big_value_tmp_57455_42.get_m31(0);
            *row[84] = op1_limb_0_col84;let op1_limb_1_col85 = memory_id_to_big_value_tmp_57455_42.get_m31(1);
            *row[85] = op1_limb_1_col85;let op1_limb_2_col86 = memory_id_to_big_value_tmp_57455_42.get_m31(2);
            *row[86] = op1_limb_2_col86;let op1_limb_3_col87 = memory_id_to_big_value_tmp_57455_42.get_m31(3);
            *row[87] = op1_limb_3_col87;let op1_limb_4_col88 = memory_id_to_big_value_tmp_57455_42.get_m31(4);
            *row[88] = op1_limb_4_col88;let op1_limb_5_col89 = memory_id_to_big_value_tmp_57455_42.get_m31(5);
            *row[89] = op1_limb_5_col89;let op1_limb_6_col90 = memory_id_to_big_value_tmp_57455_42.get_m31(6);
            *row[90] = op1_limb_6_col90;let op1_limb_7_col91 = memory_id_to_big_value_tmp_57455_42.get_m31(7);
            *row[91] = op1_limb_7_col91;let op1_limb_8_col92 = memory_id_to_big_value_tmp_57455_42.get_m31(8);
            *row[92] = op1_limb_8_col92;let op1_limb_9_col93 = memory_id_to_big_value_tmp_57455_42.get_m31(9);
            *row[93] = op1_limb_9_col93;let op1_limb_10_col94 = memory_id_to_big_value_tmp_57455_42.get_m31(10);
            *row[94] = op1_limb_10_col94;let op1_limb_11_col95 = memory_id_to_big_value_tmp_57455_42.get_m31(11);
            *row[95] = op1_limb_11_col95;let op1_limb_12_col96 = memory_id_to_big_value_tmp_57455_42.get_m31(12);
            *row[96] = op1_limb_12_col96;let op1_limb_13_col97 = memory_id_to_big_value_tmp_57455_42.get_m31(13);
            *row[97] = op1_limb_13_col97;let op1_limb_14_col98 = memory_id_to_big_value_tmp_57455_42.get_m31(14);
            *row[98] = op1_limb_14_col98;let op1_limb_15_col99 = memory_id_to_big_value_tmp_57455_42.get_m31(15);
            *row[99] = op1_limb_15_col99;let op1_limb_16_col100 = memory_id_to_big_value_tmp_57455_42.get_m31(16);
            *row[100] = op1_limb_16_col100;let op1_limb_17_col101 = memory_id_to_big_value_tmp_57455_42.get_m31(17);
            *row[101] = op1_limb_17_col101;let op1_limb_18_col102 = memory_id_to_big_value_tmp_57455_42.get_m31(18);
            *row[102] = op1_limb_18_col102;let op1_limb_19_col103 = memory_id_to_big_value_tmp_57455_42.get_m31(19);
            *row[103] = op1_limb_19_col103;let op1_limb_20_col104 = memory_id_to_big_value_tmp_57455_42.get_m31(20);
            *row[104] = op1_limb_20_col104;let op1_limb_21_col105 = memory_id_to_big_value_tmp_57455_42.get_m31(21);
            *row[105] = op1_limb_21_col105;let op1_limb_22_col106 = memory_id_to_big_value_tmp_57455_42.get_m31(22);
            *row[106] = op1_limb_22_col106;let op1_limb_23_col107 = memory_id_to_big_value_tmp_57455_42.get_m31(23);
            *row[107] = op1_limb_23_col107;let op1_limb_24_col108 = memory_id_to_big_value_tmp_57455_42.get_m31(24);
            *row[108] = op1_limb_24_col108;let op1_limb_25_col109 = memory_id_to_big_value_tmp_57455_42.get_m31(25);
            *row[109] = op1_limb_25_col109;let op1_limb_26_col110 = memory_id_to_big_value_tmp_57455_42.get_m31(26);
            *row[110] = op1_limb_26_col110;let op1_limb_27_col111 = memory_id_to_big_value_tmp_57455_42.get_m31(27);
            *row[111] = op1_limb_27_col111;*sub_component_inputs.memory_id_to_big[2] =
                op1_id_col83;
            *lookup_data.memory_id_to_big_2 = [op1_id_col83, op1_limb_0_col84, op1_limb_1_col85, op1_limb_2_col86, op1_limb_3_col87, op1_limb_4_col88, op1_limb_5_col89, op1_limb_6_col90, op1_limb_7_col91, op1_limb_8_col92, op1_limb_9_col93, op1_limb_10_col94, op1_limb_11_col95, op1_limb_12_col96, op1_limb_13_col97, op1_limb_14_col98, op1_limb_15_col99, op1_limb_16_col100, op1_limb_17_col101, op1_limb_18_col102, op1_limb_19_col103, op1_limb_20_col104, op1_limb_21_col105, op1_limb_22_col106, op1_limb_23_col107, op1_limb_24_col108, op1_limb_25_col109, op1_limb_26_col110, op1_limb_27_col111];let read_positive_known_id_num_bits_252_output_tmp_57455_43 = PackedFelt252::from_limbs([op1_limb_0_col84, op1_limb_1_col85, op1_limb_2_col86, op1_limb_3_col87, op1_limb_4_col88, op1_limb_5_col89, op1_limb_6_col90, op1_limb_7_col91, op1_limb_8_col92, op1_limb_9_col93, op1_limb_10_col94, op1_limb_11_col95, op1_limb_12_col96, op1_limb_13_col97, op1_limb_14_col98, op1_limb_15_col99, op1_limb_16_col100, op1_limb_17_col101, op1_limb_18_col102, op1_limb_19_col103, op1_limb_20_col104, op1_limb_21_col105, op1_limb_22_col106, op1_limb_23_col107, op1_limb_24_col108, op1_limb_25_col109, op1_limb_26_col110, op1_limb_27_col111]);

            let read_positive_num_bits_252_output_tmp_57455_44 = (read_positive_known_id_num_bits_252_output_tmp_57455_43, op1_id_col83);

            // Add 252.

            let add_res_tmp_57455_45 = ((read_positive_num_bits_252_output_tmp_57455_36.0) + (read_positive_num_bits_252_output_tmp_57455_44.0));let add_res_limb_0_col112 = add_res_tmp_57455_45.get_m31(0);
            *row[112] = add_res_limb_0_col112;let add_res_limb_1_col113 = add_res_tmp_57455_45.get_m31(1);
            *row[113] = add_res_limb_1_col113;let add_res_limb_2_col114 = add_res_tmp_57455_45.get_m31(2);
            *row[114] = add_res_limb_2_col114;let add_res_limb_3_col115 = add_res_tmp_57455_45.get_m31(3);
            *row[115] = add_res_limb_3_col115;let add_res_limb_4_col116 = add_res_tmp_57455_45.get_m31(4);
            *row[116] = add_res_limb_4_col116;let add_res_limb_5_col117 = add_res_tmp_57455_45.get_m31(5);
            *row[117] = add_res_limb_5_col117;let add_res_limb_6_col118 = add_res_tmp_57455_45.get_m31(6);
            *row[118] = add_res_limb_6_col118;let add_res_limb_7_col119 = add_res_tmp_57455_45.get_m31(7);
            *row[119] = add_res_limb_7_col119;let add_res_limb_8_col120 = add_res_tmp_57455_45.get_m31(8);
            *row[120] = add_res_limb_8_col120;let add_res_limb_9_col121 = add_res_tmp_57455_45.get_m31(9);
            *row[121] = add_res_limb_9_col121;let add_res_limb_10_col122 = add_res_tmp_57455_45.get_m31(10);
            *row[122] = add_res_limb_10_col122;let add_res_limb_11_col123 = add_res_tmp_57455_45.get_m31(11);
            *row[123] = add_res_limb_11_col123;let add_res_limb_12_col124 = add_res_tmp_57455_45.get_m31(12);
            *row[124] = add_res_limb_12_col124;let add_res_limb_13_col125 = add_res_tmp_57455_45.get_m31(13);
            *row[125] = add_res_limb_13_col125;let add_res_limb_14_col126 = add_res_tmp_57455_45.get_m31(14);
            *row[126] = add_res_limb_14_col126;let add_res_limb_15_col127 = add_res_tmp_57455_45.get_m31(15);
            *row[127] = add_res_limb_15_col127;let add_res_limb_16_col128 = add_res_tmp_57455_45.get_m31(16);
            *row[128] = add_res_limb_16_col128;let add_res_limb_17_col129 = add_res_tmp_57455_45.get_m31(17);
            *row[129] = add_res_limb_17_col129;let add_res_limb_18_col130 = add_res_tmp_57455_45.get_m31(18);
            *row[130] = add_res_limb_18_col130;let add_res_limb_19_col131 = add_res_tmp_57455_45.get_m31(19);
            *row[131] = add_res_limb_19_col131;let add_res_limb_20_col132 = add_res_tmp_57455_45.get_m31(20);
            *row[132] = add_res_limb_20_col132;let add_res_limb_21_col133 = add_res_tmp_57455_45.get_m31(21);
            *row[133] = add_res_limb_21_col133;let add_res_limb_22_col134 = add_res_tmp_57455_45.get_m31(22);
            *row[134] = add_res_limb_22_col134;let add_res_limb_23_col135 = add_res_tmp_57455_45.get_m31(23);
            *row[135] = add_res_limb_23_col135;let add_res_limb_24_col136 = add_res_tmp_57455_45.get_m31(24);
            *row[136] = add_res_limb_24_col136;let add_res_limb_25_col137 = add_res_tmp_57455_45.get_m31(25);
            *row[137] = add_res_limb_25_col137;let add_res_limb_26_col138 = add_res_tmp_57455_45.get_m31(26);
            *row[138] = add_res_limb_26_col138;let add_res_limb_27_col139 = add_res_tmp_57455_45.get_m31(27);
            *row[139] = add_res_limb_27_col139;

            // Range Check Mem Value N 28.

            *sub_component_inputs.range_check_9_9[0] =
                [add_res_limb_0_col112, add_res_limb_1_col113];
            *lookup_data.range_check_9_9_0 = [add_res_limb_0_col112, add_res_limb_1_col113];*sub_component_inputs.range_check_9_9_b[0] =
                [add_res_limb_2_col114, add_res_limb_3_col115];
            *lookup_data.range_check_9_9_b_0 = [add_res_limb_2_col114, add_res_limb_3_col115];*sub_component_inputs.range_check_9_9_c[0] =
                [add_res_limb_4_col116, add_res_limb_5_col117];
            *lookup_data.range_check_9_9_c_0 = [add_res_limb_4_col116, add_res_limb_5_col117];*sub_component_inputs.range_check_9_9_d[0] =
                [add_res_limb_6_col118, add_res_limb_7_col119];
            *lookup_data.range_check_9_9_d_0 = [add_res_limb_6_col118, add_res_limb_7_col119];*sub_component_inputs.range_check_9_9_e[0] =
                [add_res_limb_8_col120, add_res_limb_9_col121];
            *lookup_data.range_check_9_9_e_0 = [add_res_limb_8_col120, add_res_limb_9_col121];*sub_component_inputs.range_check_9_9_f[0] =
                [add_res_limb_10_col122, add_res_limb_11_col123];
            *lookup_data.range_check_9_9_f_0 = [add_res_limb_10_col122, add_res_limb_11_col123];*sub_component_inputs.range_check_9_9_g[0] =
                [add_res_limb_12_col124, add_res_limb_13_col125];
            *lookup_data.range_check_9_9_g_0 = [add_res_limb_12_col124, add_res_limb_13_col125];*sub_component_inputs.range_check_9_9_h[0] =
                [add_res_limb_14_col126, add_res_limb_15_col127];
            *lookup_data.range_check_9_9_h_0 = [add_res_limb_14_col126, add_res_limb_15_col127];*sub_component_inputs.range_check_9_9[1] =
                [add_res_limb_16_col128, add_res_limb_17_col129];
            *lookup_data.range_check_9_9_1 = [add_res_limb_16_col128, add_res_limb_17_col129];*sub_component_inputs.range_check_9_9_b[1] =
                [add_res_limb_18_col130, add_res_limb_19_col131];
            *lookup_data.range_check_9_9_b_1 = [add_res_limb_18_col130, add_res_limb_19_col131];*sub_component_inputs.range_check_9_9_c[1] =
                [add_res_limb_20_col132, add_res_limb_21_col133];
            *lookup_data.range_check_9_9_c_1 = [add_res_limb_20_col132, add_res_limb_21_col133];*sub_component_inputs.range_check_9_9_d[1] =
                [add_res_limb_22_col134, add_res_limb_23_col135];
            *lookup_data.range_check_9_9_d_1 = [add_res_limb_22_col134, add_res_limb_23_col135];*sub_component_inputs.range_check_9_9_e[1] =
                [add_res_limb_24_col136, add_res_limb_25_col137];
            *lookup_data.range_check_9_9_e_1 = [add_res_limb_24_col136, add_res_limb_25_col137];*sub_component_inputs.range_check_9_9_f[1] =
                [add_res_limb_26_col138, add_res_limb_27_col139];
            *lookup_data.range_check_9_9_f_1 = [add_res_limb_26_col138, add_res_limb_27_col139];

            // Verify Add 252.

            let sub_p_bit_tmp_57455_46 = ((UInt16_1) & (((((PackedUInt16::from_m31(op0_limb_0_col53)) ^ (PackedUInt16::from_m31(op1_limb_0_col84)))) ^ (PackedUInt16::from_m31(add_res_limb_0_col112)))));let sub_p_bit_col140 = sub_p_bit_tmp_57455_46.as_m31();
            *row[140] = sub_p_bit_col140;

            let add_252_output_tmp_57455_56 = add_res_tmp_57455_45;

            // Mul 252.

            let mul_res_tmp_57455_57 = ((read_positive_num_bits_252_output_tmp_57455_36.0) * (read_positive_num_bits_252_output_tmp_57455_44.0));let mul_res_limb_0_col141 = mul_res_tmp_57455_57.get_m31(0);
            *row[141] = mul_res_limb_0_col141;let mul_res_limb_1_col142 = mul_res_tmp_57455_57.get_m31(1);
            *row[142] = mul_res_limb_1_col142;let mul_res_limb_2_col143 = mul_res_tmp_57455_57.get_m31(2);
            *row[143] = mul_res_limb_2_col143;let mul_res_limb_3_col144 = mul_res_tmp_57455_57.get_m31(3);
            *row[144] = mul_res_limb_3_col144;let mul_res_limb_4_col145 = mul_res_tmp_57455_57.get_m31(4);
            *row[145] = mul_res_limb_4_col145;let mul_res_limb_5_col146 = mul_res_tmp_57455_57.get_m31(5);
            *row[146] = mul_res_limb_5_col146;let mul_res_limb_6_col147 = mul_res_tmp_57455_57.get_m31(6);
            *row[147] = mul_res_limb_6_col147;let mul_res_limb_7_col148 = mul_res_tmp_57455_57.get_m31(7);
            *row[148] = mul_res_limb_7_col148;let mul_res_limb_8_col149 = mul_res_tmp_57455_57.get_m31(8);
            *row[149] = mul_res_limb_8_col149;let mul_res_limb_9_col150 = mul_res_tmp_57455_57.get_m31(9);
            *row[150] = mul_res_limb_9_col150;let mul_res_limb_10_col151 = mul_res_tmp_57455_57.get_m31(10);
            *row[151] = mul_res_limb_10_col151;let mul_res_limb_11_col152 = mul_res_tmp_57455_57.get_m31(11);
            *row[152] = mul_res_limb_11_col152;let mul_res_limb_12_col153 = mul_res_tmp_57455_57.get_m31(12);
            *row[153] = mul_res_limb_12_col153;let mul_res_limb_13_col154 = mul_res_tmp_57455_57.get_m31(13);
            *row[154] = mul_res_limb_13_col154;let mul_res_limb_14_col155 = mul_res_tmp_57455_57.get_m31(14);
            *row[155] = mul_res_limb_14_col155;let mul_res_limb_15_col156 = mul_res_tmp_57455_57.get_m31(15);
            *row[156] = mul_res_limb_15_col156;let mul_res_limb_16_col157 = mul_res_tmp_57455_57.get_m31(16);
            *row[157] = mul_res_limb_16_col157;let mul_res_limb_17_col158 = mul_res_tmp_57455_57.get_m31(17);
            *row[158] = mul_res_limb_17_col158;let mul_res_limb_18_col159 = mul_res_tmp_57455_57.get_m31(18);
            *row[159] = mul_res_limb_18_col159;let mul_res_limb_19_col160 = mul_res_tmp_57455_57.get_m31(19);
            *row[160] = mul_res_limb_19_col160;let mul_res_limb_20_col161 = mul_res_tmp_57455_57.get_m31(20);
            *row[161] = mul_res_limb_20_col161;let mul_res_limb_21_col162 = mul_res_tmp_57455_57.get_m31(21);
            *row[162] = mul_res_limb_21_col162;let mul_res_limb_22_col163 = mul_res_tmp_57455_57.get_m31(22);
            *row[163] = mul_res_limb_22_col163;let mul_res_limb_23_col164 = mul_res_tmp_57455_57.get_m31(23);
            *row[164] = mul_res_limb_23_col164;let mul_res_limb_24_col165 = mul_res_tmp_57455_57.get_m31(24);
            *row[165] = mul_res_limb_24_col165;let mul_res_limb_25_col166 = mul_res_tmp_57455_57.get_m31(25);
            *row[166] = mul_res_limb_25_col166;let mul_res_limb_26_col167 = mul_res_tmp_57455_57.get_m31(26);
            *row[167] = mul_res_limb_26_col167;let mul_res_limb_27_col168 = mul_res_tmp_57455_57.get_m31(27);
            *row[168] = mul_res_limb_27_col168;

            // Range Check Mem Value N 28.

            *sub_component_inputs.range_check_9_9[2] =
                [mul_res_limb_0_col141, mul_res_limb_1_col142];
            *lookup_data.range_check_9_9_2 = [mul_res_limb_0_col141, mul_res_limb_1_col142];*sub_component_inputs.range_check_9_9_b[2] =
                [mul_res_limb_2_col143, mul_res_limb_3_col144];
            *lookup_data.range_check_9_9_b_2 = [mul_res_limb_2_col143, mul_res_limb_3_col144];*sub_component_inputs.range_check_9_9_c[2] =
                [mul_res_limb_4_col145, mul_res_limb_5_col146];
            *lookup_data.range_check_9_9_c_2 = [mul_res_limb_4_col145, mul_res_limb_5_col146];*sub_component_inputs.range_check_9_9_d[2] =
                [mul_res_limb_6_col147, mul_res_limb_7_col148];
            *lookup_data.range_check_9_9_d_2 = [mul_res_limb_6_col147, mul_res_limb_7_col148];*sub_component_inputs.range_check_9_9_e[2] =
                [mul_res_limb_8_col149, mul_res_limb_9_col150];
            *lookup_data.range_check_9_9_e_2 = [mul_res_limb_8_col149, mul_res_limb_9_col150];*sub_component_inputs.range_check_9_9_f[2] =
                [mul_res_limb_10_col151, mul_res_limb_11_col152];
            *lookup_data.range_check_9_9_f_2 = [mul_res_limb_10_col151, mul_res_limb_11_col152];*sub_component_inputs.range_check_9_9_g[1] =
                [mul_res_limb_12_col153, mul_res_limb_13_col154];
            *lookup_data.range_check_9_9_g_1 = [mul_res_limb_12_col153, mul_res_limb_13_col154];*sub_component_inputs.range_check_9_9_h[1] =
                [mul_res_limb_14_col155, mul_res_limb_15_col156];
            *lookup_data.range_check_9_9_h_1 = [mul_res_limb_14_col155, mul_res_limb_15_col156];*sub_component_inputs.range_check_9_9[3] =
                [mul_res_limb_16_col157, mul_res_limb_17_col158];
            *lookup_data.range_check_9_9_3 = [mul_res_limb_16_col157, mul_res_limb_17_col158];*sub_component_inputs.range_check_9_9_b[3] =
                [mul_res_limb_18_col159, mul_res_limb_19_col160];
            *lookup_data.range_check_9_9_b_3 = [mul_res_limb_18_col159, mul_res_limb_19_col160];*sub_component_inputs.range_check_9_9_c[3] =
                [mul_res_limb_20_col161, mul_res_limb_21_col162];
            *lookup_data.range_check_9_9_c_3 = [mul_res_limb_20_col161, mul_res_limb_21_col162];*sub_component_inputs.range_check_9_9_d[3] =
                [mul_res_limb_22_col163, mul_res_limb_23_col164];
            *lookup_data.range_check_9_9_d_3 = [mul_res_limb_22_col163, mul_res_limb_23_col164];*sub_component_inputs.range_check_9_9_e[3] =
                [mul_res_limb_24_col165, mul_res_limb_25_col166];
            *lookup_data.range_check_9_9_e_3 = [mul_res_limb_24_col165, mul_res_limb_25_col166];*sub_component_inputs.range_check_9_9_f[3] =
                [mul_res_limb_26_col167, mul_res_limb_27_col168];
            *lookup_data.range_check_9_9_f_3 = [mul_res_limb_26_col167, mul_res_limb_27_col168];

            // Verify Mul 252.

            // Double Karatsuba 1454 B.

            // Single Karatsuba N 7.

            let z0_tmp_57455_58 = [((op0_limb_0_col53) * (op1_limb_0_col84)), ((((op0_limb_0_col53) * (op1_limb_1_col85))) + (((op0_limb_1_col54) * (op1_limb_0_col84)))), ((((((op0_limb_0_col53) * (op1_limb_2_col86))) + (((op0_limb_1_col54) * (op1_limb_1_col85))))) + (((op0_limb_2_col55) * (op1_limb_0_col84)))), ((((((((op0_limb_0_col53) * (op1_limb_3_col87))) + (((op0_limb_1_col54) * (op1_limb_2_col86))))) + (((op0_limb_2_col55) * (op1_limb_1_col85))))) + (((op0_limb_3_col56) * (op1_limb_0_col84)))), ((((((((((op0_limb_0_col53) * (op1_limb_4_col88))) + (((op0_limb_1_col54) * (op1_limb_3_col87))))) + (((op0_limb_2_col55) * (op1_limb_2_col86))))) + (((op0_limb_3_col56) * (op1_limb_1_col85))))) + (((op0_limb_4_col57) * (op1_limb_0_col84)))), ((((((((((((op0_limb_0_col53) * (op1_limb_5_col89))) + (((op0_limb_1_col54) * (op1_limb_4_col88))))) + (((op0_limb_2_col55) * (op1_limb_3_col87))))) + (((op0_limb_3_col56) * (op1_limb_2_col86))))) + (((op0_limb_4_col57) * (op1_limb_1_col85))))) + (((op0_limb_5_col58) * (op1_limb_0_col84)))), ((((((((((((((op0_limb_0_col53) * (op1_limb_6_col90))) + (((op0_limb_1_col54) * (op1_limb_5_col89))))) + (((op0_limb_2_col55) * (op1_limb_4_col88))))) + (((op0_limb_3_col56) * (op1_limb_3_col87))))) + (((op0_limb_4_col57) * (op1_limb_2_col86))))) + (((op0_limb_5_col58) * (op1_limb_1_col85))))) + (((op0_limb_6_col59) * (op1_limb_0_col84)))), ((((((((((((op0_limb_1_col54) * (op1_limb_6_col90))) + (((op0_limb_2_col55) * (op1_limb_5_col89))))) + (((op0_limb_3_col56) * (op1_limb_4_col88))))) + (((op0_limb_4_col57) * (op1_limb_3_col87))))) + (((op0_limb_5_col58) * (op1_limb_2_col86))))) + (((op0_limb_6_col59) * (op1_limb_1_col85)))), ((((((((((op0_limb_2_col55) * (op1_limb_6_col90))) + (((op0_limb_3_col56) * (op1_limb_5_col89))))) + (((op0_limb_4_col57) * (op1_limb_4_col88))))) + (((op0_limb_5_col58) * (op1_limb_3_col87))))) + (((op0_limb_6_col59) * (op1_limb_2_col86)))), ((((((((op0_limb_3_col56) * (op1_limb_6_col90))) + (((op0_limb_4_col57) * (op1_limb_5_col89))))) + (((op0_limb_5_col58) * (op1_limb_4_col88))))) + (((op0_limb_6_col59) * (op1_limb_3_col87)))), ((((((op0_limb_4_col57) * (op1_limb_6_col90))) + (((op0_limb_5_col58) * (op1_limb_5_col89))))) + (((op0_limb_6_col59) * (op1_limb_4_col88)))), ((((op0_limb_5_col58) * (op1_limb_6_col90))) + (((op0_limb_6_col59) * (op1_limb_5_col89)))), ((op0_limb_6_col59) * (op1_limb_6_col90))];let z2_tmp_57455_59 = [((op0_limb_7_col60) * (op1_limb_7_col91)), ((((op0_limb_7_col60) * (op1_limb_8_col92))) + (((op0_limb_8_col61) * (op1_limb_7_col91)))), ((((((op0_limb_7_col60) * (op1_limb_9_col93))) + (((op0_limb_8_col61) * (op1_limb_8_col92))))) + (((op0_limb_9_col62) * (op1_limb_7_col91)))), ((((((((op0_limb_7_col60) * (op1_limb_10_col94))) + (((op0_limb_8_col61) * (op1_limb_9_col93))))) + (((op0_limb_9_col62) * (op1_limb_8_col92))))) + (((op0_limb_10_col63) * (op1_limb_7_col91)))), ((((((((((op0_limb_7_col60) * (op1_limb_11_col95))) + (((op0_limb_8_col61) * (op1_limb_10_col94))))) + (((op0_limb_9_col62) * (op1_limb_9_col93))))) + (((op0_limb_10_col63) * (op1_limb_8_col92))))) + (((op0_limb_11_col64) * (op1_limb_7_col91)))), ((((((((((((op0_limb_7_col60) * (op1_limb_12_col96))) + (((op0_limb_8_col61) * (op1_limb_11_col95))))) + (((op0_limb_9_col62) * (op1_limb_10_col94))))) + (((op0_limb_10_col63) * (op1_limb_9_col93))))) + (((op0_limb_11_col64) * (op1_limb_8_col92))))) + (((op0_limb_12_col65) * (op1_limb_7_col91)))), ((((((((((((((op0_limb_7_col60) * (op1_limb_13_col97))) + (((op0_limb_8_col61) * (op1_limb_12_col96))))) + (((op0_limb_9_col62) * (op1_limb_11_col95))))) + (((op0_limb_10_col63) * (op1_limb_10_col94))))) + (((op0_limb_11_col64) * (op1_limb_9_col93))))) + (((op0_limb_12_col65) * (op1_limb_8_col92))))) + (((op0_limb_13_col66) * (op1_limb_7_col91)))), ((((((((((((op0_limb_8_col61) * (op1_limb_13_col97))) + (((op0_limb_9_col62) * (op1_limb_12_col96))))) + (((op0_limb_10_col63) * (op1_limb_11_col95))))) + (((op0_limb_11_col64) * (op1_limb_10_col94))))) + (((op0_limb_12_col65) * (op1_limb_9_col93))))) + (((op0_limb_13_col66) * (op1_limb_8_col92)))), ((((((((((op0_limb_9_col62) * (op1_limb_13_col97))) + (((op0_limb_10_col63) * (op1_limb_12_col96))))) + (((op0_limb_11_col64) * (op1_limb_11_col95))))) + (((op0_limb_12_col65) * (op1_limb_10_col94))))) + (((op0_limb_13_col66) * (op1_limb_9_col93)))), ((((((((op0_limb_10_col63) * (op1_limb_13_col97))) + (((op0_limb_11_col64) * (op1_limb_12_col96))))) + (((op0_limb_12_col65) * (op1_limb_11_col95))))) + (((op0_limb_13_col66) * (op1_limb_10_col94)))), ((((((op0_limb_11_col64) * (op1_limb_13_col97))) + (((op0_limb_12_col65) * (op1_limb_12_col96))))) + (((op0_limb_13_col66) * (op1_limb_11_col95)))), ((((op0_limb_12_col65) * (op1_limb_13_col97))) + (((op0_limb_13_col66) * (op1_limb_12_col96)))), ((op0_limb_13_col66) * (op1_limb_13_col97))];let x_sum_tmp_57455_60 = [((op0_limb_0_col53) + (op0_limb_7_col60)), ((op0_limb_1_col54) + (op0_limb_8_col61)), ((op0_limb_2_col55) + (op0_limb_9_col62)), ((op0_limb_3_col56) + (op0_limb_10_col63)), ((op0_limb_4_col57) + (op0_limb_11_col64)), ((op0_limb_5_col58) + (op0_limb_12_col65)), ((op0_limb_6_col59) + (op0_limb_13_col66))];let y_sum_tmp_57455_61 = [((op1_limb_0_col84) + (op1_limb_7_col91)), ((op1_limb_1_col85) + (op1_limb_8_col92)), ((op1_limb_2_col86) + (op1_limb_9_col93)), ((op1_limb_3_col87) + (op1_limb_10_col94)), ((op1_limb_4_col88) + (op1_limb_11_col95)), ((op1_limb_5_col89) + (op1_limb_12_col96)), ((op1_limb_6_col90) + (op1_limb_13_col97))];let single_karatsuba_n_7_output_tmp_57455_62 = [z0_tmp_57455_58[0], z0_tmp_57455_58[1], z0_tmp_57455_58[2], z0_tmp_57455_58[3], z0_tmp_57455_58[4], z0_tmp_57455_58[5], z0_tmp_57455_58[6], ((z0_tmp_57455_58[7]) + (((((((x_sum_tmp_57455_60[0]) * (y_sum_tmp_57455_61[0]))) - (z0_tmp_57455_58[0]))) - (z2_tmp_57455_59[0])))), ((z0_tmp_57455_58[8]) + (((((((((x_sum_tmp_57455_60[0]) * (y_sum_tmp_57455_61[1]))) + (((x_sum_tmp_57455_60[1]) * (y_sum_tmp_57455_61[0]))))) - (z0_tmp_57455_58[1]))) - (z2_tmp_57455_59[1])))), ((z0_tmp_57455_58[9]) + (((((((((((x_sum_tmp_57455_60[0]) * (y_sum_tmp_57455_61[2]))) + (((x_sum_tmp_57455_60[1]) * (y_sum_tmp_57455_61[1]))))) + (((x_sum_tmp_57455_60[2]) * (y_sum_tmp_57455_61[0]))))) - (z0_tmp_57455_58[2]))) - (z2_tmp_57455_59[2])))), ((z0_tmp_57455_58[10]) + (((((((((((((x_sum_tmp_57455_60[0]) * (y_sum_tmp_57455_61[3]))) + (((x_sum_tmp_57455_60[1]) * (y_sum_tmp_57455_61[2]))))) + (((x_sum_tmp_57455_60[2]) * (y_sum_tmp_57455_61[1]))))) + (((x_sum_tmp_57455_60[3]) * (y_sum_tmp_57455_61[0]))))) - (z0_tmp_57455_58[3]))) - (z2_tmp_57455_59[3])))), ((z0_tmp_57455_58[11]) + (((((((((((((((x_sum_tmp_57455_60[0]) * (y_sum_tmp_57455_61[4]))) + (((x_sum_tmp_57455_60[1]) * (y_sum_tmp_57455_61[3]))))) + (((x_sum_tmp_57455_60[2]) * (y_sum_tmp_57455_61[2]))))) + (((x_sum_tmp_57455_60[3]) * (y_sum_tmp_57455_61[1]))))) + (((x_sum_tmp_57455_60[4]) * (y_sum_tmp_57455_61[0]))))) - (z0_tmp_57455_58[4]))) - (z2_tmp_57455_59[4])))), ((z0_tmp_57455_58[12]) + (((((((((((((((((x_sum_tmp_57455_60[0]) * (y_sum_tmp_57455_61[5]))) + (((x_sum_tmp_57455_60[1]) * (y_sum_tmp_57455_61[4]))))) + (((x_sum_tmp_57455_60[2]) * (y_sum_tmp_57455_61[3]))))) + (((x_sum_tmp_57455_60[3]) * (y_sum_tmp_57455_61[2]))))) + (((x_sum_tmp_57455_60[4]) * (y_sum_tmp_57455_61[1]))))) + (((x_sum_tmp_57455_60[5]) * (y_sum_tmp_57455_61[0]))))) - (z0_tmp_57455_58[5]))) - (z2_tmp_57455_59[5])))), ((((((((((((((((((x_sum_tmp_57455_60[0]) * (y_sum_tmp_57455_61[6]))) + (((x_sum_tmp_57455_60[1]) * (y_sum_tmp_57455_61[5]))))) + (((x_sum_tmp_57455_60[2]) * (y_sum_tmp_57455_61[4]))))) + (((x_sum_tmp_57455_60[3]) * (y_sum_tmp_57455_61[3]))))) + (((x_sum_tmp_57455_60[4]) * (y_sum_tmp_57455_61[2]))))) + (((x_sum_tmp_57455_60[5]) * (y_sum_tmp_57455_61[1]))))) + (((x_sum_tmp_57455_60[6]) * (y_sum_tmp_57455_61[0]))))) - (z0_tmp_57455_58[6]))) - (z2_tmp_57455_59[6])), ((z2_tmp_57455_59[0]) + (((((((((((((((((x_sum_tmp_57455_60[1]) * (y_sum_tmp_57455_61[6]))) + (((x_sum_tmp_57455_60[2]) * (y_sum_tmp_57455_61[5]))))) + (((x_sum_tmp_57455_60[3]) * (y_sum_tmp_57455_61[4]))))) + (((x_sum_tmp_57455_60[4]) * (y_sum_tmp_57455_61[3]))))) + (((x_sum_tmp_57455_60[5]) * (y_sum_tmp_57455_61[2]))))) + (((x_sum_tmp_57455_60[6]) * (y_sum_tmp_57455_61[1]))))) - (z0_tmp_57455_58[7]))) - (z2_tmp_57455_59[7])))), ((z2_tmp_57455_59[1]) + (((((((((((((((x_sum_tmp_57455_60[2]) * (y_sum_tmp_57455_61[6]))) + (((x_sum_tmp_57455_60[3]) * (y_sum_tmp_57455_61[5]))))) + (((x_sum_tmp_57455_60[4]) * (y_sum_tmp_57455_61[4]))))) + (((x_sum_tmp_57455_60[5]) * (y_sum_tmp_57455_61[3]))))) + (((x_sum_tmp_57455_60[6]) * (y_sum_tmp_57455_61[2]))))) - (z0_tmp_57455_58[8]))) - (z2_tmp_57455_59[8])))), ((z2_tmp_57455_59[2]) + (((((((((((((x_sum_tmp_57455_60[3]) * (y_sum_tmp_57455_61[6]))) + (((x_sum_tmp_57455_60[4]) * (y_sum_tmp_57455_61[5]))))) + (((x_sum_tmp_57455_60[5]) * (y_sum_tmp_57455_61[4]))))) + (((x_sum_tmp_57455_60[6]) * (y_sum_tmp_57455_61[3]))))) - (z0_tmp_57455_58[9]))) - (z2_tmp_57455_59[9])))), ((z2_tmp_57455_59[3]) + (((((((((((x_sum_tmp_57455_60[4]) * (y_sum_tmp_57455_61[6]))) + (((x_sum_tmp_57455_60[5]) * (y_sum_tmp_57455_61[5]))))) + (((x_sum_tmp_57455_60[6]) * (y_sum_tmp_57455_61[4]))))) - (z0_tmp_57455_58[10]))) - (z2_tmp_57455_59[10])))), ((z2_tmp_57455_59[4]) + (((((((((x_sum_tmp_57455_60[5]) * (y_sum_tmp_57455_61[6]))) + (((x_sum_tmp_57455_60[6]) * (y_sum_tmp_57455_61[5]))))) - (z0_tmp_57455_58[11]))) - (z2_tmp_57455_59[11])))), ((z2_tmp_57455_59[5]) + (((((((x_sum_tmp_57455_60[6]) * (y_sum_tmp_57455_61[6]))) - (z0_tmp_57455_58[12]))) - (z2_tmp_57455_59[12])))), z2_tmp_57455_59[6], z2_tmp_57455_59[7], z2_tmp_57455_59[8], z2_tmp_57455_59[9], z2_tmp_57455_59[10], z2_tmp_57455_59[11], z2_tmp_57455_59[12]];

            // Single Karatsuba N 7.

            let z0_tmp_57455_63 = [((op0_limb_14_col67) * (op1_limb_14_col98)), ((((op0_limb_14_col67) * (op1_limb_15_col99))) + (((op0_limb_15_col68) * (op1_limb_14_col98)))), ((((((op0_limb_14_col67) * (op1_limb_16_col100))) + (((op0_limb_15_col68) * (op1_limb_15_col99))))) + (((op0_limb_16_col69) * (op1_limb_14_col98)))), ((((((((op0_limb_14_col67) * (op1_limb_17_col101))) + (((op0_limb_15_col68) * (op1_limb_16_col100))))) + (((op0_limb_16_col69) * (op1_limb_15_col99))))) + (((op0_limb_17_col70) * (op1_limb_14_col98)))), ((((((((((op0_limb_14_col67) * (op1_limb_18_col102))) + (((op0_limb_15_col68) * (op1_limb_17_col101))))) + (((op0_limb_16_col69) * (op1_limb_16_col100))))) + (((op0_limb_17_col70) * (op1_limb_15_col99))))) + (((op0_limb_18_col71) * (op1_limb_14_col98)))), ((((((((((((op0_limb_14_col67) * (op1_limb_19_col103))) + (((op0_limb_15_col68) * (op1_limb_18_col102))))) + (((op0_limb_16_col69) * (op1_limb_17_col101))))) + (((op0_limb_17_col70) * (op1_limb_16_col100))))) + (((op0_limb_18_col71) * (op1_limb_15_col99))))) + (((op0_limb_19_col72) * (op1_limb_14_col98)))), ((((((((((((((op0_limb_14_col67) * (op1_limb_20_col104))) + (((op0_limb_15_col68) * (op1_limb_19_col103))))) + (((op0_limb_16_col69) * (op1_limb_18_col102))))) + (((op0_limb_17_col70) * (op1_limb_17_col101))))) + (((op0_limb_18_col71) * (op1_limb_16_col100))))) + (((op0_limb_19_col72) * (op1_limb_15_col99))))) + (((op0_limb_20_col73) * (op1_limb_14_col98)))), ((((((((((((op0_limb_15_col68) * (op1_limb_20_col104))) + (((op0_limb_16_col69) * (op1_limb_19_col103))))) + (((op0_limb_17_col70) * (op1_limb_18_col102))))) + (((op0_limb_18_col71) * (op1_limb_17_col101))))) + (((op0_limb_19_col72) * (op1_limb_16_col100))))) + (((op0_limb_20_col73) * (op1_limb_15_col99)))), ((((((((((op0_limb_16_col69) * (op1_limb_20_col104))) + (((op0_limb_17_col70) * (op1_limb_19_col103))))) + (((op0_limb_18_col71) * (op1_limb_18_col102))))) + (((op0_limb_19_col72) * (op1_limb_17_col101))))) + (((op0_limb_20_col73) * (op1_limb_16_col100)))), ((((((((op0_limb_17_col70) * (op1_limb_20_col104))) + (((op0_limb_18_col71) * (op1_limb_19_col103))))) + (((op0_limb_19_col72) * (op1_limb_18_col102))))) + (((op0_limb_20_col73) * (op1_limb_17_col101)))), ((((((op0_limb_18_col71) * (op1_limb_20_col104))) + (((op0_limb_19_col72) * (op1_limb_19_col103))))) + (((op0_limb_20_col73) * (op1_limb_18_col102)))), ((((op0_limb_19_col72) * (op1_limb_20_col104))) + (((op0_limb_20_col73) * (op1_limb_19_col103)))), ((op0_limb_20_col73) * (op1_limb_20_col104))];let z2_tmp_57455_64 = [((op0_limb_21_col74) * (op1_limb_21_col105)), ((((op0_limb_21_col74) * (op1_limb_22_col106))) + (((op0_limb_22_col75) * (op1_limb_21_col105)))), ((((((op0_limb_21_col74) * (op1_limb_23_col107))) + (((op0_limb_22_col75) * (op1_limb_22_col106))))) + (((op0_limb_23_col76) * (op1_limb_21_col105)))), ((((((((op0_limb_21_col74) * (op1_limb_24_col108))) + (((op0_limb_22_col75) * (op1_limb_23_col107))))) + (((op0_limb_23_col76) * (op1_limb_22_col106))))) + (((op0_limb_24_col77) * (op1_limb_21_col105)))), ((((((((((op0_limb_21_col74) * (op1_limb_25_col109))) + (((op0_limb_22_col75) * (op1_limb_24_col108))))) + (((op0_limb_23_col76) * (op1_limb_23_col107))))) + (((op0_limb_24_col77) * (op1_limb_22_col106))))) + (((op0_limb_25_col78) * (op1_limb_21_col105)))), ((((((((((((op0_limb_21_col74) * (op1_limb_26_col110))) + (((op0_limb_22_col75) * (op1_limb_25_col109))))) + (((op0_limb_23_col76) * (op1_limb_24_col108))))) + (((op0_limb_24_col77) * (op1_limb_23_col107))))) + (((op0_limb_25_col78) * (op1_limb_22_col106))))) + (((op0_limb_26_col79) * (op1_limb_21_col105)))), ((((((((((((((op0_limb_21_col74) * (op1_limb_27_col111))) + (((op0_limb_22_col75) * (op1_limb_26_col110))))) + (((op0_limb_23_col76) * (op1_limb_25_col109))))) + (((op0_limb_24_col77) * (op1_limb_24_col108))))) + (((op0_limb_25_col78) * (op1_limb_23_col107))))) + (((op0_limb_26_col79) * (op1_limb_22_col106))))) + (((op0_limb_27_col80) * (op1_limb_21_col105)))), ((((((((((((op0_limb_22_col75) * (op1_limb_27_col111))) + (((op0_limb_23_col76) * (op1_limb_26_col110))))) + (((op0_limb_24_col77) * (op1_limb_25_col109))))) + (((op0_limb_25_col78) * (op1_limb_24_col108))))) + (((op0_limb_26_col79) * (op1_limb_23_col107))))) + (((op0_limb_27_col80) * (op1_limb_22_col106)))), ((((((((((op0_limb_23_col76) * (op1_limb_27_col111))) + (((op0_limb_24_col77) * (op1_limb_26_col110))))) + (((op0_limb_25_col78) * (op1_limb_25_col109))))) + (((op0_limb_26_col79) * (op1_limb_24_col108))))) + (((op0_limb_27_col80) * (op1_limb_23_col107)))), ((((((((op0_limb_24_col77) * (op1_limb_27_col111))) + (((op0_limb_25_col78) * (op1_limb_26_col110))))) + (((op0_limb_26_col79) * (op1_limb_25_col109))))) + (((op0_limb_27_col80) * (op1_limb_24_col108)))), ((((((op0_limb_25_col78) * (op1_limb_27_col111))) + (((op0_limb_26_col79) * (op1_limb_26_col110))))) + (((op0_limb_27_col80) * (op1_limb_25_col109)))), ((((op0_limb_26_col79) * (op1_limb_27_col111))) + (((op0_limb_27_col80) * (op1_limb_26_col110)))), ((op0_limb_27_col80) * (op1_limb_27_col111))];let x_sum_tmp_57455_65 = [((op0_limb_14_col67) + (op0_limb_21_col74)), ((op0_limb_15_col68) + (op0_limb_22_col75)), ((op0_limb_16_col69) + (op0_limb_23_col76)), ((op0_limb_17_col70) + (op0_limb_24_col77)), ((op0_limb_18_col71) + (op0_limb_25_col78)), ((op0_limb_19_col72) + (op0_limb_26_col79)), ((op0_limb_20_col73) + (op0_limb_27_col80))];let y_sum_tmp_57455_66 = [((op1_limb_14_col98) + (op1_limb_21_col105)), ((op1_limb_15_col99) + (op1_limb_22_col106)), ((op1_limb_16_col100) + (op1_limb_23_col107)), ((op1_limb_17_col101) + (op1_limb_24_col108)), ((op1_limb_18_col102) + (op1_limb_25_col109)), ((op1_limb_19_col103) + (op1_limb_26_col110)), ((op1_limb_20_col104) + (op1_limb_27_col111))];let single_karatsuba_n_7_output_tmp_57455_67 = [z0_tmp_57455_63[0], z0_tmp_57455_63[1], z0_tmp_57455_63[2], z0_tmp_57455_63[3], z0_tmp_57455_63[4], z0_tmp_57455_63[5], z0_tmp_57455_63[6], ((z0_tmp_57455_63[7]) + (((((((x_sum_tmp_57455_65[0]) * (y_sum_tmp_57455_66[0]))) - (z0_tmp_57455_63[0]))) - (z2_tmp_57455_64[0])))), ((z0_tmp_57455_63[8]) + (((((((((x_sum_tmp_57455_65[0]) * (y_sum_tmp_57455_66[1]))) + (((x_sum_tmp_57455_65[1]) * (y_sum_tmp_57455_66[0]))))) - (z0_tmp_57455_63[1]))) - (z2_tmp_57455_64[1])))), ((z0_tmp_57455_63[9]) + (((((((((((x_sum_tmp_57455_65[0]) * (y_sum_tmp_57455_66[2]))) + (((x_sum_tmp_57455_65[1]) * (y_sum_tmp_57455_66[1]))))) + (((x_sum_tmp_57455_65[2]) * (y_sum_tmp_57455_66[0]))))) - (z0_tmp_57455_63[2]))) - (z2_tmp_57455_64[2])))), ((z0_tmp_57455_63[10]) + (((((((((((((x_sum_tmp_57455_65[0]) * (y_sum_tmp_57455_66[3]))) + (((x_sum_tmp_57455_65[1]) * (y_sum_tmp_57455_66[2]))))) + (((x_sum_tmp_57455_65[2]) * (y_sum_tmp_57455_66[1]))))) + (((x_sum_tmp_57455_65[3]) * (y_sum_tmp_57455_66[0]))))) - (z0_tmp_57455_63[3]))) - (z2_tmp_57455_64[3])))), ((z0_tmp_57455_63[11]) + (((((((((((((((x_sum_tmp_57455_65[0]) * (y_sum_tmp_57455_66[4]))) + (((x_sum_tmp_57455_65[1]) * (y_sum_tmp_57455_66[3]))))) + (((x_sum_tmp_57455_65[2]) * (y_sum_tmp_57455_66[2]))))) + (((x_sum_tmp_57455_65[3]) * (y_sum_tmp_57455_66[1]))))) + (((x_sum_tmp_57455_65[4]) * (y_sum_tmp_57455_66[0]))))) - (z0_tmp_57455_63[4]))) - (z2_tmp_57455_64[4])))), ((z0_tmp_57455_63[12]) + (((((((((((((((((x_sum_tmp_57455_65[0]) * (y_sum_tmp_57455_66[5]))) + (((x_sum_tmp_57455_65[1]) * (y_sum_tmp_57455_66[4]))))) + (((x_sum_tmp_57455_65[2]) * (y_sum_tmp_57455_66[3]))))) + (((x_sum_tmp_57455_65[3]) * (y_sum_tmp_57455_66[2]))))) + (((x_sum_tmp_57455_65[4]) * (y_sum_tmp_57455_66[1]))))) + (((x_sum_tmp_57455_65[5]) * (y_sum_tmp_57455_66[0]))))) - (z0_tmp_57455_63[5]))) - (z2_tmp_57455_64[5])))), ((((((((((((((((((x_sum_tmp_57455_65[0]) * (y_sum_tmp_57455_66[6]))) + (((x_sum_tmp_57455_65[1]) * (y_sum_tmp_57455_66[5]))))) + (((x_sum_tmp_57455_65[2]) * (y_sum_tmp_57455_66[4]))))) + (((x_sum_tmp_57455_65[3]) * (y_sum_tmp_57455_66[3]))))) + (((x_sum_tmp_57455_65[4]) * (y_sum_tmp_57455_66[2]))))) + (((x_sum_tmp_57455_65[5]) * (y_sum_tmp_57455_66[1]))))) + (((x_sum_tmp_57455_65[6]) * (y_sum_tmp_57455_66[0]))))) - (z0_tmp_57455_63[6]))) - (z2_tmp_57455_64[6])), ((z2_tmp_57455_64[0]) + (((((((((((((((((x_sum_tmp_57455_65[1]) * (y_sum_tmp_57455_66[6]))) + (((x_sum_tmp_57455_65[2]) * (y_sum_tmp_57455_66[5]))))) + (((x_sum_tmp_57455_65[3]) * (y_sum_tmp_57455_66[4]))))) + (((x_sum_tmp_57455_65[4]) * (y_sum_tmp_57455_66[3]))))) + (((x_sum_tmp_57455_65[5]) * (y_sum_tmp_57455_66[2]))))) + (((x_sum_tmp_57455_65[6]) * (y_sum_tmp_57455_66[1]))))) - (z0_tmp_57455_63[7]))) - (z2_tmp_57455_64[7])))), ((z2_tmp_57455_64[1]) + (((((((((((((((x_sum_tmp_57455_65[2]) * (y_sum_tmp_57455_66[6]))) + (((x_sum_tmp_57455_65[3]) * (y_sum_tmp_57455_66[5]))))) + (((x_sum_tmp_57455_65[4]) * (y_sum_tmp_57455_66[4]))))) + (((x_sum_tmp_57455_65[5]) * (y_sum_tmp_57455_66[3]))))) + (((x_sum_tmp_57455_65[6]) * (y_sum_tmp_57455_66[2]))))) - (z0_tmp_57455_63[8]))) - (z2_tmp_57455_64[8])))), ((z2_tmp_57455_64[2]) + (((((((((((((x_sum_tmp_57455_65[3]) * (y_sum_tmp_57455_66[6]))) + (((x_sum_tmp_57455_65[4]) * (y_sum_tmp_57455_66[5]))))) + (((x_sum_tmp_57455_65[5]) * (y_sum_tmp_57455_66[4]))))) + (((x_sum_tmp_57455_65[6]) * (y_sum_tmp_57455_66[3]))))) - (z0_tmp_57455_63[9]))) - (z2_tmp_57455_64[9])))), ((z2_tmp_57455_64[3]) + (((((((((((x_sum_tmp_57455_65[4]) * (y_sum_tmp_57455_66[6]))) + (((x_sum_tmp_57455_65[5]) * (y_sum_tmp_57455_66[5]))))) + (((x_sum_tmp_57455_65[6]) * (y_sum_tmp_57455_66[4]))))) - (z0_tmp_57455_63[10]))) - (z2_tmp_57455_64[10])))), ((z2_tmp_57455_64[4]) + (((((((((x_sum_tmp_57455_65[5]) * (y_sum_tmp_57455_66[6]))) + (((x_sum_tmp_57455_65[6]) * (y_sum_tmp_57455_66[5]))))) - (z0_tmp_57455_63[11]))) - (z2_tmp_57455_64[11])))), ((z2_tmp_57455_64[5]) + (((((((x_sum_tmp_57455_65[6]) * (y_sum_tmp_57455_66[6]))) - (z0_tmp_57455_63[12]))) - (z2_tmp_57455_64[12])))), z2_tmp_57455_64[6], z2_tmp_57455_64[7], z2_tmp_57455_64[8], z2_tmp_57455_64[9], z2_tmp_57455_64[10], z2_tmp_57455_64[11], z2_tmp_57455_64[12]];

            let x_sum_tmp_57455_68 = [((op0_limb_0_col53) + (op0_limb_14_col67)), ((op0_limb_1_col54) + (op0_limb_15_col68)), ((op0_limb_2_col55) + (op0_limb_16_col69)), ((op0_limb_3_col56) + (op0_limb_17_col70)), ((op0_limb_4_col57) + (op0_limb_18_col71)), ((op0_limb_5_col58) + (op0_limb_19_col72)), ((op0_limb_6_col59) + (op0_limb_20_col73)), ((op0_limb_7_col60) + (op0_limb_21_col74)), ((op0_limb_8_col61) + (op0_limb_22_col75)), ((op0_limb_9_col62) + (op0_limb_23_col76)), ((op0_limb_10_col63) + (op0_limb_24_col77)), ((op0_limb_11_col64) + (op0_limb_25_col78)), ((op0_limb_12_col65) + (op0_limb_26_col79)), ((op0_limb_13_col66) + (op0_limb_27_col80))];let y_sum_tmp_57455_69 = [((op1_limb_0_col84) + (op1_limb_14_col98)), ((op1_limb_1_col85) + (op1_limb_15_col99)), ((op1_limb_2_col86) + (op1_limb_16_col100)), ((op1_limb_3_col87) + (op1_limb_17_col101)), ((op1_limb_4_col88) + (op1_limb_18_col102)), ((op1_limb_5_col89) + (op1_limb_19_col103)), ((op1_limb_6_col90) + (op1_limb_20_col104)), ((op1_limb_7_col91) + (op1_limb_21_col105)), ((op1_limb_8_col92) + (op1_limb_22_col106)), ((op1_limb_9_col93) + (op1_limb_23_col107)), ((op1_limb_10_col94) + (op1_limb_24_col108)), ((op1_limb_11_col95) + (op1_limb_25_col109)), ((op1_limb_12_col96) + (op1_limb_26_col110)), ((op1_limb_13_col97) + (op1_limb_27_col111))];

            // Single Karatsuba N 7.

            let z0_tmp_57455_70 = [((x_sum_tmp_57455_68[0]) * (y_sum_tmp_57455_69[0])), ((((x_sum_tmp_57455_68[0]) * (y_sum_tmp_57455_69[1]))) + (((x_sum_tmp_57455_68[1]) * (y_sum_tmp_57455_69[0])))), ((((((x_sum_tmp_57455_68[0]) * (y_sum_tmp_57455_69[2]))) + (((x_sum_tmp_57455_68[1]) * (y_sum_tmp_57455_69[1]))))) + (((x_sum_tmp_57455_68[2]) * (y_sum_tmp_57455_69[0])))), ((((((((x_sum_tmp_57455_68[0]) * (y_sum_tmp_57455_69[3]))) + (((x_sum_tmp_57455_68[1]) * (y_sum_tmp_57455_69[2]))))) + (((x_sum_tmp_57455_68[2]) * (y_sum_tmp_57455_69[1]))))) + (((x_sum_tmp_57455_68[3]) * (y_sum_tmp_57455_69[0])))), ((((((((((x_sum_tmp_57455_68[0]) * (y_sum_tmp_57455_69[4]))) + (((x_sum_tmp_57455_68[1]) * (y_sum_tmp_57455_69[3]))))) + (((x_sum_tmp_57455_68[2]) * (y_sum_tmp_57455_69[2]))))) + (((x_sum_tmp_57455_68[3]) * (y_sum_tmp_57455_69[1]))))) + (((x_sum_tmp_57455_68[4]) * (y_sum_tmp_57455_69[0])))), ((((((((((((x_sum_tmp_57455_68[0]) * (y_sum_tmp_57455_69[5]))) + (((x_sum_tmp_57455_68[1]) * (y_sum_tmp_57455_69[4]))))) + (((x_sum_tmp_57455_68[2]) * (y_sum_tmp_57455_69[3]))))) + (((x_sum_tmp_57455_68[3]) * (y_sum_tmp_57455_69[2]))))) + (((x_sum_tmp_57455_68[4]) * (y_sum_tmp_57455_69[1]))))) + (((x_sum_tmp_57455_68[5]) * (y_sum_tmp_57455_69[0])))), ((((((((((((((x_sum_tmp_57455_68[0]) * (y_sum_tmp_57455_69[6]))) + (((x_sum_tmp_57455_68[1]) * (y_sum_tmp_57455_69[5]))))) + (((x_sum_tmp_57455_68[2]) * (y_sum_tmp_57455_69[4]))))) + (((x_sum_tmp_57455_68[3]) * (y_sum_tmp_57455_69[3]))))) + (((x_sum_tmp_57455_68[4]) * (y_sum_tmp_57455_69[2]))))) + (((x_sum_tmp_57455_68[5]) * (y_sum_tmp_57455_69[1]))))) + (((x_sum_tmp_57455_68[6]) * (y_sum_tmp_57455_69[0])))), ((((((((((((x_sum_tmp_57455_68[1]) * (y_sum_tmp_57455_69[6]))) + (((x_sum_tmp_57455_68[2]) * (y_sum_tmp_57455_69[5]))))) + (((x_sum_tmp_57455_68[3]) * (y_sum_tmp_57455_69[4]))))) + (((x_sum_tmp_57455_68[4]) * (y_sum_tmp_57455_69[3]))))) + (((x_sum_tmp_57455_68[5]) * (y_sum_tmp_57455_69[2]))))) + (((x_sum_tmp_57455_68[6]) * (y_sum_tmp_57455_69[1])))), ((((((((((x_sum_tmp_57455_68[2]) * (y_sum_tmp_57455_69[6]))) + (((x_sum_tmp_57455_68[3]) * (y_sum_tmp_57455_69[5]))))) + (((x_sum_tmp_57455_68[4]) * (y_sum_tmp_57455_69[4]))))) + (((x_sum_tmp_57455_68[5]) * (y_sum_tmp_57455_69[3]))))) + (((x_sum_tmp_57455_68[6]) * (y_sum_tmp_57455_69[2])))), ((((((((x_sum_tmp_57455_68[3]) * (y_sum_tmp_57455_69[6]))) + (((x_sum_tmp_57455_68[4]) * (y_sum_tmp_57455_69[5]))))) + (((x_sum_tmp_57455_68[5]) * (y_sum_tmp_57455_69[4]))))) + (((x_sum_tmp_57455_68[6]) * (y_sum_tmp_57455_69[3])))), ((((((x_sum_tmp_57455_68[4]) * (y_sum_tmp_57455_69[6]))) + (((x_sum_tmp_57455_68[5]) * (y_sum_tmp_57455_69[5]))))) + (((x_sum_tmp_57455_68[6]) * (y_sum_tmp_57455_69[4])))), ((((x_sum_tmp_57455_68[5]) * (y_sum_tmp_57455_69[6]))) + (((x_sum_tmp_57455_68[6]) * (y_sum_tmp_57455_69[5])))), ((x_sum_tmp_57455_68[6]) * (y_sum_tmp_57455_69[6]))];let z2_tmp_57455_71 = [((x_sum_tmp_57455_68[7]) * (y_sum_tmp_57455_69[7])), ((((x_sum_tmp_57455_68[7]) * (y_sum_tmp_57455_69[8]))) + (((x_sum_tmp_57455_68[8]) * (y_sum_tmp_57455_69[7])))), ((((((x_sum_tmp_57455_68[7]) * (y_sum_tmp_57455_69[9]))) + (((x_sum_tmp_57455_68[8]) * (y_sum_tmp_57455_69[8]))))) + (((x_sum_tmp_57455_68[9]) * (y_sum_tmp_57455_69[7])))), ((((((((x_sum_tmp_57455_68[7]) * (y_sum_tmp_57455_69[10]))) + (((x_sum_tmp_57455_68[8]) * (y_sum_tmp_57455_69[9]))))) + (((x_sum_tmp_57455_68[9]) * (y_sum_tmp_57455_69[8]))))) + (((x_sum_tmp_57455_68[10]) * (y_sum_tmp_57455_69[7])))), ((((((((((x_sum_tmp_57455_68[7]) * (y_sum_tmp_57455_69[11]))) + (((x_sum_tmp_57455_68[8]) * (y_sum_tmp_57455_69[10]))))) + (((x_sum_tmp_57455_68[9]) * (y_sum_tmp_57455_69[9]))))) + (((x_sum_tmp_57455_68[10]) * (y_sum_tmp_57455_69[8]))))) + (((x_sum_tmp_57455_68[11]) * (y_sum_tmp_57455_69[7])))), ((((((((((((x_sum_tmp_57455_68[7]) * (y_sum_tmp_57455_69[12]))) + (((x_sum_tmp_57455_68[8]) * (y_sum_tmp_57455_69[11]))))) + (((x_sum_tmp_57455_68[9]) * (y_sum_tmp_57455_69[10]))))) + (((x_sum_tmp_57455_68[10]) * (y_sum_tmp_57455_69[9]))))) + (((x_sum_tmp_57455_68[11]) * (y_sum_tmp_57455_69[8]))))) + (((x_sum_tmp_57455_68[12]) * (y_sum_tmp_57455_69[7])))), ((((((((((((((x_sum_tmp_57455_68[7]) * (y_sum_tmp_57455_69[13]))) + (((x_sum_tmp_57455_68[8]) * (y_sum_tmp_57455_69[12]))))) + (((x_sum_tmp_57455_68[9]) * (y_sum_tmp_57455_69[11]))))) + (((x_sum_tmp_57455_68[10]) * (y_sum_tmp_57455_69[10]))))) + (((x_sum_tmp_57455_68[11]) * (y_sum_tmp_57455_69[9]))))) + (((x_sum_tmp_57455_68[12]) * (y_sum_tmp_57455_69[8]))))) + (((x_sum_tmp_57455_68[13]) * (y_sum_tmp_57455_69[7])))), ((((((((((((x_sum_tmp_57455_68[8]) * (y_sum_tmp_57455_69[13]))) + (((x_sum_tmp_57455_68[9]) * (y_sum_tmp_57455_69[12]))))) + (((x_sum_tmp_57455_68[10]) * (y_sum_tmp_57455_69[11]))))) + (((x_sum_tmp_57455_68[11]) * (y_sum_tmp_57455_69[10]))))) + (((x_sum_tmp_57455_68[12]) * (y_sum_tmp_57455_69[9]))))) + (((x_sum_tmp_57455_68[13]) * (y_sum_tmp_57455_69[8])))), ((((((((((x_sum_tmp_57455_68[9]) * (y_sum_tmp_57455_69[13]))) + (((x_sum_tmp_57455_68[10]) * (y_sum_tmp_57455_69[12]))))) + (((x_sum_tmp_57455_68[11]) * (y_sum_tmp_57455_69[11]))))) + (((x_sum_tmp_57455_68[12]) * (y_sum_tmp_57455_69[10]))))) + (((x_sum_tmp_57455_68[13]) * (y_sum_tmp_57455_69[9])))), ((((((((x_sum_tmp_57455_68[10]) * (y_sum_tmp_57455_69[13]))) + (((x_sum_tmp_57455_68[11]) * (y_sum_tmp_57455_69[12]))))) + (((x_sum_tmp_57455_68[12]) * (y_sum_tmp_57455_69[11]))))) + (((x_sum_tmp_57455_68[13]) * (y_sum_tmp_57455_69[10])))), ((((((x_sum_tmp_57455_68[11]) * (y_sum_tmp_57455_69[13]))) + (((x_sum_tmp_57455_68[12]) * (y_sum_tmp_57455_69[12]))))) + (((x_sum_tmp_57455_68[13]) * (y_sum_tmp_57455_69[11])))), ((((x_sum_tmp_57455_68[12]) * (y_sum_tmp_57455_69[13]))) + (((x_sum_tmp_57455_68[13]) * (y_sum_tmp_57455_69[12])))), ((x_sum_tmp_57455_68[13]) * (y_sum_tmp_57455_69[13]))];let x_sum_tmp_57455_72 = [((x_sum_tmp_57455_68[0]) + (x_sum_tmp_57455_68[7])), ((x_sum_tmp_57455_68[1]) + (x_sum_tmp_57455_68[8])), ((x_sum_tmp_57455_68[2]) + (x_sum_tmp_57455_68[9])), ((x_sum_tmp_57455_68[3]) + (x_sum_tmp_57455_68[10])), ((x_sum_tmp_57455_68[4]) + (x_sum_tmp_57455_68[11])), ((x_sum_tmp_57455_68[5]) + (x_sum_tmp_57455_68[12])), ((x_sum_tmp_57455_68[6]) + (x_sum_tmp_57455_68[13]))];let y_sum_tmp_57455_73 = [((y_sum_tmp_57455_69[0]) + (y_sum_tmp_57455_69[7])), ((y_sum_tmp_57455_69[1]) + (y_sum_tmp_57455_69[8])), ((y_sum_tmp_57455_69[2]) + (y_sum_tmp_57455_69[9])), ((y_sum_tmp_57455_69[3]) + (y_sum_tmp_57455_69[10])), ((y_sum_tmp_57455_69[4]) + (y_sum_tmp_57455_69[11])), ((y_sum_tmp_57455_69[5]) + (y_sum_tmp_57455_69[12])), ((y_sum_tmp_57455_69[6]) + (y_sum_tmp_57455_69[13]))];let single_karatsuba_n_7_output_tmp_57455_74 = [z0_tmp_57455_70[0], z0_tmp_57455_70[1], z0_tmp_57455_70[2], z0_tmp_57455_70[3], z0_tmp_57455_70[4], z0_tmp_57455_70[5], z0_tmp_57455_70[6], ((z0_tmp_57455_70[7]) + (((((((x_sum_tmp_57455_72[0]) * (y_sum_tmp_57455_73[0]))) - (z0_tmp_57455_70[0]))) - (z2_tmp_57455_71[0])))), ((z0_tmp_57455_70[8]) + (((((((((x_sum_tmp_57455_72[0]) * (y_sum_tmp_57455_73[1]))) + (((x_sum_tmp_57455_72[1]) * (y_sum_tmp_57455_73[0]))))) - (z0_tmp_57455_70[1]))) - (z2_tmp_57455_71[1])))), ((z0_tmp_57455_70[9]) + (((((((((((x_sum_tmp_57455_72[0]) * (y_sum_tmp_57455_73[2]))) + (((x_sum_tmp_57455_72[1]) * (y_sum_tmp_57455_73[1]))))) + (((x_sum_tmp_57455_72[2]) * (y_sum_tmp_57455_73[0]))))) - (z0_tmp_57455_70[2]))) - (z2_tmp_57455_71[2])))), ((z0_tmp_57455_70[10]) + (((((((((((((x_sum_tmp_57455_72[0]) * (y_sum_tmp_57455_73[3]))) + (((x_sum_tmp_57455_72[1]) * (y_sum_tmp_57455_73[2]))))) + (((x_sum_tmp_57455_72[2]) * (y_sum_tmp_57455_73[1]))))) + (((x_sum_tmp_57455_72[3]) * (y_sum_tmp_57455_73[0]))))) - (z0_tmp_57455_70[3]))) - (z2_tmp_57455_71[3])))), ((z0_tmp_57455_70[11]) + (((((((((((((((x_sum_tmp_57455_72[0]) * (y_sum_tmp_57455_73[4]))) + (((x_sum_tmp_57455_72[1]) * (y_sum_tmp_57455_73[3]))))) + (((x_sum_tmp_57455_72[2]) * (y_sum_tmp_57455_73[2]))))) + (((x_sum_tmp_57455_72[3]) * (y_sum_tmp_57455_73[1]))))) + (((x_sum_tmp_57455_72[4]) * (y_sum_tmp_57455_73[0]))))) - (z0_tmp_57455_70[4]))) - (z2_tmp_57455_71[4])))), ((z0_tmp_57455_70[12]) + (((((((((((((((((x_sum_tmp_57455_72[0]) * (y_sum_tmp_57455_73[5]))) + (((x_sum_tmp_57455_72[1]) * (y_sum_tmp_57455_73[4]))))) + (((x_sum_tmp_57455_72[2]) * (y_sum_tmp_57455_73[3]))))) + (((x_sum_tmp_57455_72[3]) * (y_sum_tmp_57455_73[2]))))) + (((x_sum_tmp_57455_72[4]) * (y_sum_tmp_57455_73[1]))))) + (((x_sum_tmp_57455_72[5]) * (y_sum_tmp_57455_73[0]))))) - (z0_tmp_57455_70[5]))) - (z2_tmp_57455_71[5])))), ((((((((((((((((((x_sum_tmp_57455_72[0]) * (y_sum_tmp_57455_73[6]))) + (((x_sum_tmp_57455_72[1]) * (y_sum_tmp_57455_73[5]))))) + (((x_sum_tmp_57455_72[2]) * (y_sum_tmp_57455_73[4]))))) + (((x_sum_tmp_57455_72[3]) * (y_sum_tmp_57455_73[3]))))) + (((x_sum_tmp_57455_72[4]) * (y_sum_tmp_57455_73[2]))))) + (((x_sum_tmp_57455_72[5]) * (y_sum_tmp_57455_73[1]))))) + (((x_sum_tmp_57455_72[6]) * (y_sum_tmp_57455_73[0]))))) - (z0_tmp_57455_70[6]))) - (z2_tmp_57455_71[6])), ((z2_tmp_57455_71[0]) + (((((((((((((((((x_sum_tmp_57455_72[1]) * (y_sum_tmp_57455_73[6]))) + (((x_sum_tmp_57455_72[2]) * (y_sum_tmp_57455_73[5]))))) + (((x_sum_tmp_57455_72[3]) * (y_sum_tmp_57455_73[4]))))) + (((x_sum_tmp_57455_72[4]) * (y_sum_tmp_57455_73[3]))))) + (((x_sum_tmp_57455_72[5]) * (y_sum_tmp_57455_73[2]))))) + (((x_sum_tmp_57455_72[6]) * (y_sum_tmp_57455_73[1]))))) - (z0_tmp_57455_70[7]))) - (z2_tmp_57455_71[7])))), ((z2_tmp_57455_71[1]) + (((((((((((((((x_sum_tmp_57455_72[2]) * (y_sum_tmp_57455_73[6]))) + (((x_sum_tmp_57455_72[3]) * (y_sum_tmp_57455_73[5]))))) + (((x_sum_tmp_57455_72[4]) * (y_sum_tmp_57455_73[4]))))) + (((x_sum_tmp_57455_72[5]) * (y_sum_tmp_57455_73[3]))))) + (((x_sum_tmp_57455_72[6]) * (y_sum_tmp_57455_73[2]))))) - (z0_tmp_57455_70[8]))) - (z2_tmp_57455_71[8])))), ((z2_tmp_57455_71[2]) + (((((((((((((x_sum_tmp_57455_72[3]) * (y_sum_tmp_57455_73[6]))) + (((x_sum_tmp_57455_72[4]) * (y_sum_tmp_57455_73[5]))))) + (((x_sum_tmp_57455_72[5]) * (y_sum_tmp_57455_73[4]))))) + (((x_sum_tmp_57455_72[6]) * (y_sum_tmp_57455_73[3]))))) - (z0_tmp_57455_70[9]))) - (z2_tmp_57455_71[9])))), ((z2_tmp_57455_71[3]) + (((((((((((x_sum_tmp_57455_72[4]) * (y_sum_tmp_57455_73[6]))) + (((x_sum_tmp_57455_72[5]) * (y_sum_tmp_57455_73[5]))))) + (((x_sum_tmp_57455_72[6]) * (y_sum_tmp_57455_73[4]))))) - (z0_tmp_57455_70[10]))) - (z2_tmp_57455_71[10])))), ((z2_tmp_57455_71[4]) + (((((((((x_sum_tmp_57455_72[5]) * (y_sum_tmp_57455_73[6]))) + (((x_sum_tmp_57455_72[6]) * (y_sum_tmp_57455_73[5]))))) - (z0_tmp_57455_70[11]))) - (z2_tmp_57455_71[11])))), ((z2_tmp_57455_71[5]) + (((((((x_sum_tmp_57455_72[6]) * (y_sum_tmp_57455_73[6]))) - (z0_tmp_57455_70[12]))) - (z2_tmp_57455_71[12])))), z2_tmp_57455_71[6], z2_tmp_57455_71[7], z2_tmp_57455_71[8], z2_tmp_57455_71[9], z2_tmp_57455_71[10], z2_tmp_57455_71[11], z2_tmp_57455_71[12]];

            let double_karatsuba_1454b_output_tmp_57455_75 = [single_karatsuba_n_7_output_tmp_57455_62[0], single_karatsuba_n_7_output_tmp_57455_62[1], single_karatsuba_n_7_output_tmp_57455_62[2], single_karatsuba_n_7_output_tmp_57455_62[3], single_karatsuba_n_7_output_tmp_57455_62[4], single_karatsuba_n_7_output_tmp_57455_62[5], single_karatsuba_n_7_output_tmp_57455_62[6], single_karatsuba_n_7_output_tmp_57455_62[7], single_karatsuba_n_7_output_tmp_57455_62[8], single_karatsuba_n_7_output_tmp_57455_62[9], single_karatsuba_n_7_output_tmp_57455_62[10], single_karatsuba_n_7_output_tmp_57455_62[11], single_karatsuba_n_7_output_tmp_57455_62[12], single_karatsuba_n_7_output_tmp_57455_62[13], ((single_karatsuba_n_7_output_tmp_57455_62[14]) + (((((single_karatsuba_n_7_output_tmp_57455_74[0]) - (single_karatsuba_n_7_output_tmp_57455_62[0]))) - (single_karatsuba_n_7_output_tmp_57455_67[0])))), ((single_karatsuba_n_7_output_tmp_57455_62[15]) + (((((single_karatsuba_n_7_output_tmp_57455_74[1]) - (single_karatsuba_n_7_output_tmp_57455_62[1]))) - (single_karatsuba_n_7_output_tmp_57455_67[1])))), ((single_karatsuba_n_7_output_tmp_57455_62[16]) + (((((single_karatsuba_n_7_output_tmp_57455_74[2]) - (single_karatsuba_n_7_output_tmp_57455_62[2]))) - (single_karatsuba_n_7_output_tmp_57455_67[2])))), ((single_karatsuba_n_7_output_tmp_57455_62[17]) + (((((single_karatsuba_n_7_output_tmp_57455_74[3]) - (single_karatsuba_n_7_output_tmp_57455_62[3]))) - (single_karatsuba_n_7_output_tmp_57455_67[3])))), ((single_karatsuba_n_7_output_tmp_57455_62[18]) + (((((single_karatsuba_n_7_output_tmp_57455_74[4]) - (single_karatsuba_n_7_output_tmp_57455_62[4]))) - (single_karatsuba_n_7_output_tmp_57455_67[4])))), ((single_karatsuba_n_7_output_tmp_57455_62[19]) + (((((single_karatsuba_n_7_output_tmp_57455_74[5]) - (single_karatsuba_n_7_output_tmp_57455_62[5]))) - (single_karatsuba_n_7_output_tmp_57455_67[5])))), ((single_karatsuba_n_7_output_tmp_57455_62[20]) + (((((single_karatsuba_n_7_output_tmp_57455_74[6]) - (single_karatsuba_n_7_output_tmp_57455_62[6]))) - (single_karatsuba_n_7_output_tmp_57455_67[6])))), ((single_karatsuba_n_7_output_tmp_57455_62[21]) + (((((single_karatsuba_n_7_output_tmp_57455_74[7]) - (single_karatsuba_n_7_output_tmp_57455_62[7]))) - (single_karatsuba_n_7_output_tmp_57455_67[7])))), ((single_karatsuba_n_7_output_tmp_57455_62[22]) + (((((single_karatsuba_n_7_output_tmp_57455_74[8]) - (single_karatsuba_n_7_output_tmp_57455_62[8]))) - (single_karatsuba_n_7_output_tmp_57455_67[8])))), ((single_karatsuba_n_7_output_tmp_57455_62[23]) + (((((single_karatsuba_n_7_output_tmp_57455_74[9]) - (single_karatsuba_n_7_output_tmp_57455_62[9]))) - (single_karatsuba_n_7_output_tmp_57455_67[9])))), ((single_karatsuba_n_7_output_tmp_57455_62[24]) + (((((single_karatsuba_n_7_output_tmp_57455_74[10]) - (single_karatsuba_n_7_output_tmp_57455_62[10]))) - (single_karatsuba_n_7_output_tmp_57455_67[10])))), ((single_karatsuba_n_7_output_tmp_57455_62[25]) + (((((single_karatsuba_n_7_output_tmp_57455_74[11]) - (single_karatsuba_n_7_output_tmp_57455_62[11]))) - (single_karatsuba_n_7_output_tmp_57455_67[11])))), ((single_karatsuba_n_7_output_tmp_57455_62[26]) + (((((single_karatsuba_n_7_output_tmp_57455_74[12]) - (single_karatsuba_n_7_output_tmp_57455_62[12]))) - (single_karatsuba_n_7_output_tmp_57455_67[12])))), ((((single_karatsuba_n_7_output_tmp_57455_74[13]) - (single_karatsuba_n_7_output_tmp_57455_62[13]))) - (single_karatsuba_n_7_output_tmp_57455_67[13])), ((single_karatsuba_n_7_output_tmp_57455_67[0]) + (((((single_karatsuba_n_7_output_tmp_57455_74[14]) - (single_karatsuba_n_7_output_tmp_57455_62[14]))) - (single_karatsuba_n_7_output_tmp_57455_67[14])))), ((single_karatsuba_n_7_output_tmp_57455_67[1]) + (((((single_karatsuba_n_7_output_tmp_57455_74[15]) - (single_karatsuba_n_7_output_tmp_57455_62[15]))) - (single_karatsuba_n_7_output_tmp_57455_67[15])))), ((single_karatsuba_n_7_output_tmp_57455_67[2]) + (((((single_karatsuba_n_7_output_tmp_57455_74[16]) - (single_karatsuba_n_7_output_tmp_57455_62[16]))) - (single_karatsuba_n_7_output_tmp_57455_67[16])))), ((single_karatsuba_n_7_output_tmp_57455_67[3]) + (((((single_karatsuba_n_7_output_tmp_57455_74[17]) - (single_karatsuba_n_7_output_tmp_57455_62[17]))) - (single_karatsuba_n_7_output_tmp_57455_67[17])))), ((single_karatsuba_n_7_output_tmp_57455_67[4]) + (((((single_karatsuba_n_7_output_tmp_57455_74[18]) - (single_karatsuba_n_7_output_tmp_57455_62[18]))) - (single_karatsuba_n_7_output_tmp_57455_67[18])))), ((single_karatsuba_n_7_output_tmp_57455_67[5]) + (((((single_karatsuba_n_7_output_tmp_57455_74[19]) - (single_karatsuba_n_7_output_tmp_57455_62[19]))) - (single_karatsuba_n_7_output_tmp_57455_67[19])))), ((single_karatsuba_n_7_output_tmp_57455_67[6]) + (((((single_karatsuba_n_7_output_tmp_57455_74[20]) - (single_karatsuba_n_7_output_tmp_57455_62[20]))) - (single_karatsuba_n_7_output_tmp_57455_67[20])))), ((single_karatsuba_n_7_output_tmp_57455_67[7]) + (((((single_karatsuba_n_7_output_tmp_57455_74[21]) - (single_karatsuba_n_7_output_tmp_57455_62[21]))) - (single_karatsuba_n_7_output_tmp_57455_67[21])))), ((single_karatsuba_n_7_output_tmp_57455_67[8]) + (((((single_karatsuba_n_7_output_tmp_57455_74[22]) - (single_karatsuba_n_7_output_tmp_57455_62[22]))) - (single_karatsuba_n_7_output_tmp_57455_67[22])))), ((single_karatsuba_n_7_output_tmp_57455_67[9]) + (((((single_karatsuba_n_7_output_tmp_57455_74[23]) - (single_karatsuba_n_7_output_tmp_57455_62[23]))) - (single_karatsuba_n_7_output_tmp_57455_67[23])))), ((single_karatsuba_n_7_output_tmp_57455_67[10]) + (((((single_karatsuba_n_7_output_tmp_57455_74[24]) - (single_karatsuba_n_7_output_tmp_57455_62[24]))) - (single_karatsuba_n_7_output_tmp_57455_67[24])))), ((single_karatsuba_n_7_output_tmp_57455_67[11]) + (((((single_karatsuba_n_7_output_tmp_57455_74[25]) - (single_karatsuba_n_7_output_tmp_57455_62[25]))) - (single_karatsuba_n_7_output_tmp_57455_67[25])))), ((single_karatsuba_n_7_output_tmp_57455_67[12]) + (((((single_karatsuba_n_7_output_tmp_57455_74[26]) - (single_karatsuba_n_7_output_tmp_57455_62[26]))) - (single_karatsuba_n_7_output_tmp_57455_67[26])))), single_karatsuba_n_7_output_tmp_57455_67[13], single_karatsuba_n_7_output_tmp_57455_67[14], single_karatsuba_n_7_output_tmp_57455_67[15], single_karatsuba_n_7_output_tmp_57455_67[16], single_karatsuba_n_7_output_tmp_57455_67[17], single_karatsuba_n_7_output_tmp_57455_67[18], single_karatsuba_n_7_output_tmp_57455_67[19], single_karatsuba_n_7_output_tmp_57455_67[20], single_karatsuba_n_7_output_tmp_57455_67[21], single_karatsuba_n_7_output_tmp_57455_67[22], single_karatsuba_n_7_output_tmp_57455_67[23], single_karatsuba_n_7_output_tmp_57455_67[24], single_karatsuba_n_7_output_tmp_57455_67[25], single_karatsuba_n_7_output_tmp_57455_67[26]];

            let conv_tmp_57455_76 = [((double_karatsuba_1454b_output_tmp_57455_75[0]) - (mul_res_limb_0_col141)), ((double_karatsuba_1454b_output_tmp_57455_75[1]) - (mul_res_limb_1_col142)), ((double_karatsuba_1454b_output_tmp_57455_75[2]) - (mul_res_limb_2_col143)), ((double_karatsuba_1454b_output_tmp_57455_75[3]) - (mul_res_limb_3_col144)), ((double_karatsuba_1454b_output_tmp_57455_75[4]) - (mul_res_limb_4_col145)), ((double_karatsuba_1454b_output_tmp_57455_75[5]) - (mul_res_limb_5_col146)), ((double_karatsuba_1454b_output_tmp_57455_75[6]) - (mul_res_limb_6_col147)), ((double_karatsuba_1454b_output_tmp_57455_75[7]) - (mul_res_limb_7_col148)), ((double_karatsuba_1454b_output_tmp_57455_75[8]) - (mul_res_limb_8_col149)), ((double_karatsuba_1454b_output_tmp_57455_75[9]) - (mul_res_limb_9_col150)), ((double_karatsuba_1454b_output_tmp_57455_75[10]) - (mul_res_limb_10_col151)), ((double_karatsuba_1454b_output_tmp_57455_75[11]) - (mul_res_limb_11_col152)), ((double_karatsuba_1454b_output_tmp_57455_75[12]) - (mul_res_limb_12_col153)), ((double_karatsuba_1454b_output_tmp_57455_75[13]) - (mul_res_limb_13_col154)), ((double_karatsuba_1454b_output_tmp_57455_75[14]) - (mul_res_limb_14_col155)), ((double_karatsuba_1454b_output_tmp_57455_75[15]) - (mul_res_limb_15_col156)), ((double_karatsuba_1454b_output_tmp_57455_75[16]) - (mul_res_limb_16_col157)), ((double_karatsuba_1454b_output_tmp_57455_75[17]) - (mul_res_limb_17_col158)), ((double_karatsuba_1454b_output_tmp_57455_75[18]) - (mul_res_limb_18_col159)), ((double_karatsuba_1454b_output_tmp_57455_75[19]) - (mul_res_limb_19_col160)), ((double_karatsuba_1454b_output_tmp_57455_75[20]) - (mul_res_limb_20_col161)), ((double_karatsuba_1454b_output_tmp_57455_75[21]) - (mul_res_limb_21_col162)), ((double_karatsuba_1454b_output_tmp_57455_75[22]) - (mul_res_limb_22_col163)), ((double_karatsuba_1454b_output_tmp_57455_75[23]) - (mul_res_limb_23_col164)), ((double_karatsuba_1454b_output_tmp_57455_75[24]) - (mul_res_limb_24_col165)), ((double_karatsuba_1454b_output_tmp_57455_75[25]) - (mul_res_limb_25_col166)), ((double_karatsuba_1454b_output_tmp_57455_75[26]) - (mul_res_limb_26_col167)), ((double_karatsuba_1454b_output_tmp_57455_75[27]) - (mul_res_limb_27_col168)), double_karatsuba_1454b_output_tmp_57455_75[28], double_karatsuba_1454b_output_tmp_57455_75[29], double_karatsuba_1454b_output_tmp_57455_75[30], double_karatsuba_1454b_output_tmp_57455_75[31], double_karatsuba_1454b_output_tmp_57455_75[32], double_karatsuba_1454b_output_tmp_57455_75[33], double_karatsuba_1454b_output_tmp_57455_75[34], double_karatsuba_1454b_output_tmp_57455_75[35], double_karatsuba_1454b_output_tmp_57455_75[36], double_karatsuba_1454b_output_tmp_57455_75[37], double_karatsuba_1454b_output_tmp_57455_75[38], double_karatsuba_1454b_output_tmp_57455_75[39], double_karatsuba_1454b_output_tmp_57455_75[40], double_karatsuba_1454b_output_tmp_57455_75[41], double_karatsuba_1454b_output_tmp_57455_75[42], double_karatsuba_1454b_output_tmp_57455_75[43], double_karatsuba_1454b_output_tmp_57455_75[44], double_karatsuba_1454b_output_tmp_57455_75[45], double_karatsuba_1454b_output_tmp_57455_75[46], double_karatsuba_1454b_output_tmp_57455_75[47], double_karatsuba_1454b_output_tmp_57455_75[48], double_karatsuba_1454b_output_tmp_57455_75[49], double_karatsuba_1454b_output_tmp_57455_75[50], double_karatsuba_1454b_output_tmp_57455_75[51], double_karatsuba_1454b_output_tmp_57455_75[52], double_karatsuba_1454b_output_tmp_57455_75[53], double_karatsuba_1454b_output_tmp_57455_75[54]];let conv_mod_tmp_57455_77 = [((((((M31_32) * (conv_tmp_57455_76[0]))) - (((M31_4) * (conv_tmp_57455_76[21]))))) + (((M31_8) * (conv_tmp_57455_76[49])))), ((((((conv_tmp_57455_76[0]) + (((M31_32) * (conv_tmp_57455_76[1]))))) - (((M31_4) * (conv_tmp_57455_76[22]))))) + (((M31_8) * (conv_tmp_57455_76[50])))), ((((((conv_tmp_57455_76[1]) + (((M31_32) * (conv_tmp_57455_76[2]))))) - (((M31_4) * (conv_tmp_57455_76[23]))))) + (((M31_8) * (conv_tmp_57455_76[51])))), ((((((conv_tmp_57455_76[2]) + (((M31_32) * (conv_tmp_57455_76[3]))))) - (((M31_4) * (conv_tmp_57455_76[24]))))) + (((M31_8) * (conv_tmp_57455_76[52])))), ((((((conv_tmp_57455_76[3]) + (((M31_32) * (conv_tmp_57455_76[4]))))) - (((M31_4) * (conv_tmp_57455_76[25]))))) + (((M31_8) * (conv_tmp_57455_76[53])))), ((((((conv_tmp_57455_76[4]) + (((M31_32) * (conv_tmp_57455_76[5]))))) - (((M31_4) * (conv_tmp_57455_76[26]))))) + (((M31_8) * (conv_tmp_57455_76[54])))), ((((conv_tmp_57455_76[5]) + (((M31_32) * (conv_tmp_57455_76[6]))))) - (((M31_4) * (conv_tmp_57455_76[27])))), ((((((((M31_2) * (conv_tmp_57455_76[0]))) + (conv_tmp_57455_76[6]))) + (((M31_32) * (conv_tmp_57455_76[7]))))) - (((M31_4) * (conv_tmp_57455_76[28])))), ((((((((M31_2) * (conv_tmp_57455_76[1]))) + (conv_tmp_57455_76[7]))) + (((M31_32) * (conv_tmp_57455_76[8]))))) - (((M31_4) * (conv_tmp_57455_76[29])))), ((((((((M31_2) * (conv_tmp_57455_76[2]))) + (conv_tmp_57455_76[8]))) + (((M31_32) * (conv_tmp_57455_76[9]))))) - (((M31_4) * (conv_tmp_57455_76[30])))), ((((((((M31_2) * (conv_tmp_57455_76[3]))) + (conv_tmp_57455_76[9]))) + (((M31_32) * (conv_tmp_57455_76[10]))))) - (((M31_4) * (conv_tmp_57455_76[31])))), ((((((((M31_2) * (conv_tmp_57455_76[4]))) + (conv_tmp_57455_76[10]))) + (((M31_32) * (conv_tmp_57455_76[11]))))) - (((M31_4) * (conv_tmp_57455_76[32])))), ((((((((M31_2) * (conv_tmp_57455_76[5]))) + (conv_tmp_57455_76[11]))) + (((M31_32) * (conv_tmp_57455_76[12]))))) - (((M31_4) * (conv_tmp_57455_76[33])))), ((((((((M31_2) * (conv_tmp_57455_76[6]))) + (conv_tmp_57455_76[12]))) + (((M31_32) * (conv_tmp_57455_76[13]))))) - (((M31_4) * (conv_tmp_57455_76[34])))), ((((((((M31_2) * (conv_tmp_57455_76[7]))) + (conv_tmp_57455_76[13]))) + (((M31_32) * (conv_tmp_57455_76[14]))))) - (((M31_4) * (conv_tmp_57455_76[35])))), ((((((((M31_2) * (conv_tmp_57455_76[8]))) + (conv_tmp_57455_76[14]))) + (((M31_32) * (conv_tmp_57455_76[15]))))) - (((M31_4) * (conv_tmp_57455_76[36])))), ((((((((M31_2) * (conv_tmp_57455_76[9]))) + (conv_tmp_57455_76[15]))) + (((M31_32) * (conv_tmp_57455_76[16]))))) - (((M31_4) * (conv_tmp_57455_76[37])))), ((((((((M31_2) * (conv_tmp_57455_76[10]))) + (conv_tmp_57455_76[16]))) + (((M31_32) * (conv_tmp_57455_76[17]))))) - (((M31_4) * (conv_tmp_57455_76[38])))), ((((((((M31_2) * (conv_tmp_57455_76[11]))) + (conv_tmp_57455_76[17]))) + (((M31_32) * (conv_tmp_57455_76[18]))))) - (((M31_4) * (conv_tmp_57455_76[39])))), ((((((((M31_2) * (conv_tmp_57455_76[12]))) + (conv_tmp_57455_76[18]))) + (((M31_32) * (conv_tmp_57455_76[19]))))) - (((M31_4) * (conv_tmp_57455_76[40])))), ((((((((M31_2) * (conv_tmp_57455_76[13]))) + (conv_tmp_57455_76[19]))) + (((M31_32) * (conv_tmp_57455_76[20]))))) - (((M31_4) * (conv_tmp_57455_76[41])))), ((((((((M31_2) * (conv_tmp_57455_76[14]))) + (conv_tmp_57455_76[20]))) - (((M31_4) * (conv_tmp_57455_76[42]))))) + (((M31_64) * (conv_tmp_57455_76[49])))), ((((((((M31_2) * (conv_tmp_57455_76[15]))) - (((M31_4) * (conv_tmp_57455_76[43]))))) + (((M31_2) * (conv_tmp_57455_76[49]))))) + (((M31_64) * (conv_tmp_57455_76[50])))), ((((((((M31_2) * (conv_tmp_57455_76[16]))) - (((M31_4) * (conv_tmp_57455_76[44]))))) + (((M31_2) * (conv_tmp_57455_76[50]))))) + (((M31_64) * (conv_tmp_57455_76[51])))), ((((((((M31_2) * (conv_tmp_57455_76[17]))) - (((M31_4) * (conv_tmp_57455_76[45]))))) + (((M31_2) * (conv_tmp_57455_76[51]))))) + (((M31_64) * (conv_tmp_57455_76[52])))), ((((((((M31_2) * (conv_tmp_57455_76[18]))) - (((M31_4) * (conv_tmp_57455_76[46]))))) + (((M31_2) * (conv_tmp_57455_76[52]))))) + (((M31_64) * (conv_tmp_57455_76[53])))), ((((((((M31_2) * (conv_tmp_57455_76[19]))) - (((M31_4) * (conv_tmp_57455_76[47]))))) + (((M31_2) * (conv_tmp_57455_76[53]))))) + (((M31_64) * (conv_tmp_57455_76[54])))), ((((((M31_2) * (conv_tmp_57455_76[20]))) - (((M31_4) * (conv_tmp_57455_76[48]))))) + (((M31_2) * (conv_tmp_57455_76[54]))))];let k_mod_2_18_biased_tmp_57455_78 = ((((((PackedUInt32::from_m31(((conv_mod_tmp_57455_77[0]) + (M31_134217728)))) + (((((PackedUInt32::from_m31(((conv_mod_tmp_57455_77[1]) + (M31_134217728)))) & (UInt32_511))) << (UInt32_9))))) + (UInt32_131072))) & (UInt32_262143));let k_col169 = ((k_mod_2_18_biased_tmp_57455_78.low().as_m31()) + (((((k_mod_2_18_biased_tmp_57455_78.high().as_m31()) - (M31_2))) * (M31_65536))));
            *row[169] = k_col169;*sub_component_inputs.range_check_20[0] =
                [((k_col169) + (M31_524288))];
            *lookup_data.range_check_20_0 = [((k_col169) + (M31_524288))];let carry_0_col170 = ((((conv_mod_tmp_57455_77[0]) - (k_col169))) * (M31_4194304));
            *row[170] = carry_0_col170;*sub_component_inputs.range_check_20_b[0] =
                [((carry_0_col170) + (M31_524288))];
            *lookup_data.range_check_20_b_0 = [((carry_0_col170) + (M31_524288))];let carry_1_col171 = ((((conv_mod_tmp_57455_77[1]) + (carry_0_col170))) * (M31_4194304));
            *row[171] = carry_1_col171;*sub_component_inputs.range_check_20_c[0] =
                [((carry_1_col171) + (M31_524288))];
            *lookup_data.range_check_20_c_0 = [((carry_1_col171) + (M31_524288))];let carry_2_col172 = ((((conv_mod_tmp_57455_77[2]) + (carry_1_col171))) * (M31_4194304));
            *row[172] = carry_2_col172;*sub_component_inputs.range_check_20_d[0] =
                [((carry_2_col172) + (M31_524288))];
            *lookup_data.range_check_20_d_0 = [((carry_2_col172) + (M31_524288))];let carry_3_col173 = ((((conv_mod_tmp_57455_77[3]) + (carry_2_col172))) * (M31_4194304));
            *row[173] = carry_3_col173;*sub_component_inputs.range_check_20_e[0] =
                [((carry_3_col173) + (M31_524288))];
            *lookup_data.range_check_20_e_0 = [((carry_3_col173) + (M31_524288))];let carry_4_col174 = ((((conv_mod_tmp_57455_77[4]) + (carry_3_col173))) * (M31_4194304));
            *row[174] = carry_4_col174;*sub_component_inputs.range_check_20_f[0] =
                [((carry_4_col174) + (M31_524288))];
            *lookup_data.range_check_20_f_0 = [((carry_4_col174) + (M31_524288))];let carry_5_col175 = ((((conv_mod_tmp_57455_77[5]) + (carry_4_col174))) * (M31_4194304));
            *row[175] = carry_5_col175;*sub_component_inputs.range_check_20_g[0] =
                [((carry_5_col175) + (M31_524288))];
            *lookup_data.range_check_20_g_0 = [((carry_5_col175) + (M31_524288))];let carry_6_col176 = ((((conv_mod_tmp_57455_77[6]) + (carry_5_col175))) * (M31_4194304));
            *row[176] = carry_6_col176;*sub_component_inputs.range_check_20_h[0] =
                [((carry_6_col176) + (M31_524288))];
            *lookup_data.range_check_20_h_0 = [((carry_6_col176) + (M31_524288))];let carry_7_col177 = ((((conv_mod_tmp_57455_77[7]) + (carry_6_col176))) * (M31_4194304));
            *row[177] = carry_7_col177;*sub_component_inputs.range_check_20[1] =
                [((carry_7_col177) + (M31_524288))];
            *lookup_data.range_check_20_1 = [((carry_7_col177) + (M31_524288))];let carry_8_col178 = ((((conv_mod_tmp_57455_77[8]) + (carry_7_col177))) * (M31_4194304));
            *row[178] = carry_8_col178;*sub_component_inputs.range_check_20_b[1] =
                [((carry_8_col178) + (M31_524288))];
            *lookup_data.range_check_20_b_1 = [((carry_8_col178) + (M31_524288))];let carry_9_col179 = ((((conv_mod_tmp_57455_77[9]) + (carry_8_col178))) * (M31_4194304));
            *row[179] = carry_9_col179;*sub_component_inputs.range_check_20_c[1] =
                [((carry_9_col179) + (M31_524288))];
            *lookup_data.range_check_20_c_1 = [((carry_9_col179) + (M31_524288))];let carry_10_col180 = ((((conv_mod_tmp_57455_77[10]) + (carry_9_col179))) * (M31_4194304));
            *row[180] = carry_10_col180;*sub_component_inputs.range_check_20_d[1] =
                [((carry_10_col180) + (M31_524288))];
            *lookup_data.range_check_20_d_1 = [((carry_10_col180) + (M31_524288))];let carry_11_col181 = ((((conv_mod_tmp_57455_77[11]) + (carry_10_col180))) * (M31_4194304));
            *row[181] = carry_11_col181;*sub_component_inputs.range_check_20_e[1] =
                [((carry_11_col181) + (M31_524288))];
            *lookup_data.range_check_20_e_1 = [((carry_11_col181) + (M31_524288))];let carry_12_col182 = ((((conv_mod_tmp_57455_77[12]) + (carry_11_col181))) * (M31_4194304));
            *row[182] = carry_12_col182;*sub_component_inputs.range_check_20_f[1] =
                [((carry_12_col182) + (M31_524288))];
            *lookup_data.range_check_20_f_1 = [((carry_12_col182) + (M31_524288))];let carry_13_col183 = ((((conv_mod_tmp_57455_77[13]) + (carry_12_col182))) * (M31_4194304));
            *row[183] = carry_13_col183;*sub_component_inputs.range_check_20_g[1] =
                [((carry_13_col183) + (M31_524288))];
            *lookup_data.range_check_20_g_1 = [((carry_13_col183) + (M31_524288))];let carry_14_col184 = ((((conv_mod_tmp_57455_77[14]) + (carry_13_col183))) * (M31_4194304));
            *row[184] = carry_14_col184;*sub_component_inputs.range_check_20_h[1] =
                [((carry_14_col184) + (M31_524288))];
            *lookup_data.range_check_20_h_1 = [((carry_14_col184) + (M31_524288))];let carry_15_col185 = ((((conv_mod_tmp_57455_77[15]) + (carry_14_col184))) * (M31_4194304));
            *row[185] = carry_15_col185;*sub_component_inputs.range_check_20[2] =
                [((carry_15_col185) + (M31_524288))];
            *lookup_data.range_check_20_2 = [((carry_15_col185) + (M31_524288))];let carry_16_col186 = ((((conv_mod_tmp_57455_77[16]) + (carry_15_col185))) * (M31_4194304));
            *row[186] = carry_16_col186;*sub_component_inputs.range_check_20_b[2] =
                [((carry_16_col186) + (M31_524288))];
            *lookup_data.range_check_20_b_2 = [((carry_16_col186) + (M31_524288))];let carry_17_col187 = ((((conv_mod_tmp_57455_77[17]) + (carry_16_col186))) * (M31_4194304));
            *row[187] = carry_17_col187;*sub_component_inputs.range_check_20_c[2] =
                [((carry_17_col187) + (M31_524288))];
            *lookup_data.range_check_20_c_2 = [((carry_17_col187) + (M31_524288))];let carry_18_col188 = ((((conv_mod_tmp_57455_77[18]) + (carry_17_col187))) * (M31_4194304));
            *row[188] = carry_18_col188;*sub_component_inputs.range_check_20_d[2] =
                [((carry_18_col188) + (M31_524288))];
            *lookup_data.range_check_20_d_2 = [((carry_18_col188) + (M31_524288))];let carry_19_col189 = ((((conv_mod_tmp_57455_77[19]) + (carry_18_col188))) * (M31_4194304));
            *row[189] = carry_19_col189;*sub_component_inputs.range_check_20_e[2] =
                [((carry_19_col189) + (M31_524288))];
            *lookup_data.range_check_20_e_2 = [((carry_19_col189) + (M31_524288))];let carry_20_col190 = ((((conv_mod_tmp_57455_77[20]) + (carry_19_col189))) * (M31_4194304));
            *row[190] = carry_20_col190;*sub_component_inputs.range_check_20_f[2] =
                [((carry_20_col190) + (M31_524288))];
            *lookup_data.range_check_20_f_2 = [((carry_20_col190) + (M31_524288))];let carry_21_col191 = ((((((conv_mod_tmp_57455_77[21]) - (((M31_136) * (k_col169))))) + (carry_20_col190))) * (M31_4194304));
            *row[191] = carry_21_col191;*sub_component_inputs.range_check_20_g[2] =
                [((carry_21_col191) + (M31_524288))];
            *lookup_data.range_check_20_g_2 = [((carry_21_col191) + (M31_524288))];let carry_22_col192 = ((((conv_mod_tmp_57455_77[22]) + (carry_21_col191))) * (M31_4194304));
            *row[192] = carry_22_col192;*sub_component_inputs.range_check_20_h[2] =
                [((carry_22_col192) + (M31_524288))];
            *lookup_data.range_check_20_h_2 = [((carry_22_col192) + (M31_524288))];let carry_23_col193 = ((((conv_mod_tmp_57455_77[23]) + (carry_22_col192))) * (M31_4194304));
            *row[193] = carry_23_col193;*sub_component_inputs.range_check_20[3] =
                [((carry_23_col193) + (M31_524288))];
            *lookup_data.range_check_20_3 = [((carry_23_col193) + (M31_524288))];let carry_24_col194 = ((((conv_mod_tmp_57455_77[24]) + (carry_23_col193))) * (M31_4194304));
            *row[194] = carry_24_col194;*sub_component_inputs.range_check_20_b[3] =
                [((carry_24_col194) + (M31_524288))];
            *lookup_data.range_check_20_b_3 = [((carry_24_col194) + (M31_524288))];let carry_25_col195 = ((((conv_mod_tmp_57455_77[25]) + (carry_24_col194))) * (M31_4194304));
            *row[195] = carry_25_col195;*sub_component_inputs.range_check_20_c[3] =
                [((carry_25_col195) + (M31_524288))];
            *lookup_data.range_check_20_c_3 = [((carry_25_col195) + (M31_524288))];let carry_26_col196 = ((((conv_mod_tmp_57455_77[26]) + (carry_25_col195))) * (M31_4194304));
            *row[196] = carry_26_col196;*sub_component_inputs.range_check_20_d[3] =
                [((carry_26_col196) + (M31_524288))];
            *lookup_data.range_check_20_d_3 = [((carry_26_col196) + (M31_524288))];

            let mul_252_output_tmp_57455_79 = mul_res_tmp_57455_57;

            let res_tmp_57455_80 = ((((((PackedFelt252::from_m31(decode_generic_instruction_output_tmp_57455_26.0[16])) * (read_positive_num_bits_252_output_tmp_57455_44.0))) + (((PackedFelt252::from_m31(res_mul_col12)) * (mul_252_output_tmp_57455_79))))) + (((PackedFelt252::from_m31(res_add_col11)) * (add_252_output_tmp_57455_56))));let res_limb_0_col197 = res_tmp_57455_80.get_m31(0);
            *row[197] = res_limb_0_col197;let res_limb_1_col198 = res_tmp_57455_80.get_m31(1);
            *row[198] = res_limb_1_col198;let res_limb_2_col199 = res_tmp_57455_80.get_m31(2);
            *row[199] = res_limb_2_col199;let res_limb_3_col200 = res_tmp_57455_80.get_m31(3);
            *row[200] = res_limb_3_col200;let res_limb_4_col201 = res_tmp_57455_80.get_m31(4);
            *row[201] = res_limb_4_col201;let res_limb_5_col202 = res_tmp_57455_80.get_m31(5);
            *row[202] = res_limb_5_col202;let res_limb_6_col203 = res_tmp_57455_80.get_m31(6);
            *row[203] = res_limb_6_col203;let res_limb_7_col204 = res_tmp_57455_80.get_m31(7);
            *row[204] = res_limb_7_col204;let res_limb_8_col205 = res_tmp_57455_80.get_m31(8);
            *row[205] = res_limb_8_col205;let res_limb_9_col206 = res_tmp_57455_80.get_m31(9);
            *row[206] = res_limb_9_col206;let res_limb_10_col207 = res_tmp_57455_80.get_m31(10);
            *row[207] = res_limb_10_col207;let res_limb_11_col208 = res_tmp_57455_80.get_m31(11);
            *row[208] = res_limb_11_col208;let res_limb_12_col209 = res_tmp_57455_80.get_m31(12);
            *row[209] = res_limb_12_col209;let res_limb_13_col210 = res_tmp_57455_80.get_m31(13);
            *row[210] = res_limb_13_col210;let res_limb_14_col211 = res_tmp_57455_80.get_m31(14);
            *row[211] = res_limb_14_col211;let res_limb_15_col212 = res_tmp_57455_80.get_m31(15);
            *row[212] = res_limb_15_col212;let res_limb_16_col213 = res_tmp_57455_80.get_m31(16);
            *row[213] = res_limb_16_col213;let res_limb_17_col214 = res_tmp_57455_80.get_m31(17);
            *row[214] = res_limb_17_col214;let res_limb_18_col215 = res_tmp_57455_80.get_m31(18);
            *row[215] = res_limb_18_col215;let res_limb_19_col216 = res_tmp_57455_80.get_m31(19);
            *row[216] = res_limb_19_col216;let res_limb_20_col217 = res_tmp_57455_80.get_m31(20);
            *row[217] = res_limb_20_col217;let res_limb_21_col218 = res_tmp_57455_80.get_m31(21);
            *row[218] = res_limb_21_col218;let res_limb_22_col219 = res_tmp_57455_80.get_m31(22);
            *row[219] = res_limb_22_col219;let res_limb_23_col220 = res_tmp_57455_80.get_m31(23);
            *row[220] = res_limb_23_col220;let res_limb_24_col221 = res_tmp_57455_80.get_m31(24);
            *row[221] = res_limb_24_col221;let res_limb_25_col222 = res_tmp_57455_80.get_m31(25);
            *row[222] = res_limb_25_col222;let res_limb_26_col223 = res_tmp_57455_80.get_m31(26);
            *row[223] = res_limb_26_col223;let res_limb_27_col224 = res_tmp_57455_80.get_m31(27);
            *row[224] = res_limb_27_col224;let eval_operands_output_tmp_57455_82 = [read_positive_num_bits_252_output_tmp_57455_31.0, read_positive_num_bits_252_output_tmp_57455_36.0, read_positive_num_bits_252_output_tmp_57455_44.0, res_tmp_57455_80];

            // Handle Opcodes.

            // Cond Felt 252 As Addr.

            // Cond Range Check 2.

            let partial_limb_msb_tmp_57455_83 = ((((PackedUInt16::from_m31(dst_limb_3_col26)) & (UInt16_2))) >> (UInt16_1));let partial_limb_msb_col225 = partial_limb_msb_tmp_57455_83.as_m31();
            *row[225] = partial_limb_msb_col225;

            let cond_felt_252_as_addr_output_tmp_57455_85 = ((((((dst_limb_0_col23) + (((dst_limb_1_col24) * (M31_512))))) + (((dst_limb_2_col25) * (M31_262144))))) + (((dst_limb_3_col26) * (M31_134217728))));

            // Cond Felt 252 As Addr.

            // Cond Range Check 2.

            let partial_limb_msb_tmp_57455_86 = ((((PackedUInt16::from_m31(op0_limb_3_col56)) & (UInt16_2))) >> (UInt16_1));let partial_limb_msb_col226 = partial_limb_msb_tmp_57455_86.as_m31();
            *row[226] = partial_limb_msb_col226;

            let cond_felt_252_as_addr_output_tmp_57455_88 = ((((((op0_limb_0_col53) + (((op0_limb_1_col54) * (M31_512))))) + (((op0_limb_2_col55) * (M31_262144))))) + (((op0_limb_3_col56) * (M31_134217728))));

            // Update Registers.

            // Cond Felt 252 As Addr.

            // Cond Range Check 2.

            let partial_limb_msb_tmp_57455_89 = ((((PackedUInt16::from_m31(res_limb_3_col200)) & (UInt16_2))) >> (UInt16_1));let partial_limb_msb_col227 = partial_limb_msb_tmp_57455_89.as_m31();
            *row[227] = partial_limb_msb_col227;

            let cond_felt_252_as_addr_output_tmp_57455_91 = ((((((res_limb_0_col197) + (((res_limb_1_col198) * (M31_512))))) + (((res_limb_2_col199) * (M31_262144))))) + (((res_limb_3_col200) * (M31_134217728))));

            // Cond Felt 252 As Addr.

            // Cond Range Check 2.

            let partial_limb_msb_tmp_57455_92 = ((((PackedUInt16::from_m31(dst_limb_3_col26)) & (UInt16_2))) >> (UInt16_1));let partial_limb_msb_col228 = partial_limb_msb_tmp_57455_92.as_m31();
            *row[228] = partial_limb_msb_col228;

            let cond_felt_252_as_addr_output_tmp_57455_94 = ((((((dst_limb_0_col23) + (((dst_limb_1_col24) * (M31_512))))) + (((dst_limb_2_col25) * (M31_262144))))) + (((dst_limb_3_col26) * (M31_134217728))));

            // Cond Felt 252 As Rel Imm.

            // Decode Small Sign.

            let msb_tmp_57455_95 = res_limb_27_col224.eq(M31_256);let msb_col229 = msb_tmp_57455_95.as_m31();
            *row[229] = msb_col229;let mid_limbs_set_tmp_57455_96 = ((res_limb_20_col217.eq(M31_511)) & (msb_tmp_57455_95));let mid_limbs_set_col230 = mid_limbs_set_tmp_57455_96.as_m31();
            *row[230] = mid_limbs_set_col230;let decode_small_sign_output_tmp_57455_97 = [msb_col229, mid_limbs_set_col230];

            let remainder_bits_tmp_57455_98 = ((res_limb_3_col200) - (((mid_limbs_set_col230) * (M31_508))));

            // Cond Range Check 2.

            let partial_limb_msb_tmp_57455_99 = ((((PackedUInt16::from_m31(remainder_bits_tmp_57455_98)) & (UInt16_2))) >> (UInt16_1));let partial_limb_msb_col231 = partial_limb_msb_tmp_57455_99.as_m31();
            *row[231] = partial_limb_msb_col231;

            let cond_felt_252_as_rel_imm_output_tmp_57455_101 = ((((((((((res_limb_0_col197) + (((res_limb_1_col198) * (M31_512))))) + (((res_limb_2_col199) * (M31_262144))))) + (((remainder_bits_tmp_57455_98) * (M31_134217728))))) - (msb_col229))) - (((M31_536870912) * (mid_limbs_set_col230))));

            let diff_from_p_tmp_57455_102 = ((dst_limb_0_col23) - (M31_1));let diff_from_p_tmp_57455_103 = ((dst_limb_21_col44) - (M31_136));let diff_from_p_tmp_57455_104 = ((dst_limb_27_col50) - (M31_256));let dst_sum_squares_inv_col232 = ((((((((((((((((((((((((((((((((((((((((((((((((((((((((diff_from_p_tmp_57455_102) * (diff_from_p_tmp_57455_102))) + (dst_limb_1_col24))) + (dst_limb_2_col25))) + (dst_limb_3_col26))) + (dst_limb_4_col27))) + (dst_limb_5_col28))) + (dst_limb_6_col29))) + (dst_limb_7_col30))) + (dst_limb_8_col31))) + (dst_limb_9_col32))) + (dst_limb_10_col33))) + (dst_limb_11_col34))) + (dst_limb_12_col35))) + (dst_limb_13_col36))) + (dst_limb_14_col37))) + (dst_limb_15_col38))) + (dst_limb_16_col39))) + (dst_limb_17_col40))) + (dst_limb_18_col41))) + (dst_limb_19_col42))) + (dst_limb_20_col43))) + (((diff_from_p_tmp_57455_103) * (diff_from_p_tmp_57455_103))))) + (dst_limb_22_col45))) + (dst_limb_23_col46))) + (dst_limb_24_col47))) + (dst_limb_25_col48))) + (dst_limb_26_col49))) + (((diff_from_p_tmp_57455_104) * (diff_from_p_tmp_57455_104)))).inverse();
            *row[232] = dst_sum_squares_inv_col232;let dst_sum_tmp_57455_105 = ((((((((((((((((((((((((((((((((((((((((((((((((((((((dst_limb_0_col23) + (dst_limb_1_col24))) + (dst_limb_2_col25))) + (dst_limb_3_col26))) + (dst_limb_4_col27))) + (dst_limb_5_col28))) + (dst_limb_6_col29))) + (dst_limb_7_col30))) + (dst_limb_8_col31))) + (dst_limb_9_col32))) + (dst_limb_10_col33))) + (dst_limb_11_col34))) + (dst_limb_12_col35))) + (dst_limb_13_col36))) + (dst_limb_14_col37))) + (dst_limb_15_col38))) + (dst_limb_16_col39))) + (dst_limb_17_col40))) + (dst_limb_18_col41))) + (dst_limb_19_col42))) + (dst_limb_20_col43))) + (dst_limb_21_col44))) + (dst_limb_22_col45))) + (dst_limb_23_col46))) + (dst_limb_24_col47))) + (dst_limb_25_col48))) + (dst_limb_26_col49))) + (dst_limb_27_col50));let dst_is_zero_tmp_57455_106 = dst_sum_tmp_57455_105.eq(M31_0);let dst_sum_inv_col233 = ((dst_sum_tmp_57455_105) + (dst_is_zero_tmp_57455_106.as_m31())).inverse();
            *row[233] = dst_sum_inv_col233;let op1_as_rel_imm_cond_col234 = ((pc_update_jnz_col15) * (dst_sum_tmp_57455_105));
            *row[234] = op1_as_rel_imm_cond_col234;

            // Cond Felt 252 As Rel Imm.

            // Decode Small Sign.

            let msb_tmp_57455_107 = op1_limb_27_col111.eq(M31_256);let msb_col235 = msb_tmp_57455_107.as_m31();
            *row[235] = msb_col235;let mid_limbs_set_tmp_57455_108 = ((op1_limb_20_col104.eq(M31_511)) & (msb_tmp_57455_107));let mid_limbs_set_col236 = mid_limbs_set_tmp_57455_108.as_m31();
            *row[236] = mid_limbs_set_col236;let decode_small_sign_output_tmp_57455_109 = [msb_col235, mid_limbs_set_col236];

            let remainder_bits_tmp_57455_110 = ((op1_limb_3_col87) - (((mid_limbs_set_col236) * (M31_508))));

            // Cond Range Check 2.

            let partial_limb_msb_tmp_57455_111 = ((((PackedUInt16::from_m31(remainder_bits_tmp_57455_110)) & (UInt16_2))) >> (UInt16_1));let partial_limb_msb_col237 = partial_limb_msb_tmp_57455_111.as_m31();
            *row[237] = partial_limb_msb_col237;

            let cond_felt_252_as_rel_imm_output_tmp_57455_113 = ((((((((((op1_limb_0_col84) + (((op1_limb_1_col85) * (M31_512))))) + (((op1_limb_2_col86) * (M31_262144))))) + (((remainder_bits_tmp_57455_110) * (M31_134217728))))) - (msb_col235))) - (((M31_536870912) * (mid_limbs_set_col236))));

            let next_pc_jnz_col238 = ((((dst_is_zero_tmp_57455_106.as_m31()) * (((input_pc_col0) + (decode_generic_instruction_output_tmp_57455_26.0[19]))))) + (((((M31_1) - (dst_is_zero_tmp_57455_106.as_m31()))) * (((input_pc_col0) + (cond_felt_252_as_rel_imm_output_tmp_57455_113))))));
            *row[238] = next_pc_jnz_col238;let next_pc_col239 = ((((((((decode_generic_instruction_output_tmp_57455_26.0[17]) * (((input_pc_col0) + (decode_generic_instruction_output_tmp_57455_26.0[19]))))) + (((pc_update_jump_col13) * (cond_felt_252_as_addr_output_tmp_57455_91))))) + (((pc_update_jump_rel_col14) * (((input_pc_col0) + (cond_felt_252_as_rel_imm_output_tmp_57455_101))))))) + (((pc_update_jnz_col15) * (next_pc_jnz_col238))));
            *row[239] = next_pc_col239;let next_ap_col240 = ((((((input_ap_col1) + (((ap_update_add_col16) * (cond_felt_252_as_rel_imm_output_tmp_57455_101))))) + (ap_update_add_1_col17))) + (((opcode_call_col18) * (M31_2))));
            *row[240] = next_ap_col240;

            // Range Check Ap.

            let range_check_ap_bot11bits_u32_tmp_57455_114 = ((PackedUInt32::from_m31(next_ap_col240)) & (UInt32_2047));let range_check_ap_bot11bits_col241 = range_check_ap_bot11bits_u32_tmp_57455_114.low().as_m31();
            *row[241] = range_check_ap_bot11bits_col241;*sub_component_inputs.range_check_18[0] =
                [((((next_ap_col240) - (range_check_ap_bot11bits_col241))) * (M31_1048576))];
            *lookup_data.range_check_18_0 = [((((next_ap_col240) - (range_check_ap_bot11bits_col241))) * (M31_1048576))];*sub_component_inputs.range_check_11[0] =
                [range_check_ap_bot11bits_col241];
            *lookup_data.range_check_11_0 = [range_check_ap_bot11bits_col241];

            let next_fp_col242 = ((((((decode_generic_instruction_output_tmp_57455_26.0[18]) * (input_fp_col2))) + (((opcode_ret_col19) * (cond_felt_252_as_addr_output_tmp_57455_94))))) + (((opcode_call_col18) * (((input_ap_col1) + (M31_2))))));
            *row[242] = next_fp_col242;let update_registers_output_tmp_57455_115 = PackedCasmState {
    pc: next_pc_col239, ap: next_ap_col240, fp: next_fp_col242,
};

            *lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];*lookup_data.opcodes_1 = [next_pc_col239, next_ap_col240, next_fp_col242];*row[243] = enabler_col.packed_at(row_index);
        });

    (trace, lookup_data,sub_component_inputs,)
}

#[derive(Uninitialized,IterMut, ParIterMut)]
struct LookupData
{memory_address_to_id_0: Vec<[PackedM31; 2]>,memory_address_to_id_1: Vec<[PackedM31; 2]>,memory_address_to_id_2: Vec<[PackedM31; 2]>,memory_id_to_big_0: Vec<[PackedM31; 29]>,memory_id_to_big_1: Vec<[PackedM31; 29]>,memory_id_to_big_2: Vec<[PackedM31; 29]>,opcodes_0: Vec<[PackedM31; 3]>,opcodes_1: Vec<[PackedM31; 3]>,range_check_11_0: Vec<[PackedM31; 1]>,range_check_18_0: Vec<[PackedM31; 1]>,range_check_20_0: Vec<[PackedM31; 1]>,range_check_20_1: Vec<[PackedM31; 1]>,range_check_20_2: Vec<[PackedM31; 1]>,range_check_20_3: Vec<[PackedM31; 1]>,range_check_20_b_0: Vec<[PackedM31; 1]>,range_check_20_b_1: Vec<[PackedM31; 1]>,range_check_20_b_2: Vec<[PackedM31; 1]>,range_check_20_b_3: Vec<[PackedM31; 1]>,range_check_20_c_0: Vec<[PackedM31; 1]>,range_check_20_c_1: Vec<[PackedM31; 1]>,range_check_20_c_2: Vec<[PackedM31; 1]>,range_check_20_c_3: Vec<[PackedM31; 1]>,range_check_20_d_0: Vec<[PackedM31; 1]>,range_check_20_d_1: Vec<[PackedM31; 1]>,range_check_20_d_2: Vec<[PackedM31; 1]>,range_check_20_d_3: Vec<[PackedM31; 1]>,range_check_20_e_0: Vec<[PackedM31; 1]>,range_check_20_e_1: Vec<[PackedM31; 1]>,range_check_20_e_2: Vec<[PackedM31; 1]>,range_check_20_f_0: Vec<[PackedM31; 1]>,range_check_20_f_1: Vec<[PackedM31; 1]>,range_check_20_f_2: Vec<[PackedM31; 1]>,range_check_20_g_0: Vec<[PackedM31; 1]>,range_check_20_g_1: Vec<[PackedM31; 1]>,range_check_20_g_2: Vec<[PackedM31; 1]>,range_check_20_h_0: Vec<[PackedM31; 1]>,range_check_20_h_1: Vec<[PackedM31; 1]>,range_check_20_h_2: Vec<[PackedM31; 1]>,range_check_9_9_0: Vec<[PackedM31; 2]>,range_check_9_9_1: Vec<[PackedM31; 2]>,range_check_9_9_2: Vec<[PackedM31; 2]>,range_check_9_9_3: Vec<[PackedM31; 2]>,range_check_9_9_b_0: Vec<[PackedM31; 2]>,range_check_9_9_b_1: Vec<[PackedM31; 2]>,range_check_9_9_b_2: Vec<[PackedM31; 2]>,range_check_9_9_b_3: Vec<[PackedM31; 2]>,range_check_9_9_c_0: Vec<[PackedM31; 2]>,range_check_9_9_c_1: Vec<[PackedM31; 2]>,range_check_9_9_c_2: Vec<[PackedM31; 2]>,range_check_9_9_c_3: Vec<[PackedM31; 2]>,range_check_9_9_d_0: Vec<[PackedM31; 2]>,range_check_9_9_d_1: Vec<[PackedM31; 2]>,range_check_9_9_d_2: Vec<[PackedM31; 2]>,range_check_9_9_d_3: Vec<[PackedM31; 2]>,range_check_9_9_e_0: Vec<[PackedM31; 2]>,range_check_9_9_e_1: Vec<[PackedM31; 2]>,range_check_9_9_e_2: Vec<[PackedM31; 2]>,range_check_9_9_e_3: Vec<[PackedM31; 2]>,range_check_9_9_f_0: Vec<[PackedM31; 2]>,range_check_9_9_f_1: Vec<[PackedM31; 2]>,range_check_9_9_f_2: Vec<[PackedM31; 2]>,range_check_9_9_f_3: Vec<[PackedM31; 2]>,range_check_9_9_g_0: Vec<[PackedM31; 2]>,range_check_9_9_g_1: Vec<[PackedM31; 2]>,range_check_9_9_h_0: Vec<[PackedM31; 2]>,range_check_9_9_h_1: Vec<[PackedM31; 2]>,verify_instruction_0: Vec<[PackedM31; 7]>,}

pub struct InteractionClaimGenerator {
    n_rows: usize,log_size: u32,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {

    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        verify_instruction: &relations::VerifyInstruction,
memory_address_to_id: &relations::MemoryAddressToId,
memory_id_to_big: &relations::MemoryIdToBig,
range_check_9_9: &relations::RangeCheck_9_9,
range_check_9_9_b: &relations::RangeCheck_9_9_B,
range_check_9_9_c: &relations::RangeCheck_9_9_C,
range_check_9_9_d: &relations::RangeCheck_9_9_D,
range_check_9_9_e: &relations::RangeCheck_9_9_E,
range_check_9_9_f: &relations::RangeCheck_9_9_F,
range_check_9_9_g: &relations::RangeCheck_9_9_G,
range_check_9_9_h: &relations::RangeCheck_9_9_H,
range_check_20: &relations::RangeCheck_20,
range_check_20_b: &relations::RangeCheck_20_B,
range_check_20_c: &relations::RangeCheck_20_C,
range_check_20_d: &relations::RangeCheck_20_D,
range_check_20_e: &relations::RangeCheck_20_E,
range_check_20_f: &relations::RangeCheck_20_F,
range_check_20_g: &relations::RangeCheck_20_G,
range_check_20_h: &relations::RangeCheck_20_H,
range_check_18: &relations::RangeCheck_18,
range_check_11: &relations::RangeCheck_11,
opcodes: &relations::Opcodes,

    ) -> InteractionClaim
    {
        let enabler_col = Enabler::new(self.n_rows);
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        //Sum logup terms in pairs.
let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.verify_instruction_0,
        &self.lookup_data.memory_address_to_id_0
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = verify_instruction.combine(values0);
        let denom1: PackedQM31 = memory_address_to_id.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.memory_id_to_big_0,
        &self.lookup_data.memory_address_to_id_1
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = memory_id_to_big.combine(values0);
        let denom1: PackedQM31 = memory_address_to_id.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.memory_id_to_big_1,
        &self.lookup_data.memory_address_to_id_2
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = memory_id_to_big.combine(values0);
        let denom1: PackedQM31 = memory_address_to_id.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.memory_id_to_big_2,
        &self.lookup_data.range_check_9_9_0
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = memory_id_to_big.combine(values0);
        let denom1: PackedQM31 = range_check_9_9.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_9_9_b_0,
        &self.lookup_data.range_check_9_9_c_0
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_9_9_b.combine(values0);
        let denom1: PackedQM31 = range_check_9_9_c.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_9_9_d_0,
        &self.lookup_data.range_check_9_9_e_0
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_9_9_d.combine(values0);
        let denom1: PackedQM31 = range_check_9_9_e.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_9_9_f_0,
        &self.lookup_data.range_check_9_9_g_0
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_9_9_f.combine(values0);
        let denom1: PackedQM31 = range_check_9_9_g.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_9_9_h_0,
        &self.lookup_data.range_check_9_9_1
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_9_9_h.combine(values0);
        let denom1: PackedQM31 = range_check_9_9.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_9_9_b_1,
        &self.lookup_data.range_check_9_9_c_1
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_9_9_b.combine(values0);
        let denom1: PackedQM31 = range_check_9_9_c.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_9_9_d_1,
        &self.lookup_data.range_check_9_9_e_1
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_9_9_d.combine(values0);
        let denom1: PackedQM31 = range_check_9_9_e.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_9_9_f_1,
        &self.lookup_data.range_check_9_9_2
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_9_9_f.combine(values0);
        let denom1: PackedQM31 = range_check_9_9.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_9_9_b_2,
        &self.lookup_data.range_check_9_9_c_2
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_9_9_b.combine(values0);
        let denom1: PackedQM31 = range_check_9_9_c.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_9_9_d_2,
        &self.lookup_data.range_check_9_9_e_2
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_9_9_d.combine(values0);
        let denom1: PackedQM31 = range_check_9_9_e.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_9_9_f_2,
        &self.lookup_data.range_check_9_9_g_1
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_9_9_f.combine(values0);
        let denom1: PackedQM31 = range_check_9_9_g.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_9_9_h_1,
        &self.lookup_data.range_check_9_9_3
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_9_9_h.combine(values0);
        let denom1: PackedQM31 = range_check_9_9.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_9_9_b_3,
        &self.lookup_data.range_check_9_9_c_3
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_9_9_b.combine(values0);
        let denom1: PackedQM31 = range_check_9_9_c.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_9_9_d_3,
        &self.lookup_data.range_check_9_9_e_3
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_9_9_d.combine(values0);
        let denom1: PackedQM31 = range_check_9_9_e.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_9_9_f_3,
        &self.lookup_data.range_check_20_0
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_9_9_f.combine(values0);
        let denom1: PackedQM31 = range_check_20.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_20_b_0,
        &self.lookup_data.range_check_20_c_0
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_20_b.combine(values0);
        let denom1: PackedQM31 = range_check_20_c.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_20_d_0,
        &self.lookup_data.range_check_20_e_0
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_20_d.combine(values0);
        let denom1: PackedQM31 = range_check_20_e.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_20_f_0,
        &self.lookup_data.range_check_20_g_0
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_20_f.combine(values0);
        let denom1: PackedQM31 = range_check_20_g.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_20_h_0,
        &self.lookup_data.range_check_20_1
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_20_h.combine(values0);
        let denom1: PackedQM31 = range_check_20.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_20_b_1,
        &self.lookup_data.range_check_20_c_1
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_20_b.combine(values0);
        let denom1: PackedQM31 = range_check_20_c.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_20_d_1,
        &self.lookup_data.range_check_20_e_1
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_20_d.combine(values0);
        let denom1: PackedQM31 = range_check_20_e.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_20_f_1,
        &self.lookup_data.range_check_20_g_1
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_20_f.combine(values0);
        let denom1: PackedQM31 = range_check_20_g.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_20_h_1,
        &self.lookup_data.range_check_20_2
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_20_h.combine(values0);
        let denom1: PackedQM31 = range_check_20.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_20_b_2,
        &self.lookup_data.range_check_20_c_2
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_20_b.combine(values0);
        let denom1: PackedQM31 = range_check_20_c.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_20_d_2,
        &self.lookup_data.range_check_20_e_2
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_20_d.combine(values0);
        let denom1: PackedQM31 = range_check_20_e.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_20_f_2,
        &self.lookup_data.range_check_20_g_2
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_20_f.combine(values0);
        let denom1: PackedQM31 = range_check_20_g.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_20_h_2,
        &self.lookup_data.range_check_20_3
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_20_h.combine(values0);
        let denom1: PackedQM31 = range_check_20.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_20_b_3,
        &self.lookup_data.range_check_20_c_3
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_20_b.combine(values0);
        let denom1: PackedQM31 = range_check_20_c.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_20_d_3,
        &self.lookup_data.range_check_18_0
        )
        .into_par_iter().for_each(|(writer, values0, values1)| {
        let denom0: PackedQM31 = range_check_20_d.combine(values0);
        let denom1: PackedQM31 = range_check_18.combine(values1);
        writer.write_frac(denom0 + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
        &self.lookup_data.range_check_11_0,
        &self.lookup_data.opcodes_0
        )
        .into_par_iter().enumerate().for_each(|(i, (writer, values0, values1))| {
        let denom0: PackedQM31 = range_check_11.combine(values0);
        let denom1: PackedQM31 = opcodes.combine(values1);
        writer.write_frac(denom0 * enabler_col.packed_at(i) + denom1 , denom0 * denom1);
        });
        col_gen.finalize_col();

        //Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(),
            &self.lookup_data
            .opcodes_1
            )
            .into_par_iter().enumerate().for_each(|(i, (writer, values))| {
            let denom =
                opcodes.combine(values);
            writer.write_frac(
                -PackedQM31::one()* enabler_col.packed_at(i),
                denom
            );
        });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim {
            claimed_sum,
        }
    }
}