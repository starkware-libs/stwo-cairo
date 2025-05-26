use crate::components::prelude::*;
use crate::components::subroutines::cond_decode_small_sign::CondDecodeSmallSign;

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
        segment_index: E::F,
        [read_small_input]: [E::F; 1],
        id_col0: E::F,
        msb_col1: E::F,
        mid_limbs_set_col2: E::F,
        value_limb_0_col3: E::F,
        value_limb_1_col4: E::F,
        value_limb_2_col5: E::F,
        memory_address_to_id_lookup_elements: &relations::MemoryAddressToId,
        memory_id_to_big_lookup_elements: &relations::MemoryIdToBig,
        eval: &mut E,
    ) -> [E::F; 1] {
        let M31_0 = E::F::from(M31::from(0));
        let M31_1 = E::F::from(M31::from(1));
        let M31_134217728 = E::F::from(M31::from(134217728));
        let M31_136 = E::F::from(M31::from(136));
        let M31_256 = E::F::from(M31::from(256));
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_511 = E::F::from(M31::from(511));
        let M31_512 = E::F::from(M31::from(512));

        eval.add_to_relation(RelationEntry::new(
            memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[segment_index.clone(), read_small_input.clone(), id_col0.clone()],
        ));

        CondDecodeSmallSign::evaluate(
            [M31_1.clone()],
            msb_col1.clone(),
            mid_limbs_set_col2.clone(),
            eval,
        );
        eval.add_to_relation(RelationEntry::new(
            memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                id_col0.clone(),
                value_limb_0_col3.clone(),
                value_limb_1_col4.clone(),
                value_limb_2_col5.clone(),
                (mid_limbs_set_col2.clone() * M31_511.clone()),
                (mid_limbs_set_col2.clone() * M31_511.clone()),
                (mid_limbs_set_col2.clone() * M31_511.clone()),
                (mid_limbs_set_col2.clone() * M31_511.clone()),
                (mid_limbs_set_col2.clone() * M31_511.clone()),
                (mid_limbs_set_col2.clone() * M31_511.clone()),
                (mid_limbs_set_col2.clone() * M31_511.clone()),
                (mid_limbs_set_col2.clone() * M31_511.clone()),
                (mid_limbs_set_col2.clone() * M31_511.clone()),
                (mid_limbs_set_col2.clone() * M31_511.clone()),
                (mid_limbs_set_col2.clone() * M31_511.clone()),
                (mid_limbs_set_col2.clone() * M31_511.clone()),
                (mid_limbs_set_col2.clone() * M31_511.clone()),
                (mid_limbs_set_col2.clone() * M31_511.clone()),
                (mid_limbs_set_col2.clone() * M31_511.clone()),
                (mid_limbs_set_col2.clone() * M31_511.clone()),
                (mid_limbs_set_col2.clone() * M31_511.clone()),
                (mid_limbs_set_col2.clone() * M31_511.clone()),
                ((M31_136.clone() * msb_col1.clone()) - mid_limbs_set_col2.clone()),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                (msb_col1.clone() * M31_256.clone()),
            ],
        ));

        [
            ((((value_limb_0_col3.clone() + (value_limb_1_col4.clone() * M31_512.clone()))
                + (value_limb_2_col5.clone() * M31_262144.clone()))
                - msb_col1.clone())
                - (M31_134217728.clone() * mid_limbs_set_col2.clone())),
        ]
    }
}
