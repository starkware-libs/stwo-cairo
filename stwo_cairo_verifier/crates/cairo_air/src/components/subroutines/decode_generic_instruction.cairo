use core::num::traits::Zero;
use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedColumnSetImpl, PreprocessedMaskValues,
    PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::{
    CirclePoint, CirclePointIndexTrait, CirclePointQM31AddCirclePointM31Trait,
};
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::{M31, m31};
use stwo_verifier_core::fields::qm31::{
    QM31, QM31Impl, QM31Serde, QM31Zero, QM31_EXTENSION_DEGREE, qm31_const,
};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::subroutines::decode_instruction_df7a6::decode_instruction_df7a6_evaluate;
use crate::components::verify_instruction::{
    VERIFY_INSTRUCTION_RELATION_SIZE, verify_instruction_sum,
};
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE, opcodes_sum};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 18;


pub fn decode_generic_instruction_evaluate(
    input: QM31,
    offset0_col0: QM31,
    offset1_col1: QM31,
    offset2_col2: QM31,
    dst_base_fp_col3: QM31,
    op0_base_fp_col4: QM31,
    op1_imm_col5: QM31,
    op1_base_fp_col6: QM31,
    op1_base_ap_col7: QM31,
    res_add_col8: QM31,
    res_mul_col9: QM31,
    pc_update_jump_col10: QM31,
    pc_update_jump_rel_col11: QM31,
    pc_update_jnz_col12: QM31,
    ap_update_add_col13: QM31,
    ap_update_add_1_col14: QM31,
    opcode_call_col15: QM31,
    opcode_ret_col16: QM31,
    opcode_assert_eq_col17: QM31,
    verify_instruction_alphas: Span<QM31>,
    verify_instruction_z: QM31,
    ref verify_instruction_sum_0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 23] {
    let decode_generic_instruction_input = input;

    let output: [QM31; 19] = decode_instruction_df7a6_evaluate(
        decode_generic_instruction_input,
        offset0_col0,
        offset1_col1,
        offset2_col2,
        dst_base_fp_col3,
        op0_base_fp_col4,
        op1_imm_col5,
        op1_base_fp_col6,
        op1_base_ap_col7,
        res_add_col8,
        res_mul_col9,
        pc_update_jump_col10,
        pc_update_jump_rel_col11,
        pc_update_jnz_col12,
        ap_update_add_col13,
        ap_update_add_1_col14,
        opcode_call_col15,
        opcode_ret_col16,
        opcode_assert_eq_col17,
        verify_instruction_alphas,
        verify_instruction_z,
        ref verify_instruction_sum_0,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [
        decode_instruction_df7a6_output_tmp_62f3c_20_offset0,
        decode_instruction_df7a6_output_tmp_62f3c_20_offset1,
        decode_instruction_df7a6_output_tmp_62f3c_20_offset2,
        decode_instruction_df7a6_output_tmp_62f3c_20_dst_base_fp,
        decode_instruction_df7a6_output_tmp_62f3c_20_op0_base_fp,
        decode_instruction_df7a6_output_tmp_62f3c_20_op1_imm,
        decode_instruction_df7a6_output_tmp_62f3c_20_op1_base_fp,
        decode_instruction_df7a6_output_tmp_62f3c_20_op1_base_ap,
        decode_instruction_df7a6_output_tmp_62f3c_20_res_add,
        decode_instruction_df7a6_output_tmp_62f3c_20_res_mul,
        decode_instruction_df7a6_output_tmp_62f3c_20_pc_update_jump,
        decode_instruction_df7a6_output_tmp_62f3c_20_pc_update_jump_rel,
        decode_instruction_df7a6_output_tmp_62f3c_20_pc_update_jnz,
        decode_instruction_df7a6_output_tmp_62f3c_20_ap_update_add,
        decode_instruction_df7a6_output_tmp_62f3c_20_ap_update_add_1,
        decode_instruction_df7a6_output_tmp_62f3c_20_opcode_call,
        decode_instruction_df7a6_output_tmp_62f3c_20_opcode_ret,
        decode_instruction_df7a6_output_tmp_62f3c_20_opcode_assert_eq,
        decode_instruction_df7a6_output_tmp_62f3c_20_opcode_extension,
    ] =
        output;
    let op1_base_op0_tmp_62f3c_21: QM31 = (((qm31_const::<1, 0, 0, 0>() - op1_imm_col5)
        - op1_base_fp_col6)
        - op1_base_ap_col7);

    // Constraint - op1_src is 0, 1, 2, or 4
    let constraint_quotient = ((op1_base_op0_tmp_62f3c_21
        * (qm31_const::<1, 0, 0, 0>() - op1_base_op0_tmp_62f3c_21)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let res_op1_tmp_62f3c_22: QM31 = (((qm31_const::<1, 0, 0, 0>() - res_add_col8) - res_mul_col9)
        - pc_update_jnz_col12);

    // Constraint - res_logic is 0, 1, or 2
    let constraint_quotient = ((res_op1_tmp_62f3c_22
        * (qm31_const::<1, 0, 0, 0>() - res_op1_tmp_62f3c_22)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let pc_update_regular_tmp_62f3c_23: QM31 = (((qm31_const::<1, 0, 0, 0>() - pc_update_jump_col10)
        - pc_update_jump_rel_col11)
        - pc_update_jnz_col12);

    // Constraint - pc_update is 0, 1, 2, or 4
    let constraint_quotient = ((pc_update_regular_tmp_62f3c_23
        * (qm31_const::<1, 0, 0, 0>() - pc_update_regular_tmp_62f3c_23)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let ap_update_regular_tmp_62f3c_24: QM31 = (((qm31_const::<1, 0, 0, 0>() - ap_update_add_col13)
        - ap_update_add_1_col14)
        - opcode_call_col15);

    // Constraint - ap_update is 0, 1, 2, or 4
    let constraint_quotient = ((ap_update_regular_tmp_62f3c_24
        * (qm31_const::<1, 0, 0, 0>() - ap_update_regular_tmp_62f3c_24)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let fp_update_regular_tmp_62f3c_25: QM31 = ((qm31_const::<1, 0, 0, 0>() - opcode_call_col15)
        - opcode_ret_col16);

    // Constraint - opcode is 0, 1, 2, or 4
    let constraint_quotient = ((fp_update_regular_tmp_62f3c_25
        * (qm31_const::<1, 0, 0, 0>() - fp_update_regular_tmp_62f3c_25)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    [
        dst_base_fp_col3, op0_base_fp_col4, op1_imm_col5, op1_base_fp_col6, op1_base_ap_col7,
        res_add_col8, res_mul_col9, pc_update_jump_col10, pc_update_jump_rel_col11,
        pc_update_jnz_col12, ap_update_add_col13, ap_update_add_1_col14, opcode_call_col15,
        opcode_ret_col16, opcode_assert_eq_col17, op1_base_op0_tmp_62f3c_21, res_op1_tmp_62f3c_22,
        pc_update_regular_tmp_62f3c_23, fp_update_regular_tmp_62f3c_25,
        (qm31_const::<1, 0, 0, 0>() + op1_imm_col5),
        decode_instruction_df7a6_output_tmp_62f3c_20_offset0,
        decode_instruction_df7a6_output_tmp_62f3c_20_offset1,
        decode_instruction_df7a6_output_tmp_62f3c_20_offset2,
    ]
}
