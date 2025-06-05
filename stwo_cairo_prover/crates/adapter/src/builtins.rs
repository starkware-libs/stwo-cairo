use std::collections::BTreeMap;

use cairo_vm::air_public_input::MemorySegmentAddresses as VMMemorySegmentAddresses;
use cairo_vm::stdlib::collections::HashMap;
use cairo_vm::types::builtin_name::BuiltinName;
use cairo_vm::types::relocatable::MaybeRelocatable;
use serde::{Deserialize, Serialize};
use stwo_cairo_common::prover_types::cpu::Relocatable;
use stwo_cairo_common::prover_types::simd::N_LANES;
use tracing::{span, Level};

pub const ADD_MOD_MEMORY_CELLS: usize = 7;
pub const BITWISE_MEMORY_CELLS: usize = 5;
pub const EC_OP_MEMORY_CELLS: usize = 7;
pub const ECDSA_MEMORY_CELLS: usize = 2;
pub const KECCAK_MEMORY_CELLS: usize = 16;
pub const MUL_MOD_MEMORY_CELLS: usize = 7;
pub const PEDERSEN_MEMORY_CELLS: usize = 3;
pub const POSEIDON_MEMORY_CELLS: usize = 6;
pub const RANGE_CHECK_MEMORY_CELLS: usize = 1;

// Minimal builtins instances per segment, chosen to fit SIMD requirements.
pub const MIN_SEGMENT_SIZE: usize = N_LANES;

/// This is a copy of [`cairo_vm::air_public_input::MemorySegmentAddresses`] struct from cairo_vm.
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct MemorySegmentAddresses {
    pub begin_addr: usize,
    pub stop_ptr: usize,
}

impl From<VMMemorySegmentAddresses> for MemorySegmentAddresses {
    fn from(addresses: VMMemorySegmentAddresses) -> Self {
        MemorySegmentAddresses {
            begin_addr: addresses.begin_addr,
            stop_ptr: addresses.stop_ptr,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct Segment {
    pub start: Relocatable,
    pub length: u32,
}

// TODO(ohadn): change field types in MemorySegmentAddresses to match address type.
/// This struct holds the builtins used in a Cairo program.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BuiltinSegments {
    pub add_mod: Option<Segment>,
    pub bitwise: Option<Segment>,
    pub output: Option<Segment>,
    pub mul_mod: Option<Segment>,
    pub pedersen: Option<Segment>,
    pub poseidon: Option<Segment>,
    pub range_check_bits_96: Option<Segment>,
    pub range_check_bits_128: Option<Segment>,
}

impl BuiltinSegments {
    /// Returns the number of instances for each builtin.
    pub fn get_counts(&self) -> HashMap<BuiltinName, usize> {
        let mut counts = HashMap::new();
        let mut insert_builtin = |builtin_name, segment: &Option<Segment>, n_cells_per_instance| {
            counts.insert(
                builtin_name,
                segment.map(|s| s.length as usize).unwrap_or(0) / n_cells_per_instance,
            );
        };

        insert_builtin(BuiltinName::add_mod, &self.add_mod, ADD_MOD_MEMORY_CELLS);
        insert_builtin(BuiltinName::bitwise, &self.bitwise, BITWISE_MEMORY_CELLS);
        insert_builtin(BuiltinName::mul_mod, &self.mul_mod, MUL_MOD_MEMORY_CELLS);
        insert_builtin(BuiltinName::pedersen, &self.pedersen, PEDERSEN_MEMORY_CELLS);
        insert_builtin(BuiltinName::poseidon, &self.poseidon, POSEIDON_MEMORY_CELLS);
        insert_builtin(
            BuiltinName::range_check,
            &self.range_check_bits_128,
            RANGE_CHECK_MEMORY_CELLS,
        );
        insert_builtin(
            BuiltinName::range_check96,
            &self.range_check_bits_96,
            RANGE_CHECK_MEMORY_CELLS,
        );

        counts
    }

    // Pads the relocatable builtin segments output by the VM to match the size required by Stwo.
    // Assumes and verifies that the segments contain no holes and that their length is divisible by
    // the number of cells per instance.
    pub fn pad_relocatable_builtin_segments(
        relocatable_memory: &mut [Vec<Option<MaybeRelocatable>>],
        builtins_segments: BTreeMap<usize, BuiltinName>,
    ) {
        let _span = span!(Level::INFO, "pad_relocatble_builtin_segments").entered();
        for (segment_index, builtin_name) in builtins_segments {
            let current_buitlin_segment = &mut relocatable_memory[segment_index];

            let original_segment_len = current_buitlin_segment.len();

            if original_segment_len == 0 {
                // Do not pad empty segments.
                continue;
            }

            let cells_per_instance = match builtin_name {
                BuiltinName::add_mod => ADD_MOD_MEMORY_CELLS,
                BuiltinName::bitwise => BITWISE_MEMORY_CELLS,
                BuiltinName::ec_op => EC_OP_MEMORY_CELLS,
                BuiltinName::ecdsa => ECDSA_MEMORY_CELLS,
                BuiltinName::keccak => KECCAK_MEMORY_CELLS,
                BuiltinName::mul_mod => MUL_MOD_MEMORY_CELLS,
                BuiltinName::pedersen => PEDERSEN_MEMORY_CELLS,
                BuiltinName::poseidon => POSEIDON_MEMORY_CELLS,
                BuiltinName::range_check96 => RANGE_CHECK_MEMORY_CELLS,
                BuiltinName::range_check => RANGE_CHECK_MEMORY_CELLS,
                _ => panic!("Invalid builtin name"),
            };
            assert!(
                original_segment_len % cells_per_instance == 0,
                "builtin segment: {} size is {}, which is not divisble by {}",
                builtin_name,
                original_segment_len,
                cells_per_instance
            );

            if !current_buitlin_segment.iter().all(|x| x.is_some()) {
                panic!(
                    "Builtins segments '{}' at segment index: {}, contains a hole.",
                    builtin_name, segment_index
                );
            }

            // Builtin segment size is extended to the next power of two instances.
            let new_segment_size = original_segment_len
                .div_ceil(cells_per_instance)
                .next_power_of_two()
                .max(MIN_SEGMENT_SIZE)
                * cells_per_instance;
            current_buitlin_segment.resize(new_segment_size, None);

            // Fill the segment extension with copies of the last instance.
            let last_instance_start = original_segment_len - cells_per_instance;
            for i in original_segment_len..new_segment_size {
                current_buitlin_segment[i] =
                    current_buitlin_segment[last_instance_start + (i % cells_per_instance)].clone();
            }
        }
    }

    pub fn get_builtin_segments(
        builtins_segments_indices: &BTreeMap<usize, BuiltinName>,
        relocatable_memory: &Vec<Vec<Option<MaybeRelocatable>>>,
    ) -> BuiltinSegments {
        let _span = span!(Level::INFO, "get_builtin_segments").entered();
        let mut res = BuiltinSegments::default();
        for (segment_index, builtin_name) in builtins_segments_indices.iter() {
            let segment = if relocatable_memory[*segment_index].len() == 0 {
                None
            } else {
                Some(Segment {
                    start: Relocatable {
                        segment_index: *segment_index,
                        offset: 0,
                    },
                    length: relocatable_memory[*segment_index].len() as u32,
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
}
