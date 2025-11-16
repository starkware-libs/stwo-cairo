use serde::{Deserialize, Serialize};
use stwo::core::proof::StarkProof;
use stwo::core::vcs::MerkleHasher;
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};

use crate::cairo_claim::CairoClaim;
use crate::cairo_interaction_claim::CairoInteractionClaim;

#[derive(Serialize, Deserialize)]
pub struct CairoProof<H: MerkleHasher> {
    pub claim: CairoClaim,
    pub interaction_pow: u64,
    pub interaction_claim: CairoInteractionClaim,
    pub stark_proof: StarkProof<H>,
    /// Optional salt used in the channel initialization.
    pub channel_salt: Option<u64>,
}

impl<H: MerkleHasher> CairoSerialize for CairoProof<H>
where
    H::Hash: CairoSerialize,
{
    fn serialize(&self, output: &mut Vec<starknet_ff::FieldElement>) {
        let Self {
            claim,
            interaction_pow,
            interaction_claim,
            stark_proof,
            channel_salt,
        } = self;
        CairoSerialize::serialize(claim, output);
        CairoSerialize::serialize(interaction_pow, output);
        CairoSerialize::serialize(interaction_claim, output);
        CairoSerialize::serialize(stark_proof, output);
        CairoSerialize::serialize(channel_salt, output);
    }
}

impl<H: MerkleHasher> CairoDeserialize for CairoProof<H>
where
    H::Hash: CairoDeserialize,
{
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a starknet_ff::FieldElement>) -> Self {
        let claim = CairoDeserialize::deserialize(data);
        let interaction_pow = CairoDeserialize::deserialize(data);
        let interaction_claim = CairoDeserialize::deserialize(data);
        let stark_proof = CairoDeserialize::deserialize(data);
        let channel_salt = CairoDeserialize::deserialize(data);
        Self {
            claim,
            interaction_pow,
            interaction_claim,
            stark_proof,
            channel_salt,
        }
    }
}
