use core::num::traits::one::One;
use stwo_constraint_framework::LookupElements;
use stwo_verifier_core::fields::qm31::qm31_const;
use stwo_verifier_core::utils::ArrayImpl;
use crate::{
    CairoInteractionElements, PublicData, PublicDataImpl, RangeChecksInteractionElements,
    RelationUsesDict, accumulate_relation_uses, construct_f252, deconstruct_f252,
    hash_memory_section,
};
#[test]
    #[cairofmt::skip]
    fn test_public_data_logup_sum() {
        let mut public_data_felts = array![
            0, 228, 2520, 228, 2520, 0, 228, 2520, 228, 2520, 0, 228, 
            2520, 228, 2520, 0, 5, 0, 5, 0, 0, 228, 2520, 228, 
            2520, 0, 5, 0, 5, 0, 0, 5, 0, 5, 0, 0, 228, 2520, 228, 2520, 
            0, 228, 2520, 228, 2520, 0, 228, 2520, 228, 2520,
            0, 228, 2520, 228, 2520, 0, 2, 227, 1336, 0, 0, 0, 0, 0, 
            0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1336, 1336, 5, 2520, 1336]
        .span();
        let public_data: PublicData = Serde::deserialize(ref public_data_felts).unwrap();
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

#[cfg(not(feature: "poseidon252_verifier"))]
#[test]
fn test_hash_memory_section() {
    let section = array![
        (0, [1, 2, 3, 4, 5, 6, 7, 8]), (0, [2, 3, 4, 5, 6, 7, 8, 9]),
        (0, [3, 4, 5, 6, 7, 8, 9, 10]),
    ];

    assert_eq!(
        hash_memory_section(@section).unbox(),
        [
            3098114871, 843612567, 2372208999, 1823639248, 1136624132, 2551058277, 1389013608,
            1207876589,
        ],
    );
}

#[cfg(feature: "poseidon252_verifier")]
#[test]
fn test_hash_memory_section() {
    let section = array![
        (0, [1, 2, 3, 4, 5, 6, 7, 8]), (0, [2, 3, 4, 5, 6, 7, 8, 9]),
        (0, [3, 4, 5, 6, 7, 8, 9, 10]),
    ];

    assert_eq!(
        hash_memory_section(@section).unbox(),
        [
            2433336977, 2153250057, 881002283, 2835163344, 2300811583, 376217666, 1436681392,
            91789842,
        ],
    );
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

#[test]
fn test_construct_felt() {
    assert_eq!(
        construct_f252(BoxTrait::new([1_u32, 2, 3, 4, 5, 6, 7, 8])),
        0x800000007000000060000000500000004000000030000000200000001,
    );
}

#[test]
fn test_deconstruct_felt() {
    assert_eq!(
        deconstruct_f252(0x800000007000000060000000500000004000000030000000200000001).unbox(),
        [1_u32, 2, 3, 4, 5, 6, 7, 8],
    );
}
