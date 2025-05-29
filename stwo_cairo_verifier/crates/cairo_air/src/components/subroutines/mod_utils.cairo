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
use crate::components::subroutines::mem_cond_verify_equal_known_id::mem_cond_verify_equal_known_id_evaluate;
use crate::components::subroutines::read_positive_num_bits_27::read_positive_num_bits_27_evaluate;
use crate::components::subroutines::read_positive_num_bits_99::read_positive_num_bits_99_evaluate;
use crate::components::subroutines::read_small::read_small_evaluate;


pub fn mod_utils_evaluate(
    input: [QM31; 2],
    is_instance_0_col0: QM31,
    p0_id_col1: QM31,
    p0_limb_0_col2: QM31,
    p0_limb_1_col3: QM31,
    p0_limb_2_col4: QM31,
    p0_limb_3_col5: QM31,
    p0_limb_4_col6: QM31,
    p0_limb_5_col7: QM31,
    p0_limb_6_col8: QM31,
    p0_limb_7_col9: QM31,
    p0_limb_8_col10: QM31,
    p0_limb_9_col11: QM31,
    p0_limb_10_col12: QM31,
    p1_id_col13: QM31,
    p1_limb_0_col14: QM31,
    p1_limb_1_col15: QM31,
    p1_limb_2_col16: QM31,
    p1_limb_3_col17: QM31,
    p1_limb_4_col18: QM31,
    p1_limb_5_col19: QM31,
    p1_limb_6_col20: QM31,
    p1_limb_7_col21: QM31,
    p1_limb_8_col22: QM31,
    p1_limb_9_col23: QM31,
    p1_limb_10_col24: QM31,
    p2_id_col25: QM31,
    p2_limb_0_col26: QM31,
    p2_limb_1_col27: QM31,
    p2_limb_2_col28: QM31,
    p2_limb_3_col29: QM31,
    p2_limb_4_col30: QM31,
    p2_limb_5_col31: QM31,
    p2_limb_6_col32: QM31,
    p2_limb_7_col33: QM31,
    p2_limb_8_col34: QM31,
    p2_limb_9_col35: QM31,
    p2_limb_10_col36: QM31,
    p3_id_col37: QM31,
    p3_limb_0_col38: QM31,
    p3_limb_1_col39: QM31,
    p3_limb_2_col40: QM31,
    p3_limb_3_col41: QM31,
    p3_limb_4_col42: QM31,
    p3_limb_5_col43: QM31,
    p3_limb_6_col44: QM31,
    p3_limb_7_col45: QM31,
    p3_limb_8_col46: QM31,
    p3_limb_9_col47: QM31,
    p3_limb_10_col48: QM31,
    values_ptr_id_col49: QM31,
    values_ptr_limb_0_col50: QM31,
    values_ptr_limb_1_col51: QM31,
    values_ptr_limb_2_col52: QM31,
    offsets_ptr_id_col53: QM31,
    offsets_ptr_limb_0_col54: QM31,
    offsets_ptr_limb_1_col55: QM31,
    offsets_ptr_limb_2_col56: QM31,
    offsets_ptr_prev_id_col57: QM31,
    offsets_ptr_prev_limb_0_col58: QM31,
    offsets_ptr_prev_limb_1_col59: QM31,
    offsets_ptr_prev_limb_2_col60: QM31,
    n_id_col61: QM31,
    n_limb_0_col62: QM31,
    n_limb_1_col63: QM31,
    n_limb_2_col64: QM31,
    n_prev_id_col65: QM31,
    n_prev_limb_0_col66: QM31,
    n_prev_limb_1_col67: QM31,
    n_prev_limb_2_col68: QM31,
    values_ptr_prev_id_col69: QM31,
    p_prev0_id_col70: QM31,
    p_prev1_id_col71: QM31,
    p_prev2_id_col72: QM31,
    p_prev3_id_col73: QM31,
    offsets_a_id_col74: QM31,
    msb_col75: QM31,
    mid_limbs_set_col76: QM31,
    offsets_a_limb_0_col77: QM31,
    offsets_a_limb_1_col78: QM31,
    offsets_a_limb_2_col79: QM31,
    offsets_b_id_col80: QM31,
    msb_col81: QM31,
    mid_limbs_set_col82: QM31,
    offsets_b_limb_0_col83: QM31,
    offsets_b_limb_1_col84: QM31,
    offsets_b_limb_2_col85: QM31,
    offsets_c_id_col86: QM31,
    msb_col87: QM31,
    mid_limbs_set_col88: QM31,
    offsets_c_limb_0_col89: QM31,
    offsets_c_limb_1_col90: QM31,
    offsets_c_limb_2_col91: QM31,
    a0_id_col92: QM31,
    a0_limb_0_col93: QM31,
    a0_limb_1_col94: QM31,
    a0_limb_2_col95: QM31,
    a0_limb_3_col96: QM31,
    a0_limb_4_col97: QM31,
    a0_limb_5_col98: QM31,
    a0_limb_6_col99: QM31,
    a0_limb_7_col100: QM31,
    a0_limb_8_col101: QM31,
    a0_limb_9_col102: QM31,
    a0_limb_10_col103: QM31,
    a1_id_col104: QM31,
    a1_limb_0_col105: QM31,
    a1_limb_1_col106: QM31,
    a1_limb_2_col107: QM31,
    a1_limb_3_col108: QM31,
    a1_limb_4_col109: QM31,
    a1_limb_5_col110: QM31,
    a1_limb_6_col111: QM31,
    a1_limb_7_col112: QM31,
    a1_limb_8_col113: QM31,
    a1_limb_9_col114: QM31,
    a1_limb_10_col115: QM31,
    a2_id_col116: QM31,
    a2_limb_0_col117: QM31,
    a2_limb_1_col118: QM31,
    a2_limb_2_col119: QM31,
    a2_limb_3_col120: QM31,
    a2_limb_4_col121: QM31,
    a2_limb_5_col122: QM31,
    a2_limb_6_col123: QM31,
    a2_limb_7_col124: QM31,
    a2_limb_8_col125: QM31,
    a2_limb_9_col126: QM31,
    a2_limb_10_col127: QM31,
    a3_id_col128: QM31,
    a3_limb_0_col129: QM31,
    a3_limb_1_col130: QM31,
    a3_limb_2_col131: QM31,
    a3_limb_3_col132: QM31,
    a3_limb_4_col133: QM31,
    a3_limb_5_col134: QM31,
    a3_limb_6_col135: QM31,
    a3_limb_7_col136: QM31,
    a3_limb_8_col137: QM31,
    a3_limb_9_col138: QM31,
    a3_limb_10_col139: QM31,
    b0_id_col140: QM31,
    b0_limb_0_col141: QM31,
    b0_limb_1_col142: QM31,
    b0_limb_2_col143: QM31,
    b0_limb_3_col144: QM31,
    b0_limb_4_col145: QM31,
    b0_limb_5_col146: QM31,
    b0_limb_6_col147: QM31,
    b0_limb_7_col148: QM31,
    b0_limb_8_col149: QM31,
    b0_limb_9_col150: QM31,
    b0_limb_10_col151: QM31,
    b1_id_col152: QM31,
    b1_limb_0_col153: QM31,
    b1_limb_1_col154: QM31,
    b1_limb_2_col155: QM31,
    b1_limb_3_col156: QM31,
    b1_limb_4_col157: QM31,
    b1_limb_5_col158: QM31,
    b1_limb_6_col159: QM31,
    b1_limb_7_col160: QM31,
    b1_limb_8_col161: QM31,
    b1_limb_9_col162: QM31,
    b1_limb_10_col163: QM31,
    b2_id_col164: QM31,
    b2_limb_0_col165: QM31,
    b2_limb_1_col166: QM31,
    b2_limb_2_col167: QM31,
    b2_limb_3_col168: QM31,
    b2_limb_4_col169: QM31,
    b2_limb_5_col170: QM31,
    b2_limb_6_col171: QM31,
    b2_limb_7_col172: QM31,
    b2_limb_8_col173: QM31,
    b2_limb_9_col174: QM31,
    b2_limb_10_col175: QM31,
    b3_id_col176: QM31,
    b3_limb_0_col177: QM31,
    b3_limb_1_col178: QM31,
    b3_limb_2_col179: QM31,
    b3_limb_3_col180: QM31,
    b3_limb_4_col181: QM31,
    b3_limb_5_col182: QM31,
    b3_limb_6_col183: QM31,
    b3_limb_7_col184: QM31,
    b3_limb_8_col185: QM31,
    b3_limb_9_col186: QM31,
    b3_limb_10_col187: QM31,
    c0_id_col188: QM31,
    c0_limb_0_col189: QM31,
    c0_limb_1_col190: QM31,
    c0_limb_2_col191: QM31,
    c0_limb_3_col192: QM31,
    c0_limb_4_col193: QM31,
    c0_limb_5_col194: QM31,
    c0_limb_6_col195: QM31,
    c0_limb_7_col196: QM31,
    c0_limb_8_col197: QM31,
    c0_limb_9_col198: QM31,
    c0_limb_10_col199: QM31,
    c1_id_col200: QM31,
    c1_limb_0_col201: QM31,
    c1_limb_1_col202: QM31,
    c1_limb_2_col203: QM31,
    c1_limb_3_col204: QM31,
    c1_limb_4_col205: QM31,
    c1_limb_5_col206: QM31,
    c1_limb_6_col207: QM31,
    c1_limb_7_col208: QM31,
    c1_limb_8_col209: QM31,
    c1_limb_9_col210: QM31,
    c1_limb_10_col211: QM31,
    c2_id_col212: QM31,
    c2_limb_0_col213: QM31,
    c2_limb_1_col214: QM31,
    c2_limb_2_col215: QM31,
    c2_limb_3_col216: QM31,
    c2_limb_4_col217: QM31,
    c2_limb_5_col218: QM31,
    c2_limb_6_col219: QM31,
    c2_limb_7_col220: QM31,
    c2_limb_8_col221: QM31,
    c2_limb_9_col222: QM31,
    c2_limb_10_col223: QM31,
    c3_id_col224: QM31,
    c3_limb_0_col225: QM31,
    c3_limb_1_col226: QM31,
    c3_limb_2_col227: QM31,
    c3_limb_3_col228: QM31,
    c3_limb_4_col229: QM31,
    c3_limb_5_col230: QM31,
    c3_limb_6_col231: QM31,
    c3_limb_7_col232: QM31,
    c3_limb_8_col233: QM31,
    c3_limb_9_col234: QM31,
    c3_limb_10_col235: QM31,
    memory_address_to_id_lookup_elements: @crate::MemoryAddressToIdElements,
    memory_id_to_big_lookup_elements: @crate::MemoryIdToBigElements,
    ref memory_address_to_id_sum_0: QM31,
    ref memory_id_to_big_sum_1: QM31,
    ref memory_address_to_id_sum_2: QM31,
    ref memory_id_to_big_sum_3: QM31,
    ref memory_address_to_id_sum_4: QM31,
    ref memory_id_to_big_sum_5: QM31,
    ref memory_address_to_id_sum_6: QM31,
    ref memory_id_to_big_sum_7: QM31,
    ref memory_address_to_id_sum_8: QM31,
    ref memory_id_to_big_sum_9: QM31,
    ref memory_address_to_id_sum_10: QM31,
    ref memory_id_to_big_sum_11: QM31,
    ref memory_address_to_id_sum_12: QM31,
    ref memory_id_to_big_sum_13: QM31,
    ref memory_address_to_id_sum_14: QM31,
    ref memory_id_to_big_sum_15: QM31,
    ref memory_address_to_id_sum_16: QM31,
    ref memory_id_to_big_sum_17: QM31,
    ref memory_address_to_id_sum_18: QM31,
    ref memory_address_to_id_sum_19: QM31,
    ref memory_address_to_id_sum_20: QM31,
    ref memory_address_to_id_sum_21: QM31,
    ref memory_address_to_id_sum_22: QM31,
    ref memory_address_to_id_sum_23: QM31,
    ref memory_id_to_big_sum_24: QM31,
    ref memory_address_to_id_sum_25: QM31,
    ref memory_id_to_big_sum_26: QM31,
    ref memory_address_to_id_sum_27: QM31,
    ref memory_id_to_big_sum_28: QM31,
    ref memory_address_to_id_sum_29: QM31,
    ref memory_id_to_big_sum_30: QM31,
    ref memory_address_to_id_sum_31: QM31,
    ref memory_id_to_big_sum_32: QM31,
    ref memory_address_to_id_sum_33: QM31,
    ref memory_id_to_big_sum_34: QM31,
    ref memory_address_to_id_sum_35: QM31,
    ref memory_id_to_big_sum_36: QM31,
    ref memory_address_to_id_sum_37: QM31,
    ref memory_id_to_big_sum_38: QM31,
    ref memory_address_to_id_sum_39: QM31,
    ref memory_id_to_big_sum_40: QM31,
    ref memory_address_to_id_sum_41: QM31,
    ref memory_id_to_big_sum_42: QM31,
    ref memory_address_to_id_sum_43: QM31,
    ref memory_id_to_big_sum_44: QM31,
    ref memory_address_to_id_sum_45: QM31,
    ref memory_id_to_big_sum_46: QM31,
    ref memory_address_to_id_sum_47: QM31,
    ref memory_id_to_big_sum_48: QM31,
    ref memory_address_to_id_sum_49: QM31,
    ref memory_id_to_big_sum_50: QM31,
    ref memory_address_to_id_sum_51: QM31,
    ref memory_id_to_big_sum_52: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [mod_utils_input_limb_0, mod_utils_input_limb_1] = input;

    // Constraint - is_instance_0 is 0 or 1.
    let constraint_quotient = ((is_instance_0_col0
        * (is_instance_0_col0 - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - is_instance_0 is 0 when instance_num is not 0.
    let constraint_quotient = ((is_instance_0_col0 * mod_utils_input_limb_1))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let prev_instance_addr_tmp_7b599_1: QM31 = (mod_utils_input_limb_0
        + (qm31_const::<7, 0, 0, 0>()
            * ((mod_utils_input_limb_1 - qm31_const::<1, 0, 0, 0>()) + is_instance_0_col0)));
    let instance_addr_tmp_7b599_2: QM31 = (mod_utils_input_limb_0
        + (qm31_const::<7, 0, 0, 0>() * mod_utils_input_limb_1));

    read_positive_num_bits_99_evaluate(
        [instance_addr_tmp_7b599_2],
        p0_id_col1,
        p0_limb_0_col2,
        p0_limb_1_col3,
        p0_limb_2_col4,
        p0_limb_3_col5,
        p0_limb_4_col6,
        p0_limb_5_col7,
        p0_limb_6_col8,
        p0_limb_7_col9,
        p0_limb_8_col10,
        p0_limb_9_col11,
        p0_limb_10_col12,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_0,
        ref memory_id_to_big_sum_1,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_positive_num_bits_99_evaluate(
        [(instance_addr_tmp_7b599_2 + qm31_const::<1, 0, 0, 0>())],
        p1_id_col13,
        p1_limb_0_col14,
        p1_limb_1_col15,
        p1_limb_2_col16,
        p1_limb_3_col17,
        p1_limb_4_col18,
        p1_limb_5_col19,
        p1_limb_6_col20,
        p1_limb_7_col21,
        p1_limb_8_col22,
        p1_limb_9_col23,
        p1_limb_10_col24,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_2,
        ref memory_id_to_big_sum_3,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_positive_num_bits_99_evaluate(
        [(instance_addr_tmp_7b599_2 + qm31_const::<2, 0, 0, 0>())],
        p2_id_col25,
        p2_limb_0_col26,
        p2_limb_1_col27,
        p2_limb_2_col28,
        p2_limb_3_col29,
        p2_limb_4_col30,
        p2_limb_5_col31,
        p2_limb_6_col32,
        p2_limb_7_col33,
        p2_limb_8_col34,
        p2_limb_9_col35,
        p2_limb_10_col36,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_4,
        ref memory_id_to_big_sum_5,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_positive_num_bits_99_evaluate(
        [(instance_addr_tmp_7b599_2 + qm31_const::<3, 0, 0, 0>())],
        p3_id_col37,
        p3_limb_0_col38,
        p3_limb_1_col39,
        p3_limb_2_col40,
        p3_limb_3_col41,
        p3_limb_4_col42,
        p3_limb_5_col43,
        p3_limb_6_col44,
        p3_limb_7_col45,
        p3_limb_8_col46,
        p3_limb_9_col47,
        p3_limb_10_col48,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_6,
        ref memory_id_to_big_sum_7,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_positive_num_bits_27_evaluate(
        [(instance_addr_tmp_7b599_2 + qm31_const::<4, 0, 0, 0>())],
        values_ptr_id_col49,
        values_ptr_limb_0_col50,
        values_ptr_limb_1_col51,
        values_ptr_limb_2_col52,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_8,
        ref memory_id_to_big_sum_9,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_positive_num_bits_27_evaluate(
        [(instance_addr_tmp_7b599_2 + qm31_const::<5, 0, 0, 0>())],
        offsets_ptr_id_col53,
        offsets_ptr_limb_0_col54,
        offsets_ptr_limb_1_col55,
        offsets_ptr_limb_2_col56,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_10,
        ref memory_id_to_big_sum_11,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_positive_num_bits_27_evaluate(
        [(prev_instance_addr_tmp_7b599_1 + qm31_const::<5, 0, 0, 0>())],
        offsets_ptr_prev_id_col57,
        offsets_ptr_prev_limb_0_col58,
        offsets_ptr_prev_limb_1_col59,
        offsets_ptr_prev_limb_2_col60,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_12,
        ref memory_id_to_big_sum_13,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_positive_num_bits_27_evaluate(
        [(instance_addr_tmp_7b599_2 + qm31_const::<6, 0, 0, 0>())],
        n_id_col61,
        n_limb_0_col62,
        n_limb_1_col63,
        n_limb_2_col64,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_14,
        ref memory_id_to_big_sum_15,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_positive_num_bits_27_evaluate(
        [(prev_instance_addr_tmp_7b599_1 + qm31_const::<6, 0, 0, 0>())],
        n_prev_id_col65,
        n_prev_limb_0_col66,
        n_prev_limb_1_col67,
        n_prev_limb_2_col68,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_16,
        ref memory_id_to_big_sum_17,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let block_reset_condition_tmp_7b599_30: QM31 = ((((n_prev_limb_0_col66
        + (n_prev_limb_1_col67 * qm31_const::<512, 0, 0, 0>()))
        + (n_prev_limb_2_col68 * qm31_const::<262144, 0, 0, 0>()))
        - qm31_const::<1, 0, 0, 0>())
        * (is_instance_0_col0 - qm31_const::<1, 0, 0, 0>()));

    // Constraint - Progression of n between instances.
    let constraint_quotient = ((block_reset_condition_tmp_7b599_30
        * ((((n_prev_limb_0_col66 + (n_prev_limb_1_col67 * qm31_const::<512, 0, 0, 0>()))
            + (n_prev_limb_2_col68 * qm31_const::<262144, 0, 0, 0>()))
            - qm31_const::<1, 0, 0, 0>())
            - ((n_limb_0_col62 + (n_limb_1_col63 * qm31_const::<512, 0, 0, 0>()))
                + (n_limb_2_col64 * qm31_const::<262144, 0, 0, 0>())))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Progression of offsets_ptr between instances.
    let constraint_quotient = ((block_reset_condition_tmp_7b599_30
        * ((((offsets_ptr_limb_0_col54 + (offsets_ptr_limb_1_col55 * qm31_const::<512, 0, 0, 0>()))
            + (offsets_ptr_limb_2_col56 * qm31_const::<262144, 0, 0, 0>()))
            - qm31_const::<3, 0, 0, 0>())
            - ((offsets_ptr_prev_limb_0_col58
                + (offsets_ptr_prev_limb_1_col59 * qm31_const::<512, 0, 0, 0>()))
                + (offsets_ptr_prev_limb_2_col60 * qm31_const::<262144, 0, 0, 0>())))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    mem_cond_verify_equal_known_id_evaluate(
        [
            (prev_instance_addr_tmp_7b599_1 + qm31_const::<4, 0, 0, 0>()), values_ptr_id_col49,
            block_reset_condition_tmp_7b599_30,
        ],
        values_ptr_prev_id_col69,
        memory_address_to_id_lookup_elements,
        ref memory_address_to_id_sum_18,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    mem_cond_verify_equal_known_id_evaluate(
        [prev_instance_addr_tmp_7b599_1, p0_id_col1, block_reset_condition_tmp_7b599_30],
        p_prev0_id_col70,
        memory_address_to_id_lookup_elements,
        ref memory_address_to_id_sum_19,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    mem_cond_verify_equal_known_id_evaluate(
        [
            (prev_instance_addr_tmp_7b599_1 + qm31_const::<1, 0, 0, 0>()), p1_id_col13,
            block_reset_condition_tmp_7b599_30,
        ],
        p_prev1_id_col71,
        memory_address_to_id_lookup_elements,
        ref memory_address_to_id_sum_20,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    mem_cond_verify_equal_known_id_evaluate(
        [
            (prev_instance_addr_tmp_7b599_1 + qm31_const::<2, 0, 0, 0>()), p2_id_col25,
            block_reset_condition_tmp_7b599_30,
        ],
        p_prev2_id_col72,
        memory_address_to_id_lookup_elements,
        ref memory_address_to_id_sum_21,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    mem_cond_verify_equal_known_id_evaluate(
        [
            (prev_instance_addr_tmp_7b599_1 + qm31_const::<3, 0, 0, 0>()), p3_id_col37,
            block_reset_condition_tmp_7b599_30,
        ],
        p_prev3_id_col73,
        memory_address_to_id_lookup_elements,
        ref memory_address_to_id_sum_22,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    let output: [QM31; 1] = read_small_evaluate(
        [
            ((offsets_ptr_limb_0_col54 + (offsets_ptr_limb_1_col55 * qm31_const::<512, 0, 0, 0>()))
                + (offsets_ptr_limb_2_col56 * qm31_const::<262144, 0, 0, 0>()))
        ],
        offsets_a_id_col74,
        msb_col75,
        mid_limbs_set_col76,
        offsets_a_limb_0_col77,
        offsets_a_limb_1_col78,
        offsets_a_limb_2_col79,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_23,
        ref memory_id_to_big_sum_24,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [read_small_output_tmp_7b599_41_limb_0] = output;

    let output: [QM31; 1] = read_small_evaluate(
        [
            (((offsets_ptr_limb_0_col54 + (offsets_ptr_limb_1_col55 * qm31_const::<512, 0, 0, 0>()))
                + (offsets_ptr_limb_2_col56 * qm31_const::<262144, 0, 0, 0>()))
                + qm31_const::<1, 0, 0, 0>())
        ],
        offsets_b_id_col80,
        msb_col81,
        mid_limbs_set_col82,
        offsets_b_limb_0_col83,
        offsets_b_limb_1_col84,
        offsets_b_limb_2_col85,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_25,
        ref memory_id_to_big_sum_26,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [read_small_output_tmp_7b599_47_limb_0] = output;

    let output: [QM31; 1] = read_small_evaluate(
        [
            (((offsets_ptr_limb_0_col54 + (offsets_ptr_limb_1_col55 * qm31_const::<512, 0, 0, 0>()))
                + (offsets_ptr_limb_2_col56 * qm31_const::<262144, 0, 0, 0>()))
                + qm31_const::<2, 0, 0, 0>())
        ],
        offsets_c_id_col86,
        msb_col87,
        mid_limbs_set_col88,
        offsets_c_limb_0_col89,
        offsets_c_limb_1_col90,
        offsets_c_limb_2_col91,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_27,
        ref memory_id_to_big_sum_28,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [read_small_output_tmp_7b599_53_limb_0] = output;
    let values_ptr_tmp_7b599_54: QM31 = ((values_ptr_limb_0_col50
        + (values_ptr_limb_1_col51 * qm31_const::<512, 0, 0, 0>()))
        + (values_ptr_limb_2_col52 * qm31_const::<262144, 0, 0, 0>()));

    read_positive_num_bits_99_evaluate(
        [(values_ptr_tmp_7b599_54 + read_small_output_tmp_7b599_41_limb_0)],
        a0_id_col92,
        a0_limb_0_col93,
        a0_limb_1_col94,
        a0_limb_2_col95,
        a0_limb_3_col96,
        a0_limb_4_col97,
        a0_limb_5_col98,
        a0_limb_6_col99,
        a0_limb_7_col100,
        a0_limb_8_col101,
        a0_limb_9_col102,
        a0_limb_10_col103,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_29,
        ref memory_id_to_big_sum_30,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_positive_num_bits_99_evaluate(
        [
            ((values_ptr_tmp_7b599_54 + read_small_output_tmp_7b599_41_limb_0)
                + qm31_const::<1, 0, 0, 0>())
        ],
        a1_id_col104,
        a1_limb_0_col105,
        a1_limb_1_col106,
        a1_limb_2_col107,
        a1_limb_3_col108,
        a1_limb_4_col109,
        a1_limb_5_col110,
        a1_limb_6_col111,
        a1_limb_7_col112,
        a1_limb_8_col113,
        a1_limb_9_col114,
        a1_limb_10_col115,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_31,
        ref memory_id_to_big_sum_32,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_positive_num_bits_99_evaluate(
        [
            ((values_ptr_tmp_7b599_54 + read_small_output_tmp_7b599_41_limb_0)
                + qm31_const::<2, 0, 0, 0>())
        ],
        a2_id_col116,
        a2_limb_0_col117,
        a2_limb_1_col118,
        a2_limb_2_col119,
        a2_limb_3_col120,
        a2_limb_4_col121,
        a2_limb_5_col122,
        a2_limb_6_col123,
        a2_limb_7_col124,
        a2_limb_8_col125,
        a2_limb_9_col126,
        a2_limb_10_col127,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_33,
        ref memory_id_to_big_sum_34,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_positive_num_bits_99_evaluate(
        [
            ((values_ptr_tmp_7b599_54 + read_small_output_tmp_7b599_41_limb_0)
                + qm31_const::<3, 0, 0, 0>())
        ],
        a3_id_col128,
        a3_limb_0_col129,
        a3_limb_1_col130,
        a3_limb_2_col131,
        a3_limb_3_col132,
        a3_limb_4_col133,
        a3_limb_5_col134,
        a3_limb_6_col135,
        a3_limb_7_col136,
        a3_limb_8_col137,
        a3_limb_9_col138,
        a3_limb_10_col139,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_35,
        ref memory_id_to_big_sum_36,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_positive_num_bits_99_evaluate(
        [(values_ptr_tmp_7b599_54 + read_small_output_tmp_7b599_47_limb_0)],
        b0_id_col140,
        b0_limb_0_col141,
        b0_limb_1_col142,
        b0_limb_2_col143,
        b0_limb_3_col144,
        b0_limb_4_col145,
        b0_limb_5_col146,
        b0_limb_6_col147,
        b0_limb_7_col148,
        b0_limb_8_col149,
        b0_limb_9_col150,
        b0_limb_10_col151,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_37,
        ref memory_id_to_big_sum_38,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_positive_num_bits_99_evaluate(
        [
            ((values_ptr_tmp_7b599_54 + read_small_output_tmp_7b599_47_limb_0)
                + qm31_const::<1, 0, 0, 0>())
        ],
        b1_id_col152,
        b1_limb_0_col153,
        b1_limb_1_col154,
        b1_limb_2_col155,
        b1_limb_3_col156,
        b1_limb_4_col157,
        b1_limb_5_col158,
        b1_limb_6_col159,
        b1_limb_7_col160,
        b1_limb_8_col161,
        b1_limb_9_col162,
        b1_limb_10_col163,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_39,
        ref memory_id_to_big_sum_40,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_positive_num_bits_99_evaluate(
        [
            ((values_ptr_tmp_7b599_54 + read_small_output_tmp_7b599_47_limb_0)
                + qm31_const::<2, 0, 0, 0>())
        ],
        b2_id_col164,
        b2_limb_0_col165,
        b2_limb_1_col166,
        b2_limb_2_col167,
        b2_limb_3_col168,
        b2_limb_4_col169,
        b2_limb_5_col170,
        b2_limb_6_col171,
        b2_limb_7_col172,
        b2_limb_8_col173,
        b2_limb_9_col174,
        b2_limb_10_col175,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_41,
        ref memory_id_to_big_sum_42,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_positive_num_bits_99_evaluate(
        [
            ((values_ptr_tmp_7b599_54 + read_small_output_tmp_7b599_47_limb_0)
                + qm31_const::<3, 0, 0, 0>())
        ],
        b3_id_col176,
        b3_limb_0_col177,
        b3_limb_1_col178,
        b3_limb_2_col179,
        b3_limb_3_col180,
        b3_limb_4_col181,
        b3_limb_5_col182,
        b3_limb_6_col183,
        b3_limb_7_col184,
        b3_limb_8_col185,
        b3_limb_9_col186,
        b3_limb_10_col187,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_43,
        ref memory_id_to_big_sum_44,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_positive_num_bits_99_evaluate(
        [(values_ptr_tmp_7b599_54 + read_small_output_tmp_7b599_53_limb_0)],
        c0_id_col188,
        c0_limb_0_col189,
        c0_limb_1_col190,
        c0_limb_2_col191,
        c0_limb_3_col192,
        c0_limb_4_col193,
        c0_limb_5_col194,
        c0_limb_6_col195,
        c0_limb_7_col196,
        c0_limb_8_col197,
        c0_limb_9_col198,
        c0_limb_10_col199,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_45,
        ref memory_id_to_big_sum_46,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_positive_num_bits_99_evaluate(
        [
            ((values_ptr_tmp_7b599_54 + read_small_output_tmp_7b599_53_limb_0)
                + qm31_const::<1, 0, 0, 0>())
        ],
        c1_id_col200,
        c1_limb_0_col201,
        c1_limb_1_col202,
        c1_limb_2_col203,
        c1_limb_3_col204,
        c1_limb_4_col205,
        c1_limb_5_col206,
        c1_limb_6_col207,
        c1_limb_7_col208,
        c1_limb_8_col209,
        c1_limb_9_col210,
        c1_limb_10_col211,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_47,
        ref memory_id_to_big_sum_48,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_positive_num_bits_99_evaluate(
        [
            ((values_ptr_tmp_7b599_54 + read_small_output_tmp_7b599_53_limb_0)
                + qm31_const::<2, 0, 0, 0>())
        ],
        c2_id_col212,
        c2_limb_0_col213,
        c2_limb_1_col214,
        c2_limb_2_col215,
        c2_limb_3_col216,
        c2_limb_4_col217,
        c2_limb_5_col218,
        c2_limb_6_col219,
        c2_limb_7_col220,
        c2_limb_8_col221,
        c2_limb_9_col222,
        c2_limb_10_col223,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_49,
        ref memory_id_to_big_sum_50,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_positive_num_bits_99_evaluate(
        [
            ((values_ptr_tmp_7b599_54 + read_small_output_tmp_7b599_53_limb_0)
                + qm31_const::<3, 0, 0, 0>())
        ],
        c3_id_col224,
        c3_limb_0_col225,
        c3_limb_1_col226,
        c3_limb_2_col227,
        c3_limb_3_col228,
        c3_limb_4_col229,
        c3_limb_5_col230,
        c3_limb_6_col231,
        c3_limb_7_col232,
        c3_limb_8_col233,
        c3_limb_9_col234,
        c3_limb_10_col235,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_51,
        ref memory_id_to_big_sum_52,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    []
}
