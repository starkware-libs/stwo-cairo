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
    preprocessed_column_set.insert(PreprocessedColumn::Seq(log_size));
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
    pub MemoryAddressToId_alpha0: QM31,
    pub MemoryAddressToId_alpha1: QM31,
    pub MemoryAddressToId_z: QM31,
    pub claimed_sum: QM31,
    pub preprocessed_seq: QM31,
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
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    preprocessed_seq: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_10_offset_0: QM31,
    trace_1_column_12_offset_0: QM31,
    trace_1_column_14_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
) -> Array<QM31> {
    let intermediate0 = intermediate0(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        preprocessed_seq,
        trace_1_column_0_offset_0,
    );

    let intermediate1 = intermediate1(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        preprocessed_seq,
        trace_1_column_2_offset_0,
    );

    let intermediate2 = intermediate2(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        preprocessed_seq,
        trace_1_column_4_offset_0,
    );

    let intermediate3 = intermediate3(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        preprocessed_seq,
        trace_1_column_6_offset_0,
    );

    let intermediate4 = intermediate4(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        preprocessed_seq,
        trace_1_column_8_offset_0,
    );

    let intermediate5 = intermediate5(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        preprocessed_seq,
        trace_1_column_10_offset_0,
    );

    let intermediate6 = intermediate6(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        preprocessed_seq,
        trace_1_column_12_offset_0,
    );

    let intermediate7 = intermediate7(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        preprocessed_seq,
        trace_1_column_14_offset_0,
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
    ]
}


pub fn intermediate0(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    preprocessed_seq: QM31,
    trace_1_column_0_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (preprocessed_seq)
        + (MemoryAddressToId_alpha1) * (trace_1_column_0_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate1(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    preprocessed_seq: QM31,
    trace_1_column_2_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (preprocessed_seq + m31(16).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_2_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate2(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    preprocessed_seq: QM31,
    trace_1_column_4_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (preprocessed_seq + m31(32).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_4_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate3(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    preprocessed_seq: QM31,
    trace_1_column_6_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (preprocessed_seq + m31(48).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_6_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate4(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    preprocessed_seq: QM31,
    trace_1_column_8_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (preprocessed_seq + m31(64).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_8_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate5(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    preprocessed_seq: QM31,
    trace_1_column_10_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (preprocessed_seq + m31(80).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_10_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate6(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    preprocessed_seq: QM31,
    trace_1_column_12_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (preprocessed_seq + m31(96).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_12_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate7(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    preprocessed_seq: QM31,
    trace_1_column_14_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (preprocessed_seq + m31(112).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_14_offset_0)
        - (MemoryAddressToId_z)
}

