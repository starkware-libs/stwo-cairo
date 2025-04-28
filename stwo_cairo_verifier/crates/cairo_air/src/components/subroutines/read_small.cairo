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
use crate::components::memory_address_to_id::{
    MEMORY_ADDRESS_TO_ID_RELATION_SIZE, memory_address_to_id_sum,
};
use crate::components::memory_id_to_big::{MEMORY_ID_TO_BIG_RELATION_SIZE, memory_id_to_big_sum};
use crate::components::subroutines::cond_decode_small_sign::cond_decode_small_sign_evaluate;
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE, opcodes_sum};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 6;


pub fn read_small_evaluate(
    input: QM31,
    id_col0: QM31,
    msb_col1: QM31,
    mid_limbs_set_col2: QM31,
    value_limb_0_col3: QM31,
    value_limb_1_col4: QM31,
    value_limb_2_col5: QM31,
    memory_address_to_id_alphas: Span<QM31>,
    memory_address_to_id_z: QM31,
    memory_id_to_big_alphas: Span<QM31>,
    memory_id_to_big_z: QM31,
    ref memory_address_to_id_sum_0: QM31,
    ref memory_id_to_big_sum_1: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 2] {
    let read_small_input = input;

    memory_address_to_id_sum_0 =
        memory_address_to_id_sum(
            memory_address_to_id_alphas, memory_address_to_id_z, [read_small_input, id_col0],
        );

    let output: [QM31; 2] = cond_decode_small_sign_evaluate(
        [
            qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
            qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
            qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
            qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
            qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
            qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
            qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
            qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
            qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
            qm31_const::<0, 0, 0, 0>(), qm31_const::<1, 0, 0, 0>(),
        ],
        msb_col1,
        mid_limbs_set_col2,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [
        cond_decode_small_sign_output_tmp_ceaaf_4_limb_0,
        cond_decode_small_sign_output_tmp_ceaaf_4_limb_1,
    ] =
        output;

    memory_id_to_big_sum_1 =
        memory_id_to_big_sum(
            memory_id_to_big_alphas,
            memory_id_to_big_z,
            [
                id_col0, value_limb_0_col3, value_limb_1_col4, value_limb_2_col5,
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                ((qm31_const::<136, 0, 0, 0>() * msb_col1) - mid_limbs_set_col2),
                qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                (msb_col1 * qm31_const::<256, 0, 0, 0>()),
            ],
        );

    [
        ((((value_limb_0_col3 + (value_limb_1_col4 * qm31_const::<512, 0, 0, 0>()))
            + (value_limb_2_col5 * qm31_const::<262144, 0, 0, 0>()))
            - msb_col1)
            - (qm31_const::<134217728, 0, 0, 0>() * mid_limbs_set_col2)),
        id_col0,
    ]
}
