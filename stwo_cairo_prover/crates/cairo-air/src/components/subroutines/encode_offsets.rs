// This file was created by the AIR team.

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct EncodeOffsets {}

impl EncodeOffsets {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [encode_offsets_input_offset0, encode_offsets_input_offset1, encode_offsets_input_offset2]: [E::F; 3],
        offset0_low_col0: E::F,
        offset0_mid_col1: E::F,
        offset1_low_col2: E::F,
        offset1_mid_col3: E::F,
        offset1_high_col4: E::F,
        offset2_low_col5: E::F,
        offset2_mid_col6: E::F,
        offset2_high_col7: E::F,
        range_check_7_2_5_lookup_elements: &relations::RangeCheck_7_2_5,
        range_check_4_3_lookup_elements: &relations::RangeCheck_4_3,
        eval: &mut E,
    ) -> [E::F; 2] {
        let M31_128 = E::F::from(M31::from(128));
        let M31_16 = E::F::from(M31::from(16));
        let M31_2048 = E::F::from(M31::from(2048));
        let M31_32 = E::F::from(M31::from(32));
        let M31_4 = E::F::from(M31::from(4));
        let M31_512 = E::F::from(M31::from(512));
        let M31_8192 = E::F::from(M31::from(8192));

        // Reconstructed offset0 is correct.
        eval.add_constraint(
            ((offset0_low_col0.clone() + (offset0_mid_col1.clone() * M31_512.clone()))
                - encode_offsets_input_offset0.clone()),
        );
        // Reconstructed offset1 is correct.
        eval.add_constraint(
            (((offset1_low_col2.clone() + (offset1_mid_col3.clone() * M31_4.clone()))
                + (offset1_high_col4.clone() * M31_2048.clone()))
                - encode_offsets_input_offset1.clone()),
        );
        // Reconstructed offset2 is correct.
        eval.add_constraint(
            (((offset2_low_col5.clone() + (offset2_mid_col6.clone() * M31_16.clone()))
                + (offset2_high_col7.clone() * M31_8192.clone()))
                - encode_offsets_input_offset2.clone()),
        );
        eval.add_to_relation(RelationEntry::new(
            range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                offset0_mid_col1.clone(),
                offset1_low_col2.clone(),
                offset1_high_col4.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            range_check_4_3_lookup_elements,
            E::EF::one(),
            &[offset2_low_col5.clone(), offset2_high_col7.clone()],
        ));

        [
            (offset0_mid_col1.clone() + (offset1_low_col2.clone() * M31_128.clone())),
            (offset1_high_col4.clone() + (offset2_low_col5.clone() * M31_32.clone())),
        ]
    }
}
