use stwo_constraint_framework::{PreprocessedColumnSet, PreprocessedMaskValues};
use stwo_verifier_core::ColumnSpan;
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::QM31;
pub mod generic_opcode;

pub mod memory_address_to_id;
pub mod memory_id_to_big;
pub mod range_check_vector;
pub mod ret_opcode;
pub mod verify_instruction;

/// A component is a set of trace columns of the same sizes along with a set of constraints on them.
pub trait CairoComponent<T> {
    /// Specifies the component's mask points.
    ///
    /// Preprocessed columns that the component needs should be added to `preprocessed_column_set`.
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
