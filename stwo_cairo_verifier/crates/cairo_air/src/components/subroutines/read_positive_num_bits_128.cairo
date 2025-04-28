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
use crate::components::memory_address_to_id::MEMORY_ADDRESS_TO_ID_RELATION_SIZE;
use crate::components::memory_id_to_big::MEMORY_ID_TO_BIG_RELATION_SIZE;
use crate::components::subroutines::range_check_last_limb_bits_in_ms_limb_2::range_check_last_limb_bits_in_ms_limb_2_evaluate;


pub const N_TRACE_COLUMNS: usize = 17;



pub fn read_positive_num_bits_128_evaluate(
    input: [QM31; 1],
    id_col0: QM31,
value_limb_0_col1: QM31,
value_limb_1_col2: QM31,
value_limb_2_col3: QM31,
value_limb_3_col4: QM31,
value_limb_4_col5: QM31,
value_limb_5_col6: QM31,
value_limb_6_col7: QM31,
value_limb_7_col8: QM31,
value_limb_8_col9: QM31,
value_limb_9_col10: QM31,
value_limb_10_col11: QM31,
value_limb_11_col12: QM31,
value_limb_12_col13: QM31,
value_limb_13_col14: QM31,
value_limb_14_col15: QM31,
msb_col16: QM31,
memory_address_to_id_lookup_elements: @crate::MemoryAddressToIdElements,
memory_id_to_big_lookup_elements: @crate::MemoryIdToBigElements,
ref memory_address_to_id_sum_0: QM31,
ref memory_id_to_big_sum_1: QM31,

    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [read_positive_num_bits_128_input] = input;

    

    memory_address_to_id_sum_0 = memory_address_to_id_lookup_elements.combine_qm31(
        [
            read_positive_num_bits_128_input,
id_col0
        ],
    );


    range_check_last_limb_bits_in_ms_limb_2_evaluate(
            [value_limb_14_col15],
msb_col16,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );


    memory_id_to_big_sum_1 = memory_id_to_big_lookup_elements.combine_qm31(
        [
            id_col0,
value_limb_0_col1,
value_limb_1_col2,
value_limb_2_col3,
value_limb_3_col4,
value_limb_4_col5,
value_limb_5_col6,
value_limb_6_col7,
value_limb_7_col8,
value_limb_8_col9,
value_limb_9_col10,
value_limb_10_col11,
value_limb_11_col12,
value_limb_12_col13,
value_limb_13_col14,
value_limb_14_col15,
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>()
        ],
    );


    []
}