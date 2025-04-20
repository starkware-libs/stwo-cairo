use std::collections::BTreeMap;

use cairo_vm::air_public_input::MemorySegmentAddresses as VMMemorySegmentAddresses;
use cairo_vm::stdlib::collections::HashMap;
use cairo_vm::types::builtin_name::BuiltinName;
use cairo_vm::types::relocatable::MaybeRelocatable;
use serde::{Deserialize, Serialize};
use stwo_cairo_common::prover_types::simd::N_LANES;

use super::memory::MemoryBuilder;

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

// TODO(ohadn): change field types in MemorySegmentAddresses to match address type.
/// This struct holds the builtins used in a Cairo program.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BuiltinSegments {
    pub add_mod: Option<MemorySegmentAddresses>,
    pub bitwise: Option<MemorySegmentAddresses>,
    pub output: Option<MemorySegmentAddresses>,
    pub mul_mod: Option<MemorySegmentAddresses>,
    pub pedersen: Option<MemorySegmentAddresses>,
    pub poseidon: Option<MemorySegmentAddresses>,
    pub range_check_bits_96: Option<MemorySegmentAddresses>,
    pub range_check_bits_128: Option<MemorySegmentAddresses>,
}

impl BuiltinSegments {
    /// Creates a new `BuiltinSegments` struct from a map of memory segment names to addresses.
    /// TODO(Stav): remove after using 'get_builtins_segment'.
    pub fn from_memory_segments(memory_segments: &HashMap<&str, MemorySegmentAddresses>) -> Self {
        let mut res = BuiltinSegments::default();
        for (name, value) in memory_segments.iter() {
            if let Some(builtin_name) = BuiltinName::from_str(name) {
                // Filter empty segments.
                let segment = if value.begin_addr == value.stop_ptr {
                    None
                } else {
                    assert!(
                        value.begin_addr < value.stop_ptr,
                        "Invalid segment addresses: '{}'-'{}' for builtin '{name}'",
                        value.begin_addr,
                        value.stop_ptr,
                    );
                    Some(*value)
                };
                match builtin_name {
                    BuiltinName::range_check => res.range_check_bits_128 = segment,
                    BuiltinName::pedersen => res.pedersen = segment,
                    BuiltinName::bitwise => res.bitwise = segment,
                    BuiltinName::poseidon => res.poseidon = segment,
                    BuiltinName::range_check96 => res.range_check_bits_96 = segment,
                    BuiltinName::add_mod => res.add_mod = segment,
                    BuiltinName::mul_mod => res.mul_mod = segment,
                    BuiltinName::output => res.output = segment,
                    BuiltinName::ec_op | BuiltinName::keccak | BuiltinName::ecdsa => {
                        assert!(segment.is_none(), "{} builtin is not supported", name);
                    }
                    // Not builtins.
                    BuiltinName::segment_arena => {}
                }
            };
        }
        res
    }

    /// Returns the number of instances for each builtin.
    pub fn get_counts(&self) -> HashMap<BuiltinName, usize> {
        let mut counts = HashMap::new();
        let mut insert_builtin = |builtin_name, segment: &Option<_>, n_cells_per_instance| {
            counts.insert(
                builtin_name,
                segment.as_ref().map(get_memory_segment_size).unwrap_or(0) / n_cells_per_instance,
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

    /// Pads every builtin segment to the next power of 2, using copies of its last instance.
    ///
    /// # Panics
    /// - if the remainder for the next power of 2 is not empty.
    // Note: the last instance was already verified as valid by the VM and in the case of add_mod
    // and mul_mod, security checks have verified that instance has n=1. Thus the padded segment
    // satisfies all the AIR constraints.
    // TODO (ohadn): relocate this function if a more appropriate place is found.
    // TODO (Stav): remove when the new adapter flow is used.
    pub fn pad_builtin_segments(&mut self, memory: &mut MemoryBuilder) {
        if let Some(segment) = &self.add_mod {
            self.add_mod = Some(pad_segment(
                segment,
                memory,
                ADD_MOD_MEMORY_CELLS as u32,
                Some("add_mod"),
            ));
        }
        if let Some(segment) = &self.bitwise {
            self.bitwise = Some(pad_segment(
                segment,
                memory,
                BITWISE_MEMORY_CELLS as u32,
                Some("bitwise"),
            ));
        }
        if let Some(segment) = &self.mul_mod {
            self.mul_mod = Some(pad_segment(
                segment,
                memory,
                MUL_MOD_MEMORY_CELLS as u32,
                Some("mul_mod"),
            ));
        }
        if let Some(segment) = &self.pedersen {
            self.pedersen = Some(pad_segment(
                segment,
                memory,
                PEDERSEN_MEMORY_CELLS as u32,
                Some("pedersen"),
            ));
        }
        if let Some(segment) = &self.poseidon {
            self.poseidon = Some(pad_segment(
                segment,
                memory,
                POSEIDON_MEMORY_CELLS as u32,
                Some("poseidon"),
            ));
        }
        if let Some(segment) = &self.range_check_bits_96 {
            self.range_check_bits_96 = Some(pad_segment(
                segment,
                memory,
                RANGE_CHECK_MEMORY_CELLS as u32,
                Some("range_check_96"),
            ));
        }
        if let Some(segment) = &self.range_check_bits_128 {
            self.range_check_bits_128 = Some(pad_segment(
                segment,
                memory,
                RANGE_CHECK_MEMORY_CELLS as u32,
                Some("range_check_128"),
            ));
        }
    }

    // Pads the relocatable builtin segments output by the VM to match the size required by Stwo.
    // Assumes and verifies that the segments contain no holes and that their length is divisible by
    // the number of cells per instance.
    pub fn pad_relocatble_builtin_segments(
        relocatable_memory: &mut [Vec<Option<MaybeRelocatable>>],
        builtins_segments: BTreeMap<usize, BuiltinName>,
    ) {
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

    /// Fills memory cells in builtin segments with the appropriate values according to the builtin.
    ///
    /// The memory provided by the runner only contains values that were accessed during program
    /// execution. However, the builtin AIR applies constraints on it's entire range, including
    /// addresses that were not accessed.
    // TODO(Stav): remove when this is fixed by the runner.
    pub fn fill_memory_holes(&self, memory: &mut MemoryBuilder) {
        self.resize_memory_to_cover_holes(memory);
        // bitwise.
        if let Some(segment) = &self.bitwise {
            builtin_padding::bitwise(segment, memory)
        };
    }

    // If the final segment in a builtin segment, and the final entry has a hole, the memory must be
    // resized to include the hole.
    // TODO (Stav): remove when the new adapter is used.
    fn resize_memory_to_cover_holes(&self, memory: &mut MemoryBuilder) {
        let max_stop_ptr = [
            self.add_mod.as_ref(),
            self.bitwise.as_ref(),
            self.mul_mod.as_ref(),
            self.pedersen.as_ref(),
            self.poseidon.as_ref(),
            self.range_check_bits_96.as_ref(),
            self.range_check_bits_128.as_ref(),
        ]
        .iter()
        .filter_map(|segment| segment.map(|s| s.stop_ptr))
        .max()
        .unwrap_or(0);
        let len = memory.address_to_id.len();
        memory
            .address_to_id
            .resize(std::cmp::max(len, max_stop_ptr), Default::default());
    }
}

/// The minimum number of instances in a builtin segment supported by the prover. This must be a
/// power of 2.
const MIN_N_INSTANCES_IN_BUILTIN_SEGMENT: u32 = 16;

// Assert MIN_N_INSTANCES_IN_BUILTIN_SEGMENT is a power of 2.
const _: () = assert!(MIN_N_INSTANCES_IN_BUILTIN_SEGMENT.is_power_of_two());

/// Pads the given segment to the next power of 2, and at least MIN_N_INSTANCES_IN_BUILTIN_SEGMENT
/// (in number of instances).
// TODO (Stav): remove when the new adapter is used.
fn pad_segment(
    MemorySegmentAddresses {
        begin_addr,
        stop_ptr,
    }: &MemorySegmentAddresses,
    mem: &mut MemoryBuilder,
    n_cells_per_instance: u32,
    segment_name: Option<&str>,
) -> MemorySegmentAddresses {
    let (begin_addr, stop_ptr) = (*begin_addr as u32, *stop_ptr as u32);
    let initial_length = stop_ptr - begin_addr;
    assert!(initial_length > 0);
    assert!(initial_length % n_cells_per_instance == 0);
    let num_instances = initial_length / n_cells_per_instance;
    let padded_size = std::cmp::max(
        MIN_N_INSTANCES_IN_BUILTIN_SEGMENT,
        num_instances.next_power_of_two(),
    );

    let segment_name = segment_name.unwrap_or("<unnamed>");
    log::info!("Padding segment {segment_name} of size {num_instances} to size {padded_size}");

    // Verify that the segment we intend to pad is empty.
    mem.assert_segment_is_empty(
        stop_ptr,
        (padded_size - num_instances) * n_cells_per_instance,
    );

    let mut instance_to_fill_start = stop_ptr;
    let last_instance_start = stop_ptr - n_cells_per_instance;
    for _ in num_instances..padded_size {
        mem.copy_block(
            last_instance_start,
            instance_to_fill_start,
            n_cells_per_instance,
        );
        instance_to_fill_start += n_cells_per_instance;
    }
    let stop_ptr = begin_addr + n_cells_per_instance * padded_size;
    MemorySegmentAddresses {
        begin_addr: begin_addr as usize,
        stop_ptr: stop_ptr as usize,
    }
}

/// Return the size of a memory segment.
fn get_memory_segment_size(segment: &MemorySegmentAddresses) -> usize {
    segment.stop_ptr - segment.begin_addr
}

mod builtin_padding {
    use itertools::Itertools;

    use super::MemorySegmentAddresses;
    use crate::builtins::BITWISE_MEMORY_CELLS;
    use crate::memory::{value_from_felt252, MemoryBuilder, MemoryValueId};

    pub fn bitwise(segment: &MemorySegmentAddresses, memory: &mut MemoryBuilder) {
        let range = segment.begin_addr as u32..segment.stop_ptr as u32;
        assert!(range.len() % BITWISE_MEMORY_CELLS == 0);
        for (op0_addr, op1_addr, and_addr, xor_addr, or_addr) in range.tuples() {
            let op0 = memory.get(op0_addr).as_u256();
            let op1 = memory.get(op1_addr).as_u256();

            if let MemoryValueId::Empty = memory.address_to_id[and_addr as usize].decode() {
                let and_res = value_from_felt252(std::array::from_fn(|i| op0[i] & op1[i]));
                memory.set(and_addr, and_res);
            }

            if let MemoryValueId::Empty = memory.address_to_id[xor_addr as usize].decode() {
                let xor_res = value_from_felt252(std::array::from_fn(|i| op0[i] ^ op1[i]));
                memory.set(xor_addr, xor_res);
            }

            if let MemoryValueId::Empty = memory.address_to_id[or_addr as usize].decode() {
                let or_res = value_from_felt252(std::array::from_fn(|i| op0[i] | op1[i]));
                memory.set(or_addr, or_res);
            }
        }
    }
}

// TODO(Stav): move read json to a test function.
#[cfg(test)]
mod test_builtin_segments {
    use std::path::PathBuf;

    use cairo_vm::air_public_input::PublicInput;
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use test_case::test_case;

    use super::*;
    use crate::builtins::BITWISE_MEMORY_CELLS;
    use crate::memory::{
        u128_to_4_limbs, Memory, MemoryBuilder, MemoryConfig, MemoryEntry, MemoryValue,
    };

    /// Asserts that the values at addresses start_addr1 to start_addr1 + segment_length - 1
    /// are equal to values at the addresses start_addr2 to start_addr2 + segment_length - 1.
    pub fn assert_identical_blocks(
        memory: &Memory,
        start_addr1: u32,
        start_addr2: u32,
        segment_length: u32,
    ) {
        for i in 0..segment_length {
            assert_eq!(memory.get(start_addr1 + i), memory.get(start_addr2 + i));
        }
    }

    #[test]
    fn test_builtin_segments() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../test_data/test_builtins_segments/test_public_input.json");
        let pub_data_string = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Unable to read file: {}", path.display()));
        let pub_data: PublicInput<'_> =
            serde_json::from_str(&pub_data_string).expect("Unable to parse JSON");

        let builtin_segments = BuiltinSegments::from_memory_segments(
            &pub_data
                .memory_segments
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        );
        assert_eq!(builtin_segments.add_mod, None);
        assert_eq!(
            builtin_segments.bitwise,
            Some(MemorySegmentAddresses {
                begin_addr: 23581,
                stop_ptr: 23901
            })
        );
        assert_eq!(builtin_segments.mul_mod, None);
        assert_eq!(builtin_segments.pedersen, None);
        assert_eq!(builtin_segments.poseidon, None);
        assert_eq!(builtin_segments.range_check_bits_96, None);
        assert_eq!(
            builtin_segments.range_check_bits_128,
            Some(MemorySegmentAddresses {
                begin_addr: 7069,
                stop_ptr: 7187
            })
        );
    }

    /// Initializes a memory builder with the given u128 values.
    /// Places the value instance_example[i] at the address memory_write_start + i.
    fn initialize_memory(memory_write_start: u32, instance_example: &[u128]) -> MemoryBuilder {
        let memory_config = MemoryConfig::default();
        let mut memory_builder = MemoryBuilder::new(memory_config.clone());
        for (i, &value) in instance_example.iter().enumerate() {
            let memory_value = if value <= memory_config.small_max {
                MemoryValue::Small(value)
            } else {
                let x = u128_to_4_limbs(value);
                MemoryValue::F252([x[0], x[1], x[2], x[3], 0, 0, 0, 0])
            };
            memory_builder.set(memory_write_start + i as u32, memory_value);
        }
        memory_builder
    }

    #[test_case(71, 128; "71->128")]
    #[test_case(64, 64; "64->64")]
    #[test_case(11, 16; "11->16")]
    #[test_case(16, 16; "16->16")]
    #[test_case(7, 16; "7->16")]
    #[test_case(1, 16; "1->16")]
    fn test_pad_builtin_segments(num_instances: usize, padded_num_instances: usize) {
        let instance_example = [
            123456789,
            4385067362534966725237889432551,
            50448645,
            4385067362534966725237911992050,
            4385067362534966725237962440695,
        ];
        let mut builtin_segments = BuiltinSegments::default();
        let cells_per_instance = BITWISE_MEMORY_CELLS;
        assert_eq!(cells_per_instance, instance_example.len());
        let begin_addr = 23581;
        let stop_ptr = begin_addr + cells_per_instance * num_instances;
        builtin_segments.bitwise = Some(MemorySegmentAddresses {
            begin_addr,
            stop_ptr,
        });
        let memory_write_start = (stop_ptr - cells_per_instance) as u32;
        let mut memory_builder = initialize_memory(memory_write_start, &instance_example);

        builtin_segments.pad_builtin_segments(&mut memory_builder);

        let &MemorySegmentAddresses {
            begin_addr: new_begin_addr,
            stop_ptr: new_stop_ptr,
        } = builtin_segments.bitwise.as_ref().unwrap();
        assert_eq!(new_begin_addr, begin_addr);
        let segment_length = new_stop_ptr - new_begin_addr;
        assert_eq!(segment_length % cells_per_instance, 0);
        let new_num_instances = segment_length / cells_per_instance;
        assert_eq!(new_num_instances, padded_num_instances);

        let (memory, ..) = memory_builder.build();
        assert_eq!(memory.address_to_id.len(), new_stop_ptr);

        let mut instance_to_verify_start = stop_ptr as u32;
        let last_instance_start = (stop_ptr - cells_per_instance) as u32;
        for _ in num_instances..new_num_instances {
            assert_identical_blocks(
                &memory,
                last_instance_start,
                instance_to_verify_start,
                cells_per_instance as u32,
            );
            instance_to_verify_start += cells_per_instance as u32;
        }
    }

    #[test]
    fn test_bitwise_hole_filling() {
        let mut small_rng = SmallRng::seed_from_u64(0);
        let op0 = small_rng.gen::<[u32; 8]>();
        let op1 = small_rng.gen::<[u32; 8]>();
        let entries = [
            MemoryEntry {
                address: 0,
                value: op0,
            },
            MemoryEntry {
                address: 1,
                value: op1,
            },
        ];
        let mut memory = MemoryBuilder::from_iter(MemoryConfig::default(), entries);
        let builtin_segments = BuiltinSegments {
            bitwise: Some(MemorySegmentAddresses {
                begin_addr: 0,
                stop_ptr: 5,
            }),
            ..Default::default()
        };
        let expected_and = std::array::from_fn(|i| op0[i] & op1[i]);
        let expected_xor = std::array::from_fn(|i| op0[i] ^ op1[i]);
        let expected_or = std::array::from_fn(|i| op0[i] | op1[i]);
        builtin_segments.fill_memory_holes(&mut memory);
        let (memory, ..) = memory.build();

        let and_res = memory.get(2).as_u256();
        let xor_res = memory.get(3).as_u256();
        let or_res = memory.get(4).as_u256();

        assert_eq!(and_res, expected_and);
        assert_eq!(xor_res, expected_xor);
        assert_eq!(or_res, expected_or);
    }

    #[test]
    fn test_resize_memory_to_cover_holes() {
        let entries = [
            MemoryEntry {
                address: 0,
                value: [0; 8],
            },
            MemoryEntry {
                address: 1,
                value: [1; 8],
            },
        ];
        let mut memory = MemoryBuilder::from_iter(MemoryConfig::default(), entries);
        let builtin_segments = BuiltinSegments {
            bitwise: Some(MemorySegmentAddresses {
                begin_addr: 0,
                stop_ptr: 5,
            }),
            ..Default::default()
        };
        assert_eq!(memory.address_to_id.len(), 2);
        builtin_segments.resize_memory_to_cover_holes(&mut memory);
        assert_eq!(memory.address_to_id.len(), 5);
    }

    #[test]
    fn test_pad_relocatble_builtin_segments() {
        let bitwise_instance = [787, 365, 257, 895, 638, 55, 102, 3, 4, 7];
        let segment0 = bitwise_instance
            .iter()
            .map(|&x| Some(MaybeRelocatable::Int(x.into())))
            .collect::<Vec<_>>();
        let segment0_extended = segment0
            .clone()
            .into_iter()
            .cycle()
            .take(18 * 5)
            .collect::<Vec<_>>();

        let ec_op_instance = [1, 2, 3, 4, 5, 6, 7];
        let segment1 = ec_op_instance
            .iter()
            .map(|&x| Some(MaybeRelocatable::Int(x.into())))
            .collect::<Vec<_>>();

        let non_builtin_segment = vec![Some(MaybeRelocatable::Int(5.into())); 10];

        let mut relocatable_memory = [segment0_extended, segment1.clone(), non_builtin_segment];

        let builtins_segments =
            BTreeMap::from([(0, BuiltinName::bitwise), (1, BuiltinName::ec_op)]);

        BuiltinSegments::pad_relocatble_builtin_segments(
            &mut relocatable_memory,
            builtins_segments,
        );

        assert!(relocatable_memory[0].len() == 32 * BITWISE_MEMORY_CELLS);
        assert!(relocatable_memory[1].len() == 16 * EC_OP_MEMORY_CELLS);
        assert!(relocatable_memory[2].len() == 10);

        // Check that the values in the extended segment are identical to the original segment.
        let last_bitwise_instance = &segment0[5..10];
        for (value, expected_value) in relocatable_memory[0]
            .iter()
            .skip(17 * 5)
            .zip(last_bitwise_instance.iter().cycle())
        {
            assert_eq!(value, expected_value);
        }
        for (value, expected_value) in relocatable_memory[1].iter().zip(segment1.iter().cycle()) {
            assert_eq!(value, expected_value);
        }
    }

    #[test]
    #[should_panic(
        expected = "builtin segment: mul_mod_builtin size is 864, which is not divisble by 7"
    )]
    fn test_builtin_segment_invalid_size() {
        let mul_mod_segment =
            vec![Some(MaybeRelocatable::Int(7.into())); MUL_MOD_MEMORY_CELLS * 123 + 3];
        let mut relocatable_memory = [mul_mod_segment];
        let builtins_segments = BTreeMap::from([(0, BuiltinName::mul_mod)]);

        BuiltinSegments::pad_relocatble_builtin_segments(
            &mut relocatable_memory,
            builtins_segments,
        );
    }

    #[test]
    #[should_panic(
        expected = "Builtins segments 'bitwise_builtin' at segment index: 0, contains a hole."
    )]
    fn test_builtin_segment_contains_hole() {
        let mut segment0 = vec![Some(MaybeRelocatable::Int(1.into())); 80];
        segment0[62] = None;

        let mut relocatable_memory = vec![segment0];
        let builtins_segments = BTreeMap::from([(0, BuiltinName::bitwise)]);

        BuiltinSegments::pad_relocatble_builtin_segments(
            &mut relocatable_memory,
            builtins_segments,
        );
    }
}
