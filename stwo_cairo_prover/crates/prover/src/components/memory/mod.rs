pub mod memory_address_to_id;
pub mod memory_id_to_big;

/// Used for sanity checks and assertions.
pub const LOG_MEMORY_ADDRESS_BOUND: u32 = 27;
pub const MEMORY_ADDRESS_BOUND: usize = 1 << LOG_MEMORY_ADDRESS_BOUND;

#[cfg(test)]
mod tests {
    use std::simd::Simd;

    use itertools::Itertools;
    use stwo_prover::core::backend::simd::m31::N_LANES;
    use stwo_prover::core::fields::m31::BaseField;

    use crate::components::memory::memory_address_to_id;
    use crate::input::memory::{MemoryBuilder, MemoryConfig, MemoryValueId};
    use crate::input::vm_import::MemoryEntry;

    #[test]
    fn test_memory_trace_prover() {
        const N_ENTRIES: u64 = 10;
        let memory = MemoryBuilder::from_iter(
            MemoryConfig::default(),
            (0..N_ENTRIES).map(|i| MemoryEntry {
                address: i,
                value: [i as u32; 8],
            }),
        )
        .build();
        let mut memory_address_to_id_gen = memory_address_to_id::ClaimGenerator::new(&memory);
        let mut memory_id_to_big = memory_address_to_id::ClaimGenerator::new(&memory);
        let address_usages = [0, 1, 1, 2, 2, 2]
            .into_iter()
            .map(BaseField::from)
            .collect_vec();
        let expected_addr_mult: [u32; N_LANES] = [1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let expected_f252_mult: [u32; N_LANES] = [2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

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

        let addr_to_id_mults = memory_address_to_id_gen.multiplicities.into_simd_vec();
        let id_to_big_mults = memory_id_to_big.multiplicities.into_simd_vec();
        assert_eq!(addr_to_id_mults.len(), 1);
        assert_eq!(
            addr_to_id_mults[0].into_simd(),
            Simd::from_array(expected_addr_mult),
        );
        assert_eq!(id_to_big_mults.len(), 1);
        assert_eq!(
            id_to_big_mults[0].into_simd(),
            Simd::from_array(expected_f252_mult)
        );
    }
}
