// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::mem_verify::MemVerify;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct VerifyU32 {}

impl VerifyU32 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [verify_u_32_input_limb_0, verify_u_32_input_limb_1, verify_u_32_input_limb_2]: [E::F; 3],
        low_7_ms_bits_col0: E::F,
        high_14_ms_bits_col1: E::F,
        high_5_ms_bits_col2: E::F,
        id_col3: E::F,
        range_check_7_2_5_lookup_elements: &relations::RangeCheck_7_2_5,
        memory_address_to_id_lookup_elements: &relations::MemoryAddressToId,
        memory_id_to_big_lookup_elements: &relations::MemoryIdToBig,
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_0 = E::F::from(M31::from(0));
        let M31_128 = E::F::from(M31::from(128));
        let M31_4 = E::F::from(M31::from(4));
        let M31_512 = E::F::from(M31::from(512));

        let high_2_ls_bits_tmp_c4bc0_2 = eval.add_intermediate(
            (verify_u_32_input_limb_2.clone() - (high_14_ms_bits_col1.clone() * M31_4.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col0.clone(),
                high_2_ls_bits_tmp_c4bc0_2.clone(),
                high_5_ms_bits_col2.clone(),
            ],
        ));

        MemVerify::evaluate(
            [
                verify_u_32_input_limb_0.clone(),
                (verify_u_32_input_limb_1.clone() - (low_7_ms_bits_col0.clone() * M31_512.clone())),
                (low_7_ms_bits_col0.clone()
                    + (high_2_ls_bits_tmp_c4bc0_2.clone() * M31_128.clone())),
                (high_14_ms_bits_col1.clone() - (high_5_ms_bits_col2.clone() * M31_512.clone())),
                high_5_ms_bits_col2.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
            ],
            id_col3.clone(),
            memory_address_to_id_lookup_elements,
            memory_id_to_big_lookup_elements,
            eval,
        );
        []
    }
}
