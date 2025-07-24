use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize, CompactBinary)]
pub struct CondFelt252AsAddr {}

impl CondFelt252AsAddr {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [cond_felt_252_as_addr_input_limb_0, cond_felt_252_as_addr_input_limb_1, cond_felt_252_as_addr_input_limb_2, cond_felt_252_as_addr_input_limb_3, cond_felt_252_as_addr_input_limb_4, cond_felt_252_as_addr_input_limb_5, cond_felt_252_as_addr_input_limb_6, cond_felt_252_as_addr_input_limb_7, cond_felt_252_as_addr_input_limb_8, cond_felt_252_as_addr_input_limb_9, cond_felt_252_as_addr_input_limb_10, cond_felt_252_as_addr_input_limb_11, cond_felt_252_as_addr_input_limb_12, cond_felt_252_as_addr_input_limb_13, cond_felt_252_as_addr_input_limb_14, cond_felt_252_as_addr_input_limb_15, cond_felt_252_as_addr_input_limb_16, cond_felt_252_as_addr_input_limb_17, cond_felt_252_as_addr_input_limb_18, cond_felt_252_as_addr_input_limb_19, cond_felt_252_as_addr_input_limb_20, cond_felt_252_as_addr_input_limb_21, cond_felt_252_as_addr_input_limb_22, cond_felt_252_as_addr_input_limb_23, cond_felt_252_as_addr_input_limb_24, cond_felt_252_as_addr_input_limb_25, cond_felt_252_as_addr_input_limb_26, cond_felt_252_as_addr_input_limb_27, cond_felt_252_as_addr_input_limb_28]: [E::F; 29],
        eval: &mut E,
    ) -> [E::F; 1] {
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_512 = E::F::from(M31::from(512));

        // Address limb 3 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_3.clone()),
        );
        // Address limb 4 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_4.clone()),
        );
        // Address limb 5 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_5.clone()),
        );
        // Address limb 6 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_6.clone()),
        );
        // Address limb 7 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_7.clone()),
        );
        // Address limb 8 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_8.clone()),
        );
        // Address limb 9 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_9.clone()),
        );
        // Address limb 10 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_10.clone()),
        );
        // Address limb 11 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_11.clone()),
        );
        // Address limb 12 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_12.clone()),
        );
        // Address limb 13 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_13.clone()),
        );
        // Address limb 14 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_14.clone()),
        );
        // Address limb 15 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_15.clone()),
        );
        // Address limb 16 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_16.clone()),
        );
        // Address limb 17 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_17.clone()),
        );
        // Address limb 18 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_18.clone()),
        );
        // Address limb 19 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_19.clone()),
        );
        // Address limb 20 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_20.clone()),
        );
        // Address limb 21 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_21.clone()),
        );
        // Address limb 22 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_22.clone()),
        );
        // Address limb 23 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_23.clone()),
        );
        // Address limb 24 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_24.clone()),
        );
        // Address limb 25 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_25.clone()),
        );
        // Address limb 26 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_26.clone()),
        );
        // Address limb 27 equals 0.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * cond_felt_252_as_addr_input_limb_27.clone()),
        );
        [((cond_felt_252_as_addr_input_limb_0.clone()
            + (cond_felt_252_as_addr_input_limb_1.clone() * M31_512.clone()))
            + (cond_felt_252_as_addr_input_limb_2.clone() * M31_262144.clone()))]
    }
}
