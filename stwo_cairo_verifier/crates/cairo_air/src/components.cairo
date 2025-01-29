use stwo_constraint_framework::{PreprocessedColumnSet, PreprocessedMaskValues};
use stwo_verifier_core::ColumnSpan;
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::QM31;

pub mod addr_to_id;
pub mod genericopcode;
pub mod id_to_f252;
pub mod range_check;
pub mod ret_opcode;
pub mod verify_instruction;

/// A component is a set of trace columns of the same sizes along with a set of constraints on them.
pub trait CairoComponent<T> {
    /// Specifies the component's mask points.
    ///
    /// Preprocessed columns that the component needs should be added to `preprocessed_column_set`.
    // TODO(andrew): In each implementation rename `trace_gen` to `trace_step` and move the variable
    // to the mask_points function within constraints.cairo.
    fn mask_points(
        self: @T,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    );

    fn max_constraint_log_degree_bound(self: @T) -> u32;

    fn evaluate_constraints_at_point(
        self: @T,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Array<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Array<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    );
}
