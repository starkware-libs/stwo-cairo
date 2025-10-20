// This file was created by the AIR team.

use crate::prelude::*;


pub fn felt_252_unpack_from_27_evaluate(
    input: [QM31; 10],
    unpacked_limb_0_col0: QM31,
    unpacked_limb_1_col1: QM31,
    unpacked_limb_3_col2: QM31,
    unpacked_limb_4_col3: QM31,
    unpacked_limb_6_col4: QM31,
    unpacked_limb_7_col5: QM31,
    unpacked_limb_9_col6: QM31,
    unpacked_limb_10_col7: QM31,
    unpacked_limb_12_col8: QM31,
    unpacked_limb_13_col9: QM31,
    unpacked_limb_15_col10: QM31,
    unpacked_limb_16_col11: QM31,
    unpacked_limb_18_col12: QM31,
    unpacked_limb_19_col13: QM31,
    unpacked_limb_21_col14: QM31,
    unpacked_limb_22_col15: QM31,
    unpacked_limb_24_col16: QM31,
    unpacked_limb_25_col17: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 10] {
    let [
        felt_252_unpack_from_27_input_limb_0,
        felt_252_unpack_from_27_input_limb_1,
        felt_252_unpack_from_27_input_limb_2,
        felt_252_unpack_from_27_input_limb_3,
        felt_252_unpack_from_27_input_limb_4,
        felt_252_unpack_from_27_input_limb_5,
        felt_252_unpack_from_27_input_limb_6,
        felt_252_unpack_from_27_input_limb_7,
        felt_252_unpack_from_27_input_limb_8,
        felt_252_unpack_from_27_input_limb_9,
    ] =
        input;

    [
        (((felt_252_unpack_from_27_input_limb_0 - unpacked_limb_0_col0)
            - (unpacked_limb_1_col1 * qm31_const::<512, 0, 0, 0>()))
            * qm31_const::<8192, 0, 0, 0>()),
        (((felt_252_unpack_from_27_input_limb_1 - unpacked_limb_3_col2)
            - (unpacked_limb_4_col3 * qm31_const::<512, 0, 0, 0>()))
            * qm31_const::<8192, 0, 0, 0>()),
        (((felt_252_unpack_from_27_input_limb_2 - unpacked_limb_6_col4)
            - (unpacked_limb_7_col5 * qm31_const::<512, 0, 0, 0>()))
            * qm31_const::<8192, 0, 0, 0>()),
        (((felt_252_unpack_from_27_input_limb_3 - unpacked_limb_9_col6)
            - (unpacked_limb_10_col7 * qm31_const::<512, 0, 0, 0>()))
            * qm31_const::<8192, 0, 0, 0>()),
        (((felt_252_unpack_from_27_input_limb_4 - unpacked_limb_12_col8)
            - (unpacked_limb_13_col9 * qm31_const::<512, 0, 0, 0>()))
            * qm31_const::<8192, 0, 0, 0>()),
        (((felt_252_unpack_from_27_input_limb_5 - unpacked_limb_15_col10)
            - (unpacked_limb_16_col11 * qm31_const::<512, 0, 0, 0>()))
            * qm31_const::<8192, 0, 0, 0>()),
        (((felt_252_unpack_from_27_input_limb_6 - unpacked_limb_18_col12)
            - (unpacked_limb_19_col13 * qm31_const::<512, 0, 0, 0>()))
            * qm31_const::<8192, 0, 0, 0>()),
        (((felt_252_unpack_from_27_input_limb_7 - unpacked_limb_21_col14)
            - (unpacked_limb_22_col15 * qm31_const::<512, 0, 0, 0>()))
            * qm31_const::<8192, 0, 0, 0>()),
        (((felt_252_unpack_from_27_input_limb_8 - unpacked_limb_24_col16)
            - (unpacked_limb_25_col17 * qm31_const::<512, 0, 0, 0>()))
            * qm31_const::<8192, 0, 0, 0>()),
        felt_252_unpack_from_27_input_limb_9,
    ]
}
