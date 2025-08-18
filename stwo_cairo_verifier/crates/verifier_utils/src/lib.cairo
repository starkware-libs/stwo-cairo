#[cfg(test)]
mod test;
pub mod zip_eq;
#[cfg(test)]
mod zip_eq_test;

// TODO(alonf): Change this into a struct. Remove Pub prefix.
// (id, value)
pub type PubMemoryValue = (u32, [u32; 8]);

#[derive(Drop)]
pub struct PublicMemoryEntry {
    pub address: u32,
    pub id: u32,
    pub value: [u32; 8],
}

#[derive(PanicDestruct)]
pub struct PublicMemoryEntries {
    entries: Array<PublicMemoryEntry>,
}

#[generate_trait]
pub impl PublicMemoryEntriesImpl of PublicMemoryEntriesTrait {
    /// Create `PublicMemoryEntries` with no entries.
    #[inline(always)]
    fn empty() -> PublicMemoryEntries {
        PublicMemoryEntries { entries: array![] }
    }

    /// Add an entry to the public memory entries.
    #[inline(always)]
    fn add_memory_entry(ref self: PublicMemoryEntries, entry: PublicMemoryEntry) {
        self.entries.append(entry);
    }


    /// Adds all entries from a memory section to the public memory entries, starting at the given
    /// address.
    #[inline(always)]
    fn add_memory_section(
        ref self: PublicMemoryEntries, memory_section: @MemorySection, mut address: u32,
    ) {
        for (id, value) in *memory_section {
            self.add_memory_entry(PublicMemoryEntry { address, id: *id, value: *value });
            address += 1;
        }
    }
}


impl PublicMemoryEntriesIntoIterator of core::iter::IntoIterator<PublicMemoryEntries> {
    type IntoIter = core::array::ArrayIter<PublicMemoryEntry>;
    fn into_iter(self: PublicMemoryEntries) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

/// A contiguous memory section.
pub type MemorySection = Span<PubMemoryValue>;


pub mod blake2s;
pub mod poseidon252;
#[cfg(not(feature: "poseidon252_verifier"))]
pub use stwo_verifier_utils::blake2s::*;
#[cfg(feature: "poseidon252_verifier")]
pub use stwo_verifier_utils::poseidon252::*;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(not(feature: "poseidon252_verifier"))]
    fn test_encode_felt_in_limbs() {
        let felt0 = [0x12345678, 0x70000000, 0, 0, 0, 0, 0, 0];
        let felt1 = [
            0x12345678, 0x90abcdef, 0xabcdef12, 0x34567890, 0x01234567, 0x89abcdef, 0x01234567, 0,
        ];
        let mut array = array![];
        crate::encode_felt_in_limbs_to_array(felt0, ref array);
        crate::encode_felt_in_limbs_to_array(felt1, ref array);
        assert_eq!(
            array,
            array![
                1879048192, 305419896, 2147483648, 19088743, 2309737967, 19088743, 878082192,
                2882400018, 2427178479, 305419896,
            ],
        );
    }

    #[test]
    #[cfg(not(feature: "poseidon252_verifier"))]
    fn test_encode_and_hash_memory_section() {
        let memory_section = array![
            (0, [0x12345678, 0x90abcdef, 0, 0, 0, 0, 0, 0]),
            (1, [0xabcdef12, 0x34567890, 0, 0, 0, 0, 0, 0]),
        ];
        let hash = blake2s::encode_and_hash_memory_section(memory_section.span());
        assert_eq!(
            hash.unbox(),
            [
                2421522214, 635981307, 2862863578, 1664236125, 1878536921, 1607560013, 4274188691,
                2957079540,
            ],
        );
    }

    #[test]
    fn test_construct_f252() {
        let x = [
            2421522214, 635981307, 2862863578, 1664236125, 1878536921, 1607560013, 4274188691,
            2957079540,
        ];
        let f252 = construct_f252(BoxTrait::new(x));
        assert_eq!(
            f252, 115645365096977585374207223166120623839439046970571781411593222716768222992,
        );
    }
}
