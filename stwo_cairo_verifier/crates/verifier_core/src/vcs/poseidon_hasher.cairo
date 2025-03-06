use core::array::ArrayTrait;
use core::hash::HashStateTrait;
use core::num::traits::Zero;
use core::poseidon::{HashState, hades_permutation, poseidon_hash_span};
use crate::fields::m31::M31_SHIFT;
use crate::BaseField;
use super::MerkleHasher;

/// 8 M31 elements fit in a hash, since 31*8 = 242 < 252.
const M31_ELEMENETS_IN_HASH: usize = 8;

/// Equals `(2^31)^4`.
const M31_SHIFT_POW_4: felt252 = M31_SHIFT * M31_SHIFT * M31_SHIFT * M31_SHIFT;

pub impl PoseidonMerkleHasherImpl of MerkleHasher {
    type Hash = felt252;

    fn hash_node(
        children_hashes: Option<(Self::Hash, Self::Hash)>, mut column_values: Span<BaseField>,
    ) -> Self::Hash {
        let mut hash_array: Array<felt252> = Default::default();
        if let Some((x, y)) = children_hashes {
            // Most often a node has no column values.
            if column_values.len() == 0 {
                // Inlined and simplified `poseidon_hash_span(...)` for better performance.
                // TODO: Posiedon2 here { == hades_permutation(x, y, 2); }
                let (s0, s1, s2) = hades_permutation(x, y, 0);
                let hash_state = HashState { s0, s1, s2, odd: false };
                return hash_state.finalize();
            }
            hash_array.append(x);
            hash_array.append(y);
        } else {
            // Most often a single QM31 column commitment due to FRI.
            // TODO(andrew): Implement non-mixed degree merkle for FRI decommitments.
            if let Some(values) = column_values.try_into() {
                // Inlined and simplified `poseidon_hash_span(...)` for better performance.
                let [v0, v1, v2, v3]: [BaseField; 4] = (*values).unbox();
                let mut word = v0.into();
                word = word * M31_SHIFT + v1.into();
                word = word * M31_SHIFT + v2.into();
                word = word * M31_SHIFT + v3.into();
                word = word * M31_SHIFT_POW_4;
                let (hash, _, _) = hades_permutation(word, 1, 0);
                return hash;
            }
        }

        // TODO(andrew): Measure performance diff and consider inlining `poseidon_hash_span(..)`
        // functionality here to do all packing and hashing in a single pass.
        while let Some(values) = column_values.multi_pop_front::<8>() {
            let [v0, v1, v2, v3, v4, v5, v6, v7] = (*values).unbox();
            let mut word = v0.into();
            word = word * M31_SHIFT + v1.into();
            word = word * M31_SHIFT + v2.into();
            word = word * M31_SHIFT + v3.into();
            word = word * M31_SHIFT + v4.into();
            word = word * M31_SHIFT + v5.into();
            word = word * M31_SHIFT + v6.into();
            word = word * M31_SHIFT + v7.into();
            hash_array.append(word);
        }

        if !column_values.is_empty() {
            let mut word = (*column_values.pop_front().unwrap_or(@Zero::zero())).into();

            for _ in 1..M31_ELEMENETS_IN_HASH {
                let v = (*column_values.pop_front().unwrap_or(@Zero::zero())).into();
                word = word * M31_SHIFT + v;
            }

            hash_array.append(word);
        }

        poseidon_hash_span(hash_array.span())
    }
}

#[cfg(test)]
mod tests {
    use core::array::ToSpanTrait;
    use core::dict::Felt252DictTrait;
    use core::nullable::NullableTrait;
    use core::result::ResultTrait;
    use crate::vcs::verifier::{MerkleDecommitment, MerkleVerifier, MerkleVerifierTrait};
    use crate::fields::m31::m31;
    use super::PoseidonMerkleHasherImpl;


    #[test]
    fn test_m31() {
        assert_eq!(
            PoseidonMerkleHasherImpl::hash_node(None, array![m31(0), m31(1)].span()),
            2552053700073128806553921687214114320458351061521275103654266875084493044716,
        );

        assert_eq!(
            PoseidonMerkleHasherImpl::hash_node(Some((1, 2)), array![m31(3)].span()),
            159358216886023795422515519110998391754567506678525778721401012606792642769,
        );
    }

    #[test]
    fn test_verifier() {
        let root = 0x06e3a2499c5ee8a2a66f536f30640b9b67cb50092642003b64a60c401e280214;
        let column_log_sizes = array![4, 3, 4, 3, 3, 3, 4, 4, 3, 3];
        let decommitment = MerkleDecommitment::<
            PoseidonMerkleHasherImpl,
        > {
            hash_witness: array![
                0x037056abc40b9e8c2a67826f54a8c379b0b3ef46629e6a19609e1144bf230f36,
                0x068708ce1c3fc019a43494bd262e87fc70e5c1f68f42881f120fe90ea2bf2201,
                0x01270a97c943188a4aa8a839687ff6d2681b070d1d1627466b93843ad26f4cb2,
                0x06be4322e37fe02371c14436674765da25109e9bc3af4a683c9afea63eb3bdc3,
                0x0360c78816d1d60758c67c011dcd82396a2ccf85fe49ea45667e3cb9feca3f40,
                0x01b4e5f9533e652324ab6b5747edc3343db8f1b9432cdcf2e5ea54fa156ba483,
                0x04a389ddc8e37da68b73c185460f372a5ed8a09eab0f51c63578776db8d1b5ae,
                0x03adfd255329a9a3d49792362f34630fd6b04cc7efdb3a6a175c70b988915cdc,
            ]
                .span(),
            column_witness: array![
                m31(885772305),
                m31(94648313),
                m31(604384470),
                m31(957953858),
                m31(608524802),
                m31(428382412),
            ]
                .span(),
        };
        let mut queries_per_log_size = Default::default();
        queries_per_log_size.insert(3, NullableTrait::new(array![2, 5, 7].span()));
        queries_per_log_size.insert(4, NullableTrait::new(array![7, 11, 14].span()));
        let queried_values = array![
            m31(720125469),
            m31(968171809),
            m31(364669117),
            m31(996158769),
            m31(997644238),
            m31(100529415),
            m31(933029034),
            m31(69309287),
            m31(194302184),
            m31(1057594968),
            m31(285391207),
            m31(420798739),
            m31(122725140),
            m31(1012109813),
            m31(766295003),
            m31(552345729),
            m31(650584843),
            m31(71167745),
            m31(840979908),
            m31(428994537),
            m31(28706943),
            m31(696999129),
            m31(942699537),
            m31(330264928),
            m31(658446453),
            m31(992269493),
            m31(967997322),
            m31(287489501),
            m31(310081088),
            m31(409791388),
        ]
            .span();
        MerkleVerifier { root, column_log_sizes }
            .verify(queries_per_log_size, queried_values, decommitment)
            .expect('verification failed');
    }
}
