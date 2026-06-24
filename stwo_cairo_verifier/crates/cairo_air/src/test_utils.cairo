use stwo_constraint_framework::{CommonLookupElements, LookupElementsTrait};
use stwo_verifier_core::fields::qm31::qm31_const;
use super::{MemorySmallValue, PublicDataImpl, PublicMemory, PublicSegmentRanges, SegmentRange};

#[generate_trait]
pub impl LookupElementsDummyImpl of LookupElementsDummyTrait {
    fn dummy() -> CommonLookupElements {
        LookupElementsTrait::from_z_alpha(
            z: qm31_const::<1, 2, 3, 4>(), alpha: qm31_const::<4, 3, 2, 1>(),
        )
    }
}

/// Generates a public memory where only the output segment is populated with
/// `output_len` elements, each being a tuple of (id, value).
pub fn mock_public_memory_with_outputs(output_len: u32) -> PublicMemory {
    let mut output = array![];
    for i in 0..output_len {
        output.append((i, [i; 8]));
    }

    let empty_segment = SegmentRange {
        start_ptr: MemorySmallValue { id: 0, value: 0 },
        stop_ptr: MemorySmallValue { id: 0, value: 0 },
    };

    PublicMemory {
        program: [].span(),
        public_segments: PublicSegmentRanges {
            output: empty_segment,
            pedersen: empty_segment,
            range_check_128: empty_segment,
            ecdsa: empty_segment,
            bitwise: empty_segment,
            ec_op: empty_segment,
            keccak: empty_segment,
            poseidon: empty_segment,
            range_check_96: empty_segment,
            add_mod: empty_segment,
            mul_mod: empty_segment,
        },
        output: output.span(),
        safe_call_ids: [1, 2],
    }
}
