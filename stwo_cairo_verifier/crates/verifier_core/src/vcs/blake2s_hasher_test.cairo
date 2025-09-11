use core::box::BoxImpl;
use crate::fields::m31::m31;
use crate::vcs::blake2s_hasher::{Blake2sHash, Blake2sMerkleHasher};

#[test]
fn test_hash_node_with_no_children() {
    assert_eq!(
        Blake2sMerkleHasher::hash_node(None, array![m31(0), m31(1)].span()).hash.unbox(),
        [
            1626923414, 1435242944, 303433540, 2677787522, 3418209936, 3935220714, 2547822313,
            1957245117,
        ],
    );
}

#[test]
fn test_hash_node_with_children() {
    let l_node = Blake2sHash {
        hash: BoxImpl::new(
            [
                3950351958, 4278888560, 2450494307, 4106812851, 2998960590, 1139581150, 933467563,
                4130483740,
            ],
        ),
    };
    let r_node = Blake2sHash {
        hash: BoxImpl::new(
            [
                707328262, 1356420215, 727373128, 3551204147, 4019359644, 4016890851, 1375080809,
                3547510316,
            ],
        ),
    };
    assert_eq!(
        Blake2sMerkleHasher::hash_node(Some((l_node, r_node)), array![m31(3)].span()).hash.unbox(),
        [
            1667812920, 3868823599, 133107752, 3946813196, 1554186939, 246303193, 2376416823,
            4177911128,
        ],
    );
}

#[test]
fn test_hash_node_with_many_values() {
    let l_node = Blake2sHash {
        hash: BoxImpl::new(
            [
                3950351958, 4278888560, 2450494307, 4106812851, 2998960590, 1139581150, 933467563,
                4130483740,
            ],
        ),
    };
    let r_node = Blake2sHash {
        hash: BoxImpl::new(
            [
                707328262, 1356420215, 727373128, 3551204147, 4019359644, 4016890851, 1375080809,
                3547510316,
            ],
        ),
    };
    let values = array![
        m31(0), m31(1), m31(2), m31(3), m31(4), m31(5), m31(6), m31(7), m31(8), m31(9), m31(10),
        m31(11), m31(12), m31(13), m31(14), m31(15), m31(16), m31(17), m31(18), m31(19), m31(20),
        m31(21),
    ];
    assert_eq!(
        Blake2sMerkleHasher::hash_node(Some((l_node, r_node)), values.span()).hash.unbox(),
        [
            3196709039, 4157792026, 395775385, 2063589765, 582410954, 2098025188, 3346023828,
            3994527087,
        ],
    );
}
