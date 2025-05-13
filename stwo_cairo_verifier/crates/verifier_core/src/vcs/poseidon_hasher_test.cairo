#[cfg(test)]
mod tests {
    use crate::fields::m31::m31;
    use crate::vcs::poseidon_hasher::PoseidonMerkleHasher;

    #[test]
    fn test_m31() {
        assert_eq!(
            PoseidonMerkleHasher::hash_node(None, array![m31(0), m31(1)].span()),
            2552053700073128806553921687214114320458351061521275103654266875084493044716,
        );

        assert_eq!(
            PoseidonMerkleHasher::hash_node(Some((1, 2)), array![m31(3)].span()),
            159358216886023795422515519110998391754567506678525778721401012606792642769,
        );
    }
}
