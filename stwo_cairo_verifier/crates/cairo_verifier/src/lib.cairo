use stwo_cairo_air::{CairoProof, verify_cairo};

#[executable]
fn main(proof: CairoProof) {
    if let Result::Err(err) = verify_cairo(proof) {
        panic!("Verification failed: {:?}", err);
    }
}
