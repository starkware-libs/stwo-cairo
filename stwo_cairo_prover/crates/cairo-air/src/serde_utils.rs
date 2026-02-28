use std::ops::Deref;

use starknet_ff::FieldElement;
use stwo::core::pcs::quotients::CommitmentSchemeProof;
use stwo::core::vcs_lifted::MerkleHasherLifted;
use stwo_cairo_serialize::CairoSerialize;

use crate::air::CairoProof;
use crate::utils::sort_and_transpose_queried_values;

impl<H: MerkleHasherLifted> CairoSerialize for CairoProof<H>
where
    H::Hash: CairoSerialize,
{
    fn serialize(&self, output: &mut Vec<FieldElement>) {
        let Self {
            claim,
            interaction_pow,
            interaction_claim,
            extended_stark_proof,
            channel_salt,
            preprocessed_trace_variant: _,
            program: _,
        } = self;
        let CommitmentSchemeProof {
            config,
            commitments,
            sampled_values,
            decommitments,
            queried_values,
            proof_of_work,
            fri_proof,
        } = &extended_stark_proof.proof.0;
        // Change the layout of the queried values to match the one expected by the Cairo verifier.
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
