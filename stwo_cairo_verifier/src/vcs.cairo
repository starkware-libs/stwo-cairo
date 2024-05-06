use core::array::ArrayTrait;
use core::option::OptionTrait;
use core::poseidon::poseidon_hash_span;
use stwo_cairo_verifier::BaseField;

pub trait MerkleHasher {
    type Hash;
    /// Hashes a single Merkle node. See [MerkleHasher] for more details.
    fn hash_node(
        children_hashes: Option<(Self::Hash, Self::Hash)>, column_values: Array<BaseField>,
    ) -> Self::Hash;
}

pub impl PoseidonMerkleHasher of MerkleHasher {
    type Hash = felt252;

    fn hash_node(
        children_hashes: Option<(Self::Hash, Self::Hash)>, mut column_values: Array<BaseField>,
    ) -> Self::Hash {
        let mut hash_array: Array<felt252> = Default::default();
        if let Option::Some((x, y)) = children_hashes {
            hash_array.append(x);
            hash_array.append(y);
        }

        // Pad column_values to a multiple of 8.
        let mut pad_len = 7 - ((column_values.len() + 7) % 8);
        while pad_len > 0 {
            column_values.append(core::num::traits::Zero::zero());
            pad_len = 7 - ((column_values.len() + 7) % 8);
        };

        while !column_values.is_empty() {
            let mut word = 0;
            word = word * 0x80000000 + column_values.pop_front().unwrap().inner.into();
            word = word * 0x80000000 + column_values.pop_front().unwrap().inner.into();
            word = word * 0x80000000 + column_values.pop_front().unwrap().inner.into();
            word = word * 0x80000000 + column_values.pop_front().unwrap().inner.into();
            word = word * 0x80000000 + column_values.pop_front().unwrap().inner.into();
            word = word * 0x80000000 + column_values.pop_front().unwrap().inner.into();
            word = word * 0x80000000 + column_values.pop_front().unwrap().inner.into();
            word = word * 0x80000000 + column_values.pop_front().unwrap().inner.into();
        };

        poseidon_hash_span(hash_array.span())
    }
}

#[cfg(test)]
mod tests {
    use super::PoseidonMerkleHasher;
    use stwo_cairo_verifier::fields::m31::{m31};

    #[test]
    fn test_m31() {
        assert_eq!(
            PoseidonMerkleHasher::hash_node(Option::None, array![m31(0), m31(1)]),
            973835572668429495915136902981656666590582180872133591629269551720657739196
        );
    }
}
