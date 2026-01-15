use std::ops::Deref;

use stwo::core::fields::m31::BaseField;
use stwo::core::fields::qm31::SecureField;
use stwo::core::fri::FriProof;
use stwo::core::pcs::{PcsConfig, TreeVec};
use stwo::core::vcs_lifted::verifier::MerkleDecommitmentLifted;
use stwo::core::vcs_lifted::MerkleHasherLifted;
use stwo::core::ColumnVec;
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};

use crate::air::{CommitmentSchemeProofSorted, StarkProofSorted};
use crate::{CairoProof, CairoProofSorted};

impl<H: MerkleHasherLifted> CairoSerialize for CairoProof<H>
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

impl<H: MerkleHasherLifted> CairoDeserialize for CairoProof<H>
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

impl<H: MerkleHasherLifted> CairoSerialize for CommitmentSchemeProofSorted<H>
where
    H::Hash: CairoSerialize,
{
    fn serialize(&self, output: &mut Vec<starknet_ff::FieldElement>) {
        let Self {
            config,
            commitments,
            sampled_values,
            decommitments,
            queried_values,
            proof_of_work,
            fri_proof,
        } = self;
        CairoSerialize::serialize(config, output);
        CairoSerialize::serialize(commitments.deref(), output);
        CairoSerialize::serialize(sampled_values.deref(), output);
        CairoSerialize::serialize(decommitments.deref(), output);
        CairoSerialize::serialize(queried_values.deref(), output);
        CairoSerialize::serialize(proof_of_work, output);
        CairoSerialize::serialize(fri_proof, output);
    }
}

impl<H: MerkleHasherLifted> CairoDeserialize for CommitmentSchemeProofSorted<H>
where
    H::Hash: CairoDeserialize,
{
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a starknet_ff::FieldElement>) -> Self {
        let config = <PcsConfig as CairoDeserialize>::deserialize(data);
        let commitments = TreeVec(<Vec<H::Hash> as CairoDeserialize>::deserialize(data));
        let sampled_values =
            TreeVec(<Vec<ColumnVec<Vec<SecureField>>> as CairoDeserialize>::deserialize(data));
        let decommitments =
            TreeVec(<Vec<MerkleDecommitmentLifted<H>> as CairoDeserialize>::deserialize(data));
        let queried_values = TreeVec(<Vec<Vec<BaseField>> as CairoDeserialize>::deserialize(data));
        let proof_of_work: u64 = <u64 as CairoDeserialize>::deserialize(data);
        let fri_proof = <FriProof<H> as CairoDeserialize>::deserialize(data);
        Self {
            config,
            commitments,
            sampled_values,
            decommitments,
            queried_values,
            proof_of_work,
            fri_proof,
        }
    }
}

impl<H: MerkleHasherLifted> CairoSerialize for StarkProofSorted<H>
where
    H::Hash: CairoSerialize,
{
    fn serialize(&self, output: &mut Vec<starknet_ff::FieldElement>) {
        let Self(commitment_scheme_proof) = self;
        commitment_scheme_proof.serialize(output);
    }
}

impl<H: MerkleHasherLifted> CairoDeserialize for StarkProofSorted<H>
where
    H::Hash: CairoDeserialize,
{
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a starknet_ff::FieldElement>) -> Self {
        let commitment_scheme_proof =
            <CommitmentSchemeProofSorted<H> as CairoDeserialize>::deserialize(data);
        StarkProofSorted(commitment_scheme_proof)
    }
}

impl<H: MerkleHasherLifted> CairoSerialize for CairoProofSorted<H>
where
    H::Hash: CairoSerialize,
{
    fn serialize(&self, output: &mut Vec<starknet_ff::FieldElement>) {
        let Self {
            claim,
            interaction_pow,
            interaction_claim,
            stark_proof: sorted_stark_proof,
            channel_salt,
        } = self;
        CairoSerialize::serialize(claim, output);
        CairoSerialize::serialize(interaction_pow, output);
        CairoSerialize::serialize(interaction_claim, output);
        CairoSerialize::serialize(sorted_stark_proof, output);
        CairoSerialize::serialize(channel_salt, output);
    }
}

impl<H: MerkleHasherLifted> CairoDeserialize for CairoProofSorted<H>
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
