use stwo_circuit_air::{
    CircuitProof, VerificationOutput, compute_circuit_hash, get_verification_output,
    preprocessed_root, verify_circuit,
};

#[executable]
fn main(proof: CircuitProof) -> VerificationOutput {
    let circuit_hash = compute_circuit_hash(preprocessed_root(@proof));
    let output_values = proof.claim.public_data.output_values.span();
    verify_circuit(:proof, :circuit_hash);
    get_verification_output(output_values, circuit_hash)
}
