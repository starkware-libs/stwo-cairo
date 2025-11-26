#[cfg(feature: "poseidon252_verifier")]
mod PoseidonTest {
    use crate::fields::m31::m31;
    use crate::utils::group_columns_by_degree_bound;
    use crate::vcs::poseidon_hasher::PoseidonMerkleHasher;
    use crate::vcs::verifier::{MerkleDecommitment, MerkleVerifier, MerkleVerifierTrait};

    #[test]
    fn test_verifier() {
        let root = 0x04b89af82ac78a56cf3d53204dc5d0838e09d5d4e863ffe667c42f01077b59e0;
        let log_blowup_factor = 1;
        let degree_bound_by_column = array![3, 2, 3, 2, 2, 2, 3, 3, 2, 2];
        let decommitment = MerkleDecommitment::<
            PoseidonMerkleHasher,
        > {
            hash_witness: array![
                0x031f9875a51ab087340a79425a9c6fd6de719ad49426ae7989aedbcf6c0715e6,
                0x03b24dd37daaeb333637843bd32e53692305b37b5ee6e91346fb10e39c0dcf4c,
                0x01aa69036259fd33a58f63fdfc463cdf435d064cd20532b5767832ae9c4f4890,
                0x00ef4348a9921e79862b98432131ec9fefe2656e941ff4acdfc7d4d25076e5f4,
                0x07c041574b8c637910d6b81f190b6748f53b2518406201049a8245cd88cce07f,
                0x003ad10f628663d389c30a470c31ca6483c26d655e3e77a299e42b88d73ee62e,
                0x01927dc66c5e2db948e5da1ecf774e4accb8d634308e1bcbe2068aa7090111e4,
            ]
                .span(),
            column_witness: array![].span(),
        };
        let mut queries_per_log_size = Default::default();
        queries_per_log_size.insert(3, NullableTrait::new(array![3, 5, 7].span()));
        queries_per_log_size.insert(4, NullableTrait::new(array![7, 11, 14].span()));
        let queried_values = array![
            m31(720125469), m31(968171809), m31(364669117), m31(996158769), m31(997644238),
            m31(100529415), m31(933029034), m31(69309287), m31(194302184), m31(1057594968),
            m31(285391207), m31(420798739), m31(885772305), m31(94648313), m31(604384470),
            m31(957953858), m31(608524802), m31(428382412), m31(840979908), m31(428994537),
            m31(28706943), m31(696999129), m31(942699537), m31(330264928), m31(658446453),
            m31(992269493), m31(967997322), m31(287489501), m31(310081088), m31(409791388),
        ]
            .span();

        let column_indices_by_log_deg_bound = group_columns_by_degree_bound(
            degree_bound_by_column.span(),
        );
        assert_eq!(log_blowup_factor + column_indices_by_log_deg_bound.len() - 1, 4);
        MerkleVerifier {
            root,
            tree_height: log_blowup_factor + column_indices_by_log_deg_bound.len() - 1,
            column_indices_by_log_deg_bound,
        }
            .verify(ref queries_per_log_size, queried_values, decommitment);
    }
}
