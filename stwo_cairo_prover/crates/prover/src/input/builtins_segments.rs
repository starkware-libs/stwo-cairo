use std::collections::BTreeMap;

use cairo_vm::air_public_input::MemorySegmentAddresses;

// TODO(STAV): Understand if the adapter should pass builtins that won't be supported by stwo.
/// This struct holds the builtins used in a Cairo program.
/// Most of them are not implemented yet by Stwo.
#[derive(Debug)]
pub struct BuiltinsSegments {
    pub range_check_bits_128_builtin: MemorySegmentAddresses,
    pub range_check_bits_96_builtin: MemorySegmentAddresses,
    pub bitwise_builtin: MemorySegmentAddresses,
    pub add_mod_builtin: MemorySegmentAddresses,
    pub ec_op_builtin: MemorySegmentAddresses,
    pub ecdsa_builtin: MemorySegmentAddresses,
    pub keccak_builtin: MemorySegmentAddresses,
    pub mul_mod_builtin: MemorySegmentAddresses,
    pub pedersen_builtin: MemorySegmentAddresses,
    pub poseidon_builtin: MemorySegmentAddresses,
}

impl Default for BuiltinsSegments {
    fn default() -> Self {
        Self {
            range_check_bits_128_builtin: MemorySegmentAddresses {
                begin_addr: 0,
                stop_ptr: 0,
            },
            range_check_bits_96_builtin: MemorySegmentAddresses {
                begin_addr: 0,
                stop_ptr: 0,
            },
            bitwise_builtin: MemorySegmentAddresses {
                begin_addr: 0,
                stop_ptr: 0,
            },
            add_mod_builtin: MemorySegmentAddresses {
                begin_addr: 0,
                stop_ptr: 0,
            },
            ec_op_builtin: MemorySegmentAddresses {
                begin_addr: 0,
                stop_ptr: 0,
            },
            ecdsa_builtin: MemorySegmentAddresses {
                begin_addr: 0,
                stop_ptr: 0,
            },
            keccak_builtin: MemorySegmentAddresses {
                begin_addr: 0,
                stop_ptr: 0,
            },
            mul_mod_builtin: MemorySegmentAddresses {
                begin_addr: 0,
                stop_ptr: 0,
            },
            pedersen_builtin: MemorySegmentAddresses {
                begin_addr: 0,
                stop_ptr: 0,
            },
            poseidon_builtin: MemorySegmentAddresses {
                begin_addr: 0,
                stop_ptr: 0,
            },
        }
    }
}
impl BuiltinsSegments {
    /// Create a new `BuiltinsSegments` struct from a map of memory MemorySegmentAddressess.
    pub fn from_memory_segments(
        memory_segments: &BTreeMap<String, MemorySegmentAddresses>,
    ) -> Self {
        let mut res = Self::default();
        for (name, value) in memory_segments.iter() {
            match name.as_str() {
                "range_check" => {
                    res.range_check_bits_128_builtin = MemorySegmentAddresses {
                        begin_addr: value.begin_addr,
                        stop_ptr: value.stop_ptr,
                    }
                }
                "range_check96" => {
                    res.range_check_bits_96_builtin = MemorySegmentAddresses {
                        begin_addr: value.begin_addr,
                        stop_ptr: value.stop_ptr,
                    }
                }
                "bitwise" => {
                    res.bitwise_builtin = MemorySegmentAddresses {
                        begin_addr: value.begin_addr,
                        stop_ptr: value.stop_ptr,
                    }
                }
                "add_mod" => {
                    res.add_mod_builtin = MemorySegmentAddresses {
                        begin_addr: value.begin_addr,
                        stop_ptr: value.stop_ptr,
                    }
                }
                "ec_op" => {
                    res.ec_op_builtin = MemorySegmentAddresses {
                        begin_addr: value.begin_addr,
                        stop_ptr: value.stop_ptr,
                    }
                }
                "ecdsa" => {
                    res.ecdsa_builtin = MemorySegmentAddresses {
                        begin_addr: value.begin_addr,
                        stop_ptr: value.stop_ptr,
                    }
                }
                "keccak" => {
                    res.keccak_builtin = MemorySegmentAddresses {
                        begin_addr: value.begin_addr,
                        stop_ptr: value.stop_ptr,
                    }
                }
                "mul_mod" => {
                    res.mul_mod_builtin = MemorySegmentAddresses {
                        begin_addr: value.begin_addr,
                        stop_ptr: value.stop_ptr,
                    }
                }
                "pedersen" => {
                    res.pedersen_builtin = MemorySegmentAddresses {
                        begin_addr: value.begin_addr,
                        stop_ptr: value.stop_ptr,
                    }
                }
                "poseidon" => {
                    res.poseidon_builtin = MemorySegmentAddresses {
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
