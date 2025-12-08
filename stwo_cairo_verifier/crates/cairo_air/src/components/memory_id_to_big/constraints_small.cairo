use crate::prelude::*;

#[derive(Drop)]
pub struct ConstraintParams {
    pub MemoryIdToBig_alpha0: QM31,
    pub MemoryIdToBig_alpha1: QM31,
    pub MemoryIdToBig_alpha2: QM31,
    pub MemoryIdToBig_alpha3: QM31,
    pub MemoryIdToBig_alpha4: QM31,
    pub MemoryIdToBig_alpha5: QM31,
    pub MemoryIdToBig_alpha6: QM31,
    pub MemoryIdToBig_alpha7: QM31,
    pub MemoryIdToBig_alpha8: QM31,
    pub MemoryIdToBig_z: QM31,
    pub RangeCheck_9_9_alpha0: QM31,
    pub RangeCheck_9_9_alpha1: QM31,
    pub RangeCheck_9_9_z: QM31,
    pub RangeCheck_9_9_b_alpha0: QM31,
    pub RangeCheck_9_9_b_alpha1: QM31,
    pub RangeCheck_9_9_b_z: QM31,
    pub RangeCheck_9_9_c_alpha0: QM31,
    pub RangeCheck_9_9_c_alpha1: QM31,
    pub RangeCheck_9_9_c_z: QM31,
    pub RangeCheck_9_9_d_alpha0: QM31,
    pub RangeCheck_9_9_d_alpha1: QM31,
    pub RangeCheck_9_9_d_z: QM31,
    pub claimed_sum: QM31,
    pub seq: QM31,
    pub column_size: M31,
}

pub fn evaluate_constraints_at_point(
    ref sum: QM31,
    ref trace_mask_values: ColumnSpan<Span<QM31>>,
    ref interaction_mask_values: ColumnSpan<Span<QM31>>,
    params: ConstraintParams,
    random_coeff: QM31,
    domain_vanish_at_point_inv: QM31,
) {
    let ConstraintParams {
        MemoryIdToBig_alpha0,
        MemoryIdToBig_alpha1,
        MemoryIdToBig_alpha2,
        MemoryIdToBig_alpha3,
        MemoryIdToBig_alpha4,
        MemoryIdToBig_alpha5,
        MemoryIdToBig_alpha6,
        MemoryIdToBig_alpha7,
        MemoryIdToBig_alpha8,
        MemoryIdToBig_z,
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        RangeCheck_9_9_b_alpha0,
        RangeCheck_9_9_b_alpha1,
        RangeCheck_9_9_b_z,
        RangeCheck_9_9_c_alpha0,
        RangeCheck_9_9_c_alpha1,
        RangeCheck_9_9_c_z,
        RangeCheck_9_9_d_alpha0,
        RangeCheck_9_9_d_alpha1,
        RangeCheck_9_9_d_z,
        claimed_sum,
        seq,
        column_size,
    } = params;
    let [
        trace_1_column_0,
        trace_1_column_1,
        trace_1_column_2,
        trace_1_column_3,
        trace_1_column_4,
        trace_1_column_5,
        trace_1_column_6,
        trace_1_column_7,
        trace_1_column_8,
    ]: [Span<QM31>; 9] =
        (*trace_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    let [trace_1_column_0_offset_0]: [QM31; 1] = (*trace_1_column_0.try_into().unwrap()).unbox();

    let [trace_1_column_1_offset_0]: [QM31; 1] = (*trace_1_column_1.try_into().unwrap()).unbox();

    let [trace_1_column_2_offset_0]: [QM31; 1] = (*trace_1_column_2.try_into().unwrap()).unbox();

    let [trace_1_column_3_offset_0]: [QM31; 1] = (*trace_1_column_3.try_into().unwrap()).unbox();

    let [trace_1_column_4_offset_0]: [QM31; 1] = (*trace_1_column_4.try_into().unwrap()).unbox();

    let [trace_1_column_5_offset_0]: [QM31; 1] = (*trace_1_column_5.try_into().unwrap()).unbox();

    let [trace_1_column_6_offset_0]: [QM31; 1] = (*trace_1_column_6.try_into().unwrap()).unbox();

    let [trace_1_column_7_offset_0]: [QM31; 1] = (*trace_1_column_7.try_into().unwrap()).unbox();

    let [trace_1_column_8_offset_0]: [QM31; 1] = (*trace_1_column_8.try_into().unwrap()).unbox();

    let [
        trace_2_column_9,
        trace_2_column_10,
        trace_2_column_11,
        trace_2_column_12,
        trace_2_column_13,
        trace_2_column_14,
        trace_2_column_15,
        trace_2_column_16,
        trace_2_column_17,
        trace_2_column_18,
        trace_2_column_19,
        trace_2_column_20,
    ]: [Span<QM31>; 12] =
        (*interaction_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    let [trace_2_column_9_offset_0]: [QM31; 1] = (*trace_2_column_9.try_into().unwrap()).unbox();

    let [trace_2_column_10_offset_0]: [QM31; 1] = (*trace_2_column_10.try_into().unwrap()).unbox();

    let [trace_2_column_11_offset_0]: [QM31; 1] = (*trace_2_column_11.try_into().unwrap()).unbox();

    let [trace_2_column_12_offset_0]: [QM31; 1] = (*trace_2_column_12.try_into().unwrap()).unbox();

    let [trace_2_column_13_offset_0]: [QM31; 1] = (*trace_2_column_13.try_into().unwrap()).unbox();

    let [trace_2_column_14_offset_0]: [QM31; 1] = (*trace_2_column_14.try_into().unwrap()).unbox();

    let [trace_2_column_15_offset_0]: [QM31; 1] = (*trace_2_column_15.try_into().unwrap()).unbox();

    let [trace_2_column_16_offset_0]: [QM31; 1] = (*trace_2_column_16.try_into().unwrap()).unbox();

    let [trace_2_column_17_offset_neg_1, trace_2_column_17_offset_0]: [QM31; 2] =
        (*trace_2_column_17
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_18_offset_neg_1, trace_2_column_18_offset_0]: [QM31; 2] =
        (*trace_2_column_18
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_19_offset_neg_1, trace_2_column_19_offset_0]: [QM31; 2] =
        (*trace_2_column_19
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_20_offset_neg_1, trace_2_column_20_offset_0]: [QM31; 2] =
        (*trace_2_column_20
        .try_into()
        .unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let mut intermediates = intermediates(
        MemoryIdToBig_alpha0,
        MemoryIdToBig_alpha1,
        MemoryIdToBig_alpha2,
        MemoryIdToBig_alpha3,
        MemoryIdToBig_alpha4,
        MemoryIdToBig_alpha5,
        MemoryIdToBig_alpha6,
        MemoryIdToBig_alpha7,
        MemoryIdToBig_alpha8,
        MemoryIdToBig_z,
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        RangeCheck_9_9_b_alpha0,
        RangeCheck_9_9_b_alpha1,
        RangeCheck_9_9_b_z,
        RangeCheck_9_9_c_alpha0,
        RangeCheck_9_9_c_alpha1,
        RangeCheck_9_9_c_z,
        RangeCheck_9_9_d_alpha0,
        RangeCheck_9_9_d_alpha1,
        RangeCheck_9_9_d_z,
        seq,
        trace_1_column_0_offset_0,
        trace_1_column_1_offset_0,
        trace_1_column_2_offset_0,
        trace_1_column_3_offset_0,
        trace_1_column_4_offset_0,
        trace_1_column_5_offset_0,
        trace_1_column_6_offset_0,
        trace_1_column_7_offset_0,
    )
        .span();
    let intermediate0 = *intermediates.pop_front().unwrap();
    let intermediate1 = *intermediates.pop_front().unwrap();
    let intermediate2 = *intermediates.pop_front().unwrap();
    let intermediate3 = *intermediates.pop_front().unwrap();
    let intermediate4 = *intermediates.pop_front().unwrap();

    // Constraint 0
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_9_offset_0, trace_2_column_10_offset_0, trace_2_column_11_offset_0,
            trace_2_column_12_offset_0,
        ],
    ))
        * ((intermediate0) * (intermediate1))
        - (intermediate1 + intermediate0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 1
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_13_offset_0, trace_2_column_14_offset_0, trace_2_column_15_offset_0,
            trace_2_column_16_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_9_offset_0, trace_2_column_10_offset_0, trace_2_column_11_offset_0,
                trace_2_column_12_offset_0,
            ],
        )))
        * ((intermediate2) * (intermediate3))
        - (intermediate3 + intermediate2))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 2
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_17_offset_0, trace_2_column_18_offset_0, trace_2_column_19_offset_0,
            trace_2_column_20_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_13_offset_0, trace_2_column_14_offset_0, trace_2_column_15_offset_0,
                trace_2_column_16_offset_0,
            ],
        ))
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_17_offset_neg_1, trace_2_column_18_offset_neg_1,
                trace_2_column_19_offset_neg_1, trace_2_column_20_offset_neg_1,
            ],
        ))
        + (claimed_sum) * (column_size.inverse().into()))
        * (intermediate4)
        - (-(trace_1_column_8_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;
}


fn intermediates(
    MemoryIdToBig_alpha0: QM31,
    MemoryIdToBig_alpha1: QM31,
    MemoryIdToBig_alpha2: QM31,
    MemoryIdToBig_alpha3: QM31,
    MemoryIdToBig_alpha4: QM31,
    MemoryIdToBig_alpha5: QM31,
    MemoryIdToBig_alpha6: QM31,
    MemoryIdToBig_alpha7: QM31,
    MemoryIdToBig_alpha8: QM31,
    MemoryIdToBig_z: QM31,
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    RangeCheck_9_9_b_alpha0: QM31,
    RangeCheck_9_9_b_alpha1: QM31,
    RangeCheck_9_9_b_z: QM31,
    RangeCheck_9_9_c_alpha0: QM31,
    RangeCheck_9_9_c_alpha1: QM31,
    RangeCheck_9_9_c_z: QM31,
    RangeCheck_9_9_d_alpha0: QM31,
    RangeCheck_9_9_d_alpha1: QM31,
    RangeCheck_9_9_d_z: QM31,
    seq: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
) -> Array<QM31> {
    let intermediate4 = intermediate4(
        MemoryIdToBig_alpha0,
        MemoryIdToBig_alpha1,
        MemoryIdToBig_alpha2,
        MemoryIdToBig_alpha3,
        MemoryIdToBig_alpha4,
        MemoryIdToBig_alpha5,
        MemoryIdToBig_alpha6,
        MemoryIdToBig_alpha7,
        MemoryIdToBig_alpha8,
        MemoryIdToBig_z,
        seq,
        trace_1_column_0_offset_0,
        trace_1_column_1_offset_0,
        trace_1_column_2_offset_0,
        trace_1_column_3_offset_0,
        trace_1_column_4_offset_0,
        trace_1_column_5_offset_0,
        trace_1_column_6_offset_0,
        trace_1_column_7_offset_0,
    );

    let intermediate3 = intermediate3(
        RangeCheck_9_9_d_alpha0,
        RangeCheck_9_9_d_alpha1,
        RangeCheck_9_9_d_z,
        trace_1_column_6_offset_0,
        trace_1_column_7_offset_0,
    );

    let intermediate0 = intermediate0(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_0_offset_0,
        trace_1_column_1_offset_0,
    );

    let intermediate1 = intermediate1(
        RangeCheck_9_9_b_alpha0,
        RangeCheck_9_9_b_alpha1,
        RangeCheck_9_9_b_z,
        trace_1_column_2_offset_0,
        trace_1_column_3_offset_0,
    );

    let intermediate2 = intermediate2(
        RangeCheck_9_9_c_alpha0,
        RangeCheck_9_9_c_alpha1,
        RangeCheck_9_9_c_z,
        trace_1_column_4_offset_0,
        trace_1_column_5_offset_0,
    );
    array![intermediate0, intermediate1, intermediate2, intermediate3, intermediate4]
}


pub fn intermediate4(
    MemoryIdToBig_alpha0: QM31,
    MemoryIdToBig_alpha1: QM31,
    MemoryIdToBig_alpha2: QM31,
    MemoryIdToBig_alpha3: QM31,
    MemoryIdToBig_alpha4: QM31,
    MemoryIdToBig_alpha5: QM31,
    MemoryIdToBig_alpha6: QM31,
    MemoryIdToBig_alpha7: QM31,
    MemoryIdToBig_alpha8: QM31,
    MemoryIdToBig_z: QM31,
    seq: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
) -> QM31 {
    (MemoryIdToBig_alpha0) * (seq)
        + (MemoryIdToBig_alpha1) * (trace_1_column_0_offset_0)
        + (MemoryIdToBig_alpha2) * (trace_1_column_1_offset_0)
        + (MemoryIdToBig_alpha3) * (trace_1_column_2_offset_0)
        + (MemoryIdToBig_alpha4) * (trace_1_column_3_offset_0)
        + (MemoryIdToBig_alpha5) * (trace_1_column_4_offset_0)
        + (MemoryIdToBig_alpha6) * (trace_1_column_5_offset_0)
        + (MemoryIdToBig_alpha7) * (trace_1_column_6_offset_0)
        + (MemoryIdToBig_alpha8) * (trace_1_column_7_offset_0)
        - (MemoryIdToBig_z)
}

pub fn intermediate3(
    RangeCheck_9_9_d_alpha0: QM31,
    RangeCheck_9_9_d_alpha1: QM31,
    RangeCheck_9_9_d_z: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_d_alpha0) * (trace_1_column_6_offset_0)
        + (RangeCheck_9_9_d_alpha1) * (trace_1_column_7_offset_0)
        - (RangeCheck_9_9_d_z)
}

pub fn intermediate0(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_0_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_1_offset_0)
        - (RangeCheck_9_9_z)
}

pub fn intermediate1(
    RangeCheck_9_9_b_alpha0: QM31,
    RangeCheck_9_9_b_alpha1: QM31,
    RangeCheck_9_9_b_z: QM31,
    trace_1_column_2_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_b_alpha0) * (trace_1_column_2_offset_0)
        + (RangeCheck_9_9_b_alpha1) * (trace_1_column_3_offset_0)
        - (RangeCheck_9_9_b_z)
}

pub fn intermediate2(
    RangeCheck_9_9_c_alpha0: QM31,
    RangeCheck_9_9_c_alpha1: QM31,
    RangeCheck_9_9_c_z: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_c_alpha0) * (trace_1_column_4_offset_0)
        + (RangeCheck_9_9_c_alpha1) * (trace_1_column_5_offset_0)
        - (RangeCheck_9_9_c_z)
}

#[cfg(and(test, feature: "qm31_opcode"))]
mod tests {
    use core::num::traits::Zero;
    use core::num::traits::one::One;
    use stwo_verifier_core::circle::CirclePoint;
    use stwo_verifier_core::fields::Invertible;
    use stwo_verifier_core::fields::m31::m31;
    use stwo_verifier_core::fields::qm31::{QM31, QM31Trait, qm31_const};
    use stwo_verifier_core::poly::circle::CanonicCosetImpl;
    use stwo_verifier_core::utils::pow2;
    use crate::components::sample_evaluations::*;
    use crate::test_utils::{make_interaction_trace, make_lookup_elements};
    use super::{ConstraintParams, evaluate_constraints_at_point};

    #[test]
    fn test_evaluation_result() {
        let log_size = 15;

        let memory_id_to_big_lookup_elements = make_lookup_elements::<
            29,
        >(
            qm31_const::<844624398, 1166453613, 1247584074, 330174372>(),
            qm31_const::<1844105245, 1400976933, 1126903288, 1155460729>(),
        );
        let range_check_9_9_lookup_elements = make_lookup_elements::<
            2,
        >(
            qm31_const::<989827041, 1225728465, 1602128278, 85336129>(),
            qm31_const::<1454375758, 8286589, 1713209810, 1602293816>(),
        );
        let range_check_9_9_b_lookup_elements = make_lookup_elements::<
            2,
        >(
            qm31_const::<676159317, 930503385, 1105489908, 1544380136>(),
            qm31_const::<2129889251, 701815395, 1830411342, 2061777868>(),
        );
        let range_check_9_9_c_lookup_elements = make_lookup_elements::<
            2,
        >(
            qm31_const::<1260569667, 2138441994, 1709448741, 1544373155>(),
            qm31_const::<1022885008, 826842007, 1709607881, 1909661957>(),
        );
        let range_check_9_9_d_lookup_elements = make_lookup_elements::<
            2,
        >(
            qm31_const::<1551136661, 662010924, 2044956999, 1544361134>(),
            qm31_const::<2005146556, 852740197, 532387412, 1763320973>(),
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
            MemoryIdToBig_alpha0: One::one(),
            MemoryIdToBig_alpha1: *memory_id_to_big_lookup_elements.alpha_powers[0],
            MemoryIdToBig_alpha2: *memory_id_to_big_lookup_elements.alpha_powers[1],
            MemoryIdToBig_alpha3: *memory_id_to_big_lookup_elements.alpha_powers[2],
            MemoryIdToBig_alpha4: *memory_id_to_big_lookup_elements.alpha_powers[3],
            MemoryIdToBig_alpha5: *memory_id_to_big_lookup_elements.alpha_powers[4],
            MemoryIdToBig_alpha6: *memory_id_to_big_lookup_elements.alpha_powers[5],
            MemoryIdToBig_alpha7: *memory_id_to_big_lookup_elements.alpha_powers[6],
            MemoryIdToBig_alpha8: *memory_id_to_big_lookup_elements.alpha_powers[7],
            MemoryIdToBig_z: memory_id_to_big_lookup_elements.z,
            RangeCheck_9_9_alpha0: One::one(),
            RangeCheck_9_9_alpha1: *range_check_9_9_lookup_elements.alpha_powers[0],
            RangeCheck_9_9_z: range_check_9_9_lookup_elements.z,
            RangeCheck_9_9_b_alpha0: One::one(),
            RangeCheck_9_9_b_alpha1: *range_check_9_9_b_lookup_elements.alpha_powers[0],
            RangeCheck_9_9_b_z: range_check_9_9_b_lookup_elements.z,
            RangeCheck_9_9_c_alpha0: One::one(),
            RangeCheck_9_9_c_alpha1: *range_check_9_9_c_lookup_elements.alpha_powers[0],
            RangeCheck_9_9_c_z: range_check_9_9_c_lookup_elements.z,
            RangeCheck_9_9_d_alpha0: One::one(),
            RangeCheck_9_9_d_alpha1: *range_check_9_9_d_lookup_elements.alpha_powers[0],
            RangeCheck_9_9_d_z: range_check_9_9_d_lookup_elements.z,
            claimed_sum,
            seq,
            column_size: m31(pow2(log_size)),
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
