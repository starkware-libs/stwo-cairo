// Constraints version: cab09e9c

use core::num::traits::Zero;
use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
    PreprocessedColumnSetImpl, LookupElementsImpl,
};
use stwo_verifier_core::circle::CirclePointQM31AddCirclePointM31Trait;
use stwo_verifier_core::circle::CirclePointIndexTrait;
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::{m31, M31};
use stwo_verifier_core::fields::qm31::{qm31_const, QM31, QM31Impl, QM31Serde, QM31Zero};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE};
use crate::utils::U32Impl;
use crate::components::range_check_9_9::RANGE_CHECK_9_9_RELATION_SIZE;
use crate::components::range_check_19::RANGE_CHECK_19_RELATION_SIZE;
use crate::components::subroutines::range_check_mem_value_n_28::range_check_mem_value_n_28_evaluate;
use crate::components::subroutines::verify_mul_252::verify_mul_252_evaluate;


pub const N_TRACE_COLUMNS: usize = 56;



pub fn div_252_evaluate(
    input: [QM31; 56],
    div_res_limb_0_col0: QM31,
div_res_limb_1_col1: QM31,
div_res_limb_2_col2: QM31,
div_res_limb_3_col3: QM31,
div_res_limb_4_col4: QM31,
div_res_limb_5_col5: QM31,
div_res_limb_6_col6: QM31,
div_res_limb_7_col7: QM31,
div_res_limb_8_col8: QM31,
div_res_limb_9_col9: QM31,
div_res_limb_10_col10: QM31,
div_res_limb_11_col11: QM31,
div_res_limb_12_col12: QM31,
div_res_limb_13_col13: QM31,
div_res_limb_14_col14: QM31,
div_res_limb_15_col15: QM31,
div_res_limb_16_col16: QM31,
div_res_limb_17_col17: QM31,
div_res_limb_18_col18: QM31,
div_res_limb_19_col19: QM31,
div_res_limb_20_col20: QM31,
div_res_limb_21_col21: QM31,
div_res_limb_22_col22: QM31,
div_res_limb_23_col23: QM31,
div_res_limb_24_col24: QM31,
div_res_limb_25_col25: QM31,
div_res_limb_26_col26: QM31,
div_res_limb_27_col27: QM31,
k_col28: QM31,
carry_0_col29: QM31,
carry_1_col30: QM31,
carry_2_col31: QM31,
carry_3_col32: QM31,
carry_4_col33: QM31,
carry_5_col34: QM31,
carry_6_col35: QM31,
carry_7_col36: QM31,
carry_8_col37: QM31,
carry_9_col38: QM31,
carry_10_col39: QM31,
carry_11_col40: QM31,
carry_12_col41: QM31,
carry_13_col42: QM31,
carry_14_col43: QM31,
carry_15_col44: QM31,
carry_16_col45: QM31,
carry_17_col46: QM31,
carry_18_col47: QM31,
carry_19_col48: QM31,
carry_20_col49: QM31,
carry_21_col50: QM31,
carry_22_col51: QM31,
carry_23_col52: QM31,
carry_24_col53: QM31,
carry_25_col54: QM31,
carry_26_col55: QM31,
range_check_9_9_lookup_elements: @crate::RangeCheck_9_9Elements,
range_check_19_lookup_elements: @crate::RangeCheck_19Elements,
ref range_check_9_9_sum_0: QM31,
ref range_check_9_9_sum_1: QM31,
ref range_check_9_9_sum_2: QM31,
ref range_check_9_9_sum_3: QM31,
ref range_check_9_9_sum_4: QM31,
ref range_check_9_9_sum_5: QM31,
ref range_check_9_9_sum_6: QM31,
ref range_check_9_9_sum_7: QM31,
ref range_check_9_9_sum_8: QM31,
ref range_check_9_9_sum_9: QM31,
ref range_check_9_9_sum_10: QM31,
ref range_check_9_9_sum_11: QM31,
ref range_check_9_9_sum_12: QM31,
ref range_check_9_9_sum_13: QM31,
ref range_check_19_sum_14: QM31,
ref range_check_19_sum_15: QM31,
ref range_check_19_sum_16: QM31,
ref range_check_19_sum_17: QM31,
ref range_check_19_sum_18: QM31,
ref range_check_19_sum_19: QM31,
ref range_check_19_sum_20: QM31,
ref range_check_19_sum_21: QM31,
ref range_check_19_sum_22: QM31,
ref range_check_19_sum_23: QM31,
ref range_check_19_sum_24: QM31,
ref range_check_19_sum_25: QM31,
ref range_check_19_sum_26: QM31,
ref range_check_19_sum_27: QM31,
ref range_check_19_sum_28: QM31,
ref range_check_19_sum_29: QM31,
ref range_check_19_sum_30: QM31,
ref range_check_19_sum_31: QM31,
ref range_check_19_sum_32: QM31,
ref range_check_19_sum_33: QM31,
ref range_check_19_sum_34: QM31,
ref range_check_19_sum_35: QM31,
ref range_check_19_sum_36: QM31,
ref range_check_19_sum_37: QM31,
ref range_check_19_sum_38: QM31,
ref range_check_19_sum_39: QM31,
ref range_check_19_sum_40: QM31,
ref range_check_19_sum_41: QM31,

    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [div_252_input_c_limb_0, div_252_input_c_limb_1, div_252_input_c_limb_2, div_252_input_c_limb_3, div_252_input_c_limb_4, div_252_input_c_limb_5, div_252_input_c_limb_6, div_252_input_c_limb_7, div_252_input_c_limb_8, div_252_input_c_limb_9, div_252_input_c_limb_10, div_252_input_c_limb_11, div_252_input_c_limb_12, div_252_input_c_limb_13, div_252_input_c_limb_14, div_252_input_c_limb_15, div_252_input_c_limb_16, div_252_input_c_limb_17, div_252_input_c_limb_18, div_252_input_c_limb_19, div_252_input_c_limb_20, div_252_input_c_limb_21, div_252_input_c_limb_22, div_252_input_c_limb_23, div_252_input_c_limb_24, div_252_input_c_limb_25, div_252_input_c_limb_26, div_252_input_c_limb_27, div_252_input_a_limb_0, div_252_input_a_limb_1, div_252_input_a_limb_2, div_252_input_a_limb_3, div_252_input_a_limb_4, div_252_input_a_limb_5, div_252_input_a_limb_6, div_252_input_a_limb_7, div_252_input_a_limb_8, div_252_input_a_limb_9, div_252_input_a_limb_10, div_252_input_a_limb_11, div_252_input_a_limb_12, div_252_input_a_limb_13, div_252_input_a_limb_14, div_252_input_a_limb_15, div_252_input_a_limb_16, div_252_input_a_limb_17, div_252_input_a_limb_18, div_252_input_a_limb_19, div_252_input_a_limb_20, div_252_input_a_limb_21, div_252_input_a_limb_22, div_252_input_a_limb_23, div_252_input_a_limb_24, div_252_input_a_limb_25, div_252_input_a_limb_26, div_252_input_a_limb_27] = input;

    

    range_check_mem_value_n_28_evaluate(
            [div_res_limb_0_col0, div_res_limb_1_col1, div_res_limb_2_col2, div_res_limb_3_col3, div_res_limb_4_col4, div_res_limb_5_col5, div_res_limb_6_col6, div_res_limb_7_col7, div_res_limb_8_col8, div_res_limb_9_col9, div_res_limb_10_col10, div_res_limb_11_col11, div_res_limb_12_col12, div_res_limb_13_col13, div_res_limb_14_col14, div_res_limb_15_col15, div_res_limb_16_col16, div_res_limb_17_col17, div_res_limb_18_col18, div_res_limb_19_col19, div_res_limb_20_col20, div_res_limb_21_col21, div_res_limb_22_col22, div_res_limb_23_col23, div_res_limb_24_col24, div_res_limb_25_col25, div_res_limb_26_col26, div_res_limb_27_col27],
range_check_9_9_lookup_elements,
ref range_check_9_9_sum_0,
ref range_check_9_9_sum_1,
ref range_check_9_9_sum_2,
ref range_check_9_9_sum_3,
ref range_check_9_9_sum_4,
ref range_check_9_9_sum_5,
ref range_check_9_9_sum_6,
ref range_check_9_9_sum_7,
ref range_check_9_9_sum_8,
ref range_check_9_9_sum_9,
ref range_check_9_9_sum_10,
ref range_check_9_9_sum_11,
ref range_check_9_9_sum_12,
ref range_check_9_9_sum_13,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );


    verify_mul_252_evaluate(
            [div_252_input_a_limb_0, div_252_input_a_limb_1, div_252_input_a_limb_2, div_252_input_a_limb_3, div_252_input_a_limb_4, div_252_input_a_limb_5, div_252_input_a_limb_6, div_252_input_a_limb_7, div_252_input_a_limb_8, div_252_input_a_limb_9, div_252_input_a_limb_10, div_252_input_a_limb_11, div_252_input_a_limb_12, div_252_input_a_limb_13, div_252_input_a_limb_14, div_252_input_a_limb_15, div_252_input_a_limb_16, div_252_input_a_limb_17, div_252_input_a_limb_18, div_252_input_a_limb_19, div_252_input_a_limb_20, div_252_input_a_limb_21, div_252_input_a_limb_22, div_252_input_a_limb_23, div_252_input_a_limb_24, div_252_input_a_limb_25, div_252_input_a_limb_26, div_252_input_a_limb_27, div_res_limb_0_col0, div_res_limb_1_col1, div_res_limb_2_col2, div_res_limb_3_col3, div_res_limb_4_col4, div_res_limb_5_col5, div_res_limb_6_col6, div_res_limb_7_col7, div_res_limb_8_col8, div_res_limb_9_col9, div_res_limb_10_col10, div_res_limb_11_col11, div_res_limb_12_col12, div_res_limb_13_col13, div_res_limb_14_col14, div_res_limb_15_col15, div_res_limb_16_col16, div_res_limb_17_col17, div_res_limb_18_col18, div_res_limb_19_col19, div_res_limb_20_col20, div_res_limb_21_col21, div_res_limb_22_col22, div_res_limb_23_col23, div_res_limb_24_col24, div_res_limb_25_col25, div_res_limb_26_col26, div_res_limb_27_col27, div_252_input_c_limb_0, div_252_input_c_limb_1, div_252_input_c_limb_2, div_252_input_c_limb_3, div_252_input_c_limb_4, div_252_input_c_limb_5, div_252_input_c_limb_6, div_252_input_c_limb_7, div_252_input_c_limb_8, div_252_input_c_limb_9, div_252_input_c_limb_10, div_252_input_c_limb_11, div_252_input_c_limb_12, div_252_input_c_limb_13, div_252_input_c_limb_14, div_252_input_c_limb_15, div_252_input_c_limb_16, div_252_input_c_limb_17, div_252_input_c_limb_18, div_252_input_c_limb_19, div_252_input_c_limb_20, div_252_input_c_limb_21, div_252_input_c_limb_22, div_252_input_c_limb_23, div_252_input_c_limb_24, div_252_input_c_limb_25, div_252_input_c_limb_26, div_252_input_c_limb_27],
k_col28,
carry_0_col29,
carry_1_col30,
carry_2_col31,
carry_3_col32,
carry_4_col33,
carry_5_col34,
carry_6_col35,
carry_7_col36,
carry_8_col37,
carry_9_col38,
carry_10_col39,
carry_11_col40,
carry_12_col41,
carry_13_col42,
carry_14_col43,
carry_15_col44,
carry_16_col45,
carry_17_col46,
carry_18_col47,
carry_19_col48,
carry_20_col49,
carry_21_col50,
carry_22_col51,
carry_23_col52,
carry_24_col53,
carry_25_col54,
carry_26_col55,
range_check_19_lookup_elements,
ref range_check_19_sum_14,
ref range_check_19_sum_15,
ref range_check_19_sum_16,
ref range_check_19_sum_17,
ref range_check_19_sum_18,
ref range_check_19_sum_19,
ref range_check_19_sum_20,
ref range_check_19_sum_21,
ref range_check_19_sum_22,
ref range_check_19_sum_23,
ref range_check_19_sum_24,
ref range_check_19_sum_25,
ref range_check_19_sum_26,
ref range_check_19_sum_27,
ref range_check_19_sum_28,
ref range_check_19_sum_29,
ref range_check_19_sum_30,
ref range_check_19_sum_31,
ref range_check_19_sum_32,
ref range_check_19_sum_33,
ref range_check_19_sum_34,
ref range_check_19_sum_35,
ref range_check_19_sum_36,
ref range_check_19_sum_37,
ref range_check_19_sum_38,
ref range_check_19_sum_39,
ref range_check_19_sum_40,
ref range_check_19_sum_41,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );


    []
}