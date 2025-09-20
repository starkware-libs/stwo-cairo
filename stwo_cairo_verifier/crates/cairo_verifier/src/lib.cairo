use stwo_cairo_air::{CairoProof, VerificationOutput,  verify_cairo};

#[executable]
fn main(proof: CairoProof) -> VerificationOutput {
    let verification_output = verify_cairo(:proof);


    verification_output
}
