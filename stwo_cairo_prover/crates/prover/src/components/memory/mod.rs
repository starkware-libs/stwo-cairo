pub mod memory_address_to_id;
pub mod memory_id_to_big;

/// Used for sanity checks and assertions.
pub const LOG_MEMORY_ADDRESS_BOUND: u32 = 27;
pub const MEMORY_ADDRESS_BOUND: usize = 1 << LOG_MEMORY_ADDRESS_BOUND;

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use stwo_prover::core::fields::m31::BaseField;

    use crate::components::memory::memory_address_to_id;
    use crate::input::mem::{MemConfig, MemoryBuilder, MemoryValueId};
    use crate::input::vm_import::MemEntry;

    #[test]
    fn test_memory_trace_prover() {
        const N_ENTRIES: u64 = 10;
        let memory = MemoryBuilder::from_iter(
            MemConfig::default(),
            (0..N_ENTRIES).map(|i| MemEntry {
                addr: i,
                val: [i as u32; 8],
            }),
        )
        .build();
        let mut memory_address_to_id_gen = memory_address_to_id::ClaimGenerator::new(&memory);
        let mut memory_id_to_big = memory_address_to_id::ClaimGenerator::new(&memory);
        let address_usages = [0, 1, 1, 2, 2, 2]
            .into_iter()
            .map(BaseField::from)
            .collect_vec();
        let expected_addr_mult: [u32; N_ENTRIES as usize] = [1, 2, 3, 0, 0, 0, 0, 0, 0, 0];
        let expected_f252_mult: [u32; N_ENTRIES as usize] = [2, 3, 0, 0, 0, 0, 0, 0, 0, 0];

        address_usages.iter().for_each(|addr| {
            let decoded_id = memory.address_to_id[addr.0 as usize].decode();
            match decoded_id {
                MemoryValueId::F252(id) => {
                    memory_id_to_big.add_m31(BaseField::from_u32_unchecked(id));
                }
                MemoryValueId::Small(_id) => {}
            }
            memory_address_to_id_gen.add_m31(*addr);
        });

        assert_eq!(memory_address_to_id_gen.multiplicities, expected_addr_mult);
        assert_eq!(memory_id_to_big.multiplicities, expected_f252_mult);
    }
}
