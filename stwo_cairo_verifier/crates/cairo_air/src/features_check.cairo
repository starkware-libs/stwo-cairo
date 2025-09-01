//! The verifier exposes four features:
//! - `qm31_opcode`: Enables the QM31 opcode for Cairo verification, only supported by the Stwo
//! prover. This feature should not be used in verifications aimed to be proven by Stone.
//! - `poseidon252_verifier`: Whether to use the Poseidon252 hash or Blake hash. Blake is only
//! supported by the Stwo prover and should not be used in verifications aimed to be proven by
//! Stone.
//! - `blake_outputs_packing`: Enables outputs packing with Blake hash. Optional feature,
//! currently only supported when using the Blake hash.
//! - `poseidon_outputs_packing`: Enables outputs packing with Poseidon hash. Optional feature,
//! supported by both the verifiers, regardless of their hash function. When this feature is enabled
//! in the poseidon hash verifier, the verifier will also include the poseidon builtin.
//! Notice that `qm31_opcode` and `poseidon252_verifier` are mutually exclusive features, and thus
//! one of them is redundant, however, they mean different things, and it is possible that in the
//! future they won't be mutually exclusive.
//! This module checks that the given features set is valid.
#[cfg(not(or(feature: "qm31_opcode", feature: "poseidon252_verifier")))]
compile_error!("Either 'qm31_opcode' or 'poseidon252_verifier' feature must be enabled");
#[cfg(and(feature: "qm31_opcode", feature: "poseidon252_verifier"))]
compile_error!("'qm31_opcode' and 'poseidon252_verifier' can not be enabled at the same time");
#[cfg(and(feature: "blake_outputs_packing", feature: "poseidon252_verifier"))]
compile_error!(
    "'blake_outputs_packing' and 'poseidon252_verifier' can not be enabled at the same time",
);
#[cfg(and(feature: "blake_outputs_packing", feature: "poseidon_outputs_packing"))]
compile_error!(
    "'blake_outputs_packing' and 'poseidon_outputs_packing' can not be enabled at the same time",
);
