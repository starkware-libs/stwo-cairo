use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedColumnSetImpl,
};
use stwo_verifier_core::circle::{
    CirclePoint, CirclePointIndex, CirclePointIndexImpl, CirclePointQM31AddCirclePointM31Impl,
};
use stwo_verifier_core::fields::m31::{M31, m31};
use stwo_verifier_core::fields::qm31::{QM31, QM31Impl, qm31};
use stwo_verifier_core::{ColumnArray, ColumnSpan};


pub fn mask_points(
    ref preprocessed_column_set: PreprocessedColumnSet,
    ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
    ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
    point: CirclePoint<QM31>,
    trace_gen: CirclePointIndex,
    log_size: u32,
) {
    let point_offset_neg_1 = point.add_circle_point_m31(-trace_gen.mul(1).to_point());
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
}

#[derive(Drop)]
pub struct ConstraintParams {
    pub MemoryIdToBig_alpha0: QM31,
    pub MemoryIdToBig_alpha1: QM31,
    pub MemoryIdToBig_alpha10: QM31,
    pub MemoryIdToBig_alpha11: QM31,
    pub MemoryIdToBig_alpha12: QM31,
    pub MemoryIdToBig_alpha13: QM31,
    pub MemoryIdToBig_alpha14: QM31,
    pub MemoryIdToBig_alpha15: QM31,
    pub MemoryIdToBig_alpha16: QM31,
    pub MemoryIdToBig_alpha17: QM31,
    pub MemoryIdToBig_alpha18: QM31,
    pub MemoryIdToBig_alpha19: QM31,
    pub MemoryIdToBig_alpha2: QM31,
    pub MemoryIdToBig_alpha20: QM31,
    pub MemoryIdToBig_alpha21: QM31,
    pub MemoryIdToBig_alpha22: QM31,
    pub MemoryIdToBig_alpha23: QM31,
    pub MemoryIdToBig_alpha24: QM31,
    pub MemoryIdToBig_alpha25: QM31,
    pub MemoryIdToBig_alpha26: QM31,
    pub MemoryIdToBig_alpha27: QM31,
    pub MemoryIdToBig_alpha28: QM31,
    pub MemoryIdToBig_alpha3: QM31,
    pub MemoryIdToBig_alpha4: QM31,
    pub MemoryIdToBig_alpha5: QM31,
    pub MemoryIdToBig_alpha6: QM31,
    pub MemoryIdToBig_alpha7: QM31,
    pub MemoryIdToBig_alpha8: QM31,
    pub MemoryIdToBig_alpha9: QM31,
    pub MemoryIdToBig_z: QM31,
    pub RangeCheck_9_9_alpha0: QM31,
    pub RangeCheck_9_9_alpha1: QM31,
    pub RangeCheck_9_9_z: QM31,
    pub claimed_sum: QM31,
}

pub fn evaluate_constraints_at_point(
    ref sum: QM31,
    ref trace_mask_values: ColumnSpan<Array<QM31>>,
    ref interaction_mask_values: ColumnSpan<Array<QM31>>,
    params: ConstraintParams,
    random_coeff: QM31,
    domain_vanish_at_point_inv: QM31,
) {}


fn intermediates(
    MemoryIdToBig_alpha0: QM31,
    MemoryIdToBig_alpha1: QM31,
    MemoryIdToBig_alpha10: QM31,
    MemoryIdToBig_alpha11: QM31,
    MemoryIdToBig_alpha12: QM31,
    MemoryIdToBig_alpha13: QM31,
    MemoryIdToBig_alpha14: QM31,
    MemoryIdToBig_alpha15: QM31,
    MemoryIdToBig_alpha16: QM31,
    MemoryIdToBig_alpha17: QM31,
    MemoryIdToBig_alpha18: QM31,
    MemoryIdToBig_alpha19: QM31,
    MemoryIdToBig_alpha2: QM31,
    MemoryIdToBig_alpha20: QM31,
    MemoryIdToBig_alpha21: QM31,
    MemoryIdToBig_alpha22: QM31,
    MemoryIdToBig_alpha23: QM31,
    MemoryIdToBig_alpha24: QM31,
    MemoryIdToBig_alpha25: QM31,
    MemoryIdToBig_alpha26: QM31,
    MemoryIdToBig_alpha27: QM31,
    MemoryIdToBig_alpha28: QM31,
    MemoryIdToBig_alpha3: QM31,
    MemoryIdToBig_alpha4: QM31,
    MemoryIdToBig_alpha5: QM31,
    MemoryIdToBig_alpha6: QM31,
    MemoryIdToBig_alpha7: QM31,
    MemoryIdToBig_alpha8: QM31,
    MemoryIdToBig_alpha9: QM31,
    MemoryIdToBig_z: QM31,
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_10_offset_0: QM31,
    trace_1_column_11_offset_0: QM31,
    trace_1_column_12_offset_0: QM31,
    trace_1_column_13_offset_0: QM31,
    trace_1_column_14_offset_0: QM31,
    trace_1_column_15_offset_0: QM31,
    trace_1_column_16_offset_0: QM31,
    trace_1_column_17_offset_0: QM31,
    trace_1_column_18_offset_0: QM31,
    trace_1_column_19_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
    trace_1_column_20_offset_0: QM31,
    trace_1_column_21_offset_0: QM31,
    trace_1_column_22_offset_0: QM31,
    trace_1_column_23_offset_0: QM31,
    trace_1_column_24_offset_0: QM31,
    trace_1_column_25_offset_0: QM31,
    trace_1_column_26_offset_0: QM31,
    trace_1_column_27_offset_0: QM31,
    trace_1_column_28_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
    trace_1_column_9_offset_0: QM31,
) -> Array<QM31> {
    let intermediate0 = intermediate0(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_1_offset_0,
        trace_1_column_2_offset_0,
    );

    let intermediate1 = intermediate1(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_3_offset_0,
        trace_1_column_4_offset_0,
    );

    let intermediate2 = intermediate2(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_5_offset_0,
        trace_1_column_6_offset_0,
    );

    let intermediate3 = intermediate3(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_7_offset_0,
        trace_1_column_8_offset_0,
    );

    let intermediate4 = intermediate4(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_10_offset_0,
        trace_1_column_9_offset_0,
    );

    let intermediate5 = intermediate5(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_11_offset_0,
        trace_1_column_12_offset_0,
    );

    let intermediate6 = intermediate6(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_13_offset_0,
        trace_1_column_14_offset_0,
    );

    let intermediate7 = intermediate7(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_15_offset_0,
        trace_1_column_16_offset_0,
    );

    let intermediate8 = intermediate8(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_17_offset_0,
        trace_1_column_18_offset_0,
    );

    let intermediate9 = intermediate9(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_19_offset_0,
        trace_1_column_20_offset_0,
    );

    let intermediate10 = intermediate10(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_21_offset_0,
        trace_1_column_22_offset_0,
    );

    let intermediate11 = intermediate11(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_23_offset_0,
        trace_1_column_24_offset_0,
    );

    let intermediate12 = intermediate12(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_25_offset_0,
        trace_1_column_26_offset_0,
    );

    let intermediate13 = intermediate13(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_27_offset_0,
        trace_1_column_28_offset_0,
    );

    let intermediate14 = intermediate14(
        MemoryIdToBig_alpha0,
        MemoryIdToBig_alpha1,
        MemoryIdToBig_alpha10,
        MemoryIdToBig_alpha11,
        MemoryIdToBig_alpha12,
        MemoryIdToBig_alpha13,
        MemoryIdToBig_alpha14,
        MemoryIdToBig_alpha15,
        MemoryIdToBig_alpha16,
        MemoryIdToBig_alpha17,
        MemoryIdToBig_alpha18,
        MemoryIdToBig_alpha19,
        MemoryIdToBig_alpha2,
        MemoryIdToBig_alpha20,
        MemoryIdToBig_alpha21,
        MemoryIdToBig_alpha22,
        MemoryIdToBig_alpha23,
        MemoryIdToBig_alpha24,
        MemoryIdToBig_alpha25,
        MemoryIdToBig_alpha26,
        MemoryIdToBig_alpha27,
        MemoryIdToBig_alpha28,
        MemoryIdToBig_alpha3,
        MemoryIdToBig_alpha4,
        MemoryIdToBig_alpha5,
        MemoryIdToBig_alpha6,
        MemoryIdToBig_alpha7,
        MemoryIdToBig_alpha8,
        MemoryIdToBig_alpha9,
        MemoryIdToBig_z,
        trace_1_column_0_offset_0,
        trace_1_column_10_offset_0,
        trace_1_column_11_offset_0,
        trace_1_column_12_offset_0,
        trace_1_column_13_offset_0,
        trace_1_column_14_offset_0,
        trace_1_column_15_offset_0,
        trace_1_column_16_offset_0,
        trace_1_column_17_offset_0,
        trace_1_column_18_offset_0,
        trace_1_column_19_offset_0,
        trace_1_column_1_offset_0,
        trace_1_column_20_offset_0,
        trace_1_column_21_offset_0,
        trace_1_column_22_offset_0,
        trace_1_column_23_offset_0,
        trace_1_column_24_offset_0,
        trace_1_column_25_offset_0,
        trace_1_column_26_offset_0,
        trace_1_column_27_offset_0,
        trace_1_column_28_offset_0,
        trace_1_column_2_offset_0,
        trace_1_column_3_offset_0,
        trace_1_column_4_offset_0,
        trace_1_column_5_offset_0,
        trace_1_column_6_offset_0,
        trace_1_column_7_offset_0,
        trace_1_column_8_offset_0,
        trace_1_column_9_offset_0,
    );
    array![
        intermediate0,
        intermediate1,
        intermediate2,
        intermediate3,
        intermediate4,
        intermediate5,
        intermediate6,
        intermediate7,
        intermediate8,
        intermediate9,
        intermediate10,
        intermediate11,
        intermediate12,
        intermediate13,
        intermediate14,
    ]
}


pub fn intermediate0(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_1_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_1_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_2_offset_0)
        - (RangeCheck_9_9_z)
}

pub fn intermediate1(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_3_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_3_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_4_offset_0)
        - (RangeCheck_9_9_z)
}

pub fn intermediate2(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_5_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_5_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_6_offset_0)
        - (RangeCheck_9_9_z)
}

pub fn intermediate3(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_7_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_7_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_8_offset_0)
        - (RangeCheck_9_9_z)
}

pub fn intermediate4(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_10_offset_0: QM31,
    trace_1_column_9_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_9_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_10_offset_0)
        - (RangeCheck_9_9_z)
}

pub fn intermediate5(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_11_offset_0: QM31,
    trace_1_column_12_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_11_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_12_offset_0)
        - (RangeCheck_9_9_z)
}

pub fn intermediate6(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_13_offset_0: QM31,
    trace_1_column_14_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_13_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_14_offset_0)
        - (RangeCheck_9_9_z)
}

pub fn intermediate7(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_15_offset_0: QM31,
    trace_1_column_16_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_15_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_16_offset_0)
        - (RangeCheck_9_9_z)
}

pub fn intermediate8(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_17_offset_0: QM31,
    trace_1_column_18_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_17_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_18_offset_0)
        - (RangeCheck_9_9_z)
}

pub fn intermediate9(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_19_offset_0: QM31,
    trace_1_column_20_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_19_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_20_offset_0)
        - (RangeCheck_9_9_z)
}

pub fn intermediate10(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_21_offset_0: QM31,
    trace_1_column_22_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_21_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_22_offset_0)
        - (RangeCheck_9_9_z)
}

pub fn intermediate11(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_23_offset_0: QM31,
    trace_1_column_24_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_23_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_24_offset_0)
        - (RangeCheck_9_9_z)
}

pub fn intermediate12(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_25_offset_0: QM31,
    trace_1_column_26_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_25_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_26_offset_0)
        - (RangeCheck_9_9_z)
}

pub fn intermediate13(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_27_offset_0: QM31,
    trace_1_column_28_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_27_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_28_offset_0)
        - (RangeCheck_9_9_z)
}

pub fn intermediate14(
    MemoryIdToBig_alpha0: QM31,
    MemoryIdToBig_alpha1: QM31,
    MemoryIdToBig_alpha10: QM31,
    MemoryIdToBig_alpha11: QM31,
    MemoryIdToBig_alpha12: QM31,
    MemoryIdToBig_alpha13: QM31,
    MemoryIdToBig_alpha14: QM31,
    MemoryIdToBig_alpha15: QM31,
    MemoryIdToBig_alpha16: QM31,
    MemoryIdToBig_alpha17: QM31,
    MemoryIdToBig_alpha18: QM31,
    MemoryIdToBig_alpha19: QM31,
    MemoryIdToBig_alpha2: QM31,
    MemoryIdToBig_alpha20: QM31,
    MemoryIdToBig_alpha21: QM31,
    MemoryIdToBig_alpha22: QM31,
    MemoryIdToBig_alpha23: QM31,
    MemoryIdToBig_alpha24: QM31,
    MemoryIdToBig_alpha25: QM31,
    MemoryIdToBig_alpha26: QM31,
    MemoryIdToBig_alpha27: QM31,
    MemoryIdToBig_alpha28: QM31,
    MemoryIdToBig_alpha3: QM31,
    MemoryIdToBig_alpha4: QM31,
    MemoryIdToBig_alpha5: QM31,
    MemoryIdToBig_alpha6: QM31,
    MemoryIdToBig_alpha7: QM31,
    MemoryIdToBig_alpha8: QM31,
    MemoryIdToBig_alpha9: QM31,
    MemoryIdToBig_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_10_offset_0: QM31,
    trace_1_column_11_offset_0: QM31,
    trace_1_column_12_offset_0: QM31,
    trace_1_column_13_offset_0: QM31,
    trace_1_column_14_offset_0: QM31,
    trace_1_column_15_offset_0: QM31,
    trace_1_column_16_offset_0: QM31,
    trace_1_column_17_offset_0: QM31,
    trace_1_column_18_offset_0: QM31,
    trace_1_column_19_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
    trace_1_column_20_offset_0: QM31,
    trace_1_column_21_offset_0: QM31,
    trace_1_column_22_offset_0: QM31,
    trace_1_column_23_offset_0: QM31,
    trace_1_column_24_offset_0: QM31,
    trace_1_column_25_offset_0: QM31,
    trace_1_column_26_offset_0: QM31,
    trace_1_column_27_offset_0: QM31,
    trace_1_column_28_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
    trace_1_column_9_offset_0: QM31,
) -> QM31 {
    (MemoryIdToBig_alpha0) * (trace_1_column_0_offset_0)
        + (MemoryIdToBig_alpha1) * (trace_1_column_1_offset_0)
        + (MemoryIdToBig_alpha2) * (trace_1_column_2_offset_0)
        + (MemoryIdToBig_alpha3) * (trace_1_column_3_offset_0)
        + (MemoryIdToBig_alpha4) * (trace_1_column_4_offset_0)
        + (MemoryIdToBig_alpha5) * (trace_1_column_5_offset_0)
        + (MemoryIdToBig_alpha6) * (trace_1_column_6_offset_0)
        + (MemoryIdToBig_alpha7) * (trace_1_column_7_offset_0)
        + (MemoryIdToBig_alpha8) * (trace_1_column_8_offset_0)
        + (MemoryIdToBig_alpha9) * (trace_1_column_9_offset_0)
        + (MemoryIdToBig_alpha10) * (trace_1_column_10_offset_0)
        + (MemoryIdToBig_alpha11) * (trace_1_column_11_offset_0)
        + (MemoryIdToBig_alpha12) * (trace_1_column_12_offset_0)
        + (MemoryIdToBig_alpha13) * (trace_1_column_13_offset_0)
        + (MemoryIdToBig_alpha14) * (trace_1_column_14_offset_0)
        + (MemoryIdToBig_alpha15) * (trace_1_column_15_offset_0)
        + (MemoryIdToBig_alpha16) * (trace_1_column_16_offset_0)
        + (MemoryIdToBig_alpha17) * (trace_1_column_17_offset_0)
        + (MemoryIdToBig_alpha18) * (trace_1_column_18_offset_0)
        + (MemoryIdToBig_alpha19) * (trace_1_column_19_offset_0)
        + (MemoryIdToBig_alpha20) * (trace_1_column_20_offset_0)
        + (MemoryIdToBig_alpha21) * (trace_1_column_21_offset_0)
        + (MemoryIdToBig_alpha22) * (trace_1_column_22_offset_0)
        + (MemoryIdToBig_alpha23) * (trace_1_column_23_offset_0)
        + (MemoryIdToBig_alpha24) * (trace_1_column_24_offset_0)
        + (MemoryIdToBig_alpha25) * (trace_1_column_25_offset_0)
        + (MemoryIdToBig_alpha26) * (trace_1_column_26_offset_0)
        + (MemoryIdToBig_alpha27) * (trace_1_column_27_offset_0)
        + (MemoryIdToBig_alpha28) * (trace_1_column_28_offset_0)
        - (MemoryIdToBig_z)
}

