// AIR version eb424657
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
use crate::PreprocessedColumnTrait;
use crate::components::CairoComponent;


pub fn mem_cond_verify_equal_known_id_evaluate(
    input: [QM31; 3],
    id_col0: QM31,
    memory_address_to_id_lookup_elements: @crate::MemoryAddressToIdElements,
    ref memory_address_to_id_sum_0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [
        mem_cond_verify_equal_known_id_input_limb_0,
        mem_cond_verify_equal_known_id_input_limb_1,
        mem_cond_verify_equal_known_id_input_limb_2,
    ] =
        input;

    memory_address_to_id_sum_0 = memory_address_to_id_lookup_elements
        .combine_qm31([mem_cond_verify_equal_known_id_input_limb_0, id_col0]);

    // Constraint - The two ids are equal if the condition is met
    let constraint_quotient = (((id_col0 - mem_cond_verify_equal_known_id_input_limb_1)
        * mem_cond_verify_equal_known_id_input_limb_2))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    []
}
