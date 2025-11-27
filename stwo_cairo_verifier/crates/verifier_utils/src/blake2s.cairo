use core::blake::{blake2s_compress, blake2s_finalize};
use core::box::BoxTrait;
use super::MemorySection;

/// 2^31, used for encoding small felt252 values.
const MSB_U32: u32 = 0x80000000;

pub const BLAKE2S_256_INITIAL_STATE: [u32; 8] = [
    0x6B08E647, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];

const LEAF_INITIAL_STATE: [u32; 8] = [
    0x6510b1f7, 0xfd531f42, 0xcff75ec3, 0x382935d0, 0xab15dbf2, 0x950eb564, 0xe8e92866, 0x28047aca,
];

const NODE_INITIAL_STATE: [u32; 8] = [
    0xe5cf8926, 0x841cea30, 0x7b4acada, 0xfc5d8d28, 0xfc6ef857, 0xb29da528, 0xc0d319c7, 0x8ae795c8,
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

    let Some(head) = section.pop_front() else {
        return blake2s_finalize(
            state,
            byte_count: 32,
            msg: BoxTrait::new([d0, d1, d2, d3, d4, d5, d6, d7, 0, 0, 0, 0, 0, 0, 0, 0]),
        );
    };

    let (_id, [v0, v1, v2, v3, v4, v5, v6, v7]) = *head;
    let mut msg = BoxTrait::new([d0, d1, d2, d3, d4, d5, d6, d7, v0, v1, v2, v3, v4, v5, v6, v7]);
    let mut byte_count = 64;

    while let Some(head) = section.multi_pop_front::<2>() {
        // Append current value to the msg without its id and compress.
        state = blake2s_compress(state, byte_count, msg);
        let [
            (_id0, [v0, v1, v2, v3, v4, v5, v6, v7]),
            (_id1, [v8, v9, v10, v11, v12, v13, v14, v15]),
        ] =
            head
            .unbox();
        msg = BoxTrait::new([v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15]);
        byte_count += 64;
    }

    // Pad msg to blake hash message size.
    if let Some(head) = section.pop_front() {
        state = blake2s_compress(state, byte_count, msg);
        let (_id, [v0, v1, v2, v3, v4, v5, v6, v7]) = *head;
        msg = BoxTrait::new([v0, v1, v2, v3, v4, v5, v6, v7, 0, 0, 0, 0, 0, 0, 0, 0]);
        byte_count += 32;
    }
    assert!(section.is_empty());

    // Finalize hash.
    blake2s_finalize(state, byte_count, msg)
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

fn hash_u32s(mut values: Span<u32>) -> Box<[u32; 8]> {
    let mut state = BoxTrait::new(BLAKE2S_256_INITIAL_STATE);
    let mut byte_count = 0;
    if let Some(mut msg) = values.multi_pop_front::<16>() {
        byte_count += 64;
        while let Some(head) = values.multi_pop_front::<16>() {
            // Compress and re-fill msg.
            state = blake2s_compress(state, byte_count, *msg);
            msg = head;
            byte_count += 64;
        }

        // Here `msg` is the last full 16-element block, if there are no remaining values, we can
        // finalize the hash and return the result.
        if values.is_empty() {
            return blake2s_finalize(state, byte_count, *msg);
        }

        // Otherwise, update the state and handle the remaining values.
        state = blake2s_compress(state, byte_count, *msg);
    }

    /// pad the remaining values to a full 16-element block and hash them as a final block.
    let mut msg = array![];
    let i = values.len();
    msg.append_span(values);
    for _ in i..16 {
        msg.append(0);
    }
    byte_count += i * 4;
    blake2s_finalize(state, byte_count, *msg.span().try_into().unwrap())
}
