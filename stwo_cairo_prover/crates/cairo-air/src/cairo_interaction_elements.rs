use stwo::core::channel::Channel;

use crate::range_checks_air::RangeChecksInteractionElements;
use crate::relations;

pub struct CairoInteractionElements {
    pub opcodes: relations::Opcodes,
    pub verify_instruction: relations::VerifyInstruction,
    pub blake_round: relations::BlakeRound,
    pub blake_g: relations::BlakeG,
    pub blake_sigma: relations::BlakeRoundSigma,
    pub triple_xor_32: relations::TripleXor32,
    pub partial_ec_mul: relations::PartialEcMul,
    pub poseidon_aggregator: relations::PoseidonAggregator,
    pub pedersen_points_table: relations::PedersenPointsTable,
    pub poseidon_3_partial_rounds_chain: relations::Poseidon3PartialRoundsChain,
    pub poseidon_full_round_chain: relations::PoseidonFullRoundChain,
    pub cube_252: relations::Cube252,
    pub poseidon_round_keys: relations::PoseidonRoundKeys,
    pub range_check_252_width_27: relations::RangeCheck252Width27,
    pub memory_address_to_id: relations::MemoryAddressToId,
    pub memory_id_to_value: relations::MemoryIdToBig,
    pub range_checks: RangeChecksInteractionElements,
    pub verify_bitwise_xor_4: relations::VerifyBitwiseXor_4,
    pub verify_bitwise_xor_7: relations::VerifyBitwiseXor_7,
    pub verify_bitwise_xor_8: relations::VerifyBitwiseXor_8,
    pub verify_bitwise_xor_8_b: relations::VerifyBitwiseXor_8_B,
    pub verify_bitwise_xor_9: relations::VerifyBitwiseXor_9,
    pub verify_bitwise_xor_12: relations::VerifyBitwiseXor_12,
}

impl CairoInteractionElements {
    pub fn draw(channel: &mut impl Channel) -> CairoInteractionElements {
        CairoInteractionElements {
            opcodes: relations::Opcodes::draw(channel),
            verify_instruction: relations::VerifyInstruction::draw(channel),
            blake_round: relations::BlakeRound::draw(channel),
            blake_g: relations::BlakeG::draw(channel),
            blake_sigma: relations::BlakeRoundSigma::draw(channel),
            triple_xor_32: relations::TripleXor32::draw(channel),
            poseidon_aggregator: relations::PoseidonAggregator::draw(channel),
            poseidon_3_partial_rounds_chain: relations::Poseidon3PartialRoundsChain::draw(channel),
            poseidon_full_round_chain: relations::PoseidonFullRoundChain::draw(channel),
            cube_252: relations::Cube252::draw(channel),
            poseidon_round_keys: relations::PoseidonRoundKeys::draw(channel),
            range_check_252_width_27: relations::RangeCheck252Width27::draw(channel),
            partial_ec_mul: relations::PartialEcMul::draw(channel),
            pedersen_points_table: relations::PedersenPointsTable::draw(channel),
            memory_address_to_id: relations::MemoryAddressToId::draw(channel),
            memory_id_to_value: relations::MemoryIdToBig::draw(channel),
            range_checks: RangeChecksInteractionElements::draw(channel),
            verify_bitwise_xor_4: relations::VerifyBitwiseXor_4::draw(channel),
            verify_bitwise_xor_7: relations::VerifyBitwiseXor_7::draw(channel),
            verify_bitwise_xor_8: relations::VerifyBitwiseXor_8::draw(channel),
            verify_bitwise_xor_8_b: relations::VerifyBitwiseXor_8_B::draw(channel),
            verify_bitwise_xor_9: relations::VerifyBitwiseXor_9::draw(channel),
            verify_bitwise_xor_12: relations::VerifyBitwiseXor_12::draw(channel),
        }
    }
}
