use stwo_cairo_air::{CairoProof, VerificationOutput, get_verification_output, verify_cairo};

#[executable]
fn main(proof: CairoProof) -> VerificationOutput {
    let verification_output = get_verification_output(proof: @proof);

    verify_cairo(:proof);

    verification_output
}
