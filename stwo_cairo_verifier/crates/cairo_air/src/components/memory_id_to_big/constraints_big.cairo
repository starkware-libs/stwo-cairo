use crate::components::subroutines::range_check_mem_value_n_28::range_check_mem_value_n_28_evaluate;
use crate::prelude::*;

#[derive(Drop)]
pub struct ConstraintParams {
    pub common_lookup_elements: CommonLookupElements,
    pub claimed_sum: QM31,
    pub seq: QM31,
    pub column_size: M31,
    pub offset: M31,
}

pub fn evaluate_constraints_at_point(
    ref sum: QM31,
    ref trace_mask_values: ColumnSpan<Span<QM31>>,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    params: ConstraintParams,
    random_coeff: QM31,
) {
    let ConstraintParams { common_lookup_elements, claimed_sum, seq, column_size, offset } = params;
    let mut range_check_9_9_sum_0: QM31 = Zero::zero();
    let mut range_check_9_9_b_sum_1: QM31 = Zero::zero();
    let mut range_check_9_9_c_sum_2: QM31 = Zero::zero();
    let mut range_check_9_9_d_sum_3: QM31 = Zero::zero();
    let mut range_check_9_9_e_sum_4: QM31 = Zero::zero();
    let mut range_check_9_9_f_sum_5: QM31 = Zero::zero();
    let mut range_check_9_9_g_sum_6: QM31 = Zero::zero();
    let mut range_check_9_9_h_sum_7: QM31 = Zero::zero();
    let mut range_check_9_9_sum_8: QM31 = Zero::zero();
    let mut range_check_9_9_b_sum_9: QM31 = Zero::zero();
    let mut range_check_9_9_c_sum_10: QM31 = Zero::zero();
    let mut range_check_9_9_d_sum_11: QM31 = Zero::zero();
    let mut range_check_9_9_e_sum_12: QM31 = Zero::zero();
    let mut range_check_9_9_f_sum_13: QM31 = Zero::zero();
    let mut memory_id_to_big_sum_14: QM31 = Zero::zero();

    let [
        memory_id_to_big_output_col0,
        memory_id_to_big_output_col1,
        memory_id_to_big_output_col2,
        memory_id_to_big_output_col3,
        memory_id_to_big_output_col4,
        memory_id_to_big_output_col5,
        memory_id_to_big_output_col6,
        memory_id_to_big_output_col7,
        memory_id_to_big_output_col8,
        memory_id_to_big_output_col9,
        memory_id_to_big_output_col10,
        memory_id_to_big_output_col11,
        memory_id_to_big_output_col12,
        memory_id_to_big_output_col13,
        memory_id_to_big_output_col14,
        memory_id_to_big_output_col15,
        memory_id_to_big_output_col16,
        memory_id_to_big_output_col17,
        memory_id_to_big_output_col18,
        memory_id_to_big_output_col19,
        memory_id_to_big_output_col20,
        memory_id_to_big_output_col21,
        memory_id_to_big_output_col22,
        memory_id_to_big_output_col23,
        memory_id_to_big_output_col24,
        memory_id_to_big_output_col25,
        memory_id_to_big_output_col26,
        memory_id_to_big_output_col27,
        enabler,
    ]: [Span<QM31>; 29] =
        (*trace_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col0]: [QM31; 1] = (*memory_id_to_big_output_col0
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col1]: [QM31; 1] = (*memory_id_to_big_output_col1
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col2]: [QM31; 1] = (*memory_id_to_big_output_col2
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col3]: [QM31; 1] = (*memory_id_to_big_output_col3
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col4]: [QM31; 1] = (*memory_id_to_big_output_col4
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col5]: [QM31; 1] = (*memory_id_to_big_output_col5
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col6]: [QM31; 1] = (*memory_id_to_big_output_col6
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col7]: [QM31; 1] = (*memory_id_to_big_output_col7
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col8]: [QM31; 1] = (*memory_id_to_big_output_col8
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col9]: [QM31; 1] = (*memory_id_to_big_output_col9
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col10]: [QM31; 1] = (*memory_id_to_big_output_col10
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col11]: [QM31; 1] = (*memory_id_to_big_output_col11
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col12]: [QM31; 1] = (*memory_id_to_big_output_col12
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col13]: [QM31; 1] = (*memory_id_to_big_output_col13
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col14]: [QM31; 1] = (*memory_id_to_big_output_col14
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col15]: [QM31; 1] = (*memory_id_to_big_output_col15
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col16]: [QM31; 1] = (*memory_id_to_big_output_col16
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col17]: [QM31; 1] = (*memory_id_to_big_output_col17
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col18]: [QM31; 1] = (*memory_id_to_big_output_col18
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col19]: [QM31; 1] = (*memory_id_to_big_output_col19
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col20]: [QM31; 1] = (*memory_id_to_big_output_col20
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col21]: [QM31; 1] = (*memory_id_to_big_output_col21
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col22]: [QM31; 1] = (*memory_id_to_big_output_col22
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col23]: [QM31; 1] = (*memory_id_to_big_output_col23
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col24]: [QM31; 1] = (*memory_id_to_big_output_col24
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col25]: [QM31; 1] = (*memory_id_to_big_output_col25
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col26]: [QM31; 1] = (*memory_id_to_big_output_col26
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_big_output_col27]: [QM31; 1] = (*memory_id_to_big_output_col27
        .try_into()
        .unwrap())
        .unbox();
    let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

    core::internal::revoke_ap_tracking();

    range_check_mem_value_n_28_evaluate(
        [
            memory_id_to_big_output_col0, memory_id_to_big_output_col1,
            memory_id_to_big_output_col2, memory_id_to_big_output_col3,
            memory_id_to_big_output_col4, memory_id_to_big_output_col5,
            memory_id_to_big_output_col6, memory_id_to_big_output_col7,
            memory_id_to_big_output_col8, memory_id_to_big_output_col9,
            memory_id_to_big_output_col10, memory_id_to_big_output_col11,
            memory_id_to_big_output_col12, memory_id_to_big_output_col13,
            memory_id_to_big_output_col14, memory_id_to_big_output_col15,
            memory_id_to_big_output_col16, memory_id_to_big_output_col17,
            memory_id_to_big_output_col18, memory_id_to_big_output_col19,
            memory_id_to_big_output_col20, memory_id_to_big_output_col21,
            memory_id_to_big_output_col22, memory_id_to_big_output_col23,
            memory_id_to_big_output_col24, memory_id_to_big_output_col25,
            memory_id_to_big_output_col26, memory_id_to_big_output_col27,
        ],
        @common_lookup_elements,
        ref range_check_9_9_sum_0,
        ref range_check_9_9_b_sum_1,
        ref range_check_9_9_c_sum_2,
        ref range_check_9_9_d_sum_3,
        ref range_check_9_9_e_sum_4,
        ref range_check_9_9_f_sum_5,
        ref range_check_9_9_g_sum_6,
        ref range_check_9_9_h_sum_7,
        ref range_check_9_9_sum_8,
        ref range_check_9_9_b_sum_9,
        ref range_check_9_9_c_sum_10,
        ref range_check_9_9_d_sum_11,
        ref range_check_9_9_e_sum_12,
        ref range_check_9_9_f_sum_13,
        ref sum,
        random_coeff,
    );

    memory_id_to_big_sum_14 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1662111297, 0, 0, 0>(), seq + offset.into(),
                memory_id_to_big_output_col0, memory_id_to_big_output_col1,
                memory_id_to_big_output_col2, memory_id_to_big_output_col3,
                memory_id_to_big_output_col4, memory_id_to_big_output_col5,
                memory_id_to_big_output_col6, memory_id_to_big_output_col7,
                memory_id_to_big_output_col8, memory_id_to_big_output_col9,
                memory_id_to_big_output_col10, memory_id_to_big_output_col11,
                memory_id_to_big_output_col12, memory_id_to_big_output_col13,
                memory_id_to_big_output_col14, memory_id_to_big_output_col15,
                memory_id_to_big_output_col16, memory_id_to_big_output_col17,
                memory_id_to_big_output_col18, memory_id_to_big_output_col19,
                memory_id_to_big_output_col20, memory_id_to_big_output_col21,
                memory_id_to_big_output_col22, memory_id_to_big_output_col23,
                memory_id_to_big_output_col24, memory_id_to_big_output_col25,
                memory_id_to_big_output_col26, memory_id_to_big_output_col27,
            ]
                .span(),
        );

    lookup_constraints(
        ref sum,
        random_coeff,
        claimed_sum,
        enabler,
        column_size,
        ref interaction_trace_mask_values,
        range_check_9_9_sum_0,
        range_check_9_9_b_sum_1,
        range_check_9_9_c_sum_2,
        range_check_9_9_d_sum_3,
        range_check_9_9_e_sum_4,
        range_check_9_9_f_sum_5,
        range_check_9_9_g_sum_6,
        range_check_9_9_h_sum_7,
        range_check_9_9_sum_8,
        range_check_9_9_b_sum_9,
        range_check_9_9_c_sum_10,
        range_check_9_9_d_sum_11,
        range_check_9_9_e_sum_12,
        range_check_9_9_f_sum_13,
        memory_id_to_big_sum_14,
    );
}


fn lookup_constraints(
    ref sum: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    enabler: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    range_check_9_9_sum_0: QM31,
    range_check_9_9_b_sum_1: QM31,
    range_check_9_9_c_sum_2: QM31,
    range_check_9_9_d_sum_3: QM31,
    range_check_9_9_e_sum_4: QM31,
    range_check_9_9_f_sum_5: QM31,
    range_check_9_9_g_sum_6: QM31,
    range_check_9_9_h_sum_7: QM31,
    range_check_9_9_sum_8: QM31,
    range_check_9_9_b_sum_9: QM31,
    range_check_9_9_c_sum_10: QM31,
    range_check_9_9_d_sum_11: QM31,
    range_check_9_9_e_sum_12: QM31,
    range_check_9_9_f_sum_13: QM31,
    memory_id_to_big_sum_14: QM31,
) {
    let [
        trace_2_col0,
        trace_2_col1,
        trace_2_col2,
        trace_2_col3,
        trace_2_col4,
        trace_2_col5,
        trace_2_col6,
        trace_2_col7,
        trace_2_col8,
        trace_2_col9,
        trace_2_col10,
        trace_2_col11,
        trace_2_col12,
        trace_2_col13,
        trace_2_col14,
        trace_2_col15,
        trace_2_col16,
        trace_2_col17,
        trace_2_col18,
        trace_2_col19,
        trace_2_col20,
        trace_2_col21,
        trace_2_col22,
        trace_2_col23,
        trace_2_col24,
        trace_2_col25,
        trace_2_col26,
        trace_2_col27,
        trace_2_col28,
        trace_2_col29,
        trace_2_col30,
        trace_2_col31,
    ]: [Span<QM31>; 32] =
        (*interaction_trace_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    let [trace_2_col0]: [QM31; 1] = (*trace_2_col0.try_into().unwrap()).unbox();
    let [trace_2_col1]: [QM31; 1] = (*trace_2_col1.try_into().unwrap()).unbox();
    let [trace_2_col2]: [QM31; 1] = (*trace_2_col2.try_into().unwrap()).unbox();
    let [trace_2_col3]: [QM31; 1] = (*trace_2_col3.try_into().unwrap()).unbox();
    let [trace_2_col4]: [QM31; 1] = (*trace_2_col4.try_into().unwrap()).unbox();
    let [trace_2_col5]: [QM31; 1] = (*trace_2_col5.try_into().unwrap()).unbox();
    let [trace_2_col6]: [QM31; 1] = (*trace_2_col6.try_into().unwrap()).unbox();
    let [trace_2_col7]: [QM31; 1] = (*trace_2_col7.try_into().unwrap()).unbox();
    let [trace_2_col8]: [QM31; 1] = (*trace_2_col8.try_into().unwrap()).unbox();
    let [trace_2_col9]: [QM31; 1] = (*trace_2_col9.try_into().unwrap()).unbox();
    let [trace_2_col10]: [QM31; 1] = (*trace_2_col10.try_into().unwrap()).unbox();
    let [trace_2_col11]: [QM31; 1] = (*trace_2_col11.try_into().unwrap()).unbox();
    let [trace_2_col12]: [QM31; 1] = (*trace_2_col12.try_into().unwrap()).unbox();
    let [trace_2_col13]: [QM31; 1] = (*trace_2_col13.try_into().unwrap()).unbox();
    let [trace_2_col14]: [QM31; 1] = (*trace_2_col14.try_into().unwrap()).unbox();
    let [trace_2_col15]: [QM31; 1] = (*trace_2_col15.try_into().unwrap()).unbox();
    let [trace_2_col16]: [QM31; 1] = (*trace_2_col16.try_into().unwrap()).unbox();
    let [trace_2_col17]: [QM31; 1] = (*trace_2_col17.try_into().unwrap()).unbox();
    let [trace_2_col18]: [QM31; 1] = (*trace_2_col18.try_into().unwrap()).unbox();
    let [trace_2_col19]: [QM31; 1] = (*trace_2_col19.try_into().unwrap()).unbox();
    let [trace_2_col20]: [QM31; 1] = (*trace_2_col20.try_into().unwrap()).unbox();
    let [trace_2_col21]: [QM31; 1] = (*trace_2_col21.try_into().unwrap()).unbox();
    let [trace_2_col22]: [QM31; 1] = (*trace_2_col22.try_into().unwrap()).unbox();
    let [trace_2_col23]: [QM31; 1] = (*trace_2_col23.try_into().unwrap()).unbox();
    let [trace_2_col24]: [QM31; 1] = (*trace_2_col24.try_into().unwrap()).unbox();
    let [trace_2_col25]: [QM31; 1] = (*trace_2_col25.try_into().unwrap()).unbox();
    let [trace_2_col26]: [QM31; 1] = (*trace_2_col26.try_into().unwrap()).unbox();
    let [trace_2_col27]: [QM31; 1] = (*trace_2_col27.try_into().unwrap()).unbox();
    let [trace_2_col28_neg1, trace_2_col28]: [QM31; 2] = (*trace_2_col28.try_into().unwrap())
        .unbox();
    let [trace_2_col29_neg1, trace_2_col29]: [QM31; 2] = (*trace_2_col29.try_into().unwrap())
        .unbox();
    let [trace_2_col30_neg1, trace_2_col30]: [QM31; 2] = (*trace_2_col30.try_into().unwrap())
        .unbox();
    let [trace_2_col31_neg1, trace_2_col31]: [QM31; 2] = (*trace_2_col31.try_into().unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * range_check_9_9_sum_0
        * range_check_9_9_b_sum_1)
        - range_check_9_9_sum_0
        - range_check_9_9_b_sum_1);
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * range_check_9_9_c_sum_2
        * range_check_9_9_d_sum_3)
        - range_check_9_9_c_sum_2
        - range_check_9_9_d_sum_3);
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]))
        * range_check_9_9_e_sum_4
        * range_check_9_9_f_sum_5)
        - range_check_9_9_e_sum_4
        - range_check_9_9_f_sum_5);
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]))
        * range_check_9_9_g_sum_6
        * range_check_9_9_h_sum_7)
        - range_check_9_9_g_sum_6
        - range_check_9_9_h_sum_7);
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
        ))
        * range_check_9_9_sum_8
        * range_check_9_9_b_sum_9)
        - range_check_9_9_sum_8
        - range_check_9_9_b_sum_9);
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
        ))
        * range_check_9_9_c_sum_10
        * range_check_9_9_d_sum_11)
        - range_check_9_9_c_sum_10
        - range_check_9_9_d_sum_11);
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
        ))
        * range_check_9_9_e_sum_12
        * range_check_9_9_f_sum_13)
        - range_check_9_9_e_sum_12
        - range_check_9_9_f_sum_13);
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
    )
        - QM31Impl::from_partial_evals([trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27])
        - QM31Impl::from_partial_evals(
            [trace_2_col28_neg1, trace_2_col29_neg1, trace_2_col30_neg1, trace_2_col31_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * memory_id_to_big_sum_14)
        + enabler);
    sum = sum * random_coeff + constraint_quotient;
}
#[cfg(and(test, feature: "qm31_opcode"))]
mod tests {
    use core::num::traits::Zero;
    use stwo_constraint_framework::LookupElementsTrait;
    use stwo_verifier_core::fields::m31::{M31, m31};
    use stwo_verifier_core::fields::qm31::{QM31, QM31Trait, qm31_const};
    use stwo_verifier_core::poly::circle::CanonicCosetImpl;
    use stwo_verifier_core::utils::pow2;
    use crate::components::sample_evaluations::*;
    use crate::test_utils::make_interaction_trace;
    use super::{ConstraintParams, evaluate_constraints_at_point};

    #[test]
    fn test_evaluation_result_offset_zero() {
        test_evaluation_result_with_offset(
            m31(0), QM31Trait::from_fixed_array(MEMORY_ID_TO_BIG_SAMPLE_EVAL_RESULT),
        );
    }

    #[test]
    fn test_evaluation_result_offset_nonzero() {
        test_evaluation_result_with_offset(
            m31(pow2(16)), qm31_const::<381535455, 1343882909, 2107410580, 1315217794>(),
        );
    }

    fn test_evaluation_result_with_offset(offset: M31, expected_result: QM31) {
        let log_size = 15;

        let common_lookup_elements = LookupElementsTrait::from_z_alpha(
            qm31_const::<445623802, 202571636, 1360224996, 131355117>(),
            qm31_const::<476823935, 939223384, 62486082, 122423602>(),
        );
        let mut sum: QM31 = Zero::zero();
        let mut trace_columns = [
            [qm31_const::<1659099300, 905558730, 651199673, 1375009625>()].span(),
            [qm31_const::<1591990121, 771341002, 584090809, 1375009625>()].span(),
            [qm31_const::<1793317658, 1173994186, 785417401, 1375009625>()].span(),
            [qm31_const::<1726208479, 1039776458, 718308537, 1375009625>()].span(),
            [qm31_const::<1390662584, 368687818, 382764217, 1375009625>()].span(),
            [qm31_const::<1323553405, 234470090, 315655353, 1375009625>()].span(),
            [qm31_const::<1524880942, 637123274, 516981945, 1375009625>()].span(),
            [qm31_const::<1457771763, 502905546, 449873081, 1375009625>()].span(),
            [qm31_const::<48489085, 1979300555, 1188070585, 1375009625>()].span(),
            [qm31_const::<2128863553, 1845082826, 1120961721, 1375009625>()].span(),
            [qm31_const::<1852335767, 645078115, 2059236183, 343880121>()].span(),
            [qm31_const::<1919444946, 779295843, 2126345047, 343880121>()].span(),
            [qm31_const::<1986554125, 913513571, 45970264, 343880122>()].span(),
            [qm31_const::<2053663304, 1047731299, 113079128, 343880122>()].span(),
            [qm31_const::<1583899051, 108207203, 1790800727, 343880121>()].span(),
            [qm31_const::<1651008230, 242424931, 1857909591, 343880121>()].span(),
            [qm31_const::<1718117409, 376642659, 1925018455, 343880121>()].span(),
            [qm31_const::<1785226588, 510860387, 1992127319, 343880121>()].span(),
            [qm31_const::<1315462335, 1718819938, 1522365270, 343880121>()].span(),
            [qm31_const::<1382571514, 1853037666, 1589474134, 343880121>()].span(),
            [qm31_const::<1986820986, 913513739, 45970432, 343880178>()].span(),
            [qm31_const::<1919711807, 779296011, 2126345215, 343880177>()].span(),
            [qm31_const::<2121039344, 1181949195, 180188160, 343880178>()].span(),
            [qm31_const::<2053930165, 1047731467, 113079296, 343880178>()].span(),
            [qm31_const::<1718384270, 376642827, 1925018623, 343880177>()].span(),
            [qm31_const::<1651275091, 242425099, 1857909759, 343880177>()].span(),
            [qm31_const::<1852602628, 645078283, 2059236351, 343880177>()].span(),
            [qm31_const::<1785493449, 510860555, 1992127487, 343880177>()].span(),
            [qm31_const::<700269555, 307766862, 1685683780, 745982081>()].span(),
        ]
            .span();
        let interaction_values = array![
            qm31_const::<1005168032, 79980996, 1847888101, 1941984119>(),
            qm31_const::<1072277211, 214198724, 1914996965, 1941984119>(),
            qm31_const::<1139386390, 348416452, 1982105829, 1941984119>(),
            qm31_const::<1206495569, 482634180, 2049214693, 1941984119>(),
            qm31_const::<736731316, 1690593731, 1579452644, 1941984119>(),
            qm31_const::<803840495, 1824811459, 1646561508, 1941984119>(),
            qm31_const::<870949674, 1959029187, 1713670372, 1941984119>(),
            qm31_const::<938058853, 2093246915, 1780779236, 1941984119>(),
        ];
        let mut interaction_columns = make_interaction_trace(
            interaction_values, qm31_const::<1115374022, 1127856551, 489657863, 643630026>(),
        );

        let claimed_sum = qm31_const::<1398335417, 314974026, 1722107152, 821933968>();
        let seq = qm31_const::<735272696, 1215403647, 795393303, 879304430>();
        let params = ConstraintParams {
            common_lookup_elements, claimed_sum, seq, column_size: m31(pow2(log_size)), offset,
        };
        let random_coeff = qm31_const::<474642921, 876336632, 1911695779, 974600512>();
        evaluate_constraints_at_point(
            ref sum, ref trace_columns, ref interaction_columns, params, random_coeff,
        );
        assert_eq!(sum, expected_result)
    }
}
