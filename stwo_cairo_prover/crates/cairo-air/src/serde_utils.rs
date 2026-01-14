use std::ops::Deref;

use starknet_ff::FieldElement;
use stwo::core::fields::m31::BaseField;
use stwo::core::fields::qm31::SecureField;
use stwo::core::fri::FriProof;
use stwo::core::pcs::quotients::CommitmentSchemeProof;
use stwo::core::pcs::{PcsConfig, TreeVec};
use stwo::core::proof::StarkProof;
use stwo::core::vcs_lifted::verifier::MerkleDecommitmentLifted;
use stwo::core::vcs_lifted::MerkleHasherLifted;
use stwo::core::ColumnVec;
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};

use crate::air::CairoClaim;
use crate::utils::{sort_and_transpose_queried_values, unsort_and_transpose_queried_values};
use crate::CairoProof;

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
        let CommitmentSchemeProof {
            config,
            commitments,
            sampled_values,
            decommitments,
            queried_values,
            proof_of_work,
            fri_proof,
        } = &stark_proof.0;

        let trace_and_interaction_trace_log_sizes = claim.log_sizes();
        let sorted_queried_values = sort_and_transpose_queried_values(
            queried_values,
            trace_and_interaction_trace_log_sizes
                .iter()
                .map(|c| c.as_slice())
                .collect(),
        );

        CairoSerialize::serialize(claim, output);
        CairoSerialize::serialize(interaction_pow, output);
        CairoSerialize::serialize(interaction_claim, output);
        // Serialize the commitment scheme proof.
        CairoSerialize::serialize(config, output);
        CairoSerialize::serialize(commitments.deref(), output);
        CairoSerialize::serialize(sampled_values.deref(), output);
        CairoSerialize::serialize(decommitments.deref(), output);
        CairoSerialize::serialize(sorted_queried_values.deref(), output);
        CairoSerialize::serialize(proof_of_work, output);
        CairoSerialize::serialize(fri_proof, output);

        CairoSerialize::serialize(channel_salt, output);
    }
}

#[derive(Debug)]
struct Cairo1CommitmentSchemeProof<H: MerkleHasherLifted> {
    pub config: PcsConfig,
    pub commitments: TreeVec<H::Hash>,
    pub sampled_values: TreeVec<ColumnVec<Vec<SecureField>>>,
    pub decommitments: TreeVec<MerkleDecommitmentLifted<H>>,
    pub queried_values: TreeVec<Vec<BaseField>>,
    pub proof_of_work: u64,
    pub fri_proof: FriProof<H>,
}

pub fn deserialize_cairo1_proof<'a, H: MerkleHasherLifted>(
    data: &mut impl Iterator<Item = &'a FieldElement>,
    n_preprocessed_cols: usize,
) -> CairoProof<H>
where
    H::Hash: CairoDeserialize,
{
    let claim: CairoClaim = CairoDeserialize::deserialize(data);
    let interaction_pow = CairoDeserialize::deserialize(data);
    let interaction_claim = CairoDeserialize::deserialize(data);
    let cairo1_stark_proof = CairoDeserialize::deserialize(data);
    let channel_salt = CairoDeserialize::deserialize(data);

    let Cairo1CommitmentSchemeProof {
        config,
        commitments,
        sampled_values,
        decommitments,
        queried_values,
        proof_of_work,
        fri_proof,
    } = cairo1_stark_proof;
    let trace_and_interaction_trace_log_sizes = claim.log_sizes();

    let unsorted_queried_values = unsort_and_transpose_queried_values(
        &queried_values,
        n_preprocessed_cols,
        trace_and_interaction_trace_log_sizes
            .iter()
            .map(|c| c.as_slice())
            .collect(),
    );
    let stark_proof = CommitmentSchemeProof {
        config,
        commitments,
        sampled_values,
        decommitments,
        queried_values: unsorted_queried_values,
        proof_of_work,
        fri_proof,
    };
    CairoProof {
        claim,
        interaction_pow,
        interaction_claim,
        stark_proof: StarkProof(stark_proof),
        channel_salt,
    }
}

impl<H: MerkleHasherLifted> CairoSerialize for Cairo1CommitmentSchemeProof<H>
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

impl<H: MerkleHasherLifted> CairoDeserialize for Cairo1CommitmentSchemeProof<H>
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
