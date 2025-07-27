use core::num::traits::one::One;
use stwo_verifier_core::utils::ArrayImpl;
use super::{
    CairoInteractionElements, CasmState, LookupElements, MemorySmallValue, PublicData,
    PublicDataImpl, PublicMemory, PublicSegmentRanges, RangeChecksInteractionElements,
    RelationUsesDict, SegmentRange, accumulate_relation_uses, hash_memory_section, qm31_const,
};

#[generate_trait]
impl LookupElementsDummyImpl<const N: usize> of LookupElementsDummyTrait<N> {
    fn dummy() -> LookupElements<N> {
        LookupElements::<
            N,
        > {
            z: qm31_const::<1, 2, 3, 4>(),
            alpha: One::one(),
            alpha_powers: ArrayImpl::new_repeated(N, One::one()),
        }
    }
}

/// Generates a public memory where only the output segment is populated with
/// `output_len` elements, each being a tuple of (id, value).
pub fn mock_public_memory_with_outputs(output_len: u32) -> PublicMemory {
    let mut output = array![];
    for i in 0..output_len {
        output.append((i, [i; 8]));
    }
    PublicMemory {
        program: array![],
        public_segments: PublicSegmentRanges {
            output: SegmentRange {
                start_ptr: MemorySmallValue { id: 0, value: 0 },
                stop_ptr: MemorySmallValue { id: 0, value: output_len * 8 },
            },
            pedersen: None,
            range_check_128: None,
            ecdsa: None,
            bitwise: None,
            ec_op: None,
            keccak: None,
            poseidon: None,
            range_check_96: None,
            add_mod: None,
            mul_mod: None,
        },
        output,
        safe_call: array![(1, [1; 8]), (2, [2; 8])],
    }
}

pub fn dummy_interaction_lookup_elements() -> CairoInteractionElements {
    CairoInteractionElements {
        opcodes: LookupElementsDummyImpl::dummy(),
        verify_instruction: LookupElementsDummyImpl::dummy(),
        blake_round: LookupElementsDummyImpl::dummy(),
        blake_g: LookupElementsDummyImpl::dummy(),
        blake_round_sigma: LookupElementsDummyImpl::dummy(),
        triple_xor_32: LookupElementsDummyImpl::dummy(),
        poseidon_3_partial_rounds_chain: LookupElementsDummyImpl::dummy(),
        poseidon_full_round_chain: LookupElementsDummyImpl::dummy(),
        cube_252: LookupElementsDummyImpl::dummy(),
        poseidon_round_keys: LookupElementsDummyImpl::dummy(),
        range_check_felt_252_width_27: LookupElementsDummyImpl::dummy(),
        partial_ec_mul: LookupElementsDummyImpl::dummy(),
        pedersen_points_table: LookupElementsDummyImpl::dummy(),
        memory_address_to_id: LookupElementsDummyImpl::dummy(),
        memory_id_to_value: LookupElementsDummyImpl::dummy(),
        range_checks: RangeChecksInteractionElements {
            rc_6: LookupElementsDummyImpl::dummy(),
            rc_8: LookupElementsDummyImpl::dummy(),
            rc_11: LookupElementsDummyImpl::dummy(),
            rc_12: LookupElementsDummyImpl::dummy(),
            rc_18: LookupElementsDummyImpl::dummy(),
            rc_18_b: LookupElementsDummyImpl::dummy(),
            rc_19: LookupElementsDummyImpl::dummy(),
            rc_19_b: LookupElementsDummyImpl::dummy(),
            rc_19_c: LookupElementsDummyImpl::dummy(),
            rc_19_d: LookupElementsDummyImpl::dummy(),
            rc_19_e: LookupElementsDummyImpl::dummy(),
            rc_19_f: LookupElementsDummyImpl::dummy(),
            rc_19_g: LookupElementsDummyImpl::dummy(),
            rc_19_h: LookupElementsDummyImpl::dummy(),
            rc_4_3: LookupElementsDummyImpl::dummy(),
            rc_4_4: LookupElementsDummyImpl::dummy(),
            rc_5_4: LookupElementsDummyImpl::dummy(),
            rc_9_9: LookupElementsDummyImpl::dummy(),
            rc_9_9_b: LookupElementsDummyImpl::dummy(),
            rc_9_9_c: LookupElementsDummyImpl::dummy(),
            rc_9_9_d: LookupElementsDummyImpl::dummy(),
            rc_9_9_e: LookupElementsDummyImpl::dummy(),
            rc_9_9_f: LookupElementsDummyImpl::dummy(),
            rc_9_9_g: LookupElementsDummyImpl::dummy(),
            rc_9_9_h: LookupElementsDummyImpl::dummy(),
            rc_7_2_5: LookupElementsDummyImpl::dummy(),
            rc_3_6_6_3: LookupElementsDummyImpl::dummy(),
            rc_4_4_4_4: LookupElementsDummyImpl::dummy(),
            rc_3_3_3_3_3: LookupElementsDummyImpl::dummy(),
        },
        verify_bitwise_xor_4: LookupElementsDummyImpl::dummy(),
        verify_bitwise_xor_7: LookupElementsDummyImpl::dummy(),
        verify_bitwise_xor_8: LookupElementsDummyImpl::dummy(),
        verify_bitwise_xor_9: LookupElementsDummyImpl::dummy(),
        verify_bitwise_xor_12: LookupElementsDummyImpl::dummy(),
    }
}
