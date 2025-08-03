#[cfg(not(feature: "poseidon252_verifier"))]
use core::blake::{blake2s_compress, blake2s_finalize};
use core::box::BoxImpl;
#[cfg(feature: "poseidon252_verifier")]
use core::poseidon::poseidon_hash_span;

// TODO(alonf): Change this into a struct. Remove Pub prefix.
// (id, value)
pub type PubMemoryValue = (u32, [u32; 8]);

// TODO(alonf): Change this into a struct. Remove Pub prefix.
// (address, id, value)
pub type PubMemoryEntry = (u32, u32, [u32; 8]);

/// A contiguous memory section.
pub type MemorySection = Array<PubMemoryValue>;

/// 2^31, used for encoding small felt252 values.
const MSB_U32: u32 = 0x80000000;

#[cfg(not(feature: "poseidon252_verifier"))]
pub const BLAKE2S_256_INITIAL_STATE: [u32; 8] = [
    0x6B08E647, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];

/// Returns the hash of the memory section for packing purposes.
#[cfg(not(feature: "poseidon252_verifier"))]
pub fn hash_memory_section(section: @MemorySection) -> Box<[u32; 8]> {
    hash_memory_section_ex(section.span(), BoxTrait::new(BLAKE2S_256_INITIAL_STATE))
}

/// Returns the hash of the memory section with the channel's digest hash for mixing purposes.
#[cfg(not(feature: "poseidon252_verifier"))]
pub fn hash_memory_section_with_digest(
    section: @MemorySection, digest_hash: Box<[u32; 8]>,
) -> Box<[u32; 8]> {
    let mut state = BoxTrait::new(BLAKE2S_256_INITIAL_STATE);
    let mut section = section.span();
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
        state = hash_memory_section_ex(section, state)
    }

    state
}

/// Returns the hash of the memory section with the given initial state.
/// Note: this function ignores the ids and therefore assumes that the section is sorted.
#[cfg(not(feature: "poseidon252_verifier"))]
fn hash_memory_section_ex(section: Span<PubMemoryValue>, state: Box<[u32; 8]>) -> Box<[u32; 8]> {
    let mut state = state;
    let mut section = section;
    let (_, [v0, v1, v2, v3, v4, v5, v6, v7]) = if let Some(head) = section.pop_front() {
        *head
    } else {
        return blake2s_finalize(state, 0, BoxImpl::new([0; 16]));
    };

    let (_, [v8, v9, v10, v11, v12, v13, v14, v15]) = if let Some(head) = section.pop_front() {
        *head
    } else {
        return blake2s_finalize(
            state, 32, BoxImpl::new([v0, v1, v2, v3, v4, v5, v6, v7, 0, 0, 0, 0, 0, 0, 0, 0]),
        );
    };

    let mut buffer = [v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15];
    let mut byte_count = 64;

    while let Some(head) = section.multi_pop_front::<2>() {
        // Append current value to the buffer without its id and compress.
        state = blake2s_compress(state, byte_count, BoxImpl::new(buffer));
        let [(_, [v0, v1, v2, v3, v4, v5, v6, v7]), (_, [v8, v9, v10, v11, v12, v13, v14, v15])] =
            head
            .unbox();
        buffer = [v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15];
        byte_count += 64;
    }

    // Pad buffer to blake hash message size.
    if let Some(head) = section.pop_front() {
        state = blake2s_compress(state, byte_count, BoxImpl::new(buffer));
        let (_, [v0, v1, v2, v3, v4, v5, v6, v7]) = *head;
        buffer = [v0, v1, v2, v3, v4, v5, v6, v7, 0, 0, 0, 0, 0, 0, 0, 0];
        byte_count += 32;
    }

    // Finalize hash.
    blake2s_finalize(state, byte_count, *buffer.span().try_into().unwrap())
}

/// Returns the hash of the memory section after encoding it, for packing purposes.
/// Note: this function ignores the ids and therefore assumes that the section is sorted.
#[cfg(not(feature: "poseidon252_verifier"))]
pub fn encode_and_hash_memory_section(section: @MemorySection) -> Box<[u32; 8]> {
    let mut encoded_values = array![];
    for entry in section {
        let (_, val) = *entry;
        encode_felt_in_limbs_to_array(val, ref encoded_values);
    }
    let mut state = BoxTrait::new(BLAKE2S_256_INITIAL_STATE);
    hash_u32s(encoded_values.span(), state)
}

/// Encodes a felt, represented by 8 u32 limbs in little-endian order, and appends the encoded limbs
/// in big-endian order to an array.
/// The encoding is done in the following way:
/// * If the felt is less than 2^63, it's encoded as the 2 least significant limbs.
/// * Otherwise, it's encoded as the 8 limbs, where the most significant limb has its MSB set (Note
///   that the prime is less than 2^255 so the MSB could not be set prior to this intervention).
#[cfg(not(feature: "poseidon252_verifier"))]
fn encode_felt_in_limbs_to_array(felt: [u32; 8], ref array: Array<u32>) {
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

#[cfg(not(feature: "poseidon252_verifier"))]
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
        state = blake2s_compress(state, byte_count, BoxImpl::new(buffer));
        buffer = head.unbox();
        byte_count += 64;
    }

    if !section.is_empty() {
        // Compress, append remaining values to the buffer and pad to blake hash message size.
        state = blake2s_compress(state, byte_count, BoxImpl::new(buffer));

        let mut buffer = array![];
        let i = section.len();
        buffer.append_span(section);
        for _ in i..16 {
            buffer.append(0);
        }
        byte_count += i * 4;
        blake2s_finalize(state, byte_count, *buffer.span().try_into().unwrap())
    } else {
        blake2s_finalize(state, byte_count, BoxImpl::new(buffer))
    }
}

/// Returns the hash of the memory section, for packing purposes.
/// Note: this function ignores the ids and therefore assumes that the section is sorted.
/// Note: no encoding is done at the moment.
// TODO(Gali): There is no encoding done at the moment with poseidon, so this function is equivalent to `hash_memory_section`.
#[cfg(feature: "poseidon252_verifier")]
pub fn encode_and_hash_memory_section(section: @MemorySection) -> Box<[u32; 8]> {
    hash_memory_section(section)
}

/// Returns the hash of the memory section.
/// Note: this function ignores the ids and therefore assumes that the section is sorted.
#[cfg(feature: "poseidon252_verifier")]
pub fn hash_memory_section(section: @MemorySection) -> Box<[u32; 8]> {
    let mut felts = array![];
    for entry in section {
        let (_, val) = *entry;
        felts.append(construct_f252(BoxTrait::new(val)));
    }
    deconstruct_f252(poseidon_hash_span(felts.span()))
}

/// Constructs a `felt252` from 8 u32 little-endian limbs.
/// Doesn't check for overflow, i.e, the result is in fact a u256 modulo p252.
pub fn construct_f252(x: Box<[u32; 8]>) -> felt252 {
    let [l0, l1, l2, l3, l4, l5, l6, l7] = x.unbox();
    let offset = 0x100000000;
    let result: felt252 = l7.into();
    let result = result * offset + l6.into();
    let result = result * offset + l5.into();
    let result = result * offset + l4.into();
    let result = result * offset + l3.into();
    let result = result * offset + l2.into();
    let result = result * offset + l1.into();
    result * offset + l0.into()
}

/// Deconstructs a `felt252` to 8 u32 little-endian limbs.
pub fn deconstruct_f252(x: felt252) -> Box<[u32; 8]> {
    let offset = 0x100000000;
    let cur: u256 = x.into();
    let (cur, l0) = DivRem::div_rem(cur, offset);
    let (cur, l1) = DivRem::div_rem(cur, offset);
    let (cur, l2) = DivRem::div_rem(cur, offset);
    let (cur, l3) = DivRem::div_rem(cur, offset);
    let (cur, l4) = DivRem::div_rem(cur, offset);
    let (cur, l5) = DivRem::div_rem(cur, offset);
    let (cur, l6) = DivRem::div_rem(cur, offset);
    let (_, l7) = DivRem::div_rem(cur, offset);
    BoxTrait::new(
        [
            l0.try_into().unwrap(), l1.try_into().unwrap(), l2.try_into().unwrap(),
            l3.try_into().unwrap(), l4.try_into().unwrap(), l5.try_into().unwrap(),
            l6.try_into().unwrap(), l7.try_into().unwrap(),
        ],
    )
}
