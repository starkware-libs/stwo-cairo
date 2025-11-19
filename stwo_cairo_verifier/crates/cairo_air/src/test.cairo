use core::num::traits::one::One;
#[cfg(not(feature: "qm31_opcode"))]
use stwo_cairo_air::Invertible;
use stwo_cairo_air::components::memory_address_to_id::{
    LOG_MEMORY_ADDRESS_TO_ID_SPLIT, MEMORY_ADDRESS_TO_ID_SPLIT,
};
use stwo_cairo_air::range_checks::RangeChecksInteractionElements;
use stwo_cairo_air::{
    CasmState, MemorySmallValue, PublicData, PublicDataImpl, PublicMemory, PublicSegmentRanges,
    RelationUsesDict, SegmentRange, accumulate_relation_uses,
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

    let dummy_lookup_elements = LookupElementsDummyImpl::dummy();

    let sum = public_data.logup_sum(@dummy_lookup_elements);

    assert_eq!(sum, qm31_const::<971792689, 636659210, 1237675822, 245392094>());
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
