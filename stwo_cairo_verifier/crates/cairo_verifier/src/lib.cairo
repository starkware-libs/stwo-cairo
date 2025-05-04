use core::blake::{blake2s_compress, blake2s_finalize};
use core::box::BoxImpl;
use stwo_cairo_air::utils::construct_f252;
use stwo_cairo_air::{CairoProof, verify_cairo};

pub const BLAKE2S_256_INITIAL_STATE: [u32; 8] = [
    0x6B08E647, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];

#[executable]
fn main(proof: CairoProof) -> Array<felt252> {
    let mut output = array![];

    let mut program = proof.claim.public_data.public_memory.program.clone();

    let mut state = BoxImpl::new(BLAKE2S_256_INITIAL_STATE);
    let mut byte_count = 32;
    let (_, val) = program.pop_front().unwrap();
    let mut buffer = val;
    for entry in program {
        state = blake2s_compress(state, byte_count, buffer);
        let (_, val) = entry;
        buffer = val;
        byte_count += 32;
    }
    let res = blake2s_finalize(state, byte_count, *buffer.span().try_into().unwrap());

    output.append(construct_f252(res));

    for entry in @proof.claim.public_data.public_memory.output {
        let (_, val) = entry;
        output.append(construct_f252(BoxTrait::new(*val)));
    }

    if let Result::Err(err) = verify_cairo(proof) {
        panic!("Verification failed: {:?}", err);
    }

    output
}
