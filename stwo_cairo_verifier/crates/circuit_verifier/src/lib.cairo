use stwo_circuit_air::{CircuitProof, VerificationOutput, get_verification_output, verify_circuit};
use stwo_verifier_core::Hash;

#[executable]
fn main(proof: CircuitProof) -> VerificationOutput {
    /// Extract the commitments and output values before they are consumed by 'verify_circuit'.
    let commitments: @Box<[Hash; 4]> = proof
        .stark_proof
        .commitment_scheme_proof
        .commitments
        .try_into()
        .unwrap();
    let output_values = proof.claim.public_data.output_values.span();
    /// Verify the circuit.
    verify_circuit(:proof);
    /// Get the verification output.
    get_verification_output(commitments, output_values)
}
