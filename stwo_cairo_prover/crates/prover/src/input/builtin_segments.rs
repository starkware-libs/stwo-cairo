use cairo_vm::air_public_input::MemorySegmentAddresses;
use cairo_vm::stdlib::collections::HashMap;
use cairo_vm::types::builtin_name::BuiltinName;
use serde::{Deserialize, Serialize};

use super::memory::MemoryBuilder;

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
    /// Adds a segment to the builtin segments.
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

    /// Pads a builtin segment with additional valid instances.
    /// The segment is padded to the next power of 2 number of instances.
    pub fn fill_builtin_segment(
        &mut self,
        mut memory: MemoryBuilder,
        builtin_name: BuiltinName,
    ) -> MemoryBuilder {
        let &Some(MemorySegmentAddresses {
            begin_addr,
            stop_ptr,
        }) = self.get_segment(builtin_name)
        else {
            return memory;
        };
        let initial_length = stop_ptr - begin_addr;
        let cells_per_instance = Self::builtin_memory_cells_per_instance(builtin_name);
        assert!(initial_length % cells_per_instance == 0);
        let num_instances = initial_length / cells_per_instance;
        let next_power_of_two = num_instances.next_power_of_two();
        let mut address_to_fill = (begin_addr + num_instances * cells_per_instance) as u64;
        for _ in num_instances..next_power_of_two {
            for j in 0..cells_per_instance {
                memory.copy_value((begin_addr + j) as u32, address_to_fill as u32);
                address_to_fill += 1;
            }
        }
        self.set_segment(
            builtin_name,
            Some(MemorySegmentAddresses {
                begin_addr,
                stop_ptr: begin_addr + cells_per_instance * next_power_of_two,
            }),
        );
        memory
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

    use cairo_vm::air_public_input::PublicInput;

    use crate::input::BuiltinSegments;

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
}
