use std::array;

use starknet_ff::FieldElement;
use stwo::core::fields::m31::BaseField;
use stwo::core::fields::qm31::SecureField;
use stwo::core::fri::{FriConfig, FriLayerProof, FriProof};
use stwo::core::pcs::quotients::CommitmentSchemeProof;
use stwo::core::pcs::{PcsConfig, TreeVec};
use stwo::core::poly::line::LinePoly;
use stwo::core::proof::StarkProof;
use stwo::core::vcs::blake2_hash::Blake2sHash;
use stwo::core::vcs::verifier::MerkleDecommitment;
use stwo::core::vcs::MerkleHasher;
use stwo::core::ColumnVec;
pub use stwo_cairo_serialize_derive::CairoDeserialize;

/// Deserializes types from a format serialized by corresponding `CairoSerialize` implementations.
pub trait CairoDeserialize: Sized {
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a FieldElement>) -> Self;
}

impl CairoDeserialize for u32 {
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a FieldElement>) -> Self {
        let field_elem = data.next().unwrap();
        let bytes = field_elem.to_bytes_be();
        if bytes[0..28] != [0; 28] {
            panic!("Invalid value for u32");
        }

        u32::from_be_bytes(bytes[28..32].try_into().unwrap())
    }
}

impl CairoDeserialize for u64 {
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a FieldElement>) -> Self {
        let field_elem = data.next().unwrap();
        let bytes = field_elem.to_bytes_be();
        if bytes[0..24] != [0; 24] {
            panic!("Invalid value for u64");
        }

        u64::from_be_bytes(bytes[24..32].try_into().unwrap())
    }
}

impl CairoDeserialize for usize {
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a FieldElement>) -> Self {
        <u64 as CairoDeserialize>::deserialize(data) as usize
    }
}

impl CairoDeserialize for BaseField {
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a FieldElement>) -> Self {
        BaseField::from(u32::deserialize(data))
    }
}

impl CairoDeserialize for SecureField {
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a FieldElement>) -> Self {
        let mut m31_values = [BaseField::from(0); 4];
        #[allow(clippy::needless_range_loop)]
        for i in 0..4 {
            m31_values[i] = BaseField::deserialize(data);
        }

        SecureField::from_m31_array(m31_values)
    }
}

impl<H: MerkleHasher> CairoDeserialize for MerkleDecommitment<H>
where
    H::Hash: CairoDeserialize,
{
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a FieldElement>) -> Self {
        let hash_witness = Vec::<H::Hash>::deserialize(data);
        let column_witness = Vec::<BaseField>::deserialize(data);
        MerkleDecommitment {
            hash_witness,
            column_witness,
        }
    }
}

impl CairoDeserialize for LinePoly {
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a FieldElement>) -> Self {
        let coeffs = Vec::<SecureField>::deserialize(data);
        let log_size: u32 = u32::deserialize(data);

        let expected_len = 1usize << log_size;
        if coeffs.len() != expected_len {
            panic!("Invalid length for LinePoly");
        }

        LinePoly::new(coeffs)
    }
}

impl<H: MerkleHasher> CairoDeserialize for FriLayerProof<H>
where
    H::Hash: CairoDeserialize,
{
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a FieldElement>) -> Self {
        let fri_witness = Vec::deserialize(data);
        let decommitment = MerkleDecommitment::deserialize(data);
        let commitment = H::Hash::deserialize(data);
        FriLayerProof {
            fri_witness,
            decommitment,
            commitment,
        }
    }
}

impl<H: MerkleHasher> CairoDeserialize for FriProof<H>
where
    H::Hash: CairoDeserialize,
{
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a FieldElement>) -> Self {
        let first_layer = FriLayerProof::deserialize(data);
        let inner_layers = Vec::deserialize(data);
        let last_layer_poly = LinePoly::deserialize(data);
        FriProof {
            first_layer,
            inner_layers,
            last_layer_poly,
        }
    }
}

impl CairoDeserialize for FieldElement {
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a FieldElement>) -> Self {
        data.next().copied().unwrap()
    }
}

impl CairoDeserialize for FriConfig {
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a FieldElement>) -> Self {
        let log_blowup_factor = u32::deserialize(data);
        let log_last_layer_degree_bound = u32::deserialize(data);
        let n_queries = usize::deserialize(data);
        FriConfig {
            log_blowup_factor,
            log_last_layer_degree_bound,
            n_queries,
        }
    }
}

impl CairoDeserialize for PcsConfig {
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a FieldElement>) -> Self {
        let pow_bits = u32::deserialize(data);
        let fri_config = FriConfig::deserialize(data);
        PcsConfig {
            pow_bits,
            fri_config,
        }
    }
}

impl<H: MerkleHasher> CairoDeserialize for CommitmentSchemeProof<H>
where
    H::Hash: CairoDeserialize,
{
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a FieldElement>) -> Self {
        let config = PcsConfig::deserialize(data);
        let commitments = TreeVec(Vec::<H::Hash>::deserialize(data));
        let sampled_values = TreeVec(Vec::<ColumnVec<Vec<SecureField>>>::deserialize(data));
        let decommitments = TreeVec(Vec::<MerkleDecommitment<H>>::deserialize(data));
        let queried_values = TreeVec(Vec::<Vec<BaseField>>::deserialize(data));
        let proof_of_work: u64 = u64::deserialize(data);
        let fri_proof = FriProof::deserialize(data);
        CommitmentSchemeProof {
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

impl<H: MerkleHasher> CairoDeserialize for StarkProof<H>
where
    H::Hash: CairoDeserialize,
{
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a FieldElement>) -> Self {
        let commitment_scheme_proof = CommitmentSchemeProof::deserialize(data);
        StarkProof(commitment_scheme_proof)
    }
}

impl<T: CairoDeserialize> CairoDeserialize for Option<T> {
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a FieldElement>) -> Self {
        let discriminant = data.next().unwrap();
        if *discriminant == FieldElement::ZERO {
            let value = T::deserialize(data);
            Some(value)
        } else if *discriminant == FieldElement::ONE {
            None
        } else {
            panic!("Invalid discriminant for Option<T>");
        }
    }
}

impl<T: CairoDeserialize, const N: usize> CairoDeserialize for [T; N] {
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a FieldElement>) -> Self {
        array::from_fn(|_| T::deserialize(data))
    }
}

impl<T: CairoDeserialize> CairoDeserialize for Vec<T> {
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a FieldElement>) -> Self {
        let len: usize = usize::deserialize(data);

        (0..len).map(|_| T::deserialize(data)).collect()
    }
}

impl<T0: CairoDeserialize, T1: CairoDeserialize> CairoDeserialize for (T0, T1) {
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a FieldElement>) -> Self {
        let v0 = T0::deserialize(data);
        let v1 = T1::deserialize(data);
        (v0, v1)
    }
}

impl<T0: CairoDeserialize, T1: CairoDeserialize, T2: CairoDeserialize> CairoDeserialize
    for (T0, T1, T2)
{
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a FieldElement>) -> Self {
        let v0 = T0::deserialize(data);
        let v1 = T1::deserialize(data);
        let v2 = T2::deserialize(data);
        (v0, v1, v2)
    }
}

impl CairoDeserialize for Blake2sHash {
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a FieldElement>) -> Self {
        let mut bytes = [0u8; 32];
        for byte_chunk in bytes.array_chunks_mut() {
            let v: u32 = u32::deserialize(data);
            *byte_chunk = v.to_le_bytes();
        }
        Blake2sHash(bytes)
    }
}
