use cairo_vm::stdlib::collections::HashMap;
use cairo_vm::types::builtin_name::BuiltinName;
use cairo_vm::types::relocatable::MaybeRelocatable;
use stwo_cairo_common::memory::MEMORY_ADDRESS_BOUND;
use stwo_cairo_common::prover_types::simd::N_LANES;

use crate::builtins::{
    BuiltinSegments, ADD_MOD_MEMORY_CELLS, BITWISE_MEMORY_CELLS, ECDSA_MEMORY_CELLS,
    EC_OP_MEMORY_CELLS, KECCAK_MEMORY_CELLS, MUL_MOD_MEMORY_CELLS, PEDERSEN_MEMORY_CELLS,
    POSEIDON_MEMORY_CELLS, RANGE_CHECK_MEMORY_CELLS,
};
use crate::memory::{MemoryEntry, F252};

// Minimal builtins instances per segment, chosen to fit SIMD requirements.
pub const MIN_SEGMENT_SIZE: usize = N_LANES;

#[derive(Debug, Clone)]
/// The relocator is responsible for relocating addresses for the memory and the builtins segments.
pub struct Relocator {
    pub relocation_table: Vec<u32>,
    pub relocatable_mem: Vec<Vec<MaybeRelocatable>>,
    pub builtins_segments_indices: HashMap<usize, BuiltinName>,
}
impl Relocator {
    /// Allocates an address for each segment according to the relocatable memory.
    /// Each built-in segment is rounded up to the nearest power of two instances
    /// or `MIN_SEGMENT_SIZE`, taking the maximum of the two.
    pub fn new(
        relocatable_mem: Vec<Vec<MaybeRelocatable>>,
        builtins_segments_indices: HashMap<usize, BuiltinName>,
    ) -> Self {
        let address_base = 1;
        let mut relocation_table = vec![address_base];

        let is_builtin_segment = |segment_index: usize| {
            builtins_segments_indices
                .get(&segment_index)
                .map(|name| *name != BuiltinName::segment_arena && *name != BuiltinName::output)
                .unwrap_or(false)
        };

        for (segment_index, segment) in relocatable_mem.iter().enumerate() {
            let segment_size = if !is_builtin_segment(segment_index) {
                // If it is not a builtin segment, no need to pad.
                segment.len()
            } else {
                // If it is a builtin segment, pad its size to the next power of two instances.
                let cells_per_instance = match builtins_segments_indices.get(&segment_index) {
                    Some(BuiltinName::add_mod) => ADD_MOD_MEMORY_CELLS,
                    Some(BuiltinName::bitwise) => BITWISE_MEMORY_CELLS,
                    Some(BuiltinName::ec_op) => EC_OP_MEMORY_CELLS,
                    Some(BuiltinName::ecdsa) => ECDSA_MEMORY_CELLS,
                    Some(BuiltinName::keccak) => KECCAK_MEMORY_CELLS,
                    Some(BuiltinName::mul_mod) => MUL_MOD_MEMORY_CELLS,
                    Some(BuiltinName::pedersen) => PEDERSEN_MEMORY_CELLS,
                    Some(BuiltinName::poseidon) => POSEIDON_MEMORY_CELLS,
                    Some(BuiltinName::range_check96) => RANGE_CHECK_MEMORY_CELLS,
                    Some(BuiltinName::range_check) => RANGE_CHECK_MEMORY_CELLS,
                    _ => panic!("Segment index expected to be builtin segment"),
                };

                segment
                    .len()
                    .div_ceil(cells_per_instance)
                    .next_power_of_two()
                    .max(MIN_SEGMENT_SIZE)
                    * cells_per_instance
            };

            let addr = relocation_table.last().unwrap() + segment_size as u32;
            assert!(
                addr <= MEMORY_ADDRESS_BOUND as u32,
                "Relocated address: {} for segment: {} exceeded the maximum address value.",
                addr,
                segment_index
            );
            relocation_table.push(addr);
        }

        Self {
            relocation_table,
            relocatable_mem,
            builtins_segments_indices,
        }
    }

    pub fn calc_relocated_addr(&self, segment_index: usize, offset: usize) -> u32 {
        self.relocation_table[segment_index] + offset as u32
    }

    /// Relocate the value according to the relocation table.
    pub fn relocate_value(&self, value: &MaybeRelocatable) -> F252 {
        let mut res = [0; 8];
        match value {
            MaybeRelocatable::RelocatableValue(addr) => {
                res[0] = self.calc_relocated_addr(addr.segment_index as usize, addr.offset)
            }
            MaybeRelocatable::Int(val) => res = bytemuck::cast(val.to_bytes_le()),
        }
        res
    }

    /// Get a list of relocated addresses and values for a segment.
    fn get_relocated_segment(&self, segment_index: usize) -> Vec<MemoryEntry> {
        let mut res = vec![];
        let segment = &self.relocatable_mem[segment_index];
        for (offset, value) in segment.iter().enumerate() {
            let address = self.calc_relocated_addr(segment_index, offset) as u64;
            let value = self.relocate_value(value);
            res.push(MemoryEntry { address, value });
        }
        res
    }

    /// Get a list of relocated addresses and values for the memory.
    pub fn get_relocated_memory(&self) -> Vec<MemoryEntry> {
        let mut res = vec![];
        for (segment_index, _) in self.relocatable_mem.iter().enumerate() {
            res.extend(self.get_relocated_segment(segment_index));
        }
        res
    }

    // Return the segment info (start_address, exclusive end_address) for each builtin.
    pub fn get_builtin_segments(&self) -> BuiltinSegments {
        let mut res = BuiltinSegments::default();
        for (segment_index, builtin_name) in self.builtins_segments_indices.iter() {
            let start_addr = self.relocation_table[*segment_index];
            let end_addr = self.relocation_table[*segment_index + 1];
            let segment = Some((start_addr as usize, end_addr as usize).into());

            match builtin_name {
                BuiltinName::range_check => res.range_check_bits_128 = segment,
                BuiltinName::pedersen => res.pedersen = segment,
                BuiltinName::ecdsa => res.ecdsa = segment,
                BuiltinName::keccak => res.keccak = segment,
                BuiltinName::bitwise => res.bitwise = segment,
                BuiltinName::ec_op => res.ec_op = segment,
                BuiltinName::poseidon => res.poseidon = segment,
                BuiltinName::range_check96 => res.range_check_bits_96 = segment,
                BuiltinName::add_mod => res.add_mod = segment,
                BuiltinName::mul_mod => res.mul_mod = segment,
                // Not builtins.
                BuiltinName::output | BuiltinName::segment_arena => {}
            };
        }
        res
    }
}

#[cfg(test)]
pub mod relocator_tests {
    use std::vec;

    use cairo_vm::relocatable;
    use cairo_vm::stdlib::collections::HashMap;
    use cairo_vm::types::builtin_name::BuiltinName;
    use cairo_vm::types::relocatable::{MaybeRelocatable, Relocatable};

    use super::*;

    pub fn create_test_relocator() -> Relocator {
        let segment0 = vec![
            MaybeRelocatable::Int(1.into()),
            MaybeRelocatable::Int(9.into()),
            MaybeRelocatable::RelocatableValue(relocatable!(2, 1)),
        ];
        let builtin_segment1 = vec![MaybeRelocatable::RelocatableValue(relocatable!(0, 1))];
        let segment2 = vec![
            MaybeRelocatable::Int(1.into()),
            MaybeRelocatable::Int(2.into()),
            MaybeRelocatable::Int(3.into()),
        ];

        let relocatble_memory = vec![segment0, builtin_segment1, segment2];
        let builtins_segments =
            HashMap::from([(1, BuiltinName::bitwise), (2, BuiltinName::segment_arena)]);

        Relocator::new(relocatble_memory, builtins_segments)
    }

    #[test]
    fn test_relocation_table() {
        let relocator = create_test_relocator();
        assert_eq!(relocator.relocation_table, vec![1, 4, 84, 87]);
    }

    #[test]
    fn test_calc_relocated_addr() {
        let relocator = create_test_relocator();
        assert_eq!(relocator.calc_relocated_addr(2, 2), 86);
    }

    #[test]
    fn test_relocated_value() {
        let relocator = create_test_relocator();
        assert_eq!(
            relocator.relocate_value(&MaybeRelocatable::RelocatableValue(relocatable!(2, 2))),
            [86, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            relocator.relocate_value(&MaybeRelocatable::RelocatableValue(relocatable!(2, 2))),
            [86, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            relocator.relocate_value(&MaybeRelocatable::Int(128.into())),
            [128, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn cargo_test_relocate_segment() {
        let relocator = create_test_relocator();
        assert_eq!(
            relocator.get_relocated_segment(1),
            vec![MemoryEntry {
                address: 4,
                value: [2, 0, 0, 0, 0, 0, 0, 0]
            }]
        );
    }

    #[test]
    fn test_relocate_memory() {
        let relocator = create_test_relocator();
        assert_eq!(
            relocator.get_relocated_memory(),
            vec![
                MemoryEntry {
                    address: 1,
                    value: [1, 0, 0, 0, 0, 0, 0, 0],
                },
                MemoryEntry {
                    address: 2,
                    value: [9, 0, 0, 0, 0, 0, 0, 0],
                },
                MemoryEntry {
                    address: 3,
                    value: [85, 0, 0, 0, 0, 0, 0, 0],
                },
                MemoryEntry {
                    address: 4,
                    value: [2, 0, 0, 0, 0, 0, 0, 0],
                },
                MemoryEntry {
                    address: 84,
                    value: [1, 0, 0, 0, 0, 0, 0, 0],
                },
                MemoryEntry {
                    address: 85,
                    value: [2, 0, 0, 0, 0, 0, 0, 0],
                },
                MemoryEntry {
                    address: 86,
                    value: [3, 0, 0, 0, 0, 0, 0, 0],
                }
            ]
        );
    }

    #[test]
    fn test_create_builtins_segments() {
        let builtin_segment0 = vec![
            MaybeRelocatable::Int(1.into()),
            MaybeRelocatable::Int(9.into()),
            MaybeRelocatable::RelocatableValue(relocatable!(2, 1)),
            MaybeRelocatable::Int(5498.into()),
            MaybeRelocatable::RelocatableValue(relocatable!(2, 1478)),
        ];
        let builtin_segment1 = vec![MaybeRelocatable::RelocatableValue(relocatable!(0, 1))];
        let segment2 = vec![
            MaybeRelocatable::Int(1.into()),
            MaybeRelocatable::Int(2.into()),
            MaybeRelocatable::Int(3.into()),
        ];

        let relocatble_memory = vec![builtin_segment0, builtin_segment1, segment2];
        let builtins_segments =
            HashMap::from([(0, BuiltinName::bitwise), (1, BuiltinName::range_check)]);

        let relocator = Relocator::new(relocatble_memory, builtins_segments);
        let builtins_segments = relocator.get_builtin_segments();

        assert_eq!(builtins_segments.bitwise, Some((1, 81).into()));
        assert_eq!(
            builtins_segments.range_check_bits_128,
            Some((81, 97).into())
        );
        assert_eq!(builtins_segments.ecdsa, None);
    }
}
