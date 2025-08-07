use std::array;

use starknet_ff::FieldElement;
use stwo::core::fields::cm31::CM31;
use stwo::core::fields::m31::{BaseField, P};
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
// Make derive macro available.
pub use stwo_cairo_serialize_derive::CompactBinary;
use unsigned_varint::encode::{u32_buffer, u64_buffer, usize_buffer};
use unsigned_varint::{decode, encode};

pub trait CompactBinary {
    /// Serializes the object into a compact binary format.
    fn compact_serialize(&self, output: &mut Vec<u8>);

    /// Deserializes the object from a compact binary format.
    fn compact_deserialize(input: &[u8]) -> (&[u8], Self)
    where
        Self: Sized;
}

/// Helper function to convert a byte slice into an array of a specific size from a closure if
/// possible.
pub fn buf_to_array_ctr<F: Fn(&[u8; N]) -> V, V, const N: usize>(
    buf: &[u8],
    ctr: F,
) -> Option<(&[u8], V)> {
    Some((&buf[N..], ctr(&buf.get(..N)?.try_into().ok()?)))
}

/// Helper function to deserialize a struct's version and check it against an expected value.
pub fn strip_expected_version(input: &[u8], expected_version: u32) -> &[u8] {
    let (input, version) = u32::compact_deserialize(input);
    assert_eq!(
        version, expected_version,
        "Unexpected version during deserialization"
    );
    input
}

/// Helper function to deserialize a field's tag and check it against an expected value.
pub fn strip_expected_tag(input: &[u8], expected_tag: usize) -> &[u8] {
    let (input, tag) = usize::compact_deserialize(input);
    assert_eq!(tag, expected_tag, "Unexpected tag during deserialization");
    input
}

impl CompactBinary for u32 {
    fn compact_serialize(&self, output: &mut Vec<u8>) {
        output.extend_from_slice(encode::u32(*self, &mut u32_buffer()));
    }

    fn compact_deserialize(input: &[u8]) -> (&[u8], Self) {
        let (value, input) = decode::u32(input).unwrap();
        (input, value)
    }
}

impl CompactBinary for u64 {
    fn compact_serialize(&self, output: &mut Vec<u8>) {
        output.extend_from_slice(encode::u64(*self, &mut u64_buffer()));
    }

    fn compact_deserialize(input: &[u8]) -> (&[u8], Self) {
        let (value, input) = decode::u64(input).unwrap();
        (input, value)
    }
}

impl CompactBinary for usize {
    fn compact_serialize(&self, output: &mut Vec<u8>) {
        output.extend_from_slice(encode::usize(*self, &mut usize_buffer()));
    }

    fn compact_deserialize(input: &[u8]) -> (&[u8], Self) {
        let (value, input) = decode::usize(input).unwrap();
        (input, value)
    }
}

impl CompactBinary for BaseField {
    fn compact_serialize(&self, output: &mut Vec<u8>) {
        output.extend_from_slice(&self.0.to_be_bytes());
    }

    fn compact_deserialize(input: &[u8]) -> (&[u8], Self) {
        let (input, u32_value) = buf_to_array_ctr(input, |v| u32::from_be_bytes(*v)).unwrap();

        if u32_value > P {
            panic!("Field element is too large");
        } else {
            (input, BaseField::from_u32_unchecked(u32_value))
        }
    }
}

impl CompactBinary for CM31 {
    fn compact_serialize(&self, output: &mut Vec<u8>) {
        self.0.compact_serialize(output);
        self.1.compact_serialize(output);
    }

    fn compact_deserialize(input: &[u8]) -> (&[u8], Self) {
        let (input, u32_value_0) = BaseField::compact_deserialize(input);
        let (input, u32_value_1) = BaseField::compact_deserialize(input);
        (input, CM31::from_m31(u32_value_0, u32_value_1))
    }
}

impl CompactBinary for SecureField {
    fn compact_serialize(&self, output: &mut Vec<u8>) {
        self.0.compact_serialize(output);
        self.1.compact_serialize(output);
    }

    fn compact_deserialize(input: &[u8]) -> (&[u8], Self) {
        let (input, m31_value_0) = BaseField::compact_deserialize(input);
        let (input, m31_value_1) = BaseField::compact_deserialize(input);
        let (input, m31_value_2) = BaseField::compact_deserialize(input);
        let (input, m31_value_3) = BaseField::compact_deserialize(input);
        (
            input,
            SecureField::from_m31(m31_value_0, m31_value_1, m31_value_2, m31_value_3),
        )
    }
}

impl<H: MerkleHasher> CompactBinary for MerkleDecommitment<H>
where
    H::Hash: CompactBinary,
{
    fn compact_serialize(&self, output: &mut Vec<u8>) {
        let Self {
            hash_witness,
            column_witness,
        } = self;
        let version = 0;
        let to_serialize: Vec<(usize, &dyn CompactBinary)> =
            vec![(0, hash_witness), (1, column_witness)];
        u32::compact_serialize(&version, output);
        for (tag, value) in to_serialize {
            usize::compact_serialize(&tag, output);
            value.compact_serialize(output);
        }
    }

    fn compact_deserialize(input: &[u8]) -> (&[u8], Self) {
        let input = strip_expected_version(input, 0);
        let input = strip_expected_tag(input, 0);
        let (input, hash_witness) = Vec::<H::Hash>::compact_deserialize(input);
        let input = strip_expected_tag(input, 1);
        let (input, column_witness) = Vec::<BaseField>::compact_deserialize(input);
        (
            input,
            MerkleDecommitment {
                hash_witness,
                column_witness,
            },
        )
    }
}

impl CompactBinary for LinePoly {
    fn compact_serialize(&self, output: &mut Vec<u8>) {
        let coeffs = self.clone().into_ordered_coefficients();
        coeffs.len().compact_serialize(output);
        coeffs.iter().for_each(|c| c.compact_serialize(output));
    }

    fn compact_deserialize(input: &[u8]) -> (&[u8], Self) {
        let (mut input, len) = usize::compact_deserialize(input);
        let mut coeffs = Vec::with_capacity(len);
        for _ in 0..len {
            let (updated_input, coeff) = SecureField::compact_deserialize(input);
            input = updated_input;
            coeffs.push(coeff);
        }
        (input, LinePoly::from_ordered_coefficients(coeffs))
    }
}

impl<H: MerkleHasher> CompactBinary for FriLayerProof<H>
where
    H::Hash: CompactBinary,
{
    fn compact_serialize(&self, output: &mut Vec<u8>) {
        let Self {
            fri_witness,
            decommitment,
            commitment,
        } = self;
        let version = 0;
        let to_serialize: Vec<(usize, &dyn CompactBinary)> =
            vec![(0, fri_witness), (1, decommitment), (2, commitment)];
        u32::compact_serialize(&version, output);
        for (tag, value) in to_serialize {
            usize::compact_serialize(&tag, output);
            value.compact_serialize(output);
        }
    }

    fn compact_deserialize(input: &[u8]) -> (&[u8], Self) {
        let input = strip_expected_version(input, 0);
        let input = strip_expected_tag(input, 0);
        let (input, fri_witness) = Vec::<SecureField>::compact_deserialize(input);
        let input = strip_expected_tag(input, 1);
        let (input, decommitment) = MerkleDecommitment::compact_deserialize(input);
        let input = strip_expected_tag(input, 2);
        let (input, commitment) = H::Hash::compact_deserialize(input);
        (
            input,
            FriLayerProof {
                fri_witness,
                decommitment,
                commitment,
            },
        )
    }
}

impl<H: MerkleHasher> CompactBinary for FriProof<H>
where
    H::Hash: CompactBinary,
{
    fn compact_serialize(&self, output: &mut Vec<u8>) {
        let Self {
            first_layer,
            inner_layers,
            last_layer_poly,
        } = self;
        let version = 0;
        let to_serialize: Vec<(usize, &dyn CompactBinary)> =
            vec![(0, first_layer), (1, inner_layers), (2, last_layer_poly)];
        u32::compact_serialize(&version, output);
        for (tag, value) in to_serialize {
            usize::compact_serialize(&tag, output);
            value.compact_serialize(output);
        }
    }

    fn compact_deserialize(input: &[u8]) -> (&[u8], Self) {
        let input = strip_expected_version(input, 0);
        let input = strip_expected_tag(input, 0);
        let (input, first_layer) = FriLayerProof::compact_deserialize(input);
        let input = strip_expected_tag(input, 1);
        let (input, inner_layers) = Vec::<FriLayerProof<H>>::compact_deserialize(input);
        let input = strip_expected_tag(input, 2);
        let (input, last_layer_poly) = LinePoly::compact_deserialize(input);
        (
            input,
            FriProof {
                first_layer,
                inner_layers,
                last_layer_poly,
            },
        )
    }
}

impl CompactBinary for FieldElement {
    fn compact_serialize(&self, output: &mut Vec<u8>) {
        output.extend_from_slice(&self.to_bytes_be());
    }

    fn compact_deserialize(input: &[u8]) -> (&[u8], Self) {
        let (input, field_elem) =
            buf_to_array_ctr(input, |v| FieldElement::from_bytes_be(v).unwrap()).unwrap();
        (input, field_elem)
    }
}

impl CompactBinary for FriConfig {
    fn compact_serialize(&self, output: &mut Vec<u8>) {
        let Self {
            log_blowup_factor,
            log_last_layer_degree_bound,
            n_queries,
        } = self;
        let version = 0;
        let to_serialize: Vec<(usize, &dyn CompactBinary)> = vec![
            (0, log_blowup_factor),
            (1, log_last_layer_degree_bound),
            (2, n_queries),
        ];
        u32::compact_serialize(&version, output);
        for (tag, value) in to_serialize {
            usize::compact_serialize(&tag, output);
            value.compact_serialize(output);
        }
    }

    fn compact_deserialize(input: &[u8]) -> (&[u8], Self) {
        let input = strip_expected_version(input, 0);
        let input = strip_expected_tag(input, 0);
        let (input, log_blowup_factor) = u32::compact_deserialize(input);
        let input = strip_expected_tag(input, 1);
        let (input, log_last_layer_degree_bound) = u32::compact_deserialize(input);
        let input = strip_expected_tag(input, 2);
        let (input, n_queries) = usize::compact_deserialize(input);
        (
            input,
            FriConfig {
                log_blowup_factor,
                log_last_layer_degree_bound,
                n_queries,
            },
        )
    }
}

impl CompactBinary for PcsConfig {
    fn compact_serialize(&self, output: &mut Vec<u8>) {
        let Self {
            pow_bits,
            fri_config,
        } = self;
        let version = 0;
        let to_serialize: Vec<(usize, &dyn CompactBinary)> = vec![(0, pow_bits), (1, fri_config)];
        u32::compact_serialize(&version, output);
        for (tag, value) in to_serialize {
            usize::compact_serialize(&tag, output);
            value.compact_serialize(output);
        }
    }

    fn compact_deserialize(input: &[u8]) -> (&[u8], Self) {
        let input = strip_expected_version(input, 0);
        let input = strip_expected_tag(input, 0);
        let (input, pow_bits) = u32::compact_deserialize(input);
        let input = strip_expected_tag(input, 1);
        let (input, fri_config) = FriConfig::compact_deserialize(input);
        (
            input,
            PcsConfig {
                pow_bits,
                fri_config,
            },
        )
    }
}

impl<H: MerkleHasher> CompactBinary for CommitmentSchemeProof<H>
where
    H::Hash: CompactBinary,
{
    fn compact_serialize(&self, output: &mut Vec<u8>) {
        let Self {
            config,
            commitments,
            sampled_values,
            decommitments,
            queried_values,
            proof_of_work,
            fri_proof,
        } = self;
        let version = 0;
        let to_serialize: Vec<(usize, &dyn CompactBinary)> = vec![
            (0, config),
            (1, &commitments.0),
            (2, &sampled_values.0),
            (3, &decommitments.0),
            (4, &queried_values.0),
            (5, proof_of_work),
            (6, fri_proof),
        ];
        u32::compact_serialize(&version, output);
        for (tag, value) in to_serialize {
            usize::compact_serialize(&tag, output);
            value.compact_serialize(output);
        }
    }

    fn compact_deserialize(input: &[u8]) -> (&[u8], Self) {
        let input = strip_expected_version(input, 0);
        let input = strip_expected_tag(input, 0);
        let (input, config) = PcsConfig::compact_deserialize(input);
        let input = strip_expected_tag(input, 1);
        let (input, commitments) = Vec::<H::Hash>::compact_deserialize(input);
        let input = strip_expected_tag(input, 2);
        let (input, sampled_values) =
            Vec::<ColumnVec<Vec<SecureField>>>::compact_deserialize(input);
        let input = strip_expected_tag(input, 3);
        let (input, decommitments) = Vec::<MerkleDecommitment<H>>::compact_deserialize(input);
        let input = strip_expected_tag(input, 4);
        let (input, queried_values) = Vec::<Vec<BaseField>>::compact_deserialize(input);
        let input = strip_expected_tag(input, 5);
        let (input, proof_of_work) = u64::compact_deserialize(input);
        let input = strip_expected_tag(input, 6);
        let (input, fri_proof) = FriProof::compact_deserialize(input);
        (
            input,
            CommitmentSchemeProof {
                config,
                commitments: TreeVec::new(commitments),
                sampled_values: TreeVec::new(sampled_values),
                decommitments: TreeVec::new(decommitments),
                queried_values: TreeVec::new(queried_values),
                proof_of_work,
                fri_proof,
            },
        )
    }
}

impl<H: MerkleHasher> CompactBinary for StarkProof<H>
where
    H::Hash: CompactBinary,
{
    fn compact_serialize(&self, output: &mut Vec<u8>) {
        let Self(commitment_scheme_proof) = self;
        let version = 0;
        let to_serialize: Vec<(usize, &dyn CompactBinary)> = vec![(0, commitment_scheme_proof)];
        u32::compact_serialize(&version, output);
        for (tag, value) in to_serialize {
            usize::compact_serialize(&tag, output);
            value.compact_serialize(output);
        }
    }

    // TODO: Add Versioning and tags
    fn compact_deserialize(input: &[u8]) -> (&[u8], Self) {
        let input = strip_expected_version(input, 0);
        let input = strip_expected_tag(input, 0);
        let (input, commitment_scheme_proof) = CommitmentSchemeProof::compact_deserialize(input);
        (input, StarkProof(commitment_scheme_proof))
    }
}

impl<T: CompactBinary> CompactBinary for Option<T> {
    fn compact_serialize(&self, output: &mut Vec<u8>) {
        if let Some(value) = self {
            output.push(b'1');
            value.compact_serialize(output);
        } else {
            output.push(b'0');
        }
    }

    fn compact_deserialize(input: &[u8]) -> (&[u8], Self) {
        if *input.first().unwrap() == b'1' {
            let input = &input[1..];
            let (input, value) = T::compact_deserialize(input);
            (input, Some(value))
        } else {
            let input = &input[1..];
            (input, None)
        }
    }
}

impl<T: CompactBinary + Clone, const N: usize> CompactBinary for [T; N] {
    fn compact_serialize(&self, output: &mut Vec<u8>) {
        self.iter().for_each(|v| v.compact_serialize(output));
    }

    fn compact_deserialize(input: &[u8]) -> (&[u8], Self) {
        let mut input = input;
        let mut values = Vec::with_capacity(N);
        for _ in 0..N {
            let (updated_input, value) = T::compact_deserialize(input);
            input = updated_input;
            values.push(value);
        }
        (input, array::from_fn(|i| values[i].clone()))
    }
}

impl<T: CompactBinary> CompactBinary for Vec<T> {
    fn compact_serialize(&self, output: &mut Vec<u8>) {
        self.len().compact_serialize(output);
        self.iter().for_each(|v| v.compact_serialize(output));
    }

    fn compact_deserialize(input: &[u8]) -> (&[u8], Self) {
        let (mut input, len) = usize::compact_deserialize(input);
        let mut values = Vec::with_capacity(len);
        for _ in 0..len {
            let (updated_input, value) = T::compact_deserialize(input);
            input = updated_input;
            values.push(value);
        }
        (input, values)
    }
}

impl<T0: CompactBinary, T1: CompactBinary> CompactBinary for (T0, T1) {
    fn compact_serialize(&self, output: &mut Vec<u8>) {
        let (v0, v1) = self;
        v0.compact_serialize(output);
        v1.compact_serialize(output);
    }

    fn compact_deserialize(input: &[u8]) -> (&[u8], Self) {
        let (input, v0) = T0::compact_deserialize(input);
        let (input, v1) = T1::compact_deserialize(input);
        (input, (v0, v1))
    }
}

impl<T0: CompactBinary, T1: CompactBinary, T2: CompactBinary> CompactBinary for (T0, T1, T2) {
    fn compact_serialize(&self, output: &mut Vec<u8>) {
        let (v0, v1, v2) = self;
        v0.compact_serialize(output);
        v1.compact_serialize(output);
        v2.compact_serialize(output);
    }

    fn compact_deserialize(input: &[u8]) -> (&[u8], Self) {
        let (input, v0) = T0::compact_deserialize(input);
        let (input, v1) = T1::compact_deserialize(input);
        let (input, v2) = T2::compact_deserialize(input);
        (input, (v0, v1, v2))
    }
}

impl CompactBinary for Blake2sHash {
    fn compact_serialize(&self, output: &mut Vec<u8>) {
        output.extend_from_slice(&self.0);
    }

    fn compact_deserialize(input: &[u8]) -> (&[u8], Self) {
        let (input, hash) = buf_to_array_ctr(input, |v| Blake2sHash(*v)).unwrap();
        (input, hash)
    }
}
