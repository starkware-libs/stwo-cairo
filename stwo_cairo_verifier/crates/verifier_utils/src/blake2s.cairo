use core::blake::{blake2s_compress, blake2s_finalize};
use core::box::BoxTrait;
use super::MemorySection;

/// 2^31, used for encoding small felt252 values.
const MSB_U32: u32 = 0x80000000;

pub const BLAKE2S_256_INITIAL_STATE: [u32; 8] = [
    0x6B08E647, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];

/// Returns the hash of the memory section for packing purposes.
pub fn hash_memory_section(section: MemorySection) -> Box<[u32; 8]> {
    hash_memory_section_ex(section, BoxTrait::new(BLAKE2S_256_INITIAL_STATE), 0)
}

/// Returns the hash of the memory section with the channel's digest hash for mixing purposes.
pub fn hash_memory_section_with_digest(
    mut section: MemorySection, digest_hash: Box<[u32; 8]>,
) -> Box<[u32; 8]> {
    let mut state = BoxTrait::new(BLAKE2S_256_INITIAL_STATE);
    let [d0, d1, d2, d3, d4, d5, d6, d7] = digest_hash.unbox();

    let (buffer, byte_count) = if let Some(head) = section.pop_front() {
        let (_, [v0, v1, v2, v3, v4, v5, v6, v7]) = *head;
        (BoxTrait::new([d0, d1, d2, d3, d4, d5, d6, d7, v0, v1, v2, v3, v4, v5, v6, v7]), 64)
    } else {
        (BoxTrait::new([d0, d1, d2, d3, d4, d5, d6, d7, 0, 0, 0, 0, 0, 0, 0, 0]), 32)
    };
    if section.is_empty() {
        state = blake2s_finalize(state, byte_count, buffer);
    } else {
        state = blake2s_compress(state, byte_count, buffer);
        state = hash_memory_section_ex(section, state, byte_count)
    }

    state
}

/// Returns the hash of the memory section with the given initial state.
/// Note: this function ignores the ids and therefore assumes that the section is sorted.
fn hash_memory_section_ex(
    mut section: MemorySection, state: Box<[u32; 8]>, mut byte_count: u32,
) -> Box<[u32; 8]> {
    let mut state = state;
    let (_, [v0, v1, v2, v3, v4, v5, v6, v7]) = if let Some(head) = section.pop_front() {
        *head
    } else {
        return blake2s_finalize(state, byte_count, BoxTrait::new([0; 16]));
    };

    let (_, [v8, v9, v10, v11, v12, v13, v14, v15]) = if let Some(head) = section.pop_front() {
        *head
    } else {
        byte_count += 32;
        return blake2s_finalize(
            state,
            byte_count,
            BoxTrait::new([v0, v1, v2, v3, v4, v5, v6, v7, 0, 0, 0, 0, 0, 0, 0, 0]),
        );
    };

    let mut buffer = [v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15];
    byte_count += 64;

    while let Some(head) = section.multi_pop_front::<2>() {
        // Append current value to the buffer without its id and compress.
        state = blake2s_compress(state, byte_count, BoxTrait::new(buffer));
        let [(_, [v0, v1, v2, v3, v4, v5, v6, v7]), (_, [v8, v9, v10, v11, v12, v13, v14, v15])] =
            head
            .unbox();
        buffer = [v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15];
        byte_count += 64;
    }

    // Pad buffer to blake hash message size.
    if let Some(head) = section.pop_front() {
        state = blake2s_compress(state, byte_count, BoxTrait::new(buffer));
        let (_, [v0, v1, v2, v3, v4, v5, v6, v7]) = *head;
        buffer = [v0, v1, v2, v3, v4, v5, v6, v7, 0, 0, 0, 0, 0, 0, 0, 0];
        byte_count += 32;
    }

    // Finalize hash.
    blake2s_finalize(state, byte_count, *buffer.span().try_into().unwrap())
}

/// Returns the hash of the memory section after encoding it, for packing purposes.
/// Note: this function ignores the ids and therefore assumes that the section is sorted.
pub fn encode_and_hash_memory_section(section: MemorySection) -> Box<[u32; 8]> {
    let mut encoded_values = array![];
    for entry in section {
        let (_, val) = *entry;
        encode_felt_in_limbs_to_array(val, ref encoded_values);
    }
    let mut state = BoxTrait::new(BLAKE2S_256_INITIAL_STATE);
    hash_u32s(encoded_values.span(), state)
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

fn hash_u32s(section: Span<u32>, state: Box<[u32; 8]>) -> Box<[u32; 8]> {
    let mut state = state;
    let mut section = section;

    // Fill the buffer with the first 16 values.
    // TODO(Gali): Use `let ... else ...` when supported.
    let mut buffer = if let Some(head) = section.multi_pop_front::<16>() {
        head.unbox()
    } else {
        // Append values to the buffer and pad to blake hash message size.
        let mut buffer = array![];
        let mut i = 0;
        buffer.append_span(section);
        i += section.len();
        for _ in i..16 {
            buffer.append(0);
        }
        return blake2s_finalize(state, i * 4, *buffer.span().try_into().unwrap());
    };
    let mut byte_count = 64;
    while let Some(head) = section.multi_pop_front::<16>() {
        // Compress and re-fill the buffer.
        state = blake2s_compress(state, byte_count, BoxTrait::new(buffer));
        buffer = head.unbox();
        byte_count += 64;
    }

    if !section.is_empty() {
        // Compress, append remaining values to the buffer and pad to blake hash message size.
        state = blake2s_compress(state, byte_count, BoxTrait::new(buffer));

        let mut buffer = array![];
        let i = section.len();
        buffer.append_span(section);
        for _ in i..16 {
            buffer.append(0);
        }
        byte_count += i * 4;
        blake2s_finalize(state, byte_count, *buffer.span().try_into().unwrap())
    } else {
        blake2s_finalize(state, byte_count, BoxTrait::new(buffer))
    }
}
