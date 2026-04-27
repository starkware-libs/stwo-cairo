use stwo_circuit_air::{
    CircuitProof, CircuitVerifierConfig, VerificationOutput, get_verification_output,
    verify_circuit,
};

#[executable]
fn main(proof: CircuitProof, config: CircuitVerifierConfig) -> VerificationOutput {
    let verification_output = get_verification_output(proof: @proof);

    verify_circuit(:proof, config: @config);

    verification_output
}
