// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::cond_range_check_2::CondRangeCheck2;
use crate::components::subroutines::decode_small_sign::DecodeSmallSign;
use crate::components::subroutines::read_id::ReadId;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct ReadSmall {}

impl ReadSmall {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [read_small_input]: [E::F; 1],
        id_col0: E::F,
        msb_col1: E::F,
        mid_limbs_set_col2: E::F,
        value_limb_0_col3: E::F,
        value_limb_1_col4: E::F,
        value_limb_2_col5: E::F,
        remainder_bits_col6: E::F,
        partial_limb_msb_col7: E::F,
        common_lookup_elements: &relations::CommonLookupElements,
        eval: &mut E,
    ) -> [E::F; 1] {
        let M31_0 = E::F::from(M31::from(0));
        let M31_1 = E::F::from(M31::from(1));
        let M31_134217728 = E::F::from(M31::from(134217728));
        let M31_1662111297 = E::F::from(M31::from(1662111297));
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_512 = E::F::from(M31::from(512));
        let M31_536870912 = E::F::from(M31::from(536870912));

        ReadId::evaluate(
            [read_small_input.clone()],
            id_col0.clone(),
            common_lookup_elements,
            eval,
        );
        let [decode_small_sign_output_tmp_ceaaf_5_limb3_7_high_bits, decode_small_sign_output_tmp_ceaaf_5_limbs4_to_20, decode_small_sign_output_tmp_ceaaf_5_limb21, decode_small_sign_output_tmp_ceaaf_5_limb27] =
            DecodeSmallSign::evaluate(
                [],
                msb_col1.clone(),
                mid_limbs_set_col2.clone(),
                common_lookup_elements,
                eval,
            );
        CondRangeCheck2::evaluate(
            [remainder_bits_col6.clone(), M31_1.clone()],
            partial_limb_msb_col7.clone(),
            common_lookup_elements,
            eval,
        );
        eval.add_to_relation(RelationEntry::new(
            common_lookup_elements,
            E::EF::one(),
            &[
                M31_1662111297.clone(),
                id_col0.clone(),
                value_limb_0_col3.clone(),
                value_limb_1_col4.clone(),
                value_limb_2_col5.clone(),
                (remainder_bits_col6.clone()
                    + decode_small_sign_output_tmp_ceaaf_5_limb3_7_high_bits.clone()),
                decode_small_sign_output_tmp_ceaaf_5_limbs4_to_20.clone(),
                decode_small_sign_output_tmp_ceaaf_5_limbs4_to_20.clone(),
                decode_small_sign_output_tmp_ceaaf_5_limbs4_to_20.clone(),
                decode_small_sign_output_tmp_ceaaf_5_limbs4_to_20.clone(),
                decode_small_sign_output_tmp_ceaaf_5_limbs4_to_20.clone(),
                decode_small_sign_output_tmp_ceaaf_5_limbs4_to_20.clone(),
                decode_small_sign_output_tmp_ceaaf_5_limbs4_to_20.clone(),
                decode_small_sign_output_tmp_ceaaf_5_limbs4_to_20.clone(),
                decode_small_sign_output_tmp_ceaaf_5_limbs4_to_20.clone(),
                decode_small_sign_output_tmp_ceaaf_5_limbs4_to_20.clone(),
                decode_small_sign_output_tmp_ceaaf_5_limbs4_to_20.clone(),
                decode_small_sign_output_tmp_ceaaf_5_limbs4_to_20.clone(),
                decode_small_sign_output_tmp_ceaaf_5_limbs4_to_20.clone(),
                decode_small_sign_output_tmp_ceaaf_5_limbs4_to_20.clone(),
                decode_small_sign_output_tmp_ceaaf_5_limbs4_to_20.clone(),
                decode_small_sign_output_tmp_ceaaf_5_limbs4_to_20.clone(),
                decode_small_sign_output_tmp_ceaaf_5_limbs4_to_20.clone(),
                decode_small_sign_output_tmp_ceaaf_5_limb21.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                decode_small_sign_output_tmp_ceaaf_5_limb27.clone(),
            ],
        ));

        [
            (((((value_limb_0_col3.clone() + (value_limb_1_col4.clone() * M31_512.clone()))
                + (value_limb_2_col5.clone() * M31_262144.clone()))
                + (remainder_bits_col6.clone() * M31_134217728.clone()))
                - msb_col1.clone())
                - (M31_536870912.clone() * mid_limbs_set_col2.clone())),
        ]
    }
}
