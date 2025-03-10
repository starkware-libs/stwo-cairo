use stwo_cairo_air::{CairoProof, verify_cairo};

#[executable]
fn main(run: bool, proof: CairoProof) {
    if run {
        if let Result::Err(err) = verify_cairo(proof) {
            panic!("Verification failed: {:?}", err);
        }
    }
}
