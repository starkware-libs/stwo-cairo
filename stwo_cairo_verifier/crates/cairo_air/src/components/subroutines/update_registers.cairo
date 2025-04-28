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
use crate::components::subroutines::cond_felt_252_as_addr::cond_felt_252_as_addr_evaluate;
use crate::components::subroutines::cond_felt_252_as_rel_imm::cond_felt_252_as_rel_imm_evaluate;
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE, opcodes_sum};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 11;


pub fn update_registers_evaluate(
    input: [QM31; 107],
    msb_col0: QM31,
    mid_limbs_set_col1: QM31,
    dst_sum_squares_inv_col2: QM31,
    dst_sum_inv_col3: QM31,
    op1_as_rel_imm_cond_col4: QM31,
    msb_col5: QM31,
    mid_limbs_set_col6: QM31,
    next_pc_jnz_col7: QM31,
    next_pc_col8: QM31,
    next_ap_col9: QM31,
    next_fp_col10: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 3] {
    let [
        update_registers_input_limb_0,
        update_registers_input_limb_1,
        update_registers_input_limb_2,
        update_registers_input_limb_3,
        update_registers_input_limb_4,
        update_registers_input_limb_5,
        update_registers_input_limb_6,
        update_registers_input_limb_7,
        update_registers_input_limb_8,
        update_registers_input_limb_9,
        update_registers_input_limb_10,
        update_registers_input_limb_11,
        update_registers_input_limb_12,
        update_registers_input_limb_13,
        update_registers_input_limb_14,
        update_registers_input_limb_15,
        update_registers_input_limb_16,
        update_registers_input_limb_17,
        update_registers_input_limb_18,
        update_registers_input_limb_19,
        update_registers_input_limb_20,
        update_registers_input_limb_21,
        update_registers_input_limb_22,
        update_registers_input_limb_23,
        update_registers_input_limb_24,
        update_registers_input_limb_25,
        update_registers_input_limb_26,
        update_registers_input_limb_27,
        update_registers_input_limb_28,
        update_registers_input_limb_29,
        update_registers_input_limb_30,
        update_registers_input_limb_31,
        update_registers_input_limb_32,
        update_registers_input_limb_33,
        update_registers_input_limb_34,
        update_registers_input_limb_35,
        update_registers_input_limb_36,
        update_registers_input_limb_37,
        update_registers_input_limb_38,
        update_registers_input_limb_39,
        update_registers_input_limb_40,
        update_registers_input_limb_41,
        update_registers_input_limb_42,
        update_registers_input_limb_43,
        update_registers_input_limb_44,
        update_registers_input_limb_45,
        update_registers_input_limb_46,
        update_registers_input_limb_47,
        update_registers_input_limb_48,
        update_registers_input_limb_49,
        update_registers_input_limb_50,
        update_registers_input_limb_51,
        update_registers_input_limb_52,
        update_registers_input_limb_53,
        update_registers_input_limb_54,
        update_registers_input_limb_55,
        update_registers_input_limb_56,
        update_registers_input_limb_57,
        update_registers_input_limb_58,
        update_registers_input_limb_59,
        update_registers_input_limb_60,
        update_registers_input_limb_61,
        update_registers_input_limb_62,
        update_registers_input_limb_63,
        update_registers_input_limb_64,
        update_registers_input_limb_65,
        update_registers_input_limb_66,
        update_registers_input_limb_67,
        update_registers_input_limb_68,
        update_registers_input_limb_69,
        update_registers_input_limb_70,
        update_registers_input_limb_71,
        update_registers_input_limb_72,
        update_registers_input_limb_73,
        update_registers_input_limb_74,
        update_registers_input_limb_75,
        update_registers_input_limb_76,
        update_registers_input_limb_77,
        update_registers_input_limb_78,
        update_registers_input_limb_79,
        update_registers_input_limb_80,
        update_registers_input_limb_81,
        update_registers_input_limb_82,
        update_registers_input_limb_83,
        update_registers_input_limb_84,
        update_registers_input_limb_85,
        update_registers_input_limb_86,
        update_registers_input_limb_87,
        update_registers_input_limb_88,
        update_registers_input_limb_89,
        update_registers_input_limb_90,
        update_registers_input_limb_91,
        update_registers_input_limb_92,
        update_registers_input_limb_93,
        update_registers_input_limb_94,
        update_registers_input_limb_95,
        update_registers_input_limb_96,
        update_registers_input_limb_97,
        update_registers_input_limb_98,
        update_registers_input_limb_99,
        update_registers_input_limb_100,
        update_registers_input_limb_101,
        update_registers_input_limb_102,
        update_registers_input_limb_103,
        update_registers_input_limb_104,
        update_registers_input_limb_105,
        update_registers_input_limb_106,
    ] =
        input;

    let cond_felt_252_as_addr_output_tmp_783d5_0: QM31 = cond_felt_252_as_addr_evaluate(
        [
            update_registers_input_limb_79, update_registers_input_limb_80,
            update_registers_input_limb_81, update_registers_input_limb_82,
            update_registers_input_limb_83, update_registers_input_limb_84,
            update_registers_input_limb_85, update_registers_input_limb_86,
            update_registers_input_limb_87, update_registers_input_limb_88,
            update_registers_input_limb_89, update_registers_input_limb_90,
            update_registers_input_limb_91, update_registers_input_limb_92,
            update_registers_input_limb_93, update_registers_input_limb_94,
            update_registers_input_limb_95, update_registers_input_limb_96,
            update_registers_input_limb_97, update_registers_input_limb_98,
            update_registers_input_limb_99, update_registers_input_limb_100,
            update_registers_input_limb_101, update_registers_input_limb_102,
            update_registers_input_limb_103, update_registers_input_limb_104,
            update_registers_input_limb_105, update_registers_input_limb_106,
            update_registers_input_limb_10,
        ],
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let cond_felt_252_as_addr_output_tmp_783d5_1: QM31 = cond_felt_252_as_addr_evaluate(
        [
            update_registers_input_limb_23, update_registers_input_limb_24,
            update_registers_input_limb_25, update_registers_input_limb_26,
            update_registers_input_limb_27, update_registers_input_limb_28,
            update_registers_input_limb_29, update_registers_input_limb_30,
            update_registers_input_limb_31, update_registers_input_limb_32,
            update_registers_input_limb_33, update_registers_input_limb_34,
            update_registers_input_limb_35, update_registers_input_limb_36,
            update_registers_input_limb_37, update_registers_input_limb_38,
            update_registers_input_limb_39, update_registers_input_limb_40,
            update_registers_input_limb_41, update_registers_input_limb_42,
            update_registers_input_limb_43, update_registers_input_limb_44,
            update_registers_input_limb_45, update_registers_input_limb_46,
            update_registers_input_limb_47, update_registers_input_limb_48,
            update_registers_input_limb_49, update_registers_input_limb_50,
            update_registers_input_limb_16,
        ],
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let cond_felt_252_as_rel_imm_output_tmp_783d5_5: QM31 = cond_felt_252_as_rel_imm_evaluate(
        [
            update_registers_input_limb_79, update_registers_input_limb_80,
            update_registers_input_limb_81, update_registers_input_limb_82,
            update_registers_input_limb_83, update_registers_input_limb_84,
            update_registers_input_limb_85, update_registers_input_limb_86,
            update_registers_input_limb_87, update_registers_input_limb_88,
            update_registers_input_limb_89, update_registers_input_limb_90,
            update_registers_input_limb_91, update_registers_input_limb_92,
            update_registers_input_limb_93, update_registers_input_limb_94,
            update_registers_input_limb_95, update_registers_input_limb_96,
            update_registers_input_limb_97, update_registers_input_limb_98,
            update_registers_input_limb_99, update_registers_input_limb_100,
            update_registers_input_limb_101, update_registers_input_limb_102,
            update_registers_input_limb_103, update_registers_input_limb_104,
            update_registers_input_limb_105, update_registers_input_limb_106,
            (update_registers_input_limb_11 + update_registers_input_limb_13),
        ],
        msb_col0,
        mid_limbs_set_col1,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let diff_from_p_tmp_783d5_6: QM31 = (update_registers_input_limb_23
        - qm31_const::<1, 0, 0, 0>());
    let diff_from_p_tmp_783d5_7: QM31 = (update_registers_input_limb_44
        - qm31_const::<136, 0, 0, 0>());
    let diff_from_p_tmp_783d5_8: QM31 = (update_registers_input_limb_50
        - qm31_const::<256, 0, 0, 0>());

    // Constraint - dst_not_p
    let constraint_quotient = (((((((((((((((((((((((((((((((diff_from_p_tmp_783d5_6
        * diff_from_p_tmp_783d5_6)
        + update_registers_input_limb_24)
        + update_registers_input_limb_25)
        + update_registers_input_limb_26)
        + update_registers_input_limb_27)
        + update_registers_input_limb_28)
        + update_registers_input_limb_29)
        + update_registers_input_limb_30)
        + update_registers_input_limb_31)
        + update_registers_input_limb_32)
        + update_registers_input_limb_33)
        + update_registers_input_limb_34)
        + update_registers_input_limb_35)
        + update_registers_input_limb_36)
        + update_registers_input_limb_37)
        + update_registers_input_limb_38)
        + update_registers_input_limb_39)
        + update_registers_input_limb_40)
        + update_registers_input_limb_41)
        + update_registers_input_limb_42)
        + update_registers_input_limb_43)
        + (diff_from_p_tmp_783d5_7 * diff_from_p_tmp_783d5_7))
        + update_registers_input_limb_45)
        + update_registers_input_limb_46)
        + update_registers_input_limb_47)
        + update_registers_input_limb_48)
        + update_registers_input_limb_49)
        + (diff_from_p_tmp_783d5_8 * diff_from_p_tmp_783d5_8))
        * dst_sum_squares_inv_col2)
        - qm31_const::<1, 0, 0, 0>()))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let dst_sum_tmp_783d5_9: QM31 = (((((((((((((((((((((((((((update_registers_input_limb_23
        + update_registers_input_limb_24)
        + update_registers_input_limb_25)
        + update_registers_input_limb_26)
        + update_registers_input_limb_27)
        + update_registers_input_limb_28)
        + update_registers_input_limb_29)
        + update_registers_input_limb_30)
        + update_registers_input_limb_31)
        + update_registers_input_limb_32)
        + update_registers_input_limb_33)
        + update_registers_input_limb_34)
        + update_registers_input_limb_35)
        + update_registers_input_limb_36)
        + update_registers_input_limb_37)
        + update_registers_input_limb_38)
        + update_registers_input_limb_39)
        + update_registers_input_limb_40)
        + update_registers_input_limb_41)
        + update_registers_input_limb_42)
        + update_registers_input_limb_43)
        + update_registers_input_limb_44)
        + update_registers_input_limb_45)
        + update_registers_input_limb_46)
        + update_registers_input_limb_47)
        + update_registers_input_limb_48)
        + update_registers_input_limb_49)
        + update_registers_input_limb_50);

    // Constraint - op1_as_rel_imm_cond
    let constraint_quotient = ((op1_as_rel_imm_cond_col4
        - (update_registers_input_limb_12 * dst_sum_tmp_783d5_9)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let cond_felt_252_as_rel_imm_output_tmp_783d5_14: QM31 = cond_felt_252_as_rel_imm_evaluate(
        [
            update_registers_input_limb_51, update_registers_input_limb_52,
            update_registers_input_limb_53, update_registers_input_limb_54,
            update_registers_input_limb_55, update_registers_input_limb_56,
            update_registers_input_limb_57, update_registers_input_limb_58,
            update_registers_input_limb_59, update_registers_input_limb_60,
            update_registers_input_limb_61, update_registers_input_limb_62,
            update_registers_input_limb_63, update_registers_input_limb_64,
            update_registers_input_limb_65, update_registers_input_limb_66,
            update_registers_input_limb_67, update_registers_input_limb_68,
            update_registers_input_limb_69, update_registers_input_limb_70,
            update_registers_input_limb_71, update_registers_input_limb_72,
            update_registers_input_limb_73, update_registers_input_limb_74,
            update_registers_input_limb_75, update_registers_input_limb_76,
            update_registers_input_limb_77, update_registers_input_limb_78,
            op1_as_rel_imm_cond_col4,
        ],
        msb_col5,
        mid_limbs_set_col6,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    // Constraint - Constraint1 for conditional jump
    let constraint_quotient = (((next_pc_jnz_col7
        - (update_registers_input_limb_0 + cond_felt_252_as_rel_imm_output_tmp_783d5_14))
        * dst_sum_tmp_783d5_9))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Constraint2 for conditional jump
    let constraint_quotient = (((next_pc_jnz_col7
        - (update_registers_input_limb_0 + update_registers_input_limb_22))
        * ((dst_sum_tmp_783d5_9 * dst_sum_inv_col3) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - next_pc
    let constraint_quotient = ((next_pc_col8
        - ((((update_registers_input_limb_20
            * (update_registers_input_limb_0 + update_registers_input_limb_22))
            + (update_registers_input_limb_10 * cond_felt_252_as_addr_output_tmp_783d5_0))
            + (update_registers_input_limb_11
                * (update_registers_input_limb_0 + cond_felt_252_as_rel_imm_output_tmp_783d5_5)))
            + (update_registers_input_limb_12 * next_pc_jnz_col7))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - next_ap
    let constraint_quotient = ((next_ap_col9
        - (((update_registers_input_limb_1
            + (update_registers_input_limb_13 * cond_felt_252_as_rel_imm_output_tmp_783d5_5))
            + update_registers_input_limb_14)
            + (update_registers_input_limb_15 * qm31_const::<2, 0, 0, 0>()))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - next_fp
    let constraint_quotient = ((next_fp_col10
        - (((update_registers_input_limb_21 * update_registers_input_limb_2)
            + (update_registers_input_limb_16 * cond_felt_252_as_addr_output_tmp_783d5_1))
            + (update_registers_input_limb_15
                * (update_registers_input_limb_1 + qm31_const::<2, 0, 0, 0>())))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    [next_pc_col8, next_ap_col9, next_fp_col10]
}
