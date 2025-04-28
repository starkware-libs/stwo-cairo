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


pub const N_TRACE_COLUMNS: usize = 1;



pub fn mem_verify_equal_evaluate(
    input: [QM31; 2],
    id_col0: QM31,
memory_address_to_id_lookup_elements: @crate::MemoryAddressToIdElements,
ref memory_address_to_id_sum_0: QM31,
ref memory_address_to_id_sum_1: QM31,

    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [mem_verify_equal_input_address1, mem_verify_equal_input_address2] = input;

    

    memory_address_to_id_sum_0 = memory_address_to_id_lookup_elements.combine_qm31(
        [
            mem_verify_equal_input_address1,
id_col0
        ],
    );


    memory_address_to_id_sum_1 = memory_address_to_id_lookup_elements.combine_qm31(
        [
            mem_verify_equal_input_address2,
id_col0
        ],
    );


    []
}