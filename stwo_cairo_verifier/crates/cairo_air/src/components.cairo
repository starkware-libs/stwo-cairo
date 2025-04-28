use stwo_constraint_framework::{PreprocessedColumnSet, PreprocessedMaskValues};
use stwo_verifier_core::ColumnSpan;
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::QM31;

pub mod add_ap_opcode;
pub mod add_mod_builtin;
pub mod add_opcode;
pub mod add_opcode_small;
pub mod assert_eq_opcode;
pub mod assert_eq_opcode_double_deref;
pub mod assert_eq_opcode_imm;
pub mod bitwise_builtin;
pub mod blake_compress_opcode;
pub mod blake_g;
pub mod blake_round;
pub mod blake_round_sigma;
pub mod call_opcode;
pub mod call_opcode_op_1_base_fp;
pub mod call_opcode_rel;
pub mod cube_252;
pub mod generic_opcode;
pub mod jnz_opcode;
pub mod jnz_opcode_taken;
pub mod jump_opcode;
pub mod jump_opcode_double_deref;
pub mod jump_opcode_rel;
pub mod jump_opcode_rel_imm;
pub mod memory_address_to_id;
pub mod memory_id_to_big;
pub mod mul_mod_builtin;
pub mod mul_opcode;
pub mod mul_opcode_small;
pub mod partial_ec_mul;
pub mod pedersen_builtin;
pub mod pedersen_points_table;
pub mod poseidon_3_partial_rounds_chain;
pub mod poseidon_builtin;
pub mod poseidon_full_round_chain;
pub mod poseidon_round_keys;
pub mod qm_31_add_mul_opcode;
pub mod range_check_11;
pub mod range_check_12;
pub mod range_check_18;
pub mod range_check_19;
pub mod range_check_3_3_3_3_3;
pub mod range_check_3_6;
pub mod range_check_3_6_6_3;
pub mod range_check_4_3;
pub mod range_check_4_4;
pub mod range_check_4_4_4_4;
pub mod range_check_5_4;
pub mod range_check_6;
pub mod range_check_7_2_5;
pub mod range_check_8;
pub mod range_check_9_9;
pub mod range_check_builtin_bits_128;
pub mod range_check_builtin_bits_96;
pub mod range_check_felt_252_width_27;
pub mod range_check_vector;
pub mod ret_opcode;
pub mod subroutines;
pub mod triple_xor_32;
pub mod verify_bitwise_xor_12;
pub mod verify_bitwise_xor_4;
pub mod verify_bitwise_xor_7;
pub mod verify_bitwise_xor_8;
pub mod verify_bitwise_xor_9;
pub mod verify_instruction;

pub const VERIFY_BITWISE_XOR_12_LOG_SIZE: u32 = 20;
pub const VERIFY_BITWISE_XOR_9_LOG_SIZE: u32 = 18;
pub const VERIFY_BITWISE_XOR_8_LOG_SIZE: u32 = 16;
pub const VERIFY_BITWISE_XOR_7_LOG_SIZE: u32 = 14;
pub const VERIFY_BITWISE_XOR_4_LOG_SIZE: u32 = 8;

pub const RANGE_CHECK_3_3_3_3_3_LOG_SIZE: u32 = 15;
pub const RANGE_CHECK_3_6_6_3_LOG_SIZE: u32 = 18;
pub const RANGE_CHECK_3_6_LOG_SIZE: u32 = 9;
pub const RANGE_CHECK_4_3_LOG_SIZE: u32 = 7;
pub const RANGE_CHECK_4_4_4_4_LOG_SIZE: u32 = 16;
pub const RANGE_CHECK_4_4_LOG_SIZE: u32 = 8;
pub const RANGE_CHECK_5_4_LOG_SIZE: u32 = 9;
pub const RANGE_CHECK_6_LOG_SIZE: u32 = 6;
pub const RANGE_CHECK_7_2_5_LOG_SIZE: u32 = 14;
pub const RANGE_CHECK_8_LOG_SIZE: u32 = 8;
pub const RANGE_CHECK_9_9_LOG_SIZE: u32 = 18;
pub const RANGE_CHECK_11_LOG_SIZE: u32 = 11;
pub const RANGE_CHECK_12_LOG_SIZE: u32 = 12;
pub const RANGE_CHECK_18_LOG_SIZE: u32 = 18;
pub const RANGE_CHECK_19_LOG_SIZE: u32 = 19;

pub const POSEIDON_ROUND_KEYS_LOG_SIZE: u32 = 6;
pub const PEDERSEN_POINTS_TABLE_LOG_SIZE: u32 = 23;
pub const BLAKE_ROUND_SIGMA_LOG_SIZE: u32 = 4;

pub const OPCODES_RELATION_SIZE: u32 = 3;

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
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    );
}
