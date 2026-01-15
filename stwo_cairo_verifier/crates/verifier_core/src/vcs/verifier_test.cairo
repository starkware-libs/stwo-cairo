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
                    526321295, 157905731, 3034573069, 4201848894, 327295601, 4260345375, 411496437,
                    1759757165,
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
}
