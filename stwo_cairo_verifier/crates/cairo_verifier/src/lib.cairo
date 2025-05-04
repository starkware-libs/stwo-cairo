use core::blake::{blake2s_compress, blake2s_finalize};
use stwo_cairo_air::utils::construct_f252;
use stwo_cairo_air::{CairoProof, verify_cairo};
use verifier_core::channel_blake2s::BLAKE2S_256_INITIAL_STATE;

#[executable]
fn main(proof: CairoProof) -> Array<felt252> {
    let mut output = array![];

    let mut program = @proof.claim.public_data.public_memory.program;

    let mut state = BoxImpl::new(BLAKE2S_256_INITIAL_STATE);
    let mut byte_count = 32;
    let buffer = program.pop_front().1;
    for entry in @proof.claim.public_data.public_memory.program {
        state = blake2s_compress(state, byte_count, *buffer);
        let (_, buffer) = entry;
        byte_count += 32;
    }
    let res = blake2s_finalize(state, byte_count, *buffer.span().try_into().unwrap());

    for elm in res {
        output.append(elm);
    }

    for entry in @proof.claim.public_data.public_memory.output {
        let (_, val) = entry;
        output.append(construct_f252(BoxTrait::new(*val)));
    }

    if let Result::Err(err) = verify_cairo(proof) {
        panic!("Verification failed: {:?}", err);
    }

    output
}
