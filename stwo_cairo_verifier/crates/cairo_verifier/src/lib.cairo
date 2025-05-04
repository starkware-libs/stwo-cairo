use stwo_cairo_air::utils::{blake_segment, construct_f252};
use stwo_cairo_air::{CairoProof, verify_cairo};

#[derive(Drop, Serde)]
struct VerificationOutput {
    pub program_hash: felt252,
    pub output: Array<felt252>,
}

#[executable]
fn main(proof: CairoProof) -> VerificationOutput {
    let mut output = array![];

    let program_hash = construct_f252(
        blake_segment(@proof.claim.public_data.public_memory.program),
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
