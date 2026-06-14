use stwo_circuit_air::{CircuitProof, VerificationOutput, get_verification_output, verify_circuit};

#[executable]
fn main(proof: CircuitProof) -> VerificationOutput {
    let verification_output = get_verification_output(@proof);
    verify_circuit(:proof);
    verification_output
}
