use stwo_circuit_air::{
    CircuitProof, VerificationOutput, compute_circuit_hash, get_verification_output,
    preprocessed_root, verify_circuit,
};

#[executable]
fn main(proof: CircuitProof) -> VerificationOutput {
    let circuit_hash = compute_circuit_hash(preprocessed_root(@proof));
    let verification_output = get_verification_output(@proof);
    verify_circuit(:proof, :circuit_hash);
    verification_output
}
