use stwo_circuit_air::{CircuitProof, VerificationOutput, get_verification_output, verify_circuit};

mod privacy_consts;

#[executable]
fn main(proof: CircuitProof) -> VerificationOutput {
    // The verifier-config constants are hardcoded for the privacy/recursion circuit
    // topology (see `privacy_consts.cairo`).
    let config = privacy_consts::privacy_config();

    let verification_output = get_verification_output(proof: @proof);
    verify_circuit(:proof, config: @config);
    verification_output
}
