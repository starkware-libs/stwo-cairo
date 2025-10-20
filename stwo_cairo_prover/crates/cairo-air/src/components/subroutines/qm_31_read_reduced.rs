// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::read_positive_num_bits_144::ReadPositiveNumBits144;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Qm31ReadReduced {}

impl Qm31ReadReduced {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [qm_31_read_reduced_input]: [E::F; 1],
        id_col0: E::F,
        value_limb_0_col1: E::F,
        value_limb_1_col2: E::F,
        value_limb_2_col3: E::F,
        value_limb_3_col4: E::F,
        value_limb_4_col5: E::F,
        value_limb_5_col6: E::F,
        value_limb_6_col7: E::F,
        value_limb_7_col8: E::F,
        value_limb_8_col9: E::F,
        value_limb_9_col10: E::F,
        value_limb_10_col11: E::F,
        value_limb_11_col12: E::F,
        value_limb_12_col13: E::F,
        value_limb_13_col14: E::F,
        value_limb_14_col15: E::F,
        value_limb_15_col16: E::F,
        delta_ab_inv_col17: E::F,
        delta_cd_inv_col18: E::F,
        memory_address_to_id_lookup_elements: &relations::MemoryAddressToId,
        memory_id_to_big_lookup_elements: &relations::MemoryIdToBig,
        range_check_4_4_4_4_lookup_elements: &relations::RangeCheck_4_4_4_4,
        eval: &mut E,
    ) -> [E::F; 4] {
        let M31_1 = E::F::from(M31::from(1));
        let M31_134217728 = E::F::from(M31::from(134217728));
        let M31_1548 = E::F::from(M31::from(1548));
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_512 = E::F::from(M31::from(512));

        ReadPositiveNumBits144::evaluate(
            [qm_31_read_reduced_input.clone()],
            id_col0.clone(),
            value_limb_0_col1.clone(),
            value_limb_1_col2.clone(),
            value_limb_2_col3.clone(),
            value_limb_3_col4.clone(),
            value_limb_4_col5.clone(),
            value_limb_5_col6.clone(),
            value_limb_6_col7.clone(),
            value_limb_7_col8.clone(),
            value_limb_8_col9.clone(),
            value_limb_9_col10.clone(),
            value_limb_10_col11.clone(),
            value_limb_11_col12.clone(),
            value_limb_12_col13.clone(),
            value_limb_13_col14.clone(),
            value_limb_14_col15.clone(),
            value_limb_15_col16.clone(),
            memory_address_to_id_lookup_elements,
            memory_id_to_big_lookup_elements,
            eval,
        );
        eval.add_to_relation(RelationEntry::new(
            range_check_4_4_4_4_lookup_elements,
            E::EF::one(),
            &[
                value_limb_3_col4.clone(),
                value_limb_7_col8.clone(),
                value_limb_11_col12.clone(),
                value_limb_15_col16.clone(),
            ],
        ));

        // delta_ab doesn't equal 0.
        eval.add_constraint(
            (((((((value_limb_0_col1.clone() + value_limb_1_col2.clone())
                + value_limb_2_col3.clone())
                + value_limb_3_col4.clone())
                - M31_1548.clone())
                * ((((value_limb_4_col5.clone() + value_limb_5_col6.clone())
                    + value_limb_6_col7.clone())
                    + value_limb_7_col8.clone())
                    - M31_1548.clone()))
                * delta_ab_inv_col17.clone())
                - M31_1.clone()),
        );
        // delta_cd doesn't equal 0.
        eval.add_constraint(
            (((((((value_limb_8_col9.clone() + value_limb_9_col10.clone())
                + value_limb_10_col11.clone())
                + value_limb_11_col12.clone())
                - M31_1548.clone())
                * ((((value_limb_12_col13.clone() + value_limb_13_col14.clone())
                    + value_limb_14_col15.clone())
                    + value_limb_15_col16.clone())
                    - M31_1548.clone()))
                * delta_cd_inv_col18.clone())
                - M31_1.clone()),
        );
        [
            (((value_limb_0_col1.clone() + (value_limb_1_col2.clone() * M31_512.clone()))
                + (value_limb_2_col3.clone() * M31_262144.clone()))
                + (value_limb_3_col4.clone() * M31_134217728.clone())),
            (((value_limb_4_col5.clone() + (value_limb_5_col6.clone() * M31_512.clone()))
                + (value_limb_6_col7.clone() * M31_262144.clone()))
                + (value_limb_7_col8.clone() * M31_134217728.clone())),
            (((value_limb_8_col9.clone() + (value_limb_9_col10.clone() * M31_512.clone()))
                + (value_limb_10_col11.clone() * M31_262144.clone()))
                + (value_limb_11_col12.clone() * M31_134217728.clone())),
            (((value_limb_12_col13.clone() + (value_limb_13_col14.clone() * M31_512.clone()))
                + (value_limb_14_col15.clone() * M31_262144.clone()))
                + (value_limb_15_col16.clone() * M31_134217728.clone())),
        ]
    }
}
