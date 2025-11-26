use stwo_constraint_framework::LookupElementsImpl;
use stwo_verifier_core::channel::Channel;
use super::cairo_air::{
    BlakeGElements, BlakeRoundElements, BlakeRoundSigmaElements, Cube252Elements,
    MemoryAddressToIdElements, MemoryIdToBigElements, OpcodesElements, PartialEcMulElements,
    PedersenPointsTableElements, Poseidon3PartialRoundsChainElements, PoseidonAggregatorElements,
    PoseidonFullRoundChainElements, PoseidonRoundKeysElements, RangeCheck252Width27Elements,
    TripleXor32Elements, VerifyBitwiseXor_12Elements, VerifyBitwiseXor_4Elements,
    VerifyBitwiseXor_7Elements, VerifyBitwiseXor_8Elements, VerifyBitwiseXor_8_BElements,
    VerifyBitwiseXor_9Elements, VerifyInstructionElements,
};
use super::range_checks::{RangeChecksInteractionElements, RangeChecksInteractionElementsImpl};

#[derive(Drop)]
pub struct CairoInteractionElements {
    pub opcodes: OpcodesElements,
    pub verify_instruction: VerifyInstructionElements,
    pub blake_round: BlakeRoundElements,
    pub blake_g: BlakeGElements,
    pub blake_round_sigma: BlakeRoundSigmaElements,
    pub triple_xor_32: TripleXor32Elements,
    pub partial_ec_mul: PartialEcMulElements,
    pub pedersen_points_table: PedersenPointsTableElements,
    pub poseidon_aggregator: PoseidonAggregatorElements,
    pub poseidon_3_partial_rounds_chain: Poseidon3PartialRoundsChainElements,
    pub poseidon_full_round_chain: PoseidonFullRoundChainElements,
    pub cube_252: Cube252Elements,
    pub poseidon_round_keys: PoseidonRoundKeysElements,
    pub range_check_252_width_27: RangeCheck252Width27Elements,
    pub memory_address_to_id: MemoryAddressToIdElements,
    pub memory_id_to_value: MemoryIdToBigElements,
    pub range_checks: RangeChecksInteractionElements,
    pub verify_bitwise_xor_4: VerifyBitwiseXor_4Elements,
    pub verify_bitwise_xor_7: VerifyBitwiseXor_7Elements,
    pub verify_bitwise_xor_8: VerifyBitwiseXor_8Elements,
    pub verify_bitwise_xor_8_b: VerifyBitwiseXor_8_BElements,
    pub verify_bitwise_xor_9: VerifyBitwiseXor_9Elements,
    pub verify_bitwise_xor_12: VerifyBitwiseXor_12Elements,
}

#[generate_trait]
pub impl CairoInteractionElementsImpl of CairoInteractionElementsTrait {
    fn draw(ref channel: Channel) -> CairoInteractionElements {
        CairoInteractionElements {
            opcodes: LookupElementsImpl::draw(ref channel),
            verify_instruction: LookupElementsImpl::draw(ref channel),
            blake_round: LookupElementsImpl::draw(ref channel),
            blake_g: LookupElementsImpl::draw(ref channel),
            blake_round_sigma: LookupElementsImpl::draw(ref channel),
            triple_xor_32: LookupElementsImpl::draw(ref channel),
            poseidon_aggregator: LookupElementsImpl::draw(ref channel),
            poseidon_3_partial_rounds_chain: LookupElementsImpl::draw(ref channel),
            poseidon_full_round_chain: LookupElementsImpl::draw(ref channel),
            cube_252: LookupElementsImpl::draw(ref channel),
            poseidon_round_keys: LookupElementsImpl::draw(ref channel),
            range_check_252_width_27: LookupElementsImpl::draw(ref channel),
            partial_ec_mul: LookupElementsImpl::draw(ref channel),
            pedersen_points_table: LookupElementsImpl::draw(ref channel),
            memory_address_to_id: LookupElementsImpl::draw(ref channel),
            memory_id_to_value: LookupElementsImpl::draw(ref channel),
            range_checks: RangeChecksInteractionElementsImpl::draw(ref channel),
            verify_bitwise_xor_4: LookupElementsImpl::draw(ref channel),
            verify_bitwise_xor_7: LookupElementsImpl::draw(ref channel),
            verify_bitwise_xor_8: LookupElementsImpl::draw(ref channel),
            verify_bitwise_xor_8_b: LookupElementsImpl::draw(ref channel),
            verify_bitwise_xor_9: LookupElementsImpl::draw(ref channel),
            verify_bitwise_xor_12: LookupElementsImpl::draw(ref channel),
        }
    }
}
