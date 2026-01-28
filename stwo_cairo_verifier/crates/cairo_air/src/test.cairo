#[cfg(not(feature: "qm31_opcode"))]
use stwo_cairo_air::Invertible;
use stwo_cairo_air::components::memory_address_to_id::{
    LOG_MEMORY_ADDRESS_TO_ID_SPLIT, MEMORY_ADDRESS_TO_ID_SPLIT,
};
use stwo_cairo_air::{
    CasmState, MemorySmallValue, PublicData, PublicDataImpl, PublicMemory, PublicSegmentRanges,
    RelationUsesDict, SegmentRange, accumulate_relation_uses,
};
use stwo_verifier_core::fields::m31::M31Trait;
use stwo_verifier_core::fields::qm31::qm31_const;
use stwo_verifier_core::utils::ArrayImpl;
use crate::pow2;
use super::test_utils::LookupElementsDummyImpl;
use crate::PubMemoryValue;

#[test]
fn test_public_data_logup_sum() {
       let program: Array<PubMemoryValue> = array![
            (0, [2147450879, 67600385, 0, 0, 0, 0, 0, 0]),
            (1, [11, 0, 0, 0, 0, 0, 0, 0]),
            (2, [2147581952, 285507585, 0, 0, 0, 0, 0, 0]),
            (3, [4, 0, 0, 0, 0, 0, 0, 0]),
            (4, [2147450879, 17268737, 0, 0, 0, 0, 0, 0]),
            (5, [0, 0, 0, 0, 0, 0, 0, 0]),
            (6, [2147450880, 1208647667, 0, 0, 0, 0, 0, 0]),
            (7, [2147450880, 1208647668, 0, 0, 0, 0, 0, 0]),
            (8, [2147450880, 1208647669, 0, 0, 0, 0, 0, 0]),
            (9, [2147450880, 1208647670, 0, 0, 0, 0, 0, 0]),
            (10, [2147450880, 1208647671, 0, 0, 0, 0, 0, 0]),
            (11, [2147450880, 1208647672, 0, 0, 0, 0, 0, 0]),
            (12, [2147450880, 1208647673, 0, 0, 0, 0, 0, 0]),
            (13, [2147450880, 1208647674, 0, 0, 0, 0, 0, 0]),
            (14, [2147450880, 1208647675, 0, 0, 0, 0, 0, 0]),
            (15, [2147450880, 1208647676, 0, 0, 0, 0, 0, 0]),
            (16, [2147450880, 1208647677, 0, 0, 0, 0, 0, 0]),
            (17, [2147450878, 546013183, 0, 0, 0, 0, 0, 0]),
        ];


    let public_data = PublicData {
        public_memory: PublicMemory {
            program: program.span(),
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

    assert_eq!(sum, qm31_const::<908842852, 42171643, 313383432, 1019452808>());
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
