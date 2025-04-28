// Constraints version: 9330aaaf

use core::num::traits::Zero;
use stwo_constraint_framework::{
    LookupElementsImpl, PreprocessedColumn, PreprocessedColumnSet, PreprocessedColumnSetImpl,
    PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::{
    CirclePoint, CirclePointIndexTrait, CirclePointQM31AddCirclePointM31Trait,
};
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::{M31, m31};
use stwo_verifier_core::fields::qm31::{QM31, QM31Impl, QM31Serde, QM31Zero, qm31_const};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::CairoComponent;
use crate::components::subroutines::cond_felt_252_as_addr::cond_felt_252_as_addr_evaluate;
use crate::utils::U32Impl;


pub fn handle_opcodes_evaluate(
    input: [QM31; 98], ref sum: QM31, domain_vanishing_eval_inv: QM31, random_coeff: QM31,
) -> [QM31; 0] {
    let [
        handle_opcodes_input_limb_0,
        handle_opcodes_input_limb_2,
        handle_opcodes_input_limb_3,
        handle_opcodes_input_limb_4,
        handle_opcodes_input_limb_6,
        handle_opcodes_input_limb_10,
        handle_opcodes_input_limb_15,
        handle_opcodes_input_limb_16,
        handle_opcodes_input_limb_17,
        handle_opcodes_input_limb_19,
        handle_opcodes_input_limb_22,
        handle_opcodes_input_limb_23,
        handle_opcodes_input_limb_24,
        handle_opcodes_input_limb_25,
        handle_opcodes_input_limb_26,
        handle_opcodes_input_limb_27,
        handle_opcodes_input_limb_28,
        handle_opcodes_input_limb_29,
        handle_opcodes_input_limb_30,
        handle_opcodes_input_limb_31,
        handle_opcodes_input_limb_32,
        handle_opcodes_input_limb_33,
        handle_opcodes_input_limb_34,
        handle_opcodes_input_limb_35,
        handle_opcodes_input_limb_36,
        handle_opcodes_input_limb_37,
        handle_opcodes_input_limb_38,
        handle_opcodes_input_limb_39,
        handle_opcodes_input_limb_40,
        handle_opcodes_input_limb_41,
        handle_opcodes_input_limb_42,
        handle_opcodes_input_limb_43,
        handle_opcodes_input_limb_44,
        handle_opcodes_input_limb_45,
        handle_opcodes_input_limb_46,
        handle_opcodes_input_limb_47,
        handle_opcodes_input_limb_48,
        handle_opcodes_input_limb_49,
        handle_opcodes_input_limb_50,
        handle_opcodes_input_limb_51,
        handle_opcodes_input_limb_52,
        handle_opcodes_input_limb_53,
        handle_opcodes_input_limb_54,
        handle_opcodes_input_limb_55,
        handle_opcodes_input_limb_56,
        handle_opcodes_input_limb_57,
        handle_opcodes_input_limb_58,
        handle_opcodes_input_limb_59,
        handle_opcodes_input_limb_60,
        handle_opcodes_input_limb_61,
        handle_opcodes_input_limb_62,
        handle_opcodes_input_limb_63,
        handle_opcodes_input_limb_64,
        handle_opcodes_input_limb_65,
        handle_opcodes_input_limb_66,
        handle_opcodes_input_limb_67,
        handle_opcodes_input_limb_68,
        handle_opcodes_input_limb_69,
        handle_opcodes_input_limb_70,
        handle_opcodes_input_limb_71,
        handle_opcodes_input_limb_72,
        handle_opcodes_input_limb_73,
        handle_opcodes_input_limb_74,
        handle_opcodes_input_limb_75,
        handle_opcodes_input_limb_76,
        handle_opcodes_input_limb_77,
        handle_opcodes_input_limb_78,
        handle_opcodes_input_limb_79,
        handle_opcodes_input_limb_80,
        handle_opcodes_input_limb_81,
        handle_opcodes_input_limb_82,
        handle_opcodes_input_limb_83,
        handle_opcodes_input_limb_84,
        handle_opcodes_input_limb_85,
        handle_opcodes_input_limb_86,
        handle_opcodes_input_limb_87,
        handle_opcodes_input_limb_88,
        handle_opcodes_input_limb_89,
        handle_opcodes_input_limb_90,
        handle_opcodes_input_limb_91,
        handle_opcodes_input_limb_92,
        handle_opcodes_input_limb_93,
        handle_opcodes_input_limb_94,
        handle_opcodes_input_limb_95,
        handle_opcodes_input_limb_96,
        handle_opcodes_input_limb_97,
        handle_opcodes_input_limb_98,
        handle_opcodes_input_limb_99,
        handle_opcodes_input_limb_100,
        handle_opcodes_input_limb_101,
        handle_opcodes_input_limb_102,
        handle_opcodes_input_limb_103,
        handle_opcodes_input_limb_104,
        handle_opcodes_input_limb_105,
        handle_opcodes_input_limb_106,
        handle_opcodes_input_limb_107,
        handle_opcodes_input_limb_108,
        handle_opcodes_input_limb_109,
    ] =
        input;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_82 - handle_opcodes_input_limb_26)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_83 - handle_opcodes_input_limb_27)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_84 - handle_opcodes_input_limb_28)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_85 - handle_opcodes_input_limb_29)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_86 - handle_opcodes_input_limb_30)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_87 - handle_opcodes_input_limb_31)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_88 - handle_opcodes_input_limb_32)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_89 - handle_opcodes_input_limb_33)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_90 - handle_opcodes_input_limb_34)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_91 - handle_opcodes_input_limb_35)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_92 - handle_opcodes_input_limb_36)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_93 - handle_opcodes_input_limb_37)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_94 - handle_opcodes_input_limb_38)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_95 - handle_opcodes_input_limb_39)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_96 - handle_opcodes_input_limb_40)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_97 - handle_opcodes_input_limb_41)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_98 - handle_opcodes_input_limb_42)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_99 - handle_opcodes_input_limb_43)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_100 - handle_opcodes_input_limb_44)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_101 - handle_opcodes_input_limb_45)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_102 - handle_opcodes_input_limb_46)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_103 - handle_opcodes_input_limb_47)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_104 - handle_opcodes_input_limb_48)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_105 - handle_opcodes_input_limb_49)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_106 - handle_opcodes_input_limb_50)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_107 - handle_opcodes_input_limb_51)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_108 - handle_opcodes_input_limb_52)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_17
        * (handle_opcodes_input_limb_109 - handle_opcodes_input_limb_53)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - ret opcode offset0 equals -2
    let constraint_quotient = ((handle_opcodes_input_limb_16
        * (handle_opcodes_input_limb_23 + qm31_const::<2, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - ret opcode offset2 equals -1
    let constraint_quotient = ((handle_opcodes_input_limb_16
        * (handle_opcodes_input_limb_25 + qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - ret opcode flags pc_update_jump and dst_base_fp and op1_base_fp_and_res_op1 are
    // on
    let constraint_quotient = ((handle_opcodes_input_limb_16
        * ((((qm31_const::<4, 0, 0, 0>() - handle_opcodes_input_limb_10)
            - handle_opcodes_input_limb_3)
            - handle_opcodes_input_limb_6)
            - handle_opcodes_input_limb_19)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - call opcode offset0 equals 0
    let constraint_quotient = ((handle_opcodes_input_limb_15 * handle_opcodes_input_limb_23))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - call opcode offset1 equals 1
    let constraint_quotient = ((handle_opcodes_input_limb_15
        * (qm31_const::<1, 0, 0, 0>() - handle_opcodes_input_limb_24)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - call opcode flags op0_base_fp and dst_base_fp are off
    let constraint_quotient = ((handle_opcodes_input_limb_15
        * (handle_opcodes_input_limb_4 + handle_opcodes_input_limb_3)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let output: [QM31; 1] = cond_felt_252_as_addr_evaluate(
        [
            handle_opcodes_input_limb_26, handle_opcodes_input_limb_27,
            handle_opcodes_input_limb_28, handle_opcodes_input_limb_29,
            handle_opcodes_input_limb_30, handle_opcodes_input_limb_31,
            handle_opcodes_input_limb_32, handle_opcodes_input_limb_33,
            handle_opcodes_input_limb_34, handle_opcodes_input_limb_35,
            handle_opcodes_input_limb_36, handle_opcodes_input_limb_37,
            handle_opcodes_input_limb_38, handle_opcodes_input_limb_39,
            handle_opcodes_input_limb_40, handle_opcodes_input_limb_41,
            handle_opcodes_input_limb_42, handle_opcodes_input_limb_43,
            handle_opcodes_input_limb_44, handle_opcodes_input_limb_45,
            handle_opcodes_input_limb_46, handle_opcodes_input_limb_47,
            handle_opcodes_input_limb_48, handle_opcodes_input_limb_49,
            handle_opcodes_input_limb_50, handle_opcodes_input_limb_51,
            handle_opcodes_input_limb_52, handle_opcodes_input_limb_53,
            handle_opcodes_input_limb_15,
        ],
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [cond_felt_252_as_addr_output_tmp_aa5c5_0] = output;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_15
        * (cond_felt_252_as_addr_output_tmp_aa5c5_0 - handle_opcodes_input_limb_2)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let output: [QM31; 1] = cond_felt_252_as_addr_evaluate(
        [
            handle_opcodes_input_limb_54, handle_opcodes_input_limb_55,
            handle_opcodes_input_limb_56, handle_opcodes_input_limb_57,
            handle_opcodes_input_limb_58, handle_opcodes_input_limb_59,
            handle_opcodes_input_limb_60, handle_opcodes_input_limb_61,
            handle_opcodes_input_limb_62, handle_opcodes_input_limb_63,
            handle_opcodes_input_limb_64, handle_opcodes_input_limb_65,
            handle_opcodes_input_limb_66, handle_opcodes_input_limb_67,
            handle_opcodes_input_limb_68, handle_opcodes_input_limb_69,
            handle_opcodes_input_limb_70, handle_opcodes_input_limb_71,
            handle_opcodes_input_limb_72, handle_opcodes_input_limb_73,
            handle_opcodes_input_limb_74, handle_opcodes_input_limb_75,
            handle_opcodes_input_limb_76, handle_opcodes_input_limb_77,
            handle_opcodes_input_limb_78, handle_opcodes_input_limb_79,
            handle_opcodes_input_limb_80, handle_opcodes_input_limb_81,
            handle_opcodes_input_limb_15,
        ],
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [cond_felt_252_as_addr_output_tmp_aa5c5_1] = output;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_limb_15
        * (cond_felt_252_as_addr_output_tmp_aa5c5_1
            - (handle_opcodes_input_limb_0 + handle_opcodes_input_limb_22))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    []
}
