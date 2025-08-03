#[cfg(not(feature: "poseidon252_verifier"))]
use core::blake::{blake2s_compress, blake2s_finalize};
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

#[cfg(not(feature: "poseidon252_verifier"))]
pub const BLAKE2S_256_INITIAL_STATE: [u32; 8] = [
    0x6B08E647, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];

/// Returns the hash of the memory section.
/// Note: this function ignores the ids and therefore assumes that the section is sorted.
#[cfg(not(feature: "poseidon252_verifier"))]
pub fn hash_memory_section(section: @MemorySection) -> Box<[u32; 8]> {
    let mut state = BoxTrait::new(BLAKE2S_256_INITIAL_STATE);
    let mut byte_count = 0;

    let mut buffer = array![];
    for entry in section {
        // Compress whenever the buffer reaches capacity.
        if let Some(msg) = buffer.span().try_into() {
            state = blake2s_compress(state, byte_count, *msg);
            buffer = array![];
        }

        // Append current value to the buffer without its id.
        let (_, val) = *entry;
        buffer.append_span(val.span());
        byte_count += 32;
    }

    // Pad buffer to blake hash message size.
    for _ in buffer.len()..16 {
        buffer.append(0);
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
