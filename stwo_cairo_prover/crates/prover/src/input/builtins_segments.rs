use std::collections::HashMap;

use cairo_vm::air_public_input::MemorySegmentAddresses;

// TODO(Stav): Understand if the adapter should pass builtins that won't be supported by stwo.
/// This struct holds the builtins used in a Cairo program.
/// Most of them are not implemented yet by Stwo.
#[derive(Debug, Default)]
pub struct BuiltinsSegments {
    pub range_check_bits_128_builtin: Option<MemorySegmentAddresses>,
    pub range_check_bits_96_builtin: Option<MemorySegmentAddresses>,
    pub bitwise_builtin: Option<MemorySegmentAddresses>,
    pub add_mod_builtin: Option<MemorySegmentAddresses>,
    pub ec_op_builtin: Option<MemorySegmentAddresses>,
    pub ecdsa_builtin: Option<MemorySegmentAddresses>,
    pub keccak_builtin: Option<MemorySegmentAddresses>,
    pub mul_mod_builtin: Option<MemorySegmentAddresses>,
    pub pedersen_builtin: Option<MemorySegmentAddresses>,
    pub poseidon_builtin: Option<MemorySegmentAddresses>,
}

impl BuiltinsSegments {
    /// Create a new `BuiltinsSegments` struct from a map of memory MemorySegmentAddressess.
    pub fn from_memory_segments(
        memory_segments: &HashMap<&str, cairo_vm::air_public_input::MemorySegmentAddresses>,
    ) -> Self {
        let mut res = BuiltinsSegments::default();
        for (name, value) in memory_segments.iter() {
            let value = (value.begin_addr, value.stop_ptr).into();
            match *name {
                "range_check" => res.range_check_bits_128_builtin = Some(value),
                "range_check96" => res.range_check_bits_96_builtin = Some(value),
                "bitwise" => res.bitwise_builtin = Some(value),
                "add_mod" => res.add_mod_builtin = Some(value),
                "ec_op" => res.ec_op_builtin = Some(value),
                "ecdsa" => res.ecdsa_builtin = Some(value),
                "keccak" => res.keccak_builtin = Some(value),
                "mul_mod" => res.mul_mod_builtin = Some(value),
                "pedersen" => res.pedersen_builtin = Some(value),
                "poseidon" => res.poseidon_builtin = Some(value),
                // Output, execution and program segments are not builtins.
                "output" => {}
                "execution" => {}
                "program" => {}
                _ => panic!("Unknown memory segment: {name}"),
            }
        }
        res
    }
}
