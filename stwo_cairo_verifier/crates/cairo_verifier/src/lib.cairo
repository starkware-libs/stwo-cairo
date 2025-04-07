use stwo_cairo_air::utils::reconstitute_f252;
use stwo_cairo_air::{CairoProof, verify_cairo};

#[executable]
fn main(proof: CairoProof) -> Array<felt252> {
    let mut output = array![];

    for entry in @proof.claim.public_data.public_memory.output {
        let (_, val) = entry;
        output.append(reconstitute_f252(*val));
    }

    if let Result::Err(err) = verify_cairo(proof) {
        panic!("Verification failed: {:?}", err);
    }

    output
}
