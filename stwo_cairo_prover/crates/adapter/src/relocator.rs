use std::collections::BTreeMap;

use cairo_vm::types::builtin_name::BuiltinName;
use cairo_vm::types::relocatable::MaybeRelocatable;
use cairo_vm::vm::trace::trace_entry::TraceEntry;
use stwo_cairo_common::memory::MEMORY_ADDRESS_BOUND;
use stwo_cairo_common::prover_types::simd::N_LANES;

use crate::builtins::MemorySegmentAddresses;
use crate::memory::{MemoryEntry, F252};
use crate::vm_import::RelocatedTraceEntry;
use crate::BuiltinSegments;

// Minimal builtins instances per segment, chosen to fit SIMD requirements.
pub const MIN_SEGMENT_SIZE: usize = N_LANES;

#[derive(Debug, Clone)]
/// The relocator is responsible for relocating addresses for the memory and the builtins segments.
pub struct Relocator {
    pub relocation_table: Vec<u32>,
    pub relocatable_mem: Vec<Vec<Option<MaybeRelocatable>>>,
    pub builtins_segments_indices: BTreeMap<usize, BuiltinName>,
}
impl Relocator {
    /// Allocates an address for each segment according to the relocatable memory.
    /// Each built-in segment is rounded up to the nearest power of two instances
    /// or `MIN_SEGMENT_SIZE`, taking the maximum of the two.
    pub fn new(
        relocatable_mem: Vec<Vec<Option<MaybeRelocatable>>>,
        builtins_segments_indices: BTreeMap<usize, BuiltinName>,
    ) -> Self {
        let address_base = 1;
        let mut relocation_table = vec![address_base];

        for (segment_index, segment) in relocatable_mem.iter().enumerate() {
            let segment_size = segment.len();

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
            let value = if let Some(val) = value {
                self.relocate_value(val)
            } else {
                // If this cell is None, fill with zero.
                [0; 8]
            };
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
        assert!(
            res.len() <= MEMORY_ADDRESS_BOUND,
            "Relocated memory size exceeded the maximum address value",
        );
        res
    }

    // Return the segment info (start_address, exclusive end_address) for each builtin.
    pub fn get_builtin_segments(&self) -> BuiltinSegments {
        let mut res = BuiltinSegments::default();
        for (segment_index, builtin_name) in self.builtins_segments_indices.iter() {
            let start_addr = self.relocation_table[*segment_index];
            let end_addr = self.relocation_table[*segment_index + 1];
            let segment = if start_addr == end_addr {
                None
            } else {
                Some(MemorySegmentAddresses {
                    begin_addr: start_addr as usize,
                    stop_ptr: end_addr as usize,
                })
            };

            match builtin_name {
                BuiltinName::range_check => res.range_check_bits_128 = segment,
                BuiltinName::pedersen => res.pedersen = segment,
                BuiltinName::bitwise => res.bitwise = segment,
                BuiltinName::poseidon => res.poseidon = segment,
                BuiltinName::range_check96 => res.range_check_bits_96 = segment,
                BuiltinName::add_mod => res.add_mod = segment,
                BuiltinName::mul_mod => res.mul_mod = segment,
                BuiltinName::ecdsa | BuiltinName::keccak | BuiltinName::ec_op => {
                    panic!("Builtin {} is not supported in Stwo", builtin_name)
                }
                // Not builtins.
                BuiltinName::output | BuiltinName::segment_arena => {}
            };
        }
        res
    }

    pub fn relocate_trace(&self, relocatble_trace: &[TraceEntry]) -> Vec<RelocatedTraceEntry> {
        let mut res = vec![];
        for entry in relocatble_trace {
            res.push(RelocatedTraceEntry {
                pc: self.relocation_table[entry.pc.segment_index as usize] as usize
                    + entry.pc.offset,
                // The segment indexes for `ap` and `fp` are always 1, see
                // 'https://github.com/lambdaclass/cairo-vm/blob/main/vm/src/vm/trace/mod.rs#L12'.
                ap: self.relocation_table[1] as usize + entry.ap,
                fp: self.relocation_table[1] as usize + entry.fp,
            })
        }
        res
    }

    pub fn relocate_public_addresses(
        &self,
        public_addresses: BTreeMap<usize, Vec<usize>>,
    ) -> Vec<u32> {
        let mut res = vec![];
        for (segment_index, offsets) in public_addresses {
            let base_addr = self.relocation_table[segment_index];

            for offset in offsets {
                let addr = base_addr + offset as u32;
                assert!(
                    addr < self.relocation_table[segment_index + 1],
                    "Offset {} is out of segment {}",
                    offset,
                    segment_index
                );
                res.push(addr);
            }
        }

        res
    }
}

#[cfg(test)]
pub mod relocator_tests {
    use std::vec;

    use cairo_vm::relocatable;
    use cairo_vm::types::builtin_name::BuiltinName;
    use cairo_vm::types::relocatable::{MaybeRelocatable, Relocatable};

    use super::*;
    use crate::builtins::MemorySegmentAddresses;
    use crate::relocated_trace_entry;

    pub fn create_test_relocator() -> Relocator {
        let segment0 = vec![
            Some(MaybeRelocatable::Int(1.into())),
            Some(MaybeRelocatable::Int(9.into())),
            Some(MaybeRelocatable::RelocatableValue(relocatable!(2, 1))),
        ];
        let builtin_segment1 =
            vec![Some(MaybeRelocatable::RelocatableValue(relocatable!(0, 1))); 80];
        let segment2 = vec![
            Some(MaybeRelocatable::Int(1.into())),
            Some(MaybeRelocatable::Int(2.into())),
            Some(MaybeRelocatable::Int(3.into())),
        ];

        let relocatble_memory = vec![segment0, builtin_segment1, segment2];
        let builtins_segments =
            BTreeMap::from([(1, BuiltinName::bitwise), (2, BuiltinName::segment_arena)]);

        Relocator::new(relocatble_memory, builtins_segments)
    }

    pub fn get_test_relocatble_trace() -> Vec<TraceEntry> {
        vec![
            TraceEntry {
                pc: relocatable!(0, 0),
                ap: 1,
                fp: 1,
            },
            TraceEntry {
                pc: relocatable!(1, 1),
                ap: 2,
                fp: 2,
            },
            TraceEntry {
                pc: relocatable!(2, 1),
                ap: 2,
                fp: 2,
            },
        ]
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
            (4..84)
                .map(|addr| MemoryEntry {
                    address: addr,
                    value: [2, 0, 0, 0, 0, 0, 0, 0],
                })
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_relocate_memory() {
        let relocator = create_test_relocator();

        let relocated_memory = relocator.get_relocated_memory();
        assert_eq!(
            relocated_memory[0],
            MemoryEntry {
                address: 1,
                value: [1, 0, 0, 0, 0, 0, 0, 0]
            }
        );

        assert_eq!(
            relocated_memory[1],
            MemoryEntry {
                address: 2,
                value: [9, 0, 0, 0, 0, 0, 0, 0]
            }
        );

        assert_eq!(
            relocated_memory[2],
            MemoryEntry {
                address: 3,
                value: [85, 0, 0, 0, 0, 0, 0, 0]
            }
        );

        assert_eq!(
            relocated_memory[3],
            MemoryEntry {
                address: 4,
                value: [2, 0, 0, 0, 0, 0, 0, 0],
            }
        );

        assert_eq!(
            relocated_memory[83],
            MemoryEntry {
                address: 84,
                value: [1, 0, 0, 0, 0, 0, 0, 0],
            }
        );

        assert_eq!(
            relocated_memory[84],
            MemoryEntry {
                address: 85,
                value: [2, 0, 0, 0, 0, 0, 0, 0],
            }
        );

        assert_eq!(
            relocated_memory[85],
            MemoryEntry {
                address: 86,
                value: [3, 0, 0, 0, 0, 0, 0, 0],
            }
        );
    }

    #[test]
    fn test_create_builtins_segments() {
        let builtin_segment0 = vec![
            Some(MaybeRelocatable::Int(1.into())),
            Some(MaybeRelocatable::Int(9.into())),
            Some(MaybeRelocatable::RelocatableValue(relocatable!(2, 1))),
            Some(MaybeRelocatable::Int(5498.into())),
            Some(MaybeRelocatable::RelocatableValue(relocatable!(2, 1478))),
        ];
        let segment0 = builtin_segment0
            .clone()
            .into_iter()
            .cycle()
            .take(16 * 5)
            .collect::<Vec<_>>();

        let builtin_segment1 =
            vec![Some(MaybeRelocatable::RelocatableValue(relocatable!(0, 1))); 16];
        let segment2 = vec![
            Some(MaybeRelocatable::Int(1.into())),
            Some(MaybeRelocatable::Int(2.into())),
            Some(MaybeRelocatable::Int(3.into())),
        ];

        let relocatble_memory = vec![segment0, builtin_segment1, segment2];
        let builtins_segments =
            BTreeMap::from([(0, BuiltinName::bitwise), (1, BuiltinName::range_check)]);

        let relocator = Relocator::new(relocatble_memory, builtins_segments);
        let builtins_segments = relocator.get_builtin_segments();

        assert_eq!(
            builtins_segments.bitwise,
            Some(MemorySegmentAddresses {
                begin_addr: 1,
                stop_ptr: 81
            })
        );
        assert_eq!(
            builtins_segments.range_check_bits_128,
            Some(MemorySegmentAddresses {
                begin_addr: 81,
                stop_ptr: 97
            })
        );
    }

    #[test]
    fn test_relocate_trace() {
        let relocatble_trace = get_test_relocatble_trace();
        let relocator = create_test_relocator();
        let relocated_trace = relocator.relocate_trace(&relocatble_trace);

        let expected_relocated_trace = vec![
            relocated_trace_entry!(5, 5, 1),
            relocated_trace_entry!(6, 6, 5),
            relocated_trace_entry!(6, 6, 85),
        ];
        assert_eq!(relocated_trace, expected_relocated_trace);
    }

    #[test]
    fn test_relocate_public_adresses() {
        let relocator = create_test_relocator();

        let relocatble_public_addrs = BTreeMap::from([(0, vec![2]), (1, vec![0, 1, 43])]);
        let relocated_public_addrs = relocator.relocate_public_addresses(relocatble_public_addrs);

        let expected_relocated_public_addresses = vec![3, 4, 5, 47];
        assert_eq!(relocated_public_addrs, expected_relocated_public_addresses);
    }
}
