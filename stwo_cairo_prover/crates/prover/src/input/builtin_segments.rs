use cairo_vm::air_public_input::MemorySegmentAddresses;
use cairo_vm::stdlib::collections::HashMap;
use cairo_vm::types::builtin_name::BuiltinName;
use serde::{Deserialize, Serialize};

use super::memory::MemoryBuilder;

const RANGE_CHECK_MEMORY_CELLS: usize = 1;
const PEDERSEN_MEMORY_CELLS: usize = 3;
const ECDSA_MEMORY_CELLS: usize = 2;
const KECCAK_MEMORY_CELLS: usize = 16;
const BITWISE_MEMORY_CELLS: usize = 5;
const EC_OP_MEMORY_CELLS: usize = 7;
const POSEIDON_MEMORY_CELLS: usize = 6;
const ADD_MOD_MEMORY_CELLS: usize = 7;
const MUL_MOD_MEMORY_CELLS: usize = 7;

// TODO(ohadn): change field types in MemorySegmentAddresses to match address type.
/// This struct holds the builtins used in a Cairo program.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BuiltinSegments {
    pub add_mod: Option<MemorySegmentAddresses>,
    pub bitwise: Option<MemorySegmentAddresses>,
    pub ec_op: Option<MemorySegmentAddresses>,
    pub ecdsa: Option<MemorySegmentAddresses>,
    pub keccak: Option<MemorySegmentAddresses>,
    pub mul_mod: Option<MemorySegmentAddresses>,
    pub pedersen: Option<MemorySegmentAddresses>,
    pub poseidon: Option<MemorySegmentAddresses>,
    pub range_check_bits_96: Option<MemorySegmentAddresses>,
    pub range_check_bits_128: Option<MemorySegmentAddresses>,
}

impl BuiltinSegments {
    /// Creates a new `BuiltinSegments` struct from a map of memory segment names to addresses.
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
                    Some((value.begin_addr, value.stop_ptr).into())
                };
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
                }
            };
        }
        res
    }

    /// Returns the number of memory cells per instance for a given builtin name.
    pub fn builtin_memory_cells_per_instance(builtin_name: BuiltinName) -> usize {
        match builtin_name {
            BuiltinName::range_check => RANGE_CHECK_MEMORY_CELLS,
            BuiltinName::pedersen => PEDERSEN_MEMORY_CELLS,
            BuiltinName::ecdsa => ECDSA_MEMORY_CELLS,
            BuiltinName::keccak => KECCAK_MEMORY_CELLS,
            BuiltinName::bitwise => BITWISE_MEMORY_CELLS,
            BuiltinName::ec_op => EC_OP_MEMORY_CELLS,
            BuiltinName::poseidon => POSEIDON_MEMORY_CELLS,
            BuiltinName::range_check96 => RANGE_CHECK_MEMORY_CELLS,
            BuiltinName::add_mod => ADD_MOD_MEMORY_CELLS,
            BuiltinName::mul_mod => MUL_MOD_MEMORY_CELLS,
            // Not builtins.
            BuiltinName::output | BuiltinName::segment_arena => 0,
        }
    }

    /// Returns the number of instances for each builtin.
    pub fn get_counts(&self) -> HashMap<BuiltinName, usize> {
        let mut counts = HashMap::new();
        let mut insert = |builtin_name, segment: &Option<_>, n_cells_per_instance| {
            counts.insert(
                builtin_name,
                segment.as_ref().map(get_memory_segment_size).unwrap_or(0) / n_cells_per_instance,
            );
        };

        insert(BuiltinName::pedersen, &self.pedersen, PEDERSEN_MEMORY_CELLS);
        insert(BuiltinName::ecdsa, &self.ecdsa, ECDSA_MEMORY_CELLS);
        insert(BuiltinName::keccak, &self.keccak, KECCAK_MEMORY_CELLS);
        insert(BuiltinName::bitwise, &self.bitwise, BITWISE_MEMORY_CELLS);
        insert(BuiltinName::ec_op, &self.ec_op, EC_OP_MEMORY_CELLS);
        insert(BuiltinName::poseidon, &self.poseidon, POSEIDON_MEMORY_CELLS);
        insert(BuiltinName::add_mod, &self.add_mod, ADD_MOD_MEMORY_CELLS);
        insert(BuiltinName::mul_mod, &self.mul_mod, MUL_MOD_MEMORY_CELLS);
        insert(
            BuiltinName::range_check,
            &self.range_check_bits_128,
            RANGE_CHECK_MEMORY_CELLS,
        );
        insert(
            BuiltinName::range_check96,
            &self.range_check_bits_96,
            RANGE_CHECK_MEMORY_CELLS,
        );

        counts
    }

    /// Pads a builtin segment to the next power of 2, using copies of its last instance.
    ///
    /// # Panics
    /// - if the remainder for the next power of 2 is not empty.
    // Note: the last instance was already verified as valid by the VM and in the case of add_mod
    // and mul_mod, security checks have verified that instance has n=1. Thus the padded segment
    // satisfies all the AIR constraints.
    // TODO (ohadn): relocate this function if a more appropriate place is found.
    pub fn pad_builtin_segments(&mut self, memory: &mut MemoryBuilder) {
        if let Some(segment) = &self.add_mod {
            self.add_mod = Some(pad_segment(segment, memory, ADD_MOD_MEMORY_CELLS as u32));
        }
        if let Some(segment) = &self.bitwise {
            self.bitwise = Some(pad_segment(segment, memory, BITWISE_MEMORY_CELLS as u32));
        }
        if let Some(segment) = &self.ec_op {
            self.ec_op = Some(pad_segment(segment, memory, EC_OP_MEMORY_CELLS as u32));
        }
        if let Some(segment) = &self.ecdsa {
            self.ecdsa = Some(pad_segment(segment, memory, ECDSA_MEMORY_CELLS as u32));
        }
        if let Some(segment) = &self.keccak {
            self.keccak = Some(pad_segment(segment, memory, KECCAK_MEMORY_CELLS as u32));
        }
        if let Some(segment) = &self.mul_mod {
            self.mul_mod = Some(pad_segment(segment, memory, MUL_MOD_MEMORY_CELLS as u32));
        }
        if let Some(segment) = &self.pedersen {
            self.pedersen = Some(pad_segment(segment, memory, PEDERSEN_MEMORY_CELLS as u32));
        }
        if let Some(segment) = &self.poseidon {
            self.poseidon = Some(pad_segment(segment, memory, POSEIDON_MEMORY_CELLS as u32));
        }
        if let Some(segment) = &self.range_check_bits_96 {
            self.range_check_bits_96 = Some(pad_segment(
                segment,
                memory,
                RANGE_CHECK_MEMORY_CELLS as u32,
            ));
        }
        if let Some(segment) = &self.range_check_bits_128 {
            self.range_check_bits_128 = Some(pad_segment(
                segment,
                memory,
                RANGE_CHECK_MEMORY_CELLS as u32,
            ));
        }
    }

    /// Fills memory cells in builtin segments with the appropriate values according to the builtin.
    ///
    /// The memory provided by the runner only contains values that were accessed during program
    /// execution. However, the builtin AIR applies constraints on it's entire range, including
    /// addresses that were not accessed.
    pub fn fill_memory_holes(&self, memory: &mut MemoryBuilder) {
        // bitwise.
        if let Some(segment) = &self.bitwise {
            builtin_padding::bitwise(segment, memory)
        };
        // TODO(ohad): fill other builtins.
    }
}

fn pad_segment(
    MemorySegmentAddresses {
        begin_addr,
        stop_ptr,
    }: &MemorySegmentAddresses,
    mem: &mut MemoryBuilder,
    n_cells_per_instance: u32,
) -> MemorySegmentAddresses {
    let (begin_addr, stop_ptr) = (*begin_addr as u32, *stop_ptr as u32);
    let initial_length = stop_ptr - begin_addr;
    assert!(initial_length > 0);
    assert!(initial_length % n_cells_per_instance == 0);
    let num_instances = initial_length / n_cells_per_instance;
    let next_power_of_two = num_instances.next_power_of_two();

    // Verify that the segment we intend to pad is empty.
    mem.verify_empty_segment(
        stop_ptr,
        (next_power_of_two - num_instances) * n_cells_per_instance,
    );

    let mut instance_to_fill_start = stop_ptr;
    let last_instance_start = stop_ptr - n_cells_per_instance;
    for _ in num_instances..next_power_of_two {
        mem.copy_block(
            last_instance_start,
            instance_to_fill_start,
            n_cells_per_instance,
        );
        instance_to_fill_start += n_cells_per_instance;
    }
    let stop_ptr = begin_addr + n_cells_per_instance * next_power_of_two;
    MemorySegmentAddresses {
        begin_addr: begin_addr as usize,
        stop_ptr: stop_ptr as usize,
    }
}

/// Return the size of a memory segment.
fn get_memory_segment_size(segment: &MemorySegmentAddresses) -> usize {
    segment.stop_ptr - segment.begin_addr
}

// TODO(Ohad): move.
mod builtin_padding {
    use cairo_vm::air_public_input::MemorySegmentAddresses;
    use itertools::Itertools;

    use crate::input::builtin_segments::BITWISE_MEMORY_CELLS;
    use crate::input::memory::{Memory, MemoryBuilder};

    pub fn bitwise(segment: &MemorySegmentAddresses, memory: &mut MemoryBuilder) {
        let range = segment.begin_addr as u32..segment.stop_ptr as u32;
        assert!(range.len() % BITWISE_MEMORY_CELLS == 0);
        for (op0_addr, op1_addr, and_addr, xor_addr, or_addr) in range.tuples() {
            let op0 = memory.get(op0_addr).as_u256();
            let op1 = memory.get(op1_addr).as_u256();

            let and_res = Memory::value_from_felt252(std::array::from_fn(|i| op0[i] & op1[i]));
            let xor_res = Memory::value_from_felt252(std::array::from_fn(|i| op0[i] ^ op1[i]));
            let or_res = Memory::value_from_felt252(std::array::from_fn(|i| op0[i] | op1[i]));

            memory.set(and_addr as u64, and_res);
            memory.set(xor_addr as u64, xor_res);
            memory.set(or_addr as u64, or_res);
        }
    }
}

// TODO(Stav): move read json to a test function.
#[cfg(test)]
mod test_builtin_segments {
    use std::path::PathBuf;

    use cairo_vm::air_public_input::{MemorySegmentAddresses, PublicInput};
    use cairo_vm::types::builtin_name::BuiltinName;

    use crate::input::memory::{u128_to_4_limbs, Memory, MemoryBuilder, MemoryConfig, MemoryValue};
    use crate::input::BuiltinSegments;

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
            .join("src/input/test_builtins_segments/test_public_input.json");
        let pub_data_string = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Unable to read file: {}", path.display()));
        let pub_data: PublicInput<'_> =
            serde_json::from_str(&pub_data_string).expect("Unable to parse JSON");

        let builtin_segments = BuiltinSegments::from_memory_segments(&pub_data.memory_segments);
        assert_eq!(builtin_segments.add_mod, None);
        assert_eq!(builtin_segments.bitwise, Some((23581, 23901).into()));
        assert_eq!(builtin_segments.ec_op, None);
        assert_eq!(builtin_segments.ecdsa, None);
        assert_eq!(builtin_segments.keccak, None);
        assert_eq!(builtin_segments.mul_mod, None);
        assert_eq!(builtin_segments.pedersen, None);
        assert_eq!(builtin_segments.poseidon, None);
        assert_eq!(builtin_segments.range_check_bits_96, None);
        assert_eq!(
            builtin_segments.range_check_bits_128,
            Some((7069, 7187).into())
        );
    }

    /// Initializes a memory builder with the given u128 values.
    /// Places the value instance_example[i] at the address memory_write_start + i.
    fn initialize_memory(memory_write_start: u64, instance_example: &[u128]) -> MemoryBuilder {
        let memory_config = MemoryConfig::default();
        let mut memory_builder = MemoryBuilder::new(memory_config.clone());
        for (i, &value) in instance_example.iter().enumerate() {
            let memory_value = if value <= memory_config.small_max {
                MemoryValue::Small(value)
            } else {
                let x = u128_to_4_limbs(value);
                MemoryValue::F252([x[0], x[1], x[2], x[3], 0, 0, 0, 0])
            };
            memory_builder.set(memory_write_start + i as u64, memory_value);
        }
        memory_builder
    }

    #[test]
    fn test_fill_builtin_segment() {
        let builtin_name = BuiltinName::bitwise;
        let instance_example = [
            123456789,
            4385067362534966725237889432551,
            50448645,
            4385067362534966725237911992050,
            4385067362534966725237962440695,
        ];
        let mut builtin_segments = BuiltinSegments::default();
        let cells_per_instance = BuiltinSegments::builtin_memory_cells_per_instance(builtin_name);
        assert_eq!(cells_per_instance, instance_example.len());
        let num_instances = 71;
        let begin_addr = 23581;
        let stop_ptr = begin_addr + cells_per_instance * num_instances;
        builtin_segments.bitwise = Some(MemorySegmentAddresses {
            begin_addr,
            stop_ptr,
        });
        let memory_write_start = (stop_ptr - cells_per_instance) as u64;
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
        assert_eq!(new_num_instances, 128);

        let memory = memory_builder.build();
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
}
