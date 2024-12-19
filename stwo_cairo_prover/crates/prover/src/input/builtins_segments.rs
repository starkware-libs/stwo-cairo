use std::collections::HashMap;

use cairo_vm::air_public_input::MemorySegmentAddresses;
use cairo_vm::types::builtin_name::BuiltinName;

// TODO(Stav): Understand if the adapter should pass builtins that won't be supported by stwo.
/// This struct holds the builtins used in a Cairo program.
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

impl From<&HashMap<&str, MemorySegmentAddresses>> for BuiltinsSegments {
    fn from(memory_segments: &HashMap<&str, MemorySegmentAddresses>) -> Self {
        let mut res = BuiltinsSegments::default();
        for (name, value) in memory_segments.iter() {
            let value = Some((value.begin_addr, value.stop_ptr).into());
            if let Some(builtin) = BuiltinName::from_str(name) {
                match builtin {
                    BuiltinName::range_check => res.range_check_bits_128_builtin = value,
                    BuiltinName::range_check96 => res.range_check_bits_96_builtin = value,
                    BuiltinName::bitwise => res.bitwise_builtin = value,
                    BuiltinName::add_mod => res.add_mod_builtin = value,
                    BuiltinName::ec_op => res.ec_op_builtin = value,
                    BuiltinName::ecdsa => res.ecdsa_builtin = value,
                    BuiltinName::keccak => res.keccak_builtin = value,
                    BuiltinName::mul_mod => res.mul_mod_builtin = value,
                    BuiltinName::pedersen => res.pedersen_builtin = value,
                    BuiltinName::poseidon => res.poseidon_builtin = value,
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
        let builtins_segments = BuiltinsSegments::from(&pub_data.memory_segments);
        assert_eq!(
            builtins_segments.range_check_bits_128_builtin,
            Some((289, 289).into())
        );
        assert_eq!(builtins_segments.range_check_bits_96_builtin, None);
        assert_eq!(builtins_segments.bitwise_builtin, None);
        assert_eq!(builtins_segments.add_mod_builtin, None);
        assert_eq!(builtins_segments.ec_op_builtin, None);
        assert_eq!(builtins_segments.ecdsa_builtin, Some((353, 353).into()));
        assert_eq!(builtins_segments.keccak_builtin, None);
        assert_eq!(builtins_segments.mul_mod_builtin, None);
        assert_eq!(builtins_segments.pedersen_builtin, Some((97, 97).into()));
        assert_eq!(builtins_segments.poseidon_builtin, None);
    }
}
