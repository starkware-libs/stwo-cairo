use std::collections::BTreeMap;

use cairo_vm::types::builtin_name::BuiltinName;
use cairo_vm::types::relocatable::MaybeRelocatable;
use cairo_vm::vm::runners::cairo_runner::ProverInputInfo;
use cairo_vm::vm::trace::trace_entry::TraceEntry;
use stwo_cairo_common::memory::MEMORY_ADDRESS_BOUND;
use stwo_cairo_common::prover_types::simd::N_LANES;
use tracing::{span, Level};

use crate::builtins::MemorySegmentAddresses;
use crate::memory::{MemoryEntry, F252};
use crate::vm_import::RelocatedTraceEntry;
use crate::BuiltinSegments;

// Minimal builtins instances per segment, chosen to fit SIMD requirements.
pub const MIN_SEGMENT_SIZE: usize = N_LANES;

pub fn relocate_prover_input_info(
    input: &ProverInputInfo,
) -> (
    Vec<MemoryEntry>,
    Vec<RelocatedTraceEntry>,
    BuiltinSegments,
    Vec<u32>,
) {
    let _span: span::EnteredSpan = span!(Level::INFO, "relocate_prover_input_info").entered();
    let relocation_table = create_relocation_table(
        input
            .relocatable_memory
            .iter()
            .map(|segment| segment.len() as u32)
            .collect(),
    );

    let relocated_memory = relocate_memory(&input.relocatable_memory, &relocation_table);
    let relocated_trace = relocate_trace(&input.relocatable_trace, &relocation_table);
    let builtin_segments = relocate_builtin_segments(&input.builtins_segments, &relocation_table);
    let pa_addresses =
        relocate_public_addresses(input.public_memory_offsets.clone(), &relocation_table);

    (
        relocated_memory,
        relocated_trace,
        builtin_segments,
        pa_addresses,
    )
}

/// Allocates an address for each segment according to the relocatable memory segments size.
fn create_relocation_table(segments_size: Vec<u32>) -> Vec<u32> {
    let _span = span!(Level::INFO, "create_relocation_table").entered();
    let address_base = 1;
    let mut relocation_table = vec![address_base];

    for (segment_index, segment_size) in segments_size.iter().enumerate() {
        let addr = relocation_table.last().unwrap() + *segment_size;
        assert!(
            addr <= MEMORY_ADDRESS_BOUND as u32,
            "Relocated address: {} for segment: {} exceeded the maximum address value.",
            addr,
            segment_index
        );
        relocation_table.push(addr);
    }
    relocation_table
}

/// Relocate the value according to the relocation table.
fn relocate_value(value: &MaybeRelocatable, relocation_table: &[u32]) -> F252 {
    let mut res = [0; 8];
    match value {
        MaybeRelocatable::RelocatableValue(addr) => {
            res[0] = relocation_table[addr.segment_index as usize] + addr.offset as u32
        }
        MaybeRelocatable::Int(val) => res = bytemuck::cast(val.to_bytes_le()),
    }
    res
}

/// Relocates the given segment according to the relocation table.
fn relocated_segment(
    segment_index: usize,
    segment: &[Option<MaybeRelocatable>],
    relocation_table: &[u32],
) -> Vec<MemoryEntry> {
    let mut res = vec![];
    for (offset, value) in segment.iter().enumerate() {
        let address = relocation_table[segment_index] as u64 + offset as u64;
        let value = if let Some(val) = value {
            relocate_value(val, relocation_table)
        } else {
            // If this cell is None, fill with zero.
            [0; 8]
        };
        res.push(MemoryEntry { address, value });
    }
    res
}

/// Relocates the given memory segment by segment.
fn relocate_memory(
    memory: &[Vec<Option<MaybeRelocatable>>],
    relocation_table: &[u32],
) -> Vec<MemoryEntry> {
    let _span = span!(Level::INFO, "relocate_memory").entered();
    let mut res = vec![];
    for (segment_index, segment) in memory.iter().enumerate() {
        res.extend(relocated_segment(segment_index, segment, relocation_table));
    }
    assert!(
        res.len() <= MEMORY_ADDRESS_BOUND,
        "Relocated memory size exceeded the maximum address value",
    );
    res
}

// Return the segment info (start_address, exclusive end_address) for each builtin.
fn relocate_builtin_segments(
    builtins: &BTreeMap<usize, BuiltinName>,
    relocation_table: &[u32],
) -> BuiltinSegments {
    let _span = span!(Level::INFO, "get_builtin_segments").entered();
    let mut res = BuiltinSegments::default();
    for (segment_index, builtin_name) in builtins.iter() {
        let start_addr = relocation_table[*segment_index];
        let end_addr = relocation_table[*segment_index + 1];
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

fn relocate_trace(
    relocatble_trace: &[TraceEntry],
    relocation_table: &[u32],
) -> Vec<RelocatedTraceEntry> {
    let _span = span!(Level::INFO, "relocate_trace").entered();
    let mut res = vec![];
    for entry in relocatble_trace {
        res.push(RelocatedTraceEntry {
            pc: relocation_table[entry.pc.segment_index as usize] as usize + entry.pc.offset,
            // The segment indexes for `ap` and `fp` are always 1, see
            // 'https://github.com/lambdaclass/cairo-vm/blob/main/vm/src/vm/trace/mod.rs#L12'.
            ap: relocation_table[1] as usize + entry.ap,
            fp: relocation_table[1] as usize + entry.fp,
        })
    }
    res
}

fn relocate_public_addresses(
    public_addresses: BTreeMap<usize, Vec<usize>>,
    relocation_table: &[u32],
) -> Vec<u32> {
    let _span = span!(Level::INFO, "relocate_public_addresses").entered();
    let mut res = vec![];
    for (segment_index, offsets) in public_addresses {
        let base_addr = relocation_table[segment_index];

        for offset in offsets {
            let addr = base_addr + offset as u32;
            assert!(
                addr < relocation_table[segment_index + 1],
                "Offset {} is out of segment {}",
                offset,
                segment_index
            );
            res.push(addr);
        }
    }

    res
}

#[cfg(test)]
pub mod relocator_tests {
    use std::vec;

    use cairo_vm::relocatable;
    use cairo_vm::types::builtin_name::BuiltinName;
    use cairo_vm::types::relocatable::{MaybeRelocatable, Relocatable};
    use stwo::core::fields::m31::M31;
    use stwo_cairo_common::prover_types::cpu::CasmState;

    use super::*;
    use crate::builtins::MemorySegmentAddresses;
    use crate::memory::{u128_to_4_limbs, MemoryBuilder, MemoryConfig, MemoryEntry, MemoryValue};
    use crate::{casm_state, relocated_trace_entry, StateTransitions};

    pub fn create_test_memory_and_relocation_table(
    ) -> (Vec<Vec<Option<MaybeRelocatable>>>, Vec<u32>) {
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
        let segments_size = vec![
            segment0.len() as u32,
            builtin_segment1.len() as u32,
            segment2.len() as u32,
        ];
        let memory = vec![segment0, builtin_segment1, segment2];

        (memory, create_relocation_table(segments_size))
    }

    pub fn create_test_relocatble_trace() -> Vec<TraceEntry> {
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
        let (_, relocation_table) = create_test_memory_and_relocation_table();
        assert_eq!(relocation_table, vec![1, 4, 84, 87]);
    }

    #[test]
    fn test_relocated_value() {
        let (_, relocation_table) = create_test_memory_and_relocation_table();
        assert_eq!(
            relocate_value(
                &MaybeRelocatable::RelocatableValue(relocatable!(2, 2)),
                &relocation_table
            ),
            [86, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            relocate_value(
                &MaybeRelocatable::RelocatableValue(relocatable!(2, 2)),
                &relocation_table
            ),
            [86, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            relocate_value(&MaybeRelocatable::Int(128.into()), &relocation_table),
            [128, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn cargo_test_relocate_segment() {
        let (memory, relocation_table) = create_test_memory_and_relocation_table();

        assert_eq!(
            relocated_segment(1, &memory[1], &relocation_table),
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
        let (memory, relocation_table) = create_test_memory_and_relocation_table();

        let relocated_memory = relocate_memory(&memory, &relocation_table);
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

        let relocatble_memory = [segment0, builtin_segment1, segment2];
        let builtins_segments =
            BTreeMap::from([(0, BuiltinName::bitwise), (1, BuiltinName::range_check)]);

        let relocation_table = create_relocation_table(
            relocatble_memory
                .iter()
                .map(|segment| segment.len() as u32)
                .collect(),
        );
        let builtins_segments = relocate_builtin_segments(&builtins_segments, &relocation_table);

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
        let relocatble_trace = create_test_relocatble_trace();
        let (_, relocation_table) = create_test_memory_and_relocation_table();

        let relocated_trace = relocate_trace(&relocatble_trace, &relocation_table);

        let expected_relocated_trace = vec![
            relocated_trace_entry!(5, 5, 1),
            relocated_trace_entry!(6, 6, 5),
            relocated_trace_entry!(6, 6, 85),
        ];
        assert_eq!(relocated_trace, expected_relocated_trace);
    }

    #[test]
    fn test_relocate_public_adresses() {
        let (_, relocation_table) = create_test_memory_and_relocation_table();

        let relocatble_public_addrs = BTreeMap::from([(0, vec![2]), (1, vec![0, 1, 43])]);
        let relocated_public_addrs =
            relocate_public_addresses(relocatble_public_addrs, &relocation_table);

        let expected_relocated_public_addresses = vec![3, 4, 5, 47];
        assert_eq!(relocated_public_addrs, expected_relocated_public_addresses);
    }

    #[test]
    fn test_memory_from_relocator() {
        let (memory, reolcation_table) = create_test_memory_and_relocation_table();

        let memory = MemoryBuilder::from_iter(
            MemoryConfig::default(),
            relocate_memory(&memory, &reolcation_table),
        );
        assert_eq!(memory.get(1), MemoryValue::Small(1));
        assert_eq!(memory.get(85), MemoryValue::Small(2));
    }

    #[test]
    fn test_casm_state_from_relocator() {
        let (_, reolcation_table) = create_test_memory_and_relocation_table();
        let encoded_qm_31_add_mul_inst =
            0b11100000001001010011111111111110101111111111111001000000000000000;
        let x = u128_to_4_limbs(encoded_qm_31_add_mul_inst);

        let memory_value = MemoryValue::F252([x[0], x[1], x[2], x[3], 0, 0, 0, 0]);
        let mut memory_builder = MemoryBuilder::new(MemoryConfig::default());
        memory_builder.set(1, memory_value);
        memory_builder.set(5, memory_value);
        memory_builder.set(85, memory_value);

        let relocated_trace = relocate_trace(&create_test_relocatble_trace(), &reolcation_table);

        let state_transitions =
            StateTransitions::from_iter(relocated_trace.into_iter(), &memory_builder);
        assert_eq!(
            state_transitions.casm_states_by_opcode.qm_31_add_mul_opcode,
            vec![casm_state!(1, 5, 5), casm_state!(5, 6, 6)]
        );
        assert_eq!(state_transitions.final_state, casm_state!(85, 6, 6));
        assert_eq!(state_transitions.initial_state, casm_state!(1, 5, 5));
    }
}
