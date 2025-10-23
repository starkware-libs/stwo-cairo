use std::collections::BTreeMap;

use cairo_vm::air_public_input::MemorySegmentAddresses as VMMemorySegmentAddresses;
use cairo_vm::stdlib::collections::HashMap;
use cairo_vm::types::builtin_name::BuiltinName;
use cairo_vm::types::relocatable::MaybeRelocatable;
use serde::{Deserialize, Serialize};
use stwo_cairo_common::builtins::{
    ADD_MOD_MEMORY_CELLS, BITWISE_MEMORY_CELLS, ECDSA_MEMORY_CELLS, EC_OP_MEMORY_CELLS,
    KECCAK_MEMORY_CELLS, MUL_MOD_MEMORY_CELLS, OUTPUT_MEMORY_CELLS, PEDERSEN_MEMORY_CELLS,
    POSEIDON_MEMORY_CELLS, RANGE_CHECK_MEMORY_CELLS,
};
use stwo_cairo_common::prover_types::simd::N_LANES;
use tracing::{info, span, Level};

use super::memory::MemoryBuilder;

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
        insert_builtin(BuiltinName::output, &self.output, OUTPUT_MEMORY_CELLS);
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
        builtin_segments: &BTreeMap<usize, BuiltinName>,
    ) {
        let _span = span!(Level::INFO, "pad_relocatble_builtin_segments").entered();
        for (segment_index, builtin_name) in builtin_segments {
            if *builtin_name == BuiltinName::segment_arena || *builtin_name == BuiltinName::output {
                continue;
            };
            let current_builtin_segment = &mut relocatable_memory[*segment_index];

            let original_segment_len = current_builtin_segment.len();

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
                original_segment_len.is_multiple_of(cells_per_instance),
                "builtin segment: {builtin_name} size is {original_segment_len}, which is not divisble by {cells_per_instance}"
            );

            if !current_builtin_segment.iter().all(|x| x.is_some()) {
                panic!(
                    "Builtins segments '{builtin_name}' at segment index: {segment_index}, contains a hole."
                );
            }

            // Builtin segment size is extended to the next power of two instances.
            let new_segment_size = original_segment_len
                .div_ceil(cells_per_instance)
                .next_power_of_two()
                .max(MIN_SEGMENT_SIZE)
                * cells_per_instance;
            current_builtin_segment.resize(new_segment_size, None);

            // Fill the segment extension with copies of the last instance.
            let last_instance_start = original_segment_len - cells_per_instance;
            for i in original_segment_len..new_segment_size {
                current_builtin_segment[i] =
                    current_builtin_segment[last_instance_start + (i % cells_per_instance)].clone();
            }

            info!(
                "Padded builtin segment '{}' from {} to {} instances.",
                builtin_name,
                original_segment_len / cells_per_instance,
                new_segment_size / cells_per_instance
            );
        }
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
    assert!(initial_length.is_multiple_of(n_cells_per_instance));
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

#[cfg(test)]
mod test_builtin_segments {
    use test_case::test_case;

    use super::*;
    use crate::builtins::BITWISE_MEMORY_CELLS;
    use crate::memory::{u128_to_4_limbs, Memory, MemoryBuilder, MemoryConfig, MemoryValue};

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

        let builtin_segments = BTreeMap::from([(0, BuiltinName::bitwise), (1, BuiltinName::ec_op)]);

        BuiltinSegments::pad_relocatble_builtin_segments(
            &mut relocatable_memory,
            &builtin_segments,
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
        let builtin_segments = BTreeMap::from([(0, BuiltinName::mul_mod)]);

        BuiltinSegments::pad_relocatble_builtin_segments(
            &mut relocatable_memory,
            &builtin_segments,
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
        let builtin_segments = BTreeMap::from([(0, BuiltinName::bitwise)]);

        BuiltinSegments::pad_relocatble_builtin_segments(
            &mut relocatable_memory,
            &builtin_segments,
        );
    }
}
