// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::single_karatsuba_n_7::SingleKaratsubaN7;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct DoubleKaratsuba1454B {}

impl DoubleKaratsuba1454B {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [double_karatsuba_1454b_input_limb_0, double_karatsuba_1454b_input_limb_1, double_karatsuba_1454b_input_limb_2, double_karatsuba_1454b_input_limb_3, double_karatsuba_1454b_input_limb_4, double_karatsuba_1454b_input_limb_5, double_karatsuba_1454b_input_limb_6, double_karatsuba_1454b_input_limb_7, double_karatsuba_1454b_input_limb_8, double_karatsuba_1454b_input_limb_9, double_karatsuba_1454b_input_limb_10, double_karatsuba_1454b_input_limb_11, double_karatsuba_1454b_input_limb_12, double_karatsuba_1454b_input_limb_13, double_karatsuba_1454b_input_limb_14, double_karatsuba_1454b_input_limb_15, double_karatsuba_1454b_input_limb_16, double_karatsuba_1454b_input_limb_17, double_karatsuba_1454b_input_limb_18, double_karatsuba_1454b_input_limb_19, double_karatsuba_1454b_input_limb_20, double_karatsuba_1454b_input_limb_21, double_karatsuba_1454b_input_limb_22, double_karatsuba_1454b_input_limb_23, double_karatsuba_1454b_input_limb_24, double_karatsuba_1454b_input_limb_25, double_karatsuba_1454b_input_limb_26, double_karatsuba_1454b_input_limb_27, double_karatsuba_1454b_input_limb_28, double_karatsuba_1454b_input_limb_29, double_karatsuba_1454b_input_limb_30, double_karatsuba_1454b_input_limb_31, double_karatsuba_1454b_input_limb_32, double_karatsuba_1454b_input_limb_33, double_karatsuba_1454b_input_limb_34, double_karatsuba_1454b_input_limb_35, double_karatsuba_1454b_input_limb_36, double_karatsuba_1454b_input_limb_37, double_karatsuba_1454b_input_limb_38, double_karatsuba_1454b_input_limb_39, double_karatsuba_1454b_input_limb_40, double_karatsuba_1454b_input_limb_41, double_karatsuba_1454b_input_limb_42, double_karatsuba_1454b_input_limb_43, double_karatsuba_1454b_input_limb_44, double_karatsuba_1454b_input_limb_45, double_karatsuba_1454b_input_limb_46, double_karatsuba_1454b_input_limb_47, double_karatsuba_1454b_input_limb_48, double_karatsuba_1454b_input_limb_49, double_karatsuba_1454b_input_limb_50, double_karatsuba_1454b_input_limb_51, double_karatsuba_1454b_input_limb_52, double_karatsuba_1454b_input_limb_53, double_karatsuba_1454b_input_limb_54, double_karatsuba_1454b_input_limb_55]: [E::F; 56],
        eval: &mut E,
    ) -> [E::F; 55] {
        let [single_karatsuba_n_7_output_tmp_1454b_4_limb_0, single_karatsuba_n_7_output_tmp_1454b_4_limb_1, single_karatsuba_n_7_output_tmp_1454b_4_limb_2, single_karatsuba_n_7_output_tmp_1454b_4_limb_3, single_karatsuba_n_7_output_tmp_1454b_4_limb_4, single_karatsuba_n_7_output_tmp_1454b_4_limb_5, single_karatsuba_n_7_output_tmp_1454b_4_limb_6, single_karatsuba_n_7_output_tmp_1454b_4_limb_7, single_karatsuba_n_7_output_tmp_1454b_4_limb_8, single_karatsuba_n_7_output_tmp_1454b_4_limb_9, single_karatsuba_n_7_output_tmp_1454b_4_limb_10, single_karatsuba_n_7_output_tmp_1454b_4_limb_11, single_karatsuba_n_7_output_tmp_1454b_4_limb_12, single_karatsuba_n_7_output_tmp_1454b_4_limb_13, single_karatsuba_n_7_output_tmp_1454b_4_limb_14, single_karatsuba_n_7_output_tmp_1454b_4_limb_15, single_karatsuba_n_7_output_tmp_1454b_4_limb_16, single_karatsuba_n_7_output_tmp_1454b_4_limb_17, single_karatsuba_n_7_output_tmp_1454b_4_limb_18, single_karatsuba_n_7_output_tmp_1454b_4_limb_19, single_karatsuba_n_7_output_tmp_1454b_4_limb_20, single_karatsuba_n_7_output_tmp_1454b_4_limb_21, single_karatsuba_n_7_output_tmp_1454b_4_limb_22, single_karatsuba_n_7_output_tmp_1454b_4_limb_23, single_karatsuba_n_7_output_tmp_1454b_4_limb_24, single_karatsuba_n_7_output_tmp_1454b_4_limb_25, single_karatsuba_n_7_output_tmp_1454b_4_limb_26] =
            SingleKaratsubaN7::evaluate(
                [
                    double_karatsuba_1454b_input_limb_0.clone(),
                    double_karatsuba_1454b_input_limb_1.clone(),
                    double_karatsuba_1454b_input_limb_2.clone(),
                    double_karatsuba_1454b_input_limb_3.clone(),
                    double_karatsuba_1454b_input_limb_4.clone(),
                    double_karatsuba_1454b_input_limb_5.clone(),
                    double_karatsuba_1454b_input_limb_6.clone(),
                    double_karatsuba_1454b_input_limb_7.clone(),
                    double_karatsuba_1454b_input_limb_8.clone(),
                    double_karatsuba_1454b_input_limb_9.clone(),
                    double_karatsuba_1454b_input_limb_10.clone(),
                    double_karatsuba_1454b_input_limb_11.clone(),
                    double_karatsuba_1454b_input_limb_12.clone(),
                    double_karatsuba_1454b_input_limb_13.clone(),
                    double_karatsuba_1454b_input_limb_28.clone(),
                    double_karatsuba_1454b_input_limb_29.clone(),
                    double_karatsuba_1454b_input_limb_30.clone(),
                    double_karatsuba_1454b_input_limb_31.clone(),
                    double_karatsuba_1454b_input_limb_32.clone(),
                    double_karatsuba_1454b_input_limb_33.clone(),
                    double_karatsuba_1454b_input_limb_34.clone(),
                    double_karatsuba_1454b_input_limb_35.clone(),
                    double_karatsuba_1454b_input_limb_36.clone(),
                    double_karatsuba_1454b_input_limb_37.clone(),
                    double_karatsuba_1454b_input_limb_38.clone(),
                    double_karatsuba_1454b_input_limb_39.clone(),
                    double_karatsuba_1454b_input_limb_40.clone(),
                    double_karatsuba_1454b_input_limb_41.clone(),
                ],
                eval,
            );
        let [single_karatsuba_n_7_output_tmp_1454b_9_limb_0, single_karatsuba_n_7_output_tmp_1454b_9_limb_1, single_karatsuba_n_7_output_tmp_1454b_9_limb_2, single_karatsuba_n_7_output_tmp_1454b_9_limb_3, single_karatsuba_n_7_output_tmp_1454b_9_limb_4, single_karatsuba_n_7_output_tmp_1454b_9_limb_5, single_karatsuba_n_7_output_tmp_1454b_9_limb_6, single_karatsuba_n_7_output_tmp_1454b_9_limb_7, single_karatsuba_n_7_output_tmp_1454b_9_limb_8, single_karatsuba_n_7_output_tmp_1454b_9_limb_9, single_karatsuba_n_7_output_tmp_1454b_9_limb_10, single_karatsuba_n_7_output_tmp_1454b_9_limb_11, single_karatsuba_n_7_output_tmp_1454b_9_limb_12, single_karatsuba_n_7_output_tmp_1454b_9_limb_13, single_karatsuba_n_7_output_tmp_1454b_9_limb_14, single_karatsuba_n_7_output_tmp_1454b_9_limb_15, single_karatsuba_n_7_output_tmp_1454b_9_limb_16, single_karatsuba_n_7_output_tmp_1454b_9_limb_17, single_karatsuba_n_7_output_tmp_1454b_9_limb_18, single_karatsuba_n_7_output_tmp_1454b_9_limb_19, single_karatsuba_n_7_output_tmp_1454b_9_limb_20, single_karatsuba_n_7_output_tmp_1454b_9_limb_21, single_karatsuba_n_7_output_tmp_1454b_9_limb_22, single_karatsuba_n_7_output_tmp_1454b_9_limb_23, single_karatsuba_n_7_output_tmp_1454b_9_limb_24, single_karatsuba_n_7_output_tmp_1454b_9_limb_25, single_karatsuba_n_7_output_tmp_1454b_9_limb_26] =
            SingleKaratsubaN7::evaluate(
                [
                    double_karatsuba_1454b_input_limb_14.clone(),
                    double_karatsuba_1454b_input_limb_15.clone(),
                    double_karatsuba_1454b_input_limb_16.clone(),
                    double_karatsuba_1454b_input_limb_17.clone(),
                    double_karatsuba_1454b_input_limb_18.clone(),
                    double_karatsuba_1454b_input_limb_19.clone(),
                    double_karatsuba_1454b_input_limb_20.clone(),
                    double_karatsuba_1454b_input_limb_21.clone(),
                    double_karatsuba_1454b_input_limb_22.clone(),
                    double_karatsuba_1454b_input_limb_23.clone(),
                    double_karatsuba_1454b_input_limb_24.clone(),
                    double_karatsuba_1454b_input_limb_25.clone(),
                    double_karatsuba_1454b_input_limb_26.clone(),
                    double_karatsuba_1454b_input_limb_27.clone(),
                    double_karatsuba_1454b_input_limb_42.clone(),
                    double_karatsuba_1454b_input_limb_43.clone(),
                    double_karatsuba_1454b_input_limb_44.clone(),
                    double_karatsuba_1454b_input_limb_45.clone(),
                    double_karatsuba_1454b_input_limb_46.clone(),
                    double_karatsuba_1454b_input_limb_47.clone(),
                    double_karatsuba_1454b_input_limb_48.clone(),
                    double_karatsuba_1454b_input_limb_49.clone(),
                    double_karatsuba_1454b_input_limb_50.clone(),
                    double_karatsuba_1454b_input_limb_51.clone(),
                    double_karatsuba_1454b_input_limb_52.clone(),
                    double_karatsuba_1454b_input_limb_53.clone(),
                    double_karatsuba_1454b_input_limb_54.clone(),
                    double_karatsuba_1454b_input_limb_55.clone(),
                ],
                eval,
            );
        let x_sum_tmp_1454b_10_limb_0 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_0.clone()
                + double_karatsuba_1454b_input_limb_14.clone()),
        );
        let x_sum_tmp_1454b_10_limb_1 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_1.clone()
                + double_karatsuba_1454b_input_limb_15.clone()),
        );
        let x_sum_tmp_1454b_10_limb_2 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_2.clone()
                + double_karatsuba_1454b_input_limb_16.clone()),
        );
        let x_sum_tmp_1454b_10_limb_3 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_3.clone()
                + double_karatsuba_1454b_input_limb_17.clone()),
        );
        let x_sum_tmp_1454b_10_limb_4 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_4.clone()
                + double_karatsuba_1454b_input_limb_18.clone()),
        );
        let x_sum_tmp_1454b_10_limb_5 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_5.clone()
                + double_karatsuba_1454b_input_limb_19.clone()),
        );
        let x_sum_tmp_1454b_10_limb_6 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_6.clone()
                + double_karatsuba_1454b_input_limb_20.clone()),
        );
        let x_sum_tmp_1454b_10_limb_7 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_7.clone()
                + double_karatsuba_1454b_input_limb_21.clone()),
        );
        let x_sum_tmp_1454b_10_limb_8 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_8.clone()
                + double_karatsuba_1454b_input_limb_22.clone()),
        );
        let x_sum_tmp_1454b_10_limb_9 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_9.clone()
                + double_karatsuba_1454b_input_limb_23.clone()),
        );
        let x_sum_tmp_1454b_10_limb_10 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_10.clone()
                + double_karatsuba_1454b_input_limb_24.clone()),
        );
        let x_sum_tmp_1454b_10_limb_11 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_11.clone()
                + double_karatsuba_1454b_input_limb_25.clone()),
        );
        let x_sum_tmp_1454b_10_limb_12 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_12.clone()
                + double_karatsuba_1454b_input_limb_26.clone()),
        );
        let x_sum_tmp_1454b_10_limb_13 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_13.clone()
                + double_karatsuba_1454b_input_limb_27.clone()),
        );
        let y_sum_tmp_1454b_11_limb_0 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_28.clone()
                + double_karatsuba_1454b_input_limb_42.clone()),
        );
        let y_sum_tmp_1454b_11_limb_1 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_29.clone()
                + double_karatsuba_1454b_input_limb_43.clone()),
        );
        let y_sum_tmp_1454b_11_limb_2 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_30.clone()
                + double_karatsuba_1454b_input_limb_44.clone()),
        );
        let y_sum_tmp_1454b_11_limb_3 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_31.clone()
                + double_karatsuba_1454b_input_limb_45.clone()),
        );
        let y_sum_tmp_1454b_11_limb_4 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_32.clone()
                + double_karatsuba_1454b_input_limb_46.clone()),
        );
        let y_sum_tmp_1454b_11_limb_5 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_33.clone()
                + double_karatsuba_1454b_input_limb_47.clone()),
        );
        let y_sum_tmp_1454b_11_limb_6 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_34.clone()
                + double_karatsuba_1454b_input_limb_48.clone()),
        );
        let y_sum_tmp_1454b_11_limb_7 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_35.clone()
                + double_karatsuba_1454b_input_limb_49.clone()),
        );
        let y_sum_tmp_1454b_11_limb_8 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_36.clone()
                + double_karatsuba_1454b_input_limb_50.clone()),
        );
        let y_sum_tmp_1454b_11_limb_9 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_37.clone()
                + double_karatsuba_1454b_input_limb_51.clone()),
        );
        let y_sum_tmp_1454b_11_limb_10 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_38.clone()
                + double_karatsuba_1454b_input_limb_52.clone()),
        );
        let y_sum_tmp_1454b_11_limb_11 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_39.clone()
                + double_karatsuba_1454b_input_limb_53.clone()),
        );
        let y_sum_tmp_1454b_11_limb_12 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_40.clone()
                + double_karatsuba_1454b_input_limb_54.clone()),
        );
        let y_sum_tmp_1454b_11_limb_13 = eval.add_intermediate(
            (double_karatsuba_1454b_input_limb_41.clone()
                + double_karatsuba_1454b_input_limb_55.clone()),
        );
        let [single_karatsuba_n_7_output_tmp_1454b_16_limb_0, single_karatsuba_n_7_output_tmp_1454b_16_limb_1, single_karatsuba_n_7_output_tmp_1454b_16_limb_2, single_karatsuba_n_7_output_tmp_1454b_16_limb_3, single_karatsuba_n_7_output_tmp_1454b_16_limb_4, single_karatsuba_n_7_output_tmp_1454b_16_limb_5, single_karatsuba_n_7_output_tmp_1454b_16_limb_6, single_karatsuba_n_7_output_tmp_1454b_16_limb_7, single_karatsuba_n_7_output_tmp_1454b_16_limb_8, single_karatsuba_n_7_output_tmp_1454b_16_limb_9, single_karatsuba_n_7_output_tmp_1454b_16_limb_10, single_karatsuba_n_7_output_tmp_1454b_16_limb_11, single_karatsuba_n_7_output_tmp_1454b_16_limb_12, single_karatsuba_n_7_output_tmp_1454b_16_limb_13, single_karatsuba_n_7_output_tmp_1454b_16_limb_14, single_karatsuba_n_7_output_tmp_1454b_16_limb_15, single_karatsuba_n_7_output_tmp_1454b_16_limb_16, single_karatsuba_n_7_output_tmp_1454b_16_limb_17, single_karatsuba_n_7_output_tmp_1454b_16_limb_18, single_karatsuba_n_7_output_tmp_1454b_16_limb_19, single_karatsuba_n_7_output_tmp_1454b_16_limb_20, single_karatsuba_n_7_output_tmp_1454b_16_limb_21, single_karatsuba_n_7_output_tmp_1454b_16_limb_22, single_karatsuba_n_7_output_tmp_1454b_16_limb_23, single_karatsuba_n_7_output_tmp_1454b_16_limb_24, single_karatsuba_n_7_output_tmp_1454b_16_limb_25, single_karatsuba_n_7_output_tmp_1454b_16_limb_26] =
            SingleKaratsubaN7::evaluate(
                [
                    x_sum_tmp_1454b_10_limb_0.clone(),
                    x_sum_tmp_1454b_10_limb_1.clone(),
                    x_sum_tmp_1454b_10_limb_2.clone(),
                    x_sum_tmp_1454b_10_limb_3.clone(),
                    x_sum_tmp_1454b_10_limb_4.clone(),
                    x_sum_tmp_1454b_10_limb_5.clone(),
                    x_sum_tmp_1454b_10_limb_6.clone(),
                    x_sum_tmp_1454b_10_limb_7.clone(),
                    x_sum_tmp_1454b_10_limb_8.clone(),
                    x_sum_tmp_1454b_10_limb_9.clone(),
                    x_sum_tmp_1454b_10_limb_10.clone(),
                    x_sum_tmp_1454b_10_limb_11.clone(),
                    x_sum_tmp_1454b_10_limb_12.clone(),
                    x_sum_tmp_1454b_10_limb_13.clone(),
                    y_sum_tmp_1454b_11_limb_0.clone(),
                    y_sum_tmp_1454b_11_limb_1.clone(),
                    y_sum_tmp_1454b_11_limb_2.clone(),
                    y_sum_tmp_1454b_11_limb_3.clone(),
                    y_sum_tmp_1454b_11_limb_4.clone(),
                    y_sum_tmp_1454b_11_limb_5.clone(),
                    y_sum_tmp_1454b_11_limb_6.clone(),
                    y_sum_tmp_1454b_11_limb_7.clone(),
                    y_sum_tmp_1454b_11_limb_8.clone(),
                    y_sum_tmp_1454b_11_limb_9.clone(),
                    y_sum_tmp_1454b_11_limb_10.clone(),
                    y_sum_tmp_1454b_11_limb_11.clone(),
                    y_sum_tmp_1454b_11_limb_12.clone(),
                    y_sum_tmp_1454b_11_limb_13.clone(),
                ],
                eval,
            );
        [
            single_karatsuba_n_7_output_tmp_1454b_4_limb_0.clone(),
            single_karatsuba_n_7_output_tmp_1454b_4_limb_1.clone(),
            single_karatsuba_n_7_output_tmp_1454b_4_limb_2.clone(),
            single_karatsuba_n_7_output_tmp_1454b_4_limb_3.clone(),
            single_karatsuba_n_7_output_tmp_1454b_4_limb_4.clone(),
            single_karatsuba_n_7_output_tmp_1454b_4_limb_5.clone(),
            single_karatsuba_n_7_output_tmp_1454b_4_limb_6.clone(),
            single_karatsuba_n_7_output_tmp_1454b_4_limb_7.clone(),
            single_karatsuba_n_7_output_tmp_1454b_4_limb_8.clone(),
            single_karatsuba_n_7_output_tmp_1454b_4_limb_9.clone(),
            single_karatsuba_n_7_output_tmp_1454b_4_limb_10.clone(),
            single_karatsuba_n_7_output_tmp_1454b_4_limb_11.clone(),
            single_karatsuba_n_7_output_tmp_1454b_4_limb_12.clone(),
            single_karatsuba_n_7_output_tmp_1454b_4_limb_13.clone(),
            (single_karatsuba_n_7_output_tmp_1454b_4_limb_14.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_0.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_0.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_0.clone())),
            (single_karatsuba_n_7_output_tmp_1454b_4_limb_15.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_1.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_1.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_1.clone())),
            (single_karatsuba_n_7_output_tmp_1454b_4_limb_16.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_2.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_2.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_2.clone())),
            (single_karatsuba_n_7_output_tmp_1454b_4_limb_17.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_3.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_3.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_3.clone())),
            (single_karatsuba_n_7_output_tmp_1454b_4_limb_18.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_4.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_4.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_4.clone())),
            (single_karatsuba_n_7_output_tmp_1454b_4_limb_19.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_5.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_5.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_5.clone())),
            (single_karatsuba_n_7_output_tmp_1454b_4_limb_20.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_6.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_6.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_6.clone())),
            (single_karatsuba_n_7_output_tmp_1454b_4_limb_21.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_7.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_7.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_7.clone())),
            (single_karatsuba_n_7_output_tmp_1454b_4_limb_22.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_8.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_8.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_8.clone())),
            (single_karatsuba_n_7_output_tmp_1454b_4_limb_23.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_9.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_9.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_9.clone())),
            (single_karatsuba_n_7_output_tmp_1454b_4_limb_24.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_10.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_10.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_10.clone())),
            (single_karatsuba_n_7_output_tmp_1454b_4_limb_25.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_11.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_11.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_11.clone())),
            (single_karatsuba_n_7_output_tmp_1454b_4_limb_26.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_12.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_12.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_12.clone())),
            ((single_karatsuba_n_7_output_tmp_1454b_16_limb_13.clone()
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_13.clone())
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_13.clone()),
            (single_karatsuba_n_7_output_tmp_1454b_9_limb_0.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_14.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_14.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_14.clone())),
            (single_karatsuba_n_7_output_tmp_1454b_9_limb_1.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_15.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_15.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_15.clone())),
            (single_karatsuba_n_7_output_tmp_1454b_9_limb_2.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_16.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_16.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_16.clone())),
            (single_karatsuba_n_7_output_tmp_1454b_9_limb_3.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_17.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_17.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_17.clone())),
            (single_karatsuba_n_7_output_tmp_1454b_9_limb_4.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_18.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_18.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_18.clone())),
            (single_karatsuba_n_7_output_tmp_1454b_9_limb_5.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_19.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_19.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_19.clone())),
            (single_karatsuba_n_7_output_tmp_1454b_9_limb_6.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_20.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_20.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_20.clone())),
            (single_karatsuba_n_7_output_tmp_1454b_9_limb_7.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_21.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_21.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_21.clone())),
            (single_karatsuba_n_7_output_tmp_1454b_9_limb_8.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_22.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_22.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_22.clone())),
            (single_karatsuba_n_7_output_tmp_1454b_9_limb_9.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_23.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_23.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_23.clone())),
            (single_karatsuba_n_7_output_tmp_1454b_9_limb_10.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_24.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_24.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_24.clone())),
            (single_karatsuba_n_7_output_tmp_1454b_9_limb_11.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_25.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_25.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_25.clone())),
            (single_karatsuba_n_7_output_tmp_1454b_9_limb_12.clone()
                + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_26.clone()
                    - single_karatsuba_n_7_output_tmp_1454b_4_limb_26.clone())
                    - single_karatsuba_n_7_output_tmp_1454b_9_limb_26.clone())),
            single_karatsuba_n_7_output_tmp_1454b_9_limb_13.clone(),
            single_karatsuba_n_7_output_tmp_1454b_9_limb_14.clone(),
            single_karatsuba_n_7_output_tmp_1454b_9_limb_15.clone(),
            single_karatsuba_n_7_output_tmp_1454b_9_limb_16.clone(),
            single_karatsuba_n_7_output_tmp_1454b_9_limb_17.clone(),
            single_karatsuba_n_7_output_tmp_1454b_9_limb_18.clone(),
            single_karatsuba_n_7_output_tmp_1454b_9_limb_19.clone(),
            single_karatsuba_n_7_output_tmp_1454b_9_limb_20.clone(),
            single_karatsuba_n_7_output_tmp_1454b_9_limb_21.clone(),
            single_karatsuba_n_7_output_tmp_1454b_9_limb_22.clone(),
            single_karatsuba_n_7_output_tmp_1454b_9_limb_23.clone(),
            single_karatsuba_n_7_output_tmp_1454b_9_limb_24.clone(),
            single_karatsuba_n_7_output_tmp_1454b_9_limb_25.clone(),
            single_karatsuba_n_7_output_tmp_1454b_9_limb_26.clone(),
        ]
    }
}
