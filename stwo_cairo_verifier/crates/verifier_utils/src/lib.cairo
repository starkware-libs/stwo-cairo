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

// TODO: Stone uses a different initial state with the key set to 0.
// Consider using this initial state instead.
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
