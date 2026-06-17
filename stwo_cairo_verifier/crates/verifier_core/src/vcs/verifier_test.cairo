#[cfg(not(feature: "poseidon252_verifier"))]
mod BlakeTest {
    use core::box::BoxImpl;
    use crate::fields::m31::m31;
    use crate::vcs::blake2s_hasher::{Blake2sHash, Blake2sMerkleHasher};
    use crate::vcs::verifier::{MerkleDecommitment, MerkleVerifier, MerkleVerifierTrait};

    fn prepare_verifier_values() -> Array<Blake2sHash> {
        let root = Blake2sHash {
            hash: BoxImpl::new(
                [
                    1166138673, 2445736790, 15165448, 367559997, 1337972960, 1582281981, 2599545023,
                    2602241306,
                ],
            ),
        };

        let witness0 = Blake2sHash {
            hash: BoxImpl::new(
                [
                    3527665513, 2883753490, 2824372921, 1126320381, 3284464291, 412328431,
                    4179897757, 4108441023,
                ],
            ),
        };
        let witness1 = Blake2sHash {
            hash: BoxImpl::new(
                [
                    1895781335, 406586817, 2943640900, 1162698830, 3402416469, 3995066570,
                    1037625426, 4261489706,
                ],
            ),
        };
        let witness2 = Blake2sHash {
            hash: BoxImpl::new(
                [
                    2986588344, 2715718596, 1645705056, 1245334775, 704985260, 1912793072,
                    797619254, 402598591,
                ],
            ),
        };
        let witness3 = Blake2sHash {
            hash: BoxImpl::new(
                [
                    2423846805, 76366567, 1876163899, 509464482, 2624872233, 1389355692, 1983230920,
                    337017200,
                ],
            ),
        };
        array![root, witness0, witness1, witness2, witness3]
    }

    #[test]
    fn test_verifier() {
        let test_values = prepare_verifier_values();
        let decommitment = MerkleDecommitment::<
            Blake2sMerkleHasher,
        > {
            hash_witness: array![*test_values[1], *test_values[2], *test_values[3], *test_values[4]]
                .span(),
        };
        let queries_position = array![4].span();
        let queried_values = array![m31(0), m31(2), m31(4)].span();
        let column_log_deg_bounds = array![2, 3, 4].span();
        MerkleVerifier { root: *test_values[0], tree_height: 4, column_log_deg_bounds }
            .verify(queries_position, queried_values, decommitment);
    }


    #[test]
    fn test_duplication_success() {
        let test_values = prepare_verifier_values();
        let decommitment = MerkleDecommitment::<
            Blake2sMerkleHasher,
        > {
            hash_witness: array![*test_values[1], *test_values[2], *test_values[3], *test_values[4]]
                .span(),
        };
        // Query the same position twice.
        let queries_position = array![4, 4].span();
        let queried_values = array![m31(0), m31(2), m31(4), m31(0), m31(2), m31(4)].span();
        let column_log_deg_bounds = array![2, 3, 4].span();
        MerkleVerifier { root: *test_values[0], tree_height: 4, column_log_deg_bounds }
            .verify(queries_position, queried_values, decommitment);
    }

    #[test]
    #[should_panic]
    fn test_duplication_fail() {
        let test_values = prepare_verifier_values();
        let decommitment = MerkleDecommitment::<
            Blake2sMerkleHasher,
        > {
            hash_witness: array![*test_values[1], *test_values[2], *test_values[3], *test_values[4]]
                .span(),
        };
        // Query the same position twice but give different queried values.
        let queries_position = array![4, 4].span();
        let queried_values = array![m31(0), m31(2), m31(4), m31(0), m31(3), m31(4)].span();
        let column_log_deg_bounds = array![2, 3, 4].span();

        MerkleVerifier { root: *test_values[0], tree_height: 4, column_log_deg_bounds }
            .verify(queries_position, queried_values, decommitment);
    }

    /// Test data for a tree over columns of log sizes [2, 3, 4] queried at positions `[7, 1, 4,
    /// 7]`.
    fn unsorted_decommitment() -> (Blake2sHash, MerkleDecommitment<Blake2sMerkleHasher>) {
        let root = Blake2sHash {
            hash: BoxImpl::new(
                [
                    799111052, 3851046149, 767764003, 750019224, 868185721, 2802927524, 2631427736,
                    2953523631,
                ],
            ),
        };
        let decommitment = MerkleDecommitment::<
            Blake2sMerkleHasher,
        > {
            hash_witness: array![
                Blake2sHash {
                    hash: BoxImpl::new(
                        [
                            1686078255, 1251989663, 1399841896, 1070415095, 77577676, 3523387273,
                            772468701, 1963156859,
                        ],
                    ),
                },
                Blake2sHash {
                    hash: BoxImpl::new(
                        [
                            3190349240, 949347602, 832165461, 554053959, 4286760971, 996742177,
                            2101294015, 3914298350,
                        ],
                    ),
                },
                Blake2sHash {
                    hash: BoxImpl::new(
                        [
                            823642232, 1108693860, 1374718226, 4116224843, 2342976400, 4273534153,
                            4280580980, 2568719764,
                        ],
                    ),
                },
                Blake2sHash {
                    hash: BoxImpl::new(
                        [
                            287489983, 282350153, 2774670933, 2151505221, 910818842, 2214082789,
                            2388741195, 2405550818,
                        ],
                    ),
                },
                Blake2sHash {
                    hash: BoxImpl::new(
                        [
                            1410283336, 2815891655, 3263519339, 573411765, 393219428, 2870469238,
                            587231783, 2042633243,
                        ],
                    ),
                },
            ]
                .span(),
        };
        (root, decommitment)
    }

    /// Verification must succeed when the query positions are given out of order and contain a
    /// non-adjacent duplicate.
    #[test]
    fn test_unsorted_and_duplicate_query_positions() {
        let (root, decommitment) = unsorted_decommitment();
        let queries_position = array![7, 1, 4, 7].span();
        let queried_values = array![
            m31(1), m31(3), m31(7), m31(1), m31(1), m31(1), m31(0), m31(2), m31(4), m31(1), m31(3),
            m31(7),
        ]
            .span();
        let column_log_deg_bounds = array![2, 3, 4].span();
        MerkleVerifier { root, tree_height: 4, column_log_deg_bounds }
            .verify(queries_position, queried_values, decommitment);
    }

    /// Verification must reject a duplicated position whose queried values disagree.
    #[test]
    #[should_panic]
    fn test_non_adjacent_inconsistent_duplicate_fail() {
        let (root, decommitment) = unsorted_decommitment();
        let queries_position = array![7, 1, 4, 7].span();
        // Same as the success case, but the second occurrence of `7` carries a different value.
        let queried_values = array![
            m31(1), m31(3), m31(7), m31(1), m31(1), m31(1), m31(0), m31(2), m31(4), m31(1), m31(3),
            m31(8),
        ]
            .span();
        let column_log_deg_bounds = array![2, 3, 4].span();
        MerkleVerifier { root, tree_height: 4, column_log_deg_bounds }
            .verify(queries_position, queried_values, decommitment);
    }
}
