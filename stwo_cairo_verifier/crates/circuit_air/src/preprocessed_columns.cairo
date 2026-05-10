// Preprocessed-column index constants for the recursive circuit.
//
// Source of truth: the actual preprocessed trace produced by `PreprocessedCircuit::
// preprocess_circuit` on the privacy recursive verification circuit. Dump captured by
// `proving-utils/crates/circuit_verifier_e2e/src/main.rs` against the privacy fixture.
//
// The auto-generated component files reference some IDX symbols whose names differ from
// the prover-side column names (e.g. `ADD_FLAG_IDX` corresponds to the prover-side column
// `qm31_ops_add_flag`). These are aliased here so the auto-generated code keeps working
// without rewriting every call site.

use stwo_constraint_framework::{INVALID_COLUMN_IDX, PreprocessedColumnIdx};

pub const NUM_PREPROCESSED_COLUMNS: u32 = 63;

// 0..15: blake_sigma_0 .. blake_sigma_15
pub const BLAKE_SIGMA_0_IDX: PreprocessedColumnIdx = 0;
pub const BLAKE_SIGMA_1_IDX: PreprocessedColumnIdx = 1;
pub const BLAKE_SIGMA_2_IDX: PreprocessedColumnIdx = 2;
pub const BLAKE_SIGMA_3_IDX: PreprocessedColumnIdx = 3;
pub const BLAKE_SIGMA_4_IDX: PreprocessedColumnIdx = 4;
pub const BLAKE_SIGMA_5_IDX: PreprocessedColumnIdx = 5;
pub const BLAKE_SIGMA_6_IDX: PreprocessedColumnIdx = 6;
pub const BLAKE_SIGMA_7_IDX: PreprocessedColumnIdx = 7;
pub const BLAKE_SIGMA_8_IDX: PreprocessedColumnIdx = 8;
pub const BLAKE_SIGMA_9_IDX: PreprocessedColumnIdx = 9;
pub const BLAKE_SIGMA_10_IDX: PreprocessedColumnIdx = 10;
pub const BLAKE_SIGMA_11_IDX: PreprocessedColumnIdx = 11;
pub const BLAKE_SIGMA_12_IDX: PreprocessedColumnIdx = 12;
pub const BLAKE_SIGMA_13_IDX: PreprocessedColumnIdx = 13;
pub const BLAKE_SIGMA_14_IDX: PreprocessedColumnIdx = 14;
pub const BLAKE_SIGMA_15_IDX: PreprocessedColumnIdx = 15;

// 16..18: m31_to_u32 columns
pub const M_31_TO_U_32_INPUT_ADDR_IDX: PreprocessedColumnIdx = 16;
pub const M_31_TO_U_32_OUTPUT_ADDR_IDX: PreprocessedColumnIdx = 17;
pub const M_31_TO_U_32_MULTIPLICITY_IDX: PreprocessedColumnIdx = 18;

// 19: seq_4
pub const SEQ_4_IDX: PreprocessedColumnIdx = 19;

// 20..22: bitwise_xor_4_{0,1,2}
pub const BITWISE_XOR_4_0_IDX: PreprocessedColumnIdx = 20;
pub const BITWISE_XOR_4_1_IDX: PreprocessedColumnIdx = 21;
pub const BITWISE_XOR_4_2_IDX: PreprocessedColumnIdx = 22;

// 23: final_state_addr
pub const FINAL_STATE_ADDR_IDX: PreprocessedColumnIdx = 23;

// 24..27: blake_output{0,1}_{addr,mults}
pub const BLAKE_OUTPUT_0_ADDR_IDX: PreprocessedColumnIdx = 24;
pub const BLAKE_OUTPUT_1_ADDR_IDX: PreprocessedColumnIdx = 25;
pub const BLAKE_OUTPUT_0_MULTS_IDX: PreprocessedColumnIdx = 26;
pub const BLAKE_OUTPUT_1_MULTS_IDX: PreprocessedColumnIdx = 27;

// 28,29: t0, t1
pub const T_0_IDX: PreprocessedColumnIdx = 28;
pub const T_1_IDX: PreprocessedColumnIdx = 29;

// 30: finalize_flag
pub const FINALIZE_FLAG_IDX: PreprocessedColumnIdx = 30;

// 31,32: state_{before,after}_addr
pub const STATE_BEFORE_ADDR_IDX: PreprocessedColumnIdx = 31;
pub const STATE_AFTER_ADDR_IDX: PreprocessedColumnIdx = 32;

// 33..36: message{0..3}_addr
pub const MESSAGE_0_ADDR_IDX: PreprocessedColumnIdx = 33;
pub const MESSAGE_1_ADDR_IDX: PreprocessedColumnIdx = 34;
pub const MESSAGE_2_ADDR_IDX: PreprocessedColumnIdx = 35;
pub const MESSAGE_3_ADDR_IDX: PreprocessedColumnIdx = 36;

// 37: compress_enabler
pub const COMPRESS_ENABLER_IDX: PreprocessedColumnIdx = 37;

// 38: seq_14
pub const SEQ_14_IDX: PreprocessedColumnIdx = 38;

// 39..41: bitwise_xor_7_{0,1,2}
pub const BITWISE_XOR_7_0_IDX: PreprocessedColumnIdx = 39;
pub const BITWISE_XOR_7_1_IDX: PreprocessedColumnIdx = 40;
pub const BITWISE_XOR_7_2_IDX: PreprocessedColumnIdx = 41;

// 42,43: eq_in{0,1}_address (eq has log_size=15, inserted before seq_15/16 so stable
// sort places eq columns ahead).
pub const EQ_IN0_ADDRESS_IDX: PreprocessedColumnIdx = 42;
pub const EQ_IN1_ADDRESS_IDX: PreprocessedColumnIdx = 43;

// 44,45: seq_15, seq_16 (log_size=15 and 16; seq_15 follows the eq columns, also log_size=15).
pub const SEQ_15_IDX: PreprocessedColumnIdx = 44;
pub const SEQ_16_IDX: PreprocessedColumnIdx = 45;

// 46..48: bitwise_xor_8_{0,1,2} (log_size=16, sorted after seq_15 at 44 and seq_16 at 45).
pub const BITWISE_XOR_8_0_IDX: PreprocessedColumnIdx = 46;
pub const BITWISE_XOR_8_1_IDX: PreprocessedColumnIdx = 47;
pub const BITWISE_XOR_8_2_IDX: PreprocessedColumnIdx = 48;

// 49..51: bitwise_xor_9_{0,1,2}
pub const BITWISE_XOR_9_0_IDX: PreprocessedColumnIdx = 49;
pub const BITWISE_XOR_9_1_IDX: PreprocessedColumnIdx = 50;
pub const BITWISE_XOR_9_2_IDX: PreprocessedColumnIdx = 51;

// 52..59: qm31_ops_* (log_size=19). Auto-generated components reference these without
// the `qm31_ops_` prefix (e.g. `ADD_FLAG_IDX`); the prover-side column names use the prefix.
pub const ADD_FLAG_IDX: PreprocessedColumnIdx = 52;
pub const SUB_FLAG_IDX: PreprocessedColumnIdx = 53;
pub const MUL_FLAG_IDX: PreprocessedColumnIdx = 54;
pub const POINTWISE_MUL_FLAG_IDX: PreprocessedColumnIdx = 55;
pub const OP_0_ADDR_IDX: PreprocessedColumnIdx = 56;
pub const OP_1_ADDR_IDX: PreprocessedColumnIdx = 57;
pub const DST_ADDR_IDX: PreprocessedColumnIdx = 58;
pub const QM_31_OPS_MULTIPLICITY_IDX: PreprocessedColumnIdx = 59;

// 60..62: bitwise_xor_10_{0,1,2} (log_size=20) — privacy circuit names them
// `bitwise_xor_10_*` rather than `bitwise_xor_12_*`. Auto-generated component code
// references the legacy name, so we alias `BITWISE_XOR_12_*_IDX` here.
pub const BITWISE_XOR_12_0_IDX: PreprocessedColumnIdx = 60;
pub const BITWISE_XOR_12_1_IDX: PreprocessedColumnIdx = 61;
pub const BITWISE_XOR_12_2_IDX: PreprocessedColumnIdx = 62;

pub const BLAKE_G_GATE_INPUT_ADDR_A_IDX: PreprocessedColumnIdx = INVALID_COLUMN_IDX;
pub const BLAKE_G_GATE_INPUT_ADDR_B_IDX: PreprocessedColumnIdx = INVALID_COLUMN_IDX;
pub const BLAKE_G_GATE_INPUT_ADDR_C_IDX: PreprocessedColumnIdx = INVALID_COLUMN_IDX;
pub const BLAKE_G_GATE_INPUT_ADDR_D_IDX: PreprocessedColumnIdx = INVALID_COLUMN_IDX;
pub const BLAKE_G_GATE_INPUT_ADDR_F_0_IDX: PreprocessedColumnIdx = INVALID_COLUMN_IDX;
pub const BLAKE_G_GATE_INPUT_ADDR_F_1_IDX: PreprocessedColumnIdx = INVALID_COLUMN_IDX;
pub const BLAKE_G_GATE_OUTPUT_ADDR_A_IDX: PreprocessedColumnIdx = INVALID_COLUMN_IDX;
pub const BLAKE_G_GATE_OUTPUT_ADDR_B_IDX: PreprocessedColumnIdx = INVALID_COLUMN_IDX;
pub const BLAKE_G_GATE_OUTPUT_ADDR_C_IDX: PreprocessedColumnIdx = INVALID_COLUMN_IDX;
pub const BLAKE_G_GATE_OUTPUT_ADDR_D_IDX: PreprocessedColumnIdx = INVALID_COLUMN_IDX;
pub const BLAKE_G_GATE_MULTIPLICITY_IDX: PreprocessedColumnIdx = INVALID_COLUMN_IDX;

pub const TRIPLE_XOR_INPUT_ADDR_0_IDX: PreprocessedColumnIdx = INVALID_COLUMN_IDX;
pub const TRIPLE_XOR_INPUT_ADDR_1_IDX: PreprocessedColumnIdx = INVALID_COLUMN_IDX;
pub const TRIPLE_XOR_INPUT_ADDR_2_IDX: PreprocessedColumnIdx = INVALID_COLUMN_IDX;
pub const TRIPLE_XOR_OUTPUT_ADDR_IDX: PreprocessedColumnIdx = INVALID_COLUMN_IDX;
pub const TRIPLE_XOR_MULTIPLICITY_IDX: PreprocessedColumnIdx = INVALID_COLUMN_IDX;

// Make sure INVALID_COLUMN_IDX is not the ID of any column
const INVALID_IDX_CHECK: () = if NUM_PREPROCESSED_COLUMNS >= INVALID_COLUMN_IDX {
    core::panic_with_felt252('invalid idx too small')
};

/// Maps a `log_size` to the index of the corresponding `seq_<log_size>` preprocessed
/// column. Only sizes used by the privacy recursive circuit are supported.
pub fn seq_column_idx(log_size: u32) -> PreprocessedColumnIdx {
    match log_size {
        4 => SEQ_4_IDX,
        14 => SEQ_14_IDX,
        15 => SEQ_15_IDX,
        16 => SEQ_16_IDX,
        _ => panic!("unsupported seq column log_size"),
    }
}
