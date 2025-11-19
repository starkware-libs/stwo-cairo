use core::num::traits::Zero;
use crate::components::subroutines::range_check_mem_value_n_8::range_check_mem_value_n_8_evaluate;
use crate::prelude::*;

#[derive(Drop)]
pub struct ConstraintParams {
    pub common_lookup_elements: crate::CommonElements,
    pub claimed_sum: QM31,
    pub seq: QM31,
    pub column_size: M31,
}

pub fn evaluate_constraints_at_point(
    ref sum: QM31,
    ref trace_mask_values: ColumnSpan<Span<QM31>>,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    params: ConstraintParams,
    random_coeff: QM31,
    domain_vanishing_eval_inv: QM31,
) {
    let ConstraintParams { common_lookup_elements, claimed_sum, seq, column_size } = params;
    let mut range_check_9_9_sum_0: QM31 = Zero::zero();
    let mut range_check_9_9_b_sum_1: QM31 = Zero::zero();
    let mut range_check_9_9_c_sum_2: QM31 = Zero::zero();
    let mut range_check_9_9_d_sum_3: QM31 = Zero::zero();
    let mut memory_id_to_big_sum_4: QM31 = Zero::zero();

    let [
        memory_id_to_small_output_col0,
        memory_id_to_small_output_col1,
        memory_id_to_small_output_col2,
        memory_id_to_small_output_col3,
        memory_id_to_small_output_col4,
        memory_id_to_small_output_col5,
        memory_id_to_small_output_col6,
        memory_id_to_small_output_col7,
        enabler,
    ]: [Span<QM31>; 9] =
        (*trace_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();
    let [memory_id_to_small_output_col0]: [QM31; 1] = (*memory_id_to_small_output_col0
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_small_output_col1]: [QM31; 1] = (*memory_id_to_small_output_col1
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_small_output_col2]: [QM31; 1] = (*memory_id_to_small_output_col2
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_small_output_col3]: [QM31; 1] = (*memory_id_to_small_output_col3
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_small_output_col4]: [QM31; 1] = (*memory_id_to_small_output_col4
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_small_output_col5]: [QM31; 1] = (*memory_id_to_small_output_col5
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_small_output_col6]: [QM31; 1] = (*memory_id_to_small_output_col6
        .try_into()
        .unwrap())
        .unbox();
    let [memory_id_to_small_output_col7]: [QM31; 1] = (*memory_id_to_small_output_col7
        .try_into()
        .unwrap())
        .unbox();
    let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

    core::internal::revoke_ap_tracking();

    range_check_mem_value_n_8_evaluate(
        [
            memory_id_to_small_output_col0, memory_id_to_small_output_col1,
            memory_id_to_small_output_col2, memory_id_to_small_output_col3,
            memory_id_to_small_output_col4, memory_id_to_small_output_col5,
            memory_id_to_small_output_col6, memory_id_to_small_output_col7,
        ],
        @common_lookup_elements,
        ref range_check_9_9_sum_0,
        ref range_check_9_9_b_sum_1,
        ref range_check_9_9_c_sum_2,
        ref range_check_9_9_d_sum_3,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    memory_id_to_big_sum_4 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1662111297, 0, 0, 0>(), seq, memory_id_to_small_output_col0,
                memory_id_to_small_output_col1, memory_id_to_small_output_col2,
                memory_id_to_small_output_col3, memory_id_to_small_output_col4,
                memory_id_to_small_output_col5, memory_id_to_small_output_col6,
                memory_id_to_small_output_col7,
            ]
                .span(),
        );

    lookup_constraints(
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
        claimed_sum,
        enabler,
        column_size,
        ref interaction_trace_mask_values,
        range_check_9_9_sum_0,
        range_check_9_9_b_sum_1,
        range_check_9_9_c_sum_2,
        range_check_9_9_d_sum_3,
        memory_id_to_big_sum_4,
    )
}


fn lookup_constraints(
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    enabler: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    range_check_9_9_sum_0: QM31,
    range_check_9_9_b_sum_1: QM31,
    range_check_9_9_c_sum_2: QM31,
    range_check_9_9_d_sum_3: QM31,
    memory_id_to_big_sum_4: QM31,
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
    ]: [Span<QM31>; 12] =
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
    let [trace_2_col8_neg1, trace_2_col8]: [QM31; 2] = (*trace_2_col8.try_into().unwrap()).unbox();
    let [trace_2_col9_neg1, trace_2_col9]: [QM31; 2] = (*trace_2_col9.try_into().unwrap()).unbox();
    let [trace_2_col10_neg1, trace_2_col10]: [QM31; 2] = (*trace_2_col10.try_into().unwrap())
        .unbox();
    let [trace_2_col11_neg1, trace_2_col11]: [QM31; 2] = (*trace_2_col11.try_into().unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * range_check_9_9_sum_0
        * range_check_9_9_b_sum_1)
        - range_check_9_9_sum_0
        - range_check_9_9_b_sum_1)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * range_check_9_9_c_sum_2
        * range_check_9_9_d_sum_3)
        - range_check_9_9_c_sum_2
        - range_check_9_9_d_sum_3)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7])
        - QM31Impl::from_partial_evals(
            [trace_2_col8_neg1, trace_2_col9_neg1, trace_2_col10_neg1, trace_2_col11_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * memory_id_to_big_sum_4)
        + enabler)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}
#[cfg(and(test, feature: "qm31_opcode"))]
mod tests {
    use core::num::traits::Zero;
    use stwo_constraint_framework::LookupElementsTrait;
    use stwo_verifier_core::circle::CirclePoint;
    use stwo_verifier_core::fields::Invertible;
    use stwo_verifier_core::fields::m31::m31;
    use stwo_verifier_core::fields::qm31::{QM31, QM31Trait, qm31_const};
    use stwo_verifier_core::poly::circle::CanonicCosetImpl;
    use stwo_verifier_core::utils::pow2;
    use crate::components::sample_evaluations::*;
    use crate::test_utils::make_interaction_trace;
    use super::{ConstraintParams, evaluate_constraints_at_point};

    #[test]
    fn test_evaluation_result() {
        let log_size = 15;

        let common_lookup_elements = LookupElementsTrait::from_z_alpha(
            qm31_const::<445623802, 202571636, 1360224996, 131355117>(),
            qm31_const::<476823935, 939223384, 62486082, 122423602>(),
        );

        let mut sum: QM31 = Zero::zero();
        let point = CirclePoint {
            x: qm31_const::<461666434, 38651694, 1083586041, 510305943>(),
            y: qm31_const::<817798294, 862569777, 2091320744, 1178484122>(),
        };
        let mut trace_columns = [
            [qm31_const::<1659099300, 905558730, 651199673, 1375009625>()].span(),
            [qm31_const::<1591990121, 771341002, 584090809, 1375009625>()].span(),
            [qm31_const::<1793317658, 1173994186, 785417401, 1375009625>()].span(),
            [qm31_const::<1726208479, 1039776458, 718308537, 1375009625>()].span(),
            [qm31_const::<1390662584, 368687818, 382764217, 1375009625>()].span(),
            [qm31_const::<1323553405, 234470090, 315655353, 1375009625>()].span(),
            [qm31_const::<1524880942, 637123274, 516981945, 1375009625>()].span(),
            [qm31_const::<1457771763, 502905546, 449873081, 1375009625>()].span(),
            [qm31_const::<179325277, 825275894, 97341591, 1357105975>()].span(),
        ]
            .span();
        let interaction_values = array![
            qm31_const::<1005168032, 79980996, 1847888101, 1941984119>(),
            qm31_const::<1072277211, 214198724, 1914996965, 1941984119>(),
            qm31_const::<1139386390, 348416452, 1982105829, 1941984119>(),
        ];
        let mut interaction_columns = make_interaction_trace(
            interaction_values, qm31_const::<1115374022, 1127856551, 489657863, 643630026>(),
        );

        let claimed_sum = qm31_const::<1398335417, 314974026, 1722107152, 821933968>();
        let seq = qm31_const::<735272696, 1215403647, 795393303, 879304430>();
        let params = ConstraintParams {
            common_lookup_elements, claimed_sum, seq, column_size: m31(pow2(log_size)),
        };
        let trace_domain = CanonicCosetImpl::new(log_size);
        let domain_vanishing_eval_inv = trace_domain.eval_vanishing(point).inverse();
        let random_coeff = qm31_const::<474642921, 876336632, 1911695779, 974600512>();
        evaluate_constraints_at_point(
            ref sum,
            ref trace_columns,
            ref interaction_columns,
            params,
            random_coeff,
            domain_vanishing_eval_inv,
        );
        assert_eq!(sum, QM31Trait::from_fixed_array(MEMORY_ID_TO_SMALL_SAMPLE_EVAL_RESULT))
    }
}
