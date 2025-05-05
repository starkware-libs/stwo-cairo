use stwo_cairo_air::utils::construct_f252;
use stwo_cairo_air::{CairoProof, VerificationOutput, hash_memory_section, verify_cairo};

#[executable]
fn main(proof: CairoProof) -> VerificationOutput {
    let mut output = array![];

    // Note: the blake hash yields a 256-bit integer, the given program hash is taken modulo the
    // f252 prime to yield a felt.
    let program_hash = construct_f252(
        hash_memory_section(@proof.claim.public_data.public_memory.program),
    );

    for entry in @proof.claim.public_data.public_memory.output {
        let (_, val) = entry;
        output.append(construct_f252(BoxTrait::new(*val)));
    }

    if let Result::Err(err) = verify_cairo(proof) {
        panic!("Verification failed: {:?}", err);
    }

    VerificationOutput { program_hash, output }
}
