use stwo_constraint_framework::{PreprocessedColumnSet, PreprocessedMaskValues};
use stwo_verifier_core::ColumnSpan;
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::QM31;

pub mod add_ap_opcode;
pub mod add_ap_opcode_imm;
pub mod add_ap_opcode_op_1_base_fp;
pub mod add_opcode;
pub mod add_opcode_imm;
pub mod add_opcode_small;
pub mod add_opcode_small_imm;
pub mod assert_eq_opcode;
pub mod assert_eq_opcode_double_deref;
pub mod assert_eq_opcode_imm;
pub mod call_opcode;
pub mod call_opcode_op_1_base_fp;
pub mod call_opcode_rel;
pub mod generic_opcode;
pub mod jnz_opcode;
pub mod jnz_opcode_dst_base_fp;
pub mod jnz_opcode_taken;
pub mod jnz_opcode_taken_dst_base_fp;
pub mod jump_opcode;
pub mod jump_opcode_double_deref;
pub mod jump_opcode_rel;
pub mod jump_opcode_rel_imm;
pub mod memory_address_to_id;
pub mod memory_id_to_big;
pub mod mul_opcode;
pub mod mul_opcode_imm;
pub mod range_check_builtin_bits_128;
pub mod range_check_builtin_bits_96;
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
