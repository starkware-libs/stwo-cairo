use stwo_circuit_air::{CircuitProof, VerificationOutput, get_verification_output, verify_circuit};

#[executable]
fn main(proof: CircuitProof) -> VerificationOutput {
    // The verifier constants are hardcoded for the privacy/recursion circuit topology
    // (see `stwo_circuit_air::privacy_consts`).
    let verification_output = get_verification_output(proof: @proof);
    verify_circuit(:proof);
    verification_output
}
