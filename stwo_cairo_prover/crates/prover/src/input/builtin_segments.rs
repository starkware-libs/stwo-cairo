use std::collections::HashMap;

use cairo_vm::air_public_input::MemorySegmentAddresses;
use cairo_vm::types::builtin_name::BuiltinName;

/// This struct holds the builtins used in a Cairo program.
#[derive(Debug, Default)]
pub struct BuiltinSegments {
    pub range_check_bits_128: Option<MemorySegmentAddresses>,
    pub pedersen: Option<MemorySegmentAddresses>,
    pub ecdsa: Option<MemorySegmentAddresses>,
    pub keccak: Option<MemorySegmentAddresses>,
    pub bitwise: Option<MemorySegmentAddresses>,
    pub ec_op: Option<MemorySegmentAddresses>,
    pub poseidon: Option<MemorySegmentAddresses>,
    pub range_check_bits_96: Option<MemorySegmentAddresses>,
    pub add_mod: Option<MemorySegmentAddresses>,
    pub mul_mod: Option<MemorySegmentAddresses>,
}

impl BuiltinSegments {
    /// Creates a new `BuiltinSegments` struct from a map of memory segment names to addresses.
    pub fn from_memory_segments(memory_segments: &HashMap<&str, MemorySegmentAddresses>) -> Self {
        let mut res = BuiltinSegments::default();
        for (name, value) in memory_segments.iter() {
            if let Some(builtin) = BuiltinName::from_str(name) {
                let segments = if value.begin_addr == value.stop_ptr {
                    None
                } else {
                    Some((value.begin_addr, value.stop_ptr).into())
                };
                match builtin {
                    BuiltinName::range_check => res.range_check_bits_128 = segments,
                    BuiltinName::pedersen => res.pedersen = segments,
                    BuiltinName::ecdsa => res.ecdsa = segments,
                    BuiltinName::keccak => res.keccak = segments,
                    BuiltinName::bitwise => res.bitwise = segments,
                    BuiltinName::ec_op => res.ec_op = segments,
                    BuiltinName::poseidon => res.poseidon = segments,
                    BuiltinName::range_check96 => res.range_check_bits_96 = segments,
                    BuiltinName::add_mod => res.add_mod = segments,
                    BuiltinName::mul_mod => res.mul_mod = segments,
                    // Not builtins.
                    BuiltinName::output | BuiltinName::segment_arena => {}
                }
            };
        }
        res
    }
}

#[cfg(test)]
mod test_builtin_segments {
    use std::path::PathBuf;

    use cairo_vm::air_public_input::PublicInput;

    use crate::input::BuiltinSegments;

    // TODO(Stav): move read json to be a unit function
    #[test]
    fn test_builtin_segments() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("src/input/test_builtins_segments/blake2s_felts_public_input.json");
        let pub_data_string = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Unable to read file: {}", path.display()));
        let pub_data: PublicInput<'_> =
            sonic_rs::from_str(&pub_data_string).expect("Unable to parse JSON");
        let builtins_segments = BuiltinSegments::from_memory_segments(&pub_data.memory_segments);
        assert_eq!(
            builtins_segments.range_check_bits_128,
            Some((7069, 7187).into())
        );
        assert_eq!(builtins_segments.pedersen, None);
        assert_eq!(builtins_segments.ecdsa, None);
        assert_eq!(builtins_segments.keccak, None);
        assert_eq!(builtins_segments.bitwise, Some((23581, 23901).into()));
        assert_eq!(builtins_segments.ec_op, None);
        assert_eq!(builtins_segments.poseidon, None);
        assert_eq!(builtins_segments.range_check_bits_96, None);
        assert_eq!(builtins_segments.add_mod, None);
        assert_eq!(builtins_segments.mul_mod, None);
    }
}
