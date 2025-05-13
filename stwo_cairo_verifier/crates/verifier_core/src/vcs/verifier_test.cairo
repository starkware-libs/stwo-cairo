#[cfg(test)]
mod tests {
    use core::array::ToSpanTrait;
    use core::dict::Felt252DictTrait;
    use core::nullable::NullableTrait;
    use core::result::ResultTrait;
    use crate::fields::m31::m31;
    use crate::vcs::poseidon_hasher::PoseidonMerkleHasher;
    use crate::vcs::verifier::{MerkleDecommitment, MerkleVerifier, MerkleVerifierImpl};

    #[test]
    fn test_verifier() {
        let root = 0x06e3a2499c5ee8a2a66f536f30640b9b67cb50092642003b64a60c401e280214;
        let column_log_sizes = array![4, 3, 4, 3, 3, 3, 4, 4, 3, 3];
        let decommitment = MerkleDecommitment::<
            PoseidonMerkleHasher,
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
