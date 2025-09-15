use core::box::BoxImpl;
use crate::channel::blake2s::{Blake2sChannel, ChannelTrait, check_leading_zeros};
use crate::fields::qm31::qm31_const;
use crate::vcs::blake2s_hasher::Blake2sHash;

#[test]
fn test_mix_felts_with_1_felt() {
    let mut channel: Blake2sChannel = Default::default();

    channel.mix_felts([qm31_const::<1, 2, 3, 4>()].span());

    // Tested against values produced from Rust code.
    // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
    assert_eq!(
        channel.digest.hash.unbox(),
        [
            1586304710, 1167332849, 1688630032, 429142330, 4001363212, 2013799503, 180553907,
            2044853257,
        ],
    );
}

#[test]
fn test_mix_felts_with_2_felts() {
    let mut channel: Blake2sChannel = Default::default();

    channel.mix_felts([qm31_const::<1, 2, 3, 4>(), qm31_const::<5, 6, 7, 8>()].span());

    // Tested against values produced from Rust code.
    // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
    assert_eq!(
        channel.digest.hash.unbox(),
        [
            1835698174, 2969628929, 1758616107, 158303712, 3820231193, 179192886, 4063347398,
            3332297509,
        ],
    );
}

#[test]
fn test_mix_felts_with_3_felts() {
    let mut channel: Blake2sChannel = Default::default();

    channel
        .mix_felts(
            [qm31_const::<1, 2, 3, 4>(), qm31_const::<5, 6, 7, 8>(), qm31_const::<9, 10, 11, 12>()]
                .span(),
        );

    // Tested against values produced from Rust code.
    // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
    assert_eq!(
        channel.digest.hash.unbox(),
        [
            2116479765, 3227507660, 1737697798, 2518684651, 1068812914, 1858078313, 1722202885,
            2198022752,
        ],
    );
}

#[test]
fn test_mix_felts_with_4_felts() {
    let mut channel: Blake2sChannel = Default::default();

    channel
        .mix_felts(
            [
                qm31_const::<1, 2, 3, 4>(), qm31_const::<5, 6, 7, 8>(),
                qm31_const::<9, 10, 11, 12>(), qm31_const::<13, 14, 15, 16>(),
            ]
                .span(),
        );

    // Tested against values produced from Rust code.
    // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
    assert_eq!(
        channel.digest.hash.unbox(),
        [
            940149128, 1354728945, 2816315586, 1690943110, 210254904, 3746481728, 1339132640,
            3760408575,
        ],
    );
}

#[test]
fn test_mix_felts_with_5_felts() {
    let mut channel: Blake2sChannel = Default::default();

    channel
        .mix_felts(
            [
                qm31_const::<1, 2, 3, 4>(), qm31_const::<5, 6, 7, 8>(),
                qm31_const::<9, 10, 11, 12>(), qm31_const::<13, 14, 15, 16>(),
                qm31_const::<17, 18, 19, 20>(),
            ]
                .span(),
        );

    // Tested against values produced from Rust code.
    // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
    assert_eq!(
        channel.digest.hash.unbox(),
        [
            3425911356, 1462327982, 3241135902, 4212900065, 3145879221, 3413011910, 3946733048,
            4081152200,
        ],
    );
}

#[test]
fn test_mix_u64() {
    let mut channel: Blake2sChannel = Default::default();

    channel.mix_u64(0x1111222233334444);

    // Tested against values produced from Rust code.
    // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
    assert_eq!(
        channel.digest.hash.unbox(),
        [
            0xc13f9ebc, 0x97884ed2, 0x59336d95, 0x24977332, 0xcdca6b9d, 0x74924d22, 0x4abae704,
            0xce6edc77,
        ],
    );
}

#[test]
fn test_check_proof_of_work() {
    let digest = Blake2sHash { hash: BoxImpl::new([0b1000, 0, 0, 0, 0, 0, 0, 0]) };

    let res = check_leading_zeros(digest, 3);

    assert!(res);
}

#[test]
fn test_check_proof_of_work_with_invalid_n_bits() {
    let digest = Blake2sHash { hash: BoxImpl::new([0b1000, 0, 0, 0, 0, 0, 0, 0]) };

    let res = check_leading_zeros(digest, 4);

    assert!(!res);
}

#[test]
fn test_blake_u32s() {
    let mut channel: Blake2sChannel = Default::default();

    let result = channel.draw_u32s();
    // Tested against values produced from Rust code.
    // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
    assert_eq!(
        result,
        array![
            1508103417, 49928118, 1851109195, 649450964, 1514800545, 4236765031, 523819246,
            4066564620,
        ]
            .span(),
    );
}

#[test]
fn test_draw_secure_felt() {
    let mut channel: Blake2sChannel = Default::default();

    let felt = channel.draw_secure_felt();

    // Tested against values produced from Rust code.
    // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
    assert_eq!(felt, qm31_const::<1508103417, 49928118, 1851109195, 649450964>());
}

#[test]
fn test_draw_secure_felts() {
    let mut channel: Blake2sChannel = Default::default();

    let felts = channel.draw_secure_felts(8);

    // Tested against values produced from Rust code.
    // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
    assert_eq!(
        felts,
        array![
            qm31_const::<1508103417, 49928118, 1851109195, 649450964>(),
            qm31_const::<1514800545, 2089281384, 523819246, 1919080973>(),
            qm31_const::<1769619091, 1335149496, 2007506569, 1426464368>(),
            qm31_const::<853727757, 1673676888, 635879929, 1327640380>(),
            qm31_const::<1751831125, 1559795173, 442209472, 1280396692>(),
            qm31_const::<1528893536, 644814910, 503157674, 286565543>(),
            qm31_const::<1839261536, 1778992654, 807858428, 143171319>(),
            qm31_const::<128590986, 1618375851, 213276683, 76892197>(),
        ],
    );
}
