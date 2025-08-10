use crate::fields::m31::m31;
use crate::utils::group_columns_by_log_size;
use crate::vcs::poseidon_hasher::PoseidonMerkleHasher;
use crate::vcs::verifier::{MerkleDecommitment, MerkleVerifier, MerkleVerifierTrait};

#[test]
fn test_verifier() {
    let root = 0x487d4619a3b49e2d33f289d6a47f4cdf3f71a03f48ac7012d36c2da9ed91d80;
    let column_log_sizes = array![4, 3, 4, 3, 3, 3, 4, 4, 3, 3];
    let decommitment = MerkleDecommitment::<
        PoseidonMerkleHasher,
    > {
        hash_witness: array![
            0x5c76ecdbaa8b3a6304fb93f5fdf88e4280531ef9dc3dbcde458bd94bfa26ae8,
            0x108997bc3a7f5fe7fc54dfa9d35d17f5aee79947219e205ac76409db6bf50ca,
            0x31f9875a51ab087340a79425a9c6fd6de719ad49426ae7989aedbcf6c0715e6,
            0x3b24dd37daaeb333637843bd32e53692305b37b5ee6e91346fb10e39c0dcf4c,
            0x1aa69036259fd33a58f63fdfc463cdf435d064cd20532b5767832ae9c4f4890,
            0x7c041574b8c637910d6b81f190b6748f53b2518406201049a8245cd88cce07f,
            0x3ad10f628663d389c30a470c31ca6483c26d655e3e77a299e42b88d73ee62e,
            0x1ce41b469e51820da799eabfaef4aa08b3ad84263289f14be28a6a3db4fdb63,
        ]
            .span(),
        column_witness: array![
            m31(885772305), m31(94648313), m31(604384470), m31(957953858), m31(608524802),
            m31(428382412),
        ]
            .span(),
    };
    let mut queries_per_log_size = Default::default();
    queries_per_log_size.insert(3, NullableTrait::new(array![2, 5, 7].span()));
    queries_per_log_size.insert(4, NullableTrait::new(array![7, 11, 14].span()));
    let queried_values = array![
        m31(720125469), m31(968171809), m31(364669117), m31(996158769), m31(997644238),
        m31(100529415), m31(933029034), m31(69309287), m31(194302184), m31(1057594968),
        m31(285391207), m31(420798739), m31(122725140), m31(1012109813), m31(766295003),
        m31(552345729), m31(650584843), m31(71167745), m31(840979908), m31(428994537),
        m31(28706943), m31(696999129), m31(942699537), m31(330264928), m31(658446453),
        m31(992269493), m31(967997322), m31(287489501), m31(310081088), m31(409791388),
    ]
        .span();
    let columns_by_log_size = group_columns_by_log_size(column_log_sizes.span());
    MerkleVerifier { root, column_log_sizes, columns_by_log_size }
        .verify(queries_per_log_size, queried_values, decommitment);
}
