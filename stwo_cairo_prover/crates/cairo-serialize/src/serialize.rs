use starknet_ff::FieldElement;
use stwo::core::fields::m31::BaseField;
use stwo::core::fields::qm31::SecureField;
use stwo::core::fri::{FriConfig, FriLayerProof, FriProof};
use stwo::core::pcs::quotients::CommitmentSchemeProof;
use stwo::core::pcs::PcsConfig;
use stwo::core::poly::line::LinePoly;
use stwo::core::proof::StarkProof;
use stwo::core::vcs::blake2_hash::Blake2sHash;
use stwo::core::vcs::verifier::MerkleDecommitment;
use stwo::core::vcs::MerkleHasher;
// Make derive macro available.
pub use stwo_cairo_serialize_derive::CairoSerialize;

/// Serializes types into a format for deserialization by corresponding types in a Cairo program.
pub trait CairoSerialize {
    fn serialize(&self, output: &mut Vec<FieldElement>);
}

impl CairoSerialize for u32 {
    fn serialize(&self, output: &mut Vec<FieldElement>) {
        output.push((*self).into());
    }
}

impl CairoSerialize for u64 {
    fn serialize(&self, output: &mut Vec<FieldElement>) {
        output.push((*self).into());
    }
}

impl CairoSerialize for usize {
    fn serialize(&self, output: &mut Vec<FieldElement>) {
        output.push((*self).into());
    }
}

impl CairoSerialize for BaseField {
    fn serialize(&self, output: &mut Vec<FieldElement>) {
        output.push(self.0.into());
    }
}

impl CairoSerialize for SecureField {
    fn serialize(&self, output: &mut Vec<FieldElement>) {
        output.extend(self.to_m31_array().map(|c| FieldElement::from(c.0)));
    }
}

impl<H: MerkleHasher> CairoSerialize for MerkleDecommitment<H>
where
    H::Hash: CairoSerialize,
{
    fn serialize(&self, output: &mut Vec<FieldElement>) {
        let Self {
            hash_witness,
            column_witness,
        } = self;
        hash_witness.serialize(output);
        column_witness.serialize(output);
    }
}

impl CairoSerialize for LinePoly {
    fn serialize(&self, output: &mut Vec<FieldElement>) {
        (**self).serialize(output);
        output.push((self.len().ilog2()).into());
    }
}

impl<H: MerkleHasher> CairoSerialize for FriLayerProof<H>
where
    H::Hash: CairoSerialize,
{
    fn serialize(&self, output: &mut Vec<FieldElement>) {
        let Self {
            fri_witness,
            decommitment,
            commitment,
        } = self;
        fri_witness.serialize(output);
        decommitment.serialize(output);
        commitment.serialize(output);
    }
}

impl<H: MerkleHasher> CairoSerialize for FriProof<H>
where
    H::Hash: CairoSerialize,
{
    fn serialize(&self, output: &mut Vec<FieldElement>) {
        let Self {
            first_layer,
            inner_layers,
            last_layer_poly,
        } = self;
        first_layer.serialize(output);
        inner_layers.serialize(output);
        last_layer_poly.serialize(output);
    }
}

impl CairoSerialize for FieldElement {
    fn serialize(&self, output: &mut Vec<FieldElement>) {
        output.push(*self);
    }
}

impl CairoSerialize for FriConfig {
    fn serialize(&self, output: &mut Vec<FieldElement>) {
        let Self {
            log_blowup_factor,
            log_last_layer_degree_bound,
            n_queries,
        } = self;
        log_blowup_factor.serialize(output);
        log_last_layer_degree_bound.serialize(output);
        n_queries.serialize(output);
    }
}

impl CairoSerialize for PcsConfig {
    fn serialize(&self, output: &mut Vec<FieldElement>) {
        let Self {
            pow_bits,
            fri_config,
        } = self;
        pow_bits.serialize(output);
        fri_config.serialize(output);
    }
}

impl<H: MerkleHasher> CairoSerialize for CommitmentSchemeProof<H>
where
    H::Hash: CairoSerialize,
{
    fn serialize(&self, output: &mut Vec<FieldElement>) {
        let Self {
            config,
            commitments,
            sampled_values,
            decommitments,
            queried_values,
            proof_of_work,
            fri_proof,
        } = self;
        config.serialize(output);
        commitments.serialize(output);
        sampled_values.serialize(output);
        decommitments.serialize(output);
        queried_values.serialize(output);
        output.push((*proof_of_work).into());
        fri_proof.serialize(output);
    }
}

impl<H: MerkleHasher> CairoSerialize for StarkProof<H>
where
    H::Hash: CairoSerialize,
{
    fn serialize(&self, output: &mut Vec<FieldElement>) {
        let Self(commitment_scheme_proof) = self;
        commitment_scheme_proof.serialize(output);
    }
}

impl<T: CairoSerialize> CairoSerialize for Option<T> {
    fn serialize(&self, output: &mut Vec<FieldElement>) {
        match self {
            Some(v) => {
                output.push(FieldElement::ZERO);
                v.serialize(output);
            }
            None => output.push(FieldElement::ONE),
        }
    }
}

impl<T: CairoSerialize> CairoSerialize for [T] {
    fn serialize(&self, output: &mut Vec<FieldElement>) {
        output.push(self.len().into());
        self.iter().for_each(|v| v.serialize(output));
    }
}

impl<T: CairoSerialize, const N: usize> CairoSerialize for [T; N] {
    fn serialize(&self, output: &mut Vec<FieldElement>) {
        self.iter().for_each(|v| v.serialize(output));
    }
}

impl<T: CairoSerialize> CairoSerialize for Vec<T> {
    fn serialize(&self, output: &mut Vec<FieldElement>) {
        (**self).serialize(output);
    }
}

impl<T0: CairoSerialize, T1: CairoSerialize> CairoSerialize for (T0, T1) {
    fn serialize(&self, output: &mut Vec<FieldElement>) {
        let (v0, v1) = self;
        v0.serialize(output);
        v1.serialize(output);
    }
}

impl<T0: CairoSerialize, T1: CairoSerialize, T2: CairoSerialize> CairoSerialize for (T0, T1, T2) {
    fn serialize(&self, output: &mut Vec<FieldElement>) {
        let (v0, v1, v2) = self;
        v0.serialize(output);
        v1.serialize(output);
        v2.serialize(output);
    }
}

impl CairoSerialize for Blake2sHash {
    fn serialize(&self, output: &mut Vec<FieldElement>) {
        for byte_chunk in self.0.array_chunks() {
            let v = u32::from_le_bytes(*byte_chunk);
            CairoSerialize::serialize(&v, output);
        }
    }
}
