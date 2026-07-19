use stwo_circuit_air::{
    CircuitProof, VerificationOutput, compute_circuit_hash, get_verification_output, verify_circuit,
};
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

    // Compute the circuit hash.
    let [preprocessed_commitment, _, _, _] = commitments.unbox();
    let circuit_hash = compute_circuit_hash(
        proof.stark_proof.commitment_scheme_proof.config.fri_config.log_blowup_factor,
        preprocessed_commitment,
    );

    // Verify the circuit proof; panics on an invalid proof.
    verify_circuit(:proof, :circuit_hash);

    // Return the verification output.
    get_verification_output(:circuit_hash, :output_values)
}
