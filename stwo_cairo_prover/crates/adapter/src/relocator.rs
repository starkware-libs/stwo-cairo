use std::collections::HashMap;

use cairo_vm::types::builtin_name::BuiltinName;
use cairo_vm::types::relocatable::MaybeRelocatable;
use stwo_cairo_common::memory::MEMORY_ADDRESS_BOUND;

use crate::builtins::{
    ADD_MOD_MEMORY_CELLS, BITWISE_MEMORY_CELLS, ECDSA_MEMORY_CELLS, EC_OP_MEMORY_CELLS,
    KECCAK_MEMORY_CELLS, MUL_MOD_MEMORY_CELLS, PEDERSEN_MEMORY_CELLS, POSEIDON_MEMORY_CELLS,
    RANGE_CHECK_MEMORY_CELLS,
};
use crate::memory::FELT252;

// The minimal memory entries or builtins instances per segment.
pub const MIN_SEGMENT_SIZE: usize = 16;

#[derive(Debug, Clone)]
/// The relocator is responsible for relocating the memory and the builtins.
pub struct Relocator {
    pub relocation_table: Vec<u32>,
    pub relocatable_mem: Vec<Vec<MaybeRelocatable>>,
    pub builtins_segments_indices: HashMap<usize, BuiltinName>,
}
impl Relocator {
    /// Allocate an address for each segment according to the relocatable memory.
    /// Each segment size is rounded up to the nearest power of two, and each built-in segment is
    /// rounded up to contain a power of two instances.
    /// Each segment size must be greater than 'MIN_SEGMENT_SIZE'.
    pub fn new(
        relocatable_mem: Vec<Vec<MaybeRelocatable>>,
        builtins_segments_indices: HashMap<usize, BuiltinName>,
    ) -> Self {
        let address_base = 1;
        let mut relocation_table = vec![address_base];
        for (index_segment, segment) in relocatable_mem.iter().enumerate() {
            let cells_per_instance = match builtins_segments_indices.get(&index_segment) {
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
                // None, segment_arena, output
                _ => 1,
            };

            let segment_size = segment
                .len()
                .div_ceil(cells_per_instance)
                .next_power_of_two()
                .max(MIN_SEGMENT_SIZE)
                * cells_per_instance;
            let addr = relocation_table.last().unwrap() + segment_size as u32;
            assert!(
                addr <= MEMORY_ADDRESS_BOUND as u32,
                "Relocated address: {} for segment: {} exceeded the maximum address value.",
                addr,
                index_segment
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

    /// Calculate the segment stop addr according to following segment start address.
    pub fn get_segment_stop_addr(&self, segment_index: usize) -> u32 {
        // Relocation table last element is the total memory size and not a valid segment start
        // addr.
        assert!(
            segment_index < self.relocation_table.len() - 1,
            "Not a valid segment index"
        );
        self.relocation_table[segment_index + 1] - 1
    }

    fn is_builtin_segment(&self, segment_index: usize) -> bool {
        self.builtins_segments_indices.contains_key(&segment_index)
    }

    /// Relocate the value according to the relocation table.
    pub fn relocate_value(&self, value: &MaybeRelocatable) -> FELT252 {
        let mut res = [0; 8];
        match value {
            MaybeRelocatable::RelocatableValue(addr) => {
                res[0] = self.calc_relocated_addr(addr.segment_index as usize, addr.offset)
            }
            MaybeRelocatable::Int(val) => res = bytemuck::cast(val.to_bytes_le()),
        }
        res
    }

    /// Returns a list of addresses that require zero padding,
    /// due to segment size extension to the next power of two.
    /// For builtins segments we want to pad with existing instances.
    pub fn get_memory_zero_padding_addresses(&self) -> Vec<u32> {
        let mut res = vec![];
        for (segment_index, segment) in self.relocatable_mem.iter().enumerate() {
            if self.is_builtin_segment(segment_index) {
                continue;
            }

            let mut last_addr = self.calc_relocated_addr(segment_index, segment.len());
            while last_addr <= self.get_segment_stop_addr(segment_index) {
                res.push(last_addr);
                last_addr += 1;
            }
        }
        res
    }

    /// Get a list of relocated addresses and values for a segment.
    fn get_relocatable_segment(&self, segment_index: usize) -> Vec<(u32, FELT252)> {
        let mut res = vec![];
        let segment = &self.relocatable_mem[segment_index];
        for (offset, value) in segment.iter().enumerate() {
            let addr = self.calc_relocated_addr(segment_index, offset);
            let val = self.relocate_value(value);
            res.push((addr, val));
        }
        res
    }

    /// Get a list of relocated addresses and values for the memory.
    pub fn get_relocatable_memory(&self) -> Vec<(u32, FELT252)> {
        let mut res = vec![];
        for (segment_index, _) in self.relocatable_mem.iter().enumerate() {
            res.extend(self.get_relocatable_segment(segment_index));
        }
        res
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;
    use std::vec;

    use cairo_vm::relocatable;
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
        let builtins_segments = HashMap::from([(1, BuiltinName::bitwise)]);

        Relocator::new(relocatble_memory, builtins_segments)
    }

    #[test]
    fn test_relocator() {
        let relocator = create_test_relocator();
        assert_eq!(relocator.relocation_table, vec![1, 17, 97, 113]);

        // test calc_relocated_addr
        assert_eq!(relocator.calc_relocated_addr(2, 2), 99);

        // test get_segment_stop_addr
        assert_eq!(relocator.get_segment_stop_addr(2), 112);

        // test is_builtin_segment
        assert!(relocator.is_builtin_segment(1));
        assert!(!relocator.is_builtin_segment(2));

        // test relocate_value
        assert_eq!(
            relocator.relocate_value(&MaybeRelocatable::RelocatableValue(relocatable!(2, 2))),
            [99, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            relocator.relocate_value(&MaybeRelocatable::Int(128.into())),
            [128, 0, 0, 0, 0, 0, 0, 0]
        );

        // test get_memory_zero_padding_addresses
        assert_eq!(
            relocator.get_memory_zero_padding_addresses(),
            vec![
                4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 100, 101, 102, 103, 104, 105, 106,
                107, 108, 109, 110, 111, 112
            ]
        );

        // test relocate segment
        assert_eq!(
            relocator.get_relocatable_segment(1),
            vec![(17, [2, 0, 0, 0, 0, 0, 0, 0])]
        );

        // test relocate memory
        assert_eq!(
            relocator.get_relocatable_memory(),
            vec![
                (1, [1, 0, 0, 0, 0, 0, 0, 0]),
                (2, [9, 0, 0, 0, 0, 0, 0, 0]),
                (3, [98, 0, 0, 0, 0, 0, 0, 0]),
                (17, [2, 0, 0, 0, 0, 0, 0, 0]),
                (97, [1, 0, 0, 0, 0, 0, 0, 0]),
                (98, [2, 0, 0, 0, 0, 0, 0, 0]),
                (99, [3, 0, 0, 0, 0, 0, 0, 0])
            ]
        );
    }
}
