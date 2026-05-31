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

pub const NUM_PREPROCESSED_COLUMNS: u32 = 45;

// m31_to_u32 columns
pub const M_31_TO_U_32_INPUT_ADDR_IDX: PreprocessedColumnIdx = 0;
pub const M_31_TO_U_32_OUTPUT_ADDR_IDX: PreprocessedColumnIdx = 1;
pub const M_31_TO_U_32_MULTIPLICITY_IDX: PreprocessedColumnIdx = 2;

// bitwise_xor_4_{0,1,2}
pub const BITWISE_XOR_4_0_IDX: PreprocessedColumnIdx = 3;
pub const BITWISE_XOR_4_1_IDX: PreprocessedColumnIdx = 4;
pub const BITWISE_XOR_4_2_IDX: PreprocessedColumnIdx = 5;

// bitwise_xor_7_{0,1,2}
pub const BITWISE_XOR_7_0_IDX: PreprocessedColumnIdx = 6;
pub const BITWISE_XOR_7_1_IDX: PreprocessedColumnIdx = 7;
pub const BITWISE_XOR_7_2_IDX: PreprocessedColumnIdx = 8;

// eq_in{0,1}_address (eq has log_size=15, inserted before seq_16 so stable
// sort places eq columns ahead).
pub const EQ_IN0_ADDRESS_IDX: PreprocessedColumnIdx = 9;
pub const EQ_IN1_ADDRESS_IDX: PreprocessedColumnIdx = 10;

// seq_16 (log_size=16; seq_16 follows the eq columns).
pub const SEQ_16_IDX: PreprocessedColumnIdx = 11;

// bitwise_xor_8_{0,1,2} (log_size=16, sorted after seq_16).
pub const BITWISE_XOR_8_0_IDX: PreprocessedColumnIdx = 12;
pub const BITWISE_XOR_8_1_IDX: PreprocessedColumnIdx = 13;
pub const BITWISE_XOR_8_2_IDX: PreprocessedColumnIdx = 14;

// bitwise_xor_9_{0,1,2}
pub const BITWISE_XOR_9_0_IDX: PreprocessedColumnIdx = 15;
pub const BITWISE_XOR_9_1_IDX: PreprocessedColumnIdx = 16;
pub const BITWISE_XOR_9_2_IDX: PreprocessedColumnIdx = 17;

// qm31_ops_* (log_size=19). Auto-generated components reference these without
// the `qm31_ops_` prefix (e.g. `ADD_FLAG_IDX`); the prover-side column names use the prefix.
pub const ADD_FLAG_IDX: PreprocessedColumnIdx = 18;
pub const SUB_FLAG_IDX: PreprocessedColumnIdx = 19;
pub const MUL_FLAG_IDX: PreprocessedColumnIdx = 20;
pub const POINTWISE_MUL_FLAG_IDX: PreprocessedColumnIdx = 21;
pub const OP_0_ADDR_IDX: PreprocessedColumnIdx = 22;
pub const OP_1_ADDR_IDX: PreprocessedColumnIdx = 23;
pub const DST_ADDR_IDX: PreprocessedColumnIdx = 24;
pub const QM_31_OPS_MULTIPLICITY_IDX: PreprocessedColumnIdx = 25;

// bitwise_xor_10_{0,1,2} (log_size=20) — privacy circuit names them
// `bitwise_xor_10_*` rather than `bitwise_xor_12_*`. Auto-generated component code
// references the legacy name, so we alias `BITWISE_XOR_12_*_IDX` here.
pub const BITWISE_XOR_12_0_IDX: PreprocessedColumnIdx = 26;
pub const BITWISE_XOR_12_1_IDX: PreprocessedColumnIdx = 27;
pub const BITWISE_XOR_12_2_IDX: PreprocessedColumnIdx = 28;

pub const BLAKE_G_GATE_INPUT_ADDR_A_IDX: PreprocessedColumnIdx = 29;
pub const BLAKE_G_GATE_INPUT_ADDR_B_IDX: PreprocessedColumnIdx = 30;
pub const BLAKE_G_GATE_INPUT_ADDR_C_IDX: PreprocessedColumnIdx = 31;
pub const BLAKE_G_GATE_INPUT_ADDR_D_IDX: PreprocessedColumnIdx = 32;
pub const BLAKE_G_GATE_INPUT_ADDR_F_0_IDX: PreprocessedColumnIdx = 33;
pub const BLAKE_G_GATE_INPUT_ADDR_F_1_IDX: PreprocessedColumnIdx = 34;
pub const BLAKE_G_GATE_OUTPUT_ADDR_A_IDX: PreprocessedColumnIdx = 35;
pub const BLAKE_G_GATE_OUTPUT_ADDR_B_IDX: PreprocessedColumnIdx = 36;
pub const BLAKE_G_GATE_OUTPUT_ADDR_C_IDX: PreprocessedColumnIdx = 37;
pub const BLAKE_G_GATE_OUTPUT_ADDR_D_IDX: PreprocessedColumnIdx = 38;
pub const BLAKE_G_GATE_MULTIPLICITY_IDX: PreprocessedColumnIdx = 39;

pub const TRIPLE_XOR_INPUT_ADDR_0_IDX: PreprocessedColumnIdx = 40;
pub const TRIPLE_XOR_INPUT_ADDR_1_IDX: PreprocessedColumnIdx = 41;
pub const TRIPLE_XOR_INPUT_ADDR_2_IDX: PreprocessedColumnIdx = 42;
pub const TRIPLE_XOR_OUTPUT_ADDR_IDX: PreprocessedColumnIdx = 43;
pub const TRIPLE_XOR_MULTIPLICITY_IDX: PreprocessedColumnIdx = 44;

// Make sure INVALID_COLUMN_IDX is not the ID of any column
const INVALID_IDX_CHECK: () = if NUM_PREPROCESSED_COLUMNS >= INVALID_COLUMN_IDX {
    core::panic_with_felt252('invalid idx too small')
};

/// Maps a `log_size` to the index of the corresponding `seq_<log_size>` preprocessed
/// column. Only sizes used by the privacy recursive circuit are supported.
pub fn seq_column_idx(log_size: u32) -> PreprocessedColumnIdx {
    match log_size {
        16 => SEQ_16_IDX,
        _ => panic!("unsupported seq column log_size"),
    }
}
