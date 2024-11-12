pub mod addr_to_id;
pub mod id_to_f252;

pub const LOG_MEMORY_ADDRESS_BOUND: u32 = 20;
pub const MEMORY_ADDRESS_BOUND: usize = 1 << LOG_MEMORY_ADDRESS_BOUND;

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use stwo_prover::core::fields::m31::BaseField;

    use crate::components::memory::addr_to_id;
    use crate::input::mem::{MemConfig, MemoryBuilder, MemoryValueId};
    use crate::input::vm_import::MemEntry;

    #[test]
    fn test_memory_trace_prover() {
        let memory = MemoryBuilder::from_iter(
            MemConfig::default(),
            (0..10).map(|i| MemEntry {
                addr: i,
                val: [i as u32; 8],
            }),
        )
        .build();
        let mut addr_to_id_gen = addr_to_id::ClaimGenerator::new(&memory);
        let mut id_to_f252 = addr_to_id::ClaimGenerator::new(&memory);
        let address_usages = [0, 1, 1, 2, 2, 2]
            .into_iter()
            .map(BaseField::from)
            .collect_vec();
        let expected_addr_mult: [u32; 16] = [1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let expected_f252_mult: [u32; 16] = [2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        address_usages.iter().for_each(|addr| {
            let decoded_id = memory.address_to_id[addr.0 as usize].decode();
            match decoded_id {
                MemoryValueId::F252(id) => {
                    id_to_f252.add_m31(BaseField::from_u32_unchecked(id as u32));
                }
                MemoryValueId::Small(_id) => {}
            }
            addr_to_id_gen.add_m31(*addr);
        });

        assert_eq!(addr_to_id_gen.multiplicities, expected_addr_mult);
        assert_eq!(id_to_f252.multiplicities, expected_f252_mult);
    }
}
