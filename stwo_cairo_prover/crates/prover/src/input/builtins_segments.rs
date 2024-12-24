use std::collections::HashMap;

use cairo_vm::air_public_input::MemorySegmentAddresses;
use cairo_vm::types::builtin_name::BuiltinName;

/// This struct holds the builtins used in a Cairo program.
#[derive(Debug, Default)]
pub struct BuiltinsSegments {
    pub range_check_bits_128: Option<MemorySegmentAddresses>,
    pub range_check_bits_96: Option<MemorySegmentAddresses>,
    pub bitwise: Option<MemorySegmentAddresses>,
    pub add_mod: Option<MemorySegmentAddresses>,
    pub ec_op: Option<MemorySegmentAddresses>,
    pub ecdsa: Option<MemorySegmentAddresses>,
    pub keccak: Option<MemorySegmentAddresses>,
    pub mul_mod: Option<MemorySegmentAddresses>,
    pub pedersen: Option<MemorySegmentAddresses>,
    pub poseidon: Option<MemorySegmentAddresses>,
}

impl BuiltinsSegments {
    /// Create a new `BuiltinsSegments` struct from a map of memory MemorySegmentAddressess.
    pub fn from_memory_segments(memory_segments: &HashMap<&str, MemorySegmentAddresses>) -> Self {
        let mut res = BuiltinsSegments::default();
        for (name, value) in memory_segments.iter() {
            let value = Some((value.begin_addr, value.stop_ptr).into());
            if let Some(builtin) = BuiltinName::from_str(name) {
                match builtin {
                    BuiltinName::range_check => res.range_check_bits_128 = value,
                    BuiltinName::range_check96 => res.range_check_bits_96 = value,
                    BuiltinName::bitwise => res.bitwise = value,
                    BuiltinName::add_mod => res.add_mod = value,
                    BuiltinName::ec_op => res.ec_op = value,
                    BuiltinName::ecdsa => res.ecdsa = value,
                    BuiltinName::keccak => res.keccak = value,
                    BuiltinName::mul_mod => res.mul_mod = value,
                    BuiltinName::pedersen => res.pedersen = value,
                    BuiltinName::poseidon => res.poseidon = value,
                    // Not builtins.
                    BuiltinName::output | BuiltinName::segment_arena => {}
                }
            };
        }
        res
    }
}

#[cfg(test)]
mod test_builtins_segments {
    use std::path::PathBuf;

    use cairo_vm::air_public_input::PublicInput;

    use crate::input::BuiltinsSegments;

    #[test]
    fn test_builtins_segments() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/input/test_builtins_segments/air_pub.json");
        let pub_data_string = std::fs::read_to_string(&d)
            .unwrap_or_else(|_| panic!("Unable to read file: {}", d.display()));
        let pub_data: PublicInput<'_> =
            sonic_rs::from_str(&pub_data_string).expect("Unable to parse JSON");
        let builtins_segments = BuiltinsSegments::from_memory_segments(&pub_data.memory_segments);
        assert_eq!(
            builtins_segments.range_check_bits_128,
            Some((289, 289).into())
        );
        assert_eq!(builtins_segments.range_check_bits_96, None);
        assert_eq!(builtins_segments.bitwise, None);
        assert_eq!(builtins_segments.add_mod, None);
        assert_eq!(builtins_segments.ec_op, None);
        assert_eq!(builtins_segments.ecdsa, Some((353, 353).into()));
        assert_eq!(builtins_segments.keccak, None);
        assert_eq!(builtins_segments.mul_mod, None);
        assert_eq!(builtins_segments.pedersen, Some((97, 97).into()));
        assert_eq!(builtins_segments.poseidon, None);
    }
}
