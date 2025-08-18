//! The verifier exposes three features:
//! - `qm31_opcode`: Enables the QM31 opcode for Cairo verification, only supported by the Stwo
//! prover. This feature should not be used in verifications aimed to be proven by Stone.
//! - `poseidon252_verifier`: Whether to use the Poseidon252 hash or Blake hash. Blake is only
//! supported by the Stwo prover and should not be used in verifications aimed to be proven by
//! Stone.
//! - `outputs_packing`: Enables outputs packing. Optional feature, currently only supported when
//! using the Blake hash.
//! Notice that `qm31_opcode` and `poseidon252_verifier` are mutually exclusive features, and thus
//! one of them is redundant, however, they mean different things, and it is possible that in the
//! future they won't be mutually exclusive.
//! This module checks that the given features set is valid.
#[cfg(not(or(feature: "qm31_opcode", feature: "poseidon252_verifier")))]
compile_error!("Either 'qm31_opcode' or 'poseidon252_verifier' feature must be enabled");
#[cfg(and(feature: "qm31_opcode", feature: "poseidon252_verifier"))]
compile_error!("'qm31_opcode' and 'poseidon252_verifier' can not be enabled at the same time");
#[cfg(and(feature: "outputs_packing", feature: "poseidon252_verifier"))]
compile_error!("'outputs_packing' and 'poseidon252_verifier' can not be enabled at the same time");
