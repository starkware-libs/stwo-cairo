use std::collections::BTreeMap;

use super::vm_import::json::Segment;

// TODO(STAV): Understand if the adapter should pass builtins that won't be supported by stwo.
/// This struct holds the builtins used in a Cairo program.
/// Most of them are not implemented yet by Stwo.
#[derive(Debug, Default)]
pub struct BuiltinsSegments {
    pub range_check_bits_128_builtin: Segment,
    pub range_check_bits_96_builtin: Segment,
    pub bitwise_builtin: Segment,
    pub add_mod_builtin: Segment,
    pub ec_op_builtin: Segment,
    pub ecdsa_builtin: Segment,
    pub keccak_builtin: Segment,
    pub mul_mod_builtin: Segment,
    pub pedersen_builtin: Segment,
    pub poseidon_builtin: Segment,
}

impl BuiltinsSegments {
    /// Create a new `BuiltinsSegments` struct from a map of memory segments.
    pub fn from_memory_segments(memory_segments: &BTreeMap<String, Segment>) -> Self {
        let mut res = Self::default();
        for (name, value) in memory_segments.iter() {
            match name.as_str() {
                "range_check" => {
                    res.range_check_bits_128_builtin = Segment {
                        begin_addr: value.begin_addr,
                        stop_ptr: value.stop_ptr,
                    }
                }
                "range_check96" => {
                    res.range_check_bits_96_builtin = Segment {
                        begin_addr: value.begin_addr,
                        stop_ptr: value.stop_ptr,
                    }
                }
                "bitwise" => {
                    res.bitwise_builtin = Segment {
                        begin_addr: value.begin_addr,
                        stop_ptr: value.stop_ptr,
                    }
                }
                "add_mod" => {
                    res.add_mod_builtin = Segment {
                        begin_addr: value.begin_addr,
                        stop_ptr: value.stop_ptr,
                    }
                }
                "ec_op" => {
                    res.ec_op_builtin = Segment {
                        begin_addr: value.begin_addr,
                        stop_ptr: value.stop_ptr,
                    }
                }
                "ecdsa" => {
                    res.ecdsa_builtin = Segment {
                        begin_addr: value.begin_addr,
                        stop_ptr: value.stop_ptr,
                    }
                }
                "keccak" => {
                    res.keccak_builtin = Segment {
                        begin_addr: value.begin_addr,
                        stop_ptr: value.stop_ptr,
                    }
                }
                "mul_mod" => {
                    res.mul_mod_builtin = Segment {
                        begin_addr: value.begin_addr,
                        stop_ptr: value.stop_ptr,
                    }
                }
                "pedersen" => {
                    res.pedersen_builtin = Segment {
                        begin_addr: value.begin_addr,
                        stop_ptr: value.stop_ptr,
                    }
                }
                "poseidon" => {
                    res.poseidon_builtin = Segment {
                        begin_addr: value.begin_addr,
                        stop_ptr: value.stop_ptr,
                    }
                }
                // Output, executaion and program segments are not builtins.
                "output" => {}
                "execution" => {}
                "program" => {}
                _ => panic!("Unknown memory segment: {name}"),
            }
        }
        res
    }
}
