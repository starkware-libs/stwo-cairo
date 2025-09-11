use core::blake::{blake2s_compress, blake2s_finalize};
use core::box::BoxTrait;
use super::MemorySection;

/// 2^31, used for encoding small felt252 values.
const MSB_U32: u32 = 0x80000000;

pub const BLAKE2S_256_INITIAL_STATE: [u32; 8] = [
    0x6B08E647, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];

/// Returns the hash of the given digest and all of the memory section ids, according to their order
/// in the memory section.
pub fn hash_memory_section_ids(section: MemorySection, digest: [u32; 8]) -> Box<[u32; 8]> {
    let [d0, d1, d2, d3, d4, d5, d6, d7] = digest;
    let mut ids = array![d0, d1, d2, d3, d4, d5, d6, d7];
    for entry in section {
        let (id, _val) = *entry;
        ids.append(id);
    }
    hash_u32s(ids.span())
}

/// Returns the hash of the given digest and all of the memory section values, according to their
/// order in the memory section.
pub fn hash_memory_section_values(mut section: MemorySection, digest: [u32; 8]) -> Box<[u32; 8]> {
    let mut state = BoxTrait::new(BLAKE2S_256_INITIAL_STATE);
    let [d0, d1, d2, d3, d4, d5, d6, d7] = digest;

    let (_id, [v0, v1, v2, v3, v4, v5, v6, v7]) = section
        .pop_front()
        .unwrap_or_else(|| {
            panic!("Empty memory section is not supported");
        });

    let mut msg = [d0, d1, d2, d3, d4, d5, d6, d7, *v0, *v1, *v2, *v3, *v4, *v5, *v6, *v7];
    let mut byte_count = 64;

    while let Some(head) = section.multi_pop_front::<2>() {
        // Append current value to the msg without its id and compress.
        state = blake2s_compress(state, byte_count, BoxTrait::new(msg));
        let [
            (_id0, [v0, v1, v2, v3, v4, v5, v6, v7]),
            (_id1, [v8, v9, v10, v11, v12, v13, v14, v15]),
        ] =
            head
            .unbox();
        msg = [v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15];
        byte_count += 64;
    }

    // Pad msg to blake hash message size.
    if let Some(head) = section.pop_front() {
        state = blake2s_compress(state, byte_count, BoxTrait::new(msg));
        let (_id, [v0, v1, v2, v3, v4, v5, v6, v7]) = *head;
        msg = [v0, v1, v2, v3, v4, v5, v6, v7, 0, 0, 0, 0, 0, 0, 0, 0];
        byte_count += 32;
    }
    assert!(section.is_empty());

    // Finalize hash.
    blake2s_finalize(state, byte_count, BoxTrait::new(msg))
}

/// Returns the hash of the memory section after encoding it, for packing purposes.
/// Note: this function ignores the ids and therefore assumes that the section is sorted.
pub fn encode_and_hash_memory_section(section: MemorySection) -> Box<[u32; 8]> {
    let mut encoded_values = array![];
    for entry in section {
        let (_id, val) = *entry;
        encode_felt_in_limbs_to_array(val, ref encoded_values);
    }
    hash_u32s(encoded_values.span())
}

/// Encodes a felt, represented by 8 u32 limbs in little-endian order, and appends the encoded
/// limbs in big-endian order to an array.
/// The encoding is done in the following way:
/// * If the felt is less than 2^63, it's encoded as the 2 least significant limbs.
/// * Otherwise, it's encoded as the 8 limbs, where the most significant limb has its MSB set
/// (Note
///   that the prime is less than 2^255 so the MSB could not be set prior to this intervention).
pub fn encode_felt_in_limbs_to_array(felt: [u32; 8], ref array: Array<u32>) {
    let [v0, v1, v2, v3, v4, v5, v6, v7] = felt;
    if v2 == 0 && v3 == 0 && v4 == 0 && v5 == 0 && v6 == 0 && v7 == 0 && v1 < MSB_U32 {
        array.append(v1);
        array.append(v0);
    } else {
        array.append(v7 + MSB_U32);
        array.append(v6);
        array.append(v5);
        array.append(v4);
        array.append(v3);
        array.append(v2);
        array.append(v1);
        array.append(v0);
    }
}

fn hash_u32s(mut section: Span<u32>) -> Box<[u32; 8]> {
    let mut state = BoxTrait::new(BLAKE2S_256_INITIAL_STATE);

    // Fill msg with the first 16 values.
    // TODO(Gali): Use `let ... else ...` when supported.
    let mut msg = if let Some(head) = section.multi_pop_front::<16>() {
        *head
    } else {
        // Append values to msg and pad to blake hash message size.
        let mut msg = array![];
        msg.append_span(section);
        let i = section.len();
        for _ in i..16 {
            msg.append(0);
        }
        return blake2s_finalize(state, byte_count: i * 4, msg: *msg.span().try_into().unwrap());
    };

    let mut byte_count = 64;
    while let Some(head) = section.multi_pop_front::<16>() {
        // Compress and re-fill msg.
        state = blake2s_compress(state, byte_count, msg);
        msg = *head;
        byte_count += 64;
    }

    if !section.is_empty() {
        // Compress, append remaining values to msg and pad to blake hash message size.
        state = blake2s_compress(state, byte_count, msg);

        let mut msg = array![];
        let i = section.len();
        msg.append_span(section);
        for _ in i..16 {
            msg.append(0);
        }
        byte_count += i * 4;
        blake2s_finalize(state, byte_count, *msg.span().try_into().unwrap())
    } else {
        blake2s_finalize(state, byte_count, msg)
    }
}
