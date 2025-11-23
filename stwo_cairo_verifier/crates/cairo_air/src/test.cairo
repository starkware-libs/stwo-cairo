use core::num::traits::one::One;
#[cfg(not(feature: "qm31_opcode"))]
use stwo_cairo_air::Invertible;
use stwo_cairo_air::components::memory_address_to_id::{
    LOG_MEMORY_ADDRESS_TO_ID_SPLIT, MEMORY_ADDRESS_TO_ID_SPLIT,
};
use stwo_cairo_air::range_checks::RangeChecksInteractionElements;
use stwo_cairo_air::{
    CairoInteractionElements, CasmState, MemorySmallValue, PublicData, PublicDataImpl, PublicMemory,
    PublicSegmentRanges, RelationUsesDict, SegmentRange, accumulate_relation_uses,
};
use stwo_constraint_framework::LookupElements;
use stwo_verifier_core::fields::m31::M31Trait;
use stwo_verifier_core::fields::qm31::qm31_const;
use stwo_verifier_core::utils::ArrayImpl;
use crate::pow2;

#[test]
fn test_public_data_logup_sum() {
    let public_data = PublicData {
        public_memory: PublicMemory {
            program: [].span(),
            public_segments: PublicSegmentRanges {
                output: SegmentRange {
                    start_ptr: MemorySmallValue { id: 228, value: 2520 },
                    stop_ptr: MemorySmallValue { id: 228, value: 2520 },
                },
                pedersen: SegmentRange {
                    start_ptr: MemorySmallValue { id: 228, value: 2520 },
                    stop_ptr: MemorySmallValue { id: 228, value: 2520 },
                },
                range_check_128: SegmentRange {
                    start_ptr: MemorySmallValue { id: 228, value: 2520 },
                    stop_ptr: MemorySmallValue { id: 228, value: 2520 },
                },
                ecdsa: SegmentRange {
                    start_ptr: MemorySmallValue { id: 5, value: 0 },
                    stop_ptr: MemorySmallValue { id: 5, value: 0 },
                },
                bitwise: SegmentRange {
                    start_ptr: MemorySmallValue { id: 228, value: 2520 },
                    stop_ptr: MemorySmallValue { id: 228, value: 2520 },
                },
                ec_op: SegmentRange {
                    start_ptr: MemorySmallValue { id: 5, value: 0 },
                    stop_ptr: MemorySmallValue { id: 5, value: 0 },
                },
                keccak: SegmentRange {
                    start_ptr: MemorySmallValue { id: 5, value: 0 },
                    stop_ptr: MemorySmallValue { id: 5, value: 0 },
                },
                poseidon: SegmentRange {
                    start_ptr: MemorySmallValue { id: 228, value: 2520 },
                    stop_ptr: MemorySmallValue { id: 228, value: 2520 },
                },
                range_check_96: SegmentRange {
                    start_ptr: MemorySmallValue { id: 228, value: 2520 },
                    stop_ptr: MemorySmallValue { id: 228, value: 2520 },
                },
                add_mod: SegmentRange {
                    start_ptr: MemorySmallValue { id: 228, value: 2520 },
                    stop_ptr: MemorySmallValue { id: 228, value: 2520 },
                },
                mul_mod: SegmentRange {
                    start_ptr: MemorySmallValue { id: 228, value: 2520 },
                    stop_ptr: MemorySmallValue { id: 228, value: 2520 },
                },
            },
            output: [].span(),
            safe_call_ids: [227, 5],
        },
        initial_state: CasmState {
            pc: M31Trait::new(1), ap: M31Trait::new(1336), fp: M31Trait::new(1336),
        },
        final_state: CasmState {
            pc: M31Trait::new(5), ap: M31Trait::new(2520), fp: M31Trait::new(1336),
        },
    };

    let dummy_lookup_elements = dummy_interaction_lookup_elements();

    let sum = public_data.logup_sum(@dummy_lookup_elements);

    assert_eq!(sum, qm31_const::<971792689, 636659210, 1237675822, 245392094>());
}

fn dummy_interaction_lookup_elements() -> CairoInteractionElements {
    CairoInteractionElements {
        opcodes: LookupElementsDummyImpl::dummy(),
        verify_instruction: LookupElementsDummyImpl::dummy(),
        blake_round: LookupElementsDummyImpl::dummy(),
        blake_g: LookupElementsDummyImpl::dummy(),
        blake_round_sigma: LookupElementsDummyImpl::dummy(),
        triple_xor_32: LookupElementsDummyImpl::dummy(),
        poseidon_aggregator: LookupElementsDummyImpl::dummy(),
        poseidon_3_partial_rounds_chain: LookupElementsDummyImpl::dummy(),
        poseidon_full_round_chain: LookupElementsDummyImpl::dummy(),
        cube_252: LookupElementsDummyImpl::dummy(),
        poseidon_round_keys: LookupElementsDummyImpl::dummy(),
        range_check_252_width_27: LookupElementsDummyImpl::dummy(),
        pedersen_aggregator: LookupElementsDummyImpl::dummy(),
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
            rc_20: LookupElementsDummyImpl::dummy(),
            rc_20_b: LookupElementsDummyImpl::dummy(),
            rc_20_c: LookupElementsDummyImpl::dummy(),
            rc_20_d: LookupElementsDummyImpl::dummy(),
            rc_20_e: LookupElementsDummyImpl::dummy(),
            rc_20_f: LookupElementsDummyImpl::dummy(),
            rc_20_g: LookupElementsDummyImpl::dummy(),
            rc_20_h: LookupElementsDummyImpl::dummy(),
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
        verify_bitwise_xor_8_b: LookupElementsDummyImpl::dummy(),
        verify_bitwise_xor_9: LookupElementsDummyImpl::dummy(),
        verify_bitwise_xor_12: LookupElementsDummyImpl::dummy(),
    }
}

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

#[test]
fn test_accumulate_relation_uses() {
    let mut relation_uses: RelationUsesDict = Default::default();
    relation_uses.insert('relation_1', 4);
    relation_uses.insert('relation_2', 10);
    let log_size = 2;
    let relation_uses_per_row = [('relation_1', 2), ('relation_2', 4)];

    accumulate_relation_uses(ref relation_uses, relation_uses_per_row.span(), log_size);

    assert_eq!(relation_uses.get('relation_1'), 12);
    assert_eq!(relation_uses.get('relation_2'), 26);
}

#[cfg(not(feature: "qm31_opcode"))]
#[test]
fn test_sum_inverses_qm31() {
    let a = qm31_const::<1, 2, 3, 4>();
    let b = qm31_const::<5, 6, 7, 8>();
    let c = qm31_const::<9, 10, 11, 12>();
    let d = qm31_const::<13, 14, 15, 16>();
    let expected = a.inverse() + b.inverse() + c.inverse() + d.inverse();
    let array = array![a, b, c, d];

    let sum = crate::utils::sum_inverses_qm31(@array);

    assert_eq!(sum, expected);
}
#[test]
fn test_split_memory_address_to_id_constants_compatibility() {
    assert!(pow2(LOG_MEMORY_ADDRESS_TO_ID_SPLIT) == MEMORY_ADDRESS_TO_ID_SPLIT);
}
