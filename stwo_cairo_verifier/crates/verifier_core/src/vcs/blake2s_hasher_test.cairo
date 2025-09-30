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
            3432458808, 4283433860, 761879229, 2715090978, 2102167318, 1865479142, 1634176718,
            817874949,
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
            1822702744, 914685986, 3195420188, 541197842, 3984139296, 2532724566, 4128774187,
            3849664352,
        ],
    );
}
