use crate::fields::m31::m31;
use crate::vcs::poseidon_hasher::PoseidonMerkleHasher;

#[test]
fn test_m31() {
    assert_eq!(
        PoseidonMerkleHasher::hash_node(None, array![m31(0), m31(1)].span()),
        2978883932528585652864046122079599882777358126302490183268546077323303473078,
    );

    assert_eq!(
        PoseidonMerkleHasher::hash_node(Some((1, 2)), array![m31(3)].span()),
        3286095315900630438551061262740794783852190427874264245042874292062185873630,
    );
}
