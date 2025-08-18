use core::box::BoxTrait;
use core::poseidon::poseidon_hash_span;
use super::{MemorySection, construct_f252, deconstruct_f252};

/// Returns the hash of the memory section, for packing purposes.
/// Note: this function ignores the ids and therefore assumes that the section is sorted.
/// Note: no encoding is done at the moment.
// TODO(Gali): There is no encoding done at the moment with poseidon, so this function is
// equivalent to `hash_memory_section`.
pub fn encode_and_hash_memory_section(section: MemorySection) -> Box<[u32; 8]> {
    hash_memory_section(section)
}

/// Returns the hash of the memory section.
/// Note: this function ignores the ids and therefore assumes that the section is sorted.
pub fn hash_memory_section(section: MemorySection) -> Box<[u32; 8]> {
    let mut felts = array![];
    for entry in section {
        let (_, val) = *entry;
        felts.append(construct_f252(BoxTrait::new(val)));
    }
    deconstruct_f252(poseidon_hash_span(felts.span()))
}
