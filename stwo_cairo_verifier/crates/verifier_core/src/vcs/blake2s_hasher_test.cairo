use core::box::BoxImpl;
use crate::fields::m31::m31;
use crate::vcs::blake2s_hasher::{Blake2sHash, Blake2sMerkleHasher};

#[test]
fn test_hash_node_with_no_children() {
    assert_eq!(
        Blake2sMerkleHasher::hash_node(None, array![m31(0), m31(1)].span()).hash.unbox(),
        [
            3950351958, 4278888560, 2450494307, 4106812851, 2998960590, 1139581150, 933467563,
            4130483740,
        ],
    );
}

#[test]
fn test_hash_node_no_values() {
    let l_node = Blake2sHash {
        hash: BoxImpl::new(
            [
                3681832419, 3337899951, 3430013099, 3528524834, 2219321066, 4216813420, 3845114026,
                3930838554,
            ],
        ),
    };
    let r_node = Blake2sHash {
        hash: BoxImpl::new(
            [
                4059334652, 764682711, 430320168, 2993275017, 2101662096, 297325600, 3930525507,
                2442780430,
            ],
        ),
    };
    let values = array![];
    assert_eq!(
        Blake2sMerkleHasher::hash_node(Some((l_node, r_node)), values.span()).hash.unbox(),
        [
            3396225248, 2356918684, 762400913, 361771559, 2717670650, 3342565712, 4023740156,
            2064877106,
        ],
    );
}

#[test]
fn test_hash_node_with_values_and_children() {
    let l_node = Blake2sHash {
        hash: BoxImpl::new(
            [
                3681832419, 3337899951, 3430013099, 3528524834, 2219321066, 4216813420, 3845114026,
                3930838554,
            ],
        ),
    };
    let r_node = Blake2sHash {
        hash: BoxImpl::new(
            [
                4059334652, 764682711, 430320168, 2993275017, 2101662096, 297325600, 3930525507,
                2442780430,
            ],
        ),
    };
    let values = array![m31(0), m31(1), m31(2), m31(3), m31(4), m31(5), m31(6), m31(7), m31(8)];
    assert_eq!(
        Blake2sMerkleHasher::hash_node(Some((l_node, r_node)), values.span()).hash.unbox(),
        [
            766742790, 300787742, 2865225068, 485846962, 3944318576, 3450413777, 1081705156,
            662001748,
        ],
    );
}
