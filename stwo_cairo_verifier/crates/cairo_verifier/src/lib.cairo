use stwo_cairo_air::VerificationOutput;
use stwo_cairo_air::utils::construct_f252;
use stwo_cairo_air::{CairoProof, verify_cairo};

#[executable]
fn main(proof: CairoProof) -> VerificationOutput {
    let mut output = array![];

    for entry in @proof.claim.public_data.public_memory.output {
        let (_, val) = entry;
        output.append(construct_f252(BoxTrait::new(*val)));
    }

    if let Result::Err(err) = verify_cairo(proof) {
        panic!("Verification failed: {:?}", err);
    }

    // TODO(alont): replace this with the computation of the real program hash of the bootloader.
    VerificationOutput { program_hash: 0x12345, output }
}
