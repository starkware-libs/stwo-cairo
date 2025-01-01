use stwo_constraint_framework::PreprocessedMaskValues;
use stwo_verifier_core::ColumnSpan;
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::QM31;

pub mod addr_to_id;
pub mod genericopcode;
pub mod id_to_f252;
pub mod jump_t_t_f_opcode;
pub mod range_check;
pub mod ret_opcode;
pub mod verify_instruction;

pub trait CairoComponent<T> {
    fn mask_points(
        self: @T,
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
