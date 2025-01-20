use cairo_vm::air_public_input::MemorySegmentAddresses;
use cairo_vm::stdlib::collections::HashMap;
use cairo_vm::types::builtin_name::BuiltinName;
use serde::{Deserialize, Serialize};

use super::memory::MemoryBuilder;

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
    /// Sets a segment in the builtin segments.
    /// If a segment already exists for the given builtin name, it will be overwritten.
    fn set_segment(&mut self, builtin_name: BuiltinName, segment: Option<MemorySegmentAddresses>) {
        match builtin_name {
            BuiltinName::range_check => self.range_check_bits_128 = segment,
            BuiltinName::pedersen => self.pedersen = segment,
            BuiltinName::ecdsa => self.ecdsa = segment,
            BuiltinName::keccak => self.keccak = segment,
            BuiltinName::bitwise => self.bitwise = segment,
            BuiltinName::ec_op => self.ec_op = segment,
            BuiltinName::poseidon => self.poseidon = segment,
            BuiltinName::range_check96 => self.range_check_bits_96 = segment,
            BuiltinName::add_mod => self.add_mod = segment,
            BuiltinName::mul_mod => self.mul_mod = segment,
            // Not builtins.
            BuiltinName::output | BuiltinName::segment_arena => {}
        }
    }

    // TODO(ohadn): change return type to non reference once MemorySegmentAddresses implements
    // clone.
    // TODO(ohadn): change output type to match address type.
    /// Returns the segment for a given builtin name.
    fn get_segment(&self, builtin_name: BuiltinName) -> &Option<MemorySegmentAddresses> {
        match builtin_name {
            BuiltinName::range_check => &self.range_check_bits_128,
            BuiltinName::pedersen => &self.pedersen,
            BuiltinName::ecdsa => &self.ecdsa,
            BuiltinName::keccak => &self.keccak,
            BuiltinName::bitwise => &self.bitwise,
            BuiltinName::ec_op => &self.ec_op,
            BuiltinName::poseidon => &self.poseidon,
            BuiltinName::range_check96 => &self.range_check_bits_96,
            BuiltinName::add_mod => &self.add_mod,
            BuiltinName::mul_mod => &self.mul_mod,
            // Not builtins.
            BuiltinName::output | BuiltinName::segment_arena => &None,
        }
    }

    /// Returns the number of memory cells per instance for a given builtin name.
    pub fn builtin_memory_cells_per_instance(builtin_name: BuiltinName) -> usize {
        match builtin_name {
            BuiltinName::range_check => 1,
            BuiltinName::pedersen => 3,
            BuiltinName::ecdsa => 2,
            BuiltinName::keccak => 16,
            BuiltinName::bitwise => 5,
            BuiltinName::ec_op => 7,
            BuiltinName::poseidon => 6,
            BuiltinName::range_check96 => 1,
            BuiltinName::add_mod => 7,
            BuiltinName::mul_mod => 7,
            // Not builtins.
            BuiltinName::output | BuiltinName::segment_arena => 0,
        }
    }

    /// Pads a builtin segment with copies of its last instance if that segment isn't None, in
    /// which case at least one instance is guaranteed to exist.
    /// The segment is padded to the next power of 2 number of instances.
    /// Note: the last instance was already verified as valid by the VM and in the case of add_mod
    /// and mul_mod, security checks have verified that instance has n=1. Thus the padded segment
    /// satisfies all the AIR constraints.
    // TODO (ohadn): relocate this function if a more appropriate place is found.
    pub fn fill_builtin_segment(&mut self, memory: &mut MemoryBuilder, builtin_name: BuiltinName) {
        let &Some(MemorySegmentAddresses {
            begin_addr,
            stop_ptr,
        }) = self.get_segment(builtin_name)
        else {
            return;
        };
        let initial_length = stop_ptr - begin_addr;
        assert!(initial_length > 0);
        let cells_per_instance = Self::builtin_memory_cells_per_instance(builtin_name);
        assert!(initial_length % cells_per_instance == 0);
        let num_instances = initial_length / cells_per_instance;
        let next_power_of_two = num_instances.next_power_of_two();

        // Verify that the segment we intend to pad is empty.
        memory.verify_empty_segment(
            stop_ptr as u32,
            ((next_power_of_two - num_instances) * cells_per_instance) as u32,
        );

        let mut instance_to_fill_start = stop_ptr as u32;
        let last_instance_start = (stop_ptr - cells_per_instance) as u32;
        for _ in num_instances..next_power_of_two {
            memory.copy_block(
                last_instance_start,
                instance_to_fill_start,
                cells_per_instance as u32,
            );
            instance_to_fill_start += cells_per_instance as u32;
        }
        self.set_segment(
            builtin_name,
            Some(MemorySegmentAddresses {
                begin_addr,
                stop_ptr: begin_addr + cells_per_instance * next_power_of_two,
            }),
        );
    }

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
                res.set_segment(builtin_name, segment);
            };
        }
        res
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
        builtin_segments.set_segment(
            builtin_name,
            Some(MemorySegmentAddresses {
                begin_addr,
                stop_ptr,
            }),
        );
        let memory_write_start = (stop_ptr - cells_per_instance) as u64;
        let mut memory_builder = initialize_memory(memory_write_start, &instance_example);

        builtin_segments.fill_builtin_segment(&mut memory_builder, builtin_name);

        let &MemorySegmentAddresses {
            begin_addr: new_begin_addr,
            stop_ptr: new_stop_ptr,
        } = builtin_segments.get_segment(builtin_name).as_ref().unwrap();
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
