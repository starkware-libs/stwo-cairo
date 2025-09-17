use starknet_ff::FieldElement;
use stwo::prover::backend::simd::conversion::{Pack, Unpack};
use stwo::prover::backend::simd::m31::PackedM31;
use stwo_cairo_common::preprocessed_columns::poseidon_round_keys::{
    round_keys, POSEIDON_ROUND_KEYS,
};
use stwo_cairo_common::prover_types::cpu::{Felt252Width27, M31};
use stwo_cairo_common::prover_types::simd::PackedFelt252Width27;

pub fn round_keys_field_elements(round: usize) -> [FieldElement; 3] {
    POSEIDON_ROUND_KEYS[round].map(FieldElement::from_mont)
}

#[derive(Debug)]
pub struct PoseidonRoundKeys {}
impl PoseidonRoundKeys {
    pub fn deduce_output(round: M31) -> [Felt252Width27; 3] {
        round_keys(round.0 as usize)
    }
}

pub struct PackedPoseidonRoundKeys {}
impl PackedPoseidonRoundKeys {
    pub fn deduce_output(input: [PackedM31; 1]) -> [PackedFelt252Width27; 3] {
        let unpacked_inputs = input.unpack();
        <_ as Pack>::pack(unpacked_inputs.map(|round| PoseidonRoundKeys::deduce_output(round[0])))
    }
}

#[derive(Debug)]
pub struct Cube252 {}
impl Cube252 {
    // This value is equal to 2**768 % PRIME, which compensates for the three Montgomery divisions
    // by 2**256 performed in the three multiplications of a cubing.
    const FELT252_MONT_CUBE_FACTOR: FieldElement = FieldElement::from_mont([
        14731687596718420366,
        8450283861232831494,
        17617383518939119640,
        256247204371237485,
    ]);

    /// This method returns the cube of a FieldElement, as if it represents an element directly
    /// rather than in Montgomery form. While normal cubing of an element x = a * R (representing a
    /// in Montgomenry form) results in a**3 * R, this function instead returns x**3 = a**3 * R**3.
    pub fn cube(x: FieldElement) -> FieldElement {
        x * x * x * Self::FELT252_MONT_CUBE_FACTOR
    }

    pub fn deduce_output(x: Felt252Width27) -> Felt252Width27 {
        let x: FieldElement = x.into();
        Self::cube(x).into()
    }
}

pub struct PackedCube252 {}
impl PackedCube252 {
    pub fn deduce_output(input: PackedFelt252Width27) -> PackedFelt252Width27 {
        let unpacked_inputs = input.unpack();
        <_ as Pack>::pack(unpacked_inputs.map(Cube252::deduce_output))
    }
}

#[derive(Debug)]
pub struct PoseidonFullRoundChain {}
impl PoseidonFullRoundChain {
    pub fn deduce_output(
        chain: M31,
        round: M31,
        state: [Felt252Width27; 3],
    ) -> (M31, M31, [Felt252Width27; 3]) {
        let [x, y, z] = state.map(|x| Cube252::cube(x.into()));
        let keys = round_keys_field_elements(round.0 as usize);

        // An implementation of the MDS [[3, 1, 1], [1, -1, 1], [1, 1, -2]] using only
        // 7 field adds/subs (plus 3 more additions for the round keys) and no muls.
        let y1_zm1 = y - z;
        let x1_ym1_z1 = x - y1_zm1;
        let x1_y1_zm1 = x + y1_zm1;
        let x1_y1 = x + y;
        let x2_y2 = x1_y1 + x1_y1;

        let new_x = x2_y2 + x1_ym1_z1 + keys[0];
        let new_y = x1_ym1_z1 + keys[1];
        let new_z = x1_y1_zm1 - z + keys[2];
        let state = [new_x, new_y, new_z].map(Felt252Width27::from);

        (chain, round + M31::from(1), state)
    }
}
pub struct PackedPoseidonFullRoundChain {}
impl PackedPoseidonFullRoundChain {
    pub fn deduce_output(
        input: (PackedM31, PackedM31, [PackedFelt252Width27; 3]),
    ) -> (PackedM31, PackedM31, [PackedFelt252Width27; 3]) {
        let unpacked_inputs = input.unpack();
        <_ as Pack>::pack(unpacked_inputs.map(|(chain, round, state)| {
            PoseidonFullRoundChain::deduce_output(chain, round, state)
        }))
    }
}

#[derive(Debug)]
pub struct Poseidon3PartialRoundsChain {}
impl Poseidon3PartialRoundsChain {
    fn partial_round_field_elements(
        [z03, z1, z13, z2]: [FieldElement; 4],
        half_key: FieldElement,
    ) -> [FieldElement; 4] {
        let z23 = Cube252::cube(z2);

        // An implementation of the linear combination
        //   z3 = 8*z03 + 4*z1 + 6*z13 + 2*z2 - 2*z23 + 2*half_key
        // using only 9 field adds/subs, and no muls.
        let z03_z13 = z03 + z13;
        let z03_z13_z1 = z03_z13 + z1;
        let longsum = z03_z13_z1 + z2 - z23 + half_key;
        let half_z3 = longsum + z03_z13_z1 + z03_z13 + z03;
        let z3 = half_z3 + half_z3;

        [z13, z2, z23, z3]
    }

    pub fn deduce_output(
        chain: M31,
        round: M31,
        state: [Felt252Width27; 4],
    ) -> (M31, M31, [Felt252Width27; 4]) {
        let mut state = state.map(FieldElement::from);
        let keys = round_keys_field_elements(round.0 as usize);
        for half_key in keys {
            state = Self::partial_round_field_elements(state, half_key);
        }
        let state = state.map(Felt252Width27::from);

        (chain, round + M31::from(1), state)
    }
}
pub struct PackedPoseidon3PartialRoundsChain {}
impl PackedPoseidon3PartialRoundsChain {
    pub fn deduce_output(
        input: (PackedM31, PackedM31, [PackedFelt252Width27; 4]),
    ) -> (PackedM31, PackedM31, [PackedFelt252Width27; 4]) {
        let unpacked_inputs = input.unpack();
        <_ as Pack>::pack(unpacked_inputs.map(|(chain, round, state)| {
            Poseidon3PartialRoundsChain::deduce_output(chain, round, state)
        }))
    }
}

#[cfg(test)]
mod tests {
    use num_traits::Zero;

    use super::*;

    fn into_felt252(l: u128, h: u128) -> Felt252Width27 {
        Felt252Width27 {
            limbs: [
                (l & 0xffffffff_ffffffffu128) as u64,
                (l >> 64) as u64,
                (h & 0xffffffff_ffffffffu128) as u64,
                (h >> 64) as u64,
            ],
        }
    }

    #[test]
    fn test_cube252() {
        assert_eq!(
            Cube252::deduce_output(into_felt252(3, 0)),
            into_felt252(27, 0)
        );

        assert_eq!(
            Cube252::deduce_output(into_felt252(1_u128 << 64, 0)),
            into_felt252(0, 1_u128 << 64)
        );

        assert_eq!(
            Cube252::deduce_output(into_felt252(0, 1u128 << (251 - 128))),
            into_felt252(
                0x30ea8ae0ccefff980d98e2efa280dcu128,
                0xe7271cbef30e79ffffe8fb09f06c60u128
            )
        );

        assert_eq!(
            Cube252::deduce_output(into_felt252(
                0xffffffff_ffffffff_ffffffff_ffffffffu128,
                0x07ffffff_ffffffff_ffffffff_ffffffffu128
            )),
            into_felt252(
                0x30ea8adfe6a3ff980d98e4d90400d5u128,
                0xe7273d3e6c8de0ffffe8fb09f0d8c0u128
            )
        );

        assert_eq!(
            Cube252::deduce_output(into_felt252(
                0x01234567_89abcdef_fedcba98_76543210u128,
                0x01234567_89abcdef_fedcba98_76543210u128
            )),
            into_felt252(
                0x7f0d24086b967fd21d60203b176451d4u128,
                0x552b02c038f04d62a878b4b879c017du128
            )
        );

        assert_eq!(
            Cube252::deduce_output(into_felt252(0xffffffffffffffffffffffffffffffu128, 0u128)),
            into_felt252(
                0x2ffe000000000000043ffffffffffffu128,
                0x4810000000000000000000000000000u128
            )
        );
    }

    #[test]
    fn test_full_round() {
        assert_eq!(
            PoseidonFullRoundChain::deduce_output(
                M31::zero(),
                M31::zero(),
                [
                    into_felt252(
                        0xe58e2ad98109ae4780b7fd8eac77fe70u128,
                        0x6861759ea556a2339dd92f9562a30b9u128
                    ),
                    into_felt252(
                        0x3da43f76abf28a64e4ab1a22f27508c6u128,
                        0x3827681995d5af9ffc8397a3d00425au128
                    ),
                    into_felt252(
                        0x2cac75dc279b2d687a0dbe17704a830cu128,
                        0x3a3956d2fad44d0e7f760a2277dc7cbu128
                    ),
                ]
            ),
            (
                M31::zero(),
                M31::from(1),
                [
                    into_felt252(
                        0x182ac04678b725e9cd28a9a910551228u128,
                        0x7fe46959f384f2a87db105d7a8ef27u128
                    ),
                    into_felt252(
                        0xb9da7ad31613d5fc91092ddda81c074eu128,
                        0xbda37a4df11995b73b227acf31008cu128
                    ),
                    into_felt252(
                        0x269c04ea49158efba215b84ebe38196cu128,
                        0x2a756b59d27dc9eacc79f2ec1a3cbceu128
                    ),
                ]
            )
        );

        assert_eq!(
            PoseidonFullRoundChain::deduce_output(
                M31::zero(),
                M31::from(3),
                [
                    into_felt252(
                        0xcbafc1adb06e95d212132b0c0d0e5646u128,
                        0x208ff902dab0ef051b12fdb03b79a5bu128
                    ),
                    into_felt252(
                        0x12b2b2f261de39db323c24b6bb80a4e6u128,
                        0x1c2b35cc5f2ffb5c8949be8fdaa0a69u128
                    ),
                    into_felt252(
                        0x4ea222e01d5fc8d79794cd442dc7e2c7u128,
                        0x6bed17577108d9c3e7248bd95f32a21u128
                    ),
                ]
            ),
            (
                M31::zero(),
                M31::from(4),
                [
                    into_felt252(
                        0x44fa12d484b6715dda64b90368464504u128,
                        0x849be1221f8c38be2766fcadb3e4c8u128
                    ),
                    into_felt252(
                        0xfbd0349285a7795ebd6234122d513069u128,
                        0x4af998bff9c606f1b444844f9536292u128
                    ),
                    into_felt252(
                        0x27192be6b723923be05140f2d45c356eu128,
                        0x20aaaee2bc0e29c34a78721e2ca88abu128
                    ),
                ]
            )
        );

        assert_eq!(
            PoseidonFullRoundChain::deduce_output(
                M31::zero(),
                M31::from(34),
                [
                    into_felt252(
                        0x932437b264692f535842627ff2a74c19u128,
                        0x1c74c0faeb7590134bad3b2156e51e4u128
                    ),
                    into_felt252(
                        0xd8cdad6756c0158c4f560628e5035f40u128,
                        0x319010cf1fbc3a6a5cc8d74a7b6e0fbu128
                    ),
                    into_felt252(
                        0x35c9170386dbaafb0940ba8864dff049u128,
                        0xbcb539f18f2fc93a79d794f43dd10eu128
                    ),
                ]
            ),
            (
                M31::zero(),
                M31::from(35),
                [
                    into_felt252(
                        0xe932a9bf7456d009b1b174f36d558c5u128,
                        0xfa8c9b6742b6176139365833d001e3u128
                    ),
                    into_felt252(
                        0x2d16fba2151e4252a2e2111cde08bfe6u128,
                        0x4f04deca4cb7f9f2bd16b1d25b817cau128
                    ),
                    into_felt252(
                        0x72ab826e9bb5383a8018b59772964892u128,
                        0x58dde0a2a785b395ee2dc7b60b79e94u128
                    ),
                ]
            )
        );
    }

    #[test]
    fn test_partial_round() {
        assert_eq!(
            Poseidon3PartialRoundsChain::deduce_output(
                M31::zero(),
                M31::from(4),
                [
                    into_felt252(
                        0xe3537dca9f28533cade50a1864816c70u128,
                        0x9b4e4d358f1423380738337bbf67c8u128
                    ),
                    into_felt252(
                        0xe243fdbf6366aed9d5369d50e2b9b480u128,
                        0x3c4e1f55af4447cadf6bd704eb01d79u128
                    ),
                    into_felt252(
                        0xa07a6ff0d70f8d1338144fd09791059du128,
                        0x243f648e2ecd9720bb2657aad0a79b5u128
                    ),
                    into_felt252(
                        0xae15579d749d434bece9fb13fa3a39d7u128,
                        0x396488f7b22e929802a48afea919080u128
                    ),
                ]
            ),
            (
                M31::zero(),
                M31::from(5),
                [
                    into_felt252(
                        0x8c0a712f0be18a3795c645569683af77u128,
                        0x6533e92d5aa8e4f6867494ec59573cbu128
                    ),
                    into_felt252(
                        0xc48f1f967e974e894689a7d15f60af10u128,
                        0x37292a0a739ee1dc8b6b27cc4e2a6a4u128
                    ),
                    into_felt252(
                        0x13a7f72654da48fb022be337f1179920u128,
                        0x6e43f5086c021c40056b2673a6a9a4cu128
                    ),
                    into_felt252(
                        0xef171642498d0042106c34abdf2c6b5du128,
                        0x63c745ffb88adea76b5f49ce227847fu128
                    ),
                ]
            )
        );

        assert_eq!(
            Poseidon3PartialRoundsChain::deduce_output(
                M31::zero(),
                M31::from(30),
                [
                    into_felt252(
                        0x1d810ad08ec7292a3e480d064e3c33fdu128,
                        0x24197cdb19d911de8c1aefd79a1e361u128
                    ),
                    into_felt252(
                        0x3df9d9c8f0ea31661422c910a953e90eu128,
                        0x79bc5f0ef7295007407214b31fc9c5du128
                    ),
                    into_felt252(
                        0x37f0afa4744515e8b6289c6e7088fa53u128,
                        0x5fede0e8c1d858b07208faaf87e2825u128
                    ),
                    into_felt252(
                        0xd990717ed1feae68b4ce9a884f5fa1dcu128,
                        0x124ab89fe982b447d1721cab0feeeddu128
                    ),
                ]
            ),
            (
                M31::zero(),
                M31::from(31),
                [
                    into_felt252(
                        0xb6f492c4e386fea4cae83dcb7d09dcfau128,
                        0x11737e2519e23a2efbeb74703e14806u128
                    ),
                    into_felt252(
                        0x19c06386a2f3c99c45a26f4bb3484bb7u128,
                        0x681bfdd9318d1399d41f9a200353addu128
                    ),
                    into_felt252(
                        0xc9509dbbe4d81cc3bda85be223cbab69u128,
                        0x6446e7317d7f5c81b361fa0a425b38fu128
                    ),
                    into_felt252(
                        0x9e0c2c87b68a3eb97700c355bfbfdf01u128,
                        0x3c04b83fb3af01feceeb140bbab0322u128
                    ),
                ]
            )
        );
    }
}
